---
document_type: behavioral-contract
level: L3
version: "1.12"
status: draft
producer: product-owner
timestamp: 2026-07-30T00:00:00Z
phase: v1.0-feature-engine-discipline-pass-1
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-28.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-29.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-30.md
  - .factory/cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-1.md
input-hash: "1053116"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
extracted_from: null
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-07-30"
  - "2026-07-30 (v1.1)"
  - "2026-07-30 (v1.2)"
  - "2026-07-30 (v1.3)"
  - "2026-08-03 (v1.4)"
  - "2026-08-04 (v1.5)"
  - "2026-08-04 (v1.6)"
  - "2026-08-04 (v1.7)"
  - "2026-08-04 (v1.8)"
  - "2026-08-04 (v1.9)"
  - "2026-08-05 (v1.10)"
  - "2026-08-05 (v1.11)"
  - "2026-08-05 (v1.12)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.010
section: "5.39"
last_amended: |-
  2026-08-05 (v1.12) — EC table aligned with v1.11 normative postconditions (F-P6-001 Option 1 propagation; story-writer routing): EC-002 corrected (primary-newer-than-index direction → advisory + Continue per PC2a; prior text erroneously prescribed Block for this direction); EC-005 description updated (block trigger is B2≠B3 per PC13b, not B2≠B1 — parenthetical was semantically incorrect under v1.11); EC-019 updated (Class A Arm1 primary-newer direction is advisory per PC2a, not a block violation; cross-arm output is advisory + E1 block, not combined-block); EC-034 added (PC13a advisory path: B2==B3 AND B1≠B2 → advisory + Continue; state-manager STORY-INDEX update pending; no internal inconsistency); EC-035 added (PC2b block path: index-newer-than-primary → Block; anomalous direction). Canonical Test Vector "A Arm1 — stale" corrected (advisory + Continue per PC2a; mutant extended with PC2b block case). Gate Spec §Parts B, D, E run_part_b_arm1 prose extended: v1.11 PC13a/13b split described inline (prior prose described only the PC40/v1.5 volatile-check change, leaving the v1.11 behavioral change undescribed). VP table updated: "A Arm1 Stale-Index Block" renamed to "A Arm1 Index-Newer-than-Primary Block (PC2b)" to match the actual PC2b condition; "A Arm1 Primary-Newer-than-Index Advisory (PC2a)" and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)" added as new pending VP entries. (product-owner; F-P6-001 Option 1 propagation.) [Prior: 2026-08-05 (v1.11) — PC2 directional carve-out (F-P6-001 human-approved Option 1): primary-newer-than-index direction downgraded to advisory ("primary newer than index; state-manager index update pending; Class A BLOCK suspended"); index-newer-than-primary retains BLOCK (anomalous; no POLICY 3 ordering explanation). PC13 two sub-cases (F-P6-001): B2==B3 AND B1≠B2 → advisory (POLICY 3 ordering artefact; STORY-INDEX internally consistent); B2≠B3 → retains BLOCK (STORY-INDEX internal inconsistency). Both carve-outs mirror PC3/PC12 POLICY 3 rationale. PC4a verbatim assertion strengthened (F-P6-002): test-writer MUST assert COMPLETE formatted string by equality check; .contains()-only on substrings is NON-CONFORMING. PC5 ≥6-field no-v-token state defined normatively (F-P6-018): ≥6 non-empty fields AND no \bv([0-9]+\.[0-9]+)\b in field 6 → RowPresentNoVersion; non-empty counting confirmed canonical; empty cell → counts as 5 non-empty → RowPresentNoVersion, field 6 unread. PC36 corpus updated 2→3 (F-P6-012): ADR-037 adopted last_amended: |- in v1.1 amendment; grep confirmed 3 files. PC40 transitional clause corrected (F-P6-007): both prior preconditions satisfied in pass-5 burst (S-21.07 added to ADR-037 §Context; ARCH-INDEX.md removed from S-21.07 inputs:); PC40 vacuous for S-21.07; "no permanent weakening" guarantee holds; "widening" characterization of ARCH-INDEX.md in is_volatile_path WITHDRAWN — pattern 6 is normative conformance per ADR-037 §Decision 2; 77 stories remain in scope. Architecture Anchors corrected (F-P6-004): extract_bc_index_version_state replaces extract_bc_index_version (wrapper deleted by F-P4-016 in pass-5 burst; spec named the deleted symbol); is_volatile_path(path: &str) -> bool + parse_story_volatile_inputs(content: &str) -> Vec<String> replace check_volatile_inputs (never existed; actual shape is two public fns + inline check in run_arm_b1). §Traceability Stories + §Story Anchor: v1.4 in flight → v1.5 in flight (F-P6-005). §VP Anchors: v1.1 Changelog erratum added — "VP table extended to 17 entries" was a planning annotation; VP-102..VP-118 anchored to S-21.07 post-merge per D-945; state-manager allocates at post-merge burst; no VP table was written into BC body. Gate Spec pseudocode: PC2a/PC2b split in run_part_a_arm1 Version arm; PC13a/PC13b split in B1 check; check_volatile_inputs reference replaced with is_volatile_path + parse_story_volatile_inputs. (product-owner; S-21.07 pass-6 fix burst.) [Prior: 2026-08-05 (v1.10) — PC5 self-contradiction fixed (F-S2107-P4-022): split into two-level locator/body-table predicates; RowMalformed redefined as "locator-matched line (conditions (1)+(2) satisfied) with <5 fields after escape-aware split" — eliminating the contradiction where the normative three-condition candidacy predicate made RowMalformed empty by construction. PC5 candidate-selection order added (F-S2107-P4-005): full-file scan must prefer first (1)+(2)+(3)-satisfying line; RowMalformed only when ALL locator-matched lines fail (3); first-match-wins implementation is NON-CONFORMING. Postcondition 4a pinned normatively (F-S2107-P4-025): prescribed advisory text is MUST-verbatim with <id>/<N> as only substitutions; omitting "Registration status cannot be determined" and "Verify BC-INDEX body-table registration manually" is NON-CONFORMING. Postcondition 13 expanded with three-category enumeration (F-S2107-P4-006 ruling): hook MUST enumerate stale/fabricated/algorithm-divergent as possible explanations without asserting which applies; classify_provenance heuristic picking one label is NON-CONFORMING; invariant 11 governs; AC-009 stops requiring classification. Invariant 11 SHOULD→MUST. Postcondition 22 prescribed message added citing POLICY 14 leg 3 (F-S2107-P4-008 sibling sweep: postconditions 2 and 7 both cite leg 5 correctly; postcondition 20 cites leg 4 correctly; gap was postcondition 22 only). PC36 block-scalar normative requirement added (F-S2107-P4-004 coupling): extract_frontmatter_field MUST handle |- block scalars; returning "|-" is NON-CONFORMING; corpus 2 occurrences (this BC + S-21.07 story); load-bearing for Class E1 enforcement on governing artifacts. PC40 transitional clause updated (F-S2107-P4-013 BC-side ruling): "no permanent weakening" guarantee requires exhaustive ADR-037 §Context enumeration; S-21.07 absent from 19-story table; correction routed to architect; story-writer removes volatile ARCH-INDEX.md input. Gate Spec pseudocode updated for two-level predicates. Architecture Anchors: extract_bc_index_version updated v1.9→v1.10 terminology; extract_frontmatter_field block-scalar requirement added. Story Anchor and §Traceability Stories TBD→S-21.07. (product-owner; pass-5 fix burst.) [Prior: 2026-08-04 (v1.9) — PC5 fourth state `RowMalformed`: a candidate line matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) was found but has <5 non-empty fields after escape-aware splitting — it is NOT a valid body-table row (likely a Changelog entry, subsystem-section row, or notes table that incidentally carries the BC ID link). `RowMalformed` disposition: advisory + Continue; NEVER reaches postcondition 4 blocking path. This state is distinct from `RowAbsent`: a candidate line WAS found; the found-but-malformed case cannot be collapsed into RowAbsent without triggering false BLOCKs. Narrows `RowAbsent` to exclusively mean "no candidate line found at all." Normative body-table row recognition predicate specified: condition (1) starts with `|`; condition (2) first non-empty field matches `^\[X\]` link form or equals `X` plain form; condition (3) total non-empty field count ≥5. First-cell link form alone is insufficient — a 4-field line `| [BC-5.39.010](path) | title | draft | v1.6 |` satisfies condition (2) but fails condition (3) and is NOT a body-table row. Corpus-validated 2026-08-04: 0 RowMalformed lines in real BC-INDEX (all 1,983 BC-ID-matching lines have ≥5 fields); forward-looking protection. Postcondition 4a added: advisory message prescribing manual verification. Gate Spec `run_part_a_arm1` match extended to four arms. (product-owner; resolves internal contradiction discovered by implementer during v1.8 implementation.) [Prior: 2026-08-04 (v1.8) — PC5 column-anchored locator: state classification now uses escape-aware column count (5 fields → RowPresentNoVersion unconditionally; 6+ fields → Version(v) from 6th column) — token-search approach was a spec gap because story IDs like `S-15.01` in the Stories column match bare-form `\bv?([0-9]+\.[0-9]+)\b`, producing Version("15.01") instead of RowPresentNoVersion; 194 of 1,943 canonical rows carry such story IDs (load-bearing count). Escape-aware splitting required: `\|` within version-chain cells is non-splitting literal; naive `|` split inflates field count. PC13 two-phase algorithm: Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`) covers 58 BC-section rows; Phase 2 mandatory-v inline (`\bv([0-9]+\.[0-9]+)\b`) covers 30 Token Budget rows; prior optional-v bare form excluded — produces story-ID collision (29 rows / 6 stories), BC-section-number collision (Token Budget `BC-5.39.010 v1.7` → `5.39` extracted before `1.7`), and ACs-column collision (S-21.07 `DEFERRED v1.6` in rightmost ACs field). Corpus-validated 2026-08-04: 1983 total rows; 1943 five-field (RowPresentNoVersion); 40 six-field (Version(v)); 194 story-ID hazard rows; 1 ACs-column hazard row (S-21.07/BC-5.39.010). PC5 also corrects: version-chain extraction algorithm — latest (rightmost) `\bv([0-9]+\.[0-9]+)\b` match in 6th field. (product-owner; closes F-S2107-P3-001 + PC13 two-phase. Prior v1.7 fixes retained.) [Prior: 2026-08-04 (v1.7) — PC5 corrected: BC-INDEX canonical shape is 5-column (`| BC ID | Title | Status | Capability | Stories |`); version-chain cell is ad-hoc 6th column present on only 40 of 1983 body-table rows (corpus 2026-08-04, adversary pass-3 verified); `extract_bc_index_version` rearchitected from two-state `Option<String>` to three-state `RowAbsent` / `RowPresentNoVersion` / `Version(v)` — two-state `None` conflating the first two misdiagnosed ≥1,712 correct registrations as structural faults per F-S2107-P3-001. Postcondition 4 expanded: `RowAbsent` + version > "1.0" → block (unchanged; genuine structural fault); `RowPresentNoVersion` → silent-continue (5-column canonical shape is standard for ~98% of rows; advisory would be unactionable noise). Part B postconditions note and invariant 11 corrected: `1acf3c6` reclassified from "fabricated" to ALGORITHM-DIVERGENT per ADR-036 §Decision 4 — produced by rc.23 CACHE binary trailing-newline-stripping algorithm, not fabricated; no PROVENANCE-BREAK annotation was warranted; Pass-30 M02 POLICY 18 violation claim for `1acf3c6` retracted. Invariant 11 title updated to three-category taxonomy (stale / fabricated / algorithm-divergent). Gate Spec `run_part_a_arm1` pseudocode updated to reflect three-state match. PC40 confirmed as-written conformant — F-S2107-P3-002 is implementation non-conformance to existing spec, not a spec defect; no PC40 amendment warranted. (product-owner; closes F-S2107-P3-001 spec-side; closes ADR-036 §Decision 4 annotation corrections routed at D-952.) [Prior: 2026-08-04 (v1.6) — Class D (finding-ID namespace advisory in Closes/Refs lines) descoped entirely; active gated classes now A, B, E only. `is_cycle_artifact` dispatch branch marked DEFERRED; `.factory/cycles/` removed from registry path_allow. Premise unsound against unstandardized Closes/Refs convention: six shapes measured across both cycle burst-logs (`**Closes:**`=70, `**Closes (per …):**`=13, no-colon bold=13, non-bold=12, hyphen-form=8); PC31 failed three iterations (v1.2 plain-colon→0 matches; v1.3 bold-bare-colon→20/34; v1.5 bold-word-boundary-colon→86/96 bold but 0/20 non-bold); v1.5 measurement taken against wrong cycle. PC28-PC33 DEFERRED; postconditions 16-18/24 DEFERRED; invariant 6 DEFERRED — all IDs preserved per POLICY 1 append-only. Knowledge preserved in §Deferred Scope with follow-up story target S-21.08 (E-21 epic). PC34 VP-path correction, PC40 volatile-input precondition, invariant-6 I/O-vs-content adjudication, and all Class A/B/E amendments from v1.4-v1.5 survive intact. (product-owner; human-approved scope decision 2026-08-04.) [Prior: 2026-08-04 (v1.5) — Amendment 1 (PC31): Closes/Refs regex corrected to `^\*\*Closes\b[^:]*:\*\*`/`^\*\*Refs\b[^:]*:\*\*` — bare-colon form `^\*\*Closes:\*\*` matched only 20 of 34 burst-log Closes lines (corpus check 2026-08-04, full-file grep); 14 missed (parenthetical `**Closes (per ...):**` x11 + bare-word `**Closes per ...:**` x3); Refs = 0 corpus instances, retained forward-looking; PC31a scope-count advisory added (postcondition 24). Amendment 2 (PC34): VP path `ss-*/VP-*.md` → flat `^VP-[0-9]+\.md$` with VP-INDEX.md exclusion (corpus 2026-08-04: zero ss-* subdirs; 102 VPs flat); epics clause added (dispatch.rs carried arm without PC34 counterpart). Amendment 3 (invariant 6 adjudication): CapabilityDenied/Timeout on cycle artifact is BLOCKING per PC33/postcondition 18/invariant 5; invariant 6 scopes to finding-content verdicts only; postcondition 18 expanded to include Timeout. Amendment 4 (PC40): volatile-input precondition for Class B Arm B1 per ADR-037 §Decision 4; scan story inputs: for volatile patterns; emit prescribed advisory + Continue if found; transitional (vacuous post-remediation); EC-032 added. (product-owner.) [Prior: 2026-08-03 (v1.4) — PC13: bounding-section heading-match predicates changed from exact equality to prefix-with-word-boundary (^## Behavioral Contracts\b, ^## Token Budget\b); 133 of 144 production stories use ## Token Budget Estimate or ## Token Budget Estimate (MANDATORY), which exact equality skipped, causing stale Token Budget citations to go undetected; corpus check (2026-08-03) confirmed zero false positives on .factory/stories/*.md; ## Edge Cases (148 occurrences) remains excluded. Architecture Anchor for extract_story_bc_version_citations updated. Exact-equality non-conformance note added. (product-owner.) [Prior: 2026-07-30 (v1.3) — PC13: bounding section added (scan confined to ##Behavioral Contracts + ##Token Budget sections; ≥9 spurious blocks from Edge Cases rows eliminated); dual version-token format (\bv?([0-9]+\.[0-9]+)\b covers both bare 1.2 and v-prefixed v1.2); LAST rightmost pipe-field algorithm stated. PC31: bold-markdown form (**Closes:**/**Refs:**) required to match D-444(c) real burst-log format; union scan not else-if. PC38 + postcondition 21: non-decreasing relation (∀i: date[i] ≤ date[i+1]); equal same-day dates PERMITTED; EC-030/031 + test vectors added. Amendment 4: no spec change — PC29 (2 MiB) and PC33 (NotFound advisory+Continue on cycle artifact) already unambiguous; fault is purely implementational. PC32: O- deliberately non-excluded per D-449(d)(i); ruling made explicit. POLICY 14 five-leg parity; v1.1 modified[] entry restored (missing since initial authoring — irony: this hook checks modified[] monotonicity but not modified[]↔Changelog row correspondence, so it structurally cannot catch this defect in its own governing BC). (product-owner.) [Prior: 2026-07-30 (v1.2) — Registry entry corrected: tools = [...] array replaced with tool = "^(Edit|Write|MultiEdit)$" regex string (field name singular + MultiEdit added; POLICY 13 ESCAPE-SCOPE-PARITY). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5. BC-version-pin datum-copy ruling added to Postconditions §Part A Arm2. (product-owner.) [Prior: 2026-07-30 (v1.1) — Part A Arm2 (story-file-side trigger) added; advisory rationales made explicit for every advisory arm; Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected; Class A coverage-gap routing replaced with correctly-sized latency-gap explanation; Invariant 11 (fabricated vs stale hash provenance) added; EC-026/027/028/029 added; Gate Spec updated with run_part_a_arm2; VP table extended to 17 entries. (product-owner; coordinator review.) [Prior: 2026-07-30 (v1.0) — Initial authoring (product-owner; pre-pass-30 fix-burst). BC-5.39.010 allocated after BC-5.39.009. input-hash d248fc3 per hook-authoritative marketplace binary. lifecycle_status: draft.]]]]]]]]
---

# BC-5.39.010: validate-cross-site-correspondence WASM hook MUST block on stale BC-INDEX version-cite after a BC frontmatter bump (Class A Arm1), stale story body BC-table and Token Budget citations after a story edit (Class A Arm2), STORY-INDEX three-way input-hash inequality (Class B), and frontmatter version↔last_amended text-prefix mismatch and modified[] date-decrease (Class E); Class C count/enumeration parity is not mechanically checkable in WASM; Class D (finding-ID namespace advisory in Closes/Refs lines) deferred pending Closes/Refs convention standardization

## Description

The `validate-cross-site-correspondence` WASM hook enforces value-equivalence invariants across
multiple artifact sites that hold the same semantic datum. Six consecutive adversarial review
passes (passes 28-30) produced the same class of finding: a BC or story was correctly updated
at its primary site, but one or more secondary sites holding the same datum were left stale.
In all observed failures **the secondary site was present — its value was stale**. A
presence-only gate would have passed every one of the six failures. Every invariant in this BC
compares values across sites, not the existence of sites.

This BC specifies three mechanically-gateable classes (A, B, E) via a single WASM hook named
`validate-cross-site-correspondence`, and one honest gap (Class C). All three gated arms fire
PostToolUse (Edit/Write); none prevent a write; all signal after the write has completed. Class A
has two arms: Arm1 fires on a BC file write and checks BC-INDEX.md; Arm2 fires on a story file
write and checks the story's own BC version citations against each cited BC's current frontmatter
version. Both arms close the same recurring failure pattern from complementary directions with no
directory enumeration required. Part B covers Class B (three-way input-hash equality per POLICY 18
D-923). Part E covers Class E (frontmatter version↔last_amended text-prefix parity + modified[] monotonicity per
POLICY 14 leg 4 / POLICY 17). Class D (finding-ID namespace advisory in Closes/Refs lines) is
deferred pending Closes/Refs convention standardization; see §Deferred Scope.

Class C is documented in §Honest Gap. It is NOT deferred arbitrarily — it is declared ungatable
by a generic WASM hook for specific structural reasons, and routed to a Rust workspace integration
test per POLICY 21.

## Preconditions

### Part A — Arm1 (BC-File Trigger): BC-INDEX Version-Cite Correspondence (Class A)

1. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^BC-[0-9]+\.[0-9]+\.[0-9]+\.md$` (regexp on
     basename — path-component-strict per BC-5.39.008 precedent)
   - The file is under `.factory/specs/behavioral-contracts/` (checked via
     `components().any(|c| c.as_os_str() == "behavioral-contracts")`)
   - The file is NOT `BC-INDEX.md` itself (basename guard)
2. The hook reads the BC file via `host::read_file` and extracts the YAML frontmatter `version:`
   field. Strip surrounding quotes. Result: decimal string like `1.18`.
3. The BC identifier is derived from the basename by stripping `.md` (e.g., `BC-5.39.010`).
4. The hook reads `.factory/specs/behavioral-contracts/BC-INDEX.md` via `host::read_file` with
   `max_bytes = 1048576` (1 MiB) and `timeout_ms = 3000`. The 1 MiB cap prevents the
   META-LEVEL-24 silent-truncation false-green class.
5. The hook locates the body-table row for the BC ID post-frontmatter (pattern `^\| \[<id>\]` or
   `^\| <id> \|`). **BC-INDEX canonical table shape is 5-column: `| BC ID | Title | Status | Capability | Stories |`.**
   A version-chain cell is an **ad-hoc 6th column** present on only **40 of 1983** body-table rows
   (corpus 2026-08-04, escape-aware count: total 1983; 5-field 1943; 6-field 40).

   **Escape-aware splitting is required.** The version-chain cell uses `\|` (backslash+pipe) as
   an internal separator between version tokens (e.g., `v1.3 \| v1.4 \| v1.5`). The row splitter
   MUST substitute `\|` → placeholder before splitting on `|`, then restore — so that `\|` within
   the version-chain cell does NOT create additional column boundaries. A naive `|` split on a
   version-chain row produces 15+ fields instead of 6; the escape-aware split yields exactly 6.
   Corpus verification (2026-08-04):
   `python3 -c "import re; lines = open('BC-INDEX.md').readlines(); [...]"` →
   `5-field rows: 1943 / 6+-field rows: 40 / total: 1983`.

   **Column-count-anchored state classification.** After escape-aware splitting, count non-empty
   (whitespace-stripped) fields. The `extract_bc_index_version_state` function MUST return one of
   **four** normative states — **NOT** a two-state `Option<String>` that conflates the first two:
   - **`RowAbsent`**: **no candidate line found at all** for this BC ID post-frontmatter. No line
     matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) exists in BC-INDEX.md. This
     is the genuine structural fault: a previously registered BC whose INDEX row was dropped.
     `RowAbsent` means **exclusively** "no candidate line found" — it does NOT cover found-but-
     malformed lines.
   - **`RowPresentNoVersion`**: row found AND non-empty field count is exactly **5** (the 5-column
     canonical shape). This state is determined **structurally by column count alone** — **no
     token search is performed on any field of the row**. This is the canonical normal state for
     exactly **1,943 of 1,983** rows (corpus 2026-08-04). It is NOT a defect.
   - **`Version(v)`**: row found AND non-empty field count is **≥6** AND the 6th non-empty field
     contains at least one `\bv([0-9]+\.[0-9]+)\b` token. The 6th field is the version-chain cell.
     Extract the **latest (rightmost)** such token (mandatory `v` prefix — all real version-chain
     tokens use `v` prefix). Exactly **40 of 1,983** rows are in this state (corpus 2026-08-04).

   **≥6-field / no-v-token case (normative; F-P6-018)**: row found AND non-empty field count is
   **≥6** AND the 6th non-empty field contains **no** `\bv([0-9]+\.[0-9]+)\b` token → classified
   as **`RowPresentNoVersion`**. Rationale: the version-chain cell exists structurally (≥6 fields)
   but carries no recognizable v-prefixed version token. This is a forward-looking state for future
   BC-INDEX annotations in the 6th column that are not version chains. Disposition is identical to
   the 5-field `RowPresentNoVersion` case: `HookResult::Continue` silently with no version
   comparison.

   **Empty-cell counting (normative)**: field count uses **non-empty** (whitespace-stripped) fields.
   A 6-column row (6 pipe-delimited cells) with any one cell blank has 5 non-empty fields →
   classified as `RowPresentNoVersion` regardless of structural column count. The 6th positional
   cell is never read in the 5-count case. Both escape paths (≥6 fields / no v-token, and ≤5
   non-empty fields from a blank cell) produce the same safe outcome: `Continue` silently.

   - **`RowMalformed`**: a **locator-matched line was found** — a line satisfying the locator
     predicate (conditions (1)+(2); see Normative Recognition Predicates below) — but after
     escape-aware splitting the total non-empty field count is **<5** (1–4 fields). The line
     satisfies the locator predicate but fails the body-table row predicate. It is a structurally
     different table (Changelog entry, subsystem-section row, notes table, or other Markdown table)
     that incidentally carries the BC ID link or plain-ID pattern in its first cell. Disposition:
     `host::log_warn` advisory + `HookResult::Continue` — see postcondition 4a. **`RowMalformed`
     is distinct from `RowAbsent`**: a locator-matched line WAS found; the found-but-malformed case
     MUST NOT be collapsed into `RowAbsent` (which would trigger postcondition 4's blocking path
     and produce false BLOCKs). Corpus count (2026-08-04): **0 RowMalformed lines** in real
     BC-INDEX — all 1,983 locator-matched lines have ≥5 fields. This state is forward-looking
     protection.

   **Normative recognition predicates — two-level (F-S2107-P4-022 resolution).** Row classification
   uses two separate predicates, eliminating the apparent contradiction between RowMalformed's
   "locator-matched line found with <5 fields" and the prior three-condition candidacy wording:

   **Locator predicate**: a line satisfies the locator predicate for BC ID X if and only if BOTH
   conditions hold: (1) the line starts with `|`; AND (2) after escape-aware splitting, the first
   non-empty field's stripped content matches `^\[X\]` (link form: `[BC-5.39.010](path)`) OR
   equals `X` exactly (plain form: `BC-5.39.010`). A line satisfying (1)+(2) is called a
   **locator-matched line**. The locator predicate governs which lines are examined; it does NOT
   guarantee the line is a valid body-table row.

   **Body-table row predicate**: a locator-matched line is a **valid BC-INDEX body-table row** if
   and only if condition (3) also holds: after escape-aware splitting, the total non-empty field
   count is **≥5**. Condition (3) is required because first-cell link form alone cannot
   discriminate body-table rows from other tables: a 4-field line
   `| [BC-5.39.010](ss-05/BC-5.39.010.md) | title | draft | v1.6 |` satisfies the locator
   predicate but fails condition (3) — it is `RowMalformed`, not a body-table row.

   **Four-state classification summary**: `RowAbsent` (no locator-matched line anywhere in the
   file); `RowMalformed` (locator-matched line found; field count <5; fails body-table row
   predicate); `RowPresentNoVersion` (locator-matched AND exactly 5 fields); `Version(v)`
   (locator-matched AND ≥6 fields).

   **Candidate-selection order — prefer valid over malformed (F-S2107-P4-005 resolution)**:
   when multiple locator-matched lines exist in BC-INDEX.md, the hook MUST scan the **full file**
   and return the state of the **first locator-matched line that also satisfies condition (3)**.
   `RowMalformed` is returned only when ALL locator-matched lines in the file fail condition (3);
   `RowAbsent` only when no locator-matched line exists at all. A first-match-wins implementation
   that stops on the first locator-matched line without checking whether a subsequent valid line
   exists is **NON-CONFORMING**: a single malformed line appearing before the real body-table row
   permanently silences postcondition 2 (stale-version block) and postcondition 4 (dropped-
   registration block) for that BC — the mirror-image false-negative to the false-positive class
   this state was added to prevent. See Gate Spec pseudocode for the conforming scan algorithm.

   Corpus verification (2026-08-04, escape-aware split):
   `python3 -c "..."` → **0 locator-matched lines** fail condition (3) among BC-INDEX's 1,983
   BC-ID-candidate lines; all are currently valid body-table rows. `RowMalformed` is forward-
   looking protection for anomalous lines that future BC-INDEX maintenance might introduce.

   **Why column-anchored, not token-search.** A token-search implementation (scanning all pipe
   fields for any `\bv?([0-9]+\.[0-9]+)\b` match) is **NON-CONFORMING** for two reasons:

   (a) **RowAbsent/RowPresentNoVersion conflation (F-S2107-P3-001 blast radius ≥1,712)**: a
   two-state `Option<String>` mapping both states to `None` misdiagnoses ≥1,712 correct
   registrations as dropped rows (`RowAbsent` false positives). Per F-S2107-P3-001 corpus
   analysis: 1,983 − 40 version-cell rows − at most 231 v1.0 advisory rows = ≥1,712 false BLOCKs.

   (b) **Story-ID column collision (194 rows)**: the Stories column (field 5) of 194 of the 1,943
   canonical rows contains a story ID of the form `S-NNN.NN` (e.g., `S-15.01`) whose digits
   `NNN.NN` match the bare-form pattern `\bv?([0-9]+\.[0-9]+)\b`. A token-search extractor returns
   `Version("15.01")` for such rows instead of `RowPresentNoVersion`, then compares `15.01`
   against the BC's actual version (e.g., `1.0`) and blocks falsely. The 194 count is the
   load-bearing corpus figure — it is why column-anchoring is necessary rather than merely tidier.
   Corpus verification (2026-08-04):
   `grep -E '^\| \[BC-[0-9]+\.[0-9]+\.[0-9]+\]' BC-INDEX.md | grep -vE '\| v[0-9]+\.[0-9]+' | grep -cE '\| S-[0-9]+\.[0-9]+'`
   → **194**.

   An implementation returning `Option<String>` that maps both `RowAbsent` and `RowPresentNoVersion`
   to `None` is **NON-CONFORMING**: it misdiagnoses ≥1,712 correct registrations as structural
   faults (blast radius confirmed, F-S2107-P3-001). **Additionally**: the test `corpus_arm_a1`
   MUST NOT select only from the 40 version-cell rows — it MUST include at least one BC from the
   1,943 `RowPresentNoVersion` majority; selecting only version-chain rows cannot detect this
   failure class (the corpus test picked BC-1.17.001, one of the 40, allowing F-S2107-P3-001 to
   survive three passes undetected).
6. `RowPresentNoVersion` route: the hook proceeds directly to postcondition 4 (silent-continue).
   No version comparison is performed. `Version(v)` route: the version token is the LAST
   (rightmost) `\bv([0-9]+\.[0-9]+)\b` match in the 6th column's cell content, representing the
   current version in the chain; it is normalized by stripping the leading `v`; both values
   (frontmatter `version:` and extracted token) compared as case-sensitive decimal strings after
   normalization (postconditions 1-2). Note: the 6th column may contain multiple version tokens
   separated by `\|` (e.g., `v1.3 \| v1.4 \| v1.5`); the rightmost token is always the current.

### Part A — Arm1 read failure semantics

7. `host::read_file` for the BC file returns any HostError: `HookResult::block_with_fix(...)`
   naming the error class and path. Primary target, PostToolUse — no fail-open path
   (BC-5.39.008 v1.6 fail-closed ruling).
8. `host::read_file` for BC-INDEX.md returns `HostError::NotFound`: `host::log_warn` advisory +
   `HookResult::Continue` (legitimate bootstrap).
   `HostError::CapabilityDenied` or any capability-class error on BC-INDEX.md:
   `HookResult::block_with_fix(...)` naming the error. Sandbox misconfiguration on a secondary
   target is blocking.

### Part A — Arm2 (Story-File Trigger): Story BC-Citation Currency (Class A)

9. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^S-[0-9]+\.[0-9]+.*\.md$` (basename)
   - The file is under directory component `stories` (path-component-strict)
   - The file is NOT `STORY-INDEX.md` itself (basename guard)
10. The hook reads the story file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the YAML frontmatter `behavioral_contracts:` sequence.
    If `behavioral_contracts:` is absent or empty: Arm A2 skips entirely (`HookResult::Continue`).
11. For each BC ID in `behavioral_contracts:`, the hook derives the BC file path deterministically:
    - Strip `BC-` prefix from the ID; split on `.`; take first component as the major section
      integer S (e.g., `BC-5.39.010` → S=5)
    - Path: `.factory/specs/behavioral-contracts/ss-<S zero-padded to 2 digits>/<BC-ID>.md`
    - Example: `BC-5.39.010` → S=5 → `ss-05` → `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md`
    - Example: `BC-6.26.001` → S=6 → `ss-06` → `.factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md`
    - No directory enumeration required; path is mechanically derived from the BC ID.
12. For each BC, the hook reads the BC file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the BC's `version:` from frontmatter.
13. Within the story file content, **the scan is CONFINED to the two BC-citation sections**,
    identified by the following heading-match predicates applied to each `^## ` boundary line:

    - **Behavioral Contracts section start predicate**: a heading line H matches iff H satisfies
      `^## Behavioral Contracts\b` — the text after `## ` begins with the byte sequence
      `Behavioral Contracts` immediately followed by a word boundary (`\b`): a space, `(`, or
      end-of-line. This covers all corpus variants measured 2026-08-03 against
      `.factory/stories/*.md` (139 total sections): `## Behavioral Contracts` (131),
      `## Behavioral Contracts Table` (6), `## Behavioral Contracts Referenced` (1),
      `## Behavioral Contracts (BC Count: NNN)` (1).

    - **Token Budget section start predicate**: a heading line H matches iff H satisfies
      `^## Token Budget\b` — the text after `## ` begins with `Token Budget` immediately
      followed by a word boundary. This covers all corpus variants measured 2026-08-03 (144
      total sections): `## Token Budget` (11), `## Token Budget Estimate` (39),
      `## Token Budget Estimate (MANDATORY)` (94).

    Both predicates MUST be evaluated as regex prefix-with-word-boundary matches, NOT as exact
    equality. An implementation using `heading == "Behavioral Contracts"` or
    `heading == "Token Budget"` (string equality) is **non-conforming**: it silently skips 133
    of 144 production Token Budget sections (`## Token Budget Estimate` and
    `## Token Budget Estimate (MANDATORY)`) and leaves stale Token Budget citations undetected —
    which is one of the two failure modes the BC's H1 title explicitly promises to catch.
    The prior v1.3 text named exact heading text and was misread as exact equality by the
    implementer; this clause corrects that by stating the predicate form normatively.

    **False-positive safety**: the corpus check found zero other `^## ` headings in
    `.factory/stories/*.md` that match either predicate. `## Edge Cases` (148 occurrences)
    begins with `Edge` — it does NOT match `^## Behavioral Contracts\b` or `^## Token Budget\b`,
    so the ≥9-spurious-block regression introduced by an unbounded scan remains prevented.

    Each matched section spans from the matching H2 line to the next `^## ` heading (or EOF for
    the final section). Rows outside these two sections MUST NOT be scanned. Rationale: the Edge
    Cases section, Out-of-Scope section, and other narrative sections in a story file contain
    references of the form `BC-S.SS.NNN EC-0NN` and descriptions that incidentally carry
    version-like tokens (e.g., `v1.0`, `v1.17`); an unbounded scan generates ≥9 spurious
    blocking violations on any story that documents BC edge cases in a table, making the gate
    unwritable for its own governing story.

    Within each BC-citation section, a **version citation** is a table row (contains `|`) where
    the BC ID is present as an **exact word-boundary token**: the text `BC-S.SS.NNN` must be
    bounded by `\b` on both sides. Implementations MUST NOT use a plain `line.contains(bc_id)`
    test. Version extraction uses a **two-phase algorithm**:

    **Phase 1 — Pure-version field**: scan all pipe-delimited fields of the row. If any field's
    stripped content matches `^v?([0-9]+\.[0-9]+)$` exactly (the entire field is exclusively a
    version number, optionally v-prefixed), use it. Strip the leading `v` if present. This covers
    the `## Behavioral Contracts` body table where the Version column is an isolated field like
    `1.7` or `v1.6`. Corpus count (2026-08-04): **58 rows** across all story files.

    **Phase 2 — Inline v-prefixed token (fallback)**: if Phase 1 finds no pure-version field,
    scan fields in **REVERSE order** (rightmost first) for the pattern `\bv([0-9]+\.[0-9]+)\b`
    (**mandatory `v` prefix**). Return the first match found. This covers `## Token Budget` rows
    where the BC ID and version appear inline in a single field (e.g., `BC-5.39.010 v1.7 (full
    text, 33 ECs...)`). Corpus count (2026-08-04): **30 rows** across all story files.

    If neither phase produces a match: the row is NOT a version citation; skip it
    (`HookResult::Continue` for that BC). Absence is not a violation.

    **Why the prior `\bv?([0-9]+\.[0-9]+)\b` bare form (v1.3–v1.7) is NON-CONFORMING:**

    The optional-v bare form creates three collision classes confirmed by corpus analysis
    (2026-08-04, 197 story files):

    (1) **Story-ID collision** (29 rows / 6 stories): story files use tables like `| BC ID | Title
    | Trace |` where the Trace column carries the implementing story ID (`S-1.03 implements
    HookResult...`). The bare form `1.03` matches `\bv?([0-9]+\.[0-9]+)\b`, extracting `1.03` as
    the cited version. The referenced BCs (BC-2.01.001 at `v1.0`, BC-2.02.001 at `v1.1`, etc.)
    are not at version `1.03` → false BLOCK on 29 rows across `S-0.03`, `S-1.03`, `S-2.06`,
    `S-3.01`, `S-4.07`, `S-8.09`. Corpus command:
    `python3 -c "...BC rows where S-N.NN in rightmost Trace field and no Version col..." → 29`.

    (2) **BC-section-number collision** (Token Budget rows): Token Budget rows embed BC ID inline
    (`BC-5.39.010 v1.7 (full text...)`). The bare-form rightmost algorithm extracts `5.39` (from
    `BC-5.39.010`) before `v1.7` (the actual version), because `5.39` is a word-boundary match
    appearing before `v1.7` in the string. All 30 v-prefix-inline Token Budget rows and all 82
    bare-only-or-none Token Budget rows are mishandled. The two-phase algorithm's Phase 2
    (mandatory `v`) correctly extracts `v1.7` from `BC-5.39.010 v1.7 (...)`.

    (3) **ACs-column collision** (1 row — S-21.07, this BC): the Story ACs column contains
    `AC-001 through AC-021 (AC-012/013/014 DEFERRED v1.6 — Class D)`. The rightmost algorithm
    returns `v1.6` from this column instead of `1.7` from the Version column (which is to the
    LEFT of the ACs column). BC-5.39.010 is at `v1.7` → false BLOCK on its own governing story.
    Phase 1 resolves this by finding the isolated `1.7` pure-version field before the ACs column.

    **Corpus summary (2026-08-04):**
    `## Behavioral Contracts` rows: 58 pure-v-col + 10 v-prefix-inline + 375 bare/none = 443 total.
    `## Token Budget` rows: 30 v-prefix-inline + 82 bare/none = 112 total.
    Story-ID hazard (correctly excluded by mandatory-v Phase 2): 29 rows.
    ACs-column hazard (correctly resolved by Phase 1): 1 row.

### Part A — Arm2 read failure semantics

14. `host::read_file` for the story file returns any HostError: `HookResult::block_with_fix(...)`
    — primary target, fail-closed.
15. `host::read_file` for a BC file returns `HostError::NotFound`: `host::log_warn` advisory for
    that specific BC ID + continue checking remaining BCs.
    `HostError::CapabilityDenied` on any BC file: `HookResult::block_with_fix(...)` naming the BC
    path — sandbox misconfiguration is blocking regardless of whether the target is primary.

### Part B — Three-Way Input-Hash Equality (Class B)

Three sites that must hold identical values for each story S-NNN:

- **Site B1**: story frontmatter `input-hash:` field
- **Site B2**: STORY-INDEX.md body-table catalog row, `input-hash` token
- **Site B3**: STORY-INDEX.md aggregation blockquote `S-NNN=HHHHHHH`

### Arm B1 Preconditions (story file write)

16. PostToolUse on a story file (basename `S-[0-9]+\.[0-9]+.*\.md`, component `stories`, NOT
    `STORY-INDEX.md`).
17. Story ID extracted from basename: `^(S-[0-9]+\.[0-9]+)`.
18. `host::read_file` reads the story file; extracts `input-hash:` from frontmatter.
    If `input-hash:` is absent or null: Arm B1 skips entirely.
19. `host::read_file` reads `.factory/stories/STORY-INDEX.md` with `max_bytes = 1048576` and
    `timeout_ms = 3000`.
20. Catalog row located (post-frontmatter). `input-hash` token extracted:
    `\binput-hash\s+([0-9a-f]{7,40})\b`. If absent from row, Site B2 is absent.
21. Aggregation blockquote (`^> ` lines) scanned. Pattern `\b<id>=([0-9a-f]{7,40})\b` extracts
    Site B3. If story ID absent from blockquote, Site B3 is absent.

### Arm B2 Preconditions (STORY-INDEX.md write)

22. PostToolUse on `STORY-INDEX.md` (basename guard + component `stories`).
23. `host::read_file` reads STORY-INDEX.md with `max_bytes = 2097152` and `timeout_ms = 5000`.
24. All `S-NNN.MM=HHHHHHH` pairs extracted from aggregation blockquote region.
25. For each story ID in the blockquote set, the catalog row is located and `input-hash` token
    extracted. No individual story file reads are performed in Arm B2.

### Part B — read failure semantics

26. Arm B1: story file HostError → block (primary). STORY-INDEX.md `HostError::NotFound` →
    advisory + Continue. `HostError::CapabilityDenied` → block.
27. Arm B2: STORY-INDEX.md IS the primary target. Any HostError → block.

### Part D — Finding-ID Namespace Format (Class D) — **[DEFERRED v1.6; see §Deferred Scope]**

> **Class D is deferred as of v1.6 (human-approved scope decision 2026-08-04). All precondition**
> **IDs below (PC28–PC33) are preserved for POLICY 1 append-only traceability. The**
> **`is_cycle_artifact` dispatch branch MUST NOT be compiled into the hook in v1.6. The**
> **`.factory/cycles/` path_allow entry is removed from the registry until convention**
> **standardization is complete. See §Deferred Scope for the full knowledge base and the**
> **follow-up story brief (target: S-21.08, E-21 epic).**

28. PostToolUse on a file satisfying ANY of:
    - Basename `burst-log.md` AND component `cycles`
    - Basename `lessons.md` AND component `cycles`
    - Basename `INDEX.md` AND component `cycles`
29. `host::read_file` with `max_bytes = 2097152` and `timeout_ms = 5000`.
30. **Scope-limited extraction** (frozen-provenance exclusion by structural position):
    - `burst-log.md`: last H2 section (text from last `^## ` heading through end-of-file)
    - `lessons.md`: last `^L-EDP1-[0-9]+-[0-9]+:` anchor block; if absent, last 200 lines
    - `INDEX.md`: `## Adversarial Reviews` section (between that heading and the next `^## `)
31. Extract all Closes and Refs lines from the scoped region using the **bold-markdown form**
    that D-444(c) mandatory blocks actually use in `burst-log.md`:
    - Closes lines: `^\*\*Closes\b[^:]*:\*\*\s*(.+)$`
    - Refs lines: `^\*\*Refs\b[^:]*:\*\*\s*(.+)$`
    **Both patterns are applied independently (UNION, not `else if`)** to every line.
    Rationale: (i) plain-colon forms `^Closes:\s*` and `^Refs:\s*` match zero real burst-log
    lines — D-444(c) blocks always use bold-markdown; a plain-colon implementation would make
    Class D entirely inert, undetectably. (ii) `^\*\*Closes:\*\*` (bare-colon form) matches
    only **20 of 34** `^\*\*Closes`-class lines in `burst-log.md` (corpus check 2026-08-04,
    full-file grep); the remaining 14 use parenthetical or bare-word qualifiers:
    `**Closes (per D-413(b)):**` (11 instances) and `**Closes per D-413(b) completeness
    mandate:**` (3 instances) — both covered by `\b[^:]*:`. An implementation using the
    bare-colon form misses 41% of live Closes lines in the most recent bursts, making Class D
    exit 0 indistinguishable from "nothing scanned" for those entries. (iii) `^\*\*Refs` = **0**
    corpus instances in `burst-log.md` as of 2026-08-04 (full-file grep). The Refs pattern is
    retained **forward-looking**: D-444(c) reserves `**Refs:**` lines for reference-only
    citations in mandatory burst-log blocks; the predicate uses the word-boundary form for
    consistency and will be ready when that form appears. (iv) a compound line carrying both
    markers must have BOTH segments tokenized — an `else if` implementation drops the Refs
    segment from compound lines. Each line is tested against both patterns independently.

    **Positive-coverage assertion (PC31a)**: after line extraction from the scoped region
    (PC30), the hook MUST emit a `host::log_warn` advisory with the count of Closes/Refs lines
    matched:
    `"validate-cross-site-correspondence [Class D] scope: <N> Closes/Refs line(s) scanned in
    <section> of <file>"`. This fires before the namespace-format check loop (postconditions
    16-17). When N = 0, this makes the inert-scope condition observable to the operator — an
    inert scope (no Closes/Refs lines in the scoped region) cannot be distinguished from a
    clean scope without this advisory; exit 0 becomes vacuously passing rather than
    meaningfully passing. See postcondition 24.
32. For each Closes/Refs line: tokenize by comma and whitespace. A token is classified as
    **finding-like** if and only if BOTH conditions hold:
    - It matches shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` (starts with letter, ends with digit)
    - It does NOT start with any known-safe namespace prefix:
      `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`, `FM-`
    Finding-like tokens that do NOT start with `F-` are flagged for advisory.
    **`O-` observation IDs are DELIBERATELY non-excluded.** Rationale: D-449(d)(i) explicitly
    scopes 4-index changelog `Refs:` cells to findings (`F-`), policy gates (`PG`), and
    D-NNN decisions only — observation IDs (`O-P30-001`) are declared out of scope for those
    cells. Flagging `O-` tokens as advisory (postcondition 17) correctly surfaces misuse of
    observation IDs in `Closes:`/`Refs:` lines and enforces D-449(d)(i). If a future cycle
    legitimately needs to reference observations in burst-log Closes/Refs, add `O-` to the
    exclusion list with new D-NNN authorization. No such authorization exists as of v1.3.

### Part D — read failure semantics — **[DEFERRED v1.6]**

33. `HostError::CapabilityDenied` on the cycle artifact: block. `HostError::NotFound`:
    advisory + Continue. `HostError::Timeout` or other: block.

### Part E — Frontmatter Internal Parity (Class E)

34. PostToolUse on a file satisfying ANY of:
    - Under `.factory/specs/behavioral-contracts/ss-*/` with basename `BC-*.md` (not
      `BC-INDEX.md`)
    - Under `.factory/specs/verification-properties/` (component-strict: checked via
      `components().any(|c| c.as_os_str() == "verification-properties")`) with basename
      matching `^VP-[0-9]+\.md$`. The `[0-9]+` digit requirement naturally excludes
      `VP-INDEX.md`; an explicit basename guard `file_name() != "VP-INDEX.md"` is REQUIRED for
      defence-in-depth, because `starts_with("VP-") && ends_with(".md")` admits VP-INDEX.md.
      **Corpus verification (2026-08-04):** zero `ss-*` subdirectories exist under
      `.factory/specs/verification-properties/`; all 102 VP files sit flat at
      `.factory/specs/verification-properties/VP-NNN.md`. An implementation scoping to
      `verification-properties/ss-*/VP-*.md` is inert for all 102 VPs in the repository —
      the same defect class as `is_bc_file` admitting `BC-INDEX.md` (pass-1 BLOCKER). The
      prior v1.4 `ss-*/` clause is corrected to the flat path by this amendment.
    - Under `.factory/stories/` with basename `S-*.md` (not `STORY-INDEX.md`)
    - Under `.factory/stories/epics/` (both `stories` and `epics` path components present,
      each verified via `components().any(…)`, component-strict) with basename matching
      `^E-[0-9]+-.*\.md$`. **Rationale:** the shipped `is_frontmatter_parity_target` function
      includes an epics arm (`dispatch.rs`); the prior PC34 had no counterpart, leaving the
      spec incomplete relative to the implementation. Epic files at `.factory/stories/epics/`
      follow POLICY 14 frontmatter conventions (version/last_amended/modified[]) identical to
      story files and must be subject to Class E parity enforcement. This clause closes the
      spec gap.
35. `host::read_file` with `max_bytes = 524288` and `timeout_ms = 3000`. If content does not begin
    with `---`: `HookResult::Continue` immediately.
36. **version: extraction**: extract `version:` YAML field; strip quotes. Result: e.g., `1.6`.
    **Block-scalar parsing REQUIRED** (F-S2107-P4-004 normative coupling): the field extraction
    function `extract_frontmatter_field` MUST handle YAML block scalar indicators (`|`, `|-`,
    `>`, `>-`). When a field line is `<field>: |-` (or any block scalar form), the function MUST
    collect the block body from subsequent indented lines rather than returning the literal
    indicator string `"|-"`. Returning `"|-"` for a block scalar field is **NON-CONFORMING**:
    PC37 feeds the extracted `last_amended:` value directly into a date-prefix regex; a `"|-"`
    return causes `extract_last_amended_outer_version("|-")` to fail (length 2 < 14) → advisory
    branch → Class E1 structurally inert on any artifact using block scalar form. **BC-5.39.010
    itself uses `last_amended: |-` (per D-953 block-scalar convention for long narrative fields),
    as does S-21.07's own story file and ADR-037 — these are the three files this hook most needs
    to gate.**
    Corpus (2026-08-05): `grep -rl '^last_amended: |-' .factory/` → **3 occurrences**:
    `BC-5.39.010.md`, `S-21.07-validate-cross-site-correspondence.md`, and
    `ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md` (ADR-037 adopted
    the `|-` convention in its v1.1 amendment, 2026-08-05). Block-scalar support is a load-bearing
    normative requirement to prevent silent regression of Class E1 enforcement on the BC's own
    governing artifacts.
37. **last_amended: outermost version extraction**: apply regex
    `^\d{4}-\d{2}-\d{2}\s+\(v([0-9]+(?:\.[0-9]+)*)\)` at CHARACTER POSITION 0 of the field value.
    Captures the outermost (active) version. `[Prior:` chains appear later in the string and are
    excluded structurally by the positional anchor. If regex fails to match: `host::log_warn`
    advisory + `HookResult::Continue` (do NOT block on unparseable format).
38. **modified: extraction**: extract YAML sequence under `modified:`. Strip annotation suffixes
    (e.g., `" (v1.3)"`); compare date strings lexicographically. The required ordering relation
    is **non-decreasing (weak-ascending)**: `∀i: date[i] ≤ date[i+1]`. Equal consecutive dates
    are PERMITTED — a date[i] == date[i+1] pair does NOT violate E2. Only a strict decrease
    (date[i] > date[i+1] after suffix-strip) is a violation. Rationale: same-day multi-burst
    authoring is normal factory operation; this BC's own `modified[]` array at v1.0/v1.1
    authoring was `["2026-07-30", "2026-07-30 (v1.1)"]`, an equal-date pair that would
    self-violate under strict-ascending comparison — the spec must not prohibit its own
    authoring pattern. If absent or empty: skip E2.
39. Any HostError on the primary target file: `HookResult::block_with_fix(...)` — fail-closed.

### Part B — Volatile-Input Precondition (ADR-037 §Decision 4)

40. **Volatile-input scan (Arm B1 only; invoked after PC18, before PC19)**: after extracting
    the story's `input-hash:` field (PC18) and IF the field was present and non-null, the hook
    MUST scan the story's `inputs:` YAML sequence for paths matching the volatile patterns
    defined in **ADR-037 §Decision 2**:

    | Pattern | Matches when |
    |---------|-------------|
    | `.factory/STATE.md` | path equals `.factory/STATE.md` exactly |
    | `.factory/cycles/**/STATE.md` | path contains `.factory/cycles/` AND ends with `/STATE.md` |
    | `.factory/cycles/**/{decision-log,lessons,burst-log}.md` | path contains `.factory/cycles/` AND ends with `/decision-log.md`, `/lessons.md`, or `/burst-log.md` |
    | `.factory/stories/STORY-INDEX.md` | path equals `.factory/stories/STORY-INDEX.md` |
    | `.factory/specs/behavioral-contracts/BC-INDEX.md` | path equals `.factory/specs/behavioral-contracts/BC-INDEX.md` |
    | `.factory/specs/architecture/ARCH-INDEX.md` | path equals `.factory/specs/architecture/ARCH-INDEX.md` |

    Matching uses string equality and suffix checks only — no glob evaluator required. Collect
    all matching paths as `volatile_found`.

    If `volatile_found` is **non-empty**: emit `host::log_warn` advisory with the prescribed
    message from ADR-037 §Decision 4 and return `HookResult::Continue` for this story WITHOUT
    proceeding to PC19-21 or performing the three-way comparison:

    `"validate-cross-site-correspondence [Class B] advisory: Story <id> has volatile inputs per
    ADR-037 §Decision 2 — three-way equality is unsatisfiable until story-writer removes volatile
    inputs and state-manager recomputes the hash; Class B BLOCK suspended. Volatile path(s): <list>"`

    If `volatile_found` is **empty**: proceed to PC19 normally (full three-way check, BLOCKING).

    **Implementation note**: the volatile-pattern list MUST be kept in sync with ADR-037
    §Decision 2. Implement as a compile-time constant slice.

    **Transitional clause**: this precondition is vacuous once **all** stories with volatile inputs
    have had their `inputs:` arrays corrected per ADR-037 §Decision 5. After exhaustive
    remediation, no story matches the volatile patterns and Class B enforces full BLOCKING severity
    for all stories with no carve-outs. **The "imposes no permanent weakening" guarantee holds only
    if ADR-037 §Context enumerates every story with a volatile input exhaustively.**

    **Status as of v1.11 (F-P6-007 correction)**: both preconditions named in the v1.10
    transitional clause have been satisfied. ADR-037 v1.1 §Context added S-21.07 to the
    volatile-story table (corrected to 77 stories in ADR-037 v1.2 after S-21.07 remediation), and
    S-21.07's `inputs:` array no longer contains `ARCH-INDEX.md` (removed in the pass-5 fix burst,
    2026-08-05). **PC40 is therefore vacuous for S-21.07** — `volatile_found` is empty, and Class B
    proceeds to the full three-way check. **The "no permanent weakening" guarantee holds for
    S-21.07.**

    **Remaining remediation scope**: ADR-037 v1.2 records **77 stories** with volatile inputs.
    PC40 remains non-vacuous for those 77 stories until story-writer completes the §Decision 5
    remediation sweep.

    **Note on `.factory/specs/architecture/ARCH-INDEX.md` in the volatile pattern table** (row 6):
    including `ARCH-INDEX.md` in `is_volatile_path` is **mandated conformance** to ADR-037
    §Decision 2 — it is one of the six canonical volatile patterns listed there. The v1.10
    characterization of this addition as "widening the suppression rather than closing it" was
    **incorrect and is withdrawn**. Pattern 6 is normative per ADR-037 §Decision 2.

    This clause exists solely to prevent the self-locking failure mode described in ADR-037
    §Rationale during the transition window.

    **Arm B2 not affected**: Arm B2 (PC22-25) fires on STORY-INDEX.md writes and checks
    catalog-vs-blockquote consistency only; it does not read individual story `inputs:` arrays.
    PC40 applies exclusively to Arm B1.

## Postconditions

### Part A Arm1 postconditions

1. BC ID found in BC-INDEX body table with matching version cell (normalized): `HookResult::Continue`.
2. BC ID found with DIFFERENT version cell. Two sub-cases (directional carve-out per F-P6-001
   Option 1; rationale mirrors PC3/PC12 — POLICY 3 (`state_manager_runs_last`) guarantees the BC
   file is always written before the BC-INDEX row in a conforming burst; at the PostToolUse
   instant after a BC write, "index behind primary" is the expected intermediate state of a correct
   in-progress burst, not a defect):

   **2a. Primary newer than index** (strip-v-prefix(`fm_version`) parses as numerically greater
   than strip-v-prefix(`index_version`), comparing `major.minor` as decimal integers):
   `host::log_warn` advisory + `HookResult::Continue`:
   `"validate-cross-site-correspondence [Class A Arm1] advisory: BC-INDEX.md body-table row for
   <id> cites v<index_version> but frontmatter version: is \"<fm_version>\" — primary newer than
   index; state-manager index update pending; Class A BLOCK suspended."`.
   **Advisory rationale**: POLICY 3 forces state-manager to update BC-INDEX (secondary site) AFTER
   the product-owner writes the BC (primary site). "Index behind primary" at the primary-write
   instant is therefore guaranteed in every conforming burst — it is a burst-ordering artefact, not
   a defect. Blocking here produces a spurious `exit 2` for every correct BC authoring burst. This
   is the only direction for which the POLICY 3 ordering argument applies; sub-case (b) has no such
   explanation and retains BLOCK.

   **2b. Index newer than primary** (strip-v-prefix(`index_version`) parses as numerically greater
   than strip-v-prefix(`fm_version`)):
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md body-table row for <id> cites
   v<index_version> but frontmatter version: is \"<fm_version>\" — index is newer than primary.
   This is anomalous: the index cannot legitimately advance ahead of the BC it cites. Verify no
   index row was updated out-of-burst or under the wrong BC path. Update per POLICY 14 leg 5."`.
   **Rationale**: no burst-ordering argument explains the index carrying a higher version than the
   BC file itself. This direction is genuinely anomalous.

   **Version comparison**: strip the leading `v` from both tokens; split on `.`; compare `major`
   then `minor` as non-negative integers. If either token fails to parse as `major.minor` decimal:
   treat as anomalous and emit the 2b block — an unparseable version cell is itself an anomaly.

3. `RowAbsent` (BC ID not in BC-INDEX body table) AND frontmatter `version:` is `"1.0"`:
   `host::log_warn` advisory + `HookResult::Continue`.
   **Advisory rationale**: a v1.0 BC not yet in BC-INDEX is the expected state immediately after
   writing a new BC, before the INDEX update tool call completes in the same burst. Blocking would
   make correct BC authoring impossible — the BC file is always written before the INDEX row. This
   is NOT a "partial check = advisory" rationale; it is a "blocking causes systematic false
   positives in correct authoring bursts" rationale. When `RowAbsent` and version > 1.0, the
   hook blocks (postcondition 4) — so advisory is selective, not the default for all absent rows.
4. `RowAbsent` (BC ID not in BC-INDEX body table) AND `version:` > `"1.0"`:
   `HookResult::block_with_fix(...)` — `RowAbsent` with version > 1.0 means the BC was previously
   registered; a dropped row is a structural fault, not bootstrap ordering. This is the ONLY path
   that blocks for the absent/no-version-cell family.

   `RowPresentNoVersion` (body-table row found; row carries no `\bv[0-9]+\.[0-9]+\b` version
   cell): `HookResult::Continue` silently, regardless of frontmatter `version:`.
   **Rationale**: the 5-column canonical shape — no version cell — is the standard for
   approximately 1,943 of 1,983 BC-INDEX rows (~98%). The absence of a version cell is NOT a
   deficiency in the registration. Emitting an advisory for every write to a 5-column row would
   produce noise on ~98% of BC writes, making it indistinguishable from a genuine advisory signal
   and training operators to ignore advisories. The genuine structural fault this postcondition was
   authored to catch — a dropped registration — is exclusively the `RowAbsent` case (postcondition
   4 above), which retains its BLOCKING severity unchanged. The `RowPresentNoVersion` state is the
   correct `Continue` path for all BCs whose INDEX row has never been annotated with a version chain.
4a. `RowMalformed` (a locator-matched line was found but has <5 non-empty fields after escape-
    aware splitting): `host::log_warn` advisory + `HookResult::Continue`, regardless of
    frontmatter `version:`.
    Advisory message (**NORMATIVE — implementation MUST reproduce this text verbatim**, with
    `<id>` and `<N>` as the only interpolated substitutions):
    `"validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md contains a malformed
    candidate line for <id> (<N> fields found; expected ≥5 for a valid body-table row). This
    line is structurally not a BC-INDEX body-table row (likely a Changelog entry or notes
    table). Registration status cannot be determined from this line. Verify BC-INDEX body-table
    registration manually."`.
    The clauses "Registration status cannot be determined from this line" and "Verify BC-INDEX
    body-table registration manually" are the operator-actionable instructions — omitting them
    is **NON-CONFORMING** regardless of what other text is injected. **Test-writer MUST assert
    the COMPLETE formatted message text verbatim** (equality check on the full rendered string,
    NOT only `.contains()` assertions on individual clauses) in the `RowMalformed` unit test and
    bats fixture. A test that asserts only two `.contains()` substrings does NOT satisfy the
    MUST-verbatim requirement: it cannot detect an injected sentence, an altered phrase, or a
    dropped sentence that leaves both target substrings intact. Full-string equality is the
    load-bearing gate on the MUST-verbatim clause. The only permissible interpolation points are
    `<id>` and `<N>`; all surrounding text including the hook-name+class prefix, the preamble,
    sentence 2, and both operator-actionable clauses must be reproduced exactly.
    **Rationale**: a found-but-malformed line indicates structural ambiguity, not a confirmed
    dropped registration. The genuine dropped-registration case (no locator-matched line at all)
    is `RowAbsent` (postcondition 4). Blocking on `RowMalformed` would false-positive on any
    non-body-table line that carries the BC ID link pattern in its first cell.
    `RowMalformed` is advisory-only and NEVER reaches the postcondition 4 blocking path.
5. Multiple simultaneous BC writes: each Write is a separate hook invocation; violations are NOT
   accumulated across invocations.

**Class A latency gap (not a coverage gap)**: Arm1 fires on every BC write and catches stale
BC-INDEX immediately. Arm2 fires on every story write and catches stale story citations when the
story is edited. The remaining gap is **latency**: if a BC bumps version in a burst that does not
also touch the story, Arm2 does not fire until the story is next edited. This is not a coverage
gap — in all six observed Class A failures (passes 28-30), the story WAS edited in the same burst
as the BC bump, meaning Arm2 would have caught every observed failure. The empirically relevant
trigger path is fully covered. The POLICY 14 leg 5 obligation to update story citations same-burst
as a BC bump remains binding on the author; Arm2 provides the gate for that case.

**BC-version-pin design ruling (product-owner, 2026-07-30)**: BC-version cells in STORY-INDEX and
all BC-version citations in story bodies carry **datum-copy semantics** — they must match the BC's
current frontmatter `version:` field at the time of any story edit. They do NOT carry
reconciliation-marker semantics (a frozen record of "which BC version the ACs were last reconciled
against that could legitimately lag"). Rationale: (1) POLICY 14 leg 5's "update same-burst"
obligation is incoherent under reconciliation-marker semantics — a reconciliation marker could
intentionally lag, making a same-burst update obligation meaningless; (2) the six Class A failures
across passes 28-30 were correctly identified and closed as defects — retroactive reclassification
as "correct-but-lagging markers" has no new evidentiary basis and would reopen closed findings; (3)
STORY-INDEX.md language "BCs cells reconciled with source frontmatter" describes the act of
updating values to match their source of truth (datum maintenance), not the preservation of a
frozen point-in-time marker. **Implication for Arm A2**: Arm A2 correctly blocks on any
present-but-stale version citation in a story body. No change to Arm A2 blocking logic or behavior
is warranted by this ruling.

### Part A Arm2 postconditions

6. For a given BC ID in `behavioral_contracts:`: all version-citing rows in the story agree with
   the BC's current frontmatter `version:` (normalized): `HookResult::Continue` for that BC.
7. One or more version-citing rows show a version that does NOT match BC frontmatter version:
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm2]: story <story_id> cites <bc_id> at
   v<cited> (in <location>) but BC frontmatter version: is \"<bc_version>\". Update story
   citation same-burst per POLICY 14 leg 5."`.
   All mismatching BCs reported in one combined block message (cascade — do not stop on first).
8. No version-citing row found for a given BC ID: skip that BC, `HookResult::Continue`. The hook
   does NOT block on missing citations; only on present-but-stale citations.
9. `behavioral_contracts:` absent or empty: Arm A2 skips, `HookResult::Continue`.
10. BC file returns `HostError::NotFound` for a cited BC: advisory per PC15; continue checking
    remaining BCs. `HookResult::Continue` for that BC.
    **Advisory rationale**: a cited BC may be retired, may have a non-standard path, or may be a
    new BC added in the same burst whose file exists but the path derivation encounters an edge
    case. `HostError::NotFound` is a known-legitimate transient. `HostError::CapabilityDenied` on
    a BC file blocks (PC15) — sandbox misconfiguration is never legitimate, regardless of target
    type.

### Part B postconditions

11. Arm B1 — all three sites present and equal: `HookResult::Continue`.
12. Arm B1 — B2 or B3 absent: `host::log_warn` advisory + `HookResult::Continue`.
    **Advisory rationale**: a story with `input-hash:` populated but not yet in STORY-INDEX.md is
    the expected state when a story is first authored or when STORY-INDEX.md is updated in a later
    tool call within the same burst. This arm is NOT "advisory because the check is partial." The
    hook blocks when present values disagree (postcondition 13) — it blocks on what it CAN verify.
    Advisory fires only when absence is the anomaly, and absence has a known-legitimate
    interpretation (correct burst ordering). Blocking on absence would cause systematic false
    positives in correct new-story authoring bursts.
13. Arm B1 — B2 or B3 present but differs from B1. Two sub-cases (directional carve-out per
    F-P6-001 Option 1; rationale mirrors PC3/PC12 and PC2a — POLICY 3 (`state_manager_runs_last`)
    forces STORY-INDEX secondary sites to be updated AFTER the primary story write in every
    conforming burst):

    **13a. STORY-INDEX internally consistent, story just rewritten** (B2 == B3 AND B1 ≠ B2):
    `host::log_warn` advisory + `HookResult::Continue`:
    `"validate-cross-site-correspondence [Class B] advisory: Story <id> input-hash mismatch —
    frontmatter=<h1>; STORY-INDEX-catalog=<h2>; STORY-INDEX-blockquote=<h3>. STORY-INDEX sites
    agree with each other; story frontmatter differs. State-manager STORY-INDEX update pending;
    Class B BLOCK suspended."`.
    **Advisory rationale**: POLICY 3 forces STORY-INDEX (both catalog row and blockquote) to be
    updated AFTER the primary story write. When B2==B3 but B1≠B2, the natural explanation is that
    the story was rewritten this burst and the hash has not yet propagated — the POLICY 3 ordering
    artefact. No STORY-INDEX inconsistency exists. Blocking produces a spurious `exit 2` on every
    correct story authoring burst.

    **13b. STORY-INDEX internally inconsistent** (B2 ≠ B3, regardless of B1):
    `HookResult::block_with_fix(...)` (**NORMATIVE — implementation MUST enumerate all three
    provenance categories; a `classify_provenance` heuristic that picks one label is
    NON-CONFORMING per invariant 11**):
    `"validate-cross-site-correspondence [Class B]: Story <id> input-hash three-way mismatch:
    frontmatter=<h1> STORY-INDEX-catalog=<h2 or absent> STORY-INDEX-blockquote=<h3 or absent>.
    STORY-INDEX catalog and blockquote disagree — this is anomalous and has no burst-ordering
    explanation. Update per POLICY 18 (D-923). This hook detects inconsistency only — operator
    MUST determine which of the following applies before remediating: (a) STALE: previously valid
    hash; inputs changed after authoring; remedy: rerun \`compute-input-hash --update\` on the
    story. (b) FABRICATED: hash was never output of \`compute-input-hash --update\` at any
    revision (POLICY 18 violation); remedy: acknowledge PROVENANCE-BREAK in burst-log before
    recomputing. (c) ALGORITHM-DIVERGENT: hash produced by prior binary version per ADR-036
    §Decision 4; NOT fabricated; remedy: recompute with current authoritative binary, no
    PROVENANCE-BREAK annotation required."`.
    **Rationale for blocking on B2≠B3 regardless of B1**: internal STORY-INDEX inconsistency has
    no burst-ordering explanation — both catalog row and blockquote are written by state-manager in
    the same commit. B2≠B3 indicates partial-write, wrong-story editing, or a state-manager bug,
    none of which POLICY 3 explains.

14. Arm B2 — catalog and blockquote agree for all blockquote stories: `HookResult::Continue`.
15. Arm B2 — catalog ≠ blockquote for any story: `HookResult::block_with_fix(...)` reporting ALL
    mismatching stories in one message (cascade).

**Fabricated vs stale vs algorithm-divergent distinction — not resolved by this hook**: Class B
detects cross-site *inconsistency* only. It cannot distinguish among: a stale hash (previously
valid computed value; inputs changed after authoring; remedy: `compute-input-hash --update`); a
fabricated hash (never the output of `compute-input-hash --update` at any revision, constituting
a POLICY 18 violation; remedy: PROVENANCE-BREAK annotation in burst-log before recompute); or an
algorithm-divergent hash (legitimately computed by a prior binary version whose algorithm differs
from the current authoritative binary per ADR-036 §Decision 4; NOT fabricated; remedy: recompute
with current authoritative binary, no PROVENANCE-BREAK annotation required). All three trigger
Class B. See Invariant 11 for the three-category remediation protocol.

### Part D postconditions — **[DEFERRED v1.6; see §Deferred Scope]**

16. **[DEFERRED v1.6]** All finding-like tokens on Closes/Refs lines start with `F-`: `HookResult::Continue`.
17. **[DEFERRED v1.6]** Any finding-like token does NOT start with `F-`: `host::log_warn` per token + `HookResult::Continue`.
    Message: `"validate-cross-site-correspondence [Class D] advisory: non-canonical finding-ID
    token '<token>' on line '<line>' in <section> of <file>. Finding IDs must start with 'F-'.
    Verify this is not a phantom ID or retracted reference."`.
    **Advisory rationale**: blocking on syntactic format alone, even after namespace exclusions,
    causes false positives on tokens from unknown future namespaces added in later cycles. The
    namespace exclusion list (PC32) catches known-legitimate non-finding tokens (`D-NNN`, `S-NNN`,
    etc.). The residual advisory population is tokens that look like finding IDs (right shape, not
    in any known-safe namespace) but don't start with `F-`. Blocking these would prevent forward-
    compatibility as new ID classes are introduced. This is NOT "partial check = advisory"; it is
    "the namespace registry cannot be closed, so blocking cannot be made false-positive-free."
18. **[DEFERRED v1.6]** `HostError::CapabilityDenied` or `HostError::Timeout` on the cycle artifact:
    `HookResult::block_with_fix(...)` naming the error variant and artifact path. These are
    infrastructure faults — not finding-content decisions — so invariant 6 does not apply to
    them. Consistent with PC33 (`HostError::Timeout or other: block`) and invariant 5
    (sandbox misconfiguration is never a legitimate state). See invariant 6 adjudication
    ruling for scope clarification.

**Gap — semantic existence**: whether a cited `F-` ID exists in an adversary pass record is
infeasible in WASM (unbounded scan). Routed to Rust workspace test per POLICY 21.

### Part E postconditions

19. E1 version match: `HookResult::Continue`.
20. E1 mismatch: `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class E1]: frontmatter version: \"<ver_fm>\" does not
    match last_amended: outermost text-prefix \"(v<ver_la>)\". Update last_amended: text-prefix
    to (v<ver_fm>) per POLICY 14 leg 4 / POLICY 17."`.
21. E2 non-decreasing (∀i: date[i] ≤ date[i+1], including equal dates): `HookResult::Continue`.
22. E2 strict-decrease: first pair where date[i] > date[i+1] after suffix-strip:
    `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class E2]: modified[] date-decrease at index <i>:
    \"<date_i>\" > \"<date_i+1>\" (after suffix-strip). modified[] must be non-decreasing
    (∀j: date[j] ≤ date[j+1]). Update modified[] per POLICY 14 leg 3."`.
    **POLICY 14 leg enumeration**: leg 3 = `modified[]`; leg 4 = `last_amended:` prefix (cited
    in postcondition 20). An implementation citing "POLICY 14 leg 4" for an E2 violation is
    **NON-CONFORMING** — it routes the fixer to the wrong parity field.
23. Combined E1 + E2 violations: ONE combined block enumerating both.

### Part D — Scope-Count Advisory (postcondition 24) — **[DEFERRED v1.6]**

24. **[DEFERRED v1.6] Scope-count advisory (PC31a)**: after line extraction from the scoped region (PC30-31),
    a `host::log_warn` advisory is emitted with the count of Closes/Refs lines matched by the
    PC31 patterns:
    `"validate-cross-site-correspondence [Class D] scope: <N> Closes/Refs line(s) scanned in
    <section> of <file>"`. This fires unconditionally — before the namespace-format check loop
    (postconditions 16-17) and regardless of block state. When N = 0, the advisory makes a
    silent inert-scope observable. The scope-count advisory does NOT suppress or replace the
    namespace-format advisories (postcondition 17); both are emitted independently.

### Cross-arm combination

When a single file write triggers multiple arms, all run independently. Violations from all arms
are combined into one `HookResult::block_with_fix(...)`. Part D advisories are logged regardless
of block state.

## Honest Gap — Class C (Count/Enumeration/Word-Token Parity)

Three observed failure shapes from passes 29-30:

1. **Numeric assertion vs enumeration length** (pass-29 H01, pass-30 H04): prose claims "N gates"
   while the enumeration has N-1 items.
2. **Two count-word tokens disagreeing** (pass-30 H05): `bats:643` `Twenty-four` vs `bats:786`
   `twenty-three`.
3. **Asserted count vs runtime-derived count**: coupling gate compared two literal strings, not a
   mechanical count.

**Why NOT gatable in a generic WASM hook**:

(a) English cardinal parsing (`"twenty-three"` → 23) requires a non-closed lookup table; coupling
    a generic hook to this creates maintenance drag.
(b) The structural anchor for "the count assertion" vs "the enumeration" is file-specific. No
    generic regex reliably distinguishes the asserted count from line numbers or unrelated integers.
(c) Three-way correlation requires semantic region identification with no machine-readable separator.

**Recommended alternative (POLICY 21-compliant)**: Rust workspace integration test with per-file
fixtures. Parameterized test: (i) count enumeration items by structural anchor; (ii) extract
stated count by stable anchor; (iii) assert equality. Flagged for architect to commission as a
follow-on story.

**Class D semantic existence gap**: `F-` ID existence check requires enumerating
`adv-cycle-pass-*.md` files. Same routing: Rust workspace test or CI scan.

## Deferred Scope — Class D (Finding-ID Namespace Advisory)

**Status: DEFERRED as of v1.6. Human-approved scope decision 2026-08-04.**
**Dependency: Class D implementation requires Closes/Refs convention standardization first.**
**Target story: S-21.08 (E-21 epic). Story-writer must confirm next available ID against**
**`.factory/stories/STORY-INDEX.md` before allocating — S-21.07 is the current last entry.**

### Why Class D was deferred

Class D extracts finding-ID tokens from `Closes:` / `Refs:` lines in cycle burst-logs and emits
an advisory when tokens have non-canonical namespace format (not starting with `F-`). The premise
is unsound against the real corpus because the `Closes` convention is unstandardized.

**Corpus measurement (2026-08-04, orchestrator-measured across both cycle burst-logs
`v1.0-brownfield-backfill/burst-log.md` and `v1.0-feature-engine-discipline-pass-1/burst-log.md`):**

| Shape | Count |
|-------|-------|
| `**Closes:**` (bold bare-colon) | 70 |
| `**Closes (per …):**` (bold parenthetical) | 13 |
| `**Closes …` (bold, no colon at all) | 13 |
| `Closes …` (non-bold) | 12 |
| `Closes-…` (non-bold hyphen form) | 8 |
| **Bold total** | **96** |
| **Non-bold total** | **20** |
| **Grand total** | **116** |

The v1.5 predicate `^\*\*Closes\b[^:]*:\*\*` requires a colon — it covers 86 of 96 bold lines
but misses all 20 non-bold lines entirely, and misses the 10 no-colon bold lines. Across the full
corpus of 116 lines, v1.5 covers 86/116 = 74%. No regex is stable against an unstandardized
convention.

### Three failed PC31 iterations

| Version | Predicate | Matches in burst-log | Miss class |
|---------|-----------|---------------------|------------|
| v1.2 | `^Closes:\s*` (plain-colon) | 0 of 116 | All lines — D-444(c) blocks always bold |
| v1.3 | `^\*\*Closes:\*\*` (bold bare-colon) | 20 of 34 in one cycle | `**Closes (per ...):**` (11) + bare-word qualifiers (3) |
| v1.5 | `^\*\*Closes\b[^:]*:\*\*` (bold word-boundary-colon) | 86 of 96 bold; 0 of 20 non-bold | All non-bold lines |

Each iteration narrowed the miss for the bold subpopulation, but none covers non-bold lines at
all. The convention is unstandardized and no predicate is stable until the convention is fixed.

### Wrong cycle measurement (v1.5 defect — recorded for follow-up)

v1.5 Amendment 1 (PC31) took its corpus measurement against `v1.0-feature-engine-discipline-pass-1/burst-log.md`,
reporting the predicate score as 86/96 (90%). However, `STATE.md` `current_cycle:` is
`v1.0-brownfield-backfill` — the **active cycle** whose burst-log is the primary Class D target.
Measuring the inactive archived cycle is a defect: the active cycle has a different population.
Both cycles have non-bold lines; the defect class is cross-cycle in scope. The follow-up story
must measure against BOTH cycle burst-logs after convention standardization.

### L-EDP1- line-anchoring defect (F-S2107-P2-012, preserved for follow-up)

PC30 scopes `lessons.md` extraction to the "last `^L-EDP1-[0-9]+-[0-9]+:` anchor block."
The regex `L-EDP1-[0-9]+-[0-9]+:` expects a **two-group** pattern (e.g., `L-EDP1-001-01:`).
**Corpus measurement (2026-08-04) against `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`:**
- Pattern `^L-EDP1-[0-9]+-[0-9]+:`: **0 matches** — no L-EDP1 entry uses two-group format
- Actual format in corpus: `^L-EDP1-[0-9]+:` (e.g., `L-EDP1-061:`) — single-group
- Pattern `^L-EDP1-[0-9]+:`: **61 matches** (L-EDP1-001 through L-EDP1-061)

The two-group predicate in PC30 never matches any real `lessons.md` anchor. The fallback "last
200 lines" fires for every invocation, making scope-limited extraction structurally broken for
`lessons.md`. The follow-up story MUST fix PC30 by changing the regex to `^L-EDP1-[0-9]+:`.

### D-449(d)(i) O- observation ID non-exclusion ruling (preserved)

PC32 deliberately excludes `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`,
`FM-` from the finding-like namespace but NOT `O-`. This is intentional: D-449(d)(i) explicitly
scopes 4-index changelog `Refs:` cells to findings (`F-`), policy gates (`PG`), and D-NNN
decisions only — observation IDs (`O-P30-001`) are out of scope for those cells. Flagging `O-`
tokens as advisory surfaces misuse of observation IDs in `Closes:`/`Refs:` lines. **This ruling
MUST be carried forward intact** into the follow-up story. Any `O-` exclusion requires new D-NNN
authorization. No such authorization exists as of v1.6.

### Invariant 6 adjudication (preserved for follow-up story)

The v1.5 invariant-6 adjudication (inline above, preserved here for the follow-up story) establishes
two non-overlapping scopes:

- **Finding-content path** (advisory-only): the non-canonical namespace advisory cannot be made
  false-positive-free because the namespace registry cannot be closed. Applies to finding-content
  verdicts only.
- **I/O-error path** (blocking): `CapabilityDenied` and `Timeout` on the cycle artifact are
  infrastructure faults — fail-open silently disables Class D enforcement. Per invariant 5,
  `CapabilityDenied` on any target is blocking without exception.

The follow-up story MUST implement both paths. A catch-all error arm that swallows all
`HostError` variants as "advisory because Class D is advisory-only" is explicitly non-conforming
(TD-VSDD-059 non-conforming pattern, documented in invariant 6 above).

### Required follow-up story scope

**Target:** S-21.08, E-21 epic. Story-writer assigns; confirm next available number against
`.factory/stories/STORY-INDEX.md` (S-21.07 is current last as of 2026-08-04).

**Dependency direction:** Class D implementation DEPENDS ON Phase 1 (convention standardization).
Do not implement Phase 2 until Phase 1 is complete and all burst-log lines are in canonical form.

**Phase 1 — Convention standardization (prerequisite, must precede Phase 2):**

1. Audit both cycle burst-logs — approximately 116 existing Closes/Refs lines total across
   `v1.0-brownfield-backfill/burst-log.md` and `v1.0-feature-engine-discipline-pass-1/burst-log.md`.
2. Normalize all six shapes to a single canonical form. Recommended canonical: `**Closes:** F-XXXX`
   (bold bare-colon, matching the most common existing shape and D-444(c) intent).
3. Add a shape-enforcing gate (WASM hook or Rust workspace test) that blocks non-canonical
   Closes/Refs shapes on new burst-log entries going forward.
4. Record a corpus measurement after normalization: every shape variant must read 0.

**Phase 2 — Class D implementation (after Phase 1):**

5. Implement the `is_cycle_artifact` dispatch arm with the corrected PC30 L-EDP1 regex
   (`^L-EDP1-[0-9]+:` — single-group, not `^L-EDP1-[0-9]+-[0-9]+:`).
6. Implement PC31 against the now-stable canonical form. Re-take corpus measurement.
7. Restore `.factory/cycles/` to the registry `path_allow` list.
8. Un-retire invariant 6 and postconditions 16-18, 24 — remove DEFERRED markers.
9. Activate EC-010 through EC-013, EC-024, EC-033 and the three Class D test vectors.
10. Run the deferred VP entries (D Namespace-Excluded Token Pass, D Phantom-ID Advisory,
    D Historical-Excluded Pass).

## Invariants

1. The hook NEVER writes to any file. All arms are read-only post-write validators.
2. The hook fires PostToolUse only — writes succeed before the hook fires; the hook signals after.
3. All file-path matching uses path-component-strict guards. Raw string `ends_with` is forbidden.
4. **Fail-closed for primary targets**: any HostError on the PostToolUse trigger file →
   `HookResult::block_with_fix(...)`. No fail-open path for the primary target.
   (BC-5.39.008 v1.6 fail-closed ruling; Canonical Principle + TD-VSDD-059.)
5. **Selective fail-open for secondary targets on NotFound only**: BC-INDEX.md, STORY-INDEX.md,
   and BC files cited in story `behavioral_contracts:` return advisory + Continue on
   `HostError::NotFound` (bootstrap/ordering). `HostError::CapabilityDenied` on any secondary
   target is blocking — sandbox misconfiguration is never a legitimate state.
6. **[DEFERRED v1.6 — moves with Class D; see §Deferred Scope] Class D finding-content verdict is advisory-only, never blocking**: the non-canonical
   namespace advisory (postcondition 17) is the only verdict Class D emits based on what it
   *finds* — it never blocks based on finding-content decisions. **This invariant scopes to
   finding-content verdicts only.** `HostError::CapabilityDenied` and `HostError::Timeout` on
   the cycle artifact are infrastructure faults — not finding-content decisions — and remain
   BLOCKING per PC33 and postcondition 18. Invariant 6 does not affect them.

   **Adjudication ruling (product-owner, 2026-08-04; preserved in §Deferred Scope):** the
   apparent contradiction between invariant 6 ("Class D advisory-only") and
   PC33/postcondition 18/invariant 5 (`CapabilityDenied`/`Timeout` → block) is resolved as
   follows. Invariant 6 governs the *finding-content path*: the advisory on non-canonical
   finding-ID format is advisory because "blocking on syntactic format cannot be made
   false-positive-free" — that rationale applies to finding-content verdicts only. PC33 and
   postcondition 18 govern the *I/O-error path*: sandbox misconfiguration (`CapabilityDenied`)
   and transport failure (`Timeout`) are infrastructure faults where fail-open would silently
   disable Class D enforcement. Invariant 5 confirms: `CapabilityDenied` on any secondary
   target is blocking without exception. Neither scope overlaps — the two invariants are
   complementary, not contradictory. **This two-path distinction MUST be carried forward intact**
   when Class D is implemented in the follow-up story (S-21.08).

   **Non-conforming pattern (TD-VSDD-059):** an implementation using a single catch-all error
   arm (e.g., `Err(e) if cycle_kind.is_some()` → `log_warn` + Continue) to swallow all
   `HostError` variants on the grounds that "Class D is advisory-only" is non-conforming to
   this invariant's explicit scope. The conforming implementation handles each HostError variant
   independently: `NotFound` → advisory + Continue (PC33); `CapabilityDenied` and `Timeout`
   → `HookResult::block_with_fix(...)` naming the variant and path (PC33/postcondition 18).
7. **No cross-arm suppression**: all arms run independently; violations combine into one block;
   Part D advisories are logged regardless of block state.
8. **Class B Arm B2 cascade**: all STORY-INDEX mismatches reported in ONE combined block.
9. **is_char_boundary() guard**: byte-index slicing on extracted strings MUST use
   `is_char_boundary()` checks where multi-byte UTF-8 is possible (BC-5.39.008 inv-11).
10. **POLICY 21 compliance**: no `.sh` scripts. All gating uses WASM plugin or Rust workspace
    tests. Class C and the Class D existence-check gap are routed to Rust workspace tests.
11. **Stale vs fabricated vs algorithm-divergent hash provenance (Class B)**: this hook detects
    cross-site *inconsistency* only — it cannot distinguish among three categories, all of which
    trigger Class B: (a) **stale** — a previously valid computed value; inputs changed after
    authoring; remedy: `compute-input-hash --update`; (b) **fabricated** — a value that was never
    the output of `compute-input-hash --update` at any revision, constituting a POLICY 18
    violation; remedy: acknowledge PROVENANCE-BREAK in burst-log before running `--update`;
    (c) **algorithm-divergent** — legitimately computed by a prior binary version whose algorithm
    differs from the current authoritative binary (per ADR-036 §Decision 4); NOT fabricated, NOT
    a POLICY 18 violation; remedy: recompute with the current authoritative binary, no
    PROVENANCE-BREAK annotation required. The distinction is load-bearing for remediation. When
    Class B blocks, the fix team MUST verify which category applies before acting: (1) trace the
    stored hash to a prior `--update` invocation (stale path); (2) confirm no `--update`
    invocation ever produced that value and document PROVENANCE-BREAK (fabricated path); (3)
    identify the binary version that produced the hash — if it matches a known older algorithm,
    classify as algorithm-divergent and recompute with the authoritative binary (no annotation).

    **Correction (v1.7, per ADR-036 §Decision 4)**: Pass-30 M02 characterized S-21.04's stored
    hash `1acf3c6` as **fabricated** and asserted a POLICY 18 violation. That characterization is
    **retracted**. Per ADR-036 §Decision 4 and its §Ruling on four pass-29/30 annotations,
    `1acf3c6` is **ALGORITHM-DIVERGENT** — it was produced by the rc.23 CACHE binary
    (`~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash`) which
    strips trailing newlines via `$(cat file)` command substitution. It was a legitimately computed
    value using that binary's algorithm. No PROVENANCE-BREAK annotation was warranted; the
    Pass-30 M02 POLICY 18 violation claim for this specific hash is incorrect and is retracted.

    **The hook block message MUST enumerate all three categories as possible explanations without
    asserting which one applies** — see postcondition 13 prescribed message (F-S2107-P4-006
    ruling: AC-009 must stop requiring classification; invariant 11 governs). The implementation
    pattern of a `classify_provenance` function that picks one of three return strings based on
    B1/B2/B3 positional heuristics — including asserting `"fabricated"` when B1≠B2 and B1==B3 —
    is **NON-CONFORMING**: (1) it asserts as fact a category invariant 11 declares undecidable
    from observed hash values alone; (2) "fabricated" triggers a governance-level PROVENANCE-BREAK
    obligation; a false accusation of fabrication has operational consequences. The conforming
    implementation emits the full three-category enumeration in every Class B block message.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC file written with version "1.0" not yet in BC-INDEX body table | Advisory + Continue. New BC; registration expected in same burst. |
| EC-002 | BC bumped v1.17→v1.18; BC-INDEX row still says v1.17 | Advisory + Continue: Class A Arm1, sub-case 2a (primary newer than index; fm=v1.18 > index=v1.17; state-manager index update pending; Class A BLOCK suspended per PC2a). |
| EC-003 | BC file write; BC-INDEX.md returns `HostError::NotFound` | Advisory + Continue. Secondary target NotFound = bootstrap. |
| EC-004 | BC file write; `host::read_file` for the BC file returns `HostError::CapabilityDenied` | Block. Primary target fail-closed. |
| EC-005 | Story S-21.04 frontmatter hash "47a65c9"; STORY-INDEX catalog "4be9d21"; blockquote "S-21.04=47a65c9" | Block: Class B Arm1, sub-case 13b (B2≠B3: catalog="4be9d21" ≠ blockquote="47a65c9"; STORY-INDEX internally inconsistent; no POLICY 3 ordering explanation). |
| EC-006 | Story file has no `input-hash:` field | Continue — Arm B1 skips. |
| EC-007 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 47a65c9" | Continue — B2 == B3 (Arm B2 passes). |
| EC-008 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 1acf3c6" | Block: Class B Arm B2 mismatch. |
| EC-009 | Story file write; STORY-INDEX.md returns `HostError::CapabilityDenied` | Block. Capability-class error on secondary target. |
| EC-010 | burst-log.md; last H2 "Closes: F-S2104-P29-H01, F-S2104-P29-H02" | **(DEFERRED v1.6 — Class D)** Continue (no advisory). |
| EC-011 | burst-log.md; last H2 "Closes: B01, F-S2104-P29-H02" | **(DEFERRED v1.6 — Class D)** Advisory for "B01" (finding-like, not F-, not excluded); Continue. |
| EC-012 | lessons.md; latest L-EDP1 entry "Closes: 001" (bare numeric) | **(DEFERRED v1.6 — Class D)** `001` starts with a digit → does NOT match shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` → NOT flagged. Continue. |
| EC-013 | lessons.md; historical Changelog section contains `P45-001` | **(DEFERRED v1.6 — Class D)** Continue — Changelog is outside the scope-limited window (last L-EDP1 entry only). |
| EC-014 | BC written with `version: "1.6"` and `last_amended: "2026-07-29 (v1.6) — ..."` | Part E passes. |
| EC-015 | BC written with `version: "1.33"` and `last_amended: "2026-07-29 (v1.31) — ..."` | Block: Class E1 (v1.33 vs v1.31). |
| EC-016 | BC written with `modified: ["2026-05-14", "2026-05-18 (v1.1)", "2026-05-15"]` | Block: Class E2 (2026-05-15 follows 2026-05-18). |
| EC-017 | BC written with `modified: ["2026-05-14", "2026-05-18", "2026-05-20 (v1.3)"]` | Part E passes (ascending after suffix strip). |
| EC-018 | BC with `last_amended:` containing `[Prior: ... (v1.5) ...]`; version "1.6"; outermost prefix "(v1.6)" | Part E passes — positional anchor matches `(v1.6)` at date position; Prior tokens excluded structurally. |
| EC-019 | BC write triggers Class A Arm1 (index stale, primary-newer direction: fm=v1.18, index=v1.17) + Class E1 (fm version "1.18" ≠ last_amended "(v1.17)") | Advisory from Class A Arm1 (PC2a; BLOCK suspended) emitted to log + Block from Class E1. Arms run independently; Class A advisory does not suppress Class E1 block. Note: combined two-violation block requires PC2b direction (index-newer-than-primary) + E1 — see EC-035 for the PC2b block scenario. |
| EC-020 | VP file written with `version: "2.4"` and `last_amended: "2026-05-20 (v2.4)"` | Part E passes. VP files are in Part E scope (PC34). |
| EC-021 | Story S-21.04 written; B1 = "47a65c9"; B2 absent; B3 absent | Advisory + Continue. New story pre-registration. |
| EC-022 | `last_amended:` does not match `\d{4}-\d{2}-\d{2}\s+\(v` | Advisory + Continue. Do NOT block on unparseable format. |
| EC-023 | STORY-INDEX.md written | Only Arm B2 fires. STORY-INDEX.md does not match `S-*.md` pattern. |
| EC-024 | burst-log.md; last H2 "Refs: D-944" | **(DEFERRED v1.6 — Class D)** `D-944` matches shape BUT `D-` is in the exclusion list (PC32) → NOT flagged. Continue. |
| EC-025 | BC file write; both BC file AND BC-INDEX return `HostError::CapabilityDenied` | Combined block citing both failures. |
| EC-026 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001]`; Token Budget row cites "v1.17"; BC-6.26.001 fm "1.18" | Block: Class A Arm2 (story cites v1.17 vs BC v1.18). |
| EC-027 | Story S-21.04 written; `behavioral_contracts: [BC-5.39.010]`; BC-5.39.010.md returns `HostError::NotFound` | Advisory for NotFound BC + Continue. |
| EC-028 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001, BC-5.39.008]`; both stale | Single combined block listing both BCs (cascade). |
| EC-029 | Story written; `behavioral_contracts: [BC-6.26.001]`; BC cited only in prose, no version token in any table row | Arm A2 finds no version-citing rows → skip → Continue. |
| EC-030 | BC written with `modified: ["2026-07-30", "2026-07-30 (v1.1)"]` — two entries with equal date after suffix-strip | Part E passes. Equal consecutive dates satisfy `date[i] ≤ date[i+1]`; same-day multi-burst cadence is permitted. This BC's own v1.0/v1.1 `modified[]` is exactly this shape. |
| EC-031 | BC written with `modified: ["2026-07-30", "2026-07-29 (v1.1)"]` — date decreases after suffix-strip | Block: Class E2. `2026-07-30 > 2026-07-29` — strict decrease violates the non-decreasing requirement. |
| EC-032 | Story S-19.01 written; `inputs:` contains `.factory/cycles/v1.0-brownfield-backfill/lessons.md`; input-hash present; B2 and B3 absent | Advisory (volatile input detected per ADR-037 §Decision 2; Class B BLOCK suspended; story-writer must remove volatile input) + Continue. PC40 fires at Arm B1 before reading STORY-INDEX.md. |
| EC-033 | `burst-log.md` last H2 section contains no lines matching `^\*\*Closes\b` or `^\*\*Refs\b` | **(DEFERRED v1.6 — Class D)** Scope-count advisory "0 Closes/Refs line(s) scanned" emitted (postcondition 24) + Continue. Namespace-format check loop is skipped (nothing to tokenize). N=0 makes inert scope observable rather than vacuously passing. |
| EC-034 | Story S-21.04 written; input-hash B1="d4f8a12"; STORY-INDEX catalog B2="47a65c9"; blockquote B3="47a65c9" (B2==B3; B1≠B2) | Advisory + Continue: Class B Arm1, sub-case 13a (STORY-INDEX sites agree with each other; story frontmatter differs from index; state-manager STORY-INDEX update pending; Class B BLOCK suspended per PC13a). |
| EC-035 | BC at frontmatter v1.17; BC-INDEX row shows v1.18 (index version numerically greater than BC frontmatter) | Block: Class A Arm1, sub-case 2b (index newer than primary — anomalous; no POLICY 3 burst-ordering explanation; verify no index row was updated out-of-burst per PC2b message). |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Output | Part | Mutant | Control |
|----------|----------------|-----------------|------|--------|---------|
| A Arm1 — new BC | v1.0; no INDEX row | advisory + Continue | A Arm1 | v1.1, no row → block | v1.0 with INDEX row v1.0 → Continue |
| A Arm1 — primary-newer advisory (PC2a) | BC-5.39.008 v1.6; INDEX "v1.5" (primary newer than index) | advisory + Continue (PC2a) | A Arm1 | INDEX "v1.6" (equal) → Continue; INDEX "v1.7" (index newer, PC2b) → block | |
| A Arm2 — current | S-21.04; `behavioral_contracts: [BC-6.26.001]`; story Token Budget "v1.18"; BC fm "1.18" | Continue | A Arm2 | BC fm "1.19" while story says "v1.18" → block | `behavioral_contracts:` empty → Continue |
| A Arm2 — stale | S-21.04; story cites "v1.17"; BC fm "1.18" | block | A Arm2 | Both "v1.18" → Continue | |
| B Arm1 — match | hash "47a65c9"; catalog "47a65c9"; blockquote "47a65c9" | Continue | B Arm1 | B1="d4f8a12" (story rewritten), B2=B3="47a65c9" → advisory (PC13a); B3="4be9d21" while B2="47a65c9" (B2≠B3) → block (PC13b) | no input-hash → Continue |
| B Arm2 — mismatch | STORY-INDEX catalog "47a65c9"; blockquote "4be9d21" | block | B Arm2 | both "47a65c9" → Continue | |
| D — excluded token | "Closes: F-S2104-P29-H01, D-944" | Continue (D-944 excluded) | D (DEFERRED v1.6) | "Closes: B01" → advisory | |
| D — phantom | "Closes: B01, F-S2104-P29-H01" | advisory for B01 + Continue | D (DEFERRED v1.6) | only "F-..." → Continue | |
| E1 — match | version "1.6"; last_amended "(v1.6)" | Continue | E | "(v1.5)" → block | Prior chain "(v1.5)" deeper → Continue |
| E2 — out-of-order | modified: ["2026-05-14","2026-05-18","2026-05-15"] | block | E | Ascending → Continue | |
| E2 — equal dates | modified: ["2026-07-30","2026-07-30 (v1.1)"] — same date after suffix-strip | Continue (equal dates satisfy ≤) | E | strict-ascending impl rejects equal → block (wrong) | strictly ascending dates → Continue |
| E2 — genuine decrease | modified: ["2026-07-30","2026-07-29 (v1.1)"] — date decreases after suffix-strip | block: E2 (decrease) | E | non-decreasing → Continue | |
| Combined A+E | INDEX stale + E1 mismatch | single combined block | A+E | each alone → block | both fixed → Continue |
| B volatile-input | Story with `inputs: [".factory/cycles/v1.0-brownfield-backfill/lessons.md"]`; input-hash present | advisory (volatile; B suspended) + Continue | B Arm1 | `inputs:` corrected (no volatile) + B2==B3 but B1≠B2 → advisory (PC13a; story just rewritten, STORY-INDEX update pending); `inputs:` corrected + B2≠B3 → block (PC13b; STORY-INDEX internally inconsistent) | `inputs:` corrected + all three sites equal → Continue |
| D zero-scope | `burst-log.md` last H2 has no `**Closes`/`**Refs` lines | scope-count advisory N=0 + Continue | D (DEFERRED v1.6) | Scope has 1 Closes line → N=1 scope-count advisory emitted | |

## Gate Specifications

### Registry entry

```toml
[[hooks]]
name = "validate-cross-site-correspondence"
plugin = "hook-plugins/validate-cross-site-correspondence.wasm"
event = "PostToolUse"
tool = "^(Edit|Write|MultiEdit)$"
tier = "sync"
on_error = "continue"
path_allow = [
  ".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
  # ".factory/cycles/" — DEFERRED v1.6: Class D (cycle artifact arm) deferred pending
  # Closes/Refs convention standardization. Re-add when S-21.08 implements Class D.
]
timeout_ms = 8000
```

`on_error = "continue"`: fuel exhaustion or plugin crash is non-blocking. Fuel exhaustion is the
primary risk for large artifacts: BC-INDEX.md and `lessons.md` approaching 3,000+ lines may exhaust
the WASM sandbox fuel budget before validation logic runs, causing a silent skip that becomes
observable once the host-level fuel advisory log is shipped (ADR-035 §Decision 5). The `max_bytes`
caps in PC4 (1 MiB), PC10/12/15/35 (512 KiB), and PC19/23/29 (2 MiB) are calibrated to bound
reads inside the fuel budget at current artifact sizes. Fuel exhaustion is silenced at the registry
level via `on_error = "continue"` — there is no WASM-side handling for it in this hook.

**`fuel_cap` field**: ADR-035 §Decision 5 introduced a per-plugin `fuel_cap` registry field. This
hook does NOT require a non-default cap — the `max_bytes` limits already bound the read-dominated
computation per invocation. A future implementer MUST NOT add a `fuel_cap` entry without first
confirming that `max_bytes` caps are genuinely insufficient; adding a cap without that evidence is
premature optimization against a non-observed exhaustion scenario at current artifact scales.

### Internal dispatch logic

```
fn run(payload):
    file_path = payload.tool_input.file_path
    violations = []
    advisories = []

    if is_bc_file(file_path):           // Part A Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_file(file_path):      // Part A Arm2 + Part B Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm2(file_path, content)
        violations += run_part_b_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_index(file_path):     // Part B Arm2
        content = read_primary(file_path, 2097152, 5000)
        violations += run_part_b_arm2(file_path, content)
    // DEFERRED v1.6 — Class D descoped; is_cycle_artifact arm inactive until S-21.08
    // elif is_cycle_artifact(file_path):  // Part D
    //     content = read_primary(file_path, 2097152, 5000)
    //     advisories += run_part_d(file_path, content)

    for adv in advisories: host::log_warn(adv)
    if violations: return combined_block(violations)
    return HookResult::Continue
```

### Part A Arm1: `run_part_a_arm1(file_path, content)`

```
bc_id = basename(file_path).trim_suffix(".md")
fm_version = extract_frontmatter_field(content, "version") |> strip_v_prefix
index_content = host::read_file(BC_INDEX_PATH, 1048576, 3000)
  // NotFound → log_warn advisory + return []
  // CapabilityDenied → return [block(cap_denied)]
match extract_bc_index_version_state(index_content, bc_id):
    // four-state per PC5 v1.11; two-level predicates:
    //   locator predicate: (1) line starts with |; (2) first non-empty field matches ^\[<id>\] or equals <id>
    //   body-table row predicate: locator-matched AND (3) non-empty field count ≥5 after escape-aware split
    //   scan MUST prefer first (1)+(2)+(3) line — full-file scan; RowMalformed only if ALL (1)+(2) lines fail (3)
    //   if no (1)+(2) line exists anywhere → RowAbsent
    RowAbsent:
        // no locator-matched line found at all — genuinely dropped registration
        if fm_version == "1.0": log_warn(new_bc_advisory); return []
        else: return [block(row_absent_v_gt_1_message)]     // postcondition 4 blocking path
    RowPresentNoVersion:
        return []   // 5-column canonical shape; no version cell; silent-continue (postcondition 4)
    RowMalformed:
        // locator-matched line found but ALL such lines have <5 fields — not a body-table row
        log_warn(malformed_candidate_advisory(bc_id, field_count)); return []  // postcondition 4a; never blocks
    Version(index_version):
        // PC2 directional carve-out (v1.11 / F-P6-001 Option 1):
        // compare major.minor as decimal integers after stripping 'v' prefix
        if parse_version(fm_version) > parse_version(index_version):
            // 2a: primary newer than index — POLICY 3 ordering artefact; advisory only
            log_warn(advisory_primary_newer_msg(bc_id, index_version, fm_version)); return []
        elif parse_version(fm_version) < parse_version(index_version):
            // 2b: index newer than primary — anomalous; retain BLOCK
            return [block(anomalous_index_newer_msg(bc_id, index_version, fm_version))]
        // equal: postcondition 1 — Continue
        return []
        // parse_version failure (unparseable token) → treat as anomalous → 2b block
```

### Part A Arm2: `run_part_a_arm2(story_path, story_content)`

```
bc_ids = extract_frontmatter_sequence(story_content, "behavioral_contracts")
if bc_ids.is_empty(): return []
story_id = extract_story_id_prefix(story_path)
violations = []
for bc_id in bc_ids:
    // Deterministic path derivation — no list_dir
    section = bc_id.trim_start_matches("BC-").split('.').next().parse::<u32>()
    bc_path = format!(".factory/specs/behavioral-contracts/ss-{:02}/{}.md", section, bc_id)
    bc_content = match host::read_file(bc_path, 524288, 3000):
        Ok(c) => c
        Err(HostError::NotFound) => { log_warn(not_found_advisory(bc_id)); continue }
        Err(e) => return [block(bc_read_error(bc_id, bc_path, e))]
    bc_version = extract_frontmatter_field(bc_content, "version") |> strip_v_prefix
    citations = extract_story_bc_version_citations(story_content, bc_id)
    // Vec<(location: String, cited_version: String)>
    // scoped to sections matching ^## Behavioral Contracts\b or ^## Token Budget\b (PC13 prefix predicates)
    // bc_id must be an exact word-boundary token (\b on both sides)
    // TWO-PHASE version extraction (PC13 v1.8):
    //   Phase 1: any field matching ^v?([0-9]+\.[0-9]+)$ exactly (pure-version field) → use it
    //   Phase 2 (fallback): rightmost field with \bv([0-9]+\.[0-9]+)\b (mandatory v-prefix)
    // prior optional-v \bv?([0-9]+\.[0-9]+)\b is NON-CONFORMING: story-ID collision (29 rows),
    // BC-section-number collision (Token Budget), ACs-column collision (1 row, S-21.07)
    for (location, cited_ver) in citations:
        if strip_v_prefix(cited_ver) != bc_version:
            violations.push(stale_arm2_msg(story_id, bc_id, location, cited_ver, bc_version))
return violations
```

### Parts B, D, E

`run_part_b_arm2` and `run_part_e` are unchanged from v1.0 specification.

`run_part_b_arm1` changed per v1.5 Amendment 4 (PC40): after extracting `input-hash` (PC18),
call `parse_story_volatile_inputs(story_content)` to extract the `inputs:` sequence, then check
each entry against `is_volatile_path(path)` before reading STORY-INDEX.md. If any volatile path
is found, emit the PC40 prescribed advisory and return `HookResult::Continue` immediately
(skipping PC19-21). See PC40 for the volatile-pattern table and prescribed advisory message. If
`inputs:` is absent or empty, `volatile_found` is empty and B1 proceeds normally. (Corrected
from `check_volatile_inputs` which was never the shipped function name — F-P6-004.)

`run_part_b_arm1` also changed per v1.11 (F-P6-001 PC13 directional carve-out): after the
three-way comparison (PC19-21), when B2 or B3 is present but the values do not all agree, the
hook MUST distinguish two sub-cases per postcondition 13:
- **(13a) B2==B3 AND B1≠B2**: `host::log_warn` advisory + `HookResult::Continue`. STORY-INDEX
  catalog row and blockquote agree with each other; the story frontmatter just differs. This is
  the POLICY 3 burst-ordering artefact — state-manager writes STORY-INDEX AFTER the primary
  story write in every conforming burst. No inconsistency exists in STORY-INDEX itself.
- **(13b) B2≠B3, regardless of B1**: `HookResult::block_with_fix(...)`. STORY-INDEX catalog
  row and blockquote disagree with each other. No burst-ordering argument explains this; it
  indicates a partial write, wrong-story edit, or state-manager bug.
**The prior any-mismatch-blocks behavior** (blocking whenever B1≠B2 OR B1≠B3, regardless of
the B2/B3 relationship) **is NON-CONFORMING** under v1.11. Conforming implementations MUST
check B2≠B3 as the blocking condition. See postconditions 13a and 13b for prescribed advisory
and block messages respectively.

**[DEFERRED v1.6 — `run_part_d` is NOT part of the v1.6 hook implementation]**
`run_part_d` specification: after scope-limited line extraction (PC30), count the lines matched
by the PC31 patterns and emit the scope-count advisory via `host::log_warn` before entering the
namespace-format check loop. (Preserved here per POLICY 1 for the follow-up story S-21.08.)

```
fn run_part_d(file_path, content):
    scoped = extract_scope_limited_region(file_path, content)   // PC30
    closes_refs_lines = extract_closes_refs_lines(scoped)       // PC31 patterns
    // PC31a — always emit before namespace check
    host::log_warn(scope_count_msg(closes_refs_lines.len(), file_path))
    for line in closes_refs_lines:                              // postconditions 16-17
        for token in tokenize(line):
            if is_finding_like(token) && !starts_with_F(token):
                host::log_warn(namespace_advisory(token, line, file_path))
    // postcondition 18 — handled via read_primary (PC29) before this point
    return []   // Class D never returns violations; all advisory-only

// Volatile-check shape (F-P6-004 correction — check_volatile_inputs was never the shipped fn):
// run_arm_b1 calls: is_volatile_path(path: &str) -> bool          [arm_b.rs]
//                   parse_story_volatile_inputs(content: &str) -> Vec<String>  [arm_b.rs]
// Inline logic in run_arm_b1:
//   inputs = parse_story_volatile_inputs(story_content)
//   volatile_found = [p for p in inputs if is_volatile_path(p)]   // PC40 table
//   if volatile_found is non-empty:
//     host::log_warn(volatile_advisory_msg(story_id, volatile_found))
//     return HookResult::Continue   // skip PC19-21
//   // proceed to PC19 normally
```

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | A Arm1 Index-Newer-than-Primary Block (PC2b) | bats integration test |
| (pending) | A Arm1 Primary-Newer-than-Index Advisory (PC2a) | bats integration test |
| (pending) | A Arm1 New-BC Advisory | bats integration test |
| (pending) | A Arm1 Primary-CapabilityDenied Block | bats integration test |
| (pending) | A Arm2 Stale-Citation Block | bats integration test (story Token Budget stale) |
| (pending) | A Arm2 No-Citation Skip (Continue) | bats integration test (no version-citing rows) |
| (pending) | A Arm2 BC-NotFound Advisory (Continue) | bats integration test |
| (pending) | A Arm2 BC-CapabilityDenied Block | bats integration test |
| (pending) | B Arm1 STORY-INDEX-Internally-Inconsistent Block (PC13b) | bats integration test |
| (pending) | B Arm1 STORY-INDEX-Consistent Advisory (PC13a) | bats integration test |
| (pending) | B Arm1 Absent-Sites Advisory | bats integration test |
| (pending) | B Arm2 Internal Mismatch Block | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Namespace-Excluded Token Pass (D-944) | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Phantom-ID Advisory | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Historical-Excluded Pass | bats integration test |
| (pending) | E1 Version-Mismatch Block | bats integration test |
| (pending) | E2 Non-Ascending Block | bats integration test |
| (pending) | E Prior-Chain Exclusion Pass | bats integration test |
| (pending) | Combined A+E Block | bats integration test |

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — cross-site value correspondence enforcement) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes PostToolUse gates enforcing cross-site value correspondence invariants recurring across passes 28-30 of the F5 adversarial cycle: Class A = POLICY 14 leg 5 (two-arm: BC-INDEX Arm1 + story citation Arm2); Class B = POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY-GATE (D-923); Class E = POLICY 14 leg 4 / POLICY 17; Class D = finding-ID namespace advisory. No formal CAP-NNN from domain-spec/capabilities.md covers engine process governance automation at this layer; E-12 is the established sub-capability anchor for this BC family per BC-5.39.008 §Traceability. |
| Architecture Module | `crates/hook-plugins/validate-cross-site-correspondence/` (new WASM crate); `plugins/vsdd-factory/hooks-registry.toml`; `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` |
| D-NNN Sub-Clauses Closed | POLICY 14 leg 5 (BC-INDEX body-table + story citation sync; Class A Arm1 + Arm2); POLICY 18 D-923 (Class B); POLICY 14 leg 4 / POLICY 17 (Class E); Canonical Principle + TD-VSDD-059 (fail-closed) |
| Stories | S-21.07 (implementing story; v1.5 in flight) |
| L2 Invariants | (none — process-automation gate) |

## Related BCs

- BC-5.39.003 — `validate-index-cite-refresh`: Class A Arm1 is the per-BC-write counterpart
- BC-5.39.004 — `validate-burst-log`: burst-log structural completeness
- BC-5.39.007 — `validate-closes-completeness`: Closes block presence; Class D checks format
- BC-5.39.008 — `validate-policies-schema`: establishes fail-closed ruling governing Invariant 4
- BC-6.26.001 — primary motivating artifact for Class A; repeatedly re-staled in passes 28-30

## Architecture Anchors

- `crates/hook-plugins/validate-cross-site-correspondence/`
- `crates/hook-sdk/src/host.rs` — `host::read_file`, `host::log_warn`
- `crates/hook-sdk/src/result.rs` — `HookResult::Continue`, `HookResult::block_with_fix`
- `extract_bc_index_version_state(content: &[u8], bc_id: &str) -> BcIndexVersionState` — four-state return per PC5 v1.11 (corrected from `extract_bc_index_version`, which was deleted by F-P4-016 in the pass-5 burst; spec named the deleted symbol): `RowAbsent` (no locator-matched line at all), `RowPresentNoVersion` (locator-matched; exactly 5 non-empty fields, OR ≥6 non-empty fields but no `\bv([0-9]+\.[0-9]+)\b` in field 6 — F-P6-018 normative addition), `Version(v)` (locator-matched; ≥6 non-empty fields AND field 6 contains a `\bv([0-9]+\.[0-9]+)\b` token), `RowMalformed` (locator-matched but ALL such lines have <5 non-empty fields; NOT a body-table row — advisory + Continue). Two-level predicates: locator=(1)+(2); body-table=(1)+(2)+(3). Scan MUST prefer first valid (≥5-field) locator-matched line; first-match-wins on malformed line is NON-CONFORMING (F-S2107-P4-005). Uses escape-aware splitting (`\|` non-splitting). `RowMalformed` MUST NOT be collapsed into `RowAbsent`.
- `extract_frontmatter_field(content, field)` — extracts a named YAML field from frontmatter. **MUST handle block scalar forms** (`|`, `|-`, `>`, `>-`): when the field line ends with a block scalar indicator, collect the block body from subsequent indented lines (do not return the literal indicator). Returning `"|-"` for a block-scalar field is NON-CONFORMING (PC36 normative requirement; BC-5.39.010 and S-21.07 both use `last_amended: |-`).
- `derive_bc_path(bc_id)` — deterministic BC file path derivation from BC ID (no list_dir)
- `extract_story_bc_version_citations(content, bc_id)` — finds version-citing table rows for a given BC ID within sections matching `^## Behavioral Contracts\b` or `^## Token Budget\b` ONLY (PC13 prefix-with-word-boundary predicates; NOT exact equality); two-phase version extraction: Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`), Phase 2 fallback mandatory-v inline (`\bv([0-9]+\.[0-9]+)\b` rightmost-first); prior optional-v bare form NON-CONFORMING (29-row story-ID collision, Token Budget BC-section-number collision, 1-row ACs-column collision); returns Vec<(location, version)>
- `extract_frontmatter_sequence(content, field)` — parses YAML sequence field from frontmatter
- `is_volatile_path(path: &str) -> bool` — returns `true` if `path` matches any ADR-037 §Decision 2 volatile pattern from the PC40 table; used by `run_arm_b1` inline logic to scan the story's `inputs:` sequence. (Corrected from `check_volatile_inputs` which was never the shipped function name — F-P6-004.)
- `parse_story_volatile_inputs(content: &str) -> Vec<String>` — extracts the `inputs:` YAML sequence from story frontmatter content, returning the list of input paths. Called by `run_arm_b1` before invoking `is_volatile_path` per-path. (Part of the actual volatile-check shape; replaces the non-existent `check_volatile_inputs` wrapper.)
- `is_frontmatter_parity_target(file_path)` — PC34 classifier (BC, VP, story, epic files); VP arm uses flat `verification-properties/` path + `^VP-[0-9]+\.md$` predicate (no `ss-*/`)

## Story Anchor

S-21.07 — `validate-cross-site-correspondence` WASM hook (v1.5 in flight; BC-5.39.010 is the governing behavioral contract per story frontmatter `behavioral_contracts: [BC-5.39.010]`).

## VP Anchors

VP-102 through VP-118 (17 VPs) are planned for this story per D-945 (anchored to S-21.07
post-merge). VP IDs have not yet been formally allocated to VP-INDEX — that occurs at the
state-manager post-merge burst per established practice. Once allocated, this section will be
updated to a full VP table with property names and proof methods. The story carries
`verification_properties: []` reflecting this pending state.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.12 | 2026-08-05 | EC table aligned with v1.11 normative postconditions (F-P6-001 Option 1 propagation): EC-002 corrected (primary-newer → advisory + Continue per PC2a; was Block — incorrect); EC-005 block trigger clarified as B2≠B3 per PC13b (was "B2 ≠ B1" — misleading); EC-019 updated (Class A Arm1 primary-newer is advisory, not a violation; output is advisory + E1 block, not combined-block); EC-034 added (PC13a advisory: B2==B3/B1≠B2 → advisory + Continue); EC-035 added (PC2b block: index-newer-than-primary → Block). Canonical Test Vector "A Arm1 — stale" corrected to advisory + Continue per PC2a; PC2b block mutant added. Gate Spec run_part_b_arm1 prose extended with v1.11 PC13a/13b split (prior prose described only PC40/v1.5 change). VP table updated: "A Arm1 Stale-Index Block" → "A Arm1 Index-Newer-than-Primary Block (PC2b)"; new VP entries for PC2a advisory and PC13a advisory added. Refs: F-P6-001, Option 1. (product-owner.) |
| 1.11 | 2026-08-05 | PC2 directional carve-out (F-P6-001 Option 1): primary-newer-than-index → advisory ("state-manager index update pending; Class A BLOCK suspended"); index-newer-than-primary → retains BLOCK (anomalous). PC13 two sub-cases (F-P6-001): B2==B3/B1≠B2 → advisory (POLICY 3 artefact); B2≠B3 → retains BLOCK (STORY-INDEX internal inconsistency). Both carve-outs mirror PC3/PC12 POLICY 3 ordering rationale. PC4a verbatim assertion strengthened (F-P6-002): test-writer MUST assert COMPLETE formatted string by equality check; .contains()-only is NON-CONFORMING. PC5 ≥6-field/no-v-token state normatively defined as RowPresentNoVersion (F-P6-018); empty-cell counting confirmed canonical. PC36 corpus updated 2→3 (F-P6-012): ADR-037 adopted last_amended: |- in v1.1 amendment; grep -rl confirmed 3 files. PC40 transitional clause corrected (F-P6-007): both conditions satisfied in pass-5 burst; PC40 vacuous for S-21.07; "widening" characterization WITHDRAWN — ARCH-INDEX.md in is_volatile_path is normative per ADR-037 §Decision 2; 77 stories remain. Architecture Anchors (F-P6-004): extract_bc_index_version_state replaces deleted extract_bc_index_version; is_volatile_path + parse_story_volatile_inputs replace non-existent check_volatile_inputs. §Traceability + §Story Anchor: v1.4→v1.5 (F-P6-005). §VP Anchors: D-945 basis documented; erratum added to v1.1 row (F-P6-024). Gate Spec: PC2a/PC2b Version arm split; PC13a/PC13b B1 split; check_volatile_inputs reference corrected. (product-owner; S-21.07 pass-6 fix burst.) |
| 1.10 | 2026-08-05 | PC5 self-contradiction fixed (F-S2107-P4-022): split into two-level locator/body-table predicates; `RowMalformed` redefined as "locator-matched line (conditions (1)+(2)) with <5 fields". PC5 candidate-selection order added (F-S2107-P4-005): full-file scan must prefer first valid (≥5-field) locator-matched line; first-match-wins on malformed is NON-CONFORMING. Postcondition 4a pinned normatively (F-S2107-P4-025): prescribed advisory text is MUST-verbatim; omitting operator-actionable clauses is NON-CONFORMING. Postcondition 13 expanded with three-category enumeration (F-S2107-P4-006 ruling: hook MUST enumerate stale/fabricated/algorithm-divergent as possibilities without classifying; `classify_provenance` heuristic is NON-CONFORMING; AC-009 stops requiring classification; invariant 11 governs). Invariant 11 SHOULD→MUST. Postcondition 22 prescribed message added citing POLICY 14 leg 3 (F-S2107-P4-008 sibling sweep: all other leg citations confirmed correct — postconditions 2 and 7 both already cite leg 5 correctly; postcondition 20 cites leg 4 correctly). PC36 block-scalar normative requirement: `extract_frontmatter_field` MUST handle `\|-` block scalars; returning `"|-"` is NON-CONFORMING (F-S2107-P4-004 coupling declared; corpus: 2 occurrences). PC40 transitional clause updated: "no permanent weakening" guarantee requires exhaustive ADR-037 §Context; S-21.07 absent from that table; architect-routed (F-S2107-P4-013 BC-side ruling). Gate Spec pseudocode updated for two-level predicates and selection order. Architecture Anchors: `extract_bc_index_version` description updated to v1.10 terminology; `extract_frontmatter_field` block-scalar requirement added. Story Anchor and §Traceability Stories updated TBD→S-21.07. (product-owner; pass-5 fix burst.) |
| 1.9 | 2026-08-04 | PC5 fourth state `RowMalformed`: a candidate line matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) was found but has <5 non-empty fields after escape-aware splitting — it is NOT a valid body-table row (likely a Changelog entry, subsystem-section row, or notes table that incidentally carries the BC ID link). `RowMalformed` disposition: advisory + Continue; NEVER reaches postcondition 4 blocking path. Narrows `RowAbsent` to exclusively mean "no candidate line found at all." Normative body-table row recognition predicate: first-cell link/plain match AND field count ≥5 both required for valid body-table classification. Corpus-validated 2026-08-04: 0 RowMalformed lines in real BC-INDEX (all 1,983 BC-ID-matching lines have ≥5 fields); forward-looking protection. Postcondition 4a added. Gate Spec `run_part_a_arm1` match extended to four arms. (product-owner; resolves internal contradiction discovered by implementer during v1.8 implementation.) |
| 1.8 | 2026-08-04 | PC5 column-anchored locator: state classification changed from token-search-based to escape-aware column-count-anchored — split by unescaped `|` (treating `\|` as non-splitting), count non-empty fields: 5 → RowPresentNoVersion unconditionally; ≥6 → Version(v) from 6th column. Token-search approach was NON-CONFORMING: 194 of 1,943 canonical rows carry story IDs (e.g., `S-15.01`) in the Stories column whose digits match bare `\bv?([0-9]+\.[0-9]+)\b` → Version("15.01") false-BLOCK; 194 is the load-bearing corpus count. Corpus-validated: 1983 total / 1943 five-field / 40 six-field / 194 story-ID hazard rows. PC6 updated: Version(v) extraction uses rightmost `\bv([0-9]+\.[0-9]+)\b` match in 6th field. PC13 two-phase algorithm replaces LAST rightmost pipe-field algorithm (prior optional-v form NON-CONFORMING): Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`, 58 BC-section rows); Phase 2 mandatory-v inline (`\bv([0-9]+\.[0-9]+)\b`, 30 Token Budget rows); eliminates: (1) story-ID collision 29 rows/6 stories, (2) BC-section-number collision in Token Budget rows, (3) ACs-column collision 1 row S-21.07/BC-5.39.010. Gate Spec pseudocode and Architecture Anchor updated. PC40 ruling and ALGORITHM-DIVERGENT corrections from v1.7 retained. (product-owner.) |
| 1.7 | 2026-08-04 | PC5 corrected: BC-INDEX canonical shape 5-column (`| BC ID | Title | Status | Capability | Stories |`); version-chain cell is ad-hoc 6th column present on only 40 of 1983 body-table rows (corpus 2026-08-04, adversary pass-3 verified); `extract_bc_index_version` rearchitected to three-state return `RowAbsent` / `RowPresentNoVersion` / `Version(v)` — prior two-state `Option<String>` conflating `RowAbsent` and `RowPresentNoVersion` into a single `None` produced ≥1,712 false BLOCKs per F-S2107-P3-001. Postcondition 4 expanded: `RowAbsent` + version > "1.0" → block (unchanged, genuine structural fault); `RowPresentNoVersion` → silent-continue (5-column canonical shape is standard for ~98% of rows; advisory would be unactionable noise). Part B postconditions note and invariant 11 corrected: `1acf3c6` reclassified from fabricated to ALGORITHM-DIVERGENT per ADR-036 §Decision 4 — produced by rc.23 CACHE binary (trailing-newline-stripping algorithm), not fabricated; no PROVENANCE-BREAK annotation was warranted; Pass-30 M02 POLICY 18 violation claim for `1acf3c6` retracted. Invariant 11 updated to three-category taxonomy. Gate Spec `run_part_a_arm1` pseudocode updated to reflect three-state match. PC40 confirmed conformant — F-S2107-P3-002 is implementation non-conformance, not a spec defect; no PC40 amendment warranted. (product-owner; F-S2107-P3-001 spec-side; ADR-036 §Decision 4 annotation corrections per D-952.) |
| 1.6 | 2026-08-04 | Class D (finding-ID namespace advisory in Closes/Refs lines) descoped entirely per human-approved scope decision 2026-08-04. Active gated classes: A, B, E only. `is_cycle_artifact` dispatch branch marked DEFERRED — must not be compiled into v1.6 hook. `.factory/cycles/` removed from registry `path_allow`. Rationale: Closes/Refs convention is unstandardized (six shapes across both cycle burst-logs; PC31 failed three iterations: v1.2 plain-colon→0 matches, v1.3 bold-bare-colon→20/34, v1.5 bold-word-boundary-colon→86/96 bold but 0/20 non-bold); v1.5 measurement taken against wrong cycle (`v1.0-feature-engine-discipline-pass-1` while active cycle is `v1.0-brownfield-backfill`). PC28-PC33 and postconditions 16-18/24 and invariant 6 marked DEFERRED; IDs preserved per POLICY 1 append-only. Knowledge carried forward in §Deferred Scope with follow-up story target S-21.08 (E-21 epic). Class A, B, E amendments from v1.4-v1.5 (PC34 VP-path correction, PC40 volatile-input precondition, invariant-6 adjudication) survive intact. (product-owner; human-approved scope decision.) |
| 1.5 | 2026-08-04 | Amendment 1 (PC31): Closes/Refs regex corrected — bare-colon `^\*\*Closes:\*\*` matched only 20 of 34 burst-log Closes lines (corpus 2026-08-04; 14 missed due to parenthetical/bare-word qualifiers); new form `^\*\*Closes\b[^:]*:\*\*` covers all 34; Refs=0 corpus instances, forward-looking; PC31a scope-count advisory added (postcondition 24) so inert scope is observable. Amendment 2 (PC34): VP path corrected from `ss-*/VP-*.md` (inert for all 102 VPs) to flat `^VP-[0-9]+\.md$` with VP-INDEX.md exclusion (corpus 2026-08-04: zero ss-* dirs, 102 VPs flat); epics clause added (dispatch.rs carried it without PC34 counterpart; closes spec gap). Amendment 3 (invariant 6 adjudication): CapabilityDenied/Timeout on cycle artifact remain BLOCKING per PC33/postcondition 18/invariant 5 — invariant 6 scopes to finding-content verdicts only; postcondition 18 expanded to include Timeout; non-conforming swallow-all error arm named explicitly. Amendment 4 (PC40): volatile-input precondition for Class B Arm B1 per ADR-037 §Decision 4 — scan story inputs: for volatile patterns (decision-log/lessons/burst-log/STATE.md/catalog indexes); emit prescribed advisory + Continue if found; transitional clause (vacuous once 19-story remediation complete, per ADR-037 §Decision 5); EC-032 + test vector added. (product-owner; S-21.07 LOCAL adversary pass-2 fix.) |
| 1.4 | 2026-08-03 | PC13 amended: bounding-section heading-match predicates changed from exact equality to prefix-with-word-boundary (`^## Behavioral Contracts\b`, `^## Token Budget\b`). v1.3 named exact heading text which the implementer rendered as `heading == "Behavioral Contracts"` and `heading == "Token Budget"` — causing 133 of 144 production stories (those using `## Token Budget Estimate` or `## Token Budget Estimate (MANDATORY)`) to be skipped, making stale Token Budget citations invisible. Corpus check (2026-08-03) against `.factory/stories/*.md` confirmed all measured variants covered and zero false positives: no other `^## ` heading in the corpus matches either predicate; `## Edge Cases` (148 occurrences) begins with `Edge` and remains excluded, preserving the ≥9-spurious-block regression fix from v1.3. Architecture Anchor for `extract_story_bc_version_citations` updated. Pseudocode scoping comment updated. Explicit non-conformance note added to PC13 body. (product-owner; S-21.07 LOCAL adversary cascade pass-1b fix.) |
| 1.3 | 2026-07-30 | PC13 amended: bounding section added — scan confined to `## Behavioral Contracts` and `## Token Budget` sections only; unbounded scan caused ≥9 spurious blocking violations on stories that document BC edge cases in a table (Edge Cases rows carry `BC-5.39.010 EC-0NN` + prose `v1.x` tokens). Dual version-token format: `\bv?([0-9]+\.[0-9]+)\b` matches both bare `1.2` (body BC-table) and v-prefixed `v1.2` (Token Budget rows); prior regex `\bv([0-9]+\.[0-9]+)\b` was unreachable for bare form. LAST rightmost pipe-field algorithm stated explicitly (was already mandated but not algorithmic). Amendment 2 (PC31): bold-markdown form `**Closes:**`/`**Refs:**` required to match D-444(c) real burst-log format; prior plain-colon `^Closes:\s*` matched zero real burst-log lines; union scan (not `else if`) required so compound lines carrying both markers (e.g., `**Closes:** F-X ... **Refs:** B01`) must scan both segments. Amendment 3 (PC38 + postcondition 21): non-decreasing relation stated explicitly as `∀i: date[i] ≤ date[i+1]`; equal same-day dates PERMITTED (not a violation); prior "ascending" wording admitted strict-comparison re-implementation that would self-violate on this BC's own `modified[]`; EC-030/031 added; test vectors for equal-dates and genuine-decrease added. Amendment 4: no spec change — PC29 (`max_bytes = 2097152`) and PC33 (NotFound → advisory+Continue on cycle artifact) already unambiguous; implementation used wrong 1 MiB constant and wrong NotFound handling, both purely implementational faults. Amendment 5 (PC32): `O-` observation IDs deliberately non-excluded per D-449(d)(i) which scopes Closes/Refs to findings, PG, D-NNN only; ruling made explicit to prevent ambiguity. POLICY 14 five-leg parity; v1.1 `modified[]` entry restored (was missing — irony: this hook verifies `modified[]` monotonicity but not `modified[]`↔Changelog row correspondence, so it structurally cannot catch this defect in its own governing BC). (product-owner; S-21.07 LOCAL adversary pass-1 fix-burst.) |
| 1.2 | 2026-07-30 | Registry entry corrected: `tools = [...]` array replaced with `tool = "^(Edit\|Write\|MultiEdit)$"` regex string (field name singular + MultiEdit added; all 41 Edit/Write hooks in live registry guard this pattern; omitting MultiEdit was a POLICY 13 ESCAPE-SCOPE-PARITY gap identical in class to F-S2104-P29-H02). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5: `on_error = "continue"` silences the hook non-blockingly on fuel exhaustion (not WASM-side logic); `max_bytes` caps bound reads inside the fuel budget; `fuel_cap` not required. BC-version-pin datum-copy ruling added as design note in §Postconditions Part A Arm2. (product-owner.) |
| 1.1 | 2026-07-30 | Part A Arm2 (story-file-side trigger) added: PostToolUse on story → read each `behavioral_contracts:` BC via deterministic path derivation → compare against story version citations (Token Budget + BC-table rows). No list_dir required. Latency gap correctly sized (all 6 observed failures occurred during story-editing bursts). Advisory rationales made explicit for every advisory arm; confirmed no arm is advisory merely because the check is partial. Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected (D-944 matches shape BUT is excluded by namespace list). Invariant 11 added: stale vs fabricated hash provenance — stale = sweep fix, fabricated = POLICY 18 acknowledgment required (pass-30 M02 precedent). EC-026/027/028/029 added. Gate Spec updated with run_part_a_arm2 pseudocode and story-file dispatch branch. *[Erratum v1.11 (F-P6-024): "VP table extended to 17 entries" was a planning annotation — VP-102..VP-118 (17 VPs) were anchored to S-21.07 post-merge per D-945, but no VP table was written into the BC body at v1.1; §VP Anchors placeholder correctly reflects pending state-manager allocation.]* |
| 1.0 | 2026-07-30 | Initial authoring (product-owner; pre-pass-30 fix-burst). Classes A Arm1/B/D/E gated; Class C honest-gap + Rust test recommendation. |
