//! Dispatch queue queries and mutations.

use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};

use super::client_from_ctx;

#[derive(SimpleObject, Default)]
pub struct QueueEntry {
    pub id: ID,
    pub task_id: ID,
    pub workflow: String,
    pub priority: i32,
    pub state: String,
    pub enqueued_at: String,
    pub held: bool,
    pub hold_reason: Option<String>,
}

#[derive(SimpleObject, Default)]
pub struct QueueStats {
    pub total: i32,
    pub ready: i32,
    pub held: i32,
    pub dispatched: i32,
}

#[derive(InputObject)]
pub struct EnqueueInput {
    pub task_id: ID,
    pub workflow: String,
    #[graphql(default = 0)]
    pub priority: i32,
}

#[derive(Default)]
pub struct QueueQuery;

#[Object]
impl QueueQuery {
    /// List queue entries.
    async fn queue(&self, ctx: &Context<'_>, only_ready: Option<bool>) -> Result<Vec<QueueEntry>> {
        let _client = client_from_ctx(ctx).await?;
        let _ = only_ready;
        // TODO(v0.1.5): client.call("queue.list", ...)
        Ok(Vec::new())
    }

    async fn queue_stats(&self, ctx: &Context<'_>) -> Result<QueueStats> {
        let _client = client_from_ctx(ctx).await?;
        Ok(QueueStats::default())
    }
}

#[derive(Default)]
pub struct QueueMutation;

#[Object]
impl QueueMutation {
    async fn enqueue(&self, ctx: &Context<'_>, input: EnqueueInput) -> Result<QueueEntry> {
        let _client = client_from_ctx(ctx).await?;
        let _ = input;
        Ok(QueueEntry::default())
    }

    async fn drop_queue(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(true)
    }

    async fn hold_queue(&self, ctx: &Context<'_>, id: ID, reason: Option<String>) -> Result<QueueEntry> {
        let _client = client_from_ctx(ctx).await?;
        let _ = (id, reason);
        Ok(QueueEntry::default())
    }

    async fn release_queue(&self, ctx: &Context<'_>, id: ID) -> Result<QueueEntry> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(QueueEntry::default())
    }

    async fn reorder_queue(&self, ctx: &Context<'_>, ids: Vec<ID>) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = ids;
        Ok(true)
    }
}
