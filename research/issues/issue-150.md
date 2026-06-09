# Issue #150 — Per-Story Uncertainty Removal + Self-Containment Review (Pre-Phase-3 Gate)

**Date:** 2026-06-09
**Issue:** #150 — "Feature: Per-Story Uncertainty Removal + Self-Containment Review (Pre-Phase-3 Quality Gate)"
**State:** OPEN, label: enhancement
**Reviewer:** research-agent (cluster: review/adversarial-quality)
**Repo:** vsdd-factory @ develop `82163b7f`

---

## Restated Question

Add a new skill `/vsdd-factory:story-uncertainty-review` that runs a structured per-story
completeness audit **between Phase 2 (Story Decomposition) and Phase 3 (TDD Implementation)**.
It adapts the `dclaude:remove-uncertainty` pattern (tech-uncertainty research) and extends it to
a **5-dimension** framework (Tech Uncertainty / Context Gap / AC Executability / Implementation
Detail / Cross-Story Dependency), run as a 4-stage SCAN→RESEARCH→FIX→VERIFY lifecycle using the
`spec-reviewer` agent (cognitive diversity). Thesis: a story can be adversarially *converged*
(internally consistent) yet still be **unexecutable by a fresh-context LLM** with no prior project
knowledge. Calibration on monocle hit 3/3 stories needing revision with ≥1 CRIT each. Gate:
CRIT-finding count must be 0 to clear Phase 3.

---

## Codebase Grounding

### What already exists (substantial overlap — this is the most-covered issue in the cluster)

1. **`rules/story-completeness.md` — a 14-point "Story Self-Containment Audit."** This is
   essentially the issue's framework already in prose form. Its stated goal is verbatim the
   issue's thesis: *"an implementer can execute the story without leaving the file."* Its 14 checks
   map onto the issue's 5 dimensions:
   - Check 1 "Source of truth alignment" + Check 12 "Internal consistency" → **Dim D
     (Implementation Detail / spec drift)** — "Do embedded configs, dependency rules, and crate
     lists match the architecture docs they reference? Compare line by line — stale data is the #1
     gap" (mirrors the issue's `SessionStatus` 3-vs-5-variant drift finding).
   - Check 2 "All deliverable files specified", Check 9 "Test fixtures defined" → **Dim D**.
   - Check 3 "Technical gotchas", Check 14 "Prerequisites" → **Dim A/B (tech + context)**.
   - Check 7 "License stated", Check 6 "Hosting/infra explicit" → self-containment.
   The audit even prescribes a **process** (read end-to-end → run checks → research-or-fix →
   fix-one-at-a-time → final consistency pass). **What it lacks:** an *agentic, fan-out,
   spec-reviewer-driven, gated* lifecycle; explicit AC-executability/oracle and cross-story-handoff
   dimensions; and a Phase-2.5 workflow position.

2. **`skills/implementation-readiness/SKILL.md`** — the existing "gate between planning and
   building." It validates the *whole spec package* (PRD/arch/stories/BC/VP) across 8 dimensions
   including "Story Coverage" (every story has numbered behavioral-assertion ACs; token-budget
   20–30%; 300–800-token density), "Context Budget Validation" (lost-in-the-middle audit;
   reference-vs-inline), and "Cross-Document Consistency (traceability matrix)." **This is a
   package-level readiness gate, not a per-story cold-LLM-executability fan-out.** It overlaps the
   issue's intent but at a coarser grain.

3. **`rules/step-decomposition.md`** (matched on "uncertainty" grep) — step-level decomposition
   guidance; adjacent.

4. **`agents/spec-reviewer.md`** — the cognitive-diversity constructive reviewer the issue
   nominates as the primary agent. It exists and is the correct owner per CLAUDE.md routing
   ("Constructive spec/story review (different-model cognitive diversity)").

5. **Phase 2 → Phase 3 transition + `decompose-stories` skill** — the workflow position the issue
   targets. Currently the orchestrator's MANDATORY STEPS jump from Phase 2 adversarial story
   convergence to Pre-Phase-3 DTU/CI checks; there is **no `phase-2.5-story-uncertainty-review`
   step**.

### `remove-uncertainty` history (D-501/D-523 check requested in the task)

- **No `dclaude:remove-uncertainty` skill exists in this repo** (Grep of `plugins/` for
  `remove-uncertainty` → 0 hits; the term appears only in `architect.md`, `CONVERGENCE.md`,
  `onboard-observability`, and the two `rules/` files, none of which is the dclaude skill). The
  issue correctly frames dclaude's variant as the *external seed*, not an existing vsdd-factory
  asset.
- **No D-501 or D-523 found** in `.factory/cycles/*/decision-log.md` or STATE.md (Grep returned no
  decision rows with those IDs tied to "remove-uncertainty"). The latest decision in STATE.md is
  D-449. **Conclusion: there is no prior "remove-uncertainty" closure to dedupe against in this
  repo** — the requested D-501/D-523 history does not exist here (it likely belongs to a different
  downstream project, e.g., monocle/dclaude). Flagging as NEEDS-HUMAN-confirmation only on the
  provenance point; it does not change the verdict.

### What does NOT exist (the genuine residual)

- No `skills/story-uncertainty-review/SKILL.md` with the 5-dimension framework + 4-stage
  SCAN/RESEARCH/FIX/VERIFY lifecycle.
- No **AC-executability/oracle dimension** as a first-class check ("what HTTP status counts as
  'proceeds'?") — neither story-completeness.md nor implementation-readiness has an explicit
  oracle-definition check per AC.
- No **Cross-Story Dependency handoff** dimension (`depends_on` must *summarize what predecessors
  produce*: paths, symbols, types) — story-completeness.md has no cross-story handoff check.
- No `phase-2.5-story-uncertainty-review` STATE.md phase tag / orchestrator MANDATORY-STEPS entry.
- No per-story report path registry (`.factory/plans/story-uncertainty-review/<cycle>/`).

### Prior-closure check

No decision/lesson/changelog closes this. The capability is **partially pre-implemented as a
manual rule** (`story-completeness.md`) and a **coarser package gate** (`implementation-readiness`),
but not as the proposed agentic, gated, per-story skill.

---

## External Research

Primary call: `perplexity_research` (reasoning_effort=medium) on pre-implementation readiness
gates, DoR/INVEST, self-contained specs for LLM agents, lost-in-the-middle, cognitive diversity,
shift-left cost asymmetry. Saved at
`.../tool-results/mcp-perplexity-perplexity_research-1781028647965.txt`.

### Soundness (CONFIRMED — strong, multi-strand prior art)

- **"Definition of Ready" (DoR) + INVEST** are the established Agile codification of *exactly*
  this gate: a story is "Ready" for a sprint only when it is independently executable, with clear
  acceptance criteria and no unresolved dependencies. INVEST's **I (Independent)** and
  **T (Testable)** criteria map directly onto the issue's Dim E (cross-story dependency) and Dim C
  (AC executability/oracle). The proposed skill is, in effect, an **automated, LLM-specific DoR
  gate** — a well-precedented concept, not a novel one. (Research repeatedly returns DoR/INVEST as
  the canonical frame; treat the specific sources as the standard Agile literature.)

- **"Internally consistent ≠ fresh-context executable" is sound and specifically LLM-relevant.**
  Research on **context-window limits and "lost in the middle"** (degraded retrieval of mid-context
  information) confirms that a cold-context agent grepping a vague reference ("see BC-X §Y" with no
  anchor) incurs real reasoning-quality cost — the issue's Dim B (Context Gap). The recommended
  mitigation is frontloading critical constraints and making references resolvable without scanning
  whole files (mirrors implementation-readiness's existing "Information placement audit" and the
  issue's anchor requirement).

- **Cognitive diversity / different-model review** is a validated defect-detection lever:
  reviewing with a model from a *different family* than the author catches author-blind-spot
  defects. The issue's use of `spec-reviewer` (different model from the story-writer that produced
  the corpus) is the correct application — and is already the documented purpose of the
  spec-reviewer agent.

- **Shift-left cost asymmetry is well-supported.** Catching a spec defect before implementation is
  cheaper than after by a large multiple (research cites 5–7× remediation cost for defects that
  escape into hollow-test-covered code; the issue's "~5 min wall time vs 2–4 hrs Phase 3/5 cycles
  per defect" estimate is directionally consistent). This is the core ROI argument and it holds.

### What I could NOT find

- No standardized "cold-LLM executability score" metric (the issue's Open Design Question #3).
  Research offers proxies (token-count, findings-per-story, DoR-checklist pass rate) but no
  authoritative single metric. Recommend a checklist-pass + zero-CRIT gate (matching DoR practice)
  rather than inventing a numeric score.
- No external guidance resolving the auto-routing-vs-report-only question (Open Q #2) or the
  sampling-vs-full question for 50+ story corpora (Open Q #5) — these are local engineering
  decisions. Given vsdd-factory's worktree-race history (SE-18), **report-only with orchestrator-
  driven serialized routing is the lower-risk default**; full-run (not sampling) matches the
  production-grade default unless the human directs sampling for cost.

---

## Verdict

**VALID-PARTIAL** — Confidence: **HIGH**

The capability is real, well-precedented (it is an automated LLM-specific Definition-of-Ready
gate), and **not closed**. But it **substantially overlaps existing assets** and must be scoped to
*extend, not duplicate* them:

- `rules/story-completeness.md` already encodes ~3 of the 5 dimensions (Tech, Context partial,
  Impl Detail/spec-drift) as a 14-point manual audit. The new skill should **operationalize this
  rule into an agentic fan-out**, not author a parallel checklist.
- `implementation-readiness` already provides a package-level readiness gate with token-budget and
  lost-in-the-middle checks. The new skill is the **per-story, cold-LLM-executability** complement
  at finer grain — position it as Phase-2.5 *feeding* the existing readiness gate, not replacing it.
- The genuinely *new* content is: **Dim C (AC executability / oracle definition)** and **Dim E
  (cross-story handoff summary)** as first-class checks, plus the **4-stage gated lifecycle** and
  the Phase-2.5 workflow position.

CRIT=0 gate is reasonable and matches DoR practice. Provenance of D-501/D-523/remove-uncertainty
is **NEEDS-HUMAN** confirmation (those IDs are not in this repo).

---

## Recommended Approach & Scope

### Owning agents/skills
- **New skill:** `skills/story-uncertainty-review/SKILL.md`. Authoring routes through the
  skill/spec owner; embeds the spec-reviewer agent prompt template.
- **Primary agent:** `vsdd-factory:spec-reviewer` (different model family from story-writer) for
  SCAN + VERIFY stages. RESEARCH stage → `research-agent` (MCP). FIX stage → `story-writer`
  (cross-routing to `architect` for pin-manifest/spec-drift, `product-owner` for BC/PRD edits,
  `consistency-validator` post-fix), per the issue's Stage-3 routing — which matches CLAUDE.md.
- **Workflow/STATE wiring:** `state-manager` records the `phase-2.5-story-uncertainty-review` tag;
  orchestrator owns the MANDATORY-STEPS insertion.

### Key files to touch
- `plugins/vsdd-factory/skills/story-uncertainty-review/SKILL.md` (new) — 5-dim framework +
  4-stage lifecycle; **cite `rules/story-completeness.md` as the dimension source-of-truth for the
  3 overlapping dimensions** and add only Dim C (AC/oracle) + Dim E (cross-story handoff) as new.
- `plugins/vsdd-factory/rules/story-completeness.md` — extend with the AC-oracle and cross-story-
  handoff checks (so rule and skill stay single-sourced).
- `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` — add to MANDATORY STEPS:
  "Pre-Phase 3: story-uncertainty-review ... CRIT-finding count must be 0 to clear Phase 3."
- Workflow files: `workflows/greenfield.lobster`, `workflows/brownfield.lobster`,
  `workflows/feature.lobster` — insert the Phase-2.5 step between Phase-2 GATE PASS and Phase-3
  dispatch.
- Artifact path registry: `.factory/plans/story-uncertainty-review/<cycle>/` (+ master-inventory).
- `.factory/policies.yaml` — optional POL making the CRIT=0 gate (and configurable HIGH/MED/LOW
  thresholds, Open Q #1) declarative.

### Approach
1. Single-source the dimensions: extend `story-completeness.md` (the existing rule) to all 5
   dimensions; have the skill *invoke* spec-reviewer against that rule. Avoids the
   two-checklists-drift problem.
2. 4-stage lifecycle as written (SCAN parallel-per-story → RESEARCH parallel-per-Q → FIX serialized
   → VERIFY parallel), with **report-only routing by default** (orchestrator dispatches fixes;
   avoids SE-18 worktree/version-bump races — Open Q #2).
3. Full-corpus by default; sampling only on explicit human direction (production-grade default;
   Open Q #5).
4. Gate = checklist-pass + CRIT=0 (DoR-style), not an invented numeric score (Open Q #3).

### Risks
- **Duplication / drift with story-completeness.md and implementation-readiness.** The #1 risk.
  Mitigate by single-sourcing dimensions in the rule and positioning the skill as per-story
  operationalization feeding the existing package gate.
- **Cost on large corpora** (50+ stories) — spec-reviewer dispatch per story. Default full; allow
  human-directed sampling.
- **Auto-routing version-bump races** (SE-18) — default to report-only + serialized fixes.
- **Idempotency (AC-7)** — spec-reviewer outputs vary run-to-run; "deterministic" should mean
  "same findings class," not byte-identical. Clarify the AC with product-owner.

### Dependencies / overlaps
- **Strong overlap with `rules/story-completeness.md`** (extend, don't fork) and
  **`implementation-readiness`** (complement at finer grain). This is the key cross-issue note.
- **Overlaps #133/#162 thematically** (adversarial vs procedural; harness vs prose) but operates
  at the *story-spec, pre-implementation* layer — distinct perimeter from #133 (intra-phase spec
  adversarial) and #162 (implementation-time runtime gates). Position story-uncertainty-review as
  the cold-LLM-executability gate; #133 as the semantic-soundness adversary; complementary, not
  redundant.
- **Provenance NEEDS-HUMAN:** confirm the dclaude `remove-uncertainty` / D-501 / D-523 lineage
  (not present in this repo) so the upstream skill is credited correctly and no duplicate decision
  IDs are minted.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | DoR/INVEST, self-contained specs for LLM agents, lost-in-the-middle/context-window, cognitive-diversity review, shift-left cost asymmetry |
| Read | 4 | story-completeness.md, implementation-readiness SKILL, spec-reviewer routing (via CLAUDE.md), per-story-delivery |
| Grep / Glob | 6 | remove-uncertainty/uncertainty/D-501/D-523/self-containment sweeps across plugins + .factory; skill + agent discovery |
| Training data | 1 area | INVEST/DoR mapping to the 5-dimension framework (cross-checked vs Perplexity synthesis) |

**Total MCP tool calls:** 1 deep research (#150-specific) + supporting reads.
**Training data reliance:** LOW — DoR/INVEST/lost-in-the-middle corroborated by research; all
codebase + D-NNN claims file/grep-grounded. The one open provenance point (D-501/D-523) is
explicitly flagged NEEDS-HUMAN rather than guessed.
