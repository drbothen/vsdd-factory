# AC-008: host::emit_event replaces bin/emit-event; bare statement form; bin/emit-event preserved

**BC trace:** BC-7.04.040 postcondition 1 (emit_event host fn)
**Status:** PASS

## What was verified

### host::emit_event used; no bin/emit-event reference

`grep -rn 'bin/emit-event' crates/hook-plugins/validate-pr-review-posted/` returns no results.
The crate uses `vsdd_hook_sdk::host::emit_event` exclusively via the SDK's host function ABI.

### Bare statement form (AC-008 requirement)

```rust
vsdd_hook_sdk::host::emit_event(
    "hook.block",
    &[
        ("hook", "validate-pr-review-posted"),
        ("matcher", "SubagentStop"),
        ("reason", "pr_review_not_posted"),
        ("subagent", agent),
    ],
);
```

The function signature is `pub fn emit_event(event_type: &str, fields: &[(&str, &str)]) -> ()`.
Returns unit `()` — there is no Result to discard. `let _ = ...` form is NOT used.
This matches the `capture-commit-activity` sibling pattern (`src/main.rs:28-37`).

### bin/emit-event binary preserved

`ls -la bin/emit-event` confirms the binary still exists. Removal is deferred to S-8.29 (E-8 D-10).
Other hooks may still depend on `bin/emit-event` — removing it is coordinated at E-8 closure.

### HOST_ABI_VERSION unchanged

No new host functions were added. `validate-pr-review-posted` uses only `emit_event` + stdin + stderr.
HOST_ABI_VERSION = 1 preserved (BC-2.01.003 / ADR-006).

## Recording

[AC-008-emit-event-host-fn.gif](AC-008-emit-event-host-fn.gif)
