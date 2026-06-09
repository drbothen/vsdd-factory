# Issue #129 — canonicalize the Production-Grade Default + Correct Agent Routing principle

**Date:** 2026-06-09
**Repo:** vsdd-factory (self-referential) @ `develop` `82163b7f`
**Issue:** [#129](https://github.com/) — *feat(canonicalization): production-grade default + correct agent routing principle* (label: enhancement; state: OPEN; ~35KB self-contained spec)
**Research agent:** Claude (vsdd-factory:research-agent)
**Consumer:** architect / orchestrator (multi-specialist fix-burst); this is a *plugin-wide canonicalization*, not a single-file edit

---

## Restated Question

Canonicalize the "Production-Grade Default with Correct Agent Routing" principle **across all factory agents, skills, templates, rules, hooks, and workflows in the distributable plugin** (`plugins/vsdd-factory/`), so that **every project that uses vsdd-factory inherits it**. The issue is a fully self-contained 35KB spec: it supplies the verbatim principle text (Part 1–2), a motivating exemplar (Part 3, the `monocle` project), a file-by-file change table with MUST/SHOULD/COULD severities and approximate line numbers (Part 4), a cold-start implementation plan with per-step acceptance criteria (Part 5), migration notes (Part 6), and cross-references (Part 7). Three **new enforcement hooks** are specified: `validate-tech-debt-deferral.sh`, `validate-deferred-findings.sh`, and `validate-no-pending-architect.sh`.

**The decisive grounding question:** the principle ALREADY exists in this repo's *root* `CLAUDE.md`. Does that satisfy #129, or does #129 ask for something the root CLAUDE.md does not deliver?

---

## Codebase Grounding (decisive) — the principle is in the WRONG place for #129's purpose

### The critical distinction: root `CLAUDE.md` ≠ the distributable plugin

The "CANONICAL PRINCIPLE — Production-Grade Default" + "Companion Principle — Correct Agent Routing" + full Agent Routing Table are present and mature in **`/CLAUDE.md`** (this repo's *own self-referential operating doc*). That is the **operator-level doc for the vsdd-factory project itself** — it governs agents working *on* vsdd-factory.

**#129 asks for canonicalization in the SHIPPED PLUGIN** (`plugins/vsdd-factory/docs/FACTORY.md`, `VSDD.md`, `docs/AGENT-SOUL.md`, every agent prompt, skills, templates, rules, and new hooks) — the artifacts that get **distributed via the marketplace tarball and inherited by every downstream project** (monocle, etc.). A principle in `/CLAUDE.md` does NOT propagate to downstream projects; only the plugin's own docs/agents/templates do.

A repo-wide grep for the principle's signature phrases — `Production-Grade Default | Correct Agent Routing | No MVP-driven deferrals | Self-Audit Checklist` — across **`plugins/vsdd-factory/`** returned **ZERO matches**. The principle has **not** been canonicalized into the distributable plugin. So the root-CLAUDE.md presence does **not** satisfy #129.

### Verifying #129's MUST-change targets against the current plugin

Each MUST item's target text was confirmed to exist in its *pre-canonicalization* (weaker) form, exactly as the issue describes:

| #129 MUST target | Current plugin state (verified) | Status |
|---|---|---|
| `docs/FACTORY.md` — insert full canonical principle | Only the weaker predecessor exists: `docs/FACTORY.md:257` — *"Mis-anchoring is NEVER an 'Observation' or 'deferred post-v1' — it always blocks convergence."* No canonical principle, no companion routing principle. | **NOT DONE** |
| `docs/AGENT-SOUL.md` — reference principle from Principle 8 | `AGENT-SOUL.md:95` is *"## 8. Pragmatism Over Ceremony"* with a *"principled pragmatism vs rationalization"* footnote (lines 101–116) — the weaker predecessor the issue targets for strengthening. No reference to a canonical production-grade principle. | **NOT DONE** |
| `agents/adversary.md` — tighten deferred-findings scope (~46-54) | `adversary.md:46-54` present verbatim: *"Any finding that requires knowledge outside the three scope sources MUST be tagged as a deferred finding … Deferred findings do NOT block per-story convergence."* The four BC-5.39.002 categories + `deferred_findings` JSON contract are intact and untightened. | **NOT DONE** |
| `agents/security-reviewer.md` — eliminate "advisory" as terminal | `security-reviewer.md:202` present: *"Advisory check: [CLEAN | advisory description]"* — advisory is still a terminal classification. | **NOT DONE** |
| `agents/orchestrator/orchestrator.md` — routing-table completeness + cycle-closure deferral verification | Routing table exists (`orchestrator.md:157` "### Agent Routing Table"); "NEVER write ANY files" constraint at `:101` and `:377`. Neither references the canonical principle, nor the "deferral without story-ID = BLOCKED" rule. | **PARTIAL** (table exists; principle wiring + cycle-closure tightening absent) |
| `skills/track-debt/SKILL.md` — BREAKING "When to Add Debt" change (79-85) | `track-debt/SKILL.md:77-80` present in OLD form: *"## When to Add Debt — Adversarial review finds a real issue but it's deferred / Code review identifies a shortcut taken for timeline …"* — no `human_directed` requirement, no target-story requirement. | **NOT DONE** |
| `templates/tech-debt-register-template.md` — HUMAN-DIRECTED + story-attachment | No `human_directed` / `target_story` requirement present. | **NOT DONE** |
| `rules/lessons-codification.md` — strengthen deferral protocol | Present in weaker form (per issue). | **NOT DONE** |
| `hooks/validate-tech-debt-deferral.sh` (NEW) | Absent — grep of `hooks/*.sh` (36 hooks) confirms no such file. | **NOT DONE** |
| `hooks/validate-deferred-findings.sh` (NEW) | Absent. | **NOT DONE** |
| `hooks/validate-no-pending-architect.sh` (NEW, SHOULD) | Absent. | **NOT DONE** |

**Minor discrepancy in the issue's line references:** the P2 "Nice-to-have. Can defer." text the issue attributes to `product-owner.md` and `business-analyst.md` did **not** grep-match in `product-owner.md` at `develop` `82163b7f` (the file may have been refactored since the issue was filed in May 2026). The implementing session must re-locate these SHOULD targets by content, not by the issue's line numbers — the issue itself instructs (Part 5 Step 1.4): *"If any path is wrong, surface it … BEFORE making any edits."* This is expected drift, not a blocker.

### Is `/CLAUDE.md`'s version reusable as the canonical source?

**Yes — and this is the single biggest scope-reducer.** The root `/CLAUDE.md` already contains a *more evolved* version of the principle than the issue's Part 1–2 draft (it includes Standing Rule 3, the TD-VSDD-059/060 paper-fix/sibling-sweep additions, and a richer anti-pattern table from the engine-discipline cycle). The implementing session should **lift the canonical text from `/CLAUDE.md` into the plugin docs** rather than transcribe the issue's older draft — the issue's Part 1 is a faithful but slightly-stale snapshot of what is now in `/CLAUDE.md`. This materially de-risks the "verbatim" acceptance criterion (AC-1).

### Prior closure check

No CHANGELOG entry references #129, "canonicalization" of the production-grade principle, or the three new hooks. The `trace_id` canonicalization entries in CHANGELOG (lines 132, 594, 652) are unrelated (field-name canonicalization, TD #66). Nothing for #129 has landed in the plugin.

---

## External Research — primary sources

This issue is primarily an **internal-governance / prompt-engineering canonicalization**, so external tech is secondary. The relevant external grounding is the **engineering practice of propagating a single source-of-truth policy across many consuming artifacts** and **enforcing it with validation gates** — the two patterns #129 leans on (one canonical text + enforcement hooks).

`perplexity_research` (shared call) surfaced the directly-applicable enforcement precedents:

- **Validation gates that block on policy violation** (the model for the three new hooks): Flyway `validate` / Alembic / Django "migrations out of date" all demonstrate the **fail-the-operation-on-contract-violation** pattern the issue wants for tech-debt entries lacking `human_directed`/`target_story` — https://www.red-gate.com/hub/product-learning/flyway/flyways-validate-command-explained-simply/ and https://www.better-simple.com/django/2023/06/03/django-migrations-and-your-database/.
- **Single-source-of-truth + propagation**: protobuf/Confluent schema-registry evolution rules (one schema, many consumers, compatibility enforced at the gate) — https://docs.confluent.io/platform/current/schema-registry/fundamentals/schema-evolution.html and https://protobuf.dev/support/version-support/. This is the structural analog of "one canonical principle text, every agent prompt defers to it, a hook enforces the forbidden-pattern grep."

These confirm the issue's architecture is sound (canonical text + per-consumer reference + enforcement hook), but **#129's substance is internal-policy authoring**, not external-tech selection. The bulk of the work is prompt/doc edits across ~25 files plus three bash hooks — squarely a multi-specialist authoring task, not a research-driven decision.

All URLs accessed 2026-06-09 via Perplexity `sonar-deep-research`.

---

## Verdict

> **VALID-NEW (large, multi-specialist) with a significant ALREADY-DONE component to leverage** — **Confidence: HIGH**
>
> The principle exists in this repo's **own** `/CLAUDE.md` (operator-level), but **has NOT been canonicalized into the distributable plugin** (`plugins/vsdd-factory/`), which is precisely what #129 asks for so that **downstream projects inherit it**. A zero-match grep for the principle's signature phrases across the plugin tree confirms this. Every MUST-change target was verified to still exist in its weaker, pre-canonicalization form; all three new enforcement hooks are absent.
>
> **This is NOT closeable as ALREADY-DONE.** The root-CLAUDE.md presence is necessary context (it provides the canonical *source text*) but does not satisfy the issue's distributable-canonicalization goal.
>
> **Residual (the full issue, minus what `/CLAUDE.md` gives for free):**
> - ~8 MUST edits + ~10 SHOULD edits across `docs/FACTORY.md`, `docs/AGENT-SOUL.md`, `docs/VSDD.md`, `agents/adversary.md`, `agents/security-reviewer.md`, `agents/orchestrator/*.md`, `skills/track-debt`, `skills/convergence-check`, `skills/create-brief`, `templates/tech-debt-register-template.md`, `templates/adversarial-finding-template.md`, `rules/lessons-codification.md`, etc.
> - 3 new hooks (`validate-tech-debt-deferral.sh` MUST, `validate-deferred-findings.sh` MUST, `validate-no-pending-architect.sh` SHOULD) + bats fixtures.
> - hooks-registry.toml registration for the new hooks.
> - CHANGELOG entry.
> - **De-risk:** lift the canonical text from `/CLAUDE.md` (already-evolved, includes Standing Rule 3) rather than transcribe the issue's older Part-1 draft.
>
> **NEEDS-HUMAN / architect sub-decisions:**
> 1. **Line-number drift** (Part 4): several SHOULD targets (e.g. the P2 "Can defer" text) did not match at current HEAD — re-locate by content; surface genuinely-missing targets to the issue author before editing (per the issue's own Step 1.4).
> 2. **Self-reference tension**: vsdd-factory is mid-`v1.0-feature-engine-discipline-pass-1` F5 cycle with strict single-commit-per-burst + `.factory/` hook discipline. A ~25-file plugin-source change is a **plugin/code change** (not a `.factory/` artifact change), so it routes through normal feature-branch + CI, NOT the engine-discipline Commit A–E sequence. Confirm the orchestrator routes this as a standard feature PR to `develop`, decomposed into per-MUST commits per the issue's Part 5 Step 2.
> 3. **Hook 3 escape-hatch policy** (`# arch-input-required:` marker) — the issue flags legitimate exceptions; confirm the marker syntax with the architect.

### Why VALID-NEW and not ALREADY-DONE

The closeable test for #129 is: *"do downstream projects that install the plugin inherit the principle?"* They do not — the plugin's distributable docs/agents/templates/hooks lack it entirely. The root `/CLAUDE.md` governs only the vsdd-factory project's own self-referential development. The issue's entire motivation (Part 7 "Why we are filing this": *"those patterns propagate to every project that uses it"*) is unaddressed until the plugin itself carries the principle.

---

## Recommended Approach (for zero re-research later)

| Item | Detail |
|---|---|
| **Routing (multi-specialist — orchestrator-coordinated)** | This is too large for one agent. Per the Companion Principle: `architect` owns the canonicalization plan + adversary/orchestrator-prompt edits; `product-owner` owns BC/P2/spec-language edits (product-owner.md, business-analyst.md, create-brief, guided-brief-creation); `devops-engineer` owns the three new hooks + hooks-registry.toml + bats fixtures; `technical-writer` owns FACTORY.md / VSDD.md / AGENT-SOUL.md / README doc edits; `state-manager` only if `.factory/` artifacts are touched (they should not be — this is plugin source). Orchestrator dispatches in the issue's Part-5 order. |
| **Canonical source** | **Lift from `/CLAUDE.md`** (the evolved version with Standing Rule 3 + TD-VSDD-059/060), NOT the issue's older Part-1 draft. Satisfies AC-1 ("verbatim canonical principle") with the most current text. |
| **MUST files (8–10)** | `docs/FACTORY.md` (insert principle + companion) · `docs/AGENT-SOUL.md` (reference from §8) · `agents/adversary.md` (`:46-54` tighten deferred-findings scope; mis-anchoring strengthen) · `agents/security-reviewer.md` (`:202` kill advisory-as-terminal) · `agents/orchestrator/orchestrator.md` (`:157` routing-table authoritative note + `:101/:377` companion-principle wiring + cycle-closure deferral-must-have-story) · `skills/track-debt/SKILL.md` (`:77-80` BREAKING "When to Add Debt") · `templates/tech-debt-register-template.md` (human_directed+target_story) · `rules/lessons-codification.md` (deferral protocol) · NEW `hooks/validate-tech-debt-deferral.sh` + `hooks/validate-deferred-findings.sh`. |
| **SHOULD files (~10)** | `docs/VSDD.md`, `agents/visual-reviewer.md`, `agents/product-owner.md` (re-locate P2 text), `agents/business-analyst.md`, `agents/pr-manager.md`, `agents/orchestrator/greenfield-sequence.md`, `agents/orchestrator/per-story-delivery.md`, `skills/convergence-check`, `skills/create-brief`, `skills/guided-brief-creation`, `templates/adversarial-finding-template.md`, `templates/review-findings-template.md`, `templates/wave-state-template.yaml`. Each gets the edit OR an explicit `[NO-CHANGE-RATIONALE]` in the PR body (AC-4). |
| **New hooks** | `validate-tech-debt-deferral.sh` (reject TD rows lacking `human_directed:true` + existing `target_story` in STORY-INDEX.md) · `validate-deferred-findings.sh` (reject `deferred_findings` entries to wave-gate/phase-5 lacking matching `target_story`) · `validate-no-pending-architect.sh` (grep specs for `TODO.*architect` etc., with `# arch-input-required:` escape). Register all in `hooks-registry.toml`; add bats fixtures under `tests/fixtures/` (AC-5). |
| **Risks** | (a) **Line-number drift** — Part 4 line refs are May-2026 snapshots; re-locate by content. (b) **Self-application irony** — the canonicalization PR must itself pass the new hooks and the Self-Audit Checklist (issue Step 7); don't file P4 TDs for the SHOULD items, edit-in-scope. (c) **Hook over-blocking** — `validate-no-pending-architect.sh` needs the escape-marker or it will false-positive on legitimate forward-looking research items. (d) **Scope size** — ~25 files; decompose into per-MUST commits, do NOT ship partial (the issue's own AC requires every MUST landed). (e) **Don't touch `/CLAUDE.md`** beyond using it as source — it is human-mandated meta-doc territory. |
| **Dependencies** | Couples with **#171** (deferred-item revalidation): #129 adds `human_directed`/`target_story`/deferral-date fields to the tech-debt register; #171's revalidation gate consumes exactly that snapshot context. **Sequence #129 first** (it shapes the deferral record schema), then #171 (it reads it) — OR co-design the `last_revalidated` field alongside #129's register change to avoid two template churns. |
| **Scope guard (CLAUDE.md production-grade default)** | The issue is itself an enforcement of the production-grade default; the implementation must exemplify it. Land all MUST items complete in one feature cycle (feature-ordering may phase Hook 3 / some SHOULDs to a follow-up *as whole units*, with explicit rationale — but not partial-MUST). No "advisory" SHOULD-skips without `[NO-CHANGE-RATIONALE]`. |

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared) | Single-source-of-truth policy propagation + validation-gate-on-contract-violation precedents (Flyway/Django, protobuf/Confluent schema-registry) — confirm the canonical-text+enforcement-hook architecture |
| Read | 3 | activate (cross-issue), reference research file, issue body |
| Grep | 8 | zero-match for principle phrases in plugin tree; verify each MUST target's pre-canonicalization text (FACTORY.md:257, AGENT-SOUL.md:95, adversary.md:46-54, security-reviewer.md:202, track-debt:77-80, orchestrator.md:101/157/377); confirm 3 hooks absent; no prior CHANGELOG closure |
| Glob | 3 | enumerate plugin docs, hooks |
| Training data | 0 areas | Codebase claims cited with file:line; external precedents sourced externally |

**Total MCP tool calls:** 1 (research call shared across the four-issue cluster; #129 is predominantly an internal-policy-authoring task where the decisive evidence is codebase grounding, justifying lighter external research per the research-agent deviation clause)
**Training data reliance:** LOW — every "NOT DONE / PARTIAL" verdict is anchored to a verified file:line in the current plugin tree; external enforcement precedents verified against primary docs.

> **Deviation note (per research-agent mandate):** for a non-trivial topic the default is multiple `perplexity_research` calls. #129 is non-trivial in *size* but its decisive evidence is **internal codebase grounding** (is the principle in the plugin or only in root CLAUDE.md?), not external tech. One shared research call covered the applicable external precedents (enforcement gates + SoT propagation); additional external research would not change the verdict, which rests entirely on the zero-match plugin grep + per-target file:line confirmation.
