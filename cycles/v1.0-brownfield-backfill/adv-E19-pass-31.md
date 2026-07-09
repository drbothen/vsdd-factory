# Adversarial Review — E-19 Pass 31 (post-D-784 delta; perimeter = VP-095.md v1.1 + BC-5.42.001 v1.4 + BC-2.07.001 v1.3 + BC-4.13.001 v1.12 + BC-1.17.001 v1.5 + S-19.01/02/03/06/07 at D-784 versions + epic v1.19)

**Perimeter:** E-19 epic v1.19 + S-19.01..S-19.07 + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.4 + BC-4.13.001 v1.12 + BC-2.07.001 v1.3 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095.md v1.1 + VP-096.md v1.1
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 (1 total)
**Streak:** 0/3 (pass-31 NOT-CLEAN; severity improvement from B0/H0/M2/L2 → B0/H0/M1/L0)
**Model family:** Claude Sonnet 4.6
**Delta artifact versions verified:** VP-095.md v1.1 (was v1.0); BC-5.42.001 v1.4 (was v1.3); BC-2.07.001 v1.3 (was v1.2); BC-4.13.001 v1.12 (was v1.11); BC-1.17.001 v1.5 (was v1.4); S-19.01 v1.15 (was v1.14); S-19.02 v1.15 (was v1.14); S-19.03 v1.15 (was v1.14); S-19.06 v1.17 (was v1.16); S-19.07 v1.14 (was v1.13); epic v1.19 (was v1.18).

## Part A — D-784 Delta Verification + New Findings

### Amendment 1 — VP-095.md v1.0 → v1.1 (F-P30-001: four volatile BC-4.13.001 v1.5 pins → stable §Precondition 3 anchor; O-P30-01: 262145-byte @test added)

F-P30-001 fix applied — four volatile "BC-4.13.001 v1.5" pins in VP-095 Full Index row description and Story Anchors row replaced with stable "BC-4.13.001 §Precondition 3" anchor form per TD-VSDD-091 ✓. VP-INDEX v2.55 VP-095 row updated ✓. 262145-byte @test added per §PC6+§Invariant 10 (O-P30-01 closure) ✓. Input-hash ce25941 ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in VP-095.md v1.1 content.

### Amendment 2 — BC-5.42.001 v1.3 → v1.4 (F-P30-002: input-hash placeholder retired → 27acee3)

F-P30-002 fix applied — input-hash placeholder "[pending-recompute]" retired; real hash 27acee3 recorded ✓. O-P26-001 accepted-with-record SUPERSEDED ✓. POLICY 18 satisfied ✓. H1 title UNCHANGED ✓. Input-hash 27acee3 ✓. POLICY 14 5-leg parity confirmed ✓. BC-INDEX v3.84 BC-5.42.001 row updated ✓.

No new findings in BC-5.42.001 v1.4 content.

### Amendment 3 — BC-2.07.001 v1.2 → v1.3 (F-P30-002: input-hash placeholder retired → caea652)

F-P30-002 fix applied — input-hash placeholder "[pending-recompute]" retired; real hash caea652 recorded ✓. POLICY 18 satisfied ✓. H1 title UNCHANGED ✓. Input-hash caea652 ✓. POLICY 14 5-leg parity confirmed ✓. BC-INDEX v3.84 BC-2.07.001 row updated ✓.

No new findings in BC-2.07.001 v1.3 content.

### Amendment 4 — BC-4.13.001 v1.11 → v1.12 (F-P30-003: stale "new crate (to be created)" prose retired)

F-P30-003 fix applied — stale "new crate (to be created)" phrase in §Precondition 3 retired; factory-lock-parse described as designated pure-crate home per D-783 architect ruling ✓. H1 title UNCHANGED ✓. Input-hash e1e1a0a ✓. POLICY 14 5-leg parity confirmed ✓. BC-INDEX v3.84 BC-4.13.001 row updated ✓.

No new findings in BC-4.13.001 v1.12 content.

### Amendment 5 — BC-1.17.001 v1.4 → v1.5 (O-P30-02: domain_invariants TBD → none; F-P30-004 BC version sweep ×10 sites)

O-P30-02 fix applied — §L2 Domain Invariants TBD → `domain_invariants: []` (none; affirmative empty-list form) ✓. Input-hash 03fa998 ✓. POLICY 14 5-leg parity confirmed ✓. BC-INDEX v3.84 BC-1.17.001 row updated ✓.

F-P30-004 BC-1.17.001 cite sweep propagated to S-19.06 v1.17 (Task 8 EC-004/T-010 inserted; v1.4→v1.5 sweep ×10 sites) ✓.

No new findings in BC-1.17.001 v1.5 content.

### Amendment 6 — S-19.01 v1.14 → v1.15 (F-P30-002: BC-5.42.001 v1.3→v1.4 cite sweep ×3 sites)

F-P30-002 cite sweep applied ×3 sites ✓. Input-hash d40bd21 ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.01 v1.15 content.

### Amendment 7 — S-19.02 v1.14 → v1.15 (F-P30-002: BC-5.42.001 v1.4 + BC-2.07.001 v1.3 cite sweep ×3 sites each)

F-P30-002 cite sweeps applied — BC-5.42.001 v1.4 ×3 sites + BC-2.07.001 v1.3 ×3 sites ✓. Input-hash d377821 ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.02 v1.15 content.

### Amendment 8 — S-19.03 v1.14 → v1.15 (BC-2.07.001 v1.2→v1.3 cite sweep ×3 sites)

BC-2.07.001 v1.3 cite sweep applied ×3 sites ✓. Input-hash 8d1225d (unchanged) ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.03 v1.15 content.

### Amendment 9 — S-19.06 v1.16 → v1.17 (F-P30-004: Task 8 EC-004/T-010 inserted; BC-1.17.001 v1.4→v1.5 cite sweep ×10 sites)

F-P30-004 fix applied — Task 8 inserted for EC-004/T-010 (capability_denied error path for read_prefix on out-of-allowlist path); subsequent tasks renumbered ✓. BC-1.17.001 v1.4→v1.5 cite sweep ×10 sites ✓. Input-hash 998ac74 ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.06 v1.17 content.

### Amendment 10 — S-19.07 v1.13 → v1.14 (BC-4.13.001 v1.11→v1.12 cite sweep ×12 sites)

BC-4.13.001 v1.12 cite sweep applied ×12 sites ✓. Input-hash 938e7fb ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.07 v1.14 content.

### Amendment 11 — E-19 epic v1.18 → v1.19 (BC-1.17.001 v1.4→v1.5 cite sweep; §PRD Capabilities Covered ×2 + §Out of Scope ×1 = 3 sites; BC-2.07.001 v1.2→v1.3 cite sweep; EAC-003 ×1 site)

BC-1.17.001 v1.4→v1.5 sweep applied — §PRD Capabilities Covered ×2 + §Out of Scope ×1 = 3 sites ✓. BC-2.07.001 v1.2→v1.3 sweep applied — EAC-003 ×1 site ✓. Input-hash 68a89c0 ✓. POLICY 14 5-leg parity confirmed ✓.

One new finding in the v1.19 content:

**F-P31-001 MEDIUM — E-19 epic v1.19 §Out of Scope BC-1.17.001 bullet introductory phrase retains stale "LANDED as v1.3" when BC-1.17.001 is currently at v1.5.**

The §Out of Scope section of epic v1.19 contains the bullet:

> **BC-1.17.001 host::read_prefix:** LANDED as v1.3 (product-owner, E-19 pass-2 fix burst; subsequently amended through v1.5 — see BC changelog).

The introductory phrase "LANDED as v1.3" is stale — BC-1.17.001 first LANDED at v1.2 (E-19 pass-2 authored the BC), and subsequent amendments have progressed the BC to v1.5 through passes 12, 16, 22, 28, and 30. The phrase "LANDED as v1.3" was never accurate: the BC was introduced at v1.0 via pass-2, received the layering parenthetical at v1.2 (pass-12), and the "LANDED" phrase refers to the point at which the spec was production-grade and ready for TDD implementation. The correct landing version is the version at which the BC reached its intended specification scope.

This is a partial-sweep escape from the D-784 (pass-30) fix burst. The epic v1.18→v1.19 sweep updated 3 sites (§PRD Capabilities Covered ×2 + §Out of Scope ×1 = the section heading line), but the §Out of Scope bullet contained two distinct version-related tokens: the section body reference (updated correctly) and the introductory "LANDED as vX.Y" phrase (not updated). The sweep propagated the BC version update but did not audit the "LANDED as" introductory phrase separately. With BC-1.17.001 now at v1.5, the canonical "LANDED" phrase should reference v1.5 as the current production-grade version, or reflect the actual initial-landing version with an "subsequently amended" qualifier to v1.5.

Additionally, the phrase "LANDED as v1.3" is factually incorrect: BC-1.17.001 was not at v1.3 at any "landing" milestone. The v1.2 version (pass-12) added the layering parenthetical; v1.3 (pass-22) was a metadata-only cite propagation; the BC was operationally production-grade from v1.2 onward. The "LANDED as vX.Y" should reference the most recent production-grade version (v1.5) to avoid misleading the implementer about the canonical version to follow.

**Locus:** E-19 epic v1.19 §Out of Scope BC-1.17.001 bullet — introductory phrase "LANDED as v1.3".
**Routing:** story-writer (epic body content — §Out of Scope bullet text).
**Fix:** Story-writer epic v1.19→v1.20 — §Out of Scope BC-1.17.001 bullet corrected: "LANDED as v1.3" → "LANDED as v1.5 (subsequently amended through v1.5 — see BC changelog)"; clarifies that v1.5 is the current production-grade version the implementer follows. Input-hash 68a89c0 (unchanged — input-hash is computed from referenced input files, not from the epic file content). **CLOSED F-P31-001.**

### Full E-19 Epic and Story Suite Review

All D-784 amendments verified closed as documented above. No further findings in the full E-19 suite (S-19.01 v1.15 / S-19.02 v1.15 / S-19.03 v1.15 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.14). STORY-INDEX E-19 section verified consistent with story versions. BC-INDEX v3.84 / VP-INDEX v2.55 / ARCH-INDEX v2.95 verified. No POLICY violations detected beyond F-P31-001.

## Part B — Severity + Novelty

**Severity (B0/H0/M1/L0):** One MEDIUM finding. Severity improvement from pass-30 (B0/H0/M2/L2) — the D-784 fix burst successfully closed all four pass-30 findings (F-P30-001..004 + O-P30-01/02). Pass-31 finding is a single partial-sweep escape in the epic §Out of Scope bullet introductory phrase.

**Novelty:** LOW (sweep-hygiene-only; propagation-gap class). F-P31-001 belongs to the same partial-sweep-escape class as prior finding classes in this cascade: the D-784 sweep updated 3 sites in the epic but missed a fourth distinct token ("LANDED as vX.Y" introductory phrase) within the same §Out of Scope bullet. This is the same propagation-gap taxonomy as TD-VSDD-060 sibling-sweep misses observed at passes 13/22/27/28/30. No new structural defect class emerged. The finding is isolated to the epic file and does not indicate systemic gaps in the BC/VP/story content.

**Cascade trajectory (pass-23 onward):** 3→4→2→2→4→6→5→4→1. Downward trend from pass-28 peak (6) — passes 29-31: 5→4→1. Asymptotic approach continues.

## Fix Burst Closure (D-785)

**Fix burst D-785 applied.** Story-writer epic v1.19→v1.20 (F-P31-001: §Out of Scope BC-1.17.001 bullet "LANDED as v1.3" → "LANDED as v1.5 (subsequently amended through v1.5 — see BC changelog)"). Input-hash 68a89c0 (unchanged). STORY-INDEX v4.162→v4.163 (SM: epic header v1.20; DAG footnote pass-31 epic update). BC-INDEX / VP-INDEX / ARCH-INDEX UNCHANGED (no BC/VP/ARCH content changes in this burst). STATE.md v5.35→v5.36 (SM: D-785 advance; trajectory →6→5→4→1; checkpoint refresh). Streak 0/3. NEXT: E-19 adv pass-32 (fresh context).
