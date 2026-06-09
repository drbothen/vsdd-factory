# Issue #133 — Intra-Phase Adversarial Passes After Architecture Artifacts + Fix-Bursts

**Date:** 2026-06-09
**Issue:** #133 — "feat(workflows): add intra-phase adversarial passes after architecture artifacts + fix-bursts"
**State:** OPEN, label: enhancement
**Reviewer:** research-agent (cluster: review/adversarial-quality)
**Repo:** vsdd-factory @ develop `82163b7f`

---

## Restated Question

The current pipeline runs adversarial passes at **phase boundaries** (Phase 1d spec review;
Phase 3 per-story + wave convergence; Phase 5 implementation review) but does **NOT** run an
adversarial pass when new content lands *within* a phase or during pre-phase prep — specifically
after a new architecture artifact is created, after a major rewrite, after a fix-burst, or at a
pre-phase "FULLY CONVERGED" gate. The motivating monocle case: a 700-line architecture artifact
passed consistency-validator, validate-brief, and state-manager close-out as "FULLY CONVERGED,"
then a fresh-context adversary pass found 13 substantive defects (won't-compile Rust, wrong
frontmatter field names, silent 7→3-field contract downgrade, placeholder code). The proposed
fix: add intra-phase adversarial checkpoints to the workflow files + orchestrator guidance + a
procedural-validator "complementarity warning" + a planted-defect test fixture.

---

## Codebase Grounding

### What already exists

1. **Phase-1d adversarial spec review** — `skills/phase-1d-adversarial-spec-review/SKILL.md`
   reviews the *complete* Phase 1 spec package (PRD + architecture + BCs + VPs + UX) with fresh
   context, includes a "Step 4b Cross-Document Sync Check" (consistency-validator after
   remediation, before re-review), and loops to convergence. This is a *phase-boundary* pass, not
   an after-each-artifact pass — exactly the gap the issue describes.

2. **Per-story Step 4.5 + wave convergence + Phase 5** — `workflows/phases/per-story-delivery.md`
   Step 4.5 (BC-5.39.001 loop, 3-CLEAN); `agents/adversary.md` three-perimeter contract
   (per-story / wave-gate / phase-5). All phase-boundary or post-implementation, not
   intra-spec-phase.

3. **Orchestrator already has the *principle* in prose, partially:**
   `agents/orchestrator/orchestrator.md` "Fresh-Context Consistency Audit at Every Gate"
   (lines 289–291): "At every phase gate (not just adversarial convergence), spawn
   consistency-validator with fresh context BEFORE human approval. The adversary catches defects
   WITHIN the perimeter it's shown; the consistency-validator checks whether the perimeter is
   right. 'Previously-converged' does NOT mean 'correct' — A project went from '50-pass converged'
   to '19 gaps found' in one fresh-context audit." **This is the issue's tertiary "complementarity
   warning" already half-codified** — but it dispatches the *consistency-validator*, not the
   *adversary*, and only at the *phase gate*, not after each artifact / fix-burst.

4. **Adversary "Partial-Fix Regression Discipline (S-7.01)"** — `agents/adversary.md` lines
   281–314: "For every adversarial pass after pass 1, you MUST explicitly verify that prior-pass
   fixes have fully propagated" (frontmatter→body, sibling files, prose references). This is the
   *content* of the issue's secondary "adversary post-fix-burst protocol" — but it's a within-pass
   axis, not a *scheduled* dispatch *after* a fix-burst.

5. **Process-gap tagging + Cycle-Closing Checklist** — adversary tags `[process-gap]`;
   orchestrator's Cycle-Closing Checklist (orchestrator.md 386–408) requires every process-gap to
   get a follow-up story or deferral. This is the routing rail the new checkpoints would feed.

6. **`.lobster` workflow files exist and are data-driven** —
   `workflows/phases/phase-1-spec-crystallization.lobster`, `phase-3-tdd-implementation.lobster`,
   `code-delivery.lobster`, etc. The orchestrator parses them via `bin/lobster-parse`. The issue's
   proposed edits (new `type: adversarial-review` steps with `depends_on` / `when:` guards) are
   structurally compatible with this format. (Note: present in main tree and the `.lazyclaude`
   worktree per Grep.)

### What does NOT exist (the genuine gap)

- No workflow step that triggers an adversary pass **after a new `.factory/specs/architecture/`
  artifact is committed** within Phase 1 (before the phase-1d package-level pass).
- No `when: fix_burst_commits >= 3` post-fix-burst adversary checkpoint in code-delivery /
  per-story-delivery.
- No "major content addition" heuristic trigger (>100 lines added, or version bump > patch).
- No pre-phase-prep ("FULLY CONVERGED" / "PHASE N READY") mandatory adversary gate distinct from
  the phase-1d package pass.
- No explicit "Complementarity Warning" section in `consistency-validation` /
  `validate-brief` SKILL files (the orchestrator has the principle; the validator skills do not
  self-disclose their scope limits).
- No planted-defect test fixture under `tests/fixtures/adversarial-coverage/` demonstrating
  procedural-clean-but-adversary-caught.

### Prior-closure check

Grep of `.factory/` for `intra-phase adversar` → no decision/lesson/changelog closing this.
The orchestrator's "Fresh-Context Consistency Audit at Every Gate" is the nearest prior art and is
*adjacent but not equivalent* (consistency-validator at gate ≠ adversary after each artifact).

---

## External Research

Primary call: `perplexity_research` (reasoning_effort=high) on runtime workflow enforcement +
why procedural-clean ≠ sound, saved at `.../tool-results/toolu_01QSq1zA7CnicP9ZsZnhT2Mu.txt`.

### Soundness (CONFIRMED)

- **"Procedural validators check what we know to check; adversaries check what we didn't know to
  check" is a sound, well-supported principle.** Research on multi-agent pipelines repeatedly
  distinguishes *schema/structure validation* (cheap, deterministic, catches completeness) from
  *semantic validation* ("deeper semantic analysis — such as using secondary LLMs to evaluate
  test quality against requirements — before accepting step completion"). The recommended pattern
  is exactly the issue's: "multi-stage validation where initial schema checks ensure basic
  completeness, followed by deeper semantic analysis ... before accepting step completion."

- **"Fix introduces new defects" cascade is real.** The research describes *instruction drift* and
  fix-burst novelty: "each fix burst introduced new findings ... ~50% novelty decay rate per
  round." The monocle observation of needing manually-inserted adversary passes between fix bursts
  matches the documented need for re-verification after remediation. This validates the secondary
  "adversary post-fix-burst protocol."

- **State-machine sequencing with evidence-of-completion** is the canonical enforcement pattern
  (LangGraph etc.): "a 'design specification' state cannot transition to 'implementation' until
  the system verifies existence of approved architecture diagrams and interface contracts." This
  supports encoding the checkpoint as a *workflow step with a gate*, not just orchestrator prose.

- **Caveat from research (relevant to scope):** semantic re-review adds latency (cited 100–300ms
  per validation in the abstract; for LLM adversary passes it's minutes each). The research's own
  recommendation is "strategic scoping" — run the heavy semantic pass on *critical/changed* paths,
  not everything, every time. This argues for the issue's *scoped* (new-artifact / fix-burst-only)
  triggers rather than a blanket every-commit adversary pass.

### What I could NOT find

- No external benchmark on optimal frequency of intra-phase adversarial passes for spec artifacts
  specifically — this is a judgment call. The "after architecture artifact" and ">100 lines /
  >patch bump" heuristics in the issue are reasonable but not externally validated; recommend the
  architect ratify thresholds.

---

## Verdict

**VALID-PARTIAL** — Confidence: **HIGH**

The underlying gap is real and the fix direction is sound. But the issue **overstates the
greenfield-ness**: the *principle* ("procedural clean ≠ sound; re-review after change") is
**already half-codified** in the orchestrator's "Fresh-Context Consistency Audit at Every Gate"
and the adversary's "Partial-Fix Regression Discipline (S-7.01)." What is genuinely missing is
**scheduled, workflow-level adversary dispatches at three new trigger points**:

1. After a new architecture artifact lands (intra-Phase-1, before the phase-1d package pass).
2. After a fix-burst (≥3 finding-resolving commits) — promote the adversary's *within-pass*
   propagation axis into a *scheduled re-dispatch*.
3. At pre-phase "FULLY CONVERGED" prep gates (distinct from the phase-1d package pass).

Plus the two documentation deliverables (validator complementarity-warning sections; the planted-
defect fixture). The "every-commit" framing should be narrowed to scoped triggers per the
strategic-scoping caveat.

---

## Recommended Approach & Scope

### Owning agents/skills
- **Workflow file edits** (`.lobster`): `devops-engineer` owns repo/workflow scaffolding; the
  *content* of new adversarial-review steps is mechanical (step shape mirrors existing
  `type: adversarial-review` nodes) → can be authored in-scope.
- **Orchestrator guidance** (`orchestrator.md` + per-story-delivery.md): the human edits root meta
  per CLAUDE.md; orchestrator prose changes route through whoever owns the orchestrator agent file.
- **Validator complementarity warnings**: edit `consistency-validation/SKILL.md` and the
  brief-validation skill; owner is whoever maintains those skills (architect/product-owner-adjacent).
- **Test fixture**: `test-writer` authors the planted-defect fixture; bats/integration owner wires
  it.

### Key files to touch
- `plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster` — add
  `architecture-content-adversarial-checkpoint` (depends_on create-architecture) and a
  PRD-revision checkpoint (`when: iterations > 1`).
- `plugins/vsdd-factory/workflows/code-delivery.lobster` (+ `per-story-delivery.md`) — add
  `post-fix-burst-adversarial-checkpoint` (`when: fix_burst_commits >= 3`).
- `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` — extend the existing
  "Fresh-Context Consistency Audit at Every Gate" with an explicit *adversary* (not just
  consistency-validator) dispatch rule for: new architecture artifact, major addition, post-fix-
  burst, pre-phase gate.
- `plugins/vsdd-factory/skills/consistency-validation/SKILL.md` + brief-validation skill — add a
  "Complementarity Warning" section ("A clean procedural pass does NOT mean the artifact is
  sound").
- `plugins/vsdd-factory/tests/fixtures/adversarial-coverage/` — 700-line artifact with planted
  won't-compile Rust / wrong-field / silent-downgrade defects; bats asserting adversary catches
  them and procedural validators do not.

### Approach
1. Encode the three triggers as scoped `.lobster` steps (mirror existing adversarial-review node
   shape; use `depends_on` + `when:` guards) so enforcement is workflow-level, not prose-only
   (matches the harness-enforcement research; complements #162).
2. Reuse the adversary's existing three-perimeter scope contract — the new intra-phase pass is a
   *spec-review-mode* pass scoped to the new/changed artifact + immediate dependencies, NOT a
   whole-system pass.
3. Have the adversary explicitly NOT re-run procedural checks (consistency-validator owns those) —
   the issue's "Do NOT re-validate procedural items" instruction is correct and avoids waste.

### Risks
- **Cost / loop amplification.** Each intra-phase adversary pass is minutes and can itself spawn a
  fix-burst → new pass. Bound it: scope to the changed artifact, cap iterations (the adversary's
  existing "min 3 / max 10 then escalate" applies), and use the strategic-scoping triggers rather
  than every-commit.
- **3-CLEAN interaction.** Decide whether intra-phase passes participate in the BC-5.39.001
  3-CLEAN streak or are advisory pre-gates. Recommend: advisory pre-gate that must reach NITPICK-
  ONLY before the artifact is declared part of a "converged" package; the package-level phase-1d
  pass still owns formal convergence. Surface to architect/human.
- **Threshold bikeshedding.** ">100 lines" / "> patch bump" heuristics need architect ratification.

### Dependencies / overlaps
- **Overlaps #162 heavily** on the meta-thesis "move load-bearing checks from prose into the
  harness." #133 adds *spec-phase* adversarial scheduling; #162 adds *implementation-phase*
  precondition/asymmetry/merge hooks. Sequence them together so the orchestrator-prose and
  workflow-enforcement edits are coherent (do the orchestrator.md "no prose-only" framing once).
- **Overlaps #177:** the intra-phase adversary pass is a natural host for the hollow-demo lens on
  spec artifacts (claimed-but-undemonstrated clauses). Keep taxonomies distinct but co-located.
- **Builds on** the adversary's S-7.01 partial-fix axis and the orchestrator's at-gate audit —
  this is *promotion of existing within-pass axes into scheduled dispatches*, not net-new analysis.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared with #162) | Procedural vs. semantic validation, fix-burst novelty/instruction-drift, state-machine sequence enforcement, scoping of heavy semantic re-review |
| Read | 6 | orchestrator.md, adversary.md, per-story-delivery.md, phase-1d skill, implementation-readiness, hooks-registry slice |
| Grep / Glob | 6 | intra-phase/adversarial/red-gate/uncertainty sweeps across plugins + .factory + CHANGELOG; workflow file discovery |
| Training data | 1 area | Mapping monocle defect classes to known spec-review failure modes |

**Total MCP tool calls:** 1 deep research (shared) + supporting reads.
**Training data reliance:** LOW — principle corroborated by research; all codebase claims
file-grounded (orchestrator audit + adversary S-7.01 axis are quoted from source).
