# Templates Reference

The vsdd-factory plugin ships 108 template files in `plugins/vsdd-factory/templates/`. Templates define the exact output structure for every artifact the pipeline produces. Skills reference templates via `${CLAUDE_PLUGIN_ROOT}/templates/<name>` and agents read them before generating output.

---

## Template Categories

### Spec Templates

Templates for Phase 1 spec crystallization artifacts.

| Template | Used By |
|----------|---------|
| `product-brief-template.md` | `/create-brief`, `/guided-brief-creation` |
| `L2-domain-spec-template.md` | `/create-domain-spec` |
| `L2-domain-spec-index-template.md` | `/create-domain-spec` |
| `L2-domain-spec-section-template.md` | `/create-domain-spec` |
| `prd-template.md` | `/create-prd` |
| `prd-supplement-error-taxonomy-template.md` | `/create-prd` |
| `prd-supplement-interface-definitions-template.md` | `/create-prd` |
| `prd-supplement-nfr-catalog-template.md` | `/create-prd` |
| `prd-supplement-test-vectors-template.md` | `/create-prd` |
| `behavioral-contract-template.md` | `/create-prd`, `/adversarial-review` |
| `L4-verification-property-template.md` | `/create-architecture` |
| `architecture-template.md` | `/create-architecture` |
| `architecture-section-template.md` | `/create-architecture` |
| `architecture-index-template.md` | `/create-architecture` |
| `architecture-feasibility-report-template.md` | `/create-architecture` |
| `module-criticality-template.md` | `/create-architecture` |
| `domain-research-template.md` | `/research` |

### Delivery Templates

Templates for Phase 2-3 story decomposition and implementation.

| Template | Used By |
|----------|---------|
| `story-template.md` | `/create-story`, `/decompose-stories` |
| `epic-template.md` | `/decompose-stories` |
| `wave-schedule-template.md` | `/decompose-stories`, `/wave-scheduling` |
| `pr-description-template.md` | `/pr-create` |
| `red-gate-log-template.md` | `/deliver-story` |
| `cycle-manifest-template.md` | Phase gate transitions |
| `state-template.md` | `/state-update` |
| `factory-project-state-template.md` | `/state-update` (multi-repo) |
| `factory-project-structure-template.md` | `/factory-health` |
| `traceability-matrix-template.md` | `/validate-consistency` |
| `spec-changelog-template.md` | Spec steward |

### Review Templates

Templates for adversarial review, code review, and consistency validation.

| Template | Used By |
|----------|---------|
| `adversarial-review-template.md` | `/adversarial-review` |
| `adversarial-finding-template.md` | `/adversarial-review` |
| `adversarial-review-index-template.md` | `/adversarial-review` |
| `code-review-template.md` | Code reviewer agent |
| `security-review-template.md` | `/formal-verify`, security reviewer |
| `security-scan-report-template.md` | `/formal-verify` |
| `consistency-report-template.md` | `/validate-consistency` |
| `review-findings-template.md` | PR review cycle |
| `findings-tracker-template.md` | Finding lifecycle |
| `delta-analysis-report-template.md` | Feature mode delta analysis |

### Evaluation Templates

Templates for holdout evaluation and DTU assessment.

| Template | Used By |
|----------|---------|
| `holdout-evaluation-report-template.md` | `/holdout-eval` |
| `holdout-scenario-template.md` | `/decompose-stories` |
| `evaluation-summary-template.md` | `/holdout-eval` |
| `evaluation-per-scenario-template.md` | `/holdout-eval` |
| `evaluation-index-template.md` | `/holdout-eval` |
| `dtu-assessment-template.md` | `/dtu-validate` |
| `dtu-clone-spec-template.md` | `/dtu-creation` |
| `dtu-fidelity-report-template.md` | `/dtu-validate` |
| `gene-transfusion-assessment-template.md` | `/semport-analyze` |

### Convergence and Release Templates

| Template | Used By |
|----------|---------|
| `convergence-report-template.md` | `/convergence-check` |
| `release-notes-template.md` | `/release` |
| `verification-gap-analysis-template.md` | `/formal-verify` |
| `fuzz-report-template.md` | `/formal-verify` |
| `vp-withdrawal-template.md` | VP lifecycle management |

### Pipeline and Session Templates

| Template | Used By |
|----------|---------|
| `session-review-template.md` | `/session-review` |
| `sweep-report-template.md` | `/maintenance-sweep` |
| `tech-debt-register-template.md` | `/track-debt` |
| `conventions-template.md` | Brownfield ingest |
| `recovered-architecture-template.md` | Brownfield ingest |
| `project-context-template.md` | Pipeline initialization |
| `project-manifest-template.yaml` | Multi-repo project setup |
| `skill-delegation-template.md` | Orchestrator agent delegation |
| `skill-execution-template.md` | Orchestrator agent execution |
| `autonomy-config-template.yaml` | Progressive autonomy |
| `discovery-config-template.yaml` | Discovery mode |
| `discovery-report-template.md` | Discovery mode |
| `merge-config-template.yaml` | PR merge configuration |
| `agents-md-template.md` | Agent documentation generation |
| `feature-request-template.md` | Feature mode intake |
| `idea-brief-template.md` | Idea capture |
| `demo-evidence-report-template.md` | `/record-demo` |
| `demo-ci-workflow-template.yaml` | Demo CI pipeline |
| `demo-playwright-template.spec.ts` | Playwright demo scripts |
| `demo-tape-template.tape` | VHS terminal recording |

### UX Spec Templates

| Template | Used By |
|----------|---------|
| `ux-spec-template.md` | `/ux-heuristic-evaluation` |
| `ux-spec-index-template.md` | UX spec index |
| `ux-spec-screen-template.md` | Per-screen UX spec |
| `ux-spec-flow-template.md` | UX interaction flows |
| `ui-traceability-template.yaml` | `/ui-completeness-check` |

### Design System Templates (in `templates/design-system/`)

A structured directory containing design token definitions and component specifications for UI products:

- **tokens/** -- JSON token files for colors, typography, spacing, sizing, elevation, motion, and accessibility
- **components/** -- Component registry YAML and per-component contract definitions
- **patterns/** -- Composite pattern definitions for forms, layouts, and navigation
- **constraints.yaml** -- Design system constraint rules

### UI Quality Templates (in `templates/ui-quality/`)

Templates for the UI quality gate pipeline:

| Template | Used By |
|----------|---------|
| `completeness-report-template.md` | `/ui-completeness-check` |
| `gate-report-template.md` | `/ui-quality-gate` |
| `heuristic-evaluation-template.md` | `/ux-heuristic-evaluation` |
| `responsive-report-template.md` | `/responsive-validation` |

### Adversary Prompt Templates (in `templates/adversary-prompt-templates/`)

Pre-built prompt templates for the adversary agent, scoped to different review targets (spec review, implementation review, security review).

### Justfile Template (in `templates/project-justfile-template/`)

Template for generating project justfiles with standard recipes for build, test, lint, and CI commands.

---

## How Templates Are Used

Skills and agents reference templates at the start of their execution:

```
Read and follow the output format in:
- ${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md
```

The `${CLAUDE_PLUGIN_ROOT}` variable resolves to the plugin's installation directory. Templates are read-only -- agents consume them as format specifications and produce output conforming to the template structure. Templates are never modified during pipeline execution.
