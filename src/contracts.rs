use serde_json::Value;

use crate::{error::Error, types::BootParameters};

pub trait BackendTrait {
    async fn get_api_token(&self, site_name: &str) -> Result<String, Error> {
        Err(Error::Message(
            "Authentication command not implemented for this backend".to_string(),
        ))
    }

    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    async fn power_on_sync(&self, auth_token: &str, nodes: &[String]) -> Result<Value, Error> {
        Err(Error::Message(
            "Power on command not implemented for this backend".to_string(),
        ))
    }

    /* // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    async fn power_off_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<Value, Error> {
        Err(Error::Message(
            "Power off command not implemented for this backend".to_string(),
        ))
    } */

    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    async fn power_reset_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<Value, Error> {
        Err(Error::Message(
            "Power reset command not implemented for this backend".to_string(),
        ))
    }

    /* async fn get_bootparameters(
        &self,
        auth_token: &str,
        nodes: &[String],
    ) -> Result<Vec<BootParameters>, Error> {
        Err(Error::Message(
            "Get boot parameters command not implemented for this backend".to_string(),
        ))
    } */

    /* async fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> Result<BootParameters, Error> {
        Err(Error::Message(
            "Update boot parameters command not implemented for this backend".to_string(),
        ))
    } */
}
