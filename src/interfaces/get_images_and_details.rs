use std::future::Future;

use crate::{error::Error, types::ims::Image};

pub trait GetImagesAndDetailsTrait {
    fn get_images_and_details(
        &self,
        _shasta_token: &str,
        _shasta_base_url: &str,
        _shasta_root_cert: &[u8],
        _hsm_group_name_vec: &[String],
        _id_opt: Option<&String>,
        _limit_number: Option<&u8>,
    ) -> impl Future<Output = Result<Vec<(Image, String, String, bool)>, Error>> + Send {
        async {
            Err(Error::Message(
                "Get images and details command not implemented for this backend".to_string(),
            ))
        }
    }
}
