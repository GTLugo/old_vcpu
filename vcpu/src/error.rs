use thiserror::Error;
use bau::error::{AssemblerError, InstructionError};

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  AssemblerError(#[from] AssemblerError),
  #[error("{0}")]
  CpuError(#[from] CpuError),
  #[error("{0}")]
  MemoryError(#[from] MemoryError),
}

#[derive(Error, Debug)]
pub enum CpuError {
  #[error("{0}")]
  InstructionError(#[from] InstructionError),
  #[error("{0}")]
  MemoryError(#[from] MemoryError),
  #[error("op code `{0:04X}` is invalid")]
  InvalidOpCode(u16),
  #[error("program counter overflowed")]
  ProgramCounterOverflow,
  #[error("stack overflow")]
  StackOverflow,
  #[error("stack underflow")]
  StackUnderflow,
  #[error("feature not implemented")]
  Unimplemented,
  #[error("{0}")]
  Other(String),
  #[error("unspecified cpu error")]
  Unspecified,
}

#[derive(Error, Debug)]
pub enum MemoryError {
  #[error("memory address `0x{0:08X}` is part of null sector")]
  NullAddress(u64),
  #[error("memory address `0x{0:08X}` is invalid")]
  InvalidAddress(u64),
  #[error("memory address `0x{0:08X}` is reserved")]
  ReservedAddress(u64),
  #[error("attempted to write to read-only memory address `0x{0:08X}`")]
  WriteToReadOnlyAddress(u64),
  #[error("memory read in exceeds max limit")]
  MemoryOverflow,
  #[error("unspecified memory error")]
  Unspecified,
}
