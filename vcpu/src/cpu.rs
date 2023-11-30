use tracing::{info, trace};
use bau::instruction::Instruction;

use crate::cpu::status::Status;
use crate::error::CpuError;
use crate::memory::MemoryRead;
use crate::memory::mmu::MMU;
use crate::time::Time;

mod status;

pub struct Clock {

}

pub trait ClockObserver {

}

pub struct DataBus {
  // TODO: Data will be sent onto the bus on the clock tick and read from the bus on the clock tick
}

pub trait DataBusObserver {

}

// memory_bus will be a u128. send/receive data from memory address sent
// address_bus will be a u64. send address to r/w to/from
#[derive(Debug)]
pub struct CPU {
  status: Status,

  // special registers: will have first bit set
  zero_register: u128,        // always holds zero
  one_register: u128,         // always holds one
  two_register: u128,         // always holds two
  program_counter: u64,       // each memory address points to 1 byte, therefore increment by 16
  instruction_register: u128, // current fetched instruction
  stack_pointer: u64,         //
  frame_pointer: u64,         // base of the current stack frame
  return_address: u64,        // address to return to after subroutine
  // general registers
  registers: [u128; 26],
}

impl CPU {
  const INSTRUCTION_WIDTH: u64 = 16;

  pub fn new(reset_address: u64) -> Self {
    info!("initializing cpu");
    let mut cpu = Self {
      ..Default::default()
    };

    cpu.reset();

    cpu
  }

  pub fn continuous_step(
    &mut self,
    memory: &mut MMU,
    clock_speed: f64,
  ) -> Result<(), CpuError> {
    let mut time = Time::new(clock_speed, 1024);
    let mut cycles_until_next_instruction = 0;
    loop {
      if !time.should_execute_next_cycle() {
        continue;
      }

      cycles_until_next_instruction = match cycles_until_next_instruction {
        0 => self.step(memory)? - 1,
        _ => {
          // debug!("|");
          cycles_until_next_instruction - 1
        }
      }
    }
  }

  pub fn step(&mut self, memory: &mut MMU) -> Result<u32, CpuError> {
    // debug!("{self:X?}");
    // let pc = self.program_counter;
    let instruction = self.fetch(memory)?;
    let instruction = Instruction::decode(instruction);
    // trace!("[{pc:04X}]: {}", instruction.debug());
    // instruction.execute(self, memory)?;
    // Ok(instruction.cycles())
    Err(CpuError::Unimplemented)
  }
}

impl CPU {
  fn reset(&mut self) {
    info!("resetting cpu");
  }

  fn fetch(&mut self, memory: &MMU) -> Result<u128, CpuError> {
    let instruction = memory.read_u128(self.program_counter)?;
    self.program_counter += Self::INSTRUCTION_WIDTH;
    Ok(instruction)
  }

  fn execute(&mut self, memory: &mut MMU, instruction: Instruction) -> Result<(), CpuError> {
    match instruction {
      Instruction::Null => Ok(()),
      Instruction::SetLo { .. } => Err(CpuError::Unimplemented),
      Instruction::SetHi { .. } => Err(CpuError::Unimplemented),
      Instruction::Load { .. } => Err(CpuError::Unimplemented),
      Instruction::Store { .. } => Err(CpuError::Unimplemented),
    }
  }
}

impl Default for CPU {
  fn default() -> Self {
    Self {
      status: Default::default(),
      zero_register: 0,
      one_register: 1,
      two_register: 2,
      program_counter: 0,
      instruction_register: 0,
      stack_pointer: 0,
      frame_pointer: 0,
      return_address: 0,
      registers: [0; 26],
    }
  }
}
