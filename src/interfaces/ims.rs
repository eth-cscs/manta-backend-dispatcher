use std::future::Future;

use crate::{error::Error, types::ims::Image};

pub trait ImsTrait {
    fn get_images(
        &self,
        shasta_token: &str,
        shasta_base_url: &str,
        shasta_root_cert: &[u8],
        image_id_opt: Option<&str>,
    ) -> impl Future<Output = Result<Vec<Image>, Error>> + Send;
}
