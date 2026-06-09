# Issue #169 — Per-story sub-agents read stale worktree `.factory/specs` instead of canonical repo-root specs

**Date:** 2026-06-09
**Issue:** #169 (label: `bug`) — *"Per-story sub-agents read stale worktree .factory/specs instead of canonical repo-root specs (phantom adversarial findings)"*
**Validator:** research-agent
**Branch/commit at validation:** `develop` @ `82163b7f`

---

## Restated Question

Per-story sub-agents (adversary, implementer, test-writer) running inside a story **worktree** read the `.factory/specs` snapshot carried on the `develop`-based feature branch instead of the **canonical specs at the repo-root `.factory/`** (mounted from the `factory-artifacts` orphan branch). The worktree's spec copy is stale relative to the canonical copy, so the fresh-context adversary compares code against the wrong spec version — producing phantom CRITICAL findings (e.g., "ADR-0002 does not exist", "the BC specifies PR-level dedup") that are false, and — more dangerously — can mask real spec drift. A secondary contributor is a **case-sensitive glob** (`Glob('**/ADR*')`) that misses lowercase `adr-0002-*.md` files. The issue proposes three options: (1) codify dispatch guidance to read canonical repo-root absolute paths + case-insensitive matching; (2) sync specs into the worktree at creation; (3) stop tracking `.factory` on `develop` entirely.

---

## Codebase Grounding

### The dispatch context never mentions canonical-vs-worktree spec paths

The per-story delivery dispatch tables that hand spec files to specialists carry **no** guidance distinguishing the canonical repo-root `.factory/specs` from a stale worktree snapshot:

- `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — the "Context Discipline for Dispatches" table (lines 47-54) tells the orchestrator to pass "story file, … relevant BC files" to each specialist but never specifies *which* `.factory/specs` copy (canonical vs worktree). No absolute-path mandate, no staleness warning.
- `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md` — Step-1 dispatch (lines 30-37) passes "Story spec (`.factory/stories/<STORY-ID>-*.md`)" and "Anchored BCs listed in the story's `behavioral_contracts:` frontmatter field" with a relative-looking path, no canonicalization.
- `plugins/vsdd-factory/skills/deliver-story/steps/step-a-create-worktree.md` — creates `.worktrees/STORY-NNN/` from `develop` (lines 13-14). If `.factory/` is tracked on `develop`, the new worktree carries a `.factory/specs` snapshot frozen at that `develop` commit, which is exactly the stale-tree precondition the issue describes.

### The adversary agent's Perimeter-1 scope contract has the same gap and the case-sensitivity gap

`plugins/vsdd-factory/agents/adversary.md`, "Three-Perimeter Scope Contract → Perimeter 1: Per-story" (lines 38-46):
- Scope is "story worktree diff against `develop`, story spec, and BCs listed in the story's `bcs:` frontmatter array." It does **not** instruct the adversary to read specs from the canonical repo-root copy rather than the worktree copy.
- Story-spec lookup uses `Glob('.factory/stories/S-{story-id}-*.md')` (line 42) — a *relative* glob with no canonical-root anchor and no case-insensitivity note. This is the same class of bug as the `Glob('**/ADR*')` miss in the issue.

The "Semantic Anchoring Audit" (lines 164-180) requires anchors like ADR IDs and file paths to "resolve to real workspace artifacts" and treats a missing-file conclusion as convergence-blocking — which is precisely how a stale/lowercase-missed spec turns into a phantom CRITICAL finding.

### Self-referential nuance for vsdd-factory itself

In *this* repo, `.factory/` is a mounted worktree of the `factory-artifacts` orphan branch (per CLAUDE.md "Self-referential" + `.factory/STATE.md:334` "TWO worktrees only"). The downstream-project failure mode in the issue (`prism` S-2.03) is the *general-project* shape where `.factory/specs` is tracked on `develop` and travels with feature worktrees. The fix must be robust to both branch models.

### Verdict-relevant: no prior fix

Grep of `plugins/`, `.factory/cycles/*/decision-log.md`, `STATE.md`, and `CHANGELOG.md` found **no** codification of canonical-spec-path dispatch guidance, no worktree-spec-sync step, and no decision to stop tracking `.factory` on `develop`. The mid-run workaround in the issue (Pass 4+ manual canonical-path injection) is exactly that — manual, per-dispatch, uncodified.

---

## External Research (technical soundness)

Primary-source confirmations (Perplexity deep-research sweep, 2026-06-09):

- **Worktrees carry independent snapshots of tracked files.** A linked worktree shares the object DB but has its own working tree / index / HEAD; tracked files (including a tracked `.factory/`) reflect the branch the worktree is on, not any other branch. — git-worktree docs: https://git-scm.com/docs/git-worktree
- **Canonical current-branch / HEAD identity** for asserting which tree you're reading: `git branch --show-current` (Git ≥ 2.22) and `git rev-parse HEAD`. — https://git-scm.com/docs/git-rev-parse , https://adamj.eu/tech/2023/08/20/git-output-just-current-branch-name/
- **Case-insensitive filesystems (macOS APFS default, Windows NTFS)** make case-sensitive path/glob matching error-prone; Rust/`Path` comparisons are case-sensitive on every platform unless you normalize. This is the root of the `ADR*` vs `adr-0002` miss and means case-insensitive matching is the correct mitigation regardless of OS. — https://doc.rust-lang.org/std/path/index.html , https://swild.dev/dev/apfs-case-insensitive/

These confirm option 1 (canonical absolute paths + case-insensitive matching) is technically sound, and that the staleness is an inherent property of tracking specs on a per-feature branch (motivating options 2/3 as deeper fixes).

---

## Options Comparison

| Option | What it does | Pros | Cons / Risk | Effort |
|--------|--------------|------|-------------|--------|
| **1. Codify dispatch guidance** (canonical repo-root absolute spec paths + treat worktree `.factory/specs` as stale + case-insensitive matching) | Bakes the proven mid-run workaround into `_shared-context.md`, `step-d5`, and `adversary.md` Perimeter-1 | Lowest risk; no branch-model change; immediately effective; matches what already worked Passes 4-10 | Relies on agent compliance (prompt-level, not enforced by hook); doesn't remove the stale snapshot, just routes around it | LOW |
| **2. Sync specs into worktree at creation** | devops copies canonical `.factory/specs` into the worktree at `git worktree add` time | Worktree becomes self-consistent; specs travel with code | Changes spec/code branch model; copies can themselves go stale within a long-lived worktree; adds a sync step + drift detection | MED |
| **3. Stop tracking `.factory` on `develop`** | Specs live only on the canonical/orphan branch; worktrees never carry a snapshot | Cleanest conceptually — removes the staleness *source* | Largest blast radius; for vsdd-factory itself `.factory/` is already an orphan-branch worktree, so this is partly true here but NOT true for general downstream projects that track `.factory` on `develop` | HIGH |

**Recommended:** Option 1 now (production-grade, immediate), with Option 3 evaluated as the durable structural fix in a follow-up architecture decision. Per the Canonical Principle, Option 1 is not an "MVP shortcut" — it fully closes the phantom-finding failure mode at the dispatch layer; Option 3 is a separate, larger branch-model feature that can be deferred to its own cycle by feature-ordering.

---

## Verdict

> **VALID-NEW** — Confidence: **High**
>
> The phantom-finding failure mode is real and reproducible: the dispatch context (`_shared-context.md`, `step-d5-adversary-convergence.md`) and the adversary's Perimeter-1 scope contract (`adversary.md` lines 38-46) contain neither a canonical-spec-path mandate nor case-insensitive matching guidance, and `.factory/`-tracked-on-`develop` worktrees inherently carry a stale snapshot. No prior codification exists in decision-log, STATE, or CHANGELOG. Worth doing.
>
> **Cross-issue note:** #169 and #176 share a root cause (sub-agents reading the wrong git tree in a multi-worktree project). #169 is the *spec-snapshot staleness* facet; #176 is the *feature-code/evidence wrong-checkout* facet. They should be fixed together with a single worktree-identity-preflight + canonical-path discipline that covers BOTH spec reads (canonical repo-root) and feature-code reads (worktree-rooted).

---

## Recommended Approach (zero re-research)

**Route to:** `product-owner` is NOT the owner here — this is agent-prompt + skill-step process content. Per CLAUDE.md routing, agent/skill prompt edits are engine-discipline content; route the codification through the orchestrator → `state-manager` fix burst (for `.factory/` decision-log/lessons) and direct skill/agent file edits land via the normal develop-branch PR path (these are `plugins/` source files, not `.factory/` artifacts).

**Key files to touch:**
1. `plugins/vsdd-factory/agents/adversary.md` — Perimeter-1 scope contract (lines 38-46): add a "Spec-read discipline" sub-block: (a) read all spec artifacts (`.factory/specs/**`, ADRs, BC-INDEX) from the **canonical repo-root** absolute path, NOT the worktree copy; (b) the worktree `.factory/specs` is a potentially-stale branch snapshot — never glob or read it for spec ground-truth; (c) use case-insensitive matching for ID-bearing globs (`adr`/`ADR`, `bc`/`BC`). Fix the example `Glob('.factory/stories/S-{story-id}-*.md')` to note case-insensitivity and canonical-root anchoring.
2. `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — "Context Discipline for Dispatches" table (lines 47-54): add a note that all spec/BC/ADR files passed to specialists MUST be canonical repo-root absolute paths; worktree `.factory/specs` is stale and off-limits for spec ground-truth.
3. `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md` — Step-1 dispatch (lines 30-37): make the spec/BC paths explicitly canonical-absolute and add the case-insensitive-matching instruction.

**Approach:**
- Codify Option 1 across the three files. Add a guard rule mirroring #176: an "absent file / missing deliverable / missing ADR" finding MUST be corroborated by a canonical-path + case-insensitive check before it is reported as a finding (this single guard closes both the staleness and the case-sensitivity vectors).
- Record the codification as a decision (D-NNN) + lesson (L-EDP1-NNN) via state-manager so it is enforced going forward.

**Risks:**
- Prompt-level guidance is not hook-enforced; combine with #176's mechanical worktree-identity preflight (`git rev-parse HEAD` assertion) for defense-in-depth. The two issues' fixes are complementary.
- Option 1 leaves the stale snapshot in place; if a future cycle adopts Option 3 (stop tracking `.factory` on `develop`), revisit and simplify the guidance.

**Test strategy:**
- Add a bats/integration check (under `plugins/vsdd-factory/tests/`) that asserts the adversary dispatch prompt template contains the canonical-spec-path + case-insensitive-matching clauses (prompt-contract test, analogous to existing template-reference assertions in CHANGELOG line 5667).
- Regression scenario: a worktree whose tracked `.factory/specs` is intentionally older than canonical; assert the adversary reads canonical (no phantom "missing file").

**Dependencies:**
- **#176** (worktree-identity preflight) — same root cause, fix together. #176 provides the mechanical `git rev-parse HEAD` assertion; #169 provides the canonical-spec-path + case-insensitive discipline. One combined work item.
- Option 3 (branch-model change) would be a separate architect-owned decision.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared across #128/#130/#169/#176) | git worktree identity detection, case-insensitive path matching gotchas, primary-source git-scm docs |
| Read | 6 | adversary.md, deliver-story _shared-context.md, step-d5, step-a, code-delivery SKILL, existing research template |
| Grep | 4 | dispatch/worktree/stale-spec patterns across plugins + .factory cycles |
| Glob | 2 | agent + skill file enumeration |
| Training data | 0 areas | All worktree/path claims externally sourced |

**Total MCP tool calls:** 1 (deep research, shared)
**Training data reliance:** Low — git worktree + case-insensitivity claims verified against git-scm.com and rust-lang.org; codebase claims verified by direct file reads with line cites.
