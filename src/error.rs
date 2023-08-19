use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpuError {
  #[error("{0}")]
  MemoryError(#[from] MemoryError),
  #[error("op code `{0:02X}` is invalid")]
  InvalidOpCode(u8),
  #[error("program counter overflowed")]
  ProgramCounterOverflow,
  #[error("unspecified cpu error")]
  Unspecified,
  #[error("feature not implemented")]
  Unimplemented,
}

#[derive(Error, Debug)]
pub enum MemoryError {
  #[error("memory address `0x{0:04X}` is invalid")]
  InvalidAddress(u16),
  #[error("attempted to write to read-only memory address `0x{0:04X}`")]
  WriteToRomAddress(u16),
  #[error("unspecified memory error")]
  Unspecified,
}
