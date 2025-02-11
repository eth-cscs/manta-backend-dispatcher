use crate::{error::Error, types::Group};

pub trait BootParametersTrait {
    fn get_logs(
        &self,
        auth_token: &str,
        vault_base_url: &str,
        vault_secret_path: &str,
        vault_role_id: &str,
        k8s_api_url: &str,
        group_available_vec: &[Group],
        user_input: &str,
    ) -> Result<(), Error>;
}
