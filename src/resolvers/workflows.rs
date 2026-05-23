//! Workflow queries, mutations, and event subscription.
//!
//! Wire shape: defers to `animus-control-protocol` types so the GraphQL
//! schema and the JSON-RPC wire stay in lockstep (canonical contract).

use animus_control_protocol::types::WorkflowEventsRequest;
use async_graphql::{Context, Object, Result, SimpleObject, Subscription, ID};
use futures_util::stream::{self, Stream};

use super::client_from_ctx;

/// Lean workflow projection mirroring the control-wire `Workflow` shape.
#[derive(SimpleObject, Default)]
pub struct Workflow {
    pub id: ID,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub task_id: Option<String>,
    pub current_phase: Option<String>,
}

#[derive(SimpleObject, Default)]
pub struct WorkflowEvent {
    pub workflow_id: ID,
    pub kind: String,
    pub payload: String,
    pub at: String,
}

#[derive(Default)]
pub struct WorkflowQuery;

#[Object]
impl WorkflowQuery {
    /// List workflows, optionally filtered by status.
    async fn workflows(&self, ctx: &Context<'_>, status: Option<String>) -> Result<Vec<Workflow>> {
        let _client = client_from_ctx(ctx).await?;
        // TODO(v0.1.5): client.call("workflow.list", &{ status })
        let _ = status;
        Ok(Vec::new())
    }

    /// Look up a single workflow by id.
    async fn workflow(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Workflow>> {
        let _client = client_from_ctx(ctx).await?;
        // TODO(v0.1.5): client.call("workflow.get", &{ id })
        let _ = id;
        Ok(None)
    }
}

#[derive(Default)]
pub struct WorkflowMutation;

#[Object]
impl WorkflowMutation {
    async fn run_workflow(
        &self,
        ctx: &Context<'_>,
        task_id: ID,
        workflow: String,
    ) -> Result<Workflow> {
        let _client = client_from_ctx(ctx).await?;
        let _ = (task_id, workflow);
        // TODO(v0.1.5): client.call("workflow.run", ...)
        Ok(Workflow::default())
    }

    async fn pause_workflow(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(true)
    }

    async fn resume_workflow(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(true)
    }

    async fn cancel_workflow(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(true)
    }
}

#[derive(Default)]
pub struct WorkflowEventsSubscription;

#[Subscription]
impl WorkflowEventsSubscription {
    /// Subscribers may hang on `recv` until the daemon-side `workflow/events`
    /// handler ships — v0.1.10 only ships the client-side surface.
    async fn workflow_events(
        &self,
        ctx: &Context<'_>,
        workflow_id: Option<ID>,
        kinds: Option<Vec<String>>,
    ) -> Result<impl Stream<Item = WorkflowEvent>> {
        let client = client_from_ctx(ctx).await?;
        let request = WorkflowEventsRequest {
            workflow_id: workflow_id.map(|id| id.to_string()),
            kinds,
        };
        let subscription = client
            .workflow_events(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("workflow/events failed: {e}")))?;
        Ok(stream::unfold(
            (subscription, client),
            |(mut sub, client)| async move {
                let event = sub.recv().await?;
                let projected = WorkflowEvent {
                    workflow_id: ID(event.workflow_id),
                    kind: event.kind,
                    payload: event.payload.to_string(),
                    at: event.occurred_at.to_rfc3339(),
                };
                Some((projected, (sub, client)))
            },
        ))
    }
}
