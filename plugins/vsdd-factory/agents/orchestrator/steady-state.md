---
name: orchestrator-steady-state
description: Orchestrator workflow reference for steady-state operations once a project is in ongoing maintenance. Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Steady-State Operations

Reference file for the orchestrator. Load after v1.0.0 release or when
operating in maintenance/feature mode.

## Greenfield to Steady-State Handoff

After v1.0.0 releases:

1. Archive greenfield cycle:
   Spawn state-manager: move operational artifacts to cycles/vX.Y.Z-greenfield/
2. Preserve living specs:
   specs/ remains as-is (living truth for vX.Y.Z)
3. Create cycle manifest:
   Spawn state-manager: create cycles/vX.Y.Z-greenfield/cycle-manifest.md
4. Snapshot factory-artifacts:
   Spawn state-manager: git tag vX.Y.Z on factory-artifacts branch
5. Enable steady-state paths:
   Maintenance sweep schedule activated (Path 10)
   Discovery schedule activated (Path 8, if configured)
6. Notify human:
   "vX.Y.Z released. Factory is now in steady-state mode.
    Maintenance sweeps running weekly. Discovery running [schedule].
    Ready for feature requests."

## Feature Cycle Initialization

Spawn state-manager:
1. Create cycle directory: cycles/vX.Y.Z-feature-NAME/
2. Initialize cycle-manifest.md from template
3. Update STATE.md: pipeline: FEATURE-CYCLE, active_cycle: vX.Y.Z-feature-NAME
4. Pause maintenance sweeps (avoid conflicts on develop branch)

## Feature Cycle Handoff

Spawn state-manager:
1. Archive cycle operational artifacts to cycles/vX.Y.Z-feature-NAME/
2. Update cycle manifest with delivered stories, BCs, VPs
3. Snapshot factory-artifacts: git tag vX.Y.Z
4. Resume maintenance sweeps
5. Run spec coherence check (33 criteria — see consistency-validator)
6. Update tech debt register
7. Present product backlog to human

## Product Backlog Management

Multiple sources generate work items concurrently:
- Discovery (Path 8): new feature ideas
- Human: feature requests
- Bug reports: from production
- Maintenance: sweep findings
- Tech debt: accumulated items
- Deprecation: features to remove

Track the backlog in STATE.md under `## Product Backlog`.
The human is the final prioritizer. Present the backlog and ask:
"Ready to start vX.Y.Z Feature A? [yes / reorder / defer]"

## Concurrent Path Coordination

| Rule | Why |
|------|-----|
| One feature cycle at a time | Avoid merge conflicts on develop |
| Maintenance pauses during feature cycles | Don't create fix PRs while feature PRs are in flight |
| Discovery continues during feature cycles | Research doesn't modify code -- safe to run |
| Bug fixes can interrupt feature cycles | P0 bugs get a hotfix branch, not develop |
| Tech debt cycles are scheduled like features | One at a time, same VSDD rigor |

## Hotfix Flow (Urgent Bug During Feature Cycle)

1. **Pause feature cycle** — Spawn state-manager: "Save feature cycle state to STATE.md,
   mark pipeline as HOTFIX-IN-PROGRESS"
2. **Create hotfix branch** — Spawn devops-engineer:
   "Create hotfix branch from latest release tag:
   `git checkout -b hotfix/vX.Y.Z+1 vX.Y.Z`"
3. **Compressed bug-fix delivery** (per feature-sequence.md Bug-Fix Routing):
   a. Spawn architect: "Delta analysis of the bug — classify, identify affected modules/BCs"
   b. Spawn story-writer: "Create fix story with mandatory regression test"
   c. Spawn devops-engineer: "Create worktree for fix story on hotfix branch"
   d. Per-story delivery cycle: tests → implementation → demo → PR → review → merge
      (PR targets hotfix branch, not develop)
   e. Spawn holdout-evaluator: "Scoped holdout evaluation on fix"
   f. Spawn formal-verifier: "Compressed hardening — re-run only affected VPs"
   g. Spawn consistency-validator: "Compressed convergence check on fix"
4. **Release patch** — Spawn devops-engineer:
   "Tag vX.Y.Z+1, create release, publish"
5. **Merge hotfix into develop** — Spawn devops-engineer:
   "Merge hotfix branch into develop. Resolve conflicts if needed."
6. **Resume feature cycle** — Spawn devops-engineer:
   "Rebase feature worktrees onto develop (post-hotfix)"
   Spawn state-manager: "Restore feature cycle state, mark HOTFIX-COMPLETE"

## Feature Deprecation Path

Deprecation follows Feature Mode (Path 3) with inverted intent:

1. PHASE F1: DEPRECATION ANALYSIS
   Spawn architect: identify all affected BCs, VPs, stories, holdout scenarios, DTU clones
2. PHASE F2: SPEC DEPRECATION
   Spawn product-owner: mark BCs as deprecated, holdout scenarios as stale
   Spawn architect: mark VPs as withdrawn, update architecture, module-criticality
3. PHASE F3: DEPRECATION STORIES
   Spawn story-writer: create removal stories (STORY-DEP-NNN)
4. PHASE F4-F7: standard delivery with adversarial + hardening + convergence
5. RELEASE (minor bump -- deprecation is a feature, not a patch)

Track active deprecations in STATE.md.

## DX Engineer Delegation

Spawn dx-engineer at these points:
1. Repo initialization: environment setup, direnv, .env.example, mcporter install
2. Pre-pipeline: LLM availability check, MCP preflight validation
3. Pre-Phase-3: full toolchain preflight (after Phase 2 human approval)
4. Pre-Phase-5: targeted recheck (Kani, fuzzers, Semgrep versions)
5. Pre-release: validate .env has registry keys (names only)

## Human Notification System

Notifications are sent via the configured channel in `.factory/merge-config.yaml`.
Default: github-issues. Options: github-issues, slack, discord, email, terminal.

| Event | Severity | When |
|-------|---------|------|
| Need API keys in .env | BLOCKING | DTU assessment, pre-release |
| Tool has security advisory | BLOCKING | Pre-Phase-3 toolchain audit |
| Agent timeout (3rd retry) | ESCALATION | Any phase |
| Budget threshold reached | WARNING/BLOCKING | Any phase (70%/95%) |
| Critical security finding | URGENT | Per-story PR, Phase 4/5 |
| CI failed 3 times | ESCALATION | Per-story delivery |
| Merge conflict unresolvable | BLOCKING | Per-story merge |
| Phase gate passed | INFO | Phase transitions |
| Pipeline complete | INFO | Phase 6 |
| Model unavailable | BLOCKING | Any phase |
| Multi-repo wave complete | INFO | Per-repo wave transitions |
| Cross-repo integration gate failed | BLOCKING | Integration gate |
| Contract change detected | WARNING | After wave 0 implementation |
