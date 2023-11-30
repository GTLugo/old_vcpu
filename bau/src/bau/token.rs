use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Token {
  LeftParenthesis    { line: u32 },
  RightParenthesis   { line: u32 },
  LeftAngledBracket  { line: u32 },
  RightAngledBracket { line: u32 },
  LeftSquareBracket  { line: u32 },
  RightSquareBracket { line: u32 },
  Hashtag            { line: u32 },
  Comma              { line: u32 },
  Equals             { line: u32 },
  Period             { line: u32 },
  Colon              { line: u32 },
  DoubleColon        { line: u32 },
  Slash              { line: u32 },
  HexPrefix          { line: u32 },
  EndOfFile          { line: u32 },

  Directive   { line: u32, lexeme: String },
  Instruction { line: u32, lexeme: String },
  Label       { line: u32, lexeme: String },
  Register    { line: u32, lexeme: String },
  Number      { line: u32, lexeme: String },
}

impl Token {
  pub fn line(&self) -> u32 {
    match self {
      Token::LeftParenthesis    { line, .. } => *line,
      Token::RightParenthesis   { line, .. } => *line,
      Token::LeftAngledBracket  { line, .. } => *line,
      Token::RightAngledBracket { line, .. } => *line,
      Token::LeftSquareBracket  { line, .. } => *line,
      Token::RightSquareBracket { line, .. } => *line,
      Token::Hashtag            { line, .. } => *line,
      Token::Comma              { line, .. } => *line,
      Token::Equals             { line, .. } => *line,
      Token::Period             { line, .. } => *line,
      Token::Colon              { line, .. } => *line,
      Token::DoubleColon        { line, .. } => *line,
      Token::Slash              { line, .. } => *line,
      Token::HexPrefix          { line, .. } => *line,
      Token::EndOfFile          { line, .. } => *line,
      Token::Directive          { line, .. } => *line,
      Token::Instruction        { line, .. } => *line,
      Token::Label              { line, .. } => *line,
      Token::Register           { line, .. } => *line,
      Token::Number             { line, .. } => *line,
    }
  }

  pub fn lexeme(&self) -> &str {
    match self {
      Token::LeftParenthesis    { .. } => "(",
      Token::RightParenthesis   { .. } => ")",
      Token::LeftAngledBracket  { .. } => "<",
      Token::RightAngledBracket { .. } => ">",
      Token::LeftSquareBracket  { .. } => "[",
      Token::RightSquareBracket { .. } => "]",
      Token::Hashtag            { .. } => "#",
      Token::Period             { .. } => ".",
      Token::Comma              { .. } => ",",
      Token::Equals             { .. } => "=",
      Token::Colon              { .. } => ":",
      Token::DoubleColon        { .. } => "::",
      Token::HexPrefix          { .. } => "0x",
      Token::Slash              { .. } => "/",
      Token::EndOfFile          { .. } => "EOF",
      Token::Directive          { lexeme, .. } => lexeme,
      Token::Instruction        { lexeme, .. } => lexeme,
      Token::Label              { lexeme, .. } => lexeme,
      Token::Register           { lexeme, .. } => lexeme,
      Token::Number             { lexeme, .. } => lexeme,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.lexeme())
  }
}