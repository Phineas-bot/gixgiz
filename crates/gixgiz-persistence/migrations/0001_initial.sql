CREATE TABLE schema_migrations (
    version INTEGER PRIMARY KEY CHECK (version > 0),
    name TEXT NOT NULL UNIQUE CHECK (length(name) BETWEEN 1 AND 128),
    applied_at_unix_ms INTEGER NOT NULL CHECK (applied_at_unix_ms >= 0)
);

CREATE TABLE application_metadata (
    key TEXT PRIMARY KEY CHECK (length(key) BETWEEN 1 AND 128),
    value TEXT NOT NULL CHECK (length(CAST(value AS BLOB)) <= 4096),
    updated_at_unix_ms INTEGER NOT NULL CHECK (updated_at_unix_ms >= 0)
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY CHECK (length(key) BETWEEN 1 AND 128),
    value TEXT NOT NULL CHECK (length(CAST(value AS BLOB)) <= 4096),
    updated_at_unix_ms INTEGER NOT NULL CHECK (updated_at_unix_ms >= 0)
);

CREATE TABLE durable_jobs (
    id TEXT PRIMARY KEY CHECK (length(id) BETWEEN 1 AND 128),
    kind TEXT NOT NULL CHECK (length(kind) BETWEEN 1 AND 128),
    state TEXT NOT NULL CHECK (
        state IN ('queued', 'running', 'succeeded', 'failed', 'cancelled')
    ),
    updated_at_unix_ms INTEGER NOT NULL CHECK (updated_at_unix_ms >= 0)
);

CREATE TABLE audit_events (
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    category TEXT NOT NULL CHECK (length(category) BETWEEN 1 AND 64),
    action TEXT NOT NULL CHECK (length(action) BETWEEN 1 AND 128),
    outcome TEXT NOT NULL CHECK (length(outcome) BETWEEN 1 AND 64),
    correlation_id TEXT NOT NULL CHECK (length(correlation_id) = 36),
    occurred_at_unix_ms INTEGER NOT NULL CHECK (occurred_at_unix_ms >= 0)
);

CREATE INDEX audit_events_occurred_at_idx
    ON audit_events (occurred_at_unix_ms, id);
