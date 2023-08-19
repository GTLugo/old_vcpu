use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;
use crate::error::CpuError;
use crate::memory::MemoryIO;

#[derive(Debug)]
pub struct NOP;

impl NOP {
  pub const OPCODE: u8 = 0xEA;
}

impl Instruction for NOP {
  fn cycles(&self) -> u32 {
    2
  }

  fn opcode(&self) -> u8 {
    Self::OPCODE
  }

  fn decode(
    _cpu: &mut CPU,
    _memory: &dyn MemoryIO,
    opcode: u8
  ) -> Result<Self, CpuError> {
    match opcode {
      Self::OPCODE => Ok(Self),
      _ => Err(CpuError::InvalidOpCode(opcode))
    }
  }

  fn execute(&self, _cpu: &mut CPU, _memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    // TODO: If changing to fetch cycles, then keep in mind NOP should take 2 CPU cycles to execute.
    Ok(())
  }

  fn debug(&self) -> String {
    "NOP".to_string()
  }
}
