use crate::cpu::instruction::Instruction;
use crate::cpu::CPU;
use crate::error::CpuError;
use crate::memory::MemoryIO;
use strum::Display;

#[derive(Debug, Display)]
pub enum LDA {
  Immediate(u8),
  ZeroPage(u8),
  ZeroPageX(u8),
  Absolute(u16),
  AbsoluteX(u16),
  AbsoluteY(u16),
  IndirectX(u8),
  IndirectY(u8),
}

impl LDA {
  pub const OPCODE_IMMEDIATE: u8   = 0xA9;
  pub const OPCODE_ZERO_PAGE: u8   = 0xA5;
  pub const OPCODE_ZERO_PAGE_X: u8 = 0xB5;
  pub const OPCODE_ABSOLUTE: u8    = 0xAD;
  pub const OPCODE_ABSOLUTE_X: u8  = 0xBD;
  pub const OPCODE_ABSOLUTE_Y: u8  = 0xB9;
  pub const OPCODE_INDIRECT_X: u8  = 0xA1;
  pub const OPCODE_INDIRECT_Y: u8  = 0xB1;
}

impl Instruction for LDA {
  fn cycles(&self) -> u32 {
    match self {
      Self::Immediate(_)        => 2,
      Self::ZeroPage(_)         => 3,
      Self::ZeroPageX(_)        => 4,
      Self::Absolute(_)         => 4,
      Self::AbsoluteX(_address) => 4,
      Self::AbsoluteY(_address) => 4,
      Self::IndirectX(_)        => 6,
      Self::IndirectY(_address) => 5,
    }
  }

  fn opcode(&self) -> u8 {
    match self {
      Self::Immediate(_) => Self::OPCODE_IMMEDIATE,
      Self::ZeroPage(_)  => Self::OPCODE_ZERO_PAGE,
      Self::ZeroPageX(_) => Self::OPCODE_ZERO_PAGE_X,
      Self::Absolute(_)  => Self::OPCODE_ABSOLUTE,
      Self::AbsoluteX(_) => Self::OPCODE_ABSOLUTE_X,
      Self::AbsoluteY(_) => Self::OPCODE_ABSOLUTE_Y,
      Self::IndirectX(_) => Self::OPCODE_INDIRECT_X,
      Self::IndirectY(_) => Self::OPCODE_INDIRECT_Y,
    }
  }

  fn decode(cpu: &mut CPU, memory: &dyn MemoryIO, opcode: u8) -> Result<Box<dyn Instruction>, CpuError> {
    match opcode {
      Self::OPCODE_IMMEDIATE => {
        let immediate = cpu.fetch(memory)?;
        Ok(Box::new(Self::Immediate(immediate)))
      }
      _ => Err(CpuError::InvalidOpCode(opcode)),
    }
  }

  fn execute(&self, cpu: &mut CPU, _memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    match self {
      Self::Immediate(value) => {
        cpu.accumulator = *value;
      }
      Self::ZeroPage(_)  => return Err(CpuError::Unspecified),
      Self::ZeroPageX(_) => return Err(CpuError::Unspecified),
      Self::Absolute(_)  => return Err(CpuError::Unspecified),
      Self::AbsoluteX(_) => return Err(CpuError::Unspecified),
      Self::AbsoluteY(_) => return Err(CpuError::Unspecified),
      Self::IndirectX(_) => return Err(CpuError::Unspecified),
      Self::IndirectY(_) => return Err(CpuError::Unspecified),
    }

    Ok(())
  }

  fn debug(&self) -> String {
    match self {
      Self::Immediate(value) => format!("LDA #${value:X}"),
      Self::ZeroPage(_)  => "LDA".to_string(),
      Self::ZeroPageX(_) => "LDA".to_string(),
      Self::Absolute(_)  => "LDA".to_string(),
      Self::AbsoluteX(_) => "LDA".to_string(),
      Self::AbsoluteY(_) => "LDA".to_string(),
      Self::IndirectX(_) => "LDA".to_string(),
      Self::IndirectY(_) => "LDA".to_string(),
    }
  }
}
