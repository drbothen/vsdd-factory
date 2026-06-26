# Demo Evidence Report — S-18.06

**Story:** S-18.06 v1.10 — validate-heavy-op-delegation WASM gate — advisory DelegationRecommended on heavy Bash operations; SEC-002 4-pass secret redaction in command_preview
**BC gate:** BC-4.15.001 v1.2
**VP:** VP-091
**Recorded:** 2026-06-26
**Toolchain:** VHS 0.11.0 (terminal recording) + factory-dispatcher (release build) + validate-heavy-op-delegation.wasm (~228 KB)

## Summary

All 6 acceptance criteria / edge-case segments have been recorded with VHS terminal sessions. Each segment shows the actual dispatcher invocation, the observed output (stderr advisory and/or plugin.log structured record), and a PASS assertion confirming the behavior. The gate NEVER produces exit code 2 (block) in any recording — INV2 is visually confirmed across all segments.

The AC-012 segment (added 2026-06-26) covers SEC-002 / BC-4.15.001 INV5: 5 sub-cases proving that secrets are masked as `***REDACTED***` in `command_preview` before it is written to the plugin.log, while the raw secret string is demonstrably absent from all emitted channels.

## Coverage Map

| Segment | AC / EC | Description | Artifacts |
|---------|---------|-------------|-----------|
| AC-001/AC-002 match advisory | AC-001 (PC-B-B1), AC-002 (PC-B-B2) | `cargo test --release --workspace` PreToolUse → stderr nudge with matched pattern + command preview; plugin.log `DelegationRecommended` record with all 5 required fields; dispatcher exit 0 | [gif](AC-001-AC-002-match-advisory.gif) [webm](AC-001-AC-002-match-advisory.webm) [tape](AC-001-AC-002-match-advisory.tape) |
| AC-003 no-match silent | AC-003 (PC-A) | `cargo fmt --check --all` PreToolUse → NO advisory; `DelegationRecommended` count = 0; silent Continue; dispatcher exit 0 | [gif](AC-003-no-match-silent.gif) [webm](AC-003-no-match-silent.webm) [tape](AC-003-no-match-silent.tape) |
| AC-006 truncation | AC-006 (INV4) | 149-char command containing `grep -r` → `command_preview` = first 120 chars + U+2026 ellipsis (121 code points total); identical in both stderr and plugin.log channels; dispatcher exit 0 | [gif](AC-006-truncation.gif) [webm](AC-006-truncation.webm) [tape](AC-006-truncation.tape) |
| AC-007 non-Bash no-op | AC-007 (PC-D) | Write PreToolUse event → `sync_plugins=0` in dispatcher trace (plugin NOT dispatched due to registry `tool="Bash"` filter); no advisory; `DelegationRecommended` count = 0; dispatcher exit 0 | [gif](AC-007-non-bash-noop.gif) [webm](AC-007-non-bash-noop.webm) [tape](AC-007-non-bash-noop.tape) |
| EC-012 + EC-013 registry config | AC-011/EC-012, EC-013 | EC-012: `patterns=[]` → normally-matching `cargo test --release --workspace` produces NO advisory (count=0). EC-013: `patterns=["./ci.sh"]` → `./ci.sh build` triggers `DelegationRecommended` with `matched_pattern="./ci.sh"`; confirms runtime registry-config-driven behavior (not hardcoded defaults). Both exit 0. | [gif](EC-012-EC-013-registry-config.gif) [webm](EC-012-EC-013-registry-config.webm) [tape](EC-012-EC-013-registry-config.tape) |
| AC-012 secret redaction (5-pass) | AC-012 (SEC-002 / BC-4.15.001 INV5) | 5 sub-cases: (1) flag-arg `--token supersecrettoken123` → `--token ***REDACTED***`; raw secret absent. (2) env-assignment `API_KEY=sk-abc123` → `API_KEY=***REDACTED***`; raw absent. (3) Authorization header `Bearer eyJtoken123` → `Authorization:***REDACTED***`; raw absent. (4) URL credentials `user:pass@example.com` → `https://***REDACTED***@example.com/db`; raw absent. (5) clean command `grep -r TODO . --include=*.rs` → preview UNCHANGED (no over-redaction). All exit 0 (INV2). | [gif](AC-012-secret-redaction.gif) [webm](AC-012-secret-redaction.webm) [tape](AC-012-secret-redaction.tape) |

## Acceptance Criteria Coverage

| AC | Title | Status | Segment |
|----|-------|--------|---------|
| AC-001 | Stderr nudge emitted on pattern match (PC-B-B1) | DEMONSTRATED | AC-001/AC-002 match advisory |
| AC-002 | plugin.log structured DelegationRecommended record (PC-B-B2) | DEMONSTRATED | AC-001/AC-002 match advisory |
| AC-003 | No emission on no-match; Continue (PC-A) | DEMONSTRATED | AC-003 no-match silent |
| AC-004 | Never blocks; always exit 0 (INV2) | DEMONSTRATED | All segments — exit 0 confirmed in every recording |
| AC-006 | command_preview truncated at 120 chars + U+2026 (INV4) | DEMONSTRATED | AC-006 truncation |
| AC-007 | Non-Bash tool call: no-op via registry tool filter (PC-D) | DEMONSTRATED | AC-007 non-Bash no-op |
| AC-011 / EC-012 | Empty patterns list: no emission on any Bash command | DEMONSTRATED | EC-012/EC-013 registry config |
| EC-013 | Custom pattern ./ci.sh: triggers advisory (registry-config-driven) | DEMONSTRATED | EC-012/EC-013 registry config |
| AC-012 | SEC-002 / INV5: 4-pass secret redaction — raw secret absent from plugin.log command_preview; `***REDACTED***` present; gate always Continue | DEMONSTRATED | AC-012 secret redaction (5-pass) |

## Key Observations

1. **INV2 (never blocks) confirmed across all segments:** Every recording shows `block_intent=false exit_code=0` in the dispatcher trace line. No segment produces exit code 2.

2. **AC-002 plugin.log JSON structure:** The AC-001/AC-002 recording shows the full JSON record pretty-printed — all 5 required fields visible: `code`, `level` (warn), `matched_pattern`, `command_preview`, `message`.

3. **AC-006 U+2026 ellipsis:** The truncation recording shows the `…` character at position 121 in both the stderr nudge and the extracted `command_preview` field. Length assertion: 121 code points confirmed by bash `${#PREVIEW}`.

4. **AC-007 sync_plugins=0:** The dispatcher trace header for the Write event shows `sync_plugins=0` — confirming the tool filter prevents dispatch before the plugin is even loaded, not just before it emits.

5. **EC-013 runtime config read:** The custom `./ci.sh` pattern is not in `DEFAULT_PATTERNS`. The recording confirms `matched_pattern="./ci.sh"` — proving `on_pre_tool_use` reads `plugin_config.patterns` at runtime rather than using hardcoded defaults (the F-P1-001 regression guard).

6. **AC-012 / INV5 — 4-pass redaction ordering:** All 4 redaction passes are applied to `command_preview` before the record is written to plugin.log. The recording shows all 5 sub-cases sequentially: (1) flag-arg `--token VALUE` masked; (2) `VAR=VALUE` env-assignment masked; (3) `Authorization: Bearer TOKEN` masked; (4) `https://user:password@host` URL credentials masked; (5) clean command `grep -r TODO . --include=*.rs` passes through unchanged — confirming no over-redaction. In every sub-case the dispatcher exits 0 (INV2 preserved). The raw secret strings (`supersecrettoken123`, `sk-abc123`, `eyJtoken123`, `pass@`) are demonstrably absent from the plugin.log record (verified by inline `PASS: raw secret ... ABSENT from plugin.log` assertion lines in the recording).

## Invocation Pattern

All recordings use the same dispatcher invocation pattern as the bats test suite:

```bash
printf '%s' "$ENVELOPE" | \
  FACTORY_DISPATCHER_INTERNAL_LOG=1 \
  VSDD_LOG_DIR="$WORK/.factory/logs" \
  CLAUDE_PLUGIN_ROOT="$WORK" \
  CLAUDE_PROJECT_DIR="$WORK" \
  "$DISPATCHER"
```

Where `$ENVELOPE` is a synthetic PreToolUse JSON event (Bash or Write), `$DISPATCHER` is `target/release/factory-dispatcher`, and the WASM is copied into `$WORK/hook-plugins/validate-heavy-op-delegation.wasm` along with a synthetic `hooks-registry.toml`.

## Artifacts

All artifacts are in `docs/demo-evidence/S-18.06/`:

```
AC-001-AC-002-match-advisory.gif    (303 KB)
AC-001-AC-002-match-advisory.webm   (249 KB)
AC-001-AC-002-match-advisory.tape
AC-003-no-match-silent.gif          (89 KB)
AC-003-no-match-silent.webm         (93 KB)
AC-003-no-match-silent.tape
AC-006-truncation.gif               (164 KB)
AC-006-truncation.webm              (171 KB)
AC-006-truncation.tape
AC-007-non-bash-noop.gif            (105 KB)
AC-007-non-bash-noop.webm           (107 KB)
AC-007-non-bash-noop.tape
EC-012-EC-013-registry-config.gif   (205 KB)
EC-012-EC-013-registry-config.webm  (343 KB)
EC-012-EC-013-registry-config.tape
AC-012-secret-redaction.gif         (1600 KB)
AC-012-secret-redaction.webm        (876 KB)
AC-012-secret-redaction.tape
evidence-report.md                  (this file)
```
