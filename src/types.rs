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
