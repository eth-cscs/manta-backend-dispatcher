use std::future::Future;

use crate::error::Error;

pub trait MigrateRestoreTrait {
  fn migrate_restore(
    &self,
    _shasta_token: &str,
    _bos_file: Option<&str>,
    _cfs_file: Option<&str>,
    _hsm_file: Option<&str>,
    _ims_file: Option<&str>,
    _image_dir: Option<&str>,
    _overwrite_group: bool,
    _overwrite_configuration: bool,
    _overwrite_image: bool,
    _overwrite_template: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Migrate/restore command not implemented for this backend".to_string(),
      ))
    }
  }
}
