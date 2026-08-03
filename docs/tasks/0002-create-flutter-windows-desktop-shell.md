# Task 02: Create Flutter Windows desktop shell

- **GitHub issue:** [#2](https://github.com/Phineas-bot/gixgiz/issues/2)
- **Depends on:** Task 01
- **Primary ADRs:** 0001, 0002, 0005
- **Blocks:** Task 04 and Task 06

## Context and user value

GixGiz needs a minimal Windows desktop application that can host onboarding and later platform workflows without embedding systems logic in widgets.

## Desired behavior

A user can launch a branded GixGiz Windows application that renders a small diagnostic shell with accessible navigation and explicit presentation states. The shell does not yet communicate with Rust.

## In scope

- Flutter project at `apps/desktop/` with Windows support.
- Product identity `GixGiz` and package namespace consistent with ADR 0005.
- Minimal application composition, routing/navigation foundation, theme and localization structure.
- Typed UI states: loading, ready-placeholder, degraded-placeholder, failed and cancelled.
- A simple Foundation screen explaining that the core is not connected yet.
- Keyboard navigation, visible focus and semantic labels.
- Widget tests and a Windows launch/build check.
- `apps/desktop/AGENTS.md` with local UI rules.
- Exact setup, test and build commands in README documentation.

## Out of scope

- Rust crates or sidecar process.
- Flutter–Rust transport.
- SQLite, hardware scanning, recommendations, Ollama, models or chat.
- Final product navigation for deferred Packs, Voice, Media or Agents.
- Production visual design, installer or code signing.

## Architecture constraints

- Widgets render state and emit intentions only.
- No shell commands, direct filesystem management, SQLite or provider calls in Flutter.
- Core access must be represented by an interface that Task 04 can implement; do not fake a successful core response.
- Deferred platform features must not appear as inactive primary navigation.

## Security and privacy constraints

- Request no broad permissions.
- Make no network calls.
- Store no secrets or personal information.
- Do not add analytics or telemetry.

## Acceptance criteria

- [ ] AC-1: `flutter build windows` produces a launchable development build on a supported Windows environment.
- [ ] AC-2: The application displays GixGiz branding and a minimal Foundation screen.
- [ ] AC-3: UI code contains no system, provider, database or recommendation business logic.
- [ ] AC-4: Keyboard navigation, visible focus and basic semantic labels are present.
- [ ] AC-5: Widget tests cover loading, placeholder-ready, degraded, failed and cancelled states.
- [ ] AC-6: Strings are prepared for localization and layouts tolerate text scaling.
- [ ] AC-7: Exact Flutter setup, analysis, test, run and build commands are documented.

## Required tests

- Application bootstrap widget test.
- State rendering tests for each required presentation state.
- Navigation and keyboard-focus tests.
- Semantics test for the primary status and actions.

## Validation commands

```bash
cd apps/desktop
flutter pub get
flutter analyze
flutter test
flutter build windows
```

## Documentation updates

- Root or desktop development guide.
- `apps/desktop/AGENTS.md`.
- Any application-identity notes affected by generated Windows files.

## Completion evidence

Report the Flutter version, Windows runner generated, commands executed, test names, accessibility checks and any environment-specific build limitation.