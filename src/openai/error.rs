use std::fmt::{Display, Debug};

pub enum Error {
  ApiError {status: u16, message: String},
}

impl Error {
  fn label(&self) -> &'static str {
    match self {
      Error::ApiError {..} => "ApiError",
    }
  }

  fn error_message(&self) -> String {
    match self {
      Error::ApiError {status, message} => format!("(HTTP {}) {}]", status, message),
    }
  }
}

impl Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.label(), self.error_message())
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.error_message())
  }
}

impl std::error::Error for Error {}