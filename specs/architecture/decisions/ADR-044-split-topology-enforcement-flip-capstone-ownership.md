---
document_type: adr
adr_id: ADR-044
version: "1.2"
title: "ADR-044: Split-topology flip-sequencing — the enforcement-active wiring commit is capstone-owned, not core-decision-story-owned, when an atomicity-critical story is partitioned across independently-mergeable sub-stories"
status: ratified
date: 2026-08-20
producer: architect
deciders:
  - architect
subsystems_affected: [SS-01]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
extends: ADR-039 §Decision 3
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/stories/S-21.19-executor-decision-function-core.md
  - .factory/planning/S-21.11-decomposition-plan.md
modified:
  - "2026-08-20 (v1.0) — created by architect adjudicating BLOCKER split-integrity finding F-S2119-P1-001 (S-21.19 pre-TDD adversarial convergence, brownfield cycle v1.0-brownfield-backfill): the 6-seam split of CONVERGED S-21.11 placed the executor's enforcement-active wiring in S-21.19 (wave 6) while the 5 on_error=block plugins' fail-closed annotations landed downstream in S-21.21/S-21.23/S-21.24 (waves 7-8), opening a multi-wave CWE-636 fail-open window on every commit of `develop` between S-21.19's merge and S-21.24's merge — contradicting BC-1.03.017 v1.18 Invariant 7 + PC11's mechanical-un-mergeability guarantee. This ADR records the resolution: the wiring event is capstone-owned. Full per-story change-list lives in `.factory/planning/S-21.11-decomposition-plan.md` §8."
  - "2026-08-22 (v1.1) — architect adjudicating HIGH finding F-S2121-P1-002 (S-21.21 pre-TDD adversarial pass-1, Wave-7, same split lineage): this ADR's v1.0 ruling bundled TWO orthogonal predicates — the failure_policy/Timeout exhaustion axis (§Decision 3 Phase 4 of ADR-039, references `.failure_policy`, trips PC11) and AMD-003's on_error==Block && Ok{exit_code!=0} error-exit axis (PC13, references neither `.failure_policy` nor `Timeout`, trips nothing in PC11) — into one function and deferred BOTH legs' live wiring to S-21.24 (wave 8). This manufactured a false wave-8 dependency for S-21.21's own AC-013b/AC-013c end-to-end legs, which need only the error-exit axis wired live. RATIFIED 2026-08-22 by human, this session, via orchestrator relay of the architect's three-question decision memo (Q1, Option A): reopen S-21.19 and wire the error-exit (PC13) axis live at wave 7, owned by S-21.21; the exhaustion axis's deferral to S-21.24 is UNCHANGED and remains this ADR's core ruling. See Addendum below."
  - "2026-08-22 (v1.2) — architect fixing a self-contradiction in the v1.1 Addendum, caught
    independently by two fresh-context adversary passes (S-21.19 R1 F-001 HIGH,
    S-21.21 P2 F-001 HIGH), both dispatched after v1.1 landed at commit a3bfa1af: Addendum
    Decision item 2 stated `plugin_fail_closed_on_error_exit` \"Governs the existing
    `on_error==Block && (Crashed | Timeout)` case ... EXTENDED ... to also cover
    `Ok{exit_code!=0}`\" — i.e. it INCLUDED `Timeout` in the error-exit leg's scope. This directly
    contradicted the SAME Addendum's own qualifier three lines later (\"references neither
    `.failure_policy` nor `Timeout`\") and the authoritative BC-1.03.017 v1.20 Architecture
    Anchors + decomposition-plan §8.7, both of which scope the error-exit function to
    `Crashed | Ok{exit_code!=0}` ONLY — `Timeout {..}` is governed EXCLUSIVELY by
    `plugin_fail_closed_on_exhaustion` via the `failure_policy` axis, never via the error-exit
    function. An implementer following item 2's literal (Crashed | Timeout) wording would carry
    the `Timeout` arm into `plugin_fail_closed_on_error_exit`, OR-combine it with `on_error` alone
    at the wired call site, and regress the axes-independence invariant this same document (and
    ADR-039 §Decision 1/§Decision 6 test #4/§AMD-003 condition 1) already establishes: a
    fuel-exhausted plugin with `failure_policy=FailOpen` and `on_error=Block` would wrongly BLOCK.
    Corrected item 2's scope to `on_error==Block && Crashed` (unchanged, live today) EXTENDED to
    `Ok{exit_code!=0}` (PC13) — `Timeout` explicitly and unconditionally excluded, matching
    BC-1.03.017 v1.20 + plan §8.7. No change to the exhaustion leg (item 1) or to any other
    ADR-044 ruling. Does not require separate POLICY 22 re-ratification: this restores internal
    consistency with content the v1.1 Addendum, ADR-039, and BC-1.03.017 v1.20 already
    established — it introduces no new decision, only corrects a wording defect that
    contradicted already-ratified content (same non-re-ratification category as ADR-039's
    E-001..E-007 erratum precedent). Adjudicates F-S2119-R1-001, F-S2121-P2-001. ADR-044 v1.2."
input-hash: "0acab83"
---

# ADR-044: Split-topology flip-sequencing — the enforcement-active wiring commit is capstone-owned, not core-decision-story-owned, when an atomicity-critical story is partitioned across independently-mergeable sub-stories

## Context

ADR-039 §Decision 3 establishes that the executor's fail-closed enforcement flip and the
per-plugin `failure_policy = "fail-closed"` registry annotations for the five
`on_error = "block"` targeted plugins must be atomic — no commit tree may exist where
enforcement is active and any of the five plugins remains unannotated. BC-1.03.017 v1.18
Invariant 7 states this as the human-readable policy; PC11
(`test_no_on_error_block_without_fail_closed_when_3arg_executor`) makes it a
"design-flow-independent," mechanically un-mergeable static gate: it scans
`crates/factory-dispatcher/src/executor.rs`'s actual block-decision chain (`execute_tier`,
`execute_tiers`, or their helpers) for ANY site that references `.failure_policy` when
deciding to block on a `Timeout` outcome — regardless of how the data reaches that site — and,
if such a site is present, requires all five targeted plugins to already carry the annotation.

S-21.11 (v2.11, 32 pts, 16-pass 3-CLEAN BC-5.39.001 convergence) was the single unified story
that delivered both halves of this atomic pair together. On 2026-08-20 the operator directed a
6-way split of S-21.11 into independently-mergeable sub-stories (`.factory/planning/S-21.11-decomposition-plan.md`),
placing the executor decision-function extension in S-21.19 (wave 6, the DAG's sole
gate for four parallel downstream seams) and the five plugins' annotations in S-21.21
(3 plugins, wave 7, gated on its own AMD-002 bash-adapter wiring fix), S-21.23 (break-glass
prerequisite, wave 7), and S-21.24 (the capstone, wave 8, which performs the actual annotation
commits for all five plugins using S-21.21's calibration).

S-21.19's own Task 5 wired the extended decision function directly into the real
`execute_tiers`/`execute_tier` block-decision call site — including populating
`PluginOutcome.failure_policy` at all construction sites and exercising the extended predicate
end-to-end via a bats/integration test against a real dispatch (AC-002) and an integration-level
test revision (AC-011's `TC-12` mirror in `full_stack_plugin_invocation.rs`). Because PC11's
detector is a static, source-level scan of the real `executor.rs` — not a runtime/registry-scoped
check — the moment this wiring merged to `develop` in wave 6, the executor became
"enforcement-active" by PC11's own definition, while all five targeted plugins remained
unannotated until S-21.24 (wave 8). This opened a genuine, multi-wave, multi-PR CWE-636
fail-open window on `develop` — precisely the state Invariant 7 and PC11 exist to make
mechanically impossible. S-21.19's own Task 5 "ATOMICITY GATE" note asserted the intermediate
state was "vacuously satisfied" because "no plugin is annotated yet" — this is incorrect:
vacuous truth requires an empty quantified set, and the five-plugin set is fixed and
non-empty, so "zero of five annotated" makes PC11's universal claim FALSE, not vacuously true.
This defect was caught as BLOCKER F-S2119-P1-001 during S-21.19's pre-TDD adversarial
convergence pass and is adjudicated here.

## Decision

**The enforcement-active wiring event — the commit that makes any block-decision site in the
executor's real block-decision chain reference `.failure_policy` for a `Timeout` outcome — is
owned by the LAST-landing story in an atomicity-critical split, not by the story that authors
the underlying decision-function logic.**

Concretely for this split: S-21.19 authors and unit-tests the extended 3-arg
`plugin_fail_closed` function (or its replacement) and the `PluginOutcome.failure_policy` field
as a **standalone, callable, fully unit-tested pure function** — proven correct via direct
function-level tests that bypass `execute_tiers` entirely — but does **not** wire that function
into the real block-decision call site. Adding the field to `PluginOutcome` and populating it
at construction sites is retained in S-21.19: this is inert data plumbing, not a "block-decision
site... deciding to block," and does not trip PC11's detector. The actual wiring — replacing the
`execute_tiers`/`execute_tier` call site's 2-arg invocation with the 3-arg form that consults
`.failure_policy` — moves to **S-21.24**, the capstone, which by construction cannot land before
all five plugins are already annotated (S-21.24 `depends_on: [S-21.19, S-21.20, S-21.21, S-21.22, S-21.23]`).
The two integration-level tests that require the real wiring to exist — AC-002's bats
fixture-registry dispatch and AC-011's `TC-12` integration mirror in
`full_stack_plugin_invocation.rs` — move with the wiring: they are authored and confirmed GREEN
in S-21.24, not S-21.19. This follows the same "author pure-function controls early, confirm the
live-tree control at the capstone" pattern the story itself already established for AC-009 and
three of AC-012's four controls.

This makes "annotate before flip" hold by wave-schedule construction — no same-commit
choreography is required. At no commit in the merged history does PC11 ever observe
"enforcement-active AND any of the five plugins unannotated," because the wiring commit that
makes enforcement active does not exist until the same story that also confirms all five
annotations are already present in the live registry.

**This v1.0 ruling is refined, not superseded, by the Addendum below — read both together.**
The Addendum splits "the wiring event" into two independently-timed events; the ordering
guarantee stated in this paragraph is preserved in full for the exhaustion axis specifically.

## Rationale

Three resolutions were evaluated (see Alternatives). The chosen resolution is the only one that
satisfies all of: (1) CLAUDE.md's Canonical Principle Rule 1 — no MVP-style deferral of a
security invariant; (2) BC-1.03.017 v1.18 Invariant 7 + PC11, unweakened — this decision does
not relax PC11's design-flow-independent detection; it changes WHICH story's commit trips it;
(3) every sub-story remains independently mergeable, with no story landing in a state where its
own required gate (PC11, AC-012) blocks its own merge; (4) the six-seam wave/dependency
topology the operator already approved is preserved unchanged — only intra-story task/AC
ownership shifts between two of the six existing seams (S-21.19 and S-21.24), so no new
human re-approval of the split shape itself is required, only awareness of the scope
redistribution.

PC11 is deliberately "design-flow-independent" specifically so an implementer cannot dodge it
by routing the `.failure_policy` read through an indirect path. This decision does not
circumvent that design: S-21.19's function is never called from the real block-decision chain
at all until S-21.24, so there is no "genuine, working, enforcement" hidden from PC11 while it
runs on `develop` — a state PC11's own construction rules out as impossible to achieve cleanly
via indirection or flags. Attempting to keep the wiring in S-21.19 behind a runtime flag would
either (a) still trip PC11's static source scan (the reference would still be present in the
block-decision chain's source, flag or not), or (b) require weakening PC11's own detection logic
to ignore flagged code — which would reopen exactly the gaming vector PC11 was built to close.
Moving the wiring event itself, rather than trying to hide it, is the only option that leaves
PC11's guarantee fully intact.

## Consequences

### Positive

- No commit in `develop`'s history — from S-21.19's merge through S-21.24's merge — has the
  executor enforcement-active with any of the five targeted plugins unannotated. The CWE-636
  window this split risked opening does not exist at any point.
- PC11's design-flow-independent detector is preserved at full strength; no exception, flag, or
  scope-narrowing was introduced to accommodate the split.
- The six-seam wave schedule and dependency graph are unchanged — S-21.19 remains wave 6, gates
  S-21.20/21/22/23; S-21.24 remains wave 8, strictly last. No new human re-approval of the split
  shape is required.
- The pattern (pure-function logic authored early; live-tree wiring + confirmation owned by the
  capstone) already had precedent within S-21.19 itself for AC-009 and 3-of-4 AC-012 controls;
  this decision generalizes that existing pattern rather than inventing a new one.

### Negative / Trade-offs

- S-21.19 no longer delivers an end-to-end, real-dispatch demonstration of the extended
  decision function within its own PR — that demonstration (AC-002's bats leg, AC-011's TC-12
  mirror) is deferred to S-21.24. Reviewers of S-21.19's PR see unit-level proof only; the
  end-to-end proof arrives two waves later.
- S-21.24 gains a new task (the wiring commit itself) and two test legs it did not previously
  own, increasing its scope and point estimate modestly (S-21.19 estimated down from 9 to ~7
  points; S-21.24 estimated up from 3 to ~5 points — net split-total unchanged at ~12).
  **Superseded by the Addendum (v1.1) below for the error-exit (PC13/AMD-003) leg specifically:**
  that leg's wiring and its end-to-end proof move to S-21.21 instead of S-21.24; the point-shift
  figures above now describe the exhaustion axis alone.
- Implementers must take care that S-21.19's own unit tests, which call the extended 3-arg
  function directly, are written in a way a scoped (non-test-body) detector correctly excludes
  from PC11's block-decision-chain scan — an existing discipline already required by the story's
  own guidance on the `emit_lifecycle_timeout_carries_fuel_cap_and_consumed` false-match hazard,
  now load-bearing for the atomicity guarantee, not just for detector-accuracy.

### Status as of v1.0

In effect for the S-21.19/S-21.24 split-story pair as of this ADR's ratification. Evidence:
`.factory/planning/S-21.11-decomposition-plan.md` §8 records the full per-story task/AC
change-list; story-writer applies it to the S-21.19 and S-21.24 story files as a follow-up
dispatch.

## Addendum (v1.1, 2026-08-22) — Split the wiring event itself into two orthogonal, independently-timed legs; the error-exit (PC13/AMD-003) leg is wireable at wave 7, owned by S-21.21

### Context

S-21.21 pre-TDD adversarial pass-1 raised HIGH finding F-S2121-P1-002: AC-013b/AC-013c require
a real dispatch against `legacy-bash-adapter.wasm` to produce `block_intent=true`/exit 2 when a
bash script exceeds `timeout_ms`, i.e. `PluginResult::Ok { exit_code: 1 }` under
`on_error=Block`. Per this ADR's v1.0 ruling, the ENTIRE extended `plugin_fail_closed` — both the
`failure_policy`-consulting exhaustion leg and AMD-003's `on_error==Block && Ok{exit_code!=0}`
error-exit leg (PC13) — is authored as one bundled, unwired standalone function in S-21.19, with
ALL of its live wiring deferred to S-21.24 (wave 8). Under that bundling, S-21.21's AC-013b/c
cannot pass at wave 7: no real dispatch can produce `block_intent=true` on `Ok{exit_code:1}`
until S-21.24 lands.

Independent review (architect adjudication, cross-story decision memo Q1, cycle
`v1.0-brownfield-backfill`) found this coupling to be an artifact of bundling, not a genuine
architectural constraint. The v1.0 ruling's rationale — PC11's static scan of the real
block-decision chain fires on ANY site referencing `.failure_policy` for a `Timeout` outcome,
and the annotate-before-flip half-state hazard this ADR exists to prevent — applies ONLY to the
`failure_policy`/`Timeout` exhaustion leg. AMD-003's error-exit leg (`on_error==Block &&
PluginResult::Ok { exit_code != 0 }`, per ADR-039 §AMD-003 condition 2) references neither
`.failure_policy` nor `Timeout` and consults no per-plugin calibration/annotation field at all —
it is unconditional over the fixed, already-live `on_error=block` registry state. Wiring it live
trips nothing in PC11's detector, and there is no partially-annotated-fleet hazard for it to
protect against: unlike the exhaustion leg (which requires calibrated `fuel_cap`/`timeout_ms`
values to exist before the flip is safe — the actual hazard this ADR's v1.0 ruling addresses),
the error-exit leg's correctness does not depend on any plugin's calibration state.

### Decision

**The single "wiring event" this ADR's v1.0 Decision described is split into two independently-
timed legs, each retaining its own atomicity analysis:**

1. **Exhaustion leg — `plugin_fail_closed_on_exhaustion(result, on_error, failure_policy)`.**
   Governs `Timeout { cause: Fuel | Epoch }` with `failure_policy == FailClosed` (ADR-039
   §Decision 3 Phase 4). References `.failure_policy`; trips PC11; carries the annotate-before-
   flip half-state hazard this ADR's v1.0 ruling exists to prevent. **UNCHANGED from v1.0: wiring
   remains capstone-owned, deferred to S-21.24 (wave 8), which by construction cannot land before
   all five plugins are annotated.**
2. **Error-exit leg — `plugin_fail_closed_on_error_exit(result, on_error)`.** Governs the existing
   `on_error==Block && Crashed` case (already live today, unchanged) EXTENDED per AMD-003 to also
   cover `on_error==Block && Ok { exit_code != 0 }` (PC13). **`Timeout {..}` is OUT OF SCOPE for
   this function, unconditionally — it is governed exclusively by the exhaustion leg above, via
   the `failure_policy` axis, never by `on_error` in isolation** (this is the same
   axes-independence invariant ADR-039 §Decision 1/§Decision 6 test #4 and §AMD-003's own Precise
   Rule condition 1 already establish; see the v1.2 correction note below). This function
   references neither `.failure_policy` nor `Timeout` at all; trips nothing in PC11; has no
   annotated-fleet precondition. **NEW in this Addendum: wireable immediately, owned by S-21.21,
   not S-21.24.**

**Correction (v1.2, 2026-08-22).** Item 2 above originally (v1.1) read
"`on_error==Block && (Crashed | Timeout)` case ... EXTENDED ... to also cover
`Ok{exit_code!=0}`" — wording that included `Timeout` in the error-exit leg's scope, directly
contradicting this same item's own next sentence ("references neither `.failure_policy` nor
`Timeout`") and BC-1.03.017 v1.20 Architecture Anchors + decomposition-plan §8.7, both of which
scope `plugin_fail_closed_on_error_exit` to `Crashed | Ok{exit_code!=0}` ONLY. Caught
independently by two fresh-context adversary passes (S-21.19 R1 F-001, S-21.21 P2 F-001, both
HIGH) after v1.1 landed. Corrected in place above: item 2 now reads `Crashed` (not
`Crashed | Timeout`) as the pre-existing leg, with `Timeout` stated as explicitly and
unconditionally out of scope for this function. See §Erratum-equivalent note in the frontmatter
`modified` history (v1.2 entry) for the full before/after diff and non-re-ratification reasoning.

S-21.19 Task 6 is narrowed (not expanded) to author these as two separate standalone functions
rather than one bundled 3-arg function — `PluginOutcome.failure_policy` field-population and
both functions' unit tests remain in S-21.19 exactly as before; only the SHAPE (two functions,
not one) changes. S-21.19's own scope does not grow: this is a decomposition of already-planned
work, not new work. S-21.21 gains one new task: wire
`plugin_fail_closed_on_error_exit` into `execute_tiers`'s/`execute_tier`'s real block-decision
call site (replacing the current bare `plugin_fail_closed` call with the renamed/extended
function), plus AC-013b/AC-013c's already-planned end-to-end bats/dispatch legs, which now prove
themselves against a genuinely live call site at wave 7 instead of waiting for wave 8. S-21.24's
scope narrows correspondingly: it wires and proves ONLY the exhaustion leg; the error-exit leg's
end-to-end proof moves out of S-21.24 entirely (superseding the "S-21.24 gains ... two test legs"
Negative/Trade-offs bullet above, which described both legs undifferentiated).

### Rationale

This satisfies the same four criteria the v1.0 Decision's Rationale already established: (1) no
MVP-style deferral — the error-exit leg has no genuine dependency on wave 8, so deferring it
anyway would itself be an unjustified deferral under CLAUDE.md's Canonical Principle Rule 1; (2)
PC11 remains unweakened — nothing about this split loosens or special-cases PC11's detector; it
simply recognizes that PC11's detector was never triggered by the error-exit leg's code shape in
the first place; (3) every sub-story remains independently mergeable — S-21.21 gains a
self-contained wiring task with no new cross-story landing dependency; (4) the six-seam wave/
dependency topology is unchanged — S-21.19 remains wave 6, S-21.21 remains wave 7, S-21.24
remains wave 8; only intra-story task ownership shifts (S-21.19 → S-21.21 for one function's
wiring), which is the same class of change the v1.0 Decision already made between S-21.19 and
S-21.24 and required no new human re-approval of the split shape itself.

### Consequences (Addendum-specific, additive to v1.0's)

**Positive:** S-21.21's AC-013b/AC-013c become satisfiable at wave 7 as originally intended,
without waiting on S-21.24. No commit in `develop`'s history ever has the error-exit leg wired
while any registry state it depends on is inconsistent, because — unlike the exhaustion leg —
the error-exit leg depends on no per-plugin annotation state at all; it is safe to wire the
moment it exists as a proven, unit-tested function, which S-21.19 already delivers.

**Negative / Trade-offs:** S-21.19, already CONVERGED (3-CLEAN, BC-5.39.001), requires a narrow
spec-text amendment (splitting Task 6's one bundled function into two) and therefore a fresh
convergence pass on that narrowed scope — a real cost, explicitly flagged to the human as part of
the originating decision memo (Q1) and accepted as Option A. `execute_tiers`'s real
block-decision call site now has two call-sites/functions to reason about instead of one
temporarily-bundled one, until S-21.24 lands and both legs coexist permanently in their final
split form — this is a net simplification once landed (each function has a single, clear
concern) but a minor transitional complexity during waves 6-8.

### Ratification

**RATIFIED 2026-08-22 by human, this session, via orchestrator relay of the architect's
three-question decision memo (Q1, Option A: "reopen S-21.19 and wire PC13 at wave 7").** The
human's directive ("Human decided Q1 = Option A... Proceed to APPLY your drafted ADR changes
now") constitutes explicit sign-off on this Addendum as drafted, satisfying POLICY 22.
Not implemented in this burst — architect scope is spec-only; the S-21.19 Task 6 function-split,
S-21.19's reconvergence pass, and S-21.21's new wiring task + AC-013b/c re-anchor are follow-up
dispatches to story-writer and downstream implementer work. `.factory/planning/S-21.11-
decomposition-plan.md` §8 requires a corresponding update (story-writer/state-manager domain,
not applied by this ADR edit).

## Alternatives Considered

- **Fold the 5 plugins' calibration and annotation into S-21.19 (same-commit atomicity).**
  Rejected: the 3 non-Agent bash-adapter plugins' annotation requires S-21.21's own AMD-002
  wiring-fix and calibration to already exist to be meaningful (PC9's registry-complete vs.
  runtime-complete distinction), and the 2 Agent-gate plugins' annotation requires S-21.23's
  break-glass mechanism to already exist (BC-1.03.018 PC9). Folding all five in would require
  folding S-21.21's and S-21.23's entire scope into S-21.19 as well — effectively re-merging
  three of the six seams (9+9+7 = 25 points, well over the ~13-pt per-seam ceiling the split was
  designed to respect) and defeating the purpose of the split.
- **Leave the wiring in S-21.19 but hide it behind a feature flag or dormant code path.**
  Rejected: PC11's detector is deliberately design-flow-independent (a static scan of the real
  block-decision chain's source), so a flagged-but-present reference to `.failure_policy` would
  still trip it — or would require weakening PC11's own detection logic to special-case flagged
  code, which reopens the exact gaming vector PC11 exists to close. This is not a viable
  "dormant" state; it is either genuinely absent from the block-decision chain (this ADR's
  chosen resolution) or it trips the gate.
- **Accept the multi-wave window as a documented, tracked residual risk.** Rejected outright per
  CLAUDE.md's Canonical Principle Rule 1 (no MVP-style deferral of a security invariant) and
  Rule 3 (tech-debt-register entries require explicit human direction, a concrete future
  dependency, and a specific story/wave anchor — none of which apply to a live, already-detected
  CWE-636 window with a mechanical fix available in-scope).
- **(Addendum, v1.1) Keep the error-exit leg bundled with the exhaustion leg and defer both to
  S-21.24, accepting S-21.21's AC-013b/AC-013c as wave-8-dependent.** Rejected: this was the
  status quo the Addendum corrects. It would have required re-anchoring AC-013b/AC-013c's stated
  dependency to S-21.24 with no offsetting benefit — the error-exit leg carries none of the
  annotate-before-flip hazard that justifies deferring the exhaustion leg, so deferring it anyway
  is an unjustified MVP-style wait under CLAUDE.md's Canonical Principle Rule 1, not a genuine
  architectural necessity.

## Source / Origin

- **Adversarial finding:** F-S2119-P1-001 (BLOCKER), S-21.19 pre-TDD adversarial convergence,
  brownfield cycle `v1.0-brownfield-backfill`.
- **(Addendum, v1.1) Adversarial finding:** F-S2121-P1-002 (HIGH), S-21.21 pre-TDD adversarial
  pass-1, Wave-7 split lineage, brownfield cycle `v1.0-brownfield-backfill` — adjudicated via the
  architect's three-question cross-story decision memo (Q1), human-ratified 2026-08-22.
- **(Addendum, v1.1) Extends:** ADR-039 §AMD-003 (the error-exit predicate this Addendum splits
  out and schedules for early wiring).
- **Behavioral contract:** BC-1.03.017 v1.18, Invariant 7 + Postcondition 11 (PC11).
- **Extends:** ADR-039 §Decision 3 (atomicity requirement this ADR refines for the
  split-topology delivery context; ADR-039's own single-story atomicity guidance is unchanged
  for non-split deliveries).
- **Decomposition plan:** `.factory/planning/S-21.11-decomposition-plan.md` §8 (full per-story
  task/AC change-list applying this decision).
