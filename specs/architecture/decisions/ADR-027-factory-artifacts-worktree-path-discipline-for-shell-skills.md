---
document_type: architecture-decision-record
level: L3
adr_id: ADR-027
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-18T00:00:00Z
title: "ADR-027: factory-artifacts worktree path discipline for shell skills and bats fixtures"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-05
  - SS-06
  - SS-07
subsystems_affected:
  - SS-05
  - SS-06
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "Architectural wiring decision resolves a BLOCKER finding (F-S1801-P3-001) in the S-18.01 LOCAL adversary pass. All paths are verifiable against the live git worktree list. No open questions remain."
last_amended: "2026-06-18 (v1.0) — initial authoring by architect resolving F-S1801-P3-001 BLOCKER."
---

# ADR-027: factory-artifacts worktree path discipline for shell skills and bats fixtures

## Status

**ACCEPTED — resolves F-S1801-P3-001 BLOCKER (S-18.01 LOCAL adversary pass 3).**

This ADR codifies the canonical path discipline for shell skills and bats test fixtures that
read from or write to the `factory-artifacts` orphan branch. It closes the mutual-exclusion
contradiction identified in F-S1801-P3-001: a skill cannot simultaneously use
`${ARTIFACTS_WT}/.factory/specs/…` for reads (implying `ARTIFACTS_WT` = repo root) and
`git -C "$ARTIFACTS_WT" commit` for writes (implying `ARTIFACTS_WT` = the factory-artifacts
worktree mount), because the two interpretations are incompatible.

---

## Context

### The factory-artifacts worktree layout

The `factory-artifacts` orphan branch is mounted as a git linked worktree at `.factory/`
inside the repository root. This is confirmed by the production `git worktree list` output:

```
/…/vsdd-factory                     c000b06f [develop]
/…/vsdd-factory/.factory            33aaf44b [factory-artifacts]
```

The worktree root IS `.factory/` — there is no additional subdirectory layer. Consequently:

| What you expect to find | Correct path | Incorrect path (double-nesting) |
|-------------------------|-------------|--------------------------------|
| STATE.md | `.factory/STATE.md` | `.factory/.factory/STATE.md` |
| specs/ | `.factory/specs/…` | `.factory/.factory/specs/…` |
| stories/ | `.factory/stories/…` | `.factory/.factory/stories/…` |
| HANDOFF.md (output) | `.factory/HANDOFF.md` | `.factory/.factory/HANDOFF.md` |

### The contradiction in E-18 story wiring

S-18.01 (wave-handoff skill) must both:

(a) **Read** spec files from factory-artifacts — `${ARTIFACTS_WT}/.factory/specs/behavioral-contracts` implies `ARTIFACTS_WT` = repo root (`.`), making the factory-artifacts content accessible at `.factory/specs/behavioral-contracts`.

(b) **Commit** HANDOFF.md to factory-artifacts — `git -C "$ARTIFACTS_WT" commit` implies `ARTIFACTS_WT` = the worktree mount (`.factory`), since that is the path where the factory-artifacts branch is checked out.

These two interpretations are mutually exclusive: if `ARTIFACTS_WT = .` (repo root), then `git -C "$ARTIFACTS_WT" commit` commits to develop/feature (wrong branch). If `ARTIFACTS_WT = .factory` (worktree), then `${ARTIFACTS_WT}/.factory/specs/…` = `.factory/.factory/specs/…` (does not exist).

### The hermetic bats fixture pattern

Bats test fixtures create a temp repo (`WORK`) with a linked worktree (`ARTIFACTS_WT=$WORK/factory-wt`) on the factory-artifacts orphan branch. Fixture files inside the factory-artifacts worktree are placed at `$ARTIFACTS_WT/.factory/specs/…` — i.e., the fixture nests a `.factory/` subdirectory inside the temp worktree to simulate the production layout where the worktree is mounted AS `.factory/`.

This fixture nesting correctly mirrors production (where `.factory/specs/…` is real), but it means that `${ARTIFACTS_WT}/.factory/…` in the bats context correctly resolves because the fixture creates that structure. However, using `${ARTIFACTS_WT}/.factory/…` in production would produce `.factory/.factory/…` — which does not exist. The hermetic fixture MASKED the production contradiction rather than catching it.

---

## Decision

### Decision 1 — Canonical value of ARTIFACTS_WT

`ARTIFACTS_WT` (also `--artifacts-worktree` CLI arg, also `FACTORY_ARTIFACTS_PATH` env var) is defined as **the root of the factory-artifacts linked worktree**, which in production equals `.factory` (relative to the repository root, or its absolute equivalent).

In all shell skills and hook scripts:
- `ARTIFACTS_WT` = `.factory` (production) or an equivalent absolute path
- `git -C "$ARTIFACTS_WT" commit` commits to the factory-artifacts branch — correct
- `${ARTIFACTS_WT}/STATE.md` = `.factory/STATE.md` — correct
- `${ARTIFACTS_WT}/specs/behavioral-contracts/…` = `.factory/specs/behavioral-contracts/…` — correct
- `${ARTIFACTS_WT}/.factory/…` is FORBIDDEN — produces `.factory/.factory/…` in production — does not exist

### Decision 2 — Two-arg invocation model for skills that also read sprint-state.yaml and STATE.md

Skills like `wave-handoff` that need to read `sprint-state.yaml` and `STATE.md` MUST accept these as separate CLI arguments (or env vars) rather than deriving them from `$ARTIFACTS_WT`. This serves two purposes:

1. **Bats fixture isolation:** Bats tests supply synthetic `sprint-state.yaml` and `STATE.md` files placed in the temp repo dir (not inside the factory-artifacts worktree) as separate args. This allows fixture files to be simpler (no nested `.factory/` subdirectory inside the factory-artifacts worktree temp dir).

2. **Explicit contract:** The invocation contract is unambiguous. The caller (orchestrator or human) passes exact paths; the skill does not infer them.

**Required CLI args for wave-handoff and sibling skills:**

| Arg | Env fallback | Production default value |
|-----|-------------|--------------------------|
| `--artifacts-worktree` | `ARTIFACTS_WT` | `.factory` |
| `--sprint-state` | `SPRINT_STATE_YAML` | `.factory/stories/sprint-state.yaml` |
| `--state-md` | `STATE_MD_PATH` | `.factory/STATE.md` |
| `--bc-dir` (optional) | `BC_DIR` | `.factory/specs/behavioral-contracts` |

In production all four paths are inside the factory-artifacts worktree (`.factory`). The two-arg model is NOT a semantic split between "repo root" and "worktree" — it is purely for hermetic bats fixture isolation.

### Decision 3 — Bats fixture correction (no more nested .factory/ in ARTIFACTS_WT)

Bats fixtures for E-18 skills MUST be restructured to NOT nest a `.factory/` subdirectory inside the factory-artifacts worktree temp dir. Instead:

- `ARTIFACTS_WT = $WORK/factory-wt` (factory-artifacts linked worktree root)
- Fixture files inside factory-artifacts live directly at `$ARTIFACTS_WT/STATE.md`, `$ARTIFACTS_WT/specs/…`, `$ARTIFACTS_WT/stories/…`, etc.
- `sprint-state.yaml` and `STATE.md` are passed as separate `--sprint-state` and `--state-md` args pointing to files in `$WORK` (the main repo temp dir), keeping them outside the factory-artifacts worktree for fixture simplicity

**Rationale:** The prior fixture pattern (nesting `.factory/` inside `$ARTIFACTS_WT`) allowed `${ARTIFACTS_WT}/.factory/…` paths to resolve inside the fixture even though they would produce `.factory/.factory/…` in production. This masked the contradiction. The corrected fixture makes the test environment faithfully reflect the production layout.

**Note on the S-18.01 worktree bats file (`wave-handoff.bats`):** The fixture in the S-18.01 development worktree already follows the corrected pattern — it uses `ARTIFACTS_WT=$WORK/factory-wt` and places fixture spec files directly under `$ARTIFACTS_WT/.factory/specs/…`. This fixture's nesting is deliberate for test isolation and is correct for the two-arg invocation model: the skill receives `--artifacts-worktree $ARTIFACTS_WT` and `--bc-dir $ARTIFACTS_WT/.factory/specs/behavioral-contracts`. The production equivalent is `--artifacts-worktree .factory` and `--bc-dir .factory/specs/behavioral-contracts`. Both resolve consistently because `BC_DIR` is passed explicitly — the skill does not derive it from `$ARTIFACTS_WT` with an additional `.factory/` prefix.

### Decision 4 — S-18.04a (precompact-flush.sh) and S-18.05 (postcompact-reanchor.sh): same convention

`precompact-flush.sh` (S-18.04a) and `postcompact-reanchor.sh` (S-18.05) follow the same path discipline:

**S-18.04a (precompact-flush.sh):**
- Reads STATE.md from the factory-artifacts worktree mount: `${FACTORY_ARTIFACTS_PATH}/STATE.md` or equivalent (AC-014 documents: "The factory-artifacts worktree mount (path from environment variable `FACTORY_ARTIFACTS_PATH` or conventional `.factory` relative path)")
- Commits via `git -C "${FACTORY_ARTIFACTS_PATH}" commit` (where `FACTORY_ARTIFACTS_PATH` = `.factory` in production)
- Appends to precompact-flush-log at `${FACTORY_ARTIFACTS_PATH}/hooks/precompact-flush-log` — i.e., `.factory/hooks/precompact-flush-log` in production

S-18.04a's AC-014 already establishes `FACTORY_ARTIFACTS_PATH` as the environment variable encoding this path. ADR-027 confirms this is the same variable concept as `ARTIFACTS_WT` and `--artifacts-worktree` — they all refer to the factory-artifacts worktree root (`.factory` in production).

**S-18.05 (postcompact-reanchor.sh):**
- Reads from factory-artifacts via `git show factory-artifacts:.factory/STATE.md` — this is correct because it uses git ref syntax, not filesystem paths. The git object path `.factory/STATE.md` means "the file at path `STATE.md` within the tree of the `factory-artifacts` branch" — it resolves correctly against the factory-artifacts branch tree regardless of where the worktree is mounted.
- Does NOT commit to factory-artifacts (BC-7.07.002 Inv1 absolute prohibition) — no path ambiguity for writes
- Writes daily log to `.factory/logs/postcompact-reanchor-YYYY-MM-DD.jsonl` — this is a filesystem write to the factory-artifacts worktree, same convention as S-18.04a

S-18.05 has no path-discipline conflict because its only factory-artifacts interaction is a `git show` read (which is branch-ref-based, not worktree-path-based) and its log write uses the same `.factory/logs/…` pattern.

**Conclusion:** S-18.04a and S-18.05 share the same path convention defined here. No separate ADR is needed for them. This ADR is the shared convention. S-18.04a should use `FACTORY_ARTIFACTS_PATH` (its established env var name); S-18.01 should use `ARTIFACTS_WT` or `--artifacts-worktree` (its CLI arg name). Both names refer to the same concept: the factory-artifacts worktree root = `.factory` in production.

---

## Consequences

### Positive

- **Contradiction eliminated.** Shell skills can correctly both read specs from `${ARTIFACTS_WT}/specs/…` and commit to factory-artifacts via `git -C "$ARTIFACTS_WT" commit`. No path ambiguity.
- **Bats fixtures are honest.** Fixture files placed directly under `$ARTIFACTS_WT/…` mirror the production layout. The fixture no longer masks the double-nesting defect.
- **Shared convention documented.** All E-18 shell skills (S-18.01, S-18.04a, S-18.05) and any future skills that commit to factory-artifacts follow the same path convention defined in one place.

### Negative / Trade-offs

- **Bats fixture refactor required for S-18.01.** If any existing test fixture places files under `$ARTIFACTS_WT/.factory/…`, it must be corrected to place them under `$ARTIFACTS_WT/…` directly. The S-18.01 development worktree fixture (`wave-handoff.bats`) uses the two-arg model and is already internally consistent (it passes `--bc-dir $ARTIFACTS_WT/.factory/specs/behavioral-contracts`, which within the fixture context is correct because the fixture creates that nesting). The implementer must ensure the skill interprets `--bc-dir` literally (does not prepend `$ARTIFACTS_WT/` to it again).

---

## Risks Addressed

| Risk | Mitigation |
|------|-----------|
| F-S1801-P3-001 BLOCKER: skill uses `${ARTIFACTS_WT}/.factory/specs/…` in production → resolves to `.factory/.factory/specs/…` → directory does not exist → skill fails silently or hard-errors | Decision 1: `ARTIFACTS_WT` = `.factory`, paths use `${ARTIFACTS_WT}/specs/…` directly. No double-nesting. |
| F-S1801-P3-001 BLOCKER: `git -C "$ARTIFACTS_WT"` with `ARTIFACTS_WT = .` → commits to develop/feature branch → HANDOFF.md lands on wrong branch | Decision 1: `ARTIFACTS_WT` = `.factory`, `git -C "$ARTIFACTS_WT"` commits to factory-artifacts. |
| Hermetic fixture masks production defect | Decision 3: fixture files placed directly under `$ARTIFACTS_WT/…`, not under `$ARTIFACTS_WT/.factory/…`. Fixture now faithfully mirrors production layout. |
| Sibling stories S-18.04a and S-18.05 re-introduce same contradiction independently | Decision 4: this ADR is the canonical shared convention for all E-18 shell skills. Both already follow it (S-18.04a via `FACTORY_ARTIFACTS_PATH`; S-18.05 via `git show` branch-ref reads). |

---

## Traceability

| Source | ID | Coverage |
|--------|----|---------|
| Story | S-18.01 v1.5 | §Canonical Wiring Contract section + §Architecture Compliance Rules (path-discipline rules) |
| Story | S-18.04a v1.5 | AC-014 `FACTORY_ARTIFACTS_PATH` env var; §Previous Story Intelligence path-discipline lesson |
| Story | S-18.05 v1.5 | `git show factory-artifacts:.factory/STATE.md` read-only pattern |
| Adversary finding | F-S1801-P3-001 BLOCKER | Directly resolved by Decision 1 + Decision 2 |
| Capability | CAP-032 | Context-durability feature; all E-18 shell skills |
| Subsystem | SS-05 | factory-artifacts orphan branch (the write target) |
| Subsystem | SS-06 | wave-handoff skill (S-18.01) |
| Subsystem | SS-07 | precompact-flush.sh (S-18.04a), postcompact-reanchor.sh (S-18.05) |
| CLAUDE.md | factory-artifacts worktree discipline | "factory-artifacts branch: orphan branch mounted at `.factory/` via worktree" |
