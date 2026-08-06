# AGENTS.md - Rust workspace policy

> Applies to all crates under `crates/` in addition to the repository root policy.

## Ownership and dependency direction

- `gixgiz-contracts` owns versioned, provider-neutral data shared across process boundaries. It must not depend on the core or host.
- `gixgiz-core` owns platform lifecycle, readiness policy, typed failures, and operation conventions. It may depend on contracts, but not on the desktop host or Flutter.
- `gixgiz-desktop-host` is the composition boundary for the future supervised sidecar. It may depend on core and contracts, but must not contain platform policy.
- Keep dependency direction `gixgiz-desktop-host -> gixgiz-core -> gixgiz-contracts`.

## Task 03 boundaries

- Do not add transport, listeners, FFI, persistence, hardware scanning, runtime providers, model management, downloads, chat, or installer behavior.
- Do not add `unsafe` Rust.
- Tests must use deterministic fakes and must not use live network services or user data.
- Logs and boundary errors may include stable identifiers, state, and version metadata, but not secrets, prompts, private content, or raw provider errors.

## Validation

Run from the repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
