use std::fmt;

use gixgiz_contracts::{
    ApplicationInfo, PlatformStatus, ReadinessReport, ReadinessStatus, ServiceHealth,
    ServiceHealthStatus, ServiceRequirement,
};

use crate::{CoreError, OperationContext};

/// Explicit in-process lifecycle of the Task 03 platform core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreLifecycle {
    /// The service has been constructed but not started.
    Created,
    /// Startup health collection is in progress.
    Starting,
    /// The service is running and can report health.
    Running,
    /// Shutdown is in progress.
    Stopping,
    /// The service completed shutdown.
    Stopped,
    /// Startup health collection failed.
    Failed,
}

impl fmt::Display for CoreLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

/// Provider-neutral dependency that supplies one service health record.
pub trait ServiceHealthSource: Send + Sync {
    /// Collects one safe service-health record for the current operation.
    fn health(&self, context: &OperationContext) -> Result<ServiceHealth, CoreError>;
}

/// Minimal platform service that owns lifecycle and readiness composition.
pub struct PlatformCore {
    lifecycle: CoreLifecycle,
    health_sources: Vec<Box<dyn ServiceHealthSource>>,
}

impl PlatformCore {
    /// Creates a core with provider-neutral health dependencies.
    #[must_use]
    pub fn new(health_sources: Vec<Box<dyn ServiceHealthSource>>) -> Self {
        Self {
            lifecycle: CoreLifecycle::Created,
            health_sources,
        }
    }

    /// Returns the current explicit lifecycle state.
    #[must_use]
    pub const fn lifecycle(&self) -> CoreLifecycle {
        self.lifecycle
    }

    /// Returns compiled application and protocol version information.
    #[must_use]
    pub fn version(&self) -> ApplicationInfo {
        ApplicationInfo::current()
    }

    /// Starts the core, verifies health sources, and returns initial readiness.
    pub fn start(&mut self, context: &OperationContext) -> Result<PlatformStatus, CoreError> {
        context.check()?;

        match self.lifecycle {
            CoreLifecycle::Running => return self.collect_status(context),
            CoreLifecycle::Created | CoreLifecycle::Stopped | CoreLifecycle::Failed => {}
            CoreLifecycle::Starting | CoreLifecycle::Stopping => {
                return Err(CoreError::LifecycleConflict {
                    operation: "start",
                    state: self.lifecycle,
                });
            }
        }

        self.lifecycle = CoreLifecycle::Starting;
        match self.collect_status(context) {
            Ok(status) => {
                self.lifecycle = CoreLifecycle::Running;
                tracing::info!(
                    correlation_id = %context.correlation_id(),
                    request_id = %context.request_id(),
                    lifecycle = "running",
                    "platform core started"
                );
                Ok(status)
            }
            Err(error) => {
                self.lifecycle = CoreLifecycle::Failed;
                Err(error)
            }
        }
    }

    /// Returns deterministic version and health information while running.
    pub fn health_and_version(
        &self,
        context: &OperationContext,
    ) -> Result<PlatformStatus, CoreError> {
        context.check()?;
        if self.lifecycle != CoreLifecycle::Running {
            return Err(CoreError::LifecycleConflict {
                operation: "health_and_version",
                state: self.lifecycle,
            });
        }

        self.collect_status(context)
    }

    /// Stops the core without performing external system work.
    pub fn shutdown(&mut self, context: &OperationContext) -> Result<(), CoreError> {
        context.check()?;

        match self.lifecycle {
            CoreLifecycle::Created => {
                self.lifecycle = CoreLifecycle::Stopped;
            }
            CoreLifecycle::Running | CoreLifecycle::Failed => {
                self.lifecycle = CoreLifecycle::Stopping;
                self.lifecycle = CoreLifecycle::Stopped;
                tracing::info!(
                    correlation_id = %context.correlation_id(),
                    request_id = %context.request_id(),
                    lifecycle = "stopped",
                    "platform core stopped"
                );
            }
            CoreLifecycle::Stopped => {}
            CoreLifecycle::Starting | CoreLifecycle::Stopping => {
                return Err(CoreError::LifecycleConflict {
                    operation: "shutdown",
                    state: self.lifecycle,
                });
            }
        }

        Ok(())
    }

    fn collect_status(&self, context: &OperationContext) -> Result<PlatformStatus, CoreError> {
        let mut services = vec![ServiceHealth::new(
            "platform_core",
            "Platform core",
            ServiceRequirement::Mandatory,
            ServiceHealthStatus::Healthy,
            Some("The platform core foundation is responsive.".to_owned()),
        )];
        let dependency_health = self
            .health_sources
            .iter()
            .map(|source| {
                context.check()?;
                source.health(context)
            })
            .collect::<Result<Vec<_>, _>>()?;
        services.extend(dependency_health);

        Ok(PlatformStatus {
            application: self.version(),
            readiness: compose_readiness(services),
        })
    }
}

/// Applies conservative, provider-neutral readiness precedence to service evidence.
#[must_use]
pub fn compose_readiness(services: Vec<ServiceHealth>) -> ReadinessReport {
    let has_mandatory_failure = services.iter().any(|service| {
        service.requirement.is_mandatory() && service.status == ServiceHealthStatus::Failed
    });
    let has_mandatory_unavailable = services.iter().any(|service| {
        service.requirement.is_mandatory()
            && matches!(
                service.status,
                ServiceHealthStatus::Unavailable | ServiceHealthStatus::Unknown
            )
    });
    let has_degraded_evidence = services
        .iter()
        .any(|service| service.status != ServiceHealthStatus::Healthy);

    let (status, summary) = if services.is_empty() {
        (
            ReadinessStatus::Unavailable,
            "No service health evidence is available.",
        )
    } else if has_mandatory_failure {
        (
            ReadinessStatus::Failed,
            "A mandatory platform service failed.",
        )
    } else if has_mandatory_unavailable {
        (
            ReadinessStatus::Unavailable,
            "A mandatory platform service is unavailable.",
        )
    } else if has_degraded_evidence {
        (
            ReadinessStatus::Degraded,
            "The platform core is available with reduced capability.",
        )
    } else {
        (
            ReadinessStatus::Ready,
            "All mandatory platform services are healthy.",
        )
    };

    ReadinessReport {
        status,
        summary: summary.to_owned(),
        services,
    }
}
