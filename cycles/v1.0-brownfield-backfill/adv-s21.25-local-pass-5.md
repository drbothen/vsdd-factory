---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-21T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-4.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: "11f321d"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 5
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-4.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 5) — NOT-CLEAN

## Finding ID Convention

Finding IDs in this cascade use the format `F-S2125-P<PASS>-<SEQ>` (three-digit sequence within the
pass), matching the convention established at pass 1 and used consistently through pass 4 for the
S-21.25 LOCAL cascade (e.g., `F-S2125-P4-001`).

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.4 (input-hash `4af3ec2`,
UNCHANGED since pass 3); `BC-1.03.019.md` v1.2; `BC-3.08.001.md` v1.26; `VP-079.md` v1.21;
`STORY-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`. Rubric: full `.factory/policies.yaml`
(POLICY 1-22). This is pass 5 of the S-21.25 LOCAL pre-TDD cascade — a full re-derivation of all 7
previously-named risk areas plus a fresh cross-index parity audit, fresh-context, no visibility
into prior review passes per the Iron Law.

## Verdict: NOT-CLEAN

2 MEDIUM findings, both located in index files (VP-INDEX.md, BC-INDEX.md) — index-propagation
residue from the Event-7 registration cluster (concurrency-residue, split across agents at prior
bursts), NOT a defect in the S-21.25 story body itself. LOCAL streak REMAINS 0/3 (a NOT-CLEAN pass
does not advance the streak per BC-5.39.001, and resolving the findings does not retroactively
advance it either).

## Part A — Fix Verification (pass 4 findings)

**F-S2125-P4-001** (concurrency-residue stale `VP-079 v1.20` cite/quotation at 5 live sites):
**VERIFIED FIXED, no recurrence.** S-21.25 v1.4 cites `VP-079 v1.21` at all 5 previously-affected
sites (VP-status note, Context/Finding-Summary, Architecture Mapping table, Task 5, Architecture
Compliance Rules table); the SITE_7 quotation is now a paraphrase of VP-079 v1.21's actual current
text, with v1.20's superseded text correctly framed as historical carry-forward, not present-tense.

Both prior structural-fix sets — AC-005 SINGLE-EMIT-SITE self-match/RED-GREEN-inversion guard
(D-1060, pass 2) and Task 7/11 test-distribution 14/3/1=18 correction (D-1061, pass 3) — CONFIRMED
HELD at this pass, no recurrence.

## Independent Re-Derivation of All 7 Named Risk Areas (S-21.25 story body)

1. **Emitter-name parity:** `emit_plugin_fuel_headroom_warning` consistent across story body,
   BC-1.03.019 v1.2, and BC-3.08.001 v1.26 — CLEAN.
2. **AC-005 self-match/RED-GREEN:** guard lives in a separate `tests/` file using a `concat!`-built
   needle, breaking the self-scanning-its-own-marker-string trap — CLEAN, no recurrence.
3. **Test-distribution 14/3/1=18:** Task 7/11 narrative correctly enumerates 14 (invoke.rs) + 3
   (emit_event.rs) + 1 (separate integration file) = 18 — CLEAN.
4. **Threshold-predicate testability:** `fuel_headroom_exceeded`/`fuel_headroom_ratio` remain
   extracted as named pure functions, independently unit-testable — CLEAN.
5. **BC-3.08.001 Event-7 field-set:** story's field enumeration matches BC-3.08.001 v1.26 §Mandatory
   Fields for Event 7 (`+message`/`timestamp`) — CLEAN.
6. **Message-text parity:** strict `>90%` wording matches ADR-039 v1.15 §Erratum E-006 and
   BC-1.03.019 v1.2 PC8 — CLEAN.
7. **Three-way input-hash parity:** story frontmatter `input-hash: "4af3ec2"` = STORY-INDEX catalog
   row `S-21.25=4af3ec2` = STORY-INDEX D-1057 blockquote enumeration `S-21.25=4af3ec2` — all three
   agree, POLICY 18 satisfied.

**The S-21.25 story body itself is CLEAN across all 7 risk areas.** Both findings below are located
entirely in sibling index files, discovered by tracing this story's own Event-7 anchor status
outward into VP-INDEX and BC-INDEX rather than by any defect in the story text.

## Part B — New Findings

### MEDIUM

#### F-S2125-P5-001: VP-INDEX §Story Anchors VP-079 row stale six-event enumeration + load-bearing version pin
- **Severity:** MEDIUM
- **Category:** POLICY 4 DESCRIPTION-BEARING ANCHOR-PROSE PARITY / POLICY 9
- **Location:** `.factory/specs/verification-properties/VP-INDEX.md` (~line 527), the VP-079
  §Story Anchors row (S-15.01 anchor row)
- **Description:** The row read "…all **six** async-semantics event types (…) per **BC-3.08.001
  v1.19**." VP-079 is now v1.21 and its own §Full Index row already correctly enumerates SEVEN
  event types (adds `plugin.fuel_headroom_warning` at SITE_7, swept at v1.20/D-1059); BC-3.08.001 is
  now v1.26. The row's trailing `v1.19` cite was additionally a load-bearing version pin that goes
  stale on every subsequent BC-3.08.001 amendment (POLICY 19 spirit).
- **Evidence:** VP-INDEX §Full Index row (line 458) already reads "seven" and lists
  `plugin.fuel_headroom_warning`; the §Story Anchors row at line 527 was the sole lagging site.
- **Proposed Fix:** Update to "all seven … event types" including `plugin.fuel_headroom_warning`;
  replace the trailing version pin with the stable unpinned form `per BC-3.08.001`.
- **Status:** RESOLVED this burst (D-1063) — state-manager swept the row: six→seven,
  `plugin.fuel_headroom_warning` added, version pin removed in favor of the stable form. VP-INDEX
  v2.78→v2.79.

#### F-S2125-P5-002: BC-INDEX BC-3.08.001 Stories column missing S-21.25
- **Severity:** MEDIUM
- **Category:** BC-INDEX-mirrors-BC-file / anchor-back propagation
- **Location:** `.factory/specs/behavioral-contracts/BC-INDEX.md` (~line 769), BC-3.08.001 Stories
  column
- **Description:** The Stories column read "S-15.01, S-19.05" but BC-3.08.001's own §Traceability
  Stories and §Story Anchor sections list S-21.25 as the Event-7 anchor story. Precedent: S-19.05
  (Event 6) IS in the column, establishing that event-adding stories belong there.
- **Evidence:** BC-3.08.001.md §Traceability (~line 394) and §Story Anchor (~line 335) both name
  S-21.25 as the Event-7 anchor; BC-INDEX's own Stories column had not been updated to match.
- **Proposed Fix:** Append ", S-21.25" to the BC-3.08.001 Stories column.
- **Status:** RESOLVED this burst (D-1063) — state-manager appended `, S-21.25`. BC-INDEX
  v4.86→v4.87. `total_bcs` UNCHANGED 1987 (column-cell fix only, no new BC).

## Process-Gap Recurrence (recorded, not a new story)

Both findings are a recurrence of the existing anchored class D-1044(g)/D-995 (governing-BC-bump
lacks same-burst story-propagation-dispatch discipline) — the v1.25 Event-7 registration burst
(D-1059) self-deferred the BC-INDEX Stories-column leg and the VP-INDEX §Story Anchors leg of its
own propagation. Per orchestrator direction, no new follow-up story is opened; this is recorded as
a recurrence anchored to the existing S-15.03 PRIORITY-A candidate (a POLICY-14-leg-5 same-burst
index-Stories-column sweep gate). See `lessons.md` recurrence note.

## Non-Resetting Observation

**O-S2125-P5-001 (non-finding, informational):** VP-079's own frontmatter carries `modified: []`
and no `last_amended` field despite 21 body `## Amendment` sections spanning VP-079's whole history
— a POLICY 17 gap. Architect-owned, out of S-21.25's own scope. Recorded as a drift item anchored
to the architect's next VP-079 touch, alongside the existing D-1062 VP-079 BC-3.08.001 v1.25→v1.26
stale-cite drift item (a distinct, still-open item — VP-079's own Property-Statement opening
parenthetical and Property-6 SITE_7 site-description sentence still cite `BC-3.08.001 v1.25`; this
finding was NOT re-fixed this burst, it remains a separate open drift item).

## Disposition

Both MEDIUM findings are index-domain (VP-INDEX, BC-INDEX), not S-21.25 story-body defects — routed
directly to state-manager, no story-writer dispatch needed. Resolved same burst (D-1063). S-21.25
story file itself (v1.4) is UNCHANGED this round, CONFIRMED CLEAN across all 7 named risk areas.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 2     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, VP-INDEX v2.78 + BC-INDEX v4.86) — RESOLVED same
burst via state-manager VP-INDEX + BC-INDEX remediation (D-1063). S-21.25 story file itself (v1.4)
UNCHANGED and CONFIRMED CLEAN across all 7 named risk areas throughout this pass.
**Convergence:** findings remain — iterate. LOCAL streak REMAINS 0/3 (resolving index-propagation
residue does not itself advance the streak per BC-5.39.001; a fresh CLEAN pass 6 is required).
**Readiness:** requires re-review (pass 6, against S-21.25 v1.4 [UNCHANGED] + VP-INDEX v2.79 +
BC-INDEX v4.87 bundle) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 2 (both index-propagation residue, both MEDIUM) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (2/2) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 3 → 2 → 1 → 2 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1063); pass 6 required to confirm CLEAN against the remediated VP-INDEX/BC-INDEX. Note: both findings located entirely in index artifacts, not the story file — S-21.25 v1.4's own substantive content is CONFIRMED CLEAN across all 7 named risk areas. |
