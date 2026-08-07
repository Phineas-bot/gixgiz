# AGENTS.md - Rust workspace policy

> Applies to all crates under `crates/` in addition to the repository root policy.

## Ownership and dependency direction

- `gixgiz-contracts` owns versioned, provider-neutral data shared across process boundaries. It must not depend on the core or host.
- `gixgiz-core` owns platform lifecycle, readiness policy, typed failures, and operation conventions. It may depend on contracts, but not on the desktop host or Flutter.
- `gixgiz-desktop-host` owns the supervised sidecar process bootstrap and authenticated loopback transport. It may depend on core and contracts, but must not contain platform policy.
- Keep dependency direction `gixgiz-desktop-host -> gixgiz-core -> gixgiz-contracts`.

## Transport boundaries

- Bind internal HTTP only to `127.0.0.1` on a dynamic port and authenticate before processing privileged bodies.
- Keep per-launch bootstrap tokens out of arguments, URLs, logs, persistence, and error text.
- Rust contracts and the generated JSON Schema are authoritative. Regenerate checked Dart bindings instead of editing them.
- Keep provider names, paths, persistence, hardware scanning, runtime providers, model management, downloads, chat, and installer behavior out of the Task 04 transport.
- Do not add `unsafe` Rust.
- Tests may bind ephemeral loopback listeners, but must not use external network services or user data.
- Logs and boundary errors may include stable identifiers, state, and version metadata, but not secrets, prompts, private content, or raw provider errors.

## Validation

Run from the repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p gixgiz-contracts --example generate_bindings -- --check
```
