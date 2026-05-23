//! Dispatch queue queries and mutations.

use animus_control_protocol::types::{
    QueueEntry as WireQueueEntry, QueueEntryStatus, QueueListRequest,
};
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
        let client = client_from_ctx(ctx).await?;
        let request = QueueListRequest {
            status: only_ready
                .unwrap_or(false)
                .then_some(QueueEntryStatus::Ready),
            cursor: None,
            limit: None,
        };
        let response = client
            .queue_list(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("queue/list failed: {e}")))?;
        Ok(response.entries.into_iter().map(QueueEntry::from).collect())
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

    async fn hold_queue(
        &self,
        ctx: &Context<'_>,
        id: ID,
        reason: Option<String>,
    ) -> Result<QueueEntry> {
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

impl From<WireQueueEntry> for QueueEntry {
    fn from(entry: WireQueueEntry) -> Self {
        let state = match entry.status {
            QueueEntryStatus::Ready => "ready",
            QueueEntryStatus::Held => "held",
            QueueEntryStatus::InFlight => "in-flight",
            QueueEntryStatus::Done => "done",
            QueueEntryStatus::Dropped => "dropped",
        }
        .to_string();
        let held = matches!(entry.status, QueueEntryStatus::Held);
        QueueEntry {
            id: ID(entry.id),
            task_id: ID(entry.subject_id.0),
            workflow: String::new(),
            priority: entry.priority as i32,
            state,
            enqueued_at: entry.enqueued_at.to_rfc3339(),
            held,
            hold_reason: entry.hold_reason,
        }
    }
}
