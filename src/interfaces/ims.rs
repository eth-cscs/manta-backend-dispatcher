use std::future::Future;

use crate::{
  error::Error,
  types::ims::{Image, PatchImage},
};

pub trait ImsTrait {
  fn get_images(
    &self,
    _shasta_token: &str,
    _image_id_opt: Option<&str>,
  ) -> impl Future<Output = Result<Vec<Image>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get images command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_all_images(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
  ) -> impl Future<Output = Result<Vec<Image>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get all images command not implemented for this backend".to_string(),
      ))
    }
  }

  fn filter_images(&self, _image_vec: &mut Vec<Image>) -> Result<(), Error> {
    Err(Error::Message(
      "Filter images command not implemented for this backend".to_string(),
    ))
  }

  fn update_image(
    &self,
    _shasta_token: &str,
    _image_id: &str,
    _image: &PatchImage,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Update image command not implemented for this backend".to_string(),
      ))
    }
  }

  fn delete_image(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _image_id: &str,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Delete image command not implemented for this backend".to_string(),
      ))
    }
  }
}

/// Returns a tuple like(Image sruct, cfs configuration name, list of target - either hsm group name
/// or xnames, bool - indicates if image is used to boot a node or not)
/// This method tries to filter by HSM group which means it will make use of:
///  - CFS sessions to find which image id was created against which HSM group
///  - BOS sessiontemplates to find the HSM group related to nodes being rebooted in the past
///  - Image ids in boot params for nodes in HSM groups we are looking for (This is needed to not miss
/// images currenly used which name may not have HSM group we are looking for included not CFS
/// session nor BOS sessiontemplate)
///  - Image names with HSM group name included (This is a bad practice because this is a free text
/// prone to human errors)
pub trait GetImagesAndDetailsTrait {
  fn get_images_and_details(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_group_name_vec: &[String],
    _id_opt: Option<&str>,
    _limit_number: Option<&u8>,
  ) -> impl Future<Output = Result<Vec<(Image, String, String, bool)>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get images and details command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
