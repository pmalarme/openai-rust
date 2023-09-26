use crate::openai::{Auth, ApiType};

const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/";

pub struct Client {
  endpoint: String,
  pub(crate) api_type: ApiType,
  auth: Auth,
  pub(crate) http_client: reqwest::Client,
}

impl Client {
  pub fn new_openai_client(auth: Auth) -> Client {
    let auth = auth;
    Client::new(auth, OPENAI_ENDPOINT, ApiType::OpenAI)
  }

  pub fn new(auth: Auth, endpoint: &str, api_type: ApiType) -> Client {
    let endpoint: String = Client::update_endpoint_to_have_slash_add_the_end(endpoint);
    let http_client = reqwest::Client::new();
    Client {
      endpoint,
      api_type,
      auth,
      http_client,
    }
  }

  pub fn get_api_key(&self) -> String {
    self.auth.api_key.clone()
  }

  pub fn get_api_type(&self) -> ApiType {
    self.api_type.clone()
  }

  pub fn generate_api_uri(&self, api_path: &str, model_id: Option<&str>, api_version: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match self.api_type {
      ApiType::Azure | ApiType::AzureAD => {
        match model_id {
          Some(model_id) => {
            match api_version {
              Some(api_version) => Ok(format!("{}openai/deployments/{}/{}?api-version={}", self.endpoint, model_id, api_path, api_version)),
              None => Err("api_version is missing".into()),
            }
          },
          None => Err("model_id is missing".into()),
        }
      },
      ApiType::OpenAI => Ok(format!("{}engines/{}", self.endpoint, api_path)),
    }
  }

  fn update_endpoint_to_have_slash_add_the_end(endpoint: &str) -> String {
    if !endpoint.ends_with("/") {
      endpoint.to_string() + &"/"
    } else {
      endpoint.to_string()
    }
  }
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
  use crate::openai::auth::{create_auth_with_given_api_key, create_auth_with_environment_variable};
  use super::*;

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new_openai_client() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new_openai_client(auth);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, None).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_env_variable_using_new_openai_client() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_environment_variable(&api_key).unwrap();

    let client: Client = Client::new_openai_client(auth);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, Some("not"), Some("used")).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, "https://api.openai.com/v1/", ApiType::OpenAI);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, Some("not used")).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_env_variable_using_new() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_environment_variable(&api_key).unwrap();

    let client: Client = Client::new(auth, "https://api.openai.com/v1/", ApiType::OpenAI);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, Some("not used"), None).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new_with_missing_slash_at_the_end_of_the_endpoint() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, "https://api.openai.com/v1", ApiType::OpenAI);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, None).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_azure_openai_client_with_given_key_using_new() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_endpoint, ApiType::Azure);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.endpoint, azure_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_with_env_variable_using_new() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_endpoint, ApiType::Azure);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.endpoint, azure_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_with_given_key_using_new_with_missing_slash_at_the_end_of_the_endpoint() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, "https://my-resource-name.openai.azure.com", ApiType::Azure);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.endpoint, azure_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }
}
