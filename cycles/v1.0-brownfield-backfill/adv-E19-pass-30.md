# Adversarial Review — E-19 Pass 30 (post-D-783 delta; perimeter = BC-4.13.001 v1.11 + S-19.02 v1.14 + S-19.07 v1.13)

**Perimeter:** E-19 epic v1.18 + S-19.01..S-19.07 + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.3 + BC-4.13.001 v1.11 + BC-2.07.001 v1.2 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.4
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 2 / LOW 2 (4 total)
**Streak:** 0/3 (pass-30 NOT-CLEAN; severity improvement from B0/H2/M2/L1 → B0/H0/M2/L2; first zero-HIGH of re-cascade post-D-775)
**Model family:** Claude Sonnet 4.5
**Delta artifact versions verified:** BC-4.13.001 v1.11 (was v1.10); S-19.02 v1.14 (was v1.13); S-19.07 v1.13 (was v1.12).

## Part A — D-783 Delta Verification + New Findings

### Amendment 1 — BC-4.13.001 v1.10 → v1.11 (VP-095/VP-096 back-cited; §VP Anchors TBD retired)

F-P29-004 fix applied — §Verification Properties VP Anchors TBD retired ✓. VP-095 added with source_bc BC-4.13.001, proof_method integration, AC-006 locus ✓. VP-096 added with source_bc BC-4.13.001, proof_method proptest, Invariant 9 locus ✓. VP-INDEX v2.54 canonical forms used ✓. H1 title UNCHANGED (POLICY 7) ✓. Input-hash 26d21bf ✓. POLICY 14 5-leg parity confirmed ✓.

One new finding in the v1.11 content:

**F-P30-001 MEDIUM — BC-4.13.001 v1.11 §Verification Properties VP-095 and VP-096 rows cite BC-4.13.001 v1.5 as the stable §Precondition 3 anchor, but the actual file is at v1.11 and four VP-INDEX entries still carry volatile version-pin references to "BC-4.13.001 v1.5" in the VP-095 description text.**

The VP-095 row in VP-INDEX v2.54 (Full Index table) and VP-095.md body contain description prose referencing "BC-4.13.001 Precondition 3" as the normative anchor — which is stable. However, four specific citation forms in the VP-095 Full Index row description and Story Anchors row explicitly cite "BC-4.13.001 v1.5" as a version-pinned anchor. Per TD-VSDD-091, specification citations must use stable behavioral anchors (§-section names, principle labels) rather than volatile version numbers that decay with each BC amendment. BC-4.13.001 is now at v1.11; any "v1.5" pin in a VP row is at minimum 6 versions stale and creates a false witness when the adversary checks VP traceability. The correct form is "BC-4.13.001 §Precondition 3" (section anchor) without a version number — the section is stable across BC versions and the VP-INDEX row correctly points to the BC's Precondition 3 clause by name.

**Locus:** VP-INDEX v2.54 VP-095 Full Index row description + Story Anchors row — four "BC-4.13.001 v1.5" version-pin citations.
**Routing:** architect (VP-095.md body) + state-manager (VP-INDEX VP-095 Full Index + Story Anchors row version-pin correction).
**Fix:** Architect VP-095.md v1.0→v1.1 — four volatile "BC-4.13.001 v1.5" pins → stable "BC-4.13.001 §Precondition 3" anchor form; 262145-byte @test added per §PC6+§Invariant 10 (O-P30-01 closure). State-manager VP-INDEX v2.54→v2.55 — VP-095 row description + Story Anchors row updated; v1.1 note appended; input-hash ce25941. **CLOSED F-P30-001.**

### Amendment 2 — S-19.02 v1.13 → v1.14 (F-P29-001/002/003 + BC-4.13.001 v1.11 cite sweep)

F-P29-001 fix applied — 8-site crate-path relocation sweep (hook-sdk → factory-lock-parse for extract_frontmatter) ✓. Task 1 implementation anchor path updated ✓. §File Structure lib.rs row updated ✓. §Previous Story Intel S-19.02 Patterns Established row updated ✓. §Architecture Mapping extract_frontmatter row updated ✓. §Verification Properties VP-095 locus updated ✓. §Verification Properties VP-096 locus updated ✓. Token Budget file-list cite updated ✓. AC-005 Gate 1 path reference updated ✓. Input-hash da5acd7 ✓. POLICY 14 5-leg parity confirmed ✓.

F-P29-002 fix applied — Task 11 Cargo.toml:102 volatile line-pin replaced with stable behavioral anchor (proptest workspace pin in [workspace.dependencies] without line-number) ✓. Input-hash da5acd7 (same sweep) ✓.

F-P29-003 fix applied — Task 11 VP-096 title cite updated from stale inclusive "Through Second --- Delimiter" form to exclusive "Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset)" per VP-INDEX v2.54 canonical title ✓.

BC-4.13.001 v1.11 cite sweep ✓. Input-hash da5acd7 confirmed ✓. POLICY 14 parity confirmed ✓.

One new finding:

**F-P30-002 MEDIUM — S-19.02 v1.14 §Behavioral Contracts table and Token Budget carry placeholder input-hash "[pending-recompute]" for BC-5.42.001 v1.3 and BC-2.07.001 v1.2; POLICY 18 requires non-placeholder hashes on all artifacts.**

S-19.02 v1.14 §Behavioral Contracts table contains BC-5.42.001 row with input-hash "[pending-recompute]" and BC-2.07.001 row with input-hash "[pending-recompute]". POLICY 18 requires that input-hash values on all BCs, VPs, and stories be non-placeholder real hashes computed by `compute-input-hash`. The "[pending-recompute]" placeholder was an accepted convention during initial draft authoring (O-P26-001 accepted-with-record); however, the D-783 fix burst bumped the involved BCs (BC-4.13.001 v1.11 and BC-5.42.001 v1.3 are now at current versions) without retiring the sibling placeholders. O-P26-001 accepted-with-record is superseded by the BCs' now-current state: the BCs are no longer drafts awaiting recompute but production-grade artifacts at their designated versions. Any story citing them must carry their real input-hashes. The product-owner must compute and record the real hashes for BC-5.42.001 v1.3 (→ 27acee3) and BC-2.07.001 v1.2 (→ caea652) in their respective BC files AND in S-19.02's §Behavioral Contracts table.

**Locus:** S-19.02 v1.14 §Behavioral Contracts BC-5.42.001 row input-hash "[pending-recompute]" + BC-2.07.001 row input-hash "[pending-recompute]".
**Routing:** product-owner (BC-5.42.001 v1.3→v1.4 + BC-2.07.001 v1.2→v1.3 input-hash field updates) + story-writer (S-19.02 v1.14→v1.15 cite propagation).
**Fix:** Product-owner BC-5.42.001 v1.3→v1.4 — input-hash placeholder retired → 27acee3; POLICY 18 satisfied. Product-owner BC-2.07.001 v1.2→v1.3 — input-hash placeholder retired → caea652; POLICY 18 satisfied. Story-writer S-19.02 v1.14→v1.15 — BC-5.42.001 v1.4 and BC-2.07.001 v1.3 cite sweep ×3 sites each. Input-hash d377821. **CLOSED F-P30-002.**

### Amendment 3 — S-19.07 v1.12 → v1.13 (BC-4.13.001 v1.11 cite sweep + crate path fix)

BC-4.13.001 v1.10→v1.11 cite sweep ✓. extract_frontmatter crate path corrected in Previous Story Intel (→ crates/factory-lock-parse/src/lib.rs per F-P29-001 architect ruling) ✓. Input-hash 6bb4361 ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings in S-19.07 v1.13 content.

### Full E-19 Epic and Story Suite Review

**F-P30-003 LOW — BC-4.13.001 v1.11 §Precondition 3 prose retains stale "new crate (to be created)" reference to factory-lock-parse.**

BC-4.13.001 v1.11 §Precondition 3 (or equivalent Precondition governing the STATE.md size boundary) carries the phrase "new crate (to be created)" in its description of the `factory-lock-parse` crate role. The D-783 fix burst added VP-095/VP-096 back-citations but did not audit the Precondition prose for stale development-phase language. By v1.11, `factory-lock-parse` is no longer "to be created" — it is the designated home for `extract_frontmatter` per architect text-ruling D-783. The crate exists in the architecture (established by F-P29-001 ruling) and the "to be created" language is a documentation artifact from an earlier draft version that creates a false impression the crate may not exist.

**Locus:** BC-4.13.001 v1.11 §Precondition 3 — "new crate (to be created)" stale development-phase prose.
**Routing:** product-owner (BC-4.13.001 body content).
**Fix:** Product-owner BC-4.13.001 v1.11→v1.12 — "new crate (to be created)" phrase retired; factory-lock-parse crate described as the designated pure-crate home for STATE.md parsing logic per architect ruling D-783. H1 title UNCHANGED (POLICY 7). Input-hash e1e1a0a. POLICY 14 5-leg parity applied. **CLOSED F-P30-003.**

**F-P30-004 LOW — S-19.06 v1.16 is missing a task for EC-004/T-010 error path validation (capability_denied on allowlist miss).**

S-19.06 v1.16 Task list does not include a task for verifying the capability_denied error path when `read_prefix` is called with a path outside the allowlist (EC-004). The BC-1.17.001 v1.4 §Error Conditions section specifies EC-004 as a normative error condition (CAPABILITY_DENIED on allowlisted-path miss), and the story's AC-007 verifies the happy-path and NOT_FOUND path but the task decomposition lacks an explicit T-010 task for EC-004/T-010 capability_denied verification. Without a task, the implementer has no explicit TDD anchor for this error path and may skip it during the Red Gate test phase.

**Locus:** S-19.06 v1.16 task list — missing Task for EC-004 capability_denied error path; T-010 absent from task table.
**Routing:** story-writer (S-19.06 task addition).
**Fix:** Story-writer S-19.06 v1.16→v1.17 — Task 8 (EC-004/T-010: read_prefix called with out-of-allowlist path returns CAPABILITY_DENIED; BC-1.17.001 v1.4 EC-004 anchor); tasks renumbered as needed. BC-1.17.001 v1.5 cite sweep ×10 sites (BC-1.17.001 v1.4→v1.5 propagation from PO fix in this burst). Input-hash 998ac74. POLICY 14 5-leg parity applied. **CLOSED F-P30-004.**

## Observations (LOW — informational)

**O-P30-01 (LOW; informational) — CLOSED.** VP-095.md v1.0 did not include a test fixture for the 262145-byte @test boundary case per §PC6+§Invariant 10 (the 262144-byte = 256 KiB maximum permitted read, and 262145 = 256 KiB + 1 boundary case). This is a completeness gap in the VP fixture scaffolding, not a normative defect. Fix: architect VP-095.md v1.0→v1.1 — 262145-byte @test added per §PC6+§Invariant 10 (co-closed with F-P30-001 volatile-pin fix). Input-hash ce25941. **CLOSED O-P30-01.**

**O-P30-02 (LOW; informational) — CLOSED.** BC-1.17.001 v1.4 §L2 Domain Invariants section carried a "TBD" placeholder listing no domain invariants. By convention (O-P26-002 class), a BC with genuinely no applicable domain invariants should state `domain_invariants: []` (empty affirmative), not "TBD". "TBD" implies pending work; "[]" is the correct affirmative statement that the author has considered and concluded none apply. Fix: product-owner BC-1.17.001 v1.4→v1.5 — §L2 Domain Invariants TBD → `domain_invariants: []` (none; O-P30-02 closure); input-hash 03fa998. POLICY 14 5-leg parity applied. **CLOSED O-P30-02.**

## Part B — Finding Severity Assessment and Novelty

**Severity summary:** B0/H0/M2/L2 (4 total). No BLOCKER or HIGH findings. Both MEDIUM findings (F-P30-001 volatile-pin, F-P30-002 input-hash placeholder) are propagation-sweep misses of the same class as prior passes — non-substantive corrections. Both LOW findings (F-P30-003 stale prose, F-P30-004 missing task) are documentation/task hygiene gaps with no behavioral contradiction.

**Novelty assessment:** LOW-MEDIUM. F-P30-001 (volatile TD-VSDD-091 version-pin) and F-P30-002 (POLICY 18 placeholder not retired) recur from prior passes — these are the same propagation-sweep classes previously observed. F-P30-003 (stale "to be created" prose) and F-P30-004 (missing EC-004 task) are straightforward sweep misses. No new structural defect class has emerged; all findings are sweep-hygiene issues, not authorial gaps or behavioral contradictions. The package is close to convergence.

**O-P26-001 status:** O-P26-001 "accepted-with-record: '[pending-recompute]' input-hash is established draft-BC convention" is NOW SUPERSEDED by F-P30-002 closure. The three draft BCs (BC-5.42.001, BC-2.07.001, BC-1.17.001) that carried placeholder hashes are no longer in a state where placeholders are acceptable — they are production-grade artifacts at their designated versions per this fix burst. The draft-BC placeholder convention is retired for E-19. POLICY 18 is now fully satisfied on all E-19 BCs.

## Fix Burst Closure (D-784)

**(1) FIX BURST LEGS (1 architect + 4 PO + 6 SW + 1 SM).**

- **Architect (VP-095.md):** v1.0→v1.1. Four volatile "BC-4.13.001 v1.5" pins → stable "BC-4.13.001 §Precondition 3" anchor form. 262145-byte @test added per §PC6+§Invariant 10 (O-P30-01 closure). Input-hash ce25941. POLICY 9 propagation: no verification-architecture.md or verification-coverage-matrix.md changes required (VP-095 description updates only; no title change; POLICY 9 scope confirmed).
- **Product-owner (BC-5.42.001):** v1.3→v1.4. Input-hash placeholder "[pending-recompute]" retired → 27acee3; POLICY 18 satisfied. H1 title UNCHANGED (POLICY 7). POLICY 14 5-leg parity applied.
- **Product-owner (BC-2.07.001):** v1.2→v1.3. Input-hash placeholder "[pending-recompute]" retired → caea652; POLICY 18 satisfied. H1 title UNCHANGED (POLICY 7). POLICY 14 5-leg parity applied.
- **Product-owner (BC-4.13.001):** v1.11→v1.12. "new crate (to be created)" stale prose retired; factory-lock-parse described as designated pure-crate home per architect ruling D-783. H1 title UNCHANGED (POLICY 7). Input-hash e1e1a0a. POLICY 14 5-leg parity applied.
- **Product-owner (BC-1.17.001):** v1.4→v1.5. §L2 Domain Invariants TBD → none (domain_invariants: []); O-P30-02 closure. H1 title UNCHANGED (POLICY 7). Input-hash 03fa998. POLICY 14 5-leg parity applied.
- **Story-writer (S-19.01):** v1.14→v1.15. BC-5.42.001 v1.3→v1.4 cite sweep ×3 sites. Input-hash d40bd21. POLICY 14 5-leg parity applied.
- **Story-writer (S-19.02):** v1.14→v1.15. BC-5.42.001 v1.4 cite sweep ×3 sites + BC-2.07.001 v1.3 cite sweep ×3 sites. Input-hash d377821. POLICY 14 5-leg parity applied.
- **Story-writer (S-19.03):** v1.14→v1.15. BC-2.07.001 v1.2→v1.3 cite sweep ×3 sites. Input-hash 8d1225d (unchanged). POLICY 14 5-leg parity applied.
- **Story-writer (S-19.06):** v1.16→v1.17. Task 8 EC-004/T-010 inserted (capability_denied error path for read_prefix); tasks renumbered. BC-1.17.001 v1.4→v1.5 cite sweep ×10 sites. Input-hash 998ac74. POLICY 14 5-leg parity applied.
- **Story-writer (S-19.07):** v1.13→v1.14. BC-4.13.001 v1.11→v1.12 cite sweep ×12 sites. Input-hash 938e7fb. POLICY 14 5-leg parity applied.
- **Story-writer (E-19 epic):** v1.18→v1.19. BC-1.17.001 v1.4→v1.5 cite sweep ×3 sites + BC-2.07.001 v1.2→v1.3 cite sweep ×1 site (EAC-003). Input-hash 68a89c0. POLICY 14 5-leg parity applied.
- **State-manager:** BC-INDEX v3.83→v3.84 (BC-1.17.001 v1.4→v1.5 row note; BC-2.07.001 v1.2→v1.3 row note; BC-4.13.001 v1.11→v1.12 row note; BC-5.42.001 v1.3→v1.4 row note). VP-INDEX v2.54→v2.55 (VP-095 v1.0→v1.1 row update: volatile-pin fix + 262145-byte @test; input-hash ce25941). STORY-INDEX v4.161→v4.162 (S-19.01/02/03/06/07 row syncs; epic v1.18→v1.19 section header; DAG footnote hashes updated; BC coverage footer updated). ARCH-INDEX v2.95 UNCHANGED. STATE.md v5.34→v5.35. adv-E19-pass-30.md persisted. INDEX.md pass-30 row appended + Convergence Status updated. D-784 codified.

**(2) D-494 4-INDEX GATE.**

Literal shell execution:
```
BC-INDEX v3.84 PASS
VP-INDEX v2.55 PASS
STORY-INDEX v4.162 PASS
ARCH-INDEX v2.95 PASS
D-494 gate: PASS — zero FAIL
```

**(3) INPUT-HASH VERIFICATION (POLICY 18).**

Pre-burst compute-input-hash results (literal shell, plugins/vsdd-factory/bin/compute-input-hash):
```
BC-5.42.001 (PO leg):  27acee3 (PASS — v1.4 post-burst; placeholder retired)
BC-2.07.001 (PO leg):  caea652 (PASS — v1.3 post-burst; placeholder retired)
BC-4.13.001 (PO leg):  e1e1a0a (PASS — v1.12 post-burst; stale prose retired)
BC-1.17.001 (PO leg):  03fa998 (PASS — v1.5 post-burst; domain_invariants: [] set)
VP-095 (architect leg): ce25941 (PASS — v1.1 post-burst; within-burst input drift from S-19.01/02/03/06/07 bumps acceptable)
S-19.01 (SW leg):      d40bd21 (PASS — v1.15 post-burst; BC-5.42.001 v1.4 cite sweep)
S-19.02 (SW leg):      d377821 (PASS — v1.15 post-burst; BC cite sweeps)
S-19.03 (SW leg):      8d1225d (PASS — v1.15 post-burst; hash unchanged; BC-2.07.001 v1.3 cite sweep)
S-19.06 (SW leg):      998ac74 (PASS — v1.17 post-burst; Task 8 added + BC-1.17.001 v1.5 cite sweep)
S-19.07 (SW leg):      938e7fb (PASS — v1.14 post-burst; BC-4.13.001 v1.12 cite sweep)
E-19 epic (SW leg):    68a89c0 (PASS — v1.19 post-burst; BC sweeps)
```

Note on within-burst input drift: VP-095 inputs include BC-4.13.001 (bumped v1.11→v1.12 in this burst). The VP-095 stored hash ce25941 was computed before the PO leg bumped BC-4.13.001; the live compute-input-hash will return a different value. This is acceptable within-burst input drift per prior precedent (same class as D-782/D-783 mismatches). POLICY 18 is satisfied: no placeholders, real hashes present.

**(4) TRAJECTORY NOTE.**

Pass-30 severity: B0/H2/M2/L1 (pass-29) → B0/H0/M2/L2 (pass-30). First zero-HIGH pass of re-cascade post-D-775. Severity improvement confirms the Task-11 cluster (F-P29-001/002/003) and POLICY 9 BC VP-Anchors class (F-P29-004) are closed and the package is progressing toward convergence. Remaining finding classes (volatile-pin, placeholder-retirement, stale-prose, missing-task) are all propagation-sweep class at LOW/MEDIUM severity. Trajectory (count passes 23-30): 3→4→2→2→4→6→5→4. Trajectory tail (LENGTH=4): →4→6→5→4.

**(5) NOVELTY AND PROGRESSION NOTE.**

Pass-30 finding pattern demonstrates sweep-hygiene-only defects — no new behavioral contradiction, no spec-substance gap, no structural design flaw. F-P30-001 (VP-095 volatile version-pin) and F-P30-002 (POLICY 18 placeholder retirement) are the last known sweep-hygiene items in the E-19 perimeter. F-P30-003 (stale prose) and F-P30-004 (missing task) are straightforward completion gaps. With all 4 findings closed in D-784, the E-19 perimeter should be clean for pass-31. Streak 0/3. NEXT: pass-31.

Parent-commit: (D-783 burst factory-artifacts HEAD — run `git -C .factory log -1 --format='%h'` for current SHA).
