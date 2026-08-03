# ADR 0004: SQLite persistence and migrations

- **Status:** Accepted
- **Date:** 2026-08-03
- **Owners:** GixGiz project owner
- **Decision scope:** Local structured persistence; required before Task 05
- **Related:** ADR 0001, ADR 0003, Architecture section 10, Module Specifications section 1, GitHub issue #5

## Context

GixGiz needs durable local state for configuration, protocol/schema versions, setup progress, jobs, runtime and model metadata, audit events and future conversation history. The first release is single-device and local-first, so a network database would add deployment and security complexity without benefit.

Persistence must survive crashes and upgrades, remain private to the signed-in user, and support transactional migrations. Large model files, downloads and extracted content are not relational database payloads and require separate storage.

## Decision

GixGiz will use **SQLite** as the platform database, owned exclusively by the Rust core process.

### 1. Ownership and access

- Only `gixgiz-core.exe` opens the platform database.
- Flutter, provider processes, future Packs and external clients never access SQLite directly.
- Access occurs through typed repository/application interfaces.
- One writable database owner exists per user/application data root, enforced by the single-core policy in ADR 0001.

### 2. Location and files

The default per-user data root is:

```text
%LOCALAPPDATA%\GixGiz\
```

The platform database is stored under a versioned application-data subdirectory, for example:

```text
%LOCALAPPDATA%\GixGiz\data\gixgiz.db
```

The exact final directory layout is confirmed by ADR 0005 and implementation tests.

The database must not be placed in the installation directory. User-selected model/content storage may live elsewhere, but database records store only validated references and metadata.

### 3. Technology and configuration

- Use a maintained Rust SQLite library selected during Task 05 after dependency review.
- Enable foreign-key enforcement on every connection.
- Use WAL mode where supported and verified by tests.
- Set a bounded busy timeout.
- Use transactions for related writes and migrations.
- Keep connection count deliberately small because the core is the sole owner.
- Do not build SQL from untrusted string concatenation; use parameters and validated identifiers.

### 4. Data boundaries

SQLite stores structured records such as:

- schema and application versions;
- device-local settings and feature flags;
- machine profiles and evidence metadata;
- durable job state and checkpoints;
- runtime/model catalogue and installation metadata;
- conversation/session metadata and messages in later tasks;
- sanitized audit and diagnostic event metadata.

SQLite does not store:

- model binaries;
- partially downloaded artifacts;
- installer packages;
- plaintext secrets, API keys or bearer tokens;
- unrestricted diagnostic dumps;
- large extracted document/media blobs when content-file storage is more appropriate.

Secrets are represented by opaque references to an OS-backed secret store when that service is introduced.

### 5. Schema and migrations

- Every persisted schema has an explicit integer migration version.
- Migrations are immutable after release. Corrections require a new migration.
- Migrations run before the core reports full readiness.
- Each migration executes inside a transaction when SQLite supports the operations transactionally.
- A failed migration leaves the previous supported schema intact or restores a verified pre-migration backup.
- Irreversible or potentially lossy transformations require a recoverable snapshot before execution.
- Expensive content re-indexing or file conversion is not performed inside a blocking schema migration; it becomes a resumable post-migration job.
- Downgrade is not assumed to be safe. A newer unsupported schema must produce a clear `IncompatibleVersion` state rather than being opened destructively.

### 6. Compatibility policy

During prototype development, migrations must support the immediately previous committed development schema when practical. Before external alpha, the project will define an explicit supported upgrade window.

The core records at least:

- current schema version;
- application version that last migrated the database;
- migration timestamp and outcome;
- compatibility metadata needed for diagnostics.

Public serialized records evolve additively where practical. Unknown future fields must not be silently reinterpreted.

### 7. Startup health

Persistence startup distinguishes:

- healthy and writable;
- healthy but degraded/read-only where a safe limited mode is explicitly supported;
- permission denied;
- locked/busy beyond policy;
- corrupted or failed integrity check;
- schema too new/incompatible;
- migration failed with recovery available;
- unavailable path/storage.

A mandatory persistence failure blocks normal core readiness and returns a safe diagnostic through ADR 0003.

### 8. Backup, recovery and corruption

- Before an irreversible migration, create a verified snapshot using a SQLite-safe backup method rather than copying an actively written file blindly.
- Recovery never overwrites the only known-good database without preserving evidence.
- Integrity checks are bounded and used when corruption is suspected, not as an expensive unconditional startup ritual unless measurements justify it.
- Diagnostic bundles redact private content by default.
- Users receive a plain-language recovery path and the exact location of retained backups when appropriate.

### 9. Deletion and retention

- Deleting an owning entity must define treatment of dependent records and external content.
- Foreign keys and repository rules prevent orphaned metadata.
- Audit records are append-oriented and have an explicit future retention/export policy.
- Uninstall behavior is defined by ADR 0005; uninstall does not silently delete user data or model libraries.

## Alternatives considered

### JSON/TOML files only

Rejected as the primary store. They are useful for static defaults or export but do not provide safe concurrent transactions, migrations, relational integrity or durable job queries.

### PostgreSQL or another server database

Rejected. A server introduces installation, ports, credentials, updates and resource overhead that conflict with a single-user local-first desktop foundation.

### Embedded key-value database

Deferred. It could suit selected high-throughput domains, but SQLite provides mature transactions, inspection, migrations and relational queries for the current structured state.

### Flutter-owned SQLite

Rejected. It would move platform state into the presentation layer, complicate sidecar recovery and allow UI/provider coupling to storage schemas.

### One database per module

Rejected for the foundation. Multiple databases would complicate transactions, backup and migration ordering. Namespaced tables/repositories preserve modular ownership inside one platform database. Separate stores may later be justified for sandboxed Packs or high-volume indexes.

## Consequences

### Positive

- No database server or network configuration.
- Mature transactions, constraints, backups and migration support.
- Fits local-first privacy and offline operation.
- Supports durable workflows and restart recovery.
- Easy to test with temporary databases.

### Negative

- The core is the single database writer and must manage connection/transaction discipline.
- SQLite is not a multi-device synchronization solution.
- Large binary/content workloads require separate storage and lifecycle coordination.
- Migration and backup behavior become release-critical responsibilities.

## Security and privacy impact

- Database access is restricted to the current user and core process as far as Windows ACLs permit.
- Secrets and per-launch transport tokens are prohibited from SQLite.
- Queries use bound parameters and validated paths.
- Logs and diagnostics do not expose private rows by default.
- Backups inherit the same privacy classification and access restrictions as the primary database.
- User-selected paths are canonicalized and checked before use.

## Testing requirements

Task 05 must include deterministic tests for:

- fresh database initialization;
- closing and reopening with preserved state;
- foreign-key enforcement;
- migration from the previous fixture schema;
- migration rollback/failure behavior;
- permission-denied or unavailable path handling where reliably simulated;
- incompatible newer schema;
- transaction rollback after an injected failure;
- absence of secrets/private fixture data in logs.

Tests use temporary directories and databases. They do not depend on a live network or the user’s real application-data directory.

## Implementation constraints

- Task 05 introduces only the persistence foundation and minimal records needed by current foundation services.
- Do not pre-create schemas for every future Pack, tool or enterprise feature.
- No model binary or download staging data is written into SQLite.
- Flutter receives persistence health through core contracts, not through direct database plugins.
- Schema generation and migration commands must be documented and reproducible.

## Follow-up work

- Select the Rust SQLite library in Task 05 with licence, maintenance and binary-size review.
- Define alpha/beta migration support windows before external releases.
- Add conversation retention and deletion rules before Task 11.
- Evaluate an embedded vector index separately when the Knowledge/Document stage begins.
- Define OS-backed secret storage before any cloud/provider credentials are supported.