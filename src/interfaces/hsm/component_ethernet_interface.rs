use crate::{
  error::Error,
  types::hsm::inventory::{
    ComponentEthernetInterface, ComponentEthernetInterfaceArray,
    IpAddressMapping,
  },
};
use serde_json::Value;
use std::future::Future;

pub trait ComponentEthernetInterfaceTrait {
  fn get_all_component_ethernet_interfaces(
    &self,
    auth_token: &str,
  ) -> impl Future<Output = Result<Vec<ComponentEthernetInterface>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get all component ethernet interfaces command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> impl Future<Output = Result<ComponentEthernetInterface, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

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
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Update component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn delete_all_component_ethernet_interfaces(
    &self,
    auth_token: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Delete all component ethernet interfaces command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn delete_component_ethernet_interface(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Delete component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_ip_addresses(
    &self,
    auth_token: &str,
    eth_interface_id: &str,
  ) -> impl Future<Output = Result<Vec<IpAddressMapping>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get IP addresses command not implemented for this backend".to_string(),
      ))
    }
  }

  fn delete_ip_address(
    &self,
    auth_token: &str,
    //base_url: &str,
    //root_cert: &[u8],
    _group_label: &str,
    eth_interface_id: &str,
    ip_address: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Delete IP address command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
