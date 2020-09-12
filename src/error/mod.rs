use std::fmt;
use std::fmt::{Formatter, Display};

#[derive(Debug)]
pub enum RunError {
  Config(String),
  IO(std::io::Error),
  Input(String),
  Repository(String),
  Template(String),
  Update(String),
}

impl Display for RunError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Config(e) => write!(f, "Failed to load Config! Error: {}", e),
      Self::IO(e) => write!(f, "{}", e),
      Self::Input(e) => write!(f, "{}", e),
      Self::Repository(e) => write!(f, "Unable to load repository! Error: {}", e),
      Self::Template(e) => write!(f, "Unable to load template! Error: {}", e),
      Self::Update(e) => write!(f, "Unable to update! Error: {}", e),
    }
  }
}

impl From<std::io::Error> for RunError {
  fn from(e: std::io::Error) -> Self {
      RunError::IO(e)
  }
}
