---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-21T00:00:40Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-7.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: "dd6ee20"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 8
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-7.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 8) — CLEAN

## Finding ID Convention

Finding IDs in this cascade use the format `F-S2125-P<PASS>-<SEQ>`, matching the convention
established at pass 1 and used consistently through pass 7 for the S-21.25 LOCAL cascade.

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.5 (input-hash `eefe28b`,
UNCHANGED since D-1064); `BC-1.03.019.md` v1.3; `BC-3.08.001.md` v1.27; `VP-079.md` v1.21;
`STORY-INDEX.md` v4.380, `BC-INDEX.md` v4.88, `VP-INDEX.md` v2.79. Rubric: full
`.factory/policies.yaml` (POLICY 1-22), including a repeat corpus-wide grep sweep for POLICY 19
(`adr_version_cite_volatile_pin_prohibition`) compliance. This is pass 8 of the S-21.25 LOCAL
pre-TDD cascade — a full re-derivation of all 7 previously-named risk areas, fresh-context, no
visibility into prior review passes per the Iron Law.

## Verdict: CLEAN

Zero BLOCKER/HIGH/MEDIUM findings (streak-resetting classes). LOCAL BC-5.39.001 streak
**ADVANCES 1/3 → 2/3** — second consecutive clean pass. One LOW finding (part of the ongoing
F-S2125-P7-001-class cluster) and two ADVISORY observations recorded (all non-resetting),
deferred to a post-convergence cosmetic sweep — see below.

## Part A — Fix Verification (pass 7 findings)

**F-S2125-P7-001** (erratum-annotation phrasing wording difference between the two
POLICY-19-compliant BC Traceability rows): unchanged, still open, correctly DEFERRED —
non-resetting, not re-fixed this pass.

**F-S2125-P7-002** (Context parenthetical conflates current version cite with BC's original
authoring date): unchanged, still open, correctly DEFERRED — non-resetting.

**F-S2125-P7-003** (AC-header BC-version-token asymmetry, pending-intent CONVENTION question):
unchanged, still open, correctly DEFERRED to orchestrator/human convention adjudication.

**F-S2125-P7-004** (story frontmatter `last_amended` bare-date form, repo-wide convention
question): unchanged, still open, correctly DEFERRED to S-15.03 PRIORITY-A.

None of the four pass-7 LOW cosmetic observations were fixed this pass (by design — all four are
explicitly anchored to a future post-convergence cosmetic sweep or a human convention call, not to
individual adversary passes). All four confirmed still accurately described; no drift, no
recurrence, no new severity.

## Independent Re-Derivation of All 7 Named Risk Areas (S-21.25 story body)

1. **Emitter-name parity:** `emit_plugin_fuel_headroom_warning` consistent across story body,
   BC-1.03.019 v1.3, and BC-3.08.001 v1.27 — CLEAN.
2. **AC-005 self-match/RED-GREEN:** guard remains in its own `tests/` file using a `concat!`-built
   needle — CLEAN, no recurrence.
3. **Test-distribution 14/3/1=18:** Task 7/11 narrative correctly enumerates 14 (invoke.rs) + 3
   (emit_event.rs) + 1 (separate integration file) = 18 — CLEAN.
4. **Threshold-predicate testability:** `fuel_headroom_exceeded`/`fuel_headroom_ratio` remain
   extracted as named pure functions, independently unit-testable — CLEAN.
5. **BC-3.08.001 Event-7 field-set:** story's field enumeration matches BC-3.08.001 v1.27
   §Mandatory Fields for Event 7 — CLEAN.
6. **Message-text parity:** strict `>90%` wording matches ADR-039 v1.15 §Erratum E-006 and
   BC-1.03.019 v1.3 PC8 — CLEAN.
7. **Three-way input-hash parity (POLICY 18):** story frontmatter `input-hash: "eefe28b"` =
   STORY-INDEX catalog row `S-21.25=eefe28b` = STORY-INDEX D-1057 blockquote enumeration
   `S-21.25=eefe28b` — all three agree.

```
$ grep -n "ADR-039 v1\.[0-9]* §Decision 5" .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
(no output — zero remaining load-bearing version-pinned ADR-039 §Decision 5 cites in either BC's
own Traceability row; D-1064 fix confirmed held for the second consecutive pass)
```

## Part B — New Findings

None BLOCKER/HIGH/MEDIUM (streak-resetting). One new LOW finding and two ADVISORY observations
recorded below, all non-resetting.

### LOW (non-resetting — extends the F-S2125-P7-001-class drift item)

#### F-S2125-P8-001: Story body itself still carries load-bearing `ADR-039 v1.15 §Decision 5` version-pin cites at multiple live sites, now destabilized relative to the BC layer's own D-1064 fix
- **Severity:** LOW (extends the D-1065 drift item; part of the same cluster as F-S2125-P7-001)
- **Location:** `S-21.25-fuel-headroom-warn-event.md`: Task 1 (~L460), AC-008 (~L305/L309),
  Architecture Compliance Rules table (~L575), Token Budget section (~L449), opening narrative
  (~L100).
- **Description:** At D-1064, `BC-1.03.019` and `BC-3.08.001` swept their own Traceability rows
  away from the load-bearing `ADR-039 v1.15 §Decision 5 Mitigation 1` version-pin form to the
  stable, version-invariant `ADR-039 §Decision 5 Mitigation 1 (E-006)` form (POLICY 19 fix). The
  story body itself was NOT swept at that time (D-1064's scope was the two BC Traceability rows
  only) — the five story-body sites above still cite the versioned `ADR-039 v1.15 §Decision 5`
  form, which is now one version-generation destabilized relative to the BCs it traces to. POLICY
  19's own scope (`adr_version_cite_volatile_pin_prohibition`) textually applies to
  BC-Traceability-row cites; whether it also reaches story-body narrative cites is a
  pending-intent CONVENTION question, not a mechanical gate violation — the gate that exists today
  checks a version-invariant message string, not story-body provenance narrative. Not a
  correctness defect (the message text and semantics are unaffected); a provenance-hygiene
  drift item.
- **Proposed Fix:** Either (a) sweep all five story-body sites to the stable
  `ADR-039 §Decision 5 Mitigation 1 (E-006)` form to match the BC layer, or (b) rule that
  story-body narrative cites are out of POLICY 19's scope and leave as historical-provenance
  narrative. Requires an orchestrator/human convention ruling on POLICY 19's exact reach before a
  mechanical fix can be applied.
- **Status:** DEFERRED — anchored to a post-convergence cosmetic sweep for S-21.25, extending the
  D-1065 drift item; pending-intent CONVENTION question. Not fixed this pass.

### ADVISORY (non-resetting)

#### F-S2125-P8-002: Story frontmatter `last_amended` bare-date form (repeat observation, repo-wide norm)
- **Severity:** ADVISORY
- **Location:** `S-21.25-fuel-headroom-warn-event.md` frontmatter `last_amended` field.
- **Description:** Same observation as F-S2125-P7-004 (bare-date form with no "(v1.5)" version
  prefix) — confirmed still the repo-wide story-template norm, not a per-story defect. Recorded
  again at this pass per the adversary's full-rubric re-derivation; not a new distinct issue.
- **Proposed Fix:** Repo-wide frontmatter convention change (S-15.03 PRIORITY-A), not a per-story
  fix.
- **Status:** DEFERRED — anchored to a repo-wide convention decision (S-15.03 PRIORITY-A).

#### F-S2125-P8-003: AC-header / reference-row citation-convention asymmetry (repeat observation)
- **Severity:** ADVISORY
- **Location:** `S-21.25-fuel-headroom-warn-event.md`, AC-header rows and the story's own
  §References table.
- **Description:** Extends F-S2125-P7-003 — beyond the AC-header BC-version-token asymmetry
  already recorded, the story's §References row for `BC-1.03.019` also does not carry an explicit
  version token, while some sibling stories' reference rows do. A convention-consistency
  observation, not a staleness defect (the cite is current).
- **Proposed Fix:** N/A — requires an orchestrator/human convention ruling alongside F-S2125-P7-003.
- **Status:** DEFERRED — anchored to a post-convergence cosmetic sweep for S-21.25, or a human
  convention call, whichever comes first.

## Non-Resetting Observations (carried forward)

**O-S2125-P6-001/002** (not-a-finding, carried forward): AC-trace version-token cosmetic
inconsistency; ADR ratification-version pins in historical Changelog/Amendment rows are immutable
provenance, not stale cites. Both remain non-actionable.

## Disposition

Zero streak-resetting (BLOCKER/HIGH/MEDIUM) findings. All four pass-7 LOW cosmetic observations
confirmed still open and correctly deferred, no recurrence, no severity change. One new LOW finding
(F-S2125-P8-001) extends the same drift-item cluster with story-body-level detail; two new
ADVISORY observations (F-S2125-P8-002/003) recorded, both repeat/extension observations of
already-deferred convention questions. S-21.25 story body v1.5 CONFIRMED CLEAN across all 7 named
risk areas plus POLICY 18 three-way input-hash parity.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 0     |
| LOW      | 1 (non-resetting, DEFERRED — F-S2125-P8-001, extends D-1065 drift item) |
| ADVISORY | 2 (non-resetting, DEFERRED — F-S2125-P8-002/003) |

**Overall Assessment:** CLEAN — second consecutive clean pass.
**Convergence:** LOCAL streak **1/3 → 2/3**. Pass 9 next — one more CLEAN pass converges.
**Readiness:** continue cascade — pass 9 required against S-21.25 v1.5 (UNCHANGED) bundle,
fresh-context.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings (streak-resetting)** | 0 |
| **New LOW/ADVISORY observations (non-resetting)** | 3 (F-S2125-P8-001/002/003, all deferred) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 0.0 for streak-resetting classes (0/0, CLEAN) |
| **Median severity** | n/a (CLEAN) |
| **Trajectory** | 2 → 1 → 0 → 0 |
| **Verdict** | CLEAN — streak 1/3 → 2/3. One more CLEAN pass converges. |
