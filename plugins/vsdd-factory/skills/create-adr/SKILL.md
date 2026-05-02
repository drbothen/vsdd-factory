---
name: create-adr
description: Scaffold a new ADR file from template, allocate collision-free ID, insert ARCH-INDEX row, patch any superseded ADR bidirectionally, and validate template compliance. Atomic-or-nothing execution.
argument-hint: "--title <text> --subsystems <SS-NN[,...]> [--supersedes <ADR-NNN>] [--brownfield] [--id <ADR-NNN>] [--dry-run]"

allowed-tools: Read, Write, Edit, Bash
---

## Hard Gate

Do NOT ghost-write ADR section prose. Template placeholder text is preserved verbatim. The skill scaffolds structure only — the architect agent fills content. Every write is atomic: if any step fails, all prior writes in that invocation are reverted.

# Create ADR

Scaffold a new Architectural Decision Record (ADR) from `adr-template.md`, allocate the next sequential ADR-NNN ID, insert the ARCH-INDEX row, optionally patch a superseded ADR bidirectionally, and run `validate-template-compliance.sh` as the final gate.

## Arguments

| Flag | Required | Description |
|------|----------|-------------|
| `--title <text>` | YES | Human-readable decision title. Used as-is in the ARCH-INDEX row. Slug-derived for the filename. |
| `--subsystems <SS-NN[,SS-NN...]>` | YES | Comma-separated list of affected subsystem IDs from ARCH-INDEX Subsystem Registry (e.g., `SS-06,SS-08`). |
| `--supersedes <ADR-NNN>` | no | Existing ADR being superseded. Triggers bidirectional patch (sets `superseded_by` on the old ADR). Also implies `--brownfield`. |
| `--brownfield` | no | Injects a non-skippable warning comment in the `Source / Origin` section requiring implementation evidence. |
| `--id <ADR-NNN>` | no | Override auto-allocated ID. Refused if that ID already exists in the filesystem or ARCH-INDEX. |
| `--dry-run` | no | Allocates ID and validates inputs; prints proposed ID + slug; writes nothing. Required by VP-059 harness. |

## Process

### Step 1: Validate ARCH-INDEX has Architecture Decisions section

Read `ARCH-INDEX.md`. If `## Architecture Decisions` section header is absent, exit non-zero with message and write nothing.

### Step 2: Validate subsystems against ARCH-INDEX Subsystem Registry (BC-6.20.005)

Parse `--subsystems` list. For each `SS-NN` ID, confirm it appears in the ARCH-INDEX Subsystem Registry table. On unknown ID, exit non-zero with the invalid ID and the list of valid IDs. No files written.

Validation order (BC-6.20.008 invariant 5): `--title` first; subsystem registry second; supersedes existence third; ID allocation/override last.

### Step 3: Validate --supersedes exists (BC-6.20.006)

If `--supersedes ADR-NNN` supplied, confirm `decisions/ADR-NNN-*.md` exists. If not found, exit non-zero with the missing ID. No files written.

### Step 4: Allocate ADR ID (BC-6.20.001)

Scan both:
- Filesystem: `decisions/ADR-[0-9][0-9][0-9]-*.md` filenames
- ARCH-INDEX: `## Architecture Decisions` table rows (parse `ADR-NNN` from each `| ADR-NNN |` row)

Take the maximum of all found numbers across both sources. Propose `ADR-<max+1>` zero-padded to three digits (e.g., max=013 → propose ADR-014).

If the filesystem ID set and the ARCH-INDEX ID set are NOT identical (mismatch), report the inconsistency and exit non-zero. No files written. (BC-6.20.003)

If `--id ADR-NNN` supplied: confirm it does not already exist in either source. If it does, exit non-zero with "ADR-NNN already exists at decisions/ADR-NNN-*.md. Omit --id to auto-allocate or choose a free ID." (BC-6.20.002)

Derive `<slug>` from `--title`: lowercase; whitespace runs → single `-`; strip all non-`[a-z0-9-]` characters (including non-ASCII — strip, do NOT transliterate); collapse consecutive `-`; trim leading/trailing `-`. The original title is used unchanged in the ARCH-INDEX row. (BC-6.20.008)

### Step 5: Dry-run exit

If `--dry-run` flag supplied, print `Dry-run: proposed ADR ID = ADR-NNN, slug = <slug>` and exit 0. No writes.

### Step 6: Write ADR file (BC-6.20.004, BC-6.20.009)

Render the new ADR file from `adr-template.md`:

| Frontmatter field | Value |
|-------------------|-------|
| `document_type` | `adr` (literal) |
| `adr_id` | `ADR-NNN` (allocated ID) |
| `status` | `proposed` (always at creation — never `accepted`) |
| `date` | today's ISO-8601 date (YYYY-MM-DD) |
| `subsystems_affected` | YAML array from `--subsystems` list |
| `supersedes` | `ADR-NNN` if `--supersedes` supplied; `null` otherwise |
| `superseded_by` | `null` (always at creation) |

Section bodies (Context, Decision, Rationale, Consequences, Alternatives Considered, Source / Origin) are copied VERBATIM from the template — no ghost-writing, no prose generation.

If `--brownfield` flag is set OR `--supersedes` is set (implies brownfield), inject the following non-skippable warning comment immediately after the `## Source / Origin` header line (BC-6.20.010):

```markdown
<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->
```

Write file to `decisions/ADR-NNN-<slug>.md`. If write fails, exit non-zero. No ARCH-INDEX row inserted yet.

### Step 7: Bidirectional supersession patch (BC-6.20.007)

If `--supersedes ADR-NNN` supplied, patch the frontmatter of `decisions/ADR-NNN-<existing-slug>.md` by setting `superseded_by: ADR-<new>` (replacing the existing `superseded_by:` line only — no other content changed).

Check file writeability before attempting the patch. If the old ADR is not writable, trigger rollback (Step 9 path A): delete the newly written ADR file, exit non-zero.

The patch is applied via direct write to the existing file (not a rename) so that filesystem permission checks are enforced correctly.

### Step 8: ARCH-INDEX row insertion (BC-6.20.008)

Insert one new row into the `## Architecture Decisions` table, positioned immediately after the row for the current highest ADR, maintaining ascending numeric order.

Row format:
```
| ADR-NNN | <decision-title> | <subsystems joined by ", "> | decisions/ADR-NNN-<slug>.md |
```

The `<decision-title>` is the original unsanitized `--title` value. The `<slug>` matches the filename.

Write directly to `ARCH-INDEX.md` (not via temp-file rename) so that a read-only ARCH-INDEX fails the write and triggers rollback (Step 9 path B): revert supersession patch (if applied), delete the new ADR file, exit non-zero.

If ARCH-INDEX lacks the `## Architecture Decisions` section (caught in Step 1, but re-checked here as a defense), exit non-zero.

### Step 9: Run validate-template-compliance.sh (BC-6.20.011)

Execute:
```
${VALIDATE_BIN} decisions/ADR-NNN-<slug>.md
```

If exit code 0: print `Template compliance: PASS` and proceed to Step 10.

If exit code non-zero: print any output from the validator, then print:
```
Template compliance: FAIL — ADR-NNN not registered. Fix the issues above and re-run.
```
Then apply partial rollback (BC-6.20.012 EC-002):
- Revert ARCH-INDEX row insertion
- Revert supersession patch (if applied)
- Leave the ADR file on disk (intentional — the implementer inspects it to fix the issues)
Exit non-zero.

### Step 10: Emit structured event (BC-6.20.009)

Emit event via `bin/emit-event`:
```
emit-event type=adr.scaffolded adr_id=ADR-NNN slug=<slug> subsystems=<csv> path=decisions/ADR-NNN-<slug>.md
```

Event is failure-tolerant (always exit 0 per emit-event contract). Failure to emit does NOT fail the skill.

### Step 11: Print guidance block to stdout

```
Template compliance: PASS

ADR-NNN scaffolded at: decisions/ADR-NNN-<slug>.md

Sections to flesh out:
  - Context      (2-5 paragraphs: background, forces, constraints)
  - Decision     (1-3 paragraphs: the choice itself)
  - Rationale    (2-5 paragraphs: why this, not alternatives)
  - Consequences (Positive / Negative sub-headings)
  - Alternatives Considered (top 2-3 options rejected)
  - Source / Origin (MUST cite implementation evidence for brownfield ADRs)

Recommended next step:
  Spawn architect agent: "Flesh out ADR-NNN sections. File: .factory/specs/architecture/decisions/ADR-NNN-<slug>.md"
```

Exit 0.

## Atomicity Contract (BC-6.20.012)

The skill treats all writes as a single atomic unit. If any step fails:

| Failure point | Revert actions |
|--------------|----------------|
| ADR file write fails | Nothing to revert. Exit non-zero. |
| Supersession patch fails | Delete new ADR file. Exit non-zero. |
| ARCH-INDEX insertion fails | Revert supersession patch (if applied). Delete new ADR file. Exit non-zero. |
| Validation fails | Revert ARCH-INDEX row. Revert supersession patch (if applied). Leave ADR file on disk. Exit non-zero. |

Revert messages are always specific: e.g., "Deleted ADR-NNN-slug.md", "Reverted ARCH-INDEX row for ADR-NNN", "Restored ADR-MMM superseded_by to null".

The skill never exits 0 after a partial-state failure.

After a full rollback (all paths except validation failure), re-invocation with the same arguments succeeds (idempotent). After a validation failure (file left on disk), re-invocation triggers the duplicate-ID check and exits non-zero — user must delete the leftover file first.

## Edge Cases

| ID | Scenario | Behavior |
|----|----------|----------|
| EC-001 | `decisions/` directory does not exist | Skill creates it before scanning; filesystem count treated as zero |
| EC-002 | ARCH-INDEX has ADR-013 in table but no file exists | Mismatch detected; skill refuses; user reconciles manually |
| EC-003 | `--title` contains special characters (`/`, `<`, `>`) | Slug strips them; title in ARCH-INDEX row uses original unsanitized title |
| EC-004 | `--supersedes ADR-NNN` already has `superseded_by` set | Skill warns "ADR-NNN is already superseded by ADR-MMM" but does NOT block |
| EC-005 | `validate-template-compliance.sh` not found or not executable | Exit non-zero: "validate-template-compliance.sh not found at expected path — cannot complete AC-7" |
| EC-006 | `--subsystems` contains unknown SS-ID (e.g., `SS-99`) | Exit non-zero with list of valid SS-IDs from ARCH-INDEX Subsystem Registry |
| EC-007 | Concurrent invocations write same ADR-NNN | Loser detects collision via duplicate-ID check and rolls back per BC-6.20.012. Users SHOULD serialize. |
| EC-008 | No arguments supplied | Print usage summary; exit 0 without writing anything |
| EC-009 | `--title` absent with other args present | Exit non-zero with usage error before any other argument is processed; no side-effects |
| EC-010 | ARCH-INDEX has uncommitted local edits | Warn via stderr: "WARNING: ARCH-INDEX.md has uncommitted local changes..." then proceed |

## Output Files

| File | Action | Notes |
|------|--------|-------|
| `.factory/specs/architecture/decisions/ADR-NNN-<slug>.md` | created | New ADR file scaffolded from template |
| `.factory/specs/architecture/ARCH-INDEX.md` | modified | New row inserted in Architecture Decisions table |
| `.factory/specs/architecture/decisions/ADR-MMM-<old-slug>.md` | modified (if `--supersedes`) | `superseded_by` field patched to `ADR-NNN` |

## Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `DECISIONS_DIR` | `.factory/specs/architecture/decisions/` | Path to decisions directory (overridable for tests) |
| `ARCH_INDEX` | `.factory/specs/architecture/ARCH-INDEX.md` | Path to ARCH-INDEX (overridable for tests) |
| `ADR_TEMPLATE` | `plugins/vsdd-factory/templates/adr-template.md` | Path to ADR template (read-only) |
| `VALIDATE_BIN` | `plugins/vsdd-factory/bin/validate-template-compliance.sh` | Path to validation script (mockable via `MOCK_VALIDATE_EXIT`) |

## Self-Review (before delivery)

1. Does the new ADR file contain `status: proposed`? (Never `accepted` at creation.)
2. Does the ARCH-INDEX row use the original unsanitized title (not the slug)?
3. Is the brownfield annotation present when `--brownfield` or `--supersedes` was supplied?
4. Did `validate-template-compliance.sh` exit 0?
5. Is the guidance block printed to stdout?
