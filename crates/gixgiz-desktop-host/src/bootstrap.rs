use std::{fmt, sync::Arc, time::Duration};

use gixgiz_contracts::BootstrapRequest;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::HostError;

const BOOTSTRAP_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_BOOTSTRAP_BYTES: usize = 8 * 1024;
const TOKEN_HEX_LENGTH: usize = 64;

#[derive(Clone)]
pub(crate) struct BearerToken(Arc<str>);

impl BearerToken {
    pub(crate) fn parse(value: String) -> Result<Self, HostError> {
        if value.len() != TOKEN_HEX_LENGTH || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(HostError::InvalidBootstrap);
        }
        Ok(Self(Arc::from(value)))
    }

    pub(crate) fn matches_authorization(&self, value: &str) -> bool {
        let Some(candidate) = value.strip_prefix("Bearer ") else {
            return false;
        };
        constant_time_eq(self.0.as_bytes(), candidate.as_bytes())
    }
}

impl fmt::Debug for BearerToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BearerToken([REDACTED])")
    }
}

pub(crate) struct BootstrapSession<R> {
    pub(crate) token: BearerToken,
    reader: BufReader<R>,
}

impl<R> BootstrapSession<R>
where
    R: AsyncRead + Unpin,
{
    pub(crate) async fn monitor_supervisor(mut self) {
        let mut buffer = [0_u8; 64];
        loop {
            match self.reader.read(&mut buffer).await {
                Ok(0) | Err(_) => return,
                Ok(_) => {}
            }
        }
    }
}

pub(crate) async fn read_bootstrap<R>(
    mut reader: BufReader<R>,
) -> Result<BootstrapSession<R>, HostError>
where
    R: AsyncRead + Unpin,
{
    let record = tokio::time::timeout(BOOTSTRAP_TIMEOUT, read_bounded_record(&mut reader))
        .await
        .map_err(|_| HostError::BootstrapTimedOut)??;
    let request: BootstrapRequest =
        serde_json::from_slice(&record).map_err(|_| HostError::InvalidBootstrap)?;
    validate_request(&request)?;
    let token = BearerToken::parse(request.bearer_token)?;

    Ok(BootstrapSession { token, reader })
}

async fn read_bounded_record<R>(reader: &mut R) -> Result<Vec<u8>, HostError>
where
    R: AsyncRead + Unpin,
{
    let mut record = Vec::with_capacity(512);
    loop {
        let byte = reader
            .read_u8()
            .await
            .map_err(|_| HostError::BootstrapClosed)?;
        if byte == b'\n' {
            return if record.is_empty() {
                Err(HostError::InvalidBootstrap)
            } else {
                Ok(record)
            };
        }
        if record.len() >= MAX_BOOTSTRAP_BYTES {
            return Err(HostError::BootstrapTooLarge);
        }
        record.push(byte);
    }
}

fn validate_request(request: &BootstrapRequest) -> Result<(), HostError> {
    if request.protocol_min == 0
        || request.protocol_min > request.protocol_max
        || request.supervisor_process_id == 0
    {
        return Err(HostError::InvalidBootstrap);
    }
    Ok(())
}

fn constant_time_eq(expected: &[u8], candidate: &[u8]) -> bool {
    let mut difference = expected.len() ^ candidate.len();
    for index in 0..expected.len().max(candidate.len()) {
        let left = expected.get(index).copied().unwrap_or_default();
        let right = candidate.get(index).copied().unwrap_or_default();
        difference |= usize::from(left ^ right);
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use gixgiz_contracts::SupervisionNonce;
    use tokio::io::BufReader;

    use super::*;

    fn request(token: &str) -> BootstrapRequest {
        BootstrapRequest {
            bearer_token: token.to_owned(),
            protocol_min: 1,
            protocol_max: 1,
            supervisor_process_id: 42,
            supervision_nonce: SupervisionNonce::new(),
        }
    }

    #[tokio::test]
    async fn valid_bootstrap_token_is_read_once_and_redacted() {
        let token = "ab".repeat(32);
        let input = format!(
            "{}\n",
            serde_json::to_string(&request(&token)).expect("request serializes")
        );
        let session = read_bootstrap(BufReader::new(input.as_bytes()))
            .await
            .expect("valid bootstrap is accepted");

        assert!(
            session
                .token
                .matches_authorization(&format!("Bearer {token}"))
        );
        assert!(!format!("{:?}", session.token).contains(&token));
    }

    #[tokio::test]
    async fn oversized_or_invalid_bootstrap_fails_closed() {
        let oversized = vec![b'a'; MAX_BOOTSTRAP_BYTES + 1];
        let invalid = format!(
            "{}\n",
            serde_json::to_string(&request("weak")).expect("request serializes")
        );

        assert!(matches!(
            read_bootstrap(BufReader::new(oversized.as_slice())).await,
            Err(HostError::BootstrapTooLarge | HostError::BootstrapClosed)
        ));
        assert!(matches!(
            read_bootstrap(BufReader::new(invalid.as_bytes())).await,
            Err(HostError::InvalidBootstrap)
        ));
    }

    #[test]
    fn bearer_comparison_rejects_missing_and_wrong_values() {
        let token = BearerToken::parse("ab".repeat(32)).expect("fixed token is valid");

        assert!(!token.matches_authorization(""));
        assert!(!token.matches_authorization(&format!("Bearer {}", "cd".repeat(32))));
        assert!(token.matches_authorization(&format!("Bearer {}", "ab".repeat(32))));
    }
}
