# Secrets Platform — Project Reference

> **This is the stable design reference.** It changes rarely (only when we make a new design
> decision). For "what's done / what's next", see [`TASK.md`](../TASK.md) at the repo root.
>
> Last updated: 2026-06-22

---

## 1. What this is

A self-hosted, **multi-tenant secrets management platform** in Rust — conceptually in the
family of AWS Secrets Manager and HashiCorp Vault. It stores secrets (encrypted at rest),
versions them, controls who/what can read them via a policy engine, and exposes them through
a REST API, a CLI, and (eventually) a web app.

### Why it exists

This is a **learning project**. The goal is to learn idiomatic Rust and REST API design by
building something real and non-trivial. Shipping a product is secondary to *understanding
how each piece works*.

### ⚠️ Security disclaimer (read this)

This system is **NOT production-secure** and must never be presented as such. We roll our own
auth and crypto orchestration for learning purposes. We mitigate the obvious risks by leaning
on **vetted crates** (`argon2`, `jsonwebtoken`, a reviewed AEAD) rather than inventing
cryptographic primitives — but the threat model (e.g. master key derived from a startup
passphrase living near the database) is explicitly a learning-grade compromise, not a
hardened one. Do not store real secrets in it before the encryption milestone (M7), and do
not deploy it as a real secrets manager.

---

## 2. How we work (the workflow)

- **The user (Ahmed) handwrites the code.** Claude reviews, nudges, explains tradeoffs, and
  says what to do next — Claude does **not** write the feature code.
- **Definition of Done for any task** (the gate before we advance):
  1. `cargo build` succeeds.
  2. `cargo clippy --all-targets -- -D warnings` is clean.
  3. `cargo fmt --check` is clean.
  4. The change has been **manually tested** (curl / running the binary / CLI) and observed working.
  5. There is an **automated test** covering it (unit and/or `#[sqlx::test]` integration).
  6. Claude has reviewed it.
- We do **not** start the next task until the current one meets Done.
- Time budget is real: the user works on this **a few days a week, not daily**. Tasks are
  sized to fit a single focused session. Each session ends with a one-line entry in the
  `TASK.md` session log.

---

## 3. Architecture

A Cargo workspace (`edition = "2024"`, resolver 2):

```
shared/      Shared models, request/response DTOs, errors, policy types.
             Feature-gated: `db` -> sqlx, `axum` -> axum, so the CLI doesn't pull server deps.
api-server/  The axum HTTP API. Layered:
               routes      -> URL wiring
               controller  -> HTTP <-> domain (extract, validate shape, map errors)
               service     -> business logic, validation rules, orchestration
               repository  -> data access (behind a trait, so it is mockable)
               db / state / config -> infra
worker/      [NEW, M3] Separate binary: version pruning + recovery-window purge.
cli/         [M9] clap + reqwest client (login, get/set/list/rotate, export).
web/         [M10] Server-rendered Rust + HTMX.
```

Request flow: `routes -> controller -> service -> repository(trait) -> Postgres`.

---

## 4. Data model (target)

Tables are tagged with the milestone that introduces them. Only M1 tables exist initially.

```
M1  organizations    (id, name, slug UNIQUE, created_at, updated_at)
M1  projects         (id, org_id FK, name, UNIQUE(org_id, name), timestamps)
M1  environments     (id, project_id FK, name, UNIQUE(project_id, name), timestamps)  -- free-form names
M1  secrets          (id, environment_id FK, path, description, tags JSONB,
                      current_version INT, deleted_at, purge_after,
                      UNIQUE(environment_id, path) WHERE deleted_at IS NULL, timestamps)
M1  secret_versions  (id, secret_id FK, version INT, value BYTEA, created_at,
                      UNIQUE(secret_id, version))
                      -- M7 adds: wrapped_dek BYTEA, nonce BYTEA, kek_version INT

M4  users            (id, email UNIQUE, password_hash, timestamps)
M4  memberships      (id, user_id FK, org_id FK, role, UNIQUE(user_id, org_id))  -- role: owner|admin|member
M4  refresh_tokens   (id, user_id FK, token_hash UNIQUE, expires_at, revoked_at, created_at)

M5  service_accounts (id, org_id FK, name, timestamps)
M5  tokens           (id, principal_type, principal_id, token_hash UNIQUE, name,
                      last_used_at, expires_at, revoked_at, created_at)  -- hashed, shown once

M6  policies         (id, org_id FK, name, document JSONB, timestamps)  -- statements: {effect, actions[], paths[]}
M6  policy_bindings  (id, policy_id FK, principal_type, principal_id)

M7  kek_versions     (id, version INT, created_at, ...)  -- supports master-key rotation

M8  audit_log        (id, org_id, actor_type, actor_id, action, resource_type, resource_id,
                      path, source_ip, metadata JSONB, created_at)  -- append-only
```

### Identity hierarchy

`Organization → Project → Environment → Secret(path) → versions`

- A user belongs to **many** orgs (via `memberships`).
- Environments are **first-class** and **free-form** ("dev", "prod", "eu-prod", ...). A new
  project starts with **no** environments; you create them explicitly.
- Secrets are **independent per environment**: `prod`'s `db/password` and `dev`'s `db/password`
  are unrelated rows. The same path in another environment is a separate secret by convention.
- Secret identity is `(environment_id, path)`. Paths are **strict**: `/`-separated segments of
  `[A-Za-z0-9._-]`, no leading/trailing slash, capped depth and length.

---

## 5. Security model (summary)

### Authentication
- **Humans** (web/CLI): JWT **access + refresh** tokens. Passwords hashed with **argon2id**.
  Refresh tokens are stored hashed and support rotation + revocation.
- **Machines** (apps fetching secrets): **service-account tokens** — first-class non-human
  identities. Tokens are stored **hashed** and shown **once** at creation. Scoped via policy
  (per secret-path).

### Authorization — the policy engine (M6)
- Org-level roles: **Owner / Admin / Member** (for managing the org itself).
- Access to secrets is **policy-based and path-aware**. A policy document is a list of
  statements: `{ effect: allow|deny, actions: [...], paths: ["prod/db/*", ...] }`.
- **Fine-grained verbs**: `list`, `read-metadata`, `reveal-value`, `write`, `delete`,
  `rotate`, `manage-access`, `manage-tokens`. Crucially, *seeing that a secret exists*
  (`read-metadata`) is separate from *revealing its value* (`reveal-value`).
- Policies attach to both human principals and service accounts (`policy_bindings`).
- **Keep this engine minimal.** It is the easiest subsystem to over-engineer.

### Encryption at rest (M7) — envelope encryption
- Server starts **sealed**. An **unseal passphrase** (or keyfile) at startup derives/unwraps
  the **master key (KEK)**.
- Each secret gets its own **data key (DEK)**, wrapped by the KEK.
- Secret values are sealed with an **AEAD** under the DEK.
- **Cipher choice (delegated to Claude):** **XChaCha20-Poly1305** (via the `chacha20poly1305`
  RustCrypto crate). Rationale: its 192-bit nonce makes random-nonce reuse practically
  impossible, which is the right property for this envelope scheme. (AES-256-GCM is the
  documented alternative if hardware acceleration ever matters more.)
- **Master-key rotation** is supported by **re-wrapping** DEKs with the new KEK — no need to
  re-encrypt every value.

---

## 6. Decisions log

The full set of decisions made during the 2026-06-22 design session.

| Area | Decision |
|---|---|
| Sequencing | Working slice first; security layered in later milestones. |
| Tenancy | Multi-tenant: Org → Project → Environment → Secret. |
| Secret value | Opaque single value, binary-capable (BYTEA, base64 over API). |
| Naming | Path-style, strict segments `[A-Za-z0-9._-]`, capped depth/length. |
| Metadata | Standard: description + tags/labels (JSONB). |
| Versioning | Append-only; retain last **10** (per-secret override); single current pointer; read any version, **no rollback**. |
| Deletion | Soft-delete + **7-day** recovery window, then purge; deleting a non-empty org/project is **blocked**. |
| Multi-org | A user can belong to many orgs. |
| Onboarding | Sign up creates the user first; orgs created/joined later. |
| Env names | Free-form; projects start with **no** environments. |
| Env/secret | Independent per environment. |
| Human auth | JWT access + refresh; argon2id passwords. |
| Machine auth | Service-account tokens, hashed + shown once, scoped per secret-path. |
| Authorization | Org roles Owner/Admin/Member **+ path-aware policy engine**, fine-grained verbs. |
| Encryption | Seal/unseal passphrase → KEK → per-secret DEK → XChaCha20-Poly1305; master-key rotation via re-wrap. |
| Rotation | Manual (submit new value), on-demand only. |
| Generation | Passwords/strings (configurable length/charset/complexity). |
| External secrets | **Not supported** (platform is sole source of truth). |
| Promotion | Secrets promotable between environments (dev → prod). |
| Bulk ops | `.env` + JSON import **and** export. |
| API addressing | Flat-by-ID + nested lists; slashed paths travel in body/query. |
| API responses | Raw JSON; errors as **RFC 7807** problem+json. |
| Pagination | Cursor-based. |
| Audit | Full audit (all access incl. value reveals), append-only. |
| sqlx | Compile-time checked macros + committed `.sqlx` offline cache; repository behind a trait. |
| Testing | Mocked services + `#[sqlx::test]` for repos/handlers. |
| Errors | Single shared `AppError` to start; split per-layer later as it grows. |
| Validation | `validator` derive (field shape) + service-layer business rules. |
| Observability | `tracing` + structured logs. |
| API versioning | `/v1` URI prefix (Claude recommendation; revisit if needed). |
| Background worker | Separate `worker` binary. |
| CLI | `login` + kubectl-style config/context; env-var overrides for CI. |
| Web | Server-rendered Rust + HTMX (M10). |

---

## 7. Out of scope (foreseeable roadmap)

- Automated rotation that connects to live systems to change real credentials.
- MFA / SSO / SAML.
- Webhooks / notifications.
- Billing / payments.
- Language SDKs (future, post-roadmap).

---

## 8. Glossary

- **Org / Project / Environment** — the tenancy hierarchy levels.
- **Secret** — a named (path) entry within one environment; has many versions.
- **Version** — an immutable snapshot of a secret's value.
- **Current pointer** — `secrets.current_version`, the version reads return by default.
- **KEK** — Key Encryption Key (the master key, unwrapped at startup).
- **DEK** — Data Encryption Key (per-secret, wrapped by the KEK).
- **AEAD** — Authenticated Encryption with Associated Data (the cipher type we use).
- **Policy** — a document of `{effect, actions, paths}` statements granting/denying access.
- **Principal** — an actor: a user or a service account.
- **Seal/unseal** — the server boots "sealed"; an unseal passphrase makes it able to decrypt.

---

## 9. Pointers

- [`TASK.md`](../TASK.md) — the living tracker: milestones, checkable tasks, session log.
- `docker-compose.yml` — local Postgres.
- `api-server/migrations/` — sqlx migrations.
