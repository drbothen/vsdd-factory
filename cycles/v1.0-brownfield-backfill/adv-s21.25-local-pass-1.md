---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "5733f7a"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 1
cascade: S-21.25-local
previous_review: null
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 1) — NOT-CLEAN

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.0 (input-hash `775050b`);
`BC-1.03.019.md` v1.0; `ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md` v1.14.
Rubric: full `.factory/policies.yaml` (POLICY 1-22). This is the first LOCAL pre-TDD adversarial
pass against S-21.25 as an independently-mergeable wave-6 seam (fuel-headroom WARN event,
ADR-039 §Decision 5 Mitigation 1).

## Verdict: NOT-CLEAN
2 HIGH (test-design defects) + 2 MEDIUM (event-catalog/hash) + 3 LOW findings. LOCAL streak 0/3
(this pass establishes the cascade).

## Finding ID Convention

Finding IDs for the S-21.25 LOCAL cascade use the format `F-S2125-P<PASS>-<SEQ>` (e.g.
`F-S2125-P1-001`), matching the sibling per-story LOCAL cascade convention already established
for S-21.09, S-21.11 v2, and S-21.19 (`F-S21NN-P<PASS>-<SEQ>` via `adv-s21NN-local-pass-N.md`).

## Part A — Fix Verification (pass >= 2 only)

N/A — this is pass 1 of the S-21.25 LOCAL cascade; there is no prior pass to verify fixes against.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S2125-P1-001: AC-005 regression guard specifies an impossible source-scan count
- **Severity:** HIGH
- **Category:** test-design defect (unsatisfiable acceptance criterion)
- **Location:** S-21.25 v1.0 AC-005 (regression guard for exactly-once-per-invocation emission)
- **Description:** AC-005 as authored specified a literal occurrence-count scan for the string
  `emit_fuel_headroom_warning(` across the centralized post-invocation check point. Because the
  single call site sits inside a shared helper function body (not duplicated per `Ok`-constructing
  match arm per PC5's own centralization requirement), a literal-count grep of the *call
  expression* text is not a stable proxy for "exactly one production call site" — refactors that
  are semantically single-emit (e.g. extracting the check into its own named function, which PC5
  itself anticipates) change the literal count without changing emission cardinality, making the
  guard both a false-positive trap on legitimate refactors and, more seriously, incapable of
  distinguishing "one call site" from "one call site invoked from N branches," which is exactly
  the multi-branch-uniformity property PC5 exists to guarantee.
- **Evidence:** No `fuel_headroom` occurrence exists anywhere in `crates/` at HEAD (verified via
  repo grep); the guard is therefore currently vacuously "satisfied" by absence, which is not
  evidence the guard will correctly discriminate the intended violation once the emitter exists.
- **Proposed Fix:** Redesign the guard around a named, greppable marker comment
  (`// SINGLE-EMIT-SITE`) placed at the one sanctioned call site, with the regression test scanning
  for exactly one occurrence of the marker rather than the call-expression text. Route to
  `vsdd-factory:story-writer` (test-design is story-writer's scope for AC construction).
- **Status:** RESOLVED this burst — see Disposition.

#### F-S2125-P1-002: Inline-vs-synthetic-vector untestability of the fuel-headroom predicate
- **Severity:** HIGH
- **Category:** test-design defect (untestable acceptance criterion)
- **Location:** S-21.25 v1.0 Task/AC set governing PC1/PC7 (threshold predicate + `headroom_ratio`
  formula)
- **Description:** The story's v1.0 task breakdown left the `fuel_consumed > 0.9 × fuel_cap`
  threshold check and the `headroom_ratio = 1 - fuel_consumed/fuel_cap` computation embedded
  inline inside the same post-invocation match arm that also performs the event emission side
  effect, with no extracted pure function boundary. This makes the arithmetic untestable in
  isolation (unit tests would require driving the full `invoke_plugin` effectful path just to
  exercise a boundary-value comparison), and it is the same class of defect Property 6's
  SITE-uniformity concern in VP-079 warns against: an inline check duplicated or drifted across
  branches cannot be verified by a single targeted test.
- **Evidence:** No pure-function extraction existed in the v1.0 task list; PC1/PC2/PC3's boundary
  controls (92.5%, exactly-90%, 50%) as drafted could only be exercised via full `PluginResult`
  fixture construction, not direct arithmetic assertion.
- **Proposed Fix:** Extract named pure helpers — `fuel_headroom_exceeded(fuel_consumed, fuel_cap)
  -> bool` and `fuel_headroom_ratio(fuel_consumed, fuel_cap) -> f64` — callable and unit-testable
  independent of the effectful `invoke_plugin` shell; the shell becomes a thin
  `check_and_emit_fuel_headroom_warning` orchestration function that calls the two pure helpers
  and performs the single emission. Route to `vsdd-factory:story-writer`.
- **Status:** RESOLVED this burst — see Disposition.

### MEDIUM

#### F-S2125-P1-003: BC-1.03.019 PC6 required-fields enumeration omits `timestamp` (missing/unregistered event)
- **Severity:** MEDIUM
- **Category:** contradictions / spec-fidelity (sibling-parity gap)
- **Location:** BC-1.03.019 v1.0 PC6 ("Required event fields")
- **Description:** PC6's enumeration listed `plugin_name`, `fuel_consumed`, `fuel_cap`,
  `headroom_ratio`, plus envelope fields, but omitted `timestamp` — a field every sibling
  `plugin.*` emitter in `emit_event.rs` already carries (`emit_plugin_timeout_async`,
  `emit_plugin_abandoned`, `emit_plugin_completed_async`). S-19.09 T-013/F-WG-003 was a dedicated
  prior fix that added `timestamp` to `emit_plugin_completed_async` precisely because it had been
  missing from that sibling emitter; this new event was at risk of reintroducing the identical
  gap. Additionally, the new event type was not yet registered in BC-3.08.001's SS-03 event
  catalog (the wire-format catalog authority), leaving `plugin.fuel_headroom_warning` an
  unregistered event type at the SS-03 catalog level even though BC-1.03.019 governed its
  triggering-condition/semantics.
- **Evidence:** `emit_event.rs` sibling emitters' mandatory-field lists (cross-checked against
  BC-3.08.001 v1.24's own mandatory-fields table) all carry `timestamp`; PC6 v1.0 did not.
- **Proposed Fix:** Add `timestamp: String` to PC6's field enumeration with a cross-reference note
  to the S-19.09 precedent; register Event 7 in BC-3.08.001's SS-03 catalog in the same burst.
  Route to `vsdd-factory:product-owner` (BC content is product-owner's scope).
- **Status:** RESOLVED this burst — see Disposition.

#### F-S2125-P1-004: Input-hash "PENDING" narrative vs. real computed frontmatter value
- **Severity:** MEDIUM
- **Category:** hash reconciliation / narrative-vs-frontmatter drift
- **Location:** BC-1.03.019 v1.0 Changelog row and S-21.25 v1.0 frontmatter
- **Description:** BC-1.03.019 v1.0's own Changelog row narrated `input-hash: "PENDING"` (citing
  product-owner's lack of `exec`/`process` tool access), but the frontmatter `input-hash` key
  already carried a real computed value (`57262cf`) at authoring time — the row's own narrative
  had gone stale relative to the frontmatter it was describing, understating the artifact's actual
  hash-reconciliation state. Separately, S-21.25's frontmatter `input-hash` (`775050b`, computed
  against BC-1.03.019 v1.0) goes stale the moment BC-1.03.019 is amended to v1.1 by this same
  burst's PC6/PC8 fixes, requiring reconciliation in the identical commit that lands the BC change
  (POLICY 18 per-file operator binary discipline; per D-952, never dev-source `--scan --update`).
- **Evidence:** `compute-input-hash --check` against BC-1.03.019 v1.0 frontmatter matched
  `57262cf` (no drift) at the time this finding was raised, contradicting the Changelog row's
  "PENDING" text.
- **Proposed Fix:** Correct the v1.0 Changelog row in place to state the actual `57262cf` value
  rather than "PENDING" (narrative-text-only correction; state-manager does not itself recompute
  or alter the frontmatter `input-hash` field via prose edits). Reconcile S-21.25's input-hash
  against BC-1.03.019 v1.1 via the per-file operator binary in the same burst that closes this
  finding set. Route to `vsdd-factory:state-manager` (hash reconciliation is state-manager's
  scope per POLICY 18).
- **Status:** RESOLVED this burst — see Disposition.

### LOW

#### F-S2125-P1-005: PC6 "exactly these fields" wording falsely excludes `message`
- **Severity:** LOW
- **Category:** internal contradiction (PC6 vs PC8)
- **Location:** BC-1.03.019 v1.0 PC6 opening clause
- **Description:** PC6's opening clause read "The emitted event MUST carry exactly these fields,"
  immediately followed by an enumeration that omitted `message` — a field PC8 separately and
  unambiguously mandates ("MUST additionally carry a `message` field equal, verbatim, to..."). The
  false-exclusivity framing of "exactly these fields" created an internal contradiction: read
  literally, PC6 forbids the `message` field PC8 requires.
- **Evidence:** PC6 v1.0 enumeration vs. PC8 v1.0 mandate, same document, contradictory field-set
  claims.
- **Proposed Fix:** Reword PC6 to drop the false-exclusivity framing and enumerate the complete,
  correct field set (folding in `level`/`message` from PC8 and `timestamp` from F-S2125-P1-003)
  so PC6 and PC8 no longer contradict each other. Route to `vsdd-factory:product-owner`.
- **Status:** RESOLVED this burst — see Disposition.

#### F-S2125-P1-006: ADR-039 §Decision 5 WARN message wording contradicts its own strict trigger predicate
- **Severity:** LOW
- **Category:** wording erratum (message string vs. trigger predicate)
- **Location:** ADR-039 v1.14 §Decision 5 Mitigation 1, WARN message string
- **Description:** §Decision 5 Mitigation 1's mandated verbatim WARN message read `"...plugin
  consumed ≥90% of budget..."`, but the mitigation's own trigger predicate, stated one sentence
  earlier in the same paragraph, is strict inequality (`fuel_consumed > 0.9 × cap`) — exactly 90%
  does NOT fire the warning (BC-1.03.019 PC2 boundary control). The `≥90%` wording in the message
  string therefore misdescribed the condition under which the dispatcher emits it. Because
  BC-1.03.019 PC8 and S-21.25 AC-008 both require byte-for-byte reproduction of this ADR string,
  the drift was upstream-load-bearing and would have propagated a wrong operator-facing message
  into production.
- **Evidence:** ADR-039 v1.14 §Decision 5 trigger-predicate sentence (`fuel_consumed > 0.9 × cap`)
  immediately followed by the `≥90%` message string in the same paragraph.
- **Proposed Fix:** Correct the message text to `"fuel-headroom-warning: plugin consumed >90% of
  budget; next larger input may trap — recalibrate fuel_cap"` (strict `>`, matching the trigger
  predicate verbatim); file as a non-re-ratifying erratum (same category as E-001..E-005) since no
  decision semantics or threshold value changes, only the message string's inequality symbol.
  Route to `vsdd-factory:architect` (ADR content is architect's scope); cascade to
  BC-1.03.019 PC8 and S-21.25 AC-008 in the same/follow-up burst.
- **Status:** RESOLVED this burst — see Disposition.

#### F-S2125-P1-007: capabilities.md CAP-011 body cites the pre-ADR-042 fuel-cap default
- **Severity:** LOW
- **Category:** pre-existing staleness (not introduced by S-21.25, but load-bearing for this story)
- **Location:** `.factory/specs/domain-spec/capabilities.md` §CAP-011 body text
- **Description:** CAP-011's body read "Every plugin invocation has a bounded fuel cap (default
  10M operations)...", but ADR-042 §Decision 2 raised the default fuel cap to 20M operations. The
  10M figure predated ADR-042's fuel-cap raise and had gone stale. BC-1.03.019 anchors to CAP-011
  as its Capability Anchor, making the staleness load-bearing for this story's spec chain even
  though the defect pre-dates S-21.25's authoring.
- **Evidence:** ADR-042 §Decision 2 (fuel-cap raise 10M→20M) vs. CAP-011 body's unchanged "default
  10M operations" text.
- **Proposed Fix:** Correct CAP-011 body to "default 20M operations (per ADR-042 §Decision 2)". No
  capability semantics, subsystem mapping, or outcome statement altered — precision fix only.
  Route to `vsdd-factory:architect` (capabilities.md content is architect/business-analyst scope;
  architect performed this fix as the CAP-011 owner for the ADR-042 cross-reference).
- **Status:** RESOLVED this burst — see Disposition.

## Disposition

All 7 findings routed and remediated in the same fix burst (no BLOCKER; production-grade default
— fix in scope, no deferral):

- **story-writer** (S-21.25 v1.0→v1.1): F-S2125-P1-002 extracted named pure helpers
  `fuel_headroom_exceeded`/`fuel_headroom_ratio` + `check_and_emit_fuel_headroom_warning`
  orchestration shell, single call site; F-S2125-P1-001 AC-005 regression guard redesigned around
  a `// SINGLE-EMIT-SITE` marker-scan (satisfiable, refactor-tolerant); F-S2125-P1-005 AC-006
  field enumeration corrected (+`message`/`timestamp`); F-S2125-P1-006 cascade AC-008 message
  string corrected to strict `>90%`; BC-1.03.019 v1.0→v1.1 and ADR-039 v1.14→v1.15 re-anchor swept
  throughout.
- **product-owner** (BC-1.03.019 v1.0→v1.1; BC-3.08.001 v1.24→v1.25): F-S2125-P1-003 PC6 field set
  corrected (+`timestamp`/`message`) with S-19.09 sibling-parity cross-reference; BC-3.08.001
  Event 7 `plugin.fuel_headroom_warning` registered in the SS-03 catalog (six→seven count-phrase
  sweep, H1 title updated); F-S2125-P1-005 PC6 false-exclusivity wording corrected; F-S2125-P1-006
  cascade PC8 message string corrected to strict `>90%`; F-S2125-P1-004 v1.0 Changelog row
  "PENDING" narrative corrected to the actual `57262cf` value.
- **architect** (ADR-039 v1.14→v1.15; capabilities.md v1.11→v1.12; VP-079 v1.19→v1.20):
  F-S2125-P1-006 §Decision 5 Mitigation 1 WARN message corrected (`≥90%`→`>90%`) via non-
  re-ratifying §Erratum E-006; F-S2125-P1-007 CAP-011 body corrected (10M→20M, ADR-042 cite);
  VP-079 amended (POLICY 9 propagation) to add Event 7's Mandatory-Fields row and SITE_7 with an
  explicit not-yet-mutation-proven scope note (SITE_7's fixture is pending S-21.25 delivery;
  Event 7's triggering-condition/semantics properties remain out of VP-079's scope, owed to a
  forthcoming BC-1.03.019-anchored VP-TBD).
- **state-manager** (this burst, D-1059): F-S2125-P1-004 input-hash reconcile — S-21.25, 
  BC-1.03.019, BC-3.08.001, VP-079, capabilities.md all recomputed/verified via the per-file
  operator `compute-input-hash` binary in dependency order (capabilities.md → BC-1.03.019 →
  BC-3.08.001 → VP-079 → S-21.25); BC-INDEX/ARCH-INDEX/VP-INDEX/STORY-INDEX 4-index propagation.

verification-architecture.md and verification-coverage-matrix.md were checked by architect and
confirmed to require no per-event edit (VP-079's row in both files is a bare stable anchor with no
per-event-count suffix; POLICY 9 grep-verified).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 2     |
| MEDIUM   | 2     |
| LOW      | 3     |

**Overall Assessment:** block (pre-remediation, v1.0) — RESOLVED same burst via story-writer +
product-owner + architect + state-manager remediation (D-1059).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving findings does not itself
advance the streak per BC-5.39.001; pass 2 required against the v1.1 bundle to confirm CLEAN).
**Readiness:** requires re-review (pass 2, against S-21.25 v1.1 + BC-1.03.019 v1.1 +
BC-3.08.001 v1.25 + ADR-039 v1.15 + VP-079 v1.20 + capabilities.md v1.12) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 7 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (7/7) |
| **Median severity** | 3.0 (MEDIUM — 2 HIGH, 2 MEDIUM, 3 LOW) |
| **Trajectory** | 7 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1059); pass 2 required to confirm CLEAN against the remediated bundle. |
