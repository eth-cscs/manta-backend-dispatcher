use std::future::Future;
use serde_json::Value;
use crate::{
  error::Error,
  types::hsm::inventory::{ 
      IpAddressMapping,
      ComponentEthernetInterface, 
      ComponentEthernetInterfaceArray,
  },
};

pub trait ComponentEthernetInterfaceTrait{
  fn get_all_component_ethernet_interfaces(
    &self,
    auth_token: &str,
    mac_address: &str,
    ip_address: &str,
    network: &str,
    component_id: &str, // Node's xname
    r#type: &str,
    older_than: &str,
    newer_than: &str,
  ) -> impl Future<Output = Result<ComponentEthernetInterfaceArray, Error>> + Send;

  fn get_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> impl Future<Output = Result<ComponentEthernetInterface, Error>> + Send;

//  fn add_component_ethernet_interface(
//    &self,
//    auth_token: &str,
//    component_ethernet_interface: &ComponentEthernetInterfaceArray,
//  ) -> impl Future<Output = Result<(), Error>> + Send;

  fn update_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
    description: Option<&str>,
    ip_address_mapping: (&str, &str), 
  ) -> impl Future<Output = Result<Value, Error>> + Send;

  fn delete_all_component_ethernet_interfaces(
    &self,
    auth_token: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send;

  fn delete_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send;


  fn get_ip_addresses(
      &self,
      auth_token: &str,
      eth_interface_id: &str,
  ) -> impl Future<Output = Result<Vec<IpAddressMapping>, Error>> + Send;

  fn delete_ip_address(
    &self,
    auth_token: &str,
    //base_url: &str,
    //root_cert: &[u8],
    _group_label: &str,
    eth_interface_id: &str,
    ip_address: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send;
}
