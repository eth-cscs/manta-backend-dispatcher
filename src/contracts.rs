use std::collections::HashMap;

use serde_json::Value;

use crate::{
    error::Error,
    types::{BootParameters, HsmGroup},
};

pub trait BackendTrait {
    fn test_backend_trait(&self) -> String;

    // AUTHORIZATION
    fn get_api_token(
        &self,
        site_name: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send {
        async {
            Err(Error::Message(
                "Authentication command not implemented for this backend".to_string(),
            ))
        }
    }

    // AUTHORIZATION : HSM/GROUP
    fn get_hsm_name_available(
        &self,
        jwt_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get HSM name available command not implemented for this backend".to_string(),
            ))
        }
    }

    // HSM/GROUP
    // FIXME: rename function to 'get_hsm_group_members'
    fn get_member_vec_from_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_group_name_vec: Vec<String>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get group members command not implemented for this backend".to_string(),
            ))
        }
    }

    // HSM/GROUP
    fn get_hsm_map_and_filter_by_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_name_vec: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get HSM map and filter by HSM name command not implemented for this backend"
                    .to_string(),
            ))
        }
    }

    // HSM/GROUP
    fn get_all_hsm(
        &self,
        auth_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<HsmGroup>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get all HSM command not implemented for this backend".to_string(),
            ))
        }
    }

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_on_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send {
        async {
            Err(Error::Message(
                "Power on command not implemented for this backend".to_string(),
            ))
        }
    }

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_off_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send {
        async {
            Err(Error::Message(
                "Power off command not implemented for this backend".to_string(),
            ))
        }
    }

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_reset_sync(
        &self,
        auth_token: &str,
        nodes: &[String],
        force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send {
        async {
            Err(Error::Message(
                "Power reset command not implemented for this backend".to_string(),
            ))
        }
    }

    // BSS/BOOTPARAMETERS
    fn get_bootparameters(
        &self,
        auth_token: &str,
        nodes: &[String],
    ) -> impl std::future::Future<Output = Result<Vec<BootParameters>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get boot parameters command not implemented for this backend".to_string(),
            ))
        }
    }

    // BSS/BOOTPARAMETERS
    fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send {
        async {
            Err(Error::Message(
                "Update boot parameters command not implemented for this backend".to_string(),
            ))
        }
    }

    // BSS/BOOTPARAMETERS
    fn add_kernel_params(&mut self, new_kernel_params: &str) -> Result<bool, Error> {
        Err(Error::Message(
            "Add kernel parameters command not implemented for this backend".to_string(),
        ))
    }
}
