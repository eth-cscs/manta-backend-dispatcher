use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

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
    // Assumes s3 path looks like:
    // - s3://boot-images/59e0180a-3fdd-4936-bba7-14ba914ffd34/kernel
    // - craycps-s3:s3://boot-images/59e0180a-3fdd-4936-bba7-14ba914ffd34/rootfs:3dfae8d1fa3bb2bfb18152b4f9940ad0-667:dvs:api-gw-service-nmn.local:300:nmn0,hsn0:0
    // - url=s3://boot-images/59e0180a-3fdd-4936-bba7-14ba914ffd34/rootfs,etag=3dfae8d1fa3bb2bfb18152b4f9940ad0-667 bos_update_frequency=4h
    pub fn get_image_id_from_s3_path(s3_path: &str) -> Option<&str> {
        s3_path.split("/").skip(3).next()
    }

    /// Returns the image id. This function may fail since it assumes kernel path has the following
    // FIXME: Change function signature so it returns a Result<String, Error> instead of String
    pub fn get_boot_image(&self) -> String {
        let params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| {
                kernel_param
                    .split_once('=')
                    .map(|(key, value)| (key.trim(), value.trim()))
                    .unwrap_or((kernel_param, ""))
            })
            .collect();

        // NOTE: CN nodes have UIID image id in 'root' kernel parameter
        // Get `root` kernel parameter and split it by '/'
        let root_kernel_param_opt = params.get("root");
        // NOTE: CN nodes have UIID image id in 'metal.server' kernel parameter
        // Get `root` kernel parameter and split it by '/'
        let metal_server_kernel_param_opt = params.get("metal.server");

        let boot_image_id_opt: Option<&str> = if let Some(root_kernel_param) = root_kernel_param_opt
        {
            Self::get_image_id_from_s3_path(root_kernel_param)
        } else if let Some(metal_server_kernel_param) = metal_server_kernel_param_opt {
            Self::get_image_id_from_s3_path(metal_server_kernel_param)
        } else {
            None
        };

        boot_image_id_opt.unwrap_or("").to_string()

        /* let mut path_elem_vec = self.kernel.split("/").skip(3);

        let mut image_id: String = path_elem_vec.next().unwrap_or_default().to_string();

        for path_elem in path_elem_vec {
            if !path_elem.eq("kernel") {
                image_id = format!("{}/{}", image_id, path_elem);
            } else {
                break;
            }
        }

        image_id */
    }

    /// Update boot image in kernel boot parameters and also in kernel and initrd fields if
    /// exists. Otherwise nothing is changed. This method updates both kernel params related to
    /// NCN and also CN
    /// Returns a boolean that indicates if kernel parameters have change:
    /// - kernel parameter value changed
    ///  - number of kernel parameters have changed
    pub fn update_boot_image(&mut self, new_image_id: &str) -> Result<bool, Error> {
        let mut changed = false;
        // replace image id in 'root' kernel param

        // convert kernel params to a hashmap
        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .collect();

        // NOTE: CN nodes have UIID image id in 'root' kernel parameter
        // Get `root` kernel parameter and split it by '/'
        let root_kernel_param_rslt = params.get("root");

        let mut root_kernel_param: Vec<&str> = match root_kernel_param_rslt {
            Some(root_kernel_param) => root_kernel_param.split("/").collect::<Vec<&str>>(),
            None => {
                return Err(Error::Message(
                    "ERROR - The 'root' kernel param is missing from user input".to_string(),
                ));
            }
        };

        // Replace image id in root kernel param with new image id
        for current_image_id in &mut root_kernel_param {
            // Look for any substring between '/' that matches an UUID formant and take it as
            // the image id
            if uuid::Uuid::try_parse(current_image_id).is_ok() {
                if *current_image_id != new_image_id {
                    changed = true;
                }
                // Replace image id in `root` kernel parameter with new value
                *current_image_id = new_image_id;
            }
        }

        // Create new `root` kernel param string
        let new_root_kernel_param = root_kernel_param.join("/");

        // Create new kernel parameters
        params
            .entry("root")
            .and_modify(|root_param| *root_param = &new_root_kernel_param);

        self.update_kernel_param("root", &new_root_kernel_param);

        // replace image id in 'nmd_data' kernel param

        // convert kernel params to a hashmap
        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        // NOTE: NCN nodes have UUID image id in 'metal.server' kernel parameter
        let mut metal_server_kernel_param: Vec<&str>;
        if let Some(metal_server_data) = params.get("metal.server") {
            metal_server_kernel_param = metal_server_data.split("/").collect();

            for substring in &mut metal_server_kernel_param {
                if uuid::Uuid::try_parse(substring).is_ok() {
                    *substring = new_image_id;
                    changed = true;
                }
            }

            let new_metal_server_kernel_param = metal_server_kernel_param.join("/");

            params
                .entry("metal.server")
                .and_modify(|metal_server_param| {
                    *metal_server_param = &new_metal_server_kernel_param
                });

            self.update_kernel_param("metal.server", &new_metal_server_kernel_param);

            // convert kernel params to a hashmap
            params = self
                .params
                .split_whitespace()
                .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
                .collect();
        } else {
        };

        // NOTE: NCN nodes have UUID image id 'nmd_data' kernel parameter
        let mut nmd_kernel_param: Vec<&str>;
        if let Some(nmd_data) = params.get("nmd_data") {
            nmd_kernel_param = nmd_data.split("/").collect();

            for substring in &mut nmd_kernel_param {
                if uuid::Uuid::try_parse(substring).is_ok() {
                    *substring = new_image_id;
                    changed = true;
                }
            }

            let new_nmd_kernel_param = nmd_kernel_param.join("/");

            params
                .entry("nmd_data")
                .and_modify(|nmd_param| *nmd_param = &new_nmd_kernel_param);

            self.update_kernel_param("nmd_data", &new_nmd_kernel_param);
        } else {
        };

        self.kernel = format!("s3://boot-images/{}/kernel", new_image_id);

        self.initrd = format!("s3://boot-images/{}/initrd", new_image_id);

        Ok(changed)
    }

    /// Set a str of kernel parameters:
    ///  - if kernel parameter already exists, then it will be updated
    ///  - if kernel parameter does not exists, then it will be added
    /// Returns true if kernel params have change
    pub fn update_kernel_params(&mut self, new_params: &str) -> bool {
        let mut change = false;

        let new_params: Vec<(&str, &str)> = new_params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .collect();

        for (new_key, new_value) in &new_params {
            for (key, value) in params.iter_mut() {
                if *key == *new_key {
                    log::debug!("key '{}' found", key);
                    if value != new_value {
                        log::info!("changing key {} from {} to {}", key, value, new_value);

                        *value = new_value;
                        change = true
                    } else {
                        log::debug!("key '{}' value does not change ({})", key, value);
                    }
                }
            }
        }

        if change == false {
            log::debug!("No value change in kernel params. Checking is either new params have been added or removed");
            if new_params.len() != params.len() {
                log::info!("num kernel parameters have changed");
                change = true;
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

        change
    }

    /// Update kernel parameter. If kernel parameter exists, then it will be updated with new
    /// Note: This function won't make any change to params without values (eg: 'quiet') since
    /// they don't have values
    /// value. otherwise nothing will change
    pub fn update_kernel_param(&mut self, new_key: &str, new_value: &str) -> bool {
        let mut changed = false;
        // convert kernel params to a hashmap
        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        // Update kernel param with new value
        // params.entry(key).and_modify(|value| *value = new_value);
        for (current_key, current_value) in params.iter_mut() {
            if *current_key == new_key {
                log::debug!("key '{}' found", new_key);
                if *current_value != new_value {
                    log::info!(
                        "changing key {} from {} to {}",
                        new_key,
                        current_value,
                        new_value
                    );

                    *current_value = new_value;
                    changed = true
                } else {
                    log::debug!(
                        "key '{}' value does not change ({})",
                        new_key,
                        current_value
                    );
                }
            }
            /* if *current_key == new_key {
                *current_value = new_value;
                changed = true;
            } */
        }

        // Create new kernel params as a string
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

    /// Delete kernel parameter. If kernel parameter exists, then it will be removed, otherwise
    /// nothing will be changed
    /// It accepts kernel params in format 'key=value' or just 'key'
    /// Returns true if kernel params have change
    pub fn delete_kernel_params(&mut self, kernel_params_to_delete: &str) -> bool {
        let mut changed = false;
        let mut params: HashMap<&str, &str> = self
            .params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        let kernel_params_to_delete_tuple: HashMap<&str, &str> = kernel_params_to_delete
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .collect();

        for (key, _value) in kernel_params_to_delete_tuple {
            changed = changed | params.remove(key).is_some();
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
