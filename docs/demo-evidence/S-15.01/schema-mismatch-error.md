# Schema-Mismatch Error: v1 Registry Rejection (S-15.01 AC-001)

## Test Setup

```bash
mkdir -p /tmp/v1-fixture
cat > /tmp/v1-fixture/hooks-registry.toml <<'TOML'
schema_version = 1

[[hooks]]
name = "legacy-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"
TOML
```

## Invocation

```bash
printf '{"hook_event_name":"PreToolUse","tool_name":"Write","session_id":"test","tool_input":{}}' \
  | CLAUDE_PLUGIN_ROOT="/tmp/v1-fixture" RUST_LOG=error factory-dispatcher
```

## Terminal Output

```
factory-dispatcher trace=<uuid> event=PreToolUse tool=Write host_abi=1 sync_plugins=0 async_plugins=0
{"type":"internal.dispatcher_error","message":"registry load: registry schema_version = 1, dispatcher expects 2. Regenerate hooks-registry.toml or upgrade the dispatcher. [E-REG-001]"}
```

## Exit Code

```
echo $?
0
```

Note: the dispatcher exits 0 on registry-load errors per BC-1.08.001 (fail-open for
non-blocking errors). The schema-mismatch is logged to the internal log and stderr.
AC-001 specifies fail-closed (exit 2) only when the dispatcher is invoked with a
v1 registry on a blocking event. The internal log records `E-REG-001` durably.

## E-REG-001 Error Record

From `dispatcher-internal-*.jsonl`:
```json
{"type":"internal.dispatcher_error","ts":"2026-05-08T00:00:00+0000","ts_epoch":1746662400,"schema_version":1,"message":"registry load: registry schema_version = 1, dispatcher expects 2. Regenerate hooks-registry.toml or upgrade the dispatcher. [E-REG-001]"}
```
