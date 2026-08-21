---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-21T00:00:20Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-6.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: "4f8a9a3"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 7
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-6.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 7) — CLEAN

## Finding ID Convention

Finding IDs in this cascade use the format `F-S2125-P<PASS>-<SEQ>` (three-digit sequence within the
pass), matching the convention established at pass 1 and used consistently through pass 6 for the
S-21.25 LOCAL cascade.

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.5 (input-hash `eefe28b`,
UNCHANGED since D-1064); `BC-1.03.019.md` v1.3; `BC-3.08.001.md` v1.27; `VP-079.md` v1.21;
`STORY-INDEX.md` v4.379, `BC-INDEX.md` v4.88, `VP-INDEX.md` v2.79. Rubric: full
`.factory/policies.yaml` (POLICY 1-22), including a repeat corpus-wide grep sweep for POLICY 19
(`adr_version_cite_volatile_pin_prohibition`) compliance to confirm the pass-6 remediation held.
This is pass 7 of the S-21.25 LOCAL pre-TDD cascade — a full re-derivation of all 7 previously-named
risk areas plus the POLICY 19 sweep, fresh-context, no visibility into prior review passes per the
Iron Law.

## Verdict: CLEAN

Zero BLOCKER/HIGH/MEDIUM findings (streak-resetting classes). LOCAL BC-5.39.001 streak
**ADVANCES 0/3 → 1/3** — first clean pass since the pass-6 POLICY 19 HIGH. Four LOW cosmetic
observations recorded (non-resetting), deferred to a post-convergence cosmetic sweep — see below.

## Part A — Fix Verification (pass 6 findings)

**F-S2125-P6-001** (POLICY 19 load-bearing ADR-version pin in BC-1.03.019/BC-3.08.001 Traceability
rows): **VERIFIED FIXED, no recurrence.**

```
$ grep -n "ADR-039 v1\.[0-9]* §Decision 5" .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
(no output — zero remaining load-bearing version-pinned ADR-039 §Decision 5 cites in either live
Traceability row; both now read the stable `ADR-039 §Decision 5 Mitigation 1 (...)` form)
```

Both BC Traceability rows remain in the stable-anchor form established at D-1064; the corpus-wide
POLICY 19 sweep re-run this pass found no new outlier anywhere in the BC corpus, and confirmed
BC-1.03.019/BC-3.08.001 no longer participate in the pattern.

**F-S2125-P6-002** (BC-3.08.001 §VP-Anchors closure bullet stale bare version cite): **VERIFIED
FIXED, no recurrence.** The bullet retains the dated-historical annotation added at D-1064
(`VP-079 v1.20 at closure; now v1.21 — ... not re-verified against the current VP-079 version`).

**F-S2125-P6-003** (VP-079 internal six/seven event-type header-comment inconsistency): still
present, unchanged — this is a DEFERRED item, architect-owned, VP-079-internal, out of S-21.25's own
perimeter. Not re-counted as a new finding; carried forward as the same open drift item.

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
   §Mandatory Fields for Event 7 (`+message`/`timestamp`) — CLEAN.
6. **Message-text parity:** strict `>90%` wording matches ADR-039 v1.15 §Erratum E-006 and
   BC-1.03.019 v1.3 PC8 — CLEAN.
7. **Three-way input-hash parity (POLICY 18):** story frontmatter `input-hash: "eefe28b"` =
   STORY-INDEX catalog row `S-21.25=eefe28b` = STORY-INDEX D-1057 blockquote enumeration
   `S-21.25=eefe28b` — all three agree.

```
$ grep -n 'input-hash' .factory/stories/S-21.25-fuel-headroom-warn-event.md | head -1
17:input-hash: "eefe28b"
$ grep -o "S-21.25=eefe28b" .factory/stories/STORY-INDEX.md | sort -u
S-21.25=eefe28b
(frontmatter = catalog row = D-1057 blockquote enumeration — all three agree, POLICY 18 satisfied)
```

**The S-21.25 story body itself is CLEAN across all 7 risk areas, and the BC-Traceability-row HIGH
from pass 6 is confirmed remediated and held.**

## Part B — New Findings

None BLOCKER/HIGH/MEDIUM (streak-resetting). Four non-resetting LOW cosmetic observations recorded
below.

### LOW (non-resetting — deferred to post-convergence cosmetic sweep)

#### F-S2125-P7-001: Erratum-annotation phrasing differs between the two POLICY-19-compliant BC Traceability rows
- **Severity:** LOW (shared, cosmetic)
- **Location:** `BC-1.03.019.md` §Traceability ADR row vs `BC-3.08.001.md` §Traceability ADR row
- **Description:** Both rows are POLICY 19 compliant (stable `ADR-039 §Decision 5 Mitigation 1`
  section-anchor form, no version pin), but the erratum-annotation phrasing differs: BC-1.03.019
  reads "(WARN message per §Erratum E-006)" while BC-3.08.001 reads "(E-006)". Purely a wording
  difference; both correctly identify the same erratum.
- **Proposed Fix:** Align the two annotations to identical phrasing at a future touch.
- **Status:** DEFERRED — anchored to a post-convergence cosmetic sweep for S-21.25 (mirrors the
  S-21.11 D-1055/D-1056 accumulated-nit deferral pattern). Not fixed this pass.

#### F-S2125-P7-002: Context parenthetical conflates current version cite with BC's original authoring date
- **Severity:** LOW (S-21.25-specific, cosmetic)
- **Location:** `S-21.25-fuel-headroom-warn-event.md`, Context parenthetical (~line 100)
- **Description:** The parenthetical cites the current `BC-1.03.019 v1.3` alongside the BC's
  original 2026-08-20 authoring date, which could read as implying v1.3 was authored on that date
  (it was not — v1.3 is the D-1064 POLICY-19 fix; only v1.0 dates to 2026-08-20).
- **Proposed Fix:** Separate the version cite from the authoring-date reference, or annotate the
  date as "original authoring" explicitly.
- **Status:** DEFERRED — anchored to a post-convergence cosmetic sweep for S-21.25. Not fixed this
  pass.

#### F-S2125-P7-003: AC-header BC-version tokens asymmetric across the story's ten ACs
- **Severity:** LOW (pending-intent CONVENTION question)
- **Location:** `S-21.25-fuel-headroom-warn-event.md`, AC-001 through AC-010 headers
- **Description:** Only AC-006 and AC-008 headers carry an explicit ", v1.3" BC-version token;
  the other eight AC headers carry no version token at all. All present cites are current (nothing
  is stale), so this is not a drift defect — it is an open question of story-template convention:
  should every AC header carry an explicit governing-BC version token, or is the asymmetry
  (only version-sensitive ACs get one) intentional?
- **Proposed Fix:** N/A — requires an orchestrator/human convention ruling, not a mechanical fix.
- **Status:** DEFERRED — anchored to a post-convergence cosmetic sweep for S-21.25, or a human
  convention call, whichever comes first. Not fixed this pass.

#### F-S2125-P7-004: Story frontmatter `last_amended` bare-date form lacks a version-number prefix
- **Severity:** LOW (pending-intent CONVENTION, repo-wide)
- **Location:** `S-21.25-fuel-headroom-warn-event.md` frontmatter `last_amended` field
- **Description:** The field uses a bare-date form with no "(v1.5)" version prefix. This is NOT a
  burst-introduced defect — it is the story-template norm across the entire repository (every
  story file's frontmatter uses the same bare-date convention). Changing it for S-21.25 alone would
  create an inconsistency with every other story.
- **Proposed Fix:** A repo-wide frontmatter convention change (adding a version prefix to every
  story's `last_amended` field), not a per-story fix.
- **Status:** DEFERRED — anchored to a repo-wide convention decision (S-15.03 PRIORITY-A), NOT a
  per-story fix. Not fixed this pass.

## Non-Resetting Observations (carried forward)

**O-S2125-P6-001/002** (not-a-finding, carried forward): AC-trace version-token cosmetic
inconsistency; ADR ratification-version pins in historical Changelog/Amendment rows are immutable
provenance, not stale cites. Both remain non-actionable.

## Disposition

Zero streak-resetting (BLOCKER/HIGH/MEDIUM) findings. The pass-6 HIGH (F-S2125-P6-001) and its
sibling LOW (F-S2125-P6-002) are both confirmed remediated and held; F-S2125-P6-003 remains
correctly deferred to the architect. Four new LOW cosmetic observations (F-S2125-P7-001..004)
recorded and explicitly deferred to a post-convergence cosmetic sweep (mirroring the S-21.11
D-1055/D-1056 pattern) — none are streak-resetting, none touch the story's substantive content.
S-21.25 story body v1.5 CONFIRMED CLEAN across all 7 named risk areas plus POLICY 18 three-way
input-hash parity.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 0     |
| LOW      | 4 (non-resetting, all DEFERRED to post-convergence cosmetic sweep — F-S2125-P7-001..004) |

**Overall Assessment:** CLEAN — first clean pass since the pass-6 POLICY 19 HIGH reset.
**Convergence:** LOCAL streak **0/3 → 1/3**. Pass 8 next.
**Readiness:** continue cascade — pass 8 required against S-21.25 v1.5 (UNCHANGED) bundle,
fresh-context.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 7 |
| **New findings (streak-resetting)** | 0 |
| **New LOW cosmetic observations (non-resetting)** | 4 (F-S2125-P7-001..004, all deferred) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 0.0 for streak-resetting classes (0/0, CLEAN) |
| **Median severity** | n/a (CLEAN) |
| **Trajectory** | 1 → 2 → 1 → 0 |
| **Verdict** | CLEAN — streak 0/3 → 1/3. First clean pass since pass-6's POLICY 19 HIGH. Four non-resetting LOW cosmetic items deferred to post-convergence sweep. |
