# animus-transport-graphql

GraphQL transport plugin for [Animus](https://github.com/launchapp-dev/animus).
Exposes the daemon's control RPC surface as a GraphQL endpoint with queries,
mutations, and subscriptions.

## Overview

This plugin runs as a standalone Animus `transport_backend` and translates
inbound GraphQL operations into control RPCs against the local daemon over
the control socket. The wire shape from
[`animus-control-protocol`](https://github.com/launchapp-dev/animus-protocol)
is adopted directly as the canonical GraphQL contract — there is no
rich-vs-lean split.

## Endpoints

| Path             | Method  | Purpose                                  |
|------------------|---------|------------------------------------------|
| `/graphql`       | POST    | GraphQL query / mutation execution       |
| `/graphql`       | GET     | GraphQL Playground (interactive IDE)     |
| `/graphql/ws`    | WS      | Subscriptions over WebSocket             |
| `/graphql/sdl`   | GET     | Schema SDL dump                          |
| `/healthz`       | GET     | Liveness probe                           |

Default port: **8081** (HTTP transport occupies 8080).

## Schema coverage

**Queries:** `workflows`, `workflow(id)`, `queue`, `plugin`, `daemon`,
`subject`, `agent`.

**Mutations:** `runWorkflow`, `pauseWorkflow`, `resumeWorkflow`,
`cancelWorkflow`, `enqueue`, `dropQueue`, `holdQueue`, `releaseQueue`,
`reorderQueue`, `installPlugin`, `uninstallPlugin`, `createSubject`,
`updateSubject`.

**Subscriptions:** `workflowEvents`, `daemonEvents`, `subjectChanged`
(streamed through control wire notification RPCs).

## Build

```bash
cargo build --release
```

The binary lands at `target/release/animus-transport-graphql`.

## Run (standalone)

```bash
ANIMUS_CONTROL_SOCKET=/tmp/animus.sock \
ANIMUS_TRANSPORT_BIND=127.0.0.1:8081 \
./target/release/animus-transport-graphql
```

## Install into Animus

```bash
animus plugin install --kind transport_backend launchapp-dev/animus-transport-graphql
animus plugin enable animus-transport-graphql
```

## License

Elastic License 2.0. See [LICENSE](./LICENSE).
