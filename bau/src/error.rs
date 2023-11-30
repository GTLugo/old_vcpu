use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstructionError {
  #[error("op code `0x{0:04X}` is invalid")]
  InvalidOpCode(u16),
  #[error("feature not implemented")]
  Unimplemented,
  #[error("{0}")]
  Other(String),
  #[error("unspecified instruction error")]
  Unspecified,
}

#[derive(Error, Debug)]
pub enum AssemblerError {
  #[error("script `{0}` was not found")]
  IOError(#[from] io::Error),
  #[error("{location}: {line} | Unknown Token: `{token}`")]
  UnknownToken { location: String, line: u32, token: String },
  #[error("{location}: {line} | Syntax Error: `{message}`")]
  SyntaxError { location: String, line: u32, message: String },
  #[error("feature not implemented")]
  Unimplemented,
  #[error("{0}")]
  Other(String),
  #[error("unspecified bau error")]
  Unspecified,
}