---
document_type: bc-index
level: L3
version: "1.0"
status: draft
producer: state-manager
timestamp: 2026-04-26T00:00:00
phase: 1.4c
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
total_bcs: 1912
traces_to: bc-id-mapping.md
---

# Behavioral Contract Index

> Master index of all 1,912 behavioral contracts across 10 subsystems.
> Source of truth for BC count, status, and subsystem assignment.

## Summary

| Subsystem | BC Prefix | Count | Shard Directory |
|-----------|-----------|-------|----------------|
| SS-01 Hook Dispatcher Core | BC-1 | 101 (99 active; 2 retired) | ss-01/ |
| SS-02 Hook SDK and Plugin ABI | BC-2 | 22 | ss-02/ |
| SS-03 Observability Sinks | BC-3 | 51 | ss-03/ |
| SS-04 Plugin Ecosystem | BC-4 | 30 | ss-04/ |
| SS-05 Pipeline Orchestration | BC-5 | 646 | ss-05/ |
| SS-06 Skill Catalog | BC-6 | 585 | ss-06/ |
| SS-07 Hook Bash Layer | BC-7 | 196 | ss-07/ |
| SS-08 Templates and Rules | BC-8 | 218 | ss-08/ |
| SS-09 Configuration and Activation | BC-9 | 5 | ss-09/ |
| SS-10 CLI Tools and Bin | BC-10 | 58 | ss-10/ |
| **Total** | | **1912** | |

## Index by subsystem

### SS-01 — Hook Dispatcher Core (BC-1)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-1.01.001](ss-01/BC-1.01.001.md) | Registry rejects unknown schema version | draft | CAP-TBD | TBD |
| [BC-1.01.002](ss-01/BC-1.01.002.md) | Registry rejects invalid tool regex at load time | draft | CAP-TBD | TBD |
| [BC-1.01.003](ss-01/BC-1.01.003.md) | Registry rejects unknown entry fields (typo guard) | draft | CAP-TBD | TBD |
| [BC-1.01.004](ss-01/BC-1.01.004.md) | Relative plugin paths resolve against registry file's parent directory | draft | CAP-TBD | TBD |
| [BC-1.01.005](ss-01/BC-1.01.005.md) | Plugin filter requires event match AND (no tool OR tool regex matches) | draft | CAP-TBD | TBD |
| [BC-1.01.006](ss-01/BC-1.01.006.md) | Tiers ordered ascending by priority, registry order preserved within tier | draft | CAP-TBD | TBD |
| [BC-1.01.007](ss-01/BC-1.01.007.md) | factory-dispatcher::registry::parses_minimal_registry — minimum-viable registry parses with one hook entry, schema_version=1, enabled defaults to true | draft | CAP-TBD | TBD |
| [BC-1.01.008](ss-01/BC-1.01.008.md) | factory-dispatcher::registry::config_defaults_to_empty_table_when_absent — missing [hooks.config] yields empty table, not None | draft | CAP-TBD | TBD |
| [BC-1.01.009](ss-01/BC-1.01.009.md) | factory-dispatcher::registry::config_block_parses_into_entry — [hooks.config] supports nested tables and string fields | draft | CAP-TBD | TBD |
| [BC-1.01.010](ss-01/BC-1.01.010.md) | factory-dispatcher::registry::defaults_applied_when_missing — omitted entry timeouts/fuel/priority/on_error fall through to Registry.defaults | draft | CAP-TBD | TBD |
| [BC-1.01.011](ss-01/BC-1.01.011.md) | factory-dispatcher::registry::rejects_unknown_on_error_value — on_error="shout" (or any non-{block,continue}) fails parse | draft | CAP-TBD | TBD |
| [BC-1.01.012](ss-01/BC-1.01.012.md) | factory-dispatcher::registry::accepts_capabilities_block — [hooks.capabilities] + nested [hooks.capabilities.exec_subprocess] + [hooks.capabilities.read_file] all parse and round-trip into typed Capabilities | draft | CAP-TBD | TBD |
| [BC-1.01.013](ss-01/BC-1.01.013.md) | factory-dispatcher::registry::overrides_priority_per_entry — per-entry priority field overrides registry default | draft | CAP-TBD | TBD |
| [BC-1.01.014](ss-01/BC-1.01.014.md) | factory-dispatcher::registry::load_returns_not_found_for_missing_path — missing registry file produces RegistryError::NotFound | draft | CAP-TBD | TBD |
| [BC-1.01.015](ss-01/BC-1.01.015.md) | factory-dispatcher::routing::group_returns_empty_for_no_matches — no-match payload yields empty tier list | draft | CAP-TBD | TBD |
| [BC-1.02.001](ss-01/BC-1.02.001.md) | HookPayload requires non-empty event_name and session_id | draft | CAP-TBD | TBD |
| [BC-1.02.002](ss-01/BC-1.02.002.md) | HookPayload accepts both `event_name` and `hook_event_name` | draft | CAP-TBD | TBD |
| [BC-1.02.003](ss-01/BC-1.02.003.md) | factory-dispatcher::payload::parses_pretooluse — PreToolUse envelope deserializes with tool_input populated, tool_response None | draft | CAP-TBD | TBD |
| [BC-1.02.004](ss-01/BC-1.02.004.md) | factory-dispatcher::payload::parses_posttooluse_with_response — PostToolUse envelope carries tool_response | draft | CAP-TBD | TBD |
| [BC-1.02.005](ss-01/BC-1.02.005.md) | factory-dispatcher::payload::accepts_session_event_without_tool_name — SessionStart parses with tool_name defaulting to "" | draft | CAP-TBD | TBD |
| [BC-1.03.001](ss-01/BC-1.03.001.md) | Plugin in infinite loop times out via epoch interruption | draft | CAP-TBD | TBD |
| [BC-1.03.002](ss-01/BC-1.03.002.md) | Plugin in tight arithmetic loop runs out of fuel | draft | CAP-TBD | TBD |
| [BC-1.03.003](ss-01/BC-1.03.003.md) | Plugin trap (e.g., `unreachable`) reports as Crashed | draft | CAP-TBD | TBD |
| [BC-1.03.004](ss-01/BC-1.03.004.md) | Normal plugin returns Ok with exit_code 0 + fuel consumption recorded | draft | CAP-TBD | TBD |
| [BC-1.03.005](ss-01/BC-1.03.005.md) | stderr captured per plugin and truncated at 4 KiB with marker | draft | CAP-TBD | TBD |
| [BC-1.03.006](ss-01/BC-1.03.006.md) | Empty stderr is omitted from lifecycle events (noise reduction) | draft | CAP-TBD | TBD |
| [BC-1.03.007](ss-01/BC-1.03.007.md) | Tier execution preserves between-tier order | draft | CAP-TBD | TBD |
| [BC-1.03.008](ss-01/BC-1.03.008.md) | Plugins within a tier execute concurrently | draft | CAP-TBD | TBD |
| [BC-1.03.009](ss-01/BC-1.03.009.md) | `block_intent` set only when on_error=block AND plugin asks to block | draft | CAP-TBD | TBD |
| [BC-1.03.010](ss-01/BC-1.03.010.md) | Per-plugin `plugin_config` spliced into HookPayload before invocation | draft | CAP-TBD | TBD |
| [BC-1.03.011](ss-01/BC-1.03.011.md) | WASI exit(N) maps to PluginResult::Ok with exit_code N | draft | CAP-TBD | TBD |
| [BC-1.03.012](ss-01/BC-1.03.012.md) | factory-dispatcher::executor (integration)::parallel_happy_path_five_plugins_one_tier — 5 plugins at same priority all return Ok and exit_code=0 | draft | CAP-TBD | TBD |
| [BC-1.03.013](ss-01/BC-1.03.013.md) | factory-dispatcher::executor (integration)::crash_does_not_affect_siblings — one Crashed plugin doesn't break sibling Ok plugins; final exit_code stays 0 | draft | CAP-TBD | TBD |
| [BC-1.03.014](ss-01/BC-1.03.014.md) | factory-dispatcher::executor (integration)::parallel_timeout_does_not_cascade — hang plugin times out at 120ms while siblings complete in parallel; wall < 2s for 4-plugin tier | draft | CAP-TBD | TBD |
| [BC-1.03.015](ss-01/BC-1.03.015.md) | factory-dispatcher::executor (integration)::multi_tier_runs_in_priority_order — tier 10 plugin executes before tier 100 plugins | draft | CAP-TBD | TBD |
| [BC-1.03.016](ss-01/BC-1.03.016.md) | factory-dispatcher::executor (integration)::empty_tier_set_returns_zero_exit_code — empty tier list yields summary with no results, exit_code=0, block_intent=false | draft | CAP-TBD | TBD |
| [BC-1.04.001](ss-01/BC-1.04.001.md) | Engine builds with epoch interruption + fuel + reference types | draft | CAP-TBD | TBD |
| [BC-1.04.002](ss-01/BC-1.04.002.md) | Epoch ticker advances epoch every 10ms; cooperative shutdown | draft | CAP-TBD | TBD |
| [BC-1.04.003](ss-01/BC-1.04.003.md) | timeout_ms_to_epochs rounds up | draft | CAP-TBD | TBD |
| [BC-1.05.001](ss-01/BC-1.05.001.md) | exec_subprocess denies when no exec_subprocess capability | draft | CAP-TBD | TBD |
| [BC-1.05.002](ss-01/BC-1.05.002.md) | exec_subprocess denies binaries not on allow-list | draft | CAP-TBD | TBD |
| [BC-1.05.003](ss-01/BC-1.05.003.md) | exec_subprocess denies shell interpreters without shell_bypass_acknowledged | draft | CAP-TBD | TBD |
| [BC-1.05.004](ss-01/BC-1.05.004.md) | exec_subprocess refuses setuid/setgid binaries categorically (Unix) | draft | CAP-TBD | TBD |
| [BC-1.05.005](ss-01/BC-1.05.005.md) | exec_subprocess returns OUTPUT_TOO_LARGE when result exceeds buffer | draft | CAP-TBD | TBD |
| [BC-1.05.006](ss-01/BC-1.05.006.md) | exec_subprocess result envelope is i32_LE then u32_LE_stdout_len then stdout then u32_LE_stderr_len then stderr | draft | CAP-TBD | TBD |
| [BC-1.05.007](ss-01/BC-1.05.007.md) | env host fn denies env var not on allow-list | draft | CAP-TBD | TBD |
| [BC-1.05.008](ss-01/BC-1.05.008.md) | env host fn returns 0 when var allowed but unset | draft | CAP-TBD | TBD |
| [BC-1.05.009](ss-01/BC-1.05.009.md) | read_file at the StoreData-typed linker layer is currently a CAPABILITY_DENIED stub | draft | CAP-TBD | TBD |
| [BC-1.05.010](ss-01/BC-1.05.010.md) | Context getters (session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd) always return current value | draft | CAP-TBD | TBD |
| [BC-1.05.011](ss-01/BC-1.05.011.md) | log host fn emits `plugin.log` internal event with level mapped to {trace,debug,info,warn,error} | draft | CAP-TBD | TBD |
| [BC-1.05.012](ss-01/BC-1.05.012.md) | emit_event enriches every emitted event with host-owned identity fields and filters reserved field names from plugin payload | draft | CAP-TBD | TBD |
| [BC-1.05.013](ss-01/BC-1.05.013.md) | factory-dispatcher::host::emit_event::decode_single_pair — length-prefixed key/value buffer with one pair round-trips through decode_fields | draft | CAP-TBD | TBD |
| [BC-1.05.014](ss-01/BC-1.05.014.md) | factory-dispatcher::host::emit_event::decode_multiple_pairs — 3-pair buffer round-trips with order preserved | draft | CAP-TBD | TBD |
| [BC-1.05.015](ss-01/BC-1.05.015.md) | factory-dispatcher::host::emit_event::decode_empty_buffer_yields_empty_vec — empty input → empty result, no error | draft | CAP-TBD | TBD |
| [BC-1.05.016](ss-01/BC-1.05.016.md) | factory-dispatcher::host::emit_event::decode_rejects_truncated_key_length — <4-byte buffer triggers Err | draft | CAP-TBD | TBD |
| [BC-1.05.017](ss-01/BC-1.05.017.md) | factory-dispatcher::host::emit_event::decode_rejects_truncated_key_bytes — declared key_len exceeds remaining buffer → Err | draft | CAP-TBD | TBD |
| [BC-1.05.018](ss-01/BC-1.05.018.md) | factory-dispatcher::host::emit_event::reserved_fields_rejected — every name in RESERVED_FIELDS is recognized by is_reserved_field | draft | CAP-TBD | TBD |
| [BC-1.05.019](ss-01/BC-1.05.019.md) | factory-dispatcher::host::emit_event::non_reserved_field_accepted — non-reserved keys (commit_sha, file_path) pass | draft | CAP-TBD | TBD |
| [BC-1.05.020](ss-01/BC-1.05.020.md) | factory-dispatcher::host::log::level_mapping_matches_sdk — level u32 0..=4 maps to {trace,debug,info,warn,error} | draft | CAP-TBD | TBD |
| [BC-1.05.021](ss-01/BC-1.05.021.md) | factory-dispatcher::host::read_file::denies_when_no_capability_block — no Capabilities.read_file block → CAPABILITY_DENIED | draft | CAP-TBD | TBD |
| [BC-1.05.022](ss-01/BC-1.05.022.md) | factory-dispatcher::host::read_file::reads_allowed_file — file under path_allow with size <= max_bytes returns its contents | draft | CAP-TBD | TBD |
| [BC-1.05.023](ss-01/BC-1.05.023.md) | factory-dispatcher::host::read_file::rejects_path_outside_allow_list — file outside any allow-list prefix → CAPABILITY_DENIED | draft | CAP-TBD | TBD |
| [BC-1.05.024](ss-01/BC-1.05.024.md) | factory-dispatcher::host::read_file::rejects_file_exceeding_max_bytes — file size > max_bytes → OUTPUT_TOO_LARGE | draft | CAP-TBD | TBD |
| [BC-1.05.025](ss-01/BC-1.05.025.md) | factory-dispatcher::host::read_file::relative_path_resolves_under_plugin_root — relative path joins under ctx.plugin_root | draft | CAP-TBD | TBD |
| [BC-1.05.026](ss-01/BC-1.05.026.md) | factory-dispatcher::host::exec_subprocess::allows_shell_with_acknowledgment — shell_bypass_acknowledged set → bash allowed (gate passes; spawn may still fail with INTERNAL_ERROR on bashless hosts) | draft | CAP-TBD | TBD |
| [BC-1.05.027](ss-01/BC-1.05.027.md) | factory-dispatcher::host::exec_subprocess::stdin_bytes_reach_subprocess — non-empty stdin_bytes is piped to bash and bash sees it on cat | draft | CAP-TBD | TBD |
| [BC-1.05.028](ss-01/BC-1.05.028.md) | factory-dispatcher::host::exec_subprocess::binary_allow_matches_basename — allow-list compares against basename, not full path | draft | CAP-TBD | TBD |
| [BC-1.05.029](ss-01/BC-1.05.029.md) | factory-dispatcher::host::exec_subprocess::is_shell_detects_interpreters — SHELL_NAMES set is bash, sh, zsh, pwsh + path variants | draft | CAP-TBD | TBD |
| [BC-1.05.030](ss-01/BC-1.05.030.md) | factory-dispatcher::host::exec_subprocess::decode_args_round_trip — encoded args buffer round-trips through decode_args | draft | CAP-TBD | TBD |
| [BC-1.05.031](ss-01/BC-1.05.031.md) | factory-dispatcher::host::exec_subprocess::decode_args_rejects_truncated_buffer — declared length > available bytes → None | draft | CAP-TBD | TBD |
| [BC-1.05.032](ss-01/BC-1.05.032.md) | factory-dispatcher::host::exec_subprocess::timeout_enforced — command exceeding timeout_ms is killed and returns TIMEOUT | draft | CAP-TBD | TBD |
| [BC-1.05.033](ss-01/BC-1.05.033.md) | factory-dispatcher::host_functions (integration)::setup_linker_registers_every_vsdd_import — setup_linker exports every named host fn in the vsdd namespace | draft | CAP-TBD | TBD |
| [BC-1.05.034](ss-01/BC-1.05.034.md) | factory-dispatcher::host_functions (integration)::wat_module_importing_host_functions_instantiates — WAT module declaring vsdd imports links and runs to completion | draft | CAP-TBD | TBD |
| [BC-1.06.001](ss-01/BC-1.06.001.md) | Internal log writes are best-effort; never panic; never propagate | draft | CAP-TBD | TBD |
| [BC-1.06.002](ss-01/BC-1.06.002.md) | Daily rotation by event timestamp produces separate files per UTC date | draft | CAP-TBD | TBD |
| [BC-1.06.003](ss-01/BC-1.06.003.md) | Internal log auto-creates missing parent directories | draft | CAP-TBD | TBD |
| [BC-1.06.004](ss-01/BC-1.06.004.md) | prune_old removes only `dispatcher-internal-*.jsonl` files older than threshold | draft | CAP-TBD | TBD |
| [BC-1.06.005](ss-01/BC-1.06.005.md) | prune_old is no-op when log dir missing | draft | CAP-TBD | TBD |
| [BC-1.06.006](ss-01/BC-1.06.006.md) | InternalEvent fields flatten to top-level JSON (no nested `fields`) | draft | CAP-TBD | TBD |
| [BC-1.06.007](ss-01/BC-1.06.007.md) | factory-dispatcher::internal_log::writes_jsonl_events_with_expected_shape — 10 events with trace_id and iteration field write 10 JSONL lines into a single rotated file | draft | CAP-TBD | TBD |
| [BC-1.06.008](ss-01/BC-1.06.008.md) | factory-dispatcher::internal_log::skips_serializing_none_optional_fields — None-valued optional fields are skipped (not serialized as null) | draft | CAP-TBD | TBD |
| [BC-1.06.009](ss-01/BC-1.06.009.md) | factory-dispatcher::internal_log (integration)::startup_flow_writes_parseable_jsonl — 4-event dispatcher startup flow round-trips through JSONL with correct envelope per event | draft | CAP-TBD | TBD |
| [BC-1.06.010](ss-01/BC-1.06.010.md) | factory-dispatcher::internal_log (integration)::write_is_best_effort_when_path_is_a_file — log_dir pointing at an existing file → write returns silently without panic | draft | CAP-TBD | TBD |
| [BC-1.07.001](ss-01/BC-1.07.001.md) | All 30+ existing bash hooks fire via legacy-bash-adapter on Linux/macOS | draft | CAP-002 | S-2.07 |
| [BC-1.07.002](ss-01/BC-1.07.002.md) | `commit.made` events fire reliably on real Claude Code git commit | draft | CAP-002 | S-2.07 |
| [BC-1.07.003](ss-01/BC-1.07.003.md) | Generated hooks-registry.toml round-trips through Registry::load | draft | CAP-002 | S-2.02 |
| [BC-1.07.004](ss-01/BC-1.07.004.md) | registry-generation script is idempotent | draft | CAP-002 | S-2.02 |
| [BC-1.07.005](ss-01/BC-1.07.005.md) | factory-dispatcher::loads_legacy_registry::every_entry_routes_through_legacy_bash_adapter — every entry in the production registry routes through legacy-bash-adapter.wasm | draft | CAP-TBD | TBD |
| [BC-1.07.006](ss-01/BC-1.07.006.md) | factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path — every entry has plugin_config.script_path matching `hooks/<name>.sh` | draft | CAP-TBD | TBD |
| [BC-1.08.001](ss-01/BC-1.08.001.md) | dispatcher exits 0 on registry/payload/engine errors (non-blocking) | draft | CAP-002 | S-2.07 |
| [BC-1.08.002](ss-01/BC-1.08.002.md) | dispatcher exit code is 2 iff at least one block_intent recorded | draft | CAP-002 | S-2.07 |
| [BC-1.08.003](ss-01/BC-1.08.003.md) | dispatcher uses current_thread tokio runtime (not multi-threaded pool) | draft | CAP-TBD | TBD |
| [BC-1.08.004](ss-01/BC-1.08.004.md) | dispatcher uses CLAUDE_PROJECT_DIR for cwd, falling back to current_dir | draft | CAP-TBD | TBD |
| [BC-1.08.005](ss-01/BC-1.08.005.md) | dispatcher injects CLAUDE_PLUGIN_ROOT into base_host_ctx.plugin_root | draft | CAP-TBD | TBD |
| [BC-1.08.006](ss-01/BC-1.08.006.md) | dispatcher projects whole process env into env_view (capability gate enforced at host fn call time) | draft | CAP-TBD | TBD |
| [BC-1.09.001](ss-01/BC-1.09.001.md) | PluginCache key is `path` only; invalidation is mtime-driven | draft | CAP-TBD | TBD |
| [BC-1.09.002](ss-01/BC-1.09.002.md) | PluginCache.get_or_compile is thread-safe via Mutex<HashMap> | draft | CAP-TBD | TBD |
| [BC-1.09.003](ss-01/BC-1.09.003.md) | PluginCache has no eviction policy — entries live for the dispatcher's process lifetime | draft | CAP-TBD | TBD |
| [BC-1.09.004](ss-01/BC-1.09.004.md) | Missing plugin path returns NotFound; corrupt bytes return Compile; IO errors carry path context | draft | CAP-TBD | TBD |
| [BC-1.10.001](ss-01/BC-1.10.001.md) | Dispatcher exposes vsdd::activated_platform() host function returning activation record platform string | **retired** | CAP-002 | S-5.01 |
| [BC-1.10.002](ss-01/BC-1.10.002.md) | Dispatcher suppresses duplicate once:true events by tracking per-event-name + per-session_id in dispatcher memory | **retired** | CAP-002 | S-5.01 |

### SS-02 — Hook SDK and Plugin ABI (BC-2)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-2.01.001](ss-02/BC-2.01.001.md) | HookResult serialization is tagged with `outcome` field | draft | CAP-009 | S-1.03 |
| [BC-2.01.002](ss-02/BC-2.01.002.md) | HookResult exit codes Continue=0 / Block=2 / Error=1 | draft | CAP-009 | S-1.03, S-3.03 |
| [BC-2.01.003](ss-02/BC-2.01.003.md) | HOST_ABI_VERSION is 1 in both crates | draft | CAP-009 | S-1.03 |
| [BC-2.01.004](ss-02/BC-2.01.004.md) | SDK HookPayload has `plugin_config` field defaulting to Null | draft | CAP-009 | S-1.03 |
| [BC-2.02.001](ss-02/BC-2.02.001.md) | Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private (`mod ffi;`) | draft | CAP-009 | S-1.03 |
| [BC-2.02.002](ss-02/BC-2.02.002.md) | Bounded host calls are mandatory — `read_file` and `exec_subprocess` REQUIRE `timeout_ms` and a byte cap | draft | CAP-009 | S-1.03 |
| [BC-2.02.003](ss-02/BC-2.02.003.md) | HostError code mapping: -1 = CapabilityDenied, -2 = Timeout, -3 = OutputTooLarge, -4 = InvalidArgument, other negative = Other(i32) | draft | CAP-009 | S-1.03 |
| [BC-2.02.004](ss-02/BC-2.02.004.md) | SubprocessResult envelope decoding is paranoid — rejects truncated input rather than panicking | draft | CAP-009 | S-1.03 |
| [BC-2.02.005](ss-02/BC-2.02.005.md) | SDK-side `read_string` re-call protocol — host returns required size; SDK reallocates and re-calls | draft | CAP-009 | S-1.03 |
| [BC-2.02.006](ss-02/BC-2.02.006.md) | SDK ffi.rs uses `#[link(wasm_import_module = "vsdd")]` on wasm32 targets, host stubs on others | draft | CAP-009 | S-1.03 |
| [BC-2.02.007](ss-02/BC-2.02.007.md) | hook-sdk::host::encode_fields_uses_length_prefix — encode_fields([(k,v)]) lays out key_len\|key\|value_len\|value LE-prefixed | draft | CAP-009 | S-1.03 |
| [BC-2.02.008](ss-02/BC-2.02.008.md) | hook-sdk::host::encode_args_round_trip — encode_args matches the same length-prefix shape with no separator | draft | CAP-009 | S-1.03 |
| [BC-2.02.009](ss-02/BC-2.02.009.md) | hook-sdk::host::decode_subprocess_result_parses_envelope — SubprocessResult envelope `i32 \| u32 \| stdout \| u32 \| stderr` decodes correctly | draft | CAP-009 | S-1.03 |
| [BC-2.02.010](ss-02/BC-2.02.010.md) | hook-sdk::host::log_levels_are_stable — LogLevel discriminants 0..=4 are pinned (Trace=0, Debug=1, Info=2, Warn=3, Error=4) | draft | CAP-009 | S-1.03 |
| [BC-2.04.001](ss-02/BC-2.04.001.md) | hook-sdk::payload::pretooluse_payload_deserializes — full envelope parses with tool_input populated, tool_response None | draft | CAP-009 | S-1.03 |
| [BC-2.04.002](ss-02/BC-2.04.002.md) | hook-sdk::payload::posttooluse_payload_with_response — SDK payload includes typed access to tool_response.exit_code | draft | CAP-009 | S-1.03 |
| [BC-2.04.003](ss-02/BC-2.04.003.md) | hook-sdk::payload::lifecycle_payload_without_tool_name — SessionStart parses with tool_name="" and tool_input is JSON null | draft | CAP-009 | S-1.03 |
| [BC-2.04.004](ss-02/BC-2.04.004.md) | hook-sdk::payload::payload_round_trip_via_serde — serialize → deserialize preserves event_name and session_id | draft | CAP-009 | S-1.03 |
| [BC-2.04.005](ss-02/BC-2.04.005.md) | hook-sdk::payload::plugin_config_passes_through_when_present — plugin_config field arrives populated when the registry sets it | draft | CAP-009 | S-1.03 |
| [BC-2.05.001](ss-02/BC-2.05.001.md) | hook-sdk::__internal::panic_message_extracts_static_str — panic of `&str` is extracted into the panic message | draft | CAP-009 | S-1.03 |
| [BC-2.05.002](ss-02/BC-2.05.002.md) | hook-sdk::__internal::panic_message_extracts_string — panic of `String` is extracted | draft | CAP-009 | S-1.03 |
| [BC-2.05.003](ss-02/BC-2.05.003.md) | hook-sdk::__internal::panic_message_falls_back_for_unknown_types — non-string panic payloads return "(no panic message)" | draft | CAP-009 | S-1.03 |

### SS-03 — Observability Sinks (BC-3)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-3.01.001](ss-03/BC-3.01.001.md) | Empty SinkRegistry submit/flush/shutdown is no-op | draft | TBD | TBD |
| [BC-3.01.002](ss-03/BC-3.01.002.md) | Unknown sink type warns to stderr but does not fail config load | draft | TBD | TBD |
| [BC-3.01.003](ss-03/BC-3.01.003.md) | Sink schema_version != 1 is a hard error | draft | TBD | TBD |
| [BC-3.01.004](ss-03/BC-3.01.004.md) | RoutingFilter empty = pass-through, allow non-empty = whitelist, deny applied after allow | draft | TBD | TBD |
| [BC-3.01.005](ss-03/BC-3.01.005.md) | SinkEvent serializes flat (transparent over Map) | draft | TBD | TBD |
| [BC-3.01.006](ss-03/BC-3.01.006.md) | file sink path template substitutes `{date}`, `{name}`, `{project}` and rejects unknown placeholders | draft | TBD | TBD |
| [BC-3.01.007](ss-03/BC-3.01.007.md) | file sink mpsc bounded at default 1000; submit is non-blocking via try_send | draft | TBD | TBD |
| [BC-3.01.008](ss-03/BC-3.01.008.md) | file sink failures recorded into Mutex<Vec<SinkFailure>> | draft | TBD | TBD |
| [BC-3.01.009](ss-03/BC-3.01.009.md) | otel-grpc sink loads with unreachable endpoint (lazy connect) | draft | TBD | TBD |
| [BC-3.02.001](ss-03/BC-3.02.001.md) | sink-file::template_date_only: `{date}` substitutes to YYYY-MM-DD | draft | TBD | TBD |
| [BC-3.02.002](ss-03/BC-3.02.002.md) | sink-file::template_name_only: `{name}` substitutes to the sink's operator-assigned name | draft | TBD | TBD |
| [BC-3.02.003](ss-03/BC-3.02.003.md) | sink-file::template_project_basename: `{project}` substitutes to the basename of project_dir | draft | TBD | TBD |
| [BC-3.02.004](ss-03/BC-3.02.004.md) | sink-file::template_all_placeholders: `{project}/{name}/{date}` interpolates all three with trail | draft | TBD | TBD |
| [BC-3.02.005](ss-03/BC-3.02.005.md) | sink-file::template_no_project_yields_empty_basename: template uses {project} but None passed → | draft | TBD | TBD |
| [BC-3.02.006](ss-03/BC-3.02.006.md) | sink-file::template_unbalanced_brace_treated_literally: opening `{` without closing `}` is treate | draft | TBD | TBD |
| [BC-3.02.007](ss-03/BC-3.02.007.md) | sink-file::auto_creates_parent_directory: nested non-existent parent dirs are mkdir-p'd on first | draft | TBD | TBD |
| [BC-3.02.008](ss-03/BC-3.02.008.md) | sink-file::jsonl_append_preserves_three_events: 3 sequential events produce 3 lines in submission | draft | TBD | TBD |
| [BC-3.02.009](ss-03/BC-3.02.009.md) | sink-file::routing_filter_drops_excluded_events: allow=["commit.made"] passes 2 of 3 events through | draft | CAP-003 | DEPRECATED → BC-3.04.004 (S-4.06) |
| [BC-3.02.010](ss-03/BC-3.02.010.md) | sink-file::tag_enrichment_writes_tags_onto_every_event: configured tags `env=prod,team=factory` l | draft | CAP-003 | DEPRECATED → BC-3.04.004 (S-4.06) |
| [BC-3.02.011](ss-03/BC-3.02.011.md) | sink-file::tag_enrichment_does_not_overwrite_producer_fields: tag with key="type" does NOT clobbe | draft | CAP-003 | DEPRECATED → BC-3.04.004 (S-4.06) |
| [BC-3.02.012](ss-03/BC-3.02.012.md) | sink-file::disabled_sink_drops_every_event: enabled=false → no file written, no events accepted | draft | TBD | TBD |
| [BC-3.02.013](ss-03/BC-3.02.013.md) | sink-file::read_only_path_records_failure_without_panic: read-only target dir → SinkFailure rec | draft | TBD | TBD |
| [BC-3.02.014](ss-03/BC-3.02.014.md) | sink-file::backpressure_fills_queue_and_increments_counter: queue_depth=2 + 500 submitted events | draft | TBD | TBD |
| [BC-3.02.015](ss-03/BC-3.02.015.md) | sink-file::shutdown_drains_queued_events: shutdown() drains pending events; post-shutdown submit | draft | TBD | TBD |
| [BC-3.02.016](ss-03/BC-3.02.016.md) | sink-file::config_deserializes_from_toml: minimal TOML config parses with queue_depth defaulting | draft | TBD | TBD |
| [BC-3.03.001](ss-03/BC-3.03.001.md) | Batch trigger thresholds are independent — `size` (default 100) AND `interval_ms` (default 5000 | draft | TBD | TBD |
| [BC-3.03.002](ss-03/BC-3.03.002.md) | Send failure protocol — drop the gRPC client on error; rebuild on next batch (self-healing tran | draft | TBD | TBD |
| [BC-3.03.003](ss-03/BC-3.03.003.md) | Connection lifecycle — endpoint validated EAGERLY at constructor; channel built LAZILY in worke | draft | TBD | TBD |
| [BC-3.03.004](ss-03/BC-3.03.004.md) | Worker thread owns its own current_thread tokio runtime on a dedicated OS thread | draft | TBD | TBD |
| [BC-3.03.005](ss-03/BC-3.03.005.md) | Producer-side `submit` is fully non-blocking via `try_send`; overflow increments `queue_full_coun | draft | TBD | TBD |
| [BC-3.03.006](ss-03/BC-3.03.006.md) | `flush()` is a synchronous oneshot round-trip; producer blocks on `rx.blocking_recv()` until the | draft | TBD | TBD |
| [BC-3.03.007](ss-03/BC-3.03.007.md) | Shutdown drains and joins the worker thread; idempotent post-`accepts` rejection | draft | TBD | TBD |
| [BC-3.03.008](ss-03/BC-3.03.008.md) | sink-otel-grpc::event_to_log_record_maps_reserved_fields: SinkEvent → LogRecord lifts type→bo | draft | TBD | TBD |
| [BC-3.03.009](ss-03/BC-3.03.009.md) | sink-otel-grpc::event_attributes_flatten_non_reserved_fields: non-reserved fields flatten to OTLP | draft | TBD | TBD |
| [BC-3.03.010](ss-03/BC-3.03.010.md) | sink-otel-grpc::event_to_log_record_nested_value_serialized_to_string: nested JSON values are str | draft | TBD | TBD |
| [BC-3.03.011](ss-03/BC-3.03.011.md) | sink-otel-grpc::event_to_log_record_missing_type_yields_empty_body: producer-bug missing-type yie | draft | TBD | TBD |
| [BC-3.03.012](ss-03/BC-3.03.012.md) | sink-otel-grpc::event_to_log_record_missing_ts_yields_zero_timestamp: missing ts_epoch → time_u | draft | TBD | TBD |
| [BC-3.03.013](ss-03/BC-3.03.013.md) | sink-otel-grpc::resource_attributes_merge_defaults_with_config: operator overrides win over auto- | draft | TBD | TBD |
| [BC-3.04.001](ss-03/BC-3.04.001.md) | Router is currently a thin pass-through wrapper around SinkRegistry | draft | TBD | DEPRECATED → BC-3.04.004 |
| [BC-3.04.002](ss-03/BC-3.04.002.md) | Router exists as the future extension point for S-4.x retry / circuit-breaker / batching / routing | draft | TBD | fulfilled (S-4.06) |
| [BC-3.04.003](ss-03/BC-3.04.003.md) | Router::submit silently drops events that fail RoutingFilter; no SinkFailure recorded; debug-level log emitted | draft | CAP-003 | active |
| [BC-3.04.004](ss-03/BC-3.04.004.md) | Router::submit applies RoutingFilter before delegating to each sink (wired dispatch) | draft | CAP-003 | active |
| [BC-3.05.001](ss-03/BC-3.05.001.md) | factory-dispatcher::sinks::mod::load_builds_file_sink_from_parsed_config: ObservabilityConfig wit | draft | TBD | TBD |
| [BC-3.05.002](ss-03/BC-3.05.002.md) | factory-dispatcher::sinks_file_integration::registry_fans_events_to_file_sinks_with_filter_and_ta | draft | TBD | TBD |
| [BC-3.05.003](ss-03/BC-3.05.003.md) | factory-dispatcher::sinks_otel_grpc (integration)::ten_events_arrive_with_correct_attribute_mappi | draft | TBD | TBD |
| [BC-3.06.001](ss-03/BC-3.06.001.md) | sink-core::routing_filter_default_accepts_everything: empty allow + empty deny → every event pa | draft | TBD | TBD |
| [BC-3.06.002](ss-03/BC-3.06.002.md) | sink-core::sink_event_event_type_accessor_reads_type_field: SinkEvent.event_type() returns the "t | draft | TBD | TBD |
| [BC-3.06.003](ss-03/BC-3.06.003.md) | sink-core::sink_event_event_type_missing_returns_none: no "type" field → event_type() returns None | draft | TBD | TBD |
| [BC-3.06.004](ss-03/BC-3.06.004.md) | sink-core::sink_event_event_type_non_string_returns_none: "type" set to non-string Value → even | draft | TBD | TBD |
| [BC-3.06.005](ss-03/BC-3.06.005.md) | sink-core::sink_config_common_defaults_enabled_true: minimal SinkConfigCommon TOML defaults enabl | draft | TBD | TBD |
| [BC-3.06.006](ss-03/BC-3.06.006.md) | sink-core::routing_filter_allow_case_sensitive: allow-list compares case-sensitively (Commit.Made | draft | TBD | TBD |
| [BC-3.06.007](ss-03/BC-3.06.007.md) | sink-core::routing_filter_plugin_ids_allow — only events from listed plugins pass; empty list = pass-through | draft | CAP-003 | active |
| [BC-3.07.001](ss-03/BC-3.07.001.md) | sink-http exponential backoff with jitter between 5xx retries | draft | CAP-024 | S-4.09 |
| [BC-3.07.002](ss-03/BC-3.07.002.md) | sink driver emits `internal.sink_error` event on each recorded failure | draft | CAP-003 | S-4.10 |

### SS-04 — Plugin Ecosystem (BC-4)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-4.01.001](ss-04/BC-4.01.001.md) | legacy-bash-adapter requires non-empty `plugin_config.script_path` | draft | CAP-TBD | TBD |
| [BC-4.01.002](ss-04/BC-4.01.002.md) | legacy-bash-adapter strips plugin_config to Null before piping payload to bash | draft | CAP-TBD | TBD |
| [BC-4.01.003](ss-04/BC-4.01.003.md) | legacy-bash-adapter maps bash exit codes to HookResult | draft | CAP-TBD | TBD |
| [BC-4.01.004](ss-04/BC-4.01.004.md) | legacy-bash-adapter caps combined output at 1 MiB | draft | CAP-TBD | TBD |
| [BC-4.01.005](ss-04/BC-4.01.005.md) | legacy-bash-adapter caps wall-clock at 60_000ms (backstop only) | draft | CAP-TBD | TBD |
| [BC-4.01.006](ss-04/BC-4.01.006.md) | hook-plugins::legacy-bash-adapter::passes_payload_bytes_to_bash_with_plugin_config_stripped — re-serialized payload reaches bash with plugin_config=null while preserving event_name + dispatcher_trace_id | draft | CAP-TBD | TBD |
| [BC-4.02.001](ss-04/BC-4.02.001.md) | Adapter forwards stdout AND stderr to host log via `host::log_info` / `host::log_warn` (per-stream, non-empty) | draft | CAP-TBD | TBD |
| [BC-4.02.002](ss-04/BC-4.02.002.md) | Adapter exit-code mapping: 0 → Continue, 2 → Block (reason=first stderr line OR synthetic), other → Error (message includes script path + code + stderr) | draft | CAP-TBD | TBD |
| [BC-4.02.003](ss-04/BC-4.02.003.md) | Adapter's plugin_config.script_path validation is checked BEFORE any subprocess invocation | draft | CAP-TBD | TBD |
| [BC-4.02.004](ss-04/BC-4.02.004.md) | Adapter strips plugin_config to Null before piping to bash — bash hooks predate the field | draft | CAP-TBD | TBD |
| [BC-4.02.005](ss-04/BC-4.02.005.md) | Adapter resolves relative `script_path` under `${CLAUDE_PLUGIN_ROOT}`; absolute paths bypass the join | draft | CAP-TBD | TBD |
| [BC-4.02.006](ss-04/BC-4.02.006.md) | Adapter's wall-clock cap (BASH_TIMEOUT_MS = 60_000) is a backstop; real per-call deadline = dispatcher's epoch-interruption (default 5_000ms) | draft | CAP-TBD | TBD |
| [BC-4.03.001](ss-04/BC-4.03.001.md) | hook-plugins::capture-commit-activity::on_hook_returns_zero_in_stub — stub on_hook returns 0 (pre-S-3.1 placeholder) | draft | CAP-TBD | TBD |
| [BC-4.04.001](ss-04/BC-4.04.001.md) | session-start plugin emits session.started event with session telemetry on SessionStart event | draft | CAP-002 | S-5.01 |
| [BC-4.04.002](ss-04/BC-4.04.002.md) | session-start plugin invokes factory-health subprocess; emits session.started even if check fails | draft | CAP-002 | S-5.01 |
| [BC-4.04.003](ss-04/BC-4.04.003.md) | session-start plugin is idempotent on duplicate SessionStart events within the same session_id | draft | CAP-002 | S-5.01 |
| [BC-4.04.004](ss-04/BC-4.04.004.md) | hooks.json.template registers SessionStart event with `command` field routing to dispatcher binary; once:true and async:true | draft | CAP-002 | S-5.01 |
| [BC-4.04.005](ss-04/BC-4.04.005.md) | hooks-registry.toml registers SessionStart event routing to hook-plugins/session-start-telemetry.wasm with read_file + exec_subprocess capability tables and timeout_ms:8000 | draft | CAP-002 | S-5.01 |
| [BC-4.05.001](ss-04/BC-4.05.001.md) | session-end plugin emits session.ended event with session telemetry on SessionEnd event | draft | CAP-002 | S-5.02 |
| [BC-4.05.002](ss-04/BC-4.05.002.md) | session-end plugin does not invoke any subprocess; fast-path completion with no exec_subprocess capability | draft | CAP-002 | S-5.02 |
| [BC-4.05.003](ss-04/BC-4.05.003.md) | session-end plugin is unconditionally stateless; idempotency enforced by Layer 1 once:true directive | draft | CAP-002 | S-5.02 |
| [BC-4.05.004](ss-04/BC-4.05.004.md) | hooks.json.template registers SessionEnd event with `command` field routing to dispatcher binary; once:true and async:true | draft | CAP-002 | S-5.02 |
| [BC-4.05.005](ss-04/BC-4.05.005.md) | hooks-registry.toml registers SessionEnd event routing to hook-plugins/session-end-telemetry.wasm with timeout_ms:5000 | draft | CAP-002 | S-5.02 |
| [BC-4.07.001](ss-04/BC-4.07.001.md) | worktree-hooks plugin emits worktree.created event with {worktree_path, worktree_name} on WorktreeCreate event | draft | CAP-002 | S-5.03 |
| [BC-4.07.002](ss-04/BC-4.07.002.md) | worktree-hooks plugin emits worktree.removed event with {worktree_path} on WorktreeRemove event | draft | CAP-002 | S-5.03 |
| [BC-4.07.003](ss-04/BC-4.07.003.md) | hooks.json.template registers WorktreeCreate and WorktreeRemove events with `command` field routing to dispatcher binary; once key ABSENT (can re-fire); async:true; timeout:10000 | draft | CAP-002 | S-5.03 |
| [BC-4.07.004](ss-04/BC-4.07.004.md) | hooks-registry.toml registers WorktreeCreate and WorktreeRemove routing to hook-plugins/worktree-hooks.wasm; single crate, two entries; ZERO capability tables; timeout_ms:5000 | draft | CAP-002 | S-5.03 |
| [BC-4.08.001](ss-04/BC-4.08.001.md) | tool-failure-hooks plugin emits tool.error event with {tool_name, error_message} on PostToolUseFailure event; tool_name="unknown" if absent; error_message truncated to 2000 chars; 10-field wire payload; RESERVED_FIELDS not set by plugin | active | CAP-013 | S-5.04 |
| [BC-4.08.002](ss-04/BC-4.08.002.md) | hooks.json.template registers PostToolUseFailure with `command` routing to dispatcher binary; once key ABSENT (fires per-failure); async:true; timeout:10000 | active | CAP-013 | S-5.04 |
| [BC-4.08.003](ss-04/BC-4.08.003.md) | hooks-registry.toml registers PostToolUseFailure with name="tool-failure-hooks", event="PostToolUseFailure", plugin="hook-plugins/tool-failure-hooks.wasm", timeout_ms=5000; ZERO capability tables; NO once field | active | CAP-013 | S-5.04 |

### SS-05 — Pipeline Orchestration (BC-5)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-5.01.001](ss-05/BC-5.01.001.md) | A `.lobster` file is YAML at the top level with a single `workflow:` key | draft | CAP-TBD | TBD |
| [BC-5.01.002](ss-05/BC-5.01.002.md) | Workflow `defaults:` block sets default `on_failure`, `max_retries`, `timeout` for unspecified steps | draft | CAP-TBD | TBD |
| [BC-5.01.003](ss-05/BC-5.01.003.md) | Step taxonomy: `type:` enumerated as `skill`, `agent`, `gate`, `loop`, `human-approval`, `sub-workflow`, `parallel`, `compound` | draft | CAP-TBD | TBD |
| [BC-5.01.004](ss-05/BC-5.01.004.md) | Step ordering is by `depends_on:` topological resolution (NOT array position) | draft | CAP-TBD | TBD |
| [BC-5.01.005](ss-05/BC-5.01.005.md) | Steps SHALL declare `condition:` for conditional execution; condition is a string expression evaluated against scoped context | draft | CAP-TBD | TBD |
| [BC-5.01.006](ss-05/BC-5.01.006.md) | Failure handling — `on_failure: escalate` is the workflow default; per-step override via `on_failure: <action>`; `gate.fail_action: block` is the explicit blocking shape | draft | CAP-TBD | TBD |
| [BC-5.01.007](ss-05/BC-5.01.007.md) | `loop:` blocks are bounded; require `max_iterations` and `exit_condition` | draft | CAP-TBD | TBD |
| [BC-5.01.008](ss-05/BC-5.01.008.md) | `human-approval` steps declare `approval: { prompt, artifacts, timeout }` | draft | CAP-TBD | TBD |
| [BC-5.01.009](ss-05/BC-5.01.009.md) | `agent` steps with `model_tier:` override the default agent model assignment | draft | CAP-TBD | TBD |
| [BC-5.01.010](ss-05/BC-5.01.010.md) | `agent` steps declare `context: { include: [...], exclude: [...] }` to enforce information-asymmetry walls | draft | CAP-TBD | TBD |
| [BC-5.01.011](ss-05/BC-5.01.011.md) | Sub-workflow invocation: `type: sub-workflow` with `sub_workflow: "<filename>.lobster"` | draft | CAP-TBD | TBD |
| [BC-5.02.001](ss-05/BC-5.02.001.md) | orchestrator: never writes any files — delegates all writes | draft | CAP-TBD | TBD |
| [BC-5.02.002](ss-05/BC-5.02.002.md) | orchestrator: never delegates to itself | draft | CAP-TBD | TBD |
| [BC-5.02.003](ss-05/BC-5.02.003.md) | orchestrator: never skips per-story delivery sub-steps | draft | CAP-TBD | TBD |
| [BC-5.02.004](ss-05/BC-5.02.004.md) | orchestrator: never composes PR bodies or gh commands | draft | CAP-TBD | TBD |
| [BC-5.02.005](ss-05/BC-5.02.005.md) | orchestrator: state-manager runs LAST in every burst | draft | CAP-TBD | TBD |
| [BC-5.02.006](ss-05/BC-5.02.006.md) | orchestrator: never sets runTimeoutSeconds below 300 | draft | CAP-TBD | TBD |
| [BC-5.02.007](ss-05/BC-5.02.007.md) | orchestrator: input-hash drift check before Phase 1/2/3/7 human approval | draft | CAP-TBD | TBD |
| [BC-5.02.008](ss-05/BC-5.02.008.md) | orchestrator: prepends `cd <project-path> &&` and uses absolute paths in every dispatch | draft | CAP-TBD | TBD |
| [BC-5.02.009](ss-05/BC-5.02.009.md) | orchestrator: workspace resolution at session start (not from env var) | draft | CAP-TBD | TBD |
| [BC-5.02.010](ss-05/BC-5.02.010.md) | orchestrator: 3-clean-passes minimum for adversarial convergence | draft | CAP-TBD | TBD |
| [BC-5.02.011](ss-05/BC-5.02.011.md) | orchestrator: split bursts of >8 artifacts into create + integrate sub-bursts | draft | CAP-TBD | TBD |
| [BC-5.02.012](ss-05/BC-5.02.012.md) | orchestrator: heartbeat is read-only (no spawning, no writes) | draft | CAP-TBD | TBD |
| [BC-5.02.013](ss-05/BC-5.02.013.md) | orchestrator: pipeline resume requires factory-worktree-health BEFORE STATE.md read | draft | CAP-TBD | TBD |
| [BC-5.03.001](ss-05/BC-5.03.001.md) | accessibility-auditor: WCAG criterion citation is mandatory for every finding | draft | CAP-TBD | TBD |
| [BC-5.03.002](ss-05/BC-5.03.002.md) | accessibility-auditor: read-only — never modifies source | draft | CAP-TBD | TBD |
| [BC-5.03.003](ss-05/BC-5.03.003.md) | accessibility-auditor: skip cleanly when product has no UI | draft | CAP-TBD | TBD |
| [BC-5.03.004](ss-05/BC-5.03.004.md) | accessibility-auditor: automated tools run before manual review | draft | CAP-TBD | TBD |
| [BC-5.03.005](ss-05/BC-5.03.005.md) | accessibility-auditor: cannot load architecture files | draft | CAP-TBD | TBD |
| [BC-5.03.006](ss-05/BC-5.03.006.md) | demo-recorder: output strictly to docs/demo-evidence/<STORY-ID>/ | draft | CAP-TBD | TBD |
| [BC-5.03.007](ss-05/BC-5.03.007.md) | demo-recorder: VHS for CLI, Playwright for web — never plain text captures | draft | CAP-TBD | TBD |
| [BC-5.03.008](ss-05/BC-5.03.008.md) | demo-recorder: both success AND error paths recorded per AC | draft | CAP-TBD | TBD |
| [BC-5.03.009](ss-05/BC-5.03.009.md) | demo-recorder: every recording links to a specific AC via AC-NNN naming | draft | CAP-TBD | TBD |
| [BC-5.03.010](ss-05/BC-5.03.010.md) | demo-recorder: VHS tapes use Wait+Line, not Sleep | draft | CAP-TBD | TBD |
| [BC-5.03.011](ss-05/BC-5.03.011.md) | ux-designer: every screen traces to a PRD requirement | draft | CAP-TBD | TBD |
| [BC-5.03.012](ss-05/BC-5.03.012.md) | ux-designer: every interaction has both success AND error paths | draft | CAP-TBD | TBD |
| [BC-5.03.013](ss-05/BC-5.03.013.md) | ux-designer: sharded UX (UX-INDEX + screen + flow files), never monolithic | draft | CAP-TBD | TBD |
| [BC-5.03.014](ss-05/BC-5.03.014.md) | ux-designer: WCAG 2.1 AA documented per screen | draft | CAP-TBD | TBD |
| [BC-5.03.015](ss-05/BC-5.03.015.md) | visual-reviewer: analyzes recordings, never source | draft | CAP-TBD | TBD |
| [BC-5.03.016](ss-05/BC-5.03.016.md) | visual-reviewer: 4-dimensional satisfaction scoring (functional / visual / timing / completeness) | draft | CAP-TBD | TBD |
| [BC-5.03.017](ss-05/BC-5.03.017.md) | visual-reviewer: blank/missing demos report BLOCKED with satisfaction 0.0 | draft | CAP-TBD | TBD |
| [BC-5.03.018](ss-05/BC-5.03.018.md) | visual-reviewer: distinguishes intentional changes from regressions | draft | CAP-TBD | TBD |
| [BC-5.04.001](ss-05/BC-5.04.001.md) | adversary: cannot see prior adversarial reviews (information wall) | draft | CAP-TBD | TBD |
| [BC-5.04.002](ss-05/BC-5.04.002.md) | adversary: every finding tagged with HIGH/MEDIUM/LOW confidence | draft | CAP-TBD | TBD |
| [BC-5.04.003](ss-05/BC-5.04.003.md) | adversary: mis-anchoring always blocks convergence | draft | CAP-TBD | TBD |
| [BC-5.04.004](ss-05/BC-5.04.004.md) | adversary: minimum 3 clean passes, max 10 before human escalation | draft | CAP-TBD | TBD |
| [BC-5.04.005](ss-05/BC-5.04.005.md) | adversary: max 3 self-validation iterations per pass (AgenticAKM) | draft | CAP-TBD | TBD |
| [BC-5.04.006](ss-05/BC-5.04.006.md) | adversary: returns findings as chat text, never writes files | draft | CAP-TBD | TBD |
| [BC-5.04.007](ss-05/BC-5.04.007.md) | spec-reviewer: never re-reports adversary findings | draft | CAP-TBD | TBD |
| [BC-5.05.001](ss-05/BC-5.05.001.md) | architect: every module gets a purity boundary classification | draft | CAP-TBD | TBD |
| [BC-5.05.002](ss-05/BC-5.05.002.md) | architect: every VP has a viable proof strategy and feasibility note | draft | CAP-TBD | TBD |
| [BC-5.05.003](ss-05/BC-5.05.003.md) | architect: ARCH-INDEX must declare deployment_topology | draft | CAP-TBD | TBD |
| [BC-5.05.004](ss-05/BC-5.05.004.md) | architect: DTU assessment is mandatory and covers all 6 categories | draft | CAP-TBD | TBD |
| [BC-5.05.005](ss-05/BC-5.05.005.md) | architect: VP-INDEX changes propagate in same burst to verification-architecture.md and verification-coverage-matrix.md | draft | CAP-TBD | TBD |
| [BC-5.05.006](ss-05/BC-5.05.006.md) | architect: VP-locking is 5-step protocol, after which VP is immutable | draft | CAP-TBD | TBD |
| [BC-5.05.007](ss-05/BC-5.05.007.md) | consistency-validator: 80 criteria, none skipped | draft | CAP-TBD | TBD |
| [BC-5.05.008](ss-05/BC-5.05.008.md) | consistency-validator: index-first validation discipline | draft | CAP-TBD | TBD |
| [BC-5.05.009](ss-05/BC-5.05.009.md) | consistency-validator: gate fails when blocking findings exist | draft | CAP-TBD | TBD |
| [BC-5.05.010](ss-05/BC-5.05.010.md) | consistency-validator: mis-anchoring is never an "Observation" | draft | CAP-TBD | TBD |
| [BC-5.05.011](ss-05/BC-5.05.011.md) | formal-verifier: VP withdrawal requires architect approval | draft | CAP-TBD | TBD |
| [BC-5.05.012](ss-05/BC-5.05.012.md) | spec-reviewer: SR-NNN ID space, distinct from ADV-NNN and CR-NNN | draft | CAP-TBD | TBD |
| [BC-5.05.013](ss-05/BC-5.05.013.md) | spec-reviewer: cannot see implementation details (information wall) | draft | CAP-TBD | TBD |
| [BC-5.05.014](ss-05/BC-5.05.014.md) | spec-reviewer: 6-category finding taxonomy | draft | CAP-TBD | TBD |
| [BC-5.05.015](ss-05/BC-5.05.015.md) | spec-steward: never modifies spec content — governance only | draft | CAP-TBD | TBD |
| [BC-5.05.016](ss-05/BC-5.05.016.md) | spec-steward: every spec change requires version bump | draft | CAP-TBD | TBD |
| [BC-5.05.017](ss-05/BC-5.05.017.md) | spec-steward: locked VP enforcement (immutable after lock) | draft | CAP-TBD | TBD |
| [BC-5.05.018](ss-05/BC-5.05.018.md) | spec-steward: append-only IDs and immutable filename slugs | draft | CAP-TBD | TBD |
| [BC-5.05.019](ss-05/BC-5.05.019.md) | technical-writer: documents only current code, never aspirational | draft | CAP-TBD | TBD |
| [BC-5.05.020](ss-05/BC-5.05.020.md) | technical-writer: never modifies source/tests/configs | draft | CAP-TBD | TBD |
| [BC-5.05.021](ss-05/BC-5.05.021.md) | technical-writer: gaps in source documentation explicitly listed | draft | CAP-TBD | TBD |
| [BC-5.06.001](ss-05/BC-5.06.001.md) | business-analyst: never invents capabilities — must ground in product brief | draft | CAP-TBD | TBD |
| [BC-5.06.002](ss-05/BC-5.06.002.md) | business-analyst: produces sharded L2 (L2-INDEX + section files), never monolithic | draft | CAP-TBD | TBD |
| [BC-5.06.003](ss-05/BC-5.06.003.md) | business-analyst: include all template sections (mark N/A with justification) | draft | CAP-TBD | TBD |
| [BC-5.06.004](ss-05/BC-5.06.004.md) | business-analyst: every ASM has a validation method; every R-NNN has a mitigation | draft | CAP-TBD | TBD |
| [BC-5.06.005](ss-05/BC-5.06.005.md) | product-owner: BC-S.SS.NNN numbering scheme | draft | CAP-TBD | TBD |
| [BC-5.06.006](ss-05/BC-5.06.006.md) | product-owner: BC H1 heading is title source of truth | draft | CAP-TBD | TBD |
| [BC-5.06.007](ss-05/BC-5.06.007.md) | product-owner: append-only IDs and slugs | draft | CAP-TBD | TBD |
| [BC-5.06.008](ss-05/BC-5.06.008.md) | product-owner: every domain invariant lifted to a BC | draft | CAP-TBD | TBD |
| [BC-5.06.009](ss-05/BC-5.06.009.md) | product-owner: same-burst anchor-back when creating BCs | draft | CAP-TBD | TBD |
| [BC-5.06.010](ss-05/BC-5.06.010.md) | product-owner: subsystem ID from ARCH-INDEX, never names | draft | CAP-TBD | TBD |
| [BC-5.06.011](ss-05/BC-5.06.011.md) | story-writer: one file per story, never monolithic | draft | CAP-TBD | TBD |
| [BC-5.06.012](ss-05/BC-5.06.012.md) | story-writer: every AC traces to a BC clause; six context-engineering sections mandatory | draft | CAP-TBD | TBD |
| [BC-5.06.013](ss-05/BC-5.06.013.md) | story-writer: no story exceeds 13 points or 20-30% agent context window | draft | CAP-TBD | TBD |
| [BC-5.06.014](ss-05/BC-5.06.014.md) | story-writer: BC array changes propagate to body and ACs in same atomic commit | draft | CAP-TBD | TBD |
| [BC-5.06.015](ss-05/BC-5.06.015.md) | story-writer: dependency graph must be acyclic | draft | CAP-TBD | TBD |
| [BC-5.07.001](ss-05/BC-5.07.001.md) | code-reviewer: cannot see adversarial reviews (information wall) | draft | CAP-TBD | TBD |
| [BC-5.07.002](ss-05/BC-5.07.002.md) | code-reviewer: every finding classified into exactly one of 6 categories | draft | CAP-TBD | TBD |
| [BC-5.07.003](ss-05/BC-5.07.003.md) | code-reviewer: pass 2+ never re-reports prior findings | draft | CAP-TBD | TBD |
| [BC-5.07.004](ss-05/BC-5.07.004.md) | code-reviewer: convergence verdict line is exact format | draft | CAP-TBD | TBD |
| [BC-5.07.005](ss-05/BC-5.07.005.md) | codebase-analyzer: 6-pass protocol with per-pass output files | draft | CAP-TBD | TBD |
| [BC-5.07.006](ss-05/BC-5.07.006.md) | codebase-analyzer: never returns inline findings on Write denial | draft | CAP-TBD | TBD |
| [BC-5.07.007](ss-05/BC-5.07.007.md) | codebase-analyzer: convergence requires binary novelty (SUBSTANTIVE / NITPICK) | draft | CAP-TBD | TBD |
| [BC-5.07.008](ss-05/BC-5.07.008.md) | codebase-analyzer: convergence bounds — min 2 rounds, max 5 before escalation | draft | CAP-TBD | TBD |
| [BC-5.07.009](ss-05/BC-5.07.009.md) | codebase-analyzer: state checkpoint at end of every pass | draft | CAP-TBD | TBD |
| [BC-5.07.010](ss-05/BC-5.07.010.md) | dtu-validator: never modifies clone source — spawns implementer for fixes | draft | CAP-TBD | TBD |
| [BC-5.07.011](ss-05/BC-5.07.011.md) | dx-engineer: never logs API key values — only names + pass/fail | draft | CAP-TBD | TBD |
| [BC-5.07.012](ss-05/BC-5.07.012.md) | dx-engineer: blocks pipeline when any of 3 model families unreachable | draft | CAP-TBD | TBD |
| [BC-5.07.013](ss-05/BC-5.07.013.md) | dx-engineer: tool installation requires security-reviewer audit | draft | CAP-TBD | TBD |
| [BC-5.07.014](ss-05/BC-5.07.014.md) | dx-engineer: SHA pinning of dependencies and Docker images | draft | CAP-TBD | TBD |
| [BC-5.07.015](ss-05/BC-5.07.015.md) | e2e-tester: never mocks internal components | draft | CAP-TBD | TBD |
| [BC-5.07.016](ss-05/BC-5.07.016.md) | e2e-tester: BC-NNN traceable test naming | draft | CAP-TBD | TBD |
| [BC-5.07.017](ss-05/BC-5.07.017.md) | e2e-tester: tests are idempotent and clean up | draft | CAP-TBD | TBD |
| [BC-5.07.018](ss-05/BC-5.07.018.md) | e2e-tester: writes tests, not implementation code | draft | CAP-TBD | TBD |
| [BC-5.07.019](ss-05/BC-5.07.019.md) | formal-verifier: never marks VP verified without running proof to completion | draft | CAP-TBD | TBD |
| [BC-5.07.020](ss-05/BC-5.07.020.md) | formal-verifier: cannot see adversarial reviews (information wall) | draft | CAP-TBD | TBD |
| [BC-5.07.021](ss-05/BC-5.07.021.md) | formal-verifier: mutation kill rate enforced per module-criticality tier | draft | CAP-TBD | TBD |
| [BC-5.07.022](ss-05/BC-5.07.022.md) | formal-verifier: fuzz targets run ≥5 minutes with no crashes | draft | CAP-TBD | TBD |
| [BC-5.07.023](ss-05/BC-5.07.023.md) | formal-verifier: purity boundary audit catches I/O in pure-core | draft | CAP-TBD | TBD |
| [BC-5.07.024](ss-05/BC-5.07.024.md) | holdout-evaluator: cannot read source code, specs, or prior reviews | draft | CAP-TBD | TBD |
| [BC-5.07.025](ss-05/BC-5.07.025.md) | holdout-evaluator: gate criteria — mean ≥0.85, every critical scenario ≥0.60 | draft | CAP-TBD | TBD |
| [BC-5.07.026](ss-05/BC-5.07.026.md) | holdout-evaluator: 0.0–1.0 satisfaction scoring per scenario | draft | CAP-TBD | TBD |
| [BC-5.07.027](ss-05/BC-5.07.027.md) | holdout-evaluator: read-only — no Write tool | draft | CAP-TBD | TBD |
| [BC-5.07.028](ss-05/BC-5.07.028.md) | implementer: never writes code without a failing test (Red Gate) | draft | CAP-TBD | TBD |
| [BC-5.07.029](ss-05/BC-5.07.029.md) | implementer: minimum code per test (TDD discipline) | draft | CAP-TBD | TBD |
| [BC-5.07.030](ss-05/BC-5.07.030.md) | implementer: micro-commit per passing test, squash before PR | draft | CAP-TBD | TBD |
| [BC-5.07.031](ss-05/BC-5.07.031.md) | implementer: respects purity boundary map | draft | CAP-TBD | TBD |
| [BC-5.07.032](ss-05/BC-5.07.032.md) | implementer: HALT only on blocker, impossibility, or 3 consecutive failures | draft | CAP-TBD | TBD |
| [BC-5.07.033](ss-05/BC-5.07.033.md) | implementer: status reporting in {DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED} | draft | CAP-TBD | TBD |
| [BC-5.07.034](ss-05/BC-5.07.034.md) | performance-engineer: never modifies source code — measurement only | draft | CAP-TBD | TBD |
| [BC-5.07.035](ss-05/BC-5.07.035.md) | performance-engineer: capture baseline BEFORE changes | draft | CAP-TBD | TBD |
| [BC-5.07.036](ss-05/BC-5.07.036.md) | performance-engineer: numerical thresholds only, never qualitative | draft | CAP-TBD | TBD |
| [BC-5.07.037](ss-05/BC-5.07.037.md) | performance-engineer: every NFR-NNN gets a compliance row | draft | CAP-TBD | TBD |
| [BC-5.07.038](ss-05/BC-5.07.038.md) | security-reviewer: cite CWE/CVE for every finding | draft | CAP-TBD | TBD |
| [BC-5.07.039](ss-05/BC-5.07.039.md) | security-reviewer: 4-tier severity (CRITICAL/HIGH/MEDIUM/LOW) | draft | CAP-TBD | TBD |
| [BC-5.07.040](ss-05/BC-5.07.040.md) | security-reviewer: cannot see implementer reasoning (information wall) | draft | CAP-TBD | TBD |
| [BC-5.07.041](ss-05/BC-5.07.041.md) | security-reviewer: never dismiss without documented reasoning | draft | CAP-TBD | TBD |
| [BC-5.07.042](ss-05/BC-5.07.042.md) | security-reviewer: supply chain audit ANY finding blocks installation | draft | CAP-TBD | TBD |
| [BC-5.07.043](ss-05/BC-5.07.043.md) | security-reviewer: posts via gh pr review, never gh pr comment (per-story) | draft | CAP-TBD | TBD |
| [BC-5.07.044](ss-05/BC-5.07.044.md) | test-writer: never writes implementation code | draft | CAP-TBD | TBD |
| [BC-5.07.045](ss-05/BC-5.07.045.md) | test-writer: BC-NNN-traceable test naming required | draft | CAP-TBD | TBD |
| [BC-5.07.046](ss-05/BC-5.07.046.md) | test-writer: Red Gate must be verified — all tests fail before implementation | draft | CAP-TBD | TBD |
| [BC-5.07.047](ss-05/BC-5.07.047.md) | test-writer: never writes vacuously true tests | draft | CAP-TBD | TBD |
| [BC-5.07.048](ss-05/BC-5.07.048.md) | test-writer: property-based tests generate ≥1000 random cases | draft | CAP-TBD | TBD |
| [BC-5.07.049](ss-05/BC-5.07.049.md) | test-writer: uses canonical test vectors from BCs when available | draft | CAP-TBD | TBD |
| [BC-5.08.001](ss-05/BC-5.08.001.md) | data-engineer: every migration has both up and down scripts | draft | CAP-TBD | TBD |
| [BC-5.08.002](ss-05/BC-5.08.002.md) | data-engineer: every field has a privacy classification before schema finalization | draft | CAP-TBD | TBD |
| [BC-5.08.003](ss-05/BC-5.08.003.md) | data-engineer: pure validation logic never touches DB I/O | draft | CAP-TBD | TBD |
| [BC-5.08.004](ss-05/BC-5.08.004.md) | data-engineer: every schema traces to a BC-NNN data contract | draft | CAP-TBD | TBD |
| [BC-5.08.005](ss-05/BC-5.08.005.md) | devops-engineer: never commits secrets | draft | CAP-TBD | TBD |
| [BC-5.08.006](ss-05/BC-5.08.006.md) | devops-engineer: all GitHub Actions pinned to SHA, never tag | draft | CAP-TBD | TBD |
| [BC-5.08.007](ss-05/BC-5.08.007.md) | devops-engineer: develop branch protected with CI status checks | draft | CAP-TBD | TBD |
| [BC-5.08.008](ss-05/BC-5.08.008.md) | devops-engineer: .factory mounted as git worktree on factory-artifacts orphan branch | draft | CAP-TBD | TBD |
| [BC-5.08.009](ss-05/BC-5.08.009.md) | devops-engineer: worktree-per-story discipline (.worktrees/STORY-NNN) | draft | CAP-TBD | TBD |
| [BC-5.08.010](ss-05/BC-5.08.010.md) | github-ops: executes only — never makes decisions | draft | CAP-TBD | TBD |
| [BC-5.08.011](ss-05/BC-5.08.011.md) | github-ops: returns full stdout + stderr unmodified | draft | CAP-TBD | TBD |
| [BC-5.08.012](ss-05/BC-5.08.012.md) | github-ops: retry once on transient errors, then report | draft | CAP-TBD | TBD |
| [BC-5.08.013](ss-05/BC-5.08.013.md) | orchestrator: dispatches state-manager directly for .factory/ commits — never devops-engineer | draft | CAP-TBD | TBD |
| [BC-5.08.014](ss-05/BC-5.08.014.md) | pr-manager: 9-step coordinator, never exits mid-flow | draft | CAP-TBD | TBD |
| [BC-5.08.015](ss-05/BC-5.08.015.md) | pr-manager: delegates all gh/git commands to github-ops | draft | CAP-TBD | TBD |
| [BC-5.08.016](ss-05/BC-5.08.016.md) | pr-manager: max 10 review convergence cycles before human escalation | draft | CAP-TBD | TBD |
| [BC-5.08.017](ss-05/BC-5.08.017.md) | pr-manager: never merges with failing CI checks or unmerged dependency PRs | draft | CAP-TBD | TBD |
| [BC-5.08.018](ss-05/BC-5.08.018.md) | pr-manager: max 3 CI fix cycles before human escalation | draft | CAP-TBD | TBD |
| [BC-5.08.019](ss-05/BC-5.08.019.md) | pr-reviewer: cannot see .factory/ artifacts (information wall) | draft | CAP-TBD | TBD |
| [BC-5.08.020](ss-05/BC-5.08.020.md) | pr-reviewer: posts via `gh pr review`, never `gh pr comment` | draft | CAP-TBD | TBD |
| [BC-5.08.021](ss-05/BC-5.08.021.md) | pr-reviewer: spawns `github-ops` (exact name) for posting | draft | CAP-TBD | TBD |
| [BC-5.08.022](ss-05/BC-5.08.022.md) | pr-reviewer: 3-tier severity classification (BLOCKING / WARNING / NIT) | draft | CAP-TBD | TBD |
| [BC-5.08.023](ss-05/BC-5.08.023.md) | pr-reviewer: no rubber-stamping — explain what was verified | draft | CAP-TBD | TBD |
| [BC-5.08.024](ss-05/BC-5.08.024.md) | pr-reviewer: demo evidence in `.gif`/`.webm`, not `.txt` | draft | CAP-TBD | TBD |
| [BC-5.09.001](ss-05/BC-5.09.001.md) | dtu-validator: never use production API keys with write access | draft | CAP-TBD | TBD |
| [BC-5.09.002](ss-05/BC-5.09.002.md) | dtu-validator: fidelity thresholds enforced per L-tier | draft | CAP-TBD | TBD |
| [BC-5.09.003](ss-05/BC-5.09.003.md) | dtu-validator: drift >5% triggers stale flag and fix story | draft | CAP-TBD | TBD |
| [BC-5.09.004](ss-05/BC-5.09.004.md) | research-agent: every claim cited; never relies on training data alone | draft | CAP-TBD | TBD |
| [BC-5.09.005](ss-05/BC-5.09.005.md) | research-agent: library versions verified against registries, never training data | draft | CAP-TBD | TBD |
| [BC-5.09.006](ss-05/BC-5.09.006.md) | research-agent: mandatory Research Methods section per report | draft | CAP-TBD | TBD |
| [BC-5.09.007](ss-05/BC-5.09.007.md) | research-agent: never overwrites prior research — appends new dated file | draft | CAP-TBD | TBD |
| [BC-5.09.008](ss-05/BC-5.09.008.md) | research-agent: no source code modification, no Bash | draft | CAP-TBD | TBD |
| [BC-5.09.009](ss-05/BC-5.09.009.md) | session-reviewer: T1 read-only, NEVER writes files | draft | CAP-TBD | TBD |
| [BC-5.09.010](ss-05/BC-5.09.010.md) | session-reviewer: 8-dimensional analysis required | draft | CAP-TBD | TBD |
| [BC-5.09.011](ss-05/BC-5.09.011.md) | session-reviewer: actionable proposals, not vague observations | draft | CAP-TBD | TBD |
| [BC-5.09.012](ss-05/BC-5.09.012.md) | session-reviewer: no information walls — sees everything | draft | CAP-TBD | TBD |
| [BC-5.09.013](ss-05/BC-5.09.013.md) | session-reviewer: tracks own cost; flags >5% of pipeline run cost | draft | CAP-TBD | TBD |
| [BC-5.09.014](ss-05/BC-5.09.014.md) | validate-extraction: behavioral and metric phases must be split | draft | CAP-TBD | TBD |
| [BC-5.09.015](ss-05/BC-5.09.015.md) | validate-extraction: every numeric claim has a (claimed, recounted, delta) triple | draft | CAP-TBD | TBD |
| [BC-5.09.016](ss-05/BC-5.09.016.md) | validate-extraction: max 3 refinement iterations (AgenticAKM) | draft | CAP-TBD | TBD |
| [BC-5.09.017](ss-05/BC-5.09.017.md) | validate-extraction: 4-tier per-item disposition (VERIFIED / INACCURATE / HALLUCINATED / UNVERIFIABLE) | draft | CAP-TBD | TBD |
| [BC-5.09.018](ss-05/BC-5.09.018.md) | validate-extraction: never modifies source code | draft | CAP-TBD | TBD |
| [BC-5.09.019](ss-05/BC-5.09.019.md) | validate-extraction: >50% hallucination rate triggers Level 3 escalation | draft | CAP-TBD | TBD |
| [BC-5.10.001](ss-05/BC-5.10.001.md) | state-manager: git access scoped to .factory/ only | draft | CAP-TBD | TBD |
| [BC-5.10.002](ss-05/BC-5.10.002.md) | state-manager: never writes spec documents or source code | draft | CAP-TBD | TBD |
| [BC-5.10.003](ss-05/BC-5.10.003.md) | state-manager: STATE.md cap of 200 lines (hook blocks at 500) | draft | CAP-TBD | TBD |
| [BC-5.10.004](ss-05/BC-5.10.004.md) | state-manager: worktree preconditions verified before any .factory/ creation | draft | CAP-TBD | TBD |
| [BC-5.10.005](ss-05/BC-5.10.005.md) | state-manager: wave-gate remediation uses Single Canonical SHA + Two-Commit Protocol | draft | CAP-TBD | TBD |
| [BC-5.20.001](ss-05/BC-5.20.001.md) | phase-0-codebase-ingestion: identity | draft | CAP-070 | TBD |
| [BC-5.20.002](ss-05/BC-5.20.002.md) | phase-0-codebase-ingestion: entry-point | draft | CAP-070 | TBD |
| [BC-5.20.003](ss-05/BC-5.20.003.md) | phase-0-codebase-ingestion: terminal-step | draft | CAP-070 | TBD |
| [BC-5.20.004](ss-05/BC-5.20.004.md) | phase-0-codebase-ingestion: DAG integrity | draft | CAP-070 | TBD |
| [BC-5.20.005](ss-05/BC-5.20.005.md) | phase-0-codebase-ingestion: failure semantics | draft | CAP-070 | TBD |
| [BC-5.20.006](ss-05/BC-5.20.006.md) | phase-0:source-acquisition | draft | CAP-070 | TBD |
| [BC-5.20.007](ss-05/BC-5.20.007.md) | phase-0:backup-source-acquisition | draft | CAP-070 | TBD |
| [BC-5.20.008](ss-05/BC-5.20.008.md) | phase-0:broad-sweep | draft | CAP-070 | TBD |
| [BC-5.20.009](ss-05/BC-5.20.009.md) | phase-0:backup-broad-sweep | draft | CAP-070 | TBD |
| [BC-5.20.010](ss-05/BC-5.20.010.md) | phase-0:convergence-deepening | draft | CAP-070 | TBD |
| [BC-5.20.011](ss-05/BC-5.20.011.md) | phase-0:backup-convergence-deepening | draft | CAP-070 | TBD |
| [BC-5.20.012](ss-05/BC-5.20.012.md) | phase-0:coverage-audit | draft | CAP-070 | TBD |
| [BC-5.20.013](ss-05/BC-5.20.013.md) | phase-0:backup-coverage-audit | draft | CAP-070 | TBD |
| [BC-5.20.014](ss-05/BC-5.20.014.md) | phase-0:extraction-validation | draft | CAP-070 | TBD |
| [BC-5.20.015](ss-05/BC-5.20.015.md) | phase-0:backup-extraction-validation | draft | CAP-070 | TBD |
| [BC-5.20.016](ss-05/BC-5.20.016.md) | phase-0:final-synthesis | draft | CAP-070 | TBD |
| [BC-5.20.017](ss-05/BC-5.20.017.md) | phase-0:backup-final-synthesis | draft | CAP-070 | TBD |
| [BC-5.20.018](ss-05/BC-5.20.018.md) | phase-0:phase-0-gate | draft | CAP-070 | TBD |
| [BC-5.20.019](ss-05/BC-5.20.019.md) | phase-0:input-hash-drift-check | draft | CAP-070 | TBD |
| [BC-5.20.020](ss-05/BC-5.20.020.md) | phase-0:human-approval | draft | CAP-070 | TBD |
| [BC-5.21.001](ss-05/BC-5.21.001.md) | phase-1-spec-crystallization: identity | draft | CAP-071 | TBD |
| [BC-5.21.002](ss-05/BC-5.21.002.md) | phase-1: entry-point | draft | CAP-071 | TBD |
| [BC-5.21.003](ss-05/BC-5.21.003.md) | phase-1: terminal-step | draft | CAP-071 | TBD |
| [BC-5.21.004](ss-05/BC-5.21.004.md) | phase-1: DAG integrity | draft | CAP-071 | TBD |
| [BC-5.21.005](ss-05/BC-5.21.005.md) | phase-1: failure semantics | draft | CAP-071 | TBD |
| [BC-5.21.006](ss-05/BC-5.21.006.md) | phase-1:create-brief | draft | CAP-071 | TBD |
| [BC-5.21.007](ss-05/BC-5.21.007.md) | phase-1:backup-create-brief | draft | CAP-071 | TBD |
| [BC-5.21.008](ss-05/BC-5.21.008.md) | phase-1:create-domain-spec | draft | CAP-071 | TBD |
| [BC-5.21.009](ss-05/BC-5.21.009.md) | phase-1:backup-create-domain-spec | draft | CAP-071 | TBD |
| [BC-5.21.010](ss-05/BC-5.21.010.md) | phase-1:create-prd | draft | CAP-071 | TBD |
| [BC-5.21.011](ss-05/BC-5.21.011.md) | phase-1:backup-create-prd | draft | CAP-071 | TBD |
| [BC-5.21.012](ss-05/BC-5.21.012.md) | phase-1:create-architecture | draft | CAP-071 | TBD |
| [BC-5.21.013](ss-05/BC-5.21.013.md) | phase-1:backup-create-architecture | draft | CAP-071 | TBD |
| [BC-5.21.014](ss-05/BC-5.21.014.md) | phase-1:prd-revision | draft | CAP-071 | TBD |
| [BC-5.21.015](ss-05/BC-5.21.015.md) | phase-1:backup-prd-revision | draft | CAP-071 | TBD |
| [BC-5.21.016](ss-05/BC-5.21.016.md) | phase-1:spec-gate | draft | CAP-071 | TBD |
| [BC-5.21.017](ss-05/BC-5.21.017.md) | phase-1:adversarial-spec-review | draft | CAP-071 | TBD |
| [BC-5.21.018](ss-05/BC-5.21.018.md) | phase-1:input-hash-drift-check | draft | CAP-071 | TBD |
| [BC-5.21.019](ss-05/BC-5.21.019.md) | phase-1:human-approval | draft | CAP-071 | TBD |
| [BC-5.22.001](ss-05/BC-5.22.001.md) | phase-2-story-decomposition: identity | draft | CAP-072 | TBD |
| [BC-5.22.002](ss-05/BC-5.22.002.md) | phase-2: entry-point | draft | CAP-072 | TBD |
| [BC-5.22.003](ss-05/BC-5.22.003.md) | phase-2: terminal-step | draft | CAP-072 | TBD |
| [BC-5.22.004](ss-05/BC-5.22.004.md) | phase-2: DAG integrity | draft | CAP-072 | TBD |
| [BC-5.22.005](ss-05/BC-5.22.005.md) | phase-2: failure semantics | draft | CAP-072 | TBD |
| [BC-5.22.006](ss-05/BC-5.22.006.md) | phase-2:define-epics | draft | CAP-072 | TBD |
| [BC-5.22.007](ss-05/BC-5.22.007.md) | phase-2:backup-define-epics | draft | CAP-072 | TBD |
| [BC-5.22.008](ss-05/BC-5.22.008.md) | phase-2:create-stories | draft | CAP-072 | TBD |
| [BC-5.22.009](ss-05/BC-5.22.009.md) | phase-2:backup-create-stories | draft | CAP-072 | TBD |
| [BC-5.22.010](ss-05/BC-5.22.010.md) | phase-2:dependency-graph | draft | CAP-072 | TBD |
| [BC-5.22.011](ss-05/BC-5.22.011.md) | phase-2:backup-dependency-graph | draft | CAP-072 | TBD |
| [BC-5.22.012](ss-05/BC-5.22.012.md) | phase-2:wave-schedule | draft | CAP-072 | TBD |
| [BC-5.22.013](ss-05/BC-5.22.013.md) | phase-2:backup-wave-schedule | draft | CAP-072 | TBD |
| [BC-5.22.014](ss-05/BC-5.22.014.md) | phase-2:holdout-scenarios | draft | CAP-072 | TBD |
| [BC-5.22.015](ss-05/BC-5.22.015.md) | phase-2:backup-holdout-scenarios | draft | CAP-072 | TBD |
| [BC-5.22.016](ss-05/BC-5.22.016.md) | phase-2:decomposition-gate | draft | CAP-072 | TBD |
| [BC-5.22.017](ss-05/BC-5.22.017.md) | phase-2:adversarial-story-review | draft | CAP-072 | TBD |
| [BC-5.22.018](ss-05/BC-5.22.018.md) | phase-2:input-hash-drift-check | draft | CAP-072 | TBD |
| [BC-5.22.019](ss-05/BC-5.22.019.md) | phase-2:human-approval | draft | CAP-072 | TBD |
| [BC-5.23.001](ss-05/BC-5.23.001.md) | phase-3-tdd-implementation: identity | draft | CAP-073 | TBD |
| [BC-5.23.002](ss-05/BC-5.23.002.md) | phase-3: entry-point | draft | CAP-073 | TBD |
| [BC-5.23.003](ss-05/BC-5.23.003.md) | phase-3: terminal-step | draft | CAP-073 | TBD |
| [BC-5.23.004](ss-05/BC-5.23.004.md) | phase-3: DAG integrity | draft | CAP-073 | TBD |
| [BC-5.23.005](ss-05/BC-5.23.005.md) | phase-3: failure semantics | draft | CAP-073 | TBD |
| [BC-5.23.006](ss-05/BC-5.23.006.md) | phase-3:create-worktree | draft | CAP-073 | TBD |
| [BC-5.23.007](ss-05/BC-5.23.007.md) | phase-3:backup-create-worktree | draft | CAP-073 | TBD |
| [BC-5.23.008](ss-05/BC-5.23.008.md) | phase-3:generate-stubs | draft | CAP-073 | TBD |
| [BC-5.23.009](ss-05/BC-5.23.009.md) | phase-3:backup-generate-stubs | draft | CAP-073 | TBD |
| [BC-5.23.010](ss-05/BC-5.23.010.md) | phase-3:failing-tests | draft | CAP-073 | TBD |
| [BC-5.23.011](ss-05/BC-5.23.011.md) | phase-3:backup-failing-tests | draft | CAP-073 | TBD |
| [BC-5.23.012](ss-05/BC-5.23.012.md) | phase-3:implement | draft | CAP-073 | TBD |
| [BC-5.23.013](ss-05/BC-5.23.013.md) | phase-3:backup-implement | draft | CAP-073 | TBD |
| [BC-5.23.014](ss-05/BC-5.23.014.md) | phase-3:record-demos | draft | CAP-073 | TBD |
| [BC-5.23.015](ss-05/BC-5.23.015.md) | phase-3:backup-record-demos | draft | CAP-073 | TBD |
| [BC-5.23.016](ss-05/BC-5.23.016.md) | phase-3:pr-lifecycle | draft | CAP-073 | TBD |
| [BC-5.23.017](ss-05/BC-5.23.017.md) | phase-3:backup-pr-lifecycle | draft | CAP-073 | TBD |
| [BC-5.23.018](ss-05/BC-5.23.018.md) | phase-3:cleanup | draft | CAP-073 | TBD |
| [BC-5.23.019](ss-05/BC-5.23.019.md) | phase-3:backup-cleanup | draft | CAP-073 | TBD |
| [BC-5.23.020](ss-05/BC-5.23.020.md) | phase-3:implementation-gate | draft | CAP-073 | TBD |
| [BC-5.23.021](ss-05/BC-5.23.021.md) | phase-3:input-hash-drift-check | draft | CAP-073 | TBD |
| [BC-5.24.001](ss-05/BC-5.24.001.md) | phase-4-holdout-evaluation: identity | draft | CAP-074 | TBD |
| [BC-5.24.002](ss-05/BC-5.24.002.md) | phase-4: entry-point | draft | CAP-074 | TBD |
| [BC-5.24.003](ss-05/BC-5.24.003.md) | phase-4: terminal-step | draft | CAP-074 | TBD |
| [BC-5.24.004](ss-05/BC-5.24.004.md) | phase-4: DAG integrity | draft | CAP-074 | TBD |
| [BC-5.24.005](ss-05/BC-5.24.005.md) | phase-4: failure semantics | draft | CAP-074 | TBD |
| [BC-5.24.006](ss-05/BC-5.24.006.md) | phase-4:scenario-rotation | draft | CAP-074 | TBD |
| [BC-5.24.007](ss-05/BC-5.24.007.md) | phase-4:holdout-evaluation | draft | CAP-074 | TBD |
| [BC-5.24.008](ss-05/BC-5.24.008.md) | phase-4:holdout-gate | draft | CAP-074 | TBD |
| [BC-5.25.001](ss-05/BC-5.25.001.md) | phase-5-adversarial-refinement: identity | draft | CAP-075 | TBD |
| [BC-5.25.002](ss-05/BC-5.25.002.md) | phase-5: entry-point | draft | CAP-075 | TBD |
| [BC-5.25.003](ss-05/BC-5.25.003.md) | phase-5: terminal-step | draft | CAP-075 | TBD |
| [BC-5.25.004](ss-05/BC-5.25.004.md) | phase-5: DAG integrity | draft | CAP-075 | TBD |
| [BC-5.25.005](ss-05/BC-5.25.005.md) | phase-5: failure semantics | draft | CAP-075 | TBD |
| [BC-5.25.006](ss-05/BC-5.25.006.md) | phase-5:adversarial-review-loop | draft | CAP-075 | TBD |
| [BC-5.25.007](ss-05/BC-5.25.007.md) | phase-5:adversary-code-review (nested) | draft | CAP-075 | TBD |
| [BC-5.25.008](ss-05/BC-5.25.008.md) | phase-5:triage-and-fix (nested) | draft | CAP-075 | TBD |
| [BC-5.25.009](ss-05/BC-5.25.009.md) | phase-5:gemini-secondary-review | draft | CAP-075 | TBD |
| [BC-5.26.001](ss-05/BC-5.26.001.md) | phase-6-formal-hardening: identity | draft | CAP-076 | TBD |
| [BC-5.26.002](ss-05/BC-5.26.002.md) | phase-6: entry-point | draft | CAP-076 | TBD |
| [BC-5.26.003](ss-05/BC-5.26.003.md) | phase-6: terminal-step | draft | CAP-076 | TBD |
| [BC-5.26.004](ss-05/BC-5.26.004.md) | phase-6: DAG integrity | draft | CAP-076 | TBD |
| [BC-5.26.005](ss-05/BC-5.26.005.md) | phase-6: failure semantics | draft | CAP-076 | TBD |
| [BC-5.26.006](ss-05/BC-5.26.006.md) | phase-6:kani-proofs | draft | CAP-076 | TBD |
| [BC-5.26.007](ss-05/BC-5.26.007.md) | phase-6:backup-kani-proofs | draft | CAP-076 | TBD |
| [BC-5.26.008](ss-05/BC-5.26.008.md) | phase-6:fuzz-testing | draft | CAP-076 | TBD |
| [BC-5.26.009](ss-05/BC-5.26.009.md) | phase-6:backup-fuzz-testing | draft | CAP-076 | TBD |
| [BC-5.26.010](ss-05/BC-5.26.010.md) | phase-6:mutation-testing | draft | CAP-076 | TBD |
| [BC-5.26.011](ss-05/BC-5.26.011.md) | phase-6:backup-mutation-testing | draft | CAP-076 | TBD |
| [BC-5.26.012](ss-05/BC-5.26.012.md) | phase-6:security-scan | draft | CAP-076 | TBD |
| [BC-5.26.013](ss-05/BC-5.26.013.md) | phase-6:backup-security-scan | draft | CAP-076 | TBD |
| [BC-5.26.014](ss-05/BC-5.26.014.md) | phase-6:hardening-gate | draft | CAP-076 | TBD |
| [BC-5.27.001](ss-05/BC-5.27.001.md) | phase-7-convergence: identity | draft | CAP-077 | TBD |
| [BC-5.27.002](ss-05/BC-5.27.002.md) | phase-7: entry-point | draft | CAP-077 | TBD |
| [BC-5.27.003](ss-05/BC-5.27.003.md) | phase-7: terminal-step | draft | CAP-077 | TBD |
| [BC-5.27.004](ss-05/BC-5.27.004.md) | phase-7: DAG integrity | draft | CAP-077 | TBD |
| [BC-5.27.005](ss-05/BC-5.27.005.md) | phase-7: failure semantics | draft | CAP-077 | TBD |
| [BC-5.27.006](ss-05/BC-5.27.006.md) | phase-7:spec-convergence | draft | CAP-077 | TBD |
| [BC-5.27.007](ss-05/BC-5.27.007.md) | phase-7:backup-spec-convergence | draft | CAP-077 | TBD |
| [BC-5.27.008](ss-05/BC-5.27.008.md) | phase-7:test-convergence | draft | CAP-077 | TBD |
| [BC-5.27.009](ss-05/BC-5.27.009.md) | phase-7:backup-test-convergence | draft | CAP-077 | TBD |
| [BC-5.27.010](ss-05/BC-5.27.010.md) | phase-7:implementation-convergence | draft | CAP-077 | TBD |
| [BC-5.27.011](ss-05/BC-5.27.011.md) | phase-7:backup-implementation-convergence | draft | CAP-077 | TBD |
| [BC-5.27.012](ss-05/BC-5.27.012.md) | phase-7:verification-convergence | draft | CAP-077 | TBD |
| [BC-5.27.013](ss-05/BC-5.27.013.md) | phase-7:backup-verification-convergence | draft | CAP-077 | TBD |
| [BC-5.27.014](ss-05/BC-5.27.014.md) | phase-7:visual-convergence | draft | CAP-077 | TBD |
| [BC-5.27.015](ss-05/BC-5.27.015.md) | phase-7:backup-visual-convergence | draft | CAP-077 | TBD |
| [BC-5.27.016](ss-05/BC-5.27.016.md) | phase-7:performance-convergence | draft | CAP-077 | TBD |
| [BC-5.27.017](ss-05/BC-5.27.017.md) | phase-7:backup-performance-convergence | draft | CAP-077 | TBD |
| [BC-5.27.018](ss-05/BC-5.27.018.md) | phase-7:documentation-convergence | draft | CAP-077 | TBD |
| [BC-5.27.019](ss-05/BC-5.27.019.md) | phase-7:backup-documentation-convergence | draft | CAP-077 | TBD |
| [BC-5.27.020](ss-05/BC-5.27.020.md) | phase-7:convergence-gate | draft | CAP-077 | TBD |
| [BC-5.27.021](ss-05/BC-5.27.021.md) | phase-7:input-hash-drift-check | draft | CAP-077 | TBD |
| [BC-5.27.022](ss-05/BC-5.27.022.md) | phase-7:convergence-demo | draft | CAP-077 | TBD |
| [BC-5.27.023](ss-05/BC-5.27.023.md) | phase-7:visual-review | draft | CAP-077 | TBD |
| [BC-5.27.024](ss-05/BC-5.27.024.md) | phase-7:human-approval | draft | CAP-077 | TBD |
| [BC-5.28.001](ss-05/BC-5.28.001.md) | greenfield-vsdd: identity | draft | CAP-078 | TBD |
| [BC-5.28.002](ss-05/BC-5.28.002.md) | greenfield: entry-point | draft | CAP-078 | TBD |
| [BC-5.28.003](ss-05/BC-5.28.003.md) | greenfield: terminal-step | draft | CAP-078 | TBD |
| [BC-5.28.004](ss-05/BC-5.28.004.md) | greenfield: DAG integrity | draft | CAP-078 | TBD |
| [BC-5.28.005](ss-05/BC-5.28.005.md) | greenfield: failure semantics | draft | CAP-078 | TBD |
| [BC-5.28.006](ss-05/BC-5.28.006.md) | greenfield: cost monitoring (workflow-level) | draft | CAP-078 | TBD |
| [BC-5.28.007](ss-05/BC-5.28.007.md) | greenfield:repo-initialization | draft | CAP-078 | TBD |
| [BC-5.28.008](ss-05/BC-5.28.008.md) | greenfield:factory-worktree-health | draft | CAP-078 | TBD |
| [BC-5.28.009](ss-05/BC-5.28.009.md) | greenfield:factory-worktree-gate | draft | CAP-078 | TBD |
| [BC-5.28.010](ss-05/BC-5.28.010.md) | greenfield:scaffold-claude-md | draft | CAP-078 | TBD |
| [BC-5.28.011](ss-05/BC-5.28.011.md) | greenfield:state-initialization | draft | CAP-078 | TBD |
| [BC-5.28.012](ss-05/BC-5.28.012.md) | greenfield:adaptive-planning | draft | CAP-078 | TBD |
| [BC-5.28.013](ss-05/BC-5.28.013.md) | greenfield:phase-1-spec-crystallization | draft | CAP-078 | TBD |
| [BC-5.28.014](ss-05/BC-5.28.014.md) | greenfield:architect-feasibility-review | draft | CAP-078 | TBD |
| [BC-5.28.015](ss-05/BC-5.28.015.md) | greenfield:prd-revision | draft | CAP-078 | TBD |
| [BC-5.28.016](ss-05/BC-5.28.016.md) | greenfield:phase-1-dtu-assessment | draft | CAP-078 | TBD |
| [BC-5.28.017](ss-05/BC-5.28.017.md) | greenfield:phase-1-gene-transfusion-assessment | draft | CAP-078 | TBD |
| [BC-5.28.018](ss-05/BC-5.28.018.md) | greenfield:phase-1-cicd-setup | draft | CAP-078 | TBD |
| [BC-5.28.019](ss-05/BC-5.28.019.md) | greenfield:phase-1-design-system-bootstrap | draft | CAP-078 | TBD |
| [BC-5.28.020](ss-05/BC-5.28.020.md) | greenfield:phase-1-design-system-approval | draft | CAP-078 | TBD |
| [BC-5.28.021](ss-05/BC-5.28.021.md) | greenfield:phase-1-multi-variant-design | draft | CAP-078 | TBD |
| [BC-5.28.022](ss-05/BC-5.28.022.md) | greenfield:phase-1-multi-variant-approval | draft | CAP-078 | TBD |
| [BC-5.28.023](ss-05/BC-5.28.023.md) | greenfield:phase-1-heuristic-evaluation | draft | CAP-078 | TBD |
| [BC-5.28.024](ss-05/BC-5.28.024.md) | greenfield:phase-1-consistency-audit | draft | CAP-078 | TBD |
| [BC-5.28.025](ss-05/BC-5.28.025.md) | greenfield:phase-1-gate | draft | CAP-078 | TBD |
| [BC-5.28.026](ss-05/BC-5.28.026.md) | greenfield:phase-1d-adversarial-spec-review | draft | CAP-078 | TBD |
| [BC-5.28.027](ss-05/BC-5.28.027.md) | greenfield:phase-1d-spec-review-gemini | draft | CAP-078 | TBD |
| [BC-5.28.028](ss-05/BC-5.28.028.md) | greenfield:phase-1-state-backup | draft | CAP-078 | TBD |
| [BC-5.28.029](ss-05/BC-5.28.029.md) | greenfield:phase-1-human-approval | draft | CAP-078 | TBD |
| [BC-5.28.030](ss-05/BC-5.28.030.md) | greenfield:multi-repo-topology-check | draft | CAP-078 | TBD |
| [BC-5.28.031](ss-05/BC-5.28.031.md) | greenfield:multi-repo-human-confirmation | draft | CAP-078 | TBD |
| [BC-5.28.032](ss-05/BC-5.28.032.md) | greenfield:multi-repo-transition | draft | CAP-078 | TBD |
| [BC-5.28.033](ss-05/BC-5.28.033.md) | greenfield:multi-repo-state-migration | draft | CAP-078 | TBD |
| [BC-5.28.034](ss-05/BC-5.28.034.md) | greenfield:phase-2-story-decomposition | draft | CAP-078 | TBD |
| [BC-5.28.035](ss-05/BC-5.28.035.md) | greenfield:phase-2-consistency-check | draft | CAP-078 | TBD |
| [BC-5.28.036](ss-05/BC-5.28.036.md) | greenfield:phase-2-adversarial-review | draft | CAP-078 | TBD |
| [BC-5.28.037](ss-05/BC-5.28.037.md) | greenfield:phase-2-consistency-audit | draft | CAP-078 | TBD |
| [BC-5.28.038](ss-05/BC-5.28.038.md) | greenfield:phase-2-gate | draft | CAP-078 | TBD |
| [BC-5.28.039](ss-05/BC-5.28.039.md) | greenfield:phase-2-spec-review-gemini | draft | CAP-078 | TBD |
| [BC-5.28.040](ss-05/BC-5.28.040.md) | greenfield:phase-2-state-backup | draft | CAP-078 | TBD |
| [BC-5.28.041](ss-05/BC-5.28.041.md) | greenfield:phase-2-human-approval | draft | CAP-078 | TBD |
| [BC-5.28.042](ss-05/BC-5.28.042.md) | greenfield:dx-engineer-preflight | draft | CAP-078 | TBD |
| [BC-5.28.043](ss-05/BC-5.28.043.md) | greenfield:dx-engineer-preflight-gate | draft | CAP-078 | TBD |
| [BC-5.28.044](ss-05/BC-5.28.044.md) | greenfield:pre-phase-4-dtu-gate | draft | CAP-078 | TBD |
| [BC-5.28.045](ss-05/BC-5.28.045.md) | greenfield:pre-phase-4-cicd-gate | draft | CAP-078 | TBD |
| [BC-5.28.046](ss-05/BC-5.28.046.md) | greenfield:phase-3-per-story-delivery | draft | CAP-078 | TBD |
| [BC-5.28.047](ss-05/BC-5.28.047.md) | greenfield:phase-3-dtu-validation | draft | CAP-078 | TBD |
| [BC-5.28.048](ss-05/BC-5.28.048.md) | greenfield:phase-3-consistency-audit | draft | CAP-078 | TBD |
| [BC-5.28.049](ss-05/BC-5.28.049.md) | greenfield:phase-3-gate | draft | CAP-078 | TBD |
| [BC-5.28.050](ss-05/BC-5.28.050.md) | greenfield:phase-4-scenario-rotation | draft | CAP-078 | TBD |
| [BC-5.28.051](ss-05/BC-5.28.051.md) | greenfield:phase-4-dtu-startup | draft | CAP-078 | TBD |
| [BC-5.28.052](ss-05/BC-5.28.052.md) | greenfield:phase-4-holdout-evaluation | draft | CAP-078 | TBD |
| [BC-5.28.053](ss-05/BC-5.28.053.md) | greenfield:phase-4-gate | draft | CAP-078 | TBD |
| [BC-5.28.054](ss-05/BC-5.28.054.md) | greenfield:phase-4-demo-recording | draft | CAP-078 | TBD |
| [BC-5.28.055](ss-05/BC-5.28.055.md) | greenfield:phase-5-adversarial-refinement | draft | CAP-078 | TBD |
| [BC-5.28.056](ss-05/BC-5.28.056.md) | greenfield:phase-5-gemini-review | draft | CAP-078 | TBD |
| [BC-5.28.057](ss-05/BC-5.28.057.md) | greenfield:phase-4b-holdout-regression | draft | CAP-078 | TBD |
| [BC-5.28.058](ss-05/BC-5.28.058.md) | greenfield:phase-6-formal-hardening | draft | CAP-078 | TBD |
| [BC-5.28.059](ss-05/BC-5.28.059.md) | greenfield:phase-5-fix-delivery | draft | CAP-078 | TBD |
| [BC-5.28.060](ss-05/BC-5.28.060.md) | greenfield:phase-6-dtu-adversarial | draft | CAP-078 | TBD |
| [BC-5.28.061](ss-05/BC-5.28.061.md) | greenfield:phase-5-gate | draft | CAP-078 | TBD |
| [BC-5.28.062](ss-05/BC-5.28.062.md) | greenfield:phase-6-heuristic-evaluation | draft | CAP-078 | TBD |
| [BC-5.28.063](ss-05/BC-5.28.063.md) | greenfield:phase-6-ui-completeness-final | draft | CAP-078 | TBD |
| [BC-5.28.064](ss-05/BC-5.28.064.md) | greenfield:phase-6-responsive-final | draft | CAP-078 | TBD |
| [BC-5.28.065](ss-05/BC-5.28.065.md) | greenfield:phase-6-ui-quality-gate | draft | CAP-078 | TBD |
| [BC-5.28.066](ss-05/BC-5.28.066.md) | greenfield:phase-6-ui-fix-delivery | draft | CAP-078 | TBD |
| [BC-5.28.067](ss-05/BC-5.28.067.md) | greenfield:phase-7-convergence | draft | CAP-078 | TBD |
| [BC-5.28.068](ss-05/BC-5.28.068.md) | greenfield:phase-6-gate | draft | CAP-078 | TBD |
| [BC-5.28.069](ss-05/BC-5.28.069.md) | greenfield:phase-6-final-demo | draft | CAP-078 | TBD |
| [BC-5.28.070](ss-05/BC-5.28.070.md) | greenfield:phase-6-visual-review | draft | CAP-078 | TBD |
| [BC-5.28.071](ss-05/BC-5.28.071.md) | greenfield:phase-6-state-backup | draft | CAP-078 | TBD |
| [BC-5.28.072](ss-05/BC-5.28.072.md) | greenfield:phase-6-human-approval | draft | CAP-078 | TBD |
| [BC-5.28.073](ss-05/BC-5.28.073.md) | greenfield:release | draft | CAP-078 | TBD |
| [BC-5.28.074](ss-05/BC-5.28.074.md) | greenfield:steady-state-handoff | draft | CAP-078 | TBD |
| [BC-5.28.075](ss-05/BC-5.28.075.md) | greenfield:post-feature-validation | draft | CAP-078 | TBD |
| [BC-5.28.076](ss-05/BC-5.28.076.md) | greenfield:session-review | draft | CAP-078 | TBD |
| [BC-5.28.077](ss-05/BC-5.28.077.md) | greenfield:session-review-approval | draft | CAP-078 | TBD |
| [BC-5.28.078](ss-05/BC-5.28.078.md) | greenfield:process-review-decisions | draft | CAP-078 | TBD |
| [BC-5.29.001](ss-05/BC-5.29.001.md) | brownfield-vsdd: identity | draft | CAP-079 | TBD |
| [BC-5.29.002](ss-05/BC-5.29.002.md) | brownfield: entry-point | draft | CAP-079 | TBD |
| [BC-5.29.003](ss-05/BC-5.29.003.md) | brownfield: terminal-step | draft | CAP-079 | TBD |
| [BC-5.29.004](ss-05/BC-5.29.004.md) | brownfield: DAG integrity | draft | CAP-079 | TBD |
| [BC-5.29.005](ss-05/BC-5.29.005.md) | brownfield: failure semantics | draft | CAP-079 | TBD |
| [BC-5.29.006](ss-05/BC-5.29.006.md) | brownfield:environment-setup | draft | CAP-079 | TBD |
| [BC-5.29.007](ss-05/BC-5.29.007.md) | brownfield:environment-gate | draft | CAP-079 | TBD |
| [BC-5.29.008](ss-05/BC-5.29.008.md) | brownfield:repo-verification | draft | CAP-079 | TBD |
| [BC-5.29.009](ss-05/BC-5.29.009.md) | brownfield:factory-worktree-health | draft | CAP-079 | TBD |
| [BC-5.29.010](ss-05/BC-5.29.010.md) | brownfield:factory-worktree-gate | draft | CAP-079 | TBD |
| [BC-5.29.011](ss-05/BC-5.29.011.md) | brownfield:scaffold-claude-md | draft | CAP-079 | TBD |
| [BC-5.29.012](ss-05/BC-5.29.012.md) | brownfield:state-initialization | draft | CAP-079 | TBD |
| [BC-5.29.013](ss-05/BC-5.29.013.md) | brownfield:phase-0-codebase-ingestion | draft | CAP-079 | TBD |
| [BC-5.29.014](ss-05/BC-5.29.014.md) | brownfield:phase-0-artifact-backup | draft | CAP-079 | TBD |
| [BC-5.29.015](ss-05/BC-5.29.015.md) | brownfield:phase-0-gate | draft | CAP-079 | TBD |
| [BC-5.29.016](ss-05/BC-5.29.016.md) | brownfield:phase-0-human-approval | draft | CAP-079 | TBD |
| [BC-5.29.017](ss-05/BC-5.29.017.md) | brownfield:post-phase-0-routing | draft | CAP-079 | TBD |
| [BC-5.29.018](ss-05/BC-5.29.018.md) | brownfield:brownfield-market-intel | draft | CAP-079 | TBD |
| [BC-5.29.019](ss-05/BC-5.29.019.md) | brownfield:brownfield-market-review | draft | CAP-079 | TBD |
| [BC-5.29.020](ss-05/BC-5.29.020.md) | brownfield:detect-cross-language-porting | draft | CAP-079 | TBD |
| [BC-5.29.021](ss-05/BC-5.29.021.md) | brownfield:semport-translation | draft | CAP-079 | TBD |
| [BC-5.29.022](ss-05/BC-5.29.022.md) | brownfield:semport-validation-gate | draft | CAP-079 | TBD |
| [BC-5.29.023](ss-05/BC-5.29.023.md) | brownfield:brownfield-design-system-extract | draft | CAP-079 | TBD |
| [BC-5.29.024](ss-05/BC-5.29.024.md) | brownfield:brownfield-design-system-approval | draft | CAP-079 | TBD |
| [BC-5.29.025](ss-05/BC-5.29.025.md) | brownfield:brownfield-to-greenfield-transition | draft | CAP-079 | TBD |
| [BC-5.29.026](ss-05/BC-5.29.026.md) | brownfield:greenfield-pipeline | draft | CAP-079 | TBD |
| [BC-5.29.027](ss-05/BC-5.29.027.md) | brownfield:multi-repo-handoff-check | draft | CAP-079 | TBD |
| [BC-5.29.028](ss-05/BC-5.29.028.md) | brownfield:multi-repo-pipeline | draft | CAP-079 | TBD |
| [BC-5.29.029](ss-05/BC-5.29.029.md) | brownfield:session-review | draft | CAP-079 | TBD |
| [BC-5.29.030](ss-05/BC-5.29.030.md) | brownfield:session-review-approval | draft | CAP-079 | TBD |
| [BC-5.29.031](ss-05/BC-5.29.031.md) | brownfield:process-review-decisions | draft | CAP-079 | TBD |
| [BC-5.30.001](ss-05/BC-5.30.001.md) | feature-vsdd: identity | draft | CAP-080 | TBD |
| [BC-5.30.002](ss-05/BC-5.30.002.md) | feature: entry-point | draft | CAP-080 | TBD |
| [BC-5.30.003](ss-05/BC-5.30.003.md) | feature: terminal-step | draft | CAP-080 | TBD |
| [BC-5.30.004](ss-05/BC-5.30.004.md) | feature: DAG integrity | draft | CAP-080 | TBD |
| [BC-5.30.005](ss-05/BC-5.30.005.md) | feature: failure semantics | draft | CAP-080 | TBD |
| [BC-5.30.006](ss-05/BC-5.30.006.md) | feature:factory-worktree-health | draft | CAP-080 | TBD |
| [BC-5.30.007](ss-05/BC-5.30.007.md) | feature:factory-worktree-gate | draft | CAP-080 | TBD |
| [BC-5.30.008](ss-05/BC-5.30.008.md) | feature:feature-cycle-init | draft | CAP-080 | TBD |
| [BC-5.30.009](ss-05/BC-5.30.009.md) | feature:environment-check | draft | CAP-080 | TBD |
| [BC-5.30.010](ss-05/BC-5.30.010.md) | feature:feature-market-intel | draft | CAP-080 | TBD |
| [BC-5.30.011](ss-05/BC-5.30.011.md) | feature:feature-market-review | draft | CAP-080 | TBD |
| [BC-5.30.012](ss-05/BC-5.30.012.md) | feature:establish-demo-baseline | draft | CAP-080 | TBD |
| [BC-5.30.013](ss-05/BC-5.30.013.md) | feature:phase-f1-delta-analysis | draft | CAP-080 | TBD |
| [BC-5.30.014](ss-05/BC-5.30.014.md) | feature:phase-f1-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.015](ss-05/BC-5.30.015.md) | feature:phase-f1-gate | draft | CAP-080 | TBD |
| [BC-5.30.016](ss-05/BC-5.30.016.md) | feature:phase-f1-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.017](ss-05/BC-5.30.017.md) | feature:quick-dev-single-story | draft | CAP-080 | TBD |
| [BC-5.30.018](ss-05/BC-5.30.018.md) | feature:quick-dev-regression | draft | CAP-080 | TBD |
| [BC-5.30.019](ss-05/BC-5.30.019.md) | feature:quick-dev-f7-lite | draft | CAP-080 | TBD |
| [BC-5.30.020](ss-05/BC-5.30.020.md) | feature:quick-dev-f7-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.021](ss-05/BC-5.30.021.md) | feature:quick-dev-release | draft | CAP-080 | TBD |
| [BC-5.30.022](ss-05/BC-5.30.022.md) | feature:quick-dev-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.023](ss-05/BC-5.30.023.md) | feature:bugfix-demo-baseline | draft | CAP-080 | TBD |
| [BC-5.30.024](ss-05/BC-5.30.024.md) | feature:bugfix-single-story-delivery | draft | CAP-080 | TBD |
| [BC-5.30.025](ss-05/BC-5.30.025.md) | feature:bugfix-build-verification | draft | CAP-080 | TBD |
| [BC-5.30.026](ss-05/BC-5.30.026.md) | feature:bugfix-holdout-scoped | draft | CAP-080 | TBD |
| [BC-5.30.027](ss-05/BC-5.30.027.md) | feature:bugfix-f5-scoped | draft | CAP-080 | TBD |
| [BC-5.30.028](ss-05/BC-5.30.028.md) | feature:bugfix-f6-scoped | draft | CAP-080 | TBD |
| [BC-5.30.029](ss-05/BC-5.30.029.md) | feature:bugfix-f6-a11y | draft | CAP-080 | TBD |
| [BC-5.30.030](ss-05/BC-5.30.030.md) | feature:bugfix-demo-comparison | draft | CAP-080 | TBD |
| [BC-5.30.031](ss-05/BC-5.30.031.md) | feature:bugfix-f7-verification | draft | CAP-080 | TBD |
| [BC-5.30.032](ss-05/BC-5.30.032.md) | feature:bugfix-f7-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.033](ss-05/BC-5.30.033.md) | feature:bugfix-release | draft | CAP-080 | TBD |
| [BC-5.30.034](ss-05/BC-5.30.034.md) | feature:bugfix-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.035](ss-05/BC-5.30.035.md) | feature:bugfix-post-monitoring | draft | CAP-080 | TBD |
| [BC-5.30.036](ss-05/BC-5.30.036.md) | feature:phase-f2-spec-evolution | draft | CAP-080 | TBD |
| [BC-5.30.037](ss-05/BC-5.30.037.md) | feature:phase-f2-dtu-reassessment | draft | CAP-080 | TBD |
| [BC-5.30.038](ss-05/BC-5.30.038.md) | feature:phase-f2-gene-transfusion-assessment | draft | CAP-080 | TBD |
| [BC-5.30.039](ss-05/BC-5.30.039.md) | feature:phase-f2-ux-design | draft | CAP-080 | TBD |
| [BC-5.30.040](ss-05/BC-5.30.040.md) | feature:phase-f2-design-system-bootstrap | draft | CAP-080 | TBD |
| [BC-5.30.041](ss-05/BC-5.30.041.md) | feature:phase-f2-multi-variant | draft | CAP-080 | TBD |
| [BC-5.30.042](ss-05/BC-5.30.042.md) | feature:phase-f2-multi-variant-approval | draft | CAP-080 | TBD |
| [BC-5.30.043](ss-05/BC-5.30.043.md) | feature:phase-f2-a11y-review | draft | CAP-080 | TBD |
| [BC-5.30.044](ss-05/BC-5.30.044.md) | feature:phase-f2-adversarial-review | draft | CAP-080 | TBD |
| [BC-5.30.045](ss-05/BC-5.30.045.md) | feature:phase-f2-spec-review-gemini | draft | CAP-080 | TBD |
| [BC-5.30.046](ss-05/BC-5.30.046.md) | feature:phase-f2-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.047](ss-05/BC-5.30.047.md) | feature:phase-f2-gate | draft | CAP-080 | TBD |
| [BC-5.30.048](ss-05/BC-5.30.048.md) | feature:phase-f2-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.049](ss-05/BC-5.30.049.md) | feature:phase-f3-incremental-stories | draft | CAP-080 | TBD |
| [BC-5.30.050](ss-05/BC-5.30.050.md) | feature:phase-f3-spec-review-gemini | draft | CAP-080 | TBD |
| [BC-5.30.051](ss-05/BC-5.30.051.md) | feature:phase-f3-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.052](ss-05/BC-5.30.052.md) | feature:phase-f3-gate | draft | CAP-080 | TBD |
| [BC-5.30.053](ss-05/BC-5.30.053.md) | feature:phase-f3-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.054](ss-05/BC-5.30.054.md) | feature:toolchain-preflight | draft | CAP-080 | TBD |
| [BC-5.30.055](ss-05/BC-5.30.055.md) | feature:phase-f4-delta-implementation | draft | CAP-080 | TBD |
| [BC-5.30.056](ss-05/BC-5.30.056.md) | feature:phase-f4-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.057](ss-05/BC-5.30.057.md) | feature:phase-f4-gate | draft | CAP-080 | TBD |
| [BC-5.30.058](ss-05/BC-5.30.058.md) | feature:build-verification | draft | CAP-080 | TBD |
| [BC-5.30.059](ss-05/BC-5.30.059.md) | feature:build-gate | draft | CAP-080 | TBD |
| [BC-5.30.060](ss-05/BC-5.30.060.md) | feature:holdout-evaluation | draft | CAP-080 | TBD |
| [BC-5.30.061](ss-05/BC-5.30.061.md) | feature:holdout-gate | draft | CAP-080 | TBD |
| [BC-5.30.062](ss-05/BC-5.30.062.md) | feature:phase-f5-scoped-adversarial | draft | CAP-080 | TBD |
| [BC-5.30.063](ss-05/BC-5.30.063.md) | feature:phase-f5-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.064](ss-05/BC-5.30.064.md) | feature:phase-f6-targeted-hardening | draft | CAP-080 | TBD |
| [BC-5.30.065](ss-05/BC-5.30.065.md) | feature:phase-f6-security-scan | draft | CAP-080 | TBD |
| [BC-5.30.066](ss-05/BC-5.30.066.md) | feature:phase-f6-dtu-adversarial | draft | CAP-080 | TBD |
| [BC-5.30.067](ss-05/BC-5.30.067.md) | feature:phase-f6-a11y-recheck | draft | CAP-080 | TBD |
| [BC-5.30.068](ss-05/BC-5.30.068.md) | feature:phase-f6-fix-delivery | draft | CAP-080 | TBD |
| [BC-5.30.069](ss-05/BC-5.30.069.md) | feature:phase-f6-gate | draft | CAP-080 | TBD |
| [BC-5.30.070](ss-05/BC-5.30.070.md) | feature:phase-f6-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.071](ss-05/BC-5.30.071.md) | feature:phase-f6-demo-recording | draft | CAP-080 | TBD |
| [BC-5.30.072](ss-05/BC-5.30.072.md) | feature:phase-f6-visual-regression | draft | CAP-080 | TBD |
| [BC-5.30.073](ss-05/BC-5.30.073.md) | feature:phase-f7-heuristic-evaluation | draft | CAP-080 | TBD |
| [BC-5.30.074](ss-05/BC-5.30.074.md) | feature:phase-f7-ui-completeness-final | draft | CAP-080 | TBD |
| [BC-5.30.075](ss-05/BC-5.30.075.md) | feature:phase-f7-responsive-final | draft | CAP-080 | TBD |
| [BC-5.30.076](ss-05/BC-5.30.076.md) | feature:phase-f7-ui-quality-gate | draft | CAP-080 | TBD |
| [BC-5.30.077](ss-05/BC-5.30.077.md) | feature:phase-f7-ui-fix-delivery | draft | CAP-080 | TBD |
| [BC-5.30.078](ss-05/BC-5.30.078.md) | feature:phase-f7-delta-convergence | draft | CAP-080 | TBD |
| [BC-5.30.079](ss-05/BC-5.30.079.md) | feature:phase-f7-gate | draft | CAP-080 | TBD |
| [BC-5.30.080](ss-05/BC-5.30.080.md) | feature:phase-f7-state-backup | draft | CAP-080 | TBD |
| [BC-5.30.081](ss-05/BC-5.30.081.md) | feature:phase-f7-human-approval | draft | CAP-080 | TBD |
| [BC-5.30.082](ss-05/BC-5.30.082.md) | feature:release | draft | CAP-080 | TBD |
| [BC-5.30.083](ss-05/BC-5.30.083.md) | feature:feature-cycle-handoff | draft | CAP-080 | TBD |
| [BC-5.30.084](ss-05/BC-5.30.084.md) | feature:post-feature-validation | draft | CAP-080 | TBD |
| [BC-5.30.085](ss-05/BC-5.30.085.md) | feature:session-review | draft | CAP-080 | TBD |
| [BC-5.30.086](ss-05/BC-5.30.086.md) | feature:session-review-approval | draft | CAP-080 | TBD |
| [BC-5.30.087](ss-05/BC-5.30.087.md) | feature:process-review-decisions | draft | CAP-080 | TBD |
| [BC-5.31.001](ss-05/BC-5.31.001.md) | per-story-delivery: identity | draft | CAP-TBD | TBD |
| [BC-5.31.002](ss-05/BC-5.31.002.md) | code-delivery: entry-point | draft | CAP-TBD | TBD |
| [BC-5.31.003](ss-05/BC-5.31.003.md) | code-delivery: terminal-step | draft | CAP-TBD | TBD |
| [BC-5.31.004](ss-05/BC-5.31.004.md) | code-delivery: DAG integrity | draft | CAP-TBD | TBD |
| [BC-5.31.005](ss-05/BC-5.31.005.md) | code-delivery: failure semantics | draft | CAP-TBD | TBD |
| [BC-5.31.006](ss-05/BC-5.31.006.md) | code-delivery:create-worktree | draft | CAP-TBD | TBD |
| [BC-5.31.007](ss-05/BC-5.31.007.md) | code-delivery:generate-stubs | draft | CAP-TBD | TBD |
| [BC-5.31.008](ss-05/BC-5.31.008.md) | code-delivery:write-tests | draft | CAP-TBD | TBD |
| [BC-5.31.009](ss-05/BC-5.31.009.md) | code-delivery:red-gate | draft | CAP-TBD | TBD |
| [BC-5.31.010](ss-05/BC-5.31.010.md) | code-delivery:implement | draft | CAP-TBD | TBD |
| [BC-5.31.011](ss-05/BC-5.31.011.md) | code-delivery:per-story-adversarial-review | draft | CAP-TBD | TBD |
| [BC-5.31.012](ss-05/BC-5.31.012.md) | code-delivery:e2e-tests | draft | CAP-TBD | TBD |
| [BC-5.31.013](ss-05/BC-5.31.013.md) | code-delivery:storybook-story-generation | draft | CAP-TBD | TBD |
| [BC-5.31.014](ss-05/BC-5.31.014.md) | code-delivery:storybook-component-tests | draft | CAP-TBD | TBD |
| [BC-5.31.015](ss-05/BC-5.31.015.md) | code-delivery:per-story-ui-quality-gate | draft | CAP-TBD | TBD |
| [BC-5.31.016](ss-05/BC-5.31.016.md) | code-delivery:demo-recording | draft | CAP-TBD | TBD |
| [BC-5.31.017](ss-05/BC-5.31.017.md) | code-delivery:squash-and-push | draft | CAP-TBD | TBD |
| [BC-5.31.018](ss-05/BC-5.31.018.md) | code-delivery:create-pr | draft | CAP-TBD | TBD |
| [BC-5.31.019](ss-05/BC-5.31.019.md) | code-delivery:ai-pr-review | draft | CAP-TBD | TBD |
| [BC-5.31.020](ss-05/BC-5.31.020.md) | code-delivery:security-review | draft | CAP-TBD | TBD |
| [BC-5.31.021](ss-05/BC-5.31.021.md) | code-delivery:pr-review-convergence | draft | CAP-TBD | TBD |
| [BC-5.31.022](ss-05/BC-5.31.022.md) | code-delivery:brownfield-full-regression | draft | CAP-TBD | TBD |
| [BC-5.31.023](ss-05/BC-5.31.023.md) | code-delivery:brownfield-codeowners-check | draft | CAP-TBD | TBD |
| [BC-5.31.024](ss-05/BC-5.31.024.md) | code-delivery:wait-for-ci | draft | CAP-TBD | TBD |
| [BC-5.31.025](ss-05/BC-5.31.025.md) | code-delivery:dependency-merge-check | draft | CAP-TBD | TBD |
| [BC-5.31.026](ss-05/BC-5.31.026.md) | code-delivery:merge-pr | draft | CAP-TBD | TBD |
| [BC-5.31.027](ss-05/BC-5.31.027.md) | code-delivery:delivery-human-approval | draft | CAP-TBD | TBD |
| [BC-5.31.028](ss-05/BC-5.31.028.md) | code-delivery:cleanup-worktree | draft | CAP-TBD | TBD |
| [BC-5.32.001](ss-05/BC-5.32.001.md) | autonomous-discovery: identity | draft | CAP-TBD | TBD |
| [BC-5.32.002](ss-05/BC-5.32.002.md) | discovery: entry-point | draft | CAP-TBD | TBD |
| [BC-5.32.003](ss-05/BC-5.32.003.md) | discovery: terminal-step | draft | CAP-TBD | TBD |
| [BC-5.32.004](ss-05/BC-5.32.004.md) | discovery: DAG integrity | draft | CAP-TBD | TBD |
| [BC-5.32.005](ss-05/BC-5.32.005.md) | discovery: failure semantics | draft | CAP-TBD | TBD |
| [BC-5.32.006](ss-05/BC-5.32.006.md) | discovery:load-discovery-config | draft | CAP-TBD | TBD |
| [BC-5.32.007](ss-05/BC-5.32.007.md) | discovery:state-init | draft | CAP-TBD | TBD |
| [BC-5.32.008](ss-05/BC-5.32.008.md) | discovery:feature-research | draft | CAP-TBD | TBD |
| [BC-5.32.009](ss-05/BC-5.32.009.md) | discovery:state-backup-feature-research | draft | CAP-TBD | TBD |
| [BC-5.32.010](ss-05/BC-5.32.010.md) | discovery:customer-feedback-ingestion | draft | CAP-TBD | TBD |
| [BC-5.32.011](ss-05/BC-5.32.011.md) | discovery:competitive-monitoring | draft | CAP-TBD | TBD |
| [BC-5.32.012](ss-05/BC-5.32.012.md) | discovery:usage-analytics | draft | CAP-TBD | TBD |
| [BC-5.32.013](ss-05/BC-5.32.013.md) | discovery:state-backup-ingestion | draft | CAP-TBD | TBD |
| [BC-5.32.014](ss-05/BC-5.32.014.md) | discovery:intelligence-synthesis | draft | CAP-TBD | TBD |
| [BC-5.32.015](ss-05/BC-5.32.015.md) | discovery:state-backup-synthesis | draft | CAP-TBD | TBD |
| [BC-5.32.016](ss-05/BC-5.32.016.md) | discovery:feature-scoring-value | draft | CAP-TBD | TBD |
| [BC-5.32.017](ss-05/BC-5.32.017.md) | discovery:feature-scoring-feasibility | draft | CAP-TBD | TBD |
| [BC-5.32.018](ss-05/BC-5.32.018.md) | discovery:feature-scoring-novelty | draft | CAP-TBD | TBD |
| [BC-5.32.019](ss-05/BC-5.32.019.md) | discovery:feature-debate | draft | CAP-TBD | TBD |
| [BC-5.32.020](ss-05/BC-5.32.020.md) | discovery:product-research | draft | CAP-TBD | TBD |
| [BC-5.32.021](ss-05/BC-5.32.021.md) | discovery:product-scoring | draft | CAP-TBD | TBD |
| [BC-5.32.022](ss-05/BC-5.32.022.md) | discovery:deduplication | draft | CAP-TBD | TBD |
| [BC-5.32.023](ss-05/BC-5.32.023.md) | discovery:state-backup-scoring | draft | CAP-TBD | TBD |
| [BC-5.32.024](ss-05/BC-5.32.024.md) | discovery:generate-report | draft | CAP-TBD | TBD |
| [BC-5.32.025](ss-05/BC-5.32.025.md) | discovery:state-backup-report | draft | CAP-TBD | TBD |
| [BC-5.32.026](ss-05/BC-5.32.026.md) | discovery:discovery-notifications | draft | CAP-TBD | TBD |
| [BC-5.32.027](ss-05/BC-5.32.027.md) | discovery:discovery-review | draft | CAP-TBD | TBD |
| [BC-5.32.028](ss-05/BC-5.32.028.md) | discovery:route-approved-ideas | draft | CAP-TBD | TBD |
| [BC-5.32.029](ss-05/BC-5.32.029.md) | discovery:state-final | draft | CAP-TBD | TBD |
| [BC-5.32.030](ss-05/BC-5.32.030.md) | discovery:execute-product-ideas | draft | CAP-TBD | TBD |
| [BC-5.32.031](ss-05/BC-5.32.031.md) | discovery:execute-feature-ideas | draft | CAP-TBD | TBD |
| [BC-5.32.032](ss-05/BC-5.32.032.md) | discovery:session-review | draft | CAP-TBD | TBD |
| [BC-5.32.033](ss-05/BC-5.32.033.md) | discovery:session-review-approval | draft | CAP-TBD | TBD |
| [BC-5.32.034](ss-05/BC-5.32.034.md) | discovery:process-review-decisions | draft | CAP-TBD | TBD |
| [BC-5.33.001](ss-05/BC-5.33.001.md) | maintenance-sweep: identity | draft | CAP-TBD | TBD |
| [BC-5.33.002](ss-05/BC-5.33.002.md) | maintenance: entry-point | draft | CAP-TBD | TBD |
| [BC-5.33.003](ss-05/BC-5.33.003.md) | maintenance: terminal-step | draft | CAP-TBD | TBD |
| [BC-5.33.004](ss-05/BC-5.33.004.md) | maintenance: DAG integrity | draft | CAP-TBD | TBD |
| [BC-5.33.005](ss-05/BC-5.33.005.md) | maintenance: failure semantics | draft | CAP-TBD | TBD |
| [BC-5.33.006](ss-05/BC-5.33.006.md) | maintenance:load-config | draft | CAP-TBD | TBD |
| [BC-5.33.007](ss-05/BC-5.33.007.md) | maintenance:state-init | draft | CAP-TBD | TBD |
| [BC-5.33.008](ss-05/BC-5.33.008.md) | maintenance:dependency-audit-scan | draft | CAP-TBD | TBD |
| [BC-5.33.009](ss-05/BC-5.33.009.md) | maintenance:dependency-audit-analysis | draft | CAP-TBD | TBD |
| [BC-5.33.010](ss-05/BC-5.33.010.md) | maintenance:state-backup-sweep-1 | draft | CAP-TBD | TBD |
| [BC-5.33.011](ss-05/BC-5.33.011.md) | maintenance:doc-drift-scan | draft | CAP-TBD | TBD |
| [BC-5.33.012](ss-05/BC-5.33.012.md) | maintenance:state-backup-sweep-2 | draft | CAP-TBD | TBD |
| [BC-5.33.013](ss-05/BC-5.33.013.md) | maintenance:pattern-consistency-scan | draft | CAP-TBD | TBD |
| [BC-5.33.014](ss-05/BC-5.33.014.md) | maintenance:state-backup-sweep-3 | draft | CAP-TBD | TBD |
| [BC-5.33.015](ss-05/BC-5.33.015.md) | maintenance:holdout-freshness-check | draft | CAP-TBD | TBD |
| [BC-5.33.016](ss-05/BC-5.33.016.md) | maintenance:state-backup-sweep-4 | draft | CAP-TBD | TBD |
| [BC-5.33.017](ss-05/BC-5.33.017.md) | maintenance:performance-regression-scan | draft | CAP-TBD | TBD |
| [BC-5.33.018](ss-05/BC-5.33.018.md) | maintenance:state-backup-sweep-5 | draft | CAP-TBD | TBD |
| [BC-5.33.019](ss-05/BC-5.33.019.md) | maintenance:dtu-fidelity-drift | draft | CAP-TBD | TBD |
| [BC-5.33.020](ss-05/BC-5.33.020.md) | maintenance:state-backup-sweep-6 | draft | CAP-TBD | TBD |
| [BC-5.33.021](ss-05/BC-5.33.021.md) | maintenance:spec-coherence | draft | CAP-TBD | TBD |
| [BC-5.33.022](ss-05/BC-5.33.022.md) | maintenance:state-backup-sweep-7 | draft | CAP-TBD | TBD |
| [BC-5.33.023](ss-05/BC-5.33.023.md) | maintenance:tech-debt-register | draft | CAP-TBD | TBD |
| [BC-5.33.024](ss-05/BC-5.33.024.md) | maintenance:state-backup-sweep-8 | draft | CAP-TBD | TBD |
| [BC-5.33.025](ss-05/BC-5.33.025.md) | maintenance:accessibility-regression | draft | CAP-TBD | TBD |
| [BC-5.33.026](ss-05/BC-5.33.026.md) | maintenance:state-backup-sweep-9 | draft | CAP-TBD | TBD |
| [BC-5.33.027](ss-05/BC-5.33.027.md) | maintenance:design-drift-scan | draft | CAP-TBD | TBD |
| [BC-5.33.028](ss-05/BC-5.33.028.md) | maintenance:state-backup-sweep-10 | draft | CAP-TBD | TBD |
| [BC-5.33.029](ss-05/BC-5.33.029.md) | maintenance:risk-assumption-monitoring | draft | CAP-TBD | TBD |
| [BC-5.33.030](ss-05/BC-5.33.030.md) | maintenance:state-backup-sweep-11 | draft | CAP-TBD | TBD |
| [BC-5.33.031](ss-05/BC-5.33.031.md) | maintenance:maintenance-report | draft | CAP-TBD | TBD |
| [BC-5.33.032](ss-05/BC-5.33.032.md) | maintenance:fix-pr-delivery | draft | CAP-TBD | TBD |
| [BC-5.33.033](ss-05/BC-5.33.033.md) | maintenance:maintenance-demo-recording | draft | CAP-TBD | TBD |
| [BC-5.33.034](ss-05/BC-5.33.034.md) | maintenance:notifications | draft | CAP-TBD | TBD |
| [BC-5.33.035](ss-05/BC-5.33.035.md) | maintenance:state-final | draft | CAP-TBD | TBD |
| [BC-5.33.036](ss-05/BC-5.33.036.md) | maintenance:maintenance-gate | draft | CAP-TBD | TBD |
| [BC-5.33.037](ss-05/BC-5.33.037.md) | maintenance:session-review | draft | CAP-TBD | TBD |
| [BC-5.33.038](ss-05/BC-5.33.038.md) | maintenance:session-review-approval | draft | CAP-TBD | TBD |
| [BC-5.33.039](ss-05/BC-5.33.039.md) | maintenance:process-review-decisions | draft | CAP-TBD | TBD |
| [BC-5.34.001](ss-05/BC-5.34.001.md) | multi-repo-vsdd: identity | draft | CAP-TBD | TBD |
| [BC-5.34.002](ss-05/BC-5.34.002.md) | multi-repo: entry-point | draft | CAP-TBD | TBD |
| [BC-5.34.003](ss-05/BC-5.34.003.md) | multi-repo: terminal-step (primary track) | draft | CAP-TBD | TBD |
| [BC-5.34.004](ss-05/BC-5.34.004.md) | multi-repo: DAG integrity (primary track) | draft | CAP-TBD | TBD |
| [BC-5.34.005](ss-05/BC-5.34.005.md) | multi-repo: failure semantics | draft | CAP-TBD | TBD |
| [BC-5.34.006](ss-05/BC-5.34.006.md) | multi-repo: cross-repo information asymmetry walls | draft | CAP-TBD | TBD |
| [BC-5.34.007](ss-05/BC-5.34.007.md) | multi-repo:environment-setup | draft | CAP-TBD | TBD |
| [BC-5.34.008](ss-05/BC-5.34.008.md) | multi-repo:read-project-manifest | draft | CAP-TBD | TBD |
| [BC-5.34.009](ss-05/BC-5.34.009.md) | multi-repo:compute-repo-waves | draft | CAP-TBD | TBD |
| [BC-5.35.001](ss-05/BC-5.35.001.md) | adaptive-planning: identity | draft | CAP-TBD | TBD |
| [BC-5.35.002](ss-05/BC-5.35.002.md) | planning: entry-point | draft | CAP-TBD | TBD |
| [BC-5.35.003](ss-05/BC-5.35.003.md) | planning: terminal-step | draft | CAP-TBD | TBD |
| [BC-5.35.004](ss-05/BC-5.35.004.md) | planning: DAG integrity | draft | CAP-TBD | TBD |
| [BC-5.35.005](ss-05/BC-5.35.005.md) | planning: failure semantics | draft | CAP-TBD | TBD |
| [BC-5.36.001](ss-05/BC-5.36.001.md) | story-writer agent rejects status=ready when behavioral_contracts is empty | draft | CAP-001 | S-7.01 |
| [BC-5.36.002](ss-05/BC-5.36.002.md) | story-writer requires AC↔BC bidirectional traces before marking a story ready | draft | CAP-001 | S-7.01 |
| [BC-5.36.003](ss-05/BC-5.36.003.md) | product-owner agent requires Capability Anchor Justification cell on every BC | draft | CAP-001 | S-7.01 |
| [BC-5.36.004](ss-05/BC-5.36.004.md) | product-owner cites capabilities.md verbatim in every capability anchor justification | draft | CAP-001 | S-7.01 |
| [BC-5.36.005](ss-05/BC-5.36.005.md) | adversary explicitly checks partial-fix-regression for every finding closed in a prior pass | draft | CAP-001 | S-7.01 |
| [BC-5.36.006](ss-05/BC-5.36.006.md) | adversary checks fix propagation to bodies, sibling files, and prose — not just frontmatter | draft | CAP-001 | S-7.01 |
| [BC-5.36.007](ss-05/BC-5.36.007.md) | all three agent prompts updated atomically in single delivery; no partial update | draft | CAP-001 | S-7.01 |
| [BC-5.37.001](ss-05/BC-5.37.001.md) | state-manager runs corpus-wide grep before declaring count change complete | draft | CAP-001 | S-7.02 |
| [BC-5.37.002](ss-05/BC-5.37.002.md) | state-manager logs sweep results before declaring count-change complete | draft | CAP-001 | S-7.02 |
| [BC-5.38.001](ss-05/BC-5.38.001.md) | stub-architect commit must contain todo!()/unimplemented!() bodies for all non-trivial function implementations | draft | CAP-016 | S-7.03 |
| [BC-5.38.002](ss-05/BC-5.38.002.md) | pure data mappings in stub commits may be implemented inline and must be flagged GREEN-BY-DESIGN | draft | CAP-016 | S-7.03 |
| [BC-5.38.003](ss-05/BC-5.38.003.md) | framework integration wiring may have minimal real code for cargo check; handler business logic must be todo!() | draft | CAP-016 | S-7.03 |
| [BC-5.38.004](ss-05/BC-5.38.004.md) | stub-architect must not use pre-implemented sibling crates as stub templates | draft | CAP-016 | S-7.03 |
| [BC-5.38.005](ss-05/BC-5.38.005.md) | stub-architect applies self-check before committing any non-todo!() function body | draft | CAP-016 | S-7.03 |
| [BC-5.38.006](ss-05/BC-5.38.006.md) | deliver-story SKILL.md and per-story-delivery.md Step 2 must contain anti-precedent guard text verbatim | draft | CAP-016 | S-7.03 |
| [BC-8.29.001](ss-08/BC-8.29.001.md) | RED_RATIO = RED_TESTS / TOTAL_NEW_TESTS must be ≥ 0.5 before Step 4 implementer dispatch (BLOCKING) | draft | CAP-016 | S-7.03 |
| [BC-8.29.002](ss-08/BC-8.29.002.md) | each non-RED test must be documented in red-gate-log with rationale before threshold relaxation | draft | CAP-016 | S-7.03 |
| [BC-8.29.003](ss-08/BC-8.29.003.md) | on RED_RATIO < 0.5 without GREEN-BY-DESIGN justification, orchestrator must choose remediation option A or B | draft | CAP-016 | S-7.03 |
| [BC-8.30.002](ss-08/BC-8.30.002.md) | tdd_mode=facade modifies per-story-delivery semantics and mandates mutation testing at wave gate | draft | CAP-016 | S-7.03 |

> **Note:** BC-8.29.001, BC-8.29.002, BC-8.29.003, and BC-8.30.002 are listed in the SS-05 section above (authoritative subsystem) but their physical files live in the `ss-08/` directory (historical artifact from initial allocation). Per POLICY 1 (append-only numbering), file IDs are immutable; subsystem authority comes from BC frontmatter, not directory.

### SS-06 — Skill Catalog (BC-6)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-6.01.001](ss-06/BC-6.01.001.md) | brownfield-ingest enforces strict-binary novelty | draft | TBD | TBD |
| [BC-6.01.002](ss-06/BC-6.01.002.md) | brownfield-ingest "Iron Law" — no round completion without honest convergence check | draft | TBD | TBD |
| [BC-6.01.003](ss-06/BC-6.01.003.md) | activate skill requires platform detection success | draft | CAP-007 | S-0.03 |
| [BC-6.01.004](ss-06/BC-6.01.004.md) | activate skill copies hooks.json.<platform> to hooks.json then verifies dispatcher binary | draft | CAP-007 | S-2.06 |
| [BC-6.01.005](ss-06/BC-6.01.005.md) | activate skill writes platform + plugin version + activated_at to .claude/settings.local.json | draft | CAP-007 | S-2.06 |
| [BC-6.01.006](ss-06/BC-6.01.006.md) | activate drift warns on cross-host re-activation | draft | CAP-007 | S-2.06 |
| [BC-6.02.001](ss-06/BC-6.02.001.md) | SKILL.md frontmatter requires `name` and `description`; both are non-empty strings | draft | CAP-TBD | TBD |
| [BC-6.02.002](ss-06/BC-6.02.002.md) | SKILL.md description supports YAML block scalar (`>` folded) for multi-line text | draft | CAP-TBD | TBD |
| [BC-6.02.003](ss-06/BC-6.02.003.md) | Skill invocation surface is `/vsdd-factory:<skill-name>` slash command | draft | CAP-TBD | TBD |
| [BC-6.02.004](ss-06/BC-6.02.004.md) | Skills with `disable-model-invocation: true` are dispatcher-only — model cannot self-invoke | draft | CAP-TBD | TBD |
| [BC-6.02.005](ss-06/BC-6.02.005.md) | Skills with `allowed-tools:` whitelist restrict tool surface inside the skill body | draft | CAP-TBD | TBD |
| [BC-6.02.006](ss-06/BC-6.02.006.md) | "Announce at Start" protocol — verbatim opening line per skill | draft | CAP-TBD | TBD |
| [BC-6.02.007](ss-06/BC-6.02.007.md) | Skills SHALL link to template files via `${CLAUDE_PLUGIN_ROOT}/templates/...` references | draft | CAP-TBD | TBD |
| [BC-6.02.008](ss-06/BC-6.02.008.md) | Skill quality gates expressed as a "Hard Gate" or "Iron Law" prose section | draft | CAP-TBD | TBD |
| [BC-6.02.009](ss-06/BC-6.02.009.md) | Skill bodies MAY include a "Red Flags" table to enumerate fabrication / shortcut anti-patterns | draft | CAP-TBD | TBD |
| [BC-6.02.010](ss-06/BC-6.02.010.md) | Skills that dispatch sub-agents declare a "Canonical Source" or single-source-of-truth playbook r... | draft | CAP-TBD | TBD |
| [BC-6.02.011](ss-06/BC-6.02.011.md) | Skills with `argument-hint:` declare inline `$ARGUMENTS[N]` / `$ARGUMENTS` semantics | draft | CAP-TBD | TBD |
| [BC-6.02.012](ss-06/BC-6.02.012.md) | Skill output paths follow `${CLAUDE_PLUGIN_ROOT}` / `.factory/` placement convention | draft | CAP-TBD | TBD |
| [BC-6.03.001](ss-06/BC-6.03.001.md) | activate: skill identity contract | draft | CAP-007 | S-2.06 |
| [BC-6.03.002](ss-06/BC-6.03.002.md) | activate: aborts on unsupported platform | draft | CAP-007 | S-0.03 |
| [BC-6.03.003](ss-06/BC-6.03.003.md) | activate: drift warning on host change | draft | CAP-007 | S-2.06 |
| [BC-6.03.004](ss-06/BC-6.03.004.md) | activate: writes activation block with three named fields | draft | CAP-007 | S-2.06 |
| [BC-6.03.005](ss-06/BC-6.03.005.md) | activate: dry-run mode performs no writes | draft | CAP-007 | S-2.06 |
| [BC-6.03.006](ss-06/BC-6.03.006.md) | activate: applies per-platform variant via apply-platform.sh | draft | CAP-007 | S-2.06 |
| [BC-6.03.007](ss-06/BC-6.03.007.md) | deactivate: skill identity (inverse of activate) | draft | TBD | TBD |
| [BC-6.03.008](ss-06/BC-6.03.008.md) | deactivate: sanity-check before clobbering | draft | TBD | TBD |
| [BC-6.03.009](ss-06/BC-6.03.009.md) | deactivate: empty-file disposition asks user | draft | TBD | TBD |
| [BC-6.04.001](ss-06/BC-6.04.001.md) | adversarial-review: skill identity contract | draft | TBD | TBD |
| [BC-6.04.002](ss-06/BC-6.04.002.md) | adversarial-review: announces verbatim before any other action | draft | TBD | TBD |
| [BC-6.04.003](ss-06/BC-6.04.003.md) | adversarial-review: minimum 3 consecutive clean passes for convergence | draft | TBD | TBD |
| [BC-6.04.004](ss-06/BC-6.04.004.md) | adversarial-review: filename collision guard pre-flight | draft | TBD | TBD |
| [BC-6.04.005](ss-06/BC-6.04.005.md) | adversarial-review: policy rubric auto-loading from policies.yaml | draft | TBD | TBD |
| [BC-6.04.006](ss-06/BC-6.04.006.md) | adversarial-review: post-adversary persistence via state-manager | draft | TBD | TBD |
| [BC-6.04.007](ss-06/BC-6.04.007.md) | adversarial-review: trajectory monotonicity (findings never increase) | draft | TBD | TBD |
| [BC-6.04.008](ss-06/BC-6.04.008.md) | agent-file-review: skill identity contract | draft | TBD | TBD |
| [BC-6.04.009](ss-06/BC-6.04.009.md) | agent-file-review: token budget thresholds (PASS/WARN/FAIL) | draft | TBD | TBD |
| [BC-6.04.010](ss-06/BC-6.04.010.md) | agent-file-review: 15-check list runs all checks | draft | TBD | TBD |
| [BC-6.04.011](ss-06/BC-6.04.011.md) | agent-file-review: tool profile match against openclaw.json (FAIL on mismatch) | draft | TBD | TBD |
| [BC-6.04.012](ss-06/BC-6.04.012.md) | agent-file-review: batch summary mode | draft | TBD | TBD |
| [BC-6.04.013](ss-06/BC-6.04.013.md) | code-delivery: skill identity (post-convergence delivery) | draft | TBD | TBD |
| [BC-6.04.014](ss-06/BC-6.04.014.md) | code-delivery: pre-push test gate via before_tool_call hook | draft | TBD | TBD |
| [BC-6.04.015](ss-06/BC-6.04.015.md) | code-delivery: per-AC demo evidence with both success and error paths | draft | TBD | TBD |
| [BC-6.04.016](ss-06/BC-6.04.016.md) | code-delivery: 4-model-family review (4th model in pr-reviewer) | draft | TBD | TBD |
| [BC-6.04.017](ss-06/BC-6.04.017.md) | code-delivery: review convergence loop (max 10 cycles) | draft | TBD | TBD |
| [BC-6.04.018](ss-06/BC-6.04.018.md) | code-delivery: autonomy-level-driven merge decision | draft | TBD | TBD |
| [BC-6.04.019](ss-06/BC-6.04.019.md) | fix-pr-delivery: Identity — streamlined fix PR flow with same rigor minus stubs/Red Gate/wave gates | draft | CAP-TBD | TBD |
| [BC-6.04.020](ss-06/BC-6.04.020.md) | fix-pr-delivery: Branch and PR title naming uses `fix/FIX-P[phase]-NNN` and `fix(FIX-P[phase]-NNN): ...` | draft | CAP-TBD | TBD |
| [BC-6.04.021](ss-06/BC-6.04.021.md) | fix-pr-delivery: Demo recording is conditional on behavior-changing fixes | draft | CAP-TBD | TBD |
| [BC-6.04.022](ss-06/BC-6.04.022.md) | fix-pr-delivery: Max 10 review cycles before convergence or exhaustion | draft | CAP-TBD | TBD |
| [BC-6.04.023](ss-06/BC-6.04.023.md) | fix-pr-delivery: Hardening fixes re-run only failing checks, not all checks | draft | CAP-TBD | TBD |
| [BC-6.04.024](ss-06/BC-6.04.024.md) | fix-pr-delivery: Output is fix PR merged to develop with worktree cleaned up | draft | CAP-TBD | TBD |
| [BC-6.04.025](ss-06/BC-6.04.025.md) | holdout-eval: Identity — runs holdout evaluation with strict information asymmetry, returns satisfaction scores | draft | CAP-TBD | TBD |
| [BC-6.04.026](ss-06/BC-6.04.026.md) | holdout-eval: Iron Law — evaluator MUST NOT see specs, source, BCs, architecture, or prior reviews | draft | CAP-TBD | TBD |
| [BC-6.04.027](ss-06/BC-6.04.027.md) | holdout-eval: Gate is mean satisfaction ≥ 0.85 AND every critical scenario ≥ 0.60 | draft | CAP-TBD | TBD |
| [BC-6.04.028](ss-06/BC-6.04.028.md) | holdout-eval: Output written to .factory/holdout-scenarios/evaluations/wave-<N>/ | draft | CAP-TBD | TBD |
| [BC-6.04.029](ss-06/BC-6.04.029.md) | phase-4-holdout-evaluation: Identity — Phase 4 entry point with scenario rotation + holdout-eval skill | draft | CAP-TBD | TBD |
| [BC-6.04.030](ss-06/BC-6.04.030.md) | phase-4-holdout-evaluation: Gate — adversary tier (GPT-5.4 not Claude), mean ≥0.85, no must-pass <0.6, std-dev <0.15, 80% rotation | draft | CAP-TBD | TBD |
| [BC-6.04.031](ss-06/BC-6.04.031.md) | phase-4-holdout-evaluation: Direct command is /vsdd-factory:holdout-eval | draft | CAP-TBD | TBD |
| [BC-6.04.032](ss-06/BC-6.04.032.md) | session-review: identity, trigger, primary agent | draft | CAP-TBD | TBD |
| [BC-6.04.033](ss-06/BC-6.04.033.md) | session-review: 8 analysis dimensions | draft | CAP-TBD | TBD |
| [BC-6.04.034](ss-06/BC-6.04.034.md) | session-review: 10 proposal categories with routing | draft | CAP-TBD | TBD |
| [BC-6.04.035](ss-06/BC-6.04.035.md) | session-review: 72h non-blocking timeout | draft | CAP-TBD | TBD |
| [BC-6.04.036](ss-06/BC-6.04.036.md) | session-review: cross-run pattern database + benchmarks | draft | CAP-TBD | TBD |
| [BC-6.04.037](ss-06/BC-6.04.037.md) | session-review: failure-mode safety (incomplete logs / missing cost / corrupt pattern db) | draft | CAP-TBD | TBD |
| [BC-6.05.001](ss-06/BC-6.05.001.md) | brownfield-ingest: skill identity (broad-then-converge protocol) | draft | TBD | TBD |
| [BC-6.05.002](ss-06/BC-6.05.002.md) | brownfield-ingest: announces verbatim and creates phase A/B/B5/B6/C TodoWrite entries | draft | TBD | TBD |
| [BC-6.05.003](ss-06/BC-6.05.003.md) | brownfield-ingest: source acquisition into .reference/ | draft | TBD | TBD |
| [BC-6.05.004](ss-06/BC-6.05.004.md) | brownfield-ingest: strict-binary novelty (NITPICK token only) | draft | TBD | TBD |
| [BC-6.05.005](ss-06/BC-6.05.005.md) | brownfield-ingest: minimum 2 deepening rounds, no fixed maximum | draft | TBD | TBD |
| [BC-6.05.006](ss-06/BC-6.05.006.md) | brownfield-ingest: honest convergence clause in every round prompt | draft | TBD | TBD |
| [BC-6.05.007](ss-06/BC-6.05.007.md) | brownfield-ingest: Phase B.5 coverage audit is mandatory | draft | TBD | TBD |
| [BC-6.05.008](ss-06/BC-6.05.008.md) | brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split | draft | TBD | TBD |
| [BC-6.05.009](ss-06/BC-6.05.009.md) | competitive-monitoring: skill identity contract | draft | TBD | TBD |
| [BC-6.05.010](ss-06/BC-6.05.010.md) | competitive-monitoring: urgency classification HIGH/MEDIUM/LOW | draft | TBD | TBD |
| [BC-6.05.011](ss-06/BC-6.05.011.md) | competitive-monitoring: VERIFIED/UNVERIFIED flagging on findings | draft | TBD | TBD |
| [BC-6.05.012](ss-06/BC-6.05.012.md) | customer-feedback-ingestion: skill identity (read-only, no customer interaction) | draft | TBD | TBD |
| [BC-6.05.013](ss-06/BC-6.05.013.md) | customer-feedback-ingestion: 5 categorization buckets with priority | draft | TBD | TBD |
| [BC-6.05.014](ss-06/BC-6.05.014.md) | customer-feedback-ingestion: deduplication via 0.80 semantic similarity threshold | draft | TBD | TBD |
| [BC-6.05.015](ss-06/BC-6.05.015.md) | discovery-engine: skill identity (autonomous opportunity research) | draft | TBD | TBD |
| [BC-6.05.016](ss-06/BC-6.05.016.md) | discovery-engine: 2 modes (Feature Discovery vs Product Discovery) | draft | TBD | TBD |
| [BC-6.05.017](ss-06/BC-6.05.017.md) | discovery-engine: 7-dimension scoring with weights summing to 1.00 | draft | TBD | TBD |
| [BC-6.05.018](ss-06/BC-6.05.018.md) | discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent) | draft | TBD | TBD |
| [BC-6.05.019](ss-06/BC-6.05.019.md) | disposition-pass: skill identity (Pass 9, vision-lens re-examination) | draft | TBD | TBD |
| [BC-6.05.020](ss-06/BC-6.05.020.md) | disposition-pass: 4-bucket mandatory categorization | draft | TBD | TBD |
| [BC-6.05.021](ss-06/BC-6.05.021.md) | disposition-pass: every disposition tied to named vision section | draft | TBD | TBD |
| [BC-6.05.022](ss-06/BC-6.05.022.md) | disposition-pass: parallelism in batches of 10 with --all | draft | TBD | TBD |
| [BC-6.05.023](ss-06/BC-6.05.023.md) | disposition-pass: vision SHA tracked in rollup header for staleness | draft | TBD | TBD |
| [BC-6.05.024](ss-06/BC-6.05.024.md) | intelligence-synthesis: Identity — correlates market/feedback/competitive/analytics into scored insights | draft | CAP-TBD | TBD |
| [BC-6.05.025](ss-06/BC-6.05.025.md) | intelligence-synthesis: Market research is the only required input; works with partial data | draft | CAP-TBD | TBD |
| [BC-6.05.026](ss-06/BC-6.05.026.md) | intelligence-synthesis: Themes formed by semantic clustering across sources, not per-source listing | draft | CAP-TBD | TBD |
| [BC-6.05.027](ss-06/BC-6.05.027.md) | intelligence-synthesis: Insights scored on 7 dimensions including evidence_strength | draft | CAP-TBD | TBD |
| [BC-6.05.028](ss-06/BC-6.05.028.md) | intelligence-synthesis: Routing — composite ≥0.7 AND evidence ≥0.6 → Brief; URGENT competitive HIGH triggers immediate human notification | draft | CAP-TBD | TBD |
| [BC-6.05.029](ss-06/BC-6.05.029.md) | intelligence-synthesis: Output is insights-YYYY-MM-DD.md with frontmatter and per-insight detail | draft | CAP-TBD | TBD |
| [BC-6.05.030](ss-06/BC-6.05.030.md) | market-intelligence-assessment: Identity — mandatory pre-spec gate producing GO/CAUTION/STOP | draft | CAP-TBD | TBD |
| [BC-6.05.031](ss-06/BC-6.05.031.md) | market-intelligence-assessment: 5 parallel research tracks via research-agent + Perplexity | draft | CAP-TBD | TBD |
| [BC-6.05.032](ss-06/BC-6.05.032.md) | market-intelligence-assessment: Recommendation criteria — GO requires pain confirmed + market viable + differentiation + manageable risks | draft | CAP-TBD | TBD |
| [BC-6.05.033](ss-06/BC-6.05.033.md) | market-intelligence-assessment: Depth scaled by input level L0-L4; L4 has auto-GO without human gate | draft | CAP-TBD | TBD |
| [BC-6.05.034](ss-06/BC-6.05.034.md) | market-intelligence-assessment: Output is market-intel.md; STOP override is recorded with reasoning | draft | CAP-TBD | TBD |
| [BC-6.05.035](ss-06/BC-6.05.035.md) | market-intelligence-assessment: Quality Gate — assumptions explicitly flagged for human validation | draft | CAP-TBD | TBD |
| [BC-6.05.036](ss-06/BC-6.05.036.md) | multi-repo-phase-0-synthesis: Identity — synthesizes per-repo ingestion outputs into unified project context | draft | CAP-TBD | TBD |
| [BC-6.05.037](ss-06/BC-6.05.037.md) | multi-repo-phase-0-synthesis: 8 sequential synthesis steps with named agent per step | draft | CAP-TBD | TBD |
| [BC-6.05.038](ss-06/BC-6.05.038.md) | multi-repo-phase-0-synthesis: Adversary review uses information asymmetry wall — cannot see raw codebase | draft | CAP-TBD | TBD |
| [BC-6.05.039](ss-06/BC-6.05.039.md) | multi-repo-phase-0-synthesis: All T1 writes routed through state-manager | draft | CAP-TBD | TBD |
| [BC-6.05.040](ss-06/BC-6.05.040.md) | multi-repo-phase-0-synthesis: Quality Gate — unified project-context.md exists with cross-repo dependencies, conventions, validation | draft | CAP-TBD | TBD |
| [BC-6.05.041](ss-06/BC-6.05.041.md) | multi-repo-phase-0-synthesis: Failure — incomplete per-repo ingestion halts synthesis | draft | CAP-TBD | TBD |
| [BC-6.05.042](ss-06/BC-6.05.042.md) | planning-research: Identity — domain/market/technical research via Perplexity + Context7 | draft | CAP-TBD | TBD |
| [BC-6.05.043](ss-06/BC-6.05.043.md) | planning-research: Cross-reference findings across ≥2 independent sources; date-stamp all findings | draft | CAP-TBD | TBD |
| [BC-6.05.044](ss-06/BC-6.05.044.md) | planning-research: Output is research-report.md + research-sources.md | draft | CAP-TBD | TBD |
| [BC-6.05.045](ss-06/BC-6.05.045.md) | planning-research: Quality Gate — sources cited with URLs+dates, uncertainties flagged, ≥2 sources cross-referenced | draft | CAP-TBD | TBD |
| [BC-6.05.046](ss-06/BC-6.05.046.md) | planning-research: Failure — MCP tools unavailable → use training data with explicit "UNVERIFIED" disclaimer | draft | CAP-TBD | TBD |
| [BC-6.05.047](ss-06/BC-6.05.047.md) | research: identity & sub-agent fork | draft | CAP-TBD | TBD |
| [BC-6.05.048](ss-06/BC-6.05.048.md) | research: domain vs general routing by first arg | draft | CAP-TBD | TBD |
| [BC-6.05.049](ss-06/BC-6.05.049.md) | research: pre-run cache scan + post-run index update + factory commit | draft | CAP-TBD | TBD |
| [BC-6.05.050](ss-06/BC-6.05.050.md) | research-cache-ops: identity & wraps research-cache binary | draft | CAP-TBD | TBD |
| [BC-6.05.051](ss-06/BC-6.05.051.md) | research-cache-ops: six operations (stats/key/has/get/put/clear) | draft | CAP-TBD | TBD |
| [BC-6.05.052](ss-06/BC-6.05.052.md) | research-cache-ops: SHA-256 deterministic key + research-agent integration pattern | draft | CAP-TBD | TBD |
| [BC-6.05.053](ss-06/BC-6.05.053.md) | semport-analyze: identity, two modes, reference resolution | draft | CAP-TBD | TBD |
| [BC-6.05.054](ss-06/BC-6.05.054.md) | semport-analyze: incremental protocol is delta-only (5 steps) | draft | CAP-TBD | TBD |
| [BC-6.05.055](ss-06/BC-6.05.055.md) | semport-analyze: full mode runs codebase-analyzer 6-pass protocol | draft | CAP-TBD | TBD |
| [BC-6.05.056](ss-06/BC-6.05.056.md) | semport-analyze: language idiom mapping table (Python/TS/Rust pairs) | draft | CAP-TBD | TBD |
| [BC-6.05.057](ss-06/BC-6.05.057.md) | semport-analyze: validate-extraction agent post-pass (max 3 iterations) | draft | CAP-TBD | TBD |
| [BC-6.05.058](ss-06/BC-6.05.058.md) | semport-analyze: outputs and post-skill report | draft | CAP-TBD | TBD |
| [BC-6.06.001](ss-06/BC-6.06.001.md) | check-input-drift: skill identity contract | draft | TBD | TBD |
| [BC-6.06.002](ss-06/BC-6.06.002.md) | check-input-drift: scan via single binary, not inline shell loops | draft | TBD | TBD |
| [BC-6.06.003](ss-06/BC-6.06.003.md) | check-input-drift: mandatory resolve step after scan | draft | TBD | TBD |
| [BC-6.06.004](ss-06/BC-6.06.004.md) | check-input-drift: cluster-drift triage before bulk --update | draft | TBD | TBD |
| [BC-6.06.005](ss-06/BC-6.06.005.md) | check-state-health: skill identity (read-only diagnostic) | draft | TBD | TBD |
| [BC-6.06.006](ss-06/BC-6.06.006.md) | check-state-health: 7 numbered checks executed in order | draft | TBD | TBD |
| [BC-6.06.007](ss-06/BC-6.06.007.md) | check-state-health: stale-phase detection patterns | draft | TBD | TBD |
| [BC-6.06.008](ss-06/BC-6.06.008.md) | check-state-health: content-routing antipattern catalog | draft | TBD | TBD |
| [BC-6.06.009](ss-06/BC-6.06.009.md) | compact-state: skill identity (extract historical content from STATE.md) | draft | TBD | TBD |
| [BC-6.06.010](ss-06/BC-6.06.010.md) | compact-state: 7-pattern extraction map | draft | TBD | TBD |
| [BC-6.06.011](ss-06/BC-6.06.011.md) | compact-state: never-deletes safety guarantee | draft | TBD | TBD |
| [BC-6.06.012](ss-06/BC-6.06.012.md) | compact-state: post-compaction STATE.md <200 lines + verify | draft | TBD | TBD |
| [BC-6.06.013](ss-06/BC-6.06.013.md) | convergence-check: skill identity (Phase 7, 7-dimension validation) | draft | TBD | TBD |
| [BC-6.06.014](ss-06/BC-6.06.014.md) | convergence-check: iron law all-7-must-pass | draft | TBD | TBD |
| [BC-6.06.015](ss-06/BC-6.06.015.md) | convergence-check: per-dimension pass criteria | draft | TBD | TBD |
| [BC-6.06.016](ss-06/BC-6.06.016.md) | convergence-check: writes report at .factory/cycles/<current>/convergence-report.md | draft | TBD | TBD |
| [BC-6.06.017](ss-06/BC-6.06.017.md) | convergence-tracking: skill identity (quantitative metrics-driven) | draft | TBD | TBD |
| [BC-6.06.018](ss-06/BC-6.06.018.md) | convergence-tracking: spec convergence formula (Novelty < 0.15 + median severity decay) | draft | TBD | TBD |
| [BC-6.06.019](ss-06/BC-6.06.019.md) | convergence-tracking: tier-based mutation kill rate thresholds | draft | TBD | TBD |
| [BC-6.06.020](ss-06/BC-6.06.020.md) | convergence-tracking: convergence index formula (CI(i)) | draft | TBD | TBD |
| [BC-6.06.021](ss-06/BC-6.06.021.md) | recover-state: identity, dry-run, and backup | draft | CAP-TBD | TBD |
| [BC-6.06.022](ss-06/BC-6.06.022.md) | recover-state: artifact directory probe table is exhaustive | draft | CAP-TBD | TBD |
| [BC-6.06.023](ss-06/BC-6.06.023.md) | recover-state: phase decision tree is total and ordered | draft | CAP-TBD | TBD |
| [BC-6.06.024](ss-06/BC-6.06.024.md) | recover-state: requires user approval before write | draft | CAP-TBD | TBD |
| [BC-6.06.025](ss-06/BC-6.06.025.md) | recover-state: documented limitations are honored (no fabrication) | draft | CAP-TBD | TBD |
| [BC-6.06.026](ss-06/BC-6.06.026.md) | state-burst: identity & defect-class context | draft | CAP-TBD | TBD |
| [BC-6.06.027](ss-06/BC-6.06.027.md) | state-burst: announces protocol verbatim | draft | CAP-TBD | TBD |
| [BC-6.06.028](ss-06/BC-6.06.028.md) | state-burst: Stage 1 with `15fa97e6` placeholder + tense rule | draft | CAP-TBD | TBD |
| [BC-6.06.029](ss-06/BC-6.06.029.md) | state-burst: Stage 2 global SHA replace + backfill commit | draft | CAP-TBD | TBD |
| [BC-6.06.030](ss-06/BC-6.06.030.md) | state-burst: refuses 3rd commit + recovery via `git reset --soft HEAD~2` | draft | CAP-TBD | TBD |
| [BC-6.06.031](ss-06/BC-6.06.031.md) | state-burst: documented bypass paths | draft | CAP-TBD | TBD |
| [BC-6.06.032](ss-06/BC-6.06.032.md) | state-update: identity & internal-only contract | draft | CAP-TBD | TBD |
| [BC-6.06.033](ss-06/BC-6.06.033.md) | state-update: 4-step procedure (read → frontmatter → history → commit) | draft | CAP-TBD | TBD |
| [BC-6.06.034](ss-06/BC-6.06.034.md) | state-update: enumerates 5 pipeline statuses + 7 phase IDs | draft | CAP-TBD | TBD |
| [BC-6.06.035](ss-06/BC-6.06.035.md) | wave-gate: identity, allowed tools, pre-flight | draft | CAP-TBD | TBD |
| [BC-6.06.036](ss-06/BC-6.06.036.md) | wave-gate: announces protocol verbatim + TodoWrite per gate | draft | CAP-TBD | TBD |
| [BC-6.06.037](ss-06/BC-6.06.037.md) | wave-gate: gate sequence is load-bearing (1→2→3→4→5→6, stop on first failure) | draft | CAP-TBD | TBD |
| [BC-6.06.038](ss-06/BC-6.06.038.md) | wave-gate: 8-row red-flag rationalization table | draft | CAP-TBD | TBD |
| [BC-6.06.039](ss-06/BC-6.06.039.md) | wave-gate: GATE_CHECK telemetry lines (validated by hook) | draft | CAP-TBD | TBD |
| [BC-6.06.040](ss-06/BC-6.06.040.md) | wave-gate: outputs and post-pass guidance | draft | CAP-TBD | TBD |
| [BC-6.06.041](ss-06/BC-6.06.041.md) | wave-scheduling: identity & topo-sort algorithm | draft | CAP-TBD | TBD |
| [BC-6.06.042](ss-06/BC-6.06.042.md) | wave-scheduling: parallel groups (≤2 S/M or ≤1 L/XL per group) | draft | CAP-TBD | TBD |
| [BC-6.06.043](ss-06/BC-6.06.043.md) | wave-scheduling: pipeline overlap (Wave N+1 stubs while Wave N implements) | draft | CAP-TBD | TBD |
| [BC-6.06.044](ss-06/BC-6.06.044.md) | wave-scheduling: wave-schedule.md output + quality gate | draft | CAP-TBD | TBD |
| [BC-6.06.045](ss-06/BC-6.06.045.md) | wave-scheduling: failure modes (cycle/missing dep/no roots) | draft | CAP-TBD | TBD |
| [BC-6.06.046](ss-06/BC-6.06.046.md) | wave-status: identity & read-only contract | draft | CAP-TBD | TBD |
| [BC-6.07.001](ss-06/BC-6.07.001.md) | conform-to-template: skill identity (additive only — never deletes) | draft | TBD | TBD |
| [BC-6.07.002](ss-06/BC-6.07.002.md) | conform-to-template: refuses table-column changes and section reordering | draft | TBD | TBD |
| [BC-6.07.003](ss-06/BC-6.07.003.md) | conform-to-template: user approval gate before write | draft | TBD | TBD |
| [BC-6.07.004](ss-06/BC-6.07.004.md) | conform-to-template: post-conformance re-validation reports before/after | draft | TBD | TBD |
| [BC-6.07.005](ss-06/BC-6.07.005.md) | consistency-validation: skill identity (cross-document validator) | draft | TBD | TBD |
| [BC-6.07.006](ss-06/BC-6.07.006.md) | consistency-validation: 36 numbered rules executed in order | draft | TBD | TBD |
| [BC-6.07.007](ss-06/BC-6.07.007.md) | consistency-validation: index-first validation precedes detail loading (DF-021) | draft | TBD | TBD |
| [BC-6.07.008](ss-06/BC-6.07.008.md) | consistency-validation: BC clause reverse-coverage severity (Rule 25) | draft | TBD | TBD |
| [BC-6.07.009](ss-06/BC-6.07.009.md) | consistency-validation: NFR-to-Story severity by priority tier (Rule 27) | draft | TBD | TBD |
| [BC-6.07.010](ss-06/BC-6.07.010.md) | create-architecture: skill identity + iron law (verification feasibility) | draft | TBD | TBD |
| [BC-6.07.011](ss-06/BC-6.07.011.md) | create-architecture: ADR style for every decision | draft | TBD | TBD |
| [BC-6.07.012](ss-06/BC-6.07.012.md) | create-architecture: sharded output (ARCH-INDEX + section files) | draft | TBD | TBD |
| [BC-6.07.013](ss-06/BC-6.07.013.md) | create-architecture: VP files written to verification-properties/ | draft | TBD | TBD |
| [BC-6.07.014](ss-06/BC-6.07.014.md) | create-brief: skill identity + hard gate | draft | TBD | TBD |
| [BC-6.07.015](ss-06/BC-6.07.015.md) | create-brief: factory-health prerequisite + research check | draft | TBD | TBD |
| [BC-6.07.016](ss-06/BC-6.07.016.md) | create-brief: questions one-at-a-time, multiple choice when possible | draft | TBD | TBD |
| [BC-6.07.017](ss-06/BC-6.07.017.md) | create-brief: writes product-brief.md with 8 named sections + state-update | draft | TBD | TBD |
| [BC-6.07.018](ss-06/BC-6.07.018.md) | create-domain-spec: skill identity (sharded L2 spec) | draft | TBD | TBD |
| [BC-6.07.019](ss-06/BC-6.07.019.md) | create-domain-spec: 3-pass extraction (structural + behavioral + context) | draft | TBD | TBD |
| [BC-6.07.020](ss-06/BC-6.07.020.md) | create-domain-spec: sharded output structure (5 named files) | draft | TBD | TBD |
| [BC-6.07.021](ss-06/BC-6.07.021.md) | create-excalidraw: skill identity (programmatic .excalidraw JSON generation) | draft | TBD | TBD |
| [BC-6.07.022](ss-06/BC-6.07.022.md) | create-excalidraw: deterministic IDs (not random UUIDs) | draft | TBD | TBD |
| [BC-6.07.023](ss-06/BC-6.07.023.md) | create-excalidraw: arrow points property required (workaround for export bug) | draft | TBD | TBD |
| [BC-6.07.024](ss-06/BC-6.07.024.md) | create-prd: skill identity + hard gate | draft | TBD | TBD |
| [BC-6.07.025](ss-06/BC-6.07.025.md) | create-prd: each BC must be testable, unambiguous, complete | draft | TBD | TBD |
| [BC-6.07.026](ss-06/BC-6.07.026.md) | create-prd: 3 named PRD supplements | draft | TBD | TBD |
| [BC-6.07.027](ss-06/BC-6.07.027.md) | create-prd: BC reference repos integration (Source line in BC traceability) | draft | TBD | TBD |
| [BC-6.07.028](ss-06/BC-6.07.028.md) | create-story: skill identity + hard gate | draft | TBD | TBD |
| [BC-6.07.029](ss-06/BC-6.07.029.md) | create-story: 7 plan-failure patterns block proceeding | draft | TBD | TBD |
| [BC-6.07.030](ss-06/BC-6.07.030.md) | create-story: forbidden dependencies + version pin enforcement | draft | TBD | TBD |
| [BC-6.07.031](ss-06/BC-6.07.031.md) | decompose-stories: skill identity + iron law (BC traceability) | draft | TBD | TBD |
| [BC-6.07.032](ss-06/BC-6.07.032.md) | decompose-stories: 13-point story size limit (must split before implementation) | draft | TBD | TBD |
| [BC-6.07.033](ss-06/BC-6.07.033.md) | decompose-stories: dependency graph acyclicity verified programmatically | draft | TBD | TBD |
| [BC-6.07.034](ss-06/BC-6.07.034.md) | decompose-stories: 5 named output artifacts + holdout scenarios | draft | TBD | TBD |
| [BC-6.07.035](ss-06/BC-6.07.035.md) | guided-brief-creation: Identity — staged elicitation from raw idea to product brief | draft | CAP-TBD | TBD |
| [BC-6.07.036](ss-06/BC-6.07.036.md) | guided-brief-creation: Hard gate — must complete brief before any PRD/architecture/implementation | draft | CAP-TBD | TBD |
| [BC-6.07.037](ss-06/BC-6.07.037.md) | guided-brief-creation: Capture-don't-interrupt rule preserves human creative flow | draft | CAP-TBD | TBD |
| [BC-6.07.038](ss-06/BC-6.07.038.md) | guided-brief-creation: Output is product-brief.md (and elicitation-notes.md if applicable) | draft | CAP-TBD | TBD |
| [BC-6.07.039](ss-06/BC-6.07.039.md) | guided-brief-creation: Failure mode — contradictory requirements halt elicitation for human resolution | draft | CAP-TBD | TBD |
| [BC-6.07.040](ss-06/BC-6.07.040.md) | spec-drift: identity & forked Explore agent | draft | CAP-TBD | TBD |
| [BC-6.07.041](ss-06/BC-6.07.041.md) | spec-drift: scans 4 spec dirs + checks naming + finds orphans | draft | CAP-TBD | TBD |
| [BC-6.07.042](ss-06/BC-6.07.042.md) | spec-drift: writes spec-drift-report.md to current cycle | draft | CAP-TBD | TBD |
| [BC-6.07.043](ss-06/BC-6.07.043.md) | spec-versioning: identity & semver scheme | draft | CAP-TBD | TBD |
| [BC-6.07.044](ss-06/BC-6.07.044.md) | spec-versioning: bump-type rules (MAJOR/MINOR/PATCH) | draft | CAP-TBD | TBD |
| [BC-6.07.045](ss-06/BC-6.07.045.md) | spec-versioning: per-story spec_version + drift detection | draft | CAP-TBD | TBD |
| [BC-6.07.046](ss-06/BC-6.07.046.md) | spec-versioning: L4 immutability rules + locked-VP enforcement | draft | CAP-TBD | TBD |
| [BC-6.07.047](ss-06/BC-6.07.047.md) | spec-versioning: failure-mode safety (inconsistent versions / locked-VP modified / unparseable) | draft | CAP-TBD | TBD |
| [BC-6.07.048](ss-06/BC-6.07.048.md) | traceability-extension: identity & chain semantics | draft | CAP-TBD | TBD |
| [BC-6.07.049](ss-06/BC-6.07.049.md) | traceability-extension: 7 extension rules (IDs new, links append-only, deprecated stays) | draft | CAP-TBD | TBD |
| [BC-6.07.050](ss-06/BC-6.07.050.md) | traceability-extension: architecture-section-level references (DF-021) | draft | CAP-TBD | TBD |
| [BC-6.07.051](ss-06/BC-6.07.051.md) | traceability-extension: chain verification command | draft | CAP-TBD | TBD |
| [BC-6.07.052](ss-06/BC-6.07.052.md) | validate-brief: identity & step-file note | draft | CAP-TBD | TBD |
| [BC-6.07.053](ss-06/BC-6.07.053.md) | validate-brief: structure check requires 6 sections each meeting minimums | draft | CAP-TBD | TBD |
| [BC-6.07.054](ss-06/BC-6.07.054.md) | validate-brief: bloat check (<500/<800/<1500 token bands) | draft | CAP-TBD | TBD |
| [BC-6.07.055](ss-06/BC-6.07.055.md) | validate-brief: implementation-leakage tech-name scanner | draft | CAP-TBD | TBD |
| [BC-6.07.056](ss-06/BC-6.07.056.md) | validate-brief: information density anti-patterns (4 categories + thresholds) | draft | CAP-TBD | TBD |
| [BC-6.07.057](ss-06/BC-6.07.057.md) | validate-brief: market intel cross-check + report file + overall verdict | draft | CAP-TBD | TBD |
| [BC-6.07.058](ss-06/BC-6.07.058.md) | validate-consistency: identity & frontmatter | draft | CAP-TBD | TBD |
| [BC-6.07.059](ss-06/BC-6.07.059.md) | validate-consistency: 7 cross-file checks (BC/VP/Story/Architecture/Counts/Status/Naming) | draft | CAP-TBD | TBD |
| [BC-6.07.060](ss-06/BC-6.07.060.md) | validate-consistency: report format with Failures/Warnings/All Passed | draft | CAP-TBD | TBD |
| [BC-6.07.061](ss-06/BC-6.07.061.md) | validate-template-compliance: identity & three scopes (file/dir/all) | draft | CAP-TBD | TBD |
| [BC-6.07.062](ss-06/BC-6.07.062.md) | validate-template-compliance: template resolution by document_type then path | draft | CAP-TBD | TBD |
| [BC-6.07.063](ss-06/BC-6.07.063.md) | validate-template-compliance: 3-level compliance check (frontmatter/sections/tables) | draft | CAP-TBD | TBD |
| [BC-6.07.064](ss-06/BC-6.07.064.md) | validate-template-compliance: report format (per-file detail + summary table + aggregate counts) | draft | CAP-TBD | TBD |
| [BC-6.07.065](ss-06/BC-6.07.065.md) | validate-template-compliance: documented limitations (no content quality, no value validation) | draft | CAP-TBD | TBD |
| [BC-6.07.066](ss-06/BC-6.07.066.md) | validate-workflow: identity & static-only contract | draft | CAP-TBD | TBD |
| [BC-6.07.067](ss-06/BC-6.07.067.md) | validate-workflow: 6 checks (required fields/agent/skill/depends_on/dup names/top-level) | draft | CAP-TBD | TBD |
| [BC-6.07.068](ss-06/BC-6.07.068.md) | validate-workflow: collects all errors (no early bail) + exit code | draft | CAP-TBD | TBD |
| [BC-6.08.001](ss-06/BC-6.08.001.md) | demo-recording: skill identity (CLI/web/API/library) | draft | TBD | TBD |
| [BC-6.08.002](ss-06/BC-6.08.002.md) | demo-recording: 5 detection signals → demo type → tool | draft | TBD | TBD |
| [BC-6.08.003](ss-06/BC-6.08.003.md) | demo-recording: target sizes (WebM <2MB, GIF <5MB, total <25MB) | draft | TBD | TBD |
| [BC-6.08.004](ss-06/BC-6.08.004.md) | demo-recording: every AC has user-observable behavior covered + visual review | draft | TBD | TBD |
| [BC-6.08.005](ss-06/BC-6.08.005.md) | design-drift-detection: skill identity (Sweep 10, UI products only) | draft | TBD | TBD |
| [BC-6.08.006](ss-06/BC-6.08.006.md) | design-drift-detection: emergent pattern threshold (>=3 instances → propose) | draft | TBD | TBD |
| [BC-6.08.007](ss-06/BC-6.08.007.md) | design-drift-detection: graceful skip when no design system | draft | TBD | TBD |
| [BC-6.08.008](ss-06/BC-6.08.008.md) | design-system-bootstrap: skill identity (greenfield + brownfield + feature) | draft | TBD | TBD |
| [BC-6.08.009](ss-06/BC-6.08.009.md) | design-system-bootstrap: minimal bootstrap fallback when no brand guidelines | draft | TBD | TBD |
| [BC-6.08.010](ss-06/BC-6.08.010.md) | design-system-bootstrap: WCAG AA contrast validation (accessibility-auditor) | draft | TBD | TBD |
| [BC-6.08.011](ss-06/BC-6.08.011.md) | excalidraw-export: skill identity (reference-only batch PNG export) | draft | TBD | TBD |
| [BC-6.08.012](ss-06/BC-6.08.012.md) | excalidraw-export: arrow points workaround documented (must have `points`) | draft | TBD | TBD |
| [BC-6.08.013](ss-06/BC-6.08.013.md) | multi-variant-design: Identity — generates 2-3 variants per complex screen scored by 4 agents on 6 dimensions | draft | CAP-TBD | TBD |
| [BC-6.08.014](ss-06/BC-6.08.014.md) | multi-variant-design: Top variant + runner-up presented for human selection or synthesis | draft | CAP-TBD | TBD |
| [BC-6.08.015](ss-06/BC-6.08.015.md) | multi-variant-design: Output is SCR-NNN-variants.md per complex screen | draft | CAP-TBD | TBD |
| [BC-6.08.016](ss-06/BC-6.08.016.md) | multi-variant-design: Failure — score deadlock (within 0.05) presents both with dimension breakdown | draft | CAP-TBD | TBD |
| [BC-6.08.017](ss-06/BC-6.08.017.md) | record-demo: identity & template usage | draft | CAP-TBD | TBD |
| [BC-6.08.018](ss-06/BC-6.08.018.md) | record-demo: per-AC evidence capture (CLI vs web) | draft | CAP-TBD | TBD |
| [BC-6.08.019](ss-06/BC-6.08.019.md) | record-demo: writes demo-report.md with per-AC table | draft | CAP-TBD | TBD |
| [BC-6.08.020](ss-06/BC-6.08.020.md) | record-demo: tool-unavailable fallback never skips evidence | draft | CAP-TBD | TBD |
| [BC-6.08.021](ss-06/BC-6.08.021.md) | record-demo: commits evidence to factory-artifacts | draft | CAP-TBD | TBD |
| [BC-6.08.022](ss-06/BC-6.08.022.md) | responsive-validation: identity, agents, conditional UI gating | draft | CAP-TBD | TBD |
| [BC-6.08.023](ss-06/BC-6.08.023.md) | responsive-validation: 4 mandatory breakpoints (375/768/1024/1440) | draft | CAP-TBD | TBD |
| [BC-6.08.024](ss-06/BC-6.08.024.md) | responsive-validation: critical-failure list is blocking | draft | CAP-TBD | TBD |
| [BC-6.08.025](ss-06/BC-6.08.025.md) | responsive-validation: screenshot evidence + per-screen pass/fail matrix | draft | CAP-TBD | TBD |
| [BC-6.08.026](ss-06/BC-6.08.026.md) | responsive-validation: failure modes (resize/screenshot/auth) | draft | CAP-TBD | TBD |
| [BC-6.08.027](ss-06/BC-6.08.027.md) | storybook-mcp-integration: identity & UI-conditional invocation | draft | CAP-TBD | TBD |
| [BC-6.08.028](ss-06/BC-6.08.028.md) | storybook-mcp-integration: install + config + register procedure | draft | CAP-TBD | TBD |
| [BC-6.08.029](ss-06/BC-6.08.029.md) | storybook-mcp-integration: 6 MCP tools mapped to agent roles | draft | CAP-TBD | TBD |
| [BC-6.08.030](ss-06/BC-6.08.030.md) | storybook-mcp-integration: T1/T2/T3 access pattern via DF-023+DF-027 | draft | CAP-TBD | TBD |
| [BC-6.08.031](ss-06/BC-6.08.031.md) | storybook-mcp-integration: self-healing loop with 10-iteration cap | draft | CAP-TBD | TBD |
| [BC-6.08.032](ss-06/BC-6.08.032.md) | storybook-mcp-integration: reuse-first enforcement before new components | draft | CAP-TBD | TBD |
| [BC-6.08.033](ss-06/BC-6.08.033.md) | storybook-mcp-integration: non-React fallback (manifest only) | draft | CAP-TBD | TBD |
| [BC-6.08.034](ss-06/BC-6.08.034.md) | ui-completeness-check: identity, agents, UI gating, zero-gap rule | draft | CAP-TBD | TBD |
| [BC-6.08.035](ss-06/BC-6.08.035.md) | ui-completeness-check: 3 pipeline points with strictness gradient | draft | CAP-TBD | TBD |
| [BC-6.08.036](ss-06/BC-6.08.036.md) | ui-completeness-check: ui-traceability.yaml schema (screens with components/interactions/responsi... | draft | CAP-TBD | TBD |
| [BC-6.08.037](ss-06/BC-6.08.037.md) | ui-completeness-check: 7-axis gap detection (screens/components/states/interactions/responsive/a1... | draft | CAP-TBD | TBD |
| [BC-6.08.038](ss-06/BC-6.08.038.md) | ui-completeness-check: state coverage (D4) — 4 async states + per-component-type required states | draft | CAP-TBD | TBD |
| [BC-6.08.039](ss-06/BC-6.08.039.md) | ui-completeness-check: 100% fidelity target + fix story generation | draft | CAP-TBD | TBD |
| [BC-6.08.040](ss-06/BC-6.08.040.md) | ui-quality-gate: identity, agents, UI conditional | draft | CAP-TBD | TBD |
| [BC-6.08.041](ss-06/BC-6.08.041.md) | ui-quality-gate: comprehensive checklist across 5 dimensions | draft | CAP-TBD | TBD |
| [BC-6.08.042](ss-06/BC-6.08.042.md) | ui-quality-gate: 4 strictness levels (per-story/wave/build/convergence) | draft | CAP-TBD | TBD |
| [BC-6.08.043](ss-06/BC-6.08.043.md) | ui-quality-gate: failure → FIX-UI-NNN routing | draft | CAP-TBD | TBD |
| [BC-6.08.044](ss-06/BC-6.08.044.md) | ui-quality-gate: performance targets (D8 LCP/FID/CLS/TTI/bundle/images) | draft | CAP-TBD | TBD |
| [BC-6.08.045](ss-06/BC-6.08.045.md) | ui-quality-gate: gate-report.md structure (Gate Level + Result + Checklist + Failures + Perf table) | draft | CAP-TBD | TBD |
| [BC-6.08.046](ss-06/BC-6.08.046.md) | ux-heuristic-evaluation: identity, conditional, four pipeline points | draft | CAP-TBD | TBD |
| [BC-6.08.047](ss-06/BC-6.08.047.md) | ux-heuristic-evaluation: Nielsen 10 heuristics with explicit subchecks | draft | CAP-TBD | TBD |
| [BC-6.08.048](ss-06/BC-6.08.048.md) | ux-heuristic-evaluation: cognitive walkthrough per key task | draft | CAP-TBD | TBD |
| [BC-6.08.049](ss-06/BC-6.08.049.md) | ux-heuristic-evaluation: 0.7 threshold + remediation flagging | draft | CAP-TBD | TBD |
| [BC-6.08.050](ss-06/BC-6.08.050.md) | ux-heuristic-evaluation: report path + failure modes | draft | CAP-TBD | TBD |
| [BC-6.08.051](ss-06/BC-6.08.051.md) | visual-companion: identity, prerequisites, optional setup | draft | CAP-TBD | TBD |
| [BC-6.08.052](ss-06/BC-6.08.052.md) | visual-companion: server lifecycle (start/url/state-dir/stop, 30-min auto-exit) | draft | CAP-TBD | TBD |
| [BC-6.08.053](ss-06/BC-6.08.053.md) | visual-companion: write-loop discipline (Write tool, semantic filenames, no reuse) | draft | CAP-TBD | TBD |
| [BC-6.08.054](ss-06/BC-6.08.054.md) | visual-companion: visual-vs-terminal decision rule | draft | CAP-TBD | TBD |
| [BC-6.08.055](ss-06/BC-6.08.055.md) | visual-companion: excalidraw mode + composed views (screen.json manifest) | draft | CAP-TBD | TBD |
| [BC-6.09.001](ss-06/BC-6.09.001.md) | dtu-creation: skill identity (build behavioral clones) | draft | TBD | TBD |
| [BC-6.09.002](ss-06/BC-6.09.002.md) | dtu-creation: fidelity level driven by SUT usage | draft | TBD | TBD |
| [BC-6.09.003](ss-06/BC-6.09.003.md) | dtu-creation: clone validation via contract tests + Schemathesis | draft | TBD | TBD |
| [BC-6.09.004](ss-06/BC-6.09.004.md) | dtu-validate: skill identity (independent reimplementation comparison) | draft | TBD | TBD |
| [BC-6.09.005](ss-06/BC-6.09.005.md) | dtu-validate: criticality-driven candidacy | draft | TBD | TBD |
| [BC-6.09.006](ss-06/BC-6.09.006.md) | dtu-validate: divergence in CRITICAL = blocking | draft | TBD | TBD |
| [BC-6.10.001](ss-06/BC-6.10.001.md) | deliver-story: skill identity (dispatcher, not implementer) | draft | TBD | TBD |
| [BC-6.10.002](ss-06/BC-6.10.002.md) | deliver-story: 9-step dispatch sequence with exit conditions | draft | TBD | TBD |
| [BC-6.10.003](ss-06/BC-6.10.003.md) | deliver-story: Red Gate verification in step 3 (mandatory) | draft | TBD | TBD |
| [BC-6.10.004](ss-06/BC-6.10.004.md) | deliver-story: verification discipline — never trust agent reports | draft | TBD | TBD |
| [BC-6.10.005](ss-06/BC-6.10.005.md) | deliver-story: context discipline mapping per specialist | draft | TBD | TBD |
| [BC-6.10.006](ss-06/BC-6.10.006.md) | implementation-readiness: Identity — gate between planning and building, validates spec package consistency | draft | CAP-TBD | TBD |
| [BC-6.10.007](ss-06/BC-6.10.007.md) | implementation-readiness: Validation runs 8 dimensions in parallel, not sequential | draft | CAP-TBD | TBD |
| [BC-6.10.008](ss-06/BC-6.10.008.md) | implementation-readiness: PRD bloat check — narrative padding in requirements is a finding | draft | CAP-TBD | TBD |
| [BC-6.10.009](ss-06/BC-6.10.009.md) | implementation-readiness: Context budget warns when total exceeds 60% of agent context window | draft | CAP-TBD | TBD |
| [BC-6.10.010](ss-06/BC-6.10.010.md) | implementation-readiness: PRD implementation leakage scan flags premature tech decisions | draft | CAP-TBD | TBD |
| [BC-6.10.011](ss-06/BC-6.10.011.md) | implementation-readiness: PRD information density — Critical >10, Warning 5-10, Pass <5 issues per page | draft | CAP-TBD | TBD |
| [BC-6.10.012](ss-06/BC-6.10.012.md) | implementation-readiness: Story tokens 300-800; total context ≤60% of agent window | draft | CAP-TBD | TBD |
| [BC-6.10.013](ss-06/BC-6.10.013.md) | implementation-readiness: Output is readiness-report.md with READY\|CONCERNS\|NOT_READY verdict | draft | CAP-TBD | TBD |
| [BC-6.10.014](ss-06/BC-6.10.014.md) | post-feature-validation: Identity — monitors post-ship feedback at 7/30/90-day intervals; entirely optional | draft | CAP-TBD | TBD |
| [BC-6.10.015](ss-06/BC-6.10.015.md) | post-feature-validation: Verdict thresholds — SUCCESS / PARTIAL / MISS based on adoption + feedback ratio + bugs | draft | CAP-TBD | TBD |
| [BC-6.10.016](ss-06/BC-6.10.016.md) | post-feature-validation: Default success criteria — adoption ≥0.10, positive ratio ≥0.6, error rate <5% | draft | CAP-TBD | TBD |
| [BC-6.10.017](ss-06/BC-6.10.017.md) | post-feature-validation: Output is feature-impact-[name]-YYYY-MM-DD.md with adoption + feedback + verdict + recommendations | draft | CAP-TBD | TBD |
| [BC-6.10.018](ss-06/BC-6.10.018.md) | post-feature-validation: Feeds back into discovery — calibration data, new pain points, evidence for backlog | draft | CAP-TBD | TBD |
| [BC-6.10.019](ss-06/BC-6.10.019.md) | post-feature-validation: Quality Gate — only runs when enabled, evidence-based verdict, actionable recommendations, results feed back | draft | CAP-TBD | TBD |
| [BC-6.10.020](ss-06/BC-6.10.020.md) | post-feature-validation: Failure — feedback channels unavailable → analyze available data and note unreachable channels | draft | CAP-TBD | TBD |
| [BC-6.11.001](ss-06/BC-6.11.001.md) | factory-cycles-bootstrap: skill identity (flat → cycle-keyed migration) | draft | TBD | TBD |
| [BC-6.11.002](ss-06/BC-6.11.002.md) | factory-cycles-bootstrap: archives via `git mv` (preserves history) | draft | TBD | TBD |
| [BC-6.11.003](ss-06/BC-6.11.003.md) | factory-cycles-bootstrap: writes .factory/current-cycle pointer | draft | TBD | TBD |
| [BC-6.11.004](ss-06/BC-6.11.004.md) | factory-dashboard: skill identity (read-only diagnostic) | draft | TBD | TBD |
| [BC-6.11.005](ss-06/BC-6.11.005.md) | factory-dashboard: missing files produce "not initialized" notices, not errors | draft | TBD | TBD |
| [BC-6.11.006](ss-06/BC-6.11.006.md) | factory-dashboard: --factory PATH and --days N options | draft | TBD | TBD |
| [BC-6.11.007](ss-06/BC-6.11.007.md) | factory-health: skill identity (auto-repairing worktree validator) | draft | TBD | TBD |
| [BC-6.11.008](ss-06/BC-6.11.008.md) | factory-health: 8 sequential checks with auto-repair on missing structures | draft | TBD | TBD |
| [BC-6.11.009](ss-06/BC-6.11.009.md) | factory-health: STATE.md size thresholds (200/500 lines) | draft | TBD | TBD |
| [BC-6.11.010](ss-06/BC-6.11.010.md) | factory-obs: skill identity (manage local observability stack) | draft | TBD | TBD |
| [BC-6.11.011](ss-06/BC-6.11.011.md) | factory-obs: 9-arg subcommand surface | draft | TBD | TBD |
| [BC-6.11.012](ss-06/BC-6.11.012.md) | factory-obs: multi-factory watched-list at ~/.config/vsdd-factory/watched-factories | draft | TBD | TBD |
| [BC-6.11.013](ss-06/BC-6.11.013.md) | factory-obs: env override port allowlist | draft | TBD | TBD |
| [BC-6.11.014](ss-06/BC-6.11.014.md) | factory-worktree-health: skill identity (blocking precondition) | draft | TBD | TBD |
| [BC-6.11.015](ss-06/BC-6.11.015.md) | factory-worktree-health: workspace isolation guard (Step 0) | draft | TBD | TBD |
| [BC-6.11.016](ss-06/BC-6.11.016.md) | factory-worktree-health: 5-step sync state evaluation matrix | draft | TBD | TBD |
| [BC-6.11.017](ss-06/BC-6.11.017.md) | factory-worktree-health: dual-worktree check for multi-repo mode | draft | TBD | TBD |
| [BC-6.11.018](ss-06/BC-6.11.018.md) | generate-pdf: Identity — convert markdown to 1898 & Co. branded PDF via pandoc + weasyprint | draft | CAP-TBD | TBD |
| [BC-6.11.019](ss-06/BC-6.11.019.md) | generate-pdf: Required frontmatter fields are title, author, date | draft | CAP-TBD | TBD |
| [BC-6.11.020](ss-06/BC-6.11.020.md) | generate-pdf: Output PDF defaults to <input>.pdf in same directory | draft | CAP-TBD | TBD |
| [BC-6.11.021](ss-06/BC-6.11.021.md) | generate-pdf: Errors must be reported with specific solutions per error class | draft | CAP-TBD | TBD |
| [BC-6.11.022](ss-06/BC-6.11.022.md) | jira: Identity is reference-only documentation for ankitpokhrel/jira-cli | draft | CAP-TBD | TBD |
| [BC-6.11.023](ss-06/BC-6.11.023.md) | maintenance-sweep: Identity — periodic sweeps + cleanup PRs through standard quality gates | draft | CAP-TBD | TBD |
| [BC-6.11.024](ss-06/BC-6.11.024.md) | maintenance-sweep: 9 sweep types run in parallel after STARTED commit | draft | CAP-TBD | TBD |
| [BC-6.11.025](ss-06/BC-6.11.025.md) | maintenance-sweep: Dependency audit splits T3 (run scans) and T2 (analyze) | draft | CAP-TBD | TBD |
| [BC-6.11.026](ss-06/BC-6.11.026.md) | maintenance-sweep: Performance regression — >25% triggers PR; 10-25% logs trend; <10% no action | draft | CAP-TBD | TBD |
| [BC-6.11.027](ss-06/BC-6.11.027.md) | maintenance-sweep: Auto-PR quality gate same as Feature Mode (regression + holdout + adversarial + lint) | draft | CAP-TBD | TBD |
| [BC-6.11.028](ss-06/BC-6.11.028.md) | maintenance-sweep: Output is sweep-report-YYYY-MM-DD.md plus per-sweep findings files | draft | CAP-TBD | TBD |
| [BC-6.11.029](ss-06/BC-6.11.029.md) | model-routing: Identity — LiteLLM model tier assignment reference | draft | CAP-TBD | TBD |
| [BC-6.11.030](ss-06/BC-6.11.030.md) | model-routing: Iron rule — Adversary MUST use adversary tier (GPT-5.4), never Claude | draft | CAP-TBD | TBD |
| [BC-6.11.031](ss-06/BC-6.11.031.md) | model-routing: Three-tier fallback — primary → standard fallback → reasoning fallback (for adversary/judgment) or fast fallback → standard (for impl/validation) | draft | CAP-TBD | TBD |
| [BC-6.11.032](ss-06/BC-6.11.032.md) | model-routing: Compounding correctness — pause if budget forces downgrade in P3-P5 | draft | CAP-TBD | TBD |
| [BC-6.11.033](ss-06/BC-6.11.033.md) | multi-repo-health: Identity — scan .worktrees/ for multi-repo layout, report repos with manifests | draft | CAP-TBD | TBD |
| [BC-6.11.034](ss-06/BC-6.11.034.md) | multi-repo-health: Read-only — does not mutate any repo | draft | CAP-TBD | TBD |
| [BC-6.11.035](ss-06/BC-6.11.035.md) | multi-repo-health: Single-repo path — count == 0 reports "single-repo project" and stops | draft | CAP-TBD | TBD |
| [BC-6.11.036](ss-06/BC-6.11.036.md) | multi-repo-health: Story-repo cross-check warns when stories reference undetected repo or repo lacks stories | draft | CAP-TBD | TBD |
| [BC-6.11.037](ss-06/BC-6.11.037.md) | policy-add: Identity — register new governance policy in .factory/policies.yaml with sequential ID | draft | CAP-TBD | TBD |
| [BC-6.11.038](ss-06/BC-6.11.038.md) | policy-add: Policy name must be snake_case and unique across registry | draft | CAP-TBD | TBD |
| [BC-6.11.039](ss-06/BC-6.11.039.md) | policy-add: Severity HIGH or MEDIUM only; HIGH violations block convergence | draft | CAP-TBD | TBD |
| [BC-6.11.040](ss-06/BC-6.11.040.md) | policy-add: Enforced_by + scope each must have ≥1 entry; custom policies must include verification_steps | draft | CAP-TBD | TBD |
| [BC-6.11.041](ss-06/BC-6.11.041.md) | policy-add: Output appends to policies.yaml; runs validate after; reports next steps | draft | CAP-TBD | TBD |
| [BC-6.11.042](ss-06/BC-6.11.042.md) | policy-add: Prerequisite — policies.yaml must exist; otherwise run policy-registry init first | draft | CAP-TBD | TBD |
| [BC-6.11.043](ss-06/BC-6.11.043.md) | policy-registry: Identity — view/validate/manage governance policy registry | draft | CAP-TBD | TBD |
| [BC-6.11.044](ss-06/BC-6.11.044.md) | policy-registry: Init copies template + populates 9 baseline governance policies | draft | CAP-TBD | TBD |
| [BC-6.11.045](ss-06/BC-6.11.045.md) | policy-registry: Validate checks ID/name uniqueness, snake_case, severity ∈ {HIGH,MEDIUM}, lint_hook exists+executable, scope ∈ allowed types | draft | CAP-TBD | TBD |
| [BC-6.11.046](ss-06/BC-6.11.046.md) | policy-registry: List shows summary table with #/Policy/Severity/Enforced By/Lint Hook | draft | CAP-TBD | TBD |
| [BC-6.11.047](ss-06/BC-6.11.047.md) | policy-registry: Adversarial review auto-loads policies.yaml as rubric | draft | CAP-TBD | TBD |
| [BC-6.11.048](ss-06/BC-6.11.048.md) | quick-dev-routing: identity & qualification gate | draft | CAP-TBD | TBD |
| [BC-6.11.049](ss-06/BC-6.11.049.md) | quick-dev-routing: multi-goal detection precedes routing | draft | CAP-TBD | TBD |
| [BC-6.11.050](ss-06/BC-6.11.050.md) | quick-dev-routing: compressed pipeline preserves regression + adversary + human merge | draft | CAP-TBD | TBD |
| [BC-6.11.051](ss-06/BC-6.11.051.md) | quick-dev-routing: writes routing-decision.md and falls back on regression failure | draft | CAP-TBD | TBD |
| [BC-6.11.052](ss-06/BC-6.11.052.md) | worktree-manage: identity & 3 commands (create/list/cleanup) | draft | CAP-TBD | TBD |
| [BC-6.11.053](ss-06/BC-6.11.053.md) | worktree-manage: create produces `.worktrees/STORY-NNN/` on `feature/STORY-NNN-<desc>` | draft | CAP-TBD | TBD |
| [BC-6.11.054](ss-06/BC-6.11.054.md) | worktree-manage: cleanup refuses dirty + warns unmerged | draft | CAP-TBD | TBD |
| [BC-6.12.001](ss-06/BC-6.12.001.md) | feature-mode-scoping-rules: Identity is reference doc consumed by F1-F7 phase skills | draft | CAP-TBD | TBD |
| [BC-6.12.002](ss-06/BC-6.12.002.md) | feature-mode-scoping-rules: Regression scope is the FULL test suite, never scoped | draft | CAP-TBD | TBD |
| [BC-6.12.003](ss-06/BC-6.12.003.md) | feature-mode-scoping-rules: Adversarial review covers NEW + MODIFIED + DEPENDENT, never previous review reports | draft | CAP-TBD | TBD |
| [BC-6.12.004](ss-06/BC-6.12.004.md) | feature-mode-scoping-rules: Scope is immutable after F1 (Rule 6) | draft | CAP-TBD | TBD |
| [BC-6.13.001](ss-06/BC-6.13.001.md) | analytics-integration: skill identity contract (optional, no-op when not configured) | draft | TBD | TBD |
| [BC-6.13.002](ss-06/BC-6.13.002.md) | analytics-integration: feature health classification thresholds | draft | TBD | TBD |
| [BC-6.13.003](ss-06/BC-6.13.003.md) | analytics-integration: error severity classification | draft | TBD | TBD |
| [BC-6.13.004](ss-06/BC-6.13.004.md) | analytics-integration: digest output path and quality gate | draft | TBD | TBD |
| [BC-6.13.005](ss-06/BC-6.13.005.md) | claude-telemetry: skill identity (manage 5 OTEL_* env vars) | draft | TBD | TBD |
| [BC-6.13.006](ss-06/BC-6.13.006.md) | claude-telemetry: 3 modes (on/off/status) | draft | TBD | TBD |
| [BC-6.13.007](ss-06/BC-6.13.007.md) | claude-telemetry: prunes legacy temporality key | draft | TBD | TBD |
| [BC-6.13.008](ss-06/BC-6.13.008.md) | claude-telemetry: prominent restart reminder after `on` | draft | TBD | TBD |
| [BC-6.13.009](ss-06/BC-6.13.009.md) | onboard-observability: Identity — registers project + writes Claude OTel env vars; idempotent | draft | CAP-TBD | TBD |
| [BC-6.13.010](ss-06/BC-6.13.010.md) | onboard-observability: Required announce-at-start verbatim message | draft | CAP-TBD | TBD |
| [BC-6.13.011](ss-06/BC-6.13.011.md) | onboard-observability: Aborts if no .factory/ ancestor or factory-obs binary missing | draft | CAP-TBD | TBD |
| [BC-6.13.012](ss-06/BC-6.13.012.md) | onboard-observability: Writes exactly 5 OTEL_* env vars; preserves all other keys | draft | CAP-TBD | TBD |
| [BC-6.13.013](ss-06/BC-6.13.013.md) | onboard-observability: Idempotency — register dedupes on absolute path; jq merge overwrites only the 5 keys | draft | CAP-TBD | TBD |
| [BC-6.13.014](ss-06/BC-6.13.014.md) | onboard-observability: Non-goals — does not start/stop Docker stack, does not unregister, does not run cloud-only | draft | CAP-TBD | TBD |
| [BC-6.14.001](ss-06/BC-6.14.001.md) | artifact-detection: skill identity contract (universal pipeline front-end) | draft | TBD | TBD |
| [BC-6.14.002](ss-06/BC-6.14.002.md) | artifact-detection: 5-tier readiness classification (L0-L4) | draft | TBD | TBD |
| [BC-6.14.003](ss-06/BC-6.14.003.md) | artifact-detection: format detection flags FR-NNN legacy for migration | draft | TBD | TBD |
| [BC-6.14.004](ss-06/BC-6.14.004.md) | artifact-detection: writes 3 routing artifacts | draft | TBD | TBD |
| [BC-6.14.005](ss-06/BC-6.14.005.md) | artifact-detection: failure modes (no .factory/, corruption, legacy) | draft | TBD | TBD |
| [BC-6.14.006](ss-06/BC-6.14.006.md) | formal-verify: Identity — Phase 6 quality gate runs Kani + cargo-fuzz + cargo-mutants + semgrep | draft | CAP-TBD | TBD |
| [BC-6.14.007](ss-06/BC-6.14.007.md) | formal-verify: Iron Law — every VP needs passing proof + saturated fuzz + meeting kill rate | draft | CAP-TBD | TBD |
| [BC-6.14.008](ss-06/BC-6.14.008.md) | formal-verify: Fuzz saturation requires ≥5 minutes per target with stable coverage | draft | CAP-TBD | TBD |
| [BC-6.14.009](ss-06/BC-6.14.009.md) | formal-verify: Mutation kill rate target is ≥90% | draft | CAP-TBD | TBD |
| [BC-6.14.010](ss-06/BC-6.14.010.md) | formal-verify: Security scan clean = zero CRITICAL and zero HIGH | draft | CAP-TBD | TBD |
| [BC-6.14.011](ss-06/BC-6.14.011.md) | formal-verify: Output is formal-verification-report.md at .factory/cycles/<current>/ | draft | CAP-TBD | TBD |
| [BC-6.14.012](ss-06/BC-6.14.012.md) | formal-verify: Missing tools must be reported, never silently skipped | draft | CAP-TBD | TBD |
| [BC-6.14.013](ss-06/BC-6.14.013.md) | perf-check: Identity — bench regression + resource profiling + budget compliance | draft | CAP-TBD | TBD |
| [BC-6.14.014](ss-06/BC-6.14.014.md) | perf-check: Regression threshold — flag > 10% vs baseline | draft | CAP-TBD | TBD |
| [BC-6.14.015](ss-06/BC-6.14.015.md) | perf-check: Default budgets — startup <100ms; binary <50MB; debug build <60s; tests <120s | draft | CAP-TBD | TBD |
| [BC-6.14.016](ss-06/BC-6.14.016.md) | perf-check: Output is performance-report.md with PASS\|WARN\|FAIL gate | draft | CAP-TBD | TBD |
| [BC-6.14.017](ss-06/BC-6.14.017.md) | perf-check: No benchmarks → report and recommend creating them | draft | CAP-TBD | TBD |
| [BC-6.14.018](ss-06/BC-6.14.018.md) | register-artifact: identity & justification | draft | CAP-TBD | TBD |
| [BC-6.14.019](ss-06/BC-6.14.019.md) | register-artifact: type identification by path pattern (4-row table) | draft | CAP-TBD | TBD |
| [BC-6.14.020](ss-06/BC-6.14.020.md) | register-artifact: idempotent (refuses duplicate ID) | draft | CAP-TBD | TBD |
| [BC-6.14.021](ss-06/BC-6.14.021.md) | register-artifact: refuses to create INDEX file (separation of concerns) | draft | CAP-TBD | TBD |
| [BC-6.14.022](ss-06/BC-6.14.022.md) | register-artifact: batch mode aggregates results | draft | CAP-TBD | TBD |
| [BC-6.14.023](ss-06/BC-6.14.023.md) | systematic-debugging: identity + hard gate | draft | CAP-TBD | TBD |
| [BC-6.14.024](ss-06/BC-6.14.024.md) | systematic-debugging: 4-phase sequence (root cause → pattern → hypothesis → implementation) | draft | CAP-TBD | TBD |
| [BC-6.14.025](ss-06/BC-6.14.025.md) | systematic-debugging: Phase 4.5 — 3+ failed fixes = STOP and escalate | draft | CAP-TBD | TBD |
| [BC-6.14.026](ss-06/BC-6.14.026.md) | systematic-debugging: 8-row red-flag rationalization table | draft | CAP-TBD | TBD |
| [BC-6.14.027](ss-06/BC-6.14.027.md) | systematic-debugging: BC-aware mode + status-protocol reporting | draft | CAP-TBD | TBD |
| [BC-6.14.028](ss-06/BC-6.14.028.md) | track-debt: identity & three commands (add/list/resolve) | draft | CAP-TBD | TBD |
| [BC-6.14.029](ss-06/BC-6.14.029.md) | track-debt: add assigns next TD-NNN with full metadata | draft | CAP-TBD | TBD |
| [BC-6.14.030](ss-06/BC-6.14.030.md) | track-debt: register format (Active vs Resolved sections) | draft | CAP-TBD | TBD |
| [BC-6.14.031](ss-06/BC-6.14.031.md) | track-debt: when-to-add catalogue (6 sources) | draft | CAP-TBD | TBD |
| [BC-6.15.001](ss-06/BC-6.15.001.md) | brainstorming: skill identity + hard gate | draft | TBD | TBD |
| [BC-6.15.002](ss-06/BC-6.15.002.md) | brainstorming: 6 named techniques and selection logic | draft | TBD | TBD |
| [BC-6.15.003](ss-06/BC-6.15.003.md) | brainstorming: every idea goes through process even when "obvious" | draft | TBD | TBD |
| [BC-6.15.004](ss-06/BC-6.15.004.md) | brainstorming: report output and quality gate | draft | TBD | TBD |
| [BC-6.15.005](ss-06/BC-6.15.005.md) | writing-skills: identity & TDD-for-skills mapping | draft | CAP-TBD | TBD |
| [BC-6.15.006](ss-06/BC-6.15.006.md) | writing-skills: hard gate (NO SKILL WITHOUT FAILING TEST FIRST) | draft | CAP-TBD | TBD |
| [BC-6.15.007](ss-06/BC-6.15.007.md) | writing-skills: when-to-create vs when-not catalogues | draft | CAP-TBD | TBD |
| [BC-6.15.008](ss-06/BC-6.15.008.md) | writing-skills: SKILL.md structure with 6 mandatory sections | draft | CAP-TBD | TBD |
| [BC-6.15.009](ss-06/BC-6.15.009.md) | writing-skills: CSO description rules (Use when… , no workflow summary, <500 chars) | draft | CAP-TBD | TBD |
| [BC-6.15.010](ss-06/BC-6.15.010.md) | writing-skills: red-green-refactor cycle for skills + bulletproofing table | draft | CAP-TBD | TBD |
| [BC-6.15.011](ss-06/BC-6.15.011.md) | writing-skills: vsdd-factory conventions + checklist | draft | CAP-TBD | TBD |
| [BC-6.16.001](ss-06/BC-6.16.001.md) | mode-decision-guide: Identity — reference doc for orchestrator mode detection | draft | CAP-TBD | TBD |
| [BC-6.16.002](ss-06/BC-6.16.002.md) | mode-decision-guide: Feature Mode threshold — <30% files changed AND <50% components AND ≤2 cascade levels | draft | CAP-TBD | TBD |
| [BC-6.16.003](ss-06/BC-6.16.003.md) | mode-decision-guide: Greenfield switchover — ≥30% files OR ≥50% components OR breaking interfaces OR ≥3 cascade levels | draft | CAP-TBD | TBD |
| [BC-6.16.004](ss-06/BC-6.16.004.md) | mode-decision-guide: Bug fix minimal route — F1 → F4 → F5 → F7, skip F2/F3 | draft | CAP-TBD | TBD |
| [BC-6.16.005](ss-06/BC-6.16.005.md) | mode-decision-guide: Human override always wins over auto-detection | draft | CAP-TBD | TBD |
| [BC-6.16.006](ss-06/BC-6.16.006.md) | next-step: Identity — read STATE.md and propose next workflow step, do not execute | draft | CAP-TBD | TBD |
| [BC-6.16.007](ss-06/BC-6.16.007.md) | next-step: STATE.md missing → directs user to factory-health and stops | draft | CAP-TBD | TBD |
| [BC-6.16.008](ss-06/BC-6.16.008.md) | next-step: Does not execute — proposal only | draft | CAP-TBD | TBD |
| [BC-6.16.009](ss-06/BC-6.16.009.md) | next-step: Uses lobster-parse to enumerate workflow steps with dependencies | draft | CAP-TBD | TBD |
| [BC-6.16.010](ss-06/BC-6.16.010.md) | phase-0-codebase-ingestion: Identity — Phase 0 entry point delegating to brownfield-ingest sub-workflow | draft | CAP-TBD | TBD |
| [BC-6.16.011](ss-06/BC-6.16.011.md) | phase-0-codebase-ingestion: Gate — context doc, criticality, BCs (origin: recovered), coverage PASS, drift clean, human approval | draft | CAP-TBD | TBD |
| [BC-6.16.012](ss-06/BC-6.16.012.md) | phase-0-codebase-ingestion: Direct work skill is /vsdd-factory:brownfield-ingest <path> | draft | CAP-TBD | TBD |
| [BC-6.16.013](ss-06/BC-6.16.013.md) | phase-1-prd-revision: Identity — PO revises PRD per architect feasibility report; max 3 iterations | draft | CAP-TBD | TBD |
| [BC-6.16.014](ss-06/BC-6.16.014.md) | phase-1-prd-revision: Skip when feasibility report says "validated — no issues" | draft | CAP-TBD | TBD |
| [BC-6.16.015](ss-06/BC-6.16.015.md) | phase-1-prd-revision: 3-round deadlock escalates to human with both positions | draft | CAP-TBD | TBD |
| [BC-6.16.016](ss-06/BC-6.16.016.md) | phase-1-prd-revision: Quality Gate — every concern addressed or contested with rationale | draft | CAP-TBD | TBD |
| [BC-6.16.017](ss-06/BC-6.16.017.md) | phase-1-spec-crystallization: Identity — Phase 1 entry point spanning brief → architecture | draft | CAP-TBD | TBD |
| [BC-6.16.018](ss-06/BC-6.16.018.md) | phase-1-spec-crystallization: Gate — IDs unique, VPs cover security boundaries, purity map complete, adversarial converged, human approves | draft | CAP-TBD | TBD |
| [BC-6.16.019](ss-06/BC-6.16.019.md) | phase-1-spec-crystallization: Sub-workflow is workflows/phases/phase-1-spec-crystallization.lobster | draft | CAP-TBD | TBD |
| [BC-6.16.020](ss-06/BC-6.16.020.md) | phase-1d-adversarial-spec-review: Identity — adversary reviews spec package with fresh context | draft | CAP-TBD | TBD |
| [BC-6.16.021](ss-06/BC-6.16.021.md) | phase-1d-adversarial-spec-review: Adversary reviews 7 categories — ambiguity, missing edges, implicit assumptions, contradictions, testable-vs-provable, purity, tool mismatch | draft | CAP-TBD | TBD |
| [BC-6.16.022](ss-06/BC-6.16.022.md) | phase-1d-adversarial-spec-review: Findings triaged C/H/M/L; cross-doc sync check before re-review | draft | CAP-TBD | TBD |
| [BC-6.16.023](ss-06/BC-6.16.023.md) | phase-1d-adversarial-spec-review: Convergence — adversary reports "CONVERGENCE REACHED — findings are cosmetic only" | draft | CAP-TBD | TBD |
| [BC-6.16.024](ss-06/BC-6.16.024.md) | phase-1d-adversarial-spec-review: Quality Gate — different model family + fresh context every pass + all C/H resolved + convergence reported | draft | CAP-TBD | TBD |
| [BC-6.16.025](ss-06/BC-6.16.025.md) | phase-2-story-decomposition: Identity — Phase 2 entry point delegating to decompose-stories sub-workflow | draft | CAP-TBD | TBD |
| [BC-6.16.026](ss-06/BC-6.16.026.md) | phase-2-story-decomposition: Gate — every BC traces to ≥1 story, no placeholder ACs, no cycles, ≥1 holdout/wave, drift clean, human approval | draft | CAP-TBD | TBD |
| [BC-6.16.027](ss-06/BC-6.16.027.md) | phase-2-story-decomposition: Direct command is /vsdd-factory:decompose-stories | draft | CAP-TBD | TBD |
| [BC-6.16.028](ss-06/BC-6.16.028.md) | phase-3-tdd-implementation: Identity — per-story TDD delivery via deliver-story sub-workflow | draft | CAP-TBD | TBD |
| [BC-6.16.029](ss-06/BC-6.16.029.md) | phase-3-tdd-implementation: Gate — Red Gate passed, all tests pass, demos cover ACs, PR merged, worktree cleaned, drift clean | draft | CAP-TBD | TBD |
| [BC-6.16.030](ss-06/BC-6.16.030.md) | phase-3-tdd-implementation: Prerequisites — Phase 2 complete, story status `ready`, all dependency stories completed | draft | CAP-TBD | TBD |
| [BC-6.16.031](ss-06/BC-6.16.031.md) | phase-5-adversarial-refinement: Identity — multi-model adversarial loop until novelty=0 | draft | CAP-TBD | TBD |
| [BC-6.16.032](ss-06/BC-6.16.032.md) | phase-5-adversarial-refinement: Gate — novelty=0, all findings addressed/accepted, ≥3 clean passes minimum | draft | CAP-TBD | TBD |
| [BC-6.16.033](ss-06/BC-6.16.033.md) | phase-5-adversarial-refinement: Direct command is /vsdd-factory:adversarial-review implementation | draft | CAP-TBD | TBD |
| [BC-6.16.034](ss-06/BC-6.16.034.md) | phase-6-formal-hardening: Identity — Phase 6 entry point applying 4 verification techniques | draft | CAP-TBD | TBD |
| [BC-6.16.035](ss-06/BC-6.16.035.md) | phase-6-formal-hardening: Gate — all proofs pass, fuzz 5min/target zero crashes, mutation >90%, zero CRIT/HIGH semgrep, cargo audit clean, purity intact | draft | CAP-TBD | TBD |
| [BC-6.16.036](ss-06/BC-6.16.036.md) | phase-6-formal-hardening: Prerequisites — cargo-kani, cargo-fuzz, cargo-mutants, semgrep installed | draft | CAP-TBD | TBD |
| [BC-6.16.037](ss-06/BC-6.16.037.md) | phase-7-convergence: Identity — 7-dimensional convergence assessment | draft | CAP-TBD | TBD |
| [BC-6.16.038](ss-06/BC-6.16.038.md) | phase-7-convergence: Gate — all 7 dimensions CONVERGED, traceability matrix, demo verified by visual-reviewer, drift clean, human approval | draft | CAP-TBD | TBD |
| [BC-6.16.039](ss-06/BC-6.16.039.md) | phase-7-convergence: Outcome — CONVERGED → release; NOT CONVERGED → loop back to Phase 3 | draft | CAP-TBD | TBD |
| [BC-6.16.040](ss-06/BC-6.16.040.md) | run-phase: identity & resolution rules | draft | CAP-TBD | TBD |
| [BC-6.16.041](ss-06/BC-6.16.041.md) | run-phase: validates workflow before execution | draft | CAP-TBD | TBD |
| [BC-6.16.042](ss-06/BC-6.16.042.md) | run-phase: topological execution honors depends_on | draft | CAP-TBD | TBD |
| [BC-6.16.043](ss-06/BC-6.16.043.md) | run-phase: STATE.md update + final summary | draft | CAP-TBD | TBD |
| [BC-6.17.001](ss-06/BC-6.17.001.md) | phase-f1-delta-analysis: Identity — analyzes feature request against existing artifacts to determine impact boundary | draft | CAP-TBD | TBD |
| [BC-6.17.002](ss-06/BC-6.17.002.md) | phase-f1-delta-analysis: Components classified NEW/MODIFIED/DEPENDENT by architect | draft | CAP-TBD | TBD |
| [BC-6.17.003](ss-06/BC-6.17.003.md) | phase-f1-delta-analysis: Intent classification — feature, enhancement, bug-fix maps to F1-F7 vs bug-fix route | draft | CAP-TBD | TBD |
| [BC-6.17.004](ss-06/BC-6.17.004.md) | phase-f1-delta-analysis: Trivial scope — single module + no new BCs + no arch change + no new deps + LOW risk → quick-dev routing | draft | CAP-TBD | TBD |
| [BC-6.17.005](ss-06/BC-6.17.005.md) | phase-f1-delta-analysis: Severity (bug-fix only) — CRITICAL triggers expedited flow with skipped baseline/proofs | draft | CAP-TBD | TBD |
| [BC-6.17.006](ss-06/BC-6.17.006.md) | phase-f1-delta-analysis: Output is delta-analysis.md + affected-files.txt (+ affected-repos.txt for multi-repo) | draft | CAP-TBD | TBD |
| [BC-6.17.007](ss-06/BC-6.17.007.md) | phase-f1-delta-analysis: Quality Gate — feature_type, intent, scope, severity (if bug-fix), BC-S.SS.NNN refs, multi-repo, human-approved | draft | CAP-TBD | TBD |
| [BC-6.17.008](ss-06/BC-6.17.008.md) | phase-f2-spec-evolution: Identity — incremental spec updates (PRD + arch + VPs), delta only | draft | CAP-TBD | TBD |
| [BC-6.17.009](ss-06/BC-6.17.009.md) | phase-f2-spec-evolution: PRD delta appends new BCs continuing BC-S.SS.NNN sequence; modified BCs marked UPDATED with previous version inline | draft | CAP-TBD | TBD |
| [BC-6.17.010](ss-06/BC-6.17.010.md) | phase-f2-spec-evolution: UX delta + accessibility review run only when feature_type ∈ ['ui', 'full-stack'] | draft | CAP-TBD | TBD |
| [BC-6.17.011](ss-06/BC-6.17.011.md) | phase-f2-spec-evolution: Spec version bump per semver — MAJOR/MINOR/PATCH | draft | CAP-TBD | TBD |
| [BC-6.17.012](ss-06/BC-6.17.012.md) | phase-f2-spec-evolution: Adversary reviews ONLY the delta (PRD + arch + VP + UX), not unchanged sections | draft | CAP-TBD | TBD |
| [BC-6.17.013](ss-06/BC-6.17.013.md) | phase-f2-spec-evolution: Quality Gate — BC-S.SS.NNN format, append-only, acyclic deps, version bumped, changelog written, adversary cosmetic only, human approved | draft | CAP-TBD | TBD |
| [BC-6.17.014](ss-06/BC-6.17.014.md) | phase-f2-spec-evolution: Failure — missing F1 output halts F2; CRITICAL after 3 rounds escalates to human | draft | CAP-TBD | TBD |
| [BC-6.17.015](ss-06/BC-6.17.015.md) | phase-f3-incremental-stories: Identity — adds new stories integrated into existing dependency graph without cycles | draft | CAP-TBD | TBD |
| [BC-6.17.016](ss-06/BC-6.17.016.md) | phase-f3-incremental-stories: Story IDs continue existing sequence; per-file STORY-NNN.md, not monolithic | draft | CAP-TBD | TBD |
| [BC-6.17.017](ss-06/BC-6.17.017.md) | phase-f3-incremental-stories: Cycle detection via Kahn's algorithm topological sort | draft | CAP-TBD | TBD |
| [BC-6.17.018](ss-06/BC-6.17.018.md) | phase-f3-incremental-stories: DTU clones stories placed in Wave 1; gene transfusion stories flagged | draft | CAP-TBD | TBD |
| [BC-6.17.019](ss-06/BC-6.17.019.md) | phase-f3-incremental-stories: Quality Gate — IDs continue, per-file, BC-S.SS.NNN, testable AC, VP-NNN, no cycles, append-only, wave schedule + holdouts, conflicts resolved, human approved | draft | CAP-TBD | TBD |
| [BC-6.17.020](ss-06/BC-6.17.020.md) | phase-f4-delta-implementation: Identity — TDD scoped to new stories with full regression as safety net | draft | CAP-TBD | TBD |
| [BC-6.17.021](ss-06/BC-6.17.021.md) | phase-f4-delta-implementation: Establish regression baseline before any new code; if any fail, STOP | draft | CAP-TBD | TBD |
| [BC-6.17.022](ss-06/BC-6.17.022.md) | phase-f4-delta-implementation: Per-story delivery uses code-delivery.lobster sub-workflow with 11 stages | draft | CAP-TBD | TBD |
| [BC-6.17.023](ss-06/BC-6.17.023.md) | phase-f4-delta-implementation: Wave Integration Gate — full tests + adversary + security + holdout + a11y + demo + fix loop max 10 | draft | CAP-TBD | TBD |
| [BC-6.17.024](ss-06/BC-6.17.024.md) | phase-f4-delta-implementation: Regression failure — fix the implementation, not the test | draft | CAP-TBD | TBD |
| [BC-6.17.025](ss-06/BC-6.17.025.md) | phase-f4-delta-implementation: Quality Gate — regression baseline + Two-Step Red Gate + full regression pass + reviewer + security if CRIT/HIGH + max 10 + wave gate + E2E for UI + no out-of-scope edits + summary | draft | CAP-TBD | TBD |
| [BC-6.17.026](ss-06/BC-6.17.026.md) | phase-f4-delta-implementation: No human gate — automated quality gate | draft | CAP-TBD | TBD |
| [BC-6.17.027](ss-06/BC-6.17.027.md) | phase-f5-scoped-adversarial: Identity — adversary reviews only delta files, fresh context, different model family | draft | CAP-TBD | TBD |
| [BC-6.17.028](ss-06/BC-6.17.028.md) | phase-f5-scoped-adversarial: Review package excludes prior reviews, implementation rationale, semport, red-gate logs | draft | CAP-TBD | TBD |
| [BC-6.17.029](ss-06/BC-6.17.029.md) | phase-f5-scoped-adversarial: 5 review categories — spec fidelity, regression risk, convention, security, test quality | draft | CAP-TBD | TBD |
| [BC-6.17.030](ss-06/BC-6.17.030.md) | phase-f5-scoped-adversarial: Severity scale CRITICAL/HIGH/MEDIUM/LOW/COSMETIC; convergence at novelty < 0.15 AND no CRIT/HIGH | draft | CAP-TBD | TBD |
| [BC-6.17.031](ss-06/BC-6.17.031.md) | phase-f5-scoped-adversarial: Secondary review (Gemini/review-tier) optional for security-critical or large delta | draft | CAP-TBD | TBD |
| [BC-6.17.032](ss-06/BC-6.17.032.md) | phase-f5-scoped-adversarial: Output convergence-summary.md; F5 fixes through code-delivery.lobster as FIX-F5-NNN | draft | CAP-TBD | TBD |
| [BC-6.17.033](ss-06/BC-6.17.033.md) | phase-f5-scoped-adversarial: Quality Gate — delta scope only, fresh context, different model family, all CRIT/HIGH resolved, novelty < 0.15, regression still passes | draft | CAP-TBD | TBD |
| [BC-6.17.034](ss-06/BC-6.17.034.md) | phase-f6-targeted-hardening: Identity — Kani+fuzz+mutation scoped to delta; regression+security on full tree | draft | CAP-TBD | TBD |
| [BC-6.17.035](ss-06/BC-6.17.035.md) | phase-f6-targeted-hardening: Hardening scope per-tool varies — Kani/fuzz/mutation/Semgrep delta; regression+audit full tree | draft | CAP-TBD | TBD |
| [BC-6.17.036](ss-06/BC-6.17.036.md) | phase-f6-targeted-hardening: Mutation kill rate ≥90% on changed files (≥95% for security-critical) | draft | CAP-TBD | TBD |
| [BC-6.17.037](ss-06/BC-6.17.037.md) | phase-f6-targeted-hardening: Information asymmetry wall — formal-verifier cannot see F5 adversarial findings | draft | CAP-TBD | TBD |
| [BC-6.17.038](ss-06/BC-6.17.038.md) | phase-f6-targeted-hardening: Quality Gate — proofs pass, fuzz clean, mutation 90% (95% critical), no CRIT/HIGH, regression passes, DTU adversarial if external svc, a11y if UI, FIX-F6-NNN via code-delivery, partial re-verification | draft | CAP-TBD | TBD |
| [BC-6.17.039](ss-06/BC-6.17.039.md) | phase-f7-delta-convergence: Identity — 5-dimensional convergence on delta + full regression validation; final human gate | draft | CAP-TBD | TBD |
| [BC-6.17.040](ss-06/BC-6.17.040.md) | phase-f7-delta-convergence: 5 dimensions — Spec novelty<0.15, Test mutation≥90%, Impl verification rate<60%, Verification all-pass, Holdout≥0.85 | draft | CAP-TBD | TBD |
| [BC-6.17.041](ss-06/BC-6.17.041.md) | phase-f7-delta-convergence: Regression validation is binary pass/fail, not "convergence" | draft | CAP-TBD | TBD |
| [BC-6.17.042](ss-06/BC-6.17.042.md) | phase-f7-delta-convergence: Cost-benefit — flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost > P(finding) * Value_avg / 1.5 | draft | CAP-TBD | TBD |
| [BC-6.17.043](ss-06/BC-6.17.043.md) | phase-f7-delta-convergence: Traceability chain extended (append, not replace) with new BC→VP→test→src→ADV→KANI links | draft | CAP-TBD | TBD |
| [BC-6.17.044](ss-06/BC-6.17.044.md) | phase-f7-delta-convergence: Final human authorization gate; failure routes to specific phase | draft | CAP-TBD | TBD |
| [BC-6.17.045](ss-06/BC-6.17.045.md) | phase-f7-delta-convergence: Quality Gate — all 5 dims pass, regression passes, traceability extended, cost-benefit included, max 5 cycles, FIX-F7-NNN, human authorized | draft | CAP-TBD | TBD |
| [BC-6.17.046](ss-06/BC-6.17.046.md) | phase-f7-delta-convergence: Max 5 convergence cycles before cost-benefit escalation | draft | CAP-TBD | TBD |
| [BC-6.18.001](ss-06/BC-6.18.001.md) | pr-create: identity & invocation contract | draft | CAP-TBD | TBD |
| [BC-6.18.002](ss-06/BC-6.18.002.md) | pr-create: gathers story context before generating body | draft | CAP-TBD | TBD |
| [BC-6.18.003](ss-06/BC-6.18.003.md) | pr-create: PR body must follow templated structure with mermaid + traceability table | draft | CAP-TBD | TBD |
| [BC-6.18.004](ss-06/BC-6.18.004.md) | pr-create: PR creation targets develop with feat-prefixed title | draft | CAP-TBD | TBD |
| [BC-6.18.005](ss-06/BC-6.18.005.md) | pr-create: post-creation report includes URL + next steps | draft | CAP-TBD | TBD |
| [BC-6.18.006](ss-06/BC-6.18.006.md) | pr-review-triage: identity & dispatch role | draft | CAP-TBD | TBD |
| [BC-6.18.007](ss-06/BC-6.18.007.md) | pr-review-triage: classification table is complete and exhaustive | draft | CAP-TBD | TBD |
| [BC-6.18.008](ss-06/BC-6.18.008.md) | pr-review-triage: size-blocking finding STOPS pr-manager | draft | CAP-TBD | TBD |
| [BC-6.18.009](ss-06/BC-6.18.009.md) | pr-review-triage: ten-cycle escalation cap | draft | CAP-TBD | TBD |
| [BC-6.18.010](ss-06/BC-6.18.010.md) | pr-review-triage: writes review-findings.md with cycle row + triage table | draft | CAP-TBD | TBD |
| [BC-6.18.011](ss-06/BC-6.18.011.md) | release: identity, modes, factory worktree pre-flight | draft | CAP-TBD | TBD |
| [BC-6.18.012](ss-06/BC-6.18.012.md) | release: announces protocol verbatim | draft | CAP-TBD | TBD |
| [BC-6.18.013](ss-06/BC-6.18.013.md) | release: bootstrap detects markers across five project types | draft | CAP-TBD | TBD |
| [BC-6.18.014](ss-06/BC-6.18.014.md) | release: version bump determined from story types when no explicit version | draft | CAP-TBD | TBD |
| [BC-6.18.015](ss-06/BC-6.18.015.md) | release: quality-gate modes (standard/vsdd-partial/vsdd-full) | draft | CAP-TBD | TBD |
| [BC-6.18.016](ss-06/BC-6.18.016.md) | release: per-format version-bump dispatch (json/toml/yaml/regex) | draft | CAP-TBD | TBD |
| [BC-6.18.017](ss-06/BC-6.18.017.md) | release: tag, push (with confirm), CI watch, gh-release verify | draft | CAP-TBD | TBD |
| [BC-6.18.018](ss-06/BC-6.18.018.md) | release: dry-run produces complete plan with no side effects | draft | CAP-TBD | TBD |
| [BC-6.18.019](ss-06/BC-6.18.019.md) | release: error-handling catalog | draft | CAP-TBD | TBD |
| [BC-6.19.001](ss-06/BC-6.19.001.md) | repo-initialization: identity & delegation reference | draft | CAP-TBD | TBD |
| [BC-6.19.002](ss-06/BC-6.19.002.md) | repo-initialization: workspace-isolation guard refuses dark-factory cwd | draft | CAP-TBD | TBD |
| [BC-6.19.003](ss-06/BC-6.19.003.md) | repo-initialization: develop branch is the protected default | draft | CAP-TBD | TBD |
| [BC-6.19.004](ss-06/BC-6.19.004.md) | repo-initialization: factory-artifacts orphan worktree pre-check (NOT dark-factory) | draft | CAP-TBD | TBD |
| [BC-6.19.005](ss-06/BC-6.19.005.md) | repo-initialization: architect signal table for multi-vs-single-repo | draft | CAP-TBD | TBD |
| [BC-6.19.006](ss-06/BC-6.19.006.md) | repo-initialization: multi-repo creates .factory-project/ + project.yaml + per-service repos | draft | CAP-TBD | TBD |
| [BC-6.19.007](ss-06/BC-6.19.007.md) | repo-initialization: dx-engineer environment setup (DF-027) | draft | CAP-TBD | TBD |
| [BC-6.19.008](ss-06/BC-6.19.008.md) | repo-initialization: CI/CD setup is deferred to post-architecture | draft | CAP-TBD | TBD |
| [BC-6.19.009](ss-06/BC-6.19.009.md) | repo-initialization: outputs | draft | CAP-TBD | TBD |
| [BC-6.19.010](ss-06/BC-6.19.010.md) | scaffold-claude-md: identity & overwrite confirmation | draft | CAP-TBD | TBD |
| [BC-6.19.011](ss-06/BC-6.19.011.md) | scaffold-claude-md: four detectors run in priority order | draft | CAP-TBD | TBD |
| [BC-6.19.012](ss-06/BC-6.19.012.md) | scaffold-claude-md: CLAUDE.md does NOT duplicate plugin methodology | draft | CAP-TBD | TBD |
| [BC-6.19.013](ss-06/BC-6.19.013.md) | scaffold-claude-md: present-confirm-write loop | draft | CAP-TBD | TBD |
| [BC-6.19.014](ss-06/BC-6.19.014.md) | sdk-generation: identity & trigger triad | draft | CAP-TBD | TBD |
| [BC-6.19.015](ss-06/BC-6.19.015.md) | sdk-generation: contract validation gates generation | draft | CAP-TBD | TBD |
| [BC-6.19.016](ss-06/BC-6.19.016.md) | sdk-generation: language idiom enforcement (TS async, Py snake_case, Go errors) | draft | CAP-TBD | TBD |
| [BC-6.19.017](ss-06/BC-6.19.017.md) | sdk-generation: tool-format dispatch (OpenAPI/protobuf/GraphQL) | draft | CAP-TBD | TBD |
| [BC-6.19.018](ss-06/BC-6.19.018.md) | sdk-generation: contract-test integration (Pact / Specmatic / Schemathesis / openapi-diff) | draft | CAP-TBD | TBD |
| [BC-6.19.019](ss-06/BC-6.19.019.md) | sdk-generation: contract evolution (semver + breaking detection) | draft | CAP-TBD | TBD |
| [BC-6.19.020](ss-06/BC-6.19.020.md) | sdk-generation: outputs and quality gate | draft | CAP-TBD | TBD |
| [BC-6.19.021](ss-06/BC-6.19.021.md) | setup-env: identity & frontmatter | draft | CAP-TBD | TBD |
| [BC-6.19.022](ss-06/BC-6.19.022.md) | setup-env: tool-check tables (8 required + 8 optional) | draft | CAP-TBD | TBD |
| [BC-6.19.023](ss-06/BC-6.19.023.md) | setup-env: MCP env-var prefix check + git config (rerere on) | draft | CAP-TBD | TBD |
| [BC-6.19.024](ss-06/BC-6.19.024.md) | setup-env: factory-health invocation + final missing-tools list | draft | CAP-TBD | TBD |
| [BC-6.19.025](ss-06/BC-6.19.025.md) | toolchain-provisioning: identity & 4 trigger points | draft | CAP-TBD | TBD |
| [BC-6.19.026](ss-06/BC-6.19.026.md) | toolchain-provisioning: precedence rule (architect > verification > manifest > human) | draft | CAP-TBD | TBD |
| [BC-6.19.027](ss-06/BC-6.19.027.md) | toolchain-provisioning: language detection cascade | draft | CAP-TBD | TBD |
| [BC-6.19.028](ss-06/BC-6.19.028.md) | toolchain-provisioning: install-priority (lang-native → brew → system) + pkg-mgr per type | draft | CAP-TBD | TBD |
| [BC-6.19.029](ss-06/BC-6.19.029.md) | toolchain-provisioning: writes detailed toolchain-state.yaml | draft | CAP-TBD | TBD |
| [BC-6.19.030](ss-06/BC-6.19.030.md) | toolchain-provisioning: integration with formal-hardening + multi-repo + new-language | draft | CAP-TBD | TBD |
| [BC-6.19.031](ss-06/BC-6.19.031.md) | toolchain-provisioning: Storybook + Excalidraw MCP for UI products | draft | CAP-TBD | TBD |
| [BC-6.19.032](ss-06/BC-6.19.032.md) | toolchain-provisioning: quality-gate criteria | draft | CAP-TBD | TBD |
| [BC-6.20.001](ss-06/BC-6.20.001.md) | create-adr allocates next sequential ADR-NNN by scanning filesystem and ARCH-INDEX | draft | CAP-017 | S-6.01 |
| [BC-6.20.002](ss-06/BC-6.20.002.md) | create-adr refuses explicit --id override that already exists | draft | CAP-017 | S-6.01 |
| [BC-6.20.003](ss-06/BC-6.20.003.md) | create-adr blocks on filesystem-vs-ARCH-INDEX ID mismatch | draft | CAP-017 | S-6.01 |
| [BC-6.20.004](ss-06/BC-6.20.004.md) | create-adr writes frontmatter with status=proposed (always at creation) | draft | CAP-017 | S-6.01 |
| [BC-6.20.005](ss-06/BC-6.20.005.md) | create-adr validates subsystems_affected against ARCH-INDEX Subsystem Registry | draft | CAP-017 | S-6.01 |
| [BC-6.20.006](ss-06/BC-6.20.006.md) | create-adr validates --supersedes ADR-NNN exists before proceeding | draft | CAP-017 | S-6.01 |
| [BC-6.20.007](ss-06/BC-6.20.007.md) | create-adr bidirectionally patches old ADR's superseded_by on supersession | draft | CAP-017 | S-6.01 |
| [BC-6.20.008](ss-06/BC-6.20.008.md) | create-adr inserts ARCH-INDEX row in numeric order, pipe-aligned | draft | CAP-017 | S-6.01 |
| [BC-6.20.009](ss-06/BC-6.20.009.md) | create-adr scaffolds placeholder section bodies verbatim from template (no ghost-writing) | draft | CAP-017 | S-6.01 |
| [BC-6.20.010](ss-06/BC-6.20.010.md) | create-adr annotates Source/Origin section under --brownfield or implicit-brownfield | draft | CAP-017 | S-6.01 |
| [BC-6.20.011](ss-06/BC-6.20.011.md) | create-adr runs validate-template-compliance.sh as final gate, blocks on non-zero | draft | CAP-017 | S-6.01 |
| [BC-6.20.012](ss-06/BC-6.20.012.md) | create-adr is atomic — any partial-state failure rolls back all side-effects | draft | CAP-017 | S-6.01 |
| [BC-6.21.001](ss-06/BC-6.21.001.md) | wave-gate skill must run cargo mutants for every story with tdd_mode=facade in the wave | draft | CAP-016 | S-7.03 |
| [BC-6.21.002](ss-06/BC-6.21.002.md) | mutation kill rate floor is 80%; surviving mutants must be addressed via test, dead-code confirmation, or explicit waiver | draft | CAP-016 | S-7.03 |

### SS-07 — Hook Bash Layer (BC-7)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-7.01.001](ss-07/BC-7.01.001.md) | block-ai-attribution blocks git commit messages with AI attribution | draft | TBD | TBD |
| [BC-7.01.002](ss-07/BC-7.01.002.md) | capture-commit-activity (PostToolUse:Bash) emits `commit.made` on successful commits | draft | TBD | TBD |
| [BC-7.01.003](ss-07/BC-7.01.003.md) | regression-gate (PostToolUse) fails when bash command interrupted | draft | TBD | TBD |
| [BC-7.01.004](ss-07/BC-7.01.004.md) | protect-secrets (PreToolUse:Bash + PreToolUse:Read) blocks reads of dotenv / credentials | draft | TBD | TBD |
| [BC-7.01.005](ss-07/BC-7.01.005.md) | check-factory-commit warns when committing in `.factory/` without STATE.md update | draft | TBD | TBD |
| [BC-7.01.006](ss-07/BC-7.01.006.md) | validate-* family (24 validators on PostToolUse:Edit\ | draft | TBD | TBD |
| [BC-7.01.007](ss-07/BC-7.01.007.md) | track-agent-{start,stop} (PreToolUse:Agent / SubagentStop) records agent lifecycle | draft | TBD | TBD |
| [BC-7.02.001](ss-07/BC-7.02.001.md) | Hooks read JSON envelope from stdin and parse with jq | draft | TBD | TBD |
| [BC-7.02.002](ss-07/BC-7.02.002.md) | Hook exit code semantics: 0 = pass/allow, 2 = block, with stderr diagnostic | draft | TBD | TBD |
| [BC-7.02.003](ss-07/BC-7.02.003.md) | Hooks emit `hook.block` event on block via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` | draft | TBD | TBD |
| [BC-7.02.004](ss-07/BC-7.02.004.md) | Hook scoping uses `case "$FILE_PATH"` glob narrowing — early `exit 0` on irrelevant files | draft | TBD | TBD |
| [BC-7.02.005](ss-07/BC-7.02.005.md) | Hook latency budget is sub-100ms; deterministic; LLM-free | draft | TBD | TBD |
| [BC-7.02.006](ss-07/BC-7.02.006.md) | factory-dispatcher routing binds hooks via `[[hooks]]` entry: `name`, `event`, optional `tool` re... | draft | TBD | TBD |
| [BC-7.02.007](ss-07/BC-7.02.007.md) | Validator hooks at `tool = "Edit\ | draft | TBD | TBD |
| [BC-7.02.008](ss-07/BC-7.02.008.md) | Hook capability model: every legacy-routed hook declares `[hooks.capabilities.exec_subprocess]` w... | draft | TBD | TBD |
| [BC-7.02.009](ss-07/BC-7.02.009.md) | Native (non-legacy) hook plugins MUST link `vsdd-hook-sdk` and use the `#[hook]` macro (not curre... | draft | TBD | TBD |
| [BC-7.03.001](ss-07/BC-7.03.001.md) | block-ai-attribution: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.002](ss-07/BC-7.03.002.md) | block-ai-attribution: substring gate on `git commit` | draft | TBD | TBD |
| [BC-7.03.003](ss-07/BC-7.03.003.md) | block-ai-attribution: blocks Co-Authored-By with AI tool name | draft | TBD | TBD |
| [BC-7.03.004](ss-07/BC-7.03.004.md) | block-ai-attribution: blocks "Generated with Claude Code" / generated-by-AI / noreply email | draft | TBD | TBD |
| [BC-7.03.005](ss-07/BC-7.03.005.md) | block-ai-attribution: jq absence is graceful no-op | draft | TBD | TBD |
| [BC-7.03.006](ss-07/BC-7.03.006.md) | brownfield-discipline: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.007](ss-07/BC-7.03.007.md) | brownfield-discipline: blocks edits to `.reference/**` | draft | TBD | TBD |
| [BC-7.03.008](ss-07/BC-7.03.008.md) | brownfield-discipline: jq missing is hard error (rare among hooks) | draft | TBD | TBD |
| [BC-7.03.009](ss-07/BC-7.03.009.md) | capture-commit-activity: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.010](ss-07/BC-7.03.010.md) | capture-commit-activity: skips non-Bash, interrupted, or non-zero exits | draft | TBD | TBD |
| [BC-7.03.011](ss-07/BC-7.03.011.md) | capture-commit-activity: matches `git commit` as real subcommand and parses preamble | draft | TBD | TBD |
| [BC-7.03.012](ss-07/BC-7.03.012.md) | capture-commit-activity: detects `--amend` and emits `amended="true"` | draft | TBD | TBD |
| [BC-7.03.013](ss-07/BC-7.03.013.md) | capture-commit-activity: emits `commit.made` event via emit-event helper | draft | TBD | TBD |
| [BC-7.03.014](ss-07/BC-7.03.014.md) | capture-pr-activity: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.015](ss-07/BC-7.03.015.md) | capture-pr-activity: emits `pr.opened` from `gh pr create` stdout | draft | TBD | TBD |
| [BC-7.03.016](ss-07/BC-7.03.016.md) | capture-pr-activity: emits `pr.merged` and computes `open_to_merge_seconds` | draft | TBD | TBD |
| [BC-7.03.017](ss-07/BC-7.03.017.md) | capture-pr-activity: log directory resolution order | draft | TBD | TBD |
| [BC-7.03.018](ss-07/BC-7.03.018.md) | check-factory-commit: identity & registry binding (with semantic mismatch) | draft | TBD | TBD |
| [BC-7.03.019](ss-07/BC-7.03.019.md) | check-factory-commit: emits additionalContext when .factory commit lacks STATE.md | draft | TBD | TBD |
| [BC-7.03.020](ss-07/BC-7.03.020.md) | convergence-tracker: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.021](ss-07/BC-7.03.021.md) | convergence-tracker: enforces trajectory monotonicity (warn-only) | draft | TBD | TBD |
| [BC-7.03.022](ss-07/BC-7.03.022.md) | convergence-tracker: BLOCKS premature CONVERGENCE_REACHED with novelty>0.15 | draft | TBD | TBD |
| [BC-7.03.023](ss-07/BC-7.03.023.md) | convergence-tracker: BLOCKS CONVERGENCE_REACHED when CRIT or HIGH count > 0 | draft | TBD | TBD |
| [BC-7.03.024](ss-07/BC-7.03.024.md) | convergence-tracker: BLOCKS CONVERGENCE_REACHED with <3 consecutive clean passes | draft | TBD | TBD |
| [BC-7.03.025](ss-07/BC-7.03.025.md) | convergence-tracker: warns on zero findings on first pass | draft | TBD | TBD |
| [BC-7.03.026](ss-07/BC-7.03.026.md) | destructive-command-guard: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.027](ss-07/BC-7.03.027.md) | destructive-command-guard: blocks `rm -rf` on catastrophic roots (8 patterns) | draft | TBD | TBD |
| [BC-7.03.028](ss-07/BC-7.03.028.md) | destructive-command-guard: blocks `rm -rf` on protected paths | draft | TBD | TBD |
| [BC-7.03.029](ss-07/BC-7.03.029.md) | destructive-command-guard: blocks `rm` on source-of-truth files (8 SOT files) | draft | TBD | TBD |
| [BC-7.03.030](ss-07/BC-7.03.030.md) | destructive-command-guard: blocks clobbering redirects to SOT files | draft | TBD | TBD |
| [BC-7.03.031](ss-07/BC-7.03.031.md) | destructive-command-guard: blocks `find -delete\ | draft | TBD | TBD |
| [BC-7.03.032](ss-07/BC-7.03.032.md) | destructive-command-guard: blocks irreversible git operations (8 patterns) | draft | TBD | TBD |
| [BC-7.03.033](ss-07/BC-7.03.033.md) | destructive-command-guard: blocks `--no-verify` and `--no-gpg-sign` | draft | TBD | TBD |
| [BC-7.03.034](ss-07/BC-7.03.034.md) | destructive-command-guard: blocks `git rm` on living spec/story files | draft | TBD | TBD |
| [BC-7.03.035](ss-07/BC-7.03.035.md) | destructive-command-guard: blocks gh shared-state destructive ops (4 codes) | draft | TBD | TBD |
| [BC-7.03.036](ss-07/BC-7.03.036.md) | destructive-command-guard: blocks curl\ | draft | TBD | TBD |
| [BC-7.03.037](ss-07/BC-7.03.037.md) | destructive-command-guard: blocks recursive chmod/chown on protected paths | draft | TBD | TBD |
| [BC-7.03.038](ss-07/BC-7.03.038.md) | factory-branch-guard: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.039](ss-07/BC-7.03.039.md) | factory-branch-guard: blocks if .factory/ is not a worktree | draft | TBD | TBD |
| [BC-7.03.040](ss-07/BC-7.03.040.md) | factory-branch-guard: blocks if worktree on wrong branch | draft | TBD | TBD |
| [BC-7.03.041](ss-07/BC-7.03.041.md) | factory-branch-guard: skips paths outside .factory tree | draft | TBD | TBD |
| [BC-7.03.042](ss-07/BC-7.03.042.md) | handoff-validator: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.043](ss-07/BC-7.03.043.md) | handoff-validator: warns on empty subagent result | draft | TBD | TBD |
| [BC-7.03.044](ss-07/BC-7.03.044.md) | handoff-validator: warns on suspiciously short result (<40 chars) | draft | TBD | TBD |
| [BC-7.03.045](ss-07/BC-7.03.045.md) | pr-manager-completion-guard: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.046](ss-07/BC-7.03.046.md) | pr-manager-completion-guard: counts STEP_COMPLETE emissions; passes if ≥8 | draft | TBD | TBD |
| [BC-7.03.047](ss-07/BC-7.03.047.md) | pr-manager-completion-guard: BLOCKED status is legitimate early exit | draft | TBD | TBD |
| [BC-7.03.048](ss-07/BC-7.03.048.md) | pr-manager-completion-guard: blocks with step-specific continuation hint | draft | TBD | TBD |
| [BC-7.03.049](ss-07/BC-7.03.049.md) | protect-bc: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.050](ss-07/BC-7.03.050.md) | protect-bc: denies edits to green BCs at .factory/specs/behavioral-contracts/BC-*.md | draft | TBD | TBD |
| [BC-7.03.051](ss-07/BC-7.03.051.md) | protect-bc: jq-missing fail-closed (exit 1) | draft | TBD | TBD |
| [BC-7.03.052](ss-07/BC-7.03.052.md) | protect-bc: missing file or non-BC path → allow | draft | TBD | TBD |
| [BC-7.03.053](ss-07/BC-7.03.053.md) | protect-secrets: identity & dual registry binding | draft | TBD | TBD |
| [BC-7.03.054](ss-07/BC-7.03.054.md) | protect-secrets: blocks Read of `.env`/`.envrc`/`.env.*` (excludes `.example`/`.sample`/`.template`) | draft | TBD | TBD |
| [BC-7.03.055](ss-07/BC-7.03.055.md) | protect-secrets: blocks shell content reads of .env files | draft | TBD | TBD |
| [BC-7.03.056](ss-07/BC-7.03.056.md) | protect-secrets: blocks copy/move of real .env (allows template→real) | draft | TBD | TBD |
| [BC-7.03.057](ss-07/BC-7.03.057.md) | protect-secrets: blocks tar/zip including .env | draft | TBD | TBD |
| [BC-7.03.058](ss-07/BC-7.03.058.md) | protect-secrets: blocks `echo\ | draft | TBD | TBD |
| [BC-7.03.059](ss-07/BC-7.03.059.md) | protect-secrets: blocks env\ | draft | TBD | TBD |
| [BC-7.03.060](ss-07/BC-7.03.060.md) | protect-vp: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.061](ss-07/BC-7.03.061.md) | protect-vp: denies edits to green VPs at .factory/specs/verification-properties/VP-*.md | draft | TBD | TBD |
| [BC-7.03.062](ss-07/BC-7.03.062.md) | purity-check: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.063](ss-07/BC-7.03.063.md) | purity-check: scopes to pure/core/kernel paths or *_pure.rs / *.pure.ts | draft | TBD | TBD |
| [BC-7.03.064](ss-07/BC-7.03.064.md) | purity-check: detects 16 forbidden side-effect patterns and warns | draft | TBD | TBD |
| [BC-7.03.065](ss-07/BC-7.03.065.md) | red-gate: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.066](ss-07/BC-7.03.066.md) | red-gate: skips test files always | draft | TBD | TBD |
| [BC-7.03.067](ss-07/BC-7.03.067.md) | red-gate: scopes to known source extensions | draft | TBD | TBD |
| [BC-7.03.068](ss-07/BC-7.03.068.md) | red-gate: state file absent → skip | draft | TBD | TBD |
| [BC-7.03.069](ss-07/BC-7.03.069.md) | red-gate: blocks edits to files not in `red[]` (path normalization 4-way) | draft | TBD | TBD |
| [BC-7.03.070](ss-07/BC-7.03.070.md) | red-gate: jq parse error → fail-closed exit 1 | draft | TBD | TBD |
| [BC-7.03.071](ss-07/BC-7.03.071.md) | regression-gate: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.072](ss-07/BC-7.03.072.md) | regression-gate: matches 9 test runners | draft | TBD | TBD |
| [BC-7.03.073](ss-07/BC-7.03.073.md) | regression-gate: pass/fail derivation prefers exit_code; falls back to interrupted | draft | TBD | TBD |
| [BC-7.03.074](ss-07/BC-7.03.074.md) | regression-gate: writes state file with status, timestamp, command | draft | TBD | TBD |
| [BC-7.03.075](ss-07/BC-7.03.075.md) | regression-gate: warns on pass→fail transition | draft | TBD | TBD |
| [BC-7.03.076](ss-07/BC-7.03.076.md) | session-learning: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.077](ss-07/BC-7.03.077.md) | session-learning: appends timestamped marker to .factory/sidecar-learning.md | draft | TBD | TBD |
| [BC-7.03.078](ss-07/BC-7.03.078.md) | session-learning: skips when .factory/ absent | draft | TBD | TBD |
| [BC-7.03.079](ss-07/BC-7.03.079.md) | track-agent-start: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.080](ss-07/BC-7.03.080.md) | track-agent-start: emits agent.start with subagent + best-effort story_id | draft | TBD | TBD |
| [BC-7.03.081](ss-07/BC-7.03.081.md) | track-agent-stop: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.082](ss-07/BC-7.03.082.md) | track-agent-stop: classifies result as ok\ | draft | TBD | TBD |
| [BC-7.03.083](ss-07/BC-7.03.083.md) | update-wave-state-on-merge: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.084](ss-07/BC-7.03.084.md) | update-wave-state-on-merge: scopes to pr-manager + successful merge signal | draft | TBD | TBD |
| [BC-7.03.085](ss-07/BC-7.03.085.md) | update-wave-state-on-merge: appends story to wave_data.stories_merged via python YAML | draft | TBD | TBD |
| [BC-7.03.086](ss-07/BC-7.03.086.md) | update-wave-state-on-merge: flips gate_status to pending when wave fully merged | draft | TBD | TBD |
| [BC-7.03.087](ss-07/BC-7.03.087.md) | verify-git-push: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.088](ss-07/BC-7.03.088.md) | verify-git-push: blocks --force / -f, allows --force-with-lease | draft | TBD | TBD |
| [BC-7.03.089](ss-07/BC-7.03.089.md) | verify-git-push: blocks pushes to main\ | draft | TBD | TBD |
| [BC-7.03.090](ss-07/BC-7.03.090.md) | verify-git-push: emits additionalContext on allowed pushes | draft | TBD | TBD |
| [BC-7.03.091](ss-07/BC-7.03.091.md) | warn-pending-wave-gate: identity & registry binding | draft | TBD | TBD |
| [BC-7.03.092](ss-07/BC-7.03.092.md) | warn-pending-wave-gate: stderr warning when any wave has gate_status: pending | draft | TBD | TBD |
| [BC-7.03.093](ss-07/BC-7.03.093.md) | verify-git-push: identity confirmation (already covered in BC-1086 routing section) | draft | TBD | TBD |
| [BC-7.04.001](ss-07/BC-7.04.001.md) | validate-anchor-capabilities-union: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.002](ss-07/BC-7.04.002.md) | validate-anchor-capabilities-union: scopes to .factory/stories/S-*.md or STORY-*.md | draft | TBD | TBD |
| [BC-7.04.003](ss-07/BC-7.04.003.md) | validate-anchor-capabilities-union: blocks when frontmatter caps ≠ union over BC capability fields | draft | TBD | TBD |
| [BC-7.04.004](ss-07/BC-7.04.004.md) | validate-anchor-capabilities-union: missing BC files warn-only | draft | TBD | TBD |
| [BC-7.04.005](ss-07/BC-7.04.005.md) | validate-bc-title: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.006](ss-07/BC-7.04.006.md) | validate-bc-title: scopes to behavioral-contracts/BC-*.md, skips BC-INDEX | draft | TBD | TBD |
| [BC-7.04.007](ss-07/BC-7.04.007.md) | validate-bc-title: blocks when BC H1 != BC-INDEX row title | draft | TBD | TBD |
| [BC-7.04.008](ss-07/BC-7.04.008.md) | validate-changelog-monotonicity: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.009](ss-07/BC-7.04.009.md) | validate-changelog-monotonicity: scopes to .factory/*.md, skips known no-changelog files | draft | TBD | TBD |
| [BC-7.04.010](ss-07/BC-7.04.010.md) | validate-changelog-monotonicity: blocks duplicate version rows | draft | TBD | TBD |
| [BC-7.04.011](ss-07/BC-7.04.011.md) | validate-changelog-monotonicity: blocks rows with date older above newer (non-decreasing) | draft | TBD | TBD |
| [BC-7.04.012](ss-07/BC-7.04.012.md) | validate-changelog-monotonicity: cross-checks frontmatter version against top changelog row | draft | TBD | TBD |
| [BC-7.04.013](ss-07/BC-7.04.013.md) | validate-demo-evidence-story-scoped: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.014](ss-07/BC-7.04.014.md) | validate-demo-evidence-story-scoped: blocks flat-level demo-evidence files | draft | TBD | TBD |
| [BC-7.04.015](ss-07/BC-7.04.015.md) | validate-factory-path-root: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.016](ss-07/BC-7.04.016.md) | validate-factory-path-root: blocks paths through .worktrees/<X>/.factory/ | draft | TBD | TBD |
| [BC-7.04.017](ss-07/BC-7.04.017.md) | validate-finding-format: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.018](ss-07/BC-7.04.018.md) | validate-finding-format: blocks legacy ADV-NNN and ADV-PN-NNN formats | draft | TBD | TBD |
| [BC-7.04.019](ss-07/BC-7.04.019.md) | validate-finding-format: blocks legacy STORY-NNN-FIX-NNN format | draft | TBD | TBD |
| [BC-7.04.020](ss-07/BC-7.04.020.md) | validate-index-self-reference: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.021](ss-07/BC-7.04.021.md) | validate-index-self-reference: warns when index lacks current pass/burst row | draft | TBD | TBD |
| [BC-7.04.022](ss-07/BC-7.04.022.md) | validate-input-hash: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.023](ss-07/BC-7.04.023.md) | validate-input-hash: skips intentional placeholders [live-state] and [pending-recompute] | draft | TBD | TBD |
| [BC-7.04.024](ss-07/BC-7.04.024.md) | validate-input-hash: warns on missing/template/null hash | draft | TBD | TBD |
| [BC-7.04.025](ss-07/BC-7.04.025.md) | validate-input-hash: BLOCKS on hash length != 7 | draft | TBD | TBD |
| [BC-7.04.026](ss-07/BC-7.04.026.md) | validate-input-hash: BLOCKS on non-hex chars | draft | TBD | TBD |
| [BC-7.04.027](ss-07/BC-7.04.027.md) | validate-input-hash: warns on stored ≠ computed (advisory) | draft | TBD | TBD |
| [BC-7.04.028](ss-07/BC-7.04.028.md) | validate-novelty-assessment: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.029](ss-07/BC-7.04.029.md) | validate-novelty-assessment: scopes to adversarial review pass files; excludes index/findings | draft | TBD | TBD |
| [BC-7.04.030](ss-07/BC-7.04.030.md) | validate-novelty-assessment: blocks when section is missing | draft | TBD | TBD |
| [BC-7.04.031](ss-07/BC-7.04.031.md) | validate-novelty-assessment: requires Pass / Novelty score / Verdict / Trajectory fields | draft | TBD | TBD |
| [BC-7.04.032](ss-07/BC-7.04.032.md) | validate-pr-description-completeness: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.033](ss-07/BC-7.04.033.md) | validate-pr-description-completeness: scopes to code-delivery/<STORY>/pr-description.md | draft | TBD | TBD |
| [BC-7.04.034](ss-07/BC-7.04.034.md) | validate-pr-description-completeness: blocks missing 6 required H2 sections | draft | TBD | TBD |
| [BC-7.04.035](ss-07/BC-7.04.035.md) | validate-pr-description-completeness: blocks unresolved {placeholder} tokens | draft | TBD | TBD |
| [BC-7.04.036](ss-07/BC-7.04.036.md) | validate-pr-merge-prerequisites: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.037](ss-07/BC-7.04.037.md) | validate-pr-merge-prerequisites: scopes to github-ops merge dispatches | draft | TBD | TBD |
| [BC-7.04.038](ss-07/BC-7.04.038.md) | validate-pr-merge-prerequisites: blocks when 3 evidence files missing | draft | TBD | TBD |
| [BC-7.04.039](ss-07/BC-7.04.039.md) | validate-pr-merge-prerequisites: missing delivery dir is warn-only (graceful early pipeline) | draft | TBD | TBD |
| [BC-7.04.040](ss-07/BC-7.04.040.md) | validate-pr-review-posted: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.041](ss-07/BC-7.04.041.md) | validate-pr-review-posted: scopes to pr-reviewer / pr-review-triage | draft | TBD | TBD |
| [BC-7.04.042](ss-07/BC-7.04.042.md) | validate-pr-review-posted: blocks when pr-review.md not written | draft | TBD | TBD |
| [BC-7.04.043](ss-07/BC-7.04.043.md) | validate-pr-review-posted: blocks `gh pr comment` fallback (not a review verdict) | draft | TBD | TBD |
| [BC-7.04.044](ss-07/BC-7.04.044.md) | validate-pr-review-posted: blocks when no formal review posted | draft | TBD | TBD |
| [BC-7.04.045](ss-07/BC-7.04.045.md) | validate-state-index-status-coherence: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.046](ss-07/BC-7.04.046.md) | validate-state-index-status-coherence: scopes to STATE.md or cycles/*/INDEX.md | draft | TBD | TBD |
| [BC-7.04.047](ss-07/BC-7.04.047.md) | validate-state-index-status-coherence: WARNS (exit 1) when STATE.convergence_status drifts from c... | draft | TBD | TBD |
| [BC-7.04.048](ss-07/BC-7.04.048.md) | validate-state-pin-freshness: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.049](ss-07/BC-7.04.049.md) | validate-state-pin-freshness: blocks when STATE pin != artifact frontmatter version (5 fields) | draft | TBD | TBD |
| [BC-7.04.050](ss-07/BC-7.04.050.md) | validate-state-size: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.051](ss-07/BC-7.04.051.md) | validate-state-size: warns at >200 lines, blocks at >500 unless reducing | draft | TBD | TBD |
| [BC-7.04.052](ss-07/BC-7.04.052.md) | validate-story-bc-sync: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.053](ss-07/BC-7.04.053.md) | validate-story-bc-sync: blocks frontmatter BC missing from body BC table | draft | TBD | TBD |
| [BC-7.04.054](ss-07/BC-7.04.054.md) | validate-story-bc-sync: blocks frontmatter BC missing AC trace annotation | draft | TBD | TBD |
| [BC-7.04.055](ss-07/BC-7.04.055.md) | validate-story-bc-sync: blocks body-table BC missing from frontmatter array | draft | TBD | TBD |
| [BC-7.04.056](ss-07/BC-7.04.056.md) | validate-subsystem-names: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.057](ss-07/BC-7.04.057.md) | validate-subsystem-names: scopes to BC-*.md and STORY-*.md, requires ARCH-INDEX | draft | TBD | TBD |
| [BC-7.04.058](ss-07/BC-7.04.058.md) | validate-subsystem-names: blocks BC `subsystem:` not in canonical SS-NN list | draft | TBD | TBD |
| [BC-7.04.059](ss-07/BC-7.04.059.md) | validate-subsystem-names: blocks story `subsystems:` array members not in canonical list | draft | TBD | TBD |
| [BC-7.04.060](ss-07/BC-7.04.060.md) | validate-table-cell-count: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.061](ss-07/BC-7.04.061.md) | validate-table-cell-count: scopes to .factory/*.md | draft | TBD | TBD |
| [BC-7.04.062](ss-07/BC-7.04.062.md) | validate-table-cell-count: blocks rows with pipe count != header pipe count | draft | TBD | TBD |
| [BC-7.04.063](ss-07/BC-7.04.063.md) | validate-template-compliance: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.064](ss-07/BC-7.04.064.md) | validate-template-compliance: skips INDEX/yaml/json/current-cycle files | draft | TBD | TBD |
| [BC-7.04.065](ss-07/BC-7.04.065.md) | validate-template-compliance: resolves template by document_type, then path pattern (15 patterns) | draft | TBD | TBD |
| [BC-7.04.066](ss-07/BC-7.04.066.md) | validate-template-compliance: blocks missing frontmatter keys vs template | draft | TBD | TBD |
| [BC-7.04.067](ss-07/BC-7.04.067.md) | validate-template-compliance: blocks missing H2 sections vs template | draft | TBD | TBD |
| [BC-7.04.068](ss-07/BC-7.04.068.md) | validate-vp-consistency: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.069](ss-07/BC-7.04.069.md) | validate-vp-consistency: scopes to VP-INDEX.md, verification-architecture.md, verification-covera... | draft | TBD | TBD |
| [BC-7.04.070](ss-07/BC-7.04.070.md) | validate-vp-consistency: blocks VPs in INDEX missing from verification-architecture.md | draft | TBD | TBD |
| [BC-7.04.071](ss-07/BC-7.04.071.md) | validate-vp-consistency: blocks VPs in INDEX missing from coverage-matrix | draft | TBD | TBD |
| [BC-7.04.072](ss-07/BC-7.04.072.md) | validate-vp-consistency: blocks VP-INDEX summary totals != row counts | draft | TBD | TBD |
| [BC-7.04.073](ss-07/BC-7.04.073.md) | validate-vp-consistency: blocks coverage-matrix column sum != VP-INDEX summary total | draft | TBD | TBD |
| [BC-7.04.074](ss-07/BC-7.04.074.md) | validate-vp-consistency: blocks VPs in coverage-matrix missing from VP-INDEX | draft | TBD | TBD |
| [BC-7.04.075](ss-07/BC-7.04.075.md) | validate-wave-gate-completeness: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.076](ss-07/BC-7.04.076.md) | validate-wave-gate-completeness: scopes to wave-state.yaml writes; finds newly-passed waves | draft | TBD | TBD |
| [BC-7.04.077](ss-07/BC-7.04.077.md) | validate-wave-gate-completeness: blocks gate_status:passed without gate_report | draft | TBD | TBD |
| [BC-7.04.078](ss-07/BC-7.04.078.md) | validate-wave-gate-completeness: blocks when gate_report file not found | draft | TBD | TBD |
| [BC-7.04.079](ss-07/BC-7.04.079.md) | validate-wave-gate-completeness: blocks gate report missing evidence for any of 6 gates | draft | TBD | TBD |
| [BC-7.04.080](ss-07/BC-7.04.080.md) | validate-wave-gate-prerequisite: identity & registry binding | draft | TBD | TBD |
| [BC-7.04.081](ss-07/BC-7.04.081.md) | validate-wave-gate-prerequisite: adversary dispatches go through SHA-currency hook | draft | TBD | TBD |
| [BC-7.04.082](ss-07/BC-7.04.082.md) | validate-wave-gate-prerequisite: worker agents go through gate prerequisite check | draft | TBD | TBD |
| [BC-7.04.083](ss-07/BC-7.04.083.md) | validate-wave-gate-prerequisite: skips non-worker, non-adversary subagents | draft | TBD | TBD |
| [BC-7.05.001](ss-07/BC-7.05.001.md) | validate-count-propagation.sh detects count drift across index files and exits non-zero | draft | CAP-001 | S-7.02 |
| [BC-7.05.002](ss-07/BC-7.05.002.md) | validate-count-propagation.sh runs in under 200ms and is deterministic | draft | CAP-001 | S-7.02 |
| [BC-7.05.003](ss-07/BC-7.05.003.md) | validate-template-compliance.sh enforces VP multi-BC source_bc convention | draft | CAP-001 | S-7.02 |
| [BC-7.05.004](ss-07/BC-7.05.004.md) | hooks-registry.toml registers validate-count-propagation.sh as PostToolUse on index file writes | draft | CAP-001 | S-7.02 |

### SS-08 — Templates and Rules (BC-8)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-8.01.001](ss-08/BC-8.01.001.md) | product-brief-template: governs L1 Product Brief artifact identity | draft | CAP-TBD | TBD |
| [BC-8.01.002](ss-08/BC-8.01.002.md) | product-brief-template: required frontmatter fields | draft | CAP-TBD | TBD |
| [BC-8.01.003](ss-08/BC-8.01.003.md) | product-brief-template: required sections | draft | CAP-TBD | TBD |
| [BC-8.01.004](ss-08/BC-8.01.004.md) | L2-domain-spec-template: deprecated monolithic L2 template kept for reference | draft | CAP-TBD | TBD |
| [BC-8.01.005](ss-08/BC-8.01.005.md) | L2-domain-spec-template: required sections (legacy 10-section schema) | draft | CAP-TBD | TBD |
| [BC-8.01.006](ss-08/BC-8.01.006.md) | L2-domain-spec-index-template: sharded L2 index identity | draft | CAP-TBD | TBD |
| [BC-8.01.007](ss-08/BC-8.01.007.md) | L2-domain-spec-index-template: required sections | draft | CAP-TBD | TBD |
| [BC-8.01.008](ss-08/BC-8.01.008.md) | L2-domain-spec-section-template: minimal per-section shard | draft | CAP-TBD | TBD |
| [BC-8.01.009](ss-08/BC-8.01.009.md) | prd-template: governs L3 PRD identity | draft | CAP-TBD | TBD |
| [BC-8.01.010](ss-08/BC-8.01.010.md) | prd-template: required sections (1 through 7) | draft | CAP-TBD | TBD |
| [BC-8.01.011](ss-08/BC-8.01.011.md) | prd-supplement-error-taxonomy-template: PRD error-taxonomy supplement | draft | CAP-TBD | TBD |
| [BC-8.01.012](ss-08/BC-8.01.012.md) | prd-supplement-interface-definitions-template: PRD CLI/JSON/Config supplement | draft | CAP-TBD | TBD |
| [BC-8.01.013](ss-08/BC-8.01.013.md) | prd-supplement-nfr-catalog-template: PRD non-functional requirements supplement | draft | CAP-TBD | TBD |
| [BC-8.01.014](ss-08/BC-8.01.014.md) | prd-supplement-test-vectors-template: canonical test vectors supplement | draft | CAP-TBD | TBD |
| [BC-8.01.015](ss-08/BC-8.01.015.md) | behavioral-contract-template: per-BC structural contract | draft | CAP-TBD | TBD |
| [BC-8.01.016](ss-08/BC-8.01.016.md) | behavioral-contract-template: optional anchor sections | draft | CAP-TBD | TBD |
| [BC-8.01.017](ss-08/BC-8.01.017.md) | L4-verification-property-template: VP-NNN identity + lifecycle | draft | CAP-TBD | TBD |
| [BC-8.02.001](ss-08/BC-8.02.001.md) | story-template: governs STORY-NNN identity | draft | CAP-TBD | TBD |
| [BC-8.02.002](ss-08/BC-8.02.002.md) | story-template: required sections (10 mandatory) | draft | CAP-TBD | TBD |
| [BC-8.02.003](ss-08/BC-8.02.003.md) | story-template: optional planning + ASM/R + lifecycle frontmatter blocks | draft | CAP-TBD | TBD |
| [BC-8.02.004](ss-08/BC-8.02.004.md) | story-index-template: STORY-INDEX identity | draft | CAP-TBD | TBD |
| [BC-8.02.005](ss-08/BC-8.02.005.md) | epic-template: EPIC-XXX identity | draft | CAP-TBD | TBD |
| [BC-8.02.006](ss-08/BC-8.02.006.md) | epic-index-template: EPIC-INDEX identity + epic-to-capability mapping | draft | CAP-TBD | TBD |
| [BC-8.02.007](ss-08/BC-8.02.007.md) | cycle-manifest-template: per-cycle manifest identity | draft | CAP-TBD | TBD |
| [BC-8.03.001](ss-08/BC-8.03.001.md) | architecture-template: governs L3 architecture document identity | draft | CAP-TBD | TBD |
| [BC-8.03.002](ss-08/BC-8.03.002.md) | architecture-template: Part 1 sections (1–9, system + data + integration) | draft | CAP-TBD | TBD |
| [BC-8.03.003](ss-08/BC-8.03.003.md) | architecture-template: Part 2 verification architecture (10–14) | draft | CAP-TBD | TBD |
| [BC-8.03.004](ss-08/BC-8.03.004.md) | architecture-template: Part 3 module specifications (15) | draft | CAP-TBD | TBD |
| [BC-8.03.005](ss-08/BC-8.03.005.md) | architecture-index-template: ARCH-INDEX governs sharded architecture index | draft | CAP-TBD | TBD |
| [BC-8.03.006](ss-08/BC-8.03.006.md) | architecture-section-template: per-section ARCH-NN shard | draft | CAP-TBD | TBD |
| [BC-8.03.007](ss-08/BC-8.03.007.md) | architecture-feasibility-report-template: pre-architecture feasibility check | draft | CAP-TBD | TBD |
| [BC-8.03.008](ss-08/BC-8.03.008.md) | verification-architecture-template: verification-arch shard | draft | CAP-TBD | TBD |
| [BC-8.03.009](ss-08/BC-8.03.009.md) | verification-coverage-matrix-template: coverage matrix shard | draft | CAP-TBD | TBD |
| [BC-8.03.010](ss-08/BC-8.03.010.md) | verification-gap-analysis-template: brownfield verification gap report | draft | CAP-TBD | TBD |
| [BC-8.03.011](ss-08/BC-8.03.011.md) | recovered-architecture-template: brownfield recovered architecture | draft | CAP-TBD | TBD |
| [BC-8.04.001](ss-08/BC-8.04.001.md) | adversarial-review-template: per-pass adversarial review identity | draft | CAP-TBD | TBD |
| [BC-8.04.002](ss-08/BC-8.04.002.md) | adversarial-review-index-template: per-pass index of findings | draft | CAP-TBD | TBD |
| [BC-8.04.003](ss-08/BC-8.04.003.md) | adversarial-finding-template: per-finding ADV-N identity | draft | CAP-TBD | TBD |
| [BC-8.04.004](ss-08/BC-8.04.004.md) | findings-tracker-template: cycle-level findings tracker | draft | CAP-TBD | TBD |
| [BC-8.04.005](ss-08/BC-8.04.005.md) | fix-template: per-fix FIX-P[N]-NNN identity | draft | CAP-TBD | TBD |
| [BC-8.04.006](ss-08/BC-8.04.006.md) | convergence-trajectory-template: pass-by-pass finding trajectory | draft | CAP-TBD | TBD |
| [BC-8.04.007](ss-08/BC-8.04.007.md) | review-findings-template: PR-review findings per story | draft | CAP-TBD | TBD |
| [BC-8.04.008](ss-08/BC-8.04.008.md) | code-review-template: code-reviewer per-pass output | draft | CAP-TBD | TBD |
| [BC-8.04.009](ss-08/BC-8.04.009.md) | agent-file-review-template: agent persona doc review | draft | CAP-TBD | TBD |
| [BC-8.04.010](ss-08/BC-8.04.010.md) | adversary-prompt-templates: subdir governs phase-specific adversary prompt scaffolds | draft | CAP-TBD | TBD |
| [BC-8.04.011](ss-08/BC-8.04.011.md) | adversary-prompt-templates: required Review Focus + Not-Reviewing sections | draft | CAP-TBD | TBD |
| [BC-8.04.012](ss-08/BC-8.04.012.md) | adversary-prompt-templates: previous-findings handlebars template | draft | CAP-TBD | TBD |
| [BC-8.05.001](ss-08/BC-8.05.001.md) | holdout-scenario-template: HS-NNN scenario identity | draft | CAP-TBD | TBD |
| [BC-8.05.002](ss-08/BC-8.05.002.md) | holdout-scenario-index-template: HS-INDEX scenario catalog | draft | CAP-TBD | TBD |
| [BC-8.05.003](ss-08/BC-8.05.003.md) | evaluation-per-scenario-template: HS-NNN per-scenario evaluation | draft | CAP-TBD | TBD |
| [BC-8.05.004](ss-08/BC-8.05.004.md) | evaluation-index-template: per-pass holdout evaluation index | draft | CAP-TBD | TBD |
| [BC-8.05.005](ss-08/BC-8.05.005.md) | evaluation-summary-template: holdout evaluation final summary | draft | CAP-TBD | TBD |
| [BC-8.05.006](ss-08/BC-8.05.006.md) | holdout-evaluation-report-template: cycle-level holdout report | draft | CAP-TBD | TBD |
| [BC-8.06.001](ss-08/BC-8.06.001.md) | convergence-report-template: 7-dimension pipeline convergence scorecard | draft | CAP-TBD | TBD |
| [BC-8.06.002](ss-08/BC-8.06.002.md) | convergence-report-template: 7 named dimensions | draft | CAP-TBD | TBD |
| [BC-8.06.003](ss-08/BC-8.06.003.md) | consistency-report-template: 10-section L1→L4 consistency validation | draft | CAP-TBD | TBD |
| [BC-8.06.004](ss-08/BC-8.06.004.md) | consistency-validation-report-template: minimal consistency-validation gate output | draft | CAP-TBD | TBD |
| [BC-8.06.005](ss-08/BC-8.06.005.md) | traceability-matrix-template: forward + reverse L1→Proof traceability | draft | CAP-TBD | TBD |
| [BC-8.06.006](ss-08/BC-8.06.006.md) | traceability-matrices-template: multi-axis traceability collection (BC/VP/NFR/clause/edge/gap) | draft | CAP-TBD | TBD |
| [BC-8.07.001](ss-08/BC-8.07.001.md) | project-context-template: brownfield project context summary | draft | CAP-TBD | TBD |
| [BC-8.07.002](ss-08/BC-8.07.002.md) | conventions-template: brownfield conventions extraction | draft | CAP-TBD | TBD |
| [BC-8.07.003](ss-08/BC-8.07.003.md) | extraction-validation-template: brownfield extraction validation | draft | CAP-TBD | TBD |
| [BC-8.07.004](ss-08/BC-8.07.004.md) | gene-transfusion-assessment-template: brownfield gene-transfusion candidate assessment | draft | CAP-TBD | TBD |
| [BC-8.07.005](ss-08/BC-8.07.005.md) | domain-research-template: L2 domain-research report | draft | CAP-TBD | TBD |
| [BC-8.07.006](ss-08/BC-8.07.006.md) | research-index-template: per-cycle research index | draft | CAP-TBD | TBD |
| [BC-8.07.007](ss-08/BC-8.07.007.md) | discovery-report-template: discovery-engine periodic report | draft | CAP-TBD | TBD |
| [BC-8.07.008](ss-08/BC-8.07.008.md) | idea-brief-template: pre-brief idea capture | draft | CAP-TBD | TBD |
| [BC-8.07.009](ss-08/BC-8.07.009.md) | feature-request-template: feature-mode FR-NNN identity | draft | CAP-TBD | TBD |
| [BC-8.07.010](ss-08/BC-8.07.010.md) | delta-analysis-report-template: feature-mode delta analysis | draft | CAP-TBD | TBD |
| [BC-8.08.001](ss-08/BC-8.08.001.md) | demo-evidence-report-template: per-product demo evidence rollup | draft | CAP-TBD | TBD |
| [BC-8.08.002](ss-08/BC-8.08.002.md) | demo-tape-template: VHS .tape demo recording template | draft | CAP-TBD | TBD |
| [BC-8.08.003](ss-08/BC-8.08.003.md) | demo-playwright-template: Playwright per-AC video+screenshot demo | draft | CAP-TBD | TBD |
| [BC-8.08.004](ss-08/BC-8.08.004.md) | demo-ci-workflow-template: GitHub Actions demo-generation workflow | draft | CAP-TBD | TBD |
| [BC-8.09.001](ss-08/BC-8.09.001.md) | formal-verification-template: formal-verify pass output | draft | CAP-TBD | TBD |
| [BC-8.09.002](ss-08/BC-8.09.002.md) | fuzz-report-template: fuzz testing per-target report | draft | CAP-TBD | TBD |
| [BC-8.09.003](ss-08/BC-8.09.003.md) | performance-report-template: perf-check pass output | draft | CAP-TBD | TBD |
| [BC-8.09.004](ss-08/BC-8.09.004.md) | security-review-template: security-review per-pass output | draft | CAP-TBD | TBD |
| [BC-8.09.005](ss-08/BC-8.09.005.md) | security-scan-report-template: static-analysis security scan | draft | CAP-TBD | TBD |
| [BC-8.10.001](ss-08/BC-8.10.001.md) | dtu-assessment-template: DTU assessment for SUT | draft | CAP-TBD | TBD |
| [BC-8.10.002](ss-08/BC-8.10.002.md) | dtu-clone-spec-template: per-service DTU clone specification | draft | CAP-TBD | TBD |
| [BC-8.10.003](ss-08/BC-8.10.003.md) | dtu-fidelity-report-template: DTU clone fidelity report | draft | CAP-TBD | TBD |
| [BC-8.11.001](ss-08/BC-8.11.001.md) | ux-spec-template: deprecated monolithic UX spec | draft | CAP-TBD | TBD |
| [BC-8.11.002](ss-08/BC-8.11.002.md) | ux-spec-index-template: sharded UX-spec index | draft | CAP-TBD | TBD |
| [BC-8.11.003](ss-08/BC-8.11.003.md) | ux-spec-screen-template: per-screen SCR-NNN spec | draft | CAP-TBD | TBD |
| [BC-8.11.004](ss-08/BC-8.11.004.md) | ux-spec-flow-template: per-flow FLOW-NNN spec | draft | CAP-TBD | TBD |
| [BC-8.12.001](ss-08/BC-8.12.001.md) | design-system/: subdir governs design-token + component-contract + pattern catalog | draft | CAP-TBD | TBD |
| [BC-8.12.002](ss-08/BC-8.12.002.md) | design-system/constraints.yaml: global UI generation rules | draft | CAP-TBD | TBD |
| [BC-8.12.003](ss-08/BC-8.12.003.md) | design-system/tokens/: 7 token JSON catalogs | draft | CAP-TBD | TBD |
| [BC-8.12.004](ss-08/BC-8.12.004.md) | design-system/components/: registry + 11 component contracts | draft | CAP-TBD | TBD |
| [BC-8.12.005](ss-08/BC-8.12.005.md) | design-system/patterns/: 3 cross-component pattern catalogs | draft | CAP-TBD | TBD |
| [BC-8.12.006](ss-08/BC-8.12.006.md) | ui-quality/: subdir governs UI quality gate + report templates | draft | CAP-TBD | TBD |
| [BC-8.12.007](ss-08/BC-8.12.007.md) | ui-quality/gate-report-template: 4-gate-level UI quality gate | draft | CAP-TBD | TBD |
| [BC-8.12.008](ss-08/BC-8.12.008.md) | ui-quality/heuristic-evaluation-template: 10-heuristic UX evaluation | draft | CAP-TBD | TBD |
| [BC-8.12.009](ss-08/BC-8.12.009.md) | ui-quality/responsive-report-template: 4-breakpoint responsive validation | draft | CAP-TBD | TBD |
| [BC-8.12.010](ss-08/BC-8.12.010.md) | ui-quality/completeness-report-template: UI completeness fidelity report | draft | CAP-TBD | TBD |
| [BC-8.12.011](ss-08/BC-8.12.011.md) | ui-traceability-template: UI element → story → component → test → visual evidence matrix | draft | CAP-TBD | TBD |
| [BC-8.13.001](ss-08/BC-8.13.001.md) | spec-changelog-template: spec-versioning changelog | draft | CAP-TBD | TBD |
| [BC-8.13.002](ss-08/BC-8.13.002.md) | spec-drift-report-template: spec-drift report | draft | CAP-TBD | TBD |
| [BC-8.13.003](ss-08/BC-8.13.003.md) | vp-withdrawal-template: green-VP retirement record | draft | CAP-TBD | TBD |
| [BC-8.13.004](ss-08/BC-8.13.004.md) | design-drift-template: design-drift detection report | draft | CAP-TBD | TBD |
| [BC-8.14.001](ss-08/BC-8.14.001.md) | state-template: STATE.md pipeline-state identity | draft | CAP-TBD | TBD |
| [BC-8.14.002](ss-08/BC-8.14.002.md) | state-manager-checklist-template: wave-gate remediation-burst checklist | draft | CAP-TBD | TBD |
| [BC-8.14.003](ss-08/BC-8.14.003.md) | burst-log-template: state-burst log | draft | CAP-TBD | TBD |
| [BC-8.14.004](ss-08/BC-8.14.004.md) | session-checkpoints-template: cycle session resume checkpoints | draft | CAP-TBD | TBD |
| [BC-8.14.005](ss-08/BC-8.14.005.md) | session-review-template: post-cycle session review | draft | CAP-TBD | TBD |
| [BC-8.14.006](ss-08/BC-8.14.006.md) | lessons-template: cycle lessons-learned | draft | CAP-TBD | TBD |
| [BC-8.14.007](ss-08/BC-8.14.007.md) | blocking-issues-resolved-template: cycle blockers-resolved log | draft | CAP-TBD | TBD |
| [BC-8.14.008](ss-08/BC-8.14.008.md) | wave-schedule-template: per-cycle wave schedule | draft | CAP-TBD | TBD |
| [BC-8.14.009](ss-08/BC-8.14.009.md) | wave-state-template: wave-state.yaml lifecycle tracker schema | draft | CAP-TBD | TBD |
| [BC-8.14.010](ss-08/BC-8.14.010.md) | red-gate-log-template: TDD red-gate verification log | draft | CAP-TBD | TBD |
| [BC-8.15.001](ss-08/BC-8.15.001.md) | pr-description-template: per-story PR description | draft | CAP-TBD | TBD |
| [BC-8.15.002](ss-08/BC-8.15.002.md) | release-notes-template: per-version release notes | draft | CAP-TBD | TBD |
| [BC-8.16.001](ss-08/BC-8.16.001.md) | autonomy-config-template: budget + protected-agents schema | draft | CAP-TBD | TBD |
| [BC-8.16.002](ss-08/BC-8.16.002.md) | merge-config-template: code-delivery autonomy + branch + PR config | draft | CAP-TBD | TBD |
| [BC-8.16.003](ss-08/BC-8.16.003.md) | policies-template: declarative governance policy registry schema | draft | CAP-TBD | TBD |
| [BC-8.16.004](ss-08/BC-8.16.004.md) | discovery-config-template: discovery-engine ingestion config | draft | CAP-TBD | TBD |
| [BC-8.16.005](ss-08/BC-8.16.005.md) | project-manifest-template: multi-repo project.yaml schema | draft | CAP-TBD | TBD |
| [BC-8.16.006](ss-08/BC-8.16.006.md) | reference-manifest-template: .reference/ rebuild manifest | draft | CAP-TBD | TBD |
| [BC-8.16.007](ss-08/BC-8.16.007.md) | factory-project-state-template: multi-repo project-level STATE.md | draft | CAP-TBD | TBD |
| [BC-8.16.008](ss-08/BC-8.16.008.md) | factory-project-structure-template: .factory-project/ multi-repo directory structure | draft | CAP-TBD | TBD |
| [BC-8.16.009](ss-08/BC-8.16.009.md) | tech-debt-register-template: project tech-debt register | draft | CAP-TBD | TBD |
| [BC-8.16.010](ss-08/BC-8.16.010.md) | sweep-report-template: maintenance sweep report | draft | CAP-TBD | TBD |
| [BC-8.16.011](ss-08/BC-8.16.011.md) | project-justfile-template: per-project justfile bootstrap | draft | CAP-TBD | TBD |
| [BC-8.16.012](ss-08/BC-8.16.012.md) | implementation-readiness-template: pre-implementation readiness gate | draft | CAP-TBD | TBD |
| [BC-8.16.013](ss-08/BC-8.16.013.md) | brief-validation-template: brief-quality gate report | draft | CAP-TBD | TBD |
| [BC-8.16.014](ss-08/BC-8.16.014.md) | module-criticality-template: module criticality classification | draft | CAP-TBD | TBD |
| [BC-8.17.001](ss-08/BC-8.17.001.md) | skill-execution-template: SKILL.md (execution variant) shape | draft | CAP-TBD | TBD |
| [BC-8.17.002](ss-08/BC-8.17.002.md) | skill-delegation-template: SKILL.md (delegation variant) shape | draft | CAP-TBD | TBD |
| [BC-8.17.003](ss-08/BC-8.17.003.md) | agents-md-template: AGENTS.md shape | draft | CAP-TBD | TBD |
| [BC-8.18.001](ss-08/BC-8.18.001.md) | verify-sha-currency.sh: state-manager burst-hygiene gate (template-distributed; opt-in, NOT registered as a vsdd-factory hook) | draft | CAP-TBD | TBD |
| [BC-8.19.001](ss-08/BC-8.19.001.md) | rules/_index.md: rule include-order via @-references | draft | CAP-TBD | TBD |
| [BC-8.20.001](ss-08/BC-8.20.001.md) | rules/bash.md: SHALL NOT suppress stderr with `2>/dev/null` in production scripts | draft | CAP-TBD | TBD |
| [BC-8.20.002](ss-08/BC-8.20.002.md) | rules/bash.md: SHALL NOT use `eval` in shell helpers | draft | CAP-TBD | TBD |
| [BC-8.20.003](ss-08/BC-8.20.003.md) | rules/bash.md: justfile recipes MUST guard optional tools with `command -v` check | draft | CAP-TBD | TBD |
| [BC-8.20.004](ss-08/BC-8.20.004.md) | rules/bash.md: test files MUST verify tool dependencies at the top before any assertions | draft | CAP-TBD | TBD |
| [BC-8.20.005](ss-08/BC-8.20.005.md) | rules/bash.md: negative assertions MUST verify the search tool ran successfully | draft | CAP-TBD | TBD |
| [BC-8.20.006](ss-08/BC-8.20.006.md) | rules/bash.md: literal-string matching SHALL use `grep -F` | draft | CAP-TBD | TBD |
| [BC-8.20.007](ss-08/BC-8.20.007.md) | rules/bash.md: test headers SHALL state accurate test counts | draft | CAP-TBD | TBD |
| [BC-8.20.008](ss-08/BC-8.20.008.md) | rules/bash.md: file-path references SHALL be validated by structural tests | draft | CAP-TBD | TBD |
| [BC-8.20.009](ss-08/BC-8.20.009.md) | rules/bash.md: every `2>/dev/null` MUST carry a `# STDERR-EXEMPT: <rationale>` tag | draft | CAP-TBD | TBD |
| [BC-8.20.010](ss-08/BC-8.20.010.md) | rules/bash.md: `just ci` MUST run the same commands as `.github/workflows/ci.yml` | draft | CAP-TBD | TBD |
| [BC-8.21.001](ss-08/BC-8.21.001.md) | rules/factory-protocol.md: `.factory/` is a git worktree on the orphan `factory-artifacts` branch | draft | CAP-TBD | TBD |
| [BC-8.21.002](ss-08/BC-8.21.002.md) | rules/factory-protocol.md: `.factory/` directory layout (canonical 8-section structure) | draft | CAP-TBD | TBD |
| [BC-8.21.003](ss-08/BC-8.21.003.md) | rules/factory-protocol.md: all `.factory/` changes commit to `factory-artifacts`, NOT main/develop | draft | CAP-TBD | TBD |
| [BC-8.21.004](ss-08/BC-8.21.004.md) | rules/factory-protocol.md: file lifecycle classification (Living / Accumulating / Cycle-scoped / Critical) | draft | CAP-TBD | TBD |
| [BC-8.21.005](ss-08/BC-8.21.005.md) | rules/factory-protocol.md: NEVER put target project source code in `.factory/` | draft | CAP-TBD | TBD |
| [BC-8.21.006](ss-08/BC-8.21.006.md) | rules/factory-protocol.md: NEVER modify `.factory/` files from main/develop branch | draft | CAP-TBD | TBD |
| [BC-8.21.007](ss-08/BC-8.21.007.md) | rules/factory-protocol.md: STATE.md is the single source of truth for pipeline progress | draft | CAP-TBD | TBD |
| [BC-8.21.008](ss-08/BC-8.21.008.md) | rules/factory-protocol.md: specs are the product, code is disposable (SOUL.md #3 reified) | draft | CAP-TBD | TBD |
| [BC-8.22.001](ss-08/BC-8.22.001.md) | rules/git-commits.md: all commits MUST follow Conventional Commits | draft | CAP-014 | S-0.05, S-5.05, S-5.06 |
| [BC-8.22.002](ss-08/BC-8.22.002.md) | rules/git-commits.md: commit type SHALL be one of 10 known values (feat/fix/docs/style/refactor/perf/test/build/ci/chore) | draft | CAP-TBD | TBD |
| [BC-8.22.003](ss-08/BC-8.22.003.md) | rules/git-commits.md: description uses imperative present tense, lowercase initial, no period | draft | CAP-TBD | TBD |
| [BC-8.22.004](ss-08/BC-8.22.004.md) | rules/git-commits.md: scope (optional) is parenthesized after type — `feat(api):` | draft | CAP-TBD | TBD |
| [BC-8.22.005](ss-08/BC-8.22.005.md) | rules/git-commits.md: body separated from description with blank line; explains motivation + previous behavior contrast | draft | CAP-TBD | TBD |
| [BC-8.22.006](ss-08/BC-8.22.006.md) | rules/git-commits.md: footers — `Refs:`, `Closes:`, `BREAKING CHANGE:` | draft | CAP-TBD | TBD |
| [BC-8.22.007](ss-08/BC-8.22.007.md) | rules/git-commits.md: breaking changes — `!` after type/scope OR `BREAKING CHANGE:` footer | draft | CAP-TBD | TBD |
| [BC-8.22.008](ss-08/BC-8.22.008.md) | rules/git-commits.md: NEVER include AI attribution in commit messages | draft | CAP-TBD | TBD |
| [BC-8.22.009](ss-08/BC-8.22.009.md) | rules/git-commits.md: NEVER use `gh pr merge --admin` without explicit per-merge user permission | draft | CAP-TBD | TBD |
| [BC-8.23.001](ss-08/BC-8.23.001.md) | rules/rust.md: every application crate MUST declare `#![forbid(unsafe_code)]` | draft | CAP-TBD | TBD |
| [BC-8.23.002](ss-08/BC-8.23.002.md) | rules/rust.md: NO `unwrap()` in production code — use `?` or `expect("actionable msg")` | draft | CAP-TBD | TBD |
| [BC-8.23.003](ss-08/BC-8.23.003.md) | rules/rust.md: NEVER block the async runtime — use `spawn_blocking` for CPU work, `tokio::time::sleep` for delays | draft | CAP-TBD | TBD |
| [BC-8.23.004](ss-08/BC-8.23.004.md) | rules/rust.md: type design — newtypes for IDs, validated constructors at trust boundaries, `#[non_exhaustive]` on growing enums, UUID v7 for time-ordered IDs | draft | CAP-TBD | TBD |
| [BC-8.23.005](ss-08/BC-8.23.005.md) | rules/rust.md: error handling — thiserror enums, per-crate `pub type Result<T>`, sanitize `Display` before client send | draft | CAP-TBD | TBD |
| [BC-8.23.006](ss-08/BC-8.23.006.md) | rules/rust.md: module structure — `lib.rs` is pure re-export barrel; impl in domain modules | draft | CAP-TBD | TBD |
| [BC-8.23.007](ss-08/BC-8.23.007.md) | rules/rust.md: dependencies declared at workspace level; Edition 2024, MSRV 1.85+; clippy warnings are errors | draft | CAP-TBD | TBD |
| [BC-8.23.008](ss-08/BC-8.23.008.md) | rules/rust.md: testing — unit/`#[cfg(test)]` in same file, integration in `tests/`, property in `tests/property_*.rs`, snapshot in `tests/snapshot_*.rs` | draft | CAP-TBD | TBD |
| [BC-8.23.009](ss-08/BC-8.23.009.md) | rules/rust.md: architecture — strictly acyclic dependency graph; no circular deps between crates | draft | CAP-TBD | TBD |
| [BC-8.24.001](ss-08/BC-8.24.001.md) | rules/spec-format.md: 4-level spec hierarchy (L1 brief / L2 domain / L3 BC / L4 VP) | draft | CAP-TBD | TBD |
| [BC-8.24.002](ss-08/BC-8.24.002.md) | rules/spec-format.md: BC numbering — `BC-S.SS.NNN` (S=subsystem, SS=section, NNN=contract) | draft | CAP-TBD | TBD |
| [BC-8.24.003](ss-08/BC-8.24.003.md) | rules/spec-format.md: BC file format SHALL contain Subsystem/Section/Contract/Preconditions/Postconditions/Error Cases/Verification/Traceability sections | draft | CAP-TBD | TBD |
| [BC-8.24.004](ss-08/BC-8.24.004.md) | rules/spec-format.md: BC-INDEX.md format — table with ID/Title/Subsystem/Status/Stories columns | draft | CAP-TBD | TBD |
| [BC-8.24.005](ss-08/BC-8.24.005.md) | rules/spec-format.md: VP numbering — sequential `VP-NNN` | draft | CAP-TBD | TBD |
| [BC-8.24.006](ss-08/BC-8.24.006.md) | rules/spec-format.md: VP file format SHALL contain Property/Type/Scope/Verification Method/Status/Traceability sections | draft | CAP-TBD | TBD |
| [BC-8.24.007](ss-08/BC-8.24.007.md) | rules/spec-format.md: green VPs are IMMUTABLE — modification requires new VP supersedes old | draft | CAP-TBD | TBD |
| [BC-8.24.008](ss-08/BC-8.24.008.md) | rules/spec-format.md: architecture is sharded into ARCH-NN sections, NOT a monolith | draft | CAP-TBD | TBD |
| [BC-8.24.009](ss-08/BC-8.24.009.md) | rules/spec-format.md: ARCH-NN section template — Overview/Decisions/Components/Data Flow/Constraints/Dependencies | draft | CAP-TBD | TBD |
| [BC-8.24.010](ss-08/BC-8.24.010.md) | rules/spec-format.md: PRD supplements live in `.factory/specs/prd-supplements/` (5 named files) | draft | CAP-TBD | TBD |
| [BC-8.24.011](ss-08/BC-8.24.011.md) | rules/spec-format.md: STORY-NNN file format — Epic/Description/AC/BCs/VPs/Tasks/Strategy/Dependencies/Wave | draft | CAP-TBD | TBD |
| [BC-8.24.012](ss-08/BC-8.24.012.md) | rules/spec-format.md: BC retirement requires updating ALL 5 artifacts in same burst | draft | CAP-TBD | TBD |
| [BC-8.25.001](ss-08/BC-8.25.001.md) | rules/step-decomposition.md: VSDD pipeline has 8 phases numbered 0–7 | draft | CAP-TBD | TBD |
| [BC-8.25.002](ss-08/BC-8.25.002.md) | rules/step-decomposition.md: phase numbers are sequential integers; no fractional phases ("3.5") | draft | CAP-TBD | TBD |
| [BC-8.25.003](ss-08/BC-8.25.003.md) | rules/step-decomposition.md: every phase has exactly two skill entry points (work skill + phase entry-point skill) | draft | CAP-TBD | TBD |
| [BC-8.25.004](ss-08/BC-8.25.004.md) | rules/step-decomposition.md: 4-layer orchestration architecture (lobster → phase entry-point skill → phase sub-workflow → step files) | draft | CAP-TBD | TBD |
| [BC-8.25.005](ss-08/BC-8.25.005.md) | rules/step-decomposition.md: step IDs are LOWERCASE ALPHABETIC ONLY — `step-a-`, `step-b-`, `step-c-` | draft | CAP-TBD | TBD |
| [BC-8.25.006](ss-08/BC-8.25.006.md) | rules/step-decomposition.md: step file structure includes `_shared-context.md` + per-step files | draft | CAP-TBD | TBD |
| [BC-8.25.007](ss-08/BC-8.25.007.md) | rules/step-decomposition.md: lobster step `name:` MUST match the step file ID (without `step-` prefix) | draft | CAP-TBD | TBD |
| [BC-8.25.008](ss-08/BC-8.25.008.md) | rules/step-decomposition.md: `_shared-context.md` holds constraints applying to ALL steps in the phase | draft | CAP-TBD | TBD |
| [BC-8.25.009](ss-08/BC-8.25.009.md) | rules/step-decomposition.md: content completeness — no content loss on decomposition | draft | CAP-TBD | TBD |
| [BC-8.25.010](ss-08/BC-8.25.010.md) | rules/step-decomposition.md: phase sub-workflow lobster pattern — step + state-manager backup + phase gate + input-hash drift check + human-approval | draft | CAP-TBD | TBD |
| [BC-8.25.011](ss-08/BC-8.25.011.md) | rules/step-decomposition.md: forbidden practices (no fractional phases / numeric step IDs / sub-step numbering / parent gutting / "see parent" deferrals / wired-less step files / shared-context skip) | draft | CAP-TBD | TBD |
| [BC-8.25.012](ss-08/BC-8.25.012.md) | rules/step-decomposition.md: verification — lobster-parse + path resolution + content completeness + bats + phase-number consistency + grep for old phase numbers | draft | CAP-TBD | TBD |
| [BC-8.26.001](ss-08/BC-8.26.001.md) | rules/story-completeness.md: 14-check audit before marking a story ready for implementation | draft | CAP-014 | S-0.05, S-5.05, S-5.06 |
| [BC-8.26.002](ss-08/BC-8.26.002.md) | rules/story-completeness.md: check 1 — source-of-truth alignment (line-by-line vs architecture docs) | draft | CAP-TBD | TBD |
| [BC-8.26.003](ss-08/BC-8.26.003.md) | rules/story-completeness.md: check 2 — every file in project structure has Deliverable section OR implementation Task | draft | CAP-TBD | TBD |
| [BC-8.26.004](ss-08/BC-8.26.004.md) | rules/story-completeness.md: check 3 — technical gotchas documented in Dev Notes (API quirks, version-specific behavior, platform diffs) | draft | CAP-TBD | TBD |
| [BC-8.26.005](ss-08/BC-8.26.005.md) | rules/story-completeness.md: check 4 — CI/CD workflows complete (workflow YAML deliverables, secrets/branch-protection prerequisites) | draft | CAP-TBD | TBD |
| [BC-8.26.006](ss-08/BC-8.26.006.md) | rules/story-completeness.md: check 5 — README/user-facing-docs deliverable covers what-it-is/install/quickstart/config/CLI/exit-codes/integration/license | draft | CAP-014 | S-5.05, S-5.06 (S-0.05 removed F-003 Wave 8 pass-1) |
| [BC-8.26.007](ss-08/BC-8.26.007.md) | rules/story-completeness.md: check 6 — hosting/infra decisions explicit (org/repo/visibility/branch-strategy/protection/secrets) | draft | CAP-TBD | TBD |
| [BC-8.26.008](ss-08/BC-8.26.008.md) | rules/story-completeness.md: check 7 — license stated explicitly + consistent across 5 surfaces | draft | CAP-TBD | TBD |
| [BC-8.26.009](ss-08/BC-8.26.009.md) | rules/story-completeness.md: check 8 — generated output specified (format/sort-order/edge-cases/exit-codes) | draft | CAP-TBD | TBD |
| [BC-8.26.010](ss-08/BC-8.26.010.md) | rules/story-completeness.md: check 9 — test fixtures defined with directory/config/expected-behavior/AC-coverage | draft | CAP-TBD | TBD |
| [BC-8.26.011](ss-08/BC-8.26.011.md) | rules/story-completeness.md: check 10 — shell/script rules addressed (or `bash.md` excluded if no shell) | draft | CAP-TBD | TBD |
| [BC-8.26.012](ss-08/BC-8.26.012.md) | rules/story-completeness.md: check 11 — `.claude/rules/_index.md` references EXACTLY the rules files that exist | draft | CAP-TBD | TBD |
| [BC-8.26.013](ss-08/BC-8.26.013.md) | rules/story-completeness.md: check 12 — internal consistency (crate names / license text / file paths / config option names / badges / org names match) | draft | CAP-TBD | TBD |
| [BC-8.26.014](ss-08/BC-8.26.014.md) | rules/story-completeness.md: check 13 — project-specific vs generic separation (tool deliverables use generic names; project-specific config separate) | draft | CAP-TBD | TBD |
| [BC-8.26.015](ss-08/BC-8.26.015.md) | rules/story-completeness.md: check 14 — prerequisites listed (manual steps, repo creation, branch protection, secrets, external accounts) | draft | CAP-TBD | TBD |
| [BC-8.26.016](ss-08/BC-8.26.016.md) | rules/story-completeness.md: process — read end-to-end, run each check, fix gaps one at a time with approval, final consistency pass | draft | CAP-TBD | TBD |
| [BC-8.27.001](ss-08/BC-8.27.001.md) | rules/worktree-protocol.md: branch hierarchy — main (releases) > develop (integration) > feature/STORY-NNN-<desc> | draft | CAP-TBD | TBD |
| [BC-8.27.002](ss-08/BC-8.27.002.md) | rules/worktree-protocol.md: story worktrees live in `.worktrees/STORY-NNN/` | draft | CAP-TBD | TBD |
| [BC-8.27.003](ss-08/BC-8.27.003.md) | rules/worktree-protocol.md: worktree creation — `git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN-<desc> develop` | draft | CAP-TBD | TBD |
| [BC-8.27.004](ss-08/BC-8.27.004.md) | rules/worktree-protocol.md: micro-commits per test pass; commit format `feat(STORY-NNN): <desc>` or `test(STORY-NNN): <desc>` | draft | CAP-TBD | TBD |
| [BC-8.27.005](ss-08/BC-8.27.005.md) | rules/worktree-protocol.md: merge protocol — tests pass, PR to develop, adversarial+code review, squash-merge, worktree+branch cleanup | draft | CAP-TBD | TBD |
| [BC-8.27.006](ss-08/BC-8.27.006.md) | rules/worktree-protocol.md: wave integration — full test suite, adversarial review of wave diff, holdout evaluation, wave gate | draft | CAP-TBD | TBD |
| [BC-8.27.007](ss-08/BC-8.27.007.md) | rules/worktree-protocol.md: `.factory/` worktree is PERMANENT — never remove it | draft | CAP-TBD | TBD |
| [BC-8.27.008](ss-08/BC-8.27.008.md) | rules/worktree-protocol.md: cleanup rules — remove worktrees promptly, never force-remove with uncommitted changes, audit via `git worktree list` | draft | CAP-TBD | TBD |
| [BC-8.28.001](ss-08/BC-8.28.001.md) | rules/lessons-codification.md requires codification follow-up for every novel process catch | draft | CAP-001 | S-7.02 |
| [BC-8.28.002](ss-08/BC-8.28.002.md) | orchestrator cycle-closing checklist references lessons-codification.md rule | draft | CAP-001 | S-7.02 |
| [BC-8.30.001](ss-08/BC-8.30.001.md) | story template must include tdd_mode field with strict\|facade enum and strict default | draft | CAP-016 | S-7.03 |
<!-- BC-8.29.001, BC-8.29.002, BC-8.29.003, BC-8.30.002 listed under SS-05 above — authoritative subsystem is SS-05 (files remain in ss-08/ per POLICY 1 append-only) -->

### SS-09 — Configuration and Activation (BC-9)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-9.01.001](ss-09/BC-9.01.001.md) | bump-version.sh accepts semver release format (stable N.N.N + prerelease 1.0.0-beta.N / 1.0.0-rc.N) | draft | CAP-028 | S-0.01, S-2.08, S-0.02, S-4.08, S-5.07 |
| [BC-9.01.002](ss-09/BC-9.01.002.md) | chore commit (operator-staged) modifies only CHANGELOG.md | draft | CAP-028 | S-2.04, S-2.08 |
| [BC-9.01.003](ss-09/BC-9.01.003.md) | release workflow's bot commit atomically writes binaries + plugin.json + marketplace.json | draft | CAP-028 | S-2.04, S-2.08, S-0.02 |
| [BC-9.01.004](ss-09/BC-9.01.004.md) | 5-platform CI matrix is the build matrix; drift gated by check-platforms-drift.py | draft | CAP-007 | S-0.03, S-2.03, S-2.08 |
| [BC-9.01.005](ss-09/BC-9.01.005.md) | hooks.json is gitignored; hooks.json.template + per-platform variants are committed | draft | CAP-007 | S-2.06, S-0.04, S-2.02, S-2.08 |

### SS-10 — CLI Tools and Bin (BC-10)

| BC ID | Title | Status | Capability | Stories |
|-------|-------|--------|-----------|---------|
| [BC-10.01.001](ss-10/BC-10.01.001.md) | compute-input-hash: input-hash drift detection + remediation tool | draft | TBD | TBD |
| [BC-10.01.002](ss-10/BC-10.01.002.md) | compute-input-hash: I/O — argv command form, prints hash to stdout, diagnostics to stderr | draft | TBD | TBD |
| [BC-10.01.003](ss-10/BC-10.01.003.md) | compute-input-hash: input resolution against ARTIFACT_DIR + .factory/ search bases | draft | TBD | TBD |
| [BC-10.01.004](ss-10/BC-10.01.004.md) | compute-input-hash: refuses to hash with missing inputs | draft | TBD | TBD |
| [BC-10.01.005](ss-10/BC-10.01.005.md) | compute-input-hash: exit codes — 0 success/match, 1 usage/missing/scan-update-failed, 2 drift/s | draft | TBD | TBD |
| [BC-10.02.001](ss-10/BC-10.02.001.md) | emit-event: failure-tolerant structured event emitter | draft | TBD | TBD |
| [BC-10.02.002](ss-10/BC-10.02.002.md) | emit-event: I/O — argv key=value pairs become JSON top-level fields; stdin ignored; no stdout/s | draft | TBD | TBD |
| [BC-10.02.003](ss-10/BC-10.02.003.md) | emit-event: auto-injects `ts`, `ts_epoch`, `schema_version=1` | draft | TBD | TBD |
| [BC-10.02.004](ss-10/BC-10.02.004.md) | emit-event: auto-injects session_id from VSDD_SESSION_ID > CLAUDE_SESSION_ID | draft | TBD | TBD |
| [BC-10.02.005](ss-10/BC-10.02.005.md) | emit-event: log-dir resolution — VSDD_LOG_DIR > main-worktree/.factory/logs > cwd/.factory/logs | draft | TBD | TBD |
| [BC-10.02.006](ss-10/BC-10.02.006.md) | emit-event: VSDD_TELEMETRY=off short-circuit (line-1 kill switch) | draft | TBD | TBD |
| [BC-10.02.007](ss-10/BC-10.02.007.md) | emit-event: atomic JSONL append exploits POSIX PIPE_BUF guarantee | draft | TBD | TBD |
| [BC-10.03.001](ss-10/BC-10.03.001.md) | factory-dashboard: live pipeline dashboard markdown renderer | draft | TBD | TBD |
| [BC-10.03.002](ss-10/BC-10.03.002.md) | factory-dashboard: I/O — CLI flags `--days N` (default 7), `--factory PATH` (default ./.factory) | draft | TBD | TBD |
| [BC-10.03.003](ss-10/BC-10.03.003.md) | factory-dashboard: STATE.md size warnings (>500 lines block, >200 lines info) | draft | TBD | TBD |
| [BC-10.03.004](ss-10/BC-10.03.004.md) | factory-dashboard: Health checks section probes existence of key paths | draft | TBD | TBD |
| [BC-10.03.005](ss-10/BC-10.03.005.md) | factory-dashboard: never crashes on missing dependencies (degrades gracefully) | draft | TBD | TBD |
| [BC-10.04.001](ss-10/BC-10.04.001.md) | factory-obs: lifecycle manager for the local observability docker-compose stack | draft | TBD | TBD |
| [BC-10.04.002](ss-10/BC-10.04.002.md) | factory-obs: 9 subcommands (up, regenerate, down, reset, status, logs, dashboard, register, unreg | draft | TBD | TBD |
| [BC-10.04.003](ss-10/BC-10.04.003.md) | factory-obs: registry resolution — VSDD_OBS_REGISTRY > XDG_CONFIG_HOME/vsdd-factory/watched-fac | draft | TBD | TBD |
| [BC-10.04.004](ss-10/BC-10.04.004.md) | factory-obs: docker-compose-safe subdir name = `<basename>-<8-char-shasum>` | draft | TBD | TBD |
| [BC-10.04.005](ss-10/BC-10.04.005.md) | factory-obs: register validates absolute path + .factory/ subdir presence; dedups; seeds header o | draft | TBD | TBD |
| [BC-10.04.006](ss-10/BC-10.04.006.md) | factory-obs: exit-code semantics — 0 success, 1 usage/validation/missing-deps, 127 docker-compo | draft | TBD | TBD |
| [BC-10.05.001](ss-10/BC-10.05.001.md) | factory-query: canned queries against the observability event log | draft | TBD | TBD |
| [BC-10.05.002](ss-10/BC-10.05.002.md) | factory-query: 6 subcommands (top, recent, grep, hooks, stats, reasons, help) | draft | TBD | TBD |
| [BC-10.05.003](ss-10/BC-10.05.003.md) | factory-query: shared flag surface (--days N, --limit N, --severity, --type, --tsv) | draft | TBD | TBD |
| [BC-10.05.004](ss-10/BC-10.05.004.md) | factory-query: portable date helpers handle BSD + GNU date | draft | TBD | TBD |
| [BC-10.05.005](ss-10/BC-10.05.005.md) | factory-query: stats output enumerates total/blocks/warns/actions/unique_reasons/unique_hooks + l | draft | TBD | TBD |
| [BC-10.05.006](ss-10/BC-10.05.006.md) | factory-query: exit codes — 0 normal/empty results, 1 missing jq / unknown flag / unknown subco | draft | TBD | TBD |
| [BC-10.06.001](ss-10/BC-10.06.001.md) | factory-replay: reconstructs a session's hook activity from the event log | draft | TBD | TBD |
| [BC-10.06.002](ss-10/BC-10.06.002.md) | factory-replay: 3 subcommands (sessions, show, latest, help) | draft | TBD | TBD |
| [BC-10.06.003](ss-10/BC-10.06.003.md) | factory-replay: pairing rule — sort by ts_epoch, group by session_id, latest events at top | draft | TBD | TBD |
| [BC-10.06.004](ss-10/BC-10.06.004.md) | factory-replay: render format — `ts  severity  hook  reason  context` | draft | TBD | TBD |
| [BC-10.07.001](ss-10/BC-10.07.001.md) | factory-report: markdown-formatted summary of the observability event log | draft | TBD | TBD |
| [BC-10.07.002](ss-10/BC-10.07.002.md) | factory-report: 3 subcommands (daily, weekly, range, help) | draft | TBD | TBD |
| [BC-10.07.003](ss-10/BC-10.07.003.md) | factory-report: report shape — Summary table + Top reasons + Hook activity + (Wave merges) + (S | draft | TBD | TBD |
| [BC-10.07.004](ss-10/BC-10.07.004.md) | factory-report: portable BSD/GNU days-between calculation | draft | TBD | TBD |
| [BC-10.08.001](ss-10/BC-10.08.001.md) | factory-sla: agent.start/agent.stop pairing for subagent SLA tracking | draft | TBD | TBD |
| [BC-10.08.002](ss-10/BC-10.08.002.md) | factory-sla: 3 subcommands (durations, summary, open, help) | draft | TBD | TBD |
| [BC-10.08.003](ss-10/BC-10.08.003.md) | factory-sla: pairing implemented as O(n) awk stack per (session, subagent) key | draft | TBD | TBD |
| [BC-10.08.004](ss-10/BC-10.08.004.md) | factory-sla: percentile computation in awk (p50/p90/p99 + min/max/mean) | draft | TBD | TBD |
| [BC-10.08.005](ss-10/BC-10.08.005.md) | factory-sla: `open` surfaces orphan starts so silent agent failures are visible | draft | TBD | TBD |
| [BC-10.09.001](ss-10/BC-10.09.001.md) | lobster-parse: thin yq + jq wrapper for .lobster YAML workflow files | draft | TBD | TBD |
| [BC-10.09.002](ss-10/BC-10.09.002.md) | lobster-parse: I/O — argv positional `<file.lobster>` + optional `[jq-expression]` (default `.` | draft | TBD | TBD |
| [BC-10.09.003](ss-10/BC-10.09.003.md) | lobster-parse: missing yq/jq error to stderr + exit 1; missing file = exit 1; missing arg = exit 2 | draft | TBD | TBD |
| [BC-10.10.001](ss-10/BC-10.10.001.md) | multi-repo-scan: detect multi-repo layout under .worktrees/ and emit JSON dependency report | draft | TBD | TBD |
| [BC-10.10.002](ss-10/BC-10.10.002.md) | multi-repo-scan: 3 modes — `json` (default), `--list`, `--count` | draft | TBD | TBD |
| [BC-10.10.003](ss-10/BC-10.10.003.md) | multi-repo-scan: manifest detection priority — Cargo.toml > package.json > pyproject.toml > go | draft | TBD | TBD |
| [BC-10.10.004](ss-10/BC-10.10.004.md) | multi-repo-scan: empty directory = `{"repos": [], "count": 0}` not error | draft | TBD | TBD |
| [BC-10.11.001](ss-10/BC-10.11.001.md) | research-cache: SHA-256-keyed disk cache for research-agent queries | draft | TBD | TBD |
| [BC-10.11.002](ss-10/BC-10.11.002.md) | research-cache: 6 subcommands (get, put, has, key, clear, stats) | draft | TBD | TBD |
| [BC-10.11.003](ss-10/BC-10.11.003.md) | research-cache: query normalization (whitespace-collapse + trim) before hashing | draft | TBD | TBD |
| [BC-10.11.004](ss-10/BC-10.11.004.md) | research-cache: stats output `entries=N bytes=M dir=PATH` | draft | TBD | TBD |
| [BC-10.11.005](ss-10/BC-10.11.005.md) | research-cache: clear is idempotent and removes only `*.json` (not the dir itself) | draft | TBD | TBD |
| [BC-10.12.001](ss-10/BC-10.12.001.md) | wave-state: read-only query against .factory/stories/sprint-state.yaml | draft | TBD | TBD |
| [BC-10.12.002](ss-10/BC-10.12.002.md) | wave-state: 4 subcommands (current, stories, ready, summary) | draft | TBD | TBD |
| [BC-10.12.003](ss-10/BC-10.12.003.md) | wave-state: schema fallback — `.current_wave // .active_wave // 1`; both wave-shapes supported | draft | TBD | TBD |
| [BC-10.12.004](ss-10/BC-10.12.004.md) | wave-state: ready exit code — 0 if all ready, 1 otherwise | draft | TBD | TBD |
