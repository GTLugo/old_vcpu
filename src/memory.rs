use tracing::info;
use crate::error::MemoryError;

// TODO: Research making this a trait instead to allow more custom Memory implementations

pub const SIZE_1KB: usize = 1024;
pub const SIZE_64KB: usize = 64 * SIZE_1KB;

pub trait MemoryIO {
  fn read(&self, address: u16) -> Result<&u8, MemoryError>;
  fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;
  fn zero(&mut self);
}

#[derive(Debug)]
pub struct Memory<const MAX_MEMORY: usize> {
  bytes: [u8; MAX_MEMORY],
  read_only_begin: u16,
}

impl<const MAX_MEMORY: usize> Memory<MAX_MEMORY> { // 8-bit can only access up to 64k
  pub fn new(bytes: [u8; MAX_MEMORY], read_only_begin: u16) -> Self {
    Self {
      bytes,
      read_only_begin
    }
  }
}

impl<const MAX_MEMORY: usize> MemoryIO for Memory<MAX_MEMORY> {
  fn read(&self, address: u16) -> Result<&u8, MemoryError> {
    self.bytes.get(address as usize).ok_or(MemoryError::InvalidAddress(address))
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
    match self.bytes.get_mut(address as usize) {
      Some(v) => {
        if address < self.read_only_begin {
          *v = value;
          Ok(())
        } else {
          Err(MemoryError::WriteToRomAddress(address))
        }
      }
      None => {
        Err(MemoryError::InvalidAddress(address))
      }
    }
  }

  fn zero(&mut self) {
    info!("zeroing memory");
    let ram = &mut self.bytes[0..self.read_only_begin as usize];
    ram.fill(0);
  }
}

impl<const MAX_MEMORY: usize> Default for Memory<MAX_MEMORY> {
  fn default() -> Self {
    Self {
      bytes: [0x00; MAX_MEMORY],
      read_only_begin: 0xFFFC,
    }
  }
}