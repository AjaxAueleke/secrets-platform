# Secrets Platform

A self-hosted, multi-tenant **secrets management platform** written in Rust — in the spirit of
AWS Secrets Manager and HashiCorp Vault. It versions secrets, encrypts them at rest, controls
access with a path-aware policy engine, and exposes them via a REST API, a CLI, and (later) a
web app.

> ⚠️ **Learning project — NOT production-secure.** Built to learn idiomatic Rust and REST API
> design. Do not use it to store real secrets, and do not deploy it as a real secrets manager.
> See [`docs/PROJECT.md`](docs/PROJECT.md) §1 for the threat model and disclaimer.

## Status

Early development. The live roadmap and progress are in **[`TASK.md`](TASK.md)**.
Current milestone: **M0 — make it compile & run**.

## Documentation

- **[`docs/PROJECT.md`](docs/PROJECT.md)** — design reference: architecture, data model,
  decisions log, security model, glossary.
- **[`TASK.md`](TASK.md)** — milestone tracker, task checklist, and session log.
- **[`CLAUDE.md`](CLAUDE.md)** — how the project is built (AI/human collaboration workflow).

## Stack

Rust (edition 2024) · [axum](https://github.com/tokio-rs/axum) · [sqlx](https://github.com/launchbadge/sqlx) · PostgreSQL.

## Workspace layout

| Crate | Purpose |
|---|---|
| `shared/` | Models, DTOs, errors, policy types (feature-gated for `db`/`axum`) |
| `api-server/` | HTTP API: routes → controller → service → repository → Postgres |
| `worker/` | Background jobs: version pruning + purge (planned, M3) |
| `cli/` | Command-line client (planned, M9) |

## Quickstart (local dev)

```bash
docker compose up -d                  # start Postgres (see docker-compose.yml)
# DATABASE_URL is in .env (gitignored); export it or use a tool that loads .env
cd api-server && sqlx migrate run     # apply migrations
cargo run -p api-server               # serves on http://localhost:3000
curl localhost:3000/health-check
```
