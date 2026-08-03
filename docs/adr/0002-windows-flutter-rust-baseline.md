# ADR 0002: Windows, Flutter and Rust baseline

- **Status:** Accepted
- **Date:** 2026-08-03
- **Owners:** GixGiz project owner
- **Decision scope:** Foundation technology baseline; required before Tasks 02–05
- **Related:** ADR 0001, Product Vision, Architecture, Development Roadmap, GitHub issues #2 and #3

## Context

GixGiz needs a Windows-first desktop experience that is accessible to non-technical users while supporting system inspection, runtime supervision, durable jobs and future local integrations. The project must avoid a UI framework that owns platform business logic and avoid a systems stack that makes desktop development unnecessarily difficult.

The initial release targets Windows 11 x64. Cross-platform support remains a future architectural goal, but it must not force the first release to support incomplete abstractions or untested operating systems.

## Decision

### 1. Supported foundation platform

- The first supported operating system is **Windows 11 x64**.
- Windows 10, ARM64, Linux and macOS are not claimed as supported by v0.1 unless later evidence and an explicit scope update add them.
- Platform-neutral domain contracts must avoid unnecessary Windows-specific fields, but operating-system implementations may use Windows-native APIs behind provider interfaces.

### 2. Desktop technology

The desktop shell will use **Flutter stable** and Dart stable as delivered by the selected Flutter release.

Flutter owns:

- window composition and navigation;
- onboarding and setup presentation;
- localization, theming and accessibility;
- rendering typed state received from the Rust core;
- transient UI state and user intentions.

Flutter does not own:

- hardware or runtime business rules;
- process execution or installer operations;
- SQLite access;
- provider-specific APIs;
- authoritative readiness decisions.

Task 02 will create a minimal Windows desktop shell under `apps/desktop/` with no speculative future workspace modules.

### 3. Core technology

The platform core will use **stable Rust** in a Cargo workspace.

Rust owns:

- domain contracts and application services;
- long-running job coordination;
- Windows system providers;
- runtime and model adapters;
- persistence and migrations;
- structured diagnostics and cancellation;
- the local transport host.

The initial workspace will create only crates required by the active task. Long-term crate maps are architectural direction, not an instruction to create empty crates.

### 4. Toolchain policy

- Use the stable Rust channel, pinned through `rust-toolchain.toml` once Task 03 scaffolds the workspace.
- Use a concrete Flutter stable version in CI and developer documentation once Task 02 scaffolds the desktop application.
- Toolchain upgrades are deliberate changes with release notes, CI validation and dependency compatibility review.
- Do not use Rust nightly unless a later ADR justifies a specific feature.
- Do not track floating `latest` versions in reproducible CI configuration.

### 5. Application and source conventions

- Product display name: `GixGiz`.
- Repository and source identifier: `gixgiz`.
- Dart package names use lowercase snake case where required by Dart conventions.
- Rust crates use lowercase kebab case, including `gixgiz-core` and `gixgiz-contracts`.
- Text files use UTF-8 and repository line-ending rules.
- User-facing strings are localizable from the beginning; widgets do not embed reusable production copy directly.
- Generated bindings are produced from one authoritative contract source and are not hand-edited.

### 6. UI architecture baseline

The Flutter application uses feature-oriented presentation modules with a centralized typed core client. Every asynchronous screen state must be able to distinguish, where relevant:

- initial;
- loading;
- ready;
- empty;
- degraded;
- failed;
- cancelled;
- attention required.

Widgets emit intentions and render state. Domain decisions remain in Rust.

### 7. Rust engineering baseline

- Explicit domain types are preferred over raw strings for identifiers, versions, states and paths.
- Module boundaries use typed errors and preserve diagnostic causes.
- Async operations accept cancellation and timeouts.
- Blocking work must not run on async executor threads.
- Production paths avoid panic, `unwrap` and `expect` except for documented proven invariants.
- `unsafe` code requires a separate accepted ADR, isolated implementation and dedicated review.
- Structured logging uses correlation IDs and excludes secrets and private content by default.

## Alternatives considered

### Electron or Tauri for the desktop shell

Not selected. They could provide strong web-development familiarity, but Flutter better matches the existing product direction, cross-platform desktop goals, stateful onboarding and accessibility requirements. Tauri would also blur the chosen Flutter/Rust split.

### Native Windows UI

Not selected for the foundation. WinUI or WPF could provide deep Windows integration but would reduce future cross-platform reuse and add another primary language/runtime to the product.

### Dart-only implementation

Rejected. It would simplify one language boundary but is less suitable for the planned systems, process, installer and provider responsibilities.

### Rust-native UI

Deferred. Rust UI frameworks do not currently provide the same mature desktop product workflow, accessibility and localization direction required for this project.

### Supporting all desktop operating systems immediately

Rejected. It would weaken testing and delay the first complete vertical slice. Platform boundaries remain extensible, but Windows receives the first supported implementation.

## Consequences

### Positive

- Clear ownership between presentation and systems logic.
- Stable, strongly typed systems core suited to Windows integration and durable jobs.
- Flutter supports polished guided workflows and later cross-platform UI reuse.
- Toolchain pinning improves reproducibility.
- Windows-first scope keeps validation realistic.

### Negative

- The product must maintain a language and process boundary.
- Developers need both Flutter and Rust toolchains plus Windows C++ build prerequisites.
- Generated contracts and packaging must keep Flutter, Rust and Windows artifacts compatible.
- Cross-platform claims remain deferred until real providers and CI exist.

## Security and privacy impact

- Flutter cannot directly execute privileged or provider operations.
- Rust contains the security-sensitive system boundaries and is reviewed accordingly.
- Toolchains and dependencies must be pinned and audited to reduce supply-chain drift.
- No user content, credentials or model binaries belong in source control or test fixtures.
- Debug builds must not weaken local authentication or secret handling in a way that can accidentally ship.

## Implementation constraints

- Task 02 creates the Flutter shell without Rust business logic.
- Task 03 creates the Rust workspace without Flutter presentation dependencies.
- Task 04 connects them through ADR 0003.
- Task 05 adds persistence only in Rust.
- The foundation does not create Linux/macOS runners, alternative desktop shells or future Pack UI.

## Follow-up work

- Pin exact Flutter and Rust versions when their project manifests are introduced.
- Document Windows prerequisites: Visual Studio Build Tools with Desktop development with C++, Flutter Windows support, Git and Rustup.
- Add nested `AGENTS.md` files to `apps/desktop/` and the Rust core when those directories exist.
- Review supported Windows versions before the first external alpha.