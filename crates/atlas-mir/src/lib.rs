//! Experimental MIR adapter boundary for MVP 4.
//!
//! Atlas semantics remain outside this crate. The adapter exercises private
//! interpreter and host-JIT probes while preserving native Rust as oracle.

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

/// One AST operation emitted by the private `is_sorted` MIR lowering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IsSortedTraceEvent {
    pub ast_node_id: &'static str,
    pub operation: IsSortedTraceOperation,
}

/// Semantic kind asserted by an `is_sorted` trace event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsSortedTraceOperation {
    Read,
    Compare,
}

/// Bounded, process-local trace from the private `is_sorted` MIR lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsSortedTrace {
    pub sorted: bool,
    /// Index of the right-hand value in the first inverted adjacent pair.
    pub first_inversion: Option<usize>,
    pub events: Vec<IsSortedTraceEvent>,
    pub truncated: bool,
}

/// Result of the private host-JIT `is_sorted` correction probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JitIsSortedResult {
    pub sorted: bool,
    pub first_inversion: Option<usize>,
}

/// Optimization levels exposed by the pinned MIR host generator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum JitOptimizationLevel {
    FastGeneration = 0,
    RegisterAllocation = 1,
    Default = 2,
    Full = 3,
}

const PARTITION_TRACE_CAPACITY: usize = 128;
const IS_SORTED_TRACE_CAPACITY: usize = 128;

#[repr(C)]
struct RawPartitionTrace {
    boundary: u32,
    count: u32,
    truncated: u32,
    events: [u32; PARTITION_TRACE_CAPACITY],
}

#[repr(C)]
struct RawIsSortedTrace {
    sorted: u32,
    first_inversion: u32,
    count: u32,
    truncated: u32,
    events: [u32; IS_SORTED_TRACE_CAPACITY],
}

#[repr(C)]
struct RawSelectionResult {
    found: u32,
    index: u32,
    value: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionResult {
    pub value: i64,
    pub index: usize,
}

/// Private 16-byte guest element used to observe insertion-sort stability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InsertionPair {
    pub key: i64,
    pub original_index: u64,
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

/// Executes the scalar addition probe through MIR's host generator.
pub fn jit_add_u64(left: u64, right: u64) -> u64 {
    jit_add_u64_with_optimization(left, right, JitOptimizationLevel::Default)
}

/// Executes scalar addition with an explicit MIR optimization level.
pub fn jit_add_u64_with_optimization(
    left: u64,
    right: u64,
    optimization: JitOptimizationLevel,
) -> u64 {
    unsafe { atlas_mir_jit_add_u64_at_level(left, right, optimization as u32) }
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

/// Lowers adjacent signed `i64` reads and comparisons from `is_sorted_ast()`
/// to MIR over a bounded little-endian offset region.
pub fn interpret_is_sorted_i64(values: &[i64]) -> Result<IsSortedTrace, GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(8)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let element_count =
        u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?;
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
    let mut raw = RawIsSortedTrace {
        sorted: 0,
        first_inversion: u32::MAX,
        count: 0,
        truncated: 0,
        events: [0; IS_SORTED_TRACE_CAPACITY],
    };
    let status = unsafe {
        atlas_mir_interpret_is_sorted_i64(
            memory.bytes.as_mut_ptr(),
            byte_length,
            element_count,
            &mut raw,
        )
    };
    if status != 0 {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    if raw.sorted > 1 {
        return Err(GuestMemoryError::InvalidTraceEvent);
    }
    let count = usize::try_from(raw.count).map_err(|_| GuestMemoryError::InvalidTraceEvent)?;
    if count > IS_SORTED_TRACE_CAPACITY {
        return Err(GuestMemoryError::InvalidTraceEvent);
    }
    let sorted = raw.sorted != 0;
    let first_inversion = match (sorted, raw.first_inversion) {
        (true, u32::MAX) => None,
        (false, index) if (index as usize) < values.len() => Some(index as usize),
        _ => return Err(GuestMemoryError::InvalidTraceEvent),
    };
    let events = raw.events[..count]
        .iter()
        .copied()
        .map(is_sorted_trace_event)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IsSortedTrace {
        sorted,
        first_inversion,
        events,
        truncated: raw.truncated != 0,
    })
}

/// Executes the adjacent signed `i64` scan through MIR's host generator.
pub fn jit_is_sorted_i64(values: &[i64]) -> Result<JitIsSortedResult, GuestMemoryError> {
    jit_is_sorted_i64_with_optimization(values, JitOptimizationLevel::Default)
}

/// Executes the adjacent scan with an explicit MIR optimization level.
pub fn jit_is_sorted_i64_with_optimization(
    values: &[i64],
    optimization: JitOptimizationLevel,
) -> Result<JitIsSortedResult, GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(8)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let count = u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?;
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
    let mut first_inversion = u32::MAX;
    if unsafe {
        atlas_mir_jit_is_sorted_i64_at_level(
            memory.bytes.as_mut_ptr(),
            byte_length,
            count,
            &mut first_inversion,
            optimization as u32,
        )
    } != 0
    {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    let first_inversion = if first_inversion == u32::MAX {
        None
    } else if (first_inversion as usize) < values.len() {
        Some(first_inversion as usize)
    } else {
        return Err(GuestMemoryError::InvalidTraceEvent);
    };
    Ok(JitIsSortedResult {
        sorted: first_inversion.is_none(),
        first_inversion,
    })
}

fn is_sorted_trace_event(code: u32) -> Result<IsSortedTraceEvent, GuestMemoryError> {
    let (ast_node_id, operation) = match code {
        1 => ("is-sorted.left.read", IsSortedTraceOperation::Read),
        2 => ("is-sorted.right.read", IsSortedTraceOperation::Read),
        3 => (
            "is-sorted.adjacent.compare",
            IsSortedTraceOperation::Compare,
        ),
        _ => return Err(GuestMemoryError::InvalidTraceEvent),
    };
    Ok(IsSortedTraceEvent {
        ast_node_id,
        operation,
    })
}

pub fn interpret_minimum_i64(values: &[i64]) -> Result<Option<SelectionResult>, GuestMemoryError> {
    interpret_selection_i64(values, false)
}

pub fn interpret_maximum_i64(values: &[i64]) -> Result<Option<SelectionResult>, GuestMemoryError> {
    interpret_selection_i64(values, true)
}

fn interpret_selection_i64(
    values: &[i64],
    select_max: bool,
) -> Result<Option<SelectionResult>, GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(8)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let count = u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?;
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
    let mut raw = RawSelectionResult {
        found: 0,
        index: 0,
        value: 0,
    };
    let status = unsafe {
        atlas_mir_interpret_select_i64(
            memory.bytes.as_mut_ptr(),
            byte_length,
            count,
            u32::from(select_max),
            &mut raw,
        )
    };
    if status != 0 {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    match raw.found {
        0 if values.is_empty() => Ok(None),
        1 if (raw.index as usize) < values.len() => Ok(Some(SelectionResult {
            value: raw.value,
            index: raw.index as usize,
        })),
        _ => Err(GuestMemoryError::InvalidTraceEvent),
    }
}

pub fn interpret_reverse_i64(values: &mut [i64]) -> Result<(), GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(8)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let count = u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?;
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
    if unsafe { atlas_mir_interpret_reverse_i64(memory.bytes.as_mut_ptr(), byte_length, count) }
        != 0
    {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    for (index, value) in values.iter_mut().enumerate() {
        let offset = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_mul(8))
            .ok_or(GuestMemoryError::AddressOverflow)?;
        *value = memory.read_i64_le(GuestOffset::new(offset))?;
    }
    Ok(())
}

pub fn interpret_insertion_pairs(values: &mut [InsertionPair]) -> Result<(), GuestMemoryError> {
    let byte_length = values
        .len()
        .checked_mul(16)
        .ok_or(GuestMemoryError::AddressOverflow)?;
    let byte_length = u32::try_from(byte_length).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let count = u32::try_from(values.len()).map_err(|_| GuestMemoryError::AddressOverflow)?;
    let _guard = MIR_ADAPTER_LOCK
        .lock()
        .expect("MIR adapter lock must not be poisoned");
    let mut memory = OffsetMemory::new(byte_length as usize);
    for (index, value) in values.iter().copied().enumerate() {
        let offset = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_mul(16))
            .ok_or(GuestMemoryError::AddressOverflow)?;
        memory.write_i64_le(GuestOffset::new(offset), value.key)?;
        memory.write_i64_le(GuestOffset::new(offset + 8), value.original_index as i64)?;
    }
    if unsafe { atlas_mir_interpret_insertion_pairs(memory.bytes.as_mut_ptr(), byte_length, count) }
        != 0
    {
        return Err(GuestMemoryError::RuntimeFailure);
    }
    for (index, value) in values.iter_mut().enumerate() {
        let offset = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_mul(16))
            .ok_or(GuestMemoryError::AddressOverflow)?;
        value.key = memory.read_i64_le(GuestOffset::new(offset))?;
        value.original_index = memory.read_i64_le(GuestOffset::new(offset + 8))? as u64;
    }
    Ok(())
}

unsafe extern "C" {
    fn atlas_mir_interpret_add_u64(left: u64, right: u64) -> u64;
    fn atlas_mir_jit_add_u64_at_level(left: u64, right: u64, optimize_level: u32) -> u64;
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
    fn atlas_mir_interpret_is_sorted_i64(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
        trace: *mut RawIsSortedTrace,
    ) -> i32;
    fn atlas_mir_interpret_select_i64(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
        select_max: u32,
        selection: *mut RawSelectionResult,
    ) -> i32;
    fn atlas_mir_interpret_reverse_i64(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
    ) -> i32;
    fn atlas_mir_interpret_insertion_pairs(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
    ) -> i32;
    fn atlas_mir_jit_is_sorted_i64_at_level(
        guest_bytes: *mut u8,
        byte_length: u32,
        element_count: u32,
        first_inversion: *mut u32,
        optimize_level: u32,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use super::{
        CompareEvent, GuestMemoryError, GuestOffset, GuestRegionOffset, HandleMemory,
        JitIsSortedResult, JitOptimizationLevel, OffsetMemory, RegionMemory, interpret_add_u64,
        interpret_is_sorted_i64, interpret_maximum_i64, interpret_minimum_i64,
        interpret_minimum3_i64, interpret_partition_even_i64, jit_add_u64,
        jit_add_u64_with_optimization, jit_is_sorted_i64, jit_is_sorted_i64_with_optimization,
    };

    #[test]
    fn mir_interpreter_executes_a_scalar_function() {
        assert_eq!(interpret_add_u64(40, 2), 42);
        assert_eq!(interpret_add_u64(12, 30), 42);
    }

    #[test]
    fn mir_host_jit_matches_scalar_interpreter_and_rust() {
        for (left, right) in [(0, 0), (40, 2), (u64::MAX, 1)] {
            let expected = left.wrapping_add(right);
            assert_eq!(interpret_add_u64(left, right), expected);
            assert_eq!(jit_add_u64(left, right), expected);
        }
    }

    #[test]
    fn mir_host_jit_scalar_is_correct_at_every_documented_optimization_level() {
        for optimization in [
            JitOptimizationLevel::FastGeneration,
            JitOptimizationLevel::RegisterAllocation,
            JitOptimizationLevel::Default,
            JitOptimizationLevel::Full,
        ] {
            assert_eq!(jit_add_u64_with_optimization(u64::MAX, 2, optimization), 1);
        }
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
    fn mir_is_sorted_lowering_matches_native_and_ast_operations() {
        let ast = atlas::ast::is_sorted_ast();
        for values in [vec![], vec![42], vec![-2, 0, 0, 4], vec![3, 2, 1]] {
            let trace = interpret_is_sorted_i64(&values).expect("MIR is-sorted execution");
            let native = atlas_algorithms::is_sorted::is_sorted_by(&values, i64::cmp);
            let expected_inversion = values
                .windows(2)
                .position(|pair| pair[0] > pair[1])
                .map(|index| index + 1);

            assert_eq!(trace.sorted, native);
            assert_eq!(trace.first_inversion, expected_inversion);
            assert!(!trace.truncated);
            for event in &trace.events {
                let expected = match event.operation {
                    super::IsSortedTraceOperation::Read => atlas::ast::SemanticOperation::Read,
                    super::IsSortedTraceOperation::Compare => {
                        atlas::ast::SemanticOperation::Compare
                    }
                };
                assert_eq!(ast.operation_by_id(event.ast_node_id), Some(expected));
            }
        }
    }

    #[test]
    fn mir_is_sorted_stops_after_the_first_inversion() {
        let trace = interpret_is_sorted_i64(&[3, 2, 1]).expect("MIR is-sorted execution");

        assert!(!trace.sorted);
        assert_eq!(trace.first_inversion, Some(1));
        assert_eq!(trace.events.len(), 3);
        assert_eq!(
            trace
                .events
                .iter()
                .map(|event| event.ast_node_id)
                .collect::<Vec<_>>(),
            [
                "is-sorted.left.read",
                "is-sorted.right.read",
                "is-sorted.adjacent.compare",
            ]
        );
    }

    #[test]
    fn mir_host_jit_matches_is_sorted_interpreter_and_native() {
        for values in [vec![], vec![42], vec![-2, 0, 0, 4], vec![1, 5, 4, 6]] {
            let interpreted = interpret_is_sorted_i64(&values).expect("MIR interpreter");
            let generated = jit_is_sorted_i64(&values).expect("MIR host JIT");
            let native = atlas_algorithms::is_sorted::is_sorted_by(&values, i64::cmp);

            assert_eq!(generated.sorted, native);
            assert_eq!(generated.sorted, interpreted.sorted);
            assert_eq!(generated.first_inversion, interpreted.first_inversion);
        }
    }

    #[test]
    fn mir_host_jit_guest_scan_is_correct_at_every_documented_optimization_level() {
        let values = [1, 5, 4, 6];
        for optimization in [
            JitOptimizationLevel::FastGeneration,
            JitOptimizationLevel::RegisterAllocation,
            JitOptimizationLevel::Default,
            JitOptimizationLevel::Full,
        ] {
            assert_eq!(
                jit_is_sorted_i64_with_optimization(&values, optimization).expect("MIR host JIT"),
                JitIsSortedResult {
                    sorted: false,
                    first_inversion: Some(2),
                }
            );
        }
    }

    #[test]
    fn mir_selection_matches_native_values_and_first_ties() {
        for values in [vec![], vec![42], vec![7, -2, 4, -2], vec![3, 9, 9, 2]] {
            let minimum = interpret_minimum_i64(&values).expect("MIR minimum");
            let maximum = interpret_maximum_i64(&values).expect("MIR maximum");
            let native_minimum = atlas_algorithms::minimum::minimum_by(&values, i64::cmp);
            let native_maximum = atlas_algorithms::maximum::maximum_by(&values, i64::cmp);
            assert_eq!(minimum.map(|result| result.value), native_minimum.copied());
            assert_eq!(maximum.map(|result| result.value), native_maximum.copied());
            if let Some(result) = minimum {
                assert_eq!(values[result.index], result.value);
            }
            if let Some(result) = maximum {
                assert_eq!(values[result.index], result.value);
            }
        }
        assert_eq!(
            interpret_minimum_i64(&[7, -2, 4, -2])
                .unwrap()
                .unwrap()
                .index,
            1
        );
        assert_eq!(
            interpret_maximum_i64(&[3, 9, 9, 2]).unwrap().unwrap().index,
            1
        );
    }

    #[test]
    fn mir_reverse_matches_native_and_is_an_involution() {
        for original in [vec![], vec![42], vec![1, 2, 3, 4, 5]] {
            let mut mir = original.clone();
            super::interpret_reverse_i64(&mut mir).expect("MIR reverse");
            let mut native = original.clone();
            atlas_algorithms::reverse::reverse_in_place(&mut native);
            assert_eq!(mir, native);
            super::interpret_reverse_i64(&mut mir).expect("second MIR reverse");
            assert_eq!(mir, original);
        }
    }

    #[test]
    fn mir_insertion_pairs_match_native_stable_sort() {
        for keys in [vec![], vec![42], vec![5, -1, 5, 3, -1, 0]] {
            let original = keys
                .into_iter()
                .enumerate()
                .map(|(original_index, key)| super::InsertionPair {
                    key,
                    original_index: original_index as u64,
                })
                .collect::<Vec<_>>();
            let mut mir = original.clone();
            super::interpret_insertion_pairs(&mut mir).expect("MIR insertion sort");
            let mut native = original;
            atlas_algorithms::insertion_sort::insertion_sort_by(&mut native, |left, right| {
                left.key.cmp(&right.key)
            });

            assert_eq!(mir, native);
            assert!(mir.windows(2).all(|pair| pair[0].key <= pair[1].key));
            assert!(mir.windows(2).all(|pair| {
                pair[0].key != pair[1].key || pair[0].original_index < pair[1].original_index
            }));
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
