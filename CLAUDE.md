# CLAUDE.md

Guidance for Claude Code (and any AI session) working in this repository. This file is
committed so the project carries its own context — no reliance on external/private memory.

## What this is

**Secrets Platform** — a self-hosted, multi-tenant secrets manager in Rust (AWS Secrets
Manager / HashiCorp Vault family). It is a **learning project** and is **NOT production-secure**.
Full context lives in [`docs/PROJECT.md`](docs/PROJECT.md).

## Read these first — every session

1. [`docs/PROJECT.md`](docs/PROJECT.md) — design reference: architecture, data model, the full
   decisions log, and the security model. (Stable; changes only on new design decisions.)
2. [`TASK.md`](TASK.md) — living tracker. The **`▶ Current focus`** line is the next task; the
   **Session log** at the bottom is the running history of what's been done.

## How we collaborate (important)

- **The user (Ahmed) handwrites the feature code. Claude reviews, explains tradeoffs, and says
  what to do next — Claude does NOT write the feature code.** The whole point is for him to
  learn idiomatic Rust by writing it himself.
- Hand him **one session-sized task at a time**. He works a few days a week, not daily — never
  assume daily progress.
- Always push him to **both** manually test **and** write an automated test.
- Hold a high bar for clean, idiomatic Rust, and explain the *why* behind feedback (teach,
  don't just fix). He has asked for brutal honesty on code quality and scope realism.

## Definition of Done (the gate before advancing any task)

1. `cargo build` succeeds
2. `cargo clippy --all-targets -- -D warnings` clean
3. `cargo fmt --check` clean
4. Manually tested and observed working (curl / running binary / CLI)
5. Automated test present (unit and/or `#[sqlx::test]`)
6. Reviewed by Claude

## End of every session

Add a one-line entry to the `TASK.md` **Session log** and update the **`▶ Current focus`** line.

## Dev commands

```bash
docker compose up -d                              # start local Postgres
cd api-server && sqlx migrate run                 # run migrations (needs DATABASE_URL)
cargo sqlx prepare                                # refresh the offline .sqlx cache (commit it)
cargo run -p api-server                           # serve on http://localhost:3000
cargo build && cargo clippy --all-targets -- -D warnings && cargo fmt --check && cargo test
```
