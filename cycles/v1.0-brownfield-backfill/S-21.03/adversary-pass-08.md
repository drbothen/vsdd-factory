---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 6
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.03-pr-manager-orphan-merge.md"
pass: 8
previous_review: "adversary-pass-07.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: CLEAN
reviewed_head: 194e98fe
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-8 — CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 8 of BC-5.39.001 cascade
**Result:** CLEAN — streak 2/3
**Severity breakdown:** B0 / H0 / M0 / L0 / NITPICK0 / OBS1
**Total findings:** 0 findings + 1 observation
**Reviewed diff:** HEAD 194e98fe on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09
**Note:** Implementation HEAD unchanged since pass 5 — pass-6 fix burst ae1ad428 was story-spec-only (§File Structure Requirements Notes cell range update); pass-7 was CLEAN with no fix burst dispatched; pass-8 reviews same HEAD

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P8`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P8-001` (hypothetical finding, pass 8, first finding).

---

## Part A — Prior Pass Finding Resolution

### OBS-P7-1 — Trunk-parameterization asymmetry: Step 3 error body hardcodes `develop`

- **Pass-7 verdict:** ACCEPTED-WITH-RECORD (observation only; not a finding)
- **Pass-8 resolution:** CARRY-FORWARD-CONFIRMED
- **Evidence:** Independent re-examination at HEAD 194e98fe. Step 3 error body continues to hardcode `develop`. Disposition stands: Step 3 is the feature-PR create flow, pre-existing hardcode outside the S-21.03 story diff, assertion instruction correctly uses "configured trunk," and test environment alignment makes the assertion functionally correct at current HEAD. No new angles surfaced that would change the adjudication. Observation remains ACCEPTED-WITH-RECORD; still noted for wave-gate.

---

## Part B — New Findings

No findings at BLOCKER, HIGH, MEDIUM, LOW, or NITPICK severity.

Independent fresh-context confirmation of pass-7. New angles surveyed under this pass:

**Surface 1 — Doc-parity grep fidelity (exhaustive):** Every `_assert_doc_marker` regex in the bats test harness was resolved to real text in its extracted section with line evidence. Specifically: the Step 8-post-A label regex matches the exact string present in the pr-manager.md Step 8-post-A heading; the BC-6.10.002 EC-007 anchor regex matches the EC-007 clause in the behavioral contract; the Step A / Step B split label regexes match the exact sub-heading text under the fetch/ancestry decomposition section. No grep target was found to resolve to an absent, renamed, or moved anchor. Doc-parity grep fidelity is EXHAUSTIVELY VERIFIED.

**Surface 2 — Extractor exit-anchor correctness:** The extractor exit boundary logic (the boundary that terminates section extraction before Step 8b and subsequent steps) was independently verified not to truncate prematurely. Step A and Step B sub-heads under Step 8-post-A are fully captured within the extraction window; the extractor terminates correctly at the Step 8b heading boundary without including subsequent steps. No truncation of normative content detected. Extractor exit-anchor is CORRECT.

**Surface 3 — Harness/fixture logic (env isolation + null-before-fetch ordering):** Per-test environment isolation was verified: each bats test function sets up its own `GIT_FETCH_FAIL_ONCE` stub state independently and tears it down via the base-setup helper's cleanup hook, preventing stub state leakage between tests. Null-before-fetch ordering fidelity was confirmed: T-001 (null PR) is asserted before T-006 (TrunkFetchFailed) and T-007 (retry-succeed) in test execution order, matching the protocol's required null-guard-first ordering. Harness/fixture logic is VERIFIED.

**Surface 4 — Semantic anchoring audit (§Decision 8, §Consequences #6, CAP-038, BC H1, target_module):** All five semantic anchors independently re-verified at HEAD 194e98fe:
  - `§Decision 8` in ADR-031 v1.8: resolves correctly to the Step 8-post-A placement governing decision.
  - `§Consequences #6` in ADR-031 v1.8: resolves correctly to the deletion-agnostic recovery consequence clause.
  - `CAP-038`: resolves correctly to the orphan-merge detection capability in the domain spec capability registry.
  - `BC H1`: resolves correctly to the BC-6.10.002 behavioral contract title header.
  - `target_module` in the story implementation mapping: resolves correctly to the pr-manager module without phantom references.
  All five anchors confirmed present, non-stale, and semantically accurate. Semantic anchoring audit is CLEAN.

**Surface 5 — Version cross-checks:** ADR-031 cited version (v1.8) confirmed matches the actual ADR-031 frontmatter version at HEAD; BC-6.10.002 cited version confirmed matches the actual BC frontmatter version; story v1.8 frontmatter version consistent with the §File Structure Requirements Notes cell update at ae1ad428. All version cross-references match. No version-drift detected.

**Surface 6 — S-7.01 partial-fix regression axis:** All prior fixes from passes 1–7 confirmed propagated and no orphaned old-value references detected. Specifically: the stale "Step 9" sweep (P4-004 + P5-001: 4 comment lines + 22-hit classified grep) confirms zero residual Step-9 normative occurrences; the RG-004 registration row (P5-002) is present and load-bearing-backed; the §Decision 8 + §Consequences #6 re-anchor (P5-003) is intact; the inert-idiom audit (P4-002: 30 `false` occurrences all load-bearing) is unchanged. Zero orphaned old-value references from any prior pass. S-7.01 regression axis CLEAN.

---

### Observations (ACCEPTED-WITH-RECORD)

#### OBS-P8-1 — git stub line 23: `exec /usr/bin/git` fallback — latent portability seam

- **Observation:** The `GIT_FETCH_FAIL_ONCE` stub script in the bats fixture directory contains a fallback at line 23: `exec /usr/bin/git "$@"`. This hardcodes the git executable path to `/usr/bin/git` for the post-intercept passthrough. On macOS and most Linux CI hosts, git resides at `/usr/bin/git` (or `/usr/bin/git` is a shim to the system git), so this is inert in the current suite and is never reached on failure paths. However, on hosts where git is installed at a non-standard path (e.g., `/opt/homebrew/bin/git`, `/usr/local/bin/git`, or NixOS-style paths), the stub's passthrough would invoke an absent binary, causing test failures unrelated to the story's actual logic.
- **Classification:** Inert in current suite — the stub is only reached when `GIT_FETCH_FAIL_ONCE` is active, and the passthrough at line 23 is the post-intercept one-shot path that executes exactly once per T-006/T-007 test, successfully. The fallback never fails on any standard macOS or Linux CI environment. This is a latent portability seam for future tests on non-`/usr/bin/git` hosts (e.g., containerized builds with custom git paths).
- **Disposition:** ACCEPTED. Recorded informational. Adjudicated inert in current suite — the hardcode path is functional on all current CI environments and is never reached on failure paths in the tested scenarios. Not dispatching a fix in-cascade. Noted for wave-gate as a stub-portability hardening item.

---

### Clean Axes Verified

- **All P7 CLEAN confirmation re-verified:** Pass-7's four converged surfaces independently re-confirmed under fresh context — doc-parity grep targets, ordering invariant, Step A/B retry-once harness fidelity, deletion-agnostic recovery — all CONVERGED.
- **Doc-parity grep fidelity (exhaustive, new angle):** Every `_assert_doc_marker` regex resolved to real text with line evidence. No ghost anchors. Exhaustively verified in this pass.
- **Extractor exit-anchor (new angle):** Step A/B sub-heads fully captured; no premature truncation at step boundary. Correct.
- **Harness/fixture env isolation + null-before-fetch ordering (new angle):** Per-test stub state isolation confirmed; null-guard-first ordering confirmed. Fidelity verified.
- **Semantic anchoring audit (§Decision 8, §Consequences #6, CAP-038, BC H1, target_module) (new angle):** All five anchors resolve correctly. Clean.
- **Version cross-checks (new angle):** ADR-031 v1.8, BC-6.10.002, story v1.8 all consistent. No version-drift.
- **S-7.01 partial-fix regression axis (new angle):** Zero orphaned old-value references from passes 1–7. Full propagation confirmed.
- **Scope discipline:** No fix burst dispatched since ae1ad428 (pass-6). Implementation remains at 194e98fe. No lateral scope expansion detected.
- **TD-VSDD-091:** All spec narrative at 194e98fe uses behavioral anchors. No `file.rs:NNN` line-number citations found in normative sections.
- **TD-VSDD-060 sibling sweep (carry-forward):** 22-hit Step-9 grep zero residue confirmed; no new sibling sites introduced.
- **Mutation coverage (carry-forward):** P4-002 mutation kill classes (overrun, misclassification, mandate-deletion) re-verified at HEAD — no regression.
- **Inert-idiom audit (carry-forward):** File-wide `false` occurrence audit at 194e98fe — all 30 confirmed load-bearing; no `&&{false;}||true` anti-pattern residue.
- **OBS-P6-1 / OBS-P6-2 / OBS-P7-1 (carry-forward):** All three accepted-with-record observations re-confirmed as non-normative at HEAD.
- **Novelty:** LOW — zero actionable findings. Single low-severity portability observation on the git stub fallback path (inert in current suite; never reached). All substantive structural axes CONVERGED across passes 4–8. Residue trend: CONVERGED.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 0 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 1 |

**Overall Assessment:** CLEAN (0 actionable findings; 1 observation ACCEPTED)
**Convergence:** streak 2/3 — iterate (1 more CLEAN pass required for BC-5.39.001 convergence)
**Readiness:** CLEAN — no revision required; OBS-P8-1 noted for wave-gate alongside OBS-P7-1
**Next reviewed HEAD:** 194e98fe (unchanged — no fix burst dispatched; pass-9 will re-review same HEAD)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 0 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | N/A (no findings) |
| **Median severity** | N/A |
| **Trajectory** | 3→3→3→5→3→1→0→0 |
| **Verdict** | CONVERGED — all substantive axes confirmed clean across six new angles; single inert portability observation on git stub fallback; streak 2/3 |
