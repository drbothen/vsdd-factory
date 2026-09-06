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
3. **Compute `projected_size` PER-TOOL-SEMANTICS (CORRECTED, fix-burst F-P2-002,
   BLOCKER; the v1.1 formula below is WITHDRAWN as unsound):** `Write` REPLACES a file's entire
   content — `len(content)` alone IS the file's post-apply size, never `current_size + len(content)`.
   `Edit`/`MultiEdit` MUTATE existing content in place — `current_size + net_delta_bytes` (the sum
   of each edit's `len(new_string) - len(old_string)`) correctly models their post-apply size. The
   WITHDRAWN v1.1 formula (`current_size + payload_size` for every tool, including `Write`)
   double-counted a `Write`'s own already-complete content on top of the current shard's size,
   over-triggering rotation on ordinary same-size-or-shrinking full-file `Write` calls. The
   corrected, tool-discriminated formula:
   - `Write`: `projected_size = len(content)`.
   - `Edit`/`MultiEdit`: `projected_size = current_size + net_delta_bytes` (unchanged from v1.0/
     v1.1 — this leg was never wrong; only the `Write` leg was).
4. If `projected_size <= shard_cap_bytes`: `Continue`.
5. If `projected_size > shard_cap_bytes`: **perform the roll** (Decision 3) — seal the current
   shard's content (copy to its sealed name, then atomically empty the canonical file in place —
   see Decision 3's fix-burst correction; the current shard is NEVER renamed away), atomically
   publish the updated shard index (Decision 4/Decision 11) — THEN return `HookResult::Block`
   with the CORRECTED retry-instruction wording (Decision 3's fix-burst correction below; the
   v1.1 "if you used Write, simply retry unchanged" wording is WITHDRAWN as unsound — see F-P2-002
   in the companion F2 architecture-delta doc §4b).

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
(actively-written) shard. Sealing a shard publishes its content under a NEW sealed filename
`<stem>.<seq:04>.md` (e.g. `decision-log.0001.md`) and leaves the canonical file in place,
**atomically emptied**. This is the addressing resolution for OQ-2's option (a) — achieved by
naming convention, not a symlink (a symlink would need to be re-pointed atomically alongside the
seal step and adds a platform-portability concern the `factory-dispatcher` cross-compilation
targets (darwin/linux/windows) would need to separately verify).

**CORRECTED (fix-burst F-P2-003, HIGH) — seal is COPY-then-ATOMIC-TRUNCATE-IN-PLACE,
NEVER a rename-away of the canonical path.** The v1.0/v1.1 text above ("renames it away") and
BC-1.18.006 Postcondition 1(a) as originally drafted described the seal step as `rename(canonical,
sealed)` followed by a separate `create(canonical)` — two distinct filesystem operations with an
interstitial window, between the rename completing and the fresh-file create completing, during
which the canonical path **does not exist on disk at all**. Any shard-unaware reader (the ~76
fail-open production plugins with directory-scoped `path_allow` globs, `check_d_chain_currency`,
a human `cat`) that happens to `open()` the canonical path inside that window observes `ENOENT` —
a hard failure, not a stale-but-valid read — directly contradicting AC-007's "zero-code-change
transparency" guarantee and BC-1.18.006 Invariant 3's own text ("the canonical filename is NEVER
renamed away; only its CONTENT is replaced"), which the v1.0/v1.1 Postcondition 1(a) rename-based
mechanism structurally could not satisfy. The corrected seal sequence, reusing ONLY the
already-established `write_atomic` (`crates/last-amended-migrate/src/atomic_write.rs`) /
`write_indeterminate_marker` (`crates/factory-dispatcher/src/indeterminate_marker.rs`)
temp-file-then-rename primitive — no new atomic-write primitive, no reimplementation:

1. **Read** the canonical file's current full content (a one-time, roll-only read — the cheap
   per-write TRIGGER check, BC-1.18.005 Postcondition 2, remains `stat()`-only; content is read
   ONLY once a roll is already confirmed necessary).
2. **Publish the sealed shard as a brand-NEW file** at `<stem>.<seq:04>.md` via `write_atomic`
   (temp-file-then-rename; the destination does not yet exist, so this is a `rename()` that
   CREATES an entry, not one that could interrupt any reader of the canonical path — sealed
   filenames are never read by shard-unaware code, only by whole-corpus glob consumers).
3. **Atomically REPLACE the canonical file's content with empty**, via the SAME temp-file-then-
   rename primitive — write an empty temp file, then `rename(temp, canonical)`. A `rename()` onto
   an EXISTING destination path is an atomic in-place replace at the directory-entry level (POSIX
   `rename(2)`; the dispatcher's Windows target uses the equivalent `MoveFileEx` with
   `MOVEFILE_REPLACE_EXISTING`, already required by `write_atomic`'s existing cross-platform
   contract) — the canonical path resolves to SOME valid file (old content, then instantaneously
   the new empty content) at every observable instant; it is NEVER absent. This is the crucial
   difference from the withdrawn rename-away mechanism: step 3 renames a temp file **INTO** the
   canonical path (always-occupied), never the canonical path itself **OUT** (would-be-vacated).
4. Publish the updated shard index (Decision 4/Decision 11).

This produces the IDENTICAL on-disk end-state and IDENTICAL sealed-shard naming/glob-sort
properties the v1.0/v1.1 text already established (the sort-order rationale below is unchanged —
only the MECHANISM by which the canonical file ends up empty and the sealed file ends up populated
changes, from "rename away + create" to "copy-out + atomic-replace-in-place"), while making
BC-1.18.006 Invariant 3's "canonical filename never renamed away" claim literally, structurally
true rather than contradicted by its own Postcondition 1(a). See the companion F2
architecture-delta doc §4b (F-P2-003) for the exact BC-1.18.006 Postcondition 1(a)/Invariant
2/Invariant 3 rewrite this requires of product-owner.

**Corrected retry-instruction wording (supersedes the v1.1 "if you used Write, simply retry
unchanged" text, which is UNSOUND under BOTH the withdrawn rename mechanism and the corrected
per-tool formula above — F-P2-002):** because the canonical file is now EMPTY after a
roll (copy+truncate, not rename-away), and because a blocked `Write`'s own `content` parameter was
composed by the agent BEFORE the roll (typically by reading the OLD, over-cap file and appending
one new entry — the same "stale full-file payload" pattern already named unsound for mechanism
B1's `Write` case, Decision 7), retrying that SAME `content` unchanged would resubmit content that
is STILL over cap relative to the fresh empty shard (since `projected_size = len(content)` for
`Write`, per the corrected formula, and `len(content)` has not shrunk) — producing a permanent
block/retry deadlock, not a duplicate. The corrected, UNIFIED retry instruction (same text
regardless of original tool, since both branches now converge on "recompute against the current,
post-roll state"): `"Shard <artifact> rotated (cap <N> bytes reached); the current shard is now
empty. Retry your write against the CURRENT (post-roll, empty) file — do not resubmit your
original payload unchanged: if you used Edit or MultiEdit, your old_string will no longer match
(the content it targeted is now in <sealed-path>) — reissue as a fresh Write containing ONLY your
new entry; if you used Write, recompute content to contain ONLY your new entry (not your original
full pre-roll payload, which reflects discarded state and will exceed the cap again if
resubmitted)."` This closes the mechanism-A analogue of the exact hazard Decision 7's fix-burst
correction already closed for mechanism B1's `Write` path — mechanism A's own `Write` path carried
the identical unfixed hazard through v1.1, per F-P2-002's finding.

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
the gate performs the sealed-shard publish, the canonical atomic-truncate (Decision 3's fix-burst
copy-then-atomic-truncate-in-place correction), and the index publish before returning `Block`, and
because all three are filesystem writes issued by the SAME PreToolUse invocation before any
`git add`/`git commit` occurs, they are guaranteed to land in the SAME subsequent factory-artifacts
commit — satisfying AC-004/TD-VSDD-053 by construction, not by state-manager discipline (this
directly resolves F1 §4's "TD-VSDD-053 vs. shard+index atomicity" MEDIUM risk). **A crash between
these three writes is a genuinely distinct, previously-unspecified failure surface — see Decision
11's fix-burst addition for the staged execution order and the partial-failure postconditions/error
codes this composite operation requires (F-P2-004, MEDIUM).**

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

**Why this ADR's `subsystems_affected` includes SS-04 (Plugin Ecosystem) (S-25.02 F2 gate-audit
finding F3, MEDIUM, architect-adjudicated 2026-09-06):** this Decision's validator enumeration
below directly audits four SS-04-owned WASM crates under `crates/hook-plugins/` —
`validate-dispatch-advance`, `validate-state-structure`, `validate-closes-completeness`, and
`validate-cross-site-correspondence` — for correctness under the new archival/retention scheme,
and imposes a new archive-inclusive-glob obligation on POLICY-1's SS-04-adjacent enforcement path.
This is the ADR's sole substantive SS-04 touchpoint (Decisions 1–5 and 7–14 are exclusively
SS-01/SS-07 concerns); CAP-043's §Subsystems list (business-analyst/product-owner domain) was
correspondingly extended to include SS-04, closing the drift this Decision originally flagged as a
product-owner follow-up in the companion S-25.02 F2 gate-audit report.

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
tool) to move the overflowing tail into a sealed changelog archive — THEN returns
`HookResult::Block` with a retry instruction, per the CORRECTED single-actor contract below. This
automates, for BC-INDEX specifically, exactly what ADR-049 could previously only do via manual CLI
invocation — no new rotation logic is designed, only a new automatic size-triggered CALLER of the
existing `rotate_changelog` function.

**CORRECTED (fix-burst F-P2-001, HIGH) — the archive is a SINGLE, ever-growing
append-file at a BC-INDEX-appropriate path, NEVER per-`seq` sealed shards under a
`BC-INDEX-changelog-shards/` directory; reusing `rotate_changelog` requires a small, NAMED,
bounded extension to its path-resolution surface, not the unqualified "zero new logic" claim the
v1.0/v1.1 text above made.** Direct inspection of the SHIPPED implementation
(`crates/last-amended-migrate/src/rotate.rs`) shows `rotate_changelog`/`resolve_archive_path`:

- derive exactly ONE fixed destination path per invocation —
  `<factory-root>/cycles/<cycle_name>/<basename>-changelog-archive.md` — with NO `<seq:04>`
  per-rotation numbering scheme anywhere in the function;
- **APPEND** to that single destination on every invocation (`archive_content.push_str(item)` for
  each moved item, after first reading any pre-existing archive content at that same path) — the
  function is already, by construction, a single-evergreen-file archiver, not a shard-per-rotation
  archiver;
- **REQUIRE** a `cycle_name: &str` parameter used only to construct the
  `cycles/<cycle_name>/` path segment — `BC-INDEX.md` is a `.factory/specs/behavioral-contracts/`
  catalog artifact, not a cycle artifact, and has no natural `cycle_name` value to supply; forcing
  a synthetic/sentinel `cycle_name` string would misfile BC-INDEX's changelog archive under
  `.factory/cycles/`, a directory whose semantic meaning (and whose `path_allow`-scoped validators)
  is "this cycle's artifacts," not "catalog metadata archives."

The v1.0/v1.1 text's claim that B1 "reuses `rotate_changelog`, no reimplementation" while
separately specifying a per-`seq` sealed-shard-directory archive layout
(`BC-INDEX-changelog-shards/BC-INDEX-changelog.<seq:04>.md`) is **internally impossible**: the
shipped function cannot produce that layout under any call pattern. Two remediation options were
weighed (per this fix-burst's own dispatch instructions): (a) accept `rotate_changelog`'s ACTUAL
single-append-file behavior as-is, forcing a sentinel `cycle_name`; or (b) make a small, explicit,
NAMED extension to the primitive's path-resolution surface so a non-cycle caller can supply its own
archive path directly. **Option (b) is adopted** — it is the sounder engineering choice (it avoids
semantically misfiling a specs-catalog artifact under `.factory/cycles/`) and remains a genuinely
bounded extension, not new rotation logic:

- **New, additive function `resolve_archive_path_at(archive_path: &Path) -> PathBuf`** (or,
  equivalently, generalize `rotate_changelog`'s existing internals to accept an `archive_path: &Path`
  parameter DIRECTLY in place of deriving one from `cycle_name`) — this changes ONLY where the
  archive destination path comes from; every other line of `rotate_changelog`'s logic (frontmatter
  parsing, `keep_recent` split, `archive_content.push_str` accumulation, `yaml_guard` validation,
  `write_atomic` for both files) is REUSED VERBATIM, unmodified. Existing mechanism-A-style callers
  (which ARE genuinely cycle-scoped) are UNAFFECTED: they continue to compute their archive path via
  the EXISTING, unchanged `resolve_archive_path(path, cycle_name)` helper and pass the result
  through the same call surface.
- **The dispatcher's B1 handler in `shard_manager.rs` pre-computes a FIXED, non-cycle,
  BC-INDEX-sibling archive path** — `.factory/specs/behavioral-contracts/BC-INDEX-changelog-archive.md`
  (a single evergreen file, sibling to `BC-INDEX.md` itself, matching `rotate_changelog`'s actual
  single-append-file behavior exactly) — and calls the generalized primitive with that path. NO
  `cycle_name` value is invented or threaded through for this call at all.
- **Accepted trade-off, documented not hidden:** `BC-INDEX-changelog-archive.md` is itself
  APPEND-ONLY and UNBOUNDED across the artifact's lifetime (every rotation appends more, never
  splits into fresh files) — this is the SAME shape ADR-049's original manual tool already accepted
  as sufficient, and is sound here because (a) the archive is small per-append (individual
  `changelog:` items, not whole BC rows), (b) it is read by NEITHER the item-count trigger (which
  inspects only the LIVE frontmatter sequence) NOR any Cohort B validator (confirmed per the
  companion F2 delta doc §5 migration-impact map), so it sits entirely outside Layer 2's own
  bounded-artifact concern, and (c) if it later becomes large enough to be a NEW forensic
  contributor in its own right, that is a follow-up Layer-2-on-Layer-2 story, not a defect of this
  design. This is a deliberate, minimal-footprint choice consistent with this finding's own framing
  ("the changelog array is small metadata, so a single append-archive may be entirely adequate").

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
   `keep_items` (the retained `keep_recent` most-recent items — for B1, the configured
   `low_water_mark`, default `floor(N/2)` per Decision 14; NEVER a fixed `N-1`) and writes a
   `changelog_archive:` discoverability
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
3. If true: the gate invokes `rotate_changelog` (via the generalized, explicit-`archive_path`
   surface, Decision 7's fix-burst correction above) to trim the live `changelog:` sequence down to
   `low_water_mark` items (Decision 14's high-water/low-water hysteresis target — default
   `floor(N/2)`; NEVER a fixed `N-1`), appending the overflow tail to the SINGLE evergreen archive
   file, THEN returns `HookResult::Block` with an explicit retry instruction: "BC-INDEX.md's
   `changelog:` sequence was rotated to make room (oldest item(s) appended to
   `.factory/specs/behavioral-contracts/BC-INDEX-changelog-archive.md`); the frontmatter now has
   `low_water_mark` items. Retry your write: if you
   used `Edit`, reissue as a fresh `Write` or a fresh `Edit` re-read against the current
   (post-rotation) file, since your original `old_string`/`new_string` pair may no longer match; if
   you used `Write`, recompute your `content` payload against the current (post-rotation) file
   before retrying — do not resubmit your original payload unchanged, since it reflects
   pre-rotation state." (This is a shape-appropriate specialization of BC-1.18.006 Postcondition
   2's retry-wording contract, not a divergent one — B1's wording differs from mechanism A's only
   because the underlying state changed shape, not filename.)
4. Agent retries, reading/recomputing against the now-rotated file; its retried prepend lands via
   `Continue` (item count is now `low_water_mark + 1`, comfortably below the `N` trigger threshold —
   Decision 14's amortization gain over the withdrawn fixed-`N-1` target, which left the count back
   at exactly `N`) — SINGLE ACTOR, exactly once.

**Atomicity is preserved identically to mechanism A's pattern:** the rotate/trim step (evergreen
archive-file append + live-frontmatter trim) completes BEFORE the `Block` is returned, in the
SAME native-gate invocation — the same "seal-then-block" atomicity guarantee Decision 4/Decision 11
already establishes for mechanism A, just with `rotate_changelog`'s trim in place of mechanism A's
copy-then-atomic-truncate seal. The actual NEW-item prepend, like mechanism A's actual oversized write, lands in
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
   count-oracle — its value AT MIGRATION TIME, whatever the corpus has grown to by then; `total_bcs`
   is an actively-incrementing counter this ADR does not pin to a specific historical figure, per
   the STRUCTURAL-FORM re-grounding convention F-P2-006 already established for BC-1.18.010's own
   count citation — the migration re-reads the live field at execution time rather than trusting any
   number written into this ADR's prose) as a sanity bound. After the split, verify: (a)
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

### Decision 11 — Staged, Crash-Recoverable Per-Write Roll Sequence and Partial-Failure Error Codes (fix-burst addition, F-P2-004, MEDIUM)

BC-1.18.006/BC-1.18.009's ongoing (per-write, NOT one-time-migration) roll is the FAR more frequent
operation in this ADR's scope — it fires on every over-cap `Edit`/`Write`/`MultiEdit`, unlike
BC-1.18.008/BC-1.18.011's one-time backfills, which run exactly once each. Yet, as originally
drafted, the ongoing roll's crash-atomicity was under-specified relative to the one-time
migrations: BC-1.18.008/011 both received explicit staging + independent-verification + atomic
replace + rollback-on-failure treatment; the per-write roll's Postcondition 1 asserted an ordered
sequence of filesystem writes but named only ONE failure mode (`E-SHD-001`, "seal[-write] failure")
and left the OTHER two possible crash points between the three composite writes (Decision 3's
corrected copy-then-atomic-truncate seal step, and Decision 4's index publish) completely
unspecified. This Decision closes that gap with a staged sequence and named partial-failure
postconditions, reusing ONLY already-established atomic-write primitives — no new atomicity
mechanism is invented.

**Staged sequence (mechanism A; mechanism B1 substitutes `rotate_changelog`'s own trim+archive-append
write for steps 1-2, per Decision 7, but composes with steps 3-4 identically for the frontmatter
truncate-to-`low_water_mark`-items (Decision 14; NEVER a fixed `N-1`) and index-adjacent
bookkeeping):**

1. **Read** the canonical file's current full content (roll-only; the cheap per-write trigger check
   remains `stat()`-only, per BC-1.18.005 Postcondition 2).
2. **Publish the sealed shard as a new file** at `<stem>.<seq:04>.md` via `write_atomic` (creates a
   not-yet-existing path).
3. **Atomically replace the canonical file's content with empty** via `write_atomic` (renames a temp
   file ONTO the existing canonical path — Decision 3's fix-burst correction).
4. **Atomically publish the updated shard-index TOML** via `write_atomic` (records the new
   `[[shard]]` entry).
5. Return `HookResult::Block` (Decision 3's corrected retry wording).

**Partial-failure postconditions, one new/refined `E-SHD-NNN` code per crash point (product-owner
adds these rows to `.factory/specs/prd-supplements/error-taxonomy.md` — architect does not edit
that file directly per CLAUDE.md routing; see the companion F2 architecture-delta doc §4b for the
exact obligation):**

- **Step 1-2 fails (`E-SHD-001`, REFINED — description text updated from "seal-rename failure" to
  "shard-seal-write failure" to match the corrected copy-based mechanism; the error CODE and
  observable contract — `HookResult::Error`, canonical file completely untouched — are unchanged,
  so no error-taxonomy renumbering is required, only a description-text refresh):** the canonical
  file is left in its exact pre-roll state (still over cap, still holding its full original
  content) — safe, no data loss, no duplicate; the next dispatch attempt against this artifact
  re-evaluates the trigger and re-attempts the FULL sequence from step 1.
- **Step 3 fails after step 2 succeeded (NEW `E-SHD-006`):** the sealed shard now durably exists
  (a byte-for-byte copy of the pre-roll content) AND the canonical file STILL holds that same
  content too (not yet truncated) — a transient, DETECTABLE duplicate-content state, not a
  data-loss state. **Recovery (self-healing, no operator intervention):** on the NEXT dispatch
  attempt for this artifact, BEFORE evaluating any new trigger, the gate checks whether a sealed
  shard exists at the index's next-expected `seq` path whose content is byte-identical to the
  canonical file's CURRENT content; if so, this is recognized as "seal published, truncate did
  not," and the gate resumes from step 3 alone (re-attempting ONLY the truncate + index publish,
  never re-writing the already-correct sealed shard) — idempotent by construction, since step 2's
  `write_atomic` create is itself a no-op if reissued against identical content.
- **Step 4 fails after step 3 succeeded (NEW `E-SHD-007`):** the canonical file is CORRECTLY fresh
  and empty (safe for all future writes — no over-cap risk, no data loss) and the sealed shard file
  exists correctly on disk, but `<artifact-stem>.shard-index.toml` has not yet recorded the new
  `[[shard]]` entry — a discoverability-METADATA gap only: whole-corpus glob-based readers
  (`<stem>*.md`) still find the sealed file regardless of index membership, so no reader-visible
  data loss occurs. **Recovery (self-healing):** on the next dispatch attempt, the gate reconciles
  the index by scanning the filesystem for sealed-shard files matching the artifact's naming
  convention that are absent from the index, and appends the missing entries before evaluating any
  new trigger.
- **All four steps succeed:** normal `Block` outcome, no error.

This staged model is the mechanism-A/B1 per-write-roll analogue of the staging+verify+atomic-replace
+rollback discipline BC-1.18.008/BC-1.18.011 already apply to their one-time migrations — applied
here to a composite THREE-write operation instead of an N-way partition, with detection-and-resume
substituting for a from-scratch rollback (rollback-to-original-state is not meaningful here, since
unlike the one-time migrations, the "original state" — the over-cap canonical file — is exactly the
state the roll exists to eliminate; resuming forward through the remaining steps is the correct
recovery direction, not reverting).

### Decision 12 — Non-Append-Edit Gate Scope: the Append-Only-Tail Assumption Made Explicit, and the Sealed-Shard Direct-Edit Escape Hatch (fix-burst addition, F-P2-005, MEDIUM)

The gate (Decision 1) matches `Edit`/`Write`/`MultiEdit` against a sharded artifact's CANONICAL
path and computes `projected_size` from a pure byte-delta/length formula (Decision 1 step 3,
fix-burst-corrected) — it has NO semantic understanding of WHERE within the file an edit lands, and
was never designed to. The roll+block+retry wording (Decision 3's corrected text, and
BC-1.18.009's B1 equivalent) is phrased for the common case this gate exists to serve: a pure
APPEND of one new record at the file's end. This Decision makes explicit an assumption the v1.0/
v1.1 text left implicit, and specifies the (narrow, caller-responsibility, not gate-defect) failure
mode when the assumption is violated.

**The four mechanism-A artifacts and BC-INDEX's `changelog:` array are, by construction, POLICY-1
governed append-only records.** POLICY-1 (`append_only_numbering`, `.factory/policies.yaml` id 1)
already forbids renumbering or rewriting historical D-NNN/BC/VP/story entries. Legitimate
`Edit`/`MultiEdit` mutations against these artifacts are therefore, by the SAME policy, already
expected to be one of: (a) a pure append of a brand-new record at file end, or (b) a narrow
amendment to a STILL-MUTABLE, recently-added record near the tail (e.g., a same-burst typo fix to
an entry that has not yet been sealed away) — never an edit to arbitrarily old, already-sealed, or
deep-mid-file historical content, since POLICY-1 already forbids rewriting that content's meaning
regardless of this gate's existence.

**Gate behavior is UNCHANGED and requires no new detection logic — this is a documentation/edge-case
clarification, not a code change to the trigger or roll.** A net-positive `Edit`/`MultiEdit` that
happens to target a still-mutable tail record and pushes `projected_size` over cap triggers the
SAME generic roll+block+retry sequence as any other over-cap write; BC-1.18.006 EC-002 (an `Edit`'s
`old_string` failing to match against the emptied canonical file) already covers the resulting
tool-level failure mode generically. **The genuinely new edge case this Decision names:** if an
`Edit`/`MultiEdit`'s target content was ALREADY relocated to a SEALED shard by an EARLIER roll (a
policy-violating attempt to amend deep-historical content, or a caller operating on stale
in-memory state), the retry-instruction text ("reissue as a fresh Write containing only your new
entry") is INAPPLICABLE — there is no "new entry" to reissue; the caller's actual goal (amending
old content) cannot be satisfied against the canonical file AT ALL, because that content no longer
lives there. **Resolution (an explicit escape hatch, not a workaround):** a sealed shard file
(`<stem>.<seq:04>.md`) is an ORDINARY file that does NOT match any `[[shard]]` config entry's
canonical-path pattern (Postcondition 1's zero-cost bypass for unmatched paths) — it is therefore
entirely UNGATED by Layer 2, and a caller with a genuine, policy-sanctioned need to touch historical
content addresses the sealed file DIRECTLY by its own on-disk filename, exactly as it would edit
any other ordinary file. Layer 2 makes no attempt to detect, permit, or forbid such an edit — that
is POLICY-1's concern (enforced at the `consistency-validator`/adversary-prompt agent level per
Decision 6's amendment), entirely orthogonal to this gate's byte-size-triggered rotation concern.

### Decision 13 — Governed One-Time B1 Changelog Backfill Migration Required at Cold Start (fix-burst addition, F-P2-007, MEDIUM)

Decision 1's trigger-shape dispatch (BC-1.18.005 Postcondition 8, v1.1) characterizes the
item-count trigger's frontmatter-parse read as "bounded... the live sequence is itself capped at N
items by this same mechanism after every prior rotation." **This characterization is TRUE only in
STEADY STATE (after at least one rotation has occurred) and is FALSE at cold start.** Direct
measurement (2026-09-05, this ADR's own Context section) shows `BC-INDEX.md`'s `changelog:`
sequence, which has NEVER been rotated, holds approximately 1,997 items across 177,305 bytes of
frontmatter — the FIRST `Edit`/`Write`/`MultiEdit` against `BC-INDEX.md` after Layer 2 activates
would need to (a) parse and count roughly 1,997 items to evaluate the trigger (an unbounded-relative-
to-N read, though still a finite, single-file read — this is a mischaracterization to correct, not
a fuel-budget hazard, since the check is native code with no fuel budget), and (b), if the trigger
fires, invoke a SINGLE `rotate_changelog` call moving approximately 1,972 items (down to a
`keep_recent = low_water_mark ≈ 25`, **CORRECTED, fix-burst pass-3, F-P3-005 — NEVER
`keep_recent = N`; see Decision 14 below**) into the archive in one operation.
**CORRECTED (fix-burst pass-4, F-P4-004, LOW):** the archived-item count is `1,972`, not `1,947` —
`1,997 (pre-migration count) - 25 (low_water_mark) = 1,972`; the withdrawn `1,947` figure was the
arithmetic remainder against the WITHDRAWN `keep_recent = 50` (`N`, not `low_water_mark`) target
(`1,997 - 50 = 1,947`), left uncorrected when Decision 14 (below) retargeted `keep_recent` to
`low_water_mark ≈ 25`. `1,972` matches BC-1.18.012 v1.1's own EC-001 and Canonical Test Vector
("25 (retained) + 1972 (archived) == 1997 (pre-migration)").

**This is the B1 analogue of the exact gap BC-1.18.008 (mechanism A) and BC-1.18.011 (mechanism B2)
were each independently created to close, and B1 must not be the one mechanism left to a
lazy/ungoverned first-write trigger.** Unlike mechanism A's/B2's monolithic files, B1's cold-start
excess (≈1,972 items) does not risk data LOSS on its own — `rotate_changelog` already validates via
`yaml_guard` and writes both files via `write_atomic` — but performing a ~1,972-item one-time
displacement as an incidental SIDE EFFECT of whichever ordinary agent write happens to be first
after F4 activation has two production-grade deficiencies relative to BC-1.18.008/011's governed
pattern: (1) it imposes an unpredictable, undocumented latency/behavior surprise on an arbitrary
future caller instead of being an explicit, planned, operator-visible activation step; and (2) it
receives NONE of BC-1.18.008/011's INDEPENDENT-CENSUS verification (a fresh, oracle-cross-checked
count confirming every item is preserved in exactly one location) — it relies solely on
`rotate_changelog`'s own internal correctness, with no external check that the split was lossless,
for the single largest content-volume migration this entire ADR specifies.

**Resolution: a NEW governed one-time migration BC is required, modeled directly on BC-1.18.008's
structure (Preconditions/Postconditions/Invariants/Edge Cases/Canonical Test Vectors/Verification
Properties), applied to BC-INDEX's `changelog:` array instead of a monolithic append-log file.
Illustrative numbering: BC-1.18.012 (the next free SS-01 slot after BC-1.18.011 at time of
authoring — product-owner confirms the exact free slot against BC-INDEX at authoring time, per this
ADR's own numbering convention).** Postcondition obligations this BC MUST encode (architect's
authorship input to product-owner, enumerated in full in the companion F2 architecture-delta doc
§4b):

1. Executes exactly once, at F4 activation, BEFORE the ongoing per-write B1 gate (BC-1.18.009) is
   treated as steady-state-bounded — this BC's successful completion is what MAKES BC-1.18.005
   Postcondition 8's "bounded read" characterization true; it is false as a description of the
   COLD state, which this BC exists to eliminate.
2. Uses the SAME `rotate_changelog` primitive (via the Decision 7 fix-burst's generalized
   `archive_path`-parameterized call surface) — **CORRECTED (fix-burst pass-3, F-P3-005, MEDIUM):
   `keep_recent = low_water_mark`, NEVER `keep_recent = N`.** The v1.2 text's `keep_recent = N`
   target was withdrawn because it would leave the live sequence at EXACTLY the trigger boundary
   immediately after this one-time migration completes — the very FIRST ordinary write against
   `BC-INDEX.md`'s frontmatter after go-live would then re-evaluate `current_item_count + 1 > N`
   against a count already AT `N` and immediately re-trigger rotation, reproducing from write #1 of
   the new steady state the exact per-write retrigger pathology Decision 14 exists to eliminate.
   Migrating to `low_water_mark` (the SAME companion config value Decision 14 introduces for the
   ongoing per-write gate, BC-1.18.005 Postcondition 8) instead of `N` establishes the steady state
   at the correct hysteresis floor from the FIRST post-migration write, not merely from the SECOND
   rotation onward — no new rotation logic, only a governed ONE-TIME CALLER with pre/post
   verification wrapped around it, mirroring BC-1.18.008's exact relationship to BC-1.18.006's
   primitives.
3. **Independent-census integrity check:** capture the exact pre-migration `changelog:` item count
   (a fresh enumeration, not reused from any cached count) BEFORE invoking `rotate_changelog`;
   after it completes, verify `(items retained in the live frontmatter) + (items appended to the
   archive) == pre-migration count` exactly — this is BC-1.18.008 Postcondition 6(b)'s exact
   analogue, applied to `changelog:` items instead of decision-log rows.
4. **Content-preservation, byte-for-byte:** every migrated item's `date:`/`summary:` text is
   preserved verbatim in the archive (BC-1.18.008 Postcondition 6(a)'s analogue) — `rotate_changelog`
   already guarantees this internally (Description above), but this BC's independent verification
   re-confirms it externally, exactly as BC-1.18.008/011 re-confirm their own respective primitives'
   internal guarantees rather than trusting them un-verified.
5. **Fail-loud on verification failure:** if the independent census does not reconcile, the
   migration aborts and `BC-INDEX.md`'s frontmatter is left in its exact pre-migration state
   (`rotate_changelog`'s own `write_atomic` calls are the last step, not the census check — so an
   aborted migration means the census ran against a DRY-RUN/staged computation before any write,
   OR — if `rotate_changelog` must actually execute to be checked — a restorable pre-migration
   snapshot is retained until the census passes; product-owner selects the exact staging mechanic,
   mirroring whichever of BC-1.18.008/011's two staging patterns fits `rotate_changelog`'s actual
   write ordering, per architect's `rotate.rs` grounding above). A new error-taxonomy row
   (`E-SHD-003`'s existing wording, "backfill-split content-preservation verification failed for
   `<artifact>`," is already artifact-generic and MAY be reused for this BC rather than allocating a
   new code — product-owner confirms).
6. **Idempotency:** re-running against an already-migrated (post-first-rotation, steady-state)
   `changelog:` sequence is a safe no-op (the sequence is already `<= N` items, so the trigger
   simply does not fire) — no special-casing needed beyond BC-1.18.009's own EC-002 (under-N
   no-rotation `Continue`).
7. **Corrects BC-1.18.005 Postcondition 8's "bounded" claim:** Postcondition 8's read-cost
   characterization MUST be split into two explicit states — cold (pre-this-BC, a single
   one-time oversized-but-finite read, non-fuel-budgeted since native) and steady-state
   (post-this-BC, genuinely bounded at `<= N` items per read, by construction) — never a single
   unqualified "bounded" claim.

**This closes the ONE remaining asymmetry among Layer 2's three structured/append artifact classes:
mechanism A has BC-1.18.008, mechanism B2 has BC-1.18.011, and mechanism B1 now has this Decision's
BC-1.18.012 — no sharded artifact class is left to depend on an ungoverned lazy-first-write
migration for its largest one-time content displacement.**

### Decision 14 — B1 High-Water/Low-Water Hysteresis for the Item-Count Rotation Target (fix-burst addition, F-P3-005, MEDIUM)

**Problem, traced through one full cycle.** BC-1.18.009 v1.2's rotation step trims the live
`changelog:` sequence to `N-1` items (calling `rotate_changelog`'s `keep_recent: usize` parameter
with the value `N-1`), while BC-1.18.005 v1.2's trigger fires at `current_item_count + 1 > N` —
i.e., exactly when the live count reaches `N`. Following one full rotation cycle: rotation fires
when the live count reaches `N`, trims to `N-1`, blocks, and the agent retries — landing its own
prepend and bringing the live count back to exactly `N` again (BC-1.18.009 Postcondition 2 step 4:
`current_item_count = (N-1)+1 = N`). The VERY NEXT `Edit`/`Write`/`MultiEdit` against
`BC-INDEX.md`'s frontmatter therefore evaluates the trigger against a live count already AT `N` —
`N + 1 > N` is true — and rotates again. Because `BC-INDEX.md`'s frontmatter changelog is prepended
on essentially every burst that amends `BC-INDEX.md` (ADR-049 §Decision 2's own prepend discipline,
exercised by every POLICY-7/8-propagating burst), this means **every steady-state write to
`BC-INDEX.md` after the first rotation pays the full block+retry (≈2 dispatch cycles) round-trip
cost** — directly contradicting this ADR's own "zero added latency for ~99% of writes" framing
(Decision 1 step 1's zero-cost bypass covers only the UNMATCHED-path case; it does not help here,
since `BC-INDEX.md` is a matched, and — under 4-index discipline — one of the MOST FREQUENTLY
written artifacts in the whole pipeline). This is a genuine steady-state design defect in the
minimal-eviction (trim-by-exactly-one) policy, not a cosmetic one.

**Grounded fix: reuse `rotate_changelog`'s ALREADY-EXISTING `keep_recent: usize` parameter as a
genuine, independently-configurable low-water floor, distinct from `N-1`.** Direct re-inspection of
the shipped `pub fn rotate_changelog(path: &Path, cycle_name: &str, keep_recent: usize, mode:
MigrationMode)` (`crates/last-amended-migrate/src/rotate.rs`) confirms `keep_recent` is ALREADY a
free parameter, not hardcoded to `N-1` anywhere in the function: its no-op guard is
`total <= keep_recent` (`rotate_changelog`'s own EC-004), and its retained/moved split
(`let (keep_items, move_items) = doc.changelog_items_raw.split_at(keep_recent)`, operating on the
existing newest-first `changelog_items_raw` ordering) already moves exactly "everything older than
the retained `keep_recent` newest items" to the archive, in order, for ANY `keep_recent` value the
caller supplies. **Introducing hysteresis therefore requires ZERO new logic in `rotate_changelog` or
any change to its ordering/content-preservation guarantees — it is purely a call-site config-value
change** (the gate's B1 handler passes a different, independently-configured number instead of
`N-1`), staying entirely inside Decision 7's "reuse, never reimplement" boundary and
BC-1.18.009 Invariant 1's "no new rotation/trim/validate/write logic" constraint.

**High-water/low-water semantics:**
- **`N` (high-water mark, UNCHANGED — already BC-1.18.005 Postcondition 8's config value)** remains
  the trigger threshold: rotation fires when `current_item_count + 1 > N`, exactly as today. No
  change to the trigger condition itself.
- **`low_water_mark` (NEW config value, sibling to `N` in the same `[[shard]]` config entry, for
  `shape = "frontmatter-changelog-array"` artifacts)** is the rotation TARGET: when the trigger
  fires, the gate calls `rotate_changelog(..., keep_recent = low_water_mark, ...)` — NEVER `N-1`.
  Constraint, fail-loud (mirroring BC-1.18.005 EC-009's "no silent default for a malformed
  shape-config" posture, extended to this new field): `0 <= low_water_mark < N`, validated at
  config-load time; a config declaring `low_water_mark >= N` (the degenerate `== N` boundary
  included) or a negative value is a config error (`HookResult::Error`), never silently clamped or
  defaulted around. **CORRECTED (fix-burst pass-4, F-P4-001, HIGH) — `low_water_mark = N-1` is NOT
  part of this fail-loud set.** `N-1` satisfies `0 <= low_water_mark < N` exactly as any other
  interior value does (`N-1 < N` is true by construction for `N >= 1`); it is a legal, merely
  poorly-amortizing, configuration — see the dedicated correction below this Decision's semantics
  list, which withdraws the v1.3 text's erroneous "including the degenerate `N-1`" framing and
  replaces the closed-off latent-pathology concern with a non-fatal advisory instead of a false
  fail-loud claim.
- **Default when `low_water_mark` is omitted from config:** `floor(N / 2)`. This is a config
  DEFAULT, not a hardcoded Rust constant inside `shard_manager.rs` — consistent with BC-1.18.005
  Invariant 4's "formula inputs are configuration, not embedded constants" discipline, extended to
  this new field.

**CORRECTED (fix-burst pass-4, F-P4-001, HIGH) — ADJUDICATION: `low_water_mark = N-1` is
legal-but-poor, never fail-loud; the "including the degenerate `N-1`" fail-loud framing above (v1.3
text) and BC-1.18.005 EC-011's identical phrasing are WITHDRAWN as a live, mutually-unsatisfiable
contradiction with formal-verifier's own VP-140/VP-125 and with BC-1.18.005's own Canonical Test
Vectors table.**

**The contradiction, as found by fresh-context adversary pass-4.** The declared numeric constraint
is `0 <= low_water_mark < N`. This constraint mathematically ADMITS `N-1` — `N-1 < N` is true for
every `N >= 1`, with no special case. Yet the v1.3 prose immediately above (and BC-1.18.005 EC-011
as originally worded) parenthetically folded `N-1` INTO the fail-loud `>= N` bucket ("including the
degenerate `N-1`"), asserting it produces `HookResult::Error`. Meanwhile THREE independent
downstream artifacts — all authored or reviewed AFTER this ADR's own v1.3 text — already state the
opposite as their own authoritative content: BC-1.18.005's Canonical Test Vectors table's last row
("`low_water_mark=49`, i.e. `N-1`, is a VALID boundary value — `49 < 50` satisfies the
constraint — and does NOT fail-loud"); VP-140 §Property Statement point 4's boundary partition
("`low_water_mark == N - 1`: **VALID** — `N - 1 < N` satisfies the constraint; loads normally, no
error... it is a legal `low_water_mark` value, merely a poor one for amortization — Decision 14 —
not a config error"); and VP-125 §Property Statement point 1, whose bound is stated to "hold for any
valid pair `0 <= low_water_mark < N`" without excluding `N-1`. This is not a stale-vs-current
version mismatch — both readings were live in the SAME v1.3 fix-burst's own output set, imposing
mutually-unsatisfiable test obligations on any future implementer (an `EC-011`-literal test asserting
`Error` at `N-1` cannot pass alongside a `VP-140`-literal test asserting normal load at `N-1`).

**Adjudication: Option (b) — accept the full declared range `0 <= low_water_mark < N`, including
`N-1`, as legal; strike the erroneous "fail-loud on `N-1`" language rather than tighten the numeric
constraint.** Two coherent resolutions were weighed:

- *Option (a) — forbid the degenerate range* (e.g. cap `low_water_mark <= floor(N/2)`, or
  `< ceil(N/2)`) would require picking a SECOND principled, non-arbitrary threshold distinct from
  `N-1` itself — and no such threshold is non-arbitrary in the way this ADR's other numeric
  constants (Decision 2's fuel-derived `shard_cap_bytes`) are: `N-2` and `N-3` amortize almost as
  poorly as `N-1` (amortization factor `N - low_water_mark` is `1`, `2`, `3` respectively — all
  "essentially every write" by the standard this Decision's own "Problem" paragraph sets), so a hard
  floor drawn at `floor(N/2)` would be an equally defensible-or-arbitrary line as one drawn at
  `floor(N/4)` or anywhere else in between. Tightening the constraint would ALSO require rewriting
  BOTH already-authored, internally-consistent downstream artifacts (BC-1.18.005's CTV + EC-011,
  and VP-140's four-cell fail-loud boundary table + proof harness, plus a re-review of VP-125's
  general "any valid pair" property) to newly exclude values they currently — correctly, per their
  own content — treat as valid.
- *Option (b) — accept `N-1` (and every other interior value) as legal-but-poor* requires striking
  ONE incorrect parenthetical from Decision 14 (this document) and ONE structurally identical
  parenthetical from BC-1.18.005 EC-011 — a strictly smaller, consistency-PRESERVING change, since
  it aligns this ADR with content THREE other artifacts (BC-1.18.005's own CTV, VP-140, VP-125)
  already state correctly, rather than requiring those three artifacts to be reopened and rewritten
  to match this ADR's erroneous framing.

Option (b) is ADOPTED. The numeric constraint `0 <= low_water_mark < N` itself was NEVER wrong and
is UNCHANGED by this correction — only the prose claiming `N-1` falls inside the fail-loud `>= N`
bucket is withdrawn, because it does not: `N-1` is, by construction, strictly less than `N`.

**Closing the latent-pathology risk without a false fail-loud claim: a non-fatal amortization
advisory, not a rejection.** Accepting `N-1` as legal does not leave the latent design hole
unaddressed — an operator configuring `low_water_mark` close to `N` (up to and including `N-1`)
reproduces exactly the every-write (or near-every-write) rotation churn this SAME Decision 14 exists
to eliminate for the DEFAULT case. The corrected resolution is a NEW, NON-FATAL, WARN-level
diagnostic advisory (via `tracing::warn!`, per CLAUDE.md's structured-logging convention — never a
`println!`, never `HookResult::Error`), emitted at config-load time AFTER the existing fail-loud
validation (`0 <= low_water_mark < N`) already passes:

- **Advisory condition:** `low_water_mark > floor(N/2)` — i.e., the configured value amortizes
  rotation WORSE than this Decision's own recommended default. This threshold is deliberately NOT a
  new free-standing arbitrary constant: it reuses the ALREADY-JUSTIFIED default `floor(N/2)` this
  same Decision establishes above as the anchor for "worse than recommended," rather than inventing
  a second independent numeric line the way Option (a)'s hard floor would have required.
  `low_water_mark <= floor(N/2)` (including the default itself and every value below it) emits no
  advisory.
- **Advisory content:** the configured `(N, low_water_mark)` pair and the resulting amortization
  factor `N - low_water_mark` (writes-per-rotation), compared against the default's
  `N - floor(N/2)` amortization, so an operator can see quantitatively how much worse their
  configured value performs (e.g., at `N=50`, `low_water_mark=49`: "rotation will fire roughly every
  1 write (amortization factor 1), versus 25 for the recommended default `low_water_mark=25`").
- **Advisory is NEVER fatal.** Config load succeeds and returns the configured value unchanged
  (`Continue`/normal load) regardless of whether the advisory fires — this is the load-time
  equivalent of a lint warning, not a validation gate. This is the mechanism-appropriate way to
  address an operator-misconfiguration performance concern without contradicting the numeric
  constraint's own text.

**Exact wording obligations — product-owner (BC-1.18.005), enumerated in full:**

1. **Postcondition 8's rotation-target-config bullet** ("Fail-loud validation constraint"
   sentence): strike the parenthetical `"(including the degenerate N-1)"` from the fail-loud
   sentence. Corrected sentence: "a malformed value (`low_water_mark >= N`, or a negative value) is
   NEVER silently clamped or defaulted around; the config is treated as malformed and the check
   returns `HookResult::Error`." Immediately follow with a NEW sentence: "A legal-but-poor value in
   `(floor(N/2), N)` — up to and including the boundary `low_water_mark = N-1` — loads normally (no
   `HookResult::Error`) but MUST emit a non-fatal `tracing::warn!` advisory citing the configured
   `(N, low_water_mark)` pair and the resulting amortization factor `N - low_water_mark`, compared
   against the recommended default's `N - floor(N/2)` amortization."
2. **EC-011:** strike `"(including the degenerate low_water_mark = N-1)"` from the description
   column — EC-011's scope becomes exactly `low_water_mark >= N` (the `== N` boundary, or any value
   strictly greater) or a negative value; its Expected Behavior column (`HookResult::Error`) is
   UNCHANGED for that (now-corrected) scope.
3. **NEW EC-012:** Description: "A `\"frontmatter-changelog-array\"`-shaped config entry declares a
   legal-but-poor `low_water_mark` in `(floor(N/2), N)`, up to and including `low_water_mark = N-1`."
   Expected Behavior: "Loads normally (`Continue`, no `HookResult::Error`); emits a non-fatal
   `tracing::warn!` advisory citing the resulting amortization factor `N - low_water_mark` (worse
   than the recommended default's `N - floor(N/2)`)."
4. **Canonical Test Vectors:** the existing last-row parenthetical ("`low_water_mark=49`, i.e.
   `N-1`, is a VALID boundary value... and does NOT fail-loud") is ALREADY CORRECT under this
   adjudication and requires NO wording change. ADD one new row: `N=50`, `low_water_mark=49`
   (`== N-1`) → "Loads normally, no `HookResult::Error`; emits `tracing::warn!` advisory
   (amortization factor `N - low_water_mark = 1` — rotation fires on essentially every write,
   versus `25` for the recommended default)." (EC-012.)
5. **Version/changelog:** bump BC-1.18.005 to v1.4; changelog entry cites this ADR-051 v1.4
   Decision 14 correction (F-P4-001, HIGH) and the exact strike/add above.

**Exact wording obligations — formal-verifier (VP-140, VP-125), enumerated in full:**

1. **VP-140 §Property Statement point 4:** the existing `low_water_mark == N - 1` bullet ("VALID —
   `N - 1 < N` satisfies the constraint... a legal `low_water_mark` value, merely a poor one for
   amortization — Decision 14 — not a config error") is ALREADY CORRECT under this adjudication and
   requires NO wording change to its conclusion.
2. **VP-140 — ADD a new proof leg** ("`low_water_mark` amortization advisory (non-fatal)"): a
   unit-test table over `low_water_mark ∈ {floor(N/2), floor(N/2)+1, N-1}` (for representative
   symbolic `N`) asserting the `tracing::warn!` advisory fires iff `low_water_mark > floor(N/2)`,
   and that config load succeeds (returns the configured value, never `Err`/`HookResult::Error`) in
   ALL three cases — the advisory is orthogonal to, and never downgrades, the existing fail-loud
   legs. Add the corresponding row to the Proof Method table and a new `#[test]` function to the
   Proof Harness Skeleton (e.g. `test_BC_1_18_005_PC8_low_water_mark_amortization_advisory`).
   Bump VP-140 to v1.1; update `last_amended` to record the F-P4-001 addition and cite ADR-051
   Decision 14's corrected text.
3. **VP-125:** NO wording change required. VP-125 §Property Statement point 1 already states its
   bound "holds for any valid pair `0 <= low_water_mark < N`" without excluding `N-1`, and its
   proptest harness (`arb_prepends_and_config()`) already generates arbitrary valid `(N,
   low_water_mark)` pairs, which includes `low_water_mark = N-1` in its generated space by
   construction — VP-125's property was never in conflict with this adjudication; only Decision 14's
   OWN prose and BC-1.18.005 EC-011 needed correction. Formal-verifier MAY, at its discretion, add an
   explicit `N-1`-boundary regression case to VP-125's proptest suite (belt-and-suspenders, since
   proptest's random generation already covers it structurally), but this is not a required wording
   change.

**Amortized cost, stated precisely.** After a rotation trims to `low_water_mark` and the retry
lands (count becomes `low_water_mark + 1`), every subsequent write is a plain `Continue`
(Postcondition 8's trigger does not fire) until the live count climbs back to `N` — i.e., for
`N - low_water_mark - 1` additional writes beyond the retry. Rotation (and its expensive block+retry
round-trip) therefore fires once per `N - low_water_mark` writes to the artifact, amortized — a
strict, quantified reduction from today's once-per-write. At the recommended default
`low_water_mark = floor(N/2)`, this is a ~`N/2`-times reduction in block+retry frequency — for the
illustrative `N ≈ 50` this ADR's own Decision 13 already cites, once per ~25 writes instead of
every write. The per-rotation-EVENT cost rises correspondingly (each rotation now moves `~N/2`
items instead of exactly 1), but remains `O(N)` — bounded by the same small, config-driven `N` this
ADR already treats as a small illustrative constant (tens of items, not thousands) — so the
batched cost is not a new scalability concern; it trades a fixed per-rotation `O(N)` archive-write
cost, paid `1/(N - low_water_mark)` as often, for the previous `O(1)`-per-rotation cost paid on
EVERY write. The thing that amortizes is the actually latency-dominant part this finding
identifies: the fixed block+retry dispatch round-trip overhead, not the archive-write itself.

**Why mechanism A needs NO analogous change (confirmed, not merely assumed).** Mechanism A's seal
step (Decision 3, copy-then-atomic-truncate-in-place) has no "trim by one record" concept at all —
it seals the ENTIRE current shard's content and atomically replaces the canonical file's content
with EMPTY (0 bytes). This is already the MAXIMAL possible low-water mark for a flat-file artifact
shape (there is no partial-retention concept analogous to B1's "keep the newest `keep_recent` YAML
items" for an undifferentiated byte stream — a flat file has no internal record boundary the seal
step could partially preserve across) — mechanism A's low-water mark is unconditionally `0` after
every roll, by construction, not by a tunable choice a config value could set differently.
Consequently, mechanism A's amortization factor is already `shard_cap_bytes / typical_write_size`
(e.g., ~48 KiB / a few hundred bytes for a typical `decision-log.md` entry — hundreds of writes
between rolls), far larger than B1's pre-fix one-write amortization, and requires no
Decision-14-style correction. This asymmetry — mechanism A structurally hysteretic by the shape of
its seal operation, mechanism B1 requiring an explicit low-water parameter because its shape
supports partial retention — is the reason F-P3-005 is a B1-only defect, not a two-mechanism one,
stated here explicitly per this fix-burst's own instruction that the asymmetry be documented rather
than left for a future reader to independently re-derive.

**Consistency with Decision 13's one-time cold-start migration.** Decision 13 (above) is amended in
this SAME fix-burst to target `keep_recent = low_water_mark`, not `keep_recent = N`, for the exact
same reason this Decision exists: migrating the cold-start backfill down to `N` would leave the
live sequence at the trigger boundary immediately after go-live, and the very first ordinary
post-migration write would immediately re-trigger rotation — reproducing this Decision's own
just-closed pathology from write #1 of the new steady state. Both the ongoing per-write gate
(BC-1.18.009) and the one-time cold-start migration (BC-1.18.012) MUST target the SAME
`low_water_mark` config value for the corrected steady state to hold from the first post-migration
write onward, not merely from the second rotation.

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
`rotate_changelog`'s own signature is a pure trim (keep `keep_recent` items — for B1, the
configured `low_water_mark` per Decision 14, NEVER a fixed `N-1` — archive the rest) — it has no
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

**Why copy-then-atomic-truncate over rename-away for the seal step (fix-burst, F-P2-003):**
a two-step "rename canonical away, then create a fresh canonical" sequence has an unavoidable
interstitial window where the canonical path resolves to nothing at all — `rename()` and `create()`
are two separate syscalls, and nothing prevents a concurrent reader's `open()` from landing between
them. Replacing the canonical file's CONTENT via a single `write_atomic` temp-then-rename-ONTO
operation has no such window, because renaming a temp file onto an EXISTING destination is an
atomic directory-entry REPLACEMENT, not a delete-then-create — this is the same distinction that
makes `write_atomic`/`write_indeterminate_marker` safe for every OTHER mutation in this codebase,
simply applied to "replace with empty content" instead of "replace with new content."

**Why hysteresis (a `low_water_mark` distinct from `N-1`) for B1's rotation target, and why
mechanism A needs no analogous change (fix-burst, F-P3-005):** trimming to exactly `N-1` on every
rotation leaves the live sequence back at the trigger boundary (`N`) after the very next successful
prepend, so every steady-state write after the first rotation re-triggers the block+retry
round-trip — the opposite of this ADR's own "zero added latency for ~99% of writes" goal for an
artifact (`BC-INDEX.md`) that is written on essentially every burst. `rotate_changelog`'s
`keep_recent: usize` parameter is ALREADY free (never hardcoded to `N-1` in the shipped function),
so batching the eviction down to a configurable low-water floor costs nothing beyond a different
call-site argument and one new config field — no new rotation logic, no change to
`rotate_changelog`'s content-preservation or ordering guarantees. Mechanism A needs no equivalent
because its seal step already truncates the canonical file to fully empty (0 bytes) on every roll —
the maximal possible low-water mark for an undifferentiated flat-file shape, which has no partial
per-record retention concept to tune in the first place. See Decision 14 for the full design.

**Why B1's archive path needed a bounded extension, not a forced `cycle_name` (fix-burst,
F-P2-001):** `rotate_changelog`'s existing `cycle_name`-derived path convention is correct
and unchanged for its EXISTING (cycle-scoped) callers; forcing a synthetic `cycle_name` for a
non-cycle catalog artifact like `BC-INDEX.md` would misfile its archive under `.factory/cycles/`,
a directory whose established meaning is cycle-scoped content — a cheap-looking shortcut that would
create a permanent, confusing address-space collision. Parameterizing the archive path directly
(reusing every other line of the existing function unmodified) costs one new parameter and
preserves both callers' correctness.

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
10. **(fix-burst, v1.2)** No sharded artifact class is left depending on a rename-away seal with an
    ENOENT window (Decision 3) — every shard-unaware reader's AC-007 transparency guarantee now
    holds structurally, not merely in the common case.
11. **(fix-burst, v1.2)** The per-write roll's crash surface is now fully enumerated with
    self-healing recovery for every partial-failure point (Decision 11), matching the rigor
    BC-1.18.008/011 already apply to the (far less frequent) one-time migrations.
12. **(fix-burst, v1.2)** No sharded artifact class (mechanism A, B1, or B2) is left depending on an
    ungoverned lazy-first-write migration for its largest one-time content displacement (Decision
    13 closes B1's remaining asymmetry with BC-1.18.008/BC-1.18.011).
13. **(fix-burst, v1.3)** B1's rotation cost now amortizes across a batch of `N - low_water_mark`
    writes instead of being paid on every single write to `BC-INDEX.md` after the first rotation —
    closing the steady-state-retrigger defect F-P3-005 identified, with zero new logic in
    `rotate_changelog` (Decision 14 reuses its already-free `keep_recent` parameter).

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
7. **(fix-burst, v1.2)** B1's archive file (`BC-INDEX-changelog-archive.md`) is itself append-only
   and unbounded across the artifact's lifetime — an accepted, documented residual (Decision 7's
   fix-burst correction), not a defect, since it is read by neither the trigger nor any Cohort B
   validator today; if it later becomes large enough to matter, that is a follow-up story, not a
   gap in this design.
8. **(fix-burst, v1.2)** The corrected retry wording (Decision 3) removes the v1.1 asymmetry between
   `Edit`/`MultiEdit` and `Write` retry text — both branches now converge on "recompute against
   post-roll state" — which is a net UX simplification, not a cost, but is listed here because it
   is a behavior CHANGE relative to v1.1 that downstream test vectors must track.
9. **(fix-burst, v1.2)** Decision 12's sealed-shard direct-edit escape hatch means Layer 2 provides
   NO automated detection or prevention of a POLICY-1-violating edit to historical content —
   enforcement remains entirely at the `consistency-validator`/adversary-prompt agent level
   (unchanged from Decision 6's existing posture), an explicit, accepted scope boundary rather than
   a gap this ADR silently leaves unaddressed.
10. **(fix-burst, v1.3)** `low_water_mark` introduces one additional config value per
    `"frontmatter-changelog-array"`-shaped `[[shard]]` entry, beyond `N` — mitigated by defaulting
    to `floor(N/2)` when omitted, so an operator who never touches the new field gets a sound
    hysteresis band automatically, not a required extra configuration burden.
11. **(fix-burst, v1.3)** Each individual rotation event now moves `~N - low_water_mark` items
    instead of exactly 1 — a larger, but still `O(N)`-bounded (tens of items, not thousands),
    per-event archive-write payload, traded against a `(N - low_water_mark)`-times reduction in how
    often that payload is written at all (Decision 14's amortization analysis).

### Status as of 2026-09-05 (v1.0); fix-burst amendment 2026-09-05 (v1.1); fix-burst amendment 2026-09-05 (v1.2); fix-burst amendment 2026-09-05 (v1.3)

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

**v1.2 (this fix burst) resolves a fresh-context adversary pass-2 review's ARCHITECTURE-routed
findings against v1.1:** F-P2-001 (HIGH — `rotate_changelog`'s shipped signature cannot produce the
per-`seq` sealed-shard layout BC-1.18.009 v1.1 described; corrected to a single evergreen
archive file at `.factory/specs/behavioral-contracts/BC-INDEX-changelog-archive.md`, reached via a
small, named, bounded extension to `rotate_changelog`'s path-resolution surface — Decision 7
amendment); F-P2-002 (HIGH — the `Write`-tool `projected_size` formula double-counted a `Write`'s
already-complete `content` on top of `current_size`, over-triggering rotation, and the v1.1 "retry
unchanged" `Write` guidance could re-land a stale over-cap payload into a freshly-emptied shard,
causing a permanent block/retry deadlock; corrected to a tool-discriminated formula and a unified
"recompute against post-roll state" retry instruction — Decision 1 step 3 + Decision 3 amendments);
F-P2-003 (HIGH — BC-1.18.006 Postcondition 1(a)'s rename-away seal directly contradicted its own
Invariant 3 and opened an ENOENT transparency window for shard-unaware readers; corrected to a
copy-then-atomic-truncate-in-place seal mechanism using only already-established `write_atomic`
primitives — Decision 3 amendment); F-P2-004 (MEDIUM — the per-write roll's three composite writes
had only ONE named partial-failure code, `E-SHD-001`, leaving two crash points unspecified;
resolved with a staged sequence and two new self-healing partial-failure codes, `E-SHD-006`/
`E-SHD-007` — new Decision 11); F-P2-005 (MEDIUM — the roll/retry contract's implicit
append-only-tail assumption was never stated, and had no defined behavior for an edit targeting
already-sealed historical content; resolved by making the POLICY-1-grounded append-only assumption
explicit and specifying the sealed-shard-direct-edit escape hatch as the caller's correct recovery
path — new Decision 12); F-P2-007 (MEDIUM — BC-1.18.005 Postcondition 8's "bounded read"
characterization is false at B1's cold start, ~1,997 unrotated `changelog:` items today, and B1 had
no governed one-time backfill migration analogous to BC-1.18.008/BC-1.18.011; resolved by requiring
a new governed migration BC, illustratively BC-1.18.012, modeled on BC-1.18.008 — new Decision 13).
Status remains PROPOSED — none of these are POLICY 22 design-direction reversals; all correct or
complete v1.0/v1.1's own stated design intent, grounded directly in `rotate_changelog`'s shipped
implementation. Downstream: product-owner rewrites BC-1.18.005 (per-tool formula), BC-1.18.006
(copy+truncate seal mechanics, unified retry wording, new Invariant for the append-only-tail
assumption, new EC for the sealed-shard escape hatch), BC-1.18.009 (corrected archive path/scheme,
no per-seq shard directory), and authors the new B1 backfill migration BC (illustratively
BC-1.18.012); formal-verifier allocates new VP coverage (next free VP-135+ against VP-INDEX v3.03)
for the staged partial-failure model, the corrected formula/retry contract, and the new migration
BC — full enumeration in the companion F2 architecture-delta doc §4b.

**v1.3 (this fix burst) resolves a fresh-context adversary pass-3 review's ARCHITECTURE-routed
findings against v1.2:** F-P3-005 (MEDIUM — B1's minimal-eviction rotation target, trimming to
exactly `N-1` on every rotation, leaves the live `changelog:` sequence back at the trigger boundary
after the very next successful prepend, so EVERY steady-state write to `BC-INDEX.md` after the
first rotation re-triggers a block+retry round-trip, contradicting this ADR's own "zero added
latency for ~99% of writes" framing for the single hottest 4-index-discipline artifact; resolved by
new Decision 14, introducing a `low_water_mark` config value — sibling to the unchanged trigger
threshold `N` — that the gate's rotation step targets instead of `N-1`, reusing `rotate_changelog`'s
already-free `keep_recent: usize` parameter with zero new rotation/ordering/content-preservation
logic; default `floor(N/2)` when omitted; rotation now amortizes to once per `N - low_water_mark`
writes; mechanism A confirmed, not merely assumed, to need no analogous change, since its
copy-then-atomic-truncate seal already resets the live shard to the maximal possible low-water
mark — fully empty — on every roll). Decision 13 amended in the SAME burst for consistency: the
one-time cold-start backfill migration (BC-1.18.012) now targets `keep_recent = low_water_mark`,
never `keep_recent = N`, so the corrected steady state holds from the FIRST post-migration write,
not merely from the second rotation onward (an inconsistency this burst caught and closed
proactively, not itself a separately-numbered finding). F-P3-007 (LOW — Decision 10 Postcondition
2's independent-census sanity-bound cited a pinned, already-stale illustrative `total_bcs` figure,
"e.g. 1,997 per BC-INDEX v5.50"; corrected to a structural, count-redacted description — the
field's value AT MIGRATION TIME, re-read live rather than pinned to a historical snapshot — per
TD-VSDD-091 and the SAME re-grounding convention F-P2-006 already established for BC-1.18.010's own
count citation). Status remains PROPOSED — neither finding is a POLICY 22 design-direction
reversal; both correct v1.2's own stated design intent. Downstream: product-owner rewrites
BC-1.18.005 (new `low_water_mark` config field declaration + fail-loud validation, Postcondition 8
amendment; two new Edge Cases), BC-1.18.009 (Postcondition 1's "capped at N most-recent items"
framing corrected to the fluctuating-between-`low_water_mark+1`-and-`N` hysteresis band;
Postcondition 2's rotation-target citation corrected from `N-1` to `low_water_mark`;
retry-instruction text updated; one new Edge Case and Canonical Test Vector demonstrating the
amortized cadence), and BC-1.18.012 (Decision 13 item 2's `keep_recent` target citation corrected
from `N` to `low_water_mark`); formal-verifier reviews VP-125/126 fixtures against the
`low_water_mark`-parameterized `keep_recent` call (the bounded-live-sequence and no-history-loss
properties still hold structurally; fixtures must assert the post-rotation floor is
`low_water_mark`, not `N-1`) — full enumeration in the companion F2 architecture-delta doc §4c.

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
- **(fix-burst, v1.2) Option: force a sentinel `cycle_name` value through `rotate_changelog`'s
  existing signature unmodified, accepting a `BC-INDEX-changelog-archive.md` path under
  `.factory/cycles/<sentinel>/`.** Rejected: this is the "zero-code-change" option the F-P2-001
  finding also offered, but it permanently misfiles a specs-catalog artifact's archive under a
  directory whose established meaning is cycle-scoped content, creating a standing address-space
  confusion for any future human or validator inspecting `.factory/cycles/`. A one-parameter,
  additive path-resolution extension (Decision 7's adopted fix) costs less than the semantic debt
  the sentinel-value option would leave behind.
- **(fix-burst, v1.2) Option: leave the ongoing per-write roll's crash-atomicity unspecified beyond
  `E-SHD-001`, treating a mid-roll crash as an out-of-scope operational concern.** Rejected: the
  per-write roll is the single MOST FREQUENT operation this ADR introduces (it fires on every
  future over-cap write, unlike the one-time migrations), and CLAUDE.md's production-grade default
  forbids leaving a genuinely-identified partial-failure surface undocumented merely because the
  one-time migrations already received more rigorous treatment — Decision 11 closes the gap with
  bounded, reused-primitive machinery, not a new atomicity mechanism.

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
- **Fix-burst (v1.2) code inspection (2026-09-05), grounding Decisions 1/3/7/11/12/13's
  amendments:** `crates/last-amended-migrate/src/rotate.rs` (`resolve_archive_path`,
  `rewrite_source_after_rotation`, `rotate_changelog` — re-inspected line-by-line to confirm the
  single-fixed-destination/`cycle_name`-required/append-not-shard behavior grounding Decision 7's
  archive-scheme correction, and the write-then-validate-then-`write_atomic` ordering grounding
  Decision 11's staged-sequence design); `crates/last-amended-migrate/src/atomic_write.rs`
  (`write_atomic` — re-confirmed as a temp-file-then-rename-ONTO-destination primitive, grounding
  Decision 3's copy-then-atomic-truncate correction: renaming onto an EXISTING path is an atomic
  replace, never a delete-then-create); `.factory/specs/prd-supplements/error-taxonomy.md`
  (existing `E-SHD-001`..`E-SHD-005` rows — confirmed `E-SHD-001`'s code/contract is reusable
  under a refreshed description, and `E-SHD-003`'s wording is already artifact-generic and
  reusable for Decision 13's B1 backfill, avoiding unnecessary new-code proliferation);
  `.factory/specs/behavioral-contracts/ss-01/BC-1.18.005.md`/`BC-1.18.006.md`/`BC-1.18.009.md`
  (v1.1 bodies — direct inspection confirming the exact Postcondition/Invariant text this fix-burst's
  findings contradict, grounding the precise rewrite obligations enumerated in the companion F2
  architecture-delta doc §4b).
- **Fix-burst (v1.3) code re-inspection (2026-09-05), grounding Decision 14:**
  `crates/last-amended-migrate/src/rotate.rs`'s `rotate_changelog` signature
  (`keep_recent: usize` — confirmed a genuinely free caller-supplied parameter, never hardcoded to
  `N-1` anywhere in the function) and its no-op guard (`total <= keep_recent`, `rotate_changelog`'s
  own EC-004) and retained/moved split
  (`doc.changelog_items_raw.split_at(keep_recent)`, operating on the existing newest-first ordering)
  — re-inspected to confirm Decision 14's hysteresis fix requires passing a DIFFERENT numeric value
  to this ALREADY-EXISTING parameter, not any change to `rotate_changelog`'s own logic, ordering, or
  content-preservation guarantees. This ADR's own Decision 13 (v1.2 text) was re-inspected in the
  same pass to find and correct the `keep_recent = N` cold-start-backfill target, which would have
  reintroduced Decision 14's just-closed pathology from the first post-migration write.

## Changelog

| Version | Date | Author | Summary |
|---|---|---|---|
| 1.7 | 2026-09-06 | architect | S-25.02 F2 sibling-sweep micro-burst (adversary pass-7 F-P7-001 closure, product-owner-flagged architect stragglers): Decision 7's block-and-retry sequence (the "PURE TRIM" grounding bullet, and steps 3-4 of the corrected single-actor contract) and Decision 11's staged-roll-sequence heading both still described B1's rotation TARGET as a literal `N-1`, contradicting Decision 14 (v1.3+), which replaced the fixed `N-1` eviction target with the configured `low_water_mark` (default `floor(N/2)`) precisely to close the every-write rotation-churn pathology Decision 14 documents. Corrected all four LIVE occurrences (Decision 7's pure-trim descriptor; Decision 7 step 3's rotation-target citation and step 4's post-retry item-count math; Decision 11's "truncate-to-N-1-items" heading clause; the Rationale section's "Why B1's gate performs ONLY the trim" pure-trim descriptor) to cite `low_water_mark`/`keep_recent` generically, each with an explicit "NEVER a fixed `N-1`" cross-reference to Decision 14. No decision content changed — Decision 14 already establishes `low_water_mark` as the authoritative target; this burst brings Decision 7/11/Rationale's own exposition into agreement with the Decision they predate. Full grep-verified: every remaining `N-1` occurrence in this ADR is now either an explicit negation ("NEVER `N-1`", "distinct from `N-1`"), a legal-but-poor-boundary-value discussion (Decision 14's own F-P4-001 adjudication, which correctly treats `N-1` as an admitted-but-suboptimal value, not the design target), a superseded-version attribution (Decision 14's "Problem" paragraph, explicitly citing "`BC-1.18.009` **v1.2**'s rotation step"), or a Changelog/Status-narrative historical row (POLICY-1 append-only exempt). Reviewed the companion `S-25.02-f2-architecture-delta.md`'s §4a/§4b per-pass BC-authorship-input tables for the same staleness: LEFT UNCHANGED — those sections are explicitly labeled by adversary-pass number ("adversary pass-1"/"adversary pass-2, architect-routed findings"), and §4c/§4d already perform the identical `N-1`→`low_water_mark` correction one/two passes later in the SAME append-only document, so §4a/§4b's `N-1` content is a genuinely historical record of what THAT pass's ADR version (v1.1/v1.2) instructed, superseded in-document rather than in need of retroactive rewrite. Status remains PROPOSED — not a POLICY 22 reversal; corrects this ADR's own exposition to agree with its own already-adopted Decision 14, no decision content altered. Companion `S-25.02-f2-architecture-delta.md` UNCHANGED this burst.|
| 1.6 | 2026-09-06 | architect | S-25.02 F2 gate-audit fix (F-P6-001, MEDIUM): Decision 6's opening justification paragraph carried a now-stale present-tense claim that CAP-043's `SS-01/SS-07`-only subsystem list was "incomplete" and "flagged as a product-owner follow-up ... not amended here." That follow-up CLOSED same-cycle — CAP-043 (v1.21) now lists SS-01/SS-04/SS-07 — leaving the ADR body contradicting the capability it describes. Rewrote the clause to past tense/closure-acknowledging, referencing CAP-043's §Subsystems list structurally (by name/section anchor, no version pin) rather than reasserting incompleteness. SS-04 justification substance (the four-WASM-crate validator-enumeration audit + POLICY-1 archive-inclusive-glob obligation) is UNCHANGED. Status remains PROPOSED — not a POLICY 22 reversal; corrects a stale cross-reference only.|
| 1.5 | 2026-09-06 | architect | S-25.02 F2 gate-audit fix (F3, MEDIUM, ADJUDICATION) resolving a perimeter-consistency finding raised ahead of the F2 human ratification gate: `subsystems_affected: [SS-01, SS-04, SS-07]` carried SS-04 with no substantive body justification (Decisions 1–5/7–14 are exclusively SS-01/SS-07; CAP-043 lists only SS-01/SS-07). ADJUDICATED Option (a) — SS-04 is genuinely affected: Decision 6's validator enumeration audits four SS-04-owned WASM crates (`validate-dispatch-advance`, `validate-state-structure`, `validate-closes-completeness`, `validate-cross-site-correspondence`) for archival-scheme correctness and imposes a new archive-inclusive-glob obligation on POLICY-1's SS-04-adjacent enforcement path. Added an explicit one-line justification note at the top of Decision 6 naming this touchpoint; `subsystems_affected` and the mirrored ARCH-INDEX ADR-051 row Subsystems column are UNCHANGED (`[SS-01, SS-04, SS-07]` was already correct, only unjustified). Flags CAP-043's `SS-01/SS-07`-only subsystem list as a product-owner follow-up (business-analyst/product-owner domain, not amended by this architect-authored fix). Status remains PROPOSED — not a POLICY 22 design-direction reversal; adds justification only, no decision content altered.|
| 1.4 | 2026-09-06 | architect | Fix-burst amendment resolving fresh-context adversary pass-4 findings against v1.3 (both ARCHITECTURE-routed). F-P4-001 (HIGH, ADJUDICATION): Decision 14's own "fail-loud... including the degenerate `N-1`" framing and BC-1.18.005 EC-011's identical phrasing directly contradicted the numeric constraint they both cite (`0 <= low_water_mark < N` mathematically ADMITS `N-1`) and directly contradicted BC-1.18.005's own Canonical Test Vectors table plus VP-140/VP-125, all of which already treat `N-1` as VALID — a live, mutually-unsatisfiable test-obligation contradiction (an EC-011-literal test expecting `Error` at `N-1` cannot pass alongside a VP-140-literal test expecting normal load at `N-1`). ADJUDICATED as Option (b): the numeric constraint `0 <= low_water_mark < N` is UNCHANGED and remains correct; the erroneous "fail-loud on `N-1`" prose is WITHDRAWN from Decision 14 (this ADR corrects its own text in this burst) with the exact strike/add wording obligations enumerated for product-owner's BC-1.18.005 v1.3→v1.4 follow-on fix-burst (Postcondition 8 sentence rewrite, EC-011 scope narrowed to exactly `>= N`/negative, NEW EC-012, one new Canonical Test Vector row) and formal-verifier's VP-140 v1.0→v1.1 follow-on fix-burst (one new amortization-advisory proof leg; VP-125 confirmed to need NO wording change, its property already generalizes over the full valid domain). The latent every-write-rotation pathology a legal `low_water_mark` close to `N` could reproduce is closed via a NEW non-fatal `tracing::warn!` amortization advisory (fires when `low_water_mark > floor(N/2)`, reusing this Decision's own already-justified default as the non-arbitrary advisory threshold rather than inventing a second free constant) — never a fail-loud rejection of a value the constraint already declares legal. Option (a) (tightening the numeric constraint to forbid high `low_water_mark` values) was considered and rejected: any such hard floor is exactly as arbitrary as `N-1` itself (`N-2`/`N-3` amortize almost as poorly) and would require reopening three already-correct downstream artifacts to newly exclude values they currently, correctly, treat as valid — Option (b) is the smaller, consistency-preserving change. F-P4-004 (LOW): Decision 13's worked-example arithmetic corrected from the stale withdrawn-target remainder `1,947` (`1,997 - 50`, the WITHDRAWN `keep_recent = N` target) to the correct `1,972` (`1,997 - 25`, the CURRENT `keep_recent = low_water_mark` target per Decision 14) — three occurrences corrected, now consistent with BC-1.18.012 v1.1's own EC-001/Canonical-Test-Vector figures (not itself a wording obligation for product-owner, since BC-1.18.012 already carries the correct number; this was an ADR-body-only citation lag). Status remains PROPOSED — neither finding is a POLICY 22 reversal; both correct v1.3's own stated design intent, the first by withdrawing an internally-contradictory claim in favor of content three sibling artifacts already state correctly, the second by fixing stale arithmetic against an already-superseded target. Companion `S-25.02-f2-architecture-delta.md` v1.3→v1.4 (§4d added).|
| 1.3 | 2026-09-05 | architect | Fix-burst amendment resolving fresh-context adversary pass-3 findings against v1.2 (both ARCHITECTURE-routed). F-P3-005 (MEDIUM): new Decision 14 introduces high-water/low-water hysteresis for B1's item-count rotation target — trimming to exactly `N-1` on every rotation left the live `changelog:` sequence back at the trigger boundary after the very next successful prepend, so EVERY steady-state write to `BC-INDEX.md` re-triggered a block+retry round-trip; corrected by reusing `rotate_changelog`'s already-free `keep_recent: usize` parameter with a NEW sibling config value `low_water_mark` (default `floor(N/2)`, fail-loud-validated `0 <= low_water_mark < N`) instead of `N-1` — zero new rotation/ordering logic, rotation now amortizes to once per `N - low_water_mark` writes. Decision 13 amended for consistency: the one-time cold-start backfill migration now targets `keep_recent = low_water_mark`, never `keep_recent = N`, so the corrected steady state holds from the FIRST post-migration write. Mechanism A confirmed (not merely assumed) to need no analogous change — its copy-then-atomic-truncate seal already resets the live shard to the maximal possible low-water mark (fully empty) on every roll, since a flat-file shape has no partial-retention concept to tune. F-P3-007 (LOW): Decision 10 Postcondition 2's pinned, already-stale illustrative `total_bcs` citation ("e.g. 1,997 per BC-INDEX v5.50") corrected to a structural, count-redacted description per TD-VSDD-091, matching the F-P2-006 re-grounding convention already established for BC-1.18.010. Status remains PROPOSED — neither finding is a POLICY 22 reversal; both correct v1.2's own stated design intent. Companion `S-25.02-f2-architecture-delta.md` v1.2→v1.3 (§4c added).|
| 1.2 | 2026-09-05 | architect | Fix-burst amendment resolving fresh-context adversary pass-2 findings against v1.1 (all ARCHITECTURE-routed). F-P2-001 (HIGH): Decision 7's B1 archive scheme corrected from an impossible per-`seq` sealed-shard-directory layout to a single evergreen append-file (`.factory/specs/behavioral-contracts/BC-INDEX-changelog-archive.md`), reached via a small, named, bounded extension to `rotate_changelog`'s path-resolution surface (explicit `archive_path` parameter, replacing forced `cycle_name` derivation for non-cycle callers) — grounded in direct re-inspection of `resolve_archive_path`'s actual single-fixed-destination/append/cycle_name-required behavior. F-P2-002 (HIGH): Decision 1 step 3's `projected_size` formula corrected to be tool-discriminated (`Write`: `len(content)` alone; `Edit`/`MultiEdit`: `current_size + net_delta_bytes`, unchanged) — the withdrawn v1.1 formula double-counted a `Write`'s complete content on top of current size; Decision 3's retry wording unified into a single "recompute against post-roll state" instruction for both tool classes, closing the stale-full-payload block/retry deadlock the v1.1 "if Write, retry unchanged" text permitted. F-P2-003 (HIGH): Decision 3's seal mechanism corrected from rename-away (which directly contradicted BC-1.18.006's own Invariant 3 and opened an ENOENT transparency window) to copy-then-atomic-truncate-in-place, reusing only the existing `write_atomic` temp-file-then-rename-ONTO-destination primitive. F-P2-004 (MEDIUM): new Decision 11 adds a staged, crash-recoverable per-write roll sequence with two new self-healing partial-failure error codes (`E-SHD-006` seal-published-but-canonical-not-truncated; `E-SHD-007` canonical-truncated-but-index-not-published), closing the gap between the per-write roll's under-specified atomicity and the one-time migrations' (BC-1.18.008/011) already-rigorous staging+verify+rollback treatment. F-P2-005 (MEDIUM): new Decision 12 makes the append-only-tail assumption underlying the roll/retry contract explicit (grounded in POLICY-1) and specifies the sealed-shard direct-edit escape hatch as the correct, gate-transparent recovery path for a caller needing to touch already-relocated historical content. F-P2-007 (MEDIUM): new Decision 13 requires a governed one-time B1 changelog backfill migration (illustratively BC-1.18.012, modeled on BC-1.18.008) to eliminate B1's cold-start ~1,997-item ungoverned lazy-first-write migration and corrects BC-1.18.005 Postcondition 8's "bounded" claim to distinguish cold-state from steady-state. Status remains PROPOSED — none of these are POLICY 22 reversals; all correct v1.1's own stated design intent against the actual shipped `rotate_changelog`/`write_atomic` implementations. Companion `S-25.02-f2-architecture-delta.md` v1.1→v1.2 (§4b added).|
| 1.1 | 2026-09-05 | architect | Fix-burst amendment resolving fresh-context adversary pass-1 findings against v1.0. F-S2502-F2-001 (BLOCKER): Decision 7's B1 sub-mechanism corrected from a double-actor "gate rotates+prepends, then Continue" design (unsound — double-prepend/stale-payload-clobber, the exact hazard BC-1.18.006 forbids) to a single-actor block-and-retry contract structurally identical to mechanism A's (gate performs ONLY the `rotate_changelog` trim, then Blocks with a retry instruction; the agent's own call, original or retried, performs the sole prepend), grounded in direct inspection of `rotate_changelog`'s actual pure-trim signature. F-S2502-F2-002 (HIGH): added Decision 10, a governed one-time migration for the B2 BC-INDEX body split (content-preservation, independent-census, crash-atomicity, rollback, idempotency, covering SS-05/SS-06 second-level sub-splits in the same operation), modeled on BC-1.18.008, with enumerated postcondition obligations for product-owner's new migration BC (illustratively BC-1.18.011). F-S2502-F2-005 (MEDIUM): Decision 1 amended with an explicit trigger-shape dispatch — BC-1.18.005 owns BOTH the byte-size trigger (mechanism A) and the item-count trigger (mechanism B1), with the item-count shape's distinct (bounded-parse, not `stat()`-only) read-cost model documented. F-S2502-F2-008 (MEDIUM): Decision 6 amended with a code-grounded enumeration of every candidate whole-corpus history-scanning validator — `check_d_chain_currency`/`scan_max_d_nnn`/`scan_max_decision_log_id`, `check_decisions_log_monotonicity`, `validate-closes-completeness`'s decision-log arm, `validate-cross-site-correspondence`'s `is_volatile_path`, and Cohort B (`validate-burst-log`/`regression-gate`/`convergence-tracker`) are all verified NOT affected by archival (STATE.md-scoped or correctly current-shard-scoped); POLICY-1 (`append_only_numbering`) enforcement (`consistency-validator`/adversary-prompt, `lint_hook: null`) is identified as the one genuine gap and MUST default to an archive-inclusive whole-corpus scan mode, an explicit carve-out from this Decision's general opt-in-required default. Cosmetic: Decision 3's sort-order rationale corrected (the operative comparison is digit-vs-`m` one byte past the shared `decision-log` prefix, not `.` vs. digit; conclusion unchanged). Status remains PROPOSED — none of these are POLICY 22 reversals.|
| 1.0 | 2026-09-05 | architect | Initial authoring. Layer-2 two-mechanism design (append-log rotation + BC-INDEX structured-catalog sharding) per D-1166 widest-scope human decision. Resolves OQ-2 (stable-current-filename addressing + BC-ID-prefix deterministic addressing), OQ-3 (`/compact-state` gets shard-awareness for free via the native dispatcher-mediated gate), OQ-4 (synthetic calibration harness adopted; provisional constants derived from ADR-042's measured fuel/byte model and direct byte measurements of the live artifacts), OQ-5 (co-amended into ADR-047 in the same burst). Identifies and resolves a structural gap the story draft did not address: `HookResult`'s Continue/Block/Error-only contract makes transparent write-redirection impossible, requiring a block-and-retry roll mechanism instead of silent rotation. Status: proposed, pending F2 human gate.|
