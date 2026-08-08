# Persistence Crate Instructions

This file applies to `crates/gixgiz-persistence/` and strengthens the root and
`crates/AGENTS.md` policies.

- This crate is the only SQLite ownership boundary. Do not expose a raw
  `rusqlite::Connection` or arbitrary SQL execution API.
- Keep migrations immutable, ordered, repository-owned, and transactional.
  Never edit an applied migration; add a new version instead.
- Back up the current database before an irreversible migration and fail closed
  if the backup cannot be created and verified.
- Store structured, bounded metadata only. Never store secrets, prompts,
  conversations, logs, downloaded artifacts, model binaries, or large blobs.
- All tests must use an explicit temporary data-root override. Tests must not
  resolve or touch the user's `%LOCALAPPDATA%` tree.
- Map filesystem and SQLite failures to stable, sanitized errors. Raw SQL,
  paths, and underlying error text are diagnostic causes only and must not cross
  the application boundary.
