# Adversarial Review — E-9 v1.29 Inverse-Traceability Fix (D-274) — Pass 32

**Date:** 2026-05-05
**Commit reviewed:** 699785f (v1.28 → v1.29)
**Cumulative surface:** v1.7..v1.29 (4 files + open-questions.md + 2 BCs)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Reading-order audit — read all 7 amendment-surface files in story-writer's natural consumption order (epic → gap-analysis → audit-w16 → perf-baseline → BC-1.05.035 → BC-1.05.036 → open-questions), testing sequential coherence + pinpoint citation accuracy + tense/voice consistency along the chain. NEW per TD-VSDD-057.

## Summary

All v1.29 fixes propagated correctly: tense correction MED-P31-001 present at gap-analysis:334 + audit-w16 B-7; outcome enum + exit_code mapping at BC-1.05.036:49; stdin write cite :262→:259; perf-baseline "10k events/minute" wording. All convention checks PASS. TD-VSDD-079 8-term grep returns zero non-changelog matches. v1.7-v1.29 changelog blocks intact. Source-code citations verified (12+ sites at exec_subprocess.rs + ADR-015 + mod.rs:184). 3 LOW findings (cosmetic citation precision, POLICY-1 changelog cosmetics, out-of-scope S-9.00 question). No HIGH or MED.

## Findings

### HIGH
None.

### MED
None.

### LOW

**LOW-P32-001 [LOW]: BC-1.05.036 Postcondition 2 outcome enum citation precision (D-15.2:270 vs D-15.3 enrichment).** Cosmetic; both decisions exist; reword as "host-enrichment layer (per D-15.3) stamps outcome enum defined in D-15.2:270".

**LOW-P32-002 [LOW] (POLICY 1 immutable): v1.29 changelog cites gap-analysis:334-337 but actual edit footprint 334-339.** Cosmetic; POLICY 1 makes immutable.

**LOW-P32-003 [LOW] (out-of-scope-but-noted): perf-baseline:71 cold_start_p95_measured_ms = 642.6 vs E-9 AC-3 hard gate ≤500ms apparent contradiction.** S-9.00 spec scope; AC-3 likely applies to post-Tier-2 delta not pre-Tier-2 baseline floor; not addressed in either document.

## Out-of-scope-but-noted

- audit-w16:166 "D-2 Option C" deferred per L-P14-001.
- ADR-015:438 contains "fan-out" in legitimate prose; ADR-015 out-of-scope for amendment surface.

## Process-gaps

(none new in pass-32)

## Convention checks

All PASS:
- Frontmatter `version:` = 1.29
- All 7 files have `last_amended: 2026-05-05`
- gap-analysis:334+ tense correction present
- audit-w16 B-7 row tense correction present
- BC-1.05.036 outcome enum field + exit_code→outcome mapping
- BC-1.05.036 stdin cite :259 (not :262)
- perf-baseline "10k events/minute" wording
- TD-VSDD-079 8-term family grep zero non-changelog matches
- BC-1.05.036:38/51/135 ADR-015-correct wording
- BC-1.05.035 awareness clause + NUL byte attribution + precedence ladder
- INTERNAL_ERROR enumeration in §Postcondition 5/§EC-007/§Canonical Test Vectors
- OQ-W16-001 binary acceptance (a) AND-link
- v1.30 reserved row present
- POLICY 1 — v1.7..v1.29 changelog blocks intact

## Angle-specific outputs

Reading-order chain verified: epic → gap-analysis → audit-w16 → perf-baseline → BC-1.05.035 → BC-1.05.036 → open-questions. No sequential contradictions detected. Source-code citations all resolve at exec_subprocess.rs:148/155/162/169/252/258/259/267/268/270/299. ADR-015 line citations all resolve (270 outcome, 329 capability.denied, 437 10k events/min, 634 Wave 3 AC-2). Triangle anchor verified: epic:386 → OQ-W16-001:21 → gap-analysis:326 → OQ-W16-001 (closed).

Cumulative v1.7-v1.29 fix work has progressively closed substantive issues. Reading-order audit could only surface micro-level precision items.
