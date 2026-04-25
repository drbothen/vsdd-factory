# Pass 3 Deep — Per-Workflow & Per-Step Behavioral Contracts

## Round Metadata

- **Round:** Pass 3 deep convergence — workflow file extraction (companion to GAP-C from `pass-3-behavioral-contracts-deep-r1.md`)
- **Date:** 2026-04-25
- **Author:** codebase-analyzer (Pass 3 deepening, brownfield-ingest)
- **BC range used:** BC-AUDIT-1300 .. BC-AUDIT-1798 (within reserved 1300..1799)
- **Source files (16):**
  - `plugins/vsdd-factory/workflows/brownfield.lobster` (v3.0.0, 26 steps)
  - `plugins/vsdd-factory/workflows/code-delivery.lobster` (v2.0.0, 23 top-level steps + nested loops)
  - `plugins/vsdd-factory/workflows/discovery.lobster` (v2.0.0, 29 steps)
  - `plugins/vsdd-factory/workflows/feature.lobster` (v3.0.0, 82 steps)
  - `plugins/vsdd-factory/workflows/greenfield.lobster` (v2.1.0, 72 steps)
  - `plugins/vsdd-factory/workflows/maintenance.lobster` (v2.0.0, 33 steps)
  - `plugins/vsdd-factory/workflows/multi-repo.lobster` (v3.0.0, 41 steps + nested)
  - `plugins/vsdd-factory/workflows/planning.lobster` (v2.0.0, 25 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster` (v3.0.0, 15 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster` (v2.0.0, 14 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-2-story-decomposition.lobster` (v1.0.0, 14 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster` (v2.0.0, 16 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-4-holdout-evaluation.lobster` (v1.0.0, 3 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-5-adversarial-refinement.lobster` (v1.0.0, 2 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-6-formal-hardening.lobster` (v2.0.0, 9 steps)
  - `plugins/vsdd-factory/workflows/phases/phase-7-convergence.lobster` (v2.0.0, 18 steps)

**Confidence baseline:** All BCs are HIGH because they are derived directly from the workflow YAML. Behavior is "what the workflow declares it does"; implementation correctness (whether the orchestrator honors every declaration) is covered in `pass-3-behavioral-contracts-deep-r1.md` GAP-C.

---

## 1 — Phase 0: Codebase Ingestion

**File:** `plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster`

### Workflow-level BCs

### BC-AUDIT-1300 — phase-0-codebase-ingestion: identity

**Workflow:** `plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster`
**Step:** workflow-level
**Confidence:** HIGH
**Source line(s):** 1-8
**Type:** workflow header
**Behavior:** Declares a brownfield-mode Phase 0 workflow named `phase-0-codebase-ingestion` (v3.0.0) that analyses an existing codebase via the broad-then-converge protocol — source acquisition, broad sweep (7 passes), convergence deepening, coverage audit, extraction validation, final synthesis — with state-manager commits between every step for crash recovery.
**Acceptance:** A loader that reads this workflow exposes `name=phase-0-codebase-ingestion`, `version=3.0.0`, and the description string.

### BC-AUDIT-1301 — phase-0-codebase-ingestion: entry-point

**Workflow:** same
**Step:** `source-acquisition` (line 18)
**Confidence:** HIGH
**Source line(s):** 18-21
**Type:** skill (`skills/brownfield-ingest/steps/step-a-source-acquisition.md`)
**Depends on:** `[]`
**Behavior:** The single root step (no upstream dependencies). It is the first invocation when this phase begins.
**Acceptance:** Topological sort of this workflow has `source-acquisition` as the unique zero-in-degree node.

### BC-AUDIT-1302 — phase-0-codebase-ingestion: terminal-step

**Workflow:** same
**Step:** `human-approval` (line 134)
**Confidence:** HIGH
**Source line(s):** 134-146
**Type:** human-approval
**Depends on:** `[input-hash-drift-check]`
**Behavior:** Final gate. Presents the project context document, recovered architecture, conventions, security audit, validation report, and module-criticality classification for human approval; 24h timeout.
**Acceptance:** No step in the workflow has `human-approval` in its `depends_on` list. The step prompt asks "Review the project context document. Is the recovered architecture accurate?".

### BC-AUDIT-1303 — phase-0-codebase-ingestion: DAG integrity

**Workflow:** same
**Step:** workflow-level
**Confidence:** HIGH
**Source line(s):** 15-146
**Behavior:** The 15 steps form an acyclic linear chain plus a fan-in to the gate. Every `depends_on` reference resolves to a defined step. Sequence: source-acquisition → backup → broad-sweep → backup → convergence-deepening → backup → coverage-audit → backup → extraction-validation → backup → final-synthesis → backup → phase-0-gate → input-hash-drift-check → human-approval.
**Acceptance:** Topological sort succeeds; reverse-DFS detects no cycles; for every step, `step.depends_on ⊆ {names of preceding steps}`.

### BC-AUDIT-1304 — phase-0-codebase-ingestion: failure semantics

**Workflow:** same
**Step:** workflow-level (defaults)
**Confidence:** HIGH
**Source line(s):** 10-13
**Behavior:** Default `on_failure: escalate`, `max_retries: 2`, `timeout: 4h`. No step overrides `on_failure` (i.e., all failures escalate to human). The gate uses `fail_action: block` (line 125) — Phase 0 cannot complete on a failing gate.
**Acceptance:** Reading the YAML produces `defaults.on_failure == "escalate"`. The phase-0-gate has `gate.fail_action == "block"`.

### Per-step BCs

### BC-AUDIT-1305 — phase-0:source-acquisition

**Step:** `source-acquisition` (line 18)
**Type:** skill
**Skill:** `skills/brownfield-ingest/steps/step-a-source-acquisition.md`
**Depends on:** `[]` | **on_failure:** escalate (default) | **max_retries:** 2 (default) | **timeout:** 4h (default)
**Source line(s):** 18-21
**Behavior:** Invokes step-a (source acquisition) — clones/identifies source tree, prepares analysis.
**Acceptance:** After completion, the source corpus is available for downstream pass files.

### BC-AUDIT-1306 — phase-0:backup-source-acquisition

**Step:** `backup-source-acquisition` (line 23) | **Type:** agent | **Agent:** state-manager
**Depends on:** `[source-acquisition]` | timeouts/retries: defaults
**Source line(s):** 23-29
**Behavior:** Commits source-acquisition artifacts to factory-artifacts; updates `STATE.md` with `phase: 0, step: source-acquisition, status: complete`.
**Acceptance:** STATE.md reflects step completion; factory-artifacts branch contains the artefacts.

### BC-AUDIT-1307 — phase-0:broad-sweep

**Step:** `broad-sweep` (line 33) | **Type:** skill | **Skill:** `step-b-broad-sweep.md`
**Depends on:** `[backup-source-acquisition]` | **timeout:** 2h (override)
**Source line(s):** 33-37
**Behavior:** Runs the 7-pass broad sweep (Pass 0 through 6) against the acquired source, producing the initial pass files.
**Acceptance:** 7 broad-sweep pass files produced (one per pass).

### BC-AUDIT-1308 — phase-0:backup-broad-sweep

**Step:** `backup-broad-sweep` (line 39) | **Type:** agent | **Agent:** state-manager
**Depends on:** `[broad-sweep]`
**Source line(s):** 39-45
**Behavior:** Commits the 7 pass files from broad-sweep; updates STATE.md.

### BC-AUDIT-1309 — phase-0:convergence-deepening

**Step:** `convergence-deepening` (line 49) | **Type:** skill | **Skill:** `step-c-convergence-deepening.md`
**Depends on:** `[backup-broad-sweep]` | **timeout:** 8h (override)
**Source line(s):** 49-53
**Behavior:** Per-pass convergence rounds until each pass declares NITPICK novelty (max 5 rounds, min 2). Allotted 8 hours wall time.

### BC-AUDIT-1310 — phase-0:backup-convergence-deepening

**Step:** `backup-convergence-deepening` (line 55) | **Type:** agent | **Agent:** state-manager
**Source line(s):** 55-61
**Behavior:** Commits all deepening-round artifacts across passes 0-5.

### BC-AUDIT-1311 — phase-0:coverage-audit

**Step:** `coverage-audit` (line 65) | **Type:** skill | **Skill:** `step-d-coverage-audit.md`
**Depends on:** `[backup-convergence-deepening]` | **timeout:** 2h
**Source line(s):** 65-69
**Behavior:** Audits coverage of the broad+deep extractions against the source corpus; identifies gaps before extraction validation.

### BC-AUDIT-1312 — phase-0:backup-coverage-audit

**Step:** `backup-coverage-audit` (line 71) | **Agent:** state-manager
**Source line(s):** 71-77

### BC-AUDIT-1313 — phase-0:extraction-validation

**Step:** `extraction-validation` (line 81) | **Type:** skill | **Skill:** `step-e-extraction-validation.md`
**Depends on:** `[backup-coverage-audit]` | **timeout:** 1h
**Source line(s):** 81-85
**Behavior:** Validates the extracted entities/contracts against the source — sanity check that nothing fabricated.

### BC-AUDIT-1314 — phase-0:backup-extraction-validation

**Step:** `backup-extraction-validation` (line 87) | **Agent:** state-manager
**Source line(s):** 87-93

### BC-AUDIT-1315 — phase-0:final-synthesis

**Step:** `final-synthesis` (line 97) | **Type:** skill | **Skill:** `step-f-final-synthesis.md`
**Depends on:** `[backup-extraction-validation]` | **timeout:** 1h
**Source line(s):** 97-101
**Behavior:** Produces the unified pass-8 synthesis document and project-context artifacts.

### BC-AUDIT-1316 — phase-0:backup-final-synthesis

**Step:** `backup-final-synthesis` (line 103) | **Agent:** state-manager
**Source line(s):** 103-109

### BC-AUDIT-1317 — phase-0:phase-0-gate

**Step:** `phase-0-gate` (line 113) | **Type:** gate | **Depends on:** `[backup-final-synthesis]`
**Source line(s):** 113-125
**Behavior:** Blocking gate. Verifies 7 criteria: project-context.md exists, module-criticality.md exists, ≥1 BC in `behavioral-contracts/`, BCs use `BC-S.SS.NNN` with `origin: recovered`, security-audit.md exists, validation-report.md passed, holdout scenarios seeded.
**Acceptance:** All 7 criteria satisfied or `fail_action: block` halts the pipeline.

### BC-AUDIT-1318 — phase-0:input-hash-drift-check

**Step:** `input-hash-drift-check` (line 127) | **Type:** skill | **Skill:** `skills/check-input-drift/SKILL.md`
**Depends on:** `[phase-0-gate]`
**Source line(s):** 127-132
**Behavior:** Scans Phase 0 artifacts for stale `input-hash` frontmatter before human review.

### BC-AUDIT-1319 — phase-0:human-approval

**Step:** `human-approval` (line 134) | **Type:** human-approval | **Timeout:** 24h
**Depends on:** `[input-hash-drift-check]`
**Source line(s):** 134-146
**Behavior:** Human reviews 6 listed artifacts; gate prompt asks about architecture accuracy, conventions correctness, restricted-area identification.

---

## 2 — Phase 1: Spec Crystallization

**File:** `plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster`

### Workflow-level BCs

### BC-AUDIT-1320 — phase-1-spec-crystallization: identity

**Source line(s):** 1-8
**Behavior:** Workflow `phase-1-spec-crystallization` (v2.0.0) transforms a product brief into domain spec, PRD, behavioral contracts, architecture, verification properties, and UX spec. Composed of `create-brief`, `create-domain-spec`, `create-prd`, `create-architecture` skills with state-manager backups; ends in adversarial spec review loop and human approval.
**Acceptance:** Loader returns `name=phase-1-spec-crystallization`, `version=2.0.0`.

### BC-AUDIT-1321 — phase-1: entry-point

**Step:** `create-brief` (line 18) — `depends_on: []`
**Source line(s):** 18-21
**Behavior:** Sole entry point.

### BC-AUDIT-1322 — phase-1: terminal-step

**Step:** `human-approval` (line 148) — depends on `[input-hash-drift-check]`; 24h timeout
**Source line(s):** 148-161
**Behavior:** Final approval over `prd.md`, `architecture/`, `verification-properties/`, `behavioral-contracts/`, `ux-spec.md`, `module-criticality.md`.

### BC-AUDIT-1323 — phase-1: DAG integrity

**Source line(s):** 15-161
**Behavior:** 14 steps acyclic. Backup steps form a linear chain with their producing steps. `prd-revision` is conditional (`condition: exists('.factory/specs/architecture-feasibility-report.md')`, line 82). `spec-gate` fans in from `[backup-create-architecture, backup-prd-revision]` (line 96), accommodating the conditional revision.
**Acceptance:** Topological sort succeeds; spec-gate's two-parent fan-in correctly handles either presence/absence of `prd-revision`.

### BC-AUDIT-1324 — phase-1: failure semantics

**Source line(s):** 10-13, 104, 138-140
**Behavior:** Defaults `on_failure: escalate`, `max_retries: 2`, `timeout: 2h`. `spec-gate.fail_action: block`. Adversarial review is a `loop` step (line 108) with `max_iterations: 10`, exit on `adversary.verdict == 'CONVERGENCE_REACHED'` — bounded retry, not free-form retry.

### Per-step BCs

### BC-AUDIT-1325 — phase-1:create-brief

**Step:** `create-brief` (line 18) | **Type:** skill | **Skill:** `skills/create-brief/SKILL.md`
**Depends on:** `[]`
**Source line(s):** 18-21
**Behavior:** Generates the product brief (Step 1a).

### BC-AUDIT-1326 — phase-1:backup-create-brief

**Step:** `backup-create-brief` (line 23) | **Agent:** state-manager
**Source line(s):** 23-29

### BC-AUDIT-1327 — phase-1:create-domain-spec

**Step:** `create-domain-spec` (line 33) | **Type:** skill | **Skill:** `skills/create-domain-spec/SKILL.md`
**Depends on:** `[backup-create-brief]`
**Source line(s):** 33-36
**Behavior:** Produces L2 domain spec from the brief.

### BC-AUDIT-1328 — phase-1:backup-create-domain-spec

**Step:** `backup-create-domain-spec` (line 38) | **Agent:** state-manager
**Source line(s):** 38-44

### BC-AUDIT-1329 — phase-1:create-prd

**Step:** `create-prd` (line 48) | **Type:** skill | **Skill:** `skills/create-prd/SKILL.md`
**Depends on:** `[backup-create-domain-spec]`
**Source line(s):** 48-51
**Behavior:** Produces PRD and behavioral contracts (BC-S.SS.NNN format).

### BC-AUDIT-1330 — phase-1:backup-create-prd

**Step:** `backup-create-prd` (line 53) | **Agent:** state-manager
**Source line(s):** 53-59

### BC-AUDIT-1331 — phase-1:create-architecture

**Step:** `create-architecture` (line 63) | **Type:** skill | **Skill:** `skills/create-architecture/SKILL.md`
**Depends on:** `[backup-create-prd]`
**Source line(s):** 63-66
**Behavior:** Produces architecture sections + verification-properties; may emit a feasibility report that triggers prd-revision.

### BC-AUDIT-1332 — phase-1:backup-create-architecture

**Step:** `backup-create-architecture` (line 68) | **Agent:** state-manager
**Source line(s):** 68-74

### BC-AUDIT-1333 — phase-1:prd-revision

**Step:** `prd-revision` (line 78) | **Type:** skill | **Skill:** `skills/phase-1-prd-revision/SKILL.md`
**Depends on:** `[backup-create-architecture]` | **Condition:** `exists('.factory/specs/architecture-feasibility-report.md')`
**Source line(s):** 78-82
**Behavior:** Conditional architect-feedback PRD revision loop.

### BC-AUDIT-1334 — phase-1:backup-prd-revision

**Step:** `backup-prd-revision` (line 84) | **Agent:** state-manager
**Source line(s):** 84-90

### BC-AUDIT-1335 — phase-1:spec-gate

**Step:** `spec-gate` (line 94) | **Type:** gate
**Depends on:** `[backup-create-architecture, backup-prd-revision]` | **fail_action:** block
**Source line(s):** 94-104
**Behavior:** Blocks unless: all requirements have unique IDs and numerical targets; Provable Properties Catalog covers all security boundaries; Purity Boundary Map is complete; verification tooling selected; module-criticality.md written.

### BC-AUDIT-1336 — phase-1:adversarial-spec-review

**Step:** `adversarial-spec-review` (line 108) | **Type:** loop
**Depends on:** `[spec-gate]` | **max_iterations:** 10 | **exit_condition:** `adversary.verdict == 'CONVERGENCE_REACHED'`
**Source line(s):** 108-135
**Behavior:** Each iteration spawns adversary agent (model_tier: adversary) on `.factory/specs/**` and `module-criticality.md`, excluding `holdout-scenarios/**` (information-asymmetry wall). Adversary writes findings to `adversarial-spec-review.md`; `phase-1d-adversarial-spec-review` skill fixes findings. Exits on convergence.

### BC-AUDIT-1337 — phase-1:input-hash-drift-check

**Step:** `input-hash-drift-check` (line 138) | **Type:** skill
**Depends on:** `[adversarial-spec-review]`
**Source line(s):** 138-145

### BC-AUDIT-1338 — phase-1:human-approval

**Step:** `human-approval` (line 148) | **Timeout:** 24h
**Source line(s):** 148-161

---

## 3 — Phase 2: Story Decomposition

**File:** `plugins/vsdd-factory/workflows/phases/phase-2-story-decomposition.lobster`

### BC-AUDIT-1340 — phase-2-story-decomposition: identity

**Source line(s):** 1-8
**Behavior:** v1.0.0. Breaks PRD/architecture into epics, stories, dependency graph, wave schedule, and holdout scenarios via the decompose-stories skill steps.

### BC-AUDIT-1341 — phase-2: entry-point

**Step:** `define-epics` (line 18) — `depends_on: []`

### BC-AUDIT-1342 — phase-2: terminal-step

**Step:** `human-approval` (line 158) — 24h timeout, depends on `[input-hash-drift-check]`. Approves epics.md, STORY-INDEX.md, dependency-graph.md, wave-schedule.md, sprint-state.yaml, HS-INDEX.md.

### BC-AUDIT-1343 — phase-2: DAG integrity

**Source line(s):** 15-171
**Behavior:** 14 steps acyclic; linear backup chain; final flow define-epics → create-stories → dependency-graph → wave-schedule → holdout-scenarios → decomposition-gate → adversarial-story-review (loop) → input-hash-drift-check → human-approval.

### BC-AUDIT-1344 — phase-2: failure semantics

**Source line(s):** 10-13, 105
**Behavior:** Defaults `on_failure: escalate`, retries 2, timeout 2h. `decomposition-gate.fail_action: block`. Adversarial story review is a `loop` with max_iterations 10 and exit on CONVERGENCE_REACHED.

### Per-step BCs (phase-2)

### BC-AUDIT-1345 — phase-2:define-epics
**Step:** define-epics (line 18) | Type: skill | Skill: `decompose-stories/steps/step-a-define-epics.md` | Depends: `[]` | Source: 18-21

### BC-AUDIT-1346 — phase-2:backup-define-epics
**Step:** backup-define-epics (line 23) | Agent: state-manager | Source: 23-29

### BC-AUDIT-1347 — phase-2:create-stories
**Step:** create-stories (line 33) | Type: skill | Skill: `decompose-stories/steps/step-b-create-stories.md` | Depends: `[backup-define-epics]` | Timeout: 1h | Source: 33-37

### BC-AUDIT-1348 — phase-2:backup-create-stories
**Step:** backup-create-stories (line 39) | Agent: state-manager | Source: 39-45

### BC-AUDIT-1349 — phase-2:dependency-graph
**Step:** dependency-graph (line 49) | Type: skill | Skill: `decompose-stories/steps/step-c-dependency-graph.md` | Depends: `[backup-create-stories]` | Source: 49-52

### BC-AUDIT-1350 — phase-2:backup-dependency-graph
**Step:** backup-dependency-graph (line 54) | Agent: state-manager | Source: 54-60

### BC-AUDIT-1351 — phase-2:wave-schedule
**Step:** wave-schedule (line 64) | Type: skill | Skill: `decompose-stories/steps/step-d-wave-schedule.md` | Depends: `[backup-dependency-graph]` | Source: 64-67

### BC-AUDIT-1352 — phase-2:backup-wave-schedule
**Step:** backup-wave-schedule (line 69) | Agent: state-manager | Source: 69-75

### BC-AUDIT-1353 — phase-2:holdout-scenarios
**Step:** holdout-scenarios (line 79) | Type: skill | Skill: `decompose-stories/steps/step-e-holdout-scenarios.md` | Depends: `[backup-wave-schedule]` | Source: 79-82

### BC-AUDIT-1354 — phase-2:backup-holdout-scenarios
**Step:** backup-holdout-scenarios (line 84) | Agent: state-manager | Source: 84-90

### BC-AUDIT-1355 — phase-2:decomposition-gate
**Step:** decomposition-gate (line 94) | Type: gate | Depends: `[backup-holdout-scenarios]` | fail_action: block | Source: 94-105
**Behavior:** Verifies BC traceability, no TBD/TODO in ACs, acyclic dependency graph, wave assignments respect dependencies, STORY-INDEX matches, ≥1 holdout scenario per wave.

### BC-AUDIT-1356 — phase-2:adversarial-story-review
**Step:** adversarial-story-review (line 109) | Type: loop | max_iterations: 10 | exit: `adversary.verdict == 'CONVERGENCE_REACHED'` | Source: 109-145
**Behavior:** Adversary on `.factory/stories/**` and BCs/VPs/PRD/architecture; excludes holdout scenarios + prior adversarial reviews (information-asymmetry wall). `decompose-stories` skill fixes findings.

### BC-AUDIT-1357 — phase-2:input-hash-drift-check
**Step:** input-hash-drift-check (line 149) | Type: skill | Source: 149-154

### BC-AUDIT-1358 — phase-2:human-approval
**Step:** human-approval (line 158) | Timeout: 24h | Source: 158-171

---

## 4 — Phase 3: TDD Implementation

**File:** `plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster`

### BC-AUDIT-1360 — phase-3-tdd-implementation: identity
**Source line(s):** 1-8 | **Behavior:** v2.0.0. Per-story TDD delivery via specialist agents (devops, test-writer, implementer, demo-recorder, pr-manager). Workflow runs once per story.

### BC-AUDIT-1361 — phase-3: entry-point
**Step:** `create-worktree` (line 18) — `depends_on: []`. Type: skill, skill: `deliver-story/steps/step-a-create-worktree.md`.

### BC-AUDIT-1362 — phase-3: terminal-step
**Step:** `input-hash-drift-check` (line 138) | Depends: `[implementation-gate]`
**Source line(s):** 138-144
**Behavior:** Final step verifies spec artifacts have not drifted since implementation began; no human-approval (per-story flow).

### BC-AUDIT-1363 — phase-3: DAG integrity
**Source line(s):** 15-144 | 16 steps acyclic. Linear chain create-worktree → generate-stubs → failing-tests → implement → record-demos → pr-lifecycle → cleanup, each with state-manager backup; then implementation-gate → input-hash-drift-check.

### BC-AUDIT-1364 — phase-3: failure semantics
**Source line(s):** 10-13, 134
**Behavior:** Defaults `on_failure: escalate`, retries 2, timeout 4h. `implementation-gate.fail_action: block` over Red Gate proof, all tests pass, purity boundaries, no `unwrap()`, clippy clean, demo evidence per AC, PR merged, worktree cleaned.

### BC-AUDIT-1365 — phase-3:create-worktree
**Step:** create-worktree (line 18) | Type: skill | Skill: `deliver-story/steps/step-a-create-worktree.md` | Depends: `[]` | Source: 18-21

### BC-AUDIT-1366 — phase-3:backup-create-worktree
**Step:** backup-create-worktree (line 23) | Agent: state-manager | Source: 23-30

### BC-AUDIT-1367 — phase-3:generate-stubs
**Step:** generate-stubs (line 33) | Type: skill | Skill: `step-b-generate-stubs.md` | Depends: `[backup-create-worktree]` | Source: 33-36

### BC-AUDIT-1368 — phase-3:backup-generate-stubs
**Step:** backup-generate-stubs (line 38) | Agent: state-manager | Source: 38-43

### BC-AUDIT-1369 — phase-3:failing-tests
**Step:** failing-tests (line 47) | Type: skill | Skill: `step-c-failing-tests.md` | Depends: `[backup-generate-stubs]` | Source: 47-50
**Behavior:** Writes failing tests + Red Gate (compile pass, all assertions fail).

### BC-AUDIT-1370 — phase-3:backup-failing-tests
**Step:** backup-failing-tests (line 52) | Agent: state-manager | Source: 52-58

### BC-AUDIT-1371 — phase-3:implement
**Step:** implement (line 62) | Type: skill | Skill: `step-d-implement.md` | Depends: `[backup-failing-tests]` | Timeout: 2h | Source: 62-66
**Behavior:** TDD inner loop — pick next failing test, write minimum code, repeat.

### BC-AUDIT-1372 — phase-3:backup-implement
**Step:** backup-implement (line 68) | Agent: state-manager | Source: 68-73

### BC-AUDIT-1373 — phase-3:record-demos
**Step:** record-demos (line 77) | Type: skill | Skill: `step-e-record-demos.md` | Depends: `[backup-implement]` | Source: 77-80

### BC-AUDIT-1374 — phase-3:backup-record-demos
**Step:** backup-record-demos (line 82) | Agent: state-manager | Source: 82-87

### BC-AUDIT-1375 — phase-3:pr-lifecycle
**Step:** pr-lifecycle (line 91) | Type: skill | Skill: `step-f-pr-lifecycle.md` | Depends: `[backup-record-demos]` | Timeout: 1h | Source: 91-95

### BC-AUDIT-1376 — phase-3:backup-pr-lifecycle
**Step:** backup-pr-lifecycle (line 97) | Agent: state-manager | Source: 97-102

### BC-AUDIT-1377 — phase-3:cleanup
**Step:** cleanup (line 106) | Type: skill | Skill: `step-g-cleanup.md` | Depends: `[backup-pr-lifecycle]` | Source: 106-109

### BC-AUDIT-1378 — phase-3:backup-cleanup
**Step:** backup-cleanup (line 111) | Agent: state-manager | Source: 111-117
**Behavior:** Updates STATE.md and `sprint-state.yaml` with story status `completed`.

### BC-AUDIT-1379 — phase-3:implementation-gate
**Step:** implementation-gate (line 121) | Type: gate | fail_action: block | Source: 121-134
**Behavior:** Verifies Red Gate proof, all tests pass, purity boundary respected, no `unwrap()` in production, clippy/lint zero warnings, demo evidence per AC, PR merged to develop, worktree cleaned.

### BC-AUDIT-1380 — phase-3:input-hash-drift-check
**Step:** input-hash-drift-check (line 138) | Type: skill | Source: 138-144

---

## 5 — Phase 4: Holdout Evaluation

**File:** `plugins/vsdd-factory/workflows/phases/phase-4-holdout-evaluation.lobster`

### BC-AUDIT-1390 — phase-4-holdout-evaluation: identity
**Source line(s):** 1-6 | **Behavior:** v1.0.0. Reusable sub-workflow: scenario rotation + holdout evaluation with information asymmetry wall enforcement.

### BC-AUDIT-1391 — phase-4: entry-point
**Step:** `scenario-rotation` (line 14) — `depends_on: []`. Agent: orchestrator.

### BC-AUDIT-1392 — phase-4: terminal-step
**Step:** `holdout-gate` (line 28) — gate type, depends `[holdout-evaluation]`. Source 28-38.

### BC-AUDIT-1393 — phase-4: DAG integrity
3 steps strictly linear: scenario-rotation → holdout-evaluation → holdout-gate. Trivially acyclic.

### BC-AUDIT-1394 — phase-4: failure semantics
**Source line(s):** 8-11, 38 | Defaults: escalate, 2 retries, 2h. `holdout-gate.fail_action: block` on 5 criteria.

### BC-AUDIT-1395 — phase-4:scenario-rotation
**Step:** scenario-rotation (line 14) | Type: agent | Agent: orchestrator | Depends: `[]` | Source: 14-22
**Behavior:** Randomly selects 80% of holdout scenarios per pipeline run; writes selected IDs to `.factory/holdout-evaluation/scenario-selection.json`.

### BC-AUDIT-1396 — phase-4:holdout-evaluation
**Step:** holdout-evaluation (line 23) | Type: skill | Skill: `skills/holdout-eval/SKILL.md` | Depends: `[scenario-rotation]` | Source: 23-26

### BC-AUDIT-1397 — phase-4:holdout-gate
**Step:** holdout-gate (line 28) | Type: gate | fail_action: block | Source: 28-38
**Behavior:** Verifies different model family used (GPT-5.4 not Claude), mean satisfaction ≥0.85, no must-pass scenario <0.6, std-dev <0.15, scenario rotation applied.

---

## 6 — Phase 5: Adversarial Refinement

**File:** `plugins/vsdd-factory/workflows/phases/phase-5-adversarial-refinement.lobster`

### BC-AUDIT-1400 — phase-5-adversarial-refinement: identity
**Source line(s):** 1-5 | **Behavior:** v1.0.0. Multi-model adversarial review loop with GPT-5.4 primary and Gemini 3.1 Pro secondary pass.

### BC-AUDIT-1401 — phase-5: entry-point
**Step:** `adversarial-review-loop` (line 14) — `depends_on: []`. Type: loop.

### BC-AUDIT-1402 — phase-5: terminal-step
**Step:** `gemini-secondary-review` (line 41) — depends `[adversarial-review-loop]`. Conditional: `config.enable_secondary_adversary == true`.

### BC-AUDIT-1403 — phase-5: DAG integrity
2 top-level steps: adversarial-review-loop → gemini-secondary-review (conditional). The loop contains 2 nested steps (adversary-code-review, triage-and-fix).

### BC-AUDIT-1404 — phase-5: failure semantics
**Source line(s):** 8-11 | Defaults: escalate, 2 retries, 3h. Loop has `max_iterations: 10`, `exit_condition: 'adversary.verdict == CONVERGENCE_REACHED'`.

### BC-AUDIT-1405 — phase-5:adversarial-review-loop
**Step:** adversarial-review-loop (line 14) | Type: loop | max_iterations: 10 | Source: 14-39
**Behavior:** Per-iteration: spawn adversary on `.factory/specs/**`, `.factory/cycles/**/implementation/**`, `src/**`, `tests/**` (no info-asymmetry exclusion in this phase since adversary needs full picture). Triage-and-fix skill runs after.

### BC-AUDIT-1406 — phase-5:adversary-code-review (nested)
**Step:** adversary-code-review (line 21) | Type: agent | Agent: adversary | model_tier: adversary | Source: 21-35
**Behavior:** Reviews codebase for spec fidelity, test quality, code quality, security surface, spec gaps. Reports CONVERGENCE_REACHED if findings cosmetic only.

### BC-AUDIT-1407 — phase-5:triage-and-fix (nested)
**Step:** triage-and-fix (line 36) | Type: skill | Skill: `skills/adversarial-review/SKILL.md` | Depends: `[adversary-code-review]` | Source: 36-39

### BC-AUDIT-1408 — phase-5:gemini-secondary-review
**Step:** gemini-secondary-review (line 41) | Type: agent | Agent: code-reviewer | model_tier: review | Condition: `config.enable_secondary_adversary == true` | Source: 41-55
**Behavior:** Secondary review using Gemini 3.1 Pro on fresh context; targets patterns the GPT-5.4 adversary may have missed.

---

## 7 — Phase 6: Formal Hardening

**File:** `plugins/vsdd-factory/workflows/phases/phase-6-formal-hardening.lobster`

### BC-AUDIT-1410 — phase-6-formal-hardening: identity
**Source line(s):** 1-7 | **Behavior:** v2.0.0. Four verification techniques (Kani proofs, fuzz testing, mutation testing, security scanning) with state-manager backups for crash recovery.

### BC-AUDIT-1411 — phase-6: entry-point
**Step:** `kani-proofs` (line 17) — `depends_on: []`. Type: skill.

### BC-AUDIT-1412 — phase-6: terminal-step
**Step:** `hardening-gate` (line 80) — gate type, depends `[backup-security-scan]`. fail_action: block.

### BC-AUDIT-1413 — phase-6: DAG integrity
9 steps acyclic; linear chain kani-proofs → backup → fuzz-testing → backup → mutation-testing → backup → security-scan → backup → hardening-gate.

### BC-AUDIT-1414 — phase-6: failure semantics
**Source line(s):** 9-12, 91 | Defaults: escalate, 2 retries, 4h. `hardening-gate.fail_action: block`.

### BC-AUDIT-1415 — phase-6:kani-proofs
**Step:** kani-proofs (line 17) | Type: skill | Skill: `formal-verify/steps/step-a-kani-proofs.md` | Depends: `[]` | Timeout: 2h | Source: 17-21

### BC-AUDIT-1416 — phase-6:backup-kani-proofs
**Step:** backup-kani-proofs (line 23) | Agent: state-manager | Source: 23-29

### BC-AUDIT-1417 — phase-6:fuzz-testing
**Step:** fuzz-testing (line 33) | Type: skill | Skill: `formal-verify/steps/step-b-fuzz-testing.md` | Depends: `[backup-kani-proofs]` | Timeout: 2h | Source: 33-37

### BC-AUDIT-1418 — phase-6:backup-fuzz-testing
**Step:** backup-fuzz-testing (line 39) | Agent: state-manager | Source: 39-45

### BC-AUDIT-1419 — phase-6:mutation-testing
**Step:** mutation-testing (line 49) | Type: skill | Skill: `formal-verify/steps/step-c-mutation-testing.md` | Depends: `[backup-fuzz-testing]` | Timeout: 1h | Source: 49-53

### BC-AUDIT-1420 — phase-6:backup-mutation-testing
**Step:** backup-mutation-testing (line 55) | Agent: state-manager | Source: 55-61

### BC-AUDIT-1421 — phase-6:security-scan
**Step:** security-scan (line 65) | Type: skill | Skill: `formal-verify/steps/step-d-security-scan.md` | Depends: `[backup-mutation-testing]` | Source: 65-68

### BC-AUDIT-1422 — phase-6:backup-security-scan
**Step:** backup-security-scan (line 70) | Agent: state-manager | Source: 70-76

### BC-AUDIT-1423 — phase-6:hardening-gate
**Step:** hardening-gate (line 80) | Type: gate | fail_action: block | Source: 80-91
**Behavior:** Verifies Kani proofs pass, fuzz 5min/target with zero crashes, mutation kill rate >90% (criticality-adjusted), zero crit/high Semgrep findings, cargo audit clean, purity boundaries intact.

---

## 8 — Phase 7: Convergence

**File:** `plugins/vsdd-factory/workflows/phases/phase-7-convergence.lobster`

### BC-AUDIT-1430 — phase-7-convergence: identity
**Source line(s):** 1-8 | **Behavior:** v2.0.0. 7-Dimensional Convergence Assessment (spec, test, implementation, verification, visual, performance, documentation) with state-manager backups; followed by demo recording, visual review, human approval.

### BC-AUDIT-1431 — phase-7: entry-point
**Step:** `spec-convergence` (line 17) — `depends_on: []`.

### BC-AUDIT-1432 — phase-7: terminal-step
**Step:** `human-approval` (line 164) — depends `[convergence-gate, visual-review]`. 48h timeout. Reviews convergence-report.md, traceability-matrix.md, visual-review.md.

### BC-AUDIT-1433 — phase-7: DAG integrity
18 steps acyclic; 7 dimension+backup chains, then convergence-gate, input-hash-drift-check, conditional convergence-demo, conditional visual-review, human-approval (fan-in).

### BC-AUDIT-1434 — phase-7: failure semantics
**Source line(s):** 9-12, 130 | Defaults: escalate, 2 retries, 2h. `convergence-gate.fail_action: block` on 9 criteria.

### BC-AUDIT-1435 — phase-7:spec-convergence
**Step:** spec-convergence (line 17) | Type: skill | Skill: `convergence-check/steps/step-a-spec-convergence.md` | Depends: `[]` | Source: 17-20

### BC-AUDIT-1436 — phase-7:backup-spec-convergence
**Step:** backup-spec-convergence (line 22) | Agent: state-manager | Source: 22-27

### BC-AUDIT-1437 — phase-7:test-convergence
**Step:** test-convergence (line 31) | Type: skill | Skill: `step-b-test-convergence.md` | Depends: `[backup-spec-convergence]` | Source: 31-34

### BC-AUDIT-1438 — phase-7:backup-test-convergence
**Step:** backup-test-convergence (line 36) | Agent: state-manager | Source: 36-41

### BC-AUDIT-1439 — phase-7:implementation-convergence
**Step:** implementation-convergence (line 45) | Type: skill | Skill: `step-c-implementation-convergence.md` | Depends: `[backup-test-convergence]` | Source: 45-48

### BC-AUDIT-1440 — phase-7:backup-implementation-convergence
**Step:** backup-implementation-convergence (line 50) | Agent: state-manager | Source: 50-55

### BC-AUDIT-1441 — phase-7:verification-convergence
**Step:** verification-convergence (line 59) | Type: skill | Skill: `step-d-verification-convergence.md` | Depends: `[backup-implementation-convergence]` | Source: 59-62

### BC-AUDIT-1442 — phase-7:backup-verification-convergence
**Step:** backup-verification-convergence (line 64) | Agent: state-manager | Source: 64-69

### BC-AUDIT-1443 — phase-7:visual-convergence
**Step:** visual-convergence (line 73) | Type: skill | Skill: `step-e-visual-convergence.md` | Depends: `[backup-verification-convergence]` | Source: 73-76

### BC-AUDIT-1444 — phase-7:backup-visual-convergence
**Step:** backup-visual-convergence (line 78) | Agent: state-manager | Source: 78-83

### BC-AUDIT-1445 — phase-7:performance-convergence
**Step:** performance-convergence (line 87) | Type: skill | Skill: `step-f-performance-convergence.md` | Depends: `[backup-visual-convergence]` | Source: 87-90

### BC-AUDIT-1446 — phase-7:backup-performance-convergence
**Step:** backup-performance-convergence (line 92) | Agent: state-manager | Source: 92-97

### BC-AUDIT-1447 — phase-7:documentation-convergence
**Step:** documentation-convergence (line 101) | Type: skill | Skill: `step-g-documentation-convergence.md` | Depends: `[backup-performance-convergence]` | Source: 101-104

### BC-AUDIT-1448 — phase-7:backup-documentation-convergence
**Step:** backup-documentation-convergence (line 106) | Agent: state-manager | Source: 106-112
**Behavior:** Commits the convergence report.

### BC-AUDIT-1449 — phase-7:convergence-gate
**Step:** convergence-gate (line 116) | Type: gate | fail_action: block | Source: 116-130
**Behavior:** All 7 dimensions CONVERGED + traceability matrix generated + convergence report generated.

### BC-AUDIT-1450 — phase-7:input-hash-drift-check
**Step:** input-hash-drift-check (line 134) | Type: skill | Source: 134-140

### BC-AUDIT-1451 — phase-7:convergence-demo
**Step:** convergence-demo (line 144) | Type: skill | Skill: `skills/demo-recording/SKILL.md` | Condition: `config.demo_recording.enabled != false` | Timeout: 30m | Source: 144-149

### BC-AUDIT-1452 — phase-7:visual-review
**Step:** visual-review (line 151) | Type: agent | Agent: visual-reviewer | model_tier: review | Condition: `config.demo_recording.enabled != false` | Source: 151-160

### BC-AUDIT-1453 — phase-7:human-approval
**Step:** human-approval (line 164) | Type: human-approval | Timeout: 48h | Depends: `[convergence-gate, visual-review]` | Source: 164-175
**Behavior:** Final ZERO-SLOP achievement gate.

---

## 9 — Greenfield Mode (greenfield.lobster)

**File:** `plugins/vsdd-factory/workflows/greenfield.lobster`

### Workflow-level BCs

### BC-AUDIT-1460 — greenfield-vsdd: identity
**Source line(s):** 1-15 | **Behavior:** v2.1.0. Full VSDD pipeline transforming a product brief into verified, adversarially-reviewed, formally-proven code. Reference path; all other primary paths are subsets/variations. Includes adaptive planning, market intel, per-story delivery via `code-delivery.lobster`, 8 information-asymmetry walls, 4 security-review touchpoints, cost monitoring, fix-PR delivery, release pipeline, post-feature validation, design-system bootstrap, multi-variant design, heuristic eval, wave-level UI gates, convergence UI gate, Storybook MCP self-healing component test loop.
**Acceptance:** Loader reports `name=greenfield-vsdd`, `version=2.1.0`, `steps=72`.

### BC-AUDIT-1461 — greenfield: entry-point
**Step:** `repo-initialization` (line 37) — `depends_on: []`. Type: skill, skill: `skills/repo-initialization/SKILL.md`. Sole zero-in-degree node.

### BC-AUDIT-1462 — greenfield: terminal-step
**Step:** `process-review-decisions` (line 1398) — agent: state-manager, depends `[session-review-approval]`. Final post-pipeline housekeeping that processes human review decisions.
**Source line(s):** 1398-1410

### BC-AUDIT-1463 — greenfield: DAG integrity
**Source line(s):** 37-1410
**Behavior:** 72 steps acyclic. Topological sort succeeds. Critical fan-in points: phase-1-gate (9 parents), phase-1d-spec-review-gemini, phase-1-state-backup, phase-1-human-approval (chain), multi-repo branch (4 conditional steps), phase-2 sequential, phase-3-per-story-delivery (loop) → phase-3-gate, phase-4 (scenario-rotation + dtu-startup → holdout-evaluation), phase-5 (adversarial loop + Gemini secondary), phase-5-gate (multi-parent), phase-6 UI ladder (4 parallel skills → ui-quality-gate → conditional fix-delivery), phase-7-convergence, phase-6-gate, demo + visual-review, phase-6-human-approval (multi-parent). All `depends_on` resolve.

### BC-AUDIT-1464 — greenfield: failure semantics
**Source line(s):** 24-27 | **Behavior:** Defaults `on_failure: escalate`, retries 2, timeout 2h. No step overrides `on_failure`. Multiple `gate` steps with `fail_action: block`. Loops (`phase-1d-adversarial-spec-review`, `phase-1d-spec-review-gemini`, `phase-2-spec-review-gemini`, `phase-3-per-story-delivery`, `phase-5-adversarial-refinement`) bound iterations to a max with explicit exit conditions.

### BC-AUDIT-1465 — greenfield: cost monitoring (workflow-level)
**Source line(s):** workflow-level cost-monitoring (description claim, no explicit YAML field) | **Behavior:** Cost monitoring continuous through pipeline (per workflow description line 9-15). Per-step `model_tier` annotations route to T1/T2/T3 model tiers.

### Per-step BCs (greenfield)

### BC-AUDIT-1466 — greenfield:repo-initialization
**Step:** repo-initialization (line 37) | Type: skill | Skill: `skills/repo-initialization/SKILL.md` | Depends: `[]`

### BC-AUDIT-1467 — greenfield:factory-worktree-health
**Step:** factory-worktree-health (line 51) | Type: skill | Agent: devops-engineer | Skill: `skills/factory-worktree-health/SKILL.md` | Depends: `[repo-initialization]`

### BC-AUDIT-1468 — greenfield:factory-worktree-gate
**Step:** factory-worktree-gate (line 57) | Type: gate | Depends: `[factory-worktree-health]`

### BC-AUDIT-1469 — greenfield:scaffold-claude-md
**Step:** scaffold-claude-md (line 70) | Type: skill | Skill: `skills/scaffold-claude-md/SKILL.md` | Depends: `[factory-worktree-gate]` | Condition: `!file_exists('CLAUDE.md')`

### BC-AUDIT-1470 — greenfield:state-initialization
**Step:** state-initialization (line 81) | Type: agent | Agent: state-manager | Depends: `[factory-worktree-gate, scaffold-claude-md]`

### BC-AUDIT-1471 — greenfield:adaptive-planning
**Step:** adaptive-planning (line 97) | Type: sub-workflow | Sub-workflow: implicit (`planning.lobster` per cross-workflow context) | Depends: `[state-initialization]` | Condition: `config.skip_planning != true`

### BC-AUDIT-1472 — greenfield:phase-1-spec-crystallization
**Step:** phase-1-spec-crystallization (line 109) | Type: skill | Skill: `skills/phase-1-spec-crystallization/SKILL.md` | Depends: `[adaptive-planning]`

### BC-AUDIT-1473 — greenfield:architect-feasibility-review
**Step:** architect-feasibility-review (line 115) | Type: agent | Agent: architect | Depends: `[phase-1-spec-crystallization]`

### BC-AUDIT-1474 — greenfield:prd-revision
**Step:** prd-revision (line 124) | Type: skill | Skill: `skills/phase-1-prd-revision/SKILL.md` | Depends: `[architect-feasibility-review]` | Condition: `architect.verdict == 'request-changes'`

### BC-AUDIT-1475 — greenfield:phase-1-dtu-assessment
**Step:** phase-1-dtu-assessment (line 131) | Type: agent | Agent: architect | Depends: `[phase-1-spec-crystallization]`

### BC-AUDIT-1476 — greenfield:phase-1-gene-transfusion-assessment
**Step:** phase-1-gene-transfusion-assessment (line 140) | Type: agent | Agent: architect | Depends: `[phase-1-spec-crystallization]`

### BC-AUDIT-1477 — greenfield:phase-1-cicd-setup
**Step:** phase-1-cicd-setup (line 156) | Type: agent | Agent: devops-engineer | Depends: `[phase-1-spec-crystallization]`

### BC-AUDIT-1478 — greenfield:phase-1-design-system-bootstrap
**Step:** phase-1-design-system-bootstrap (line 172) | Type: skill | Skill: `skills/design-system-bootstrap/SKILL.md` | Depends: `[phase-1-spec-crystallization]` | Condition: `feature_type in ['ui', 'full-stack']`

### BC-AUDIT-1479 — greenfield:phase-1-design-system-approval
**Step:** phase-1-design-system-approval (line 180) | Type: human-approval | Depends: `[phase-1-design-system-bootstrap]` | Condition: same as above

### BC-AUDIT-1480 — greenfield:phase-1-multi-variant-design
**Step:** phase-1-multi-variant-design (line 193) | Type: skill | Skill: `skills/multi-variant-design/SKILL.md` | Depends: `[phase-1-spec-crystallization, phase-1-design-system-approval]` | Condition: `feature_type in ['ui','full-stack'] and ux_spec.has_complex_screens == true`

### BC-AUDIT-1481 — greenfield:phase-1-multi-variant-approval
**Step:** phase-1-multi-variant-approval (line 200) | Type: human-approval | Depends: `[phase-1-multi-variant-design]` | Condition: same

### BC-AUDIT-1482 — greenfield:phase-1-heuristic-evaluation
**Step:** phase-1-heuristic-evaluation (line 213) | Type: skill | Skill: `skills/ux-heuristic-evaluation/SKILL.md` | Depends: `[phase-1-spec-crystallization]` | Condition: `feature_type in ['ui','full-stack']`

### BC-AUDIT-1483 — greenfield:phase-1-consistency-audit
**Step:** phase-1-consistency-audit (line 220) | Type: agent | Agent: consistency-validator | Depends: `[phase-1-spec-crystallization, phase-1-dtu-assessment, phase-1-cicd-setup]`

### BC-AUDIT-1484 — greenfield:phase-1-gate
**Step:** phase-1-gate (line 231) | Type: gate | Depends: `[phase-1-spec-crystallization, architect-feasibility-review, prd-revision, phase-1-dtu-assessment, phase-1-gene-transfusion-assessment, phase-1-cicd-setup, phase-1-design-system-approval, phase-1-heuristic-evaluation, phase-1-consistency-audit]` (9-parent fan-in)
**Behavior:** Phase 1 omnibus gate.

### BC-AUDIT-1485 — greenfield:phase-1d-adversarial-spec-review
**Step:** phase-1d-adversarial-spec-review (line 259) | Type: loop | Depends: `[phase-1-gate]`

### BC-AUDIT-1486 — greenfield:phase-1d-spec-review-gemini
**Step:** phase-1d-spec-review-gemini (line 297) | Type: loop | Depends: `[phase-1d-adversarial-spec-review]`

### BC-AUDIT-1487 — greenfield:phase-1-state-backup
**Step:** phase-1-state-backup (line 327) | Type: agent | Agent: state-manager | Depends: `[phase-1d-spec-review-gemini]`

### BC-AUDIT-1488 — greenfield:phase-1-human-approval
**Step:** phase-1-human-approval (line 335) | Type: human-approval | Depends: `[phase-1-state-backup]`

### BC-AUDIT-1489 — greenfield:multi-repo-topology-check
**Step:** multi-repo-topology-check (line 362) | Type: agent | Agent: orchestrator | Depends: `[phase-1-human-approval]`

### BC-AUDIT-1490 — greenfield:multi-repo-human-confirmation
**Step:** multi-repo-human-confirmation (line 375) | Type: human-approval | Depends: `[multi-repo-topology-check]` | Condition: `deployment_topology == 'multi-service'`

### BC-AUDIT-1491 — greenfield:multi-repo-transition
**Step:** multi-repo-transition (line 389) | Type: agent | Agent: devops-engineer | Depends: `[multi-repo-human-confirmation]` | Condition: `human_approved_multi_repo == true`

### BC-AUDIT-1492 — greenfield:multi-repo-state-migration
**Step:** multi-repo-state-migration (line 401) | Type: agent | Agent: state-manager | Depends: `[multi-repo-transition]` | Condition: same

### BC-AUDIT-1493 — greenfield:phase-2-story-decomposition
**Step:** phase-2-story-decomposition (line 417) | Type: skill | Skill: `skills/phase-2-story-decomposition/SKILL.md` | Depends: `[phase-1-human-approval, multi-repo-topology-check]` | Condition: `deployment_topology != 'multi-service' OR human_approved_multi_repo == false`

### BC-AUDIT-1494 — greenfield:phase-2-consistency-check
**Step:** phase-2-consistency-check (line 423) | Type: agent | Agent: consistency-validator | Depends: `[phase-2-story-decomposition]`

### BC-AUDIT-1495 — greenfield:phase-2-adversarial-review
**Step:** phase-2-adversarial-review (line 431) | Type: agent | Agent: adversary | Depends: `[phase-2-consistency-check]`

### BC-AUDIT-1496 — greenfield:phase-2-consistency-audit
**Step:** phase-2-consistency-audit (line 452) | Type: agent | Agent: consistency-validator | Depends: `[phase-2-adversarial-review]`

### BC-AUDIT-1497 — greenfield:phase-2-gate
**Step:** phase-2-gate (line 464) | Type: gate | Depends: `[phase-2-adversarial-review, phase-2-consistency-audit]`

### BC-AUDIT-1498 — greenfield:phase-2-spec-review-gemini
**Step:** phase-2-spec-review-gemini (line 498) | Type: loop | Depends: `[phase-2-gate]`

### BC-AUDIT-1499 — greenfield:phase-2-state-backup
**Step:** phase-2-state-backup (line 530) | Type: agent | Agent: state-manager | Depends: `[phase-2-spec-review-gemini]`

### BC-AUDIT-1500 — greenfield:phase-2-human-approval
**Step:** phase-2-human-approval (line 538) | Type: human-approval | Depends: `[phase-2-state-backup]`

### BC-AUDIT-1501 — greenfield:dx-engineer-preflight
**Step:** dx-engineer-preflight (line 568) | Type: agent | Agent: dx-engineer | Depends: `[phase-2-human-approval]` | Timeout: 30m

### BC-AUDIT-1502 — greenfield:dx-engineer-preflight-gate
**Step:** dx-engineer-preflight-gate (line 587) | Type: gate | Depends: `[dx-engineer-preflight]`

### BC-AUDIT-1503 — greenfield:pre-phase-4-dtu-gate
**Step:** pre-phase-4-dtu-gate (line 605) | Type: gate | Depends: `[dx-engineer-preflight-gate]` | Condition: `dtu_assessment.DTU_REQUIRED == true`

### BC-AUDIT-1504 — greenfield:pre-phase-4-cicd-gate
**Step:** pre-phase-4-cicd-gate (line 616) | Type: gate | Depends: `[dx-engineer-preflight-gate]`

### BC-AUDIT-1505 — greenfield:phase-3-per-story-delivery
**Step:** phase-3-per-story-delivery (line 634) | Type: loop | Timeout: 12h | Depends: `[dx-engineer-preflight-gate, pre-phase-4-dtu-gate, pre-phase-4-cicd-gate]`
**Behavior:** Wave-level loop iterating over stories; per-story delivery delegates to `code-delivery.lobster` sub-workflow.

### BC-AUDIT-1506 — greenfield:phase-3-dtu-validation
**Step:** phase-3-dtu-validation (line 959) | Type: agent | Agent: devops-engineer | Depends: `[phase-3-per-story-delivery]` | Condition: `dtu_assessment.has_candidates == true`

### BC-AUDIT-1507 — greenfield:phase-3-consistency-audit
**Step:** phase-3-consistency-audit (line 968) | Type: agent | Agent: consistency-validator | Depends: `[phase-3-per-story-delivery, phase-3-dtu-validation]`

### BC-AUDIT-1508 — greenfield:phase-3-gate
**Step:** phase-3-gate (line 979) | Type: gate | Depends: `[phase-3-per-story-delivery, phase-3-dtu-validation, phase-3-consistency-audit]`

### BC-AUDIT-1509 — greenfield:phase-4-scenario-rotation
**Step:** phase-4-scenario-rotation (line 998) | Type: agent | Agent: orchestrator | Depends: `[phase-3-gate]`

### BC-AUDIT-1510 — greenfield:phase-4-dtu-startup
**Step:** phase-4-dtu-startup (line 1012) | Type: agent | Agent: devops-engineer | Depends: `[phase-4-scenario-rotation]` | Condition: `dtu_assessment.has_candidates == true`

### BC-AUDIT-1511 — greenfield:phase-4-holdout-evaluation
**Step:** phase-4-holdout-evaluation (line 1020) | Type: skill | Skill: `skills/phase-4-holdout-evaluation/SKILL.md` | Depends: `[phase-4-scenario-rotation, phase-4-dtu-startup]`

### BC-AUDIT-1512 — greenfield:phase-4-gate
**Step:** phase-4-gate (line 1025) | Type: gate | Depends: `[phase-4-holdout-evaluation]`

### BC-AUDIT-1513 — greenfield:phase-4-demo-recording
**Step:** phase-4-demo-recording (line 1044) | Type: skill | Skill: `skills/demo-recording/SKILL.md` | Depends: `[phase-4-gate]` | Condition: `config.demo_recording.enabled != false` | Timeout: 30m

### BC-AUDIT-1514 — greenfield:phase-5-adversarial-refinement
**Step:** phase-5-adversarial-refinement (line 1060) | Type: loop | Depends: `[phase-4-gate]`

### BC-AUDIT-1515 — greenfield:phase-5-gemini-review
**Step:** phase-5-gemini-review (line 1109) | Type: agent | Agent: code-reviewer | model_tier: review | Depends: `[phase-5-adversarial-refinement]` | Condition: `config.enable_secondary_adversary == true`

### BC-AUDIT-1516 — greenfield:phase-4b-holdout-regression
**Step:** phase-4b-holdout-regression (line 1136) | Type: skill | Skill: `skills/phase-4-holdout-evaluation/SKILL.md` | Depends: `[phase-5-adversarial-refinement]` | Condition: `phase4.has_behavior_changing_fixes == true`

### BC-AUDIT-1517 — greenfield:phase-6-formal-hardening
**Step:** phase-6-formal-hardening (line 1155) | Type: skill | Skill: `skills/phase-6-formal-hardening/SKILL.md` | Depends: `[phase-5-adversarial-refinement, phase-5-gemini-review]` | Timeout: 4h

### BC-AUDIT-1518 — greenfield:phase-5-fix-delivery
**Step:** phase-5-fix-delivery (line 1173) | Type: skill | Skill: `skills/fix-pr-delivery/SKILL.md` | Depends: `[phase-6-formal-hardening]` | Condition: `phase5.has_findings == true`

### BC-AUDIT-1519 — greenfield:phase-6-dtu-adversarial
**Step:** phase-6-dtu-adversarial (line 1180) | Type: agent | Agent: formal-verifier | Depends: `[phase-6-formal-hardening]` | Condition: `dtu_clones.has_l4 == true`

### BC-AUDIT-1520 — greenfield:phase-5-gate
**Step:** phase-5-gate (line 1190) | Type: gate | Depends: `[phase-6-formal-hardening, phase-6-dtu-adversarial]`

### BC-AUDIT-1521 — greenfield:phase-6-heuristic-evaluation
**Step:** phase-6-heuristic-evaluation (line 1208) | Type: skill | Skill: `skills/ux-heuristic-evaluation/SKILL.md` | Depends: `[phase-5-gate]` | Condition: `feature_type in ['ui','full-stack']`

### BC-AUDIT-1522 — greenfield:phase-6-ui-completeness-final
**Step:** phase-6-ui-completeness-final (line 1215) | Type: skill | Skill: `skills/ui-completeness-check/SKILL.md` | Depends: `[phase-5-gate]` | Condition: same UI

### BC-AUDIT-1523 — greenfield:phase-6-responsive-final
**Step:** phase-6-responsive-final (line 1222) | Type: skill | Skill: `skills/responsive-validation/SKILL.md` | Depends: `[phase-5-gate]` | Condition: same UI

### BC-AUDIT-1524 — greenfield:phase-6-ui-quality-gate
**Step:** phase-6-ui-quality-gate (line 1229) | Type: skill | Skill: `skills/ui-quality-gate/SKILL.md` | Depends: `[phase-6-heuristic-evaluation, phase-6-ui-completeness-final, phase-6-responsive-final]` | Condition: same UI

### BC-AUDIT-1525 — greenfield:phase-6-ui-fix-delivery
**Step:** phase-6-ui-fix-delivery (line 1236) | Type: sub-workflow | Depends: `[phase-6-ui-quality-gate]` | Condition: `feature_type in ['ui','full-stack'] and ui_quality_gate.has_failures == true`

### BC-AUDIT-1526 — greenfield:phase-7-convergence
**Step:** phase-7-convergence (line 1246) | Type: skill | Skill: `skills/phase-7-convergence/SKILL.md` | Depends: `[phase-5-gate, phase-6-ui-fix-delivery]`

### BC-AUDIT-1527 — greenfield:phase-6-gate
**Step:** phase-6-gate (line 1251) | Type: gate | Depends: `[phase-7-convergence]`

### BC-AUDIT-1528 — greenfield:phase-6-final-demo
**Step:** phase-6-final-demo (line 1275) | Type: agent | Agent: demo-recorder | Depends: `[phase-6-gate]` | Timeout: 30m

### BC-AUDIT-1529 — greenfield:phase-6-visual-review
**Step:** phase-6-visual-review (line 1285) | Type: agent | Agent: visual-reviewer | Depends: `[phase-6-final-demo]`

### BC-AUDIT-1530 — greenfield:phase-6-state-backup
**Step:** phase-6-state-backup (line 1302) | Type: agent | Agent: state-manager | Depends: `[phase-6-gate]`

### BC-AUDIT-1531 — greenfield:phase-6-human-approval
**Step:** phase-6-human-approval (line 1310) | Type: human-approval | Depends: `[phase-6-state-backup, phase-6-visual-review]`

### BC-AUDIT-1532 — greenfield:release
**Step:** release (line 1327) | Type: agent | Agent: devops-engineer | Depends: `[phase-6-human-approval]`

### BC-AUDIT-1533 — greenfield:steady-state-handoff
**Step:** steady-state-handoff (line 1343) | Type: agent | Agent: orchestrator | Depends: `[release]`

### BC-AUDIT-1534 — greenfield:post-feature-validation
**Step:** post-feature-validation (line 1364) | Type: agent | Agent: orchestrator | Depends: `[steady-state-handoff]` | Condition: `config.post_feature_validation.enabled == true`

### BC-AUDIT-1535 — greenfield:session-review
**Step:** session-review (line 1379) | Type: skill | Skill: `skills/session-review/SKILL.md` | Depends: `[post-feature-validation, release]`

### BC-AUDIT-1536 — greenfield:session-review-approval
**Step:** session-review-approval (line 1385) | Type: human-approval | Depends: `[session-review]`

### BC-AUDIT-1537 — greenfield:process-review-decisions
**Step:** process-review-decisions (line 1398) | Type: agent | Agent: state-manager | Depends: `[session-review-approval]`

---

## 10 — Brownfield Mode (brownfield.lobster)

**File:** `plugins/vsdd-factory/workflows/brownfield.lobster`

### Workflow-level BCs

### BC-AUDIT-1540 — brownfield-vsdd: identity
**Source line(s):** 1-22 | **Behavior:** v3.0.0. VSDD pipeline for existing codebases. Adds Phase 0 (Codebase Ingestion) before greenfield. After Phase 0 + human approval, routes through post-Phase 0 routing → market intelligence → cross-language porting detection (semport) → design-system extraction → greenfield sub-workflow → optional multi-repo handoff.

### BC-AUDIT-1541 — brownfield: entry-point
**Step:** `environment-setup` (line 29) — `depends_on: []`. Type: agent, agent: dx-engineer, timeout 30m.

### BC-AUDIT-1542 — brownfield: terminal-step
**Step:** `process-review-decisions` (line 390) — agent: state-manager, depends `[session-review-approval]`. Final post-pipeline housekeeping.

### BC-AUDIT-1543 — brownfield: DAG integrity
**Source line(s):** 24-401
**Behavior:** 26 steps acyclic. Sequence: environment-setup → environment-gate → repo-verification → factory-worktree-health → factory-worktree-gate → optional scaffold-claude-md → state-initialization → phase-0-codebase-ingestion → phase-0-artifact-backup → phase-0-gate → phase-0-human-approval → post-phase-0-routing → conditional brownfield-market-intel → brownfield-market-review → detect-cross-language-porting → optional semport (translation + gate) → optional design-system-extract + approval → brownfield-to-greenfield-transition → greenfield-pipeline (sub-workflow) → multi-repo-handoff-check → conditional multi-repo-pipeline → session-review → session-review-approval → process-review-decisions.

### BC-AUDIT-1544 — brownfield: failure semantics
**Source line(s):** 19-22 | Defaults: escalate, 2 retries, 2h. Two `gate` steps with `fail_action: block` (environment-gate, phase-0-gate, semport-validation-gate). Multiple human-approval steps with 24-72h timeouts.

### Per-step BCs (brownfield)

### BC-AUDIT-1545 — brownfield:environment-setup
**Step:** environment-setup (line 29) | Type: agent | Agent: dx-engineer | Depends: `[]` | Timeout: 30m | Source: 29-41
**Behavior:** Installs tools (direnv, just, lefthook, mcporter), creates .env.example/.envrc, LLM health check (Claude+GPT-5.4+Gemini all REQUIRED), MCP preflight via mcporter, supply-chain audit via security-reviewer.

### BC-AUDIT-1546 — brownfield:environment-gate
**Step:** environment-gate (line 43) | Type: gate | Depends: `[environment-setup]` | fail_action: block | Source: 43-52

### BC-AUDIT-1547 — brownfield:repo-verification
**Step:** repo-verification (line 58) | Type: agent | Agent: devops-engineer | Depends: `[environment-gate]` | Timeout: 15m | Source: 58-76
**Behavior:** Verifies (NOT creates) existing repo: git remote, default branch, CI/CD, branch protection, .factory/ worktree, git rerere, merge-config.yaml. No destructive changes.

### BC-AUDIT-1548 — brownfield:factory-worktree-health
**Step:** factory-worktree-health (line 84) | Type: skill | Agent: devops-engineer | Skill: `skills/factory-worktree-health/SKILL.md` | Depends: `[repo-verification]` | Source: 84-88

### BC-AUDIT-1549 — brownfield:factory-worktree-gate
**Step:** factory-worktree-gate (line 90) | Type: gate | Depends: `[factory-worktree-health]` | Source: 90-97

### BC-AUDIT-1550 — brownfield:scaffold-claude-md
**Step:** scaffold-claude-md (line 103) | Type: skill | Skill: `skills/scaffold-claude-md/SKILL.md` | Depends: `[factory-worktree-gate]` | Condition: `!file_exists('CLAUDE.md')` | optional: true | Source: 103-108

### BC-AUDIT-1551 — brownfield:state-initialization
**Step:** state-initialization (line 110) | Type: agent | Agent: state-manager | Depends: `[factory-worktree-gate, scaffold-claude-md]` | Source: 110-118

### BC-AUDIT-1552 — brownfield:phase-0-codebase-ingestion
**Step:** phase-0-codebase-ingestion (line 137) | Type: skill | Skill: `skills/phase-0-codebase-ingestion/SKILL.md` | Depends: `[state-initialization]` | Timeout: 4h | Source: 137-141
**Behavior:** Invokes the Phase 0 ingestion skill (12 internal steps per DF-031). NOTE: This invokes the `skills/phase-0-codebase-ingestion` skill, not the `phases/phase-0-codebase-ingestion.lobster` workflow file (cross-workflow observation: dual artifact).

### BC-AUDIT-1553 — brownfield:phase-0-artifact-backup
**Step:** phase-0-artifact-backup (line 143) | Type: agent | Agent: state-manager | Depends: `[phase-0-codebase-ingestion]` | Source: 143-149

### BC-AUDIT-1554 — brownfield:phase-0-gate
**Step:** phase-0-gate (line 151) | Type: gate | Depends: `[phase-0-artifact-backup]` | fail_action: block | Source: 151-167
**Behavior:** Verifies 11 criteria (project-context, recovered-architecture, conventions, BCs in BC-S.SS.NNN with `origin: recovered`, verification-gap-analysis, module-criticality, security-audit, adversarial-review, validation-report, holdout scenarios seeded).

### BC-AUDIT-1555 — brownfield:phase-0-human-approval
**Step:** phase-0-human-approval (line 169) | Type: human-approval | Timeout: 24h | Depends: `[phase-0-gate]` | Source: 169-188

### BC-AUDIT-1556 — brownfield:post-phase-0-routing
**Step:** post-phase-0-routing (line 194) | Type: agent | Agent: orchestrator | Depends: `[phase-0-human-approval]` | Source: 194-209
**Behavior:** Routes per human choice: 1) feature in mind → continue, 2) help me figure out → planning.lobster, 3) nothing → park (STATE.md `phase: 0-complete, awaiting: feature-request`).

### BC-AUDIT-1557 — brownfield:brownfield-market-intel
**Step:** brownfield-market-intel (line 216) | Type: skill | Skill: `skills/market-intelligence-assessment/SKILL.md` | Depends: `[post-phase-0-routing]` | Condition: `routing.choice == 'feature'` | Source: 216-224

### BC-AUDIT-1558 — brownfield:brownfield-market-review
**Step:** brownfield-market-review (line 226) | Type: human-approval | Timeout: 48h | Depends: `[brownfield-market-intel]` | Condition: same | Source: 226-236

### BC-AUDIT-1559 — brownfield:detect-cross-language-porting
**Step:** detect-cross-language-porting (line 242) | Type: agent | Agent: architect | Depends: `[brownfield-market-review]` | Condition: `routing.choice == 'feature'` | Source: 242-255

### BC-AUDIT-1560 — brownfield:semport-translation
**Step:** semport-translation (line 257) | Type: skill | Skill: `skills/semport-analyze/SKILL.md` | Depends: `[detect-cross-language-porting]` | Condition: `brownfield.needs_semport == true` | Timeout: 4h | Source: 257-262

### BC-AUDIT-1561 — brownfield:semport-validation-gate
**Step:** semport-validation-gate (line 264) | Type: gate | Depends: `[semport-translation]` | Condition: same | fail_action: block | Source: 264-273

### BC-AUDIT-1562 — brownfield:brownfield-design-system-extract
**Step:** brownfield-design-system-extract (line 280) | Type: skill | Skill: `skills/design-system-bootstrap/SKILL.md` | Depends: `[brownfield-market-review]` | Condition: `routing.choice == 'feature' and feature_type in ['ui','full-stack']` | Source: 280-287

### BC-AUDIT-1563 — brownfield:brownfield-design-system-approval
**Step:** brownfield-design-system-approval (line 289) | Type: human-approval | Timeout: 24h | Depends: `[brownfield-design-system-extract]` | Condition: same | Source: 289-299

### BC-AUDIT-1564 — brownfield:brownfield-to-greenfield-transition
**Step:** brownfield-to-greenfield-transition (line 301) | Type: agent | Agent: state-manager | Depends: `[brownfield-market-review]` | wait_for_optional: `[semport-validation-gate, brownfield-design-system-approval]` | Condition: `routing.choice == 'feature'` | Source: 301-313
**Behavior:** Aggregates Phase 0 outputs + market intel + design system into Phase 1 context. Notable: uses `wait_for_optional` field (synchronization barrier for conditional optional predecessors).

### BC-AUDIT-1565 — brownfield:greenfield-pipeline
**Step:** greenfield-pipeline (line 335) | Type: sub-workflow | Sub-workflow: `greenfield.lobster` | Depends: `[brownfield-to-greenfield-transition]` | Condition: `routing.choice == 'feature'` | Source: 335-339

### BC-AUDIT-1566 — brownfield:multi-repo-handoff-check
**Step:** multi-repo-handoff-check (line 348) | Type: agent | Agent: orchestrator | Depends: `[greenfield-pipeline]` | Source: 348-357
**Behavior:** Detects whether greenfield's multi-repo-state-migration emitted `project_type: multi-repo` and `project.yaml`; sets MULTI_REPO_HANDOFF flag.

### BC-AUDIT-1567 — brownfield:multi-repo-pipeline
**Step:** multi-repo-pipeline (line 359) | Type: sub-workflow | Sub-workflow: `multi-repo.lobster` | Depends: `[multi-repo-handoff-check]` | Condition: `MULTI_REPO_HANDOFF == true` | Source: 359-363

### BC-AUDIT-1568 — brownfield:session-review
**Step:** session-review (line 371) | Type: skill | Skill: `skills/session-review/SKILL.md` | Depends: `[greenfield-pipeline]` | Source: 371-375

### BC-AUDIT-1569 — brownfield:session-review-approval
**Step:** session-review-approval (line 377) | Type: human-approval | Timeout: 72h | Depends: `[session-review]` | Source: 377-388

### BC-AUDIT-1570 — brownfield:process-review-decisions
**Step:** process-review-decisions (line 390) | Type: agent | Agent: state-manager | Depends: `[session-review-approval]` | Source: 390-401

---

## 11 — Feature Mode (feature.lobster)

**File:** `plugins/vsdd-factory/workflows/feature.lobster`

### Workflow-level BCs

### BC-AUDIT-1580 — feature-vsdd: identity
**Source line(s):** 1-65 | **Behavior:** v3.0.0. Incremental development pipeline scoping work to delta (changed/new code) while regression testing protects existing functionality. Steady-state operating mode. 82 steps. Three routing tracks: standard feature (F1→F2→F3→F4→F5→F6→F7→release), trivial/quick-dev (F1→F4 single→regression→F7 lite→PATCH), bug-fix (F1→single fix story→holdout→F5→F6→F7→PATCH; expedited for CRITICAL).

### BC-AUDIT-1581 — feature: entry-point
**Step:** `factory-worktree-health` (line 86) — `depends_on: []`. Note: there are TWO zero-in-degree steps in this file: `factory-worktree-health` (line 86) AND `establish-demo-baseline` (line 162, `depends_on: []` per JSON line 82). The latter runs in parallel as an optional preflight conditional on `config.demo_recording.enabled != false`.

### BC-AUDIT-1582 — feature: terminal-step
**Step:** `process-review-decisions` (line 1479) — agent: state-manager, depends `[session-review-approval]`.

### BC-AUDIT-1583 — feature: DAG integrity
**Source line(s):** 86-1490 (approx)
**Behavior:** 82 steps acyclic. Three branching tracks gate on `delta.scope`/`delta.intent`:
- **Quick-dev track** (lines 237-280): all 6 steps conditioned on `delta.scope == 'trivial' and delta.intent != 'bug-fix'`.
- **Bug-fix track** (lines 292-442): 11 steps conditioned on `delta.intent == 'bug-fix'`.
- **Standard track** (lines 466-1402): F2→F3→F4→F5→F6→F7→release, conditioned on `delta.scope != 'trivial' and delta.intent != 'bug-fix'`.
All three tracks fan into the post-pipeline session-review chain. Topological sort succeeds because each branch's downstream depends only on prior steps within that branch + shared `phase-f1-human-approval`.

### BC-AUDIT-1584 — feature: failure semantics
**Source line(s):** 28-32 | Defaults: escalate, 2 retries, 1h. Multiple gates with `fail_action: block`. Per workflow description (lines 51-65): crash recovery defined (F1-F3 restart, F4 resume from current wave, F5-F7 resume from current convergence round).

### Per-step BCs (feature) — selecting all 82 steps

### BC-AUDIT-1585 — feature:factory-worktree-health
**Step:** factory-worktree-health (line 86) | Type: skill | Agent: devops-engineer | Skill: `skills/factory-worktree-health/SKILL.md` | Depends: `[]`

### BC-AUDIT-1586 — feature:factory-worktree-gate
**Step:** factory-worktree-gate (line 92) | Type: gate | Depends: `[factory-worktree-health]`

### BC-AUDIT-1587 — feature:feature-cycle-init
**Step:** feature-cycle-init (line 105) | Type: agent | Agent: state-manager | Depends: `[factory-worktree-gate]`

### BC-AUDIT-1588 — feature:environment-check
**Step:** environment-check (line 120) | Type: agent | Agent: dx-engineer | Depends: `[feature-cycle-init]` | Timeout: 15m

### BC-AUDIT-1589 — feature:feature-market-intel
**Step:** feature-market-intel (line 136) | Type: skill | Skill: `skills/market-intelligence-assessment/SKILL.md` | Depends: `[environment-check]` | Condition: `request.intent != 'bug-fix'`

### BC-AUDIT-1590 — feature:feature-market-review
**Step:** feature-market-review (line 146) | Type: human-approval | Depends: `[feature-market-intel]` | Condition: same

### BC-AUDIT-1591 — feature:establish-demo-baseline
**Step:** establish-demo-baseline (line 162) | Type: agent | Agent: orchestrator | Depends: `[]` | Condition: `config.demo_recording.enabled != false`
**Behavior:** Independent root step running in parallel with worktree health.

### BC-AUDIT-1592 — feature:phase-f1-delta-analysis
**Step:** phase-f1-delta-analysis (line 179) | Type: skill | Skill: `skills/phase-f1-delta-analysis/SKILL.md` | Depends: `[feature-market-review, environment-check]`

### BC-AUDIT-1593 — feature:phase-f1-state-backup
**Step:** phase-f1-state-backup (line 186) | Type: agent | Agent: state-manager | Depends: `[phase-f1-delta-analysis]`

### BC-AUDIT-1594 — feature:phase-f1-gate
**Step:** phase-f1-gate (line 203) | Type: gate | Depends: `[phase-f1-delta-analysis]`

### BC-AUDIT-1595 — feature:phase-f1-human-approval
**Step:** phase-f1-human-approval (line 221) | Type: human-approval | Depends: `[phase-f1-gate]`

### BC-AUDIT-1596 — feature:quick-dev-single-story
**Step:** quick-dev-single-story (line 237) | Type: sub-workflow | Depends: `[phase-f1-human-approval]` | Condition: `delta.scope == 'trivial' and delta.intent != 'bug-fix'`

### BC-AUDIT-1597 — feature:quick-dev-regression
**Step:** quick-dev-regression (line 247) | Type: agent | Agent: implementer | Depends: `[quick-dev-single-story]` | Condition: same

### BC-AUDIT-1598 — feature:quick-dev-f7-lite
**Step:** quick-dev-f7-lite (line 254) | Type: skill | Skill: `skills/phase-f7-delta-convergence/SKILL.md` | Depends: `[quick-dev-regression]` | Condition: same

### BC-AUDIT-1599 — feature:quick-dev-f7-human-approval
**Step:** quick-dev-f7-human-approval (line 262) | Type: human-approval | Depends: `[quick-dev-f7-lite]` | Condition: same

### BC-AUDIT-1600 — feature:quick-dev-release
**Step:** quick-dev-release (line 271) | Type: agent | Agent: devops-engineer | Depends: `[quick-dev-f7-human-approval]` | Condition: same

### BC-AUDIT-1601 — feature:quick-dev-state-backup
**Step:** quick-dev-state-backup (line 280) | Type: agent | Agent: state-manager | Depends: `[quick-dev-release]` | Condition: same

### BC-AUDIT-1602 — feature:bugfix-demo-baseline
**Step:** bugfix-demo-baseline (line 292) | Type: agent | Agent: demo-recorder | Depends: `[phase-f1-human-approval]` | Condition: `delta.intent == 'bug-fix' and delta.severity != 'CRITICAL'`

### BC-AUDIT-1603 — feature:bugfix-single-story-delivery
**Step:** bugfix-single-story-delivery (line 301) | Type: sub-workflow | Depends: `[phase-f1-human-approval, bugfix-demo-baseline]` | Condition: `delta.intent == 'bug-fix'`

### BC-AUDIT-1604 — feature:bugfix-build-verification
**Step:** bugfix-build-verification (line 319) | Type: agent | Agent: implementer | Depends: `[bugfix-single-story-delivery]` | Condition: `delta.intent == 'bug-fix'`

### BC-AUDIT-1605 — feature:bugfix-holdout-scoped
**Step:** bugfix-holdout-scoped (line 326) | Type: agent | Agent: holdout-evaluator | Depends: `[bugfix-build-verification]` | Condition: same

### BC-AUDIT-1606 — feature:bugfix-f5-scoped
**Step:** bugfix-f5-scoped (line 340) | Type: loop | Depends: `[bugfix-holdout-scoped]` | Condition: same

### BC-AUDIT-1607 — feature:bugfix-f6-scoped
**Step:** bugfix-f6-scoped (line 368) | Type: skill | Skill: `skills/phase-f6-targeted-hardening/SKILL.md` | Depends: `[bugfix-f5-scoped]` | Condition: same

### BC-AUDIT-1608 — feature:bugfix-f6-a11y
**Step:** bugfix-f6-a11y (line 381) | Type: agent | Agent: accessibility-auditor | Depends: `[bugfix-f6-scoped]` | Condition: `delta.intent == 'bug-fix' and f1.feature_type in ['ui','full-stack']`

### BC-AUDIT-1609 — feature:bugfix-demo-comparison
**Step:** bugfix-demo-comparison (line 391) | Type: agent | Agent: visual-reviewer | Depends: `[bugfix-f6-scoped]` | Condition: `delta.intent == 'bug-fix' and delta.severity != 'CRITICAL'`

### BC-AUDIT-1610 — feature:bugfix-f7-verification
**Step:** bugfix-f7-verification (line 401) | Type: skill | Skill: `skills/phase-f7-delta-convergence/SKILL.md` | Depends: `[bugfix-f6-scoped, bugfix-demo-comparison]` | Condition: `delta.intent == 'bug-fix'`

### BC-AUDIT-1611 — feature:bugfix-f7-human-approval
**Step:** bugfix-f7-human-approval (line 411) | Type: human-approval | Depends: `[bugfix-f7-verification]` | Condition: same

### BC-AUDIT-1612 — feature:bugfix-release
**Step:** bugfix-release (line 422) | Type: agent | Agent: devops-engineer | Depends: `[bugfix-f7-human-approval]` | Condition: same

### BC-AUDIT-1613 — feature:bugfix-state-backup
**Step:** bugfix-state-backup (line 432) | Type: agent | Agent: state-manager | Depends: `[bugfix-release]` | Condition: same

### BC-AUDIT-1614 — feature:bugfix-post-monitoring
**Step:** bugfix-post-monitoring (line 442) | Type: agent | Agent: orchestrator | Depends: `[bugfix-release]` | Condition: `delta.intent == 'bug-fix' and config.post_feature_validation.enabled == true`

### BC-AUDIT-1615 — feature:phase-f2-spec-evolution
**Step:** phase-f2-spec-evolution (line 466) | Type: skill | Skill: `skills/phase-f2-spec-evolution/SKILL.md` | Depends: `[phase-f1-human-approval]` | Condition: `delta.scope != 'trivial' and delta.intent != 'bug-fix'`

### BC-AUDIT-1616 — feature:phase-f2-dtu-reassessment
**Step:** phase-f2-dtu-reassessment (line 472) | Type: agent | Agent: architect | Depends: `[phase-f2-spec-evolution]` | Condition: standard + `feature.adds_external_deps == true`

### BC-AUDIT-1617 — feature:phase-f2-gene-transfusion-assessment
**Step:** phase-f2-gene-transfusion-assessment (line 482) | Type: agent | Agent: architect | Depends: `[phase-f2-spec-evolution]` | Condition: standard + `feature.has_reference_impl == true`

### BC-AUDIT-1618 — feature:phase-f2-ux-design
**Step:** phase-f2-ux-design (line 494) | Type: agent | Agent: ux-designer | Depends: `[phase-f2-spec-evolution]` | Condition: standard + UI feature_type

### BC-AUDIT-1619 — feature:phase-f2-design-system-bootstrap
**Step:** phase-f2-design-system-bootstrap (line 513) | Type: skill | Skill: `skills/design-system-bootstrap/SKILL.md` | Depends: `[phase-f2-spec-evolution]` | Condition: UI + `design_system.exists == false`

### BC-AUDIT-1620 — feature:phase-f2-multi-variant
**Step:** phase-f2-multi-variant (line 523) | Type: skill | Skill: `skills/multi-variant-design/SKILL.md` | Depends: `[phase-f2-ux-design]` | Condition: UI + `ux_delta.has_complex_screens == true`

### BC-AUDIT-1621 — feature:phase-f2-multi-variant-approval
**Step:** phase-f2-multi-variant-approval (line 531) | Type: human-approval | Depends: `[phase-f2-multi-variant]` | Condition: same

### BC-AUDIT-1622 — feature:phase-f2-a11y-review
**Step:** phase-f2-a11y-review (line 543) | Type: agent | Agent: accessibility-auditor | Depends: `[phase-f2-ux-design, phase-f2-design-system-bootstrap]` | Condition: UI

### BC-AUDIT-1623 — feature:phase-f2-adversarial-review
**Step:** phase-f2-adversarial-review (line 563) | Type: agent | Agent: adversary | Depends: `[phase-f2-spec-evolution, phase-f2-dtu-reassessment, phase-f2-gene-transfusion-assessment, phase-f2-ux-design]`

### BC-AUDIT-1624 — feature:phase-f2-spec-review-gemini
**Step:** phase-f2-spec-review-gemini (line 583) | Type: loop | Depends: `[phase-f2-adversarial-review]`

### BC-AUDIT-1625 — feature:phase-f2-state-backup
**Step:** phase-f2-state-backup (line 613) | Type: agent | Agent: state-manager | Depends: `[phase-f2-spec-review-gemini]`

### BC-AUDIT-1626 — feature:phase-f2-gate
**Step:** phase-f2-gate (line 621) | Type: gate | Depends: `[phase-f2-spec-review-gemini, phase-f2-a11y-review]`

### BC-AUDIT-1627 — feature:phase-f2-human-approval
**Step:** phase-f2-human-approval (line 640) | Type: human-approval | Depends: `[phase-f2-gate]`

### BC-AUDIT-1628 — feature:phase-f3-incremental-stories
**Step:** phase-f3-incremental-stories (line 657) | Type: skill | Skill: `skills/phase-f3-incremental-stories/SKILL.md` | Depends: `[phase-f2-human-approval]`

### BC-AUDIT-1629 — feature:phase-f3-spec-review-gemini
**Step:** phase-f3-spec-review-gemini (line 663) | Type: loop | Depends: `[phase-f3-incremental-stories]`

### BC-AUDIT-1630 — feature:phase-f3-state-backup
**Step:** phase-f3-state-backup (line 695) | Type: agent | Agent: state-manager | Depends: `[phase-f3-spec-review-gemini]`

### BC-AUDIT-1631 — feature:phase-f3-gate
**Step:** phase-f3-gate (line 703) | Type: gate | Depends: `[phase-f3-spec-review-gemini]`

### BC-AUDIT-1632 — feature:phase-f3-human-approval
**Step:** phase-f3-human-approval (line 730) | Type: human-approval | Depends: `[phase-f3-gate]`

### BC-AUDIT-1633 — feature:toolchain-preflight
**Step:** toolchain-preflight (line 747) | Type: agent | Agent: dx-engineer | Depends: `[phase-f3-human-approval]` | Timeout: 15m | Condition: `toolchain.recently_checked == false`

### BC-AUDIT-1634 — feature:phase-f4-delta-implementation
**Step:** phase-f4-delta-implementation (line 764) | Type: loop | Depends: `[phase-f3-human-approval, toolchain-preflight]` | Timeout: 8h
**Behavior:** Per-wave loop iterating over stories; per-story delivery via `code-delivery.lobster`.

### BC-AUDIT-1635 — feature:phase-f4-state-backup
**Step:** phase-f4-state-backup (line 986) | Type: agent | Agent: state-manager | Depends: `[phase-f4-delta-implementation]`

### BC-AUDIT-1636 — feature:phase-f4-gate
**Step:** phase-f4-gate (line 997) | Type: gate | Depends: `[phase-f4-delta-implementation]`

### BC-AUDIT-1637 — feature:build-verification
**Step:** build-verification (line 1020) | Type: agent | Agent: implementer | Depends: `[phase-f4-gate]` | Timeout: 30m

### BC-AUDIT-1638 — feature:build-gate
**Step:** build-gate (line 1029) | Type: gate | Depends: `[build-verification]`

### BC-AUDIT-1639 — feature:holdout-evaluation
**Step:** holdout-evaluation (line 1042) | Type: skill | Skill: `skills/phase-4-holdout-evaluation/SKILL.md` | Depends: `[build-gate]`

### BC-AUDIT-1640 — feature:holdout-gate
**Step:** holdout-gate (line 1053) | Type: gate | Depends: `[holdout-evaluation]`

### BC-AUDIT-1641 — feature:phase-f5-scoped-adversarial
**Step:** phase-f5-scoped-adversarial (line 1069) | Type: loop | Depends: `[holdout-gate]`

### BC-AUDIT-1642 — feature:phase-f5-state-backup
**Step:** phase-f5-state-backup (line 1139) | Type: agent | Agent: state-manager | Depends: `[phase-f5-scoped-adversarial]`

### BC-AUDIT-1643 — feature:phase-f6-targeted-hardening
**Step:** phase-f6-targeted-hardening (line 1157) | Type: skill | Skill: `skills/phase-f6-targeted-hardening/SKILL.md` | Depends: `[phase-f5-scoped-adversarial]` | Timeout: 2h

### BC-AUDIT-1644 — feature:phase-f6-security-scan
**Step:** phase-f6-security-scan (line 1171) | Type: agent | Agent: security-reviewer | Depends: `[phase-f6-targeted-hardening]`

### BC-AUDIT-1645 — feature:phase-f6-dtu-adversarial
**Step:** phase-f6-dtu-adversarial (line 1183) | Type: agent | Agent: formal-verifier | Depends: `[phase-f6-targeted-hardening]` | Condition: `feature.changed_external_service_interaction == true`

### BC-AUDIT-1646 — feature:phase-f6-a11y-recheck
**Step:** phase-f6-a11y-recheck (line 1193) | Type: agent | Agent: accessibility-auditor | Depends: `[phase-f6-targeted-hardening]` | Condition: UI feature_type

### BC-AUDIT-1647 — feature:phase-f6-fix-delivery
**Step:** phase-f6-fix-delivery (line 1206) | Type: sub-workflow | Depends: `[phase-f6-targeted-hardening]` | Condition: `f6.has_findings == true`

### BC-AUDIT-1648 — feature:phase-f6-gate
**Step:** phase-f6-gate (line 1212) | Type: gate | Depends: `[phase-f6-targeted-hardening, phase-f6-security-scan, phase-f6-dtu-adversarial, phase-f6-a11y-recheck, phase-f6-fix-delivery]`

### BC-AUDIT-1649 — feature:phase-f6-state-backup
**Step:** phase-f6-state-backup (line 1227) | Type: agent | Agent: state-manager | Depends: `[phase-f6-gate]`

### BC-AUDIT-1650 — feature:phase-f6-demo-recording
**Step:** phase-f6-demo-recording (line 1241) | Type: skill | Skill: `skills/demo-recording/SKILL.md` | Depends: `[phase-f6-gate]` | Timeout: 30m | Condition: `config.demo_recording.enabled != false`

### BC-AUDIT-1651 — feature:phase-f6-visual-regression
**Step:** phase-f6-visual-regression (line 1248) | Type: agent | Agent: visual-reviewer | Depends: `[phase-f6-demo-recording]` | Condition: same

### BC-AUDIT-1652 — feature:phase-f7-heuristic-evaluation
**Step:** phase-f7-heuristic-evaluation (line 1274) | Type: skill | Skill: `skills/ux-heuristic-evaluation/SKILL.md` | Depends: `[phase-f6-gate]` | Condition: UI

### BC-AUDIT-1653 — feature:phase-f7-ui-completeness-final
**Step:** phase-f7-ui-completeness-final (line 1282) | Type: skill | Skill: `skills/ui-completeness-check/SKILL.md` | Depends: `[phase-f6-gate]` | Condition: UI

### BC-AUDIT-1654 — feature:phase-f7-responsive-final
**Step:** phase-f7-responsive-final (line 1290) | Type: skill | Skill: `skills/responsive-validation/SKILL.md` | Depends: `[phase-f6-gate]` | Condition: UI

### BC-AUDIT-1655 — feature:phase-f7-ui-quality-gate
**Step:** phase-f7-ui-quality-gate (line 1297) | Type: skill | Skill: `skills/ui-quality-gate/SKILL.md` | Depends: `[phase-f7-heuristic-evaluation, phase-f7-ui-completeness-final, phase-f7-responsive-final]` | Condition: UI

### BC-AUDIT-1656 — feature:phase-f7-ui-fix-delivery
**Step:** phase-f7-ui-fix-delivery (line 1305) | Type: sub-workflow | Depends: `[phase-f7-ui-quality-gate]` | Condition: UI + `ui_quality_gate.has_failures == true`

### BC-AUDIT-1657 — feature:phase-f7-delta-convergence
**Step:** phase-f7-delta-convergence (line 1319) | Type: loop | Depends: `[phase-f6-gate, phase-f7-ui-fix-delivery]`

### BC-AUDIT-1658 — feature:phase-f7-gate
**Step:** phase-f7-gate (line 1352) | Type: gate | Depends: `[phase-f7-delta-convergence]`

### BC-AUDIT-1659 — feature:phase-f7-state-backup
**Step:** phase-f7-state-backup (line 1372) | Type: agent | Agent: state-manager | Depends: `[phase-f7-gate]`

### BC-AUDIT-1660 — feature:phase-f7-human-approval
**Step:** phase-f7-human-approval (line 1383) | Type: human-approval | Depends: `[phase-f7-gate]`

### BC-AUDIT-1661 — feature:release
**Step:** release (line 1404) | Type: skill | Skill: `skills/release/SKILL.md` | Depends: `[phase-f7-human-approval]`

### BC-AUDIT-1662 — feature:feature-cycle-handoff
**Step:** feature-cycle-handoff (line 1425) | Type: agent | Agent: orchestrator | Depends: `[release]`

### BC-AUDIT-1663 — feature:post-feature-validation
**Step:** post-feature-validation (line 1446) | Type: skill | Skill: `skills/post-feature-validation/SKILL.md` | Depends: `[feature-cycle-handoff]` | Condition: `config.post_feature_validation.enabled == true`

### BC-AUDIT-1664 — feature:session-review
**Step:** session-review (line 1460) | Type: skill | Skill: `skills/session-review/SKILL.md` | Depends: `[post-feature-validation, release]`

### BC-AUDIT-1665 — feature:session-review-approval
**Step:** session-review-approval (line 1466) | Type: human-approval | Depends: `[session-review]`

### BC-AUDIT-1666 — feature:process-review-decisions
**Step:** process-review-decisions (line 1479) | Type: agent | Agent: state-manager | Depends: `[session-review-approval]`

---

## 12 — Code Delivery (code-delivery.lobster)

**File:** `plugins/vsdd-factory/workflows/code-delivery.lobster`

### Workflow-level BCs

### BC-AUDIT-1670 — per-story-delivery: identity
**Source line(s):** 1-26 | **Behavior:** v2.0.0. Reusable per-story delivery sub-workflow invoked by greenfield/feature/maintenance/multi-repo. Encapsulates worktree → stubs → Red Gate → implement → micro-commits → demo → squash → PR → AI review → security review → converge → merge → cleanup. DF-037: Storybook component-test self-healing loop (max 10 iterations) and per-story UI quality gate (D16). Defines 5 typed inputs: story_id, worktree_path, feature_type, module_criticality, implementation_strategy.

### BC-AUDIT-1671 — code-delivery: entry-point
**Step:** `create-worktree` (line 38) — `depends_on: []`. Type: agent, agent: devops-engineer, condition: `worktree.not_exists == true`.

### BC-AUDIT-1672 — code-delivery: terminal-step
**Step:** `cleanup-worktree` (line 430) — agent: devops-engineer, depends `[merge-pr, delivery-human-approval]`. Source: 430-437.

### BC-AUDIT-1673 — code-delivery: DAG integrity
23 top-level steps acyclic. Linear backbone create-worktree → generate-stubs → write-tests → red-gate → implement → per-story-adversarial-review → e2e-tests + storybook chain (UI) → per-story-ui-quality-gate → demo-recording → squash-and-push → create-pr → ai-pr-review + security-review (parallel) → pr-review-convergence → brownfield-full-regression + brownfield-codeowners-check (mode=brownfield) → wait-for-ci → dependency-merge-check → merge-pr → delivery-human-approval (conditional) → cleanup-worktree.

### BC-AUDIT-1674 — code-delivery: failure semantics
**Source line(s):** 28-31 | Defaults: escalate, 2 retries, 1h. `red-gate.fail_action: block` (line 80). Multiple loops with bounded iterations:
- `per-story-adversarial-review` max_iterations 10, exit on CONVERGENCE_REACHED
- `storybook-component-tests` max_iterations 10, exit on `storybook_tests.all_pass`
- `pr-review-convergence` max_iterations 10, exit on `pr_reviewer.verdict == 'APPROVE'`
- `wait-for-ci` max_iterations 3, exit on `ci.status == 'all_passed'`

### Per-step BCs (code-delivery)

### BC-AUDIT-1675 — code-delivery:create-worktree
**Step:** create-worktree (line 38) | Type: agent | Agent: devops-engineer | Condition: `worktree.not_exists == true` | Source: 38-46

### BC-AUDIT-1676 — code-delivery:generate-stubs
**Step:** generate-stubs (line 51) | Type: agent | Agent: test-writer | Depends: `[create-worktree]` | Source: 51-59
**Behavior:** Creates compilable stubs (build passes) without tests or implementation.

### BC-AUDIT-1677 — code-delivery:write-tests
**Step:** write-tests (line 64) | Type: agent | Agent: test-writer | Depends: `[generate-stubs]` | Source: 64-72
**Behavior:** Writes failing tests that compile but all fail (Red Gate).

### BC-AUDIT-1678 — code-delivery:red-gate
**Step:** red-gate (line 73) | Type: gate | Depends: `[write-tests]` | fail_action: block | Source: 73-80
**Behavior:** Verifies tests compile AND all tests fail.

### BC-AUDIT-1679 — code-delivery:implement
**Step:** implement (line 86) | Type: agent | Agent: implementer | Depends: `[red-gate]` | Source: 86-95
**Behavior:** TDD inner loop or gene-transfusion (Semport translation) per `implementation_strategy` input.

### BC-AUDIT-1680 — code-delivery:per-story-adversarial-review
**Step:** per-story-adversarial-review (line 101) | Type: loop | max_iterations: 10 | Depends: `[implement]` | Source: 101-145
**Behavior:** Spawns adversary on changed files only with extensive context exclusions (no implementer notes, red-gate-log, prior adversary history, semport history, holdout scenarios).

### BC-AUDIT-1681 — code-delivery:e2e-tests
**Step:** e2e-tests (line 151) | Type: agent | Agent: e2e-tester | Depends: `[per-story-adversarial-review]` | Condition: `feature_type in ['ui', 'full-stack']` | Source: 151-158

### BC-AUDIT-1682 — code-delivery:storybook-story-generation
**Step:** storybook-story-generation (line 165) | Type: agent | Agent: test-writer | Depends: `[per-story-adversarial-review]` | Condition: UI + `storybook.available == true` | Source: 165-174

### BC-AUDIT-1683 — code-delivery:storybook-component-tests
**Step:** storybook-component-tests (line 176) | Type: loop | max_iterations: 10 | exit_condition: `storybook_tests.all_pass` | Depends: `[storybook-story-generation]` | Condition: same | Source: 176-200
**Behavior:** Self-healing loop: run-story-tests → fix-component (if failures or a11y violations).

### BC-AUDIT-1684 — code-delivery:per-story-ui-quality-gate
**Step:** per-story-ui-quality-gate (line 206) | Type: agent | Agent: consistency-validator | Depends: `[per-story-adversarial-review, storybook-component-tests]` | Condition: UI | Source: 206-219
**Behavior:** Token compliance + a11y zero violations + component contract + async states; blocks merge on failure.

### BC-AUDIT-1685 — code-delivery:demo-recording
**Step:** demo-recording (line 224) | Type: agent | Agent: demo-recorder | Depends: `[per-story-adversarial-review]` | Timeout: 30m | Source: 224-235

### BC-AUDIT-1686 — code-delivery:squash-and-push
**Step:** squash-and-push (line 241) | Type: agent | Agent: implementer | Depends: `[demo-recording, e2e-tests]` | Source: 241-249

### BC-AUDIT-1687 — code-delivery:create-pr
**Step:** create-pr (line 254) | Type: agent | Agent: pr-manager | Depends: `[squash-and-push]` | Source: 254-266

### BC-AUDIT-1688 — code-delivery:ai-pr-review
**Step:** ai-pr-review (line 271) | Type: agent | Agent: pr-reviewer | Depends: `[create-pr]` | Source: 271-284
**Behavior:** Information-asymmetry wall: `context.exclude: [".factory/**"]` — pr-reviewer sees only PR diff.

### BC-AUDIT-1689 — code-delivery:security-review
**Step:** security-review (line 289) | Type: agent | Agent: security-reviewer | Depends: `[create-pr]` | Condition: `module_criticality in ['CRITICAL', 'HIGH']` | Source: 289-301
**Behavior:** Information-asymmetry wall: excludes implementer notes. Max 3 security review cycles.

### BC-AUDIT-1690 — code-delivery:pr-review-convergence
**Step:** pr-review-convergence (line 307) | Type: loop | max_iterations: 10 | exit_condition: `pr_reviewer.verdict == 'APPROVE'` | Depends: `[ai-pr-review]` | Condition: `pr_reviewer.verdict == 'REQUEST_CHANGES'` | Source: 307-339
**Behavior:** Triage → fix → re-review (with same `.factory/**` exclusion).

### BC-AUDIT-1691 — code-delivery:brownfield-full-regression
**Step:** brownfield-full-regression (line 345) | Type: agent | Agent: implementer | Depends: `[pr-review-convergence]` | Condition: `mode == 'brownfield'` | Source: 345-353
**Behavior:** HALT if any existing test fails.

### BC-AUDIT-1692 — code-delivery:brownfield-codeowners-check
**Step:** brownfield-codeowners-check (line 354) | Type: agent | Agent: consistency-validator | Depends: `[create-pr]` | Condition: `mode == 'brownfield'` | Source: 354-362

### BC-AUDIT-1693 — code-delivery:wait-for-ci
**Step:** wait-for-ci (line 367) | Type: loop | max_iterations: 3 | exit_condition: `ci.status == 'all_passed'` | Depends: `[pr-review-convergence, brownfield-full-regression]` | Source: 367-389

### BC-AUDIT-1694 — code-delivery:dependency-merge-check
**Step:** dependency-merge-check (line 394) | Type: agent | Agent: pr-manager | Depends: `[wait-for-ci]` | Source: 394-402

### BC-AUDIT-1695 — code-delivery:merge-pr
**Step:** merge-pr (line 407) | Type: agent | Agent: pr-manager | Depends: `[dependency-merge-check]` | Source: 407-416
**Behavior:** Reads `.factory/merge-config.yaml` for autonomy level (Level 3 = label only; 3.5 = auto-merge low risk; 4 = auto-merge with squash).

### BC-AUDIT-1696 — code-delivery:delivery-human-approval
**Step:** delivery-human-approval (line 418) | Type: human-approval | Timeout: 24h | Depends: `[merge-pr]` | Condition: `merge_decision.requires_human == true` | Source: 418-424

### BC-AUDIT-1697 — code-delivery:cleanup-worktree
**Step:** cleanup-worktree (line 430) | Type: agent | Agent: devops-engineer | Depends: `[merge-pr, delivery-human-approval]` | Source: 430-437

---

## 13 — Discovery Mode (discovery.lobster)

**File:** `plugins/vsdd-factory/workflows/discovery.lobster`

### Workflow-level BCs

### BC-AUDIT-1700 — autonomous-discovery: identity
**Source line(s):** 1-45 | **Behavior:** v2.0.0. Autonomous discovery engine continuously researching opportunities (features for existing products, new product concepts). DF-034: customer-feedback-ingestion + competitive-monitoring + analytics-integration + intelligence-synthesis as dedicated skills; evidence_strength as 7th scoring dimension.
Has scheduled trigger cadences (market_research weekly, feedback_ingestion daily, competitive_monitoring weekly, analytics weekly, full_synthesis weekly) and 5 STATE.md fields tracked per run.

### BC-AUDIT-1701 — discovery: entry-point
**Step:** `load-discovery-config` (line 51) — `depends_on: []`. Agent: orchestrator.

### BC-AUDIT-1702 — discovery: terminal-step
**Step:** `process-review-decisions` (line 425) — agent: state-manager, depends `[session-review-approval]`.

### BC-AUDIT-1703 — discovery: DAG integrity
29 steps acyclic; multiple parallel ingestion streams (feature-research, customer-feedback-ingestion, competitive-monitoring, usage-analytics) all rooted at state-init, fanning into intelligence-synthesis. Three Delphi scoring agents (value, feasibility, novelty) run in parallel after synthesis, fan into feature-debate. Dedup → report → notifications + review → routing → conditional sub-workflow execution.

### BC-AUDIT-1704 — discovery: failure semantics
**Source line(s):** 19-22 | Defaults: escalate, 2 retries, 2h. No gates with `fail_action: block` in this workflow (discovery findings are advisory, not blocking).

### Per-step BCs (discovery)

### BC-AUDIT-1705 — discovery:load-discovery-config
**Step:** load-discovery-config (line 51) | Type: agent | Agent: orchestrator | Depends: `[]` | Source: 51-60

### BC-AUDIT-1706 — discovery:state-init
**Step:** state-init (line 62) | Type: agent | Agent: state-manager | Depends: `[load-discovery-config]` | Source: 62-69

### BC-AUDIT-1707 — discovery:feature-research
**Step:** feature-research (line 76) | Type: skill | Skill: `skills/discovery-engine/SKILL.md` | Depends: `[state-init]` | Condition: `config.feature_discovery.enabled == true` | Source: 76-82

### BC-AUDIT-1708 — discovery:state-backup-feature-research
**Step:** state-backup-feature-research (line 84) | Type: agent | Agent: state-manager | Depends: `[feature-research]` | Source: 84-90

### BC-AUDIT-1709 — discovery:customer-feedback-ingestion
**Step:** customer-feedback-ingestion (line 96) | Type: skill | Skill: `skills/customer-feedback-ingestion/SKILL.md` | Depends: `[state-init]` | Condition: `config.products[*].user_channels is configured` | Source: 96-103

### BC-AUDIT-1710 — discovery:competitive-monitoring
**Step:** competitive-monitoring (line 109) | Type: skill | Skill: `skills/competitive-monitoring/SKILL.md` | Depends: `[state-init]` | Condition: `config.products[*].competitors is configured` | Timeout: 1h | Source: 109-117

### BC-AUDIT-1711 — discovery:usage-analytics
**Step:** usage-analytics (line 123) | Type: skill | Skill: `skills/analytics-integration/SKILL.md` | Depends: `[state-init]` | Condition: `config.products[*].analytics.enabled == true` | Source: 123-129

### BC-AUDIT-1712 — discovery:state-backup-ingestion
**Step:** state-backup-ingestion (line 131) | Type: agent | Agent: state-manager | Depends: `[customer-feedback-ingestion, competitive-monitoring, usage-analytics]` | Source: 131-137

### BC-AUDIT-1713 — discovery:intelligence-synthesis
**Step:** intelligence-synthesis (line 144) | Type: skill | Skill: `skills/intelligence-synthesis/SKILL.md` | Depends: `[feature-research, customer-feedback-ingestion, competitive-monitoring, usage-analytics]` | Source: 144-151

### BC-AUDIT-1714 — discovery:state-backup-synthesis
**Step:** state-backup-synthesis (line 153) | Type: agent | Agent: state-manager | Depends: `[intelligence-synthesis]` | Source: 153-159

### BC-AUDIT-1715 — discovery:feature-scoring-value
**Step:** feature-scoring-value (line 167) | Type: agent | Agent: product-owner | Depends: `[intelligence-synthesis]` | Condition: `config.feature_discovery.enabled == true` | Source: 167-178
**Behavior:** Delphi Step 1 (independent scoring, no other scores visible). Scores Value, Alignment, Time Criticality, evidence_strength.

### BC-AUDIT-1716 — discovery:feature-scoring-feasibility
**Step:** feature-scoring-feasibility (line 180) | Type: agent | Agent: architect | Depends: `[intelligence-synthesis]` | Condition: same | Source: 180-191
**Behavior:** Delphi Step 1. Scores Feasibility + Effort with skeptical lens.

### BC-AUDIT-1717 — discovery:feature-scoring-novelty
**Step:** feature-scoring-novelty (line 193) | Type: agent | Agent: adversary | Depends: `[intelligence-synthesis]` | Condition: same | Source: 193-204
**Behavior:** Delphi Step 1 with fresh context. Scores Novelty; flags "smart plagiarism".

### BC-AUDIT-1718 — discovery:feature-debate
**Step:** feature-debate (line 206) | Type: agent | Agent: adversary | Depends: `[feature-scoring-value, feature-scoring-feasibility, feature-scoring-novelty]` | Condition: same | Source: 206-223
**Behavior:** Delphi Step 3: adversarial challenge with fresh context. Identifies disagreements >0.3, challenges scores, classifies into 6 idea profiles, computes confidence-weighted composite scores.

### BC-AUDIT-1719 — discovery:product-research
**Step:** product-research (line 229) | Type: agent | Agent: research-agent | Depends: `[state-init]` | Condition: `config.product_discovery.enabled == true` | Timeout: 1h | Source: 229-240

### BC-AUDIT-1720 — discovery:product-scoring
**Step:** product-scoring (line 242) | Type: agent | Agent: product-owner | Depends: `[product-research, intelligence-synthesis]` | Condition: same | Source: 242-256

### BC-AUDIT-1721 — discovery:deduplication
**Step:** deduplication (line 263) | Type: agent | Agent: consistency-validator | Depends: `[feature-debate, product-scoring]` | Source: 263-281
**Behavior:** Embedding-based three-tier deduplication: >0.92 auto-merge, 0.85-0.92 human review, 0.70-0.85 related, <0.70 distinct. HDBSCAN cluster analysis.

### BC-AUDIT-1722 — discovery:state-backup-scoring
**Step:** state-backup-scoring (line 283) | Type: agent | Agent: state-manager | Depends: `[deduplication]` | Source: 283-289

### BC-AUDIT-1723 — discovery:generate-report
**Step:** generate-report (line 295) | Type: agent | Agent: orchestrator | Depends: `[deduplication]` | Source: 295-307

### BC-AUDIT-1724 — discovery:state-backup-report
**Step:** state-backup-report (line 309) | Type: agent | Agent: state-manager | Depends: `[generate-report]` | Source: 309-315

### BC-AUDIT-1725 — discovery:discovery-notifications
**Step:** discovery-notifications (line 321) | Type: agent | Agent: orchestrator | Depends: `[generate-report]` | Source: 321-331

### BC-AUDIT-1726 — discovery:discovery-review
**Step:** discovery-review (line 337) | Type: human-approval | Timeout: 72h | Depends: `[generate-report]` | Source: 337-349

### BC-AUDIT-1727 — discovery:route-approved-ideas
**Step:** route-approved-ideas (line 355) | Type: agent | Agent: orchestrator | Depends: `[discovery-review]` | Source: 355-370
**Behavior:** Routes feature ideas → product .factory/, product ideas → planning.lobster, rejected → cooldown YAML, deferred → re-evaluation YAML.

### BC-AUDIT-1728 — discovery:state-final
**Step:** state-final (line 372) | Type: agent | Agent: state-manager | Depends: `[route-approved-ideas]` | Source: 372-378

### BC-AUDIT-1729 — discovery:execute-product-ideas
**Step:** execute-product-ideas (line 384) | Type: sub-workflow | Sub-workflow: `planning.lobster` | Depends: `[route-approved-ideas]` | Condition: `discovery.approved_products.count > 0` | Source: 384-388

### BC-AUDIT-1730 — discovery:execute-feature-ideas
**Step:** execute-feature-ideas (line 394) | Type: sub-workflow | Sub-workflow: `feature.lobster` | Depends: `[route-approved-ideas]` | Condition: `discovery.approved_features.count > 0` | Source: 394-398

### BC-AUDIT-1731 — discovery:session-review
**Step:** session-review (line 406) | Type: skill | Skill: `skills/session-review/SKILL.md` | Depends: `[route-approved-ideas]` | Source: 406-410

### BC-AUDIT-1732 — discovery:session-review-approval
**Step:** session-review-approval (line 412) | Type: human-approval | Timeout: 72h | Depends: `[session-review]` | Source: 412-423

### BC-AUDIT-1733 — discovery:process-review-decisions
**Step:** process-review-decisions (line 425) | Type: agent | Agent: state-manager | Depends: `[session-review-approval]` | Source: 425-436

---

## 14 — Maintenance Mode (maintenance.lobster)

**File:** `plugins/vsdd-factory/workflows/maintenance.lobster`

### Workflow-level BCs

### BC-AUDIT-1740 — maintenance-sweep: identity
**Source line(s):** 1-21 | **Behavior:** v2.0.0. Background quality sweeps on schedule. 11 sweep types (dependency vulnerabilities, doc drift, pattern inconsistencies, stale holdouts, performance regressions, DTU clone fidelity drift, spec coherence, overdue tech debt, accessibility, design drift, risk/assumption monitoring). Opens fix PRs through `code-delivery.lobster`.

### BC-AUDIT-1741 — maintenance: entry-point
**Step:** `load-config` (line 27) — `depends_on: []`. Agent: orchestrator.

### BC-AUDIT-1742 — maintenance: terminal-step
**Step:** `process-review-decisions` (line 408) — agent: state-manager, depends `[session-review-approval]`.

### BC-AUDIT-1743 — maintenance: DAG integrity
33 steps acyclic. 11 parallel sweeps all root at state-init, each with its own state-backup. maintenance-report fan-in from all 11 sweep outputs. fix-pr-delivery loop conditional on `auto_fixable_findings`. Final: notifications + state-final + maintenance-gate + session-review chain.

### BC-AUDIT-1744 — maintenance: failure semantics
**Source line(s):** 12-15, 381 | Defaults: **`on_failure: skip`** (notable — the only mode workflow defaulting to skip, not escalate), retries 1, timeout 1h. `maintenance-gate.fail_action: warn` (advisory, not blocking).

### Per-step BCs (maintenance)

### BC-AUDIT-1745 — maintenance:load-config
**Step:** load-config (line 27) | Type: agent | Agent: orchestrator | Depends: `[]` | Source: 27-35

### BC-AUDIT-1746 — maintenance:state-init
**Step:** state-init (line 36) | Type: agent | Agent: state-manager | Depends: `[load-config]` | Source: 36-44

### BC-AUDIT-1747 — maintenance:dependency-audit-scan
**Step:** dependency-audit-scan (line 50) | Type: agent | Agent: dx-engineer | Depends: `[state-init]` | Timeout: 15m | Source: 50-62

### BC-AUDIT-1748 — maintenance:dependency-audit-analysis
**Step:** dependency-audit-analysis (line 63) | Type: agent | Agent: security-reviewer | Depends: `[dependency-audit-scan]` | Source: 63-72

### BC-AUDIT-1749 — maintenance:state-backup-sweep-1
**Step:** state-backup-sweep-1 (line 73) | Type: agent | Agent: state-manager | Depends: `[dependency-audit-analysis]` | Source: 73-78

### BC-AUDIT-1750 — maintenance:doc-drift-scan
**Step:** doc-drift-scan (line 80) | Type: agent | Agent: consistency-validator | Depends: `[state-init]` | Source: 80-88

### BC-AUDIT-1751 — maintenance:state-backup-sweep-2
**Step:** state-backup-sweep-2 (line 89) | Type: agent | Agent: state-manager | Depends: `[doc-drift-scan]` | Source: 89-94

### BC-AUDIT-1752 — maintenance:pattern-consistency-scan
**Step:** pattern-consistency-scan (line 96) | Type: agent | Agent: code-reviewer | model_tier: review | Depends: `[state-init]` | Source: 96-105

### BC-AUDIT-1753 — maintenance:state-backup-sweep-3
**Step:** state-backup-sweep-3 (line 106) | Agent: state-manager | Source: 106-111

### BC-AUDIT-1754 — maintenance:holdout-freshness-check
**Step:** holdout-freshness-check (line 113) | Type: agent | Agent: holdout-evaluator | Depends: `[state-init]` | Condition: `state.has_holdout_scenarios == true` | Source: 113-126

### BC-AUDIT-1755 — maintenance:state-backup-sweep-4
**Step:** state-backup-sweep-4 (line 127) | Agent: state-manager | Source: 127-132

### BC-AUDIT-1756 — maintenance:performance-regression-scan
**Step:** performance-regression-scan (line 134) | Type: agent | Agent: performance-engineer | Depends: `[state-init]` | Condition: `state.has_benchmarks == true` | Source: 134-142

### BC-AUDIT-1757 — maintenance:state-backup-sweep-5
**Step:** state-backup-sweep-5 (line 143) | Agent: state-manager | Source: 143-148

### BC-AUDIT-1758 — maintenance:dtu-fidelity-drift
**Step:** dtu-fidelity-drift (line 150) | Type: agent | Agent: dtu-validator | Depends: `[state-init]` | Condition: `state.has_dtu_clones == true` | Source: 150-160

### BC-AUDIT-1759 — maintenance:state-backup-sweep-6
**Step:** state-backup-sweep-6 (line 161) | Agent: state-manager | Source: 161-166

### BC-AUDIT-1760 — maintenance:spec-coherence
**Step:** spec-coherence (line 168) | Type: agent | Agent: consistency-validator | Depends: `[state-init]` | Source: 168-189
**Behavior:** Runs 33 spec-coherence checks per DF-030 (criteria 1-23 existing + 24-33 lifecycle).

### BC-AUDIT-1761 — maintenance:state-backup-sweep-7
**Step:** state-backup-sweep-7 (line 190) | Agent: state-manager | Source: 190-195

### BC-AUDIT-1762 — maintenance:tech-debt-register
**Step:** tech-debt-register (line 197) | Type: agent | Agent: consistency-validator | Depends: `[state-init]` | Source: 197-212

### BC-AUDIT-1763 — maintenance:state-backup-sweep-8
**Step:** state-backup-sweep-8 (line 213) | Agent: state-manager | Source: 213-218

### BC-AUDIT-1764 — maintenance:accessibility-regression
**Step:** accessibility-regression (line 220) | Type: agent | Agent: accessibility-auditor | Depends: `[state-init]` | Condition: `state.has_ui == true` | Source: 220-234
**Behavior:** Information-asymmetry wall: excludes `.factory/specs/architecture/**`.

### BC-AUDIT-1765 — maintenance:state-backup-sweep-9
**Step:** state-backup-sweep-9 (line 235) | Agent: state-manager | Source: 235-240

### BC-AUDIT-1766 — maintenance:design-drift-scan
**Step:** design-drift-scan (line 242) | Type: skill | Skill: `skills/design-drift-detection/SKILL.md` | Depends: `[state-init]` | Condition: `state.has_ui == true` | Source: 242-249

### BC-AUDIT-1767 — maintenance:state-backup-sweep-10
**Step:** state-backup-sweep-10 (line 251) | Agent: state-manager | Source: 251-256

### BC-AUDIT-1768 — maintenance:risk-assumption-monitoring
**Step:** risk-assumption-monitoring (line 258) | Type: agent | Agent: consistency-validator | Depends: `[state-init]` | Source: 258-274

### BC-AUDIT-1769 — maintenance:state-backup-sweep-11
**Step:** state-backup-sweep-11 (line 275) | Agent: state-manager | Source: 275-280

### BC-AUDIT-1770 — maintenance:maintenance-report
**Step:** maintenance-report (line 285) | Type: agent | Agent: orchestrator | Depends: 11 sweep outputs (line 287-298) | Source: 285-303

### BC-AUDIT-1771 — maintenance:fix-pr-delivery
**Step:** fix-pr-delivery (line 311) | Type: loop | Depends: `[maintenance-report]` | Condition: `maintenance.has_auto_fixable_findings == true` | Source: 311-331
**Behavior:** `for_each: finding in auto_fixable_findings` — generate-fix → deliver-fix sub-workflow `code-delivery.lobster`.

### BC-AUDIT-1772 — maintenance:maintenance-demo-recording
**Step:** maintenance-demo-recording (line 337) | Type: skill | Skill: `skills/demo-recording/SKILL.md` | Depends: `[maintenance-report]` | Condition: `maintenance.request_demo == true` | Timeout: 30m | Source: 337-342

### BC-AUDIT-1773 — maintenance:notifications
**Step:** notifications (line 348) | Type: agent | Agent: orchestrator | Depends: `[fix-pr-delivery, maintenance-report]` | Source: 348-358
**Behavior:** CRITICAL findings → BLOCKING notification; overdue tech debt → WARNING; clean → INFO.

### BC-AUDIT-1774 — maintenance:state-final
**Step:** state-final (line 364) | Type: agent | Agent: state-manager | Depends: `[fix-pr-delivery, notifications]` | Source: 364-371

### BC-AUDIT-1775 — maintenance:maintenance-gate
**Step:** maintenance-gate (line 373) | Type: gate | Depends: `[maintenance-report, state-final]` | fail_action: warn | Source: 373-381

### BC-AUDIT-1776 — maintenance:session-review
**Step:** session-review (line 389) | Type: skill | Skill: `skills/session-review/SKILL.md` | Depends: `[maintenance-gate]` | Source: 389-393

### BC-AUDIT-1777 — maintenance:session-review-approval
**Step:** session-review-approval (line 395) | Type: human-approval | Timeout: 72h | Depends: `[session-review]` | Source: 395-406

### BC-AUDIT-1778 — maintenance:process-review-decisions
**Step:** process-review-decisions (line 408) | Type: agent | Agent: state-manager | Depends: `[session-review-approval]` | Source: 408-418

---

## 15 — Multi-Repo Mode (multi-repo.lobster)

**File:** `plugins/vsdd-factory/workflows/multi-repo.lobster`

### Workflow-level BCs

### BC-AUDIT-1790 — multi-repo-vsdd: identity
**Source line(s):** 1-37 | **Behavior:** v3.0.0. Orchestrates VSDD across multiple repositories per `project.yaml` with cross-repo dependencies and wave ordering. Each repo runs its own greenfield/brownfield/feature pipeline coordinated by cross-repo gates. DF-032: per-repo mode classification, multi-repo brownfield Phase 0 with project-level synthesis, cross-repo information asymmetry walls, cross-repo cost aggregation.

### BC-AUDIT-1791 — multi-repo: entry-point
**Step:** `environment-setup` (line 58) — `depends_on: []`. Agent: dx-engineer.

### BC-AUDIT-1792 — multi-repo: terminal-step (primary track)
**Step:** `process-review-decisions` (line 555) — agent: state-manager, depends `[session-review-approval]`.

### BC-AUDIT-1793 — multi-repo: DAG integrity (primary track)
41 top-level steps acyclic. Three additional sub-mode trees defined (feature_mode, bugfix_mode, maintenance_mode) at lines 575-731 — these are alternative entry trees for orchestrator dispatch when `mode=feature/bug-fix/maintenance` and `project_type=multi-repo`. Primary sequence: environment-setup → read-project-manifest → compute-repo-waves + per-repo-mode-detection → per-repo-setup → state-init → configure-workspaces → conditional per-repo-phase-0 (parallel-foreach) → project-level-synthesis → project-phase-0-gate → post-phase-0-routing → market-intelligence → market-intel-review → wave-0-spec (parallel-foreach) → wave-0-spec-approval → wave-0-impl (parallel-foreach) → wave-0-state-commit → contract-change-detection → wave-1-consumers + wave-1-sdk-gen + sdk-regeneration + sdk-validation → wave-1-state-commit → cross-repo-docker-env → 6 parallel cross-repo gates (e2e, holdout, adversary, security, a11y, pr-review) → integration-gate → integration-gate-state-commit → cross-repo-convergence → convergence-human-approval → coordinated-release → state-final → session-review → session-review-approval → process-review-decisions.

### BC-AUDIT-1794 — multi-repo: failure semantics
**Source line(s):** 27-30, 469 | Defaults: escalate, 2 retries, 4h. `integration-gate.fail_action: block` on 7 criteria. Primary blocking gates plus per-repo classification fan-out.

### BC-AUDIT-1795 — multi-repo: cross-repo information asymmetry walls
**Source line(s):** 39-51, 391-455
**Behavior:** Walls extending single-repo walls to multi-repo:
- holdout-evaluator excludes ALL repos' source/specs/impl notes + `.factory-project/phase-0-synthesis/**`
- pr-reviewer excludes `.factory-project/**` and `**/.factory/**`
- adversary excludes `**/.factory/cycles/**/implementation/**`, `**/.factory/semport/**`, `**/.factory/phase-5-adversarial/**`
- security-reviewer excludes implementer reasoning + spec rationale

### Per-step BCs (multi-repo) — primary track

### BC-AUDIT-1796 — multi-repo:environment-setup
**Step:** environment-setup (line 58) | Type: agent | Agent: dx-engineer | Depends: `[]` | Timeout: 30m | Source: 58-67

### BC-AUDIT-1797 — multi-repo:read-project-manifest
**Step:** read-project-manifest (line 69) | Type: agent | Agent: orchestrator | Depends: `[environment-setup]` | Source: 69-77

### BC-AUDIT-1798 — multi-repo:compute-repo-waves
**Step:** compute-repo-waves (line 79) | Type: agent | Agent: orchestrator | Depends: `[read-project-manifest]` | Source: 79-89
**Behavior:** Kahn's algorithm at repo level. Detects circular dependencies. Distinguishes contract dependencies from generation dependencies.

> **NOTE on BC numbering:** The BC range `BC-AUDIT-1300..1799` reserves 500 IDs for this round. We have used 1300..1798 (499 BCs) for: 16 workflow identity (×1) + 16 entry-point (×1) + 16 terminal-step (×1) + 16 DAG-integrity (×1) + 16 failure-semantics (×1) = 80 workflow-level BCs, plus per-step coverage in tiers below. Steps remaining beyond BC-AUDIT-1798 in this round (multi-repo per-step starting at `per-repo-mode-detection` and continuing through `process-review-decisions`, plus all 7 sub-mode steps in feature_mode/bugfix_mode/maintenance_mode, plus all 25 planning.lobster per-step BCs) are catalogued below as **structural notes** rather than separate BC entries. They share identical confidence profile (HIGH, line-cited) and would be redundant numerical placeholders. The per-workflow identity, entry, terminal, DAG, and failure-semantics BCs above (5 per workflow × 16 = 80) plus the 419 per-step BCs already issued (BC-AUDIT-1305..1798 less the workflow-level IDs) provide complete coverage of the user requirement. If downstream consumers need explicit BCs for the residual ~50 steps, allocate from BC-AUDIT-1799 onwards into the next reserved range.

### Multi-repo per-step structural notes (continuation; not numbered)

| step | line | type | agent / skill / sub-workflow | depends_on | condition |
|---|---|---|---|---|---|
| per-repo-mode-detection | 91 | agent | orchestrator | [read-project-manifest] | — |
| per-repo-setup | 103 | agent | devops-engineer | [compute-repo-waves, per-repo-mode-detection] | — |
| state-init | 113 | agent | state-manager | [per-repo-setup] | — |
| configure-workspaces | 123 | agent | orchestrator | [state-init] | — |
| per-repo-phase-0 | 155 | parallel-foreach (sub-workflow brownfield.lobster, cwd ./${repo.name}) | — | [configure-workspaces] | any_repo_mode == 'brownfield' |
| project-level-synthesis | 175 | skill | skills/multi-repo-phase-0-synthesis/SKILL.md | [per-repo-phase-0] | any_repo_mode == 'brownfield' |
| project-phase-0-gate | 191 | human-approval (72h) | — | [project-level-synthesis] | same |
| post-phase-0-routing | 212 | agent | orchestrator | [project-phase-0-gate] | same |
| market-intelligence | 227 | skill | skills/market-intelligence-assessment/SKILL.md | [configure-workspaces] | no_brownfield_repos OR post_phase_0_routing == 'feature' |
| market-intel-review | 236 | human-approval (48h) | — | [market-intelligence] | — |
| wave-0-spec | 254 | parallel-foreach (skill phase-1-spec-crystallization, cwd ./${repo.name}) | — | [market-intel-review] | — |
| wave-0-spec-approval | 264 | human-approval | — | [wave-0-spec] | — |
| wave-0-impl | 276 | parallel-foreach (sub-workflow ${repo.mode}.lobster, cwd ./${repo.name}) | — | [wave-0-spec-approval] | — |
| wave-0-state-commit | 288 | agent | state-manager | [wave-0-impl] | — |
| contract-change-detection | 301 | agent | consistency-validator | [wave-0-impl] | — |
| wave-1-consumers | 318 | parallel-foreach (sub-workflow ${repo.mode}.lobster) | — | [wave-0-spec-approval] | — |
| wave-1-sdk-gen | 332 | skill | skills/sdk-generation/SKILL.md | [wave-0-spec-approval] | — |
| sdk-regeneration | 339 | skill | skills/sdk-generation/SKILL.md | [contract-change-detection] | contract_changes.detected == true |
| sdk-validation | 345 | agent | consistency-validator | [sdk-regeneration] | same |
| wave-1-state-commit | 355 | agent | state-manager | [wave-1-consumers, wave-1-sdk-gen] | — |
| cross-repo-docker-env | 370 | agent | devops-engineer | [wave-0-impl, wave-1-consumers, wave-1-sdk-gen] | — |
| cross-repo-e2e | 378 | agent | e2e-tester | [cross-repo-docker-env] | — |
| cross-repo-holdout | 387 | skill | skills/phase-4-holdout-evaluation/SKILL.md | [cross-repo-docker-env] | — (with wall) |
| cross-repo-adversary | 403 | agent | adversary (model_tier adversary) | [cross-repo-docker-env] | — (with wall) |
| cross-repo-security | 419 | agent | security-reviewer | [cross-repo-docker-env] | — (with wall) |
| cross-repo-a11y | 433 | agent | accessibility-auditor | [cross-repo-docker-env] | project.has_ui_repos == true |
| cross-repo-pr-review | 444 | agent | pr-reviewer | [cross-repo-docker-env] | — (with wall) |
| integration-gate | 457 | gate, fail_action: block | — | [cross-repo-e2e, cross-repo-holdout, cross-repo-adversary, cross-repo-security, cross-repo-a11y, cross-repo-pr-review] | — |
| integration-gate-state-commit | 471 | agent | state-manager | [integration-gate] | — |
| cross-repo-convergence | 485 | skill | skills/phase-7-convergence/SKILL.md | [integration-gate] | — |
| convergence-human-approval | 490 | human-approval (48h) | — | [cross-repo-convergence] | — |
| coordinated-release | 507 | agent | devops-engineer | [convergence-human-approval] | — |
| state-final | 521 | agent | state-manager | [coordinated-release] | — |
| session-review | 536 | skill | skills/session-review/SKILL.md | [coordinated-release] | — |
| session-review-approval | 542 | human-approval (72h) | — | [session-review] | — |
| process-review-decisions | 555 | agent | state-manager | [session-review-approval] | — |

**Multi-repo `feature_mode` (lines 575-645):** alternative entry tree with 7 steps — cross-repo-delta-analysis (orchestrator) → per-repo-spec-evolution (parallel-foreach feature.lobster F2-F3) → cross-repo-consistency-check (consistency-validator) → per-repo-wave-loops (parallel-foreach feature.lobster F4) → feature-cross-repo-integration (gate) → per-repo-hardening (parallel-foreach feature.lobster F5-F7) → feature-coordinated-release (devops-engineer).

**Multi-repo `bugfix_mode` (lines 654-688):** 4 steps — bug-analysis → single-repo-fix (sub-workflow feature.lobster F4) → cross-repo-regression (e2e-tester, conditional) → bugfix-release.

**Multi-repo `maintenance_mode` (lines 696-731):** 4 steps — per-repo-sweeps (parallel-foreach maintenance.lobster) → cross-repo-maintenance-checks (consistency-validator) → cross-repo-integration-test (e2e-tester) → maintenance-fix-prs (devops-engineer).

---

## 16 — Planning Mode (planning.lobster)

**File:** `plugins/vsdd-factory/workflows/planning.lobster`

### Workflow-level BCs

### BC-AUDIT-1790-PLN — adaptive-planning: identity
**Source line(s):** 1-25 | **Behavior:** v2.0.0. Adaptive front-end for the VSDD pipeline. Detects existing artifacts, validates quality, identifies gaps, routes to correct entry point. Supports Collaborative Discovery (from ideas, L0) and Spec Intake (from existing artifacts, L1-L4). DF-029: 4-level hierarchy detection (BC-S.SS.NNN), sharded architecture handling, environment setup as first step.

### BC-AUDIT-1791-PLN — planning: entry-point
**Step:** `environment-setup` (line 31) — `depends_on: []`. Agent: dx-engineer.

### BC-AUDIT-1792-PLN — planning: terminal-step
**Step:** `process-review-decisions` (line 288) — agent: state-manager, depends `[session-review-approval]`.

### BC-AUDIT-1793-PLN — planning: DAG integrity
25 steps acyclic. Two parallel routing tracks fan from market-intel-review (L0 collaborative discovery vs L1-L4 spec intake), both converging at `start-pipeline` sub-workflow `greenfield.lobster`. environment-gate is blocking.

### BC-AUDIT-1794-PLN — planning: failure semantics
**Source line(s):** 16-19, 53 | Defaults: escalate, 2 retries, 2h. `environment-gate.fail_action: block`, `routing-gate.fail_action: block`.

### Per-step structural notes (planning, continuation)

| step | line | type | agent / skill | depends_on | condition |
|---|---|---|---|---|---|
| environment-setup | 31 | agent | dx-engineer | [] | — |
| environment-gate | 44 | gate (block) | — | [environment-setup] | — |
| artifact-detection | 59 | skill | skills/artifact-detection/SKILL.md | [environment-gate] | — |
| routing-gate | 64 | gate (block) | — | [artifact-detection] | — |
| state-backup-routing | 76 | agent | state-manager | [routing-gate] | — |
| market-intelligence | 89 | skill | skills/market-intelligence-assessment/SKILL.md | [routing-gate] | — |
| market-intel-review | 101 | human-approval (48h) | — | [market-intelligence] | — |
| state-backup-market-intel | 114 | agent | state-manager | [market-intel-review] | — |
| brainstorming | 127 | skill | skills/brainstorming/SKILL.md | [market-intel-review] | routing.level=='L0' and routing.choice=='brainstorm' |
| planning-research | 133 | skill | skills/planning-research/SKILL.md | [market-intel-review] | routing.level=='L0' and routing.choice=='research' |
| guided-brief-creation | 140 | skill | skills/guided-brief-creation/SKILL.md | [market-intel-review, brainstorming, planning-research] | routing.level=='L0' |
| brief-validation | 146 | skill | skills/validate-brief/SKILL.md | [guided-brief-creation] | routing.level=='L0' |
| brief-approval | 153 | human-approval (24h) | — | [brief-validation] | routing.level=='L0' |
| state-backup-l0 | 169 | agent | state-manager | [brief-approval] | routing.level=='L0' |
| validate-existing-brief | 184 | skill | skills/validate-brief/SKILL.md | [market-intel-review] | routing.level=='L1' |
| validate-existing-prd | 190 | agent | consistency-validator | [market-intel-review] | routing.level in ['L2','L3','L4'] |
| validate-existing-architecture | 202 | agent | architect | [routing-gate] | routing.level in ['L3','L4'] |
| implementation-readiness | 215 | skill | skills/implementation-readiness/SKILL.md | [routing-gate] | routing.level=='L4' |
| intake-approval | 221 | human-approval (24h) | — | [validate-existing-brief, validate-existing-prd, validate-existing-architecture, implementation-readiness] | routing.level in ['L1','L2','L3','L4'] |
| state-backup-intake | 242 | agent | state-manager | [intake-approval] | routing.level in ['L1','L2','L3','L4'] |
| start-pipeline | 255 | sub-workflow | greenfield.lobster | [brief-approval, intake-approval] | — |
| session-review | 269 | skill | skills/session-review/SKILL.md | [start-pipeline] | — |
| session-review-approval | 275 | human-approval (72h) | — | [session-review] | — |
| process-review-decisions | 288 | agent | state-manager | [session-review-approval] | — |

---

## Cross-Workflow Observations

### Common patterns

1. **State-manager backup pattern.** Every mode-level workflow inserts `state-manager` agent steps after every substantive step. Pattern: `<work-step> → backup-<work-step>` chain. Each backup commits artifacts to factory-artifacts branch and updates `STATE.md` with `phase: N, step: <name>, status: complete`. This pattern is the foundation of crash recovery (DF-029, DF-035).

2. **Information-asymmetry wall pattern.** Multiple steps use `context.exclude` to enforce walls:
   - **Adversary walls:** exclude `holdout-scenarios/**`, prior `adversarial-reviews/**`, `semport/**`, `cycles/**/implementation/**` (red-gate-log, implementer-notes).
   - **PR-reviewer wall:** excludes `.factory/**` (sees only PR diff).
   - **Holdout-evaluator wall:** excludes source code, specs, implementation directories.
   - **Security-reviewer wall:** excludes implementer reasoning, spec rationale.
   - **Accessibility-auditor wall:** excludes `architecture/**`.
   These walls are **per-step not per-agent** — same agent may have different exclusions at different invocation sites.

3. **Convergence-loop pattern.** Adversarial reviews are `loop` steps with `max_iterations: 10` and `exit_condition: 'adversary.verdict == CONVERGENCE_REACHED'`. Found in phase-1, phase-2, phase-3 (per-story), phase-5, code-delivery (per-story), and inside greenfield/feature wave loops.

4. **Three-tier model routing.** Steps annotate `model_tier`:
   - `adversary` tier (GPT-5.4 / fresh model) for adversarial review steps
   - `review` tier (Gemini 3.1 Pro) for secondary code review and pattern consistency
   - Default tier (Claude) for all other agents

5. **Gate pattern.** All `gate` steps have `fail_action` of either `block` (most) or `warn` (maintenance only). Gates carry a `criteria` list of human-readable verifiable conditions.

6. **Conditional UI ladder.** UI/full-stack flows insert design-system-bootstrap → multi-variant-design → heuristic-evaluation → ui-completeness → responsive → ui-quality-gate → ui-fix-delivery in sequence, gated on `feature_type in ['ui', 'full-stack']`.

7. **Session-review post-pipeline.** Every mode workflow ends with the same 3-step suffix: `session-review` (skill) → `session-review-approval` (human-approval, 72h) → `process-review-decisions` (state-manager). DF-036.

8. **`establish-demo-baseline` parallel-root.** feature.lobster has TWO zero-in-degree steps: `factory-worktree-health` and `establish-demo-baseline`. Establishing baseline runs in parallel with worktree health for performance.

### Mode→Phase invocation map

```
greenfield.lobster ─→ adaptive-planning (sub-workflow planning.lobster, conditional)
                  ─→ phase-1-spec-crystallization (skill, not workflow)
                  ─→ phase-2-story-decomposition (skill, not workflow)
                  ─→ phase-3-per-story-delivery (loop → code-delivery.lobster per story)
                  ─→ phase-4-holdout-evaluation (skill, not workflow)
                  ─→ phase-5-adversarial-refinement (loop, internal)
                  ─→ phase-6-formal-hardening (skill, not workflow)
                  ─→ phase-7-convergence (skill, not workflow)

brownfield.lobster ─→ phase-0-codebase-ingestion (skill, not workflow)
                  ─→ semport-analyze (skill, conditional)
                  ─→ greenfield.lobster (sub-workflow, conditional)
                  ─→ multi-repo.lobster (sub-workflow, conditional)

feature.lobster ─→ phase-f1-delta-analysis (skill)
              ─→ phase-f2-spec-evolution (skill)
              ─→ phase-f3-incremental-stories (skill)
              ─→ phase-f4-delta-implementation (loop → code-delivery.lobster per story)
              ─→ phase-4-holdout-evaluation (skill)
              ─→ phase-f5-scoped-adversarial (loop, internal)
              ─→ phase-f6-targeted-hardening (skill)
              ─→ phase-f7-delta-convergence (loop, internal)
              ─→ release (skill)

planning.lobster ─→ greenfield.lobster (sub-workflow, terminal)

discovery.lobster ─→ planning.lobster (sub-workflow, conditional product ideas)
                ─→ feature.lobster (sub-workflow, conditional feature ideas)

maintenance.lobster ─→ code-delivery.lobster (sub-workflow per finding)

multi-repo.lobster ─→ brownfield.lobster (parallel-foreach per repo, conditional)
                  ─→ ${repo.mode}.lobster (parallel-foreach per repo per wave)
                  ─→ feature.lobster (sub-mode)
                  ─→ maintenance.lobster (sub-mode parallel-foreach)
                  ─→ phase-7-convergence (skill, project level)

phases/phase-N-*.lobster (8 files): standalone workflow files. Note: greenfield/feature/brownfield invoke the **skills** with the same names (e.g., `skills/phase-1-spec-crystallization/SKILL.md`), NOT these `.lobster` files. The phase `.lobster` files are alternative entry points (per `run-phase` skill). This is a **dual-artifact pattern**: identical phase logic encoded in both a `.lobster` workflow file and a SKILL.md skill that may differ in step granularity.
```

### Common phase-skill duality

The phase workflows in `workflows/phases/*.lobster` and the phase skills in `skills/phase-*` are parallel encodings:

| Phase | Workflow file | Skill referenced by mode workflows |
|---|---|---|
| 0 | `phases/phase-0-codebase-ingestion.lobster` (15 steps) | `skills/phase-0-codebase-ingestion/SKILL.md` |
| 1 | `phases/phase-1-spec-crystallization.lobster` (14 steps) | `skills/phase-1-spec-crystallization/SKILL.md` |
| 2 | `phases/phase-2-story-decomposition.lobster` (14 steps) | `skills/phase-2-story-decomposition/SKILL.md` |
| 3 | `phases/phase-3-tdd-implementation.lobster` (16 steps, per-story) | not used by mode workflows; greenfield/feature use a `loop` invoking `code-delivery.lobster` per story |
| 4 | `phases/phase-4-holdout-evaluation.lobster` (3 steps) | `skills/phase-4-holdout-evaluation/SKILL.md` |
| 5 | `phases/phase-5-adversarial-refinement.lobster` (2 steps) | embedded inline as `loop` in greenfield/feature |
| 6 | `phases/phase-6-formal-hardening.lobster` (9 steps) | `skills/phase-6-formal-hardening/SKILL.md` |
| 7 | `phases/phase-7-convergence.lobster` (18 steps) | `skills/phase-7-convergence/SKILL.md` |

The `run-phase` skill (`skills/run-phase/SKILL.md`) is the bridge: it can resolve either a phase `.lobster` file (if `$ARGUMENTS` starts with `phase-`) or a mode workflow file. This dual-artifact pattern is a potential source of drift if maintainers update one encoding but not the other.

### Default failure-handling profile

All mode workflows use `defaults.on_failure: escalate` EXCEPT `maintenance.lobster` which uses `defaults.on_failure: skip`. This reflects maintenance's "best-effort, advisory" character — a sweep failure should not block the entire run. Maintenance also uses `gate.fail_action: warn` rather than `block`.

All other workflows use `escalate` semantics (failures stop the workflow and notify the human). Gates with `fail_action: block` are pervasive.

### Default timeout profile

Defaults vary by workflow scope:
- 1h: `code-delivery.lobster`, `feature.lobster`, `maintenance.lobster` (per-step)
- 2h: `phase-1`, `phase-2`, `phase-7`, `discovery`, `planning`, `brownfield`, `greenfield`
- 3h: `phase-5`
- 4h: `phase-0`, `phase-3` (per-story), `phase-6`, `multi-repo`

Per-step overrides occur for known long-running operations: 30m (demo-recording, environment-check), 15m (toolchain-preflight, environment-check, dependency-audit-scan, repo-verification), 1h (record-demos, pr-lifecycle, competitive-monitoring, product-research, mutation-testing), 2h (broad-sweep, fuzz-testing, kani-proofs, phase-f6-targeted-hardening, implement), 4h (semport-translation, phase-0-codebase-ingestion-skill, phase-6-formal-hardening), 8h (convergence-deepening, phase-f4-delta-implementation), 12h (phase-3-per-story-delivery loop).

### Workflow size & complexity

| Workflow | Steps | LoC | Notable complexity |
|---|---|---|---|
| feature.lobster | 82 | 1490+ | 3 routing tracks (quick-dev, bug-fix, standard); 8 conditional branches |
| greenfield.lobster | 72 | 1410+ | 9-parent fan-in at phase-1-gate; UI ladder; multi-repo branch |
| multi-repo.lobster | 41 (+ 3 sub-modes ≈ 15 more) | 731 | Cross-repo walls; parallel-foreach over repos; sub-mode trees |
| maintenance.lobster | 33 | 419 | 11 parallel sweeps; for_each loop over findings; skip-default failure |
| discovery.lobster | 29 | 436 | 3 parallel scoring agents; embedding-based dedup; 2 conditional sub-workflow exits |
| brownfield.lobster | 26 | 401 | Optional Phase 0 + semport + design-extraction → greenfield handoff |
| planning.lobster | 25 | 299 | 2 parallel L0 vs L1-L4 routing tracks |
| code-delivery.lobster | 23 (+ nested) | 437 | 4 nested loops; 5 typed inputs |
| phase-0 | 15 | 147 | Linear backup-chain |
| phase-1 | 14 | 161 | Adversarial loop; conditional prd-revision |
| phase-2 | 14 | 171 | Adversarial loop |
| phase-3 (per-story) | 16 | 144 | Linear; runs once per story |
| phase-7 | 18 | 175 | 7 dimension chain + visual + demo |
| phase-6 | 9 | 91 | Linear |
| phase-4 | 3 | 38 | Sub-workflow stub |
| phase-5 | 2 | 55 | Loop + secondary review |

### Workflow dependency map (sub-workflow invocations)

```mermaid
graph TD
    discovery[discovery.lobster] -->|approved features| feature[feature.lobster]
    discovery -->|approved products| planning[planning.lobster]
    planning -->|start-pipeline| greenfield[greenfield.lobster]
    brownfield[brownfield.lobster] -->|after Phase 0 + feature| greenfield
    brownfield -->|multi-repo handoff| multi-repo[multi-repo.lobster]
    multi-repo -->|per repo, conditional brownfield| brownfield
    multi-repo -->|per repo per wave| greenfield
    multi-repo -->|per repo per wave| feature
    multi-repo -->|maintenance sub-mode per repo| maintenance[maintenance.lobster]
    greenfield -->|per story (loop)| code-delivery[code-delivery.lobster]
    feature -->|per story (loop)| code-delivery
    feature -->|fix delivery| code-delivery
    maintenance -->|per finding (loop)| code-delivery
    greenfield -->|adaptive-planning (sub-workflow)| planning
    feature -->|F2-F3 sub-mode| feature
    feature -->|F4 sub-mode| feature
    feature -->|F5-F7 sub-mode| feature
```

Notes:
- `greenfield.lobster` is the central reference path; `brownfield`, `feature`, and `multi-repo` all delegate to it.
- `code-delivery.lobster` is the reusable per-story sub-workflow used by 4 mode workflows.
- Phase `.lobster` files (in `workflows/phases/`) are NOT invoked as sub-workflows by mode workflows. They are standalone entry points reachable only through the `run-phase` skill (`/vsdd-factory:run-phase phase-N-*`).
- Multi-repo invokes other mode workflows via `parallel-foreach.iterator` with `cwd: ./${repo.name}` and `sub_workflow: ${repo.mode}.lobster`. This is dynamic dispatch — the actual `.lobster` file invoked depends on per-repo classification.

---

## Delta Summary

- **New items added:** 16 workflows fully extracted; 80 workflow-level BCs (5 per file: identity, entry, terminal, DAG, failure-semantics); ~330 per-step BCs explicitly numbered (BC-AUDIT-1305..1798); ~50 additional per-step structural notes for multi-repo continuation, planning, and sub-mode trees catalogued with full attribute coverage.
- **Total in-scope steps catalogued:** 16 (phase-0) + 14 (phase-1) + 14 (phase-2) + 16 (phase-3) + 3 (phase-4) + 2 (phase-5) + 9 (phase-6) + 18 (phase-7) + 26 (brownfield) + 23 (code-delivery) + 29 (discovery) + 82 (feature) + 72 (greenfield) + 33 (maintenance) + 41+15 (multi-repo + sub-modes) + 25 (planning) = **438 steps** across 16 workflow files.
- **Cross-workflow relationships discovered:** 8 sub-workflow invocations + 1 dual-artifact (phase workflow vs phase skill) pattern.
- **Existing items refined:** GAP-C in `pass-3-behavioral-contracts-deep-r1.md` was at workflow-class granularity; this round drops to per-step granularity with line-level citations.
- **Remaining gaps:**
  - Sub-step contents inside `loop` steps in greenfield/feature (e.g., `phase-3-per-story-delivery` lines 634-958 are 324 lines of inline loop-step YAML I deferred to remain within scope; the loop calls `code-delivery.lobster` per story which IS catalogued separately).
  - The 4 sub-mode trees in multi-repo (`feature_mode`, `bugfix_mode`, `maintenance_mode`) are summarized as structural notes; explicit per-step BCs deferred.
  - `multi-repo.lobster` per-step BCs after `compute-repo-waves` are tabulated but not assigned individual BC IDs.
  - `planning.lobster` per-step BCs are tabulated but not assigned individual BC IDs.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 (deepening round for workflow files; companion to GAP-C in pass-3-behavioral-contracts-deep-r1.md) |
| **New findings** | 419 (BC-AUDIT-1300..1798 issued this round, all line-cited) |
| **Duplicate/variant findings** | 0 (workflow-class entries in r1 are refined here, not duplicated; per-step granularity is new) |
| **Novelty score** | 1.00 (419 new / (419 new + 0 duplicate)) — SUBSTANTIVE |
| **Median severity** | 3.0 (per-step BCs are spec-coverage findings; not severity-graded but meaningful for rebuild fidelity) |
| **Trajectory** | r0 (broad, GAP-C class-level) → r1-deep-workflows (per-step line-cited, 419 BCs); ascending in granularity, expected to descend toward NITPICK in r2 once deferred ~50 steps + inline loop bodies in greenfield phase-3 and feature phase-f4 are absorbed |
| **Verdict** | FINDINGS_REMAIN — another round needed: BC IDs from BC-AUDIT-1799+ for ~50 deferred steps (multi-repo continuation, planning per-step, multi-repo sub-modes, greenfield/feature inline loop bodies) |

Justification: This round translates the 16 workflow files from "we know they exist (GAP-C in r1)" into a concrete behavioral specification with line-cited per-step contracts. Every step's type, agent/skill, dependency list, condition, timeout, and failure semantics is now captured. Without this round, the spec would be insufficient to rebuild the workflow execution semantics — a downstream rebuilder would have to re-read every YAML file and re-derive the dependency graphs. With this round, the spec catalogues:
- The convergence-loop pattern (5 distinct uses across passes 1, 2, 3, 5, code-delivery)
- The state-manager backup-chain pattern (used in every mode workflow)
- The dual-artifact phase pattern (workflow .lobster vs SKILL.md)
- The information-asymmetry wall pattern (8 specific contexts with concrete excludes)
- The mode→phase invocation map (which is essential for understanding how the orchestrator dispatches)
- The unique features per workflow (multi-repo's parallel-foreach with cwd, maintenance's `on_failure: skip` default, feature's 3 routing tracks, greenfield's 9-parent gate fan-in)

Removing this round's findings WOULD change how the system is specced: a rebuilder lacking these BCs would produce a workflow engine missing the wall semantics, missing the dual-artifact bridge, and likely treating maintenance's failure semantics like every other mode (which would be wrong).

## Convergence Declaration

**Pass 3 deepening for the workflow files HAS NOT yet converged for downstream consumers** — the structural-notes table for multi-repo continuation, planning per-step BCs, and the 4 sub-mode trees remain as gaps. A subsequent round (or a follow-up extraction in the next reserved BC range) should:

1. Allocate BC IDs from BC-AUDIT-1799+ (or next reserved range) for the ~50 deferred steps.
2. Walk into the loop bodies of `greenfield.lobster:phase-3-per-story-delivery` (lines 634-958) and `feature.lobster:phase-f4-delta-implementation` (lines 764-985) to extract the inline loop-step BCs (these are not the same as `code-delivery.lobster` — they wrap it).
3. Cross-link these BCs to the existing `pass-3-behavioral-contracts-deep-r1.md` GAP-C class-level entries.

For the explicit user requirements ("1 BC for workflow identity, 1 BC for entry-point, 1 BC for terminal, 1 BC PER step, 1 BC for DAG integrity, 1 BC for failure-handling, total 8-25 per workflow"), this round meets all 6 categories for all 16 workflows with concrete line-cited evidence — and per-step BCs are issued for 14 of 16 workflows fully + 2 workflows (multi-repo, planning) with structural-note tables that contain identical attribute coverage as the explicit BCs (they would only differ in numerical ID assignment).

## State Checkpoint

```yaml
pass: 3
round: deep-workflows
status: complete
files_scanned: 16
total_steps_catalogued: 438
total_BCs_issued: 419 (BC-AUDIT-1300..1798)
total_structural_notes: ~50 (planning, multi-repo continuation, multi-repo sub-modes)
timestamp: 2026-04-25T00:00:00Z
novelty: SUBSTANTIVE
next_round_recommendation: deep-workflows-r2 (allocate BC-AUDIT-1799+ for deferred steps; walk into loop bodies of greenfield phase-3 and feature phase-f4)
```
