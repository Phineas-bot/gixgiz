# Task 04: Establish typed Flutter–Rust communication

- **GitHub issue:** [#4](https://github.com/Phineas-bot/gixgiz/issues/4)
- **Depends on:** Tasks 02 and 03
- **Primary ADRs:** 0001, 0002, 0003, 0005
- **Blocks:** Tasks 05, 07 and 11

## Context and user value

The desktop shell must obtain authoritative state from the Rust core without exposing provider details or trusting unauthenticated local callers. This task proves the complete desktop-to-core boundary using a small health handshake.

## Desired behavior

Flutter launches or connects to the bundled `gixgiz-core.exe`, authenticates using the approved per-launch bootstrap, negotiates protocol compatibility and displays the real Rust version/readiness state. Requests, errors and streamed events use one versioned contract source.

## In scope

- Supervised core-sidecar startup and bounded shutdown.
- Loopback-only local endpoint selected dynamically.
- Secure per-launch bootstrap secret passed without command-line or log exposure.
- Typed health/version request and response.
- Protocol and capability handshake.
- Correlation IDs, bounded payloads, timeouts and cancellation foundation.
- Event-stream transport foundation for later progress/chat use.
- One authoritative schema source and reproducible Dart bindings/client generation.
- Error mapping to Flutter presentation states.
- Contract, integration and widget tests.

## Out of scope

- SQLite.
- Hardware scanning.
- Ollama, models and chat semantics.
- Public local API, browser access or external-client registration.
- Permanent Windows service.

## Architecture constraints

- Follow ADR 0003 exactly.
- Flutter communicates through a `CoreClient` abstraction and never spawns arbitrary commands.
- Provider and OS implementation details do not cross the transport boundary.
- Generated files are marked and never hand-edited.
- Core remains the authority for readiness.

## Security and privacy constraints

- Bind only to `127.0.0.1`; no LAN exposure.
- Reject requests without the exact per-launch secret.
- No wildcard CORS or browser-origin access.
- Do not place secrets in process arguments, URLs, logs, database or crash text.
- Limit body size, stream size, time and concurrent requests.
- Validate content type and contract version before deserialization.

## Acceptance criteria

- [ ] AC-1: Flutter displays the actual Rust application, protocol and readiness information.
- [ ] AC-2: Missing core, startup timeout and connection loss produce distinct recoverable states.
- [ ] AC-3: Missing or incorrect authentication is rejected and tested.
- [ ] AC-4: Protocol incompatibility fails closed with an explicit user-facing state.
- [ ] AC-5: Every request and error carries a correlation ID.
- [ ] AC-6: Cancellation and ordered event delivery are demonstrated with a deterministic test operation.
- [ ] AC-7: Binding generation is reproducible and CI-checkable.
- [ ] AC-8: Normal desktop shutdown requests bounded core shutdown without leaving an unbounded orphan.

## Required tests

- Successful authenticated handshake.
- Missing/invalid secret.
- Incompatible protocol version.
- Core unavailable and startup timeout.
- Mid-request disconnect and reconnect.
- Ordered event delivery and cancellation.
- Flutter widget states backed by fake and real test clients.
- Generated-binding drift check.

## Validation commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd apps/desktop
flutter analyze
flutter test
flutter build windows
```

Run the repository binding-generation/check command once it is introduced.

## Documentation updates

- Transport developer guide.
- Contract/schema ownership documentation.
- Core and desktop nested instructions if transport-specific rules are needed.

## Completion evidence

Report endpoint binding, bootstrap mechanism, schema source, generated files, tested failure modes, commands and any Windows process-lifecycle limitation.