---
document_type: adr
adr_id: ADR-044
version: "1.0"
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
input-hash: "8136896"
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

## Source / Origin

- **Adversarial finding:** F-S2119-P1-001 (BLOCKER), S-21.19 pre-TDD adversarial convergence,
  brownfield cycle `v1.0-brownfield-backfill`.
- **Behavioral contract:** BC-1.03.017 v1.18, Invariant 7 + Postcondition 11 (PC11).
- **Extends:** ADR-039 §Decision 3 (atomicity requirement this ADR refines for the
  split-topology delivery context; ADR-039's own single-story atomicity guidance is unchanged
  for non-split deliveries).
- **Decomposition plan:** `.factory/planning/S-21.11-decomposition-plan.md` §8 (full per-story
  task/AC change-list applying this decision).
