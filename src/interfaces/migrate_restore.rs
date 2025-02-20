use std::future::Future;

use crate::error::Error;

pub trait MigrateRestoreTrait {
    fn migrate_restore(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_file: Option<&String>,
        cfs_file: Option<&String>,
        hsm_file: Option<&String>,
        ims_file: Option<&String>,
        image_dir: Option<&String>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
