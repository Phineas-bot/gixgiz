# Windows CI development guide

Task 06 adds one required GitHub Actions job, **Windows validation**, in
`.github/workflows/ci.yml`. It runs for every pull request, every push to
`main`, and manual `workflow_dispatch` runs. Concurrency is scoped to the
workflow and pull request or Git ref, so a newer run cancels superseded work
without suppressing a completed failure.

## Permissions and supply chain

The workflow has only `contents: read`. It does not request repository secrets,
write GitHub state, sign binaries, publish releases, or deploy. Direct action
dependencies are pinned to immutable commit SHAs; comments retain the reviewed
major version for update tooling and human review.

Rust is selected by the repository-owned `rust-toolchain.toml`, currently
`1.97.1` with `rustfmt`, `clippy`, and the Windows MSVC target. Flutter is pinned
to stable `3.44.8`, which supplies Dart `3.12.2`. Toolchain upgrades must update
the repository documentation and pins in the same reviewed change, then pass
the complete local and CI validation suite.

## Required validation

The Windows job runs these commands from the repository root:

```powershell
cargo run -p gixgiz-contracts --example generate_bindings -- --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

The binding check fails if the Rust-owned JSON Schema or checked Dart models
drift. The workspace test command includes the temporary-database SQLite,
migration ordering, rollback, backup, locking, and newer-schema refusal tests.

The same job then runs from `apps/desktop`:

```powershell
flutter pub get
flutter gen-l10n
flutter analyze
flutter test
flutter build windows
```

The Windows build also compiles the release Rust sidecar with Cargo `--locked`.
CI verifies that both `GixGiz.exe` and `gixgiz-core.exe` are present in the
Release runner directory.

Required CI remains deterministic. Tests use fakes, temporary databases, and
ephemeral loopback listeners; they do not call Ollama, inspect real hardware,
or use external services. Future provider and real-device tests require a
separate explicitly approved manual lane and must not weaken these checks.

## Caches

Cargo registry, Git dependency, and root build caches are keyed by runner OS,
architecture, `Cargo.lock`, and `rust-toolchain.toml`. The Flutter SDK cache is
keyed by OS, architecture, channel, and exact Flutter version. The pub cache is
placed under the runner temporary directory and keyed by `pubspec.yaml` and
`pubspec.lock`.

Caches never include `%LOCALAPPDATA%\GixGiz`, SQLite files, logs, `.env` files,
credentials, signing material, or the packaged Windows bundle.

## Unsigned diagnostic artifact

After a successful build, CI uploads the Release runner directory as
`gixgiz-windows-unsigned-dev` for seven days. Database, log, environment,
private-key, and certificate patterns are excluded. The artifact is unsigned,
intended only for development diagnosis, and is not a release distribution.
Signing and publication require a separate approved release workflow.

## Controlled failure check

Before accepting a workflow change, prove a required command fails by adding a
temporary formatting or test failure on the task branch, observing the matching
Windows validation step fail, and then restoring the source in a follow-up
commit. Never merge or leave intentional failures in `main`.

For a local pre-push probe, temporarily add an unformatted Rust source file and
confirm `cargo fmt --all -- --check` exits nonzero, then remove the file. This
proves the command detects drift, but a GitHub-hosted lane is verified only when
the temporary failing commit is pushed and its Actions run is recorded.
