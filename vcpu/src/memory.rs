use std::cmp::Ordering;

use tracing::info;

use crate::error::MemoryError;

// TODO: Research making this a trait instead to allow more custom Memory implementations


pub trait MemoryIO {
  fn read(&self, address: u16) -> Result<u8, MemoryError>;
  fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;
  fn zero(&mut self);
}

#[derive(Debug)]
pub struct Memory {
  bytes: Vec<u8>,
  read_only_begin: u16,
}

impl Memory {
  // 8-bit can only access up to 64k
  pub const MAX: usize = 1024 * 64;

  pub fn new(byte_slice: &[u8], read_only_begin: u16) -> Result<Self, MemoryError> {
    let bytes = match byte_slice.len().cmp(&Self::MAX) {
      Ordering::Less => {
        let mut vec = byte_slice.to_vec();
        vec.extend(vec![0x00; Self::MAX - byte_slice.len()]);
        vec
      }
      Ordering::Equal => byte_slice.to_vec(),
      Ordering::Greater => return Err(MemoryError::MemoryLimitExceeded),
    };

    Ok(Self {
      bytes,
      read_only_begin,
    })
  }

  pub fn dump(&self) -> String {
    self.bytes.iter().enumerate().fold(String::new(), |acc, (i, x)| {
      match (i + 1) % 16 {
        0 => format!("{acc}{x:02X}\n"),
        1 => format!("{acc}[{i:04X}] {x:02X} "),
        4 | 8 | 12 => format!("{acc}{x:02X}  "),
        _ => format!("{acc}{x:02X} "),
      }
    })
  }
}

impl MemoryIO for Memory {
  fn read(&self, address: u16) -> Result<u8, MemoryError> {
    self.bytes
      .get(address as usize)
      .ok_or(MemoryError::InvalidAddress(address))
      .copied()
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
      None => Err(MemoryError::InvalidAddress(address)),
    }
  }

  fn zero(&mut self) {
    info!("zeroing memory");
    let ram = &mut self.bytes[0..self.read_only_begin as usize];
    ram.fill(0);
  }
}

impl Default for Memory {
  fn default() -> Self {
    Self {
      bytes: vec![0x00; Self::MAX],
      read_only_begin: 0xFFFC,
    }
  }
}
