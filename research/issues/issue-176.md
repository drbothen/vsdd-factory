# Issue #176 — adversarial-review: add worktree-identity preflight to prevent wrong-tree false-positive findings

**Date:** 2026-06-09
**Issue:** #176 (label: `enhancement`) — *"adversarial-review: add worktree-identity preflight to prevent wrong-tree false-positive findings"*
**Validator:** research-agent
**Branch/commit at validation:** `develop` @ `82163b7f`

---

## Restated Question

The `adversary` agent and the `adversarial-review` dispatch skill can silently review the **wrong git tree** in a multi-worktree project. When the agent's working directory resolves to a different worktree than the one under review (inheriting the orchestrator's shell cwd, or resolving a bare project path to the main checkout on the base branch), it reads files that lack the feature branch's changes and reports **false-positive "absent file / missing deliverable" findings** (observed twice on `prism` Wave-5: a PR-level review reported `search.rs` and the entire `docs/demo-evidence/<STORY>/` directory as absent + 2 CRITICAL findings, when both existed in the feature worktree at the dispatched HEAD). This class can also **false-GREEN** in the opposite direction (passing a PR whose feature-branch code was never actually reviewed). The proposed fix is a mandatory **worktree-identity preflight**: (1) assert `git -C <worktree-abs-path> rev-parse HEAD` equals the dispatched feature HEAD SHA, else STOP; (2) assert the worktree basename equals the story id; (3) mandate absolute worktree-rooted paths for all feature-code/evidence reads, forbid bare relative paths / main-checkout reads; (4) require an "absent file" finding to be corroborated by a path-correct check first. Codified as "Lesson 62" downstream and filed upstream.

---

## Codebase Grounding

### No worktree-identity preflight exists in the adversary agent or the skill

`plugins/vsdd-factory/agents/adversary.md`:
- "Three-Perimeter Scope Contract → Perimeter 1: Per-story" (lines 38-46) defines scope as "story worktree diff against `develop`" and uses `Glob('.factory/stories/S-{story-id}-*.md')` (line 42), but there is **no** instruction to verify the worktree's HEAD SHA equals the dispatched target, no basename==story-id check, and no mandate for absolute worktree-rooted feature-code paths.
- The "Implementation Review (Phase 5)" mode (lines 87-96) and "Confidence Levels" table (lines 182-190) classify an absent-file finding as HIGH/CRITICAL with "Specific file path + line" evidence — but "the file is absent" from the *wrong tree* still satisfies that evidence bar, so the self-validation loop (lines 144-152: Evidence/Actionability/Duplication checks) does NOT catch a pathing artifact. The issue's failure mode passes every existing guard.

`plugins/vsdd-factory/skills/adversarial-review/SKILL.md`:
- The "For Implementation Review" section (lines 127-129) says "Read specs first, then review source code against them" with scope flags, but no preflight asserting the dispatched HEAD matches the tree being read.
- The "Red Flags" table (lines 32-42) and "Post-Adversary Persistence" (lines 131-145) have no wrong-tree guard.

`plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`:
- Step-1 dispatch (lines 30-37) passes "Story worktree diff (`.worktrees/<STORY-ID>/`)" but provides no HEAD-SHA assertion and no absolute-path mandate for the diff/evidence reads.

### The exact false-positive class the issue names is already a known cost in this engine's lessons

`plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` "Verification Discipline" (lines 60-69) preaches "Agent says X is a CLAIM, not EVIDENCE" and `step-f-pr-lifecycle.md:16` already uses `git ls-remote origin feature/STORY-NNN-<desc>` as an exit-condition verifier — i.e., the engine already trusts git-level assertions over agent claims elsewhere, but has NOT applied that discipline to the adversary's tree identity. The proposed preflight is consistent with established in-repo patterns.

### No prior fix

Grep across `plugins/`, `.factory/cycles/*/decision-log.md`, `STATE.md`, and `CHANGELOG.md` found no "worktree-identity", "rev-parse HEAD" preflight, or "wrong-tree" guard in the adversary path. Not addressed.

---

## External Research (technical soundness)

Primary-source confirmations (Perplexity deep-research, 2026-06-09):

- **`git rev-parse HEAD`** consistently returns the full commit SHA for the current worktree regardless of attached/detached HEAD — the correct, stable primitive for the preflight's SHA assertion. — https://git-scm.com/docs/git-rev-parse
- **`git -C <path>`** runs the command as if started in `<path>`, so `git -C <worktree-abs-path> rev-parse HEAD` reliably reads *that* worktree's HEAD even when the process cwd is elsewhere — directly enabling proposal step (1). — https://git-scm.com/docs/git
- **Each linked worktree maintains its own HEAD** while sharing the object DB; `git branch --show-current` / `git rev-parse HEAD` are worktree-local, so per-worktree identity assertion is well-defined. — https://git-scm.com/docs/git-worktree , https://git-scm.com/docs/git-branch
- **`git rev-parse --show-toplevel`** yields the current worktree's root, useful for asserting the basename==story-id check (proposal step 2). — https://git-scm.com/docs/git-rev-parse

The proposed preflight is technically sound and uses the canonical git primitives. It is the "trust but verify" pattern the issue correctly notes is applied everywhere else in `deliver-story` but not by the adversary to its own input tree.

---

## Verdict

> **VALID-NEW** — Confidence: **High**
>
> The wrong-tree false-positive (and the dangerous false-GREEN inverse) is real: neither `adversary.md` (Perimeter-1 scope, lines 38-46) nor `adversarial-review/SKILL.md` (implementation-review section, lines 127-129) nor `step-d5-adversary-convergence.md` (dispatch, lines 30-37) contains a HEAD-SHA preflight, a basename check, an absolute-path mandate, or an "absent-file must be path-corroborated" guard. The proposed git primitives (`git -C <wt> rev-parse HEAD`) are canonical and confirmed by git-scm docs. No prior codification. Worth doing.
>
> **Cross-issue note:** #176 and #169 are the same root cause — sub-agents reading the wrong git tree in a multi-worktree project. #176 covers *feature-code/evidence wrong-checkout* (read the worktree, not the main checkout); #169 covers *spec-snapshot staleness* (read the canonical repo-root specs, not the stale worktree snapshot). The combined fix is: a mechanical worktree-identity preflight (this issue) PLUS canonical-spec-path + case-insensitive discipline (#169). Land them together.

---

## Recommended Approach (zero re-research)

**Route to:** orchestrator → engine-discipline codification. Agent/skill prompt edits are `plugins/` source (develop-branch PR path); the decision/lesson record lands via `state-manager` fix burst in `.factory/`.

**Key files to touch:**
1. `plugins/vsdd-factory/agents/adversary.md` — add a mandatory **"Worktree-Identity Preflight"** section (before any finding is produced) to the Perimeter-1 scope contract:
   - (a) `git -C <worktree-abs-path> rev-parse HEAD` MUST equal the dispatched feature HEAD SHA; mismatch → STOP, emit a dispatch-error (NOT content findings).
   - (b) Assert `basename(git -C <wt> rev-parse --show-toplevel)` matches the dispatched story id / target.
   - (c) Mandate absolute worktree-rooted paths for all feature-code/evidence reads; forbid bare relative paths and reads of the main checkout for feature code.
   - (d) Guard rule: an "absent file / missing deliverable" finding MUST be path-corroborated (correct worktree-rooted absolute path + case-insensitive match) before being reported; pathing-artifact absences are NOT findings. (This guard also closes #169's case-sensitivity vector.)
2. `plugins/vsdd-factory/skills/adversarial-review/SKILL.md` — add a "Worktree-Identity Preflight (MANDATORY)" subsection to the dispatch contract requiring the orchestrator to PASS the expected `(worktree-abs-path, feature-HEAD-SHA, story-id)` triple into the adversary task, and the adversary to ASSERT it.
3. `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md` — Step-1 dispatch (lines 30-37): include the expected HEAD SHA + absolute worktree path in the dispatch, and require the preflight assertion to pass before findings.

**Approach:**
- Adversary is `read-only` (Read/Grep/Glob; no Bash — adversary.md lines 320-328). So the SHA assertion cannot be executed *by the adversary itself*. Two viable designs:
  - **Design A (orchestrator-asserted, recommended):** orchestrator (which has shell) runs `git -C <wt> rev-parse HEAD` before dispatch, embeds the verified SHA + absolute worktree path in the task prompt; the adversary treats the embedded absolute paths as the only legal read roots and refuses bare-relative/main-checkout reads. The orchestrator's pre-dispatch assertion IS the preflight; the adversary enforces the path discipline.
  - **Design B (hook-enforced):** add/extend a WASM preflight hook (cf. `validate-per-story-adversary-convergence` referenced in step-d5 line 12) that blocks the adversary dispatch unless the dispatch payload carries a matching `(worktree, head_sha)` tuple. Stronger but larger.
- Recommend Design A in the prompt/skill now (production-grade, immediate), with Design B as an optional hardening follow-up.

**Risks:**
- If only the prompt is changed (no hook), enforcement depends on the orchestrator embedding the triple correctly. The "absent-file must be corroborated" guard is the backstop that catches the residual case.
- Detached-HEAD worktrees: `git rev-parse HEAD` still works (returns the SHA), so the assertion holds; basename check should compare the dispatched target, not the branch name.

**Test strategy:**
- Prompt-contract bats test: assert `adversary.md` contains the preflight clauses (HEAD-SHA assertion + absolute-path mandate + absent-file-corroboration guard).
- Integration regression: dispatch adversary with a cwd pointing at the main checkout while target is a feature worktree; assert it STOPs with a dispatch-error rather than emitting "file absent" findings.

**Dependencies:**
- **#169** — same root cause; combine. #176 = mechanical tree-identity preflight; #169 = canonical-spec-path + case-insensitive discipline. The shared guard rule (d) serves both.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared across #128/#130/#169/#176) | git worktree identity detection, `git -C`/`rev-parse HEAD`/`--show-toplevel` semantics, primary-source git-scm docs |
| Read | 4 | adversary.md, adversarial-review SKILL, step-d5, _shared-context.md |
| Grep | 3 | worktree-identity / rev-parse / wrong-tree patterns across plugins + .factory |
| Glob | 1 | agent enumeration |
| Training data | 0 areas | All git claims externally sourced |

**Total MCP tool calls:** 1 (deep research, shared)
**Training data reliance:** Low — git primitives verified against git-scm.com; codebase claims verified by direct file reads with line cites.
