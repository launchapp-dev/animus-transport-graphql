//! `TransportBackend` impl that drives the Axum + async-graphql server.
//!
//! The trait surface lives in `animus-transport-protocol` (v0.1.5).
//! Until that crate's API stabilises, the impl here is the working draft —
//! the trait methods compile against what the spec describes; rename / adjust
//! as the published trait lands.

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use animus_transport_protocol::{TransportBackend, TransportSchema, TransportStartRequest, TransportStopRequest};

use crate::{config::GraphqlConfig, server};

#[derive(Default)]
pub struct GraphqlTransportBackend {
    handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

#[async_trait]
impl TransportBackend for GraphqlTransportBackend {
    fn schema(&self) -> TransportSchema {
        TransportSchema {
            kinds: vec!["graphql".into()],
            supports_streaming: true,
            supports_websocket: true,
            default_port: Some(8081),
        }
    }

    async fn start(&self, req: TransportStartRequest) -> anyhow::Result<()> {
        let cfg = GraphqlConfig {
            bind: req.bind.unwrap_or_else(|| "127.0.0.1:8081".into()),
            control_socket_path: req.control_socket_path,
            auth_token: req.auth_token,
            playground_enabled: true,
        };

        let task = tokio::spawn(async move {
            if let Err(err) = server::run(cfg).await {
                tracing::error!(error = %err, "graphql transport server exited with error");
            }
        });

        let mut guard = self.handle.lock().await;
        if let Some(prev) = guard.take() {
            prev.abort();
        }
        *guard = Some(task);
        Ok(())
    }

    async fn stop(&self, _req: TransportStopRequest) -> anyhow::Result<()> {
        let mut guard = self.handle.lock().await;
        if let Some(handle) = guard.take() {
            handle.abort();
        }
        Ok(())
    }
}
