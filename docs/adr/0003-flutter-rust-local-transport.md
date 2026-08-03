# ADR 0003: Flutter–Rust local transport

- **Status:** Accepted
- **Date:** 2026-08-03
- **Owners:** GixGiz project owner
- **Decision scope:** Desktop-to-core boundary; required before Task 04
- **Related:** ADR 0001, ADR 0002, Architecture section 3, GitHub issue #4

## Context

The Flutter desktop shell and Rust core run as separate processes under ADR 0001. They need a typed, local, secure and versioned communication boundary that supports short commands, health checks, long-running job progress, streamed model output, cancellation and future additional local clients.

A native FFI bridge would tightly couple the UI process to the core lifecycle. A plain unauthenticated loopback HTTP API would be easy to inspect but unsafe because arbitrary local processes or browser pages could attempt to call privileged operations. The first implementation must remain understandable and testable without prematurely building the complete public gateway.

## Decision

GixGiz will use a **supervised loopback-only local API** between Flutter and the Rust core sidecar.

### 1. Transport shape

- The core binds to an operating-system-assigned port on `127.0.0.1` only.
- It must never bind to `0.0.0.0`, a LAN address or an externally reachable interface by default.
- Request/response commands use a typed HTTP API.
- Ordered asynchronous events use one authenticated streaming channel. The initial implementation may use WebSocket or server-sent events according to the accepted Task 04 design, provided bidirectional cancellation remains explicit.
- This boundary is internal to the installed desktop product. It is not the future public OpenAI-compatible or Pack gateway.

### 2. Secure bootstrap

The desktop starts the core with a minimal bootstrap channel that does not expose secrets in command-line arguments.

The chosen implementation must use one of these Windows-safe approaches:

1. inherited anonymous pipe carrying the bootstrap payload; or
2. a temporary per-user file with restrictive ACLs, read once and deleted immediately.

The bootstrap payload contains:

- a cryptographically random per-launch bearer token;
- requested protocol version range;
- parent process identity or supervision nonce;
- optional application data root handle/configuration reference.

The token must not be written to normal logs, SQLite, crash reports or process command lines.

### 3. Authentication and origin control

- Every request and stream connection requires the per-launch token.
- Authentication occurs before request-body processing for privileged operations.
- Browser CORS is disabled. No wildcard origins are allowed.
- Browser cookies are not used for authentication.
- The internal API rejects requests with unexpected browser-origin headers unless a later explicit client-registration design allows them.
- Requests are size-limited, rate-bounded and assigned correlation IDs.
- Tokens expire when the core exits and are not reused across launches.

### 4. Handshake and compatibility

The first call is a typed handshake containing at least:

```text
ClientHello {
  client_name,
  client_version,
  protocol_min,
  protocol_max,
  requested_capabilities,
  correlation_id
}

CoreHello {
  core_version,
  selected_protocol,
  supported_capabilities,
  readiness,
  instance_id,
  correlation_id
}
```

- No normal command is accepted before a successful handshake.
- The selected protocol must be within both supported ranges.
- Incompatible versions fail clearly and never degrade into undefined behavior.
- Optional unsupported capabilities are reported explicitly.
- Public fields are evolved additively; fields are never silently repurposed.

### 5. Authoritative contracts

- Rust types and a machine-readable schema generated from the `gixgiz-contracts` crate are the source of truth.
- Dart client models are generated where practical.
- Generated files are marked and not hand-edited.
- CI must later verify that generated bindings match the authoritative schema.
- Provider-specific names, commands, paths and payloads must not enter the desktop-core contract.

### 6. Command and event semantics

Commands carry:

- schema/protocol version;
- correlation ID;
- operation ID or idempotency key where retry can create effects;
- bounded payload;
- cancellation relationship where relevant.

Events carry:

- job/session identifier;
- monotonically increasing sequence number within the stream scope;
- event kind and version;
- timestamp;
- progress or output payload;
- terminal state where applicable.

The UI must be able to reconnect and query authoritative state. It must not assume that receiving every transient event is required to reconstruct a durable job.

### 7. Errors

Boundary errors contain:

- stable machine-readable code;
- safe user-facing message;
- correlation ID;
- retry or recovery guidance where available;
- optional sanitized technical detail.

Raw Rust panics, database errors, provider responses and stack traces are never the sole API response.

### 8. Cancellation and timeouts

- Long operations return a job or session identifier rather than blocking one request indefinitely.
- Cancellation is an explicit authenticated command.
- Cancellation propagates through application service, provider adapter, process/download operation and event stream where supported.
- The terminal result distinguishes `Cancelled`, `Failed`, `TimedOut` and successful completion.
- Client disconnect alone must not be interpreted as authorization to abandon or repeat a system-changing operation.

### 9. Supervision and reconnect

- Flutter launches or discovers the one compatible per-user core instance.
- The desktop waits for a bounded bootstrap/readiness interval.
- If startup fails, it shows an actionable unavailable or incompatible state.
- The desktop may reconnect to the same authenticated core during the launch session.
- A core restart requires a new token and handshake.
- The core monitors the parent/supervisor relationship and follows ADR 0001 orphan-shutdown behavior.

## Alternatives considered

### `flutter_rust_bridge` or direct FFI

Rejected for the primary boundary. It reduces network concerns but ties core lifetime to Flutter, complicates future external clients and introduces generated/native ABI and `unsafe` review obligations.

### gRPC over loopback

Deferred. It offers strong contracts and streaming but adds code generation, HTTP/2 and packaging complexity beyond what the initial internal API requires.

### Named pipes as the complete API

Not selected initially. Named pipes provide strong local-only semantics, but cross-language tooling, debugging and future client compatibility are less straightforward. A pipe remains appropriate for secret bootstrap.

### Unauthenticated loopback HTTP

Rejected. Loopback is not an authentication boundary; local malware, unrelated processes or browser-origin attacks could invoke the service.

### Fixed port and long-lived token

Rejected. Fixed ports create collision and discovery problems, while persistent shared tokens increase replay and secret-storage risk.

## Consequences

### Positive

- Separates UI and core lifecycles while preserving a clear typed boundary.
- Supports commands, streaming, cancellation and future local clients.
- Loopback-only binding avoids firewall prompts and LAN exposure when implemented correctly.
- Per-launch credentials reduce persistent secret management.
- HTTP-based contracts are straightforward to test with fake clients and servers.

### Negative

- Requires local authentication, process bootstrap and origin defenses.
- Adds generated schema/client maintenance.
- Requires explicit event ordering, reconnect and version compatibility rules.
- Local processes with sufficient user-level privileges may still inspect another process; this is not a sandbox against a fully compromised account.

## Security and privacy impact

- The API is inaccessible without a per-launch secret and successful handshake.
- Secrets are excluded from arguments, logs and persistence.
- Loopback binding and disabled CORS reduce remote and browser-origin attack surfaces.
- Payload, stream, request-rate and timeout limits reduce local denial-of-service risk.
- File access uses future approved handles/scopes; unrestricted UI-supplied paths are not inherently trusted.
- This transport does not authorize operations by itself. Application policy and permission checks still run at execution boundaries.

## Implementation constraints

- Task 04 implements only the minimum handshake, health/readiness request, stable error mapping and transport tests.
- Hardware scanning, SQLite behavior, Ollama, downloads and chat remain out of scope for Task 04.
- The first transport implementation must include fake-core/fake-client tests.
- No public external-client promise is made by the internal route structure.
- The Flutter app must depend on a `CoreClient` abstraction so widget tests do not require a real core.

## Follow-up work

- Select WebSocket versus server-sent events during Task 04 based on the minimum tested streaming and cancellation needs.
- Add generated-binding drift checks to Task 06 CI.
- Define public client registration and stable gateway APIs in a later ADR before IDE or third-party clients are supported.
- Perform a security review covering token leakage, origin bypass, oversized payloads, replay and orphan-core behavior.