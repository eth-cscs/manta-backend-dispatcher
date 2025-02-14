use std::future::Future;

use crate::types::K8sDetails;
use crate::{error::Error, types::cfs::CfsSessionGetResponse};
use futures::AsyncBufRead;

pub trait CfsTrait {
    type T: AsyncBufRead;

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
    ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send;

    fn get_session_logs_stream(
        &self,
        cfs_session_name: &str,
        k8s_api_url: &str,
        k8s: &K8sDetails,
    ) -> impl Future<Output = Result<Self::T, Error>> + Send;

    fn get_session_logs_stream_by_xname(
        &self,
        auth_token: &str,
        xname: &str,
        k8s_api_url: &str,
        k8s: &K8sDetails,
    ) -> impl Future<Output = Result<Self::T, Error>> + Send;
}
