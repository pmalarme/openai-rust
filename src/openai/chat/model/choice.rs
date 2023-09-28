use serde::{Serialize, Deserialize};

use crate::openai::chat::model::ChatMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
  pub index: u16,
  pub message: ChatMessage,
  pub finish_reason: String,
}
