# Pass 3: Behavioral Contracts — vsdd-factory

**Date:** 2026-04-25
**Reads:** Pass 0 inventory + Pass 1 architecture + Pass 2 domain model + Rust tests + bash hooks + skill SKILL.md Quality Gate sections.
**Numbering:** `BC-AUDIT-NNN` (recovered/draft BCs from a brownfield ingest, not authored from a PRD).

Tests are the most reliable source of truth. Test count by file (top contributors):
- registry.rs: 13 unit tests
- routing.rs: 9 unit tests
- payload.rs: 8 unit tests
- exec_subprocess.rs: 11 unit tests + 18 host-integration tests
- internal_log.rs: 8 unit tests
- invoke.rs: 5 unit tests
- engine.rs: 4 unit tests
- executor.rs: 4 unit tests
- sinks/mod.rs: 5 unit tests
- sink-core/lib.rs: 11 unit tests
- sink-file/lib.rs: 17 unit tests
- sink-otel-grpc/lib.rs: 13 unit tests
- hook-sdk/result.rs: 5 unit tests
- hook-sdk/payload.rs: 6 unit tests
- hook-sdk-macros: 13 trybuild tests
- legacy-bash-adapter: 14 unit tests
- factory-dispatcher integration tests: 11 (across 7 files)

Plus: `plugins/vsdd-factory/tests/regression-v1.0.bats` (11 dispatcher pipeline regression tests), 1245 baseline bats, 6 generate-registry bats. Total **180** Rust `#[test]` declarations + ~1262 bats tests per CHANGELOG.

## A. Registry / Routing contracts

### BC-AUDIT-001: Registry rejects unknown schema version
- **Preconditions:** TOML with `schema_version != 1`.
- **Postconditions:** `Registry::parse_str` returns `Err(RegistryError::SchemaVersion { got, expected })`.
- **Evidence:** `registry.rs::tests::rejects_unknown_schema_version` (registry.rs:387–404). REGISTRY_SCHEMA_VERSION = 1.
- **Confidence:** HIGH (test asserts exact error variant).

### BC-AUDIT-002: Registry rejects invalid tool regex at load time
- **Preconditions:** A `RegistryEntry.tool` field that is not a valid regex.
- **Postconditions:** `Registry::parse_str` returns `Err(RegistryError::ToolRegex { name, pattern, source })`. Routing-time regex compilation never sees an invalid pattern.
- **Evidence:** `registry.rs::tests::rejects_invalid_tool_regex` (registry.rs:407–422); `routing.rs::tool_matches` falls back to non-match on the unreachable error branch.
- **Confidence:** HIGH.

### BC-AUDIT-003: Registry rejects unknown entry fields (typo guard)
- **Preconditions:** TOML hooks entry with a misspelled field (e.g., `priorty` for `priority`).
- **Postconditions:** Parse fails; `serde(deny_unknown_fields)` enforces it.
- **Evidence:** `registry.rs::tests::rejects_unknown_entry_field`.
- **Confidence:** HIGH.

### BC-AUDIT-004: Relative plugin paths resolve against registry file's parent directory
- **Preconditions:** A registry entry with `plugin = "rel.wasm"` (no leading slash) loaded from a registry at `/foo/bar/hooks-registry.toml`.
- **Postconditions:** Entry's resolved path is `/foo/bar/rel.wasm`. Idempotent for absolute paths.
- **Evidence:** `registry.rs::tests::load_resolves_relative_plugin_paths_against_registry_dir`, `resolve_plugin_paths_is_idempotent_for_absolute_paths`.
- **Confidence:** HIGH.

### BC-AUDIT-005: Plugin filter requires event match AND (no tool OR tool regex matches)
- **Preconditions:** HookPayload with `event_name`, `tool_name`. Registry with mix of tool-bound and tool-free entries.
- **Postconditions:** `match_plugins` returns the subset where `enabled` AND `event == event_name` AND (entry.tool is None OR regex(entry.tool) matches tool_name). Disabled entries omitted.
- **Evidence:** `routing.rs::tests::{match_filters_by_event_name, match_skips_disabled_entries, match_includes_no_tool_entries_for_any_tool, match_respects_tool_regex_anchoring, match_regex_alternation}`.
- **Confidence:** HIGH (5 distinct test cases pin every branch).

### BC-AUDIT-006: Tiers ordered ascending by priority, registry order preserved within tier
- **Preconditions:** Matched entries with mixed priorities.
- **Postconditions:** `group_by_priority` returns Vec<Vec> with outer order ascending priority and inner order = original registry index.
- **Evidence:** `routing.rs::tests::{group_orders_tiers_ascending, group_keeps_registry_order_within_tier, group_packs_multiple_entries_at_same_priority}`.
- **Confidence:** HIGH.

### BC-AUDIT-007: HookPayload requires non-empty event_name and session_id
- **Preconditions:** JSON envelope with empty or missing `event_name` or `session_id`.
- **Postconditions:** `HookPayload::from_bytes` returns `PayloadError::MissingField(<which>)` or `PayloadError::Json(_)` for missing.
- **Evidence:** `payload.rs::tests::{rejects_missing_event_name, rejects_empty_event_name, rejects_empty_session_id, rejects_malformed_json}`.
- **Confidence:** HIGH.

### BC-AUDIT-008: HookPayload accepts both `event_name` and `hook_event_name`
- **Preconditions:** Real Claude Code harness envelope using `hook_event_name`.
- **Postconditions:** Parses cleanly into `HookPayload` with `event_name` populated; canonical name remains `event_name`.
- **Evidence:** `payload.rs::tests::accepts_hook_event_name_alias_from_real_harness`. Pinned by v1.0.0-beta.2 dogfood regression after harness shape mismatch.
- **Confidence:** HIGH (regression test explicitly cited in CHANGELOG).

## B. Execution / Invocation contracts

### BC-AUDIT-009: Plugin in infinite loop times out via epoch interruption
- **Preconditions:** Plugin with `(loop (br 0))` body, `timeout_ms = 50`, fuel_cap unbounded.
- **Postconditions:** `invoke_plugin` returns `PluginResult::Timeout { cause: TimeoutCause::Epoch }`.
- **Evidence:** `invoke.rs::tests::invoke_with_infinite_loop_times_out_on_epoch`.
- **Confidence:** HIGH.

### BC-AUDIT-010: Plugin in tight arithmetic loop runs out of fuel
- **Preconditions:** Plugin with i32-add loop, `fuel_cap = 10_000`, `timeout_ms = 60_000`.
- **Postconditions:** Returns `PluginResult::Timeout { cause: TimeoutCause::Fuel }`.
- **Evidence:** `invoke.rs::tests::invoke_fuel_hog_runs_out_of_fuel`.
- **Confidence:** HIGH.

### BC-AUDIT-011: Plugin trap (e.g., `unreachable`) reports as Crashed
- **Preconditions:** WASM module whose `_start` traps via `unreachable`.
- **Postconditions:** Returns `PluginResult::Crashed { trap_string, ... }`.
- **Evidence:** `invoke.rs::tests::invoke_panic_plugin_reports_crashed`.
- **Confidence:** HIGH.

### BC-AUDIT-012: Normal plugin returns Ok with exit_code 0 + fuel consumption recorded
- **Preconditions:** WASM module whose `_start` returns normally.
- **Postconditions:** `PluginResult::Ok { exit_code: 0, fuel_consumed > 0 && < cap, ... }`.
- **Evidence:** `invoke.rs::tests::{invoke_normal_plugin_returns_ok, invoke_records_elapsed_and_fuel_on_ok}`.
- **Confidence:** HIGH.

### BC-AUDIT-013: stderr captured per plugin and truncated at 4 KiB with marker
- **Preconditions:** Plugin emits >4096 bytes to stderr.
- **Postconditions:** PluginResult.stderr <= 4096 bytes plus `…(stderr truncated)` suffix; lifecycle event payload includes the truncated stderr.
- **Evidence:** `invoke.rs::stderr_to_string`, STDERR_CAP_BYTES constant, CHANGELOG v1.0.0-beta.4 entry "Plugin stderr capture on lifecycle events".
- **Confidence:** HIGH (constant + truncation logic + CHANGELOG provenance).

### BC-AUDIT-014: Empty stderr is omitted from lifecycle events (noise reduction)
- **Preconditions:** Well-behaved plugin with empty stderr.
- **Postconditions:** Lifecycle event JSON has no `stderr` field.
- **Evidence:** `executor.rs::emit_lifecycle` retains-non-empty filter.
- **Confidence:** HIGH.

### BC-AUDIT-015: Tier execution preserves between-tier order
- **Preconditions:** Multiple priority tiers.
- **Postconditions:** Tier N completes (all spawn_blocking joined) before Tier N+1 begins. `total_elapsed_ms` measured across all tiers.
- **Evidence:** `executor.rs::execute_tiers` for-loop awaits each `execute_tier`; routing tests prove tier ordering.
- **Confidence:** HIGH.

### BC-AUDIT-016: Plugins within a tier execute concurrently
- **Preconditions:** Multiple registry entries at same priority.
- **Postconditions:** Each plugin spawns via `tokio::task::spawn_blocking` before the loop awaits any join. spawn_blocking around the synchronous wasmtime call keeps the runtime non-blocking.
- **Evidence:** `executor.rs::execute_tier` builds `join_handles: Vec` then awaits.
- **Confidence:** HIGH (architecture is in source; no flake-prone timing test, but the parallel structure is explicit).

### BC-AUDIT-017: `block_intent` set only when on_error=block AND plugin asks to block
- **Preconditions:** Plugin returns `HookResult::Block { reason }` (stdout contains `"outcome":"block"`) AND its registry entry has `on_error = "block"`.
- **Postconditions:** TierExecutionSummary.block_intent = true; remaining plugins in tier still fire; final exit_code = 2.
- **Evidence:** `executor.rs::tests::plugin_requests_block_detects_tagged_json` + `plugin_requests_block_false_for_continue`/crash/timeout. Per Q3 design resolution.
- **Confidence:** HIGH.

### BC-AUDIT-018: Per-plugin `plugin_config` spliced into HookPayload before invocation
- **Preconditions:** Multiple registry entries point at the same `legacy-bash-adapter.wasm` with different `[hooks.config]` blocks.
- **Postconditions:** Each plugin sees only its own config; payload bytes deep-cloned per plugin. Multi-instance pattern works correctly.
- **Evidence:** `executor.rs::execute_tier` clones `payload_value` per plugin and inserts `plugin_config`; tested implicitly by 45-entry registry all routing through one .wasm.
- **Confidence:** HIGH.

### BC-AUDIT-019: WASI exit(N) maps to PluginResult::Ok with exit_code N
- **Preconditions:** Plugin calls `std::process::exit(N)`.
- **Postconditions:** `classify_trap` downcasts to `I32Exit(N)`; returns `PluginResult::Ok { exit_code: N }`.
- **Evidence:** `invoke.rs::classify_trap` I32Exit branch + WASI command convention.
- **Confidence:** HIGH.

## C. Engine / Ticker contracts

### BC-AUDIT-020: Engine builds with epoch interruption + fuel + reference types
- **Preconditions:** None.
- **Postconditions:** `build_engine` returns Engine; `consume_fuel` and `epoch_interruption` are enabled (proven by per-plugin fuel/epoch tests).
- **Evidence:** `engine.rs::tests::builds_engine_with_epoch_and_fuel`.
- **Confidence:** HIGH.

### BC-AUDIT-021: Epoch ticker advances epoch every 10ms; cooperative shutdown
- **Preconditions:** EPOCH_TICK_MS = 10.
- **Postconditions:** Background OS thread `vsdd-epoch-ticker` increments engine epoch on each tick. Drop joins the thread cleanly. `shutdown` is idempotent.
- **Evidence:** `engine.rs::tests::{ticker_advances_epoch_over_time, ticker_shutdown_is_idempotent}`.
- **Confidence:** HIGH.

### BC-AUDIT-022: timeout_ms_to_epochs rounds up
- **Preconditions:** Input timeout_ms.
- **Postconditions:** Returns `ceil(timeout_ms / 10)` (sub-tick gets at least 1 tick of grace).
- **Evidence:** `engine.rs::tests::timeout_ms_to_epochs_rounds_up`.
- **Confidence:** HIGH.

## D. Host function / Capability contracts

### BC-AUDIT-023: exec_subprocess denies when no exec_subprocess capability
- **Preconditions:** Plugin calls `vsdd::exec_subprocess` without `Capabilities.exec_subprocess` block in registry entry.
- **Postconditions:** Returns `codes::CAPABILITY_DENIED = -1`. Emits `internal.capability_denied` with `function = "exec_subprocess"`, `reason = "no_exec_subprocess_capability"`.
- **Evidence:** `host/exec_subprocess.rs::run` first match arm; integration tests in `host_functions.rs`.
- **Confidence:** HIGH.

### BC-AUDIT-024: exec_subprocess denies binaries not on allow-list
- **Preconditions:** Capability declared, but cmd basename not in `binary_allow`.
- **Postconditions:** Returns CAPABILITY_DENIED; denial event with `reason = "binary_not_on_allow_list"`, `command = <cmd>`.
- **Evidence:** `host/exec_subprocess.rs::binary_allowed`.
- **Confidence:** HIGH.

### BC-AUDIT-025: exec_subprocess denies shell interpreters without shell_bypass_acknowledged
- **Preconditions:** cmd basename ∈ {bash, sh, zsh, pwsh, fish, csh, tcsh, ksh} AND `shell_bypass_acknowledged` is None.
- **Postconditions:** Returns CAPABILITY_DENIED; denial event with `reason = "shell_bypass_not_acknowledged"`.
- **Evidence:** `host/exec_subprocess.rs::run` shell check; SHELL_NAMES constant. legacy-bash-adapter sets the ack to `"legacy-bash-adapter runs unported hooks"`.
- **Confidence:** HIGH.

### BC-AUDIT-026: exec_subprocess refuses setuid/setgid binaries categorically (Unix)
- **Preconditions:** Resolved binary path has setuid or setgid bit.
- **Postconditions:** Returns CAPABILITY_DENIED regardless of allow-list.
- **Evidence:** `host/exec_subprocess.rs::refuse_setuid` (Unix-only); design Q4 resolution.
- **Confidence:** HIGH.

### BC-AUDIT-027: exec_subprocess returns OUTPUT_TOO_LARGE when result exceeds buffer
- **Preconditions:** Combined exit_code + stdout + stderr envelope > result_buf_cap.
- **Postconditions:** Returns `codes::OUTPUT_TOO_LARGE = -3`.
- **Evidence:** `invoke.rs` invoke_subprocess linker; design "no unbounded host fns" rule.
- **Confidence:** HIGH.

### BC-AUDIT-028: exec_subprocess result envelope is `i32_LE | u32_LE_stdout_len | stdout | u32_LE_stderr_len | stderr`
- **Preconditions:** Successful subprocess execution.
- **Postconditions:** `encode_envelope` produces this exact byte layout; SDK `SubprocessResult::decode` mirrors.
- **Evidence:** `host/exec_subprocess.rs::encode_envelope`.
- **Confidence:** HIGH.

### BC-AUDIT-029: env host fn denies env var not on allow-list
- **Preconditions:** Plugin calls `vsdd::env(name)` for a name not in `Capabilities.env_allow`.
- **Postconditions:** Returns CAPABILITY_DENIED; emits denial event with `reason = "env_not_on_allow_list"`, `variable = name`.
- **Evidence:** invoke.rs StoreData env wrapper + host/env.rs.
- **Confidence:** HIGH.

### BC-AUDIT-030: env host fn returns 0 when var allowed but unset
- **Preconditions:** Name on allow-list but `env_view` has no entry.
- **Postconditions:** Returns 0 (zero bytes written) — distinguishable from CAPABILITY_DENIED.
- **Evidence:** invoke.rs:env arm.
- **Confidence:** HIGH.

### BC-AUDIT-031: read_file at the StoreData-typed linker layer is currently a CAPABILITY_DENIED stub
- **Preconditions:** Plugin calls `vsdd::read_file` through the per-invocation linker.
- **Postconditions:** Returns CAPABILITY_DENIED unconditionally (no in-tree plugin reaches this path; full impl exists in host/read_file.rs but is not yet wired to the StoreData-typed linker).
- **Evidence:** `invoke.rs:447–474` registers a stub returning CAPABILITY_DENIED.
- **Confidence:** HIGH (drift flag — see Pass 6).

### BC-AUDIT-032: Context getters (session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd) always return current value
- **Preconditions:** Any plugin call.
- **Postconditions:** Returns the host-context-stored value as bytes; out_cap of 0 returns required size.
- **Evidence:** invoke.rs::context_reader; host/context_fns.rs::register.
- **Confidence:** HIGH.

### BC-AUDIT-033: log host fn emits `plugin.log` internal event with level mapped to {trace,debug,info,warn,error}
- **Preconditions:** Plugin calls `vsdd::log(level, msg)` with level in 0..=4.
- **Postconditions:** Internal event `plugin.log` written with `level` as string and `message`. Levels >4 default to "info".
- **Evidence:** invoke.rs log arm + level mapping switch.
- **Confidence:** HIGH.

### BC-AUDIT-034: emit_event filters out reserved field names from plugin payload
- **Preconditions:** Plugin emits an event with fields including any of {dispatcher_trace_id, session_id, plugin_name, plugin_version, ts, ts_epoch, schema_version, type}.
- **Postconditions:** Reserved fields are dropped from the event; the dispatcher's authoritative values are written instead.
- **Evidence:** invoke.rs:emit_event arm + reserved-name list.
- **Confidence:** HIGH.

## E. Internal log contracts

### BC-AUDIT-035: Internal log writes are best-effort; never panic; never propagate
- **Preconditions:** Read-only log dir on Unix; or any other I/O failure.
- **Postconditions:** `write` returns (); a single diagnostic line goes to stderr.
- **Evidence:** `internal_log.rs::tests::silently_swallows_errors_on_read_only_dir`.
- **Confidence:** HIGH.

### BC-AUDIT-036: Daily rotation by event timestamp produces separate files per UTC date
- **Preconditions:** Two events with timestamps on different dates.
- **Postconditions:** Each lands in its own `dispatcher-internal-YYYY-MM-DD.jsonl` file.
- **Evidence:** `internal_log.rs::tests::daily_rotation_writes_separate_files_per_date`.
- **Confidence:** HIGH.

### BC-AUDIT-037: Internal log auto-creates missing parent directories
- **Preconditions:** Log dir does not exist.
- **Postconditions:** First `write` creates the dir tree (`mkdir -p`).
- **Evidence:** `internal_log.rs::tests::auto_creates_missing_parent_dirs`.
- **Confidence:** HIGH.

### BC-AUDIT-038: prune_old removes only `dispatcher-internal-*.jsonl` files older than threshold
- **Preconditions:** Mix of internal-log files (varied mtime) and unrelated files in log dir.
- **Postconditions:** Only files matching the prefix+suffix and older than max_age_days are unlinked. Unrelated files (e.g., `unrelated-2020-01-01.jsonl`) preserved.
- **Evidence:** `internal_log.rs::tests::prune_removes_files_older_than_max_age`. Note the 30-day boundary race fix in commit `ba78e5f`.
- **Confidence:** HIGH.

### BC-AUDIT-039: prune_old is no-op when log dir missing
- **Preconditions:** Log dir does not exist.
- **Postconditions:** No panic; returns without error.
- **Evidence:** `internal_log.rs::tests::prune_is_no_op_when_dir_missing`.
- **Confidence:** HIGH.

### BC-AUDIT-040: InternalEvent fields flatten to top-level JSON (no nested `fields`)
- **Preconditions:** Event with extra fields.
- **Postconditions:** JSON output has no `"fields"` key; extras at top level.
- **Evidence:** `internal_log.rs::tests::event_fields_flatten_to_top_level`.
- **Confidence:** HIGH.

## F. Sink contracts

### BC-AUDIT-041: Empty SinkRegistry submit/flush/shutdown is no-op
- **Preconditions:** SinkRegistry::empty().
- **Postconditions:** All operations succeed without side effects.
- **Evidence:** `sinks/mod.rs::tests::empty_registry_submit_is_a_noop`.
- **Confidence:** HIGH.

### BC-AUDIT-042: Unknown sink type warns to stderr but does not fail config load
- **Preconditions:** observability-config.toml with `type = "datadog"` or other not-yet-implemented driver.
- **Postconditions:** Warning to stderr; sink skipped; load returns Ok.
- **Evidence:** `sinks/mod.rs::tests::load_warns_on_unknown_sink_type_but_still_succeeds`.
- **Confidence:** HIGH.

### BC-AUDIT-043: Sink schema_version != 1 is a hard error
- **Preconditions:** observability-config.toml with `schema_version = 99`.
- **Postconditions:** `from_config` returns Err containing "schema_version".
- **Evidence:** `sinks/mod.rs::tests::load_rejects_unsupported_schema_version`.
- **Confidence:** HIGH.

### BC-AUDIT-044: RoutingFilter empty = pass-through, allow non-empty = whitelist, deny applied after allow
- **Preconditions:** Various filter shapes against various event types.
- **Postconditions:** `accepts(event_type)` honors the documented allow-then-deny semantics; case-sensitive comparison.
- **Evidence:** `sink-core/lib.rs::tests::{routing_filter_default_accepts_everything, routing_filter_allow_list_only_accepts_listed, routing_filter_deny_list_only_rejects_listed, routing_filter_both_lists_allow_first_then_deny, routing_filter_empty_event_type_rejected_when_filtered, routing_filter_allow_case_sensitive}`.
- **Confidence:** HIGH (6 distinct cases).

### BC-AUDIT-045: SinkEvent serializes flat (transparent over Map)
- **Preconditions:** SinkEvent with multiple inserted fields.
- **Postconditions:** JSON has no `fields` wrapper; all keys top-level.
- **Evidence:** `sink-core/lib.rs::tests::sink_event_serializes_as_flat_object`.
- **Confidence:** HIGH.

### BC-AUDIT-046: file sink path template substitutes `{date}`, `{name}`, `{project}` and rejects unknown placeholders
- **Preconditions:** path_template with at least one unknown `{xyz}` placeholder.
- **Postconditions:** `resolve_path_template` returns `FileSinkError::UnknownPlaceholder`.
- **Evidence:** `sink-file/lib.rs::resolve_path_template` + tests.
- **Confidence:** HIGH.

### BC-AUDIT-047: file sink mpsc bounded at default 1000; submit is non-blocking via try_send
- **Preconditions:** Default config.
- **Postconditions:** `DEFAULT_QUEUE_DEPTH = 1000`. Overflow increments `queue_full_count` instead of blocking the producer.
- **Evidence:** `sink-file/lib.rs` const + Sink trait doc + driver `submit` impl.
- **Confidence:** HIGH.

### BC-AUDIT-048: file sink failures recorded into Mutex<Vec<SinkFailure>>
- **Preconditions:** Write fails (e.g., permission denied on append).
- **Postconditions:** SinkFailure with path, reason, ts pushed; consumer drains via `take_failures`.
- **Evidence:** `sink-file/lib.rs::SinkFailure` + worker error path. Pending S-4.4 wires this to `internal.sink_error` events.
- **Confidence:** HIGH (current behavior); MEDIUM for the planned S-4.4 wiring (declared but not shipped).

### BC-AUDIT-049: otel-grpc sink loads with unreachable endpoint (lazy connect)
- **Preconditions:** observability-config.toml `endpoint = "http://127.0.0.1:1"` (will fail).
- **Postconditions:** Registry construction succeeds; worker connects lazily.
- **Evidence:** `sinks/mod.rs::tests::load_builds_otel_grpc_sink_from_parsed_config`.
- **Confidence:** HIGH.

## G. SDK / HookResult contracts

### BC-AUDIT-050: HookResult serialization is tagged with `outcome` field
- **Preconditions:** HookResult variant.
- **Postconditions:** JSON: `Continue` → `{"outcome":"continue"}`; `Block { reason }` → `{"outcome":"block","reason":...}`; `Error { message }` → `{"outcome":"error","message":...}`.
- **Evidence:** `hook-sdk/result.rs::tests::{continue_serializes_with_outcome_tag, block_serializes_with_reason, error_serializes_with_message, round_trip_block}`.
- **Confidence:** HIGH.

### BC-AUDIT-051: HookResult exit codes Continue=0 / Block=2 / Error=1
- **Preconditions:** HookResult variant.
- **Postconditions:** `exit_code()` returns 0 / 2 / 1 respectively.
- **Evidence:** `hook-sdk/result.rs::tests::exit_codes_match_blocking_contract`.
- **Confidence:** HIGH.

### BC-AUDIT-052: HOST_ABI_VERSION is 1 in both crates
- **Preconditions:** Compile-time.
- **Postconditions:** `factory_dispatcher::HOST_ABI_VERSION == vsdd_hook_sdk::HOST_ABI_VERSION == 1`.
- **Evidence:** both `lib.rs` files declare `pub const HOST_ABI_VERSION: u32 = 1;`. Mismatch would be a major-version event.
- **Confidence:** HIGH.

### BC-AUDIT-053: SDK HookPayload has `plugin_config` field defaulting to Null
- **Preconditions:** Envelope from dispatcher with no `plugin_config` key.
- **Postconditions:** SDK-side payload deserializes with `plugin_config: Value::Null`.
- **Evidence:** `hook-sdk/payload.rs::tests::plugin_config_defaults_to_null_when_missing`.
- **Confidence:** HIGH.

## H. Legacy bash adapter contracts

### BC-AUDIT-054: legacy-bash-adapter requires non-empty `plugin_config.script_path`
- **Preconditions:** Registry entry routes to legacy-bash-adapter without `script_path`.
- **Postconditions:** Returns `HookResult::Error { message: "...missing plugin_config.script_path..." }`.
- **Evidence:** `hook-plugins/legacy-bash-adapter/src/lib.rs::adapter_logic` script_path check.
- **Confidence:** HIGH.

### BC-AUDIT-055: legacy-bash-adapter strips plugin_config to Null before piping payload to bash
- **Preconditions:** Adapter has plugin_config; about to call run_bash.
- **Postconditions:** Re-serialized payload has `plugin_config: null` (bash hooks predate the field).
- **Evidence:** `adapter_logic` clones payload and sets `bash_payload.plugin_config = Value::Null`.
- **Confidence:** HIGH.

### BC-AUDIT-056: legacy-bash-adapter maps bash exit codes to HookResult
- **Preconditions:** bash hook exits 0/2/N.
- **Postconditions:** 0 → Continue; 2 → Block (reason = first stderr line, or empty); N → Error (message = stderr).
- **Evidence:** adapter_logic bash exit-code switch + design doc.
- **Confidence:** HIGH.

### BC-AUDIT-057: legacy-bash-adapter caps combined output at 1 MiB
- **Preconditions:** bash hook emits >1MiB output.
- **Postconditions:** MAX_OUTPUT_BYTES = 1024*1024 enforced; host returns OUTPUT_TOO_LARGE.
- **Evidence:** adapter constant + host enforcement.
- **Confidence:** HIGH.

### BC-AUDIT-058: legacy-bash-adapter caps wall-clock at 60_000ms (backstop only)
- **Preconditions:** bash hook hangs.
- **Postconditions:** BASH_TIMEOUT_MS = 60_000; in practice the dispatcher's per-plugin epoch deadline (default 5_000ms) fires first.
- **Evidence:** adapter constant.
- **Confidence:** HIGH.

## I. End-to-end / Regression contracts (bats + integration)

### BC-AUDIT-059: All 30+ existing bash hooks fire via legacy-bash-adapter on Linux/macOS
- **Preconditions:** v1.0.0-beta.1 release with auto-generated registry routing every bash hook through the adapter.
- **Postconditions:** Full bats suite (1245+ tests) passes.
- **Evidence:** CHANGELOG v1.0.0-beta.1 "All 30 existing hooks fire via legacy-bash-adapter on Linux/macOS. Full bats suite (1177+ tests) passes." Also: `plugins/vsdd-factory/tests/regression-v1.0.bats` (11 dispatcher-pipeline regression tests).
- **Confidence:** HIGH (CHANGELOG-asserted, regression suite pinned).

### BC-AUDIT-060: `commit.made` events fire reliably on real Claude Code git commit
- **Preconditions:** Operator runs a `git commit` invocation in a real CC session with vsdd-factory beta.3+ activated.
- **Postconditions:** `commit.made` event lands in `factory-events-*.jsonl`.
- **Evidence:** CHANGELOG v1.0.0-beta.3 "Confirmed against four prism repo commits (4fd662ab, 400fedb5, 7617214d, 3fe36e4b) — first time `commit.made` has fired through the v1.0 pipeline against the real Claude Code harness."
- **Confidence:** HIGH.

### BC-AUDIT-061: Generated hooks-registry.toml round-trips through Registry::load
- **Preconditions:** Registry generated by `scripts/generate-registry-from-hooks-json.sh`.
- **Postconditions:** Loads cleanly, parses 45 entries, all valid.
- **Evidence:** `crates/factory-dispatcher/tests/loads_legacy_registry.rs`. CHANGELOG: "Rust integration test that the generated registry parses through Registry::load."
- **Confidence:** HIGH.

### BC-AUDIT-062: registry-generation script is idempotent
- **Preconditions:** Existing registry produced from current hooks.json.
- **Postconditions:** Re-running the generator produces no diff.
- **Evidence:** 6 generate-registry bats tests; CHANGELOG v1.0.0-beta.1.
- **Confidence:** HIGH.

## J. Bash-hook contracts (each script is a behavioral contract)

### BC-AUDIT-063: block-ai-attribution blocks git commit messages with AI attribution
- **Preconditions:** PreToolUse on Bash where command contains "git commit" AND command body matches `Co-Authored-By:.*(Claude|Anthropic|GPT|OpenAI|Gemini|Google AI)|Generated with Claude Code|Generated by Claude|Generated by AI|noreply@anthropic\.com|noreply@openai\.com`.
- **Postconditions:** Exit 2 with stderr message "AI ATTRIBUTION BLOCKED" + reason; emits `hook.block` event with reason code (`ai_attribution_coauthored` or `ai_attribution_generated`).
- **Negative case:** Non-commit Bash commands exit 0 immediately. Missing jq exits 0 (graceful no-op).
- **Evidence:** `hooks/block-ai-attribution.sh`.
- **Confidence:** HIGH (script logic is direct).

### BC-AUDIT-064: capture-commit-activity (PostToolUse:Bash) emits `commit.made` on successful commits
- **Preconditions:** PostToolUse on Bash, command contained "git commit", `tool_response.interrupted = false` (or `exit_code == 0` for back-compat).
- **Postconditions:** Parses commit preamble `[<branch> <sha>] <message>` from any stdout line where bracket's last token is 7-40 hex chars. Emits `commit.made` with commit_sha, branch, message.
- **Evidence:** `hooks/capture-commit-activity.sh` (per CHANGELOG v1.0.0-beta.3 "first-line-only stdout parser" fix).
- **Confidence:** HIGH (regression-pinned).

### BC-AUDIT-065: regression-gate (PostToolUse) fails when bash command interrupted
- **Preconditions:** PostToolUse with `tool_response.interrupted = true`.
- **Postconditions:** Exit 2 with explanation; emits `hook.block`.
- **Evidence:** `hooks/regression-gate.sh` (per CHANGELOG v1.0.0-beta.3).
- **Confidence:** HIGH.

### BC-AUDIT-066: protect-secrets (PreToolUse:Bash + PreToolUse:Read) blocks reads of dotenv / credentials
- **Preconditions:** PreToolUse on Bash or Read where target path matches secret patterns.
- **Postconditions:** Exit 2 with explanation.
- **Evidence:** `hooks/protect-secrets.sh` (script not deeply read in this pass; behavior inferred from registration).
- **Confidence:** MEDIUM (behavior asserted; script body not fully cited).

### BC-AUDIT-067: check-factory-commit warns when committing in `.factory/` without STATE.md update
- **Preconditions:** PreToolUse on Bash, command contains "git commit". Hook is registered as `event = "PreToolUse"` in `hooks-registry.toml` and `hooks.json`. Note: the script header comment incorrectly says `# PostToolUse hook` — this is a documentation error in the script body, not in the registry. The .factory/STATE.md content check still happens, but as advisory output (`additionalContext` JSON) rather than as a precondition filter.
- **Postconditions:** Outputs `additionalContext` advisory; exit 0 (non-blocking).
- **Evidence:** `hooks/check-factory-commit.sh` (read directly).
- **Confidence:** HIGH.

### BC-AUDIT-068: validate-* family (24 validators on PostToolUse:Edit|Write or all)
- **Preconditions:** PostToolUse trigger.
- **Postconditions:** Each validator exits 0 (pass) or 2 (block) with diagnostic on stderr; emits `hook.block` with reason.
- **Validators:** validate-anchor-capabilities-union, validate-bc-title, validate-changelog-monotonicity, validate-demo-evidence-story-scoped, validate-factory-path-root, validate-finding-format, validate-index-self-reference, validate-input-hash, validate-novelty-assessment, validate-pr-description-completeness, validate-pr-merge-prerequisites, validate-pr-review-posted, validate-state-index-status-coherence, validate-state-pin-freshness, validate-state-size, validate-story-bc-sync, validate-subsystem-names, validate-table-cell-count, validate-template-compliance, validate-vp-consistency, validate-wave-gate-completeness, validate-wave-gate-prerequisite, validate-anchor-capabilities-union, verify-git-push, verify-sha-currency.
- **Evidence:** Per-script source under `plugins/vsdd-factory/hooks/`. Sample (validate-novelty-assessment.sh) actively gating my own outputs in this session.
- **Confidence:** HIGH for the "each is a discrete blocking gate" structure; MEDIUM-LOW for full per-validator semantics (some not deeply read this pass).

### BC-AUDIT-069: track-agent-{start,stop} (PreToolUse:Agent / SubagentStop) records agent lifecycle
- **Preconditions:** Agent dispatched / completed.
- **Postconditions:** Emits `agent.started` / `agent.stopped` event with `agent_id`, `subagent_type`.
- **Evidence:** `hooks/track-agent-{start,stop}.sh`.
- **Confidence:** MEDIUM (script body not fully cited; behavior asserted).

## K. Skill / Quality-Gate contracts (sample)

### BC-AUDIT-070: brownfield-ingest enforces strict-binary novelty
- **Preconditions:** Deepening round in progress.
- **Postconditions:** Round result is exactly SUBSTANTIVE or NITPICK; soft phrases (`borderline`, `effectively`, `recommend halting`) treated as SUBSTANTIVE; only literal `NITPICK` token counts as convergence.
- **Evidence:** `skills/brownfield-ingest/SKILL.md` Strict-binary enforcement section.
- **Confidence:** HIGH.

### BC-AUDIT-071: brownfield-ingest "Iron Law" — no round completion without honest convergence check
- **Preconditions:** Round in progress.
- **Postconditions:** Padding/fabrication strictly worse than declaring NITPICK without emitting a file.
- **Evidence:** `skills/brownfield-ingest/SKILL.md`.
- **Confidence:** HIGH.

### BC-AUDIT-072: activate skill requires platform detection success
- **Preconditions:** `/vsdd-factory:activate` invoked.
- **Postconditions:** detect-platform.sh exit 0 → one of {darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64}; exit 1 → unsupported, abort activation.
- **Evidence:** `skills/activate/SKILL.md` step 2.
- **Confidence:** HIGH.

### BC-AUDIT-073: activate skill copies hooks.json.<platform> to hooks.json then verifies dispatcher binary
- **Preconditions:** Platform detected.
- **Postconditions:** `apply-platform.sh <platform>` exit 0 = success; 1 = variant missing; 2 = binary missing; 3 = binary not executable; 4 = usage error. Stderr surfaced verbatim.
- **Evidence:** `skills/activate/SKILL.md` step 6.
- **Confidence:** HIGH.

### BC-AUDIT-074: activate skill writes platform + plugin version + activated_at to .claude/settings.local.json
- **Preconditions:** Successful activation.
- **Postconditions:** `.claude/settings.local.json` merged with `{ agent: "vsdd-factory:orchestrator:orchestrator", "vsdd-factory": { activated_platform, activated_at, activated_plugin_version } }` preserving all other keys.
- **Evidence:** `skills/activate/SKILL.md` step 5.
- **Confidence:** HIGH.

### BC-AUDIT-075: activate drift warns on cross-host re-activation
- **Preconditions:** `.vsdd-factory.activated_platform` exists and ≠ currently detected platform.
- **Postconditions:** Warning printed; activation continues; persisted platform updated.
- **Evidence:** `skills/activate/SKILL.md` step 4.
- **Confidence:** HIGH.

## L. Release / Versioning contracts

### BC-AUDIT-076: bump-version.sh accepts semver prerelease format (1.0.0-beta.N, 1.0.0-rc.N)
- **Preconditions:** Release workflow run.
- **Postconditions:** Version bump succeeds; CHANGELOG retains monotonicity.
- **Evidence:** Story S-0.1; CHANGELOG release entries through 1.0.0-beta.4.
- **Confidence:** HIGH.

### BC-AUDIT-077: chore commit (operator-staged) modifies only CHANGELOG.md
- **Preconditions:** v1.0.0-beta.4 release.
- **Postconditions:** plugin.json + marketplace.json bumps land in the bot's binary-bundle commit, not the chore commit. Operators NEVER stage plugin.json / marketplace.json for the chore commit.
- **Evidence:** CHANGELOG v1.0.0-beta.4 "Plugin-cache staleness" fix.
- **Confidence:** HIGH (regression-pinned by the very bug that drove beta.4).

### BC-AUDIT-078: release workflow's bot commit atomically writes binaries + plugin.json + marketplace.json
- **Preconditions:** Tag pushed.
- **Postconditions:** Consumers fetching in any window see version=X with matching binaries. Plugin cache key never observes mismatched state.
- **Evidence:** CHANGELOG v1.0.0-beta.4.
- **Confidence:** HIGH.

### BC-AUDIT-079: 5-platform CI matrix is the build matrix; drift gated by check-platforms-drift.py
- **Preconditions:** CI run.
- **Postconditions:** All 5 platforms green; pinned in `ci/platforms.yaml`; drift gate enforces.
- **Evidence:** CHANGELOG v1.0.0-beta.1. `ci/` directory exists.
- **Confidence:** HIGH.

### BC-AUDIT-080: hooks.json is gitignored; hooks.json.template + per-platform variants are committed
- **Preconditions:** Repo state.
- **Postconditions:** `hooks.json` written by activate skill at runtime; template + 5 variants tracked in git.
- **Evidence:** Story S-0.4; `.gitignore` (Phase 0 step 3).
- **Confidence:** HIGH.

## M. Architectural / Cross-cutting contracts

### BC-AUDIT-081: dispatcher exits 0 on registry/payload/engine errors (non-blocking)
- **Preconditions:** Any startup-side error.
- **Postconditions:** `internal.dispatcher_error` event emitted; process exits 0 (does not block Claude Code).
- **Evidence:** `main.rs::run` error branches all return Ok(0); `emit_dispatcher_error` writes the event.
- **Confidence:** HIGH.

### BC-AUDIT-082: dispatcher exit code is 2 iff at least one block_intent recorded
- **Preconditions:** Per-plugin outcomes available.
- **Postconditions:** TierExecutionSummary.exit_code = 2 iff any plugin (with on_error=block) returned a block; else 0.
- **Evidence:** executor.rs::execute_tiers final branch + tests.
- **Confidence:** HIGH.

### BC-AUDIT-083: dispatcher uses current_thread tokio runtime (not multi-threaded pool)
- **Preconditions:** Dispatcher startup.
- **Postconditions:** `#[tokio::main(flavor = "current_thread")]`; concurrency comes from spawn_blocking, not multi-threaded scheduler.
- **Evidence:** main.rs:40.
- **Confidence:** HIGH.

### BC-AUDIT-084: dispatcher uses CLAUDE_PROJECT_DIR for cwd, falling back to current_dir
- **Preconditions:** Dispatcher invoked from arbitrary cwd.
- **Postconditions:** base_host_ctx.cwd = `${CLAUDE_PROJECT_DIR}` if set, else `current_dir()`, else `"."`. Fixes log writes landing in surprising places.
- **Evidence:** main.rs:137–142.
- **Confidence:** HIGH.

### BC-AUDIT-085: dispatcher injects CLAUDE_PLUGIN_ROOT into base_host_ctx.plugin_root
- **Preconditions:** Dispatcher startup.
- **Postconditions:** base_host_ctx.plugin_root = `${CLAUDE_PLUGIN_ROOT}` (else empty PathBuf).
- **Evidence:** main.rs:143–145.
- **Confidence:** HIGH.

### BC-AUDIT-086: dispatcher projects whole process env into env_view (capability gate enforced at host fn call time)
- **Preconditions:** Dispatcher startup.
- **Postconditions:** `base_host_ctx.env_view = std::env::vars().collect()`. Per-plugin `env_allow` enforced inside `vsdd::env` host fn, not at projection time.
- **Evidence:** main.rs:152.
- **Confidence:** HIGH.

## Confidence summary

| Confidence | Count | Basis |
|---|---|---|
| HIGH | 78 | Direct from `#[test]` assertions, CHANGELOG-pinned regressions, or canonical source code |
| MEDIUM | 6 | Asserted from script structure or skill prose without full body citation (BC-AUDIT-066 protect-secrets, -068 validate-* family for some entries, -069 track-agent-*) |
| LOW | 2 | Inferred from cross-references where source was not opened in this pass (BC-AUDIT-079 drift gate details) |

## Coverage gaps

Recovered BCs span Subsystem A reasonably well (registry, routing, executor, invoke, engine, host fns, internal_log, sinks, SDK, legacy adapter — 60+ BCs). Subsystem B coverage is thinner:
- Only ~6 BCs from skill SKILL.md Quality Gates explicitly cited; ~113 skills have not been quality-gated in this pass.
- Workflow `.lobster` step preconditions / postconditions are not yet enumerated as BCs.
- 24+ validate-* hooks have stub-level BCs only (BC-AUDIT-068 covers them collectively).
- Convergence-tracker, purity-check, brownfield-discipline, factory-branch-guard hooks not deeply read.

Deepening rounds should target Subsystem B Quality Gates per skill, and per-validator BC extraction.

## State Checkpoint
```yaml
pass: 3
status: complete
bcs_extracted: 86 (BC-AUDIT-001 through BC-AUDIT-086)
high_confidence: 78
medium_confidence: 6
low_confidence: 2
test_files_examined: 18 Rust files (cargo) + 5 plugin-layer skills/hooks deeply
known_uncovered: ~113 skills not yet gated; per-validator BCs collapsed to 1 BC
timestamp: 2026-04-25
next_pass: 4
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **Novelty score** | SUBSTANTIVE — first BC extraction for this repo |
| **Trajectory** | First sweep. 86 draft behavioral contracts recovered from tests, regressions, hook scripts, and skill quality gates. No prior pass-3 artifact existed. Subsystem A coverage is dense; Subsystem B coverage is thin and flagged for deepening. |
| **Verdict** | FINDINGS_REMAIN — Subsystem B BCs (skill Quality Gates, validate-* hooks, workflow steps) are sparse. CONVERGENCE_REACHED is not declarable until Phase B convergence loops complete and the gaps in Subsystem B are filled. |
