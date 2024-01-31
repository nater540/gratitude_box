#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod parser;
pub mod error;

use error::*;

mod prelude {
  pub use crate::parser::MessageParser;
  pub use crate::error::ParseError;
  pub use crate::PointAward;
}

/// Default number of points to grant when we couldn't parse any digits
const DEFAULT_POINTS: i32 = 1;

#[derive(Debug)]
pub struct PointAward {
  pub target: String,
  pub points: i32,
  pub operator: Operator,
  pub reason: Option<String>
}

impl Default for PointAward {
  fn default() -> Self {
    PointAward {
      target: String::new(),
      points: DEFAULT_POINTS,
      operator: Operator::Add,
      reason: None
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Subtract
}
