use crate::bau::token::Token;
use crate::error::AssemblerError;

pub struct Scanner {
  source: String
}

impl Scanner {
  pub fn new(source: String) -> Self {
    Self {
      source
    }
  }

  pub fn scan_tokens(&self) -> Result<Vec<Token>, AssemblerError> {
    Err(AssemblerError::Unimplemented)
  }
}
