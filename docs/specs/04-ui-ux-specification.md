---
title: 1. Experience Principles and Information Architecture
---

*Desktop user experience for the complete platform*

**Document 4 of 6 \| UI/UX Specification \| Page 1 of 5**

## Experience objective

GixGiz must make a technically complex platform feel calm, trustworthy and task-oriented. Beginners should be able to act without understanding infrastructure, while experts can inspect and control every important decision. The interface uses progressive disclosure rather than separate products for novice and advanced users.

## Design principles

| **Principle** | **UX implication** |
|----|----|
| Intent first | Primary actions are Chat, Code, Documents, Automate and Add Capability---not runtime and quantization selectors. |
| Explain decisions | Recommendations show plain reasons, trade-offs and confidence. |
| Preview impactful actions | File changes, commands, installations and cloud escalation are reviewed before execution according to risk. |
| Visible locality | The UI clearly shows Local, Offline, Network Tool or future Cloud for each task. |
| Recoverable progress | Downloads and workflows show checkpoints, pause/cancel, retry and meaningful failure states. |
| No false readiness | A runtime is "Ready" only after health checks and a model is "Available" only after verification. |

## Navigation

Home\
├─ Workspaces: Chat \| Developer \| Documents \| Research \| Automation\
├─ Library: Models \| Knowledge \| Outputs\
├─ AI Packs / Store\
├─ Activity: Jobs \| Agent Runs \| Audit\
├─ System: Health \| Runtimes \| Storage\
└─ Settings: Privacy \| Permissions \| Integrations \| Advanced

## Modes

Beginner mode uses outcome language and recommended settings. Advanced mode adds provider, model, context, resource and log controls. Switching modes never changes security policy or hides past activity.

2\. Onboarding and Intelligent Setup

**Document 4 of 6 \| UI/UX Specification \| Page 2 of 5**

## First-run sequence

1.  Welcome: explain local-first operation and request no broad permissions.

2.  Scan: display categories being checked and allow retry for unavailable metrics.

3.  Capability result: translate hardware into practical workload levels.

4.  Goal selection: general use, coding, documents, research, voice, media or custom.

5.  Recommended plan: show components, storage, expected performance, licences and alternatives.

6.  Approval: separate normal downloads from privileged runtime installation.

7.  Installation: step-by-step progress with pause, cancellation and recovery.

8.  Verification: run health test and short benchmark, then open the chosen workspace.

## Recommendation card

| **Field** | **Example** |
|----|----|
| Outcome | Fast local coding and general chat. |
| Plan | Ollama + compatible 7--9B model + Developer Pack. |
| Resources | 7.2 GB download; approximately 6 GB RAM/VRAM use depending on context. |
| Expectation | Comfortable interactive responses; large-repository tasks may be slower. |
| Confidence | High, based on detected GPU and verified driver. |
| Alternatives | Smaller/Faster and Larger/Better quality cards. |

## Existing installation path

When a runtime or model already exists, GixGiz offers "Use existing," "Manage through GixGiz," or "Keep external." It explains what management implies and avoids duplicate installation by default.

| **Critical UX rule:** Never display terminal commands as the normal path. Diagnostics may reveal them in Advanced mode, but users act through validated controls. |
|----|

3\. Home, Management and Daily Use

**Document 4 of 6 \| UI/UX Specification \| Page 3 of 5**

## Home dashboard

- Task launcher with recent workspaces and suggested next actions.

- AI status strip: Ready/Degraded, Local/Offline, active model and current resource profile.

- Resume cards for downloads, indexing and paused agent runs.

- Storage and health warnings only when actionable; avoid constant technical noise.

- Pack shortcuts such as Developer, Document or Voice after installation.

## Management screens

| **Screen** | **Primary user actions** |
|----|----|
| Models | Search, compare, install, benchmark, choose defaults, archive and remove. |
| Runtimes | See managed/external status, version, health, update, restart and diagnostics. |
| AI Packs | Discover, inspect permissions/dependencies, install, update, disable and remove. |
| Storage | Move model library, clean temporary files, identify large/unused items. |
| System Health | View CPU/GPU/RAM, active jobs, model load state and performance history. |
| Permissions | Review grants by pack, capability and folder; revoke or change confirmation level. |

## Daily chat and document use

Chat supports model-independent sessions, attachment chips, source citations, stop generation and visible locality. Document workspace guides users through selecting files, indexing, asking questions and exporting outputs. Long ingestion jobs remain visible outside the workspace and can be resumed.

## Status language

Use "Preparing AI," "Checking runtime," "Downloading model," "Verifying" and "Ready." Reserve provider names and technical settings for details. Error messages pair a plain explanation with one recommended action and an expandable diagnostic section.

4\. Agent Workspaces, Approvals and Integrations

**Document 4 of 6 \| UI/UX Specification \| Page 4 of 5**

## Agent workspace

Agent tasks use a three-pane or staged experience: goal and conversation; current plan/activity; changes and outputs. The user can inspect what the model knows, what tools are allowed and what step is running.

## Approval design

| **Risk level** | **Interaction** |
|----|----|
| Read-only | May proceed under active workspace grant; activity remains visible. |
| Reversible change | Show grouped plan; allow approve once, always for this workspace, or deny. |
| Destructive | Always require explicit confirmation with exact files/data affected. |
| Privileged/system | Separate system dialog and platform explanation; no hidden elevation. |
| Network/cloud future | Show destination, data summary, provider and estimated cost before sending. |

## Coding integration

VS Code displays GixGiz chat, code actions and agent status inside the editor. Edits appear as standard diffs; terminal commands show command, working directory and output. The user can accept selected files, revert the GixGiz change set and open the same run in the desktop Activity screen.

## Automation plan preview

Organize Downloads\
• Create 5 folders\
• Move 128 files\
• Rename 12 duplicates\
• No files will be deleted\
\[Review items\] \[Approve\] \[Edit plan\] \[Cancel\]

Progress must remain interruptible. When the user cancels, the interface explains which steps completed, what was rolled back and what may need review.

5\. Accessibility, Errors and Advanced/Enterprise UX

**Document 4 of 6 \| UI/UX Specification \| Page 5 of 5**

## Accessibility

- Full keyboard navigation, visible focus, scalable text and no colour-only status distinctions.

- Screen-reader names for model status, progress, resource charts and approval controls.

- Reduced-motion setting and non-animated progress alternative.

- Plain-language mode and terminology help for unavoidable technical concepts.

- Localization architecture that supports long translated text without clipping.

## Error and recovery states

| **Scenario** | **UX response** |
|----|----|
| Download interrupted | Show retained progress and Resume; do not restart from zero silently. |
| Runtime unhealthy | Offer Restart, View cause, Repair or Use alternative. |
| Insufficient memory | Explain expected requirement and offer smaller model or close-workload guidance. |
| Permission denied | Name the denied capability and open the exact permission setting. |
| Partial agent changes | Show completed actions, remaining plan and recovery/revert choices. |
| Corrupt package | Quarantine, explain verification failure and offer trusted re-download. |

## Advanced and enterprise experience

Advanced mode exposes runtime endpoints, context, quantization, offload, logs, API clients and policy evaluation. Enterprise-managed settings are labelled with the organization source and cannot be mistaken for technical failure. Users can still see which rules affected a decision.

## UX validation

- Usability tests with non-technical users for setup and permission comprehension.

- Task tests for developers using VS Code integration and diff approval.

- Failure-injection tests for cancelled downloads, runtime crashes and partial workflows.

- Accessibility audit for onboarding, management tables, dialogs and streaming content.
