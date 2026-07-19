---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-07-19T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - plugins/vsdd-factory/hooks/factory-branch-guard.sh
  - plugins/vsdd-factory/hooks-registry.toml
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-04"
capability: "TBD — E-21 CAP pending ARCH-INDEX registration by architect"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.16.001
section: "4.16"
last_amended: "2026-07-19 (v1.0) — Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #342). validate-artifact-path WASM PreToolUse guard: blocks `git add` of `.factory/` paths on non-factory-artifacts branches; INV-E21-001 instantiation (invariant layer). lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge)."
---

# BC-4.16.001: validate-artifact-path WASM PreToolUse guard MUST block any `git add` command that stages a path under `.factory/` on a product branch, and MUST pass all non-`.factory/` staging commands unconditionally

## Description

The `validate-artifact-path` native-WASM plugin enforces the nested-worktree path-exclusivity
invariant (INV-E21-001): no path under the `.factory/` physical directory may be tracked on a
product branch (`develop`, `main`, `feature/*`, `release/*`, `maintenance/*`). It fires on every
`PreToolUse` event where the Bash tool payload contains a `git add` command. On invocation, the
plugin inspects the staged paths in the command: if any path matches `.factory/**` (or `\.factory/`
as a component) AND the current working branch is a product branch (not `factory-artifacts`), the
plugin emits `block_intent = true` (exit code 2) with an actionable error. All other `git add`
commands and all non-`git add` Bash commands pass unconditionally.

This is the **invariant layer** for issue #342. It prevents the dual-tracking condition that makes
product-branch merges dangerous: if no `.factory/` path is ever staged on a product branch, the
git working-tree update triggered by a product-branch merge/checkout cannot delete a path the
nested `.factory/` worktree is serving. The companion **safety-net layer** (BC-5.43.001) intercepts
product-branch merges that would clobber `.factory/` paths even if the invariant layer is bypassed
or preceded by a prior accidental staging.

This BC governs the planned `validate-artifact-path` WASM crate already listed as PLANNED in the
SS-04 subsystem (`crates/hook-plugins/validate-artifact-path/`). POLICY 21 (no new shell scripts)
is satisfied: the guard is a native WASM plugin, not a `.sh` file.

## Preconditions

1. A `PreToolUse` event has fired for the `Bash` tool.

2. The Bash tool payload contains a `git add` command (detected by matching the payload against
   `git\s+add` as a substring, case-insensitive). All other Bash payloads — including `git commit`,
   `git push`, `git checkout`, `git merge`, and all non-git commands — are NOT in scope for this
   plugin and pass unconditionally.

3. The dispatcher has invoked the `validate-artifact-path` WASM plugin. The plugin is registered
   in `hooks-registry.toml` as a PreToolUse handler for `^Bash$` with `on_error = "continue"`
   (fail-open: a crashed or timed-out guard never blocks the session).

## Postconditions

### PC1 — Blocked: `git add` of `.factory/` path on a product branch

When the Bash payload contains `git add` AND at least one argument (expanded path or glob) resolves
to a path matching `^\.factory/` (or containing `/.factory/` as a path component, capturing the
unnested form), AND the guard's branch detection reports a product branch (any branch other than
`factory-artifacts`):

1. The plugin returns `block_intent = true` (exit code 2).
2. The plugin emits a blocking error message:
   ```
   BLOCKED: git add of .factory/ path on product branch '<branch>'.
   .factory/ paths are exclusively owned by the factory-artifacts worktree.
   Staging .factory/ content on a product branch creates the dual-tracking condition
   that allows product-branch merges to silently delete factory artifact files.
   If you intended to commit factory artifacts: switch to the .factory/ worktree and
   commit from there on the factory-artifacts branch.
   ```
3. The `git add` command is NOT executed.

**Error variant:** `FactoryPathOnProductBranch`

### PC2 — Passed: `git add` of non-`.factory/` paths

When the Bash payload contains `git add` AND no argument resolves to a `.factory/`-rooted path,
the plugin returns `block_intent = false` (exit code 0). The `git add` proceeds normally.

### PC3 — Passed: `git add` on `factory-artifacts` branch

When the Bash payload contains `git add` AND the current branch is `factory-artifacts`, the plugin
MUST pass unconditionally regardless of the paths being staged. Factory artifact commits require
staging `.factory/` paths on the `factory-artifacts` branch — that is correct behavior. The plugin
MUST NOT block legitimate factory-artifacts branch staging.

### PC4 — Passed: non-`git add` Bash command

When the Bash payload does NOT contain a `git add` command (regardless of paths), the plugin returns
`block_intent = false` (exit code 0) immediately without path inspection.

## Invariants

1. **INV-E21-001 (Nested Worktree Path Exclusivity) — instantiation:** No `git add` command
   executed via the Bash tool on a product branch may stage a path under `.factory/`. This invariant
   prevents the dual-tracking condition: `.factory/` paths tracked on a product branch are a P0
   data-loss landmine (issue #342 root cause).

2. **Fail-open on crash:** A crashed or timed-out `validate-artifact-path` plugin MUST return
   `block_intent = false` (pass-through). The guard MUST be registered with `on_error = "continue"`.
   A broken guard is disruptive but never wedges the session.

3. **Branch detection source:** The plugin detects the current product branch via `host::exec_subprocess`
   (`git branch --show-current` or equivalent). If branch detection fails (git unavailable, detached
   HEAD, non-zero exit), the plugin MUST fail-open (pass-through). Uncertain branch state is NOT
   a blocking condition.

4. **Path matching is conservative:** The path pattern `.factory/` is matched as a literal path
   prefix or path component. Glob expansions in the `git add` argument are NOT evaluated by the
   plugin at PreToolUse time (git has not yet run); the plugin inspects only the literal argument
   text. A `git add .` or `git add -A` command from a working directory whose CWD is under
   `.factory/` IS caught by this guard (the `.` or `-A` is treated as a potential `.factory/`-rooted
   staging if the detected branch is a product branch — the guard blocks with a conservative
   message).

5. **No overlap with BC-4.13.001:** `verify-factory-lock` (BC-4.13.001) guards against concurrent
   write races on Edit/Write/MultiEdit/Agent tool calls using the lock mechanism. This plugin
   guards against a different surface (Bash-tool `git add` of `.factory/` paths on product branches).
   Both plugins may fire on the same dispatch; they are additive guards, not substitutes.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `git add .factory/STATE.md` on `develop` branch | BLOCKED: PC1 fires; `FactoryPathOnProductBranch` error emitted |
| EC-002 | `git add .factory/STATE.md` on `factory-artifacts` branch | PASSED: PC3 fires unconditionally |
| EC-003 | `git add src/main.rs` on `develop` branch | PASSED: PC2 fires; no `.factory/` path |
| EC-004 | `git add -A` run from inside `.factory/` directory (CWD = `.factory/`) | BLOCKED: plugin detects `-A` or `.` as potentially staging CWD-relative paths under `.factory/`; conservative block with PC1 error |
| EC-005 | `git add src/main.rs .factory/STATE.md` (mixed) on `develop` | BLOCKED: ANY `.factory/` match is sufficient to block the whole command |
| EC-006 | Branch detection fails (detached HEAD) | PASSED: fail-open per Invariant 3; git add proceeds |
| EC-007 | Plugin crashes or times out | PASSED: fail-open per `on_error = "continue"` (Invariant 2) |
| EC-008 | `git add` with glob `*.md` from project root on `develop` | BLOCKED: conservative — glob may expand to `.factory/**/*.md`; plugin blocks; user must re-run with explicit non-`.factory/` paths |
| EC-009 | `git commit --amend` (no `git add`) | PASSED: PC4 fires; non-`git add` command |
| EC-010 | `git add -u` on `feature/S-21.01` branch | BLOCKED: `-u` stages all tracked modifications; conservative block since CWD or tracked files may include `.factory/` paths |

## Canonical Test Vectors

| Test # | Precondition | Command (Bash payload) | Expected Result |
|--------|-------------|------------------------|----------------|
| T-1 | Branch = `develop`; path = `.factory/STATE.md` | `git add .factory/STATE.md` | BLOCKED: `FactoryPathOnProductBranch` error |
| T-2 | Branch = `factory-artifacts`; path = `.factory/STATE.md` | `git add .factory/STATE.md` | PASSED: PC3 |
| T-3 | Branch = `feature/S-21.01`; path = `src/lib.rs` | `git add src/lib.rs` | PASSED: PC2 |
| T-4 | Branch = `develop`; payload = `git add -A` | `git add -A` | BLOCKED: conservative PC1 |
| T-5 | Branch = `release/v1.0.0-rc.24`; path = `.factory/stories/S-21.01.md` | `git add .factory/stories/S-21.01.md` | BLOCKED: PC1 (release branch is a product branch) |
| T-6 | Plugin crashes mid-invocation | `git add src/main.rs` | PASSED: fail-open |

## SDK Grounding Evidence

**Grep 1 — WASM crate stub location (PLANNED, to be created by S-21.01):**
```
ls crates/hook-plugins/validate-artifact-path/ 2>/dev/null || echo "PLANNED — not yet created"
```
Expected: "PLANNED — not yet created" at authoring time; crate created by S-21.01 implementation.

**Grep 2 — factory-branch-guard.sh scope (does NOT guard Bash git add):**
```
grep -n "git add" plugins/vsdd-factory/hooks/factory-branch-guard.sh
```
Expected: no hits — confirms the existing guard does not intercept `git add` on Bash tool calls,
validating the gap this BC closes.

**Grep 3 — hooks-registry.toml PLANNED entry:**
```
grep -n "validate-artifact-path" plugins/vsdd-factory/hooks-registry.toml
```
Expected: PLANNED entry present (confirms SS-04 intent) or absent (to be added by S-21.01).

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned by state-manager after VP authoring pass) | Block fires on `git add .factory/X` when branch != `factory-artifacts` | bats: invoke guard with mocked branch=develop; payload=`git add .factory/STATE.md`; assert exit 2 + `FactoryPathOnProductBranch` |
| (TBD) | Pass unconditionally when branch == `factory-artifacts` | bats: invoke guard with branch=factory-artifacts; payload=`git add .factory/STATE.md`; assert exit 0 |
| (TBD) | Fail-open on crash | unit: force panic in guard body; assert dispatcher continues with `block_intent=false` |
| (TBD) | Path-pattern coverage: proptest | proptest: fuzz .factory/ path variants (with components, root-relative, abs path); assert block fires on all |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD — E-21 CAP pending ARCH-INDEX registration |
| Capability Anchor Justification | New capability for INV-E21-001 (Nested Worktree Path Exclusivity); no existing CAP covers product-branch `git add` interception of `.factory/` paths. Requires architect to register in ARCH-INDEX and capabilities.md. |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `crates/hook-plugins/validate-artifact-path/` (PLANNED; created by S-21.01) |
| Stories | S-21.01 (E-21 Wave 1) |
| Source Issues | #342 (product-branch merge silently rm's a `.factory/` file) |
| ADR Reference | none (fix is structural; no new ADR required per architect delta analysis) |

## Related BCs

- BC-4.13.001 — sibling guard on same SS-04 surface; governs Edit/Write/MultiEdit/Agent lock enforcement; orthogonal (different tool surface, different invariant)
- BC-5.43.001 — safety-net layer companion; governs orchestrator pre-merge intersection check that defends against dual-tracking even if this guard was bypassed (issue #342 defense-in-depth)

## Architecture Anchors

- `crates/hook-plugins/validate-artifact-path/` — WASM plugin source (PLANNED; to be created by S-21.01)
- `plugins/vsdd-factory/hooks-registry.toml` — registry entry for `validate-artifact-path` (to be added by S-21.01)
- `plugins/vsdd-factory/hooks/factory-branch-guard.sh` — existing guard that covers Edit/Write surface but NOT Bash `git add`; grandfathered per E-20 scope; this BC closes the `git add` gap without touching that script

## Story Anchor

S-21.01 (E-21 Wave 1 — factory artifact path guard: prevent dual-tracking and intercept product-branch merges that would clobber `.factory/` paths)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #342). validate-artifact-path WASM PreToolUse Bash guard — block `git add` of `.factory/` paths on product branches (PC1), pass all other commands (PC2/PC3/PC4). INV-E21-001 instantiation (invariant layer). 4 error variants: `FactoryPathOnProductBranch` (PC1), fail-open (Invariants 2/3). 10 edge cases EC-001..EC-010. 6 canonical test vectors T-1..T-6. lifecycle_status: draft (POL-14 auto-promotion on S-21.01 PR merge). |
