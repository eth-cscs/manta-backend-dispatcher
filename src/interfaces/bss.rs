use std::future::Future;

use crate::{error::Error, types::BootParameters};

pub trait BootParametersTrait {
    fn get_bootparameters(
        &self,
        auth_token: &str,
        nodes: &[String], // FIXME: This argument should be a struct of type BootParameters
    ) -> impl Future<Output = Result<Vec<BootParameters>, Error>> + Send;

    fn add_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn update_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn delete_bootparameters(
        &self,
        auth_token: &str,
        boot_parameters: &BootParameters,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}
