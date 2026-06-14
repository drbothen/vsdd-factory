---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-002 BLOCKER) EC-002/003/004/005/006 `current_wave = N` load-bearing references replaced with first-wave detection language (wave-group position per sprint-state.yaml OR absence of prior HANDOFF.md on factory-artifacts); test vector `any current_wave` → `any pipeline context`; VP-083 + VP-081 verification property rows updated to first-wave detection semantics — no phantom `current_wave:` field referenced anywhere in body. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-1) Precondition 4 re-anchored: `current_wave` field removed; wave-1 no-op now derives from sprint-state.yaml dependency-order wave-group OR STATE.md `current_step:` for engine context. PC3 + PC8 updated to reflect real substrate (no phantom `current_wave:` field). (DI) TBD-DI replaced with DI-020.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: EC-002/003/004/005/006 phantom current_wave = N → first-wave detection; test vector + VP-083 + VP-081 rows updated to first-wave detection semantics."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: PC4 + PC3 + PC8 re-anchored to real substrate (sprint-state.yaml wave-group order or STATE.md current_step: for engine context); phantom current_wave: field removed; TBD-DI replaced with DI-020; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.14.001: validate-wave-handoff-completeness WASM gate blocks HandoffIncomplete on PostToolUse HANDOFF.md writes; no-op on wave-1 and non-HANDOFF.md writes

## Description

`validate-wave-handoff-completeness` is a native WASM plugin registered as a PostToolUse gate on Write/Edit tool calls that target `HANDOFF.md` on `factory-artifacts`. When fired on a HANDOFF.md write, it validates that all 9 required fields specified in ADR-026 §Decision 2 are syntactically present. It blocks with `HandoffIncomplete` if any field is missing or malformed. It is a strict no-op (returns `Continue`) when `current_wave = 1` OR when the tool call does not target `HANDOFF.md`. This design follows the established factory WASM gate pattern (ADR-008, ADR-014) — deterministic parse-heavy validation with no filesystem or git side effects.

## Preconditions

1. The plugin is registered in `hooks-registry.toml` as:
   ```toml
   [[hooks]]
   event = "PostToolUse"
   plugin = "validate-wave-handoff-completeness"
   tool = "Write|Edit"
   on_error = "continue"
   async = false
   timeout_ms = 5000
   ```
2. The plugin WASM binary exists at `plugins/vsdd-factory/hook-plugins/validate-wave-handoff-completeness.wasm`.
3. The invoking tool call is a Write or Edit targeting `HANDOFF.md` on `factory-artifacts` OR targeting a path that matches `*/HANDOFF.md`.
4. Wave identity context is determinable from real substrate: for product pipelines, `sprint-state.yaml` wave-group ordering derived by `wave-scheduling` skill; for the self-referential engine, STATE.md frontmatter `current_step:` field. No phantom `current_wave:` field is referenced — this field does not exist on STATE.md. The gate uses the wave-group ordinal from sprint-state.yaml (position 1 = wave 1) or derives first-wave status from the absence of a prior HANDOFF.md on factory-artifacts.

## Postconditions

1. **Happy path — all 9 fields present and syntactically valid**: Gate returns `Continue`. Wave close may proceed. No block.

2. **HandoffIncomplete — missing or malformed field**: Gate returns `block_intent = true`, exit code 2, with a structured block message naming each missing or malformed field:
   ```
   HandoffIncomplete: required fields missing or malformed: [<field1>, <field2>, ...]
   ```
   The block message must name ALL failing fields in a single invocation, not just the first.

3. **No-op rule (wave-1)**: When the pipeline context is the first wave (wave-group position 1 per sprint-state.yaml dependency order, OR when no prior HANDOFF.md exists on factory-artifacts, OR when wave context cannot be determined), the gate returns `Continue` unconditionally without parsing HANDOFF.md. No phantom `current_wave:` field is read from STATE.md — that field does not exist. This prevents friction on the first wave where no prior handoff exists.

4. **No-op rule (non-HANDOFF.md write)**: When the PostToolUse event's tool call target path does not match `HANDOFF.md` (case-sensitive), the gate returns `Continue` immediately without reading or parsing any file.

5. **Fuel budget**: Gate completes within `timeout_ms = 5000` on any HANDOFF.md body up to 200 lines (ADR-026 Decision 8 200-line cap). Bodies exceeding 200 lines produce an advisory warning but are still parsed (gate does not hard-fail on length alone).

6. **on_error = "continue"**: A gate crash (WASM panic, fuel exhaustion) results in `Continue` (fail-open). Gate crash is logged to the dispatcher internal log with `plugin.crashed` record.

7. **Field validation scope**: The gate validates field PRESENCE and basic syntactic form only (field key exists; value is non-empty or null only where null is permitted). It does NOT perform anti-fabrication cross-checks against git or filesystem — those are performed by the `wave-handoff` skill (BC-5.41.001). Cross-checks require side effects; the WASM gate is pure-parse per ADR-026 Decision 8.

8. **Wave-1 no-op is unconditional**: Even if `HANDOFF.md` is written with deliberate content on the first wave, the gate does not validate it. Validation only activates when the pipeline is on wave > 1 (or when wave context cannot be determined from real substrate, defaulting to fail-open Continue).

## Invariants

1. **No git or filesystem side effects**: The WASM gate reads only the Write/Edit tool call payload (the content being written). It does NOT exec git, read other files, or access the filesystem. It is a pure-parse WASM function. This is the invariant distinguishing it from the `wave-handoff` skill's anti-fabrication checks.

2. **Block message names ALL failing fields**: A partial block message (naming only the first missing field) is a specification violation. The operator must see the complete failure set in one invocation.

3. **No-op conditions are checked first**: The gate checks the no-op conditions (non-HANDOFF.md target, wave-1) before any parsing. A gate that parses HANDOFF.md and then returns Continue on wave-1 is correct in outcome but wastes fuel unnecessarily.

4. **Field ordering in HandoffIncomplete is deterministic**: Fields are listed in the same order as ADR-026 §Decision 2 schema table for reproducibility in test assertions.

5. **200-line HANDOFF.md cap is advisory**: The gate parses bodies > 200 lines without hard failure but emits `plugin.log` warning with `level: warn` and `message: "HANDOFF.md body exceeds 200-line advisory cap"`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | HANDOFF.md write on first wave (no prior HANDOFF.md on factory-artifacts; or wave-group position 1 per sprint-state.yaml) | Continue (wave-1 no-op); no validation |
| EC-002 | HANDOFF.md write; all 9 fields present; wave-group position 2 per sprint-state.yaml (not first wave — prior HANDOFF.md exists on factory-artifacts) | Continue; no block |
| EC-003 | HANDOFF.md write; `last_verified_develop_sha` field missing; not first wave (prior HANDOFF.md exists on factory-artifacts) | HandoffIncomplete: `["last_verified_develop_sha"]` |
| EC-004 | HANDOFF.md write; `last_verified_develop_sha` present but empty string; not first wave | HandoffIncomplete: field present but empty is treated as malformed |
| EC-005 | HANDOFF.md write; 4 fields missing; wave-group position 3 per sprint-state.yaml (not first wave) | HandoffIncomplete: names all 4 missing fields in one message |
| EC-006 | HANDOFF.md write; `precompact_flush_sha: null`; not first wave (prior HANDOFF.md exists on factory-artifacts) | Continue if null is explicitly permitted in the field schema for wave > 1 (advisory only — anti-fabrication check on null is the skill's job, not the gate's) |
| EC-007 | Edit to `STATE.md` (not HANDOFF.md) | Continue (non-HANDOFF.md no-op); gate fires but immediately returns Continue |
| EC-008 | Gate crashes (WASM panic) | fail-open Continue; plugin.crashed log; compaction/wave-close not blocked |
| EC-009 | HANDOFF.md body is 350 lines (over 200-line cap) | Gate parses it; emits advisory warn; validates all fields normally |
| EC-010 | Wave context cannot be determined (sprint-state.yaml absent; factory-artifacts unreachable; STATE.md unreadable) | fail-open Continue per wave-1 no-op default |
| EC-011 | wave-gate invoked with intent to close wave > 1; no HANDOFF.md exists yet | Gate not triggered (no Write to HANDOFF.md occurred); wave-gate skill is responsible for ensuring HANDOFF.md is written before close — the gate validates the write, not the absence |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Write to HANDOFF.md; first wave (no prior HANDOFF.md on factory-artifacts) | Continue | wave-1-no-op |
| Write to HANDOFF.md; second wave (prior HANDOFF.md exists on factory-artifacts); all 9 fields present + well-formed | Continue | happy-path |
| Write to HANDOFF.md; second wave; `wave_id` missing | `HandoffIncomplete: ["wave_id"]`; exit 2 | missing-field-single |
| Write to HANDOFF.md; second wave; 3 fields missing | `HandoffIncomplete: ["<f1>", "<f2>", "<f3>"]`; exit 2 | missing-field-multiple |
| Write to STATE.md; any pipeline context | Continue (non-HANDOFF.md target) | non-target-no-op |
| WASM panic during field parse | Continue; `plugin.crashed` in dispatcher log | crash-fail-open |
| Write to HANDOFF.md; body = 350 lines; all fields present | Continue + advisory warn in plugin.log | over-cap-advisory |

## Related BCs

- BC-5.41.001 — composes with: this gate validates the HANDOFF.md written by wave-gate/wave-handoff; anti-fabrication cross-checks are the skill's responsibility
- BC-4.13.001 — sibling: verify-factory-lock WASM gate; shares the same WASM plugin pattern (pure-parse, no git side effects, fail-open)
- BC-1.15.001 — depends on: dispatcher routing must handle PostToolUse events for WASM plugins

## Architecture Anchors

- `crates/hook-plugins/validate-wave-handoff-completeness/` — NEW Rust crate; `[[bin]]`-bearing; produces `validate-wave-handoff-completeness.wasm`
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]]` entry for `validate-wave-handoff-completeness`; PostToolUse; Write|Edit; on_error=continue; timeout_ms=5000
- ADR-026 §Decision 8 (WASM for completeness gate; 200-line HANDOFF.md cap) and §Decision 9 (wave-1 no-op rule)

## Story Anchor

S-18.02 (validate-wave-handoff-completeness WASM gate crate + registry)

## VP Anchors

- VP-083 — Completeness Gate Is No-Op on Wave-1 and HANDOFF.md Absent
- VP-081 — Wave Cannot Close Without Verified Handoff (partial anchor — VP-083 covers the no-op leg; VP-081 covers the blocking leg)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-083 | validate-wave-handoff-completeness returns Continue unconditionally on first wave (wave-group position 1 per sprint-state.yaml, OR absence of prior HANDOFF.md on factory-artifacts); also no-op on non-HANDOFF.md writes | unit-test |
| VP-081 | Gate blocks HandoffIncomplete when any required ADR-026 §D2 field is missing on a HANDOFF.md write on a non-first wave (prior HANDOFF.md exists on factory-artifacts, OR wave-group position > 1 per sprint-state.yaml) | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the WASM completeness gate that enforces HANDOFF.md integrity at write time, preventing a partial or incomplete handoff artifact from being committed and corrupting the wave-boundary continuity guarantee; it directly enforces ADR-026 Decision 8 and Decision 9 |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — enforced by this gate blocking incomplete HANDOFF.md writes) |
| Architecture Module | SS-04 (Plugin Ecosystem) — new WASM crate under `crates/hook-plugins/validate-wave-handoff-completeness/` |
| ADR | ADR-026 v1.1 Decision 8 (WASM for completeness gate; deterministic parse-heavy validation; shell for flush), Decision 9 (no-op on wave-1 / HANDOFF.md absent; wave identity from real substrate — sprint-state.yaml wave-group order or factory-artifacts HANDOFF.md presence) |
| Stories | S-18.02 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
