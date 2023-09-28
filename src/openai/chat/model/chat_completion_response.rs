use serde::{Serialize, Deserialize};

use crate::openai::chat::model::{Choice, Usage}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
  pub id: String,
  pub object: String,
  pub created: u64,
  pub model: String,
  pub choices: Vec<Choice>,
  pub usage: Option<Usage>,
}
