---
pass: 15
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 1199aae360794e1caa9d905959eea04d2bdaf2da
novelty: null
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-14.md"
---

## Summary

VERDICT: NOT-CLEAN. Counts: BLOCKER 0, HIGH 0, MEDIUM 1, LOW 0, NIT 0 = 1. Streak RESETS 1/3 → 0/3 (human perimeter decision: IN-PERIMETER). Trajectory pass-10=10, 11=1, 12=1, 13=2, 14=0, 15=1 (tail →2→0→1).

## Part A — Findings

F-S2107-P15-001 (MEDIUM) — STORY-INDEX E-21 epic-total aggregation cells disagree: 111 pts (L721 authored-provenance blockquote tail) vs 117 pts (L741 delivery blockquote) for the identical 14-story E-21 set. POLICY 5 v1.3.7 SIBLING-SWEEP CATEGORY (i) same-file aggregation cells + TD-VSDD-060. Catalog rows (L726-739) sum to 117 (S-21.01=11,.02=3,.03=3,.04=5,.05=5,.06=8,.07=11,.09=16,.10=5,.11=16,.12=8,.13=13,.14=8,.15=5); L741 self-enumeration = 117 (authoritative); L721 "111" wrong by 6, reconciles to no plausible value. Provenance totals are maintained-current not frozen (the "8 stories; 62 pts" snapshot requires S-21.09's re-estimated 16 pts, proving running totals were swept for later re-estimates) → stale un-swept live aggregation cell, not POLICY-1 historical. Perimeter note: this is E-21 epic aggregation (S-21.10-S-21.15 pts), not S-21.07's own datum (S-21.07=11, correct everywhere); adversary flagged the orchestrator may reclassify as cross-story/system-level (would NOT reset) — HUMAN DECISION this session: IN-PERIMETER, resets. Fix: L721 111→117 (state-manager).

### Independent axes re-derived CLEAN (no finding)

Independent axes re-derived CLEAN (no finding): retracted-claim class (fuel_cap/10M/20M/calibrat/on_error/BC-version) ZERO live members whole-story (AC-019 L508-513, AC-020 Notes L569-608, Out-of-Scope L798 all correctly qualified/retracted/forward-looking/append-only; backtick `fuel_cap` L586/L596 checked); POLICY 7 H1 parity BC-5.39.010 H1 (BC L111)=BC-INDEX title (L1464)=story BC-table (L783)=STORY-INDEX catalog (L732) verbatim; POLICY 14 leg-5 BC-INDEX chain …|v1.17|v1.18 synced; BC-version currency story cites v1.18 throughout, BC frontmatter=1.18, STORY-INDEX catalog+coverage both v1.18; POLICY 18 three-way input-hash 7bc1850 (story L54=catalog L732=blockquote L741); load-bearing code claim DEFAULT_FUEL_CAP=20_000_000 at invoke.rs:279 accurate; count parity 24 ACs / 34 story-ECs matches STORY-INDEX; D-996/D-997 pass-record internal consistency confirmed.

### Observations

Observations: O-P15-01 [process-gap]: the 111-vs-117 class survived passes 13-14 because prior passes checked the delivery blockquote but not the sibling authored-provenance blockquote; STORY-INDEX-aggregation review needs a "sum every points/count total in ALL epic blockquotes and diff against catalog sums" mechanical step (route: adversary-review rubric / S-15.03 automation). O-P15-02 (non-finding): AC-020 Notes L596 illustrative grep `grep -c 'fuel_cap|failure_policy' hooks-registry.toml` lacks -E so literal-pipe is vacuous; illustrative prose not a POLICY-15 attestation; NOT a finding but if ever promoted to load-bearing must become grep -cE. O-P15-03 (out-of-perimeter, non-finding): BC-5.39.010 EC-034/EC-035 (BC v1.12) not mirrored to story EC table (story EC-033→EC-036), coverage carried by story AC-022/023; POLICY 8 EC-mirror is story→BC directional so not a violation.

## Part B — Streak / Trajectory

- Streak: **0/3** (BC-5.39.001 — RESET from 1/3 by human perimeter decision this session: F-S2107-P15-001 classified IN-PERIMETER, therefore counts against the streak per the standing BC-5.39.001 protocol; a NOT-CLEAN verdict resets any in-progress streak).
- Trajectory: `47→18→25→25→24→20→16→8→10→1→1→2→0→1` (tail: `→1→2→0→1`, D-433(e)+D-439(c) LENGTH=4).
- 14 true adversary reviews; 1 CLEAN verdict (pass-14).
- Next gate: **pass-16 adversary** (fresh-context, reads `adversary-pass-15.md` Part A only per the Iron Law). 3 fresh CONSECUTIVE CLEAN passes are now required from pass-16 onward to converge (BC-5.39.001).
