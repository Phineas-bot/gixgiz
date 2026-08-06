use gixgiz_contracts::{ErrorCategory, RecoveryAction, RecoveryGuidance, SafeErrorPayload};
use thiserror::Error;

use crate::{CoreLifecycle, OperationContext};

/// Typed failures produced by the Task 03 platform-core foundation.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CoreError {
    /// Cooperative cancellation was observed.
    #[error("the operation was cancelled")]
    Cancelled,
    /// The operation deadline elapsed.
    #[error("the operation timed out")]
    TimedOut,
    /// A lifecycle transition was requested from an incompatible state.
    #[error("the requested lifecycle operation is not valid in the current state")]
    LifecycleConflict {
        /// Stable name of the requested lifecycle operation.
        operation: &'static str,
        /// Lifecycle state observed when the request was rejected.
        state: CoreLifecycle,
    },
    /// A service-health dependency failed to return safe evidence.
    #[error("service health collection failed")]
    HealthCollection {
        /// Stable internal identifier of the failing service.
        service_id: String,
    },
}

impl CoreError {
    /// Maps internal failures to stable payloads that contain no raw causes.
    #[must_use]
    pub fn to_safe_payload(&self, context: &OperationContext) -> SafeErrorPayload {
        let (category, code, message, recovery) = match self {
            Self::Cancelled => (
                ErrorCategory::Cancelled,
                "core.operation_cancelled",
                "The operation was cancelled.",
                RecoveryGuidance {
                    action: RecoveryAction::NoAction,
                    message: "No further action is required.".to_owned(),
                },
            ),
            Self::TimedOut => (
                ErrorCategory::TimedOut,
                "core.operation_timed_out",
                "The platform core did not finish the operation in time.",
                RecoveryGuidance {
                    action: RecoveryAction::Retry,
                    message: "Retry the operation.".to_owned(),
                },
            ),
            Self::LifecycleConflict { .. } => (
                ErrorCategory::Conflict,
                "core.lifecycle_conflict",
                "The platform core cannot perform that operation in its current state.",
                RecoveryGuidance {
                    action: RecoveryAction::Restart,
                    message: "Restart the platform core and retry.".to_owned(),
                },
            ),
            Self::HealthCollection { .. } => (
                ErrorCategory::Unavailable,
                "core.health_collection_failed",
                "The platform core could not collect required service health.",
                RecoveryGuidance {
                    action: RecoveryAction::Retry,
                    message: "Retry the health check.".to_owned(),
                },
            ),
        };

        SafeErrorPayload::new(
            category,
            code,
            message,
            recovery,
            context.correlation_id(),
            context.request_id(),
        )
    }
}
