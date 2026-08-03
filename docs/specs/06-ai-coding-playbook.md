---
title: 1. Purpose, Operating Model and Non-Negotiables
---

*How to develop GixGiz reliably with Codex and other frontier coding agents*

**Document 6 of 6 \| AI Coding Playbook \| Page 1 of 15**

## Operating model

AI coding agents act as implementers, reviewers and investigators; the human remains product owner and architecture authority. GixGiz is developed through small verified tasks, not one instruction to "build the platform." Every task begins from written scope and ends with evidence.

## Non-negotiables

- Inspect before editing.

- State a file-level plan before substantial changes.

- Respect module boundaries and existing Architecture Decision Records.

- Run formatting, linting, tests and relevant builds.

- Review the final diff and report limitations.

- Never hide failing tests, warnings, unsafe operations or incomplete acceptance criteria.

- No system-wide installation, deletion, release or push without explicit human approval.

## Agent roles

| **Role** | **Use** |
|----|----|
| Explorer | Map current code, contracts and risks without modifying files. |
| Implementer | Make one focused change with tests. |
| Reviewer | Inspect diff for correctness, architecture, security and missing tests. |
| Debugger | Reproduce a concrete failure, isolate cause and propose minimal fix. |
| Documenter | Update specs, ADRs and user/developer documentation from verified implementation. |
| Release assistant | Prepare changelog and validation evidence; never publish without approval. |

| **Core discipline:** AI speed is useful only when each generated change remains understandable, testable and reversible. |
|----|

2\. Repository and Documentation Structure

**Document 6 of 6 \| AI Coding Playbook \| Page 2 of 15**

gixgiz/\
apps/desktop/\
crates/core/ hardware/ capability/ gateway/ tools/ \...\
packages/first-party/\
integrations/vscode/ cli/\
sdk/\
docs/specs/ adr/ guides/\
tests/fixtures/ e2e/ security/\
AGENTS.md

## Documentation hierarchy

| **Document** | **Authority** |
|----|----|
| Product Vision | Why the platform exists and the full product direction. |
| Architecture | System boundaries, dependency direction and major contracts. |
| Module Specifications | Responsibilities, interfaces and acceptance expectations. |
| UI/UX Specification | User-facing behaviour, states and interaction principles. |
| Development Roadmap | Order and release scope; not a substitute for feature specification. |
| AI Coding Playbook | How agents must work in the repository. |
| ADR | Reason for one important architectural decision and its consequences. |
| Issue/task spec | Immediate implementable scope and acceptance criteria. |

## Repository rules

- One source of truth for shared API schemas.

- Generated files are marked and never hand-edited.

- Fixtures contain no personal or secret data.

- Large model binaries are never committed.

- Module-specific AGENTS.md can add constraints but cannot weaken root safety rules.

- Every externally consumed contract has an owner and compatibility policy.

3\. Architectural Guardrails

**Document 6 of 6 \| AI Coding Playbook \| Page 3 of 15**

## Dependency direction

UI / Packs / Integrations\
↓\
Gateway / Workflows / Application services\
↓\
Domain contracts / Policy\
↓\
System adapters / Runtime providers / Storage

## Guardrails

- Flutter UI never invokes shell commands or provider endpoints directly.

- Packs never read platform databases directly.

- Runtime-specific code stays in adapters.

- OS-specific code stays behind system-provider interfaces.

- Policy checks occur before every tool execution; UI confirmation alone is insufficient.

- Business logic does not depend on global mutable state.

- Long operations use durable jobs and cancellation, not blocking UI calls.

- Unknown hardware/provider values remain explicit rather than guessed.

## When an agent proposes a new dependency

1.  Explain the unmet requirement.

2.  Compare standard library and existing dependencies.

3.  Evaluate licence, maintenance, security and cross-platform support.

4.  Identify binary size and packaging impact.

5.  Obtain human approval when dependency is architectural or privileged.

| **Refusal rule:** An agent should stop and report conflict when a task requires violating a guardrail rather than quietly working around it. |
|----|

4\. Task Definition and Prompt Format

**Document 6 of 6 \| AI Coding Playbook \| Page 4 of 15**

## Required task template

Title\
Context and user value\
Current behaviour\
Desired behaviour\
In scope / Out of scope\
Architecture constraints\
Security constraints\
Acceptance criteria\
Required tests\
Validation commands\
Documentation updates

## Good scope

A task should normally change one capability and be reviewable as one coherent diff. "Detect existing Ollama installation on Windows and map health states" is actionable. "Implement runtime management" is an epic and must be decomposed.

## Acceptance criteria rules

- Describe externally observable behaviour, not implementation preference alone.

- Include failure, cancellation, unknown and permission-denied states.

- Specify what must not happen.

- Include tests or evidence for each criterion.

- Use concrete platform terms and exact supported OS/provider scope.

## Prompt example

Inspect the repository first. Do not edit until you identify the\
existing runtime contracts and propose a file-level plan.\
Implement only the scope below\...\
After implementation run: cargo fmt \--check, cargo clippy \...,\
cargo test \..., flutter test. Review the diff and report evidence.

5\. Codex Session Workflow

**Document 6 of 6 \| AI Coding Playbook \| Page 5 of 15**

## Standard session loop

6.  Open with the issue and relevant specification links.

7.  Ask the agent to inspect symbols, tests and recent related changes.

8.  Review the proposed plan; correct scope or architecture before edits.

9.  Let the agent implement incrementally and surface early blockers.

10. Require validation commands and inspect their actual outputs.

11. Request self-review against acceptance criteria and guardrails.

12. Use a fresh reviewer session for important security or architecture changes.

13. Commit only after human diff review.

## Modes

| **Mode** | **Instruction** |
|----|----|
| Explore | Do not modify files; map architecture, references, tests and risks. |
| Implement | Make focused changes, test and stop when scope is complete. |
| Review | Do not modify; rank findings with exact file/line evidence. |
| Fix review | Address selected findings only and preserve unrelated code. |
| Refactor | First prove behaviour with tests; preserve public contracts unless approved. |
| Document | Derive documentation from verified code and decisions, not assumptions. |

## Interruptions

If the agent discovers a material architecture conflict, unsafe operation or missing requirement, it should pause implementation and present the conflict. Minor details can use conservative assumptions that are documented in the final report.

6\. Context Management and AGENTS.md

**Document 6 of 6 \| AI Coding Playbook \| Page 6 of 15**

## Root AGENTS.md content {#root-agents.md-content}

- Product identity and current release scope.

- Module boundaries and dependency rules.

- Safety restrictions for files, terminal, installers and network.

- Formatting, linting, testing and build commands.

- Definition of done and required report format.

- Generated-code and documentation conventions.

## Nested instructions

Place focused instructions near complex areas: Rust core, Flutter UI, runtime adapters, pack sandbox, VS Code extension and release tooling. Nested rules may define frameworks, tests and local conventions but must reference shared contracts rather than duplicating them.

## Context packets for major tasks

| **Include** | **Avoid** |
|----|----|
| Relevant spec sections and ADRs | Entire repository dump when symbols can be discovered. |
| Current interface definitions | Outdated copied interfaces in prompts. |
| Acceptance tests and failure examples | Only happy-path prose. |
| Exact validation commands | Vague "make sure it works." |
| Known constraints and non-goals | Inviting broad unsolicited rewrites. |
| Recent related diff if necessary | Unbounded conversation history as primary authority. |

## Context freshness

Repository documents are version-controlled and authoritative over previous chat discussions. When a prompt conflicts with code or specifications, the agent must call out the conflict and ask the human to select the intended authority before changing public contracts.

7\. Coding Standards: Rust Core

**Document 6 of 6 \| AI Coding Playbook \| Page 7 of 15**

## Core conventions

- Stable Rust unless an approved feature requires nightly.

- Explicit domain types instead of stringly typed IDs, states and paths.

- thiserror-style typed errors at module boundaries; anyhow only at application edges where appropriate.

- Async operations accept cancellation and timeouts; no blocking work on async executors.

- No unsafe code without an ADR, isolated module, invariants and dedicated review.

- Use tracing spans and structured fields; never log secrets or full private content by default.

## Interface pattern

pub trait ToolExecutor: Send + Sync {\
async fn execute(\
&self, ctx: ToolContext, input: ValidatedInput,\
cancel: CancellationToken\
) -\> Result\<ToolObservation, ToolError\>;\
}

## Error requirements

| **Error class**   | **Example**                                       |
|-------------------|---------------------------------------------------|
| InvalidInput      | Malformed path or unsupported model reference.    |
| PermissionDenied  | Capability or scoped resource not granted.        |
| Unavailable       | Runtime not installed or provider offline.        |
| Conflict          | File changed since proposed edit.                 |
| ResourceExhausted | Insufficient RAM, storage or scheduler budget.    |
| Cancelled         | User or policy cancelled the job.                 |
| IntegrityFailure  | Checksum or signature mismatch.                   |
| Internal          | Unexpected invariant failure with correlation ID. |

8\. Coding Standards: Flutter Desktop

**Document 6 of 6 \| AI Coding Playbook \| Page 8 of 15**

## UI architecture

- Presentation consumes typed application services; no core domain logic in widgets.

- State distinguishes loading, ready, empty, degraded, failed and cancelled.

- Long-running operations subscribe to job/event streams.

- Navigation and permissions use centralized services.

- Strings are localized from the start; layouts tolerate expansion.

- Widgets expose semantic labels, keyboard focus and test keys where needed.

## Component expectations

| **Component** | **Requirement** |
|----|----|
| Recommendation card | Reasons, resource estimate, confidence and alternatives. |
| Progress view | Current step, overall progress, pause/cancel and resumability. |
| Approval dialog | Exact capability, scope, risk, proposed effects and choices. |
| Diff/change view | File-by-file selection, explanation and revert path. |
| Health status | Plain meaning plus expandable technical detail. |
| Model list | Compatibility, installed state, storage and licence---not leaderboard hype. |

## Testing

Widget tests cover all states, not only the successful one. Golden tests may protect critical setup layouts, but semantic and behavioural tests remain primary. Integration tests use a fake core service and deterministic event streams.

9\. API, Schemas and Data Evolution

**Document 6 of 6 \| AI Coding Playbook \| Page 9 of 15**

## Schema discipline

- Define schemas once and generate clients where practical.

- All messages carry schema/API version and correlation ID.

- Enums include Unknown or forward-compatible handling where external providers evolve.

- Public fields are never repurposed; use additive evolution and deprecation.

- Paths and secrets use opaque handles across trust boundaries when possible.

## Database evolution

14. Write migration with forward and tested rollback/restore strategy.

15. Back up before irreversible transformation.

16. Test migration from every supported previous release.

17. Separate schema migration from expensive content re-indexing.

18. Keep jobs resumable and report progress.

## API review checklist

| **Question** | **Expected answer** |
|----|----|
| Who authenticates? | Named client/pack identity and required scope. |
| Can it be cancelled? | Cancellation semantics and propagation defined. |
| What is bounded? | Payload, stream, context, output and time limits. |
| How does it fail? | Stable error code and retry guidance. |
| Is it idempotent? | Explicit key or documented non-idempotence. |
| What is audited? | Relevant request metadata and policy decision without leaking content. |

10\. Testing Strategy and Acceptance Evidence

**Document 6 of 6 \| AI Coding Playbook \| Page 10 of 15**

## Test pyramid

| **Level** | **Examples** |
|----|----|
| Unit | Rule evaluation, state machines, parsers, policy decisions and path validation. |
| Contract | Every runtime adapter and tool provider against shared behaviour suites. |
| Integration | Core + SQLite + fake provider; installer staging; gateway streaming and cancellation. |
| UI | Widget states, onboarding flow, approval interactions and accessibility semantics. |
| End-to-end | Scan → plan → simulated install → model → chat; coding agent in disposable repository. |
| System/security | Real provider smoke tests, sandbox escape attempts, malicious pack and prompt injection. |
| Performance | Startup, model load, token stream, indexing, memory and large job recovery. |

## Agent evidence report

Changed files\
Acceptance criteria mapping\
Commands run and results\
Tests added/updated\
Manual checks performed\
Known limitations / follow-up\
Security or migration impact

## Test quality rules

- Do not weaken assertions to make generated code pass.

- No network-dependent unit tests.

- Use temporary directories and disposable repositories.

- Failure injection must cover cancellation and partial effects.

- Flaky tests are treated as defects and not automatically retried into invisibility.

11\. Security and System-Access Rules

**Document 6 of 6 \| AI Coding Playbook \| Page 11 of 15**

## Agent environment restrictions

- Repository read/write is allowed only within the active workspace unless approved.

- No access to personal directories, credentials or browser profiles.

- No administrator/elevated commands without explicit per-operation approval.

- No publishing, pushing, package release or external upload without approval.

- Network access is limited to required documentation/dependency sources and is logged.

- Destructive commands and broad wildcards are prohibited in agent-generated automation.

## Secure implementation checklist

| **Area** | **Checks** |
|----|----|
| Paths | Canonicalize; enforce approved roots; reject traversal and symlink escapes. |
| Commands | Structured arguments; no user-data shell concatenation; time/resource limits. |
| Local API | Authenticate, restrict origins, bind safely and validate all input. |
| Packages | Signature/checksum/licence verification and permission-delta review. |
| Secrets | OS store; handles in prompts; redact logs and diagnostics. |
| Files | Expected hash before overwrite; staged writes; backups for impactful changes. |
| Prompts | Treat retrieved content as untrusted data, not system instructions. |

## Security review prompt

Review only. Focus on privilege escalation, command injection,\
path traversal, symlink attacks, local API abuse, package integrity,\
prompt injection, secrets exposure, cancellation and rollback. Rank\
findings by severity with exact evidence and exploit scenario.

12\. Review, Debugging and Refactoring

**Document 6 of 6 \| AI Coding Playbook \| Page 12 of 15**

## Review passes

19. Correctness against acceptance criteria.

20. Architecture and dependency direction.

21. Security and trust-boundary impact.

22. Failure, cancellation, retry and recovery behaviour.

23. Cross-platform and packaging impact.

24. Test coverage and test validity.

25. UX state and diagnostic quality.

26. Documentation and migration changes.

## Debugging protocol

- Reproduce first and capture exact environment/version.

- Reduce to the smallest failing path.

- Identify whether failure belongs to platform, adapter, provider or OS.

- Add a failing regression test when feasible.

- Apply minimal fix; avoid unrelated refactors.

- Validate both original failure and nearby behaviour.

## Refactoring protocol

Refactors require characterization tests before structural changes. Public contracts, error semantics, database migrations and serialized state are preserved unless the task explicitly authorizes a breaking change. Large refactors are split into behaviour-preserving preparation and separately reviewed functional changes.

## Review output

Findings are ranked Critical, High, Medium and Low. Each includes evidence, consequence and recommended correction. A clean review states what was examined and what remains outside scope; it does not claim universal safety.

13\. Git, CI/CD and Release Discipline

**Document 6 of 6 \| AI Coding Playbook \| Page 13 of 15**

## Branch and commit discipline

- One branch per coherent task or stacked series with explicit dependency.

- Commits explain intent and keep generated changes separate where useful.

- Do not mix mass formatting with functional changes.

- Never commit secrets, downloaded models, personal test files or local databases.

- Agents may prepare commits but human approval controls push and pull-request creation.

## Required CI lanes

| **Lane** | **Checks** |
|----|----|
| Fast | Formatting, lint, unit tests and schema generation consistency. |
| Platform | Windows first; Linux/macOS as support is added. |
| Integration | Database migrations, fake runtime, gateway streaming and job recovery. |
| Security | Dependency audit, secret scan, package signature tests and SAST where useful. |
| Packaging | Installer build, signing verification and clean-machine smoke test. |
| Compatibility | Supported previous config/database versions and adapter contract suites. |

## Release preparation

27. Freeze and classify changes.

28. Run full matrix and manual critical-path checks.

29. Create signed artifacts and software bill of materials.

30. Verify updater and rollback from previous supported version.

31. Update changelog, known issues and migration notes.

32. Human approves publication and staged rollout.

14\. Parallel Agents and Work Decomposition

**Document 6 of 6 \| AI Coding Playbook \| Page 14 of 15**

## When parallel agents help

- Independent modules with stable shared interfaces.

- Tests, documentation and security review that do not edit the same files.

- Separate operating-system provider implementations.

- Research tasks producing options before one architectural choice.

- Review agents with different focus: correctness, security, performance and UX.

## When not to parallelize

- Multiple agents changing one public interface simultaneously.

- Architecture is unresolved or acceptance criteria are vague.

- Migrations and serialized contracts are changing without one owner.

- A failing build prevents reliable branch validation.

## Work packet

Shared baseline commit\
Owned directories / files\
Allowed interface assumptions\
Task acceptance criteria\
Validation commands\
Expected output and integration notes\
No-touch areas

## Integration owner

One agent or human owns final integration. It rebases or cherry-picks deliberately, resolves contract drift, runs the full validation suite and rejects overlapping shortcuts. Parallel output is not automatically correct merely because individual branches pass isolated tests.

| **Rule:** Parallelize implementation after interfaces stabilize; parallelize review early and often. |
|----|

15\. Definition of Done and Reusable Prompt Templates

**Document 6 of 6 \| AI Coding Playbook \| Page 15 of 15**

## Definition of done

- All acceptance criteria are demonstrated.

- Code follows architecture and module ownership.

- Tests cover success, failure, cancellation and permission boundaries.

- Formatting, linting, tests and relevant builds pass.

- No new high-severity security finding remains.

- User-visible states and error messages are implemented.

- Docs, schemas, migrations and changelog are updated where applicable.

- Diff is reviewed by a human and important changes receive an independent agent review.

## Reusable implementation prompt

Read AGENTS.md and the linked specs. Inspect before editing.\
Implement only \[TASK\]. Preserve \[CONTRACTS\]. Do not touch \[OUT\].\
Security constraints: \[RULES\]. Acceptance criteria: \[LIST\].\
Add tests for success, failure and cancellation. Run \[COMMANDS\].\
Then review your diff and return the evidence report format.

## Reusable review prompt

Do not modify files. Review this change against \[SPEC/ISSUE\].\
Check correctness, architecture, security, cancellation, recovery,\
API compatibility, tests and UX states. Rank findings with file/line\
evidence. State reviewed scope and remaining uncertainty.

## Reusable bug-fix prompt

Reproduce \[FAILURE\] first. Identify the smallest responsible layer.\
Add a regression test, make the minimal fix, and avoid unrelated\
refactors. Run \[COMMANDS\]. Report root cause, changed behaviour and\
remaining environment-specific uncertainty.

| **Final principle:** GixGiz should be built at AI speed with software-engineering control: specifications define intent, tests define evidence, and human review defines acceptance. |
|----|
