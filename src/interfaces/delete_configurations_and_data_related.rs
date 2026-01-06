use std::future::Future;

use chrono::NaiveDateTime;

use crate::{
  error::Error,
  types::cfs::{
    cfs_configuration_response::CfsConfigurationResponse,
    session::CfsSessionGetResponse,
  },
};

pub trait DeleteConfigurationsAndDataRelatedTrait {
  fn get_data_to_delete(
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_name_available_vec: &[&str],
    _configuration_name_pattern_opt: Option<&str>,
    _since_opt: Option<NaiveDateTime>,
    _until_opt: Option<NaiveDateTime>,
  ) -> impl Future<
    Output = Result<
      (
        Vec<CfsSessionGetResponse>,
        Vec<(String, String, String)>,
        Vec<String>,
        Vec<String>,
        Vec<(String, String, String)>,
        Vec<CfsConfigurationResponse>,
      ),
      Error,
    >,
  > {
    async {
      Err(Error::Message(
        "Get data to delete command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn delete(
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _cfs_configuration_name_vec: &[String],
    _image_id_vec: &[String],
    _cfs_session_name_vec: &[String],
    _bos_sessiontemplate_name_vec: &[String],
  ) -> impl Future<Output = Result<(), Error>> {
    async {
      Err(Error::Message(
        "Delete data command not implemented for this backend".to_string(),
      ))
    }
  }

  /* fn delete_data_related_to_cfs_configuration(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_name_available_vec: &[&str],
    _configuration_name_pattern: Option<&str>,
    _since_opt: Option<NaiveDateTime>,
    _until_opt: Option<NaiveDateTime>,
    _assume_yes: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
                "Delete data related to CFS configuration command not implemented for this backend"
                    .to_string(),
            ))
    }
  } */
}
