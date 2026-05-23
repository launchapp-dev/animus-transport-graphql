//! Agent registry queries and runtime mutations.

use async_graphql::{Context, Object, Result, SimpleObject, ID};

use super::client_from_ctx;

#[derive(SimpleObject, Default)]
pub struct Agent {
    pub id: ID,
    pub name: String,
    pub kind: String,
    pub model: Option<String>,
    pub tool: Option<String>,
    pub enabled: bool,
}

#[derive(Default)]
pub struct AgentQuery;

#[Object]
impl AgentQuery {
    async fn agent(&self, ctx: &Context<'_>) -> Result<Vec<Agent>> {
        let _client = client_from_ctx(ctx).await?;
        // TODO(v0.1.5): client.call("agent.list", ())
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct AgentMutation;

#[Object]
impl AgentMutation {
    async fn enable_agent(&self, ctx: &Context<'_>, id: ID) -> Result<Agent> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(Agent::default())
    }

    async fn disable_agent(&self, ctx: &Context<'_>, id: ID) -> Result<Agent> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(Agent::default())
    }
}
