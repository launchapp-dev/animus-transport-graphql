//! Subject (task/requirement/etc.) queries, mutations, and change stream.

use animus_control_protocol::types::SubjectWatchRequest;
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
        let client = client_from_ctx(ctx).await?;
        let request = SubjectWatchRequest { kind, filter: None };
        let subscription = client
            .subject_watch(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("subject/watch failed: {e}")))?;
        Ok(stream::unfold(
            (subscription, client),
            |(mut sub, client)| async move {
                let event = sub.recv().await?;
                let projected = SubjectChangeEvent {
                    subject_id: ID(event.id.as_str().to_string()),
                    change: format!("{:?}", event.change_kind).to_lowercase(),
                    at: event.subject.updated_at.to_rfc3339(),
                };
                Some((projected, (sub, client)))
            },
        ))
    }
}
