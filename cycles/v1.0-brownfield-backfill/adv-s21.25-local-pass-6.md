---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-21T00:00:10Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-5.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: "d590a59"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 6
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-5.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 6) — NOT-CLEAN

## Finding ID Convention

Finding IDs in this cascade use the format `F-S2125-P<PASS>-<SEQ>` (three-digit sequence within the
pass), matching the convention established at pass 1 and used consistently through pass 5 for the
S-21.25 LOCAL cascade (e.g., `F-S2125-P5-001`).

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.4 (input-hash `4af3ec2`,
UNCHANGED since pass 3); `BC-1.03.019.md` v1.2; `BC-3.08.001.md` v1.26; `VP-079.md` v1.21;
`STORY-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`. Rubric: full `.factory/policies.yaml`
(POLICY 1-22), with a corpus-wide grep sweep for POLICY 19 (`adr_version_cite_volatile_pin_prohibition`)
compliance across every BC Traceability row citing an ADR. This is pass 6 of the S-21.25 LOCAL
pre-TDD cascade — a full re-derivation of all 7 previously-named risk areas plus the corpus-wide
POLICY 19 sweep, fresh-context, no visibility into prior review passes per the Iron Law.

## Verdict: NOT-CLEAN

1 HIGH finding (POLICY 19 violation, streak-resetting) + 2 LOW findings (1 remediated this burst,
1 deferred). The story BODY itself is independently CLEAN across all 7 named risk areas — the HIGH
finding is located in the governing BC's own Traceability row (BC-1.03.019), discovered by a
corpus-wide grep sweep rather than any defect in the story text. LOCAL streak REMAINS 0/3 (resolving
a HIGH does not advance the streak per BC-5.39.001).

## Part A — Fix Verification (pass 5 findings)

**F-S2125-P5-001** (VP-INDEX §Story Anchors VP-079 row stale six-event enumeration + load-bearing
version pin): **VERIFIED FIXED, no recurrence.** VP-INDEX §Story Anchors row now reads "seven" and
lists `plugin.fuel_headroom_warning`; the trailing version pin is now the stable unpinned form
`per BC-3.08.001`.

**F-S2125-P5-002** (BC-INDEX BC-3.08.001 Stories column missing S-21.25): **VERIFIED FIXED, no
recurrence.** BC-INDEX Stories column for BC-3.08.001 now reads "S-15.01, S-19.05, S-21.25".

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
   agree, POLICY 18 satisfied (prior to this burst's own reconciliation).

**The S-21.25 story body itself is CLEAN across all 7 risk areas.** The HIGH finding below is
located entirely in the governing BC's own Traceability row, discovered by a corpus-wide grep
sweep for POLICY 19 compliance rather than by any defect in the story text.

## Part B — New Findings

### HIGH

#### F-S2125-P6-001: Load-bearing ADR-version pin in BC-1.03.019 (and sibling BC-3.08.001) Traceability row — POLICY 19 violation
- **Severity:** HIGH (streak-resetting)
- **Category:** POLICY 19 `adr_version_cite_volatile_pin_prohibition`
- **Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md` §Traceability, ADR row;
  sibling `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` §Traceability, ADR row.
- **Description:** BC-1.03.019's Traceability ADR row read: "ADR-039 **v1.15** §Decision 5
  Mitigation 1 (...). Version cite updated v1.14→v1.15 (this BC's v1.1 fix burst) to track ADR-039
  E-006's `≥90%`→`>90%` WARN-message correction, reproduced verbatim in PC8." This is a load-bearing
  version pin on an ADR citation — the exact pattern POLICY 19 prohibits, because ADR-039 will
  continue to amend (it already has, through §AMD-003 and multiple Erratum entries) and every future
  amendment silently stales this row without any BC-1.03.019 edit ever occurring. A corpus-wide grep
  sweep of every BC Traceability row citing an ADR found this as the **sole outlier** in the entire
  BC corpus — every other BC Traceability ADR row uses the stable section-anchor form (`ADR-NNN
  §Decision N`, no version token). BC-3.08.001's sibling Traceability ADR row carried the same
  pattern for its Event-7 provenance clause ("Event 7 (...) is added by ADR-039 v1.15 §Decision 5
  Mitigation 1 (...)").
- **Evidence:**
  ```
  $ grep -rn "ADR-[0-9]\{3\} v[0-9]" .factory/specs/behavioral-contracts/ss-*/BC-*.md | grep -i traceability -A0 -B0
  # (corpus-wide sweep; BC-1.03.019.md and BC-3.08.001.md were the only two live Traceability-row
  # hits carrying a load-bearing ADR version token; every other BC Traceability ADR row across the
  # corpus uses the stable `ADR-NNN §Decision N` form with no version number)
  ```
- **Proposed Fix:** Replace the load-bearing `ADR-039 v1.15 §Decision 5 Mitigation 1` form with the
  stable `ADR-039 §Decision 5 Mitigation 1 (...)` form in both BCs' Traceability rows; relocate the
  v1.14→v1.15 version-delta provenance narrative out of the Traceability cell into the BC's own
  Changelog, where version-delta history belongs.
- **Status:** RESOLVED this burst (D-1064) — product-owner rewrote BC-1.03.019's Traceability row to
  `ADR-039 §Decision 5 Mitigation 1 (WARN message per §Erratum E-006)` (v1.2→v1.3), relocating the
  v1.14→v1.15 provenance to the Changelog; sibling BC-3.08.001 Traceability row swept to `ADR-039
  §Decision 5 Mitigation 1 (E-006)` (v1.26→v1.27) in the same burst. Story-writer swept all 13 live
  `BC-1.03.019 v1.2` citations in S-21.25 to `v1.3` (S-21.25 v1.4→v1.5); BC-3.08.001 citations in the
  story were all bare/unversioned, nothing to sweep. This resolves a HIGH but does not advance the
  streak per BC-5.39.001.

### LOW

#### F-S2125-P6-002: BC-3.08.001 §VP Anchors VP-079 closure bullet cites stale bare version
- **Severity:** LOW
- **Category:** POLICY 4 description-bearing anchor-prose parity
- **Location:** `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` §VP Anchors, VP-079
  closure bullet (~line 342)
- **Description:** The live closure bullet ("Staleness flag CLOSED...") cited `VP-079 v1.20`, which
  is now stale (VP-079 has advanced to v1.21) even though the bullet is a historical closure record
  documenting what was true at 2026-08-20 closure time, not a live re-verification claim.
- **Proposed Fix:** Annotate the bullet as a dated historical record rather than silently leaving the
  bare version cite to read as current, or silently updating it to "v1.21" (which would
  misrepresent what was actually verified at closure time).
- **Status:** RESOLVED this burst (D-1064) — product-owner appended `(VP-079 v1.20 at closure; now
  v1.21 — this bullet is a historical closure record and is not re-verified against the current VP-079
  version; see v1.27 Changelog, F-S2125-P6-002)` to the bullet. BC-3.08.001 v1.26→v1.27 (same burst
  as F-S2125-P6-001's sibling sweep).

#### F-S2125-P6-003: VP-079 internal six/seven event-type header-comment inconsistency
- **Severity:** LOW
- **Category:** internal consistency, VP-079-domain
- **Location:** `.factory/specs/verification-properties/VP-079.md`, Proof-Harness-Skeleton header
  comments (~lines 149, 482)
- **Description:** The header comments still say "six" event types though the v1.21 Property
  Statement says "seven" (added `plugin.fuel_headroom_warning` at SITE_7, D-1059/v1.20).
- **Proposed Fix:** Sweep the header comments to "seven" to match the Property Statement.
- **Status:** DEFERRED — architect-owned, VP-079-internal, out of S-21.25's own perimeter. New drift
  item anchored the architect's next VP-079 touch, alongside the existing D-1062 VP-079
  BC-3.08.001-cite drift item and the VP-079 frontmatter `modified: []`/missing `last_amended` gap
  (O-S2125-P5-001, pass 5).

## Process-Gap Recurrence (recorded, not a new story)

POLICY 19 (`adr_version_cite_volatile_pin_prohibition`) was never applied to BC-1.03.019's own
Traceability row across its v1.0→v1.2 authoring and 5 prior adversary passes — a governance-
discipline gap that only fresh-context corpus-grep surfaced at pass 6. Per orchestrator direction,
no new follow-up story is opened; this is recorded as a recurrence-class candidate anchored to
S-15.03 PRIORITY-A (a tree-wide POLICY-19 Traceability-row sweep gate at BC-authoring time). See
`lessons.md` recurrence note.

## Non-Resetting Observations

**O-S2125-P6-001 (not-a-finding, carried forward):** AC-trace version-token cosmetic
inconsistency — not a substantive drift, all cite correct current content post-remediation. Not
actioned.

**O-S2125-P6-002 (not-a-finding, carried forward):** ADR ratification-version pins that appear in
historical Changelog rows or Amendment sections are immutable historical provenance, not stale
cites — distinct from the live-Traceability-row load-bearing pin this pass's HIGH finding
addresses. Documented as non-actionable.

## Disposition

The HIGH finding (F-S2125-P6-001) is a BC-Traceability-row defect, not an S-21.25 story-body
defect — routed to product-owner (both BC fixes) with a story-writer downstream cite-propagation
leg. Resolved same burst (D-1064). One LOW (F-S2125-P6-002) also resolved same burst by
product-owner; one LOW (F-S2125-P6-003) deferred to architect's next VP-079 touch. S-21.25 story
file itself is UNCHANGED in body content this round (only cite-propagation edits, v1.4→v1.5),
CONFIRMED CLEAN across all 7 named risk areas.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 1     |
| MEDIUM   | 0     |
| LOW      | 2 (1 remediated, 1 deferred) |

**Overall Assessment:** block (pre-remediation, BC-1.03.019 v1.2 + BC-3.08.001 v1.26) — RESOLVED
same burst via product-owner BC-1.03.019/BC-3.08.001 remediation + story-writer cite propagation
(D-1064). S-21.25 story body itself CONFIRMED CLEAN across all 7 named risk areas throughout this
pass.
**Convergence:** findings remain — iterate. LOCAL streak REMAINS 0/3 (resolving a HIGH does not
advance the streak per BC-5.39.001; a fresh CLEAN pass 7 is required).
**Readiness:** requires re-review (pass 7, against S-21.25 v1.5 + BC-1.03.019 v1.3 + BC-3.08.001
v1.27 bundle) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **New findings** | 3 (1 HIGH BC-Traceability-domain, 2 LOW — 1 BC-domain remediated, 1 VP-079-domain deferred) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 0.5 (3/6, moderate — first POLICY 19 finding in this cascade) |
| **Median severity** | 3.0 (HIGH) |
| **Trajectory** | 2 → 1 → 2 → 1 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1064); pass 7 required to confirm CLEAN against the remediated BC-1.03.019/BC-3.08.001. Note: HIGH finding located entirely in the governing BC's own Traceability row, not the story file — S-21.25 v1.5's own substantive content is CONFIRMED CLEAN across all 7 named risk areas. |
