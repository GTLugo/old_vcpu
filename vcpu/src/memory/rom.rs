use crate::error::MemoryError;
use crate::memory::MemoryRead;

#[derive(Default, Debug)]
pub struct ROM {
  bytes: Vec<u8>,
}

impl ROM {
  pub fn new(bytes: Vec<u8>) -> Self {
    Self {
      bytes
    }
  }

  pub fn bytes(&self) -> &[u8] {
    &self.bytes
  }
}

impl MemoryRead for ROM {
  // TODO: add error to elaborate WHY it's invalid
  fn read_u8(&self, address: u64) -> Result<u8, MemoryError> {
    self.bytes.get(address as usize)
      .ok_or(MemoryError::InvalidAddress(address))
      .copied()
  }
}