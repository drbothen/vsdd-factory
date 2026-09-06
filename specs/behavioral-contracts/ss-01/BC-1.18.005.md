---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: product-owner
timestamp: 2026-09-05T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-051-layer-2-two-mechanism-size-triggered-shard-rotation-append-logs-and-bc-index-sharding.md
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/architecture/decisions/ADR-042-validate-cross-site-correspondence-fuel-budget-raise-and-loud-exhaustion-signaling.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f1-delta-analysis.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "b4d181e"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-043"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.005: Byte-Size-Denominated Shard-Cap Formula and Native Deterministic Size-Trigger (Provisional Constants Pending F4 Harness Calibration)

## Description

A native (non-WASM) dispatcher function computes, for every registered sharded artifact, a
`shard_cap_bytes` ceiling from four calibrated inputs (`PRACTICAL_FUEL_CEILING`,
`WORST_CASE_FUEL_PER_BYTE`, `MAX_SINGLE_RECORD_BYTES`, `SAFETY_MARGIN`) and performs a `stat()`-only
byte-size check — never a fuel-budgeted content read — before every `Edit`/`Write`/`MultiEdit`
against a matched artifact. This is the deterministic, mechanistic size-trigger AC-001/AC-002 of
S-25.02 require: no LLM-side awareness of shard size is required or permitted, and the check itself
cannot exhaust a fuel budget because it has none (native code, not a WASM plugin). This BC owns
BOTH trigger shapes the native gate dispatches on: mechanism A's byte-size-denominated `stat()`-only
trigger (Postconditions 1-7) AND mechanism B1's item-count-denominated trigger (Postcondition 8,
added in this fix-burst per ADR-051 v1.1 Decision 1's "Trigger-shape dispatch" amendment,
F-S2502-F2-005).

## Preconditions

1. The dispatcher's PreToolUse handling path for `Edit`/`Write`/`MultiEdit` tool calls is reached
   (ADR-051 Decision 1 placement: before the registry-driven plugin loop).
2. A `[[shard]]` config table (`hooks-registry.toml` or a sibling `shard-config.toml`, TBD at F4)
   exists naming the target artifact's stem, its cap-formula inputs, its current `shard_cap_bytes`,
   and (per Postcondition 8) a `shape` field declaring the artifact's trigger shape (`"flat"` or
   `"frontmatter-changelog-array"`).
3. The tool call's target path resolves against an entry in the `[[shard]]` config (if it does
   not, this BC's check is a no-op — see Postcondition 1).

## Postconditions

1. **Zero-cost bypass for unmatched paths.** If the tool call's target path does not match any
   `[[shard]]` config entry, the native check returns `Continue` immediately, performing no
   `stat()` call and no arithmetic. This preserves zero added latency for the ~99% of
   `Edit`/`Write`/`MultiEdit` calls Layer 2 does not touch (ADR-051 Decision 1 step 1).

2. **`stat()`-only size read — no file content enters WASM memory or a fuel budget.** For a
   matched path, the check reads ONLY the current shard's byte size via filesystem metadata
   (`stat()`/`metadata()`), never reading file content into memory for the size determination.
   This is native dispatcher-process code with no WASM sandbox and no fuel budget of its own —
   the postcondition that must hold is that the size-check itself is structurally incapable of
   producing a `plugin.timeout`/`plugin.indeterminate` event, because it is not a WASM plugin
   invocation at all (ADR-051 §Rationale — "Why native, not WASM"). This postcondition applies to
   the `"flat"` artifact shape only — Postcondition 8 specifies the DIFFERENT, more-than-`stat()`
   read cost for the `"frontmatter-changelog-array"` shape.

3. **CORRECTED (fix-burst pass-2, F-P2-002, HIGH) — projected size is computed PER-TOOL-SEMANTICS,
   never by a single `current_shard_bytes + payload_bytes` formula applied uniformly to every
   tool.** The WITHDRAWN formula (`current_shard_bytes + payload_bytes` for every tool, including
   `Write`) is UNSOUND for `Write`: a `Write` call REPLACES a file's entire content — `content`'s
   own length already IS the file's post-apply size — so adding `current_shard_bytes` on top
   double-counts the shard's own pre-existing bytes on top of a payload that already represents the
   complete post-apply file, over-triggering rotation on ordinary same-size-or-shrinking full-file
   `Write` calls. The corrected, tool-discriminated formula:
   - **`Write`:** `projected_size = len(content)` alone. `current_shard_bytes` is NOT added — the
     `content` parameter is the file's complete post-apply state, not a delta.
   - **`Edit`/`MultiEdit`:** `projected_size = current_shard_bytes + net_delta_bytes`, where
     `net_delta_bytes` is the net length delta — `len(new_string) - len(old_string)` — for `Edit`,
     or the SUM of each individual edit's net delta for `MultiEdit`. This leg is UNCHANGED from the
     original formula — `Edit`/`MultiEdit` mutate existing content in place, so the delta-against-
     current-size model was always correct for these two tools; only the `Write` leg required
     correction.
   Never a re-read of the file after a hypothetical apply, for either tool class. If
   `projected_size <= shard_cap_bytes`, the check returns `Continue` (no roll). If
   `projected_size > shard_cap_bytes`, the check triggers the roll-before-write sequence
   specified in BC-1.18.006 (this BC does not itself perform the roll or the block — it is the
   formula-and-trigger boundary; BC-1.18.006 owns the observable roll/block outcome).

4. **The cap formula is byte-size-denominated, never line-count-denominated.**
   `shard_cap_bytes <= (PRACTICAL_FUEL_CEILING / WORST_CASE_FUEL_PER_BYTE) -
   MAX_SINGLE_RECORD_BYTES - SAFETY_MARGIN`. A line-count proxy (e.g. the D-442(e) STATE.md
   soft/hard line-budget convention) MUST NOT be substituted for this formula on any Layer-2
   sharded artifact: `lessons.md` is a directly-measured counter-example (1,646/3,500 lines —
   comfortably under its own D-442(e) line budget — while carrying 234,731 bytes, per the F1
   delta analysis), and BC-INDEX.md carries a single 16,521-byte physical line (per ADR-051
   direct measurement) that a naive per-line cap would not catch. Every `shard_cap_bytes`,
   `current_shard_bytes`, `payload_bytes`, and `projected_size` value in this BC's postconditions
   is a byte count. This postcondition governs the `"flat"` shape's byte-size trigger; it does NOT
   govern mechanism B1's item-count trigger (Postcondition 8), which is deliberately NOT
   byte-size-denominated for its rotation decision.

5. **Per-artifact cap is the MINIMUM across every Cohort B validator that reads that artifact
   (Cross-Validator Minimum Rule).** `burst-log.md`'s effective `shard_cap_bytes` is
   `MIN(cap_for(validate-burst-log), cap_for(regression-gate), cap_for(convergence-tracker))`
   because ADR-047 §8a's Cohort B table names all three as readers of `burst-log.md`.
   `decision-log.md`, `lessons.md`, and `session-checkpoints.md`'s effective caps are
   `MIN(cap_for(regression-gate), cap_for(convergence-tracker))` only — `validate-burst-log` does
   not read these three artifacts, so its (possibly tighter) per-plugin cap is not applied to
   them. A single global cap across all four mechanism-A artifacts MUST NOT be used, since it
   would be needlessly conservative for artifacts only two of the three Cohort B validators read
   (ADR-051 Decision 2 step 5).

6. **Constants are PROVISIONAL until the F4 synthetic calibration harness runs; the formula shape
   is locked now, the numbers are not.** The current provisional values (ADR-051 Decision 2,
   grounded in ADR-042's measured `fuel = 2,585,970 + 53.18 × payload_bytes` linear model and
   direct 2026-09-05 byte measurements) are:

   | Constant | Provisional value | Locked at F4? |
   |---|---|---|
   | `PRACTICAL_FUEL_CEILING` (today, `DEFAULT_FUEL_CAP=10M`-effective) | 8,000,000 | YES — replace the 80% haircut with the harness's measured "reliably completes" percentile |
   | `PRACTICAL_FUEL_CEILING` (post rc.24, `DEFAULT_FUEL_CAP=20M`, informational only) | 16,000,000 | YES — recompute once rc.24 is confirmed live at the operator level; re-run the harness, do not simply double |
   | `WORST_CASE_FUEL_PER_BYTE` | 106.36 (2× ADR-042's measured 53.18 fuel/byte coefficient, a conservative floor from a comparable legacy-bash-adapter-routed validator) | YES — per-plugin measured value from the harness |
   | `MAX_SINGLE_RECORD_BYTES` | 16,384 (64% margin over the 9,987-byte largest single physical line measured in the active cycle's four append-log files, 2026-09-05) | YES — full-repository sweep at F4 |
   | `SAFETY_MARGIN` | 8,192 (buffer for shard-index-entry + shard-header overhead; not yet measured against the real index schema's per-entry byte cost) | YES — recompute once BC-1.18.006's shard-index schema's real per-entry size is known |

   Illustrative worked example (NOT the locked value): today's ceiling yields
   `(8,000,000 / 106.36) - 16,384 - 8,192 = 50,640`, rounded down to a provisional **49,152 bytes
   (48 KiB)** cap. This BC's implementation MUST treat these five constants as configuration
   inputs to the formula (not hardcoded into the check's logic), so the F4 harness can replace
   them without a code change to the check itself.

7. **Calibration harness ownership and re-run trigger.** The F4 synthetic calibration harness
   (owned by `performance-engineer`, co-run with `implementer`) that produces the final, locked
   constants is executed once before this BC's postconditions are treated as final, and re-run
   whenever `DEFAULT_FUEL_CAP` changes at the operator level (ADR-051 Decision 2). Harness design:
   adversarially-shaped synthetic fixtures at a geometric size series, dispatched directly against
   each of the three Cohort B plugins in isolation, fuel captured via `VSDD_SINK_FILE`, fit for
   linearity/superlinearity exactly as ADR-042 did; if superlinearity is found,
   `WORST_CASE_FUEL_PER_BYTE` MUST be the LOCAL marginal rate at the largest tested size, never the
   global average slope (ADR-051 Decision 2 step 4).

8. **Item-count-denominated trigger for the `"frontmatter-changelog-array"` artifact shape
   (mechanism B1; added F-S2502-F2-005 fix-burst, ADR-051 v1.1 Decision 1 amendment).** This BC
   owns BOTH trigger shapes the SAME native gate dispatches on, distinguished by the `[[shard]]`
   config entry's `shape` field:
   - **Shape declaration.** `shape = "flat"` selects Postconditions 1-7's `stat()`-only byte-size
     trigger. `shape = "frontmatter-changelog-array"` selects THIS postcondition's item-count
     trigger — the gate dispatches to the shape-appropriate check based on this field alone; both
     shapes share the SAME config-match/no-match entry point (Postcondition 1's zero-cost bypass
     applies identically to both).
   - **Read cost — CORRECTED cold/steady-state split (fix-burst pass-2, F-P2-007, MEDIUM).** The
     read-cost claim below is TRUE ONLY IN STEADY STATE (after at least one rotation has occurred)
     and is FALSE at cold start; the two states MUST be characterized separately, never collapsed
     into a single unqualified "bounded" claim:
     - **Cold state (pre-BC-1.18.012 migration).** Direct measurement (2026-09-05) shows
       `BC-INDEX.md`'s `changelog:` sequence, which has never been rotated, holds approximately
       1,997 items across 177,305 bytes of frontmatter. The FIRST trigger evaluation against this
       state must parse and count all ~1,997 items — a finite, single-file read, but NOT bounded
       relative to N in any meaningful sense (it is ~40x the illustrative `N≈50` retention target).
       This is a mischaracterization to correct, not a fuel-budget hazard (the check is native code
       with no fuel budget), but it is genuinely NOT "bounded" in the sense Postcondition 8's
       steady-state claim below uses that word. BC-1.18.012 (the governed one-time B1 changelog
       backfill migration) exists specifically to eliminate this cold state before the ongoing
       per-write gate (BC-1.18.009) is treated as steady-state-bounded.
     - **Steady state (post-BC-1.18.012, and after every subsequent rotation).** The trigger check
       parses the target file's frontmatter far enough to count the existing `changelog:`
       sequence's items — a GENUINELY BOUNDED read: the live sequence is capped at `<= N` items by
       this same mechanism after BC-1.18.012's one-time migration and every prior rotation, per
       BC-1.18.009's corrected contract, so this is never an unbounded-growth read once steady state
       is established. This is MORE than a `stat()` call (it requires reading and lightly
       parsing frontmatter content) but remains native, fuel-budget-free dispatcher code, not a WASM
     plugin invocation — the "why native, not WASM" rationale (Postcondition 2) applies identically
     to this shape.
   - **Trigger condition.** `current_item_count + 1 > N` (a config value, per BC-1.18.009
     Postcondition 1) — NEVER a byte-size comparison for this shape. `shard_cap_bytes`
     (Postcondition 4's byte-size formula) still bounds `BC-INDEX.md`'s TOTAL byte footprint as a
     whole-artifact concern, but the ROTATION decision within this shape is item-count-based only.
   - **Ownership.** Both trigger shapes are specified as THIS BC's postconditions — there is no
     separate, competing trigger-owning BC. BC-1.18.009 owns the observable rotate/block-and-retry
     OUTCOME once this shape's trigger fires (the same division of responsibility Postcondition 3
     already establishes between this BC's trigger and BC-1.18.006's outcome for the `"flat"`
     shape).
   - **Rotation-target config, NEW (fix-burst pass-3, F-P3-005, ADR-051 v1.3 Decision 14).** A
     sibling config value, `low_water_mark`, is declared alongside `N` in the SAME `[[shard]]`
     config entry (`"frontmatter-changelog-array"` shape only) — it is the rotation TARGET
     BC-1.18.009's gate trims the live `changelog:` sequence down to, NEVER `N` itself and NEVER a
     hardcoded `N-1`. **Default:** `floor(N/2)` when the field is omitted. **Fail-loud validation
     constraint:** `0 <= low_water_mark < N`, enforced at config-load time — a malformed value
     (`low_water_mark >= N`, including the degenerate `N-1`, or a negative value) is NEVER silently
     clamped or silently defaulted around; the config is treated as malformed and the check returns
     `HookResult::Error` (mirroring EC-009's "no silent default for malformed shape" posture,
     extended to this field). `N` remains the UNCHANGED trigger threshold
     (`current_item_count + 1 > N`, above) — this BC owns declaring/validating BOTH `N` and
     `low_water_mark`; BC-1.18.009 owns what its rotation step DOES with `low_water_mark` once the
     trigger fires (the same trigger-vs-outcome division this bullet's own "Ownership" text already
     establishes for `N`). Rationale: trimming to exactly `N-1` on every rotation left the live
     sequence back at the trigger boundary after the very next prepend, causing a block+retry
     round-trip on essentially every subsequent write; `low_water_mark` amortizes rotation to once
     per `N - low_water_mark` writes (ADR-051 §Decision 14).

## Invariants

1. **The size check is native code, never a WASM plugin.** No `[[hooks]]` registry entry, no
   `.wasm` binary, and no `HOST_ABI_VERSION` bump implements this BC's check. It is invoked from
   inside `crates/factory-dispatcher/src/shard_manager.rs` (new module), called from the
   dispatcher's own PreToolUse handling path, architecturally analogous to the existing native
   `block_if_marker_check` precedent (`indeterminate_marker.rs`, consulted from `executor.rs`).

2. **The cap value is always expressed and compared in bytes.** No code path in this BC's
   implementation may substitute a line count, a character count under a non-byte encoding
   assumption, or any other non-byte unit for `shard_cap_bytes`, `current_shard_bytes`,
   `payload_bytes`, or `projected_size`. This invariant governs the `"flat"` shape only;
   Postcondition 8's item-count trigger is explicitly exempt (it is item-count-denominated by
   design, not a violation of byte-denomination).

3. **Unmatched paths never pay a `stat()` cost.** The config-match check (Postcondition 1) MUST
   occur before any filesystem `stat()` call, so that Layer 2 imposes zero measurable per-call
   overhead on the ~99% of `Edit`/`Write`/`MultiEdit` calls that do not target a sharded artifact.

4. **The formula's four inputs are configuration, not embedded constants.** `PRACTICAL_FUEL_CEILING`,
   `WORST_CASE_FUEL_PER_BYTE`, `MAX_SINGLE_RECORD_BYTES`, and `SAFETY_MARGIN` are read from the
   `[[shard]]` config (or the shard-index TOML per BC-1.18.006 Decision 4's schema), never
   hardcoded as Rust constants inside `shard_manager.rs` — this is what makes the F4 harness's
   recalibration a config change, not a code change. **Extended (fix-burst pass-3, F-P3-005):** `N`
   and `low_water_mark` (Postcondition 8's item-count-shape config pair) are likewise always
   configuration, never embedded constants or a derived-and-hardcoded value — `low_water_mark`'s
   default (`floor(N/2)`) is computed at config-load time when the field is omitted, never baked
   into `shard_manager.rs` as a fallback constant.

5. **The `shape` field is read once per config entry, never inferred from the target path's
   content or extension.** Shape dispatch (Postcondition 8) is a config-declared property of the
   `[[shard]]` entry, not a runtime heuristic over the file's actual structure — this keeps the
   dispatch itself a cheap, deterministic config lookup rather than a content-sniffing operation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tool call target path does not match any `[[shard]]` config entry | `Continue` immediately; no `stat()` call; zero overhead (Postcondition 1) |
| EC-002 | `projected_size` exactly equals `shard_cap_bytes` (boundary, not exceeding) | `Continue` — no roll (Postcondition 3's `<=` comparison is inclusive of the boundary) |
| EC-003 | `projected_size` exceeds `shard_cap_bytes` by exactly 1 byte | Roll triggers (Postcondition 3's `>` comparison; see BC-1.18.006 for the observable roll/block sequence) |
| EC-004 | The matched artifact's current shard does not yet exist on disk (first write ever) | `current_shard_bytes` treated as 0; `projected_size = payload_bytes`; compared normally against `shard_cap_bytes` |
| EC-005 | `MultiEdit` with multiple edit blocks, some net-negative (deletions), some net-positive | `payload_bytes` is the SUM of each edit's net length delta (may be negative overall — a net-shrinking `MultiEdit` never triggers a roll even if individual edit blocks are large) |
| EC-006 | The harness (F4) finds a Cohort B plugin's fuel-per-byte relationship is superlinear, not linear | `WORST_CASE_FUEL_PER_BYTE` for that plugin MUST use the local marginal rate at the largest tested fixture size, not the global average slope (Postcondition 7) |
| EC-007 | `DEFAULT_FUEL_CAP` changes at the operator level after F4 lock (e.g., rc.24 ships 20M) | `PRACTICAL_FUEL_CEILING` and all downstream `shard_cap_bytes` values MUST be recomputed via a harness re-run, not a naive proportional scale (Postcondition 7) |
| EC-008 | A `[[shard]]` config entry for `BC-INDEX.md` declares `shape = "frontmatter-changelog-array"` and the live `changelog:` sequence is at exactly N items | Postcondition 8's item-count trigger fires (`current_item_count + 1 = N+1 > N`); BC-1.18.009's rotate-then-block-and-retry outcome applies, NOT this BC's own byte-size roll path |
| EC-009 | A `[[shard]]` config entry omits the `shape` field entirely (malformed config) | Fail-loud: this BC's implementation MUST NOT default silently to either shape; the dispatch is treated as a config error (`HookResult::Error`), never a silent `Continue` that would leave an oversized artifact unguarded |
| EC-010 (fix-burst pass-3, F-P3-005) | A `"frontmatter-changelog-array"`-shaped config entry omits `low_water_mark` | Defaults to `floor(N/2)` (Postcondition 8's rotation-target-config bullet) — computed at config-load time, never a hardcoded fallback constant in `shard_manager.rs` |
| EC-011 (fix-burst pass-3, F-P3-005) | A `"frontmatter-changelog-array"`-shaped config entry declares `low_water_mark >= N` (including the degenerate `low_water_mark = N-1`) or a negative `low_water_mark` | Fail-loud: `HookResult::Error` — the config is treated as malformed and NEVER silently clamped or defaulted around (mirrors EC-009's posture for the `shape` field, extended to this field) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| **CORRECTED (fix-burst pass-2, F-P2-002).** `Write` to `decision-log.md`, current shard 40,000 bytes (irrelevant to the `Write` formula), `content` length 5,000 bytes, `shard_cap_bytes=49,152` | `projected_size = len(content) = 5,000 <= 49,152` → `Continue` — `current_shard_bytes` is NOT added for `Write` | happy-path |
| **CORRECTED (fix-burst pass-2, F-P2-002).** `Write` to `decision-log.md`, current shard 45,000 bytes (irrelevant), `content` length 50,000 bytes, `shard_cap_bytes=49,152` | `projected_size = len(content) = 50,000 > 49,152` → roll triggers (BC-1.18.006) | edge-case |
| **NEW regression vector (fix-burst pass-2, F-P2-002) — demonstrates the WITHDRAWN formula's over-trigger bug.** `Write` to `decision-log.md`, current shard 40,000 bytes, `content` length 40,000 bytes (a same-size full-file rewrite), `shard_cap_bytes=49,152` | CORRECTED: `projected_size = len(content) = 40,000 <= 49,152` → `Continue`. The WITHDRAWN formula would have wrongly computed `40,000 + 40,000 = 80,000 > 49,152` and over-triggered a roll on an ordinary same-size `Write` | edge-case |
| `Edit` to `decision-log.md`, current shard 45,000 bytes, `old_string`/`new_string` netting +5,000 bytes, `shard_cap_bytes=49,152` | `Edit`/`MultiEdit` formula UNCHANGED: `projected_size = current_shard_bytes + net_delta_bytes = 45,000 + 5,000 = 50,000 > 49,152` → roll triggers (BC-1.18.006) | edge-case |
| `Edit` to `some/unrelated/file.md` (no `[[shard]]` match) | `Continue`, zero `stat()` calls | happy-path |
| `Write` to `burst-log.md`, `cap_for(validate-burst-log)=40,000`, `cap_for(regression-gate)=49,152`, `cap_for(convergence-tracker)=52,000` | Effective `shard_cap_bytes = MIN(40000, 49152, 52000) = 40,000` (Cross-Validator Minimum Rule) | edge-case |
| `Write` to `decision-log.md` (not read by `validate-burst-log`), `cap_for(regression-gate)=49,152`, `cap_for(convergence-tracker)=52,000` | Effective `shard_cap_bytes = MIN(49152, 52000) = 49,152` — `validate-burst-log`'s cap is NOT applied | edge-case |
| `MultiEdit` on `lessons.md` with edits netting +2,000 / -500 / +100 bytes, current shard 48,000, `shard_cap_bytes=49,152` | `net_delta_bytes = 2000-500+100 = 1,600`; `projected_size = current_shard_bytes + net_delta_bytes = 49,600 > 49,152` → roll triggers (`Edit`/`MultiEdit` formula, unchanged) | edge-case |
| `projected_size` == `shard_cap_bytes` exactly (boundary) | `Continue` — inclusive boundary, no roll | error |
| `Edit` to `BC-INDEX.md`'s frontmatter, `shape="frontmatter-changelog-array"`, `N=50`, live `changelog:` at 50 items | `current_item_count + 1 = 51 > 50` → item-count trigger fires (Postcondition 8); BC-1.18.009's rotation-then-retry outcome applies | edge-case |
| `Edit` to `BC-INDEX.md`'s frontmatter, `shape="frontmatter-changelog-array"`, `N=50`, live `changelog:` at 10 items | `current_item_count + 1 = 11 <= 50` → `Continue`, no rotation | happy-path |
| **NEW (fix-burst pass-3, F-P3-005).** `shape="frontmatter-changelog-array"` config entry declares `N=50`, omits `low_water_mark` | `low_water_mark` defaults to `floor(50/2) = 25` at config-load time (EC-010) | edge-case |
| **NEW (fix-burst pass-3, F-P3-005).** `shape="frontmatter-changelog-array"` config entry declares `N=50`, `low_water_mark=50` (the degenerate `== N` case) or `low_water_mark=-1` (negative) | Fail-loud: `HookResult::Error` in both cases — both values violate `0 <= low_water_mark < N` and are NEVER silently clamped or defaulted around (EC-011). (`low_water_mark=49`, i.e. `N-1`, is a VALID boundary value — `49 < 50` satisfies the constraint — and does NOT fail-loud.) | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-117 | Unmatched-path zero-cost invariant (no `stat()` call issued when target path does not match any `[[shard]]` config entry); Cross-Validator Minimum Rule (effective cap for a multi-reader artifact equals the MIN of all applicable per-plugin caps); Byte-denomination invariant (no code path compares a non-byte-denominated quantity against `shard_cap_bytes`) | unit test — three facets: mock filesystem call counter; table-driven over the 4 mechanism-A artifacts × 3 Cohort B plugins; arbitrary payload/current-shard sizes compared against a byte-for-byte oracle |
| VP-116 | Boundary inclusivity — `projected_size == shard_cap_bytes` never triggers a roll; `projected_size == shard_cap_bytes + 1` always triggers a roll | kani-proof (exact-boundary + overflow/underflow-safety over symbolic inputs) |

**Fix-burst note (F-S2502-F2-003):** the two rows above that previously read "unit test" (for
VP-116) and "proptest" (for VP-117's byte-denomination row) were reconciled to the authoritative
`VP-INDEX.md` v3.02 catalog assignment — VP-116 = kani-proof, VP-117 = unit-test — in this
fix-burst. No property content changed, only the Proof Method column.

**Fix-burst note (fix-burst pass-3, F-P3-006):** VP-117's three previously-separate rows are
collapsed to ONE row listing its three facets (multi-facet convention — VP-117 is a single
allocated VP covering all three properties, not three separate VPs). No property content or
coverage change, only table presentation.

## Related BCs

- BC-1.18.006 — owns the observable roll/block outcome once this BC's mechanism-A trigger fires (depends on)
- BC-1.18.007 — retention/compaction policy for shards this BC's cap produces (composes with)
- BC-1.18.008 — one-time backfill-split applies this BC's SAME formula retroactively (depends on)
- BC-1.18.009 — owns the observable rotate/block-and-retry outcome once this BC's item-count trigger (Postcondition 8) fires (depended on by)
- BC-1.18.012 — the governed one-time B1 changelog backfill migration whose successful completion is what MAKES Postcondition 8's steady-state "bounded" characterization true (the cold-state characterization above holds only before this BC's migration runs) (depended on by)
- BC-7.08.001 — Cohort B fail-closed flip is gated on this BC's postconditions holding at F4-lock (depended on by)
- BC-1.18.001 — sibling Layer-1 fail-closed INDETERMINATE contract this BC's cap prevents from firing (related to)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` (new module) — cap-formula evaluation, `[[shard]]` config resolution, `stat()`-only size check, item-count trigger read
- `crates/factory-dispatcher/src/executor.rs` — PreToolUse dispatch path invocation point, before the registry-driven plugin loop
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `block_if_marker_check` native-check precedent this BC's placement pattern follows
- `plugins/vsdd-factory/hooks-registry.toml` — `[[shard]]` config table (or sibling `shard-config.toml`, TBD at F4), including the `shape` field this BC's Postcondition 8 dispatches on

## SDK Grounding Evidence

Literal stable-anchor greps substantiating this BC's external-artifact claims (POLICY 5;
`grep -n` and file:line citations are forbidden per TD-VSDD-091 — anchors below are grepped by
stable pattern only):

```
$ grep -oE "^pub enum HookResult" crates/hook-sdk/src/result.rs
pub enum HookResult
```

```
$ grep -oE "^\s*(Continue|Block \{[^}]*\}|Error \{[^}]*\})" crates/hook-sdk/src/result.rs | sed -E 's/^\s+//' | sort -u
Block { reason: String }
Continue
Error { message: String }
```

Confirms `HookResult`'s three-variant contract this BC's Postcondition 3 hands off to (roll
triggered → BC-1.18.006's `Block`; this BC itself never constructs a `HookResult` other than
`Continue` for the trigger-boundary role it owns).

```
$ grep -oE "^pub fn write_indeterminate_marker|^pub fn should_write_marker|^pub fn block_if_marker_check" crates/factory-dispatcher/src/indeterminate_marker.rs
pub fn block_if_marker_check
pub fn should_write_marker
pub fn write_indeterminate_marker
```

Confirms the native, non-WASM `block_if_marker_check` precedent this BC's Invariant 1 cites as the
architectural analogue for a fuel-budget-free dispatcher-native check.

```
$ grep -A6 'name = "validate-burst-log"' plugins/vsdd-factory/hooks-registry.toml | grep -E "^tool =|^priority ="
tool = "^(Edit|Write|MultiEdit)$"
priority = 152
tool = "^Bash$"
priority = 152
```

```
$ grep -A4 'name = "convergence-tracker"' plugins/vsdd-factory/hooks-registry.toml | grep -E "priority|async"
priority = 210
```

```
$ grep -A4 'name = "regression-gate"' plugins/vsdd-factory/hooks-registry.toml | grep "priority"
priority = 230
```

Confirms the three Cohort B validator identities and priorities the Cross-Validator Minimum Rule
(Postcondition 5) is keyed against.

```
$ grep -oE "^\*\*CAP-04[123] " .factory/specs/domain-spec/capabilities.md
**CAP-041 
**CAP-042 
**CAP-043 
```

Confirms CAP-041/CAP-042/CAP-043 all exist as registered capabilities, grounding this BC's
capability anchor (CAP-043) and its distinction from CAP-041/CAP-042 cited in the Description.

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- VP-116, VP-117 — allocated by formal-verifier (S-25.02 F2 verification-property extension burst; VP-INDEX v3.02). VP-116 (kani-proof; boundary inclusivity + cap-comparison arithmetic overflow-safety), VP-117 (unit-test; unmatched-path zero-cost bypass, Cross-Validator Minimum Rule, byte denomination). Cap-constant numeric bounds PROVISIONAL-until-F4 per ADR-051 §Decision 2. **Forward reference (fix-burst pass-3, F-P3-005, superseding the prior stale note):** formal-verifier authors a dedicated PC8 item-count-trigger VP — covering the shape-dispatch, the `current_item_count + 1 > N` trigger condition, and the `low_water_mark` rotation-target config (default/fail-loud validation, EC-010/EC-011) added in this fix-burst — in the following verification-property burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies CAP-043's cap-formula-and-trigger boundary: "a native... dispatcher function computes, for every registered sharded artifact, a `shard_cap_bytes` ceiling from four calibrated inputs... and performs a `stat()`-only byte-size check... before every `Edit`/`Write`/`MultiEdit`." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN — consistent with BC-1.18.004's own precedent for this class of dispatcher-mechanics contract) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` native PreToolUse check) |
| ADR | ADR-051 §Decision 1 (native gate placement + algorithm + v1.1 trigger-shape dispatch amendment), §Decision 2 (cap formula, calibration method, provisional constants); ADR-047 §Decision 8b (ratified future phase this BC elaborates); ADR-042 (measured fuel/byte linear model grounding `WORST_CASE_FUEL_PER_BYTE`'s provisional floor) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.3 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-3 finding F-P3-005 MEDIUM, ADR-051 v1.3 Decision 14): ADDED a `low_water_mark` rotation-target config field to Postcondition 8 (sibling to `N`, `"frontmatter-changelog-array"` shape only) — default `floor(N/2)` when omitted, fail-loud-validated `0 <= low_water_mark < N` (never silently clamped or defaulted around a violation); `N` remains the unchanged trigger threshold, this BC owns declaring/validating both `N` and `low_water_mark`, BC-1.18.009 owns what its rotation step does with `low_water_mark`. Extended Invariant 4 to cover the new field. Added EC-010 (omitted → default) and EC-011 (fail-loud on `>= N` or negative) plus two matching Canonical Test Vectors. Replaced the stale §VP Anchors note (which referenced an unauthored PC8 item-count VP) with a clean forward reference to formal-verifier's follow-on PC8 VP authorship. Collapsed the §Verification Properties table's three separate VP-117 rows into one multi-facet row (F-P3-006; no coverage change, presentation only). |
| 1.2 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-2 findings F-P2-002 HIGH + F-P2-007 MEDIUM, ADR-051 v1.2 Decisions 1/13): REWROTE Postcondition 3's `projected_size` formula from the WITHDRAWN uniform `current_shard_bytes + payload_bytes` (unsound for `Write` — double-counted a `Write`'s own already-complete content on top of the current shard's size) to the CORRECTED tool-discriminated formula: `Write` → `projected_size = len(content)` alone; `Edit`/`MultiEdit` → `projected_size = current_shard_bytes + net_delta_bytes` (unchanged — this leg was never wrong). Updated Canonical Test Vectors: corrected the two `Write` vectors, added a regression vector demonstrating the withdrawn formula's over-trigger bug on a same-size full-file `Write`, and added an explicit `Edit` vector to preserve coverage of the unchanged current+delta formula. Corrected Postcondition 8's read-cost claim into an explicit COLD-STATE (pre-BC-1.18.012 migration: ~1,997-item, non-N-relative-bounded, one-time read) vs. STEADY-STATE (post-migration: genuinely `<= N`-item-bounded) split — the prior text's unqualified "bounded" claim was true only in steady state. Added BC-1.18.012 to Related BCs (the new governed one-time B1 changelog backfill migration BC that makes the steady-state characterization true). |
| 1.1 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-1 findings F-S2502-F2-005 + F-S2502-F2-003 + F-S2502-F2-007, ADR-051 v1.1 Decision 1 amendment): NEW Postcondition 8 + Invariant 5 + EC-008/EC-009 adding the item-count-denominated trigger for the `"frontmatter-changelog-array"` artifact shape (mechanism B1, BC-INDEX's `changelog:` array) — this BC now owns BOTH trigger shapes the native gate dispatches on, distinguished by a `[[shard]]` config `shape` field; does not replace or weaken the existing byte-size postconditions. VP table reconciled to VP-INDEX v3.02 authoritative methods: VP-116 unit-test→kani-proof, VP-117 byte-denomination row proptest→unit-test (no property content change). Added `## SDK Grounding Evidence` section with literal stable-anchor grep output for `HookResult`, `block_if_marker_check`, Cohort B validator registry entries, and CAP-041/042/043 existence. |
| 1.0 | 2026-09-05 | product-owner | Initial creation. F2 spec-evolution burst, S-25.02 activation. Byte-size-denominated shard-cap formula, native `stat()`-only deterministic size-trigger, Cross-Validator Minimum Rule, provisional-constants table (explicitly marked PROVISIONAL-until-F4-harness-calibration per ADR-051 Decision 2). CAP-043 capability anchor. ADR-051 §D1/§D2 + ADR-047 §D8b + ADR-042 citations. |
