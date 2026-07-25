---
document_type: behavioral-contract
level: L3
version: "1.6"
status: draft
producer: product-owner
timestamp: 2026-07-25T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
  - plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md
input-hash: "aff43f3"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-036"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-036 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-036; §Traceability L2 Capability TBD→CAP-036; Capability Anchor Justification updated to cite CAP-036/ARCH-INDEX v3.07."
  - "2026-07-19 (v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description preflight rationale clarified — `--force` mechanism premise (later corrected at v1.3); Invariant 5 added."
  - "2026-07-19 (v1.3) — adv pass-1 fix burst (F-P1-005) per ADR-031 v1.1 delta analysis v1.2 §Issue #523 (product-owner): §Description + Invariant 5 mechanism corrected — false --force premise removed; actual mechanism: .factory/ is gitignored on story branch → shadow content is gitignored (not untracked) → plain git worktree remove passes clean-state check (gitignored ≠ untracked for the check) → underlying rm-rf silently destroys shadow content; preflight is correct fix because find sees gitignored files that git's check ignores; --force secondary note retained as clearly-labeled secondary. PC2a corrected: git worktree remove --force→git worktree remove (plain command per step-g-cleanup.md)."
  - "2026-07-24 (v1.4) — S-21.04 adv pass-1 fix burst F-004/006/007/010 (product-owner): F-007: §Description provenance corrected — shadow .factory/ created by errant write (not at git worktree add time; .factory/ is gitignored on product branch so checkout is empty). F-010: --force rationale corrected in §Description ¶2 and PC2a — BC mandate (strips git built-in protection for non-gitignored untracked files), not guard-enforced constraint. F-006: PC2a/EC-005 amended fail-closed — absent .factory/→PC2a sub-case (a); find error (non-path-absent)→PC2c HALT; blanket 2>/dev/null suppression removed; PC2c block added. F-004: PC1 git command fixed — <story-worktree-path>→<main-worktree-path> with clarification that story-worktree rev-parse returns story-worktree root. CANONICAL_FACTORY_ROOT defined: repo-root of main checkout (not .factory/ mount)."
  - "2026-07-24 (v1.5) — S-21.04 adv pass-2 fix burst F-002/O-005 (product-owner): F-002: §Description ~line 64 corrected — suppressed preflight command `find <worktree-path>/.factory -type f 2>/dev/null` corrected to unsuppressed form `find <worktree-path>/.factory -type f`; consistent with PC2a/PC2b/PC2c and v1.4 changelog claim \"blanket 2>/dev/null suppression removed.\" TD-VSDD-060 sweep: grep -n \"2>/dev/null\" BC-6.26.001.md — zero occurrences on live preflight command (lines 26/35/280 are historical changelog text only). O-005: §Preconditions ¶2 caller-side aligned — callee-side phrasing corrected to caller-side per ADR-031 §Rationale and step-g-cleanup.md §G.1."
  - "2026-07-25 (v1.6) — S-21.04 adv pass-4 fix burst F-S2104-P4-007 (product-owner): PC2a sub-case (a) discrimination predicate corrected from directory-ness (`[ ! -d ]`) to existence (`[ ! -e ]`); non-directory inode at path (regular file, symlink-to-file) treated as stray shadow content → PC2b BLOCKED. TD-VSDD-060 within-file sweep complete: §Description steps 1/2/3 updated (existence-predicate + non-directory→PC2b path); non-directory-path paragraph added between PC2a and PC2b; PC2b header updated; PC2c parenthetical updated (path-nonexistence unreachable after pre-verification); EC-005 updated; EC-008 added; T-6 added."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.26.001
section: "6.26"
last_amended: "(v1.6) — S-21.04 adv pass-4 fix burst F-S2104-P4-007 (product-owner): PC2a(a) existence-predicate precision — `[ ! -d ]` → `[ ! -e ]`; non-directory at path = stray content → PC2b BLOCKED; TD-VSDD-060 within-file sweep complete. [Prior: (v1.5) — F-002/O-005. (v1.4) — F-004/006/007/010. (v1.3) — F-P1-005 gitignored. (v1.2) — research. (v1.1) — CAP-036. (v1.0) — Initial.]"
---

# BC-6.26.001: deliver-story step agents MUST write all `.factory/**` artifacts using absolute paths anchored to the canonical main-checkout `.factory/` mount, and step-G cleanup MUST run a worktree `.factory/` inventory preflight before `git worktree remove`

## Description

Story agents execute with their CWD set to the story worktree (`.worktrees/<STORY-ID>/`). When
an agent writes a `.factory/` artifact using a relative path (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD), the write creates or populates a shadow `.factory/` tree
inside the story worktree — a directory that does not exist at checkout time (`.factory/` is
gitignored on the product branch, so `git worktree add` checks out nothing there) and is neither
tracked on `factory-artifacts` nor ever updated. When step G runs `git worktree remove`, the shadow
tree and all artifacts written to it are permanently destroyed with no warning.

This BC governs two complementary protocol requirements that close the loss window.

**Why the teardown preflight is load-bearing (the gitignored shadow mechanism):**
Step G dispatches `devops-engineer` to run plain `git worktree remove` on the story worktree (no
`--force` flag — `--force` is prohibited by this BC because it strips git's built-in unclean-worktree
protection for non-gitignored untracked files; the prohibition is a BC mandate, not a guard-enforced
constraint). Stock git's
clean-state check inside `git worktree remove` gates on *untracked* files, not *gitignored* files.
Because `.factory/` is listed in `.gitignore` on the product branch, the shadow `.factory/` content
inside the story worktree is **gitignored** rather than untracked. Gitignored files are excluded from
git's untracked-file clean-state check, so the check passes silently as a false negative — even when
the shadow tree contains stray factory artifacts. The underlying `rm -rf <worktree-path>` then
silently destroys the gitignored shadow content with no warning.

The teardown preflight (`find <worktree-path>/.factory -type f`) is the correct fix
precisely because `find` reads the filesystem directly — it sees gitignored files that git's
clean-state check ignores. The preflight is the only mechanism that catches this class of loss.

**Secondary note (`--force` as an additional bypass):** If a future change introduced `--force` to
the worktree remove command, that would additionally strip git's built-in unclean-worktree
protection for any non-gitignored untracked files. The preflight would still catch `.factory/`
shadow content in that scenario. The primary failure mode for the current codebase is the gitignored
mechanism above.

**Write-path discipline (INV-E21-002 instantiation):** Every agent operating within the
deliver-story skill protocol that writes to a factory artifact MUST use an absolute path anchored
to the canonical main-checkout root. Relative paths from story worktree CWD are FORBIDDEN for
factory artifact writes. The load-bearing cases named explicitly: the DELIVERY ledger
(`.factory/stories/<STORY-ID>-DELIVERY.md`), story-frontmatter files (`.factory/stories/<STORY-ID>.md`),
and the pr-reviewer's pr-review.md record. The canonical main-checkout `.factory/` mount is always
at `<repo-root>/.factory/`; agents determine it via `git rev-parse --show-toplevel` from the
main worktree (NOT from the story worktree CWD) or from the orchestrator-provided `CANONICAL_FACTORY_ROOT`
environment variable. **`CANONICAL_FACTORY_ROOT` is the absolute path to the main-checkout
repository root** (e.g., `/abs/path/to/repo`) — NOT the `.factory/` mount directory itself. The
invariant is `$CANONICAL_FACTORY_ROOT/.factory/<artifact>` resolves to the canonical path; passing
the mount as the root would produce `$CANONICAL_FACTORY_ROOT/.factory/.factory/<artifact>` nesting.

**Teardown preflight (INV-E21-004 instantiation):** Before any `git worktree remove` command on
a story worktree, step G MUST apply a fail-closed `.factory/` inventory protocol:

1. If nothing exists at `<worktree-path>/.factory` (`[ ! -e "<worktree-path>/.factory" ]`), treat
   as no-stray-files (PC2a sub-case a) — teardown authorized immediately. If something exists at
   that path but is NOT a directory (regular file, symlink-to-file, or other non-directory inode),
   treat as stray shadow content — proceed directly to PC2b BLOCKED (list the path; do NOT run
   `find`; do NOT proceed with `git worktree remove`).
2. If `.factory/` exists and IS a directory, run `find <worktree-path>/.factory -type f` (without
   blanket error suppression). A `find` exit error for any reason other than path absence (e.g.,
   permission denial, path error) MUST HALT teardown — this is a fail-closed error (PC2c); do not
   proceed with `git worktree remove`.
3. Empty `find` output → no stray files (PC2a sub-case b — directory exists, no files). Non-empty
   output → stray artifacts found (PC2b-blocked).

If the result is non-empty (any files found), step G MUST NOT proceed with `git worktree remove`.
Instead it MUST either: (a) relocate each found file to the canonical `.factory/` mount (if valid
factory artifacts), then verify the shadow tree is empty, then proceed with teardown; or (b) STOP
with a visible error requiring manual intervention.

No new shell script or WASM plugin required (POLICY 21 satisfied). Both requirements are skill-doc
mandates.

## Preconditions

### Write-discipline precondition

1. A specialist agent (implementer, demo-recorder, pr-reviewer, or state-manager) is about to
   write a factory artifact (DELIVERY ledger, story-frontmatter, pr-review.md, or any
   `.factory/**` file) while operating with its CWD inside a story worktree
   (`.worktrees/<STORY-ID>/`).

### Teardown preflight precondition

2. The orchestrator is about to dispatch step G (devops-engineer cleanup) to execute
   `git worktree remove` on a story worktree path (`.worktrees/<STORY-ID>/`).

## Postconditions

### PC1 — Write-path discipline: all `.factory/**` writes use canonical absolute paths

Every agent writing to any `.factory/**` path during deliver-story execution MUST target the
canonical absolute path under the main-checkout root. Concretely:

- **Correct:** `Write(file_path="/abs/path/to/repo/.factory/stories/S-NNN-DELIVERY.md", ...)`
- **Forbidden:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (relative to story
  worktree CWD — silently writes to shadow tree)
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative
  traversal — brittle and error-prone)

The canonical root is determined exactly once per dispatch via `git -C <main-worktree-path> rev-parse --show-toplevel`
(where `<main-worktree-path>` is the path to the primary checkout, NOT the story worktree — running
this command against the story worktree path would return the story-worktree root, not the
canonical main-checkout root, which is the root cause of issue #523) or via the `CANONICAL_FACTORY_ROOT`
variable provided by the orchestrator dispatch preamble. It MUST NOT be assumed from CWD.

**INV-E21-002 instantiation:** Factory artifact writes MUST use canonical absolute paths. A write
to a story-worktree shadow `.factory/` is undetectable as incorrect at write time but results in
permanent data loss at teardown.

### PC2 — Teardown preflight: shadow `.factory/` must be empty before `git worktree remove`

Step G MUST apply the fail-closed inventory protocol before any `git worktree remove` command.
Three cases:

**PC2a — No stray files (teardown authorized):** Either (a) nothing exists at path
`<worktree-path>/.factory` (`[ ! -e "<worktree-path>/.factory" ]` is true — nothing to inspect or
destroy), or (b) `.factory/` exists as a directory AND `find <worktree-path>/.factory -type f`
succeeds with empty output. In both sub-cases no stray factory artifacts exist; teardown is
authorized. Step G proceeds with plain `git worktree remove` (no `--force` — `--force` is
prohibited by this BC because it strips git's built-in unclean-worktree protection for
non-gitignored untracked files; this prohibition is a BC mandate, not a guard-enforced constraint).

**Non-directory path (stray shadow content — routed to PC2b):** If something exists at
`<worktree-path>/.factory` but is NOT a directory (regular file, symlink-to-file, or other
non-directory inode), it constitutes stray shadow content — a file written at that path is subject
to exactly the same `rm -rf` destruction risk as files inside a shadow `.factory/` directory tree.
Step G MUST proceed to PC2b BLOCKED, listing the non-directory path. The `-d` test alone MUST NOT
be used as the path-absence discriminator.

**PC2b — Stray factory artifacts found (teardown blocked):** Either (a) the `find` command
returns one or more file paths, or (b) something exists at `<worktree-path>/.factory` but is not a
directory (non-directory inode = stray shadow content at the path itself — see non-directory-path
paragraph above). Step G MUST NOT proceed with `git worktree remove`. It MUST:

1. Log each stray file path with the message:
   ```
   PREFLIGHT BLOCKED: Found factory artifact(s) in story worktree shadow .factory/:
     <path1>
     <path2>
     ...
   These files were written to the wrong worktree (issue #523 class) and would be
   permanently destroyed by git worktree remove. Manual intervention required:
     Option A: Relocate to canonical .factory/ mount, verify content, then retry teardown.
     Option B: Discard (only if files are confirmed redundant copies already committed on factory-artifacts).
   ```
2. Halt teardown. `git worktree remove` is NOT executed.
3. The story cleanup MUST NOT complete until the preflight returns an empty result.

**PC2c — Preflight error (fail-closed):** `find <worktree-path>/.factory -type f` exits non-zero
for any reason (note: path nonexistence is unreachable here — step 1 has already confirmed that
`.factory/` exists and is a directory before `find` is invoked; any non-zero exit therefore
signals a genuine error such as permission denial or path traversal error). Step G MUST HALT. The
exact `find` exit code and stderr output MUST be surfaced to the operator. `git worktree remove`
is NOT executed. The preflight is a destructive-operation gate; `find` errors must not silently
authorize `rm -rf`.

**INV-E21-004 instantiation:** `git worktree remove` on a story worktree MUST be preceded by an
empty-`.factory/` assertion.

## Invariants

1. **INV-E21-002 (Factory Artifact Write Canonical-Path Discipline):** Any agent writing to a
   factory artifact MUST use an absolute path anchored to the canonical main-checkout `.factory/`
   mount. Worktree-relative paths are categorically forbidden for `.factory/**` writes. The
   _shared-context.md `§Spec-Path Discipline` rule (which covers reads) MUST be extended to
   cover writes explicitly, naming the DELIVERY ledger and pr-review.md as load-bearing cases.

2. **INV-E21-004 (Story Worktree Teardown Preflight):** `git worktree remove` on a story worktree
   MUST be preceded by the `find <worktree>/.factory -type f` inventory check. No exceptions —
   not even when the agent is confident no `.factory/` writes occurred (the preflight is the
   mechanical gate, not agent confidence).

3. **Canonical root determination is orchestrator-provided or git-derived, never CWD-assumed.**
   The canonical `.factory/` root MUST be determined via `git -C <worktree-path> rev-parse --show-toplevel`
   on the MAIN worktree (not the story worktree, which would return the story worktree root) or
   from `CANONICAL_FACTORY_ROOT`. CWD-relative resolution is the root cause of issue #523 and
   is forbidden.

4. **Write discipline covers all factory-artifact categories.** The mandate applies to: DELIVERY
   ledgers (`*-DELIVERY.md`), story-frontmatter files, pr-review.md records, STATE.md updates,
   VP anchor files, and any other file under `.factory/**`. It is NOT limited to DELIVERY ledgers.

5. **The gitignored mechanism is the primary failure mode; `find` is the only gate that catches it.**
   Git's `git worktree remove` clean-state check gates on untracked files only — gitignored files
   are explicitly excluded. Because `.factory/` is gitignored on the product branch, the shadow
   `.factory/` content inside the story worktree is gitignored, not untracked. The clean-state
   check therefore passes silently (false negative) regardless of the shadow tree's contents, and
   the underlying `rm -rf` destroys it without warning. The `find <worktree>/.factory -type f`
   preflight is load-bearing because `find` reads the filesystem without gitignore filtering —
   it is the only mechanism that surfaces this category of stray content before destruction.
   No alternative git-level check (git status, git ls-files) would catch gitignored content in
   this scenario.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent writes `Write(".factory/stories/S-021-DELIVERY.md")` with CWD = `.worktrees/S-021/` | FORBIDDEN: relative path resolves to shadow tree; violates PC1 (INV-E21-002) |
| EC-002 | Agent writes `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | CORRECT: absolute path anchored to canonical root; PC1 compliant |
| EC-003 | `find .worktrees/S-021/.factory -type f` returns empty | Teardown proceeds (PC2a) |
| EC-004 | `find .worktrees/S-021/.factory -type f` returns `.worktrees/S-021/.factory/stories/S-021-DELIVERY.md` | BLOCKED: PC2b; relocate or discard then retry |
| EC-005 | Story worktree has nothing at the `.factory/` path (`[ ! -e ]` is true; clean worktree) | Path nonexistent → PC2a sub-case (a): no stray files, teardown authorized. Path-nonexistent is not a PC2c error; it is the expected clean state. Distinguished from PC2c (non-path-absent `find` failures). Note: use `[ ! -e ]` not `[ ! -d ]` — a regular file at that path would incorrectly satisfy `[ ! -d ]` but IS stray shadow content (EC-008). |
| EC-006 | Agent correctly writes DELIVERY ledger to canonical path, but a prior agent also wrote to the shadow tree | Preflight catches the stray copy; step G blocked until resolved |
| EC-007 | pr-reviewer writes `pr-review.md` using a relative path from its CWD (story worktree) | FORBIDDEN: violates PC1; shadow-tree write; would be lost at teardown |
| EC-008 | Story worktree has `.factory` as a regular file (not a directory) at the worktree root | Non-directory inode at path → PC2b BLOCKED: file is stray shadow content subject to `rm -rf` destruction; list the path; do NOT run `find`; do NOT proceed with `git worktree remove`. `[ ! -d ".factory" ]` would be true (wrong, authorizes teardown); `[ ! -e ".factory" ]` correctly identifies the path as occupied. |

## Canonical Test Vectors

| Test # | Precondition | Action | Expected Result |
|--------|-------------|--------|----------------|
| T-1 | Agent CWD = `.worktrees/S-021/` | `Write(".factory/stories/S-021-DELIVERY.md")` | FORBIDDEN: violates PC1 (caught by adversarial review or step check) |
| T-2 | Agent CWD = `.worktrees/S-021/` | `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | Correct: PC1 compliant; file lands on canonical mount |
| T-3 | Shadow tree empty | Step G: `find .worktrees/S-021/.factory -type f` | Returns empty; teardown proceeds |
| T-4 | Shadow tree has `S-021-DELIVERY.md` | Step G: `find .worktrees/S-021/.factory -type f` | Non-empty: teardown BLOCKED; PC2b error message |
| T-5 | Shadow tree has stray file; file relocated to canonical mount; re-run find | Step G retry: find returns empty | Teardown proceeds after relocation |
| T-6 | `.factory` exists as a regular file (not a directory) in story worktree | Step G: `[ ! -e .worktrees/S-021/.factory ]` is false; `[ ! -d .worktrees/S-021/.factory ]` would be true (wrong) | PC2b BLOCKED: non-directory inode at `.factory/` path listed as stray shadow content; `find` NOT invoked; teardown halted |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | _shared-context.md §Spec-Path Discipline clause explicitly covers `.factory/**` writes | manual: confirm clause present in S-21.04 skill-doc deliverable |
| (TBD) | Teardown preflight step present in step-g-cleanup.md | manual: confirm preflight sub-step in S-21.04 deliverable |
| (TBD) | Preflight blocks teardown when shadow `.factory/` contains files | bats: create stray `.factory/` file in test worktree fixture; invoke teardown; assert `git worktree remove` NOT called |
| (TBD) | Preflight passes when shadow `.factory/` is empty | bats: no stray files; assert teardown proceeds normally |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-036 |
| Capability Anchor Justification | CAP-036 registered in ARCH-INDEX v3.07 (ADR-031, commit 14a78515): "Story-Worktree Write-Path Discipline — factory artifact writes within story worktrees MUST target the canonical main-checkout `.factory/` mount via absolute paths; teardown preflight asserts shadow `.factory/` is empty before `git worktree remove`." BC-6.26.001 is the sole implementing BC for CAP-036 (INV-E21-002 + INV-E21-004). |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` (write-discipline clause extension); `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` (preflight sub-step addition) |
| Stories | S-21.04 (E-21 Wave 2) |
| Source Issues | #523 (story-worktree `.factory` artifacts silently lost at teardown) |
| ADR Reference | none |

## Related BCs

- BC-6.10.002 — deliver-story 9-step dispatch; this BC adds write-path discipline and teardown-preflight requirements to steps within that sequence
- BC-6.27.001 — sibling BC governing factory-side PR worktree lifecycle (shared-mutable-worktree class; different trigger and fix vector)

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — §Spec-Path Discipline to be extended with write-discipline clause (to be amended by S-21.04)
- `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` — teardown preflight sub-step to be added before `git worktree remove` dispatch (to be amended by S-21.04)

## Story Anchor

S-21.04 (E-21 Wave 2 — story-worktree factory artifact write-path discipline and teardown preflight)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.6 | 2026-07-25 | S-21.04 adv pass-4 fix burst F-S2104-P4-007 (product-owner). PC2a sub-case (a) discrimination predicate corrected from directory-ness (`[ ! -d ]`) to existence (`[ ! -e ]`): nothing-at-path → PC2a(a) proceed; non-directory inode (regular file, symlink-to-file) exists at path → PC2b BLOCKED (stray shadow content; same `rm -rf` destruction risk as files inside a shadow directory tree); existing directory → run `find` per PC2a(b)/PC2b/PC2c. TD-VSDD-060 within-file sweep: §Description numbered list steps 1/2/3 updated; non-directory-path paragraph added between PC2a and PC2b; PC2b header updated to cover non-directory case; PC2c parenthetical updated (path-nonexistence unreachable after pre-verification); EC-005 updated (path-nonexistent vs path-occupied distinction); EC-008 added (non-directory at path → PC2b BLOCKED); T-6 added (regular-file at `.factory/` path → PC2b BLOCKED). |
| 1.5 | 2026-07-24 | S-21.04 adv pass-2 fix burst F-002/O-005 (product-owner). F-002: §Description ~line 64 corrected — suppressed preflight command form `find <worktree-path>/.factory -type f 2>/dev/null` corrected to unsuppressed `find <worktree-path>/.factory -type f`; consistent with PC2a/PC2b/PC2c and v1.4 changelog claim "blanket `2>/dev/null` suppression removed." TD-VSDD-060 file-scope sweep: `grep -n "2>/dev/null" BC-6.26.001.md` — zero results on live preflight command (lines 26/35/280 are historical changelog text only). O-005: §Preconditions ¶2 caller-side alignment — callee-side phrasing ("Step G (devops-engineer cleanup step) is about to execute") corrected to caller-side ("The orchestrator is about to dispatch step G (devops-engineer cleanup)") per ADR-031 §Rationale (caller-side gating) and step-g-cleanup.md §G.1 (orchestrator-assigned gate). |
| 1.4 | 2026-07-24 | S-21.04 adv pass-1 fix burst F-004/006/007/010 (product-owner). F-007: §Description provenance corrected — `.factory/` directory absent at `git worktree add` time (gitignored on product branch); shadow created by errant write, not by checkout. F-010: `--force` prohibition rationale corrected in §Description ¶2 and PC2a — prohibition is a BC mandate (strips git's built-in protection for non-gitignored untracked files), not a guard-enforced constraint (guard permits `--force` for `.worktrees/`-containing commands). F-006: PC2a amended fail-closed — absent `.factory/` → PC2a sub-case (a); `find` error for non-path-absent reason → PC2c HALT; blanket `2>/dev/null` suppression removed; EC-005 updated. F-004: PC1 git command corrected — `<story-worktree-path>` → `<main-worktree-path>` (story-worktree `rev-parse --show-toplevel` returns story-worktree root, not canonical root; consistent with Invariant 3 + §Description). `CANONICAL_FACTORY_ROOT` defined: repo-root of main checkout (not `.factory/` mount). |
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #523; S-21.04). PC1: write-path discipline — all `.factory/**` writes MUST use canonical absolute paths anchored to main-checkout root (INV-E21-002). PC2a/PC2b: teardown preflight — `find <worktree>/.factory -type f` before `git worktree remove`; non-empty result blocks teardown (INV-E21-004). 4 invariants. 7 edge cases EC-001..EC-007. 5 test vectors T-1..T-5. lifecycle_status: draft (POL-14 auto-promotion on S-21.04 PR merge). |
| 1.3 | 2026-07-19 | adv pass-1 fix burst (F-P1-005) per ADR-031 v1.1 delta analysis v1.2 §Issue #523 (product-owner). §Description "Why --force requires a preflight" paragraph replaced: false --force premise removed; corrected mechanism documented — .factory/ is gitignored on story branch, shadow content is gitignored (not untracked), plain `git worktree remove` passes clean-state check as false negative (gitignored ≠ untracked for the check), rm-rf silently destroys shadow content; `find` is correct fix because it sees gitignored files. --force secondary note retained (clearly labeled). Invariant 5 replaced: gitignored mechanism as primary; `find` is only gate that catches it. PC2a corrected: `git worktree remove --force` → plain `git worktree remove`. |
| 1.2 | 2026-07-19 | Research validation precision amendments (product-owner; research validation 2026-07-19). §Description: preflight rationale added — --force mechanism (premise incorrect; corrected at v1.3). Invariant 5 added: --force stripping mechanism (corrected at v1.3 to gitignored mechanism). |
| 1.1 | 2026-07-19 | CAP-036 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-036; §Traceability L2 Capability TBD→CAP-036; Capability Anchor Justification updated to cite CAP-036/ARCH-INDEX v3.07. |
