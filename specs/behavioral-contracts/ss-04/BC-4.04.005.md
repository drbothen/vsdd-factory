---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "5765182"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2, v1.0-pass-3, v1.0-pass-4, v1.0-pass-5, v1.0-pass-6]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.005: hooks-registry.toml registers SessionStart event routing to hook-plugins/session-start-telemetry.wasm with read_file + exec_subprocess capability tables and timeout_ms:8000

## Description

The dispatcher reads `plugins/vsdd-factory/hooks-registry.toml` (owned by SS-07 Hook Bash Layer) to map event names to WASM plugin paths. The `SessionStart` entry in that file MUST exist with `event = "SessionStart"`, `name = "session-start-telemetry"`, `plugin = "hook-plugins/session-start-telemetry.wasm"`, a `[hooks.capabilities.read_file]` table with `path_allow = [".claude/settings.local.json"]`, a `[hooks.capabilities.exec_subprocess]` table with `binary_allow = ["factory-health"]`, and `timeout_ms = 8000`. The `name` field is required by `RegistryEntry` in `registry.rs` (line 124, no default) and MUST be a stable identifier matching the plugin crate name. This is Layer 2 of the dual-routing-tables pattern per ADR-011 — the dispatcher-side counterpart to BC-4.04.004's `hooks.json.template` entry. Once-per-session discipline is enforced at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (BC-4.04.004 invariant); the dispatcher does not enforce per-event dedup at Layer 2 (BC-1.10.002 retired in pass-4). The `[hooks.capabilities.read_file]` table enables the plugin to read `activated_platform` from `.claude/settings.local.json` via the existing `read_file` host fn. The `[hooks.capabilities.exec_subprocess]` table declaration is required for the factory-health subprocess invocation described in BC-4.04.002. The `hooks-registry.toml` entry is added by **direct edit** per the file's own header comment ("Human-edited source of truth as of v1.0.0. The v0.79.x→v1.0 migration generator has been retired; edit this file directly."). The legacy `generate-registry-from-hooks-json.sh` script is retired and MUST NOT be modified for this purpose.

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` is the dispatcher-side routing source of truth (per ADR-011 dual-hook-routing-tables, SS-07 ownership).
2. The `hooks-registry.toml` file is syntactically valid TOML and passes dispatcher schema validation (`schema_version = 1`).
3. The `session-start-telemetry.wasm` binary is registered in the dispatcher's plugin registry (or will be loaded from the plugin directory at dispatch time).

## Postconditions

1. `hooks-registry.toml` contains a `[[hooks]]` entry (or equivalent TOML structure) with `event = "SessionStart"` and `name = "session-start-telemetry"`. The `name` field is required by `RegistryEntry` (registry.rs line 124, no default); production entries always carry it (e.g., `name = "capture-commit-activity"`).
2. The `SessionStart` entry specifies `plugin = "hook-plugins/session-start-telemetry.wasm"`.
3. Once-per-session discipline for the `SessionStart` entry is enforced upstream at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (BC-4.04.004 invariant 1). The dispatcher does not enforce per-event dedup at Layer 2 for this entry; BC-1.10.002 was retired in pass-4 as over-engineering after confirming that the Layer 1 once-discipline guarantees the dispatcher receives at most one `SessionStart` invocation per session.
4. The `SessionStart` entry declares capability tables in the following form (both tables required):

   ```toml
   [hooks.capabilities.read_file]
   path_allow = [".claude/settings.local.json"]

   [hooks.capabilities.exec_subprocess]
   binary_allow = ["factory-health"]
   ```

   The `read_file` table enables the plugin to read `activated_platform` from `.claude/settings.local.json` via the existing `read_file` host fn (`crates/factory-dispatcher/src/host/read_file.rs`). The `exec_subprocess` table is required for BC-4.04.002 factory-health subprocess invocation. Both table forms match the production schema verified against `crates/factory-dispatcher/src/registry.rs` `Capabilities` struct (`#[serde(deny_unknown_fields)]`).
5. The `SessionStart` entry has `timeout_ms = 8000`. Justification: the plugin's 5000ms subprocess timeout (BC-4.04.002 Invariant 4) plus plugin startup, payload parse, host-fn calls, and `exec_subprocess` setup overhead exceeds the 5000ms global epoch default (CAP-011). The 8000ms value gives 3000ms headroom above the subprocess timeout and prevents `Timeout{Epoch}` from killing the plugin before it can map a subprocess timeout to `factory_health = "unknown"` (per BC-4.04.002 EC-003). Field name `timeout_ms` per F-13 ruling — `RegistryEntry` in `registry.rs` declares `timeout_ms: Option<u32>` with `deny_unknown_fields`; `epoch_budget_ms` would cause a hard registry-load rejection.
6. Dispatcher successfully loads the entry without error at startup.

## Invariants

1. The `hooks-registry.toml` `SessionStart` entry must remain registered through all v1.0 releases — removal requires a deprecation pass.
2. Once-per-session discipline is invariant at Layer 1 (Claude Code `once: true` directive in `hooks.json.template`, BC-4.04.004 invariant 1). The `hooks-registry.toml` entry itself does not carry a `once` field — `RegistryEntry` has no such field and `deny_unknown_fields` would reject it. Once-discipline is entirely a Layer 1 concern.
3. Both capability tables must remain present as long as the respective dependencies exist:
   - `[hooks.capabilities.read_file]` with `path_allow = [".claude/settings.local.json"]` must remain while BC-4.04.001 requires `activated_platform` from that file. Removing it would cause the `read_file` host fn to deny the read with `internal.capability_denied` (per DI-004).
   - `[hooks.capabilities.exec_subprocess]` with `binary_allow = ["factory-health"]` must remain while BC-4.04.002 requires the factory-health subprocess call. Removing it would cause the dispatcher to refuse the subprocess call (per DI-004).
4. `plugin` must stay in sync with the canonical plugin path `hook-plugins/session-start-telemetry.wasm`; any rename propagates to this entry in the same commit.
5. Entries declaring `[hooks.capabilities.exec_subprocess]` MUST declare `timeout_ms` exceeding the longest expected subprocess wait. For the `SessionStart` entry, this means `timeout_ms > 5000` (the subprocess timeout per BC-4.04.002 Invariant 4); `8000` is the required minimum. Field name is `timeout_ms` per `RegistryEntry` schema (F-13 ruling). **Note:** this invariant is operator-enforced and test-verified (VP-065 `test_bc_4_04_005_hooks_registry_toml_has_session_start` asserts `timeout_ms >= 8000`); it is NOT enforced at registry-load time — `RegistryEntry.timeout_ms` is `Option<u32>` and missing values inherit the 5000ms `RegistryDefaults` value rather than triggering a load error. Lifting this class invariant to dispatcher load-time validation is a v1.1 candidate per F-11 process-gap (see Traceability).
6. The `name` field MUST be a unique, stable identifier matching the plugin crate name (e.g., `name = "session-start-telemetry"`). It is required by `RegistryEntry` (no default); omitting it causes a deserialization error at registry-load time. The `name` value must not change without a corresponding deprecation pass. **Note:** `name` uniqueness across all registry entries is operator-enforced in v1.0; `Registry::validate()` does not scan for duplicate names at load time (the validator only checks `schema_version` and per-entry `tool` regex patterns — see `registry.rs::validate()`). Lifting this to a load-time duplicate-name detection error is a v1.1 candidate; see F-11 (timeout_ms) for the same process-gap pattern.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `[hooks.capabilities.exec_subprocess]` table absent from the `SessionStart` entry | Dispatcher refuses to invoke `factory-health` subprocess; emits `internal.capability_denied` per DI-004; plugin sets `factory_health = "unknown"` (fail-open per BC-4.04.002); `session.started` still emitted |
| EC-002 | `SessionStart` entry is duplicated in `hooks-registry.toml` | Duplicate entries are not detected by the loader: `toml::from_str` accepts both; both entries are loaded into `Vec<RegistryEntry>`; the dispatcher routes the event to the first-iteration-order entry. No warning is emitted. Operator must avoid duplicates manually. v1.1 candidate: registry validation duplicate-detection. |
| EC-003 | `hooks-registry.toml` contains a TOML syntax error | Dispatcher fails to start; emits `internal.dispatcher_error` with structured error message including file path and parse error; dispatcher exits non-zero |
| EC-004 | `timeout_ms` field is absent from the `SessionStart` entry that declares `[hooks.capabilities.exec_subprocess]` | Registry load **succeeds** — `RegistryEntry.timeout_ms` is `Option<u32>` with `#[serde(default)]` (registry.rs line 149); missing values default to `RegistryDefaults.timeout_ms = 5000` rather than causing a load error. However, Invariant 5 (`timeout_ms > 5000`) is structurally violated: the 5000ms default equals the subprocess timeout, removing the 3000ms headroom that prevents the epoch budget from terminating the plugin before it can map a subprocess timeout to `factory_health = "unknown"`. VP-065 `test_bc_4_04_005_hooks_registry_toml_has_session_start` asserts `timeout_ms >= 8000` and FAILS at test time when the field is absent. **Operator must set `timeout_ms = 8000` explicitly.** Note: BC-1.01.003 (unknown-field rejection via `deny_unknown_fields`) does NOT apply here — this is a missing-optional-field scenario, not a typo-guard scenario; `deny_unknown_fields` only rejects fields that are present but unrecognized. A load-time validation rule asserting `timeout_ms > subprocess_timeout` for entries declaring `exec_subprocess` capability is a v1.1 candidate per F-11 process-gap. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks-registry.toml` and inspect entries | `SessionStart` entry present with `event = "SessionStart"`, `name = "session-start-telemetry"`, `plugin = "hook-plugins/session-start-telemetry.wasm"`, `[hooks.capabilities.read_file]` table with `path_allow = [".claude/settings.local.json"]`, `[hooks.capabilities.exec_subprocess]` table with `binary_allow = ["factory-health"]` | happy-path |
| `hooks-registry.toml` `SessionStart` entry with `[hooks.capabilities.exec_subprocess]` table absent | Dispatcher refuses factory-health subprocess call; `internal.capability_denied` emitted; `session.started` still emitted with `factory_health = "unknown"` | error (capability denied) |
| `hooks-registry.toml` with duplicate `SessionStart` entries (first valid, second with identical fields) | Dispatcher uses first entry; both entries loaded into `Vec<RegistryEntry>`; no warning emitted (per EC-002 — `toml::from_str` does not scan for duplicates; operator must avoid duplicates manually); `session.started` emitted normally | edge-case (duplicate entry) |
| Parse `hooks-registry.toml` `SessionStart` entry; inspect `timeout_ms` field | `timeout_ms = 8000` present; value is integer ≥ 8000 | happy-path (timeout budget) |
| `hooks-registry.toml` `SessionStart` entry with `[hooks.capabilities.exec_subprocess]` present but `timeout_ms` absent | Registry load **succeeds** (timeout_ms defaults to 5000ms via RegistryDefaults); VP-065 test-time assertion `timeout_ms >= 8000` FAILS — operator must set `timeout_ms = 8000` explicitly (per EC-004; v1.1 candidate for load-time enforcement) | error (test-time assertion failure; not load-time error) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.004** — counterpart (hooks.json.template is the Claude Code-side registration; this is the dispatcher-side registration; both must exist for full routing; BC-4.04.004's `once: true` directive is the upstream once-discipline this entry relies on)
- **BC-4.04.002** — depends on (exec_subprocess capability in this entry enables the factory-health subprocess call)
- **BC-4.04.003** — aligns with (idempotency is delegated to Layer 1 once-discipline per BC-4.04.004)
- **BC-4.04.001** — enables (this routing entry causes the dispatcher to invoke the plugin that emits session.started; read_file capability in this entry enables activated_platform read)
- **BC-1.01.001** — depends on (registry schema validation applies to this toml entry)
- **BC-1.10.001** — retired (pass-4: over-engineering; canonical read_file host fn + path_allow pattern used instead)
- **BC-1.10.002** — retired (pass-4: over-engineering; Layer 1 once-discipline makes dispatcher-side dedup redundant)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionStart routing entry; routing semantics
  for the session-start-telemetry WASM plugin are SS-04-owned even though the file lives in SS-07
  space; see F-8 ruling below)
- SS-07 — Hook Bash Layer owns `hooks-registry.toml` as the committed, operator-editable dispatcher
  routing table (per ARCH-INDEX.md line 98 and ADR-011). SS-07 contracts the file-as-text; SS-04
  contracts the routing semantics for WASM plugin entries within it. These scopes are non-overlapping.
- SS-09 — `plugins/vsdd-factory/skills/activate/SKILL.md` (the activate skill writes `vsdd-factory.activated_platform` to `.claude/settings.local.json`; BC-4.04.001 reads this value via the `read_file` host fn, which this BC's `[hooks.capabilities.read_file]` capability table enables). This is the live SS-09 dependency post-pass-4; the legacy `generate-registry-from-hooks-json.sh` migration script is retired per F-17 and carries no live SS-09 dependency.

**Architectural Notes:**

**F-8 Ruling (subsystem ownership, C1 accepted 2026-04-28):** This BC retains `subsystem: SS-04`
in its frontmatter. Rationale: BC-4.04.005 contracts the *routing semantics* of the SessionStart
WASM plugin entry — which plugin is invoked, with which capabilities, and under which
deduplication policy. SS-04 owns these routing semantics. SS-07 owns `hooks-registry.toml` as
a file-level artifact (schema validation, TOML format, registry loading). The two ownership
claims are at different abstraction layers and are mutually consistent. No renumbering required.

**F-13 Schema Requirement (field name correction, REVISED in pass-3 2026-04-28):** The SessionStart
entry in `hooks-registry.toml` MUST include a `timeout_ms` override of `8000` (NOT `epoch_budget_ms`).

Pass-3 code verification finding: `RegistryEntry` struct in `crates/factory-dispatcher/src/registry.rs`
declares `timeout_ms: Option<u32>` as the per-call wall-clock budget field (line 149). The struct
carries `#[serde(deny_unknown_fields)]` (line 120), so any entry containing `epoch_budget_ms` will
cause `BC-1.01.003` typo-guard rejection and kill the entire registry load. The field name
`epoch_budget_ms` does not exist anywhere in the `RegistryEntry`, `RegistryDefaults`, or
`Capabilities` structs.

**Correct field name: `timeout_ms = 8000`** — this IS the epoch/wall-clock budget for the entry.
The semantics are identical to the intended `epoch_budget_ms` intent: it is the per-call wall-clock
budget that must exceed the subprocess timeout (5000ms) plus overhead. The value `8000` (3000ms
headroom above the subprocess timeout) is unchanged; only the field name is corrected.

All references in this BC (Postcondition 5, Invariant 5, EC-004, Canonical Test Vectors) have been
updated to use `timeout_ms = 8000` (PO body-content update applied in pass-3). The architectural
ruling is: **use `timeout_ms = 8000`**.

**F-17 Generator Workflow (verified 2026-04-28):** `hooks-registry.toml` is **generated-once,
then committed and directly edited** (option b). Evidence: (1) The file header reads "Human-edited
source of truth as of v1.0.0. The v0.79.x→v1.0 migration generator has been retired; edit this
file directly." (2) The generator script (`generate-registry-from-hooks-json.sh`) reads from the
historical git commit `7b4b774^` (v0.79.x hooks.json), not from the current file state — it is a
one-time migration tool, not a continuous code generator. (3) The SessionStart entry for the
session-start-telemetry plugin does NOT appear in the historical v0.79.x hooks.json (which only
covered bash hooks); therefore the generator would never emit it. Story S-5.01 Task 6 must instruct
the implementer to **directly add** the SessionStart `[[hooks]]` entry to `hooks-registry.toml`
rather than updating the generator script. The generator is retired and must not be modified for
this purpose.

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — absent `exec_subprocess` capability table triggers `internal.capability_denied`); DI-007 (always-on telemetry — `session.started` is emitted unconditionally even when subprocess capability is denied or subprocess fails); DI-015 (per-project activation required — this entry is directly added by human edit per the file header; SS-09 generator retired) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionStart entry added by direct edit; SS-09 generator retired as of v1.0.0) |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
| Process Gap (F-11, v1.1 candidate) | The class invariant that `timeout_ms` must exceed the longest expected subprocess wait (Invariant 5) is not enforced by the dispatcher schema at registry-load time; EC-004 documents what happens when `timeout_ms` is absent, but there is no runtime check that `timeout_ms > subprocess_timeout`. A registry-load validation rule asserting `timeout_ms > 5000` for entries declaring `exec_subprocess` capability would codify this invariant in code. Deferred to v1.1. |
