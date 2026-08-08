use std::{fs, time::Duration};

use gixgiz_contracts::{CorrelationId, ErrorCategory, RequestId};
use gixgiz_persistence::{
    CURRENT_SCHEMA_VERSION, DataRoot, JobId, JobMetadata, JobState, Persistence, PersistenceError,
    PersistenceOptions,
};
use rusqlite::{Connection, TransactionBehavior};

fn test_root(temporary: &tempfile::TempDir) -> DataRoot {
    DataRoot::from_override(temporary.path().join("gixgiz-test-root"))
        .expect("isolated test root initializes")
}

#[test]
fn fresh_database_is_configured_and_records_schema_version() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let persistence = Persistence::open(test_root(&temporary)).expect("fresh database opens");

    assert_eq!(
        persistence
            .health_check()
            .expect("health check succeeds")
            .schema_version,
        CURRENT_SCHEMA_VERSION
    );
    let configuration = persistence.configuration().expect("configuration reads");
    assert!(configuration.foreign_keys);
    assert_eq!(configuration.journal_mode, "wal");
    assert_eq!(configuration.busy_timeout_millis, 5_000);
    assert_eq!(
        persistence
            .platform_metadata()
            .get("last_migrated_application_version")
            .expect("migration application version reads")
            .as_deref(),
        Some(env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn reopening_preserves_settings_jobs_metadata_and_audit_events() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let root = test_root(&temporary);
    let correlation_id = CorrelationId::new();
    let job_id = JobId::new("job-1").expect("job ID is valid");
    {
        let persistence = Persistence::open(root.clone()).expect("database opens");
        persistence
            .platform_metadata()
            .set("installation_state", "initialized", 1)
            .expect("metadata writes");
        persistence
            .settings()
            .set("appearance", "system", 2)
            .expect("setting writes");
        persistence
            .jobs()
            .upsert(
                &JobMetadata::new(job_id.clone(), "foundation", JobState::Queued, 3)
                    .expect("job is valid"),
            )
            .expect("job writes");
        persistence
            .audit_events()
            .append(
                "platform",
                "database_initialized",
                "succeeded",
                correlation_id,
                4,
            )
            .expect("audit event writes");
    }

    let reopened = Persistence::open(root).expect("database reopens");
    assert_eq!(
        reopened
            .platform_metadata()
            .get("installation_state")
            .expect("metadata reads")
            .as_deref(),
        Some("initialized")
    );
    assert_eq!(
        reopened
            .settings()
            .get("appearance")
            .expect("setting reads")
            .as_deref(),
        Some("system")
    );
    assert_eq!(
        reopened.jobs().get(&job_id).expect("job reads"),
        Some(JobMetadata::new(job_id, "foundation", JobState::Queued, 3).expect("job is valid"))
    );
    let events = reopened
        .audit_events()
        .list_recent(100)
        .expect("audit events read");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].correlation_id, correlation_id);
}

#[test]
fn repository_updates_are_transactional_and_prepared() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let persistence = Persistence::open(test_root(&temporary)).expect("database opens");
    let id = JobId::generated();
    let jobs = persistence.jobs();
    jobs.upsert(
        &JobMetadata::new(id.clone(), "foundation", JobState::Queued, 1).expect("job is valid"),
    )
    .expect("job creates");
    jobs.upsert(
        &JobMetadata::new(id.clone(), "foundation", JobState::Running, 2).expect("job is valid"),
    )
    .expect("job updates");

    let observed = jobs.get(&id).expect("job reads").expect("job exists");
    assert_eq!(observed.state, JobState::Running);
    assert_eq!(observed.updated_at_unix_ms, 2);
}

#[test]
fn newer_schema_is_refused_without_downgrade() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let root = test_root(&temporary);
    let connection = Connection::open(root.database_path()).expect("database opens");
    connection
        .pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION + 1)
        .expect("newer version is written");
    drop(connection);

    let error = Persistence::open(root)
        .err()
        .expect("newer schema must be refused");
    assert!(matches!(
        error,
        PersistenceError::SchemaTooNew {
            found,
            supported
        } if found == CURRENT_SCHEMA_VERSION + 1 && supported == CURRENT_SCHEMA_VERSION
    ));
}

#[test]
fn a_second_platform_owner_is_refused() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let root = test_root(&temporary);
    let first = Persistence::open(root.clone()).expect("first owner opens");

    let error = Persistence::open(root)
        .err()
        .expect("second owner must fail");

    assert!(matches!(error, PersistenceError::AlreadyOwned { .. }));
    drop(first);
}

#[test]
fn external_write_contention_honors_the_bounded_busy_timeout() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let root = test_root(&temporary);
    let persistence = Persistence::open_with_options(
        root.clone(),
        PersistenceOptions::new(Duration::from_millis(25)),
    )
    .expect("database opens");
    let mut external = Connection::open(root.database_path()).expect("external connection opens");
    let transaction = external
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .expect("external writer acquires lock");

    let error = persistence
        .settings()
        .set("appearance", "system", 1)
        .expect_err("bounded write must time out while locked");

    assert!(matches!(&error, PersistenceError::Locked { .. }));
    let payload = error.to_safe_payload(CorrelationId::new(), RequestId::new());
    assert_eq!(payload.category, ErrorCategory::Unavailable);
    assert_eq!(payload.code, "persistence.locked");
    transaction.rollback().expect("external lock releases");
}

#[test]
fn corrupt_database_is_detected_and_safely_mapped() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let root = test_root(&temporary);
    fs::write(root.database_path(), b"not a sqlite database").expect("corrupt test fixture writes");

    let error = Persistence::open(root)
        .err()
        .expect("corrupt database must fail");
    let payload = error.to_safe_payload(CorrelationId::new(), RequestId::new());

    assert!(matches!(error, PersistenceError::Corrupt { .. }));
    assert_eq!(payload.category, ErrorCategory::IntegrityFailure);
    assert_eq!(payload.code, "persistence.corrupt");
    assert!(!payload.to_string().contains("sqlite"));
}

#[test]
fn foundation_rejects_secret_keys_large_values_and_private_audit_payloads() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let persistence = Persistence::open(test_root(&temporary)).expect("database opens");

    assert!(
        persistence
            .settings()
            .set("api_token", "do-not-store", 1)
            .is_err()
    );
    assert!(
        persistence
            .settings()
            .set("large_value", &"x".repeat(4097), 1)
            .is_err()
    );
    persistence
        .audit_events()
        .append(
            "platform",
            "safe_action",
            "succeeded",
            CorrelationId::new(),
            1,
        )
        .expect("categorical audit event is accepted");

    let connection = Connection::open(persistence.data_root().database_path())
        .expect("inspection connection opens");
    let audit_columns: Vec<String> = connection
        .prepare("PRAGMA table_info(audit_events)")
        .expect("schema statement prepares")
        .query_map([], |row| row.get(1))
        .expect("schema rows query")
        .collect::<Result<_, _>>()
        .expect("schema rows decode");
    assert!(!audit_columns.iter().any(|column| {
        matches!(
            column.as_str(),
            "message" | "payload" | "content" | "secret"
        )
    }));
}
