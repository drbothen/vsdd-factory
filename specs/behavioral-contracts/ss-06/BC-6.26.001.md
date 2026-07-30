---
document_type: behavioral-contract
level: L3
version: "1.18"
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
  - "2026-07-25 (v1.7) — S-21.04 pass-5 F-S2104-P5-011/F-P5-009/F-P5-010 spec side (product-owner): F-011: discrimination chain amended — step 2 added as explicit `[ -L ]` symlink guard before any `[ -d ]` test; symlink-to-directory at path → PC2b BLOCKED (find NOT invoked); POSIX test -d follows symlinks; POSIX find without -H/-L does not descend symlinks → empty output → false PC2a escape documented and closed. F-009/F-010: PC2b condition tightened to 'find returns paths OR symlink/non-directory inode occupies the path'; PC2c unreachability note extended (symlink-at-path ruled out by step 2). Trailing-slash find form mandated throughout as defense-in-depth. Non-directory paragraph renamed and expanded to cover symlinks. EC-008 expanded to cover symlink-to-directory. T-7 added (symlink-to-dir → PC2b, find NOT invoked, remove NOT called). Invariant 2 and 5 updated."
  - "2026-07-25 (v1.8) — S-21.04 pass-6 F-S2104-P6-005(b) executor-side verification precondition (product-owner; ADR-031 §Rationale adjudication): §Preconditions gains Precondition 3 — executor-side trigger for devops-engineer before executing git worktree remove; Invariant 2 extended with executor-side clause covering both obligation surfaces (Preconditions 2+3 parity). Caller-side gating PRIMARY per ADR-031 §Rationale. AC-008."
  - "2026-07-25 (v1.9) — S-21.04 pass-7 F-S2104-P7-006 count-free case labels (product-owner; D-902 L-BB-count-bearing-crossref-residue-class): PC2 lead-in 'Three cases:' replaced with count-free form (class-death at the definition site). §Description numbered steps 1–4 retain their inline count (adjacent-to-enumeration acceptable per class-death convention). No other count-bearing case/branch/step labels found in sweep."
  - "2026-07-25 (v1.10) — S-21.04 pass-8 F-S2104-P8-003 ADR Reference traceability row (product-owner): §Traceability ADR Reference corrected from 'none' to ADR-031 §Decision 4 + §Rationale; document carries two live ADR-031 §Rationale anchors (Precondition 3, Invariant 2) and CAJ row also cites ADR-031. Class-bounded sweep: no other traceability/metadata row contradicts body anchors. Sibling BC-6.27.001 amended v1.3→v1.4 for same defect class in same burst."
  - "2026-07-25 (v1.11) — S-21.04 pass-10 F-S2104-P10-005 architecture-surface traceability completion (product-owner): §Traceability Architecture Module row and §Architecture Anchors extended to name all five obligation surfaces with obligation classes. v1.10 attestation gap acknowledged in changelog (error-acknowledgment discipline; v1.10 entry not rewritten)."
  - "2026-07-27 (v1.12) — S-21.04 pass-22 F-S2104-P22-002 self-contradiction fix (product-owner; D-933): two mutually exclusive claims about the mandated trailing-slash find command corrected at six body sites — (1) 'POSIX find without -H/-L does not descend symlinks → empty output' applies only to the no-trailing-slash form; the BC mandates the trailing-slash form (find \"<path>/\") which dereferences a symlink-to-directory via POSIX pathname resolution — find enumerates target files outside worktree boundary (out-of-scope traversal); corrected in §Description step 2, §Postconditions non-directory/symlink-path paragraph, §Invariant 5(a); (2) 'rm -rf destroys the symlink target' is empirically false — recursive-remove removes only the symlink entry, target is untouched; corrected in §Description step 2, §Postconditions non-directory/symlink-path paragraph, EC-008 contrast, T-7 contrast; (3) trailing-slash defense-in-depth claim removed from §Description step 3 parenthetical — trailing slash dereferences rather than protects; [ -L ] guard in step 2 is the actual protection. Changelog v1.7 entries preserved per append-only/error-acknowledgment policy."
  - "2026-07-28 (v1.13) — S-21.04 pass-23 F-S2104-P23-005 un-swept seventh site (product-owner; D-936): EC-005 note corrected — retracted claim 'falls through to find which returns empty → false PC2a' replaced with verified mechanism: POSIX test -d follows symlinks (symlink-to-directory satisfies [ -d ] → [ ! -d ] is false → falls through to find); trailing-slash form dereferences symlink-to-directory via POSIX pathname resolution, enumerating files from target directory outside worktree boundary (out-of-scope traversal); empty target yields false PC2a(b) → teardown proceeds; symlink entry removed without operator notification; no target data loss (recursive-remove does not follow symlinks). Consistent with EC-008 contrast and T-7 contrast corrected at v1.12."
  - "2026-07-28 (v1.14) — S-21.04 pass-25 F-S2104-P25-L01 changelog row ordering (product-owner; D-937): ## Changelog table v1.0 row moved to table bottom (was misplaced between v1.4 and v1.3). Full-table row sweep: no other out-of-order or duplicated rows found."
  - "2026-07-28 (v1.15) — S-21.04 pass-27 F-S2104-P27-M01/M02/M03 find-form adjudication (product-owner; D-940): M01: trailing-slash mandate retracted; plain-path form `find \"<path>/.factory\"` adopted; rationale in §Description step 2 + Invariant 5(a). M02: EC-003, EC-004, T-3, T-4 normalized to quoted plain-path form. M03: predicate widened from `-type f` to `! -type d` at all normative sites; EC-009 added; contrast prose updated. TD-VSDD-060 sweep: all 14 live `find` sites updated; changelog/historical sites preserved per append-only policy."
  - "2026-07-29 (v1.16) — S-21.04 pass-28 F-S2104-P28-009 frontmatter modified-array ordering (product-owner): v1.15 entry inserted before v1.14 (lines 36–37 transposed); corrected to ascending monotonic order. v1.14 sweep was scope-limited to the ## Changelog table only — the frontmatter modified: array was not covered; pass-27 re-opened the defect class one structure over. Dual-structure sweep complete: ## Changelog rows v1.16–v1.0 monotonic descending (CLEAN); modified: array v1.0–v1.16 monotonic ascending (CLEAN). Error-acknowledgment: v1.14 Changelog entry not rewritten; scope limitation documented in v1.16 Changelog row per append-only/error-acknowledgment policy. Ruling: governance BC is the appropriate home for the monotonic-ordering invariant; gate specification in ruling report."
  - "2026-07-29 (v1.17) — S-21.04 pass-28 F-S2104-P28-006 EC-009 test-coverage cross-reference (product-owner): EC-009 Expected Behavior extended with T-010/RG-010 coverage annotation; T-010 row added to §Canonical Test Vectors. Coverage partial — proven: symlink (type l) and FIFO (type p) by bats T-010 (POSIX mkfifo, macOS + Linux CI portable); unproven: socket (type s; needs bound process) and device node (types b/c; mknod needs root on Linux). No normative predicate or postcondition changed."
  - "2026-07-30 (v1.18) — S-21.04 pass-29 F-S2104-P29-M01 numeric finding-ID namespace correction (product-owner): IDs `F-S2104-P28-006` and `F-S2104-P28-009` cited in v1.17 and v1.16 modified-array entries (and corresponding ## Changelog rows) were dispatch artifacts from the pass-28 numeric dispatch convention — they do not exist in the authoritative pass-28 finding set (B01, H01–H07, M01–M07, L01, L02). Canonical mappings: `F-S2104-P28-006` → `F-S2104-P28-H05` (EC-009 had no test or Red Gate row); `F-S2104-P28-009` → `F-S2104-P28-M01` (frontmatter modified-array ordering). Prior entries preserved per append-only/error-acknowledgment discipline; this entry is the authoritative namespace-correction record. No normative content changed."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.26.001
section: "6.26"
last_amended: "(v1.18) — S-21.04 pass-29 F-S2104-P29-M01 numeric ID namespace correction (product-owner): F-S2104-P28-006→H05, F-S2104-P28-009→M01; dispatch artifacts; prior entries preserved. [Prior: (v1.17) — F-S2104-P28-006. (v1.16) — F-S2104-P28-009. (v1.15) — F-S2104-P27-M01/M02/M03. (v1.14) — F-S2104-P25-L01. (v1.13) — F-S2104-P23-005. (v1.12) — F-S2104-P22-002. (v1.11) — F-S2104-P10-005. (v1.10) — F-S2104-P8-003. (v1.9) — F-S2104-P7-006. (v1.8) — F-S2104-P6-005(b). (v1.7) — F-S2104-P5-011/009/010. (v1.6) — F-S2104-P4-007. (v1.5) — F-002/O-005. (v1.4) — F-004/006/007/010. (v1.3) — F-P1-005. (v1.2) — research. (v1.1) — CAP-036. (v1.0) — Initial.]"
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

The teardown preflight (`find "<worktree-path>/.factory" ! -type d`) is the correct fix
precisely because `find` reads the filesystem directly — it sees gitignored files and other
non-directory inodes that git's clean-state check ignores. The preflight is the only mechanism
that catches this class of loss.

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
   as no-stray-files (PC2a sub-case a) — teardown authorized immediately.
2. If a symlink exists at that path (`[ -L "<worktree-path>/.factory" ]`), OR if something exists
   but is NOT a real directory (regular file, symlink-to-file, symlink-to-directory, device node,
   or any other non-real-directory inode), treat as stray shadow content — proceed directly to
   PC2b BLOCKED (list the path; do NOT run `find`; do NOT proceed with `git worktree remove`).
   The `[ -L ]` test MUST precede any `[ -d ]` test: POSIX `test -d` follows symlinks, so a
   symlink-to-directory satisfies `[ -d ]` and would otherwise fall through to the `find` branch.
   Plain-path `find` without `-H`/`-L` lstats its entries; a symlink-to-directory has inode type
   `l`, not `f`, so the prior `-type f` predicate returned empty output — false PC2a(b) — and
   teardown would proceed, silently removing the symlink entry without operator notification (no
   target data loss since recursive-remove does not follow symlinks). The corrected `! -type d`
   predicate catches the symlink itself (type `l` satisfies `! -type d`), yielding a PC2b BLOCKED
   signal even if the `[ -L ]` guard were bypassed — but the `[ -L ]` guard remains the normative
   protection, routing symlinks to PC2b before `find` is invoked at all.
3. If `.factory/` exists and IS a real directory (not a symlink), run
   `find "<worktree-path>/.factory" ! -type d` (no blanket error suppression). A `find` exit error for any reason (e.g., permission denial, path error) MUST
   HALT teardown — this is a fail-closed error (PC2c); do not proceed with `git worktree remove`.
4. Empty `find` output → no non-directory content (PC2a sub-case b — real directory exists, no
   stray files, symlinks, FIFOs, or other non-directory inodes).
   Non-empty output → stray non-directory content found (PC2b-blocked).

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

### Executor-side verification precondition

3. The devops-engineer agent is about to execute `git worktree remove` on a story worktree path
   (`.worktrees/<STORY-ID>/`). Before issuing the command, the agent MUST have obtained a PASS
   result from the §G.1 preflight procedure defined in step-g-cleanup.md §G.1. If a PASS result
   is not evident from the dispatch context, the agent MUST execute the §G.1 procedure by
   reference — the discrimination-chain logic (existence check → symlink guard → `find` inventory)
   is defined solely in step-g-cleanup.md §G.1; this precondition references that procedure by
   name only and does not reproduce it. Stable anchors: ADR-031 §Rationale
   (verification-and-delegation; caller-side gating PRIMARY), AC-008, Invariant 2.

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
The discrimination chain routes to exactly one of the following outcomes:

**PC2a — No stray files (teardown authorized):** Either (a) nothing exists at path
`<worktree-path>/.factory` (`[ ! -e "<worktree-path>/.factory" ]` is true — nothing to inspect or
destroy), or (b) `.factory/` exists as a real directory (not a symlink) AND
`find "<worktree-path>/.factory" ! -type d` succeeds with empty output. In both sub-cases no stray
non-directory content exists; teardown is authorized. Step G proceeds with plain `git worktree remove`
(no `--force` — `--force` is prohibited by this BC because it strips git's built-in
unclean-worktree protection for non-gitignored untracked files; this prohibition is a BC mandate,
not a guard-enforced constraint).

**Non-directory or symlink path (stray shadow content — routed to PC2b):** If a symlink exists at
`<worktree-path>/.factory` (regardless of target type — symlink-to-file, symlink-to-directory, or
dangling), OR if something exists at that path but is NOT a real directory (regular file, device
node, or other non-directory non-symlink inode), it constitutes stray shadow content — subject to
exactly the same `rm -rf` destruction risk as files inside a shadow `.factory/` directory tree.
Step G MUST proceed to PC2b BLOCKED, listing the path. `find` MUST NOT be invoked. The `-d` test
alone MUST NOT be used as the discriminator: POSIX `test -d` follows symlinks, so a
symlink-to-directory satisfies `[ -d ]`, causing it to fall through to the `find` branch. The
`[ -L ]` symlink guard must precede any `[ -d ]` test to ensure all symlinks (regardless of
target type) are routed to PC2b BLOCKED before `find` is invoked.

**PC2b — Stray factory artifacts found (teardown blocked):** Either (a) the `find` command
returns one or more non-directory inode paths (regular files, symlinks, FIFOs, or other
non-directory inodes inside the shadow tree), or (b) a symlink or non-directory inode occupies
`<worktree-path>/.factory` (any symlink regardless of target type, or any non-directory non-symlink
inode — see non-directory-or-symlink-path paragraph above). In either case, step G MUST NOT
proceed with `git worktree remove`. It MUST:

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

**PC2c — Preflight error (fail-closed):** `find "<worktree-path>/.factory" ! -type d` exits
non-zero for any reason (note: path nonexistence is unreachable at this point — step 1 has already
confirmed something exists at the path; symlink-at-path is also unreachable — step 2 has already
routed all symlinks to PC2b before `find` is invoked; any non-zero exit therefore signals a genuine
error such as permission denial or path traversal error). Step G MUST HALT. The exact `find` exit
code and stderr output MUST be surfaced to the operator. `git worktree remove` is NOT executed.
The preflight is a destructive-operation gate; `find` errors must not silently authorize `rm -rf`.

**INV-E21-004 instantiation:** `git worktree remove` on a story worktree MUST be preceded by an
empty-`.factory/` assertion.

## Invariants

1. **INV-E21-002 (Factory Artifact Write Canonical-Path Discipline):** Any agent writing to a
   factory artifact MUST use an absolute path anchored to the canonical main-checkout `.factory/`
   mount. Worktree-relative paths are categorically forbidden for `.factory/**` writes. The
   _shared-context.md `§Spec-Path Discipline` rule (which covers reads) MUST be extended to
   cover writes explicitly, naming the DELIVERY ledger and pr-review.md as load-bearing cases.

2. **INV-E21-004 (Story Worktree Teardown Preflight):** `git worktree remove` on a story worktree
   MUST be preceded by the full discrimination-chain preflight: existence check → symlink/non-directory
   guard (`[ -L ]` before any `[ -d ]` test) → `find "<worktree>/.factory" ! -type d` inventory
   check (for real directories only). No exceptions — not even when the agent is confident no
   `.factory/` writes occurred (the preflight is the mechanical gate, not agent confidence).
   This invariant covers both obligation surfaces: the caller-side dispatch gate (Precondition 2 —
   the orchestrator MUST gate step G dispatch on a PASS preflight result) and the executor-side
   execution gate (Precondition 3 — devops-engineer MUST verify or execute the §G.1 procedure
   before issuing `git worktree remove` when the PASS result is not evident from the dispatch
   context). The discrimination-chain logic is defined solely in step-g-cleanup.md §G.1.
   Verification-and-delegation per ADR-031 §Rationale; caller-side gating remains PRIMARY.

3. **Canonical root determination is orchestrator-provided or git-derived, never CWD-assumed.**
   The canonical `.factory/` root MUST be determined via `git -C <worktree-path> rev-parse --show-toplevel`
   on the MAIN worktree (not the story worktree, which would return the story worktree root) or
   from `CANONICAL_FACTORY_ROOT`. CWD-relative resolution is the root cause of issue #523 and
   is forbidden.

4. **Write discipline covers all factory-artifact categories.** The mandate applies to: DELIVERY
   ledgers (`*-DELIVERY.md`), story-frontmatter files, pr-review.md records, STATE.md updates,
   VP anchor files, and any other file under `.factory/**`. It is NOT limited to DELIVERY ledgers.

5. **The gitignored mechanism is the primary failure mode; the discrimination chain is the only gate that catches it.**
   Git's `git worktree remove` clean-state check gates on untracked files only — gitignored files
   are explicitly excluded. Because `.factory/` is gitignored on the product branch, the shadow
   `.factory/` content inside the story worktree is gitignored, not untracked. The clean-state
   check therefore passes silently (false negative) regardless of the shadow tree's contents, and
   the underlying `rm -rf` destroys it without warning. The discrimination chain preflight is
   load-bearing: (a) the `[ -L ]` symlink guard catches any symlink at the path before `find` is
   invoked (including symlink-to-directory, where plain-path `find "<worktree>/.factory" ! -type d`
   without `-H`/`-L` would lstat the path as type `l` — returning it as a PC2b BLOCKED signal —
   but the `[ -L ]` guard routes the symlink to PC2b without invoking `find` at all, providing
   fail-fast behavior); (b) the `find "<worktree>/.factory" ! -type d` check reads the filesystem
   without gitignore filtering and catches all non-directory stray content — regular files,
   symlinks, FIFOs, and other non-directory inodes — inside real shadow directories.
   No alternative git-level check (git status, git ls-files) would catch gitignored content in
   this scenario.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent writes `Write(".factory/stories/S-021-DELIVERY.md")` with CWD = `.worktrees/S-021/` | FORBIDDEN: relative path resolves to shadow tree; violates PC1 (INV-E21-002) |
| EC-002 | Agent writes `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | CORRECT: absolute path anchored to canonical root; PC1 compliant |
| EC-003 | `find ".worktrees/S-021/.factory" ! -type d` returns empty | Teardown proceeds (PC2a sub-case b) |
| EC-004 | `find ".worktrees/S-021/.factory" ! -type d` returns `.worktrees/S-021/.factory/stories/S-021-DELIVERY.md` | BLOCKED: PC2b; relocate or discard then retry |
| EC-005 | Story worktree has nothing at the `.factory/` path (`[ ! -e ]` is true; clean worktree) | Path nonexistent → PC2a sub-case (a): no stray files, teardown authorized. Path-nonexistent is not a PC2c error; it is the expected clean state. Distinguished from PC2c (`find` errors such as permission denial). Note: use `[ ! -e ]` not `[ ! -d ]` — a regular file at that path would satisfy `[ ! -d ]` (true, wrong: authorizes teardown), while a symlink-to-directory would NOT satisfy `[ ! -d ]` (POSIX `test -d` follows symlinks → false; falls through to `find`; with the corrected plain-path `find ! -type d` form, the symlink inode has type `l` — `! -type d` is true — so the symlink path appears in output → PC2b BLOCKED, operator notified; the `[ -L ]` guard provides fail-fast routing before `find` is invoked, but `find ! -type d` catches the symlink even if the guard were bypassed). The `[ -L ]` guard in step 2 handles the symlink-to-directory case (EC-008). |
| EC-006 | Agent correctly writes DELIVERY ledger to canonical path, but a prior agent also wrote to the shadow tree | Preflight catches the stray copy; step G blocked until resolved |
| EC-007 | pr-reviewer writes `pr-review.md` using a relative path from its CWD (story worktree) | FORBIDDEN: violates PC1; shadow-tree write; would be lost at teardown |
| EC-008 | Story worktree has `.factory` as a regular file OR any symlink (symlink-to-file, symlink-to-directory, or dangling) at the worktree root | Any non-real-directory inode at path → PC2b BLOCKED: stray shadow content subject to `rm -rf` destruction; list the path; do NOT run `find`; do NOT proceed with `git worktree remove`. For a regular file: `[ ! -e ]` is false; `[ -L ]` is false; path is not a real directory → step 2 routes to PC2b. For a symlink-to-directory: `[ ! -e ]` is false; `[ -L ]` is true → step 2 routes to PC2b immediately. Wrong approach (contrast): using only `[ ! -d ]` — a regular file satisfies it (true, wrong: authorizes teardown — this is the primary failure of `[ ! -d ]` alone); a symlink-to-directory does NOT satisfy it (test -d follows symlinks → false, falls through to find; with plain-path `find ! -type d`, the symlink has type `l` → `! -type d` is true → symlink path appears in output → PC2b BLOCKED, operator notified — `find ! -type d` provides defense-in-depth even if `[ ! -d ]` were used alone for the directory predicate; however `[ ! -d ]` is still wrong because it authorizes teardown for a regular file at the path). |
| EC-009 | Shadow `.factory/` exists as a real directory containing a stray symlink inside the tree (e.g., `<worktree>/.factory/stories/S-021-DELIVERY.md` is a symlink-to-file rather than a regular file) | `find "<worktree>/.factory" ! -type d` returns the symlink path (type `l` satisfies `! -type d`) → PC2b BLOCKED: operator notified, teardown halted. Prior `-type f` form missed this (type `l` ≠ `f` → empty output → false PC2a(b) → teardown authorized, symlink removed without notification; no target data loss since recursive-remove does not follow symlinks). Coverage (T-010, RG-010): bats T-010 verifies symlink (type `l`) and FIFO (type `p`) inode classes — both placed inside a real shadow directory (POSIX `mkfifo`; macOS + Linux CI portable). Unproven: socket (type `s`; requires a bound process) and device node (types `b`/`c`; `mknod` requires root on Linux). |

## Canonical Test Vectors

| Test # | Precondition | Action | Expected Result |
|--------|-------------|--------|----------------|
| T-1 | Agent CWD = `.worktrees/S-021/` | `Write(".factory/stories/S-021-DELIVERY.md")` | FORBIDDEN: violates PC1 (caught by adversarial review or step check) |
| T-2 | Agent CWD = `.worktrees/S-021/` | `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | Correct: PC1 compliant; file lands on canonical mount |
| T-3 | Shadow tree empty (real directory, no files or symlinks) | Step G: `find ".worktrees/S-021/.factory" ! -type d` | Returns empty; teardown proceeds (PC2a sub-case b) |
| T-4 | Shadow tree has `S-021-DELIVERY.md` (regular file) | Step G: `find ".worktrees/S-021/.factory" ! -type d` | Non-empty: teardown BLOCKED; PC2b error message |
| T-5 | Shadow tree has stray file; file relocated to canonical mount; re-run find | Step G retry: find returns empty | Teardown proceeds after relocation |
| T-6 | `.factory` exists as a regular file (not a directory) in story worktree | Step G discrimination: step 1 `[ ! -e ]` is false; step 2 `[ -L ]` is false; path is not a real directory → PC2b. (Wrong approach contrast: `[ ! -d ]` would be true → authorizes teardown.) | PC2b BLOCKED: non-directory inode at `.factory/` path listed as stray shadow content; `find` NOT invoked; teardown halted |
| T-7 | `.factory` exists as a symlink-to-directory in story worktree (symlink target is a real directory) | Step G discrimination: step 1 `[ ! -e ]` is false; step 2 `[ -L ]` is true → PC2b immediately. Without `[ -L ]` guard: `[ -d ]` is true (test -d follows symlinks) → falls through to `find`; plain-path `find ! -type d` without `-H`/`-L` lstats the starting path — the symlink-to-directory has type `l`, which satisfies `! -type d` → symlink path appears in output → PC2b BLOCKED, operator notified (defense-in-depth: `find ! -type d` catches the symlink even without the `[ -L ]` guard; prior `-type f` form would have returned empty output → false PC2a(b) → teardown authorized, symlink silently removed) | PC2b BLOCKED: symlink-to-directory at `.factory/` path listed as stray shadow content; `find` NOT invoked; `git worktree remove` NOT called |
| T-010 | Shadow `.factory/` is a real directory containing a stray symlink-to-file and a stray FIFO inside the tree (each placed at e.g. `<worktree>/.factory/stories/`; FIFO created via POSIX `mkfifo`) | Step G: `find ".worktrees/S-021/.factory" ! -type d` | Non-empty: returns symlink path (type `l` satisfies `! -type d`) and FIFO path (type `p` satisfies `! -type d`) → PC2b BLOCKED; operator notified, teardown halted. Reversion evidence: `find … -type f` returns empty for both — prior predicate missed both; `find … ! -type d` catches both. Coverage partial: symlink and FIFO proven; socket and device node unproven. (EC-009, RG-010) |

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
| Architecture Module | `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` (primary protocol — write-discipline clause extension); `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` (primary protocol — preflight sub-step addition); `plugins/vsdd-factory/agents/devops-engineer.md` (executor-side verification — Precondition 3); `plugins/vsdd-factory/agents/adversary.md` (awareness surface — Invariant 5 / story AC-009); `plugins/vsdd-factory/skills/adversarial-review/SKILL.md` (awareness surface — Invariant 5 / story AC-009) |
| Stories | S-21.04 (E-21 Wave 2) |
| Source Issues | #523 (story-worktree `.factory` artifacts silently lost at teardown) |
| ADR Reference | ADR-031 §Decision 4 (INV-E21-002 + INV-E21-004 skill-doc enforcement); ADR-031 §Rationale (caller-side gating PRIMARY; executor-side verification-and-delegation layer — Precondition 3, Invariant 2) |

## Related BCs

- BC-6.10.002 — deliver-story 9-step dispatch; this BC adds write-path discipline and teardown-preflight requirements to steps within that sequence
- BC-6.27.001 — sibling BC governing factory-side PR worktree lifecycle (shared-mutable-worktree class; different trigger and fix vector)

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — primary protocol: §Spec-Path Discipline to be extended with write-discipline clause (to be amended by S-21.04)
- `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` — primary protocol: teardown preflight sub-step to be added before `git worktree remove` dispatch (to be amended by S-21.04)
- `plugins/vsdd-factory/agents/devops-engineer.md` — executor-side verification (Precondition 3): devops-engineer MUST have obtained a PASS result from the §G.1 preflight procedure before issuing `git worktree remove` on a story worktree
- `plugins/vsdd-factory/agents/adversary.md` — awareness surface (Invariant 5 / story AC-009): adversary agent must recognize the gitignored-shadow mechanism and discrimination-chain protocol when reviewing story-worktree lifecycle operations
- `plugins/vsdd-factory/skills/adversarial-review/SKILL.md` — awareness surface (Invariant 5 / story AC-009): adversarial-review skill must incorporate the shadow-factory discrimination-chain as a known failure class when reviewing deliver-story step execution

## Story Anchor

S-21.04 (E-21 Wave 2 — story-worktree factory artifact write-path discipline and teardown preflight)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.18 | 2026-07-30 | S-21.04 pass-29 F-S2104-P29-M01 numeric finding-ID namespace correction (product-owner): IDs `F-S2104-P28-006` and `F-S2104-P28-009` cited in v1.17 and v1.16 changelog entries (and corresponding `modified:` array entries) were dispatch artifacts from the pass-28 numeric dispatch convention — they do not exist in the authoritative pass-28 finding set (B01, H01–H07, M01–M07, L01, L02). Canonical mappings: `F-S2104-P28-006` → `F-S2104-P28-H05` (EC-009 had no test or Red Gate row); `F-S2104-P28-009` → `F-S2104-P28-M01` (frontmatter `modified:` array ordering). Prior v1.16 and v1.17 entries preserved per append-only/error-acknowledgment discipline. No normative content changed. |
| 1.17 | 2026-07-29 | S-21.04 pass-28 F-S2104-P28-006 EC-009 test-coverage cross-reference (product-owner): EC-009 Expected Behavior extended to reference T-010 and RG-010; T-010 row added to §Canonical Test Vectors. Coverage partial: symlink (type `l`) and FIFO (type `p`) inode classes proven by bats T-010 (POSIX `mkfifo`; macOS + Linux CI portable); socket (type `s`) and device node (types `b`/`c`) unproven — socket requires a bound process, `mknod` requires root on Linux. No normative predicate or postcondition changed. |
| 1.16 | 2026-07-29 | S-21.04 pass-28 F-S2104-P28-009 frontmatter modified-array ordering (product-owner): v1.15 entry was inserted before v1.14 in the frontmatter modified: array (lines 36–37 transposed); corrected to ascending monotonic order. v1.14 sweep was scope-limited to the ## Changelog table only — the frontmatter modified: array was not covered by that sweep; pass-27 insertion re-opened the exact defect class one structure over. Dual-structure sweep now complete: ## Changelog table rows v1.16–v1.0 confirmed monotonic descending (CLEAN); modified: array v1.0–v1.16 confirmed monotonic ascending (CLEAN). Error-acknowledgment: v1.14 Changelog entry is not rewritten; its scope limitation is documented here per append-only/error-acknowledgment policy. Ruling: monotonic-ordering invariant warranted for both provenance structures; governance BC is the appropriate home (not this BC); gate specification provided in ruling report. |
| 1.15 | 2026-07-28 | S-21.04 pass-27 F-S2104-P27-M01/M02/M03 find-form adjudication (product-owner; D-940): M01 — trailing-slash mandate retracted; plain-path form adopted throughout; rationale: `[ -L ]` guard is the actual protection; for real directories (the only `find` input in correct execution) both forms produce identical output; with `! -type d` (M03), plain-path on a symlink-to-dir returns the symlink path itself (PC2b signal), whereas trailing-slash enumerates target contents (out-of-scope traversal). M02 — five non-normative sites (EC-003, EC-004, T-3, T-4, T-5) normalized to match normative form (no trailing slash, quoted paths). M03 — predicate widened from `-type f` to `! -type d` at all normative sites; rationale: symlink inside a real shadow directory has type `l`, invisible to `-type f`, visible to `! -type d`; BC's own harm criterion (EC-008/T-7: unexpected non-directory inode silently removed without operator notification) applies identically one level deeper; no false positives (empty shadow dir returns empty under both predicates). EC-009 added: symlink inside real shadow directory. Contrast prose in step 2 of §Description, §Postconditions non-directory paragraph, Invariant 5(a), EC-005/EC-008/T-7 updated to reflect plain-path `! -type d` behavior (defense-in-depth: catches symlink-to-dir even if `[ -L ]` guard bypassed). Gating: harness extraction gate in `story-worktree-write-path-discipline.bats` (pattern `-type f`) must be updated by test-writer; see downstream propagation list in pass-27 adjudication record. |
| 1.14 | 2026-07-28 | S-21.04 pass-25 F-S2104-P25-L01 changelog row ordering (product-owner; D-937): ## Changelog table v1.0 row corrected — was misplaced between v1.4 and v1.3; moved to table bottom to restore monotonic newest-first descending order. Full-table sweep verdict: v1.13–v1.4 correct monotonic descent (CLEAN); v1.3–v1.1 correct relative order after removal of misplaced v1.0 (CLEAN); no duplicate row version numbers found. Ordering/formatting-only change; no substantive row text altered. |
| 1.13 | 2026-07-28 | S-21.04 pass-23 F-S2104-P23-005 un-swept seventh site (product-owner; D-936): EC-005 note corrected — retracted claim 'falls through to find which returns empty → false PC2a' replaced with verified mechanism. POSIX `test -d` follows symlinks (symlink-to-directory satisfies `[ -d ]`, so `[ ! -d ]` is false → falls through to `find`); the mandated trailing-slash form dereferences the symlink-to-directory via POSIX pathname resolution, enumerating files from the target directory outside the worktree boundary (out-of-scope traversal that cannot reliably detect stray worktree content); an empty target yields false PC2a(b) → teardown proceeds; the symlink entry is removed without operator notification — no target data loss, since recursive-remove does not follow symlinks. Consistent with EC-008 contrast and T-7 contrast corrected at v1.12. Sweep confirmed: one live body site (EC-005). All other 'returns empty' / 'false PC2a' hits are historical changelog entries (preserved per append-only/error-acknowledgment policy) or already-corrected contrast text in EC-008/T-7. |
| 1.12 | 2026-07-27 | S-21.04 pass-22 F-S2104-P22-002 self-contradiction fix (product-owner; D-933): two mutually exclusive claims about the mandated trailing-slash `find` command corrected at six body sites. (1) 'POSIX find without -H/-L does not descend symlinks → empty output → false PC2a' — applies only to the no-trailing-slash form; the BC mandates `find "<path>/"` (trailing slash), which POSIX pathname resolution dereferences on a symlink-to-directory — find enumerates files from the target directory outside the worktree boundary (out-of-scope traversal), not empty output. Corrected in §Description step 2, §Postconditions non-directory/symlink-path paragraph, §Invariant 5(a). (2) 'rm -rf destroys the symlink target' — empirically false: recursive-remove removes only the symlink entry; target directory is untouched. Corrected in §Description step 2, §Postconditions non-directory/symlink-path paragraph, EC-008 contrast, T-7 contrast. (3) 'trailing slash is defense-in-depth against symlinks' — removed from §Description step 3 parenthetical; trailing slash on a real directory is harmless but would dereference (not protect against) a symlink that reached this branch; the [ -L ] guard in step 2 is the actual protection. [ -L ] guard, all postconditions, and invariant conclusions unchanged. Changelog v1.7 entries preserved per append-only/error-acknowledgment policy; v1.7 entries contain the now-corrected claims. |
| 1.11 | 2026-07-25 | S-21.04 pass-10 F-S2104-P10-005 architecture-surface traceability completion (product-owner): §Traceability Architecture Module row and §Architecture Anchors extended to name all five obligation surfaces with their obligation class — `_shared-context.md` (primary protocol — write-discipline clause extension); `step-g-cleanup.md` (primary protocol — preflight sub-step addition); `agents/devops-engineer.md` (executor-side verification — Precondition 3); `agents/adversary.md` (awareness surface — Invariant 5 / story AC-009); `skills/adversarial-review/SKILL.md` (awareness surface — Invariant 5 / story AC-009). Architecture-impact sweeps over devops-engineer.md, adversary.md, and adversarial-review/SKILL.md were previously unreachable from this BC. v1.10 attestation gap acknowledged: the v1.10 "no other traceability/metadata row contradicts body anchors" sweep claim is falsified by this finding — Architecture Module row and §Architecture Anchors listed only two of the five obligation surfaces that Precondition 3 (v1.8) and Invariant 5 + story AC-009 made normative; those omissions constituted metadata-body contradictions that the v1.10 sweep failed to catch (error-acknowledgment discipline; v1.10 entry is not rewritten). |
| 1.10 | 2026-07-25 | S-21.04 pass-8 F-S2104-P8-003 ADR Reference traceability row (product-owner): §Traceability ADR Reference corrected from 'none' to ADR-031 §Decision 4 + §Rationale — document carries two live ADR-031 §Rationale anchors (Precondition 3 "Stable anchors: ADR-031 §Rationale"; Invariant 2 "Verification-and-delegation per ADR-031 §Rationale") and Capability Anchor Justification row also cites ADR-031. Class-bounded sweep: no other traceability/metadata row contradicts body anchors. Sibling BC-6.27.001 confirmed same defect class (CAJ cites ADR-031; ADR Reference: none); fixed in same burst (v1.3→v1.4). |
| 1.9 | 2026-07-25 | S-21.04 pass-7 F-S2104-P7-006 count-free case labels (product-owner; D-902 L-BB-count-bearing-crossref-residue-class): PC2 lead-in "Three cases:" replaced with count-free form "The discrimination chain routes to exactly one of the following outcomes:" (class-death at the definition site). §Description numbered steps 1–4 retain their inline count — the STEP count is the enumeration itself (steps numbered 1–4 inline; adjacent-to-enumeration is acceptable per class-death convention). No other count-bearing case/branch/step labels found in sweep. |
| 1.8 | 2026-07-25 | S-21.04 pass-6 F-S2104-P6-005(b) executor-side verification precondition (product-owner; ADR-031 §Rationale adjudication — verification-and-delegation; AC-008). §Preconditions: Precondition 3 added — executor-side trigger for devops-engineer before executing `git worktree remove` on a story worktree; obligation: verify PASS §G.1 preflight result was obtained; when not evident from dispatch context, execute §G.1 by reference to step-g-cleanup.md §G.1 (discrimination-chain logic defined solely there; this precondition references it by name only). Invariant 2 extended — INV-E21-004 now covers both obligation surfaces: caller-side dispatch gate (Precondition 2) and executor-side execution gate (Precondition 3). No new PC or EC required: PC2 ("Step G MUST apply the fail-closed inventory protocol before any `git worktree remove` command") already mandates the executor-side behavior; Invariant 2 extension provides explicit parity naming. Caller-side gating PRIMARY per ADR-031 §Rationale. |
| 1.7 | 2026-07-25 | S-21.04 pass-5 F-S2104-P5-011/F-P5-009/F-P5-010 spec side (product-owner). F-011 (symlink-to-directory escape): discrimination chain amended — step 2 added as explicit `[ -L ]` symlink guard before any `[ -d ]` test; any symlink at `<worktree-path>/.factory` (regardless of target type) → PC2b BLOCKED; `find` NOT invoked; rationale documented: POSIX `test -d` follows symlinks (symlink-to-directory satisfies `[ -d ]`); POSIX `find` without `-H`/`-L` does not descend symlinks (returns empty → false PC2a(b) → `rm -rf` destroys symlink target). F-009/F-010 spec precision: PC2b condition = "find returns paths OR symlink/non-directory inode occupies the path"; PC2c unreachability note extended (symlink-at-path ruled out by step 2 before `find` is invoked). Trailing-slash find form `find "<path>/.factory/" -type f` mandated throughout (defense-in-depth; forces traversal entry). EC-008 expanded to cover symlink-to-directory explicitly (both regular-file and symlink cases; contrast text shows why `[ ! -d ]` alone fails for symlink-to-dir). T-7 added: symlink-to-dir → PC2b, find NOT invoked, remove NOT called. Invariant 2 updated to describe full discrimination chain. Invariant 5 title updated (discrimination chain, not just find). |
| 1.6 | 2026-07-25 | S-21.04 adv pass-4 fix burst F-S2104-P4-007 (product-owner). PC2a sub-case (a) discrimination predicate corrected from directory-ness (`[ ! -d ]`) to existence (`[ ! -e ]`): nothing-at-path → PC2a(a) proceed; non-directory inode (regular file, symlink-to-file) exists at path → PC2b BLOCKED (stray shadow content; same `rm -rf` destruction risk as files inside a shadow directory tree); existing directory → run `find` per PC2a(b)/PC2b/PC2c. TD-VSDD-060 within-file sweep: §Description numbered list steps 1/2/3 updated; non-directory-path paragraph added between PC2a and PC2b; PC2b header updated to cover non-directory case; PC2c parenthetical updated (path-nonexistence unreachable after pre-verification); EC-005 updated (path-nonexistent vs path-occupied distinction); EC-008 added (non-directory at path → PC2b BLOCKED); T-6 added (regular-file at `.factory/` path → PC2b BLOCKED). |
| 1.5 | 2026-07-24 | S-21.04 adv pass-2 fix burst F-002/O-005 (product-owner). F-002: §Description ~line 64 corrected — suppressed preflight command form `find <worktree-path>/.factory -type f 2>/dev/null` corrected to unsuppressed `find <worktree-path>/.factory -type f`; consistent with PC2a/PC2b/PC2c and v1.4 changelog claim "blanket `2>/dev/null` suppression removed." TD-VSDD-060 file-scope sweep: `grep -n "2>/dev/null" BC-6.26.001.md` — zero results on live preflight command (lines 26/35/280 are historical changelog text only). O-005: §Preconditions ¶2 caller-side alignment — callee-side phrasing ("Step G (devops-engineer cleanup step) is about to execute") corrected to caller-side ("The orchestrator is about to dispatch step G (devops-engineer cleanup)") per ADR-031 §Rationale (caller-side gating) and step-g-cleanup.md §G.1 (orchestrator-assigned gate). |
| 1.4 | 2026-07-24 | S-21.04 adv pass-1 fix burst F-004/006/007/010 (product-owner). F-007: §Description provenance corrected — `.factory/` directory absent at `git worktree add` time (gitignored on product branch); shadow created by errant write, not by checkout. F-010: `--force` prohibition rationale corrected in §Description ¶2 and PC2a — prohibition is a BC mandate (strips git's built-in protection for non-gitignored untracked files), not a guard-enforced constraint (guard permits `--force` for `.worktrees/`-containing commands). F-006: PC2a amended fail-closed — absent `.factory/` → PC2a sub-case (a); `find` error for non-path-absent reason → PC2c HALT; blanket `2>/dev/null` suppression removed; EC-005 updated. F-004: PC1 git command corrected — `<story-worktree-path>` → `<main-worktree-path>` (story-worktree `rev-parse --show-toplevel` returns story-worktree root, not canonical root; consistent with Invariant 3 + §Description). `CANONICAL_FACTORY_ROOT` defined: repo-root of main checkout (not `.factory/` mount). |
| 1.3 | 2026-07-19 | adv pass-1 fix burst (F-P1-005) per ADR-031 v1.1 delta analysis v1.2 §Issue #523 (product-owner). §Description "Why --force requires a preflight" paragraph replaced: false --force premise removed; corrected mechanism documented — .factory/ is gitignored on story branch, shadow content is gitignored (not untracked), plain `git worktree remove` passes clean-state check as false negative (gitignored ≠ untracked for the check), rm-rf silently destroys shadow content; `find` is correct fix because it sees gitignored files. --force secondary note retained (clearly labeled). Invariant 5 replaced: gitignored mechanism as primary; `find` is only gate that catches it. PC2a corrected: `git worktree remove --force` → plain `git worktree remove`. |
| 1.2 | 2026-07-19 | Research validation precision amendments (product-owner; research validation 2026-07-19). §Description: preflight rationale added — --force mechanism (premise incorrect; corrected at v1.3). Invariant 5 added: --force stripping mechanism (corrected at v1.3 to gitignored mechanism). |
| 1.1 | 2026-07-19 | CAP-036 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-036; §Traceability L2 Capability TBD→CAP-036; Capability Anchor Justification updated to cite CAP-036/ARCH-INDEX v3.07. |
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #523; S-21.04). PC1: write-path discipline — all `.factory/**` writes MUST use canonical absolute paths anchored to main-checkout root (INV-E21-002). PC2a/PC2b: teardown preflight — `find <worktree>/.factory -type f` before `git worktree remove`; non-empty result blocks teardown (INV-E21-004). 4 invariants. 7 edge cases EC-001..EC-007. 5 test vectors T-1..T-5. lifecycle_status: draft (POL-14 auto-promotion on S-21.04 PR merge). |
