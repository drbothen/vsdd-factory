---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p2.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/legacy-bash-adapter/Cargo.toml
  - crates/hook-plugins/capture-pr-activity/Cargo.toml
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
input-hash: "68f3d16"
traces_to: prd.md
pass: p3
previous_review: adv-s8.05-p2.md
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 2
findings_low: 1
findings_nit: 0
---

# Adversarial Review: S-8.05 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `F-S805-P3-<SEQ>` per project short-form convention.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S805-P2-001 result extraction dual-fallback | HIGH | PARTIALLY_RESOLVED | `result` field fixed; `agent` field gap introduced |
| F-S805-P2-002 | MED | RESOLVED | Resolved |
| F-S805-P2-003 | MED | RESOLVED | Resolved |
| F-S805-P2-004 | LOW | RESOLVED | Resolved |

## Part B — New Findings (or all findings for pass 1)

Pass-3 review of S-8.05 v1.2 (hash 68f3d16). 3 of 4 pass-2 findings closed. F-S805-P2-001 closed for `result` extraction but introduces NEW gap on `agent` field (same dual-fallback pattern). Universal patch #4 (`vsdd-hook-sdk path = "../hook-sdk"`) is empirically WRONG: every sibling plugin uses `../../hook-sdk`. Story would not build.

5 findings: 2H, 2M, 1L. Verdict SUBSTANTIVE. Clock held. Trajectory 4 -> 5 (NOT healthy decay; v1.2 burst introduced regressions).

### HIGH

#### F-S805-P3-001: vsdd-hook-sdk path dep wrong — universal-patch error
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.05 lines 326 (T-2), 391 (Architecture Compliance), 406 (Library Reqs), 416 (File Structure)
- **Description:** vsdd-hook-sdk path dep is wrong: `../hook-sdk` resolves to non-existent directory. Empirical sibling pattern: `legacy-bash-adapter/Cargo.toml:25`, `capture-pr-activity/Cargo.toml:17`, `capture-commit-activity/Cargo.toml:23` all use `path = "../../hook-sdk"`. Story would not build.
- **Evidence:** `legacy-bash-adapter/Cargo.toml:25`: `path = "../../hook-sdk"`. `capture-pr-activity/Cargo.toml:17`: `path = "../../hook-sdk"`. `capture-commit-activity/Cargo.toml:23`: `path = "../../hook-sdk"`. Story says `../hook-sdk` (one level too shallow).
- **Proposed Fix:** Replace all 4 occurrences with `../../hook-sdk`. THIS IS A UNIVERSAL-PATCH ERROR PROPAGATED FROM ORCHESTRATOR — correct the universal patch list.

#### F-S805-P3-002: Agent extraction missing dual-fallback
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** S-8.05 T-3 line 336; AC-007 case (e)
- **Description:** Agent extraction missing dual-fallback. Bash source `validate-pr-review-posted.sh:21` uses `.agent_type // .subagent_name // "unknown"`. T-3:336 says only "parse agent_type" — no fallback. AC-007 case (e) JSON has `subagent_name` but NO `agent_type`. Implementer reads agent_type, gets None, exits 0 immediately, Check 3b never fires; bats assertion fails. Same defect class as F-S805-P2-001 RESULT fix — pass-2 didn't generalize.
- **Evidence:** `validate-pr-review-posted.sh:21`: `.agent_type // .subagent_name // "unknown"`. T-3:336 silent on fallback chain. AC-007(e) fixture: `{"subagent_name": "foo"}` with no `agent_type`.
- **Proposed Fix:** Pin `envelope.get("agent_type").or(envelope.get("subagent_name")).and_then(|v| v.as_str()).unwrap_or("unknown")` in T-3.

### MEDIUM

#### F-S805-P3-003: SS-02 missing from frontmatter subsystems
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.05 frontmatter line 40; Stretch-Anchor Disclosure lines 73-89
- **Description:** SS-02 referenced in body twice (lines 391, 406) but missing from frontmatter `subsystems` array (line 40). Stretch-Anchor Disclosure (lines 73-89) does not enumerate SS-02.
- **Evidence:** Body references SS-02 at lines 391, 406; frontmatter subsystems omits it.
- **Proposed Fix:** Add SS-02 to frontmatter + stretch disclosure, OR reverse universal-1 attribution.

#### F-S805-P3-004: emit_event call signature not pinned
- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** S-8.05 T-5; Goal narrative line 157
- **Description:** `host::emit_event` call signature not pinned — slice-of-tuples mapping ambiguous. Goal narrative line 157 mixes event_type with field-list notation.
- **Evidence:** Line 157 mixes notation styles; T-5 does not provide canonical Rust call snippet.
- **Proposed Fix:** Add canonical Rust call snippet to T-5 with explicit slice-of-tuples form.

### LOW

#### F-S805-P3-005: Case (e) prose conflates "fire" with "match"
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.05 lines 240-243
- **Description:** Case (e) prose conflates "fire" with "match" (lines 240-243). Rewrite to clarify: Check 3a passes (regex matches → no error); Check 3b accumulates the no-verdict error.
- **Evidence:** Lines 240-243 use "fire" to mean both regex match and error accumulation.
- **Proposed Fix:** Rewrite: "Check 3a: regex matches → no format error raised. Check 3b: no verdict field → no-verdict error accumulated."

## [process-gap]

The Universal Patch list provided to story-writer asserted `path = "../hook-sdk"` for vsdd-hook-sdk in plugin crates. EMPIRICAL CHECK against 3 sibling crates shows canonical form is `../../hook-sdk` (two levels up from `crates/hook-plugins/<name>/`). Patch list is WRONG and would propagate the same defect to every Tier-1 port story. Codify "verify universal patch against >= 1 sibling artifact before propagation" gate in story-writer prompt.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 2 |
| LOW | 1 |
| NIT | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 5 |
| **Duplicate/variant findings** | 1 (agent dual-fallback same class as P2-001) |
| **Novelty score** | 0.83 (5/6) |
| **Median severity** | 3.5 |
| **Trajectory** | 4→5 |
| **Verdict** | FINDINGS_REMAIN |
