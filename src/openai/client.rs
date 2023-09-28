use crate::openai::{Auth, ApiType};
use crate::openai::error::{Error, ClientErrorType};

const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/";

pub struct Client {
  api_endpoint: String,
  pub(crate) api_type: ApiType,
  auth: Auth,
  pub(crate) http_client: reqwest::Client,
}

impl Client {
  pub fn new_openai_client(auth: Auth) -> Client {
    let auth = auth;
    Client::new(auth, OPENAI_ENDPOINT, ApiType::OpenAI)
  }

  pub fn new(auth: Auth, api_endpoint: &str, api_type: ApiType) -> Client {
    let api_endpoint: String = Client::update_api_endpoint_to_have_a_slash_add_the_end(api_endpoint);
    let http_client = reqwest::Client::new();
    Client {
      api_endpoint,
      api_type,
      auth,
      http_client,
    }
  }

  /// Create a new client from the environment variables. It creates both the Auth
  /// using [`crate::openai::Auth::from_env()`] and the client.
  /// 
  /// If the API type is [`crate::openai::ApiType::OpenAI`] then the api_endpoint is set
  /// to the default value of https://api.openai.com/v1/. The client is created using
  /// [`Self::new_openai_client()`].
  /// 
  /// If the API type is [`crate::openai::ApiType::Azure`] or [`crate::openai::ApiType::AzureAD`],
  /// then the api_endpoint is set to the value of the environment variable OPENAI_API_ENDPOINT.
  /// 
  /// # Errors
  /// 
  /// This function will return an error if:
  /// - [`crate::openai::Auth::from_env()`] returns an error, i.e. OPENAI_API_KEY environment
  /// variable is not set.
  /// - the API type is [`crate::openai::ApiType::Azure`] or [`crate::openai::ApiType::AzureAD`]
  /// and OPENAI_API_ENDPOINT environment variable is not set.
  /// 
  pub fn from_env(api_type: ApiType) -> Result<Client, std::env::VarError> {
    let auth: Auth = Auth::from_env()?;
    match api_type {
      ApiType::OpenAI => Ok(Client::new_openai_client(auth)),
      ApiType::Azure | ApiType::AzureAD => {
        let api_endpoint: String = std::env::var("OPENAI_API_ENDPOINT")?;
        Ok(Client::new(auth, &api_endpoint, api_type))
      },
    }
  }

  pub fn get_api_key(&self) -> String {
    self.auth.api_key.clone()
  }

  pub fn get_api_type(&self) -> ApiType {
    self.api_type.clone()
  }

  /// Generate OpenAI API URI from the given attributes:
  /// - api_path: The path of the API
  /// - model_id: The model ID. It is required for Azure and Azure AD. For OpenAI, it is not used.
  /// - api_version: The API version. It is used for Azure and Azure AD. For OpenAI, it is not used.
  /// 
  /// If the API type is [`crate::openai::ApiType::Azure`] or [`crate::openai::ApiType::AzureAD`],
  /// and if API version is not set, the default value of `2023-05-15` is used.
  /// 
  /// # Errors
  /// 
  /// The function return an error if the API type is [`crate::openai::ApiType::Azure`] or
  /// [`crate::openai::ApiType::AzureAD`] and the model ID is not set. The type of the error is
  /// [`crate::openai::error::Error::ClientError`] with the value of
  /// [`crate::openai::error::ClientErrorType::ModelIdMissingToGenerateApiUriForAzure`].
  /// 
  /// # Examples
  /// 
  /// For OpenAI:
  /// ```
  /// client.generate_api_uri("chat/completions", None, None);
  /// ```
  /// 
  /// For Azure and Azure AD:
  /// ```
  /// client.generate_api_uri("chat/completions", Some("model-id"), Some("2023-05-15"));
  /// ```
  /// 
  /// ```
  /// client.generate_api_uri("chat/completions", Some("model-id"), None);
  /// ```
  /// 
  pub fn generate_api_uri(&self, api_path: &str, model_id: Option<&str>, api_version: Option<&str>) -> Result<String, Error> {
    match self.api_type {
      ApiType::Azure | ApiType::AzureAD => {
        match model_id {
          Some(model_id) => {
            let api_version: &str = match api_version {
              Some(api_version) => api_version,
              None => &"2023-05-15",
            };
            Ok(format!("{}openai/deployments/{}/{}?api-version={}", self.api_endpoint, model_id, api_path, api_version))
          },
          None => Err(Error::ClientError(ClientErrorType::ModelIdMissingToGenerateApiUriForAzure)),
        }
      },
      ApiType::OpenAI => Ok(format!("{}engines/{}", self.api_endpoint, api_path)),
    }
  }

  fn update_api_endpoint_to_have_a_slash_add_the_end(api_endpoint: &str) -> String {
    if !api_endpoint.ends_with("/") {
      api_endpoint.to_string() + &"/"
    } else {
      api_endpoint.to_string()
    }
  }
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
pub fn create_client_fom_env_variables(api_key: &str, api_type: ApiType, api_endpoint: Option<&str>) -> Result<Client, std::env::VarError> {
  std::env::set_var("OPENAI_API_KEY", api_key.to_string());
  if let Some(api_endpoint) = api_endpoint {
    std::env::set_var("OPENAI_API_ENDPOINT", api_endpoint.to_string());
  }
  Client::from_env(api_type)
}

#[cfg(test)]
mod tests {
  use crate::openai::auth::create_auth_with_given_api_key;
  use super::*;

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new_openai_client() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new_openai_client(auth);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.get_api_type(), ApiType::OpenAI);
    assert_eq!(client.api_endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, None).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, "https://api.openai.com/v1/", ApiType::OpenAI);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.get_api_type(), ApiType::OpenAI);
    assert_eq!(client.api_endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, Some("not used")).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_env_variables_using_from_env() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    
    let client: Client = create_client_fom_env_variables(&api_key, ApiType::OpenAI, None).unwrap();
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.get_api_type(), ApiType::OpenAI);
    assert_eq!(client.api_endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, Some("not"), Some("used")).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_openai_client_with_given_key_using_new_with_missing_slash_at_the_end_of_the_api_endpoint() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, "https://api.openai.com/v1", ApiType::OpenAI);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::OpenAI);
    assert_eq!(client.get_api_type(), ApiType::OpenAI);
    assert_eq!(client.api_endpoint, OPENAI_ENDPOINT);
    assert_eq!(client.generate_api_uri(&api_path, None, None).unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_with_given_key_using_new() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_api_endpoint, ApiType::Azure);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.get_api_type(), ApiType::Azure);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_with_env_variables_using_from_env() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::Azure, Some(&azure_api_endpoint)).unwrap();
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.get_api_type(), ApiType::Azure);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_with_given_key_using_new_with_missing_slash_at_the_end_of_the_api_endpoint() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_api_endpoint, ApiType::Azure);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::Azure);
    assert_eq!(client.get_api_type(), ApiType::Azure);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_ad_with_given_key_using_new() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_api_endpoint, ApiType::AzureAD);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::AzureAD);
    assert_eq!(client.get_api_type(), ApiType::AzureAD);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_ad_with_env_variables_using_from_env() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::AzureAD, Some(&azure_api_endpoint)).unwrap();
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::AzureAD);
    assert_eq!(client.get_api_type(), ApiType::AzureAD);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_create_azure_openai_client_of_type_azure_ad_with_given_key_using_new_with_missing_slash_at_the_end_of_the_api_endpoint() {
    let api_key: String = String::from("12345abcd");
    let azure_api_version: String = String::from("2023-05-15");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");
    let auth: Auth = create_auth_with_given_api_key(&api_key);

    let client: Client = Client::new(auth, &azure_api_endpoint, ApiType::AzureAD);
    assert_eq!(client.auth.api_key, api_key);
    assert_eq!(client.get_api_key(), api_key);
    assert_eq!(client.api_type, ApiType::AzureAD);
    assert_eq!(client.get_api_type(), ApiType::AzureAD);
    assert_eq!(client.api_endpoint, azure_api_endpoint);
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), Some(&azure_api_version)).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_use_default_api_version_when_it_is_set_to_none_for_azure() {
    let api_key: String = String::from("12345abcd");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::Azure, Some(&azure_api_endpoint)).unwrap();
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), None).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_use_default_api_version_when_it_is_set_to_none_for_azure_ad() {
    let api_key: String = String::from("12345abcd");
    let azure_model_id: String = String::from("model-deployment-id");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::AzureAD, Some(&azure_api_endpoint)).unwrap();
    assert_eq!(client.generate_api_uri(&api_path, Some(&azure_model_id), None).unwrap(), String::from("https://my-resource-name.openai.azure.com/openai/deployments/model-deployment-id/api/path?api-version=2023-05-15"));
  }

  #[test]
  fn it_should_not_return_an_error_when_model_id_is_not_set_for_openai_type() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::OpenAI, None).unwrap();
    let result = client.generate_api_uri(&api_path, None, Some("not used"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_not_return_an_error_when_model_id_is_set_and_version_is_not_set_for_azure_type() {
    let api_key: String = String::from("12345abcd");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::OpenAI, None).unwrap();
    let result = client.generate_api_uri(&api_path, Some("model-id"), None);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), String::from("https://api.openai.com/v1/engines/api/path"));
  }

  #[test]
  fn it_should_return_an_error_when_model_id_is_not_set_for_azure_type() {
    let api_key: String = String::from("12345abcd");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::Azure, Some(&azure_api_endpoint)).unwrap();
    let result = client.generate_api_uri(&api_path, None, None);
    match result {
      Ok(_) => panic!("It should return an error"),
      Err(error) => assert_eq!(error, Error::ClientError(ClientErrorType::ModelIdMissingToGenerateApiUriForAzure)),
    }
  }

  #[test]
  fn it_should_return_an_error_when_model_id_is_not_set_for_azure_ad_type() {
    let api_key: String = String::from("12345abcd");
    let azure_api_endpoint: String = String::from("https://my-resource-name.openai.azure.com/");
    let api_path: String = String::from("api/path");

    let client: Client = create_client_fom_env_variables(&api_key, ApiType::Azure, Some(&azure_api_endpoint)).unwrap();
    let result = client.generate_api_uri(&api_path, None, None);
    match result {
      Ok(_) => panic!("It should return an error"),
      Err(error) => assert_eq!(error, Error::ClientError(ClientErrorType::ModelIdMissingToGenerateApiUriForAzure)),
    }
  }
}
