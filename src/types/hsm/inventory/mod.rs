use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryInfo {
  #[serde(rename = "LastAttempt")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_attempt: Option<String>,
  #[serde(rename = "LastStatus")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_status: Option<String>,
  #[serde(rename = "RedfishVersion")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redfish_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpoint {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "Type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(rename = "Name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(rename = "Hostname")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hostname: Option<String>,
  #[serde(rename = "Domain")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
  #[serde(rename = "FQDN")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fqdn: Option<String>,
  #[serde(rename = "Enabled")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(rename = "UUID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uuid: Option<String>,
  #[serde(rename = "User")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
  #[serde(rename = "Password")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  #[serde(rename = "UseSSDP")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_ssdp: Option<bool>,
  #[serde(rename = "MacRequired")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_required: Option<bool>,
  #[serde(rename = "MACAddr")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_addr: Option<String>,
  #[serde(rename = "IPAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_address: Option<String>,
  #[serde(rename = "RediscoverOnUpdate")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rediscover_on_update: Option<bool>,
  #[serde(rename = "TemplateID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template_id: Option<String>,
  #[serde(rename = "DiscoveryInfo")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub discovery_info: Option<DiscoveryInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedfishEndpointArray {
  #[serde(rename = "RedfishEndpoints")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redfish_endpoints: Option<Vec<RedfishEndpoint>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct IpAddressMapping {
  #[serde(rename = "IPAddress")]
  pub ip_address: String,
  #[serde(rename = "Network")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub network: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentEthernetInterface {
  #[serde(rename = "ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "Description")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(rename = "MACAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_address: Option<String>,
  #[serde(rename = "IPAddresses")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_addresses: Option<Vec<IpAddressMapping>>,
  #[serde(rename = "LastUpdate")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_update: Option<String>,
  #[serde(rename = "ComponentID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub component_id: Option<String>,
  #[serde(rename = "Type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_hms_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentEthernetInterfaceArray {
  #[serde(rename = "EthernetInterfaces")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ethernet_interfaces: Option<Vec<ComponentEthernetInterface>>,
}
