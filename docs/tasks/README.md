# Implementation Task Specifications

This directory contains the authoritative implementation work packets for GixGiz v0.1. Each file maps one logical task to its GitHub issue, dependencies, architecture constraints, acceptance criteria, required tests, validation commands and completion evidence.

A task should change one coherent capability and remain reviewable as one focused diff. GitHub issues summarize the work; these version-controlled task specifications are the detailed implementation authority.

## v0.1 task sequence

| Task | Specification | GitHub issue | Depends on |
|---|---|---:|---|
| 01 | [Establish repository and build architecture](./0001-establish-repository-and-build-architecture.md) | [#1](https://github.com/Phineas-bot/gixgiz/issues/1) | — |
| 02 | [Create Flutter Windows desktop shell](./0002-create-flutter-windows-desktop-shell.md) | [#2](https://github.com/Phineas-bot/gixgiz/issues/2) | 01 |
| 03 | [Create Rust workspace and platform core](./0003-create-rust-workspace-and-platform-core.md) | [#3](https://github.com/Phineas-bot/gixgiz/issues/3) | 01 |
| 04 | [Establish typed Flutter–Rust communication](./0004-establish-typed-flutter-rust-communication.md) | [#4](https://github.com/Phineas-bot/gixgiz/issues/4) | 02, 03 |
| 05 | [Add SQLite persistence foundation](./0005-add-sqlite-persistence-foundation.md) | [#5](https://github.com/Phineas-bot/gixgiz/issues/5) | 03, 04 |
| 06 | [Add Windows CI pipeline](./0006-add-windows-ci-pipeline.md) | [#6](https://github.com/Phineas-bot/gixgiz/issues/6) | 02–05 |
| 07 | [Implement hardware scan vertical slice](./0007-implement-hardware-scan-vertical-slice.md) | [#7](https://github.com/Phineas-bot/gixgiz/issues/7) | 02–06 |
| 08 | [Implement capability recommendation engine](./0008-implement-capability-recommendation-engine.md) | [#8](https://github.com/Phineas-bot/gixgiz/issues/8) | 07 |
| 09 | [Implement Ollama runtime adapter](./0009-implement-ollama-runtime-adapter.md) | [#9](https://github.com/Phineas-bot/gixgiz/issues/9) | 03–08 plus runtime/installer ADRs |
| 10 | [Implement model installation workflow](./0010-implement-model-installation-workflow.md) | [#10](https://github.com/Phineas-bot/gixgiz/issues/10) | 05, 08, 09 |
| 11 | [Implement local streaming chat](./0011-implement-local-streaming-chat.md) | [#12](https://github.com/Phineas-bot/gixgiz/issues/12) | 04, 05, 09, 10 |

Task 11 is physically GitHub issue #12 because repository number #11 was previously assigned to a pull request.

## How to use a task specification

Before implementation, the coding agent must:

1. Read the root `AGENTS.md` and any nearer nested instructions.
2. Read the linked task specification, GitHub issue, relevant product specifications and accepted ADRs.
3. Inspect the repository before editing.
4. Restate scope and non-goals and present a file-level plan.
5. Identify any missing decision or conflict before implementation.

After implementation, the agent must run the specified validation, review the complete diff and return the evidence report required by `AGENTS.md`.

## Required task structure

New task specifications should include:

```markdown
# Task NN: Title

- GitHub issue
- Dependencies
- Required ADRs

## Context and user value
## Desired behavior
## In scope
## Out of scope
## Architecture constraints
## Security and privacy constraints
## Acceptance criteria
## Required tests
## Validation commands
## Documentation updates
## Completion evidence
```

## Change control

- Update a task specification when its intended behavior or acceptance criteria change.
- Update the linked issue summary when the task specification changes materially.
- Create or update an ADR when a task requires a cross-cutting, difficult-to-reverse or security-sensitive decision.
- Do not mark a task complete solely because mocked tests pass when its acceptance criteria require Windows packaging, real hardware or real-provider evidence.
- Do not begin a blocked task until its dependencies and required ADRs are accepted.
