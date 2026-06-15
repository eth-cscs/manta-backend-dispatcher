use std::future::Future;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc::Sender;

use crate::{error::Error, types::K8sDetails};

/// Terminal dimensions, used both as initial-size input to
/// [`ConsoleTrait::attach_to_node_console`] / [`ConsoleTrait::attach_to_session_console`]
/// and as the message type carried by [`ConsoleAttachment::resize`].
///
/// Mirrors the kube-rs `TerminalSize` shape so the CSM backend can
/// forward incoming events to the kube exec subprotocol's
/// resize channel without re-derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermSize {
  pub width: u16,
  pub height: u16,
}

/// Bidirectional handle to an attached interactive console.
///
/// - `stdin` / `stdout` carry the byte streams of the underlying PTY.
/// - `resize` is the control channel the caller pushes new
///   [`TermSize`] values into when the client terminal is resized.
///   Implementations forward each received value to the underlying
///   transport's resize channel (e.g. k8s exec subprotocol channel 4
///   for CSM / OpenCHAMI). Dropping the sender ends resize forwarding
///   but does not close the console; the console ends when `stdin`
///   or `stdout` does.
pub struct ConsoleAttachment {
  pub stdin: Box<dyn AsyncWrite + Unpin + Send>,
  pub stdout: Box<dyn AsyncRead + Unpin + Send>,
  pub resize: Sender<TermSize>,
}

/// Attach an interactive console to a node or CFS session pod.
///
/// Returns a [`ConsoleAttachment`] carrying the stdin/stdout streams
/// plus a resize sender. The initial terminal size is supplied as
/// `initial_size`; subsequent resizes are sent through
/// [`ConsoleAttachment::resize`].
pub trait ConsoleTrait {
  fn attach_to_node_console(
    &self,
    _shasta_token: &str,
    _site_name: &str,
    _xname: &str,
    _initial_size: TermSize,
    _k8s: &K8sDetails,
  ) -> impl Future<Output = Result<ConsoleAttachment, Error>> + Send {
    async {
      Err(Error::Message(
        "Attach to node console command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn attach_to_session_console(
    &self,
    _shasta_token: &str,
    _site_name: &str,
    _session_name: &str,
    _initial_size: TermSize,
    _k8s: &K8sDetails,
  ) -> impl Future<Output = Result<ConsoleAttachment, Error>> + Send {
    async {
      Err(Error::Message(
        "Attach to session console command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
