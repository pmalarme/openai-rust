use openai_rust::openai::chat::{ChatCompletion, model::ChatMessageBuilder};
use openai_rust::openai::{Client, ApiType};
// TODO create a runtime for tokio

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let openai_client = Client::from_env(ApiType::Azure)?;
  let chat_messages = ChatMessageBuilder::new()
    .system(String::from("You are a helpful assistant."))
    .user(String::from("Tell me a very long joke"))
    .build()  ;
  let chat_completion_response = ChatCompletion::new()
    .temperature(0.8)?
    .top_p(0.5)?
    .max_tokens(5000)
    .stream(false)
    .messages(chat_messages)
    .create(openai_client, "gpt-4-8k", Some("2023-08-01-preview")).await?;
  println!("{:?}", chat_completion_response);
  Ok(())
} 