use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{CorrelationId, RequestId};

/// Stable failure taxonomy used across GixGiz process boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// Input failed validation.
    InvalidInput,
    /// The caller lacks permission for the requested operation.
    PermissionDenied,
    /// The requested capability is not supported.
    NotSupported,
    /// A required resource or service is currently unavailable.
    Unavailable,
    /// Current state conflicts with the requested operation.
    Conflict,
    /// A bounded resource is exhausted.
    ResourceExhausted,
    /// The operation was cancelled.
    Cancelled,
    /// The operation exceeded its deadline.
    TimedOut,
    /// Integrity verification failed.
    IntegrityFailure,
    /// A component or protocol version is incompatible.
    IncompatibleVersion,
    /// The operation completed with reduced capability.
    Degraded,
    /// An internal failure has no safer specific category.
    Internal,
    /// A newer peer supplied an unrecognized category.
    #[serde(other)]
    Unknown,
}

/// Recommended safe next action for a boundary failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    /// Retry the operation without changing configuration.
    Retry,
    /// Restart the affected GixGiz component.
    Restart,
    /// Review and correct platform prerequisites.
    CheckPrerequisites,
    /// Escalate using sanitized diagnostics.
    ContactSupport,
    /// No recovery action is necessary.
    NoAction,
    /// A newer peer supplied an unrecognized recovery action.
    #[serde(other)]
    Unknown,
}

/// Plain-language recovery information safe to show to a user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryGuidance {
    /// Stable action that a client may present or interpret.
    pub action: RecoveryAction,
    /// Safe plain-language explanation of the next step.
    pub message: String,
}

/// Error payload that excludes raw causes, private input, and provider output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SafeErrorPayload {
    /// Stable broad classification of the failure.
    pub category: ErrorCategory,
    /// Stable machine-readable error code.
    pub code: String,
    /// Safe user-facing error message.
    pub message: String,
    /// Recommended recovery action and explanation.
    pub recovery: RecoveryGuidance,
    /// Identifier shared by related operations and events.
    pub correlation_id: CorrelationId,
    /// Identifier unique to the failed request.
    pub request_id: RequestId,
}

impl SafeErrorPayload {
    /// Creates a boundary-safe payload from already sanitized values.
    #[must_use]
    pub fn new(
        category: ErrorCategory,
        code: impl Into<String>,
        message: impl Into<String>,
        recovery: RecoveryGuidance,
        correlation_id: CorrelationId,
        request_id: RequestId,
    ) -> Self {
        Self {
            category,
            code: code.into(),
            message: message.into(),
            recovery,
            correlation_id,
            request_id,
        }
    }
}

impl fmt::Display for SafeErrorPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} (correlation {})",
            self.code, self.message, self.correlation_id
        )
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn stable_category_and_safe_display_survive_serialization() {
        let payload = SafeErrorPayload::new(
            ErrorCategory::Unavailable,
            "core.service_unavailable",
            "A required platform service is unavailable.",
            RecoveryGuidance {
                action: RecoveryAction::Retry,
                message: "Retry the operation.".to_owned(),
            },
            CorrelationId::from_uuid(Uuid::nil()),
            RequestId::from_uuid(Uuid::from_u128(1)),
        );

        let json = serde_json::to_string(&payload).expect("serializes");
        let display = payload.to_string();

        assert!(json.contains("\"category\":\"unavailable\""));
        assert!(json.contains("core.service_unavailable"));
        assert_eq!(
            display,
            "core.service_unavailable: A required platform service is unavailable. (correlation 00000000-0000-0000-0000-000000000000)"
        );
        assert!(!display.contains("request"));
    }

    #[test]
    fn unknown_error_categories_are_forward_compatible() {
        let category: ErrorCategory =
            serde_json::from_str("\"future_category\"").expect("unknown category is accepted");

        assert_eq!(category, ErrorCategory::Unknown);
    }
}
