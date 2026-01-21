use crate::{error::Error, types::hsm::inventory::ComponentEthernetInterface};
use serde_json::Value;
use std::future::Future;

pub trait ComponentEthernetInterfaceTrait {
  fn get_all_component_ethernet_interfaces(
    &self,
    _auth_token: &str,
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
    _auth_token: &str,
    _eth_interface_id: &str,
  ) -> impl Future<Output = Result<ComponentEthernetInterface, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn add_component_ethernet_interface(
    &self,
    _auth_token: &str,
    _component_ethernet_interface: &ComponentEthernetInterface,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Add component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn update_component_ethernet_interface(
    &self,
    _auth_token: &str,
    _eth_interface_id: &str,
    _description: Option<&str>,
    _ip_address_mapping: (&str, &str),
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
    _auth_token: &str,
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
    _auth_token: &str,
    _eth_interface_id: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Delete component ethernet interface command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  /* fn get_ip_addresses(
    &self,
    _auth_token: &str,
    _eth_interface_id: &str,
  ) -> impl Future<Output = Result<Vec<IpAddressMapping>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get IP addresses command not implemented for this backend".to_string(),
      ))
    }
  }

  fn delete_ip_address(
    &self,
    _auth_token: &str,
    _group_label: &str,
    _eth_interface_id: &str,
    _ip_address: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send {
    async {
      Err(Error::Message(
        "Delete IP address command not implemented for this backend"
          .to_string(),
      ))
    }
  } */
}
