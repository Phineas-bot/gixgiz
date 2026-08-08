//! Authenticated loopback composition boundary for the GixGiz desktop sidecar.
//!
//! This crate owns only process bootstrap, supervision, and transport
//! adaptation. Platform policy remains in `gixgiz-core`, while serialized
//! boundary types remain in `gixgiz-contracts`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod bootstrap;
mod error;
mod operations;
mod server;

use std::time::Duration;

use gixgiz_contracts::{BootstrapReady, InstanceId};
use gixgiz_core::{OperationContext, PlatformCore};
use tokio::io::{AsyncWriteExt, BufReader};

pub use error::HostError;

use crate::{bootstrap::read_bootstrap, server::SidecarHost};

const CORE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(3);

/// Reads the inherited bootstrap pipe, runs the loopback host, and shuts the
/// core down within a fixed deadline.
pub async fn run_sidecar() -> Result<(), HostError> {
    let stdin = BufReader::new(tokio::io::stdin());
    let bootstrap = read_bootstrap(stdin).await?;
    let instance_id = InstanceId::new();
    let context = OperationContext::generated();
    let (mut core, status) = tokio::task::spawn_blocking(move || {
        let mut core = PlatformCore::with_default_persistence();
        let status = core.start(&context)?;
        Ok::<_, gixgiz_core::CoreError>((core, status))
    })
    .await
    .map_err(HostError::CoreWorker)?
    .map_err(HostError::Core)?;

    let serve_result = async {
        let host = SidecarHost::bind(bootstrap.token.clone(), instance_id, status).await?;
        let ready = BootstrapReady::new(
            host.local_addr().port(),
            instance_id,
            gixgiz_contracts::PROTOCOL_VERSION,
            gixgiz_contracts::PROTOCOL_VERSION,
        );
        write_bootstrap_ready(&ready).await?;

        let shutdown = host.shutdown_handle();
        tokio::spawn(async move {
            bootstrap.monitor_supervisor().await;
            shutdown.request();
        });

        host.run().await
    }
    .await;
    let shutdown_context = OperationContext::generated().with_timeout(CORE_SHUTDOWN_TIMEOUT);
    let core_result = tokio::task::spawn_blocking(move || core.shutdown(&shutdown_context))
        .await
        .map_err(HostError::CoreWorker)
        .and_then(|result| result.map_err(HostError::Core));

    serve_result.and(core_result)
}

async fn write_bootstrap_ready(ready: &BootstrapReady) -> Result<(), HostError> {
    let mut output = serde_json::to_vec(ready).map_err(|_| HostError::BootstrapResponse)?;
    output.push(b'\n');
    let mut stdout = tokio::io::stdout();
    stdout
        .write_all(&output)
        .await
        .map_err(|_| HostError::BootstrapResponse)?;
    stdout
        .flush()
        .await
        .map_err(|_| HostError::BootstrapResponse)
}
