---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.07 v1.0
target_file: .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 14
findings_high: 6
findings_med: 5
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.07 v1.0 (Pass 1)

## Finding ID Convention

`F-S807-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S807-P1-001: wasm32-wasi target wrong; correct target is wasm32-wasip1

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** T-2 / Library & Framework Requirements
- **Description:** Verified against `crates/hook-sdk/src/lib.rs` and `crates/hook-plugins/capture-commit-activity/Cargo.toml`: the project mandates `wasm32-wasip1`. `wasm32-wasi` is deprecated. Same as F-S804-P1-002; universal systematic fix required across all Tier 1 stories.
- **Evidence:** crates/hook-plugins/capture-commit-activity/Cargo.toml: `[lib] crate-type = ["cdylib"]`; build target in CI: `wasm32-wasip1`.
- **Proposed Fix:** Replace wasm32-wasi with wasm32-wasip1 throughout.

#### F-S807-P1-002: AC-002 prescribes deletion of hooks.json entries that DO NOT EXIST (D-7 architecture)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002
- **Description:** Same class as F-S801-P1-001. D-7 already shipped; hooks.json command entries for warn-pending-wave-gate do not exist. AC-002's deletion step is a phantom operation.
- **Proposed Fix:** Remove hooks.json deletion AC or rewrite as verification assertion.

#### F-S807-P1-003: serde_yaml NOT in workspace.dependencies; AC-006/T-2/Library table omit workspace-Cargo.toml step; serde_yaml deprecated 2024

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** AC-006 / T-2 / Library & Framework Requirements
- **Description:** The story adds serde_yaml as a dependency but: (1) serde_yaml is not in the workspace Cargo.toml `[workspace.dependencies]` — the story must add it there first, then reference it via `{ workspace = true }` in the crate Cargo.toml; (2) serde_yaml was deprecated by dtolnay in 2024 and is unmaintained. The story does not include the workspace-Cargo.toml modification step, and does not address the deprecation.
- **Evidence:** Workspace Cargo.toml lacks serde_yaml. dtolnay/serde-yaml README: "no longer maintained."
- **Proposed Fix:** (1) Add explicit task: "Add serde_yaml (or chosen alternative) to workspace Cargo.toml [workspace.dependencies]." (2) Add a decision point for serde_yaml alternative selection (same as F-S804-P1-006 class). Block on decision before implementation.

#### F-S807-P1-004: host::emit_event signature mismatch — bash uses type=hook.block as KV; SDK has separate positional event_type + fields slice

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** AC-005 / T-5
- **Description:** The bash source calls bin/emit-event with `type=hook.block` as a key-value argument. The SDK's `host::emit_event` signature is `emit_event(event_type: &str, fields: &[(key, value)])` — a separate positional event_type string plus a fields slice. The spec does not map the bash call arguments to the SDK signature, leaving the implementer to guess the mapping.
- **Evidence:** SDK `host.rs`: `fn emit_event(event_type: &str, fields: &[(&str, &str)]) -> Result<(), HostError>`. Bash: `bin/emit-event type=hook.block field1=val1`.
- **Proposed Fix:** Add explicit mapping in AC-005/T-5: `event_type = "hook.block"`, `fields = [("field1", val1), ...]` per the bash call site.

#### F-S807-P1-005: Bash uses python3 as soft dependency (early-exit if absent); AC-004 misses python3-absent equivalence path

- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Location:** AC-004
- **Description:** The bash source uses python3 with a soft-dependency early-exit: if python3 is absent, the hook exits 0 without warning. AC-004 covers the main warning paths but does not specify the python3-absent equivalence path for the WASM port. The WASM port has no python3 dependency, so the behavior change (no early-exit on python3 absence) is a silent behavioral difference vs bash that must be explicitly documented.
- **Proposed Fix:** Add AC or EC: "WASM port does not use python3; python3-absent path from bash (early-exit) is superseded — equivalent WASM behavior is always execute the warning check."

#### F-S807-P1-006: D-6 Option A trigger condition incorrectly evaluated; read_file IS available but registry capability declaration missing

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** T-4 / AC-003 capability declaration
- **Description:** Unlike S-8.04 (which needs write_file — absent), warn-pending-wave-gate needs only read_file (host::read_file IS present in SDK). However, the story omits the required registry capability declaration `[hooks.capabilities.read_file] path_allow=[".factory/wave-state.yaml"]`. Without this declaration, the hook will fail at runtime when attempting to read the state file.
- **Evidence:** SDK host.rs: `fn read_file(path: &str) -> Result<Vec<u8>, HostError>` — present. Registry entries require explicit path_allow declarations for read_file.
- **Proposed Fix:** Add `[hooks.capabilities.read_file] path_allow=[".factory/wave-state.yaml"]` to the registry migration specification in AC-003/T-4.

### MEDIUM

#### F-S807-P1-007: Wave 15 conflicts with E-8 D-13 "provisional" status; wave: 15 [process-gap] disclosure missing

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter / Goal section
- **Description:** The story targets Wave 15 but E-8 D-13 marks Wave 15 as "provisional" (calendar-gated post-v1.0.0 GA). The story does not include the `wave: 15 [provisional]` disclosure that sibling S-8.00 established as required.
- **Proposed Fix:** Add wave: 15 [provisional] disclosure per S-8.00 pattern.

#### F-S807-P1-008: input-hash 5015917 unverifiable; commit message doesn't match recent git log

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash comment
- **Description:** The input-hash comment references a commit message that does not appear in the recent factory-artifacts git log. The hash may be stale or from a different branch.
- **Proposed Fix:** Re-derive the input-hash from the E-8 epic content hash per convention.

#### F-S807-P1-009: AC-004 conflates 3 distinct early-exit conditions without pinning equivalence to bash branches

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** AC-004
- **Description:** AC-004 covers multiple early-exit conditions (wave-state absent, field absent, status not pending) but does not map each to the specific bash branch it replaces. The implementer cannot verify parity without the explicit bash → WASM mapping.
- **Proposed Fix:** Add inline comments in AC-004 mapping each condition to the corresponding bash branch: (a) wave-state.yaml absent → bash `[[ ! -f ... ]] && exit 0`; (b) field absent → jq null fallback; (c) status not pending → string comparison branch.

#### F-S807-P1-010: AC-006 says "binary_allow can be empty or section removed" but does not specify which is correct

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** AC-006
- **Description:** AC-006 presents two alternatives for the post-port binary_allow state: empty array or section removed entirely. The hooks-registry.toml schema behavior differs for these two cases (empty array may be treated differently than absent key). The spec must specify which is correct.
- **Proposed Fix:** Pin the correct form: either `binary_allow = []` (empty array) or remove the field entirely, and confirm which the registry schema expects for a WASM hook.

#### F-S807-P1-011: AC-005 perf comparison "vs S-8.00 baseline" impossible — S-8.00 doesn't measure warn-pending-wave-gate

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-005
- **Description:** AC-005 prescribes performance comparison against the S-8.00 baseline. The S-8.00 baseline does not measure warn-pending-wave-gate (S-8.00 measured a different hook set). The comparison target does not exist.
- **Proposed Fix:** Remove the S-8.00 baseline reference. If perf measurement is desired, define a new per-story baseline or mark INFORMATIONAL per E-8 AC-7 Tier 1 exclusion.

### LOW

#### F-S807-P1-012: Library table lists vsdd-hook-sdk without path-dep convention or workspace member name

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Library & Framework Requirements table
- **Description:** The Library table lists `vsdd-hook-sdk` without specifying whether it's referenced as a path dependency (`path = "../../crates/hook-sdk"`) or a workspace member (`{ workspace = true }`). The correct form is the path-dep convention used by existing hook plugins.
- **Proposed Fix:** Specify: `vsdd-hook-sdk = { path = "../../crates/hook-sdk" }` (or workspace form if the SDK is added to workspace.dependencies).

#### F-S807-P1-013: Bats test path tests/integration/hooks/ unverified to exist

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** T-6 bats test path
- **Description:** T-6 specifies bats tests at `tests/integration/hooks/`. This path should be verified against the actual test directory structure; other stories use different path conventions.
- **Proposed Fix:** Verify the bats test path against the actual project test layout and correct if needed.

### NIT

#### F-S807-P1-014: EC-006 "host fn unavailable" framing is incorrect — host fn IS available

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** EC-006
- **Description:** EC-006 frames the emit_event failure scenario as "host fn unavailable." For warn-pending-wave-gate, host::emit_event IS available (it exists in the SDK). The correct framing is "host::emit_event returns an error" not "host fn unavailable."
- **Proposed Fix:** Reframe EC-006 as "host::emit_event returns Err" rather than "host fn unavailable."

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 6 |
| MEDIUM | 5 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision; serde_yaml deprecation decision required before implementation

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 14/14 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 14 |
| **Verdict** | FINDINGS_REMAIN |
