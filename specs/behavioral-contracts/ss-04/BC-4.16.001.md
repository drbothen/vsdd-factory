---
document_type: behavioral-contract
level: L3
version: "1.1"
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
capability: "CAP-034"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-034 backfill + crate name correction (product-owner; ADR-031 §Decision 3): capability frontmatter TBD→CAP-034; all 10 occurrences of wrong crate name `validate-artifact-path` replaced with `validate-factory-path-staging` (TD-VSDD-060 sibling-site sweep: H1 title, §Description, Precondition 3, Invariants 2/4, SDK Grounding Evidence Grep 1/3, §Traceability Architecture Module, §Architecture Anchors ×2); ADR Reference row added; Capability Anchor Justification updated to cite CAP-034/ADR-031; BC-4.11.001 note added to §Related BCs."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.16.001
section: "4.16"
last_amended: "(v1.1) — CAP-034 backfill + crate name correction (product-owner; ADR-031 §Decision 3): capability frontmatter TBD→CAP-034; all 10 occurrences of wrong crate name `validate-artifact-path` replaced with `validate-factory-path-staging`; ADR-031 §Decision 3 ADR Reference added; Capability Anchor Justification updated; §Related BCs BC-4.11.001 note added. TD-VSDD-060 sibling-site sweep complete. [Prior: (v1.0) — Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #342). validate-factory-path-staging WASM PreToolUse guard: blocks `git add` of `.factory/` paths on non-factory-artifacts branches; INV-E21-001 instantiation (invariant layer). lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge).]"
---

# BC-4.16.001: validate-factory-path-staging WASM PreToolUse guard MUST block any `git add` command that stages a path under `.factory/` on a product branch, and MUST pass all non-`.factory/` staging commands unconditionally

## Description

The `validate-factory-path-staging` native-WASM plugin enforces the nested-worktree path-exclusivity
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

This BC governs the `validate-factory-path-staging` WASM crate at
`crates/hook-plugins/validate-factory-path-staging/` (new crate to be created by S-21.01; named
per ADR-031 §Decision 3 to avoid collision with `crates/hook-plugins/validate-artifact-path/`
which serves BC-4.11.001 since S-13.01). POLICY 21 (no new shell scripts) is satisfied: the guard
is a native WASM plugin, not a `.sh` file.

## Preconditions

1. A `PreToolUse` event has fired for the `Bash` tool.

2. The Bash tool payload contains a `git add` command (detected by matching the payload against
   `git\s+add` as a substring, case-insensitive). All other Bash payloads — including `git commit`,
   `git push`, `git checkout`, `git merge`, and all non-git commands — are NOT in scope for this
   plugin and pass unconditionally.

3. The dispatcher has invoked the `validate-factory-path-staging` WASM plugin. The plugin is
   registered in `hooks-registry.toml` as a PreToolUse handler for `^Bash$` with
   `on_error = "continue"` (fail-open: a crashed or timed-out guard never blocks the session).

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

2. **Fail-open on crash:** A crashed or timed-out `validate-factory-path-staging` plugin MUST
   return `block_intent = false` (pass-through). The guard MUST be registered with
   `on_error = "continue"`. A broken guard is disruptive but never wedges the session.

3. **Branch detection source:** The plugin detects the current product branch via `host::exec_subprocess`
   (`git branch --show-current` or equivalent). If branch detection fails (git unavailable, detached
   HEAD, non-zero exit), the plugin MUST fail-open (pass-through). Uncertain branch state is NOT
   a blocking condition.

4. **Path matching is conservative:** The path pattern `.factory/` is matched as a literal path
   prefix or path component. Glob expansions in the `git add` argument are NOT evaluated by the
   `validate-factory-path-staging` plugin at PreToolUse time (git has not yet run); the plugin
   inspects only the literal argument text. A `git add .` or `git add -A` command from a working
   directory whose CWD is under `.factory/` IS caught by this guard (the `.` or `-A` is treated as
   a potential `.factory/`-rooted staging if the detected branch is a product branch — the guard
   blocks with a conservative message).

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

**Grep 1 — WASM crate stub location (to be created by S-21.01):**
```
ls crates/hook-plugins/validate-factory-path-staging/ 2>/dev/null || echo "PLANNED — not yet created"
```
Expected: "PLANNED — not yet created" at authoring time; crate created by S-21.01 implementation.

**Grep 2 — factory-branch-guard.sh scope (does NOT guard Bash git add):**
```
grep -n "git add" plugins/vsdd-factory/hooks/factory-branch-guard.sh
```
Expected: no hits — confirms the existing guard does not intercept `git add` on Bash tool calls,
validating the gap this BC closes.

**Grep 3 — hooks-registry.toml entry for validate-factory-path-staging:**
```
grep -n "validate-factory-path-staging" plugins/vsdd-factory/hooks-registry.toml
```
Expected: entry present (added by S-21.01) or absent before implementation.

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
| L2 Capability | CAP-034 |
| Capability Anchor Justification | CAP-034 (Nested Worktree Path Exclusivity) per ARCH-INDEX v3.07 / ADR-031 §Decision 3. This BC is the primary enforcement mechanism for CAP-034's invariant layer: the `validate-factory-path-staging` WASM guard blocks the `git add` surface that creates the dual-tracking condition. BC-5.43.001 is the safety-net layer of the same CAP-034 capability. |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `crates/hook-plugins/validate-factory-path-staging/` (new crate; to be created by S-21.01; ADR-031 §Decision 3 naming authority) |
| Stories | S-21.01 (E-21 Wave 1) |
| Source Issues | #342 (product-branch merge silently rm's a `.factory/` file) |
| ADR Reference | ADR-031 §Decision 3 (authoritative crate naming: `validate-factory-path-staging.wasm`; `crates/hook-plugins/validate-artifact-path/` serves BC-4.11.001 Edit/Write/MultiEdit path validation since S-13.01 and cannot be reused — registry filename + cargo output collisions) |

## Related BCs

- BC-4.13.001 — sibling guard on same SS-04 surface; governs Edit/Write/MultiEdit/Agent lock enforcement; orthogonal (different tool surface, different invariant)
- BC-4.11.001 — the existing BC that owns `crates/hook-plugins/validate-artifact-path/` (Edit/Write/MultiEdit path validation since S-13.01); NOT the same crate as this BC; naming distinction enforced by ADR-031 §Decision 3
- BC-5.43.001 — safety-net layer companion; governs orchestrator pre-merge intersection check that defends against dual-tracking even if this guard was bypassed (issue #342 defense-in-depth)

## Architecture Anchors

- `crates/hook-plugins/validate-factory-path-staging/` — WASM plugin source (new crate; to be created by S-21.01; ADR-031 §Decision 3 naming authority)
- `plugins/vsdd-factory/hooks-registry.toml` — registry entry for `validate-factory-path-staging` (to be added by S-21.01)
- `plugins/vsdd-factory/hooks/factory-branch-guard.sh` — existing guard that covers Edit/Write surface but NOT Bash `git add`; grandfathered per E-20 scope; this BC closes the `git add` gap without touching that script

## Story Anchor

S-21.01 (E-21 Wave 1 — factory artifact path guard: prevent dual-tracking and intercept product-branch merges that would clobber `.factory/` paths)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.1 | 2026-07-19 | CAP-034 backfill + crate name correction (product-owner; ADR-031 §Decision 3). All 10 occurrences of wrong crate name `validate-artifact-path` replaced with `validate-factory-path-staging` (TD-VSDD-060 sibling-site sweep: H1 title, §Description ×2, Precondition 3, Invariant 2/4 ×2, SDK Grounding Evidence Grep 1/3, §Traceability Architecture Module, §Architecture Anchors ×2). §Traceability ADR Reference row added. §Traceability Capability Anchor Justification updated to cite CAP-034 / ARCH-INDEX v3.07 / ADR-031 §Decision 3. §Related BCs: BC-4.11.001 note added clarifying `validate-artifact-path` crate ownership. |
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #342). validate-factory-path-staging WASM PreToolUse Bash guard — block `git add` of `.factory/` paths on product branches (PC1), pass all other commands (PC2/PC3/PC4). INV-E21-001 instantiation (invariant layer). 4 error variants: `FactoryPathOnProductBranch` (PC1), fail-open (Invariants 2/3). 10 edge cases EC-001..EC-010. 6 canonical test vectors T-1..T-6. lifecycle_status: draft (POL-14 auto-promotion on S-21.01 PR merge). |
