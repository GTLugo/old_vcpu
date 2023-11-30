use crate::error::MemoryError;
use crate::memory::{MemoryRead, MemoryWrite};

#[derive(Default, Debug)]
pub struct RAM {
  bytes: Vec<u8>,
}

impl RAM {
  pub fn new(bytes: Vec<u8>) -> Self {
    Self {
      bytes
    }
  }

  pub fn zero(&mut self) {
    self.bytes.fill(0);
  }
}

impl MemoryRead for RAM {
  // TODO: add error to elaborate WHY it's invalid
  fn read_u8(&self, address: u64) -> Result<u8, MemoryError> {
    self.bytes.get(address as usize)
      .ok_or(MemoryError::InvalidAddress(address))
      .copied()
  }
}

impl MemoryWrite for RAM {
  // TODO: add error to elaborate WHY it's invalid
  fn write_u8(&mut self, address: u64, value: u8) -> Result<(), MemoryError> {
    let byte = self.bytes
      .get_mut(address as usize)
      .ok_or(MemoryError::InvalidAddress(address))?;
    *byte = value;
    Ok(())
  }
}