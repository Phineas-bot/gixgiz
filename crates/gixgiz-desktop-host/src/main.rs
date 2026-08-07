//! Executable entry point for the supervised GixGiz core sidecar.

#![forbid(unsafe_code)]

use gixgiz_core::init_tracing;

#[tokio::main]
async fn main() {
    if let Err(error) = init_tracing() {
        eprintln!("Unable to initialize diagnostics: {error}");
        std::process::exit(1);
    }

    if let Err(error) = gixgiz_desktop_host::run_sidecar().await {
        eprintln!("Unable to run the platform core: {error}");
        std::process::exit(1);
    }
}
