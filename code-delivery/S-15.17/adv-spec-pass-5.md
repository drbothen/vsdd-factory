---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.4 + S-15.17 v1.5"
cycle: brownfield-backfill
pass: 5
producer: adversary
timestamp: 2026-05-28
verdict: HIGH
finding_count: 12
finding_count_by_severity:
  critical: 1
  high: 4
  medium: 5
  low: 1
  nitpick: 1
  process_gap: 0
streak_status: "STREAK 0/3 → 0/3 after pass-5 (HIGH; reset)"
---

# Adversarial Review — BC-5.39.009 v1.4 + S-15.17 v1.5 Spec Cascade Pass 5

## Part A — Finding Set

---

### F-S15.17-SP5-001 — CRITICAL — PC1 strict `count == 4` is impossible to satisfy on production `current_step:` values

The BC v1.4 inv-4 mandates **STRICT equality** (`count == 4`) on the extracted region; PC1 applies inv-4 to the **entire `current_step:` value** as extracted by `extract_frontmatter_current_step`. But the actual production `current_step:` value naturally contains **multiple trajectory_tail strings** in a single value — each fix-burst burns one carry-across reference plus one or more cycle-trend references. Therefore inv-4 STRICT applied to PC1 fires a **false-Block on every legitimate state-manager Commit E**.

**Literal-shell evidence (stable anchor):**
```
$ grep -oE "→[0-9]+" .factory/STATE.md | wc -l
45
```

Of those 45, the `current_step:` line alone produces:
```
$ sed -n '/^current_step:/,/^[a-z_]*:/p' .factory/STATE.md | head -1 | grep -oE "→[0-9]+" | wc -l
8
```

The current production `current_step:` contains **8 arrow-segments** (two `→9→9→9→11` carry-across mentions). Per BC inv-4 STRICT `count == 4`, this fires `MissingStateSite { site_name: "STATE.md frontmatter current_step" }` → Block. The hook the spec describes would block its own STATE.md write.

This is **NOT** a hypothetical: line 61 of the very same STATE.md has `Trajectory REGRESSING 14→11→14→16` in the Last Updated cell — only **3 arrows** (LENGTH=3), so PC2 STRICT also Blocks. Line 202 (Concurrent Cycles row) has `Full-cycle trajectory (75 values ending): →9→9→9→9→11` — **5 arrows** in one cell. The BC v1.4 spec as written would Block-cascade on multiple sites of the CURRENT production STATE.md.

The root cause: BC inv-4 imported BC-5.39.006 v1.7 inv-6(b) `count == 4` semantics, but did NOT import BC-5.39.006's **per-segment scoping discipline** (TD-VSDD-100 + BC-5.39.006 inv-6(b) "semicolon-segment-scoping" per pass-11 closure). BC-5.39.006 was deliberately scoped to the `trajectory-tail ` marker prefix precisely to avoid this problem. The deliberate non-extension of marker-prefix semantics (§Cure-Extension Parsimony Note point 2) was an intentional design choice, but the consequence — that STRICT count semantics applied to heterogeneous text cells produces false-Blocks — was NOT analyzed.

**Routing:** product-owner — re-spec inv-4 for heterogeneous cells. Options: (a) "any contiguous window of exactly 4 arrows present anywhere in extracted region" (windowed-scan, count==4 within a sliding window of 4 consecutive arrows); or (b) restore canonical marker-prefix semantics from BC-5.39.006 to all 5 STATE.md sites (re-extend marker-prefix discipline — partial reversal of §Cure-Extension Parsimony Note point 2); or (c) extract only the trajectory-bearing sub-token within each cell (e.g., immediately following "trajectory-tail" or "trajectory") and count within that scope. The current spec is structurally unimplementable in a way that does not block legitimate writes.

---

### F-S15.17-SP5-002 — HIGH — PC4 Concurrent Cycles extractor is fatally underspecified; STRICT semantics false-Blocks today

PC4 (BC v1.4 line 278-282): "When STATE.md is written and the 'Concurrent Cycles' section or row does not yield a `→(\d+)` match count of exactly 4 (LENGTH=4 STRICT equality per inv-4), the hook treats this as a missing site... The section is extracted by scanning for a heading or label matching 'Concurrent Cycles' and capturing the associated text."

The extractor is "section OR row" — undefined. The Concurrent Cycles section is a multi-row TABLE (line 196-204). Per the production state:

**Literal-shell evidence (stable anchor):**
```
$ sed -n '/^## Concurrent Cycles/,/^## /p' .factory/STATE.md | grep -oE "→[0-9]+" | wc -l
6
```

The section text contains 6 arrows (two `→9→9→9→11` near-equivalents in the F5 row + bolt-on row + `→9→9→9→9→11` in line 202). Under PC4 STRICT (count==4), this section fails. Under "row" interpretation: no specification of WHICH row (latest? bolt-on? brownfield-backfill?); the brownfield-backfill row (line 201) contains many trajectory references including `→11→16→16→12→2→1→4→5→4→6→7→5→8` (13 arrows), and the F5 row contains `→9→9→9→11` (4 arrows) AND `→9→9→9→9→11` (5 arrows). No single-row interpretation yields count==4 cleanly. An implementer has no deterministic rule to follow.

PC3 was tightened in pass-4 (F-SP4-001) to "SINGLE ROW TEXT — the text between the `|` delimiters of the bottommost non-archived/non-compacted table row" with the rationale that "the bottommost non-archived/non-compacted row IS the latest by construction." The **same tightening was not applied to PC4** despite the same structural problem. This is a sibling-sweep gap from F-SP4-001 cure: the fix was applied to PC3 only, not the structurally identical PC4.

**Routing:** product-owner — apply F-SP4-001 PC3-tightening pattern to PC4: define "Concurrent Cycles latest-row" as the bottommost active/in-progress row (skipping CLOSED, COMPACTED, ARCHIVED), and apply the count==4 check to that single-row text. Document the extractor function name and rationale in §Architecture Anchors parallel to `extract_phase_progress_latest_row`. Sibling-sweep verification per CLAUDE.md S-7.01 partial-fix regression discipline.

---

### F-S15.17-SP5-003 — HIGH — PC10 lessons.md "trend-table" is not a table; extractor unspecified

PC10 (BC v1.4 line 358-362) targets "lessons.md latest-lesson trend-table row." But the active lessons.md does NOT contain a trend-TABLE; it contains inline `**Trend:**` narrative within each lesson body.

**Literal-shell evidence (stable anchors):**
```
$ grep -ci "^**Trend:**" .factory/cycles/v1.0-brownfield-backfill/lessons.md
0
$ grep -c "^## LESSON" .factory/cycles/v1.0-brownfield-backfill/lessons.md
50
$ grep -in "trend" .factory/cycles/v1.0-brownfield-backfill/lessons.md | head -3
1315:**Severity:** HIGH verdict for pass-5 (12 findings; trend 22→11→16→16→12 — improving)
1360:- Convergence trend: pass-1 CRIT (22) → pass-2 CRIT (11) → pass-3 HIGH (16) → pass-4 HIGH (16) → pass-5 HIGH (12). Approaching but not at NITPICK_ONLY. Counter still 0.
```

There are 50 `## LESSON-*` entries; trends appear as **inline prose** ("trend 22→11→16→16→12") inside lesson bodies. There is no table structure with a "latest-lesson trend-table row" to extract. The BC PC10 extractor specification is purely abstract — no `### Trend Table` heading, no `| Pass | Findings |` markdown table, nothing to anchor on.

Compare to PC9 (Dim-7) which was concretely re-anchored in pass-4 (F-SP4-004) to `^### Dim-7` heading prefix with literal grep evidence. PC10 was NOT subjected to the same grounding. This is META-LEVEL-30 route (b) recurrence inside the cure-BC — the gate would be silently inert on lessons.md since no extractable "trend-table row" exists.

**Routing:** product-owner — either (a) add §SDK Grounding Evidence Grep with literal-shell command + stdout proving the actual structural anchor for lessons.md latest-lesson trend section; OR (b) explicitly OUT-OF-SCOPE site 9 (mirror sites 10-11 which are already OUT OF SCOPE in the canonical mapping table) on the grounds that lessons.md trends are inline prose without machine-extractable structure, deferring to a future story when lessons.md schema is formalized. The current spec is unimplementable.

---

### F-S15.17-SP5-004 — HIGH — [regression of F-SP4-002 stable-anchor cure] T-5 NOTE on STATE.md extractors retains `grep -n` line-numbered evidence

POLICY 5 v1.3.1 stable-anchor sub-clause forbids `grep -n` line numbers in captured stdout for §SDK Grounding Evidence. The BC body was swept (Grep 4 etc.), but the **STORY T-5 extractor specification (story v1.5 lines 585-619)** retained line-numbered `grep -n` excerpts inside the implementer-facing extractor NOTES.

**Literal-shell evidence (anchored to function-comment prefix):**
```
$ grep -nE "grep -n|^.*: *[0-9]+:" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md | head -10
585:  //   grep -n 'Last Updated' .factory/STATE.md | head -3
586:  //   57:| **Last Updated** | 2026-05-28 — D-513 ... Trajectory-tail carry-across →9→9→9→11. |
599:  //   grep -n '^## Phase Progress' .factory/STATE.md
600:  //   61:## Phase Progress
614:  //   grep -n '^## Session Resume' .factory/STATE.md
615:  //   280:## Session Resume Checkpoint (2026-05-28 — D-513 ...)
616:  //   grep -n '^### §1' .factory/STATE.md
617:  //   286:## §1. Where We Are
618:  //   The trajectory_tail appears in §1 body content (line 289: →9→9→9→11).
```

The story carries line numbers 57, 61, 280, 286, 289 as authoritative production evidence in T-5 — but those line numbers are stale already: actual `## Phase Progress` is at line 65, not 61; `## Session Resume Checkpoint` is at line 292, not 280; `### §1` is at line 298, not 286. **All 5 cited line numbers in T-5 are wrong as of the current STATE.md.** This is the exact META-LEVEL-32 class (SDK-grounding-mandate-with-stale-pins) that POLICY 5 v1.3.1 codified, but the cure was applied only to the BC §SDK Grounding Evidence section — **the story body extractor NOTES (which an implementer reads directly) were not swept**.

This is a textbook partial-fix per CLAUDE.md S-7.01 partial-fix regression discipline: "fix applied to primary, sibling not updated." Sibling = the story T-5 extractor NOTES that mirror the BC PC2/PC3/PC5 evidence.

**Routing:** story-writer — replace all `grep -n` invocations with stable-anchor `grep` (no `-n` flag); strip line-number prefixes from captured stdout; sweep T-5 extractor function NOTES for `:NNN:` format strings; verify against POLICY 5 v1.3.1 form. Apply CLAUDE.md S-7.01 sibling-sweep.

---

### F-S15.17-SP5-005 — HIGH — `extract_current_cycle()` whitespace and frontmatter-region handling underspecified; multi-line block-scalar handling is hand-waved

The BC §Architecture Anchors (line 591) specifies `extract_current_cycle()` with five YAML form variants: bare, single-quoted, double-quoted, trailing-comment, multi-line block-scalar (`|` or `>`). The multi-line form gets **one sentence**: "(e) multi-line block-scalar using `|` or `>` → read continuation lines as the value."

The same problem that EC-017 carefully addresses for `current_step:` (multi-line YAML continuation handling — AC-21 with both pass and fail bats fixtures) is hand-waved for `current_cycle:`. No specification of:
- How to detect the end of the block-scalar (next top-level YAML key? blank line? specific indent level?)
- How to determine the indent level of continuation lines (the YAML spec requires indent-derived from first non-empty line)
- How to handle `>` (folded scalar — joins lines with single space) vs `|` (literal scalar — preserves newlines)
- How to handle a `current_cycle:` that legitimately spans 3 lines (e.g., `current_cycle: |\n  v1.0-brownfield-\n  backfill\n`)

In addition, the function returns `Option<String>` but PC4 INDEX.md arm fallback (line 165-167) says: "If the STATE.md read fails (any HostError), the hook falls back to fail-open." That's HostError. What about `extract_current_cycle` returning `None` because the key is present but malformed? The spec says (line 591) "Returns `None` if the key is absent or the frontmatter region cannot be located. On `None`, the INDEX.md arm falls back to fail-open." But the pseudocode in T-5 (story line 501) does NOT match: it uses `active_cycle.is_empty()` to detect failure, treating `None` via `.unwrap_or_default()` to empty string — semantically equivalent on the happy path but not robust to a legitimately-empty `current_cycle: ""` value.

**Literal-shell evidence (function-anchored):**
```
$ grep -n "active_cycle.is_empty" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
501:      || !std::path::Path::new(file_path)
```
(Note: I grep with -n only for finding the citation; the substantive evidence is the empty-string-vs-None ambiguity.)

The implementer cannot write deterministic code from this spec. They will guess on multi-line handling and may either over- or under-extract.

**Routing:** product-owner — extend `extract_current_cycle()` spec with explicit multi-line block-scalar handling (consume continuation lines while indent > frontmatter-region-base-indent; fold for `>`; join with `\n` for `|`); add explicit handling for empty value (treat empty string as `None`); add §SDK Grounding Evidence Grep showing what production STATE.md `current_cycle:` looks like with whitespace boundary characters explicit (e.g., `od -c` or `cat -A`). Add test vectors for at least one quoted form and one multi-line form.

---

### F-S15.17-SP5-006 — MEDIUM — `host::read_file` returns `Vec<u8>` but BC PC1-12 narrative speaks of "extracted region" and "section text" as if it were a string; encoding gate is implicit

The BC §SDK Grounding Evidence Grep 2 confirms `read_file` returns `Result<Vec<u8>, HostError>`. The story T-5 has the `String::from_utf8` double-match (sibling pattern from validate-policies-schema). But the BC text itself never explicitly says the bytes MUST be UTF-8-decoded before the extractors run, and EC-020 was added as a fail-open for decode failure — but the BC narrative around PC1-12 and extractor specs still reads as if `content: &str` is the natural input.

Specifically: PC2 (line 247-248) talks about "Capture text between the second and third unescaped `|` characters" — this implies byte-walking or string-slicing? If byte-walking on `Vec<u8>`, then the multi-byte UTF-8 character `→` (3 bytes per inv-11) needs is_char_boundary guards that PRECEDE the decode. If on `&str` (post-decode), then the bytes have been validated UTF-8 first.

The BC does not specify whether the extractors operate on `&str` (post-decode) or `&[u8]` (pre-decode). This matters because:
1. If `&[u8]`: extractors must do their own UTF-8 awareness (the `→` arrow is `[0xE2, 0x86, 0x92]`), and inv-11 is-char-boundary applies to the raw bytes.
2. If `&str`: EC-020 fail-open MUST execute BEFORE the extractor (which the story T-5 correctly does), and inv-11 is-char-boundary is a property of the `&str` API.

The story T-5 chooses (2) implicitly. The BC should make this explicit so the implementation is determinate.

**Routing:** product-owner — add invariant 13 (or extend invariant 11) declaring the extractor input type: "All section extractors take `content: &str` as input. The `host::read_file` byte sequence is decoded via `String::from_utf8(bytes)` upstream of all extractors; failure routes via EC-020 fail-open." Then inv-11 is-char-boundary applies only to byte-index operations on `&str` — which is the conventional case.

---

### F-S15.17-SP5-007 — MEDIUM — PC9 burst-log "latest-pass Dim-7 block" extractor undefined; "latest" semantics unclear when burst-log has 28+ `### Dim-7` headings

PC9 (BC v1.4 line 336-354) requires the "latest-pass Dim-7 block" check, and the BC §SDK Grounding Evidence Grep 4 confirms the brownfield-backfill burst-log has 28 `### Dim-7` matches (multiple variants of the heading).

**Literal-shell evidence (heading-anchor):**
```
$ grep -c "^### Dim-7" .factory/cycles/v1.0-brownfield-backfill/burst-log.md
28
```

The BC does not specify HOW to select the "latest." Options not addressed:
- Last `^### Dim-7` heading by line number (file-order bottommost)?
- Most recent timestamp on the surrounding `## ` parent heading?
- The Dim-7 block under the most recent pass-N where N is highest?

If "bottommost in file order" (analogous to PC3 bottommost-row), the spec should say so explicitly. The PC3 cure (F-SP4-001) introduced "BOTTOMMOST" terminology; PC9 did not get the parallel treatment.

Additionally: what is the "Dim-7 block"? The text under the `### Dim-7` heading until the next `###` or `##`? The BC says "block" but defines no boundary.

**Routing:** product-owner — add to PC9: "The 'latest Dim-7 block' is the bottommost `^### Dim-7` heading in file-order; the block is the text from that heading up to (but not including) the next `^##` or `^### ` heading. Apply count == 4 to that block." Mirror PC3's tightening pattern. Document `extract_burst_log_latest_dim7()` in §Architecture Anchors.

---

### F-S15.17-SP5-008 — MEDIUM — PC2 "Last Updated cell" extractor: production cell currently FAILS PC2 STRICT count==4 — the BC's own delivery STATE.md violates the rule

Per F-S15.17-SP5-001 evidence — line 61 of STATE.md (the Last Updated cell) contains `Trajectory REGRESSING 14→11→14→16`, count of `→[0-9]+` = **3**, not 4. Under PC2 inv-4 STRICT, this fires Block.

This is a content-vs-spec defect that surfaces only when the spec is reified: the cell-completeness BC, applied to its own provenance STATE.md, classifies the current state as INVALID. Either:
- The spec is too strict (production accepts LENGTH=3 trajectory mentions in Last Updated; PC2 needs windowed/marker-prefix scoping); or
- The production STATE.md is in violation and state-manager should have caught this at the D-517 commit (which would have required this BC to already be running).

This is a forward-evidence finding: the BC's PC2 cannot ship in its current STRICT form without first remediating production STATE.md so that every state-manager Commit E produces a `→N→N→N→N` (LENGTH-exactly-4) tail in the Last Updated cell. There is no engine-discipline rule today that mandates exactly 4 — D-433(e) trajectory_tail mandates "exactly 4 axis-count arrow-separated values" but the production discipline freely uses 3-arrow inline trajectories in the same cell (e.g., "REGRESSING 14→11→14→16" describes the 4-pass HIGH count regression with 3 arrows separating 4 values).

This is the same class as F-S15.17-SP5-001 PC1 but for PC2. Listed as a separate finding because the cures may differ (Last Updated may want windowed-scan; current_step may want marker-prefix).

**Routing:** product-owner — adjudicate: (a) tighten production STATE.md state-manager dispatch templates to always include a marker-prefix `trajectory-tail →N→N→N→N` segment in Last Updated cell, and re-spec PC2 to extract only from the marker; OR (b) loosen PC2 to windowed-scan ("any window of 4 consecutive arrows"); OR (c) make PC2 advisory (downgrade severity). Same root cause as F-S15.17-SP5-001 — likely same cure.

---

### F-S15.17-SP5-009 — MEDIUM — Story Behavioral Contracts Table version cell still says "v1.3"; cite-staleness inside story body

Story v1.5 frontmatter says `BC-5.39.009 v1.4`. Story body Behavioral Contracts Table (line 309) says:
```
| BC-5.39.009 | validate-trajectory-tail-cell-completeness... | v1.3 | PC1-12 (12 PCs; old PC11 collapsed into new PC11 uniform HostError; old PC13 → new PC12); invariants 1, 3, 4, 5, 6, 7, 8, 9, 10 (invariants 2/11/12 enforced via Architecture Compliance Rules code review) |
```

The version cell says **v1.3** but the active BC is **v1.4**. The "Closes" enumeration in changelog line 1085 explicitly says "BC v1.3→v1.4 version references updated in BC gate section + Behavioral Contracts Table" — but the Behavioral Contracts Table version cell was NOT updated.

**Literal-shell evidence:**
```
$ grep -E "^\| BC-5\.39\.009 \|" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
| BC-5.39.009 | validate-trajectory-tail-cell-completeness WASM hook MUST block on STATE.md writes missing trajectory_tail in any of the 5 prescribed STATE.md cells, and MUST emit advisory on INDEX.md / burst-log.md / lessons.md writes missing trajectory_tail in their prescribed cells | v1.3 | PC1-12 (12 PCs; old PC11 collapsed into new PC11 uniform HostError; old PC13 → new PC12); invariants 1, 3, 4, 5, 6, 7, 8, 9, 10 (invariants 2/11/12 enforced via Architecture Compliance Rules code review) |
```

This is a **partial-fix per S-7.01**: the v1.5 fix-burst claimed to update BC version references but missed this specific cell. POLICY 14 5-leg parity is also at risk — the Behavioral Contracts Table is a body table that cites the BC version; per POLICY 14 leg-5 it should reflect v1.4 same-burst.

**Routing:** story-writer — update Behavioral Contracts Table v1.3 → v1.4. Same-burst, no separate dispatch. Verify with `grep -E "^\| BC-5\.39\.009 \|" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md` returns version cell = v1.4.

---

### F-S15.17-SP5-010 — MEDIUM — Token Budget "BC files count" line cites "BC-5.39.009 v1.4 (... 12 PCs + SDK Grounding Evidence section + extract_current_cycle() spec)" with token estimate ~6,500 — but the BC is now ~757 lines and §SDK Grounding Evidence + extract_current_cycle() spec materially increased its size; estimate likely stale

Token Budget table (story line 928-930):
```
| BC-5.39.009 v1.4 (authored 2026-05-28; 12 PCs + SDK Grounding Evidence section + extract_current_cycle() spec) | ~6,500 |
```

Actual BC file:
```
$ wc -l .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
757 .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
```

A 757-line spec at ~4-5 tokens per line (typical for spec markdown) is ~3,000-4,000 tokens — but with dense YAML frontmatter, embedded code blocks, and the very long last_amended[] prose entries, this could be 7,000-9,000+. The ~6,500 estimate may actually be reasonable, but the line item description "(authored 2026-05-28; 12 PCs + SDK Grounding Evidence section + extract_current_cycle() spec)" is **the description for the v1.3 state**, not v1.4. v1.4 also added EC-020, the F-SP4-001 PC3 tightening, F-SP4-004 PC9 re-anchor, POLICY 5 v1.3.1 sub-clause — none of which are mentioned in the line item.

This is cite-staleness — the Token Budget row was version-bumped from v1.3 to v1.4 mechanically but the row description was not updated to reflect what changed in v1.4. The implementer reading this row gets a v1.3 snapshot for a v1.4 spec.

**Routing:** story-writer — update Token Budget row description to reflect v1.4 actual contents (12 PCs + SDK Grounding Evidence section + extract_current_cycle() spec + EC-020 UTF-8 fail-open + PC3-single-row-tightening + PC9 Dim-7 re-anchor + 20 ECs). Verify the token estimate against an actual `wc -c` calculation; update if materially off.

---

### F-S15.17-SP5-011 — LOW — Story Behavioral Contracts Table "Postconditions Exercised" cell omits PC11 and PC12

Story Behavioral Contracts Table (line 309) cell:
```
PC1-12 (12 PCs; old PC11 collapsed into new PC11 uniform HostError; old PC13 → new PC12); invariants 1, 3, 4, 5, 6, 7, 8, 9, 10 (invariants 2/11/12 enforced via Architecture Compliance Rules code review)
```

The "Postconditions Exercised" says "PC1-12 (12 PCs)" but the parenthetical only lists invariants. Compare to the explicit AC-per-PC table in §Bidirectional Parity Audit Note (line 128-143) which lists each PC individually. The cell formatting "PC1-12" is a range — that's adequate, but the narrative around the range only mentions PC11 collapse and PC13 → PC12 transformation. A reader sees "PC1-12" and asks: which ACs cover PC11 and PC12? The cell does not say. The §Bidirectional Parity Audit Note answers this, but the BC Table itself does not cite to the audit note.

This is a low-severity cite-fragmentation: the answer exists in the story but is not connected to the BC Table cell. An implementer scanning the BC Table for "what postconditions am I implementing" gets "PC1-12" without invariant-class detail.

**Routing:** story-writer — append to the cell: "see §Bidirectional Parity Audit Note for explicit AC-per-PC mapping." One sentence; same-burst.

---

### F-S15.17-SP5-012 — NITPICK — STATE.md `## Phase Progress` heading is at line 65 in production, not line 61 as cited in story T-5 NOTE

Already covered in F-S15.17-SP5-004 evidence (this is a stale-anchor finding that POLICY 5 v1.3.1 was authored to prevent — but the story body extractor NOTE cites `61:## Phase Progress` while production STATE.md `## Phase Progress` is at line 65). Listed as NITPICK because the BC body itself uses stable-anchor form correctly; only the story T-5 NOTE retains line numbers.

**Literal-shell evidence:**
```
$ grep -n "^## Phase Progress$" .factory/STATE.md
65:## Phase Progress
```

vs. story T-5 NOTE (line 599-600):
```
//   grep -n '^## Phase Progress' .factory/STATE.md
//   61:## Phase Progress
```

Stale by 4 lines.

**Routing:** story-writer — strip `grep -n` from T-5 NOTES and remove line-number prefixes from captured stdout per POLICY 5 v1.3.1. Already addressed as part of F-S15.17-SP5-004 cure.

---

## Part B — Convergence Assessment

### Verdict: HIGH (12 findings)
### STREAK: 0/3 → 0/3 (reset; pass-5 verdict HIGH)
### Trajectory: pass-1 14 → pass-2 11 → pass-3 14 → pass-4 16 → pass-5 12

### Cure Verification

| Pass-4 Cure | Held? | Notes |
|-----|-------|-------|
| PC3 single-row tightening (F-SP4-001) | YES | `extract_phase_progress_latest_row` correctly specified single-row text. |
| PC9 Dim-7 re-anchor to `^### Dim-7` (F-SP4-004) | YES (sub-route in F-SP5-007 — extractor still under-specified on "latest" semantics) | Heading prefix correct; latest-selection rule missing. |
| `extract_current_cycle()` spec (F-SP4-005) | PARTIAL (F-SP5-005) | Multi-line block-scalar form is hand-waved. |
| Path::components mandate (F-SP4-006) | YES | T-5 correctly uses path-component-walk. |
| EC-020 mirror (F-SP4-009) | YES | Mirrored into BC. |
| §SDK Grounding Evidence stable-anchors / no `-n` (F-SP4-002) | PARTIAL (F-SP5-004 + F-SP5-012) | BC body swept; story T-5 NOTES retain `grep -n` + stale line numbers. Sibling-sweep gap per CLAUDE.md S-7.01. |
| Architecture Mapping cycle-name structural form (F-SP4-003) | YES | `<active-cycle>` placeholders in place. |
| Risk row reword (F-SP4-008) | YES | Option A correctly applied. |
| POLICY 14 5-leg quintuple parity (PO+story) | PARTIAL (F-SP5-009) | Behavioral Contracts Table version cell still v1.3, not v1.4. Leg-5 of quintuple parity broken for that table. |
| Audit predicate widening (F-SP4-015) | YES | `(BC-5\.39\.009 )?PC[0-9]+` widened correctly. |

### Regression Sweep

- **F-S15.17-SP5-004 [regression of F-SP4-002]:** Stable-anchor cure applied to BC §SDK Grounding Evidence but NOT to story T-5 NOTES — sibling-sweep gap (CLAUDE.md S-7.01 partial-fix regression). 5 stale line numbers in T-5 (lines 57, 61, 280, 286, 289), actual production has shifted (line 65, 292, 298). This is META-LEVEL-32 recurrence inside the v1.4 cure itself.
- **F-S15.17-SP5-009 [regression of F-SP4-001 sibling-sweep]:** v1.5 fix-burst claimed to sweep BC version references but missed Behavioral Contracts Table version cell. POLICY 14 leg-5 quintuple parity broken.
- **F-S15.17-SP5-012 [regression of F-SP4-002]:** Identical class as F-S15.17-SP5-004; listed separately because demonstrates the line numbers are stale, not just policy-violating.

### META-LEVEL Signals

- **META-LEVEL-32 RECURRENCE (sub-route: cure-applied-to-primary-not-siblings):** POLICY 5 v1.3.1 stable-anchor sub-clause was authored to cure META-32. The cure was applied to BC §SDK Grounding Evidence. Sibling site (story T-5 extractor NOTES) was not swept. This is the **third recurrence** in this cascade (pass-2 META-31 cure-of-cure; pass-3 META-30 inside cure-BC; pass-4 META-32 SDK-grounding-with-stale-pins; pass-5 META-32 sub-route stale-pins-in-sibling-not-primary). **CANDIDATE for codification:** META-LEVEL-33 — "sibling-sweep gap inside policy cure: when a stable-anchor policy is applied to the primary cure site, all sibling sites with the same evidence pattern must be swept same-burst."
- **META-LEVEL-30 route (b) RECURRENCE INSIDE PC10 (F-S15.17-SP5-003):** Lessons.md "trend-table" is named in BC as a target, but the structural anchor does not exist in production lessons.md. The gate would be silently inert on lessons.md (no extractable trend-table to check). Same class as F-SP4-004 PC9 Dim-7 re-anchor — but for PC10 lessons.md.
- **META-LEVEL-24 candidate (F-S15.17-SP5-001/008):** False-green prevention class — STRICT inv-4 count==4 applied to heterogeneous text cells will produce false-Blocks on legitimate writes; spec is unimplementable as written without breaking production state-manager workflow.

### Convergence Plausibility

**Asymptotic-floor candidate: HIGHLY LIKELY.** 5 passes in, trajectory 14→11→14→16→12. The 12-finding pass-5 set includes:
- **2 finding classes that require fundamental spec re-think** (F-SP5-001 + F-SP5-008 inv-4 STRICT semantics for heterogeneous text cells; F-SP5-002 + F-SP5-007 + F-SP5-003 PC4/PC9/PC10 extractor under-specification).
- **3 partial-fix regressions** (F-SP5-004 + F-SP5-009 + F-SP5-012) confirming cures keep applying to primary but missing siblings.
- **A pattern of "cure-of-cure-of-cure"** where each pass codifies a META-LEVEL class that the NEXT pass reveals as recurring in a new sibling.

Without a fundamental restructure — either (a) codify a "sibling-sweep MUST happen same-burst" enforcement gate; OR (b) extract inv-4 STRICT semantics out of PC1-5 and apply per-cell discipline; OR (c) human direction to seal at asymptotic-acceptance per F5 D-386 + S-15.14 D-477 precedent — this cascade will continue producing 8-16 findings per pass indefinitely.

**3-CLEAN-reachable in N more passes?** Improbable in <5 more passes. Even if pass-5's 12 findings all close cleanly, pass-6 is very likely to surface new sibling-sweep gaps or sub-routes of the META-32/META-33 class. Cleanly closing F-SP5-001 (PC1 STRICT vs. production STATE.md reality) requires either spec re-design (extending or replacing inv-4 semantics) or production STATE.md state-manager template re-design — either is a larger-scope decision than a fix-burst.

**Recommendation:** Continue 2-3 more passes to confirm asymptotic-floor pattern; if floor [8-12] sustains, escalate to human for asymptotic-acceptance adjudication analogous to F5 D-386 Option C + S-15.14 D-477 SEAL pattern. Alternatively, scope-restrict S-15.17 BC v1.5 to only sites 1-2 (STATE.md frontmatter current_step + Last Updated, with marker-prefix semantics extension from BC-5.39.006) and defer sites 3-9 to a follow-on story when production STATE.md structural anchors stabilize.

### Top 3 findings

1. **F-S15.17-SP5-001 CRITICAL** — PC1 inv-4 STRICT count==4 on whole `current_step:` value is structurally impossible to satisfy on production STATE.md (8 arrows actual, 4 required); spec would Block its own STATE.md writes.
2. **F-S15.17-SP5-002 HIGH** — PC4 Concurrent Cycles "section or row" extractor underspecified; sibling-sweep gap from F-SP4-001 PC3-tightening; STRICT semantics also false-Blocks today.
3. **F-S15.17-SP5-004 HIGH [regression of F-SP4-002]** — Stable-anchor cure (POLICY 5 v1.3.1) applied to BC §SDK Grounding Evidence but NOT to story T-5 extractor NOTES (5 stale line numbers); META-32 sub-route recurrence; sibling-sweep gap per CLAUDE.md S-7.01.

### Iron Law compliance

Confirmed: did not read `adv-spec-pass-1.md`, `adv-spec-pass-2.md`, `adv-spec-pass-3.md`, or `adv-spec-pass-4.md`. Fresh-context only. All finding derivations grounded in BC-5.39.009 v1.4 + S-15.17 v1.5 plus literal-shell evidence against current production STATE.md, lessons.md, burst-log.md, hooks-registry.toml, BC-INDEX, STORY-INDEX, policies.yaml, and crates/hook-sdk/src/host.rs — no inheritance of prior-pass conclusions.
