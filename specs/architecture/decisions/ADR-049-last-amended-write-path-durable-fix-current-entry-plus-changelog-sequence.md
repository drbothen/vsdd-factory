---
document_type: adr
adr_id: ADR-049
status: accepted
date: 2026-09-02
subsystems_affected: [SS-04, SS-05, SS-06, SS-10]
supersedes: null
superseded_by: null
---

<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->

# ADR-049: `last_amended` Write-Path Durable Fix — Current-Entry-Only Scalar Plus Existing `changelog:` Sequence

## Context

D-1149 (`.factory/cycles/v1.0-brownfield-backfill/decision-log.md`, 2026-09-02) performed a
one-time, human-authorized POL-3/TD-FACTORY-HOOK-BYPASS-001 exception to slim the
`last_amended` frontmatter scalar on five files — `STORY-INDEX.md` (323,499 chars before
surgery), `BC-INDEX.md`, `ARCH-INDEX.md`, `VP-INDEX.md`, and `STATE.md` — into a short
current-entry-plus-pointer form, moving the removed tail verbatim into five
`*-amendment-history.md` sidecars. D-1149's own lesson (`L-BB-D1149`) is explicit that this
is a mitigation, not a cure: the root cause is that every state-manager burst PREPENDS the new
entry and wraps the ENTIRE prior value inline as a single quoted-string `[Prior: ...]` chain, so
the field grows without bound across a long-running cycle. This produced a 329,499-char single
physical line that exhausted the bash-adapter WASM validators' fuel budget (743
fuel-timeouts/day). S-15.03 (`.factory/stories/S-15.03-index-cite-refresh-hook.md`
§Scope Extension) exists to change the WRITE PATH itself so a mega-line can never re-form.

This ADR executes S-15.03 Phase A: AC-002's mandatory validator-compatibility audit, run BEFORE
any write-path change ships (per the story's own gate). The audit covered every reader of these
five files' frontmatter across the dispatcher/hook-plugin/skill surface:

- `crates/hook-plugins/validate-cross-site-correspondence/src/frontmatter.rs`
  (`extract_frontmatter_field`, `collect_block_scalar_body`) — the shared scalar/block-scalar
  extractor.
- `crates/hook-plugins/validate-cross-site-correspondence/src/arm_e.rs`
  (`extract_last_amended_outer_version`, E1; `strip_date_annotation`, E2) and
  `crates/hook-plugins/validate-cross-site-correspondence/src/dispatch.rs`
  (`is_frontmatter_parity_target`, `is_bc_file`, `is_story_file`, `is_canonical_vp_filename`).
- `plugins/vsdd-factory/hooks/validate-count-propagation.sh` (`_extract_counts`).
- `plugins/vsdd-factory/hooks/validate-changelog-monotonicity.sh` (file-type dispatch).
- `crates/hook-plugins/validate-state-structure/src/lib.rs` (`extract_banner_block`,
  `extract_trajectory_tail_line`, `extract_section`, `extract_pass_count`).
- `crates/hook-plugins/verify-factory-lock/src/lib.rs` (frontmatter byte-length bound,
  factory_lock block scan).
- `crates/hook-plugins/validate-policies-schema/src/lib.rs` (a structurally unrelated
  `last_amended` field on `policies.yaml`, not one of the five target files).
- `plugins/vsdd-factory/bin/compute-input-hash` (`inputs:` dependency content hashing).

### Audit finding 1 — F-P4-004 is already fixed

`frontmatter.rs::collect_block_scalar_body` (docs/red-gate-log.md F-P4-004; tests
`test_BC_5_39_010_frontmatter_field_block_scalar_pipe_literal` /
`..._pipe_strip` / `..._fold_gt` / `..._fold_strip`, all passing) already scans past the `|`,
`|-`, `>`, `>-` block-scalar indicators and returns the collected body, joining literal-mode
content lines with `\n` — it does NOT return the indicator string. This closes the specific
regression the story's AC-002(ii) worried about; no fix is required here.

### Audit finding 2 — arm_e (Class E1/E2) never fires on any of the five target files

`dispatch.rs::is_frontmatter_parity_target` restricts Class E (`version:` vs `last_amended:`
parity, and `modified:` monotonicity) to BC files, story files, VP files, and epic files, and
explicitly EXCLUDES the index files: `is_bc_file` rejects `BC-INDEX.md`
(F-S2107-P1B-005), `is_story_file` rejects `STORY-INDEX.md` ("STORY-INDEX.md is Arm B2's
trigger, not a story file"), `is_canonical_vp_filename` rejects `VP-INDEX.md`
(F-S2107-P3-011/F-P2-003+F-P2-008). `ARCH-INDEX.md` and `STATE.md` are never admitted by any of
the four `is_*` predicates the function calls. `lib.rs` states this directly in a doc comment:
"STORY-INDEX.md: no Class E arm — PC34 scopes Class E to BC/VP/story/epic files only."
Grepping `arm_a1.rs`, `arm_b1.rs`, `arm_b2.rs` for `last_amended` returns zero hits — no other
arm reads these five files' own `last_amended` field either.

This means the story's AC-002(i)/AC-007 framing ("arm_e reads position-0 of the INDEX file's own
`last_amended` for E1") describes a check that does not exist for these five files today. The
audit is still valuable — it establishes that IF a future validator extension ever adds
INDEX/STATE coverage to Class E, a first-content-line-anchored regex parse (as arm_e already does
for BC/VP/story/epic files) would work unmodified against a multi-line block scalar, because
`extract_last_amended_outer_version` is a byte-walk anchored at position 0 that stops at the
first `)` — it never needs to inspect what follows the first line.

### Audit finding 3 — three of the five files already implement a `changelog:` sequence

`ARCH-INDEX.md`, `BC-INDEX.md`, and `VP-INDEX.md` frontmatter already carry a `changelog:` block
sequence (`- date: YYYY-MM-DD` / `change: "..."` items) alongside `last_amended:`, and have for
some time (STORY-INDEX.md's own header blockquote calls out the "D-448(b)/D-414(c) migration
deferral" for extending this same pattern to itself). Each `changelog:` item is an independent
top-level list entry — appending a new item does not require touching or rewriting any prior
item. `STORY-INDEX.md` has no `changelog:` field yet (deferred). `STATE.md` has no `changelog:`
field and does not need one: its body already carries an append-only `## Decisions Log` (D-NNN
rows) and `## Phase Progress` table, which is the structural equivalent for an ops-state document
(as opposed to a spec/index document with `inputs:`-based traceability).

No validator in the audited surface parses `changelog:` semantically on any of these files
(`extract_frontmatter_sequence` exists in `frontmatter.rs` but is used only for
`behavioral_contracts:` and `modified:`, never for `changelog:` on these five files). A
`changelog:` sequence is therefore inert from every current reader's perspective — safe to grow
by list-item append with zero validator risk.

### Audit finding 4 — count-propagation and changelog-monotonicity are unaffected

`validate-count-propagation.sh::_extract_counts` does not parse `last_amended` as a named field;
it scans every non-historical-section line of the file for count-keyword regexes
(`NNN BCs`, `total_bcs: NNN`, etc.). This already runs against today's single physical
`last_amended:` line's free-text summary and carries the same (pre-existing, not new) risk of a
coincidental count-shaped substring in prose regardless of whether that prose lives on one line
or is spread across several `changelog:`/block-scalar lines. No behavior change; no fix required.
`validate-changelog-monotonicity.sh` unconditionally exits 0 for
`*STATE.md|*INDEX.md|*burst-log*|*convergence-trajectory*|*session-checkpoint*|*lessons*` before
any parsing occurs — confirmed unaffected for all five files regardless of shape.

### Audit finding 5 — validate-state-structure and verify-factory-lock are unaffected

`validate-state-structure/src/lib.rs` extracts only: the `<!-- STATE.md SIZE BUDGET -->` HTML
comment block, the convergence-trajectory tail line, named `## ` body sections, and a pass count
from body prose. It has no dependency on `crates/hook-plugins/validate-cross-site-correspondence`
and contains no scalar/block-scalar frontmatter extractor of its own — `last_amended`'s shape is
invisible to it. `verify-factory-lock/src/lib.rs` scans for the `factory_lock:` key and the SIZE
BUDGET banner using dedicated, already-hardened HTML-comment-block and byte-length-bounded scans
(F-P5-006 fixed the "marker string appears in frontmatter prose" false-negative; the frontmatter
parse-length bound was already raised for "real STATE.md files [that] have frontmatter that
easily exceeds 30KB [from] long `last_amended` values"). A current-entry-only `last_amended`
strictly SHRINKS frontmatter size relative to today; no new risk.

### Audit finding 6 — `compute-input-hash` hashes file content, not `last_amended` specifically

`plugins/vsdd-factory/bin/compute-input-hash` computes an MD5 over the full byte content of each
file listed in a dependent artifact's `inputs:` array — it does not special-case
`last_amended`. All five target files are listed as `inputs:` dependencies by numerous stories
(e.g., `S-7.10`, `S-25.01`, `S-15.07`, `S-17.07`, `S-16.02`, `S-4.06`, and others). Any byte-level
edit to these files — under the CURRENT single-line-scalar shape or the new shape — already
changes their hash and requires the standard `bin/compute-input-hash --update`/`--scan` resync
for dependents. The one-time migration (AC-005/AC-006) is therefore a MECHANICAL, already-familiar
one-time resync event, not a new ongoing cost.

## Decision

1. **`last_amended:` becomes CURRENT-ENTRY-ONLY on all five files, permanently.** Every future
   state-manager burst writes `last_amended:` as a single-line, double-quoted YAML scalar holding
   ONLY the new entry (`"YYYY-MM-DD (vX.Y) — <summary>"`, D-1144-escaped). The burst MUST NOT read
   the existing value and wrap it in a `[Prior: ...]` bracket. This is a strict simplification of
   AC-001's Shape 1 (no block-scalar indicator is introduced on `last_amended:` itself), keeping
   every future edit to this field on the exact code path already exercised by
   `extract_frontmatter_field`'s quoted-scalar branch (zero new frontmatter.rs code paths).
2. **The outgoing entry moves to `changelog:`, not to `last_amended`'s own bracket.** For
   `ARCH-INDEX.md`, `BC-INDEX.md`, and `VP-INDEX.md`, the burst that overwrites `last_amended`
   PREPENDS one new list item (`- date: ... / change: "..."`) to the existing `changelog:`
   sequence, carrying the entry that was just displaced from `last_amended`. This is the same
   discipline these three files already use for `changelog:` today; the ADR makes explicit that
   `last_amended`'s outgoing value is now the source for the next `changelog:` item, replacing the
   D-1149-era `[Prior: ...]` inline nesting.
3. **`STORY-INDEX.md` gains a `changelog:` sequence, completing the D-448(b)/D-414(c) deferral.**
   Same shape and discipline as the three files in (2).
4. **`STATE.md` keeps `last_amended:` current-entry-only with NO frontmatter `changelog:` field.**
   Its existing body-level `## Decisions Log` (D-NNN rows) and `## Phase Progress` table remain
   the durable, already-structured, already-append-only historical record for this file; adding a
   redundant frontmatter `changelog:` would duplicate that mechanism without benefit.
5. **The five D-1149 `*-amendment-history.md` sidecars are FROZEN, not folded back.** They remain
   the archive of record for pre-2026-09-02 history that predates this fix; no future burst writes
   to them. This matches the disposition already recorded in S-15.03 §Migration.
6. **The AC-006 migration/rotation tool is a Rust CLI binary under `plugins/vsdd-factory/bin/`**
   (POLICY 21 `no_new_shell_scripts` — not a `.sh` script), performing: (i) the one-time conversion
   of each file's current slim `last_amended` value into the strict current-entry-only form (no
   change needed where it is already current-entry-only per D-1149, other than the D-1144
   escape remediation in finding 7 below); (ii) as a safety-net utility, rotation of an
   over-long `changelog:` sequence into a per-cycle archive under `.factory/cycles/<cycle-name>/`,
   mirroring how `burst-log.md`/`decision-log.md` already archive per cycle.
7. **D-1144 YAML-escape remediation ships in the same migration pass.** The three files with a
   currently-broken (unescaped literal double-quote) `last_amended` entry —
   `BC-INDEX.md`, `ARCH-INDEX.md`, `STATE.md` (D-1149 incidental finding) — are corrected to
   strictly-valid YAML as part of the AC-006 tool's one-time run, not as a separate pass.

This decision selects Option 2 from S-15.03's Primary Design ("structured `changelog:`
sequence") over Option 1 ("YAML block scalar on `last_amended` itself") and over the
"stop-accumulating" alternative — see Rationale and Alternatives Considered.

## Rationale

- **Option 2 is not a new mechanism — it completes an already-shipped one.** `ARCH-INDEX.md`,
  `BC-INDEX.md`, and `VP-INDEX.md` already carry `changelog:` sequences populated by
  append-style edits. Choosing Option 2 means the "durable fix" is: stop writing history INTO
  `last_amended`, and instead route it to the structure that already exists and is already proven
  safe (no validator parses it, and list-item append cannot regrow a mega-line the way
  string-concatenation-into-one-scalar can). This is the lowest-code-change, lowest-validator-risk
  path available, directly satisfying AC-002's "minimize validator changes" framing.
- **Zero frontmatter.rs / arm_e code changes are required.** Finding 1 shows F-P4-004 (the named
  regression risk) is already fixed. Finding 2 shows arm_e never reads these five files' own
  `last_amended` at all, so there is no E1/E2 invariant to preserve here (AC-007's concern is moot
  for these five files specifically, though the underlying position-0-anchored parse would also
  tolerate a multi-line block scalar unmodified, per finding 2's closing observation — useful if
  Class E coverage is ever extended to index/state files in the future).
  Keeping `last_amended:` a plain single-line quoted scalar (rather than switching it to a block
  scalar, Option 1) means this field never exercises `collect_block_scalar_body` at all — the
  smallest possible surface change.
  **On STORY-INDEX.md specifically:** `dispatch.rs::is_story_file` and
  `dispatch.rs::is_frontmatter_parity_target` both explicitly exclude `STORY-INDEX.md`
  (finding 2), so adding `changelog:` to it triggers no Class E parsing either — the same
  zero-validator-risk argument applies uniformly across all five files, not just the three that
  already have `changelog:`.
- **Option 1 (block scalar on `last_amended` itself) was evaluated and found technically viable
  but strictly worse here.** The audit confirms `collect_block_scalar_body` already joins ALL
  literal-mode content lines with `\n`, and `extract_last_amended_outer_version`'s position-0
  byte-walk would still correctly extract the newest entry's date+version from a multi-line block
  (it stops parsing at the first `)`, regardless of what follows). So Option 1 would ALSO work
  wherever arm_e is live (BC/VP/story/epic files, not the five index/state files). But choosing it
  for the five index/state files would (a) introduce a NEW code path
  (`collect_block_scalar_body`) on a field where the audit shows zero readers currently need it to
  handle block scalars, and (b) duplicate — under a different field name and shape — the exact
  `changelog:` structure three of the five files already have. Option 2 achieves the same
  mega-line-proofing with less surface area disturbed.
- **The "stop-accumulating" alternative (current-entry-only, git log as sole archive, no
  `changelog:` at all) was rejected** for the same reason S-15.03 rejects it: it would make
  in-file history non-browsable without a `git log -p` round-trip, discarding a structure
  (`changelog:`) that three of the five files already maintain and that engine conventions
  (quick `grep`/read audits of a file's own history) already rely on.
- **Sidecar freeze, not fold-back, is confirmed correct by finding 6.** Folding hundreds of KB of
  `[Prior: ...]` text back into `changelog:` would itself require the same kind of large,
  hash-changing, single-pass surgery this ADR exists to avoid needing again, for content already
  fully preserved in the sidecars and in `git log -p`.
- **The AC-006 tool must be a Rust `bin/` binary, not a shell script**, per POLICY 21
  (`no_new_shell_scripts`) and per TD-FACTORY-HOOK-BYPASS-001 (the tool is the SANCTIONED
  mechanism that removes the need for a future ad hoc POL-3 exception on this class of edit).

## Consequences

### Positive

- No production validator code changes are required to ship AC-001 for the five target files —
  `last_amended` never leaves the already-supported single-line-quoted-scalar shape, and
  `changelog:` is already inert to every current reader (findings 1-5).
- Mega-line recurrence is structurally impossible going forward: `last_amended` never accumulates
  (current entry only, overwritten each burst) and `changelog:` grows by independent list-item
  append (bounded per-item size; no nested-bracket string concatenation).
- Completes, rather than replaces, prior architecture: `ARCH-INDEX.md`/`BC-INDEX.md`/`VP-INDEX.md`
  already model the target end state; this ADR generalizes it to `STORY-INDEX.md` and states the
  discipline explicitly for all four spec/index files plus STATE.md's own body-level mechanism.
- The D-1149 sidecars, `.factory/cycles/*` archives, and `git log -p` remain the full historical
  record with no loss of information versus today.

### Negative / Trade-offs

- `changelog:` sequences on `ARCH-INDEX.md`/`BC-INDEX.md`/`VP-INDEX.md` will keep growing
  unboundedly across a very long cycle, same as `burst-log.md`/`decision-log.md` already do — the
  AC-006(ii) rotation tool is a REQUIRED safety net for this story to be complete, not optional
  polish; without it, this ADR only slows the mega-line class of failure (moves it from
  `last_amended` to `changelog:`) rather than bounding it, though a `changelog:` mega-growth is
  categorically less severe (list items, not a single string) and does not by itself exhaust the
  same validator fuel path today, since no validator parses `changelog:` at all.
- Adding `changelog:` to `STORY-INDEX.md` is itself a byte-level edit to a 411,448-byte file
  already listed as an `inputs:` dependency by many stories (finding 6) — the migration burst MUST
  run `bin/compute-input-hash --scan .factory --update` (or targeted per-dependent runs) as part
  of the same burst, or POLICY 18 hash-parity checks will fail for every dependent story until
  resynced.
- The D-1149 sidecar naming/location is not yet registered in
  `plugins/vsdd-factory/config/artifact-path-registry.yaml` (STATE.md v9.64 Drift Item (a),
  anchored to S-15.03). This ADR does not resolve that registration — it is a Phase C follow-up
  (see architect's final report) — but flags it so the AC-006 tool's own output paths (rotation
  archives under `.factory/cycles/<cycle-name>/`) are registered at the same time, not left as a
  second gap.
- `BC-INDEX.md`, `ARCH-INDEX.md`, and `STATE.md` currently carry the D-1144 unescaped-double-quote
  defect in their live `last_amended` entries (audit finding 7); until the AC-006 tool's migration
  pass runs, these three files do not parse under strict YAML `safe_load` (though the hand-rolled
  Rust extractors in this audit tolerate it structurally, since they scan for the literal `"` at
  a specific position rather than running a real YAML parser).

### Status as of 2026-09-02

**ACCEPTED — Human-Ratified 2026-09-02 (POLICY 22).** The human read this ADR in full and
explicitly approved its Decision Option 2 (`last_amended` = current-entry-only scalar; history
routes to the existing `changelog:` sequence; zero validator code changes required per findings
1-5; the five D-1149 sidecars remain frozen per Decision item 5). Zero code has been written or
changed under this ADR — Phase A was design/audit only per the S-15.03 dispatch instructions.
Phase B (product-owner BCs) and Phase C (TDD implementation of the AC-006 tool, the write-path
discipline update in `state-burst`/`state-manager`, and the one-time migration) are now unblocked
by this acceptance.

## Alternatives Considered

- **Option 1 — YAML block scalar on `last_amended` itself (`last_amended: |`, newest-first, one
  entry per physical line):** Technically viable per findings 1-2, but rejected as the primary
  design because it introduces a new code path on a field none of the five target files' current
  readers need to handle as a block scalar, and duplicates the `changelog:` structure three of the
  five files already have under a different, non-machine-distinguished shape. Documented in
  S-15.03 as "Shape 1"; this ADR selects "Shape 2" (already partially in production) instead.
- **Stop-accumulating (current-entry-only, git log as sole in-file-adjacent archive, no
  `changelog:`):** Simplest possible cure. Rejected because it discards the browsable
  `changelog:` structure that three files already maintain and that existing engine conventions
  (quick in-file history greps) rely on.
- **Append-to-sidecar + per-cycle rotation (skip `changelog:`, append every outgoing entry
  directly to the `*-amendment-history.md` sidecar):** Rejected because it relocates the
  single-scalar-accumulation risk to the sidecar file rather than eliminating it, and abandons the
  `changelog:` structure already live in three of the five files.

## Source / Origin

- `.factory/stories/S-15.03-index-cite-refresh-hook.md` §Scope Extension (last_amended
  Write-Path Durable Fix), AC-001 through AC-010 — the story this ADR's Phase A design gate
  serves.
- `.factory/cycles/v1.0-brownfield-backfill/decision-log.md` D-1149 (one-time sidecar surgery)
  and its lesson `L-BB-D1149`.
- `crates/hook-plugins/validate-cross-site-correspondence` crate, module `frontmatter`:
  function `extract_frontmatter_field` and its block-scalar helper `collect_block_scalar_body` —
  block-scalar handling, F-P4-004 already fixed (regression tests
  `test_BC_5_39_010_frontmatter_field_block_scalar_pipe_literal`,
  `..._pipe_strip`, `..._fold_gt`, `..._fold_strip`, all passing on develop).
- Same crate, module `arm_e`: functions `extract_last_amended_outer_version` (position-0
  byte-walk) and `run_arm_e1`.
- Same crate, module `dispatch`: functions `is_bc_file`, `is_story_file`, `is_story_index`,
  `is_canonical_vp_filename`, `is_frontmatter_parity_target`.
- Same crate, module `lib`: doc comment above the Arm B2 dispatch arm stating "STORY-INDEX.md:
  no Class E arm — PC34 scopes Class E to BC/VP/story/epic files only."
- `plugins/vsdd-factory/hooks/validate-count-propagation.sh`: the `BASENAME` case-statement
  file-type dispatch and the `_extract_counts` function's per-line regex extraction.
- `plugins/vsdd-factory/hooks/validate-changelog-monotonicity.sh`: the
  `*STATE.md|*INDEX.md|*burst-log*|*convergence-trajectory*|*session-checkpoint*|*lessons*`
  early-exit case arm.
- `crates/hook-plugins/validate-state-structure` crate, module `lib`: functions
  `extract_banner_block`, `extract_trajectory_tail_line`, `extract_section`,
  `extract_pass_count` — no `last_amended` structural dependency in any of them.
- `crates/hook-plugins/verify-factory-lock` crate, module `lib`: the frontmatter byte-length
  bound and F-P5-006 marker-in-prose fix, both documented in that module's own doc comments.
- `crates/hook-plugins/validate-policies-schema` crate, module `lib`: struct
  `FrontmatterHeader` field `last_amended` and function `check_header_fields` — unrelated
  `policies.yaml` header field, cited for completeness/scope exclusion.
- `plugins/vsdd-factory/bin/compute-input-hash`: the `inputs:` frontmatter extraction block and
  the hash-computation loop over resolved input files — no `last_amended`-specific handling.
- `.factory/specs/architecture/ARCH-INDEX.md`, `.factory/specs/behavioral-contracts/BC-INDEX.md`,
  `.factory/specs/verification-properties/VP-INDEX.md` frontmatter (`changelog:` sequences
  already live, read directly during this audit on 2026-09-02).
- `.factory/stories/STORY-INDEX.md`, `.factory/STATE.md` frontmatter (no `changelog:` field
  present as of 2026-09-02, confirmed by direct read during this audit).
- `plugins/vsdd-factory/config/artifact-path-registry.yaml`, entry for the
  `decisions/ADR-{adr-id}-{slug}.md` canonical path pattern — confirmed registered before this
  ADR was written.
