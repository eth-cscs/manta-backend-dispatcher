use std::future::Future;

use crate::error::Error;

pub trait MigrateBackupTrait {
  fn migrate_backup(
    &self,
    _shasta_token: &str,
    _bos: Option<&str>,
    _destination: Option<&str>,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Migrate/backup command not implemented for this backend".to_string(),
      ))
    }
  }
}
