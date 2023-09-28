use std::error::Error;
use std::fmt::{Display, Debug};

#[derive(Clone, PartialEq)]
pub enum ChatCompletionError {
  EmptyMessageContent,
  EmptyMessages,
  FrequencyPenaltyValueOutOfRange(f32),
  PresencePenaltyValueOutOfRange(f32),
  StopSequencesOutOfRange(usize),
  TemperatureValueOutOfRange(f32),
  TopPValueOutOfRange(f32),
  
}

impl ChatCompletionError {
  fn label(&self) -> &'static str {
    match self {
      ChatCompletionError::EmptyMessageContent => "EmptyMessageContent",
      ChatCompletionError::EmptyMessages => "EmptyMessages",
      ChatCompletionError::FrequencyPenaltyValueOutOfRange(_) => "FrequencyPenaltyValueOutOfRange",
      ChatCompletionError::PresencePenaltyValueOutOfRange(_) => "PresencePenaltyValueOutOfRange",
      ChatCompletionError::StopSequencesOutOfRange(_) => "StopSequencesOutOfRange",
      ChatCompletionError::TemperatureValueOutOfRange(_) => "TemperatureValueOutOfRange",
      ChatCompletionError::TopPValueOutOfRange(_) => "TopPValueOutOfRange",
    }
  }

  fn error_message(&self) -> String {
    match self {
      ChatCompletionError::EmptyMessageContent => String::from("Message content cannot be empty"),
      ChatCompletionError::EmptyMessages => String::from("Messages cannot be empty."),
      ChatCompletionError::FrequencyPenaltyValueOutOfRange(frequency_penalty) => format!("Frequency penalty value must be between -2.0 and 2.0 [Given value: {}]", frequency_penalty),
      ChatCompletionError::PresencePenaltyValueOutOfRange(presence_penalty) => format!("Presence penalty value must be between -2.0 and 2.0 [Given value: {}]", presence_penalty),
      ChatCompletionError::StopSequencesOutOfRange(sequences_count) => format!("Stop value must have between 0 and 4 sequences [Number of sequences: {}]", sequences_count),
      ChatCompletionError::TemperatureValueOutOfRange(temperature) => format!("Temperature value must be between 0.0 and 2.0 [Given value: {}]", temperature),
      ChatCompletionError::TopPValueOutOfRange(top_p) => format!("Top P value must be between 0.0 and 1.0 [Given value: {}]", top_p),
    }
  }
}

impl Debug for ChatCompletionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.label(), self.error_message())
  }
}

impl Display for ChatCompletionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.error_message())
  }
}

impl Error for ChatCompletionError {}