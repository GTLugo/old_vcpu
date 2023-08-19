use tracing::error;

use vcpu::cpu::CPU;
use vcpu::log;
use vcpu::memory::{Memory, SIZE_64KB};

fn main() {
  log::init_max();

  let mut bytes = [0xEA; SIZE_64KB];
  bytes[0xFFFC] = 0xEA; // LDA
  bytes[0xFFFD] = 0xEA; // LDA
  bytes[0x8000] = 0xA9; // LDA
  bytes[0x8001] = 0x69; // $0x69
  bytes[0x8002] = 0xA9; // LDA
  bytes[0x8003] = 0x34; // $0x69

  let mut memory = Memory::new(bytes, 0x8000);
  let mut cpu = CPU::new(0x8000, 2.0);

  cpu.reset(&mut memory);
  if let Err(err) = cpu.continuous_step(&mut memory) {
    error!("{err}")
  };
}