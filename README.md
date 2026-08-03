# GixGiz

GixGiz is a desktop app that make local AI simple. It auto-detects your PC hardware, installs the right runtime, downloads the best AI models, and gives you a GUI to use them. No technical setup needed.

## One click local setup, No CLI needed.

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

GixGiz is developed as a local-first modular monolith using vertical slices. The planned implementation order is:

1. Repository and build foundation.
2. Flutter–Rust typed health handshake.
3. Hardware scanning and `MachineProfile`.
4. Deterministic capability recommendation.
5. Ollama lifecycle management.
6. Resumable model acquisition and verification.
7. Streaming local chat and persistence.

All contributors and coding agents must read [`AGENTS.md`](./AGENTS.md) before changing the repository.

## Technology direction

- **Desktop UI:** Flutter
- **Core platform:** Rust
- **Local persistence:** SQLite
- **Initial runtime provider:** Ollama
- **Initial platform:** Windows 11 x64
- **Automation:** GitHub Actions

These choices remain subject to accepted Architecture Decision Records before implementation is locked.

## Contributing

The repository is in its foundation stage. Work should be linked to a focused issue with explicit scope, non-goals, acceptance criteria, tests, and validation evidence. Do not submit broad "build the platform" changes.

## Licence

A project licence has not yet been selected. Until one is added, no licence is granted for reuse, redistribution, or modification outside GitHub's normal viewing and forking functionality.
