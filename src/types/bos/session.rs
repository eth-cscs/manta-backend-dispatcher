use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct BosSession {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tenant: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub operation: Option<Operation>,
  pub template_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stage: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub components: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub include_disabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
  #[serde(rename = "boot")]
  Boot,
  #[serde(rename = "reboot")]
  Reboot,
  #[serde(rename = "shutdown")]
  Shutdown,
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Operation::Boot => "boot",
      Operation::Reboot => "reboot",
      Operation::Shutdown => "shutdown",
    })
  }
}

impl FromStr for Operation {
  type Err = Error;

  fn from_str(operation: &str) -> Result<Operation, Error> {
    match operation {
      "boot" => Ok(Operation::Boot),
      "reboot" => Ok(Operation::Reboot),
      "shutdown" => Ok(Operation::Shutdown),
      _ => Err(Error::Message("Operation not valid".to_string())),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
  pub start_time: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_time: Option<String>,
  pub status: StatusLabel,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusLabel {
  #[serde(rename = "pending")]
  Pending,
  #[serde(rename = "running")]
  Running,
  #[serde(rename = "complete")]
  Complete,
}
