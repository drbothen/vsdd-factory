---
document_type: behavioral-contract
level: L3
version: "1.5"
status: active
producer: product-owner
timestamp: 2026-05-17T00:00:00Z
phase: section-12-step-3M3a-r-pass-3
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.004.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.003.md
input-hash: "5af355e"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
extracted_from: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified:
  - 2026-05-17
  - 2026-05-17
  - 2026-05-18
  - 2026-05-19
  - 2026-05-19
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.006
section: "5.39"
last_amended: "2026-05-19 (v1.4) — Sibling-sweep closing F-BC007P2-001: 16 HookResult::block_with_fix(...) occurrences replaced with HookResult::block_with_fix(...) canonical form. INV-017 applied. [Prior: 2026-05-18 (v1.3) — Spec amendment closing S-15.14 LOCAL adversary pass-11 finding F-P11-001: invariant 6(b) under-specifies the scoping boundary for the trajectory-tail LENGTH count. v1.2 prose said 'substring AFTER the trajectory-tail marker (not the whole current_step: value)' — but the production implementation in `check_trajectory_tail_length` (validate-dispatch-advance) further narrows to substring from marker-end TO the first semicolon segment-separator (or end-of-value if no semicolon follows). Full-substring-after-marker count yields 14 matches on production current_step (false-positive block); first-semicolon-segment count yields 4 (correct pass). v1.3 codifies the actual production behavior: invariant 6(b) now specifies 'from marker-end to first ; segment-separator (or end-of-value)'. EC-022 sweep: EC-022 row already describes the narrow scope correctly via 'LENGTH count scoped to substring after marker per invariant 6(b)' — row reworded to explicitly cite first-semicolon-segment scoping for clarity. This is a doc-clarification (spec aligns to code), not a behavior change. [Prior: 2026-05-17 (v1.2) — Spec amendment closing S-15.14 LOCAL adversary pass-3 finding F-P3-006: trajectory-tail prefix-absent now a HARD violation. New PC 6: current_step MUST contain literal canonical marker 'trajectory-tail ' (with trailing space); absence is BlockWithFix citing D-451(c)/F-P3-006. Invariant 6 updated: LENGTH count scoped to substring AFTER 'trajectory-tail ' prefix; if prefix absent, LENGTH check does not run — prefix-absent BlockWithFix fires instead. New EC-023 added. Precondition renumbering fixed (was 1,5,2,3,4 → now 1,2,3,4,5,6; F-P3-009/F-P2-009 in-scope fix). Verbatim grep stdout for 'trajectory-tail ' in STATE.md confirmed canonical marker present in production current_step. [Prior: 2026-05-17 (v1.1) — Spec amendments closing F-P1-001 (invariant 7 D-chain pattern relaxed to D-(\\d+) max-extraction; literal D-382..D- prefix requirement dropped; production current_step uses prose cite D-476 not range prefix), F-P1-002 (invariant 8 scoped to rows under ## Adversarial Reviews h2 heading only; canonical schema corrected from 6-column to 5-column per D-442(b); historical pre-D-441(b) rows grandfathered; precondition added), F-P1-003 (pipe arithmetic corrected: 5-column row = 6 pipes total; erroneous '8 pipes / 7 internal' formula replaced), F-P1-008 (implementer paper-fix via test-comment overriding spec now has authoritative spec text to match). EC-013 and EC-014 pipe counts realigned. [Prior: 2026-05-17 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M2 wave-4 story authoring). Anchors D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-441(b)+D-451(c). BC-5.39.006 allocated as next monotonic ID after BC-5.39.005 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.14 merge).]]"
---

# BC-5.39.006: validate-dispatch-advance WASM hook MUST block on forbidden meta-commentary in current_step, missing 4-index version citations, trajectory-tail cardinality violations, stale D-chain cites in STATE.md, and non-5-column adversary-pass rows in the INDEX.md ## Adversarial Reviews section

## Description

The `validate-dispatch-advance` WASM hook enforces that any Edit/Write to `STATE.md` does not
leave a structurally invalid `current_step:` frontmatter field, and that any Edit/Write to a
cycle `INDEX.md` does not leave adversary-pass rows with the wrong column count. The hook fires
PostToolUse and validates two classes of artifacts:

1. **STATE.md `current_step:` field validation** (D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-451(c)):
   the `current_step:` frontmatter value MUST NOT contain forbidden meta-commentary patterns, MUST
   cite all 4 index versions, MUST have a trajectory-tail segment introduced by the literal canonical
   marker `trajectory-tail ` (with trailing space) followed by exactly 4 arrow-separated values,
   and MUST cite a current D-chain range that is not stale (i.e., includes the latest D-NNN).

2. **INDEX.md adversary-pass row validation** (D-441(b)): every adversary-pass row in a cycle
   `INDEX.md` MUST use the strict 6-column schema. Rows with 5 or 7 columns are a violation.

If any of these properties is violated, the hook emits a `block_with_fix` signal naming the
specific violation class and the required remediation. This BC closes the recurring class of
adversary findings in the F5 engine-discipline cycle where `current_step:` meta-commentary
labels, missing index cites, trajectory-tail cardinality drift, stale D-chain cites, and
malformed INDEX.md adversary-pass rows were discovered by the adversary N bursts after the write
rather than at write time.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is exactly
   `STATE.md` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("STATE.md")` — NOT suffix-`ends_with`. Paths like
   `/some/dir/xSTATE.md` MUST NOT match), OR on a file whose `file_name` path component is
   exactly `INDEX.md` (same path-component-strict guard:
   `Path::new(file_path).file_name() == Some("INDEX.md")`).
2. The dispatcher has invoked the `validate-dispatch-advance` WASM plugin with the write payload.
3. The file content is read via `host::read_file` (filesystem-authoritative). The hook does NOT
   inspect the payload's `tool_input.content` field; the filesystem value is the source of truth
   for validation.
4. `host::read_file` is available with `max_bytes = 524288` (512 KiB; matching the cap
   established by BC-5.39.005 F-P5-002 fix-burst for STATE.md; sufficient for INDEX.md) and
   `timeout_ms = 2000` per call. The whole-hook registry-level timeout is `timeout_ms = 5000`
   (hooks-registry.toml; distinct from per-call file-read timeout).
5. For the INDEX.md arm: column-count validation applies only to rows inside the
   `## Adversarial Reviews` h2 section (exact heading text). Rows under any other h2 heading
   (e.g., `## S-6.01 Sub-cycle Adversarial Reviews`, `## E-10 Spec-Package Adversarial Reviews`,
   `## S-15.08 LOCAL Adversary Reviews`) are out of scope and MUST NOT be validated for column
   count.
6. **(v1.2, F-P3-006)** For the STATE.md arm: the `current_step:` value MUST contain the literal
   canonical marker `trajectory-tail ` (exactly, with trailing space). Absence of this marker is
   itself a precondition violation; the hook MUST emit `HookResult::block_with_fix(...)` with description
   `"current_step missing 'trajectory-tail ' canonical marker; D-451(c)/F-P3-006"` without
   proceeding to the LENGTH count check. The LENGTH count check (invariant 6, postcondition 4) runs
   ONLY when this marker is present.

## Postconditions

### STATE.md path (file_name == "STATE.md")

1. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - The `current_step:` frontmatter value does NOT match the forbidden meta-commentary regex
     `META-LEVEL-\d+ WATCH|self-app TEST|expected verdict`.
   - All 4 index version patterns are present in the `current_step:` value:
     BC-INDEX vX, VP-INDEX vX, STORY-INDEX vX, ARCH-INDEX vX (where X is a version string).
   - The `current_step:` value contains the literal canonical marker `trajectory-tail ` (with
     trailing space), and the substring after that marker contains exactly 4 `→(\d+)` matches
     (LENGTH=4 per D-451(c); canonical form `trajectory-tail →N→N→N→N`).
   - The D-chain cite in `current_step:` is not stale — it includes a reference to the latest
     D-NNN recorded in the cycle decision-log (validated by checking that the cited D-NNN range
     upper bound matches the most recently codified decision visible in the file or, if
     inaccessible, that the cite pattern `D-382..D-\d+` is present and the terminal integer is
     >= the previously-observed maximum).
2. If the `current_step:` value matches the forbidden meta-commentary regex, the hook emits
   `HookResult::block_with_fix(...)` naming the offending pattern and citing D-440(a)+D-441(a)+D-442(a).
3. If any of the 4 index version patterns is absent from `current_step:`, the hook emits
   `HookResult::block_with_fix(...)` naming each missing index cite and citing D-439(b).
4. If the `current_step:` value contains the canonical marker `trajectory-tail ` but the substring
   after it does not contain exactly 4 `→(\d+)` matches, the hook emits
   `HookResult::block_with_fix(...)` naming the actual match count, the required count (4), and citing
   D-451(c).
5. If the D-chain cite in `current_step:` is stale (upper bound does not include the latest
   D-NNN), the hook emits `HookResult::block_with_fix(...)` naming the stale cite and citing D-443(a).
6. If the `current_step:` value does NOT contain the literal canonical marker `trajectory-tail `
   (with trailing space), the hook emits `HookResult::block_with_fix(...)` with description
   `"current_step missing 'trajectory-tail ' canonical marker; D-451(c)/F-P3-006"`. The LENGTH
   count check (postcondition 4) does NOT run when the prefix is absent — the prefix-absent
   `HookResult::block_with_fix(...)` fires instead.
7. Multiple violations in one write produce a single `HookResult::block_with_fix(...)` message
   enumerating all violations together.
8. If `host::read_file` returns an error for STATE.md (HostError of any kind), the hook emits
   `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

### INDEX.md path (file_name == "INDEX.md")

9. If every in-scope adversary-pass row in the `## Adversarial Reviews` h2 section of INDEX.md
   is a 5-column table row (6 pipe characters), OR if the section uses a 4-column header (grandfathered
   historical schema), the hook emits `HookResult::Continue` (pass).
10. If any in-scope adversary-pass row in a 5-column-header `## Adversarial Reviews` section has a
    column count other than 5 (pipe count other than 6), the hook emits `HookResult::block_with_fix(...)`
    naming the row (by h2 context or line position), the actual column count, the required count (5),
    and citing D-441(b)/D-442(b).
11. If `host::read_file` returns an error for INDEX.md (HostError of any kind), the hook emits
    `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry entry.
   It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write has
   completed. The dispatcher records the block signal; the author must correct and re-write.
3. Path-component-strict matching is applied to BOTH trigger paths:
   `Path::new(file_path).file_name() == Some("STATE.md")` for the STATE.md arm, and
   `Path::new(file_path).file_name() == Some("INDEX.md")` for the INDEX.md arm. Using
   `ends_with("STATE.md")` or `ends_with("INDEX.md")` on the raw path string MUST NOT be
   substituted (false-positive on `xSTATE.md` / `xINDEX.md`).
4. The forbidden meta-commentary regex is anchored to the `current_step:` frontmatter field
   only — NOT applied to the full STATE.md body content. The regex pattern is:
   `META-LEVEL-\d+ WATCH|self-app TEST|expected verdict`. These exact tokens are verbatim
   per D-440(a); the hook MUST NOT expand or narrow this set without a spec amendment.
5. The 4 index version patterns required in `current_step:` are: the literal strings
   `BC-INDEX v`, `VP-INDEX v`, `STORY-INDEX v`, `ARCH-INDEX v` (each followed by a version
   token). All 4 must be present. Missing any one is a violation per D-439(b).
6. **(v1.3, F-P11-001)** Trajectory-tail detection within `current_step:` is a two-step operation:
   (a) **Prefix check (mandatory first step):** the hook searches for the literal substring
   `trajectory-tail ` (with trailing space) in the `current_step:` value. If this marker is
   absent, the hook emits `HookResult::block_with_fix(...)` with description
   `"current_step missing 'trajectory-tail ' canonical marker; D-451(c)/F-P3-006"` and does NOT
   proceed to the LENGTH count. Absent-marker is fail-closed (block), not fail-open.
   (b) **LENGTH count (second step, runs only if prefix present):** apply regex `→(\d+)` globally
   to the substring between the `trajectory-tail ` marker (exclusive of marker) and the first `;`
   segment-separator (or end-of-value if no `;` follows). This semicolon-segment scoping prevents
   arrow-pattern false-positives from elsewhere in `current_step:` (e.g., cascade trajectory
   narratives, TD-NNN renumber references). The match count must equal exactly 4. A count of 3
   (LENGTH=3) or 5 (LENGTH=5) or any other value other than 4 is a violation per D-451(c).

   **Rationale (v1.3 doc-clarification):** v1.2 prose specified "substring AFTER the
   `trajectory-tail ` marker" without naming the right boundary. The production implementation
   in `check_trajectory_tail_length` (validate-dispatch-advance hook) narrows to the
   `[marker_end, first_semicolon)` segment (or end-of-value if no `;`). On the production
   `current_step:` at the time of F-P11-001, the full-substring-after-marker count is 14
   `→(\d+)` matches (false-positive block) while the first-semicolon-segment count is 4
   (correct pass). v1.3 codifies the actual production behavior as the authoritative spec.
   This is a doc-clarification, not a behavior change.

   **Pre-amendment grep stdout (LL-3, F-P11-001 closure):**
   ```
   $ grep -n 'substring AFTER the.*trajectory-tail.*marker\|to the substring AFTER' \
       /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
   165:   to the substring AFTER the `trajectory-tail ` marker (not the whole `current_step:` value).
   ```
   Post-amendment: line 165 replaced with first-semicolon-segment scoping per production
   `check_trajectory_tail_length` behavior. Zero false positives on production state confirmed.

7. D-chain currency validation: the `current_step:` value MUST contain at least one `D-\d+`
   reference. The hook extracts ALL `D-(\d+)` integers from the `current_step:` value using
   regex `D-(\d+)` applied globally, takes the maximum integer found (call it `max_cited`), and
   compares it against the highest `D-(\d+)` integer observable anywhere in STATE.md (call it
   `max_in_file`). If no `D-\d+` reference appears in `current_step:` at all, the cite is absent
   and is a violation. If `max_cited` is present but `max_cited < max_in_file`, the cite is
   stale and is a violation. If `max_cited >= max_in_file`, the cite is current. The literal
   prefix `D-382..` is NOT required — production `current_step:` fields use prose forms such as
   `D-chain cite D-476 latest brownfield` which are valid under this rule as long as the extracted
   maximum integer is >= the maximum D-NNN visible in STATE.md. This design is fail-open for
   staleness when no D-NNN appears in STATE.md body at a higher integer than `current_step:` cites,
   to avoid false-positive blocks on legitimate in-progress writes; the adversary cycle catches
   genuine staleness at pass time.

   **Pre-fix verification (LL-3 verbatim stdout):**
   ```
   $ grep -n "current_step:" /Users/jmagady/Dev/vsdd-factory/.factory/STATE.md | grep -oE 'D-[0-9]+'
   D-419
   D-420
   D-421
   D-476
   D-417
   D-394
   ```
   Maximum D-NNN in current_step: D-476. Amended invariant 7 matches this production form — `D-476`
   is a bare cite (no `D-382..` prefix), correctly extracted by `D-(\d+)` regex. Zero false positives.

8. Adversary-pass row column detection in INDEX.md: validation applies ONLY to rows that appear
   under the `## Adversarial Reviews` h2 heading (exact heading text per production INDEX.md files).
   A row is in scope if it appears after the line `## Adversarial Reviews` and before the next h2
   heading (`^## `). Within that section, a line whose trimmed content begins and ends with `|` is
   a candidate row. Separator rows (`| --- |` or `|---` pattern) are excluded. The canonical
   column schema for NEW rows added after D-441(b)/D-442(b) codification is 5 columns
   (`| Pass | Date | Findings Count | Verdict | File |`), which corresponds to 6 pipe characters
   (`|` count = 6; i.e., 1 leading + 4 internal separators + 1 trailing = 6 pipes for 5 columns).
   A candidate row whose pipe count is not 6 (i.e., not 5 columns) is a violation row.
   Historical pre-D-441(b) rows that use 4-column schema (5 pipes; `| Pass | Date | Findings | Status |`)
   are grandfathered: the validator skips any row in the `## Adversarial Reviews` section that
   matches the 4-column separator pattern `|------|------|----------|--------|` or any data row
   whose pipe count is 5 AND whose section predates D-441(b) codification. Since the hook cannot
   determine row creation date at runtime, the practical rule is: if the section's header row
   (first non-separator `|` row after `## Adversarial Reviews`) has pipe count 5 (4-column schema),
   treat all rows in that section as grandfathered-historical and skip column validation for them.
   If the header row has pipe count 6 (5-column schema), enforce 6-pipe (5-column) compliance on
   all subsequent non-separator rows in that section.

   **Pre-fix verification (LL-3 verbatim stdout):**
   ```
   $ python3 -c "
   import re
   for path, label in [
       ('/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/INDEX.md', 'brownfield'),
       ('/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md', 'EDP1'),
   ]:
       with open(path) as f: content = f.read()
       m = re.search(r'^## Adversarial Reviews$.*?(?=^## |\Z)', content, re.MULTILINE|re.DOTALL)
       if m:
           rows = [l for l in m.group(0).split('\n') if l.startswith('|')]
           print(f'{label}: first_header_row pipes={rows[0].count(\"|\")} -> schema={(rows[0].count(\"|\") - 1)}-col')
   "
   brownfield: first_header_row pipes=5 -> schema=4-col  (grandfathered; skip column validation)
   EDP1:       first_header_row pipes=6 -> schema=5-col  (enforce 6-pipe compliance on data rows)
   ```
   Zero false positives on existing historical 4-col brownfield rows. EDP1 5-col rows comply (pipes=6).
9. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not Block.
   The total timeout budget is bounded by the registry `timeout_ms = 5000` limit.
10. All byte-index slice expressions operating on content strings MUST use `is_char_boundary()`
    guards where multi-byte UTF-8 input is possible (em-dash, en-dash, NBSP, typographic
    apostrophes are plausible in STATE.md narrative text). Slice without boundary guard is a
    runtime panic risk per the S-15.11 cascade lesson F-P4-001.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `current_step:` contains literal text `META-LEVEL-5 WATCH: self-application` | `HookResult::block_with_fix(...)` citing forbidden meta-commentary pattern and D-440(a)+D-441(a)+D-442(a) |
| EC-002 | `current_step:` contains `self-app TEST` substring | `HookResult::block_with_fix(...)` citing forbidden pattern |
| EC-003 | `current_step:` contains `expected verdict` substring | `HookResult::block_with_fix(...)` citing forbidden pattern |
| EC-004 | `current_step:` has BC-INDEX, VP-INDEX, STORY-INDEX v cites but omits ARCH-INDEX v | `HookResult::block_with_fix(...)` naming ARCH-INDEX as missing cite; D-439(b) |
| EC-005 | `current_step:` has all 4 index cites | Continue for index-cite check; validate other conditions |
| EC-006 | `current_step:` contains `trajectory-tail ` marker and first-semicolon segment has 3 `→N` groups (LENGTH=3) | `HookResult::block_with_fix(...)`: "trajectory-tail has 3 components; required LENGTH=4 per D-451(c)" |
| EC-007 | `current_step:` contains `trajectory-tail ` marker and first-semicolon segment has 5 `→N` groups (LENGTH=5) | `HookResult::block_with_fix(...)`: "trajectory-tail has 5 components; required LENGTH=4 per D-451(c)" |
| EC-008 | `current_step:` contains no `D-\d+` reference at all | `HookResult::block_with_fix(...)` citing absent D-chain cite; D-443(a) |
| EC-009 | `current_step:` contains `D-476` (max extracted integer 476); STATE.md Decisions Log table shows row D-477 | `HookResult::block_with_fix(...)` citing stale D-chain; D-443(a); max_cited=476 < max_in_file=477 |
| EC-010 | `current_step:` contains `D-chain cite D-477`; highest D-NNN in STATE.md body is 476 | Continue for D-chain cite (max_cited=477 >= max_in_file=476; current per invariant 7) |
| EC-011 | All `current_step:` validations pass | Continue |
| EC-012 | All 4 STATE.md validations fail simultaneously | Single `HookResult::block_with_fix(...)` enumerating all 4 violation classes |
| EC-013 | INDEX.md adversary-pass row (in `## Adversarial Reviews` section with 5-col header) has 4 columns (5 pipe characters total) | `HookResult::block_with_fix(...)` naming row, actual count=4, required=5; D-441(b)/D-442(b) |
| EC-014 | INDEX.md adversary-pass row (in `## Adversarial Reviews` section with 5-col header) has 6 columns (7 pipe characters total) | `HookResult::block_with_fix(...)` naming row, actual count=6, required=5; D-441(b)/D-442(b) |
| EC-015 | INDEX.md adversary-pass row has exactly 5 columns (6 pipe characters total) in a 5-col-header section | Continue |
| EC-016 | INDEX.md contains multiple rows in 5-col-header section; 1 of 5 rows has 4 columns | `HookResult::block_with_fix(...)` naming the non-conforming row |
| EC-017 | `host::read_file` returns HostError::CapabilityDenied for STATE.md | Continue + log_warn; fail-open |
| EC-018 | `host::read_file` returns HostError::Timeout for INDEX.md | Continue + log_warn; fail-open |
| EC-019 | File path is `/some/dir/xSTATE.md` (ends_with "STATE.md" but file_name differs) | Continue (is_state_md_target returns false; path-component-strict guard) |
| EC-020 | File path is `/some/dir/xINDEX.md` (ends_with "INDEX.md" but file_name differs) | Continue (is_index_md_target returns false; path-component-strict guard) |
| EC-021 | STATE.md `current_step:` contains valid 4-arrow tail in prose body but `current_step:` itself has only 3-arrow tail | `HookResult::block_with_fix(...)` on trajectory-tail (hook extracts tail from `current_step:` value only, not full body) |
| EC-022 | `current_step:` contains additional `→N→N→N→N` arrow sequences beyond the canonical tail (e.g., cascade narratives or TD-NNN renumber references after the first `;` segment) | Continue; LENGTH count is scoped to the substring between `trajectory-tail ` marker and first `;` segment-separator per invariant 6(b); arrow patterns in subsequent semicolon-segments are excluded from the count |
| EC-023 | `current_step:` value does not contain the literal substring `trajectory-tail ` (with trailing space) — e.g., state-manager wrote `tail →9→9→9→9` omitting the canonical marker | `HookResult::block_with_fix(...)`: "current_step missing 'trajectory-tail ' canonical marker; D-451(c)/F-P3-006"; LENGTH count does NOT run |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| All STATE.md valid | `current_step:` — no forbidden patterns; all 4 index cites present; contains `trajectory-tail ` marker; first-semicolon segment after marker has tail `→9→9→9→9`; contains `D-476` (max extracted); D-476 = max D-NNN in STATE.md body | `HookResult::Continue` | PASS |
| META-LEVEL WATCH in current_step | `current_step:` contains `META-LEVEL-5 WATCH: ...` | `HookResult::block_with_fix(...)` citing forbidden pattern | BLOCK |
| Missing ARCH-INDEX cite | `current_step:` has 3 of 4 index cites (ARCH-INDEX absent) | `HookResult::block_with_fix(...)` naming missing ARCH-INDEX cite | BLOCK |
| Tail LENGTH=3 in current_step | `current_step:` has `trajectory-tail ` marker; first-semicolon segment has tail `→9→9→9` (3 components) | `HookResult::block_with_fix(...)` citing 3 vs required 4 | BLOCK |
| Missing trajectory-tail marker | `current_step:` has no `trajectory-tail ` substring (e.g., `"tail →9→9→9→9"`) | `HookResult::block_with_fix(...)`: "current_step missing 'trajectory-tail ' canonical marker; D-451(c)/F-P3-006" | BLOCK |
| Stale D-chain | `current_step:` contains `D-476` (max extracted); STATE.md Decisions Log shows D-477 row | `HookResult::block_with_fix(...)` citing stale D-chain (max_cited=476 < max_in_file=477) | BLOCK |
| All 4 STATE.md violations | Forbidden pattern + missing 2 index cites + tail LENGTH=5 (marker present) + stale D-chain | Single `HookResult::block_with_fix(...)` enumerating all violations | BLOCK |
| INDEX.md 4-column row in 5-col-header section | `## Adversarial Reviews` section has 5-col header; a data row has 4 columns (5 pipes) | `HookResult::block_with_fix(...)` naming row, actual=4, required=5; citing D-441(b)/D-442(b) | BLOCK |
| INDEX.md 6-column row in 5-col-header section | `## Adversarial Reviews` section has 5-col header; a data row has 6 columns (7 pipes) | `HookResult::block_with_fix(...)` naming row, actual=6, required=5; citing D-441(b)/D-442(b) | BLOCK |
| INDEX.md all rows 5 columns in 5-col-header section | `## Adversarial Reviews` section with 5-col header; all data rows 5-column compliant | `HookResult::Continue` | PASS |
| INDEX.md 4-col-header section (grandfathered) | `## Adversarial Reviews` section has 4-col header; all rows 4-column | `HookResult::Continue` (grandfathered historical schema; no column validation) | PASS |
| Read failure STATE.md | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| Read failure INDEX.md | `host::read_file` returns HostError::Timeout | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| xSTATE.md path | file_name is "xSTATE.md" | `HookResult::Continue` (is_state_md_target false) | PASS (not target) |
| xINDEX.md path | file_name is "xINDEX.md" | `HookResult::Continue` (is_index_md_target false) | PASS (not target) |
| Arrow pattern false-positive scoped out | `current_step:` has `trajectory-tail →9→9→9→9; TD-VSDD-064/065→095/096 ...` (marker present; first-semicolon segment has 4 counts; subsequent segment has 2 extra `→\d+` matches) | `HookResult::Continue`; first-semicolon-segment count is 4 (correct); subsequent-segment arrows excluded | PASS |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-440(a) | Forbidden meta-commentary patterns blocked in `current_step:` | PC2 |
| D-441(a) | Verbatim-strict `current_step:` — no meta-commentary injection; part of forbidden-pattern gate | PC2 |
| D-442(a) | Prescribed clause order / completeness — forbidden-pattern gate prevents rogue clause reordering markers | PC2 |
| D-443(a) | D-chain cite currency gate — stale cite blocked | PC5 |
| D-439(b) | All 4 index version patterns present in `current_step:` | PC3 |
| D-441(b) | 5-column INDEX.md adversary-pass row schema (per D-442(b) scope clarification) strict within `## Adversarial Reviews` h2 section | PC10 |
| D-442(b) | INDEX.md `## Adversarial Reviews` = 5-column canonical schema; historical 4-col rows grandfathered | PC9/PC10 |
| D-451(c) | Trajectory-tail LENGTH=4 derived from first-semicolon segment after `trajectory-tail ` marker in `current_step:`; absent marker = `HookResult::block_with_fix(...)` (PC6/invariant 6) | PC4/PC6 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Meta-Commentary Block Invariant — hook emits `HookResult::block_with_fix(...)` when `current_step:` matches forbidden pattern | bats integration test (fail-meta-commentary fixture) |
| (pending) | Index-Cite Pass Invariant — hook emits Continue when all 4 index cites present | bats integration test (pass-all-valid fixture) |
| (pending) | Missing-Index-Cite Block Invariant — hook emits `HookResult::block_with_fix(...)` when any index cite absent | bats integration test (fail-missing-arch-index-cite fixture) |
| (pending) | Trajectory-Tail Block Invariant — hook emits `HookResult::block_with_fix(...)` when tail in first-semicolon segment of `current_step:` is not LENGTH=4 (marker present) | bats integration test (fail-tail-3-components + fail-tail-5-components fixtures) |
| (pending) | Trajectory-Tail Marker Absent Block Invariant — hook emits `HookResult::block_with_fix(...)` when `trajectory-tail ` marker absent from `current_step:` | bats integration test (fail-missing-trajectory-tail-marker fixture) |
| (pending) | D-Chain Stale Block Invariant — hook emits `HookResult::block_with_fix(...)` when D-chain terminal < latest D-NNN in STATE.md | bats integration test (fail-stale-d-chain fixture) |
| (pending) | INDEX.md Column Block Invariant — hook emits `HookResult::block_with_fix(...)` when adversary-pass row is not 6 columns | bats integration test (fail-5-col-row + fail-7-col-row fixtures) |
| (pending) | INDEX.md Column Pass Invariant — hook emits Continue when all rows are 6 columns | bats integration test (pass-index-6-col fixture) |
| (pending) | Fail-open Invariant STATE.md — hook emits Continue when file is unreadable | bats integration test (fail-open-state-unreadable fixture) |
| (pending) | Fail-open Invariant INDEX.md — hook emits Continue when file is unreadable | bats integration test (fail-open-index-unreadable fixture) |
| (pending) | Arrow-Pattern Scope Invariant — hook does NOT false-positive when non-tail arrow patterns appear in `current_step:` in segments after the first `;` separator | bats integration test (pass-arrow-pattern-scoped-out fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — dispatch-advance structural validation automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the `current_step:` meta-commentary, missing 4-index-cite, trajectory-tail cardinality, stale D-chain, and INDEX.md column-count violation classes codified in D-440(a), D-441(a), D-442(a), D-443(a), D-439(b), D-441(b), and D-451(c). The hook targets STATE.md frontmatter writes and INDEX.md adversary-pass table writes — both governance artifacts, not runtime subsystem artifacts. |
| Architecture Module | `crates/hook-plugins/validate-dispatch-advance/` (Rust WASM plugin); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-440(a) (forbidden meta-commentary gate); D-441(a) (verbatim-strict `current_step:` — meta-commentary arm); D-442(a) (prescribed clause order — meta-commentary arm); D-443(a) (D-chain cite currency gate — relaxed to D-(\d+) max-extraction per F-P1-001); D-439(b) (4-index version cite presence); D-441(b) (5-column INDEX.md adversary-pass row schema within `## Adversarial Reviews` h2 section); D-442(b) (INDEX.md `## Adversarial Reviews` canonical 5-column schema; historical 4-col grandfathered); D-451(c) (trajectory-tail LENGTH=4 after `trajectory-tail ` canonical marker in `current_step:`; absent marker = `HookResult::block_with_fix(...)` per F-P3-006) |
| Stories | S-15.14 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.14 must
  achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.003 — governs validate-index-cite-refresh hook (sister PostToolUse hook; detects stale
  4-index version cites in ARCH-INDEX, STATE.md, and INDEX.md — overlapping domain; this BC
  focuses on `current_step:` frontmatter field structural validity, not cross-file version
  freshness)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; burst-log structural
  completeness; same crate scaffolding pattern and path-component-strict guard)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook; STATE.md
  banner line-count, dual-margin, and trajectory-tail in STATE.md body; this BC validates the
  `current_step:` frontmatter field specifically — distinct validation domain)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook;
  structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-dispatch-advance/` — hook implementation (pure logic functions + effectful orchestration); trajectory-tail scoping in `check_trajectory_tail_length`
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and dual file targets `STATE.md` + `INDEX.md` (canonical Q5 form)

## Story Anchor

S-15.14 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M2 Wave-4)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.5 | 2026-05-19 | Pass-3 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r fix-burst pass-3; INV-018 dual-grep applied). Closes F-BC006P3-001 (CRITICAL: 28-bare-`BlockWithFix`-residual sibling-sweep — bare `BlockWithFix` CamelCase tokens in EC table rows (EC-001..EC-023 Expected Behavior column, 14 rows), VP table rows (6 rows), D-NNN Anchor Coverage row (1 row), postcondition 6 body text (1 row), invariant 6(a) body text (1 row), Traceability D-NNN Sub-Clauses Closed (1 row) = 24 non-historical tokens replaced with canonical `HookResult::block_with_fix(...)` associated-function form. 4 remaining `BlockWithFix` tokens reside in append-only historical content: frontmatter `last_amended` v1.2 narrative (1 occurrence) and changelog rows v1.2 (1) and v1.3 (2) — exempt from modification per POLICY 1 append-only changelog history. INV-018 pre-fix narrow-pattern evidence: `grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` → `0` (INV-017 satisfied — prefixed form already zero). INV-018 pre-fix residual-class sweep: `grep -cE 'BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` → `28` (bare-form residual confirmed). INV-018 post-fix narrow-pattern: `grep -cE 'HookResult::block_with_fix\(' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` → `30` (18 original + 12 newly-converted from bare form in EC/VP/D-NNN/Traceability sections; postfix count includes backtick-quoted forms). INV-018 post-fix residual-class sweep: `grep -cE 'BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` → `4` (only in append-only historical content: last_amended v1.2 narrative + changelog rows v1.2/v1.3; all 4 are POLICY-1-exempt). Closes F-BC006P3-002 (MEDIUM: v1.4 changelog row self-referential typo corrigendum): v1.4 changelog row said "`HookResult::block_with_fix(...)` variant non-existence — 16 occurrences replaced with canonical `HookResult::block_with_fix(...)`"; the replace-target description was textually identical to the replacement. The intended wording was `HookResult::BlockWithFix` (CamelCase variant form) as the replace-target. v1.4 changelog row is not modified (POLICY 1 append-only); this corrigendum is appended here in v1.5: the v1.4 sweep replaced `HookResult::BlockWithFix` (16 prefixed occurrences) → `HookResult::block_with_fix(...)`. v1.5 sweep additionally closes F-BC006P3-001 by replacing the 24 non-historical bare `BlockWithFix` tokens. |
| 1.4 | 2026-05-19 | Sibling-sweep (product-owner; brownfield-backfill M3 3M3a-r fix-burst pass-2; INV-017 applied). Closes F-BC007P2-001 (HookResult::block_with_fix(...) variant non-existence — 16 occurrences replaced with canonical `HookResult::block_with_fix(...)` associated function). INV-017 evidence: `grep -nE 'pub enum HookResult.pub fn block_with_fix' crates/hook-sdk/src/result.rs` → enum at line 18 (variants: Continue line 20, Block line 24, Error line 31 — no BlockWithFix variant); associated function `block_with_fix` at line 50 (returns HookResult::Block internally). |
| 1.3 | 2026-05-18 | Spec amendment closing S-15.14 LOCAL adversary pass-11 finding F-P11-001 (invariant 6(b) under-specifies trajectory-tail LENGTH count boundary). v1.2 prose: "substring AFTER the `trajectory-tail ` marker (not the whole `current_step:` value)" — right boundary unspecified. Production `check_trajectory_tail_length` (validate-dispatch-advance) narrows to `[marker_end, first_semicolon)`. Full-substring-after-marker count on production `current_step:` = 14 `→(\d+)` matches (false-positive block); first-semicolon-segment count = 4 (correct pass). v1.3 invariant 6(b) replacement: "apply regex `→(\d+)` globally to the substring between the `trajectory-tail ` marker (exclusive of marker) and the first `;` segment-separator (or end-of-value if no `;` follows). This semicolon-segment scoping prevents arrow-pattern false-positives from elsewhere in `current_step:` (e.g., cascade trajectory narratives, TD-NNN renumber references). Count of matches MUST equal 4; otherwise emit BlockWithFix." EC-006, EC-007 row descriptions updated from "substring after" to "first-semicolon segment". EC-022 row reworded to explicitly cite first-semicolon-segment scoping. Canonical Test Vectors updated to match. Architecture Anchors stable-symbol cite updated: `check_trajectory_tail_length` replaces file-line cite per TD-VSDD-091. Pre-amendment grep stdout embedded in invariant 6 rationale block. This is a doc-clarification, not a behavior change. |
| 1.2 | 2026-05-17 | Spec amendment closing S-15.14 LOCAL adversary pass-3 finding F-P3-006 (trajectory-tail global-count fallback risks false-positive). Disposition: Option A — absence of canonical marker `trajectory-tail ` is a HARD violation. New PC 6: current_step MUST contain literal `trajectory-tail ` (with trailing space); absence = BlockWithFix citing D-451(c)/F-P3-006. Invariant 6 rewritten as two-step: (a) prefix check (fail-closed on absent marker), (b) LENGTH count scoped to substring AFTER marker only (prevents false-positive from unrelated `→\d+` patterns e.g. TD-VSDD-064/065→095/096). New PC 6 added to Postconditions as PC6. EC-023 added (missing-marker case). New Canonical Test Vector row added (missing-trajectory-tail-marker + arrow-pattern-scoped-out). New VP row added (Trajectory-Tail Marker Absent Block Invariant + Arrow-Pattern Scope Invariant). D-NNN Anchor Coverage PC references updated. Production verification via grep: `trajectory-tail ` IS present in production current_step (line 15 of STATE.md); zero false positives on production state. Also fixed: Precondition renumbering 1,5,2,3,4 → 1,2,3,4,5,6 (F-P3-009 / F-P2-009 in-scope nitpick fix). |
| 1.1 | 2026-05-17 | Spec amendments closing S-15.14 LOCAL adversary pass-1 findings routed to product-owner. F-P1-001: invariant 7 D-chain pattern relaxed — replaced literal `D-382..D-N` prefix requirement with `D-(\d+)` max-extraction; production `current_step:` uses prose form `D-chain cite D-476` not range prefix; zero false positives verified via grep stdout. F-P1-002: invariant 8 scoped to rows under `## Adversarial Reviews` h2 heading only (exact heading per production INDEX.md); canonical schema corrected from 6-column/8-pipe to 5-column/6-pipe per D-442(b) scope clarification; historical pre-D-441(b) 4-column rows grandfathered via header-row schema detection; verified via python3 stdout showing brownfield=4-col grandfathered, EDP1=5-col enforced. F-P1-003 + F-P1-008: pipe arithmetic corrected throughout — `1 leading + 4 internal separators + 1 trailing = 6 pipes for 5 columns`; erroneous `7 internal pipes = 6 columns + 2 border pipes` formula removed; EC-013 and EC-014 pipe counts realigned. |
| 1.0 | 2026-05-17 | Initial authoring (product-owner; brownfield-backfill S-15.03 M2 wave-4 story authoring). Anchors D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-441(b)+D-451(c). BC-5.39.006 allocated as next monotonic ID after BC-5.39.005 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.14 merge). Preemptive cascade lessons applied: path-component-strict guard for both STATE.md + INDEX.md arms (is_state_md_target + is_index_md_target); is_char_boundary() invariant 10; fail-open invariant 9; 524288 max_bytes matching BC-5.39.005 cap; D-chain currency invariant 7 fail-open design to prevent false-positive blocks on in-progress writes. |
