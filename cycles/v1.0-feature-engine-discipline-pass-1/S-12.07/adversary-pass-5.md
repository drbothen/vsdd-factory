---
document_type: adversary-review
level: ops
version: "1.0"
status: draft
producer: adversary
timestamp: 2026-05-10T23:59:00Z
phase: engine-discipline-F4-S-12.07
inputs:
  - story_spec_v1.2
  - bcs (BC-4.12.001..005, BC-1.13.001 v1.2, BC-8.14.009)
  - vps (VP-073..076)
  - ADR-018 (amended)
  - S-12.08 spec v1.1
  - branch_diff (origin/develop..2440b979)
  - factory-artifacts (a3f04181)
input-hash: "[pending-recompute]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-12.07 adversary pass-5 review (fresh context)"
current_cycle: v1.0-feature-engine-discipline-pass-1
story_id: S-12.07
pass: 5
previous_review: adversary-pass-4.md
verdict: LOW
findings_count: { critical: 0, high: 0, medium: 0, low: 1, nitpick: 3 }
deferred_findings: 0
convergence_reached: false
---

# S-12.07 Adversary Pass-5 (Fresh Context)

## Finding ID Convention

`ADV-BF1-P05-<SEV>-<SEQ>`.

## Part A — Fix Verification (S-7.01)

All pass-4 fixes verified propagated. No regression. Cross-story drift (DEFER-001/002/003) cleanly fixed in factory-artifacts a3f04181.

## Part B — New Findings

**Verdict: LOW.** Pass-4 trajectory held — no MEDIUM/HIGH/CRITICAL regressions. One LOW finding (spec descriptive drift — `WaveContext` struct mentioned in File List but never implemented) + 3 NITPICKs.

### LOW

#### ADV-BF1-P05-LOW-001: Story spec File List references `WaveContext` struct that does not exist in implementation

- **File:** `.factory/stories/S-12.07-vsdd-context-resolvers-crate.md:175, 207, 278`
- **Evidence:** Story line 175 ("File List → NEW → wave_context.rs"): says "`WaveContext` struct (output type for `wave_context` JSON value); parsing logic". Line 207 task T-4 mentions "WaveContext struct stubs". Line 278: "WaveState, WaveContext parsing". Implementation has only `WaveState` and `WaveEntry`. JSON built directly via `serde_json::json!`.
- **Severity rationale:** LOW. Descriptive drift in informational table, not behavioral. Architecture Mapping (lines 153-156) correctly omits `WaveContext`; AC-001 specifies JSON shape directly. Reader comprehension friction only.
- **Fix:** Edit story File List + tasks to remove `WaveContext` mention. Replace with `WaveEntry` reference. Add CHANGELOG row.

### NITPICK

#### ADV-BF1-P05-NIT-001: "AC-005 four-case truth table" doc-comments cross-reference wrong story's AC

- **File:** `crates/vsdd-context-resolvers/src/wave_context.rs:28`; `tests/wave_context_test.rs:462, 483, 502, 521`
- **Evidence:** S-12.07 AC-005 is about path_allow capability (DEFERRED). "Four-case truth table" originates from S-8.04 AC-005 (`update-wave-state-on-merge` gate_status flip).
- **Fix:** Rename to "gate_status four-case truth table" with S-8.04 citation OR drop the AC-005 reference.

#### ADV-BF1-P05-NIT-002: Asymmetric observability — YAML parse errors silently swallowed

- **File:** `crates/vsdd-context-resolvers/src/lib.rs:120`
- **Evidence:** read_file errors at lines 90-100 use log_warn; YAML parse error at line 120 silently `.unwrap_or_default()`. Per SOUL.md #4 (no silent failures), should log.
- **Fix:** Add log_warn on Err arm of parse_wave_state before .unwrap_or_default() fallback.

#### ADV-BF1-P05-NIT-003: Production source retains forensic finding-ID markers (MED-005, MED-004) in doc-comments

- **File:** `crates/vsdd-context-resolvers/src/lib.rs:34, 80`; `crates/vsdd-context-resolvers/src/wave_context.rs:134`
- **Evidence:** "memory use inside the WASM linear memory (MED-005)", "MED-005: log unexpected read errors", "MED-004 (pass-2): normalize CRLF → LF". NIT-003 carve-out is for test files only; production should be clean.
- **Fix:** Replace MED-NNN cross-references with stable rationale ("Read errors are logged at warn level to aid diagnosis." / "Normalize CRLF before frontmatter splitting to handle Windows checkouts.")

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 1 |
| NITPICK | 3 |
| Deferred | 0 |
| **Total** | **4** |

## Novelty Assessment

Novelty: MEDIUM. LOW-001 genuinely new — prior 4 passes did not flag spec File List vs implementation drift. NIT-001 cross-story AC-005 label is new. NIT-002 asymmetric YAML observability is new. NIT-003 extends pass-4 NIT-002 (config markers) to production source code.

## Convergence

`convergence_reached`: false. Verdict LOW (not NITPICK_ONLY) — one LOW blocks the streak.

**Trajectory:** P1 CRIT → P2 HIGH → P3 MED → P4 LOW → **P5 LOW (held)**. After pass-5 fixes (1 LOW + 3 NITs), pass-6 should land NITPICK_ONLY (start of 3-streak per BC-5.39.001). Estimated convergence: end of pass-8.

## Process-Gap Findings

None this pass.
