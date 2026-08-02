# Architecture Decision Records

Architecture Decision Records (ADRs) preserve the context and consequences of important JimotoAI engineering decisions.

## Naming

Use sequential names:

```text
0001-use-a-local-first-modular-monolith.md
0002-select-flutter-for-the-desktop-shell.md
0003-select-rust-for-the-platform-core.md
```

## Required structure

```markdown
# ADR NNNN: Decision title

- Status: Proposed | Accepted | Superseded | Deprecated
- Date: YYYY-MM-DD
- Owners: names or team

## Context

## Decision

## Alternatives considered

## Consequences

## Security and privacy impact

## Follow-up work
```

## Initial ADR backlog

- Local-first modular monolith rather than microservices.
- Flutter for the desktop experience.
- Rust for the platform core.
- Flutter–Rust communication mechanism.
- SQLite local persistence and migration policy.
- Windows-first support strategy.
- Runtime adapter boundary and initial Ollama provider.
- Installer privilege and rollback strategy.

Create an ADR before implementing a decision that is difficult to reverse, cross-cutting, security-sensitive, externally visible, or likely to be questioned later.
