use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
  pub name: String,
  // TODO Update arguments to a JSON
  pub arguments: String,
}