pub struct Auth {
  pub api_key: String,
}

impl Auth {
  pub fn new(api_key: String) -> Auth {
    Auth { api_key }
  }

  // TODO Update the error
  pub fn from_env() -> Result<Self, std::env::VarError> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    Ok(Self::new(api_key))
  }
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
pub fn create_auth_with_given_api_key(api_key: &str) -> Auth {
  Auth::new(api_key.to_string())
}


#[cfg(test)]
pub fn create_auth_with_environment_variable(api_key: &str) -> Result<Auth, std::env::VarError> {
  std::env::set_var("OPENAI_API_KEY", api_key.to_string());
  Auth::from_env()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_auth_with_given_api_key() {
    let api_key: String = String::from("12345abcd");

    let auth: Auth = create_auth_with_given_api_key(&api_key);
    assert_eq!(auth.api_key, api_key);
  }

  #[test]
  fn it_should_create_auth_with_environment_variable() {
    let api_key: String = String::from("12345abcd");
    
    let auth: Result<Auth, std::env::VarError> = create_auth_with_environment_variable(&api_key);
    match auth {
      Ok(auth) => assert_eq!(auth.api_key, api_key),
      Err(_) => assert!(false, "Auth should be created with environment variable")
    }
  }

}