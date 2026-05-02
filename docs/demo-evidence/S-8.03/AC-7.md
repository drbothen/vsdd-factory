# AC-007 Evidence — Malformed JSON Graceful Exit

**AC:** AC-007 (traces to BC-7.03.081 invariant 1)
**Statement:** Malformed or missing JSON stdin -> graceful exit 0; no panic, no stderr backtrace.

## Recording

- `AC-007-malformed-json-graceful-exit.gif` / `.webm` / `.tape`

## Verification Steps Shown

1. `printf 'not-json' | env CLAUDE_PLUGIN_ROOT=... CLAUDE_PROJECT_DIR=... target/release/factory-dispatcher; echo exit=$?`
   — dispatcher exits 0; `exit=0` printed; no panic output; no backtrace in stderr.

## Bats Test Coverage (AC-007 case from test suite)

```bats
@test "AC-007: malformed JSON stdin => exit 0 and no panic (best-effort)" {
  run env ... bash -c "printf '%s' 'not valid json at all {{{}}}' | '$DISPATCHER'"
  [ "$status" -eq 0 ]
  ! echo "$output" | grep -qi "panic"
  ! echo "$output" | grep -qi "backtrace"
  # No agent.stop event in sink (non-SubagentStop envelopes don't trigger hook)
  if [ -f "$SINK_FILE" ]; then
    run grep -c '"agent.stop"' "$SINK_FILE"
    [ "$output" = "0" ]
  fi
}
```

Status: ok 13 in the bats suite.

## Implementation Note

Malformed JSON is handled at the dispatcher level (stdin deserialization failure).
The `on_error = "continue"` registry flag ensures dispatcher does not block on errors.
The hook itself never sees malformed JSON — `HookPayload` deserialization failure in
the SDK results in graceful `HookResult::Continue` (best-effort path per `set +e`
equivalent behavior).

## Result

PASS — Malformed JSON stdin results in exit 0; no panic; no backtrace; bats test ok 13.
