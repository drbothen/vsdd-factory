# Pass 3 — Deep Rust Tests: per-test BC extraction

**Date:** 2026-04-25
**Scope:** Every `#[test]` and `#[tokio::test]` (incl. `#[tokio::test(flavor = "current_thread")]`) under `crates/`.
**Goal:** Extract NEW per-instance BCs not already covered by BC-AUDIT-001..143.
**BC numbering reservation:** BC-AUDIT-2300..2799.

## 1. Round metadata

### Independent test recount

```
find crates -name '*.rs' -type f -exec awk '/^[[:space:]]*#\[(tokio::)?test/ {c++} END {print c+0}' {} + | awk '{s+=$1} END {print s}'
→ 185
```

The broad-sweep claim of **180** Rust tests was off by **5** because the regex `#\[(tokio::)?test\]` (with the literal `]` close-bracket) misses the 5 `#[tokio::test(flavor = "current_thread")]` annotations in `tests/executor_integration.rs`. The corrected total is **185**.

### Coverage by file (recounted with the corrected regex)

| File | Tests |
|---|---|
| `crates/sink-file/src/lib.rs` | 17 |
| `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` | 14 |
| `crates/sink-otel-grpc/src/lib.rs` | 13 |
| `crates/factory-dispatcher/src/registry.rs` | 13 |
| `crates/sink-core/src/lib.rs` | 11 |
| `crates/factory-dispatcher/src/host/exec_subprocess.rs` | 11 |
| `crates/factory-dispatcher/src/routing.rs` | 9 |
| `crates/factory-dispatcher/src/payload.rs` | 8 |
| `crates/factory-dispatcher/src/internal_log.rs` | 8 |
| `crates/factory-dispatcher/src/host/emit_event.rs` | 7 |
| `crates/hook-sdk/src/payload.rs` | 6 |
| `crates/hook-sdk/src/host.rs` | 6 |
| `crates/hook-sdk/src/result.rs` | 5 |
| `crates/factory-dispatcher/tests/executor_integration.rs` | 5 (`#[tokio::test(flavor = "current_thread")]`) |
| `crates/factory-dispatcher/src/sinks/mod.rs` | 5 |
| `crates/factory-dispatcher/src/invoke.rs` | 5 |
| `crates/factory-dispatcher/src/host/read_file.rs` | 5 |
| `crates/factory-dispatcher/tests/sinks_otel_grpc.rs` | 4 |
| `crates/factory-dispatcher/tests/routing_integration.rs` | 4 |
| `crates/factory-dispatcher/src/plugin_loader.rs` | 4 |
| `crates/factory-dispatcher/src/executor.rs` | 4 |
| `crates/factory-dispatcher/src/engine.rs` | 4 |
| `crates/hook-sdk/src/__internal.rs` | 3 |
| `crates/factory-dispatcher/tests/loads_legacy_registry.rs` | 3 |
| `crates/factory-dispatcher/tests/internal_log_integration.rs` | 2 |
| `crates/factory-dispatcher/tests/host_functions.rs` | 2 |
| `crates/factory-dispatcher/src/host/log.rs` | 2 |
| `crates/factory-dispatcher/src/host/env.rs` | 2 |
| `crates/hook-plugins/capture-commit-activity/src/lib.rs` | 1 |
| `crates/factory-dispatcher/tests/sinks_file_integration.rs` | 1 |
| `crates/factory-dispatcher/src/sinks/router.rs` | 1 |
| **TOTAL** | **185** |

`crates/hook-sdk-macros/src/lib.rs` has no inline `#[test]` declarations. The Pass 0 inventory's "13 trybuild tests" claim is **NOT** reproducible from source — there is no `tests/` directory under `hook-sdk-macros/` and no `trybuild` invocation in the only file present (`src/lib.rs`). Flagged as a Pass 0 inventory drift but irrelevant for this round (no extra tests to mine).

### Coverage map — per crate / per module

| Crate / module | Tests | Already covered by BC-AUDIT-001..143 | NEW BCs added (BC-AUDIT-2300..) |
|---|---|---|---|
| `factory-dispatcher::registry` | 13 | 9 (BC-AUDIT-001..004 plus 5 derivable from existing) | 5 new |
| `factory-dispatcher::routing` | 9 | 8 (BC-AUDIT-005, -006) | 1 new |
| `factory-dispatcher::payload` | 8 | 7 (BC-AUDIT-007, -008) | 1 new |
| `factory-dispatcher::engine` | 4 | 4 (BC-AUDIT-020..022) | 0 |
| `factory-dispatcher::invoke` | 5 | 5 (BC-AUDIT-009..012) | 0 |
| `factory-dispatcher::executor` | 4 | 4 (BC-AUDIT-017) | 0 |
| `factory-dispatcher::plugin_loader` | 4 | 4 (BC-AUDIT-119..122) | 0 |
| `factory-dispatcher::internal_log` | 8 | 6 (BC-AUDIT-035..040) | 2 new |
| `factory-dispatcher::host::emit_event` | 7 | 1 (BC-AUDIT-034) | 6 new |
| `factory-dispatcher::host::env` | 2 | 2 (BC-AUDIT-029, -030) | 0 |
| `factory-dispatcher::host::log` | 2 | 1 (BC-AUDIT-033) | 1 new |
| `factory-dispatcher::host::read_file` | 5 | 1 (BC-AUDIT-031 stub-only flagged) | 5 new |
| `factory-dispatcher::host::exec_subprocess` | 11 | 6 (BC-AUDIT-023..028) | 5 new |
| `factory-dispatcher::sinks::mod` | 5 | 4 (BC-AUDIT-041..043, -049) | 1 new |
| `factory-dispatcher::sinks::router` | 1 | 1 (BC-AUDIT-123) | 0 |
| `tests/executor_integration.rs` | 5 | 2 (BC-AUDIT-015, -016 partial) | 4 new |
| `tests/host_functions.rs` | 2 | 0 | 2 new |
| `tests/internal_log_integration.rs` | 2 | 1 (BC-AUDIT-035) | 1 new |
| `tests/loads_legacy_registry.rs` | 3 | 1 (BC-AUDIT-061) | 2 new |
| `tests/routing_integration.rs` | 4 | 4 (BC-AUDIT-005, -006) | 0 |
| `tests/sinks_file_integration.rs` | 1 | 1 (BC-AUDIT-046, -047) | 1 new |
| `tests/sinks_otel_grpc.rs` | 4 | 3 (BC-AUDIT-137..143) | 1 new |
| `sink-core::lib` | 11 | 4 (BC-AUDIT-044, -045) | 6 new |
| `sink-file::lib` | 17 | 6 (BC-AUDIT-046..048) | 11 new |
| `sink-otel-grpc::lib` | 13 | 9 (BC-AUDIT-137..143, partial) | 5 new |
| `hook-sdk::result` | 5 | 5 (BC-AUDIT-050, -051) | 0 |
| `hook-sdk::payload` | 6 | 1 (BC-AUDIT-053) | 5 new |
| `hook-sdk::host` | 6 | 1 (BC-AUDIT-127, -128) | 4 new |
| `hook-sdk::__internal` | 3 | 0 | 3 new |
| `hook-plugins::legacy-bash-adapter::lib` | 14 | 12 (BC-AUDIT-054..058, BC-AUDIT-132..135) | 2 new |
| `hook-plugins::capture-commit-activity::lib` | 1 | 0 | 1 new |
| **TOTAL** | **185** | **~125** | **~75** |

The estimate is tight: **~68% of tests** are already covered by BC-AUDIT-001..143; **~32% (60+ tests)** carry distinct behavioral claims not yet pinned. New BCs follow.

## 2. NEW BCs by crate (alphabetical)

### 2.1 — factory-dispatcher

#### BC-AUDIT-2300 — factory-dispatcher::registry::parses_minimal_registry: minimum-viable registry parses with one hook entry, schema_version=1, enabled defaults to true

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::parses_minimal_registry`
**Source line(s):** 324–333
**Confidence:** HIGH (tests are direct contracts)
**Test type:** unit
**Given:** TOML with `schema_version = 1` and a single `[[hooks]]` stanza declaring `name`, `event`, `tool`, `plugin`.
**When:** `Registry::parse_str(toml)` is called.
**Then:** Returns `Ok(Registry)` with `schema_version=1`, `hooks.len()==1`, the entry's `name`, `event`, `tool`, and `enabled=true` (default).
**Acceptance:** Test passes when invariant holds; fails CI if regressed.

#### BC-AUDIT-2301 — factory-dispatcher::registry::config_defaults_to_empty_table_when_absent: missing [hooks.config] yields empty table, not None

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::config_defaults_to_empty_table_when_absent`
**Source line(s):** 335–343
**Confidence:** HIGH
**Test type:** unit
**Given:** Registry TOML with a hook entry that has no `[hooks.config]` block.
**When:** Entry's `config` field is inspected; `entry.config_as_json()` is called.
**Then:** `config.is_table()` is true, `config.as_table().unwrap().len() == 0`; `config_as_json()` returns an empty JSON object (not null, not absent).
**Acceptance:** Plugin-config-aware code paths can rely on a non-null empty table for entries without config.

#### BC-AUDIT-2302 — factory-dispatcher::registry::config_block_parses_into_entry: [hooks.config] supports nested tables and string fields

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::config_block_parses_into_entry`
**Source line(s):** 345–373
**Confidence:** HIGH
**Test type:** unit
**Given:** Registry TOML with `[hooks.config]` containing both a string (`script_path`) and a nested inline table (`extra = { key = "value" }`).
**When:** `Registry::parse_str` parses; `entry.config_as_json()` projects to JSON.
**Then:** `config_as_json().get("script_path").as_str() == Some("legacy-hooks/validate-template.sh")`; `config_as_json().get("extra").get("key").as_str() == Some("value")`.
**Acceptance:** The legacy-bash-adapter (and any future plugin) can rely on TOML `[hooks.config]` round-tripping through to its plugin-config view.

#### BC-AUDIT-2303 — factory-dispatcher::registry::defaults_applied_when_missing: omitted entry timeouts/fuel/priority/on_error fall through to Registry.defaults

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::defaults_applied_when_missing`
**Source line(s):** 375–384
**Confidence:** HIGH
**Test type:** unit
**Given:** Registry TOML with no `[defaults]` block and a hook entry that omits `timeout_ms`, `fuel_cap`, `priority`, `on_error`.
**When:** Entry helpers `priority(&defaults)` and `timeout_ms(&defaults)` are called.
**Then:** Defaults match documented sentinel values: `timeout_ms=5_000`, `fuel_cap=10_000_000`, `priority=500`, `on_error=Continue`. Helpers return those defaults.
**Acceptance:** Operators can omit fields from registry stanzas and rely on the spec-pinned defaults.

#### BC-AUDIT-2304 — factory-dispatcher::registry::rejects_unknown_on_error_value: on_error="shout" (or any non-{block,continue}) fails parse

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::rejects_unknown_on_error_value`
**Source line(s):** 438–450
**Confidence:** HIGH
**Test type:** unit
**Given:** TOML hook stanza with `on_error = "shout"`.
**When:** `Registry::parse_str` is called.
**Then:** Returns Err (serde rejects the unknown enum variant before any registry validation).
**Acceptance:** Typos in `on_error` produce a clean parse-time error rather than silently defaulting to `Continue`.

#### BC-AUDIT-2305 — factory-dispatcher::registry::accepts_capabilities_block: [hooks.capabilities] + nested [hooks.capabilities.exec_subprocess] + [hooks.capabilities.read_file] all parse and round-trip into typed Capabilities

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::accepts_capabilities_block`
**Source line(s):** 452–478
**Confidence:** HIGH
**Test type:** unit
**Given:** Hook stanza with top-level `[hooks.capabilities]` (env_allow), nested `[hooks.capabilities.exec_subprocess]` (binary_allow, cwd_allow, env_allow), and `[hooks.capabilities.read_file]` (path_allow).
**When:** `Registry::parse_str(toml)` parses.
**Then:** `entry.capabilities` is `Some(Capabilities)`; `caps.env_allow == ["CLAUDE_SESSION_ID"]`; `caps.exec_subprocess.unwrap().binary_allow == ["git"]`; `caps.read_file` is populated. The capability hierarchy is preserved through TOML parsing.
**Acceptance:** Capability-aware host functions can rely on the typed `Capabilities` struct, not raw TOML access.

#### BC-AUDIT-2306 — factory-dispatcher::registry::overrides_priority_per_entry: per-entry priority field overrides registry default

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::overrides_priority_per_entry`
**Source line(s):** 480–500
**Confidence:** HIGH
**Test type:** unit
**Given:** Two hooks declaring `priority = 10` and `priority = 900` respectively.
**When:** Each entry's `priority(&defaults)` is called.
**Then:** Returns the entry-declared values (10 and 900), not the default 500.
**Acceptance:** Operators can fine-tune per-entry priority without changing global defaults.

#### BC-AUDIT-2307 — factory-dispatcher::registry::load_returns_not_found_for_missing_path: missing registry file produces RegistryError::NotFound

**Test:** `crates/factory-dispatcher/src/registry.rs::tests::load_returns_not_found_for_missing_path`
**Source line(s):** 502–506
**Confidence:** HIGH
**Test type:** unit
**Given:** Path `/nonexistent/registry.toml` that does not exist.
**When:** `Registry::load(path)` is called.
**Then:** Returns `Err(RegistryError::NotFound(_))` (distinct error variant from `Io` so callers can produce a precise diagnostic).
**Acceptance:** Operator gets a precise "registry file not found" error instead of a generic IO error.

#### BC-AUDIT-2308 — factory-dispatcher::routing::group_returns_empty_for_no_matches: no-match payload yields empty tier list

**Test:** `crates/factory-dispatcher/src/routing.rs::tests::group_returns_empty_for_no_matches`
**Source line(s):** 257–263
**Confidence:** HIGH
**Test type:** unit
**Given:** Registry has no hooks for `event_name="SessionStart"`; payload requests SessionStart.
**When:** `match_plugins` then `group_by_priority` are called.
**Then:** `tiers.is_empty() == true`. Caller can safely treat the empty case as no-op.
**Acceptance:** Lifecycle events with no registered hooks produce empty tier output, not an error.

#### BC-AUDIT-2309 — factory-dispatcher::payload::parses_pretooluse: PreToolUse envelope deserializes with tool_input populated, tool_response None

**Test:** `crates/factory-dispatcher/src/payload.rs::tests::parses_pretooluse`
**Source line(s):** 88–100
**Confidence:** HIGH
**Test type:** unit
**Given:** JSON envelope with `event_name="PreToolUse"`, `tool_name="Bash"`, `session_id="sess-1"`, `tool_input={command:"ls"}`.
**When:** `HookPayload::from_bytes(json)` is called.
**Then:** Parsed payload has matching fields; `tool_response.is_none()` (PreToolUse pre-call invariant).
**Acceptance:** Pre-tool hooks can rely on `tool_response: None` as their PreToolUse signature.

#### BC-AUDIT-2310 — factory-dispatcher::payload::parses_posttooluse_with_response: PostToolUse envelope carries tool_response

**Test:** `crates/factory-dispatcher/src/payload.rs::tests::parses_posttooluse_with_response`
**Source line(s):** 102–114
**Confidence:** HIGH
**Test type:** unit
**Given:** JSON envelope with `event_name="PostToolUse"`, `tool_response={success:true}`.
**When:** Parsed via `HookPayload::from_bytes`.
**Then:** `tool_response.is_some()`. PostToolUse hooks can rely on response presence.
**Acceptance:** PostToolUse hooks differentiate from PreToolUse by `tool_response` presence.

#### BC-AUDIT-2311 — factory-dispatcher::payload::accepts_session_event_without_tool_name: SessionStart parses with tool_name defaulting to ""

**Test:** `crates/factory-dispatcher/src/payload.rs::tests::accepts_session_event_without_tool_name`
**Source line(s):** 116–125
**Confidence:** HIGH
**Test type:** unit
**Given:** JSON envelope `{event_name:"SessionStart", session_id:"s"}` with no `tool_name`.
**When:** `HookPayload::from_bytes` is called.
**Then:** Parses cleanly; `tool_name == ""` (serde default for empty-string).
**Acceptance:** Lifecycle events (SessionStart, SessionEnd, etc.) parse without a tool field.

#### BC-AUDIT-2312 — factory-dispatcher::internal_log::writes_jsonl_events_with_expected_shape: 10 events with trace_id and iteration field write 10 JSONL lines into a single rotated file

**Test:** `crates/factory-dispatcher/src/internal_log.rs::tests::writes_jsonl_events_with_expected_shape`
**Source line(s):** 317–347
**Confidence:** HIGH
**Test type:** unit (filesystem-touching)
**Given:** `InternalLog::new(tempdir)` plus a fixed timestamp 2026-04-24T12:00:00.
**When:** 10 events are written via `log.write(&InternalEvent::with_ts(DISPATCHER_STARTED, ts).with_trace_id(...).with_field("iteration", i))`.
**Then:** Exactly one rotated file `dispatcher-internal-2026-04-24.jsonl` exists; it contains 10 lines; each line is valid JSON with `type="dispatcher.started"`, `schema_version=INTERNAL_EVENT_SCHEMA_VERSION` (=1), `dispatcher_trace_id="trace-{i}"`, `iteration=i`, `ts` starts with "2026-04-24", `ts_epoch` is i64.
**Acceptance:** Internal-log writers produce JSONL of the documented shape with no surprise field reshuffling.

#### BC-AUDIT-2313 — factory-dispatcher::internal_log::skips_serializing_none_optional_fields: None-valued optional fields are skipped (not serialized as null)

**Test:** `crates/factory-dispatcher/src/internal_log.rs::tests::skips_serializing_none_optional_fields`
**Source line(s):** 495–505
**Confidence:** HIGH
**Test type:** unit
**Given:** `InternalEvent::with_ts(DISPATCHER_STARTED, ts)` (no `with_trace_id`, no `with_session_id`, no `with_plugin_*`).
**When:** Serialized to JSON.
**Then:** Result has no `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version` keys (they're absent, not null).
**Acceptance:** Downstream pipelines (jq filtering, OTel attribute promotion) see absence rather than null when a correlation field doesn't apply.

#### BC-AUDIT-2314 — factory-dispatcher::host::emit_event::decode_single_pair: length-prefixed key/value buffer with one pair round-trips through decode_fields

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::decode_single_pair`
**Source line(s):** 128–133
**Confidence:** HIGH
**Test type:** unit
**Given:** Buffer encoding `("k", "v")` as `[1u32_LE | 'k' | 1u32_LE | 'v']`.
**When:** `decode_fields(&buf)` is called.
**Then:** Returns `Ok(vec![("k".to_string(), "v".to_string())])`.
**Acceptance:** SDK-side `encode_fields` and dispatcher-side `decode_fields` are byte-compatible for the single-pair case.

#### BC-AUDIT-2315 — factory-dispatcher::host::emit_event::decode_multiple_pairs: 3-pair buffer round-trips with order preserved

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::decode_multiple_pairs`
**Source line(s):** 135–142
**Confidence:** HIGH
**Test type:** unit
**Given:** Buffer encoding pairs `[("a","1"), ("bb","22"), ("ccc","333")]`.
**When:** Decoded.
**Then:** Returns 3 pairs in order; last pair is `("ccc", "333")`.
**Acceptance:** Multi-field events preserve insertion order across the wire.

#### BC-AUDIT-2316 — factory-dispatcher::host::emit_event::decode_empty_buffer_yields_empty_vec: empty input → empty result, no error

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::decode_empty_buffer_yields_empty_vec`
**Source line(s):** 144–147
**Confidence:** HIGH
**Test type:** unit
**Given:** `decode_fields(&[])`.
**When:** Called.
**Then:** Returns `Ok(Vec::new())` — no error for the zero-pair case (events with only top-level reserved fields are valid).
**Acceptance:** A plugin can emit an event with zero extras without tripping the decoder.

#### BC-AUDIT-2317 — factory-dispatcher::host::emit_event::decode_rejects_truncated_key_length: <4-byte buffer triggers Err

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::decode_rejects_truncated_key_length`
**Source line(s):** 149–153
**Confidence:** HIGH
**Test type:** unit
**Given:** 3-byte buffer (insufficient for the 4-byte length prefix).
**When:** `decode_fields(&[0u8, 0, 0])`.
**Then:** Returns `Err`. Decoder refuses to over-read.
**Acceptance:** Malformed plugin output is rejected without panic.

#### BC-AUDIT-2318 — factory-dispatcher::host::emit_event::decode_rejects_truncated_key_bytes: declared key_len exceeds remaining buffer → Err

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::decode_rejects_truncated_key_bytes`
**Source line(s):** 155–161
**Confidence:** HIGH
**Test type:** unit
**Given:** Buffer `[5u32_LE | 'a','b']` declaring 5-byte key but only providing 2.
**When:** `decode_fields(&buf)`.
**Then:** Returns `Err`. No buffer-overrun panic.
**Acceptance:** Length-prefix validation is strict.

#### BC-AUDIT-2319 — factory-dispatcher::host::emit_event::reserved_fields_rejected: every name in RESERVED_FIELDS is recognized by is_reserved_field

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::reserved_fields_rejected`
**Source line(s):** 163–168
**Confidence:** HIGH
**Test type:** unit
**Given:** The constant slice `RESERVED_FIELDS` (8 names: dispatcher_trace_id, session_id, plugin_name, plugin_version, ts, ts_epoch, schema_version, type).
**When:** `is_reserved_field(name)` is called for each.
**Then:** All return true. The reserved-name list is the authoritative gate.
**Acceptance:** No reserved name slips through emit_event into plugin extras.

#### BC-AUDIT-2320 — factory-dispatcher::host::emit_event::non_reserved_field_accepted: non-reserved keys (commit_sha, file_path) pass

**Test:** `crates/factory-dispatcher/src/host/emit_event.rs::tests::non_reserved_field_accepted`
**Source line(s):** 170–173
**Confidence:** HIGH
**Test type:** unit
**Given:** `is_reserved_field("commit_sha")` and `is_reserved_field("file_path")`.
**When:** Called.
**Then:** Both return false. Plugin-relevant event data is allowed through.
**Acceptance:** The reserved list is closed (no false positives on plugin-specific fields).

#### BC-AUDIT-2321 — factory-dispatcher::host::log::level_mapping_matches_sdk: level u32 0..=4 maps to {trace,debug,info,warn,error}

**Test:** `crates/factory-dispatcher/src/host/log.rs::tests::level_mapping_matches_sdk`
**Source line(s):** 58–65
**Confidence:** HIGH
**Test type:** unit
**Given:** u32 levels 0..=4.
**When:** `level_to_str(level)` is called.
**Then:** Returns "trace", "debug", "info", "warn", "error" respectively. Mapping is paired with `vsdd_hook_sdk::LogLevel` (BC-AUDIT-127 covers SDK side).
**Acceptance:** Cross-crate level enum stays in lock-step with the dispatcher's host log.

#### BC-AUDIT-2322 — factory-dispatcher::host::read_file::denies_when_no_capability_block: no Capabilities.read_file block → CAPABILITY_DENIED

**Test:** `crates/factory-dispatcher/src/host/read_file.rs::tests::denies_when_no_capability_block`
**Source line(s):** 156–161
**Confidence:** HIGH
**Test type:** unit
**Given:** `bare_context()` (no capabilities populated).
**When:** `prepare(&ctx, "foo.txt", 1024)` is called.
**Then:** Returns `Err(CAPABILITY_DENIED)` — the deny-by-default path. (Note: this test exercises the standalone `prepare` fn at the host module level. The StoreData-level `read_file` is currently a stub returning CAPABILITY_DENIED unconditionally per BC-AUDIT-031; the prepare fn covers the hooked-up path that S-1.x integration wires in.)
**Acceptance:** read_file fails closed when no capability is declared.

#### BC-AUDIT-2323 — factory-dispatcher::host::read_file::reads_allowed_file: file under path_allow with size <= max_bytes returns its contents

**Test:** `crates/factory-dispatcher/src/host/read_file.rs::tests::reads_allowed_file`
**Source line(s):** 163–172
**Confidence:** HIGH
**Test type:** unit
**Given:** Tempdir contains `ok.txt = "hello world"`. Capability declares `path_allow = [tempdir]`. ctx.plugin_root = tempdir.
**When:** `prepare(&ctx, file_path, 1024)`.
**Then:** Returns `Ok((b"hello world", _))`.
**Acceptance:** Allow-listed paths can be read up to `max_bytes`.

#### BC-AUDIT-2324 — factory-dispatcher::host::read_file::rejects_path_outside_allow_list: file outside any allow-list prefix → CAPABILITY_DENIED

**Test:** `crates/factory-dispatcher/src/host/read_file.rs::tests::rejects_path_outside_allow_list`
**Source line(s):** 174–182
**Confidence:** HIGH
**Test type:** unit
**Given:** File at `tempdir/ok.txt` exists, but capability declares `path_allow = ["/nowhere/that/exists"]`.
**When:** `prepare(&ctx, file_path, 1024)`.
**Then:** Returns `Err(CAPABILITY_DENIED)`.
**Acceptance:** Paths outside the operator-declared allow-list are not readable, even if they exist.

#### BC-AUDIT-2325 — factory-dispatcher::host::read_file::rejects_file_exceeding_max_bytes: file size > max_bytes → OUTPUT_TOO_LARGE

**Test:** `crates/factory-dispatcher/src/host/read_file.rs::tests::rejects_file_exceeding_max_bytes`
**Source line(s):** 184–194
**Confidence:** HIGH
**Test type:** unit
**Given:** 2048-byte file. Capability allows the dir. Caller passes `max_bytes=512`.
**When:** `prepare(&ctx, file_path, 512)`.
**Then:** Returns `Err(OUTPUT_TOO_LARGE)`. Bounds are enforced by file metadata BEFORE read_to_end runs (defensive).
**Acceptance:** A plugin cannot accidentally OOM the dispatcher by reading a giant file.

#### BC-AUDIT-2326 — factory-dispatcher::host::read_file::relative_path_resolves_under_plugin_root: relative path joins under ctx.plugin_root

**Test:** `crates/factory-dispatcher/src/host/read_file.rs::tests::relative_path_resolves_under_plugin_root`
**Source line(s):** 196–204
**Confidence:** HIGH
**Test type:** unit
**Given:** File `rel.txt` at `tempdir/rel.txt`. Capability declares `path_allow = ["."]`. ctx.plugin_root = tempdir.
**When:** `prepare(&ctx, "rel.txt", 1024)`.
**Then:** Returns `Ok((b"yes", _))`. Relative paths resolve under plugin_root, not cwd.
**Acceptance:** Plugins can use relative paths that resolve consistently regardless of where the dispatcher is invoked from.

#### BC-AUDIT-2327 — factory-dispatcher::host::exec_subprocess::allows_shell_with_acknowledgment: shell_bypass_acknowledged set → bash allowed (gate passes; spawn may still fail with INTERNAL_ERROR on bashless hosts)

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::allows_shell_with_acknowledgment`
**Source line(s):** 346–366
**Confidence:** HIGH
**Test type:** unit
**Given:** Capability allows `bash` with `shell_bypass_acknowledged = "needed for git status parsing"`. Command `bash -c 'exit 0'`.
**When:** `run` is called.
**Then:** `result.is_ok() OR result == Err(INTERNAL_ERROR)`. INTERNAL_ERROR is acceptable on a bash-less host (which proves the policy gate passed and only the spawn failed).
**Acceptance:** Shell-bypass acknowledgment unblocks shell execution; the policy gate is unambiguous.

#### BC-AUDIT-2328 — factory-dispatcher::host::exec_subprocess::stdin_bytes_reach_subprocess: non-empty stdin_bytes is piped to bash and bash sees it on cat

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::stdin_bytes_reach_subprocess`
**Source line(s):** 368–399
**Confidence:** HIGH
**Test type:** unit (filesystem-touching, requires bash)
**Given:** Capability allows bash; stdin_bytes = `b"hello-from-stdin"`. Command `bash -c 'cat'`.
**When:** `run` executes; envelope decoded.
**Then:** Envelope's stdout (per BC-AUDIT-028 layout `i32_LE | u32_LE | stdout | u32_LE | stderr`) contains `hello-from-stdin`. Pipe is closed before stdout drain begins.
**Acceptance:** Plugins can pipe arbitrary bytes into bash subprocesses (legacy-bash-adapter relies on this for the JSON envelope).

#### BC-AUDIT-2329 — factory-dispatcher::host::exec_subprocess::binary_allow_matches_basename: allow-list compares against basename, not full path

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::binary_allow_matches_basename`
**Source line(s):** 401–406
**Confidence:** HIGH
**Test type:** unit
**Given:** `binary_allowed("/usr/bin/git", &["git"])`, `binary_allowed("git", &["git"])`, `binary_allowed("curl", &["git"])`.
**When:** Called.
**Then:** First two return true, third returns false. Allow-list match is on the file basename, not the absolute or relative path string.
**Acceptance:** Operators can declare `binary_allow = ["git"]` regardless of where git is installed on the host.

#### BC-AUDIT-2330 — factory-dispatcher::host::exec_subprocess::is_shell_detects_interpreters: SHELL_NAMES set is bash, sh, zsh, pwsh + path variants

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::is_shell_detects_interpreters`
**Source line(s):** 408–417
**Confidence:** HIGH
**Test type:** unit
**Given:** Various command strings.
**When:** `is_shell(cmd)` called.
**Then:** True for "bash", "/bin/bash", "sh", "zsh", "pwsh"; false for "git", "curl". Shell detection is by basename and matches SHELL_NAMES.
**Acceptance:** Shell-bypass-acknowledged enforcement (BC-AUDIT-025) catches every documented interpreter.

#### BC-AUDIT-2331 — factory-dispatcher::host::exec_subprocess::decode_args_round_trip: encoded args buffer round-trips through decode_args

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::decode_args_round_trip`
**Source line(s):** 419–431
**Confidence:** HIGH
**Test type:** unit
**Given:** Length-prefixed args buffer encoding ["a", "bb", "ccc"].
**When:** `decode_args(&buf)`.
**Then:** Returns `Some(vec!["a", "bb", "ccc"])`.
**Acceptance:** Args wire format is bidirectional and stable.

#### BC-AUDIT-2332 — factory-dispatcher::host::exec_subprocess::decode_args_rejects_truncated_buffer: declared length > available bytes → None

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::decode_args_rejects_truncated_buffer`
**Source line(s):** 433–437
**Confidence:** HIGH
**Test type:** unit
**Given:** Buffer `[10u8, 0, 0, 0, 'a']` declaring 10-byte arg but supplying 1.
**When:** `decode_args(&bad)`.
**Then:** Returns `None`. No panic, no over-read.
**Acceptance:** Malformed args buffers are rejected cleanly.

#### BC-AUDIT-2333 — factory-dispatcher::host::exec_subprocess::timeout_enforced: command exceeding timeout_ms is killed and returns TIMEOUT

**Test:** `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::timeout_enforced`
**Source line(s):** 449–462
**Confidence:** HIGH
**Test type:** unit (filesystem-touching, requires sleep)
**Given:** Capability allows `sleep` with PATH env. Command `sleep 5` with `timeout_ms = 200`.
**When:** `run` executes.
**Then:** Returns `Err(TIMEOUT)`. The deadline trips on the polling loop; child is killed via `child.kill()` and reaped.
**Acceptance:** Subprocess wall-clock is enforced; hung commands cannot stall the dispatcher.

#### BC-AUDIT-2334 — factory-dispatcher::sinks::mod::load_builds_file_sink_from_parsed_config: ObservabilityConfig with `type="file"` constructs a FileSink with the declared name

**Test:** `crates/factory-dispatcher/src/sinks/mod.rs::tests::load_builds_file_sink_from_parsed_config`
**Source line(s):** 262–282
**Confidence:** HIGH
**Test type:** unit
**Given:** ObservabilityConfig with one `type="file"` stanza named "local-events", path_template, enabled=true.
**When:** `SinkRegistry::from_config(cfg)`.
**Then:** Returns Ok; `reg.sinks().len() == 1`; `reg.sinks()[0].name() == "local-events"`. The sink-type dispatch correctly routes "file" to FileSink::new.
**Acceptance:** Operators write `type = "file"` and get a file sink in the registry.

#### BC-AUDIT-2335 — factory-dispatcher::executor (integration)::parallel_happy_path_five_plugins_one_tier: 5 plugins at same priority all return Ok and exit_code=0

**Test:** `crates/factory-dispatcher/tests/executor_integration.rs::parallel_happy_path_five_plugins_one_tier`
**Source line(s):** 91–118
**Confidence:** HIGH
**Test type:** integration (real wasmtime)
**Given:** 5 minimal WASI plugins compiled to a tempdir, all priority=100, all enabled.
**When:** `execute_tiers(inputs, tiers)` runs.
**Then:** `summary.per_plugin_results.len() == 5`, `exit_code == 0`, `block_intent == false`, every per-plugin result is `PluginResult::Ok { .. }`.
**Acceptance:** Same-tier plugins all execute and report results without interfering.

#### BC-AUDIT-2336 — factory-dispatcher::executor (integration)::crash_does_not_affect_siblings: one Crashed plugin doesn't break sibling Ok plugins; final exit_code stays 0

**Test:** `crates/factory-dispatcher/tests/executor_integration.rs::crash_does_not_affect_siblings`
**Source line(s):** 120–159
**Confidence:** HIGH
**Test type:** integration
**Given:** 3 plugins at same priority: ok-a (normal), crash (unreachable instruction), ok-b (normal).
**When:** `execute_tiers` runs.
**Then:** All 3 results present; ok-a and ok-b are `Ok`; crash is `Crashed`; `exit_code == 0` (crash with default `on_error=continue` does not block).
**Acceptance:** Plugin crashes are isolated to the crashing plugin (BC-AUDIT-016 isolation invariant pinned with a real test).

#### BC-AUDIT-2337 — factory-dispatcher::executor (integration)::parallel_timeout_does_not_cascade: hang plugin times out at 120ms while siblings complete in parallel; wall < 2s for 4-plugin tier

**Test:** `crates/factory-dispatcher/tests/executor_integration.rs::parallel_timeout_does_not_cascade`
**Source line(s):** 161–215
**Confidence:** HIGH
**Test type:** integration
**Given:** EpochTicker started; 4 plugins at priority=100: ok-a, hanger (timeout_ms=120, infinite loop), ok-b, ok-c.
**When:** `execute_tiers` runs.
**Then:** `wall_ms < 2_000` (asserts parallel execution; sequential would be much longer); `hanger` result is `Timeout`; all others are `Ok`.
**Acceptance:** Concurrent execution within a tier is real, not theoretical (BC-AUDIT-016 pinned with timing assertion).

#### BC-AUDIT-2338 — factory-dispatcher::executor (integration)::multi_tier_runs_in_priority_order: tier 10 plugin executes before tier 100 plugins

**Test:** `crates/factory-dispatcher/tests/executor_integration.rs::multi_tier_runs_in_priority_order`
**Source line(s):** 217–248
**Confidence:** HIGH
**Test type:** integration
**Given:** 3 plugins: late-a (priority=100), early (priority=10), late-b (priority=100).
**When:** `execute_tiers` runs.
**Then:** `summary.per_plugin_results[0].plugin_name == "early"`; `[1..]` contains both "late-a" and "late-b" (order within tier unspecified).
**Acceptance:** Tier ordering is observable in the per_plugin_results vec; pins BC-AUDIT-015 with a real run.

#### BC-AUDIT-2339 — factory-dispatcher::executor (integration)::empty_tier_set_returns_zero_exit_code: empty tier list yields summary with no results, exit_code=0, block_intent=false

**Test:** `crates/factory-dispatcher/tests/executor_integration.rs::empty_tier_set_returns_zero_exit_code`
**Source line(s):** 250–265
**Confidence:** HIGH
**Test type:** integration
**Given:** Empty tier list.
**When:** `execute_tiers(inputs, vec![])`.
**Then:** Summary has empty per_plugin_results, `exit_code=0`, `block_intent=false`.
**Acceptance:** No-match dispatches return cleanly (companion to BC-AUDIT-2308).

#### BC-AUDIT-2340 — factory-dispatcher::host_functions (integration)::setup_linker_registers_every_vsdd_import: setup_linker exports every named host fn in the vsdd namespace

**Test:** `crates/factory-dispatcher/tests/host_functions.rs::setup_linker_registers_every_vsdd_import`
**Source line(s):** 27–54
**Confidence:** HIGH
**Test type:** integration
**Given:** `setup_linker(&engine)` returns a fresh linker.
**When:** `linker.get(&mut store, "vsdd", name)` is called for each of {log, emit_event, read_file, exec_subprocess, session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd, env}.
**Then:** Every lookup succeeds. The named-import surface is complete and stable.
**Acceptance:** No host function added/dropped without the linker test failing CI.

#### BC-AUDIT-2341 — factory-dispatcher::host_functions (integration)::wat_module_importing_host_functions_instantiates: WAT module declaring vsdd imports links and runs to completion

**Test:** `crates/factory-dispatcher/tests/host_functions.rs::wat_module_importing_host_functions_instantiates`
**Source line(s):** 56–76
**Confidence:** HIGH
**Test type:** integration
**Given:** WAT module with imports for `vsdd::log`, `emit_event`, `session_id`, `env`, `read_file`, `exec_subprocess`. `_start` is a `nop`.
**When:** `linker.instantiate(&mut store, &module)` and `_start.call(&mut store, ())`.
**Then:** Both succeed without trap.
**Acceptance:** End-to-end host-link surface holds for a wasm module that imports the full vsdd ABI.

#### BC-AUDIT-2342 — factory-dispatcher::internal_log (integration)::startup_flow_writes_parseable_jsonl: 4-event dispatcher startup flow round-trips through JSONL with correct envelope per event

**Test:** `crates/factory-dispatcher/tests/internal_log_integration.rs::startup_flow_writes_parseable_jsonl`
**Source line(s):** 29–131
**Confidence:** HIGH
**Test type:** integration
**Given:** Fresh InternalLog at nested non-existent dir. 4 events written: dispatcher.started, plugin.loaded, plugin.invoked, internal.dispatcher_error.
**When:** Files are read back and parsed.
**Then:** Exactly one rotated file; 4 lines; each carries common envelope (schema_version, ts, ts_epoch, dispatcher_trace_id); event-specific extras are present (dispatcher_version, loaded_plugin_count, plugin_name, plugin_version, tool_name, message); `plugin.loaded` event has no `session_id` (omitted because not set).
**Acceptance:** Library re-export surface (`InternalEvent`, `InternalLog`, the const event-name strings) matches the unit-tested behavior end-to-end.

#### BC-AUDIT-2343 — factory-dispatcher::internal_log (integration)::write_is_best_effort_when_path_is_a_file: log_dir pointing at an existing file → write returns silently without panic

**Test:** `crates/factory-dispatcher/tests/internal_log_integration.rs::write_is_best_effort_when_path_is_a_file`
**Source line(s):** 133–144
**Confidence:** HIGH
**Test type:** integration
**Given:** `InternalLog::new(file_path)` where file_path is an existing regular file (not a dir).
**When:** `log.write(&InternalEvent::now(DISPATCHER_STARTED))`.
**Then:** No panic. (Pinned BC-AUDIT-035 best-effort contract for the "create_dir_all on a file path" failure mode.)
**Acceptance:** Misconfiguration of log_dir cannot crash the dispatcher.

#### BC-AUDIT-2344 — factory-dispatcher::loads_legacy_registry::every_entry_routes_through_legacy_bash_adapter: every entry in the production registry routes through legacy-bash-adapter.wasm

**Test:** `crates/factory-dispatcher/tests/loads_legacy_registry.rs::every_entry_routes_through_legacy_bash_adapter`
**Source line(s):** 60–76
**Confidence:** HIGH
**Test type:** integration
**Given:** `plugins/vsdd-factory/hooks-registry.toml` (45 entries today).
**When:** Each entry's plugin file basename is checked.
**Then:** All 45 plugins resolve to `legacy-bash-adapter.wasm`. (At this point in the migration, every v0.79.x bash hook routes through the adapter.)
**Acceptance:** No native plugin has been "smuggled" into the registry without the appropriate engineering work (S-3.1+ swap-out story).

#### BC-AUDIT-2345 — factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path: every entry has plugin_config.script_path matching `hooks/<name>.sh`

**Test:** `crates/factory-dispatcher/tests/loads_legacy_registry.rs::every_entry_carries_a_script_path`
**Source line(s):** 78–105
**Confidence:** HIGH
**Test type:** integration
**Given:** Production registry.
**When:** Each entry's `config_as_json().get("script_path")` is checked.
**Then:** Non-empty string matching `hooks/*.sh`. (Catches generator drift: if generate-registry-from-hooks-json.sh ever drops script_path, the dispatcher refuses the payload at runtime — BC-AUDIT-054.)
**Acceptance:** Schema-time invariant for legacy-bash-adapter routing is enforced; CI catches regressions before runtime.

#### BC-AUDIT-2346 — factory-dispatcher::sinks_file_integration::registry_fans_events_to_file_sinks_with_filter_and_tags: 10 events route to 2 sinks with filter (audit drops 2) and tag enrichment (env=dev,host=ci on every local event)

**Test:** `crates/factory-dispatcher/tests/sinks_file_integration.rs::registry_fans_events_to_file_sinks_with_filter_and_tags`
**Source line(s):** 68–166
**Confidence:** HIGH
**Test type:** integration (end-to-end)
**Given:** Real observability-config.toml with two `type="file"` stanzas (one tagged, one filtered) and one unknown `type="datadog"` stanza. 10 events submitted via Router.
**When:** Router.flush() then files inspected.
**Then:** local-events file has 10 lines (accepts everything) with `env=dev,host=ci` on every line; audit-filtered file has 8 lines (denies plugin.timeout + internal.sink_error); datadog stanza was skipped (per BC-AUDIT-042); post-shutdown submit is a no-op (no new lines).
**Acceptance:** End-to-end sink fan-out + filter + tag enrichment is wired through Router (not just per-driver behavior).

#### BC-AUDIT-2347 — factory-dispatcher::sinks_otel_grpc (integration)::ten_events_arrive_with_correct_attribute_mapping: 10 events sent through real gRPC to mock collector arrive with body=type, time_unix_nano=ts_epoch*1_000_000, reserved attrs lifted, no leak of type/ts_epoch as attributes

**Test:** `crates/factory-dispatcher/tests/sinks_otel_grpc.rs::ten_events_arrive_with_correct_attribute_mapping`
**Source line(s):** 198–251
**Confidence:** HIGH
**Test type:** integration (real gRPC server)
**Given:** Mock LogsService bound on 127.0.0.1:0; sink configured with batch.size=100, batch.interval=60s; 10 events submitted.
**When:** sink.flush() then mock server's snapshot is taken.
**Then:** 10 records arrive; first record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000 (ms→ns); attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version, seq; type and ts_epoch are NOT attributes (they were lifted to body and time_unix_nano).
**Acceptance:** OTLP wire-shape is byte-compatible with operator dashboards (Grafana / Tempo / Loki) without translation.

### 2.2 — hook-plugins

#### BC-AUDIT-2348 — hook-plugins::capture-commit-activity::on_hook_returns_zero_in_stub: stub on_hook returns 0 (pre-S-3.1 placeholder)

**Test:** `crates/hook-plugins/capture-commit-activity/src/lib.rs::tests::on_hook_returns_zero_in_stub`
**Source line(s):** 16–19
**Confidence:** HIGH
**Test type:** unit
**Given:** Pre-S-3.1 stub crate.
**When:** `on_hook()` is called.
**Then:** Returns 0.
**Acceptance:** Crate compiles and the workspace member layout / wasm32-wasip1 build path stays alive even before S-3.1's real binding lands. (The BC documents the stub state of a story-traced TODO; S-3.1 will replace this BC.)

#### BC-AUDIT-2349 — hook-plugins::legacy-bash-adapter::passes_payload_bytes_to_bash_with_plugin_config_stripped: re-serialized payload reaches bash with plugin_config=null while preserving event_name + dispatcher_trace_id

**Test:** `crates/hook-plugins/legacy-bash-adapter/src/lib.rs::tests::passes_payload_bytes_to_bash_with_plugin_config_stripped`
**Source line(s):** 298–331
**Confidence:** HIGH
**Test type:** unit
**Given:** Payload with `plugin_config = {script_path:"echo.sh", extra:1}`.
**When:** `adapter_logic` runs with a runner closure that captures the bytes piped to bash.
**Then:** Captured bytes parse as JSON with `plugin_config: null`, but `event_name == "PostToolUse"` and `dispatcher_trace_id == "trace-1"` are preserved. (Pins BC-AUDIT-055 / BC-AUDIT-134 with a real captured-bytes assertion.)
**Acceptance:** Bash hooks see no plugin_config (predates the field) but get full upstream context.

### 2.3 — hook-sdk

#### BC-AUDIT-2350 — hook-sdk::__internal::panic_message_extracts_static_str: panic of `&str` is extracted into the panic message

**Test:** `crates/hook-sdk/src/__internal.rs::tests::panic_message_extracts_static_str`
**Source line(s):** 83–87
**Confidence:** HIGH
**Test type:** unit
**Given:** `Box::new("boom")` as the panic payload.
**When:** `panic_message(&panic)`.
**Then:** Returns "boom".
**Acceptance:** The `#[hook]` macro's catch_unwind flow can reliably surface panic messages to the dispatcher.

#### BC-AUDIT-2351 — hook-sdk::__internal::panic_message_extracts_string: panic of `String` is extracted

**Test:** `crates/hook-sdk/src/__internal.rs::tests::panic_message_extracts_string`
**Source line(s):** 89–93
**Confidence:** HIGH
**Test type:** unit
**Given:** `Box::new("formatted".to_string())`.
**When:** `panic_message(&panic)`.
**Then:** Returns "formatted".
**Acceptance:** Both formatted and static panic messages are surfaced.

#### BC-AUDIT-2352 — hook-sdk::__internal::panic_message_falls_back_for_unknown_types: non-string panic payloads return "(no panic message)"

**Test:** `crates/hook-sdk/src/__internal.rs::tests::panic_message_falls_back_for_unknown_types`
**Source line(s):** 95–99
**Confidence:** HIGH
**Test type:** unit
**Given:** `Box::new(42i32)`.
**When:** `panic_message(&panic)`.
**Then:** Returns "(no panic message)" — does not panic recursively.
**Acceptance:** Even exotic panic payloads produce a sensible diagnostic.

#### BC-AUDIT-2353 — hook-sdk::host::encode_fields_uses_length_prefix: encode_fields([(k,v)]) lays out key_len|key|value_len|value LE-prefixed

**Test:** `crates/hook-sdk/src/host.rs::tests::encode_fields_uses_length_prefix`
**Source line(s):** 317–328
**Confidence:** HIGH
**Test type:** unit
**Given:** `encode_fields(&[("k","vv"), ("aa","b")])`.
**When:** Buffer inspected byte-by-byte.
**Then:** Total length matches `4+1 + 4+2 + 4+2 + 4+1`; `buf[0..4] == 1u32_LE`; `buf[4] == b'k'`; `buf[5..9] == 2u32_LE`; `buf[9..11] == b"vv"`. (Byte-compatible with dispatcher decode_fields per BC-AUDIT-2314.)
**Acceptance:** Cross-crate length-prefix encoding is wire-stable.

#### BC-AUDIT-2354 — hook-sdk::host::encode_args_round_trip: encode_args matches the same length-prefix shape with no separator

**Test:** `crates/hook-sdk/src/host.rs::tests::encode_args_round_trip`
**Source line(s):** 330–336
**Confidence:** HIGH
**Test type:** unit
**Given:** `encode_args(&["a", "bb", "ccc"])`.
**When:** Buffer inspected.
**Then:** Total length `4+1 + 4+2 + 4+3`; `buf[0..4] == 1u32_LE`, `buf[4] == b'a'`. Same shape as encode_fields but pairs reduce to 1 length-prefix per arg.
**Acceptance:** SDK args encoder matches dispatcher decode_args (BC-AUDIT-2331).

#### BC-AUDIT-2355 — hook-sdk::host::decode_subprocess_result_parses_envelope: SubprocessResult envelope `i32 | u32 | stdout | u32 | stderr` decodes correctly

**Test:** `crates/hook-sdk/src/host.rs::tests::decode_subprocess_result_parses_envelope`
**Source line(s):** 347–359
**Confidence:** HIGH
**Test type:** unit
**Given:** Hand-built envelope: `7i32_LE | 3u32_LE | "out" | 2u32_LE | "er"`.
**When:** `decode_subprocess_result(&bytes)`.
**Then:** Returns `Some(SubprocessResult { exit_code: 7, stdout: b"out", stderr: b"er" })`. (Mirrors dispatcher's encode_envelope per BC-AUDIT-028.)
**Acceptance:** Plugins can rely on the documented envelope layout end-to-end.

#### BC-AUDIT-2356 — hook-sdk::host::log_levels_are_stable: LogLevel discriminants 0..=4 are pinned (Trace=0, Debug=1, Info=2, Warn=3, Error=4)

**Test:** `crates/hook-sdk/src/host.rs::tests::log_levels_are_stable`
**Source line(s):** 367–374
**Confidence:** HIGH
**Test type:** unit
**Given:** The LogLevel enum.
**When:** Each variant is cast to `u32`.
**Then:** Discriminants are exactly 0..=4 in the order Trace/Debug/Info/Warn/Error. (Cross-crate constant; the dispatcher's `level_to_str` per BC-AUDIT-2321 maps the same range.)
**Acceptance:** LogLevel discriminants are wire-stable; a future renumbering is a major-version event.

#### BC-AUDIT-2357 — hook-sdk::payload::pretooluse_payload_deserializes: full envelope parses with tool_input populated, tool_response None

**Test:** `crates/hook-sdk/src/payload.rs::tests::pretooluse_payload_deserializes`
**Source line(s):** 63–83
**Confidence:** HIGH
**Test type:** unit
**Given:** SDK-side full envelope (with `dispatcher_trace_id`).
**When:** `serde_json::from_str` parses.
**Then:** All fields populate; `tool_response.is_none()`; `tool_input.command == "git status"`.
**Acceptance:** SDK consumers see the same shape as dispatcher payload but with `dispatcher_trace_id` (added by dispatcher pre-handoff).

#### BC-AUDIT-2358 — hook-sdk::payload::posttooluse_payload_with_response: SDK payload includes typed access to tool_response.exit_code

**Test:** `crates/hook-sdk/src/payload.rs::tests::posttooluse_payload_with_response`
**Source line(s):** 85–100
**Confidence:** HIGH
**Test type:** unit
**Given:** PostToolUse envelope with `tool_response={exit_code:0}`.
**When:** Parsed; `tool_response.get("exit_code").as_i64()`.
**Then:** Returns `Some(0)`.
**Acceptance:** Plugin authors can downcast tool_response to typed values without manual JSON walking.

#### BC-AUDIT-2359 — hook-sdk::payload::lifecycle_payload_without_tool_name: SessionStart parses with tool_name="" and tool_input is JSON null

**Test:** `crates/hook-sdk/src/payload.rs::tests::lifecycle_payload_without_tool_name`
**Source line(s):** 102–115
**Confidence:** HIGH
**Test type:** unit
**Given:** `{event_name:"SessionStart", session_id:"sess-x", dispatcher_trace_id:"trace-x"}`.
**When:** Parsed.
**Then:** `tool_name == ""`; `tool_input.is_null() == true`; `tool_response.is_none()`. SDK lifecycle hooks see no tool fields.
**Acceptance:** Hooks for SessionStart/SessionEnd/SubagentStop/etc. don't fail when no tool is in scope.

#### BC-AUDIT-2360 — hook-sdk::payload::payload_round_trip_via_serde: serialize → deserialize preserves event_name and session_id

**Test:** `crates/hook-sdk/src/payload.rs::tests::payload_round_trip_via_serde`
**Source line(s):** 117–132
**Confidence:** HIGH
**Test type:** unit
**Given:** Original PreToolUse payload.
**When:** `to_string` then `from_str`.
**Then:** Round-trip preserves event_name and session_id.
**Acceptance:** Plugins that need to re-serialize the payload (e.g., to pipe to bash) can rely on round-trip safety.

#### BC-AUDIT-2361 — hook-sdk::payload::plugin_config_passes_through_when_present: plugin_config field arrives populated when the registry sets it

**Test:** `crates/hook-sdk/src/payload.rs::tests::plugin_config_passes_through_when_present`
**Source line(s):** 146–160
**Confidence:** HIGH
**Test type:** unit
**Given:** Envelope `{ ..., "plugin_config": {"script_path": "hooks/foo.sh"} }`.
**When:** Parsed.
**Then:** `payload.plugin_config.get("script_path").as_str() == Some("hooks/foo.sh")`.
**Acceptance:** Plugin authors (esp. legacy-bash-adapter) can rely on plugin_config arriving populated when the registry declares `[hooks.config]`.

### 2.4 — sink-core

#### BC-AUDIT-2362 — sink-core::routing_filter_default_accepts_everything: empty allow + empty deny → every event passes

**Test:** `crates/sink-core/src/lib.rs::tests::routing_filter_default_accepts_everything`
**Source line(s):** 249–255
**Confidence:** HIGH
**Test type:** unit
**Given:** `RoutingFilter::default()` (both lists empty).
**When:** `accepts(<any-name>)` is called.
**Then:** Always true. Pass-through default is preserved when no filter is configured. (Companion to BC-AUDIT-044.)
**Acceptance:** Operators omitting routing_filter get a no-op pass-through.

#### BC-AUDIT-2363 — sink-core::sink_event_event_type_accessor_reads_type_field: SinkEvent.event_type() returns the "type" field as &str

**Test:** `crates/sink-core/src/lib.rs::tests::sink_event_event_type_accessor_reads_type_field`
**Source line(s):** 324–328
**Confidence:** HIGH
**Test type:** unit
**Given:** `SinkEvent::new().insert("type", "commit.made")`.
**When:** `event_type()` is called.
**Then:** Returns `Some("commit.made")`.
**Acceptance:** Routing inspection of `type` is via the typed accessor, not raw map indexing.

#### BC-AUDIT-2364 — sink-core::sink_event_event_type_missing_returns_none: no "type" field → event_type() returns None

**Test:** `crates/sink-core/src/lib.rs::tests::sink_event_event_type_missing_returns_none`
**Source line(s):** 330–334
**Confidence:** HIGH
**Test type:** unit
**Given:** Empty SinkEvent.
**When:** `event_type()`.
**Then:** Returns None.
**Acceptance:** Producer-bug case (forgot to set `type`) is observable, not implicit.

#### BC-AUDIT-2365 — sink-core::sink_event_event_type_non_string_returns_none: "type" set to non-string Value → event_type() returns None

**Test:** `crates/sink-core/src/lib.rs::tests::sink_event_event_type_non_string_returns_none`
**Source line(s):** 336–340
**Confidence:** HIGH
**Test type:** unit
**Given:** `SinkEvent::new().insert("type", json!(42))`.
**When:** `event_type()`.
**Then:** Returns None. (Type is a string-typed reserved field; numeric `type` is producer error.)
**Acceptance:** Drivers can drop events with malformed type without panicking.

#### BC-AUDIT-2366 — sink-core::sink_config_common_defaults_enabled_true: minimal SinkConfigCommon TOML defaults enabled=true, no filter, empty tags

**Test:** `crates/sink-core/src/lib.rs::tests::sink_config_common_defaults_enabled_true`
**Source line(s):** 356–366
**Confidence:** HIGH
**Test type:** unit
**Given:** TOML `name = "local-events"` (only).
**When:** Parsed via `toml::from_str::<SinkConfigCommon>`.
**Then:** `cfg.enabled == true`; `cfg.routing_filter.is_none()`; `cfg.tags.is_empty()`.
**Acceptance:** Operators write minimal sink stanzas and get sensible defaults.

#### BC-AUDIT-2367 — sink-core::routing_filter_allow_case_sensitive: allow-list compares case-sensitively (Commit.Made ≠ commit.made)

**Test:** `crates/sink-core/src/lib.rs::tests::routing_filter_allow_case_sensitive`
**Source line(s):** 312–322
**Confidence:** HIGH
**Test type:** unit
**Given:** Filter `allow=["Commit.Made"]`.
**When:** `accepts("commit.made")` and `accepts("Commit.Made")`.
**Then:** First false, second true. Event-type names are case-sensitive (lowercase-with-dots is the spec convention).
**Acceptance:** Typo-detection works; "Commit.Made" doesn't accidentally match "commit.made".

### 2.5 — sink-file

#### BC-AUDIT-2368 — sink-file::template_date_only: `{date}` substitutes to YYYY-MM-DD

**Test:** `crates/sink-file/src/lib.rs::tests::template_date_only`
**Source line(s):** 549–554
**Confidence:** HIGH
**Test type:** unit
**Given:** `path_template = ".factory/logs/events-{date}.jsonl"`, fixed_date = 2026-04-24.
**When:** `resolve_path_template`.
**Then:** Returns `".factory/logs/events-2026-04-24.jsonl"`.
**Acceptance:** Date templating produces ISO-8601 dates compatible with daily-rotation downstream tooling.

#### BC-AUDIT-2369 — sink-file::template_name_only: `{name}` substitutes to the sink's operator-assigned name

**Test:** `crates/sink-file/src/lib.rs::tests::template_name_only`
**Source line(s):** 556–561
**Confidence:** HIGH
**Test type:** unit
**Given:** `{name}.jsonl`, name="audit-archive".
**When:** Resolved.
**Then:** Returns `/var/log/audit-archive.jsonl`.
**Acceptance:** Operators can disambiguate per-sink output files via the sink name token.

#### BC-AUDIT-2370 — sink-file::template_project_basename: `{project}` substitutes to the basename of project_dir

**Test:** `crates/sink-file/src/lib.rs::tests::template_project_basename`
**Source line(s):** 563–573
**Confidence:** HIGH
**Test type:** unit
**Given:** project_dir = `/home/dev/vsdd-factory`, template = `{project}/logs/{date}.jsonl`.
**When:** Resolved.
**Then:** Returns `vsdd-factory/logs/2026-04-24.jsonl` — only the basename of project_dir is used.
**Acceptance:** Operators get clean per-project output files without leaking absolute paths into log filenames.

#### BC-AUDIT-2371 — sink-file::template_all_placeholders: `{project}/{name}/{date}` interpolates all three with trailing-slash-tolerant project

**Test:** `crates/sink-file/src/lib.rs::tests::template_all_placeholders`
**Source line(s):** 575–585
**Confidence:** HIGH
**Test type:** unit
**Given:** project_dir = `/opt/work/myproj/` (with trailing slash), name="local", template uses all 3 placeholders.
**When:** Resolved.
**Then:** Returns `myproj/local/events-2026-04-24.jsonl`. Trailing slashes on project_dir don't break basename extraction.
**Acceptance:** Operators can pass project_dir with or without trailing slash.

#### BC-AUDIT-2372 — sink-file::template_no_project_yields_empty_basename: template uses {project} but None passed → empty interpolation, no error

**Test:** `crates/sink-file/src/lib.rs::tests::template_no_project_yields_empty_basename`
**Source line(s):** 587–592
**Confidence:** HIGH
**Test type:** unit
**Given:** project_dir = None, template = `{project}events-{date}.jsonl`.
**When:** Resolved.
**Then:** Returns `events-2026-04-24.jsonl` (project replaced with empty string, not error).
**Acceptance:** Optional project_dir gracefully degrades to empty rather than failing config load.

#### BC-AUDIT-2373 — sink-file::template_unbalanced_brace_treated_literally: opening `{` without closing `}` is treated as a literal

**Test:** `crates/sink-file/src/lib.rs::tests::template_unbalanced_brace_treated_literally`
**Source line(s):** 605–609
**Confidence:** HIGH
**Test type:** unit
**Given:** Template `weird-{date-only.jsonl` (no closing brace).
**When:** Resolved.
**Then:** Returns `weird-{date-only.jsonl` literally (no substitution attempted on unclosed brace).
**Acceptance:** Operator typos in templates produce a recognizable filename rather than a parse error or partial substitution.

#### BC-AUDIT-2374 — sink-file::auto_creates_parent_directory: nested non-existent parent dirs are mkdir-p'd on first event

**Test:** `crates/sink-file/src/lib.rs::tests::auto_creates_parent_directory`
**Source line(s):** 633–649
**Confidence:** HIGH
**Test type:** unit (filesystem-touching)
**Given:** Path template points 3 levels deep into a non-existent directory.
**When:** First event submitted + flushed.
**Then:** Parent dirs are created; file exists.
**Acceptance:** Operators don't have to pre-create the log directory tree.

#### BC-AUDIT-2375 — sink-file::jsonl_append_preserves_three_events: 3 sequential events produce 3 lines in submission order, each parseable JSON

**Test:** `crates/sink-file/src/lib.rs::tests::jsonl_append_preserves_three_events`
**Source line(s):** 651–680
**Confidence:** HIGH
**Test type:** unit
**Given:** 3 events submitted: plugin.invoked (a), plugin.completed (a), commit.made (sha=deadbeef).
**When:** flush().
**Then:** File has 3 lines; first line's `type == "plugin.invoked"`; last line's `sha == "deadbeef"`. Append-only ordering is preserved.
**Acceptance:** Downstream consumers (jq, logsearch tooling) get reliable JSONL.

#### BC-AUDIT-2376 — sink-file::routing_filter_drops_excluded_events: allow=["commit.made"] passes 2 of 3 events through

**Test:** `crates/sink-file/src/lib.rs::tests::routing_filter_drops_excluded_events`
**Source line(s):** 682–710
**Confidence:** HIGH
**Test type:** unit
**Given:** Sink with allow=["commit.made"]. 3 events submitted: commit.made, plugin.invoked, commit.made.
**When:** flush().
**Then:** File has 2 lines, both `type == "commit.made"`. Filter applied at the sink level.
**Acceptance:** Per-sink filtering works at the FileSink layer, not just the registry layer.

#### BC-AUDIT-2377 — sink-file::tag_enrichment_writes_tags_onto_every_event: configured tags `env=prod,team=factory` land on every event written by the sink

**Test:** `crates/sink-file/src/lib.rs::tests::tag_enrichment_writes_tags_onto_every_event`
**Source line(s):** 712–736
**Confidence:** HIGH
**Test type:** unit
**Given:** Sink config with tags = `{env: "prod", team: "factory"}`.
**When:** One event submitted + flushed.
**Then:** Written line has `env == "prod"` and `team == "factory"` alongside the producer-supplied `type == "commit.made"`.
**Acceptance:** Operators can attribute per-sink source data via static tags without touching the producer.

#### BC-AUDIT-2378 — sink-file::tag_enrichment_does_not_overwrite_producer_fields: tag with key="type" does NOT clobber the producer's `type` field

**Test:** `crates/sink-file/src/lib.rs::tests::tag_enrichment_does_not_overwrite_producer_fields`
**Source line(s):** 738–759
**Confidence:** HIGH
**Test type:** unit
**Given:** Sink configured with `tags = {type: "stomped"}`. Event with `type="commit.made"` submitted.
**When:** Flushed.
**Then:** File line has `type == "commit.made"` — producer field WINS over tag-key collision.
**Acceptance:** Tag enrichment is non-destructive; producer-supplied `type` is authoritative.

#### BC-AUDIT-2379 — sink-file::disabled_sink_drops_every_event: enabled=false → no file written, no events accepted

**Test:** `crates/sink-file/src/lib.rs::tests::disabled_sink_drops_every_event`
**Source line(s):** 761–778
**Confidence:** HIGH
**Test type:** unit
**Given:** Sink config with `enabled = false`.
**When:** Event submitted + flushed.
**Then:** Output file does NOT exist. Disabled sink is a hard no-op.
**Acceptance:** Operators can disable a misbehaving sink without deleting the stanza.

#### BC-AUDIT-2380 — sink-file::read_only_path_records_failure_without_panic: read-only target dir → SinkFailure recorded, no panic

**Test:** `crates/sink-file/src/lib.rs::tests::read_only_path_records_failure_without_panic` (Unix-only `#[cfg(unix)]`)
**Source line(s):** 780–811
**Confidence:** HIGH
**Test type:** unit (filesystem-touching)
**Given:** Tempdir chmod 0o555. Sink writes into it.
**When:** Event submitted + flushed.
**Then:** `take_failures()` returns at least one SinkFailure; no panic. Pins BC-AUDIT-048 with a real error.
**Acceptance:** Permission errors are observable via the failure log, not by crashing the dispatcher.

#### BC-AUDIT-2381 — sink-file::backpressure_fills_queue_and_increments_counter: queue_depth=2 + 500 submitted events → queue_full_count > 0

**Test:** `crates/sink-file/src/lib.rs::tests::backpressure_fills_queue_and_increments_counter`
**Source line(s):** 813–839
**Confidence:** HIGH
**Test type:** unit
**Given:** Sink with `queue_depth = 2`. 500 events submitted in a tight loop.
**When:** After a 50ms drain delay, `queue_full_count()` is read.
**Then:** Counter > 0. Pins BC-AUDIT-047 backpressure with a real overflow.
**Acceptance:** Producer never blocks under flood; overflow is observable.

#### BC-AUDIT-2382 — sink-file::shutdown_drains_queued_events: shutdown() drains pending events; post-shutdown submit is a no-op

**Test:** `crates/sink-file/src/lib.rs::tests::shutdown_drains_queued_events`
**Source line(s):** 841–867
**Confidence:** HIGH
**Test type:** unit
**Given:** 5 events submitted. shutdown() called.
**When:** File is read; then a "after" event is submitted; file is re-read.
**Then:** First read shows 5 lines (drain succeeded); second read still shows 5 lines (post-shutdown submit is no-op).
**Acceptance:** Shutdown is graceful and sinks become inert (don't crash, don't append) post-close.

#### BC-AUDIT-2383 — sink-file::config_deserializes_from_toml: minimal TOML config parses with queue_depth defaulting to DEFAULT_QUEUE_DEPTH=1000

**Test:** `crates/sink-file/src/lib.rs::tests::config_deserializes_from_toml`
**Source line(s):** 869–880
**Confidence:** HIGH
**Test type:** unit
**Given:** Minimal TOML `name="local-events", enabled=true, path_template="..."`.
**When:** Parsed via `toml::from_str::<FileSinkConfig>`.
**Then:** `cfg.queue_depth == DEFAULT_QUEUE_DEPTH (1000)`.
**Acceptance:** Operators omit queue_depth and get the documented default.

### 2.6 — sink-otel-grpc

#### BC-AUDIT-2384 — sink-otel-grpc::event_to_log_record_maps_reserved_fields: SinkEvent → LogRecord lifts type→body, ts_epoch_ms→time_unix_nano (×1_000_000), reserved attrs to OTLP key-values

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_maps_reserved_fields`
**Source line(s):** 810–847
**Confidence:** HIGH
**Test type:** unit
**Given:** Event with `type=plugin.invoked`, `ts_epoch=1_777_003_425_000`, dispatcher_trace_id, session_id, plugin_name, plugin_version.
**When:** `event_to_log_record(event)`.
**Then:** Record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000; observed_time_unix_nano == time_unix_nano; attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version; type and ts_epoch are NOT also attributes.
**Acceptance:** OTLP wire shape is correct out of the box; reserved fields don't leak as attributes.

#### BC-AUDIT-2385 — sink-otel-grpc::event_attributes_flatten_non_reserved_fields: non-reserved fields flatten to OTLP attributes with type-correct AnyValueInner variants

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::event_attributes_flatten_non_reserved_fields`
**Source line(s):** 849–878
**Confidence:** HIGH
**Test type:** unit
**Given:** Event with `sha=deadbeef` (string), `files_changed=7` (int), `dirty=true` (bool), `score=0.42` (float).
**When:** `event_to_log_record` → attribute map inspected.
**Then:** `sha` is StringValue, `files_changed` is IntValue(7), `dirty` is BoolValue(true), `score` is DoubleValue(~0.42).
**Acceptance:** Non-reserved field types map to OTLP variants without lossy coercion.

#### BC-AUDIT-2386 — sink-otel-grpc::event_to_log_record_nested_value_serialized_to_string: nested JSON values are stringified for OTLP attributes (analysts can parse_json downstream)

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_nested_value_serialized_to_string`
**Source line(s):** 880–899
**Confidence:** HIGH
**Test type:** unit
**Given:** Event with `nested = {inner:"value", n:1}`.
**When:** `event_to_log_record`.
**Then:** Attribute `nested` is StringValue(JSON-serialized) and parses back to `{inner:"value", n:1}`.
**Acceptance:** Plugin-supplied nested data round-trips through OTLP without loss.

#### BC-AUDIT-2387 — sink-otel-grpc::event_to_log_record_missing_type_yields_empty_body: producer-bug missing-type yields empty body, no panic

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_missing_type_yields_empty_body`
**Source line(s):** 901–907
**Confidence:** HIGH
**Test type:** unit
**Given:** Event with `ts_epoch` only, no `type`.
**When:** `event_to_log_record`.
**Then:** body_string == ""; no panic.
**Acceptance:** Defense-in-depth: producer bugs producing a missing `type` don't crash the pipeline.

#### BC-AUDIT-2388 — sink-otel-grpc::event_to_log_record_missing_ts_yields_zero_timestamp: missing ts_epoch → time_unix_nano = 0, no panic

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_missing_ts_yields_zero_timestamp`
**Source line(s):** 909–914
**Confidence:** HIGH
**Test type:** unit
**Given:** Event with only `type`.
**When:** `event_to_log_record`.
**Then:** time_unix_nano == 0.
**Acceptance:** Missing timestamps are zeroed (downstream OTel ingest can detect and synthesize their own observed time).

#### BC-AUDIT-2389 — sink-otel-grpc::resource_attributes_merge_defaults_with_config: operator overrides win over auto-populated defaults; auto-populated host.name is preserved

**Test:** `crates/sink-otel-grpc/src/lib.rs::tests::resource_attributes_merge_defaults_with_config`
**Source line(s):** 916–943
**Confidence:** HIGH
**Test type:** unit
**Given:** Operator overrides include `service.name="custom-svc"`, `deployment.env="dev"`.
**When:** `build_resource_attributes(&overrides)`.
**Then:** service.name=="custom-svc" (override wins); deployment.env=="dev" (operator-only); host.name is auto-populated from gethostname() and non-empty.
**Acceptance:** Resource attributes blend operator config with sensible defaults.

### 2.7 — Tests with multiple BCs (rare; flagged here)

No tests in this round produce multiple BCs. Each test maps 1:1 to a BC. The closest split-candidate was `event_to_log_record_maps_reserved_fields` (which tests body-mapping AND timestamp-mapping AND reserved-attribute-promotion) — but the assertions are unified under "OTLP record-shape contract" and properly belong to a single BC (BC-AUDIT-2384).

### 2.8 — Tests already covered by BC-AUDIT-001..143 (cross-reference, no new BCs needed)

| Test (file::fn) | Already covered by |
|---|---|
| registry::rejects_unknown_schema_version | BC-AUDIT-001 |
| registry::rejects_invalid_tool_regex | BC-AUDIT-002 |
| registry::rejects_unknown_entry_field | BC-AUDIT-003 |
| registry::load_resolves_relative_plugin_paths_against_registry_dir | BC-AUDIT-004 |
| registry::resolve_plugin_paths_is_idempotent_for_absolute_paths | BC-AUDIT-004 |
| routing::match_filters_by_event_name | BC-AUDIT-005 |
| routing::match_skips_disabled_entries | BC-AUDIT-005 |
| routing::match_includes_no_tool_entries_for_any_tool | BC-AUDIT-005 |
| routing::match_respects_tool_regex_anchoring | BC-AUDIT-005 |
| routing::match_regex_alternation | BC-AUDIT-005 |
| routing::group_orders_tiers_ascending | BC-AUDIT-006 |
| routing::group_keeps_registry_order_within_tier | BC-AUDIT-006 |
| routing::group_packs_multiple_entries_at_same_priority | BC-AUDIT-006 |
| payload::rejects_missing_event_name | BC-AUDIT-007 |
| payload::rejects_empty_event_name | BC-AUDIT-007 |
| payload::rejects_empty_session_id | BC-AUDIT-007 |
| payload::rejects_malformed_json | BC-AUDIT-007 |
| payload::accepts_hook_event_name_alias_from_real_harness | BC-AUDIT-008 |
| invoke::invoke_normal_plugin_returns_ok | BC-AUDIT-012 |
| invoke::invoke_with_infinite_loop_times_out_on_epoch | BC-AUDIT-009 |
| invoke::invoke_fuel_hog_runs_out_of_fuel | BC-AUDIT-010 |
| invoke::invoke_panic_plugin_reports_crashed | BC-AUDIT-011 |
| invoke::invoke_records_elapsed_and_fuel_on_ok | BC-AUDIT-012 |
| executor::plugin_requests_block_detects_tagged_json | BC-AUDIT-017 |
| executor::plugin_requests_block_false_for_continue | BC-AUDIT-017 |
| executor::plugin_requests_block_false_for_crash | BC-AUDIT-017 |
| executor::plugin_requests_block_false_for_timeout | BC-AUDIT-017 |
| engine::builds_engine_with_epoch_and_fuel | BC-AUDIT-020 |
| engine::ticker_advances_epoch_over_time | BC-AUDIT-021 |
| engine::ticker_shutdown_is_idempotent | BC-AUDIT-021 |
| engine::timeout_ms_to_epochs_rounds_up | BC-AUDIT-022 |
| host::env::allow_list_grants_access | BC-AUDIT-029 / BC-AUDIT-030 |
| host::env::denial_for_missing_allow_list | BC-AUDIT-029 |
| host::log::unknown_level_falls_back_to_info | BC-AUDIT-033 |
| host::exec_subprocess::denies_without_capability_block | BC-AUDIT-023 |
| host::exec_subprocess::denies_binary_not_on_allow_list | BC-AUDIT-024 |
| host::exec_subprocess::denies_shell_without_acknowledgment | BC-AUDIT-025 |
| host::exec_subprocess::envelope_encodes_expected_shape | BC-AUDIT-028 |
| internal_log::auto_creates_missing_parent_dirs | BC-AUDIT-037 |
| internal_log::silently_swallows_errors_on_read_only_dir | BC-AUDIT-035 |
| internal_log::daily_rotation_writes_separate_files_per_date | BC-AUDIT-036 |
| internal_log::prune_removes_files_older_than_max_age | BC-AUDIT-038 |
| internal_log::prune_is_no_op_when_dir_missing | BC-AUDIT-039 |
| internal_log::event_fields_flatten_to_top_level | BC-AUDIT-040 |
| sinks::mod::empty_registry_submit_is_a_noop | BC-AUDIT-041 |
| sinks::mod::load_warns_on_unknown_sink_type_but_still_succeeds | BC-AUDIT-042 |
| sinks::mod::load_rejects_unsupported_schema_version | BC-AUDIT-043 |
| sinks::mod::load_builds_otel_grpc_sink_from_parsed_config | BC-AUDIT-049 |
| sinks::router::router_delegates_to_empty_registry | BC-AUDIT-123 |
| sink-core::routing_filter_allow_list_only_accepts_listed | BC-AUDIT-044 |
| sink-core::routing_filter_deny_list_only_rejects_listed | BC-AUDIT-044 |
| sink-core::routing_filter_both_lists_allow_first_then_deny | BC-AUDIT-044 |
| sink-core::routing_filter_empty_event_type_rejected_when_filtered | BC-AUDIT-044 |
| sink-core::sink_event_serializes_as_flat_object | BC-AUDIT-045 |
| sink-file::template_unknown_placeholder_errors | BC-AUDIT-046 |
| sink-otel-grpc::config_deserializes_with_defaults | BC-AUDIT-049 / BC-AUDIT-137 (defaults) |
| sink-otel-grpc::config_deserializes_with_batch_overrides | BC-AUDIT-137 |
| sink-otel-grpc::routing_filter_drops_unmatched_events | BC-AUDIT-044 + BC-AUDIT-143 (shutdown branch) |
| sink-otel-grpc::batch_size_triggers_flush | BC-AUDIT-137 |
| sink-otel-grpc::batch_interval_triggers_flush | BC-AUDIT-137 |
| sink-otel-grpc::shutdown_drains_queued_events_to_failure_log | BC-AUDIT-143 |
| sink-otel-grpc::invalid_endpoint_url_errors_at_construction | BC-AUDIT-139 |
| tests/sinks_otel_grpc::batch_size_trigger_flushes_without_explicit_flush_call | BC-AUDIT-137 |
| tests/sinks_otel_grpc::endpoint_unreachable_records_failure_without_panicking | BC-AUDIT-138 |
| tests/sinks_otel_grpc::flush_after_unreachable_returns_ok_but_records_failure | BC-AUDIT-142 |
| tests/routing_integration::load_from_disk_and_route_pretooluse_edit | BC-AUDIT-005 / BC-AUDIT-006 |
| tests/routing_integration::load_from_disk_and_route_pretooluse_bash | BC-AUDIT-005 / BC-AUDIT-006 |
| tests/routing_integration::disabled_entries_do_not_match | BC-AUDIT-005 |
| tests/routing_integration::missing_registry_surfaces_not_found_error | BC-AUDIT-2307 (this round) — duplicate of BC-AUDIT-2307 |
| tests/loads_legacy_registry::loads_generated_registry_from_disk | BC-AUDIT-061 |
| tests/internal_log_integration::startup_flow_writes_parseable_jsonl | partial cover BC-AUDIT-035..040; new BC-AUDIT-2342 above for the integration shape |
| plugin_loader::not_found_error_for_missing_path | BC-AUDIT-122 |
| plugin_loader::compiles_on_first_use_and_caches | BC-AUDIT-119 / BC-AUDIT-120 |
| plugin_loader::invalidates_on_mtime_change | BC-AUDIT-119 |
| plugin_loader::corrupt_bytes_produce_compile_error | BC-AUDIT-122 |
| hook-sdk::result::continue_serializes_with_outcome_tag | BC-AUDIT-050 |
| hook-sdk::result::block_serializes_with_reason | BC-AUDIT-050 |
| hook-sdk::result::error_serializes_with_message | BC-AUDIT-050 |
| hook-sdk::result::round_trip_block | BC-AUDIT-050 |
| hook-sdk::result::exit_codes_match_blocking_contract | BC-AUDIT-051 |
| hook-sdk::payload::plugin_config_defaults_to_null_when_missing | BC-AUDIT-053 |
| hook-sdk::host::host_error_code_mapping | BC-AUDIT-127 |
| hook-sdk::host::decode_subprocess_result_rejects_truncated | BC-AUDIT-128 |
| legacy-bash-adapter::errors_when_plugin_config_missing_script_path | BC-AUDIT-054 / BC-AUDIT-133 |
| legacy-bash-adapter::errors_when_script_path_not_a_string | BC-AUDIT-133 |
| legacy-bash-adapter::errors_when_script_path_empty | BC-AUDIT-133 |
| legacy-bash-adapter::maps_exit_zero_to_continue | BC-AUDIT-056 / BC-AUDIT-132 |
| legacy-bash-adapter::maps_exit_two_to_block_with_first_stderr_line | BC-AUDIT-056 / BC-AUDIT-132 |
| legacy-bash-adapter::maps_exit_two_with_no_stderr_to_synthetic_block_reason | BC-AUDIT-132 |
| legacy-bash-adapter::maps_other_nonzero_to_error_with_stderr | BC-AUDIT-132 |
| legacy-bash-adapter::surfaces_runner_error_as_hook_error | BC-AUDIT-132 |
| legacy-bash-adapter::is_absolute_recognizes_unix_root | BC-AUDIT-135 |
| legacy-bash-adapter::is_absolute_recognizes_windows_drive | BC-AUDIT-135 |
| legacy-bash-adapter::join_path_inserts_separator_when_missing | BC-AUDIT-135 |
| legacy-bash-adapter::join_path_respects_existing_trailing_separator | BC-AUDIT-135 |
| legacy-bash-adapter::join_path_with_empty_root_returns_relative | BC-AUDIT-135 |

A handful of duplicates surfaced (e.g., `tests/routing_integration::missing_registry_surfaces_not_found_error` covers the same ground as new BC-AUDIT-2307 — redirected to the unit test). The overlap is intentional: integration tests pin the publicly-exposed library surface while unit tests pin internal contracts.

### 2.9 — Coverage gaps (tests covering behavior not yet in any BC)

None. All 185 tests have been mapped either to existing BC-AUDIT-001..143 OR to new BC-AUDIT-2300..2389.

A separate question: **which behaviors are NOT covered by any test?** That's a different inventory (architectural drift, undocumented invariants), and is the proper subject of Pass 4 (NFR catalog) and Pass 6 (synthesis), not this round.

## 3. Delta Summary

- **Tests recounted:** 185 (vs broad-sweep claim of 180 — discrepancy traced to `#[tokio::test(flavor = "current_thread")]` regex miss)
- **NEW BCs added:** 90 (BC-AUDIT-2300 .. BC-AUDIT-2389)
- **Tests cross-referenced to existing BCs (no new BC):** ~95 tests already covered by BC-AUDIT-001..143
- **Tests with multiple BCs:** 0 (rare; all tests map 1:1 to one BC)
- **Coverage gaps in tests:** 0 (every test now has a BC owner)
- **New retractions / refinements of broad-sweep BCs:** none (no broad-sweep error surfaced this round)
- **Pass 0 inventory drift:** "13 trybuild tests" claim for hook-sdk-macros is unsupported — no `tests/` dir, no trybuild invocation. Recommended for synthesis-pass cleanup; not blocking this round.

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 |
| **Round** | Deep-rust-tests (per-test BC extraction) |
| **Novelty score** | SUBSTANTIVE |
| **Trajectory** | Round 1 of deep extraction. 90 net-new per-instance BCs derived directly from test assertions. Each new BC is grounded in a specific test function with a cited line range. The class-level BCs in BC-AUDIT-001..143 covered "this kind of behavior happens"; this round pins "this exact assertion encodes this exact contract", which is what brownfield-ingest needs to crystallize a spec sufficient to rebuild the system. |
| **Verdict** | FINDINGS_REMAIN — but only weakly. The 90 new BCs cover every test, which makes downstream ingestion much sharper. Future rounds may want to (a) sample a 5-10% subset of validate-* hooks for per-instance bash-script BCs, and (b) extract per-validator BCs from `plugins/vsdd-factory/tests/regression-v1.0.bats`. Both are out of scope this round. |

**Verdict justification:** Would removing this round's findings change how downstream skills would spec the system? **YES**. Per-test BCs make every test a regression gate that the rebuilt-from-spec system must reproduce. Without per-instance BCs, the spec-from-BC pipeline could only require "validate exit codes correctly" — with them, it requires "exit 0 on continue, exit 2 on block, exit 1 on error AND the JSON envelope serializes with `outcome` tag AND empty stderr is omitted from lifecycle events AND…". That latter shape is what rebuilds the system. The former is only enough to make a passable approximation.

**Strict-binary check:** The findings are not nitpicks. Every BC names a unique test, cites a unique source line range, and encodes a unique assertion. Removing any one of the 90 BCs leaves a hole that the rebuilt system could fail in an undetected way.

## 5. Convergence Declaration

Per-test BC extraction is **substantively complete** for `crates/`. No further deepening rounds are needed for the Rust workspace test-derived BCs unless new tests are added.

Future rounds may target:
- **bats regressions** (per-test BCs from `plugins/vsdd-factory/tests/regression-v1.0.bats` — 11 tests not yet pinned)
- **per-validate-*.sh BCs** (sample 5–8 of the 22 validators for per-instance behavior)
- **`generate-registry.bats`** (6 tests not yet pinned)

Those are workflow-side / shell-side BCs and are out of scope for this Rust-only round.

## State Checkpoint

```yaml
pass: 3
round: deep-rust-tests
status: complete
new_bcs: 90
new_bc_range: BC-AUDIT-2300..BC-AUDIT-2389
tests_recounted: 185
tests_already_covered: 95
tests_with_new_bcs: 90
tests_with_multiple_bcs: 0
coverage_gaps: 0
broad_sweep_corrections: 1 (test count 180 → 185; root cause: regex missed `#[tokio::test(flavor=...)]`)
pass_0_inventory_drift_flagged: 1 ("13 trybuild tests" in hook-sdk-macros — no source for this claim)
timestamp: 2026-04-25
novelty: SUBSTANTIVE
next_action: deep-bash-tests round (regression-v1.0.bats + generate-registry.bats + sample of validate-*.sh) — out of scope for this round
```
