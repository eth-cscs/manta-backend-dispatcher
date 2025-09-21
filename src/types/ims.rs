use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImsImageRecord2Update {
  pub link: Link,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Link {
  pub path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub etag: Option<String>,
  pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Image {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<Link>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<HashMap<String, String>>,
}

pub struct PatchMetadata {
  pub operation: String,
  pub key: String,
  pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PatchImage {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<Link>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<HashMap<String, String>>,
}

impl From<PatchImage> for Image {
  fn from(patch: PatchImage) -> Self {
    Image {
      id: None,
      created: None,
      name: String::new(),
      link: patch.link,
      arch: patch.arch,
      metadata: patch.metadata,
    }
  }
}

impl Into<PatchImage> for Image {
  fn into(self) -> PatchImage {
    PatchImage {
      link: self.link,
      arch: self.arch,
      metadata: self.metadata,
    }
  }
}

impl Image {
  pub fn set_boot_image_iscsi_ready(&mut self) {
    self.add_metadata(&[("sbps-project", "true")]);
  }

  pub fn add_metadata(&mut self, pairs: &[(&str, &str)]) {
    for (key, value) in pairs {
      self
        .metadata
        .as_mut()
        .map(|metadata| metadata.insert(key.to_string(), value.to_string()));
    }
  }
}
