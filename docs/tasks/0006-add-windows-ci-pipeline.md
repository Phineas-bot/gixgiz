# Task 06: Add Windows CI pipeline

- **GitHub issue:** [#6](https://github.com/Phineas-bot/gixgiz/issues/6)
- **Depends on:** Tasks 02–05
- **Primary ADRs:** 0002, 0003, 0004, 0005
- **Blocks:** Tasks 07–11

## Context and user value

The Windows-first foundation must be validated automatically before feature work expands. CI should run the same checks developers run locally and must expose failures rather than hide them through retries or permissive warnings.

## Desired behavior

Pull requests and pushes to `main` run reproducible Rust, Flutter, contract-generation, migration and Windows-build validation. The workflow does not publish or sign releases.

## In scope

- GitHub Actions workflow on `pull_request` and `push` to `main`.
- Pinned or explicitly versioned action dependencies.
- Stable Rust toolchain installation and caching.
- Flutter stable installation consistent with the repository baseline.
- Rust formatting, Clippy and test lanes.
- Flutter analysis, test and Windows build lanes.
- Generated-contract/binding consistency check.
- SQLite migration/integration tests.
- Artifact upload for the unsigned development Windows build when useful for diagnosis.
- README commands matching CI.
- Concurrency cancellation for superseded branch runs without masking failures.

## Out of scope

- Production signing, publishing, deployment or updater.
- Release tags and public binary distribution.
- Linux/macOS application builds.
- Real Ollama or hardware-dependent tests.

## Architecture constraints

- CI commands must call repository-owned scripts/task runner where practical.
- Deterministic unit and integration tests must not depend on live network providers.
- Real-device/provider tests remain clearly separated from required CI.
- Generated files come from one source and drift fails CI.

## Security and privacy constraints

- Use least-privilege workflow permissions.
- Do not expose signing keys, API keys or repository secrets to untrusted PR code.
- Pin third-party actions by trusted version or commit according to repository policy.
- Build artifacts must contain no local databases, secrets or test personal data.
- Dependency caches must not include credentials.

## Acceptance criteria

- [ ] AC-1: Pull requests and `main` pushes trigger the required validation.
- [ ] AC-2: Rust formatting, Clippy warnings-as-errors and workspace tests run.
- [ ] AC-3: Flutter dependency restore, analysis, tests and Windows build run.
- [ ] AC-4: Contract/binding drift and migration tests run when applicable.
- [ ] AC-5: Workflow permissions and secret exposure are minimized and documented.
- [ ] AC-6: Local README commands match the CI commands.
- [ ] AC-7: A deliberately failing lint/test is proven to fail the corresponding lane before removal.
- [ ] AC-8: No signing or release publication occurs.

## Required tests

- Validate workflow syntax.
- Demonstrate one controlled failing check on the task branch, then restore it.
- Confirm the unsigned Windows development artifact launches in a supported environment when artifact generation is enabled.

## Validation commands

Run the same repository commands locally:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd apps/desktop
flutter analyze
flutter test
flutter build windows
```

## Documentation updates

- CI badge and command documentation.
- Toolchain/version update policy.
- Description of deterministic versus real-provider test lanes.

## Completion evidence

Provide workflow run links, job names, permissions, cache keys, exact local/CI commands, artifact details and any check that remains environment-dependent.