---
title: 1. Architecture Purpose and System Context
---

*Full-platform architecture specification*

**Document 2 of 6 \| Architecture \| Page 1 of 15**

## Scope

This architecture describes the entire GixGiz platform: desktop experience, local platform core, runtimes and models, secure agent tools, AI packs, third-party integrations, SDK, enterprise management and future cloud/hybrid execution. The initial implementation can be smaller, but its contracts should not block the full design.

## Architectural style

GixGiz is a modular desktop platform built as a local-first modular monolith. Core capabilities are separated by strict interfaces and internal events, yet initially ship as a coordinated installation rather than independently deployed microservices. This reduces operational complexity while preserving replaceable modules.

## System boundary

GixGiz owns orchestration and policy. It does not train foundation models and does not replace operating systems, IDEs or office suites. It installs or integrates with inference engines, manages models, exposes local APIs, brokers system tools, and embeds capabilities into existing user workflows.

## Primary actors

| **Actor** | **Interaction** |
|----|----|
| End user | Installs the platform, approves permissions, uses AI capabilities and reviews actions. |
| AI pack | Task-oriented application that requests platform capabilities through declared contracts. |
| External client | VS Code, JetBrains, CLI, office plug-in or another application consuming the local gateway. |
| Runtime provider | Ollama, llama.cpp, LM Studio, vLLM or future engine wrapped by an adapter. |
| Package publisher | Produces signed AI packs, runtime adapters, tools or model catalogue metadata. |
| Administrator | Defines organization policies, deployment channels and approved packages in enterprise mode. |

User / External Client\
│\
▼\
GixGiz Desktop + Local Gateway\
│\
├── Platform Core\
├── Policy & Tool Runtime\
├── Packs & Workflows\
└── Runtime Adapters → Local Models

2\. Layered Platform Architecture

**Document 2 of 6 \| Architecture \| Page 2 of 15**

┌────────────────────────────────────────────────────────────┐\
│ EXPERIENCE: Desktop UI, CLI, VS Code, Packs, External Apps │\
├────────────────────────────────────────────────────────────┤\
│ ORCHESTRATION: Gateway, Planner, Workflows, Agent Sessions │\
├────────────────────────────────────────────────────────────┤\
│ PLATFORM: Runtime, Models, Tools, Permissions, Packages │\
├────────────────────────────────────────────────────────────┤\
│ SYSTEM: Hardware, Processes, Storage, OS Secure Services │\
├────────────────────────────────────────────────────────────┤\
│ PROVIDERS: Ollama, llama.cpp, LM Studio, vLLM, Cloud later │\
└────────────────────────────────────────────────────────────┘

## Layer responsibilities

| **Layer** | **Responsibilities** | **Constraint** |
|----|----|----|
| Experience | User interaction, visual state, client-specific context collection. | Cannot directly execute shell commands or call runtime-specific APIs. |
| Orchestration | Turns intent into plans, selects capabilities, coordinates long-running work. | Uses platform contracts and policy decisions for every action. |
| Platform services | Owns runtimes, models, packages, tools, data and security boundaries. | Must remain usable without any specific pack. |
| System integration | OS queries, processes, files, credentials, notifications and installers. | Isolated behind platform-safe adapters. |
| Providers | Inference engines and optional remote providers. | No provider-specific details leak above adapter contracts. |

The dependency direction points downward. Higher layers may depend on abstractions defined by lower or shared contract packages; low-level modules must not import UI concerns. Cross-cutting services---logging, configuration, eventing, policy and identity---are injected through explicit interfaces.

| **Design rule:** A pack asks for a capability such as chat, embeddings or filesystem access; it never assumes a particular model, runtime, path layout or operating-system command. |
|----|

3\. Desktop Shell and Local Core Boundary

**Document 2 of 6 \| Architecture \| Page 3 of 15**

## Desktop composition

The desktop shell provides onboarding, dashboards, model and runtime management, permissions, pack discovery, agent sessions, settings and diagnostics. Flutter is a suitable cross-platform UI, while the systems core is implemented in Rust. The UI communicates with the core through a typed local bridge or loopback API and receives event streams for progress and inference output.

| **Boundary** | **Recommended contract** |
|----|----|
| Commands | Typed request/response calls for scan, install, configure, approve, cancel and query operations. |
| Events | Ordered events for progress, health changes, model output tokens, tool proposals and audit updates. |
| Files | Pass handles or approved paths; do not expose unrestricted path traversal from UI strings. |
| Errors | Stable error codes plus safe user messages and technical diagnostic details. |
| Versioning | Handshake with API version and feature flags so desktop and core upgrades fail clearly. |

## Core process topology

- Desktop shell process: presentation, accessibility, local navigation and minimal transient state.

- GixGiz core service: long-running orchestration, APIs, persistent state, policy and runtime supervision.

- Provider processes: runtime servers, model workers and optional isolated tool workers.

- Updater/installer helper: narrowly privileged process invoked only for approved system changes.

- Pack sandbox processes: optional separate execution boundaries for untrusted or complex extensions.

A crash in the desktop shell should not corrupt model downloads or installations. Long-running tasks are represented in the core database and can resume or report a recoverable failure after restart.

4\. Hardware Intelligence and Capability Planning

**Document 2 of 6 \| Architecture \| Page 4 of 15**

## Machine Profile

The Hardware Scanner produces evidence, not recommendations. It collects operating system, architecture, CPU instruction support, physical and available RAM, GPU devices, dedicated/shared memory, drivers, CUDA/ROCm/Metal/DirectML capability, storage capacity, thermal/power indicators where permitted and network state. Unknown data remains unknown.

## Capability Engine

The Capability Engine converts the machine profile into supported workload envelopes. Rules combine hard compatibility constraints, benchmark evidence, conservative memory estimates and platform policy. The output distinguishes "supported," "possible with compromise," "not recommended" and "unknown."

| **Input** | **Derived output** |
|----|----|
| 6 GB VRAM + 32 GB RAM | 7--9B quantized text models likely comfortable; larger models may use partial offload. |
| No compatible GPU | CPU inference allowed with realistic latency warning and smaller default model. |
| Low free storage | Prevent large model plan, offer external storage and cleanup choices. |
| Battery mode / thermal pressure | Prefer efficient model, lower concurrency and reduced context. |
| User goal: coding | Prefer coding-capable model class and reserve tools/context requirements. |

## Planner output

InstallationPlan { runtime, model_class, concrete_model,\
quantization, context_limit, acceleration, storage_location,\
required_components, expected_memory, confidence, reasons, warnings }

Recommendation data should be updateable independently from the desktop binary and cryptographically signed. Local benchmark feedback can improve future suggestions without uploading machine details unless the user explicitly opts in.

5\. Runtime Lifecycle and Abstraction Layer

**Document 2 of 6 \| Architecture \| Page 5 of 15**

## Runtime Manager vs Runtime Abstraction

| **Component** | **Responsibility** |
|----|----|
| Runtime Manager | Detect, install, update, configure, start, stop, supervise and uninstall runtime instances. |
| Runtime Adapter | Translate GixGiz contracts into provider-specific APIs, CLI operations and status semantics. |
| Runtime Abstraction | Stable capability-oriented interfaces consumed by the gateway, packs and model manager. |
| Provider capability descriptor | Declares chat, embeddings, vision, tool calling, model pull, cancellation and metrics support. |

Application → GixGiz Runtime API → Adapter Registry\
├─ OllamaAdapter\
├─ LlamaCppAdapter\
├─ LMStudioAdapter\
└─ VllmAdapter

## Core runtime contract

- Lifecycle: detect, install, configure, start, stop, restart, update, health and version.

- Models: list, import, pull, verify, delete, load, unload and inspect metadata.

- Inference: chat, completion, embeddings, multimodal input, structured output, streaming and cancellation.

- Operations: resource metrics, logs, active requests and capability negotiation.

- Security: bind only to approved local interfaces by default and authenticate external clients through GixGiz.

Not all runtimes implement every operation. The adapter reports unsupported capabilities explicitly. The planner selects a provider only when the requested capability set is satisfiable, avoiding fake universal interfaces.

6\. Model Catalogue, Storage and Resource Scheduling

**Document 2 of 6 \| Architecture \| Page 6 of 15**

## Model catalogue and identity

GixGiz maintains a normalized catalogue independent of runtime naming. A model record contains family, task specialties, licence, formats, parameter class, quantization, context, architecture, checksums, source, safety notes, compatibility rules and estimated resource profiles. Runtime-specific identifiers map to a canonical model identity.

## Storage architecture

- Content-addressed blobs prevent duplicate downloads where formats and licences permit reuse.

- Storage locations may include internal disk or an approved external SSD with availability checks.

- Incomplete downloads use temporary files, resumable ranges and checksum validation before registration.

- Reference counts prevent deletion of a model required by installed packs.

- Archive and cleanup suggestions are based on size, last use and dependency impact.

## Resource scheduler

The scheduler coordinates model loading, memory budgets and request priority. It can unload idle models, queue expensive jobs, select a smaller compatible model under pressure and preserve interactive responsiveness. It does not silently downgrade a quality-sensitive task unless user policy permits it.

| **Signal** | **Possible scheduler action** |
|----|----|
| VRAM pressure | Unload idle image model or reduce GPU offload for next request. |
| Interactive coding request | Prioritize low-latency code model over background indexing. |
| Thermal or battery constraint | Use efficient profile and delay non-urgent background work. |
| Pack requires embeddings | Reuse compatible resident embedding model or schedule load. |
| Context exceeds model limit | Chunk through workflow or explain limitation; do not truncate silently. |

7\. AI Gateway and Inference Request Flow

**Document 2 of 6 \| Architecture \| Page 7 of 15**

## Unified local gateway

The AI Gateway is the only supported inference entry point for desktop features, AI packs and external clients. It authenticates callers, resolves requested capabilities, applies policy, selects runtime/model, manages context, streams output, records usage and normalizes provider responses.

Client request\
→ authenticate client and scope\
→ validate task and attachments\
→ resolve model capability\
→ reserve resources\
→ invoke runtime adapter\
→ stream normalized events\
→ account usage and release resources

## API surfaces

| **Surface** | **Purpose** |
|----|----|
| GixGiz native API | Full features: sessions, packs, tools, permissions, workflows and diagnostics. |
| OpenAI-compatible API | Compatibility for existing local clients using chat, embeddings and model listing. |
| MCP server/client bridge | Expose approved GixGiz tools and consume external MCP tools under policy. |
| CLI | Scriptable management and agent interaction for technical users and automation. |
| Extension transport | Authenticated WebSocket/HTTP or native messaging for VS Code and other desktop apps. |

## Context service

Context assembly is separated from inference. It collects conversation state, selected files, retrieved passages, IDE metadata, tool results and user instructions, then applies token budgets and provenance labels. Sensitive data classifications are preserved so future cloud routing can exclude protected content.

8\. Secure Tool Runtime and Agent Execution Loop

**Document 2 of 6 \| Architecture \| Page 8 of 15**

## Tool-based system access

Models never receive direct operating-system authority. They emit structured tool requests. GixGiz validates the schema, resolves permissions, evaluates risk, requests confirmation when necessary, executes through a controlled implementation and returns bounded results to the model.

| **Tool family** | **Examples** | **Default posture** |
|----|----|----|
| Filesystem | List, read, write, copy, move, search, create directory. | Scoped to explicitly approved roots. |
| Terminal/process | Run command, stream output, stop process, inspect exit code. | Workspace-bound, timeout-limited, approval for risky commands. |
| Git | Status, diff, branch, commit, restore selected changes. | Repository scope; push requires explicit approval. |
| Documents/data | Parse PDF/DOCX, OCR, query SQLite, transform CSV. | Read-only by default; output to chosen destination. |
| Desktop services | Clipboard, notification, file picker, application launch. | User-initiated and capability-specific. |
| Network/browser | Fetch approved URLs, search, browser automation later. | Disabled offline; domain policy and visible consent. |

## Agent loop

Goal → Plan → Propose tool call → Policy decision → Execute\
→ Observe result → Update plan → Continue / Ask / Finish

Every agent run has an identity, workspace, policy profile, model, budget, cancellation token and audit trail. Maximum steps, wall time, output size and process resources prevent runaway loops. Destructive actions are grouped into a previewable change set whenever possible.

9\. Workflow Engine and AI Packs

**Document 2 of 6 \| Architecture \| Page 9 of 15**

## AI Pack definition

An AI Pack is a task-oriented package that combines UI contributions, workflow definitions, prompt assets, tool requirements, model capability requirements, permissions and optional sandboxed code. Packs reuse platform services instead of shipping their own uncontrolled AI infrastructure.

developer-pack/\
manifest.toml\
workflows/\
prompts/\
ui/\
policies/\
migrations/\
assets/\
optional-worker/

| **Pack** | **Capabilities** |
|----|----|
| Developer | Repository indexing, code chat, edits, tests, terminal, Git and VS Code integration. |
| Document | PDF/DOCX ingestion, OCR, semantic retrieval, comparison, extraction and report generation. |
| Research | Local library, citations, notes, synthesis and optional online connectors later. |
| Voice | Speech-to-text, text-to-speech, dictation and meeting workflows. |
| Media | Local image generation/editing, OCR, metadata and controlled file transformations. |
| Automation | Reusable file, data and desktop workflows with explicit permissions. |

## Workflow engine

Workflows are durable graphs of validated steps: inference, retrieval, tool calls, user approvals, branching, retries and outputs. They persist checkpoints so long tasks can resume after a restart. Packs may expose templates, but the platform owns execution, policy enforcement and logs.

10\. Local Data, Knowledge and Persistence

**Document 2 of 6 \| Architecture \| Page 10 of 15**

## Local persistence domains

| **Store** | **Contents** | **Technology direction** |
|----|----|----|
| Platform database | Settings, machine profiles, jobs, runtimes, models, packs, permissions, audits. | SQLite with migrations and WAL mode. |
| Conversation store | Messages, attachments, provenance, tool events and summaries. | SQLite plus content files; encryption option. |
| Knowledge index | Chunks, embeddings, metadata, source references and access rules. | Embedded vector index or SQLite vector extension. |
| Content store | Models, pack blobs, document extracts, caches and generated outputs. | Versioned directories with content hashes. |
| Secrets store | API keys later, client tokens, signing keys and credentials. | Windows Credential Manager, Keychain or Secret Service. |
| Audit store | Tamper-evident records of approvals, tools, changes and policy decisions. | Append-oriented local log with export controls. |

## Data principles

- Local data stays local by default; telemetry and sync are opt-in and separately described.

- Every indexed chunk retains source, page/line location, timestamp and permission scope.

- Deleting a source offers deletion of derived extracts, embeddings and conversation attachments.

- Backups and migrations are versioned; destructive migrations require a recoverable snapshot.

- Pack data is namespaced to avoid accidental cross-pack reads.

Large binary models are not stored inside SQLite. Database records point to verified content blobs and track ownership, dependencies and lifecycle state.

11\. Permissions, Sandboxing and Trust Architecture

**Document 2 of 6 \| Architecture \| Page 11 of 15**

## Trust boundaries

The primary boundaries are: user versus model output, trusted platform core versus third-party pack code, normal process versus privileged installer helper, approved workspace versus the rest of the filesystem, and local execution versus any future network provider.

| **Control** | **Architecture** |
|----|----|
| Capability permissions | Read files, write files, terminal, network, microphone and other rights granted separately by scope. |
| Risk classification | Read-only, reversible, impactful, destructive and privileged actions trigger increasing approval levels. |
| Sandboxing | Pack workers and tool processes run with OS restrictions, resource limits and restricted environment variables. |
| Package integrity | Signed manifests, checksums, publisher identity, permission review and revocation lists. |
| Prompt injection defence | Tool policy does not trust instructions embedded in files or web content; provenance is marked. |
| Secrets isolation | Secrets are referenced by handles and injected only into approved calls, never copied into prompts by default. |
| Auditability | Tool calls, approvals, file diffs, commands and outputs are recorded with redaction controls. |

## Safe change model

For file and code modifications, GixGiz should stage changes, present summaries or diffs, support selective approval, and retain recovery snapshots where feasible. Terminal commands are parsed and policy-checked, but structured tools are preferred for routine file and Git operations because they are safer and cross-platform.

12\. Extension SDK, Integrations and Marketplace

**Document 2 of 6 \| Architecture \| Page 12 of 15**

## Extension types

| **Extension** | **Role** |
|----|----|
| Runtime adapter | Adds a new local or future cloud inference provider. |
| Tool provider | Adds a structured capability such as CAD, database, email or device control. |
| AI Pack | Adds end-user workflows, UI and domain behaviour. |
| Client integration | Connects an IDE, office suite, browser or CLI to the GixGiz gateway. |
| Model catalogue provider | Publishes verified metadata and compatibility profiles. |
| Policy pack | Defines organization restrictions, approval rules and approved components. |

## SDK contract

- Versioned manifests with semantic versioning and explicit compatibility ranges.

- Capability-based APIs instead of direct access to internal databases or runtime processes.

- Test harness with fake models, temporary workspaces, simulated approvals and deterministic tool outputs.

- Pack linting for undeclared permissions, unsafe paths, missing licences and incompatible dependencies.

- Developer mode with hot reload for UI assets and detailed sandbox logs.

- Publication pipeline with signing, automated scanning, review status and reproducible package metadata.

## Marketplace architecture

The public registry stores metadata, manifests, signatures and package locations. Clients verify signatures locally and resolve dependencies before downloading. Enterprises may use a private registry or mirror and can pin approved versions. A remote registry is useful, but installing and executing a pack remains a local platform operation.

13\. Updates, Observability and Reliability

**Document 2 of 6 \| Architecture \| Page 13 of 15**

## Update domains

| **Domain** | **Strategy** |
|----|----|
| Desktop and core | Signed atomic updates, compatibility handshake and rollback to previous version. |
| Runtime providers | Adapter-controlled update with health verification and version pinning. |
| Models | Catalogue metadata updates separated from optional large model replacements. |
| Packs and tools | Dependency-aware updates with permission-delta review and migration steps. |
| Rules and policies | Small signed updates for compatibility, security blocks and recommendation improvements. |

## Observability

- Structured logs with correlation IDs across UI, gateway, workflow, adapter and tool calls.

- Metrics for latency, tokens, model load time, memory, failures, queue time and tool duration.

- Health state machine: healthy, degraded, unavailable, updating and attention required.

- Local diagnostics bundle that redacts secrets and user content unless explicitly included.

- Optional privacy-preserving telemetry with clear categories and local preview before submission.

## Reliability patterns

Long operations are jobs with persistent checkpoints, idempotency keys and cancellation. Installations use staged downloads, validation and rollback. Runtime crashes trigger bounded restart policies. Database writes use transactions; file moves use temporary paths and atomic rename where supported. The platform never represents a queued or partially completed operation as finished.

14\. Enterprise and Multi-Device Architecture

**Document 2 of 6 \| Architecture \| Page 14 of 15**

## Managed deployment

Enterprise mode adds administration without removing local execution. Organizations distribute a signed configuration describing approved runtimes, model licences, packs, permissions, update channels, storage limits and audit retention. Devices report compliance summaries through an optional management service.

| **Enterprise component** | **Purpose** |
|----|----|
| Admin console | Define policies, groups, approved catalogues and deployment rings. |
| Device agent | Apply signed policy, inventory compliant components and report health. |
| Private registry | Host internal packs, adapters and approved model metadata. |
| Shared knowledge connector | Synchronize approved knowledge indexes while preserving access controls. |
| Audit exporter | Forward selected events to organization logging systems. |
| Offline bundle | Install validated runtimes, models and packs in disconnected environments. |

## Multi-device direction

Consumer sync may later synchronize settings, pack lists and selected encrypted knowledge---not necessarily model binaries. Execution remains device-specific because hardware differs. The capability engine recalculates a suitable configuration per device while preserving user intent and workspace metadata.

| **Enterprise constraint:** Central administration must not create an invisible unrestricted remote-control channel; device actions remain policy-bound and auditable. |
|----|

15\. Future Cloud and Hybrid Execution

**Document 2 of 6 \| Architecture \| Page 15 of 15**

## Extension, not foundation replacement

Cloud and hybrid execution are future provider types connected through the same gateway and capability contracts. The local architecture must classify data and tasks today so that remote routing can later be added without redesigning every pack.

Task Request → Execution Planner\
├─ Local provider\
├─ Cloud provider (user-connected)\
└─ Hybrid workflow (split, redact, merge)

| **Routing factor** | **Policy question** |
|----|----|
| Capability | Can an approved local model complete the task within context and modality limits? |
| Privacy | Does the request contain data prohibited from leaving the device? |
| Consent | Has the user enabled this provider and allowed escalation for this task class? |
| Cost | Is the estimated cost within the configured per-task and monthly budget? |
| Connectivity | Is the provider reachable, and is offline fallback required? |
| Quality/latency | Does the user prioritize local privacy, speed, price or frontier capability? |

## Hybrid patterns

- Local preprocessing and redaction followed by cloud reasoning on the minimum necessary representation.

- Local repository indexing with selected summaries sent to a frontier model after approval.

- Parallel local and cloud responses with comparison, provenance and cost disclosure.

- Cloud failure fallback to local model with a clear quality/context notice.

- Provider-specific retention and privacy terms displayed during connection and policy setup.

| **Architectural endpoint:** GixGiz becomes a provider-neutral execution platform while remaining local-first by default. |
|----|
