use tracing::error;

use bau::bau::BAU;
use vcpu::error::Error;
use vcpu::log;

fn main() {
  if let Err(error) = run() {
    // this is just so that the error is emitted as part of the logging
    error!("{error}");
  }
}

fn run() -> Result<(), Error> {
  log::init_max();

  let bau = BAU::new();
  let asm = bau.assemble("res/asm/assembly.bau")?;

  // let mut memory = MMU {
  //   rom
  // }
  // let mut cpu = CPU::new(0x0000);
  //
  // cpu.continuous_step(&mut memory, 5.0)
  //   .map_err(|error| {
  //     // debug!("CPU DUMP\n{}", cpu.dump());
  //     // debug!("MEMORY DUMP\n{}", memory.dump());
  //     error
  //   })

  Ok(())
}
