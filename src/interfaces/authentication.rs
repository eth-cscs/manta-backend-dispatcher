use crate::error::Error;

pub trait AuthenticationTrait {
  fn get_api_token(
    &self,
    username: &str,
    password: &str,
  ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

  /// Exchange `subject_token` (the caller's token from the central
  /// identity provider) for a token this backend accepts.
  ///
  /// The returned token is used for every backend call made on the
  /// caller's behalf. A backend that accepts the subject token as is
  /// returns it unchanged.
  ///
  /// # Errors
  ///
  /// - [`Error::Unauthorized`] when the identity provider rejects the
  ///   subject token (expired, revoked, untrusted issuer); the caller
  ///   should log in again.
  /// - [`Error::Forbidden`] when the exchange is not permitted for this
  ///   caller.
  /// - [`Error::NetError`] when the identity provider is unreachable or
  ///   times out, and [`Error::Message`] for any other failure,
  ///   including a misconfigured exchange client.
  fn exchange_api_token(
    &self,
    subject_token: &str,
  ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}
