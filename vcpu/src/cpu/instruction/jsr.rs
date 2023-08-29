use crate::cpu::instruction::Instruction;
use crate::cpu::CPU;
use crate::error::CpuError;
use crate::memory::MemoryIO;
use strum::Display;

#[derive(Debug, Display)]
pub enum JSR {
  Absolute(u16),
}

impl JSR {
  pub const OPCODE_ABSOLUTE: u8 = 0x20;
}

impl Instruction for JSR {
  fn cycles(&self) -> u32 {
    match self {
      Self::Absolute(_address) => 6,
    }
  }

  fn opcode(&self) -> u8 {
    match self {
      Self::Absolute(_address) => Self::OPCODE_ABSOLUTE,
    }
  }

  fn decode(cpu: &mut CPU, memory: &dyn MemoryIO, opcode: u8) -> Result<Box<dyn Instruction>, CpuError> {
    match opcode {
      Self::OPCODE_ABSOLUTE => Ok(Box::new(Self::Absolute(cpu.fetch_word(memory)?))),
      _ => Err(CpuError::InvalidOpCode(opcode)),
    }
  }

  fn execute(&self, cpu: &mut CPU, memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    match self {
      Self::Absolute(address) => {
        cpu.stack_push_word(memory, cpu.program_counter)?;
        cpu.program_counter = *address;
      }
    }
    Ok(())
  }

  fn debug(&self) -> String {
    match self {
      Self::Absolute(address) => format!("JSR ${address:X}"),
    }
  }
}
