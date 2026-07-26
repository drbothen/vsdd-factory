---
pass: 12
verdict: NOT-CLEAN
reviewed_head: 92f986ab
fixes_landed_head: 264f53b6
novelty: 0.51
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-11.md"
---

> **Human ruling (AskUserQuestion, orchestrator session, 2026-07-26):** "Keep looping unchanged" — full-perimeter loop continues; perimeter-freeze and asymptotic-acceptance both declined; adversary's executable-predicate prescription ADOPTED as mandatory for all fix-wave records.

## Summary

Pass-12 adversarial review of S-21.04 implementation. 10 findings + 1 deferred cross-story (B0 / H3 / M6 / L1). Novelty 0.51. Trajectory 14→18→17→12→11→11→9→9→10→11→7→10 (plateau broken upward from pass-11 trough). Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P12-001 | HIGH | STORY-INDEX:733 vs :724-728 | D-907 scope note asserts catalog-row BC cites "stay versionless" — every E-21 catalog row carries a versioned BC pin and F-P11-001 enforced the opposite one leg earlier in the same burst; note instructs the next state-manager to strip pins POLICY 14/17 requires | POLICY 14/17, 19, 4; TD-VSDD-060 |
| F-S2104-P12-002 | HIGH | red-gate-log:287,:43,:14/:25 vs bats:503,523,533 | Pass-11 gate strengthening left no attestation: :287 still calls the four primary gates "co-occurrence form" (three are .md-qualified now); no mutant record for the P11-003 legs; Summary GREEN cites HEAD 2c8eff8b though gate regexes changed at 92f986ab — F-P10-007 class recurrence | POLICY 15; TD-VSDD-059 |
| F-S2104-P12-003 | HIGH | story:92 vs bats:454-495; _shared-context:67-70,113 | AC-001 Gate over-claims "manual-fallback only for prose review" while T-001 had NO gate over AC-001(a) — the CWD-relative-path PROHIBITION (BC PC1 core); deleting the prohibition sentences left all 9 tests GREEN | BC PC1; POLICY 11, 14/17; TD-VSDD-059 |
| F-S2104-P12-004 | MEDIUM | story:254 vs :131 | Range fix applied to Architecture Mapping only; File Structure bats row still AC-002..AC-006 — byte-identical sibling sentence self-contradicting in one document | POLICY 14/17; TD-VSDD-060 |
| F-S2104-P12-005 | MEDIUM | story:238 | §Test Plan T-001 row maps only AC-003 though AC-001's Gate cites T-001 — reverse traceability absent | POLICY 14/17, 4 |
| F-S2104-P12-006 | MEDIUM | story:104,124-132 vs BC:323-327; ADR:248 | Count-free rewrite attributes five GROUPS (18 entries) to BC's five-FILE classification; identity-bats filed under Awareness while a Tests group exists | POLICY 4, 5 |
| F-S2104-P12-007 | MEDIUM (pattern) | bats ×12 | "predicate zero present-tense survivors" falsified: fix corrected 1 of 13 sites; 12 siblings carry historical opening but no closing SHA — third generation (P10-010→P11-006→here) | POLICY 15; TD-VSDD-059/060 |
| F-S2104-P12-008 | MEDIUM | bats:548,549,552 | Three stale :531 line pins after pass-11 inserted 3 lines above; one inside an EMITTED failure message misdirecting operators | TD-VSDD-091/060 |
| F-S2104-P12-009 | MEDIUM | bats:44-50,:1039-1048 | Header test-plan enumerates 6 of 9 tests; T-008 has no in-file anchor while T-007/T-009 do | POLICY 14/17; TD-VSDD-060 |
| F-S2104-P12-010 | LOW | red-gate-log:24 vs :25,:14,:7 | Attestation chain non-monotonic: D-906 dated 2026-07-26, D-907 (later) dated 2026-07-25 | POLICY 15 |
| F-S2104-P12-D1 | deferred cross-story → wave-gate | STORY-INDEX:719,720 vs :729,731 | E-21 header rows say "5 stories; 27 pts; 2 waves" + DAG "(2 waves)" vs 6 stories/35 pts/W3 — S-21.06 registration propagation gap | — |

---

## Observations (NOT findings)

Behavioral axis holds (chain order verified; ordering gate load-bearing; extractions bound correctly); retired-class sweeps clean; 6 sibling surfaces delegate by reference — enumeration-gate asymmetry creates no live exposure, recorded not filed; T-007/T-009 gates satisfied; CHANGELOG accurate, Task 11 satisfied; [process-gap] sweep-predicate attestations prose-declared not reproducible — three consecutive waves shipped "zero survivors" claims falsified next pass; remedy: burst record carries the LITERAL predicate command + captured stdout (D-449(a) applied to per-story fix waves) — codification target: per-story fix-wave record + adversarial-review SKILL post-fix attestation requirements. Diagnosis: "Until fix-wave predicates are recorded as executable text with captured output, each pass will keep converting one closed finding into one new one."

---

## Per-Pass-11 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-P11-001 | CONFIRMED-CLOSED | BC v1.11 pin sync resolved; residue → P12-001 scope-note contradiction |
| F-P11-002 | PARTIAL | AC-001 Gate column fixed; range parity PARTIAL; Architecture Mapping fixed but File Structure row still AC-002..AC-006 → P12-004; T-001 trace absent → P12-005 |
| F-P11-003 | CONFIRMED-CLOSED | Three sibling gates strengthened to .md-qualified form; attestation residue → P12-002 |
| F-P11-004 | CONFIRMED-CLOSED | Architecture Mapping count-free attribution rewrite complete; description stale → P12-002a |
| F-P11-005 | CONFIRMED-CLOSED | Mutant count reconciled 5/9/8; note stale form → P12-002a |
| F-P11-006 | PARTIAL | Header historical label corrected; 12 siblings still present-tense → P12-007 |
| F-P11-007 | CONFIRMED-CLOSED | S-21.02/S-21.03 catalog-row ADR pins versionless; note over-generalized → P12-001 |

Tally: 5 CONFIRMED-CLOSED / 2 PARTIAL / 0 REGRESSED + 1 falsified sweep attestation.

---

## Fix Mapping

| Finding | Status | Fix Agent / Commit |
|---------|--------|-------------------|
| F-S2104-P12-001 | FIXED this burst | state-manager D-908 |
| F-S2104-P12-002 | FIXED this burst | state-manager D-908 |
| F-S2104-P12-003 | FIXED 264f53b6 | test-writer — 2 mutants recorded |
| F-S2104-P12-004 | FIXED 7d4650fb | story-writer |
| F-S2104-P12-005 | FIXED 7d4650fb | story-writer (+in-scope T-002/AC-002 discovery) |
| F-S2104-P12-006 | FIXED 7d4650fb | story-writer |
| F-S2104-P12-007 | FIXED 264f53b6 | test-writer (predicate stdout) |
| F-S2104-P12-008 | FIXED 264f53b6 | test-writer |
| F-S2104-P12-009 | FIXED 264f53b6 | test-writer |
| F-S2104-P12-010 | FIXED this burst | state-manager D-908 |
| F-S2104-P12-D1 | FIXED this burst | state-manager D-908 (orchestrator adjudication: index rows are state-manager domain; fix now) |
