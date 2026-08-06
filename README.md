# GixGiz

GixGiz is a desktop app that makes local AI simple. It auto-detects your PC hardware, installs the right runtime, downloads the best AI models, and gives you a GUI to use them. No technical setup is required for end users.

## One-click local setup, no CLI needed

## Current target

GixGiz v0.1 is a Windows-first foundation release that will:

1. Detect the user's hardware and relevant prerequisites.
2. Generate an explainable capability report.
3. Recommend a compatible local model configuration.
4. Detect and reuse, or explicitly install, Ollama.
5. Download and verify the selected model.
6. Provide a streaming local chat experience.

The v0.1 success condition is that a supported non-technical Windows user can move from installation to a verified local-model response through one guided graphical flow without opening a terminal.

## Status

Flutter Windows desktop-shell and Rust platform-core foundations. The shell remains disconnected from the Rust core until Task 04.

## Repository map

```text
gixgiz/
├── apps/                  # User-facing applications; Flutter desktop first
├── crates/                # Rust platform core and service modules
├── docs/
│   ├── specs/             # Product, architecture, modules, UX and release specs
│   ├── adr/               # Architecture Decision Records
│   └── tasks/             # Implementable work specifications
├── integrations/          # Future IDE and external-client integrations
├── packages/              # Future first-party AI Packs
├── sdk/                   # Future public SDK
├── tests/                 # Integration, end-to-end and security tests
├── AGENTS.md              # Authoritative coding-agent instructions
└── AGENT.md               # Convenience redirect
```

Directories are added when real implementation work requires them; the project does not create speculative empty modules.

## Documentation

- [`AGENTS.md`](./AGENTS.md) — repository-wide engineering and AI-agent policy.
- [`docs/specs/`](./docs/specs/) — authoritative product and technical specifications.
- [`docs/adr/`](./docs/adr/) — accepted architectural decisions.
- [`docs/tasks/`](./docs/tasks/) — issue-quality implementation specifications.

## Development approach

GixGiz is developed as a local-first modular monolith using vertical slices. The logical v0.1 task sequence is:

| Task | Capability |
|---:|---|
| 01 | [Establish repository and build architecture](./docs/tasks/0001-establish-repository-and-build-architecture.md) |
| 02 | [Create the Flutter Windows desktop shell](./docs/tasks/0002-create-flutter-windows-desktop-shell.md) |
| 03 | [Create the Rust workspace and platform core](./docs/tasks/0003-create-rust-workspace-and-platform-core.md) |
| 04 | [Establish typed Flutter–Rust communication](./docs/tasks/0004-establish-typed-flutter-rust-communication.md) |
| 05 | [Add the SQLite persistence foundation](./docs/tasks/0005-add-sqlite-persistence-foundation.md) |
| 06 | [Add the Windows CI pipeline](./docs/tasks/0006-add-windows-ci-pipeline.md) |
| 07 | [Implement the hardware-scan vertical slice](./docs/tasks/0007-implement-hardware-scan-vertical-slice.md) |
| 08 | [Implement the capability recommendation engine](./docs/tasks/0008-implement-capability-recommendation-engine.md) |
| 09 | [Implement the Ollama runtime adapter](./docs/tasks/0009-implement-ollama-runtime-adapter.md) |
| 10 | [Implement the model installation workflow](./docs/tasks/0010-implement-model-installation-workflow.md) |
| 11 | [Implement local streaming chat](./docs/tasks/0011-implement-local-streaming-chat.md) |

All contributors and coding agents must read [`AGENTS.md`](./AGENTS.md) before changing the repository.

## Development prerequisites

The supported implementation environment is Windows 11 x64. Development requires:

- Git.
- Flutter stable with Windows desktop support enabled.
- Visual Studio 2022 or Visual Studio Build Tools with the **Desktop development with C++** workload and a Windows SDK.
- Rustup with the pinned Rust `1.97.1` MSVC toolchain from [`rust-toolchain.toml`](./rust-toolchain.toml).

Task 02 uses Flutter `3.44.8` stable with Dart `3.12.2`; the generated project metadata records framework revision `058e0af2c2b57e369d905a03ac9748b0ebf543c6`. Cargo automatically selects the repository toolchain, including `rustfmt` and `clippy`, when Rustup is available.

## Rust platform foundation

The root Cargo workspace contains three crates with a strict dependency direction:

```text
gixgiz-desktop-host -> gixgiz-core -> gixgiz-contracts
```

- `gixgiz-contracts` owns serializable, provider-neutral identity, readiness, health, error, recovery, and request-correlation contracts.
- `gixgiz-core` owns platform lifecycle, deterministic readiness policy, safe error mapping, diagnostics initialization, cancellation, and timeout conventions.
- `gixgiz-desktop-host` builds the future sidecar executable as `gixgiz-core.exe` and currently provides composition only. It has no transport, persistence, provider, or operating-system behavior.

Run the Rust checks from the repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Initial development workflow

Clone the repository and inspect the focused task before making changes:

```powershell
git clone https://github.com/Phineas-bot/gixgiz.git
Set-Location gixgiz
git status --short
```

For repository-level hygiene, run:

```powershell
git diff --check
git -c core.quotepath=false ls-files | Where-Object { $_.Contains([char]0x200B) }
```

The second command must produce no output. Also verify the Markdown navigation links manually. Each task specification defines its exact targeted commands; once a repository task runner is introduced, its documented commands become the validation source of truth.

For the Task 02 desktop shell, use the exact setup, analysis, test, run, and build commands in [`apps/desktop/README.md`](./apps/desktop/README.md).

## Technology direction

- **Desktop UI:** Flutter
- **Core platform:** Rust
- **Local persistence:** SQLite
- **Initial runtime provider:** Ollama
- **Initial platform:** Windows 11 x64
- **Automation:** GitHub Actions

These choices are governed by the accepted foundation Architecture Decision Records.

## Contributing

The repository is in its foundation stage. Work should be linked to a focused issue with explicit scope, non-goals, acceptance criteria, tests, and validation evidence. Do not submit broad "build the platform" changes.

## Licence

GixGiz is available under the [MIT License](./LICENSE).
