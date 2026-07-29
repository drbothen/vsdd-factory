---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-07-28T23:59:00Z
phase: 3
inputs:
  - stories/S-21.04-story-worktree-write-path-discipline.md
  - specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md
  - specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
input-hash: "e0bacc0"
traces_to: "BC-6.26.001 v1.15; story v1.30"
pass: 27
verdict: NOT-CLEAN
reviewed_head: "7c3338e7"
fixes_landed_head: "c7c61688"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-26.md"
findings_count: 7
severity_breakdown: "B1/H2/M4/L0"
streak: "0/3"
trajectory_append: 7
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-26 Part A inlined; prior pass files and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy15_residual_closed: "CLOSED — CONTROL-equivalence argument: each corpus vector immediately followed by a CONTROL block invoking the real guard against the unmodified production artifact re-executes on every CI run rather than once; stronger equivalent of a one-time logged restore leg; red-gate-log's own earlier records demonstrate the artifact IS producible"
retraction_note: "Adversary retracted one hypothesis mid-analysis: initially suspected PC2b was ungated; found the harness implements the chain and asserts a REMOVE_LOG sentinel; retraction recorded. Evidence of genuine analysis rather than finding-manufacture."
---

# Adversary Pass 27 — S-21.04 story-worktree write-path discipline

**Date:** 2026-07-28
**Reviewed HEAD:** `7c3338e7`
**Fixes-landed HEAD:** `c7c61688`
**Verdict:** NOT-CLEAN — B1/H2/M4/L0 = 7 findings
**Streak:** 0/3 (BC-5.39.001)

## Finding ID Convention

Finding IDs for this cascade use the format: `F-S2104-P<PASS>-<SEV><SEQ>` (project-local convention established at pass-1). Map to template ADV severity abbreviations: B→BLOCKER, H→HIGH, M→MEDIUM, L→LOW. Full format example: `F-S2104-P27-B01`.

## Mandatory Provenance Disclosure

**(1) Model override:** NO model override applied. Frontmatter `model: opus` → `claude-opus-5`. Fresh context enforced.

**(2) ADR-033 cross-family limitation:** ADR-033 §Decision 2 cross-family independence (GPT-5) NOT satisfied. The reviewing model is in the same family as the authoring agents. Fresh context and information asymmetry are intact; cross-family cognitive diversity is absent. Disclosed per ADR-033 §Decision 3.

**(3) Information asymmetry enforcement:** Pass-26 Part A findings were inlined as permitted context. Prior pass files (pass-1 through pass-25) and the cycle `INDEX.md` were off-limits. The adversary confirms it opened none.

**(4) Hypothesis retraction recorded:** The adversary initially suspected PC2b was ungated. On examination, found the harness implements the chain and asserts a `REMOVE_LOG` sentinel, correctly gating PC2b. The hypothesis was retracted. This is recorded as evidence of genuine analysis rather than finding-manufacture.

## Part A — Fix Verification (Pass-26 Closures)

| ID | Pass-26 Severity | Status | Notes |
|----|-----------------|--------|-------|
| P26-B01 | BLOCKER | GENUINELY-CLOSED | Anchor-target assertion added; deletion axis unaddressed → B01 (new axis) |
| P26-B02 | BLOCKER | PARTIAL | Pass-26 fix applied to write-directive gate but NOT swept into sibling `_guard_e_factory_artifacts` → H01 |
| P26-H01 | HIGH | GENUINELY-CLOSED | AC-001 Gate cell resynced to HEAD |
| P26-H02 | HIGH | GENUINELY-CLOSED | ADR-031 retracted BC claims removed |
| P26-H03 | HIGH | GENUINELY-CLOSED | Leg E broadened to 3 mechanisms |
| P26-H04 | HIGH | GENUINELY-CLOSED (D-939 SUPERSESSION) | 7/8 per-guard records produced in-flight; POLICY 15 restore-leg residual OPEN |
| P26-M01 | MEDIUM | GENUINELY-CLOSED | T-015 registered in 4 inventories |
| P26-M02 (gate) | MEDIUM | GENUINELY-CLOSED | Write-directive gate scope corrected |
| P26-M02 (doc) | MEDIUM | GENUINELY-CLOSED | Constraint scope corrected |
| P26-M03 | MEDIUM | GENUINELY-CLOSED | Guard (k) fail-open fallback removed |
| P26-L01 | LOW | NON-FINDING | Adjudicated non-defect |
| P26-L02 | LOW | GENUINELY-CLOSED | Unbalanced parenthesis fixed |
| POLICY 15 restore-leg residual | — | CLOSED | CONTROL-equivalence argument: corpus vectors followed by CONTROL blocks re-executing real guards on every CI run is a stronger equivalent of a one-time restore leg. Log's own earlier records demonstrate the artifact IS producible. |

**Summary:** 7 GENUINELY-CLOSED · 1 PARTIAL (P26-B02 → H01) · 1 GENUINELY-CLOSED-with-new-defect (P26-B01 → B01 deletion axis) · 1 NON-FINDING (P26-L01) · 1 GENUINELY-CLOSED (P26-H04, D-939 SUPERSESSION) · POLICY 15 residual CLOSED.

**Pass-26 overall closure assessment:** 7 GENUINELY-CLOSED · 3 PARTIAL (B02→H01; B01→B01-new; P26-H04-D939-SUPERSESSION→POLICY-15-residual-pass-27) · 1 N/A (L01 adjudicated non-defect). Pass-26 was the strongest remediation of the cascade.

## Part B — New Findings (or all findings for pass 1)

### BLOCKER

---

**Finding ID:** `F-S2104-P27-B01`
**Severity:** BLOCKER
**Surface:** `worktree-identity-preflight.bats` — the `_guard_*` any-affirmative family
**Policies violated:** POLICY 13 fail-closed; POLICY 15/D-889; TD-VSDD-060

**Description:** The entire `_guard_*` family is **location-blind**. Every helper asserted two conjuncts:
1. Zero nullified occurrences of token T anywhere in the file
2. At least one affirmative occurrence of token T anywhere in the file

Neither conjunct was anchored to the numbered clause that holds the mandate being protected. Consequently, **deleting** a mandate clause outright passed the guard whenever any incidental occurrence of T survived elsewhere in the file.

**Orchestrator-verified token counts in `adversary.md`:**
- `canonical-repo-root`: 7 occurrences
- `factory-artifacts`: 5 occurrences
- `case-insensitive`: 3 occurrences
- `checks out NOTHING under`: 2 occurrences
- `path-corroborated`: 2 occurrences
- `worktree-rooted`: 1 occurrence

**Consequence:** 5 of 7 guards fail-open on deletion (`worktree-rooted` is safe only BY ACCIDENT because its single occurrence lives in the protected clause). The deletion axis is a *structurally* different attack surface than annotation or vocabulary — neither the pass-25 fix (head -1 removal) nor the pass-26 fix (in-place annotation strip) addressed it.

**Root cause diagnosis:** Each fix hardened the predicate the mutant used, never the predicate's shape. A location-blind any-affirmative gate structurally cannot detect removal of the clause it protects. This family has been "closed" three times along three axes:
- Pass-25: `head -1` axis
- Pass-26: in-place annotation axis
- Pass-27: deletion axis

The remedy — a positional conjunct anchoring the affirmative to its numbered clause inside the bounded `#### Worktree-Identity Preflight` section — closes the axis *class* rather than an instance.

**Corpus deletion vectors:** `grep -ciE "delet|remove the (rule|clause|mandate)"` → 0; M1–M9 are all annotation-form. This pass adds M10–M14 as deletion-form corpus vectors.

---

### HIGH

---

**Finding ID:** `F-S2104-P27-H01`
**Severity:** HIGH
**Surface:** `worktree-identity-preflight.bats` — `_guard_e_factory_artifacts`
**Policies violated:** TD-VSDD-060; POLICY 13

**Description:** The pass-26 B01 fix applied an unconditional `grep -iv` exclusion BEFORE computing `fa_nullified`. This was the same root cause diagnosed for the write-directive gate in pass-26 B01 — but the sweep **was not extended to the sibling guard** `_guard_e_factory_artifacts`.

The exclusion is vestigial at HEAD (it excluded 0 lines because the production doc writes `` `factory-artifacts` branch`` with a backtick, which the exclusion pattern may not match). However, it remains an open nullification-hiding channel: a nullifying rewrite phrased in the excluded vocabulary would survive the guard.

This is a TD-VSDD-060 sibling-site sweep failure: the pass-26 B01 fix identified the pre-filter as the root cause and removed it from the write-directive gate, but did not run the sibling sweep to remove it from `_guard_e_factory_artifacts`.

---

**Finding ID:** `F-S2104-P27-H02`
**Severity:** HIGH
**Surface:** Story AC-001 Gate cell
**Policies violated:** POLICY 8; TD-VSDD-091

**Description:** Three false sub-claims in the AC-001 Gate cell:
1. Asserted the bats count-word is `Twenty-one` — false at HEAD, which reads `Twenty-three` (updated in the pass-26 tail commit)
2. Asserted "line 638 was not updated" — false at HEAD
3. Deferred the update as "test-writer follow-up scope" — the work was already done; the deferral advertised an already-closed work item

This is the **ninth** consecutive failure of the coupling mandate written inside that same Gate cell. The coupling mandate survived as prose while the actual count drifted nine times.

Additionally, the volatile `line 638` pin in narrative spec content is a TD-VSDD-091 anti-volatile-pin violation. Narrative spec content MUST use behavioral anchors, not file line numbers.

---

### MEDIUM

---

**Finding ID:** `F-S2104-P27-M01`
**Severity:** MEDIUM
**Surface:** BC-6.26.001, six normative sites
**Policies violated:** POLICY 4

**Description:** The trailing-slash `find` mandate outlived the retraction of its only rationale.

- v1.7 mandated the trailing-slash form (`find "<path>/.factory/"`) on a "defense-in-depth" basis
- v1.12 explicitly retracted that rationale: the trailing slash dereferences symlinks rather than protecting against them; the `[ -L ]` guard is the actual protection mechanism

After v1.12's retraction, the BC's remaining prose described the mandated form *only as a liability* — no replacement rationale was written. Additionally the trailing-slash form was **ungated**: the harness accepted either form.

The mandate became a stranded liability with no stated benefit and an active cost (on a symlink-to-dir, trailing slash causes `find` to traverse the target rather than returning the symlink itself, producing wrong paths in the BLOCKED message).

---

**Finding ID:** `F-S2104-P27-M02`
**Severity:** MEDIUM
**Surface:** BC-6.26.001 §Edge Cases EC-003/EC-004, Canonical Test Vectors T-3/T-4/T-5
**Policies violated:** POLICY 12; TD-VSDD-060

**Description:** Five sites in the BC body used the non-mandated bare `find <path>/.factory -type f` form (without trailing slash and without the normative quoting). These sites are the canonical transcription source for implementers.

This **falsified both prior sweep claims**:
- v1.7 claimed the mandated form was "mandated throughout"
- v1.12's six-site sweep claimed to have normalized all instances

These five vectors were never audited in 26 passes. They are functionally identical under correct preconditions (real directory, no symlinks) but constitute a consistency defect against the normative form.

---

**Finding ID:** `F-S2104-P27-M03`
**Severity:** MEDIUM
**Surface:** BC-6.26.001 §Preconditions PC2a(b) / §Description step 3
**Policies violated:** POLICY 12

**Description:** The `-type f` predicate applied the BC's own harm criterion **inconsistently by depth**:

- EC-008 / T-7: A symlink AT the `.factory` path itself → `find` not invoked; `[ -L ]` guard → PC2b BLOCKED (correct, with notification)
- EC-003 situation inverted: A symlink INSIDE a real shadow `.factory/` directory has type `l`, which is invisible to `-type f`. So `find` exits 0 with empty output → PC2a(b) satisfied → teardown authorized → file removed WITH NO NOTIFICATION

The BC's own harm criterion (§EC-008/T-7: "unexpected non-directory inode silently removed without operator notification") applies identically one level deeper. The BC hardened its top-level symlink chain through versions v1.6→v1.13 while never revisiting the inventory predicate it hands off to.

The same asymmetry applies to FIFOs, sockets, and device nodes inside a real shadow directory.

---

**Finding ID:** `F-S2104-P27-M04`
**Severity:** MEDIUM
**Surface:** `story-worktree-write-path-discipline.bats` — pipeline probe Leg E
**Policies violated:** POLICY 13

**Description:** Pass-26 H03 mechanism 3 was closed only for quote-free comments. The fix used `sed 's/…#[[:space:]][^"]*$//'` which is a no-op on a trailing comment containing a double quote. This restores the `grep -v '_build_'` hiding channel for any comment that happens to contain a double-quote character.

---

### LOW

(No LOW findings this pass.)

---

## Fix Mapping (7 of 7 CLOSED at `c7c61688`; POLICY 15 restore-leg residual CLOSED via CONTROL-equivalence)

| Finding | Owner | Status at `c7c61688` | Notes |
|---------|-------|---------------------|-------|
| **B01** | test-writer | **CLOSED at `dc3b83b3`** | Positional conjunct added to all 7 helpers (each extracts `#### Worktree-Identity Preflight` section via awk; asserts token on its `^N.`-anchored clause line). Corpus vectors M10–M14 covering deletion axis added. Fourth axis on this family; first fix that changed predicate SHAPE rather than vocabulary. |
| **H01** | test-writer | **CLOSED at `dc3b83b3`** | Vestigial `grep -iv` pre-filter removed from `_guard_e_factory_artifacts`; `fa_nullified` now computed from unfiltered output. TD-VSDD-060 sibling sweep applied. |
| **H02** | story-writer | **CLOSED at `dc3b83b3`** | AC-001 Gate cell corrected: `Twenty-one`→`Twenty-three`; stale deferral removed; `line 638` volatile pin replaced with behavioral anchor `test_write_discipline_gates` lead-in + summary comments. New structural preflight test 16 (`test_coupling_gate_story_gate_count_matches_bats_count_word`) asserts Gate cell gate count and bats count-word both equal grepped values, with both drift directions proven. |
| **M01** | product-owner | **CLOSED at `dc3b83b3`** (BC-6.26.001 v1.14→v1.15) | Trailing-slash mandate **RETRACTED** in favour of plain path. Evidence: both forms identical on real directories; plain-path on symlink-to-dir returns symlink itself (type `l`) → PC2b BLOCKED; trailing-slash enumerates target (out-of-scope traversal). bfs 4.1.1 verified. |
| **M02** | product-owner | **CLOSED at `dc3b83b3`** (BC-6.26.001 v1.15) | All 14 live body sites normalized to canonical `find "<worktree-path>/.factory" ! -type d`; 7 historical changelog/contrast sites preserved per append-only policy. |
| **M03** | product-owner | **CLOSED at `dc3b83b3`** (BC-6.26.001 v1.15) | Predicate widened `-type f` → `! -type d`. Catches symlink inside real shadow directory (type `l`, invisible to `-type f`). EC-009 added (EC count 8→9). bfs 4.1.1 verified: no false positives on empty directory. |
| **M04** | test-writer | **CLOSED at `dc3b83b3`** | Comment-strip `[^"]*$` → `.*$` with probe demonstrating both directions (quote-bearing comment now stripped). |
| **STRUCTURAL** | test-writer | **CLOSED at `dc3b83b3`** | Preflight test 16 `test_coupling_gate_story_gate_count_matches_bats_count_word` asserting Gate cell stated gate count and bats count-word both equal grepped values; both drift directions proven. |
| **POLICY 15 restore-leg residual** | state-manager | **CLOSED at `c7c61688` (CONTROL-equivalence)** | Each corpus vector in red-gate-log is immediately followed by a CONTROL block invoking the *real* guard against the unmodified production artifact. This re-executes on every CI run rather than once — a stronger equivalent of a one-time logged restore leg. The log's own earlier records demonstrate the artifact IS producible. "Not producible" would not be defensible. |

**Atomicity note:** Commits `dc3b83b3` (test-writer B01/H01/M04/STRUCTURAL + story-writer H02 + product-owner M01/M02/M03) and `c7c61688` (harness extraction gate + doc-parity patterns updated from `-type f` to `! -type d`) were jointly valid only together; `c7c61688` amends the prior RED commit `8ae161f8`. Both present as a single clean green commit.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 1 |
| HIGH | 2 |
| MEDIUM | 4 |
| LOW | 0 |

**Total findings:** 7 (B1/H2/M4/L0)

**Overall Assessment:** block — one BLOCKER requires fix before next adversary pass.

**Convergence:** findings remain — iterate. Streak: 0/3 (B1 resets).

**Readiness:** requires revision. Pass-28 NEXT.

**Pass-26 closure assessment (orchestrator-verified):** 7 GENUINELY-CLOSED · 3 PARTIAL (B02→H01 sibling-sweep gap; B01→B01-deletion-axis; H04→POLICY-15-residual-now-CLOSED) · 1 N/A (L01 adjudicated non-defect). Pass-26 assessed as the strongest remediation of the cascade.

**Completeness:** BC body audit (§Description, §Preconditions, §Postconditions, §Invariants, §Canonical Test Vectors, §Edge Cases, §Verification Properties, §Traceability, §Architecture Anchors) completed in full — first time in 27 passes. Priority 4 (bats) truncated as authorized. Residual risk: gates (13)–(18) section-wide negative gate bodies, un-re-derived since pass-19 — carry as pass-28 lead.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 27 |
| **New findings** | 7 |
| **Duplicate/variant findings** | 0 (B01=new deletion axis; H01=sibling-sweep gap; H02=ninth-class coupling; M01/M02/M03=BC body first-audit; M04=quote-bearing-comment new subclass; all 7 novel) |
| **Novelty score** | 7/7 = 1.0 |
| **Median severity** | MEDIUM |
| **Trajectory** | 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→12→14→6→17→11→**7** |
| **Verdict** | FINDINGS_REMAIN — streak 0/3; one BLOCKER; pass-28 required |

## Completeness Statement

**Priority-1 surfaces covered (BC body audit — first time in 27 passes):** §Description step-by-step (all 4 steps); §Preconditions (PC1/PC2a/PC2a(b)/PC2b/PC2c — all subcases); §Postconditions (PC2a/PC2a(b)/PC2b/PC2c postconditions); §Invariants (all 5); §Canonical Test Vectors (T-1 through T-8, coverage complete); §Edge Cases (EC-001 through EC-008, all audited — EC-009 not yet present at reviewed-HEAD); §Verification Properties (4 TBD rows noted, internally consistent with `verification_properties: []`); §Traceability (all BC cross-references verified; no mis-anchoring found — POLICY 4/5/6/7 clean on this BC); §Architecture Anchors (all 5 resolve to real workspace artifacts; no mis-anchoring).

**Positive results worth preserving:** PC2b IS behaviourally gated (harness implements the chain and asserts a `REMOVE_LOG` sentinel — adversary retracted initial hypothesis); Preconditions 1–3 gated by doc-parity; the four `(TBD)` VP rows are internally consistent with `verification_properties: []` and are a Perimeter-2/3 catalog concern, correctly not filed as a finding.

**Priority-4 surfaces truncated (authorized):** gates (13)–(23) bodies, T-002/T-003/T-005/T-006 assertion bodies, pipeline probe Legs A–D, `F-S2104-P4-002/003`. **Pass-28 lead:** (a) gates (13)–(18) section-wide negative gate bodies, un-re-derived since pass-19; (b) M03(a) behavioural widening: `! -type d` now blocks on FIFOs, sockets, device nodes inside shadow tree — product-owner argued no false positives; scrutinize on its own terms rather than accepting empirical tests alone.
