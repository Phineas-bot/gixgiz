---
title: 1. Delivery Strategy and Foundation
---

*Full-platform staged roadmap*

**Document 5 of 6 \| Development Roadmap \| Page 1 of 3**

## Roadmap principles

- Deliver vertical slices that produce user-visible value and exercise the real architecture.

- Keep one managed runtime first, but define adapter contracts before coupling features to it.

- Ship local-only capability before cloud and enterprise complexity.

- Treat security, updates, recovery and accessibility as product work, not final polish.

- Use feature flags and versioned contracts so experimental modules do not destabilize the core.

| **Stage** | **Scope** | **Exit criteria** |
|----|----|----|
| 0\. Architecture foundation | Monorepo, Flutter shell, Rust core, typed bridge, SQLite, CI, AGENTS.md and ADRs. | App starts; core handshake works; tests and signed build pipeline established. |
| 1\. Machine intelligence | Hardware scan, capability report, model catalogue and deterministic planner. | Real devices receive explainable conservative recommendations. |
| 2\. Managed local AI | Ollama adapter, transactional installation, model download, health and streaming chat. | Beginner reaches working local chat without terminal use. |
| 3\. Operations quality | Resource monitoring, model library, storage management, updates, diagnostics and recovery. | Long-running jobs resume; failures are actionable; alpha suitable for external testers. |
| 4\. Local API and Developer Pack | Gateway, OpenAI-compatible API, VS Code extension, repository context, safe edits, tests and Git. | Local Codex-like flow works with preview, cancellation and audit. |

## Engineering cadence

Each stage is divided into issue-quality tasks with acceptance criteria. A feature is complete only after automated tests, security review, UX failure states, documentation and a reproducible release artifact. Avoid fixed calendar promises until baseline velocity is measured; use milestone scope as the primary control.

2\. Product Evolution: Local Platform to Ecosystem

**Document 5 of 6 \| Development Roadmap \| Page 2 of 3**

| **Stage** | **Major deliverables** | **Key risk reduced** |
|----|----|----|
| 5\. Secure tool runtime | Filesystem, process, terminal and Git tools; scoped permissions; approval centre; staged changes. | AI can act without receiving unrestricted system access. |
| 6\. Workflow engine | Durable agent sessions, plan preview, checkpoints, budgets, retries and output validation. | Reliable multi-step automation beyond one-shot tool calls. |
| 7\. Knowledge and Document Pack | Ingestion, OCR, embeddings, local retrieval, citations, compare and export. | Demonstrates reusable platform services outside coding. |
| 8\. AI Pack system | Manifest, dependency resolver, signing, sandbox workers, install/update/remove and developer mode. | Platform expands without core feature accumulation. |
| 9\. Additional local capabilities | llama.cpp/LM Studio adapters, voice, media, automation and advanced scheduling. | Validates runtime abstraction and broader workload support. |
| 10\. Public SDK and registry | Typed SDKs, test harness, pack CLI, publication review and curated marketplace. | Third parties can build safely against stable contracts. |

## Release ladder

- Prototype: internal developer use; no migration or compatibility promises.

- Alpha: external testers, recovery paths and automatic diagnostics; APIs still experimental.

- Beta: signed updates, migrations, permission stability and documented extension preview.

- 1.0 local platform: stable gateway/runtime/tool contracts, polished setup, supported upgrade path and curated packs.

## Quality gates

Before 1.0, test on representative Windows hardware tiers, then add Linux and macOS support based on provider feasibility. Security gates include installer integrity, local API origin protection, path traversal, command policy, pack sandboxing and prompt-injection red-team scenarios.

| **Scope discipline:** The marketplace is not started until the platform can safely install, authorize, isolate, update and remove one internally developed pack. |
|----|

3\. Cloud, Enterprise and Release Governance

**Document 5 of 6 \| Development Roadmap \| Page 3 of 3**

| **Stage** | **Scope** | **Dependency** |
|----|----|----|
| 11\. Cloud provider preview | BYO API keys, secret store, provider adapters, consent and usage/cost display. | Stable local gateway and data classification. |
| 12\. Hybrid execution | Execution planner, privacy labels, redaction, local/cloud split workflows and fallback. | Provider-neutral session/workflow contracts. |
| 13\. Enterprise pilot | Signed policy, private registry, device inventory, offline bundles and audit export. | Stable package/update/security architecture. |
| 14\. Multi-device services | Encrypted settings/workspace sync, device-specific replanning and selected knowledge sync. | Identity, conflict and privacy design. |
| 15\. Ecosystem maturity | Publisher reputation, vulnerability response, compatibility certification and long-term support channels. | Sustained users and extension developers. |

## Parallel workstreams

- Core platform: architecture, database, jobs, APIs and lifecycle.

- Provider engineering: runtimes, models, benchmarks and compatibility catalogue.

- Trust and security: permissions, sandboxing, signing, secrets and incident response.

- Experience: onboarding, management, agent UI, integrations and accessibility.

- Developer ecosystem: manifests, SDK, tooling, documentation and registry.

- Release engineering: installers, signed updates, CI, diagnostics and platform testing.

## Governance checkpoints

At each major stage, update the Product Vision only when the product direction changes; update Architecture and Module Specifications when contracts change; record irreversible choices as Architecture Decision Records. Deprecations require migration guidance and a compatibility window. Security advisories can revoke vulnerable packages or adapters independently of the main release.

## Immediate starting sequence

1.  Freeze the first architecture contracts and repository rules.

2.  Create foundation plus hardware-scan vertical slice.

3.  Validate recommendations on real machines.

4.  Implement one managed runtime and one model path.

5.  Ship internal local chat before beginning autonomous tools.
