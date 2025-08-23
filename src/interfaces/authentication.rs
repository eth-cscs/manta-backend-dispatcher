use crate::error::Error;

pub trait AuthenticationTrait {
  fn get_api_token(
    &self,
    username: &str,
    password: &str,
  ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

  fn validate_api_token(
    &self,
    auth_token: &str,
  ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
