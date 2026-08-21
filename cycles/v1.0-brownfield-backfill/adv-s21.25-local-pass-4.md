---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-3.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
input-hash: "622af2b"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 4
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-3.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 4) — NOT-CLEAN

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.3 (input-hash `4af3ec2`);
`BC-1.03.019.md` v1.2; `BC-3.08.001.md` v1.26; `VP-079.md` v1.21. Rubric: full
`.factory/policies.yaml` (POLICY 1-22). This is pass 4 of the S-21.25 LOCAL pre-TDD cascade,
reviewing the D-1061 pass-3 remediation (Task 7/11 test-distribution correction + VP-079 SITE_7
acknowledgment paragraph) applied against pass 3's findings, and independently re-checking every
live VP-079 version cite and quotation in the story against VP-079's own concurrent evolution.

## Verdict: NOT-CLEAN

1 MEDIUM finding. LOCAL streak 0/3 (a NOT-CLEAN pass resets the streak per BC-5.39.001; pass 3's
fixes do not themselves count as a clean pass).

## Part A — Fix Verification (pass 3 findings)

**F-S2125-P3-001** (Task 7/11 test-distribution silently dropped 3 `emit_event.rs` tests):
**VERIFIED FIXED, no recurrence**. Task 7 and Task 11 both state the accurate 14
(`invoke.rs`) + 3 (`emit_event.rs`) + 1 (separate integration file) = 18 distribution, matching the
File Structure Requirements table. Re-derivation against the current v1.3 body confirms the
per-AC assignment is internally consistent and the "17+1" miscount class does not recur.

**F-S2125-P3-002** (story silent on VP-079's SITE_7 registration/obligation): **VERIFIED FIXED, but
surfaced a follow-on staleness issue (see Part B).** The VP status note's "VP-079 SITE_7
acknowledgment" paragraph is present and correctly states the relationship and the Phase-6
deferral. However, checking this paragraph's *version cite* against VP-079's actual current state
(VP-079 had advanced from v1.20, the version current at pass-3 remediation time, to v1.21
concurrently with an architect-side amendment) surfaced that the paragraph's `VP-079 v1.20` cite —
correct when written — was now one version behind, and its direct quotation of v1.20's SITE_7
scope-note text ("tracked for the test-writer at S-21.25 delivery time") no longer matches what
VP-079 v1.21 actually says (VP-079 v1.21 retargeted that same sentence to Phase-6
formal-verification, per F-S2125-P3-002's own architect-side half of the fix). This is not a
recurrence of F-S2125-P3-002's original defect (silence) — the story is no longer silent, it is
now stale, a distinct failure mode this pass classifies separately below.

Both prior structural-fix sets — the pure-helper extraction + effectful-shell split (D-1059, pass 1)
and the AC-005 relocation + emitter rename (D-1060, pass 2) — CONFIRMED HELD at this pass, no
recurrence.

## Part B — New Findings

### MEDIUM

#### F-S2125-P4-001: Concurrency residue — VP-079 v1.20→v1.21 cite and quotation left stale by the architect's own parallel fix
- **Severity:** MEDIUM
- **Category:** concurrency residue / stale cross-reference (TD-VSDD-060-adjacent — a sibling-site
  sweep gap caused by two agents amending related artifacts in the same burst window)
- **Location:** S-21.25 v1.3, 5 live sites: the VP-status note (SITE_7 acknowledgment paragraph,
  two mentions), Context/Finding-Summary narrative (§Architecture Anchors sentence), Architecture
  Mapping table (emitter row), Task 5 (function-name declaration bullet), and Architecture
  Compliance Rules table (naming-rule row) — every live cite of VP-079's version read `v1.20`.
- **Description:** F-S2125-P3-002's remediation was split across two concurrent agents in the same
  D-1061 burst: story-writer added S-21.25's acknowledgment paragraph citing `VP-079 v1.20` (the
  version live at the moment story-writer wrote it), while architect concurrently bumped VP-079
  itself v1.20→v1.21 (retargeting the SITE_7 scope note's ownership language). Neither agent's
  output was wrong in isolation, but the combination left S-21.25 v1.3 citing and quoting a version
  of VP-079 that no longer existed as "current" by the time the burst closed — a classic
  concurrency-residue pattern (two same-burst edits to related artifacts, each individually correct
  at the instant it was written, jointly stale once both land). The quotation problem compounds the
  version-cite problem: the note didn't just cite the wrong version number, it presented v1.20's
  superseded sentence as VP-079's present-tense text.
- **Evidence:** `grep -n "VP-079 v1\.[0-9]"` against S-21.25 v1.2 (pre-fix) showed 5 live-site
  matches, all `v1.20`; cross-checked against the architect's own VP-079 last_amended entry for
  v1.21 (F-S2125-P3-002, same D-1061 burst) confirming VP-079's SITE_7 scope note text had changed
  out from under the quotation.
- **Proposed Fix:** Sweep `VP-079 v1.20` → `VP-079 v1.21` at all 5 live sites; replace the
  superseded-text quotation with either a verbatim v1.21 quotation or a paraphrase, explicitly
  framing the prior v1.20 text as historical carry-forward rather than presenting it as VP-079's
  current wording.
- **Status:** RESOLVED this burst (D-1062) — story-writer swept all 5 live `VP-079 v1.20` cites to
  `VP-079 v1.21` in S-21.25 v1.4, and reframed the VP-status note's quotation as a paraphrase of
  VP-079 v1.21's actual current SITE_7 text (Phase-6 formal-verification tracking, explicitly NOT
  S-21.25's pre-TDD/unit scope), with v1.20's superseded text now explicitly named as historical
  carry-forward context rather than quoted as present-tense. Re-run of the same detector against
  v1.4 confirms residual `v1.20` cites are only the exempt historical Changelog/`modified:` rows for
  v1.2/v1.3 (which correctly narrate what v1.20 said at the time) — 0 live-site residuals. Prior
  AC-005 SINGLE-EMIT-SITE guard and Task 7/11 test-distribution fixes CONFIRMED HELD, no
  recurrence. `input-hash` NOT touched (body-only edit; VP-079 is not in S-21.25's own `inputs:`
  dependency list, confirmed unchanged at `4af3ec2`).

## Disposition

F-S2125-P4-001 is story content (a version-cite and quotation-accuracy correction, no BC/AC/Task-
body-content, code-shape, or field/message-text change) — routed to story-writer per the Agent
Routing Table. Story-writer resolved same-burst in S-21.25 v1.3→v1.4 (D-1062). No architect
dispatch was required this pass — VP-079 itself is not stale; it is S-21.25's cite of VP-079 that
needed to catch up.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 1     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, v1.3 bundle) — RESOLVED same burst via story-writer
S-21.25 v1.4 remediation (D-1062).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving the MEDIUM finding does not
itself advance the streak per BC-5.39.001; pass 5 required against the v1.4 bundle to confirm
CLEAN).
**Readiness:** requires re-review (pass 5, against S-21.25 v1.4 + BC-1.03.019 v1.2 + VP-079 v1.21)
before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 1 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (1/1) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 4 → 3 → 2 → 1 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1062); pass 5 required to confirm CLEAN against the remediated v1.4 bundle. Monotonic decrease in per-pass finding count continues (4→3→2→1); this pass's finding class (concurrency residue between two same-burst agent edits) is distinct from every prior pass's class, consistent with continued novel-defect discovery rather than recurrence. |
