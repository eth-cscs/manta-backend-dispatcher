use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BootParameters {
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nids: Option<Vec<u32>>,
    #[serde(default)]
    pub params: String, // FIXME: change type to HashMap<String, String> by using function
    // bss::utils::convert_kernel_params_to_map AND create new method
    // bss::BootParameters::num_kernel_params which returns the list of kernel parameters
    #[serde(default)]
    pub kernel: String,
    #[serde(default)]
    pub initrd: String,
    #[serde(rename = "cloud-init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_init: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HsmGroup {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "exclusiveGroup"))]
    pub exclusive_group: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct XnameId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BootParameters {
    /// Add a kernel parameter:
    ///  - if kernel parameter does not exists, then it will be added,
    /// otherwise nothing will change
    /// Returns true if kernel params have change
    pub fn add_kernel_params(&mut self, new_kernel_params: &str) -> bool {
        let mut changed = false;
        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        let new_kernel_params_tuple: HashMap<&str, &str> = new_kernel_params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .collect();

        for (key, new_value) in new_kernel_params_tuple {
            // NOTE: do not use --> `params.entry(key).or_insert(new_value);` otherwise, I don't know
            // how do we know if the key already exists or not
            if params.contains_key(key) {
                log::info!("key '{}' already exists, the new kernel parameter won't be added since it already exists", key);
                return changed;
            } else {
                log::info!(
                    "key '{}' not found, adding new kernel param with value '{}'",
                    key,
                    new_value
                );
                params.insert(key, new_value);
                changed = true
            }
        }

        self.params = params
            .iter()
            .map(|(key, value)| {
                if !value.is_empty() {
                    format!("{key}={value}")
                } else {
                    key.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        changed
    }
}
