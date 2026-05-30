use crate::error::Error;
use crate::types::pcs::power_status::types::PowerStatusAll;
use crate::types::pcs::transitions::types::{
  TransitionResponse, TransitionStartOutput,
};

pub trait PCSTrait {
  /// Start a power transition on the given nodes and return immediately
  /// with the PCS transition id (no polling). The CLI is responsible
  /// for tracking completion via [`Self::pcs_transitions_get`].
  ///
  /// `operation` is the PCS wire-level operation string (`"on"`,
  /// `"soft-off"`, `"force-off"`, `"soft-restart"`, `"hard-restart"`)
  /// — the mapping from the CLI's `(PowerAction, force: bool)` pair to
  /// these strings lives in the backend impl.
  fn pcs_transitions_post(
    &self,
    _auth_token: &str,
    _operation: &str,
    _nodes: &[String],
  ) -> impl std::future::Future<Output = Result<TransitionStartOutput, Error>>
  + Send {
    async {
      Err(Error::Message(
        "pcs_transitions_post not implemented for this backend".to_string(),
      ))
    }
  }

  /// Fetch a single power-transition snapshot by id. Used by the CLI
  /// poll loop after `pcs_transitions_post` returns the id.
  fn pcs_transitions_get(
    &self,
    _auth_token: &str,
    _transition_id: &str,
  ) -> impl std::future::Future<Output = Result<TransitionResponse, Error>> + Send
  {
    async {
      Err(Error::Message(
        "pcs_transitions_get not implemented for this backend".to_string(),
      ))
    }
  }

  // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
  fn power_status(
    &self,
    _auth_token: &str,
    //_nodes: &[String],
    _nodes: &[String],
    _power_status_filter: Option<&str>,
    _management_state_filter: Option<&str>,
  ) -> impl std::future::Future<Output = Result<PowerStatusAll, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Power status command not implemented for this backend".to_string(),
      ))
    }
  }
}
