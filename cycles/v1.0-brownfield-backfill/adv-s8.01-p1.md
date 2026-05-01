---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.01-native-port-handoff-validator.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.01 v1.0
target_file: .factory/stories/S-8.01-native-port-handoff-validator.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 14
findings_high: 4
findings_med: 6
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.01 v1.0 (Pass 1)

## Finding ID Convention

`F-S801-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S801-P1-001: Phantom hooks.json deletion (D-7 already shipped)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002 / T-7
- **Description:** AC-002 and T-7 prescribe deletion of hooks.json command entries for handoff-validator. D-7 architecture has already shipped these hooks in registry-only (native WASM) form. The hooks.json entries targeted for deletion do not exist in the current codebase. Executing this deletion step would be a no-op at best, or would fail if the implementation asserts file existence before deletion.
- **Evidence:** D-7 ("already shipped") is referenced in E-8 epic; hooks-registry.toml is the sole dispatch mechanism post-D-7.
- **Proposed Fix:** Remove the hooks.json deletion AC/Task, or rewrite as a verification assertion ("assert hooks.json does NOT contain handoff-validator entries — already correct per D-7").

#### F-S801-P1-002: BC trace mis-anchor on emit_event postcondition

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-007 → BC-7.03.043 postcondition 1
- **Description:** AC-007 traces to BC-7.03.043 postcondition 1 to ground the emit_event mechanism. BC-7.03.043 postcondition 1 describes empty-result emit behavior (the "hook returns no findings" path), not the emit_event host function invocation contract. The correct anchor for the emit_event mechanism is BC-7.03.044 (or the emit_event-specific BC in the family).
- **Evidence:** Semantic mismatch: "empty-result emit" vs "emit_event host function call" are distinct behavioral clauses; conflating them misleads the implementer about which invariant the test must validate.
- **Proposed Fix:** Replace BC-7.03.043 postcondition 1 reference in AC-007 with the correct BC that specifies the emit_event host function invocation contract.

#### F-S801-P1-003: BC-7.03.042 invariant 2 contradicts AC-006 exit-code claim

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** AC-006 → BC-7.03.042 invariant 2
- **Description:** AC-006 claims the hook exits 0 on graceful jq-missing degradation. BC-7.03.042 invariant 2 states "1=jq-missing-fail-closed," mandating a non-zero exit code on jq absence. The AC and its BC trace directly contradict each other on the exit-code contract.
- **Evidence:** AC-006 text: "exits 0"; BC-7.03.042 invariant 2 text: "1=jq-missing-fail-closed." These are mutually exclusive.
- **Proposed Fix:** Align AC-006 exit-code claim with BC-7.03.042 invariant 2 (exit 1 fail-closed), or update BC-7.03.042 to reflect the correct intended behavior and document the rationale.

#### F-S801-P1-004: AC-005 perf gate scope conflicts with E-8 epic AC-7 Tier 1 exclusion

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-005
- **Description:** E-8 epic AC-7 explicitly excludes Tier 1 hooks from the 20% performance gate. AC-005 nonetheless prescribes a performance measurement comparison gate for this story. This creates a scope conflict where the story imposes a gate that the epic intentionally waived for Tier 1.
- **Evidence:** E-8 v1.7 AC-7: "Tier 1 hooks excluded from 20% performance gate." AC-005 references performance measurement without the exclusion qualifier.
- **Proposed Fix:** Remove or reformulate AC-005 to align with E-8 AC-7's Tier 1 exclusion. If perf measurement is desired for informational purposes, mark it INFORMATIONAL (non-blocking).

### MEDIUM

#### F-S801-P1-005: POLICY 8 — body BC table incomplete vs frontmatter invariant coverage

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** behavioral_contracts frontmatter vs body BC table
- **Description:** BC-7.03.042 has two invariants traced by ACs (invariants 1 and 2). The body BC table references only postcondition 1 of BC-7.03.042, omitting invariant 2. POLICY 8 requires the body table to faithfully represent all anchored clauses.
- **Proposed Fix:** Expand the body BC table to enumerate all anchored clauses (postconditions + invariants) from BC-7.03.042 that are referenced by the ACs.

#### F-S801-P1-006: EC-003 off-by-one threshold parity not test-enforced (no LEN=39/40 fixtures)

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** EC-003
- **Description:** EC-003 asserts threshold parity at the LEN=39/40 boundary but no test fixtures exist at LEN=39 (just below threshold) and LEN=40 (at threshold). Without boundary fixtures the off-by-one is unverified and the test suite cannot catch a fencepost error in the WASM port.
- **Proposed Fix:** Add bats fixtures at LEN=39 (should pass) and LEN=40 (should trigger) to the EC-003 test specification.

#### F-S801-P1-007: T-6 bash removal contradicts registry shell_bypass_acknowledged field

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** T-6 vs hooks-registry.toml capability block
- **Description:** T-6 states "remove bash; retain jq only if needed." The hooks-registry.toml entry lists both `jq` in `binary_allow` and `shell_bypass_acknowledged = true`. Removing bash without removing `shell_bypass_acknowledged` leaves a dangling semantically-void field in the registry.
- **Proposed Fix:** Extend T-6 to also remove `shell_bypass_acknowledged` from the registry entry when bash is removed.

#### F-S801-P1-008: T-2 wasm-bindgen reference incorrect (project uses wasm32-wasi/wasip1)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** T-2
- **Description:** T-2 mentions wasm-bindgen as the WASM binding toolchain. vsdd-factory uses `wasm32-wasi` / `wasm32-wasip1` target directly via vsdd-hook-sdk. wasm-bindgen is a JS/browser interop tool; the WASI ABI does not use wasm-bindgen. This would cause the implementer to install and configure the wrong toolchain.
- **Proposed Fix:** Replace wasm-bindgen with the correct build pipeline: `cargo build --target wasm32-wasip1` via vsdd-hook-sdk crate dependency.

#### F-S801-P1-009: BC-7.03.042 line-range citation stale (820-837 vs actual 885-902)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Body BC table line reference
- **Description:** The body BC table cites `hooks-registry.toml:820-837` for the BC-7.03.042 registry entry. The actual line range in the current registry is 885-902.
- **Proposed Fix:** Update citation to 885-902, or replace line-range citations with stable anchor comments.

#### F-S801-P1-010: Token Budget BC token estimate understated (~400 vs actual ~950/BC)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Token Budget section
- **Description:** Token Budget estimates 3 BCs at ~400 tokens each (~1,200 total). Actual BC files average ~950 tokens each, totaling ~2,850 tokens. The 2.4× underestimate may cause the implementer to underallocate context budget for the BC deep-read phase.
- **Proposed Fix:** Update Token Budget BC estimate to ~950 tokens/BC (~2,850 total for 3 BCs).

### LOW

#### F-S801-P1-011: CAP-022 cross-CAP stretch text duplicated verbatim from S-8.00

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Capability Anchor Justification / Stretch-Anchor Disclosure
- **Description:** The CAP-022 cross-CAP stretch disclosure is copied almost verbatim from S-8.00. The disclosure should be tailored to handoff-validator's specific behaviors and BCs.
- **Proposed Fix:** Rewrite the CAP-022 stretch disclosure with handoff-validator-specific hook name, BCs, and behavioral context.

#### F-S801-P1-012: subsystems excludes SS-04 (target_module is in crates/hook-plugins/)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Frontmatter subsystems field
- **Description:** Story lists `subsystems=[SS-01, SS-07]` but the target_module lives in `crates/hook-plugins/` owned by SS-04 (plugin ecosystem). SS-04 should be included.
- **Proposed Fix:** Add SS-04 to `subsystems` in the frontmatter.

#### F-S801-P1-013: input-hash comment references develop branch instead of E-8 epic content hash

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash comment
- **Description:** The input-hash comment references "develop branch commit" but the authoritative source is the factory-artifacts branch. Convention should reference the E-8 epic file content hash as established by S-8.00.
- **Proposed Fix:** Align input-hash comment with S-8.00 convention (E-8 epic content hash).

### NIT

#### F-S801-P1-014: "WASM crate" vs "native WASM crate" inconsistency

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** Goal section and T-1
- **Description:** Goal section uses "WASM crate" while T-1 uses "native WASM crate." Canonical term per E-8 epic is "native WASM plugin."
- **Proposed Fix:** Standardize on "native WASM plugin" throughout.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 6 |
| LOW | 3 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 14/14 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 14 |
| **Verdict** | FINDINGS_REMAIN |
