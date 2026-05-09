---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-08T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "40a6fb6"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
origin: greenfield
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.11.001
section: "4.11"
---

# BC-4.11.001: validate-artifact-path WASM hook MUST consult artifact-path-registry.yaml as single source of truth and block (immediate mode) writes whose paths do not match a registered pattern

## Description

The `validate-artifact-path` WASM hook fires on `PreToolUse` events for `Write` and `Edit`
tool calls targeting paths under `.factory/`. It reads `plugins/vsdd-factory/config/artifact-path-registry.yaml`
as the single source of truth for canonical artifact locations and enforces writes against
registered patterns. Per OQ5 resolution, the hook ships in immediate `block` mode from
registration — no phased warn-then-block rollout. The 9 explicit creation skills that write
to `.factory/` MUST resolve their target paths through the same registry before writing.
The hook uses `HOST_ABI_VERSION = 1` and the canonical Why/Fix/Code block-message pattern.

## Preconditions

1. The hook is registered in `hooks-registry.toml` for `PreToolUse` events filtering on
   tool names `Write` and `Edit`.
2. `plugins/vsdd-factory/config/artifact-path-registry.yaml` exists and is readable via
   `host::read_file` (accessible via WASI preopened project root).
3. The `HookPayload` contains `tool_input.file_path` (the write target path).
4. The hook's pure `fn hook_logic(...)` function takes all host I/O as injectable closures.
   Unit tests exercise every branch without a WASM runtime.
5. The `relocate-artifact` skill has been run with `--apply` (zero violations remaining)
   BEFORE the hook is registered in `hooks-registry.toml`. This sequencing prerequisite is
   a hard delivery constraint enforced by Story C's acceptance criteria (per OQ5).

## Postconditions

1. The hook reads `plugins/vsdd-factory/config/artifact-path-registry.yaml` via `host::read_file`
   on each invocation. The registry is never embedded as literals in the hook source.
2. For each `Write`/`Edit` call where `tool_input.file_path` targets `.factory/` — in
   either relative form (e.g., `.factory/specs/foo.md`) or absolute form (e.g.,
   `/abs/proj/.factory/specs/foo.md`); see Invariant 8 for path-form recognition discipline:
   a. The hook extracts the file path from the payload.
   b. The hook attempts to match the path against each registered pattern in the registry.
   c. Each registry entry has an `enforcement_level` field: `block | warn | advisory`.
3. **Path matches a registry entry with `enforcement_level: block`:**
   - The write PROCEEDS (the path matches a valid canonical pattern).
   - Hook returns `HookResult::Continue`.
4. **Path matches a registry entry with `enforcement_level: warn`:**
   - The hook emits a `hook.warn` event via `host::emit_event`.
   - The hook writes to stderr: `"[validate-artifact-path] WARN: <path> matches registry pattern '<pattern>' with enforcement_level: warn"`
   - Hook returns `HookResult::Continue` (write proceeds).
5. **Path matches a registry entry with `enforcement_level: advisory`:**
   - The hook calls `host::log_debug(...)` with an advisory message.
   - No stderr output.
   - Hook returns `HookResult::Continue` (write proceeds).
6. **Path does NOT match any registry entry:**
   - The hook emits `HookResult::block_with_fix(hook, reason, recommendation, code)`:
     - `hook`: `"validate-artifact-path"`
     - `reason`: `"Write to '<path>' under .factory/ has no matching entry in plugins/vsdd-factory/config/artifact-path-registry.yaml"`
     - `recommendation`: `"Consult the registry to find the canonical path for this artifact type. If the artifact type is new, use /vsdd-factory:register-artifact to add it to the registry first. Do not invent directory names."`
     - `code`: `"ARTIFACT_PATH_UNREGISTERED"`
   - The write is BLOCKED.
7. **Path is outside `.factory/`:** Hook returns `HookResult::Continue` immediately. No
   registry lookup. (The hook is scoped to `.factory/` paths only — for both relative AND
   absolute path forms; see Invariant 8. A path like `prefix.factory/foo.md` does NOT
   qualify as a `.factory/` path under the leading-slash discipline.)
8. The 9 creation skills (`create-adr`, `create-architecture`, `create-brief`,
   `create-domain-spec`, `create-excalidraw`, `create-prd`, `create-story`,
   `register-artifact`, `conform-to-template`) MUST read the registry via `Read` tool at
   the start of their procedure and resolve target paths against registry patterns before
   calling `Write`. A skill that writes without a prior registry check is in violation of
   this BC's skill-layer invariant.
9. Bare `HookResult::block()` calls are prohibited. All block messages use `block_with_fix`.

## Invariants

1. `plugins/vsdd-factory/config/artifact-path-registry.yaml` is the SINGLE source of truth.
   Neither the hook, the creation skills, nor the `relocate-artifact` skill embed a duplicate
   path list. All three read the registry at runtime.
2. The `enforcement_level` field is read per registry entry. The hook MUST respect the per-entry
   level and MUST NOT treat all entries as `block`.
3. A write to `.factory/` that matches NO registry entry is ALWAYS blocked, regardless of
   enforcement_level. Unregistered paths are treated as `block` by default.
4. The hook MUST NOT modify the registry file. It is read-only.
5. The hook fires on `PreToolUse` — before the write occurs. The block prevents the write.
   A block on a tool already in flight (PostToolUse) would be too late.
6. Pattern matching uses `canonical_path_pattern` fields with `{placeholder}` expansion.
   A `{placeholder}` matches any non-empty sequence of characters that does NOT contain
   `/` (a single path segment). Multi-segment spanning is prohibited — `{placeholder}`
   cannot match a path containing a slash. This means `.factory/cycles/foo/bar/doc.md`
   does NOT match `.factory/cycles/{cycle-id}/doc.md` because `foo/bar` spans two
   segments. (Amended v1.1 per NC-1 / F5 pass-1 fix burst 2026-05-07: tightens from
   "sequence of segments" to "single segment" to match the implemented behavior.)
7. The relocate-artifact delivery prerequisite (Postcondition 5) is a hard sequencing
   constraint. Registering the hook before `relocate-artifact --apply` produces zero
   violations is a delivery error.
8. **I-4.11.001-8 (Path Form Invariance):** The hook MUST accept `tool_input.file_path`
   in both relative form (e.g., `.factory/specs/foo.md`) and absolute form (e.g.,
   `/abs/proj/.factory/specs/foo.md`). Recognition of `.factory/` is by leading-slash
   discipline implemented in `crates/hook-plugins/validate-artifact-path/src/lib.rs`:
   - **Relative form:** `path.starts_with(".factory/")` — used as-is for registry matching.
   - **Absolute form:** `path.find("/.factory/")` — the prefix up to and including the
     leading slash is stripped, yielding the `.factory/…` relative form for registry matching.
   - **False-positive prevention:** A path like `prefix.factory/foo.md` contains no `/`
     immediately before `.factory/`, so it does NOT satisfy either condition and is treated
     as outside `.factory/` scope (early-exit `Continue`). The leading slash is the
     discriminator.
   This invariant was introduced by commit 8b4f697f (which propagated the sibling fix from
   `validate-stable-anchors` cc5a016b) and applies identically in both `matches_canonical`
   (the pure matching function) and `hook_logic` (the entry-point dispatcher) in `lib.rs`.
   A re-implementation that only accepts relative paths silently bypasses absolute-path
   `Write`/`Edit` calls, re-introducing the F-P18-001 class of bug.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Registry YAML file is absent (fresh install before Story C ships) | Graceful degrade: hook returns `HookResult::Continue`. Logs: `"[validate-artifact-path] registry absent at plugins/vsdd-factory/config/artifact-path-registry.yaml — graceful degrade"`. Does NOT block. |
| EC-002 | Registry YAML is malformed (parse error) | Graceful degrade: hook returns `HookResult::Continue`. Logs parse error via `host::log_error`. Does NOT block. |
| EC-003 | Write target is a `.factory/` path that predates the registry (legitimately placed before Story C) | Hook matches against registry. If matched: Continue. If not matched: BLOCK. Mitigation: the relocation sweep (Postcondition 5) ensures zero legacy mismatches before hook registration. |
| EC-004 | Write target is outside `.factory/` (e.g., `src/lib.rs`) | Early-exit `HookResult::Continue`. No registry lookup. |
| EC-005 | Registry has two entries whose patterns both match the write target | First matching entry wins. The `enforcement_level` of the first match is applied. |
| EC-006 | `tool_input.file_path` field is absent from the payload | Graceful degrade: hook returns `HookResult::Continue`. Logs `host::log_warn`. No block on missing data. |
| EC-007 | Registry grows to 100+ entries (performance concern) | Pattern matching is linear scan. For 100 entries, WASM execution time MUST remain under 200ms. Binary size constraint: keep under 500 KB (if `serde_yaml` exceeds this, switch to a minimal YAML-subset parser per F1 Section 12 YAML dependency note). |

## Canonical Test Vectors

| Write Path | Registry State | Enforcement Level | Expected Result |
|-----------|---------------|-----------------|----------------|
| `.factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md` | Matched: artifact_type=behavioral-contract | block | Continue (valid canonical path) |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md` | Matched | warn | Continue + warn event + stderr |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md` | Matched | advisory | Continue + log_debug |
| `.factory/feature-deltas/F1-delta.md` | No match | — | BLOCK: ARTIFACT_PATH_UNREGISTERED |
| `src/main.rs` | — (non-.factory path) | — | Continue (early exit) |
| `.factory/specs/prd.md` | Matched: artifact_type=prd | block | Continue |
| Registry file absent | — | — | Continue (graceful degrade) |
| Registry YAML malformed | — | — | Continue (graceful degrade) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-069 | Registry load never panics on malformed YAML | proptest (fuzz registry contents; assert no panic) |
| VP-070 | Path matching is pure and deterministic given same registry + path | kani (pure function; deterministic given fixed inputs) |
| VP-072 | No skill or hook embeds a duplicate path list (single-source-of-truth invariant) | cross-cutting integration test: grep skills and hook source for hardcoded `.factory/` path patterns; assert zero hits |
| (unit-test) | BLOCK emitted for unregistered .factory/ path | Rust unit test (injectable callbacks) |
| (unit-test) | Continue returned for non-.factory/ path | Rust unit test |
| (unit-test) | Graceful degrade on absent registry | Rust unit test (read_file callback returns Err) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the `validate-artifact-path` WASM plugin, which is authored using the `vsdd-hook-sdk` crate's `HookPayload`, `HookResult`, and `host::read_file` host bindings. CAP-009 defines the SDK as the interface through which first-party plugin authors implement hook behavior without touching the dispatcher. This BC specifies the full behavioral contract of a first-party WASM plugin that reads the artifact registry via `host::read_file` and enforces path governance via `HookResult::block_with_fix`. |
| Secondary Capability Reference | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008 — the hook fires on `PreToolUse` events, which is CAP-008's mechanism for gating tool calls before execution. The primary anchor is CAP-009 (plugin authoring) because this BC specifies the WASM implementation contract; CAP-008 describes the lifecycle event this plugin hooks into. |
| L2 Domain Invariants | none |
| Architecture Module | crates/hook-plugins/validate-artifact-path/ (Rust crate); plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm (build output); plugins/vsdd-factory/config/artifact-path-registry.yaml (single source of truth); hooks-registry.toml (registration) |
| Stories | Story C (v1.0-feature-engine-discipline-pass-1 F3 decomposition) |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-6.22.001 — composes with (relocate-artifact skill; the skill MUST run --apply before this hook is registered; sequencing constraint)
- BC-4.10.001 — sibling (another WASM hook in this cycle; same HOST_ABI_VERSION = 1 and block_with_fix pattern)
- BC-4.10.002 — sibling (graceful degrade pattern; this BC uses the same graceful degrade idiom for absent registry)

## Architecture Anchors

- `crates/hook-plugins/validate-artifact-path/src/lib.rs` — pure `fn hook_logic(...)` with injectable read_file callback; all unit tests inline
- `crates/hook-plugins/validate-artifact-path/src/main.rs` — WASM entry point wiring `host::read_file`, `host::emit_event`, `host::log_*`
- `plugins/vsdd-factory/config/artifact-path-registry.yaml` — single source of truth for canonical artifact locations
- `hooks-registry.toml` — event: PreToolUse; tool filter: Write, Edit
- `plugins/vsdd-factory/skills/create-adr/SKILL.md` et al. — 9 creation skills that must add registry-read step (Story C implementation scope)

## Story Anchor

Story C — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

- VP-069 — registry load never panics on malformed YAML
- VP-070 — path matching is pure and deterministic
- VP-072 — single-source-of-truth invariant (cross-cutting)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). OQ5 resolution applied: immediate `block` mode from registration — no phased warn-then-block rollout for the hook itself. Enforcement_level field in registry governs per-entry behavior (block/warn/advisory), not a global rollout phase. D-337 constraint applied: WASM-only. |
| 1.1 | 2026-05-07 | Invariant 6 amendment (architect; NC-1, F5 pass-1 fix burst): `{placeholder}` semantics tightened from "any non-empty path segment or sequence of segments" to "single path segment (no `/`)". This is Option A per Appendix A of F5-pass-1-fix-plan.md and matches the implemented behavior in `validate-artifact-path/src/lib.rs`. Input-hash recomputed from `[pending-recompute]` to `40a6fb6`. |
| 1.2 | 2026-05-08 | F-P19-003 — explicit absolute-path semantics + leading-slash discipline (8b4f697f introduced behavior; spec was silent). Added Invariant 8 (Path Form Invariance) with full leading-slash discipline sourced from `crates/hook-plugins/validate-artifact-path/src/lib.rs` (`matches_canonical` and `hook_logic`). Amended Postconditions 2 and 7 to cross-reference Invariant 8. A re-implementer reading v1.1 spec alone could produce a relative-only hook and re-introduce the F-P18-001 bug. Refs: F-P19-003, F-P18-001. |
