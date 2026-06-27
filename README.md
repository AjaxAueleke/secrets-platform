# Secrets Platform

> A self-hosted, multi-tenant secrets manager in Rust — versioned, encrypted at rest, and
> access-controlled by a path-aware policy engine.

[![Rust](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-early_development-yellow.svg)](TASK.md)

A self-hosted, multi-tenant **secrets management platform** written in Rust — in the spirit of
AWS Secrets Manager and HashiCorp Vault. It versions secrets, encrypts them at rest, controls
access with a path-aware policy engine, and exposes them via a REST API, a CLI, and (later) a
web app.

> ⚠️ **Learning project — NOT production-secure.** Built to learn idiomatic Rust and REST API
> design. Do not use it to store real secrets, and do not deploy it as a real secrets manager.
> See [`docs/PROJECT.md`](docs/PROJECT.md) §1 for the threat model and disclaimer.

## Features

The platform is designed around the pillars below. Most are on the roadmap — see
**[Status / roadmap](#status--roadmap)** for what actually runs today.

- **Versioned secrets** — append-only history, retain the last N versions, read any version (no rollback).
- **Encryption at rest** — envelope encryption: startup passphrase → KEK → per-secret DEK → XChaCha20-Poly1305 (M7).
- **Path-aware policy engine** — fine-grained verbs where *seeing that a secret exists* is separate from *revealing its value* (M6).
- **Multi-tenancy** — an `Organization → Project → Environment → Secret(path)` hierarchy.
- **Human + machine identity** — JWT access/refresh with argon2id passwords; hashed, show-once service-account tokens (M4–M5).
- **Append-only audit log** — who revealed what, when, and from where (M8).
- **Multiple interfaces** — a REST API, a CLI, and a server-rendered web app.

## Tech stack

Rust (edition 2024) · [axum](https://github.com/tokio-rs/axum) (HTTP) ·
[sqlx](https://github.com/launchbadge/sqlx) (compile-time-checked queries) · PostgreSQL ·
[tokio](https://tokio.rs/) · [tracing](https://github.com/tokio-rs/tracing) (structured logs) ·
Docker Compose (local Postgres).

## Architecture

A Cargo workspace. The API server is layered so each concern stays testable in isolation:

```
routes → controller → service → repository (trait) → PostgreSQL
```

| Crate | Purpose |
|---|---|
| `shared/` | Models, request/response DTOs, errors, policy types — feature-gated (`db`/`axum`) so the CLI stays slim. |
| `api-server/` | HTTP API following the layered flow above, plus `db` / `state` / `config` infra. |
| `cli/` | Command-line client (scaffold; built out in M9). |
| `worker/` | Background jobs — version pruning + purge (planned, M3). |

See [`docs/PROJECT.md`](docs/PROJECT.md) for the full design reference: data model, security
model, and decisions log.

## Status / roadmap

Early development. The live tracker — milestones, task checklists, and a session log — is in
**[`TASK.md`](TASK.md)**.

- [x] **M0** — Compiles, runs, serves `/health-check`, structured logging, env-driven config
- [ ] **M1** — Working slice: create + get one secret against a seeded tenant *(in progress)*
- [ ] **M2** — Full secret CRUD + versioning
- [ ] **M3** — Worker binary (prune + purge)
- [ ] **M4–M6** — Identity & auth (JWT), service-account tokens, policy engine
- [ ] **M7** — Encryption at rest *(real secrets allowed only after this)*
- [ ] **M8–M10** — Audit logging, CLI, web app

## Quickstart (local dev)

```bash
docker compose up -d                              # start Postgres (see docker-compose.yml)

# Create .env at the repo root (gitignored) with the dev database URL:
#   DATABASE_URL=postgresql://dev_user:dev_password@localhost:5432/dev_database

sqlx migrate run --source api-server/migrations   # apply migrations (reads DATABASE_URL from .env)
cargo run -p api-server                           # serves on http://localhost:3000
curl localhost:3000/health-check                  # -> 200 OK
```

Run the test suite (the current M0 tests are self-contained and need no database):

```bash
cargo test
```

## Documentation

- **[`docs/PROJECT.md`](docs/PROJECT.md)** — design reference: architecture, data model,
  decisions log, security model, glossary.
- **[`TASK.md`](TASK.md)** — milestone tracker, task checklist, and session log.
- **[`CLAUDE.md`](CLAUDE.md)** — how the project is built (AI/human collaboration workflow).

## License

Released under the [MIT License](LICENSE).
