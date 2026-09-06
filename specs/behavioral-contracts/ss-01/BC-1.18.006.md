---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-09-05T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-051-layer-2-two-mechanism-size-triggered-shard-rotation-append-logs-and-bc-index-sharding.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.005.md
  - crates/hook-sdk/src/result.rs
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md
input-hash: "ea3eaf1"
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

# BC-1.18.006: Roll-Before-Write via Block-and-Retry (Not Transparent Redirection) Plus Same-Invocation Atomic Shard-Index Publication

## Description

When BC-1.18.005's size-trigger fires, the dispatcher performs the roll (publish a sealed shard
copy of the current content as a NEW file, then atomically REPLACE the canonical file's content
with empty — see the CORRECTED mechanism below — then atomically publish the updated shard index)
and THEN returns `HookResult::Block` with an explicit, actionable retry instruction — never a
silent transparent redirect, which `HookResult`'s three-variant contract (`Continue`/
`Block { reason }`/`Error { message }`) makes structurally impossible. The blocked call is never
applied, so no shard is ever observed over cap by any downstream reader, and the sealed shard plus
its index update land in the SAME native-gate invocation, guaranteeing they are staged in the same
subsequent factory-artifacts commit (TD-VSDD-053 alignment).

**CORRECTED (fix-burst pass-2, F-P2-003, HIGH) — the seal step is COPY-then-ATOMIC-TRUNCATE-
IN-PLACE, NEVER a rename of the canonical path away.** The v1.0/v1.1 text above ("seal by rename")
described the seal as `rename(canonical, sealed)` followed by a separate `create(canonical)` — two
distinct filesystem operations with an interstitial window, between the rename completing and the
fresh-file create completing, during which the canonical path DOES NOT EXIST ON DISK AT ALL. Any
shard-unaware reader (the ~76 fail-open production plugins with directory-scoped `path_allow`
globs, `check_d_chain_currency`, a human `cat`) that happens to `open()` the canonical path inside
that window observes `ENOENT` — a hard failure, not a stale-but-valid read — directly contradicting
this BC's own AC-007-derived "zero-code-change transparency" guarantee and this BC's own Invariant
3 text ("the canonical filename is NEVER renamed away; only its CONTENT is replaced"), which the
withdrawn rename-based mechanism structurally could not satisfy. See Postcondition 1's corrected
text below for the exact replacement sequence, which reuses ONLY the already-established
`write_atomic` (`crates/last-amended-migrate/src/atomic_write.rs`) temp-file-then-rename primitive
— no new atomic-write primitive, no reimplementation.

## Preconditions

1. BC-1.18.005's size-trigger has determined `projected_size > shard_cap_bytes` for a matched
   `Edit`/`Write`/`MultiEdit` tool call.
2. The current shard file for the matched artifact exists (or is treated as a zero-byte current
   shard per BC-1.18.005 EC-004 if this is the artifact's first-ever write).
3. `crates/hook-sdk/src/result.rs`'s `HookResult` enum exposes exactly three variants (`Continue`,
   `Block { reason }`, `Error { message }`) with no redirect or tool-input-mutation capability —
   this precondition is a structural SDK fact, not a runtime state, and is what makes this BC's
   block-and-retry design the ONLY implementable option for roll-before-write under the current
   dispatcher contract.

## Postconditions

1. **CORRECTED (fix-burst pass-2, F-P2-003/F-P2-004, HIGH/MEDIUM) — the roll sequence is a
   STAGED, four-step, crash-recoverable operation that executes BEFORE the block is returned:**
   (a) **read** the canonical file's current full content (a one-time, roll-only read — the cheap
   per-write TRIGGER check, BC-1.18.005 Postcondition 2, remains `stat()`-only; content is read
   ONLY once a roll is already confirmed necessary); (b) **publish the sealed shard as a brand-NEW
   file** at `<stem>.<seq:04>.md` (e.g. `decision-log.0001.md`) via `write_atomic` (a `rename()`
   that CREATES a not-yet-existing destination — never interrupts any reader of the canonical
   path, since sealed filenames are never read by shard-unaware code); (c) **atomically REPLACE
   the canonical file's content with empty**, via the SAME `write_atomic` temp-file-then-rename
   primitive — write an empty temp file, then `rename(temp, canonical)`, which is an atomic
   directory-entry REPLACEMENT of an EXISTING destination (POSIX `rename(2)`; the dispatcher's
   Windows target uses `MoveFileEx` with `MOVEFILE_REPLACE_EXISTING`), never a delete-then-create
   — the canonical path resolves to SOME valid file (old content, then instantaneously the new
   empty content) at every observable instant, never absent; (d) **atomically publish the updated
   shard-index TOML** (temp-file-then-rename, the same pattern already established by
   `write_indeterminate_marker` and `write_atomic` — no new atomic-write primitive is introduced).
   Only after (a)-(d) complete does the gate return `HookResult::Block`. **This WITHDRAWS the
   v1.0/v1.1 "seal by rename-away, then create a fresh file" mechanism**, which opened a real
   ENOENT window between the rename and the create (see Description above) — step (b) is a
   `rename()` that CREATES a new sealed path (never vacates the canonical one), and step (c) is a
   `rename()` ONTO the existing canonical path (an atomic replace-in-place), so the canonical path
   is NEVER, at any instant, absent from disk.

   **Partial-failure postconditions — one named `E-SHD-NNN` code per crash point (ADR-051
   Decision 11), because this composite three-write operation (steps b/c/d) has THREE distinct
   crash points, not one:**
   - **Steps (a)-(b) fail (`E-SHD-001`, description REFINED from "seal-rename failure" to
     "shard-seal-write failure" to match the corrected copy-based mechanism — the CODE and
     observable contract are unchanged: `HookResult::Error`, canonical file completely
     untouched):** the canonical file is left in its exact pre-roll state (still over cap, still
     holding its full original content) — safe, no data loss, no duplicate; the next dispatch
     attempt re-evaluates the trigger and re-attempts the FULL sequence from step (a).
   - **Step (c) fails after step (b) succeeded (NEW `E-SHD-006`):** the sealed shard now durably
     exists (a byte-for-byte copy of the pre-roll content) AND the canonical file STILL holds that
     same content too (not yet truncated) — a transient, DETECTABLE duplicate-content state, not a
     data-loss state. **Recovery (self-healing, no operator intervention):** on the NEXT dispatch
     attempt for this artifact, BEFORE evaluating any new trigger, the gate checks whether a
     sealed shard exists at the index's next-expected `seq` path whose content is byte-identical
     to the canonical file's CURRENT content; if so, this is recognized as "seal published,
     truncate did not," and the gate resumes from step (c) alone (re-attempting ONLY the truncate
     + index publish, never re-writing the already-correct sealed shard) — idempotent by
     construction, since step (b)'s `write_atomic` create is itself a no-op if reissued against
     identical content.
   - **Step (d) fails after step (c) succeeded (NEW `E-SHD-007`):** the canonical file is
     CORRECTLY fresh and empty (safe for all future writes — no over-cap risk, no data loss) and
     the sealed shard file exists correctly on disk, but `<artifact-stem>.shard-index.toml` has
     not yet recorded the new `[[shard]]` entry — a discoverability-METADATA gap only:
     whole-corpus glob-based readers (`<stem>*.md`) still find the sealed file regardless of index
     membership, so no reader-visible data loss occurs. **Recovery (self-healing):** on the next
     dispatch attempt, the gate reconciles the index by scanning the filesystem for sealed-shard
     files matching the artifact's naming convention that are absent from the index, and appends
     the missing entries before evaluating any new trigger.
   - **All four steps succeed:** normal `Block` outcome (Postcondition 2), no error.

2. **CORRECTED (fix-burst pass-2, F-P2-002/F-P2-003, HIGH) — the observable outcome of an over-cap
   write is `HookResult::Block` with a specific, actionable, UNIFIED retry-instruction message —
   NOT a silent transparent redirect, and NOT a tool-divergent instruction.** The block reason
   text MUST include: the artifact name, the cap that was reached (in bytes), the fact that the
   current shard is now empty, and a SINGLE unified retry instruction (the SAME wording regardless
   of the original tool, since both branches now converge on "recompute against the current,
   post-roll state"):

   > "Shard `<artifact>` rotated (cap `<N>` bytes reached); the current shard is now empty. Retry
   > your write against the CURRENT (post-roll, empty) file — do not resubmit your original
   > payload unchanged: if you used `Edit` or `MultiEdit`, your `old_string` will no longer match
   > (the content it targeted is now in `<sealed-path>`) — reissue as a fresh `Write` containing
   > ONLY your new entry; if you used `Write`, recompute `content` to contain ONLY your new entry
   > (not your original full pre-roll payload, which reflects discarded state and will exceed the
   > cap again if resubmitted)."

   **This WITHDRAWS the v1.0/v1.1 "if you used `Write`, simply retry unchanged" wording**, which
   was UNSOUND under BOTH the withdrawn rename mechanism and the corrected per-tool
   `projected_size` formula (BC-1.18.005 Postcondition 3, F-P2-002): because the canonical file is
   now EMPTY after a roll, and because a blocked `Write`'s own `content` parameter was composed by
   the agent BEFORE the roll (typically by reading the OLD, over-cap file and appending one new
   entry), retrying that SAME `content` unchanged would resubmit content that is STILL over cap
   relative to the fresh empty shard (since `projected_size = len(content)` for `Write`, per the
   corrected formula, and `len(content)` has not shrunk) — producing a permanent block/retry
   deadlock, not a duplicate. This is a structural consequence of `HookResult`'s three-variant
   contract (Precondition 3): no design that assumes transparent write-redirection is
   implementable, so the postcondition describes the message an agent WILL see, not a hypothetical
   silent success.

3. **No shard is ever observed in an over-cap state by any downstream reader.** Because the seal
   happens before the block (Postcondition 1), and the blocked call is never applied to any file,
   the sealed shard's final size is always `<= shard_cap_bytes` (BC-1.18.005's cap, evaluated at
   seal time) and the new current shard starts at exactly 0 bytes. This is a structural guarantee,
   not a convention: it holds even if the agent never retries (the sealed shard is already
   durably capped; only the RETRY's content is lost if the agent abandons the operation).

4. **Shard index and sealed shard land in the same native-gate invocation.** The shard-index TOML
   write (Postcondition 1 step (d)) and the sealed-shard publish (step (b)) are both filesystem
   writes issued by the SAME PreToolUse invocation, before any `git add`/`git commit` occurs downstream.
   This makes TD-VSDD-053's single-commit-per-burst hold STRUCTURALLY for shard+index atomicity
   (not merely by state-manager discipline), because both writes are guaranteed to be present in
   the working tree before the next `git commit` regardless of which agent or skill issued the
   original tool call.

5. **Shard-index schema (one file per sharded mechanism-A artifact):**
   ```toml
   # .factory/cycles/<cycle>/<artifact-stem>.shard-index.toml
   schema_version = 1
   artifact_stem = "decision-log"
   current_shard = "decision-log.md"
   shard_cap_bytes = 49152           # calibrated per BC-1.18.005; locked at F4
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
   Every seal event appends exactly one new `[[shard]]` table entry with `seq` incrementing
   monotonically from 1, `path` naming the sealed file, `sealed_at` in UTC ISO-8601, and
   `bytes_at_seal` recording the sealed shard's exact final byte count (always `<= shard_cap_bytes`
   per Postcondition 3).

6. **Stable-current-filename addressing is a consequence of this BC's seal mechanism, not a
   separate lookup step.** Because the seal PUBLISHES a copy of the old content as a NEW sealed
   file and ATOMICALLY REPLACES the canonical file's content with empty (CORRECTED, F-P2-003 —
   never renames the canonical path away), any shard-unaware reader or validator that opens the
   canonical filename always sees the current/latest shard, with zero code change required on the
   reader's part. Whole-corpus readers use the glob `<stem>*.md` (e.g. `decision-log*.md`), which
   sorts sealed shards ascending before the current file — the deciding byte-comparison one
   position past the shared `<stem>` prefix is a digit (`0`-`9`, from a sealed shard's `.NNNN.md`
   suffix) vs. `m` (from the current file's own `.md` suffix); since every digit byte is
   numerically less than `m`, sealed shards sort first (ADR-051 §Decision 3 fix-burst-corrected
   sort-order rationale — the "current file sorts last" conclusion is unchanged) — no
   special-casing needed in a `sort`-fed pipeline.

## Invariants

1. **`HookResult::Block` is the ONLY variant this BC's gate returns on an over-cap write.** No
   version of this check may return `Continue` after determining `projected_size >
   shard_cap_bytes` (that would silently permit an over-cap write, violating BC-1.18.005
   Postcondition 3's contract), and no version may return `Error` for a normal (non-crash)
   over-cap condition (a normal rotation is not an error condition — it is the mechanism working
   as designed).

2. **CORRECTED (fix-burst pass-2, F-P2-003, HIGH) — the read-publish-truncate-publish sequence is
   never reordered.** Publishing the shard index before the sealed shard is published, or
   truncating the canonical file's content before the sealed shard's content is durably published,
   would risk a window where NEITHER a valid sealed copy NOR a valid canonical-with-full-content
   state exists for the pre-roll content. The four sub-steps in Postcondition 1 ((a) read, (b)
   publish sealed shard, (c) atomic-truncate canonical, (d) publish index) execute in the stated
   order; each of (b), (c), and (d) is its own independent `write_atomic` temp-file-then-rename
   call — there is no OS-level atomicity spanning multiple steps, which is exactly why
   Postcondition 1's per-step partial-failure codes (`E-SHD-001`/`E-SHD-006`/`E-SHD-007`) exist:
   a crash between any two steps is a distinct, named, self-healing-recoverable state, never an
   unspecified one.

3. **The canonical filename never moves.** At every observable point in time — before a roll,
   during a roll's execution, and after a roll completes — the canonical filename (e.g.
   `decision-log.md`) refers to SOME valid file: either the not-yet-full current shard (before a
   roll), or the fresh empty current shard (after a roll completes). It is NEVER renamed away to a
   sealed name; only its CONTENT is replaced, via an atomic `write_atomic` rename-ONTO-existing-
   destination (Postcondition 1 step (c)) — never a rename-OUT-of the canonical path. This
   invariant is now structurally, not merely conventionally, true: the withdrawn v1.0/v1.1
   rename-away mechanism could not satisfy it (see Description's ENOENT-window analysis); the
   corrected copy-then-atomic-truncate mechanism can, because `rename()` onto an EXISTING
   destination never leaves that destination absent.

4. **Retry-instruction wording is a single, fixed template — never divergent per original tool
   name.** CORRECTED (fix-burst pass-2, F-P2-002): the withdrawn v1.0/v1.1 design chose between
   two DIFFERENT wordings based on the original blocked tool's name (a `Write`-specific "simply
   retry unchanged" branch that was later found unsound). The corrected design (Postcondition 2)
   uses ONE unified message template that names both tool cases within the SAME text — the
   template itself never varies, and its content is never randomized, never omitted, and never
   generic ("write failed, try again" without the specific per-tool guidance embedded in the
   unified template is insufficient).

5. **NEW (fix-burst pass-2, F-P2-005, MEDIUM) — the append-only-tail assumption is explicit,
   never silently relied upon.** This BC's gate has NO semantic understanding of WHERE within a
   file an `Edit`/`Write`/`MultiEdit` lands — it computes `projected_size` from a pure byte-delta/
   length formula (BC-1.18.005 Postcondition 3) only. The roll+block+retry wording (Postcondition
   2) is phrased for the common case this gate exists to serve: a pure APPEND of one new record at
   the file's end. The four mechanism-A artifacts are, by construction, POLICY-1
   (`append_only_numbering`) governed append-only records — POLICY-1 already forbids renumbering
   or rewriting historical entries, so legitimate `Edit`/`MultiEdit` mutations against these
   artifacts are, by that SAME policy, already expected to be either (a) a pure append of a
   brand-new record at file end, or (b) a narrow amendment to a STILL-MUTABLE, recently-added
   record near the tail — never an edit to arbitrarily old, already-sealed, or deep-mid-file
   historical content. See EC-012 below for the caller-responsibility failure mode when this
   assumption is violated, and its sanctioned escape hatch.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single appended block exceeds shard cap (story's own EC-001) | Roll to new shard BEFORE the write is applied (Postcondition 1); the write itself is blocked (Postcondition 2), never partially applied to either the sealed or fresh shard |
| EC-002 | Agent issues `Edit` against a just-rolled artifact without reading the block message | `Edit`'s `old_string` will not match the now-empty fresh shard — the `Edit` tool call itself fails with a standard "old_string not found" error; this is the exact confusing-failure class Postcondition 2's explicit retry wording exists to prevent for the FIRST block, but a repeated blind-retry `Edit` after that still fails at the tool layer (this BC's contract covers the dispatcher's OWN block message, not enforcement of agent compliance) |
| EC-003 | Postcondition 1 step (a)-(b) fails (e.g., filesystem permission error, disk full) before the sealed shard is durably published | Fail-loud: the gate returns `HookResult::Error` (`E-SHD-001`, description refined to "shard-seal-write failure"), not `Block` and not `Continue` — the canonical file is left in its exact pre-roll state; see EC-005 of the S-25.02 story draft ("Shard index unavailable or corrupt") for the sibling dispatch-blocked case |
| EC-004 | Concurrent dispatch attempts to write to the same artifact's shard (two agents/sessions racing) | Atomic temp+rename for the index publish ensures no torn shard-index write; TD-VSDD-053 single-commit-per-burst and the project's factory-lock discipline (ADR-025) prevent concurrent factory-artifacts commits from landing interleaved |
| EC-005 | `MultiEdit` with one edit block that alone exceeds the cap even against a freshly-rolled (0-byte) shard | Roll triggers as normal (BC-1.18.005 EC-004: current_shard_bytes=0), but if `payload_bytes` alone exceeds `shard_cap_bytes - MAX_SINGLE_RECORD_BYTES`'s margin, this indicates a single record larger than the calibrated `MAX_SINGLE_RECORD_BYTES` assumption — the write still blocks per Postcondition 2, and the retry-then-still-too-large condition surfaces to the agent as a repeated block, which is the correct fail-loud signal that the record itself needs to be split by the caller, not silently truncated |
| EC-006 | `/compact-state`'s own `Edit`/`Write` calls against a sharded artifact trigger a mid-extraction roll | Gets shard-awareness for free (ADR-051 Decision 5) — the skill receives the same `Block`-with-retry-instruction message any other caller would; no amendment to the gate mechanism itself is required for this (a small documentation-only note to `compact-state/SKILL.md`'s own retry-loop guidance is recommended, per ADR-051 Decision 5, but is out of this BC's and this burst's write scope — SKILL.md is not a `.factory/specs/` artifact) |
| EC-010 (fix-burst pass-2, F-P2-004) | Postcondition 1 step (c) (atomic-truncate) fails AFTER step (b) (sealed-shard publish) succeeded | `E-SHD-006`: sealed shard durably exists AND canonical file still holds the same pre-roll content (transient, detectable duplicate-content state, not data loss); self-healing recovery resumes from step (c) alone on the next dispatch attempt (Postcondition 1) |
| EC-011 (fix-burst pass-2, F-P2-004) | Postcondition 1 step (d) (index publish) fails AFTER step (c) (atomic-truncate) succeeded | `E-SHD-007`: canonical file is correctly fresh/empty and the sealed shard exists correctly on disk, but the shard-index has not yet recorded the new `[[shard]]` entry (discoverability-metadata gap only, no reader-visible data loss); self-healing index reconciliation runs on the next dispatch attempt (Postcondition 1) |
| EC-012 (fix-burst pass-2, F-P2-005) | An `Edit`/`MultiEdit`'s target content was ALREADY relocated to a SEALED shard by an EARLIER roll (a policy-violating attempt to amend deep-historical content, or a caller operating on stale in-memory state) | The retry-instruction text ("reissue as a fresh `Write` containing only your new entry") is INAPPLICABLE — there is no "new entry" to reissue against the canonical file, because that content no longer lives there. **Sanctioned escape hatch:** a sealed shard file (`<stem>.<seq:04>.md`) is an ORDINARY file that does NOT match any `[[shard]]` config entry's canonical-path pattern (BC-1.18.005 Postcondition 1's zero-cost bypass for unmatched paths) — it is entirely UNGATED by this BC's gate, and a caller with a genuine, policy-sanctioned need to touch historical content addresses the sealed file DIRECTLY by its own on-disk filename, exactly as it would edit any other ordinary file. This gate makes no attempt to detect, permit, or forbid such an edit — that is POLICY-1's concern (enforced at the `consistency-validator`/adversary-prompt agent level), entirely orthogonal to this gate's byte-size-triggered rotation concern. |
| EC-013 (fix-burst pass-2, F-P2-005) | A net-positive `Edit`/`MultiEdit` targets a STILL-MUTABLE tail record (e.g., a same-burst typo fix to an entry not yet sealed away) and pushes `projected_size` over cap | Triggers the SAME generic roll+block+retry sequence as any other over-cap write (Postcondition 1/2) — Invariant 5's append-only-tail assumption is not violated by this case (the target is still-mutable tail content, not deep-historical content); EC-002's `old_string`-mismatch failure mode applies identically |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| **CORRECTED (fix-burst pass-2, F-P2-003/F-P2-002).** `Write` to `decision-log.md`, current shard 45,000 bytes, `content` length 5,000 bytes, cap 49,152 | Publish `decision-log.0001.md` (a NEW file, byte-copy of the 45,000-byte pre-roll content); atomically replace `decision-log.md`'s content with empty (0 bytes, canonical path never absent); publish index `[[shard]] seq=1 path="decision-log.0001.md" bytes_at_seal=45000`; return `Block{reason: "Shard decision-log rotated (cap 49152 bytes reached)... recompute content to contain ONLY your new entry..."}` (unified wording) | happy-path |
| **CORRECTED (fix-burst pass-2, F-P2-002).** `Edit` to `burst-log.md` causing an over-cap projection | Roll executes identically to the `Write` case (copy-then-atomic-truncate); `Block` reason text uses the SAME unified template as the `Write` case (naming both tool branches within one message, per Postcondition 2) | happy-path |
| Second roll on the same artifact within the same cycle | `[[shard]]` gains a SECOND entry with `seq=2`; `seq=1`'s entry is untouched (append-only index) | edge-case |
| Postcondition 1 step (a)-(b) fails (simulated permission error before sealed shard is published) | `HookResult::Error` (`E-SHD-001`); no shard-index entry published; canonical file left in its pre-roll state | error |
| **NEW (fix-burst pass-2, F-P2-004).** Postcondition 1 step (c) fails after step (b) succeeded (simulated crash between sealed-shard publish and canonical truncate) | `E-SHD-006`: sealed shard `decision-log.0001.md` exists AND `decision-log.md` still holds the same 45,000-byte content; next dispatch attempt detects the byte-identical duplicate and resumes from step (c) alone (truncate + index publish only) | error |
| **NEW (fix-burst pass-2, F-P2-004).** Postcondition 1 step (d) fails after step (c) succeeded (simulated crash between canonical truncate and index publish) | `E-SHD-007`: `decision-log.md` is correctly empty and `decision-log.0001.md` exists on disk, but the shard-index has no `[[shard]] seq=1` entry; next dispatch attempt scans for un-indexed sealed shards and appends the missing entry before evaluating any new trigger | error |
| Whole-corpus `grep -n "D-1234" decision-log*.md` after 2 rolls | Glob matches `decision-log.0001.md`, `decision-log.0002.md`, `decision-log.md` in that lexicographic (and chronological) order | happy-path |
| **NEW (fix-burst pass-2, F-P2-005).** `Edit` targets content already relocated to `decision-log.0001.md` by an earlier roll | Retry-instruction text is inapplicable against the (now-unrelated) canonical file; caller addresses `decision-log.0001.md` directly by filename — ungated, since it matches no `[[shard]]` config entry | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-118 | Seal-then-block ordering invariant — the shard-index TOML always contains the seal entry for a given roll BEFORE (or atomically with) the corresponding `HookResult::Block` is observed by the caller | integration test (dispatcher harness: assert index file content immediately upon receiving the Block result) |
| VP-119 | No-over-cap invariant — no sealed shard file's byte size, sampled at any point after this BC's gate executes, ever exceeds its recorded `shard_cap_bytes` at seal time | proptest (arbitrary sequence of writes against a simulated artifact; property: every sealed shard's `bytes_at_seal <= shard_cap_bytes`) |
| VP-120 | Retry-wording determinism — the block reason's retry instruction is the SAME fixed unified template regardless of the original tool name (CORRECTED, F-P2-002: no longer a per-tool-name choice between two divergent wordings) | unit test (table-driven over both tool-name classes, asserting identical template with tool-specific guidance embedded within it) |
| VP-119 | Stable-current-filename invariant — the canonical filename is never itself renamed to a sealed name across any sequence of rolls; `stat(canonical_path)` always succeeds (CORRECTED, F-P2-003: verified against the copy-then-atomic-truncate mechanism, which structurally cannot vacate the canonical path, rather than the withdrawn rename-away mechanism) | proptest (arbitrary roll sequence; property: `stat(canonical_path)` always succeeds and is never the sealed inode from a prior roll) |
| VP-118 | **NEW (fix-burst pass-2, F-P2-004).** Partial-failure self-healing invariant — a simulated crash between any two of Postcondition 1's four steps resolves, on the NEXT dispatch attempt, to exactly the named recovery for that crash point (`E-SHD-001` full-restart; `E-SHD-006` resume-from-truncate; `E-SHD-007` index-reconciliation), never an unspecified or manual-intervention-required state | fault-injection / integration test (simulated crash at each of the three inter-step boundaries; assert post-recovery state matches the named recovery) |
| VP-138 | Truncate-after-seal self-heal invariant (`E-SHD-006`) — a crash between Postcondition 1 step (b) (sealed-shard publish) and step (c) (atomic-truncate) resolves, on the NEXT dispatch attempt, to a byte-identity check against the sealed shard followed by resume-from-step-(c)-only recovery (truncate + index publish only; the already-correct sealed shard is never rewritten), per EC-010 | integration test (fault-injection across two dispatches: simulate a crash between steps (b) and (c); assert post-recovery state is exactly one sealed shard, one index entry, and an empty canonical file) |
| VP-139 | Index-after-truncate self-heal invariant (`E-SHD-007`) — a crash between Postcondition 1 step (c) (atomic-truncate) and step (d) (index publish) resolves, on the NEXT dispatch attempt, to a filesystem scan for un-indexed sealed shards followed by an append-only reconciliation of the missing `[[shard]]` entry, per EC-011 and Postcondition 5's schema | integration test (fault-injection across two dispatches: simulate a crash between steps (c) and (d); assert the reconciled index gains exactly the missing entry, existing entries untouched, idempotent on repeat) |

## Related BCs

- BC-1.18.005 — owns the size-trigger and cap formula that determines when this BC's roll fires (depends on)
- BC-1.18.007 — retention/compaction policy consumes this BC's shard-index schema to identify archival candidates (composes with)
- BC-1.18.008 — the one-time backfill-split reuses this BC's exact seal+create+index-publish sequence, applied retroactively (composes with)
- BC-1.18.009 — mechanism B1 (BC-INDEX changelog rotation) is triggered by the SAME native gate this BC defines, with a different artifact-shape case (composes with)
- BC-1.18.010 — mechanism B2 (BC-INDEX per-subsystem sharding) is triggered by the SAME native gate, keyed by a manifest instead of the stable-filename trick (composes with)
- BC-1.18.012 — the governed one-time B1 changelog backfill migration reuses this BC's staging discipline pattern (temp-file-then-rename atomic-write) analogous to BC-1.18.008's relationship to this BC (related to)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` (new module) — publish-sealed-shard+atomic-truncate-canonical+index-publish sequence (CORRECTED, F-P2-003: copy-then-atomic-truncate, not rename-away), `HookResult::Block` construction with the unified retry-instruction text
- `crates/hook-sdk/src/result.rs` — `HookResult` enum (`Continue`/`Block { reason }`/`Error { message }`), the structural contract motivating block-and-retry over transparent redirection
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `write_indeterminate_marker`'s temp-file-then-rename atomic-write pattern, reused for shard-index publication
- `crates/last-amended-migrate/src/atomic_write.rs` — `write_atomic`, the alternative existing atomic-write primitive this BC's implementation may reuse instead of duplicating `indeterminate_marker.rs`'s

## SDK Grounding Evidence

Literal stable-anchor greps substantiating this BC's external-artifact claims (POLICY 5;
no `grep -n` / no file:line citations per TD-VSDD-091):

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

Confirms the exact three-variant `HookResult` contract this BC's Precondition 3 and Invariant 1
depend on — no fourth "redirect" variant exists, grounding the "transparent redirection is
structurally impossible" claim in the Description.

```
$ grep -oE "^pub fn write_indeterminate_marker|^pub fn block_if_marker_check|^pub fn should_write_marker" crates/factory-dispatcher/src/indeterminate_marker.rs
pub fn block_if_marker_check
pub fn should_write_marker
pub fn write_indeterminate_marker
```

Confirms `write_indeterminate_marker`'s existence as the temp-file-then-rename atomic-write
precedent this BC's Postcondition 1 steps (b)-(d) and Architecture Anchors cite.

```
$ grep -oE "^pub fn write_atomic" crates/last-amended-migrate/src/atomic_write.rs
pub fn write_atomic
```

Confirms the alternative atomic-write primitive (`write_atomic`) this BC's Architecture Anchors
name as a reuse candidate.

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- VP-118, VP-119, VP-120 — allocated by formal-verifier (S-25.02 F2 verification-property extension burst; VP-INDEX v3.02). VP-118 (integration; publish-sealed-shard→atomic-truncate-canonical→atomic-index-publish before Block + same-invocation atomicity + NEW partial-failure self-healing invariant per fix-burst pass-2), VP-119 (proptest; no-over-cap + stable-current-filename — re-verified against the corrected copy-then-atomic-truncate mechanism), VP-120 (unit-test; retry-wording determinism — re-verified as a single unified template, not a per-tool-name choice — + fail-loud shard-seal-write error E-SHD-001 + NEW E-SHD-006/E-SHD-007 partial-failure codes). Formal-verifier should review VP-118/119/120 bodies against this fix-burst's corrected mechanics (copy-then-truncate instead of rename-then-create; unified retry wording; staged 4-step sequence) — not yet actioned in this burst.
- VP-138, VP-139 — allocated by formal-verifier (S-25.02 F2 verification-property fix-burst pass-2; VP-INDEX v3.04, F-P2-004 partial-failure-code symmetry: every E-SHD code now has a VP leg). VP-138 (integration; Postcondition 1 step (c)/EC-010/Invariant 2/Invariant 3 — E-SHD-006 self-healing resume-from-truncate), VP-139 (integration; Postcondition 1 step (d)/EC-011/Postcondition 5 — E-SHD-007 self-healing index reconciliation). Back-references added S-25.02 F2 residual-cleanup micro-burst (formal-verifier's VP-138/VP-139 bodies already cited this BC in `source_bc`; this BC's own Verification Properties table and VP Anchors list did not yet cite them back — gap closed here, reference-only, no behavior change).

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies CAP-043's roll-before-write mechanics: "performs a roll-before-write (publish a sealed shard copy as a new file, then atomically replace the canonical file's content with empty, then atomically publish the updated shard index) and returns `HookResult::Block` with an explicit, actionable retry instruction (transparent write-redirection is not implementable under `HookResult`'s ... contract)." (CORRECTED, fix-burst pass-2, F-P2-003, from the withdrawn "seal the current shard by rename, create a fresh empty current file" wording). |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` roll/block sequence) |
| ADR | ADR-051 §Decision 1 (block-and-retry mechanism, full algorithm, v1.2 per-tool `projected_size` correction); §Decision 3 (stable-current-filename addressing, v1.2 copy-then-atomic-truncate correction); §Decision 4 (shard-index schema); §Decision 11 (staged partial-failure sequence + E-SHD-006/007, fix-burst addition); §Decision 12 (append-only-tail assumption + sealed-shard escape hatch, fix-burst addition); §Context (`HookResult`'s three-variant SDK constraint) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.2 | 2026-09-05 | product-owner | Fix-burst amendment (adversary pass-2 findings F-P2-003 HIGH + F-P2-002 HIGH + F-P2-004 MEDIUM + F-P2-005 MEDIUM, ADR-051 v1.2 Decisions 1/3/11/12): (1) REWROTE Postcondition 1(a)/Invariant 2/Invariant 3 from the WITHDRAWN rename-away seal mechanism (which opened a real ENOENT window on the canonical path) to the CORRECTED copy-then-atomic-truncate-in-place mechanism — publish the sealed shard as a new file, then atomically replace the canonical file's content with empty via `write_atomic`'s temp-file-then-rename primitive; the canonical path is never absent. (2) REWROTE Postcondition 2/Invariant 4 to a SINGLE UNIFIED retry-instruction template, withdrawing the divergent `Write`-"simply retry unchanged" wording (unsound: could permanently deadlock a blocked `Write` whose stale pre-roll `content` remains over cap under the corrected `projected_size = len(content)` formula). (3) ADDED the staged 4-step per-write roll sequence (read/publish-sealed/atomic-truncate/publish-index) with two NEW partial-failure error codes `E-SHD-006` (seal published, canonical not yet truncated — self-healing resume-from-truncate) and `E-SHD-007` (canonical truncated, index not yet updated — self-healing index reconciliation), plus new EC-010/EC-011 and a new VP-118 fault-injection property. (4) ADDED Invariant 5 (append-only-tail assumption, explicit) and two new edge cases EC-012 (sealed-shard direct-edit escape hatch for already-relocated content) and EC-013 (still-mutable tail edit, unaffected by Invariant 5). Updated Canonical Test Vectors, Architecture Anchors, SDK Grounding cross-references, VP Anchors, and Traceability's Capability Anchor Justification quote and ADR citation accordingly. Added BC-1.18.012 to Related BCs. |
| 1.1 | 2026-09-05 | product-owner | Fix-burst amendment (F-S2502-F2-007, POLICY 5): added `## SDK Grounding Evidence` section with literal stable-anchor grep output for `HookResult`'s three-variant enum, `write_indeterminate_marker`/`block_if_marker_check`, and `write_atomic`. No postcondition/invariant/VP content change — this BC's contract was confirmed unaffected by the sibling BC-1.18.009 BLOCKER fix (architect: "No change required," F2 architecture-delta §4a). |
| 1.0 | 2026-09-05 | product-owner | Initial creation. F2 spec-evolution burst, S-25.02 activation. Encodes the CORRECTED block-and-retry roll semantics (per ADR-051's finding that `HookResult`'s Continue/Block/Error contract forbids transparent write-redirection) rather than a transparent-redirect fiction; shard-index schema; stable-current-filename addressing as a structural consequence of the seal mechanism. CAP-043 capability anchor. ADR-051 §D1/§D3/§D4 citations. |
