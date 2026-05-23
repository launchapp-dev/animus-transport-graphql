use animus_plugin_protocol::PluginInfo;
use animus_plugin_runtime::transport_backend_main;
use animus_transport_graphql::backend::GraphqlTransportBackend;
use animus_transport_protocol::PLUGIN_KIND_TRANSPORT_BACKEND;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let backend = GraphqlTransportBackend::default();
    let info = PluginInfo {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        plugin_kind: PLUGIN_KIND_TRANSPORT_BACKEND.into(),
        description: Some(env!("CARGO_PKG_DESCRIPTION").into()),
    };

    transport_backend_main(info, backend).await
}
