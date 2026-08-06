//! Future supervised sidecar composition boundary for the GixGiz desktop app.
//!
//! Task 03 intentionally starts and stops only an in-process platform core. It
//! exposes no transport, listener, FFI bridge, persistence, or system behavior.

#![forbid(unsafe_code)]

use gixgiz_core::{CoreError, OperationContext, PlatformCore, init_tracing};

fn run(context: &OperationContext) -> Result<(), CoreError> {
    let mut core = PlatformCore::new(Vec::new());
    let status = core.start(context)?;

    tracing::info!(
        application_id = status.application.application_id,
        version = status.application.version,
        protocol_version = status.application.protocol_version,
        schema_version = status.application.schema_version,
        readiness = ?status.readiness.status,
        "platform core foundation composed"
    );

    core.shutdown(context)
}

fn main() {
    if let Err(error) = init_tracing() {
        eprintln!("Unable to initialize diagnostics: {error}");
        std::process::exit(1);
    }

    let context = OperationContext::generated();
    if let Err(error) = run(&context) {
        eprintln!("{}", error.to_safe_payload(&context));
        std::process::exit(1);
    }
}
