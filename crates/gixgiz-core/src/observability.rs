use thiserror::Error;

/// Safe initialization failure for the process-wide tracing subscriber.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TracingInitError {
    /// Another process component already installed a global subscriber.
    #[error("structured diagnostics were already initialized")]
    AlreadyInitialized,
}

/// Installs concise INFO-level structured diagnostics with no payload logging.
pub fn init_tracing() -> Result<(), TracingInitError> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| TracingInitError::AlreadyInitialized)
}
