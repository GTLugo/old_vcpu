pub mod lda;
pub mod nop;

use crate::cpu::CPU;
use crate::error::CpuError;
use crate::memory::MemoryIO;

pub trait Instruction {
  fn cycles(&self) -> u32;
  fn opcode(&self) -> u8;
  fn decode(cpu: &mut CPU, memory: &dyn MemoryIO, opcode: u8) -> Result<Self, CpuError> where Self: Sized;
  fn execute(&self, cpu: &mut CPU, memory: &mut dyn MemoryIO) -> Result<(), CpuError>;
  fn debug(&self) -> String {
    self.opcode()
      .to_string()
      .to_uppercase()
  }
}
