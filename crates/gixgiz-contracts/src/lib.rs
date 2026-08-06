//! Versioned, provider-neutral contracts shared by GixGiz processes.
//!
//! This crate contains data and stable boundary semantics only. It does not own
//! readiness policy, system access, persistence, transport, or provider logic.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod error;
mod identity;
mod ids;
mod readiness;

pub use error::{ErrorCategory, RecoveryAction, RecoveryGuidance, SafeErrorPayload};
pub use identity::{
    APPLICATION_ID, APPLICATION_NAME, APPLICATION_VERSION, ApplicationInfo, PROTOCOL_VERSION,
    PlatformStatus, SCHEMA_VERSION,
};
pub use ids::{CorrelationId, RequestId};
pub use readiness::{
    ReadinessReport, ReadinessStatus, ServiceHealth, ServiceHealthStatus, ServiceRequirement,
};
