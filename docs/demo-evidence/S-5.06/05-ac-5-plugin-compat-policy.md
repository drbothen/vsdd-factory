# AC-5: Section — Plugin backward compat policy (host ABI version guarantees)

**AC statement:** Section: Plugin backward compat policy (host ABI version guarantees).
Cite BC-2.01.003.

**Evidence type:** section heading grep + verbatim HOST_ABI_VERSION guarantee snippet

## Verification command

```
grep -n "Plugin backward\|HOST_ABI_VERSION\|BC-2.01.003" \
  /private/tmp/vsdd-S-5.06/docs/guide/semver-commitment.md
```

**Result:**
```
144:## Plugin backward compat policy
149:### HOST_ABI_VERSION = 1 guarantee
151:`HOST_ABI_VERSION = 1` is frozen for the entire v1.0 line. This means:
153:- `factory_dispatcher::HOST_ABI_VERSION == 1` (BC-2.01.003, Postcondition 1)
154:- `vsdd_hook_sdk::HOST_ABI_VERSION == 1` (BC-2.01.003, Postcondition 2)
155:- Both constants are equal as a release-gating invariant (BC-2.01.003, Invariant 1)
163:A change to `HOST_ABI_VERSION` — e.g., bumping dispatcher to `2` without a
165:release-blocking invariant violation (BC-2.01.003, EC-001).
```

## Verbatim snippet — HOST_ABI_VERSION guarantee (L144-L165)

```markdown
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
```

## Verbatim snippet — plugin author assumptions table (L167-L176)

```markdown
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
```

## BC-2.01.003 citation confirmed

BC-2.01.003 is cited 5 times in the plugin compat section alone (lines 153, 154, 155, 165,
and in the table). It is the authoritative contract for `HOST_ABI_VERSION = 1` in both
`factory-dispatcher` and `vsdd_hook_sdk`.

## Commentary

The section covers the complete plugin compatibility guarantee: what is frozen (HOST_ABI_VERSION),
what plugin authors can assume across all v1.x.y releases, and recompilation requirements.
The "recompilation requirements" subsection explicitly enumerates when plugin authors do and
do not need to recompile — a critical practical guide for the "upgrade safety" use case
described in the story narrative.
