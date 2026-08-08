use std::time::Duration;

use gixgiz_contracts::{
    APPLICATION_ID, CorrelationId, ErrorCategory, ReadinessStatus, RequestId, ServiceHealth,
    ServiceHealthStatus, ServiceRequirement,
};
use gixgiz_core::{
    CoreError, CoreLifecycle, OperationContext, PlatformCore, ServiceHealthSource,
    compose_readiness,
};

#[derive(Clone)]
struct FakeHealthSource {
    result: Result<ServiceHealth, CoreError>,
}

impl FakeHealthSource {
    fn healthy(service_id: &str, requirement: ServiceRequirement) -> Self {
        Self {
            result: Ok(health(
                service_id,
                requirement,
                ServiceHealthStatus::Healthy,
            )),
        }
    }
}

impl ServiceHealthSource for FakeHealthSource {
    fn health(&self, _context: &OperationContext) -> Result<ServiceHealth, CoreError> {
        self.result.clone()
    }
}

fn context() -> OperationContext {
    OperationContext::new(
        "00000000-0000-0000-0000-000000000001"
            .parse::<CorrelationId>()
            .expect("fixed correlation ID is valid"),
        "00000000-0000-0000-0000-000000000002"
            .parse::<RequestId>()
            .expect("fixed request ID is valid"),
    )
}

fn health(
    service_id: &str,
    requirement: ServiceRequirement,
    status: ServiceHealthStatus,
) -> ServiceHealth {
    ServiceHealth::new(service_id, service_id, requirement, status, None)
}

#[test]
fn readiness_is_ready_when_all_services_are_healthy() {
    let report = compose_readiness(vec![
        health(
            "mandatory",
            ServiceRequirement::Mandatory,
            ServiceHealthStatus::Healthy,
        ),
        health(
            "optional",
            ServiceRequirement::Optional,
            ServiceHealthStatus::Healthy,
        ),
    ]);

    assert_eq!(report.status, ReadinessStatus::Ready);
}

#[test]
fn optional_failure_degrades_instead_of_failing_readiness() {
    let report = compose_readiness(vec![
        health(
            "mandatory",
            ServiceRequirement::Mandatory,
            ServiceHealthStatus::Healthy,
        ),
        health(
            "optional",
            ServiceRequirement::Optional,
            ServiceHealthStatus::Failed,
        ),
    ]);

    assert_eq!(report.status, ReadinessStatus::Degraded);
}

#[test]
fn mandatory_degradation_is_reported_as_degraded() {
    let report = compose_readiness(vec![health(
        "mandatory",
        ServiceRequirement::Mandatory,
        ServiceHealthStatus::Degraded,
    )]);

    assert_eq!(report.status, ReadinessStatus::Degraded);
}

#[test]
fn mandatory_unavailability_blocks_readiness() {
    let report = compose_readiness(vec![health(
        "mandatory",
        ServiceRequirement::Mandatory,
        ServiceHealthStatus::Unavailable,
    )]);

    assert_eq!(report.status, ReadinessStatus::Unavailable);
}

#[test]
fn mandatory_failure_fails_readiness() {
    let report = compose_readiness(vec![health(
        "mandatory",
        ServiceRequirement::Mandatory,
        ServiceHealthStatus::Failed,
    )]);

    assert_eq!(report.status, ReadinessStatus::Failed);
}

#[test]
fn core_health_and_version_operation_is_deterministic() {
    let mut core = PlatformCore::new(vec![Box::new(FakeHealthSource::healthy(
        "platform_core",
        ServiceRequirement::Mandatory,
    ))]);
    let operation = context();

    let startup = core.start(&operation).expect("fake service starts");
    let observed = core
        .health_and_version(&operation)
        .expect("running core reports health");

    assert_eq!(startup, observed);
    assert_eq!(observed.application.application_id, APPLICATION_ID);
    assert_eq!(observed.readiness.status, ReadinessStatus::Ready);
    assert!(
        observed
            .readiness
            .services
            .iter()
            .any(|service| service.service_id == "platform_core")
    );
}

#[test]
fn startup_and_shutdown_have_explicit_states() {
    let mut core = PlatformCore::new(vec![Box::new(FakeHealthSource::healthy(
        "platform_core",
        ServiceRequirement::Mandatory,
    ))]);
    let operation = context();

    assert_eq!(core.lifecycle(), CoreLifecycle::Created);
    core.start(&operation).expect("fake service starts");
    assert_eq!(core.lifecycle(), CoreLifecycle::Running);
    core.shutdown(&operation).expect("running core stops");
    assert_eq!(core.lifecycle(), CoreLifecycle::Stopped);
    assert!(matches!(
        core.health_and_version(&operation),
        Err(CoreError::LifecycleConflict {
            operation: "health_and_version",
            state: CoreLifecycle::Stopped,
        })
    ));
}

#[test]
fn startup_failure_sets_failed_state_and_maps_to_safe_error() {
    let mut core = PlatformCore::new(vec![Box::new(FakeHealthSource {
        result: Err(CoreError::HealthCollection {
            service_id: "fake_service".to_owned(),
        }),
    })]);
    let operation = context();

    let error = core.start(&operation).expect_err("fake source fails");
    let payload = error.to_safe_payload(&operation);

    assert_eq!(core.lifecycle(), CoreLifecycle::Failed);
    assert_eq!(payload.category, ErrorCategory::Unavailable);
    assert_eq!(payload.code, "core.health_collection_failed");
    assert!(!payload.to_string().contains("fake_service"));
}

#[test]
fn operation_context_enforces_cancellation_and_timeout_conventions() {
    let cancelled = context();
    cancelled.cancellation().cancel();
    let timed_out = context().with_timeout(Duration::ZERO);

    assert_eq!(cancelled.check(), Err(CoreError::Cancelled));
    assert_eq!(timed_out.check(), Err(CoreError::TimedOut));
}

#[test]
fn persistence_health_makes_a_temporary_core_ready() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let mut core = PlatformCore::with_persistence_root(temporary.path().join("data-root"));

    let status = core.start(&context()).expect("core startup reports health");

    assert_eq!(status.readiness.status, ReadinessStatus::Ready);
    let persistence = status
        .readiness
        .services
        .iter()
        .find(|service| service.service_id == "persistence")
        .expect("persistence health is present");
    assert_eq!(persistence.requirement, ServiceRequirement::Mandatory);
    assert_eq!(persistence.status, ServiceHealthStatus::Healthy);
}

#[test]
fn persistence_open_failure_makes_core_unavailable_without_raw_path() {
    let temporary = tempfile::tempdir().expect("temporary directory is available");
    let invalid_root = temporary.path().join("not-a-directory");
    std::fs::write(&invalid_root, b"fixture").expect("fixture file writes");
    let mut core = PlatformCore::with_persistence_root(&invalid_root);

    let status = core.start(&context()).expect("core startup reports health");

    assert_eq!(status.readiness.status, ReadinessStatus::Unavailable);
    let persistence = status
        .readiness
        .services
        .iter()
        .find(|service| service.service_id == "persistence")
        .expect("persistence health is present");
    assert_eq!(persistence.status, ServiceHealthStatus::Unavailable);
    assert!(
        !persistence
            .message
            .as_deref()
            .unwrap_or_default()
            .contains("not-a-directory")
    );
}
