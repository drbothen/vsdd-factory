---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/specs/architecture/SS-01-hook-dispatcher.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.001.md
input-hash: "728da22"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.10.001: Dispatcher exposes vsdd::activated_platform() host function returning activation record platform string

## Description

The dispatcher registers a `vsdd::activated_platform` host function in the linker. When a
plugin calls this function, the dispatcher reads `.claude/settings.local.json` (resolved
relative to the project root) and returns the string value at key
`vsdd-factory.activated_platform` (e.g., `"darwin-arm64"`, `"linux-x86_64"`,
`"windows-x64"`). If the key is absent, the file is unreadable, or the value is not a
string, the host function returns the sentinel `"unknown"`.

This host function exists so that plugins can retrieve the activated platform identifier
without requiring the `read_file` capability and without being granted arbitrary file-read
access. The dispatcher mediates the read and returns only the typed platform string.

## Motivation (Architectural Ruling — S-5.01 Pass-2 Finding F-3)

BC-4.04.001 postcondition 2 requires the session-start plugin to include `activated_platform`
in the `session.started` payload. Two options were evaluated:

- **Option A — Capability extension:** Add `read_file` to BC-4.04.005 capabilities for
  `.claude/settings.local.json`. Rejected: grants the plugin arbitrary file-read at a
  well-known secrets-adjacent path; violates the principle of minimum capability surface.
- **Option B — New host-fn getter (this BC):** Expose a typed, narrow host function that
  reads the activation record and returns only the platform string. Accepted: the dispatcher
  mediates the read; the plugin receives a typed scalar; no `read_file` capability needed.

## Preconditions

1. The dispatcher has loaded and initialized the wasmtime linker for the WASM plugin.
2. The dispatcher has access to the project root (CLAUDE_PROJECT_DIR) where
   `.claude/settings.local.json` is expected.

## Postconditions

1. `linker.get(&mut store, "vsdd", "activated_platform")` succeeds — the function is
   registered in the vsdd namespace alongside existing context getters.
2. When a plugin calls `vsdd::activated_platform(out_ptr, out_len)`:
   a. The dispatcher reads `.claude/settings.local.json` (relative to CLAUDE_PROJECT_DIR).
   b. Parses the JSON and extracts the string at key `vsdd-factory.activated_platform`.
   c. Writes the platform string bytes into the plugin's memory at `out_ptr`, up to
      `out_len` bytes.
   d. Returns the number of bytes written (or the required buffer size if `out_len == 0`).
3. If the file is missing, unreadable, unparseable, or the key is absent/not-a-string,
   the host function writes `"unknown"` and returns 7.
4. The host function does NOT require the plugin to declare `read_file` in its
   `hooks-registry.toml` capabilities — the read is performed by the host on the plugin's
   behalf and is mediated by this function's narrow contract.

## Invariants

1. The `activated_platform` host function is registered in the vsdd namespace alongside
   `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`, and `env`
   (per BC-1.05.033 vsdd-namespace import surface completeness).
2. The returned value is always a valid UTF-8 string. Invalid UTF-8 in
   settings.local.json causes the fallback `"unknown"` to be returned.
3. The host function never panics or traps — all error conditions map to the `"unknown"`
   sentinel return path.
4. The read is performed at call time (not cached across invocations) so that the
   dispatcher reflects any settings changes between dispatcher restarts.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `.claude/settings.local.json` does not exist | Returns `"unknown"` |
| EC-002 | File exists but `vsdd-factory.activated_platform` key is absent | Returns `"unknown"` |
| EC-003 | `vsdd-factory.activated_platform` value is not a string (e.g., integer, object) | Returns `"unknown"` |
| EC-004 | `out_len == 0` (size probe) | Returns required byte count; no bytes written |
| EC-005 | Platform string is longer than `out_len` (truncation case) | Writes `out_len` bytes; returns full string length (caller should size-probe first) |
| EC-006 | Plugin has not declared any special capability | Host function succeeds — no capability declaration required for this getter |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid `settings.local.json` with `vsdd-factory.activated_platform = "darwin-arm64"` | Returns `"darwin-arm64"` | happy-path |
| `.claude/settings.local.json` absent | Returns `"unknown"` | error |
| `vsdd-factory.activated_platform` key missing from valid JSON | Returns `"unknown"` | error |
| `out_len == 0`, platform is `"darwin-arm64"` (12 bytes) | Returns `12`; no memory written | edge-case (size probe) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in next verification pass) | | |

## Related BCs

- **BC-1.05.010** — parallel (context getters pattern; `activated_platform` follows the same
  second-call-protocol as `session_id`, `cwd`, etc.)
- **BC-1.05.033** — extends (vsdd-namespace import surface must include `activated_platform`)
- **BC-4.04.001** — consumed by (session-start plugin reads `activated_platform` via this host fn
  to populate the `session.started` payload field; see postcondition 2 of BC-4.04.001)
- **BC-4.04.005** — related (no `read_file` capability required in hooks-registry.toml because
  this host fn mediates the read)

## Architecture Anchors

- SS-01 — `crates/factory-dispatcher/src/host/` (new `activated_platform_fn` registration
  alongside existing context getters in `context_fns.rs` or equivalent)
- SS-09 — `.claude/settings.local.json` (activation record written by the activation skill;
  `vsdd-factory.activated_platform` key is the canonical source of truth per BC-6.01.005)

## Story Anchor

S-5.01

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") — this host fn is part of the plugin ABI surface that enables sandboxed WASM execution |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — this host fn is specifically designed to avoid requiring `read_file` capability) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/context_fns.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
