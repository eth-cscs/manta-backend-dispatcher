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

use std::{collections::HashMap, future::Future};

use crate::{
  error::Error,
  types::{
    bos::{session::BosSession, session_template::BosSessionTemplate},
    cfs::{
      cfs_configuration_response::CfsConfigurationResponse,
      session::CfsSessionGetResponse,
    },
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

/// Parameters for [`SatTrait::apply_configuration`].
pub struct ApplyConfigurationParams<'a> {
  pub shasta_token: &'a str,
  pub vault_base_url: &'a str,
  pub site_name: &'a str,
  pub k8s_api_url: &'a str,
  pub gitea_base_url: &'a str,
  pub gitea_token: &'a str,
  /// One SAT `configurations[]` entry as a structured value.
  pub configuration: serde_json::Value,
  pub dry_run: bool,
  pub overwrite: bool,
}

/// Parameters for [`SatTrait::apply_image`].
pub struct ApplyImageParams<'a> {
  pub shasta_token: &'a str,
  pub vault_base_url: &'a str,
  pub site_name: &'a str,
  pub k8s_api_url: &'a str,
  /// One SAT `images[]` entry as a structured value.
  pub image: serde_json::Value,
  /// `ref_name.or(name) -> image_id` for previously-created images. The
  /// CLI accumulates this across image and session_template calls;
  /// csm-rs's per-image creator consumes it to resolve
  /// `base.image_ref` chains.
  pub ref_lookup: HashMap<String, String>,
  pub hsm_group_available_vec: &'a [String],
  pub ansible_verbosity: Option<u8>,
  pub ansible_passthrough: Option<&'a str>,
  pub debug_on_failure: bool,
  pub watch_logs: bool,
  pub timestamps: bool,
  pub dry_run: bool,
}

/// Parameters for [`SatTrait::apply_sat_image_create_session`].
///
/// Splits the "create CFS session that builds the image" half of
/// [`SatTrait::apply_image`] out into a discrete call so the CLI can
/// drive the monitor + stamp steps itself.
///
/// No `hsm_group_available_vec` here — unlike `ApplyImageParams`,
/// HSM-group access is enforced by the caller (manta-server's
/// `post_sat_image_cfs_session` handler) before this call, via
/// [`service::group::validate_hsm_group_access_many`] over the SAT
/// entry's `configuration_group_names`. Backends are not expected to
/// reproduce the check.
pub struct ApplyImageCreateSessionParams<'a> {
  pub shasta_token: &'a str,
  pub vault_base_url: &'a str,
  pub site_name: &'a str,
  pub k8s_api_url: &'a str,
  /// One SAT `images[]` entry as a structured value.
  pub image: serde_json::Value,
  /// `ref_name.or(name) -> image_id` for previously-created images.
  pub ref_lookup: HashMap<String, String>,
  pub ansible_verbosity: Option<u8>,
  pub ansible_passthrough: Option<&'a str>,
  pub dry_run: bool,
}

/// Parameters for [`SatTrait::apply_sat_image_stamp_from_session`].
///
/// The other half of the split [`SatTrait::apply_image`] flow: given a
/// terminal-complete CFS session, derive `manta.image_session.*` and
/// PATCH it onto the resulting IMS image.
pub struct ApplyImageStampParams<'a> {
  pub shasta_token: &'a str,
  /// Name of the CFS session whose result image should be stamped.
  pub cfs_session_name: &'a str,
}

/// Parameters for [`SatTrait::apply_session_template`].
pub struct ApplySessionTemplateParams<'a> {
  pub shasta_token: &'a str,
  /// One SAT `session_templates[]` entry as a structured value.
  pub session_template: serde_json::Value,
  /// `ref_name.or(name) -> image_id` for previously-created images.
  /// Used to resolve `session_template.image.image_ref`.
  pub ref_lookup: HashMap<String, String>,
  pub hsm_group_available_vec: &'a [String],
  /// Trigger a BOS session to reboot the targeted nodes after the
  /// template is created.
  pub reboot: bool,
  pub dry_run: bool,
}

/// Parameters for [`SatTrait::validate_sat_file`].
///
/// Subset of [`ApplySatFileParams`] — only the fields the validator
/// reads. The SAT content travels as a structured
/// `serde_json::Value` end-to-end, exactly like
/// [`ApplySatFileParams::sat_file`].
pub struct ValidateSatFileParams<'a> {
  /// Bearer token authenticating the caller against the backend
  /// (and against Vault when the backend fetches k8s creds).
  pub shasta_token: &'a str,
  /// Vault base URL — used to fetch the k8s secrets needed to read
  /// the `cray-product-catalog` ConfigMap.
  pub vault_base_url: &'a str,
  /// Site identifier used to namespace the Vault path
  /// (`manta/data/<site_name>/k8s`).
  pub site_name: &'a str,
  /// Kubernetes API URL — used to talk to the in-cluster product
  /// catalog after the k8s creds have been fetched from Vault.
  pub k8s_api_url: &'a str,
  /// Parsed SAT file as a structured value (identical shape to
  /// [`ApplySatFileParams::sat_file`]).
  pub sat_file: serde_json::Value,
  /// HSM groups the caller is allowed to target; SAT files
  /// referencing groups outside this slice are rejected.
  pub hsm_group_available_vec: &'a [String],
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

  /// Validate a SAT file against the backend's live state without
  /// mutating anything.
  ///
  /// Pre-flight check: returns `Ok(())` if the SAT file would
  /// apply cleanly given the current state, or `Err(...)` with the
  /// first detected mismatch (fail-fast). See the
  /// `2026-06-15-sat-file-validate-endpoint` design doc for the
  /// shape of the validator's checks.
  ///
  /// The default implementation returns
  /// [`Error::Message`](crate::error::Error::Message) so backends
  /// that don't support SAT validation can be plugged in without
  /// implementing the method.
  fn validate_sat_file(
    &self,
    _params: ValidateSatFileParams<'_>,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Validate SAT file command not implemented for this backend".to_string(),
      ))
    }
  }

  /// Apply a single SAT `configurations[]` entry.
  ///
  /// The backend validates the entry against live CSM state, converts
  /// the SAT shape into a `CfsConfigurationRequest` (resolving any
  /// `product:` layers via the cray-product-catalog ConfigMap and any
  /// `git:` branches via Gitea), POSTs it to CFS, and returns the
  /// created configuration (or, in `dry_run`, a mock response with the
  /// configuration's name).
  fn apply_configuration(
    &self,
    _params: ApplyConfigurationParams<'_>,
  ) -> impl Future<Output = Result<CfsConfigurationResponse, Error>> + Send {
    async {
      Err(Error::Message(
        "Apply configuration command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  /// Apply a single SAT `images[]` entry in one synchronous call.
  ///
  /// The backend validates the entry, resolves any `base.image_ref`
  /// against `params.ref_lookup`, constructs the CFS session that
  /// builds the image, runs it (unless `dry_run`), and returns the
  /// created IMS image. Backends MAY stamp provenance metadata
  /// (`manta.image_session.*`) onto the image as part of this call —
  /// csm-rs does this for SAT-built images so the produced image is
  /// self-describing — but this is an implementation detail and not
  /// part of the trait contract.
  ///
  /// Callers that want to observe the image build can instead drive
  /// the two split methods themselves:
  /// [`SatTrait::apply_sat_image_create_session`] kicks off the CFS
  /// session, and [`SatTrait::apply_sat_image_stamp_from_session`]
  /// fetches the now-complete session and stamps the produced IMS
  /// image. Both paths share the same backend internals.
  fn apply_image(
    &self,
    _params: ApplyImageParams<'_>,
  ) -> impl Future<Output = Result<Image, Error>> + Send {
    async {
      Err(Error::Message(
        "Apply image command not implemented for this backend".to_string(),
      ))
    }
  }

  /// Translate one SAT `images[]` entry into a CFS session payload and
  /// create the session. The returned [`CfsSessionGetResponse`] is the
  /// freshly-created session (initial status, no `result_id` yet);
  /// callers drive it to completion themselves (via the session-status
  /// or session-logs endpoints) and then call
  /// [`SatTrait::apply_sat_image_stamp_from_session`] to stamp the
  /// produced IMS image with `manta.image_session.*` provenance.
  ///
  /// This is the first half of the work [`SatTrait::apply_image`] does
  /// monolithically; exposing it on its own lets the manta-cli own the
  /// monitor + stamp orchestration while the backend stays the source
  /// of truth for the SAT-image → CFS-session translation.
  fn apply_sat_image_create_session(
    &self,
    _params: ApplyImageCreateSessionParams<'_>,
  ) -> impl Future<Output = Result<CfsSessionGetResponse, Error>> + Send {
    async {
      Err(Error::Message(
        "Apply SAT image (create-session) command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  /// Fetch the (terminal-complete) CFS session named by `params`,
  /// derive `manta.image_session.{base,groups,configuration}` from it,
  /// and PATCH them onto the IMS image the session produced.
  ///
  /// Errors with [`Error::Message`] if the session is not complete or
  /// has no `result_id` — i.e. produced no image — so callers can fail
  /// fast without attempting a PATCH against a non-existent image.
  ///
  /// This is the second half of [`SatTrait::apply_image`]'s work,
  /// exposed separately so the manta-cli can decide *when* to stamp
  /// (e.g. only after its own polling/log-streaming has observed the
  /// session reach a terminal state).
  fn apply_sat_image_stamp_from_session(
    &self,
    _params: ApplyImageStampParams<'_>,
  ) -> impl Future<Output = Result<Image, Error>> + Send {
    async {
      Err(Error::Message(
        "Apply SAT image (stamp-from-session) command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  /// Apply a single SAT `session_templates[]` entry.
  ///
  /// The backend validates the entry, resolves `image.image_ref`
  /// against `params.ref_lookup`, creates the BOS session template,
  /// and (if `params.reboot`) creates a BOS session to reboot the
  /// targeted nodes through it.
  fn apply_session_template(
    &self,
    _params: ApplySessionTemplateParams<'_>,
  ) -> impl Future<
    Output = Result<(BosSessionTemplate, Option<BosSession>), Error>,
  > + Send {
    async {
      Err(Error::Message(
        "Apply session template command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
