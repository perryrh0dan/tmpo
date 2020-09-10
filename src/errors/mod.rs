#[derive(Debug)]
pub enum RunError {
  Config(String)
  IO(std::io::Error)
  Yaml(String)
}

impl Display for RunError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Config(e) => write!(f, "Failed to load Config! Error: {}", e),
      Self::IO(e) => write!(f, "{}", e),
    }
  }
}

impl From<std::io::Error> for RunError {
  fn from(e: std::io::Error) -> Self {
      RunError::IO(e)
  }
}
