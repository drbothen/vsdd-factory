---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-09-02T00:00:00Z
phase: F2
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md
  - .factory/stories/S-15.03-index-cite-refresh-hook.md
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md
input-hash: "067b5b9"
traces_to: .factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-042"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.45.001
section: "5.45"
last_amended: "2026-09-02 (v1.1) — Scope clarification (product-owner; consistency-audit F-8): PC1-PC4's write-path discipline is scoped to exactly the 5 D-1149 files per ADR-049 Decision 1-7; explicit out-of-scope note added for other .factory/ artifacts' last_amended fields (governed instead by pre-existing arm_e Invariant 1, unaffected)."
---

# BC-5.45.001: `last_amended` Write-Path Invariant — Current-Entry-Only Overwrite Plus `changelog:` Prepend (Never Inline-Chain)

## Description

**Scope (unambiguous per ADR-049 §Decision 1-7, which ratifies this discipline for exactly five
files and does not extend it further).** The five files named in D-1149 — `STORY-INDEX.md`,
`BC-INDEX.md`, `ARCH-INDEX.md`, `VP-INDEX.md`, `STATE.md` — MUST have their `last_amended:`
field written using the current-entry-only discipline ratified by ADR-049: the executing agent
OVERWRITES `last_amended` with ONLY the new entry, and the entry it displaces is PREPENDED as a
new `changelog:` sequence list item (or, for `STATE.md`, superseded by the file's own body-level
`## Decisions Log`/`## Phase Progress`, since that file carries no frontmatter `changelog:`). The
prior discipline — read the existing value, wrap it in a `[Prior: ...]` bracket, and write the
whole concatenation back as one larger scalar — is what produced a 323,499-char single physical
line on `STORY-INDEX.md` (and comparable growth on the other four files) and repeatedly exhausted
the bash-adapter WASM validators' fuel budget (743 fuel-timeouts/day, 2026-09-02). This BC is the
write-path contract for those five files; BC-10.13.001 governs the one-time migration/rotation
tool that brings them into this shape, and BC-4.18.001 governs the fuel-budget-relief regression
proof.

**Out of scope — other `.factory/` artifacts' `last_amended` field.** This BC does NOT newly
mandate the current-entry-only/`changelog:`-prepend shape on any other `.factory/` artifact
(story, BC, VP, or epic files) carrying its own `last_amended:` field. Those files' mega-line
growth risk is structurally different from the five D-1149 files: their `last_amended` is edited
a handful of times across the file's lifetime by a specialist agent (product-owner, story-writer,
architect), not on every state-manager burst of a long-running cycle, so the D-1149
unbounded-growth failure mode this BC exists to fix does not arise there the same way. Where
arm_e's Class E1/E2 admits such a file (`dispatch.rs::is_frontmatter_parity_target` — BC, VP,
story, and epic files), it remains governed by the PRE-EXISTING position-0 `(vX.Y)` single-token
parity check described in Invariant 1 below — a constraint this BC neither introduces, relaxes,
nor extends. A non-single-token `last_amended` value on such a file (e.g. a legacy `vX.Y → vX.Y`
version-transition annotation predating this BC) is a pre-existing, out-of-scope condition for
this BC; any arm_e advisory it produces is not a regression this BC's discipline is responsible
for.

## Preconditions

1. An executing agent (state-manager, for the five D-1149 files — the four index files and
   STATE.md) is about to write a new history entry to one of those five files' `last_amended:`
   frontmatter field. (Per this BC's Description §Out-of-scope note, other `.factory/` artifacts'
   own `last_amended:` writes are outside this BC's scope.)

2. The file already has a `last_amended:` field in a valid or D-1149-slim current-entry form (a
   single-line, double-quoted YAML scalar holding one dated entry, optionally followed by a
   `[Prior history → <file>-amendment-history.md]` pointer note — never a nested `[Prior: ...]`
   bracket chain, per this BC's own Postcondition 1 applying retroactively to every future write).

3. For `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, and (post-migration, per BC-10.13.001)
   `STORY-INDEX.md`, the file's frontmatter carries a top-level `changelog:` YAML sequence
   (possibly empty on first use).

## Postconditions

### PC1 — `last_amended` is overwritten, never wrapped

The agent writes `last_amended:` as a single-line, double-quoted YAML scalar of the form
`"YYYY-MM-DD (vX.Y) — <summary>"` (D-1144-escaped — see Invariant 3) holding ONLY the new entry.
The agent MUST NOT read the field's existing value and concatenate it (as a `[Prior: ...]`
bracket or any other nested form) into the new value. The prior value is either superseded
(STATE.md, PC3) or moved to `changelog:` (PC2) — it never survives inside `last_amended` itself
after this write.

### PC2 — the displaced entry is prepended to `changelog:` (files that have one)

For `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, and `STORY-INDEX.md` (once BC-10.13.001's
migration adds the field), the agent PREPENDS exactly one new list item to the top of the
`changelog:` sequence, carrying the entry that `last_amended` held immediately before this write
(its full text, including any trailing `[Prior history → ...]` pointer note). Every existing
`changelog:` item is left byte-for-byte untouched — this is a list-item prepend, never a
rewrite-in-place of the sequence or of any existing item.

### PC3 — `STATE.md` has no frontmatter `changelog:` counterpart

For `STATE.md`, the agent applies PC1 (current-entry-only overwrite) but does NOT add a
frontmatter `changelog:` item. `STATE.md`'s already-append-only body-level `## Decisions Log`
(D-NNN rows) and `## Phase Progress` table are the durable historical record for this file; the
entry displaced from `last_amended` is simply superseded there, not re-homed into a new
structure.

### PC4 — no future write ever re-introduces inline bracket-chaining

For every one of the five D-1149 files, no write performed under this BC's discipline ever
produces a `last_amended` value containing a nested `[Prior: <date> (vX.Y) — ...]` bracket
referring to a DIFFERENT dated entry than the current one. (The D-1149-era trailing pointer note — `[Prior history →
<file>-amendment-history.md]`, a static, non-growing reference to a frozen sidecar — is
explicitly NOT this pattern and MAY be retained or repeated verbatim across writes; it is not a
history bracket, it never grows, and it carries no dated entry of its own.)

## Invariants

1. **arm_e E1 (version parity) self-consistency is preserved wherever it is checked.** For files
   where `dispatch.rs::is_frontmatter_parity_target` admits Class E checking today (BC, VP, story,
   and epic files — NOT the five index/state files per ADR-049 §Audit finding 2), the position-0
   `(vX.Y)` token in the new `last_amended` value MUST equal the file's own frontmatter `version:`
   field. For the five index/state files, arm_e never fires today, but this BC still requires the
   same self-consistency as a human-audit/future-validator-extension guarantee (ADR-049
   §Rationale, closing parenthetical).

2. **`modified:` array monotonicity (arm_e E2) is unaffected.** Nothing in this BC's write-path
   change alters how or when the separate `modified:` frontmatter array is appended to.

3. **Every value emitted by this write-path is strictly-valid YAML.** Embedded double-quotes in
   the new `last_amended` entry or in the new `changelog:` item's `change:`/`summary:` field are
   escaped (D-1144 discipline) so the field parses cleanly under strict YAML `safe_load`.

4. **`changelog:` grows by append only — never by in-place rewrite of an existing item.** This
   mirrors the append-only numbering discipline (POLICY 1) applied to a frontmatter sequence
   rather than an identifier catalog: an existing `changelog:` item's `date:`/`change:` text is
   immutable once written under this discipline.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | First-ever write under this discipline to a file whose `changelog:` sequence does not yet exist (e.g. `STORY-INDEX.md` before BC-10.13.001's migration runs) | The write-path fix (PC1/PC2) is NOT applicable until the file has a `changelog:` sequence to prepend into; the agent MUST NOT invent inline bracket-wrapping as an interim measure — it either (a) defers the write until BC-10.13.001's migration has added `changelog:`, or (b) if migration cannot run first, adds a bare single-item `changelog:` sequence in the same write that also applies PC1, satisfying PC2 immediately rather than falling back to bracket-wrapping. |
| EC-002 | Write targets `STATE.md` | PC3 applies: current-entry-only overwrite with no `changelog:` addition; the displaced entry's substance must already be captured (or is captured in the same burst) by a `## Decisions Log` row or `## Phase Progress` entry — this BC does not itself mandate a NEW Decisions Log row for every `last_amended` write, only that no data is uniquely lost by the overwrite (STATE.md's Decisions Log is the durable substrate per ADR-049 Decision 4). |
| EC-003 | The new entry text contains an embedded double-quote (e.g. a quoted commit-message fragment or a quoted identifier) | The quote is escaped (`\"`) per Invariant 3 (D-1144 discipline) in both the `last_amended` scalar and the `changelog:` item's text field; an unescaped literal `"` is a Postcondition violation. |
| EC-004 | The new entry text contains a YAML-significant colon (`: `) inside prose (e.g. `note: see below`) | The entire value remains inside its enclosing double-quoted scalar; a colon-space sequence inside an already-double-quoted YAML scalar does not require additional escaping, but the agent MUST verify the scalar's outer quoting is unbroken (no stray unescaped `"` earlier in the string that would prematurely close the scalar — this is the same class of defect as EC-003). |
| EC-005 | The displaced entry (moving from `last_amended` to `changelog:`) itself still carries a `[Prior history → <file>-amendment-history.md]` trailing pointer note from a prior D-1149-era write | The pointer note moves verbatim along with the rest of the displaced entry's text into the new `changelog:` item — it is not stripped, rewritten, or treated specially; it is simply part of the entry's text being relocated. |
| EC-006 | Agent mistakenly reads the existing `last_amended` value before writing (a process error, not a data-shape error) | This is a process violation of PC1 regardless of what is subsequently written; the read itself is not prohibited (an agent may need to read the old value in order to construct the PC2 `changelog:` item), but the OLD value must never be concatenated back into the NEW `last_amended` scalar. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `BC-INDEX.md` current `last_amended: "2026-09-01 (v5.39) — ..."`; new burst entry `"2026-09-02 (v5.40) — ..."`; existing `changelog:` has ≥1 item | `last_amended` becomes exactly `"2026-09-02 (v5.40) — ..."` (no bracket); `changelog:` sequence gains one new item at position 0 whose `change:` text equals the OLD `last_amended` value verbatim; all pre-existing `changelog:` items unchanged | happy-path |
| `STORY-INDEX.md` before BC-10.13.001 migration — no `changelog:` field exists yet; new burst needs to write `last_amended` | Per EC-001, the write either defers or bootstraps a single-item `changelog:` sequence in the same burst; `last_amended` still becomes current-entry-only | edge-case |
| New entry text contains `he said "stop"` unescaped | REJECTED shape — the correct emitted form is `he said \"stop\"` inside the double-quoted scalar; an unescaped literal `"` fails Invariant 3 / strict YAML `safe_load` | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — route to architect) | For every governed write to `last_amended`, the post-write value contains no nested `[Prior: <date> (vX.Y) — ...]` bracket referencing a different dated entry | proptest/integration: generate N synthetic writes against a fixture and assert bracket-absence |
| (TBD — route to architect) | Every `changelog:` item present before a write remains byte-identical after the write (append-only) | proptest: diff pre/post `changelog:` sequences excluding the newly-prepended item |
| (TBD — route to architect) | Every emitted `last_amended`/`changelog:` value parses under strict YAML `safe_load` | integration: round-trip parse of fixture files after N synthetic writes |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-042 |
| Capability Anchor Justification | CAP-042 ("`last_amended` Write-Path Durable Fix: current-entry-only scalar, `changelog:` prepend discipline, sanctioned migration/rotation tooling, and bash-adapter fuel-budget relief") per `.factory/specs/domain-spec/capabilities.md` §CAP-042. BC-5.45.001 is the write-path-invariant implementing BC for CAP-042 (BC-10.13.001 implements the tooling clause; BC-4.18.001 implements the fuel-relief clause). |
| L2 Domain Invariants | none (operational infrastructure — no existing DI-NNN governs frontmatter history-field write-path shape; ADR-049 itself is the governing decision record) |
| Architecture Module | `plugins/vsdd-factory/skills/state-burst/SKILL.md` (to be amended per S-15.03 AC-004 — canonical write-path instruction); `plugins/vsdd-factory/agents/state-manager.md` (to be amended per S-15.03 AC-005 — agent-prompt discipline) |
| Stories | S-15.03 |
| Source Issues | D-1149 (`.factory/cycles/v1.0-brownfield-backfill/decision-log.md`); `L-BB-D1149` |
| ADR Reference | ADR-049 §Decision 1, §Decision 2, §Decision 4 |

## Related BCs

- BC-10.13.001 — the sanctioned one-time migration/rotation tool that brings existing files into the shape this BC's write-path then maintains going forward (depends on)
- BC-4.18.001 — the fuel-budget-relief regression proof that this write-path change (together with BC-10.13.001's migration) eliminates the 743-fuel-timeouts/day symptom (composes with)
- BC-5.37.001 — state-manager corpus-wide-grep discipline before declaring a count change complete; a sibling state-manager write-discipline BC in the same SS-05 engine-governance cluster (related to)
- BC-5.40.001 — STATE.md factory_lock sole-writer discipline; a sibling STATE.md write-path BC governing a different frontmatter concern (lock state, not history) (related to)

## Architecture Anchors

- `plugins/vsdd-factory/skills/state-burst/SKILL.md` — canonical execution locus for the Single-Commit Burst Protocol (TD-VSDD-053); to be amended per S-15.03 AC-004 to instruct the current-entry-only + `changelog:`-prepend discipline, explicitly forbidding the read-wrap-rewrite-as-`[Prior: ...]` pattern.
- `plugins/vsdd-factory/agents/state-manager.md` — to be amended per S-15.03 AC-005 so every future burst dispatched to the state-manager agent follows this discipline without needing to re-discover it from the skill file alone.
- `.factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md` — the governing ADR; §Decision 1/2/4, §Rationale, §Audit findings 1-6.

## Story Anchor

S-15.03 (E-12 Engine Governance — `last_amended` Write-Path Durable Fix, Scope Extension AC-001/AC-004/AC-005/AC-007/AC-008/AC-010)

## VP Anchors

TBD — VP needs flagged above (3 candidate VP rows); route to architect for VP-NNN assignment and registration in VP-INDEX.md per `vp_index_is_vp_catalog_source_of_truth`.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.1 | 2026-09-02 | Scope clarification (product-owner; consistency-audit F-8). Added explicit §Description Scope/Out-of-scope note: PC1-PC4's write-path discipline (current-entry-only overwrite + `changelog:` prepend) is ratified by ADR-049 §Decision 1-7 for exactly the 5 D-1149 files and does NOT extend to other `.factory/` artifacts' own `last_amended` fields (those remain governed, where arm_e admits them, by the pre-existing Invariant 1 position-0 single-token check, unaffected by this BC). Precondition 1 and PC4 narrowed to name only the 5 D-1149 files. No change to PC1/PC2/PC3 substance or to Invariants/Edge Cases/Test Vectors. |
| 1.0 | 2026-09-02 | Initial authoring (product-owner; ADR-049 Phase B; S-15.03 `last_amended` Write-Path Durable Fix). PC1 current-entry-only overwrite; PC2 `changelog:` prepend for the 3 (soon 4) files that carry it; PC3 STATE.md body-level-record exception; PC4 no-inline-bracket-chaining invariant. 4 invariants (arm_e E1 self-consistency, E2 unaffected, strict-YAML validity, changelog append-only). 6 edge cases EC-001..EC-006. 3 test vectors. 3 VP candidates flagged for architect. lifecycle_status: draft. |
