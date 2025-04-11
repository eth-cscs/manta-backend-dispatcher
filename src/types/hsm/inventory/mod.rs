use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryInfo {
    #[serde(rename(serialize = "LastAttempt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt: Option<String>,
    #[serde(rename(serialize = "LastStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename(serialize = "RedfishVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redfish_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpoint {
    #[serde(rename(serialize = "ID"))]
    pub id: String,
    #[serde(rename(serialize = "Type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(serialize = "Name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename(serialize = "Hostname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename(serialize = "Domain"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename(serialize = "FQDN"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename(serialize = "Enabled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename(serialize = "UUID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename(serialize = "User"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename(serialize = "Password"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename(serialize = "UseSSDP"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ssdp: Option<bool>,
    #[serde(rename(serialize = "MacRequired"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_required: Option<bool>,
    #[serde(rename(serialize = "MACAddr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addr: Option<String>,
    #[serde(rename(serialize = "IPAddress"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename(serialize = "RediscoverOnUpdate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rediscover_on_update: Option<bool>,
    #[serde(rename(serialize = "TemplateID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename(serialize = "DiscoveryInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_info: Option<DiscoveryInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpointArray {
    #[serde(rename(serialize = "RedfishEndpoints"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redfish_endpoints: Option<Vec<RedfishEndpoint>>,
}
