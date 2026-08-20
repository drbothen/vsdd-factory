---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-19T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "fba5b34"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 5
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-4.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 5)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.4 (input-hash `97029a5`); `BC-1.03.017.md` v1.16; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `677d3da9` (D-1044 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 HIGH finding (streak-resetting). 1 non-resetting LOW/ADVISORY observation. Multiple grounding
confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), and pass-4 (F-S2111V2-P4-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P4-001 | MEDIUM | RESOLVED | story-writer swept 58 live `BC-1.03.017 v1.15`→`v1.16` cites into the S-21.11 story body (frontmatter array + body BC-table split-cell + 56 narrative/AC/Task/EC sites); re-verified this pass, no residual v1.15 cite anywhere live in the story. |

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S2111V2-P5-001 (HIGH, streak-resetting)

**Location:** BC-1.03.017.md `## Invariants` → Invariant 10 ("PC13 strict-superset invariant"),
plus 4 further residual sites found on exhaustive sweep: §Architecture Anchors `executor.rs`
bullet's closing sentence, §Traceability ADR-039 row's §AMD-003 citation closing clause, EC-011's
"Post-wiring-fix" clause, and PC13's own header grouping.

**Defect:** D-1043's remediation (BC-1.03.017 v1.15→v1.16) narrowed the §AMD-003 fail-closed
predicate at the Architecture Anchors and Traceability citation sites, but the sweep that produced
v1.16 was a targeted grep-driven sweep, not an exhaustive semantic sweep — it missed
`## Invariants` → Invariant 10, which still (a) reused the STRICT-SUPERSET-of-`Crashed | Timeout`
framing that ADR-039 Erratum E-005 identified as the root error and explicitly removed from the
authoritative "Precise rule (normative)" paragraph, and (b) literally contradicted this same BC's
own axes-independence semantics (PC5, PC10(a), EC-009, Invariant 1) by asserting that `Crashed`
and `Timeout` "continue to block under `on_error=Block` exactly as before" — wrongly implying that
`on_error` alone governs `Timeout` blocking, and mis-labeling PC10 as part of an
"`on_error`-governs-crash path" (PC10 in fact governs `Timeout`/`failure_policy`, not crash).

This is the FIFTH instance in the S-21.11 v2 cascade of the version-cite-propagates/
algorithm-content-does-not defect class first codified for the S-21.07 cascade at D-1006 — but
unlike passes 3 and 4 (pure version-*citation* staleness, content unaffected), this instance is a
**content-level** predicate-coherence residue: the remediation that produced v1.16 fixed the
predicate at SOME sites but not ALL sites stating the same predicate, within the SAME burst and
the SAME BC file. This confirms a narrower, more dangerous sub-class: a targeted-grep sweep for a
specific string pattern (e.g., "STRICT-SUPERSET" or "on_error-governs-crash") systematically
misses sibling sites that state the identical *concept* using different wording — TD-VSDD-060
sibling-sweep discipline must therefore apply to CONCEPTS, not only to literal string patterns.
Three consecutive passes (pass-3, pass-4-adjacent content risk, and now pass-5) have each
surfaced the same predicate-coherence class at a progressively deeper site the prior sweep missed.

**Routed:** product-owner (BC-1.03.017 Invariant 10 rewrite + exhaustive semantic sweep of every
predicate-stating site); story-writer (propagate the resulting version bump into the story).

**RESOLVED this burst:**
- **product-owner** — BC-1.03.017 v1.16→v1.17: rewrote and retitled Invariant 10 (new title: "PC13
  additive-only invariant — NOT a `Crashed | Timeout` superset") to state the three outcome shapes
  under `on_error=Block` as three separate, axes-independent rules rather than one shared
  predicate: `Crashed` governed solely by `on_error`/PC4; `Timeout` governed exclusively by
  `failure_policy`/PC1/PC5/PC6/PC10, never by `on_error` alone; `Ok{exit_code != 0}` is the one new
  PC13 leg, not a negation of `Ok{exit_code: 0}`. Ran an exhaustive sweep (not a single grep
  pattern) of every predicate-stating location — all Preconditions/PCs, all Invariants, Edge
  Cases, Architecture Anchors, Verification Properties, and Traceability — and found the SAME
  residual contradiction pattern surviving in 4 further sites the D-1043 sweep missed:
  - Architecture Anchors `executor.rs` bullet's closing sentence — rewritten to state the two base
    rules as governed by different axes (`Crashed`→`on_error`/PC4; `Timeout`→`failure_policy`/
    PC1/PC5/PC6/PC10), never one shared predicate.
  - Traceability ADR row's §AMD-003 citation closing clause — same rewrite applied.
  - EC-011's "Post-wiring-fix" clause, which claimed both the `Ok{exit_code:1}` and `Timeout`
    sub-outcomes "MUST produce a block under `on_error=Block`" and mis-labeled PC10 (paired with
    PC4) as closing the "`Timeout`/`Crashed`" case jointly — rewritten to condition each outcome on
    its own governing axis (`Ok{exit!=0}`→PC13/`on_error`; `Timeout`→PC1/PC6/PC10(b)/
    `failure_policy=FailClosed`) with an explicit note that the scenario assumes the plugin's
    steady-state `failure_policy=FailClosed` annotation (PC9), not `on_error=Block` alone.
  - PC13's own header, which grouped PC4/PC5/PC10 together as "`on_error`-vs-`Crashed` coverage" —
    imprecise, since PC5/PC10 are `Timeout`/`failure_policy` axes-independence coverage, not crash
    coverage; split into "PC4's `on_error`-governs-crash coverage" and "PC5/PC10's
    `failure_policy`-governs-`Timeout` axes-independence coverage."
- **story-writer** — S-21.11 v2.4→v2.5: swept 58 live `BC-1.03.017 v1.16`→`v1.17` cites
  (frontmatter `behavioral_contracts:` array + body BC-table split-cell + Routing Proposals
  parenthetical + 55 further narrative/AC/Task/EC sites). Exempted per POLICY 5 v1.3.5: 2
  occurrences of `BC-1.03.017 v1.16` inside the story's OWN historical `## Changelog` table (both
  in the v2.4 row) — historical-by-construction, correctly left unswept. Ran a defensive semantic
  check for the predicate-coherence theme (`strict superset`, `superset of`, `continue to block`,
  `exactly as before`, `NOT Ok{exit_code`, `regardless of failure_policy`) across the story body —
  confirmed clean, no in-scope fix required (the two live "superset" hits are the pre-existing,
  unrelated AC-022⊃AC-012 gate-coverage relationship). input-hash reconcile flagged for
  state-manager (resolved separately, see below).

### LOW / ADVISORY (non-resetting)

**BC-1.03.017/BC-1.03.018 Traceability rows carry inline ADR ratification-provenance version
tokens paired with stable §AMD-00N/§Decision-N anchors.** These are historical facts describing
WHEN a given ADR clause was ratified (e.g. "§AMD-003 v1.11 substantively RATIFIED"), not POLICY 19
volatile-pin violations — POLICY 19 prohibits pinning to values that will silently go stale and
break traceability; a ratification-provenance date/version is inherently historical and does not
need updating when the ADR later gains new unrelated amendments. No action; recorded so a future
pass doesn't misread these as a POLICY 19 violation.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 1 (advisory, no action) |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-6 required)
**Readiness:** requires revision (routed product-owner + story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Version-cite parity held pre-fix.** BC-1.03.017 v1.16 citation was consistent across the story
  (frontmatter + body), BC-INDEX, and STORY-INDEX prior to this pass — pass-4's remediation was
  itself clean of any residual v1.15 citation.
- **18-entry `on_error="block"` registry set exact.** `grep -c` against the live
  `hooks-registry.toml` returns 18, matching AC-024..AC-041 and PC13's Coverage Set table
  row-for-row. No drift since pass-3.
- **Token Budget POLICY 8 parity held.** No BC content-size delta affecting the story's Token
  Budget table this pass beyond the version-cite sweep itself.
- **BC H1 ↔ BC-INDEX title cell POLICY 7 parity held.** BC-1.03.017's H1 is an unmodified,
  verbatim match to the BC-INDEX title cell both before and after the v1.17 predicate-coherence
  edit (the edit touched only Invariants/Architecture Anchors/Traceability/EC-011/PC13-header
  body content, not the H1/Description).
- **FUEL vs epoch-timeout arms remain correctly distinguished** in the story's Token Budget
  Context Source line and nowhere conflated.
- **Story does NOT echo the broad `Crashed | Timeout` superset framing anywhere** — the story's
  own AC-013b/Task #19b language was already narrow and correct (confirmed at pass-3's Part A);
  only the BC's own Invariant 10 (a BC-authored artifact, not story-authored) carried the residual
  contradiction.

## Observations (non-resetting)

- **[carry-forward, = known F-007]** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked as `[F-007]` in STATE.md Blocking Issues/Drift Items, anchored
  to a future dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM-HIGH. Fifth instance of the version-cite-propagates/algorithm-content-does-not class
in this cascade, but the first instance where the residue is a CONTENT-level predicate
contradiction within the SAME artifact and the SAME remediation burst that fixed sibling sites of
the identical concept, rather than a cross-artifact version-citation staleness gap. Orchestration
lesson: literal-string-pattern sweeps ("grep for the exact phrase that was fixed") systematically
under-cover a predicate-coherence defect class, because sibling sites restate the same predicate in
different words. A semantic sweep — enumerate every location that STATES the concept, not every
location that contains a specific string — is required whenever a predicate/invariant is narrowed
or corrected. This pass's exhaustive semantic sweep found and fixed 5 sites in one burst (versus
one site per burst at passes 3/4), which should terminate this specific treadmill; TD-VSDD-060
sibling-sweep discipline is hereby understood to extend to CONCEPTS, not only string patterns.
