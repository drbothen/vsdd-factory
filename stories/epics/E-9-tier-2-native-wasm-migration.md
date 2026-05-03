---
document_type: epic
epic_id: "E-9"
version: "1.0"
title: "Tier 2 Native WASM Migration (W-16) — 23 validate-*.sh hooks"
status: draft
tech_debt_ref: TD-014
prd_capabilities: [CAP-002, CAP-008, CAP-013, CAP-022]
prd_frs: []
anchor_strategy: rewrite-clean-per-ADR-014-D-9.1
priority: P2
target_release: "v1.2 (Tier 2)"
story_count: 9
producer: story-writer
timestamp: 2026-05-03T00:00:00Z
phase: 2
traces_to: .factory/tech-debt-register.md#TD-014
depends_on: ["E-8"]
inputs:
  - .factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md
  - .factory/architecture/audit-w16.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.013-host-run-subprocess.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
input-hash: "[pending-recompute]"
---
<!-- [process-gap] Frontmatter fields tech_debt_ref, anchor_strategy, depends_on extend the canonical epic-template baseline (same as E-8 v1.9). Template update tracked as follow-up. -->

# Epic E-9: Tier 2 Native WASM Migration (W-16) — 23 validate-*.sh hooks

## Description

Port all 23 `validate-*.sh` hooks currently routed through `legacy-bash-adapter.wasm`
(hooks-registry.toml lines 145–797) to native WASM crates using the rewrite-clean
strategy (ADR-014 D-9.1). One new host ABI extension (`host::run_subprocess`,
BC-2.02.013) enables capability-gated subprocess invocation required by
`validate-wave-gate-prerequisite`. Delivered in 7 capability-cluster batches
(S-9.01..S-9.07) plus one SDK extension story (S-9.30) and one perf baseline story
(S-9.00). `HOST_ABI_VERSION` stays at 1 throughout. Closes the Tier 2 phase of TD-014.

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

**SS-02 (Hook SDK and Plugin ABI) anchor:** SS-02 owns `crates/hook-sdk/` and the
dispatcher's `host/` binding surface. S-9.00 (baseline) and S-9.30 (SDK extension)
are purely SS-02 stories. S-9.30 implements `host::run_subprocess` per BC-2.02.013,
which is a D-6 Option A additive extension. HOST_ABI_VERSION stays at 1.

**SS-07 (Hook Bash Layer) partial anchor:** SS-07 owns the 23 `.sh` files being
ported. After W-16, those files remain on disk pending Phase H deletion (R-W16-001).
E-9 does NOT delete SS-07 artifacts — each story disables the legacy-bash-adapter
registry entries and adds new WASM entries but leaves `.sh` files in place.

## Stories

| Story ID | Title | Points | Depends On | Blocks | Status |
|----------|-------|--------|-----------|--------|--------|
| S-9.00 | Perf baseline + W-16 bundle ceiling | TBD | — | S-9.01..S-9.07 | draft |
| S-9.30 | SDK extension: host::run_subprocess (ADR-014 D-9.2) | TBD | E-8 (rc.4 SDK closure) | S-9.07 | draft |
| S-9.01 | Batch B-1: pure stdin-parse validators (4 hooks) | TBD | S-9.00 | — | draft |
| S-9.02 | Batch B-2: single file-read frontmatter validators (4 hooks) | TBD | S-9.00 | — | draft |
| S-9.03 | Batch B-3: PR/delivery file validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.04 | Batch B-4: STATE.md + cycle index validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.05 | Batch B-5: story-file + BC multi-file validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.06 | Batch B-6: cross-document lookup validators (3 hooks) | TBD | S-9.00 | — | draft |
| S-9.07 | Batch B-7: complex YAML + subprocess validators (3 hooks) | TBD | S-9.00, S-9.30 | — | draft |

**Story count: 9** (S-9.00 + S-9.30 + S-9.01..S-9.07)

**Note:** S-9.01..S-9.04 are Burst 2; S-9.05..S-9.07 are Burst 3. Story-writer
authors these in subsequent bursts following adversarial convergence per ADR-013.

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
| validate-pr-merge-prerequisites | PreToolUse:Agent | no | Medium |

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
| validate-wave-gate-prerequisite | PreToolUse:Agent | no | High (subprocess-enabled via S-9.30 host::run_subprocess) |

> **Block-mode callout:** 3 of 23 Tier 2 validators use `on_error = "block"`:
> validate-factory-path-root, validate-input-hash, and validate-template-compliance.
> Per E-8 AC-8 (inherited), these MUST have additional negative (false-block) test
> fixtures in their batch story ACs.

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
4. HOST_ABI_VERSION = 1 unchanged throughout E-9 (additive extensions only per
   ADR-014 D-9.2; D-6 Option A applies).
5. Bundle size growth within R-W16-003 W-16 ceiling (measured by S-9.00, enforced
   per-batch).
6. `host::run_subprocess` ABI extension merged (S-9.30) before S-9.07 begins
   implementation, enabling `validate-wave-gate-prerequisite` to invoke
   `verify-sha-currency.sh` under capability-gated control.

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

### D-9.2: Subprocess Capability — host::run_subprocess ABI (ADR-014, user override)

Inherited from ADR-014 D-9.2. The audit's original recommendation was to drop the
two optional subprocess paths (D-9.2 option c). The orchestrator overrode this for
`validate-wave-gate-prerequisite` on the grounds that the SHA-currency check is a
critical defense-in-depth property.

Decision: `host::run_subprocess` (BC-2.02.013) is implemented in S-9.30 as an
additive SDK extension following the D-6 Option A pattern established by S-8.10
(`host::write_file`). HOST_ABI_VERSION stays at 1.

Only `validate-wave-gate-prerequisite` (S-9.07) uses `host::run_subprocess`. All
other 22 hooks use only the existing ABI (`read_file`, `emit_event`, `log`).

`validate-state-size` git subprocess: dropped per D-9.1 deliberate simplification;
`host::run_subprocess` is NOT used for this hook.

### D-9.3: Story Granularity — 7 capability-cluster batches (ADR-014)

Inherited from ADR-014 D-9.3. 7 batched stories (S-9.01..S-9.07) grouped by host
functions required. See audit-w16.md Section 4 for the capability-cluster scheme.
Batching enables uniform BC anchoring patterns per batch and efficient adversarial
review focus on each cluster's specific risk profile.

### D-9.4: BC Anchor Strategy — reuse existing BC-7.xx family per hook

Mirrors E-8 D-2 Option C: reuse existing BCs; no new BC family.

Exception: `host::run_subprocess` requires BC-2.02.013 (authored by PO in D-3).
Each Tier 2 hook's behavioral obligations are covered by existing BC-7.xx entries.
If a port reveals unspecified behavior, a new BC is drafted under the existing
BC-7.xx sub-family for the relevant hook (not a new BC-7.02.x migration family).

Story-writer identifies BC anchor(s) per batch story during S-9.01..S-9.07
authoring (Burst 2 + Burst 3).

### D-9.5: bats orphan strategy — checklist per story (inherits R-W16-001)

Each W-16 story spec must include a task to create a bats deletion checklist for
the batch hooks' corresponding bats tests. The `.sh` files and bats tests remain on
disk until Phase H. Per R-W16-001: bats orphan migration deferred to Phase H.

---

## Risks

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---------|-------------|-----------|--------|------------|
| R-W16-001 | bats orphan migration: bats tests for `.sh` hooks become orphans after WASM port (`.sh` files remain on disk until Phase H; bats tests test bash, not WASM) | HIGH | MED | Deferred to Phase H. Each story spec includes task to document the batch's bats orphan deletion checklist. No bats tests are deleted in W-16. |
| R-W16-002 | Behavioral divergence in rewrite-clean: rewriting 143-line bash hooks in idiomatic Rust may introduce subtle semantic differences (awk regex precedence, jq null-coalescing edge cases) | MED | HIGH | Each story spec must enumerate all behavioral edge cases as explicit ACs; adversarial convergence per ADR-013 surfaces divergences before implementation. E-8 OQ-001 (ERE precedence) is the canonical risk reference. |
| R-W16-003 | Bundle size ceiling: 23 new WASM plugins may exceed the W-16 bundle growth ceiling | LOW | MED | S-9.00 measures the post-rc.4 baseline and establishes the W-16 ceiling. Each batch story budgets its per-plugin delta against the ceiling. Measurement methodology mirrors E-8 S-8.00 (`du -sb`). |
| R-W16-004 | host::run_subprocess security surface: S-9.30 adds a new capability-gated subprocess path; `SubprocessCaps` must be correctly enforced or a path traversal/privilege escalation is possible | LOW | HIGH | BC-2.02.013 I-2 Security Boundaries (MUST NOT shell-interpret, MUST NOT allow path traversal, MUST constrain working directory). S-9.30 mirrors the security review pattern from S-8.10. Adversarial convergence required. |
| R-W16-005 | YAML parsing fidelity: 2 hooks (validate-wave-gate-completeness, validate-wave-gate-prerequisite) use python3 `yaml.safe_load`; replacement with `serde_yaml` must preserve parse semantics | MED | MED | Explicit test vectors for YAML edge cases (multi-doc streams, null values, integer coercion) in S-9.07 story ACs. |

---

## Acceptance Criteria

| AC | Statement |
|----|-----------|
| AC-1 | All 23 validate-*.sh hooks have native WASM equivalents in `crates/hook-plugins/validate-*/` delivered by S-9.01..S-9.07 |
| AC-2 | `hooks-registry.toml` updated: 23 WASM entries added (`plugin = "hook-plugins/validate-*.wasm"`); 23 legacy-bash-adapter entries disabled or removed for Tier 2 hooks |
| AC-3 | Bundle size growth within the W-16 ceiling established by S-9.00; each batch story reports its per-plugin `.wasm` size delta against the S-9.00 baseline |
| AC-4 | All 7 batched stories (S-9.01..S-9.07) pass adversarial convergence per ADR-013 before implementation dispatch |
| AC-5 | `host::run_subprocess` (S-9.30) merged before S-9.07 implementation begins; S-9.07 T-0 STOP CHECK verifies this |
| AC-6 | HOST_ABI_VERSION = 1 in both `crates/hook-sdk/src/lib.rs:58` and `crates/factory-dispatcher/src/lib.rs:43` after all E-9 stories merge |
| AC-7 | Legacy bash adapter entries for the 23 Tier 2 hooks removed from hooks-registry.toml; zero `validate-*.sh` hooks route through `legacy-bash-adapter.wasm` after E-9 completes |
| AC-8 | Block-mode hooks (validate-factory-path-root, validate-input-hash, validate-template-compliance) each have at least one negative (false-block) test fixture in their batch story ACs |
| AC-9 | `.sh` files remain on disk per R-W16-001; no `.sh` bash hook files deleted in W-16 stories |

---

## Open Questions

| ID | Question | Owner | Resolution |
|----|----------|-------|------------|
| OQ-1 | W-16 bundle size ceiling: what % growth is acceptable for 23 new plugins over the post-rc.4 baseline? | story-writer (S-9.00) | Resolved by S-9.00 measurement + ceiling proposal |
| OQ-2 | validate-state-size compaction-detection: the git subprocess path is simplified away in D-9.1. If the line-count-only gate triggers too many false-block events, should we revisit at v1.2? | tech-debt | File as TD after W-16 ships; low priority |
| OQ-3 | SubprocessCaps registry: validate-wave-gate-prerequisite's `hooks-registry.toml` entry needs a `[hooks.<id>.capabilities.run_subprocess]` block with `binary_allowlist = ["*/verify-sha-currency.sh"]`. Who authors this TOML snippet? | S-9.30 + S-9.07 | S-9.30 defines the schema; S-9.07 provides the concrete registry example |

---

## Library Table

| Library | Version | Purpose | First Story |
|---------|---------|---------|-------------|
| regex | workspace (1.10+) | Pattern matching replacing grep/awk in 20+ hooks | S-9.01 |
| serde_json | workspace | stdin JSON deserialization (HookPayload via vsdd-hook-sdk) | S-9.01 |
| serde_yaml | workspace (0.9.x) | YAML frontmatter + wave-state.yaml parsing | S-9.02, S-9.07 |
| walkdir | workspace | Directory traversal replacing `find` in pr-merge-prerequisites | S-9.03 |
| vsdd-hook-sdk | 0.2.0 (post-S-8.10) | Plugin ABI (read_file, emit_event, log, run_subprocess after S-9.30) | S-9.01 |
| std::process::Command | std | Subprocess execution in dispatcher (host::run_subprocess binding) | S-9.30 |
| std::time::{Duration, Instant} | std | Timeout enforcement in host::run_subprocess dispatcher binding | S-9.30 |
| glob | workspace (TBD per S-9.30) | Binary/arg allowlist pattern matching in SubprocessCaps check | S-9.30 |
| anyhow / thiserror | workspace | Error propagation in dispatcher binding | S-9.30 |
| tempfile | dev-dep workspace | Integration test temp directories in dispatcher tests | S-9.30 |
| bats-core | >=1.10 (CI) | Bats orphan checklist verification (bash hooks remain on disk) | Per-story (D-9.5) |
| hyperfine | >=1.18 | Bundle-size measurement harness | S-9.00 |

> **Version pin note:** All library versions use workspace-level version constraints.
> Do not introduce non-workspace dependencies. `serde_yaml 0.9.x` — same pin as
> established in W-15 (see E-8 S-8.07 TD entry for 0.9.34 deprecated status; same
> constraint applies in W-16 until a workspace-level upgrade is coordinated).

---

## Architecture Mapping

| Component | Module | Pure/Effectful |
|-----------|--------|---------------|
| validate-*.wasm plugins (23 new) | `crates/hook-plugins/validate-*/` | Effectful (read_file host call + emit_event) |
| host::run_subprocess SDK wrapper | `crates/hook-sdk/src/host/run_subprocess.rs` | Effectful (FFI → wasmtime → Command::new) |
| host::run_subprocess dispatcher binding | `crates/factory-dispatcher/src/host/run_subprocess.rs` | Effectful (subprocess spawn, pipe I/O, timeout) |
| SubprocessCaps struct | `crates/factory-dispatcher/src/registry.rs` | Pure (struct field + capability schema) |
| binary_allowlist_check, arg_allowlist_check, env_strip | `crates/factory-dispatcher/src/host/run_subprocess.rs` | Pure-core (deterministic; unit-testable in isolation) |
| hooks-registry.toml (23 entry updates) | `plugins/vsdd-factory/hooks-registry.toml` | Configuration (not a code module) |
| W-16 bundle baseline + ceiling | `.factory/measurements/` + `.factory/architecture/perf-baseline-w16.md` | Pure (data artifacts) |

**Architecture section files:**
- `architecture/module-decomposition.md` — confirms `crates/hook-plugins/` belongs to SS-04; `crates/hook-sdk/` to SS-02
- `architecture/SS-02-hook-sdk.md` — host::run_subprocess entry in Schema Evolution table (after S-9.30 merges)
- `architecture/SS-04-plugin-ecosystem.md` — canonical home for all hook plugin crates

---

## Dependency Graph

```
(nothing)
    ↓
S-9.00 (perf baseline + bundle ceiling)
    ↓ [blocks S-9.01..S-9.07]
S-9.01, S-9.02, S-9.03, S-9.04, S-9.05, S-9.06  ← all parallel, depends_on S-9.00

E-8 (rc.4 SDK closure: host::write_file + HookPayload SubagentStop fields merged)
    ↓
S-9.30 (host::run_subprocess SDK extension)
    ↓ [blocks S-9.07]
S-9.07 (Batch B-7: complex YAML + subprocess)
```

**Topological order (wave scheduling):**
- Wave 0: S-9.00, S-9.30 (parallel; S-9.00 has no deps; S-9.30 depends on E-8 which is done)
- Wave 1: S-9.01, S-9.02, S-9.03, S-9.04, S-9.05, S-9.06 (all depend on S-9.00; parallel)
- Wave 2: S-9.07 (depends on S-9.00 + S-9.30)

---

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| 1.0 | 2026-05-03 | story-writer | Initial authoring — Phase D-4 Burst 1. E-9 epic for W-16 Tier 2 native WASM migration (23 validate-*.sh hooks). 9-story scope: S-9.00 + S-9.30 + S-9.01..S-9.07. Based on ADR-014 (D-9.1/D-9.2/D-9.3), audit-w16.md, and BC-2.02.013. Follows E-8 v1.9 shape. |
