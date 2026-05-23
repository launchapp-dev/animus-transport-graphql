//! Schema-shape contract tests. These do not require a running daemon — the
//! resolvers return stub values until the v0.1.5 control client lands.

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
async fn workflows_query_returns_empty_list_stub() {
    let schema = build_schema_no_subs(cfg());
    let res = schema.execute("{ workflows { id name status } }").await;
    assert!(res.errors.is_empty(), "unexpected errors: {:?}", res.errors);
    let data = res.data.into_json().expect("json data");
    assert_eq!(data["workflows"], serde_json::json!([]));
}

#[tokio::test]
async fn queue_query_returns_empty_stub() {
    let schema = build_schema_no_subs(cfg());
    let res = schema
        .execute("{ queue { id taskId workflow priority state } }")
        .await;
    assert!(res.errors.is_empty(), "unexpected errors: {:?}", res.errors);
    let data = res.data.into_json().expect("json data");
    assert_eq!(data["queue"], serde_json::json!([]));
}
