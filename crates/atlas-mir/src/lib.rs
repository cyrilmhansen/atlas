//! Experimental MIR adapter boundary for MVP 4.
//!
//! Atlas semantics remain outside this crate. The adapter currently proves one
//! scalar MIR interpreter call and compares guest-reference representations.

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
    InvalidHandle,
    InvalidRegion,
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

unsafe extern "C" {
    fn atlas_mir_interpret_add_u64(left: u64, right: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use super::{
        GuestMemoryError, GuestOffset, GuestRegionOffset, HandleMemory, OffsetMemory, RegionMemory,
        interpret_add_u64,
    };

    #[test]
    fn mir_interpreter_executes_a_scalar_function() {
        assert_eq!(interpret_add_u64(40, 2), 42);
        assert_eq!(interpret_add_u64(12, 30), 42);
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
