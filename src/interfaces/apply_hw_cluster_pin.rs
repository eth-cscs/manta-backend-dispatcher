use std::future::Future;

use crate::error::Error;

pub trait ApplyHwClusterPin {
    fn apply_hw_cluster_pin(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        target_hsm_group_name: &str,
        parent_hsm_group_name: &str,
        pattern: &str,
        nodryrun: bool,
        create_target_hsm_group: bool,
        delete_empty_parent_hsm_group: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
