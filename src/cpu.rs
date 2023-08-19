use tracing::{debug, info};

use crate::cpu::instruction::Instruction;
use crate::cpu::instruction::jmp::JMP;
use crate::cpu::instruction::lda::LDA;
use crate::cpu::instruction::nop::NOP;
use crate::cpu::status::Status;
use crate::error::CpuError;
use crate::memory::MemoryIO;
use crate::time::Time;

mod status;
pub mod instruction;

#[derive(Debug)]
pub struct CPU {
  reset_vector: u16,
  program_counter: u16,
  stack_pointer: u8,
  accumulator: u8,
  x: u8,
  y: u8,
  status: Status,
  time: Time,
}

impl CPU {
  pub fn new(reset_vector: u16, clock_speed: f64) -> Self {
    info!("initializing cpu");
    Self {
      reset_vector,
      time: Time::new(clock_speed, 1024),
      ..Default::default()
    }
  }

  pub fn reset(&mut self, memory: &mut dyn MemoryIO) {
    info!("resetting cpu");
    self.program_counter = self.reset_vector;

    self.stack_pointer = 0x00;
    self.accumulator = 0x00;
    self.x = 0x00;
    self.y = 0x00;

    self.status.reset();

    memory.zero();
  }

  pub fn continuous_step(&mut self, memory: &mut dyn MemoryIO) -> Result<(), CpuError> {
    let mut cycles = 0;
    loop {
      self.time.update();
      while self.time.should_do_tick() {
        self.time.tick();
        if cycles == 0 {
          cycles = self.step(memory)? - 1;
        } else {
          debug!("...");
          cycles -= 1;
        }
      }
    }
  }

  pub fn step(&mut self, memory: &mut dyn MemoryIO) -> Result<u32, CpuError> {
    let pc = self.program_counter;
    let opcode = self.fetch(memory)?;
    let instruction = self.decode(memory, opcode)?;
    debug!("[{pc:04X}]: {}", instruction.debug());
    instruction.execute(self, memory)?;
    Ok(instruction.cycles())
  }

  fn fetch(&mut self, memory: &dyn MemoryIO) -> Result<u8, CpuError> {
    let byte = *memory.read(self.program_counter)?;
    self.program_counter = self.program_counter.checked_add(1).ok_or(CpuError::ProgramCounterOverflow)?;
    Ok(byte)
  }

  fn decode(&mut self, memory: &dyn MemoryIO, opcode: u8) -> Result<Box<dyn Instruction>, CpuError> {
    match opcode {
      LDA::OPCODE_IMMEDIATE   |
      LDA::OPCODE_ZERO_PAGE   |
      LDA::OPCODE_ZERO_PAGE_X |
      LDA::OPCODE_ABSOLUTE    |
      LDA::OPCODE_ABSOLUTE_X  |
      LDA::OPCODE_ABSOLUTE_Y  |
      LDA::OPCODE_INDIRECT_X  |
      LDA::OPCODE_INDIRECT_Y => Ok(Box::new(LDA::decode(self, memory, opcode)?)),
      NOP::OPCODE => Ok(Box::new(NOP::decode(self, memory, opcode)?)),
      JMP::OPCODE_ABSOLUTE |
      JMP::OPCODE_INDIRECT => Ok(Box::new(JMP::decode(self, memory, opcode)?)),
      _ => Err(CpuError::InvalidOpCode(opcode))
    }
  }
}

impl Default for CPU {
  fn default() -> Self {
    Self {
      reset_vector: 0x8000,
      program_counter: 0xFFFC,
      stack_pointer: 0x00,
      accumulator: 0x00,
      x: 0x00,
      y: 0x00,
      status: Default::default(),
      time: Time::new(1000.0, 1024),
    }
  }
}
