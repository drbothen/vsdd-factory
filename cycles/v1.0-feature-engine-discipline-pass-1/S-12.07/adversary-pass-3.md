---
document_type: adversary-review
level: ops
version: "1.0"
status: draft
producer: adversary
timestamp: 2026-05-10T23:00:00Z
phase: engine-discipline-F4-S-12.07
inputs:
  - story_spec_v1.1
  - bcs (BC-4.12.001..005, BC-7.03.085/086, BC-8.14.009)
  - vps (VP-073..076, VP-076 amended)
  - ADR-018 amended
  - branch_diff (origin/develop..21feef00)
  - factory-artifacts (7c485ce3)
input-hash: "[pending-recompute]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-12.07 adversary pass-3 review (fresh context)"
current_cycle: v1.0-feature-engine-discipline-pass-1
story_id: S-12.07
pass: 3
previous_review: adversary-pass-2.md
verdict: MEDIUM
findings_count: { critical: 0, high: 0, medium: 3, low: 4, nitpick: 2 }
convergence_reached: false
---

# S-12.07 Adversary Pass-3 (Fresh Context)

## Finding ID Convention

Finding IDs: `ADV-BF1-P03-<SEV>-<SEQ>`.

## Part A — Fix Verification

Reviewer dispatched FRESH CONTEXT — no knowledge of prior reviews. Re-derived findings from scratch.

## Part B — New Findings

**Verdict: MEDIUM.** Substantial improvement from pass-2. Production code, registry, story spec (v1.1), ADR-018, BC-4.12.002 v1.3, BC-4.12.005, and VP-076 all consistently use `wave_context` (underscore). Three MEDIUM findings surface from cross-referencing BC-8.14.009 (the canonical gate_status enum authority) and from missed sibling-propagation of the wave-context → wave_context rename.

### MEDIUM

#### ADV-BF1-P03-MED-001: TERMINAL_STATES omits canonical "failed" state

- **File:** `crates/vsdd-context-resolvers/src/wave_context.rs:103`
- **BC anchor:** BC-8.14.009 (canonical gate_status enum)
- **Description:** `TERMINAL_STATES = ["completed", "passed", "deferred"]` omits `"failed"`. Per BC-8.14.009 line 35, canonical enum is `{not_started | pending | passed | deferred | failed}`. A wave with `gate_status: failed` would be returned by find_active_wave as active, causing wave_context.stories to inject for a failed wave.
- **Evidence:** wave_context.rs:103; BC-8.14.009.md:35; production fixture wave-gate-hooks.bats:209.
- **Fix:** Append "failed" to TERMINAL_STATES. Add test_gate_status_yaml_failed_is_terminal.

#### ADV-BF1-P03-MED-002: capability_confinement_test.rs:47 stale "wave-context" hyphen doc-string

- **File:** `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs:47`
- **Description:** Doc-string says `resolver = "wave-context"` (hyphen). Canonical name is `wave_context` (underscore) per registry. Pass-1/pass-2 fix did not propagate to this doc-string. S-12.08 implementer will write wrong assertion.
- **Fix:** Change `wave-context` → `wave_context`.

#### ADV-BF1-P03-MED-003: wave_context.rs:97-99 doc-comment reverses canonical/legacy characterization

- **File:** `crates/vsdd-context-resolvers/src/wave_context.rs:96-99`
- **BC anchor:** BC-8.14.009
- **Description:** Doc says `"completed" — gate approved` and `"passed" — alias used in some producer versions (same semantic as completed)`. Reversed: canonical is `"passed"` per BC-8.14.009; `"completed"` is non-canonical and appears only in test fixtures.
- **Fix:** Rewrite the doc-block to put canonical names first, mark "completed" as non-canonical legacy alias.

### LOW

#### ADV-BF1-P03-LOW-001: WaveContext struct defined and exported but never instantiated

- **Files:** `crates/vsdd-context-resolvers/src/wave_context.rs:76-84`; `src/lib.rs:30`
- **Description:** WaveContext re-exported but actual output payload built inline via serde_json::json! macro. Story file list calls WaveContext "output type" but implementation routes around it.
- **Fix:** Either construct WaveContext { ... } and serialize via to_value, OR delete the struct.

#### ADV-BF1-P03-LOW-002: parse_wave_state lacks CRLF normalization while parse_cycle_id_from_state_md has it

- **File:** `crates/vsdd-context-resolvers/src/wave_context.rs:91-93`
- **Description:** Pass-2 MED-004 fixed CRLF for STATE.md but not for wave-state.yaml parser. S-7.01 partial-fix-regression discipline.
- **Fix:** Either add CRLF normalization (defensive) OR document why serde_yaml handles CRLF natively (YAML 1.2 spec allows both).

#### ADV-BF1-P03-LOW-003: Workspace clippy lints "warn" while crate enforces "deny" — comment misleading

- **File:** `Cargo.toml:95-105` (workspace)
- **Description:** Workspace sets "warn"; CI runs `-D warnings` which elevates to errors. Comment claims "violations visible without breaking build" — inconsistent with actual CI behavior.
- **Fix:** Update comment OR change workspace to "deny".

#### ADV-BF1-P03-LOW-004: AC-004 falsifiable test does not exercise spec-described path

- **Files:** `tests/wave_context_test.rs:222-239`; story line 94-97
- **Description:** Spec describes "wave has stories but no explicit wave_id key" — impossible because WaveEntry.wave is String (not Option). Test uses empty waves list as proxy.
- **Fix:** Add explicit test asserting parse_wave_state fails on schema-incomplete input. Update AC-004 spec text.

### NITPICK

- **ADV-BF1-P03-NIT-001:** Stale "Pass-1 fix" forensic doc-comments in production code become historical noise post-merge.
- **ADV-BF1-P03-NIT-002:** Comment "all-None" out of date with post-F-003 schema (default = empty waves).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 4 |
| NITPICK | 2 |
| **Total** | **9** |

## Novelty Assessment

Novelty: MEDIUM. MED-001 (failed not in TERMINAL_STATES) is genuinely new — discoverable only by cross-referencing BC-8.14.009. MED-002 is a sibling-coverage gap from pass-1/pass-2 rename. MED-003 surfaces documentation reversal. All three are not refinements of prior findings.

## Convergence

convergence_reached: false. Three MEDIUM findings require fixes before pass-4.
