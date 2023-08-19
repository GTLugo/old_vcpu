use tracing::error;

use vcpu::cpu::CPU;
use vcpu::log;
use vcpu::memory::{Memory, SIZE_64KB};

fn main() {
  log::init_max();

  let mut bytes = [0xEA; SIZE_64KB];
  bytes[0xFFFC] = 0xEA; // NOP
  bytes[0xFFFD] = 0xEA; // NOP
  bytes[0x8000] = 0xA9; // LDA
  bytes[0x8001] = 0x69; // $69
  bytes[0x8002] = 0xA9; // LDA
  bytes[0x8003] = 0x34; // $34
  bytes[0x8010] = 0x4C; // JMP
  bytes[0x8011] = 0x00; // $00
  bytes[0x8012] = 0x80; // $80

  let mut memory = Memory::new(bytes, 0x8000);
  let mut cpu = CPU::new(0x8000);

  cpu.reset(&mut memory);
  if let Err(err) = cpu.continuous_step(&mut memory, 3.0) {
    error!("{err}")
  };
}