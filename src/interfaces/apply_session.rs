use std::{future::Future, path::PathBuf};

use crate::{
    error::Error,
    types::{kafka::Kafka, K8sDetails},
};

pub trait ApplySessionTrait {
    fn apply_session(
        &self,
        gitea_token: &str,
        gitea_base_url: &str,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        k8s_api_url: &str,
        cfs_conf_sess_name: Option<&String>,
        playbook_yaml_file_name_opt: Option<&String>,
        hsm_group: Option<&String>,
        repos_paths: Vec<PathBuf>,
        ansible_limit: Option<String>,
        ansible_verbosity: Option<String>,
        ansible_passthrough: Option<String>,
        watch_logs: bool,
        /* kafka_audit: &Kafka,
        k8s: &K8sDetails, */
    ) -> impl Future<Output = Result<(String, String), Error>> + Send;
}
