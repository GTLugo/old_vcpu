use strum::Display;
use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::error::CpuError;
use crate::memory::MemoryIO;

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
  pub const OPCODE_IMMEDIATE: u8 = 0xA9;
}

impl Instruction for LDA {
  fn cycles(&self) -> u32 {
    match self {
      LDA::Immediate(_value)   => 2,
      LDA::ZeroPage(_address)  => 3,
      LDA::ZeroPageX(_address) => 4,
      LDA::Absolute(_address)  => 4,
      LDA::AbsoluteX(_address) => { 4 }
      LDA::AbsoluteY(_address) => { 4 }
      LDA::IndirectX(_address) => 6,
      LDA::IndirectY(_address) => { 5 }
    }
  }

  fn opcode(&self) -> u8 {
    match self {
      LDA::Immediate(_value)   => LDA::OPCODE_IMMEDIATE,
      LDA::ZeroPage(_address)  => 0xA5,
      LDA::ZeroPageX(_address) => 0xB5,
      LDA::Absolute(_address)  => 0xAD,
      LDA::AbsoluteX(_address) => 0xBD,
      LDA::AbsoluteY(_address) => 0xB9,
      LDA::IndirectX(_address) => 0xA1,
      LDA::IndirectY(_address) => 0xB1,
    }
  }

  fn decode(
    cpu: &mut CPU,
    memory: &dyn MemoryIO,
    opcode: u8
  ) -> Result<Self, CpuError> {
    match opcode {
      LDA::OPCODE_IMMEDIATE => {
        let immediate = cpu.fetch(memory)?;
        Ok(LDA::Immediate(immediate))
      }
      _ => Err(CpuError::InvalidOpCode(opcode))
    }
  }

  fn execute(&self, cpu: &mut CPU, _memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    match self {
      LDA::Immediate(value) => {
        cpu.accumulator = *value;
      }
      LDA::ZeroPage(_)  => return Err(CpuError::Unspecified),
      LDA::ZeroPageX(_) => return Err(CpuError::Unspecified),
      LDA::Absolute(_)  => return Err(CpuError::Unspecified),
      LDA::AbsoluteX(_) => return Err(CpuError::Unspecified),
      LDA::AbsoluteY(_) => return Err(CpuError::Unspecified),
      LDA::IndirectX(_) => return Err(CpuError::Unspecified),
      LDA::IndirectY(_) => return Err(CpuError::Unspecified),
    }

    Ok(())
  }

  fn debug(&self) -> String {
    match self {
      LDA::Immediate(value) => format!("LDA #${value:X}"),
      LDA::ZeroPage(_)  => "LDA".to_string(),
      LDA::ZeroPageX(_) => "LDA".to_string(),
      LDA::Absolute(_)  => "LDA".to_string(),
      LDA::AbsoluteX(_) => "LDA".to_string(),
      LDA::AbsoluteY(_) => "LDA".to_string(),
      LDA::IndirectX(_) => "LDA".to_string(),
      LDA::IndirectY(_) => "LDA".to_string(),
    }
  }
}
