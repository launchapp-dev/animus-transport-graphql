//! Schema-shape contract tests. These do not require a running daemon —
//! schema introspection works without a socket, and resolver queries that
//! reach for the control socket surface a connection error envelope.

use std::sync::Arc;

use animus_transport_graphql::{config::GraphqlConfig, schema::build_schema_no_subs};

fn cfg() -> Arc<GraphqlConfig> {
    Arc::new(GraphqlConfig::default())
}

#[tokio::test]
async fn introspection_includes_required_roots() {
    let schema = build_schema_no_subs(cfg());
    let sdl = schema.sdl();

    for needle in [
        "type QueryRoot",
        "type MutationRoot",
        "workflows",
        "queue",
        "plugin",
        "daemon",
        "subject",
        "agent",
        "runWorkflow",
        "enqueue",
        "installPlugin",
        "createSubject",
    ] {
        assert!(
            sdl.contains(needle),
            "schema SDL missing expected fragment: {needle}\n--- SDL ---\n{sdl}"
        );
    }
}

#[tokio::test]
async fn workflows_query_surfaces_connection_error_without_daemon() {
    let schema = build_schema_no_subs(cfg());
    let res = schema.execute("{ workflows { id name status } }").await;
    assert!(
        !res.errors.is_empty(),
        "expected control-socket error, got data: {:?}",
        res.data
    );
    let msg = res.errors[0].message.clone();
    assert!(
        msg.contains("control socket"),
        "expected control-socket failure, got: {msg}"
    );
}

#[tokio::test]
async fn queue_query_surfaces_connection_error_without_daemon() {
    let schema = build_schema_no_subs(cfg());
    let res = schema
        .execute("{ queue { id taskId workflow priority state } }")
        .await;
    assert!(
        !res.errors.is_empty(),
        "expected control-socket error, got data: {:?}",
        res.data
    );
    let msg = res.errors[0].message.clone();
    assert!(
        msg.contains("control socket"),
        "expected control-socket failure, got: {msg}"
    );
}
