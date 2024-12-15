use std::collections::HashMap;

use serde_json::Value;

use crate::{
    error::Error,
    types::{BootParameters, HsmGroup},
};

pub trait BackendTrait {
    fn test_backend_trait(&self) -> String;

    // AUTHORIZATION
    async fn get_api_token(&self, site_name: &str) -> Result<String, Error> {
        Err(Error::Message(
            "Authentication command not implemented for this backend".to_string(),
        ))
    }

    // HSM/GROUP
    async fn get_hsm_name_available(&self, jwt_token: &str) -> Result<Vec<String>, Error> {
        Err(Error::Message(
            "Get HSM name available command not implemented for this backend".to_string(),
        ))
    }

    // HSM/GROUP
    // FIXME: rename function to 'get_hsm_group_members'
    async fn get_member_vec_from_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_group_name_vec: Vec<String>,
    ) -> Result<Vec<String>, Error> {
        Err(Error::Message(
            "Get group members command not implemented for this backend".to_string(),
        ))
    }

    // HSM/GROUP
    async fn get_hsm_map_and_filter_by_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_name_vec: Vec<&str>,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        Err(Error::Message(
            "Get HSM map and filter by HSM name command not implemented for this backend"
                .to_string(),
        ))
    }

    // HSM/GROUP
    async fn get_all_hsm(&self, auth_token: &str) -> Result<Vec<HsmGroup>, Error> {
        Err(Error::Message(
            "Get all HSM command not implemented for this backend".to_string(),
        ))
    }

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    async fn power_on_sync(&self, auth_token: &str, nodes: &[String]) -> Result<Value, Error> {
        Err(Error::Message(
            "Power on command not implemented for this backend".to_string(),
        ))
    }

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    async fn power_off_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> Result<Value, Error> {
        Err(Error::Message(
            "Power off command not implemented for this backend".to_string(),
        ))
    }

    // PCS
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

    // BSS/BOOTPARAMETERS
    async fn get_bootparameters(
        &self,
        auth_token: &str,
        nodes: &[String],
    ) -> Result<Vec<BootParameters>, Error> {
        Err(Error::Message(
            "Get boot parameters command not implemented for this backend".to_string(),
        ))
    }

    // BSS/BOOTPARAMETERS
    async fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> Result<(), Error> {
        Err(Error::Message(
            "Update boot parameters command not implemented for this backend".to_string(),
        ))
    }

    // BSS/BOOTPARAMETERS
    fn add_kernel_params(&mut self, new_kernel_params: &str) -> Result<bool, Error> {
        Err(Error::Message(
            "Add kernel parameters command not implemented for this backend".to_string(),
        ))
    }
}
