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

Architecture and repository-foundation phase.

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
- Rustup with the stable Rust MSVC toolchain.

Task 02 will pin the concrete Flutter version when it creates the desktop application. Task 03 will add `rust-toolchain.toml` and pin the Rust baseline when it creates the Cargo workspace. Until those manifests exist, do not infer or document versions that the repository cannot enforce.

## Initial development workflow

Clone the repository and inspect the focused task before making changes:

```powershell
git clone https://github.com/Phineas-bot/gixgiz.git
Set-Location gixgiz
git status --short
```

Task 01 intentionally contains no Flutter or Rust workspace, so it has no root build command yet. For the current documentation foundation, run:

```powershell
git diff --check
git -c core.quotepath=false ls-files | Where-Object { $_.Contains([char]0x200B) }
```

The second command must produce no output. Also verify the Markdown navigation links manually. Each later task specification defines its exact targeted commands; once a repository task runner is introduced, its documented commands become the validation source of truth.

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
