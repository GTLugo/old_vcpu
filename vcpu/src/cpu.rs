use tracing::{info, trace};

use crate::cpu::instruction::jmp::JMP;
use crate::cpu::instruction::lda::LDA;
use crate::cpu::instruction::nop::NOP;
use crate::cpu::instruction::Instruction;
use crate::cpu::instruction::jsr::JSR;
use crate::cpu::instruction::sta::STA;
use crate::cpu::status::Status;
use crate::error::CpuError;
use crate::memory::MemoryIO;
use crate::time::Time;

pub mod instruction;
mod status;

type InstructionFn = fn(&mut CPU, &dyn MemoryIO, u8) -> Result<Box<dyn Instruction>, CpuError>;

#[derive(Default, Debug)]
pub struct CPU {
  reset_vector: u16,
  program_counter: u16,
  stack_pointer: u8,

  accumulator: u8,
  x: u8,
  y: u8,

  status: Status,
}

impl CPU {
  const INSTRUCTIONS: &'static [InstructionFn] = &[
    LDA::decode,
    NOP::decode,
    JMP::decode,
    JSR::decode,
    STA::decode,
  ];

  pub fn new(reset_vector: u16) -> Self {
    info!("initializing cpu");
    let mut cpu = Self {
      reset_vector,
      ..Default::default()
    };

    cpu.reset();

    cpu
  }

  pub fn continuous_step(
    &mut self,
    memory: &mut dyn MemoryIO,
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

  pub fn step(&mut self, memory: &mut dyn MemoryIO) -> Result<u32, CpuError> {
    // debug!("{self:X?}");
    let pc = self.program_counter;
    let opcode = self.fetch(memory)?;
    let instruction = self.decode(memory, opcode)?;
    trace!("[{pc:04X}]: {}", instruction.debug());
    instruction.execute(self, memory)?;
    Ok(instruction.cycles())
  }

  pub fn dump(&self) -> String {
    format!(
      "RESET_VECTOR: ${:04X}\
       \nCPU Registers {{\
       \n  PC : ${:04X}\
       \n  SP : ${:02X}\
       \n  A  : ${:02X}\
       \n  X  : ${:02X}\
       \n  Y  : ${:02X}\
       \n}}\
       \nCPU Status {{\
       \n  CARRY             : {}\
       \n  ZERO              : {}\
       \n  INTERRUPT_DISABLE : {}\
       \n  DECIMAL           : {}\
       \n  BREAK_COMMAND     : {}\
       \n  OVERFLOW          : {}\
       \n  NEGATIVE          : {}\
       \n}}",
      self.reset_vector,
      self.program_counter,
      self.stack_pointer,
      self.accumulator,
      self.x,
      self.y,
      self.status.carry as i32,
      self.status.zero as i32,
      self.status.interrupt_disable as i32,
      self.status.decimal as i32,
      self.status.break_command as i32,
      self.status.overflow as i32,
      self.status.negative as i32
    )
  }
}

impl CPU {
  fn reset(&mut self) {
    info!("resetting cpu");
    self.program_counter = self.reset_vector;

    self.stack_pointer = 0x00;
    self.accumulator = 0x00;
    self.x = 0x00;
    self.y = 0x00;

    self.status.reset();
  }

  fn fetch(&mut self, memory: &dyn MemoryIO) -> Result<u8, CpuError> {
    let byte = memory.read(self.program_counter)?;
    self.program_counter = self
      .program_counter
      .checked_add(1)
      .ok_or(CpuError::ProgramCounterOverflow)?;
    Ok(byte)
  }

  fn fetch_word(&mut self, memory: &dyn MemoryIO) -> Result<u16, CpuError> {
    let lo = self.fetch(memory)?;
    let hi = self.fetch(memory)?;
    Ok(u16::from_le_bytes([lo, hi]))
  }

  fn decode(
    &mut self,
    memory: &dyn MemoryIO,
    opcode: u8,
  ) -> Result<Box<dyn Instruction>, CpuError> {
    for decoder in Self::INSTRUCTIONS {
      if let Ok(instruction) = decoder(self, memory, opcode) {
        return Ok(instruction);
      }
    }

    Err(CpuError::InvalidOpCode(opcode))
  }

  fn stack_address(&self) -> u16 {
    u16::from_le_bytes([self.stack_pointer, 0x01])
  }

  fn stack_push(&mut self, memory: &mut dyn MemoryIO, value: u8) -> Result<(), CpuError> {
    let stack_address = self.stack_address();
    memory.write(stack_address, value)?;
    self.stack_pointer = self
      .stack_pointer
      .checked_add(1)
      .ok_or(CpuError::StackOverflow)?;
    Ok(())
  }

  fn stack_push_word(&mut self, memory: &mut dyn MemoryIO, value: u16) -> Result<(), CpuError> {
    let [lo, hi] = value.to_le_bytes();

    self.stack_push(memory, lo)?;
    self.stack_push(memory, hi)?;

    Ok(())
  }

  fn stack_pull(&mut self, memory: &dyn MemoryIO) -> Result<u8, CpuError> {
    let stack_address = self.stack_address();
    let value = memory.read(stack_address)?;
    self.stack_pointer = self
      .stack_pointer
      .checked_sub(1)
      .ok_or(CpuError::StackUnderflow)?;
    Ok(value)
  }

  fn stack_pull_word(&mut self, memory: &dyn MemoryIO) -> Result<u16, CpuError> {
    let hi = self.stack_pull(memory)?;
    let lo = self.stack_pull(memory)?;
    let value = u16::from_le_bytes([lo, hi]);
    Ok(value)
  }
}

struct DataBus {
  value: u8,
}

impl DataBus {
  pub fn new() -> Self {
    Self {
      value: 0x00,
    }
  }

  pub fn read(&self) -> u8 {
    self.value
  }

  pub fn write(&mut self, value: u8) {
    self.value = value;
  }
}

struct Register {
  value: u8,
}

impl Register {
  pub fn new() -> Self {
    Self {
      value: 0x00
    }
  }

  pub fn push_to_bus(&self, bus: &mut DataBus) {
    bus.write(self.value);
  }

  pub fn pull_from_bus(&mut self, bus: &DataBus) {
    self.value = bus.read();
  }
}
