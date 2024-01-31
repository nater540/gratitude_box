#[derive(thiserror::Error, Debug)]
pub enum ParseError {
  #[error("parsing error")]
  ParsingError(#[from] pest::error::Error<crate::parser::Rule>),

  #[error("invalid point value")]
  InvalidPointValue,

  #[error("invalid mention")]
  InvalidMention,

  #[error("invalid operator")]
  InvalidOperator
}
