//! Schema-shape contract tests. These do not require a running daemon —
//! schema introspection works without a socket, and resolver queries that
//! reach for the control socket surface a connection error envelope.

use std::sync::Arc;

use animus_plugin_protocol::{RpcRequest, RpcResponse};
use animus_transport_graphql::{config::GraphqlConfig, schema::build_schema_no_subs};
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

fn cfg() -> Arc<GraphqlConfig> {
    Arc::new(GraphqlConfig::default())
}

fn cfg_with_socket(path: std::path::PathBuf) -> Arc<GraphqlConfig> {
    Arc::new(GraphqlConfig {
        control_socket_path: path,
        ..GraphqlConfig::default()
    })
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

#[tokio::test]
async fn queue_query_maps_entries_from_mock_daemon() {
    let tmp = TempDir::new().unwrap();
    let socket = tmp.path().join("control.sock");
    let listener = UnixListener::bind(&socket).expect("bind socket");

    tokio::spawn(async move {
        let (mut conn, _) = listener.accept().await.expect("accept");
        let (read_half, mut write_half) = conn.split();
        let mut reader = BufReader::new(read_half);
        let mut line = String::new();
        reader.read_line(&mut line).await.expect("read request");
        let req: RpcRequest = serde_json::from_str(line.trim()).expect("decode request");
        assert_eq!(req.method, "queue/list");
        let resp = RpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(serde_json::json!({
                "entries": [
                    {
                        "id": "q-1",
                        "subject_id": "task:T1",
                        "status": "ready",
                        "priority": 3u8,
                        "enqueued_at": "2026-05-23T00:00:00Z"
                    },
                    {
                        "id": "q-2",
                        "subject_id": "task:T2",
                        "status": "held",
                        "priority": 1u8,
                        "enqueued_at": "2026-05-23T00:01:00Z",
                        "hold_reason": "manual"
                    },
                    {
                        "id": "q-3",
                        "subject_id": "task:T3",
                        "status": "in-flight",
                        "priority": 2u8,
                        "enqueued_at": "2026-05-23T00:02:00Z"
                    }
                ],
                "next_cursor": null
            })),
            error: None,
        };
        let mut frame = serde_json::to_string(&resp).unwrap();
        frame.push('\n');
        write_half.write_all(frame.as_bytes()).await.unwrap();
        write_half.flush().await.unwrap();
    });

    let schema = build_schema_no_subs(cfg_with_socket(socket));
    let res = schema
        .execute("{ queue { id taskId priority state held holdReason enqueuedAt } }")
        .await;
    assert!(res.errors.is_empty(), "unexpected errors: {:?}", res.errors);
    let data = res.data.into_json().unwrap();
    let entries = data["queue"].as_array().expect("queue array");
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0]["id"], "q-1");
    assert_eq!(entries[0]["taskId"], "task:T1");
    assert_eq!(entries[0]["state"], "ready");
    assert_eq!(entries[0]["held"], false);
    assert_eq!(entries[0]["priority"], 3);
    assert_eq!(entries[1]["state"], "held");
    assert_eq!(entries[1]["held"], true);
    assert_eq!(entries[1]["holdReason"], "manual");
    assert_eq!(entries[2]["state"], "in-flight");
}
