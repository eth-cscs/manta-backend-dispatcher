pub mod cfs;
pub mod ims;
pub mod kafka;

use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum K8sAuth {
    Native {
        certificate_authority_data: String,
        client_certificate_data: String,
        client_key_data: String,
    },
    Vault {
        base_url: String,
        secret_path: String,
        role_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct K8sDetails {
    pub api_url: String,
    pub authentication: K8sAuth,
}

// From CSM used by Manta
#[derive(
    Debug, EnumIter, EnumString, IntoStaticStr, AsRefStr, Display, Serialize, Deserialize, Clone,
)]
pub enum ArtifactType {
    Memory,
    Processor,
    NodeAccel,
    NodeHsnNic,
    Drive,
    CabinetPDU,
    CabinetPDUPowerConnector,
    CMMRectifier,
    NodeAccelRiser,
    NodeEnclosurePowerSupplie,
    NodeBMC,
    RouterBMC,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeSummary {
    pub xname: String,
    pub r#type: String,
    pub processors: Vec<ArtifactSummary>,
    pub memory: Vec<ArtifactSummary>,
    pub node_accels: Vec<ArtifactSummary>,
    pub node_hsn_nics: Vec<ArtifactSummary>,
}

/* impl From<HWInventoryByLocation> for NodeSummary {
    fn from(value: HWInventoryByLocation) -> Self {
        NodeSummary {
            xname: value.id,
            r#type: value.r#type.unwrap_or_default(),
            processors: value
                .processors
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocProcessor to ArtifactSummary
            memory: value
                .memory
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocMemory to ArtifactSummary
            node_accels: value
                .node_accels
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocNodeAccel to ArtifactSummary
            node_hsn_nics: value
                .node_hsn_nics
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocHSNNIC to ArtifactSummary
        }
    }
}

impl Into<HWInventoryByLocation> for NodeSummary {
    fn into(self) -> HWInventoryByLocation {
        let redfish_system_fru_info = RedfishSystemFRUInfo {
            asset_tag: None,
            bios_version: None,
            model: None,
            manufacturer: None,
            part_number: None,
            serial_number: None,
            sku: None,
            system_type: None,
            uuid: None,
        };

        let hw_inv_by_fr_node = HWInvByFRUNode {
            fru_id: None,
            r#type: None,
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.clone(),
            node_fru_info: redfish_system_fru_info,
        };

        let processor_summary = ProcessorSummary {
            count: self.processors.len().try_into().ok(),
            model: self.processors.first().unwrap().info.clone(),
        };

        let memory_capacity_vec: Vec<usize> = self
            .memory
            .iter()
            .map(|memory| {
                let binding = "".to_string();
                let mem_capacity = memory.info.as_ref().unwrap_or(&binding);
                /* .strip_suffix(" MiB")
                .unwrap_or_default(); */

                usize::from_str(mem_capacity.strip_suffix(" MiB").unwrap_or_default())
                    .unwrap_or_default()
            })
            .collect();

        let memory_summary = MemorySummary {
            total_system_memory_gib: Some(memory_capacity_vec.iter().sum::<usize>() as u32),
        };

        let node_location_info = NodeLocationInfo {
            id: self.xname.clone(),
            name: None,
            description: None,
            hostname: None,
            processor_summary: Some(processor_summary),
            memory_summary: Some(memory_summary),
        };

        HWInventoryByLocation {
            id: self.xname,
            r#type: Some(self.r#type.clone()),
            ordinal: None,
            status: None,
            node_location_info,
            hw_inventory_by_location_type: self.r#type,
            populated_fru: Some(hw_inv_by_fr_node),
            cabinets: None,
            chassis: None,
            compute_modules: None,
            router_modules: None,
            node_enclosures: None,
            hsn_boards: None,        // FIXME: implement!
            mgmt_switches: None,     // FIXME: implement!
            mgmt_hl_switches: None,  // FIXME: implement!
            cdu_mgmt_switches: None, // FIXME: implement!
            nodes: None,             // FIXME: implement!
            processors: Some(
                self.processors
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect(),
            ), // FIXME: implement!
            node_accels: Some(
                self.node_accels
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect(),
            ), // FIXME: implement!
            drives: None,            // FIXME: implement!
            memory: Some(
                self.memory
                    .into_iter()
                    .map(|memory| memory.into())
                    .collect(),
            ), // FIXME: implement!
            cabinet_pdus: None,      // FIXME: implement!
            cabinet_pdu_power_connectors: None, // FIXME: implement!
            cmm_rectifiers: None,    // FIXME: implement!
            node_accel_risers: None, // FIXME: implement!
            node_hsn_nics: Some(
                self.node_hsn_nics
                    .into_iter()
                    .map(|node_hsn_nic| node_hsn_nic.into())
                    .collect(),
            ), // FIXME: implement!
            node_enclosure_power_supplies: None, // FIXME: implement!
            node_bmc: None,
            router_bmc: None, // FIXME: implement!
        }
    }
} */

impl From<HWInvByLocNode> for NodeSummary {
    fn from(value: HWInvByLocNode) -> Self {
        NodeSummary {
            xname: value.id,
            r#type: value.r#type.unwrap_or_default(),
            processors: value
                .processors
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocProcessor to ArtifactSummary
            memory: value
                .memory
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocMemory to ArtifactSummary
            node_accels: value
                .node_accels
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocNodeAccel to ArtifactSummary
            node_hsn_nics: value
                .node_hsn_nics
                .unwrap_or_default()
                .into_iter()
                .map(ArtifactSummary::from)
                .collect(),
            // FIXME: implement FROM trait from HWInvByLlocHSNNIC to ArtifactSummary
        }
    }
}

impl Into<HWInvByLocNode> for NodeSummary {
    fn into(self) -> HWInvByLocNode {
        let redfish_system_fru_info = RedfishSystemFRUInfo {
            asset_tag: None,
            bios_version: None,
            model: None,
            manufacturer: None,
            part_number: None,
            serial_number: None,
            sku: None,
            system_type: None,
            uuid: None,
        };

        let hw_inv_by_fr_node = HWInvByFRUNode {
            fru_id: None,
            r#type: None,
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.clone(),
            node_fru_info: redfish_system_fru_info,
        };

        HWInvByLocNode {
            id: self.xname,
            r#type: Some(self.r#type.clone()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type,
            populated_fru: Some(hw_inv_by_fr_node),
            node_location_info: None,
            processors: Some(
                self.processors
                    .into_iter()
                    .map(|processor| processor.into())
                    .collect(),
            ),
            node_accels: Some(
                self.node_accels
                    .into_iter()
                    .map(|node_accel| node_accel.into())
                    .collect(),
            ),
            drives: None,
            memory: Some(
                self.memory
                    .into_iter()
                    .map(|memory| memory.into())
                    .collect(),
            ),
            node_accel_risers: None,
            node_hsn_nics: Some(
                self.node_hsn_nics
                    .into_iter()
                    .map(|node_hsn_nic| node_hsn_nic.into())
                    .collect(),
            ),
        }
    }
}

impl NodeSummary {
    pub fn from_csm_value(hw_artifact_value: Value) -> Self {
        let processors = hw_artifact_value["Processors"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|processor_value| ArtifactSummary::from_processor_value(processor_value.clone()))
            .collect();

        let memory = hw_artifact_value["Memory"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|memory_value| ArtifactSummary::from_memory_value(memory_value.clone()))
            .collect();

        let node_accels = hw_artifact_value["NodeAccels"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|nodeaccel_value| ArtifactSummary::from_nodeaccel_value(nodeaccel_value.clone()))
            .collect();

        let node_hsn_nics = hw_artifact_value["NodeHsnNics"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|nodehsnnic_value| {
                ArtifactSummary::from_nodehsnnics_value(nodehsnnic_value.clone())
            })
            .collect();

        Self {
            xname: hw_artifact_value["ID"].as_str().unwrap().to_string(),
            r#type: hw_artifact_value["Type"].as_str().unwrap().to_string(),
            processors,
            memory,
            node_accels,
            node_hsn_nics,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactSummary {
    pub xname: String,
    pub r#type: ArtifactType,
    pub info: Option<String>,
}

impl From<HWInvByLocProcessor> for ArtifactSummary {
    fn from(value: HWInvByLocProcessor) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_processor| {
                hw_inv_by_fru_processor.processor_fru_info.model
            }),
        }
    }
}

impl Into<HWInvByLocProcessor> for ArtifactSummary {
    fn into(self) -> HWInvByLocProcessor {
        let redfish_process_fru_info = RedfishProcessorFRUInfo {
            instruction_set: None,
            manufacturer: None,
            max_speed_mhz: None,
            model: self.info,
            processor_architecture: None,
            processor_id: None,
            processor_type: None,
            total_cores: None,
            total_threads: None,
        };

        let hw_inv_by_fru_processor = HWInvByFRUProcessor {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            processor_fru_info: redfish_process_fru_info,
        };

        let processor_location_info = RedfishProcessorLocationInfo {
            id: None,
            name: None,
            description: None,
            socket: None,
        };

        HWInvByLocProcessor {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_processor),
            processor_location_info,
        }
    }
}

impl From<HWInvByLocMemory> for ArtifactSummary {
    fn from(value: HWInvByLocMemory) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_memory| {
                hw_inv_by_fru_memory
                    .memory_fru_info
                    .capacity_mib
                    .map(|capacity_mib| capacity_mib.to_string())
            }),
        }
    }
}

impl Into<HWInvByLocMemory> for ArtifactSummary {
    fn into(self) -> HWInvByLocMemory {
        let redfish_memory_fru_info = RedfishMemoryFRUInfo {
            base_module_type: None,
            bus_width_bits: None,
            capacity_mib: self.info.map(|info| usize::from_str(&info).unwrap_or(0)), // FIXME:
            // settings memory capacity to 0 if any issue... this does not look ok...
            data_width_bits: None,
            error_correction: None,
            manufacturer: None,
            memory_type: None,
            memory_device_type: None,
            operating_speed_mhz: None,
            part_number: None,
            rank_count: None,
            serial_number: None,
        };

        let hw_inv_by_fru_memory = HWInvByFRUMemory {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            memory_fru_info: redfish_memory_fru_info,
        };

        let memory_location = MemoryLocation {
            socket: None,
            memory_controller: None,
            channel: None,
            slot: None,
        };

        let redfish_memory_location_info = RedfishMemoryLocationInfo {
            id: None,
            name: None,
            description: None,
            memory_location: Some(memory_location),
        };

        HWInvByLocMemory {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_memory),
            memory_location_info: redfish_memory_location_info,
        }
    }
}

impl From<HWInvByLocNodeAccel> for ArtifactSummary {
    fn from(value: HWInvByLocNodeAccel) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value.populated_fru.and_then(|hw_inv_by_fru_node_accel| {
                hw_inv_by_fru_node_accel.node_accel_fru_info.model
            }),
        }
    }
}

impl Into<HWInvByLocNodeAccel> for ArtifactSummary {
    fn into(self) -> HWInvByLocNodeAccel {
        // NOTE: yes, sounds weird FRU for node accelerator uses FRU for processor... but that is
        // what the API docs says...
        let redfish_processor_fru_info = RedfishProcessorFRUInfo {
            instruction_set: None,
            manufacturer: None,
            max_speed_mhz: None,
            model: self.info,
            processor_architecture: None,
            processor_id: None,
            processor_type: None,
            total_cores: None,
            total_threads: None,
        };

        let hw_inv_by_fru_node_accel = HWInvByFRUNodeAccel {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            node_accel_fru_info: redfish_processor_fru_info,
        };

        HWInvByLocNodeAccel {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_node_accel),
            node_accel_location_info: None,
        }
    }
}

impl From<HWInvByLocHSNNIC> for ArtifactSummary {
    fn from(value: HWInvByLocHSNNIC) -> Self {
        ArtifactSummary {
            xname: value.id,
            r#type: ArtifactType::from_str(value.r#type.unwrap().as_str()).unwrap(),
            info: value
                .populated_fru
                .and_then(|hw_inv_by_fru_hsn_nic| hw_inv_by_fru_hsn_nic.hsn_nic_fru_info.model),
        }
    }
}

impl Into<HWInvByLocHSNNIC> for ArtifactSummary {
    fn into(self) -> HWInvByLocHSNNIC {
        // NOTE: yes, sounds weird FRU for node accelerator uses FRU for processor... but that is
        // what the API docs says...
        let hsn_nic_fru_info = HSNNICFRUInfo {
            manufacturer: None,
            model: self.info,
            part_number: None,
            sku: None,
            serial_number: None,
        };

        let hw_inv_by_fru_hsn_nic = HWInvByFRUHSNNIC {
            fru_id: None,
            r#type: Some(self.r#type.to_string()),
            fru_sub_type: None,
            hw_inventory_by_fru_type: self.r#type.to_string(),
            hsn_nic_fru_info,
        };

        let hsn_nic_location_info = HSNNICLocationInfo {
            id: None,
            name: None,
            description: None,
        };

        HWInvByLocHSNNIC {
            id: self.xname,
            r#type: Some(self.r#type.to_string()),
            ordinal: None,
            status: None,
            hw_inventory_by_location_type: self.r#type.to_string(),
            populated_fru: Some(hw_inv_by_fru_hsn_nic),
            hsn_nic_location_info,
        }
    }
}

impl ArtifactSummary {
    fn from_processor_value(processor_value: Value) -> Self {
        Self {
            xname: processor_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(processor_value["Type"].as_str().unwrap()).unwrap(),
            info: processor_value
                .pointer("/PopulatedFRU/ProcessorFRUInfo/Model")
                .map(|model| model.as_str().unwrap().to_string()),
        }
    }

    fn from_memory_value(memory_value: Value) -> Self {
        Self {
            xname: memory_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(memory_value["Type"].as_str().unwrap()).unwrap(),
            info: memory_value
                .pointer("/PopulatedFRU/MemoryFRUInfo/CapacityMiB")
                .map(|capacity_mib| capacity_mib.as_number().unwrap().to_string() + " MiB"),
        }
    }

    fn from_nodehsnnics_value(nodehsnnic_value: Value) -> Self {
        Self {
            xname: nodehsnnic_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(nodehsnnic_value["Type"].as_str().unwrap()).unwrap(),
            info: nodehsnnic_value
                .pointer("/NodeHsnNicLocationInfo/Description")
                .map(|description| description.as_str().unwrap().to_string()),
        }
    }

    fn from_nodeaccel_value(nodeaccel_value: Value) -> Self {
        Self {
            xname: nodeaccel_value["ID"].as_str().unwrap().to_string(),
            r#type: ArtifactType::from_str(nodeaccel_value["Type"].as_str().unwrap()).unwrap(),
            info: nodeaccel_value
                .pointer("/PopulatedFRU/NodeAccelFRUInfo/Model")
                .map(|model| model.as_str().unwrap().to_string()),
        }
    }
}

// From OCHAMI API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exclusiveGroup")]
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

impl Group {
    /// Constructor
    pub fn new(
        label: &str,
        description: Option<String>,
        member_vec_opt: Option<Vec<String>>,
        tag_vec_opt: Option<Vec<String>>,
        exclusive_opt: Option<String>,
    ) -> Self {
        let members_opt = if let Some(member_vec) = member_vec_opt {
            Some(Member {
                ids: Some(member_vec),
            })
        } else {
            None
        };

        let group = Self {
            label: label.to_string(),
            description,
            tags: tag_vec_opt,
            members: members_opt,
            exclusive_group: exclusive_opt,
        };

        group
    }

    /// Get group members
    pub fn get_members(&self) -> Vec<String> {
        self.members
            .clone()
            .map(|member| member.ids.unwrap_or_default())
            .unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetadataArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Components")]
    pub components: Option<Vec<Component>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Flag")]
    pub flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SoftwareStatus")]
    pub software_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SubRole")]
    pub sub_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NID")]
    pub nid: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Subtype")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NetType")]
    pub net_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Class")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ReservationDisabled")]
    pub reservation_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Locked")]
    pub locked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentArrayPostQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ComponentIDs")]
    pub component_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stateonly")]
    pub state_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flagonly")]
    pub falg_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "roleonly")]
    pub role_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nidonly")]
    pub nid_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "softwarestatus")]
    pub software_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subrole: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nid_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nid_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentArrayPostByNidQuery {
    #[serde(rename = "NIDRanges")]
    pub nid_ranges: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stateonly")]
    pub state_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flagonly")]
    pub falg_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "roleonly")]
    pub role_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nidonly")]
    pub nid_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentArrayPostArray {
    #[serde(rename = "Components")]
    pub components: Vec<ComponentCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentCreate {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Flag")]
    pub flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SoftwareStatus")]
    pub software_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SubRole")]
    pub sub_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NID")]
    pub nid: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Subtype")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NetType")]
    pub net_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Class")]
    pub class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentPut {
    pub component: ComponentCreate,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

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

    /// Apply a str of kernel parameters:
    ///  - current kernel params will be ignored/removed and replaced by the new ones
    /// Returns true if kernel params have change
    // FIXME: integrate fixed introduced in methods 'add_kernel_param' and 'delete_kernel_param'
    pub fn apply_kernel_params(&mut self, new_params: &str) -> bool {
        let mut change = false;

        let new_params: Vec<(&str, &str)> = new_params
            .split_whitespace()
            .map(|kernel_param| kernel_param.split_once('=').unwrap_or((kernel_param, "")))
            .map(|(key, value)| (key.trim(), value.trim()))
            .collect();

        let mut params: HashMap<&str, &str> = HashMap::new();

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

        self.params = new_params
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
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComponentType {
    CDU,
    CabinetCDU,
    CabinetPDU,
    CabinetPDUOutlet,
    CabinetPDUPowerConnector,
    CabinetPDUController,
    r#Cabinet,
    Chassis,
    ChassisBMC,
    CMMRectifier,
    CMMFpga,
    CEC,
    ComputeModule,
    RouterModule,
    NodeBMC,
    NodeEnclosure,
    NodeEnclosurePowerSupply,
    HSNBoard,
    Node,
    Processor,
    Drive,
    StorageGroup,
    NodeNIC,
    Memory,
    NodeAccel,
    NodeAccelRiser,
    NodeFpga,
    HSNAsic,
    RouterFpga,
    RouterBMC,
    HSNLink,
    HSNConnector,
    INVALID,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorId {
    #[serde(rename = "EffectiveFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_family: Option<String>,
    #[serde(rename = "EffectiveModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efffective_model: Option<String>,
    #[serde(rename = "IdentificationRegisters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_registers: Option<String>,
    #[serde(rename = "MicrocodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microcode_info: Option<String>,
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUProcessor {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "ProcessorFRUInfo")]
    pub processor_fru_info: RedfishProcessorFRUInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUMemory {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "MemoryFRUInfo")]
    pub memory_fru_info: RedfishMemoryFRUInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUHSNNIC {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "HSNNICFRUInfo")]
    pub hsn_nic_fru_info: HSNNICFRUInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUNodeAccel {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "NodeAccelFRUInfo")]
    pub node_accel_fru_info: RedfishProcessorFRUInfo, // NOTE: yes, sounds weird FRU for node accelerator uses FRU for processor... but that is
                                                      // what the API docs says...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishProcessorFRUInfo {
    #[serde(rename = "InstructionSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_set: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "MaxSpeedMHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed_mhz: Option<usize>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "ProcessorArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_architecture: Option<String>,
    #[serde(rename = "ProcessorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_id: Option<ProcessorId>,
    #[serde(rename = "ProcessorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_type: Option<String>,
    #[serde(rename = "TotalCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cores: Option<usize>,
    #[serde(rename = "TotalThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_threads: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishMemoryFRUInfo {
    #[serde(rename = "BaseModuleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_module_type: Option<String>,
    #[serde(rename = "BusWidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus_width_bits: Option<usize>,
    #[serde(rename = "CapacityMiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_mib: Option<usize>,
    #[serde(rename = "DataWidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_width_bits: Option<usize>,
    #[serde(rename = "ErrorCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_correction: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "MemoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_type: Option<String>,
    #[serde(rename = "MemoryDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_device_type: Option<String>,
    #[serde(rename = "OperatingSpeedMhz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_speed_mhz: Option<usize>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "RankCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_count: Option<usize>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByFRU {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishChassisLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocChassis {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "ChassisLocatinInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "ComputeModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<HWInvByLocComputeModule>,
    #[serde(rename = "RouterModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<HWInvByLocRouterModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeEnclosure {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeEnclosureLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocComputeModule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "ComputeModuleLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_module_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "NodeEnclosures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<HWInvByLocNodeEnclosure>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNBoard {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "HSNBoardLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_board_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterModule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "RouterModuleLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_module_location_info: Option<RedfishChassisLocationInfo>,
    pub hsn_boards: Option<HWInvByLocHSNBoard>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCabinet {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CabinetLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_location_info: Option<RedfishChassisLocationInfo>,
    #[serde(rename = "Chassis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<HWInvByLocChassis>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "MgmtSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMgmtHLSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "MgmtHLSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCDUMgmtSwitch {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CDUMgmtSwitchLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switch_location_info: Option<RedfishChassisLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorSummary {
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemorySummary {
    #[serde(rename = "TotalSystemMemoryGiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_system_memory_gib: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishSystemLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "ProcessorSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_summary: Option<ProcessorSummary>,
    #[serde(rename = "MemorySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_summary: Option<MemorySummary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishProcessorLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Socket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocProcessor {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUProcessor>,
    #[serde(rename = "ProcessorLocationInfo")]
    pub processor_location_info: RedfishProcessorLocationInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccel {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUNodeAccel>,
    #[serde(rename = "NodeAccelLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_location_info: Option<RedfishProcessorLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishDriveLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocDrive {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "DriveLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_location_info: Option<RedfishDriveLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryLocation {
    #[serde(rename = "Socket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<u32>,
    #[serde(rename = "MemoryController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_controller: Option<u32>,
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u32>,
    #[serde(rename = "Slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishMemoryLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MemoryLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_location: Option<MemoryLocation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocMemory {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUMemory>,
    #[serde(rename = "MemoryLocationInfo")]
    pub memory_location_info: RedfishMemoryLocationInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeAccelRiserLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeAccelRiser {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeAccelRiserLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_riser_location_info: Option<RedfishNodeAccelRiserLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HSNNICLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocHSNNIC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUHSNNIC>,
    /* #[serde(rename = "NodeHsnNicLocationInfo")]
    pub node_hsn_nic_location_info: HSNNICLocationInfo, */
    #[serde(rename = "HSNNICLocationInfo")]
    pub hsn_nic_location_info: HSNNICLocationInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishSystemFRUInfo {
    #[serde(rename = "AssetTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(rename = "BiosVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "SystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_type: Option<String>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByFRUNode {
    #[serde(rename = "FRUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "FRUSubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fru_sub_type: Option<String>,
    #[serde(rename = "HWInventoryByFRUType")]
    pub hw_inventory_by_fru_type: String,
    #[serde(rename = "NodeFRUInfo")]
    pub node_fru_info: RedfishSystemFRUInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HSNNICFRUInfo {
    #[serde(rename = "Manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "SKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInvByFRUNode>,
    #[serde(rename = "NodeLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_location_info: Option<RedfishSystemLocationInfo>,
    #[serde(rename = "Processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<HWInvByLocProcessor>>,
    #[serde(rename = "NodeAccels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<Vec<HWInvByLocNodeAccel>>,
    #[serde(rename = "Dives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<HWInvByLocDrive>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<HWInvByLocMemory>>,
    #[serde(rename = "NodeAccelRisers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<Vec<HWInvByLocNodeAccelRiser>>,
    #[serde(rename = "NodeHsnNICs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<Vec<HWInvByLocHSNNIC>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishPDULocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishOutletLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocOutlet {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "OutletLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet_location_info: Option<RedfishOutletLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocPDU {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "PDULocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdu_location_info: Option<RedfishPDULocationInfo>,
    #[serde(rename = "CabinetPDUPowerConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<Vec<HWInvByLocOutlet>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishCMMRectifierLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocCMMRectifier {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "CMMRectifierLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmm_rectifier_location_info: Option<RedfishCMMRectifierLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishNodeEnclosurePowerSupplyLocationInfo {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodePowerSupply {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeEnclosurePowerSupplyLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supply_location_info:
        Option<RedfishNodeEnclosurePowerSupplyLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishManagerLocationInfo {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "DateTimeLocalOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_local_offset: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocNodeBMC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "NodeBMCLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInvByLocRouterBMC {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Ordinal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HWInventoryByLocationType")]
    pub hw_inventory_by_location_type: String,
    #[serde(rename = "PopulatedFRU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populated_fru: Option<HWInventoryByFRU>,
    #[serde(rename = "RouterBMCLocationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc_location_info: Option<RedfishManagerLocationInfo>,
}

/* #[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryList {
    #[serde(rename = "Hardware")]
    pub hw_inventory: Vec<HWInventory>,
} */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventory {
    #[serde(rename = "XName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xname: Option<String>,
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Cabinets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinets: Option<Vec<HWInvByLocCabinet>>,
    #[serde(rename = "Chassis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<Vec<HWInvByLocChassis>>,
    #[serde(rename = "ComputeModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_modules: Option<Vec<HWInvByLocComputeModule>>,
    #[serde(rename = "RouterModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_modules: Option<Vec<HWInvByLocRouterModule>>,
    #[serde(rename = "NodeEnclosures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosures: Option<Vec<HWInvByLocNodeEnclosure>>,
    #[serde(rename = "HSNBoards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_boards: Option<Vec<HWInvByLocHSNBoard>>,
    #[serde(rename = "MgmtSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_switches: Option<Vec<HWInvByLocMgmtSwitch>>,
    #[serde(rename = "MgmtHLSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgmt_hl_switches: Option<Vec<HWInvByLocMgmtHLSwitch>>,
    #[serde(rename = "CDUMgmtSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdu_mgmt_switches: Option<Vec<HWInvByLocCDUMgmtSwitch>>,
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<HWInvByLocNode>>,
    #[serde(rename = "Processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<HWInvByLocProcessor>>,
    #[serde(rename = "NodeAccels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accels: Option<Vec<HWInvByLocNodeAccel>>,
    #[serde(rename = "Drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<HWInvByLocDrive>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<HWInvByLocMemory>>,
    #[serde(rename = "CabinetPDUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdus: Option<Vec<HWInvByLocPDU>>,
    #[serde(rename = "CabinetPDUPowerConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cabinet_pdu_power_connectors: Option<Vec<HWInvByLocOutlet>>,
    #[serde(rename = "CMMRectifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmm_rectifiers: Option<Vec<HWInvByLocCMMRectifier>>,
    #[serde(rename = "NodeAccelRisers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_accel_risers: Option<Vec<HWInvByLocNodeAccelRiser>>,
    #[serde(rename = "NodeHsnNICs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_hsn_nics: Option<Vec<HWInvByLocHSNNIC>>,
    #[serde(rename = "NodeEnclosurePowerSupplies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_enclosure_power_supplies: Option<Vec<HWInvByLocNodePowerSupply>>,
    #[serde(rename = "NodeBMC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bmc: Option<Vec<HWInvByLocNodeBMC>>,
    #[serde(rename = "RouterBMC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_bmc: Option<Vec<HWInvByLocRouterBMC>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hardware {
    #[serde(rename = "Hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<Vec<HWInvByLocNode>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeLocationInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "ProcessorSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_summary: Option<ProcessorSummary>,
    #[serde(rename = "MemorySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_summary: Option<MemorySummary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // <-- this is important. More info https://serde.rs/enum-representations.html#untagged
pub enum HWInventoryByLocation {
    /* HWInvByLocCabinet(HWInvByLocCabinet),
    HWInvByLocChassis(HWInvByLocChassis),
    HWInvByLocComputeModule(HWInvByLocComputeModule),
    HWInvByLocRouterModule(HWInvByLocRouterModule),
    HWInvByLocNodeEnclosure(HWInvByLocNodeEnclosure),
    HWInvByLocHSNBoard(HWInvByLocHSNBoard),
    HWInvByLocMgmtSwitch(HWInvByLocMgmtSwitch),
    HWInvByLocMgmtHLSwitch(HWInvByLocMgmtHLSwitch),
    HWInvByLocCDUMgmtSwitch(HWInvByLocCDUMgmtSwitch), */
    HWInvByLocNode(HWInvByLocNode),
    HWInvByLocProcessor(HWInvByLocProcessor),
    HWInvByLocNodeAccel(HWInvByLocNodeAccel),
    /*     HWInvByLocDrive(HWInvByLocDrive), */
    HWInvByLocMemory(HWInvByLocMemory),
    /* HWInvByLocPDU(HWInvByLocPDU),
    HWInvByLocOutlet(HWInvByLocOutlet),
    HWInvByLocCMMRectifier(HWInvByLocCMMRectifier),
    HWInvByLocNodeAccelRiser(HWInvByLocNodeAccelRiser), */
    HWInvByLocHSNNIC(HWInvByLocHSNNIC),
    /* HWInvByLocNodePowerSupply(HWInvByLocNodePowerSupply),
    HWInvByLocNodeBMC(HWInvByLocNodeBMC),
    HWInvByLocRouterBMC(HWInvByLocRouterBMC), */
}

/// struct used in POST and GET endpoints that manage multiple instances of 'HWInventoryByLocation'
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HWInventoryByLocationList {
    #[serde(rename = "Hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<Vec<HWInventoryByLocation>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cfs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BootSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfs: Option<Cfs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_parameters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_roles_groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>, // TODO: use Arch enum instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs_provider_passthrough: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BosSessionTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cfs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfs: Option<Cfs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_sets: Option<HashMap<String, BootSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}
