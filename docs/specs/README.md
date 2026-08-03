# GixGiz Specifications

This directory contains the authoritative product and engineering specifications for GixGiz.

## Documents

1. [`01-product-vision`](./01-product-vision.md) — product purpose, users, principles and complete platform direction.
2. [`02-architecture`](./02-architecture.md) — layers, trust boundaries, process topology and major contracts.
3. [`03-module-specifications`](./03-module-specifications.md) — module responsibilities, interfaces, states and acceptance expectations.
4. [`04-ui-ux-specification`](./04-ui-ux-specification.md) — user flows, status language, accessibility and recovery behavior.
5. [`05-development-roadmap`](./05-development-roadmap.md) — staged implementation and release gates.
6. [`06-ai-coding-playbook`](./06-ai-coding-playbook.md) — coding-agent workflow, testing, review and evidence standards.
7. [`07-v0.1-foundation-specification`](./07-v0.1-foundation-specification.md) — the currently active Windows-first release scope.

The specifications use stable Markdown filenames for repository navigation and maintenance.

## Authority

When documents conflict, follow the precedence defined in the root [`AGENTS.md`](../../AGENTS.md). Repository specifications are authoritative over remembered chat discussions.

Do not describe roadmap functionality as implemented. Update a specification when its contract changes and create an ADR for important architectural decisions.
