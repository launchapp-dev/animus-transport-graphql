//! Subject (task/requirement/etc.) queries, mutations, and change stream.

use async_graphql::{Context, InputObject, Object, Result, SimpleObject, Subscription, ID};
use futures_util::stream::{self, Stream};

use super::client_from_ctx;

#[derive(SimpleObject, Default)]
pub struct Subject {
    pub id: ID,
    pub kind: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<String>,
}

#[derive(InputObject)]
pub struct CreateSubjectInput {
    pub kind: String,
    pub title: String,
    pub metadata: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateSubjectInput {
    pub id: ID,
    pub title: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<String>,
}

#[derive(SimpleObject, Default)]
pub struct SubjectChangeEvent {
    pub subject_id: ID,
    pub change: String,
    pub at: String,
}

#[derive(Default)]
pub struct SubjectQuery;

#[Object]
impl SubjectQuery {
    async fn subject(&self, ctx: &Context<'_>, kind: Option<String>) -> Result<Vec<Subject>> {
        let _client = client_from_ctx(ctx).await?;
        let _ = kind;
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct SubjectMutation;

#[Object]
impl SubjectMutation {
    async fn create_subject(
        &self,
        ctx: &Context<'_>,
        input: CreateSubjectInput,
    ) -> Result<Subject> {
        let _client = client_from_ctx(ctx).await?;
        let _ = input;
        Ok(Subject::default())
    }

    async fn update_subject(
        &self,
        ctx: &Context<'_>,
        input: UpdateSubjectInput,
    ) -> Result<Subject> {
        let _client = client_from_ctx(ctx).await?;
        let _ = input;
        Ok(Subject::default())
    }
}

#[derive(Default)]
pub struct SubjectChangedSubscription;

#[Subscription]
impl SubjectChangedSubscription {
    async fn subject_changed(
        &self,
        ctx: &Context<'_>,
        kind: Option<String>,
    ) -> Result<impl Stream<Item = SubjectChangeEvent>> {
        let _client = client_from_ctx(ctx).await?;
        let _ = kind;
        // Blocked on animus-control-protocol: METHOD_SUBJECT_WATCH /
        // NOTIFICATION_SUBJECT_CHANGED are reserved in method.rs but
        // ControlClient v0.1.8 exposes no subscribe/stream API. Wire this
        // once upstream adds e.g. `ControlClient::subject_watch(...) ->
        // Stream<SubjectChangeEvent>` (tracking: animus-protocol#TBD).
        Ok(stream::empty())
    }
}
