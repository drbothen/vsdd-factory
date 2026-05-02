# Bonus Evidence — VSDD_SINK_FILE: Workspace-Shared Integration Testing Improvement

**Story:** S-8.08 — Native port: track-agent-start (PreToolUse:Agent)
**Component:** `crates/factory-dispatcher/src/main.rs`
**Scope:** Cross-story infrastructure improvement enabling bats integration testing for all future hook stories
**Mutation Testing:** CC-W15-002 `mutation_testing_required: true`

## What Was Added

The `VSDD_SINK_FILE` environment variable was added to `crates/factory-dispatcher/src/main.rs`
as a test/development hook that routes all plugin-emitted events to a JSONL file after
dispatcher execution completes. This is a workspace-shared capability — it is not specific
to S-8.08, and all future hook story bats integration tests can use the same mechanism.

### Code Location

```
crates/factory-dispatcher/src/main.rs
  Line 21–27:  Module-level doc comment explaining VSDD_SINK_FILE semantics
  Line 49:     const ENV_SINK_FILE: &str = "VSDD_SINK_FILE";
  Line 205–213: flush_sink_file invocation in run() fn
  Line 239–292: flush_sink_file() implementation
```

### Implementation

```rust
/// When set, plugin-emitted events are appended as JSONL to this path.
/// Used by bats integration tests (S-8.08 AC-005). Best-effort.
const ENV_SINK_FILE: &str = "VSDD_SINK_FILE";

// In run() after execute_tiers() completes:
if let Ok(sink_path) = std::env::var(ENV_SINK_FILE) {
    if !sink_path.is_empty() {
        flush_sink_file(&sink_path, &event_queue);
    }
}
```

### flush_sink_file Semantics

The `flush_sink_file` function (lines 239–292):

1. **Drains** the plugin event queue via mutex lock + `mem::take`
2. **Filters** to plugin-domain events only — excludes `dispatcher.*`, `internal.*`,
   and `plugin.*` lifecycle events. Only hook-domain events (e.g., `agent.start`)
   are written to the sink. This matches what bats tests expect: they assert on
   telemetry content, not dispatcher plumbing.
3. **Appends** each event as a JSONL line (one JSON object per line) to the sink file
4. **Best-effort**: any I/O or serialization error is silently swallowed. The dispatcher
   always exits 0 on non-block dispatches regardless of sink write outcome.

```rust
fn flush_sink_file(sink_path: &str, event_queue: &Arc<Mutex<Vec<InternalEvent>>>) {
    // Filter to plugin-domain events (exclude dispatcher/internal/plugin lifecycle)
    let domain_events: Vec<_> = events.iter().filter(|ev| {
        !ev.type_.starts_with("dispatcher.")
            && !ev.type_.starts_with("internal.")
            && !ev.type_.starts_with("plugin.")
    }).collect();

    // Append as JSONL
    for ev in domain_events {
        if let Ok(line) = serde_json::to_string(ev) {
            let _ = file.write_all(line.as_bytes());
            let _ = file.write_all(b"\n");
        }
    }
}
```

## Why This Is a Workspace-Wide Improvement

**Before S-8.08:** There was no way for bats integration tests to capture and assert on
plugin-emitted events without a full observability sink pipeline (Prometheus, OTEL, etc.).
Tests could only assert on exit codes and stderr output.

**After S-8.08:** Any bats test can set `VSDD_SINK_FILE=/tmp/events.jsonl` and then use
`jq` to assert on the exact fields of emitted telemetry events. This is used in
`tests/integration/E-8-hook-plugins/track-agent-start.bats` for all 13 test cases.

**All future hook stories** (S-8.09 through S-8.28 and beyond) can use the same pattern:
```bash
VSDD_SINK_FILE="$TMPDIR/events.jsonl"
run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
  bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
run jq -r '.your_field' "$SINK_FILE"
[ "$output" = "expected_value" ]
```

## Mutation Testing (CC-W15-002)

`mutation_testing_required: true` per CC-W15-002.

The `flush_sink_file` function has targeted mutation-resistance properties:

### Critical Mutations That Would Be Caught by Existing Tests

| Mutation | Caught By |
|----------|-----------|
| Remove domain-event filter (include `dispatcher.*` events) | AC-002a parity audit: `jq 'has("tool_name")'` would find `plugin.invoked` events with tool_name leaking through |
| Invert filter logic (exclude domain events instead of lifecycle) | AC-005(a–c): `jq -r '.type'` → `agent.start` assertion would fail (no domain events in sink) |
| Remove `mem::take` (leave events in queue) | AC-005(a–c): sink file would be empty; `[ -f "$SINK_FILE" ]` would fail |
| Use `.truncate(true)` instead of `.append(true)` | Would still pass (single-event tests); multi-event test needed to catch this |
| Silently return when `domain_events.is_empty()` removed | No-op for existing tests (empty-domain path still creates file); covered by AC-005(d) |

### Mutation Survivability Assessment

The current test suite provides strong mutation resistance for the happy path (events
written correctly) and the filter correctness path. The append-vs-truncate semantic
is not directly tested in isolation but is not a correctness risk for current test
scenarios (each bats test uses a fresh `SINK_DIR=$(mktemp -d)`).

Future improvement (post-v1.1): add a multi-event test that dispatches two hooks
in sequence and verifies both events appear in the JSONL — this would catch the
truncate mutation definitively.

### Integration with AC-005 Bats Tests

The `VSDD_SINK_FILE` mechanism is exercised by all 8 dispatcher-level bats tests
(ok 6–13). Each test independently:
1. Creates a fresh `SINK_DIR=$(mktemp -d)` in `setup()`
2. Sets `VSDD_SINK_FILE="$SINK_DIR/events.jsonl"`
3. Asserts `[ -f "$SINK_FILE" ]` after dispatch
4. Uses `jq` to validate event field values and field presence/absence

This multi-test coverage across varied input envelopes provides good overall mutation
resistance for the sink machinery.
