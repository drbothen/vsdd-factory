# Adversarial Review — E-9 v1.28 Cross-Doc Terminology-Drift Fix (D-272) — Pass 30

**Date:** 2026-05-05
**Commit reviewed:** e8f74ad (v1.27 → v1.28)
**Cumulative surface:** v1.7..v1.28 (4 files + open-questions.md + 2 BCs)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Diff-only line-by-line of v1.28 (e8f74ad) — strict diff scope: only the two HIGH fix sites (BC-1.05.036:51 H-P29-001, BC-1.05.035:35 H-P29-002), the v1.28 H3 historical block, the v1.28 summary-table row, and the TD-VSDD-079 8-term family-grep claim. NEW per TD-VSDD-057.

## Summary

The v1.28 burst (e8f74ad) is a 2-fix HIGH-only diff. Both fixes verified PASS:
- H-P29-001: BC-1.05.036:51 fan-out + Datadog/Honeycomb scrubbed; replaced with neutral "external export to remote observability backends" — TD-VSDD-079 8-term grep returns zero non-changelog matches.
- H-P29-002: BC-1.05.035:35 §Description NUL-byte attribution corrected to align with §Postcondition 2 + §Precedence Ladder + §EC-005 (read_wasm_string error path).

TD-VSDD-079 8-term family grep PASS across all 5 in-scope files. No HIGH or MED findings within v1.28 diff scope. One LOW out-of-strict-diff-scope finding noted (citation §-name mismatch from v1.22).

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P30-001 [LOW] (out-of-strict-diff-scope; pending intent verification): BC-1.05.035:33 ADR-015 awareness clause §-name vs lines mismatch**
- Citation: `gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" lines 339-349`
- Reality: §"How ADR-015 affects the telemetry gap" begins at line 314; §"Existing denial-path telemetry" begins at line 339. Lines 339-349 fall in sibling section.
- Severity: LOW (line-range still resolves to correct content; only §-label is wrong).
- Out-of-scope: introduced in v1.22 TD-VSDD-074 propagation, not v1.28. Strict diff-only scope discipline.
- Tag: pending intent verification — adversary did not check sibling BC-1.05.036:33 for same defect; orchestrator may verify.

## Out-of-scope-but-noted

- v1.6 body and out-of-scope items per prompt confirmed.

## Process-gaps

(none new in v1.28 diff scope)

## Convention checks

- Frontmatter `version:` = 1.28: PASS
- All 7 amendment-touched files have `last_amended: 2026-05-05`: PASS
- BC-1.05.036:51 no fan-out / Datadog / Honeycomb residue: PASS (zero matches)
- BC-1.05.035:35 §Description NUL-byte attribution to read_wasm_string error path: PASS
- BC-1.05.036:38 §Description ADR-015-correct emit_internal/FileSink wording: PASS
- BC-1.05.036:135 §Purity Mutex::lock + Vec::push (NOT try_send): PASS (host/mod.rs:109-116 verified)
- BC-1.05.036 §Postcondition 5 enumerates 3 no-event error paths: PASS
- BC-1.05.036 §EC-007 + §Canonical Test Vectors INTERNAL_ERROR rows present: PASS
- BC-1.05.035 + BC-1.05.036 ADR-015 awareness clauses: PASS
- No "Lines: X → Y" footer in v1.28: PASS
- v1.28 H3 section present: PASS
- audit-w16 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: MOSTLY PASS / 1 LOW (L-P30-001)
- No retired-figure residue in non-changelog body: PASS
- TD-VSDD-079 8-term family grep zero non-changelog matches: PASS

## Angle-specific outputs

Diff-only review verified: frontmatter version bump, BC-1.05.036:51 vendor-name + fan-out scrub, BC-1.05.035:35 NUL attribution correction, v1.28 H3 + summary-table row, TD-VSDD-080 declaration. All pass strict diff-only methodology. No new defects in v1.28 surface.
