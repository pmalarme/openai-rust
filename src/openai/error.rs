use std::fmt::{Display, Debug};

#[derive(PartialEq, Eq)]
pub enum Error {
  ApiError {status: u16, message: String},
  ClientError(ClientErrorType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientErrorType {
  ModelIdMissingToGenerateApiUriForAzure,
}

impl Error {
  fn label(&self) -> &'static str {
    match self {
      Error::ApiError {..} => "ApiError",
      Error::ClientError(_) => "ClientError",
    }
  }

  fn error_message(&self) -> String {
    match self {
      Error::ApiError {status, message} => format!("(HTTP {}) {}]", status, message),
      Error::ClientError(ClientErrorType::ModelIdMissingToGenerateApiUriForAzure) => String::from("Model ID is required to generate API URI for Azure"),
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