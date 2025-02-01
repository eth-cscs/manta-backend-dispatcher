use serde_json::Value;

use crate::error::Error;

pub trait PCSTrait {
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_on_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_off_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
        _force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_reset_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
        _force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
}
