use std::future::Future;

use crate::types::cfs::cfs_configuration_request::CfsConfigurationRequest;
use crate::types::cfs::{CfsConfigurationResponse, CfsSessionPostRequest, Layer, LayerDetails};
use crate::types::ims::Image;
use crate::types::{BosSessionTemplate, K8sDetails};
use crate::{error::Error, types::cfs::CfsSessionGetResponse};
use futures::AsyncBufRead;

pub trait CfsTrait {
    type T: AsyncBufRead;

    fn post_session(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        session: &CfsSessionPostRequest,
    ) -> impl Future<Output = Result<CfsSessionGetResponse, Error>> + Send;

    fn get_sessions(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        session_name_opt: Option<&String>,
        limit_opt: Option<u8>,
        after_id_opt: Option<String>,
        min_age_opt: Option<String>,
        max_age_opt: Option<String>,
        status_opt: Option<String>,
        name_contains_opt: Option<String>,
        is_succeded_opt: Option<bool>,
        tags_opt: Option<String>,
    ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send;

    fn get_sessions_by_xname(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        xname_vec: &[&str],
        limit_opt: Option<u8>,
        after_id_opt: Option<String>,
        min_age_opt: Option<String>,
        max_age_opt: Option<String>,
        status_opt: Option<String>,
        name_contains_opt: Option<String>,
        is_succeded_opt: Option<bool>,
        tags_opt: Option<String>,
    ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send;

    fn get_and_filter_sessions(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        hsm_group_name_vec_opt: Option<Vec<String>>,
        xname_vec_opt: Option<Vec<&str>>,
        min_age_opt: Option<&String>,
        max_age_opt: Option<&String>,
        status_opt: Option<&String>,
        cfs_session_name_opt: Option<&String>,
        limit_number_opt: Option<&u8>,
        is_succeded_opt: Option<bool>,
    ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send;

    fn get_session_logs_stream(
        &self,
        shasta_token: &str,
        site_name: &str,
        cfs_session_name: &str,
        k8s_api_url: &str,
        k8s: &K8sDetails,
    ) -> impl Future<Output = Result<Self::T, Error>> + Send;

    fn get_session_logs_stream_by_xname(
        &self,
        auth_token: &str,
        site_name: &str,
        xname: &str,
        k8s_api_url: &str,
        k8s: &K8sDetails,
    ) -> impl Future<Output = Result<Self::T, Error>> + Send;

    fn get_configuration(
        &self,
        auth_token: &str,
        base_url: &str,
        root_cert: &[u8],
        configuration_name_opt: Option<&String>,
    ) -> impl Future<Output = Result<Vec<CfsConfigurationResponse>, Error>> + Send;

    fn get_and_filter_configuration(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        configuration_name: Option<&str>,
        configuration_name_pattern: Option<&str>,
        hsm_group_name_vec: &[String],
        limit_number_opt: Option<&u8>,
    ) -> impl Future<Output = Result<Vec<CfsConfigurationResponse>, Error>> + Send;

    fn get_configuration_layer_details(
        &self,
        shasta_root_cert: &[u8],
        gitea_base_url: &str,
        gitea_token: &str,
        layer: Layer,
        site_name: &str, // FIXME: Should we move 'site_name' as Self.site_name?
    ) -> impl Future<Output = Result<LayerDetails, Error>> + Send;

    fn create_configuration_from_repos(
        &self,
        gitea_token: &str,
        gitea_base_url: &str,
        shasta_root_cert: &[u8],
        // repos: Vec<PathBuf>,
        repo_name_vec: Vec<String>,
        local_git_commit_vec: Vec<String>,
        playbook_file_name_opt: Option<&String>,
    ) -> impl Future<Output = Result<CfsConfigurationRequest, Error>>;

    // This function enforces a new CFS configuration to be created. First, checks if CFS configuration
    // with same name already exists in CSM, if that is the case, it will return an error, otherwise
    // creates a new CFS configuration
    fn put_configuration(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        configuration: &CfsConfigurationRequest,
        configuration_name: &str,
    ) -> impl Future<Output = Result<CfsConfigurationResponse, Error>> + Send;

    fn update_runtime_configuration(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        xnames: Vec<String>,
        desired_configuration: &str,
        enabled: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    // Get all CFS sessions, IMS images and BOS sessiontemplates related to a CFS configuration
    fn get_derivatives(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        configuration_name: &str,
    ) -> impl Future<
        Output = Result<
            (
                Option<Vec<CfsSessionGetResponse>>,
                Option<Vec<BosSessionTemplate>>,
                Option<Vec<Image>>,
            ),
            Error,
        >,
    > + Send;
}
