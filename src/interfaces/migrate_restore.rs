use std::future::Future;

use crate::error::Error;

pub trait MigrateRestoreTrait {
  fn migrate_restore(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_file: Option<&String>,
    _cfs_file: Option<&String>,
    _hsm_file: Option<&String>,
    _ims_file: Option<&String>,
    _image_dir: Option<&String>,
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
