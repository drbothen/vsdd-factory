---
document_type: architecture-decision-record
level: L3
adr_id: ADR-042
version: "1.4"
title: "ADR-042: validate-cross-site-correspondence fuel budget raise to 20M and loud fuel-exhaustion signaling"
status: active
ratified: "2026-08-13"
ratification_note: "Human ratification 2026-08-13 (S-21.07 pass-10 fix burst, D-992): the 2026-08-08 human ruling ('raise the fuel budget AND make exhaustion loud') already authorized this ADR's substance; §Decision 1's arithmetic (12M floor / 20M chosen / 92% margin) is unchanged since v1.2, and the v1.3 amendment corrected the F-S2107-P10-006 self-contradiction (row-4 vs §Decision 2) with a POLICY 13 BOUNDARY-POLARITY mutant table. This ratification closes the F-S2107-P10-005 ADR-042 leg — the frontmatter status/decision-log-narrative gap the v1.3 §Status section flagged NEEDS-HUMAN. Ratified alongside ADR-041 in the same human decision, per v1.3 §Status recommendation."
date: 2026-08-08
producer: architect
timestamp: 2026-08-08T00:00:00Z
deciders:
  - architect
  - human (ruling 2026-08-08)
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-035 (cross-site correspondence validation — fuel error taxonomy §Decision 5; this ADR is the measured follow-up that discharges D-945 and raises the budget ADR-035 §Decision 5 left as advisory-open)
  - ADR-039 (validator failure policy — per-plugin fuel_cap calibration phases 1–4; this ADR bridges to ADR-039 Phase 3 by raising the global default while per-plugin caps are not yet implemented)
anchors:
  - SS-01
  - SS-05
subsystems_affected:
  - SS-01
  - SS-05
last_amended: "2026-08-13 (v1.4) — AMENDED (architect; S-21.07 pass-10 fix cascade, BODY-
  vs-FRONTMATTER reconciliation pass, pre-adversary-pass-11): §Status section's
  'RATIFICATION STATUS — NEEDS HUMAN ADJUDICATION (F-S2107-P10-005)' paragraph and
  'Recommendation to human' paragraph reconciled to present-tense — human ratification
  OCCURRED 2026-08-13 (D-992; see frontmatter `status: active` / `ratified: '2026-08-13'` /
  `ratification_note` above). The NEEDS-HUMAN framing is now historical, not current; the
  §Status prose is corrected in place (superseded, not deleted) so body agrees with
  frontmatter. No change to §Decision 1's fuel-budget arithmetic or §Decision 2's
  global-raise mechanism.
  [Prior: 2026-08-13 (v1.3) — AMENDED (architect; S-21.07 pass-10 ADR-anchored fix
  cascade): §Decision 1 Erratum added closing F-S2107-P10-006 (self-contradiction between
  §Decision 1 row 4 'independent budgets' claim and §Decision 2's global-raise-only
  mechanism) — corrected with POLICY 13 BOUNDARY-POLARITY mandatory mutant table and fresh
  literal-shell evidence (burst-log.md/decision-log.md/lessons.md measured at
  289%/297%/198% of the 20M cap under this ADR's own adapter-class fuel model, confirming
  the excluded region is currently harmful, not merely theoretically so); five volatile
  BC-INDEX.md line-number pins ('line 1464') replaced with stable body-table-row anchor
  form per TD-VSDD-091 closing the ADR-042 leg of F-S2107-P10-008 (verbatim literal-shell
  evidence blocks left unchanged — only authored narrative pins were volatile-anchor
  violations); new §Status section added documenting F-S2107-P10-005 ratification-status
  gap (frontmatter status: proposed with no ratified: field despite decision-log narrative
  'ratified' language) as NEEDS-HUMAN with a concrete recommendation — architect does not
  claim unilateral ratification authority per the ADR-040/D-965 precedent. No change to
  §Decision 1's fuel-budget arithmetic or §Decision 2's global-raise mechanism.
  [Prior: 2026-08-08 (v1.2) — AMENDED (architect): perf-fuel-2 measurement results
  integrated — adapter architecture documented (three validators = one shared
  legacy-bash-adapter.wasm; fuel = 29,452 + 27.514 × payload_bytes R²=0.9999999);
  19.9M illustrative bound's structural failure reason explained (architectural mismatch,
  not arithmetic error); §Decision 1 12M floor + 92% margin stated; §Decision 3 re-scoped
  into three distinct problem classes (a) silent on_error=continue (b) ambiguous
  fuel-vs-epoch cause (c) PostToolUse cannot revert; platform-wide scope (839 events /
  35 plugins) confirmed with literal shell evidence. [Prior: 2026-08-08 (v1.1) — AMENDED
  (architect): E-22 anchor corrected to E-21 W7
  (E-22 is dissolved per operator ruling 2026-08-08; story_count: 0; CLAUSE.md Rule 3
  requires attachment to a specific future story or wave that exists); platform-wide fuel
  exhaustion scope documented — three fail-closed validators (validate-factory-path-root,
  validate-input-hash, validate-template-compliance) also exhausted 10M on ARCH-INDEX.md
  (325KB) during this authoring session; whether 20M is sufficient for those three is
  unknown pending performance-engineer measurement (their fuel models differ from
  cross-site-correspondence). [Prior: 2026-08-08 (v1.0) — initial ruling (architect): fuel
  budget raise 10M→20M with derivation; loud exhaustion via dispatcher stderr summary +
  advisories[] payload; ADR-035 §Decision 5 O(n) correction + D-945 discharge; deferred
  targeted-row-lookup structural fix defined.]]"
modified:
  - "2026-08-08 (v1.0)"
  - "2026-08-08 (v1.1)"
  - "2026-08-08 (v1.2)"
  - "2026-08-13 (v1.3)"
  - "2026-08-13 (v1.4)"
---

# ADR-042: validate-cross-site-correspondence fuel budget raise to 20M and loud fuel-exhaustion signaling

## Context

ADR-035 §Decision 5 (2026-07-30) left two items open:
1. A fuel budget advisory: "any hook with `on_error = "block"` that reads `.factory/` artifacts larger than ~100KB is at risk" — guidance that was speculative, not measured.
2. Standing drift item **[D-945]**: "ADR-035 §Decision 5 fuel budget advisory — OPEN 2026-07-30 — 'May need revision after S-21.07 benchmarks.'"

S-21.07 pass-9 has now produced those benchmarks. The performance-engineer ran a production-fixture measurement on 2026-08-08 against the `validate-cross-site-correspondence` WASM plugin (binary from `.worktrees/fuel-loud`, commit `fbb9dcb6`, 231,661 bytes; comparable-magnitude caveat stated in §Source / Origin). The measurement falsifies the earlier ruling that directed keeping the budget as-is: it showed **4 rows of runway** before exhaustion, not the ~110 rows that earlier analysis suggested. The human operator reversed the "keep budget" ruling when presented with the measured data.

**The ruling this ADR implements (human-authorized, 2026-08-08):**
> Raise the fuel budget AND make exhaustion loud. Plus create a deferred item for a long-term structural fix.

**Measured facts (performance-engineer, captured stdout, production fixture, 2026-08-08):**

| extra_rows_before_BC-5.39.010 | bytes_scanned | fuel_consumed | status  |
|-------------------------------|--------------|---------------|---------|
| 0                             | 415,523      | 9,920,913     | OK      |
| 0 (independent re-run)        | 415,523      | 9,921,197     | OK      |
| 1                             | 416,009      | 9,936,674     | OK      |
| 2                             | 416,495      | 9,953,444     | OK      |
| 3                             | 416,981      | 9,970,304     | OK      |
| 4                             | 417,467      | 9,989,369     | OK      |
| 5                             | 417,953      | 10,000,000    | TIMEOUT |

Regression model (N=5..986 extra rows, 24 measured points):
`fuel = 2,585,970 + 53.18 × var_bytes`, R² = 0.998790. Quadratic term coefficient 5.42×10⁻⁵ adds +0.075% R² — fuel cost is **linear in input bytes, not superlinear**.

**Key structural fact**: `extract_bc_index_version_state` returns early once BC-5.39.010's body-table row (locatable via `grep -n '\[BC-5\.39\.010\]' BC-INDEX.md`) is found. Only the ~415KB prefix before that row is scanned. The 161KB tail is never read.

**Corpus-verified byte figures (literal shell, run 2026-08-08):**

```
$ wc -c /Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md
  576842 /Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md

$ grep -n '\[BC-5\.39\.010\]' /Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md | head -1 | cut -d: -f1
1464

$ head -n 1463 /Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md | wc -c
  415278

$ head -n 1464 /Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md | wc -c
  415961
```

BC-INDEX.md: 576,842 bytes total. BC-5.39.010's body-table row (per the captured `grep -n` above, at that corpus snapshot); bytes before its row = 415,278 (current corpus, slightly lower than the measurement's 415,523 due to the concurrent D-962 in-flight BC-5.39.010 v1.14→v1.15 amendment). The measurement's corpus was the pre-v1.15 state. Delta is 245 bytes and immaterial to budget derivation.

**Per-row marginal cost (literal shell derivation from measured data):**

```
$ printf "Row 1 delta: %d fuel\nRow 2 delta: %d fuel\nRow 3 delta: %d fuel\nRow 4 delta: %d fuel\nSum: %d\nAverage: %d fuel/row\n" \
  $(( 9936674 - 9920913 )) $(( 9953444 - 9936674 )) \
  $(( 9970304 - 9953444 )) $(( 9989369 - 9970304 )) \
  $(( 9936674 - 9920913 + 9953444 - 9936674 + 9970304 - 9953444 + 9989369 - 9970304 )) \
  $(( (9936674 - 9920913 + 9953444 - 9936674 + 9970304 - 9953444 + 9989369 - 9970304) / 4 ))
Row 1 delta: 15761 fuel
Row 2 delta: 16770 fuel
Row 3 delta: 16860 fuel
Row 4 delta: 19065 fuel
Sum: 68456
Average: 17114 fuel/row
```

Regression model marginal: 53.18 fuel/byte × 486 bytes/SS-05-row ≈ 25,845 fuel/row (over the wider N=5..986 range). The direct-measurement average (17,114 fuel/row) is more conservative for the specific near-exhaustion region. Both agree on the scale: 15,000–26,000 fuel per SS-05 row. The direct-measurement figure is used for budget derivation because it represents actual behavior at the exhaustion boundary; the regression model provides the wider linear trend confirmation.

**Exhaustion is currently silent for `on_error = "continue"` plugins:**

```json
{"type":"plugin.invoked","plugin_name":"validate-cross-site-correspondence","event":"PostToolUse"}
{"type":"plugin.timeout","cause":"fuel","elapsed_ms":1,"fuel_consumed":10000000}
```

No `plugin.completed` record. Dispatcher emits `block_intent=false exit_code=0`, stdout empty. The calling agent receives no signal that validation was skipped. `on_error = "continue"` absorbs the exhaustion. The bats integration suite catches it only when explicitly run.

**Platform-wide scope of the fuel exhaustion problem (observed during this authoring session, 2026-08-08):**

During authoring of this ADR, every Edit to ARCH-INDEX.md (current size: 325,375 bytes / 325KB, 574 lines) produced the following PostToolUse block from three fail-closed validators:

```
blocking_plugins=validate-factory-path-root,validate-input-hash,validate-template-compliance
block_reason="fail-closed: plugin timed out"
```

The fuel exhaustion problem is therefore **not specific to `validate-cross-site-correspondence`**. ARCH-INDEX.md is large enough (325KB) to exhaust the 10M budget for other validators that perform linear reads of `.factory/` artifacts. The three exhausting validators are fail-closed: for PostToolUse hooks, this means the write succeeds on disk and the block is advisory (PostToolUse cannot revert writes), but future dispatches could be affected if their PostToolUse classification changes.

**Whether the 20M raise is sufficient for these three validators against ARCH-INDEX.md at 325KB is unknown without measurement.** This ADR's 20M budget was derived from `validate-cross-site-correspondence`'s measured fuel model (`fuel = 2,585,970 + 53.18 × var_bytes`). That model does not extend to `validate-factory-path-root`, `validate-input-hash`, or `validate-template-compliance` — each has different WASM bytecode and a different per-byte fuel cost. As a reference point only (not an estimate): applying the cross-site-correspondence model at 325KB yields approximately `2,585,970 + 53.18 × 325,375 ≈ 19.9M`, within 0.1M of the new 20M cap. Whether the three validators are faster (lower per-byte cost → sufficient at 20M) or slower (higher per-byte cost → still exhausting at 20M) cannot be stated without a benchmark analogous to S-21.07's measurement suite. **Do not treat the 19.9M cross-site figure as an estimate for these plugins; it is an illustrative bound, not a calibrated model.**

Routing: performance-engineer is needed to run a multi-plugin multi-artifact-size benchmark covering `validate-factory-path-root`, `validate-input-hash`, and `validate-template-compliance` against ARCH-INDEX.md, STATE.md, lessons.md, and BC-INDEX.md at their current sizes. If any of those three still exhaust at 20M, a follow-up ADR will be needed to raise the global cap further or implement per-artifact routing. This ADR's scope is `validate-cross-site-correspondence` specifically.

**perf-fuel-2 measurement result (2026-08-08) — adapter architecture, model, and 20M validation:**

**Architectural finding: three registry entries, one shared WASM binary.** Confirmed via hooks-registry.toml inspection. All three validators share `hook-plugins/legacy-bash-adapter.wasm`:

```
$ grep -c 'legacy-bash-adapter' \
    /Users/zious/Documents/GITHUB/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml
78
```

78 occurrences cover 35 plugin entries (per file header: `# 35 legacy-bash-adapter entries in this same file`), each specifying `plugin = "hook-plugins/legacy-bash-adapter.wasm"`. The three validators are three registry entries with different `script_path` configs but identical WASM. Fuel differences between them on the same input are <300 units (noise).

`on_error = "block"` confirmed for all three (hooks-registry.toml inspection):

```
validate-factory-path-root:    on_error=block  plugin=hook-plugins/legacy-bash-adapter.wasm
validate-input-hash:           on_error=block  plugin=hook-plugins/legacy-bash-adapter.wasm
validate-template-compliance:  on_error=block  plugin=hook-plugins/legacy-bash-adapter.wasm
```

`elapsed_ms = 0` confirmed — exhaustion traps before subprocess exec (log inspection, 2026-08-08):

```
$ grep '"type":"plugin.timeout"' .factory/logs/dispatcher-internal-2026-08-08.jsonl \
    | grep '"cause":"fuel"' | grep 'validate-template-compliance' | head -1 \
    | python3 -c "import sys,json; d=json.loads(sys.stdin.read()); \
      print('elapsed_ms:', d['elapsed_ms'], '| fuel_consumed:', d['fuel_consumed'])"
elapsed_ms: 0 | fuel_consumed: 10000000
```

Because fuel exhaustion occurs before the WASI `exec_subprocess` call, the bash script body **never executes** when the adapter is fuel-starved. This is structurally different from `validate-cross-site-correspondence`'s silent `on_error = "continue"` exhaustion: these three validators ARE agent-visible via `block_intent=true exit_code=2` — already audible without §Decision 3's signaling work.

**Measured adapter fuel model (perf-fuel-2, 2026-08-08):**

```
fuel = 29,452 + 27.514 × payload_bytes     R² = 0.9999999
```

Fuel scales with *payload* size (ARCH-INDEX.md file bytes + `last_assistant_message` + injected dispatcher fields), not raw file bytes. Compare cross-site-correspondence: `2,585,970 + 53.18 × var_bytes`. Fixed cost 89× lower; marginal rate ~2× lower. The adapter WASM does only a WASI stdin read and JSON round-trip — it delegates the real work to a host bash subprocess.

**Why the 19.9M illustrative bound was structurally wrong — architectural mismatch, not arithmetic error:**

The §Context "Platform-wide scope" note computed `2,585,970 + 53.18 × 325,375 ≈ 19.9M` by applying the cross-site-correspondence model to ARCH-INDEX.md's byte count. The arithmetic is correct for `validate-cross-site-correspondence` at 325KB. It fails for the adapter class because the two plugins have different WASM ABIs: one scans a file via `read_file`; the other round-trips the dispatcher payload via WASI stdin. Fixed costs differ by 89×; marginal rates differ by ~2×; input domain differs (file bytes vs payload bytes). The note was correct to label the figure "illustrative bound, not a calibrated model" and to refuse to estimate — this confirms exactly why that instinct was right.

**Production worst-case measurement (perf-fuel-2, ARCH-INDEX.md 326,258 bytes + 50KB `last_assistant_message`, 2026-08-08):**

Payload: ARCH-INDEX.md (326,258 bytes) + `last_assistant_message` (~50KB) + injected fields = **377,109 bytes**.
Measured fuel: **10,406,058** — exhausts 10M cap; within 1% of model prediction (`29,452 + 27.514 × 377,109 ≈ 10,406,858`).

```
$ printf "Exhaustion fuel:               %d\n10%% margin floor:              %d (rounds to 12M)\nChosen cap:                    %d\nHeadroom:                      %d\nMargin above exhaustion:       %.0f%%\nPayload to re-exhaust at 20M:  %.0f bytes (~%.0f KB; factor %.1fx current)\n" \
  10406058 \
  $(python3 -c "import math; print(math.ceil(10406058 * 1.10 / 1000000) * 1000000)") \
  20000000 \
  $((20000000 - 10406058)) \
  $(python3 -c "print((20000000 - 10406058) / 10406058 * 100)") \
  $(python3 -c "print((20000000 - 29452) / 27.514)") \
  $(python3 -c "print((20000000 - 29452) / 27.514 / 1024)") \
  $(python3 -c "print(((20000000 - 29452) / 27.514) / 377109)")
Exhaustion fuel:               10406058
10% margin floor:              12000000 (rounds to 12M)
Chosen cap:                    20000000
Headroom:                      9593942
Margin above exhaustion:       92%
Payload to re-exhaust at 20M:  725832 bytes (~709 KB; factor 1.9x current)
```

**20M is validated for the adapter class.** 12M is the minimum defensible cap at 10% margin; 20M is chosen, providing 92% margin above the measured worst-case. ARCH-INDEX.md payload would need to grow to ~726KB (~1.9× current) before re-exhaustion at 20M.

**Platform-wide scope confirmed with literal shell evidence:**

```
$ grep '"type":"plugin.timeout"' .factory/logs/dispatcher-internal-2026-08-08.jsonl \
    | grep -c '"cause":"fuel"'
839
$ grep '"type":"plugin.timeout"' .factory/logs/dispatcher-internal-2026-08-08.jsonl \
    | grep '"cause":"fuel"' | grep -oE '"plugin_name":"[^"]+"' | sort -u | wc -l
35
```

839 fuel-exhaustion events across 35 distinct plugins in this single session (perf-fuel-2 measured 838; one additional event recorded from this authoring burst's own ARCH-INDEX.md writes). Most validators fire 28 times — once per ARCH-INDEX.md Edit in this session. Because 35 plugins share `legacy-bash-adapter.wasm`, they share the same fuel model and exhaust together on large-file writes. The 10M cap was degrading validation reliability across the entire hook chain, not just `validate-cross-site-correspondence`. The 20M raise is a platform-wide remedy.

**Why ADR-035 §Decision 5's "~100KB at risk" guidance was wrong:** The guidance described fuel risk for `on_error = "block"` plugins that read large artifacts. At 53.18 fuel/byte, 100KB costs 2,585,970 (fixed) + 53.18 × 102,400 = 8,041,570 fuel — nearly 80% of the 10M budget. The actual exhaustion for `validate-cross-site-correspondence` occurs at ~415KB (80% of the 10M budget consumed by WASM startup + the 415KB prefix). "~100KB" understated the risk zone by 4×.

**Why the per-plugin `fuel_cap` registry field is not yet available:** ADR-035 §Decision 5 routed registry `fuel_cap` field implementation to implementer. ADR-039 §Decision 2/4 requires the schema extension as Phase 1. Current `hooks-registry.toml` inspection:

```
$ grep -c "fuel_cap\|failure_policy" /Users/zious/Documents/GITHUB/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml
0
```

Neither `fuel_cap` nor `failure_policy` fields are present. ADR-039 Phase 1 has not shipped. The only available budget lever is the global `InvokeLimits::default()` in the dispatcher's `invoke` module.

## Decision

### Decision 1 — New fuel budget value: 20,000,000 (20M), derivation from measured data

**Budget derivation (literal shell arithmetic, run 2026-08-08):**

```
$ BASELINE=9920913; ROW_COST=17114; CURRENT_CAP=10000000; TARGET_ROWS=600; NEW_CAP=20000000
$ printf "Current cap:       %d fuel\nBaseline fuel:     %d fuel\nCurrent runway:    %d rows (%d fuel headroom / %d fuel/row)\n" \
  "$CURRENT_CAP" "$BASELINE" \
  $(( (CURRENT_CAP - BASELINE) / ROW_COST )) \
  $(( CURRENT_CAP - BASELINE )) "$ROW_COST"
Current cap:       10000000 fuel
Baseline fuel:     9920913 fuel
Current runway:    4 rows (79087 fuel headroom / 17114 fuel/row)

$ printf "Target headroom:   %d rows\nAdditional fuel:   %d * %d = %d fuel\nMinimum cap:       %d + %d = %d fuel\n" \
  "$TARGET_ROWS" "$TARGET_ROWS" "$ROW_COST" $(( TARGET_ROWS * ROW_COST )) \
  "$BASELINE" $(( TARGET_ROWS * ROW_COST )) $(( BASELINE + TARGET_ROWS * ROW_COST ))
Target headroom:   600 rows
Additional fuel:   600 * 17114 = 10268400 fuel
Minimum cap:       9920913 + 10268400 = 20189313 fuel

$ printf "Proposed new cap:  %d fuel\nActual headroom:   (%d - %d) / %d = %d rows\nHeadroom bytes:    %d rows * 486 = %d bytes (~%d KB)\n" \
  "$NEW_CAP" "$NEW_CAP" "$BASELINE" "$ROW_COST" \
  $(( (NEW_CAP - BASELINE) / ROW_COST )) \
  $(( (NEW_CAP - BASELINE) / ROW_COST )) \
  $(( ((NEW_CAP - BASELINE) / ROW_COST) * 486 )) \
  $(( ((NEW_CAP - BASELINE) / ROW_COST) * 486 / 1024 ))
Proposed new cap:  20000000 fuel
Actual headroom:   (20000000 - 9920913) / 17114 = 589 rows
Headroom bytes:    589 rows * 486 = 286254 bytes (~279 KB)
```

**The new fuel budget is 20,000,000 (20M).**

Derivation summary: Target headroom of 600 rows requires a minimum cap of 20,189,313 fuel at the measured marginal cost of 17,114 fuel/SS-05-row. 20M falls 189,313 fuel short of the 600-row target at the conservative direct-measurement marginal. This rounds to 589 rows of actual headroom at the measured marginal, or 614 rows at the regression-model marginal (16,400 fuel/row). Both values provide meaningful runway.

**Adapter-class cross-validation: 12M floor, 20M provides 92% margin.** Independent validation from the perf-fuel-2 adapter measurement (see §Context "perf-fuel-2 measurement result"): worst-case adapter fuel at ARCH-INDEX.md payload (377,109 bytes) = 10,406,058 fuel — exhausts 10M but well within 20M. Minimum defensible cap at 10% margin above the worst-case: **12M** (10,406,058 × 1.10 = 11,446,664, ceiling to nearest megabyte = 12M). Chosen cap 20M provides **92% margin** above the worst-case ((20M − 10.4M) / 10.4M). Two independent derivation paths — cross-site-correspondence row-headroom (589 rows / 12–30 waves) and adapter worst-case margin (92%) — both validate 20M. 12M is recorded as the floor so the chosen 20M margin is stated rather than implied.

**Headroom rationale (600 rows):** BC-5.39.010's body-table row falls after all SS-05.1–SS-05.38 rows in BC-INDEX.md's SS-ordered layout. The prefix contains rows from all subsections earlier than SS-05.39. New BCs added to earlier subsections consume runway; BCs added to SS-05.39 and later (after BC-5.39.010's row) do not. At a realistic production rate of 20–50 new BCs per feature wave, 600 rows of headroom covers 12–30 feature waves of growth before re-exhaustion. This exceeds the time needed to build and ship the deferred structural fix (§Decision 4).

**POLICY 13 BOUNDARY-POLARITY analysis — excluded region for the 20M boundary:**

The 20M budget does NOT cover:

| Excluded scenario | Estimated fuel requirement | Why excluded |
|-------------------|---------------------------|--------------|
| BC-5.39.010 repositioned to row ~1,980 (end of BC-INDEX) | 576,842 bytes prefix → ~33M fuel | Reposition is a user-initiated change; existing headroom gate (§Decision 4) surfaces this before exhaustion |
| Plugin extended to scan all 1,985 BC rows (no early return) | 576,842 bytes → ~33M fuel | Scope extension requires explicit spec change; §Decision 4 deferred fix eliminates this class |
| Multiple BCs scanned (e.g., BC-5.39.010 + 5 sibling BCs) | ~415KB + 5 × row_bytes ≈ ~10M additional | Multi-BC scan is an explicit spec extension; not in BC-5.39.010 current scope |
| Other O(n)-in-input plugins (lessons.md, decision-log.md, burst-log.md, STATE.md validators) | Currently EXHAUSTS the shared global cap for the three largest cycle artifacts — see erratum below | **ERRATUM (v1.3 — F-S2107-P10-006): this row was WRONG when written and is corrected here, not retracted, per POLICY 5 v1.3.5 Part A historical-by-construction discipline.** The original claim ("budgets are independent; ADR-039 Phase 3 covers them") described a FUTURE state (per-plugin `fuel_cap`, not yet implemented — zero matches for `fuel_cap` in `hooks-registry.toml`, confirmed in §Context) as if it were the CURRENT state. It directly contradicts this ADR's own §Decision 2 title ("Raise is global … not per-plugin"): under a global-only raise, every `on_error="continue"` O(n)-in-input plugin shares the SAME 20M ceiling as `validate-cross-site-correspondence` — there is no independence to appeal to until ADR-039 Phase 1 ships. See §Decision 1 Erratum below for the corrected disposition and mandatory BOUNDARY-POLARITY mutant. |

**Mutant proving the excluded region is harmful:** Change `extract_bc_index_version_state` to scan all rows instead of returning early on BC-5.39.010. Full-corpus scan = 576,842 bytes → fuel = 2,585,970 + 53.18 × 576,842 ≈ 33.3M → exhausts the 20M budget. The early-return is load-bearing for the 20M budget to hold. **This mutant demonstrates the `validate-cross-site-correspondence`-specific excluded region (row 1) is harmful if triggered; it does NOT cover row 4 (other O(n) plugins) — see the dedicated POLICY 13 BOUNDARY-POLARITY analysis immediately below, which supplies the mandatory mutant for row 4 that was absent from v1.0–v1.2.**

#### §Decision 1 Erratum (v1.3 — architect, S-21.07 pass-10 fix cascade, closes F-S2107-P10-006)

**The self-contradiction, precisely stated.** §Decision 1 row 4 (as originally written, v1.0–v1.2) claimed other O(n)-in-input plugins have "independent" budgets and are "covered" by a future calibration phase. §Decision 2's own title states the raise mechanism is global, not per-plugin, and §Decision 2 body confirms: "The per-plugin `fuel_cap` registry field … is not yet implemented (no `fuel_cap` field in `hooks-registry.toml`; zero matches verified in §Context)." A budget cannot be simultaneously (a) global/shared (§Decision 2) and (b) independent-per-plugin (§Decision 1 row 4, as originally written). Only one can be true at the current implementation state, and §Decision 2 correctly describes it: (a) is current reality, (b) is the ADR-039 Phase 3 future state this ADR explicitly says has not shipped.

**Empirical corroboration — the excluded region IS currently harmful (POLICY 13 BOUNDARY-POLARITY MANDATE, mandatory table; supplies the mutant absent from v1.0–v1.2):**

| Dimension | Analysis |
|-----------|----------|
| **False-positive class (row 4's original claim)** | "Other O(n)-in-input plugins … budgets are independent; ADR-039 Phase 3 … covers them" — implies these plugins are safely out of scope of this ADR's 20M raise and require no further action. |
| **Can harmful content occupy the excluded region?** | **YES — confirmed empirically, not hypothetically.** `decision-log.md`, `burst-log.md`, and `lessons.md` are exactly the class row 4 dismissed (O(n)-in-input, `on_error="continue"` validators reading `.factory/` cycle artifacts). Fresh literal-shell measurement (architect, 2026-08-13, current corpus — supersedes the pass-10 adversary's 2026-08-09 snapshot, which is now stale in the same direction, i.e. these artifacts have grown further): |
| **Mutant (the missing proof)** | `$ wc -c cycles/v1.0-brownfield-backfill/{burst-log,decision-log,lessons}.md` → `2050525 burst-log.md`, `2106275 decision-log.md`, `1385356 lessons.md`. Applying the adapter-class model this ADR itself derived (`fuel = 29,452 + 27.514 × payload_bytes`, payload = file_bytes + ~51,200-byte `last_assistant_message` overhead, per §Context "perf-fuel-2 measurement result"): burst-log.md → **57.9M fuel (289% of the 20M cap)**; decision-log.md → **59.4M fuel (297% of the 20M cap)**; lessons.md → **39.6M fuel (198% of the 20M cap)**. All three EXCEED the shared global 20M cap set by §Decision 1/§Decision 2 of THIS ADR. The excluded region is not merely theoretically harmful — it is currently, measurably exhausting. |

**Corrected disposition.** Row 4 is corrected (struck through above, not deleted, per historical-by-construction discipline) to state the accurate current fact: these three artifacts are NOT independently budgeted, DO share the global 20M cap this ADR sets, and DO currently exceed it by 1.9×–3.0×. This is not a new problem this erratum introduces — it is a PRE-EXISTING, ALREADY-TRACKED condition. No new tech-debt-register entry is created (Canonical Principle Rule 3 would be violated by doing so); the remediation is already anchored to two live STATE.md Drift Items: `[D-954] decision-log.md >17,000 lines — OPEN 2026-08-04 — WASM validators time out on every edit` and the `lessons.md` size-budget discipline (≤3500 soft / ≤4000 hard per D-442(e), CLAUDE.md "WASM plugin fuel budgets"). This erratum's contribution is narrow and precise: it corrects this ADR's own text so it stops asserting these artifacts are safe when they are not, and supplies the BOUNDARY-POLARITY mutant POLICY 13 requires for any narrowed-scope exclusion claim. It does not expand this ADR's remediation scope beyond `validate-cross-site-correspondence` (§Context "Routing" already correctly deferred the platform-wide multi-plugin benchmark to performance-engineer).

### Decision 2 — Raise is global (`InvokeLimits::default()`), not per-plugin; lifting of BC-5.39.010 fuel_cap prohibition authorized

**Global raise is the only available lever:** The per-plugin `fuel_cap` registry field specified in ADR-035 §Decision 5 and ADR-039 §Decision 2 is not yet implemented (no `fuel_cap` field in `hooks-registry.toml`; zero matches verified in §Context). ADR-039 Phase 1 has not shipped.

**Global raise is safe for fast-running plugins:** Wasmtime fuel is a decrement counter, not pre-allocated memory. A plugin that completes in 50,000 fuel simply decrements to 19,950,000 and terminates. There is no overhead for plugins that do not consume their budget. The epoch deadline (10ms/tick per `build_engine`) independently bounds wall-clock time regardless of fuel cap.

**Prohibited configuration is lifted:** BC-5.39.010 contains (or contained) a normative clause: "A future implementer MUST NOT add a `fuel_cap` entry." That prohibition was based on the premise that `max_bytes` limits keep the plugin within 10M fuel. The measurement falsifies the premise. The prohibition is hereby **explicitly lifted** by this ADR. Route to product-owner: amend BC-5.39.010 to replace the prohibition with:
> Once ADR-039 Phase 1 (registry `fuel_cap` schema extension) ships, a per-plugin `fuel_cap` SHOULD be set for `validate-cross-site-correspondence`, calibrated to p99×1.5 per ADR-039 §Decision 4 Option A (minimum 50M for Phase 3 fail-closed annotation).

**Scope of implementation:** The change is to `crates/factory-dispatcher/src/invoke.rs`, the `InvokeLimits::default()` implementation, `fuel_cap` field: `10_000_000` → `20_000_000`. No registry change required. Route to implementer.

### Decision 3 — Exhaustion visibility: three distinct problem classes with different remedies

The perf-fuel-2 measurement clarifies that "make exhaustion visible" is not a single problem. Three distinct classes require separation because they have different causal paths and different remedies.

#### Class (a) — Silent exhaustion: `on_error = "continue"` plugins

`validate-cross-site-correspondence` and `validate-closes-completeness` are confirmed in this class. Exhaustion path:

1. Plugin exhausts fuel → `TimeoutCause::Fuel` in `handle_plugin_err`
2. `on_error = "continue"`: `block_intent=false exit_code=0`, stdout empty
3. No `plugin.completed` record in JSONL sink
4. Calling agent receives an empty hook result — indistinguishable from "validation passed with zero findings"

**This is the original §Decision 3 problem and it remains real and unresolved.** The fix is observable signaling, not `on_error` escalation.

**Required observable behavior for class (a) (normative, for implementer and test-writer):**

When any plugin invocation produces `TimeoutCause::Fuel`, the dispatcher MUST:

1. **Emit to stderr summary line** (analogous to TD #71's `block_reason` field for blocks): include `fuel_exhausted=true cap=<N> plugin=<name>` in the dispatcher's stderr summary line. Session-visible without JSONL log inspection:
   ```
   factory-dispatcher trace=<UUID> event=PostToolUse tool=Edit host_abi=1
     plugins_run=1 total_ms=1 block_intent=false exit_code=0
     fuel_exhausted=true fuel_exhausted_plugins=validate-cross-site-correspondence
     fuel_cap=20000000 fuel_msg="validation SKIPPED due to fuel exhaustion — integrity not guaranteed"
   ```

2. **Include a structured advisory in the hook result payload**: the `advisories[]` field MUST contain:
   ```json
   {"type": "fuel_exhausted", "plugin_name": "<name>", "fuel_cap": <N>, "validation_skipped": true}
   ```

3. **Emit the advisory log** mandated by ADR-035 §Decision 5:
   ```
   [fuel-exhausted] plugin <name> ran out of fuel after <N> instructions — validation skipped; this is a resource-policy event, not a finding.
   ```

`on_error = "continue"` semantics remain unchanged. PostToolUse hooks run after a write succeeds; the write cannot be reverted. Changing `on_error` would conflate resource-policy failure with validation failure — the defect ADR-035 §Decision 5 identified. The correct fix is observable signaling.

ADR-039 §Decision 5 (M1) near-miss warning is independent and complementary: when `fuel_consumed > 0.9 × cap` on the `Ok` path, M1 fires in advance. Both MUST be implemented; neither replaces the other.

**Route:** Implementer (SS-01 scope) implements behaviors 1, 2, 3 in `crates/factory-dispatcher/src/invoke.rs` `handle_plugin_err` and PostToolUse response assembly. Test-writer adds bats integration test asserting a fuel-exhausting plugin fixture produces non-empty stderr containing `fuel_exhausted=true` and non-empty `advisories[]`, regardless of `on_error` value.

#### Class (b) — Ambiguous cause: `on_error = "block"` plugins (fuel exhaustion vs epoch timeout)

`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, and all other `on_error = "block"` plugins fall here. These ARE already agent-visible — exhaustion produces `block_intent=true exit_code=2`. The remaining problem is cause disambiguation.

`extract_reason_from_outcome` in the dispatcher matches all `PluginResult::Timeout` without inspecting the `cause` field (`TimeoutCause::Fuel` vs `TimeoutCause::Epoch`). Fuel exhaustion and epoch timeout emit an identical stderr message: `block_reason="fail-closed: plugin timed out"`. The `cause` field exists in the internal event log and is silently discarded when building the block reason. Operationally, the two need opposite responses: fuel exhaustion requires a permanent cap raise (or per-plugin cap via ADR-039 Phase 3); epoch timeout requires investigation of a slow or hanging bash script. Conflating them sends operators to the wrong remedy.

**Required fix (normative, for implementer dispatched same session):** `extract_reason_from_outcome` MUST inspect `cause` when building `block_reason`. Fuel exhaustion should produce a distinct message:
```
block_reason="fuel-exhausted: plugin <name> consumed all <N> fuel instructions — raise fuel_cap or await ADR-039 Phase 3 per-plugin calibration"
```
Epoch timeout retains `block_reason="fail-closed: plugin timed out (epoch)"` or similar distinguisher.

**Status:** implementer dispatched by team-lead (same session, 2026-08-08) on the dispatcher branch. This fix is independent of §Decision 3 class (a).

#### Class (c) — PostToolUse cannot revert: independent architectural constraint

For all PostToolUse hooks regardless of `on_error` value: by the time the hook fires, the Edit/Write tool has already committed the change to disk. `block_intent=true exit_code=2` blocks the NEXT tool dispatch, not the current write. This is an architectural property of the harness, not a dispatcher defect. It means loudness improves diagnosis and prevents subsequent operations on an un-validated artifact, but does not prevent the unvalidated write itself.

This constraint is stated here for completeness and to prevent future fix attempts from conflating it with classes (a) or (b). No implementer routing is required for class (c) itself.

### Decision 4 — Correct ADR-035 §Decision 5; discharge D-945

**Correction 1 — O(n) not O(n²) for `validate-cross-site-correspondence`:**
ADR-035 Context mentions "WASI `fd_readdir` ... paginated enumeration is O(n²) in fuel." That claim was about a REJECTED alternative (directory enumeration via `fd_readdir`). The implemented design uses `read_file` for direct file reads — confirmed linear (O(n) in input bytes). Regression R²=0.998790 over 24 measured points. No quadratic coefficient is significant (coefficient 5.42×10⁻⁵ adds +0.075% R²). **The fuel cost of this plugin's scan is linear in input bytes.**

**Correction 2 — ~100KB threshold was speculative; measured threshold is ~415KB:**
ADR-035 §Decision 5 stated: "any hook with `on_error = "block"` that reads `.factory/` artifacts larger than ~100KB is at risk." At 53.18 fuel/byte, 100KB consumes ~5.3M fuel — below the 10M cap. The exhaustion threshold for `validate-cross-site-correspondence` is the prefix length to BC-5.39.010's row: currently ~415KB, consuming ~9.9M of the 10M budget. The ~100KB guidance understated the risk zone by ~4× for this plugin's measured profile. For the general case, the risk threshold at 10M cap and 53.18 fuel/byte is: `(10M - 2,585,970) / 53.18 ≈ 139,388 bytes ≈ 136KB`. The ~100KB figure was within the same order of magnitude but imprecise; the correct general threshold from the model is **~136KB for 10M budget** and **~328KB for 20M budget** at this plugin's marginal cost.

**Correction 3 — The ARCH-INDEX body row for ADR-035 should note the amendment.** The ARCH-INDEX row is updated atomically with this ADR's insertion (see §Amendment to ADR-035).

**D-945 discharge:** Standing drift item "[D-945] ADR-035 §Decision 5 fuel budget advisory — OPEN 2026-07-30 — 'May need revision after S-21.07 benchmarks.'" These are those benchmarks. The fuel budget is raised to 20M (§Decision 1); exhaustion signaling is mandated (§Decision 3); the O(n) measurement is confirmed (Correction 1). **D-945 is DISCHARGED by this ADR.** State-manager should mark this item CLOSED in STATE.md Drift Items when recording the D-NNN for this ADR's burst.

### Decision 5 — Deferred long-term structural fix: targeted row lookup

**Problem:** The 20M budget is a bridge fix. The underlying structural issue is that `validate-cross-site-correspondence` performs a linear scan of the entire BC-INDEX prefix to find BC-5.39.010's row. As the BC-INDEX grows, the prefix grows, and fuel consumption grows linearly until it exhausts the budget. A repositioning (BC-5.39.010 moved earlier) or compaction buys more runway but does not eliminate the ceiling structurally.

**Three structural options analyzed:**

| Option | Description | Cost | Headroom effect |
|--------|-------------|------|-----------------|
| A: Reposition BC-5.39.010 earlier | Move to row ~100 (first SS-05 BC) | Low edit cost, reorders source-of-truth index | Reduces prefix from ~415KB to ~50KB; ~183K rows of headroom at 17,114 fuel/row |
| B: Compact BC-INDEX | Remove stale/deprecated rows; reduce total corpus | Medium effort; periodic maintenance | Proportional reduction; doesn't eliminate ceiling |
| C: Targeted row lookup | Implement `read_file_range(path, offset, len)` host function; plugin reads only BC-5.39.010's row directly | Highest effort; requires dispatcher host-function extension + plugin update | Eliminates linear scan ceiling structurally; fuel becomes O(1) in corpus size |

**Recommendation: Option C (targeted row lookup).** This is the structurally correct fix. Fuel consumption for BC-5.39.010 version extraction should not scale with BC-INDEX corpus size. Option A (reposition) is a viable near-term palliative with low cost — story-writer may consider it as a faster-to-ship intermediate step.

**Concrete dependency for deferral (per CLAUDE.md Rule 3):**

Option C requires:
1. A new `read_file_range(path: &str, offset: u64, len: u64) -> Vec<u8>` host function in the dispatcher's WASM ABI (SS-01 scope, `crates/factory-dispatcher/src/engine.rs` linker registration)
2. A BC-INDEX row-position index file (or equivalent: the WASM plugin needs to know BC-5.39.010's byte offset without scanning the full prefix, e.g., via a pre-computed `bc-index-offsets.jsonl` sidecar or a binary search on a sorted index)
3. BC-5.39.010 spec amendment (PC5/PC6 extraction algorithm updated to use `read_file_range`)
4. The ADR-039 Phase 1 registry `fuel_cap` schema extension should land first (so the per-plugin cap can be set atomically with the restructured plugin)

**Anchor: E-21 W7 — the wave after S-21.11 (W6) merges.** Prerequisites: S-21.10 (W5, ADR-039 Phase 1 registry schema extension) and S-21.11 (W6, per-plugin fuel-cap calibration) must both ship before this story is authored. Story-writer must author a dedicated story under E-21 W7. The story scope: (1) `read_file_range` host function in dispatcher; (2) BC-INDEX row-offset sidecar or binary-search index mechanism; (3) `validate-cross-site-correspondence` plugin Arm1/Arm2 extraction using `read_file_range` instead of linear scan; (4) per-plugin `fuel_cap` set at calibrated value per ADR-039 §Decision 4 Option A (p99×1.5, min 50M). **This ADR provides the scope definition; do not add to tech-debt-register. Story-writer routes the authoring.**

**Why E-21 W7 and not E-20 (new epic):** E-20 was never allocated. Allocating it for a single story with a clear E-21 home would fragment the narrative. The dependency chain is coherent within E-21: S-21.07 (W4) discovered the fuel problem this ADR addresses; S-21.10 (W5) implements the ADR-039 Phase 1 registry schema that is a prerequisite; S-21.11 (W6) implements per-plugin calibration (also a prerequisite); the `read_file_range` story (W7) provides the structural closure. E-21's theme "factory-state-data-loss-hardening" encompasses eliminating silent validation degradation — the exact goal of Option C. E-22 cannot host this work: the human operator dissolved E-22 on 2026-08-08 (sole founding story S-21.12 re-anchored to E-21 W4; `story_count: 0`; anchoring new work to a dissolved epic would violate CLAUDE.md Rule 3's requirement that a deferral must attach to a story or wave that exists).

**Why not Option B (compaction)?** Compaction reduces corpus size but does not eliminate the O(n) ceiling. Each BC added to sections before BC-5.39.010 consumes runway again. Compaction is useful for other reasons (CI performance, WASM fuel for other validators) but is not the correct fix for this specific problem.

**Why not Option A (reposition) as primary?** Reposition does not eliminate the ceiling — it shifts the exhaustion point. If BC-5.39.010 is moved to row ~100, 600 BC additions at the beginning of BC-INDEX refills the runway. Reposition is acceptable as a lower-cost interim step (may be worth a sub-story of Option C or a cheap preparatory edit) but is not the structural fix.

## Rationale

**Why raise the global default rather than wait for ADR-039 Phase 1?**
ADR-039 Phase 1 (per-plugin `fuel_cap` schema extension) has not shipped. The `validate-cross-site-correspondence` plugin will be registered when S-21.07 merges. If the budget is not raised before merge, the plugin will enter production with 4 rows of runway and immediately-silent exhaustion on any BC-INDEX growth. The bridge fix (global 20M raise) is the only available mechanism. ADR-039 Phase 3 will supersede this with a calibrated per-plugin cap; the global default will remain at 20M until that phase ships and the per-plugin cap is set.

**Why 20M and not the ADR-039 §Decision 4 Option A minimum of 50M?**
ADR-039 §Decision 4 Option A (`max(p99×1.5, 50M)`) is for Phase 3 fail-closed calibration, where exhaustion-induced block would prevent writes. For the current advisory-only phase (`on_error = "continue"`), a more conservative raise to 20M is sufficient. 20M provides ~589–614 rows of headroom (12–30 feature waves), which is more than enough time to build and ship Option C. The 50M target should be used when per-plugin caps are implemented under ADR-039 Phase 3.

**Why is loud signaling on `on_error = "continue"` correct if we can't revert the write?**
The inability to revert is why `on_error = "continue"` is the right setting for PostToolUse validators. But "cannot revert" does not mean "should be invisible." Silent validation bypass is CWE-636 (relying on a security control that can be bypassed) — the same defect class ADR-039 §Decision 6 addresses. The calling agent deserves to know that the integrity check was skipped. Observable advisory + advisories[] payload gives the agent and operator the information to decide whether a follow-up validation run is warranted, without falsely blocking the edit.

**Why is `on_error = "continue"` preserved rather than escalated to block?**
Escalating to `on_error = "block"` would make the PostToolUse hook block the NEXT dispatch on exhaustion — which makes future operations impossible without fixing the budget, even for edits unrelated to BC correspondence. This is the incorrect behavior: fuel exhaustion is a resource-policy failure, not a validation failure. The distinction (established in ADR-035 §Decision 5) remains correct. Loud advisory signaling (§Decision 3) is the right escalation.

## Amendment to ADR-035 §Decision 5

ADR-035 §Decision 5 is amended as follows. Amend the ADR-035 file with the AMENDED annotation block below.

**ADR-035 §Decision 5 AMENDED note (v1.1):**

> **AMENDED 2026-08-08 (v1.1 — architect, ADR-042):**
>
> 1. **O(n) confirmed, O(n²) concern resolved.** The O(n²) concern in §Context was about directory enumeration via `fd_readdir` (a rejected alternative). The implemented `read_file` linear scan is confirmed **O(n) in input bytes**, R²=0.998790 over 24 measured points (S-21.07 benchmarks, 2026-08-08). No quadratic coefficient is significant.
>
> 2. **~100KB threshold corrected to ~136KB (10M) / ~328KB (20M).** ADR-035 v1.0 stated "~100KB at risk." At 53.18 fuel/byte (measured marginal), the breakeven is (budget − 2,585,970) / 53.18: ~139KB at 10M budget, ~327KB at 20M budget. The ~100KB figure understated the risk zone. The measured exhaustion threshold for `validate-cross-site-correspondence` was ~415KB prefix (the BC-INDEX.md prefix up to BC-5.39.010's body-table row, ~415KB).
>
> 3. **Fuel budget raised to 20M per ADR-042 §Decision 1.** The global `InvokeLimits::default()` `fuel_cap` has been raised from 10M to 20M. The "no registry change needed for this new hook" note in ADR-035 v1.0 is superseded: per-plugin `fuel_cap` SHOULD be set once ADR-039 Phase 1 (registry schema extension) ships (ADR-042 §Decision 2).
>
> 4. **D-945 discharged.** Standing drift item "[D-945] ADR-035 §Decision 5 fuel budget advisory — OPEN 2026-07-30 — 'May need revision after S-21.07 benchmarks.'" These benchmarks confirm the model and the budget has been raised. D-945 is DISCHARGED. See ADR-042 §Decision 4 (Correction 3).

## Consequences

### Positive

- `validate-cross-site-correspondence` has ~589 rows of headroom from the current BC-INDEX position at the measured marginal cost — enough for 12–30 feature waves without re-exhaustion.
- Fuel exhaustion is no longer silent: the calling agent sees a session-visible `fuel_exhausted=true` signal in the dispatcher's stderr summary line and in the `advisories[]` payload.
- D-945 is discharged cleanly, with the actual measurement evidence that was explicitly requested when the drift item was opened.
- ADR-035 §Decision 5's O(n) confirmation removes a speculative concern about superlinear fuel costs for this plugin class.
- The lifted BC-5.39.010 `fuel_cap` prohibition clears the path for ADR-039 Phase 3 per-plugin calibration.
- Deferred structural fix (Option C) is specified with enough detail for story-writer to author a story without further architectural adjudication.

### Negative / Trade-offs

- Global raise from 10M to 20M affects all plugins, not just `validate-cross-site-correspondence`. Fast-running plugins see no behavioral change; O(n)-in-input validators for other large artifacts (lessons.md, STATE.md, ARCH-INDEX.md) gain more headroom but do not gain structural protection. However, the raise may still be insufficient for `validate-factory-path-root`, `validate-input-hash`, and `validate-template-compliance` against ARCH-INDEX.md at 325KB: those plugins exhausted the 10M budget on every ARCH-INDEX.md edit during this authoring session, and their fuel models are unmeasured. Whether 20M is sufficient for them is unknown — see §Context "Platform-wide scope" note. A performance-engineer dispatch is needed; a follow-up ADR may be required if any of those three still exhaust at 20M.
- The 20M global cap is a bridge fix, not the structural solution. Option C (targeted row lookup) remains necessary for long-term correctness.
- The deferred Option C requires a new host function (`read_file_range`) in the dispatcher WASM ABI — a non-trivial interface addition that requires thorough testing.
- ADR-039 Phase 3 fail-closed calibration still requires per-plugin `fuel_cap` set to p99×1.5 minimum 50M. The 20M global cap does not satisfy ADR-039 Phase 3 for fail-closed validation; it only bridges to that phase.

## Downstream Routing

| Artifact | Change | Route |
|----------|--------|-------|
| `crates/factory-dispatcher/src/invoke.rs` | `InvokeLimits::default()` `fuel_cap`: `10_000_000` → `20_000_000` | implementer |
| `crates/factory-dispatcher/src/invoke.rs` + PostToolUse response path | Implement loud exhaustion (§Decision 3 class a): `fuel_exhausted=true` in stderr summary line; `advisories[]` entry; advisory log per ADR-035 §Decision 5 mandate | implementer |
| `crates/factory-dispatcher/src/invoke.rs` `extract_reason_from_outcome` | Fuel-vs-epoch cause disambiguation (§Decision 3 class b): inspect `TimeoutCause` when building `block_reason`; emit distinct messages for fuel exhaustion vs epoch timeout | implementer (dispatched 2026-08-08 on dispatcher branch) |
| bats integration test (new) | Assert: plugin that exhausts fuel produces non-empty dispatcher stderr line containing `fuel_exhausted=true` and non-empty `advisories[]` entry, regardless of `on_error` value | test-writer |
| `BC-5.39.010.md` | Lift `fuel_cap` prohibition: replace "MUST NOT add a `fuel_cap` entry" with "SHOULD set per-plugin `fuel_cap` once ADR-039 Phase 1 ships, calibrated to p99×1.5 per ADR-039 §Decision 4 Option A" | product-owner |
| ADR-035 §Decision 5 | Add AMENDED v1.1 annotation per §Amendment to ADR-035 §Decision 5 above; update `version:` to `"1.1"`, `last_amended:` | architect (this burst, same commit) |
| ARCH-INDEX.md | Insert ADR-042 row; ADR-035 row AMENDED annotation; frontmatter `version: "3.47"` → `"3.48"`, `total_adrs: 41` → `42`, `last_amended` chain prepend | architect (this burst, same commit) |
| STATE.md Drift Items | Mark D-945 CLOSED; record that BC-5.39.010 `fuel_cap` prohibition lifted by ADR-042 §Decision 2 | state-manager (D-NNN burst) |
| Deferred structural fix story | Option C: `read_file_range` host function + BC-INDEX row-offset mechanism + plugin update + per-plugin `fuel_cap` calibration | story-writer (E-21 W7, after S-21.10 and S-21.11 merge) |
| Platform-wide fuel measurement | Benchmark `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance` against ARCH-INDEX.md / STATE.md / lessons.md / BC-INDEX.md at current sizes; determine whether 20M is sufficient or a follow-up ADR is needed | performance-engineer |

## Alternatives Considered

**Keep 10M budget, implement loud signaling only (prior ruling):**
The human's initial 2026-08-08 ruling was "fail loud only, keep budget as-is" based on a belief that ~110 rows of runway remained. The measurement falsified this (~4 rows actual). The human reversed the ruling on seeing the measurement. This alternative is rejected by human direction.

**Raise to 50M (ADR-039 §Decision 4 Option A minimum):**
Consistent with ADR-039 Phase 3 calibration guidance. Rejected for this bridge fix because (a) 50M is a per-plugin Phase 3 target, not a global default; (b) 20M provides adequate bridge runway; (c) premature convergence to the Phase 3 target number would obscure the distinction between the bridge fix and the fully-calibrated Phase 3 state. Phase 3 should own the 50M derivation against its own p99 corpus.

**Option A (reposition BC-5.39.010 earlier) as the structural fix:**
Low cost, fast to ship. Rejected as primary structural fix because it doesn't eliminate the O(n) ceiling — it shifts the exhaustion point. Acceptable as an interim step (story-writer may evaluate whether a cheap pre-story reposition is worth doing before Option C). Not recommended as the permanent fix.

**Option B (compact BC-INDEX) as the structural fix:**
Useful but doesn't eliminate the ceiling. BC-INDEX compaction serves other purposes (CI performance, WASM fuel for all validators) but is not targeted at this problem. Periodic compaction as ongoing maintenance is appropriate; not as the answer to this specific fuel-ceiling problem.

**Change `on_error` to `"block"` on fuel exhaustion for this plugin:**
Would surface exhaustion as `block_intent=true exit_code=2`, making it visible. Rejected. PostToolUse cannot revert a write. Blocking the next dispatch conflates resource-policy failure with validation failure. ADR-035 §Decision 5 correctly identifies `TimeoutCause::Fuel` as a resource-policy error; the correct fix is observable advisory emission (§Decision 3), not severity escalation.

## Source / Origin

- Performance-engineer measurement session 2026-08-08, `/tmp/fuel-measure-01/`, binary from `.worktrees/fuel-loud` commit `fbb9dcb6` (231,661 bytes). Caveat: production S-21.07 WASM binary (`b0373eb2`, 231,661 bytes) is a different build; figures comparable in magnitude, not identical.
- Human ruling (2026-08-08): raise budget + make exhaustion loud + deferred structural fix with concrete anchor. Supersedes the earlier 2026-08-08 "fail loud only, keep budget as-is" ruling (which was based on a falsified ~110-row runway figure).
- ADR-035 §Decision 5 (2026-07-30): fuel error taxonomy; advisory emission mandate (not yet implemented); "~100KB at risk" guidance (corrected in §Decision 4); D-945 drift item opened.
- ADR-039 §Decision 2/4 (2026-08-06): per-plugin `failure_policy` and `fuel_cap` calibration phases; 50M minimum for Phase 3 fail-closed annotation.
- `InvokeLimits::default()` behavioral anchor in `crates/factory-dispatcher/src/invoke.rs`: `fuel_cap: 10_000_000` current value — route to implementer per §Downstream Routing.
- `extract_bc_index_version_state` behavioral anchor in `crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs`: early-return scan that halts at BC-5.39.010's row — this early return is load-bearing for the 20M budget (§Decision 5 mutant analysis).
- BC-INDEX.md corpus metrics (literal shell, 2026-08-08): 576,842 bytes, 2,579 lines, BC-5.39.010's body-table row located via `grep -n '\[BC-5\.39\.010\]'`, ~415,278 byte prefix.
- **F-S2107-P10-006 (HIGH, this ADR's own §Decision 1 vs §Decision 2 self-contradiction):** row 4 of the §Decision 1 excluded-region table asserted "independent budgets … covered" for other O(n) plugins while §Decision 2 established the raise as global-only; empirically falsified by fresh measurement of `decision-log.md`/`burst-log.md`/`lessons.md` (1.9×–3.0× over the 20M cap). Addressed by §Decision 1 Erratum (v1.3, this amendment).
- **F-S2107-P10-008 (MEDIUM, TD-VSDD-091):** volatile `BC-INDEX.md` line-number pins ("line 1464") in five narrative sites. Addressed by replacing with stable body-table-row anchor form (v1.3, this amendment); captured literal-shell evidence blocks (which legitimately contain the grep-derived line number as verbatim stdout) were left unchanged.

## Status

PROPOSED 2026-08-08; ADR-042 v1.0 (architect; human ruling 2026-08-08 authorized the substance: raise fuel budget + make exhaustion loud + deferred structural fix). AMENDED 2026-08-08; ADR-042 v1.1 (architect): E-22→E-21 W4 re-anchor; platform-wide fuel exhaustion scope documented. AMENDED 2026-08-08; ADR-042 v1.2 (architect): perf-fuel-2 adapter-class measurement integrated; §Decision 1 12M floor + 92% margin stated; §Decision 3 re-scoped into three problem classes. AMENDED 2026-08-13; ADR-042 v1.3 (architect; S-21.07 pass-10 ADR-anchored fix cascade): §Decision 1 Erratum added — corrects the row-4/§Decision-2 self-contradiction (F-S2107-P10-006) with a POLICY 13 BOUNDARY-POLARITY mandatory mutant table and fresh literal-shell corroboration (burst-log.md/decision-log.md/lessons.md at 289%/297%/198% of the 20M cap under this ADR's own adapter-class model); five volatile `BC-INDEX.md` line-number pins replaced with stable body-table-row anchors per TD-VSDD-091 (F-S2107-P10-008 ADR-042 leg). No change to §Decision 1's fuel-budget arithmetic (12M floor / 20M chosen / 92% margin, all `validate-cross-site-correspondence`-specific) or §Decision 2's global-raise mechanism — only the excluded-region characterization in row 4 was factually wrong and is corrected. AMENDED 2026-08-13; ADR-042 v1.4 (architect; S-21.07 pass-10 fix cascade, body-vs-frontmatter reconciliation pass, pre-adversary-pass-11): `## Status` section's "RATIFICATION STATUS — NEEDS HUMAN ADJUDICATION" and "Recommendation to human" paragraphs reconciled to present-tense — human ratification OCCURRED 2026-08-13 (D-992); frontmatter now carries `status: active`/`ratified: "2026-08-13"`. Historical text preserved with a superseding note, not deleted.

> **SUPERSEDED 2026-08-13 (v1.4 amendment, body-vs-frontmatter reconciliation, S-21.07
> pass-10 fix cascade):** the two paragraphs immediately below this note ("RATIFICATION
> STATUS — NEEDS HUMAN ADJUDICATION" and "Recommendation to human") described the state as
> of v1.3 authoring (2026-08-13, earlier same day) and were not revisited when ratification
> occurred later the same day. **Human ratification HAS NOW OCCURRED**: D-992 (S-21.07
> pass-10 fix burst, 2026-08-13) — the human answered "Ratify both now" for ADR-041 and
> ADR-042 together, exactly the disposition the v1.3 §Status recommendation proposed.
> Frontmatter now carries `status: active`, `ratified: "2026-08-13"`, and a
> `ratification_note` recording the D-992 event. The proposed-ADR-governing-live-gate
> contradiction the v1.3 paragraphs flagged is RESOLVED — `InvokeLimits::default() fuel_cap`
> and the §Decision 3 loud-exhaustion signaling now run under a ratified, `status: active`
> ADR, not a `proposed` one. The v1.3 text is preserved below verbatim for historical
> continuity (it accurately reflects the ADR's pre-ratification state and the reasoning that
> led to ratification).

**RATIFICATION STATUS AS OF v1.3 AUTHORING (2026-08-13, HISTORICAL — F-S2107-P10-005,
RESOLVED BY D-992 LATER THE SAME DAY).** Frontmatter `status:
proposed`, no `ratified:` field, despite `decision-log.md` D-964(b) narrative stating "ADR-042
v1.2 ratified" and STATE.md/burst-log echoing that language in six subsequent bursts
(D-965/966/967/968/969/970 fuel-exhaustion-telemetry notes). Architect does not have
unilateral authority to self-declare this ADR ratified — the ADR-040 precedent (D-965's
premature "ratified" label, later found PROCURED-ON-MISCHARACTERIZATION at F-S2107-P10-003,
requiring a genuine fresh human ratification event at D-970 with an explicit
`ratification_note`) establishes that narrative "ratified" language in a decision-log burst
summary is not equivalent to actual human ratification of the ADR document. Distinct from
ADR-040's case: here there IS a genuine, correctly-attributed human ruling on the ADR's
*substance* (§Context: "The ruling this ADR implements (human-authorized, 2026-08-08): Raise
the fuel budget AND make exhaustion loud"). What is missing is the explicit, dated
ratification of the ADR *document* — the `status: proposed → active` / `ratified: <date>`
frontmatter transition with a `ratification_note` comparable to ADR-040 v1.12's.

**Recommendation to human (as of v1.3, historical — ACTED ON at D-992):** ratify ADR-042 v1.3 (frontmatter `status: proposed → active`,
`ratified: <date-of-explicit-confirmation>`) now that F-S2107-P10-006's self-contradiction is
corrected, since (a) the underlying 2026-08-08 ruling on substance is already genuine and
on record, (b) the fuel-budget arithmetic (§Decision 1) has not changed, only the row-4
excluded-region text, and (c) POLICY 16's ALLOCATOR-CEILING gate (ADR-041, same F-005
disposition) is already running as a live blocking pre-allocation gate in every burst without
its governing ADR's frontmatter reflecting ratification — the proposed-ADR-governing-live-gate
contradiction is real and growing more awkward with each burst it persists. If the human
instead determines the 2026-08-08 substance ruling did NOT constitute ADR-level ratification
intent, the alternative disposition is: keep `status: proposed`, and route to devops-engineer/
state-manager to gate `InvokeLimits::default() fuel_cap` and POLICY 16's ALLOCATOR-CEILING
enforcement on explicit ratification before further reliance is placed on either.

**Disposition (v1.4): the human chose the recommended disposition.** D-992 (2026-08-13)
ratified ADR-042 and ADR-041 together in one pass, per the recommendation above. This ADR's
ratification status is CLOSED — see frontmatter and the superseding note at the top of this
`## Status` section.
