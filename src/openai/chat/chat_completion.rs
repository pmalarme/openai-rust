use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::openai::requestor::Requestor;
use crate::openai::{Client, ApiType};
use crate::openai::chat::{FunctionDefinition, ChatMessage};

const API_PATH: &str = "chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
  #[serde(skip_serializing_if = "Option::is_none")]
  model: Option<String>, // This is required by open ai
  messages: Vec<ChatMessage>,
  #[serde(skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  n: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  stream: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  stop: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  max_tokens: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  presence_penalty: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  frequency_penalty: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logit_bias: Option<HashMap<String, f32>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user: Option<String>,
  // TODO check the value of the function call
  #[serde(skip_serializing_if = "Option::is_none")]
  function_call: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  functions: Option<Vec<FunctionDefinition>>,
}

// TODO add model id an api version but do not serialize it
impl ChatCompletion {
  pub fn new() -> ChatCompletion {
    ChatCompletion {
      model: None,
      messages: Vec::new(),
      temperature: None,
      n: None,
      stream: None,
      stop: None,
      max_tokens: None,
      presence_penalty: None,
      frequency_penalty: None,
      logit_bias: None,
      user: None,
      function_call: None,
      functions: None,
    }
  }

  pub fn message(mut self, message: ChatMessage) -> ChatCompletion {
    self.messages.push(message);
    self
  }

  pub fn messages(mut self, messages: Vec<ChatMessage>) -> ChatCompletion {
    self.messages = messages;
    self
  }

  pub fn temperature(mut self, temperature: f32) -> ChatCompletion {
    self.temperature = Some(temperature);
    self
  }

  pub fn n(mut self, n: u16) -> ChatCompletion {
    self.n = Some(n);
    self
  }

  pub fn stream(mut self, stream: bool) -> ChatCompletion {
    self.stream = Some(stream);
    self
  }

  pub fn stop(mut self, stop: Vec<String>) -> ChatCompletion {
    // Check the size
    self.stop = Some(stop);
    self
  }

  pub fn max_tokens(mut self, max_tokens: u32) -> ChatCompletion {
    self.max_tokens = Some(max_tokens);
    self
  }

  pub fn presence_penalty(mut self, presence_penalty: f32) -> ChatCompletion {
    self.presence_penalty = Some(presence_penalty);
    self
  }

  pub fn frequency_penalty(mut self, frequency_penalty: f32) -> ChatCompletion {
    self.frequency_penalty = Some(frequency_penalty);
    self
  }

  pub fn logit_bias(mut self, logit_bias: HashMap<String, f32>) -> ChatCompletion {
    self.logit_bias = Some(logit_bias);
    self
  }

  pub fn user(mut self, user: String) -> ChatCompletion {
    self.user = Some(user);
    self
  }

  pub fn function_call(mut self, function_call: String) -> ChatCompletion {
    self.function_call = Some(function_call);
    self
  }

  pub fn functions(mut self, functions: Vec<FunctionDefinition>) -> ChatCompletion {
    self.functions = Some(functions);
    self
  }

  pub async fn create(&mut self, client: Client, model_id: &str, api_version: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let request_body = serde_json::to_string(self)?;
    println!("{:?}", request_body);

    // Model id is required only for Open AI
    if client.api_type == ApiType::OpenAI {
      self.model = Some(model_id.to_string());
      let response = client.post(API_PATH, &request_body, None, None).await?;
      Ok(response)
    } else {
      let response = client.post(API_PATH, &request_body, Some(model_id), api_version).await?;
      Ok(response)
    }
  }
}