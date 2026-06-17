---
document_type: adversarial-review
cascade: E-18-story
pass: 9
verdict: NOT-CLEAN
date: 2026-06-17
reviewer: adversary
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-e18-story-pass-8.md
  - .factory/stories/S-18.06-validate-heavy-op-delegation-wasm-gate.md
  - .factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md
streak: 0/3
findings_count: 3
---

# E-18 Story Cascade Adversarial Review — Pass 9

**Date:** 2026-06-17
**Verdict:** NOT-CLEAN
**Streak:** 0/3 (pass-9 NOT-CLEAN)
**Pass-8 closures verified:** All 8 pass-8 closures (F-P8-001 BLOCKER awk-range-collapse, F-P8-002 BLOCKER BC-4.15.001 PC-B labels, F-P8-003 MED ARCH-INDEX drift, O-P8-A OBS, F-P8-004 MED withdrawn carve-out, F-P8-005 MED ss-%02d, C-P8-001) CLOSED per orchestrator report.

---

## Part A — Findings

### F-P9-001 — MAJOR: BC-INDEX v3.07 changelog-array row missing

**Severity:** MAJOR
**File:** `.factory/specs/behavioral-contracts/BC-INDEX.md`
**Status:** FIXED by state-manager (pass-9 fix burst)

**Description:** BC-INDEX frontmatter declares `version: "3.07"` and `last_amended` cites D-625 (E-18 STORY PASS-8 FIX BURST) as the v3.07 bump. However the `changelog:` YAML array has no v3.07 entry — the topmost entry is v3.06 (D-619). The D-625 fix burst updated the `version:` and `last_amended:` legs of POLICY 14 parity but silently skipped the `changelog:` array leg. This is a partial-fix regression: the version-leg was advanced but the changelog-array leg was omitted.

**Root cause:** D-625 state-manager burst performed a POLICY 14 parity bump for BC-4.15.001 catalog-row version-cell (BC-INDEX body edit) but only updated 4 of the 5 POLICY 14 legs: (1) frontmatter version, (2) body Changelog section (BC-INDEX has no body changelog section — N/A), (3) modified[] array (N/A), (4) last_amended text-prefix — YES, (5) upstream-index body-table cells (N/A for self). The missing leg is the `changelog:` YAML array, which is specific to BC-INDEX/VP-INDEX/ARCH-INDEX/STORY-INDEX as structured index files. The D-625 burst moved the `version:` and `last_amended:` keys but missed inserting the `changelog:` array entry.

**Verification:** `grep -n "v3.07" .factory/specs/behavioral-contracts/BC-INDEX.md` returns hits only in `version:` and `last_amended:` lines (lines 3 and 8), not in the `changelog:` array body. The top `changelog:` array entry is `v3.06`.

**Fix applied:** State-manager (pass-9 fix burst): append v3.07 entry at top of `changelog:` array, above the v3.06 entry. Content mirrors the `last_amended:` D-625 text. This is a parity-REPAIR of the existing v3.07 version — no version bump (already at v3.07).

---

### F-P9-002 — MEDIUM (load-bearing): S-18.09 AC-008 compound-cite gate checks only first cite per line

**Severity:** MEDIUM (load-bearing)
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED by story-writer (pass-9 fix burst)

**Description:** The AC-008 compound-cite extraction gate in S-18.09 v1.8 uses a pattern that extracts only the FIRST BC cite per line (e.g., `grep -oE 'BC-[0-9]+\.[0-9]+\.[0-9]+' | head -1` style). The spec requires checking ALL cites present on any given line. When a line contains multiple cites (e.g., `BC-4.15.001, BC-5.41.001`), the gate passes as long as the first cite resolves, silently ignoring subsequent cites on the same line that may be stale or unresolvable. This is a gate-cardinality-completeness gap: the gate checked only first-cite-per-line vs the spec's all-cites requirement.

**Root cause:** The awk compound-cite extraction pattern was written to stop after the first match per line (first-match semantics) rather than extracting all matches on each line (global-match semantics). The difference only surfaces when a line has two or more BC cites. Additionally, the EC-009 fixture (broken-second-cite scenario) was absent from the test vectors, leaving this cardinality gap undetected.

**Fix applied:** Story-writer (pass-9 fix burst): S-18.09 v1.9 rewrites AC-008 gate to use global-match extraction (carry-forward all cites per line); adds EC-009 broken-second-cite fixture to test vectors.

---

### F-P9-003 — LOW: S-18.09 AC-section scope specification underspecified

**Severity:** LOW
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED by story-writer (pass-9 fix burst)

**Description:** The AC-section scope definition in S-18.09 v1.8 does not precisely specify which Markdown heading levels delimit an "AC section" for the extraction gate. The gate currently assumes `## ` (H2) as section boundaries, but some story files use `### ` (H3) subsections within ACs. Without an explicit scope specification, a future gate implementation might scan too narrowly (missing H3 content) or too broadly (including H3 from other sections). The spec should state explicitly: "AC section = text between the `## Acceptance Criteria` H2 heading and the next H2 heading."

**Fix applied:** Story-writer (pass-9 fix burst): S-18.09 v1.9 adds explicit scope note to AC-section extraction definition.

---

## Part B — Observations (Non-blocking)

No Part B observations.

---

## Summary

| Finding | Severity | Status |
|---------|----------|--------|
| F-P9-001 | MAJOR | FIXED state-manager (BC-INDEX changelog-array parity repair) |
| F-P9-002 | MEDIUM (load-bearing) | FIXED story-writer (S-18.09 v1.9 compound-cite global-match + EC-009) |
| F-P9-003 | LOW | FIXED story-writer (S-18.09 v1.9 AC-section scope explicit) |

**3-CLEAN streak:** 0/3. Pass-10 adversary + consistency re-verify NEXT.
