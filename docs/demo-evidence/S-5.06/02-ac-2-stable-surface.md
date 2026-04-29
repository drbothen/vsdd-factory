# AC-2: Section — What's stable (hook-sdk ABI, registry schema, hooks.json format, event type namespaces)

**AC statement:** Section: What's stable (hook-sdk ABI, registry schema, hooks.json format,
event type namespaces).

**Evidence type:** section heading grep + verbatim table snippet

## Verification command

```
grep -n "What's stable" /private/tmp/vsdd-S-5.06/docs/guide/semver-commitment.md
```

**Result:**
```
13:## What's stable in v1.0
```

## Verbatim snippet — stable surface table (L13-L31)

```markdown
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
```

## All four required surfaces confirmed

| Required surface | Present? | File line |
|-----------------|---------|-----------|
| hook-sdk ABI (`vsdd_hook_sdk::host::*`) | Yes | L21 |
| Registry schema (`hooks-registry.toml`, `schema_version = 1`) | Yes | L22 |
| `hooks.json` format | Yes | L23 |
| Event type namespaces (17 event-type constants) | Yes | L24 |

## Commentary

The section enumerates 11 stable surfaces total, a superset of the 4 required by the AC.
Each row cites a behavioral contract or PRD reference, linking the stability commitment
back to verifiable specification artifacts.
