//! Experimental MIR adapter boundary for MVP 4.
//!
//! Atlas semantics remain outside this crate. The adapter currently proves one
//! scalar MIR interpreter call and compares guest-reference representations.

use std::sync::Mutex;

static MIR_ADAPTER_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GuestOffset(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GuestHandle(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GuestRegionId(u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GuestRegionOffset {
    region: GuestRegionId,
    offset: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GuestMemoryError {
    AddressOverflow,
    OutOfBounds,
    Misaligned,
    InvalidHandle,
    InvalidRegion,
    RuntimeFailure,
    InvalidTraceEvent,
}

/// One semantic comparison explicitly emitted by the experimental MIR program.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CompareEvent {
    pub candidate: i64,
    pub current: i64,
}

/// Bounded, process-local trace for the three-value minimum experiment.
///
/// This is debugging instrumentation only. It is neither an Atlas evidence
/// format nor a stable FFI contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct MinimumTrace {
    pub minimum: i64,
    pub count: u32,
    pub events: [CompareEvent; 2],
}

/// One AST operation emitted by the private partition MIR lowering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartitionTraceEvent {
    pub ast_node_id: &'static str,
    pub operation: PartitionTraceOperation,
}

/// Semantic kind asserted by a partition trace event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartitionTraceOperation {
    Read,
    Predicate,
    Swap,
    Partition,
}

/// Bounded, process-local trace from the private partition MIR lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionTrace {
    pub boundary: usize,
    pub events: Vec<PartitionTraceEvent>,
    pub truncated: bool,
}

const PARTITION_TRACE_CAPACITY: usize = 128;

#[repr(C)]
struct RawPartitionTrace {
    boundary: u32,
    count: u32,
    truncated: u32,
    events: [u32; PARTITION_TRACE_CAPACITY],
}

impl GuestOffset {
    pub fn new(offset: u32) -> Self {
        Self(offset)
    }

    pub fn checked_add(self, amount: u32) -> Result<Self, GuestMemoryError> {
        self.0
            .checked_add(amount)
            .map(Self)
            .ok_or(GuestMemoryError::AddressOverflow)
    }
}

impl GuestHandle {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }
}

impl GuestRegionId {
    pub fn new(raw: u16) -> Self {
        Self(raw)
    }
}

impl GuestRegionOffset {
    pub fn new(region: GuestRegionId, offset: u32) -> Self {
        Self { region, offset }
    }

    pub fn checked_add(self, amount: u32) -> Result<Self, GuestMemoryError> {
        Ok(Self {
            region: self.region,
            offset: self
                .offset
                .checked_add(amount)
                .ok_or(GuestMemoryError::AddressOverflow)?,
        })
    }
}

#[derive(Debug)]
pub struct OffsetMemory {
    bytes: Vec<u8>,
}

impl OffsetMemory {
    pub fn new(size: usize) -> Self {
        Self {
            bytes: vec![0; size],
        }
    }

    pub fn write(&mut self, reference: GuestOffset, value: u8) -> Result<(), GuestMemoryError> {
        *self
            .bytes
            .get_mut(reference.0 as usize)
            .ok_or(GuestMemoryError::OutOfBounds)? = value;
        Ok(())
    }

    pub fn read(&self, reference: GuestOffset) -> Result<u8, GuestMemoryError> {
        self.bytes
            .get(reference.0 as usize)
            .copied()
            .ok_or(GuestMemoryError::OutOfBounds)
    }

    pub fn write_i64_le(
        &mut self,
        reference: GuestOffset,
        value: i64,
    ) -> Result<(), GuestMemoryError> {
        let range = self.checked_range(reference, 8, 8)?;
        self.bytes[range].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    pub fn read_i64_le(&self, reference: GuestOffset) -> Result<i64, GuestMemoryError> {
        let range = self.checked_range(reference, 8, 8)?;
        let bytes: [u8; 8] = self.bytes[range]
            .try_into()
            .expect("checked i64 range has exactly eight bytes");
        Ok(i64::from_le_bytes(bytes))
    }

    fn checked_range(
        &self,
        reference: GuestOffset,
        width: u32,
        alignment: u32,
    ) -> Result<std::ops::Range<usize>, GuestMemoryError> {
        if reference.0 % alignment != 0 {
            return Err(GuestMemoryError::Misaligned);
        }
        let end = reference
            .0
            .checked_add(width)
            .ok_or(GuestMemoryError::AddressOverflow)?;
        let range = reference.0 as usize..end as usize;
        if range.end > self.bytes.len() {
            return Err(GuestMemoryError::OutOfBounds);
        }
        Ok(range)
    }
}

#[derive(Debug, Default)]
pub struct HandleMemory {
    objects: Vec<Option<Vec<u8>>>,
}

impl HandleMemory {
    pub fn allocate(&mut self, bytes: Vec<u8>) -> GuestHandle {
        let index = self.objects.len();
        self.objects.push(Some(bytes));
        GuestHandle(index as u32)
    }

    pub fn read(&self, handle: GuestHandle, offset: u32) -> Result<u8, GuestMemoryError> {
        self.objects
            .get(handle.0 as usize)
            .and_then(Option::as_ref)
            .ok_or(GuestMemoryError::InvalidHandle)?
            .get(offset as usize)
            .copied()
            .ok_or(GuestMemoryError::OutOfBounds)
    }
}

#[derive(Debug, Default)]
pub struct RegionMemory {
    regions: Vec<Vec<u8>>,
}

impl RegionMemory {
    pub fn add_region(&mut self, bytes: Vec<u8>) -> GuestRegionId {
        let index = self.regions.len();
        self.regions.push(bytes);
        GuestRegionId(index as u16)
    }

    pub fn read(&self, reference: GuestRegionOffset) -> Result<u8, GuestMemoryError> {
        self.regions
            .get(reference.region.0 as usize)
            .ok_or(GuestMemoryError::InvalidRegion)?
            .get(reference.offset as usize)
            .copied()
            .ok_or(GuestMemoryError::OutOfBounds)
    }
}

/// Executes a scalar i64 MIR function through the host interpreter.
pub fn interpret_add_u64(left: u64, right: u64) -> u64 {
    unsafe { atlas_mir_interpret_add_u64(left, right) }
}

/// Executes a three-value minimum program and returns its explicit MIR trace.
pub fn interpret_minimum3_i64(left: i64, middle: i64, right: i64) -> MinimumTrace {
    let _guard = MIR_ADAPTER_LOCK
        .lock()
        .expect("minimum trace lock must not be poisoned");
    let mut trace = MinimumTrace {
        minimum: 0,
        count: 0,
        events: [
            CompareEvent {
                candidate: 0,
                current: 0,
            },
            CompareEvent {
                candidate: 0,
                current: 0,
            },
        ],
    };

    unsafe {
        atlas_mir_interpret_minimum3_i64(left, middle, right, &mut trace);
    }
    trace
}

/// Lowers the explicit read, predicate and swap subset of `partition_ast()` to
/// MIR over a bounded little-endian offset region.
pub fn interpret_partition_even_i64(
    values: &mut [i64],
) -> Result<PartitionTrace, GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(8)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let _guard = MIR_ADAPTER_LOCK
        .lock()
        .expect("MIR adapter lock must not be poisoned");
    let mut memory = OffsetMemory::new(byte_length as usize);
    for (index, value) in values.iter().copied().enumerate() {
        let offset = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_mul(8))
            .ok_or(GuestMemoryError::AddressOverflow)?;
        memory.write_i64_le(GuestOffset::new(offset), value)?;
    }
    let mut raw = RawPartitionTrace {
        boundary: 0,
        count: 0,
        truncated: 0,
        events: [0; PARTITION_TRACE_CAPACITY],
    };
    let status = unsafe {
        atlas_mir_interpret_partition_even_i64(
            memory.bytes.as_mut_ptr(),
            byte_length,
            u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?,
            &mut raw,
        )
    };
    if status != 0 {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    let count = usize::try_from(raw.count).map_err(|_| GuestMemoryError::InvalidTraceEvent)?;
    if count > PARTITION_TRACE_CAPACITY {
        return Err(GuestMemoryError::InvalidTraceEvent);
    }
    for (index, value) in values.iter_mut().enumerate() {
        let offset = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_mul(8))
            .ok_or(GuestMemoryError::AddressOverflow)?;
        *value = memory.read_i64_le(GuestOffset::new(offset))?;
    }
    let events = raw.events[..count]
        .iter()
        .copied()
        .map(partition_trace_event)
        .collect::<Result<Vec<_>, _>>()?;

    let boundary = raw.boundary as usize;
    if boundary > values.len() {
        return Err(GuestMemoryError::InvalidTraceEvent);
    }

    Ok(PartitionTrace {
        boundary,
        events,
        truncated: raw.truncated != 0,
    })
}

fn partition_trace_event(code: u32) -> Result<PartitionTraceEvent, GuestMemoryError> {
    let (ast_node_id, operation) = match code {
        1 => ("partition.left.read", PartitionTraceOperation::Read),
        2 => (
            "partition.left.predicate",
            PartitionTraceOperation::Predicate,
        ),
        3 => ("partition.right.read", PartitionTraceOperation::Read),
        4 => (
            "partition.right.predicate",
            PartitionTraceOperation::Predicate,
        ),
        5 => ("partition.swap", PartitionTraceOperation::Swap),
        6 => ("partition.boundary", PartitionTraceOperation::Partition),
        _ => return Err(GuestMemoryError::InvalidTraceEvent),
    };
    Ok(PartitionTraceEvent {
        ast_node_id,
        operation,
    })
}

unsafe extern "C" {
    fn atlas_mir_interpret_add_u64(left: u64, right: u64) -> u64;
    fn atlas_mir_interpret_minimum3_i64(
        left: i64,
        middle: i64,
        right: i64,
        trace: *mut MinimumTrace,
    );
    fn atlas_mir_interpret_partition_even_i64(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
        trace: *mut RawPartitionTrace,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use super::{
        CompareEvent, GuestMemoryError, GuestOffset, GuestRegionOffset, HandleMemory, OffsetMemory,
        RegionMemory, interpret_add_u64, interpret_minimum3_i64, interpret_partition_even_i64,
    };

    #[test]
    fn mir_interpreter_executes_a_scalar_function() {
        assert_eq!(interpret_add_u64(40, 2), 42);
        assert_eq!(interpret_add_u64(12, 30), 42);
    }

    #[test]
    fn mir_trace_records_stable_minimum_comparisons() {
        let trace = interpret_minimum3_i64(7, -2, 4);
        let native =
            atlas_algorithms::minimum::minimum_by(&[7_i64, -2, 4], |left: &i64, right: &i64| {
                left.cmp(right)
            })
            .copied()
            .expect("non-empty native input");

        assert_eq!(trace.minimum, native);
        assert_eq!(trace.count, 2);
        assert_eq!(
            trace.events,
            [
                CompareEvent {
                    candidate: -2,
                    current: 7,
                },
                CompareEvent {
                    candidate: 4,
                    current: -2,
                },
            ]
        );
    }

    #[test]
    fn mir_trace_keeps_the_first_value_on_a_tie() {
        let trace = interpret_minimum3_i64(1, 1, 2);

        assert_eq!(trace.minimum, 1);
        assert_eq!(trace.count, 2);
        assert_eq!(
            trace.events,
            [
                CompareEvent {
                    candidate: 1,
                    current: 1,
                },
                CompareEvent {
                    candidate: 2,
                    current: 1,
                },
            ]
        );
    }

    #[test]
    fn mir_partition_lowering_matches_native_and_ast_operations() {
        let ast = atlas::ast::partition_ast();
        for original in [vec![], vec![2, 4], vec![1, 3], vec![3, 2, 5, 4, 7, 6]] {
            let mut values = original.clone();
            let trace = interpret_partition_even_i64(&mut values).expect("MIR partition execution");
            let mut native = original;
            let native_boundary =
                atlas_algorithms::partition::partition_in_place(&mut native, |value| {
                    value % 2 == 0
                });

            assert_eq!(trace.boundary, native_boundary);
            assert_eq!(values, native);
            assert!(!trace.truncated);
            for event in &trace.events {
                let expected = match event.operation {
                    super::PartitionTraceOperation::Read => atlas::ast::SemanticOperation::Read,
                    super::PartitionTraceOperation::Predicate => {
                        atlas::ast::SemanticOperation::Predicate
                    }
                    super::PartitionTraceOperation::Swap => atlas::ast::SemanticOperation::Swap,
                    super::PartitionTraceOperation::Partition => {
                        atlas::ast::SemanticOperation::Partition
                    }
                };
                assert_eq!(ast.operation_by_id(event.ast_node_id), Some(expected));
            }
        }
    }

    #[test]
    fn offset_model_detects_overflow_and_bounds() {
        let mut memory = OffsetMemory::new(4);
        let reference = GuestOffset::new(3);
        memory.write(reference, 9).expect("in bounds write");
        assert_eq!(memory.read(reference), Ok(9));
        assert_eq!(reference.checked_add(1), Ok(GuestOffset::new(4)));
        assert_eq!(
            memory.read(GuestOffset::new(4)),
            Err(GuestMemoryError::OutOfBounds)
        );
        assert_eq!(
            GuestOffset::new(u32::MAX).checked_add(1),
            Err(GuestMemoryError::AddressOverflow)
        );
    }

    #[test]
    fn offset_model_uses_aligned_little_endian_i64_values() {
        let mut memory = OffsetMemory::new(16);
        memory
            .write_i64_le(GuestOffset::new(0), 0x0102_0304_0506_0708)
            .expect("aligned i64 write");
        assert_eq!(memory.read(GuestOffset::new(0)), Ok(0x08));
        assert_eq!(
            memory.read_i64_le(GuestOffset::new(0)),
            Ok(0x0102_0304_0506_0708)
        );
        assert_eq!(
            memory.read_i64_le(GuestOffset::new(1)),
            Err(GuestMemoryError::Misaligned)
        );
    }

    #[test]
    fn handle_model_separates_identity_from_offset() {
        let mut memory = HandleMemory::default();
        let handle = memory.allocate(vec![4, 9]);
        assert_eq!(memory.read(handle, 1), Ok(9));
        assert_eq!(memory.read(handle, 2), Err(GuestMemoryError::OutOfBounds));
        assert_eq!(
            memory.read(super::GuestHandle::new(1), 0),
            Err(GuestMemoryError::InvalidHandle)
        );
    }

    #[test]
    fn region_offset_model_rejects_cross_region_confusion() {
        let mut memory = RegionMemory::default();
        let first = memory.add_region(vec![3]);
        let second = memory.add_region(vec![7]);
        assert_eq!(memory.read(GuestRegionOffset::new(first, 0)), Ok(3));
        assert_eq!(memory.read(GuestRegionOffset::new(second, 0)), Ok(7));
        assert_eq!(
            memory.read(GuestRegionOffset::new(super::GuestRegionId::new(2), 0)),
            Err(GuestMemoryError::InvalidRegion)
        );
        assert_eq!(
            GuestRegionOffset::new(first, u32::MAX).checked_add(1),
            Err(GuestMemoryError::AddressOverflow)
        );
    }
}
