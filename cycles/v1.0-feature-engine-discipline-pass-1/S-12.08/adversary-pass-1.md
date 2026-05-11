---
document_type: adversary-review
level: ops
version: "1.0"
status: draft
producer: adversary
timestamp: 2026-05-11T03:00:00Z
phase: engine-discipline-F4-S-12.08
inputs:
  - story_spec_v1.1
  - bcs (BC-1.13.001, BC-4.10.001, BC-4.12.001..005)
  - vps (VP-071, VP-073..076)
  - ADR-017, ADR-018
  - branch_diff (origin/develop..db298c94)
input-hash: "[pending-recompute]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-12.08 adversary pass-1 review (fresh context)"
current_cycle: v1.0-feature-engine-discipline-pass-1
story_id: S-12.08
pass: 1
previous_review: null
verdict: MEDIUM
findings_count: { critical: 0, high: 3, medium: 6, low: 3, nitpick: 0 }
deferred_findings: 1
convergence_reached: false
---

# S-12.08 Adversary Pass-1 (Fresh Context)

## Finding ID Convention

`ADV-BF1-P01-<SEV>-<SEQ>`.

## Severity Verdict

**MEDIUM.** Implementation is fundamentally sound (clean migration to extract_stories_from_wave_context, old static-config path removed, resolver-linker WASI fix landed, bats integration GREEN). However, 3 HIGH findings warrant remediation before convergence + 6 MEDIUM quality issues.

## Part B — Findings

### HIGH

#### ADV-BF1-P01-HIGH-001: BC-4.10.001 amendment missing for new WAVE_CONTEXT_* codes (Compliance Rule 5 violation)

- **Files:** `.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md` (no WAVE_CONTEXT_* in changelog); `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:244,252` (constants declared)
- **Evidence:** Story spec line 234 (Compliance Rule 5): "WAVE_CONTEXT_MISSING and WAVE_CONTEXT_SCHEMA_ERROR must either already appear in BC-4.10.001 or PR must include amendment-required note". BC-4.10.001 currently enumerates only CONVERGENCE_STATE_MISSING. POL-7 (BC authoritative) violation.
- **Fix:** Add BC-4.10.001 v1.3 amendment row to changelog with WAVE_CONTEXT_MISSING and WAVE_CONTEXT_SCHEMA_ERROR as new postconditions. OR add explicit amendment-required note to PR description.

#### ADV-BF1-P01-HIGH-002: Empty-stories cross-story interface mismatch (resolver returns value:None; hook EC-001 unreachable)

- **Files:** `crates/vsdd-context-resolvers/src/lib.rs:192-198` (resolver returns value:None on empty stories); `crates/factory-dispatcher/src/resolver.rs:467-483` (merge skips None values); `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:474-481` (hook EC-001 empty-stories branch)
- **Evidence:** BC-4.10.001 EC-001 says "wave with 0 stories → Continue (vacuously cleared)". Implementation chain: resolver emits None → dispatcher skip → hook never sees wave_context → returns Block(WAVE_CONTEXT_MISSING). The EC-001 branch in lib.rs is unreachable in production.
- **Fix recommendation:** Modify resolver to emit `value: Some({stories: [], wave_id, cycle_id})` for empty waves. Preserves BC-4.10.001 EC-001 Continue semantic AND makes the hook EC-001 branch actually reach lib.rs.

#### ADV-BF1-P01-HIGH-003: Non-object wave_context misclassified as Missing instead of SchemaError

- **File:** `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:312-321`
- **Evidence:** When `plugin_config["wave_context"]` is a non-object scalar (string, number, array, bool), `wave_context.get("stories")` returns None (Value::get only returns Some for objects) → WaveContextError::Missing. Should be SchemaError per AC-003 semantics.
- **Fix:** Add guard `if !wave_context.is_object() { return Err(SchemaError(...)) }` before line 318.

### MEDIUM

#### ADV-BF1-P01-MED-001: Dead-code fallback cycle_id.unwrap_or("current") masks resolver bugs

- **File:** `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:467-472`
- **Fix:** Either require cycle_id in wave_context (treat absent/wrong-type as SchemaError) OR emit WAVE_CONTEXT_SCHEMA_ERROR instead of silent fallback.

#### ADV-BF1-P01-MED-002: test_wave_context_block_codes_are_non_empty is tautology (POL-11)

- **File:** lib.rs:2033-2047
- **Fix:** Add `/// data-shape pin` opt-out doc-comment OR extend to invoke production fn.

#### ADV-BF1-P01-MED-003: extract_code_from_reason parse fragility

- **File:** lib.rs:54-63
- **Fix:** Construct code directly in loop instead of round-tripping through reason parsing; add test against canonical block_with_fix output.

#### ADV-BF1-P01-MED-004: Bats skip on missing dispatcher binary = false-green vector

- **File:** `plugins/vsdd-factory/tests/resolver-integration.bats:47-50,128,172`
- **Fix:** Change skip → fail OR add setup-time cargo build invocation OR document build prerequisite + assert.

#### ADV-BF1-P01-MED-005: Bats relies on default gate_status absent (schema-evolution fragile)

- **File:** `plugins/vsdd-factory/tests/resolver-integration.bats:84-90`
- **Fix:** Explicitly seed `gate_status: not_started`.

#### ADV-BF1-P01-MED-006: Test name test_BC_4_10_002_graceful_degrade_absent_cycle_dir stale

- **File:** lib.rs:1384-1410
- **Fix:** Rename to `test_BC_4_10_001_blocks_when_state_file_unreadable`; update BC reference comment.

### LOW

- **LOW-001:** `_ = resolver_name;` unused parameter in build_resolver_wasi_linker (resolver_loader.rs:751)
- **LOW-002:** kani toolchain-blocked; document fallback obligation
- **LOW-003:** RESOLVER_TIMEOUT_MS = 1500 literal hardcoded

## Deferred Findings (cross-story per BC-5.39.002 PC2)

- **DEFER-HIGH-002**: empty-stories semantic conflict. Route to wave-gate. Recommended resolution: resolver emits `value: Some({stories: [], ...})` (Option A above).

## Process-Gap Findings

- **PG-1**: No automated enforcement of story Compliance Rule 5 (BC amendment trigger). Recommended: lint hook scanning new block codes vs cited BCs.
- **PG-2**: Bats tests use `skip` for missing binaries — false-green vector. Recommended: project-wide audit.

## R-PLAT-004 Self-Check

PASS. Review reads code/spec directly; does not depend on running the hook against itself. Bats test (passing at db298c94) is the recursive-bootstrap forcing function.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 6 |
| LOW | 3 |
| NITPICK | 0 |
| Deferred | 1 |
| Process-gap | 2 |
| **Total** | **15** |

## Novelty Assessment

HIGH. All findings are first-pass discoveries against fresh-context review. None reworded from prior passes (this IS pass-1).

## Convergence

`convergence_reached`: false. Pass-1 verdict MEDIUM. Per BC-5.39.001 need 3 consecutive NITPICK_ONLY for convergence.
