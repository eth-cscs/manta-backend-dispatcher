use std::collections::HashMap;

use serde_json::Value;

use crate::{
    error::Error,
    types::{BootParameters, ComponentArrayPostArray, Group, HWInventoryByLocationList},
};

pub trait BackendTrait {
    fn test_backend_trait(&self) -> String;

    // AUTHORIZATION
    fn get_api_token(
        &self,
        _site_name: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    /* fn get_api_token(
        &self,
        _site_name: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send {
        async {
            Err(Error::Message(
                "Authentication command not implemented for this backend".to_string(),
            ))
        }
    } */

    // AUTHORIZATION : HSM/GROUP
    fn get_group_name_available(
        &self,
        _jwt_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;
    /* fn get_hsm_name_available(
        &self,
        _jwt_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get HSM name available command not implemented for this backend".to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn get_member_vec_from_group_name_vec(
        &self,
        _auth_token: &str,
        _hsm_group_name_vec: Vec<String>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;
    /* fn get_member_vec_from_hsm_name_vec(
        &self,
        _auth_token: &str,
        _hsm_group_name_vec: Vec<String>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get group members command not implemented for this backend".to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn get_group_map_and_filter_by_group_vec(
        &self,
        _auth_token: &str,
        _hsm_name_vec: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>> + Send;
    /* fn get_hsm_map_and_filter_by_hsm_name_vec(
        &self,
        _auth_token: &str,
        _hsm_name_vec: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get HSM map and filter by HSM name command not implemented for this backend"
                    .to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn post_member(
        &self,
        auth_token: &str,
        group_label: &str,
        members: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // HSM/GROUP
    fn add_members_to_group(
        &self,
        auth_token: &str,
        group_label: &str,
        members: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;

    // HSM/GROUP
    fn delete_member_from_group(
        &self,
        auth_token: &str,
        group_label: &str,
        xname: &str,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    // HSM/GROUP
    fn update_group_members(
        &self,
        auth_token: &str,
        group_name: &str,
        members_to_remove: &Vec<String>,
        members_to_add: &Vec<String>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    // HSM/GROUP
    fn get_all_groups(
        &self,
        _auth_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Group>, Error>> + Send;

    // HSM/GROUP
    fn get_group(
        &self,
        _auth_token: &str,
        _hsm_name: &str,
    ) -> impl std::future::Future<Output = Result<Group, Error>> + Send;
    /* fn get_hsm_group(
        &self,
        _auth_token: &str,
        _hsm_name: &str,
    ) -> impl std::future::Future<Output = Result<Vec<HsmGroup>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get HSM command not implemented for this backend".to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn add_group(
        &self,
        _auth_token: &str,
        _hsm_name: Group,
    ) -> impl std::future::Future<Output = Result<Group, Error>> + Send;
    /* fn add_hsm_group(
        &self,
        _auth_token: &str,
        _hsm_name: HsmGroup,
    ) -> impl std::future::Future<Output = Result<Vec<HsmGroup>, Error>> + Send {
        async {
            Err(Error::Message(
                "Create HSM command not implemented for this backend".to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn delete_group(
        &self,
        _auth_token: &str,
        _hsm_group_label: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
    /* fn delete_hsm_group(
        &self,
        _auth_token: &str,
        _hsm_name: HsmGroup,
    ) -> impl std::future::Future<Output = Result<Vec<HsmGroup>, Error>> + Send {
        async {
            Err(Error::Message(
                "Delete HSM command not implemented for this backend".to_string(),
            ))
        }
    } */

    // HSM/GROUP
    fn migrate_group_members(
        &self,
        shasta_token: &str,
        target_hsm_group_name: &str,
        parent_hsm_group_name: &str,
        new_target_hsm_members: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<(Vec<String>, Vec<String>), Error>> + Send;

    // HSM/COMPONENT
    fn post_nodes(
        &self,
        auth_token: &str,
        component: ComponentArrayPostArray,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    // HSM/COMPONENT
    fn delete_node(
        &self,
        auth_token: &str,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // HSM/INVENTORY/HARDWARE
    fn get_inventory_hardware_query(
        &self,
        auth_token: &str,
        xname: &str,
        r#type: Option<&str>,
        children: Option<bool>,
        parents: Option<bool>,
        partition: Option<&str>,
        format: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // HSM/INVENTORY/HARDWARE
    fn post_inventory_hardware(
        &self,
        auth_token: &str,
        hardware: HWInventoryByLocationList,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_on_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
    /* async {
            Err(Error::Message(
                "Power on command not implemented for this backend".to_string(),
            ))
        }
    } */

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_off_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
        _force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
    /* async {
            Err(Error::Message(
                "Power off command not implemented for this backend".to_string(),
            ))
        }
    } */

    // PCS
    // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
    fn power_reset_sync(
        &self,
        _auth_token: &str,
        _nodes: &[String],
        _force: bool,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
    /* async {
            Err(Error::Message(
                "Power reset command not implemented for this backend".to_string(),
            ))
        }
    } */

    // BSS/BOOTPARAMETERS
    fn get_bootparameters(
        &self,
        _auth_token: &str,
        _nodes: &[String],
    ) -> impl std::future::Future<Output = Result<Vec<BootParameters>, Error>> + Send;
    /* async {
            Err(Error::Message(
                "Get boot parameters command not implemented for this backend".to_string(),
            ))
        }
    } */

    // BSS/BOOTPARAMETERS
    fn update_bootparameters(
        &self,
        _auth_token: &str,
        _boot_parameters: &BootParameters,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /* async {
            Err(Error::Message(
                "Update boot parameters command not implemented for this backend".to_string(),
            ))
        }
    } */

    /* // BSS/BOOTPARAMETERS
    fn add_kernel_params(&mut self, _new_kernel_params: &str) -> Result<bool, Error>;
    /* Err(Error::Message(
            "Add kernel parameters command not implemented for this backend".to_string(),
        ))
    } */ */

    /* // BSS/BOOTPARAMETERS
    fn delete_kernel_params(&mut self, _new_kernel_params: &str) -> Result<bool, Error> {
        Err(Error::Message(
            "Delete kernel parameters command not implemented for this backend".to_string(),
        ))
    } */

    fn get_hsm_map_and_filter_by_hsm_name_vec(
        &self,
        auth_token: &str,
        hsm_name_vec: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>> + Send;

    /// Get list of xnames from NIDs
    /// The list of NIDs can be:
    ///     - comma separated list of NIDs (eg: nid000001,nid000002,nid000003)
    ///     - regex (eg: nid00000.*)
    ///     - hostlist (eg: nid0000[01-15])
    fn nid_to_xname(
        &self,
        auth_token: &str,
        user_input_nid: &str,
        is_regex: bool,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;
}
