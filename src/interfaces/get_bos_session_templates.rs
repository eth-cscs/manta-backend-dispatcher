use std::future::Future;

use crate::{error::Error, types::bos::session_template::BosSessionTemplate};

pub trait GetTemplatesTrait {
    fn get_templates(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _hsm_group_name_vec: &Vec<String>,
        _hsm_member_vec: &[String],
        _bos_sessiontemplate_name_opt: Option<&String>,
        _limit_number_opt: Option<&u8>,
    ) -> impl Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get templates command not implemented for this backend".to_string(),
            ))
        }
    }
}
