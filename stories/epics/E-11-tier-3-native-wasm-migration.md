---
document_type: epic
epic_id: "E-11"
version: "1.0"
title: "Tier 3 Native WASM Migration (W-17) — PreToolUse protections + process discipline"
status: draft
tech_debt_ref: TD-014
prd_capabilities: [CAP-002, CAP-008, CAP-013, CAP-022]
prd_frs: []
anchor_strategy: rewrite-clean-per-ADR-014-D-9.1
priority: P2
target_release: "v1.3 (Tier 3)"
story_count: 8
subsystems_affected: [SS-04, SS-07]
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 2
traces_to: .factory/tech-debt-register.md#TD-014
depends_on: ["E-8", "E-9"]
last_amended: "2026-05-06 (v1.0 initial authoring — 10 orphan hooks anchored)"
inputs:
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/epics/E-9-tier-2-native-wasm-migration.md
  - .factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/SS-07-hook-bash.md
  - .factory/specs/prd.md
  - .factory/stories/STORY-INDEX.md
input-hash: "TBD"
---
<!-- [process-gap] Frontmatter fields tech_debt_ref, anchor_strategy, depends_on extend the canonical epic-template baseline (same as E-8 v1.9 / E-9 v1.0). Template update tracked as follow-up. -->

# Epic E-11: Tier 3 Native WASM Migration (W-17) — PreToolUse protections + process discipline

## Description

Port the 10 remaining bash hooks (8 Tier 3 PreToolUse protection hooks + 2 Tier 2
residual PostToolUse hooks) from `legacy-bash-adapter.wasm` routing to native WASM
crates using the rewrite-clean strategy (ADR-014 D-9.1). These hooks fire before and
after tool execution respectively; 8 emit `permissionDecision: deny` on violation —
making behavioral correctness safety-critical. An over-blocking native port bricks the
dev loop; an under-blocking port silently leaks protected artifacts. E-11 is the final
WASM migration wave (W-17); upon completion, the `legacy-bash-adapter` crate is deleted
and TD-014 is closed.

> **Orphan provenance:** E-8 v1.10 CHANGELOG (2026-05-03) retired S-8.20–S-8.27 with
> `superseded_by: future E-10`. E-10's slot was subsequently reallocated to ADR-015
> single-stream OTel emission per D-236 (2026-05-04). The 8 Tier 3 hooks therefore
> had no active epic anchor. Two Tier 2 residuals (convergence-tracker, purity-check)
> were assigned to E-8 S-8.18 then dropped from E-9 because E-9 is `validate-*.sh`-only.
> E-11 anchors all 10 orphan hooks and supersedes the stale "future E-10" pointers.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-002 | Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins | P0 |
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-013 | Capture post-execution activity (PostToolUse hooks) | P0 |
| CAP-022 | Port hook plugins from bash to native WASM | P2 |

## Capability Anchor Justification

**Primary anchor:** CAP-022 ("Port hook plugins from bash to native WASM") per
`domain-spec/capabilities.md` §CAP-022. E-11 is the W-17 Tier 3 cluster within the
CAP-022 migration: porting 8 PreToolUse protection hooks + 2 PostToolUse process hooks
from bash to native WASM. Without E-11, these hooks continue through
`legacy-bash-adapter.wasm`, preventing Phase H adapter deletion and blocking TD-014 closure.

**CAP-008 (PreToolUse gate) anchor:** 8 of the 10 hooks fire on PreToolUse events and
emit `permissionDecision: deny` to block tool execution. These are the primary CAP-008
surface in the factory's hook stack; their native port is required to make block-mode
semantics ABI-stable.

**CAP-013 (PostToolUse) anchor:** convergence-tracker and purity-check fire on
PostToolUse:Edit|Write. Porting these closes the last gap in the PostToolUse class
(E-9 closed the 23 `validate-*.sh` hooks; E-11 closes these 2 residuals).

**SS-04 (Plugin Ecosystem) anchor:** SS-04 owns `crates/hook-plugins/`; all 10 new
WASM crates are SS-04 artifacts.

**SS-07 (Hook Bash Layer) partial anchor:** SS-07 owns the `.sh` files and registry
entries being migrated. Post-E-11, SS-07 contains only the `verify-git-push.sh`
exception (D-1) and residual bash files pending Phase H deletion (R-W16-001).

## Stories

| Story ID | Title | Points | Depends On | Blocks | Status |
|----------|-------|--------|-----------|--------|--------|
| S-11.00 | Perf baseline + W-17 bundle ceiling (reuse W-16 model) | TBD | E-9 W-16 closure | S-11.01..S-11.06 | draft |
| S-11.01 | Batch B-1: artifact guard hooks (protect-bc, protect-vp) | TBD | S-11.00 | — | draft |
| S-11.02 | Batch B-2: process-discipline PostToolUse hooks (convergence-tracker, purity-check) | TBD | S-11.00 | — | draft |
| S-11.03 | Batch B-3: identity guard hooks (brownfield-discipline, factory-branch-guard) | TBD | S-11.00 | — | draft |
| S-11.04 | Native port: protect-secrets (dual-event PreToolUse:Bash+Read) | TBD | S-11.00 | — | draft |
| S-11.05 | Native port: check-factory-commit (PreToolUse:Bash, git log) | TBD | S-11.00 | — | draft |
| S-11.06 | Native port: red-gate (High complexity, state-file + path matching) | TBD | S-11.00 | — | draft |
| S-11.07 | Native port: destructive-command-guard (High complexity, command tokenization) + adapter retirement | TBD | S-11.01..S-11.06 | — | draft |

**Story count: 8** (S-11.00 + S-11.01..S-11.07)

> **Note on S-11.07:** destructive-command-guard is the last Tier 3 hook and also the
> adapter retirement story. Like S-8.09 for Tier 1 and S-9.07 for Tier 2, the final
> story in the wave performs the pre-deletion audit (zero `legacy-bash-adapter.wasm`
> references in hooks-registry.toml) and deletes the adapter crate. This closes TD-014.

> Story-writer authors S-11.01..S-11.07 in subsequent bursts following adversarial
> convergence per ADR-013.

## Hook Inventory (10 hooks)

### Batch B-1 (S-11.01): Artifact guard hooks (PreToolUse:Edit|Write, block)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| protect-bc.sh | PreToolUse:Edit\|Write | block (permissionDecision: deny) | `.factory/specs/behavioral-contracts/` | BC-7.03.049, BC-7.03.050, BC-7.03.051, BC-7.03.052 | Medium |
| protect-vp.sh | PreToolUse:Edit\|Write | block (permissionDecision: deny) | `.factory/specs/verification-properties/` | BC-7.03.060, BC-7.03.061 | Medium |

### Batch B-2 (S-11.02): Process-discipline PostToolUse hooks (PostToolUse:Edit|Write, continue)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| convergence-tracker.sh | PostToolUse:Edit\|Write | continue | `.factory/` | BC-7.03.020, BC-7.03.021, BC-7.03.022, BC-7.03.023, BC-7.03.024, BC-7.03.025 | Medium |
| purity-check.sh | PostToolUse:Edit\|Write | continue | `.` (reads source files outside .factory/) | BC-7.03.062, BC-7.03.063, BC-7.03.064 | Medium |

> **Tier 2 residuals:** convergence-tracker and purity-check were originally assigned
> to E-8 S-8.18 (bundle B-6a), then excluded from E-9 because E-9 is `validate-*.sh`-only.
> They are PostToolUse:Edit|Write continue-mode hooks — the same semantics as Tier 2,
> but their on-disk bash source is not named `validate-*.sh`. They are ported here
> rather than retrofitting E-9's scope. Block-mode ACs (per E-8 AC-8 pattern) do NOT
> apply to these two hooks since `on_error = "continue"`.

### Batch B-3 (S-11.03): Identity guard hooks (PreToolUse:Edit|Write, block)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| brownfield-discipline.sh | PreToolUse:Edit\|Write | block (permissionDecision: deny) | `.reference/`, `.` (broad path check) | BC-7.03.006, BC-7.03.007, BC-7.03.008 | Medium |
| factory-branch-guard.sh | PreToolUse:Edit\|Write | block (permissionDecision: deny) | `.factory/`, `.git/` (git branch check) | BC-7.03.038, BC-7.03.039, BC-7.03.040, BC-7.03.041 | Medium |

### Solo S-11.04: protect-secrets (dual-event, PreToolUse:Bash+Read, block)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| protect-secrets.sh | PreToolUse:Bash + PreToolUse:Read | block (permissionDecision: deny) | `.` (broad; reads FILE_PATH from any location) | BC-7.03.053, BC-7.03.054, BC-7.03.055, BC-7.03.056, BC-7.03.057, BC-7.03.058, BC-7.03.059 | Medium (dual-event registration) |

> **Dual-event note:** protect-secrets registers two `[[hooks]]` entries in
> hooks-registry.toml — one for Bash events (command string inspection), one for Read
> events (file path inspection). The native WASM plugin handles both via `tool_name`
> field from the hook payload, or via two separate registry entries pointing to the
> same WASM. D-11.6 confirms the dispatcher already handles permissionDecision envelopes
> (pr-manager-completion-guard and validate-pr-review-posted shipped block-mode in E-8).
> protect-secrets counts as 1 plugin, 2 registry entries.

### Solo S-11.05: check-factory-commit (PreToolUse:Bash, block)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| check-factory-commit.sh | PreToolUse:Bash | block (permissionDecision: deny) | `.factory/`, `.git/` (git log invocation) | BC-7.03.018, BC-7.03.019 | Medium (git log via exec_subprocess) |

> **Subprocess note:** check-factory-commit invokes `git log` to determine whether
> recent commits touching `.factory/` include a STATE.md update. This uses
> `host::exec_subprocess` (existing ABI, BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036)
> — no new SDK extension required. See D-11.2.

### Solo S-11.06: red-gate (PreToolUse:Edit|Write, block)

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| red-gate.sh | PreToolUse:Edit\|Write | block (permissionDecision: deny) | `.factory/` (reads wave-state.yaml / red[] list) | BC-7.03.065, BC-7.03.066, BC-7.03.067, BC-7.03.068, BC-7.03.069, BC-7.03.070 | High (state-file YAML parse + 4-way path normalization) |

### Solo S-11.07: destructive-command-guard + adapter retirement

| Hook | Event | On-Error | path_allow | BC Anchor | Complexity |
|------|-------|----------|-----------|-----------|------------|
| destructive-command-guard.sh | PreToolUse:Bash | block (permissionDecision: deny) | none (pure command-string inspection) | BC-7.03.026, BC-7.03.027, BC-7.03.028, BC-7.03.029, BC-7.03.030, BC-7.03.031, BC-7.03.032, BC-7.03.033, BC-7.03.034, BC-7.03.035, BC-7.03.036, BC-7.03.037 | High (shell tokenization, 11+ pattern families, regex coverage) |

> **Adapter retirement:** S-11.07 also performs the pre-deletion audit (zero
> `[[hooks]]` entries use `legacy-bash-adapter.wasm`) and deletes
> `crates/hook-plugins/legacy-bash-adapter/` + removes `bin/emit-event` from the
> dispatcher binary tree. Closes TD-014 and R-W16-001 bats orphan deferred Phase H.

---

## Problem Statement

After W-16 (E-9 / rc.X), 10 bash hooks remain routed through
`legacy-bash-adapter.wasm` in `hooks-registry.toml`. These hooks:

1. **Prevent legacy-bash-adapter deletion (Phase H):** The adapter crate cannot be
   retired until W-17 completes (E-8 D-10). With E-9 shipped, these 10 hooks are
   the sole blocker to Phase H and TD-014 closure.

2. **Carry Windows compatibility debt (DRIFT-010):** All 10 require bash on Windows.
   A mis-installed or absent bash silently no-ops all PreToolUse gates.

3. **Carry heightened safety risk:** 8 of 10 hooks emit block-mode responses
   (`permissionDecision: deny`). Bash execution via the adapter introduces subprocess
   latency and a jq-missing failure mode (jq absence causes silent pass-through for
   most hooks). A native WASM port eliminates the jq dependency and makes failure
   semantics deterministic.

4. **Have stale epic pointers:** S-8.20–S-8.27 were retired with
   `superseded_by: future E-10`. E-10's slot was reallocated to ADR-015 (D-236,
   2026-05-04). These pointers now point to a non-existent anchor. E-11 corrects
   the supersession chain (see D-11.7).

E-11 resolves all four problems by porting all 10 hooks as native WASM crates and
retiring the adapter in S-11.07.

---

## Goals

1. All 10 bash hooks (8 Tier 3 PreToolUse + 2 Tier 2 PostToolUse residuals) ported to
   native WASM plugins under `crates/hook-plugins/<name>/`.
2. `hooks-registry.toml` updated: 11 WASM entries added (protect-secrets = 2 entries);
   10 legacy-bash-adapter entries disabled or removed for Tier 3 hooks.
3. Silent-jq-missing failure mode eliminated: native WASM plugins never silently pass
   due to missing toolchain dependencies.
4. HOST_ABI_VERSION = 1 unchanged throughout E-11 (no new host ABI extension required;
   `host::exec_subprocess` (BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036) is
   sufficient for check-factory-commit's git log use case; see D-11.2).
5. Bundle size growth within the W-17 latency-primary + advisory-ceiling model
   (primary gate: cold-start p95 ≤ 500ms; hard kill-switch: 30MB cumulative; advisory
   soft cap: cumulative ≤ 100% growth vs v1.0.0-rc.1 baseline per ADR-014 R-8.09
   revised model; measured by S-11.00; enforced per-batch).
6. `legacy-bash-adapter` crate deleted at S-11.07 close (Phase H). TD-014 closed.
7. Supersession-pointer drift corrected: STORY-INDEX S-8.20–S-8.27 entries updated
   from `superseded_by: future E-10` to `superseded_by: E-11` by state-manager
   follow-up burst per D-11.7.

---

## Non-Goals

- Porting `verify-git-push.sh` — stays bash per E-8 D-1 (Windows git-bash compatibility
  for git itself; non-negotiable). This exclusion is permanent through E-11.
- Changing any hook's behavior during the port (rewrite-clean but behavior-parity;
  deliberate simplifications documented per ADR-014 D-9.1 mitigation).
- Adding new host ABI extensions beyond existing `host::exec_subprocess` (see D-11.2).
- Deleting `.sh` bash hook files — Phase H only (R-W16-001); bats orphan cleanup deferred.
- Modifying hooks.json direct-command entries (only hooks-registry.toml changes in E-11;
  the single dispatcher entry per (event, matcher) tuple was established by E-8 D-7).

---

## Decisions

### D-11.1: Port Strategy — rewrite-clean (ADR-014 D-9.1)

Inherited from ADR-014 D-9.1. All 10 hooks ported as idiomatic Rust using `regex`,
`serde_json`, `serde_yaml` where applicable. No 1:1 bash-to-Rust translation. Each
story spec must enumerate all behavioral edge cases from the bash original as explicit
ACs. The WASM Rust integration tests are the authoritative behavioral contract, not
bats (which tests bash execution only).

**Tier 3-specific implication:** block-mode hooks require additional rigor. Each story
spec MUST enumerate both the block path (correct deny) AND the false-block path (deny
on valid input). A false-block regression is immediately visible as a broken dev loop —
heightened acceptance criteria, not just a latency regression.

### D-11.2: Subprocess Capability — existing host::exec_subprocess (no new ABI)

No new SDK extension required. `host::exec_subprocess` (BC-1.05.001..034 + BC-1.05.035
+ BC-1.05.036, per ADR-014 Amendment 2026-05-03) is sufficient for all Tier 3 use cases.

Hooks requiring subprocess calls:
- `check-factory-commit.sh` — invokes `git log` to inspect recent commits. Uses
  `exec_subprocess` with `binary_allow = ["git"]`, `shell_bypass_acknowledged`, and
  `cwd_allow = ["."]`.
- All other 9 hooks — pure Rust logic (regex pattern matching, YAML parse, file read);
  no subprocess calls.

`host::run_subprocess` was withdrawn in ADR-014 Amendment D-9.2. That withdrawal
applies equally to W-17. HOST_ABI_VERSION stays at 1.

### D-11.3: Story Granularity — batched + solo by semantics

Mixed strategy: two batches where hooks share event type, failure mode, and implementation
patterns; four solos for hooks whose complexity or event semantics require individual focus.

| Story | Hooks | Rationale |
|-------|-------|-----------|
| S-11.00 | (perf baseline) | pre-work; confirms W-16 perf model extends to W-17 |
| S-11.01 | protect-bc, protect-vp | both PreToolUse:Edit\|Write block-mode; near-identical logic (BC frontmatter lifecycle_status check) |
| S-11.02 | convergence-tracker, purity-check | both PostToolUse:Edit\|Write continue-mode; Tier 2 residuals with homogeneous pattern |
| S-11.03 | brownfield-discipline, factory-branch-guard | both PreToolUse:Edit\|Write block-mode; identity guard semantics (path/branch allowlist) |
| S-11.04 | protect-secrets | dual-event (Bash+Read) registration is unique; warrants solo story |
| S-11.05 | check-factory-commit | git subprocess dependency requires exec_subprocess ABI; warrants solo story |
| S-11.06 | red-gate | High complexity; state-file YAML parse + 4-way path normalization; warrants solo |
| S-11.07 | destructive-command-guard + adapter retirement | High complexity; also carries Phase H adapter deletion work |

**Story count: 8** (S-11.00 + S-11.01..S-11.07).

### D-11.4: BC Anchor Strategy — reuse existing BC-7.xx family; no new BCs expected

Mirrors E-9 D-9.4 (which mirrors E-8 D-2 Option C). All 10 hooks have existing BC-7.03.xxx
anchors in BC-INDEX.md (confirmed by BC-INDEX survey 2026-05-06). The existing anchors
cover the bash implementation behavioral obligations. The WASM port must satisfy the same
behavioral contracts.

**Block-mode envelope semantics:** BC-7.02.002 covers exit code semantics for the bash
layer (exit 2 = block). Native WASM ports use `HookResult::Block` + dispatcher-emitted
`permissionDecision: deny` instead of exit 2. This is an implementation-layer difference,
not a behavioral contract difference — the observable outcome (tool call blocked) is
identical. Existing BC-7.03.xxx anchors remain valid for the WASM port.

**New BCs may be needed if:** (a) a port reveals an unspecified edge case in the bash
original (bash exit-code ambiguity, error path differences), OR (b) the block-mode
deny-path postconditions are not explicitly covered in existing BCs. Story-writer
identifies BC gaps during per-story decomposition. New BCs, if needed, are authored
under the existing BC-7.xx sub-family (not a new BC-7.04.xxx migration family).

**Block-mode BCs already shipping (E-8 precedent):** `pr-manager-completion-guard`
(BC-7.03.045..048) and `validate-pr-review-posted` (E-8 Tier 1) ship with block-mode
semantics in the existing dispatcher. The dispatcher's `permissionDecision` envelope
handling is verified. E-11 reuses this verified path.

### D-11.5: bats Orphan Strategy — checklist per story (inherits R-W16-001)

Each W-17 story spec must include a task to document the bats deletion checklist for
the story's hooks' corresponding bats tests. The `.sh` files and bats tests remain on
disk until Phase H (per R-W16-001 mitigation). No bats tests are deleted in W-17 stories,
except that S-11.07 triggers Phase H as part of adapter retirement.

### D-11.6: Block-Mode Envelope Semantics — dispatcher already handles; no new work

All 8 PreToolUse block-mode hooks in E-11 emit `permissionDecision: deny` via
`HookResult::Block`. The dispatcher already handles this envelope — `validate-pr-review-posted`
and `pr-manager-completion-guard` (E-8 Tier 1) ship with block-mode semantics, and the
dispatcher's `vsdd.block.plugin_blocked.v1` event is emitted automatically (ADR-015 D-15.3).
No dispatcher changes are required for E-11.

**Verification:** Existing BCs for block-mode Tier 1 hooks (BC-7.03.045..048 for
pr-manager-completion-guard) serve as the implementation pattern reference. Each E-11
block-mode story's ACs must include at minimum one negative test fixture (false-block
scenario) per hook, matching E-9 AC-8 pattern and E-8 AC-8 precedent.

### D-11.7: Supersession-Pointer Correction (state-manager follow-up burst)

STORY-INDEX entries S-8.20–S-8.27 currently read `superseded_by: future E-10`. This
pointer is stale: E-10 was reallocated to ADR-015 (D-236, 2026-05-04). The correct
pointer is `superseded_by: E-11 S-11.xx`.

This correction is NOT applied in this burst (E-11 epic authoring only per constraints).
State-manager MUST apply the correction in a follow-up burst:

1. Update STORY-INDEX S-8.20 → `superseded_by: E-11 S-11.01`
2. Update STORY-INDEX S-8.21 → `superseded_by: E-11 S-11.01`
3. Update STORY-INDEX S-8.22 → `superseded_by: E-11 S-11.04`
4. Update STORY-INDEX S-8.23 → `superseded_by: E-11 S-11.06`
5. Update STORY-INDEX S-8.24 → `superseded_by: E-11 S-11.03`
6. Update STORY-INDEX S-8.25 → `superseded_by: E-11 S-11.03`
7. Update STORY-INDEX S-8.26 → `superseded_by: E-11 S-11.05`
8. Update STORY-INDEX S-8.27 → `superseded_by: E-11 S-11.07`
9. Add E-11 and S-11.00..S-11.07 rows to STORY-INDEX.
10. Update E-8 target_release field: `"v1.3 (Tier 3 — E-11)"` (was `"v1.3 (Tier 3 — future E-10)"`).

POLICY 1 (append-only numbering) is satisfied — old IDs are not removed; the
`superseded_by` pointer is corrected in-place.

---

## Risks

> **Risk ID alignment:** R-W16-001 through R-W16-008 are the canonical ADR-014 risk
> namespace (same as E-9). W-17 inherits all applicable R-W16-NNN risks. Tier 3-specific
> additive risks use R-W17-NNN namespace (append-only per POLICY 1).

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---------|-------------|-----------|--------|------------|
| R-W16-001 | bats orphan migration: bats tests for `.sh` hooks become orphans after WASM port | HIGH | MED | Deferred to Phase H. Each story spec includes a bats orphan deletion checklist task. S-11.07 triggers Phase H. |
| R-W16-002 | WASI preopens: 6 of 10 hooks read files from disk. path_allow declarations must be pinned per hook. Missing path_allow causes silent read failure or false-block. | MED | HIGH | Each story spec MUST pin `path_allow` declarations per hook in the AC table and registry TOML snippet. Adversarial review checks path_allow coverage before story reaches `ready`. |
| R-W16-003 | Latency regression: 10 new WASM plugins may regress cold-start p95 beyond 500ms or exceed 30MB hard kill-switch | LOW | HIGH | Same latency-primary + advisory model as W-16 (ADR-014 R-8.09 revised). S-11.00 measures W-17 baseline. Wave paused if cold-start regresses >10%. |
| R-W16-004 | bats/WASM test infrastructure: each story includes a WASM integration test task; bats migration deferred | MED | MED | Explicit WASM integration test task in each story ACs. Bats tests remain on disk until Phase H. |
| R-W16-007 | Behavioral divergence in rewrite-clean: block-mode hooks have nuanced bash logic (regex tokenization, path normalization) | MED | HIGH | Each story spec must enumerate all behavioral edge cases as explicit ACs. False-block test fixtures required per D-11.6. |
| R-W17-001 | PreToolUse block-mode regression: a wrong `permissionDecision` (false deny) on a PreToolUse hook immediately blocks the user's tool call — no batch effect, no recover path until the plugin is rolled back | HIGH | HIGH | Every block-mode story (S-11.01, S-11.03, S-11.04, S-11.05, S-11.06, S-11.07) MUST include negative test fixtures (false-block scenario) for EVERY blocking condition. Adversarial review is required before implementation dispatch. Mirror E-8 F-011 pattern for negative fixture ACs. |
| R-W17-002 | destructive-command-guard shell tokenization: the hook's command-pattern matching relies on shell-aware tokenization (token splitting, quoting). Rewriting in pure Rust regex may miss tokenization edge cases (quoted args, heredocs) | MED | HIGH | S-11.07 story spec MUST enumerate all 11+ command-pattern families from the bash original as explicit test vectors. Shell tokenization is not a 1:1 regex port — see Library Table entry for `shlex`. Adversarial review required. |
| R-W17-003 | protect-secrets dual-event registration: two `[[hooks]]` entries for one plugin; dispatcher must route both correctly | LOW | MED | S-11.04 story spec must verify dual-registration in hooks-registry.toml (both Bash and Read entries point to same `.wasm`). Integration test covers both event paths. Mirror how `protect-secrets.sh` dual-registration was originally verified in E-8. |
| R-W17-004 | adapter retirement sequencing: S-11.07 deletes adapter crate. If any hook in S-11.01..S-11.06 left a lingering adapter entry, deletion causes runtime dispatch failure | MED | HIGH | S-11.07 pre-deletion audit: `grep -r 'legacy-bash-adapter' hooks-registry.toml` must return zero matches before deletion proceeds. This is a hard pre-condition in S-11.07 ACs. |

---

## Acceptance Criteria

| AC | Statement |
|----|-----------|
| AC-1 | All 10 hooks (8 Tier 3 PreToolUse + 2 Tier 2 PostToolUse residuals) have native WASM equivalents in `crates/hook-plugins/<name>/` delivered by S-11.01..S-11.07 |
| AC-2 | `hooks-registry.toml` updated: 11 WASM entries added (protect-secrets = 2 entries); 10 legacy-bash-adapter entries disabled or removed for Tier 3 hooks |
| AC-3 | W-17 bundle growth within the latency-primary + advisory-ceiling model per ADR-014 R-8.09 revised: cold-start p95 ≤ 500ms (hard gate, from S-11.00 baseline); advisory soft cap ≤ 100% cumulative growth at end of W-17 (shared ceiling with W-16 per ADR-014 R-8.09); hard kill-switch ≤ 30MB. Per-wave telemetry `(bundle_size_delta_bytes, cold_start_p95_delta_ms)` published from S-11.00 baseline. Wave paused if cold-start regresses >10%. |
| AC-4 | All 7 batched/solo stories (S-11.01..S-11.07) pass adversarial convergence per ADR-013 before implementation dispatch |
| AC-5 | Every block-mode hook story (S-11.01, S-11.03, S-11.04, S-11.05, S-11.06, S-11.07) has at least one negative (false-block) test fixture per blocking condition in its story ACs. Mirror E-8 F-011 pattern. |
| AC-6 | HOST_ABI_VERSION = 1 in both `crates/hook-sdk/src/lib.rs` and `crates/factory-dispatcher/src/lib.rs` after all E-11 stories merge. |
| AC-7 | Legacy bash adapter crate (`crates/hook-plugins/legacy-bash-adapter/`) deleted at S-11.07 close. Pre-deletion audit confirms zero `[[hooks]]` entries use `legacy-bash-adapter.wasm`. TD-014 closed. |
| AC-8 | `.sh` files remain on disk per R-W16-001 (Phase H deferred); no `.sh` bash hook files deleted in W-17 stories (except Phase H triggers at S-11.07). |
| AC-9 | Windows CI runner (windows-x64) included in W-17 CI matrix; each story's integration tests pass on all 5 platforms: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64. Closes R-W16-006 for W-17. |
| AC-10 | S-11.07 T-0 STOP CHECK verifies that S-11.01..S-11.06 are all in `merged` status and zero legacy-bash-adapter registry entries remain before S-11.07 implementation begins. |
| AC-11 | All native WASM plugins comply with ADR-015 single-stream OTel emit contract: events route via `host::emit_event` to `events-YYYY-MM-DD.jsonl`; `event.name` uses reverse-DNS format with `.v1` suffix; `outcome` uses canonical enum (success\|failure\|error\|timeout\|skipped\|blocked). Block-mode hook auto-audit-trail emitted by dispatcher per D-15.3 — no additional plugin event on block path. |

---

## Open Questions

| ID | Question | Owner | Resolution |
|----|----------|-------|------------|
| OQ-11-001 | S-11.00 perf baseline: should W-17 measure independently (new perf-baseline-w17.md) or reuse the S-9.00 W-16 baseline (perf-baseline-w16.md) as the starting point? W-16 and W-17 share the advisory soft-cap ceiling; a shared baseline avoids double-counting. | S-11.00 story-writer | Resolved by S-11.00 measurement + state-manager update to perf-baseline-w16.md or new perf-baseline-w17.md |
| OQ-11-002 | check-factory-commit subprocess: what is the correct `timeout_ms` and `max_output_bytes` for `git log --oneline -1` (the expected command)? Ballpark: git log returns ~80 bytes; 5000ms timeout is conservative. Needs explicit pin per R-W16-002 guidance. | S-11.05 story-writer | Pin in S-11.05 ACs based on ADR-014 gap-analysis §4 advisory caps |
| OQ-11-003 | destructive-command-guard tokenization: should the Rust port use `shlex` crate for POSIX shell-aware tokenization, or pure regex over the raw command string? The bash original uses shell word-splitting semantics implicitly. A pure-regex port may miss quoted args or compound commands. | S-11.07 story-writer + adversarial review | Adversarial review before S-11.07 implementation dispatch |
| OQ-11-004 | OQ-W16-001 inheritance: the `vsdd.host.*` registry-prefix binary-acceptance decision (pending per E-9 Open Questions table) is required before E-11 Wave 1 ships if any Tier 3 hook emits `vsdd.host.*`-namespaced events. | SS-01 implementer or E-11 Wave 1 | Inherit OQ-W16-001 resolution from E-9; no new action unless Tier 3 hooks require novel event names |
| OQ-11-005 | purity-check path_allow: purity-check inspects source files potentially anywhere in the repo (outside `.factory/`). Does `path_allow = ["."]` correctly express WASI preopens for the full repo tree? Verify against WASI preopen semantics in the dispatcher. | S-11.02 story-writer | Confirm in S-11.02 ACs per R-W16-002 / R-W16-005 path_allow audit |

---

## Library Table

| Library | Version | Purpose | First Story |
|---------|---------|---------|-------------|
| regex | workspace (1.10+) | Pattern matching for protect-bc/protect-vp (lifecycle_status), red-gate (path patterns), destructive-command-guard (command families) | S-11.01, S-11.06, S-11.07 |
| serde_json | workspace | stdin JSON deserialization (HookPayload via vsdd-hook-sdk) | S-11.01..S-11.07 |
| serde_yaml | workspace (0.9.x) | YAML parse for red-gate (wave-state.yaml red[] list) | S-11.06 |
| vsdd-hook-sdk | 0.2.0 (post-S-8.10) | Plugin ABI shim (read_file, emit_event, log, exec_subprocess) | S-11.01..S-11.07 |
| vsdd-hook-sdk (subprocess facet) | 0.2.0+ | exec_subprocess wrapper for check-factory-commit (git log) | S-11.05 |
| shlex | 1.x (TBD — see OQ-11-003) | POSIX shell-aware tokenization for destructive-command-guard | S-11.07 |
| bats-core | >=1.10 (CI) | Bats orphan checklist verification (bash hooks remain on disk) | Per-story (D-11.5) |
| wc (POSIX) | system | Bundle-size measurement (`wc -c < file`); portable across macOS BSD and Linux GNU | S-11.00 |
| hyperfine | >=1.18 | Latency benchmarking harness (cold-start p95 measurement per wave) | S-11.00 |
| git | system | check-factory-commit subprocess via host::exec_subprocess (BC-1.05.001..034 + BC-1.05.035) | S-11.05 |

> **shlex note (OQ-11-003):** destructive-command-guard's bash original relies on
> implicit shell word-splitting. A naive regex port risks false-negatives on quoted
> arguments (e.g., `rm -rf "/factory"` vs `rm -rf /factory`). If the `shlex` crate
> is adopted, it must be added to workspace `Cargo.toml`. Adversarial review on S-11.07
> must include tokenization edge cases in test vectors.

> **serde_yaml pin:** Same `serde_yaml 0.9.x` workspace constraint as W-16 (E-9 v1.12
> note). Do not introduce a non-workspace serde_yaml version.

---

## Architecture Mapping

| Component | Module | Pure/Effectful |
|-----------|--------|---------------|
| PreToolUse protection WASM plugins (8 new) | `crates/hook-plugins/{protect-bc,protect-vp,protect-secrets,brownfield-discipline,factory-branch-guard,check-factory-commit,red-gate,destructive-command-guard}/` | Effectful (read_file or exec_subprocess + emit_event) |
| PostToolUse process WASM plugins (2 new) | `crates/hook-plugins/{convergence-tracker,purity-check}/` | Effectful (read_file + emit_event) |
| hooks-registry.toml (11 entry updates) | `plugins/vsdd-factory/hooks-registry.toml` | Configuration (not a code module) |
| legacy-bash-adapter crate (deleted at S-11.07) | `crates/hook-plugins/legacy-bash-adapter/` | N/A — deleted |
| W-17 bundle baseline + ceiling | `.factory/measurements/` + `.factory/architecture/perf-baseline-w17.md` (or perf-baseline-w16.md extended) | Pure (data artifacts) |

**Architecture section files:**
- `architecture/module-decomposition.md` — confirms `crates/hook-plugins/` belongs to SS-04
- `architecture/SS-07-hook-bash.md` — origin subsystem for all 10 bash hook files; post-E-11, SS-07 contains only `verify-git-push.sh` (D-1 exclusion)
- `architecture/SS-04-plugin-ecosystem.md` — destination subsystem for all 10 WASM crates

---

## Dependency Graph

```
E-8 (Tier 1 shipped — rc.4; adapter NOT deleted; 9 Tier 1 hooks native)
    ↓
E-9 (W-16 Tier 2 closure — 23 validate-*.sh hooks native; adapter still present)
    ↓
S-11.00 (W-17 perf baseline + bundle ceiling)
    ↓ [blocks S-11.01..S-11.06]
S-11.01, S-11.02, S-11.03, S-11.04, S-11.05, S-11.06  ← all parallel, depends_on S-11.00
    ↓ [all must be merged before S-11.07]
S-11.07 (destructive-command-guard + adapter retirement + Phase H)
    ↓
TD-014 CLOSED
```

> **Independence from E-9 (W-16):** E-8 explicitly noted "Tier 2 and Tier 3 share no
> implementation dependencies" (E-8 D-4). E-11 formally depends on E-9 closure
> (adapter retirement at S-11.07 requires all Tier 2 entries to be already native),
> but the hook port stories S-11.01..S-11.06 may begin in parallel with E-9 Wave 1
> implementation if the adapter crate remains on disk. The hard sequencing constraint
> is: S-11.07 (adapter deletion) MUST occur after both E-9 Wave 1 AND S-11.01..S-11.06
> are merged.

**Topological order (wave scheduling):**
- Wave 0: S-11.00 (depends on E-9 W-16 rc.X closure being done)
- Wave 1: S-11.01, S-11.02, S-11.03, S-11.04, S-11.05, S-11.06 (all depend on S-11.00; parallel)
- Wave 2: S-11.07 (depends on all Wave 1 stories merged + pre-deletion audit passes)

---

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| 1.0 | 2026-05-06 | product-owner | Initial authoring to anchor 10 orphan hooks following E-10's reallocation to ADR-015 OTel emission per D-236 (2026-05-04). Supersedes the stale "future E-10" pointer in E-8 v1.10 CHANGELOG and S-8.20–S-8.27 retirement entries. 8 stories (S-11.00 + S-11.01..S-11.07). BC anchors confirmed from BC-INDEX survey. D-11.7 documents state-manager follow-up burst for STORY-INDEX pointer correction. |
