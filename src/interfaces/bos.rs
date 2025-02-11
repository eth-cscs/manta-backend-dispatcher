use crate::{error::Error, types::BosSessionTemplate};

pub trait ClusterTemplateTrait {
    fn get(
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_session_template_id_opt: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send;

    fn get_all(
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send;

    fn put(
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_template: &BosSessionTemplate,
        bos_template_name: &str,
    ) -> impl std::future::Future<Output = Result<BosSessionTemplate, Error>> + Send;

    fn delete(
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        bos_template_id: &str,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
