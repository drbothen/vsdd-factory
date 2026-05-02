# vsdd-factory Host ABI Reference

This document describes the runtime contract between WASM hook plugins
and the `factory-dispatcher` host. Plugin authors who use `vsdd-hook-sdk`
do not call this ABI directly — the SDK's [`host`](src/host.rs) module
wraps every function in safe, ergonomic Rust. This file exists for:

- ABI version negotiation (`HOST_ABI_VERSION`)
- Implementers writing alternate-language SDKs (Go, AssemblyScript, etc.)
- Forensic debugging when a plugin / dispatcher pairing misbehaves

The ABI is **stable as of `HOST_ABI_VERSION = 1`**. Breaking changes
require a major bump on both `vsdd-hook-sdk` and `factory-dispatcher`
per the semver commitment (S-5.6).

---

## Negotiation

Every plugin exports a constant `HOST_ABI_VERSION: u32`. The dispatcher
reads it before any host call and refuses to invoke plugins whose
version it does not understand. SDK 0.0.x → ABI 1; SDK 1.x → ABI 1;
SDK 2.x → ABI 2 (when it lands).

The dispatcher's compile-time `HOST_ABI_VERSION` constant is the source
of truth for what it speaks. The SDK's `vsdd_hook_sdk::HOST_ABI_VERSION`
constant is the source of truth for what plugins built against that SDK
expect.

---

## Plugin entry point

WASI preview-1 command convention. The dispatcher invokes the
auto-generated `_start` function. The `#[hook]` macro emits a `fn main()`
that drives the user function:

1. Reads stdin to a `Vec<u8>`.
2. Deserializes JSON into `HookPayload`.
3. Calls the user's `fn(HookPayload) -> HookResult` with a
   `std::panic::catch_unwind` boundary so a panicking plugin terminates
   cleanly rather than aborting the wasmtime store.
4. Serializes the `HookResult` JSON to stdout.
5. Calls `std::process::exit` with the result's exit code.

### Stdin envelope (host → plugin)

UTF-8 JSON. Schema:

```json
{
  "event_name": "PreToolUse",
  "tool_name": "Bash",
  "session_id": "abc-123",
  "dispatcher_trace_id": "uuidv4",
  "tool_input": { "...": "tool-specific" },
  "tool_response": null
}
```

`tool_response` is `null` for `PreToolUse` and most pre-call events;
present for `PostToolUse` and lifecycle events that carry a result.

### SubagentStop stdin envelope

When Claude Code fires a `SubagentStop` event the dispatcher writes the
following shape to plugin stdin.  The four SubagentStop-specific fields
(`agent_type`, `subagent_name`, `last_assistant_message`, `result`) are
**top-level** — not nested inside `tool_input` or `tool_response`.

```json
{
  "event_name": "SubagentStop",
  "session_id": "abc-123",
  "dispatcher_trace_id": "uuidv4",
  "tool_name": "",
  "tool_input": null,
  "tool_response": null,
  "agent_type": "product-owner",
  "subagent_name": "product-owner-fallback",
  "last_assistant_message": "Wrote the story spec and committed.",
  "result": "fallback-result-string"
}
```

**Field presence semantics**

| Field | Type in `HookPayload` | Present on | Absent behaviour |
|---|---|---|---|
| `agent_type` | `Option<String>` | SubagentStop only | `None` (via `#[serde(default)]`) |
| `subagent_name` | `Option<String>` | SubagentStop only | `None` |
| `last_assistant_message` | `Option<String>` | SubagentStop only | `None` |
| `result` | `Option<String>` | SubagentStop only | `None` |

JSON `null` deserializes to `None` for all four fields — identical to jq's
`//` null-advance semantics.

**Canonical fallback chains (BC-2.02.012 Postconditions 5–6)**

```rust
// Agent identity — mirrors: jq -r '.agent_type // .subagent_name // "unknown"'
let agent = payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");

// Assistant message — mirrors: jq -r '.last_assistant_message // .result // empty'
let message = payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");
```

Both chains are normative.  Plugin implementations for S-8.01, S-8.02,
S-8.03, and S-8.05 MUST use these expressions; divergence requires an
explicit rationale (see BC-2.02.012 EC-004 for the `handoff-validator`
three-stage `output` fallback).

**BC reference:** BC-2.02.012 — "HookPayload SubagentStop fields: top-level
envelope schema for agent_type, subagent_name, last_assistant_message,
result."

**ABI version:** These fields are an additive `HookPayload` extension
under D-6 Option A and D-183.  `HOST_ABI_VERSION` remains `1`.

---

### Stdout envelope (plugin → host)

UTF-8 JSON, single-line, terminated with `\n`. One of:

```json
{ "outcome": "continue" }
{ "outcome": "block",    "reason":  "<short string>" }
{ "outcome": "error",    "message": "<diagnostic>" }
```

### Exit code contract

| `outcome` | Exit code | Meaning to dispatcher |
|---|---|---|
| `continue` | `0` | Allow the tool call / non-blocking event |
| `block`    | `2` | Block (PreToolUse / PermissionRequest only) |
| `error`    | `1` | Plugin failed; non-blocking unless `on_error = "block"` |

---

## Host functions

All host functions are imported under the WASI module name `vsdd`.
Strings are passed as `(ptr: u32, len: u32)` UTF-8 byte tuples; out-strings
follow the `(out_ptr: u32, out_cap: u32) -> u32 bytes_written` convention.
Negative return codes signal errors:

| Code | Meaning |
|---:|---|
| `-1` | Capability denied (caller lacks the registry-declared capability) |
| `-2` | Timeout exceeded the call's `timeout_ms` |
| `-3` | Output exceeded `max_output_bytes`; truncated |
| `-4` | Invalid argument (path traversal, unknown env name, etc.) |
| Other | Reserved for future categorized errors |

### `log(level, msg_ptr, msg_len) -> ()`

Write `msg` (UTF-8) to the dispatcher's internal log at `level`.

| Level | Value |
|---|---|
| Trace | 0 |
| Debug | 1 |
| Info  | 2 |
| Warn  | 3 |
| Error | 4 |

Always succeeds. No return value, no error path.

### `emit_event(type_ptr, type_len, fields_ptr, fields_len) -> ()`

Emit a structured event. `type` is the event name (e.g. `"commit.made"`).
`fields` is a length-prefixed sequence of UTF-8 key/value pairs:

```
[ key_len: u32 LE | key: u8 × key_len | value_len: u32 LE | value: u8 × value_len ]+
```

The dispatcher enriches the event with `dispatcher_trace_id`,
`session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, and
`schema_version` automatically.

### `read_file(path_ptr, path_len, max_bytes, timeout_ms, out_ptr_out, out_len_out) -> i32`

Read a file under the dispatcher's read allow-list. The host writes the
returned buffer's `(ptr, len)` into the two out-params; the SDK copies
the bytes and surfaces them to the plugin.

`max_bytes` and `timeout_ms` are mandatory. Return: `0` on success or a
negative error code.

### `exec_subprocess(cmd_ptr, cmd_len, args_ptr, args_len, stdin_ptr, stdin_len, timeout_ms, max_output_bytes, result_buf_ptr, result_buf_cap) -> i32`

Run a subprocess against the dispatcher's binary allow-list. `cmd` is the
basename or absolute path; `args` is a length-prefixed sequence of
arguments (same encoding as `emit_event`'s fields, key-only).

`(stdin_ptr, stdin_len)` is an optional payload written to the
subprocess's stdin. Pass `(_, 0)` for no stdin (the host wires
`Stdio::null()` and skips the write). The legacy-bash-adapter (S-2.1)
uses this to forward the Claude Code hook envelope unchanged to bash
hooks.

`(result_buf_ptr, result_buf_cap)` points to a guest-pre-allocated
buffer. The host writes the result envelope into it:

```
[ exit_code: i32 LE | stdout_len: u32 LE | stdout: u8 × stdout_len | stderr_len: u32 LE | stderr: u8 × stderr_len ]
```

Callers should size the buffer at `max_output_bytes + 16` (12 bytes of
length headers + 4 bytes slack). The SDK wrapper does this automatically.

Return:

- `> 0` — number of bytes written to the result buffer (the SDK reads
  exactly that many bytes and decodes the envelope).
- `0` — never returned in practice; the envelope always carries at
  least the exit-code header.
- `< 0` — error code: `-1` capability denied, `-2` timeout, `-3` output
  larger than `max_output_bytes` (or buffer too small), `-4` invalid
  argument.

`timeout_ms` and `max_output_bytes` are mandatory.

### `session_id(out_ptr, out_cap) -> u32`
### `dispatcher_trace_id(out_ptr, out_cap) -> u32`
### `plugin_root(out_ptr, out_cap) -> u32`
### `plugin_version(out_ptr, out_cap) -> u32`
### `cwd(out_ptr, out_cap) -> u32`

Retrieve a fixed string from the host. The host writes UTF-8 bytes into
`out_ptr` (up to `out_cap`) and returns the number of bytes written. If
the value is longer than `out_cap`, the host returns the **required
capacity** without writing partial data; the SDK re-calls with a larger
buffer.

### `env(name_ptr, name_len, out_ptr, out_cap) -> i32`

Read a single environment variable. Returns:

- `>= 0` — number of bytes written (`0` = variable unset)
- `< 0`  — error code (typically `-1` / capability denied if the name is
  not on the dispatcher's env allow-list)

### `write_file(path_ptr, path_len, contents_ptr, contents_len, max_bytes, timeout_ms) -> i32`

Write a guest-owned byte slice to the filesystem through the dispatcher's
bounded host function (BC-2.02.011 — additive ABI extension, D-6 Option A;
`HOST_ABI_VERSION` stays at 1).

**Protocol:** input-pointer — the SDK passes guest-owned bytes;
the dispatcher copies them via `read_wasm_bytes`. This is the **inverse**
of `read_file`'s output-pointer protocol and the two must not be confused.

**Parameters:**

| Parameter | Type | Description |
|---|---|---|
| `path_ptr` | `u32` | Pointer to UTF-8 path in guest memory |
| `path_len` | `u32` | Byte length of path |
| `contents_ptr` | `u32` | Pointer to content bytes in guest memory |
| `contents_len` | `u32` | Byte length of content |
| `max_bytes` | `u32` | Mandatory byte cap; content exceeding this returns `-3` |
| `timeout_ms` | `u32` | Mandatory timeout budget; accepted for ABI stability (epoch interruption enforced in S-1.5) |

**Return values:**

| Code | Meaning |
|---|---|
| `0` | Success; full byte slice durably written to `path` |
| `-1` | Capability denied: path not in `capabilities.write_file.path_allow`, path traversal attempt, or no `write_file` capability block present |
| `-2` | Timeout exceeded `timeout_ms` |
| `-3` | Content length exceeded `max_bytes` cap; **no bytes written to disk** |
| `-4` | Invalid argument (e.g. UTF-8 path decoding failure) |
| `-99` | Filesystem I/O error or missing parent directory |

**Safety policy:** path must be within the plugin's declared
`capabilities.write_file.path_allow` list. Traversal attempts (`..`) return
`-1` (same as `read_file`). Deny-by-default: no capability block → `-1`.

---

## Future ABI versions

**ABI 2** (post-1.0, tentative) is expected to land alongside the WASI
preview-2 / component-model migration:

- Component-model exports replace `_start`
- Strings as Component Model `string` instead of byte tuples
- Async host functions where it makes sense

ABI 2 will not be back-compatible with ABI 1 plugins; the dispatcher
will load both during a transition window per the semver commitment.
