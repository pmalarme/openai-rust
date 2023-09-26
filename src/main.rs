// use std::collections::HashMap;

use openai_rust::openai::chat::{ChatCompletion, ChatMessageBuilder};
use openai_rust::openai::{Auth, Client, ApiType};
// TODO create a runtime for tokio

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let auth = Auth::new(String::from(""));
  let openai_client = Client::new(auth, "", ApiType::Azure);

  let chat_messages = ChatMessageBuilder::new()
    .system(String::from("You are a helpful assistant."))
    .user(String::from("Tell me a very long joke"))
    .build()  ;
  let response = ChatCompletion::new()
    .temperature(0.8)
    .max_tokens(5000)
    .stream(false)
    .messages(chat_messages)
    .create(openai_client, "gpt-4-8k", Some("2023-05-15")).await?;
  println!("{:?}", response);
  Ok(())
} 