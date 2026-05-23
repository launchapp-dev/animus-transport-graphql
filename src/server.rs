//! Axum app exposing the GraphQL endpoint, Playground, and SDL dump.

use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

use crate::{
    config::GraphqlConfig,
    schema::{build_schema, AnimusSchema},
};

pub async fn run(config: GraphqlConfig) -> anyhow::Result<()> {
    let bind = config.bind.clone();
    let cfg = Arc::new(config);
    let schema = build_schema(cfg.clone());

    let app = router(schema, cfg.clone());

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(addr = %bind, "graphql transport listening");
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn router(schema: AnimusSchema, cfg: Arc<GraphqlConfig>) -> Router {
    let playground_enabled = cfg.playground_enabled;

    let mut graphql_route = post(graphql_handler);
    if playground_enabled {
        graphql_route = graphql_route.get(graphql_playground);
    }

    Router::new()
        .route("/graphql", graphql_route)
        .route_service("/graphql/ws", GraphQLSubscription::new(schema.clone()))
        .route("/graphql/sdl", get(graphql_sdl_handler))
        .route("/healthz", get(healthz))
        .layer(Extension(schema))
        .layer(Extension(cfg))
}

async fn graphql_handler(
    Extension(schema): Extension<AnimusSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"),
    ))
}

async fn graphql_sdl_handler(Extension(schema): Extension<AnimusSchema>) -> impl IntoResponse {
    schema.sdl()
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}
