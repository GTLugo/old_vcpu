use tracing::info;

use crate::error::MemoryError;
use crate::memory::{MemoryRead, MemoryWrite};
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;

pub trait Device: MemoryWrite + MemoryRead {}

pub struct MMU {
  rom: ROM,
  ram: RAM,
}

impl MMU {
  // Memory Map

  const NULL_BEGIN     : u64 = 0x0000_0000_0000_0000;
  const NULL_END       : u64 = 0x0000_0000_0000_000F;

  const SYSTEM_BEGIN   : u64 = 0x0000_0000_0000_0010;
  const SYSTEM_END     : u64 = 0x0000_0000_000F_FFFF;

  const ROM_BEGIN      : u64 = 0x0000_0000_0010_0000;
  const ROM_END        : u64 = 0x0000_0001_FFFF_FFFF;

  const HARDWARE_BEGIN : u64 = 0x0000_0002_0000_0000;
  const HARDWARE_END   : u64 = 0x0000_0002_FFFF_FFFF;

  const RAM_BEGIN      : u64 = 0x0000_0003_0000_0000;
  const RAM_END        : u64 = 0xFFFF_FFFF_FFFF_FFFF;

  pub fn new(rom: ROM, ram_size: u64) -> Self {
    Self {
      rom,
      ram: RAM::new(vec![0u8; ram_size as usize])
    }
  }

  pub fn zero(&mut self) {
    info!("zeroing memory");
    self.ram.zero();
  }
}

impl MemoryRead for MMU {
  fn read_u8(&self, address: u64) -> Result<u8, MemoryError> {
    match address {
      Self::NULL_BEGIN..=Self::NULL_END     => Err(MemoryError::NullAddress(address)),
      Self::SYSTEM_BEGIN..=Self::SYSTEM_END => Err(MemoryError::ReservedAddress(address)),
      Self::ROM_BEGIN..=Self::ROM_END       => {
        self.rom.read_u8(address - Self::ROM_BEGIN)
          .map_err(|err| match err {
            MemoryError::InvalidAddress(_) => MemoryError::InvalidAddress(address),
            _ => err,
          })
      },
      Self::HARDWARE_BEGIN..=Self::HARDWARE_END => Err(MemoryError::ReservedAddress(address)),
      Self::RAM_BEGIN..=Self::RAM_END           => {
        self.ram.read_u8(address - Self::RAM_BEGIN)
          .map_err(|err| match err {
            MemoryError::InvalidAddress(_) => MemoryError::InvalidAddress(address),
            _ => err,
          })
      },
      _ => Err(MemoryError::InvalidAddress(address))
    }
  }
}

impl MemoryWrite for MMU {
  fn write_u8(&mut self, address: u64, value: u8) -> Result<(), MemoryError> {
    match address {
      Self::NULL_BEGIN..=Self::NULL_END         => Err(MemoryError::NullAddress(address)),
      Self::SYSTEM_BEGIN..=Self::SYSTEM_END     => Err(MemoryError::ReservedAddress(address)),
      Self::ROM_BEGIN..=Self::ROM_END           => Err(MemoryError::WriteToReadOnlyAddress(address)),
      Self::HARDWARE_BEGIN..=Self::HARDWARE_END => Err(MemoryError::ReservedAddress(address)),
      Self::RAM_BEGIN..=Self::RAM_END           => {
        self.ram.write_u8(address - Self::RAM_BEGIN, value)
          .map_err(|err| match err {
            MemoryError::InvalidAddress(_) => MemoryError::InvalidAddress(address),
            _ => err,
          })
      },
      _ => Err(MemoryError::InvalidAddress(address))
    }
  }
}