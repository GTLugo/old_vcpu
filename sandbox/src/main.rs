use tracing::{debug, error};

use vcpu::cpu::CPU;
use vcpu::error::CpuError;
use vcpu::log;
use vcpu::memory::Memory;

fn main() {
  if let Err(error) = run() {
    error!("{error}");
  }
}

fn run() -> Result<(), CpuError> {
  log::init_max();

  debug!("{:?}", std::env::current_dir());

  let asm = std::fs::read("./asm/f.asm")
    .map_err(|error| CpuError::Other(format!("{error}")))?;

  let mut memory = Memory::new(&asm, 0x8000)?;
  let mut cpu = CPU::new(0x0000);

  cpu.continuous_step(&mut memory, 5.0)
    .map_err(|error| {
      debug!("CPU DUMP\n{}", cpu.dump());
      debug!("MEMORY DUMP\n{}", memory.dump());
      error
    })
}
