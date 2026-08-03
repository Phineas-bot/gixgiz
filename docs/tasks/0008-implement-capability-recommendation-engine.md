# Task 08: Implement capability recommendation engine

- **GitHub issue:** [#8](https://github.com/Phineas-bot/gixgiz/issues/8)
- **Depends on:** Task 07
- **Primary ADRs:** 0001, 0002, 0004
- **Blocks:** Tasks 09 and 10

## Context and user value

Users should not need to understand model sizes, quantization, VRAM offload or runtime compatibility. GixGiz must convert machine evidence into conservative, explainable recommendations without using an LLM as the sole decision-maker.

## Desired behavior

Given the same `MachineProfile`, user preferences, catalogue version and rule-set version, the engine returns the same capability report and installation plan with reasons, warnings, confidence and fallback choices.

## In scope

- Versioned model/runtime catalogue metadata required for v0.1 planning.
- Deterministic hard-constraint and preference-scoring pipeline.
- Conservative RAM, VRAM, storage and context estimates.
- Workload tiers for local text/chat and coding-oriented use.
- Recommended plan, smaller/faster fallback and optional larger/slower alternative.
- Confidence, reasons, warnings and explicit unknown handling.
- Rule/catalogue provenance in persisted plan metadata.
- Rust unit/property tests with representative machine fixtures.
- Flutter capability and recommendation presentation.

## Out of scope

- Downloading or installing runtimes/models.
- Live benchmarks as a hard dependency.
- Voice, image-generation or full AI Pack planning.
- Remote catalogue service or automatic unsigned rule updates.
- LLM-generated compatibility decisions.

## Architecture constraints

- Separate hard compatibility constraints from preference scoring.
- Rules consume the versioned `MachineProfile`; they do not query Windows directly.
- Catalogue identity is independent from Ollama-specific names.
- Same inputs and versions produce the same output.
- Unknown evidence reduces confidence or blocks unsafe plans; it is never guessed.

## Security and privacy constraints

- Planning is local and requires no machine-data upload.
- Updateable metadata must have integrity/version fields and a future rollback path.
- Display licences and source provenance required for informed installation choices.
- Do not recommend configurations estimated to leave unsafe memory/storage margins.

## Acceptance criteria

- [ ] AC-1: Determinism is proven for fixed machine, catalogue, rules and preferences.
- [ ] AC-2: Output contains recommendation, fallback, resource estimates, confidence, reasons and warnings.
- [ ] AC-3: Hard incompatibilities cannot be overridden by preference scoring.
- [ ] AC-4: Unknown evidence remains visible and affects confidence safely.
- [ ] AC-5: Low-storage and CPU-only cases return actionable conservative plans or explicit no-plan results.
- [ ] AC-6: Recommendations never depend on an LLM as the sole decision-maker.
- [ ] AC-7: Flutter explains outcomes and trade-offs without requiring local-AI vocabulary.
- [ ] AC-8: Catalogue/rule versions are recorded with every generated plan.

## Required tests

- Representative high, medium, low and CPU-only fixtures.
- Missing/unknown VRAM and acceleration.
- Low storage and user-selected external storage.
- Determinism and stable ordering.
- Hard-constraint versus preference conflict.
- Catalogue/rule version change.
- UI cards for recommended, fallback, unknown and unsupported states.

## Validation commands

Run the complete Rust, Flutter, binding and Windows CI-equivalent command set, plus deterministic fixture tests.

## Documentation updates

- Catalogue schema and rule-version policy.
- Resource-estimation assumptions and safety margins.
- User-facing recommendation terminology.

## Completion evidence

Provide fixture-to-output examples, rule/catalogue versions, estimation assumptions, tests, commands and limitations for unverified hardware/runtime combinations.