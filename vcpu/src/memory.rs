use crate::error::MemoryError;

pub mod ram;
pub mod rom;
pub mod disk;
pub mod mmu;

// TODO: Research making this a trait instead to allow more custom Memory implementations


pub trait MemoryRead {
  fn read_u8(&self, address: u64) -> Result<u8, MemoryError>;
  fn read_u16(&self, address: u64) -> Result<u16, MemoryError> {
    let lo = self.read_u8(address)?;
    let hi = self.read_u8(address + 1)?;
    Ok(u16::from_le_bytes([lo, hi]))
  }
  fn read_u32(&self, address: u64) -> Result<u32, MemoryError> {
    let mut bytes = [0; 4];
    bytes[0..2].copy_from_slice(&self.read_u16(address)?.to_le_bytes());
    bytes[2..4].copy_from_slice(&self.read_u16(address + 2)?.to_le_bytes());
    Ok(u32::from_le_bytes(bytes))
  }
  fn read_u64(&self, address: u64) -> Result<u64, MemoryError> {
    let mut bytes = [0; 8];
    bytes[0..4].copy_from_slice(&self.read_u16(address)?.to_le_bytes());
    bytes[4..8].copy_from_slice(&self.read_u16(address + 4)?.to_le_bytes());
    Ok(u64::from_le_bytes(bytes))
  }
  fn read_u128(&self, address: u64) -> Result<u128, MemoryError> {
    let mut bytes = [0; 16];
    bytes[0..8].copy_from_slice(&self.read_u16(address)?.to_le_bytes());
    bytes[8..16].copy_from_slice(&self.read_u16(address + 8)?.to_le_bytes());
    Ok(u128::from_le_bytes(bytes))
  }
}

pub trait MemoryWrite {
  fn write_u8(&mut self, address: u64, value: u8) -> Result<(), MemoryError>;
  fn write_u16(&mut self, address: u64, value: u16) -> Result<(), MemoryError> {
    self.write_bytes(address, &value.to_le_bytes())
  }
  fn write_u32(&mut self, address: u64, value: u32) -> Result<(), MemoryError> {
    self.write_bytes(address, &value.to_le_bytes())
  }
  fn write_u64(&mut self, address: u64, value: u64) -> Result<(), MemoryError> {
    self.write_bytes(address, &value.to_le_bytes())
  }
  fn write_u128(&mut self, address: u64, value: u128) -> Result<(), MemoryError> {
    self.write_bytes(address, &value.to_le_bytes())
  }
  fn write_bytes(&mut self, address: u64, bytes: &[u8]) -> Result<(), MemoryError> {
    bytes.iter()
      .enumerate()
      .try_for_each(|(i, byte)| {
        let address = address.checked_add(i as u64)
          .ok_or_else(|| MemoryError::MemoryOverflow)?;
        self.write_u8(address, *byte)
      })
  }
}
