use std::collections::HashMap;

use crate::{error::Error, types::Group};

pub trait GroupTrait {
    fn get_group_available(
        &self,
        auth_token: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Group>, Error>> + Send;

    fn add_group(
        &self,
        auth_token: &str,
        hsm_name: Group,
    ) -> impl std::future::Future<Output = Result<Group, Error>> + Send;

    fn get_member_vec_from_group_name_vec(
        &self,
        auth_token: &str,
        hsm_group_name_vec: Vec<String>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;

    fn get_group_map_and_filter_by_group_vec(
        &self,
        _auth_token: &str,
        _hsm_name_vec: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>> + Send;
}
