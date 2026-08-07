//! Platform lifecycle and provider-neutral readiness policy for GixGiz.
//!
//! This crate owns core application behavior and consumes shared contracts. It
//! does not own transport, persistence, Flutter presentation, or provider and
//! operating-system integrations. The desktop host adapts these services to
//! the authenticated local boundary.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod error;
mod observability;
mod operation;
mod service;

pub use error::CoreError;
pub use observability::{TracingInitError, init_tracing};
pub use operation::{CancellationToken, OperationContext};
pub use service::{CoreLifecycle, PlatformCore, ServiceHealthSource, compose_readiness};
