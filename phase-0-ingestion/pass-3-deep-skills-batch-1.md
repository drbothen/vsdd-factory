# Pass 3 — Deep Skills Batch 1: Per-Skill Behavioral Contracts (Skills 1-40 Alphabetical)

**Date:** 2026-04-25
**Pass:** 3 (Behavioral Contracts) | **Round:** Skills deep batch 1 of N
**Project:** vsdd-factory (engine + product, self-referential ingest)
**BC Range:** BC-AUDIT-200..399 (this batch uses BC-AUDIT-200..360)

## 1. Round metadata

**Goal of this round:** Extract per-skill behavioral contracts to a sufficient density that the spec set can rebuild what currently ships. Each skill yields 3-8 BCs covering identity, triggers, behaviors, quality gates, and outputs.

**Inputs read for context (not re-extracted):**
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` — 86 existing BCs (BC-AUDIT-001..086) covering Rust crates (registry, routing, exec, sinks, hook-sdk).
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` — 57 class-level BCs (BC-AUDIT-087..143) covering hooks-registry distribution, lobster workflows, retraction CONV-ABS-1/2.

**Source files freshly read this batch (per-skill SKILL.md):**

1. `plugins/vsdd-factory/skills/activate/SKILL.md` (79 LOC)
2. `plugins/vsdd-factory/skills/adversarial-review/SKILL.md` (182 LOC)
3. `plugins/vsdd-factory/skills/agent-file-review/SKILL.md` (199 LOC)
4. `plugins/vsdd-factory/skills/analytics-integration/SKILL.md` (193 LOC)
5. `plugins/vsdd-factory/skills/artifact-detection/SKILL.md` (175 LOC)
6. `plugins/vsdd-factory/skills/brainstorming/SKILL.md` (151 LOC)
7. `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` (464 LOC)
8. `plugins/vsdd-factory/skills/check-input-drift/SKILL.md` (241 LOC)
9. `plugins/vsdd-factory/skills/check-state-health/SKILL.md` (115 LOC)
10. `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md` (155 LOC)
11. `plugins/vsdd-factory/skills/code-delivery/SKILL.md` (229 LOC)
12. `plugins/vsdd-factory/skills/compact-state/SKILL.md` (141 LOC)
13. `plugins/vsdd-factory/skills/competitive-monitoring/SKILL.md` (177 LOC)
14. `plugins/vsdd-factory/skills/conform-to-template/SKILL.md` (136 LOC)
15. `plugins/vsdd-factory/skills/consistency-validation/SKILL.md` (289 LOC)
16. `plugins/vsdd-factory/skills/convergence-check/SKILL.md` (115 LOC)
17. `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md` (165 LOC)
18. `plugins/vsdd-factory/skills/create-architecture/SKILL.md` (153 LOC)
19. `plugins/vsdd-factory/skills/create-brief/SKILL.md` (124 LOC)
20. `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md` (122 LOC)
21. `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md` (177 LOC)
22. `plugins/vsdd-factory/skills/create-prd/SKILL.md` (149 LOC)
23. `plugins/vsdd-factory/skills/create-story/SKILL.md` (130 LOC)
24. `plugins/vsdd-factory/skills/customer-feedback-ingestion/SKILL.md` (239 LOC)
25. `plugins/vsdd-factory/skills/deactivate/SKILL.md` (53 LOC)
26. `plugins/vsdd-factory/skills/decompose-stories/SKILL.md` (194 LOC)
27. `plugins/vsdd-factory/skills/deliver-story/SKILL.md` (243 LOC)
28. `plugins/vsdd-factory/skills/demo-recording/SKILL.md` (298 LOC)
29. `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md` (115 LOC)
30. `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md` (149 LOC)
31. `plugins/vsdd-factory/skills/discovery-engine/SKILL.md` (316 LOC)
32. `plugins/vsdd-factory/skills/disposition-pass/SKILL.md` (171 LOC)
33. `plugins/vsdd-factory/skills/dtu-creation/SKILL.md` (125 LOC)
34. `plugins/vsdd-factory/skills/dtu-validate/SKILL.md` (123 LOC)
35. `plugins/vsdd-factory/skills/excalidraw-export/SKILL.md` (52 LOC)
36. `plugins/vsdd-factory/skills/factory-cycles-bootstrap/SKILL.md` (112 LOC)
37. `plugins/vsdd-factory/skills/factory-dashboard/SKILL.md` (69 LOC)
38. `plugins/vsdd-factory/skills/factory-health/SKILL.md` (146 LOC)
39. `plugins/vsdd-factory/skills/factory-obs/SKILL.md` (138 LOC)
40. `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` (206 LOC)

---

## 2. BC Catalog (alphabetical by skill)

### activate (5 BCs)

#### BC-AUDIT-200 — activate: skill identity contract

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** User invokes `/vsdd-factory:activate`
**Behavior:** The skill is the per-project opt-in entry point that flips the default Claude agent to `vsdd-factory:orchestrator:orchestrator`, persists detected platform string, and writes `.claude/settings.local.json`. Reversible via `/vsdd-factory:deactivate`.
**Acceptance:** Frontmatter declares `name: activate`; description references "VSDD factory persona" + ".claude/settings.local.json" + "deactivate".

#### BC-AUDIT-201 — activate: aborts on unsupported platform

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 14-17
**Trigger:** `detect-platform.sh` returns exit code 1 (e.g., FreeBSD, 32-bit host)
**Behavior:** Skill MUST print `detected_from.raw_uname`, tell user no v1.0 dispatcher binary exists for that host, and abort. No file is written.
**Acceptance:** On exit 1 from detect-platform, no settings.local.json mutation occurs and activation is reported aborted.

#### BC-AUDIT-202 — activate: drift warning on host change

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 21-26
**Trigger:** `.vsdd-factory.activated_platform` exists in settings.local.json AND does not match current detected platform
**Behavior:** Skill MUST surface a warning that quotes both the persisted platform and the current host platform; activation continues after warning and the new platform overwrites the persisted value.
**Acceptance:** Warning string contains both `<persisted>` and `<current>` strings; settings.local.json is updated post-warning.

#### BC-AUDIT-203 — activate: writes activation block with three named fields

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 27-40
**Trigger:** All preflight checks pass
**Behavior:** Skill writes `.claude/settings.local.json` merging top-level `agent` plus a `vsdd-factory` block with exactly `activated_platform`, `activated_at` (ISO 8601 with timezone), and `activated_plugin_version`. Other top-level keys MUST be preserved.
**Acceptance:** Resulting JSON has `agent == "vsdd-factory:orchestrator:orchestrator"` and `vsdd-factory.{activated_platform,activated_at,activated_plugin_version}` present; pre-existing keys still present.

#### BC-AUDIT-204 — activate: dry-run mode performs no writes

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-62
**Trigger:** User invokes with `--dry-run`
**Behavior:** Skill performs steps 1-4 (detection + drift check) but skips file write and hooks.json copy; prints proposed settings diff and platform that would be persisted.
**Acceptance:** No filesystem mutation observable; printed output names the platform that would be persisted.

#### BC-AUDIT-205 — activate: applies per-platform variant via apply-platform.sh

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 42-48
**Trigger:** Successful detection + write of activation block
**Behavior:** Skill MUST run `apply-platform.sh <platform>` which copies `hooks.json.<platform>` to `hooks.json` and verifies dispatcher binary at `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`. Exit codes 0/1/2/3/4 are surfaced to user with helper's stderr verbatim.
**Acceptance:** Helper's stderr is shown verbatim; binary-missing case (exit 2) is documented as expected on fresh install pre-S-2.4.

---

### adversarial-review (7 BCs)

#### BC-AUDIT-206 — adversarial-review: skill identity contract

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8
**Trigger:** Orchestrator or user invokes `/vsdd-factory:adversarial-review`
**Behavior:** Skill launches a fresh-context adversary agent against specs or implementation; uses information asymmetry; minimum 3 clean passes to convergence; runs in forked context; routes to `adversary` subagent.
**Acceptance:** Frontmatter has `agent: adversary`, `context: fork`, `disable-model-invocation: false`.

#### BC-AUDIT-207 — adversarial-review: announces verbatim before any other action

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-29
**Trigger:** Skill invocation
**Behavior:** Before any tool call, skill MUST emit verbatim: "I'm using the adversarial-review skill to launch a fresh-context adversary pass on <target>." Then create TodoWrite entries (minimum 2 passes).
**Acceptance:** First skill output line matches the verbatim string with `<target>` substituted.

#### BC-AUDIT-208 — adversarial-review: minimum 3 consecutive clean passes for convergence

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 39-40, 165-168
**Trigger:** Adversary returns "no findings" or low-novelty findings on a pass
**Behavior:** Convergence requires 3 consecutive clean passes (not 2). Maximum 10 passes before escalating to human. Minimum 3 enforced even when "Novelty is LOW after one pass."
**Acceptance:** Pass count >=3 AND last 3 passes report no substantive findings before convergence is declared.

#### BC-AUDIT-209 — adversarial-review: filename collision guard pre-flight

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-87
**Trigger:** Before writing `.factory/cycles/<current>/adversarial-reviews/pass-<N>.md`
**Behavior:** Skill MUST check if target exists with different content; REFUSE the write and emit error `"Collision: <target-path> already exists with different content. Use a different cycle name or pass number."`
**Acceptance:** No silent overwrite of historical reviews; error path tested for differing-content case.

#### BC-AUDIT-210 — adversarial-review: policy rubric auto-loading from policies.yaml

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 106-125
**Trigger:** Before dispatching adversary agent
**Behavior:** Skill MUST read `.factory/policies.yaml`, format each policy as a rubric block (id, name, description, severity, scope, verification steps), and append all blocks under `## Project Policy Rubric` heading in adversary's task prompt. Adversary executes verification steps and reports compliance per-policy.
**Acceptance:** Adversary task prompt contains a `## Project Policy Rubric` section with one block per policy in policies.yaml.

#### BC-AUDIT-211 — adversarial-review: post-adversary persistence via state-manager

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 131-145
**Trigger:** Adversary returns findings as chat text
**Behavior:** Adversary cannot Write — orchestrator MUST capture full output verbatim, determine target path `.factory/cycles/<current-cycle>/adversarial-reviews/pass-<N>.md`, dispatch state-manager to write it, and dispatch state-manager to update `ADV-P<N>-INDEX.md`.
**Acceptance:** Findings file exists at the target path AND the index is updated, both via state-manager subagent dispatch.

#### BC-AUDIT-212 — adversarial-review: trajectory monotonicity (findings never increase)

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 170-178
**Trigger:** Pass N has more findings than pass N-1
**Behavior:** Skill MUST stop and investigate; this is a regression. Causes: new scope without invariant pre-validation, new defect from a fix, perimeter expansion. Convergence MUST NOT continue until regression is explained and resolved.
**Acceptance:** Pass-over-pass finding count is monotonically non-increasing OR convergence is paused with documented root-cause investigation.

---

### agent-file-review (5 BCs)

#### BC-AUDIT-213 — agent-file-review: skill identity contract

**Skill:** `plugins/vsdd-factory/skills/agent-file-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** When creating/modifying an agent's AGENTS.md, debugging agent misbehavior, periodic audit, or before commit.
**Behavior:** Reviews AGENTS.md for compliance with Dark Factory agent design principles: token budget, contradictions, negative examples, FACTORY.md duplication, tool profile mismatches, and structural compliance with canonical template.
**Acceptance:** Frontmatter declares `name: agent-file-review`; description names the 5 review categories.

#### BC-AUDIT-214 — agent-file-review: token budget thresholds (PASS/WARN/FAIL)

**Skill:** `plugins/vsdd-factory/skills/agent-file-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-46
**Trigger:** Each AGENTS.md reviewed
**Behavior:** Counts words via `wc -w`. PASS if <=3,500; WARN if 3,501-5,000; FAIL if >5,000 (compliance degradation severe due to "lost middle" effect).
**Acceptance:** Word count check produces exactly one of {PASS, WARN, FAIL} matching the thresholds; FAIL forces split recommendation.

#### BC-AUDIT-215 — agent-file-review: 15-check list runs all checks

**Skill:** `plugins/vsdd-factory/skills/agent-file-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-153
**Trigger:** Skill invocation per agent
**Behavior:** Runs ALL 15 numbered checks: token budget, global header, hard constraints in first 20%, recency restatement, no negative code examples, no model names, no pipeline position references, tool profile match, internal contradictions, FACTORY.md duplication, contract-based structure, escalation levels, context discipline, information wall, agent-tool usage in non-orchestrator agents.
**Acceptance:** Output table has exactly 15 numbered rows with Check/Result/Details columns.

#### BC-AUDIT-216 — agent-file-review: tool profile match against openclaw.json (FAIL on mismatch)

**Skill:** `plugins/vsdd-factory/skills/agent-file-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 94-102
**Trigger:** Check 8 of review
**Behavior:** Reads agent's tool profile from `openclaw.json` and compares to "Tool Restrictions"/"Tool Access" section in AGENTS.md. Reports FAIL on mismatch with examples like "AGENTS.md says 'messaging' but config says 'full' with deny list."
**Acceptance:** When config and AGENTS.md disagree, the check returns FAIL and identifies both stated values.

#### BC-AUDIT-217 — agent-file-review: batch summary mode

**Skill:** `plugins/vsdd-factory/skills/agent-file-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 182-198
**Trigger:** Target == "all"
**Behavior:** Produces a summary table with columns Agent / Words / FAIL / WARN / PASS / Top Issue, plus an "Agents needing attention (sorted by FAIL count)" list.
**Acceptance:** Batch summary table is emitted with all 6 columns and agents are sorted by FAIL count descending.

---

### analytics-integration (4 BCs)

#### BC-AUDIT-218 — analytics-integration: skill identity contract (optional, no-op when not configured)

**Skill:** `plugins/vsdd-factory/skills/analytics-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15, 178-179
**Trigger:** Scheduled (weekly/configured) or manual; ONLY runs if `analytics.enabled == true` for the target product
**Behavior:** Reads product analytics data (file or API) to identify feature adoption, error patterns, usage signals. The factory does NOT implement telemetry in the product. Exit no-op if not enabled.
**Acceptance:** Skill exits with no-op when `analytics.enabled != true`; never writes telemetry into the product.

#### BC-AUDIT-219 — analytics-integration: feature health classification thresholds

**Skill:** `plugins/vsdd-factory/skills/analytics-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 97-106
**Trigger:** Per-feature analysis
**Behavior:** Classifies each feature as Healthy (>20% adoption AND stable/growing), Concerning (declining OR <10% after 30+ days), or Unused (<2% after 30+ days — deprecation candidate).
**Acceptance:** Each feature in digest has exactly one health classification matching the thresholds.

#### BC-AUDIT-220 — analytics-integration: error severity classification

**Skill:** `plugins/vsdd-factory/skills/analytics-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 108-116
**Trigger:** Per-feature error analysis
**Behavior:** HIGH severity if error rate >5% or rising trend; MEDIUM if 1-5% and stable; LOW if <1%.
**Acceptance:** Each feature error row in digest carries HIGH/MED/LOW classification matching the thresholds.

#### BC-AUDIT-221 — analytics-integration: digest output path and quality gate

**Skill:** `plugins/vsdd-factory/skills/analytics-integration/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 13-15, 178-186
**Trigger:** Successful run
**Behavior:** Writes `.factory/discovery/analytics-digest-YYYY-MM-DD.md` with frontmatter document_type/date/product/period/sources_read; quality gate requires features+trends, errors+severity, signals synthesized, missing/stale data noted.
**Acceptance:** Digest exists at expected path with correct frontmatter; quality-gate checklist items are addressed.

---

### artifact-detection (5 BCs)

#### BC-AUDIT-222 — artifact-detection: skill identity contract (universal pipeline front-end)

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** START of every pipeline invocation, before spec crystallization
**Behavior:** Scans for existing planning artifacts (brief, PRD, architecture, UX spec, stories), validates quality, identifies gaps, routes to correct pipeline entry point. Replaces former first step "Receive Product Brief" — does not assume human arrives with a finished brief.
**Acceptance:** Skill is the documented Step 0 of every pipeline mode; description names "universal front-end."

#### BC-AUDIT-223 — artifact-detection: 5-tier readiness classification (L0-L4)

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-72
**Trigger:** After artifact scan complete
**Behavior:** Classifies project as L0 (nothing) → Collaborative Discovery; L1 (brief) → spec crystallization; L2 (PRD) → remaining crystallization; L3 (PRD+arch+L3 BCs) → story decomposition; L4 (PRD+arch+L3+L4+stories) → implementation.
**Acceptance:** Output declares exactly one of {L0, L1, L2, L3, L4} per the artifact set found.

#### BC-AUDIT-224 — artifact-detection: format detection flags FR-NNN legacy for migration

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 46-58, 174
**Trigger:** Spec uses FR-NNN format (legacy)
**Behavior:** Skill MUST flag FR-NNN format for migration to BC-S.SS.NNN before proceeding. Does not silently skip legacy format.
**Acceptance:** When FR-NNN is detected, output includes a "migration required" recommendation; pipeline does not continue without acknowledgment.

#### BC-AUDIT-225 — artifact-detection: writes 3 routing artifacts

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 164-169
**Trigger:** End of skill execution
**Behavior:** Writes three files: `.factory/planning/artifact-inventory.md` (what was found), `.factory/planning/gap-analysis.md` (validation results), `.factory/planning/routing-decision.md` (entry point selected).
**Acceptance:** All three files exist after skill completes.

#### BC-AUDIT-226 — artifact-detection: failure modes (no .factory/, corruption, legacy)

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 171-175
**Trigger:** Various failure conditions
**Behavior:** Missing `.factory/` → report L0 → route Discovery; corrupt artifacts (unparseable frontmatter, truncated) → flag specific corruption + recommend re-creation; legacy FR-NNN → flag for migration, do not silently skip.
**Acceptance:** Each failure mode is handled per the documented branch rather than crashing or silent fallback.

---

### brainstorming (4 BCs)

#### BC-AUDIT-227 — brainstorming: skill identity + hard gate

**Skill:** `plugins/vsdd-factory/skills/brainstorming/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** L0 detection where human chooses brainstorming, or explicit request
**Behavior:** Guided brainstorming using SCAMPER, reverse brainstorming, mind mapping, constraint removal, six thinking hats. Hard gate: skill MUST NOT skip to brief creation, spec writing, or implementation. Brainstorming report MUST be written and human MUST select a direction first.
**Acceptance:** Skill produces report and a selected direction before any downstream skill is invoked.

#### BC-AUDIT-228 — brainstorming: 6 named techniques and selection logic

**Skill:** `plugins/vsdd-factory/skills/brainstorming/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-86
**Trigger:** Step 2 (Technique Selection)
**Behavior:** Recommends 2-3 techniques from a fixed catalog of 6: Brain dump, SCAMPER, Reverse brainstorming, Mind mapping, Constraint removal, Six thinking hats — each tied to a "Best For" use case.
**Acceptance:** Skill output names at least 2 techniques from the 6-item catalog.

#### BC-AUDIT-229 — brainstorming: every idea goes through process even when "obvious"

**Skill:** `plugins/vsdd-factory/skills/brainstorming/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-46
**Trigger:** User declares "this is too simple to need brainstorming" or any of the 7 red-flag thoughts
**Behavior:** Skill MUST NOT skip — even CLI flags, single endpoints, config changes go through process. Session can be short (10 min, one technique) but cannot be skipped.
**Acceptance:** When user requests skip, skill explicitly explains the anti-pattern and proceeds with at minimum one technique.

#### BC-AUDIT-230 — brainstorming: report output and quality gate

**Skill:** `plugins/vsdd-factory/skills/brainstorming/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 113-150
**Trigger:** End of session
**Behavior:** Writes `.factory/planning/brainstorming-report.md` containing session summary, ALL ideas (including discarded), themes/groupings, selected direction, open questions, recommended next step. Quality gate: ≥2 distinct ideas, human selected direction, report written, direction includes problem+solution+audience+differentiator.
**Acceptance:** Report exists at expected path with all 4 quality-gate items satisfied.

---

### brownfield-ingest (8 BCs)

#### BC-AUDIT-231 — brownfield-ingest: skill identity (broad-then-converge protocol)

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** User invokes `/brownfield-ingest <path-or-url> [--resume]`
**Behavior:** Analyzes existing codebase via 6 broad passes (0-6) followed by iterative deepening on every pass until novelty decays to NITPICK. Produces complete semantic understanding feeding spec crystallization.
**Acceptance:** Frontmatter declares `name: brownfield-ingest`; argument-hint matches `[codebase-path] [--resume]`.

#### BC-AUDIT-232 — brownfield-ingest: announces verbatim and creates phase A/B/B5/B6/C TodoWrite entries

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-23
**Trigger:** Skill invocation
**Behavior:** Before any other action, skill MUST emit verbatim: "I'm using the brownfield-ingest skill to run the broad-then-converge analysis protocol on <project>." Then create TodoWrite entries for Phase A (7 passes), Phase B (deepening), Phase B.5 (coverage audit), Phase B.6 (extraction validation), Phase C (final synthesis).
**Acceptance:** First skill output line matches verbatim string AND TodoWrite list contains entries for all 5 phases.

#### BC-AUDIT-233 — brownfield-ingest: source acquisition into .reference/

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 68-82
**Trigger:** Step 0 of the protocol
**Behavior:** Git URL → `git clone --depth=1 <url> .reference/<project>/`; local path outside `.reference/` → copy/move; already in `.reference/` → no-op. Updates `.factory/reference-manifest.yaml` per template. Creates `.factory/semport/<project>/` output dir.
**Acceptance:** `.reference/<project>/` exists with source AND `reference-manifest.yaml` has new entry with url/SHA/date/depth/focus/status.

#### BC-AUDIT-234 — brownfield-ingest: strict-binary novelty (NITPICK token only)

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 213-232
**Trigger:** Each deepening round assesses novelty
**Behavior:** Only the literal token `NITPICK` in agent's Novelty Assessment counts as convergence. Orchestrator MUST ignore self-declarations like "borderline NITPICK", "effectively converged", "convergence declared", "functionally complete", "another round may be needed but probably nitpick", "recommend halting" — these all mean SUBSTANTIVE.
**Acceptance:** Convergence trigger fires only on literal `NITPICK` token; softer phrasings trigger another round.

#### BC-AUDIT-235 — brownfield-ingest: minimum 2 deepening rounds, no fixed maximum

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 176-183
**Trigger:** Each pass during deepening
**Behavior:** Minimum 2 deepening rounds per pass before declaring NITPICK. No fixed maximum — rounds continue until honest NITPICK. Empirical: Vault Pass 2 needed 62 rounds. Passes 2 and 3 deepen first; then 0,1,4,5. One repo per agent always.
**Acceptance:** Round count >= 2 per pass before convergence; SUBSTANTIVE always triggers round N+1 regardless of agent self-declaration.

#### BC-AUDIT-236 — brownfield-ingest: honest convergence clause in every round prompt

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 234-238
**Trigger:** Every deepening-round prompt
**Behavior:** Round prompts MUST include the verbatim clause requiring agents to declare convergence and emit no updated file ("converged, no file emitted") if they find <3 substantive items. Fabricating findings is strictly worse than stopping. Default to NITPICK when uncertain.
**Acceptance:** Round prompts contain the literal honest-convergence paragraph.

#### BC-AUDIT-237 — brownfield-ingest: Phase B.5 coverage audit is mandatory

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 264-303
**Trigger:** After all passes reach NITPICK
**Behavior:** Must run grep-driven (not agent-judgment) coverage audit. Inventory source tree, grep deep files for package/subsystem references, flag zero/minimal hits as blind spots. One agent per project. Loop until no substantive gaps remain. Empirically every secrets-corpus repo had blind spots after 19-62 rounds — B.5 is the only check that catches them.
**Acceptance:** `<project>-coverage-audit.md` exists; subsequent passes have either PASS verdict or surgical mini-rounds (`-phase-b5-tr-N.md`) that close gaps.

#### BC-AUDIT-238 — brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 305-359
**Trigger:** After B.5 passes
**Behavior:** validate-extraction agent splits work into Phase 1 (behavioral verification using judgment — sample contracts/entities/relationships against source) and Phase 2 (metric verification using arithmetic — recount every numeric claim). Two tables, one per phase. Up to 3 refinement iterations.
**Acceptance:** `<project>-extraction-validation.md` exists with two tables (behavioral + metric); metric table compares claimed vs recounted values; PASS/FAIL verdict rendered after ≤3 iterations.

---

### check-input-drift (4 BCs)

#### BC-AUDIT-239 — check-input-drift: skill identity contract

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** Before phase gates, after spec edits, during maintenance sweeps, after pipeline resume
**Behavior:** Scans all `.factory/` artifacts for input-hash drift; recomputes hashes from current input files; reports stale artifacts. With `--fix`, batch-updates stale hashes.
**Acceptance:** Frontmatter declares `name: check-input-drift`; argument-hint `[--fix]`.

#### BC-AUDIT-240 — check-input-drift: scan via single binary, not inline shell loops

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 31-58
**Trigger:** Step 1 of skill
**Behavior:** Runs `${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory`. Skill MUST NOT use inline bash loops — Claude Code harness auto-backgrounds multi-line commands and kills before output flushes. Output: `TOTAL=N MATCH=N STALE=N UNCOMPUTED=N NOINPUT=N UPDATED=0 UPDATE_FAILED=0`. Exit 0=clean, 2=drift.
**Acceptance:** Single-command binary invocation; output format matches; INDEX files are skipped by binary.

#### BC-AUDIT-241 — check-input-drift: mandatory resolve step after scan

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 61-93
**Trigger:** After Step 1 scan completes
**Behavior:** Skill MUST always run `--scan .factory --resolve`. Files with missing inputs cannot be hashed; the binary refuses to hash partial input sets because partial hash produces false MATCH. If UNRESOLVABLE>0, diagnose before proceeding. NOT skipping this step is mandatory.
**Acceptance:** Resolve invocation always follows scan; UNRESOLVABLE>0 blocks proceeding without diagnosis.

#### BC-AUDIT-242 — check-input-drift: cluster-drift triage before bulk --update

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 147-216
**Trigger:** Cluster patterns detected (all shards drift, all subsystem files drift, >5 artifacts share upstream, PRD+supplements drift, all VPs in family drift)
**Behavior:** Skill MUST STOP before running `--update`. Reads upstream diff first; if cosmetic → proceed; if semantic → dispatch producing agent (business-analyst, product-owner, architect, story-writer per artifact type) for content review. After review, run `--update` then re-scan to verify zero drift and no cascading staleness.
**Acceptance:** Cluster pattern detection triggers triage flow; bulk `--update` is only run after content-review or after explicit "bulk update, no investigation" override.

---

### check-state-health (4 BCs)

#### BC-AUDIT-243 — check-state-health: skill identity (read-only diagnostic)

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-11
**Trigger:** At session start, after phase transitions, before declaring convergence, when state-size hook fires
**Behavior:** Validates STATE.md against VSDD standards. Reports HEALTHY, WARNINGS, or NEEDS-COMPACT. Reads and reports — does NOT modify any files. `disable-model-invocation: true`; allowed-tools: Read, Glob, Grep, Bash.
**Acceptance:** No write tool invocation observable; final verdict is exactly one of {HEALTHY, WARNINGS, NEEDS-COMPACT}.

#### BC-AUDIT-244 — check-state-health: 7 numbered checks executed in order

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-93
**Trigger:** Skill invocation
**Behavior:** Runs (1) Existence; (2) Frontmatter validation against fixed schema; (3) Size check (≤200 HEALTHY, 201-500 WARNING, 501+ NEEDS-COMPACT); (4) Phase numbering (no Phase 3.5/4.x adversar/5.x formal/6.x converg); (5) Structure compliance (6 required sections); (6) Content routing compliance (5 antipatterns); (7) Convergence counter format `N of 3`.
**Acceptance:** Output table has exactly 7 numbered rows.

#### BC-AUDIT-245 — check-state-health: stale-phase detection patterns

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 54-62
**Trigger:** Check 4
**Behavior:** Greps STATE.md for stale phase references and reports each with line number: `Phase 3.5` or `phase: 3.5` → 4; `Phase 4.*adversar` → 5; `Phase 5.*formal` or `Phase 5.*harden` → 6; `Phase 6.*converg` → 7.
**Acceptance:** Stale-phase findings include exact line numbers; non-standard compound phases (e.g., `2-story-decomposition-patch-cycle`) flagged.

#### BC-AUDIT-246 — check-state-health: content-routing antipattern catalog

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 78-89
**Trigger:** Check 6
**Behavior:** Detects 5 antipatterns: >10 `## Burst` or `## Pass` sections (route → burst-log.md); >1 `## Session Resume Checkpoint` (route → session-checkpoints.md); >20 `adversary_pass_*` frontmatter fields (route → convergence-trajectory.md); `## Lessons` with >5 entries (route → lessons.md); resolved blocking issues still listed (route → blocking-issues-resolved.md).
**Acceptance:** Each antipattern detection cites the destination cycle file; threshold counts match.

---

### claude-telemetry (4 BCs)

#### BC-AUDIT-247 — claude-telemetry: skill identity (manage 5 OTEL_* env vars)

**Skill:** `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9, 56-66
**Trigger:** User asks to enable/disable Claude OTel telemetry, check env vars, troubleshoot Grafana
**Behavior:** Writes 5 OTEL env vars (CLAUDE_CODE_ENABLE_TELEMETRY=1, OTEL_METRICS_EXPORTER=otlp, OTEL_LOGS_EXPORTER=otlp, OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf, OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318) to `.claude/settings.local.json` under `env` block. Reversible; local-only. allowed-tools: Read, Write, Bash.
**Acceptance:** `env` block in settings.local.json contains exactly the 5 keys with the documented values.

#### BC-AUDIT-248 — claude-telemetry: 3 modes (on/off/status)

**Skill:** `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-55
**Trigger:** Argument parsing
**Behavior:** `on` (default) writes the 5 keys + prunes legacy 6th key (OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE); `off` removes those 6 keys; `status` prints which of the 5 are set; unrecognized arg → show status + usage hint.
**Acceptance:** Each mode produces the documented mutation and confirmation output.

#### BC-AUDIT-249 — claude-telemetry: prunes legacy temporality key

**Skill:** `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-29, 73-86, 96-114
**Trigger:** `on` mode (and `off` mode)
**Behavior:** As of v0.76.0 the deltatocumulative collector processor handles cumulative conversion. The legacy 6th env var is no longer needed. `on` prunes it via `del(.env.OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE)` after merging the 5 current keys. `off` deletes it as well.
**Acceptance:** After `on`, the legacy key is absent from settings.local.json regardless of prior state.

#### BC-AUDIT-250 — claude-telemetry: prominent restart reminder after `on`

**Skill:** `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-37, 88-94
**Trigger:** Successful `on` mutation
**Behavior:** Skill MUST print prominent restart reminder ("Restart your Claude Code session so the env vars take effect") and confirm collector is up via `docker ps --filter name=vsdd-obs-collector` (or recommend `factory-obs up`). Also mentions LogQL query `{service_name="claude-code"}`.
**Acceptance:** Final user-facing output of `on` contains the verbatim restart string.

---

### code-delivery (6 BCs)

#### BC-AUDIT-251 — code-delivery: skill identity (post-convergence delivery)

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-8, 13-18
**Trigger:** Greenfield (per story per wave + convergence fixes); Feature mode (after F7 delta convergence); Maintenance (after sweep findings); Multi-repo (per story per repo)
**Behavior:** Post-convergence workflow: pushes verified code, creates PRs with structured evidence, waits for CI, executes merge based on autonomy level.
**Acceptance:** Skill is invoked after convergence (not before); supports all 4 modes named.

#### BC-AUDIT-252 — code-delivery: pre-push test gate via before_tool_call hook

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 64-74
**Trigger:** `git push` to remote in Step 3
**Behavior:** code-delivery's `before_tool_call` hook verifies tests have passed before allowing push. If tests haven't passed, push is blocked with `{ skip: true, reason: "Tests must pass before push" }`. Push uses `--force-with-lease`.
**Acceptance:** Push blocked when tests not green; `--force-with-lease` used (not raw `--force`).

#### BC-AUDIT-253 — code-delivery: per-AC demo evidence with both success and error paths

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-63
**Trigger:** Step 2 (per-story demo recording in worktree)
**Behavior:** For each AC, create recording script from template, execute, verify output. Record BOTH success AND error paths for each AC. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`. Demo artifacts go to feature branch (per-story subfolder), NOT `.factory/`. Gate: at least 1 recording (`.gif`/`.webm`, NOT `.txt`) per AC, both paths.
**Acceptance:** Per-story subfolder exists with evidence-report.md and at least 1 binary recording per AC covering both success and error.

#### BC-AUDIT-254 — code-delivery: 4-model-family review (4th model in pr-reviewer)

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 99-117
**Trigger:** Step 5 after PR creation
**Behavior:** pr-reviewer reviews PR diff with fresh context using a different model family from implementer, adversary, code-reviewer (the 4th family). Reviews 8 checklist items. Verdict: APPROVE / REQUEST_CHANGES / COMMENT. Findings posted as inline PR comments via github-ops AND written to `.factory/code-delivery/STORY-NNN/pr-review.md`.
**Acceptance:** pr-reviewer model differs from implementer/adversary/code-reviewer model AND output covers all 8 checklist items.

#### BC-AUDIT-255 — code-delivery: review convergence loop (max 10 cycles)

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 119-134
**Trigger:** pr-reviewer returns REQUEST_CHANGES
**Behavior:** Triage (pr-manager) → Fix (implementer/test-writer/demo-recorder) → Re-review (pr-reviewer). Tracked in `.factory/code-delivery/STORY-NNN/review-findings.md` with cycle/findings/blocking/fixed/remaining columns. Max 10 cycles before escalating to human.
**Acceptance:** Cycle count ≤10; review-findings.md exists with the 5-column tracking table.

#### BC-AUDIT-256 — code-delivery: autonomy-level-driven merge decision

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 164-185
**Trigger:** Step 9 after CI green and dependencies merged
**Behavior:** Reads `.factory/merge-config.yaml`. Level 3: needs-review label, wait for human. Level 3.5: low-risk auto-merge, medium/high requires human (security-critical files always require human). Level 4: auto-merge with squash if CI passes; feature flag wrapping recommended for high-risk.
**Acceptance:** Merge decision aligns with the autonomy level documented in merge-config.yaml.

---

### compact-state (4 BCs)

#### BC-AUDIT-257 — compact-state: skill identity (extract historical content from STATE.md)

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-11
**Trigger:** STATE.md health flagged WARNING/NEEDS-COMPACT or operator-invoked
**Behavior:** Extracts historical content from bloated STATE.md into cycle files (burst-log, convergence-trajectory, session-checkpoints, lessons, blocking-issues-resolved). Slims STATE.md to <200 lines.
**Acceptance:** Frontmatter declares `disable-model-invocation: false`; allowed-tools include Read, Write, Edit, Bash, Glob, Grep.

#### BC-AUDIT-258 — compact-state: 7-pattern extraction map

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 42-55
**Trigger:** Step 2 (Identify Extractable Content)
**Behavior:** Extracts 7 patterns to specific cycle files: `## Burst N` → burst-log.md; `## Pass N` → convergence-trajectory.md; `adversary_pass_*` frontmatter → convergence-trajectory.md; old `## Session Resume Checkpoint` (all but last) → session-checkpoints.md; `## Lessons` → lessons.md; resolved blockers → blocking-issues-resolved.md; `## Session Chain Summary` → session-checkpoints.md.
**Acceptance:** Each detected pattern routes to the documented destination file in `cycles/<cycle>/`.

#### BC-AUDIT-259 — compact-state: never-deletes safety guarantee

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 135-141
**Trigger:** Any extraction operation
**Behavior:** Skill ONLY moves content between files — never deletes. All extracted content is written to cycle files BEFORE removed from STATE.md. If any write fails, abort without modifying STATE.md. Git commit captures slim STATE.md and new cycle files atomically.
**Acceptance:** Sum of extracted content + remaining STATE.md = original content (no loss); abort path tested.

#### BC-AUDIT-260 — compact-state: post-compaction STATE.md <200 lines + verify

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-110
**Trigger:** After extraction in Step 4-5
**Behavior:** Rewrites STATE.md keeping only: frontmatter (sans adversary_pass_*), Project Metadata, Phase Progress, last-5 Current Phase Steps, Decisions Log, Skip Log, open Blocking Issues, Phase Numbering Reconciliation if present, latest Session Resume Checkpoint only. Replaces extracted sections with pointer block. Verifies <200 lines.
**Acceptance:** Final STATE.md line count < 200 AND all 9 retained sections present AND adversary_pass_* fields removed from frontmatter.

---

### competitive-monitoring (3 BCs)

#### BC-AUDIT-261 — competitive-monitoring: skill identity contract

**Skill:** `plugins/vsdd-factory/skills/competitive-monitoring/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-16
**Trigger:** Scheduled (weekly), manual ("Check competitor activity"), or event-driven (news alerts)
**Behavior:** Monitors competitor activity (releases, pricing, funding, acquisitions, shutdowns, new entrants). Produces structured competitive update report, updates competitive baseline. Primary agent: research-agent.
**Acceptance:** Frontmatter declares primary agent research-agent; outputs `.factory/discovery/competitive-update-YYYY-MM-DD.md` and updated baseline.

#### BC-AUDIT-262 — competitive-monitoring: urgency classification HIGH/MEDIUM/LOW

**Skill:** `plugins/vsdd-factory/skills/competitive-monitoring/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 110-118
**Trigger:** Each detected change classified
**Behavior:** HIGH = competitor ships feature customers request OR competitor acquired OR competitor shuts down; MEDIUM = pricing change OR new entrant OR funding (acceleration signal); LOW = minor release OR informational funding OR no roadmap impact.
**Acceptance:** Each detected change carries exactly one HIGH/MED/LOW classification matching the criteria.

#### BC-AUDIT-263 — competitive-monitoring: VERIFIED/UNVERIFIED flagging on findings

**Skill:** `plugins/vsdd-factory/skills/competitive-monitoring/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 159-176
**Trigger:** Each finding produced
**Behavior:** Findings cite source URLs and dates; VERIFIED requires multiple independent sources, UNVERIFIED for single-source. Quality criteria: implications assessed (not just "competitor shipped X" but "this means Y for us"); previous baseline updated with new state changes.
**Acceptance:** Each finding has source URL+date and VERIFIED/UNVERIFIED flag.

---

### conform-to-template (4 BCs)

#### BC-AUDIT-264 — conform-to-template: skill identity (additive only — never deletes)

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-25
**Trigger:** After validate-template-compliance reports WARN/FAIL, during template migration, or when manually-created artifacts need structure
**Behavior:** Fixes structural gaps by adding missing frontmatter fields, sections, table structures from the template. Safety guarantees: never deletes, never modifies existing content, always shows changes before applying, creates backup `<filename>.backup-YYYY-MM-DD-HHMMSS`, single file per invocation.
**Acceptance:** Argument parsing accepts `<file-path> [--template=<template-name>]`; backup file exists post-modification.

#### BC-AUDIT-265 — conform-to-template: refuses table-column changes and section reordering

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 67-77
**Trigger:** Template defines column or order not in target
**Behavior:** Skill MUST report missing table columns AND section order violations but MUST NOT auto-fix them. Reports the mismatch as "Manual fix required" / "Manual reorder recommended" because adding empty columns or moving sections breaks semantics/cross-references.
**Acceptance:** Output flags column mismatches and ordering violations as manual fixes; no automatic mutation occurs for these cases.

#### BC-AUDIT-266 — conform-to-template: user approval gate before write

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 80-115
**Trigger:** After plan computed
**Behavior:** Presents diff (frontmatter additions, section additions, manual fixes needed). Waits for `yes / no / edit` approval. On `no`, stops without changes. On `edit`, allows user to modify plan. On `yes`, creates backup, adds frontmatter fields before closing `---`, adds `[TODO: populate this section per template]` placeholders.
**Acceptance:** No filesystem mutation observable until explicit `yes`; placeholder format matches the template literal `[TODO: populate this section per template]`.

#### BC-AUDIT-267 — conform-to-template: post-conformance re-validation reports before/after

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 117-125
**Trigger:** After applying changes
**Behavior:** Runs validate-template-compliance on modified file. Reports `Before: ... → Overall WARN` and `After: ... → Overall PASS` (or whatever current verdict is).
**Acceptance:** Skill output includes both Before and After lines summarizing structural compliance change.

---

### consistency-validation (5 BCs)

#### BC-AUDIT-268 — consistency-validation: skill identity (cross-document validator)

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** At phase gates and during convergence checks
**Behavior:** Cross-document validation skill. Checks alignment between PRD, architecture, UX specs, stories, and implementation artifacts.
**Acceptance:** Frontmatter declares `name: consistency-validation`; description names PRD/architecture/UX/stories.

#### BC-AUDIT-269 — consistency-validation: 36 numbered rules executed in order

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 21-289
**Trigger:** Skill invocation
**Behavior:** Runs Rules 0-36 in order: spec format (DF-020), PRD→Epic, Epic→Story, Story→Architecture, Story→UX, AC testability, VP coverage, Dependency acyclicity, Data model consistency, Performance target alignment, Purity boundary consistency, Semantic drift detection, Token budget, Upstream traceability chain, Downstream traceability completeness, L1→L2/L2→L3/L3→L4/L1→L4 chain, BC-to-Story, AC-to-BC, VP registry completeness, Design system token compliance, Component contract compliance, UI traceability, BC clause reverse coverage, EC+E error reverse coverage, NFR-to-Story reverse coverage, Holdout-BC-AC alignment, UI state completeness, PRD scope+differentiator enforcement, PRD RTM completeness, Frontmatter cross-reference integrity, BC lifecycle field coherence, FM-NNN to holdout coverage, L2 sharding integrity, UX sharding integrity (UI products only).
**Acceptance:** Output table contains exactly 36 numbered rule rows with Status (PASS/FAIL) and violation lists.

#### BC-AUDIT-270 — consistency-validation: index-first validation precedes detail loading (DF-021)

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 9-19
**Trigger:** Before any detail-file reads
**Behavior:** Validates ARCH-INDEX.md, BC-INDEX.md, VP-INDEX.md, ADV-P[N]-INDEX.md, EVAL-INDEX.md reference all existing files; every detail file has `traces_to:` pointing at its index.
**Acceptance:** Index references match actual file set; orphans (files not in index) and dangling refs (in index but no file) flagged.

#### BC-AUDIT-271 — consistency-validation: BC clause reverse-coverage severity (Rule 25)

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 187-193
**Trigger:** Rule 25 execution
**Behavior:** Every numbered precondition/postcondition/invariant clause in active BCs MUST have at least one AC tracing to it. Gap Register entries with non-empty justification (min 10 chars) count as covered. Severity: postconditions = Critical; preconditions/invariants = Major.
**Acceptance:** Uncovered clauses flagged with the documented severity; gap-register justification length validated.

#### BC-AUDIT-272 — consistency-validation: NFR-to-Story severity by priority tier (Rule 27)

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 201-205
**Trigger:** Rule 27 execution
**Behavior:** Every NFR-NNN must be referenced by at least one story. Severity by priority: P0 = Critical, P1 = Major, P2 = Minor. Greps story files and dependency-graph.md.
**Acceptance:** Uncovered NFRs flagged with severity matching priority tier.

---

### convergence-check (4 BCs)

#### BC-AUDIT-273 — convergence-check: skill identity (Phase 7, 7-dimension validation)

**Skill:** `plugins/vsdd-factory/skills/convergence-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-7
**Trigger:** Phase 7 / pre-release
**Behavior:** Runs 7-dimension convergence validation — spec, tests, implementation, verification, visual, performance, documentation. Determines release readiness. `disable-model-invocation: true`; allowed-tools: Read, Write, Bash, Glob, Grep.
**Acceptance:** Frontmatter declares disable-model-invocation true; output matches the 7-dimension verdict format.

#### BC-AUDIT-274 — convergence-check: iron law all-7-must-pass

**Skill:** `plugins/vsdd-factory/skills/convergence-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 13-30
**Trigger:** Each verdict
**Behavior:** "Six out of seven is good enough" is NOT convergence. A skipped, mocked, or stale dimension counts as NOT_CONVERGED. Human can override with documented rationale; orchestrator cannot. 90% threshold = exact (89% fails — no rounding). "Mostly up to date" = NOT_CONVERGED.
**Acceptance:** Overall CONVERGED only when all 7 dimensions are CONVERGED; partial passes do not roll up.

#### BC-AUDIT-275 — convergence-check: per-dimension pass criteria

**Skill:** `plugins/vsdd-factory/skills/convergence-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 39-85
**Trigger:** Each dimension assessed
**Behavior:** D1 Spec: latest adversary novelty LOW + findings addressed. D2 Test: mutation kill ≥90%, coverage ≥85%, runs `cargo test --release`. D3 Implementation: tests green, no spec drift, no `todo!()/unimplemented!()/FIXME`, clean lint. D4 Verification: all Kani proofs pass, no fuzz crashes, purity intact. D5 Visual: demo recordings for all stories, design system compliance. D6 Performance: budgets met, no regressions. D7 Docs: CLAUDE.md current, API docs generated, README accurate.
**Acceptance:** Each dimension's verdict cites the exact pass criterion documented.

#### BC-AUDIT-276 — convergence-check: writes report at .factory/cycles/<current>/convergence-report.md

**Skill:** `plugins/vsdd-factory/skills/convergence-check/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 87-110
**Trigger:** After all 7 dimensions assessed
**Behavior:** Writes Convergence Report with Summary table (7 rows × {Dimension, Status, Notes}), Overall verdict, and Remaining Items list (what needs to happen before convergence) when NOT_CONVERGED.
**Acceptance:** Report exists at the path with all 7 rows and Overall verdict.

---

### convergence-tracking (4 BCs)

#### BC-AUDIT-277 — convergence-tracking: skill identity (quantitative metrics-driven)

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Convergence assessment
**Behavior:** Computes quantitative convergence metrics across adversarial passes, mutation testing runs, formal verification results. Produces CONVERGED/NOT_CONVERGED with supporting metrics. Reads adversarial review reports, mutation results, Kani results, fuzz results, Semgrep results, module criticality, cost log.
**Acceptance:** Skill produces objective numerical assessment, not narrative judgment.

#### BC-AUDIT-278 — convergence-tracking: spec convergence formula (Novelty < 0.15 + median severity decay)

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 40-49
**Trigger:** Dimension 1 computation
**Behavior:** Computes Novelty = N / (N + D) where N = new findings, D = duplicates. CONVERGED if Novelty < 0.15 for 2+ consecutive passes AND median severity < 2.0 for 3+ passes of strict decrease.
**Acceptance:** D1 verdict matches the 2 + 3 pass formula precisely.

#### BC-AUDIT-279 — convergence-tracking: tier-based mutation kill rate thresholds

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 51-60
**Trigger:** Dimension 2 computation
**Behavior:** Per-module kill rate excludes equivalent mutants and compares against tier thresholds: CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%. Survivors classified equivalent/dead-code/insufficient-assertions/complex-logic. CONVERGED requires all modules meet tier target AND survivors >80% equivalent/dead AND all invariants have property tests.
**Acceptance:** Each module's verdict cites its tier target and actual kill rate.

#### BC-AUDIT-280 — convergence-tracking: convergence index formula (CI(i))

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 62-73
**Trigger:** Dimension 3 (Implementation) computation
**Behavior:** CI(i) = (Novelty(i) * (1 - AvgSimilarity) * (6 - MedianSeverity)) / Cost(i). CONVERGED if verification rate < 60% OR (projected findings < 0.5 AND CI < 0.3 declining for 3+ iterations).
**Acceptance:** CI value is computed via the documented formula and threshold check is applied as documented.

---

### create-architecture (4 BCs)

#### BC-AUDIT-281 — create-architecture: skill identity + iron law (verification feasibility)

**Skill:** `plugins/vsdd-factory/skills/create-architecture/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Phase 1 architecture
**Behavior:** Creates sharded architecture documents from PRD and behavioral contracts. Iron law: NO architecture without verification feasibility assessment. Every decision must include feasibility (provable vs testable-only), purity boundaries, VP assignments. `disable-model-invocation: true`.
**Acceptance:** Architecture decisions include verification feasibility section AND VP-NNN files exist.

#### BC-AUDIT-282 — create-architecture: ADR style for every decision

**Skill:** `plugins/vsdd-factory/skills/create-architecture/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 88-103
**Trigger:** Every architectural decision
**Behavior:** Each decision uses ADR format with explicit Status, Context, Options considered (numbered with pros/cons), Decision, Rationale, Consequences. ADR rationale must be explicit (future readers don't have your context).
**Acceptance:** Each ADR contains all 6 fields; "options considered" lists ≥2 alternatives.

#### BC-AUDIT-283 — create-architecture: sharded output (ARCH-INDEX + section files)

**Skill:** `plugins/vsdd-factory/skills/create-architecture/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 73-87, 132-138
**Trigger:** Output writing
**Behavior:** Writes sharded files to `.factory/specs/architecture/` with ARCH-00-overview always, ARCH-01-core-services always, ARCH-02-data-layer if persistent data, ARCH-03-api-layer if API surface, ARCH-04-agent-system if multi-agent, ARCH-05-workflow-engine if workflows, ARCH-06-integration if integrations. Single monolith forbidden.
**Acceptance:** ARCH-INDEX.md exists referencing all section files; no single monolithic architecture.md exists.

#### BC-AUDIT-284 — create-architecture: VP files written to verification-properties/

**Skill:** `plugins/vsdd-factory/skills/create-architecture/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 124-138
**Trigger:** Step 6 (Define Verification Properties)
**Behavior:** Creates VP-NNN files in `.factory/specs/verification-properties/` covering invariants, safety properties (bad things must not happen), liveness properties (good things eventually happen). Plus VP-INDEX.md.
**Acceptance:** VP files exist with safety+liveness sections; VP-INDEX.md links them.

---

### create-brief (4 BCs)

#### BC-AUDIT-285 — create-brief: skill identity + hard gate

**Skill:** `plugins/vsdd-factory/skills/create-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** L1 brief creation
**Behavior:** Creates product brief through guided discovery. Hard gate: do NOT skip to PRD or architecture; every discovery section MUST be explored with the human; do not auto-fill from assumptions. `disable-model-invocation: true`; allowed-tools include AskUserQuestion.
**Acceptance:** Brief covers all 6 discovery areas (Vision, Users, Value Proposition, Success Criteria, Constraints, Prior Art); no auto-filled sections.

#### BC-AUDIT-286 — create-brief: factory-health prerequisite + research check

**Skill:** `plugins/vsdd-factory/skills/create-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-26
**Trigger:** Before discovery starts
**Behavior:** Run `/factory-health`; if existing `.factory/specs/product-brief.md` exists ask "update vs start fresh"; check `.factory/specs/research/RESEARCH-INDEX.md` for prior domain research and read relevant reports — do NOT ask questions research already answers.
**Acceptance:** Factory-health invoked first; existing brief triggers update-vs-fresh prompt; research reports consulted before redundant questions.

#### BC-AUDIT-287 — create-brief: questions one-at-a-time, multiple choice when possible

**Skill:** `plugins/vsdd-factory/skills/create-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 28-30
**Trigger:** Discovery process
**Behavior:** Asks questions ONE at a time (not batched). Uses multiple-choice format when possible to reduce response friction.
**Acceptance:** Each question is a separate user prompt; multiple-choice options surfaced where applicable.

#### BC-AUDIT-288 — create-brief: writes product-brief.md with 8 named sections + state-update

**Skill:** `plugins/vsdd-factory/skills/create-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 70-123
**Trigger:** End of discovery
**Behavior:** Writes `.factory/specs/product-brief.md` with sections: Problem Statement, Target Users, Value Proposition, Success Criteria, Scope (In/Out), Constraints, Prior Art & References, Open Questions. Commits to factory-artifacts. Invokes state-update to set phase=phase-1, status=in-progress.
**Acceptance:** All 8 sections present; commit created on factory-artifacts; STATE.md transitioned to phase-1.

---

### create-domain-spec (3 BCs)

#### BC-AUDIT-289 — create-domain-spec: skill identity (sharded L2 spec)

**Skill:** `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** L2 domain modeling between brief and PRD
**Behavior:** Builds sharded L2 domain specification — entities, relationships, processes, invariants, ubiquitous language. Bridges brief and PRD. `disable-model-invocation: true`; uses AskUserQuestion.
**Acceptance:** Output is sharded into index + section files (not monolithic).

#### BC-AUDIT-290 — create-domain-spec: 3-pass extraction (structural + behavioral + context)

**Skill:** `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 26-83
**Trigger:** Discovery process
**Behavior:** Pass 1 (Structural): nouns — entities, relationships, value objects, ubiquitous language. Pass 2 (Behavioral): verbs — processes, domain events, business rules/invariants, state machines. Pass 3 (Context): bounded contexts. Brownfield enhancement: read `.factory/semport/` Pass 2/3 if present and validate with user rather than ask from scratch.
**Acceptance:** All 3 passes covered with sub-pass deliverables; brownfield path checks semport.

#### BC-AUDIT-291 — create-domain-spec: sharded output structure (5 named files)

**Skill:** `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 92-105
**Trigger:** Output writing
**Behavior:** Writes `.factory/specs/domain-spec/` with L2-INDEX.md + capabilities.md + entities.md + invariants.md + bounded-contexts.md + ubiquitous-language.md. Sections can be added/removed per domain — these are common, not fixed.
**Acceptance:** L2-INDEX.md exists referencing all section files.

---

### create-excalidraw (3 BCs)

#### BC-AUDIT-292 — create-excalidraw: skill identity (programmatic .excalidraw JSON generation)

**Skill:** `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-11
**Trigger:** Architecture diagrams, entity relationships, flow charts as `.excalidraw`
**Behavior:** Generates `.excalidraw` JSON files programmatically. Each file is valid excalidraw document openable in excalidraw.com, VS Code excalidraw extension, or visual companion browser. Output directory: `.factory/diagrams/<name>.excalidraw`.
**Acceptance:** File at `.factory/diagrams/<name>.excalidraw` is valid JSON with `type: excalidraw`, `version: 2`, `source: vsdd-factory`.

#### BC-AUDIT-293 — create-excalidraw: deterministic IDs (not random UUIDs)

**Skill:** `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 173-176
**Trigger:** Element ID assignment
**Behavior:** Uses deterministic IDs based on element purpose (e.g., `api-gateway-rect`, `db-to-api-arrow`). Avoids random UUIDs because deterministic IDs produce readable diffs and predictable updates.
**Acceptance:** Element IDs follow purpose-based naming pattern; no random-looking UUID strings.

#### BC-AUDIT-294 — create-excalidraw: arrow points property required (workaround for export bug)

**Skill:** `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 102-130
**Trigger:** Arrow element generation
**Behavior:** Each arrow element MUST include a `points` array (e.g., `[[0, 0], [100, 0]]`). When using `startBinding`/`endBinding`, also add the arrow to each rectangle's `boundElements` array as `{ "type": "arrow", "id": "arrow-id" }`. Excalidraw shows error modal blocking export if points missing.
**Acceptance:** Generated arrows always have non-empty `points` array; bindings round-trip via boundElements.

---

### create-prd (4 BCs)

#### BC-AUDIT-295 — create-prd: skill identity + hard gate

**Skill:** `plugins/vsdd-factory/skills/create-prd/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** PRD creation from brief
**Behavior:** Transforms product brief (and optional domain spec) into comprehensive PRD with behavioral contracts. Hard gate: do NOT skip to architecture or stories; every BC MUST be defined with testable preconditions/postconditions before proceeding. `disable-model-invocation: true`.
**Acceptance:** No architecture or story files created before this skill's BCs are defined.

#### BC-AUDIT-296 — create-prd: each BC must be testable, unambiguous, complete

**Skill:** `plugins/vsdd-factory/skills/create-prd/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 52-59
**Trigger:** BC writing
**Behavior:** Each behavioral contract (BC-S.SS.NNN) MUST be testable (a test can verify it), unambiguous (single interpretation), complete (preconditions + postconditions + error cases defined).
**Acceptance:** Each BC file has all 3 properties verifiable by inspection.

#### BC-AUDIT-297 — create-prd: 3 named PRD supplements

**Skill:** `plugins/vsdd-factory/skills/create-prd/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-84
**Trigger:** PRD generation
**Behavior:** Creates 3 supplements: `prd-supplements/error-taxonomy.md` (codes, categories, severity, recovery, user vs internal messages); `prd-supplements/interface-definitions.md` (CLI, API endpoints, library exports, I/O formats, types); `prd-supplements/module-criticality.md` (CRITICAL/HIGH/MEDIUM/LOW). Criticality determines review depth, coverage, holdout density.
**Acceptance:** All 3 supplement files exist at the documented paths.

#### BC-AUDIT-298 — create-prd: BC reference repos integration (Source line in BC traceability)

**Skill:** `plugins/vsdd-factory/skills/create-prd/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 33-38
**Trigger:** When `.factory/reference-manifest.yaml` exists
**Behavior:** When BC traces to ingested repo behavior, BC's Traceability section MUST include `Source: <project>/<file>:<function>`. Use `.reference/<project>/` to verify behavioral claims when semport summary is ambiguous.
**Acceptance:** BCs derived from reference repos cite source path:function in traceability.

---

### create-story (3 BCs)

#### BC-AUDIT-299 — create-story: skill identity + hard gate

**Skill:** `plugins/vsdd-factory/skills/create-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** User invokes `/create-story STORY-NNN`
**Behavior:** Creates or refines a single story spec with full ACs, tasks, implementation details. Hard gate: do NOT skip to implementation or deliver before fully elaborated. Every mandatory section MUST be completed — no stub stories. `disable-model-invocation: true`.
**Acceptance:** Story file has every mandatory template section filled with non-stub content.

#### BC-AUDIT-300 — create-story: 7 plan-failure patterns block proceeding

**Skill:** `plugins/vsdd-factory/skills/create-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 84-94
**Trigger:** Self-review
**Behavior:** Skill MUST fix before proceeding if any of: "TBD"/"TODO"/"implement later" in any section; "Add appropriate error handling" without specifying errors; "Write tests for the above" without test descriptions; "Similar to STORY-NNN" without repeating details; ACs without testable assertions; file list saying "and other files as needed"; tasks describing what without how.
**Acceptance:** None of the 7 patterns appear in the produced story file.

#### BC-AUDIT-301 — create-story: forbidden dependencies + version pin enforcement

**Skill:** `plugins/vsdd-factory/skills/create-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 100-112
**Trigger:** Story finalization
**Behavior:** Story MUST include verbatim external dependency table from `dependency-graph.md` in "Library & Framework Requirements". MUST include "Forbidden Dependencies" section listing crates/packages that must NOT appear in module's dependency graph (compile-time enforcement). Error codes MUST come from `prd-supplements/error-taxonomy.md` (new codes flagged "NEW — add E-xxx-NNN to taxonomy").
**Acceptance:** Story has Library Requirements with verbatim version pins, Forbidden Dependencies section, and error codes from existing taxonomy or flagged NEW.

---

### customer-feedback-ingestion (3 BCs)

#### BC-AUDIT-302 — customer-feedback-ingestion: skill identity (read-only, no customer interaction)

**Skill:** `plugins/vsdd-factory/skills/customer-feedback-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-23
**Trigger:** Scheduled (daily), manual, or event-driven (high-volume spikes)
**Behavior:** Ingests customer feedback from configured channels (GitHub Issues/Discussions, Slack/Discord, App Reviews, G2/Capterra, Support, NPS). Categorizes, deduplicates, produces digest. Read-only — factory does NOT interact with customers directly.
**Acceptance:** No outbound message-to-customer tool calls; output limited to digest + state file.

#### BC-AUDIT-303 — customer-feedback-ingestion: 5 categorization buckets with priority

**Skill:** `plugins/vsdd-factory/skills/customer-feedback-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 150-161
**Trigger:** Per ingested item
**Behavior:** Assigns each item to one of 5 categories with priority: Feature Request (HIGH), Bug Report (HIGH), Pain Point (MEDIUM), Praise (LOW), Question (MEDIUM). Categorization uses business-analyst judgment, not just keyword matching — context and tone matter.
**Acceptance:** Each item carries one of the 5 categories and matching priority; rationale present.

#### BC-AUDIT-304 — customer-feedback-ingestion: deduplication via 0.80 semantic similarity threshold

**Skill:** `plugins/vsdd-factory/skills/customer-feedback-ingestion/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 165-173
**Trigger:** Step 5 (Deduplicate)
**Behavior:** Groups items by category; within each, computes semantic similarity; clusters items with >0.80 similarity into single entry with count (frequency signal); preserves most detailed/articulate item as cluster representative.
**Acceptance:** Cluster size ≥2 means similarity computation succeeded; no inflated raw counts in digest.

---

### deactivate (3 BCs)

#### BC-AUDIT-305 — deactivate: skill identity (inverse of activate)

**Skill:** `plugins/vsdd-factory/skills/deactivate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** User invokes `/vsdd-factory:deactivate`
**Behavior:** Reverses activate. Clears orchestrator default-agent override in `.claude/settings.local.json`, removes v1.0 platform activation block, deletes per-machine `hooks/hooks.json` (the per-platform variants remain). Plugin itself stays enabled.
**Acceptance:** After deactivation, `.claude/settings.local.json` lacks `agent` and `vsdd-factory` keys; hooks.json removed but `hooks.json.<platform>` files remain.

#### BC-AUDIT-306 — deactivate: sanity-check before clobbering

**Skill:** `plugins/vsdd-factory/skills/deactivate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-23
**Trigger:** Step 1-2
**Behavior:** If settings.local.json doesn't exist → say so and stop. If `agent` value does NOT point at `vsdd-factory:` agent → STOP and warn — do not clobber unrelated config.
**Acceptance:** Skill never deletes `agent` keys for non-vsdd-factory agents.

#### BC-AUDIT-307 — deactivate: empty-file disposition asks user

**Skill:** `plugins/vsdd-factory/skills/deactivate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 25-28
**Trigger:** After `del(.agent) | del(.["vsdd-factory"])` produces empty object
**Behavior:** Skill asks user whether to delete the file or leave it as `{}` — does not silently choose.
**Acceptance:** User prompt occurs when post-deletion JSON would be empty.

---

### decompose-stories (4 BCs)

#### BC-AUDIT-308 — decompose-stories: skill identity + iron law (BC traceability)

**Skill:** `plugins/vsdd-factory/skills/decompose-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Phase 2 / story decomposition
**Behavior:** Decomposes PRD and architecture into epics, stories, dependency graph, wave schedule. Iron law: every story must trace to ≥1 BC; every AC must trace to a BC precondition or postcondition. Untraced stories drift from spec. `disable-model-invocation: true`.
**Acceptance:** Every story file references at least one BC-S.SS.NNN; every AC traces to a clause.

#### BC-AUDIT-309 — decompose-stories: 13-point story size limit (must split before implementation)

**Skill:** `plugins/vsdd-factory/skills/decompose-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 17-22
**Trigger:** Story sizing
**Behavior:** Stories over 13 points MUST be split NOW (not during implementation). Implementation is too late.
**Acceptance:** No story exceeds 13 points in the produced set.

#### BC-AUDIT-310 — decompose-stories: dependency graph acyclicity verified programmatically

**Skill:** `plugins/vsdd-factory/skills/decompose-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 79-103
**Trigger:** Step 3 (Build Dependency Graph)
**Behavior:** Analyzes story dependencies; verifies acyclicity programmatically (not by inspection). Circular dependencies deadlock wave scheduling.
**Acceptance:** Topological sort succeeds; no cycle reported.

#### BC-AUDIT-311 — decompose-stories: 5 named output artifacts + holdout scenarios

**Skill:** `plugins/vsdd-factory/skills/decompose-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 71-152
**Trigger:** Output writing
**Behavior:** Writes `.factory/stories/epics.md`, per-story files `.factory/stories/STORY-NNN.md` with wave field, `.factory/stories/dependency-graph.md`, `.factory/cycles/<current>/wave-schedule.md`, `.factory/stories/STORY-INDEX.md`, `.factory/stories/sprint-state.yaml`, plus holdout scenarios in `.factory/holdout-scenarios/wave-scenarios/` and `HS-INDEX.md`.
**Acceptance:** All listed artifacts exist post-decomposition.

---

### deliver-story (5 BCs)

#### BC-AUDIT-312 — deliver-story: skill identity (dispatcher, not implementer)

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** User invokes `/deliver-story STORY-NNN`
**Behavior:** Skill is a DISPATCHER — does not write code, write tests, create worktrees, or open PRs directly. Reads canonical workflow from `agents/orchestrator/per-story-delivery.md` and delegates each step to a fresh specialist subagent. Single-context delivery is a correctness bug. `allowed-tools: Read, Bash, Glob, Grep, AskUserQuestion, Task`.
**Acceptance:** Skill's allowed-tools does NOT include Write/Edit; uses Task tool to dispatch specialists.

#### BC-AUDIT-313 — deliver-story: 9-step dispatch sequence with exit conditions

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 52-121
**Trigger:** Skill invocation after preconditions pass
**Behavior:** 9 sequential steps each with named subagent + exit condition: 1) devops-engineer create worktree → `git worktree list` shows it; 2) test-writer stubs → `cargo check` passes; 3) test-writer failing tests → Red Gate verified; 4) implementer TDD → tests green + clippy + fmt + zero todo!()/unimplemented!(); 5) demo-recorder → evidence per AC; 6) implementer push → remote SHA visible; 7) pr-manager 9-step PR lifecycle; 8) devops-engineer cleanup; 9) state update on factory-artifacts.
**Acceptance:** Each step's exit condition independently verified before advancing; skill never skips a step.

#### BC-AUDIT-314 — deliver-story: Red Gate verification in step 3 (mandatory)

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 70-81
**Trigger:** After test-writer (test writer mode) returns
**Behavior:** Orchestrator independently runs `cd .worktrees/STORY-NNN && cargo test`; verifies tests compile, all new tests fail, fail with assertion errors not build errors, failure messages reference behavior under test (not "not yet implemented"). Records outcome in `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`. Implementation MUST NOT proceed until Red Gate is correctly red.
**Acceptance:** Red-gate-log.md exists post-step-3; test failure mode is "assertion" not "build error".

#### BC-AUDIT-315 — deliver-story: verification discipline — never trust agent reports

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 138-148
**Trigger:** After every specialist dispatch
**Behavior:** Orchestrator runs verification command itself, reads FULL output (not just summary), compares against expected exit condition, and only then proceeds. Agent's "all tests pass" is a CLAIM not EVIDENCE. If verification reveals report was inaccurate, dispatch a NEW agent to fix — do not trust subsequent claims from same session.
**Acceptance:** Orchestrator's tool-call sequence shows verification step between every two dispatches.

#### BC-AUDIT-316 — deliver-story: context discipline mapping per specialist

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 122-136
**Trigger:** Each Task dispatch
**Behavior:** Pass only the minimum context to each specialist per a fixed mapping: devops-engineer → worktree protocol; test-writer (stubs) → story+dep-graph+api-surface+BCs; test-writer (tests) → story+api-surface+test-vectors+BCs; implementer → story+module-decomp+dep-graph+api-surface+BCs; demo-recorder → story+AC extract only; pr-manager → story ID + branch name + template path. If story ≥60% of agent context window, STOP and dispatch story-writer to split.
**Acceptance:** Each dispatch payload matches the documented per-specialist mapping; no whole-story-file passes to demo-recorder/pr-manager.

---

### demo-recording (4 BCs)

#### BC-AUDIT-317 — demo-recording: skill identity (CLI/web/API/library)

**Skill:** `plugins/vsdd-factory/skills/demo-recording/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** After implementation+tests pass; after holdout eval; after convergence; on demand
**Behavior:** Records visual demonstrations: CLI via VHS, web via Playwright, API via cURL, library via VHS test output. Generates demo scripts from acceptance criteria. Produces optimized WebM/GIF.
**Acceptance:** Skill picks correct tool per detection signal table.

#### BC-AUDIT-318 — demo-recording: 5 detection signals → demo type → tool

**Skill:** `plugins/vsdd-factory/skills/demo-recording/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-39
**Trigger:** Step 1 (Detect Demo Type)
**Behavior:** Reads architecture and selects demo type per signals: clap/argparse/commander → CLI/VHS; HTTP routes/REST/GraphQL → Web API/cURL sequence; React/Next.js/Vue/Angular/HTML → Web UI/Playwright; library crate/npm/pip → Library/VHS test output; multiple → Composite/VHS+Playwright. Persists to `.factory/demo-state.yaml`.
**Acceptance:** demo-state.yaml has demo_type and recording_tool fields matching the signal mapping.

#### BC-AUDIT-319 — demo-recording: target sizes (WebM <2MB, GIF <5MB, total <25MB)

**Skill:** `plugins/vsdd-factory/skills/demo-recording/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 188-193
**Trigger:** ffmpeg post-processing
**Behavior:** Per-demo target sizes: WebM <2MB per 30s; GIF <5MB (GitHub inline limit); Total per PR <25MB (GitHub attachment limit).
**Acceptance:** No demo file exceeds the limits; total per-PR <25MB.

#### BC-AUDIT-320 — demo-recording: every AC has user-observable behavior covered + visual review

**Skill:** `plugins/vsdd-factory/skills/demo-recording/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 266-298
**Trigger:** Quality gate
**Behavior:** Every AC with user-observable behavior has a recording. Recordings under 2MB/5MB. Full user journey demo for happy path. Evidence report links all recordings. PR description snippet generated. ffmpeg post-processed (optimized, trimmed, labeled). Visual reviewer (DF-018) analyzes all recordings and writes findings to `.factory/demo-evidence/visual-review.md`.
**Acceptance:** All 7 quality-gate items pass; visual-review.md exists.

---

### design-drift-detection (3 BCs)

#### BC-AUDIT-321 — design-drift-detection: skill identity (Sweep 10, UI products only)

**Skill:** `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-22
**Trigger:** Weekly during maintenance sweep, condition `state.has_ui == true`
**Behavior:** Detects design system drift: token overrides (hardcoded values), component misuse (invalid prop combos), pattern violations, emergent patterns (new clusters), style inconsistency. Sweep 10 alongside existing 9 sweeps in maintenance.lobster. Primary agent: ux-designer; supporting: consistency-validator.
**Acceptance:** Frontmatter declares condition `state.has_ui == true`; produces design-drift.md.

#### BC-AUDIT-322 — design-drift-detection: emergent pattern threshold (>=3 instances → propose)

**Skill:** `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-55
**Trigger:** Step 4 (New Pattern Detection — D15)
**Behavior:** Clusters similar DOM tree shapes across codebase. If cluster size >= 3 instances: propose as design-system component (name, props interface, variants, states). Present to human for approval. If approved: add to design system + create refactoring stories.
**Acceptance:** Threshold check is exactly ≥3; below threshold not surfaced as proposal.

#### BC-AUDIT-323 — design-drift-detection: graceful skip when no design system

**Skill:** `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 110-115
**Trigger:** Various missing-input failures
**Behavior:** No `.factory/design-system/` → "no design system detected", skip all checks, exit cleanly. No CSS/UI components → "no UI artifacts found", skip scan. Missing pattern definition (e.g., form-patterns.yaml) → skip that pattern check, note which not validated.
**Acceptance:** Each failure mode produces clean exit with named reason.

---

### design-system-bootstrap (3 BCs)

#### BC-AUDIT-324 — design-system-bootstrap: skill identity (greenfield + brownfield + feature)

**Skill:** `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-26
**Trigger:** Greenfield Phase 1 (after spec crystallization, before story decomp); Brownfield Phase 0 (during ingestion); Feature F2 (only if no design system); condition `feature_type in ['ui', 'full-stack']`
**Behavior:** Bootstraps design system. Greenfield: from product brief + brand guidelines. Brownfield: extracts from existing codebase. Produces `.factory/design-system/` with tokens, components, patterns, constraints. Human review gate before downstream use. Primary: ux-designer; supporting: architect, codebase-analyzer, accessibility-auditor.
**Acceptance:** `.factory/design-system/` directory exists with the 4-section structure (tokens/components/patterns/constraints.yaml).

#### BC-AUDIT-325 — design-system-bootstrap: minimal bootstrap fallback when no brand guidelines

**Skill:** `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 90-100
**Trigger:** No brand guidelines provided
**Behavior:** Creates minimal design system: neutral gray palette + single blue primary; system font stack; standard 4px/8px grid; full template registry; full template patterns. Human can override any token after bootstrap.
**Acceptance:** Tokens populated with documented neutral defaults; human-override path documented.

#### BC-AUDIT-326 — design-system-bootstrap: WCAG AA contrast validation (accessibility-auditor)

**Skill:** `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 43-49, 136-140
**Trigger:** Greenfield step 3 (and brownfield equivalent)
**Behavior:** accessibility-auditor verifies all color combinations meet WCAG AA contrast ratios; touch target minimums; focus styles for all interactive states; reduced-motion overrides present. Quality gate requires WCAG AA validation completed.
**Acceptance:** Color combinations have validated contrast ratios; failures flagged.

---

### discovery-engine (4 BCs)

#### BC-AUDIT-327 — discovery-engine: skill identity (autonomous opportunity research)

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-13
**Trigger:** Scheduled (weekly features, bi-weekly products), manual, or event-driven
**Behavior:** Continuously researches opportunities for new features (existing products) and net-new product concepts. Evaluates ideas against structured criteria. Facilitates planning document creation. Routes approved ideas to development pipeline. Orchestrator delegates to specialist agents (research-agent primarily) — does not execute steps directly.
**Acceptance:** Outputs feature-research, feature-ideas, product-research, product-concepts, briefs, and discovery-report files.

#### BC-AUDIT-328 — discovery-engine: 2 modes (Feature Discovery vs Product Discovery)

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-209
**Trigger:** Mode selection per request
**Behavior:** Mode 1 Feature Discovery (per existing product): load product context → research landscape (decomposition-based across 4 segments) → generate feature ideas → evaluate+rank → facilitate brief. Mode 2 Product Discovery: load discovery context → research opportunities (4 segments) → generate concepts → evaluate+rank → facilitate brief.
**Acceptance:** Mode 1 outputs feature-research-[product]-DATE.md; Mode 2 outputs product-research-DATE.md.

#### BC-AUDIT-329 — discovery-engine: 7-dimension scoring with weights summing to 1.00

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 226-258
**Trigger:** Idea evaluation
**Behavior:** Scores ideas on 7 dimensions: Value (0.25), Feasibility (0.15), Alignment (0.15), Novelty (0.10), Time-Criticality (0.10), Effort (0.10), Evidence Strength (0.15). Weights sum to 1.00. Evidence Strength rubric: 0.0-0.2 speculation; 0.3-0.5 market research only; 0.5-0.6 + one customer signal; 0.6-0.8 multiple signals; 0.8-0.9 all sources; 0.9-1.0 + revenue impact.
**Acceptance:** Each idea has all 7 dimension scores and a composite from weighted average.

#### BC-AUDIT-330 — discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent)

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 263-289
**Trigger:** After scoring
**Behavior:** Auto-brief generation: composite ≥0.7 AND evidence_strength ≥0.6 (human approves). Product Backlog: 0.5-0.7 OR evidence 0.4-0.6 (resurface next run). Discovery Registry: <0.5 AND evidence <0.4 (re-evaluate next run). Urgent Action: HIGH competitive urgency AND evidence_strength ≥0.7 → immediate human notification.
**Acceptance:** Each idea routes to exactly one bucket per the threshold rules; auto-brief requires evidence ≥0.6 (not just composite).

---

### disposition-pass (5 BCs)

#### BC-AUDIT-331 — disposition-pass: skill identity (Pass 9, vision-lens re-examination)

**Skill:** `plugins/vsdd-factory/skills/disposition-pass/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** After vision doc exists AND brownfield-ingest Phase C complete; or after material vision change; before `/create-prd` or `/decompose-stories`
**Behavior:** Re-examines ingested reference repos through Corverax vision lens to decide what to Model, Reimplement, Enhance, or Leave Behind. Produces per-repo Pass 9 disposition docs, master rollup, and optionally vision-doc updates. Runs against one repo or all 44.
**Acceptance:** argument-hint matches `[<repo>|--all] [--rollup] [--update-vision]`; outputs `<repo>-pass-9-corverax-disposition.md`.

#### BC-AUDIT-332 — disposition-pass: 4-bucket mandatory categorization

**Skill:** `plugins/vsdd-factory/skills/disposition-pass/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 30-40
**Trigger:** Per-repo categorization
**Behavior:** Every substantive capability sorts into EXACTLY one bucket: Model (adopt as-is), Take but reimplement (right idea, rebuild cleanly), Enhance (extend beyond source), Leave behind (explicitly reject — REASON REQUIRED).
**Acceptance:** No capability appears in 2 buckets; every "Leave behind" entry has a reason.

#### BC-AUDIT-333 — disposition-pass: every disposition tied to named vision section

**Skill:** `plugins/vsdd-factory/skills/disposition-pass/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 64-79
**Trigger:** Each row in disposition tables
**Behavior:** Tie every disposition to a named vision section. No hand-waving. Cite source `file:line` for any new finding beyond existing ingest. Be specific — counts, file paths, version numbers — not "robust" or "scalable". One agent per repo always (combined agents context-exhaust).
**Acceptance:** Each disposition row references a vision section name; new findings cite file:line.

#### BC-AUDIT-334 — disposition-pass: parallelism in batches of 10 with --all

**Skill:** `plugins/vsdd-factory/skills/disposition-pass/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 80-83
**Trigger:** `--all` flag
**Behavior:** Launches in batches of 10 concurrent agents to respect rate limits. Wait for each batch to complete before launching next.
**Acceptance:** Concurrent agent count never exceeds 10; batch boundaries observed.

#### BC-AUDIT-335 — disposition-pass: vision SHA tracked in rollup header for staleness

**Skill:** `plugins/vsdd-factory/skills/disposition-pass/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 99-112, 127-138
**Trigger:** `--rollup` generation
**Behavior:** Rollup header MUST include `generated_against_vision_sha`, `generated_against_vision_path`, `generated_at`. When current vision SHA advances past this value, rollup is stale and `--all --rollup` MUST be re-run. Per-repo staleness: re-ingest triggers single-repo Pass 9 re-run; vision change triggers `--all`.
**Acceptance:** Rollup frontmatter contains all 3 SHA fields; SHA mismatch is detectable.

---

### dtu-creation (3 BCs)

#### BC-AUDIT-336 — dtu-creation: skill identity (build behavioral clones)

**Skill:** `plugins/vsdd-factory/skills/dtu-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Phase 1b (architect identifies third-party dependencies); Phase 3 (before holdout eval); on-demand for new integrations
**Behavior:** Builds behavioral clones of third-party services for Digital Twin Universe. Agents construct clones from API docs, OpenAPI specs, recorded traffic. Packaged as Docker containers for testing/holdout eval. Orchestrator delegates to specialist agents.
**Acceptance:** Outputs include clone spec + clone implementation + Dockerfile + validation report + docker-compose.dtu.yml + dtu-env.sh.

#### BC-AUDIT-337 — dtu-creation: fidelity level driven by SUT usage

**Skill:** `plugins/vsdd-factory/skills/dtu-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 47-56
**Trigger:** Step 2 (Determine Fidelity)
**Behavior:** Read-only integration → L1 (API Shape) sufficient. CRUD operations → L2 (Stateful) required. Complex workflows (OAuth, webhooks) → L3 (Behavioral) required. Reliability-critical (payments) → L4 (Adversarial) required. Architect's Phase 1b classification drives this.
**Acceptance:** Each clone declares fidelity level matching the SUT usage classification.

#### BC-AUDIT-338 — dtu-creation: clone validation via contract tests + Schemathesis

**Skill:** `plugins/vsdd-factory/skills/dtu-creation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 67-76
**Trigger:** Step 4 (Validate Clone)
**Behavior:** test-writer agent writes contract tests verifying clone matches real service's API shape. If OpenAPI spec exists, uses Schemathesis to auto-generate API tests. Stores results in `validation-report.md`.
**Acceptance:** validation-report.md exists per clone with test results; Schemathesis used when spec available.

---

### dtu-validate (3 BCs)

#### BC-AUDIT-339 — dtu-validate: skill identity (independent reimplementation comparison)

**Skill:** `plugins/vsdd-factory/skills/dtu-validate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-22
**Trigger:** After wave merge (before holdout eval); after CRITICAL/HIGH refactor; in convergence check (Phase 6); when mutation testing finds DTU-covered survivors
**Behavior:** Maintains independent, minimal reimplementations of critical subsystems. NOT a copy of the code — written to be obviously correct rather than performant. When clone and real implementation diverge, one has a bug. `disable-model-invocation: true`.
**Acceptance:** Each DTU clone is in `.factory/dtu-clones/<module>/` with clone.rs, harness.rs, README.md.

#### BC-AUDIT-340 — dtu-validate: criticality-driven candidacy

**Skill:** `plugins/vsdd-factory/skills/dtu-validate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 26-31
**Trigger:** Step 1 (Identify DTU Candidates)
**Behavior:** Reads `prd-supplements/module-criticality.md`. CRITICAL modules → DTU clones MANDATORY. HIGH modules → DTU clones recommended. MEDIUM/LOW → no clones (cost > benefit).
**Acceptance:** Every CRITICAL module has a DTU clone; MEDIUM/LOW modules have none.

#### BC-AUDIT-341 — dtu-validate: divergence in CRITICAL = blocking

**Skill:** `plugins/vsdd-factory/skills/dtu-validate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 121-124
**Trigger:** ComparisonResult::Divergence detected on CRITICAL module
**Behavior:** Skill MUST report divergences immediately. DTU divergence in CRITICAL module is BLOCKING — implementation MUST be fixed before proceeding.
**Acceptance:** Pipeline halts on CRITICAL divergence; non-blocking warning for HIGH/MEDIUM.

---

### excalidraw-export (2 BCs — reference-only skill)

#### BC-AUDIT-342 — excalidraw-export: skill identity (reference-only batch PNG export)

**Skill:** `plugins/vsdd-factory/skills/excalidraw-export/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-6
**Trigger:** Reference-only — `disable-model-invocation: true`; not directly invokable as command
**Behavior:** Documents how to batch-render `.excalidraw` wireframe diagrams to pixel-perfect PNG using headless Firefox via Playwright + `excalidraw-brute-export-cli` v0.4.0+. Uses 4 parallel workers, 2x scale, white background. PNGs placed alongside source files. Throughput ~5-8s/diagram, ~4-6 min for 175 files.
**Acceptance:** Frontmatter declares `disable-model-invocation: true`; skill is documentation/reference, not an autonomous workflow.

#### BC-AUDIT-343 — excalidraw-export: arrow points workaround documented (must have `points`)

**Skill:** `plugins/vsdd-factory/skills/excalidraw-export/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 49-52
**Trigger:** Documentation
**Behavior:** Documents that arrow elements MUST have `points` property OR excalidraw.com shows error modal blocking export. Must use v0.4.0+ of excalidraw-brute-export-cli (v0.2.0 times out on current excalidraw.com UI).
**Acceptance:** Documentation cites exact version pin and points-property requirement.

---

### factory-cycles-bootstrap (3 BCs)

#### BC-AUDIT-344 — factory-cycles-bootstrap: skill identity (flat → cycle-keyed migration)

**Skill:** `plugins/vsdd-factory/skills/factory-cycles-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** First adversarial review on project with historical `specs/adversarial-review-pass-*.md`; new convergence cycle; collision-guard warning
**Behavior:** Migrates `.factory/` from flat layout (all reviews in `specs/`) to cycle-keyed layout (organized by convergence cycle). Archives historical reviews under named cycle dir, sets current-cycle pointer.
**Acceptance:** Argument-hint matches `[cycle-name]`; produces cycle directory structure.

#### BC-AUDIT-345 — factory-cycles-bootstrap: archives via `git mv` (preserves history)

**Skill:** `plugins/vsdd-factory/skills/factory-cycles-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 53-62
**Trigger:** Step 4 with historical files
**Behavior:** Moves each file using `git mv` (preserves history): `specs/adversarial-review-pass-1.md` → `cycles/<historical-cycle>/adversarial-reviews/pass-1.md`. Renumbers if naming inconsistent. Creates INDEX.md in historical cycle dir listing all archived passes.
**Acceptance:** Git log shows the move (not delete + add); INDEX.md exists in historical cycle dir.

#### BC-AUDIT-346 — factory-cycles-bootstrap: writes .factory/current-cycle pointer

**Skill:** `plugins/vsdd-factory/skills/factory-cycles-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 65-72
**Trigger:** Step 5
**Behavior:** Writes `.factory/current-cycle` with new cycle name (single line, no trailing newline). This file is read by adversarial-review skill to determine output path. Cycle names lowercase, hyphenated, no spaces.
**Acceptance:** `.factory/current-cycle` exists with the cycle name and no trailing newline; cycle name passes the format validation.

---

### factory-dashboard (3 BCs)

#### BC-AUDIT-347 — factory-dashboard: skill identity (read-only diagnostic)

**Skill:** `plugins/vsdd-factory/skills/factory-dashboard/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Session start, after long break, before spawning worker, when diagnosing "why X fired"
**Behavior:** Renders single-page markdown dashboard combining STATE.md frontmatter, wave-state.yaml, and observability event log. Read-only — never modifies files, never runs hooks/triggers actions, never queries external services. Distinct from factory-health (which validates worktree).
**Acceptance:** No write/exec tool calls; output is markdown summary only.

#### BC-AUDIT-348 — factory-dashboard: missing files produce "not initialized" notices, not errors

**Skill:** `plugins/vsdd-factory/skills/factory-dashboard/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-36
**Trigger:** STATE.md / wave-state.yaml / events-*.jsonl missing
**Behavior:** All three sources are optional. Missing files produce clean "not initialized" notices rather than errors.
**Acceptance:** Skill exits 0 with informational dashboard even when none of the three sources exist.

#### BC-AUDIT-349 — factory-dashboard: --factory PATH and --days N options

**Skill:** `plugins/vsdd-factory/skills/factory-dashboard/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 37-46
**Trigger:** Argument parsing
**Behavior:** `--factory PATH` uses different `.factory/` location; `--days N` changes event-log lookback (default 7).
**Acceptance:** Both flags accepted; default lookback is 7 days when omitted.

---

### factory-health (3 BCs)

#### BC-AUDIT-350 — factory-health: skill identity (auto-repairing worktree validator)

**Skill:** `plugins/vsdd-factory/skills/factory-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-11
**Trigger:** Session start or when `.factory/` state seems wrong
**Behavior:** Validates `.factory/` worktree health. Auto-repairs common issues. Checks orphan branch existence, worktree mount, STATE.md presence. `disable-model-invocation: true`; allowed-tools: Bash, Read, Write.
**Acceptance:** Output is HEALTHY or REPAIRED with list of repairs.

#### BC-AUDIT-351 — factory-health: 8 sequential checks with auto-repair on missing structures

**Skill:** `plugins/vsdd-factory/skills/factory-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 13-123
**Trigger:** Skill invocation
**Behavior:** 8 checks: 1) factory-artifacts orphan branch (auto-create); 2) worktree mounted (auto-mount); 3) worktree on correct branch (remove+remount if wrong); 4) STATE.md exists (auto-create with initial frontmatter); 5) directory structure intact (15 named dirs created with .gitkeep if missing); 6) reference repos check vs reference-manifest.yaml; 7) sync state (porcelain status); 8) STATE.md health (200/500 line thresholds + content routing antipatterns).
**Acceptance:** Each check has explicit auto-repair OR explicit human-action recommendation; missing dirs auto-created.

#### BC-AUDIT-352 — factory-health: STATE.md size thresholds (200/500 lines)

**Skill:** `plugins/vsdd-factory/skills/factory-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 113-123
**Trigger:** Check 8
**Behavior:** ≤200 lines → Healthy. 201-500 → Warn (recommend `/compact-state`). 501+ → Error (must compact before proceeding). Also checks `## Burst`/`## Pass` count >10, `## Session Resume Checkpoint` >1, `adversary_pass_` fields >5 → recommend compact.
**Acceptance:** Thresholds exact: 200, 500, 10, 1, 5.

---

### factory-obs (4 BCs)

#### BC-AUDIT-353 — factory-obs: skill identity (manage local observability stack)

**Skill:** `plugins/vsdd-factory/skills/factory-obs/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-14
**Trigger:** Start/stop/reset Docker stack; register/unregister/list watched factories; open Grafana dashboards
**Behavior:** Manages 5-service Docker observability stack (OTel Collector + Loki + Prometheus + Grafana + Image Renderer). Ingests `.factory/logs/events-*.jsonl` into Loki and Claude Code OTel into Prometheus. Surfaces 7 preconfigured Grafana dashboards. Opt-in, local-only — no cloud services.
**Acceptance:** No cloud-service fallback; all data stays local.

#### BC-AUDIT-354 — factory-obs: 9-arg subcommand surface

**Skill:** `plugins/vsdd-factory/skills/factory-obs/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 32-50
**Trigger:** Argument parsing
**Behavior:** Accepts: `up` (default — regenerate override + start), `down` (keep volumes), `reset` (wipe volumes), `status` (compose ps), `logs` (tail collector+grafana), `dashboard` (print URL + open browser if interactive), `register [PATH]`, `unregister [PATH]`, `list`/`registered`, `regenerate` (override file only), `help`.
**Acceptance:** Each subcommand executes the documented action.

#### BC-AUDIT-355 — factory-obs: multi-factory watched-list at ~/.config/vsdd-factory/watched-factories

**Skill:** `plugins/vsdd-factory/skills/factory-obs/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 16-26
**Trigger:** v0.78.0+; multiple factory projects
**Behavior:** Stack watches multiple factory projects via user-level registry at `~/.config/vsdd-factory/watched-factories`. `up` generates `docker-compose.override.yml` from registry with one bind mount per factory at `/var/log/factory/<safe-name>/`. Collector globs `/var/log/factory/*/events-*.jsonl`.
**Acceptance:** Multiple registered factories all feed the same Loki without conflict.

#### BC-AUDIT-356 — factory-obs: env override port allowlist

**Skill:** `plugins/vsdd-factory/skills/factory-obs/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 69-87
**Trigger:** Port collision recovery
**Behavior:** Honors VSDD_OBS_GRAFANA_PORT (3000), VSDD_OBS_LOKI_PORT (3100), VSDD_OBS_OTLP_HTTP_PORT (4318), VSDD_OBS_PROMETHEUS_PORT (9090), VSDD_OBS_RENDERER_PORT (8081), VSDD_FACTORY_LOGS (legacy single-path), VSDD_OBS_REGISTRY (test override), VSDD_OBS_OPEN_BROWSER (1=force, 0=suppress, unset=auto-detect TTY).
**Acceptance:** Each env var produces the documented effect when set.

---

### factory-worktree-health (4 BCs)

#### BC-AUDIT-357 — factory-worktree-health: skill identity (blocking precondition)

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-19
**Trigger:** Every pipeline start (greenfield, brownfield, feature, resume)
**Behavior:** Validates factory worktree health on pipeline start. Checks `.factory/` (all modes) and `.factory-project/` (multi-repo only when project.yaml exists). Verifies remote branch existence, local worktree mount, sync state. Auto-repairs if possible. BLOCKING precondition — no pipeline work proceeds until this passes. Executor: devops-engineer (requires `exec` tool).
**Acceptance:** Pipeline does not proceed when this skill reports FAIL.

#### BC-AUDIT-358 — factory-worktree-health: workspace isolation guard (Step 0)

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-66
**Trigger:** Before ANY git commands
**Behavior:** Verifies cwd does NOT contain "dark-factory"; remote URL does NOT contain "dark-factory"; if `.factory/.git` exists, the gitdir path does NOT contain "dark-factory". Any failure → FATAL exit with documented fix. This prevents engine/project mix-ups.
**Acceptance:** All 3 isolation checks pass before pipeline proceeds; failure produces FATAL with actionable fix.

#### BC-AUDIT-359 — factory-worktree-health: 5-step sync state evaluation matrix

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 134-149
**Trigger:** Step 4
**Behavior:** Evaluates LOCAL/REMOTE/BASE. LOCAL==REMOTE → in-sync (proceed). REMOTE==none → fresh-init (proceed). LOCAL!=REMOTE && BASE==REMOTE → local ahead (push). LOCAL!=REMOTE && BASE==LOCAL → local behind (`pull --ff-only`). LOCAL!=REMOTE && BASE!=LOCAL && BASE!=REMOTE → diverged → STOP + report both SHAs (human resolves).
**Acceptance:** Each branch of the 5-row truth table produces the documented action.

#### BC-AUDIT-360 — factory-worktree-health: dual-worktree check for multi-repo mode

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 21-31
**Trigger:** project.yaml exists in repo root
**Behavior:** Checks BOTH `.factory/` (branch `factory-artifacts`, all modes) and `.factory-project/` (branch `factory-project-artifacts`, multi-repo only). Without project.yaml, only `.factory/` checked. Per-worktree variables WORKTREE_DIR and BRANCH_NAME drive the loop.
**Acceptance:** Multi-repo project produces 2 health reports (one per worktree); single-repo produces 1.

---

## 3. Skills NOT covered by additional BCs (none in this batch)

All 40 skills in this batch produced ≥2 BCs. The reference-only `excalidraw-export` skill (disable-model-invocation: true, documentation-only) was given 2 minimal BCs covering identity and the load-bearing arrow-points workaround. No skill was deemed too thin to extract from.

## 4. Cross-skill observations

### CSO-1: Iron Law convention is dominant pattern (8+ skills)

Skills with hard-gate behavior use a verbatim "Iron Law" header + Red Flags table:
- adversarial-review (NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST)
- brownfield-ingest (NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST)
- create-architecture (NO ARCHITECTURE WITHOUT VERIFICATION FEASIBILITY ASSESSMENT)
- decompose-stories (NO STORY WITHOUT BC TRACEABILITY FIRST)
- deliver-story (NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST)
- convergence-check (NO RELEASE WITHOUT ALL SEVEN DIMENSIONS CONVERGED)
- create-prd (Hard Gate — Do NOT skip to architecture or stories)
- brainstorming (Hard Gate — Do NOT skip to brief creation)
- create-brief (Hard Gate — every section MUST be explored)
- create-story (Hard Gate — no stub stories)

This is a strong project-wide convention. Spec must enforce: every gateable skill has an iron law + red flags + verbatim announcement.

### CSO-2: "Announce at Start" verbatim pattern (5 skills)

Skills that dispatch subagents or run autonomously must emit a verbatim opening line:
- adversarial-review ("I'm using the adversarial-review skill to launch...")
- brownfield-ingest ("I'm using the brownfield-ingest skill to run...")
- check-state-health ("I'm using the check-state-health skill to validate...")
- compact-state ("I'm using the compact-state skill to extract...")
- claude-telemetry ("I'm using the claude-telemetry skill to configure...")
- factory-dashboard ("I'm using the factory-dashboard skill to render...")
- factory-obs ("I'm using the factory-obs skill to manage...")
- deliver-story ("I'm using the deliver-story skill to dispatch...")

This is the operator-visibility pattern. Spec should make verbatim announcement a required behavior for all autonomously-dispatched skills.

### CSO-3: Fresh-context discipline pattern (3 skills)

Fresh-context isolation is load-bearing in:
- adversarial-review (information asymmetry; minimum 3 clean passes)
- deliver-story (specialist split; 9-step dispatch with verification between each)
- code-delivery (4th model family for pr-reviewer; review convergence loop)

Spec should call this out as a primary pattern: agents must not see prior pass artifacts that contaminate their reasoning.

### CSO-4: Strict-binary novelty enforcement (brownfield-ingest is canonical)

The "only literal NITPICK token counts as convergence" pattern is documented in brownfield-ingest. The convergence-check skill uses an analogous all-7-dimensions-must-pass enforcement. Spec should generalize this: when convergence is binary, soft phrasings ("borderline", "effectively", "functionally") all mean SUBSTANTIVE / NOT_CONVERGED.

### CSO-5: Sandbox awareness (3 skills cite the bash allowlist)

- brownfield-ingest documents the allowlist + 2 working bash patterns + grep-tool fallback
- disposition-pass documents the same allowlist + "try a different formulation before reporting failure"
- The codebase-analyzer agent file is the canonical reference both skills point at

Spec should pull this out as a cross-cutting concern: any skill that runs subagents in the sandbox must inherit the allowlist documentation.

### CSO-6: Quality Gate checklist convention (15+ skills)

Most behavior-completing skills end with a `## Quality Gate` checklist of `- [ ]` items. The factory has standardized on this format. Spec should make Quality Gate sections mandatory for skills that produce artifacts.

### CSO-7: Output artifact path convention (`.factory/<area>/<filename>`)

All skills write under `.factory/`:
- planning artifacts → `.factory/planning/`
- specs → `.factory/specs/` and `.factory/specs/{behavioral-contracts,verification-properties,architecture,prd-supplements,domain-spec}/`
- cycles → `.factory/cycles/<cycle-name>/{adversarial-reviews,convergence}/`
- stories → `.factory/stories/`
- holdout → `.factory/holdout-scenarios/`
- discovery → `.factory/discovery/`
- semport → `.factory/semport/<project>/`
- dtu-clones → `.factory/dtu-clones/<module>/`
- demo-evidence → `.factory/demo-evidence/` (NOT for code-delivery — that goes to feature branch)
- code-delivery — `.factory/code-delivery/STORY-NNN/`
- maintenance → `.factory/maintenance/`
- design-system → `.factory/design-system/`
- diagrams → `.factory/diagrams/`

The exception is `code-delivery` Step 2 demo evidence which goes to feature branch (`docs/demo-evidence/<STORY-ID>/`) so it appears in PR diffs. Spec must call out this exception.

### CSO-8: Sharded-output convention (4 skills)

Sharded artifacts (INDEX + section files) are mandatory for:
- create-architecture → ARCH-INDEX + ARCH-NN section files
- create-prd → PRD body + per-BC files BC-S.SS.NNN.md + BC-INDEX
- create-domain-spec → L2-INDEX + capability/entity/invariant/context/glossary section files
- artifact-detection / consistency-validation enforce traces_to: <index> on every detail file

Monoliths are forbidden. Index references must match actual file set; orphan and dangling-ref detection is a Rule in consistency-validation.

### CSO-9: Self-review section before adversarial review (3 spec-creation skills)

create-brief / create-prd / create-architecture / create-domain-spec each include a `## Self-Review` section with the same 4-question format: placeholder scan, internal consistency, scope check, ambiguity check. This is a cheap filter to catch obvious gaps before paying tokens for adversary. Spec should standardize self-review as a mandatory step in all artifact-producing skills.

### CSO-10: One-agent-per-X discipline (3 skills)

Repeated emphasis: combined agents context-exhaust:
- brownfield-ingest: "One agent per project — always."
- disposition-pass: "One agent per repo. Always."
- factory-worktree-health: "FACTORY_WORKTREE_HEALTH: PASS reported for each required worktree" (one per dir)

Spec should make this an explicit invariant for any skill that fans out work across multiple targets.

---

## 5. Delta Summary

- **New BCs added this round:** 161 (BC-AUDIT-200..360)
- **Skills covered:** 40 (alphabetical 1..40)
- **Average BCs per skill:** 4.0
- **High-density skills (≥6 BCs):** adversarial-review (7), brownfield-ingest (8), code-delivery (6), deliver-story (5), disposition-pass (5)
- **Reference-only skills (≤2 BCs):** excalidraw-export (2)
- **Confidence distribution:** 100% HIGH (every BC cites direct frontmatter, prose, or quality-gate language; no inferred behaviors required)
- **Cross-skill observations:** 10 patterns identified

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 (Behavioral Contracts) — deep skills batch 1 (skills 1-40 alphabetical) |
| **Novelty score** | SUBSTANTIVE (binary; 161 new per-skill BCs added) |
| **Trajectory** | 86 BCs (broad pass-3) | 143 BCs (deep-r1) | 304 BCs cumulative (this batch) — per-skill density shifted from sample-only to comprehensive on the first 40/119 skills |
| **Verdict** | FINDINGS_REMAIN |

**Trajectory narrative:** Round-1 deep-r1 covered 4-6 SKILL.md files at sample-depth (143 BCs total); this round covers 40 SKILL.md files at per-instance depth (161 new BCs). Findings change the spec model materially — they enumerate per-skill identity, triggers, behaviors, quality gates, and outputs at a density required to rebuild what currently ships per operator clarification. Verdict is FINDINGS_REMAIN because skills 41-119 (alphabetical batches 2 and 3) are not yet covered; the per-skill BC catalog will reach CONVERGENCE_REACHED after those batches complete.

**Justification:** This round materially expands the spec footprint — the prior 143 BCs covered Rust crates and a small handful of skill-level signals (only 4-6 SKILL.md files were spot-checked in deep-r1). With 161 new per-skill BCs across 40 skills, the BC catalog now has the per-instance density the user said was required to rebuild what currently ships. Removing this round would leave a multi-hundred-skill behavioral surface unspecified — that's exactly the gap the operator flagged.

Specific items that change how we'd spec the system:
1. The 40 quality-gate checklists each enumerate verifiable acceptance properties (not all of them previously surfaced — e.g., the 36-rule consistency-validation surface, the 7-dimension convergence-check, the 9-step deliver-story dispatch).
2. 10 cross-skill observations (CSO-1..10) reveal project-wide conventions that should become first-class spec invariants (Iron Law header, Announce-at-Start verbatim, sharded outputs, fresh-context discipline, sandbox allowlist propagation, etc.).
3. Several skills have non-obvious branching behavior (activate platform exit codes, check-input-drift cluster-drift triage, claude-telemetry legacy-key pruning, conform-to-template no-auto-reorder safety, dtu-validate criticality blocking) that prior passes did not capture.

## 7. Convergence Declaration

Another round needed for skills 41-119 (alphabetical batches 2-3). This batch alone covers 40/119 skills — the BC catalog is not yet complete enough to rebuild the full skill surface. Subsequent batches will continue with `feature-mode-scoping-rules` through `wave-gate`.

## 8. State Checkpoint

```yaml
pass: 3
round: deep-skills-batch-1
status: complete
batch_scope: alphabetical-skills-1-to-40
bc_range_used: BC-AUDIT-200..360
bc_range_remaining: BC-AUDIT-361..399 (this batch); BC-AUDIT-400+ (next batches)
files_scanned: 40
timestamp: 2026-04-25T00:00:00Z
novelty: SUBSTANTIVE
next_action: dispatch-batch-2-skills-41-80
```
