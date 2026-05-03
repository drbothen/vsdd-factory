---
document_type: epic
epic_id: "E-9"
version: "1.3"
title: "Tier 2 Native WASM Migration (W-16) — 23 validate-*.sh hooks"
status: in-review
tech_debt_ref: TD-014
prd_capabilities: [CAP-002, CAP-008, CAP-013, CAP-022]
prd_frs: []
anchor_strategy: rewrite-clean-per-ADR-014-D-9.1
priority: P2
target_release: "v1.2 (Tier 2)"
story_count: 8
subsystems_affected: [SS-04, SS-07]
producer: story-writer
timestamp: 2026-05-03T00:00:00Z
phase: 2
traces_to: .factory/tech-debt-register.md#TD-014
depends_on: ["E-8"]
last_amended: 2026-05-03
inputs:
  - .factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md
  - .factory/architecture/audit-w16.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/architecture/gap-analysis-w16-subprocess.md
input-hash: "3458e0a"
---
<!-- [process-gap] Frontmatter fields tech_debt_ref, anchor_strategy, depends_on extend the canonical epic-template baseline (same as E-8 v1.9). Template update tracked as follow-up. -->

# Epic E-9: Tier 2 Native WASM Migration (W-16) — 23 validate-*.sh hooks

## Description

Port all 23 `validate-*.sh` hooks currently routed through `legacy-bash-adapter.wasm`
(hooks-registry.toml lines 145–797) to native WASM crates using the rewrite-clean
strategy (ADR-014 D-9.1). `validate-wave-gate-prerequisite` invokes
`verify-sha-currency.sh` via the existing `host::exec_subprocess` ABI
(BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03)) —
no new host ABI extension is required (ADR-014 D-9.2 withdrawn per gap analysis
2026-05-03; see gap-analysis-w16-subprocess.md Section 7). Delivered in 7
capability-cluster batches (S-9.01..S-9.07) plus one perf baseline story (S-9.00).
`HOST_ABI_VERSION` stays at 1 throughout. Closes the Tier 2 phase of TD-014.

> **Scope reduction 2026-05-03 (ADR-014 Amendment D-9.2):** S-9.30 (host::run_subprocess
> SDK extension) withdrawn. story_count 9 → 8. S-9.07 `depends_on` updated from
> [S-9.00, S-9.30] to [S-9.00] only. See CHANGELOG v1.1 and ADR-014 Amendment
> 2026-05-03. E-8 D-13 Tier 2/3 wave plan superseded by E-9 + future E-10 — see
> E-8 v1.10 CHANGELOG.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-002 | Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins | P0 |
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-013 | Capture post-execution activity (PostToolUse hooks) | P0 |
| CAP-022 | Port hook plugins from bash to native WASM | P2 |

## Capability Anchor Justification

**Primary anchor:** CAP-022 ("Port hook plugins from bash to native WASM") per
`domain-spec/capabilities.md` §CAP-022. E-9 is the W-16 Tier 2 cluster within the
CAP-022 migration: porting 23 PostToolUse:Edit|Write validators and two PreToolUse
gates from bash to native WASM. Without E-9, Tier 2 validators continue running
through `legacy-bash-adapter.wasm` with git-bash dependency and subprocess overhead.

**SS-04 (Plugin Ecosystem) anchor:** SS-04 owns `crates/hook-plugins/`; the 23
new validator WASM crates are SS-04 artifacts. Every S-9.0N port story touches SS-04.

**SS-07 (Hook Bash Layer) partial anchor:** SS-07 owns the `.sh` files in
`plugins/vsdd-factory/hooks/validate-*.sh` (23 files) and the `[[hooks]]` registry
entries in `plugins/vsdd-factory/hooks-registry.toml` that reference them. E-9 work
in SS-07 is registry-edit-only: per the W-15 D-7 schema, each Tier 2 hook keeps its
existing `[[hooks]]` entry (pointing to `hooks/validate-*.sh`) but gains a sibling
entry pointing to the new `hook-plugins/validate-*.wasm`. The `.sh` files remain on
disk (POLICY 1 append-only) but become unreferenced by any active registry entry
once W-16 completes. No Rust code changes in SS-07. After W-16, those files remain
on disk pending Phase H deletion (R-W16-001).

> [process-gap] stretch-anchor: The exact count of `.sh` files and TOML entry
> locations above is derived from audit-w16.md §1 (23 hooks confirmed) and
> hooks-registry.toml on-disk at 2026-05-03. If refactoring moves `.sh` files
> or TOML before W-16 implementation, verify the paths against the on-disk state
> at implementation time.

> **SS-02 dropped from E-9 anchors (v1.1, 2026-05-03):** S-9.30 (host::run_subprocess
> SDK extension, which was the SS-02 anchor story) was withdrawn per ADR-014 D-9.2
> amendment. SS-02 is no longer a primary anchor for E-9. `validate-wave-gate-prerequisite`
> (S-9.07) uses the existing `host::exec_subprocess`
> (BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03))
> which is already under SS-01 ownership — no new SS-02 artifacts are created in E-9. See
> gap-analysis-w16-subprocess.md Section 7 + ADR-014 Amendment 2026-05-03.

## Stories

| Story ID | Title | Points | Depends On | Blocks | Status |
|----------|-------|--------|-----------|--------|--------|
| S-9.00 | Perf baseline + W-16 bundle ceiling | TBD | E-8 (W-15 / rc.4 closure) | S-9.01..S-9.07 | draft |
| S-9.01 | Batch B-1: pure stdin-parse validators (4 hooks) | TBD | S-9.00 | — | draft |
| S-9.02 | Batch B-2: single file-read frontmatter validators (4 hooks) | TBD | S-9.00 | — | draft |
| S-9.03 | Batch B-3: PR/delivery file validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.04 | Batch B-4: STATE.md + cycle index validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.05 | Batch B-5: story-file + BC multi-file validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.06 | Batch B-6: cross-document lookup validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.07 | Batch B-7: complex YAML + subprocess validators (3 hooks) | TBD | S-9.00 | — | draft |
| **— Withdrawn —** | | | | | |
| S-9.30 | ~~SDK extension: host::run_subprocess (ADR-014 D-9.2)~~ | — | — | — | **withdrawn** (2026-05-03; see ADR-014 Amendment) |

**Story count: 8** (S-9.00 + S-9.01..S-9.07; S-9.30 withdrawn 2026-05-03)

**Note:** Story-writer authors S-9.01..S-9.07 in subsequent bursts following
adversarial convergence per ADR-013.

## Hook Inventory (23 hooks)

### Batch B-1 (S-9.01): Pure stdin-parse + emit_event (4 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-demo-evidence-story-scoped | PostToolUse:Edit\|Write | no | Low |
| validate-factory-path-root | PostToolUse:Edit\|Write | **YES** | Low |
| validate-finding-format | PostToolUse:Edit\|Write | no | Low |
| validate-novelty-assessment | PostToolUse:Edit\|Write | no | Low |

### Batch B-2 (S-9.02): Single file-read frontmatter validators (4 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-bc-title | PostToolUse:Edit\|Write | no | Low |
| validate-changelog-monotonicity | PostToolUse:Edit\|Write | no | Low |
| validate-red-ratio | PostToolUse:Edit\|Write | no | Low |
| validate-input-hash | PostToolUse:Edit\|Write | **YES** | Medium |

### Batch B-3 (S-9.03): PR/delivery file validators (3 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-pr-description-completeness | PostToolUse:Edit\|Write | no | Low |
| validate-table-cell-count | PostToolUse:Edit\|Write | no | Low |
| validate-pr-merge-prerequisites | PreToolUse:Agent | **YES** (`on_error = "block"` per hooks-registry.toml line 774) | Medium |

### Batch B-4 (S-9.04): STATE.md + cycle index validators (3 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-state-index-status-coherence | PostToolUse:Edit\|Write | no | Medium |
| validate-state-pin-freshness | PostToolUse:Edit\|Write | no | Medium |
| validate-state-size | PostToolUse:Edit\|Write | no | Low (subprocess-simplified: git compaction-detection dropped per ADR-014 D-9.1) |

### Batch B-5 (S-9.05): Story-file + BC multi-file validators (3 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-story-bc-sync | PostToolUse:Edit\|Write | no | Low |
| validate-count-propagation | PostToolUse:Edit\|Write | no | Medium |
| validate-index-self-reference | PostToolUse:Edit\|Write | no | Low |

### Batch B-6 (S-9.06): Cross-document lookup validators (3 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-anchor-capabilities-union | PostToolUse:Edit\|Write | no | High |
| validate-subsystem-names | PostToolUse:Edit\|Write | no | Low |
| validate-template-compliance | PostToolUse:Edit\|Write | **YES** | High |

### Batch B-7 (S-9.07): Complex YAML + subprocess validators (3 hooks)

| Hook | Event | Block-mode | Complexity |
|------|-------|-----------|------------|
| validate-vp-consistency | PostToolUse:Edit\|Write | no | High |
| validate-wave-gate-completeness | PostToolUse:Edit\|Write | no | Medium |
| validate-wave-gate-prerequisite | PreToolUse:Agent | **YES** (`on_error = "block"` per hooks-registry.toml line 794) | High (subprocess via existing host::exec_subprocess / BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036; S-9.30 withdrawn) |

> **Block-mode callout:** 5 of 23 Tier 2 validators use `on_error = "block"` per
> on-disk `hooks-registry.toml` audit (2026-05-03):
> - PostToolUse blockers (3): validate-factory-path-root (line 231), validate-input-hash (line 291),
>   validate-template-compliance (line 471)
> - PreToolUse blockers (2): validate-pr-merge-prerequisites (line 774),
>   validate-wave-gate-prerequisite (line 794)
> All PreToolUse hooks with `on_error = "block"` hard-block the tool invocation on error,
> consistent with their role as gate hooks.
> Per E-8 AC-8 (inherited), all 5 block-mode hooks MUST have additional negative (false-block)
> test fixtures in their batch story ACs.

## Problem Statement

After W-15 (rc.4), 23 `validate-*.sh` hooks remain routed through
`legacy-bash-adapter.wasm` in `hooks-registry.toml` (lines 145–797). These hooks:

1. **Block Windows compatibility (DRIFT-010):** All 23 require bash + jq on Windows.
2. **Have silent-jq-missing failure mode:** Every hook opens with
   `if ! command -v jq &>/dev/null; then exit 0; fi` — jq absence silently passes
   all hooks, creating false green validation on systems without jq.
3. **Add subprocess overhead on every Edit|Write event:** Legacy-bash-adapter spawns
   a bash subprocess per hook invocation. A native WASM plugin eliminates this.
4. **Block Phase H (adapter deletion):** Tier 2 must complete before legacy-bash-adapter
   can be retired (together with Tier 3 in W-17).

E-9 addresses all four problems for the Tier 2 hook class via rewrite-clean migration
to idiomatic Rust WASM plugins.

---

## Goals

1. All 23 `validate-*.sh` hooks ported to native WASM plugins under
   `crates/hook-plugins/validate-*/`.
2. `hooks-registry.toml` updated: 23 WASM entries added; 23 legacy-bash-adapter
   entries disabled (removed or `enabled = false`).
3. Silent-jq-missing failure mode eliminated: WASM plugins never silently pass due
   to missing toolchain dependencies.
4. HOST_ABI_VERSION = 1 unchanged throughout E-9 (no new host ABI extension
   required; D-9.2 withdrawn; existing host::exec_subprocess used,
   anchored by BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03)).
5. Bundle size growth within the W-16 latency-primary + advisory-ceiling model
   (measured by S-9.00; ADR-014 R-8.09 revised model; enforced per-batch).
   Primary gate: cold-start p95 ≤ 500ms. Hard kill-switch: 30MB cumulative bundle.
6. `validate-wave-gate-prerequisite` (S-9.07) invokes `verify-sha-currency.sh` via
   `host::exec_subprocess` with `shell_bypass_acknowledged` capability gate — no new
   SDK extension required. Anchored by BC-1.05.001..034 (existing) + BC-1.05.035 +
   BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03). See gap-analysis-w16-subprocess.md Section 7.

---

## Non-Goals

- Porting Tier 3 hooks (PreToolUse protections, 10 hooks — deferred to W-17).
- Deleting `legacy-bash-adapter.wasm` crate (requires W-17 completion first per
  Phase H sequencing in R-W16-001).
- Deleting `.sh` bash hook files (Phase H only — R-W16-001; bats orphan cleanup
  deferred to Phase H per R-W16-001 mitigation).
- Changing any hook's behavior during the port (rewrite-clean but behavior-parity;
  deliberate simplifications documented per ADR-014 D-9.1 mitigation).
- Modifying hooks.json direct-command entries (Tier 2 hooks use hooks-registry.toml
  only; hooks.json direct entries are not affected).

---

## Decisions

### D-9.1: Port Strategy — rewrite-clean (ADR-014)

Inherited from ADR-014 D-9.1. All 23 hooks ported as idiomatic Rust using `regex`,
`serde_json`, `serde_yaml`, `walkdir`/`std::fs`. No 1:1 bash-to-Rust translation.
Bash-specific idioms (awk field-splitting, jq pipelines, sed substitution) replaced
with semantically equivalent Rust stdlib/crate counterparts.

Key implication: each story spec must enumerate all behavioral edge cases from the
bash original as explicit ACs. The WASM Rust integration tests become the
authoritative behavioral contract (not bats, which tests bash execution only).

**validate-state-size subprocess simplification (D-9.1 deliberate simplification):**
The git compaction-detection branch (`git -C "$PARENT_DIR" show HEAD:STATE.md`) is
dropped in the WASM port. The hook's primary invariant (block at >500 lines) is
preserved. Documented as a deliberate simplification in S-9.04's EC table with a
TD entry for v1.2 if git-aware compaction detection is desired.

### D-9.2: Subprocess Capability — WITHDRAWN (ADR-014 Amendment 2026-05-03)

> **WITHDRAWN 2026-05-03.** Original decision was to build `host::run_subprocess`
> (BC-2.02.013, S-9.30) as a new ABI extension. Gap analysis
> (`gap-analysis-w16-subprocess.md` Section 7) confirmed that the existing
> `host::exec_subprocess` (anchored by BC-1.05.001..034 (existing) + BC-1.05.035 +
> BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03); production-verified) is
> sufficient for all W-16 use cases. Section 5 "fundamentally insufficient" list is
> empty for `validate-wave-gate-prerequisite`. See ADR-014 Amendment 2026-05-03.

**Revised decision:** `validate-wave-gate-prerequisite` (S-9.07) uses the existing
`host::exec_subprocess` ABI directly with `shell_bypass_acknowledged` capability
gate. Two minor additive extensions (BC-1.05.035: path traversal guard on binary arg;
BC-1.05.036: success-path telemetry event) fit within S-9.07 scope. These are
additive and ABI-stable (D-6 Option A precedent). HOST_ABI_VERSION stays at 1.
S-9.30 withdrawn; BC-2.02.013 withdrawn (preserved as audit trail).

All other 22 hooks use only the existing ABI (`read_file`, `emit_event`, `log`).

`validate-state-size` git subprocess: dropped per D-9.1 deliberate simplification.
`host::exec_subprocess` is NOT used for this hook.

### D-9.3: Story Granularity — 7 capability-cluster batches (ADR-014)

Inherited from ADR-014 D-9.3. 7 batched stories (S-9.01..S-9.07) grouped by host
functions required. See audit-w16.md Section 4 for the capability-cluster scheme.
Batching enables uniform BC anchoring patterns per batch and efficient adversarial
review focus on each cluster's specific risk profile.

### D-9.4: BC Anchor Strategy — reuse existing BC-7.xx family per hook

Mirrors E-8 D-2 Option C: reuse existing BCs; no new BC family.

All Tier 2 hooks reuse existing BC-7.xx anchors. The S-9.07 subprocess use case is
covered by existing BC-1.05.001..034 plus the additive BC-1.05.035 + BC-1.05.036
(per ADR-014 Amendment 2026-05-03). Each Tier 2 hook's behavioral obligations are
covered by existing BC-7.xx entries. If a port reveals unspecified behavior, a new
BC is drafted under the existing BC-7.xx sub-family for the relevant hook (not a
new BC-7.02.x migration family).

Story-writer identifies BC anchor(s) per batch story during S-9.01..S-9.07
authoring (Burst 2 + Burst 3).

### D-9.5: bats orphan strategy — checklist per story (inherits R-W16-001)

Each W-16 story spec must include a task to create a bats deletion checklist for
the batch hooks' corresponding bats tests. The `.sh` files and bats tests remain on
disk until Phase H. Per R-W16-001: bats orphan migration deferred to Phase H.

---

## Risks

> **Risk ID alignment note (v1.3):** Risk IDs below are aligned to ADR-014's
> canonical R-W16-NNN namespace. R-W16-001 through R-W16-004 use ADR-014's verbatim
> definitions: R-W16-001 = bats orphan migration, R-W16-002 = WASI preopens
> (ADR-014 §"Audit Risk Items Carried Forward"), R-W16-003 = latency/bundle growth,
> R-W16-004 = bats/WASM test infrastructure (ADR-014 §"Audit Risk Items Carried Forward").
> R-W16-005 and R-W16-006 are E-9-original additive IDs (per pass-1 F-5): R-W16-005 =
> path_allow failure-mode semantics (distinct from R-W16-002's scope; see R-W16-005 entry),
> R-W16-006 = Windows CI gap. E-9-specific risks that collided with ADR-014 IDs
> renumbered: Behavioral divergence → R-W16-007; YAML parsing fidelity → R-W16-008
> (append-only per POLICY 1). See CHANGELOG v1.1 F-1, v1.2 F-1 PARTIAL resolution.

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---------|-------------|-----------|--------|------------|
| R-W16-001 | bats orphan migration: bats tests for `.sh` hooks become orphans after WASM port (`.sh` files remain on disk until Phase H; bats tests test bash, not WASM) | HIGH | MED | Deferred to Phase H. Each story spec includes task to document the batch's bats orphan deletion checklist. No bats tests are deleted in W-16. |
| R-W16-002 | WASI preopens: 19 of 23 hooks read `FILE_PATH`. Canonical capability is `path_allow = [".factory/"]` for spec-file readers; `path_allow = ["."]` for hooks that may read files outside `.factory/`. Each story spec must pin per-hook path_allow declarations. (Per ADR-014 §"Audit Risk Items Carried Forward".) | MED | HIGH | Each story spec (S-9.01..S-9.07) MUST pin `path_allow` declarations per hook in the AC table and registry TOML snippet. Adversarial review checks path_allow coverage before story reaches `ready`. |
| R-W16-003 | Latency regression and bundle growth: 23 new WASM plugins may regress cold-start p95 beyond 500ms or exceed bundle hard kill-switch (30MB) | LOW | HIGH | Primary gate: cold-start p95 ≤ 500ms (ADR-014 R-8.09 revised model). S-9.00 measures post-rc.4 baseline capturing both `bundle_size_delta_bytes` and `cold_start_p95_delta_ms`. Wave pause if cold-start regresses >10%. Advisory soft cap: cumulative ≤100% growth (~14MB) at end of W-17. Hard kill-switch: 30MB cumulative; crossing requires fresh architecture review. Per-wave telemetry: `(bundle_size_delta_bytes, cold_start_p95_delta_ms)`. See ADR-014 Amendment 2026-05-03 (R-8.09 revised). |
| R-W16-004 | bats/WASM test infrastructure: each story spec includes a WASM integration test task (Rust `factory-dispatcher/tests/`) and defers bats migration to Phase H, citing the TD-020 class problem. (Per ADR-014 §"Audit Risk Items Carried Forward".) | MED | MED | Explicit WASM integration test task in each batch story ACs. Bats tests remain on disk until Phase H. |
| R-W16-005 | path_allow runtime fail-mode (HIGH/HIGH): Same WASI preopens scope as R-W16-002, but emphasizes the failure semantics distinct from R-W16-002's per-hook coverage framing. Missing or incorrect `path_allow` declarations cause runtime `read_file` denial. The hook either silently fails (returning empty data → downstream defects) OR incorrectly blocks (when `on_error="block"`, false positive). These two fail-modes are operationally different: silent-pass is invisible to the user; false-block is a hard stop. Registry-TOML-specific failure mode: an entry that declares `path_allow = [".factory/"]` for a hook that reads files outside `.factory/` will silently return empty on out-of-scope reads with no error log visible to the implementer. See also R-W16-002 (canonical WASI preopens per-hook coverage requirement). Retained per POLICY 1; future references should cite R-W16-002 for coverage scope and R-W16-005 for fail-mode semantics. | MED | HIGH | Adversarial pre-merge audit MUST grep each story's hooks-registry.toml additions for `path_allow = ` and verify against the per-hook FILE_PATH read pattern. Failure-mode tests required in the wave-gate suite: at least one test per block-mode hook exercises the false-block scenario (incorrect path_allow + on_error=block). |
| R-W16-006 | Windows CI gap: no Windows runner in W-16 CI plan | LOW | MED | Add AC for Windows CI runner per E-8 AC-5 pattern (see AC-10). Track as DRIFT-010 closure verification. |
| R-W16-007 | Behavioral divergence in rewrite-clean: rewriting 143-line bash hooks in idiomatic Rust may introduce subtle semantic differences (awk regex precedence, jq null-coalescing edge cases) | MED | HIGH | Each story spec must enumerate all behavioral edge cases as explicit ACs; adversarial convergence per ADR-013 surfaces divergences before implementation. E-8 OQ-001 (ERE precedence) is the canonical risk reference. (Renumbered from former R-W16-002 to avoid collision with ADR-014 WASI preopens definition.) |
| R-W16-008 | YAML parsing fidelity: 2 hooks (validate-wave-gate-completeness, validate-wave-gate-prerequisite) use python3 `yaml.safe_load`; replacement with `serde_yaml` must preserve parse semantics | MED | MED | Explicit test vectors for YAML edge cases (multi-doc streams, null values, integer coercion) in S-9.07 story ACs. (Renumbered from former R-W16-004 to avoid collision with ADR-014 bats/WASM test infrastructure definition.) |

---

## Acceptance Criteria

| AC | Statement |
|----|-----------|
| AC-1 | All 23 validate-*.sh hooks have native WASM equivalents in `crates/hook-plugins/validate-*/` delivered by S-9.01..S-9.07 |
| AC-2 | `hooks-registry.toml` updated: 23 WASM entries added (`plugin = "hook-plugins/validate-*.wasm"`); 23 legacy-bash-adapter entries disabled or removed for Tier 2 hooks |
| AC-3 | W-16 bundle growth within the latency-primary + advisory-ceiling model per ADR-014 R-8.09 revised (2026-05-03): cold-start p95 ≤ 500ms (hard gate, inherited from S-9.00 / E-8 AC-7b); advisory soft cap ≤ 100% cumulative growth at end of W-17 (~14MB); hard kill-switch ≤ 30MB. Per-wave telemetry `(bundle_size_delta_bytes, cold_start_p95_delta_ms)` published by each batch story from S-9.00 baseline values. Wave paused if cold-start regresses >10%. |
| AC-4 | All 7 batched stories (S-9.01..S-9.07) pass adversarial convergence per ADR-013 before implementation dispatch |
| AC-5 | S-9.07 T-0 STOP CHECK verifies only `depends_on: S-9.00` is satisfied before S-9.07 implementation begins. (S-9.30 dependency removed — D-9.2 withdrawn.) |
| AC-6 | HOST_ABI_VERSION = 1 in both `crates/hook-sdk/src/lib.rs` and `crates/factory-dispatcher/src/lib.rs` after all E-9 stories merge. Verified via: `grep -n 'pub const HOST_ABI_VERSION: u32 = 1' crates/hook-sdk/src/lib.rs` returns exactly one match; same check for `crates/factory-dispatcher/src/lib.rs`. |
| AC-7 | Legacy bash adapter entries for the 23 Tier 2 hooks removed from hooks-registry.toml; zero `validate-*.sh` hooks route through `legacy-bash-adapter.wasm` after E-9 completes |
| AC-8 | Block-mode hooks (validate-factory-path-root, validate-input-hash, validate-template-compliance, validate-pr-merge-prerequisites, validate-wave-gate-prerequisite) each have at least one negative (false-block) test fixture in their batch story ACs. (5 of 23 hooks; verified against hooks-registry.toml on-disk `on_error = "block"` fields.) |
| AC-9 | `.sh` files remain on disk per R-W16-001; no `.sh` bash hook files deleted in W-16 stories |
| AC-10 | Windows CI runner (windows-x64) included in W-16 CI matrix; each batch story's integration tests pass on all 5 platforms: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64. Per E-8 AC-5 pattern. Closes R-W16-006. |

---

## Open Questions

| ID | Question | Owner | Resolution |
|----|----------|-------|------------|
| OQ-1 | W-16 bundle size ceiling: what % growth is acceptable for 23 new plugins over the post-rc.4 baseline? | story-writer (S-9.00) | Resolved by S-9.00 measurement + ceiling proposal |
| OQ-2 | validate-state-size compaction-detection: the git subprocess path is simplified away in D-9.1. If the line-count-only gate triggers too many false-block events, should we revisit at v1.2? | tech-debt | File as TD after W-16 ships; low priority |
| OQ-3 | exec_subprocess registry: validate-wave-gate-prerequisite's `hooks-registry.toml` entry needs a `[hooks.<id>.capabilities.exec_subprocess]` block. Per gap-analysis-w16-subprocess.md §7 (ExecSubprocessCaps schema) the required fields are: `binary_allow = ["bash"]`, `shell_bypass_acknowledged = "acknowledged"`, `env_allow = ["PATH"]`, `cwd_allow = []` (validate-wave-gate-prerequisite uses `$SHA_PROJECT_ROOT` flag, not cwd — empty allow-list correct). Who authors this TOML snippet? | S-9.07 | RESOLVED — S-9.30 withdrawn; S-9.07 provides the concrete registry example using exec_subprocess. See gap-analysis-w16-subprocess.md Section 7 migration plan. |

---

## Library Table

| Library | Version | Purpose | First Story |
|---------|---------|---------|-------------|
| regex | workspace (1.10+) | Pattern matching replacing grep/awk in 20+ hooks | S-9.01 |
| serde_json | workspace | stdin JSON deserialization (HookPayload via vsdd-hook-sdk) | S-9.01 |
| serde_yaml | workspace (0.9.x) | YAML frontmatter + wave-state.yaml parsing | S-9.02, S-9.07 |
| walkdir | workspace | Directory traversal replacing `find` in pr-merge-prerequisites | S-9.03 |
| vsdd-hook-sdk | 0.2.0 (post-S-8.10) | Plugin ABI shim (read_file, emit_event, log) | S-9.01..S-9.06 |
| vsdd-hook-sdk (subprocess facet) | 0.2.0+ | exec_subprocess wrapper for S-9.07 | S-9.07 |
| bats-core | >=1.10 (CI) | Bats orphan checklist verification (bash hooks remain on disk) | Per-story (D-9.5) |
| wc (POSIX) | system | Bundle-size measurement (`wc -c < file`); portable across macOS BSD and Linux GNU. NOT `du -sb` (GNU-only `-b` flag; macOS `du` uses `-k` for kibibytes). | S-9.00 |
| hyperfine | >=1.18 | Latency benchmarking harness (cold-start p95 measurement per wave) | S-9.00 |

> **Library table change (v1.1, 2026-05-03):** `std::process::Command`,
> `std::time::{Duration, Instant}`, `glob`, `anyhow/thiserror`, and `tempfile`
> rows removed — these were for the S-9.30 `host::run_subprocess` dispatcher binding,
> which is withdrawn. `hyperfine` row corrected to "Latency benchmarking harness"
> (was incorrectly listed as "Bundle-size measurement harness"; `du` is the bundle-size
> tool). See CHANGELOG v1.1 F-3.

> **Version pin note:** All library versions use workspace-level version constraints.
> Do not introduce non-workspace dependencies. `serde_yaml 0.9.x` — same pin as
> established in W-15 (see E-8 S-8.07 TD entry for 0.9.34 deprecated status; same
> constraint applies in W-16 until a workspace-level upgrade is coordinated).

---

## Architecture Mapping

| Component | Module | Pure/Effectful |
|-----------|--------|---------------|
| validate-*.wasm plugins (23 new) | `crates/hook-plugins/validate-*/` | Effectful (read_file host call + emit_event) |
| hooks-registry.toml (23 entry updates) | `plugins/vsdd-factory/hooks-registry.toml` | Configuration (not a code module) |
| W-16 bundle baseline + ceiling | `.factory/measurements/` + `.factory/architecture/perf-baseline-w16.md` | Pure (data artifacts) |

> **Architecture table change (v1.1, 2026-05-03):** `host::run_subprocess` SDK
> wrapper, dispatcher binding, `SubprocessCaps` struct, and pure-core check
> functions removed — these were S-9.30 artifacts, now withdrawn. No new SS-02
> modules are created in E-9. `validate-wave-gate-prerequisite` (S-9.07) uses the
> existing `crates/factory-dispatcher/src/host/exec_subprocess.rs` path.

**Architecture section files:**
- `architecture/module-decomposition.md` — confirms `crates/hook-plugins/` belongs to SS-04; `crates/hook-sdk/` to SS-02
- `architecture/SS-02-hook-sdk.md` — host::run_subprocess section marked WITHDRAWN per ADR-014 Amendment 2026-05-03 (gap-analysis-w16-subprocess.md §7); no Schema Evolution entry required
- `architecture/SS-04-plugin-ecosystem.md` — canonical home for all hook plugin crates

---

## Dependency Graph

```
E-8 (rc.4 SDK closure: host::write_file + HookPayload SubagentStop fields merged)
    ↓
S-9.00 (perf baseline + W-16 bundle ceiling)
    ↓ [blocks S-9.01..S-9.07]
S-9.01, S-9.02, S-9.03, S-9.04, S-9.05, S-9.06, S-9.07  ← all parallel, depends_on S-9.00
```

> **S-9.30 removed from dependency graph (v1.1, 2026-05-03):** S-9.30 was the
> only story blocking S-9.07 beyond S-9.00. With S-9.30 withdrawn, S-9.07 depends
> only on S-9.00. The Wave 2 single-story bottleneck is eliminated.

**Topological order (wave scheduling):**
- Wave 0: S-9.00 (depends on E-8 / rc.4 closure being done)
- Wave 1: S-9.01, S-9.02, S-9.03, S-9.04, S-9.05, S-9.06, S-9.07 (all depend on S-9.00; parallel)

---

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| 1.0 | 2026-05-03 | story-writer | Initial authoring — Phase D-4 Burst 1. E-9 epic for W-16 Tier 2 native WASM migration (23 validate-*.sh hooks). 9-story scope: S-9.00 + S-9.30 + S-9.01..S-9.07. Based on ADR-014 (D-9.1/D-9.2/D-9.3), audit-w16.md, and BC-2.02.013. Follows E-8 v1.9 shape. |
| 1.1 | 2026-05-03 | story-writer | Pass-1 fix burst (18 findings) + scope reduction per ADR-014 Amendment 2026-05-03. See details below. |
| 1.2 | 2026-05-03 | story-writer | Pass-2 fix burst (12 findings from W-16-E-9-pass-2-adversary.md). See v1.2 changelog below. |
| 1.3 | 2026-05-03 | story-writer | Pass-3 fix burst (2 E-9-own findings + 1 cross-doc from S-9.00). See v1.3 changelog below. |

### v1.1 (2026-05-03) — Pass-1 fix burst + D-9.2 scope reduction

**HIGH findings closed (F-1..F-5):**

- **F-1 CLOSED (risk ID drift):** Risk table renumbered and aligned to ADR-014's
  canonical R-W16-NNN namespace. Old R-W16-004 (run_subprocess security surface)
  replaced by R-W16-005 (WASI preopens / path_allow) since S-9.30 is withdrawn.
  R-W16-003 updated with ADR-014 R-8.09 revised latency-primary model. R-W16-006
  added for Windows CI gap. Cross-reference note added in Risk table header.

- **F-2 CLOSED (W-16 wave collision with E-8):** Note added: "E-8 D-13 Tier 2/3
  wave plan superseded by E-9 + future E-10 — see E-8 v1.10 CHANGELOG." E-8's
  W-16/W-17 references in D-13 are struck-through in E-8 v1.10. E-9 is the
  authoritative source for W-16 scope.

- **F-3 CLOSED (hyperfine mis-anchor):** Library table corrected. `hyperfine`
  row updated to "Latency benchmarking harness" (not bundle-size). `du (coreutils)`
  row added for bundle-size measurement. S-9.30-specific dependencies removed
  (std::process::Command, std::time, glob, anyhow/thiserror, tempfile).

- **F-4 CLOSED (stale line-number anchors in AC-6):** AC-6 now uses grep-based
  assertion: `grep -n 'pub const HOST_ABI_VERSION: u32 = 1' <file>` instead of
  brittle `:58` and `:43` line-number anchors.

- **F-5 CLOSED (missing WASI preopens risk):** R-W16-005 added: WASI preopens /
  path_allow coverage — each story spec MUST pin per-hook `path_allow` declarations;
  missing pins cause runtime read denial. Mitigation: per-story AC requiring
  path_allow in registry TOML snippet.

**Scope reduction (ADR-014 D-9.2 amendment 2026-05-03):**
- S-9.30 withdrawn (story_count 9 → 8)
- Wave 0: was {S-9.00, S-9.30}; now {S-9.00 only}
- S-9.07 `depends_on`: was [S-9.00, S-9.30]; now [S-9.00]
- Stories table: S-9.30 row marked `withdrawn` with ADR-014 reference
- D-9.2 decision section: updated to WITHDRAWN with revised decision prose
- Capability Anchor: SS-02 dropped (no new SS-02 artifacts in E-9)
- Architecture Mapping: S-9.30 components removed
- Dependency graph: S-9.30 node removed; Wave 2 (single-story) eliminated
- Goals: Goal #4 and #6 rewritten to reflect exec_subprocess usage
- Bundle ceiling reference: Goal #5 + AC-3 updated to ADR-014 R-8.09 revised model

**MEDIUM findings closed (F-6..F-13):**
- F-6: S-9.00 `depends_on` note added in Stories table (E-8 / rc.4 closure)
- F-7: AC-3 (bundle ceiling) replaced with ADR-014 R-8.09 revised dual-gate model
- F-8: AC-10 added for Windows CI runner (5-platform coverage)
- F-11: Scope note re generate-registry.sh deferred — E-9 covers 23 validate-*.sh
  hooks only; generate-registry.sh not in W-16 rewrite-clean scope
- F-12: AC-3 updated to reference S-9.00 baseline values rather than embed numbers
- F-13: Wave topology simplified: burst-vs-wave language removed; Wave 0 = S-9.00;
  Wave 1 = S-9.01..S-9.07 (all parallel after S-9.30 removal)

**LOW findings closed (F-14..F-18):**
- F-14: `subsystems_affected: [SS-04, SS-07]` added to frontmatter (SS-02 dropped)
- F-15: Wave sub-table formatting normalized
- F-16: ADR-013 convergence requirement note added to success criteria (AC-4)
- F-17: S-9.00 wave:16 consistency preserved; dependency graph updated
- F-18: Background wording aligned to ADR-014 §2 "20/23" phrasing

### v1.2 (2026-05-03) — Pass-2 fix burst

Fixes from W-16-E-9-pass-2-adversary.md:

- **F-1 PARTIAL [HIGH]: R-W16-002/R-W16-004 semantic collision with ADR-014** — Adopted
  ADR-014's verbatim definitions for R-W16-001..006: R-W16-002 = WASI preopens;
  R-W16-004 = bats/WASM test infrastructure. E-9 v1.1's redefined "Behavioral divergence"
  renumbered to R-W16-007; "YAML parsing fidelity" renumbered to R-W16-008 (append-only
  per POLICY 1). All internal cross-references in Risk table updated. Risk alignment note
  in Risk table header updated to v1.2 canonical description.

- **F-P2-001 + F-P2-004 [HIGH]: D-9.4 Exception clause invalidated by D-9.2 withdrawal** —
  Removed "Exception: `host::run_subprocess` requires BC-2.02.013 (authored by PO in D-3)"
  clause from D-9.4. Replaced with: "All Tier 2 hooks reuse existing BC-7.xx anchors. The
  S-9.07 subprocess use case is covered by existing BC-1.05.001..034 plus the additive
  BC-1.05.035 + BC-1.05.036 (per ADR-014 Amendment 2026-05-03)."

- **F-P2-002 [HIGH]: Architecture section files stale (SS-02 host::run_subprocess)** —
  Replaced line 374 (`architecture/SS-02-hook-sdk.md` Schema Evolution reference) with:
  "host::run_subprocess section marked WITHDRAWN per ADR-014 Amendment 2026-05-03
  (gap-analysis-w16-subprocess.md §7); no Schema Evolution entry required."

- **F-P2-003 [HIGH]: BC-2.02.005 mis-anchor — propagation across 6 sites** —
  Replaced ALL E-9 references to `BC-2.02.005` in exec_subprocess context with
  "BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment
  2026-05-03)". Sites fixed: Description (line 38), SS-02 dropped note (line 78),
  Batch B-7 table (line 159), Goal #4 (line 194), Goal #6 (lines 198–200), D-9.2 prose
  (lines 242–248). BC-2.02.005 = read_string SDK protocol (correct per ADR-014 Correction
  2026-05-03 and gap-analysis-w16-subprocess.md).

- **F-P2-005 [MEDIUM]: Burst language at line 99** — Replaced "S-9.01..S-9.04 are Burst 2;
  S-9.05..S-9.07 are Burst 3" with "Story-writer authors S-9.01..S-9.07 in subsequent
  bursts following adversarial convergence per ADR-013."

- **F-P2-006 [MEDIUM]: SS-07 anchor justification strengthened** — Added concrete artifacts
  to SS-07 paragraph: references `plugins/vsdd-factory/hooks/validate-*.sh` (23 files),
  `hooks-registry.toml` `[[hooks]]` entries, and registry-edit-only scope. [process-gap]
  stretch-anchor disclosure block added per POLICY 5.

- **F-P2-007 [MEDIUM]: vsdd-hook-sdk Library Table Purpose column** — Split single row into
  two: "Plugin ABI shim (read_file, emit_event, log) | S-9.01..S-9.06" and "exec_subprocess
  wrapper for S-9.07 | S-9.07". Aligns exec_subprocess consumer with its actual first story.

- **F-P2-008 [MEDIUM]: OQ-3 exec_subprocess registry block missing cwd_allow** — Added
  `cwd_allow = []` to OQ-3's resolution text, citing gap-analysis-w16-subprocess.md §7
  ExecSubprocessCaps schema. Justification: validate-wave-gate-prerequisite uses
  `$SHA_PROJECT_ROOT` flag, not cwd.

- **F-P2-009 [MEDIUM]: Block-mode column inventory corrected via on-disk TOML audit** —
  Audited `plugins/vsdd-factory/hooks-registry.toml` on-disk `on_error` fields for all
  Tier 2 hooks. Found 5 block-mode hooks (not 3): validate-pr-merge-prerequisites (line
  774) and validate-wave-gate-prerequisite (line 794) both have `on_error = "block"`.
  Corrected their Block-mode column from "no" to "YES" with TOML line citations. Block-mode
  callout updated: "5 of 23" with enumeration. AC-8 updated to list all 5 block-mode hooks.

- **F-P2-010 [LOW]: input-hash placeholder** — Retained as `[pending-recompute]`. Per POLICY
  3, state-manager recomputes SHA after fix burst completes. No change to frontmatter.

- **F-P2-011 [LOW]: S-9.30 row ordering** — Moved withdrawn S-9.30 row to END of Stories
  table. Added "**— Withdrawn —**" subsection separator row per POLICY 1.

- **F-P2-012 [LOW]: status: in-review** — Flipped frontmatter `status: draft` → `status:
  in-review` per pass-1 F-14 suggestion. Convergence is in progress (pass-2 SUBSTANTIVE).

- **F-P2-013 [LOW]: epic-level depends_on intent** — E-9 `depends_on: ["E-8"]` is the
  epic-level declaration. Sibling stories (e.g., S-9.00) independently declaring
  `depends_on: ["E-8"]` is correct: epic-level depends_on describes the epic's predecessor;
  story-level depends_on is the implementer's gate. Both are correct and intentional.
  No change to E-9 frontmatter.

Lines: v1.1 (495L) → v1.2 (~560L)

### v1.3 (2026-05-03) — Pass-3 fix burst

Fixes from W-16-E-9-pass-3-adversary.md:

- **F-P3-001 [MED]: R-W16-005 vs R-W16-002 duplication** — R-W16-005 description
  rewritten to highlight failure-mode semantics distinct from R-W16-002's per-hook
  coverage framing. R-W16-005 now focuses on the two operationally distinct fail-modes
  (silent-pass vs false-block) and the registry-TOML-specific silent-empty case. Added
  cross-reference "See also R-W16-002 (canonical WASI preopens per-hook coverage
  requirement)." R-W16-005 retained per POLICY 1 with semantic value clarified; future
  references cite R-W16-002 for coverage scope and R-W16-005 for fail-mode semantics.

- **F-P3-002 [LOW]: Risk header wording precision** — Replaced "R-W16-001 through
  R-W16-006 use ADR-014's verbatim definitions" with "R-W16-001 through R-W16-004 use
  ADR-014's verbatim definitions; R-W16-005 and R-W16-006 are E-9-original additive IDs
  (per pass-1 F-5)." Eliminates the self-contradiction where the header claimed ADR-014
  verbatim for IDs that the same paragraph admitted were E-9-original.

Fixes from W-16-S-9.00-pass-3-adversary.md (cross-doc):

- **F-P3-001 [HIGH] (cross-doc): Library Table line 373 updated — du → wc (POSIX)** —
  Replaced "| du (coreutils) | system | Bundle-size measurement ... wc -c portable fallback |"
  with "| wc (POSIX) | system | Bundle-size measurement (wc -c < file); portable across
  macOS BSD and Linux GNU. NOT du -sb (GNU-only) |". The du -sb prohibition propagated
  from S-9.00 v1.2 (F-7 + F-P2-001) to the parent epic library table. du -sb is now
  uniformly prohibited across both E-9 epic and S-9.00 story per F-7 (pass-1) propagation.

Lines: v1.2 (~570L) → v1.3 (598L)
