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
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.005.md
  - crates/hook-sdk/src/result.rs
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md
input-hash: "c344290"
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

When BC-1.18.005's size-trigger fires, the dispatcher performs the roll (seal the current shard by
rename, create a fresh empty current file, atomically publish the updated shard index) and THEN
returns `HookResult::Block` with an explicit, actionable retry instruction — never a silent
transparent redirect, which `HookResult`'s three-variant contract (`Continue`/`Block { reason }`/
`Error { message }`) makes structurally impossible. The blocked call is never applied, so no shard
is ever observed over cap by any downstream reader, and the sealed shard plus its index update land
in the SAME native-gate invocation, guaranteeing they are staged in the same subsequent
factory-artifacts commit (TD-VSDD-053 alignment).

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

1. **The roll sequence executes BEFORE the block is returned, in this exact order:** (a) seal the
   current shard by renaming it from its canonical name (e.g. `decision-log.md`) to its sealed name
   (`<stem>.<seq:04>.md`, e.g. `decision-log.0001.md`); (b) create a fresh, empty file at the
   canonical name; (c) atomically publish the updated shard-index TOML (temp-file-then-rename, the
   same pattern already established by `write_indeterminate_marker` and `write_atomic` — no new
   atomic-write primitive is introduced). Only after (a)-(c) complete does the gate return
   `HookResult::Block`.

2. **The observable outcome of an over-cap write is `HookResult::Block` with a specific,
   actionable retry-instruction message — NOT a silent transparent redirect.** The block reason
   text MUST include: the artifact name, the cap that was reached (in bytes), the fact that the
   current shard is now empty, and an explicit retry instruction distinguishing the `Edit` case
   from the `Write` case:
   - If the original call was `Edit` or `MultiEdit`: "reissue as a fresh `Write` containing only
     your new entry (the `old_string`/`new_string` pair(s) will no longer match against the
     now-empty current shard)."
   - If the original call was `Write`: "simply retry unchanged" (a `Write` call's `content`
     parameter is self-contained and remains valid against the fresh empty shard).
   This is a structural consequence of `HookResult`'s three-variant contract (Precondition 3): no
   design that assumes transparent write-redirection is implementable, so the postcondition
   describes the message an agent WILL see, not a hypothetical silent success.

3. **No shard is ever observed in an over-cap state by any downstream reader.** Because the seal
   happens before the block (Postcondition 1), and the blocked call is never applied to any file,
   the sealed shard's final size is always `<= shard_cap_bytes` (BC-1.18.005's cap, evaluated at
   seal time) and the new current shard starts at exactly 0 bytes. This is a structural guarantee,
   not a convention: it holds even if the agent never retries (the sealed shard is already
   durably capped; only the RETRY's content is lost if the agent abandons the operation).

4. **Shard index and sealed shard land in the same native-gate invocation.** The shard-index TOML
   write (Postcondition 1 step (c)) and the shard-seal rename (step (a)) are both filesystem writes
   issued by the SAME PreToolUse invocation, before any `git add`/`git commit` occurs downstream.
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
   separate lookup step.** Because the seal RENAMES the old content away and CREATES a fresh file
   at the canonical name (never the reverse), any shard-unaware reader or validator that opens the
   canonical filename always sees the current/latest shard, with zero code change required on the
   reader's part. Whole-corpus readers use the glob `<stem>*.md` (e.g. `decision-log*.md`), which
   sorts sealed shards ascending before the current file, since `.` (0x2E) sorts before `0`-`9`
   (0x30+) in the shared `<stem>.` prefix — no special-casing needed in a `sort`-fed pipeline.

## Invariants

1. **`HookResult::Block` is the ONLY variant this BC's gate returns on an over-cap write.** No
   version of this check may return `Continue` after determining `projected_size >
   shard_cap_bytes` (that would silently permit an over-cap write, violating BC-1.18.005
   Postcondition 3's contract), and no version may return `Error` for a normal (non-crash)
   over-cap condition (a normal rotation is not an error condition — it is the mechanism working
   as designed).

2. **The seal-then-create-then-publish sequence is never reordered.** Publishing the shard index
   before the seal rename completes, or creating the fresh current file before the old content is
   sealed away, would risk a window where the canonical filename holds neither the full old
   content nor a valid fresh-empty state. The three sub-steps in Postcondition 1 execute in the
   stated order, and steps (a)+(b) are a single filesystem rename operation where the underlying
   OS supports it (POSIX `rename()` on the same filesystem is atomic for the seal step itself;
   the fresh-file creation is a separate subsequent write).

3. **The canonical filename never moves.** At every observable point in time — before a roll,
   during a roll's execution, and after a roll completes — the canonical filename (e.g.
   `decision-log.md`) refers to SOME valid file: either the not-yet-full current shard (before a
   roll), or the fresh empty current shard (after a roll completes). It is never renamed away to a
   sealed name; only its CONTENT is replaced by the seal+create sequence.

4. **Retry-instruction wording is deterministic per original tool name.** The choice between the
   `Edit`/`MultiEdit` retry wording and the `Write` retry wording (Postcondition 2) is a pure
   function of the ORIGINAL blocked tool call's name — never randomized, never omitted, never
   generic ("write failed, try again" without the specific `Edit`-vs-`Write` guidance is
   insufficient).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single appended block exceeds shard cap (story's own EC-001) | Roll to new shard BEFORE the write is applied (Postcondition 1); the write itself is blocked (Postcondition 2), never partially applied to either the sealed or fresh shard |
| EC-002 | Agent issues `Edit` against a just-rolled artifact without reading the block message | `Edit`'s `old_string` will not match the now-empty fresh shard — the `Edit` tool call itself fails with a standard "old_string not found" error; this is the exact confusing-failure class Postcondition 2's explicit retry wording exists to prevent for the FIRST block, but a repeated blind-retry `Edit` after that still fails at the tool layer (this BC's contract covers the dispatcher's OWN block message, not enforcement of agent compliance) |
| EC-003 | The seal rename fails (e.g., filesystem permission error, disk full) mid-roll | Fail-loud: the gate returns `HookResult::Error`, not `Block` and not `Continue` — a roll that cannot complete must not silently permit an unbounded write nor silently pretend the roll succeeded; see EC-005 of the S-25.02 story draft ("Shard index unavailable or corrupt") for the sibling dispatch-blocked case |
| EC-004 | Concurrent dispatch attempts to write to the same artifact's shard (two agents/sessions racing) | Atomic temp+rename for the index publish ensures no torn shard-index write; TD-VSDD-053 single-commit-per-burst and the project's factory-lock discipline (ADR-025) prevent concurrent factory-artifacts commits from landing interleaved |
| EC-005 | `MultiEdit` with one edit block that alone exceeds the cap even against a freshly-rolled (0-byte) shard | Roll triggers as normal (BC-1.18.005 EC-004: current_shard_bytes=0), but if `payload_bytes` alone exceeds `shard_cap_bytes - MAX_SINGLE_RECORD_BYTES`'s margin, this indicates a single record larger than the calibrated `MAX_SINGLE_RECORD_BYTES` assumption — the write still blocks per Postcondition 2, and the retry-then-still-too-large condition surfaces to the agent as a repeated block, which is the correct fail-loud signal that the record itself needs to be split by the caller, not silently truncated |
| EC-006 | `/compact-state`'s own `Edit`/`Write` calls against a sharded artifact trigger a mid-extraction roll | Gets shard-awareness for free (ADR-051 Decision 5) — the skill receives the same `Block`-with-retry-instruction message any other caller would; no amendment to the gate mechanism itself is required for this (a small documentation-only note to `compact-state/SKILL.md`'s own retry-loop guidance is recommended, per ADR-051 Decision 5, but is out of this BC's and this burst's write scope — SKILL.md is not a `.factory/specs/` artifact) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Write` to `decision-log.md`, current shard 45,000 bytes, payload 5,000 bytes, cap 49,152 | Seal `decision-log.md`→`decision-log.0001.md` (45,000 bytes); create fresh `decision-log.md` (0 bytes); publish index `[[shard]] seq=1 path="decision-log.0001.md" bytes_at_seal=45000`; return `Block{reason: "...Write: simply retry unchanged."}` | happy-path |
| `Edit` to `burst-log.md` causing an over-cap projection | Roll executes identically to the `Write` case; `Block` reason text uses the `Edit`/`MultiEdit`-specific retry wording ("reissue as a fresh `Write`...") | happy-path |
| Second roll on the same artifact within the same cycle | `[[shard]]` gains a SECOND entry with `seq=2`; `seq=1`'s entry is untouched (append-only index) | edge-case |
| Seal rename fails (simulated permission error) | `HookResult::Error`, not `Block`; no shard-index entry published; canonical file left in its pre-roll state | error |
| Whole-corpus `grep -n "D-1234" decision-log*.md` after 2 rolls | Glob matches `decision-log.0001.md`, `decision-log.0002.md`, `decision-log.md` in that lexicographic (and chronological) order | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-118 | Seal-then-block ordering invariant — the shard-index TOML always contains the seal entry for a given roll BEFORE (or atomically with) the corresponding `HookResult::Block` is observed by the caller | integration test (dispatcher harness: assert index file content immediately upon receiving the Block result) |
| VP-119 | No-over-cap invariant — no sealed shard file's byte size, sampled at any point after this BC's gate executes, ever exceeds its recorded `shard_cap_bytes` at seal time | proptest (arbitrary sequence of writes against a simulated artifact; property: every sealed shard's `bytes_at_seal <= shard_cap_bytes`) |
| VP-120 | Retry-wording determinism — the block reason's retry instruction is a pure function of the original tool name (`Edit`/`MultiEdit` vs `Write`) | unit test (table-driven over both tool-name classes) |
| VP-119 | Stable-current-filename invariant — the canonical filename is never itself renamed to a sealed name across any sequence of rolls | proptest (arbitrary roll sequence; property: `stat(canonical_path)` always succeeds and is never the sealed inode from a prior roll) |

## Related BCs

- BC-1.18.005 — owns the size-trigger and cap formula that determines when this BC's roll fires (depends on)
- BC-1.18.007 — retention/compaction policy consumes this BC's shard-index schema to identify archival candidates (composes with)
- BC-1.18.008 — the one-time backfill-split reuses this BC's exact seal+create+index-publish sequence, applied retroactively (composes with)
- BC-1.18.009 — mechanism B1 (BC-INDEX changelog rotation) is triggered by the SAME native gate this BC defines, with a different artifact-shape case (composes with)
- BC-1.18.010 — mechanism B2 (BC-INDEX per-subsystem sharding) is triggered by the SAME native gate, keyed by a manifest instead of the stable-filename trick (composes with)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` (new module) — seal+create+index-publish sequence, `HookResult::Block` construction with retry-instruction text
- `crates/hook-sdk/src/result.rs` — `HookResult` enum (`Continue`/`Block { reason }`/`Error { message }`), the structural contract motivating block-and-retry over transparent redirection
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `write_indeterminate_marker`'s temp-file-then-rename atomic-write pattern, reused for shard-index publication
- `crates/last-amended-migrate/src/atomic_write.rs` — `write_atomic`, the alternative existing atomic-write primitive this BC's implementation may reuse instead of duplicating `indeterminate_marker.rs`'s

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- VP-118, VP-119, VP-120 — allocated by formal-verifier (S-25.02 F2 verification-property extension burst; VP-INDEX v3.02). VP-118 (integration; seal→create→atomic-index-publish before Block + same-invocation atomicity), VP-119 (proptest; no-over-cap + stable-current-filename), VP-120 (unit-test; retry-wording determinism + fail-loud seal-rename error E-SHD-001).

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies CAP-043's roll-before-write mechanics: "performs a roll-before-write (seal the current shard by rename, create a fresh empty current file, atomically publish the updated shard index) and returns `HookResult::Block` with an explicit, actionable retry instruction (transparent write-redirection is not implementable under `HookResult`'s ... contract)." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` roll/block sequence) |
| ADR | ADR-051 §Decision 1 (block-and-retry mechanism, full algorithm); §Decision 3 (stable-current-filename addressing); §Decision 4 (shard-index schema); §Context (`HookResult`'s three-variant SDK constraint) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-09-05 | product-owner | Initial creation. F2 spec-evolution burst, S-25.02 activation. Encodes the CORRECTED block-and-retry roll semantics (per ADR-051's finding that `HookResult`'s Continue/Block/Error contract forbids transparent write-redirection) rather than a transparent-redirect fiction; shard-index schema; stable-current-filename addressing as a structural consequence of the seal mechanism. CAP-043 capability anchor. ADR-051 §D1/§D3/§D4 citations. |
