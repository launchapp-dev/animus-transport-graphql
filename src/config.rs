use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Runtime config for the GraphQL transport backend.
///
/// Populated from the `TransportBackend::start` payload supplied by the
/// daemon, with fallbacks to environment variables for standalone runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlConfig {
    /// `host:port` to bind the Axum server to.
    pub bind: String,
    /// Path to the daemon's control socket (unix) or pipe (windows).
    pub control_socket_path: PathBuf,
    /// Optional bearer token required on `/graphql` requests.
    #[serde(default)]
    pub auth_token: Option<String>,
    /// Enable the interactive GraphQL Playground on GET `/graphql`.
    #[serde(default = "default_playground_enabled")]
    pub playground_enabled: bool,
}

fn default_playground_enabled() -> bool {
    true
}

impl GraphqlConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind =
            std::env::var("ANIMUS_TRANSPORT_BIND").unwrap_or_else(|_| "127.0.0.1:8081".into());
        let control_socket_path = std::env::var("ANIMUS_CONTROL_SOCKET")
            .map(PathBuf::from)
            .map_err(|_| anyhow::anyhow!("ANIMUS_CONTROL_SOCKET env var is required"))?;
        let auth_token = std::env::var("ANIMUS_TRANSPORT_AUTH_TOKEN").ok();
        let playground_enabled = std::env::var("ANIMUS_GRAPHQL_PLAYGROUND")
            .ok()
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true);

        Ok(Self {
            bind,
            control_socket_path,
            auth_token,
            playground_enabled,
        })
    }
}

impl Default for GraphqlConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:8081".into(),
            control_socket_path: PathBuf::from("/tmp/animus.sock"),
            auth_token: None,
            playground_enabled: true,
        }
    }
}
