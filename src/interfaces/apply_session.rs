use std::future::Future;

use crate::error::Error;

pub trait ApplySessionTrait {
  fn apply_session(
    &self,
    _gitea_token: &str,
    _gitea_base_url: &str,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    // _k8s_api_url: &str,
    _cfs_conf_sess_name: Option<&str>,
    _playbook_yaml_file_name_opt: Option<&str>,
    _hsm_group: Option<&str>,
    _repo_name_vec: &[&str],
    _repo_last_commit_id_vec: &[&str],
    _ansible_limit: Option<&str>,
    _ansible_verbosity: Option<&str>,
    _ansible_passthrough: Option<&str>,
    // _watch_logs: bool,
  ) -> impl Future<Output = Result<(String, String), Error>> + Send {
    async {
      Err(Error::Message(
        "Apply session command not implemented for this backend".to_string(),
      ))
    }
  }
}
