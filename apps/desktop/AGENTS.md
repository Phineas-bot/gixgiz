# GixGiz Desktop Agent Rules

These instructions apply to `apps/desktop/` in addition to the repository root `AGENTS.md`.

## Ownership

- Flutter owns presentation, navigation, localization, accessibility, transient view state, and user intentions.
- Keep hardware, runtime, installer, provider, recommendation, persistence, and readiness rules outside Dart widgets.
- Access the future Rust core only through the typed `CoreClient` boundary. Never invoke shell commands, Ollama endpoints, SQLite, or broad filesystem APIs from Flutter.
- Do not report GixGiz as ready from presentation state alone. Readiness must eventually come from verified core state.

## Local core transport

- Production code launches only the controlled package-relative `gixgiz-core.exe` through `SidecarCoreClient`; widgets continue to depend only on `CoreClient`.
- Keep bearer tokens in memory and the inherited stdin pipe only. Never place them in arguments, URLs, diagnostics, preferences, or test snapshots.
- Require loopback endpoint validation, bounded startup/request/shutdown behavior, and typed safe error mapping.
- Files under `lib/core/generated/` are generated from Rust-owned contracts and must not be hand-edited.
- Regenerate or verify bindings from the repository root with `cargo run -p gixgiz-contracts --example generate_bindings -- --write` or `--check`.

## UI conventions

- Put reusable user-facing strings in `lib/l10n/app_en.arb` and regenerate localization sources.
- Preserve keyboard traversal, visible focus, semantic labels, text scaling, and non-color-only state indicators.
- Keep asynchronous presentation states explicit. Add state tests when introducing loading, ready, empty, degraded, failed, cancelled, or attention-required behavior.
- Keep navigation centralized in `lib/app/app_routes.dart` and `lib/shared/app_shell.dart`.

## Validation

Run from `apps/desktop/`:

```powershell
flutter pub get
flutter gen-l10n
flutter analyze
flutter test
flutter build windows
```

The Windows build requires Visual Studio 2022 or Build Tools with the Desktop development with C++ workload and a Windows SDK.
