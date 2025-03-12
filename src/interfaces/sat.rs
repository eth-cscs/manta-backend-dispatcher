use std::future::Future;

use crate::error::Error;

pub trait SatTrait {
    fn apply_sat_file(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        vault_base_url: &str,
        vault_secret_path: &str,
        k8s_api_url: &str,
        shasta_k8s_secrets: serde_json::Value,
        sat_file_content: String,
        sat_template_file_yaml: serde_yaml::Value,
        hsm_group_param_opt: Option<&String>,
        hsm_group_available_vec: &Vec<String>,
        ansible_verbosity_opt: Option<u8>,
        ansible_passthrough_opt: Option<&String>,
        gitea_base_url: &str,
        gitea_token: &str,
        do_not_reboot: bool,
        watch_logs: bool,
        image_only: bool,
        session_template_only: bool,
        debug_on_failure: bool,
        dry_run: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        async {
            Err(Error::Message(
                "Apply SAT file command not implemented for this backend".to_string(),
            ))
        }
    }
}
