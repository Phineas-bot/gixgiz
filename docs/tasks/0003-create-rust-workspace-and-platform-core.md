# Task 03: Create Rust workspace and platform core

- **GitHub issue:** [#3](https://github.com/Phineas-bot/gixgiz/issues/3)
- **Depends on:** Task 01
- **Primary ADRs:** 0001, 0002, 0005
- **Blocks:** Tasks 04–09

## Context and user value

The Rust core will own GixGiz orchestration, contracts, durable operations and system-facing behavior. This task creates only the minimal platform foundation required for later communication and persistence.

## Desired behavior

A pinned stable Rust workspace builds and exposes a tested in-process health/version operation using shared domain contracts. No Flutter, provider or database concern enters the core.

## In scope

- Root Cargo workspace and committed `Cargo.lock`.
- Pinned stable toolchain and required components.
- `crates/gixgiz-contracts` for versioned domain contracts.
- `crates/gixgiz-core` for application startup and readiness composition.
- `crates/gixgiz-desktop-host` as the future sidecar composition boundary, without transport implementation.
- Core version, protocol version, readiness and typed error contracts.
- Structured tracing foundation with redaction-safe defaults.
- Cancellation and timeout conventions.
- Unit tests and core-specific `AGENTS.md`.

## Out of scope

- Flutter integration or network listener.
- SQLite and migrations.
- Windows hardware queries.
- Ollama, model management or chat.
- Installer and privileged helper.

## Architecture constraints

- `gixgiz-core` must not import Flutter, Windows UI or provider-specific concerns.
- Contracts are explicit, versioned and serializable where crossing future boundaries.
- The desktop host is a thin outer composition layer, not a second domain core.
- Avoid speculative crates not exercised by this task.

## Security and privacy constraints

- No `unsafe` code without a separate accepted ADR.
- No secrets or private content in logs.
- No production-path panic, `unwrap` or `expect` except for documented proven invariants.
- No network or filesystem access beyond test-owned temporary paths.

## Acceptance criteria

- [ ] AC-1: The workspace builds on the pinned stable Rust baseline.
- [ ] AC-2: `gixgiz-contracts` defines application, protocol, readiness and error types.
- [ ] AC-3: `gixgiz-core` returns a deterministic tested health/version result.
- [ ] AC-4: Mandatory and optional service readiness can be represented without provider-specific types.
- [ ] AC-5: Structured errors include stable code, safe message, recovery guidance and correlation ID fields.
- [ ] AC-6: Formatting, Clippy with warnings denied and all workspace tests pass.
- [ ] AC-7: Crate responsibilities and dependency direction are documented.

## Required tests

- Version and protocol serialization tests.
- Ready, degraded and unavailable readiness composition tests.
- Error conversion and safe-display tests.
- Startup/shutdown state tests using fake dependencies.

## Validation commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Documentation updates

- Root README Rust commands and prerequisites.
- Core crate documentation.
- Nested `AGENTS.md` for Rust contracts/core.

## Completion evidence

Report the workspace/crate tree, dependency choices and licences, exact commands, test results, public contract surface and confirmation that no transport, persistence or provider implementation was added.