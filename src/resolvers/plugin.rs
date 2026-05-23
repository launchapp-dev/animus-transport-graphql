//! Plugin registry queries and lifecycle mutations.

use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};

use super::client_from_ctx;

#[derive(SimpleObject, Default)]
pub struct Plugin {
    pub id: ID,
    pub name: String,
    pub version: String,
    pub kind: String,
    pub enabled: bool,
    pub installed_at: String,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct InstallPluginInput {
    pub source: String,
    pub kind: Option<String>,
    pub version: Option<String>,
}

#[derive(Default)]
pub struct PluginQuery;

#[Object]
impl PluginQuery {
    async fn plugin(&self, ctx: &Context<'_>, kind: Option<String>) -> Result<Vec<Plugin>> {
        let _client = client_from_ctx(ctx).await?;
        let _ = kind;
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct PluginMutation;

#[Object]
impl PluginMutation {
    async fn install_plugin(&self, ctx: &Context<'_>, input: InstallPluginInput) -> Result<Plugin> {
        let _client = client_from_ctx(ctx).await?;
        let _ = input;
        Ok(Plugin::default())
    }

    async fn uninstall_plugin(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _client = client_from_ctx(ctx).await?;
        let _ = id;
        Ok(true)
    }
}
