---
document_type: behavioral-contract
level: L3
version: "1.5"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1b
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - .factory/specs/architecture/SS-03-event-emission.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: greenfield
subsystem: "SS-03"
capability: "CAP-029"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-3.05.004: factory-dispatcher::observability_config::v2_schema_validation — observability-config.toml v2 schema validated at load time; schema_version=1 hard-errors with migration hint; two-key debug-stream gate (VSDD_DEBUG_LOG=1 env var dominates; debug_log_enabled config key governs when env var absent)

## Description

ADR-015 D-15.1 retires multi-sink stanzas; OQ-1 (resolved in
SS-03-event-emission.md) defines the v2 `observability-config.toml` schema.
The multi-sink stanza model of the v1 schema is removed. The v2 schema
configures only the single `FileSink` path, retention policy, debug-stream gate,
and fsync behavior. Operators who need remote OTel export configure the OTel
Collector externally.

This BC specifies the schema validation contract at load time: which fields are
valid, what `schema_version` values are accepted and which hard-error, what the
two-key debug-stream gate semantics are (incorporating OQ-W16-011 resolution,
D-311 2026-05-06), and how unknown keys are handled.

The schema definition is owned by SS-03. SS-01 consumes the parsed config; it
does not own the schema.

This BC supersedes the multi-sink config validation behavior described in the
retired BC-3.05.001/002/003 (marked `lifecycle_status: retired`,
`superseded_by: ADR-015` in D-312 corrigendum 2026-05-06).

## Preconditions

1. The dispatcher process is starting up.
2. `observability-config.toml` is present in the expected location (or the
   dispatcher uses built-in defaults if the file is absent — see EC-001).
3. The file is readable by the dispatcher process (not permissions-denied).

## Postconditions

### v2 schema validation

1. A v2 `observability-config.toml` file with `schema_version = 2` MUST be
   parsed successfully if it contains only valid v2 fields. The parsed config
   is used for the dispatcher invocation.
2. The following fields are valid in a v2 `observability-config.toml`:

   | Field | Type | Default | Description |
   |-------|------|---------|-------------|
   | `schema_version` | integer | (required) | MUST equal `2`; see Postcondition 4 |
   | `events_file` | string | `".factory/logs/events-{date}.jsonl"` | Path template; supports `{date}` (YYYY-MM-DD) and `{project}` (basename of `CLAUDE_PROJECT_DIR`) |
   | `retention_days` | integer | `90` | Retention policy for `events-*.jsonl` files (days) |
   | `debug_log_retention_days` | integer | `30` | Retention policy for `dispatcher-internal-*.jsonl` files (days) |
   | `debug_log_enabled` | boolean | `false` | Config-key gate for the debug stream; see Postcondition 5 for two-key gate semantics |
   | `sync_on_write` | boolean | `false` | When `true`, `fsync` called after each `FileSink::write`; durability vs performance trade-off |

3. The multi-sink stanza model (e.g., `[[sinks]]` blocks, `[sink.file]`,
   `[sink.otel-grpc]`) that was valid in the v1 schema is NO LONGER VALID in
   the v2 schema. A v2 config containing these stanzas MUST warn-and-skip the
   unknown keys (see Postcondition 7).

### schema_version hard-error

4. A config file with `schema_version = 1` (or any integer less than 2) MUST
   hard-error at load time. The dispatcher MUST:
   a. Log a structured error to stderr of the form:
      `[vsdd-dispatcher] ERROR: observability-config.toml has schema_version=1
      (v1 schema is no longer supported). Remove all [[sinks]] stanzas and set
      schema_version=2. See ADR-015 migration guide.`
   b. Exit with a non-zero exit code (the specific code is governed by the
      dispatcher's existing config-error exit semantics).
   A v1 schema file MUST NOT cause the dispatcher to silently continue with
   default config. The hard-error forces explicit operator migration.

### Two-key debug-stream gate (OQ-W16-011 resolved D-311 2026-05-06)

5. The debug stream (`dispatcher-internal-*.jsonl` voluntary writes) is
   controlled by a two-key gate with the following precedence semantics:

   **Rule (verbatim from OQ-W16-011 resolution):**

   > When `VSDD_DEBUG_LOG=1` is set in the dispatcher process environment, the
   > debug stream is enabled regardless of the `debug_log_enabled` config key
   > value. When `VSDD_DEBUG_LOG` is absent from the environment, the
   > `debug_log_enabled` config key governs: `debug_log_enabled = true` enables
   > the debug stream; `debug_log_enabled = false` (the default) disables it.
   > Any non-`"1"` value for `VSDD_DEBUG_LOG` (including `"0"`, `"true"`,
   > `"false"`) does NOT enable the stream via the env var path — the config key
   > governs in that case as if the env var were unset.

   This is the 12-factor override pattern: env var is the runtime override;
   config key is the static operator-controlled default. The env var ALWAYS
   dominates when present (set to exactly `"1"`).

   **Authoritative sources for this rule:**
   - `SS-03-event-emission.md` § `observability-config.toml` Schema: "The
     `VSDD_DEBUG_LOG=1` environment variable ALWAYS overrides `debug_log_enabled`;
     the env var takes precedence."
   - `SS-03-event-emission.md` Cross-Cutting: "The debug file is active only when
     `VSDD_DEBUG_LOG=1` is set (or `debug_log_enabled = true` in config)."
   - OQ-W16-011 resolution (open-questions.md § OQ-W16-011): "Option chosen: (c) —
     12-factor override semantics: env var dominates when present; config key
     governs when env var absent."

6. The gate is evaluated at the write site (not at startup). The
   `debug_log_enabled` value is read from the loaded config at startup and held
   for the process lifetime. `VSDD_DEBUG_LOG` is read from the process
   environment (stable for process lifetime).

### Unknown key handling

7. Unknown top-level keys in a v2 `observability-config.toml` MUST be warned
   and skipped (not hard-errored). A warning of the form:
   `[vsdd-dispatcher] WARN: Unknown key in observability-config.toml: '<key>'.
   Ignoring.` is emitted to stderr.
   This is graceful degradation, consistent with the behavior for unknown sink
   types in v1. Operators who have stale multi-sink stanzas in a config file
   after upgrading to v2 will see warnings but the dispatcher will not crash.

### File-absent default behavior

8. If `observability-config.toml` is absent from its expected location, the
   dispatcher MUST use the built-in defaults for all fields (per Postcondition 2
   defaults column). No error or warning is emitted for a missing config file.
   The dispatcher operates correctly without a config file.

## Invariants

1. `schema_version = 2` is the only accepted version. Any other integer
   (including 1) hard-errors with a migration hint.
2. **The `schema_version` domain is `{1, 2, >2}` with explicit behavior for each
   partition:**
   - `schema_version = 1` → hard-error with v1→v2 migration hint (message format:
     `[vsdd-dispatcher] ERROR: observability-config.toml has schema_version=1 (v1
     schema is no longer supported). Remove all [[sinks]] stanzas and set
     schema_version=2. See ADR-015 migration guide.`; exit code 1)
   - `schema_version = 2` → accepted; dispatcher loads config and operates normally
   - `schema_version > 2` (any future version) → hard-error with unknown-future-version
     message (message format: `[vsdd-dispatcher] ERROR: unknown future schema version
     <N>; this dispatcher build accepts schema_version = 2 only.`; exit code 1)
   No schema_version value outside `{2}` results in silent acceptance or
   warn-and-use-defaults behavior. The hard-error on both `< 2` and `> 2` is
   intentional and consistent with DI-014 (schema version mismatch is a hard load error).
3. The `debug_log_enabled` config key and `VSDD_DEBUG_LOG` env var are the ONLY
   two mechanisms to enable the voluntary debug stream. No other config key
   or env var activates it.
4. `VSDD_DEBUG_LOG=1` (exact string `"1"`) is the env var activation value.
   Any other string (`"true"`, `"yes"`, `"0"`, `"false"`) does NOT activate
   the env var path. The config key governs for any non-`"1"` env var value.
5. The write-failure fallback path to `dispatcher-internal-*.jsonl` (BC-1.11.002)
   is NEVER gated by `debug_log_enabled` or `VSDD_DEBUG_LOG`. Fallback writes are
   unconditional regardless of these gates. The two-key gate governs only the
   voluntary debug-supplementary write path.
6. The multi-sink `[[sinks]]` stanza model is fully retired in the v2 schema.
   No v2 config file should contain sink-type declarations; such keys are warned
   and skipped.

## Related BCs

- BC-1.12.002 — `VSDD_DEBUG_LOG` gate for `dispatcher-internal-*.jsonl` (this
  BC defines the CONFIG SCHEMA; BC-1.12.002 defines the runtime gate BEHAVIOR.
  BC-3.05.004 is the schema contract; BC-1.12.002 is the runtime contract. Both
  must be consistent: the two-key gate semantics are the same in both.)
- BC-3.05.001 — **RETIRED** (lifecycle_status: retired; superseded_by: ADR-015;
  described v1 `load_builds_file_sink_from_parsed_config` behavior; DO NOT USE
  as a behavioral reference)
- BC-3.05.002 — **RETIRED** (lifecycle_status: retired; superseded_by: ADR-015)
- BC-3.05.003 — **RETIRED** (lifecycle_status: retired; superseded_by: ADR-015)
- BC-1.12.007 — Wave 1 call-graph invariant (cross-cutting: BC-1.12.007 is the runtime call-graph enforcement that no production code path reaches Router/SinkRegistry/DlqWriter; this BC is the static config-schema enforcement that observability-config.toml v2 has no `[[sinks]]` stanzas. Together they ensure ADR-015 D-15.1 multi-sink retirement holds at both compile-time/config-time and runtime — orthogonal enforcement surfaces.)

## Architecture Anchors

- `SS-03-event-emission.md` § `observability-config.toml` Schema (OQ-1 resolution)
  — the authoritative schema definition including the TOML sample and two-key
  gate prose
- `SS-03-event-emission.md` Cross-Cutting — "The debug file is active only when
  `VSDD_DEBUG_LOG=1` is set (or `debug_log_enabled = true` in config)."
- `crates/factory-dispatcher/src/sinks/mod.rs` — config load path; `ObservabilityConfig`
  struct must be updated from v1 (schema_version + sinks fields) to v2 fields
  (schema_version, events_file, retention_days, debug_log_retention_days,
  debug_log_enabled, sync_on_write). [Stable anchor per TD-VSDD-091; line numbers
  are not authoritative — use the struct/module name `ObservabilityConfig` as the
  canonical reference.]
- ADR-015 D-15.1 — normative prose for debug stream being "gated by the
  `VSDD_DEBUG_LOG=1` environment variable"
- ADR-015 OQ-1 (resolved in SS-03-event-emission.md) — v2 schema definition
- open-questions.md § OQ-W16-011 — 12-factor override resolution text

## Story Anchor

S-10.02 (Wave 1: FileSink single-stream wiring; includes `observability-config.toml`
v2 schema loading and debug-stream gate implementation)

## VP Anchors

(TBD — to be assigned after S-10.02 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `observability-config.toml` is absent | Dispatcher uses built-in defaults: `events_file = ".factory/logs/events-{date}.jsonl"`, `retention_days = 90`, `debug_log_retention_days = 30`, `debug_log_enabled = false`, `sync_on_write = false`; no error |
| EC-002 | `schema_version = 2`; all fields at defaults | Parses successfully; dispatcher operates normally |
| EC-003 | `schema_version = 1` (v1 schema file) | Hard-error: stderr error message with migration hint; non-zero exit; no silent fallback to defaults |
| EC-004 | `schema_version = 3` (future version, not yet known) | Hard-errors with message: `[vsdd-dispatcher] ERROR: unknown future schema version 3; this dispatcher build accepts schema_version = 2 only.` Exit code 1. A future schema version must NOT be silently accepted or warn-and-defaulted — hard-error forces operators to upgrade the dispatcher binary before using a newer config format. (Resolved: option (a) hard-error for all non-2 versions, consistent with Invariant 1.) |
| EC-005 | `debug_log_enabled = true` in config; `VSDD_DEBUG_LOG` not set | Debug stream ENABLED (config key governs; env var absent); debug file written |
| EC-006 | `debug_log_enabled = false` in config; `VSDD_DEBUG_LOG=1` in env | Debug stream ENABLED (env var dominates regardless of config key value); `debug_log_enabled = false` does not suppress |
| EC-007 | `debug_log_enabled = true` in config; `VSDD_DEBUG_LOG=0` in env | `VSDD_DEBUG_LOG=0` is NOT the activation value (only `"1"` activates via env); config key governs; debug stream ENABLED (via `debug_log_enabled = true`) |
| EC-008 | `debug_log_enabled = false` in config; `VSDD_DEBUG_LOG=true` (not "1") | Env var is non-`"1"`; config key governs; debug stream DISABLED |
| EC-009 | Config contains a `[[sinks]]` stanza (old v1 multi-sink block); `schema_version = 2` | Unknown key warning per Postcondition 7; stanza is skipped; dispatcher uses v2 defaults for the unrecognized section |
| EC-010 | `sync_on_write = true` in config | `FileSink` calls `fsync` after each write; durability trade-off accepted; no error |
| EC-011 | `events_file = ".factory/logs/events-{project}-{date}.jsonl"` | Template expanded with both `{project}` and `{date}` substitutions; valid path used |
| EC-012 | Operator upgrades from v1 dispatcher to v2 without updating config | `schema_version = 1` triggers hard-error with migration hint (Postcondition 4). Operator MUST update `observability-config.toml` to v2 format to proceed. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `observability-config.toml` absent | Dispatcher loads with all built-in defaults; no error | absent-config-defaults |
| `schema_version = 2`; all fields at defaults | Config parsed; `events_file` resolves to `.factory/logs/events-YYYY-MM-DD.jsonl`; debug stream off by default | v2-default-parse |
| `schema_version = 1` | Hard-error on stderr; non-zero exit; stderr matches regex `\[vsdd-dispatcher\] ERROR: observability-config\.toml has schema_version=1` (substring from Postcondition 4(a) / Invariant 2); migration hint substring `Remove all \[\[sinks\]\] stanzas and set schema_version=2` present | v1-hard-error |
| `schema_version = 3` (future version) | Hard-error on stderr; exit code 1; no silent accept or warn-and-default; stderr matches regex `\[vsdd-dispatcher\] ERROR: unknown future schema version 3; this dispatcher build accepts schema_version = 2 only` (substring from Invariant 2 / EC-004 message format) | future-version-hard-error |
| `debug_log_enabled = true`; `VSDD_DEBUG_LOG` unset | Debug stream active; events written to `dispatcher-internal-*.jsonl` | config-key-enables-debug |
| `debug_log_enabled = false`; `VSDD_DEBUG_LOG=1` | Debug stream active (env var dominates) | env-var-overrides-config-false |
| `debug_log_enabled = true`; `VSDD_DEBUG_LOG=0` | Debug stream active (config key governs; `"0"` is not the activation value) | env-var-non-1-config-governs |
| `debug_log_enabled = false`; `VSDD_DEBUG_LOG=true` | Debug stream INACTIVE (`"true"` is not `"1"`; config governs; config says false) | non-canonical-truthy-string-inactive |
| **Misimplementation distinguisher:** `schema_version = 1` silently falls back to defaults | Test MUST assert a non-zero exit code and stderr error. A misimplementation that silently accepts v1 allows operators to run with stale config undetected. | misimplementation-witness-v1-silent-accept |
| Config with `[[sinks]]` stanza and `schema_version = 2` | Dispatcher warns on unknown key; skips stanza; operates with v2 defaults | unknown-key-warn-skip |
| `sync_on_write = true` | `fsync` called after each FileSink write; observable via test that injects a FileSink mock | sync-on-write |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | `schema_version = 1` produces hard-error exit | unit test: load v1 config; assert non-zero exit + stderr message |
| (TBD) | `VSDD_DEBUG_LOG=1` enables debug stream regardless of `debug_log_enabled` | integration test: set env var + `debug_log_enabled = false`; assert debug file written |
| (TBD) | `debug_log_enabled = true` with no env var enables debug stream | integration test: set config key + no env var; assert debug file written |
| (TBD) | Absent config uses all built-in defaults | unit test: load with no config file; assert all fields equal built-in defaults |
| (TBD) | Unknown keys warn and are skipped | unit test: config with unknown key `foo = "bar"`; assert warning on stderr; no crash |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. This BC specifies the `observability-config.toml` v2 schema — the operator's configuration surface for the single `events-*.jsonl` stream that CAP-029 defines. The schema governs the events stream file path (`events_file`), retention policy (`retention_days`), fsync behavior (`sync_on_write`), and the two-key debug-stream gate. All of these are direct configuration parameters for the CAP-029 single-stream architecture; without a correctly validated v2 schema, the FileSink write path cannot be safely initialized. |
| L2 Domain Invariants | DI-014 (schema version mismatch is a hard load error — extended by D-314 to the v1→v2 transition: `schema_version = 1` hard-errors with a migration hint; `schema_version > 2` hard-errors with `unknown future schema version` message; DI-014's spirit — "never silently process a mismatched schema" — is preserved and extended to the full `{1, >2}` partition by Invariant 2 of this BC) |
| Architecture Module | SS-03 — `crates/factory-dispatcher/src/sinks/mod.rs` (`ObservabilityConfig` struct v2 definition; schema_version validation; warn-and-skip for unknown keys) |
| Stories | S-10.02 (Wave 1: FileSink wiring + v2 config schema loading + debug-stream gate implementation) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.1 (multi-sink stanza removal; debug stream gated by `VSDD_DEBUG_LOG=1`); ADR-015 OQ-1 (resolved in SS-03-event-emission.md: v2 schema definition) |
| OQ Resolved | OQ-W16-011 (D-311 2026-05-06: 12-factor override semantics — env var dominates when present; config key governs when env var absent) |
| Supersedes | BC-3.05.001/002/003 (all `lifecycle_status: retired`, `superseded_by: ADR-015`, retired D-312 2026-05-06) |

**Secondary Capability Reference: CAP-010 ("Always-on dispatcher self-telemetry independent of sink config") per capabilities.md §CAP-010.** BC-3.05.004 governs the `observability-config.toml` v2 schema, which configures BOTH the events stream (via `events_file`, `retention_days`, `sync_on_write`) AND the debug stream gate (via `debug_log_enabled`). The debug stream is the always-on self-telemetry described by CAP-010. The schema is therefore a dual-key gate: it configures the CAP-029 events stream path AND the CAP-010 debug-stream opt-in. This dual coverage is why BC-3.05.004 requires both capability references: CAP-029 for the events stream config (primary) and CAP-010 for the debug stream config (secondary). See also OQ-W16-011 RESOLVED at D-311 for the 12-factor override decision that governs the two-key gate semantics.

**Cross-reference: BC-1.12.002 (runtime gate contract).** BC-3.05.004 is the compile-time schema contract (validates `observability-config.toml` v2 at load time); BC-1.12.002 is the runtime contract (debug stream gate evaluated at each write site). The two-key gate semantics — `VSDD_DEBUG_LOG=1` env var dominates when present; `debug_log_enabled` config key governs when env var absent — are identical in both BCs by design. See OQ-W16-011 RESOLVED at D-311 2026-05-06 for the 12-factor override decision that established this precedence.

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — config file read at startup (TOML parse) |
| Global state access | YES — reads `VSDD_DEBUG_LOG` from process environment; produces `ObservabilityConfig` struct held for process lifetime |
| Deterministic | YES given fixed config file content and fixed env vars |
| Thread safety | YES — config loaded once at startup; result is immutable thereafter |
| Overall classification | Effectful shell (file parse + env var read; deterministic output for given inputs) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Config load source-walk:

- `schema_version` check: MUST use explicit `if schema_version != 2 { hard_error(); }`.
  Using `let _ = schema_version` or matching without a catch-all would silently
  accept a v1 config. Postcondition 4 requires a HARD ERROR for schema_version < 2.
- Unknown key warn-and-skip: the implementation MUST log the unknown key before
  skipping. A `let _ = unknown_key` pattern that silently discards without warning
  violates Postcondition 7.
- Two-key gate precedence: the gate logic MUST check `VSDD_DEBUG_LOG == "1"` first
  and short-circuit to `enabled = true`. The second check (`debug_log_enabled`)
  is ONLY reached when `VSDD_DEBUG_LOG` is absent or non-`"1"`. A reversed
  precedence (config check first) would produce incorrect behavior when
  `VSDD_DEBUG_LOG=1` + `debug_log_enabled = false`.

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-06 | Initial authoring (D-313 Phase 1b). BC-3.05.004 is the corrected ID after D-312 corrigendum found BC-3.05.001 was a pre-existing brownfield BC. Two-key gate semantics incorporate OQ-W16-011 resolution (D-311). Supersedes retired BC-3.05.001/002/003. |
| v1.1 | 2026-05-06 | D-315 F-2/F-11/F-18/F-4 BC-side. Re-anchored to CAP-029 (primary). EC-004 resolved: schema_version>2 → hard-error (option a, consistent with Invariant 1). Invariant 2 added: schema_version domain `{1, 2, >2}` with explicit behavior per partition. Canonical test vector for schema_version=3 added. Secondary Capability Reference (CAP-010) and BC-1.12.002 cross-reference paragraph added (F-18). L2 Domain Invariants populated: DI-014 (hard-error on mismatch, extended to v2). |
| v1.2 | 2026-05-06 | D-319 — F-10 fix: Canonical Test Vectors strictness aligned with Postcondition 4(a) + Invariant 2 exact message texts using regex-substring form. schema_version=1 CTV row now asserts stderr matches `\[vsdd-dispatcher\] ERROR: observability-config\.toml has schema_version=1` (from PC4(a)). schema_version=3 CTV row now asserts stderr matches `\[vsdd-dispatcher\] ERROR: unknown future schema version 3; this dispatcher build accepts schema_version = 2 only` (from Invariant 2 / EC-004). |
| v1.3 | 2026-05-06 | D-322 — F-10 fix: BC-1.12.007 Related BCs entry corrected — false "sibling" claim replaced with accurate "cross-cutting orthogonal enforcement surfaces" description (call-graph runtime enforcement vs config-schema static enforcement; together enforce ADR-015 D-15.1 multi-sink retirement). |
| v1.4 | 2026-05-06 | D-325 — F-7 sweep: L2 Capability cell paraphrase removed — cell now just `CAP-029`. F-14 sweep: stable-anchor disclaimer added to `crates/factory-dispatcher/src/sinks/mod.rs` Architecture Anchor (struct `ObservabilityConfig` is the canonical reference). |
| v1.5 | 2026-05-06 | D-328 — E-10 pass-5 F-2 fix: Description line 33 corrected — observability-config.toml v2 schema is defined by ADR-015 D-15.1 (multi-sink retirement) + OQ-1 (resolved in SS-03-event-emission.md), not D-15.4 (which governs VSDD_TRACE_ID/VSDD_PARENT_SPAN_ID trace propagation, unrelated to schema). Architecture Anchors (~line 192) and Traceability ADR row (~line 270) were already correct (D-15.1 + OQ-1). ARCH-INDEX line 83 same misattribution to be fixed in D-331 state-manager seal. |
