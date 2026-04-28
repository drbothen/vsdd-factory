---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "d5ae7e4"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.05.005: hooks-registry.toml registers SessionEnd event routing to hook-plugins/session-end-telemetry.wasm with timeout_ms:5000

## Description

The dispatcher reads `plugins/vsdd-factory/hooks-registry.toml` to map event names to WASM plugin paths. The `SessionEnd` entry in that file MUST exist with `event = "SessionEnd"`, `name = "session-end-telemetry"`, `plugin = "hook-plugins/session-end-telemetry.wasm"`, and `timeout_ms = 5000`. Critically, SessionEnd requires NO capability table declarations: neither `[hooks.capabilities.read_file]` nor `[hooks.capabilities.exec_subprocess]` is needed, because the session-end plugin reads all required data from the incoming envelope (no file reads) and invokes no subprocesses (per BC-4.05.002). Deny-by-default per BC-1.05.022 leaves the plugin sandboxed with zero declared capabilities — the cleanest possible capability profile. This is Layer 2 of the dual-routing-tables pattern per ADR-011. Once-per-session discipline is enforced at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (BC-4.05.004 invariant 1); the `hooks-registry.toml` entry carries no `once` field — `RegistryEntry` has none, and `deny_unknown_fields` would reject it. The `hooks-registry.toml` entry is added by direct edit per the file's header comment ("Human-edited source of truth as of v1.0.0").

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` is the dispatcher-side routing source of truth (per ADR-011 dual-hook-routing-tables, SS-07 ownership).
2. The `hooks-registry.toml` file is syntactically valid TOML and passes dispatcher schema validation (`schema_version = 1`).
3. The `session-end-telemetry.wasm` binary is registered in the dispatcher's plugin registry (or will be loaded from the plugin directory at dispatch time).

## Postconditions

1. `hooks-registry.toml` contains a `[[hooks]]` entry with `event = "SessionEnd"` and `name = "session-end-telemetry"`. The `name` field is required by `RegistryEntry` (registry.rs line 124, no default); omitting it causes a deserialization error at registry-load time.
2. The `SessionEnd` entry specifies `plugin = "hook-plugins/session-end-telemetry.wasm"` (with `hook-plugins/` directory prefix).
3. The `SessionEnd` entry has `timeout_ms = 5000`. Justification: the session-end plugin has no subprocess wait; 5000ms is the `RegistryDefaults` value and is adequate for the stateless emit-only path. Explicitly declaring it removes ambiguity and makes the intent visible. Field name `timeout_ms` per F-13 ruling — `RegistryEntry` in `registry.rs` declares `timeout_ms: Option<u32>` with `deny_unknown_fields`.
4. The `SessionEnd` entry has NO `[hooks.capabilities.read_file]` table — the plugin does not read any files.
5. The `SessionEnd` entry has NO `[hooks.capabilities.exec_subprocess]` table — the plugin invokes no subprocesses (per BC-4.05.002 Invariant 1). The deny-by-default capability sandbox (BC-1.05.022) leaves the plugin with zero declared capabilities, which is intentional and correct for this plugin.
6. Once-per-session discipline for the `SessionEnd` entry is enforced upstream at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (BC-4.05.004 invariant 1). The `hooks-registry.toml` entry does not carry a `once` field — `RegistryEntry` has no such field.
7. Dispatcher successfully loads the entry without error at startup.

## Invariants

1. The `hooks-registry.toml` `SessionEnd` entry must remain registered through all v1.0 releases — removal requires a deprecation pass.
2. Once-per-session discipline is invariant at Layer 1 (Claude Code `once: true` directive in `hooks.json.template`, BC-4.05.004 invariant 1). The `hooks-registry.toml` entry itself does not carry a `once` field — `RegistryEntry` has no such field and `deny_unknown_fields` would reject it.
3. The `plugin` field MUST include the `hook-plugins/` directory prefix: `hook-plugins/session-end-telemetry.wasm`. A path without the prefix causes the dispatcher to fail to locate the binary.
4. NO capability tables SHALL be declared for the `SessionEnd` entry while the plugin reads no files and invokes no subprocesses. Adding capability tables unnecessarily expands the sandbox surface — removal requires a BC update.
5. `name` must be a unique, stable identifier matching the plugin crate name. It is required by `RegistryEntry` (no default); omitting it causes a deserialization error at registry-load time.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `timeout_ms` missing from the entry | Registry load succeeds — `RegistryEntry.timeout_ms` is `Option<u32>` with `#[serde(default)]`; missing values default to `RegistryDefaults.timeout_ms = 5000`. Since 5000ms is also the explicitly declared value, the behavior is identical. However, explicit declaration is required by this BC (Postcondition 3) to maintain visible intent. **Note (VP-066 test failure semantics):** If `timeout_ms` is omitted, registry load succeeds (RegistryDefaults supplies 5000ms, which IS the recommended value for SessionEnd) but VP-066 `test_bc_4_05_005_hooks_registry_toml_has_session_end` will fail at `.expect("timeout_ms must be present and an integer")` since the assertion expects an explicit `timeout_ms` field. Therefore explicit declaration IS required at v1.0 to satisfy VP-066, even though RegistryDefaults would otherwise be acceptable. |
| EC-002 | `plugin` path missing `hook-plugins/` prefix (e.g., `session-end-telemetry.wasm` without directory) | Dispatcher cannot find the binary at the resolved path; plugin load fails at dispatch time; `SessionEnd` event is not processed; `session.ended` never emitted. Operator must fix the `hooks-registry.toml` entry directly. |
| EC-003 | Duplicate `SessionEnd` entries in `hooks-registry.toml` | Both entries are loaded into `Vec<RegistryEntry>`; routing under duplicate-key conditions is presumed to follow `Vec` iteration order (first-entry-wins) per `toml::from_str` semantics for `[[hooks]]` table-arrays. This first-entry-wins behavior describes downstream dispatcher routing, not the parsing itself — `toml::from_str` simply appends each `[[hooks]]` entry in order. This routing-order behavior is plausible based on `toml::from_str` semantics but is not directly verified by VP-066's file-load harness. Duplicate detection is not enforced at load time; routing under duplicates is undefined at v1.0; operator must avoid duplicates manually. v1.1 candidate: registry validation duplicate-detection. **Note:** VP-066 `test_bc_4_05_005_hooks_registry_toml_has_session_end` asserts `session_end_count == 1` (exactly one `SessionEnd` entry) — the test fails if duplicates are present, providing test-time detection. |
| EC-004 | A `[hooks.capabilities.exec_subprocess]` table is accidentally added to the entry | `RegistryEntry` deserializes with the capability declared; plugin gains exec_subprocess access it does not use. This is a misconfiguration, not a crash. VP-066 `test_bc_4_05_005_hooks_registry_toml_has_session_end` asserts NO capability tables are present and would fail at test time. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks-registry.toml` and inspect entries | `SessionEnd` entry present with `event = "SessionEnd"`, `name = "session-end-telemetry"`, `plugin = "hook-plugins/session-end-telemetry.wasm"`, `timeout_ms = 5000`; NO `capabilities` sub-table present | happy-path |
| `hooks-registry.toml` `SessionEnd` entry with `plugin` missing `hook-plugins/` prefix | Dispatcher fails to locate binary; SessionEnd events produce no `session.ended` emissions | error (bad plugin path) |
| `hooks-registry.toml` `SessionEnd` entry with a `[hooks.capabilities.exec_subprocess]` table present | Entry deserializes; plugin gains exec_subprocess capability it should not have; VP-066 assertion `NO capability tables` fails at test time | error (misconfiguration) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-066 | Session-End Plugin Surface Invariant — All BC-4.05.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.05.004** — counterpart (hooks.json.template is the Claude Code-side registration; this is the dispatcher-side registration; both must exist for full routing; BC-4.05.004's `once: true` directive is the upstream once-discipline this entry relies on)
- **BC-4.05.002** — depends on (the absence of `exec_subprocess` capability table in this entry is the deny-by-default gate that enforces BC-4.05.002's no-subprocess invariant)
- **BC-4.05.001** — enables (this routing entry causes the dispatcher to invoke the plugin that emits `session.ended`)
- **BC-4.05.003** — aligns with (idempotency is delegated to Layer 1 once-discipline per BC-4.05.004)
- **BC-4.04.005** — structural analog (SessionStart Layer 2 registration; BC-4.05.005 mirrors it with the key difference: no capability tables needed for SessionEnd)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionEnd routing entry; routing semantics for the session-end-telemetry WASM plugin are SS-04-owned even though the file lives in SS-07 space; per F-8 ruling applied to SessionEnd)
- SS-07 — Hook Bash Layer owns `hooks-registry.toml` as the committed, operator-editable dispatcher routing table (per ARCH-INDEX.md and ADR-011). SS-07 contracts the file-as-text; SS-04 contracts the routing semantics for WASM plugin entries within it.

**Architectural Notes:**

**F-8 Ruling (subsystem ownership, applied to SessionEnd):** This BC retains `subsystem: SS-04` in its frontmatter. Same rationale as BC-4.04.005: BC-4.05.005 contracts the routing semantics of the SessionEnd WASM plugin entry — which plugin is invoked, with which (zero) capabilities, and under which deduplication policy. SS-04 owns these routing semantics.

**No capability tables (SessionEnd-specific):** Unlike BC-4.04.005 which requires both `read_file` and `exec_subprocess` capability tables, BC-4.05.005 requires NEITHER. This is the simplest possible plugin sandbox profile: deny-by-default with zero declared capabilities. The session-end plugin reads all data from the envelope and has no external dependencies.

**F-17 Generator Workflow (inherited):** `hooks-registry.toml` is human-edited source of truth as of v1.0.0. The SessionEnd entry is added by direct edit. The legacy generator script is retired and must not be used. (See BC-4.04.005 §"F-17 Generator Workflow" for the full ruling and historical context.)

## Story Anchor

S-5.02

## VP Anchors

VP-066

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial — by declaring NO capabilities, deny-by-default ensures exec_subprocess and read_file are both denied for this plugin; no audit event is emitted because the plugin never attempts to call them); DI-007 (always-on telemetry — `session.ended` is emitted unconditionally via this routing entry); DI-015 (per-project activation required — this entry is directly added by human edit per the file header; SS-09 generator retired) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionEnd entry added by direct edit) |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |
