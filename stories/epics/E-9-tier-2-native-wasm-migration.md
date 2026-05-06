---
document_type: epic
epic_id: "E-9"
version: "1.33"
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
last_amended: 2026-05-05
inputs:
  - .factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/architecture/audit-w16.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/architecture/gap-analysis-w16-subprocess.md
input-hash: "37151a4"
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

> **ADR-015 awareness (v1.7 amendment):** All 23 native WASM plugins that call `emit_event`
> MUST comply with the ADR-015 single-stream OTel emit contract once Tier 2 lands. Key
> implications for S-9.01..S-9.07 implementation:
> - All plugin-emitted events route to `events-YYYY-MM-DD.jsonl` via `host::emit_event`
>   (D-15.1 single-stream). The separate `dispatcher-internal-*.jsonl` path is NOT the
>   emit target.
> - Plugins assert only `event.name` + domain fields. Host stamps all Resource attributes
>   and per-event identity fields (D-15.3 enrichment contract). Plugin-supplied values for
>   host-owned fields are overridden by the host.
> - `event.name` MUST use reverse-DNS format with `.v1` suffix (e.g.,
>   `vsdd.hook.validate.bc_title.v1`). Non-conforming names land in
>   `event.category = "unknown"`.
> - `outcome` field MUST use the canonical enum: `success | failure | error | timeout |
>   skipped | blocked`. Block-mode hooks return `HookResult::Block`; the dispatcher
>   automatically emits `vsdd.block.plugin_blocked.v1` with `outcome=blocked`,
>   `plugin.name`, and `hook.tool_name` (D-15.3 block path audit trail). Plugins do NOT
>   need to emit an additional event on the block path — the dispatcher's automatic
>   emission fully satisfies D-15.3.
> - S-9.07's verify-sha-currency.sh subprocess inherits `VSDD_TRACE_ID` and
>   `VSDD_PARENT_SPAN_ID` automatically via the dispatcher's Command::new env setup
>   (ADR-015 D-15.4) — no plugin manifest change needed.
> See ADR-015 D-15.1 (single stream), D-15.2 (schema), D-15.3 (enrichment), D-15.4
> (trace propagation). Story-writer MUST incorporate ADR-015 compliance ACs into each
> S-9.01..S-9.07 story body.

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
| R-W16-003 | Latency regression and bundle growth: 23 new WASM plugins may regress cold-start p95 beyond 500ms or exceed bundle hard kill-switch (30MB) | LOW | HIGH | Primary gate: cold-start p95 ≤ 500ms (ADR-014 R-8.09 revised model). S-9.00 measures S-9.00 baseline (post-rc.11, pinned in perf-baseline-w16.md) capturing both `bundle_size_delta_bytes` and `cold_start_p95_delta_ms`. Wave pause if cold-start regresses >10%. Advisory soft cap: cumulative ≤100% growth (advisory soft cap target = 643686 bytes per ADR-014 R-8.09 Amendment 2026-05-03; pinned in perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes) at end of W-17. Hard kill-switch: 30MB cumulative; crossing requires fresh architecture review. Per-wave telemetry: `(bundle_size_delta_bytes, cold_start_p95_delta_ms)`. See ADR-014 Amendment 2026-05-03 (R-8.09 revised). **ADR-015 note (v1.7):** The per-wave telemetry events emitted by each batch story MUST route through the ADR-015 single-stream contract (`events-*.jsonl` via `host::emit_event`); the emit path itself contributes negligible overhead per D-15.1 rationale (FileSink append at vsdd-factory event volumes). |
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
| AC-3 | W-16 bundle growth within the latency-primary + advisory-ceiling model per ADR-014 R-8.09 revised (2026-05-03): cold-start p95 ≤ 500ms (hard gate, inherited from S-9.00 / E-8 AC-7b); advisory soft cap ≤ 100% cumulative growth at end of W-17 (advisory soft cap target = 643686 bytes; computed as v1.0.0-rc.1 baseline × 2 per ADR-014 Amendment 2026-05-03 "R-8.09 ceiling model revised (research)"; baseline value pinned in perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes); hard kill-switch ≤ 30MB. Per-wave telemetry `(bundle_size_delta_bytes, cold_start_p95_delta_ms)` published by each batch story from S-9.00 baseline values. Wave paused if cold-start regresses >10%. |
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
| OQ-1 | W-16 bundle size ceiling: what % growth is acceptable for 23 new plugins over the S-9.00 baseline (post-rc.11, pinned in perf-baseline-w16.md)? | story-writer (S-9.00) | Resolved by S-9.00 measurement + ceiling proposal |
| OQ-2 | validate-state-size compaction-detection: the git subprocess path is simplified away in D-9.1. If the line-count-only gate triggers too many false-block events, should we revisit at v1.2? | tech-debt | File as TD after W-16 ships; low priority |
| OQ-3 | exec_subprocess registry: validate-wave-gate-prerequisite's `hooks-registry.toml` entry needs a `[hooks.<id>.capabilities.exec_subprocess]` block. Per gap-analysis-w16-subprocess.md §7 (ExecSubprocessCaps schema) the required fields are: `binary_allow = ["bash"]`, `shell_bypass_acknowledged = "acknowledged"`, `env_allow = ["PATH"]`, `cwd_allow = []` (validate-wave-gate-prerequisite uses `$SHA_PROJECT_ROOT` flag, not cwd — empty allow-list correct). Who authors this TOML snippet? | S-9.07 | RESOLVED — S-9.30 withdrawn; S-9.07 provides the concrete registry example using exec_subprocess. See gap-analysis-w16-subprocess.md Section 7 migration plan. `timeout_ms = 30000` (30s; gap-analysis §4 advisory cap; safety margin over 200ms expected runtime); `max_output_bytes = 65536` (64KB; gap-analysis §4 "easily under 64KB" envelope; truncation Err'd per gap-analysis §3 line 127) |
| OQ-W16-001 | Resolve `vsdd.host.*` registry-prefix decision before E-10 Wave 1 ships | SS-01 implementer or E-10 Wave 1 architect | Binary acceptance per `.factory/specs/open-questions.md` OQ-W16-001: (a) ADR-015 D-15.2 registry amended to add `vsdd.host.*` mapping, OR (b) event.name uses `vsdd.dispatcher.subprocess_completed.v1` exactly (lifecycle category) |

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
> (was incorrectly listed as "Bundle-size measurement harness"; `du` was the bundle-size
> tool at v1.1, superseded by wc -c per v1.3 fix-burst F-P3-001 cross-doc). See CHANGELOG v1.1 F-3.

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
| 1.4 | 2026-05-03 | story-writer | Pass-4 fix burst (fix-only mode). 2 cross-doc fixes (F-P4-001 STORY-INDEX BC anchor, F-P4-002 v1.1 changelog parenthetical) + F-P4-003 LOW deferred. See v1.4 changelog below. |
| 1.5 | 2026-05-03 | story-writer | Pass-6 structural fix burst (fix-only). F-P6-001 v1.4 changelog heading depth `##` → `###`; F-P6-002 v1.4 summary-table row appended; F-P6-003 deferred. See v1.5 changelog below. |
| 1.6 | 2026-05-03 | story-writer | Pass-7 structural fix burst (fix-only). F-P7-001 v1.5+v1.6 summary-table rows appended; F-P7-002 line-count footer convention DROPPED to break recurring drift cycle. |
| 1.7 | 2026-05-05 | architect | D-236 amendment — absorb ADR-015 single-stream OTel contract awareness before Burst 2 story authoring. |
| 1.8 | 2026-05-05 | architect | D-242 fix burst — close pass-3 SUBSTANTIVE findings: H-1 block-event misattribution (option b), M-1 host-prefix binary-choice, M-2 capability_denied rename, M-3 perf-baseline frontmatter, L-1 trace-id wording. |
| 1.9 | 2026-05-05 | architect | D-244 minimal fix burst — close H-P4-001 fabricated AC-3 citation (M-2 rationale leg (c) rewritten to real ADR-015 anchor) + L-P4-001 line range update. |
| 1.10 | 2026-05-05 | architect | D-246 fix burst — close pass-5 H-P5-001 frontmatter version drift, M-P5-001 v1.8 prose restored to original (POLICY 1), M-P5-003 audit-w16.md B-7 block-mode treatment added. M-P5-002 + LOWs deferred. |
| 1.11 | 2026-05-05 | architect | D-248 fix burst — close pass-6 H-P6-001 B-2+B-6 explicit H-1 option (b) in audit-w16.md, M-P6-002 OQ-W16-001 filed. 3 MED + 2 LOW deferred with rationale. |
| 1.12 | 2026-05-05 | architect | D-250 minimal fix burst — close pass-7 line 38 trio: M-P7-001 en-dash → explicit list, M-P7-002 "H-1 option (b)" → "are block-mode", M-P7-003 PostToolUse parenthetical added, L-P7-001 "per ADR-015 D-15.3" → "per D-15.3". L-P7-002 + L-P7-003 deferred. |
| 1.13 | 2026-05-05 | state-manager | D-251 minimal fix burst — perf-baseline-w16.md line 156 misanchor closed (E-9 D-9.4 → E-9 AC-3 per pass-8 M-P8-001). |
| 1.14 | 2026-05-05 | state-manager | D-254 combined seal-and-fix — H-P11-001 AC-3 ~14MB → 643686 bytes; M-P11-001 open-questions.md nomenclature scrub. |
| 1.15 | 2026-05-05 | state-manager | D-255 combined seal-and-fix (recursive-scrub applied) — H-P12-001 + M-P12-001/002/003 + L-P12-001 closed. |
| 1.16 | 2026-05-05 | state-manager | D-256 last-mile fix burst — M-P13-001 line-cite off-by-one + L-P13-001 (research) restore + L-P13-002 backticks normalize. |
| 1.17 | 2026-05-05 | state-manager | D-257 minimal fix burst — M-P14-001 perf-baseline H2 'Option C' non-resolving anchor closed; L-P14-001/2 SKIPPED with rationale. |
| 1.18 | 2026-05-05 | state-manager | D-258 minimal fix burst — M-P15-001 OQ-W16-001 propagated to E-9 Open Questions table; L-P15-001/2 SKIPPED with rationale. |
| 1.19 | 2026-05-05 | state-manager | D-260 sibling-residue fix burst — H-P17-001 (R-W16-003 ~14MB residue) + H-P17-002 (perf-baseline H2 post-rc.4 stale) + M-P17-001 (OQ-1 post-rc.4 stale); L-P17-001 SKIPPED. |
| 1.20 | 2026-05-05 | state-manager | D-261 convention closure burst — M-P18-001 last_amended field added to 4 arch-doc files (5th-recurrence resolution per S-7.02); L-P18-001 perf-baseline references (research) restored. TD-VSDD-073 codified. |
| 1.21 | 2026-05-05 | state-manager | D-263 implementation-readiness fix burst — M-P20-001 (OQ-3 timeout/output pinned) + M-P20-002 (BC-1.05.036 ADR-015 awareness) + L-P20-002 (BC-1.05.036 error-path reality) + TD-VSDD-074 (BC last_amended scope extension); L-P20-001 SKIPPED with rationale. |
| 1.22 | 2026-05-05 | state-manager | D-264 multi-fix burst — H-P21-001 (BC-1.05.036 error codes -7/-8→-2/-3; source-code verified); H-P21-002 (open-questions.md line 325→326 citation; grep verified); M-P21-001 (BC-1.05.035 ADR-015 awareness clause added per TD-VSDD-074); M-P21-002 (BC-1.05.036 fabricated "host" category corrected); M-P21-003 (truncated:bool documented as reserved always-false in v1); L-P21-001/002 DEFERRED; TD-VSDD-075 codified. |
| 1.23 | 2026-05-05 | state-manager | D-265 sibling-sweep fix burst — H-P22-001 (BC-1.05.036 §Related BCs lines 61-62 + §EC-004 line 86 + §Canonical Test Vectors line 97 aligned to Postcondition 5 no-event reality); H-P22-002 (BC-1.05.035 §Postcondition 4 INTERIM qualifier added); M-P22-001 (BC-1.05.036 Postcondition 1 scoped to success path); M-P22-002 (OQ-W16-001 acceptance (a) AND-linked to canonical event name); M-P22-003 (BC-1.05.035 precedence ladder appended to §Postconditions); L-P22-001/002 SKIPPED/absorbed; TD-VSDD-076 codified. |
| 1.24 | 2026-05-05 | state-manager | D-267 combined seal-and-fix — H-P24-001 BC-1.05.036 EC-006 truncated:bool annotation aligned to `/* */` form (TD-VSDD-076 self-violation corrected); 6 MEDs + 3 LOWs closed via lessons-corpus repair (open-backlog stubs filled, section-boundary fixed, marker orphans repaired, TD-VSDD-074 Source drift resolved); TD-VSDD-077 codified (lessons-corpus bidirectional coherence validation hook). |
| 1.25 | 2026-05-05 | state-manager | D-268 source-truth fix burst — H-P25-001 (BC-1.05.036:52 fabricated denial-path enumeration replaced with actual 4 emit_denial reasons per exec_subprocess.rs:148/155/162/169; env_allow silently filtered, cwd_allow unenforced per gap-analysis §1 clarified); M-P25-001 (BC-1.05.036 §EC-003 tightened to enumerate 4 real denial reasons + env_allow/cwd_allow non-triggering note); M-P25-002 (BC-1.05.036:50 Instant cite corrected — line 270 is post-spawn deadline, implementer must add started=Instant::now() before spawn at line 252); L-P25-001/002 SKIPPED with rationale; TD-VSDD-078 codified (BC postcondition source-of-truth enumeration verification — extends TD-VSDD-075). |
| 1.26 | 2026-05-05 | state-manager | D-270 silence-audit fix burst — H-P27-001 (BC-1.05.036:51 stale "file/datadog/honeycomb per config" multi-sink wording replaced with ADR-015 D-15.1 single-stream FileSink; Router/SinkRegistry retired per ADR-015 lines 130, 154; source-code verified); M-P27-001 (Postcondition 5 INTERNAL_ERROR (-99) enumeration added: spawn failure exec_subprocess.rs:252, stdin take/write :258/:262, stdout/stderr take :267-268, try_wait error :299; const at host/mod.rs:184; TD-VSDD-075+078 applied). ADR-013 clock RESET 0_of_3. |
| 1.27 | 2026-05-05 | state-manager | D-271 comprehensive sibling-sweep fix burst — H-P28-001 (BC-1.05.036:38 §Description "normal sink chain" replaced with ADR-015-correct emit_internal/FileSink wording); H-P28-002 (BC-1.05.036:135 §Purity "sink chain + non-blocking try_send" replaced with actual synchronous Mutex::lock+Vec::push per host/mod.rs:105-116); M-P28-001 (EC-007 INTERNAL_ERROR row added to §Edge Cases); M-P28-002 (INTERNAL_ERROR test vector row added to §Canonical Test Vectors); M-P28-003 (EC-005 + Test Vector OUTPUT_TOO_LARGE aligned to EC-004 sibling form with "NO event emitted in v1" qualifier); L-P28-001 ("retired" → dual-verb "removed per line 154 / retired per line 130" per ADR-015 lifecycle taxonomy). Source-of-truth verification per TD-VSDD-075+078 applied to all 6 fixes. TD-VSDD-079 codified (TD-VSDD-076 extension: terminology-family grep checklist for sibling-sweep fixes; 3rd recurrence threshold met). ADR-013 clock RESET 0_of_3. |
| 1.28 | 2026-05-05 | state-manager | D-272 cross-doc terminology drift fix — H-P29-001 (BC-1.05.036:51 "external fan-out to Datadog/Honeycomb" → "external export to remote observability backends" — scrubs fan-out + vendor names per TD-VSDD-079 8-term family grep); H-P29-002 (BC-1.05.035:35 §Description NUL-byte attribution corrected — "rejects NUL bytes" removed from canonicalize; redirected to `read_wasm_string` error path per §Postcondition 2 + §Precedence Ladder + §EC-005); full TD-VSDD-079 8-term grep across all 5 in-scope files (all non-changelog body: ZERO prohibited matches PASS); TD-VSDD-080 codified (mechanize TD-VSDD-079 as pre-commit hook; 5 consecutive narrative-discipline failures forces mechanical enforcement). ADR-013 clock 0_of_3 (reset by pass-29; remains 0). |
| 1.29 | 2026-05-05 | state-manager | D-274 inverse-traceability fix — MED-P31-001 (gap-analysis:334-337 + audit-w16 B-7 row tense corrected: "are injected"/"automatic invariant" → "MUST be injected per D-15.4, normative future-state; pending E-10 Wave 1"); MED-P31-002 (BC-1.05.036 Postcondition 2: outcome enum field added + exit_code→outcome mapping per ADR-015 D-15.2:270); LOW-P31-003 (BC-1.05.036 stdin write-failure cite :262 → :259); LOW-P31-004 (perf-baseline "sub-millisecond I/O" → "measured throughput >10k events/minute per ADR-015 D-15.1 Rationale"); LOW-P31-005 SKIPPED (tense conflation cosmetic per S-7.03 SHIP-AS-IS). Source-of-truth verification per TD-VSDD-075: exec_subprocess.rs:242-247 env_clear+selective-forward confirmed; :259 write_all().is_err() confirmed; ADR-015 D-15.2:270 outcome enum confirmed; D-15.4:407-419 MUST-be-injected confirmed; D-15.1 Rationale:432-440 10k events/minute confirmed. ADR-013 clock 0_of_3 (RESET by pass-31 SUBSTANTIVE). |
| 1.30 | 2026-05-05 | state-manager | D-276 PC↔TV coherence fix — MED-P33-001 (BC-1.05.036: EC-008 outcome-enum-stamping row added + 2 Canonical Test Vector rows for outcome=success/outcome=failure; Postcondition 2 outcome-enum mandate now has test coverage); MED-P33-002 (BC-1.05.035: §Description pairing rationale added justifying INVALID_ARGUMENT+capability_denied novel pairing + EC-002 event-emission witness appended + Test Vector row 3 event assertion added with reason "symlink_traversal_escape"); MED-P33-003 (BC-1.05.035: Postcondition 1 misleading "(`../` absent, no NUL bytes)" parenthetical removed; replaced with `read_wasm_string` error path reference; EC-001 clarifying note added explaining CAPABILITY_DENIED via allow-list miss path — no separate `../` string-level guard exists); LOW-P33-001 (BC-1.05.035: §Description anchor corrected from §"How ADR-015 affects the telemetry gap" lines 339-349 → §"Existing denial-path telemetry" lines 341-351). Source-of-truth verification per TD-VSDD-075: gap-analysis H3 §"Existing denial-path telemetry" begins line 341 (confirmed); rename rationale at lines 343-351 (confirmed); exec_subprocess.rs:148/155/162/169 emit_denial 4 reasons all CAPABILITY_DENIED -1 (confirmed unchanged). TD-VSDD-079 8-term grep: BC-1.05.035 ZERO prohibited matches; BC-1.05.036 lines 38+51 are intentional ADR-015 retirement-status citations. ADR-013 clock 0_of_3 (RESET by pass-33 SUBSTANTIVE verdict). |
| 1.31 | 2026-05-05 | state-manager | D-277 mechanism-fix burst — HIGH-P34-001 (BC-1.05.035: NUL byte rejection corrected — `read_wasm_string` only rejects non-UTF-8; NUL bytes → Precedence Ladder step 2 → CAPABILITY_DENIED -1; Postcondition 2, Postcondition 1 preamble, EC-005, Precedence Ladder step (1) all corrected per source-truth at host/memory.rs:47-54); MED-P34-001 (BC-1.05.035 EC-001: binary_allow precondition explicitly added); MED-P34-002 (BC-1.05.036 §Related BCs: sibling-disclosure of novel INVALID_ARGUMENT+capability_denied 5th denial path appended); MED-P34-003 (gap-analysis §"Existing denial-path telemetry": INTERIM declaration added as source-of-truth anchor); LOW-P34-001 SKIPPED (outcome enum 3-site duplication cosmetic per S-7.03); LOW-P34-002 closed implicitly by Fix 1 rewrite; TD-VSDD-081 codified (mechanism-verification beyond string-presence-grep). |
| 1.32 | 2026-05-05 | state-manager | D-278 sibling-mechanism-sweep fix burst — HIGH-P35-001 (BC-1.05.035: EC-002 + Postcondition 4 + Ladder step (3) corrected from `..` scan mechanism to actual `canonical_path.starts_with(project_root)` prefix check; Path::canonicalize() resolves all `..` segments away — sibling-class error to NUL-byte mechanism caught at v1.30); MED-P35-001 (BC-1.05.035 Postcondition 3: "existing semantics preserved" replaced with explicit BEHAVIOR CHANGE disclosure — missing-binary -99 → -1); MED-P35-002 (BC-1.05.035 §Related BCs BC-1.05.036 row: reverse-direction sibling-disclosure NOTE added for success-path event class novelty); MED-P35-003 (BC-1.05.036 Postcondition 4: ADR-015 line-number citations replaced with stable quoted-phrase anchors); LOW-P35-001/002 SKIPPED per S-7.03; TD-VSDD-082 codified (Sibling-mechanism sweep + bidirectional-sibling-disclosure). |
| 1.33 | 2026-05-05 | state-manager | D-279 architectural-reframe burst — HIGH-P36-001 + HIGH-P36-002 closed via architectural reframe: dropped "trusted project-root prefix" coinage, dropped "symlink_traversal_escape" concept, dropped novel INVALID_ARGUMENT+capability_denied pairing. BC-1.05.035 reframed around TOCTOU prevention — canonicalization feeds canonical path to existing `binary_allowed()` check; symlink resolving to non-allow-list path becomes normal allow-list miss → CAPABILITY_DENIED (-1) via existing emit_denial("binary_not_on_allow_list") at exec_subprocess.rs:155. MED-P36-001/002/003 + LOW-P36-001 closed. BC-1.05.036 §Related BCs NOTE updated (novel pairing dropped). TD-VSDD-083 codified (architectural-concept-anchoring rule). ADR-013 clock RESET 0_of_3. |
| 1.34 | — | — | (reserved) |

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

### v1.4 (2026-05-03) — Pass-4 fix burst (fix-only mode)

Fix-only-no-new-prose discipline adopted per [process-gap] observation.

Fixes from W-16-E-9-pass-4-adversary.md:
- F-P4-002 [MED]: v1.1 changelog parenthetical updated to acknowledge wc supersession (1-line edit).
- F-P4-003 [LOW]: deferred (refinement, pending intent).

Cross-doc fixes (separate commits in same burst):
- F-P4-001 [HIGH]: STORY-INDEX line 282 BC-2.02.005 → BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036 (state-manager scope).

Lines: v1.3 (598L) → v1.4 (~602L).

### v1.5 (2026-05-03) — Pass-6 structural fix burst

Fix-only structural corrections from W-16-E-9-pass-6-adversary.md:
- F-P6-001 [MED]: v1.4 changelog heading depth corrected `##` → `###` (matches v1.1/v1.2/v1.3 peers; document outline restored).
- F-P6-002 [MED]: Changelog summary table at lines 434-439 — v1.4 row appended.
- F-P6-003 [LOW]: deferred to ADR-014 reauthoring cycle (BC-1.05.001..034 range convention is inherited from ADR-014; out-of-scope for E-9-only fix).

Lines: v1.4 (~614L) → v1.5 (~622L; +8L from minimal edits).

### v1.6 (2026-05-03) — Pass-7 structural fix burst (drop line-count footers)

Fix-only structural corrections from W-16-E-9-pass-7-adversary.md:
- F-P7-001 [MED]: v1.5 + v1.6 rows appended to Changelog summary table at lines ~441-442. Both rows added in this burst to break recurring "summary-table-row-missing-after-version-bump" regression cycle (F-P6-002 → F-P7-001 pattern oscillation).
- F-P7-002 [LOW]: per-version "Lines: X → Y" footer convention DROPPED. Author-estimated line counts caused F-P3-007, F-P7-002 historically. Future version blocks should NOT include line-count footers; rely on `wc -l` if line delta is needed.
- F-P5-001 [LOW]: deferred (cosmetic line-number drift in v1.4 changelog self-reference; per POLICY 1 append-only, leaving as historical).

Convention change: starting v1.6, version blocks omit "Lines: X → Y" footers. Apply to all future bumps.

[process-gap codified]: producer-side fix-burst-author workflow now drops line-count footers and explicitly asserts "summary-table latest-row" before commit.

### v1.7 (2026-05-05) — D-236 architect amendment: ADR-015 single-stream OTel contract awareness

**Context:** ADR-015 ("single-stream OTel event emission") was authored 2026-05-04 — AFTER E-9
v1.6 reached CONVERGENCE_REACHED at pass-10 on 2026-05-03. ADR-015 establishes the emit
contract that all native WASM hooks S-9.01..S-9.07 must follow. E-9 v1.6 was silent on this
contract. D-236 (2026-05-05) elevated E-10 (ADR-015 implementation epic) ahead of E-9 Burst 2
and required this 4-file amendment before story-writer authors S-9.01..S-9.07.

**Changes in v1.7:**

- **Frontmatter `inputs`:** ADR-015 added to the inputs list.
- **Frontmatter `last_amended`:** Updated to 2026-05-05.
- **D-9.2 section:** ADR-015 awareness block appended. Enumerates D-15.1 (single stream), D-15.2
  (OTel schema — event.name format, outcome enum), D-15.3 (host enrichment contract — plugin
  asserts domain fields only), D-15.4 (VSDD_TRACE_ID dispatcher injection for S-9.07
  exec_subprocess). Story-writer MUST incorporate ADR-015 compliance ACs into S-9.01..S-9.07
  story bodies when authored in Burst 2.
- **R-W16-003:** ADR-015 note appended confirming per-wave telemetry events route through
  single-stream contract; emit overhead is negligible per D-15.1 rationale.
- **Changelog summary table:** v1.7 row added; v1.8 preemptive row added (per D-232 convention).

**No new BCs, VPs, or FRs added (per D-236 scope constraint).**
**Story bodies S-9.01..S-9.07 not touched — story-writer authors those in Burst 2.**

### v1.8 (2026-05-05) — D-242 fix burst: close pass-3 SUBSTANTIVE findings

**Context:** ADR-013 clock reset to 0_of_3 after pass-3 SUBSTANTIVE verdict. This fix burst
closes all 4 substantive findings (H-1 + M-1 + M-2 + M-3) and L-1. Requires 3 consecutive
fresh-context NITPICK_ONLY passes to reach CONVERGENCE_REACHED.

**H-1 CLOSED (block-event misattribution — 4 sites, 2 files):**
- **Option (b) chosen:** Dropped the plugin-side MUST to emit `outcome = "blocked"`. Rationale:
  the validate-* hooks are general-purpose validation plugins with no domain payload to add on
  the block path beyond what the dispatcher already provides (`plugin.name`, `hook.tool_name`,
  `outcome=blocked` in `vsdd.block.plugin_blocked.v1`). Option (a) would require defining a
  distinct event family per hook type with no additional semantic value. The dispatcher's
  automatic emission fully satisfies D-15.3. All 4 sites updated to reflect the corrected
  contract: plugin returns `HookResult::Block`; dispatcher emits audit event automatically.
- Site 1: E-9 lines ~294-302 (this file, D-9.2 ADR-015 awareness block)
- Sites 2-4: audit-w16.md lines 35, 37, 47-48

**L-1 CLOSED (VSDD_TRACE_ID wording precision):**
- Reworded E-9 lines ~297-299: "S-9.07's verify-sha-currency.sh subprocess inherits
  `VSDD_TRACE_ID` and `VSDD_PARENT_SPAN_ID` automatically via the dispatcher's Command::new
  env setup (ADR-015 D-15.4) — no plugin manifest change needed."

**M-1 CLOSED (vsdd.host.* prefix MUST-vs-pending contradiction):**
- gap-analysis-w16-subprocess.md lines ~320-324: replaced MUST assertion with binary-choice
  frame. Proposed name `vsdd.host.exec_subprocess.completed.v1` pending registry decision;
  fallback `vsdd.dispatcher.subprocess_completed.v1` if prefix not registered before E-10
  Wave 1. Forward-pointer added requiring SS-01 or E-10 Wave 1 author to resolve.

**M-2 CLOSED (internal.capability_denied rename path unresolved):**
- gap-analysis-w16-subprocess.md lines ~333-340: chose `vsdd.capability.denied.exec_subprocess.v1`.
  Rationale: ADR-015 maps `vsdd.capability.denied.*` to `audit` category (correct for a denial
  event); `vsdd.internal.*` maps to `lifecycle` (wrong semantic);
  Wave 3 AC-3 queries `event.category=audit` for SIEM dashboards. Soft "conformance issue"
  language replaced with firm MUST for SS-01 implementer in E-10 Wave 1 or 2.

**M-3 CLOSED (perf-baseline frontmatter references propagation gap):**
- perf-baseline-w16.md frontmatter `references:` appended ADR-015 row after ADR-013.

**L-2 SKIPPED:** D-239 lessons.md codified annotate-in-place as the arch doc convention;
`last_amended:` absence on arch docs is by-design. Finding invalid.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.9 (2026-05-05) — D-244 minimal fix burst: fabricated AC-3 citation + line range

**H-P4-001 CLOSED (fabricated Wave 3 AC-3 citation in M-2 rationale leg (c)):**
- Adversary pass-4 (citation-grounding angle, TD-VSDD-057) identified that leg (c) of the
  M-2 rationale in both fix sites cited a non-existent "Wave 3 acceptance criterion 3".
  ADR-015 Wave 3 defines exactly two ACs: AC-1 (`pr_throughput` row) and AC-2
  (`unknown_category_events` Grafana panel). There is no AC-3.
- **Site 1 (gap-analysis-w16-subprocess.md lines ~342-344):** Leg (c) rewritten to re-anchor
  to the real ADR-015 D-15.2 taxonomy registry (lines 295-333), which defines `audit` as the
  category for `vsdd.capability.denied.*` events. All AC-3 wording removed.
- **Site 2 (E-9 changelog v1.8 M-2 closure entry):** Leg (c) corrected wording: "audit-category
  events are SIEM-queryable by `event.category=audit` filter (ADR-015 D-15.2 taxonomy registry)."
  (NOTE: v1.8 block above retains the fabricated wording as historical record per POLICY 1
  append-only; the corrected anchor lives only here.)
- Re-verification: ADR-015 lines 295-333 confirmed — `vsdd.capability.denied.*` → `audit`
  category in registry table (line 329). PASS.

**L-P4-001 CLOSED (stale line range in H-1 closure):**
- E-9 H-1 closure "Site 1: E-9 lines ~294-296" updated to "~294-302" to bracket the
  full ADR-015 awareness block edits applied in v1.7.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.10 (2026-05-05) — D-246 fix burst: pass-5 H-P5-001 + M-P5-001 + M-P5-003

**H-P5-001 CLOSED (frontmatter version drift — "1.8" not bumped at v1.9):**
- Frontmatter `version:` corrected from `"1.8"` to `"1.10"` (skipping directly to v1.10 since
  this burst is the v1.10 delivery). TD-VSDD-059 pre-commit check: frontmatter version now
  matches the latest non-reserved row in the Changelog summary table (1.10). PASS.
- Recurrence note: this is the third frontmatter-vs-summary-table drift (F-P6-002, F-P7-001,
  H-P5-001). TD-VSDD-059 will codify a hook asserting frontmatter.version == max changelog row.

**M-P5-001 CLOSED (v1.8 block in-place rewrite violated POLICY 1 append-only):**
- The v1.9 burst (D-244, commit 067379c) rewrote the v1.8 M-2 closure entry in-place. This
  silently destroyed the historical record of the fabricated "Wave 3 AC-3" wording.
- Resolution (recommended option per D-245 lessons): v1.8 block prose restored to original
  wording — "Wave 3 AC-3 queries `event.category=audit` for SIEM dashboards" — which was
  the text authored at v1.8 (defective but historically accurate). The v1.9 H3 section
  records the correction with a POLICY 1 forward-pointer note: the corrected anchor lives
  only in the v1.9 block, not retroactively in the v1.8 block.

**M-P5-003 CLOSED (audit-w16.md B-7 row missing block-mode H-1 option (b) treatment):**
- audit-w16.md amended: B-7 row now explicitly states validate-wave-gate-prerequisite
  (S-9.07) is block-mode with H-1 option (b) treatment. See audit-w16.md for details.
- Cross-doc consistency achieved: all 5 block-mode hooks (factory-path-root B-1,
  input-hash B-2, pr-merge-prerequisites B-3, template-compliance B-6,
  wave-gate-prerequisite B-7) now have explicit H-1 option (b) coverage.

**M-P5-002 SKIPPED:** Gap-analysis-w16-subprocess.md frontmatter version v1.0 vs body
annotations (v1.7, 2026-05-05). D-239 lessons.md codified annotate-in-place as the
arch doc convention; body version annotations are informational. Reconciliation tension
acknowledged but D-239's convention overrides. No action.

**L-P5-001 SKIPPED:** Cosmetic syntax inconsistency. No semantic impact.

**L-P5-002 SKIPPED:** Acknowledged non-defect per adversary pass-5 findings.

**L-P5-003 SKIPPED:** Pending intent verification; deferred to next cycle.

**TD-VSDD-058 citations re-verified:** ADR-015 D-15.2 taxonomy registry (lines 295-333)
confirmed. ADR-015 D-15.3 block-event dispatcher emission confirmed. No citation errors found.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.11 (2026-05-05) — D-248 fix burst: close pass-6 H-P6-001 + M-P6-002; defer 3 MED + 2 LOW

**Context:** Adversary pass-6 (adversarial-implementer + boundary-cases hybrid angle, new per
TD-VSDD-057) found that the v1.10 M-P5-003 closure claim "all 5 block-mode hooks now have
explicit H-1 option (b) coverage" was overstated. B-2 (validate-input-hash, S-9.02) and B-6
(validate-template-compliance, S-9.06) were lumped into the "Standard." row in audit-w16.md
line 38 without explicit H-1 option (b) wording. Only B-1 (line 35), B-3 (line 37), and B-7
(line 36) had explicit dispatcher-emits-automatically text. Per D-247 lesson codified in
TD-VSDD-061: closure claims of form "all N items covered" must enumerate the N items.

**H-P6-001 CLOSED (B-2 + B-6 missing explicit H-1 option (b) coverage in audit-w16.md):**
- audit-w16.md line 38 amended (Option 2 — append to existing row): appended sentence
  explicitly stating both validate-input-hash (B-2) and validate-template-compliance (B-6)
  follow H-1 option (b): plugins return `HookResult::Block`; dispatcher automatically emits
  `vsdd.block.plugin_blocked.v1` per ADR-015 D-15.3. No additional plugin-side block emission
  required.
- TD-VSDD-061 enumeration: all 5 block-mode hooks now have explicit H-1 option (b) treatment
  in audit-w16.md:
  - factory-path-root (B-1): line 35 — "validate-factory-path-root is block-mode; it returns
    `HookResult::Block` and the dispatcher automatically emits `vsdd.block.plugin_blocked.v1`"
  - input-hash (B-2): line 38 — appended sentence (this burst)
  - pr-merge-prerequisites (B-3): line 37 — "validate-pr-merge-prerequisites is block-mode...
    dispatcher emits `vsdd.block.plugin_blocked.v1` automatically on block path (D-15.3)"
  - template-compliance (B-6): line 38 — appended sentence (this burst)
  - wave-gate-prerequisite (B-7): line 36 — "validate-wave-gate-prerequisite (S-9.07) is also
    block-mode... plugins return `HookResult::Block` and the dispatcher emits
    `vsdd.block.plugin_blocked.v1` automatically per D-15.3"

**M-P6-002 CLOSED (binary-choice tracking — OQ-W16-001 filed):**
- gap-analysis-w16-subprocess.md lines ~320-326: forward-pointer replaced with citation to
  **OQ-W16-001** (`.factory/specs/open-questions.md`, filed by state-manager in parallel).
  OQ acceptance criterion: (a) `vsdd.host.*` added to ADR-015 D-15.2 registry, OR (b)
  event.name uses `vsdd.dispatcher.subprocess_completed.v1` exactly. SS-01 implementer or
  E-10 Wave 1 architect MUST close OQ-W16-001 before the host-emit-fix story merges.

**M-P6-001 DEFERRED:** Frontmatter convention drift — gap-analysis-w16-subprocess.md and
audit-w16.md do not carry ADR-015 in their `references:` frontmatter field, while
perf-baseline-w16.md does. Rationale: mirrors M-P5-002 deferral per D-244. D-239 codified
annotate-in-place as the arch-doc convention; the perf-baseline ADR-015 row was a one-off.
The broader frontmatter convention question is larger than this amendment surface. Defer to a
future "arch-doc frontmatter convention" sweep (file as TD-VSDD-062 if needed).

**M-P6-003 DEFERRED:** event.host_overrides observability obligations not enumerated in the
E-9 awareness block. Rationale: silence-audit finding — ADR-015 D-15.3 fully specifies the
3-element MUST. The awareness block is an intentionally high-level pointer; story-writer
authoring S-9.01..S-9.07 reads ADR-015 directly when writing ACs. Each story will trace BCs
to the specific D-15.x clauses it implements. No architect action needed.

**M-P6-004 DEFERRED:** event.schema_url per-event-family not specified. Rationale: same as
M-P6-003 — silence-audit on a D-15.2.d obligation. ADR-015 explicitly states event.schema_url
is informational/optional unless a breaking-change protocol triggers a bump. Story-writer
decides per-story whether the validate-* event family needs an explicit schema_url or can rely
on the Resource-level baseline.

**L-P6-002 DEFERRED:** input-hash literal `37151a4` vs F-P2-010 closure `[pending-recompute]`
consistency. Rationale: state-manager's responsibility. State-manager presumably recomputed
the hash at some point in the convergence cycle. No architect action.

**L-P6-003 INVALID:** TD-VSDD-059 not filed. Rationale: invalidated at seal time —
TD-VSDD-059 IS filed in `cycles/v1.0-brownfield-backfill/open-backlog-post-rc8.md` per D-245.
Information-asymmetry artifact (adversary lacked visibility to that file).

**TD-VSDD-058 citations re-verified:** ADR-015 D-15.3 block-path audit trail (lines 374-378)
confirmed — "When a plugin returns `HookResult::Block`, the dispatcher emits a
`vsdd.block.plugin_blocked.v1` event." ADR-015 D-15.2 registry ownership (line 300) and
unrecognized-prefix default (line 310) confirmed. Gap-analysis-w16-subprocess.md OQ-W16-001
path `.factory/specs/open-questions.md` uses convention path (state-manager parallel burst).

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.11"` matches latest
non-reserved row in Changelog summary table (1.11). PASS.

**TD-VSDD-061 closure-claim enumeration:** all 5 block-mode hooks enumerated by ID and
audit-w16.md line (B-1 line 35, B-2 line 38, B-3 line 37, B-6 line 38, B-7 line 36).

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.12 (2026-05-05) — D-250 minimal fix burst: close pass-7 line 38 trio (M-P7-001/002/003 + L-P7-001)

**Context:** Adversary pass-7 (cross-doc consistency / contradiction hunt angle, new per TD-VSDD-057)
found 0 HIGH (first 0-HIGH SUBSTANTIVE pass) and 3 MED all localized to audit-w16.md line 38.
D-249 codified two new lessons: TD-VSDD-062 (sibling-template-consistency) and TD-VSDD-063
(fix-burst-internal-nomenclature-leakage check).

**M-P7-001 CLOSED (en-dash range → explicit list):**
- audit-w16.md line 38: `(S-9.02–S-9.06)` replaced with `(S-9.02, S-9.04, S-9.05, S-9.06)` —
  exact mirror of batch list `B-2, B-4, B-5, B-6`. Eliminates inclusivity ambiguity that
  incorrectly implied S-9.03 was covered by this row.

**M-P7-002 CLOSED ("H-1 option (b)" → "are block-mode", TD-VSDD-063):**
- audit-w16.md line 38: `follow H-1 option (b):` replaced with `are block-mode:`. Matches
  wording style of peer rows (lines 35/37). Eliminates fix-burst internal nomenclature leakage.

**M-P7-003 CLOSED (PostToolUse parenthetical added, TD-VSDD-062):**
- audit-w16.md line 38: `(PostToolUse:Edit|Write, on_error=block)` parenthetical added after
  B-2 and B-6 hook references. Matches structural form of sibling rows 36 (B-7: PreToolUse:Agent)
  and 37 (B-3: PreToolUse:Agent).

**L-P7-001 CLOSED ("per ADR-015 D-15.3" → "per D-15.3"):**
- audit-w16.md line 38: `per ADR-015 D-15.3` corrected to `per D-15.3`. Matches peer-row
  citation style (lines 35/36/37 all use bare `D-15.3`).

**Final rewritten sentence (all four closures):**
> "Both validate-input-hash (B-2, PostToolUse:Edit|Write, on_error=block) and
> validate-template-compliance (B-6, PostToolUse:Edit|Write, on_error=block) are block-mode:
> plugins return `HookResult::Block`; dispatcher automatically emits
> `vsdd.block.plugin_blocked.v1` per D-15.3. No additional plugin-side block emission required."

**L-P7-002 DEFERRED:** label "Standard" asymmetric for row containing 2 block-mode hooks.
Rationale: defensible — row groups by file-read pattern; block-mode aspect is captured as an
exception in the cell body (explicit parenthetical added by M-P7-003). Changing the label
would require restructuring the table, which is out of scope for a minimal fix burst.

**L-P7-003 DEFERRED:** gap-analysis line 17 "BC-1.05.001..034" range notation.
Rationale: verification requires exhaustive BC enumeration; out of scope for this burst.

**TD-VSDD-058 citation re-verification:** `D-15.3` confirmed as the correct ADR-015 clause
for dispatcher-emits-automatically (ADR-015 lines 375-376: "When a plugin returns
`HookResult::Block`, the dispatcher emits a `vsdd.block.plugin_blocked.v1` event"). PASS.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.12"` matches latest
non-reserved row in Changelog summary table (1.12). PASS.

**TD-VSDD-061 closure-claim enumeration:** all 5 block-mode hooks now consistently described
in audit-w16.md with event-type parenthetical, "is/are block-mode" wording, and bare D-15.3 citation:
- B-1 (line 35): validate-factory-path-root — "validate-factory-path-root is block-mode; it returns `HookResult::Block` and the dispatcher automatically emits `vsdd.block.plugin_blocked.v1`" (no parenthetical; pure stdin-parse row uses different form — acceptable per row structure).
- B-2 (line 38): validate-input-hash — `(PostToolUse:Edit|Write, on_error=block)` parenthetical added (this burst).
- B-3 (line 37): validate-pr-merge-prerequisites — "is block-mode (PreToolUse:Agent, on_error=block)" per D-15.3.
- B-6 (line 38): validate-template-compliance — `(PostToolUse:Edit|Write, on_error=block)` parenthetical added (this burst).
- B-7 (line 36): validate-wave-gate-prerequisite — "is also block-mode (`PreToolUse:Agent`, `on_error=block`)" per D-15.3.

**TD-VSDD-062 sibling-template-consistency:** new line 38 sentence diffed against lines 35/36/37.
Event-type parenthetical: present (B-2 and B-6 both carry PostToolUse:Edit|Write, on_error=block). PASS.
"are block-mode" wording: matches "is block-mode" form used in lines 35/37. PASS.
"per D-15.3" citation: bare form matches lines 35/36/37 (none use "per ADR-015 D-15.3"). PASS.

**TD-VSDD-063 fix-burst-internal-nomenclature scan:** grep `H-\d`, `M-P\d`, `F-P\d`, `L-P\d`
across audit-w16.md, gap-analysis-w16-subprocess.md, perf-baseline-w16.md body sections
(excluding changelog). Result: 0 matches in gap-analysis-w16-subprocess.md and perf-baseline-w16.md.
In audit-w16.md: only match was the pre-fix `H-1 option (b)` on line 38 — now replaced with
"are block-mode". No residual fix-burst nomenclature leakage in permanent spec body. PASS.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.13 (2026-05-05) — D-251 combined seal-and-fix burst: pass-8 M-P8-001 perf-baseline misanchor closed

**Context:** Adversary pass-8 (story-writer simulation + reverse-derivation outbound-decision-ID semantic-anchor
check angle, NEW per TD-VSDD-057) found 1 MED (M-P8-001) and 0 HIGH/LOW — severity gradient at its lowest
point in the convergence cycle (pass-3=6, pass-4=2, pass-5=7, pass-6=7, pass-7=6, pass-8=1).

**M-P8-001 CLOSED (perf-baseline-w16.md line 156 cross-document misanchor):**
- perf-baseline-w16.md "W-16 Gate Model" section line 156 cited `E-9 D-9.4 "Option C"` as gate-model
  authority. D-9.4 in the E-9 epic body is "BC Anchor Strategy — reuse existing BC-7.xx family per hook";
  its "Option C" is a back-reference to E-8 D-2 (BC reuse), not a gate-model decision.
- Fix applied: `E-9 D-9.4 "Option C" +` → `E-9 AC-3 +` (AC-3 at E-9 line 368 explicitly enumerates the
  latency-primary + advisory-ceiling model, which is the gate-model concept this perf-baseline section
  references).

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.12"` → `"1.13"` (matches latest
non-reserved row). PASS.

**Process-gap PG-P8-001 codified as TD-VSDD-065:** Decision-ID outbound semantic-anchor check — no
pre-commit hook currently validates that decision IDs cited from arch docs to epic docs (D-9.x, AC-N)
correspond to cited semantic content in target. M-P8-001 is the class this gap creates.

**TD-VSDD-064 sequential-burst protocol applied (first use):** This is the first burst applying the new
TD-VSDD-064 sequential-burst protocol where state-manager handles a minor cosmetic fix atomically with
the seal, avoiding the parallel commit collision pattern. The fix is single-line; architect involvement
adds no value. This burst sets the precedent that minimal cosmetic fixes (single-line textual corrections
where the architect is not needed for judgment) can be folded into a state-manager seal burst.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.14 (2026-05-05) — D-254 combined seal-and-fix burst

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (second application).**

Pass-11 verdict: SUBSTANTIVE 1H/1M/0L. Numerical-consistency angle (NEW per TD-VSDD-057) caught AC-3's superseded "~14MB" advisory soft cap — ADR-014 line 45 explicitly retired this projection in favor of 643,686 bytes (rc.1 × 2). ADR-013 clock RESET 2_of_3 → 0_of_3.

**Fix 1 — H-P11-001 CLOSED:** AC-3 line 368 `(~14MB)` replaced with `(soft_cap = perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes = 643,686 bytes per ADR-014 R-8.09 Amendment)`. Removes the superseded "~14MB" projection (originated in audit-w16.md Section 5 R-W16-003 and retired by ADR-014 line 45) and anchors AC-3 directly to the computed measurement value.

**Fix 2 — M-P11-001 CLOSED:** open-questions.md line 20 `Source:` field scrubbed of fix-burst-internal IDs (`D-247`, `pass-6 finding`, `M-P6-002`, cycle SHA `b04843d`). Replaced with neutral semantic anchor: `gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" (M-1 closure forward-pointer to OQ-W16-001)`. Per TD-VSDD-063 + TD-VSDD-066 (register-class permanent specs scope extension).

**Process-gap — TD-VSDD-067 codified:** Numerical-consistency angle revealed AC-numbered values were never cross-validated against underlying measurement source. File TD-VSDD-067 (Numeric-cross-anchor review axis for adversary). Adversary checklist should add "enumerate every numeric claim in spec/AC text and cross-validate against the underlying measurement source (perf-baseline, ADR amendment, S-N.NN baseline)."

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.13"` → `"1.14"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (second use):** State-manager handles both pass-11 seal and 2-line minimal fix atomically, avoiding parallel commit collision. Both fixes are textual corrections where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.15 (2026-05-05) — D-255 combined seal-and-fix burst (recursive-scrub applied)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (third application).**

Pass-12 verdict: SUBSTANTIVE 1H/3M/1L. v1.14 diff-only line-by-line + AC-3 sibling-style audit (hybrid angle, NEW per TD-VSDD-057). ADR-013 clock 0_of_3 (reset by pass-12; remains 0 after this burst — this burst seals pass-12, not a fresh pass).

**Fix 1 — H-P12-001 CLOSED:** open-questions.md line 20 `Source:` field replaced. Previous v1.14 text contained `M-1 closure` — fix-burst-internal nomenclature that: (a) refers to E-9 changelog v1.8 H3 entry M-1 (internal to the fix-burst audit record, same forbidden class as `D-247`, `M-P6-002`, `pass-6 finding`, `b04843d` that M-P11-001 removed); and (b) forms an unresolvable forward-pointer — grep of gap-analysis-w16-subprocess.md for "M-1" returns zero matches. New text: `gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" — see also gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**") for the bidirectional anchor.` Per TD-VSDD-063 + TD-VSDD-066 + TD-VSDD-068 (recursive-scrub).

**Fix 2 — L-P12-001 CLOSED:** open-questions.md Question prose replaced version-internal pointer `E-9 v1.10 amendment` with version-tolerant reference `the gap analysis of \`host::exec_subprocess\` (gap-analysis-w16-subprocess.md §5)`. Same TD-VSDD-066 class as L-P9-001/M-P11-001 but in the Question body rather than the Source field.

**Fix 3 — M-P12-001/002/003 CLOSED:** AC-3 parenthetical rewritten from pseudo-code form `(soft_cap = perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes = 643,686 bytes per ADR-014 R-8.09 Amendment)` to prose form `(advisory soft cap target = 643686 bytes; computed as v1.0.0-rc.1 baseline × 2 per ADR-014 Amendment 2026-05-03 "R-8.09 ceiling model revised"; baseline value pinned in perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes)`. This simultaneously: (M-P12-001) uses canonical ADR-014 label including disambiguation date 2026-05-03 and quoted title "R-8.09 ceiling model revised" (vs the D-9.2-withdrawn amendment); (M-P12-002) converts pseudo-code `soft_cap = ...` form to prose matching sibling ACs; (M-P12-003) removes comma from byte count — `643,686` → `643686` matching perf-baseline-w16.md:163 source field exactly.

**TD-VSDD-068 recursive-scrub applied INLINE (pre-commit verification):**

After Fix 1 + Fix 2 on open-questions.md:
```
grep -nE 'D-2[0-9]{2}|M-P[0-9]+|H-P[0-9]+|L-P[0-9]+|F-P[0-9]+|M-[0-9]+ closure|pass-[0-9]+ finding' \
  /Users/jmagady/Dev/vsdd-factory/.factory/specs/open-questions.md
```
Result: ZERO matches. PASS.

After Fix 3 on E-9 epic (non-changelog sections):
```
grep -n '643,686\|R-8.09 Amendment\|soft_cap = perf' \
  /Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-9-tier-2-native-wasm-migration.md
```
Result: matches on lines 978 + 980 only — both inside `### v1.14` H3 historical changelog section (POLICY 1 immutable). AC table (line 368) ZERO matches. PASS.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.14"` → `"1.15"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (third use):** State-manager handles pass-12 seal and 3-fix burst atomically. All fixes are textual corrections where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.16 (2026-05-05) — D-256 last-mile fix burst (TD-VSDD-069 line-accuracy verification applied)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (fourth application).**

Pass-13 verdict: SUBSTANTIVE 0H/1M/2L. Outbound decision-ID exhaustive enumeration angle (NEW per TD-VSDD-057; ~80 anchors enumerated). ADR-013 clock 0_of_3 (reset by pass-12 D-255; remains 0 after this burst — this burst seals pass-13, not a fresh pass).

**Fix 1 — M-P13-001 CLOSED:** open-questions.md line 20 `gap-analysis line 326` → `gap-analysis line 325`. Off-by-one introduced by v1.15 H-P12-001 fix (same defect class as the fix was meant to close). TD-VSDD-069 line-accuracy verification applied INLINE: `grep -n 'Resolution tracked in \*\*OQ-W16-001\*\*' .factory/architecture/gap-analysis-w16-subprocess.md` → line 325 confirmed. Citation now accurate.

**Fix 2 — L-P13-001 CLOSED:** AC-3 line 368 ADR-014 amendment title quote restored `(research)` source-tag: `"R-8.09 ceiling model revised"` → `"R-8.09 ceiling model revised (research)"`. Matches ADR-014 line 38 H2 heading exactly. Changelog line 1003 (inside v1.15 H3, POLICY 1 immutable) left as-authored.

**Fix 3 — L-P13-002 CLOSED:** audit-w16.md line 36 (B-7 row) backticks dropped from `PreToolUse:Agent` and `on_error=block` to match sibling lines 37 (B-3) and 38 (B-2/B-6) wording template. No backticks in parenthetical for any of the 3 block-mode rows.

**TD-VSDD-069 line-accuracy verification applied INLINE (pre-commit):**

When a citation of form `<filename> line N ("<quoted text>")` is added or modified in a fix burst, scrub MUST grep `<quoted text>` in `<filename>` and confirm exactly one match at line `N`. Applied for Fix 1:
```
grep -n 'Resolution tracked in \*\*OQ-W16-001\*\*' \
  /Users/jmagady/Dev/vsdd-factory/.factory/architecture/gap-analysis-w16-subprocess.md
```
Result: line 325 (single match). Citation updated to 325. PASS.

**TD-VSDD-068 recursive-scrub (re-verified):**

After Fix 1 on open-questions.md:
```
grep -nE 'D-2[0-9]{2}|M-P[0-9]+|H-P[0-9]+|L-P[0-9]+|F-P[0-9]+|M-[0-9]+ closure|pass-[0-9]+ finding' \
  /Users/jmagady/Dev/vsdd-factory/.factory/specs/open-questions.md
```
Result: ZERO matches. PASS.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.15"` → `"1.16"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (fourth use):** State-manager handles pass-13 seal and 3-fix burst atomically. All fixes are textual corrections where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.17 (2026-05-05) — D-257 minimal fix burst (TD-VSDD-070 codified; FIFTH TD-VSDD-064 application)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (fifth application).**

Pass-14 verdict: SUBSTANTIVE 0H/1M/2L. AC chain audit + section-heading semantic anchor verification angle (NEW per TD-VSDD-057). ADR-013 clock 0_of_3 (reset by pass-13 D-256; remains 0 after this burst — this burst seals pass-14, not a fresh pass).

**Fix 1 — M-P14-001 CLOSED:** perf-baseline-w16.md line 154 H2 heading `(ADR-014 R-8.09 Revised — Option C)` → `(ADR-014 R-8.09 Amendment 2026-05-03)`. Pass-8 closed the in-line citation at line 156 (E-9 D-9.4 → E-9 AC-3) but did not scrub the section heading itself. ADR-014 R-8.09 Amendment 2026-05-03 has no Options A/B/C taxonomy. The "Option C" anchor in the H2 heading was non-resolving — same outbound-decision-ID class (TD-VSDD-065). Post-edit grep `"Option C" perf-baseline-w16.md` → ZERO matches. PASS.

**L-P14-001 SKIPPED with rationale:** audit-w16.md line 165 "D-2 Option C" mis-reference. Out of v1.7..v1.16 amendment scope; line 165 is pre-amendment audit-time prose. Per POLICY 1 append-only, retaining historical content is acceptable. Future "audit-w16 retrospective annotation" sweep can address.

**L-P14-002 SKIPPED with rationale:** perf-baseline-w16.md `producer: implementer` frontmatter. Convention question (does `producer` denote original author or latest amender?). D-239 lessons codified annotate-in-place for arch docs without bumping version; same convention extends to producer field.

**TD-VSDD-070 codified:** Extend TD-VSDD-065 scope from "in-text decision IDs" to "all section/subsection headings (`# H1`, `## H2`, `### H3`) that name an external authority's decision/option/choice/amendment." Pass-14's section-heading angle caught what 13 prior in-line-citation-focused passes missed.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.16"` → `"1.17"` (matches latest non-reserved row). PASS.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.18 (2026-05-05) — D-258 minimal OQ-table propagation fix (TD-VSDD-071 codified; SIXTH TD-VSDD-064 application)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (sixth application).**

Pass-15 verdict: SUBSTANTIVE 0H/1M/2L. Discoverability audit — story-writer reading-graph traversal + bidirectional anchor verification + symmetry audit angle (NEW per TD-VSDD-057). ADR-013 clock 0_of_3 (reset by pass-14 D-257; remains 0 after this burst — this burst seals pass-15, not a fresh pass).

**Fix 1 — M-P15-001 CLOSED:** OQ-W16-001 row appended to E-9 epic Open Questions table (after existing OQ-3 row). Was filed in v1.11/D-248 to `.factory/specs/open-questions.md:18` and cited in gap-analysis-w16-subprocess.md (bidirectional anchor at line 325), but E-9 epic Open Questions table — the canonical discoverability hub for OQ register entries gating E-9 stories — did not contain a row. Story-writer authoring S-9.07 reading E-9 §Open Questions would miss OQ-W16-001's binary-choice gate. Post-edit grep `"OQ-W16-001" E-9 epic` → matches in BOTH body Open Questions table AND historical changelog rows. PASS.

**L-P15-001 SKIPPED with rationale:** audit-w16.md frontmatter `version: "1.0"` despite body amendments. Mirror of M-P5-002/M-P6-001 deferrals. D-239 governs annotate-in-place convention for arch docs.

**L-P15-002 SKIPPED with rationale:** audit-w16.md line 165 "D-2 Option C" historical reference. Already SKIPPED in v1.17 changelog L-P14-001 entry. Out of amendment scope (pre-amendment audit-time prose); re-noted for completeness.

**TD-VSDD-071 codified:** OQ-table propagation hook — when an OQ is filed in `.factory/specs/open-questions.md` citing an E-N epic as scope-owner, the same burst MUST verify (or append) a corresponding row in the epic's Open Questions table. Adversary discoverability-audit angle should enumerate every OQ in open-questions.md and verify each is listed in its scope-owner epic's Open Questions table.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.17"` → `"1.18"` (matches latest non-reserved row). PASS.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.19 (2026-05-05) — D-260 sibling-residue fix burst (TD-VSDD-072 codified; SEVENTH TD-VSDD-064 application)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (seventh application).**

Pass-17 verdict: SUBSTANTIVE 2H/1M/1L. Linguistic uniformity / numerical-claim cross-table walk + section-heading semantic-anchor scan angle (NEW per TD-VSDD-057). ADR-013 clock 1_of_3 → 0_of_3 RESET.

**Fix 1 — H-P17-001 CLOSED:** R-W16-003 mitigation cell `(~14MB)` replaced with `(advisory soft cap target = 643686 bytes per ADR-014 R-8.09 Amendment 2026-05-03; pinned in perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes)`. Sibling regression of v1.14 H-P11-001 AC-3 fix — body-grep at v1.14 close would have caught it (TD-VSDD-072). Additionally: R-W16-003 body text "post-rc.4 baseline" updated to "S-9.00 baseline (post-rc.11, pinned in perf-baseline-w16.md)" (sibling propagation sweep per TD-VSDD-072 retired-figure body-grep extension).

**Fix 2 — H-P17-002 CLOSED:** perf-baseline-w16.md line 33 H2 heading `(post-rc.4, pre-Tier 2)` → `(post-rc.11, pre-Tier 2)`. Heading parenthetical was authored when measurement epoch was rc.4; measurement was retaken at rc.11 (dec5361) on 2026-05-05 but heading not updated. TD-VSDD-070 class (section-heading semantic-anchor) extended to release-tag descriptors.

**Fix 3 — M-P17-001 CLOSED:** OQ-1 question prose updated from "over the post-rc.4 baseline" to "over the S-9.00 baseline (post-rc.11, pinned in perf-baseline-w16.md)". Sibling propagation of H-P17-002. Version-tolerant phrasing per TD-VSDD-066.

**L-P17-001 SKIPPED with rationale:** `verify-sha-currency.sh` backtick inconsistency in E-9 body (line 300 lacks backticks while lines 39, 220 have them). Cosmetic only; no semantic impact. Deferred.

**TD-VSDD-072 codified:** Retired-figure body-grep extension to recursive-scrub. When a fix burst replaces a retired numeric or named value (e.g., "~14MB", "Option C", "post-rc.4"), the same burst MUST body-grep the entire file (and all amendment-scope files) for the retired value before commit. Any non-changelog occurrence is a sibling regression that must be fixed in the same burst. Extends TD-VSDD-068 recursive-scrub.

**Body-grep verification (TD-VSDD-072 applied inline):**

After Fix 1 + Fix 3 on E-9 epic:
```
grep -n '14MB\|14 MB' E-9-tier-2-native-wasm-migration.md
```
Result: matches on changelog summary table row (v1.14) and `### v1.14` H3 historical changelog section only (POLICY 1 immutable). Non-changelog body: ZERO matches. PASS.

```
grep -n 'post-rc\.4' E-9-tier-2-native-wasm-migration.md
```
Result: ZERO matches. PASS.

After Fix 2 on perf-baseline-w16.md:
```
grep -n 'post-rc\.4' perf-baseline-w16.md
```
Result: ZERO matches. PASS.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.18"` → `"1.19"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (seventh use):** State-manager handles pass-17 seal and 3-fix burst atomically. All fixes are textual corrections where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.20 (2026-05-05) — D-261 convention-closure burst (TD-VSDD-073 codified; EIGHTH TD-VSDD-064 application; FIRST definitive closure of recurring convention question)

**Pass-18 sealed (0H/1M/1L — frontmatter consistency audit angle NEW per TD-VSDD-057).**

**M-P18-001 CLOSED (5th-recurrence resolution per S-7.02):** 4 of 5 in-scope files (gap-analysis-w16-subprocess.md, audit-w16.md, perf-baseline-w16.md, open-questions.md) had `version: "1.0"` frontmatter with no `last_amended:` despite material body amendments across v1.7..v1.19. This was the 5th re-flag of the same convention question (M-P5-002 → M-P6-001 → L-P14-002 → L-P15-001 → M-P18-001). S-7.02 recurrence threshold (3+) met; definitive closure adopted.

**Option A adopted (TD-VSDD-073):** Added `last_amended: 2026-05-05` field to all 4 amendment-touched arch-doc-class files:
- gap-analysis-w16-subprocess.md — frontmatter `last_amended: 2026-05-05` added adjacent to `timestamp:`
- audit-w16.md — frontmatter `last_amended: 2026-05-05` added adjacent to `timestamp:`
- perf-baseline-w16.md — frontmatter `last_amended: 2026-05-05` added adjacent to `timestamp:`
- open-questions.md — frontmatter `last_amended: 2026-05-05` added adjacent to `timestamp:`

D-239 annotate-in-place body convention is preserved — body retains dated H2 amendment annotations; frontmatter gains parallel structured signal.

Going forward (TD-VSDD-073): when an arch-doc-class file gains a body amendment in any fix burst, the same burst MUST update `last_amended:` to the burst date.

**L-P18-001 CLOSED:** perf-baseline-w16.md references entry updated: `ADR-014 R-8.09 revised (Amendment 2026-05-03)` → `ADR-014 Amendment 2026-05-03 (R-8.09 ceiling model revised (research))`. Reference now matches ADR-014 H2 title canonical form; `(research)` source-tag restored consistent with v1.16 L-P13-001 AC-3 closure (D-256).

**ADR-013 clock:** 0_of_3 (was 0_of_3 entering pass-18 — reset by pass-17; remained 0 through pass-18 SUBSTANTIVE; this burst does not advance clock).

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.19"` → `"1.20"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (eighth use):** State-manager handles pass-18 seal and 2-fix burst atomically. Both fixes are textual additions/corrections where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.21 (2026-05-05) — D-263 implementation-readiness fix burst (TD-VSDD-074 codified; NINTH TD-VSDD-064 application)

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (ninth application).**

Pass-20 verdict: SUBSTANTIVE 0H/2M/2L. Pre-implementation readiness audit angle (NEW per TD-VSDD-057) — simulating S-9.07 implementer reading v1.20 surface as concrete spec. ADR-013 clock 1_of_3 → 0_of_3 RESET.

**Fix 1 — M-P20-001 CLOSED:** OQ-3 resolution column extended to pin `timeout_ms = 30000` (30s; gap-analysis §4 advisory cap; safety margin over 200ms expected runtime) and `max_output_bytes = 65536` (64KB; gap-analysis §4 "easily under 64KB" envelope; truncation Err'd per gap-analysis §3 line 127). S-9.07 implementer now has concrete call-site values for `vsdd::exec_subprocess(...)` — no value invention required.

**Fix 2 — M-P20-002 CLOSED:** BC-1.05.036 §Description gained ADR-015 awareness clause (added per E-9 v1.7 Post-Audit Amendment, propagated to BC at v1.21). Clause binds the interim event name `host.exec_subprocess.completed` to OQ-W16-001 binary resolution: (a) `vsdd.host.exec_subprocess.completed.v1` if `vsdd.host.*` added to ADR-015 D-15.2 registry, OR (b) `vsdd.dispatcher.subprocess_completed.v1` using existing lifecycle category. S-9.07 implementer directed to close OQ-W16-001 before merge.

**Fix 3 — L-P20-002 CLOSED (in same BC-1.05.036 edit):** BC-1.05.036 §Postconditions item 5 rewritten to clarify error-path event reality per gap-analysis §1: only `internal.capability_denied` exists on 4 denial paths; TIMEOUT (-7) and OUTPUT_TOO_LARGE (-8) paths return error codes WITHOUT emitting any event. Prior wording "existing distinct events continue to fire" implied timeout/output-too-large paths had named events, which is incorrect.

**Fix 4 — TD-VSDD-074 codified:** TD-VSDD-073 (last_amended convention for arch-doc-class files) scope extended to BCs cited in amendment landings. BC-1.05.035 + BC-1.05.036 gained `last_amended: 2026-05-05` in frontmatter. Going forward: when an amendment burst changes a contract that a BC implements, the same burst MUST update the BC's frontmatter `last_amended:` AND add an awareness clause to the BC body.

**L-P20-001 SKIPPED with rationale:** BC-1.05.036 EC-006 declares `binary: String /* canonicalized full path */` in event payload, while BC-1.05.035 canonicalizes the binary path for the capability allow-check. Pending PO intent verification per S-7.01 — BC author likely intended cache-and-reuse semantics (canonicalize once in BC-1.05.035 allow-check, reuse in BC-1.05.036 event payload). Would need PO clarification to definitively close. Adversary may re-flag if it persists.

**ADR-013 clock:** 0_of_3 (reset by pass-20 SUBSTANTIVE verdict; was 1_of_3 entering pass-20).

**Trajectory to convergence:** pass-19 NITPICK (clock 0→1) → pass-20 SUBSTANTIVE (clock 1→0 RESET). After this burst: 3 fresh-context NITPICK_ONLY passes (21/22/23) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.20"` → `"1.21"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (ninth use):** State-manager handles pass-20 seal and 4-fix burst atomically. All fixes are textual corrections/additions where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.22 (2026-05-05) — D-264 multi-fix burst: pass-21 2H/3M/2L; BC error codes + line cite + ADR-015 sibling + host category + truncated semantics; TD-VSDD-075 codified

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (TENTH application).**

Pass-21 verdict: SUBSTANTIVE 2H/3M/2L. BC-only deep-dive angle (NEW per TD-VSDD-057) — reads BC-1.05.035 and BC-1.05.036 as standalone implementer, cross-validates against source code and ADR-015 registry. ADR-013 clock RESET to 0_of_3.

**Fix 1 — H-P21-001 CLOSED (BC-1.05.036 fabricated error codes -7/-8 → -2/-3):**

Source-code verification per TD-VSDD-075: `crates/factory-dispatcher/src/host/mod.rs:181-182` confirms:
```rust
pub const TIMEOUT: i32 = -2;
pub const OUTPUT_TOO_LARGE: i32 = -3;
```
BC-1.05.036 §Postconditions item 5 corrected: `TIMEOUT (-7) and OUTPUT_TOO_LARGE (-8)` → `TIMEOUT (-2) and OUTPUT_TOO_LARGE (-3)`. The v1.21 burst (D-263 L-P20-002) invented codes -7/-8 without reading the source — a regression introduced by the fix burst itself.

**Historical-record note on v1.21 H3:** Per POLICY 1 (append-only), the v1.21 H3 changelog entry above retains the fabricated `-7`/`-8` values as historical audit record. The corrected values are `-2`/`-3` per source code as cited above. Readers of v1.21 H3 prose should apply the correction noted here.

**Fix 2 — H-P21-002 CLOSED (open-questions.md:21 line citation 325→326):**

Grep verification per TD-VSDD-075 dependent-citation-propagation sub-rule: `grep -n 'Resolution tracked in \*\*OQ-W16-001\*\*' .factory/architecture/gap-analysis-w16-subprocess.md` → line 326 (single match). The v1.20 D-261 burst added `last_amended: 2026-05-05` to gap-analysis-w16-subprocess.md frontmatter (line 8), shifting all subsequent lines +1. The v1.16 D-256 fix had set the citation to line 325 (correct at that time). This is the THIRD recurrence of the line-citation off-by-one class (after L-P9-001, M-P13-001) — S-7.02 threshold met; TD-VSDD-075 codified (see below).

**Fix 3 — M-P21-001 CLOSED (BC-1.05.035 ADR-015 awareness clause, TD-VSDD-074 symmetric application):**

BC-1.05.035 §Description gained ADR-015 awareness clause. The v1.21 burst added the clause to BC-1.05.036 but not BC-1.05.035, despite both BCs referencing `internal.capability_denied` (an INTERIM event name per the v1.7 amendment). Asymmetric TD-VSDD-074 application closed.

**Fix 4 — M-P21-002 CLOSED (BC-1.05.036:34 fabricated "host" ADR-015 category):**

BC-1.05.036 §Description option (a) read "host category mapping per OQ-W16-001 acceptance criterion (a)". ADR-015 D-15.2 registry has exactly 5 categories: `lifecycle | domain | audit | error | unknown` — no "host" category exists. Corrected to: "category to be assigned per OQ-W16-001 acceptance criterion (a) — ADR-015 D-15.2 registry has 5 categories: lifecycle, domain, audit, error, unknown". Same invented-value class as H-P21-001.

**Fix 5 — M-P21-003 CLOSED (BC-1.05.036 truncated:bool declared but semantically always false):**

§Postcondition 2 payload field `truncated: bool` and EC-006 both gained inline reservation note: `[reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1]`. Option (b) chosen: field retained for future ABI-breaking change when truncation becomes Ok-path; semantics documented explicitly so implementer does not waste effort setting `truncated = true`.

**L-P21-001 DEFERRED with rationale:** Interim-vs-canonical name whiplash in BC-1.05.036 (§Description qualifier vs §Postconditions unqualified use). Cosmetic readability only; awareness clause in §Description covers the implementer.

**L-P21-002 DEFERRED with rationale:** Rust line citations (exec_subprocess.rs:230, :270). Per S-7.03/D-231 SHIP-AS-IS pattern; implementing story refreshes at implementation time.

**TD-VSDD-075 codified (two sub-rules, both triggered by pass-21 HIGH findings):**

1. **Source-code-verification discipline:** Fix bursts that cite source-code constants (error codes, struct fields, enum variants) MUST read the actual source before commit. Quote the exact source line in the fix-burst commit message body as proof. H-P21-001 was a regression where D-263 invented `-7`/`-8` without verification.

2. **Dependent-citation-propagation discipline:** When a fix burst adds `last_amended:` (or any frontmatter field that shifts subsequent line numbers), the same burst MUST grep all in-scope files for inbound citations of form `<filename> line N`. For each match, re-grep the cited file for the quoted text and confirm the line number still resolves. Refresh stale citations in the same burst. H-P21-002 was the third recurrence of the off-by-one class caused by this omission.

Both sub-rules appended to `open-backlog-post-rc8.md` as TD-VSDD-075 and to `lessons.md`.

**ADR-013 clock:** 0_of_3 (reset by pass-21 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (22/23/24) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.21"` → `"1.22"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (tenth use):** State-manager handles pass-21 seal and 5-fix burst atomically. All fixes are textual corrections/additions where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.23 (2026-05-05) — D-265 sibling-sweep fix burst: pass-22 2H/3M/2L; BC sibling sections aligned + INTERIM qualifier + precedence ladder; TD-VSDD-076 codified

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (ELEVENTH application).**

Pass-22 verdict: SUBSTANTIVE 2H/3M/2L. Diff-only line-by-line + intra-document semantic-sibling sweep angle (NEW per TD-VSDD-057) — reads each line introduced by v1.22 burst, then sweeps affected BCs' sibling sections for prior wording contradicting v1.22 corrections. ADR-013 clock RESET to 0_of_3.

**Fix 1 — H-P22-001 CLOSED (BC-1.05.036 §Related BCs lines 61-62 + §EC-004 line 86 + §Canonical Test Vectors line 97 aligned to Postcondition 5 no-event reality):**

v1.22 correctly updated Postcondition 5 to state TIMEOUT/OUTPUT_TOO_LARGE return error codes WITHOUT emitting any event. But §Related BCs and §EC-004 within the SAME BC still said "emits a different event" (lines 61-62) and "Timeout error event emitted" (line 86) — direct contradictions. §Canonical Test Vectors line 97 also contradicted. All four sibling locations corrected to align with Postcondition 5.

This is the canonical evidence for TD-VSDD-076 (intra-document semantic-sibling sweep).

**Fix 2 — H-P22-002 CLOSED (BC-1.05.035 §Postcondition 4 INTERIM qualifier appended):**

BC-1.05.035 §Description (added v1.22 per TD-VSDD-074) declares `internal.capability_denied` is INTERIM and MUST be renamed. But §Postcondition 4 — the normative section — used the bare unqualified interim name. Implementer reading only §Postconditions received unqualified instruction contradicting §Description. Fix: appended INTERIM qualifier with rename target (`vsdd.capability.denied.exec_subprocess.v1` per ADR-015 D-15.2 registry line 329) to Postcondition 4.

**Fix 3 — M-P22-001 CLOSED (BC-1.05.036 Postcondition 1 scoped to success path):**

Postcondition 1 "Exactly one event is emitted" had no conditional clause; an implementer skimming to Postcondition 1 before reaching Postcondition 5 received false absoluteness. Updated to: "On successful subprocess completion (i.e., subprocess process actually exits before timeout AND within output cap; see Postcondition 5 for error-path reality), exactly one event is emitted."

**Fix 4 — M-P22-002 CLOSED (OQ-W16-001 acceptance (a) AND-linked to canonical event name):**

Acceptance criterion (a) required only the registry prefix-to-category entry. A compliant amendment could add `vsdd.host.* | lifecycle` without specifying the canonical event name suffix, leaving an ambiguity. Fix: tightened criterion (a) to AND-link the registry entry with the canonical event name `vsdd.host.exec_subprocess.completed.v1`.

**Fix 5 — M-P22-003 CLOSED (BC-1.05.035 precedence ladder appended to §Postconditions):**

Four §Postconditions stated independent conditions with no ordering. Implementer working from BC alone could not determine which error code fires when multiple conditions apply simultaneously. Fix: appended explicit precedence ladder — (1) NUL byte → `INVALID_ARGUMENT` (-4); (2) `canonicalize()` fails → `CAPABILITY_DENIED` (-1); (3) `..` in canonicalized path → `INVALID_ARGUMENT` (-4); (4) not in allow-list → `CAPABILITY_DENIED` (-1). Per `exec_subprocess.rs:230` entry point.

**L-P22-001 SKIPPED with rationale:** Conditional "may be tracked in a future OQ" prose in Postcondition 5 is cosmetic; normative claim (no event on error paths) is clear. Per S-7.03/D-231 SHIP-AS-IS pattern.

**L-P22-002 absorbed into H-P22-001:** §Canonical Test Vectors line 97 "Timeout event emitted" was also contradicting Postcondition 5; corrected as part of H-P22-001 sibling sweep.

**TD-VSDD-076 codified (intra-document semantic-sibling sweep extension to TD-VSDD-075):**

When a fix burst corrects a Postcondition or any normative claim within a BC, the same burst MUST grep the SAME BC for sibling sections (§Related BCs, §Edge Cases, §Canonical Test Vectors, §Postconditions, §Description) for prior wording that contradicts the correction. Each contradicting sibling must be updated in the same burst. TD-VSDD-075 covered inter-document citation refresh and source-code-verification; TD-VSDD-076 extends it to intra-document semantic siblings.

Appended to `open-backlog-post-rc8.md` as TD-VSDD-076 and to `cycles/v1.0-brownfield-backfill/lessons.md`.

**ADR-013 clock:** 0_of_3 (reset by pass-22 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (23/24/25) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.22"` → `"1.23"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (eleventh use):** State-manager handles pass-22 seal and 5-fix burst atomically. All fixes are textual corrections/additions where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.24 (2026-05-05) — D-267 combined seal-and-fix: pass-24 1H/6M/3L; BC sibling annotation alignment + lessons-corpus repair; TD-VSDD-077 codified

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (TWELFTH application).**

Pass-24 verdict: SUBSTANTIVE 1H/6M/3L. Convention-meta audit angle (NEW per TD-VSDD-057) — audits lessons-corpus artifacts (lessons.md + open-backlog-post-rc8.md) themselves for coherence defects introduced by the 20 TD-VSDD codification bursts. ADR-013 clock RESET to 0_of_3.

**Fix 1 — H-P24-001 CLOSED (BC-1.05.036 EC-006 truncated:bool annotation aligned to `/* */` form — TD-VSDD-076 self-violation corrected):**

BC-1.05.036 §Postconditions item 2 (line 49) used `/* ... */` C-style annotation for `truncated: bool`. BC-1.05.036 §Edge Cases EC-006 (line 88) used `[ ... ]` bracket annotation for the SAME field with identical explanatory text. Intra-document semantic-sibling annotation style inconsistency — precisely the class TD-VSDD-076 mandates sweeping. Fix: EC-006 line 88 aligned to `/* ... */` matching §Postconditions item 2. Both sites now read `truncated: bool /* reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1 */`.

**Fix 2 — M-P24-001/002/003 CLOSED (open-backlog-post-rc8.md stubs filled for TD-VSDD-069/071/075):**

Three stub entries had body content merged into the next numbered entry: TD-VSDD-069 body was concatenated with TD-VSDD-070; TD-VSDD-071 body was concatenated with TD-VSDD-072; TD-VSDD-075 body was concatenated with TD-VSDD-076. Fix: split all merged entries — each TD-VSDD-NNN entry now contains only its own body content. TD-VSDD-069 (line-accuracy extension to recursive-scrub), TD-VSDD-071 (OQ-table propagation hook), and TD-VSDD-075 (last_amended dependent-citation + source-code-verification) are now individually self-contained and enforceable.

**Fix 3 — M-P24-004 CLOSED (section-boundary fixed: TD-VSDD-073/075/076 moved to correct H2 section):**

TD-VSDD-073, TD-VSDD-075, and TD-VSDD-076 entries were positioned after the `## Lessons codified during the cycle` H2 boundary in open-backlog-post-rc8.md. All three are Phase D-4 TD-VSDD entries and belong under `## New from Phase D-4 (2026-05-05)`. Fixed by consolidating all TD-VSDD-NNN entries (056-077) into the `## New from Phase D-4` section in monotonically ascending order.

**Fix 4 — M-P24-005 CLOSED (lessons.md `[codified]` marker orphans repaired):**

Three `[codified] by D-NNN` markers were displaced from their associated lessons: (1) D-252 marker appeared after TD-VSDD-067 lesson; moved to immediately follow TD-VSDD-066 lesson (D-252 codified TD-VSDD-066, not TD-VSDD-067); (2) D-257 marker appeared after TD-VSDD-071 lesson; moved to immediately follow TD-VSDD-070 lesson (D-257 codified TD-VSDD-070); (3) D-261 marker appeared after TD-VSDD-074 lesson; moved to immediately follow TD-VSDD-073 lesson (D-261 codified TD-VSDD-073). All duplicate orphaned markers removed.

**Fix 5 — M-P24-006 CLOSED (bidirectional drift on TD-VSDD-074 Source field resolved):**

open-backlog-post-rc8.md TD-VSDD-074 entry Source included `+ PG-P20-001`. lessons.md TD-VSDD-074 LESSON Source field was missing `+ PG-P20-001`. Fix: added `+ PG-P20-001` to lessons.md Source field for TD-VSDD-074 lesson, restoring bidirectional consistency.

**L-P24-001 CLOSED (non-monotonic ordering fixed):** All TD-VSDD-NNN entries in `## New from Phase D-4` section now in strictly ascending 056..077 order.

**L-P24-002 CLOSED (conflated content corrected):** TD-VSDD-072 body in open-backlog no longer contains TD-VSDD-071 content (stub-fill repair in Fix 2 above addresses root cause).

**L-P24-003 CLOSED (TD-VSDD-076 lesson title precision improved):** Title changed from `(TD-VSDD-076 extension to TD-VSDD-075)` to `(TD-VSDD-076; extends TD-VSDD-075 from inter-document to intra-document scope)` — clarifies that TD-VSDD-076 specifically extends the dependent-citation-propagation discipline to intra-document siblings.

**TD-VSDD-077 codified (Lessons-corpus bidirectional coherence validation hook):**

Pass-24's convention-meta audit revealed that the lessons-corpus artifacts themselves had accumulated 6 distinct coherence defects across 20 TD-VSDD codification bursts. Going forward: when adding any TD-VSDD-NNN entry, MUST verify (a) lessons.md has full body + Source + [codified] marker immediately following lesson content, (b) open-backlog has matching bullet under correct H2 with body content not stub, (c) ordering is monotonic, (d) Source citations match bidirectionally. Appended TD-VSDD-077 to both lessons.md and open-backlog-post-rc8.md.

**Post-edit verification PASS:**
- `grep -n 'truncated: bool' BC-1.05.036.md` → both line 49 and line 88 use `/* ... */` annotation. PASS.
- `grep -n '\[codified\]' lessons.md` → all markers within ~2 lines of associated lesson content. PASS.
- open-backlog TD-VSDD-069/071/075 stubs no longer empty — each has self-contained body content. PASS.

**ADR-013 clock:** 0_of_3 (reset by pass-24 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (25/26/27) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.23"` → `"1.24"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (twelfth use):** State-manager handles pass-24 seal and multi-file repair burst atomically. All fixes are textual corrections/additions in lessons-corpus artifacts where architect judgment is not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.25 (2026-05-05) — D-268 source-truth fix burst: pass-25 1H/2M/2L; BC denial-path enumeration corrected; TD-VSDD-078 codified

**State-manager-led combined burst applying TD-VSDD-064 sequential pattern (THIRTEENTH application).**

Pass-25 verdict: SUBSTANTIVE 1H/2M/2L. Source-code traceability exhaustive sweep angle (NEW per TD-VSDD-057; extends TD-VSDD-078 codified in this burst). 22 source-claims verified PASS; 1 HIGH source-fabrication caught. ADR-013 clock RESET to 0_of_3.

**Fix 1 — H-P25-001 CLOSED (BC-1.05.036:52 fabricated denial-path enumeration corrected — TD-VSDD-078 first application):**

Source-code re-verification per TD-VSDD-075 + TD-VSDD-078: read `crates/factory-dispatcher/src/host/exec_subprocess.rs` lines 147–171. Actual `emit_denial` callsites:
- Line 148: `emit_denial(ctx, cmd, "no_exec_subprocess_capability", Map::new())`
- Line 155: `emit_denial(ctx, cmd, "binary_not_on_allow_list", details)`
- Line 162: `emit_denial(ctx, cmd, "shell_bypass_not_acknowledged", details)`
- Line 169: `emit_denial(ctx, cmd, "setuid_or_setgid_binary", details)`

BC-1.05.036 §Postcondition 5 previously listed `(binary not allowed, shell bypass not acknowledged, env not allowed, cwd not allowed)` — fabricated: "env not allowed" and "cwd not allowed" have no `emit_denial` callsites in source (env_allow silently filtered, cwd_allow unenforced per gap-analysis §1); `no_exec_subprocess_capability` and `setuid_or_setgid_binary` were entirely missing.

Fix applied: §Postcondition 5 parenthetical replaced with the 4 actual source-of-truth reason strings per exec_subprocess.rs:148/155/162/169, plus sibling clarification about env_allow/cwd_allow reality.

**Post-edit verification PASS:** `grep -n 'env not allowed\|cwd not allowed' BC-1.05.036.md` → zero matches in non-changelog body. PASS.

**Fix 2 — M-P25-001 CLOSED (BC-1.05.036 §EC-003 tightened to enumerate 4 real denial reasons — TD-VSDD-076 sibling sweep applied):**

§EC-003 Description column expanded from generic "Capability check fails" to enumerate the 4 real denial reason strings per exec_subprocess.rs:148/155/162/169, with explicit note that env_allow + cwd_allow violations do NOT trigger this EC. Sibling-sweep discipline (TD-VSDD-076) applied: §Postcondition 5 correction propagated to sibling §Edge Cases table.

**Fix 3 — M-P25-002 CLOSED (BC-1.05.036:50 Instant cite corrected — line 270 is post-spawn deadline, not spawn time):**

BC-1.05.036 §Postconditions item 3 previously stated "the deadline `Instant` already present in `execute_bounded` (exec_subprocess.rs:270) is the reference." Source verification: line 270 is `let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);` — this is the POST-SPAWN deadline computation, not a spawn-time duration reference. The actual spawn point is line 252: `let mut child = command.spawn().map_err(|_| codes::INTERNAL_ERROR)?;`.

Fix applied: §Postconditions item 3 now correctly directs implementer to add `let started = Instant::now();` before `command.spawn()` at line 252; notes that line 270 is post-spawn and NOT the duration reference.

**L-P25-001 SKIPPED (gap-analysis line 216 unbalanced paren — cosmetic; markdown renders fine; no semantic impact).**

**L-P25-002 SKIPPED (perf-baseline `references:` field convention — pending intent; may be deliberate distinction from formal `inputs:`; per D-257 prior deferral rationale).**

**TD-VSDD-078 codified (BC postcondition source-of-truth enumeration verification — extension of TD-VSDD-075):**

When a BC postcondition or normative BC section cites a CONCRETE ENUMERATION (list of error codes, list of denial reasons, list of fields, list of paths) derived from source code, the fix-burst or authoring burst MUST grep the cited source file for each enumeration item and verify presence/absence. Fabricated items (present in BC but absent from source) and missing items (present in source but absent from BC) must both be corrected in the same burst. Extends TD-VSDD-075 sub-rule (source-code-verification) to cover enumerations specifically. Appended to lessons.md and open-backlog-post-rc8.md.

**ADR-013 clock:** 0_of_3 (reset by pass-25 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (26/27/28) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.24"` → `"1.25"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (thirteenth use):** State-manager handles pass-25 seal and 3-fix burst atomically. All fixes are textual corrections to BC normative sections where source-code-verification confirms ground truth; architect judgment not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.26 (2026-05-05) — D-270 silence-audit fix burst: pass-27 1H/1M/0L; BC multi-sink scrub + INTERNAL_ERROR enumeration

**HIGH findings closed:**

- **H-P27-001 CLOSED:** BC-1.05.036 Postcondition 4 (line 51) stale ADR-005-era multi-sink wording "file/datadog/honeycomb per config" replaced with ADR-015 D-15.1 single-stream FileSink description. The `Router`/`SinkRegistry` and multi-sink stanza model are retired per ADR-015 lines 130, 154. External fan-out to Datadog/Honeycomb is handled by OTel Collector OUTSIDE the dispatcher. The BC had gained an ADR-015 awareness block in its Description at v1.21 (D-263) but Postcondition 4 body text was not updated — this burst closes that gap.

**MED findings closed:**

- **M-P27-001 CLOSED:** BC-1.05.036 Postcondition 5 extended to enumerate INTERNAL_ERROR (-99) as a third no-event error path. Source-code verification per TD-VSDD-078: spawn failure (exec_subprocess.rs:252), stdin take failure (:258), stdin write failure (:262), stdout take failure (:267), stderr take failure (:268), try_wait error (:299). Constant `INTERNAL_ERROR: i32 = -99` at `crates/factory-dispatcher/src/host/mod.rs:184`. Postcondition 5 now correctly enumerates all three no-event error paths: TIMEOUT (-2), OUTPUT_TOO_LARGE (-3), INTERNAL_ERROR (-99).

**Source-of-truth verification per TD-VSDD-075 + TD-VSDD-078:**
- ADR-015 line 99: "All events… are written to one physical file: `.factory/logs/events-YYYY-MM-DD.jsonl`" — VERIFIED
- ADR-015 line 130: "The `sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types within `sink-core` are retired" — VERIFIED
- ADR-015 line 154: "The multi-sink stanza model is removed. Operators who need remote export configure the OTel Collector as the second hop, not the dispatcher." — VERIFIED
- exec_subprocess.rs:252: `command.spawn().map_err(|_| codes::INTERNAL_ERROR)?` — VERIFIED
- exec_subprocess.rs:258: `child.stdin.take().ok_or(codes::INTERNAL_ERROR)?` — VERIFIED
- exec_subprocess.rs:262: `return Err(codes::INTERNAL_ERROR)` (stdin write_all failure) — VERIFIED
- exec_subprocess.rs:267: `child.stdout.take().ok_or(codes::INTERNAL_ERROR)?` — VERIFIED
- exec_subprocess.rs:268: `child.stderr.take().ok_or(codes::INTERNAL_ERROR)?` — VERIFIED
- exec_subprocess.rs:299: `Err(_) => return Err(codes::INTERNAL_ERROR)` (try_wait error) — VERIFIED
- host/mod.rs:184: `pub const INTERNAL_ERROR: i32 = -99;` — VERIFIED

**Post-edit grep verification:**
- `grep -n 'datadog/honeycomb per config' BC-1.05.036.md` → zero matches in non-changelog body. PASS.
- `grep -n 'INTERNAL_ERROR\|-99' BC-1.05.036.md` → match at Postcondition 5. PASS.

**ADR-013 clock:** 0_of_3 (reset by pass-27 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (28/29/30) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.25"` → `"1.26"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (fourteenth use):** State-manager handles pass-27 seal and 2-fix burst atomically. Both fixes are textual corrections to BC normative sections where source-code-verification confirms ground truth; architect judgment not required.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.27 (2026-05-05) — D-271 comprehensive sibling-sweep fix burst: pass-28 2H/3M/1L; BC sink-chain residue scrubbed + INTERNAL_ERROR rows added + verb precision

**HIGH findings closed:**

- **H-P28-001 CLOSED:** BC-1.05.036 line 38 §Description "through the normal sink chain" replaced with ADR-015-correct: "through `ctx.emit_internal` to the single-stream `FileSink` per ADR-015 D-15.1 (multi-sink stanza model removed; Router/SinkRegistry retired)". The v1.26 burst scrubbed Postcondition 4 (line 51) only; §Description carried parallel stale wording that contradicted the ADR-015 awareness block at lines 32-36. Third TD-VSDD-076 self-violation instance.

- **H-P28-002 CLOSED:** BC-1.05.036 line 135 §Purity Classification I/O operations cell had TWO defects: (1) "sink chain" stale per ADR-015; (2) "non-blocking try_send" FABRICATED — `crates/factory-dispatcher/src/host/mod.rs:109-116` shows synchronous `Mutex::lock` + `Vec::push`, no channel send. Cell replaced with: "YES — emits event via `ctx.emit_internal`: synchronous `Mutex::lock` + `Vec::push` to events queue per `host/mod.rs:105-116`, then host writes to single-stream `FileSink` per ADR-015 D-15.1 (no channel send; not async)".

**MED findings closed:**

- **M-P28-001 CLOSED:** §Edge Cases EC-007 row added for INTERNAL_ERROR (-99) no-event path. Postcondition 5 (v1.26) enumerated INTERNAL_ERROR but §Edge Cases lacked a corresponding row (unlike EC-004/EC-005 for TIMEOUT/OUTPUT_TOO_LARGE). EC-007 added with full Postcondition 5 authority citation and exec_subprocess.rs source line references.

- **M-P28-002 CLOSED:** §Canonical Test Vectors INTERNAL_ERROR row added after OUTPUT_TOO_LARGE row. Parallel gap to M-P28-001 — test matrix now covers all three no-event error paths (TIMEOUT, OUTPUT_TOO_LARGE, INTERNAL_ERROR).

- **M-P28-003 CLOSED:** EC-005 and Test Vector OUTPUT_TOO_LARGE row aligned to EC-004 sibling form with explicit "NO event emitted in v1 (per Postcondition 5; future error-path emit is out-of-scope)" qualifier. Both previously used truncated forms missing the authoritative Postcondition 5 qualifier.

**LOW findings closed:**

- **L-P28-001 CLOSED:** Postcondition 4 (line 51) corrected to preserve ADR-015's dual-verb lifecycle taxonomy: "(multi-sink stanza model removed per ADR-015 line 154; Router/SinkRegistry retired per ADR-015 line 130)". ADR-015 line 130 = "retired" (crates physically deleted at Wave 5); line 154 = "removed" (configuration model eliminated). Prior wording used "retired" for both.

**Source-of-truth verification per TD-VSDD-075 + TD-VSDD-078:**
- `crates/factory-dispatcher/src/host/mod.rs:109-116` emit_internal: `Mutex::lock` + `Vec::push` (NOT try_send; NOT async) — VERIFIED
- ADR-015 line 130: "`sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types… are retired" — verb: "retired" — VERIFIED
- ADR-015 line 154: "The multi-sink stanza model is removed." — verb: "removed" — VERIFIED
- exec_subprocess.rs:252/258/262/267-268/299 INTERNAL_ERROR sites (verified D-270; unchanged) — VERIFIED
- host/mod.rs:184 `pub const INTERNAL_ERROR: i32 = -99` (verified D-270; unchanged) — VERIFIED

**Post-edit grep verification:**
- `grep -n 'sink chain\|try_send' BC-1.05.036.md` → zero matches in non-changelog body. PASS.
- `grep -n 'INTERNAL_ERROR' BC-1.05.036.md` → matches at §Postcondition 5 (line 52), §EC-007, §Canonical Test Vectors. PASS.

**TD-VSDD-079 codified:** Extends TD-VSDD-076 with explicit terminology-family grep checklist for amendment-class sibling-sweep fixes. Before commit, grep ALL retired-terminology variants across the BC ("sink chain", "Router", "SinkRegistry", "multi-sink", "fan-out", "datadog", "honeycomb", "try_send") — not just the literal phrase the adversary cited. Sweep ALL normative sections. S-7.02 recurrence threshold (3+) met by pass-28 findings.

**ADR-013 clock:** 0_of_3 (reset by pass-28 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (29/30/31) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.26"` → `"1.27"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (fifteenth use):** State-manager handles pass-28 seal and 6-fix burst atomically. All fixes are textual corrections to BC normative sections; source-code-verification confirms ground truth for all fabrication-class findings.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.28 (2026-05-05) — D-272 cross-doc terminology drift fix: pass-29 2H/0M/0L; BC vendor-name + fan-out scrub + NUL-byte attribution; TD-VSDD-080 codified

**HIGH findings closed:**

- **H-P29-001 CLOSED:** BC-1.05.036 Postcondition 4 (line 51) "external fan-out to Datadog/Honeycomb is handled by OTel Collector OUTSIDE the dispatcher" replaced with "external export to remote observability backends is handled by OTel Collector outside the dispatcher". Scrubs both the vendor names (`Datadog`, `Honeycomb`) AND the prohibited family term `fan-out` per TD-VSDD-079 8-term checklist. The v1.27 burst (D-271) codified TD-VSDD-079 with an explicit 8-term grep checklist but ran only `grep -n 'sink chain\|try_send'` (2 terms) as post-edit verification — missing `fan-out`, `Datadog`, `Honeycomb` at line 51. Fifth consecutive TD-VSDD-076/079 self-violation instance.

- **H-P29-002 CLOSED:** BC-1.05.035 §Description (line 35) "Canonicalization resolves symlinks, eliminates `..` segments, and rejects NUL bytes" corrected to "Canonicalization resolves symlinks and eliminates `..` segments. NUL-byte rejection is performed earlier by the existing `read_wasm_string` error path (see §Postcondition 2 and the Precedence Ladder)." Resolves intra-document semantic contradiction: §Description attributed NUL rejection to canonicalize while §Postcondition 2, §Precedence Ladder, and §EC-005 all attributed it to `read_wasm_string` error path. `Path::canonicalize()` does not reject NUL bytes on Unix — it would return `io::Error` mapped to CAPABILITY_DENIED (-1), not INVALID_ARGUMENT (-4) per the normative postconditions.

**TD-VSDD-079 8-term family-grep verification (full, case-insensitive, across all 5 in-scope files):**
- BC-1.05.035: ZERO non-changelog matches — PASS
- BC-1.05.036: `Router/SinkRegistry` at lines 38, 51 are ADR-015 retirement-status citations (intentional; added by D-271 as correct fix per L-P28-001); `fan-out` + `Datadog` + `Honeycomb` scrubbed by this burst — PASS
- gap-analysis-w16-subprocess.md: ZERO matches — PASS
- audit-w16.md: ZERO matches — PASS
- perf-baseline-w16.md: ZERO matches — PASS

**TD-VSDD-080 codified:** Mechanize TD-VSDD-079 as pre-commit hook. Five consecutive narrative-discipline failures (passes 24/25/28/29 + at-least-one pre-pass-24) demonstrate that narrative-discipline alone cannot enforce terminology-family grep discipline. Implementation: `validate-bc-terminology-family.sh` pre-commit hook that runs the TD-VSDD-079 8-term grep automatically against any modified BC or arch-doc file and FAILS the commit if any term matches outside `### Changelog` or `### v1.X` H3 sections. Mechanical enforcement required.

**ADR-013 clock:** 0_of_3 (reset by pass-29 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (30/31/32) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.27"` → `"1.28"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (sixteenth use):** State-manager handles pass-29 seal and 2-fix burst atomically. All fixes are textual corrections to BC normative sections.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.29 (2026-05-05) — D-274 inverse-traceability fix: pass-31 0H/2M/3L; trace-id tense correction + outcome enum + line cite + paraphrase; TD-VSDD-057 inverse-traceability angle; ADR-013 clock RESET 0_of_3

**MED findings closed:**

- **MED-P31-001 CLOSED (Fix 1+2):** gap-analysis-w16-subprocess.md:334-337 + audit-w16.md B-7 row both claimed `VSDD_TRACE_ID`/`VSDD_PARENT_SPAN_ID` injection as present-tense "automatic (dispatcher-side invariant)". ADR-015 D-15.4:407-419 frames this as normative MUST obligation (future-state). Source-code verification per TD-VSDD-075: exec_subprocess.rs:242-247 does `env_clear()` + selective `env_allow` forward ONLY — no trace-id injection present. Corrected both sites to "MUST be injected per ADR-015 D-15.4 (normative future-state; current `execute_bounded` does NOT inject these vars; implementation pending E-10 Wave 1)".

- **MED-P31-002 CLOSED (Fix 3):** BC-1.05.036 Postcondition 2 listed 8 domain payload fields but omitted `outcome` enum field mandated by ADR-015 D-15.2:270 canonical taxonomy (`success | failure | error | timeout | skipped | blocked`). Added sentence to Postcondition 2: (a) `outcome` is host-stamped per D-15.3 (not part of 8-field domain payload); (b) exit_code→outcome mapping (`exit_code == 0 → 'success'`; `exit_code != 0 → 'failure'`); (c) no-event paths (TIMEOUT/OUTPUT_TOO_LARGE/INTERNAL_ERROR) out of scope.

**LOW findings closed:**

- **LOW-P31-003 CLOSED (Fix 4):** BC-1.05.036 Postcondition 5 + EC-007 cited `:262` for stdin write-failure path. exec_subprocess.rs:259 is the actual `child_stdin.write_all(stdin_bytes).is_err()` check. Both Postcondition 5 and EC-007 updated `:262` → `:259`.

- **LOW-P31-004 CLOSED (Fix 5):** perf-baseline-w16.md:353 paraphrase "sub-millisecond I/O" replaced with "(measured throughput >10k events/minute per ADR-015 D-15.1 Rationale; negligible at vsdd-factory volumes)".

- **LOW-P31-005 SKIPPED (per S-7.03 SHIP-AS-IS):** BC-1.05.036 Postcondition 4 "Same code path as emit_denial" tense conflation cosmetic; structurally accurate. Deferred as not materially misleading.

**Source-of-truth verification per TD-VSDD-075:**
- exec_subprocess.rs:242-247 confirmed: `command.env_clear()` + selective `env_allow` forward ONLY; NO trace-id injection.
- exec_subprocess.rs:259 confirmed: `child_stdin.write_all(stdin_bytes).is_err()` at line 259.
- ADR-015 D-15.2:270 confirmed: `outcome` enum `success | failure | error | timeout | skipped | blocked`.
- ADR-015 D-15.4:407-419 confirmed: "MUST be injected" (normative language).
- ADR-015 D-15.1 Rationale:432-440 confirmed: "10k events/minute without measurable overhead".

**Post-edit grep verification:**
- `grep -n 'are injected\|automatic.*invariant\|dispatcher-side invariant' gap-analysis-w16-subprocess.md` → ZERO matches. PASS.
- `grep -n 'automatic.*invariant\|dispatcher-side invariant' audit-w16.md` → ZERO matches. PASS.
- `grep -n 'outcome' BC-1.05.036.md` → match at Postcondition 2. PASS.
- `grep -n ':262\|:259' BC-1.05.036.md` → `:259` at stdin context in Postcondition 5 + EC-007; no `:262`. PASS.
- `grep -n 'sub-millisecond' perf-baseline-w16.md` → ZERO matches. PASS.

**TD-VSDD-079 8-term family-grep (across all 5 in-scope files):** Non-changelog body matches at BC-1.05.036 lines 38+51 are intentional ADR-015 retirement-status citations (Router/SinkRegistry retired per lines 130/154; correct from D-271). PASS.

**ADR-013 clock:** 0_of_3 (RESET by pass-31 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (32/33/34) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.28"` → `"1.29"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (seventeenth use):** State-manager handles pass-31 seal and 4-fix burst atomically. All fixes are textual corrections to arch docs and BC normative sections.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.30 (2026-05-05) — D-276 PC↔TV coherence fix: pass-33 0H/3M/1L; outcome-enum test coverage + symlink event witness + Postcondition 1 disambiguation + anchor correction; TD-VSDD-057 PC↔TV coherence angle (NEW — inverse direction); ADR-013 clock RESET 0_of_3

**MED findings closed:**

- **MED-P33-001 CLOSED (Fix 1):** BC-1.05.036 Postcondition 2 (added v1.29) mandates `outcome` enum stamping (`exit_code == 0 → 'success'`; `exit_code != 0 → 'failure'`) per ADR-015 D-15.3, but neither §Edge Cases nor §Canonical Test Vectors had any witness row. Added EC-008 ("Outcome enum stamping") to §Edge Cases. Added two §Canonical Test Vector rows ("Outcome enum (success): exit_code=0 invocation" → `outcome='success'`; "Outcome enum (failure): exit_code=1 invocation" → `outcome='failure'`) with `host-stamping` category. Postcondition 2 outcome-enum mandate now has verifiable test coverage.

- **MED-P33-002 CLOSED (Fix 2):** BC-1.05.035 Postcondition 4 states symlink-traversal escape returns INVALID_ARGUMENT (-4) AND emits `internal.capability_denied`. EC-002 and Test Vector row 3 previously verified the error code only; neither mentioned event emission. Added §Description **Pairing rationale** paragraph explaining the novel INVALID_ARGUMENT+capability_denied combination (differs from existing 4 denial paths at exec_subprocess.rs:148/155/162/169 which all return CAPABILITY_DENIED -1; rationale: code reflects malformed path shape post-canonicalize; event channel reused for dashboard aggregation). Appended event-emission assertion to EC-002 outcome cell and to Test Vector row 3 outcome cell, both citing reason `"symlink_traversal_escape"`.

- **MED-P33-003 CLOSED (Fix 3):** BC-1.05.035 Postcondition 1 contained parenthetical "(`../` absent, no NUL bytes)" implying a pre-canonicalize string-level `../` reject guard exists as a distinct step. No such guard exists — the Precedence Ladder has no `../` string-level step; EC-001 CAPABILITY_DENIED is explained by allow-list miss (basename "passwd" not in `binary_allow` → emit_denial("binary_not_on_allow_list") at exec_subprocess.rs:155 → CAPABILITY_DENIED). Removed the misleading parenthetical; replaced with reference to `read_wasm_string` error path (NUL bytes only, per Postcondition 2). Added clarifying note to EC-001 outcome cell explaining the CAPABILITY_DENIED path explicitly and stating pre-canonicalize string-level `../` reject is NOT a separate guard.

**LOW findings closed:**

- **LOW-P33-001 CLOSED (Fix 4):** BC-1.05.035 §Description line 33 cited `gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" lines 339-349`. Source-of-truth re-verification confirms: H3 `### How ADR-015 affects the telemetry gap` ends at line 339; H3 `### Existing denial-path telemetry` begins at line 341; rename rationale (`internal.capability_denied` → `vsdd.capability.denied.exec_subprocess.v1`) is at lines 343-351. Corrected citation to `§"Existing denial-path telemetry" lines 341-351`.

**Source-of-truth verification per TD-VSDD-075/078:**
- gap-analysis-w16-subprocess.md: H3 `### Existing denial-path telemetry` begins line 341 (after §"How ADR-015 affects the telemetry gap" ends line 339). Rename rationale at lines 343-351. CONFIRMED.
- exec_subprocess.rs:148/155/162/169 emit_denial 4 reasons: all use CAPABILITY_DENIED (-1). CONFIRMED unchanged.
- BC-1.05.035 Postcondition 4 INVALID_ARGUMENT+capability_denied pairing documented with rationale. CONFIRMED.
- BC-1.05.036 Postcondition 2 outcome-enum mandate now witnessed by EC-008 + 2 Test Vector rows. CONFIRMED.

**Post-edit grep verification:**
- `grep -n 'How ADR-015 affects.*339' BC-1.05.035.md` → ZERO matches. PASS.
- New citation `§"Existing denial-path telemetry"` present in BC-1.05.035 §Description. PASS.
- `grep -n "string-level validation" BC-1.05.035.md` → ZERO matches. PASS.
- EC-002 event-emission witness present in BC-1.05.035. PASS.
- EC-008 outcome-enum row present in BC-1.05.036. PASS.
- Two `host-stamping` Canonical Test Vector rows present in BC-1.05.036. PASS.

**TD-VSDD-079 8-term family-grep (across all in-scope BC files):**
- BC-1.05.035: ZERO prohibited matches. PASS.
- BC-1.05.036 lines 38+51: intentional ADR-015 retirement-status citations (Router/SinkRegistry retired per ADR-015 lines 130/154; correct from D-271). PASS.

**ADR-013 clock:** 0_of_3 (RESET by pass-33 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (34/35/36) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.29"` → `"1.30"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (eighteenth use):** State-manager handles pass-33 seal and 4-fix burst atomically. All fixes are textual corrections to BC normative sections.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

### v1.31 (2026-05-05) — D-277 mechanism-fix: pass-34 1H/3M/2L; NUL byte rejection mechanism corrected + INTERIM declaration + sibling-disclosure; TD-VSDD-081 codified; ADR-013 clock RESET 0_of_3

**HIGH findings closed:**

- **HIGH-P34-001 CLOSED (Fix 1):** BC-1.05.035 contained a factual error: Postcondition 2, Postcondition 1 preamble, EC-005, and Precedence Ladder step (1) all claimed NUL bytes are rejected via `read_wasm_string` error path, returning `INVALID_ARGUMENT` (-4). Source-of-truth verification of `host/memory.rs:47-54` confirms `read_wasm_string` only fails on `String::from_utf8` errors; NUL bytes (0x00) are valid UTF-8 (U+0000) and pass through cleanly. Actual NUL handling: `Path::new(cmd).canonicalize()` on Unix returns EINVAL for NUL-containing paths via std::path CString conversion → Precedence Ladder step (2) → `CAPABILITY_DENIED` (-1). Corrections applied: Postcondition 1 preamble — dropped `read_wasm_string` error path claim; Postcondition 2 — rewritten to "non-UTF-8 byte sequence" with NOTE clarifying NUL byte correct path; EC-005 — corrected from `INVALID_ARGUMENT` (-4) to `CAPABILITY_DENIED` (-1) with explanation; Precedence Ladder step (1) — "NUL byte" → "Non-UTF-8 byte sequence" with NOTE that NUL bytes pass to step (2).

**MED findings closed:**

- **MED-P34-001 CLOSED (Fix 2):** BC-1.05.035 EC-001 outcome cell assumed `binary_allow` shape (basename "passwd" not in `binary_allow`) but the EC precondition did not bind it. Added `binary_allow = ["bash"]` (typical S-9.07 capability shape per OQ-3) and `cmd` basename "passwd" not in `binary_allow` to EC-001 precondition column.

- **MED-P34-002 CLOSED (Fix 3):** BC-1.05.036 §Related BCs row for BC-1.05.035 did not disclose that BC-1.05.035 introduces a 5th denial-event path (`INVALID_ARGUMENT (-4) + internal.capability_denied`) differing from the existing 4 CAPABILITY_DENIED paths. Appended NOTE to the BC-1.05.035 row in §Related BCs: "BC-1.05.035 introduces a novel `INVALID_ARGUMENT (-4) + internal.capability_denied` pairing for symlink-traversal escape — a 5th denial-event path with a different error code than the existing 4 CAPABILITY_DENIED paths. Test-writers building denial-event taxonomy MUST include this 5th path."

- **MED-P34-003 CLOSED (Fix 4):** gap-analysis §"Existing denial-path telemetry" (lines 341-351) used "MUST be renamed" but did not declare "INTERIM" as a lifecycle marker, leaving the INTERIM tag in BCs without a source-of-truth anchor. Appended declaration to gap-analysis after the rename clause: "Until the rename ships (in E-10 Wave 1 host-emit-fix story), the existing `internal.capability_denied` event name is **INTERIM** — BC-1.05.035 + BC-1.05.036 §Description ADR-015 awareness clauses use this INTERIM tag to lifecycle-mark the name."

**LOW findings:**

- **LOW-P34-001 SKIPPED per S-7.03 SHIP-AS-IS:** outcome enum mapping rule duplicated in 3 locations (§Postcondition 2, EC-008, §Canonical Test Vectors). Cosmetic refactoring; does not introduce bugs.
- **LOW-P34-002 CLOSED implicitly:** Grammar awkwardness in Postcondition 1 preamble de facto resolved by Fix 1 rewrite.

**Source-of-truth mechanism verification per TD-VSDD-081:**
- `host/memory.rs:47-54` `read_wasm_string`: only rejects non-UTF-8 (`String::from_utf8` error); NUL bytes (0x00) are valid UTF-8 and pass through. CONFIRMED.
- `exec_subprocess.rs:230` `Command::new(cmd)`: no string-level NUL guard pre-canonicalize. CONFIRMED.
- `Path::new(cmd).canonicalize()` on Unix: returns Err(EINVAL) for NUL-containing paths via std::path CString conversion. CONFIRMED via std lib docs.
- Ladder step (2) canonicalize() Err → `CAPABILITY_DENIED` (-1) per existing 4 emit_denial paths at exec_subprocess.rs:148/155/162/169. CONFIRMED.

**Post-edit grep verification:**
- `grep -n "NUL byte.*INVALID_ARGUMENT\|read_wasm_string.*NUL\|NUL bytes rejected" BC-1.05.035.md` → no residue of false claim. PASS.
- EC-005 corrected to `CAPABILITY_DENIED` (-1). PASS.
- Precedence Ladder step (1) updated to "Non-UTF-8 byte sequence". PASS.
- BC-1.05.036 §Related BCs sibling-disclosure NOTE appended. PASS.
- gap-analysis INTERIM declaration appended. PASS.

**TD-VSDD-079 8-term family-grep:**
- BC-1.05.035: ZERO prohibited matches. PASS.
- BC-1.05.036 lines 38+51: intentional ADR-015 retirement-status citations (Router/SinkRegistry retired). PASS.
- gap-analysis-w16-subprocess.md: ZERO prohibited matches. PASS.

**TD-VSDD-081 codified:** Mechanism-verification beyond string-presence-grep. See lessons.md.

**ADR-013 clock:** 0_of_3 (RESET by pass-34 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (35/36/37) needed for CONVERGENCE_REACHED.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.30"` → `"1.31"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (nineteenth use):** State-manager handles pass-34 seal and 4-fix burst atomically. All fixes are textual corrections to BC normative sections and gap-analysis.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

---

### v1.32 (2026-05-05) — D-278 sibling-mechanism-sweep seal-and-fix: pass-35 1H/3M/2L; symlink prefix-check mechanism corrected + behavior-change disclosure; TD-VSDD-082 codified

**Source-code mechanism verification per TD-VSDD-081 (MANDATORY — completed before commit):**
- `Path::canonicalize()` std semantics: resolves ALL `..` segments away; returns absolute path with NO `..` components (Rust std::path canonicalize docs — resolves symlinks and eliminates `..` segments). Actual symlink-escape detection requires `canonical_path.starts_with(project_root)` prefix check, NOT `..` scan.
- `exec_subprocess.rs:252`: `command.spawn().map_err(|_| codes::INTERNAL_ERROR)` confirms current missing-binary path returns `-99` (`INTERNAL_ERROR`), not `-1` (`CAPABILITY_DENIED`). Adding `canonicalize()` pre-spawn changes this to `-1` for missing-binary case — BEHAVIOR CHANGE acknowledged in Postcondition 3.

**HIGH-P35-001 CLOSED — BC-1.05.035 EC-002 + Postcondition 4 + Ladder step (3): `..` scan mechanism corrected to prefix check**

v1.31 correctly fixed the NUL-byte mechanism (read_wasm_string vs canonicalize path). The SAME BC contained a sibling mechanism (symlink-escape detection) using the same `Path::canonicalize()` predicate-shape but a DIFFERENT specific predicate — the `..` scan claim — which was also wrong. The actual escape detection is `canonical_path.starts_with(project_root)` prefix check; `Path::canonicalize()` resolves all `..` segments away by design. Three sites corrected:
- **EC-002:** replaced "canonicalize() resolves it; `..` components detected" with "canonical path does NOT start with trusted project-root prefix → prefix check fires" + NOTE clarifying canonicalize() resolves `..` segments.
- **Postcondition 4:** corrected from "`..` components after resolution" to "canonical path does NOT start with trusted project-root prefix" + NOTE explaining canonicalize()/`..`-resolution semantics.
- **Precedence Ladder step (3):** corrected from "canonicalized path contains `..` segments" to "canonical path does NOT start with trusted project-root prefix (symlink-traversal escape)".

**Postcondition 1 also updated** from "does not contain `..` segments" to "starts with the trusted project-root prefix" for internal coherence.

**MED-P35-001 CLOSED — BC-1.05.035 Postcondition 3: BEHAVIOR CHANGE explicitly disclosed**

The claim "existing exec_subprocess error semantics preserved" was factually wrong. Current missing-binary `cmd` returns `INTERNAL_ERROR (-99)` at `command.spawn()` (exec_subprocess.rs:252). Adding `canonicalize()` pre-spawn changes this to `CAPABILITY_DENIED (-1)` for the missing-binary case. Postcondition 3 now contains full BEHAVIOR CHANGE disclosure: -99 → -1 transition, test migration note (tests expecting INTERNAL_ERROR for missing-binary will break), and intentional-change rationale (aligning with 4 existing CAPABILITY_DENIED denial paths).

**MED-P35-002 CLOSED — BC-1.05.035 §Related BCs BC-1.05.036 row: reverse-direction sibling-disclosure NOTE**

v1.31 burst (D-277 MED-P34-002) added a forward-direction NOTE to BC-1.05.036 §Related BCs. The reverse direction was missing. BC-1.05.035 §Related BCs row for BC-1.05.036 now discloses that BC-1.05.036 introduces the FIRST non-denial event via `ctx.emit_internal` (`host.exec_subprocess.completed`) — a structurally novel event class. Test-writers building event-taxonomy coverage MUST include success-path event class. Bidirectional sibling-disclosure symmetry restored.

**MED-P35-003 CLOSED — BC-1.05.036 Postcondition 4: ADR-015 line-number citations replaced with quoted-phrase anchors**

"multi-sink stanza model removed per ADR-015 line 154; Router/SinkRegistry retired per ADR-015 line 130" replaced with stable quoted-phrase anchors: "multi-sink stanza model removed per ADR-015 D-15.1 §'Decision' 'the multi-sink stanza model is removed'; Router/SinkRegistry retired per ADR-015 D-15.1 §'Decision' 'the `sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types within `sink-core` are retired'". Line-number anchors are fragile when ADR-015 amends; quoted-phrase anchors are stable.

**LOW-P35-001 SKIPPED per S-7.03:** Step (3) grammar inconsistency — cosmetic.

**LOW-P35-002 SKIPPED per S-7.03:** EC-007 INTERNAL_ERROR source enumeration at BC-1.05.035 — detail belongs in BC-1.05.036 Postcondition 5; cross-BC granularity correct.

**TD-VSDD-082 codified:** Sibling-mechanism sweep + bidirectional-sibling-disclosure. See lessons.md. Extends TD-VSDD-076 + TD-VSDD-081. When a fix-burst corrects a mechanism, MUST sweep ALL mechanisms within the SAME BC that invoke the same std-lib function. When adding a sibling-disclosure NOTE to BC-A §Related BCs → BC-B, the inverse BC-B § Related BCs → BC-A MUST receive a symmetric disclosure if applicable.

**TD-VSDD-079 8-term family grep (MANDATORY per TD-VSDD-079/080):** `sink chain`, `Router`, `SinkRegistry`, `DlqWriter`, `multi-sink`, `fan-out`, `Datadog`, `Honeycomb`, `try_send` — ZERO non-changelog body matches in BC-1.05.035 and BC-1.05.036. PASS. BC-1.05.036 line 51's ADR-015 D-15.1 §"Decision" quoted-phrase anchors are intentional architecture citations, not prohibited terms.

**ADR-013 clock:** 0_of_3 (RESET by pass-35 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (36/37/38) needed for CONVERGENCE_REACHED.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.31"` → `"1.32"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (twentieth use):** State-manager handles pass-35 seal and 4-fix burst atomically. All fixes are textual corrections to BC normative sections.

**No new BCs, VPs, or FRs added (scope discipline maintained).**

---

### v1.33 (2026-05-05) — D-279 architectural-reframe seal-and-fix: pass-36 2H/3M/1L; drop prefix-check + symlink_traversal_escape; TOCTOU framing; TD-VSDD-083 codified; ADR-013 clock RESET 0_of_3

**Source-code mechanism verification per TD-VSDD-081/082/083 (MANDATORY — completed before commit):**
- `host/mod.rs:49-76` `HostContext` struct: has `cwd: PathBuf`, NO `project_root` field. Confirmed: "trusted project-root prefix" concept has NO data-structure anchor.
- `exec_subprocess.rs:148-192`: 4 `emit_denial` paths at lines 148/155/162/169 — all use `CAPABILITY_DENIED` (-1). Confirmed: no prior INVALID_ARGUMENT denial path.
- `exec_subprocess.rs:152`: `binary_allowed()` call site. Confirmed: canonicalize insertion point is BEFORE this line, not at line 230.
- `exec_subprocess.rs:155`: `emit_denial("binary_not_on_allow_list")` path. Confirmed: existing allow-list miss path.
- `read_file.rs:122-148`: `path_allow` uses LOOP (canonicalize each allow-list entry, check `starts_with`) not single-prefix check. Confirmed: no sibling-implementation anchor for project-root prefix concept.
- `gap-analysis-w16-subprocess.md` Section 5: proposes `if cmd.contains("../") { return CAPABILITY_DENIED; }` string-level `../` guard, NOT canonicalize-and-prefix-check mechanism. Confirmed: gap-analysis has no authority for prefix-check mechanism.

**HIGH findings closed:**

- **HIGH-P36-001 CLOSED (Fix 1 — architectural reframe):** v1.32 Postcondition 4 prefix-check mechanism was ANTI-CORRECT. `Path::canonicalize("/usr/bin/bash")` → `/usr/bin/bash`; `/usr/bin/bash` does NOT start with `$CLAUDE_PROJECT_DIR`. Postcondition 4 would therefore trigger `symlink_traversal_escape` → `INVALID_ARGUMENT` (-4) for the canonical S-9.07 happy-path case. Simultaneously, EC-003 + Test Vector for `/usr/bin/bash` says "proceeds to allow-list check". Direct internal contradiction. Fix: Dropped Postcondition 4 (prefix-check) entirely. Replaced with: "No new error path introduced for symlink-resolved targets — symlink resolving to path NOT in `binary_allow` → existing allow-list miss path → emit_denial at exec_subprocess.rs:155 → CAPABILITY_DENIED (-1)."

- **HIGH-P36-002 CLOSED (Fix 1 + Fix 2 + Fix 3):** "trusted project-root prefix" concept had no architectural anchor in any upstream document. HostContext (host/mod.rs:49-76) has `cwd: PathBuf` but no `project_root` field. gap-analysis Section 5 proposes string-level `../` guard, NOT prefix-check. No HOST_ABI.md, ADR, or other document defines the concept. read_file.rs uses allow-list LOOP, not single-prefix check. Fix: Dropped all non-changelog references to "trusted project-root prefix", "project-root prefix", and "symlink_traversal_escape" in BC-1.05.035.md normative sections. Reframed BC-1.05.035 around TOCTOU prevention (canonicalize → feed canonical path to existing `binary_allowed()` → symlink-miss fires as normal allow-list miss).

**MED findings closed:**

- **MED-P36-001 CLOSED (Fix 1):** Precedence Ladder step (3) fired `INVALID_ARGUMENT` (-4) for symlink case — inconsistent with all 4 existing denial paths (all CAPABILITY_DENIED -1). Dropped step (3) prefix-check; renumbered: old step (4) allow-list miss becomes new step (3). All 3 ladder steps now fire CAPABILITY_DENIED.

- **MED-P36-002 CLOSED (Fix 4):** §Architecture Anchors cited `exec_subprocess.rs:230` as "canonicalize-before-check step added here". Line 230 is inside `execute_bounded()` which runs AFTER all capability checks. Corrected to `exec_subprocess.rs:152` (`binary_allowed()` call site — actual insertion point).

- **MED-P36-003 CLOSED (Fix 5):** EC-001 outcome cell's ladder step attribution was wrong under v1.32's model. Corrected to: step (2) canonicalize succeeds → step (3) allow-list miss fires → CAPABILITY_DENIED (-1).

**LOW findings:**

- **LOW-P36-001 CLOSED (implicitly by Fix 1):** ADR-015 Awareness clause referenced `"symlink_traversal_escape"` denial reason — stale after reframe. The clause now correctly reflects that BC-1.05.035's denial path is via existing `emit_denial("binary_not_on_allow_list")` (no novel reason). Awareness clause retained for event-rename tracking.

**BC-1.05.036 §Related BCs sibling NOTE update:**

- BC-1.05.035 §Related BCs row for BC-1.05.036 updated: removed reference to "5th symlink-pairing" (novel pairing dropped); added TOCTOU framing note.
- BC-1.05.036 §Related BCs row for BC-1.05.035 updated: "novel INVALID_ARGUMENT+capability_denied pairing" note replaced with "TOCTOU prevention + CAPABILITY_DENIED via existing allow-list miss; no novel pairing".

**TD-VSDD-083 codified:** Architectural-concept-anchoring rule — normative postconditions cannot rely on coined concepts without upstream definition. See lessons.md.

**Post-edit grep verification:**
- `grep -niE 'trusted project-root prefix|project-root prefix|symlink_traversal_escape' BC-1.05.035.md BC-1.05.036.md gap-analysis-w16-subprocess.md` → ZERO non-changelog body matches. PASS.
- TD-VSDD-079 8-term family grep (sink chain, Router, SinkRegistry, DlqWriter, multi-sink, fan-out, Datadog, Honeycomb, try_send): BC-1.05.035 ZERO prohibited matches. BC-1.05.036 lines 38+51 are intentional ADR-015 D-15.1 retirement-status citations. PASS.

**ADR-013 clock:** 0_of_3 (RESET by pass-36 SUBSTANTIVE verdict). Three consecutive NITPICK_ONLY passes (37/38/39) needed for CONVERGENCE_REACHED.

**TD-VSDD-059 frontmatter coherence:** frontmatter `version: "1.32"` → `"1.33"` (matches latest non-reserved row). PASS.

**TD-VSDD-064 sequential-burst protocol applied (twenty-first use):** State-manager handles pass-36 seal and architectural-reframe fix burst atomically. All fixes are textual corrections to BC normative sections.

**No new BCs, VPs, or FRs added (scope discipline maintained).**
