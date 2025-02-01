use serde_json::Value;

use crate::{error::Error, types::HWInventoryByLocationList};

pub trait BackendTrait {
    fn test_backend_trait(&self) -> String;

    // AUTHORIZATION
    fn get_api_token(
        &self,
        _site_name: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

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
    fn migrate_group_members(
        &self,
        shasta_token: &str,
        target_hsm_group_name: &str,
        parent_hsm_group_name: &str,
        new_target_hsm_members: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<(Vec<String>, Vec<String>), Error>> + Send;

    // HSM/INVENTORY/HARDWARE
    fn post_inventory_hardware(
        &self,
        auth_token: &str,
        hardware: HWInventoryByLocationList,
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
