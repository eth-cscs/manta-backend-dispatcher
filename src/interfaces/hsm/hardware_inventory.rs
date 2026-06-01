use crate::{
  error::Error,
  types::{
    HWInventory, HWInventoryByLocationList, HsmActionResponse, NodeSummary,
  },
};

pub trait HardwareInventory {
  /// Submit a hardware-inventory write to HSM (e.g. discovery agents
  /// publishing new components). Returns the typed
  /// [`HsmActionResponse`] ack from CSM (`code` + `message`,
  /// per swagger `Response_1.0.0`).
  fn post_inventory_hardware(
    &self,
    auth_token: &str,
    hardware: HWInventoryByLocationList,
  ) -> impl std::future::Future<Output = Result<HsmActionResponse, Error>> + Send;

  /// Fetch a single node's hardware inventory as a typed summary.
  ///
  /// Returns the structured [`NodeSummary`] derived from the HSM
  /// inventory entry at `xname` — processors, memory modules, node
  /// accelerators, and HSN NICs broken out as
  /// [`ArtifactSummary`](crate::types::ArtifactSummary) lists.
  fn get_inventory_hardware(
    &self,
    auth_token: &str,
    xname: &str,
  ) -> impl std::future::Future<Output = Result<NodeSummary, Error>> + Send;

  /// Run an `/Inventory/Hardware/Query/{xname}` request and return the
  /// hierarchical [`HWInventory`] response (cabinets → chassis →
  /// compute modules → nodes, etc., depending on the `format` and
  /// children/parents flags).
  fn get_inventory_hardware_query(
    &self,
    auth_token: &str,
    xname: &str,
    r#type: Option<&str>,
    children: Option<bool>,
    parents: Option<bool>,
    partition: Option<&str>,
    format: Option<&str>,
  ) -> impl std::future::Future<Output = Result<HWInventory, Error>> + Send;
}
