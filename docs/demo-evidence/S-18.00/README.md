# Demo Evidence — S-18.00: Dispatcher PreCompact/PostCompact Routing + check-harness-version.sh

Story: S-18.00 | Feature branch: feature/S-18.00 | Recorded: 2026-06-17

## Toolchain

VHS 0.11.0 (terminal recording); all recordings made against the real dispatcher binary built from
`cargo build --release -p factory-dispatcher` on the feature branch at HEAD `a80bac43`.
No fabricated output — every frame is a live run of `target/release/factory-dispatcher` or
`plugins/vsdd-factory/hooks/check-harness-version.sh`.

## Coverage Map

| Artifact | AC | BC Clause | Scenario | Observed Result |
|----------|----|-----------|----------|-----------------|
| `AC-001-precompact-routes-to-plugin.{gif,webm}` | AC-001 | BC-1.15.001 PC1 | PreCompact event, one registered plugin exits 0 | `sync_plugins=1 plugins_run=1 block_intent=false exit_code=0` — routing confirmed |
| `AC-004-precompact-exit2-blocks.{gif,webm}` | AC-004 / VP-086 | BC-1.15.001 PC4 | PreCompact event, plugin exits 2, on_error=block | `block_intent=true exit_code=2 blocking_plugins=precompact-blocker` — VP-086 property demonstrated |
| `AC-002-postcompact-advisory-only.{gif,webm}` | AC-002 | BC-1.15.001 PC2 | PostCompact event, plugin exits 2, on_error=block | `block_intent=false exit_code=0` — advisory suppression confirmed; contrast with AC-004 |
| `AC-008-check-harness-version.{gif,webm}` | AC-008 | BC-1.15.001 INV3 | Three paths: v2.1.177 (pass), unset (advisory), v2.1.100 (advisory) | Exit 0 on supported version; exit 1 on unset/below-threshold; never exits 2 |

## Artifact Index

```
docs/demo-evidence/S-18.00/
  AC-001-precompact-routes-to-plugin.gif     — VHS recording (PR embed)
  AC-001-precompact-routes-to-plugin.webm    — VHS recording (archival)
  AC-001-precompact-routes-to-plugin.tape    — VHS script source
  demo-run-ac001.sh                          — demo driver script (calls real dispatcher)

  AC-002-postcompact-advisory-only.gif       — VHS recording (PR embed)
  AC-002-postcompact-advisory-only.webm      — VHS recording (archival)
  AC-002-postcompact-advisory-only.tape      — VHS script source
  demo-run-ac002.sh                          — demo driver script

  AC-004-precompact-exit2-blocks.gif         — VHS recording (PR embed)
  AC-004-precompact-exit2-blocks.webm        — VHS recording (archival)
  AC-004-precompact-exit2-blocks.tape        — VHS script source
  demo-run-ac004.sh                          — demo driver script

  AC-008-check-harness-version.gif           — VHS recording (PR embed)
  AC-008-check-harness-version.webm          — VHS recording (archival)
  AC-008-check-harness-version.tape          — VHS script source
  demo-run-ac008.sh                          — demo driver script
```

## Reproduction

All demos can be re-run from the worktree root:

```bash
# Re-run a single demo driver (real binary, no VHS required):
bash docs/demo-evidence/S-18.00/demo-run-ac001.sh
bash docs/demo-evidence/S-18.00/demo-run-ac002.sh
bash docs/demo-evidence/S-18.00/demo-run-ac004.sh
bash docs/demo-evidence/S-18.00/demo-run-ac008.sh

# Re-record all VHS tapes (requires VHS >= 0.11.0):
for tape in docs/demo-evidence/S-18.00/*.tape; do vhs "$tape"; done
```

## Key Observations

**AC-001 (PC1) — routing confirmed:**
The dispatcher line `sync_plugins=1 plugins_run=1` proves the PreCompact event was matched
by the registry entry (`event = "PreCompact"`) and the plugin was invoked. Exit code 0 with
`block_intent=false` confirms no spurious blocking on a passing plugin.

**AC-004 / VP-086 (PC4) — block propagation confirmed:**
The dispatcher emits `block_intent=true exit_code=2 blocking_plugins=precompact-blocker`,
satisfying VP-086: "factory-dispatcher receives a PreCompact event; registered plugin exits 2;
dispatcher propagates block_intent=true to harness." The single-plugin-sufficient property (EC-001)
is demonstrated by having exactly one exit-2 plugin in the registry.

**AC-002 (PC2) — advisory suppression confirmed:**
Despite using the same registry shape as AC-004 (on_error=block, plugin exits 2), the PostCompact
event produces `block_intent=false exit_code=0`. The `is_advisory_only()` gate in `main.rs`
correctly suppresses block intent for PostCompact events.

**AC-008 (INV3) — three paths demonstrated:**
- `CLAUDE_CODE_VERSION=2.1.177`: `harness v2.1.177 >= v2.1.105 — PreCompact block-intent supported` → exit 0
- `CLAUDE_CODE_VERSION` unset: `harness version undeterminable … set CLAUDE_CODE_VERSION in the harness environment` → exit 1 (advisory)
- `CLAUDE_CODE_VERSION=2.1.100`: `harness v2.1.100 < v2.1.105 — PreCompact block-intent will not be honoured` → exit 1 (advisory)

The script never exits 2 (block-intent is reserved for the `precompact-flush` WASM plugin per S-18.04a).
