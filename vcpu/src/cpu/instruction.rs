pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod nop;
pub mod sta;

use crate::cpu::CPU;
use crate::error::CpuError;
use crate::memory::MemoryIO;

pub trait Instruction {
  fn cycles(&self) -> u32;
  fn opcode(&self) -> u8;
  fn decode(cpu: &mut CPU, memory: &dyn MemoryIO, opcode: u8)
    -> Result<Box<dyn Instruction>, CpuError>
    where
      Self: Sized;
  fn execute(&self, cpu: &mut CPU, memory: &mut dyn MemoryIO) -> Result<(), CpuError>;
  fn debug(&self) -> String {
    self.opcode().to_string().to_uppercase()
  }
}

// #[macro_export]
// pub macro_rules! try_decode_instructions {
//   ($cpu:expr, $memory:expr, $opcode:expr, $($Instr:ty),*) => {{
//     $(
//       if let Ok(instruction) = <$Instr>::decode($cpu, $memory, $opcode) {
//         return Ok(Box::new(instruction));
//       }
//     )*
//     Err(CpuError::InvalidOpCode($opcode))
//   }};
// }