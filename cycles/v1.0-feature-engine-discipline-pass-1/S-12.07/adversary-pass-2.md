---
document_type: adversarial-review
level: ops
version: "1.0"
status: draft
producer: adversary
timestamp: 2026-05-10T22:00:00Z
phase: engine-discipline-F4-S-12.07
inputs:
  - story_spec
  - bcs (BC-4.12.001..005)
  - vps (VP-073..076)
  - branch_diff (origin/develop..f67a1f1a)
input-hash: "[pending-recompute]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-12.07 adversary pass-2 review (fresh context)"
current_cycle: v1.0-feature-engine-discipline-pass-1
story_id: S-12.07
pass: 2
previous_review: adversary-pass-1.md
verdict: HIGH
findings_count: { critical: 0, high: 6, medium: 6, low: 5, nitpick: 1 }
convergence_reached: false
---

# S-12.07 Adversary Pass-2 (Fresh Context)

## Finding ID Convention

Finding IDs use the format: `ADV-BF1-P02-<SEV>-<SEQ>`. BF1 = cycle prefix; P02 = pass-2.

## Part A — Fix Verification

Reviewer was dispatched with FRESH CONTEXT — no knowledge of pass-1 findings. Re-derived findings from scratch against story spec, BCs, ADR-018, and VPs.

## Part B — New Findings

**Verdict: HIGH.** The implementation has been substantially improved — canonical schema adoption, key rename, panic-free production code, lint configuration, and STATE.md parsing are all sound. However, three independent HIGH-severity issues remain: (a) story spec not amended (frontmatter v1.0, 16 hyphen references); (b) ADR-018 still uses hyphen at lines 197/216; (c) BC-4.12.002 canonical test vectors still use hyphen at lines 76, 142-144. Additionally, capability_confinement_test.rs doc-strings reference stale hyphen, WaveState schema is sibling-incompatible with warn-pending-wave-gate (YAML mapping vs sequence), and VP-076 deferral is undocumented in the VP spec.

### HIGH

#### ADV-BF1-P02-HIGH-001: Story spec S-12.07 NOT amended after pass-1 — frontmatter version "1.0" and AC bodies still use "wave-context" (hyphen)
- **Severity:** HIGH
- **Category:** spec-drift / partial-fix-regression (S-7.01)
- **Location:** `.factory/stories/S-12.07-vsdd-context-resolvers-crate.md:5` (version frontmatter); lines 66, 69, 73, 75, 77, 81, 87, 99, 101, 107, 126, 131, 174, 260, 295
- **Description:** Implementation uses `wave_context` (underscore); spec still uses `wave-context` (hyphen) in 16 places. Spec is the contract; it does not match the code.
- **Fix:** Bump frontmatter `version: "1.1"`, set `last_amended: 2026-05-10`, add CHANGELOG row, replace all 16 hyphen occurrences with underscore.

#### ADV-BF1-P02-HIGH-002: ADR-018 still uses "wave-context" (hyphen) at lines 197, 216
- **Severity:** HIGH
- **Category:** spec-drift / canonical-anchor mis-anchoring
- **Location:** `.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md:197, 216`
- **Description:** Line 197: `returns ResolverOutput { key: "wave-context", value: ... }`. Line 216: `needs_context = ["wave-context"]`. Line 21 already uses underscore correctly.
- **Fix:** Update lines 197/216 to underscore form.

#### ADV-BF1-P02-HIGH-003: BC-4.12.002 Canonical Test Vectors still use "wave-context" (hyphen) at lines 76, 142, 143, 144
- **Severity:** HIGH
- **Category:** spec-drift / canonical-anchor mis-anchoring
- **Location:** `.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md:76, 142, 143, 144`
- **Description:** BC-4.12.002 PC2 example comment + 3 canonical test vector rows. BC-4.12.005 was updated; sibling BC was not.
- **Fix:** Bump BC-4.12.002 to v1.3 with changelog. Update all 4 occurrences.

#### ADV-BF1-P02-HIGH-004: capability_confinement_test.rs doc-strings reference stale "wave-context" (hyphen)
- **Severity:** HIGH
- **Category:** test-defect / stale-anchor
- **Location:** `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs:25, 39, 47, 58, 78`
- **Description:** Deferred VP-076 integration tests carry doc-strings referencing wrong key. When tests are reactivated (S-12.08), they will be written against wrong canonical.
- **Fix:** Replace all 5 occurrences with underscore.

#### ADV-BF1-P02-HIGH-005: VP-076 spec amendment incomplete — Proof Harness Location still anchors to S-12.07, no deferral recorded
- **Severity:** HIGH
- **Category:** spec-drift / story-DoD mismatch
- **Location:** `.factory/specs/verification-properties/VP-076.md:243-253, 261, 269`
- **Description:** VP-076 still says "Files to be created during S-12.07 delivery" and "Harness instantiated — test-writer (S-12.07 delivery)". Story DoD line 291 says VP-076 tests must pass. Deferral is unauditable.
- **Fix:** Add VP-076 changelog; amend Proof Harness Locations to "deferred to S-12.08 bats". Amend S-12.07 DoD to mark AC-005/006 as DEFERRED.

#### ADV-BF1-P02-HIGH-006: WaveState schema sibling-coverage violation — warn-pending-wave-gate reads waves as YAML MAPPING; new resolver + producer use YAML SEQUENCE
- **Severity:** HIGH
- **Category:** sibling-coverage (POL-5) / latent integration defect
- **Location:** `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs:42` vs `crates/vsdd-context-resolvers/src/wave_context.rs:66-70` vs `crates/hook-plugins/update-wave-state-on-merge/src/lib.rs:178-181`
- **Description:** Three crates reason about wave-state.yaml with TWO INCOMPATIBLE schemas. Producer + new resolver: SEQUENCE (`waves:\n  - wave: "..."`). warn-pending-wave-gate: MAPPING (`waves:\n  W-15:\n    gate_status: pending`). No live wave-state.yaml exists to disambiguate — meaning never tested end-to-end.
- **Fix:** Pick ONE canonical schema (sequence) and migrate warn-pending-wave-gate. OR file TD #73 with explicit scope/blast-radius. Recommend HIGH (load-bearing inconsistency) — but adjudication may classify lower if author intent says coexistence is OK.

### MEDIUM

#### ADV-BF1-P02-MED-001: Active-wave determination treats only "completed" as terminal; producer uses "pending", "passed", "deferred", "completed"
- **Severity:** MEDIUM
- **Location:** `crates/vsdd-context-resolvers/src/wave_context.rs:96-106`
- **Fix:** Add `TERMINAL_STATES: &[&str] = &["completed", "passed", "deferred"]`. Cross-reference BC-7.03.085/086.

#### ADV-BF1-P02-MED-002: AC-009 doesn't validate `name == context_key`; spec silent on dual-key requirement
- **Severity:** MEDIUM
- **Location:** `.factory/stories/S-12.07-...md:121-131`
- **Fix:** Amend AC-009 to include `context_key = "wave_context"` in example TOML.

#### ADV-BF1-P02-MED-003: AC-005 four-case gate_status truth table is unenforced; tests only cover Cases 1 and 4
- **Severity:** MEDIUM
- **Location:** `crates/vsdd-context-resolvers/tests/wave_context_test.rs:241-309`
- **Fix:** Add `gate_status_yaml_null`, `gate_status_yaml_key_absent` tests.

#### ADV-BF1-P02-MED-004: parse_cycle_id_from_state_md ignores CRLF line endings
- **Severity:** MEDIUM
- **Location:** `crates/vsdd-context-resolvers/src/wave_context.rs:116-146`
- **Fix:** Normalize `\r\n` to `\n` before parsing. Add CRLF test.

#### ADV-BF1-P02-MED-005: read_file byte budget hardcoded at 64KB — silent failure if state files grow
- **Severity:** MEDIUM
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:68, 93`
- **Fix:** Extract `MAX_STATE_FILE_BYTES = 1024 * 1024`. Emit log_warn on read error other than missing.

#### ADV-BF1-P02-MED-006: Cargo.toml unexpected_cfgs cfg(kani) declared but no kani harnesses — dead lint declaration
- **Severity:** MEDIUM
- **Location:** `crates/vsdd-context-resolvers/Cargo.toml:36`
- **Fix:** Remove (workspace setting covers it).

### LOW

- **ADV-BF1-P02-LOW-001:** parse_cycle_id_from_state_md accepts empty string; rejection happens at lib.rs:127 but is undocumented.
- **ADV-BF1-P02-LOW-002:** resolve_wave_context_pure unused `_input` parameter — wasteful proptest coverage; document or drop.
- **ADV-BF1-P02-LOW-003:** test_BC_4_12_001_wasm_artifact_registered_in_registry: substring matching without calling production fn (POL-11).
- **ADV-BF1-P02-LOW-004:** test_BC_4_12_004_no_unwrap_or_expect_in_lib: textual heuristic redundant with clippy deny.
- **ADV-BF1-P02-LOW-005:** Doc comment in lib.rs:32-36 calls resolver_macro a "TYPE path" — both are module paths.

### NITPICK

- **ADV-BF1-P02-NIT-001:** Cargo.toml description references `WaveContextResolver` (PascalCase) — implementation is free fn `resolve_impl`.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 6 |
| MEDIUM | 6 |
| LOW | 5 |
| NITPICK | 1 |
| **Total** | **18** |

## Novelty Assessment

Novelty: MEDIUM-HIGH. HIGH-001/002/003 are partial-fix-regression follow-throughs from pass-1 fixes. HIGH-006 (sibling-schema disagreement) is genuinely novel — load-bearing latent inconsistency that S-12.07's new code amplifies. HIGH-005 (VP-076 deferral) and HIGH-004 (stale doc-strings) are downstream-propagation checks.

## Process-Gap Tags

None this pass. HIGH-001's partial-fix pattern (implementation propagated, spec did not) is a recurring pattern — if a similar finding recurs in pass 3 or 4, escalate to process-gap on the orchestrator's fix-burst checklist.
