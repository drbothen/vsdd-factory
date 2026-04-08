# .factory-project/ Directory Structure

Template for the project-level factory state directory used in multi-repo
projects. Created during project planning by devops-engineer.

## Directory Layout

```
.factory-project/
├── STATE.md                          # Project-level state (phase, mode, repo statuses)
├── project-plan.md                   # From project.yaml parsing
├── repo-waves.md                     # Repo-level wave computation
├── cost-summary.md                   # Project-level cost tracking
├── contract-registry.md              # Cross-repo API contracts
├── contract-changes.md               # Contract change detection results
│
├── phase-0-synthesis/                # Multi-repo brownfield only
│   ├── cross-repo-dependencies.md
│   ├── unified-architecture.md
│   ├── convention-reconciliation.md
│   ├── unified-security-posture.md
│   ├── project-context.md
│   └── synthesis-validation-report.md
│
├── cross-repo-holdout-scenarios/     # Integrated system holdout scenarios
│   └── *.md
│
├── integration-results/              # Cross-repo integration test results
│   └── *.md
│
└── release/                          # Coordinated release artifacts
    ├── release-plan.md
    └── per-repo-changelogs/
```

## Git Strategy

`.factory-project/` is a git worktree on a `factory-project-artifacts` branch
(separate from per-repo `factory-artifacts` branches). state-manager commits
at every project-level gate.

Each individual repo also has its own `.factory/` directory (per existing design)
with per-repo artifacts on per-repo `factory-artifacts` branches.
The `.factory-project/` is the coordination layer.

## Initialization

During project planning (devops-engineer):

1. Create `.factory-project/` directory
2. Set up as git worktree on `factory-project-artifacts` branch
3. Initialize STATE.md from factory-project-state-template.md
4. Create subdirectories as needed

state-manager commits project-level artifacts at every gate.
