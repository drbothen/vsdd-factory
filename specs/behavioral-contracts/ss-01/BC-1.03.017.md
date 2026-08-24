---
document_type: behavioral-contract
level: L3
version: "v1.23"
status: draft
producer: product-owner
timestamp: 2026-08-06T00:00:00Z
last_amended: "2026-08-23 (v1.23) — Wave-7 adversary pass-4 remediation, cycle
v1.0-brownfield-backfill: (F-S2121-P4-001, HIGH) EC-011 pre-wiring-fix enforcement
characterization corrected: the prior clause claimed no fail-closed enforcement reaching
`on_error=Block` in BOTH sub-cases, contradicting this BC's own Invariant 12 (added v1.22).
Sub-case analysis: (a) `Ok{exit_code:1}` pre-§AMD-003 IS NOT blocked (PC13/§AMD-003 closes
that gap); (b) `Timeout{Epoch}` IS blocked by the retained 2-arg `plugin_fail_closed` call
via `on_error=Block` (Invariant 12 guarantees this throughout wave 7); defect for sub-case (b)
is TIMING ONLY (block fires at ≈60 s hardcoded constant, not calibrated `timeout_ms`) — NOT
enforcement absence. EC-011 and its Canonical Test Vector row updated to reflect this
sub-case distinction. (F-S2122-P4-001, LOW) ceil() adopted throughout: `max(observed_max ×
1.5, 50_000_000)` → `max(ceil(observed_max × 1.5), 50_000_000)` throughout all live predicate
occurrences (Precondition 3, Precondition 6, Invariant 2, PC9, VP Anchors, Architecture
Anchors). Rationale: `observed_max × 1.5` can yield a non-integer fuel unit; `ceil()` rounds
up to the nearest whole unit, ensuring `fuel_cap` is never truncated below the formula target.
Production-grade decision: strictly more conservative than truncation, preventing
boundary-case underflow. Formula: `max(ceil(observed_max × 1.5), 50_000_000)`. (F-S2121-P4-002,
MED, PO confirm) Precondition 6 S-21.22 ownership made explicit: (i) Task 4 one-time
calibration confirmation OWNED BY S-21.22; (ii) Task 5a durable standing CI gate OWNED BY
S-21.22. Prior text described the mechanism without naming the owning story. H1 enriched per
POLICY 7 with ceil() adoption, EC-011 sub-case correction, and S-21.22 PC6 ownership clauses.
No PC renumbering; all amendments corrective/additive. Does NOT touch BC-1.03.018 (separate
file), story bodies/ACs (story-writer step ②), or INDEX/STATE.md (state-manager step ③).
Input-hash flagged stale for state-manager reconciliation. BC-1.03.017 v1.23. [Prior:
2026-08-22 (v1.22) — Wave-7 adversary pass-3 remediation, cycle
v1.0-brownfield-backfill: (1a) F-S2122-P3-001 (MEDIUM) — Precondition 6 (the new
calibration-sufficiency check, informally cited as \"PC6\" in the finding) conflated ONE-TIME
live-corpus calibration confirmation (Task 4, deriving the annotated `fuel_cap`) with the
DURABLE, standing CI regression assertion (Task 5a); S-21.22's converged Task 5a makes the
standing gate run against a FROZEN corpus snapshot (`pc6-sufficiency-snapshot/`), explicitly
NOT live-growing files, because a standing assertion against live `decision-log.md`/
`STATE.md`/`lessons.md` is a false-fail time bomb as those files organically grow — the BC
cited a different mechanism than the story's converged one. Rewrote Precondition 6 to
distinguish the two checks explicitly: (i) the one-time live-corpus confirmation MAY reference
live files at calibration time (its purpose is to derive the `fuel_cap` value, not to be the
standing gate); (ii) the durable standing gate MUST run against the frozen snapshot. Made the
frozen-snapshot form the MANDATED shape for the standing gate, matching the story. The `>=`/
`PRACTICAL_FUEL_CEILING` trigger semantics are UNCHANGED. Swept sibling restatements for
TD-VSDD-060 consistency: two Canonical Test Vector rows updated + one new row added (frozen-
snapshot decoupling from organic corpus growth), the VP-TBD Verification Properties row (both
the Property cell's Precondition 6 clause and the Proof Method cell), and the Traceability
§Decision 4 ADR citation. (1b) F-S2121-P3-001 (HIGH, from architect ADR-044 v1.3) — new
Invariant 12 (migration coverage-continuity): across the wave-7→8 split-topology migration of
the executor's block-decision predicates, at every commit from S-21.19's merge through
S-21.24's merge, the live block-decision call site MUST block `Timeout` outcomes under
`on_error=Block` via SOME disjunct — the retained pre-migration 2-arg `plugin_fail_closed`
call through wave 7, `plugin_fail_closed_on_exhaustion` from S-21.24's single migration
commit onward — such that no commit in this range fails open on `Timeout{Fuel|Epoch}` for any
`on_error=Block`-registered plugin, including the two `^Agent$` self-lock-hazard gates
BC-1.03.018 governs. Explicitly distinguished from, and additive to, Invariant 7/PC11's
annotation-vs-enforcement direction (neither substitutes for the other). Cites ADR-044 v1.3
Addendum (RATIFIED 2026-08-22, closes HIGH F-S2121-P3-001). Added a light \"additive, not
replacing\" correction to the Architecture Anchors `executor.rs` bullet's S-21.21 wiring
sentence (the v1.1 Addendum framing this BC previously cited was itself corrected by ADR-044
v1.2→v1.3 to fix a HIGH fail-open defect in the wiring sequence) and extended the Traceability
ADR-044 citation with the v1.3 correction and Invariant 12 cross-reference. H1 enriched per
POLICY 7 with both the frozen-snapshot and migration-coverage-continuity clauses. No PC
renumbering; Invariant 12 is new (additive); Precondition 6's rewrite is corrective (frozen-
vs-live split), not a change to its `>=`/`PRACTICAL_FUEL_CEILING` trigger semantics; PC1-PC13
core predicates UNCHANGED. Does NOT touch BC-1.03.018 (separate product-owner burst, same
task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately
this burst per POLICY 8 propagation — S-21.22's Task 5a and S-21.21's wiring-sequence
citations need re-anchoring to this corrected text), or any INDEX/STATE.md (state-manager's
domain). This content change means this BC's own `input-hash` will go stale as a result and is
flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.22. [Prior:
2026-08-22 (v1.21) — Wave-7 adversary pass-2 remediation, cycle
v1.0-brownfield-backfill (F-S2121-P2-004): PC12's kill-time margin was written as \"observed kill
time within `X` plus the `exec_subprocess.rs::run()` poll interval (~5 ms)\" — this conflated the
executor's INTERNAL deadline-poll sampling granularity (~5 ms,
`std::thread::sleep(Duration::from_millis(5))`) with the OBSERVED end-to-end kill latency a real
e2e test (S-21.21's AC-013) must assert against, which spans the WASI host-call boundary, the
poll loop, `SIGKILL` delivery, process reap, and the test's own wall-clock measurement — all
subject to CI-runner scheduling jitter, realistically landing in the HUNDREDS-OF-MILLISECONDS
range above `timeout_ms`, not ~5 ms. The ~5 ms bound was a flaky-test generator; S-21.21's own
remediation had already loosened AC-013 to a one-sided hundreds-of-ms margin, creating a
story-vs-BC divergence this fix closes. Rewrote PC12 to the production-grade model: (a) kept
the ONE-SIDED guarantee — killed AT OR AFTER `X`, `X` is a FLOOR, never a ceiling, kill MUST
NOT fire before `X`; (b) added an explicit distinction between (i) the internal ~5 ms poll
granularity (an implementation-mechanism detail, not an e2e-observable bound) and (ii) the
OBSERVED end-to-end kill time, which MUST be asserted as the bounded window
`X <= observed_kill_time <= X + margin` with a CI-jitter-robust `margin` in the
hundreds-of-ms range — still ≪ the 60 s hardcoded-constant proof point, so the test still
proves the registry's `timeout_ms` is honored, not the unrelated hardcoded default coincidentally
passing a loose assertion. Updated PC12's POSITIVE control wording to the same bounded-window
form. Swept sibling restatements of the ~5 ms figure for TD-VSDD-060 consistency: the
`AMD-002-wiring-fixed` Canonical Test Vector row, the `exec_subprocess.rs::run()` Architecture
Anchors bullet, and the VP-TBD Verification Properties row's PC12 clause — all four sites now
distinguish internal poll granularity from the CI-jitter-robust observed-margin bound. H1
enriched per POLICY 7 with the kill-time-margin correction clause. No PC renumbering;
PC12-scoped corrective rewrite only — PC1-PC11, PC13, and Precondition 6 UNCHANGED. Does NOT
touch BC-1.03.018 (separate product-owner burst, same task), the S-21.21 story body/ACs
(story-writer's domain, dispatched separately this burst per POLICY 8 propagation — S-21.21's
AC-013 wording and its \"reconciled with PC12\" claim need re-anchoring to this corrected PC12
text), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own
`input-hash` will go stale as a result and is flagged for state-manager reconciliation
same-burst. BC-1.03.017 v1.21. [Prior:
2026-08-22 (v1.20) — Wave-7 adversary pass-1 remediation, cycle
v1.0-brownfield-backfill (Q1+Q2 architect-adjudicated, human-approved Q1=Option A; Q3 governs
sibling BC-1.03.018 only, not this BC): (1) Q2/F-S2122-P1-002 fuel_cap calibration-statistic bug
fix — Precondition 3's `fuel_cap` axis corrected from `measured_p99 × 1.5` to `observed_max ×
1.5` per ADR-039 §Decision 4 v1.16/§Erratum E-007 (Erratum-class, no new decision content); the
`timeout_ms` axis is UNCHANGED (`measured_p99_ms × 2.0` remains correct — wall-clock
non-determinism keeps the p99 statistic there per §Decision 4's own unchanged rationale); the
50_000_000 value is now explicitly framed as an inclusive FLOOR beneath the `observed_max × 1.5`
TARGET, not the target itself. Swept sibling restatements of the fuel_cap formula for
TD-VSDD-060 consistency: Invariant 2, EC-004, PC9 (added an explicit Precondition-6-dependency
clause and floor-vs-target qualifiers), Architecture Anchors (`hooks-registry.toml` six-plugin
bullet), Traceability's §Decision 4 ADR citation, and the Verification Properties VP-TBD row.
(2) Q2/F-S2122-P1-002 new Precondition 6 — a machine-checkable calibration-sufficiency
requirement applicable to ALL SIX ADR-039 §Decision 2 fail-closed validators: the annotated
`fuel_cap` MUST be `>= observed_max × 1.5` measured against the mandated calibration corpus
(`lessons.md` >=4000 lines; `STATE.md`/`decision-log.md` at live size; the >=574 KB
(576,396-byte) production-scale `BC-INDEX.md` fixture, per the existing S-21.22
calibration-corpus mandate), verified by a regression assertion (`fuel_consumed × 1.5 <=
registry fuel_cap`), not merely the 50M floor; cites the `PRACTICAL_FUEL_CEILING`
(`500_000_000`, ADR-039 §Decision 4 v1.16) trigger shared with EC-004. Two new Canonical Test
Vector rows added (sufficiency-assertion positive/negative controls). (3) Q1/F-S2121-P1-002
Architecture Mapping split (ADR-044 v1.1 Addendum, RATIFIED 2026-08-22, architect Q1
adjudication Option A) — replaced the single `plugin_fail_closed` (3-arg + \"PC13 extension\")
Architecture Anchors citation with the two named functions ADR-044 v1.1 introduces:
`plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)` (the `Timeout`/
`failure_policy` exhaustion axis governing PC1/PC2/PC3/PC5/PC6/PC10/PC11; wiring UNCHANGED,
capstone-owned, deferred to S-21.24) and `plugin_fail_closed_on_error_exit(result, on_error)`
(PC4's `Crashed` case EXTENDED per §AMD-003 to PC13's `on_error==Block && Ok{exit_code!=0}`
case; wireable immediately, now OWNED BY S-21.21 at wave 7, NOT S-21.24). Traceability's
ADR-044 citation extended with the v1.1 Addendum's function split and story-ownership
correction. (4) VP-TBD hygiene (S-21.21 O-3) — added an explicit anchor note (VP Anchors +
Verification Properties) matching the S-21.25/BC-1.03.019 POLICY-9 sanctioned-deferral
precedent: the real VP covering PC13 semantics and the new Precondition-6 sufficiency property
is anchored to Phase-6 formal-verifier, preventing the placeholder from reading as an
un-anchored defect. H1 enriched per POLICY 7 with the new sufficiency-gate and function-split
clauses. No PC renumbering; Precondition 6 is new (additive); all other changes are
corrective/citation-only within existing Preconditions/Invariants/Architecture
Anchors/Traceability/Verification Properties sections — PC1-PC13's core predicates are
UNCHANGED. Does NOT touch BC-1.03.018 (Q3's amendment — separate product-owner burst, same
task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately
this burst per POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain, finalizes
the spec-layer commit). This content change means this BC's own `input-hash` will go stale as a
result and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.20. [Prior:
2026-08-20 (v1.19) — S-21.19 pass-2 remediation of F-S2119-P2-001 (MEDIUM,
brownfield cycle v1.0-brownfield-backfill): Invariant 7's human-readable atomicity policy
literally contradicted ADR-044 (capstone-owned enforcement flip). ADR-044 refined the CWE-636
regression TRIGGER from \"the extended decision function EXISTS/is contained in a commit\" to
the narrower \"the extended function is WIRED INTO the executor block-decision chain (references
`.failure_policy` for a `Timeout` block decision) — enforcement-active per PC11's static-scan
signal\"; PC11 was already updated to this wiring-keyed form (v1.3-v1.5), but Invariant 7 still
read \"Any CI-passing commit that contains the extended function while any of these five plugins
remains at failure_policy = fail-open ... is a CWE-636 regression\" and \"The decision-function
change and the annotations MUST be co-committed... or ordered annotate-before-flip\" — read
literally, S-21.19's OWN compliant merge (a commit containing the inert, un-wired extended
function while the 5 plugins are still fail-open) was a \"CWE-636 regression\" by Invariant 7's
words, directly contradicting ADR-044's declaration that this exact state is SAFE because the
function is not wired; Invariant 7 also conflated \"decision-function change (authoring —
S-21.19, inert)\" with \"executor flip (wiring — S-21.24 Task 0)\". Rewrote Invariant 7 to key the
CWE-636 regression trigger on the function being WIRED INTO / IN EFFECT in the block-decision
chain (enforcement-active per PC11's signal), explicitly disambiguating authoring (3-arg function
+ `PluginOutcome.failure_policy` population — inert data plumbing, S-21.19, NOT the flip, NOT
prohibited) from wiring (replacing the `execute_tiers`/`execute_tier` 2-arg call site with the
3-arg form — the enforcement-active flip, S-21.24 Task 0, IS what must be atomic-with/after the
five annotations). Added ADR-044 to this BC's `inputs:` and Traceability ADR row (new citation
naming S-21.19 as capstone-adjacent authoring leg and S-21.24 as capstone wiring leg, and stating
that Invariant 7 is now this BC's human-readable restatement of ADR-044's authoring-vs-wiring
disambiguation, keyed on the same PC11 signal). Verified Invariant 7 is now internally consistent
with PC11 (both key on the same wiring/enforcement-active signal — \"any block-decision site in
`execute_tier`/`execute_tiers`/helpers references `.failure_policy` for a `Timeout` outcome\"),
with ADR-044 (same authoring-vs-wiring split, same S-21.19/S-21.24 story mapping), and with
PC5/PC10/Invariant 1's pre-existing axes-independence (untouched — Invariant 7's edit is scoped
to the migration-ordering trigger only, not the `on_error`-vs-`failure_policy` axis rule).
Swept sibling prose: PC8's \"Symmetric half-state prohibition\" paragraph (line ~355) already
correctly says \"Once the extended 3-arg `plugin_fail_closed` function is present in any
CI-passing commit, no targeted plugin currently carrying `on_error = \\\"block\\\"` ... MAY remain
at `failure_policy = fail-open`\" — this is PC8's OWN scope note pointing to PC11 as the
mechanical enforcer (\"The mechanical CI gate enforcing this ordering constraint is PC11 ...
not this gate\"), not an independent regression-trigger assertion; left as-is (PC8's cross-
reference to PC11 remains accurate — PC11 is the wiring-keyed mechanical gate). No other
\"contains the extended function\" framing found elsewhere in the BC (PC11 lines 471-541 already
correct, untouched per scope; EC/PC13/Invariants 8-11 unrelated to this trigger). No PC
renumbering; Invariant-7-scoped rewrite plus one new Traceability ADR citation. Does NOT touch
PC11 (already correct), the 5 citing stories (story-writer's domain — S-21.19, S-21.20, S-21.21,
S-21.22, S-21.24 cite re-anchor is story-writer's next dispatch), ADR-044/ADR-039 (architect's
domain), BC-1.03.018/BC-1.03.019, or any INDEX/STATE.md (state-manager's domain). Adding ADR-044
to `inputs:` means this BC's own `input-hash` will go stale as a result of this content change
and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.19. [Prior: 2026-08-20
(v1.18) — S-21.11 v2.8 PRE-TDD spec-convergence pass-9 remediation of
F-S2111V2-P9-001 (MEDIUM): the live `## Traceability` ADR row's §Decision 3 sub-clause cited the
break-glass amendment's delivery vehicle two contradictory ways in the SAME cell — \"...v1.9
amendment: mandatory authenticated break-glass companion, S-21.17\" (a retired, never-authored
story ID, absent from STORY-INDEX) immediately followed later in the same cell by \"...delivered
WITHIN S-21.11...governed by sibling BC-1.03.018...\". This BC's own v1.11 changelog claimed the
S-21.17->S-21.11 citation redirect was complete, but this one occurrence was missed. Rewrote the
live cite to \"mandatory authenticated break-glass companion, delivered within S-21.11 (prior
follow-up name S-21.17 retired))\" — matching the same cell's \"delivered WITHIN S-21.11\" clause
and BC-1.03.018's Stories-row retirement-annotation convention (\"the prior follow-up name S-21.17
is retired\"). Ran a literal grep sweep (`grep -no \"S-21\\.17\"`) of both BC-1.03.017 and
BC-1.03.018 (TD-VSDD-060) and classified every occurrence by captured evidence, not assertion:
BC-1.03.017 had 4 line-hits — line 99 (this frontmatter `last_amended` field, describing v1.11's
own historical S-21.17->S-21.11 redirect: HISTORICAL, left as-is), line 1016 (live
`## Traceability` ADR row, the F-S2111V2-P9-001 site: LIVE, fixed above), line 1031 (v1.11
`## Changelog` row, 2 occurrences, both describing the v1.11 redirect action retrospectively:
HISTORICAL, left as-is), and line 1032 (v1.10 `## Changelog` row, describing the not-yet-authored
S-21.17 amendment as it stood at v1.10: HISTORICAL, left as-is). BC-1.03.018 had 1 line-hit — its
Stories row (\"the prior follow-up name S-21.17 is retired\"): already correctly annotated per the
retirement-annotation convention this BC now matches; verified, no edit needed. No
PC/Invariant/predicate content altered; citation-only. POLICY 8 parity preserved: this BC has no
frontmatter `behavioral_contracts`/`bcs` array, no body BC-table, and no `## Acceptance Criteria`
section (ACs live in the S-21.11 story body) — only the `## Traceability` ADR-row prose changed.
Does NOT touch ADR-039 (architect's domain), BC-1.03.018 (verified compliant, no edit needed), the
S-21.11 story (story-writer's domain — the v1.17->v1.18 cite sweep is story-writer's next
dispatch), or any INDEX/STATE.md (state-manager's domain). BC-1.03.018's `inputs:` declaration of
this BC means its input-hash will go stale as a result of this content change and is flagged for
state-manager reconciliation same-burst. BC-1.03.017 v1.18. [Prior: 2026-08-19 (v1.17) — S-21.11 v2.4 PRE-TDD spec-convergence pass-5 remediation of
F-S2111V2-P5-001 (HIGH): the prior v1.16 remediation narrowed the §AMD-003 fail-closed
predicate at the Architecture Anchors and Traceability sites but missed `## Invariants` ->
Invariant 10 (\"PC13 strict-superset invariant\"), which still (a) reused the STRICT-SUPERSET-of-
`Crashed | Timeout` framing that ADR-039 Erratum E-005 identified as the root error and removed
from the authoritative Precise Rule, and (b) literally contradicted this BC's own axes-
independence semantics (PC5/PC10(a)/EC-009/Invariant 1) by asserting `Crashed` and `Timeout`
\"continue to block under on_error=Block exactly as before,\" wrongly implying `on_error` alone
governs `Timeout` blocking and mis-labeling PC10 as part of an \"on_error-governs-crash path\"
(PC10 governs `Timeout`/`failure_policy`, not crash). Rewrote Invariant 10 (retitled \"PC13
additive-only invariant — NOT a `Crashed | Timeout` superset\") to state the three outcome
shapes under `on_error=Block` (`Crashed` governed solely by `on_error`/PC4; `Timeout` governed
exclusively by `failure_policy`/PC1/PC5/PC6/PC10, never by `on_error` alone; `Ok{exit_code!=0}`
the one new PC13 leg, not a negation of `Ok{exit_code:0}`) as three separate, axes-independent
rules rather than one shared predicate. Ran an exhaustive sweep (not a single grep pattern) of
every predicate-stating location — all Preconditions/PCs, all Invariants, Edge Cases,
Architecture Anchors, Verification Properties, and Traceability — and found the SAME residual
contradiction pattern (\"base rule ... governed solely by on_error\" applied to BOTH `Crashed`
and `Timeout` jointly) surviving in two more sites the prior sweep's narrow grep missed: the
Architecture Anchors `executor.rs` bullet's closing sentence, and the Traceability ADR row's
§AMD-003 citation closing clause. Both rewritten to state the two base rules as governed by
different axes (`Crashed`->`on_error`/PC4; `Timeout`->`failure_policy`/PC1/PC5/PC6/PC10), never
one shared predicate. Also fixed: EC-011's \"Post-wiring-fix\" clause, which claimed both the
`Ok{exit_code:1}` and `Timeout` sub-outcomes \"MUST produce a block under on_error=Block\" and
mis-labeled PC10 (paired with PC4) as closing the \"Timeout/Crashed\" case jointly — rewritten to
condition each outcome on its own governing axis (`Ok{exit!=0}`->PC13/on_error; `Timeout`->
PC1/PC6/PC10(b)/failure_policy=FailClosed) with an explicit note that the scenario assumes the
plugin's steady-state `failure_policy=FailClosed` annotation (PC9), not `on_error=Block` alone.
Also fixed PC13's own header, which grouped PC4/PC5/PC10 together as \"on_error-vs-Crashed
coverage\" — imprecise, since PC5/PC10 are `Timeout`/`failure_policy` axes-independence
coverage, not crash coverage; split into \"PC4's on_error-governs-crash coverage\" and \"PC5/
PC10's failure_policy-governs-Timeout axes-independence coverage.\" Sites examined (complete
list): H1; Description; Preconditions 1-5; PC1-PC13 (all bodies, including PC13's Coverage Set
table); Invariants 1-11; Edge Cases EC-001 through EC-011; Canonical Test Vectors (all rows);
Related BCs; Architecture Anchors (all bullets); Story Anchors; VP Anchors; Verification
Properties (VP-TBD row); Traceability (all rows). Sites rewritten: Invariant 10 (full rewrite),
Architecture Anchors `executor.rs` bullet closing sentence, Traceability ADR row §AMD-003
closing clause, EC-011 post-wiring-fix clause, PC13 header's PC4/PC5/PC10 grouping label. All
other sites verified internally consistent with the narrow additive-only / axes-independent
predicate — no further residue. Sibling-swept BC-1.03.018 (TD-VSDD-060): grepped for
\"strict superset\", \"exactly as before\", \"governed solely by\", and `Crashed | Timeout` —
no occurrence found; no BC-1.03.018 edit needed. No AC in this BC restates the broad predicate
(this BC has no `## Acceptance Criteria` section — ACs live in the S-21.11 story body,
story-writer's domain, not swept here). POLICY 8 parity preserved: this burst does not touch
frontmatter `behavioral_contracts`/`bcs` arrays, a body BC-table, ACs, or Token Budget — only
`## Invariants`, `## Edge Cases`, `## Architecture Anchors`, and `## Traceability` prose
changed (all Invariant-adjacent predicate prose, not structural). No PC renumbering;
corrective-only. Does NOT touch ADR-039 (architect's domain, already correct via E-005),
BC-1.03.018, the S-21.11 story body/ACs (story-writer's domain — the v1.16->v1.17 cite sweep is
story-writer's next dispatch), or any INDEX/STATE.md (state-manager's domain). The S-21.11
story's and BC-1.03.018's input-hashes, which declare this BC as an input, will go stale as a
result of this content change and are flagged for state-manager reconciliation same-burst.
BC-1.03.017 v1.17. [Prior: 2026-08-19 (v1.16) — S-21.11 v2.3 PRE-TDD spec-convergence cascade remediation
of F-S2111V2-P3-001 (HIGH): swept the two remaining BC-body sites (Architecture Anchors
`executor.rs` PC13-extension clause; Traceability ADR §AMD-003 citation) that still carried the
REJECTED broad-negation form of the fail-closed predicate
(`on_error==Block AND result is NOT Ok{exit_code:0,..} => block`) after the architect narrowed
ADR-039's authoritative §AMD-003 rule to its correct non-zero-exit-`Ok`-only form in ADR-039
v1.13 / Erratum E-005. PC13's own body already asserted the correct narrow form
(`Ok{exit_code,..} where exit_code!=0`) and was untouched. Both sites rewritten to the narrow
form with explicit MUST-NOT-be-a-negation guidance and a restatement that the base
`Crashed | Timeout` rule remains governed solely by `on_error`, unchanged, and that a `Timeout`
blocking under `failure_policy=FailOpen` is decided exclusively by the `failure_policy` axis
(PC5/EC-009/Invariant 1), never by `on_error` alone. Verified PC5, PC10(a), EC-009, and
Invariant 1 remain internally consistent — the BC now speaks ONE predicate. Sibling-swept
BC-1.03.018 (TD-VSDD-060): no occurrence of the broad-negation pattern found; no edit needed.
No AC in this BC restates the broad predicate (this BC has no Acceptance Criteria section).
POLICY 8 parity preserved: no change to frontmatter `behavioral_contracts`/`bcs` arrays, body
BC-table, ACs, or Token Budget. Does NOT touch ADR-039 (architect's domain, already corrected),
S-21.11 story body/ACs (story-writer's domain), any INDEX file, or input-hashes (state-manager's
domain — input-hash drift against ADR-039 v1.13's own content bump is expected, reconciled
same-burst). BC-1.03.017 v1.16. [Prior: 2026-08-19 (v1.15) — S-21.11 v2.2 pre-TDD spec-convergence
remediation of F-S2111V2-P2-004 (MEDIUM, human-directed production-grade full-coverage decision): PC13's
`on_error=Block` + `Ok{exit_code!=0}` fail-closed rule was correctly written as a generic,
plugin-name-independent predicate, but the BC's surrounding six-plugin-scoped enumerations
(Preconditions 2/3, PC8/PC9, Invariants 7/8, Architecture Anchors) created an under-specified
reading that PC13's coverage obligation was limited to the same six PC1–PC12
exhaustion-migration-targeted plugins. Verified the registry (`plugins/vsdd-factory/hooks-registry.toml`)
directly: 18 `on_error = \"block\"` `[[hooks]]` entries exist across 17 unique plugin names
(`protect-secrets` registered twice, once per `tool = \"^Bash$\"` and once per `tool = \"^Read$\"`)
— confirms the human-cited '18' figure exactly, no discrepancy. Added a new PC13 \"Coverage
Set\" enumeration (all 18 entries, tabulated with event/tool trigger) establishing that PC13
applies to the FULL registry on_error=Block set, not a sampled subset, with the registry named
as the authoritative source for future drift re-verification. Added new Invariant 11 codifying
the full-registry-coverage rule as entry-count-agnostic (future on_error=block entries inherit
PC13 automatically). Added one Canonical Test Vector summary row citing full-coverage. Added
one Architecture Anchor bullet citing the 18-entry registry coverage set. H1 enriched with a
full-registry-coverage clause per POLICY 7. No PC renumbering; additive-only within PC13's
existing prose plus one new Invariant. Does NOT touch BC-1.03.018 (materially unaffected — the
break-glass control is a distinct enforcement point), the S-21.11 story body/ACs (story-writer's
domain, dispatched separately), ADR-039 (architect's domain), or BC-INDEX.md (state-manager's
domain). BC-1.03.017 v1.15. [Prior: 2026-08-19 (v1.14) — Product-owner self-flagged residual-inconsistency remediation: PC12's POSITIVE control (and its 'AMD-002-wiring-fixed' Canonical Test Vector row) previously asserted the wiring fix's target outcome as `PluginResult::Timeout { cause: Epoch }` — WRONG, contradicted by this BC's own v1.13-ratified PC13(a)/EC-011 mechanism trace (ADR-039 §AMD-003): an `exec_subprocess.rs::run()` wall-clock kill propagates via `run_bash_via_host`'s error map -> `adapter_logic`'s `HookResult::Error` -> `classify_trap`'s `Err(I32Exit(1))` arm -> `PluginResult::Ok { exit_code: 1, .. }`, NEVER `Timeout { cause: Epoch }` (that variant is constructed only by `classify_trap` on a genuine guest `Trap::Interrupt`, which cannot fire while the guest is blocked inside the synchronous `exec_subprocess` host call). Corrected PC12's POSITIVE control prose and its Canonical Test Vector row to assert `PluginResult::Ok { exit_code: 1, .. }` (via `HookResult::Error`) at the observed ~`timeout_ms` + ~5ms-poll-tolerance kill time, with an explicit NEVER-`Timeout{cause: Epoch}` clause. Clarified PC12/PC13 as complementary, non-redundant guarantees: PC12 owns the kill-timing/wiring guarantee; PC13 owns the block-enforcement guarantee for the resulting `Ok { exit_code != 0 }` outcome. No PC renumbering; additive/corrective only within PC12's existing prose and test-vector row. Does NOT touch BC-1.03.018, the S-21.11 story, ADR files, ARCH-INDEX, or STORY-INDEX. BC-1.03.017 v1.14. [Prior: 2026-08-19 (v1.13) — S-21.11 v2.0 adversarial pass-1 remediation (product-owner; F-S2111V2-P1-001-mechanism-adjudication memo, ADR-039 §AMD-003 RATIFIED v1.11): new PC13 asserts the §AMD-003 rule — a fail-closed-eligible plugin (`on_error = OnError::Block`) whose outcome is `PluginResult::Ok { exit_code != 0, .. }` MUST be treated as a block (`block_intent = true`, exit 2), regardless of `failure_policy`; covers both a `legacy-bash-adapter.wasm` host-wall-clock timeout surfacing as `HookResult::Error` -> exit 1 (F-001) and any other generic `HookResult::Error` exit path (F-005, ruled in-scope). Includes POSITIVE control (`on_error=Block` + `Ok{exit!=0}` -> block) and two NEGATIVE controls (`on_error=Block` + `Ok{exit==0}` -> no block; `on_error=Continue` + `Ok{exit!=0}` -> unaffected). New Invariant 10 codifies PC13 as a strict superset of the pre-existing `Crashed | Timeout` rule. Traceability ADR row extended to cite ADR-039 §AMD-003 alongside §AMD-001/§AMD-002. EC-011 corrected (F-002): the prior 'silent false clean-pass at 45s' pre-fix characterization was wrong — the pre-fix outcome is nondeterministic (`PluginResult::Ok{exit_code:1}` via `HookResult::Error`, OR a guest-epoch `Timeout` race on control-return), not a deterministic clean pass; post-fix, PC12 (kill timing) + PC13 (`Ok{exit!=0}` -> block) together close every sub-case. Three new PC13 Canonical Test Vector rows added. Architecture Anchors extended to cite `crates/hook-sdk/src/result.rs::HookResult::exit_code` and the PC13 decision-site extension in `executor.rs`. H1 enriched with PC13's clause per POLICY 7. PC count extended PC1-PC12 -> PC1-PC13 (additive-only; no renumbering). Scope note: this burst does NOT touch S-21.11's story body/ACs (story-writer's domain) and does NOT alter PC1-PC12's existing semantics. BC-1.03.017 v1.13. [Prior: 2026-08-19 (v1.12) — S-21.11 expanded-scope BC coverage burst (product-owner; scoped to the AMD-002 runtime-wiring gap only, orchestrator directive): new PC12 asserts the RUNTIME behavior AMD-002 (RATIFIED v1.10) identified as unwired — for `legacy-bash-adapter.wasm`-hosted plugins, the actual bash-subprocess kill deadline (`exec_subprocess.rs::run()`) MUST equal the registry's calibrated `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS=60_000` constant in `run_bash_via_host`; includes a POSITIVE control (short `timeout_ms` kills early) and a NEGATIVE reference documenting the current pre-fix 60s-regardless-of-config defect state, plus a highest-risk EC-011 (script duration between calibrated `timeout_ms` and the hardcoded 60s produces a silent false clean-pass under the current implementation). New Precondition 5 states the config-vs-runtime assumption gap explicitly. PC9 amended (additive) with a PC12-dependency clause: registry-config completeness (fuel_cap/timeout_ms set) is necessary but, per §AMD-002, not alone sufficient for the five bash-adapter plugins to be treated as fully protective. New Invariant 9 codifies the config-vs-runtime wiring bifurcation. Two new Canonical Test Vector rows (PC12 POSITIVE/NEGATIVE) plus one EC-011 vector added. Architecture Anchors extended to cite `legacy-bash-adapter/src/lib.rs::run_bash_via_host` and `exec_subprocess.rs::run()`'s 5ms poll loop (explicitly distinguished from the unrelated wasmtime `EPOCH_TICK_MS`=10ms guest-epoch ticker per ADR-039 §Decision 4 v1.9 mechanism-precision correction). Traceability `L2 Capability` resolved from placeholder `CAP-TBD` to `CAP-011` (\"Enforce fuel and epoch budgets on plugin execution\") with a new S-7.01 Capability Anchor Justification row added (capabilities.md §CAP-011 verbatim cite) — this BC's enforcement-dispatch scope, extended by PC12 to the bash-adapter wiring's runtime correctness, is squarely CAP-011's 'a runaway plugin is killed within timeout_ms...never hung processes' outcome. PC count extended PC1-PC11 -> PC1-PC12 (additive-only; no renumbering). Scope note: this burst does NOT touch S-21.11's story body/ACs (story-writer's domain, dispatched separately) and does NOT alter PC1-PC11's existing semantics. BC-1.03.017 v1.12. [Prior: 2026-08-19 (v1.11) — Sibling-sweep citation update (architect; TD-VSDD-060; parallel to ADR-039 v1.9->v1.10, same burst): Traceability row's ADR citation updated — §AMD-002 now cites RATIFIED (2026-08-19, v1.10, POLICY 22) instead of PROPOSED/NOT RATIFIED, with the corrected corroboration basis (ADR-039's own v1.8 §AMD-001 -> v1.9 §Decision 4 mechanism-precision self-correction, not the retracted ADR-025 §Decision 18 citation); §Decision 3's break-glass citation redirected from named follow-up S-21.17 to S-21.11 (absorbed, no-split human decision); AMD-002's named follow-up S-21.18 likewise redirected to S-21.11. Stories row unaffected (already cites S-21.11). Citation-only sweep: no PC/Precondition/Invariant content altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.11. [Prior: 2026-08-18 (v1.10) — F-S2111-P13-001 research-corrections fold-in (architect; parallel terminology sweep + AMD-002 cite, ADR-039 v1.9): swept 'epoch axis'/'epoch-axis floor'/'epoch mechanism' prose terminology to 'host-wall-clock-timeout axis' throughout Preconditions 2/3, PC8, PC9, Canonical Test Vectors, Architecture Anchors, VP-TBD, and Traceability — `timeout_ms` field name and `TimeoutCause::Epoch`/`Timeout{Epoch}` Rust code identifiers unchanged (literal code, not prose). Architecture Anchors + VP-TBD + Traceability updated to additionally cite ADR-039 §AMD-002 (PROPOSED/NOT RATIFIED architect self-verification finding: `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed 60,000 ms constant independent of the registry's calibrated `timeout_ms`; named follow-up S-21.18, new, not yet authored) alongside the now-RATIFIED §AMD-001. No PC/Precondition/Invariant semantics altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.10. [Prior: 2026-08-18 (v1.9) — F-S2111-P13-001 remediation (architect; scoped architectural precondition/PC correction; ADR-039 §Decision 1/2/3/4 v1.8 amendment): Precondition 2/3 bifurcated by plugin adapter class — the native-WASM plugin (validate-cross-site-correspondence) calibrates fuel_cap per the original formula; the five legacy-bash-adapter.wasm-hosted plugins (validate-factory-path-root, validate-input-hash, validate-template-compliance, validate-wave-gate-prerequisite, validate-pr-merge-prerequisites) additionally calibrate timeout_ms per the new epoch-axis formula (timeout_ms >= max(measured_p99_ms x 2.0, 30_000)) because their bash subprocess execution is invisible to the WASM fuel counter (ADR-042 §Decision 3 class (b)). PC8 extended with a parallel timeout_ms structural half-state assertion (POSITIVE/NEGATIVE controls added) for legacy-bash-adapter.wasm entries — fuel_cap sufficiency alone is no longer treated as complete calibration evidence for these five plugins. PC9 final-state criterion updated to require both axes per plugin's adapter class. New Invariant 8 codifies the axis-bifurcation principle. Two new Canonical Test Vector rows added (PC8 timeout_ms POSITIVE/NEGATIVE controls). Architecture Anchors + VP-TBD + Traceability updated to cite ADR-039 v1.8 §AMD-001. PC count unchanged at PC1..PC11 (no renumbering); this is additive-only within existing PCs plus one new Invariant. Residual product-owner BC-body edit noted: this burst does NOT touch AC-to-PC narrative mapping in the S-21.11 story body (deferred to post-ratification resume burst per orchestrator scoping) and does NOT alter PC1-PC7/PC10/PC11's axes-independence or migration-window substance, which remain product-owner's domain if further narrative refinement is needed. BC-1.03.017 v1.9. [Prior: 2026-08-18 (v1.8) — F-S2111-P11-001 remediation (product-owner): extended PC10 to require deliberate revision (TD-VSDD-059) of BOTH the unit test fail_closed_timeout_with_on_error_block AND its integration-level mirror test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block (TC-12, full_stack_plugin_invocation.rs); TC-12 currently asserts exit_code==2 for on_error=Block+failure_policy=FailOpen+Timeout{Epoch} — false under axes-independent semantics (PC5/EC-009); must be revised to assert exit 0, with a SHOULD arm for failure_policy=FailClosed→exit 2 (Invariant 6 / Envoy #38801 symmetric coverage); two TC-12 Canonical Test Vector rows added; Architecture Anchors updated to cite TC-12; VP-TBD updated. BC-1.03.017 v1.8.]]]]]]]]]]"
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/architecture/decisions/ADR-042-validate-cross-site-correspondence-fuel-budget-raise-and-loud-exhaustion-signaling.md
  - .factory/specs/architecture/decisions/ADR-044-split-topology-enforcement-flip-capstone-ownership.md
  - .factory/research/wasm-fuel-exhaustion-detection.md
input-hash: "1c300e8"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-011"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill-E21-W6
modified:
  - "2026-08-17 (v1.1)"
  - "2026-08-17 (v1.2)"
  - "2026-08-17 (v1.3)"
  - "2026-08-17 (v1.4)"
  - "2026-08-17 (v1.5)"
  - "2026-08-17 (v1.6)"
  - "2026-08-18 (v1.7)"
  - "2026-08-18 (v1.8)"
  - "2026-08-18 (v1.9)"
  - "2026-08-18 (v1.10)"
  - "2026-08-19 (v1.11)"
  - "2026-08-19 (v1.12)"
  - "2026-08-19 (v1.13)"
  - "2026-08-19 (v1.14)"
  - "2026-08-19 (v1.15)"
  - "2026-08-19 (v1.16)"
  - "2026-08-19 (v1.17)"
  - "2026-08-20 (v1.18)"
  - "2026-08-20 (v1.19)"
  - "2026-08-22 (v1.20)"
  - "2026-08-22 (v1.21)"
  - "2026-08-22 (v1.22)"
  - "2026-08-23 (v1.23)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.017: factory-dispatcher::executor::failure_policy enforcement — exhaustion-outcome dispatch (fail-closed→block; fail-open→advisory), on_error axes independence, crash-versus-exhaustion distinct paths, Phase-3-before-Phase-4 structural half-state gate, migration-window on_error=block completeness gate, legacy-bash-adapter runtime-timeout-wiring verification with CI-jitter-robust kill-time margin, ceil(observed_max×1.5) fuel_cap calibration-sufficiency gate against a FROZEN corpus snapshot (ceil() adopted F-S2122-P4-001 — rounds up to nearest whole fuel unit, prevents boundary-case truncation below formula target), and wave-7→8 migration coverage-continuity invariant (ADR-039 §Decision 3+6 Phase 4 enforcement leg + §AMD-002 wiring leg + §AMD-003 on_error=Block plugin-error fail-closed leg + §Decision 4 v1.16/§Erratum E-007 target-statistic correction; PC11 CWE-636 static gate; PC12 AMD-002 runtime gate — one-sided kill-time floor, hundreds-of-ms observed-margin bound, not the ~5ms internal poll granularity; PC13 AMD-003 plugin-error-exit fail-closed gate covering the FULL 18-entry on_error=Block registry set, not a 6-plugin sample; Precondition 6 machine-checkable sufficiency regression assertion — standing gate runs against a FROZEN `pc6-sufficiency-snapshot/`, not live-growing corpus files; PC6 S-21.22 ownership explicit: Task 4 one-time calibration confirmation AND Task 5a durable standing CI gate BOTH OWNED BY S-21.22 (F-S2121-P4-002); EC-011 enforcement sub-case-distinct: `Ok{exit_code:1}` pre-§AMD-003 NOT blocked, `Timeout{Epoch}` IS blocked via retained 2-arg call per Invariant 12 — timing-only defect (F-S2121-P4-001); decision function split per ADR-044 v1.1/v1.3 into `plugin_fail_closed_on_exhaustion` (S-21.24-owned wiring) and `plugin_fail_closed_on_error_exit` (S-21.21-owned ADDITIVE wiring, alongside the retained 2-arg call, not replacing it); Invariant 12 migration coverage-continuity, ADR-044 v1.3)

## Description

The factory-dispatcher executor MUST extend the `plugin_fail_closed` function (or introduce a
replacement) in `crates/factory-dispatcher/src/executor.rs` to consult the `failure_policy`
field (introduced by BC-1.01.016 / S-21.10) when handling resource-exhaustion outcomes
(`PluginResult::Timeout { cause: TimeoutCause::Fuel }` and `TimeoutCause::Epoch`).

**The enforcement defect being closed:** In the current implementation, `plugin_fail_closed`
returns `false` when `on_error == OnError::Continue`, regardless of `TimeoutCause`. For the
approximately 38 validator plugins registered with `on_error = "continue"`, fuel exhaustion
therefore produces the same allow-decision as a clean pass. This is **CWE-636 "Not Failing
Securely (Failing Open)"** (primary) and **CWE-390 "Detection of Error Condition Without
Action"** (secondary), as classified by `.factory/research/wasm-fuel-exhaustion-detection.md`.
The production-host precedent (Envoy `FailurePolicy` default `FAIL_CLOSED`; Istio `failStrategy`
default `FAIL_CLOSE`) confirms that authorization-class plugins should block on exhaustion.

**The self-lock hazard is already live today.** During ADR-039 authoring (2026-08-06), writes
to `ARCH-INDEX.md` triggered `fail-closed: plugin timed out` blocks from
`validate-factory-path-root`, `validate-input-hash`, and `validate-template-compliance`. Any
premature enforcement flip without calibrated per-plugin fuel caps would hard-block all
`.factory/` writes. The Phase-3-before-Phase-4 ordering constraint in this BC (PC8, PC9) is
therefore a correctness requirement, not a best-practice recommendation.

**Enforcement semantics:** When `failure_policy = FailClosed`, a fuel- or epoch-exhausted
plugin MUST produce a block signal (exit code 2) regardless of the value of `on_error`. When
`failure_policy = FailOpen`, a fuel- or epoch-exhausted plugin MUST produce an advisory event
(exit code 0). The `on_error` axis remains governing for crash outcomes only — it does NOT
override `failure_policy` for exhaustion outcomes.

**The Envoy #38801 lesson is binding:** The test suite MUST assert observed outcomes (block or
advisory at the dispatcher level), NOT merely that the `failure_policy` field is configured.
Envoy documented `FAIL_CLOSED` diverged from observed behavior because the test suite asserted
configuration intent rather than behavioral outcomes.

## Preconditions

1. S-21.10 has shipped: `FailurePolicy` enum and `RegistryEntry.failure_policy` field are present
   in `crates/factory-dispatcher/src/registry.rs` (BC-1.01.016 postconditions hold).
2. Per-plugin calibration (devops-engineer role) has been executed for each of the six
   targeted validator-class plugins, bifurcated by plugin adapter class per ADR-039 §Decision
   1/3/4 (v1.8 amendment; §AMD-001):
   - **Native-WASM plugin — fuel-axis calibration:** `validate-cross-site-correspondence`
     (hosted by its own `hook-plugins/validate-cross-site-correspondence.wasm` binary).
     `fuel_consumed` is measured against the calibration corpus below.
   - **`legacy-bash-adapter.wasm`-hosted plugins — host-wall-clock-timeout-axis calibration
     ADDITIONALLY required (fuel-axis calibration alone is insufficient):** `validate-factory-path-root`,
     `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
     `validate-pr-merge-prerequisites`. Their bash subprocess execution is invisible to the
     WASM fuel counter (fuel exhaustion, if any, occurs before the WASI `exec_subprocess`
     call per ADR-042 §Decision 3 class (b)); their actual resource-exhaustion axis is the
     host-enforced wall-clock deadline. `time_consumed_ms` (bash subprocess wall-clock
     duration) is measured against the same calibration corpus, in ADDITION to (not instead
     of) `fuel_consumed` for the adapter's own marshaling step.

   Calibration corpus MUST include: `lessons.md` at ≥4000 lines; `STATE.md` at current live
   size; `decision-log.md` at current live size; and the 576,396-byte production-scale
   fixture at
   `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`.
   The same corpus backs both the fuel-axis and host-wall-clock-timeout-axis measurements — only the metric
   collected differs by adapter class.
3. For each targeted plugin, the calibrated value has been set per its adapter class from
   precondition 2 measurements (**target-statistic correction, ADR-039 §Decision 4 v1.16 /
   §Erratum E-007:** the load-bearing calibration TARGET for the `fuel_cap` axis, for all six
   §Decision 2 fail-closed validators without exception, is `observed_max` — the observed
   maximum `fuel_consumed` measured over the mandated calibration corpus — NOT `measured_p99`;
   `measured_p99 × 1.5` remains a valid Phase-1 schema-parsing floor at the field-parser level
   ONLY and MUST NOT be treated as calibration-sufficient evidence for any of the six. This
   correction does NOT extend to the `timeout_ms` axis, which remains `measured_p99_ms`-derived
   per ADR-039 §Decision 4's unchanged wall-clock-nondeterminism rationale):
   - Native-WASM plugin: `fuel_cap` set to `max(ceil(observed_max × 1.5), 50_000_000)`.
     `ceil()` rounds up to the nearest whole fuel unit, preventing boundary-case truncation
     below the formula target when `observed_max × 1.5` yields a non-integer (F-S2122-P4-001,
     ceil() adoption, Wave-7 pass-4 remediation — strictly more conservative than truncation).
     `50_000_000` is an INCLUSIVE FLOOR beneath the `ceil(observed_max × 1.5)` target, not
     itself the calibration target — a plugin whose `ceil(observed_max × 1.5)` exceeds
     `50_000_000` MUST calibrate to the higher value; clearing the floor alone does not satisfy
     this Precondition (see Precondition 6 for the machine-checkable sufficiency assertion this
     distinction backs).
   - `legacy-bash-adapter.wasm`-hosted plugins: `fuel_cap` set to
     `max(ceil(observed_max × 1.5), 50_000_000)` (same ceil() adoption, target-statistic
     correction, and floor-not-target
     framing as above) for the adapter's marshaling step AND `timeout_ms` set to
     `max(measured_p99_ms × 2.0, 30_000)` for the bash subprocess wall-clock budget (ADR-039
     §Decision 4 host-wall-clock-timeout-axis formula, v1.8 — UNCHANGED: wall-clock duration is
     non-deterministic across CI runners/developer machines in a way fuel consumption (WASM
     instruction count) is not, so `measured_p99_ms × 2.0`, not `observed_max_ms`, remains the
     correct, deliberately more conservative `timeout_ms` target). Both fields MUST be set;
     neither substitutes for the other.
4. The calibration results (plugin name, p99 measured, chosen `fuel_cap`) are recorded in the
   PR description or a calibration log artifact before Phase 4 annotations land.
5. **AMD-002 runtime-wiring precondition (assumption underlying PC1/PC6/PC9 for
   `legacy-bash-adapter.wasm`-hosted plugins; ADR-039 §AMD-002, RATIFIED v1.10):**
   Preconditions 2/3's calibrated `timeout_ms` value is a REGISTRY-CONFIG assumption only.
   PC1/PC6's enforcement decision (`failure_policy=FailClosed` → block on
   `Timeout{cause: Epoch}`) implicitly assumes the `Timeout{cause: Epoch}` outcome itself
   fires at the calibrated `timeout_ms`, not at some unrelated value. For
   `legacy-bash-adapter.wasm`-hosted plugins, this assumption does NOT hold until the
   AMD-002 wiring fix lands (PC12): prior to the fix, the adapter's bash-subprocess kill
   deadline is a fixed `BASH_TIMEOUT_MS = 60_000` constant
   (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs::run_bash_via_host`), independent of
   the registry's calibrated `timeout_ms` fed to `exec_subprocess.rs::run()`. PC9's
   final-state completeness assertion for the five `legacy-bash-adapter.wasm`-hosted plugins
   is registry-complete but NOT runtime-complete until PC12 additionally holds.
6. **Calibration-sufficiency precondition — machine-checkable, not merely the 50M floor
   (Q2, F-S2122-P1-002; ADR-039 §Decision 4 v1.16 / §Erratum E-007; applies to ALL SIX
   ADR-039 §Decision 2 fail-closed validators, both adapter classes):** Precondition 3's
   `fuel_cap` calibration is complete for a given targeted plugin ONLY IF the annotated
   `fuel_cap` satisfies `fuel_cap >= ceil(observed_max × 1.5)`, where `observed_max` is
   measured against the mandated calibration corpus (Precondition 2: `lessons.md` at ≥4000
   lines; `STATE.md` at current live size; `decision-log.md` at current live size; and the
   ≥574 KB (576,396-byte) production-scale fixture at
   `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`,
   per the S-21.22 calibration-corpus mandate) — NOT merely `fuel_cap >= 50_000_000`. The
   50,000,000 floor (Precondition 3, Invariant 2, PC8) is a necessary schema-level minimum;
   it is NOT independently sufficient evidence of calibration for any plugin whose
   `ceil(observed_max × 1.5)` exceeds it.
   **S-21.22 OWNERSHIP (F-S2121-P4-002, PC6 standing-gate ownership, Wave-7 pass-4
   remediation):** Both checks below are OWNED BY S-21.22:
   (i) the one-time live-corpus calibration confirmation (Task 4) and (ii) the durable
   standing CI regression assertion (Task 5a). S-21.22 is the authoritative delivery vehicle
   for both. Prior text described the mechanisms without explicitly naming S-21.22 as the
   owning story.
   **TWO distinct checks — frozen-vs-live split (F-S2122-P3-001 correction, Wave-7 pass-3
   remediation; matches S-21.22's converged Task 5a mechanism; do not conflate):**
   (i) **One-time live-corpus calibration CONFIRMATION (Task 4, at annotation time):** the
       initial `observed_max` measurement that DERIVES the `fuel_cap` value to annotate MAY
       be taken against the live calibration corpus files (`lessons.md`, `STATE.md`,
       `decision-log.md` at their size AT CALIBRATION TIME, plus the fixed ≥574 KB
       `BC-INDEX.md` fixture) as they stand when the calibration is performed. This is a
       point-in-time measurement whose purpose is to DERIVE the annotated `fuel_cap` value —
       it is NOT the durable, standing gate.
   (ii) **DURABLE, standing CI regression assertion (Task 5a) — the MANDATED form for the
        standing gate:** the machine-checkable sufficiency gate that runs on every
        CI-passing commit going forward MUST run against a FROZEN corpus SNAPSHOT
        (`pc6-sufficiency-snapshot/`), captured once at calibration time and committed as a
        fixed fixture — NOT the live-growing files. Running the standing assertion against
        live `decision-log.md`/`STATE.md`/`lessons.md` is a FALSE-FAIL TIME BOMB: as these
        files organically grow over the project's lifetime (independent of any actual
        regression in the plugin's fuel efficiency), `fuel_consumed` measured against the
        live files would climb and could exceed `registry fuel_cap ÷ 1.5`, causing the
        standing gate to fail RED with no corresponding defect — eroding trust in the gate
        until it is disabled or ignored, the opposite of the production-grade discipline
        this Precondition exists to enforce. The frozen snapshot decouples the standing
        assertion from organic corpus growth: captured once, alongside the calibration that
        derives the annotated `fuel_cap`, the standing gate re-verifies
        `fuel_consumed(pc6-sufficiency-snapshot/) × 1.5 <= registry fuel_cap` — a stable,
        reproducible check that fails only when the PLUGIN itself regresses (e.g. an
        inefficient code change), never merely because the live corpus grew.
   **Machine-checkable, not assertion-only (mandated form for the standing gate):** this
   sufficiency requirement MUST be enforced by a regression assertion against the FROZEN
   `pc6-sufficiency-snapshot/` fixture set, not documented as a one-time calibration note and
   not re-run against live-growing files: re-run the plugin against the frozen snapshot and
   assert `fuel_consumed × 1.5 <= registry fuel_cap` for the snapshot's observed
   `fuel_consumed` reading. A calibration record (PR description or artifact) that states a
   `fuel_cap` value without a corresponding passing FROZEN-snapshot regression assertion is
   NOT calibration-sufficient evidence — this closes the gap where a plugin clears the fixed
   50M floor (PC8's structural gate) while its `ceil(observed_max × 1.5)` against the frozen
   snapshot silently exceeds the annotated cap, leaving the fail-closed flip under-calibrated
   against its own governing formula.
   **If `ceil(observed_max × 1.5) >= PRACTICAL_FUEL_CEILING` (`500_000_000`, ADR-039 §Decision 4
   v1.16):** the plugin MUST NOT receive `failure_policy = "fail-closed"` until a structural
   remedy exists (ADR-039 §Decision 4 Option B, or a validator-specific input-size-reduction
   mitigation) — this is the same `PRACTICAL_FUEL_CEILING` trigger EC-004 already routes
   through; Precondition 6 is the sufficiency check that determines when EC-004 fires, not a
   competing rule. This `>=`/`PRACTICAL_FUEL_CEILING` trigger is evaluated against whichever
   `observed_max` measurement is current at the time (i) or (ii) is performed — the
   frozen-vs-live split above governs WHICH corpus backs the measurement, not this trigger's
   own semantics, which are unchanged.

## Postconditions

1. **PC1 — Exhaustion + fail-closed → BLOCK (exit 2):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with
   `failure_policy = FailurePolicy::FailClosed` causes the executor decision function to return
   `true` (block intent); the dispatcher exit code is 2. A real dispatch with a plugin
   configured `failure_policy = "fail-closed"` and `fuel_cap = 100` (deliberately too small)
   on a payload that exhausts the budget MUST produce exit code 2 on the observed dispatcher
   output.

2. **PC2 — Exhaustion + fail-open → advisory (exit 0):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with
   `failure_policy = FailurePolicy::FailOpen` causes the executor decision function to return
   `false`; the dispatcher exit code is 0; an advisory event is emitted (not a block). This
   verifies the `fail-open` path is preserved for plugins that legitimately require it.

3. **PC3 — `on_error` independence: exhaustion + fail-closed blocks regardless of `on_error`:**
   A plugin with `on_error = OnError::Continue` AND `failure_policy = FailurePolicy::FailClosed`
   that exhausts its fuel budget MUST produce a block (exit 2). `on_error` governs crash
   outcomes only; it does NOT override `failure_policy` for exhaustion outcomes. This directly
   validates the axes-independence design and supersedes the prior
   `fail_closed_timeout_with_on_error_continue_is_open` assertion for the `FailClosed`
   configuration case.

4. **PC4 — Crash governed exclusively by `on_error` (crash ≠ exhaustion, path A):**
   `PluginResult::Crashed` (crash) with `on_error = OnError::Block` and
   `failure_policy = FailurePolicy::FailOpen` MUST produce a block (exit 2) via the `on_error`
   path. The block is caused by the crash, not by exhaustion policy. `failure_policy` is not
   consulted for crash outcomes.

5. **PC5 — `on_error = block` does NOT gate exhaustion when `failure_policy = fail-open`
   (crash ≠ exhaustion, path B):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with `on_error = OnError::Block` and
   `failure_policy = FailurePolicy::FailOpen` MUST produce exit 0. Exhaustion is a
   resource-policy outcome; `on_error = block` does not apply to exhaustion when
   `failure_policy = FailOpen`.

6. **PC6 — Epoch exhaustion treated identically to fuel exhaustion:**
   `PluginResult::Timeout { cause: TimeoutCause::Epoch }` with
   `failure_policy = FailurePolicy::FailClosed` MUST produce a block (exit 2). Both
   `TimeoutCause::Fuel` and `TimeoutCause::Epoch` are resource-exhaustion outcomes per
   ADR-039 §Decision 1; both trigger the `failure_policy` enforcement path.

7. **PC7 — `fail_closed_timeout_with_on_error_continue_is_open` revised, not deleted
   (TD-VSDD-059 compliance):**
   The existing test `fail_closed_timeout_with_on_error_continue_is_open` in the executor
   module MUST be revised (not deleted) to assert the new invariant for the `fail-open`
   configuration case: `Timeout { cause: Fuel } + on_error=Continue + failure_policy=FailOpen
   → NOT block`. The function name MUST be retained or a close derivative used. The revised
   test MUST appear in the PR diff. Deletion without an equivalent replacement is a TD-VSDD-059
   paper-fix violation.

8. **PC8 — Structural half-state gate: no `failure_policy = "fail-closed"` with uncalibrated
   `fuel_cap`; both positive and negative gate controls present (standing regression/invariant
   gate; migration-window on_error=block ordering constraint is mechanically enforced by PC11,
   not by this gate):**
   A Cargo integration test (`test_no_fail_closed_plugin_with_uncalibrated_cap`) MUST assert
   that no `[[hook]]` entry in `hooks-registry.toml` carries both
   `failure_policy = "fail-closed"` AND `fuel_cap < 50_000_000` (the calibration floor per
   ADR-039 §Decision 4 is 50_000_000 inclusive — `fuel_cap >= 50_000_000` is VALID; the
   factory default of 20_000_000 per ADR-042 §Decision 2 is below this floor and therefore
   insufficient for fail-closed annotation). This gate
   is a **standing regression/invariant gate**: it is GREEN when the registry contains zero
   fail-closed entries (vacuously satisfied with the empty set), GREEN at final state (all
   targeted plugins annotated with calibrated caps), and RED only when a bad half-state edit
   introduces a fail-closed entry without a sufficient cap. The gate MUST include both:
   (a) **POSITIVE-CONTROL fixture** (a hard-coded fail-closed entry with `fuel_cap = 20_000_000`
   — the factory default per ADR-042 §Decision 2, strictly below the 50_000_000 inclusive floor
   and therefore a realistic failing case — injected directly in the test body) that asserts the
   gate fires RED on that fixture — proving the gate is non-vacuous and not susceptible to
   false-green behavior when the live registry contains zero fail-closed entries; and
   (b) **NEGATIVE-CONTROL fixture** (a hard-coded fail-closed entry with `fuel_cap = 75_000_000`,
   i.e., above the 50_000_000 floor, injected directly in the test body) that asserts the gate
   does NOT fire on that fixture (result: PASS / no error) — proving the gate correctly
   distinguishes valid calibrated fail-closed entries from bad half-state entries (POLICY 15:
   every gate outcome requires a control; the positive-control-only version leaves the
   "gate accepts valid entry" path unverified).

   **Parallel host-wall-clock-timeout-axis assertion for `legacy-bash-adapter.wasm`-hosted entries (F-S2111-P13-001;
   ADR-039 §Decision 1/4 v1.8 amendment — fuel-axis calibration is necessary but NOT sufficient
   for these entries):** The same test MUST ALSO assert that no `[[hook]]` entry whose
   `plugin = "hook-plugins/legacy-bash-adapter.wasm"` carries both `failure_policy = "fail-closed"`
   AND `timeout_ms < 30_000` (the host-wall-clock-timeout-axis calibration floor per ADR-039 §Decision 4 v1.8
   formula: `max(measured_p99_ms × 2.0, 30_000)`; exactly `30_000` is the inclusive minimum).
   This assertion is IN ADDITION to the `fuel_cap` assertion above, not a replacement — a
   `legacy-bash-adapter.wasm`-hosted entry satisfying `fuel_cap ≥ 50_000_000` alone remains
   half-state and MUST still fail this gate if `timeout_ms < 30_000`, because `fuel_cap` gives
   no protection against that adapter class's actual exhaustion axis (the bash subprocess is
   invisible to the WASM fuel counter). The host-wall-clock-timeout-axis assertion likewise requires both
   controls:
   (c) **TIMEOUT-POSITIVE-CONTROL fixture** (a hard-coded `legacy-bash-adapter.wasm` entry with
   `failure_policy = "fail-closed"` and `timeout_ms = 10_000`, i.e., the current live default
   for four of the five targeted bash-adapter plugins and strictly below the 30_000 floor,
   injected directly in the test body) that asserts the gate fires RED on that fixture.
   (d) **TIMEOUT-NEGATIVE-CONTROL fixture** (a hard-coded `legacy-bash-adapter.wasm` entry with
   `failure_policy = "fail-closed"` and `timeout_ms = 45_000`, i.e., above the 30_000 floor,
   injected directly in the test body) that asserts the gate does NOT fire on that fixture
   (result: PASS / no error).
   The genuine red-first TDD gate is PC9
   (AC-009: `test_all_six_validator_class_plugins_are_fail_closed`), which is RED before Phase
   4 annotations land and GREEN only after all targeted plugins carry fail-closed with calibrated
   caps.
   **Symmetric half-state prohibition (F-S2111-P2-001):** Once the extended 3-arg
   `plugin_fail_closed` function is present in any CI-passing commit, no targeted plugin
   currently carrying `on_error = "block"` (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`) MAY remain at `failure_policy = fail-open`. Under the
   2-arg function these five plugins block on exhaustion via the `on_error = "block"` path;
   under the extended function exhaustion is governed exclusively by `failure_policy`, so
   failure_policy=fail-open causes them to FAIL OPEN — a CWE-636 regression. The
   decision-function change and the fail-closed annotations for these five plugins MUST be
   co-committed (same commit) or ordered annotate-first-then-flip. The mechanical CI gate
   enforcing this ordering constraint is PC11
   (`test_no_on_error_block_without_fail_closed_when_3arg_executor`), not this gate; PC8's
   test asserts only the calibration constraint (no fail-closed without `fuel_cap >= 50M`).
   A plugin entry MUST NOT carry `failure_policy = "fail-closed"` without simultaneously
   carrying `fuel_cap >= 50_000_000` (exactly 50_000_000 is the inclusive floor and a VALID
   calibrated value per ADR-039 §Decision 4).

9. **PC9 — All targeted validator-class plugins carry `failure_policy = "fail-closed"` with
   calibration sufficient for their adapter class's actual exhaustion axis in final state
   (bifurcated per ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001):**
   After all Phase 4 calibration-and-annotation commits land, `hooks-registry.toml` MUST
   contain `failure_policy = "fail-closed"` for all plugins in the **post-amendment targeted
   set**, AND each plugin's calibrated field(s) MUST satisfy its adapter class's requirement:
   the native-WASM plugin (`validate-cross-site-correspondence`) MUST carry
   `fuel_cap >= 50_000_000` (inclusive floor; see Precondition 6 for the load-bearing
   `ceil(observed_max × 1.5)` sufficiency target this floor sits beneath); each
   `legacy-bash-adapter.wasm`-hosted plugin (`validate-factory-path-root`, `validate-input-hash`,
   `validate-template-compliance`, `validate-wave-gate-prerequisite`,
   `validate-pr-merge-prerequisites`) MUST carry BOTH `fuel_cap >= 50_000_000` (same floor;
   Precondition 6 target applies identically) AND `timeout_ms >= 30_000` — `fuel_cap`
   sufficiency alone does NOT satisfy PC9 for these five (their real exhaustion axis is the
   host wall-clock timeout, `timeout_ms`, per Invariant 8).
   **Precondition 6 dependency (calibration-sufficiency, ADR-039 §Decision 4 v1.16 / §Erratum
   E-007, Q2/F-S2122-P1-002):** satisfying this postcondition's `fuel_cap >= 50_000_000` clause
   (the inclusive floor) is necessary but NOT alone sufficient — Precondition 6's
   machine-checkable sufficiency assertion (`fuel_cap >= ceil(observed_max × 1.5)` against the
   mandated calibration corpus, verified by a passing regression assertion) MUST additionally
   hold for all six targeted validators (both adapter classes) before this postcondition's
   final state is calibration-complete, not merely floor-compliant.
   **PC12 dependency for full protection (ADR-039 §AMD-002, RATIFIED v1.10):** satisfying
   this postcondition's registry-config criteria (`fuel_cap >= 50_000_000` and, for
   `legacy-bash-adapter.wasm`-hosted plugins, `timeout_ms >= 30_000`) is necessary but, per
   §AMD-002, NOT alone sufficient for the five `legacy-bash-adapter.wasm`-hosted plugins to
   be treated as fully protective — PC12's runtime-wiring assertion (the calibrated
   `timeout_ms` value must actually reach the bash-subprocess kill deadline, not the
   hardcoded `BASH_TIMEOUT_MS` constant) MUST additionally hold before S-21.11's Phase 4
   fail-closed flip for these five plugins is considered complete. A commit that satisfies
   PC9's registry-config criteria while PC12 remains unmet (wiring fix not yet landed) leaves
   the residual AMD-002 gap open and MUST be flagged to the orchestrator as a known
   limitation, not silently treated as PC9-complete. The default targeted set is all six of:
   `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`,
   `validate-cross-site-correspondence`. If EC-004 fires for any plugin in this set
   (calibration reveals an impractical cap requirement), S-21.11 is descoped to the flippable
   subset via orchestrator-approved spec amendment; PC9's asserted set is reduced to the
   flippable plugins only; the deferred plugin routes to named follow-up story S-21.13. PC9
   asserts the post-amendment set, not necessarily all six — a partial-set completion is valid
   if EC-004 applied.
   **Critical caveat for `on_error = "block"` plugins — EC-004 is NOT a valid descope path
   (F-S2111-P2-003, amended v1.4):** When calibration reveals a required `fuel_cap > 500M` for
   any of the five `on_error = "block"` plugins (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`), EC-004 deferral is NOT a permitted resolution — it is
   **annotate-or-block-the-flip only**: either (a) annotate the plugin
   `failure_policy="fail-closed"` within S-21.11 (even if the cap requirement is high; surface
   to orchestrator and raise the cap), or (b) block the entire Phase-4 executor flip (do not
   ship the enforcement-active decision path in S-21.11) until the plugin can be annotated in a
   follow-up. There is NO path that permits the enforcement-active executor to merge while any
   `on_error="block"` plugin remains at `failure_policy=fail-open` — PC11's CI gate makes that
   state mechanically un-mergeable. Routing an `on_error="block"` plugin to S-21.13 is a
   mis-route: S-21.13 is scoped exclusively to `validate-cross-site-correspondence`'s O(n)
   fuel-ceiling algorithmic fix and has no mandate to annotate `on_error="block"` plugins.
   The `validate-cross-site-correspondence` plugin (`on_error="continue"`) does NOT carry this
   regression risk — it already failed open on exhaustion under the 2-arg function; its deferral
   routes to S-21.13 per EC-004's valid on_error=continue descope path.
   **Annotation-landing obligation for EC-004 Case A (F-S2111-P5-005):** When
   `validate-cross-site-correspondence` is deferred to S-21.13, that story (or its named
   successor) MUST include an explicit mandate to annotate
   `validate-cross-site-correspondence` with `failure_policy="fail-closed"` and a calibrated
   `fuel_cap >= 50_000_000` once its O(n) fuel-ceiling algorithmic fix removes the excessive
   cap requirement. The fail-closed annotation MUST NOT fall through the EC-004 descope; the
   descope is a timing deferral only, not a permanent exemption from fail-closed enforcement.
   Advisory-only and observability plugins MUST NOT receive
   `failure_policy = "fail-closed"`.

10. **PC10 — `fail_closed_timeout_with_on_error_block` revised, not deleted (TD-VSDD-059
    complement to PC7):**
    The existing test `fail_closed_timeout_with_on_error_block` in the executor module
    (which under the 2-arg `plugin_fail_closed` currently asserts
    `Timeout { cause: Fuel|Epoch } + on_error=Block → block==true`) MUST be DELIBERATELY
    REVISED (not deleted) to assert the new axes-independent semantics. The revised test
    MUST cover both sub-cases:
    (a) `Timeout { cause: Fuel|Epoch } + on_error=Block + failure_policy=FailOpen → NOT block`
    (exit 0): exhaustion is governed by `failure_policy`; `on_error=Block` does NOT apply
    to exhaustion outcomes when `failure_policy=FailOpen` (PC5 / EC-009).
    (b) `Timeout { cause: Fuel|Epoch } + on_error=Block + failure_policy=FailClosed → block`
    (exit 2): exhaustion governed by `failure_policy=FailClosed`; both axes agree on block,
    but the block is caused by `failure_policy`, not `on_error` (PC1 / PC6 class).
    The function name MUST be retained or a close derivative used (e.g.,
    `fail_closed_timeout_with_on_error_block_axes_independent`). The revised test MUST
    appear in the PR diff. Deletion without an equivalent replacement is a TD-VSDD-059
    paper-fix violation. This PC is parallel to PC7's treatment of
    `fail_closed_timeout_with_on_error_continue_is_open` — both sibling tests require
    revision to accurately reflect the extended decision function's axes-independent
    semantics.
    **Integration-level mirror (TC-12) requires the same deliberate revision
    (F-S2111-P11-001):**
    The integration-level test `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block`
    (TC-12) in `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs` — labeled
    by the codebase as the integration-level mirror of `fail_closed_timeout_with_on_error_block`
    — constructs `on_error=Block + failure_policy=FailOpen (registry default) + Timeout{Epoch}`
    and currently asserts `exit_code==2`. Under the axes-independent semantics mandated by
    this BC (PC5/EC-009: `Timeout{Fuel|Epoch} + on_error=Block + failure_policy=FailOpen →
    NOT block → exit 0`), TC-12's `exit_code==2` assertion becomes FALSE. TC-12 MUST be
    DELIBERATELY REVISED (not deleted, per TD-VSDD-059) to assert the new semantics:
    (a) `on_error=Block + failure_policy=FailOpen + Timeout{Epoch} → exit 0`.
    TC-12 SHOULD also carry a corresponding `failure_policy=FailClosed` arm asserting:
    (b) `on_error=Block + failure_policy=FailClosed + Timeout{Epoch} → exit 2`
    for symmetric behavioral coverage (Invariant 6 / Envoy #38801 discipline: tests MUST
    assert observed outcomes, not merely configuration intent; the integration layer carrying
    only the FailOpen arm would leave the integration-level FailClosed path unverified at the
    dispatch level). Both the unit test revision and the TC-12 integration revision MUST appear
    in the PR diff. Deletion of either without an equivalent replacement is a TD-VSDD-059
    paper-fix violation.

11. **PC11 — Hard migration-window completeness gate: if the executor is in enforcement-active
    state, every `on_error="block"` targeted plugin MUST carry `failure_policy="fail-closed"`
    (CWE-636 static gate, checkable at any single commit; name-independent detection):**
    A Cargo integration test (`test_no_on_error_block_without_fail_closed_when_3arg_executor`)
    MUST assert: if `crates/factory-dispatcher/src/executor.rs` is in enforcement-active state —
    detected by the presence of any block-decision site in the executor block-decision chain
    (`execute_tier`, `execute_tiers`, or their helpers) that references a `.failure_policy` value
    when deciding to block on a `Timeout` outcome, however the data reaches that site (via
    `PluginOutcome`, direct field access on `RegistryEntry`, or any intermediate path); this
    signal is name-independent and data-flow-independent: fires for both extend-in-place and
    introduce-a-replacement implementer designs regardless of intermediate data-flow path — then every `[[hook]]` entry
    in `hooks-registry.toml` whose
    `name` is one of the five targeted `on_error = "block"` plugins (`validate-factory-path-root`,
    `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
    `validate-wave-gate-prerequisite`) MUST carry an explicit `failure_policy = "fail-closed"`.
    Any absence of this annotation while the executor is enforcement-active MUST cause the test to
    FAIL (CI blocks merge). This gate detects a failure_policy-dependent exhaustion block
    decision anywhere in the executor block-decision chain (design-flow-independent, not bound
    to any specific intermediate data-flow path); the gate evaluates both conditions (executor
    enforcement-active state AND registry annotation) on the same commit tree, with no ordering
    dependency. The bad intermediate state (enforcement-active executor + any on_error=block
    plugin at fail-open) causes the test to FAIL, making the CWE-636 migration-window
    regression mechanically detectable at any single commit. The gate is GREEN when the
    executor is NOT enforcement-active (Phase 1–2 state), and GREEN only when the executor is
    enforcement-active AND all five plugins carry `failure_policy = "fail-closed"` (Phase 4
    complete state). It fires RED on the bad intermediate state, which is precisely the CWE-636
    regression window that no prior PC gated mechanically.
    The gate MUST include four controls — three structured as PURE FUNCTIONS over INJECTABLE
    inputs (a synthetic executor-source snippet string + a synthetic registry, NOT a scan
    bound to the live tree) and one live-tree assertion at Phase-4-complete:
    (a) **POSITIVE-CONTROL:** enforcement-active executor-source snippet (any block-decision
        site references `.failure_policy` for `Timeout` outcome) + a synthetic registry MISSING
        one of the five on_error=block `failure_policy="fail-closed"` annotations → assert the
        gate fires RED. Proves non-vacuity: the detector fires on the bad intermediate CWE-636
        state; a source-text detector that silently matches zero cannot produce this RED.
    (b) **NEGATIVE-CONTROL:** enforcement-active executor-source snippet + a synthetic registry
        with ALL five on_error=block plugins annotated `failure_policy="fail-closed"` → assert
        the gate does NOT fire (result: PASS). Proves the gate correctly accepts the
        Phase-4-complete state and does not false-positive on valid fully-annotated
        configurations.
    (c) **VACUITY-CONTROL:** enforcement-ABSENT executor-source snippet (no `.failure_policy`
        reference in the block-decision chain for `Timeout` outcomes) + any synthetic registry
        state → assert the gate returns GREEN, AND assert the detector's enforcement-detection
        logic ran and returned `EnforcementAbsent` (via an explicit `detection_ran` / tri-state
        diagnostic), and that RED-emission was skipped as a consequence. This distinguishes a
        genuine Phase-1/2 GREEN (executor not yet enforcement-active; enforcement-detection
        logic ran, correctly classified the executor as enforcement-absent, and skipped
        RED-emission) from a vacuous GREEN caused by a detector failure that never ran the
        enforcement-detection logic at all.
    (d) **LIVE-TREE-CONTROL:** at Phase-4-complete, the detector MUST be run against the
        ACTUAL `crates/factory-dispatcher/src/executor.rs` (not a synthetic snippet) and MUST
        return `enforcement_active = true`. This proves the detector's enforcement-detection
        logic recognizes the real enforcement code as shipped — a syntactically-wrong live-tree
        detector whose `.failure_policy` scan matches zero against the actual `execute_tiers`
        form would pass controls (a), (b), and (c) yet be inert against real code (silent
        CWE-636 false-green). The live-tree assertion closes this gap by asserting the detector
        fires in the enforcement-ACTIVE direction against actual source. Acceptable
        implementation: the POSITIVE-CONTROL synthetic snippet MUST be a verbatim excerpt of
        the real `execute_tiers` block-decision site, AND the same detector run on the live
        tree MUST return `enforcement_active = true`.
    **Relationship to Invariant 7:** This PC makes Invariant 7's ordering rule machine-checkable.
    Invariant 7 remains in force as the human-readable policy statement; PC11 is its
    mechanically-enforced complement. PC11 does NOT replace PC8 or PC9 — those gates address
    different failure modes (uncalibrated caps and final-state completeness respectively).
    **EC-004 descope does NOT reduce the PC11 assertion set:** EC-004's reduced-set deferral
    (via orchestrator-approved spec amendment) applies ONLY to `validate-cross-site-correspondence`
    (`on_error = "continue"`), which is NOT among the five `on_error = "block"` plugins asserted
    by this gate. For the five `on_error = "block"` plugins, EC-004 is not a valid descope path
    — they must be annotated `failure_policy="fail-closed"` within S-21.11 or the Phase-4 flip
    must be blocked entirely (see EC-004 amendment v1.4). PC11's five-plugin assertion has no
    reduced-set escape.

12. **PC12 — AMD-002 runtime wiring: the effective bash-subprocess wall-clock kill deadline
    for `legacy-bash-adapter.wasm`-hosted plugins MUST equal the registry's calibrated
    `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS` constant (ADR-039 §AMD-002, RATIFIED
    v1.10; closes the runtime-wiring gap left open by PC8/PC9's registry-config-only
    assertions; kill-time margin corrected — Wave-7 pass-2 remediation, F-S2121-P2-004):**
    For any `[[hook]]` entry with `plugin = "hook-plugins/legacy-bash-adapter.wasm"` and
    registry field `timeout_ms = X`, a real dispatch that invokes that plugin against a bash
    subprocess whose runtime exceeds `X` MUST have its subprocess killed AT OR AFTER `X`.
    `X` is a FLOOR on the kill time, not a ceiling — the kill MUST NOT fire before `X`
    elapses. This is a ONE-SIDED guarantee: PC12 makes no claim about how SOON after `X` the
    kill must observably land at the granularity the pre-F-S2121-P2-004 wording implied; see
    below for the correct upper bound.

    **Two distinct margins — internal poll granularity vs. observed end-to-end kill time
    (F-S2121-P2-004 correction; these are NOT the same quantity and MUST NOT be conflated):**
    (a) **Internal poll granularity (~5 ms):** the `exec_subprocess.rs::run()` deadline-poll
        loop's own polling interval (`std::thread::sleep(Duration::from_millis(5))`) between
        successive deadline checks. This is an implementation-mechanism detail internal to
        the executor — it bounds how finely the poll loop itself samples, NOT what a real
        end-to-end e2e test may observe as the total kill latency.
    (b) **Observed end-to-end kill time (hundreds-of-ms, CI-jitter-robust):** the externally
        OBSERVED kill time in a real dispatch spans the WASI host-call boundary, the
        `exec_subprocess` poll loop, `SIGKILL` signal delivery, process reap, and the test's
        own wall-clock measurement — all subject to CI-runner scheduling jitter. Realistically
        this is `X` plus a delay in the HUNDREDS-OF-MILLISECONDS range, not ~5 ms. Asserting a
        ~5 ms upper bound on the OBSERVED end-to-end kill time is a flaky-test generator: it
        wrongly imports (a)'s internal polling granularity as a bound on (b)'s externally
        observable timing.
        **The correct e2e assertion is a bounded window:
        `X <= observed_kill_time <= X + margin`, where `margin` is a CI-jitter-robust value
        in the hundreds-of-ms range** — large enough to absorb realistic scheduling jitter,
        but still ≪ the hardcoded 60 s constant, so the test still proves the registry's
        calibrated `timeout_ms` is honored (not the unrelated 60 s hardcoded default
        coincidentally satisfying a loose assertion).

    This is a distinct enforcement point from wasmtime's `EPOCH_TICK_MS` (10 ms,
    `crates/factory-dispatcher/src/engine.rs`) — that ticker governs guest-WASM epoch
    interruption and is unrelated to this host-level subprocess-kill deadline (ADR-039
    §Decision 4 v1.9 mechanism-precision correction; do not conflate the two).

    **POSITIVE control (the wiring fix's target behavior; margin wording corrected
    F-S2121-P2-004):** a `hooks-registry.toml` entry hosted by `legacy-bash-adapter.wasm`
    with `timeout_ms = 2_000` (deliberately short), invoked against a bash script that sleeps
    10 s. The subprocess MUST be observed killed within the bounded window
    `2_000 ms <= observed_kill_time <= 2_000 ms + margin` (a CI-jitter-robust hundreds-of-ms
    `margin` — NOT the internal ~5 ms poll granularity) — well under the hardcoded 60 s —
    proving the wiring fix reads and applies the registry's calibrated value rather than the
    hardcoded constant, while remaining robust against CI-runner scheduling jitter. The
    dispatcher MUST report `PluginResult::Ok { exit_code: 1, .. }` (via `adapter_logic`'s
    `HookResult::Error` mapping, per the propagation path traced in PC13(a)) for that
    invocation, **NEVER** `PluginResult::Timeout { cause: TimeoutCause::Epoch }` — that
    variant is constructed only by `classify_trap` on a genuine guest `Trap::Interrupt` and
    cannot fire while the guest is blocked inside the synchronous `exec_subprocess` host
    call. PC12 owns the timing/wiring guarantee asserted here — that the kill fires AT OR
    AFTER the registry's calibrated `timeout_ms` (never before) and is OBSERVED within a
    CI-jitter-robust bounded window above it, not at the hardcoded constant; PC13 owns the
    complementary, non-redundant guarantee that for `on_error = Block` entries this resulting
    `Ok { exit_code != 0 }` outcome is treated as a fail-closed block regardless of
    `failure_policy`.

    **NEGATIVE reference (the AMD-002 defect state — documented as the current/pre-fix
    baseline, not a standing test that must continue passing):** under the CURRENT
    (pre-wiring-fix) implementation, the same `timeout_ms = 2_000` entry invoked against the
    same 10 s-sleeping script is NOT killed until ≈60 s, because `run_bash_via_host`
    (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs`) passes its own hardcoded
    `BASH_TIMEOUT_MS` constant to the host call, never the registry's `timeout_ms`. The
    wiring fix's Cargo integration test
    (`test_legacy_bash_adapter_honors_registry_timeout_ms`) MUST assert the POSITIVE
    behavior, and MUST fail against the pre-fix code path (red-first against the current
    implementation, green only after AMD-002's wiring fix lands) — this is the Envoy #38801
    discipline (Invariant 6) applied to the wiring fix itself: the test drives the actual
    dispatch/subprocess path with a real short `timeout_ms` and a real long-running script,
    not merely a unit-level assertion that adapter code reads a config field.

    **Blast-radius scope note (non-restrictive on this PC's assertion; restrictive on
    S-21.11's `failure_policy` flip scope):** the wiring defect this PC closes is global to
    `legacy-bash-adapter.wasm` — ADR-039 §AMD-002 (v1.10) confirms it affects all ~37
    `legacy-bash-adapter.wasm`-routed `hooks-registry.toml` entries via live grep, not only
    the five §Decision 2 plugins. `test_legacy_bash_adapter_honors_registry_timeout_ms` MUST
    therefore be written generically against the adapter's wiring behavior (any
    `legacy-bash-adapter.wasm`-hosted entry), not hardcoded to only the five targeted plugins
    — the fix is adapter-level, not per-plugin. S-21.11's `failure_policy = "fail-closed"`
    annotation scope (PC9) remains the five/six named plugins only; PC12 does not expand
    PC9's targeted set.

13. **PC13 — `on_error = Block` fails closed on ANY plugin-reported error exit, not only a
    crash/timeout outcome (ADR-039 §AMD-003, RATIFIED v1.11; closes S-21.11 v2.0 adversarial
    pass-1 BLOCKER F-S2111V2-P1-001 — a bash-adapter host-wall-clock timeout, and any other
    `HookResult::Error` path, surfaces as `PluginResult::Ok { exit_code: 1, .. }`, which PC1-PC12's
    `Crashed | Timeout` matching does not catch):**
    For any plugin dispatched with `on_error = OnError::Block`, if the plugin's outcome is
    `PluginResult::Ok { exit_code, .. }` where `exit_code != 0`, the executor's block-decision
    function (`plugin_fail_closed` or its replacement) MUST return `true`
    (`block_intent = true`, dispatcher exit code 2), REGARDLESS of `failure_policy`. This is a
    THIRD axis alongside PC1-PC9's `failure_policy` (resource-exhaustion) coverage, PC4's
    `on_error`-governs-crash coverage, and PC5/PC10's `failure_policy`-governs-`Timeout`
    axes-independence coverage — it is the missing `on_error`-vs-clean-
    nonzero-exit case: a plugin that ran to completion without crashing or timing out at the
    WASM-trap/epoch layer, but returned a nonzero exit via its own reported `HookResult::Error`
    (exit code 1 per `crates/hook-sdk/src/result.rs::HookResult::exit_code`), or via any other
    non-`outcome:block` nonzero exit path.

    **This rule closes two concrete instances of the same class, unified per ADR-039 §AMD-003's
    F-005 in-scope ruling:**
    (a) **F-001 — bash-adapter host-wall-clock timeout:** a `legacy-bash-adapter.wasm`-hosted
        plugin's bash-subprocess wall-clock timeout (`exec_subprocess.rs::run()`'s poll-loop
        kill) does NOT produce `PluginResult::Timeout { .. }` — that variant is constructed
        ONLY by `classify_trap` on a genuine `Trap::Interrupt`, which cannot fire while the
        guest is blocked inside the synchronous `exec_subprocess` host call. Instead it
        propagates as `host::exec_subprocess`'s `Err(codes::TIMEOUT)` -> `run_bash_via_host`'s
        string-erasing `Err` map -> `adapter_logic`'s `HookResult::error(...)` -> `exit_code = 1`
        -> `classify_trap`'s `Err(I32Exit(1))` arm -> `PluginResult::Ok { exit_code: 1, .. }`.
    (b) **F-005 — any other `HookResult::Error` path (generic, not timeout-specific):** ANY
        other error path inside `adapter_logic` that returns `HookResult::Error` (a missing
        `script_path`, a bash exit code other than 0/2, or `exec_subprocess` itself returning a
        non-timeout error) produces the identical `PluginResult::Ok { exit_code: 1, .. }` shape,
        and the identical fail-open gap applies. F-005 is ruled IN SCOPE for this BC (not
        deferred): both instances are closed by the identical one-line predicate change, so
        splitting them would require touching the same decision function twice for the
        identical root cause.

    **POSITIVE control:** a synthetic `PluginOutcome` with `on_error = OnError::Block` and
    `result = PluginResult::Ok { exit_code: 1, .. }` MUST cause the decision function to return
    `true` (`block_intent = true`; dispatcher exit code 2).

    **NEGATIVE control 1 (clean exit is not blocked):** a synthetic `PluginOutcome` with
    `on_error = OnError::Block` and `result = PluginResult::Ok { exit_code: 0, .. }` MUST NOT
    cause a block via this rule (exit 0) — a genuinely clean pass remains unaffected.

    **NEGATIVE control 2 (`on_error = Continue` is unaffected by this rule):** a synthetic
    `PluginOutcome` with `on_error = OnError::Continue` and
    `result = PluginResult::Ok { exit_code: 1, .. }` MUST NOT cause a block via this rule — the
    pre-existing fail-open-on-crash-when-`on_error=Continue` semantics are preserved unchanged;
    this rule is additive to the `on_error = Block` case only.

    **PC13 Coverage Set — applies to ALL `on_error = OnError::Block` registry entries, not only
    the six PC1–PC12 exhaustion-migration-targeted plugins (F-S2111V2-P2-004 remediation,
    MEDIUM, human-directed production-grade full-coverage decision):** PC13's predicate is a
    dispatcher-level, plugin-name-independent rule — it fires for ANY `on_error = Block`
    registry entry, regardless of whether that entry is among the six plugins targeted by
    PC1–PC12's `failure_policy` exhaustion-migration. The six-plugin scoping that governs
    Preconditions 2/3, PC8/PC9, and Invariants 7/8 is SPECIFIC to the exhaustion/`failure_policy`
    migration and does NOT limit PC13's `on_error`-vs-`Ok{exit != 0}` fail-closed rule, which is
    independent of `failure_policy` and applies wherever `on_error = Block` is configured,
    present or future.

    `plugins/vsdd-factory/hooks-registry.toml` is the sole authoritative source for the current
    `on_error = "block"` set; this enumeration is a snapshot and MUST be re-verified against the
    live registry if drift is suspected (`grep -c 'on_error = "block"'
    plugins/vsdd-factory/hooks-registry.toml` against the row count below). At authoring time
    (2026-08-19), the registry carries **18 `on_error = "block"` `[[hooks]]` entries across 17
    unique plugin names** (`protect-secrets` is registered twice — once for `tool = "^Bash$"`,
    once for `tool = "^Read$"` — two independently dispatched registry entries sharing one
    plugin binary, each an independent PC13-coverage unit). Story-writer MUST author one AC/test
    per entry below (differentiating `protect-secrets`'s two entries by tool trigger):

    | # | Plugin name | Event | Tool trigger |
    |---|-------------|-------|---------------|
    | 1 | `block-ai-attribution` | PreToolUse | `^Bash$` |
    | 2 | `brownfield-discipline` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 3 | `check-factory-commit` | PreToolUse | `^Bash$` |
    | 4 | `destructive-command-guard` | PreToolUse | `^Bash$` |
    | 5 | `factory-branch-guard` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 6 | `protect-bc` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 7 | `protect-secrets` (entry A) | PreToolUse | `^Bash$` |
    | 8 | `protect-secrets` (entry B) | PreToolUse | `^Read$` |
    | 9 | `protect-vp` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 10 | `red-gate` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 11 | `validate-pr-merge-prerequisites` | PreToolUse | `^Agent$` |
    | 12 | `validate-wave-gate-prerequisite` | PreToolUse | `^Agent$` |
    | 13 | `verify-git-push` | PreToolUse | `^Bash$` |
    | 14 | `validate-stable-anchors` | PreToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 15 | `validate-factory-path-root` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 16 | `validate-input-hash` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 17 | `validate-template-compliance` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` |
    | 18 | `lint-registry-async-invariant` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` |

    Of these 18, five (`validate-factory-path-root`, `validate-input-hash`,
    `validate-template-compliance`, `validate-pr-merge-prerequisites`,
    `validate-wave-gate-prerequisite`) are ALSO among the PC1–PC12 exhaustion-migration
    six-plugin set — for these five, PC9/PC11's exhaustion-axis coverage and PC13's
    plugin-error-exit coverage are independent, both-required obligations on the same entry.
    The sixth PC1–PC12-targeted plugin, `validate-cross-site-correspondence`, carries
    `on_error = "continue"` and is therefore NOT in PC13's coverage set at all (PC13 is additive
    to `on_error = Block` only, per NEGATIVE control 2 above). The remaining thirteen entries in
    the table were previously uncovered by any PC13-specific test vector or AC and are brought
    into scope by this Coverage Set.

    **Relationship to `exit_code == 2`:** `exit_code == 2` (the `HookResult::Block` mapping)
    remains additionally, independently caught by `plugin_requests_block`'s stdout-substring
    check regardless of `on_error` (unconditional per the existing CRIT-PR59-001 fix); PC13's
    rule is redundant-but-harmless for that case and newly protective for `exit_code == 1`
    (`HookResult::Error`) and any other nonzero exit a compliant or non-compliant plugin may
    produce.

## Invariants

1. **Axes-independence invariant (ADR-039 §Decision 1):** `failure_policy` governs
   resource-exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`); `on_error`
   governs crash/host-error outcomes. Neither axis overrides the other. A plugin may
   simultaneously carry `on_error = "continue"` (crash = advisory) and
   `failure_policy = "fail-closed"` (exhaustion = block) — this is the intended steady-state
   for most validator-class plugins.

2. **No-half-state invariant (ADR-039 §Decision 3):** No `failure_policy = "fail-closed"`
   annotation MAY coexist with `fuel_cap < 50_000_000` at any CI-passing commit. The
   50_000_000 value is the INCLUSIVE FLOOR beneath ADR-039 §Decision 4's load-bearing
   calibration TARGET (`max(ceil(observed_max × 1.5), 50M)` — v1.16/§Erratum E-007
   target-statistic correction; `observed_max`, not `measured_p99`; `ceil()` rounds up to the
   nearest whole fuel unit per F-S2122-P4-001 Wave-7 pass-4 remediation); exactly 50_000_000
   is the inclusive minimum VALID value (`fuel_cap >= 50_000_000` is required; `fuel_cap <
   50_000_000` is prohibited), but clearing the floor alone is a necessary schema-level
   minimum, NOT independently sufficient calibration evidence — see Precondition 6 for the
   machine-checkable `ceil(observed_max × 1.5)` sufficiency assertion. The factory default of 20_000_000
   (ADR-042 §Decision 2) is below this floor and insufficient for fail-closed annotation.
   The Phase-3-before-Phase-4 ordering constraint is structurally enforced by the
   `test_no_fail_closed_plugin_with_uncalibrated_cap` CI gate test (PC8).
   Fail-closed without a sufficient budget is equivalent to blocking unconditionally — the
   intended function is to block writes that fail validation, not to block all writes.

3. **CWE-636 closure invariant:** Once Phase 4 is complete, no authorization-class WASM
   validator plugin silently approves a write when it exhausts its fuel budget. The fail-open
   enforcement defect documented in F-S2107-P7-010 (HIGH) is closed for the six targeted
   plugins.

4. **Advisory-only plugins remain fail-open:** Observability hooks, telemetry collectors, and
   convergence-tracking plugins MUST NOT receive `failure_policy = "fail-closed"`.
   Classification as validator-class vs advisory-class is per plugin; only the six explicitly
   named plugins receive the flip.

5. **D-442(e) line-count workaround remains in force until calibration confirms sufficiency:**
   The ≤3500 soft / ≤4000 hard `lessons.md` line-count workaround from D-442(e) MUST remain
   in force until per-plugin calibration (PC2) confirms that all validators reading `lessons.md`
   have a `fuel_cap` sufficient for the D-442(e) hard limit (4000 lines). If calibration shows
   insufficiency, the workaround remains and the finding surfaces to the orchestrator rather
   than silently relaxing D-442(e).

6. **Behavioral tests, not configuration tests (Envoy #38801 lesson):** Every enforcement
   postcondition (PC1 through PC6) MUST be verified by tests that drive the actual dispatch
   path with a budget-exhausting input and assert the observed outcome (block or advisory),
   NOT merely that the `failure_policy` field is configured to the expected value.

7. **Symmetric half-state prohibition — `on_error="block"` targeted plugins must not regress
   to fail-open on exhaustion (migration-ordering atomicity, F-S2111-P2-001; regression trigger
   narrowed to WIRING, not AUTHORING, per ADR-044 capstone-owned enforcement flip,
   F-S2119-P2-001 remediation):** The five targeted plugins currently carrying
   `on_error = "block"` in `hooks-registry.toml` (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`) presently block on exhaustion via the `on_error` path of
   the 2-arg `plugin_fail_closed`. Once the extended function is **WIRED INTO** the executor's
   real block-decision chain — i.e., once any block-decision site in `execute_tier`,
   `execute_tiers`, or their helpers references a `.failure_policy` value when deciding to
   block on a `Timeout` outcome (the SAME enforcement-active signal PC11's static gate detects;
   see PC11 for the precise, name-independent, data-flow-independent detection rule) —
   exhaustion is governed exclusively by `failure_policy` from that point forward. **Two
   distinct events MUST NOT be conflated (ADR-044):**
   - **Authoring the extended decision function (inert; NOT the flip):** writing the extended
     3-arg `plugin_fail_closed` function (or a replacement) as a standalone, unit-tested pure
     function; adding the `failure_policy: FailurePolicy` field to `PluginOutcome` and
     populating it at construction sites. This is inert data plumbing — the function exists and
     is callable, but no block-decision site in the executor's real block-decision chain
     consults it. A CI-passing commit that contains this authored-but-unwired function while
     any of the five plugins remains at `failure_policy = fail-open` is **NOT** a CWE-636
     regression and is NOT prohibited by this invariant — the executor is not yet
     enforcement-active, so exhaustion for these five plugins continues to be governed by the
     pre-existing `on_error = "block"` path exactly as before. (Concretely: S-21.19 delivers
     this authoring leg.)
   - **Wiring the extended function into the block-decision chain (the flip; enforcement-active
     per PC11's signal):** replacing the `execute_tiers`/`execute_tier` call site's 2-arg
     invocation with the extended form that consults `.failure_policy` for `Timeout` outcomes.
     THIS is the event this invariant governs. Any CI-passing commit at which the executor is
     enforcement-active by this wiring signal while any of the five `on_error="block"` plugins
     remains at `failure_policy = fail-open` (absent-field default) IS a **CWE-636 regression**
     — those plugins will fail OPEN on exhaustion rather than block. This invariant prohibits
     that half-state. (Concretely: S-21.24, the capstone, delivers this wiring leg, and by
     construction cannot land before all five plugins are already annotated — ADR-044
     §Decision.)
   The wiring commit and the `failure_policy = "fail-closed"` annotations for the five
   `on_error="block"` plugins MUST be co-committed (same commit) or ordered
   annotate-before-flip (annotations committed first, wiring commit committed second) — this
   ordering constraint applies to the WIRING event, never to the authoring event, which carries
   no ordering obligation because it is not itself enforcement-active.
   The `validate-cross-site-correspondence` plugin (`on_error = "continue"`) does NOT
   contribute this regression risk — it already failed open on exhaustion under the 2-arg
   function and continues to do so until explicitly annotated. The PC8 gate test enforces
   the calibration constraint (no fail-closed without `fuel_cap >= 50M`). PC11 makes this
   ordering constraint a static CI gate (checkable at any single commit, keyed on the same
   wiring signal defined above, not on mere function presence; the bad intermediate
   state — executor enforcement-active via wiring AND any on_error=block targeted plugin at
   fail-open — causes PC11's test to FAIL, making it mechanically impossible to merge). This
   Invariant 7 remains the human-readable policy statement governing the atomicity/ordering
   constraint; it is now stated congruently with PC11's wiring-keyed detection signal and with
   ADR-044's capstone-ownership resolution for split-topology delivery (no contradiction between
   an inert authoring-only commit and this invariant's regression trigger).

8. **Fuel-axis calibration is necessary but not sufficient for `legacy-bash-adapter.wasm`-hosted
   plugins (ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001, architect-CONFIRMED HIGH):**
   Because a `legacy-bash-adapter.wasm`-hosted plugin's bash subprocess execution occurs after
   — and is invisible to — the adapter's own WASM fuel-metered marshaling step (ADR-042
   §Decision 3 class (b): fuel exhaustion, if any, occurs before the WASI `exec_subprocess`
   call), `fuel_cap` sufficiency provides no protection against a bash subprocess wall-clock
   hang. `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, and `validate-pr-merge-prerequisites` are all hosted by
   `hook-plugins/legacy-bash-adapter.wasm` and are therefore all subject to this invariant.
   These five plugins additionally require calibrated `timeout_ms` sufficiency
   (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`, ADR-039 §Decision 4 host-wall-clock-timeout-axis formula)
   before receiving `failure_policy = "fail-closed"`. `validate-cross-site-correspondence`
   (native `hook-plugins/validate-cross-site-correspondence.wasm`) is NOT subject to this
   invariant — its validation logic executes directly as WASM instructions, so `fuel_cap`
   genuinely bounds its execution end-to-end. **Self-lock consequence for the PreToolUse
   `^Agent$` gates:** `validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites`
   are two of the five `legacy-bash-adapter.wasm`-hosted plugins AND are registered on
   `event = "PreToolUse"`, `tool = "^Agent$"`. Flipping either to `failure_policy = "fail-closed"`
   on `fuel_cap` sufficiency alone, without demonstrated `timeout_ms` sufficiency, risks a
   hard, unconditional block on every future `Agent` tool dispatch — including the dispatches
   needed to fix the miscalibration (ADR-039 §Decision 3 v1.8 amendment).

9. **Config-vs-runtime wiring bifurcation for `legacy-bash-adapter.wasm`-hosted plugins
   (ADR-039 §AMD-002, RATIFIED v1.10; PC12):** A calibrated `timeout_ms >= 30_000` declared
   in `hooks-registry.toml` (Invariant 8 / PC8 / PC9) governs the DECLARED config value only.
   It does not, by itself, guarantee that value is the one enforced at the bash-subprocess
   kill deadline — `legacy-bash-adapter`'s `run_bash_via_host` currently feeds a hardcoded
   `BASH_TIMEOUT_MS = 60_000` constant to `exec_subprocess.rs::run()`, independent of the
   registry's `timeout_ms`. PC12 closes this wiring gap; until PC12's wiring fix lands,
   PC9's final-state assertion for the five `legacy-bash-adapter.wasm`-hosted plugins is
   registry-complete but NOT runtime-complete, and the residual gap MUST be surfaced to the
   orchestrator rather than silently treated as full protection.

10. **PC13 additive-only invariant — NOT a `Crashed | Timeout` superset (ADR-039 §AMD-003,
    RATIFIED v1.11; narrowed per ADR-039 v1.13 / Erratum E-005; F-S2111V2-P5-001
    remediation):** PC13 is an ADDITIVE-ONLY extension of the `on_error = Block` decision
    surface. It is NOT a "strict superset of the pre-existing `Crashed | Timeout` fail-closed
    rule" — that framing is REJECTED: it is the same broad-negation mental model that ADR-039
    Erratum E-005 identified as the root error and removed from the authoritative Precise Rule,
    because it implies a single governing axis (`on_error`) decides all three outcome shapes,
    which is false. The three outcome shapes reachable under `on_error = OnError::Block` are
    governed by DIFFERENT, axes-independent rules and MUST NOT be conflated:
    - `Crashed` outcomes are governed SOLELY by `on_error` (PC4): `on_error = Block` blocks
      unconditionally; `failure_policy` is not consulted.
    - `Timeout { cause: Fuel | Epoch }` outcomes are governed EXCLUSIVELY by the
      `failure_policy` axis, NEVER by `on_error` alone (PC1/PC5/PC6/PC10; EC-009;
      Invariant 1): under `failure_policy = FailOpen`, a `Timeout` does NOT block even when
      `on_error = Block` (PC5/PC10(a)); only `failure_policy = FailClosed` blocks a `Timeout`
      (PC1/PC6/PC10(b)). A `Timeout` does NOT "continue to block under `on_error = Block`
      exactly as before" — that phrasing wrongly imports the pre-extension
      `Timeout + on_error=Block → block` behavior that PC10/TC-12 exist specifically to
      overturn.
    - `Ok { exit_code != 0, .. }` outcomes are the ONE new leg PC13 adds:
      `on_error == OnError::Block` AND `result` is `Ok { exit_code, .. }` where
      `exit_code != 0` → block. This is NOT a negation of `Ok { exit_code: 0, .. }` — a
      negation form would also (wrongly) capture `Timeout` and `Crashed` outcomes and collapse
      the `failure_policy` axis's exclusive governance of `Timeout` blocking, reintroducing the
      CWE-636 self-lock hazard this story closes.
    PC13 does not restate, override, reinterpret, or narrow the `Crashed`/`Timeout` base rules
    above — it adds exactly one previously-uncaught outcome shape (`Ok { exit_code != 0 }`) to
    the `on_error = Block` decision surface. `exit_code == 2` (`HookResult::Block`'s mapping)
    remains additionally, independently caught by `plugin_requests_block`'s stdout-substring
    check regardless of `on_error` — PC13's rule is redundant-but-harmless for that case and
    newly protective for `exit_code == 1` (`HookResult::Error`) and any other nonzero exit a
    compliant or non-compliant plugin may produce.

11. **PC13 full-registry-coverage invariant (F-S2111V2-P2-004 remediation, MEDIUM,
    human-directed production-grade full-coverage decision):** PC13's `on_error = Block` +
    `Ok { exit_code != 0 }` → block rule is NOT scoped to the six PC1–PC12
    exhaustion-migration-targeted plugins. It is a dispatcher-level, `failure_policy`-independent
    rule that MUST hold for every `on_error = "block"` entry in `hooks-registry.toml`, present
    and future. `hooks-registry.toml` is the sole authoritative source for the current set (18
    `[[hooks]]` entries / 17 unique plugin names as of this BC version, per PC13's Coverage Set
    table); a test suite asserting only a sampled subset of `on_error = Block` plugins does NOT
    satisfy PC13. Any new `on_error = "block"` registry entry added after this BC version
    automatically inherits PC13's requirement without requiring a BC amendment — the
    requirement is entry-count-agnostic — but the enumerated Coverage Set table SHOULD be
    refreshed at the next BC touch if a registry grep shows the count has drifted from 18.

12. **Migration coverage-continuity invariant (ADR-044 v1.3 Addendum, closes HIGH
    F-S2121-P3-001, Wave-7 pass-3 remediation):** Across the wave-7→wave-8 split-topology
    migration of the executor's block-decision predicates (ADR-044 Addendum v1.3's Wiring
    Sequence), at EVERY commit from S-21.19's merge through S-21.24's merge, the live
    `execute_tiers` block-decision call site MUST block `Timeout { cause: Fuel | Epoch }`
    outcomes under `on_error = OnError::Block` via SOME live disjunct:
    - Through the end of wave 7: the retained pre-migration 2-arg
      `plugin_fail_closed(result, on_error)` call, UNMODIFIED from its pre-wave-7 baseline
      (`Crashed | Timeout` matching, referencing neither `.failure_policy` nor a
      `failure_policy`-gated `Timeout` decision).
    - From S-21.24's single migration commit onward:
      `plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)`, which ADDS the
      exhaustion-leg disjunct AND REMOVES the retained 2-arg call in the SAME commit.
    NO commit in this range fails open on `Timeout{Fuel|Epoch}` for any `on_error = Block`-
    registered plugin, including the two `PreToolUse`/`^Agent$` self-lock-hazard gates
    BC-1.03.018 governs (`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`).

    **Distinct from, and additive to, Invariant 7/PC11's ANNOTATION-vs-ENFORCEMENT direction:**
    Invariant 7/PC11 governs whether the exhaustion leg may be enforcement-active (wired) while
    any of the five `on_error="block"` plugins remains unannotated — that invariant is
    unweakened and unchanged. This Invariant 12 governs a DIFFERENT direction: COVERAGE-
    CONTINUITY — whether the wave-7 wiring step itself ever SUBTRACTS a live blocking path
    for `Timeout + on_error=Block`. Both invariants must hold simultaneously; neither
    substitutes for the other.

    **Structural closure via ADR-044 v1.3's wiring sequence:** S-21.21's wave-7 wiring task
    ADDS `plugin_fail_closed_on_error_exit(result, on_error)` ADDITIVELY, ALONGSIDE the
    existing 2-arg `plugin_fail_closed` call — it does NOT replace or remove that call,
    precisely so `Timeout + on_error=Block` coverage is never interrupted during the
    wave-7→8 window. Only S-21.24's capstone commit performs the atomic migration (remove the
    2-arg call, add the exhaustion-leg disjunct) in a single commit, at which point PC11's own
    precondition (all five plugins already annotated) is already satisfied by construction. A
    wave-7 wiring commit that literally REPLACED the retained 2-arg call with only
    `plugin_fail_closed_on_error_exit` (which is unconditionally out-of-scope for `Timeout`)
    would open exactly the fail-open window this invariant forbids — this was the HIGH defect
    ADR-044 v1.1/v1.2→v1.3 corrected (F-S2121-P3-001).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin with `failure_policy = "fail-closed"` completes successfully (no exhaustion) | Exit 0; both `on_error` and `failure_policy` are irrelevant for clean-pass outcomes |
| EC-002 | Plugin with `failure_policy = "fail-closed"` + `on_error = "continue"` crashes (not exhaustion) | Crash governed via `on_error = continue`; exit 0 (crash is advisory); `failure_policy` is not consulted for crash outcomes |
| EC-003 | Plugin with `failure_policy = "fail-closed"` + `on_error = "block"` crashes (not exhaustion) | Crash governed via `on_error = block`; exit 2; `failure_policy` not consulted for crash outcomes |
| EC-004 | Calibration reveals a targeted plugin needs `fuel_cap >= PRACTICAL_FUEL_CEILING` (`500_000_000`, ADR-039 §Decision 4 v1.16) for `ceil(observed_max × 1.5)` (Precondition 6's sufficiency target — v1.16/§Erratum E-007 target-statistic correction from the prior `p99×1.5` framing; ceil() per F-S2122-P4-001) | MUST surface to orchestrator; do not annotate with an insufficient cap. **Path diverges by `on_error` value — the two cases are NOT symmetric:** **Case A — `on_error="continue"` plugin (`validate-cross-site-correspondence`) — VALID descope path:** S-21.11 is descoped to the flippable subset via orchestrator-approved spec amendment; PC9's enumerated set is reduced to the flippable plugins only; the deferred plugin routes to named follow-up story **S-21.13** (validate-cross-site-correspondence targeted-row lookup eliminating the O(n) fuel ceiling; depends_on [S-21.10, S-21.11]). This is behavior-neutral: `validate-cross-site-correspondence` already fails open on exhaustion under the 2-arg function and continues to do so until annotated. PC11's five-plugin assertion is unaffected because `validate-cross-site-correspondence` is not among the five `on_error="block"` plugins asserted by PC11. **Annotation-landing obligation (F-S2111-P5-005):** S-21.13 (or its named successor) MUST include an explicit mandate to annotate `validate-cross-site-correspondence` with `failure_policy="fail-closed"` and a calibrated `fuel_cap >= 50_000_000` once its O(n) fuel-ceiling algorithmic fix removes the excessive cap requirement. The EC-004 descope is a timing deferral only, not a permanent exemption from fail-closed enforcement — the fail-closed annotation MUST NOT fall through the descope. **Case B — `on_error="block"` plugins (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`, `validate-wave-gate-prerequisite`) — EC-004 is NOT a valid descope path (F-S2111-P4-002):** For these five plugins, deferral via EC-004 is **forbidden**. The only permitted resolutions are: **(a) annotate-within-S-21.11:** annotate the plugin `failure_policy="fail-closed"` in S-21.11 (even if the cap requirement is high; surface to orchestrator and raise it); OR **(b) block-the-flip:** do not ship the enforcement-active decision path in S-21.11 until the plugin is annotated in a follow-up story. There is NO path that permits the enforcement-active executor to merge while any `on_error="block"` plugin remains at `failure_policy=fail-open` — PC11's CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) makes that state mechanically un-mergeable. Routing an `on_error="block"` plugin to S-21.13 is a mis-route: S-21.13 is scoped exclusively to `validate-cross-site-correspondence`'s O(n) fuel-ceiling algorithmic fix and has no mandate to annotate `on_error="block"` plugins. |
| EC-005 | `lessons.md` validator exhausts on a >4000-line `lessons.md` after Phase 4 flip | Signals calibration was insufficient (PC2+PC9 not met); surface to orchestrator; D-442(e) remains in force |
| EC-006 | `TimeoutCause::Epoch` with `failure_policy = "fail-closed"` | BLOCK (exit 2); epoch deadline is a resource-exhaustion outcome; same enforcement path as `TimeoutCause::Fuel` |
| EC-007 | New validator-class plugin added after S-21.11 merges (without `failure_policy = "fail-closed"`) | Defaults to `fail-open` per BC-1.01.016 backward-compat; PC8 gate only fires for annotated-but-uncalibrated entries; classification of new plugins is a future-story concern |
| EC-008 | Plugin with `on_error = "block"` + `failure_policy = "fail-closed"` exhausts fuel | BLOCK via `failure_policy` path; `on_error = block` is redundant for exhaustion when `failure_policy = "fail-closed"` but both agree on the block outcome |
| EC-009 | Plugin with `on_error = "block"` + `failure_policy = "fail-open"` exhausts fuel | Exit 0; exhaustion governed by `failure_policy = fail-open`; `on_error = block` does not apply to exhaustion outcomes (PC5) |
| EC-010 | `legacy-bash-adapter.wasm`-hosted plugin's script completes within the calibrated `timeout_ms` (no exhaustion) | Clean pass; PC12 not exercised (no kill event) — same as EC-001 baseline |
| EC-011 | `legacy-bash-adapter.wasm`-hosted plugin's script runs LONGER than the calibrated `timeout_ms` but SHORTER than the hardcoded 60,000 ms constant (e.g., `timeout_ms=30_000`, script runs 45 s) — the highest-risk AMD-002/AMD-003 defect window | **Pre-wiring-fix (current implementation) — CORRECTED (F-002; prior "silent false clean-pass at 45s" characterization was WRONG):** the outcome is NONDETERMINISTIC, not a deterministic clean pass. Because `run_bash_via_host` passes the hardcoded `BASH_TIMEOUT_MS=60_000` constant to `exec_subprocess.rs::run()` regardless of the registry's calibrated `timeout_ms=30_000`, the subprocess is not killed at 30 s — but the dispatch surfaces as EITHER (a) `PluginResult::Ok { exit_code: 1, .. }` via `adapter_logic`'s `HookResult::Error` mapping (if the bash script, `exec_subprocess`, or the adapter's own marshaling encounters any error condition), OR (b) a guest-epoch `PluginResult::Timeout { cause: TimeoutCause::Epoch }` race on control-return (if wasmtime's independent `EPOCH_TICK_MS` guest-interruption ticker fires against the adapter's own WASM execution once control returns from the blocking host call). **Sub-case enforcement distinction (F-S2121-P4-001, Wave-7 pass-4 remediation — corrects the prior blanket "no enforcement" claim that contradicted Invariant 12):** (a) `Ok{exit_code:1}` sub-case (via `HookResult::Error`) — IS NOT blocked pre-§AMD-003: PC13's `on_error=Block+Ok{exit!=0}→block` rule does not exist yet; the dispatcher treats this as a clean pass. §AMD-003 closes this gap. (b) `Timeout{Epoch}` sub-case — IS BLOCKED via `on_error=Block` by the RETAINED 2-arg `plugin_fail_closed` call (Invariant 12 guarantees coverage-continuity throughout wave 7 per ADR-044 v1.3 ADDITIVE-then-migrate model); the defect is TIMING ONLY: the block fires at ≈60 s (hardcoded `BASH_TIMEOUT_MS` constant), not at the calibrated `timeout_ms=30_000` — so the 30 s budget IS violated, but the `Timeout` outcome IS blocked once the 60 s constant fires. PC12 closes the timing defect. The enforcement absence for sub-case (a) is the distinct gap §AMD-003/PC13 closes. **Post-wiring-fix + PC12 + PC13 (§AMD-002 + §AMD-003 combined; assumes the plugin's steady-state annotation `failure_policy=FailClosed` per PC9, in addition to its pre-existing `on_error=Block`):** PC12 fixes the kill timing (killed at ≈30 s); whichever outcome results is blocked, but by DIFFERENT governing axes, not by `on_error=Block` alone: `PluginResult::Ok { exit_code: 1, .. }` (via `HookResult::Error`) is blocked by PC13 under `on_error=Block` (`failure_policy`-independent); `PluginResult::Timeout { .. }` is blocked by PC1/PC6/PC10(b) under `failure_policy=FailClosed` (`on_error`-independent for the exhaustion decision) — a `Timeout` does NOT block merely because `on_error=Block`; that requires `failure_policy=FailClosed` (PC5/PC10(a) show the `FailOpen` counter-case: `Timeout` + `on_error=Block` + `failure_policy=FailOpen` → NOT block). The nondeterminism in WHICH outcome surfaces no longer has a safety impact under this `FailClosed` steady state — every sub-case now blocks, each via its own axis. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Timeout { cause: Fuel }` + `FailClosed` | Decision function returns `true`; exit 2 | happy-path (fail-closed enforcement, PC1) |
| `Timeout { cause: Fuel }` + `FailOpen` | Decision function returns `false`; exit 0 | happy-path (fail-open pass-through, PC2) |
| `Timeout { cause: Fuel }` + `on_error=Continue` + `FailClosed` | Decision returns `true`; exit 2 — `on_error` does not override `failure_policy` for exhaustion | axes-independence (PC3) |
| `Crashed` (crash) + `on_error=Block` + `FailOpen` | Decision returns `true`; exit 2 — crash governed by `on_error` | crash-path (PC4) |
| `Timeout { cause: Fuel }` + `on_error=Block` + `FailOpen` | Decision returns `false`; exit 0 — exhaustion not governed by `on_error=block` when `failure_policy=FailOpen` | exhaustion-is-not-crash (PC5) |
| `Timeout { cause: Epoch }` + `FailClosed` | Decision returns `true`; exit 2 — epoch exhaustion = fuel exhaustion for enforcement | epoch-exhaustion (PC6) |
| Real dispatch: plugin with `fuel_cap=100` + `failure_policy="fail-closed"` on budget-exhausting payload | Observed dispatcher exit code 2 | integration (PC1 behavioral) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=20_000_000` (factory default per ADR-042 §Decision 2, strictly below floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` FAILS (CI blocks the half-state) | half-state-rejected (PC8 POSITIVE-CONTROL, F-S2111-P3-001) |
| All six targeted plugins with `failure_policy="fail-closed"` + `fuel_cap=75_000_000` (example calibrated) | `test_all_six_validator_class_plugins_are_fail_closed` passes; PC8 gate passes | final-state (PC9) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=75_000_000` (>= 50M floor, calibrated) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (gate accepts valid calibrated entry) | negative-control (PC8 NEGATIVE-CONTROL fixture, F-S2111-P2-004) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=50_000_000` (exactly at inclusive floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (inclusive floor: exactly 50_000_000 is a valid calibrated value per ADR-039 §Decision 4) | boundary-pass (PC8, F-S2111-P3-001 inclusive-floor) |
| Plugin annotated `fuel_cap=50_000_000` (clears PC8's floor); standing CI gate re-runs against the FROZEN `pc6-sufficiency-snapshot/` fixture, yielding `observed_max`=`40_000_000` → `ceil(observed_max × 1.5) = ceil(60_000_000.0) = 60_000_000 > 50_000_000` | Sufficiency regression assertion (`fuel_consumed × 1.5 <= registry fuel_cap`) FAILS even though PC8's floor gate PASSES — floor-clearing alone is not calibration-sufficient | half-state-rejected (Precondition 6 sufficiency assertion, Q2/F-S2122-P1-002, ADR-039 §Decision 4 v1.16/§Erratum E-007; ceil() per F-S2122-P4-001) |
| Plugin annotated `fuel_cap=75_000_000`; standing CI gate re-runs against the FROZEN `pc6-sufficiency-snapshot/` fixture, yielding `observed_max`=`40_000_000` → `ceil(observed_max × 1.5) = ceil(60_000_000.0) = 60_000_000 <= 75_000_000` | Sufficiency regression assertion PASSES (`fuel_consumed × 1.5 <= registry fuel_cap` holds) | negative-control (Precondition 6 sufficiency assertion, Q2/F-S2122-P1-002; ceil() per F-S2122-P4-001) |
| `lessons.md`/`STATE.md`/`decision-log.md` grow substantially (organic project growth) AFTER the frozen `pc6-sufficiency-snapshot/` was captured, with NO change to the plugin's own fuel efficiency | Standing CI gate's `fuel_consumed` reading is UNCHANGED (measured against the frozen snapshot, not the live-grown files) — the gate does NOT false-fail merely because the live corpus grew; proves the frozen-snapshot mechanism decouples the standing assertion from organic corpus growth (F-S2122-P3-001 correction, Wave-7 pass-3) | frozen-vs-live-split (Precondition 6, F-S2122-P3-001) |
| `hooks-registry.toml` entry with `plugin="hook-plugins/legacy-bash-adapter.wasm"` + `failure_policy="fail-closed"` + `timeout_ms=10_000` (current live default for four of the five targeted bash-adapter plugins, strictly below the 30_000 host-wall-clock-timeout-axis floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` FAILS (CI blocks the half-state — fuel_cap alone does not calibrate the bash subprocess's actual exhaustion axis) | half-state-rejected (PC8 TIMEOUT-POSITIVE-CONTROL, F-S2111-P13-001, ADR-039 §Decision 4 v1.8) |
| `hooks-registry.toml` entry with `plugin="hook-plugins/legacy-bash-adapter.wasm"` + `failure_policy="fail-closed"` + `timeout_ms=45_000` (>= 30_000 host-wall-clock-timeout-axis floor, calibrated) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (gate accepts valid calibrated entry) | negative-control (PC8 TIMEOUT-NEGATIVE-CONTROL, F-S2111-P13-001, ADR-039 §Decision 4 v1.8) |
| `Timeout { cause: Fuel\|Epoch }` + `on_error=Block` + `FailOpen` (revision of `fail_closed_timeout_with_on_error_block` sub-case a) | Decision returns `false`; exit 0 — exhaustion governed by `failure_policy=FailOpen`; `on_error=Block` does not apply to exhaustion | axes-independence-on_error_block-fail-open (PC10a) |
| `Timeout { cause: Fuel\|Epoch }` + `on_error=Block` + `FailClosed` (revision of `fail_closed_timeout_with_on_error_block` sub-case b) | Decision returns `true`; exit 2 — exhaustion governed by `failure_policy=FailClosed`; block caused by failure_policy, not on_error | axes-independence-on_error_block-fail-closed (PC10b) |
| TC-12 revised: `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (`full_stack_plugin_invocation.rs`) — `on_error=Block` + `failure_policy=FailOpen` (registry default) + `Timeout{Epoch}` (integration dispatch) | Observed dispatcher exit code 0 — axes-independent semantics: exhaustion governed by `failure_policy=FailOpen`; `on_error=Block` does not apply to exhaustion outcomes (revision of TC-12 per PC10 / F-S2111-P11-001; TD-VSDD-059 deliberate revision) | integration-mirror-fail-open (PC10 TC-12 revision) |
| TC-12 symmetric arm (SHOULD): `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` — `on_error=Block` + `failure_policy=FailClosed` + `Timeout{Epoch}` (integration dispatch) | Observed dispatcher exit code 2 — exhaustion governed by `failure_policy=FailClosed`; symmetric behavioral coverage per Invariant 6 / Envoy #38801 discipline (integration-layer fail-closed path verified at observed-outcome level, not merely configuration assertion) | integration-mirror-fail-closed-symmetric (PC10 TC-12 symmetric arm) |
| Synthetic enforcement-active executor-source snippet (any block-decision site in `execute_tier`/`execute_tiers`/helpers references `.failure_policy` for `Timeout` outcome, however the data reaches it) + synthetic registry MISSING one of the five on_error=block `failure_policy="fail-closed"` annotations | `test_no_on_error_block_without_fail_closed_when_3arg_executor` FAILS (gate fires RED; POSITIVE-CONTROL: proves non-vacuity — detector fires on bad intermediate CWE-636 state; data-flow-independent detection) | migration-window-gate (PC11 POSITIVE-CONTROL) |
| Synthetic enforcement-active executor-source snippet (any block-decision site references `.failure_policy` for `Timeout` outcome) + synthetic registry with all five on_error=block plugins annotated `failure_policy="fail-closed"` | `test_no_on_error_block_without_fail_closed_when_3arg_executor` PASSES (Phase 4 complete state; NEGATIVE-CONTROL: gate does not false-positive on valid fully-annotated registry) | migration-window-pass (PC11 NEGATIVE-CONTROL) |
| Synthetic enforcement-ABSENT executor-source snippet (no `.failure_policy` reference in block-decision chain for `Timeout` outcome) + any synthetic registry state | Gate returns GREEN AND detector's enforcement-detection logic ran and returned `EnforcementAbsent` (tri-state diagnostic; RED-emission skipped as consequence; VACUITY-CONTROL: distinguishes genuine Phase-1/2 GREEN from vacuous GREEN caused by detector that never ran enforcement-detection logic) | migration-window-vacuity (PC11 VACUITY-CONTROL) |
| Live tree: actual `crates/factory-dispatcher/src/executor.rs` at Phase-4-complete (enforcement-active code present) | Detector returns `enforcement_active = true` (LIVE-TREE-CONTROL: proves detector fires against real enforcement code, not only synthetic snippets; closes CWE-636 false-green gap where a wrong detector passes synthetic controls yet is inert on real code per F-S2111-P6-002) | migration-window-live-tree (PC11 LIVE-TREE-CONTROL) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=2_000`, bash script sleeps 10 s (CURRENT pre-fix code path — `run_bash_via_host` passes hardcoded `BASH_TIMEOUT_MS`) | Subprocess killed at ≈60_000 ms; registry `timeout_ms` value ignored | AMD-002-defect-baseline (PC12 NEGATIVE reference, F-S2111-P13-001 wiring gap) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=2_000`, bash script sleeps 10 s (POST-wiring-fix: `run_bash_via_host` passes registry `timeout_ms`) | Subprocess killed within the bounded window `2_000 ms <= observed_kill_time <= 2_000 ms + margin` (a CI-jitter-robust hundreds-of-ms `margin` — NOT the internal ~5 ms `exec_subprocess.rs::run()` poll granularity, which bounds only the executor's internal deadline-poll sampling interval, not the externally observed end-to-end kill latency; corrected F-S2121-P2-004, Wave-7 pass-2); `PluginResult::Ok { exit_code: 1, .. }` (via `HookResult::Error`) observed within that window — NEVER `Timeout{cause: Epoch}` (guest-trap-only variant, cannot fire inside the blocking `exec_subprocess` host call); `test_legacy_bash_adapter_honors_registry_timeout_ms` PASSES | AMD-002-wiring-fixed (PC12 POSITIVE control) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=30_000`, bash script runs 45 s (CURRENT pre-fix code path) | NONDETERMINISTIC pre-fix outcome: EITHER (a) `PluginResult::Ok{exit_code:1}` via `HookResult::Error` — NOT blocked pre-§AMD-003 (enforcement absence; §AMD-003/PC13 closes this) — OR (b) `Timeout{cause:Epoch}` race on control-return — IS BLOCKED via retained 2-arg `plugin_fail_closed` on_error=Block path (Invariant 12), but only when the 60 s constant fires (timing defect; PC12 closes this). Not a deterministic clean pass (EC-011, F-002 corrected); enforcement is sub-case-dependent, NOT absent in both (F-S2121-P4-001, Wave-7 pass-4 remediation) | AMD-002/AMD-003-nondeterministic-defect (EC-011, PC12+PC13; sub-case enforcement distinction per Invariant 12) |
| `on_error=Block` + `PluginResult::Ok { exit_code: 1, .. }` (synthetic `PluginOutcome`) | Decision function returns `true`; `block_intent=true`; exit 2 — closes F-001 (bash-adapter timeout surfacing as `Ok{exit:1}`) and F-005 (any other `HookResult::Error` exit path) uniformly | plugin-error-fail-closed (PC13 POSITIVE control, ADR-039 §AMD-003) |
| `on_error=Block` + `PluginResult::Ok { exit_code: 0, .. }` (synthetic `PluginOutcome`) | Decision function returns `false` via this rule; exit 0 — a genuinely clean pass is unaffected by PC13 | plugin-error-fail-closed-negative (PC13 NEGATIVE control 1) |
| `on_error=Continue` + `PluginResult::Ok { exit_code: 1, .. }` (synthetic `PluginOutcome`) | Decision function returns `false` via this rule; exit 0 — PC13 is additive to `on_error=Block` only; pre-existing fail-open-on-crash-when-`on_error=Continue` semantics preserved unchanged | plugin-error-fail-closed-negative (PC13 NEGATIVE control 2) |
| All 18 `on_error="block"` `hooks-registry.toml` entries / 17 unique plugin names (see PC13 Coverage Set table), each with synthetic `PluginOutcome { on_error: Block, result: Ok { exit_code: 1 } }` | Decision function returns `true` for EVERY entry; `block_intent=true`; exit 2 — full-registry coverage, not a 6-plugin sample (F-S2111V2-P2-004 remediation) | plugin-error-fail-closed-full-coverage (PC13 Coverage Set) |

## Related BCs

- **BC-1.01.016** — prerequisite schema: provides `FailurePolicy` enum and
  `RegistryEntry.failure_policy` field; S-21.10 (BC-1.01.016) MUST merge before S-21.11 (this BC)
- **BC-1.03.002** — sibling detection layer: governs `invoke_plugin` returning
  `PluginResult::Timeout { cause: TimeoutCause::Fuel }` when fuel is exhausted; BC-1.03.017
  governs the enforcement decision the executor makes with that result based on `failure_policy`.
  The two BCs are complementary: BC-1.03.002 establishes the detection precondition;
  BC-1.03.017 establishes the enforcement postcondition.
- **BC-1.03.009** — sibling block-intent: governs `block_intent` for the `HookResult::Block`
  path; BC-1.03.017 adds a parallel block-intent path for exhaustion outcomes under
  `failure_policy = "fail-closed"`

## Architecture Anchors

- `crates/factory-dispatcher/src/executor.rs` — enforcement-active decision path, **split into
  two independently-timed, independently-owned functions per ADR-044 v1.1 Addendum (RATIFIED
  2026-08-22, architect Q1 adjudication Option A, F-S2121-P1-002; supersedes the prior
  single-function `plugin_fail_closed`/`plugin_exhaustion_fail_closed` (3-arg) + "PC13
  extension" citation used through v1.19):**
  **`plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)` — the exhaustion leg.**
  A `PluginOutcome` type carries a `failure_policy: FailurePolicy` field; this function consults
  it for block decisions; for `Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }`,
  returns `true` when `failure_policy == FailurePolicy::FailClosed` regardless of `on_error` —
  governs PC1, PC2, PC3, PC5, PC6, PC10, and is the sole wiring signal PC11's static gate
  detects (any block-decision site referencing `.failure_policy` for a `Timeout` outcome).
  `fail_closed_timeout_with_on_error_continue_is_open` test MUST be revised (not deleted) to
  assert `Timeout + Continue + FailOpen → NOT block` (PC7);
  `fail_closed_timeout_with_on_error_block` test MUST ALSO be revised (not deleted) — per
  PC10, the revised test asserts both sub-cases: (a) `Timeout + on_error=Block + FailOpen →
  NOT block` (exit 0) and (b) `Timeout + on_error=Block + FailClosed → block` (exit 2);
  both sibling tests (`on_error_continue` and `on_error_block`) require the same axes-
  independence treatment; TD-VSDD-059 applies to both;
  `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12) in
  `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs` is the integration-level
  mirror of `fail_closed_timeout_with_on_error_block` and MUST ALSO be revised (not deleted,
  per TD-VSDD-059) to assert exit 0 for the `on_error=Block + failure_policy=FailOpen + Timeout{Epoch}`
  case (F-S2111-P11-001); SHOULD additionally carry a `failure_policy=FailClosed` arm asserting
  exit 2 for symmetric behavioral coverage (Invariant 6 / Envoy #38801); both revisions MUST
  appear in the PR diff. **Live wiring of this leg is capstone-owned, deferred to S-21.24**
  (UNCHANGED from ADR-044 v1.0 — the annotate-before-flip half-state hazard governed by
  Invariant 7/PC11 applies to this leg only, because it alone references `.failure_policy` for
  a `Timeout` outcome and depends on all five `on_error="block"` plugins already being
  calibrated/annotated per Precondition 6 and PC9).
  **`plugin_fail_closed_on_error_exit(result, on_error)` — the error-exit leg (ADR-039
  §AMD-003, RATIFIED v1.11; narrowed to the correct predicate per ADR-039 v1.13 / Erratum
  E-005, F-S2111V2-P3-001 remediation; PC13's behavioral-test leg; this anchor CORRECTS the
  prior single-function "3-arg `plugin_fail_closed` + PC13 extension" citation used through
  v1.19 to this named function per ADR-044 v1.1 Addendum).** Governs `on_error ==
  OnError::Block` outcomes that are NOT resource-exhaustion: PC4's `Crashed` case (already
  live today, unchanged) EXTENDED per §AMD-003 to additionally return `true` when `result` is
  `PluginResult::Ok { exit_code, .. }` where `exit_code != 0` — this closes the
  `PluginResult::Ok { exit_code: 1 }` gap left open by `Crashed`-only matching. The extension
  fires ONLY on a non-zero-exit `Ok` outcome; it MUST NOT be written as a negation of
  `Ok { exit_code: 0, .. }` — that broad negation wrongly captures `Timeout { cause: Fuel |
  Epoch }` outcomes as well, collapsing the `failure_policy` axis's exclusive governance of
  `Timeout` blocking (PC5/EC-009/Invariant 1) into this function and reintroducing the CWE-636
  self-lock this story prevents. `Timeout{..}` remains governed EXCLUSIVELY by the
  `failure_policy` axis — via `plugin_fail_closed_on_exhaustion` above, never via this
  function — per PC1/PC5/PC6/PC10; `on_error = Block` alone does NOT block a `Timeout` when
  `failure_policy = FailOpen` (PC5/PC10(a)). **Live wiring of this leg is wireable
  immediately and is now owned by S-21.21 at wave 7** (ADR-044 v1.1 Addendum, RATIFIED
  2026-08-22 — this leg references neither `.failure_policy` nor `Timeout`-as-a-calibration-
  gate, trips nothing in PC11's detector, and carries no annotated-fleet precondition, unlike
  the exhaustion leg above); this leg is NOT deferred to S-21.24, correcting the pre-Addendum
  single-function framing that implied both legs shared S-21.24's wiring timeline. **S-21.21's
  wave-7 wiring task ADDS this call ADDITIVELY, ALONGSIDE the retained 2-arg
  `plugin_fail_closed(result, on_error)` call — it does NOT replace or remove that call
  (ADR-044 v1.3 correction, closes HIGH F-S2121-P3-001: a literal wave-7 replacement would
  drop live `Timeout + on_error=Block` coverage for the entire wave-7-to-wave-8 window, since
  this function is unconditionally out-of-scope for `Timeout`). Only S-21.24's wave-8
  capstone commit performs the atomic migration — removing the retained 2-arg call AND adding
  `plugin_fail_closed_on_exhaustion` in the SAME commit — see Invariant 12 for the
  coverage-continuity guarantee this wiring sequence establishes.
- `crates/hook-sdk/src/result.rs::HookResult::exit_code` — PC13's cited mapping:
  `HookResult::Error { .. } => 1`; this is the exit code an `adapter_logic` error path (F-001's
  bash-adapter timeout via `Err(codes::TIMEOUT)`, or F-005's any-other-error path) produces,
  which surfaces at the executor as `PluginResult::Ok { exit_code: 1, .. }` via
  `classify_trap`'s `Err(I32Exit(1))` arm — the shape PC13 closes
- `plugins/vsdd-factory/hooks-registry.toml` (PC13 full-registry scope, distinct from the
  six-plugin scope of the anchor below) — the authoritative source for PC13's Coverage Set: as
  of this BC version, 18 `[[hooks]]` entries carry `on_error = "block"` across 17 unique plugin
  names (`protect-secrets` registered twice, once per `tool = "^Bash$"` and once per
  `tool = "^Read$"`); see PC13's body for the full enumerated table. Unlike the six-plugin
  `failure_policy` exhaustion-migration scope below, PC13's coverage obligation is registry-wide
  and entry-count-agnostic — it applies to every current and future `on_error = "block"` entry
  regardless of whether that entry ever receives a `failure_policy` annotation
- `plugins/vsdd-factory/hooks-registry.toml` — six targeted plugin entries receive calibrated
  `failure_policy = "fail-closed"` atomically per-plugin, with the calibrated field(s)
  determined by adapter class (ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001): the
  native-WASM plugin (`validate-cross-site-correspondence`) needs `fuel_cap >= 50M`; the five
  `legacy-bash-adapter.wasm`-hosted plugins (`validate-factory-path-root`,
  `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
  `validate-pr-merge-prerequisites`) need BOTH `fuel_cap >= 50M` AND `timeout_ms >= 30_000`
  (the host-wall-clock-timeout-axis floor — `fuel_cap` alone does not calibrate their actual exhaustion axis,
  since the bash subprocess is invisible to the WASM fuel counter per ADR-042 §Decision 3
  class (b)). 50_000_000 / 30_000 are the inclusive FLOORS; values below are rejected by PC8 —
  but for `fuel_cap`, `50_000_000` is a floor beneath the load-bearing
  `ceil(observed_max × 1.5)` target (ADR-039 §Decision 4 v1.16/§Erratum E-007; ceil() per
  F-S2122-P4-001; Precondition 6's machine-checkable sufficiency regression assertion, not
  PC8's floor gate alone, is what determines calibration completeness);
  Phase 4 annotations land ONLY after Phase 3 calibration completes for every axis a plugin's
  adapter class is subject to; PC8 gate test enforces no half-state on EITHER axis
  (fuel_cap-only calibration is standing regression/invariant gate; timeout_ms calibration for
  `legacy-bash-adapter.wasm` entries added F-S2111-P13-001; migration-window ordering enforced
  by PC11); PC11 gate test enforces no on_error=block targeted plugin at fail-open while
  executor is enforcement-active (detected via any block-decision site in
  `execute_tier`/`execute_tiers`/helpers referencing `.failure_policy` for `Timeout` outcome,
  however the data reaches it — data-flow-independent per F-S2111-P5-002; gate includes
  POSITIVE/NEGATIVE/VACUITY/LIVE-TREE controls per F-S2111-P5-001 and F-S2111-P6-002)
- `crates/factory-dispatcher/src/registry.rs` — `FailurePolicy` enum and
  `RegistryEntry.failure_policy` field (delivered by S-21.10 / BC-1.01.016); executor reads
  `failure_policy` from the dispatched `RegistryEntry`
- ADR-039 §Decision 3 — safe migration ordering: Phase 1 (schema) → Phase 2 (mitigations) →
  Phase 3 (calibration, bifurcated by adapter class per v1.8 amendment) → Phase 4 (enforcement
  flip); no half-state at any CI-passing commit on EITHER calibration axis
- ADR-039 §Decision 6 — four required behavioral test scenarios: Timeout+FailClosed→block
  (PC1); Timeout+FailOpen→advisory (PC2); on_error independence (PC3); crash≠exhaustion
  distinct paths (PC4+PC5)
- ADR-039 §Decision 1/2/4 v1.8 amendment (§AMD-001; F-S2111-P13-001) — fuel-vs-host-wall-clock-timeout axis
  bifurcation: `fuel_cap` calibration is genuinely sufficient only for the native-WASM plugin
  (`validate-cross-site-correspondence`); the five `legacy-bash-adapter.wasm`-hosted plugins
  additionally require `timeout_ms` calibration per the new host-wall-clock-timeout-axis formula
  (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`), because their bash subprocess is
  invisible to the WASM fuel counter
- ADR-042 §Decision 3 class (b) — evidentiary basis for the v1.8 amendment: "fuel exhaustion
  occurs before the WASI `exec_subprocess` call, the bash script body never executes when the
  adapter is fuel-starved"; confirms `fuel_cap` cannot meter bash subprocess execution time
  for `legacy-bash-adapter.wasm`-hosted plugins
- `crates/hook-plugins/legacy-bash-adapter/src/lib.rs::run_bash_via_host` — PC12's wiring-fix
  target: this call site currently passes its own hardcoded `BASH_TIMEOUT_MS = 60_000`
  constant to the host `exec_subprocess` call; the fix requires it to instead pass the
  invoking `RegistryEntry`'s calibrated `timeout_ms` value (ADR-039 §AMD-002, RATIFIED v1.10)
- `crates/factory-dispatcher/src/host/exec_subprocess.rs::run()` — the actual host
  enforcement point for the bash subprocess wall-clock bound: an `Instant`-based deadline
  loop (`std::thread::sleep(Duration::from_millis(5))` poll interval) that calls
  `child.kill()` on overrun. PC12 asserts against THIS enforcement point, not wasmtime's
  `EPOCH_TICK_MS` (10 ms, `crates/factory-dispatcher/src/engine.rs`) — the two are distinct
  and unrelated mechanisms (ADR-039 §Decision 4 v1.9 mechanism-precision correction; do not
  conflate the adapter's poll interval with the epoch ticker). **The ~5 ms poll interval here
  is this loop's INTERNAL sampling granularity only (F-S2121-P2-004, Wave-7 pass-2) — it is
  NOT the bound a real e2e test may assert on OBSERVED end-to-end kill latency, which is
  subject to CI-runner scheduling jitter and realistically lands in the hundreds-of-ms range
  above `timeout_ms`; PC12's e2e assertion is the bounded window
  `X <= observed_kill_time <= X + margin`, not `X + ~5 ms`.**

## Story Anchors

- S-21.10 (prerequisite: Phase 1 schema extension; BC-1.01.016)
- S-21.11 (Phase 3 calibration + Phase 4 enforcement flip)
- S-21.13 (EC-004 follow-up for `validate-cross-site-correspondence` only: targeted-row lookup eliminating O(n) fuel ceiling; depends_on [S-21.10, S-21.11]; scoped exclusively to the on_error=continue plugin; on_error=block plugins are NOT routed here per EC-004 amendment v1.4; MUST include explicit mandate to annotate `validate-cross-site-correspondence` `failure_policy="fail-closed"` once O(n) fuel ceiling is removed — annotation-landing obligation per F-S2111-P5-005)

## VP Anchors

- VP-TBD — failure_policy enforcement dispatch: all six postconditions exercised by behavioral
  tests driving the actual dispatch path; half-state structural gate; all six targeted
  validators carry `failure_policy="fail-closed"` with calibrated `fuel_cap >= max(ceil(observed_max × 1.5), 50M)` (50M is the inclusive floor, `ceil(observed_max × 1.5)` is the load-bearing target per ADR-039 §Decision 4 v1.16/§Erratum E-007; ceil() per F-S2122-P4-001); migration-window completeness gate (PC11)
  **VP-TBD hygiene note (S-21.21 O-3; POLICY 9 sanctioned-deferral convention, following the
  S-21.25/BC-1.03.019 precedent):** this placeholder is not an un-anchored defect — the real VP
  (covering PC13's `on_error=Block` plugin-error-exit fail-closed semantics AND Precondition 6's
  `ceil(observed_max × 1.5)` calibration-sufficiency regression-assertion property) is anchored to
  **Phase-6 formal-verifier**, which assigns a real VP-NNN and propagates it to VP-INDEX,
  `verification-architecture.md`, and `verification-coverage-matrix.md` per
  `vp_index_is_vp_catalog_source_of_truth`.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | For resource-exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`): `failure_policy=FailClosed` → block (exit 2); `failure_policy=FailOpen` → advisory (exit 0); `on_error` does not override `failure_policy` for exhaustion; crash (`PluginResult::Crashed`) is governed by `on_error` only; no `failure_policy="fail-closed"` entry in `hooks-registry.toml` without `fuel_cap >= 50M` (inclusive calibration floor per ADR-039 §Decision 4; `fuel_cap < 50M` is prohibited; `fuel_cap = 50M` is VALID as a floor value, but is NOT independently sufficient calibration evidence — Precondition 6 additionally requires `fuel_cap >= ceil(observed_max × 1.5)`, verified by a passing STANDING regression assertion run against the FROZEN `pc6-sufficiency-snapshot/` fixture (not live-growing corpus files, which would false-fail as they organically grow — F-S2122-P3-001), `fuel_consumed × 1.5 <= registry fuel_cap`, per ADR-039 §Decision 4 v1.16/§Erratum E-007, Q2/F-S2122-P1-002/F-S2122-P3-001; ceil() per F-S2122-P4-001); no `legacy-bash-adapter.wasm`-hosted entry with `failure_policy="fail-closed"` without ALSO `timeout_ms >= 30_000` (inclusive host-wall-clock-timeout-axis calibration floor per ADR-039 §Decision 4 v1.8 amendment / §AMD-001; F-S2111-P13-001 — `fuel_cap` sufficiency alone does not calibrate this adapter class's actual exhaustion axis, since the bash subprocess is invisible to the WASM fuel counter); all targeted validators carry `failure_policy="fail-closed"` with calibration sufficient for their adapter class (native-WASM: `fuel_cap >= 50M`; `legacy-bash-adapter.wasm`-hosted: `fuel_cap >= 50M` AND `timeout_ms >= 30_000`); `fail_closed_timeout_with_on_error_block` test revised (not deleted) to assert both `on_error=Block + FailOpen → NOT block` and `on_error=Block + FailClosed → block` (PC10; TD-VSDD-059); integration-level mirror `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12, `full_stack_plugin_invocation.rs`) ALSO revised (not deleted) to assert `on_error=Block + failure_policy=FailOpen + Timeout{Epoch} → exit 0`, with SHOULD arm for `failure_policy=FailClosed → exit 2` (symmetric behavioral coverage per Invariant 6 / Envoy #38801; both revisions MUST appear in PR diff; F-S2111-P11-001); PC8 gate test includes both POSITIVE-CONTROL (fail-closed + fuel_cap=20M < 50M floor → RED) and NEGATIVE-CONTROL (fail-closed + fuel=75M → PASS; and fuel=50M → PASS) fixtures (POLICY 15); PC11 gate test asserts that if the executor is enforcement-active (detected via any block-decision site in `execute_tier`/`execute_tiers`/helpers referencing `.failure_policy` for `Timeout` outcome, however the data reaches it — data-flow-independent per F-S2111-P5-002; fires for both extend-in-place and introduce-a-replacement designs), all five on_error=block targeted plugins carry failure_policy=fail-closed (CWE-636 static gate; EC-004 descope does not reduce this assertion set); PC11 gate includes POSITIVE-CONTROL (enforcement-active snippet + registry missing one annotation → RED), NEGATIVE-CONTROL (enforcement-active snippet + all five annotated → PASS), VACUITY-CONTROL (enforcement-absent snippet → GREEN AND enforcement-detection logic ran and returned `EnforcementAbsent` with RED-emission skipped as consequence), and LIVE-TREE-CONTROL (detector run against actual `crates/factory-dispatcher/src/executor.rs` at Phase-4-complete → `enforcement_active = true`; closes CWE-636 false-green gap per F-S2111-P6-002) per F-S2111-P5-001 and F-S2111-P6-002. **PC12 (ADR-039 §AMD-002, RATIFIED v1.10; kill-time margin corrected F-S2121-P2-004, Wave-7 pass-2):** for any `legacy-bash-adapter.wasm`-hosted registry entry with `timeout_ms = X`, a real dispatch against a bash subprocess running longer than `X` MUST be killed AT OR AFTER `X` (a one-sided floor, never before `X`), OBSERVED within the bounded window `X <= observed_kill_time <= X + margin` where `margin` is a CI-jitter-robust value in the hundreds-of-ms range — NOT the internal ~5 ms `exec_subprocess.rs::run()` poll-loop granularity, which bounds only the executor's own internal sampling interval, not the externally observable end-to-end kill latency (WASI host-call → poll loop → SIGKILL → reap → measurement, all subject to CI-runner scheduling jitter) — not at the hardcoded `BASH_TIMEOUT_MS = 60_000` constant; includes a POSITIVE control (`timeout_ms=2_000`, script sleeps 10 s → killed within `2_000 ms <= observed_kill_time <= 2_000 ms + margin`) and documents the NEGATIVE pre-fix reference (killed ≈60 s regardless of registry `timeout_ms`) as the AMD-002 defect baseline. **PC13 (ADR-039 §AMD-003, RATIFIED v1.11):** for any plugin dispatched with `on_error = OnError::Block`, `result = PluginResult::Ok { exit_code != 0, .. }` MUST cause the decision function to return `true` (block, exit 2), regardless of `failure_policy`; includes a POSITIVE control (`on_error=Block` + `Ok{exit_code:1}` → block) and two NEGATIVE controls (`on_error=Block` + `Ok{exit_code:0}` → no block; `on_error=Continue` + `Ok{exit_code:1}` → unaffected); closes both F-001 (bash-adapter timeout surfacing as `Ok{exit:1}`) and F-005 (any other `HookResult::Error` exit path), ruled in-scope together per §AMD-003. **Precondition 6 (ADR-039 §Decision 4 v1.16/§Erratum E-007, Q2/F-S2122-P1-002; frozen-vs-live split corrected F-S2122-P3-001, Wave-7 pass-3; ceil() adopted F-S2122-P4-001, Wave-7 pass-4):** for all six §Decision 2 fail-closed validators, `fuel_cap >= ceil(observed_max × 1.5)` MUST hold, verified by a passing STANDING regression assertion run against the FROZEN `pc6-sufficiency-snapshot/` fixture — NOT live-growing corpus files, which would false-fail as they organically grow (`fuel_consumed × 1.5 <= registry fuel_cap`); the live corpus is used only for the one-time calibration confirmation that derives the annotated `fuel_cap` value, not for the standing gate; not merely the 50M floor; if `ceil(observed_max × 1.5) >= PRACTICAL_FUEL_CEILING` (500M), the plugin MUST NOT receive `failure_policy="fail-closed"` until a structural remedy exists (ceil() per F-S2122-P4-001). **VP-TBD hygiene (S-21.21 O-3):** the real VP for PC13 semantics and this sufficiency property is anchored to Phase-6 formal-verifier per POLICY 9, following the S-21.25/BC-1.03.019 precedent. | unit tests (executor path coverage per PC1–PC6, PC10, PC13) + integration/bats test (real dispatch at `fuel_cap=100` → exit 2; PC1 behavioral) + Cargo gate tests (hooks-registry.toml parse; PC8 with both controls; PC11 migration-window gate with four controls) + Cargo integration test driving the real `legacy-bash-adapter` subprocess path with a short calibrated `timeout_ms` and a long-running script (`test_legacy_bash_adapter_honors_registry_timeout_ms`; PC12, Envoy #38801 discipline — behavioral, not configuration, assertion) + unit test asserting `on_error=Block` + `PluginResult::Ok{exit_code!=0}` → `block_intent=true` (`test_on_error_block_fails_closed_on_plugin_error_exit_code`; PC13) + standing regression assertion re-running each of the six targeted plugins against the FROZEN `pc6-sufficiency-snapshot/` fixture (not live-growing corpus files) and asserting `fuel_consumed × 1.5 <= registry fuel_cap` (Precondition 6 sufficiency gate, Q2/F-S2122-P1-002/F-S2122-P3-001) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-011 |
| Capability Anchor Justification | CAP-011 ("Enforce fuel and epoch budgets on plugin execution") per capabilities.md §CAP-011 — CAP-011's stated outcome is "a runaway plugin is killed within `timeout_ms` + `EPOCH_TICK_MS` (10ms)... never hung processes." This BC's `failure_policy` enforcement dispatch (block-vs-advisory on `TimeoutCause::Fuel`/`TimeoutCause::Epoch`) is the ENFORCEMENT half of CAP-011 (detection is CAP-011/BC-1.03.002; PC1–PC11 govern what the dispatcher DOES once a budget is exceeded). PC12 (v1.12) extends this same capability anchor one step further: it asserts CAP-011's "killed within `timeout_ms`" guarantee is genuinely met at the runtime level for `legacy-bash-adapter.wasm`-hosted plugins — i.e., that the value CAP-011 promises to enforce is the registry's calibrated `timeout_ms`, not an unwired hardcoded constant (ADR-039 §AMD-002). No new/different capability is needed for PC12: it is the same "runaway plugin killed within budget" outcome, verified at the wiring layer rather than the decision layer. |
| L2 Domain Invariants | TBD (no CAP-011-adjacent DI-NNN currently exists in `domain-spec/invariants.md`; DI-002/DI-003 govern adjacent executor-tier semantics — plugin crash/timeout isolation and `block_intent` aggregation — but do not themselves assert the fuel/epoch-budget enforcement decision this BC governs) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/executor.rs`; enforcement dispatch for resource-exhaustion outcomes; PC12 additionally anchors `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` and `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| ADR | ADR-039 §Decision 1 (axes separation: exhaustion vs crash; v1.8 amendment: fuel-vs-host-wall-clock-timeout signal bifurcation by adapter class); ADR-039 §Decision 2 (validator-class plugins use `fail-closed` after calibration; v1.8 amendment: native-WASM vs `legacy-bash-adapter.wasm` scope split); ADR-039 §Decision 3 (safe migration ordering; Phase-3-before-Phase-4 atomicity; half-state forbidden on both axes; v1.8 amendment: explicit self-lock statement for the two PreToolUse `^Agent$` gates; v1.9 amendment: mandatory authenticated break-glass companion, delivered within S-21.11 (prior follow-up name S-21.17 retired)); ADR-039 §Decision 4 (Option A minimum requirement; v1.8 amendment: parallel `measured_p99_ms×2.0` `timeout_ms` host-wall-clock-timeout-axis formula, 30_000ms floor, for `legacy-bash-adapter.wasm`-hosted plugins — UNCHANGED by the v1.16 correction below; v1.9: reframed as local calibration policy, not an SRE-standard formula; **v1.16 / §Erratum E-007 target-statistic correction (Q2, F-S2122-P1-002/003, RATIFIED 2026-08-22 via orchestrator relay of architect adjudication): the load-bearing `fuel_cap`-axis calibration TARGET for all six §Decision 2 fail-closed validators, without exception, is `max(observed_max × 1.5, 50_000_000)` — `observed_max`, not `measured_p99`; the `50_000_000` value is an inclusive FLOOR beneath that target, not the target itself; `PRACTICAL_FUEL_CEILING = 500_000_000` is formally named as the impractical-cap trigger. **v1.23 / F-S2122-P4-001 ceil() adoption (Wave-7 pass-4 remediation):** this BC adopts `ceil()` on the formula — `max(ceil(observed_max × 1.5), 50_000_000)` — rounding up to the nearest whole fuel unit to prevent boundary-case truncation when `observed_max × 1.5` yields a non-integer; strictly more conservative than truncation. This BC's Precondition 3 (calibration formula) and Precondition 6 (machine-checkable sufficiency regression assertion; frozen-vs-live split corrected F-S2122-P3-001, Wave-7 pass-3, matching S-21.22's converged Task 5a) are its behavioral-contract legs for this correction — Precondition 6 additionally requires a passing STANDING regression assertion (`fuel_consumed × 1.5 <= registry fuel_cap` against the FROZEN `pc6-sufficiency-snapshot/` fixture, not live-growing corpus files), not merely a documented `fuel_cap` value**); ADR-039 §Decision 6 (four behavioral test scenarios; Envoy #38801 lesson — behavioral tests not configuration tests); ADR-039 §AMD-001 (v1.8; F-S2111-P13-001 amendment record; RATIFIED 2026-08-18 v1.9 under POLICY 22); ADR-039 §AMD-002 (v1.9; architect self-verification finding — `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed `BASH_TIMEOUT_MS=60_000` constant independent of the registry's calibrated `timeout_ms`; blast radius: all ~37 `legacy-bash-adapter.wasm`-routed registry entries, not only the five §Decision 2 plugins; RATIFIED 2026-08-19 v1.10 under POLICY 22, corroborated by independent code-review verification against live source — the prior "ADR-025 §Decision 18" corroboration citation was wrong and is retracted; genuine corroboration is ADR-039's own v1.8 §AMD-001 → v1.9 §Decision 4 mechanism-precision self-correction history; wiring-fix remediation delivered WITHIN S-21.11, not a separate follow-up story — **PC12 (v1.12) is this BC's behavioral-test leg for §AMD-002's wiring-fix obligation**); §Decision 3 v1.9 break-glass amendment (mandatory authenticated bypass for the two PreToolUse `^Agent$` gates; minimum-viable definition — environment-variable override, human-operator-only, audited via JSONL — specified in §Decision 3 v1.10 amendment; delivered WITHIN S-21.11, intra-story ordering: break-glass commit precedes or is atomic with the fail-closed-flip commit for the two named gates; **governed by sibling BC-1.03.018, not this BC** — see BC-1.03.018 for the break-glass behavioral contract); ADR-039 §AMD-003 (v1.11, RATIFIED — S-21.11 v2.0 adversarial pass-1 BLOCKER F-S2111V2-P1-001: `on_error = "block"` does not fail-closed on a plugin's own reported `HookResult::Error` (`PluginResult::Ok { exit_code: 1 }`); `plugin_fail_closed` extended per the precise rule `on_error == OnError::Block AND result is Ok { exit_code, .. } where exit_code != 0 => block` — narrowed from an earlier, INCORRECT broad-negation formulation (`result is NOT Ok { exit_code: 0, .. } => block`) to this correct non-zero-exit-`Ok`-only form per ADR-039 §AMD-003 v1.13 / Erratum E-005 (F-S2111V2-P3-001 remediation): the broad negation wrongly captured `Timeout{Fuel|Epoch}` and `Crashed` outcomes, collapsing the `on_error`-vs-`failure_policy` axes-independence this BC's Invariant 1/PC5 establishes and reintroducing the CWE-636 self-lock hazard; the base rules are unchanged but governed by different axes, not one shared predicate — `Crashed` remains governed solely by `on_error` (PC4), while `Timeout` remains governed exclusively by `failure_policy` (PC1/PC5/PC6/PC10), never by `on_error` alone (Invariant 10, v1.17 rewrite, F-S2111V2-P5-001); F-005 generic-error path ruled in-scope alongside F-001's timeout-specific leg — **PC13 (v1.13) is this BC's behavioral-test leg for §AMD-003's enforcement rule**); ADR-042 §Decision 3 class (b) (evidentiary basis: bash subprocess execution is invisible to the WASM fuel counter); **ADR-044 (v1.0, RATIFIED — S-21.19 pre-TDD adversarial convergence BLOCKER F-S2119-P1-001, brownfield cycle v1.0-brownfield-backfill): split-topology flip-sequencing — when S-21.11 is partitioned across independently-mergeable sub-stories, the enforcement-active WIRING event (the commit that makes any block-decision site in the executor's real block-decision chain reference `.failure_policy` for a `Timeout` outcome) is capstone-owned (S-21.24), not core-decision-story-owned (S-21.19); S-21.19 authors and unit-tests the extended decision function plus the inert `PluginOutcome.failure_policy` field/construction-site plumbing ONLY — this does not trip PC11's detector and is not the flip; the actual wiring moves to S-21.24, which by construction (`depends_on: [S-21.19, S-21.20, S-21.21, S-21.22, S-21.23]`) cannot land before all five plugins are annotated — **Invariant 7 (v1.19 rewrite, F-S2119-P2-001 remediation) is this BC's human-readable restatement of ADR-044's authoring-vs-wiring disambiguation, keyed on the same PC11 wiring signal, so that an authoring-only commit (e.g., S-21.19's merge) is never mischaracterized as a CWE-636 regression**; **ADR-044 v1.1 Addendum (2026-08-22, RATIFIED — architect Q1 adjudication, Option A, F-S2121-P1-002): the single "wiring event" above is split into two independently-timed, independently-owned legs, each retaining its own atomicity analysis — the EXHAUSTION leg (`plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)`, governing PC1/PC2/PC3/PC5/PC6/PC10/PC11's `Timeout`/`failure_policy` axis) is UNCHANGED from v1.0: it remains capstone-owned, wiring deferred to S-21.24, because it alone references `.failure_policy` for a `Timeout` outcome and carries the annotate-before-flip half-state hazard; the ERROR-EXIT leg (`plugin_fail_closed_on_error_exit(result, on_error)`, governing PC4's `Crashed` case, already live and unchanged, EXTENDED per §AMD-003 to PC13's `on_error==Block && Ok{exit_code!=0}` case) references neither `.failure_policy` nor `Timeout`-as-a-calibration-gate, trips nothing in PC11's detector, and carries no annotated-fleet precondition — it is now WIREABLE IMMEDIATELY and OWNED BY S-21.21 at wave 7, not deferred to S-21.24 as the pre-Addendum single-function framing implied. This BC's Architecture Anchors `executor.rs` bullet is this correction's behavioral-contract leg: it names both functions and their respective live-wiring owners (S-21.24 exhaustion / S-21.21 error-exit) in place of the prior single "3-arg `plugin_fail_closed` + PC13 extension" citation**; **ADR-044 v1.3 Addendum correction (2026-08-22, RATIFIED — closes HIGH finding F-S2121-P3-001, S-21.21 fresh-context adversarial review): the v1.1/v1.2 Addendum's wording ("wire `plugin_fail_closed_on_error_exit` ... replacing the current bare `plugin_fail_closed` call") was itself a HIGH fail-open defect — a literal wave-7 REPLACEMENT of the retained 2-arg `plugin_fail_closed` call would drop live `Timeout + on_error=Block` coverage for the entire wave-7-to-wave-8 window, because `plugin_fail_closed_on_error_exit` is UNCONDITIONALLY out-of-scope for `Timeout`. v1.3 corrects the wiring sequence: S-21.21's wave-7 task ADDS `plugin_fail_closed_on_error_exit` ADDITIVELY, ALONGSIDE the unmodified retained 2-arg call (does NOT remove it); only S-21.24's wave-8 capstone commit performs the atomic migration — removing the 2-arg call AND adding `plugin_fail_closed_on_exhaustion` in the SAME commit — at which point PC11's precondition (all five plugins already annotated) is already satisfied by construction. **Invariant 12 (new, this burst) is this BC's human-readable restatement of ADR-044 v1.3's migration coverage-continuity guarantee**, symmetric to and non-substituting for Invariant 7/PC11's annotation-vs-enforcement guarantee** |
| Security | CWE-636 (Not Failing Securely — closed for six validator-class WASM plugins after Phase 4, exhaustion path; PC13's plugin-error-exit fail-closed path additionally closes the same CWE-636 class for the FULL 18-entry `on_error="block"` registry set — not exhaustion-specific, independent of `failure_policy`); CWE-390 (Detection of Error Condition Without Action — closed for enforcement path). Research basis: `.factory/research/wasm-fuel-exhaustion-detection.md` |
| Stories | S-21.10 (prerequisite), S-21.11, S-21.13 (EC-004 follow-up for `validate-cross-site-correspondence` only; on_error=block plugins are NOT routed here per EC-004 amendment v1.4; MUST annotate `validate-cross-site-correspondence` fail-closed once O(n) fuel ceiling removed per F-S2111-P5-005) |
| Cycle | v1.0-brownfield-backfill (E-21 Wave 6) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.23 | 2026-08-23 | product-owner | Wave-7 adversary pass-4 remediation, cycle v1.0-brownfield-backfill: (1) F-S2121-P4-001 (HIGH) — EC-011 pre-wiring-fix enforcement characterization corrected: prior text claimed no fail-closed enforcement reaching `on_error=Block` in BOTH sub-cases, contradicting Invariant 12 (added v1.22). Corrected to sub-case distinction: (a) `Ok{exit_code:1}` pre-§AMD-003 IS NOT blocked (PC13/§AMD-003 closes that gap); (b) `Timeout{Epoch}` IS blocked by the retained 2-arg `plugin_fail_closed` call via `on_error=Block` (Invariant 12 guarantees throughout wave 7); defect for (b) is TIMING ONLY (block fires at ≈60 s hardcoded constant, not calibrated `timeout_ms`), not enforcement absence. EC-011 Canonical Test Vector row updated to reflect sub-case split. (2) F-S2122-P4-001 (LOW, PO decision) — ceil() adopted: `max(observed_max × 1.5, 50_000_000)` → `max(ceil(observed_max × 1.5), 50_000_000)` throughout all live predicate occurrences (Precondition 3, Precondition 6 sufficiency threshold, Invariant 2, PC9, VP Anchors, Architecture Anchors). Rationale: `observed_max × 1.5` can yield a non-integer fuel unit; `ceil()` rounds up to the nearest whole unit, preventing boundary-case truncation below the formula target. Strictly more conservative than truncation; production-grade default. (3) F-S2121-P4-002 (MED, PO confirm) — Precondition 6 S-21.22 ownership made explicit: (i) Task 4 one-time calibration confirmation OWNED BY S-21.22; (ii) Task 5a durable standing CI gate OWNED BY S-21.22. Prior text described the mechanisms without naming the owning story. H1 enriched per POLICY 7 with all three clauses. No PC renumbering; all amendments corrective/additive. Does NOT touch BC-1.03.018 (separate file), story bodies/ACs (story-writer step ②), or INDEX/STATE.md (state-manager step ③). Input-hash flagged stale for state-manager reconciliation. BC-1.03.017 v1.23. |
| v1.22 | 2026-08-22 | product-owner | Wave-7 adversary pass-3 remediation, cycle v1.0-brownfield-backfill: (1a) F-S2122-P3-001 (MEDIUM) — Precondition 6 rewritten to distinguish the ONE-TIME live-corpus calibration confirmation (Task 4, deriving the annotated `fuel_cap`) from the DURABLE standing CI regression assertion (Task 5a), which MUST run against a FROZEN corpus snapshot (`pc6-sufficiency-snapshot/`), not live-growing `decision-log.md`/`STATE.md`/`lessons.md` (a false-fail time bomb as they organically grow) — matching S-21.22's converged Task 5a mechanism, closing a story-vs-BC divergence. `>=`/`PRACTICAL_FUEL_CEILING` trigger semantics UNCHANGED. Swept sibling sites: two Canonical Test Vector rows updated + one new row added, VP-TBD Verification Properties row (Property + Proof Method), Traceability §Decision 4 ADR citation. (1b) F-S2121-P3-001 (HIGH, architect ADR-044 v1.3) — new Invariant 12 (migration coverage-continuity): across the wave-7→8 split-topology migration, at every commit from S-21.19's merge through S-21.24's merge, the live block-decision call site MUST block `Timeout` outcomes under `on_error=Block` via SOME disjunct (retained 2-arg `plugin_fail_closed` through wave 7; `plugin_fail_closed_on_exhaustion` from S-21.24's single migration commit onward), so no commit fails open on `Timeout{Fuel|Epoch}` for any `on_error=Block`-registered plugin, including the two `^Agent$` self-lock-hazard gates BC-1.03.018 governs. Distinguished from, and additive to, Invariant 7/PC11's annotation-vs-enforcement direction. Cites ADR-044 v1.3 Addendum. Added an "additive, not replacing" correction to the Architecture Anchors `executor.rs` bullet's S-21.21 wiring sentence and extended the Traceability ADR-044 citation with the v1.3 correction. H1 enriched per POLICY 7. No PC renumbering; Invariant 12 is new (additive); PC1-PC13 UNCHANGED. Does NOT touch BC-1.03.018 (separate product-owner burst, same task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately this burst per POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash` will go stale as a result and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.22. |
| v1.21 | 2026-08-22 | product-owner | Wave-7 adversary pass-2 remediation, cycle v1.0-brownfield-backfill (F-S2121-P2-004): PC12's kill-time margin was written as "observed kill time within `X` plus the `exec_subprocess.rs::run()` poll interval (~5 ms)" — this conflated the executor's INTERNAL deadline-poll sampling granularity (~5 ms) with the OBSERVED end-to-end kill latency a real e2e test (S-21.21's AC-013) must assert, which spans the WASI host-call boundary, the poll loop, SIGKILL delivery, process reap, and the test's own wall-clock measurement — all subject to CI-runner scheduling jitter, realistically landing in the HUNDREDS-OF-MILLISECONDS range above `timeout_ms`. The ~5 ms bound was a flaky-test generator; S-21.21's own remediation had already loosened AC-013 to a one-sided hundreds-of-ms margin, creating a story-vs-BC divergence this fix closes. Rewrote PC12 to the production-grade model: (a) kept the ONE-SIDED guarantee — killed AT OR AFTER `X`, `X` is a FLOOR, never a ceiling; (b) added an explicit distinction between the internal ~5 ms poll granularity (implementation-mechanism detail) and the OBSERVED end-to-end kill time, which MUST be asserted as the bounded window `X <= observed_kill_time <= X + margin` with a CI-jitter-robust `margin` in the hundreds-of-ms range — still ≪ the 60 s hardcoded-constant proof point. Updated PC12's POSITIVE control wording to the bounded-window form. Swept sibling restatements of the ~5 ms figure for TD-VSDD-060 consistency: the `AMD-002-wiring-fixed` Canonical Test Vector row, the `exec_subprocess.rs::run()` Architecture Anchors bullet, and the VP-TBD Verification Properties row's PC12 clause. H1 enriched per POLICY 7. No PC renumbering; PC12-scoped corrective rewrite only — PC1-PC11, PC13, Precondition 6 UNCHANGED. Does NOT touch BC-1.03.018 (separate product-owner burst, same task), the S-21.21 story body/ACs (story-writer's domain, dispatched separately this burst per POLICY 8 propagation — S-21.21's AC-013 wording and its "reconciled with PC12" claim need re-anchoring to this corrected text), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash` will go stale as a result and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.21. |
| v1.20 | 2026-08-22 | product-owner | Wave-7 adversary pass-1 remediation, cycle v1.0-brownfield-backfill (Q1+Q2 architect-adjudicated, human-approved Q1=Option A; Q3 governs sibling BC-1.03.018 only): (1) Q2/F-S2122-P1-002 fuel_cap calibration-statistic bug fix — Precondition 3's `fuel_cap` axis corrected from `measured_p99 × 1.5` to `observed_max × 1.5` per ADR-039 §Decision 4 v1.16/§Erratum E-007; `timeout_ms` axis UNCHANGED; 50_000_000 reframed as an inclusive FLOOR beneath the `observed_max × 1.5` TARGET, not the target itself. Swept sibling restatements for TD-VSDD-060 consistency: Invariant 2, EC-004, PC9 (Precondition-6-dependency clause added), Architecture Anchors, Traceability's §Decision 4 citation, and the VP-TBD Verification Properties row. (2) Q2/F-S2122-P1-002 new Precondition 6 — machine-checkable calibration-sufficiency requirement for all six ADR-039 §Decision 2 validators: `fuel_cap >= observed_max × 1.5` against the mandated calibration corpus (per the existing S-21.22 ≥574 KB/≥4000-line mandate), verified by a regression assertion (`fuel_consumed × 1.5 <= registry fuel_cap`), not the 50M floor alone; cites `PRACTICAL_FUEL_CEILING` (500M, ADR-039 §Decision 4 v1.16). Two new Canonical Test Vector rows (sufficiency-assertion positive/negative). (3) Q1/F-S2121-P1-002 Architecture Mapping split (ADR-044 v1.1 Addendum, RATIFIED 2026-08-22, architect Q1 Option A) — replaced the single `plugin_fail_closed` (3-arg + "PC13 extension") Architecture Anchors citation with the two named functions ADR-044 v1.1 introduces: `plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)` (Timeout/failure_policy axis, PC1/PC2/PC3/PC5/PC6/PC10/PC11; wiring UNCHANGED, capstone-owned, S-21.24) and `plugin_fail_closed_on_error_exit(result, on_error)` (PC4's `Crashed` case EXTENDED per §AMD-003 to PC13's `on_error==Block && Ok{exit_code!=0}` case; wireable immediately, now owned by S-21.21 at wave 7, NOT S-21.24). Traceability's ADR-044 citation extended with the v1.1 Addendum's function split and story-ownership correction. (4) VP-TBD hygiene (S-21.21 O-3) — added an anchor note (VP Anchors + Verification Properties) matching the S-21.25/BC-1.03.019 POLICY-9 sanctioned-deferral precedent: the real VP covering PC13 semantics and the new Precondition-6 sufficiency property is anchored to Phase-6 formal-verifier. H1 enriched per POLICY 7 with the new sufficiency-gate and function-split clauses. No PC renumbering; Precondition 6 is new (additive); all other changes are corrective/citation-only — PC1-PC13's core predicates are UNCHANGED. Does NOT touch BC-1.03.018 (Q3's amendment — separate product-owner burst, same task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately this burst per POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash` will go stale as a result and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.20. |
| v1.19 | 2026-08-20 | product-owner | S-21.19 pass-2 remediation of F-S2119-P2-001 (MEDIUM, brownfield cycle v1.0-brownfield-backfill): Invariant 7's human-readable atomicity policy literally contradicted ADR-044 (capstone-owned enforcement flip). ADR-044 refined the CWE-636 regression TRIGGER from "the extended decision function EXISTS/is contained in a commit" to the narrower "the extended function is WIRED INTO the executor block-decision chain (references `.failure_policy` for a `Timeout` block decision) — enforcement-active per PC11's static-scan signal"; PC11 was already updated to this wiring-keyed form, but Invariant 7 still read "Any CI-passing commit that contains the extended function while any of these five plugins remains at failure_policy = fail-open ... is a CWE-636 regression" and "The decision-function change and the annotations MUST be co-committed... or ordered annotate-before-flip" — read literally, S-21.19's OWN compliant merge (a commit containing the inert, un-wired extended function while the 5 plugins are still fail-open) was a "CWE-636 regression" by Invariant 7's words, directly contradicting ADR-044's declaration that this exact state is SAFE because the function is not wired; Invariant 7 also conflated "decision-function change (authoring — S-21.19, inert)" with "executor flip (wiring — S-21.24 Task 0)". Rewrote Invariant 7 to key the CWE-636 regression trigger on the function being WIRED INTO / IN EFFECT in the block-decision chain (enforcement-active per PC11's signal), explicitly disambiguating authoring (3-arg function + `PluginOutcome.failure_policy` population — inert data plumbing, S-21.19, NOT the flip, NOT prohibited) from wiring (replacing the `execute_tiers`/`execute_tier` 2-arg call site with the 3-arg form — the enforcement-active flip, S-21.24 Task 0, IS what must be atomic-with/after the five annotations). Added ADR-044 to `inputs:` and the Traceability ADR row (new citation naming S-21.19 as the authoring leg and S-21.24 as the capstone wiring leg, and stating Invariant 7 is now this BC's human-readable restatement of ADR-044's authoring-vs-wiring disambiguation, keyed on the same PC11 signal). Verified Invariant 7 is now internally consistent with PC11 (both key on the identical wiring/enforcement-active signal), with ADR-044 (same authoring-vs-wiring split, same S-21.19/S-21.24 story mapping), and with PC5/PC10/Invariant 1's pre-existing axes-independence (untouched — scoped to the migration-ordering trigger only). Swept sibling prose: PC8's "Symmetric half-state prohibition" paragraph already correctly cross-references PC11 as the mechanical enforcer of this ordering constraint ("The mechanical CI gate enforcing this ordering constraint is PC11 ... not this gate") — left as-is, no contradiction. No other "contains the extended function" framing found elsewhere in the BC. No PC renumbering; Invariant-7-scoped rewrite plus one new Traceability ADR citation. Does NOT touch PC11 (already correct, lines 471-541), the 5 citing stories (story-writer's domain — S-21.19, S-21.20, S-21.21, S-21.22, S-21.24 cite re-anchor is story-writer's next dispatch), ADR-044/ADR-039 (architect's domain), BC-1.03.018/BC-1.03.019, or any INDEX/STATE.md (state-manager's domain). Adding ADR-044 to `inputs:` means this BC's own `input-hash` will go stale as a result of this content change and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.19. |
| v1.18 | 2026-08-20 | product-owner | S-21.11 v2.8 PRE-TDD spec-convergence pass-9 remediation of F-S2111V2-P9-001 (MEDIUM): the live `## Traceability` ADR row's §Decision 3 sub-clause cited the break-glass amendment's delivery vehicle two contradictory ways in the SAME cell — "...v1.9 amendment: mandatory authenticated break-glass companion, S-21.17" (a retired, never-authored story ID, absent from STORY-INDEX) immediately followed later in the same cell by "...delivered WITHIN S-21.11...governed by sibling BC-1.03.018...". This BC's own v1.11 changelog claimed the S-21.17→S-21.11 citation redirect was complete, but this one occurrence was missed. Rewrote the live cite to "mandatory authenticated break-glass companion, delivered within S-21.11 (prior follow-up name S-21.17 retired))" — matching the same cell's "delivered WITHIN S-21.11" clause and BC-1.03.018's Stories-row retirement-annotation convention ("the prior follow-up name S-21.17 is retired"). Ran a literal grep sweep (`grep -no "S-21\.17"`) of both BC-1.03.017 and BC-1.03.018 (TD-VSDD-060) and classified every occurrence by captured evidence, not assertion: BC-1.03.017 had 4 line-hits — frontmatter `last_amended` field (describing v1.11's own historical S-21.17→S-21.11 redirect: HISTORICAL, left as-is), the live `## Traceability` ADR row (the F-S2111V2-P9-001 site: LIVE, fixed above), the v1.11 `## Changelog` row (2 occurrences, both describing the v1.11 redirect action retrospectively: HISTORICAL, left as-is), and the v1.10 `## Changelog` row (describing the not-yet-authored S-21.17 amendment as it stood at v1.10: HISTORICAL, left as-is). BC-1.03.018 had 1 line-hit — its Stories row ("the prior follow-up name S-21.17 is retired"): already correctly annotated per the retirement-annotation convention this BC now matches; verified, no edit needed. No PC/Invariant/predicate content altered; citation-only. POLICY 8 parity preserved: this BC has no frontmatter `behavioral_contracts`/`bcs` array, no body BC-table, and no `## Acceptance Criteria` section (ACs live in the S-21.11 story body) — only the `## Traceability` ADR-row prose changed. Does NOT touch ADR-039 (architect's domain), BC-1.03.018 (verified compliant, no edit needed), the S-21.11 story (story-writer's domain — the v1.17→v1.18 cite sweep is story-writer's next dispatch), or any INDEX/STATE.md (state-manager's domain). BC-1.03.018's `inputs:` declaration of this BC means its input-hash will go stale as a result of this content change and is flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.18. |
| v1.17 | 2026-08-19 | product-owner | S-21.11 v2.4 PRE-TDD spec-convergence pass-5 remediation of F-S2111V2-P5-001 (HIGH): the v1.16 remediation narrowed the §AMD-003 fail-closed predicate at Architecture Anchors and Traceability but missed `## Invariants` -> Invariant 10 ("PC13 strict-superset invariant"), which still (a) reused the STRICT-SUPERSET-of-`Crashed \| Timeout` framing ADR-039 Erratum E-005 identified as the root error and removed from the authoritative Precise Rule, and (b) literally contradicted this BC's own axes-independence semantics (PC5/PC10(a)/EC-009/Invariant 1) by asserting `Crashed` and `Timeout` "continue to block under on_error=Block exactly as before," wrongly implying `on_error` alone governs `Timeout` blocking, and mis-labeling PC10 as part of an "on_error-governs-crash path" (PC10 governs `Timeout`/`failure_policy`, not crash). Rewrote Invariant 10 (retitled "PC13 additive-only invariant — NOT a `Crashed \| Timeout` superset") to state the three `on_error=Block`-reachable outcome shapes (`Crashed`->`on_error` only/PC4; `Timeout`->`failure_policy` only, never `on_error` alone/PC1/PC5/PC6/PC10; `Ok{exit_code!=0}`->the one new PC13 leg, not a negation of `Ok{exit_code:0}`) as three separate axes-independent rules. Ran an exhaustive sweep (not a single grep pattern) of every predicate-stating site — all Preconditions/PCs, Invariants, Edge Cases, Architecture Anchors, Verification Properties, Traceability — and found the same residual contradiction pattern surviving in two more sites the prior narrow-grep sweep missed: the Architecture Anchors `executor.rs` bullet's closing sentence ("base rule ... is UNCHANGED and remains governed solely by `on_error` per PC4/PC10", applying `on_error`-only governance to BOTH `Crashed` and `Timeout` jointly) and the Traceability ADR row's §AMD-003 closing clause (identical pattern). Both rewritten to state the two base rules as governed by different axes (`Crashed`->`on_error`/PC4; `Timeout`->`failure_policy`/PC1/PC5/PC6/PC10), never one shared predicate. Also fixed EC-011's "Post-wiring-fix" clause, which claimed both the `Ok{exit_code:1}` and `Timeout` sub-outcomes "MUST produce a block under on_error=Block" and mis-labeled PC10 (paired with PC4) as closing the "Timeout/Crashed" case jointly — rewritten to condition each outcome on its own governing axis (`Ok{exit!=0}`->PC13/`on_error`; `Timeout`->PC1/PC6/PC10(b)/`failure_policy=FailClosed`), with an explicit note that the scenario assumes the plugin's steady-state `failure_policy=FailClosed` annotation (PC9), not `on_error=Block` alone. Also fixed PC13's own header, which grouped PC4/PC5/PC10 together as "on_error-vs-Crashed coverage" (imprecise — PC5/PC10 are `Timeout`/`failure_policy` axes-independence coverage, not crash coverage); split into "PC4's on_error-governs-crash coverage" and "PC5/PC10's failure_policy-governs-Timeout axes-independence coverage." Sites examined (complete list): H1; Description; Preconditions 1-5; PC1-PC13 (all bodies, including PC13's Coverage Set table); Invariants 1-11; Edge Cases EC-001 through EC-011; Canonical Test Vectors (all rows); Related BCs; Architecture Anchors (all bullets); Story Anchors; VP Anchors; Verification Properties (VP-TBD row); Traceability (all rows). Sites rewritten: Invariant 10 (full rewrite + retitle), Architecture Anchors `executor.rs` bullet closing sentence, Traceability ADR row §AMD-003 closing clause, EC-011 post-wiring-fix clause, PC13 header's PC4/PC5/PC10 grouping label. All other sites verified internally consistent with the narrow additive-only / axes-independent predicate. Sibling-swept BC-1.03.018 (TD-VSDD-060): grepped for "strict superset", "exactly as before", "governed solely by", and `Crashed \| Timeout` — no occurrence found; no BC-1.03.018 edit needed. No AC in this BC restates the broad predicate (this BC has no `## Acceptance Criteria` section — ACs live in the S-21.11 story body, story-writer's domain, not swept here). POLICY 8 parity preserved: this burst does not touch frontmatter `behavioral_contracts`/`bcs` arrays, a body BC-table, ACs, or Token Budget — only `## Invariants`, `## Edge Cases`, `## Architecture Anchors`, and `## Traceability` prose changed. No PC renumbering; corrective-only. Does NOT touch ADR-039 (architect's domain, already correct via E-005), BC-1.03.018, the S-21.11 story body/ACs (story-writer's domain — v1.16->v1.17 cite sweep is story-writer's next dispatch), or any INDEX/STATE.md (state-manager's domain). The S-21.11 story's and BC-1.03.018's input-hashes (which declare this BC as an input) will go stale as a result of this content change and are flagged for state-manager reconciliation same-burst. BC-1.03.017 v1.17. |
| v1.16 | 2026-08-19 | product-owner | S-21.11 v2.3 PRE-TDD spec-convergence cascade remediation of F-S2111V2-P3-001 (HIGH): swept the two remaining BC-body sites that still carried the REJECTED broad-negation form of the §AMD-003 fail-closed predicate (`on_error == OnError::Block AND result is NOT PluginResult::Ok { exit_code: 0, .. } => block`) after the architect narrowed ADR-039's authoritative §AMD-003 rule to its correct form in ADR-039 v1.13 / Erratum E-005. PC13's body already asserted the CORRECT narrow form (`Ok { exit_code, .. } where exit_code != 0`) — that text was untouched. Site 1 (Architecture Anchors, `executor.rs` bullet, "PC13 extension" clause): rewritten to the narrow non-zero-exit-`Ok`-only form, with an explicit MUST-NOT-be-a-negation clause and a restatement of the UNCHANGED base `Crashed | Timeout` rule (governed solely by `on_error`; a `Timeout` blocking under `failure_policy = FailOpen` is decided exclusively by the `failure_policy` axis per PC5/EC-009/Invariant 1, never by `on_error` alone). Site 2 (Traceability ADR row, §AMD-003 citation): rewritten to cite the narrow form and to reference ADR-039 v1.13 / Erratum E-005, with an inline note explaining why the broad negation was wrong (it wrongly captured `Timeout{Fuel\|Epoch}` and `Crashed`, reintroducing the CWE-636 self-lock this story prevents). Verified PC13's own body, PC5, PC10(a), EC-009, and Invariant 1 (axes-independence) are internally consistent with the swept anchors — the whole BC now speaks ONE predicate. Sibling-swept BC-1.03.018 (TD-VSDD-060): no occurrence of the broad-negation pattern or any AMD-003/`PluginResult::Ok` reference found there; no BC-1.03.018 edit required. No AC in this BC restates the broad predicate (this BC has no `## Acceptance Criteria` section — ACs live in the S-21.11 story body, story-writer's domain, not swept here). POLICY 8 parity preserved: this burst does not touch frontmatter `behavioral_contracts`/`bcs` arrays, a body BC-table, ACs, or Token Budget — only Architecture Anchors and Traceability prose changed. No PC renumbering; corrective-only within PC13's existing Architecture Anchors and Traceability citations. Does NOT touch ADR-039 (architect's domain, already corrected), the S-21.11 story body/ACs (story-writer's domain), BC-INDEX.md/STORY-INDEX.md/ARCH-INDEX.md, or input-hashes (state-manager's domain — input-hash drift against ADR-039's own v1.13 content bump is expected and reconciled by state-manager same-burst). BC-1.03.017 v1.16. |
| v1.15 | 2026-08-19 | product-owner | S-21.11 v2.2 pre-TDD spec-convergence remediation of F-S2111V2-P2-004 (MEDIUM, human-directed production-grade full-coverage decision): PC13's `on_error=Block` + `Ok{exit_code!=0}` fail-closed rule was correctly written as a generic, plugin-name-independent predicate, but the BC's surrounding six-plugin-scoped enumerations (Preconditions 2/3, PC8/PC9, Invariants 7/8, Architecture Anchors) created an under-specified reading that PC13's coverage obligation was limited to the same six PC1–PC12 exhaustion-migration-targeted plugins. Verified `plugins/vsdd-factory/hooks-registry.toml` directly: 18 `on_error = "block"` `[[hooks]]` entries exist across 17 unique plugin names (`protect-secrets` registered twice, once per `tool = "^Bash$"` and once per `tool = "^Read$"`) — confirms the human-cited "18" figure exactly; no discrepancy found. Added a new PC13 "Coverage Set" enumeration (all 18 entries, tabulated with name/event/tool trigger) establishing PC13 applies to the FULL registry `on_error=Block` set, not a sampled subset, naming the registry as the authoritative source for future drift re-verification. Added new Invariant 11 codifying the full-registry-coverage rule as entry-count-agnostic (future `on_error=block` entries inherit PC13 automatically, no BC amendment required). Added one Canonical Test Vector summary row citing full-coverage. Added one Architecture Anchor bullet citing the 18-entry registry coverage set (distinct from the pre-existing six-plugin `failure_policy` scope anchor). Extended Traceability's Security row to note PC13's CWE-636 closure is registry-wide, not exhaustion-specific. H1 enriched with a full-registry-coverage clause per POLICY 7. No PC renumbering; additive-only within PC13's existing prose plus one new Invariant. Does NOT touch BC-1.03.018 (materially unaffected — the break-glass control is a distinct enforcement point), the S-21.11 story body/ACs (story-writer's domain, dispatched separately), ADR-039 (architect's domain), or BC-INDEX.md (state-manager's domain). BC-1.03.017 v1.15. |
| v1.14 | 2026-08-19 | product-owner | Product-owner self-flagged residual-inconsistency remediation: PC12's POSITIVE control (and its "AMD-002-wiring-fixed" Canonical Test Vector row) previously asserted the wiring fix's target outcome as `PluginResult::Timeout { cause: Epoch }` — WRONG, and contradicted by this BC's own v1.13-ratified PC13(a)/EC-011 mechanism trace (ADR-039 §AMD-003): an `exec_subprocess.rs::run()` wall-clock kill propagates via `run_bash_via_host`'s error map → `adapter_logic`'s `HookResult::Error` → `classify_trap`'s `Err(I32Exit(1))` arm → `PluginResult::Ok { exit_code: 1, .. }`, NEVER `Timeout { cause: Epoch }` (that variant is constructed only by `classify_trap` on a genuine guest `Trap::Interrupt`, which cannot fire while the guest is blocked inside the synchronous `exec_subprocess` host call). Corrected PC12's POSITIVE control prose and its Canonical Test Vector row to assert `PluginResult::Ok { exit_code: 1, .. }` (via `HookResult::Error`) at the observed ≈`timeout_ms` + ~5ms-poll-tolerance kill time, with an explicit `NEVER Timeout{cause: Epoch}` clause and the `classify_trap`/`Trap::Interrupt` distinction inline. Clarified PC12/PC13 as complementary, non-redundant guarantees: PC12 owns the kill-timing/wiring guarantee (kill fires at the registry's calibrated `timeout_ms`, not the hardcoded constant); PC13 owns the block-enforcement guarantee (the resulting `Ok { exit_code != 0 }` outcome is treated as fail-closed for `on_error=Block` entries). No PC renumbering; additive/corrective only within PC12's existing prose and test-vector row. Does NOT touch BC-1.03.018, the S-21.11 story, ADR files, ARCH-INDEX, or STORY-INDEX. BC-1.03.017 v1.14. |
| v1.13 | 2026-08-19 | product-owner | S-21.11 v2.0 adversarial pass-1 remediation (F-S2111V2-P1-001-mechanism-adjudication memo; ADR-039 §AMD-003 RATIFIED v1.11): new PC13 asserts the §AMD-003 rule — a plugin dispatched with `on_error = OnError::Block` whose outcome is `PluginResult::Ok { exit_code != 0, .. }` MUST be treated as a block (`block_intent=true`, exit 2), regardless of `failure_policy`; unifies F-001 (bash-adapter host-wall-clock timeout surfacing as `HookResult::Error` → exit 1, via `Err(codes::TIMEOUT)` → `run_bash_via_host`'s error map → `adapter_logic`'s `HookResult::error(...)` → `classify_trap`'s `Err(I32Exit(1))` arm → `PluginResult::Ok{exit_code:1}`) and F-005 (any other generic `HookResult::Error` exit path), both ruled in-scope per §AMD-003's F-005 ruling. POSITIVE control (`on_error=Block` + `Ok{exit!=0}` → block) + two NEGATIVE controls (`on_error=Block` + `Ok{exit==0}` → no block; `on_error=Continue` + `Ok{exit!=0}` → unaffected). New Invariant 10 codifies PC13 as a strict superset of the pre-existing `Crashed \| Timeout` rule (does not remove any existing block path). Three new PC13 Canonical Test Vector rows. EC-011 corrected (F-002): replaced the wrong "silent false clean-pass at 45s" pre-fix characterization with the accurate nondeterministic pre-fix behavior (`PluginResult::Ok{exit_code:1}` via `HookResult::Error` OR a guest-epoch `Timeout` race on control-return) and the post-fix guarantee (PC12 kill timing + PC13 `Ok{exit!=0}`→block together close every sub-case). Architecture Anchors extended: `executor.rs` bullet gets a PC13 extension clause; new bullet cites `crates/hook-sdk/src/result.rs::HookResult::exit_code`. Traceability ADR row extended with a new §AMD-003 citation alongside §AMD-001/§AMD-002. VP-TBD property extended with the PC13 rule and its unit test (`test_on_error_block_fails_closed_on_plugin_error_exit_code`). H1 enriched with the §AMD-003 leg and PC13 clause per POLICY 7. PC count extended PC1-PC12 → PC1-PC13 (additive-only; no renumbering). Does NOT touch S-21.11's story body/ACs (story-writer's domain, dispatched separately) and does NOT alter PC1-PC12's existing semantics. BC-1.03.017 v1.13. |
| v1.12 | 2026-08-19 | product-owner | S-21.11 expanded-scope BC coverage burst (orchestrator-directed, scoped to the AMD-002 runtime-wiring gap only): new PC12 asserts the RUNTIME behavior AMD-002 (RATIFIED v1.10) identified as unwired — for `legacy-bash-adapter.wasm`-hosted plugins, the actual bash-subprocess kill deadline (`exec_subprocess.rs::run()`) MUST equal the registry's calibrated `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS=60_000` constant in `run_bash_via_host`; POSITIVE control (short `timeout_ms` kills early) + NEGATIVE reference (current pre-fix 60s-regardless-of-config defect state) + new EC-011 (highest-risk silent-false-pass window: script duration between calibrated `timeout_ms` and the hardcoded 60s). New Precondition 5 states the config-vs-runtime assumption gap. PC9 amended (additive) with a PC12-dependency clause. New Invariant 9 codifies the config-vs-runtime wiring bifurcation. Two new PC12 Canonical Test Vector rows + one EC-011 vector. Architecture Anchors extended to cite `legacy-bash-adapter/src/lib.rs::run_bash_via_host` and `exec_subprocess.rs::run()`'s 5ms poll loop, explicitly distinguished from the unrelated wasmtime `EPOCH_TICK_MS`=10ms guest-epoch ticker (ADR-039 §Decision 4 v1.9 mechanism-precision correction). Traceability `L2 Capability` resolved `CAP-TBD` → `CAP-011` ("Enforce fuel and epoch budgets on plugin execution") with new S-7.01 Capability Anchor Justification row (capabilities.md §CAP-011 verbatim cite). ADR row cross-references BC-1.03.018 (new sibling BC) for the break-glass mechanism, which is NOT governed by this BC. PC count extended PC1-PC11 → PC1-PC12 (additive-only; no renumbering). Does NOT touch S-21.11's story body/ACs (story-writer's domain) or alter PC1-PC11's existing semantics. BC-1.03.017 v1.12. |
| v1.11 | 2026-08-19 | architect | Sibling-sweep citation update (TD-VSDD-060; parallel to ADR-039 v1.9→v1.10, same burst; two human decisions this session, POLICY 22 ratification-channel): Traceability ADR row updated — §AMD-002 now cites RATIFIED (2026-08-19, v1.10) instead of PROPOSED/NOT RATIFIED, with the corrected corroboration basis (ADR-039's own v1.8 §AMD-001 → v1.9 §Decision 4 mechanism-precision self-correction; the prior "ADR-025 §Decision 18" citation was wrong — that ADR concerns the unrelated factory-artifacts lock/lease decision — and is retracted) and the reframed blast radius (~37 legacy-bash-adapter.wasm-routed registry entries affected structurally, not only the five §Decision 2 plugins targeted by S-21.11's fail-closed flip); §Decision 3 break-glass citation redirected from named follow-up S-21.17 to S-21.11 (human decided S-21.11 is NOT split — it absorbs break-glass, per-plugin timeout_ms calibration, the AMD-002 wiring fix, and the gated fail-closed flip as one unified story); AMD-002's own named follow-up S-21.18 likewise redirected to S-21.11; intra-story ordering constraint noted (break-glass commit precedes or is atomic with the fail-closed-flip commit for validate-wave-gate-prerequisite and validate-pr-merge-prerequisites). Citation-only sweep: no PC/Precondition/Invariant/AC content altered; PC count unchanged at PC1-PC11; Stories row unaffected (already cited S-21.11, not S-21.17/S-21.18). BC-1.03.017 v1.11. |
| v1.10 | 2026-08-18 | architect | F-S2111-P13-001 research-corrections fold-in (parallel to ADR-039 v1.9): swept prose terminology "epoch axis"/"epoch-axis floor"/"epoch mechanism" → "host wall-clock timeout axis" throughout Preconditions 2/3, PC8, PC9, Canonical Test Vectors, Architecture Anchors, VP-TBD, and Traceability (ADR-039 §Decision 1's technical premise, independently research-validated, is that wasmtime's `epoch_interruption` feature — like fuel — cannot bound a host-blocking subprocess call; the correct label is a dispatcher/host-enforced wall-clock timeout). `timeout_ms` field name and `TimeoutCause::Epoch`/`Timeout{Epoch}` Rust code identifiers left unchanged (literal code, not prose). Traceability ADR row updated: §AMD-001 now cites RATIFIED (2026-08-18, v1.9) instead of PENDING; new §AMD-002 cite added (architect self-verification finding, PROPOSED / NOT RATIFIED — `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed `BASH_TIMEOUT_MS=60_000` constant independent of the registry's calibrated `timeout_ms`; named follow-up S-21.18, new, not yet authored); §Decision 3 v1.9 break-glass amendment cited (S-21.17, new, not yet authored). No PC/Precondition/Invariant semantics altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.10. |
| v1.9 | 2026-08-18 | architect | F-S2111-P13-001 remediation (scoped architectural precondition/PC correction; ADR-039 §Decision 1/2/3/4 v1.8 amendment): Precondition 2/3 bifurcated by plugin adapter class — native-WASM plugin (`validate-cross-site-correspondence`) calibrates `fuel_cap` per the original formula; the five `legacy-bash-adapter.wasm`-hosted plugins (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`) additionally calibrate `timeout_ms` per the new epoch-axis formula (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`) because their bash subprocess execution is invisible to the WASM fuel counter (ADR-042 §Decision 3 class (b)). PC8 extended with a parallel `timeout_ms` structural half-state assertion (TIMEOUT-POSITIVE-CONTROL / TIMEOUT-NEGATIVE-CONTROL added) for `legacy-bash-adapter.wasm` entries — `fuel_cap` sufficiency alone is no longer treated as complete calibration evidence for these five plugins. PC9 final-state criterion updated to require both axes per plugin's adapter class. New Invariant 8 codifies the axis-bifurcation principle and the self-lock consequence for the two PreToolUse `^Agent$` gates. Two new Canonical Test Vector rows added (PC8 `timeout_ms` POSITIVE/NEGATIVE controls). Architecture Anchors + VP-TBD + Traceability updated to cite ADR-039 v1.8 §AMD-001. PC count unchanged at PC1–PC11 (additive-only; no renumbering). **Residual product-owner BC-body edit noted:** this burst is scoped to the architectural precondition/PC correction only; it does NOT touch AC-to-PC narrative mapping in the S-21.11 story body (deferred to a post-ratification resume burst) and does NOT alter PC1–PC7/PC10/PC11's axes-independence or migration-window substance, which remain product-owner's domain for any further narrative refinement. BC-1.03.017 v1.9. |
| v1.8 | 2026-08-18 | product-owner | F-S2111-P11-001 remediation: extended PC10 to require deliberate revision (TD-VSDD-059) of BOTH the unit test `fail_closed_timeout_with_on_error_block` AND its integration-level mirror `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12, `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs`). TC-12 currently constructs `on_error=Block + failure_policy=FailOpen (registry default) + Timeout{Epoch}` and asserts `exit_code==2`; under the axes-independent semantics mandated by this BC (PC5/EC-009), that assertion is FALSE and must be revised to assert exit 0. TC-12 SHOULD additionally carry a `failure_policy=FailClosed → exit 2` arm for symmetric behavioral coverage (Invariant 6 / Envoy #38801 discipline). Both revisions MUST appear in the PR diff; deletion of either without an equivalent replacement is a TD-VSDD-059 paper-fix violation. Two TC-12 Canonical Test Vector rows added (`integration-mirror-fail-open`, `integration-mirror-fail-closed-symmetric`). Architecture Anchors updated to cite TC-12 and `full_stack_plugin_invocation.rs`. VP-TBD property updated to reference TC-12 integration mirror obligation and F-S2111-P11-001. PC count unchanged: PC1..PC11. PC10↔AC-011 mapping unchanged. |
| v1.7 | 2026-08-18 | product-owner | Adversary pass-8 remediation (two F-S2111-P8 findings): (1) F-S2111-P8-002 — raised Precondition 2 calibration corpus floor for `lessons.md` from ≥3000 to ≥4000 lines; aligns with D-442(e) hard limit (4000 lines) so Invariant 5's exit condition (calibration confirms `fuel_cap` sufficient for the hard limit) is structurally achievable; a 3000-line corpus structurally cannot confirm 4000-line sufficiency; framing at 3000 was numerically wrong given D-442(e) soft=3500/hard=4000. (2) F-S2111-P8-003 — updated PC10 sub-cases (a) and (b) from `cause: Fuel` to `cause: Fuel\|Epoch` for self-consistency with the PC10 header (which already states `Fuel\|Epoch`) and with epoch-parity requirement in PC6/AC-010; Canonical Test Vectors PC10a and PC10b rows updated to match. BC-1.03.017 v1.7. |
| v1.6 | 2026-08-17 | product-owner | Adversary pass-6 remediation (three F-S2111-P6 findings): (1) F-S2111-P6-002 — added LIVE-TREE-CONTROL (fourth control) to PC11: at Phase-4-complete the detector MUST run against actual `crates/factory-dispatcher/src/executor.rs` and return `enforcement_active = true`; closes CWE-636 false-green gap where a syntactically-wrong detector could pass all three synthetic controls yet be inert against real enforcement code; acceptable implementation mandates the POSITIVE-CONTROL snippet be a verbatim excerpt of the real `execute_tiers` block-decision site AND the detector returns `enforcement_active = true` on the live tree; PC11 controls preamble updated from "three controls" to "four controls"; Canonical Test Vectors LIVE-TREE-CONTROL row added; Architecture Anchors and VP-TBD updated to reference LIVE-TREE control and F-S2111-P6-002. (2) F-S2111-P6-003 — fixed PC11(c) VACUITY-CONTROL self-contradiction: removed "evaluated the annotation-check branch"/"correctly skipped" contradictory phrasing; rewritten to assert the detector's enforcement-detection logic ran and returned `EnforcementAbsent` (via explicit `detection_ran` / tri-state diagnostic), and that RED-emission was skipped as a consequence; Canonical Test Vectors VACUITY-CONTROL row updated to match. (3) F-S2111-P6-004 — corrected PC8 title to remove migration-window on_error=block ordering claim (ordering constraint is mechanically enforced by PC11, not PC8); added clarifying sentence in Symmetric half-state prohibition text cross-referencing PC11 as the authoritative mechanical gate for the ordering constraint; PC8's scope is now unambiguous: calibration gate only (no fail-closed without fuel_cap >= 50M). BC-1.03.017 v1.6. |
| v1.5 | 2026-08-17 | product-owner | Adversary pass-5 remediation (three F-S2111-P5 findings): (1) F-S2111-P5-001 — added POSITIVE/NEGATIVE/VACUITY non-vacuity controls to PC11 (parallel to PC8); controls structured as pure functions over injectable inputs (synthetic executor-source snippet + synthetic registry, NOT bound to live tree); POSITIVE-CONTROL: enforcement-active snippet + registry missing one of five on_error=block annotations → assert RED; NEGATIVE-CONTROL: enforcement-active snippet + all five annotated → assert PASS; VACUITY-CONTROL: enforcement-absent snippet → assert GREEN AND detector reached annotation-check branch (vacuous-GREEN distinguishable from real-GREEN); Canonical Test Vectors PC11 rows updated from 2 to 3 rows. (2) F-S2111-P5-002 — broadened PC11 enforcement-active detection signal from data-flow-coupled ("PluginOutcome carries failure_policy field AND execute_tiers consults it") to data-flow-independent ("any block-decision site in execute_tier/execute_tiers/helpers references .failure_policy value when deciding to block on Timeout outcome, however the data reaches it"); softened over-claimed "structurally impossible to merge" to "mechanically detectable at any single commit"; Architecture Anchors and VP-TBD updated to match. (3) F-S2111-P5-005 — EC-004 Case A now mandates S-21.13 (or named successor) MUST annotate validate-cross-site-correspondence failure_policy="fail-closed" once its O(n) fuel-ceiling algorithmic fix removes the excessive cap requirement (descope is timing deferral only, not permanent exemption); PC9 annotation-landing obligation clause added. BC-1.03.017 v1.5. |
| v1.4 | 2026-08-17 | product-owner | Adversary pass-4 remediation (three F-S2111-P4 findings — holistic PC11/AC-012 migration-window-gate axis closure): (1) F-S2111-P4-001 — decoupled PC11 enforcement-active detection from function name; replaced name-based `fn plugin_fail_closed(` pattern with data-anchored signal (`PluginOutcome` carrying `failure_policy: FailurePolicy` field + `execute_tiers` consulting it for block decisions); detection is name-independent and holds for both extend-in-place and introduce-a-replacement implementer paths; Canonical Test Vectors PC11 rows updated to match. (2) F-S2111-P4-002 — resolved EC-004/PC11 deadlock + mis-route: EC-004 now explicitly bifurcates on `on_error` value; for `on_error="block"` plugins EC-004 is NOT a valid descope path (annotate-within-S-21.11 OR block-the-flip are the only options); path (b) "record transient fail-open window" removed for on_error=block case; S-21.13 mis-route for on_error=block removed (S-21.13 is scoped to validate-cross-site-correspondence on_error=continue only); PC9 critical caveat updated to match; PC11 extended with EC-004 non-applicability clause for the five on_error=block plugins; Story Anchors S-21.13 annotation updated. (3) F-S2111-P4-003 — H1 enriched to include migration-window on_error=block completeness gate clause (POLICY 7: enrichment must go into H1, not live only downstream in story BC-table); BC-INDEX title cell must be swept to match (state-manager same-burst per POLICY 14 leg-5). |
| v1.3 | 2026-08-17 | product-owner | Adversary pass-3 remediation (two F-S2111-P3 findings): (1) F-S2111-P3-001 — reconciled 50M boundary to inclusive floor (>= 50_000_000 ACCEPT, < 50_000_000 REJECT) — atomic sibling sweep across PC8, PC9, Invariant 2, Invariant 7, Architecture Anchors, VP-TBD, and Canonical Test Vectors; POSITIVE-CONTROL fixture updated from fuel_cap=10_000_000 to fuel_cap=20_000_000 (factory default per ADR-042 §Decision 2, clearly below floor and realistic); added boundary-pass test vector asserting fuel_cap=50_000_000 PASSES (the calibration-formula minimum is now an inclusive ACCEPT). (2) F-S2111-P3-005 — PC11 added: hard migration-window completeness CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) asserting that if the extended 3-arg plugin_fail_closed signature is present in executor.rs, every on_error="block" targeted plugin MUST carry failure_policy="fail-closed"; closes the CWE-636 static-gap left by Invariant 7's ordering rule (which was ordering-based, not commit-checkable); PC11 test vector added. |
| v1.2 | 2026-08-17 | product-owner | Adversary pass-2 remediation (five F-S2111-P2 findings): (1) F-S2111-P2-001 — PC8 extended with symmetric half-state prohibition: no on_error=block targeted plugin may remain at failure_policy=fail-open once the extended 3-arg plugin_fail_closed is in effect; Invariant 7 added codifying migration-ordering atomicity and naming the five at-risk plugins. (2) F-S2111-P2-002 — PC10 added: fail_closed_timeout_with_on_error_block MUST be deliberately revised (TD-VSDD-059) to assert axes-independent sub-cases (FailOpen→NOT block, FailClosed→block); Canonical Test Vectors and Architecture Anchors updated. (3) F-S2111-P2-003 — EC-004 extended: deferral NOT behavior-neutral for on_error=block plugins (CWE-636 regression if left at fail-open); two remediation paths enumerated (fallback gate OR hard-blocker on follow-up); PC9 annotated with cross-reference to EC-004 on_error=block consequence. (4) F-S2111-P2-004 — PC8 extended with NEGATIVE-CONTROL fixture (fuel_cap=75_000_000, >50M floor → gate must PASS/not fire), closing POLICY 15 single-outcome-control gap; Canonical Test Vectors updated. (5) F-S2111-P2-006 — EC-004 names S-21.13 as concrete follow-up story anchor (Canonical Principle Rule 3); Story Anchors updated. |
| v1.1 | 2026-08-17 | product-owner | Spec-review remediation (F-S2111 adversary + SR findings): (1) F-S2111-P1-001 — HookEntry→RegistryEntry in Precondition 1, Related BCs BC-1.01.016 bullet, and Architecture Anchors registry.rs bullet (×2); phantom struct — actual is `pub struct RegistryEntry` in registry.rs. (2) F-S2111-P1-003/SR-001 — PC8 reclassified as standing regression/invariant gate (green-when-empty, green-at-final-state, RED on bad half-state); POSITIVE-CONTROL fixture requirement added; red-first framing removed; PC9/AC-009 is the genuine red-first gate. (3) F-S2111-P1-004/SR-008 — PC8 + Invariant-2 + VP gate threshold raised 20M→50M (calibration floor ADR-039 §Decision 4; factory default 20M per ADR-042 §Decision 2 is below the floor). (4) SR-002 — EC-004 vs PC9 deadlock resolved: explicit descoping-to-flippable-subset via orchestrator-approved spec amendment added to EC-004 and PC9; PC9 now conditional on post-amendment set. (5) SR-004 — PluginResult::Error→PluginResult::Crashed in PC4 and Canonical Test Vectors; no Error variant in invoke.rs enum (variants: Ok, Timeout, Crashed). (6) F-S2111-P1-008 — Precondition 2 fixture citation corrected: phantom S-21.07 task #33 replaced with committed path (BC-INDEX.md at 576,396 bytes). |
| v1.0 | 2026-08-06 | product-owner | Initial creation (S-21.10/S-21.11 BC authoring burst; ADR-039 §Decision 3+6 Phase 4 enforcement leg; four behavioral test scenarios from Decision 6 as PC1–PC6; structural half-state gate PC8; six targeted validators PC9; CWE-636+CWE-390 closure). |
