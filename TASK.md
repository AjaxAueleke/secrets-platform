# TASK.md — Secrets Platform Tracker

> Living tracker for what's done and what's next. Design rationale lives in
> [`docs/PROJECT.md`](docs/PROJECT.md).

## How to use this file

- **Start a session:** read **Current focus** below, pick one task, do it.
- **Finish a task:** flip its `[ ]` to `[x]` only once it meets the **Definition of Done**.
- **End a session:** add one line to the **Session log** (date · what you did · what's next).
- Near-term milestones (M0–M2) are broken into session-sized tasks. Later milestones are
  coarse on purpose — we'll break them down when we get there, once you've learned more.
- Tasks describe *what* and *done when*. **You write the code; Claude reviews.**

## Definition of Done (applies to every task)

1. `cargo build` succeeds
2. `cargo clippy --all-targets -- -D warnings` clean
3. `cargo fmt --check` clean
4. Manually tested and observed working (curl / running binary / CLI)
5. Automated test covering it (unit and/or `#[sqlx::test]`)
6. Reviewed by Claude

---

## ▶ Current focus

**M1 · Task M1.1 — replace the migration with the real M1 schema: `organizations`, `projects`,
`environments`, `secrets`, `secret_versions` (see `docs/PROJECT.md` §4). Reset the dev DB with
`docker compose down -v`, then re-migrate. ⚠️ values stored PLAINTEXT until M7.**

---

## Progress at a glance

- [x] **M0** — Make it compile & run
- [ ] **M1** — Working slice (create + get one secret, seeded tenant) ⚠️ plaintext values
- [ ] **M2** — Full secret CRUD + versioning
- [ ] **M3** — Worker binary (prune + purge)
- [ ] **M4** — Identity & human auth (JWT)
- [ ] **M5** — Service accounts & machine tokens
- [ ] **M6** — Authorization / policy engine
- [ ] **M7** — Encryption at rest (real secrets allowed from here)
- [ ] **M8** — Full audit logging
- [ ] **M9** — CLI
- [ ] **M10** — Web app
- Future — language SDKs

---

## M0 — Make it compile & run

**Goal:** `cargo run` starts the server and `curl localhost:3000/health-check` returns 200.
No secret features yet. **Done when:** the whole M0 checklist is checked and the health check
works manually + a test passes.

- [x] **M0.1** Fix the health-check handler in `api-server/src/main.rs`. `get("healthcheck")`
      passes a string where a handler is required — replace it with a real async handler that
      returns a 200 and a small body.
- [x] **M0.2** Make `api-server/src/repository/secret.rs` compile. Known bugs from review:
      missing `<'a>` on the `impl`, malformed SQL (`RETURNING id)` stray paren), table name
      `secret` should be `secrets`, and the `query_as!(... secret.id,)()` call is malformed.
      For M0, make `insert` a minimal compiling stub (real implementation lands in M1). Remove
      unused imports (`sqlx::Executor` in main, `Json` in routes).
- [x] **M0.3** Add `tracing` + `tracing-subscriber`; replace `println!` with `tracing` macros;
      add a request-logging layer (`tower-http` `TraceLayer`).
- [x] **M0.4** Add a `Config` struct (e.g. `api-server/src/config.rs`) that reads
      `DATABASE_URL` and the bind address from env, instead of inline `env::var` in `main`.
- [x] **M0.5** Nail the DB workflow & document the commands in the session log / a README note:
      `docker compose up -d`, `sqlx migrate run`, and generate the offline cache with
      `cargo sqlx prepare` (commit the `.sqlx/` directory).
- [x] **M0.6** Write a small test for the health-check handler.
- [x] **M0.7** Verify everything: build, clippy, fmt, run, curl the health check → 200.

---

## M1 — Working slice (create + get one secret) ⚠️ values stored PLAINTEXT until M7

**Goal:** one secret can be created and read end-to-end (controller → service →
repository → Postgres) against a **seeded** tenant context. **Done when:** create + get work
via curl and are covered by a `#[sqlx::test]` integration test and a mocked-service unit test.

- [ ] **M1.1** Replace the migration with the M1 schema: `organizations`, `projects`,
      `environments`, `secrets`, `secret_versions` (see `docs/PROJECT.md` §4).
- [ ] **M1.2** Seed a dev org + project + environment + (placeholder) user for local dev.
- [ ] **M1.3** Define a `SecretRepository` **trait** + a Postgres impl (so the service is
      mockable). Implement `insert` (create secret + version 1) and `get` (by env + path).
- [ ] **M1.4** `CreateSecretRequest` validation with `validator` (path rules) + service-layer
      business rules.
- [ ] **M1.5** Wire the controller to call the service (stop returning the hardcoded `Secret`);
      service calls the repository; persist for real.
- [ ] **M1.6** Implement RFC 7807 problem+json error responses via `AppError`'s `IntoResponse`.
      Make sure value bytes are base64-encoded in responses.
- [ ] **M1.7** Decide & implement how the seeded tenant context reaches the handler (header or
      hardcoded) — a clearly-marked stand-in until M4 auth.
- [ ] **M1.8** Tests: `#[sqlx::test]` for the repository, a mocked-service unit test, and a
      handler test. Manual curl create + get.

---

## M2 — Full secret CRUD + versioning

**Goal:** the complete secret lifecycle minus security. **Done when:** every operation below
works via curl and has tests.

- [ ] List secrets in an environment (cursor pagination).
- [ ] Get a specific version (`?version=N`).
- [ ] Update value → creates a new version, bumps the current pointer.
- [ ] Retain last 10 versions (the rule; actual pruning is M3's worker).
- [ ] Soft-delete (set `deleted_at` + `purge_after`); deleting a non-empty org/project is blocked.
- [ ] Tags + description editing.
- [ ] Path validation enforced everywhere (strict segments).
- [ ] Environment promotion (copy a secret's value dev → prod).
- [ ] Bulk `.env` / JSON import + export for an environment.

---

## M3 — Worker binary (prune + purge)

- [ ] New `worker` crate (workspace member), shares the DB.
- [ ] Prune versions beyond the retained count (default 10, per-secret override).
- [ ] Purge soft-deleted secrets past their 7-day recovery window.
- [ ] Background loop + interval config; structured logging; tests for the prune/purge logic.

## M4 — Identity & human auth (JWT)

- [ ] `users`, `memberships`, `refresh_tokens` schema. Registration (user first).
- [ ] Org creation + join flow. argon2id password hashing.
- [ ] Login → JWT access + refresh; refresh rotation + revocation.
- [ ] Auth middleware: extract principal + real tenant context (replaces the M1 stand-in).

## M5 — Service accounts & machine tokens

- [ ] `service_accounts`, `tokens` schema. Create token (hashed, shown once).
- [ ] Bearer-token auth for machine callers; `last_used_at` tracking; revocation.

## M6 — Authorization / policy engine (keep minimal)

- [ ] `policies`, `policy_bindings` schema. Policy document type in `shared`.
- [ ] Evaluation engine: fine-grained verbs, path glob matching, allow/deny precedence.
- [ ] Enforce on every endpoint, for both users and service accounts.

## M7 — Encryption at rest (real secrets allowed after this)

- [ ] Seal/unseal: passphrase/keyfile → KEK at startup; `kek_versions`.
- [ ] Per-secret DEK wrapped by KEK; encrypt values with XChaCha20-Poly1305.
- [ ] Decrypt on `reveal-value`; migrate existing plaintext values.
- [ ] Master-key rotation by re-wrapping DEKs.

## M8 — Full audit logging

- [ ] `audit_log` schema (append-only). Record all access incl. value reveals
      (who/when/what/source IP). (Wire incrementally once identity exists from M4.)

## M9 — CLI

- [ ] `login` + kubectl-style config/context (~/.config); env-var overrides for CI.
- [ ] Commands: get / set / list / rotate / export (`.env`).

## M10 — Web app

- [ ] Server-rendered Rust + HTMX. (Break down when we arrive.)

---

## Session log

Newest first. One entry per working session: `date · what you did · what's next`.

- **2026-06-27** · Closed **M0.7** → **M0 milestone complete**. Full gate suite green
  (build/clippy `-D warnings`/fmt/7 tests), migration installed, health-check served 200
  (evidenced by M0.3 `tower_http` request log). Project compiles, runs, logs, and reads config
  from env. · **Next:** **M1.1** — real M1 schema (orgs/projects/environments/secrets/versions).
- **2026-06-27** · Closed **M0.5**: validated DB workflow — Postgres via docker compose, `0001`
  migration applied (`sqlx migrate info` → installed). Fixed the migrate command in README +
  CLAUDE.md (`sqlx migrate run --source api-server/migrations` from root, reads root `.env`) and
  added the `.env` creation step to the README. Removed obsolete `version:` from compose.
  **Deferred:** `cargo sqlx prepare` + committing `.sqlx/` → **M1** (no `query!` macros exist
  yet; the offline cache is empty until then, and that's when the pre-commit clippy needs it).
  · **Next:** **M0.7** final verify, then M1.
- **2026-06-27** · Closed **M0.4**: `Config` struct in `api-server/src/config.rs` reading
  `DATABASE_URL` + `BIND_ADDR` (defaults to `0.0.0.0:3000`), parsed to `SocketAddr`,
  `anyhow::Result` with `.context`, trims/rejects blank values. Testable `from_getter`/`from_env`
  split; 7 unit tests (happy/missing/empty/trim/default/invalid-addr) against an injected getter,
  no global env touched. · **Next:** **M0.5** DB workflow + offline `.sqlx` cache.
- **2026-06-25** · Closed **M0.3**: `tracing` + `tracing-subscriber` (registry + `EnvFilter`
  with `info,tower_http=debug` fallback + JSON `fmt` layer) and `TraceLayer` request logging.
  Verified manually: GET /health-check emits a `tower_http` request span (status 200). No
  automated test (logging output is brittle to assert — intentional gap). Also added a committed
  `.githooks/pre-commit` (fmt + clippy gate). · **Next:** **M0.4** Config struct.
- **2026-06-25** · Closed **M0.2**: repository compiles as a `todo!()` stub — deleted the
  dead/broken `INSERT_QUERY`, narrowed to `#[allow(dead_code)]`, `impl<'a>` fixed, unused
  imports gone. All gates green (real repo test deferred to M1.8). · **Next:** **M0.3** tracing.
- **2026-06-24** · Closed **M0.1 + M0.6**: real `healthcheck()` handler moved to
  `controller/healthcheck.rs`, `#[tokio::test]` asserting 200, curl'd OK. All gates green
  (build/clippy `-D warnings`/fmt/test). Repo `insert` left as a `todo!()` stub. · **Next:**
  **M0.2** tidy (drop dead `INSERT_QUERY`, narrow the `allow`s), then **M0.3** tracing.
- **2026-06-22** · Finalized the full design (13 rounds of decisions); wrote
  `docs/PROJECT.md` + this tracker; saved project memory. · **Next:** start **M0.1**
  (fix the health-check handler).
