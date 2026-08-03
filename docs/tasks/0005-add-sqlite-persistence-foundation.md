# Task 05: Add SQLite persistence foundation

- **GitHub issue:** [#5](https://github.com/Phineas-bot/gixgiz/issues/5)
- **Depends on:** Tasks 03 and 04
- **Primary ADRs:** 0001, 0004, 0005
- **Blocks:** Tasks 07, 09, 10 and 11

## Context and user value

GixGiz needs durable local state for setup, jobs, runtime/model metadata, settings, audits and later conversations. Persistence must be restart-safe without exposing SQLite directly to Flutter or future clients.

## Desired behavior

The Rust core opens a per-user database under `%LOCALAPPDATA%\GixGiz`, applies versioned transactional migrations and exposes persistence health through the core handshake.

## In scope

- Rust persistence crate and repository abstractions.
- Deterministic application-data path resolution.
- Initial schema for application metadata, settings, durable jobs and append-oriented audit events.
- Migration ledger and startup migration runner.
- WAL mode, foreign keys, busy timeout and connection-ownership policy.
- Transaction helpers and typed persistence errors.
- Backup before any migration marked irreversible.
- Temporary-database integration tests and startup health integration.
- Sanitized diagnostics for permission, lock, migration and corruption failures.

## Out of scope

- Model binaries, downloaded artifacts or secrets in SQLite.
- Full runtime/model schemas beyond fields exercised by the foundation.
- Conversation feature implementation.
- Vector storage, encryption-at-rest promises or cloud sync.
- UI database access or generic SQL endpoint.

## Architecture constraints

- Only the Rust core opens the platform database.
- Repositories expose domain operations rather than raw SQL to application modules.
- Schema migrations are forward-only; recovery uses backup/restore rather than untested down migrations.
- Long content transformations are separate jobs, not startup migration work.

## Security and privacy constraints

- Resolve and validate the data root before opening files.
- Use per-user paths and restrictive feasible ACLs.
- Do not store credentials, bootstrap secrets or full private content in audit records.
- Never include SQL parameters or personal content in default logs.
- Treat corruption and integrity failures as explicit attention states; do not silently recreate and discard user data.

## Acceptance criteria

- [ ] AC-1: A fresh supported user environment creates the data directory and initial schema deterministically.
- [ ] AC-2: Reopening preserves committed settings, job metadata and audit records.
- [ ] AC-3: Failed migrations roll back without a partially advanced schema version.
- [ ] AC-4: Concurrent write pressure follows the documented WAL/busy policy and does not create multiple database owners.
- [ ] AC-5: Permission denial, locked database and integrity/corruption errors map to stable safe errors.
- [ ] AC-6: Persistence health appears in the real core handshake as ready, degraded or unavailable.
- [ ] AC-7: No model binary, secret or unrestricted private-content logging is introduced.
- [ ] AC-8: Migration tests cover fresh creation and every supported prior schema fixture.

## Required tests

- Fresh database creation and reopen.
- Transaction commit and rollback.
- Initial migration and migration-ledger integrity.
- Simulated migration failure.
- Permission denial and lock contention.
- Corrupt database detection and safe failure.
- Audit redaction behavior.
- Handshake readiness integration.

## Validation commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Also run Flutter tests if handshake state presentation changes.

## Documentation updates

- Database schema/migration guide.
- Data location and cleanup notes.
- Backup/recovery limitations.

## Completion evidence

Report the selected driver and licence, database path, PRAGMAs, migrations, schema tables, recovery behavior, test fixtures and exact validation results.