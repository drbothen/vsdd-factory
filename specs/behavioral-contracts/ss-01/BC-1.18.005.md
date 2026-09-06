---
document_type: behavioral-contract
level: L3
version: "1.0"
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
input-hash: "49c369a"
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
cannot exhaust a fuel budget because it has none (native code, not a WASM plugin).

## Preconditions

1. The dispatcher's PreToolUse handling path for `Edit`/`Write`/`MultiEdit` tool calls is reached
   (ADR-051 Decision 1 placement: before the registry-driven plugin loop).
2. A `[[shard]]` config table (`hooks-registry.toml` or a sibling `shard-config.toml`, TBD at F4)
   exists naming the target artifact's stem, its cap-formula inputs, and its current
   `shard_cap_bytes`.
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
   invocation at all (ADR-051 §Rationale — "Why native, not WASM").

3. **Projected size, not current size, is compared against the cap.** `projected_size =
   current_shard_bytes + payload_bytes`, where `payload_bytes` is read directly from the pending
   tool call's own parameters (`content` length for `Write`; the net length delta —
   `len(new_string) - len(old_string)` — for `Edit`/`MultiEdit`; for `MultiEdit`, the sum of each
   individual edit's net delta) — never a re-read of the file after a hypothetical apply. If
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
   is a byte count.

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

## Invariants

1. **The size check is native code, never a WASM plugin.** No `[[hooks]]` registry entry, no
   `.wasm` binary, and no `HOST_ABI_VERSION` bump implements this BC's check. It is invoked from
   inside `crates/factory-dispatcher/src/shard_manager.rs` (new module), called from the
   dispatcher's own PreToolUse handling path, architecturally analogous to the existing native
   `block_if_marker_check` precedent (`indeterminate_marker.rs`, consulted from `executor.rs`).

2. **The cap value is always expressed and compared in bytes.** No code path in this BC's
   implementation may substitute a line count, a character count under a non-byte encoding
   assumption, or any other non-byte unit for `shard_cap_bytes`, `current_shard_bytes`,
   `payload_bytes`, or `projected_size`.

3. **Unmatched paths never pay a `stat()` cost.** The config-match check (Postcondition 1) MUST
   occur before any filesystem `stat()` call, so that Layer 2 imposes zero measurable per-call
   overhead on the ~99% of `Edit`/`Write`/`MultiEdit` calls that do not target a sharded artifact.

4. **The formula's four inputs are configuration, not embedded constants.** `PRACTICAL_FUEL_CEILING`,
   `WORST_CASE_FUEL_PER_BYTE`, `MAX_SINGLE_RECORD_BYTES`, and `SAFETY_MARGIN` are read from the
   `[[shard]]` config (or the shard-index TOML per BC-1.18.006 Decision 4's schema), never
   hardcoded as Rust constants inside `shard_manager.rs` — this is what makes the F4 harness's
   recalibration a config change, not a code change.

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

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Write` to `decision-log.md`, current shard 40,000 bytes, payload 5,000 bytes, `shard_cap_bytes=49,152` | `projected_size=45,000 <= 49,152` → `Continue` | happy-path |
| `Write` to `decision-log.md`, current shard 45,000 bytes, payload 5,000 bytes, `shard_cap_bytes=49,152` | `projected_size=50,000 > 49,152` → roll triggers (BC-1.18.006) | edge-case |
| `Edit` to `some/unrelated/file.md` (no `[[shard]]` match) | `Continue`, zero `stat()` calls | happy-path |
| `Write` to `burst-log.md`, `cap_for(validate-burst-log)=40,000`, `cap_for(regression-gate)=49,152`, `cap_for(convergence-tracker)=52,000` | Effective `shard_cap_bytes = MIN(40000, 49152, 52000) = 40,000` (Cross-Validator Minimum Rule) | edge-case |
| `Write` to `decision-log.md` (not read by `validate-burst-log`), `cap_for(regression-gate)=49,152`, `cap_for(convergence-tracker)=52,000` | Effective `shard_cap_bytes = MIN(49152, 52000) = 49,152` — `validate-burst-log`'s cap is NOT applied | edge-case |
| `MultiEdit` on `lessons.md` with edits netting +2,000 / -500 / +100 bytes, current shard 48,000, `shard_cap_bytes=49,152` | `payload_bytes = 2000-500+100 = 1,600`; `projected_size=49,600 > 49,152` → roll triggers | edge-case |
| `projected_size` == `shard_cap_bytes` exactly (boundary) | `Continue` — inclusive boundary, no roll | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-117 | Unmatched-path zero-cost invariant — no `stat()` call issued when target path does not match any `[[shard]]` config entry | unit test (mock filesystem call counter) |
| VP-117 | Cross-Validator Minimum Rule — effective cap for a multi-reader artifact equals the MIN of all applicable per-plugin caps | unit test (table-driven over the 4 mechanism-A artifacts × 3 Cohort B plugins) |
| VP-117 | Byte-denomination invariant — no code path compares a non-byte-denominated quantity against `shard_cap_bytes` | proptest (arbitrary payload sizes, current-shard sizes; property: comparison result matches a byte-for-byte oracle) |
| VP-116 | Boundary inclusivity — `projected_size == shard_cap_bytes` never triggers a roll; `projected_size == shard_cap_bytes + 1` always triggers a roll | unit test (exact-boundary table) |

## Related BCs

- BC-1.18.006 — owns the observable roll/block outcome once this BC's trigger fires (depends on)
- BC-1.18.007 — retention/compaction policy for shards this BC's cap produces (composes with)
- BC-1.18.008 — one-time backfill-split applies this BC's SAME formula retroactively (depends on)
- BC-7.08.001 — Cohort B fail-closed flip is gated on this BC's postconditions holding at F4-lock (depended on by)
- BC-1.18.001 — sibling Layer-1 fail-closed INDETERMINATE contract this BC's cap prevents from firing (related to)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` (new module) — cap-formula evaluation, `[[shard]]` config resolution, `stat()`-only size check
- `crates/factory-dispatcher/src/executor.rs` — PreToolUse dispatch path invocation point, before the registry-driven plugin loop
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `block_if_marker_check` native-check precedent this BC's placement pattern follows
- `plugins/vsdd-factory/hooks-registry.toml` — `[[shard]]` config table (or sibling `shard-config.toml`, TBD at F4)

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- VP-116, VP-117 — allocated by formal-verifier (S-25.02 F2 verification-property extension burst; VP-INDEX v3.02). VP-116 (kani-proof; boundary inclusivity + cap-comparison arithmetic overflow-safety), VP-117 (unit-test; unmatched-path zero-cost bypass, Cross-Validator Minimum Rule, byte denomination). Cap-constant numeric bounds PROVISIONAL-until-F4 per ADR-051 §Decision 2.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies CAP-043's cap-formula-and-trigger boundary: "a native... dispatcher function computes, for every registered sharded artifact, a `shard_cap_bytes` ceiling from four calibrated inputs... and performs a `stat()`-only byte-size check... before every `Edit`/`Write`/`MultiEdit`." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN — consistent with BC-1.18.004's own precedent for this class of dispatcher-mechanics contract) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` native PreToolUse check) |
| ADR | ADR-051 §Decision 1 (native gate placement + algorithm), §Decision 2 (cap formula, calibration method, provisional constants); ADR-047 §Decision 8b (ratified future phase this BC elaborates); ADR-042 (measured fuel/byte linear model grounding `WORST_CASE_FUEL_PER_BYTE`'s provisional floor) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-09-05 | product-owner | Initial creation. F2 spec-evolution burst, S-25.02 activation. Byte-size-denominated shard-cap formula, native `stat()`-only deterministic size-trigger, Cross-Validator Minimum Rule, provisional-constants table (explicitly marked PROVISIONAL-until-F4-harness-calibration per ADR-051 Decision 2). CAP-043 capability anchor. ADR-051 §D1/§D2 + ADR-047 §D8b + ADR-042 citations. |
