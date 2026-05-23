//! Thin wrapper around the control wire client.
//!
//! Resolvers open one of these per request (or per subscription stream) and
//! invoke RPCs by name. Connection pooling / multiplexing can be added later
//! once the protocol crate exposes a long-lived handle.

use std::path::Path;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::Stream;

/// Stub control client. The real wire client comes from
/// `animus-control-protocol` once its v0.1.5 surface lands.
pub struct ControlClient {
    socket_path: std::path::PathBuf,
}

impl ControlClient {
    pub async fn connect(socket_path: &Path) -> Result<Self> {
        // TODO(transport-protocol-v0.1.5): replace with
        // `animus_control_protocol::Client::connect(socket_path).await`.
        Ok(Self {
            socket_path: socket_path.to_path_buf(),
        })
    }

    /// Invoke a unary control RPC by name and decode the JSON response into `R`.
    pub async fn call<P, R>(&self, _method: &str, _params: &P) -> Result<R>
    where
        P: Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        anyhow::bail!(
            "control_client::call is a stub; awaiting animus-control-protocol v0.1.5 client. socket={}",
            self.socket_path.display()
        )
    }

    /// Open a streaming control RPC and return its notification stream.
    pub async fn subscribe<P, R>(
        &self,
        _method: &str,
        _params: &P,
    ) -> Result<std::pin::Pin<Box<dyn Stream<Item = Result<R>> + Send>>>
    where
        P: Serialize + Send + Sync,
        R: DeserializeOwned + Send + 'static,
    {
        anyhow::bail!(
            "control_client::subscribe is a stub; awaiting animus-control-protocol v0.1.5 client. socket={}",
            self.socket_path.display()
        )
    }
}
