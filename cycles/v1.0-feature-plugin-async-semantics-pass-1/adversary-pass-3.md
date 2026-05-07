---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 3
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 4, medium: 3, low: 0, nit: 0 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

# Adversary Pass-3 Findings — F2 spec package, v1.0-feature-plugin-async-semantics-pass-1

## 1. Verdict

**SUBSTANTIVE** — multiple HIGH-severity drift findings; clock RESETS to 0/3.

## 2. Counts

HIGH: 4, MEDIUM: 3, LOW: 0, NIT: 0

## 3. Findings

### F-P3-001 [HIGH] VP-078 Harness 3 plugin list lags BC-7.06.001 Invariant 6 v1.2 (3 plugins missing) — POLICY 9 / cross-burst residue

**Evidence:**
- BC-7.06.001 line 75 enumerates 9 async-required plugins: `capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`, `session-end-telemetry`, `worktree-hooks`, `tool-failure-hooks`, `track-agent-start`, `track-agent-stop`, `session-learning`.
- VP-078 lines 54-59 Property 2 says "**six** telemetry-only plugins" and Harness 3 array (lines 261-268) lists only the first 6.
- ADR-019 lines 184-186 matches BC-7.06.001's expanded list of 9.
- VP-078 references that confirm the gap: lines 55, 71, 255, 290, 293, 459, 472 all still say "six".

**Why it matters:** A test built from VP-078 Harness 3 will pass even if a future engineer flips `track-agent-start`, `track-agent-stop`, or `session-learning` to `async = false` — the exact silent-degradation case Invariant 6 was promoted to invariant status to prevent. F-P2-006 expansion did not propagate to VP-078.

**Fix:** Update VP-078 §Property Statement, Harness 3 docstring/array, Coverage notes, and v1.3 amendment paragraph from "six" → "nine"; add `track-agent-start`, `track-agent-stop`, `session-learning` to the `required_async` array.

### F-P3-002 [HIGH] VP-079 Scenario 4 (`plugin.timeout` async path) is structurally untestable — VP design conflicts with BC-1.14.001 PC4

**Evidence:**
- BC-1.14.001 lines 62-67 PC4 states the dispatcher does NOT await async tasks; "process exits as soon as `sync_group` completes."
- VP-079 lines 300-314 Scenario 4 fixture has only one `[[hooks]]` entry with `async = true`, `timeout_ms = 50`, `sleep 60` script — no sync plugin in the registry.

**Why it matters:** With an empty `sync_group`, the dispatcher exits immediately after spawning the async task. The async timeout enforcement (50ms) requires the dispatcher's tokio runtime to remain alive long enough to detect the timeout, terminate the plugin, and emit `plugin.timeout` to FileSink. None of these can happen after the dispatcher exits.

**Fix options:** (a) Add a sync plugin to Scenario 4 fixture that runs ≥ timeout_ms to keep the dispatcher alive; (b) Reword BC-1.14.001 PC4 to specify a brief drain window for async-task event emission; (c) Reclassify Scenario 4 as sync-path and remove the async path test.

### F-P3-003 [HIGH] BC-INDEX titles drift from BC H1 across 6 amended BCs — POLICY 7 systematic violation

**Evidence (file H1 vs BC-INDEX):**

| BC | File H1 | BC-INDEX |
|----|---------|----------|
| BC-1.08.002 (line 28) | "...iff at least one **sync-group** plugin recorded a block_intent; async-group verdicts never affect exit code" | line 179: "...iff at least one block_intent recorded" |
| BC-1.01.007 (line 28) | "...schema_version=2, enabled defaults to true" | line 93: "...schema_version=2 (v2), async=false default; enabled defaults to true" |
| BC-4.04.004 (line 30) | "...once:true **and** synchronous envelope (async:true removed per ADR-019)" | line 316: "...once:true**;** synchronous **at** envelope (async:true removed per ADR-019)" |
| BC-4.05.004 | "...once:true **and** synchronous envelope..." | line 321: "...once:true**;** synchronous **at** envelope..." |
| BC-4.07.003 | "...synchronous envelope...; timeout:10000" | line 325: "...synchronous **at** envelope..." |
| BC-4.08.002 | "...synchronous envelope...; timeout:10000" | line 328: "...synchronous **at** envelope..." |

**Why it matters:** POLICY 7 explicitly states BC H1 is the title source of truth and BC-INDEX must propagate from H1 same-burst. A 6-BC drift cluster in a single cycle indicates the F2 burst-fix cycles updated H1s without propagating to BC-INDEX. BC-1.08.002 is the worst case — a clear semantic shift (block_intent scoped to sync-group, async-group exclusion) is invisible from the index.

**Fix:** Re-sync 6 BC-INDEX rows to match H1s byte-for-byte.

### F-P3-004 [HIGH] VP-INDEX classifies VP-079 as `invariant` but VP-079 frontmatter is `postcondition` — POLICY 9 type drift

**Evidence:**
- VP-079 line 28 `type: postcondition`
- VP-INDEX line 146 Full Index Type column = "invariant"

**Why it matters:** VP-INDEX is the catalog source-of-truth (POLICY 9). Type drift can mis-classify the property in coverage rollups and prevents reverse lookups (e.g., "all postcondition VPs"). One of these is wrong.

**Fix:** Decide canonically (BC-3.08.001 cataloged-events conformance feels like a postcondition — emission triggers a payload-shape obligation) and sync both ends.

### F-P3-005 [MEDIUM] BC-7.06.001 ID-prefix vs subsystem-frontmatter accounting opaque in ARCH-INDEX — POLICY 6 ambiguity

**Evidence:**
- BC-7.06.001 line 18 `subsystem: "SS-01"` (per F-P1-006 reanchor).
- ARCH-INDEX line 103 "SS-07 ... 197 (+1 F2-async BC-7.06.001 registry schema v2 + CI lint)" — counts BC-7.06.001 as SS-07.
- ARCH-INDEX line 97 SS-01 count says "+1 F2-async BC-1.14.001" — does NOT include BC-7.06.001.
- DI-014 amendment acknowledges this dual-state explicitly.

**Why it matters:** SS-01 is BC-7.06.001's authoritative subsystem (per frontmatter and POLICY 6), but ARCH-INDEX SS-01 BC count omits it while SS-07 count includes it.

**Fix:** Add a clarifying footnote to ARCH-INDEX SS-01/SS-07 rows describing the dual-state, OR re-tally counts to authoritative subsystem.

### F-P3-006 [MEDIUM] SS-09 Modules table line 47 still says `"async": true` — body-vs-amendment readability gap

**Evidence:**
- SS-09-config-activation.md line 47 literal text: `declares event types, dispatcher binary path template, "async" flags per event type`.
- Lines 81-86 still show ORIGINAL schema example with `"async": true`.
- Lines 102-105 still claim `PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd use "async": true`.
- Lines 107-108 still say `schema_version = 1`.
- Amendment at lines 199-289 corrects all of these textually, but only as a supplemental footnote.
- Same pattern in SS-07-hook-bash.md (lines 46, 70 still have `schema_version = 1`).

**Why it matters:** A casual reader of the Modules table or Public Interface section sees stale post-ADR-019 information.

**Fix:** Either in-place replace the stale claims with corrected text and move amendment to CHANGELOG-style block, OR add inline `**[SUPERSEDED — see Amendment 2026-05-07 below]**` flag adjacent to each stale block.

### F-P3-007 [MEDIUM] VP-079 Scenario 1 fixture risks identical structural failure as Scenario 4

**Evidence:**
- VP-079 lines 166-181 Scenario 1 fixture has only one async plugin (`test-async-blocker`, `async = true`) — no sync plugin.
- Per BC-1.14.001 PC4, dispatcher exits as soon as sync_group (empty) completes, before async plugin returns exit 2 and before `plugin.async_block_discarded` can be emitted.

**Why it matters:** Same root cause as F-P3-002. The harness assumes the dispatcher waits for async plugin completion and emits `plugin.async_block_discarded`, but BC-1.14.001 PC4 says it does NOT wait.

**Fix:** Same options as F-P3-002.

## 4. Policy compliance

| Policy | Status |
|---|---|
| POLICY 1 (append-only IDs) | PASS |
| POLICY 2 (lift invariants) | PASS |
| POLICY 3 (state-manager last) | N/A |
| POLICY 4 (semantic anchoring) | PASS |
| POLICY 5 (creators justify anchors) | PASS |
| POLICY 6 (ARCH-INDEX subsystem source-of-truth) | **PARTIAL** (F-P3-005) |
| POLICY 7 (BC H1 source-of-truth) | **VIOLATION** (F-P3-003: 6-BC pattern) |
| POLICY 8 (story bcs propagate) | N/A |
| POLICY 9 (VP-INDEX source-of-truth) | **PARTIAL** (F-P3-001 + F-P3-004) |

## 5. Open questions

- BC-1.14.001 PC4 vs VP-079 Scenarios 1+4: does the dispatcher guarantee any async-task drain window before exit, or are async events truly best-effort? If the latter, VP-079's async-only fixtures are non-viable.

## 6. Top 3

1. **F-P3-002 / F-P3-007** — VP-079 Scenarios 1+4 cannot test what they claim under BC-1.14.001 PC4 semantics.
2. **F-P3-001** — VP-078 Harness 3 lags BC-7.06.001 Invariant 6 by 3 plugins.
3. **F-P3-003** — 6-BC POLICY 7 H1↔BC-INDEX drift cluster.
