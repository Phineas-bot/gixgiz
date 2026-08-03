# Task 07: Implement hardware scan vertical slice

- **GitHub issue:** [#7](https://github.com/Phineas-bot/gixgiz/issues/7)
- **Depends on:** Tasks 02–06
- **Primary ADRs:** 0001–0005
- **Blocks:** Task 08

## Context and user value

GixGiz must understand the user’s computer before recommending local AI. The scanner produces evidence only; it must not guess capability or require administrator privileges for normal profiling.

## Desired behavior

A user can run a scan from Flutter and see a versioned `MachineProfile` produced by Rust with available CPU, memory, GPU, acceleration and storage evidence. Missing or inaccessible fields remain explicitly unknown with reasons.

## In scope

- Platform-neutral hardware scanner contracts and orchestration.
- Windows provider implementation behind an interface.
- OS/version/architecture, CPU identity and core counts.
- Total and available physical RAM.
- GPU enumeration and dedicated/shared memory only when reliably obtainable.
- Acceleration evidence relevant to the planned runtime.
- Selected storage path capacity, free space, filesystem and removable status.
- Field source, timestamp, confidence/availability and unknown reason.
- Core transport operation and Flutter loading, ready, partial/degraded and failed states.
- Fixture-based parser and normalization tests.
- Privacy review of collected fields.

## Out of scope

- Capability scoring or model recommendation.
- Benchmarks and performance promises.
- Runtime installation or Ollama calls.
- Serial numbers, MAC addresses or unnecessary stable identifiers.
- Elevated driver installation or configuration changes.

## Architecture constraints

- Scanner produces evidence, not recommendations.
- Windows-specific code remains behind system-provider interfaces.
- Partial profiles are valid domain results, not generic failures.
- Unknown VRAM or acceleration support must not be inferred solely from a GPU marketing name.
- Flutter renders the profile and does not reproduce scanner rules.

## Security and privacy constraints

- Normal scanning runs without elevation.
- Commands, if unavoidable, use fixed executables and structured arguments; no user-data shell interpolation.
- Apply timeouts, output limits and cancellation.
- Exclude serial numbers and unrelated identifiers.
- Do not upload machine details or enable telemetry.

## Acceptance criteria

- [ ] AC-1: A versioned `MachineProfile` is returned through the typed core boundary.
- [ ] AC-2: CPU, physical RAM, GPU and selected storage evidence are represented with source and availability.
- [ ] AC-3: Missing metrics return `Unknown` with an explicit reason.
- [ ] AC-4: Normal scans require no administrator rights.
- [ ] AC-5: Unit normalization and provider parsing are fixture-tested.
- [ ] AC-6: Cancellation and provider timeout produce distinct states.
- [ ] AC-7: Flutter displays ready, partial/degraded, failed and cancelled scans accessibly.
- [ ] AC-8: No unnecessary stable device identifiers are collected or logged.

## Required tests

- Representative Windows provider fixtures.
- Unit conversion and physical-versus-virtual memory tests.
- Multiple GPUs, missing GPU, unknown VRAM and unsupported acceleration.
- Low/inaccessible storage path.
- Provider timeout, permission denial and cancellation.
- Transport and Flutter state integration.
- Privacy assertion for excluded identifiers.

## Validation commands

Run all Rust, Flutter, contract and Windows build checks defined by Task 06, plus targeted scanner integration tests on a supported Windows machine.

## Documentation updates

- `MachineProfile` contract documentation.
- Supported and unknown hardware fields.
- Privacy/data-collection note.

## Completion evidence

Include anonymized scan output from representative hardware, exact evidence sources, unknown behavior, tests, commands and unverified hardware combinations.