use std::future::Future;

use crate::error::Error;

pub trait MigrateBackupTrait {
    fn migrate_backup(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos: Option<&String>,
        destination: Option<&String>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
