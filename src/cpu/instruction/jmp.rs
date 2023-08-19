use strum::Display;
use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::error::CpuError;
use crate::memory::MemoryIO;

#[derive(Debug, Display)]
pub enum JMP {
  Absolute(u16),
  Indirect(u16),
}

impl JMP {
  pub const OPCODE_ABSOLUTE: u8 = 0x4C;
  pub const OPCODE_INDIRECT: u8 = 0x6C;
}

impl Instruction for JMP {
  fn cycles(&self) -> u32 {
    match self {
      Self::Absolute(_address) => 3,
      Self::Indirect(_address) => 5,
    }
  }

  fn opcode(&self) -> u8 {
    match self {
      Self::Absolute(_address) => Self::OPCODE_ABSOLUTE,
      Self::Indirect(_address) => Self::OPCODE_INDIRECT,
    }
  }

  fn decode(
    cpu: &mut CPU,
    memory: &dyn MemoryIO,
    opcode: u8
  ) -> Result<Self, CpuError> {
    match opcode {
      Self::OPCODE_ABSOLUTE => {
        let lo = cpu.fetch(memory)?;
        let hi = cpu.fetch(memory)?;
        Ok(Self::Absolute(u16::from_le_bytes([lo, hi])))
      }
      Self::OPCODE_INDIRECT => {
        let lo = cpu.fetch(memory)?;
        let hi = cpu.fetch(memory)?;
        Ok(Self::Indirect(u16::from_le_bytes([lo, hi])))
      }
      _ => Err(CpuError::InvalidOpCode(opcode))
    }
  }

  fn execute(&self, cpu: &mut CPU, _memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    match self {
      Self::Absolute(address) => {
        cpu.program_counter = *address;
      }
      Self::Indirect(address) => {
        cpu.program_counter = *address;
      }
    }

    Ok(())
  }

  fn debug(&self) -> String {
    match self {
      Self::Absolute(address) => format!("JMP ${address:X}"),
      Self::Indirect(address) => format!("JMP (${address:X})"),
    }
  }
}
