---
name: recover-state
description: Reconstruct .factory/STATE.md from artifacts on disk when the state file is corrupted, missing, or out of sync. Scans existing artifacts to determine pipeline phase, story statuses, and spec coverage.
argument-hint: "[--dry-run]"
---

# Recover State

Reconstruct `.factory/STATE.md` from the artifacts that actually exist on disk.

## When to Use

- STATE.md is missing or deleted
- STATE.md is corrupted (invalid YAML frontmatter, garbled content)
- STATE.md is out of sync with reality (says Phase 2 but Phase 3 stories exist)
- Pipeline cannot resume because state is unreadable
- After manual git operations that may have disrupted `.factory/`

## Prerequisites

- `.factory/` must be a mounted worktree on `factory-artifacts` branch
- If `.factory/` doesn't exist at all, run `/vsdd-factory:factory-health` first

## Procedure

### Step 1: Backup existing STATE.md

If STATE.md exists (even if corrupted):
1. Copy to `.factory/STATE.md.backup-YYYY-MM-DD-HHMMSS`
2. Report: "Backed up existing STATE.md to <backup-path>"

If `--dry-run` is passed, skip all writes and only report what WOULD be reconstructed.

### Step 2: Scan artifact directories

Probe each `.factory/` subdirectory for existence and content:

| Directory | What to scan | Determines |
|-----------|-------------|------------|
| `specs/product-brief.md` | Exists? | Phase 1 started |
| `specs/domain-spec/` | L2-INDEX.md exists? Section count? | Phase 1a progress |
| `specs/prd.md` | Exists? | Phase 1a complete |
| `specs/behavioral-contracts/` | BC-INDEX.md exists? BC count? | Phase 1a BCs produced |
| `specs/architecture/` | ARCH-INDEX.md exists? Section count? | Phase 1b complete |
| `specs/verification-properties/` | VP-INDEX.md exists? VP count? | Phase 1b VPs produced |
| `specs/dtu-assessment.md` | Exists? DTU_REQUIRED value? | DTU status |
| `stories/` | STORY-INDEX.md exists? Story count? | Phase 2 complete |
| `stories/sprint-state.yaml` | Exists? Story statuses? | Phase 3 progress |
| `cycles/` | Which cycle directories exist? | Cycle history |
| `holdout-scenarios/` | HS-INDEX.md exists? Scenario count? | Holdout readiness |
| `current-cycle` | Exists? Content? | Active cycle |

### Step 3: Determine current phase

Apply this decision tree based on artifact presence:

```
No product-brief.md          → Phase: pre-pipeline
product-brief.md only        → Phase: 1a (spec crystallization started)
prd.md + BC-INDEX exists      → Phase: 1a (BCs produced)
ARCH-INDEX exists             → Phase: 1b (architecture complete)
STORY-INDEX exists            → Phase: 2 (stories decomposed)
sprint-state.yaml exists      → Phase: 3 (implementation in progress)
  Check story statuses:
    All stories status=merged → Phase: 3 complete
    Some stories in-progress  → Phase: 3 (mid-wave)
holdout evaluations exist     → Phase: 3.5 (holdout eval)
adversarial-reviews/ exists   → Phase: 4 (adversarial refinement)
convergence-report.md exists  → Phase: 6 (convergence)
```

### Step 4: Extract counts from INDEX files

For each INDEX file that exists, count entries:

- **BC count:** Number of non-header rows in BC-INDEX.md
- **VP count:** Number of non-header rows in VP-INDEX.md
- **Story count:** Number of non-header rows in STORY-INDEX.md
- **Holdout count:** Number of non-header rows in HS-INDEX.md
- **Adversarial passes:** Count of `pass-*.md` files in the active cycle's `adversarial-reviews/`

### Step 5: Extract story statuses

If `sprint-state.yaml` exists, parse it for per-story status:

```yaml
stories:
  STORY-001: { status: merged, wave: 1 }
  STORY-002: { status: in-progress, wave: 2 }
```

Count: total, draft, in-progress, merged, blocked.

If `sprint-state.yaml` doesn't exist but STORY-INDEX exists, derive statuses from the STORY-INDEX status column.

### Step 6: Extract DTU status

If `specs/dtu-assessment.md` exists, extract:
- `dtu_required`: true/false (from frontmatter or body)
- `dtu_services`: list of services (from body tables)
- `dtu_clones_built`: check if `dtu-clones/` directory has content

### Step 7: Reconstruct STATE.md

Use the template at `${CLAUDE_PLUGIN_ROOT}/templates/state-template.md` and populate:

- **Project Metadata:** Derive from product-brief.md (product name), git remote (repo URL), directory structure (mode)
- **Phase Progress:** Mark phases as `passed` if their gate artifacts exist, `in-progress` for the current phase, `not-started` for future phases
- **Current Phase Steps:** Leave empty (agent-level state cannot be reconstructed from files)
- **DTU fields:** From step 6
- **Convergence trajectory:** From adversarial review pass counts if available

### Step 8: Validate reconstruction

Before writing, present the reconstructed state to the user:

```
## Reconstructed Pipeline State

Product: <name>
Mode: <greenfield/brownfield/feature>
Current Phase: <N>
Phase Progress:
  Phase 1a: PASSED (PRD + 42 BCs + 12 VPs)
  Phase 1b: PASSED (architecture + 7 sections)
  Phase 2:  PASSED (28 stories in 4 waves)
  Phase 3:  IN PROGRESS (Wave 2: 8/12 stories merged)
  Phase 3.5-6: NOT STARTED

Artifact Counts:
  BCs: 42 (from BC-INDEX)
  VPs: 12 (from VP-INDEX)
  Stories: 28 (from STORY-INDEX)
  Holdout scenarios: 15 (from HS-INDEX)

DTU: required=true, services=[Stripe, Okta], clones_built=true

Does this look correct? [approve / adjust]
```

### Step 9: Write STATE.md

On user approval:
1. Write the reconstructed STATE.md to `.factory/STATE.md`
2. Report: "STATE.md reconstructed. Pipeline can resume from Phase <N>."

If `--dry-run` was passed, report what would be written without writing.

## Limitations

- **Cannot reconstruct Decisions Log** — decisions are recorded incrementally and not derivable from artifacts
- **Cannot reconstruct Skip Log** — skip justifications are recorded at the time of skipping
- **Cannot reconstruct Blocking Issues** — blockers are resolved and removed over time
- **Cannot reconstruct Current Phase Steps** — per-step agent status is ephemeral
- **Cannot reconstruct cost data** — cost tracking is incremental

These sections will be empty in the reconstructed STATE.md. The human should populate them from memory or conversation history if needed.

## Out of Scope

- Does NOT fix corrupted artifacts other than STATE.md
- Does NOT reconstruct sprint-state.yaml (that's derived from story files)
- Does NOT modify any artifact files — read-only scan
- Does NOT commit the reconstructed STATE.md — that's state-manager's job
