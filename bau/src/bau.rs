use std::fs;
use std::path::Path;

use crate::bau::scanner::Scanner;
use crate::error::AssemblerError;

mod parser;
mod scanner;
mod token;

// The BAU-ssembler

pub struct BAU {

}

impl BAU {
  pub fn new() -> BAU {
    Self {

    }
  }

  pub fn assemble<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<u8>, AssemblerError> {
    let source = fs::read_to_string(file_path)?;
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
      println!("{token:?}");
    }

    Err(AssemblerError::Unimplemented)
  }
}
