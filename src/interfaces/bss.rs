use std::future::Future;

use crate::{error::Error, types::BootParameters};

pub trait BootParametersTrait {
    fn get_bootparameters(
        &self,
        _auth_token: &str,
        _nodes: &[String],
    ) -> impl Future<Output = Result<Vec<BootParameters>, Error>> + Send;

    fn update_bootparameters(
        &self,
        _auth_token: &str,
        _boot_parameters: &BootParameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
