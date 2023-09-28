use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::openai::requestor::Requestor;
use crate::openai::{Client, ApiType};
use crate::openai::chat::error::ChatCompletionError;
use crate::openai::chat::model::{FunctionDefinition, ChatMessage, ChatCompletionResponse};

const API_PATH: &str = "chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
  #[serde(skip_serializing_if = "Option::is_none")]
  model: Option<String>,
  messages: Vec<ChatMessage>,
  #[serde(skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,
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
  #[serde(skip_serializing_if = "Option::is_none")]
  function_call: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  functions: Option<Vec<FunctionDefinition>>,
}

impl ChatCompletion {
  pub fn new() -> ChatCompletion {
    ChatCompletion {
      model: None,
      messages: Vec::new(),
      temperature: None,
      top_p: None,
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

  pub fn message(mut self, message: ChatMessage) -> Result<ChatCompletion, ChatCompletionError> {
    if message.content.is_empty() {
      Err(ChatCompletionError::EmptyMessageContent)
    } else {
      self.messages.push(message);
      Ok(self)
    }
  }

  pub fn messages(mut self, messages: Vec<ChatMessage>) -> ChatCompletion {
    self.messages = messages;
    self
  }

  pub fn temperature(mut self, temperature: f32) -> Result<ChatCompletion, ChatCompletionError> {
    if temperature < 0.0 || temperature > 2.0 {
      Err(ChatCompletionError::TemperatureValueOutOfRange(temperature))
    } else {
      self.temperature = Some(temperature);
      Ok(self)
    }
  }

  pub fn top_p(mut self, top_p: f32) -> Result<ChatCompletion, ChatCompletionError> {
    if top_p < 0.0 || top_p > 1.0 {
      Err(ChatCompletionError::TopPValueOutOfRange(top_p))
    } else {
      self.top_p = Some(top_p);
      Ok(self)
    }
  }

  pub fn n(mut self, n: u16) -> ChatCompletion {
    self.n = Some(n);
    self
  }

  pub fn stream(mut self, stream: bool) -> ChatCompletion {
    self.stream = Some(stream);
    self
  }

  pub fn stop(mut self, stop: Vec<String>) -> Result<ChatCompletion, ChatCompletionError> {
    if stop.is_empty() {
      self.stop = None;
      Ok(self)
    } else if stop.len() > 4 {
      Err(ChatCompletionError::StopSequencesOutOfRange(stop.len()))
    } else {
      self.stop = Some(stop);
      Ok(self)
    }
  }

  pub fn max_tokens(mut self, max_tokens: u32) -> ChatCompletion {
    self.max_tokens = Some(max_tokens);
    self
  }

  pub fn presence_penalty(mut self, presence_penalty: f32) -> Result<ChatCompletion, ChatCompletionError> {
    if presence_penalty < -2.0 || presence_penalty > 2.0 {
      Err(ChatCompletionError::PresencePenaltyValueOutOfRange(presence_penalty))
    } else {
      self.presence_penalty = Some(presence_penalty);
      Ok(self)
    }
  }

  pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Result<ChatCompletion, ChatCompletionError> {
    if frequency_penalty < -2.0 || frequency_penalty > 2.0 {
      Err(ChatCompletionError::FrequencyPenaltyValueOutOfRange(frequency_penalty))
    } else {
      self.frequency_penalty = Some(frequency_penalty);
      Ok(self)
    }
  }

  pub fn logit_bias(mut self, logit_bias: HashMap<String, f32>) -> ChatCompletion {
    if logit_bias.is_empty() {
      self.logit_bias = None;
    } else {
      self.logit_bias = Some(logit_bias);
    }
    self
  }

  pub fn user(mut self, user: String) -> ChatCompletion {
    if user.is_empty() {
      self.user = None;
    } else {
      self.user = Some(user);
    }
    self
  }

  pub fn function_call(mut self, function_call: String) -> ChatCompletion {
    if function_call.is_empty() {
      self.function_call = None;
    } else {
      self.function_call = Some(function_call);
    }
    self
  }

  pub fn functions(mut self, functions: Vec<FunctionDefinition>) -> ChatCompletion {
    if functions.is_empty() {
      self.functions = None;
    } else {
      self.functions = Some(functions);
    }
    self
  }

  pub async fn create(&mut self, client: Client, model_id: &str, api_version: Option<&str>) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
    if self.messages.is_empty() {
      return Err(ChatCompletionError::EmptyMessages.into());
    }
    // Model id is required only for Open AI as it needs to be in the body. Not required for Azure OpenAI
    if client.api_type == ApiType::OpenAI {
      self.model = Some(model_id.to_string());
    }
    // Serialize the body to a string to be sent to the API
    let request_body = serde_json::to_string(self)?;
    // We can call with model id both OpenAI and Azure OpenAI the requestor will handle the logic
    let response = client.post(API_PATH, &request_body, Some(model_id), api_version).await?;
    let chat_completion_response = response.json::<ChatCompletionResponse>().await?;
    Ok(chat_completion_response)
  }
}