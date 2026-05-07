---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: 2026-05-07
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/factory-dispatcher/src/registry.rs
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0-feature-plugin-async-semantics-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-7.06.001: hooks-registry.toml schema_version 2 — per-plugin `async: bool` field with CI lint invariant `on_error = "block"` implies `async = false`

## Description

`hooks-registry.toml` is upgraded to `schema_version = 2`. Each `[[hooks]]` entry gains an optional `async: bool` field (default: `false`; absence is equivalent to `async = false`). The dispatcher hard-refuses any registry with `schema_version != 2`. There is no backward compatibility, no downgrade path, and no migration tooling — the schema bump is a hard cut. A CI lint invariant enforces that no entry may have both `on_error = "block"` and `async = true`; violation is a hard error at registry-load time and in CI. This BC formalizes the registry-layer obligation introduced by ADR-019.

## Preconditions

1. `hooks-registry.toml` is the source of truth for plugin classification (Layer 2 of the dual-routing-tables pattern per ADR-011).
2. The dispatcher is version 2 (with `REGISTRY_SCHEMA_VERSION = 2` in `registry.rs`).
3. The operator is updating the registry file to declare per-plugin async classification.

## Postconditions

1. **Schema version**: `hooks-registry.toml` top-level header specifies `schema_version = 2`. A registry with any other value is rejected by the dispatcher at load time with `E-REG-001` (`dispatcher.schema_mismatch` event logged); **dispatcher exits with exit code 2 (fail-closed)**; explicit stderr diagnostic is emitted. This is an explicit exception to BC-1.08.001 fail-open semantics — schema-version mismatch must not be a silent failure.

2. **Per-plugin `async` field**: Each `[[hooks]]` entry MAY include `async = true` or `async = false`. Absence of the field is equivalent to `async = false`. The field is type-checked at parse time: non-boolean values (e.g., `async = "true"`) produce a `RegistryError::ParseError`. The `async` field is NOT a global default — it is per-entry only.

3. **Default-false semantics**: Within a `schema_version = 2` registry, entries that do not declare `async` are parsed as `async = false`. Operators migrating from v1 must bump the schema_version header; per-entry `async` field absence is acceptable and does not require explicit `false`.

4. **No backward compatibility**: A v2 dispatcher receiving a `schema_version = 1` registry produces a deterministic error and halts. There is no compat shim, no silent acceptance of v1 fields, and no migration tooling provided. The operator must bump `schema_version = 2` in the registry file before deploying the v2 dispatcher. Migration is mechanical: the `async` field defaults to `false` when absent, so no content changes are required beyond the header bump.

5. **Lint invariant (load-time)**: `registry.rs::validate()` checks that no entry has both `on_error = "block"` and `async = true`. If this constraint is violated, the dispatcher emits `E-REG-002` and refuses to start. The error message must name the offending plugin.

6. **Lint invariant (CI)**: VP-078 is a CI integration test that scans `hooks-registry.toml` and asserts no entry has both `on_error = "block"` and `async = true`. CI fails if this invariant is violated. This prevents future regressions where a new plugin is added with conflicting classification.

7. **Lint invariant (pre-commit)**: A pre-commit hook scans `hooks-registry.toml` for the `on_error = "block"` ⇒ `async = false` invariant. Violations fail the commit. This is the first defense layer — it catches violations before they reach CI. Pre-commit, registry-load-time, and CI-PR are the three defense layers per ADR-019 §Decision 4.

## Invariants

1. **`on_error = "block"` implies `async = false`**: An entry with `on_error = "block"` and `async = true` simultaneously is structurally invalid. Enforcement is at registry-load time (hard error `E-REG-002`), pre-commit (Invariant 5 layer 1), and CI (VP-078 layer 3). This invariant exists because a blocking plugin that is classified async would have its block verdict silently discarded by the dispatcher's fire-and-forget execution model.

2. **`async` field absence is equivalent to `async = false`**: The field is optional with a `serde(default)` of `false`. This means all legacy entries (authored before schema v2) are implicitly sync. No registry edit is required for plugins that should remain synchronous.

3. **Schema version is a hard gate**: `REGISTRY_SCHEMA_VERSION` in `registry.rs` is the single authoritative value. The dispatcher accepts only this exact version. No range check, no "≥ 2" logic, no version negotiation.

4. **Per-plugin `async` field does not affect the Claude Code envelope**: The envelope (hooks.json.template and per-platform variants) is uniformly synchronous per ADR-019. The `async` field in the registry is the dispatcher's internal classification; it has no representation in `hooks.json`. The Claude Code harness never sees per-plugin async flags.

5. **Three-layer defense in depth for `on_error=block ⇒ async=false`**: The invariant is enforced at three independent layers:
   - **Layer 1 (pre-commit)**: Pre-commit hook scans `hooks-registry.toml`; fails commit on violation. Developer learns before CI.
   - **Layer 2 (registry-load-time)**: `registry.rs::validate()` hard-errors with `E-REG-002` if invariant is violated; dispatcher refuses to start.
   - **Layer 3 (CI-PR)**: VP-078 integration test scans the registry file; CI fails before merge.
   Per ADR-019 §Decision 4. Missing any single layer is a spec violation.

6. **Specific plugins MUST be `async = true`**: The following telemetry-only plugins MUST be classified `async = true` in the v2 registry: `capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`, `session-end-telemetry`, `worktree-hooks`, `tool-failure-hooks`, `track-agent-start`, `track-agent-stop`, `session-learning`. All validator and governance plugins with `on_error = "block"` MUST remain `async = false`. The following plugins are SYNC (on_error=continue but user-visible stderr warnings require reliable delivery): `warn-pending-wave-gate`, `regression-gate`. This is an invariant (not merely a postcondition) because a future engineer flipping any of these to `async = false` would silently degrade user-facing latency, or flipping a warn plugin to async would silently drop stderr warnings. Positive classification is verified by VP-078 Harness 3.

7. **(`name`, `event`) tuple is unique per registry**: Within a `schema_version = 2` registry, the tuple (`name`, `event`) is unique across all `[[hooks]]` entries. Duplicate `name` values across DIFFERENT events are permitted (e.g., `worktree-hooks` may appear once with `event = "WorktreeCreate"` and once with `event = "WorktreeRemove"`). The `(name, event)` uniqueness is enforced at registry-load time by `registry.rs::validate()`. Violations produce `dispatcher.registry_invalid` with reason `duplicate_hook_registration` and dispatcher exits non-zero (fail-closed).

## Error Paths

| Condition | Behavior |
|-----------|----------|
| Registry `schema_version = 1` (or any non-2 value) | Dispatcher hard-errors at load time; `dispatcher.schema_mismatch` logged; **exit code 2 (fail-closed)** — explicit exception to BC-1.08.001 fail-open; explicit stderr diagnostic emitted; no plugins executed |
| `async = true` (non-boolean) in a `[[hooks]]` entry | `RegistryError::ParseError` at TOML parse time; dispatcher refuses to start |
| Entry has `on_error = "block"` AND `async = true` | `E-REG-002` at `validate()` time; `dispatcher.registry_invalid` logged with plugin name; dispatcher refuses to start |
| Registry file missing | Existing behavior per BC-1.01.014 (`RegistryError::NotFound`); unchanged |

## Related BCs

- BC-1.14.001 — depends on: the `async` field in each `RegistryEntry` is read by the dispatcher partition logic defined in BC-1.14.001
- BC-1.01.001 — amends: "Registry rejects unknown schema version" must now accept only schema_version=2 (not 1); amended separately per this cycle
- BC-1.01.007 — amends: "parses_minimal_registry with schema_version=1" test vector is now invalid; must be updated to schema_version=2
- BC-7.02.006 — sibling: dispatcher routing binds hooks via `[[hooks]]` entry fields; the new `async` field is added to this entry shape

## Architecture Anchors

- `plugins/vsdd-factory/hooks-registry.toml` — schema_version bumped to 2; `async = true` added for telemetry plugins; all block-capable validators remain `async = false`
- `crates/factory-dispatcher/src/registry.rs` — `RegistryEntry.async: bool` field with `#[serde(default)]`; `REGISTRY_SCHEMA_VERSION = 2`; `validate()` enforcing Invariant 1
- CI lint hook / bats test — VP-078 harness scanning hooks-registry.toml for `on_error=block ⇒ async=false`

## Story Anchor

TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)

## VP Anchors

- VP-078 — CI lint: no registry entry has both `on_error = "block"` and `async = true`; integration test method; P1

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `[[hooks]]` entry has no `async` key | Deserialized as `async = false` (serde default); plugin is synchronous |
| EC-002 | `[[hooks]]` entry has `async = false` (explicit) | Same behavior as absent; plugin is synchronous |
| EC-003 | `[[hooks]]` entry has `async = true` AND `on_error = "continue"` | Valid: a non-blocking plugin may be async. Dispatcher places it in async_group. |
| EC-004 | `[[hooks]]` entry has `async = true` AND `on_error = "block"` | Invalid at `validate()` time; `E-REG-002`; dispatcher refuses to start |
| EC-005 | Registry has `schema_version = 1` (operator forgot to bump) | Hard error `E-REG-001`; clear error message tells operator to update `schema_version = 2` |
| EC-006 | Registry has `schema_version = 3` (future version loaded by v2 dispatcher) | Hard error `E-REG-001`; dispatcher rejects unknown versions; forward-compatibility is not provided in v2 |
| EC-007 | `capture-commit-activity` is `async = true`; `on_error = "continue"` | Valid; classified into async_group; telemetry events fire-and-forget; no block verdict possible |
| EC-008 | `validate-template-compliance` is `async = false`; `on_error = "block"` | Valid; classified into sync_group; block verdicts reach Claude Code |
| EC-009 | `async = "yes"` (string instead of bool) in a `[[hooks]]` entry | TOML type error at parse time; `RegistryError::ParseError`; dispatcher refuses to start |
| EC-010 | Registry with zero `[[hooks]]` entries (empty plugin list); `schema_version = 2` | Valid; parses successfully; dispatcher returns exit 0 on any event (no plugins to run) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Registry v2; `async` absent | `entry.async = false` (default); plugin placed in sync_group by dispatcher | happy-path (default) |
| Registry v2; `async = true`; `on_error = "continue"` | Valid parse; plugin placed in async_group by dispatcher | happy-path (async telemetry) |
| Registry v2; `async = true`; `on_error = "block"` | `E-REG-002` at `validate()` time; dispatcher refuses to start | invariant-violation |
| Registry with `schema_version = 1` | `E-REG-001` (`RegistryError::SchemaVersion { got: 1, expected: 2 }`); no plugins run | schema-mismatch |
| Registry with `schema_version = 99` | `E-REG-001` (`RegistryError::SchemaVersion { got: 99, expected: 2 }`); no plugins run | schema-mismatch |
| `async = "true"` (string) in entry | `RegistryError::ParseError`; dispatcher refuses to start | parse-error |
| Registry v2; `capture-commit-activity` with `async = true` + `on_error = "continue"` | Valid; `capture-commit-activity` in async_group; no CI lint violation | telemetry-classification |
| Registry v2; `validate-template-compliance` with `async = false` + `on_error = "block"` | Valid; `validate-template-compliance` in sync_group; block verdict can reach Claude Code | validator-classification |
| CI lint scan: registry with `on_error = "block"` AND `async = true` | VP-078 bats test fails; CI reports violation with plugin name | ci-lint-failure |
| CI lint scan: fully valid v2 registry | VP-078 bats test passes; CI continues | ci-lint-pass |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-078 | No `[[hooks]]` entry in `hooks-registry.toml` has both `on_error = "block"` and `async = true`; scanned at CI time | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC contracts the registry schema that classifies each WASM plugin as sync or async, which is the mechanism enabling per-plugin async semantics while preserving `on_error = "block"` enforcement |
| L2 Domain Invariants | DI-014 — Schema version mismatch is a hard load error (the fail-closed schema_version=2 enforcement here is the BC-7 enforcement arm of DI-014, complementing the BC-1 arm in BC-1.14.001; DI-014 enforcement-owner now extends to BC-7) |
| Architecture Module | SS-01 (primary — runtime enforcement via `crates/factory-dispatcher/src/registry.rs::validate()`) + SS-07 (registry file shape — `plugins/vsdd-factory/hooks-registry.toml`). Frontmatter `subsystem` is SS-01 per F-P1-006 resolution: the runtime enforcement is the failure-loud mechanism and is housed in SS-01. |
| ADR | ADR-019 — Async Semantics at Registry Layer, Not Envelope Layer |
| Stories | TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07) |
| Cycle | v1.0-feature-plugin-async-semantics-pass-1 (F2) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` Section 3 (Change 2 and Change 4) and Section 5 (New BCs Needed) |
| **Confidence** | HIGH — F1 explicitly called for this BC; schema_version bump and `async` field are the direct fix for 55 silently-discarded block verdicts |
| **Extraction Date** | 2026-05-07 |

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | Schema validation: reads registry file from disk. CI lint: reads registry file in CI. No I/O during `validate()` itself (pure check on already-parsed struct). |
| **Global state access** | No global state in `validate()`. |
| **Deterministic** | YES — given same registry content, always produces same validation result. |
| **Thread safety** | YES — `validate()` is a pure check on an immutable parsed struct. |
| **Overall classification** | Deterministic with filesystem I/O at load time only; `validate()` is a pure fn. |

## Amendment 2026-05-07 (v1.2 — F2 pass-2 fix burst)

Addresses adversary pass-2 findings F-P2-001, F-P2-006, F-P2-013.

**F-P2-001 (Plugin-name uniqueness invariant)**: Added Invariant 7. The correct uniqueness constraint is on the (`name`, `event`) tuple, not `name` alone. The actual registry has intentional duplicate names across different events (e.g., `worktree-hooks` for WorktreeCreate and WorktreeRemove, `protect-secrets` for Bash and Read). Invariant 7 scopes uniqueness to the tuple and documents this correctly. Architect must update VP-077 to cite Invariant 7 (not Invariant 1) for the name-uniqueness property.

**F-P2-006 (Incomplete Invariant 6)**: Added `track-agent-start`, `track-agent-stop`, and `session-learning` to the async=true required list — all three are telemetry-only and always return Continue. `warn-pending-wave-gate` (Stop) and `regression-gate` (PostToolUse) are classified SYNC: both emit human-visible stderr warnings that must reach the user reliably; async classification would silently truncate those warnings at dispatcher process exit. Classification determined by reading plugin source (`lib.rs`) — both call `write_stderr`/`eprint!` and always return `HookResult::Continue`. ADR-019 Consequences should be updated to note that `warn-pending-wave-gate` and `regression-gate` are deliberately SYNC with `on_error=continue`. (Flag for architect to sync ADR-019.)

**F-P2-013 (Postcondition 3 framing)**: Postcondition 3 reworded to remove "preserves the behavior of every validator and governance plugin" framing. ADR-019 prohibits backward compat shim. New wording: "Within a `schema_version = 2` registry, entries that do not declare `async` are parsed as `async = false`. Operators migrating from v1 must bump the schema_version header."

## Amendment 2026-05-07 (v1.1 — F2 pass-1 fix burst)

Addresses adversary pass-1 findings F-P1-003, F-P1-004, F-P1-005, F-P1-006, F-P1-010, F-P1-016.

**F-P1-006 (Subsystem anchor)**: Frontmatter `subsystem` changed from `SS-07` to `SS-01`. The runtime enforcement (`registry.rs::validate()`) lives in SS-01. SS-07 remains as secondary subsystem in the Architecture Module field. Per adversary option (b): keep one BC, choose primary subsystem reflecting the runtime enforcement mechanism. Architecture Module field updated to document both subsystems with rationale.

**F-P1-004 / F-P1-011 (Fail-closed)**: Postcondition 1 and Error Paths row 1 amended. Schema-version mismatch is now explicitly **exit code 2 (fail-closed)** with explicit stderr diagnostic. Previous text "exit per BC-1.08.001 fail-open convention" was contradictory with "hard error." Same fix applied in BC-1.14.001 and BC-1.08.001.

**F-P1-005 (Pre-commit layer)**: Postcondition 7 (pre-commit) added. Previously BCs documented only two defense layers (load-time + CI); ADR-019 §Decision 4 mandated three. Invariant 5 added to enumerate all three layers (pre-commit, registry-load-time, CI-PR) explicitly.

**F-P1-016 (Classification as invariant)**: Postcondition 7 (old classification table) promoted to Invariant 6. The specific named plugins that MUST be `async = true` are now invariant-level (catching future flips), not just a postcondition. Added `worktree-hooks` and `tool-failure-hooks` to the list per F-P1-016 adversary finding. Note to architect: VP-078 needs a positive-classification verification harness (Harness 3) per F-P1-016 — this is a cross-burst dependency on architect's VP-078 work.

**F-P1-010 (Story Anchor)**: Story Anchor updated from "Story A + Story B + Story D" to "TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)". Same change in Traceability Stories field.

**F-P1-003 (DI-014 citation)**: L2 Domain Invariants updated from "TBD" to DI-014 with explanation of the BC-7 enforcement arm.
