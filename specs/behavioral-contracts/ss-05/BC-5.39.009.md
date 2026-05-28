---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-28T00:00:00Z
phase: section-12-step-3M4
cycle: brownfield-backfill
inputs:
  - .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-75.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md
  - .factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md
  - .factory/policies.yaml
  - plugins/vsdd-factory/hooks-registry.toml
input-hash: "TBD"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
extracted_from: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-05-28"
  - "2026-05-28 (v1.1)"
  - "2026-05-28 (v1.2)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.009
section: "5.39"
last_amended: "2026-05-28 (v1.2) — Pass-2 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-2 fix-burst). Closes F-S15.17-SP2-003 (HIGH EC-008 '(PC4)' → '(Precondition 4)'; full EC table swept for Pre-vs-Post abbreviation ambiguity), F-S15.17-SP2-004 (MEDIUM status:active→status:draft to match lifecycle_status:draft; adjudication: both fields must be draft pre-merge, POL-14 fires on S-15.17 merge; sibling BC-5.39.008 is active/active only because it has shipped — BC-5.39.009 is pre-merge so status:draft is correct), F-S15.17-SP2-005 (MEDIUM PC2/PC3/PC5 line-number anti-volatile-pin per TD-VSDD-091: stripped line-number citations from literal-shell grep outputs; kept grep command + prefix/content match excerpts only), F-S15.17-SP2-006 (MEDIUM §Cure-Extension Parsimony Note and inv-4 + Description rephrase: documented deliberate non-extension of BC-5.39.006 marker-prefix semantics — BC-5.39.006 conditions LENGTH check on 'trajectory-tail ' canonical marker prefix; BC-5.39.009 per-cell sites 2-9 are heterogeneous text contexts where that marker convention does not apply), F-S15.17-SP2-007 (MEDIUM Precondition 4 parent-guard for STATE.md arm: hook MUST verify file_path is rooted at .factory/ before triggering STATE.md arm; new EC-019 non-factory STATE.md case added), F-S15.17-SP2-008 (LOW PC3 skip-list: dropped 'COMPLETE' from skip list; skip only ARCHIVED/COMPACTED; alternative bottommost-row rationale documented), F-S15.17-SP2-009 partial (LOW ADR-021 dropped from §ADR References in Traceability — cargo-audit-specific, not general no-subprocess principle; story-writer to drop from anchored_adrs frontmatter), F-S15.17-SP2-010 (LOW inv-9 anti-volatile-pin rephrase: avoid SDK-state assertion; prescribe behavior instead), F-S15.17-SP2-011 (NITPICK D-453 pass-73 cite corrected from 'pass-74'). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-2 / (b) PC2/PC3/PC5 line-number anti-volatile-pin applied / (c) cure-extension parsimony note updated with marker-prefix non-extension documentation. POLICY 14 5-leg quintuple parity applied (version v1.2 + Changelog row v1.2 + modified[] appended '2026-05-28 (v1.2)' + this last_amended text-prefix v1.2 + BC-INDEX v2.56 row). Story-writer (next burst) handles F-001 (AC-9/10/11/12/17 re-anchor + bidirectional parity check stdout), F-002 (SS-05 narrative rewrite), F-003 story EC-008 mirror, F-007 AC-23 false-positive STATE.md, F-009 anchored_adrs drop. [Prior: 2026-05-28 (v1.1) — Pass-1 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-1 fix-burst). Closes F-S15.17-SP1-002 (HIGH frontmatter ADR-017 path corrected to ADR-017-per-story-adversary-phasing.md per POLICY 4 semantic_anchoring_integrity), F-S15.17-SP1-004 (HIGH PC2/PC3/PC5 STATE.md extractor anchors updated to match production STATE.md table-cell + heading structure with literal-shell evidence; META-LEVEL-30 route-(b) partial-recurrence-inside-cure-BC closed), F-S15.17-SP1-005 (HIGH inv-4 + PC1 LENGTH=4-strict adjudication aligning with BC-5.39.006 inv-6(b)+EC-007 + D-433(e)+D-439(c) original codification; cure-extension parsimony per D-497 cited; new EC-018 LENGTH=5 added), F-S15.17-SP1-007 (MEDIUM precondition-5/EC-016 fail-open contradiction reconciled to log_warn+Continue), F-S15.17-SP1-009 (MEDIUM path_allow sibling cite corrected to BC-5.39.006), F-S15.17-SP1-010 (MEDIUM on_error=continue invariant added per sibling BC-5.39.004/005/006/007/008 precedent), F-S15.17-SP1-011 (LOW D-NNN table purified — ADR/META-LEVEL/BC refs moved out), F-S15.17-SP1-012 (LOW D-454(a) PC range clarified to 1-10), F-S15.17-SP1-014 (NITPICK STATE.md capitalization fix). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-1 / (b) structural per-cell extractor-anchor specification / (c) cure-extension of BC-5.39.005+BC-5.39.006 pattern per D-497 parsimony. POLICY 14 5-leg quintuple parity applied (version v1.1 + Changelog row + modified[] + this last_amended + BC-INDEX v2.55 cell). Story-writer (next burst) handles F-001 type, F-003 AC mis-mapping, F-006 EC renumber, F-008 BC table claim, F-013 token budget. [Prior: 2026-05-28 (v1.0) — Initial authoring (product-owner; brownfield-backfill F5 pass-75 HIGH-002 anchor; META-LEVEL-30 route (b) cure). Anchors ADV-EDP1-P75-HIGH-002. BC-5.39.009 allocated as next monotonic ID after BC-5.39.008 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.17 merge).]"
---

# BC-5.39.009: validate-trajectory-tail-cell-completeness WASM hook MUST block on STATE.md writes missing trajectory_tail in any of the 5 prescribed STATE.md cells, and MUST emit advisory on INDEX.md / burst-log.md / lessons.md writes missing trajectory_tail in their prescribed cells

## Description

The `validate-trajectory-tail-cell-completeness` WASM hook enforces per-cell presence of the
canonical trajectory_tail arrow-sequence (`(→[0-9]+){4}`, LENGTH=4 per D-433(e)) at PostToolUse
on writes to the four target artifact files that carry D-453(d)-prescribed trajectory_tail sites.
This hook closes META-LEVEL-30 route (b): "codified-canonical-registry-with-per-cell-prescribed-
sites-BUT-no-runtime-WASM-gate-enforcing-each-site." D-453(d) codified a canonical 9-site mapping
table at pass-73, but the 14-day F5 pause demonstrated that codification without a runtime gate
permits silent cell-level degradation between passes (ADV-EDP1-P75-HIGH-002).

The hook has two severity arms: STATE.md (sites 1-5, Block severity) and non-STATE.md (sites 6-9,
advisory severity). Block severity on STATE.md reflects the finding that these 5 cells are the
primary sites where trajectory_tail omission causes HIGH adversary findings (per D-411(a)). Advisory
severity for INDEX.md/burst-log.md/lessons.md reflects that these cells are enforced by the same
state-manager burst and their omission is detectable but does not independently cause trajectory
misreading by agents reading STATE.md.

The D-453(d) canonical mapping table prescribes 11 sites total. Of these, 9 are mechanically
checkable via PostToolUse on their containing files. Two are excluded from this hook's scope:
`decision-log.md trajectory-bearing rows` (advisory-only per the D-453(d) registry; sparse, not
every row carries a tail) and `adv-cycle-pass-*.md frontmatter trajectory_tail field` (written by
the adversary agent, not by state-manager; out-of-scope for a state-manager PostToolUse gate).

### D-453(d) 11-Site Canonical Mapping (with hook scope)

| Site # | Document | Section/Field | Hook Scope | Severity |
|--------|----------|---------------|------------|----------|
| 1 | STATE.md | frontmatter `current_step:` | IN SCOPE | Block |
| 2 | STATE.md | Last Updated cell | IN SCOPE | Block |
| 3 | STATE.md | Phase Progress rows (latest pass) | IN SCOPE | Block |
| 4 | STATE.md | Concurrent Cycles row | IN SCOPE | Block |
| 5 | STATE.md | Session Resume Section 1 | IN SCOPE | Block |
| 6 | INDEX.md | Convergence Status row | IN SCOPE | Advisory |
| 7 | INDEX.md | adversarial-review summary-table rows (latest pass) | IN SCOPE | Advisory |
| 8 | burst-log.md | latest-pass Dim-7 (Attestation) | IN SCOPE | Advisory |
| 9 | lessons.md | latest-lesson trend-table | IN SCOPE | Advisory |
| 10 | decision-log.md | trajectory-bearing rows | OUT OF SCOPE | Advisory-prose only |
| 11 | adv-cycle-pass-*.md | frontmatter `trajectory_tail` field | OUT OF SCOPE | Written by adversary |

## Adversary Pass Coverage

Pass-1 (2026-05-28; 14 findings 5H+5M+3L+1N; closed in this v1.1 — 9 BC findings (F-S15.17-SP1-002/004/005/007/009/010/011/012/014); story-writer fix-burst closes 5 remaining (F-001/003/006/008/013)). BC gate: BC-5.39.009 MUST reach 3-CLEAN before S-15.17 is promoted to `status: ready` per Spec-First Gate S-7.01. STREAK: 0/3 (pass-1 = HIGH verdict; streak reset).

Pass-2 (2026-05-28; 11 findings 3H+4M+3L+1N including F-001 regression of F-SP1-003 cascade-propagation-gap; closed in this v1.2 — 7-8 PO findings (F-003/004/005/006/007/008/010/011 + partial F-009); story-writer fix-burst closes 5 remaining incl. mandatory bidirectional parity check per pass-2 META-LEVEL-31 mandate). STREAK: 0/3 (pass-2 = HIGH verdict; streak reset).

## Preconditions

### PostToolUse activation

1. A PostToolUse Edit/Write event has fired on one of the four target files. Target file detection
   uses `Path::new(file_path).file_name()` path-component-strict matching (see invariant 3):
   - `STATE.md` → triggers STATE.md arm (sites 1-5; Block severity)
   - `INDEX.md` within the active cycle path (path-component-strict basename `INDEX.md` AND
     full path contains `v1.0-feature-engine-discipline-pass-1/`) → triggers INDEX.md arm
     (sites 6-7; Advisory severity)
   - `burst-log.md` → triggers burst-log arm (site 8; Advisory severity)
   - `lessons.md` → triggers lessons arm (site 9; Advisory severity)
   - Any other file → `HookResult::Continue` immediately (no validation performed)

2. The dispatcher has invoked the `validate-trajectory-tail-cell-completeness` WASM plugin with
   the write payload containing `tool_input.file_path`.

### File read capability (META-LEVEL-24 false-green prevention)

3. The hook reads the post-write content via `host::read_file` with `max_bytes = 524288` (512 KiB)
   and `timeout_ms = 2000` per call. The registry-level hook timeout is `timeout_ms = 5000`.

   **Rationale (max_bytes = 524288):** STATE.md and cycle artifacts grow monotonically. A smaller
   cap (e.g., the hook-sdk default 65536 bytes) could silently truncate large files, causing the
   hook to validate only the first portion and miss violations in later sections — the META-LEVEL-24
   false-green class. This BC EXPLICITLY sets `max_bytes = 524288` (512 KiB) on ALL `host::read_file`
   calls to prevent truncation. This matches the precedent from BC-5.39.008 invariant 7 and the
   S-15.15 story pattern.

   The hook does NOT inspect `tool_input.content`; the filesystem value is the source of truth
   for post-write validation.

### Path discrimination — STATE.md parent-guard + INDEX.md cycle-path guard

4. The STATE.md arm fires ONLY when BOTH of the following hold:
   - `Path::new(file_path).file_name() == Some("STATE.md")` (path-component-strict basename check)
   - `file_path` is rooted at `.factory/` — specifically, the path MUST contain the `.factory/`
     path component: either `file_path.starts_with(".factory/")` (relative form) OR
     `file_path.contains("/.factory/")` (absolute form, e.g., `/Users/x/dev/vsdd-factory/.factory/STATE.md`)

   **Rationale:** A WASM hook that fires on ANY `STATE.md` anywhere on the filesystem (e.g.,
   `/tmp/STATE.md`, `/home/user/notes/STATE.md`) and emits a Block-grade exit code is a false-
   positive risk. The `.factory/` parent guard is the production-grade constraint — only the
   factory pipeline's STATE.md carries the D-453(d) trajectory_tail requirement. Any `STATE.md`
   outside `.factory/` MUST be treated as a non-target file: `HookResult::Continue` immediately
   (see EC-019).

   The INDEX.md arm fires ONLY when both of the following hold:
   - `Path::new(file_path).file_name() == Some("INDEX.md")` (path-component-strict basename check)
   - `file_path.contains("v1.0-feature-engine-discipline-pass-1")` (cycle-path substring guard)

   This prevents cross-cycle INDEX.md files (e.g., future cycle directories, brownfield cycle
   INDEX.md) from triggering the arm. If future cycles use a different cycle-path string, the
   cycle-path guard must be updated in the implementation and the BC amended.

### YAML frontmatter parse

5. For STATE.md site 1 (frontmatter `current_step:`): the hook scans the YAML frontmatter region
   (bytes between the first `---\n` delimiter and the second `---\n` delimiter). If the frontmatter
   region is absent or unparseable (malformed YAML delimiters), the hook emits `host::log_warn`
   advisory and emits `HookResult::Continue` — it does NOT block on malformed frontmatter structure
   (fail-open for parse errors per invariant 10, consistent with sibling BC-5.39.006/007/008
   fail-open precedent). The `current_step:` line is extracted by line-scan within the frontmatter
   region; if the key is absent, the site is treated as missing trajectory_tail (Block per
   postcondition 1).

### Trajectory_tail pattern

6. The canonical trajectory_tail pattern is the regex `(→[0-9]+){4}` (LENGTH exactly 4 arrow-
   segments). A site is "present" if and only if the extracted section or field text contains
   exactly one contiguous sequence of LENGTH=4. LENGTH=3 (three arrow-segments) MUST be treated
   as "absent." LENGTH=5 or greater (five or more arrow-segments) MUST also be treated as "absent"
   — this closes the LENGTH right-boundary adjudication per BC-5.39.006 inv-6(b)+EC-007: sibling
   BC-5.39.006 explicitly blocks on LENGTH=5 as a violation (`HookResult::block_with_fix(...)`)
   per EC-007; this BC aligns with that strict LENGTH=4 semantics. A sequence `→9→9→9→9→9`
   does NOT match `(→[0-9]+){4}` as the sole group count — it contains LENGTH=5 which is
   also a violation per D-433(e)+D-439(c) original codification. See EC-018 for the LENGTH=5 edge
   case.

   The pattern accepts multi-digit axis values: `→9→9→9→9`, `→10→12→11→13`, etc. are all valid
   (matching `(→[0-9]+){4}` with the `[0-9]+` quantifier accepting one or more digits).
   Production STATE.md example: `→9→9→9→11` (D-513) is valid LENGTH=4.

## Postconditions

### STATE.md arm — Block severity (sites 1-5)

1. When STATE.md is written and the frontmatter `current_step:` field value does not contain a
   trajectory_tail matching `(→[0-9]+){4}` (LENGTH=4 exactly; LENGTH=3 insufficient; LENGTH=5+
   also insufficient per inv-4 STRICT), the hook treats this as a missing site and includes
   "STATE.md frontmatter current_step" in the missing-sites list (see postcondition 6 for cascade
   behavior). The field value is extracted by scanning the YAML frontmatter region for a line
   matching `^current_step:` and capturing the remainder of that line as the value.

2. When STATE.md is written and the "Last Updated" table cell does not contain a trajectory_tail
   matching `(→[0-9]+){4}`, the hook treats this as a missing site and includes "STATE.md Last
   Updated cell" in the missing-sites list. **Extractor specification:** "Last Updated" is a TABLE
   ROW inside the `## Project Metadata` heading section — there is NO `## Last Updated` heading.
   The extractor MUST scan for the markdown table row pattern `| **Last Updated** |` within the
   body and capture the cell value in the second pipe-delimited column. If the `## Project
   Metadata` table or the `| **Last Updated** |` row is absent, the site is treated as missing
   (Block per postcondition 6). Production STATE.md evidence:
   ```
   $ grep 'Last Updated' .factory/STATE.md | head -3
   | **Last Updated** | 2026-05-28 — D-513 ... Trajectory-tail carry-across →9→9→9→11. |
   ```
   The trajectory_tail appears in the cell value — NOT in a heading. Extractor must
   NOT scan for `## Last Updated` heading (no such heading exists in production STATE.md).

3. When STATE.md is written and the "Phase Progress" section's latest-pass row does not contain a
   trajectory_tail matching `(→[0-9]+){4}`, the hook treats this as a missing site and includes
   "STATE.md Phase Progress rows" in the missing-sites list. **Extractor specification:** The
   `## Phase Progress` heading IS present in production STATE.md (line 61). The extractor MUST
   scan for the `## Phase Progress` heading and capture the TABLE ROWS that follow it (until the
   next `##` heading). To identify the "latest pass row": take the BOTTOMMOST row of the Phase
   Progress table, skipping any rows whose Status cell contains "ARCHIVED" or "COMPACTED". The
   state-manager Commit E discipline appends one row per burst — the bottommost non-archived/non-
   compacted row IS the latest by construction. Do NOT skip rows with Status "COMPLETE",
   "SHIPPED", "MERGED", or "CYCLE CLOSED" — most Phase Progress rows carry these statuses and
   skipping them would skip the most recent fix-burst row. If no non-archived/non-compacted row
   exists (all rows compacted), the site is treated as present (pass-through). Production STATE.md
   evidence:
   ```
   $ grep '^## Phase Progress' .factory/STATE.md
   ## Phase Progress
   ```
   The heading exists and contains historical rows spanning D-503..D-513. The bottommost non-
   archived row carries the most recent burst's trajectory_tail.

4. When STATE.md is written and the "Concurrent Cycles" section or row does not contain a
   trajectory_tail matching `(→[0-9]+){4}`, the hook treats this as a missing site and includes
   "STATE.md Concurrent Cycles row" in the missing-sites list. The section is extracted by
   scanning for a heading or label matching "Concurrent Cycles" and capturing the associated text.

5. When STATE.md is written and the "Session Resume Checkpoint" section (specifically the `### §1.
   Where We Are` sub-section content) does not contain a trajectory_tail matching `(→[0-9]+){4}`,
   the hook treats this as a missing site and includes "STATE.md Session Resume Section 1" in
   the missing-sites list. **Extractor specification:** The heading uses the prefix
   `## Session Resume Checkpoint` which MAY be followed by an optional parenthetical suffix (e.g.,
   `## Session Resume Checkpoint (2026-05-28 — D-513 ...)`). The extractor MUST match by
   PREFIX `## Session Resume Checkpoint` (not an exact string match) to tolerate the evolving
   parenthetical. The sub-section to validate is `### §1.` content (the first `###` sub-section
   under the Checkpoint heading). Production STATE.md evidence:
   ```
   $ grep '^## Session Resume' .factory/STATE.md
   ## Session Resume Checkpoint (2026-05-28 — D-513 BC-5.39.009 v1.0 AUTHORED + ...)
   $ grep '^### §1' .factory/STATE.md
   ### §1. Where We Are
   ```
   The trajectory_tail appears in §1 body content (e.g., `→9→9→9→11`). The extractor must NOT
   require exact heading match — prefix-match `## Session Resume Checkpoint` is mandatory.

6. When one or more STATE.md sites (postconditions 1-5) are missing trajectory_tail, the hook
   emits a single `HookResult::block_with_fix(...)` enumerating ALL missing sites in the message
   body (schema-violation cascade per invariant 8 — never short-circuits on first missing site).
   The block message format is:
   ```
   STATE.md trajectory_tail missing from N prescribed site(s) (D-453(d)):
     - STATE.md <site-name-1>
     - STATE.md <site-name-2>
     ...
   ```
   with fix recommendation: "Add trajectory_tail arrow-sequence (→N→N→N→N, LENGTH=4 per
   D-433(e)) to each listed site."

### INDEX.md arm — Advisory severity (sites 6-7)

7. When INDEX.md (active cycle) is written and the "Convergence Status" row does not contain a
   trajectory_tail matching `(→[0-9]+){4}`, the hook emits `host::log_warn` with message
   "validate-trajectory-tail-cell-completeness: advisory — INDEX.md Convergence Status row missing
   trajectory_tail (→N→N→N→N, D-453(d) site 6)" and emits `HookResult::Continue` (NOT Block).

8. When INDEX.md (active cycle) is written and the adversarial-review summary-table's latest-pass
   row does not contain a trajectory_tail matching `(→[0-9]+){4}`, the hook emits `host::log_warn`
   with message "validate-trajectory-tail-cell-completeness: advisory — INDEX.md adversarial-review
   summary-table latest-pass row missing trajectory_tail (D-453(d) site 7)" and emits
   `HookResult::Continue`.

### burst-log.md arm — Advisory severity (site 8)

9. When burst-log.md is written and the latest-pass Dim-7 (Attestation) block does not contain a
   trajectory_tail matching `(→[0-9]+){4}`, the hook emits `host::log_warn` with message
   "validate-trajectory-tail-cell-completeness: advisory — burst-log.md latest-pass Dim-7
   (Attestation) block missing trajectory_tail (D-453(d) site 8)" and emits `HookResult::Continue`.

### lessons.md arm — Advisory severity (site 9)

10. When lessons.md is written and the latest-lesson trend-table row does not contain a
    trajectory_tail matching `(→[0-9]+){4}`, the hook emits `host::log_warn` with message
    "validate-trajectory-tail-cell-completeness: advisory — lessons.md latest-lesson trend-table
    missing trajectory_tail (D-453(d) site 9)" and emits `HookResult::Continue`.

### Fail-open postconditions

11. When any target file is too large (`len > MAX_BYTES = 524288`) for `host::read_file` to
    return (HostError::TooBig or equivalent), the hook emits `host::log_warn` with message
    "validate-trajectory-tail-cell-completeness: file <path> exceeds 524288 bytes; skipping
    validation (fail-open)" and emits `HookResult::Continue`. NEVER Block on file-too-large.

12. When `host::read_file` returns any HostError (Timeout, NotFound, CapabilityDenied, or other)
    for any target file, the hook emits `host::log_warn` with the error description and emits
    `HookResult::Continue`. NEVER Block on read failure.

### Pass postcondition

13. When a target file is written and ALL applicable prescribed sites for that file contain a valid
    trajectory_tail matching `(→[0-9]+){4}`, the hook emits `HookResult::Continue` with no
    warnings. This is the clean-pass case.

## Invariants

1. **Read-only validator.** The hook NEVER writes any file. It has no `write_file` capability in
   its registry entry. It is a read-only PostToolUse validator that signals AFTER the write
   completes. The dispatcher records the block signal; the author must correct and re-write if
   a block was emitted.

2. **PostToolUse only.** The hook fires PostToolUse only — it never prevents a write at
   PreToolUse; it signals after the write has completed.

3. **Path-component-strict basename detection.** Target file detection MUST use
   `Path::new(file_path).file_name()` for basename extraction. Using `ends_with("STATE.md")` or
   raw string suffix matching on the full path MUST NOT be substituted. This prevents the
   META-LEVEL-24 false-green where a non-target file with a matching path suffix triggers the arm.
   Example: file `/some/other/not-STATE.md` has basename `not-STATE.md` and does NOT trigger the
   STATE.md arm. File `/tmp/STATE.md` has basename `STATE.md` but is OUTSIDE `.factory/` —
   Precondition 4 parent-guard rejects it (see EC-019). File `.factory/STATE.md` has basename
   `STATE.md` AND is rooted at `.factory/` — it triggers the STATE.md arm.

4. **Canonical trajectory_tail pattern LENGTH=4 STRICT.** The canonical arrow-sequence regex is
   `(→[0-9]+){4}`: exactly 4 arrow-segments, each followed by one or more decimal digits. A site
   is "present" if and only if the extracted text contains at least one contiguous match of exactly
   LENGTH=4. Length-3 sequences (`→9→9→9`) are NOT present. **Length-5 sequences (`→9→9→9→9→9`)
   are also NOT present — LENGTH=5+ is equally a violation** per D-433(e)+D-439(c) original
   codification. This aligns with sibling BC-5.39.006 invariant 6(b) and EC-007 which explicitly
   block on LENGTH=5 (`HookResult::block_with_fix(...)` per EC-007 precedent). Multi-digit values
   (`→10→12→11→13`) are valid and match `[0-9]+`. Cure-extension: this BC adopts the LENGTH=4-
   strict invariant from BC-5.39.006 EC-006/EC-007, but does NOT require the literal canonical
   marker `trajectory-tail ` (with trailing space) before the LENGTH check because per-cell sites
   2-9 are heterogeneous text contexts where the BC-5.39.006 marker-prefix convention does not
   apply. This is a deliberate non-extension of marker-prefix semantics (see §Cure-Extension
   Parsimony Note).

5. **STATE.md sites (1-5): Block severity per D-411(a) HIGH classification.** Omission of
   trajectory_tail from any of the 5 STATE.md prescribed sites is classified HIGH severity
   per D-411(a) (the adversary finding ADV-EDP1-P75-HIGH-002 arose precisely from these sites
   lacking the canonical tail). Block severity applies to all 5 STATE.md sites collectively.

6. **Non-STATE.md sites (6-9): Advisory severity only.** Sites 6 (INDEX.md Convergence Status),
   7 (INDEX.md adversarial-review row), 8 (burst-log.md Dim-7), and 9 (lessons.md trend-table)
   use advisory severity: `host::log_warn` + `HookResult::Continue`. Advisory behavior MUST NOT
   use any `HookResult::Advisory` variant (see invariant 9). Advisory behavior is implemented as
   `HookResult::Continue` + `host::log_warn` exclusively.

7. **`max_bytes = 524288` on ALL `host::read_file` calls.** Every `host::read_file` invocation
   in this hook MUST specify `max_bytes = 524288` (512 KiB). This is non-negotiable and applies
   regardless of which target arm is active. Smaller values risk truncating large artifacts and
   producing false-green results (META-LEVEL-24 false-green prevention per BC-5.39.008 precedent).

8. **STATE.md cascade: all missing sites in a single Block.** When the STATE.md arm finds
   multiple missing sites, ALL missing sites MUST be enumerated in a single
   `HookResult::block_with_fix(...)` message. The hook MUST NOT short-circuit on the first
   missing site. This ensures the author can fix all issues in one edit rather than iterating
   one-at-a-time (schema-violation cascade per BC-5.39.008 invariant 8 precedent).

9. **Advisory behavior: `HookResult::Continue` + `host::log_warn` exclusively.** Use
   `HookResult::Continue` + `host::log_warn` for advisory behavior. This hook MUST NOT use any
   `HookResult::Advisory` variant the SDK may add in future. Rationale: advisory is observable
   via stderr/log_warn; constructing a separate variant would diverge from sibling BC-5.39.004/
   005/006/007/008 pattern. Advisory is implemented as `HookResult::Continue` + `host::log_warn`
   — consistent with the invariant 10 fail-open pattern. This invariant prevents implementer
   surprises if a future SDK version adds such a variant.

10. **`host::read_file` HostError → fail-open.** Any HostError from `host::read_file` (for any
    target file, any HostError variant) MUST result in `HookResult::Continue` + `host::log_warn`.
    Never block on a read failure. This matches the fail-open precedent from BC-5.39.008 invariant
    9 and BC-5.39.004/005/006/007 sibling hooks.

11. **`is_char_boundary()` guards on all byte-index slicing.** All byte-index slice expressions
    operating on content strings MUST use `is_char_boundary()` guards where multi-byte UTF-8 input
    is possible. Slice without boundary guard is a runtime panic risk per the S-15.11 cascade
    lesson F-P4-001. The `→` character is a 3-byte UTF-8 sequence; any byte-walk implementation
    must account for this.

12. **`on_error = "continue"` — plugin crash → graceful degradation.** The hooks-registry.toml
    entry for this plugin MUST set `on_error = "continue"`. This means if the plugin crashes,
    panics, or exhausts its WASM fuel budget, the dispatcher treats the result as
    `HookResult::Continue` (graceful degradation) rather than blocking the write. This is
    consistent with the fail-open postcondition (invariant 10) and matches the sibling pattern
    established by BC-5.39.004, BC-5.39.005, BC-5.39.006, BC-5.39.007, and BC-5.39.008. Rationale:
    this hook is a PostToolUse advisory/enforcement gate — a plugin crash MUST NOT prevent the
    author from continuing work; it is less harmful to miss a tail check than to falsely block
    every write. Anchor: ADR-018 capability model (minimal interference); sibling-pattern parity.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md `current_step:` key is absent from frontmatter (malformed or incomplete frontmatter) | Block: "STATE.md frontmatter current_step" listed as missing (absent key = absent tail) |
| EC-002 | STATE.md with `current_step: "Phase 3 step"` (text only, no arrow-sequence) | Block: "STATE.md frontmatter current_step" listed as missing |
| EC-003 | STATE.md with `current_step: "→9→9→9"` (LENGTH=3 — too short) | Block: site listed as missing (LENGTH=4 required per inv-4) |
| EC-004 | STATE.md exceeds 524288 bytes → `host::read_file` returns HostError::TooBig | `HookResult::Continue` + `host::log_warn` fail-open; never Block |
| EC-005 | File path `/some/dir/not-STATE.md` — `file_name() == "not-STATE.md"` | `HookResult::Continue` immediately; basename does not match "STATE.md"; STATE.md arm not triggered (inv-3) |
| EC-006 | STATE.md with 3 of 5 sites missing trajectory_tail simultaneously | Single cascade Block enumerating all 3 missing sites (inv-8); not 3 separate Blocks |
| EC-007 | STATE.md with all 5 sites carrying `→9→9→9→9` | `HookResult::Continue`; no Block or advisory (PC13) |
| EC-008 | INDEX.md from a different cycle path (e.g., `v2.0-future-cycle/INDEX.md`) | `HookResult::Continue`; cycle-path guard rejects non-active-cycle INDEX.md (Precondition 4) |
| EC-009 | INDEX.md Convergence Status row missing tail | `host::log_warn` advisory + `HookResult::Continue` (NOT Block; inv-6) |
| EC-010 | burst-log.md Dim-7 block missing tail | `host::log_warn` advisory + `HookResult::Continue` |
| EC-011 | lessons.md trend-table missing tail | `host::log_warn` advisory + `HookResult::Continue` |
| EC-012 | `host::read_file` HostError::Timeout for STATE.md | `HookResult::Continue` + `host::log_warn`; fail-open (inv-10) |
| EC-013 | STATE.md has `current_step:` with multi-digit tail `→10→12→11→13` (4 arrows, multi-digit) | `HookResult::Continue` for that site; multi-digit values match `[0-9]+` (inv-4) |
| EC-014 | STATE.md frontmatter has `→9→9→9→9` in current_step but Phase Progress section has no tail | Block: "STATE.md Phase Progress rows" listed as missing; partial presence still counts as missing-per-site |
| EC-015 | File path `/factory-artifacts/STATE.md` vs `.factory/STATE.md` — both have `file_name() == "STATE.md"` | `.factory/STATE.md` triggers STATE.md arm (basename match + parent guard satisfied). `/factory-artifacts/STATE.md` does NOT trigger STATE.md arm: path-component-strict check rejects it because `factory-artifacts` is not `.factory` (Precondition 4 parent-guard). |
| EC-016 | YAML frontmatter region absent in STATE.md (no `---\n` delimiters found) | `host::log_warn` advisory + `HookResult::Continue` (fail-open per invariant 10; consistent with sibling BC-5.39.006/007/008 fail-open precedent; does NOT block for absent frontmatter) |
| EC-017 | STATE.md with `current_step:` as multi-line YAML block scalar using `|` or `>` | Extractor reads the line containing `current_step:` and the indented continuation lines; must find `(→[0-9]+){4}` somewhere in the full value block |
| EC-018 | STATE.md with `current_step:` containing `→9→9→9→9→9` (LENGTH=5 — too long) | Block: "STATE.md frontmatter current_step" listed as missing (LENGTH=5 ≠ LENGTH=4 strict per inv-4; mirrors BC-5.39.006 EC-007 precedent) |
| EC-019 | File path `/tmp/STATE.md` or `/home/user/notes/STATE.md` — `file_name() == "STATE.md"` but path is NOT rooted at `.factory/` | `HookResult::Continue` immediately; Precondition 4 parent-guard rejects non-factory STATE.md; STATE.md arm MUST NOT fire for STATE.md files outside the `.factory/` directory |

## Canonical Test Vectors

| Scenario | File | Input Condition | Expected Hook Output | Sites Exercised |
|----------|------|----------------|---------------------|-----------------|
| STATE.md all 5 sites present | STATE.md | All 5 sections contain `→9→9→9→9` | `HookResult::Continue` | PC13 |
| STATE.md frontmatter current_step missing | STATE.md | `current_step: "Phase 3 step 42"` (no arrow) | Single Block: "STATE.md frontmatter current_step" | PC1, PC6 |
| STATE.md Last Updated missing | STATE.md | Last Updated section has no `→[0-9]+{4}` | Single Block: "STATE.md Last Updated cell" | PC2, PC6 |
| STATE.md Phase Progress missing | STATE.md | Phase Progress section has no tail | Single Block: "STATE.md Phase Progress rows" | PC3, PC6 |
| STATE.md Concurrent Cycles missing | STATE.md | Concurrent Cycles row has no tail | Single Block: "STATE.md Concurrent Cycles row" | PC4, PC6 |
| STATE.md Session Resume missing | STATE.md | Session Resume Section 1 has no tail | Single Block: "STATE.md Session Resume Section 1" | PC5, PC6 |
| STATE.md cascade: 3 sites missing | STATE.md | Last Updated + Concurrent Cycles + Session Resume all missing | Single Block listing all 3 sites (inv-8) | PC2+PC4+PC5, PC6 cascade |
| STATE.md LENGTH=3 present | STATE.md | current_step contains `→9→9→9` only | Block: site missing (LENGTH=3 ≠ LENGTH=4; inv-4) | PC1 |
| STATE.md LENGTH=4 multi-digit | STATE.md | current_step contains `→10→12→11→13` | `HookResult::Continue` | inv-4 multi-digit |
| INDEX.md Convergence Status missing | INDEX.md (active cycle) | Convergence Status row has no tail | `host::log_warn` + `HookResult::Continue` (not Block; PC7+inv-6) | PC7 |
| INDEX.md adv-table row missing | INDEX.md (active cycle) | Latest adv-review row has no tail | `host::log_warn` + `HookResult::Continue` | PC8 |
| INDEX.md all sites present | INDEX.md (active cycle) | Both sites carry `→9→9→9→9` | `HookResult::Continue` | PC13 |
| INDEX.md wrong cycle | other-cycle/INDEX.md | Different cycle-path substring | `HookResult::Continue` (cycle guard; EC-008) | Precondition 4 |
| burst-log.md Dim-7 missing | burst-log.md | Latest Dim-7 block has no tail | `host::log_warn` + `HookResult::Continue` | PC9 |
| burst-log.md Dim-7 present | burst-log.md | Latest Dim-7 block has `→9→9→9→9` | `HookResult::Continue` | PC13 |
| lessons.md trend-table missing | lessons.md | Latest trend-table row has no tail | `host::log_warn` + `HookResult::Continue` | PC10 |
| lessons.md trend-table present | lessons.md | Latest trend-table row has `→9→9→9→9` | `HookResult::Continue` | PC13 |
| File too large | STATE.md | `host::read_file` → HostError::TooBig | `HookResult::Continue` + `host::log_warn` (PC11+inv-7) | PC11 |
| Read failure | STATE.md | `host::read_file` → HostError::Timeout | `HookResult::Continue` + `host::log_warn` (PC12+inv-10) | PC12 |
| Wrong basename | not-STATE.md | `file_name() == "not-STATE.md"` | `HookResult::Continue` immediately (inv-3) | EC-005 |
| File not target | STORY-INDEX.md | Any write | `HookResult::Continue` immediately | Precondition 1 |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-453(d) | Canonical 9-site mapping table (trajectory_tail prescribed sites); this BC is the runtime enforcement of the registry | 1-10 |
| D-454(a) | Per-cell granularity gate: cell-level section extraction, not whole-file grep | 1-10 |
| D-433(e)+D-439(c) | LENGTH=4 trajectory tail requirement (not LENGTH=3; not LENGTH=5+) | inv-4 |
| D-411(a) | HIGH classification for STATE.md trajectory_tail omission → Block severity | inv-5 |

Non-D-NNN references (moved out of D-NNN table per F-S15.17-SP1-011):
- ADR-018: WASM path_allow capability model — see §ADR References in Traceability
- ADR-018: WASM path_allow capability model + no-subprocess principle — see §ADR References in Traceability (ADR-021 dropped: cargo-audit-specific, not general no-subprocess principle; generic no-subprocess principle covered by ADR-018 hook-sdk contract)
- META-LEVEL-24: `max_bytes = 524288` false-green prevention — see invariant 7
- BC-5.39.001: 3-CLEAN convergence protocol — see §Related BCs

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | STATE.md-Block Invariant — hook emits Block when current_step missing trajectory_tail | bats integration test (fail-state-frontmatter-missing-tail.bats fixture) |
| (pending) | STATE.md-Last-Updated-Block Invariant — hook emits Block when Last Updated section missing tail | bats (fail-state-last-updated-missing-tail.bats) |
| (pending) | STATE.md-Phase-Progress-Block Invariant — hook emits Block when Phase Progress section missing tail | bats (fail-state-phase-progress-missing-tail.bats) |
| (pending) | STATE.md-Concurrent-Cycles-Block Invariant — hook emits Block when Concurrent Cycles row missing tail | bats (fail-state-concurrent-cycles-missing-tail.bats) |
| (pending) | STATE.md-Session-Resume-Block Invariant — hook emits Block when Session Resume Section 1 missing tail | bats (fail-state-session-resume-missing-tail.bats) |
| (pending) | STATE.md-Cascade Invariant — single Block enumerates ALL missing STATE.md sites | bats (fail-state-cascade-missing-sites.bats — 3 sites simultaneously missing) |
| (pending) | STATE.md-Pass Invariant — Continue when all 5 STATE.md sites present | bats (pass-state-all-sites-present.bats) |
| (pending) | INDEX.md-Convergence-Advisory Invariant — Continue + log_warn (not Block) when Convergence Status missing | bats (fail-index-convergence-status-missing-tail.bats — assert exit 0 + stderr contains advisory) |
| (pending) | INDEX.md-AdvTable-Advisory Invariant — Continue + log_warn when adv-table row missing | bats (fail-index-adv-table-missing-tail.bats) |
| (pending) | burst-log-Advisory Invariant — Continue + log_warn when Dim-7 missing tail | bats (fail-burst-log-dim7-missing-tail.bats) |
| (pending) | lessons-Advisory Invariant — Continue + log_warn when trend-table missing tail | bats (fail-lessons-trend-table-missing-tail.bats) |
| (pending) | Fail-open-TooBig Invariant — Continue + log_warn when file exceeds 524288 bytes | bats (pass-file-too-large-failopen.bats) |
| (pending) | Fail-open-HostError Invariant — Continue + log_warn on any read failure | bats (pass-read-failure-failopen.bats) |
| (pending) | Path-component-strict Invariant — wrong basename does not trigger arm | bats (pass-wrong-filename-no-trigger.bats) |
| (pending) | LENGTH=4-Required Invariant — LENGTH=3 tail treated as absent | bats (fail-length-3-absent.bats) |
| (pending) | LENGTH=4-Multi-digit Pass Invariant — multi-digit arrow values pass | bats (pass-length-4-present.bats with →10→12→11→13) |
| (pending) | INDEX.md-Cycle-Path-Guard Invariant — wrong cycle INDEX.md produces Continue | bats (pass-wrong-cycle-index.bats — EC-008) |
| (pending) | Production-Registry Invariant — hook registered at priority 158 with correct tool = "Edit\|Write" | bats (integration-production-registry.bats — checks production hooks-registry.toml) |

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — trajectory_tail per-cell runtime gate automation; closes META-LEVEL-30 route (b)) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically enforces trajectory_tail per-cell presence at all 9 D-453(d) prescribed sites at write time, preventing META-LEVEL-30 route (b) recurrence ("codified-canonical-registry-with-no-runtime-WASM-gate-enforcing-each-site"). The hook targets the four STATE.md/INDEX.md/burst-log.md/lessons.md artifacts that carry the D-453(d) trajectory_tail sites — factory engine discipline artifacts. E-12 as used in the BC-5.39.xxx family per engine-discipline automation sub-capability convention. |
| Architecture Module | `crates/hook-plugins/validate-trajectory-tail-cell-completeness/` (Rust WASM plugin, new crate); `plugins/vsdd-factory/hooks-registry.toml` (registry entry, priority 158, PostToolUse, `tool = "Edit\|Write"`, `path_allow = [".factory"]`); `plugins/vsdd-factory/hook-plugins/validate-trajectory-tail-cell-completeness.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-453(d) (canonical trajectory_tail prescribed sites registry — runtime enforcement); D-454(a) (per-cell granularity gate discipline); D-433(e)+D-439(c) (LENGTH=4 requirement); D-411(a) (HIGH classification → Block severity for STATE.md sites) |
| ADR References | ADR-017 (Per-Story Adversary Phasing — 3-CLEAN applies to S-15.17 LOCAL cascade; file: `.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md`); ADR-018 (WASM-Plugin Context Resolvers — `path_allow` capability model; `path_allow = [".factory"]`; no-subprocess principle via host::read_file-only API) |
| Stories | S-15.17 (validate-trajectory-tail-cell-completeness WASM hook) |
| L2 Invariants | (none currently assigned — this BC is a process-automation gate; no L2 domain invariants apply; same classification as BC-5.39.008 sibling) |
| Predecessor Cure-Extensions | BC-5.39.005 (validate-state-structure Phase 1 — structural-gate pattern for STATE.md PostToolUse validation; extended here to per-cell trajectory_tail checks); BC-5.39.006 (validate-dispatch-advance — trajectory_tail substring enforcement on current_step; BC-5.39.009 extends to multi-site multi-file scope per D-497 cure-extension-parsimony) |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.17 must achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; same hook-sdk pattern + fail-open + path-component-strict guard)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (predecessor structural-gate pattern for STATE.md PostToolUse; BC-5.39.009 extends to trajectory_tail per-cell scope)
- BC-5.39.006 — governs validate-dispatch-advance WASM hook (predecessor trajectory_tail enforcement on current_step; BC-5.39.009 extends to multi-site multi-file per D-497)
- BC-5.39.007 — governs validate-closes-completeness hook (sister PostToolUse hook; same MAX_BYTES=524288 + advisory+blocking dual-arm pattern)
- BC-5.39.008 — governs validate-policies-schema hook (closest structural pattern; dual-arm advisory+blocking; hooks-registry cross-ref; path-component-strict)

## Architecture Anchors

- `crates/hook-plugins/validate-trajectory-tail-cell-completeness/src/lib.rs` — hook logic: `target_arm` enum routing; `has_trajectory_tail(text: &str) -> bool` (LENGTH=4 pattern check); section extractors (`extract_frontmatter_current_step`, `extract_last_updated_section`, `extract_phase_progress_section`, `extract_concurrent_cycles_section`, `extract_session_resume_section_1`); `check_state_md(content: &str) -> Vec<MissingStateSite>` (cascade accumulator); `check_index_sites`, `check_burst_log_sites`, `check_lessons_sites` (advisory checkers); `on_post_tool_use(payload: HookPayload) -> HookResult` (effectful orchestration entry point)
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook; `host::log_warn(message)` for advisory-level non-blocking log entries
- `crates/hook-sdk/src/result.rs` — `HookResult` enum: `Continue`, `Block { reason }`, `Error { message }`; `HookResult::block_with_fix(hook, reason, recommendation, code)` constructor; NOTE: NO `HookResult::Advisory` variant exists
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration: priority 158 (next monotonic after validate-policies-schema at 157); `tool = "Edit|Write"`; `path_allow = [".factory"]`; `on_error = "continue"` (inv-12); closest `path_allow = [".factory"]`-only structural sibling is BC-5.39.006 (validate-dispatch-advance) — NOT BC-5.39.008 which uses `path_allow = [".factory", "plugins/vsdd-factory"]`
- `.factory/STATE.md` — primary target artifact (5 prescribed sites; Block arm)
- `.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md` — target artifact (2 prescribed sites; Advisory arm; cycle-path-guarded)
- `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` — target artifact (1 prescribed site; Advisory arm)
- `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md` — target artifact (1 prescribed site; Advisory arm)
- `plugins/vsdd-factory/hooks-registry.toml` §validate-trajectory-tail-cell-completeness — registry entry declaring capability `path_allow = [".factory"]` per ADR-018 OD-6 minimal capability grants

## Story Anchor

S-15.17 — v1.0-brownfield-backfill (F5 pass-75 HIGH-002 anchor; META-LEVEL-30 route (b) cure; BC gate for `status: ready`)

## Cure-Extension Parsimony Note

Per D-497 cure-extension-parsimony mandate, this BC does NOT introduce new INV-NNN abstractions
for the trajectory_tail enforcement pattern. It instead extends two predecessor structural-gate
patterns:

1. **BC-5.39.005 (validate-state-structure Phase 1):** Established the STATE.md PostToolUse
   structural-gate pattern — read full file via `host::read_file`, extract sections by heading
   scan, validate per-section content. BC-5.39.009 extends this to per-cell trajectory_tail
   presence at the 5 STATE.md prescribed sites.

2. **BC-5.39.006 (validate-dispatch-advance):** Established trajectory_tail LENGTH=4 STRICT
   enforcement (inv-6(b) + EC-007). BC-5.39.009 adopts the LENGTH=4-strict invariant from
   BC-5.39.006 EC-006/EC-007 but does NOT require the literal canonical marker `trajectory-tail `
   (with trailing space) before the LENGTH check.

   **Deliberate non-extension of marker-prefix semantics:** BC-5.39.006 conditions its LENGTH check
   on the `trajectory-tail ` marker prefix being present in the `current_step:` value — EC-023
   specifies that if the marker is absent, the LENGTH count does not run. This BC does not extend
   that marker-prefix convention to sites 2-9 because those sites are heterogeneous text contexts
   (STATE.md table cells, Phase Progress rows, Session Resume §1, INDEX.md Convergence row,
   burst-log Dim-7, lessons trend-table) where the `trajectory-tail ` marker convention does not
   apply. The LENGTH=4 check runs on the extracted section text directly. This is a deliberate
   non-extension, not an oversight. Per D-497 cure-extension-parsimony, extending the marker-prefix
   semantics to heterogeneous text contexts would require a novel INV-NNN abstraction — which this
   BC explicitly avoids.

Both predecessor cure-extensions cited in Traceability §Predecessor Cure-Extensions.

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.17 post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-05-28 | Pass-2 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-2 / (b) PC2/PC3/PC5 line-number anti-volatile-pin (TD-VSDD-091) — stripped line numbers from grep excerpts, kept prefix/content match only / (c) cure-extension parsimony note updated with deliberate non-extension of BC-5.39.006 marker-prefix semantics documented per D-497. Closes 7-8 of 11 pass-2 findings: F-S15.17-SP2-003 (EC-008 + test vectors + inv-3 Pre-vs-Post disambiguation: `(PC4)` → `(Precondition 4)` where PC4 referred to cycle-path guard Precondition; full EC table swept); F-S15.17-SP2-004 (`status: active` → `status: draft` pre-merge; `lifecycle_status: draft` consistent; POL-14 fires on S-15.17 merge — both fields go active); F-S15.17-SP2-005 (PC2/PC3/PC5 grep excerpts stripped of `grep -n` line numbers; grep commands without `-n` flag; anti-volatile-pin per TD-VSDD-091); F-S15.17-SP2-006 (inv-4 + §Cure-Extension Parsimony Note updated to document deliberate non-extension of BC-5.39.006 marker-prefix `trajectory-tail ` semantics; EC-006/EC-007 context quoted; per-cell heterogeneous text context rationale); F-S15.17-SP2-007 (Precondition 4 expanded to require `.factory/` parent-guard for STATE.md arm in addition to basename check; EC-019 added for non-factory STATE.md → Continue; inv-3 updated; EC-015 revised to reflect new parent-guard semantics); F-S15.17-SP2-008 (PC3 skip-list: `COMPLETE` dropped; skip only ARCHIVED/COMPACTED; bottommost-row rationale documented — state-manager Commit E appends one row per burst so bottommost IS latest); F-S15.17-SP2-009 partial (ADR-021 dropped from §ADR References and D-NNN non-D table note; frontmatter inputs updated; ADR-018 clarified to cover no-subprocess principle; story-writer drops ADR-021 from `anchored_adrs:` frontmatter); F-S15.17-SP2-010 (inv-9 rephrased from SDK-state assertion to behavioral prescription: "MUST NOT use any HookResult::Advisory variant the SDK may add"; inv-6 updated to match); F-S15.17-SP2-011 (D-453 pass-73 cite corrected from pass-74). POLICY 14 5-leg quintuple parity applied (version "1.2" + this Changelog row v1.2 + modified[] appended "2026-05-28 (v1.2)" + last_amended text-prefix "2026-05-28 (v1.2)" + BC-INDEX v2.56 row version cell v1.2). Story-writer next burst handles F-001 (AC-9/10/11/12/17 re-anchor + bidirectional parity check stdout; META-LEVEL-31 mandate) / F-002 (SS-05 narrative) / F-003 story EC-008 mirror / F-007 AC-23 / F-009 anchored_adrs drop. EC-019 added (monotonic, no renumbering). |
| 1.1 | 2026-05-28 | Pass-1 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-1 / (b) structural per-cell extractor-anchor specification (PC2/PC3/PC5 rewritten with production STATE.md structure evidence) / (c) cure-extension of BC-5.39.005+BC-5.39.006 pattern per D-497 parsimony. Closes 9 of 14 pass-1 findings: F-S15.17-SP1-002 (ADR-017 frontmatter path corrected to `ADR-017-per-story-adversary-phasing.md`); F-S15.17-SP1-004 (PC2 Last Updated as table cell in `## Project Metadata`, PC3 Phase Progress latest non-archived row, PC5 `## Session Resume Checkpoint` prefix-match with optional parenthetical, all with literal-shell evidence from factory-artifacts HEAD `29d08cc7`); F-S15.17-SP1-005 (inv-4 + PC6 LENGTH=4 STRICT adjudication — LENGTH=5+ also violation; EC-018 LENGTH=5 added; aligns with BC-5.39.006 inv-6(b)+EC-007+D-433(e)+D-439(c)); F-S15.17-SP1-007 (PC5/EC-016 fail-open contradiction resolved to log_warn+Continue); F-S15.17-SP1-009 (path_allow sibling cite corrected to BC-5.39.006); F-S15.17-SP1-010 (inv-12 on_error=continue added per sibling BC-5.39.004/005/006/007/008 precedent); F-S15.17-SP1-011 (D-NNN table purified — ADR/META-LEVEL/BC rows removed from table, moved to §ADR References / §Related BCs / inv-7); F-S15.17-SP1-012 (D-454(a) PC range 1-9→1-10); F-S15.17-SP1-014 (EC-017 "State.md"→"STATE.md"). POLICY 14 5-leg quintuple parity applied (version "1.1" + this Changelog row v1.1 + modified[] appended "2026-05-28 (v1.1)" + last_amended text-prefix "2026-05-28 (v1.1)" + BC-INDEX v2.55 row version cell v1.1). §Cure-Extension Parsimony Note section added. Story-writer next burst handles F-001/003/006/008/013. |
| 1.0 | 2026-05-28 | Initial authoring (product-owner; brownfield-backfill F5 pass-75 HIGH-002 anchor). Anchors ADV-EDP1-P75-HIGH-002. META-LEVEL-30 route (b) cure (INV-019 cure (a)/(b)/(c)): (a) Anchor identification — closes ADV-EDP1-P75-HIGH-002 (codified-without-runtime-gate-permits-silent-degradation-over-time; D-453(d) canonical 9-site mapping had no WASM enforcement gate); (b) Cure scope — structural (multi-site PostToolUse gate at write time on all 9 D-453(d) mechanically-checkable sites), NOT codification-only; (c) Cure-extension-parsimony evaluation per D-497 — this BC EXTENDS the BC-5.39.005 + BC-5.39.006 structural-gate pattern (STATE.md PostToolUse validation from BC-5.39.005; trajectory_tail enforcement from BC-5.39.006) rather than introducing a new INV-NNN abstraction; both predecessor cure-extensions cited in Traceability §Predecessor Cure-Extensions. BC-5.39.009 allocated as next monotonic ID after BC-5.39.008 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.17 merge). Priority 158 (next available PostToolUse slot per hooks-registry.toml audit: 155=validate-stable-anchors PreToolUse; 156=validate-closes-completeness PostToolUse; 157=validate-policies-schema PostToolUse; 158=this hook, uncollided). POLICY 14 5-leg quintuple parity applied (version "1.0" + this Changelog row v1.0 + modified: ["2026-05-28"] + last_amended text-prefix "2026-05-28 (v1.0)" + BC-INDEX upstream v2.54 row with version cell v1.0). |
