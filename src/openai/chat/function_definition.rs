use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
  pub name: String,
  pub desription: String,
  // TODO Update to a JSON object
  pub parameters: String,
}