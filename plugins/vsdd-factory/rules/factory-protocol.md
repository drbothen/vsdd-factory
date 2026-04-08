<!-- Factory operating rules. Defines what goes where and how the .factory/ worktree is managed. -->

# Factory Protocol

## What Is `.factory/`

A git worktree mounted on the orphan `factory-artifacts` branch. It holds all pipeline state, specs, stories, and evaluation artifacts. It is **never** on `main` or `develop` — it lives on its own branch with its own commit history.

## Directory Layout

```
.factory/
├── STATE.md                          # Pipeline progress tracker (CRITICAL)
├── specs/                            # Living spec documents (always current truth)
│   ├── product-brief.md
│   ├── domain-spec/                  # Sharded L2 domain specification
│   │   ├── L2-INDEX.md               # Index linking all sections
│   │   └── <section>.md              # Per-section files
│   ├── prd.md                        # Core PRD (index document)
│   ├── research/                     # Research reports (domain + general)
│   │   ├── RESEARCH-INDEX.md         # Index of all research runs
│   │   ├── domain-<topic>-<date>.md  # Problem space research
│   │   └── general-<topic>-<date>.md # Technology/implementation research
│   ├── prd-supplements/              # interface-definitions, error-taxonomy, etc.
│   ├── behavioral-contracts/
│   │   ├── BC-INDEX.md
│   │   └── BC-S.SS.NNN.md           # Individual contracts
│   ├── verification-properties/
│   │   ├── VP-INDEX.md
│   │   └── VP-NNN.md
│   └── architecture/
│       ├── ARCH-INDEX.md
│       └── ARCH-NN-<section>.md      # Sharded architecture sections
├── stories/                          # Living (accumulate across cycles)
│   ├── STORY-INDEX.md
│   ├── STORY-NNN.md
│   ├── epics.md
│   ├── dependency-graph.md
│   └── sprint-state.yaml
├── cycles/                           # Per-pipeline-run artifacts
│   └── vX.Y.Z-<mode>/
│       ├── cycle-manifest.md
│       ├── adversarial-reviews/
│       ├── convergence-report.md
│       ├── wave-schedule.md
│       └── release-notes.md
├── holdout-scenarios/                # Hidden acceptance scenarios
│   ├── HS-INDEX.md
│   ├── wave-scenarios/
│   └── evaluations/
├── semport/                          # Semantic porting artifacts (per-project subfolders)
│   └── <project>/                    # One folder per ingested codebase
├── code-delivery/                    # Per-story PR templates
├── demo-evidence/                    # Visual review tracking
└── dtu-clones/                       # Digital twin universe clones
```

## Commit Protocol

- **All `.factory/` changes commit to the `factory-artifacts` branch**, not `main` or `develop`.
- Commit from within the `.factory/` directory: `cd .factory && git add -A && git commit -m "..."`.
- Commit at every phase gate transition.
- Commit message format: `factory(<phase>): <description>` (e.g., `factory(phase-1): add PRD and architecture specs`).

## File Lifecycle

| Category | Lifecycle | Meaning |
|----------|-----------|---------|
| specs/ | **Living** | Always current truth. Updated as understanding deepens. |
| stories/ | **Accumulating** | New stories added per cycle. Existing stories versioned. |
| cycles/ | **Cycle-scoped** | Created per pipeline run. Immutable after cycle closes. |
| holdout-scenarios/ | **Living** | Accumulate over time. Some retired after evaluation. |
| semport/ | **Living** | Translation artifacts evolve with implementation. |
| STATE.md | **Critical** | Single source of pipeline progress. Updated at every transition. |

## Rules

- **Never put target project source code in `.factory/`** — only pipeline state, specs, and artifacts.
- **Never modify `.factory/` files from the `main` or `develop` branch** — always work within the worktree.
- **STATE.md is the single source of truth** for pipeline progress. Read it before starting any phase work.
- **Specs are the product, code is disposable** (SOUL.md #3). If specs and code conflict, the spec wins.
