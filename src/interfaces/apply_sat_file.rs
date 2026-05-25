//! Backend interface for applying a pre-rendered SAT (System Admin
//! Toolkit) file.
//!
//! The trait method takes a single [`ApplySatFileParams`] struct so that
//! adding or renaming a parameter is a one-field change instead of a
//! signature edit across every impl. Backends are responsible for
//! fetching their own Kubernetes credentials from Vault — callers pass
//! `vault_base_url`, `site_name`, and `shasta_token`, and the backend
//! does the lookup if it needs `cray-product-catalog` or similar.
//!
//! Returns the four lists of artifacts the implementation produced —
//! CFS configurations, IMS images, BOS session templates, and BOS
//! sessions. In `dry_run` mode the same tuple is returned with the
//! artifacts that *would* be created.

use std::future::Future;

use crate::{
  error::Error,
  types::{
    bos::{session::BosSession, session_template::BosSessionTemplate},
    cfs::cfs_configuration_response::CfsConfigurationResponse,
    ims::Image,
  },
};

/// Parameters for [`SatTrait::apply_sat_file`].
///
/// Borrows where it can; owns `sat_file` so the backend can consume it
/// without a clone. The SAT content travels as a structured
/// `serde_json::Value` end-to-end — the CLI parses YAML once, the
/// server forwards the value verbatim, and the backend transcodes to
/// its preferred shape internally.
pub struct ApplySatFileParams<'a> {
  /// Bearer token authenticating the caller against the backend (and
  /// against Vault, when the backend fetches its own K8s creds).
  pub shasta_token: &'a str,
  /// Backend (CSM / OCHAMI) API base URL.
  pub shasta_base_url: &'a str,
  /// PEM-encoded root certificate trusted for the backend's HTTPS API.
  pub shasta_root_cert: &'a [u8],
  /// Optional SOCKS5 proxy URL applied to every HTTP call the backend
  /// makes.
  pub socks5_proxy: Option<&'a str>,
  /// Vault base URL. The backend uses this to fetch the Kubernetes
  /// secrets it needs to read the `cray-product-catalog` ConfigMap.
  pub vault_base_url: &'a str,
  /// Site identifier used to namespace the Vault path
  /// (`manta/data/<site_name>/k8s`).
  pub site_name: &'a str,
  /// Kubernetes API URL — used to talk to the in-cluster product
  /// catalog after the K8s creds have been fetched from Vault.
  pub k8s_api_url: &'a str,
  /// Parsed SAT file as a structured value. The CLI parses the YAML
  /// source once and forwards this verbatim through the server; the
  /// backend transcodes internally if it needs a different shape.
  pub sat_file: serde_json::Value,
  /// HSM groups the caller is allowed to target; SAT files referencing
  /// groups outside this slice are rejected.
  pub hsm_group_available_vec: &'a [String],
  /// Ansible verbosity level (0-4) for any CFS sessions created.
  pub ansible_verbosity: Option<u8>,
  /// Extra arguments forwarded verbatim to `ansible-playbook`.
  pub ansible_passthrough: Option<&'a str>,
  /// Gitea base URL (used for VCS layer URL rewrites).
  pub gitea_base_url: &'a str,
  /// Bearer token authenticating against Gitea.
  pub gitea_token: &'a str,
  /// Reboot affected nodes after the session templates are created.
  pub reboot: bool,
  /// Stream CFS session logs while the apply runs.
  pub watch_logs: bool,
  /// Prefix each streamed log line with its timestamp.
  pub timestamps: bool,
  /// Drop the operator into a debug shell if a CFS session fails.
  pub debug_on_failure: bool,
  /// Overwrite existing CFS configurations / IMS images on conflict
  /// instead of erroring.
  pub overwrite: bool,
  /// Validate the SAT file without mutating CSM state. The same return
  /// tuple is built with the artifacts that would have been created.
  pub dry_run: bool,
}

pub trait SatTrait {
  /// Apply a pre-rendered SAT file.
  ///
  /// See [`ApplySatFileParams`] for argument semantics. On success
  /// returns `(configurations, images, session_templates, sessions)` —
  /// the artifacts the backend created (or, when `params.dry_run` is
  /// true, the artifacts it would have created). Empty vectors are
  /// returned for sections absent from `params.sat_template_file_yaml`;
  /// in particular `sessions` is empty unless `params.reboot` is true.
  ///
  /// The default implementation returns
  /// [`Error::Message`](crate::error::Error::Message) so backends that
  /// don't support SAT-file apply can be plugged in without
  /// implementing the method.
  fn apply_sat_file(
    &self,
    _params: ApplySatFileParams<'_>,
  ) -> impl Future<
    Output = Result<
      (
        Vec<CfsConfigurationResponse>,
        Vec<Image>,
        Vec<BosSessionTemplate>,
        Vec<BosSession>,
      ),
      Error,
    >,
  > + Send {
    async {
      Err(Error::Message(
        "Apply SAT file command not implemented for this backend".to_string(),
      ))
    }
  }
}
