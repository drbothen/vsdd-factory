---
document_type: adr
adr_id: ADR-017
status: accepted
accepted_date: 2026-05-06
date: 2026-05-06
cycle: v1.0-feature-engine-discipline-pass-1
subsystems_affected: [SS-04, SS-05]
supersedes: null
superseded_by: null
---

# ADR-017: Per-Story Adversarial Convergence Gate — Three-Perimeter Model and WASM Hook Phasing

## Context

### The three-perimeter adversarial review model

VSDD's adversarial review structure (ADR-013) operates across three perimeters:

1. **Per-story** — adversary reviews the story worktree diff, story spec, and
   anchored BCs. Produces findings scoped to the story under delivery.
2. **Wave-gate (Gate 3)** — adversary reviews integration and cross-story
   concerns across all stories in the wave.
3. **Phase-5** — whole-system adversarial review; novelty decay to zero; the
   most comprehensive and expensive perimeter.

### Implementation gap

The orchestrator `AGENT.md` (MANDATORY STEPS, line 117) asserts that per-story
adversary convergence is required before a story can be considered complete.
However, `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` does not
implement this step. Stories proceed from implementation (Step 4) directly to
demo recording (Step 5) without any per-story adversary gate.

This gap means:
- Within-story defects are deferred until wave-gate (expensive context reload)
  or Phase-5 (very expensive full-system review).
- Wave-gate is performing double duty as a within-story discovery pass AND a
  cross-story integration pass, diluting its cross-story focus.
- The AGENT.md assertion is aspirational but unenforceable — agents can and do
  proceed without per-story adversary convergence.

### OQ-3 resolution: per-story convergence state file

The convergence state file is a per-story JSON file at:

```
.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json
```

Schema:
```json
{
  "passes_clean": 0,
  "last_finding_count": null,
  "last_classification": null,
  "last_timestamp": null,
  "deferred_findings": []
}
```

Convergence criterion: `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.

This mirrors `regression-state.json` (SS-04 precedent) and is parseable by
a WASM hook without prose parsing.

### OQ-6 resolution: adversary agent owns convergence assessment

No new convergence-checker agent is created. The existing `adversary` agent
runs the per-story convergence loop. The adversary's final output includes an
updated convergence state JSON that is written to the per-story state file.
The convergence-checker role is subsumed into the adversary agent's contract.

---

## Decision

### Step 4.5 insertion in `per-story-delivery.md`

A new step is inserted between Step 4 (Implement) and Step 5 (Record demos):

**Step 4.5 — Per-Story Adversary Convergence Loop**

1. Dispatch the `adversary` agent against the story worktree diff, story spec,
   and anchored BCs only.
2. Adversary classifies each finding as CRITICAL, HIGH, MEDIUM, LOW, or NITPICK.
   Findings that are cross-story, integration-level, architectural, or system-
   level are TAGGED as DEFERRED and written to `deferred_findings[]` in the
   convergence state file. They do NOT block per-story convergence.
3. Implementer applies fixes for within-story findings. Loop repeats from
   step 4.5 start.
4. Convergence: three consecutive passes with `last_classification == "NITPICK_ONLY"`
   (no within-story CRITICAL, HIGH, or substantive MEDIUM findings).
5. Adversary writes final convergence state JSON to
   `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
6. Proceed to Step 5 (demo recording).

### Demo recording placement rationale

Step 5 (demo recording) remains AFTER Step 4.5 (adversary convergence). Demos
capture the final converged state — not an intermediate pre-convergence state.
Recording before convergence would require re-recording when adversary-driven
fixes are applied. One recording of the accepted state is the correct invariant.

### Three-perimeter scope contracts

Each perimeter has a defined and non-overlapping scope:

**Per-story perimeter (Step 4.5):**
- Scope: story worktree diff + story spec + anchored BCs
- Finds: within-story logic errors, spec-implementation gaps, BC postcondition
  violations localized to the story's own artifacts
- Cross-story / integration / architectural findings: TAGGED DEFERRED, written
  to `deferred_findings[]`, do NOT block per-story convergence

**Wave-gate perimeter (Gate 3):**
- Scope: integration and cross-story concerns only
- Assumes: all constituent stories have passed per-story convergence (enforced
  by WASM hook prerequisite — see below)
- Finds: interface mismatches between stories, cross-cutting invariant violations,
  dependency ordering errors
- Within-story concerns: out of scope (assumed converged)

**Phase-5 perimeter:**
- Scope: whole-system adversarial review; novelty decay to zero
- Behavior: unchanged from current implementation

### WASM hook `validate-per-story-adversary-convergence`

A WASM hook enforces the wave-gate prerequisite: no story may proceed to
wave-gate dispatch until its per-story convergence state file exists and
satisfies the convergence criterion.

Hook fires on: `SubagentStop` for convergence-checker (and equivalent) agent
dispatch at wave-gate. The hook reads each story's convergence state file and
blocks wave-gate if any story's file is missing OR if
`passes_clean < 3` OR if `last_classification != "NITPICK_ONLY"`.

Block message format (HOST_ABI.md Why/Fix/Code convention):
```
BLOCKED by validate-per-story-adversary-convergence: Story <story-id>
has not reached per-story adversary convergence (passes_clean=N,
last_classification=<value>). Fix: Complete the per-story adversary loop
(Step 4.5 of per-story-delivery.md) for this story before wave-gate dispatch.
Code: per_story_convergence_not_reached.
```

Implementation path:
`/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-per-story-adversary-convergence/`

Build output:
`plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm`

### Phased story delivery: Story A before Story B

The workflow and agent contract (Story A) ships before the WASM hook (Story B).
This phasing is deliberate:

- Story A establishes the behavioral contract and the convergence state file
  schema that Story B's hook depends on.
- The workflow gate operates via workflow precondition in Story A, providing
  enforcement even before the WASM hook ships.
- Story B adds machine-enforceable gate integrity but does not change the
  behavioral contract.

---

## Rationale

### Why insert at Step 4.5 (between Implement and Demo)?

Context is hottest immediately after implementation. Within-story defects are
cheapest to fix when the implementer has full context: the worktree is active,
the spec is loaded, the BC invariants are fresh. Deferring to wave-gate means
re-loading context for a story that was "done."

### Why adversary agent (not a new agent)?

The adversary agent already has the behavioral contract for finding classification
and convergence assessment. A new convergence-checker agent would duplicate that
contract with a different name. The `deferred_findings[]` tagging mechanism is
the adversary's job, not a separate coordination role.

### Why a per-story state file (not STATE.md)?

A dedicated JSON file at a predictable per-story path is machine-parseable by
a WASM hook without prose parsing. The WASM hook cannot parse a section of
`STATE.md` reliably. The per-story file mirrors `regression-state.json` — an
established SS-04 pattern that is already in the registry.

### Why mandatory 3-pass minimum?

Three consecutive clean passes is the existing VSDD convergence criterion
(ADR-013). Per-story convergence uses the same criterion to maintain a uniform
standard across all three perimeters. A lower bar would create an asymmetry
where per-story convergence is easier to claim than wave-gate or Phase-5
convergence.

---

## Subsystem Assignments

**SS-04 (Plugin Ecosystem):** Referencing SS-04 because
`validate-per-story-adversary-convergence` is a new WASM plugin in the
`crates/hook-plugins/` ecosystem. Its state file schema (OQ-3) is an SS-04
data contract.

**SS-05 (Pipeline Orchestration):** Referencing SS-05 because Step 4.5 is an
insertion into `plugins/vsdd-factory/workflows/phases/per-story-delivery.md`
— an SS-05 Lobster workflow. The wave-gate prerequisite enforcement is also
an SS-05 pipeline concern. AGENT.md reconciliation (adversary, story-writer,
convergence-checker roles) affects SS-05 agent contracts.

---

## Alternatives Considered

### (a) New convergence-checker agent

Rejected per OQ-6. The adversary agent already owns finding classification and
convergence determination. A separate agent would require a new behavioral
contract, duplicate the finding taxonomy, and add a new dispatch step without
any new capability.

### (b) Reuse STATE.md for convergence state

Rejected per OQ-3. `STATE.md` is prose with structured sections. A WASM hook
requires a machine-readable format. The per-story JSON file is the correct
precedent (cf. `regression-state.json`).

### (c) Wave-gate-only enforcement (no per-story gate)

Rejected. Wave-gate already performs integration-level review; adding within-
story review at wave-gate scope dilutes its cross-story focus and forces context
reload for stories that were "done." The cost-per-defect is lower at per-story
time than at wave-gate time.

### (d) Per-story gate without WASM enforcement

Partially accepted for Story A (workflow precondition only). Fully rejected as
a final state: without machine enforcement, the gate can be bypassed under time
pressure. Story B (WASM hook) provides the mechanical floor.

---

## Consequences

- **Every story now goes through minimum 3 adversary passes before demo.** This
  adds latency to per-story delivery, traded against catching within-story
  defects when context is hot and the worktree is active.
- **Wave-gate becomes a confirmation pass** for cross-story and integration
  concerns, not a discovery pass for within-story errors. Gate 3 review scope
  is narrowed and focused.
- **`deferred_findings[]` accumulates cross-story findings** across per-story
  passes. The wave-gate adversary begins with this feed as input context.
- **AGENT.md reconciliation is required** for adversary, story-writer, and
  (formerly separate) convergence-checker roles to align with the three-perimeter
  model.
- **Per-story state files are new `.factory/` artifacts.** They must be added
  to `artifact-path-registry.yaml` (ADR-016) as part of Story A delivery.

---

## Verification Properties

- VP-071: `validate-per-story-adversary-convergence` block invariant — hook
  MUST block when any story's state file is missing OR `passes_clean < 3` OR
  `last_classification != "NITPICK_ONLY"` (kani, P1)

See `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/`
for full VP definition.

---

## Decision Log Reference

| Decision | ID | Rationale |
|----------|----|-----------|
| Cycle opened with per-story adversary cluster | D-336 | AGENT.md vs per-story-delivery.md gap identified |
| WASM-only hooks for this cycle | D-337 | No new Bash hook debt |
| 3-story decomposition | D-339 | Story A (workflow) + Story B (WASM hook) phasing |
| OQ-3: per-story state file path and schema | F2 OQ-3 | Machine-parseable per-story JSON |
| OQ-6: adversary agent owns convergence | F2 OQ-6 | No new convergence-checker agent |
