# vsdd-factory v1.0 semver commitment

This document states what vsdd-factory commits to as stable public API in the
v1.0 line, what remains intentionally unstable, how breaking changes are handled,
and what HOST_ABI_VERSION = 1 means for plugin authors. It is the authoritative
reference for operators and plugin authors evaluating upgrade safety.

This document's lock target is **1.0.0 GA**. Pre-GA release candidates may
revise it without a doc-version bump; once 1.0.0 ships, amendments require
a new doc version and a changelog entry.

---

## What's stable in v1.0

The following surfaces are stable for the lifetime of v1.0.x. No change to these
surfaces may occur without a major version bump (see "Breaking change policy"
below).

| Surface | Stability | Reference |
|---------|-----------|-----------|
| hook-sdk ABI (`vsdd_hook_sdk::host::*`) | Stable | BC-2.02.001; PRD §3.1 Plugin ABI |
| Registry schema (`hooks-registry.toml`, `schema_version = 1`) | Stable | PRD §3.1 Dispatcher invocation interface |
| `hooks.json` format (Claude Code hook registration shape) | Stable | PRD §3.1; BC-4.04.004, BC-4.05.004 |
| Event type namespaces (the 17 event-type constants, `vsdd::` host import module) | Stable | PRD §3.1 Plugin ABI; PRD §4.1 Observability |
| `HOST_ABI_VERSION = 1` (both `factory-dispatcher` and `vsdd_hook_sdk`) | Stable | BC-2.01.003 |
| Plugin entry point signature (`__hook_entry(ptr: i32, len: i32) -> i32`) | Stable | PRD §3.1 Plugin ABI |
| `HookResult` output variants (`Continue`, `Block`, `Error`) | Stable | BC-2.01.001–004 |
| Observability event schema version (`INTERNAL_EVENT_SCHEMA_VERSION = 1`) | Stable | PRD §3.1 Observability output interface |
| Dispatcher exit codes (0 = continue, 2 = block) | Stable | PRD §3.1 Dispatcher invocation interface |
| `observability-config.toml` `schema_version = 1` | Stable | PRD §3.1 |
| Skill invocation interface (`/vsdd-factory:<skill-name>`) | Stable | PRD §3.1 Skill invocation interface |

### hook-sdk ABI detail

The public plugin-author API surface is `vsdd_hook_sdk::host::*`. Stable
re-exports include:

- `host::log`, `host::log_info`, `host::log_warn`, `host::log_error`
- `host::emit_event`
- Context getters: `host::session_id()`, `host::tool_name()`, `host::plugin_root()`
- `host::env`, `host::read_file`, `host::exec_subprocess`
- Types: `LogLevel`, `HostError`, `SubprocessResult`

The raw `ffi` module is private (`mod ffi;`, not `pub mod ffi;`) — see BC-2.02.001.
Plugin authors who reach into FFI directly bypass the type-safe wrappers and are
not covered by this stability commitment.

### Registry schema detail

`hooks-registry.toml` is the dispatcher's hook routing table. The `[[hooks]]`
entry schema, `schema_version = 1`, and all top-level fields are stable. The
file is the human-edited source of truth after 1.0.0 ships (the generator that
bootstrapped it from the 0.79.x `hooks.json` inventory is retired at GA).

### hooks.json format detail

`hooks.json` (the Claude Code hook registration file) uses the Claude Code hook
format with `command`, `once`, `async`, and `timeout` fields per event registration.
The per-platform variants (`hooks.json.darwin-arm64`, etc.) follow the same schema.
The format is stable; Claude Code reads it, not vsdd-factory, so format breaks
would originate upstream — but vsdd-factory guarantees it will not write
non-conforming `hooks.json` content.

> **Operator note:** Operators edit `hooks-registry.toml`, not `hooks.json`
> directly. `hooks.json` is a generated artifact bundled into the plugin
> package; the canonical, human-edited source for hook routing is
> `plugins/vsdd-factory/hooks-registry.toml`.

### Event type namespaces detail

The 17 event-type constants used in dispatcher telemetry and plugin invocation
(e.g., `PreToolUse`, `PostToolUse`, `CommitMade`, `PrCreated`, `SessionStart`,
`SessionEnd`, etc.) are stable identifiers. The `vsdd` host import module namespace
is also stable — plugins import host functions as `(import "vsdd" "log" ...)`.

---

## What's NOT stable in v1.0

The following surfaces are internal implementation details and may change across
minor or patch releases without notice. Do not build integrations that depend on
them.

| Surface | Stability | Notes |
|---------|-----------|-------|
| Internal JSONL format (`dispatcher-internal-*.jsonl` field layout) | Unstable | Field names, nesting depth, and optional fields may change in any release. Use the public OTel / file-sink output for downstream integrations. |
| Dispatcher invocation args (CLI flags passed by Claude Code to the dispatcher binary) | Unstable | The dispatcher is invoked by Claude Code, not by operators directly. Flag names, order, and semantics are internal to the dispatcher–Claude Code contract and may change without notice. |
| `hooks-registry.toml` generator script (`scripts/generate-registry-from-hooks-json.sh`) | Unstable | Retired at 1.0.0 GA. Do not script against it. |
| Skill implementation internals (code inside `plugins/vsdd-factory/skills/`) | Unstable | The skill invocation interface (`/vsdd-factory:<name>`) is stable; what each skill does internally is not. |
| `StoreData` / wasmtime host context struct layout | Unstable | Crates internal to the dispatcher. Not part of the public ABI. |
| `dispatcher-internal-*.jsonl` rotation and naming pattern | Unstable | Daily rotation and the `YYYY-MM-DD` suffix are current behavior; path and naming may change. |
| Rust crate-internal module layout (non-`host::*` SDK modules) | Unstable | Only `vsdd_hook_sdk::host::*` is the stable plugin-author surface. |
| `hooks.json.template` intermediate format | Unstable | The `.template` file is a build artifact used by the activation skill. Operators receive the resolved `hooks.json`; the template format is internal. |

### Why these are unstable

The internal JSONL format and dispatcher invocation args are controlled by the
interaction between the dispatcher binary and Claude Code. Both may evolve as
Claude Code upgrades its hook execution model. vsdd-factory tracks these changes
internally and shields plugin authors from them through the stable `host::*` SDK
surface.

---

## Breaking change policy

A **breaking change** is any change to a stable surface listed above that:

- Removes or renames a stable API, type, constant, or file format field
- Changes the semantics of an existing stable API in a backward-incompatible way
- Changes `HOST_ABI_VERSION` in either `factory-dispatcher` or `vsdd_hook_sdk`
- Changes `schema_version` on `hooks-registry.toml` or `observability-config.toml`

### Rules

1. **Major version bump required.** No breaking change to any stable surface may
   ship without incrementing the major version (e.g., `1.x.y` → `2.0.0`).

2. **Migration guide required.** Every major version release MUST include a
   migration guide documenting what changed, why it changed, the step-by-step
   upgrade procedure, and any rollback steps. See
   [Migrating from 0.79.x](migrating-from-0.79.md) as the model for format and
   voice.

3. **Coordinated bump required for ABI changes.** A change to `HOST_ABI_VERSION`
   in the dispatcher must be matched by a simultaneous bump in `vsdd_hook_sdk`
   (BC-2.01.003, EC-001). A mismatch between the two constants is a major-version
   event and would block the release gate.

4. **Deprecation before removal.** Stable surfaces are deprecated in a minor
   release (with a `#[deprecated]` annotation and/or a doc notice) before removal
   in the next major version. Exception: security fixes may require immediate
   removal.

5. **No breaking changes in patch releases.** Patch releases (`x.y.Z`) are
   reserved for bug fixes and security patches. Non-breaking additions may ship
   in minor releases (`x.Y.0`).

### What is not a breaking change

- Adding a new event type to the 17-event namespace (additive)
- Adding a new host function to `vsdd_hook_sdk::host::*` (additive)
- Adding new optional fields to `hooks-registry.toml` or `observability-config.toml` (all config files use `deny_unknown_fields = false` on extensions; existing fields remain stable)
- Bug fixes that restore behavior to match the behavioral contract specification
- Changes to unstable surfaces listed above

---

## Plugin backward compat policy

This section covers what plugin authors can rely on across v1.0.x patch and minor
releases.

### HOST_ABI_VERSION = 1 guarantee

`HOST_ABI_VERSION = 1` is frozen for the entire v1.0 line. This means:

- `factory_dispatcher::HOST_ABI_VERSION == 1` (BC-2.01.003, Postcondition 1)
- `vsdd_hook_sdk::HOST_ABI_VERSION == 1` (BC-2.01.003, Postcondition 2)
- Both constants are equal as a release-gating invariant (BC-2.01.003, Invariant 1)

A plugin compiled against `vsdd_hook_sdk` v1.x will load and run correctly against
any `factory-dispatcher` binary in the v1.0 line without recompilation. This is
the core promise of host ABI version gating.

A change to `HOST_ABI_VERSION` — e.g., bumping dispatcher to `2` without a
corresponding SDK bump — is classified as a major-version mismatch and is a
release-blocking invariant violation (BC-2.01.003, EC-001).

### What plugin authors can assume

| Assumption | Stable? | Notes |
|------------|---------|-------|
| `vsdd_hook_sdk::host::*` public API surface | Yes | BC-2.02.001 |
| `HOST_ABI_VERSION = 1` for all v1.x.y | Yes | BC-2.01.003 |
| `__hook_entry(ptr: i32, len: i32) -> i32` entry point | Yes | PRD §3.1 |
| `HookResult` variants (`Continue`, `Block`, `Error`) | Yes | BC-2.01.001–004 |
| `ffi` module remaining private | Yes | BC-2.02.001, Postcondition 2 |
| Payload format (JSON `HookPayload` + `plugin_config`) | Yes | PRD §3.1 |
| Capability deny-by-default behavior | Yes | NFR-SEC-001 |

### Recompilation requirements

Plugin authors do NOT need to recompile for:
- Dispatcher patch releases (`1.0.Z → 1.0.Z+1`)
- Dispatcher minor releases that add new host functions (plugins that do not call
  the new functions are unaffected)

Plugin authors MUST recompile for:
- A `HOST_ABI_VERSION` bump (major version event — see above)
- Changes to the `HookPayload` JSON schema that affect fields the plugin reads

### Plugin SDK upgrade path

When the SDK publishes a minor release with new `host::*` functions:
- Existing plugins compiled against the older SDK continue to work (ABI is additive)
- To use new functions, recompile against the new SDK version

---

## References

| Document | Purpose |
|----------|---------|
| [BC-2.01.003](.factory/specs/behavioral-contracts/ss-02/BC-2.01.003.md) | HOST_ABI_VERSION = 1 in both crates — the authoritative contract |
| [BC-2.02.001](.factory/specs/behavioral-contracts/ss-02/BC-2.02.001.md) | Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private |
| [Migrating from 0.79.x](migrating-from-0.79.md) | Model migration guide — format and voice for future major-version migration guides |
| [PRD §3.1](../../.factory/specs/PRD.md) | Primary interfaces summary — canonical enumeration of all stable surfaces |
| [v1.0 index](v1.0-index.md) | Landing page for all v1.0 documentation |
