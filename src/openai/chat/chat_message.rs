use serde::{Serialize, Deserialize};

use crate::openai::chat::{FunctionCall, Role};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
  pub role: Role,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub function_call: Option<FunctionCall>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

pub struct ChatMessageBuilder {
  messages: Vec<ChatMessage>,
}

impl ChatMessageBuilder {
  pub fn new() -> ChatMessageBuilder {
    ChatMessageBuilder {
      messages: Vec::new(),
    }
  }

  pub fn system(mut self, content: String) -> ChatMessageBuilder {
    self.messages.push(ChatMessage {
      role: Role::System,
      content,
      function_call: None,
      name: None,
    });
    self
  }

  pub fn assistant(mut self, content: String) -> ChatMessageBuilder {
    self.messages.push(ChatMessage {
      role: Role::Assistant,
      content,
      function_call: None,
      name: None,
    });
    self
  }

  pub fn user(mut self, content: String) -> ChatMessageBuilder {
    self.messages.push(ChatMessage {
      role: Role::User,
      content,
      function_call: None,
      name: None,
    });
    self
  }

  pub fn function(mut self, content: String, function_call: FunctionCall, name: String) -> ChatMessageBuilder {
    self.messages.push(ChatMessage {
      role: Role::Function,
      content,
      function_call: Some(function_call),
      name: Some(name),
    });
    self
  }

  pub fn build(self) -> Vec<ChatMessage> {
    self.messages
  }
}