---
document_type: behavioral-contract
level: L3
version: "v1.1"
status: draft
producer: product-owner
timestamp: 2026-08-06T00:00:00Z
last_amended: "2026-08-16 (v1.1) — SR-001 spec-fidelity correction (product-owner): Replaced phantom HookEntry struct name with actual RegistryEntry; corrected on_error field type from OnError to Option<OnError> in PC5 and Architecture Anchors; fixed struct-name references across all sections. Behavioral semantics unchanged. BC-1.01.016 v1.1."
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/research/wasm-fuel-exhaustion-detection.md
input-hash: "7a9cdc6"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill-E21-W5
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.01.016: factory-dispatcher::registry::failure_policy field — parse semantics, backward-compatibility default (fail-open), unknown-value rejection, and on_error axes independence (ADR-039 §Decision 1+2 Phase 1 schema leg)

## Description

The factory-dispatcher registry loader MUST accept a new per-plugin `failure_policy` TOML field
on each `[[hook]]` entry in `hooks-registry.toml`. This field governs resource-exhaustion outcomes
(fuel/epoch) and is **orthogonal to the existing `on_error` field**, which governs crash outcomes.
The two axes cover distinct failure classes (ADR-039 §Decision 1):

| Field | Failure class it governs | Examples |
|-------|--------------------------|---------|
| `on_error` (existing) | Plugin crashes and host-side invocation errors | `Trap::UnreachableCodeReached`; ABI mismatch |
| `failure_policy` (NEW) | Resource exhaustion — `TimeoutCause::Fuel`, `TimeoutCause::Epoch` | `Trap::OutOfFuel`; epoch deadline exceeded |

The `failure_policy` field MUST parse to a `FailurePolicy` enum with two variants:
`FailurePolicy::FailClosed` (value `"fail-closed"`) and `FailurePolicy::FailOpen` (value
`"fail-open"`). Any other value MUST cause `Registry::parse_str` to return `Err` at serde
parse time — the same discipline BC-1.01.011 establishes for `on_error`. When absent from a
stanza, `failure_policy` MUST default to `FailurePolicy::FailOpen`, preserving backward
compatibility with all 52 existing plugin entries. The `RegistryEntry` struct MUST hold `on_error`
and `failure_policy` as independent fields that never collapse their semantics.

**Phase 1 scope boundary:** This BC governs the **schema extension only**, with NO enforcement
change. The existing `plugin_fail_closed` function in `executor.rs` is NOT modified. Enforcement
is deferred to BC-1.03.017 / S-21.11. PC7 is a RED Gate: any modification to `plugin_fail_closed`
behavior for exhaustion outcomes in this story is out-of-scope and causes PC7 to fail.

## Preconditions

1. Registry loader is the `Registry::parse_str` function in the `factory-dispatcher` crate.
2. TOML input is a `[[hook]]` stanza from `plugins/vsdd-factory/hooks-registry.toml` or a
   test fixture.
3. The `FailurePolicy` enum and `RegistryEntry.failure_policy` field do not yet exist in
   `registry.rs` at story dispatch time.

## Postconditions

1. **PC1 — `"fail-closed"` parsed to `FailClosed`:** A `[[hook]]` stanza with
   `failure_policy = "fail-closed"` parses without error; `RegistryEntry.failure_policy` is set
   to `FailurePolicy::FailClosed`.

2. **PC2 — `"fail-open"` parsed to `FailOpen`:** A `[[hook]]` stanza with
   `failure_policy = "fail-open"` parses without error; `RegistryEntry.failure_policy` is set to
   `FailurePolicy::FailOpen`.

3. **PC3 — Unknown values rejected at parse time:** A `[[hook]]` stanza with any
   `failure_policy` value other than `"fail-closed"` or `"fail-open"` (e.g., `"shout"`,
   `"ignore"`, `"FAIL-CLOSED"`) causes `Registry::parse_str` to return `Err`. The error is
   produced by serde at parse time, before any registry validation. No silent default to any
   variant is permitted.

4. **PC4 — Absent field defaults to `FailOpen`:** A `[[hook]]` stanza without a
   `failure_policy` field parses successfully; `RegistryEntry.failure_policy` is set to
   `FailurePolicy::FailOpen`. The absence of the field MUST NOT cause a parse error.

5. **PC5 — Axes independence in `RegistryEntry`:** The `RegistryEntry` struct has the existing
   `on_error: Option<OnError>` field and the new `failure_policy: FailurePolicy` field as
   independent axes. A stanza with both `on_error = "continue"` and
   `failure_policy = "fail-closed"` is representable: both fields hold their respective values
   simultaneously without structural conflict. The fields MUST NOT be collapsed into a single
   enum or combined representation.

6. **PC6 — Existing 52 registry entries parse without change:** All `[[hook]]` entries in the
   production `plugins/vsdd-factory/hooks-registry.toml` (none of which contain a
   `failure_policy` field) parse cleanly after this change; all resolve to
   `FailurePolicy::FailOpen`. No existing plugin changes enforcement behavior.

7. **PC7 — Phase 1 no-enforcement gate (RED Gate):** The `plugin_fail_closed` function in
   `executor.rs` is NOT modified by this story. The existing test
   `fail_closed_timeout_with_on_error_continue_is_open` (in the executor module) MUST pass
   unmodified after this story's changes land. This postcondition is a RED Gate: its test
   MUST FAIL if `plugin_fail_closed` behavior is changed for
   `Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }` outcomes.

## Invariants

1. **No-silent-default invariant:** `failure_policy` typos never silently default to any
   variant. This is the same serde-enum-rejection discipline as BC-1.01.011 Invariant 1 for
   `on_error`. Serde's enum-variant rejection catches typos and unknown values at parse time.

2. **Absent-field-is-fail-open invariant:** When `failure_policy` is absent from a stanza,
   the result is always `FailurePolicy::FailOpen`. No existing plugin entry changes enforcement
   behavior when the field is absent. The backward-compatible default is deliberate (ADR-039
   §Decision 1 backward-compatibility clause): there is no period during which unannoted
   plugins silently fail-closed.

3. **Axes-independence invariant:** `on_error` and `failure_policy` govern orthogonal failure
   classes. A plugin may simultaneously carry `on_error = "continue"` (crash = advisory) and
   `failure_policy = "fail-closed"` (exhaustion = block). The two fields coexist in `RegistryEntry`
   without semantic collision. Neither field's value affects the other's parsing or storage.

4. **Phase 1 no-enforcement-change invariant:** The `failure_policy` value is parsed and stored
   by the registry loader, but is NOT yet consulted by any enforcement path in Phase 1. The
   schema extension is safe to ship independently because no plugin's block/allow decision
   changes until S-21.11 (Phase 4) extends `plugin_fail_closed`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `failure_policy = "FAIL-CLOSED"` (wrong case) | `Err` — serde enum variant matching is case-sensitive; no silent case-folding |
| EC-002 | `failure_policy = ""` (empty string) | `Err` at parse time |
| EC-003 | `failure_policy = "fail_closed"` (underscore instead of hyphen) | `Err` — canonical values use hyphen; underscore variant is unrecognized |
| EC-004 | `failure_policy` field appears twice in the same stanza | TOML parse error (duplicate key); not a registry-layer concern |
| EC-005 | Plugin with `on_error = "block"` AND `failure_policy = "fail-open"` | Struct holds both values without conflict; crash blocks via `on_error = block`; exhaustion advisory via `failure_policy = fail-open` (enforcement in Phase 4 only) |
| EC-006 | Plugin with `on_error = "continue"` AND `failure_policy = "fail-closed"` | Both parsed and stored; `RegistryEntry` holds `OnError::Continue` and `FailurePolicy::FailClosed` simultaneously; no parse error; no enforcement change in Phase 1 |
| EC-007 | All 52 existing `hooks-registry.toml` entries (no `failure_policy` field) | All parse cleanly to `FailurePolicy::FailOpen`; no behavior change; confirmed by spot-check of at least 3 entries |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TOML stanza with `failure_policy = "fail-closed"` | `RegistryEntry.failure_policy == FailurePolicy::FailClosed` | happy-path |
| TOML stanza with `failure_policy = "fail-open"` | `RegistryEntry.failure_policy == FailurePolicy::FailOpen` | happy-path |
| TOML stanza with no `failure_policy` field (current production format) | `RegistryEntry.failure_policy == FailurePolicy::FailOpen`; no parse error | backward-compat |
| TOML stanza with `failure_policy = "unknown-value"` | `parse_str` returns `Err` | error |
| TOML stanza with `failure_policy = "FAIL-CLOSED"` | `parse_str` returns `Err` (case-sensitive) | error |
| TOML stanza with `on_error = "continue"` + `failure_policy = "fail-closed"` | Both fields stored; `RegistryEntry.on_error == OnError::Continue`, `RegistryEntry.failure_policy == FailurePolicy::FailClosed`; no conflict | axes-independence |
| Full production `hooks-registry.toml` (52 entries, no `failure_policy` fields) | All entries parse successfully; all resolve to `FailurePolicy::FailOpen`; all existing tests pass | regression |

## Related BCs

- **BC-1.01.011** — sibling: establishes the `on_error` unknown-value serde-rejection discipline
  (Invariant 1); BC-1.01.016 applies the same pattern to `failure_policy`
- **BC-1.03.017** — successor: governs the `failure_policy` enforcement semantics (Phase 4);
  BC-1.01.016 is the prerequisite schema extension that BC-1.03.017's enforcement postconditions
  depend on; S-21.10 (this BC) MUST merge before S-21.11 (BC-1.03.017)
- **BC-1.03.002** — context: governs fuel-exhaustion detection (`invoke_plugin` returns
  `PluginResult::Timeout { cause: TimeoutCause::Fuel }` when fuel is exhausted); BC-1.01.016
  governs what the registry stores about the enforcement policy to apply to that result

## Architecture Anchors

- `crates/factory-dispatcher/src/registry.rs` — `FailurePolicy` enum (new type; variants
  `FailClosed` and `FailOpen`); serde `#[derive(Deserialize, Serialize)]`; `Default` impl
  returns `FailurePolicy::FailOpen`; `RegistryEntry.failure_policy: FailurePolicy` field (new);
  `#[serde(default)]` on the field for absent-field backward-compat; existing
  `RegistryEntry.on_error: Option<OnError>` field unchanged (two-level default resolution via
  `on_error()` accessor falling back to `RegistryDefaults.on_error`; defaults to `Continue`)
- `plugins/vsdd-factory/hooks-registry.toml` — read-only in Phase 1; no `failure_policy`
  entries added until Phase 4 (S-21.11)
- `crates/factory-dispatcher/src/executor.rs` — read-only; `plugin_fail_closed` MUST NOT be
  modified in Phase 1; `fail_closed_timeout_with_on_error_continue_is_open` test MUST pass
  unmodified (PC7)

## Story Anchor

S-21.10 (Phase 1 schema extension; no enforcement change; blocks S-21.11)

## VP Anchors

- VP-TBD — FailurePolicy enum parse semantics: known values parse to correct variants; unknown
  values produce `Err` at serde parse time; absent field defaults to `FailOpen`; `on_error`
  and `failure_policy` are independent fields in `RegistryEntry`; 52 existing entries parse without
  change; `plugin_fail_closed` behavior is unchanged

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | For any `failure_policy` TOML value: `"fail-closed"` → `FailurePolicy::FailClosed`; `"fail-open"` → `FailurePolicy::FailOpen`; any other value → `Err` at parse time; absent → `FailurePolicy::FailOpen`. `on_error` and `failure_policy` coexist independently in `RegistryEntry`. All 52 existing registry entries parse without change. `plugin_fail_closed` behavior is unchanged after this story. | unit tests (serde deserialization, 5 test cases: AC-001 through AC-005) + regression (full production hooks-registry.toml parse round-trip) + AC-006 gate (existing `fail_closed_timeout_with_on_error_continue_is_open` passes unmodified) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/registry.rs`; registry parsing and `RegistryEntry` struct definition |
| ADR | ADR-039 §Decision 1 (`on_error` and `failure_policy` as separate, non-unified axes; backward-compat default `fail-open`); ADR-039 §Decision 2 (per-plugin scope; `"fail-closed"` and `"fail-open"` as the two accepted values; absent = `fail-open`); ADR-039 §Decision 3 Phase 1 (schema extension with no enforcement change; safe to ship independently) |
| Stories | S-21.10 |
| Cycle | v1.0-brownfield-backfill (E-21 Wave 5) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-08-16 | product-owner | SR-001 spec-fidelity correction: replaced phantom `HookEntry` struct name with actual `RegistryEntry`; corrected `on_error` field type from `OnError` to `Option<OnError>` in PC5 and Architecture Anchors; fixed struct-name references across Description, postconditions, invariants, edge cases, test vectors, VP anchors, and verification properties. Behavioral semantics (postcondition assertions, invariants, independence of on_error and failure_policy axes) unchanged; description-accuracy fix only. |
| v1.0 | 2026-08-06 | product-owner | Initial creation (S-21.10/S-21.11 BC authoring burst; ADR-039 §Decision 1+2 Phase 1 schema leg; `failure_policy` parse semantics, backward-compat `fail-open` default, unknown-value serde rejection, axes independence in `RegistryEntry`; Phase 1 no-enforcement gate as PC7 RED Gate). |
