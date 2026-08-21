---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-1.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
input-hash: "d631f5c"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 2
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-1.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 2) — NOT-CLEAN

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.1 (input-hash `558a5a3`);
`BC-1.03.019.md` v1.1; `BC-3.08.001.md` v1.25. Rubric: full `.factory/policies.yaml` (POLICY 1-22).
This is pass 2 of the S-21.25 LOCAL pre-TDD cascade, reviewing the D-1059 pass-1 remediation
(helper extraction, AC-005 regression-guard redesign, field-set/message corrections).

## Verdict: NOT-CLEAN

1 HIGH + 2 MEDIUM findings. LOCAL streak 0/3 (a NOT-CLEAN pass resets the streak per BC-5.39.001).

## Part A — Fix Verification (pass 1 findings)

F-S2125-P1-001/002 (AC-005 regression guard / helper extraction): **VERIFIED FIXED as designed** —
named pure helpers `fuel_headroom_exceeded`/`fuel_headroom_ratio` + `check_and_emit_fuel_headroom_warning`
shell exist, and the SINGLE-EMIT-SITE marker-scan regression guard replaced the impossible
literal-count scan. However, reviewing the guard's actual placement (co-located in the same source
file as the code it scans) surfaces a new, distinct defect — see F-S2125-P2-001; this is a fresh
finding about the fix's own placement, not a reopening of P1-001/002. F-S2125-P1-003/004/005/006/007
(field set, ADR cite, changelog reconciliation, count phrases): VERIFIED FIXED, no residual issues
found.

## Part B — New Findings

### HIGH

#### F-S2125-P2-001: AC-005 regression guard recurs the self-match trap it was designed to fix, with a RED/GREEN inversion
- **Severity:** HIGH
- **Category:** test-design defect (recurrence of a class already found once in pass 1)
- **Location:** S-21.25 v1.1 AC-005 test (SINGLE-EMIT-SITE marker scan)
- **Description:** The pass-1 fix replaced an impossible literal-count scan with a marker-string
  scan for the SINGLE-EMIT-SITE guard, but placed the scanning test in the SAME source file as the
  call site it is scanning for. Because the test file itself contains a textual reference to the
  marker string (in its own assertion/comment describing what it is checking for), the scan
  matches the test's own source in addition to (or instead of) the production call site — the
  identical self-match failure class that F-S2125-P1-001 was written to close, recurred via a
  different mechanism (co-location, not literal-count semantics). Additionally, because the guard
  currently always finds a match (either the intended one or its own self-reference), the test
  exhibits a RED/GREEN inversion: it passes even in a mutated build where the real call site is
  removed, meaning the guard currently gives no regression protection at all — worse than a merely
  redundant test, it is a false-negative regression guard.
- **Evidence:** Grep for the marker string inside the AC-005 test file returns >1 match: the
  production call site AND the test's own source (assertion message / doc-comment referencing the
  same literal marker string it scans for), confirming self-match.
- **Proposed Fix:** Relocate the AC-005 regression guard to a separate file under `tests/` (outside
  the source tree it scans), and build the marker needle via `concat!` at test-compile time (e.g.
  `concat!("check_and_emit_", "fuel_headroom_warning")`) rather than a literal string, so the
  needle itself never appears as literal source text for the scan to accidentally match against
  its own file.
- **Status:** RESOLVED this burst (D-1060) — story-writer relocated AC-005 to a separate `tests/`
  file with a `concat!`-built needle; RED/GREEN correctness re-verified (mutation-removal of the
  call site now correctly fails the test). See Disposition.

### MEDIUM

#### F-S2125-P2-002: Emitter function name diverges from sibling naming convention and BC-3.08.001's event-catalog prose
- **Severity:** MEDIUM
- **Category:** naming consistency / sibling-site sweep (TD-VSDD-060 class)
- **Location:** `check_and_emit_fuel_headroom_warning`'s emitter helper, currently named
  `emit_fuel_headroom_warning`
- **Description:** The emitter function's name (`emit_fuel_headroom_warning`) omits the `plugin_`
  qualifier that BC-3.08.001's Event 7 catalog entry (`plugin.fuel_headroom_warning`, registered
  v1.25) and the wire event name itself both carry, creating an avoidable name/wire-event mismatch
  and inconsistency with sibling emitters in the same module family (which are named after their
  full dotted event name with `.`→`_` substitution, e.g. `plugin.abandoned` →
  `emit_plugin_abandoned`).
- **Evidence:** BC-3.08.001 v1.25 Event 7 wire name is `plugin.fuel_headroom_warning`; sibling
  emitters in the same crate follow the `emit_plugin_<event_suffix>` naming pattern; the current
  `emit_fuel_headroom_warning` name breaks that pattern by dropping the `plugin_` segment.
- **Proposed Fix:** Rename to `emit_plugin_fuel_headroom_warning`, matching sibling convention and
  the BC-3.08.001 wire event name exactly (case-adjusted). Sweep both call sites (S-21.25's story
  body + BC-3.08.001's Amendment section) for the old name.
- **Status:** RESOLVED this burst (D-1060) — renamed to `emit_plugin_fuel_headroom_warning`;
  BC-1.03.019 v1.2 and BC-3.08.001 v1.26 both swept. See Disposition.

#### F-S2125-P2-003: BC-3.08.001 carries a stale VP-079-staleness flag that was already closed
- **Severity:** MEDIUM
- **Category:** stale cross-reference / un-cleared TODO-class flag
- **Location:** BC-3.08.001 v1.25 §VP Anchors bullet, Amendment "changes made" item 11, and a
  standalone Amendment paragraph — all three flag VP-079 as needing an architect follow-up for
  Event 7 registration
- **Description:** VP-079 was already amended to v1.20 to register Event 7
  (`plugin.fuel_headroom_warning`) in a prior burst — the architect follow-up the flag calls for
  was already done. The flag itself was simply never cleared at any of its three sites, leaving a
  false "still pending" signal in BC-3.08.001 that could mislead a future reader into re-requesting
  work that is already complete.
- **Evidence:** VP-079's own frontmatter shows version v1.20 with an Event 7 registration entry in
  its changelog, dated prior to this review; all three BC-3.08.001 v1.25 sites still describe the
  VP-079 update as outstanding.
- **Proposed Fix:** Close the flag at all 3 sites (§VP Anchors bullet, Amendment changes-made item
  11, standalone Amendment paragraph) — state VP-079 v1.20 already registers Event 7, flag
  resolved, no further architect action owed.
- **Status:** RESOLVED this burst (D-1060) — all 3 sites closed in BC-3.08.001 v1.26. See
  Disposition.

## Disposition

F-S2125-P2-001 routed to story-writer (test-design/placement defect, story-writer's domain):
relocated AC-005 to a dedicated `tests/` file with a `concat!`-built needle, closing both the
self-match recurrence and the RED/GREEN inversion (S-21.25 v1.1→v1.2). F-S2125-P2-002 routed to
story-writer + product-owner jointly (naming touches both the story body and the two BCs that cite
the emitter): renamed to `emit_plugin_fuel_headroom_warning` and swept both BC-1.03.019 (v1.1→v1.2)
and BC-3.08.001 (v1.25→v1.26). F-S2125-P2-003 routed to product-owner (BC content, product-owner's
domain): closed the stale VP-079 flag at all 3 cited sites in BC-3.08.001 v1.26. All three findings
resolved same-burst (D-1060).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 1     |
| MEDIUM   | 2     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, v1.1 bundle) — RESOLVED same burst via story-writer
S-21.25 v1.2 + product-owner BC-1.03.019 v1.2 / BC-3.08.001 v1.26 remediation (D-1060).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving a HIGH + 2 MEDIUM does not
itself advance the streak per BC-5.39.001; pass 3 required against the v1.2 bundle to confirm
CLEAN).
**Readiness:** requires re-review (pass 3, against S-21.25 v1.2 + BC-1.03.019 v1.2 + BC-3.08.001
v1.26) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3/3) |
| **Median severity** | 3.0 (MEDIUM, HIGH+MEDIUM+MEDIUM) |
| **Trajectory** | 7 → 3 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1060); pass 3 required to confirm CLEAN against the remediated bundle. |
