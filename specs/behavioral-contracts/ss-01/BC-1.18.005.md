---
document_type: behavioral-contract
level: L3
version: "1.9"
status: draft
producer: product-owner
timestamp: 2026-09-06T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-051-layer-2-two-mechanism-size-triggered-shard-rotation-append-logs-and-bc-index-sharding.md
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/architecture/decisions/ADR-042-validate-cross-site-correspondence-fuel-budget-raise-and-loud-exhaustion-signaling.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f1-delta-analysis.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "af83d3c"
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

1. **Zero-cost bypass for unmatched paths — scoped to the TARGET ARTIFACT (scope clarified, S-25.02
   Phase F4 LOCAL adversary cluster-1 pass-3 finding F-C1-P3-003, OBSERVATION, product-owner
   adjudication, v1.9).** If the tool call's target path does not match any `[[shard]]` config
   entry, the native check returns `Continue` immediately, performing no `stat()` call against the
   TARGET ARTIFACT and no arithmetic. This preserves zero added latency for the ~99% of
   `Edit`/`Write`/`MultiEdit` calls Layer 2 does not touch (ADR-051 Decision 1 step 1). This
   guarantee is scoped to the target artifact's own file specifically; it does NOT forbid the
   necessary one-time applicability probe against the FIXED `[[shard]]` config file itself
   (`Path::exists()` against a fixed, small config path — e.g. `.factory/shard-config.toml`) that
   determines WHETHER this BC's gate is even a candidate for the current dispatch at all. That
   config-presence probe is O(1), fixed-size regardless of the target artifact, reads no target-
   artifact content, and — being native dispatcher code — is fuel-free; it is the minimal necessary
   applicability check, not a violation of the zero-cost bypass this postcondition protects
   (Postcondition 2's stat()-only-for-matched-paths and Invariant 3's identically-scoped guarantee
   apply this same target-artifact-specific reading).

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
   - **Missing-file / first-write case (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3
     finding F-C1-P3-001, MEDIUM, product-owner adjudication, v1.9) — GRACEFUL, mirroring EC-004's
     flat-shape precedent, NOT fail-loud.** When the `"frontmatter-changelog-array"`-shaped target
     artifact does not yet exist on disk — a legitimate first-ever `Write` that CREATES the
     artifact (e.g. the first-ever `Write` of a new sharded index file, with valid frontmatter
     present in the `content` payload) — `read_changelog_item_count` MUST map
     `io::ErrorKind::NotFound` to `Ok(0)`, treating the not-yet-existing artifact as holding 0
     existing changelog items, rather than propagating a bare `io::Error` that would surface as
     `HookResult::Error` and hard-block the create (exit 2). This is the SAME missing-file-is-zero
     posture EC-004 already establishes for the `"flat"` shape's `current_shard_bytes_flat` on the
     identical "artifact absent on disk" precondition — there is no principled reason for the two
     trigger shapes to diverge on that precondition, and per CLAUDE.md's production-grade default a
     hard block on a legitimate create is a user-facing wrong behavior, not an acceptable asymmetry.
     Combined with the trigger condition below, `current_item_count + 1 = 0 + 1 = 1 > N` is false
     for any config `N >= 1`, so the create always returns `Continue`. Any OTHER `io::Error` kind
     encountered reading an EXISTING file (a genuinely malformed/unreadable frontmatter fence, a
     permissions failure, etc.) is UNCHANGED and still propagates as `HookResult::Error`
     (fail-loud) — ONLY `NotFound` is special-cased. See EC-014.
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
     (`low_water_mark >= N` — the `== N` boundary included — or a negative value) is NEVER silently
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
   - **CORRECTED (fix-burst pass-4, F-P4-001, HIGH, ADR-051 v1.4 Decision 14 adjudication Option
     (b)) — a legal-but-poorly-amortizing `low_water_mark` value NEVER fail-louds; it loads
     normally and instead emits a non-fatal amortization advisory.** Any value satisfying
     `0 <= low_water_mark < N` — including every value in `(floor(N/2), N)` up to and including the
     boundary value `N-1` — is LEGAL and loads normally (`Continue`; config load NEVER returns
     `HookResult::Error` for such a value). `N-1` is not a special case: `N-1 < N` holds by
     construction for every `N >= 1`, so it satisfies the fail-loud constraint exactly as any other
     interior value does. To close the latent amortization-churn concern WITHOUT a false fail-loud
     claim, config load additionally emits a NEW, NON-FATAL, WARN-level diagnostic (via
     `tracing::warn!`, never `println!`, never `HookResult::Error`) whenever
     `low_water_mark > floor(N/2)` — i.e., whenever the configured value amortizes rotation worse
     than the recommended default. The advisory cites the configured `(N, low_water_mark)` pair and
     the resulting amortization factor `N - low_water_mark` (writes-per-rotation), compared against
     the default's `N - floor(N/2)` amortization, so an operator can see quantitatively how much
     worse their configured value performs. The advisory fires or does not fire; either way, config
     load ALWAYS succeeds for any value satisfying the numeric constraint — this is the load-time
     equivalent of a lint warning, not a second validation gate.

9. **NEW (S-25.02 Phase F4 LOCAL adversary cluster-1 finding, PC4 load-enforcement-intent
   question, product-owner adjudication) — load-time fail-loud enforcement of the cap-vs-formula
   inequality.** `ShardRegistry::load()` MUST, for EVERY `[[shard]]` config entry — regardless of
   `shape` (Postcondition 8 already establishes that `shard_cap_bytes` bounds an artifact's TOTAL
   byte footprint as a whole-artifact concern even for the `"frontmatter-changelog-array"` shape,
   so this check is NOT `"flat"`-shape-only) — independently recompute
   `compute_shard_cap_bytes(entry.cap_formula_inputs())` from that SAME entry's own four parsed
   formula inputs (`practical_fuel_ceiling`, `worst_case_fuel_per_byte`, `max_single_record_bytes`,
   `safety_margin` — Precondition 2, Postcondition 4/6) and compare it against that entry's own
   declared `shard_cap_bytes`. If `shard_cap_bytes > compute_shard_cap_bytes(entry.cap_formula_inputs())`
   — the declared cap exceeds what the entry's OWN stated formula inputs justify — `load()` MUST
   return `HookResult::Error`; it MUST NEVER silently accept the oversized value and MUST NEVER
   silently clamp `shard_cap_bytes` down to the computed ceiling. This mirrors the fail-loud-at-
   load-time posture EC-009 (`shape` omission) and EC-011 (`low_water_mark` out of range) already
   establish for this SAME config surface, extended to the cap-vs-formula relationship. **Adjudication
   rationale (Reading (B) adopted over Reading (A)):** without this check, a `[[shard]]` entry whose
   declared `shard_cap_bytes` is set far above its own declared formula ceiling loads silently, and
   the live per-write gate (Postcondition 3) then honors the too-large cap — reintroducing the exact
   fuel-exhaustion failure mode BC-1.18.001's sibling Layer-1 fail-closed INDETERMINATE contract
   exists to prevent (see Related BCs). The four formula inputs are therefore load-bearing
   configuration at load time, not merely documentary/harness-facing metadata carried for the F4
   calibration harness's offline use — Postcondition 6's "the formula shape is locked now, the
   numbers are not" already states that the INEQUALITY itself (not just its eventual F4-locked
   numeric inputs) is a live constraint of the system today, under the current provisional
   constants. This is a load-time misconfiguration guard: it protects against a human/config-authoring
   error in a `[[shard]]` entry's own internally-inconsistent inputs; it does NOT require or perform
   any re-derivation of the F4 harness's calibration itself (Postcondition 6/7 continue to own
   HOW the constants are measured; this postcondition only owns verifying INTERNAL CONSISTENCY
   between a given entry's own declared `shard_cap_bytes` and its own declared four inputs).

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

3. **Unmatched paths never pay a `stat()` cost against the TARGET ARTIFACT (scope clarified,
   F-C1-P3-003, v1.9).** The config-match check (Postcondition 1) MUST occur before any filesystem
   `stat()` call against the target artifact's own file, so that Layer 2 imposes zero measurable
   per-call overhead, on the target artifact, for the ~99% of `Edit`/`Write`/`MultiEdit` calls that
   do not target a sharded artifact. This invariant is scoped to the target artifact specifically;
   it does NOT forbid the one-time, fixed-size, fuel-free `[[shard]]` config-presence probe
   (`Path::exists()` against the fixed config path) that gates whether this BC's check runs at all
   — that probe is the necessary applicability check this BC's own gate placement requires, is O(1)
   and fixed-size regardless of the target artifact, and is not a per-target-path filesystem read of
   the artifact itself.

4. **The formula's four inputs are configuration, not embedded constants.** `PRACTICAL_FUEL_CEILING`,
   `WORST_CASE_FUEL_PER_BYTE`, `MAX_SINGLE_RECORD_BYTES`, and `SAFETY_MARGIN` are read from the
   `[[shard]]` config (or the shard-index TOML per BC-1.18.006 Decision 4's schema), never
   hardcoded as Rust constants inside `shard_manager.rs` — this is what makes the F4 harness's
   recalibration a config change, not a code change. **Extended (fix-burst pass-3, F-P3-005):** `N`
   and `low_water_mark` (Postcondition 8's item-count-shape config pair) are likewise always
   configuration, never embedded constants or a derived-and-hardcoded value. **Timing clarified
   (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-2 finding F-C1-P2-003, product-owner
   adjudication, v1.8):** `low_water_mark`'s default (`floor(N/2)`) MUST always be DERIVED FROM the
   entry's own config `N` — it MUST NEVER be a hardcoded fallback constant baked into
   `shard_manager.rs`. The derivation's TIMING is not itself load-bearing: an implementation MAY
   materialize the default eagerly while parsing the `[[shard]]` config, OR resolve it lazily at
   first use/consumption (e.g. a pure `resolved_low_water_mark(n, low_water_mark)` function called
   at rotation-target resolution, per BC-1.18.009) — either shape satisfies this invariant, because
   both derive the value from config `N` at the moment it is needed and neither ever substitutes a
   constant. What this invariant forbids is a *value*, not a *when*: e.g. a Rust `const
   DEFAULT_LOW_WATER_MARK_FALLBACK: u64 = 25` (or any other numeral not derived from that entry's
   own `N`) would violate this invariant regardless of when it were read.

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
| EC-010 (fix-burst pass-3, F-P3-005; timing clarified S-25.02 Phase F4 LOCAL adversary cluster-1 pass-2, F-C1-P2-003, v1.8) | A `"frontmatter-changelog-array"`-shaped config entry omits `low_water_mark` | Defaults to `floor(N/2)`, DERIVED FROM the entry's own config `N` (Postcondition 8's rotation-target-config bullet) — NEVER a hardcoded fallback constant in `shard_manager.rs`. Derivation MAY be materialized eagerly at config-load time OR deferred to first-use/resolution (e.g. `resolved_low_water_mark`) — timing is an implementation choice, not a contract obligation; the obligation is that the value is always computed from `N`, never a baked-in numeral |
| EC-011 (fix-burst pass-3, F-P3-005; scope CORRECTED fix-burst pass-4, F-P4-001, HIGH) | A `"frontmatter-changelog-array"`-shaped config entry declares `low_water_mark >= N` (the `== N` boundary included) or a negative `low_water_mark` | Fail-loud: `HookResult::Error` — the config is treated as malformed and NEVER silently clamped or defaulted around (mirrors EC-009's posture for the `shape` field, extended to this field). Scope is EXACTLY `>= N` or negative — `low_water_mark = N-1` is NOT in this scope; see EC-012 |
| EC-012 (fix-burst pass-4, F-P4-001, HIGH, ADR-051 v1.4 Decision 14 adjudication Option (b)) | A `"frontmatter-changelog-array"`-shaped config entry declares a legal-but-poorly-amortizing `low_water_mark` in `(floor(N/2), N)`, up to and including the boundary value `N-1` | Loads normally — `Continue`, config load NEVER returns `HookResult::Error` (the value satisfies `0 <= low_water_mark < N`) — but config-load emits a non-fatal `tracing::warn!` amortization advisory citing the configured `(N, low_water_mark)` pair and the resulting amortization factor `N - low_water_mark`, compared against the default's `N - floor(N/2)` amortization |
| EC-013 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 finding, product-owner adjudication) | A `[[shard]]` config entry declares `shard_cap_bytes` GREATER than `compute_shard_cap_bytes(entry.cap_formula_inputs())` — i.e., the declared cap exceeds its own formula-derived ceiling (applies to ANY `shape`, not `"flat"`-only) | Fail-loud: `ShardRegistry::load()` returns `HookResult::Error` — the misconfigured entry is REJECTED; NEVER silently accepted, NEVER silently clamped down to the computed ceiling (Postcondition 9) |
| EC-014 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3 finding F-C1-P3-001, MEDIUM, product-owner adjudication) | A `"frontmatter-changelog-array"`-shaped target artifact does not yet exist on disk (a legitimate first-ever `Write` CREATES it) | `read_changelog_item_count` maps `io::ErrorKind::NotFound` to `Ok(0)` — treated as 0 existing items, mirroring EC-004's flat-shape missing-file precedent; the item-count trigger (`current_item_count + 1 > N`) evaluates `0 + 1 > N`, which is `false` for any `N >= 1`, so `Continue` — the legitimate create is NEVER hard-blocked as `HookResult::Error`. Any OTHER `io::Error` kind reading an EXISTING file (malformed frontmatter fence, permissions failure, etc.) is UNCHANGED and still propagates as `HookResult::Error` — only `NotFound` is special-cased |

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
| **NEW (fix-burst pass-4, F-P4-001, ADR-051 v1.4 Decision 14 adjudication Option (b)).** `shape="frontmatter-changelog-array"` config entry declares `N=50`, `low_water_mark=49` (i.e. `N-1`) | Loads normally — `Continue`, no `HookResult::Error` (EC-012; `49 < 50` satisfies `0 <= low_water_mark < N`) — config-load emits a non-fatal `tracing::warn!` amortization advisory: amortization factor `N - low_water_mark = 50 - 49 = 1` (rotation fires roughly every 1 write), versus the recommended default `low_water_mark=25`'s amortization factor `50 - 25 = 25` | edge-case |
| **NEW (S-25.02 Phase F4 LOCAL adversary cluster-1 finding, product-owner adjudication).** `[[shard]]` entry declares `practical_fuel_ceiling=8,000,000`, `worst_case_fuel_per_byte=106.36`, `max_single_record_bytes=16,384`, `safety_margin=8,192` (`compute_shard_cap_bytes(inputs) = 49,152`), and declares `shard_cap_bytes=100,000` (far above its own formula ceiling) | `ShardRegistry::load()` returns `HookResult::Error` (Postcondition 9 / EC-013) — the misconfigured entry is REJECTED; the too-large cap is NEVER loaded and NEVER honored by the live per-write gate | error |
| **NEW (S-25.02 Phase F4 LOCAL adversary cluster-1 finding, product-owner adjudication).** `[[shard]]` entry declares the SAME four inputs as above (`compute_shard_cap_bytes(inputs) = 49,152`) and `shard_cap_bytes=49,152` (exactly equal to its own formula ceiling) | `ShardRegistry::load()` succeeds (`Ok`) — Postcondition 4's `<=` comparison is inclusive of the boundary (Postcondition 9), mirroring EC-002's inclusive-boundary precedent for the per-write trigger | happy-path |
| **NEW (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3 finding F-C1-P3-001, product-owner adjudication).** `Write` to a NOT-YET-EXISTING `"frontmatter-changelog-array"`-shaped target artifact (e.g. the first-ever `Write` of a new sharded index file), `shape="frontmatter-changelog-array"`, `N=50`, target file absent on disk, valid frontmatter with a `changelog:` array present in the `content` payload | `read_changelog_item_count` maps `io::ErrorKind::NotFound` to `Ok(0)`; `current_item_count + 1 = 0 + 1 = 1 <= 50` → `Continue` — the create is NEVER hard-blocked as `HookResult::Error` (EC-014; mirrors EC-004's flat-shape missing-file precedent) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-117 | Unmatched-path zero-cost invariant (no `stat()` call issued when target path does not match any `[[shard]]` config entry); Cross-Validator Minimum Rule (effective cap for a multi-reader artifact equals the MIN of all applicable per-plugin caps); Byte-denomination invariant (no code path compares a non-byte-denominated quantity against `shard_cap_bytes`) | unit test — three facets: mock filesystem call counter; table-driven over the 4 mechanism-A artifacts × 3 Cohort B plugins; arbitrary payload/current-shard sizes compared against a byte-for-byte oracle |
| VP-116 | Boundary inclusivity — `projected_size == shard_cap_bytes` never triggers a roll; `projected_size == shard_cap_bytes + 1` always triggers a roll | kani-proof (exact-boundary + overflow/underflow-safety over symbolic inputs) |
| VP-140 | Postcondition 8 item-count trigger boundary AND `low_water_mark` rotation-target config validation — shape-dispatch read-once; item-count off-by-one (`current_item_count + 1 > N`: `N-1` → Continue, exactly `N`/`N+1` → fire, EC-008); `low_water_mark` default `floor(N/2)` when omitted (EC-010); fail-loud `HookResult::Error` on `low_water_mark >= N` (incl. degenerate `== N`) or negative, `N-1` valid (EC-011); **amortization-advisory biconditional (added VP-140 v1.1, fix-burst pass-4, F-P4-001, ADR-051 v1.4 Decision 14 Option (b)):** a `tracing::warn!` non-fatal amortization advisory fires IFF `low_water_mark > floor(N/2)`, while config-load ALWAYS succeeds (never `HookResult::Error`) for any `0 <= low_water_mark < N`, including `N-1` (EC-012) | unit test — five facets: mock-config shape-dispatch + content-read counter; `{N-1, N, N+1}` trigger-boundary table; even/odd-`N` default table; `{N, N-1, 0, -1}` config-validation table; `low_water_mark ∈ {floor(N/2), floor(N/2)+1, N-1}` amortization-advisory table asserting the advisory fires iff `low_water_mark > floor(N/2)` and config-load always succeeds |

**Fix-burst note (F-S2502-F2-003):** the two rows above that previously read "unit test" (for
VP-116) and "proptest" (for VP-117's byte-denomination row) were reconciled to the authoritative
`VP-INDEX.md` v3.02 catalog assignment — VP-116 = kani-proof, VP-117 = unit-test — in this
fix-burst. No property content changed, only the Proof Method column.

**Fix-burst note (fix-burst pass-3, F-P3-006):** VP-117's three previously-separate rows are
collapsed to ONE row listing its three facets (multi-facet convention — VP-117 is a single
allocated VP covering all three properties, not three separate VPs). No property content or
coverage change, only table presentation.

**Adjudication note (S-25.02 Phase F4 LOCAL adversary cluster-1, product-owner, v1.7):** new
Postcondition 9 / EC-013 (load-time fail-loud enforcement of `shard_cap_bytes <=
compute_shard_cap_bytes(inputs)`) is a NEW verification obligation not yet covered by VP-116,
VP-117, or VP-140. Per this BC's own precedent for Postcondition 8 (added at v1.1 without a VP,
with VP-140 allocated by formal-verifier in a later burst — see the §VP Anchors note below),
product-owner does NOT allocate a VP number here. **Routed to architect/formal-verifier:** allocate
a new VP (or extend VP-117's multi-facet unit-test row) covering the Postcondition 9 / EC-013
config-validation property — `compute_shard_cap_bytes(inputs) >= shard_cap_bytes` boundary-inclusive
check at `ShardRegistry::load()` time — and propagate to VP-INDEX.md, verification-architecture.md,
and verification-coverage-matrix.md per `vp_index_is_vp_catalog_source_of_truth` (POLICY 9).

**Adjudication note (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3, product-owner, v1.9):** new
EC-014 (item-count shape's `NotFound`-is-zero missing-file graceful-degradation behavior, mirroring
EC-004's flat-shape precedent) is a NEW verification obligation not yet covered by VP-140's existing
five facets (shape-dispatch, item-count off-by-one, `low_water_mark` default, fail-loud validation,
amortization advisory). Per this BC's own Postcondition-9/EC-013 precedent immediately above (where
product-owner did not invent a new VP number for a load-time-validation extension), product-owner
does NOT allocate a VP number or facet here either. **Routed to architect/formal-verifier:** extend
VP-140 with a sixth facet (or allocate a new VP) covering `read_changelog_item_count`'s
`NotFound -> Ok(0)` mapping and the resulting `0 + 1 <= N` non-trigger for a not-yet-existing target
artifact, and propagate to VP-INDEX.md, verification-architecture.md, and
verification-coverage-matrix.md per `vp_index_is_vp_catalog_source_of_truth` (POLICY 9).

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

- VP-116, VP-117, VP-140 — allocated by formal-verifier (VP-116/VP-117: S-25.02 F2 verification-property extension burst, VP-INDEX v3.02; VP-140: S-25.02 F2 verification-property fix-burst PASS-3, VP-INDEX v3.05, F-P3-004). VP-116 (kani-proof; boundary inclusivity + cap-comparison arithmetic overflow-safety, `"flat"` byte-size shape), VP-117 (unit-test; unmatched-path zero-cost bypass, Cross-Validator Minimum Rule, byte denomination, `"flat"` byte-size shape). Cap-constant numeric bounds PROVISIONAL-until-F4 per ADR-051 §Decision 2. **VP-140 (unit-test; the dedicated Postcondition 8 item-count-trigger VP the prior forward reference called for, F-P3-004):** covers the shape-dispatch (read-once, no content-sniff), the `current_item_count + 1 > N` item-count trigger off-by-one boundary (`N-1` → Continue, exactly `N`/`N+1` → fire, EC-008), and the `low_water_mark` rotation-target config validation added in the pass-3 BC fix-burst — default `floor(N/2)` when omitted (EC-010), fail-loud `HookResult::Error` on `low_water_mark >= N` or negative with `N-1` valid (EC-011). **Also covers the EC-012 amortization-advisory facet added at VP-140 v1.1 (pass-4, F-P4-001, ADR-051 v1.4 Decision 14 Option (b)):** a non-fatal `tracing::warn!` advisory fires IFF `low_water_mark > floor(N/2)`, while config-load ALWAYS succeeds (never `HookResult::Error`) for any `0 <= low_water_mark < N`, including `N-1` (EC-012). VP-140 is the item-count-shape analogue of VP-116's role for the byte boundary; it carries NO F4-provisional numeric dependency (the item-count trigger is deliberately not byte-denominated).

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
| 1.9 | 2026-09-06 | product-owner | Spec/implementation asymmetry + wording-precision adjudication (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3 findings F-C1-P3-001 MEDIUM + F-C1-P3-003 OBSERVATION). **F-C1-P3-001 (substantive, GRACEFUL ruling):** the item-count shape's `read_changelog_item_count` did NOT special-case `io::ErrorKind::NotFound` the way the flat shape's `current_shard_bytes_flat` already does per EC-004 — a bare `read_to_string(target_path)?` propagated `NotFound` as an `io::Error` -> `HookResult::Error`, hard-blocking a legitimate FIRST-EVER `Write` that CREATES a `"frontmatter-changelog-array"` artifact (e.g. the first-ever `Write` of a new sharded index file). ADJUDICATED GRACEFUL over FAIL-LOUD: this is the identical "artifact absent on disk" precondition EC-004 already treats as size-0, not an error, for the sibling flat shape; there is no principled reason for the two trigger shapes to diverge on that precondition, and per CLAUDE.md's production-grade default a hard block on a legitimate create is a user-facing wrong behavior, not an acceptable, unexamined asymmetry. Postcondition 8's read-cost bullet was SILENT on the missing-file case (it characterized only cold/steady state of an EXISTING file) — this was the gap. ADDED a new missing-file/first-write sub-bullet to Postcondition 8 specifying `read_changelog_item_count` MUST map `NotFound -> Ok(0)` (any OTHER `io::Error` kind on an existing file is UNCHANGED, still fail-loud). ADDED EC-014 (missing target artifact -> `Ok(0)` -> `Continue` for any `N >= 1`, mirroring EC-004) and a matching Canonical Test Vector. Added an adjudication note routing VP-140 sixth-facet extension (or new VP) to architect/formal-verifier, per this BC's own Postcondition-9/EC-013 precedent for not self-allocating a VP number. **F-C1-P3-003 (cheap permanent close, OBSERVATION):** Postcondition 1 / Invariant 3 said the applicability check performs "no `stat()` call" / "zero measurable per-call overhead" for unmatched paths — but the implementation's applicability check does a small `Path::exists()` probe on a FIXED, small `[[shard]]` config file (not the target artifact) to decide whether the gate is even a candidate for the dispatch. REWORDED Postcondition 1 and Invariant 3 (minimally, additively) to scope the "no stat() / zero-cost" guarantee to the TARGET ARTIFACT specifically, explicitly exempting the one-time, O(1), fixed-size, fuel-free config-presence probe as the necessary applicability check — the load-bearing guarantee (no target-artifact stat, no fuel-budgeted content read on unmatched paths) is unchanged; only the literal wording's scope is clarified, preventing a future adversary pass from re-flagging the same literal-wording tension. No change to Postconditions 2-9, other Invariants, or any pre-existing Edge Case/Canonical Test Vector/VP — additive/clarifying only. |
| 1.8 | 2026-09-06 | product-owner | Spec/implementation wording-tension adjudication (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-2 finding F-C1-P2-003, LOW). Invariant 4 and EC-010 said `low_water_mark`'s `floor(N/2)` default is "computed at config-load time when the field is omitted" — but the sanctioned cluster-1 implementation (`crates/factory-dispatcher/src/shard_manager.rs`) leaves `low_water_mark = None` after `ShardRegistry::load()` when the field is omitted (`load()`'s own EC-010 branch only runs `validate_low_water_mark` when an EXPLICIT value is present) and derives `floor(N/2)` LAZILY in the pure function `resolved_low_water_mark(n, low_water_mark)`, called by the caller at first use/consumption (BC-1.18.009's rotation-target resolution). ADJUDICATED: the BINDING invariant this BC actually needs — grounded in ADR-051 §Decision 14's own framing ("This is a config DEFAULT, not a hardcoded Rust constant inside `shard_manager.rs` — consistent with BC-1.18.005 Invariant 4's 'formula inputs are configuration, not embedded constants' discipline") — is that the default is ALWAYS DERIVED FROM config `N`, NEVER a hardcoded fallback constant; ADR-051 never states derivation TIMING (load-time materialization vs. lazy first-use resolution) as a load-bearing constraint, and the cluster-1 lazy design has zero behavioral consequence in cluster 1 (nothing consumes `low_water_mark` until BC-1.18.009/cluster 4) and is a legitimate, pure, engineering choice (single source of truth is `N`; no derived-value staleness risk since config loads once per process; avoids mutating parsed config state). Per CLAUDE.md's production-grade default (over-specifying an implementation-timing detail that carries no safety value, then treating the resulting non-conformance as a defect, is itself the anti-pattern — the mechanical question "is timing load-bearing" is answerable now from ADR-051, not a TODO), REWORDED Invariant 4's `low_water_mark` clause and EC-010 to state the constraint as "derived from config `N`, never a hardcoded fallback constant; derivation MAY be materialized at config-load time OR deferred to first-use/resolution — timing is an implementation choice, not a contract obligation." No change to the numeric default (`floor(N/2)`), to the fail-loud validation constraint (`0 <= low_water_mark < N`, still validated at config-load time when an explicit value is declared — EC-011/EC-012 unaffected), to any Postcondition, or to any other Edge Case/Canonical Test Vector/VP. The sanctioned implementation was already conformant under the corrected wording; no code or test change is required. |
| 1.7 | 2026-09-06 | product-owner | BC-semantic intent adjudication (S-25.02 Phase F4 LOCAL adversary cluster-1 finding — PC4 load-enforcement-intent question). The cluster-1 implementation (`crates/factory-dispatcher/src/shard_manager.rs`) parses the four cap-formula inputs into `ShardEntry` and computes `compute_shard_cap_bytes()`, but `ShardRegistry::load()` never compared a declared `shard_cap_bytes` against its own entry's formula-derived ceiling — a misconfigured `[[shard]]` entry declaring `shard_cap_bytes` far ABOVE its own formula ceiling would load silently and the live per-write gate would honor the too-large cap, reintroducing the exact fuel-exhaustion failure this BC exists to prevent. ADJUDICATED Reading (B) RUNTIME-ENFORCED over Reading (A) HARNESS-CONSUMED-ONLY: the four inputs are parsed into the SAME `ShardEntry` the live per-write gate consumes (not a separate harness-only config surface), Postcondition 6 already states "the formula shape is locked now" (a live constraint, not merely a future-harness reference), and this BC's own Related BCs section identifies BC-1.18.005 as the mechanism that prevents BC-1.18.001's sibling Layer-1 fail-closed INDETERMINATE contract from firing — an unenforced cap-vs-formula inequality would defeat that stated purpose. Per CLAUDE.md's production-grade default (Rule 1 — no MVP-driven deferrals; a gate that lets an over-cap misconfiguration through silently is a defer-pattern smell), ADDED new Postcondition 9 (load-time fail-loud enforcement of `shard_cap_bytes <= compute_shard_cap_bytes(entry.cap_formula_inputs())` at `ShardRegistry::load()` time, for every `[[shard]]` entry regardless of `shape`, mirroring EC-009/EC-011's established fail-loud-at-load-time posture for this same config surface). ADDED EC-013 (cap exceeds formula ceiling → `HookResult::Error`). ADDED two Canonical Test Vectors (error case: `shard_cap_bytes=100,000` against a `49,152`-byte formula ceiling → `HookResult::Error`; boundary happy-path: `shard_cap_bytes` exactly equal to the formula ceiling → `Ok`, inclusive boundary mirroring EC-002). ADDED an adjudication note under §Verification Properties routing new-VP allocation for Postcondition 9/EC-013 to architect/formal-verifier (no VP number invented by product-owner, per this BC's own Postcondition-8-then-VP-140 precedent). No change to Postconditions 1-8, Invariants, or existing Edge Cases/Canonical Test Vectors — additive only. |
| 1.6 | 2026-09-06 | product-owner | Surgical parity tidy-up (adversary pass-11 LOW observation — §VP Anchors/§Verification Properties/VP-INDEX sibling parity): the §VP Anchors VP-140 bullet enumerated VP-140's facets (shape-dispatch, item-count off-by-one/EC-008, `low_water_mark` default/EC-010, fail-loud/EC-011) but OMITTED the EC-012 amortization-advisory facet that the v1.5 fix-burst already reconciled into the sibling §Verification Properties table row and that VP-INDEX's own VP-140 entry already carried — a §VP Anchors-only lag (the pass-7 fix swept the table row but not this bullet). APPENDED the EC-012 amortization-advisory facet (`tracing::warn!` fires IFF `low_water_mark > floor(N/2)`; config-load always succeeds for any `0 <= low_water_mark < N` incl. `N-1`) to the §VP Anchors VP-140 bullet, bringing it to parity with the table row and VP-INDEX at five facets. No postcondition, invariant, or contract-behavior change; reference-completeness only. |
| 1.5 | 2026-09-06 | product-owner | Surgical residual-sweep fix-burst (adversary pass-7 observation — VP-140 row completeness): the §Verification Properties VP-140 row description listed the shape-dispatch, item-count off-by-one (EC-008), and `low_water_mark` default/fail-loud (EC-010/EC-011) facets but OMITTED the EC-012 amortization-advisory facet VP-140 gained at v1.1 (pass-4, F-P4-001, ADR-051 v1.4 Decision 14 Option (b)) — a row-description completeness lag only; the VP↔BC linkage itself was already correct (VP-INDEX.md's own VP-140 row already carries all five facets). APPENDED the amortization-advisory biconditional facet (`tracing::warn!` fires IFF `low_water_mark > floor(N/2)`; config-load always succeeds for any `0 <= low_water_mark < N` incl. `N-1`, EC-012) to the VP-140 row's Property column and Proof Method column (four facets → five facets, matching VP-140 v1.1). No postcondition, invariant, or contract-behavior change; row-description reconciliation only. |
| 1.4 | 2026-09-06 | product-owner | Fix-burst amendment (adversary pass-4 finding F-P4-001 HIGH, ADR-051 v1.4 Decision 14 ADJUDICATION Option (b), per `.factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md` §4d): resolved the live, mutually-unsatisfiable contradiction between Postcondition 8/EC-011 (which folded `low_water_mark = N-1` into the fail-loud `>= N` bucket) and this BC's own Canonical Test Vectors table + VP-140 + VP-125 (which already stated `N-1` is a VALID, non-fail-loud boundary value). Postcondition 8's rotation-target-config bullet: STRUCK the parenthetical `"(including the degenerate N-1)"` from the fail-loud sentence — the fail-loud scope is now stated as EXACTLY `low_water_mark >= N` (the `== N` boundary included) or negative. ADDED a new sentence/bullet: any legal value in `(floor(N/2), N)`, up to and including `N-1`, loads normally (`Continue`, NEVER `HookResult::Error`) but config-load emits a NEW non-fatal `tracing::warn!` amortization advisory citing `(N, low_water_mark)` and the amortization factor `N - low_water_mark`, compared against the default's `N - floor(N/2)`. REWROTE EC-011: struck `"(including the degenerate low_water_mark = N-1)"` from its description (scope narrows to exactly `>= N`/negative; `HookResult::Error` outcome unchanged for that narrowed scope) and added a cross-reference to EC-012. ADDED NEW EC-012 (legal-but-poorly-amortizing `low_water_mark` in `(floor(N/2), N)` incl. `N-1` → loads normally + `tracing::warn!` amortization advisory, never `HookResult::Error`). Canonical Test Vectors: the existing `N=50`/`low_water_mark=49` parenthetical was already correct (no change); ADDED one new row for EC-012 (`N=50`, `low_water_mark=49` → loads, advisory fires with amortization factor 1). The numeric constraint `0 <= low_water_mark < N` itself is UNCHANGED — it was never wrong; only the erroneous "fail-loud on `N-1`" prose is withdrawn. No other BC-1.18.005 wording (Postconditions 1-7, other Edge Cases, VP-140/VP-125 rows, which were already correct) required amendment for this finding. |
| 1.3 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-3 finding F-P3-005 MEDIUM, ADR-051 v1.3 Decision 14): ADDED a `low_water_mark` rotation-target config field to Postcondition 8 (sibling to `N`, `"frontmatter-changelog-array"` shape only) — default `floor(N/2)` when omitted, fail-loud-validated `0 <= low_water_mark < N` (never silently clamped or defaulted around a violation); `N` remains the unchanged trigger threshold, this BC owns declaring/validating both `N` and `low_water_mark`, BC-1.18.009 owns what its rotation step does with `low_water_mark`. Extended Invariant 4 to cover the new field. Added EC-010 (omitted → default) and EC-011 (fail-loud on `>= N` or negative) plus two matching Canonical Test Vectors. Replaced the stale §VP Anchors note (which referenced an unauthored PC8 item-count VP) with a clean forward reference to formal-verifier's follow-on PC8 VP authorship. Collapsed the §Verification Properties table's three separate VP-117 rows into one multi-facet row (F-P3-006; no coverage change, presentation only). |
| 1.2 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-2 findings F-P2-002 HIGH + F-P2-007 MEDIUM, ADR-051 v1.2 Decisions 1/13): REWROTE Postcondition 3's `projected_size` formula from the WITHDRAWN uniform `current_shard_bytes + payload_bytes` (unsound for `Write` — double-counted a `Write`'s own already-complete content on top of the current shard's size) to the CORRECTED tool-discriminated formula: `Write` → `projected_size = len(content)` alone; `Edit`/`MultiEdit` → `projected_size = current_shard_bytes + net_delta_bytes` (unchanged — this leg was never wrong). Updated Canonical Test Vectors: corrected the two `Write` vectors, added a regression vector demonstrating the withdrawn formula's over-trigger bug on a same-size full-file `Write`, and added an explicit `Edit` vector to preserve coverage of the unchanged current+delta formula. Corrected Postcondition 8's read-cost claim into an explicit COLD-STATE (pre-BC-1.18.012 migration: ~1,997-item, non-N-relative-bounded, one-time read) vs. STEADY-STATE (post-migration: genuinely `<= N`-item-bounded) split — the prior text's unqualified "bounded" claim was true only in steady state. Added BC-1.18.012 to Related BCs (the new governed one-time B1 changelog backfill migration BC that makes the steady-state characterization true). |
| 1.1 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-1 findings F-S2502-F2-005 + F-S2502-F2-003 + F-S2502-F2-007, ADR-051 v1.1 Decision 1 amendment): NEW Postcondition 8 + Invariant 5 + EC-008/EC-009 adding the item-count-denominated trigger for the `"frontmatter-changelog-array"` artifact shape (mechanism B1, BC-INDEX's `changelog:` array) — this BC now owns BOTH trigger shapes the native gate dispatches on, distinguished by a `[[shard]]` config `shape` field; does not replace or weaken the existing byte-size postconditions. VP table reconciled to VP-INDEX v3.02 authoritative methods: VP-116 unit-test→kani-proof, VP-117 byte-denomination row proptest→unit-test (no property content change). Added `## SDK Grounding Evidence` section with literal stable-anchor grep output for `HookResult`, `block_if_marker_check`, Cohort B validator registry entries, and CAP-041/042/043 existence. |
| 1.0 | 2026-09-05 | product-owner | Initial creation. F2 spec-evolution burst, S-25.02 activation. Byte-size-denominated shard-cap formula, native `stat()`-only deterministic size-trigger, Cross-Validator Minimum Rule, provisional-constants table (explicitly marked PROVISIONAL-until-F4-harness-calibration per ADR-051 Decision 2). CAP-043 capability anchor. ADR-051 §D1/§D2 + ADR-047 §D8b + ADR-042 citations. |
