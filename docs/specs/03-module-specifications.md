---
title: 1. Platform Core and Configuration
---

*Module-level responsibilities, interfaces and acceptance expectations*

**Document 3 of 6 \| Module Specifications \| Page 1 of 15**

## Purpose

The Platform Core is the composition root and stable domain layer. It initializes configuration, database migrations, service registration, job recovery, event delivery and version compatibility. It contains no runtime-specific implementation and no UI code.

## Responsibilities

- Load layered configuration: defaults, device settings, user settings, enterprise policy and session overrides.

- Start and stop services in dependency order and expose a readiness state.

- Maintain feature flags, API versions, service identities and global cancellation.

- Provide typed domain objects, error taxonomy, clock, filesystem abstraction and event bus.

- Recover interrupted durable jobs and mark unrecoverable states for user attention.

## Key interfaces

Platform::start() -\> Readiness\
Platform::shutdown(grace_period)\
ConfigService::effective_config(scope)\
EventBus::publish(event) / subscribe(filter)\
JobRegistry::recover_pending()

## Data

Stores schema version, application version, device identifier, feature flags, configuration values, service health and durable job metadata. Sensitive values are represented by secret references.

## Acceptance criteria

- A failed optional service degrades cleanly; a failed mandatory service blocks readiness with a specific diagnostic.

- Migrations are transactional and create a backup before irreversible changes.

- Shutdown cancels new work, waits for bounded cleanup and preserves resumable jobs.

2\. Hardware Scanner

**Document 3 of 6 \| Module Specifications \| Page 2 of 15**

## Inputs and outputs

Input is the current device and an optional selected storage path. Output is a versioned MachineProfile containing evidence, timestamps, collection method and confidence for each field.

| **Capability** | **Required behaviour** |
|----|----|
| CPU | Name, architecture, logical/physical cores and relevant instruction support. |
| Memory | Total and available RAM; never confuse virtual memory with physical RAM. |
| GPU | Enumerate devices, vendor, dedicated/shared memory and supported acceleration APIs. |
| Storage | Free/total space, filesystem, removable status and expected model path performance. |
| Software | OS version, drivers, installed runtimes and relevant system prerequisites. |
| Dynamic state | Power source, battery mode and optional thermal/utilization readings. |

## Failure behaviour

Partial results are valid. Unsupported metrics return Unknown with a reason. The scanner must not require administrator access for normal profiling and must not execute commands assembled from user input.

## Interfaces

scan_static() -\> MachineProfile\
scan_dynamic() -\> ResourceSnapshot\
probe_acceleration(kind) -\> ProbeResult\
watch_changes() -\> Stream\<HardwareEvent\>

## Tests

- Fixture-based parsing for OS provider outputs.

- Normalization tests for memory units, GPU identifiers and missing values.

- Integration tests on supported OS CI runners where feasible.

- Privacy test ensuring serial numbers and precise identifiers are excluded unless required and consented.

3\. Capability and Recommendation Engine

**Document 3 of 6 \| Module Specifications \| Page 3 of 15**

## Purpose

Transforms machine evidence, user goals, installed components and catalogue metadata into explainable capability reports and deployment plans.

## Rules

- Separate hard constraints from preference scoring.

- Estimate model memory using model format, quantization, context and runtime overhead.

- Expose confidence and reasons for every recommendation.

- Prefer a safe usable configuration over the largest model that might barely start.

- Support profiles: privacy, efficiency, balanced, performance and advanced custom.

assess(machine, catalogue) -\> CapabilityReport\
plan(goal, profile, constraints) -\> InstallationPlan\
replan(event: HardwareChanged \| StorageChanged \| RuntimeFailure)

| **Output** | **Description** |
|----|----|
| CapabilityReport | Text, vision, embedding, speech and image workload tiers. |
| InstallationPlan | Runtime, models, configuration, dependencies, storage and expected resource use. |
| AlternativePlan | Trade-off options such as smaller/faster or larger/slower. |
| Explanation | Plain-language reasons, warnings and unknowns. |
| BenchmarkRequest | Optional bounded benchmark needed to improve confidence. |

## Acceptance criteria

The same inputs produce deterministic results for a given rule-set version. Recommendations never rely on an LLM as the sole decision-maker. Catalogue and rules are signed and rollback-capable.

4\. Runtime Manager

**Document 3 of 6 \| Module Specifications \| Page 4 of 15**

## Purpose

Owns runtime instances as managed system resources. It selects an adapter, detects existing installations, performs approved lifecycle operations and continuously maps provider-specific state to normalized state.

| **State** | **Meaning** |
|----|----|
| NotInstalled | No supported installation detected. |
| InstalledStopped | Executable present but service not reachable. |
| Starting | Start requested and health not yet confirmed. |
| Ready | Health check and capability negotiation succeeded. |
| Degraded | Reachable but capability, version or model operation is impaired. |
| Updating | Controlled update in progress. |
| Failed | Recovery policy exhausted or installation invalid. |

## Interfaces

detect_all() -\> \[RuntimeInstance\]\
ensure(plan, approval) -\> JobId\
start(id) / stop(id) / restart(id)\
health(id) -\> RuntimeHealth\
update(id, target_version)\
uninstall(id, preserve_models)

## Constraints

- Reuse healthy user installations where compatible instead of duplicating them.

- Do not silently update externally managed runtimes.

- Bind runtime network interfaces safely and prevent unintended LAN exposure.

- Keep privileged installation steps in a narrow helper process.

5\. Runtime Adapter SDK

**Document 3 of 6 \| Module Specifications \| Page 5 of 15**

## Purpose

Defines the provider plug-in contract. Adapters translate normalized operations into provider APIs or CLI commands and report capability differences honestly.

trait RuntimeAdapter {\
descriptor(); detect(); install_spec(); configure();\
lifecycle(); models(); infer(); embeddings(); metrics(); logs();\
}

## Descriptor fields

- Provider and adapter versions; supported operating systems and architectures.

- Supported inference modalities, streaming, tool calling, cancellation and metrics.

- Configuration schema with safe defaults and secret references.

- Model format compatibility and import/pull semantics.

- Known limitations and minimum provider versions.

## Adapter quality requirements

| **Requirement** | **Expectation** |
|----|----|
| Isolation | No provider details cross the interface boundary. |
| Timeouts | Every network/process operation is bounded and cancellable. |
| Error mapping | Provider errors map to stable categories without losing diagnostic context. |
| Contract tests | Shared test suite validates all supported adapter behaviours. |
| Version negotiation | Unsupported provider versions are rejected or placed in degraded mode. |
| Security | No arbitrary shell interpolation; executable paths and arguments are validated. |

Adapters may be first-party or third-party. Third-party adapters run with restricted permissions and require signatures in standard mode.

6\. Model Catalogue and Model Manager

**Document 3 of 6 \| Module Specifications \| Page 6 of 15**

## Responsibilities

- Resolve canonical models to provider-specific artifacts.

- Acquire, verify, register, import, load, unload, archive and remove models.

- Track licences, source provenance, checksums, dependencies and last use.

- Prevent deletion of artifacts required by installed packs or active jobs.

- Expose estimated and measured performance per device/runtime combination.

## Data model

ModelIdentity → ArtifactVariant → LocalArtifact → RuntimeRegistration\
PackRequirement → CapabilityConstraint → ResolvedModel

| **Record** | **Important fields** |
|----|----|
| ModelIdentity | Family, revision, licence, modalities, specialties, context. |
| ArtifactVariant | Format, quantization, size, checksum, source and compatibility. |
| LocalArtifact | Path, verification, storage device, state and reference count. |
| RuntimeRegistration | Provider name, runtime identifier, load state and health. |
| PerformanceProfile | Tokens/sec, load time, memory, context and benchmark version. |

## Acceptance criteria

A model becomes Available only after complete verification and successful provider registration where required. Removal shows reclaimed space and impacted packs before confirmation. Catalogue licences and usage restrictions remain visible.

7\. Download, Installation and Update Engine

**Document 3 of 6 \| Module Specifications \| Page 7 of 15**

## Purpose

Executes downloads, installations and updates as durable, resumable and auditable jobs. It is shared by runtimes, models, packs and platform updates.

## Job stages

1.  Resolve dependencies and disk requirements.

2.  Download to staging with resume, mirror selection and progress.

3.  Verify signature, checksum, licence and expected package identity.

4.  Present permission or system-change approval.

5.  Apply changes using least privilege.

6.  Run post-install health and compatibility checks.

7.  Commit registrations atomically or roll back staged changes.

Job { id, kind, state, steps\[\], progress, cancellation,\
retry_policy, rollback_plan, logs, approval_requests }

| **Concern** | **Requirement** |
|----|----|
| Network interruption | Resume without corrupting already verified ranges. |
| Insufficient disk | Detect before download and during extraction; preserve clean state. |
| Cancellation | Stop safely and remove or retain resumable staging by policy. |
| Privilege | Request only for the exact step that needs it. |
| Rollback | Restore prior runtime/configuration when update validation fails. |
| Supply chain | Reject unsigned or mismatched artifacts in standard mode. |

8\. Resource Scheduler and Performance Monitor

**Document 3 of 6 \| Module Specifications \| Page 8 of 15**

## Purpose

Coordinates scarce CPU, GPU, VRAM, RAM, disk bandwidth and model slots so multiple GixGiz clients remain stable and responsive.

## Scheduling inputs

- Request priority, deadline and interactivity.

- Model load state, estimated memory and provider concurrency.

- Current resource snapshot, power profile and thermal status.

- User preference for latency, quality, energy or background completion.

- Pack-specific guarantees and enterprise limits.

## Actions

| **Condition** | **Action** |
|----|----|
| Model not loaded | Reserve budget, unload eligible idle model, then load. |
| Memory estimate unsafe | Offer compatible alternative or queue; never force likely crash. |
| Background indexing active | Throttle when interactive request arrives. |
| Runtime unresponsive | Cancel, collect diagnostics and trigger bounded recovery. |
| Battery saver | Reduce concurrency, model size or context within declared policy. |
| Overheating | Pause optional workloads and surface device-health warning. |

## Interfaces and metrics

submit(ResourceRequest) -\> Lease\
release(Lease)\
subscribe(ResourceSnapshot)\
benchmark(Model, Runtime, Profile) -\> PerformanceProfile

Leases are revoked only through cooperative cancellation unless the provider crashes. Metrics include queue time, utilization, peak memory, token throughput and cancellation effectiveness.

9\. AI Gateway and Session Service

**Document 3 of 6 \| Module Specifications \| Page 9 of 15**

## Responsibilities

- Authenticate local clients and apply scopes.

- Create sessions and normalize messages, attachments and model events.

- Resolve capability requests to providers and models.

- Assemble bounded context with provenance.

- Stream tokens, tool proposals, status and errors.

- Apply usage budgets, cancellation, concurrency and audit policies.

- Offer GixGiz-native, OpenAI-compatible and MCP-related interfaces.

## Core API

POST /sessions\
POST /responses (stream)\
POST /embeddings\
GET /models\
POST /agent-runs\
POST /approvals/{id}\
POST /jobs/{id}/cancel

| **Security property** | **Implementation direction** |
|----|----|
| Local authentication | Per-client tokens or OS-bound identity, even on loopback. |
| Origin control | Do not allow arbitrary web pages to call privileged local APIs. |
| Attachment scope | Resolve file handles through approved access grants. |
| Output limits | Bound stream size and retain cancellation control. |
| Compatibility API | Expose only safe subset unless client has enhanced GixGiz scopes. |

## Acceptance criteria

All requests receive a correlation ID. Provider-specific output is normalized. Cancelling a session propagates to workflow, adapter and tool operations. The gateway remains usable when optional packs are disabled.

10\. Tool Runtime and Permission Service

**Document 3 of 6 \| Module Specifications \| Page 10 of 15**

## Tool contract

ToolDescriptor { id, version, input_schema, output_schema,\
permissions, risk_classifier, timeout, isolation }\
ToolExecutor::propose → authorize → execute → observe

## Permission service

- Grant by capability, resource scope, duration and calling pack/client.

- Support one-time, session, workspace and persistent grants.

- Compute permission deltas on pack updates.

- Allow global denials such as "never permit terminal" or "never access network."

- Record approvals and provide a revocation dashboard.

## Built-in tools

| **Tool** | **Representative operations** |
|----|----|
| Filesystem | List/read/write/search/copy/move/create; delete requires stronger policy. |
| Process/terminal | Run bounded commands, stream output, cancel and inspect exit status. |
| Git | Status, diff, stage, commit and restore selected changes. |
| Document/data | Parse, OCR, convert, query and export. |
| Desktop | Clipboard, notifications, file picker and launch approved application. |
| Network later | HTTP fetch, browser and connectors under domain policy. |

Structured tools are preferred over shell commands. File writes should support expected-current-hash checks to avoid overwriting concurrent user edits.

11\. Agent and Workflow Engine

**Document 3 of 6 \| Module Specifications \| Page 11 of 15**

## Agent session

An AgentSession binds a user goal to a model, tool set, workspace, policy, budget and durable state. It supports plan preview, iterative execution, user questions, pause, resume, cancellation and final report.

## Workflow graph

Step types: Infer \| Retrieve \| Tool \| Approval \| Transform \|\
Branch \| Loop(bound) \| HumanInput \| EmitOutput

| **Requirement** | **Behaviour** |
|----|----|
| Durability | Checkpoint after impactful steps and before approvals. |
| Bounded autonomy | Maximum steps, time, tokens, tool calls and cost where applicable. |
| Observability | Display current step, proposed actions and completed results. |
| Recovery | Resume idempotent steps; request review for uncertain partial effects. |
| Evaluation | Workflow-specific success tests and output validation. |
| Provenance | Record which model, source and tool contributed to outputs. |

## Coding-agent workflow

8.  Index or inspect repository under granted workspace.

9.  Form a plan and identify tests.

10. Propose file edits and commands.

11. Apply staged changes, run tests and observe failures.

12. Iterate within budget.

13. Present diff, tests and unresolved risks for acceptance.

12\. AI Pack Manager and Marketplace Client

**Document 3 of 6 \| Module Specifications \| Page 12 of 15**

## Responsibilities

- Discover, resolve, download, verify, install, enable, disable, update and remove AI Packs.

- Read manifests, calculate dependency and permission changes, and provision required capabilities.

- Maintain pack data namespaces, migrations and compatibility state.

- Integrate with public, private or offline package catalogues.

- Expose publisher, licence, signature, reviews and security status.

manifest: identity, version, platform_range, entrypoints,\
capabilities, model_requirements, tools, permissions, dependencies,\
data_schema, migrations, licences, publisher_signature

| **Lifecycle event** | **Platform behaviour** |
|----|----|
| Install | Resolve dependencies, show storage and permissions, verify and activate. |
| Enable | Start workers, register UI/workflows and validate capabilities. |
| Update | Review new permissions, migrate data and preserve rollback. |
| Disable | Stop jobs and hide entrypoints without deleting data. |
| Remove | Show dependent packs and offer data retention/deletion choices. |
| Revoke | Block vulnerable signature/version and explain remediation. |

A "Developer Pack" can depend on a code model capability, filesystem/Git tools, IDE integration and workflow definitions without hard-coding Ollama or a specific model name.

13\. Knowledge, Document and Data Services

**Document 3 of 6 \| Module Specifications \| Page 13 of 15**

## Services

| **Service** | **Responsibility** |
|----|----|
| Ingestion | Detect type, extract text/tables/images, OCR where required and preserve source references. |
| Chunking | Create modality-appropriate chunks with stable identifiers and access metadata. |
| Embeddings | Select compatible local embedding model and batch through gateway. |
| Vector index | Store vectors, filters and source links in an embedded local index. |
| Retrieval | Hybrid lexical/vector search, reranking and token-budgeted evidence selection. |
| Document actions | Summarize, compare, question-answer, extract, translate and generate outputs. |
| Data tools | CSV/SQLite inspection and safe transformation workflows. |

## Privacy and correctness

- Indexes inherit source permissions and pack namespace.

- Answers identify sources and never claim retrieval evidence that was not supplied.

- Deleting or moving sources updates derived data predictably.

- OCR confidence and parsing failures are visible.

- Large-document workflows summarize hierarchically rather than silently truncating.

## Interfaces

ingest(Source, Policy) -\> CorpusItem\
index(CorpusItem, EmbeddingProfile) -\> IndexJob\
retrieve(Query, Filters, Budget) -\> EvidenceSet\
export(Result, Format, Destination)

14\. Integration and Developer SDK

**Document 3 of 6 \| Module Specifications \| Page 14 of 15**

## Client integrations

| **Integration** | **Capabilities** |
|----|----|
| VS Code | Chat, inline edits, code actions, repository context, agent tasks and diff review. |
| JetBrains | Equivalent IDE bridge through GixGiz API. |
| CLI | Manage runtimes/models/packs and launch scripted agent sessions. |
| Office/document apps | Selected text, document context and generated outputs through add-ins. |
| Browser extension later | Send selected page content under explicit permission. |
| MCP | Consume approved tools and expose GixGiz capabilities to compatible clients. |

## Developer SDK

- Typed clients for Rust, Dart/Flutter, TypeScript and Python where demand exists.

- Stable schemas generated from one source of truth.

- Authentication, streaming, retries and error handling built into clients.

- Local emulator and fake gateway for testing without downloading large models.

- Sample packs and integrations with secure defaults.

## VS Code flow

Extension gathers approved editor context → GixGiz Gateway\
→ Developer Pack workflow → local model/tool runtime\
→ streamed explanation/edit proposal → VS Code diff UI

The extension never calls Ollama directly. Switching runtimes or models occurs behind GixGiz and does not require changing the IDE integration.

15\. Enterprise, Cloud/Hybrid and Cross-Cutting Services

**Document 3 of 6 \| Module Specifications \| Page 15 of 15**

## Cross-cutting modules

| **Module** | **Role** |
|----|----|
| Policy engine | Evaluate capabilities, risks, organization rules and user grants. |
| Audit/logging | Structured events, redaction, diagnostics and optional export. |
| Secret service | OS-backed secret handles and controlled injection. |
| Identity/client registry | Trusted local clients, pack identities, publishers and admins. |
| Notification service | Progress, approvals, failures and completed-work notifications. |
| Telemetry service | Opt-in metrics with category controls and local preview. |
| Localization/accessibility | Language resources, keyboard use, screen-reader semantics and scaling. |

## Enterprise modules

The managed-device agent validates signed policy, approved registries, versions and storage constraints. It does not bypass local permission architecture; instead it supplies additional policy constraints. Admin services support groups, deployment rings, private packs and audit export.

## Cloud and hybrid modules

A CloudProviderAdapter implements normalized inference and provider metadata. Execution Planner considers capability, privacy labels, consent, connectivity, cost and latency. A HybridCoordinator can keep source data local, send approved redacted representations remotely and merge results with provenance.

## Definition of full-platform completeness

- Every major capability is reachable through stable contracts and policy checks.

- Provider, pack and tool extension points are versioned and testable.

- Local-only mode remains fully functional after cloud features are added.

- Enterprise controls compose with, rather than replace, end-user transparency.
