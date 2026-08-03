# Task 01: Establish repository and build architecture

- **GitHub issue:** [#1](https://github.com/Phineas-bot/gixgiz/issues/1)
- **Status:** Ready after foundation ADR review
- **Depends on:** None
- **Blocks:** Tasks 02–06

## Context and user value

GixGiz currently has product and engineering specifications but no implementation workspace. This task converts the planning repository into a stable foundation without creating Flutter or Rust application code prematurely.

## Desired outcome

The repository has accepted architectural decisions, predictable text-file behavior, documented development prerequisites, a stable implementation sequence and issue-quality task specifications.

## In scope

- Accepted ADRs 0001–0005.
- `.editorconfig` and `.gitattributes`.
- Stable, normalized specification filenames and working links.
- Root README updates for GixGiz identity, prerequisites, architecture direction and Tasks 01–11.
- `docs/tasks/` specifications linked to GitHub issues.
- Repository naming, package identity, data root and MIT licensing documentation.
- Identification of the task runner/validation command strategy; no empty implementation scaffolds.

## Out of scope

- Flutter application creation.
- Cargo workspace or Rust crates.
- SQLite implementation.
- GitHub Actions workflows.
- Hardware, Ollama, model or chat features.

## Architecture constraints

- Follow ADRs 0001–0005.
- Preserve the local-first modular-monolith direction.
- Do not create speculative future modules or services.
- Repository documents are authoritative over prior chat descriptions.

## Security and privacy constraints

- Do not add secrets, credentials, personal fixtures or machine-specific paths.
- Do not introduce installer, service, firewall or elevation behavior.
- Licence and copyright notices must be explicit and consistent.

## Acceptance criteria

- [ ] AC-1: ADRs 0001–0005 are accepted and indexed.
- [ ] AC-2: Specification paths contain no invisible `U+200B` characters.
- [ ] AC-3: README describes the GixGiz identity, Windows-first foundation and logical Tasks 01–11.
- [ ] AC-4: `.editorconfig` and `.gitattributes` define UTF-8, final newline and predictable line endings.
- [ ] AC-5: `docs/tasks/` contains one task specification per logical Task 01–11.
- [ ] AC-6: The MIT `LICENSE` is present and referenced.
- [ ] AC-7: No Flutter, Rust, SQLite or CI implementation is introduced.

## Required tests and validation

```bash
git diff --check
git ls-files | python scripts/check_no_zero_width_paths.py  # when such script exists
```

Until an automated documentation checker exists, manually verify Markdown links, filenames, issue links and repository identity references.

## Documentation updates

- Root `README.md`.
- `docs/adr/README.md`.
- `docs/tasks/README.md`.
- Relevant product-name references.

## Completion evidence

Return the evidence report required by `AGENTS.md`, including the exact changed files, link checks, unresolved naming references and confirmation that no application code was added.