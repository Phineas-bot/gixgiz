# ADR 0001: Modular monolith and process topology

- **Status:** Accepted
- **Date:** 2026-08-03
- **Owners:** GixGiz project owner
- **Decision scope:** Foundation architecture; required before Tasks 02–05
- **Related:** Product Vision, Architecture, Module Specifications, Development Roadmap, GitHub issue #1

## Context

GixGiz is a local-first AI operating platform, not only a chat application. The complete platform is expected to coordinate hardware discovery, capability planning, runtime and model lifecycle, local inference, durable jobs, permissions, tools, AI Packs, integrations and future cloud or enterprise extensions.

The first release must remain small enough to build and operate reliably. A microservice architecture would impose deployment, networking, versioning and debugging costs that provide little value on one user’s computer. At the same time, placing all logic inside the Flutter process would couple presentation to systems work and make long-running downloads, installations and provider supervision vulnerable to UI crashes.

The architecture therefore needs one deployable product with strong internal boundaries and a process model that supports durable systems work without introducing a permanent background service in v0.1.

## Decision

GixGiz will begin as a **local-first modular monolith** delivered as one coordinated desktop product.

### 1. Logical architecture

The codebase is divided into modules with explicit contracts and downward dependency direction:

```text
Experience
Flutter desktop shell and future clients
        ↓
Application and orchestration
Setup workflow, jobs, sessions and coordination
        ↓
Domain contracts and policy
Machine, capability, runtime, model, errors and approvals
        ↓
Platform services
Persistence, downloads, diagnostics and lifecycle management
        ↓
System and provider adapters
Windows integration, filesystem, processes and Ollama
```

Modules are not separate network services. They compile into a small number of controlled binaries and communicate in-process through typed Rust interfaces, except for the desktop-to-core boundary defined in ADR 0003.

### 2. v0.1 process topology

The v0.1 installation contains the following process roles:

1. **`GixGiz.exe` — Flutter desktop shell**
   - Owns windows, navigation, accessibility, localization and transient presentation state.
   - Sends typed intentions to the core and renders authoritative core state.
   - Does not execute shell commands, access SQLite directly or call Ollama directly.

2. **`gixgiz-core.exe` — supervised Rust core sidecar**
   - Owns application orchestration, persistent state, long-running jobs, hardware providers, runtime adapters and diagnostics.
   - Is launched and supervised by the desktop shell in v0.1.
   - Is not installed as a permanent Windows service.
   - Exposes only the authenticated local transport specified by ADR 0003.

3. **Provider processes**
   - Ollama and future inference engines remain separate provider processes.
   - They are accessed only through runtime adapters.
   - Existing externally managed installations are detected and never silently adopted, updated or removed.

4. **Privileged installer helper — future foundation extension**
   - Privileged operations, when later required, run in a narrow short-lived helper.
   - The desktop shell and normal core process remain non-elevated.
   - The helper is not implemented by Tasks 02–05.

### 3. Core lifetime and single-instance policy

- The desktop shell starts the compatible bundled core during application startup.
- One managed core instance is allowed per signed-in Windows user and application data root.
- A per-user lock or named mutex prevents concurrent cores from writing to the same database.
- A second desktop instance connects to the existing compatible core or exits with a clear message; it must not start a competing database owner.
- The desktop and core perform a protocol/version handshake before the UI reports readiness.
- When the desktop exits normally, it requests bounded core shutdown. The core stops accepting new work, checkpoints resumable jobs and closes persistence cleanly.
- v0.1 does not promise that downloads continue after all desktop windows close. Interrupted durable jobs must resume safely or surface an exact recovery state on the next launch.
- If the desktop crashes, the core may remain briefly while supervision detects the lost parent, checkpoints work and exits. It must not become an unbounded orphan process.

### 4. State ownership

- Rust core state is authoritative for setup, jobs, runtime/model readiness, storage and diagnostics.
- Flutter may cache view state but must rehydrate from the core after reconnecting.
- SQLite is owned exclusively by the core process.
- External clients, future Packs and integrations use platform contracts; they never open the platform database or provider endpoints directly.

### 5. Extensibility rule

Future capabilities are added as modules and adapters behind stable contracts. A future decision may allow the core to run independently for IDE or CLI clients, but that evolution must preserve the same domain boundaries and must receive a new ADR before changing background-lifetime or local-authentication guarantees.

## Alternatives considered

### Single Flutter process with embedded Rust FFI

**Rejected as the primary topology.** It minimizes process count and network surface, but a UI crash terminates systems work, generated FFI introduces ABI and `unsafe` review obligations, and future CLI/IDE clients would require another boundary.

### Permanent Windows service

**Deferred.** It could continue background jobs and serve multiple clients, but introduces service installation, privilege, update, recovery and user-trust complexity that is unnecessary for v0.1.

### Local microservices

**Rejected.** Multiple independently deployed services would multiply ports, authentication boundaries, logs, upgrades and failure modes on a single device without justified operational benefit.

### One executable per platform module

**Rejected for the foundation.** Process isolation may later be valuable for untrusted Pack workers or privileged helpers, but core modules should remain in-process Rust components until a measured security or reliability need justifies separation.

## Consequences

### Positive

- Preserves clear module boundaries without microservice overhead.
- Keeps system and persistence logic outside Flutter.
- Supports restart-safe long operations and future local clients.
- Provides one controlled place for policy, auditing and provider abstraction.
- Allows isolated helpers or Pack workers to be introduced later without rewriting the domain model.

### Negative

- Packaging contains at least two GixGiz executables.
- Startup, shutdown, crash detection and version compatibility must be implemented and tested.
- The local transport becomes a security boundary even though it is loopback-only.
- v0.1 jobs do not continue indefinitely after the user fully exits the desktop application.

## Security and privacy impact

- Normal desktop and core processes run without administrator rights.
- Provider and privileged operations remain behind validated adapters/helpers.
- The core binds only to the local interface and requires per-launch authentication as defined in ADR 0003.
- Only the core may access the platform database.
- Orphan detection and a per-user single-instance lock reduce accidental duplicate writers.
- Model output and external content remain untrusted; this topology does not grant them operating-system authority.

## Implementation constraints

- Task 02 creates only the Flutter shell.
- Task 03 creates the Rust workspace and core process.
- Task 04 establishes the typed local boundary.
- Task 05 adds core-owned SQLite persistence.
- No task may collapse the core into widgets or expose Ollama directly to Flutter.
- No Windows service, updater, privileged helper or plugin process is required before Task 05.

## Follow-up work

- ADR 0002 defines the Windows, Flutter and Rust baseline.
- ADR 0003 defines the local transport and bootstrap protocol.
- ADR 0004 defines SQLite ownership and migration policy.
- ADR 0005 defines application identity and packaging direction.
- A later ADR must define privileged installation and rollback before Ollama installation is implemented.
- A later ADR must define the runtime abstraction and initial Ollama adapter before Task 09.