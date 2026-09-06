---
document_type: adr
adr_id: ADR-051
status: proposed
date: 2026-09-05
subsystems_affected: [SS-01, SS-04, SS-07]
supersedes: null
superseded_by: null
---

<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->

# ADR-051: Layer-2 Two-Mechanism Size-Triggered Shard Rotation — Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding

## Context

ADR-047 §Decision 8b ratified Layer 2 (S-25.02, REGISTERED BACKLOG) as continuous size-triggered
sharding of append-only cycle artifacts (`decision-log.md`, `burst-log.md`, `lessons.md`) into
capped shards, deriving the cap from `PRACTICAL_FUEL_CEILING`, `WORST_CASE_FUEL_PER_BYTE`,
`MAX_SINGLE_RECORD_BYTES`, and `SAFETY_MARGIN`. Layer 1 (S-25.01) shipped 2026-09-03
(`f3f9b3a1`) and is producing forensic telemetry: 708 `plugin.indeterminate` events in the first
48 hours (699 `cause=fuel`, 9 `cause=epoch`), read directly from
`.factory/logs/dispatcher-internal-2026-09-0{4,5}.jsonl` as part of this ADR's own preparation
(S-25.02 F1 Delta Analysis, `.factory/cycles/v1.0-brownfield-backfill/S-25.02-f1-delta-analysis.md`).

The F1 analysis surfaced a scope-confirmation gate: only 19.8% of observed INDETERMINATE events
fall on the three files S-25.02's narrative names. The single largest contributor —
`.factory/specs/behavioral-contracts/BC-INDEX.md` at 45.2% of events — is a structured catalog
(cross-referenced BC IDs, per-subsystem counts, POLICY-7 title source-of-truth), not an
append-only log. The human resolved this scope question (D-1166) as **WIDEST SCOPE**: S-25.02
covers BOTH (A) the four cycle append-only logs per active cycle —
`decision-log.md`, `burst-log.md`, `lessons.md`, `session-checkpoints.md` (the fourth added
because F1 §0 found it the second-largest contributor at 19.5% of events and architecturally
identical in shape to the other three) — AND (B) `BC-INDEX.md`, a structured catalog requiring a
different sharding mechanism. This ADR designs both mechanisms and resolves the five Open
Questions F1 raised (OQ-1 already resolved by D-1166; OQ-2 through OQ-5 resolved below).

### A structural constraint the story draft did not address: PreToolUse hooks cannot redirect writes

S-25.02's AC-002/AC-003 (roll-before-write, PreToolUse-or-append-helper) implicitly assume the
gating mechanism can transparently redirect an in-flight `Edit`/`Write`/`MultiEdit` call to a
different target file when the current shard is full. `crates/hook-sdk/src/result.rs`'s
`HookResult` enum — the complete contract a PreToolUse hook can return — has exactly three
variants: `Continue`, `Block { reason }`, `Error { message }`. There is no `Redirect` or
"rewrite tool input" variant, and none of the host functions in `crates/factory-dispatcher/src/
host/` (`read_file`, `read_prefix`, `write_file`, `memory`, `path_util`, `exec_subprocess`,
`emit_event`) mutate the pending tool call's target path or payload. **A PreToolUse hook can
only allow or block the call the agent issued — it cannot transparently make that call land
somewhere else.** Any Layer-2 design that assumes silent redirection is not implementable against
the current dispatcher contract. Decision 1 below resolves this.

### Real fuel/byte data exists from a comparable validator

ADR-042 measured `validate-cross-site-correspondence` (routed, like `convergence-tracker`,
through `hook-plugins/legacy-bash-adapter.wasm`) directly against production-shaped fixtures at
the actual near-exhaustion boundary: `fuel = 2,585,970 + 53.18 × payload_bytes` (linear fit,
R² = 0.998790; quadratic term negligible — fuel cost confirmed linear, not superlinear, for that
validator). This is the best available real-world "worst-case fuel-per-byte" anchor for a
legacy-bash-adapter-routed validator and grounds Decision 2's provisional constants. It is not
assumed to transfer exactly to `validate-burst-log` or `regression-gate` (native WASM crates with
different internal logic — see Decision 2's harness design), only used as a conservative floor.

### BC-INDEX.md's own dominant growth vector is its frontmatter changelog, not its BC tables

Direct measurement (2026-09-05) of `.factory/specs/behavioral-contracts/BC-INDEX.md`
(539,713 bytes total, 2,621 lines, 1,997 BCs) shows the frontmatter block (lines 1–552, mostly the
`changelog:` sequence) is **177,305 bytes** — larger than the biggest single per-subsystem BC
table (`### SS-05`, 661 BCs, ~88,695 bytes; `### SS-06`, 592 BCs, ~85,407 bytes). The longest
single physical line in the file is 16,521 bytes, inside a `changelog:` item. This is the exact
unbounded-append-in-frontmatter pattern ADR-049 already named and partially mitigated: ADR-049
§Decision 6 built a **manual** CLI safety-net (`rotate_changelog`,
`crates/last-amended-migrate/src/rotate.rs`) for rotating an over-long `changelog:` sequence into
a per-cycle archive, but that tool is operator-invoked, not size-triggered or dispatcher-mediated.
Decision 7 below reuses `rotate_changelog` as a library call from the SAME native gate this ADR
introduces for mechanism A, automating what ADR-049 could only do manually.

### Cohort B plugin identity correction (OQ-5)

ADR-047 §8a's Cohort B table and S-25.02's own AC-006 cite a plugin named
`validate-burst-log-structure`. This plugin does not exist in `hooks-registry.toml`. The actual
registered plugin is `validate-burst-log`, with two `[[hooks]]` entries: one
`event="PostToolUse" tool="^(Edit|Write|MultiEdit)$"` (the content-scanning, fuel-exhausting arm
this ADR and Cohort B concern) and one `event="PostToolUse" tool="^Bash$"` (an unrelated,
exec-free git-commit chain-detection gate reading `payload.extra.git_context` — never scans file
content, not part of the Cohort B fuel-exhaustion problem). This ADR corrects ADR-047 §8a in the
same burst (see Changelog on that file) using the same erratum-class amendment process ADR-047
v1.5 itself set as precedent for the `validate-factory-path-staging` "artifact-write side"
correction.

---

## Decision

### Decision 1 — Native (Non-WASM) PreToolUse Shard-Cap Gate; Roll-Before-Write via Block-and-Retry

Layer 2's rotation trigger is implemented as a **native Rust function inside the dispatcher's own
PreToolUse dispatch path**, architecturally analogous to the already-established native
crash-path check `block_if_marker_check` (`crates/factory-dispatcher/src/indeterminate_marker.rs`,
consulted from `executor.rs`'s `plugin_block_if_marker`) — a dispatcher-native check consulted
outside the WASM-plugin-registry loop, not a new WASM plugin. This resolves AC-002's "(a) PreToolUse
WASM hook ... or (b) append helper" framing in favor of a third option that is dispatcher-mediated
like (a) but has no WASM sandbox or fuel budget of its own, avoiding exactly the self-inflicted
INDETERMINATE-loop risk F1 §4 flagged for a naive WASM-based size check.

**Placement:** invoked at the top of the dispatcher's PreToolUse handling for `Edit`/`Write`/
`MultiEdit` tool calls, before the registry-driven plugin loop (the registry already dispatches
several `event="PreToolUse" tool="^(Edit|Write|MultiEdit)$"` entries today — e.g. priorities 20,
50, 60, 80, 90, 140 in `hooks-registry.toml` — confirming this event/tool combination is an
established dispatch point; the new check runs before all of them, since a rotation must be
resolved before any registry plugin risks reading a not-yet-rotated oversized file).

**Config source:** a new `[[shard]]` table appended to `hooks-registry.toml` (or a sibling
`shard-config.toml` — final file TBD at F4; either way it is data, not code, per ADR-004's
TOML-for-config decision) naming each sharded artifact's stem, containing artifact, cap-formula
inputs, and current cap.

**Algorithm (per matching Edit/Write/MultiEdit call):**
1. Resolve the tool call's target path against the `[[shard]]` config. If no match, `Continue`
   immediately (zero added latency for the ~99% of writes untouched by Layer 2).
2. If matched: `stat()` the current shard's byte size (metadata only — no file content read into
   memory, addressing F1 §4's fuel-budget-for-the-checker-itself risk natively, since native code
   has no fuel budget at all).
3. Compute `projected_size = current_size + payload_size` (payload size is read directly from the
   tool call's own parameters — `content` for `Write`, `new_string` length delta for `Edit`/
   `MultiEdit` — no file re-read required).
4. If `projected_size <= shard_cap_bytes`: `Continue`.
5. If `projected_size > shard_cap_bytes`: **perform the roll** (Decision 3) — seal the current
   shard (rename to its sealed name), create a fresh empty current file, atomically publish the
   updated shard index (Decision 4) — THEN return `HookResult::Block { reason: "Shard <artifact>
   rotated (cap <N> bytes reached); the current shard is now empty. Retry your write: if you
   used Edit, reissue as a fresh Write containing only your new entry (Edit's old_string will no
   longer match); if you used Write, simply retry unchanged." }`.

**Why block-and-retry, not silent pass-through:** because `HookResult` cannot redirect or mutate
the pending call (see Context above), and because an `Edit` call's `old_string` is matched against
whatever content exists at apply-time — if the gate rotated the file out from under an in-flight
`Edit`, the `Edit` would fail with a confusing "old_string not found" tool error instead of a
clear, actionable message. Blocking with an explicit retry instruction converts an opaque failure
into an actionable one and requires **zero size-awareness from the agent** — the agent does not
predict when rotation will happen; it only reacts to an explicit, mechanically-generated
instruction when it does. This satisfies AC-002's "no LLM-side awareness of shard size is required
or permitted" — the agent needs no anticipatory awareness, only reactive compliance with a
dispatcher-authored message, exactly as agents already do for `validate-factory-path-staging`
blocks today.

**Effect on AC-003 ("roll-before-write ... writing an oversized shard is forbidden"):** satisfied
exactly — the roll (seal + fresh-create + index publish) completes, and the blocked call is never
applied, so the sealed shard's final size is always `<= shard_cap_bytes` and the new current shard
starts at 0 bytes. No shard is ever observed in an over-cap state by any downstream reader.

**Trigger-shape dispatch — BOTH trigger shapes are owned by this SAME gate/BC, with distinct
read-cost models (fix-burst amendment, F-S2502-F2-005).** The algorithm above (steps 1–5) is
written for mechanism A's "flat append-only file" artifact shape, whose trigger is
byte-size-denominated and reads ONLY filesystem metadata (`stat()`, step 2 — no file content
enters memory). Mechanism B1 (Decision 7, BC-INDEX's frontmatter `changelog:` array) is a SECOND,
structurally different artifact shape dispatched by the SAME gate and owned by the SAME BC
(BC-1.18.005), NOT a separate trigger mechanism, but its trigger is **item-count-denominated, not
byte-size-denominated**, and therefore cannot be evaluated from `stat()` metadata alone:

1. **Config declares the artifact's shape.** The `[[shard]]` config entry (or shard-index TOML)
   this Decision's step 1 resolves against carries a `shape` field with two values today: `"flat"`
   (mechanism A; byte-size trigger) or `"frontmatter-changelog-array"` (mechanism B1; item-count
   trigger). The gate dispatches to the shape-appropriate check based on this field — this is the
   SAME "artifact-shape case" dispatch BC-1.18.009 Postcondition 3 already describes; this
   amendment makes explicit that the TRIGGER READ, not just the roll action, differs by shape.
2. **Item-count trigger read cost:** for `"frontmatter-changelog-array"`-shaped artifacts, the
   check parses the target file's frontmatter far enough to count the existing `changelog:`
   sequence's items (a bounded read: the live sequence is itself capped at N items by this same
   mechanism after every prior rotation — see Decision 7's corrected block-and-retry contract
   below — so this is never an unbounded-growth read, unlike a naive "read the whole file" cost
   model would be). This is MORE than a `stat()` call (it requires reading and lightly parsing
   frontmatter content) but is still native, fuel-budget-free dispatcher code, not a WASM plugin
   invocation — Decision 1's "why native, not WASM" rationale (no fuel budget of its own) applies
   identically to this shape.
3. **Trigger condition:** `current_item_count + 1 > N` (config value, per BC-1.18.009 Postcondition
   1) — never a byte-size comparison for this shape. `shard_cap_bytes` (the byte-size formula,
   Decision 2) still bounds `BC-INDEX.md`'s TOTAL byte footprint as a whole-artifact concern, but
   the ROTATION decision within B1 specifically is item-count-based (this is the same distinction
   BC-1.18.009 EC-004 already draws for a single oversized `changelog:` item).
4. **Ownership:** both trigger shapes are specified as BC-1.18.005 postconditions (this BC owns
   "the formula-and-trigger boundary" for every artifact shape the gate handles, per BC-1.18.005's
   own Postcondition 3 framing) — there is no separate, competing trigger-owning BC. Product-owner
   MUST add an explicit BC-1.18.005 postcondition for the item-count shape (see the companion F2
   architecture-delta doc's BC Authorship Inputs table for the exact obligation).

### Decision 2 — Shard Cap Formula: Calibration Method and Provisional Constants (OQ-4)

**Method: synthetic calibration harness, not extended production observation.** F1 §2/§6
recommended the harness over waiting for a larger production sample (48h of data is too thin for
a defensible worst-case percentile, and a harness can construct adversarial inputs directly). This
ADR adopts that recommendation.

**Harness design (owned by `performance-engineer`, co-run with `implementer`, executed once at
F4 before BC-1.18.005's postconditions are treated as final-locked; re-run whenever
`DEFAULT_FUEL_CAP` changes, per the note below):**

1. Generate synthetic fixtures shaped exactly like the real artifact (markdown table/heading
   structure matching `decision-log.md`/`burst-log.md`/`lessons.md`/`session-checkpoints.md`), at
   a geometric series of sizes bracketing each provisional cap estimate below with margin (e.g.
   16 KiB, 32 KiB, 48 KiB, 64 KiB, 96 KiB, 128 KiB, 256 KiB) — **adversarially, not
   average-case, constructed**: maximum D-NNN cross-reference density for decision-log-shaped
   fixtures, maximum monotonicity-check targets for convergence-tracker-shaped fixtures, maximum
   cross-artifact reference count for regression-gate-shaped fixtures — because ADR-047 §8b is
   explicit that the denominator must be worst-case, not average-case, fuel-per-byte.
2. Dispatch each of the three Cohort B plugins (`validate-burst-log`'s Edit/Write arm,
   `regression-gate`, `convergence-tracker`) directly against each fixture size, in isolation
   (the same production-fixture-measurement methodology ADR-042 already used against
   `validate-cross-site-correspondence`).
3. Capture `fuel_consumed` from `plugin.completed`/`plugin.timeout` telemetry for each
   (plugin, fixture-size) pair — `VSDD_SINK_FILE` diagnostic capture (CLAUDE.md) is the concrete
   mechanism.
4. Fit `fuel_consumed` vs. `bytes` per plugin. Check the quadratic term's contribution to R²
   exactly as ADR-042 did; if superlinearity is detected (unlike ADR-042's finding for
   `validate-cross-site-correspondence`), `WORST_CASE_FUEL_PER_BYTE` MUST be taken as the LOCAL
   marginal rate at the largest tested size — `(fuel(size_max) - fuel(size_max - Δ)) / Δ` — never
   the global average slope, because a global average understates the true marginal cost exactly
   at the boundary the cap formula must protect.
5. **Per-artifact cap = MIN over every Cohort B plugin that reads that artifact** (the
   "Cross-Validator Minimum Rule"): `burst-log.md`'s cap is bound by `validate-burst-log` AND
   `regression-gate` AND `convergence-tracker` (all three read it per ADR-047 §8a's table);
   `decision-log.md`/`lessons.md`/`session-checkpoints.md`'s caps are bound by `regression-gate`
   and `convergence-tracker` only (`validate-burst-log` does not read them). A single global cap
   across all four artifacts is NOT used, because it would be needlessly conservative for
   artifacts only two of the three validators read.
6. Measure `MAX_SINGLE_RECORD_BYTES` empirically: the largest single physical line across every
   `.factory/cycles/*/{decision-log,burst-log,lessons,session-checkpoints}.md` in the repository
   (not just the active cycle), via `awk '{print length}' | sort -rn | head -1` per file family.

**Provisional constants (grounded in real data available today; every value below is
PROVISIONAL and MUST be replaced by the F4 harness's measured values before BC-1.18.005 is
treated as final — the formula shape is locked now, the numbers are not):**

| Constant | Provisional value | Derivation | Locked at F4? |
|---|---|---|---|
| `PRACTICAL_FUEL_CEILING` (today) | 8,000,000 | 80% of the CURRENTLY-EFFECTIVE `DEFAULT_FUEL_CAP` (10,000,000 — the value the bundled operator-level binary and marketplace cache actually enforce as of rc.23/rc.25 per CLAUDE.md's own diagnostic table; the develop-branch 20,000,000 from ADR-042 is NOT yet effective at the operator level and MUST NOT be used until a release ships it) | YES — replace 80% haircut with the harness's measured "reliably completes" percentile |
| `PRACTICAL_FUEL_CEILING` (post rc.24, informational only) | 16,000,000 | 80% of 20,000,000, for forward reference once the release ships | YES — recompute when `DEFAULT_FUEL_CAP=20M` is confirmed live at the operator level; re-run the harness, do not just multiply |
| `WORST_CASE_FUEL_PER_BYTE` | 106.36 | 2× ADR-042's measured 53.18 fuel/byte linear coefficient (conservative floor from a comparable legacy-bash-adapter-routed validator; `validate-burst-log`/`regression-gate` are native WASM crates with independent logic and MUST be measured directly, not assumed to share this coefficient) | YES — per-plugin measured value from harness step 4 |
| `MAX_SINGLE_RECORD_BYTES` | 16,384 | 64% margin over the largest single physical line directly measured in the active cycle's four append-log files today (9,987 bytes, `decision-log.md`, 2026-09-05) | YES — harness step 6's full-repository sweep |
| `SAFETY_MARGIN` | 8,192 | Buffer for shard-index-entry + shard-header overhead; round provisional figure, not yet measured against the actual index schema's per-entry byte cost (Decision 4) | YES — recompute once the shard-index schema's real per-entry size is known |

**Worked example (today's ceiling, illustrative only — NOT the locked value):**
`shard_cap_bytes <= (8,000,000 / 106.36) - 16,384 - 8,192 = 75,216 - 24,576 = 50,640`, rounded
down to a clean **49,152 bytes (48 KiB)** provisional cap. Once `DEFAULT_FUEL_CAP=20M` is
confirmed live and the harness is re-run: `(16,000,000 / 106.36) - 24,576 = 125,856`, rounded down
to **122,880 bytes (120 KiB)**. Both numbers are placeholders for the harness's actual per-plugin,
per-artifact output — they exist here only so BC-1.18.005's postcondition can cite a concrete
formula with a concrete illustrative instance, per CLAUDE.md's "no formula with no constants"
discipline.

**Immediate consequence the story draft did not anticipate — a one-time backfill split is
required, not just future-write protection.** At today's ~48 KiB provisional cap, the FOUR
existing monolithic append-log files (908,938 / 806,198 / 234,731 / 830,621 bytes for
decision-log/burst-log/lessons/session-checkpoints respectively) already exceed the cap by 5–19×.
AC-002/AC-003 as worded only gate FUTURE writes; they do not, by themselves, retroactively split
the four files that are already far over any sane cap. **This ADR requires a mandatory one-time
backfill-split task at F4 activation** (analogous in spirit to ADR-049's one-time migration
pattern) that splits each existing monolithic file into `ceil(current_bytes / shard_cap_bytes)`
sealed shards plus a fresh current file, publishing the shard index for the full pre-existing
history in the same operation. This is a new BC-authorship input (see the "BC Authorship Inputs"
section of the companion F2 delta doc) — without it, Layer 2 "prevents future overflow" but never
actually shrinks the artifacts that are causing today's 708 INDETERMINATE events, which would be
an incomplete, non-production-grade delivery of the story's own stated purpose.

### Decision 3 — Stable-Current-Filename Addressing for Append-Log Shards (OQ-2, mechanism A)

The artifact's canonical, unchanging filename (e.g. `decision-log.md`) is ALWAYS the current
(actively-written) shard. Sealing a shard **renames it away** to `<stem>.<seq:04>.md` (e.g.
`decision-log.0001.md`) and creates a fresh empty file at the canonical name. This is the
addressing resolution for OQ-2's option (a) — achieved by naming convention, not a symlink (a
symlink would need to be re-pointed atomically alongside the seal+create step and adds a
platform-portability concern the `factory-dispatcher` cross-compilation targets (darwin/linux/
windows) would need to separately verify; a plain rename achieves the same transparency with one
fewer moving part).

**Consequence for shard-UNAWARE readers/validators (AC-007):** any validator or human command
that reads `decision-log.md` by its canonical name — including every one of the ~76 fail-open
production plugins with `path_allow` globs like `.factory/cycles` (directory-scoped, not
filename-scoped, per F1 §4's own regression-risk finding) — continues to see the CURRENT/latest
shard transparently, with ZERO code change required, because the canonical filename never moves.
This covers the common case (a validator or human cares about the latest state — e.g. the D-chain
currency check, `check_d_chain_currency`/BC-5.39.006, cares about the most recent D-NNN).

**Consequence for whole-corpus readers:** a reader needing FULL history (e.g. `grep -n "D-1234"`
across all shards) uses the glob `decision-log*.md` (matches both the current file and every
sealed `decision-log.NNNN.md`). **Corrected sort-order rationale (fix-burst amendment,
F-S2502-F2-cosmetic):** the shared prefix across all matches is `decision-log` (not
`decision-log.`) — the byte immediately after that shared prefix is what a lexicographic sort
actually compares: `.` (0x2E, from a sealed shard's `.NNNN.md` suffix) vs. `.` (0x2E, from the
current file's own `.md` suffix) are IDENTICAL at that position, so the comparison falls through
to the NEXT byte — a digit (`0`–`9`, 0x30–0x39) for a sealed shard vs. `m` (0x6D, from `.md`) for
the current file. Since every digit byte (0x30–0x39) is numerically less than `m` (0x6D), every
sealed shard sorts before the current file. The conclusion (current `decision-log.md` sorts last)
is unchanged and correct; only the originally-stated operative comparison (`.` vs. `0`–`9`) was
wrong — the true deciding comparison is digit-vs-`m`, one byte further into the shared prefix. No
special-casing is needed in a `sort`-fed pipeline either way.

### Decision 4 — Shard Index Schema (mechanism A)

One shard-index file per sharded artifact, published atomically (temp-file-then-rename, the same
pattern already established by `write_indeterminate_marker`
(`crates/factory-dispatcher/src/indeterminate_marker.rs`) and `write_atomic`
(`crates/last-amended-migrate/src/atomic_write.rs`) — no new atomic-write primitive is needed,
the new `shard_manager` module reuses one of these):

```toml
# .factory/cycles/<cycle>/<artifact-stem>.shard-index.toml
schema_version = 1
artifact_stem = "decision-log"
current_shard = "decision-log.md"
shard_cap_bytes = 49152           # calibrated per Decision 2; locked at F4
max_single_record_bytes = 16384
safety_margin_bytes = 8192
practical_fuel_ceiling = 8000000
worst_case_fuel_per_byte = 106.36

[[shard]]
seq = 1
path = "decision-log.0001.md"
sealed_at = "2026-09-10T00:00:00Z"
bytes_at_seal = 49087
```

The index and the newly-sealed shard file MUST be written in the SAME native gate invocation,
which the native-check placement (Decision 1) makes structural rather than conventional: because
the gate performs both the rename (seal) and the index publish before returning `Block`, and
because both are filesystem writes issued by the SAME PreToolUse invocation before any
`git add`/`git commit` occurs, they are guaranteed to land in the SAME subsequent factory-artifacts
commit — satisfying AC-004/TD-VSDD-053 by construction, not by state-manager discipline (this
directly resolves F1 §4's "TD-VSDD-053 vs. shard+index atomicity" MEDIUM risk).

### Decision 5 — `/compact-state` Interaction (OQ-3)

`/compact-state` (`plugins/vsdd-factory/skills/compact-state/SKILL.md`) today appends to
`burst-log.md`/`lessons.md`/`session-checkpoints.md` via direct `Edit`/`Write` tool calls (Step 3.2
of its procedure) — there is no existing "append helper" abstraction it or any other skill calls
through. Because Decision 1's gate is dispatcher-native and intercepts `Edit`/`Write`/`MultiEdit`
regardless of which agent or skill issued the call, **`/compact-state` gets shard-awareness for
free with zero amendment to its own procedure** — its `Edit`/`Write` calls against the four
append-log artifacts are gated exactly like any other agent's, and if a rotation fires mid-
extraction, `/compact-state` receives the same `Block`-with-retry-instruction message any other
caller would. This resolves OQ-3: **no `/compact-state` amendment is required** for the gate
mechanism itself. One (mechanical, small) amendment IS required to `/compact-state`'s own Step 3
guidance text: note that a `Block` response during extraction means "retry as a fresh `Write` of
only the just-extracted section, not an `Edit`," matching Decision 1's retry contract, so the
skill's own retry loop (if it has one) or the operator reading its output understands a mid-run
block is expected/handled behavior, not a failure. This is a documentation-only change to the
skill file, owned by whichever agent next revises `compact-state/SKILL.md` (not authored in this
architecture burst — SKILL.md content is outside `.factory/specs/` and outside this F2 dispatch's
write scope).

### Decision 6 — Retention/Compaction Companion Policy (AC-005)

Shard count per artifact is unbounded absent compaction (ADR-047 §8b "honest shard count
accounting"). Retention policy: shards older than the current cycle's **10 most recent shards**
(a round, config-adjustable number set in the shard-index `schema_version = 1` block as a future
`retention_count` field — the exact value is a BC-authorship decision, not fixed immutably by this
ADR) are archived by moving them under
`.factory/cycles/<cycle>/archive/<artifact-stem>/<sealed-filename>`, still `.factory/cycles`-glob-
visible (preserving `path_allow` compatibility per Decision 3's consequence) but excluded from the
"whole-corpus" glob whole-corpus validators use by default (they must opt into `archive/` inclusion
explicitly). Whole-corpus validators operating without the opt-in are `O(active shards)`, not
`O(1)` and not `O(all shards ever)` — the correct honest claim per ADR-047 §8b.

**Whole-corpus history-scanning validator enumeration and POLICY-1 reconciliation (fix-burst
amendment, F-S2502-F2-008).** The default "opt-in required for `archive/`" posture above is safe
ONLY for validators whose correctness concern is "the current/recent state," not "the complete
historical ID space." Enumerated against the actual codebase (not assumed):

- **Verified NOT affected — STATE.md-scoped, never reads the cycle `decision-log.md` file:**
  `check_d_chain_currency`/`scan_max_d_nnn`/`scan_max_decision_log_id`
  (`crates/hook-plugins/validate-dispatch-advance/src/lib.rs`) parse ONLY the content of the
  `STATE.md` file being written (via `host::read_file` in that plugin's own PostToolUse arm),
  extracting the max `D-NNN` cited in STATE.md's OWN embedded `## Decisions Log` summary
  table/`current_step:` field. `STATE.md` is explicitly NOT one of Layer 2's four mechanism-A
  sharded artifacts (Decision 1's scope: `decision-log.md`/`burst-log.md`/`lessons.md`/
  `session-checkpoints.md`) — this validator never reads the cycle `decision-log.md` file at all,
  archived or not, so shard rotation/archival has zero effect on it. (This corrects an imprecise
  earlier characterization of this validator as a "decision-log.md whole-corpus scanner" — direct
  code inspection shows it is not.)
- **Verified NOT affected — same reasoning:** `check_decisions_log_monotonicity`
  (`crates/hook-plugins/validate-state-structure/src/lib.rs`) operates on STATE.md's own content
  exclusively (extracted via that file's `extract_section(content, "## Decisions Log")`), never on
  the cycle `decision-log.md` file.
- **Verified NOT affected — current-shard-scoped BY DESIGN, correctly so:**
  `validate-closes-completeness`'s decision-log arm (`is_decision_log_target`,
  `crates/hook-plugins/validate-closes-completeness/src/lib.rs`) fires PostToolUse against the file
  being WRITTEN — i.e., the current/live shard under Decision 3's stable-current-filename
  convention — to check Closes-annotation completeness on the entries just written. It has no
  legitimate reason to scan sealed or archived shards (those entries' Closes-completeness was
  already checked when THEY were the current shard); no amendment needed.
- **Verified NOT affected:** `validate-cross-site-correspondence`'s `is_volatile_path` classifier
  (`crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs`) matches on
  `Path::file_name()` exact-equality against `"decision-log.md"`/`"burst-log.md"`/`"lessons.md"`,
  which continues to resolve correctly under Decision 3's stable-current-filename convention
  regardless of how many times rotation or archival has occurred — the matched filename never
  moves.
- **Verified NOT affected — Cohort B validators are correctly current-shard-scoped:**
  `validate-burst-log`'s Edit/Write arm, `regression-gate`, `convergence-tracker` (ADR-047 §8a
  Cohort B) fire PostToolUse against the file being written, for FUEL-BUDGET/content-regression
  concerns bounded to the artifact currently being mutated — this is exactly what Decision 2's cap
  calibration targets, and these validators have no whole-history concern that archival could
  break.
- **REQUIRES archive-inclusive whole-corpus mode — genuine gap, not previously reconciled:**
  POLICY-1 (`append_only_numbering`, `.factory/policies.yaml` id 1: "All VSDD identifiers are never
  renumbered or reused... Filename slugs are immutable") is `enforced_by: [adversary-prompt,
  consistency-validator]` with `lint_hook: null` — i.e., there is no automated WASM/bash hook
  implementing this policy today; it is an LLM-agent-level audit run by the adversary and
  consistency-validator agents, whose `verification_steps` include "Scan all index files for
  retired/removed IDs." A `D-NNN` (or `BC`/`VP`/story-ID) append-only/no-reuse audit is a
  correctness property over the ENTIRE historical ID space, not a "latest state" concern — UNLIKE
  `check_d_chain_currency`'s legitimate latest-only scope above. If this audit uses the DEFAULT
  whole-corpus glob (which excludes `archive/` per this Decision's general rule), an ID whose sole
  prior occurrence has aged into `archive/<artifact-stem>/` becomes invisible to the audit, and a
  SUBSEQUENT reuse of that same ID would go undetected — a silent POLICY-1 enforcement gap directly
  caused by archival, not present before Layer 2 existed. **Resolution: POLICY-1's
  `consistency-validator`/`adversary-prompt` enforcement paths MUST default to the
  ARCHIVE-INCLUSIVE whole-corpus mode** (glob `<stem>*.md` UNION `archive/<stem>/*.md`) for any
  D-NNN/BC/VP/story-ID append-only, gap, or uniqueness audit against a Layer-2-sharded artifact —
  this is an explicit, named carve-out from this Decision's general "opt-in required" default,
  justified because append-only-numbering integrity is inherently a whole-history property. Because
  `lint_hook: null` today, this is presently a documentation/agent-instruction obligation (the
  `consistency-validator` and adversary-prompt agent definitions, and POLICY-1's own
  `verification_steps` in `.factory/policies.yaml`), not a new WASM/native validator this ADR
  designs — routed to whichever agent next revises `.factory/policies.yaml` (via the
  `policy-add`/`policy-registry` skill, per CLAUDE.md routing; not authored by the architect
  directly) to add: "Scan MUST include `archive/<artifact-stem>/` for any Layer-2-sharded artifact,
  not just active shards." If POLICY-1 is ever automated into a WASM/native hook in a future story,
  that hook's design MUST inherit this archive-inclusive default from day one.

### Decision 7 — BC-INDEX Two-Level Structured-Catalog Sharding (mechanism B)

BC-INDEX.md has two independent, differently-shaped growth vectors (Context above), so mechanism
B is itself two sub-mechanisms:

**B1 — Frontmatter `changelog:` array rotation (reuses mechanism A's rotation primitive, NOT a
new mechanism).** The `changelog:` YAML sequence in BC-INDEX.md's frontmatter is, structurally,
an append-only log embedded inside an otherwise-structured document — identical in shape to the
four mechanism-A artifacts, just YAML-list-item-shaped instead of markdown-section-shaped. The
SAME native gate (Decision 1) is extended with one additional artifact-shape case: when the
matched artifact is "frontmatter changelog array" rather than "flat append-only file," and the
item-count trigger (Decision 1's trigger-shape dispatch, above) fires, the gate performs a **rotate
(trim) step**, using `rotate_changelog` (`crates/last-amended-migrate/src/rotate.rs`, already
implemented and tested — ADR-049 §Decision 6 built this exact primitive as a manual safety-net
tool) to move the overflowing tail into a new sealed changelog shard under
`.factory/specs/behavioral-contracts/BC-INDEX-changelog-shards/BC-INDEX-changelog.<seq:04>.md`,
leaving the live frontmatter's `changelog:` sequence at exactly N-1 items (room for exactly one new
item) — THEN returns `HookResult::Block` with a retry instruction, per the CORRECTED single-actor
contract below. This automates, for BC-INDEX specifically, exactly what ADR-049 could previously
only do via manual CLI invocation — no new rotation logic is designed, only a new automatic
size-triggered CALLER of the existing `rotate_changelog` function.

**CORRECTED (fix-burst amendment, F-S2502-F2-001, BLOCKER) — B1 is block-and-retry, identical in
actor-ownership shape to BC-1.18.006's mechanism-A contract; the original "gate rotates AND
prepends, then Continues" design is WITHDRAWN as internally unsound.** The v1.0 design of this
Decision (and BC-1.18.009 Postconditions 2 and 6 as originally drafted) had the gate perform BOTH
the rotate/trim step AND the prepend of the just-displaced-`last_amended` entry into the
now-shortened live `changelog:` sequence (via `prepend_changelog_item`,
`crates/last-amended-migrate/src/changelog.rs`), and THEN return `HookResult::Continue`, letting
the ORIGINATING agent's own `Edit`/`Write`/`MultiEdit` call — which independently already contains
that same prepend, per the ADR-049 §Decision 2 discipline the agent follows when it constructs its
own tool call — land on top. This is unsound for two independent reasons, one of which is a
grounded code fact, not a hypothetical:

1. **Double-actor prepend.** Two different writers (the gate, via `prepend_changelog_item`; the
   agent, via its own already-composed `Edit`/`Write`/`MultiEdit` payload) both perform the
   identical logical action — inserting the new `changelog:` item — in the same operation. Letting
   both land is either a literal duplicate entry (if both writes independently succeed and neither
   overwrites the other's target region) or a race depending on write order, neither of which is an
   acceptable observable outcome.
2. **Stale-payload clobber for `Write`/`MultiEdit` (the exact hazard BC-1.18.006 exists to
   prevent).** A `Write` call's `content` parameter is the agent's own COMPLETE, pre-computed
   file content, built from whatever frontmatter state the agent last read — which, for an
   in-flight call, is the PRE-rotation state (still containing the item the gate's rotation just
   moved to a sealed shard). If the gate performs its rotation and then returns `Continue`, the
   agent's stale full-file `Write` payload lands OVER the gate's just-rotated file, silently
   RE-INTRODUCING the just-archived tail item into the live frontmatter — undoing the rotation and
   guaranteeing the very next write re-triggers rotation again (an infinite churn loop, not merely
   a cosmetic duplicate). This is exactly the class of hazard BC-1.18.006's Description names as
   "structurally forbidden" for mechanism A ("a PreToolUse hook cannot safely mutate a file
   underneath an in-flight Edit/Write") and solves via block-and-retry — B1's v1.0 design violated
   that same principle it claims (BC-1.18.009 Postcondition 6, v1.0) to be a "deliberate
   divergence," when it is in fact a regression to the exact hazard mechanism A was designed to
   avoid.
3. **Grounded in the actual `rotate_changelog` signature, not assumed:** `rotate_changelog`'s
   implementation (`rewrite_source_after_rotation` in `rotate.rs`) is a PURE TRIM — it keeps only
   `keep_items` (the retained N-1 most-recent items) and writes a `changelog_archive:` discoverability
   pointer; it never calls `prepend_changelog_item` itself and has no parameter for a "new item to
   insert." `prepend_changelog_item` (`changelog.rs`) is a SEPARATE, independently-callable
   function. Nothing in the ALREADY-SHIPPED library requires or expects the trim step and the
   prepend step to be fused into one caller — the v1.0 BC-1.18.009 design fused them by
   specification choice, not by library constraint, and that fusion is what created the
   double-actor hazard.

**Corrected contract: the gate performs ONLY the rotate/trim step (via `rotate_changelog`), NEVER
the prepend.** The prepend of the new `changelog:` item is EXCLUSIVELY the responsibility of
whichever `Edit`/`Write`/`MultiEdit` call ultimately lands successfully — the original call, if no
rotation was needed, or the RETRIED call, if the gate had to rotate first. Sequence when rotation
is needed:

1. Agent issues `Edit`/`Write`/`MultiEdit` against `BC-INDEX.md`'s frontmatter, constructing its
   payload per the standard ADR-049 §Decision 2 discipline (displaced `last_amended` → new
   `changelog:` item), exactly as it always has — this BC introduces no change to how the AGENT
   composes its own payload.
2. The gate's item-count trigger (Decision 1, trigger-shape dispatch) evaluates
   `current_item_count + 1 > N` against the file's CURRENT (pre-write) state. If false: `Continue`
   — no rotation, the agent's own prepend lands normally, unmodified (EC-002, unchanged from v1.0).
3. If true: the gate invokes `rotate_changelog` to trim the live `changelog:` sequence down to
   N-1 items (archiving the overflow tail), publishes the sealed changelog shard, THEN returns
   `HookResult::Block` with an explicit retry instruction: "BC-INDEX.md's `changelog:` sequence was
   rotated to make room (oldest item(s) moved to `BC-INDEX-changelog-shards/
   BC-INDEX-changelog.<seq:04>.md`); the frontmatter now has N-1 items. Retry your write: if you
   used `Edit`, reissue as a fresh `Write` or a fresh `Edit` re-read against the current
   (post-rotation) file, since your original `old_string`/`new_string` pair may no longer match; if
   you used `Write`, recompute your `content` payload against the current (post-rotation) file
   before retrying — do not resubmit your original payload unchanged, since it reflects
   pre-rotation state." (This is a shape-appropriate specialization of BC-1.18.006 Postcondition
   2's retry-wording contract, not a divergent one — B1's wording differs from mechanism A's only
   because the underlying state changed shape, not filename.)
4. Agent retries, reading/recomputing against the now-rotated file; its retried prepend lands via
   `Continue` (item count is now `N-1+1 = N`, not exceeding N) — SINGLE ACTOR, exactly once.

**Atomicity is preserved identically to mechanism A's pattern:** the rotate/trim step (sealed
changelog shard write + live-frontmatter trim) completes BEFORE the `Block` is returned, in the
SAME native-gate invocation — the same "seal-then-block" atomicity guarantee Decision 4 already
establishes for mechanism A, just with `rotate_changelog`'s trim in place of mechanism A's
rename-based seal. The actual NEW-item prepend, like mechanism A's actual oversized write, lands in
a SEPARATE subsequent tool call (the retry) — this is not a regression in atomicity, since
mechanism A's own contract already splits "the roll" (atomic, same-invocation) from "the content
that triggered it" (a separate, later call) in exactly this way.

**B2 — Per-subsystem body-table sharding (the genuinely novel mechanism).** BC-INDEX.md's body is
already partitioned by the 10 `### SS-NN` headings that exist today (`### SS-01` through
`### SS-10`), each a self-contained BC table for that subsystem. This partition is **not
something this ADR invents** — it already exists in the live file and mirrors the ARCH-INDEX
Subsystem Registry's `BC-S Prefix` column (`BC-1` -> `SS-01`, `BC-2` -> `SS-02`, ..., `BC-10` ->
`SS-10`), which is itself a stable, already-documented, already-authoritative mapping (ARCH-INDEX
§Subsystem Registry, POLICY 6). B2's design is: split each `### SS-NN` section into its own file
`.factory/specs/behavioral-contracts/shards/BC-INDEX-SS-NN.md`, leaving BC-INDEX.md's body as a
lean top-level index (§Summary + §Subsystem Shard Manifest + cross-cutting invariants, no full
per-BC tables). **Addressing (OQ-2 for mechanism B) requires NO index lookup for the first level**:
any reader wanting `BC-X.YY.NNN`'s row computes its shard path directly from the ID's numeric
prefix (`BC-5.39.006` -> `shards/BC-INDEX-SS-05.md`) via the SAME deterministic mapping the
Subsystem Registry already publishes — this is mechanically simpler than mechanism A's addressing,
because the partition is by stable ID-prefix, not by time.

**When a subsystem shard itself exceeds cap (second-level split):** two subsystems already exceed
the 48 KiB provisional today-cap on their OWN section size alone — `### SS-05` (Pipeline
Orchestration, 661 BCs) at ~88,695 bytes and `### SS-06` (Skill Catalog, 592 BCs) at ~85,407 bytes,
both measured directly against the live file 2026-09-05. Both will need immediate second-level
sub-sharding at F4 activation, using the SAME native gate and the SAME cap formula (Decision 2),
but keyed by a **per-subsystem manifest** (`shards/BC-INDEX-SS-05.manifest.toml`,
`shards/BC-INDEX-SS-06.manifest.toml`) recording BC-ID-range boundaries per sub-shard (e.g.
`shards/BC-INDEX-SS-05.a.md` covering `BC-5.01.001`..`BC-5.30.099`), since a sub-shard boundary is
growth-based, not ID-prefix-deterministic, and DOES require a manifest read (unlike the top-level
subsystem split). The other 8 subsystems (SS-01 through SS-04, SS-07 through SS-10; SS-07's own
section measured at ~39,072 bytes, comfortably under the 48 KiB today-cap) are not expected to
need a second level at F4, though this is re-verified empirically once the actual post-split
per-subsystem file sizes are known — sub-sharding is triggered by the SAME size-check gate for
every subsystem, not hardcoded to SS-05/SS-06 specifically.

### Decision 8 — BC-INDEX Shard Manifest Schema and Reader Migration (mechanism B addressing)

```toml
# .factory/specs/behavioral-contracts/shards/BC-INDEX.shard-manifest.toml
schema_version = 1

[[subsystem_shard]]
ss_id = "SS-01"
bc_prefix = "BC-1"
path = "shards/BC-INDEX-SS-01.md"
sub_sharded = false

[[subsystem_shard]]
ss_id = "SS-05"
bc_prefix = "BC-5"
path = "shards/BC-INDEX-SS-05.md"          # becomes a stub pointer once sub_sharded=true
sub_sharded = true
sub_manifest = "shards/BC-INDEX-SS-05.manifest.toml"
```

**Reader/writer migration surface (the genuine cost B2 imposes, distinct from mechanism A's
zero-code-change stable-alias trick — flagged explicitly per the F1-anticipated asymmetry):**
every current touchpoint that opens `BC-INDEX.md` expecting to find a specific BC row or to scan
the full BC corpus must be updated to either (a) compute the shard path from the BC-ID prefix
(mechanical, one small helper function, for the common single-BC-lookup case) or (b) iterate the
shard manifest (for whole-corpus scans). Concretely, at minimum:
- **product-owner's BC authorship/amendment workflow** — write target becomes the per-subsystem
  shard file, not `BC-INDEX.md`'s body (the top-level file's body no longer contains BC rows to
  edit).
- **state-manager's POLICY 7/8 title-sync and count-propagation bursts** — count aggregation
  (`§Summary`) must sum across shard files' actual row counts rather than scanning one file
  in-place; `validate-count-propagation.sh`'s `_extract_counts` (ADR-049 audit finding 4) needs a
  companion pass across the shard set.
- **the adversarial-review skill's POLICY auto-load**, which reads `.factory/policies.yaml` (a
  small, separate file, NOT BC-INDEX.md) — verified NOT affected: `policies.yaml` is independent
  of BC-INDEX's sharding (see the companion F2 delta doc's Regression/Ripple section for the full
  verification).
- **consistency-validator's cross-reference checks** — any check that currently globs or
  full-text-scans `BC-INDEX.md` for an ID must instead consult the shard manifest or glob
  `shards/BC-INDEX-SS-*.md`.

This migration is REQUIRED at F4 (implementer scope, informed by BC authorship), is bounded (a
small, enumerable set of touchpoints, not an open-ended scan), and does not, on its own, justify
splitting BC-INDEX sharding into a follow-up story — see the companion F2 delta doc's "Split
Proposal Assessment" section for the explicit reasoning behind keeping this in S-25.02.

### Decision 9 — Cohort B Fail-Closed Flip Sequencing (AC-006, corrected)

Once Decision 1's gate is live and Decision 2's calibration is F4-locked (not provisional), the
THREE Cohort B validators — `validate-burst-log` (PostToolUse `^(Edit|Write|MultiEdit)$` arm
only — the Bash chain-detection arm is out of scope, see OQ-5 correction above),
`regression-gate`, `convergence-tracker` — are assigned `failure_policy = "fail-closed"` in
`hooks-registry.toml`, gated on ADR-039 §Decision 3 calibration confirmation for each, per
ADR-047 §8a's existing Cohort B framing (unchanged by this ADR except for the plugin-name
correction).

### Decision 10 — Governed One-Time Migration for the B2 BC-INDEX Body Split (fix-burst addition, F-S2502-F2-002, HIGH)

BC-1.18.008 already specifies a governed one-time migration (content-preservation,
crash-atomicity, staging/rollback) for mechanism A's four append-log backfills. **Mechanism B2's
first-level split of `BC-INDEX.md`'s body into `shards/BC-INDEX-SS-NN.md` had no equivalent
migration BC — BC-1.18.010 as drafted specifies only the END-STATE (Decision 7/8's addressing
scheme), not the transition from today's monolithic body to that end-state.** Because
`BC-INDEX.md`'s H1-per-BC-row is the POLICY-7 title source-of-truth, a dropped or duplicated row
during this split corrupts title authority for that BC — this is not a cosmetic migration, it is a
governance-integrity-critical one, and MUST be elevated to a governed migration parallel to
BC-1.18.008.

**Resolution: a NEW BC (product-owner assigns; this ADR's illustrative numbering is
BC-1.18.011, the next free SS-01 slot after BC-1.18.010 at time of authoring — product-owner
confirms the exact free slot against BC-INDEX at authoring time, per this ADR's own numbering
convention for BC-1.18.008/009/010) governs the one-time B2 migration, modeled directly on
BC-1.18.008's structure.** Postcondition obligations this BC MUST encode (enumerated here as the
architect's authorship input to product-owner, per this dispatch's constraints):

1. **Content-preservation, byte-for-byte.** The concatenation of the ten (or more, once
   second-level sub-shards exist) resulting shard files, in `SS-01`..`SS-10` order, plus
   `BC-INDEX.md`'s own retained lean top-level body (`§Summary` + `§Subsystem Shard Manifest` +
   cross-cutting invariants — BC-1.18.010 Postcondition 1's end-state), reproduces the ORIGINAL
   (pre-split) `BC-INDEX.md`'s full per-BC-row content byte-for-byte — modulo the newly-introduced
   `§Subsystem Shard Manifest` section itself, which is new structural metadata, not migrated
   content. This is BC-1.18.008 Postcondition 6(a)'s exact analogue, applied to a content
   partition instead of a time partition.
2. **Independent-census integrity check — every BC row in EXACTLY one shard.** Before the split
   begins, capture an independent census: the complete set of `BC-X.YY.NNN` IDs present in the
   ORIGINAL (pre-split) `BC-INDEX.md` body (a fresh enumeration, not reused from any cached count),
   cross-checked against `BC-INDEX.md`'s own `total_bcs` frontmatter field (an independent
   count-oracle, e.g. 1,997 per BC-INDEX v5.50) as a sanity bound. After the split, verify: (a)
   every census ID appears in EXACTLY ONE resulting shard file (`shards/BC-INDEX-SS-NN.md`, or a
   sub-shard once second-level splitting applies) — never zero, never two; (b) the union of all
   shard files' row counts equals the pre-split census count exactly; (c) `BC-INDEX.md`'s own body,
   post-split, contains ZERO per-BC table rows (BC-1.18.010 Invariant 3). This is BC-1.18.008
   Postcondition 6(b)'s exact analogue (record-integrity), specialized to BC-INDEX's ID-keyed
   partition instead of decision-log's row-boundary partition, and is the "independent census"
   VP-128-class check BC-1.18.010 already specifies for the STEADY STATE — this migration BC
   specifies the ONE-TIME check that establishes that steady state correctly in the first place.
3. **Crash-atomicity: staging + verify + atomic replace, all-or-nothing.** Write all ten (or more)
   resulting shard files and the shard-manifest TOML to a staging location first; only after
   Postcondition 1 (content-preservation) and Postcondition 2 (independent census) both verify
   clean does the operation atomically replace `BC-INDEX.md`'s body and publish the shard-manifest
   at its canonical path, via the same temp-file-then-rename discipline BC-1.18.006 already
   establishes. This is BC-1.18.008 Postcondition 5's exact analogue.
4. **Rollback on verification failure.** If EITHER the content-preservation check OR the
   independent-census check fails, the migration ABORTS: `BC-INDEX.md`'s original monolithic body
   is left completely untouched (fail-loud, not partial-and-silent) — no partial set of shard files
   is ever treated as authoritative, and no partial `§Subsystem Shard Manifest` is published. This
   is BC-1.18.008 Postcondition 6's "hard gate" analogue and EC-004's exact analogue.
5. **Idempotency against a partially-completed prior attempt.** If a prior migration attempt left a
   valid partial shard-index/manifest state, re-running MUST either resume from the last
   verified-complete shard or detect the already-migrated state and skip re-splitting — never
   double-split. This is BC-1.18.008 Invariant 3's exact analogue.
6. **MUST cover the SS-05/SS-06 second-level sub-split within the SAME one-time migration
   operation, not a separate follow-on.** Both subsystems already exceed the provisional cap on
   their own section size alone (Decision 7's B2 sub-section) and require immediate second-level
   sub-sharding at the SAME F4 activation moment mechanism A's own backfill (BC-1.18.008) runs.
   This BC's content-preservation, independent-census, atomicity, and rollback obligations
   (Postconditions 1–5 above) apply IDENTICALLY at the sub-shard level for SS-05/SS-06 — i.e., the
   census for SS-05 verifies every `BC-5.YY.NNN` row lands in exactly one of
   `shards/BC-INDEX-SS-05.a.md`/`.b.md`/etc., with the SS-05-scoped total matching an independent
   pre-split count of `BC-5.*` rows specifically.
7. **No new Cohort-B dependency.** Unlike BC-1.18.008 (which BC-7.08.001's fail-closed flip depends
   on, since `regression-gate`/`convergence-tracker` read the four mechanism-A artifacts), this
   migration has NO Cohort-B sequencing dependency: the companion F2 architecture-delta doc's §5
   migration-impact map already confirms `regression-gate`/`convergence-tracker` do not read
   `BC-INDEX.md`. `BC-7.08.001`'s scope and gating conditions are UNCHANGED by this Decision.
8. **Relationships:** depends on BC-1.18.010 (the end-state addressing scheme this migration
   produces) and BC-1.18.006 (reuses its atomic-write primitives) — the same "applies an existing
   primitive retroactively, once" relationship BC-1.18.008 has to BC-1.18.006, mirrored here for
   B2's own end-state BC.

**Why this was missing in v1.0, corrected now:** BC-1.18.010 (v1.0) asserted the END-STATE
(addressing scheme, manifest schema, migration surface for READERS/writers) but never specified
the TRANSITION mechanics or their integrity checks — exactly the same gap BC-1.18.008 was written
to close for mechanism A, but B2's equivalent gap was not independently recognized during the
original F2 burst. This Decision closes it using the identical migration-governance pattern already
proven sound for mechanism A, rather than inventing new migration machinery.

---

## Rationale

**Why native, not WASM, for the shard-cap gate:** the WASM sandbox exists to bound the blast
radius of untrusted or fallible validator logic; the shard-cap check is neither — it is a single
`stat()` call and arithmetic comparison, dispatcher-trusted code that already has unrestricted
filesystem access in its own execution context (the dispatcher process itself). Routing it through
WASM would add fuel-budget risk (exactly the failure class Layer 2 exists to eliminate) for zero
security or sandboxing benefit. `block_if_marker_check`'s existing native-check precedent in
`executor.rs` establishes this pattern is already accepted in this codebase.

**Why block-and-retry over silent redirection:** `HookResult`'s three-variant contract
(`Continue`/`Block`/`Error`) is a hard SDK constraint, not a design preference this ADR could
relax without an SDK/ABI change. A `Redirect` variant was considered and rejected — see
Alternatives Considered.

**Why reuse `rotate_changelog` for B1 instead of new logic:** ADR-049 already built, tested, and
shipped a correct changelog-rotation primitive for exactly this shape of problem
(`crates/last-amended-migrate/src/rotate.rs`). Writing parallel logic in `shard_manager.rs` would
violate DRY and create two divergent changelog-rotation implementations for the same document
family (`ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md` all carry the same `changelog:` shape per
ADR-049 audit finding 3) with no benefit.

**Why B1's gate performs ONLY the trim, never the prepend (fix-burst amendment, F-S2502-F2-001):**
`rotate_changelog`'s own signature is a pure trim (keep N-1, archive the rest) — it has no
parameter for inserting a new item and never calls `prepend_changelog_item`. Fusing "trim" and
"insert the new item" into one gate-side action (the withdrawn v1.0 design) required the gate to
duplicate the AGENT's own already-planned write, which is both redundant (two writers, one logical
action) and unsafe for `Write`/`MultiEdit` (a stale full-file payload can silently re-introduce
just-archived content). Restricting the gate to ONLY the trim, and requiring the actual new-item
write to come from the tool call that ultimately lands (original or retried), makes B1 a strict
structural mirror of BC-1.18.006's already-accepted block-and-retry contract — one actor per
write, no exception carved out for B1.

**Why the per-subsystem BC-INDEX partition is not a new invention:** the `### SS-NN` sections and
the `BC-S Prefix` -> `SS-NN` mapping already exist and are already the authoritative addressing
scheme for every other BC-related lookup in the pipeline (ARCH-INDEX §Subsystem Registry, POLICY
6). B2 shards ALONG an existing seam rather than choosing an arbitrary new one, which is why this
ADR assesses B2 as tractable within S-25.02 rather than warranting a split to a follow-up story.

## Consequences

### Positive

1. Roll-before-write is achievable within the dispatcher's actual `HookResult` contract — no SDK
   or hook-sdk ABI change, no `HOST_ABI_VERSION` bump (the gate is native code, not a new WASM
   host function).
2. `/compact-state` requires zero mechanism-level amendment (Decision 5) — shard-awareness is
   free for every current and future caller of `Edit`/`Write`/`MultiEdit` against the four
   append-log artifacts.
3. Shard-UNAWARE validators and humans continue to work against append-log artifacts with ZERO
   code change for the common "care about latest state" case (Decision 3's stable-current-name
   trick), directly satisfying AC-007.
4. BC-INDEX's B1 sub-mechanism reuses an already-shipped, already-tested primitive
   (`rotate_changelog`) rather than duplicating logic.
5. BC-INDEX's B2 sub-mechanism reuses an already-authoritative addressing scheme (BC-S-prefix ->
   SS-NN), keeping the two-level split's first level lookup-free.
6. The formula's constants are grounded in real measured data (ADR-042's 53.18 fuel/byte, direct
   `wc`/`awk` byte measurements of the live artifacts) rather than pure theoretical estimates,
   consistent with the story's own explicit prohibition on hardcoding from theory alone.
7. TD-VSDD-053 single-commit-per-burst holds structurally for shard+index atomicity (Decision 4),
   not merely by state-manager convention.
8. **(fix-burst)** B1's corrected single-actor contract (Decision 7) means BC-1.18.009's block
   message and BC-1.18.006's block message now share IDENTICAL actor-ownership semantics — one
   less special case for implementer and formal-verifier to reason about, and one fewer proof
   obligation shape (no "when does Continue-after-mutation not clobber" side-condition to verify).
9. **(fix-burst)** B2's first-level split now has a governed migration path (Decision 10) with the
   same content-preservation/atomicity/rollback guarantees mechanism A's backfill already has —
   closing the one place B2 previously relied on an unstated, unverified "the split happens
   correctly" assumption for a POLICY-7 title-authority-critical file.

### Negative / Trade-offs

1. Block-and-retry surfaces one additional, occasionally-confusing interaction to agents mid-
   append (a `Block` where the agent expected success) — mitigated by an explicit, actionable
   message, but still a UX cost relative to a hypothetical silent-redirect mechanism the SDK does
   not support.
2. BC-INDEX's B2 mechanism imposes a real, non-trivial reader/writer migration cost (Decision 8)
   across product-owner's authorship workflow, state-manager's count-sync bursts, and
   consistency-validator's cross-reference checks — asymmetric with mechanism A's near-zero-cost
   migration. This is a genuine, acknowledged trade-off of the WIDEST-SCOPE decision (D-1166), not
   hidden or minimized.
3. Every numeric calibration constant in Decision 2 is provisional pending the F4 harness; if the
   harness reveals a materially smaller cap than the provisional 48 KiB estimate (e.g., if
   `regression-gate`'s cross-artifact logic proves superlinear rather than linear), the one-time
   backfill-split (Decision 2) would need to produce MORE shards per artifact than currently
   estimated. This is bounded risk (the formula shape is sound regardless of the constants), not
   open-ended risk.
4. Two of ten BC-INDEX subsystems (SS-05, SS-06) require second-level sub-sharding on day one of
   F4 activation, adding one extra manifest-read hop for any reader of those two subsystems
   specifically (the other eight need only the top-level, lookup-free hop).
5. **(fix-burst)** B1's corrected contract requires a second tool-call round-trip (block, then
   retry) for the FIRST prepend that crosses the N-item boundary, exactly mirroring mechanism A's
   existing UX cost (Negative item 1) — previously (v1.0, withdrawn) this case was designed to look
   like a zero-extra-round-trip `Continue`, which was cheaper in appearance but unsound. This is
   the same UX-cost/correctness trade-off already accepted for mechanism A, now applied
   consistently to B1 rather than carved out.
6. **(fix-burst)** POLICY-1's append-only/no-reuse enforcement (Decision 6 amendment) requires the
   `consistency-validator`/adversary-prompt agent-level audits to scan `archive/` in addition to
   active shards — a small but real widening of what "whole-corpus" means for THIS specific policy,
   diverging from every other whole-corpus validator's default active-shards-only scope (Decision 6
   general rule). This asymmetry is intentional and documented, not an oversight.

### Status as of 2026-09-05 (v1.0); fix-burst amendment 2026-09-05 (v1.1)

Proposed. Not yet human-ratified (POLICY 22). Frontmatter `status: proposed` per this project's
`create-adr` convention (never `accepted` at authoring time). The Decision 2 calibration
constants are explicitly provisional and are NOT to be treated as final until the F4 synthetic
harness runs. D-1166 (OQ-1 scope width) is the only sub-question of this ADR's scope already
human-ratified; OQ-2 through OQ-5 are architect-resolved design decisions pending the same F2
human gate this ADR's dispatch instructions describe.

**v1.1 (this fix burst) resolves a fresh-context adversarial review's findings against v1.0,
routed to the architect:** F-S2502-F2-001 (BLOCKER — B1's gate-vs-agent double-actor prepend
hazard, corrected to a single-actor block-and-retry contract, Decision 7); F-S2502-F2-002 (HIGH —
missing governed one-time migration for the B2 BC-INDEX body split, added as Decision 10);
F-S2502-F2-005 (MEDIUM — item-count trigger ownership, clarified as a BC-1.18.005-owned
trigger-shape dispatch, Decision 1 amendment); F-S2502-F2-008 (MEDIUM — whole-corpus
history-scanning validator enumeration and POLICY-1/archival reconciliation, Decision 6
amendment); and a cosmetic sort-order rationale correction (Decision 3). Status remains PROPOSED —
none of these are POLICY 22 design-direction reversals; all correct or complete v1.0's own stated
design intent. Downstream: product-owner rewrites BC-1.18.005 (add item-count trigger-shape
postcondition), BC-1.18.009 (rewrite Postconditions 2/6 to the single-actor contract), and authors
the new B2 migration BC (illustratively numbered BC-1.18.011); formal-verifier updates/adds VP
coverage for the corrected B1 contract and the new migration BC.

## Alternatives Considered

- **Option: extend `HookResult` with a `Redirect { new_path }` variant so PreToolUse could
  transparently retarget the write.** Rejected: this is a hook-sdk ABI change requiring a
  `HOST_ABI_VERSION` bump and a coordinated update to every plugin author's mental model of what a
  PreToolUse hook can do — a much larger blast radius than Layer 2's own problem justifies, for a
  capability (silent redirection) that arguably makes tool-call semantics LESS predictable for
  agents (a `Write` to path A silently landing at path B is a bigger surprise than an explicit
  block-and-retry message).
- **Option: WASM-plugin-based shard-cap check (the story draft's literal AC-002 option (a)).**
  Rejected per Decision 1's rationale — reintroduces exactly the fuel-budget risk Layer 2 exists
  to eliminate, for a check that is cheap, deterministic, dispatcher-trusted logic with no
  sandboxing benefit.
- **Option: split BC-INDEX sharding into a follow-up story (S-25.05 or later), keeping S-25.02
  scoped to only the four append-logs.** Considered and rejected for THIS ADR's scope, per D-1166
  (human already selected widest scope) and per this ADR's own finding that B2 shards along an
  existing, already-authoritative seam (BC-S-prefix -> SS-NN) rather than requiring new
  architecture from scratch. See the companion F2 delta doc's "Split Proposal Assessment" section
  for the full reasoning trail, including what WOULD have justified a split if the seam had not
  already existed.
- **Option: extend production observation window 2-4 weeks instead of a synthetic harness (OQ-4
  alternative).** Rejected per F1 §2/§6's own recommendation, adopted unchanged here: a harness is
  faster, can construct adversarial worst-case inputs directly, and does not depend on production
  happening to exercise the worst case within the observation window.

## Source / Origin

- **ADR-047** (`decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md`)
  §Decision 8b — the ratified future phase this ADR elaborates; §8a — the Cohort B partition and
  the plugin-name correction this ADR co-amends.
- **ADR-039** — `failure_policy` field schema and calibration prerequisites (§Decision 3) that
  Decision 9's fail-closed flip depends on.
- **ADR-042** (`decisions/ADR-042-validate-cross-site-correspondence-fuel-budget-raise-and-loud-exhaustion-signaling.md`)
  — measured fuel-per-byte linear model (`fuel = 2,585,970 + 53.18 × payload_bytes`) grounding
  Decision 2's provisional `WORST_CASE_FUEL_PER_BYTE`; `DEFAULT_FUEL_CAP` raise to 20M (not yet
  operator-level-effective).
- **ADR-049** (`decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md`)
  §Decision 6 — the manual `rotate_changelog` primitive Decision 7 (B1) reuses; §Decision 2 — the
  `last_amended`/`changelog:` prepend discipline B1 must respect.
- **F1 Delta Analysis:** `.factory/cycles/v1.0-brownfield-backfill/S-25.02-f1-delta-analysis.md`
  — the five Open Questions this ADR resolves, the 708-event production forensics, and the
  scope-confirmation finding that led to D-1166.
- **Code as-built:** `crates/hook-sdk/src/result.rs` (`HookResult` enum — the PreToolUse contract
  constraint motivating Decision 1); `crates/factory-dispatcher/src/indeterminate_marker.rs`
  (`write_indeterminate_marker`, `block_if_marker_check` — atomic-write and native-check
  precedents); `crates/factory-dispatcher/src/executor.rs` (`plugin_block_if_marker` — native
  check call-site precedent); `crates/last-amended-migrate/src/rotate.rs` (`rotate_changelog`,
  `rewrite_source_after_rotation` — confirmed by direct inspection to be a pure trim with no
  `prepend_changelog_item` call, grounding Decision 7's fix-burst single-actor correction) and
  `src/changelog.rs` (`prepend_changelog_item`, a separate function) — reused by Decision 7;
  `plugins/vsdd-factory/hooks-registry.toml` (`validate-burst-log`, `regression-gate`,
  `convergence-tracker` entries — read directly to confirm plugin names/tool patterns for the OQ-5
  correction and Decision 9).
- **Direct measurements performed for this ADR (2026-09-05):** `wc -c`/`awk` against
  `.factory/cycles/v1.0-brownfield-backfill/{decision-log,burst-log,lessons,session-checkpoints}.md`
  and `.factory/specs/behavioral-contracts/BC-INDEX.md` (whole-file and per-`### SS-NN`-section
  byte counts, longest-single-line byte counts) — grounding every provisional constant in Decision
  2 and the B2 sub-sharding day-one finding in Decision 7.
- **Fix-burst (v1.1) code inspection (2026-09-05), grounding Decision 6's amendment:**
  `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` (`check_d_chain_currency`,
  `scan_max_d_nnn`, `scan_max_decision_log_id` — confirmed STATE.md-content-scoped, not a
  `decision-log.md` reader); `crates/hook-plugins/validate-state-structure/src/lib.rs`
  (`check_decisions_log_monotonicity` — same confirmation); `crates/hook-plugins/
  validate-closes-completeness/src/lib.rs` (`is_decision_log_target` — confirmed current-shard-only
  by design); `crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs`
  (`is_volatile_path` — confirmed filename-exact-match, unaffected by rotation); `.factory/
  policies.yaml` (POLICY id 1, `append_only_numbering`, `enforced_by: [adversary-prompt,
  consistency-validator]`, `lint_hook: null` — the actual current enforcement mechanism the
  archive-inclusive-mode obligation attaches to).

## Changelog

| Version | Date | Author | Summary |
|---|---|---|---|
| 1.0 | 2026-09-05 | architect | Initial authoring. Layer-2 two-mechanism design (append-log rotation + BC-INDEX structured-catalog sharding) per D-1166 widest-scope human decision. Resolves OQ-2 (stable-current-filename addressing + BC-ID-prefix deterministic addressing), OQ-3 (`/compact-state` gets shard-awareness for free via the native dispatcher-mediated gate), OQ-4 (synthetic calibration harness adopted; provisional constants derived from ADR-042's measured fuel/byte model and direct byte measurements of the live artifacts), OQ-5 (co-amended into ADR-047 in the same burst). Identifies and resolves a structural gap the story draft did not address: `HookResult`'s Continue/Block/Error-only contract makes transparent write-redirection impossible, requiring a block-and-retry roll mechanism instead of silent rotation. Status: proposed, pending F2 human gate.|
| 1.1 | 2026-09-05 | architect | Fix-burst amendment resolving fresh-context adversary pass-1 findings against v1.0. F-S2502-F2-001 (BLOCKER): Decision 7's B1 sub-mechanism corrected from a double-actor "gate rotates+prepends, then Continue" design (unsound — double-prepend/stale-payload-clobber, the exact hazard BC-1.18.006 forbids) to a single-actor block-and-retry contract structurally identical to mechanism A's (gate performs ONLY the `rotate_changelog` trim, then Blocks with a retry instruction; the agent's own call, original or retried, performs the sole prepend), grounded in direct inspection of `rotate_changelog`'s actual pure-trim signature. F-S2502-F2-002 (HIGH): added Decision 10, a governed one-time migration for the B2 BC-INDEX body split (content-preservation, independent-census, crash-atomicity, rollback, idempotency, covering SS-05/SS-06 second-level sub-splits in the same operation), modeled on BC-1.18.008, with enumerated postcondition obligations for product-owner's new migration BC (illustratively BC-1.18.011). F-S2502-F2-005 (MEDIUM): Decision 1 amended with an explicit trigger-shape dispatch — BC-1.18.005 owns BOTH the byte-size trigger (mechanism A) and the item-count trigger (mechanism B1), with the item-count shape's distinct (bounded-parse, not `stat()`-only) read-cost model documented. F-S2502-F2-008 (MEDIUM): Decision 6 amended with a code-grounded enumeration of every candidate whole-corpus history-scanning validator — `check_d_chain_currency`/`scan_max_d_nnn`/`scan_max_decision_log_id`, `check_decisions_log_monotonicity`, `validate-closes-completeness`'s decision-log arm, `validate-cross-site-correspondence`'s `is_volatile_path`, and Cohort B (`validate-burst-log`/`regression-gate`/`convergence-tracker`) are all verified NOT affected by archival (STATE.md-scoped or correctly current-shard-scoped); POLICY-1 (`append_only_numbering`) enforcement (`consistency-validator`/adversary-prompt, `lint_hook: null`) is identified as the one genuine gap and MUST default to an archive-inclusive whole-corpus scan mode, an explicit carve-out from this Decision's general opt-in-required default. Cosmetic: Decision 3's sort-order rationale corrected (the operative comparison is digit-vs-`m` one byte past the shared `decision-log` prefix, not `.` vs. digit; conclusion unchanged). Status remains PROPOSED — none of these are POLICY 22 reversals.|
