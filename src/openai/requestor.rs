use std::error::Error;
use async_trait::async_trait;
use reqwest::RequestBuilder;
use crate::openai::{Client, ApiType};

#[async_trait]
pub trait Requestor {
  async fn post(&self, api_path: &str, body: &str, model_id: Option<&str>, api_version: Option<&str>) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
impl Requestor for Client {
  async fn post(&self, api_path: &str, body: &str, model_id: Option<&str>, api_version: Option<&str>) -> Result<String, Box<dyn Error>> {
    let api_uri = self.generate_api_uri(api_path, model_id, api_version)?;

    let mut request_builder: RequestBuilder = self.http_client.post(api_uri)
      .header(reqwest::header::CONTENT_TYPE, "application/json");
    
    // API Key is required for Azure and OpenAI. For Azure AD, managed identity is used.
    if self.api_type == ApiType::Azure {
      request_builder = request_builder.header("api-key", self.get_api_key());
    } else if self.api_type == ApiType::OpenAI {
      request_builder = request_builder.bearer_auth(self.get_api_key());
    }
    
    let response: String = request_builder
      .body(body.to_string())
      .send()
      .await?
      .text()
      .await?;
    Ok(response)
  }
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod test {
  use wiremock::{MockServer, Mock, ResponseTemplate};
  use wiremock::matchers::{method, path, bearer_token, header, body_string};
  use crate::openai::auth::create_auth_with_given_api_key;
  use super::*;

  #[tokio::test]
  async fn it_should_post_to_openai_api() {
    let api_key: String = String::from("12345abcd");
    let body_request: String = String::from(r#"{"prompt": "Once upon a time", "max_tokens": 5}"#);
    let body_response: String = String::from(r#"{"id": "cmpl-3QJ5nq5Z5j5J5", "object": "text_completion", "created": 1619266792, "model": "davinci:2020-05-03", "choices": [{"text": " a", "index": 0, "logprobs": null, "finish_reason": "length"}]}"#);

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
      .and(path("/engines/chat/completions"))
      .and(bearer_token(&api_key))
      .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
      .and(body_string(body_request.clone()))
      .respond_with(ResponseTemplate::new(200)
        .set_body_string(body_response.clone())
      )
      .expect(1)
      .mount(&mock_server)
      .await;

    let auth = create_auth_with_given_api_key(&api_key);
    let openai_client = Client::new(auth, &mock_server.uri(), ApiType::OpenAI);
    let response = openai_client.post("chat/completions", &body_request, None, None).await;
    assert!(response.is_ok());
    let response_as_string = response.unwrap();
    assert_eq!(response_as_string, body_response);
  }

  #[tokio::test]
  async fn it_should_post_to_azure_open_api() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let body_request: String = String::from(r#"{"prompt": "Once upon a time", "max_tokens": 5}"#);
    let body_response: String = String::from(r#"{"id": "cmpl-3QJ5nq5Z5j5J5", "object": "text_completion", "created": 1619266792, "model": "davinci:2020-05-03", "choices": [{"text": " a", "index": 0, "logprobs": null, "finish_reason": "length"}]}"#);

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
      .and(path("/openai/deployments/model-deployment-id/chat/completions"))
      .and(header("api-key", api_key.as_str()))
      .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
      .and(body_string(body_request.clone()))
      .respond_with(ResponseTemplate::new(200)
        .set_body_string(body_response.clone())
      )
      .expect(1)
      .mount(&mock_server)
      .await;
    
    let auth = create_auth_with_given_api_key(&api_key);
    let openai_client = Client::new(auth, &mock_server.uri(), ApiType::Azure);
    let response = openai_client.post("chat/completions", &body_request, Some(&azure_model_id), Some(&azure_api_version)).await;
    assert!(response.is_ok());
    let response_as_string = response.unwrap();
    assert_eq!(response_as_string, body_response);
  }

  #[tokio::test]
  async fn it_should_post_to_azure_open_api_using_azure_ad() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let body_request: String = String::from(r#"{"prompt": "Once upon a time", "max_tokens": 5}"#);
    let body_response: String = String::from(r#"{"id": "cmpl-3QJ5nq5Z5j5J5", "object": "text_completion", "created": 1619266792, "model": "davinci:2020-05-03", "choices": [{"text": " a", "index": 0, "logprobs": null, "finish_reason": "length"}]}"#);

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
      .and(path("/openai/deployments/model-deployment-id/chat/completions"))
      .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
      .and(body_string(body_request.clone()))
      .respond_with(ResponseTemplate::new(200)
        .set_body_string(body_response.clone())
      )
      .expect(1)
      .mount(&mock_server)
      .await;
    
    let auth = create_auth_with_given_api_key(&api_key);
    let openai_client = Client::new(auth, &mock_server.uri(), ApiType::AzureAD);
    let response = openai_client.post("chat/completions", &body_request, Some(&azure_model_id), Some(&azure_api_version)).await;
    assert!(response.is_ok());
    let response_as_string = response.unwrap();
    assert_eq!(response_as_string, body_response);
    
    let requests = mock_server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    let request = requests.get(0).unwrap();
    for header in request.headers.iter() {
      assert_ne!(header.0.as_str(), "api-key");
    }
  }
}
