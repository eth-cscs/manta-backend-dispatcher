use serde_json::Value;

use crate::{
    error::Error,
    types::bos::{session::BosSession, session_template::BosSessionTemplate},
};

pub trait ClusterTemplateTrait {
    fn get(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_session_template_id_opt: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send;

    fn get_all(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send;

    fn put(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_template: &BosSessionTemplate,
        bos_template_name: &str,
    ) -> impl std::future::Future<Output = Result<BosSessionTemplate, Error>> + Send;

    fn delete(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_template_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

pub trait ClusterSessionTrait {
    fn post_template_session(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _bos_session: BosSession,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send {
        async {
            Err(Error::Message(
                "Create session template command not implemented for this backend".to_string(),
            ))
        }
    }
}
