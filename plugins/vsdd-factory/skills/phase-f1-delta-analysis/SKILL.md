---
name: phase-f1-delta-analysis
description: >
  Feature Mode Phase F1: Analyze a feature request against existing artifacts
  to determine impact boundary, affected specs/stories/tests, and regression risk.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F1: Delta Analysis

## Prerequisites

- `.factory/specs/prd.md` exists (previous pipeline completed)
- `.factory/stories/` contains existing story specs
- A feature request has been provided (use `templates/feature-request-template.md` format)

## Workflow

### Step 1: Receive Feature Request

Read the feature request provided by the human. If no structured request exists, ask
the human to fill out `templates/feature-request-template.md` or provide equivalent
information (problem, proposed solution, scope, constraints, success criteria).

### Step 2: Load Existing Artifacts

Read and index all existing pipeline artifacts using 4-level hierarchy (DF-020)
and sharded architecture format (DF-021):
- `.factory/specs/prd.md` -- current PRD with all requirement IDs (BC-S.SS.NNN format)
- `.factory/specs/architecture/ARCH-INDEX.md` -- sharded architecture index
- `.factory/specs/architecture/*.md` -- architecture section files
- `.factory/specs/behavioral-contracts/` -- per-file BCs (BC-S.SS.NNN)
- `.factory/specs/verification-properties/` -- per-file VPs (VP-NNN)
- `.factory/specs/prd-supplements/` -- interface definitions, error taxonomy, etc.
- `.factory/stories/stories/` -- per-file story specs (STORY-NNN.md)
- `.factory/cycles/**/implementation/` -- existing test manifest
- `.factory/cycles/**/convergence/` -- existing traceability chain
- `.factory/specs/module-criticality.md` -- module criticality classification

### Step 3: Impact Boundary Identification

Spawn `architect` agent to:
- Map which existing components are affected by the feature request
- Identify new components that must be created
- Determine if the architecture needs structural changes (new modules, new interfaces)
  or only internal changes (new logic within existing modules)
- Classify each affected component: NEW, MODIFIED, or DEPENDENT (unchanged but
  depends on something that changed)

### Step 4: Affected Artifact Mapping

Spawn `business-analyst` agent to:
- Map feature request to existing PRD requirements using BC-S.SS.NNN format
  (new vs modified vs unchanged)
- Identify which existing stories are in the "regression risk zone" (stories whose
  implementation touches modules being modified)
- List existing tests that cover the affected modules
- Identify which verification properties (VP-NNN) need extension
- Classify feature type: `ui` | `backend` | `full-stack` | `infrastructure`
  (drives conditional routing for UX/a11y/e2e steps in subsequent phases)

### Step 4b: Intent Classification

Classify the intent of the request based on human language and context:

| Intent | Detection Signals | Route |
|--------|------------------|-------|
| `feature` | Human says "add", "build", "new" | Full F1-F7 |
| `enhancement` | Human says "improve", "update", "change" | Full F1-F7 (may be quick dev if trivial) |
| `bug-fix` | Human says "fix", "bug", "broken", "regression" | Bug fix route (skip F2, F3) |

Record intent in the delta analysis report and STATE.md.

### Step 4c: Trivial Scope Classification

Assess whether the change qualifies for quick dev routing. A change is trivial
when ALL of the following are true:

- Impact boundary: single module, single file, or documentation only
- No new BCs needed
- No architecture change
- No new external dependencies
- Regression risk: LOW

If trivial: quick dev routing compresses the pipeline to
F1 (quick scope) -> F4 (single story: worktree -> test -> implement ->
PR -> pr-reviewer -> security-reviewer if CRIT module -> merge)
-> regression suite -> F7 (quick convergence check) -> release (PATCH).

Skips: F2 (no spec change), F3 (no new stories), F5 (no scoped adversarial),
F6 (no formal hardening). Preserves: per-story delivery, PR review, security
check, regression, convergence.

Quick dev is an optimization, not a bypass -- all safety is preserved.

### Step 4d: Severity Classification (Bug Fix Intent Only)

If intent is `bug-fix`, classify severity:

| Severity | Criteria |
|----------|----------|
| CRITICAL | Production down, data loss, security breach |
| HIGH | Major functionality broken, no workaround |
| MEDIUM | Functionality impaired, workaround exists |
| LOW | Minor issue, cosmetic, edge case |

CRITICAL severity triggers the expedited flow:
- Skip demo baseline (no time)
- F1 analysis is minimal (confirm root cause only)
- Human approval is async (proceed immediately, human reviews after)
- F5 adversarial is 1 round max
- F6 hardening runs security scan only (skip proofs/fuzz/mutation)
- Release immediately after F7
- Logged in STATE.md as `expedited: true` with justification
- All skipped steps queued for post-release follow-up

For bug-fix intent, also identify:
- The BC that should hold but doesn't (root cause BC)
- The affected module for scoped holdout (not full product)

### Step 5: Regression Risk Assessment

Assess regression risk per affected module:

| Risk Level | Criteria |
|------------|----------|
| HIGH | Core module being modified, many dependents, security-critical |
| MEDIUM | Non-core module being modified, some dependents |
| LOW | New module (no existing code to break), no dependents |

### Step 6: Produce Delta Analysis Report

Write the Delta Analysis Report to `.factory/phase-f1-delta-analysis/delta-analysis.md`
using `templates/delta-analysis-report-template.md`.

The report must include:
- Feature name and brief link
- Impact assessment table (PRD, Architecture, UX, Stories, Tests, Verification)
- Files likely changed (with change type: new, modified)
- Files NOT changed (regression baseline)
- Risk assessment (regression, architecture, security)
- Recommended scope for subsequent phases

### Step 7: Human Approval Gate

Present the Delta Analysis Report to the human for review.

Questions to resolve:
- Is the scope correct? Too broad? Too narrow?
- Are the risk assessments accurate?
- Should any modules be explicitly excluded from the delta?
- Should this be Feature Mode or does the scope warrant Full Pipeline?

Phase F1 is COMPLETE only when the human explicitly approves the scope.

### Multi-Repo Delta Analysis (DF-012)

For multi-repo projects, delta analysis must identify:
- **Which repos** are affected by the feature request (a frontend change may not touch the API server)
- **Which cross-repo contracts** change (does the API contract need updating? Do SDK consumers need regeneration?)
- **Cross-repo regression risk** — a contract change in one repo may break consumers in other repos
- Include affected repos in `.factory/phase-f1-delta-analysis/affected-repos.txt`

## Output Artifacts

- `.factory/phase-f1-delta-analysis/delta-analysis.md`
- `.factory/phase-f1-delta-analysis/affected-files.txt` (machine-readable file list)
- `.factory/phase-f1-delta-analysis/affected-repos.txt` (multi-repo projects only)

## Quality Gate Criteria

- [ ] All affected components identified with change type (NEW/MODIFIED/DEPENDENT)
- [ ] Regression risk assessed per affected module
- [ ] Existing tests in the risk zone are enumerated
- [ ] Files NOT changed are explicitly listed as regression baseline
- [ ] Feature type classified (ui | backend | full-stack | infrastructure)
- [ ] Intent classified (feature | enhancement | bug-fix)
- [ ] Trivial scope assessed (trivial | standard)
- [ ] Severity classified if bug-fix intent (CRITICAL | HIGH | MEDIUM | LOW)
- [ ] Delta analysis report references BC-S.SS.NNN identifiers (no FR-NNN)
- [ ] For multi-repo projects: affected repos and contract changes identified
- [ ] Human has explicitly approved the scope

### Scoping Rules Reference

For scope classification criteria, reference `skills/feature-mode-scoping-rules/SKILL.md`.
This defines how trivial vs non-trivial scope is determined and the signals that
classify intent (feature/enhancement/bug-fix) and feature type (ui/backend/full-stack/infrastructure).

### Quick Dev Routing Reference

For trivial-scope changes, reference `skills/quick-dev-routing/SKILL.md`.
This defines the compressed pipeline: single story → regression → F7 lite → PATCH.
Invoked when F1 classifies scope as trivial (single module, no new BCs, no arch change).
