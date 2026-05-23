//! Daemon health/status queries, control mutations, and event stream.

use animus_control_protocol::types::DaemonEventsRequest;
use async_graphql::{Context, Object, Result, SimpleObject, Subscription};
use futures_util::stream::{self, Stream};

use super::client_from_ctx;

#[derive(SimpleObject, Default)]
pub struct DaemonStatus {
    pub running: bool,
    pub pid: Option<i32>,
    pub started_at: Option<String>,
    pub version: Option<String>,
    pub uptime_seconds: Option<i64>,
}

#[derive(SimpleObject, Default)]
pub struct DaemonHealth {
    pub healthy: bool,
    pub checks: Vec<HealthCheck>,
}

#[derive(SimpleObject, Default)]
pub struct HealthCheck {
    pub name: String,
    pub healthy: bool,
    pub message: Option<String>,
}

#[derive(SimpleObject, Default)]
pub struct DaemonEvent {
    pub kind: String,
    pub payload: String,
    pub at: String,
}

#[derive(Default)]
pub struct DaemonQuery;

#[Object]
impl DaemonQuery {
    async fn daemon(&self, ctx: &Context<'_>) -> Result<DaemonStatus> {
        let _client = client_from_ctx(ctx).await?;
        Ok(DaemonStatus::default())
    }

    async fn daemon_health(&self, ctx: &Context<'_>) -> Result<DaemonHealth> {
        let _client = client_from_ctx(ctx).await?;
        Ok(DaemonHealth::default())
    }
}

#[derive(Default)]
pub struct DaemonMutation;

#[Object]
impl DaemonMutation {
    async fn pause_daemon(&self, ctx: &Context<'_>) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        Ok(true)
    }

    async fn resume_daemon(&self, ctx: &Context<'_>) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct DaemonEventsSubscription;

#[Subscription]
impl DaemonEventsSubscription {
    async fn daemon_events(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = DaemonEvent>> {
        let client = client_from_ctx(ctx).await?;
        let subscription = client
            .daemon_events(DaemonEventsRequest::default())
            .await
            .map_err(|e| async_graphql::Error::new(format!("daemon/events failed: {e}")))?;
        Ok(stream::unfold(
            (subscription, client),
            |(mut sub, client)| async move {
                let event = sub.recv().await?;
                let projected = DaemonEvent {
                    kind: event.kind,
                    payload: event.payload.to_string(),
                    at: event.occurred_at.to_rfc3339(),
                };
                Some((projected, (sub, client)))
            },
        ))
    }
}
