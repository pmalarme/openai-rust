use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
  pub name: String,
  pub arguments: HashMap<String, String>,
}