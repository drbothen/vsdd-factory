---
document_type: behavioral-contract
level: L3
version: "1.9"
status: active
producer: product-owner
timestamp: 2026-05-28T00:00:00Z
phase: section-12-step-3M4 (un-sealed v1.9 per ADR-023 Option (c))
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
  - .factory/specs/architecture/decisions/ADR-023-trajectory-tail-cell-completeness-cycle-conditional-site-model.md
  - .factory/code-delivery/S-15.17/adv-local-pass-5.md
  - .factory/policies.yaml
  - plugins/vsdd-factory/hooks-registry.toml
input-hash: "0ef3170"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
extracted_from: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: active
sealed: false
unsealed_from: "v1.8 (SEALED at D-522 2026-05-29 per D-386 Option C asymptotic acceptance)"
unsealed_at: "v1.9 2026-05-30 — HUMAN-AUTHORIZED un-SEAL to apply ADR-023 Option (c) cycle-conditional STATE.md Block-arm site model (closes S-15.17 adv-local-pass-5 brick-risk root-cause-2)"
introduced: v1.0-brownfield-backfill
modified:
  - "2026-05-28"
  - "2026-05-28 (v1.1)"
  - "2026-05-28 (v1.2)"
  - "2026-05-28 (v1.3)"
  - "2026-05-28 (v1.4)"
  - "2026-05-28 (v1.5)"
  - "2026-05-29 (v1.6)"
  - "2026-05-29 (v1.7)"
  - "2026-05-29 (v1.8)"
  - "2026-05-30 (v1.9)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.009
section: "5.39"
last_amended: "2026-05-30 (v1.9) — HUMAN-AUTHORIZED un-SEAL + ADR-023 Option (c) cycle-conditional STATE.md Block-arm site model (product-owner). HUMAN AUTHORIZATION: human explicitly authorized un-SEALing BC-5.39.009 (SEALED at D-522 v1.8 per D-386 Option C asymptotic acceptance) to apply ADR-023 (`ADR-023-trajectory-tail-cell-completeness-cycle-conditional-site-model.md`, decision: Option (c)). ROOT CAUSE: S-15.17 LOCAL adversary pass-5 (`code-delivery/S-15.17/adv-local-pass-5.md`) was CLEAN/3-CLEAN against the v1.8 BC, but the architect surfaced a CRITICAL spec-vs-reality conflict — the v1.8 site model Blocks on ALL 5 STATE.md sites unconditionally, which would brick every state-manager write in milestone/story-delivery cycles (live `current_cycle: v1.0-brownfield-backfill`, whose Phase Progress / Concurrent Cycles / Session Resume §1 rows are milestone-form and structurally carry no per-pass trajectory tail — this is normal, not degradation). RE-SPEC per ADR-023 Option (c): (1) PC1 (current_step) + PC2 (Last Updated cell) remain ALWAYS-Block — cycle-invariant sites present on every cycle's STATE.md by state-manager Commit-E discipline; severity UNCHANGED. (2) PC3 (Phase Progress latest row) + PC4 (Concurrent Cycles latest active row) + PC5 (Session Resume §1 body) are now CYCLE-CONDITIONAL: they Block ONLY when the active cycle is an F5-style per-pass cycle (detected via the `per_pass_trajectory: true` frontmatter flag on the active cycle's INDEX.md, resolved at runtime: STATE.md `current_cycle:` → that cycle's INDEX.md frontmatter → check `per_pass_trajectory`); when the flag is absent/false (milestone cycles), PC3/PC4/PC5 are ADVISORY (`host::log_warn` + Continue), NOT Block. FAIL-OPEN-TO-ADVISORY: if the flag cannot be read (any HostError / utf8 decode failure / absent INDEX.md), default PC3/PC4/PC5 to ADVISORY (never Block) — fail-open-to-advisory, NEVER fail-open-to-Block (a Block fail-open IS the brick). (3) NEW Precondition 7 added: per_pass_trajectory cycle-type gate (resolution path + fail-open-to-advisory default + reuses the existing `extract_current_cycle()` / `host::read_file` machinery from the INDEX.md arm, plus new `extract_per_pass_trajectory_flag()` extractor). (4) inv-4 LENGTH=4-STRICT marker-prefix scoping PRESERVED VERBATIM — NOT relaxed to >=4. (5) NEW EC-021 (milestone-cycle tail-less PC3/PC4/PC5 → NO Block, advisory only — the exact live-STATE.md case that would have bricked) + NEW EC-022 (LENGTH=4 marker coexisting with multi-segment LENGTH=5 prose trajectory in same row → PASS; locks inv-4 marker-prefix first-semicolon-segment scoping against the LENGTH=5 prose form). (6) §D-453(d) mapping table Scope column updated: sites 3/4/5 → 'IN SCOPE (cycle-conditional Block: F5-per-pass → Block; milestone → Advisory)'. (7) PC3/PC4/PC5 'Production STATE.md evidence' prose updated to reflect reality: these rows carry per-row tails ONLY in F5-style per-pass cycles; in milestone cycles they are tail-less by convention (NOT degradation — Option (a) paper-compliance retro-fabrication REJECTED). (8) §Architecture Anchors updated with `extract_per_pass_trajectory_flag()` spec + the INDEX.md secondary-read for cycle-type detection. (9) §SDK Grounding Evidence Grep 11 added (HEAD-reproducible structural form: per_pass_trajectory flag currently ABSENT on all cycle INDEX.md — state-manager authoring obligation per ADR-023 §3; `current_cycle: v1.0-brownfield-backfill` is milestone-form). inv-1/2/3/5/6/7/8/9/10/11/12/13 + PC1/PC2/PC6/PC7-INDEX/PC8/PC9/PC10/PC11/PC12 marker semantics UNCHANGED except where cycle-conditional gating threads through. POLICY 14 5-leg quintuple parity applied (version '1.9' + this last_amended text-prefix v1.9 + ## Changelog row v1.9 + modified[] appended '2026-05-30 (v1.9)' + BC-INDEX bump — BC-INDEX cell bump is state-manager's leg, noted in Changelog). POLICY 5 SDK-grounding: Grep 11 HEAD-reproducible structural form. ADVERSARY NOTE: material behavioral re-spec of a SEALED contract — prior pass-5 CLEAN/3-CLEAN is VOIDED for the new cycle-conditional behavior; a fresh 3-CLEAN LOCAL cascade is required per ADR-023 §5 + BC-5.39.001, OR explicit asymptotic-acceptance per D-386 Option C. STORY-WRITER NEXT BURST: S-15.17 story body/AC propagation for the cycle-conditional model (PC3/PC4/PC5 conditional + new PC7 + EC-021/EC-022) per POLICY 8 — PO does NOT touch story body. STATE-MANAGER NEXT BURST: BC-INDEX title/version cell bump (v1.8→v1.9) + author `per_pass_trajectory: true` on F5-cycle INDEX.md (`v1.0-feature-engine-discipline-pass-1/INDEX.md`) and ensure milestone cycles omit or set false. IMPLEMENTER/TEST-WRITER: split STATE.md arm into always-Block (PC1/PC2) and cycle-conditional (PC3/PC4/PC5) paths; real-STATE.md milestone-cycle regression fixture (NO Block) MANDATORY. [Prior: 2026-05-29 (v1.8) — Pass-8 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-8 fix-burst). Closes 6 BC-side PO findings + 1 process-gap META-36 codification. F-SP8-001 CRITICAL (§Cure-Extension Parsimony Note point 2 PAPER-FIX — rewritten to document F-SP5-001 HUMAN-DIRECTED PARTIAL REVERSAL; 'deliberate non-extension' narrative replaced with full two-step marker-prefix discipline extension rationale per BC-5.39.006 v1.7 inv-6(b) precedent). F-SP8-002 HIGH (PC10 body PAPER-FIX — rewritten to OUT-OF-SCOPE annotation per F-SP5-003; §D-453(d) Site 9 row updated from 'IN SCOPE' to 'OUT-OF-SCOPE (per F-SP5-003; lessons.md no machine-extractable trend-table structure)'). F-SP8-006 HIGH [META-36] (Grep 10 snapshot-annotation-rescue-pattern — rewritten to STRUCTURAL-FORM-only per POLICY 5 v1.3.6 Part B + Part D self-apply; snapshot stdout removed; structural grep predicates HEAD-reproducible at any cycle SHA; non-zero results confirmed at parent-commit dfcbea39: grep -c 'trajectory-tail' STATE.md → 16; grep ... | grep -c '→[0-9]\\+→[0-9]\\+→[0-9]\\+→[0-9]\\+' → 5). F-SP8-007 MEDIUM (§Architecture Anchors catalog 'All 4 body extractors' → 'All 7 section extractors'; extract_frontmatter_current_step + extract_burst_log_latest_dim7 Option<String> spec bullets added). F-SP8-009 MEDIUM (covered by F-SP8-001 point 2 rewrite). F-SP8-PG-001 PROCESS-GAP HIGH META-LEVEL-36 CODIFICATION: policies.yaml POLICY 5 v1.3.5→v1.3.6 — Part B HEAD-reproducibility-or-structural-form mandate (snapshot-annotation-only cures FORBIDDEN) + Part D snapshot-rescue-pattern detection (adversary fresh-context loop executes at HEAD; snapshot-annotated stdout is non-reproducible in cycle loop; META-36 closes snapshot-annotation-rescue-pattern). Cure-of-cure-of-cure-OF-cure-OF-cure recursion at level 6 in POLICY 5 evolution. Extension per D-497 parsimony (single-policy-extension, no new POLICY domain). POLICY 5 v1.3.6 SELF-APPLIED this burst: Grep 10 rewritten with structural-form predicates + literal stdout captured (16 marker occurrences; 5 with 4-arrow-segment pattern). §Adversary Pass Coverage Pass-8 entry added. POLICY 14 5-leg quintuple parity applied (version v1.8 + Changelog row v1.8 + modified[] appended '2026-05-29 (v1.8)' + this last_amended text-prefix v1.8 + BC-INDEX v2.61→v2.62). Story-writer next burst: F-SP8-003/F-SP8-004/F-SP8-005/F-SP8-008/F-SP8-010. [Prior: 2026-05-29 (v1.7) — Pass-7 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-7 fix-burst). Closes 6 BC-side PO findings + 1 process-gap META-35 codification. F-SP7-002 HIGH (Grep 1 arithmetic error — 'all 4 variants' corrected to 'all 5 variants'; CapabilityDenied/Timeout/OutputTooLarge/InvalidArgument/Other(i32) = 5 variants). F-SP7-004 MEDIUM (Grep 10 snapshot non-reproducible at replay — annotated with 'Snapshot: captured at D-518 parent-commit f189b45b 2026-05-29; trajectory tail values evolve monotonically; re-execute against latest STATE.md to verify inv-4 marker-prefix invariant; scoped-segment count==4 property is structural'). F-SP7-005 MEDIUM (§Architecture Anchors extractor return type inconsistency — all 4 body extractors normalized to Option<String>: extract_last_updated_cell, extract_phase_progress_latest_row, extract_concurrent_cycles_latest_row, extract_session_resume_section_1; rationale note added: owned String for lifetime simplicity + extractors perform allocation via trim + consistency with sibling hook extractors; individual extractor spec bullets added for extract_last_updated_cell, extract_phase_progress_latest_row, extract_concurrent_cycles_latest_row, extract_session_resume_section_1). F-SP7-006 MEDIUM (PC2 + PC5 bodies missing extractor function name citations — PC2 updated to cite 'extract_last_updated_cell()' matching PC3/PC4/PC9 style; PC5 updated to cite 'extract_session_resume_section_1()' matching same style). F-SP7-008 LOW (§Adversary Pass Coverage Pass-6 count ambiguity — '11 findings 0C+5H+4M+1L+1N+1PG' summed to 12; reformatted to '11 findings 0C+5H+4M+1L+1N + 1 process-gap'). F-SP7-PG-001 PROCESS-GAP HIGH META-LEVEL-35 CODIFICATION: policies.yaml POLICY 5 v1.3.4→v1.3.5 — (Part A) historical-by-construction enumeration (EXCLUSIVELY: YAML frontmatter modified[] entries + ## Changelog table rows + [Prior:] nested clauses in last_amended + § Adversary Pass Coverage entries + cycle lessons.md [codified]/[process-gap] cross-references; ALL OTHER narrative referencing prior versions MUST cite current version); (Part B) adversary-replay-reproducibility mandate (fix-burst MUST cite parent-commit-SHA at stdout capture; adversary fresh-context replay against SAME SHA must yield IDENTICAL stdout; non-empty stdout at any other SHA = META-35 violation); (Part C) sibling-sweep categories extended from (a)-(e) to (a)-(h): (f) Risk-Mitigation table cells, (g) §Bidirectional Parity Audit Note version-cite cells, (h) §LOCAL Adversary Cascade Plan prerequisite refs + adversary-MUST-verify-notes. POLICY 5 v1.3.5 SELF-APPLIED this burst: verification gates executed (stdout captured; parent-commit cited). POLICY 14 5-leg quintuple parity applied (version v1.7 + Changelog row v1.7 + modified[] appended '2026-05-29 (v1.7)' + this last_amended text-prefix v1.7 + BC-INDEX v2.60→v2.61). Story-writer next burst: F-SP7-001 (story sweep BC v1.5→v1.7 references), F-SP7-003 (story Risk row BC version update), F-SP7-007 (story Token Budget annotation). [Prior: 2026-05-29 (v1.6) — Pass-6 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-6 fix-burst). Closes 7 BC-side PO findings + 1 process-gap META-34 codification. F-SP6-001 HIGH (missing grep evidence blocks — Grep 10 added to §SDK Grounding Evidence with production STATE.md `trajectory-tail` marker occurrences; PC1 body cite on line 235 + inv-4 body cite on line 448 updated from 'Grep-A' to 'Grep 10'; last_amended references to 'Grep NEW-A / NEW-B' updated to 'Grep 10 added'). F-SP6-002 HIGH [META-33 regression] (§Architecture Anchors extractor function names updated to new names: `extract_last_updated_section`→`extract_last_updated_cell`, `extract_concurrent_cycles_section`→`extract_concurrent_cycles_latest_row`; `extract_burst_log_latest_dim7` and `extract_current_cycle` added — all per pass-5 redesign). F-SP6-004 HIGH (PC2 NOTE describing stale D-517 production state updated to reflect D-518+ state: Last Updated cell NOW contains `trajectory-tail →9→9→9→11` marker per Grep 10 evidence; state-manager Commit E dispatch templates updated at D-518). F-SP6-005 HIGH (§Adversary Pass Coverage missing Pass-5 + Pass-6 entries: both added with full finding counts, META-LEVEL codification notices, and STREAK status). F-SP6-006 MEDIUM (Grep 1 narrative line-number anchors removed: 'Confirmed: enum closes at line 94... Closing brace on line 94' replaced with stable-anchor narrative citing variant names). F-SP6-007 MEDIUM (§SDK Grounding Evidence header 'this BC v1.4' updated to 'this BC'). F-SP6-010 LOW (PC1 prose 'two `trajectory-tail` marker mentions' replaced with precise 'multiple `trajectory-tail` marker mentions; the first-occurrence semicolon-segment scoping yields count=4 → PASS per inv-4 (see Grep 10)'). F-SP6-PG-001 PROCESS-GAP META-34 CODIFICATION: policies.yaml POLICY 5 v1.3.3→v1.3.4 SIBLING-SWEEP LITERAL-SHELL VERIFICATION GATE added (sweep claims without literal-shell stdout are MEDIUM-severity; closes META-LEVEL-34 sweep-claim-without-execution; cure-extension per D-497 parsimony extending POLICY 5 v1.3.3). POLICY 5 v1.3.4 SELF-APPLIED this burst: literal-shell verification gates executed (stdout captured). POLICY 14 5-leg quintuple parity applied (version v1.6 + Changelog row v1.6 + modified[] appended '2026-05-29 (v1.6)' + this last_amended text-prefix v1.6 + BC-INDEX v2.59→v2.60). [Prior: 2026-05-28 (v1.5) — Pass-5 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-5 fix-burst; SPEC-REDESIGN AUTHORITY granted by orchestrator). CRITICAL F-SP5-001 closed via HUMAN-DIRECTED PARTIAL REVERSAL of §Cure-Extension Parsimony Note point 2: inv-4 re-specced with marker-prefix discipline extended to all 5 STATE.md sites (PC1/PC2/PC4/PC5/PC6 extractor scoped to `trajectory-tail ` marker sub-region per BC-5.39.006 v1.7 inv-6(b) semicolon-segment-scoping). F-SP5-001 CRITICAL (inv-4 re-spec: extractor MUST first locate the `trajectory-tail ` marker in extracted text; count `→(\\d+)` matches ONLY within the segment from marker-end to first `;` (or end of text); if marker absent → MissingStateSite; inv-4 body rewritten; PC1/PC2/PC4/PC5 updated with marker-prefix semantics; §Cure-Extension Parsimony Note point 2 updated from deliberate-non-extension to PARTIAL-REVERSAL with rationale; §SDK Grounding Evidence Grep NEW-A added with production STATE.md trajectory-tail marker occurrences; sibling-sweep of ALL PC bodies, inv cross-refs, test vectors, EC rows). F-SP5-002 HIGH (PC4 Concurrent Cycles extractor underspecified — tightened to match PC3 pattern: `extract_concurrent_cycles_latest_row()` returns SINGLE ROW TEXT of the BOTTOMMOST active/in-progress row (skipping CLOSED/COMPACTED/ARCHIVED); ONE-tail-per-extracted-region precondition applied; marker-prefix count check applied to single-row text; §Architecture Anchors updated with extract_concurrent_cycles_latest_row spec parallel to extract_phase_progress_latest_row). F-SP5-003 HIGH (PC10 lessons.md trend-table extractor unimplementable — DECISION: PC10 marked OUT-OF-SCOPE; rationale: lessons.md trends are inline prose not a machine-extractable table; structural anchor does not exist; mirrored to D-453(d) mapping table site-9 OUT-OF-SCOPE annotation in §Description; lessons.md arm advisory check for site-9 now treats absent-marker as advisory-only pass-through (same fail-open as EC-011); §Cure-Extension Parsimony Note updated with point 4 documenting this restriction). F-SP5-005 HIGH (extract_current_cycle() multi-line block-scalar handling hand-waved — §Architecture Anchors `extract_current_cycle` spec extended: multi-line `|` literal scalar: consume continuation lines while indent > frontmatter-region-base-indent, join with `\\n`; multi-line `>` folded scalar: same continuation detection, join with single space; empty value `current_cycle: \"\"` treated as `None`; test vector added for multi-line form; §SDK Grounding Evidence Grep NEW-B added showing production `current_cycle:` whitespace boundary). F-SP5-006 MEDIUM (encoding gate implicit — inv-13 ADDED: all section extractors take `content: &str` as input; `host::read_file` bytes decoded via `String::from_utf8(bytes)` upstream of all extractors; failure routes via EC-020 fail-open; inv-11 is-char-boundary applies to byte-index ops on decoded `&str`; sibling-sweep: all PC bodies updated to note they operate on `&str` post-decode). F-SP5-007 MEDIUM (PC9 Dim-7 extractor undefined — PC9 updated: 'latest Dim-7 block' is the BOTTOMMOST `^### Dim-7` heading in file-order; block = text from that heading up to (but not including) next `^##` or `^### ` heading; marker-prefix count applied to that block; `extract_burst_log_latest_dim7()` added to §Architecture Anchors). F-SP5-008 MEDIUM (same root cause as F-SP5-001; cured by marker-prefix re-spec applied to PC2 — PC2 Last Updated now requires `trajectory-tail` marker in cell text; if marker absent → MissingStateSite Block; NOTE added to §Architecture Anchors: PC2 requires the `trajectory-tail` marker form in Last Updated cell; state-manager Commit E dispatch templates must include it; marker may follow other prose in the cell). POLICY 5/8 sibling-sweep extension: META-LEVEL-33 CANDIDATE codified as POLICY 5 sub-clause (sibling-sweep-inside-policy-cure: when a stable-anchor or evidence-form policy is applied to a primary cure site, ALL sibling sites with the same evidence pattern MUST be swept same-burst; per D-497 parsimony, extends POLICY 5 not new POLICY 16). policies.yaml v1.3.2→v1.3.3. POLICY 14 5-leg quintuple parity applied (version v1.5 + Changelog row v1.5 + modified[] appended '2026-05-28 (v1.5)' + this last_amended text-prefix v1.5 + BC-INDEX v2.58→v2.59). [Prior: 2026-05-28 (v1.4) — Pass-4 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-4 fix-burst). INV-019 cure (a)/(b)/(c) discipline extended. Closes F-S15.17-SP4-001 (CRITICAL: count==4 false-Block on multi-tail extracted region — PC3 tightened to single-row extraction; extract_phase_progress_latest_row returns SINGLE row text, not section; ONE-tail-per-extracted-region precondition added to PC3 and Architecture Anchors); F-S15.17-SP4-002 (HIGH: TD-VSDD-091 anti-volatile-pin violation inside POLICY 5 v1.3 cure — stripped ALL line numbers from §SDK Grounding Evidence greps; PC3 now uses stable anchor `^## Phase Progress` with no line number; Grep 4 updated; POLICY 5 extended with stable-anchor sub-clause per META-LEVEL-32 cure-extension); F-S15.17-SP4-004 (HIGH: PC9 extractor target `Dim-7 (Attestation)` heading absent from burst-log — actual heading form discovered via literal-shell grep: `^### Dim-7`; PC9 updated to match actual heading prefix `^### Dim-7`; scoped to brownfield-backfill cycle only with rationale; §SDK Grounding Evidence Grep 4 updated with literal stdout); F-S15.17-SP4-005 (HIGH: extract_current_cycle() function never specified — added extractor spec to §Architecture Anchors with all YAML form variants: bare, single-quoted, double-quoted, trailing comment, multi-line block-scalar; §SDK Grounding Evidence Grep 4 updated); F-S15.17-SP4-006 (HIGH: substring String::contains on cycle-path false-positive — §Architecture Anchors now mandates Path::new(file_path).components().any(|c| c.as_os_str() == active_cycle.as_str()) path-component-walk form; Precondition 4 INDEX.md arm updated); F-S15.17-SP4-007 (HIGH: §Cure-Extension Parsimony Note point 3 collapse direction inverted — rewritten to correct direction: old PC11→new PC11, old PC13→new PC12, net 13→12); F-S15.17-SP4-009 (MEDIUM: EC-020 UTF-8 fail-open mirrored from story into BC — EC-020 added to Edge Cases table; references added in PC11 fail-open umbrella; closes [needs-po] deferral violation of Canonical Principle Rule 3); F-S15.17-SP4-010 (MEDIUM: POLICY 15 violation inside POLICY 5 cure — §SDK Grounding Evidence Grep 1 HostError enum body replaced with literal sed -n '82,94p' stdout; closing-brace verified line 94; narrative paraphrase removed); F-S15.17-SP4-012 (MEDIUM joint w/ story-writer: §Architecture Anchors cycle-name examples replaced with structural form — NOT hardcoded to any specific cycle name; resolved from STATE.md current_cycle: at runtime); F-S15.17-SP4-013 (LOW: PC2 extractor spec extended with whitespace handling: cell value extraction captures text between second and third unescaped `|` characters, strips leading/trailing whitespace, joins continuation lines on whitespace). POLICY 5 v1.3.1 stable-anchor extension: all grep captures MUST use stable anchors only (heading prefix, fn signature); line numbers FORBIDDEN in captured stdout; if line numbers appear, re-execute with stable-anchor pattern. POLICY 14 5-leg quintuple parity applied (version v1.4 + Changelog row v1.4 + modified[] appended '2026-05-28 (v1.4)' + this last_amended text-prefix v1.4 + BC-INDEX v2.57→v2.58). [Prior: 2026-05-28 (v1.3) — Pass-3 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-3 fix-burst). SDK-GROUNDING MANDATE satisfied (9 literal-shell greps in §SDK Grounding Evidence; POLICY 15 verbatim stdout). INV-019 cure (a)/(b)/(c) extended. Closes F-SP3-001 (CRITICAL: cycle-path-guard → dynamic STATE.md current_cycle: read via host::read_file + frontmatter scan; Precondition 4 INDEX.md arm rewritten; PC7/8/9/10 + Architecture Anchors updated; §Cure-Extension Parsimony Note extended; option (a) chosen per mandate recommendation — most robust to cycle transitions), F-SP3-002 (CRITICAL regression: LENGTH=4 STRICT equality semantics adopted from BC-5.39.006 inv-6(b) — extract candidate region → count →(\\d+) matches via separate regex iteration → assert count == 4; NOT count >= 4; inv-4 + PC1..PC5 + EC-003 + EC-018 + Test Vectors updated), F-SP3-003 (HIGH: HostError::TooBig → collapsed into PC12 per adversary recommendation '(any HostError → fail-open per inv-10 makes the distinction structurally redundant)'; PC11 removed; PC12 rewritten to cover ALL HostError variants uniformly; EC-004 + Test Vector + VP updated; rationale in §Cure-Extension Parsimony Note), F-SP3-005 (HIGH: PC6 orphan cross-reference extended — PC6 cascade-Block emission is exercised via AC anchors to invariant 8 per §Behavioral Contracts Table indirection; explicit cross-reference added to PC6 narrative; story-writer handles AC-7 trace extension to PC6 alongside invariant 8), F-SP3-008 (MEDIUM: Precondition 4 + EC-015 + EC-019 parent-guard updated to path-component-walk form: Path::new(file_path).components().any(|c| c.as_os_str() == \".factory\"); platform-independent and robust to ./ prefix + Windows backslash), F-SP3-009 (MEDIUM: dual-cycle attribution documented in §Description — BC anchors F5-cycle D-453(d) codification; delivered via brownfield-backfill story cycle; runtime gate targets active cycle per dynamic current_cycle: resolution post F-SP3-001 fix), F-SP3-011 (LOW: duplicate ADR-018 cite in §D-NNN Anchor Coverage collapsed to single line; parenthetical moved to §ADR References), F-SP3-013 (NITPICK: acceptable as-is per adversary guidance; both D-411(a) cites retained — line 60 description prose + D-NNN table canonical), F-SP3-014 (PROCESS-GAP HIGH: policies.yaml POLICY 8 amended v1.1→v1.2 — audit-block exclusion mandate added; POLICY 8 extension chosen NOT new POLICY 19; rationale: this is a verification-step procedure extension not a new governance policy; POLICY 19 reserved for future scope). POLICY 14 5-leg quintuple parity applied (version v1.3 + Changelog row v1.3 + modified[] appended '2026-05-28 (v1.3)' + this last_amended text-prefix v1.3 + BC-INDEX v2.57 row). PC11 removed (PC12 absorbs); no EC renumbering (EC-004 text updated in-place). Story-writer handles F-SP3-004 (story v1.0→v1.2 body text), F-SP3-006 (T-5 String::from_utf8 double-match), F-SP3-007 (narrative-vs-constraint), F-SP3-010 (EC framing), F-SP3-005 mirror (AC-7 PC6 trace + audit grep amendment), F-SP3-001 mirror (T-5 cycle-path logic), F-SP3-002 mirror (T-5 byte-walk equality count), F-SP3-003 mirror (story EC-004 + AC-14 trace). [Prior: 2026-05-28 (v1.2) — Pass-2 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-2 fix-burst). Closes F-S15.17-SP2-003 (HIGH EC-008 '(PC4)' → '(Precondition 4)'; full EC table swept for Pre-vs-Post abbreviation ambiguity), F-S15.17-SP2-004 (MEDIUM status:active→status:draft to match lifecycle_status:draft; adjudication: both fields must be draft pre-merge, POL-14 fires on S-15.17 merge; sibling BC-5.39.008 is active/active only because it has shipped — BC-5.39.009 is pre-merge so status:draft is correct), F-S15.17-SP2-005 (MEDIUM PC2/PC3/PC5 line-number anti-volatile-pin per TD-VSDD-091: stripped line-number citations from literal-shell grep outputs; kept grep command + prefix/content match excerpts only), F-S15.17-SP2-006 (MEDIUM §Cure-Extension Parsimony Note and inv-4 + Description rephrase: documented deliberate non-extension of BC-5.39.006 marker-prefix semantics — BC-5.39.006 conditions LENGTH check on 'trajectory-tail ' canonical marker prefix; BC-5.39.009 per-cell sites 2-9 are heterogeneous text contexts where that marker convention does not apply), F-S15.17-SP2-007 (MEDIUM Precondition 4 parent-guard for STATE.md arm: hook MUST verify file_path is rooted at .factory/ before triggering STATE.md arm; new EC-019 non-factory STATE.md case added), F-S15.17-SP2-008 (LOW PC3 skip-list: dropped 'COMPLETE' from skip list; skip only ARCHIVED/COMPACTED; alternative bottommost-row rationale documented), F-S15.17-SP2-009 partial (LOW ADR-021 dropped from §ADR References in Traceability — cargo-audit-specific, not general no-subprocess principle; story-writer to drop from anchored_adrs frontmatter), F-S15.17-SP2-010 (LOW inv-9 anti-volatile-pin rephrase: avoid SDK-state assertion; prescribe behavior instead), F-S15.17-SP2-011 (NITPICK D-453 pass-73 cite corrected from 'pass-74'). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-2 / (b) PC2/PC3/PC5 line-number anti-volatile-pin applied / (c) cure-extension parsimony note updated with marker-prefix non-extension documentation. POLICY 14 5-leg quintuple parity applied (version v1.2 + Changelog row v1.2 + modified[] appended '2026-05-28 (v1.2)' + this last_amended text-prefix v1.2 + BC-INDEX v2.56 row). Story-writer (next burst) handles F-001 (AC-9/10/11/12/17 re-anchor + bidirectional parity check stdout), F-002 (SS-05 narrative rewrite), F-003 story EC-008 mirror, F-007 AC-23 false-positive STATE.md, F-009 anchored_adrs drop. [Prior: 2026-05-28 (v1.1) — Pass-1 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade pass-1 fix-burst). Closes F-S15.17-SP1-002 (HIGH frontmatter ADR-017 path corrected to ADR-017-per-story-adversary-phasing.md per POLICY 4 semantic_anchoring_integrity), F-S15.17-SP1-004 (HIGH PC2/PC3/PC5 STATE.md extractor anchors updated to match production STATE.md table-cell + heading structure with literal-shell evidence; META-LEVEL-30 route-(b) partial-recurrence-inside-cure-BC closed), F-S15.17-SP1-005 (HIGH inv-4 + PC1 LENGTH=4-strict adjudication aligning with BC-5.39.006 inv-6(b)+EC-007 + D-433(e)+D-439(c) original codification; cure-extension parsimony per D-497 cited; new EC-018 LENGTH=5 added), F-S15.17-SP1-007 (MEDIUM precondition-5/EC-016 fail-open contradiction reconciled to log_warn+Continue), F-S15.17-SP1-009 (MEDIUM path_allow sibling cite corrected to BC-5.39.006), F-S15.17-SP1-010 (MEDIUM on_error=continue invariant added per sibling BC-5.39.004/005/006/007/008 precedent), F-S15.17-SP1-011 (LOW D-NNN table purified — ADR/META-LEVEL/BC refs moved out), F-S15.17-SP1-012 (LOW D-454(a) PC range clarified to 1-10), F-S15.17-SP1-014 (NITPICK STATE.md capitalization fix). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-1 / (b) structural per-cell extractor-anchor specification / (c) cure-extension of BC-5.39.005+BC-5.39.006 pattern per D-497 parsimony. POLICY 14 5-leg quintuple parity applied (version v1.1 + Changelog row + modified[] + this last_amended + BC-INDEX v2.55 cell). Story-writer (next burst) handles F-001 type, F-003 AC mis-mapping, F-006 EC renumber, F-008 BC table claim, F-013 token budget. [Prior: 2026-05-28 (v1.0) — Initial authoring (product-owner; brownfield-backfill F5 pass-75 HIGH-002 anchor; META-LEVEL-30 route (b) cure). Anchors ADV-EDP1-P75-HIGH-002. BC-5.39.009 allocated as next monotonic ID after BC-5.39.008 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.17 merge).]]"
---

# BC-5.39.009: validate-trajectory-tail-cell-completeness WASM hook MUST block on STATE.md writes missing trajectory_tail in the 2 cycle-invariant cells (current_step + Last Updated; always-Block), MUST apply cycle-conditional Block to the 3 per-pass cells (Phase Progress / Concurrent Cycles / Session Resume §1 — Block only in F5-style per_pass_trajectory cycles, else advisory; fail-open-to-advisory), and MUST emit advisory on INDEX.md / burst-log.md / lessons.md writes missing trajectory_tail in their prescribed cells

## Description

The `validate-trajectory-tail-cell-completeness` WASM hook enforces per-cell presence of the
canonical trajectory_tail arrow-sequence (`(→[0-9]+){4}`, LENGTH=4 per D-433(e)) at PostToolUse
on writes to the four target artifact files that carry D-453(d)-prescribed trajectory_tail sites.
This hook closes META-LEVEL-30 route (b): "codified-canonical-registry-with-per-cell-prescribed-
sites-BUT-no-runtime-WASM-gate-enforcing-each-site." D-453(d) codified a canonical 9-site mapping
table at pass-73, but the 14-day F5 pause demonstrated that codification without a runtime gate
permits silent cell-level degradation between passes (ADV-EDP1-P75-HIGH-002).

The hook has a CYCLE-CONDITIONAL STATE.md severity model (ADR-023 Option (c), v1.9 re-spec)
plus a uniform advisory arm for non-STATE.md sites:

- **STATE.md cycle-invariant sites (1-2: `current_step:` + Last Updated cell): ALWAYS Block.**
  These two cells carry the single canonical trajectory-tail in EVERY cycle by state-manager
  Commit-E discipline (verified present on the live artifact today). Omission causes HIGH adversary
  findings per D-411(a). Severity is unconditional Block.
- **STATE.md per-pass sites (3-5: Phase Progress latest row + Concurrent Cycles latest active row
  + Session Resume §1 body): CYCLE-CONDITIONAL Block.** These three rows carry a per-row
  trajectory_tail ONLY in F5-style per-pass cycles, where the `→N→N→N→N` axis-count arrow
  sequence is a *per-pass convergence metric* appended each adversary pass. In milestone /
  story-delivery cycles (e.g., `v1.0-brownfield-backfill`), these rows are MILESTONE/STATUS rows
  that structurally carry no per-pass trajectory — this is the NORMAL, CORRECT state for such a
  cycle, NOT degradation. Therefore sites 3/4/5 Block ONLY when the active cycle is an F5-style
  per-pass cycle (detected via the `per_pass_trajectory: true` flag on the active cycle's INDEX.md
  frontmatter — see Precondition 7); otherwise they are ADVISORY (`host::log_warn` + Continue).
  **Fail-open-to-advisory:** if the cycle-type flag cannot be resolved (any HostError, utf8 decode
  failure, or absent INDEX.md), sites 3/4/5 default to ADVISORY — NEVER Block. A Block fail-open
  would be the pipeline-brick the v1.8 model risked; fail-open-to-advisory is the production-grade
  guard. Sites 1/2 are UNAFFECTED by the flag and always Block.
- **Non-STATE.md sites (6-9): Advisory severity always.** INDEX.md/burst-log.md/lessons.md cells
  are enforced by the same state-manager burst; their omission is detectable but does not
  independently cause trajectory misreading by agents reading STATE.md.

**v1.9 re-spec rationale (HUMAN-AUTHORIZED un-SEAL; ADR-023 Option (c)):** The v1.8 model
(SEALED at D-522) Blocked unconditionally on all 5 STATE.md sites. S-15.17 LOCAL adversary pass-5
was CLEAN/3-CLEAN against that model, but the architect surfaced (ADR-023) that the v1.8 model
conflates a *legitimately tail-less milestone row* with a *degraded F5 pass row*. Shipping the
v1.8 hook would Block every state-manager STATE.md write in the active milestone cycle
(pipeline-brick). Option (a) — retro-fabricating tails onto milestone rows to satisfy the v1.8
check — was REJECTED as paper-compliance (writes semantically-false data; SOUL.md #4 +
production-grade-default violation). The cycle-conditional model (Option (c)) is correct under
BOTH readings simultaneously: it preserves the full F5 per-row degradation Block where it actually
applies, and eliminates brick risk in milestone cycles, without fabricated data. The human
authorized the SEALED-BC reopen to apply this fix.

The D-453(d) canonical mapping table prescribes 11 sites total. Of these, 9 are mechanically
checkable via PostToolUse on their containing files. Two are excluded from this hook's scope:
`decision-log.md trajectory-bearing rows` (advisory-only per the D-453(d) registry; sparse, not
every row carries a tail) and `adv-cycle-pass-*.md frontmatter trajectory_tail field` (written by
the adversary agent, not by state-manager; out-of-scope for a state-manager PostToolUse gate).

**Dual-cycle attribution (F-SP3-009):** This BC anchors F5-cycle codification (D-453(d) 9-site
mapping, established in `v1.0-feature-engine-discipline-pass-1/decision-log.md`) but is delivered
via the brownfield-backfill story cycle (S-15.17, `cycle: brownfield-backfill`). The runtime gate
applies to whatever cycle is active per `current_cycle:` in `.factory/STATE.md` — resolved
dynamically at hook execution time (per F-SP3-001 dynamic resolution fix). This dual-cycle
attribution is unusual for the BC-5.39.005..009 family (siblings are all single-cycle) but is
technically correct: the D-453(d) codification predates the brownfield-backfill cycle, and the
brownfield-backfill cycle delivers the runtime enforcement gate post-F5-pause.

### D-453(d) 11-Site Canonical Mapping (with hook scope)

| Site # | Document | Section/Field | Hook Scope | Severity |
|--------|----------|---------------|------------|----------|
| 1 | STATE.md | frontmatter `current_step:` | IN SCOPE | Block (ALWAYS — cycle-invariant) |
| 2 | STATE.md | Last Updated cell | IN SCOPE | Block (ALWAYS — cycle-invariant) |
| 3 | STATE.md | Phase Progress rows (latest pass) | IN SCOPE (cycle-conditional per ADR-023 Option (c)) | F5-per-pass cycle → Block; milestone cycle → Advisory; fail-open-to-advisory |
| 4 | STATE.md | Concurrent Cycles row | IN SCOPE (cycle-conditional per ADR-023 Option (c)) | F5-per-pass cycle → Block; milestone cycle → Advisory; fail-open-to-advisory |
| 5 | STATE.md | Session Resume Section 1 | IN SCOPE (cycle-conditional per ADR-023 Option (c)) | F5-per-pass cycle → Block; milestone cycle → Advisory; fail-open-to-advisory |
| 6 | INDEX.md | Convergence Status row | IN SCOPE | Advisory |
| 7 | INDEX.md | adversarial-review summary-table rows (latest pass) | IN SCOPE | Advisory |
| 8 | burst-log.md | latest-pass `^### Dim-7` block (heading prefix; actual variants confirmed in brownfield-backfill burst-log: `### Dim-7 Attestation / Closes`, `### Dim-7 Attestation`, `### Dim-7 attestation`) | IN SCOPE | Advisory |
| 9 | lessons.md | latest-lesson trend-table | OUT-OF-SCOPE (per F-SP5-003; lessons.md no machine-extractable trend-table structure) | Advisory (no-op; host::log_warn only) |
| 10 | decision-log.md | trajectory-bearing rows | OUT OF SCOPE | Advisory-prose only |
| 11 | adv-cycle-pass-*.md | frontmatter `trajectory_tail` field | OUT OF SCOPE | Written by adversary |

## Adversary Pass Coverage

Pass-1 (2026-05-28; 14 findings 5H+5M+3L+1N; closed in this v1.1 — 9 BC findings (F-S15.17-SP1-002/004/005/007/009/010/011/012/014); story-writer fix-burst closes 5 remaining (F-001/003/006/008/013)). BC gate: BC-5.39.009 MUST reach 3-CLEAN before S-15.17 is promoted to `status: ready` per Spec-First Gate S-7.01. STREAK: 0/3 (pass-1 = HIGH verdict; streak reset).

Pass-2 (2026-05-28; 11 findings 3H+4M+3L+1N including F-001 regression of F-SP1-003 cascade-propagation-gap; closed in this v1.2 — 7-8 PO findings (F-003/004/005/006/007/008/010/011 + partial F-009); story-writer fix-burst closes 5 remaining incl. mandatory bidirectional parity check per pass-2 META-LEVEL-31 mandate). STREAK: 0/3 (pass-2 = HIGH verdict; streak reset).

Pass-3 (2026-05-28; 14 findings 2C+5H+4M+3L+1N+1PG; closed in this v1.3 — 9 PO findings (F-SP3-001 CRITICAL/F-SP3-002 CRITICAL/F-SP3-003/F-SP3-005/F-SP3-008/F-SP3-009/F-SP3-011/F-SP3-013/F-SP3-014); story-writer fix-burst closes 5 remaining (F-SP3-004/F-SP3-006/F-SP3-007/F-SP3-010 + story mirrors of SP3-001/SP3-002/SP3-003/SP3-005). SDK-GROUNDING MANDATE from pass-3 root-cause analysis satisfied. STREAK: 0/3 (pass-3 = HIGH verdict; streak reset; 2 CRITICALs closed this burst).

Pass-4 (2026-05-28; 16 findings 1C+6H+5M+2L+1N+1PG REGRESSING; closed in this v1.4 — 10 PO findings (F-SP4-001 CRITICAL/F-SP4-002/F-SP4-004/F-SP4-005/F-SP4-006/F-SP4-007/F-SP4-009/F-SP4-010/F-SP4-012/F-SP4-013); story-writer fix-burst closes remaining (F-SP4-003/F-SP4-008/F-SP4-011/F-SP4-015 + mirrors F-SP4-006). POLICY 5 v1.3.1 stable-anchor extension applied (META-LEVEL-32 cure-extension). STREAK: 0/3 (pass-4 = HIGH verdict; streak reset; trajectory 14→11→14→16 REGRESSING).

Pass-5 (2026-05-28; 12 findings 1C+4H+5M+1L+1N IMPROVING trajectory; closed in v1.5 — 7 PO findings: F-SP5-001 CRITICAL/F-SP5-002/F-SP5-003/F-SP5-005/F-SP5-006/F-SP5-007/F-SP5-008; story-writer fix-burst closed 5 remaining: F-SP5-004/F-SP5-009/F-SP5-010/F-SP5-011/F-SP5-012). HUMAN-DIRECTED PARTIAL REVERSAL §Cure-Extension Parsimony Note point 2; META-LEVEL-33 CANDIDATE codified via POLICY 5 v1.3.3 sibling-sweep mandate. STREAK: 0/3 → 0/3 (pass-5 HIGH; reset).

Pass-6 (2026-05-29; 11 findings 0C+5H+4M+1L+1N + 1 process-gap; slight-improvement trajectory; closed in this v1.6 — 8 PO findings: F-SP6-001/F-SP6-002/F-SP6-004/F-SP6-005/F-SP6-006/F-SP6-007/F-SP6-010 + F-SP6-PG-001 META-34 codification; story-writer fix-burst closes 5 remaining: F-SP6-003/F-SP6-008/F-SP6-009/F-SP6-011 + mirror of F-SP6-002). META-LEVEL-34 CANDIDATE codified via POLICY 5 v1.3.3 → v1.3.4 literal-shell sibling-sweep VERIFICATION GATE. STREAK: 0/3 → 0/3 (pass-6 HIGH; reset). 0 CRITICAL sustained (marker-prefix cure HELD).

Pass-7 (2026-05-29; 9 findings 0C+3H+4M+1L+1N + 1 process-gap; MATERIAL DECREASE below asymptotic-floor [11-16]; closed in this v1.7 — 6 BC-side PO findings + 1 process-gap META-35 codification: F-SP7-002 HIGH (Grep 1 arithmetic error '4 variants'→'5 variants'); F-SP7-004 MEDIUM (Grep 10 non-reproducible at replay — annotated with D-518 snapshot label + parent-commit SHA); F-SP7-005 MEDIUM (§Architecture Anchors extractor return type inconsistency — all 4 extractors normalized to Option<String> with rationale note); F-SP7-006 MEDIUM (PC2 + PC5 bodies missing extractor function name citations — extract_last_updated_cell() added to PC2; extract_session_resume_section_1() added to PC5); F-SP7-008 LOW (Pass-6 count '0C+5H+4M+1L+1N+1PG' sums to 12 — reformatted to '0C+5H+4M+1L+1N + 1 process-gap'); F-SP7-PG-001 PROCESS-GAP HIGH META-LEVEL-35 CODIFICATION (policies.yaml POLICY 5 v1.3.4→v1.3.5 — historical-by-construction enumeration + adversary-replay-reproducibility mandate + sibling-sweep categories (f)/(g)/(h) extension). Story-writer next burst: F-SP7-001/F-SP7-003/F-SP7-007. STREAK: 0/3 → 0/3 (pass-7 HIGH; reset). 0 CRITICAL sustained 3 passes (marker-prefix cure HOLDS). trajectory IMPROVING 14→11→14→16→12→11→9 (FIRST sub-11 reading; material drop; META-35 cure required for sustained convergence).

**v1.9 (2026-05-30) — HUMAN-AUTHORIZED un-SEAL + ADR-023 Option (c) cycle-conditional re-spec.** v1.8 was SEALED at D-522 (D-386 Option C asymptotic acceptance). The human explicitly authorized un-SEALing to apply ADR-023 Option (c). This is a MATERIAL behavioral re-spec of a SEALED contract: PC3/PC4/PC5 changed from unconditional-Block to cycle-conditional-Block (F5-per-pass → Block; milestone → Advisory; fail-open-to-advisory). Therefore the prior cascade CONVERGENCE (S-15.17 adv-local pass-5 CLEAN, streak 3/3) is VOIDED for the new behavior — a FRESH 3-CLEAN LOCAL cascade is required per ADR-023 §5 + BC-5.39.001, OR explicit asymptotic-acceptance per D-386 Option C. STREAK: 0/3 (un-seal re-spec resets the streak for the new cycle-conditional behavior).

Pass-8 (2026-05-29; 11 findings 1C+5H+3M+1L+0N + 1 process-gap REGRESSED 9→11; CRITICAL returned (F-SP8-001 paper-fix from pass-5); META-LEVEL-36 CANDIDATE; closed in this v1.8 — 6 BC-side PO findings + 1 process-gap META-36 codification: F-SP8-001 CRITICAL (§Cure-Extension Parsimony Note point 2 PAPER-FIX — rewritten to document F-SP5-001 HUMAN-DIRECTED PARTIAL REVERSAL; "deliberate non-extension" narrative replaced with full two-step marker-prefix discipline extension rationale); F-SP8-002 HIGH (PC10 body + §D-453(d) Site 9 PAPER-FIX — PC10 rewritten to OUT-OF-SCOPE annotation per F-SP5-003; Site 9 row updated from IN SCOPE to OUT-OF-SCOPE); F-SP8-006 HIGH [META-36] (Grep 10 snapshot-annotation-rescue-pattern — rewritten to STRUCTURAL-FORM-only per POLICY 5 v1.3.6 Part B; snapshot stdout removed; structural grep predicates now HEAD-reproducible at any cycle SHA); F-SP8-007 MEDIUM (§Architecture Anchors extractor catalog '5 of 7' — updated from 'All 4 body extractors' to 'All 7 section extractors'; extract_frontmatter_current_step + extract_burst_log_latest_dim7 Option<String> bullets added); F-SP8-009 MEDIUM (covered by F-SP8-001 rewrite of point 2); F-SP8-PG-001 PROCESS-GAP HIGH META-LEVEL-36 CODIFICATION: policies.yaml POLICY 5 v1.3.5→v1.3.6 — Part B HEAD-reproducibility-or-structural-form mandate + Part D snapshot-rescue-pattern detection (snapshot-annotation-rescue-pattern defeats Part B reproducibility through fresh-context-loop-asymmetry; adversary works at HEAD not historical SHA). Story-writer next burst: F-SP8-003/F-SP8-004/F-SP8-005/F-SP8-008/F-SP8-010. STREAK: 0/3 → 0/3 (pass-8 CRITICAL; reset). trajectory REGRESSED 14→11→14→16→12→11→9→11.

## Preconditions

### PostToolUse activation

1. A PostToolUse Edit/Write event has fired on one of the four target files. Target file detection
   uses `Path::new(file_path).file_name()` path-component-strict matching (see invariant 3):
   - `STATE.md` → triggers STATE.md arm (sites 1-5; Block severity)
   - `INDEX.md` within the active cycle path (path-component-strict basename `INDEX.md` AND
     full path contains the active cycle name resolved dynamically from STATE.md
     `current_cycle:` frontmatter field) → triggers INDEX.md arm (sites 6-7; Advisory severity)
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
   - `file_path` is within the `.factory/` tree — specifically, the path components MUST include
     the literal component `.factory`: `Path::new(file_path).components().any(|c| c.as_os_str() == ".factory")`
     (path-component-walk form — platform-independent, robust to `./` prefix, Windows backslash,
     and any path normalization variant)

   **Rationale (F-SP3-008):** A WASM hook that fires on ANY `STATE.md` anywhere on the filesystem
   (e.g., `/tmp/STATE.md`, `/home/user/notes/STATE.md`) and emits a Block-grade exit code is a
   false-positive risk. The `.factory/` parent guard is the production-grade constraint — only the
   factory pipeline's STATE.md carries the D-453(d) trajectory_tail requirement. Any `STATE.md`
   outside `.factory/` MUST be treated as a non-target file: `HookResult::Continue` immediately
   (see EC-019). The path-component-walk form replaces the v1.2 string-form checks
   (`starts_with(".factory/")` + `contains("/.factory/")`) which were not robust to path
   normalization edge cases.

   The INDEX.md arm fires ONLY when both of the following hold:
   - `Path::new(file_path).file_name() == Some("INDEX.md")` (path-component-strict basename check)
   - The **active cycle path** is matched: the hook reads `.factory/STATE.md` frontmatter
     `current_cycle:` field at runtime (via a secondary `host::read_file` call on STATE.md),
     extracts the cycle name via `extract_current_cycle()`, and checks via path-component-walk:
     `Path::new(file_path).components().any(|c| c.as_os_str() == active_cycle.as_str())`
     (MANDATORY path-component-walk form per F-SP4-006; substring `String::contains` FORBIDDEN
     — false-positive risk on overlapping cycle name prefixes). If the STATE.md read fails
     (any HostError), the hook falls back to fail-open: `HookResult::Continue` + `host::log_warn`
     for the INDEX.md arm only (not for STATE.md Block arm).

   **Rationale (F-SP3-001 — CRITICAL dynamic resolution):** The v1.2 cycle-path guard hardcoded
   `v1.0-feature-engine-discipline-pass-1/` — the F5 cycle which is PAUSED (`paused_pending_resume: true`
   per `.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md`). The active cycle is
   `v1.0-brownfield-backfill` (per STATE.md `current_cycle: v1.0-brownfield-backfill`). The
   hardcoded guard made the INDEX.md arm silently inert on all active-cycle artifacts (INDEX.md,
   burst-log.md, lessons.md in `v1.0-brownfield-backfill/`). This is the same META-LEVEL-30 route
   (b) class at the runtime layer. Dynamic resolution via STATE.md `current_cycle:` read is option
   (a) per the mandate — most robust to cycle transitions, matches the "active cycle" semantic, and
   requires no BC amendment when future cycles become active. Option (a) chosen per mandate
   recommendation.

   **§Cure-Extension Parsimony Note (F-SP3-001 addendum):** The dynamic current_cycle: resolution
   adds one additional `host::read_file` call (to read STATE.md for the INDEX.md arm cycle guard).
   This is NOT a second read of the primary STATE.md write target — it is a separate internal
   lookup whose result is used only for INDEX.md arm routing. The hook MUST distinguish these two
   paths: (1) primary STATE.md validation (Block arm, sites 1-5) and (2) cycle-name lookup (used
   only for INDEX.md arm path matching). If the cycle-name lookup fails, only the INDEX.md arm
   falls back to fail-open; the STATE.md arm is unaffected.

   The burst-log.md and lessons.md arms match by basename only (no cycle-path guard needed —
   `burst-log.md` and `lessons.md` are unambiguous basenames across cycles).

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

### Cycle-type gate — per_pass_trajectory resolution (ADR-023 Option (c); v1.9)

7. **The STATE.md per-pass sites (PC3/PC4/PC5) Block-vs-advisory routing is gated on the active
   cycle's `per_pass_trajectory` flag.** Before evaluating PC3/PC4/PC5, the hook resolves whether
   the active cycle is an F5-style per-pass cycle via the following runtime resolution path
   (reusing the existing `extract_current_cycle()` + secondary `host::read_file` machinery already
   used by the INDEX.md arm per Precondition 4, plus a new `extract_per_pass_trajectory_flag()`
   extractor — see §Architecture Anchors):

   - **Step 1 — resolve active cycle name.** Read `.factory/STATE.md` frontmatter `current_cycle:`
     via `extract_current_cycle()` (the same dynamically-resolved cycle name used by the INDEX.md
     arm; for the STATE.md arm this is the content already in hand — the primary STATE.md write
     payload read via `host::read_file` with `max_bytes = 524288` per inv-7). If `current_cycle:`
     cannot be resolved (`None`) → **fail-open-to-advisory** (Step 4).
   - **Step 2 — read the active cycle's INDEX.md frontmatter.** Construct the path
     `.factory/cycles/<active_cycle>/INDEX.md` and read it via a secondary `host::read_file` call
     (`max_bytes = 524288`, `timeout_ms = 2000` per inv-7). If the read returns ANY `HostError`
     variant (`OutputTooLarge`, `Timeout`, `CapabilityDenied`, `InvalidArgument`, `Other(i32)`),
     or the bytes fail `String::from_utf8` decode (per inv-13), or the file is absent →
     **fail-open-to-advisory** (Step 4).
   - **Step 3 — extract the flag.** Apply `extract_per_pass_trajectory_flag(index_md_content)`:
     scan the INDEX.md YAML frontmatter region (between first and second `---\n` delimiters) for a
     line matching `^per_pass_trajectory:` and parse the YAML boolean value. The flag is
     considered **TRUE** if and only if the value parses to the boolean `true` (accepting bare
     `true`, and YAML scalar forms `True`/`TRUE` per case-insensitive boolean parse; a trailing
     `#` comment is stripped and the remainder trimmed before parsing). Any other value
     (`false`, `False`, `no`, `0`, empty, or a non-boolean string) → flag is **FALSE**. If the
     `per_pass_trajectory:` key is ABSENT from the frontmatter → flag is **FALSE** (milestone
     cycles omit the flag by convention).
   - **Step 4 — fail-open-to-advisory default.** Whenever the flag cannot be read for ANY reason
     (current_cycle `None`, INDEX.md HostError, utf8 decode failure, absent INDEX.md, unparseable
     frontmatter), the resolved cycle-type is treated as **NOT per-pass (flag = FALSE)** — i.e.,
     PC3/PC4/PC5 are routed to ADVISORY (`host::log_warn` + Continue), NEVER Block. This is
     **fail-open-to-advisory, NEVER fail-open-to-Block**: a Block-on-unresolvable-flag would
     re-introduce the v1.8 pipeline-brick risk. The fail-open emits `host::log_warn` noting the
     reason the flag could not be resolved (advisory-only; non-blocking).

   **Routing result:** if `per_pass_trajectory == true` (F5-style per-pass cycle) → PC3/PC4/PC5
   are Block-severity (missing tail → listed in the STATE.md missing-sites cascade per PC6). If
   `per_pass_trajectory == false` OR unresolvable (milestone cycle / fail-open) → PC3/PC4/PC5 are
   ADVISORY-severity (missing tail → `host::log_warn` + Continue; NOT added to the Block cascade).
   **PC1 and PC2 are UNAFFECTED by this gate — they always Block on missing tail regardless of
   cycle type.**

   **Rationale (ADR-023 Option (c)):** The discriminator is an explicit per-cycle frontmatter flag
   authored by state-manager at cycle creation — NOT a hardcoded cycle-name match (which would
   repeat the F-SP3-001 hardcoding defect that the v1.2→v1.3 dynamic-resolution fix eliminated).
   This reuses the existing dynamic cycle-resolution plumbing ("wiring not redesign") and
   generalizes correctly to all future cycles: an F5-style per-pass cycle carries
   `per_pass_trajectory: true`; milestone/story-delivery cycles omit it or set `false`.
   State-manager authors `per_pass_trajectory: true` on F5-cycle INDEX.md
   (`v1.0-feature-engine-discipline-pass-1/INDEX.md`) per ADR-023 §3 (this flag is currently
   ABSENT on all cycle INDEX.md files — see §SDK Grounding Evidence Grep 11; its authoring is a
   state-manager obligation tracked in this BC's Changelog).

## Postconditions

### STATE.md arm — Block severity (sites 1-5)

1. When STATE.md is written, the frontmatter `current_step:` field value is extracted by scanning
   the YAML frontmatter region (`&str` post-decode per inv-13) for a line matching `^current_step:`
   and capturing the remainder of that line as the value. The two-step inv-4 marker-prefix check
   is then applied to this extracted value:
   - Step 1 (marker): if the literal substring `trajectory-tail ` (with trailing space) is ABSENT,
     the site is treated as missing → included in the missing-sites list → Block via PC6.
   - Step 2 (count): if the marker IS present, count `→(\d+)` matches in the substring from
     marker-end to first `;` (or end-of-value). If count ≠ 4 → site listed as missing → Block.
   **Production STATE.md evidence:** `current_step:` contains `trajectory-tail →9→9→9→11` (marker
   present; scoped segment count=4 → PASS). The full `current_step:` value contains multiple
   `trajectory-tail` marker mentions — the first-occurrence semicolon-segment scoping yields count=4
   → PASS per inv-4 (see Grep 10).

2. When STATE.md is written, the "Last Updated" table cell is extracted (on `&str` post-decode per
   inv-13) by `extract_last_updated_cell()`: scans the body for the markdown table row pattern
   `| **Last Updated** |` and captures the cell value in the second pipe-delimited column (text
   between 2nd and 3rd unescaped `|`; strip leading/trailing whitespace; join continuation lines
   on whitespace per F-SP4-013). The extraction is performed by `extract_last_updated_cell()` (see
   §Architecture Anchors). The two-step inv-4 marker-prefix check is then applied to this cell value:
   - Step 1 (marker): if the literal substring `trajectory-tail ` (with trailing space) is ABSENT
     in the cell value → site treated as missing → included in missing-sites list → Block via PC6.
   - Step 2 (count): if the marker IS present, count `→(\d+)` matches in the substring from
     marker-end to first `;` (or end-of-cell-value). If count ≠ 4 → site listed as missing → Block.
   If the `| **Last Updated** |` row is absent entirely, the site is treated as missing (Block).
   **Extractor note:** "Last Updated" is a TABLE ROW inside the body — there is NO `## Last Updated`
   heading. Extractor MUST NOT scan for `## Last Updated` heading (no such heading exists).
   **NOTE (F-SP5-001/F-SP5-008 adjudication — coordination requirement satisfied at D-518):** PC2
   REQUIRES the `trajectory-tail` marker form in the Last Updated cell.
   **Production STATE.md evidence (post-D-518):** Last Updated cell contains
   `trajectory-tail →9→9→9→11` after the prose trajectory segment (verified via Grep 10).
   State-manager Commit E dispatch templates updated at D-518 to include the marker; production
   discipline now satisfies PC2 STRICT marker-prefix two-step check. The marker MAY follow other
   prose in the cell (e.g., `2026-05-28 — D-518 ... Trajectory IMPROVING ...; trajectory-tail
   →9→9→9→11`). See §Architecture Anchors for the coordination NOTE.

3. **CYCLE-CONDITIONAL (per Precondition 7; ADR-023 Option (c)).** When STATE.md is written and
   the "Phase Progress" section's latest-pass row does not yield a `→(\d+)` match count of exactly
   4 (LENGTH=4 STRICT equality per inv-4) within the `trajectory-tail ` marker segment, the
   routing depends on the active cycle's `per_pass_trajectory` flag (Precondition 7):
   - **If `per_pass_trajectory == true` (F5-style per-pass cycle):** the hook treats this as a
     missing site and includes "STATE.md Phase Progress rows" in the STATE.md missing-sites list
     → Block via PC6.
   - **If `per_pass_trajectory == false` OR unresolvable (milestone cycle / fail-open-to-advisory
     per Precondition 7 Step 4):** the hook emits `host::log_warn`
     ("validate-trajectory-tail-cell-completeness: advisory — STATE.md Phase Progress latest row
     missing trajectory_tail (cycle-conditional; active cycle not per_pass_trajectory)") and does
     NOT add the site to the Block cascade. Milestone-cycle Phase Progress rows are milestone/status
     rows that structurally carry no per-pass trajectory — tail-less is the NORMAL state, NOT a
     violation, in such cycles.
   **Extractor specification:** The
   `## Phase Progress` heading IS present in production STATE.md (verified via stable anchor
   `^## Phase Progress`). The extractor MUST scan for the `^## Phase Progress` heading
   (prefix-match; no line number — stable anchor per TD-VSDD-091) and capture the TABLE ROWS
   that follow it (until the next `##` heading). To identify the "latest pass row": take the
   BOTTOMMOST row of the Phase Progress table, skipping any rows whose Status cell contains
   "ARCHIVED" or "COMPACTED". **ONE-tail-per-extracted-region precondition (F-SP4-001):** The
   `extract_phase_progress_latest_row` function MUST return a SINGLE ROW TEXT — the text between
   the `|` delimiters of the bottommost non-archived/non-compacted table row only, NOT the entire
   Phase Progress section. The `count == 4` equality check (inv-4) operates on this single-row
   text. Passing the whole-section text to the arrow-count check would produce false-Block
   results because the Phase Progress section contains many rows, each with trajectory_tail
   sequences, totaling far more than 4 arrow-segments. The state-manager Commit E discipline
   appends one row per burst — the bottommost non-archived/non-compacted row IS the latest by
   construction. Do NOT skip rows with Status "COMPLETE", "SHIPPED", "MERGED", or "CYCLE CLOSED"
   — most Phase Progress rows carry these statuses and skipping them would skip the most recent
   fix-burst row. If no non-archived/non-compacted row exists (all rows compacted), the site is
   treated as present (pass-through). **Production STATE.md evidence (cycle-conditional reality —
   ADR-023):**
   ```
   $ grep '^## Phase Progress' .factory/STATE.md
   ## Phase Progress
   $ grep '^current_cycle:' .factory/STATE.md
   current_cycle: v1.0-brownfield-backfill
   ```
   The `## Phase Progress` heading exists. The active cycle is `v1.0-brownfield-backfill` — a
   MILESTONE/story-delivery cycle (NOT F5-per-pass). Its `per_pass_trajectory` flag is absent
   (Precondition 7 → flag = FALSE → ADVISORY). The bottommost Phase Progress row in this cycle is
   a milestone/status row (e.g., D-524 "PR #163 ... + S-15.17 ... captured") that carries NO
   per-row trajectory_tail — this is the NORMAL, CORRECT state for a milestone cycle, NOT
   degradation. Under v1.9 cycle-conditional routing this row is ADVISORY (no Block). In an
   F5-style per-pass cycle (`per_pass_trajectory: true`), the bottommost non-archived row carries
   the most recent pass's trajectory_tail (single row, between `|` delimiters) and a missing tail
   IS a Block-grade degradation (the ADV-EDP1-P75-HIGH-002 finding class). State-manager does NOT
   retro-fabricate tails onto milestone rows (Option (a) paper-compliance — REJECTED per ADR-023).

4. **CYCLE-CONDITIONAL (per Precondition 7; ADR-023 Option (c)).** When STATE.md is written, the
   "Concurrent Cycles" LATEST ROW is extracted (on `&str` post-decode
   per inv-13) by `extract_concurrent_cycles_latest_row()`: scan for the `^## Concurrent Cycles`
   heading (stable anchor; no line number per TD-VSDD-091), capture the TABLE ROWS that follow
   until the next `^## ` heading, then take the BOTTOMMOST table data row (after stripping header
   and separator rows) whose Status cell does NOT contain "CLOSED", "COMPACTED", or "ARCHIVED".
   **ONE-tail-per-extracted-region precondition (parallel to PC3/F-SP4-001):** the function MUST
   return a SINGLE ROW TEXT — the text between the `|` delimiters of the bottommost active/in-
   progress row only, NOT the entire Concurrent Cycles section. The `count == 4` check operates on
   this single-row text. Passing the whole-section text would produce false-Block (the section
   contains multiple rows with multiple trajectory_tail sequences). The two-step inv-4 marker-prefix
   check is applied to this single-row text, and the result is routed per the cycle-type flag:
   - Step 1 (marker): if `trajectory-tail ` (with trailing space) is ABSENT in the single-row text
     → site treated as missing (subject to cycle-conditional routing below).
   - Step 2 (count): if marker IS present, count `→(\d+)` matches from marker-end to first `;`
     (or end-of-row). If count ≠ 4 → site treated as missing (subject to cycle-conditional routing).
   - **Cycle-conditional routing (Precondition 7):** if `per_pass_trajectory == true` (F5-style
     per-pass cycle) → missing site listed in the STATE.md missing-sites list → Block via PC6. If
     `per_pass_trajectory == false` OR unresolvable (milestone cycle / fail-open-to-advisory) →
     `host::log_warn` advisory ("STATE.md Concurrent Cycles latest active row missing
     trajectory_tail (cycle-conditional; active cycle not per_pass_trajectory)") + Continue; NOT
     added to the Block cascade.
   If the `## Concurrent Cycles` section is absent or all rows are CLOSED/COMPACTED/ARCHIVED, the
   site is treated as present (pass-through — no active cycle row to validate).
   **Production STATE.md evidence (cycle-conditional reality — ADR-023):** The active cycle is
   `v1.0-brownfield-backfill` (milestone cycle; `per_pass_trajectory` flag absent → ADVISORY). Its
   bottommost active Concurrent Cycles row is a milestone-form row (the brownfield-backfill bolt-on
   / F5 PAUSED status row) that may or may not carry a per-row tail; if tail-less, it is the NORMAL
   milestone state (advisory, no Block) under v1.9 — NOT degradation. In an F5-style per-pass cycle
   (`per_pass_trajectory: true`), the bottommost active row carries a per-pass `trajectory-tail
   →N→N→N→N` marker (marker present; count=4 in the first semicolon segment → PASS;
   Block-grade if missing). See §SDK Grounding Evidence for extraction pattern confirmation.
   **F-SP5-002 rationale:** v1.4 specified "section OR row — undefined"; the full section contained
   6+ arrows across multiple rows, all failing count==4. The bottommost-active-row tightening
   (parallel to PC3 F-SP4-001 cure) is the production-grade fix.

5. **CYCLE-CONDITIONAL (per Precondition 7; ADR-023 Option (c)).** When STATE.md is written, the
   "Session Resume Checkpoint" section's `### §1. Where We Are`
   sub-section content is extracted (on `&str` post-decode per inv-13) by
   `extract_session_resume_section_1()`. The heading uses the prefix `## Session Resume Checkpoint`
   which MAY be followed by an optional parenthetical suffix. The extractor MUST match by PREFIX
   `## Session Resume Checkpoint` (not exact string match) to tolerate the evolving parenthetical.
   The sub-section to validate is `### §1.` content (from the first `### §1.` heading to the next
   `### ` heading or `## ` heading). The extraction is performed by
   `extract_session_resume_section_1()` (see §Architecture Anchors). The two-step inv-4
   marker-prefix check is applied to this sub-section text, and the result is routed per the
   cycle-type flag:
   - Step 1 (marker): if `trajectory-tail ` is ABSENT in the §1 body text → site treated as missing
     (subject to cycle-conditional routing below).
   - Step 2 (count): if marker IS present, count `→(\d+)` from marker-end to first `;`. If
     count ≠ 4 → site treated as missing (subject to cycle-conditional routing).
   - **Cycle-conditional routing (Precondition 7):** if `per_pass_trajectory == true` (F5-style
     per-pass cycle) → missing site listed in the STATE.md missing-sites list → Block via PC6. If
     `per_pass_trajectory == false` OR unresolvable (milestone cycle / fail-open-to-advisory) →
     `host::log_warn` advisory ("STATE.md Session Resume §1 missing trajectory_tail
     (cycle-conditional; active cycle not per_pass_trajectory)") + Continue; NOT added to the
     Block cascade.
   **Production STATE.md evidence (cycle-conditional reality — ADR-023):**
   ```
   $ grep '^## Session Resume' .factory/STATE.md
   ## Session Resume Checkpoint (2026-05-30 — D-524 SESSION-END DURABILITY BURST; ...)
   $ grep '^### §1' .factory/STATE.md
   ### §1. Where We Are
   $ grep '^current_cycle:' .factory/STATE.md
   current_cycle: v1.0-brownfield-backfill
   ```
   The active cycle is `v1.0-brownfield-backfill` (milestone cycle; `per_pass_trajectory` flag
   absent → ADVISORY). The §1 body in a milestone cycle is a status narrative that need not carry
   a per-pass `trajectory-tail →N→N→N→N` marker — if tail-less, it is the NORMAL milestone state
   (advisory, no Block) under v1.9, NOT degradation. In an F5-style per-pass cycle
   (`per_pass_trajectory: true`), the §1 body carries the per-pass tail and a missing tail IS a
   Block-grade degradation. Extractor MUST use prefix-match `## Session Resume Checkpoint`.

6. When one or more **Block-routed** STATE.md sites are missing trajectory_tail (LENGTH ≠ 4 per
   inv-4 equality check), the hook emits a single `HookResult::block_with_fix(...)` enumerating
   ALL missing Block-routed sites in the message body (schema-violation cascade per invariant 8 —
   never short-circuits on first missing site). **Block-routed sites depend on the cycle type
   (Precondition 7; ADR-023 Option (c)):** PC1 (`current_step:`) and PC2 (Last Updated cell) are
   ALWAYS Block-routed (cycle-invariant). PC3/PC4/PC5 (Phase Progress / Concurrent Cycles /
   Session Resume §1) are Block-routed ONLY when `per_pass_trajectory == true` (F5-style per-pass
   cycle); in milestone cycles or on fail-open-to-advisory they are advisory and are NOT included
   in this cascade. Thus in a milestone cycle the maximum cascade is {current_step, Last Updated};
   in an F5-per-pass cycle the maximum cascade is all 5 STATE.md sites. **PC6 cascade-Block emission is exercised via AC anchors
   to invariant 8 per §Behavioral Contracts Table indirection** (F-SP3-005 cross-reference):
   story ACs that trace to invariant 8 provide the indirect PC6 anchor; story-writer MUST extend
   AC-7 to cite PC6 alongside invariant 8.
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

7. When INDEX.md (active cycle, resolved dynamically per Precondition 4) is written and the
   "Convergence Status" row does not yield a `→(\d+)` match count of exactly 4 (LENGTH=4 STRICT
   equality per inv-4), the hook emits `host::log_warn` with message
   "validate-trajectory-tail-cell-completeness: advisory — INDEX.md Convergence Status row missing
   trajectory_tail (→N→N→N→N, D-453(d) site 6)" and emits `HookResult::Continue` (NOT Block).

8. When INDEX.md (active cycle, resolved dynamically per Precondition 4) is written and the
   adversarial-review summary-table's latest-pass row does not yield a `→(\d+)` match count of
   exactly 4 (LENGTH=4 STRICT equality per inv-4), the hook emits `host::log_warn`
   with message "validate-trajectory-tail-cell-completeness: advisory — INDEX.md adversarial-review
   summary-table latest-pass row missing trajectory_tail (D-453(d) site 7)" and emits
   `HookResult::Continue`.

### burst-log.md arm — Advisory severity (site 8)

9. When burst-log.md is written, the "latest Dim-7 block" is extracted (on `&str` post-decode per
   inv-13) by `extract_burst_log_latest_dim7()`: scan all `^### Dim-7` headings in file-order and
   select the BOTTOMMOST occurrence. The block is the text from that heading up to (but NOT
   including) the next `^## ` or `^### ` heading (whichever comes first in file order). The two-
   step inv-4 marker-prefix check is applied to this block text:
   - Step 1 (marker): if `trajectory-tail ` is ABSENT → `host::log_warn` advisory + `HookResult::Continue` (NOT Block; advisory arm).
   - Step 2 (count): if marker IS present, count `→(\d+)` from marker-end to first `;`. count ≠ 4 → advisory log_warn + Continue (NOT Block).
   If the `^### Dim-7` heading is absent entirely (e.g., in a future cycle with different burst-log
   structure), the site is treated as PRESENT (fail-open per inv-10) with `host::log_warn`.
   **Extractor specification (F-SP4-004 — actual heading form; F-SP5-007 "latest" semantics):**
   Heading prefix `^### Dim-7` (case-insensitive prefix-match; actual variants confirmed in active
   cycle `v1.0-brownfield-backfill/burst-log.md`: `### Dim-7 Attestation / Closes`,
   `### Dim-7 Attestation`, `### Dim-7 attestation`, `### Dim-7 Attestation (Closes / Advances)` —
   all share the prefix `### Dim-7`). "Bottommost in file-order" is the canonical selection rule
   (parallel to PC3/PC4 bottommost-row rule). See §Architecture Anchors for `extract_burst_log_latest_dim7()` spec.
   **Scope: brownfield-backfill cycle only.** F5 cycle burst-log has ZERO `^### Dim-7` headings
   (absent-heading fail-open applies). §SDK Grounding Evidence Grep 4 provides literal stdout.
   Advisory arm emits `host::log_warn` message: "validate-trajectory-tail-cell-completeness: advisory
   — burst-log.md latest-pass Dim-7 block missing trajectory_tail (D-453(d) site 8)".

### lessons.md arm — Advisory severity (site 9)

### PC10 — lessons.md latest-lesson trend-table (OUT-OF-SCOPE per F-SP5-003)

OUT-OF-SCOPE annotation: lessons.md trends are inline prose ("trend N→N→N→N→N") within `## LESSON-*` bodies, NOT a machine-extractable `### Trend Table` heading or markdown table. PC9 v1.5 was successfully grounded to actual burst-log `^### Dim-7` heading; PC10 lessons.md has NO structural anchor. The lessons.md arm of the hook MUST emit `host::log_warn` advisory + `HookResult::Continue` regardless of content; no count check runs. Future story may formalize a lessons.md trend-table schema; until then, PC10 is a no-op-advisory site. See §Cure-Extension Parsimony Note point 4 for the OUT-OF-SCOPE decision rationale.

### Fail-open postconditions

11. When `host::read_file` returns any `HostError` variant (`HostError::OutputTooLarge`,
    `HostError::Timeout`, `HostError::CapabilityDenied`, `HostError::InvalidArgument`, or
    `HostError::Other(i32)`) for any target file, the hook emits `host::log_warn` with the error
    description and emits `HookResult::Continue`. NEVER Block on any read failure or size limit.

    **Rationale (F-SP3-003 — collapse PC11 into PC12):** Per the adversary recommendation, "any
    HostError → fail-open per inv-10 makes the distinction structurally redundant." The former
    PC11 (`HostError::TooBig`) was incorrect — `TooBig` is NOT a valid `HostError` variant; the
    actual variant is `HostError::OutputTooLarge` (per `crates/hook-sdk/src/host.rs` enum). Rather
    than replace the incorrect variant name in a structurally separate PC, this BC collapses to
    the uniform treatment: all HostError variants → fail-open. The `max_bytes = 524288` cap in
    invariant 7 remains — if a file exceeds this cap, `host::read_file` returns
    `HostError::OutputTooLarge`, which is handled uniformly here. See §SDK Grounding Evidence for
    literal-shell grep confirming `HostError::OutputTooLarge` as the actual variant name.

### Pass postcondition

12. When a target file is written and ALL applicable prescribed sites for that file yield a
    `→(\d+)` match count of exactly 4 (LENGTH=4 STRICT equality per inv-4), the hook emits
    `HookResult::Continue` with no warnings. This is the clean-pass case.

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

4. **Canonical trajectory_tail marker-prefix scoping — LENGTH=4 STRICT equality semantics (F-SP3-002; F-SP5-001 REDESIGN).** The trajectory_tail check for heterogeneous text cells (PC1/PC2/PC4/PC5) is a TWO-STEP operation, adopting the full BC-5.39.006 inv-6(b) semicolon-segment-scoping discipline per F-SP5-001 HUMAN-DIRECTED redesign authority:

   (a) **Marker check (mandatory first step):** search the extracted cell/value text for the literal substring `trajectory-tail ` (with trailing space). If this marker is ABSENT, the site is treated as missing (same as absent trajectory_tail). For STATE.md Block sites (PC1, PC2, PC4, PC5): absent marker → site listed as missing → Block via PC6 cascade. For advisory sites (PC7, PC8): absent marker → `host::log_warn` advisory. This is fail-CLOSED for the primary STATE.md sites — the hook enforces that state-manager consistently embeds the `trajectory-tail` marker in the prescribed cells.

   (b) **LENGTH count (second step, runs only if marker present):** apply regex `→(\d+)` globally to the **substring between the `trajectory-tail ` marker (exclusive of marker) and the first `;` segment-separator** (or end-of-text if no `;` follows). This is identical to BC-5.39.006 inv-6(b) semicolon-segment-scoping — it prevents arrow-pattern false-positives from other trajectory narratives in the same cell (e.g., `Trajectory REGRESSING 14→11→14→16` in the same Last Updated cell would contribute false arrows if counted without marker scoping). The match count MUST equal exactly 4 (count == 4; not >= 4). LENGTH=3 (count=3) is NOT present. LENGTH=5+ (count >= 5) is also NOT present — both are violations per D-433(e)+D-439(c).

   **PC3 exception (Phase Progress single-row):** PC3 applies `extract_phase_progress_latest_row` which returns a SINGLE ROW TEXT (the bottommost non-archived/non-compacted Phase Progress table row). This single-row text typically contains a `trajectory-tail` marker naturally embedded by state-manager Commit E. Apply the same two-step marker-prefix check to the single-row text. The ONE-tail-per-extracted-region precondition ensures false-Block from multi-row sections cannot occur at PC3.

   **PC9 exception (burst-log Dim-7 block):** PC9 applies `extract_burst_log_latest_dim7()` which returns the block text from the bottommost `^### Dim-7` heading. Apply the same two-step marker-prefix check to this block text.

   **Rationale (F-SP5-001 CRITICAL redesign):** The v1.4 inv-4 applied LENGTH=4 STRICT equality to the FULL extracted region without marker scoping. This produced false-Block on production STATE.md: `current_step:` contains two `trajectory-tail →9→9→9→11` mentions (8 arrows total when counted without scoping); `Last Updated` contains `Trajectory REGRESSING 14→11→14→16` (3 arrows from a prose narrative, not a trajectory-tail). The full-region count of 8 or 3 would Block on `current_step:` and `Last Updated` respectively under the v1.4 rule. The marker-prefix scoping from BC-5.39.006 inv-6(b) resolves this: scoping to the `trajectory-tail [^;]+` segment in `current_step:` yields count=4 (correct pass); requiring the marker in `Last Updated` enforces state-manager to add it going forward. This is the SAME design used by BC-5.39.006 inv-6(b) for `current_step:` since v1.3.

   Multi-digit values (`→10→12→11→13`) are valid (count=4). See §SDK Grounding Evidence Grep 10 for literal-shell confirmation of the `trajectory-tail` marker presence in production STATE.md and the correct count-4 result from marker-prefix scoping.

5. **STATE.md sites: Block severity per D-411(a) HIGH classification — CYCLE-CONDITIONAL for
   sites 3-5 (ADR-023 Option (c)).** Omission of trajectory_tail from a Block-routed STATE.md
   prescribed site is classified HIGH severity per D-411(a) (the adversary finding
   ADV-EDP1-P75-HIGH-002 arose precisely from these sites lacking the canonical tail). Block
   severity applies UNCONDITIONALLY to sites 1-2 (`current_step:` + Last Updated cell —
   cycle-invariant). For sites 3-5 (Phase Progress / Concurrent Cycles / Session Resume §1),
   Block severity applies ONLY when the active cycle is an F5-style per-pass cycle
   (`per_pass_trajectory == true` per Precondition 7); in milestone cycles or on
   fail-open-to-advisory, sites 3-5 are advisory (`host::log_warn` + Continue), NOT Block. The
   ADV-EDP1-P75-HIGH-002 degradation was a real F5-cycle event; the v1.9 cycle-conditional model
   preserves the Block for that case while not Blocking legitimately tail-less milestone rows.

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

13. **Extractor input type: `&str` (post-decode).** All section extractors (`extract_frontmatter_current_step`, `extract_last_updated_cell`, `extract_phase_progress_latest_row`, `extract_concurrent_cycles_latest_row`, `extract_session_resume_section_1`, `extract_burst_log_latest_dim7`, `extract_current_cycle`) take `content: &str` as input. The `host::read_file` byte sequence `Vec<u8>` MUST be decoded via `String::from_utf8(bytes)` UPSTREAM of all extractors; UTF-8 decode failure routes via EC-020 fail-open (`HookResult::Continue` + `host::log_warn`). All extractor operations are then performed on the decoded `&str`. The invariant 11 `is_char_boundary()` guard applies to any byte-index slice operations on the `&str` (e.g., if an extractor uses byte-index arithmetic to locate the `→` character `[0xE2, 0x86, 0x92]`). This invariant was implicit in the story T-5 pseudocode (which correctly uses `String::from_utf8` double-match pattern per §SDK Grounding Evidence Grep 3) but was not explicit in the BC until F-SP5-006. (F-SP5-006 MEDIUM cure.)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md `current_step:` key is absent from frontmatter (malformed or incomplete frontmatter) | Block: "STATE.md frontmatter current_step" listed as missing (absent key = absent tail) |
| EC-002 | STATE.md with `current_step: "Phase 3 step"` (text only, no arrow-sequence) | Block: "STATE.md frontmatter current_step" listed as missing |
| EC-003 | STATE.md with `current_step: "→9→9→9"` (LENGTH=3 — `→(\d+)` count=3 ≠ 4) | Block: site listed as missing (count must equal 4 per inv-4 equality semantics) |
| EC-004 | STATE.md exceeds 524288 bytes → `host::read_file` returns `HostError::OutputTooLarge` (actual SDK variant; NOT `TooBig` which does not exist per host.rs enum) | `HookResult::Continue` + `host::log_warn` fail-open; treated uniformly as any HostError per PC11 (collapsed) |
| EC-005 | File path `/some/dir/not-STATE.md` — `file_name() == "not-STATE.md"` | `HookResult::Continue` immediately; basename does not match "STATE.md"; STATE.md arm not triggered (inv-3) |
| EC-006 | STATE.md with 3 of 5 sites missing trajectory_tail simultaneously | Single cascade Block enumerating all 3 missing sites (inv-8); not 3 separate Blocks |
| EC-007 | STATE.md with all 5 sites carrying `→9→9→9→9` | `HookResult::Continue`; no Block or advisory (PC12) |
| EC-008 | INDEX.md from a different cycle path (e.g., `v2.0-future-cycle/INDEX.md` when active cycle is `v1.0-brownfield-backfill`) | `HookResult::Continue`; dynamic cycle-path guard rejects non-active-cycle INDEX.md (Precondition 4 — active cycle resolved from STATE.md `current_cycle:` at runtime) |
| EC-009 | INDEX.md Convergence Status row missing tail | `host::log_warn` advisory + `HookResult::Continue` (NOT Block; inv-6) |
| EC-010 | burst-log.md Dim-7 block missing tail | `host::log_warn` advisory + `HookResult::Continue` |
| EC-011 | lessons.md trend-table missing tail | `host::log_warn` advisory + `HookResult::Continue` |
| EC-012 | `host::read_file` HostError::Timeout for STATE.md | `HookResult::Continue` + `host::log_warn`; fail-open (inv-10) |
| EC-013 | STATE.md has `current_step:` with multi-digit tail `→10→12→11→13` (4 arrows, multi-digit) | `HookResult::Continue` for that site; multi-digit values match `[0-9]+` (inv-4) |
| EC-014 | STATE.md frontmatter has `→9→9→9→9` in current_step but Phase Progress section has no tail | Block: "STATE.md Phase Progress rows" listed as missing; partial presence still counts as missing-per-site |
| EC-015 | File path `/factory-artifacts/STATE.md` vs `.factory/STATE.md` — both have `file_name() == "STATE.md"` | `.factory/STATE.md` triggers STATE.md arm (basename match + parent guard satisfied: component-walk finds `.factory` component). `/factory-artifacts/STATE.md` does NOT trigger STATE.md arm: path-component-walk rejects it because none of its components equal `.factory` (components are `factory-artifacts` and `STATE.md`; `factory-artifacts` ≠ `.factory`) per Precondition 4 path-component-walk form. |
| EC-016 | YAML frontmatter region absent in STATE.md (no `---\n` delimiters found) | `host::log_warn` advisory + `HookResult::Continue` (fail-open per invariant 10; consistent with sibling BC-5.39.006/007/008 fail-open precedent; does NOT block for absent frontmatter) |
| EC-017 | STATE.md with `current_step:` as multi-line YAML block scalar using `|` or `>` | Extractor reads the line containing `current_step:` and the indented continuation lines; must find `(→[0-9]+){4}` somewhere in the full value block |
| EC-018 | STATE.md with `current_step:` containing `→9→9→9→9→9` (LENGTH=5 — `→(\d+)` count=5 ≠ 4) | Block: "STATE.md frontmatter current_step" listed as missing (count=5 ≠ 4 per inv-4 equality semantics; `(→[0-9]+){4}` non-anchored match would incorrectly pass — equality count check is required per BC-5.39.006 inv-6(b) precedent and F-SP3-002) |
| EC-019 | File path `/tmp/STATE.md` or `/home/user/notes/STATE.md` — `file_name() == "STATE.md"` but path components do NOT include `.factory` | `HookResult::Continue` immediately; Precondition 4 path-component-walk rejects non-factory STATE.md (`Path::new("/tmp/STATE.md").components().any(|c| c.as_os_str() == ".factory")` returns false); STATE.md arm MUST NOT fire for STATE.md files outside the `.factory/` path component |
| EC-020 | `host::read_file` returns `HostError::OutputTooLarge` with content that is NOT valid UTF-8 (e.g., file contains arbitrary binary bytes that exceed the 524288-byte cap OR partial-read produces a truncated multi-byte sequence at the boundary) | `HookResult::Continue` + `host::log_warn` fail-open (PC11 uniform HostError handling); the UTF-8 decode failure from `String::from_utf8(bytes)` MUST also be treated as fail-open — return `HookResult::Continue` + `host::log_warn`; NEVER block on UTF-8 decode error; consistent with PC11 fail-open umbrella and inv-10 |
| EC-021 | Milestone / story-delivery cycle (active cycle's INDEX.md has `per_pass_trajectory` absent or `false`, e.g., live `current_cycle: v1.0-brownfield-backfill`): STATE.md is written with PC1 (`current_step:`) + PC2 (Last Updated) carrying valid LENGTH=4 `trajectory-tail` markers, but PC3 (Phase Progress latest row) + PC4 (Concurrent Cycles latest active row) + PC5 (Session Resume §1) are milestone/status rows with NO `trajectory-tail` marker (tail-less by convention) | **NO Block.** PC1+PC2 PASS (markers present, count=4). PC3/PC4/PC5 route to ADVISORY per Precondition 7 (flag FALSE/absent → not per-pass) → `host::log_warn` advisory (one per tail-less per-pass site) + `HookResult::Continue`. This is the EXACT live-STATE.md case the v1.8 model would have bricked (unconditional Block on tail-less milestone rows). v1.9 correctly emits NO Block. Tail-less milestone rows are the NORMAL state — NOT degradation (ADR-023 Option (c)). State-manager does NOT retro-fabricate tails (Option (a) REJECTED). |
| EC-022 | A STATE.md cell carrying a valid LENGTH=4 `trajectory-tail →9→9→9→11` marker that ALSO contains, ELSEWHERE in the same row/region, a multi-segment prose trajectory of LENGTH=5 (e.g., `Full-cycle trajectory: →9→9→9→9→11` — 5 arrow-segments) — both present in the same extracted cell/row text | **PASS.** The inv-4 two-step marker-prefix check scopes the count to the segment from the `trajectory-tail ` marker (exclusive) to the first `;` (or end-of-text). The 5-segment `→9→9→9→9→11` is a PROSE "Full-cycle trajectory" history string, NOT a `trajectory-tail ` marker, so inv-4's first-semicolon-segment scoping correctly ignores it: the scoped count from the `trajectory-tail ` marker is exactly 4 → PASS. This LOCKS inv-4's marker-prefix scoping against the LENGTH=5 prose form — the fix MUST NOT relax inv-4 to LENGTH≥4 to "accommodate" the prose (relaxing would defeat the LENGTH=4-STRICT guard for the actual marker). The regression fixture MUST include a cell with BOTH a 5-segment prose trajectory AND a 4-segment `trajectory-tail ` marker, asserting PASS (ADR-023 LENGTH=4-vs-LENGTH=5 nuance). |

## Canonical Test Vectors

| Scenario | File | Input Condition | Expected Hook Output | Sites Exercised |
|----------|------|----------------|---------------------|-----------------|
| STATE.md all 5 sites present | STATE.md | All 5 sections contain `→9→9→9→9` (count=4 == 4; pass) | `HookResult::Continue` | PC12 |
| STATE.md frontmatter current_step missing | STATE.md | `current_step: "Phase 3 step 42"` (no arrow) | Single Block: "STATE.md frontmatter current_step" | PC1, PC6 |
| STATE.md Last Updated missing | STATE.md | Last Updated section has no `→[0-9]+{4}` | Single Block: "STATE.md Last Updated cell" | PC2, PC6 |
| STATE.md Phase Progress missing | STATE.md | Phase Progress section has no tail | Single Block: "STATE.md Phase Progress rows" | PC3, PC6 |
| STATE.md Concurrent Cycles missing | STATE.md | Concurrent Cycles row has no tail | Single Block: "STATE.md Concurrent Cycles row" | PC4, PC6 |
| STATE.md Session Resume missing | STATE.md | Session Resume Section 1 has no tail | Single Block: "STATE.md Session Resume Section 1" | PC5, PC6 |
| STATE.md cascade: 3 sites missing | STATE.md | Last Updated + Concurrent Cycles + Session Resume all missing | Single Block listing all 3 sites (inv-8) | PC2+PC4+PC5, PC6 cascade |
| STATE.md LENGTH=3 present | STATE.md | current_step contains `→9→9→9` (count=3 ≠ 4 per equality check) | Block: site missing (count=3 ≠ 4; inv-4 equality semantics) | PC1 |
| STATE.md LENGTH=4 multi-digit | STATE.md | current_step contains `→10→12→11→13` | `HookResult::Continue` | inv-4 multi-digit |
| INDEX.md Convergence Status missing | INDEX.md (active cycle) | Convergence Status row has no tail | `host::log_warn` + `HookResult::Continue` (not Block; PC7+inv-6) | PC7 |
| INDEX.md adv-table row missing | INDEX.md (active cycle) | Latest adv-review row has no tail | `host::log_warn` + `HookResult::Continue` | PC8 |
| INDEX.md all sites present | INDEX.md (active cycle) | Both sites carry `→9→9→9→9` (count=4 == 4; pass) | `HookResult::Continue` | PC12 |
| STATE.md LENGTH=5 (critical regression guard) | STATE.md | current_step contains `→9→9→9→9→9` (count=5 ≠ 4 per equality check; `(→[0-9]+){4}` would false-pass) | Block: site missing (count=5 ≠ 4; inv-4 equality semantics per F-SP3-002) | PC1, EC-018 |
| INDEX.md wrong cycle | other-cycle/INDEX.md | Path does not contain active cycle name from STATE.md current_cycle: | `HookResult::Continue` (dynamic cycle guard; EC-008) | Precondition 4 |
| burst-log.md Dim-7 missing | burst-log.md | Latest Dim-7 block has no tail | `host::log_warn` + `HookResult::Continue` | PC9 |
| burst-log.md Dim-7 present | burst-log.md | Latest Dim-7 block has `→9→9→9→9` (count=4 == 4; pass) | `HookResult::Continue` | PC12 |
| lessons.md trend-table missing | lessons.md | Latest trend-table row has no tail | `host::log_warn` + `HookResult::Continue` | PC10 |
| lessons.md trend-table present | lessons.md | Latest trend-table row has `→9→9→9→9` (count=4 == 4; pass) | `HookResult::Continue` | PC12 |
| File too large (OutputTooLarge) | STATE.md | `host::read_file` → `HostError::OutputTooLarge` (file > 524288 bytes) | `HookResult::Continue` + `host::log_warn` (PC11 uniform HostError handling+inv-7) | PC11 |
| Read failure (Timeout) | STATE.md | `host::read_file` → `HostError::Timeout` | `HookResult::Continue` + `host::log_warn` (PC11 uniform HostError handling+inv-10) | PC11 |
| Wrong basename | not-STATE.md | `file_name() == "not-STATE.md"` | `HookResult::Continue` immediately (inv-3) | EC-005 |
| File not target | STORY-INDEX.md | Any write | `HookResult::Continue` immediately | Precondition 1 |
| Milestone cycle: PC1/PC2 present, PC3/PC4/PC5 tail-less (NO Block) | STATE.md | Active cycle INDEX.md has no `per_pass_trajectory` (or `false`); current_step + Last Updated carry `trajectory-tail →9→9→9→11`; Phase Progress / Concurrent Cycles / Session Resume §1 are tail-less milestone rows | PC1+PC2 PASS; PC3/PC4/PC5 `host::log_warn` advisory + `HookResult::Continue` (NO Block) per Precondition 7 cycle-conditional routing | PC1, PC2, PC3, PC4, PC5, PC7, EC-021 |
| F5-per-pass cycle: PC3 tail-less → Block | STATE.md | Active cycle INDEX.md has `per_pass_trajectory: true`; Phase Progress latest row has no `trajectory-tail` marker | Single Block: "STATE.md Phase Progress rows" (cycle-conditional Block fires in F5 cycle) | PC3, PC6, PC7 |
| Fail-open-to-advisory: cycle INDEX.md unreadable | STATE.md | Active cycle INDEX.md read returns `HostError` (or absent / utf8 decode fail) while PC3/PC4/PC5 tail-less | PC1/PC2 evaluated normally; PC3/PC4/PC5 → ADVISORY (`host::log_warn` + Continue); NEVER Block on unresolvable flag (Precondition 7 Step 4) | PC7, EC-021 (fail-open variant) |
| LENGTH=5 prose coexisting with LENGTH=4 marker → PASS | STATE.md | A cell contains BOTH `trajectory-tail →9→9→9→11` (4-segment marker) AND `Full-cycle trajectory: →9→9→9→9→11` (5-segment prose) | `HookResult::Continue` (marker-prefix first-semicolon-segment scoping counts only the 4-segment marker; prose ignored) | inv-4, EC-022 |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-453(d) | Canonical 9-site mapping table (trajectory_tail prescribed sites); this BC is the runtime enforcement of the registry | 1-10 |
| D-454(a) | Per-cell granularity gate: cell-level section extraction, not whole-file grep | 1-11 |
| D-433(e)+D-439(c) | LENGTH=4 trajectory tail requirement (not LENGTH=3; not LENGTH=5+) | inv-4 |
| D-411(a) | HIGH classification for STATE.md trajectory_tail omission → Block severity (cycle-invariant sites PC1/PC2 always; per-pass sites PC3/PC4/PC5 cycle-conditional per ADR-023) | inv-5 |

Non-D-NNN references (moved out of D-NNN table per F-S15.17-SP1-011):
- ADR-023: cycle-conditional STATE.md Block-arm site model (Option (c)) — PC1/PC2 always-Block (cycle-invariant); PC3/PC4/PC5 Block only when active cycle is F5-style per-pass (`per_pass_trajectory: true`), else advisory, fail-open-to-advisory; new Precondition 7 cycle-type gate; EC-021/EC-022. See §ADR References in Traceability. HUMAN-AUTHORIZED un-SEAL of v1.8 (SEALED D-522) to apply this re-spec.
- ADR-018: WASM path_allow capability model + no-subprocess principle — see §ADR References in Traceability. (ADR-021 dropped: cargo-audit-specific, not general no-subprocess principle; generic no-subprocess principle covered by ADR-018 hook-sdk contract. F-SP3-011: duplicate ADR-018 cite collapsed to single line.)
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
| (pending) | Fail-open-HostError Invariant — Continue + log_warn on any HostError variant (including OutputTooLarge when file > 524288 bytes; NOT the non-existent TooBig variant) | bats (pass-read-failure-failopen.bats; pass-file-too-large-failopen.bats — uses HostError::OutputTooLarge) |
| (pending) | Path-component-strict Invariant — wrong basename does not trigger arm | bats (pass-wrong-filename-no-trigger.bats) |
| (pending) | LENGTH=4-Required Invariant — LENGTH=3 tail treated as absent | bats (fail-length-3-absent.bats) |
| (pending) | LENGTH=4-Multi-digit Pass Invariant — multi-digit arrow values pass | bats (pass-length-4-present.bats with →10→12→11→13) |
| (pending) | INDEX.md-Cycle-Path-Guard Invariant — wrong cycle INDEX.md produces Continue | bats (pass-wrong-cycle-index.bats — EC-008) |
| (pending) | Production-Registry Invariant — hook registered at priority 158 with correct tool = "Edit\|Write" | bats (integration-production-registry.bats — checks production hooks-registry.toml) |
| (pending) | Cycle-Invariant-Always-Block Invariant (ADR-023) — PC1 (current_step) + PC2 (Last Updated) Block on missing tail regardless of cycle type | bats (fail-state-current-step-missing-tail-milestone-cycle.bats — assert Block even when per_pass_trajectory absent) |
| (pending) | Milestone-Cycle-No-Block Invariant (ADR-023; EC-021) — milestone cycle (per_pass_trajectory absent/false) with tail-less PC3/PC4/PC5 → NO Block, advisory only; real-STATE.md regression fixture | bats (pass-milestone-cycle-tailless-per-pass-rows.bats — uses real .factory/STATE.md shape; assert exit 0 + advisory in stderr) |
| (pending) | F5-Per-Pass-Cycle-Block Invariant (ADR-023) — F5 cycle (per_pass_trajectory: true) with tail-less PC3 → Block | bats (fail-f5-cycle-phase-progress-missing-tail.bats — INDEX.md fixture with per_pass_trajectory: true) |
| (pending) | Fail-Open-To-Advisory Invariant (ADR-023; Precondition 7 Step 4) — cycle INDEX.md unreadable (HostError/utf8/absent) → PC3/PC4/PC5 advisory, NEVER Block | bats (pass-cycle-flag-unreadable-failopen-advisory.bats) |
| (pending) | LENGTH=5-Prose-Coexist-Pass Invariant (ADR-023; EC-022) — cell with 4-segment trajectory-tail marker + 5-segment prose trajectory → PASS (marker-prefix scoping) | bats (pass-length-4-marker-with-length-5-prose.bats) |

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst. **Architect note
(vp_index_is_vp_catalog_source_of_truth):** the v1.9 cycle-conditional re-spec adds 5 new
verification obligations above (cycle-invariant always-Block, milestone no-Block, F5 per-pass Block,
fail-open-to-advisory, LENGTH=5-prose-coexist). Architect must propagate these to VP-INDEX,
verification-architecture.md, and verification-coverage-matrix.md when VP IDs are allocated.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — trajectory_tail per-cell runtime gate automation; closes META-LEVEL-30 route (b)) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically enforces trajectory_tail per-cell presence at all 9 D-453(d) prescribed sites at write time, preventing META-LEVEL-30 route (b) recurrence ("codified-canonical-registry-with-no-runtime-WASM-gate-enforcing-each-site"). The hook targets the four STATE.md/INDEX.md/burst-log.md/lessons.md artifacts that carry the D-453(d) trajectory_tail sites — factory engine discipline artifacts. E-12 as used in the BC-5.39.xxx family per engine-discipline automation sub-capability convention. |
| Architecture Module | `crates/hook-plugins/validate-trajectory-tail-cell-completeness/` (Rust WASM plugin, new crate); `plugins/vsdd-factory/hooks-registry.toml` (registry entry, priority 158, PostToolUse, `tool = "Edit\|Write"`, `path_allow = [".factory"]`); `plugins/vsdd-factory/hook-plugins/validate-trajectory-tail-cell-completeness.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-453(d) (canonical trajectory_tail prescribed sites registry — runtime enforcement); D-454(a) (per-cell granularity gate discipline); D-433(e)+D-439(c) (LENGTH=4 requirement); D-411(a) (HIGH classification → Block severity for STATE.md sites) |
| ADR References | ADR-017 (Per-Story Adversary Phasing — 3-CLEAN applies to S-15.17 LOCAL cascade; restart-from-0/3 obligation for the v1.9 material re-spec; file: `.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md`); ADR-018 (WASM-Plugin Context Resolvers — `path_allow` capability model; `path_allow = [".factory"]`; no-subprocess principle via host::read_file-only API; the `extract_current_cycle()` / `host::read_file` cycle-resolution machinery reused by Precondition 7 for cycle-type detection; covers generic no-subprocess principle — ADR-021 dropped F-SP2-009 because cargo-audit-specific scope does not apply here); ADR-023 (cycle-conditional STATE.md Block-arm site model, Option (c) — the authoritative adjudication this v1.9 re-spec implements; file: `.factory/specs/architecture/decisions/ADR-023-trajectory-tail-cell-completeness-cycle-conditional-site-model.md`; PC1/PC2 always-Block, PC3/PC4/PC5 cycle-conditional via `per_pass_trajectory` flag, fail-open-to-advisory, new Precondition 7 + EC-021/EC-022; un-SEAL of v1.8 human-authorized). F-SP3-011: ADR-018 duplicate collapsed in §D-NNN Anchor Coverage. |
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

**Extractor return type rationale (F-SP7-005 normalization; F-SP8-007 catalog count correction):** All section extractors return `Option<String>` (owned). Rationale: (a) lifetime simplicity — no `'a` parameter required on extractor functions that may perform whitespace-stripping or segment-extraction creating new strings; (b) extractors DO perform allocation (whitespace-stripping via `trim()`, segment extraction via slice-and-collect) making `Option<&str>` non-ergonomic without lifetime annotations; (c) consistency with `validate-policies-schema` and `validate-state-structure` sibling hook extractors which all return `Option<String>`. All 7 section extractors (`extract_frontmatter_current_step`, `extract_last_updated_cell`, `extract_phase_progress_latest_row`, `extract_concurrent_cycles_latest_row`, `extract_session_resume_section_1`, `extract_burst_log_latest_dim7`, `extract_current_cycle`) return `Option<String>` uniformly. The 8th extractor `extract_per_pass_trajectory_flag` (ADR-023 cycle-type gate; v1.9) is the lone exception — it returns `bool` (NOT `Option<String>`), folding "absent / unparseable / false" to the safe milestone default `false`; the fail-open-to-advisory behavior lives in the caller (Precondition 7 routing), not the extractor.
- **`extract_frontmatter_current_step(content: &str) -> Option<String>` (PC1 extractor):** Scans the YAML frontmatter region (between first and second `---\n` delimiters) for a line matching `^current_step:` and captures the remainder of that line as the value. Returns `Some(value)` if found; `None` if the frontmatter region is absent or the key is missing (absent key = missing site → Block via PC6).
- **`extract_burst_log_latest_dim7(content: &str) -> Option<String>` (PC9 extractor; F-SP5-007 latest semantics):** Scans all `^### Dim-7` headings in file-order and selects the BOTTOMMOST occurrence. Block is the text from that heading up to (but NOT including) the next `^## ` or `^### ` heading. Returns `Some(block_text)` if the heading is found; `None` if no `^### Dim-7` heading exists (absent heading = fail-open per inv-10 — treat as advisory-only pass-through). The two-step inv-4 marker-prefix check operates on this block text.

- `crates/hook-plugins/validate-trajectory-tail-cell-completeness/src/lib.rs` — hook logic: `target_arm` enum routing; `has_trajectory_tail(text: &str) -> bool` (LENGTH=4 pattern check); section extractors (`extract_frontmatter_current_step`, `extract_last_updated_cell`, `extract_phase_progress_latest_row`, `extract_concurrent_cycles_latest_row`, `extract_session_resume_section_1`, `extract_burst_log_latest_dim7`, `extract_current_cycle`); `extract_per_pass_trajectory_flag(index_md_content: &str) -> bool` (Precondition 7 cycle-type gate; ADR-023); `check_state_md(content: &str, per_pass_trajectory: bool) -> Vec<MissingStateSite>` (cascade accumulator — takes the cycle-type flag so PC3/PC4/PC5 are added to the cascade ONLY when `per_pass_trajectory == true`; PC1/PC2 always added on missing tail); `check_index_sites`, `check_burst_log_sites`, `check_lessons_sites` (advisory checkers); `on_post_tool_use(payload: HookPayload) -> HookResult` (effectful orchestration entry point — for the STATE.md arm, resolves `per_pass_trajectory` via secondary `host::read_file` on the active cycle's INDEX.md with fail-open-to-advisory before invoking `check_state_md`)
- **`extract_last_updated_cell(content: &str) -> Option<String>` (PC2 extractor; F-SP7-005 explicit spec):** Scans the body for the markdown table row pattern `| **Last Updated** |` and captures the cell value in the second pipe-delimited column (text between 2nd and 3rd unescaped `|`; strip leading/trailing whitespace via `trim()`; join continuation lines on whitespace per F-SP4-013). Returns `Some(cell_value)` if the row is found; `None` if the `| **Last Updated** |` row is absent (absent row = missing site → Block via PC6). **NOTE (F-SP5-001/F-SP5-008):** PC2 REQUIRES the `trajectory-tail ` marker form in the Last Updated cell. State-manager Commit E dispatch templates were updated at D-518 to include the marker; production discipline satisfies PC2 STRICT marker-prefix two-step check. The marker MAY follow other prose in the cell.
- **`extract_phase_progress_latest_row(content: &str) -> Option<String>` (PC3 extractor; F-SP4-001 ONE-tail-per-region):** Scans for the `^## Phase Progress` heading (stable anchor per TD-VSDD-091), captures the TABLE ROWS that follow until the next `##` heading, then takes the BOTTOMMOST row whose Status cell does NOT contain "ARCHIVED" or "COMPACTED". Returns `Some(single_row_text)` — the text between the `|` delimiters of that bottommost row only, NOT the entire Phase Progress section. Returns `None` if no qualifying row exists (all rows archived/compacted → treat as present/pass-through). The `count == 4` check (inv-4) operates on this single-row text. Do NOT skip rows with Status "COMPLETE", "SHIPPED", "MERGED", or "CYCLE CLOSED".
- **`extract_concurrent_cycles_latest_row(content: &str) -> Option<String>` (PC4 extractor; F-SP5-002 single-row tightening):** Scans for the `^## Concurrent Cycles` heading (stable anchor per TD-VSDD-091), captures the TABLE ROWS until the next `^## ` heading, then takes the BOTTOMMOST table data row (after stripping header and separator rows) whose Status cell does NOT contain "CLOSED", "COMPACTED", or "ARCHIVED". Returns `Some(single_row_text)` — the text between the `|` delimiters of the bottommost active/in-progress row only, NOT the entire Concurrent Cycles section. Returns `None` if no qualifying row exists (all rows closed/compacted/archived → treat as present/pass-through). The two-step inv-4 marker-prefix check operates on this single-row text.
- **`extract_session_resume_section_1(content: &str) -> Option<String>` (PC5 extractor; F-SP7-005 explicit spec):** Scans for the `## Session Resume Checkpoint` heading (prefix-match; tolerates optional parenthetical suffix per F-SP1-004 production evidence), then locates the first `### §1.` sub-heading within that section, and captures the content from that sub-heading up to (but not including) the next `### ` heading or `## ` heading. Returns `Some(section_text)` if both headings are found; `None` if either is absent (absent section = missing site → Block via PC6). The two-step inv-4 marker-prefix check operates on this section text.
- **`extract_current_cycle(state_md_content: &str) -> Option<String>` (F-SP4-005 — extractor spec):** Scans the YAML frontmatter region (between first and second `---\n` delimiters) for a line matching `^current_cycle:` and extracts the value. MUST handle all production YAML forms: (a) bare form: `current_cycle: v1.0-brownfield-backfill` → extract `v1.0-brownfield-backfill`; (b) single-quoted: `current_cycle: 'v1.0-brownfield-backfill'` → strip quotes; (c) double-quoted: `current_cycle: "v1.0-brownfield-backfill"` → strip quotes; (d) trailing comment: `current_cycle: v1.0-brownfield-backfill # active` → strip `#...` suffix and trim; (e) multi-line block-scalar using `|` or `>` → read continuation lines as the value. Returns `None` if the key is absent or the frontmatter region cannot be located. On `None`, the INDEX.md arm falls back to fail-open (`HookResult::Continue` + `host::log_warn`) — parallel to EC-017 `current_step:` extraction handling. Production STATE.md uses bare form (confirmed by §SDK Grounding Evidence Grep 4: `current_cycle: v1.0-brownfield-backfill`).
- **`extract_per_pass_trajectory_flag(index_md_content: &str) -> bool` (Precondition 7 extractor; ADR-023 Option (c)):** Scans the active cycle's INDEX.md YAML frontmatter region (between first and second `---\n` delimiters) for a line matching `^per_pass_trajectory:` and parses the YAML boolean value. Returns `true` iff the value parses to boolean `true` (accepting `true`/`True`/`TRUE` case-insensitive; trailing `# comment` stripped and remainder trimmed before parse). Returns `false` for any other value (`false`, `no`, `0`, empty, non-boolean string) AND when the `per_pass_trajectory:` key is ABSENT (milestone cycles omit it by convention). Note the return type is `bool`, NOT `Option<String>` — it folds "absent / unparseable / false" all to `false` (the safe milestone default). The CALLER (Precondition 7 routing) is responsible for the fail-open-to-advisory behavior on upstream read failure: if the secondary `host::read_file` on `.factory/cycles/<active_cycle>/INDEX.md` returns any `HostError`, or the bytes fail `String::from_utf8` decode (inv-13), or `extract_current_cycle()` returned `None`, the caller treats the cycle as NOT per-pass (flag = `false` → PC3/PC4/PC5 ADVISORY) and emits `host::log_warn`. This guarantees fail-open-to-advisory, NEVER fail-open-to-Block (Precondition 7 Step 4). The INDEX.md path is constructed from the dynamically-resolved active cycle name (`extract_current_cycle()`), NOT hardcoded — reuses the existing ADR-018 cycle-resolution machinery ("wiring not redesign"). The secondary read uses `max_bytes = 524288` per inv-7. The flag is currently ABSENT on all cycle INDEX.md files (state-manager authoring obligation per ADR-023 §3 — see §SDK Grounding Evidence Grep 11).
- **Cycle-type secondary-read for the STATE.md arm (ADR-023):** The STATE.md Block arm performs a secondary `host::read_file` on `.factory/cycles/<active_cycle>/INDEX.md` ONLY to resolve the `per_pass_trajectory` flag for PC3/PC4/PC5 routing. This is a DISTINCT read from (1) the primary STATE.md write-target validation (PC1-PC5 site extraction) and (2) the INDEX.md-arm cycle-name lookup (Precondition 4). All three reads use `max_bytes = 524288`. PC1/PC2 are evaluated and Block-routed BEFORE/INDEPENDENT of the cycle-type read — so a cycle-type read failure can never affect PC1/PC2 (which always Block). The cycle-type read result affects ONLY the PC3/PC4/PC5 Block-vs-advisory routing.
- **INDEX.md cycle-path guard — path-component-walk MANDATORY (F-SP4-006):** After extracting `active_cycle` via `extract_current_cycle`, the INDEX.md arm MUST use path-component-walk to check whether `file_path` belongs to that cycle: `Path::new(file_path).components().any(|c| c.as_os_str() == active_cycle.as_str())`. Using `file_path.contains(active_cycle.as_str())` (substring `String::contains`) is FORBIDDEN — it produces false-positives when cycle names share a prefix (e.g., `v1.0-brownfield-backfill` is a substring of `v1.0-brownfield-backfill-bolt-on/INDEX.md`). The path-component-walk form is platform-independent and resistant to path normalization edge cases (same principle as the `.factory` parent-guard in Precondition 4).
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook; `host::log_warn(message)` for advisory-level non-blocking log entries
- `crates/hook-sdk/src/result.rs` — `HookResult` enum: `Continue`, `Block { reason }`, `Error { message }`; `HookResult::block_with_fix(hook, reason, recommendation, code)` constructor; NOTE: NO `HookResult::Advisory` variant exists
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration: priority 158 (next monotonic after validate-policies-schema at 157); `tool = "Edit|Write"`; `path_allow = [".factory"]`; `on_error = "continue"` (inv-12); closest `path_allow = [".factory"]`-only structural sibling is BC-5.39.006 (validate-dispatch-advance) — NOT BC-5.39.008 which uses `path_allow = [".factory", "plugins/vsdd-factory"]`
- `.factory/STATE.md` — primary target artifact (5 prescribed sites; Block arm)
- `.factory/STATE.md` `current_cycle:` frontmatter field — runtime source for active cycle name; read dynamically via secondary `host::read_file` call (F-SP3-001 dynamic resolution); NOT hardcoded to any specific cycle name; resolved from STATE.md `current_cycle:` at runtime via `extract_current_cycle()` (F-SP4-012). Current production value `v1.0-brownfield-backfill` confirmed by §SDK Grounding Evidence Grep 4 — but this value changes across cycles and MUST be read dynamically.
- `.factory/cycles/<active-cycle>/INDEX.md` — DUAL ROLE: (1) target artifact for the INDEX.md arm (2 prescribed sites; Advisory arm; cycle-path-guarded dynamically via path-component-walk per F-SP4-006); (2) cycle-type source for the STATE.md arm — its frontmatter `per_pass_trajectory` flag is read via secondary `host::read_file` to route PC3/PC4/PC5 Block-vs-advisory (Precondition 7; ADR-023). State-manager authors `per_pass_trajectory: true` on F5-cycle INDEX.md per ADR-023 §3 (currently absent — see §SDK Grounding Evidence Grep 11).
- `.factory/cycles/<active-cycle>/burst-log.md` — target artifact (1 prescribed site; Advisory arm)
- `.factory/cycles/<active-cycle>/lessons.md` — target artifact (1 prescribed site; Advisory arm)
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
   enforcement (inv-6(b) + EC-007). BC-5.39.009 adopts the FULL BC-5.39.006 v1.7 inv-6(b)
   marker-prefix + LENGTH-equality discipline across all 5 STATE.md sites (PC1/PC2/PC4/PC5).

   **Point 2 (revised pass-5 F-SP5-001 HUMAN-DIRECTED PARTIAL REVERSAL):** BC-5.39.009 inv-4 adopts the FULL BC-5.39.006 v1.7 inv-6(b) marker-prefix + LENGTH-equality discipline (NOT just the LENGTH=4 invariant). The original v1.0–v1.4 "deliberate non-extension" rationale (heterogeneous text cells unsuited to canonical marker discipline) was REVERSED at pass-5 by human direction: empirical evidence at F-SP5-001 CRITICAL demonstrated that LENGTH=4 STRICT applied to whole-cell text would false-Block legitimate state-manager Commit E writes (production current_step contains 8+ arrows when carry-across is included). The two-step marker-prefix check (Step 1: locate `trajectory-tail ` marker; Step 2: count `→(\d+)` within marker-segment to first `;`) per BC-5.39.006 v1.7 inv-6(b) precedent is now applied to ALL 5 STATE.md sites (PC1/PC2/PC4/PC5). This is a state-manager dispatch template forward requirement: Last Updated cell + current_step + Phase Progress latest row + Concurrent Cycles latest row + Session Resume §1 body MUST include the `trajectory-tail →N→N→N→N` marker segment.

Both predecessor cure-extensions cited in Traceability §Predecessor Cure-Extensions.

3. **F-SP3-003 HostError variant collapse rationale (F-SP4-007 direction corrected):** The former
   PC11 (`HostError::TooBig`) was an incorrect SDK cite — the actual variant is
   `HostError::OutputTooLarge` per host.rs enum. Rather than replace the variant name in a
   structurally separate PC (which would be fragile to further SDK evolution), this BC collapses
   the old PC11 (HostError::TooBig-specific) into the new PC11 (uniform HostError fail-open) per
   the adversary recommendation. The old PC13 (all-sites-present pass case) becomes the new PC12.
   Net: PC count 13 → 12. This reduces structural complexity by eliminating a special-case PC
   that inv-10 already covers uniformly. The `max_bytes = 524288` invariant (inv-7) remains
   unchanged — if a file exceeds the cap, `HostError::OutputTooLarge` is returned and handled
   uniformly by the new PC11. See §SDK Grounding Evidence for literal-shell confirmation of the
   actual variant name.

4. **ADR-023 cycle-conditional site model (v1.9; HUMAN-AUTHORIZED un-SEAL).** The v1.9 re-spec does
   NOT introduce a new INV-NNN abstraction for cycle-type gating. It EXTENDS the existing dynamic
   cycle-resolution machinery already present in this BC since v1.3 (F-SP3-001: the INDEX.md arm
   reads STATE.md `current_cycle:` via `extract_current_cycle()` + a secondary `host::read_file`).
   The cycle-type gate (Precondition 7) reuses exactly that resolution path, adding one new
   extractor (`extract_per_pass_trajectory_flag`, returning `bool`) and one additional secondary
   read of `.factory/cycles/<active_cycle>/INDEX.md` for the STATE.md arm — "wiring not redesign"
   per the production-grade default. inv-4 LENGTH=4-STRICT marker-prefix scoping is PRESERVED
   VERBATIM (the re-spec changes the SEVERITY ROUTING of PC3/PC4/PC5, NOT the trajectory_tail
   check itself). The discriminator is an explicit per-cycle frontmatter flag (NOT a hardcoded
   cycle-name match — which would repeat the F-SP3-001 hardcoding defect the v1.2→v1.3 fix
   eliminated). Fail-open-to-advisory (never fail-open-to-Block) preserves the inv-10 fail-open
   discipline while preventing the v1.8 pipeline-brick. Per ADR-023, Option (a) (retro-fabricating
   tails onto milestone rows to satisfy the v1.8 unconditional check) was REJECTED as
   paper-compliance (SOUL.md #4 / production-grade-default violation); Option (c) (this
   cycle-conditional model) is the only option correct under both the F5-degradation reading AND
   the milestone-cycle reading without writing meaningless data.

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.17 post-merge burst.

## SDK Grounding Evidence

**SDK-GROUNDING MANDATE (pass-3 root-cause discipline; POLICY 5 v1.3.1 stable-anchor extension; POLICY 5 v1.3.4 literal-shell verification gate):** Every SDK symbol, file path, cycle name, type signature, registry priority, or constant cited in this BC is backed by literal-shell grep of the actual source file. POLICY 15 verbatim stdout discipline applied. STABLE ANCHORS ONLY — no line numbers in captured stdout per TD-VSDD-091 + POLICY 5 v1.3.1 sub-clause (F-SP4-002 META-LEVEL-32 cure-extension).

**Grep 1 — HostError enum variants: literal enum body (crates/hook-sdk/src/host.rs) — F-SP4-010 fix**
```
$ grep "^pub enum HostError" crates/hook-sdk/src/host.rs
pub enum HostError {
$ grep -A 20 "^pub enum HostError" crates/hook-sdk/src/host.rs
pub enum HostError {
    /// The caller does not have the capability for this operation.
    CapabilityDenied,
    /// The host call exceeded its `timeout_ms` budget.
    Timeout,
    /// The output exceeded the per-call cap; truncated.
    OutputTooLarge,
    /// The argument failed host-side validation (path traversal, etc.).
    InvalidArgument,
    /// The host operation failed for a reason not classified above.
    /// `code` is the negative error number returned by the host.
    Other(i32),
}
```
Confirmed: enum body shown with all 5 variants (`CapabilityDenied`, `Timeout`, `OutputTooLarge`, `InvalidArgument`, `Other(i32)`); closing brace immediately follows the `Other(i32)` variant. No `TooBig` variant exists. `OutputTooLarge` is the correct variant (F-SP3-003). Variant names are stable structural anchors.

**Grep 2 — `read_file` signature (crates/hook-sdk/src/host.rs)**
```
$ grep "^pub fn read_file" crates/hook-sdk/src/host.rs
pub fn read_file(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError> {
```
Confirmed: `max_bytes: u32`, `timeout_ms: u32`, returns `Result<Vec<u8>, HostError>`.

**Grep 3 — Sibling double-match pattern (crates/hook-plugins/validate-policies-schema/src/lib.rs)**
```
$ grep "Ok(bytes)" crates/hook-plugins/validate-policies-schema/src/lib.rs
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(bytes) => match String::from_utf8(bytes) {
            Ok(bytes) => match String::from_utf8(bytes) {
```
Confirmed: double-match `Ok(bytes) => match String::from_utf8(bytes)` pattern at 3 call sites.
T-5 is missing this decode step (F-SP3-006 — routes to story-writer).

**Grep 4 — STATE.md `current_cycle:` bare form + `## Phase Progress` heading + Dim-7 heading variants (F-SP4-002/F-SP4-004/F-SP4-005)**
```
$ grep "^current_cycle:" .factory/STATE.md
current_cycle: v1.0-brownfield-backfill
$ grep "^## Phase Progress" .factory/STATE.md
## Phase Progress
$ grep "^### Dim-7" .factory/cycles/v1.0-brownfield-backfill/burst-log.md | sort -u
### Dim-7 attestation
### Dim-7 Attestation
### Dim-7 Attestation (Closes / Advances)
### Dim-7 Attestation / Closes
```
Confirmed: `current_cycle:` bare form (no quotes, no trailing comment) — extractor handles bare, single-quoted, double-quoted forms. `## Phase Progress` heading present (stable anchor `^## Phase Progress`; no line number per TD-VSDD-091 F-SP4-002). Dim-7 actual headings use prefix `### Dim-7` with various suffixes — PC9 extractor uses `^### Dim-7` prefix-match (F-SP4-004). F5 cycle burst-log has ZERO `### Dim-7` lines (zero output); active cycle `v1.0-brownfield-backfill` has 4 heading variants, all sharing `^### Dim-7` prefix.

**Grep 5 — Brownfield-backfill INDEX.md exists**
```
$ ls .factory/cycles/v1.0-brownfield-backfill/INDEX.md
.factory/cycles/v1.0-brownfield-backfill/INDEX.md
```
Confirmed: active cycle has INDEX.md artifact that would be missed by v1.2 hardcoded F5 guard.

**Grep 6 — F5 cycle paused**
```
$ grep "^paused_pending_resume:" .factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md
paused_pending_resume: true
```
Confirmed: F5 cycle has `paused_pending_resume: true`. F-SP3-001 CRITICAL confirmed.

**Grep 7 — Priority 158 uncollided in hooks-registry.toml**
```
$ grep "priority = 15[6-8]" plugins/vsdd-factory/hooks-registry.toml
priority = 156
priority = 157
```
Confirmed: priority 157 is the highest PostToolUse Edit|Write priority (`validate-policies-schema`).
Priority 158 is uncollided — next available for `validate-trajectory-tail-cell-completeness`.

**Grep 8 — ARCH-INDEX SS-05 = "Pipeline Orchestration"**
```
$ grep "SS-05 Pipeline Orchestration" .factory/specs/architecture/ARCH-INDEX.md
| SS-05 Pipeline Orchestration | SS-05-orchestration.md | orchestrator, story-writer | Agents, Lobster workflows, pipeline phase structure |
| SS-05 | Pipeline Orchestration | SS-05-orchestration.md | ...
```
Confirmed: SS-05 = "Pipeline Orchestration" in ARCH-INDEX Subsystem Registry.

**Grep 9 — BC-5.39.006 inv-6(b) equality semantics (count == 4 precedent)**
```
$ grep "count must equal exactly 4" .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
   narratives, TD-NNN renumber references). The match count must equal exactly 4. A count of 3
```
Confirmed: BC-5.39.006 invariant 6(b) requires `count == 4` (equality), not `count >= 4`. This
is the F-SP3-002 cure-extension precedent per D-497. BC-5.39.009 v1.7 adopts identical equality
semantics in inv-4.

**Grep 10 — STRUCTURAL FORM: `trajectory-tail` marker presence in production STATE.md (POLICY 5 v1.3.6 HEAD-reproducible structural invariant; F-SP8-006 META-LEVEL-36 self-apply cure)**

The inv-4 marker-prefix two-step check requires the `trajectory-tail ` marker (with trailing space) to be present in PC1/PC2/PC4/PC5 site texts. The STRUCTURAL INVARIANT (reproducible at any HEAD) is:

```
$ grep -c "trajectory-tail" /Users/jmagady/Dev/vsdd-factory/.factory/STATE.md
16

$ grep "trajectory-tail" /Users/jmagady/Dev/vsdd-factory/.factory/STATE.md | grep -c "→[0-9]\+→[0-9]\+→[0-9]\+→[0-9]\+"
5
```

The exact arrow values evolve monotonically per cycle (D-518 cycle = →9→9→9→11; D-520 cycle = →9→9→9→11 carry-across). The STRUCTURAL FORM (marker + 4-arrow-segment) is the invariant; specific values are not. Adversary fresh-context loop executes these structural greps at HEAD; both return non-zero across all cycles where state-manager Commit E template adheres to the forward requirement. These results are HEAD-reproducible per POLICY 5 v1.3.6 Part B (structural-form invariants survive monotonic content evolution and remain verifiable across cycle SHAs). Captured at parent-commit `dfcbea39` (pass-8 fix-burst) — re-execute at any HEAD against the latest STATE.md and expect non-zero for both predicates.

**Grep 11 — STRUCTURAL FORM: `per_pass_trajectory` cycle-type flag presence + active cycle milestone-form (ADR-023 Option (c) cycle-type gate; v1.9 re-spec; POLICY 5 v1.3.6 HEAD-reproducible structural invariant)**

Precondition 7 routes PC3/PC4/PC5 on the active cycle's `per_pass_trajectory` INDEX.md frontmatter flag. The STRUCTURAL INVARIANTS (reproducible at any HEAD) are:

```
$ grep '^current_cycle:' /Users/jmagady/Dev/vsdd-factory/.factory/STATE.md
current_cycle: v1.0-brownfield-backfill

$ grep -rl 'per_pass_trajectory: true' /Users/jmagady/Dev/vsdd-factory/.factory/cycles/*/INDEX.md
(no output — exit 1)

$ grep '^paused_pending_resume:' /Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md
paused_pending_resume: true

$ grep -c '^document_type: cycle-index' /Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md
1
```

Confirmed: (a) the active cycle is `v1.0-brownfield-backfill` — a MILESTONE/story-delivery cycle (bare YAML form). (b) The `per_pass_trajectory: true` flag is currently ABSENT on ALL cycle INDEX.md files (grep returns no matching file; exit 1) — therefore under Precondition 7 ALL cycles currently resolve to flag=FALSE → PC3/PC4/PC5 ADVISORY. This is the production-grade safe default: the hook does NOT Block on per-pass sites until state-manager explicitly authors `per_pass_trajectory: true` on the F5-cycle INDEX.md (`v1.0-feature-engine-discipline-pass-1/INDEX.md`) per ADR-023 §3 — a state-manager authoring obligation tracked in this BC's Changelog. (c) The F5 cycle INDEX.md exists (`document_type: cycle-index`, `paused_pending_resume: true`) and is the intended home for the flag. STRUCTURAL FORM (flag-presence predicate + active-cycle-name extraction) is the invariant; specific cycle names evolve. Adversary fresh-context loop executes these structural greps at HEAD — re-execute at any HEAD; once state-manager authors the flag, the `grep -rl 'per_pass_trajectory: true'` predicate returns the F5 INDEX.md path.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.9 | 2026-05-30 | **HUMAN-AUTHORIZED un-SEAL + ADR-023 Option (c) cycle-conditional STATE.md Block-arm site model (product-owner).** HUMAN AUTHORIZATION: human explicitly authorized un-SEALing BC-5.39.009 (SEALED at D-522 as v1.8 per D-386 Option C asymptotic acceptance) to apply ADR-023 (`ADR-023-trajectory-tail-cell-completeness-cycle-conditional-site-model.md`; decision: Option (c)). DEFECT FIXED (ADR-023 root-cause-2): the v1.8 model Blocked unconditionally on all 5 STATE.md sites; S-15.17 adv-local-pass-5 was CLEAN/3-CLEAN against v1.8, but shipping it would brick every state-manager STATE.md write in milestone cycles (live `current_cycle: v1.0-brownfield-backfill`, whose Phase Progress / Concurrent Cycles / Session Resume §1 rows are milestone-form and structurally carry no per-pass trajectory — NORMAL, not degradation). RE-SPEC: (1) PC1 (current_step) + PC2 (Last Updated) → ALWAYS-Block (cycle-invariant; severity UNCHANGED). (2) PC3/PC4/PC5 → CYCLE-CONDITIONAL: Block ONLY when active cycle is F5-style per-pass (detected via `per_pass_trajectory: true` flag on active cycle's INDEX.md, resolved at runtime: STATE.md `current_cycle:` → cycle INDEX.md frontmatter → check flag); else ADVISORY (log_warn + Continue). FAIL-OPEN-TO-ADVISORY: any HostError / utf8 / absent INDEX.md / unresolvable flag → default ADVISORY, NEVER Block. (3) NEW Precondition 7 (per_pass_trajectory cycle-type gate; 4-step resolution + fail-open-to-advisory; reuses existing `extract_current_cycle()`/`host::read_file` machinery + new `extract_per_pass_trajectory_flag()` extractor). (4) inv-4 LENGTH=4-STRICT marker-prefix scoping PRESERVED VERBATIM — NOT relaxed (re-spec changes PC3/PC4/PC5 severity ROUTING, not the trajectory_tail check). (5) NEW EC-021 (milestone-cycle tail-less PC3/PC4/PC5 → NO Block, advisory only — the exact live-STATE.md case that would have bricked; real-STATE.md regression fixture mandatory) + NEW EC-022 (LENGTH=4 marker coexisting with LENGTH=5 prose trajectory in same row → PASS; locks inv-4 marker-prefix first-semicolon scoping against the LENGTH=5 prose form). (6) §D-453(d) mapping table sites 3/4/5 Scope/Severity columns updated to cycle-conditional. (7) PC3/PC4/PC5 'Production STATE.md evidence' prose updated: per-row tails exist ONLY in F5 per-pass cycles; milestone-cycle rows are tail-less by convention (Option (a) retro-fabrication REJECTED). (8) §Architecture Anchors: `extract_per_pass_trajectory_flag()` spec + cycle-type secondary-read + lib.rs `check_state_md(content, per_pass_trajectory)` signature + extractor catalog (8th extractor, `bool` return). (9) Description two-arm narrative rewritten to 3-tier cycle-conditional model. (10) §SDK Grounding Evidence Grep 11 added (HEAD-reproducible: per_pass_trajectory flag ABSENT on all cycle INDEX.md → state-manager authoring obligation per ADR-023 §3; current_cycle milestone-form). (11) 5 new pending VP rows (cycle-invariant always-Block / milestone no-Block / F5 per-pass Block / fail-open-to-advisory / LENGTH=5-prose-coexist). (12) ADR-023 added to §ADR References + §D-NNN Anchor Coverage non-D table + frontmatter inputs. (13) Frontmatter: `sealed: false` + `unsealed_from`/`unsealed_at` un-seal tracking; §Adversary Pass Coverage v1.9 entry (STREAK reset 0/3 — material re-spec of SEALED contract VOIDS prior pass-5 CONVERGENCE for the new behavior; fresh 3-CLEAN required per ADR-023 §5 + BC-5.39.001, OR D-386 Option C asymptotic-acceptance). POLICY 14 5-leg quintuple parity applied (version "1.9" + this Changelog row v1.9 + modified[] appended "2026-05-30 (v1.9)" + last_amended text-prefix "2026-05-30 (v1.9)" + BC-INDEX cell bump v1.8→v1.9 — **BC-INDEX bump is state-manager's leg; noted here, NOT edited by PO**). HANDOFF NOTES: (a) STORY-WRITER NEXT BURST — S-15.17 story body/AC propagation for the cycle-conditional model (PC3/PC4/PC5 conditional + new PC7 + EC-021/EC-022) per POLICY 8 bc_array_changes_propagate; PO did NOT touch the story body. (b) STATE-MANAGER NEXT BURST — BC-INDEX v1.8→v1.9 title/version cell bump + author `per_pass_trajectory: true` on F5-cycle INDEX.md (`v1.0-feature-engine-discipline-pass-1/INDEX.md`) and ensure milestone cycles omit/false. (c) ARCHITECT — propagate 5 new VPs to VP-INDEX + verification-architecture.md + verification-coverage-matrix.md (vp_index_is_vp_catalog_source_of_truth). (d) IMPLEMENTER/TEST-WRITER — split STATE.md arm into always-Block (PC1/PC2) + cycle-conditional (PC3/PC4/PC5) paths; real-STATE.md milestone-cycle regression fixture (NO Block) + F5-shaped fixture (Block-on-missing) + LENGTH=5-prose-coexist PASS fixture. |
| 1.8 | 2026-05-29 | Pass-8 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). Closes 6 BC-side PO findings + 1 process-gap META-36 codification: F-SP8-001 CRITICAL (§Cure-Extension Parsimony Note point 2 PAPER-FIX — rewritten to document F-SP5-001 HUMAN-DIRECTED PARTIAL REVERSAL; "deliberate non-extension" narrative removed; full two-step marker-prefix extension per BC-5.39.006 v1.7 inv-6(b) documented); F-SP8-002 HIGH (PC10 body PAPER-FIX — rewritten to OUT-OF-SCOPE annotation per F-SP5-003; §D-453(d) Site 9 row updated from IN SCOPE to OUT-OF-SCOPE (per F-SP5-003; lessons.md no machine-extractable trend-table structure)); F-SP8-006 HIGH [META-36] (Grep 10 snapshot-annotation-rescue-pattern → STRUCTURAL-FORM-only per POLICY 5 v1.3.6 Part B; snapshot stdout removed; structural grep predicates HEAD-reproducible: grep -c 'trajectory-tail' STATE.md → 16; grep ... | grep -c '4-arrow-pattern' → 5); F-SP8-007 MEDIUM (§Architecture Anchors catalog corrected from 'All 4 body extractors' to 'All 7 section extractors'; extract_frontmatter_current_step + extract_burst_log_latest_dim7 Option<String> bullets added); F-SP8-009 MEDIUM (covered by F-SP8-001 point 2 rewrite); F-SP8-PG-001 PROCESS-GAP META-36 CODIFICATION: policies.yaml POLICY 5 v1.3.5→v1.3.6 — Part B HEAD-reproducibility-or-structural-form mandate + Part D snapshot-rescue-pattern detection. POLICY 5 v1.3.6 SELF-APPLIED this burst. §Adversary Pass Coverage Pass-8 entry added. POLICY 14 5-leg quintuple parity applied (version "1.8" + this Changelog row v1.8 + modified[] appended "2026-05-29 (v1.8)" + last_amended text-prefix "2026-05-29 (v1.8)" + BC-INDEX v2.61→v2.62). Story-writer next burst: F-SP8-003/F-SP8-004/F-SP8-005/F-SP8-008/F-SP8-010. |
| 1.7 | 2026-05-29 | Pass-7 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). Closes 6 BC-side PO findings + 1 process-gap META-35 codification: F-SP7-002 HIGH (Grep 1 'all 4 variants'→'all 5 variants' arithmetic fix); F-SP7-004 MEDIUM (Grep 10 annotated with D-518 snapshot label + parent-commit SHA `f189b45b` for adversary-replay-reproducibility per POLICY 5 v1.3.5); F-SP7-005 MEDIUM (§Architecture Anchors extractor return types normalized — all 4 body extractors now Option<String>; rationale note added; individual spec bullets added for extract_last_updated_cell, extract_phase_progress_latest_row, extract_concurrent_cycles_latest_row, extract_session_resume_section_1); F-SP7-006 MEDIUM (PC2 cites extract_last_updated_cell(); PC5 cites extract_session_resume_section_1() — matching PC3/PC4/PC9 style); F-SP7-008 LOW (§Adversary Pass Coverage Pass-6 count reformatted: '11 findings 0C+5H+4M+1L+1N + 1 process-gap'); F-SP7-PG-001 PROCESS-GAP META-35 CODIFICATION: policies.yaml POLICY 5 v1.3.4→v1.3.5 — historical-by-construction enumeration (Part A) + adversary-replay-reproducibility mandate (Part B) + sibling-sweep categories (f)/(g)/(h) extension (Part C, closes F-SP7-003 category gap). POLICY 5 v1.3.5 SELF-APPLIED — verification gates executed with literal-shell stdout captured (parent-commit cited). §Adversary Pass Coverage Pass-7 entry added. Story-writer next burst: F-SP7-001/F-SP7-003/F-SP7-007. POLICY 14 5-leg quintuple parity applied (version "1.7" + this Changelog row v1.7 + modified[] appended "2026-05-29 (v1.7)" + last_amended text-prefix "2026-05-29 (v1.7)" + BC-INDEX v2.60→v2.61). |
| 1.6 | 2026-05-29 | Pass-6 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). Closes 7 PO BC-side findings + 1 process-gap META-34 codification: F-SP6-001 HIGH (Grep 10 added to §SDK Grounding Evidence with production STATE.md `trajectory-tail` occurrences; PC1 + inv-4 body cites updated from 'Grep-A' to 'Grep 10'); F-SP6-002 HIGH [META-33 regression] (§Architecture Anchors extractor names updated: `extract_last_updated_section`→`extract_last_updated_cell`, `extract_concurrent_cycles_section`→`extract_concurrent_cycles_latest_row`, added `extract_burst_log_latest_dim7` + `extract_current_cycle`); F-SP6-004 HIGH (PC2 NOTE updated to post-D-518 production state: Last Updated cell NOW carries `trajectory-tail →9→9→9→11` per Grep 10; coordination requirement satisfied at D-518); F-SP6-005 HIGH (§Adversary Pass Coverage Pass-5 + Pass-6 entries added); F-SP6-006 MEDIUM (Grep 1 narrative line-number anchors removed — TD-VSDD-091; stable-anchor variant-name narrative substituted); F-SP6-007 MEDIUM (§SDK Grounding Evidence header 'this BC v1.4' → 'this BC'); F-SP6-010 LOW (PC1 prose 'two trajectory-tail marker mentions' → precise 'multiple ... marker mentions; first-occurrence semicolon-segment scoping yields count=4 → PASS (see Grep 10)'); F-SP6-PG-001 PROCESS-GAP META-34 CODIFICATION: policies.yaml POLICY 5 v1.3.3→v1.3.4 SIBLING-SWEEP LITERAL-SHELL VERIFICATION GATE codified (sweep claims without literal-shell stdout MEDIUM-severity; closes META-LEVEL-34 sweep-claim-without-execution; D-519 per D-497 parsimony cure-extension). POLICY 5 v1.3.4 SELF-APPLIED this burst — literal-shell verification gates executed and stdout captured. Story-writer next burst: F-SP6-003/F-SP6-008/F-SP6-009/F-SP6-011 + story-side mirror of F-SP6-002. POLICY 14 5-leg quintuple parity applied (version "1.6" + this Changelog row v1.6 + modified[] appended "2026-05-29 (v1.6)" + last_amended text-prefix "2026-05-29 (v1.6)" + BC-INDEX v2.59→v2.60). |
| 1.5 | 2026-05-28 | Pass-5 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade; SPEC-REDESIGN AUTHORITY granted by orchestrator). HUMAN-DIRECTED PARTIAL REVERSAL of §Cure-Extension Parsimony Note point 2: inv-4 re-specced with marker-prefix discipline extended to all 5 STATE.md sites (PC1/PC2/PC4/PC5). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-5 / (b) extends BC-5.39.006 v1.7 inv-6(b) semicolon-segment-scoping discipline structurally / (c) cure-extension parsimony per D-497 (PARTIAL REVERSAL with rationale, not new INV-NNN abstraction). Closes 7 PO findings: F-SP5-001 CRITICAL (PC1 inv-4 STRICT count==4 impossible on production current_step — two-step marker-prefix check: Step 1 locate `trajectory-tail ` marker; Step 2 count `→(\d+)` only within marker-segment until next `;` or end); F-SP5-002 HIGH (PC4 Concurrent Cycles extractor — extract_concurrent_cycles_latest_row returns SINGLE ROW TEXT of bottommost active row skipping CLOSED/COMPACTED/ARCHIVED; F-SP4-001 PC3-tightening pattern applied); F-SP5-003 HIGH (PC10 lessons.md trend-table OUT-OF-SCOPE — no machine-extractable structure; mirrored to canonical mapping table site-9 OUT-OF-SCOPE; §Cure-Extension Parsimony Note point 4 added); F-SP5-005 HIGH (extract_current_cycle() multi-line block-scalar handling — `\|` literal: continuation indent > base, join \\n; `>` folded: join space; empty value → None; test vector added); F-SP5-006 MEDIUM (encoding gate implicit — inv-13 added: all extractors take `&str` post-decode via String::from_utf8 with EC-020 fail-open); F-SP5-007 MEDIUM (PC9 Dim-7 latest semantics undefined — bottommost ### Dim-7 in file-order; block = heading to next ## or ###; extract_burst_log_latest_dim7 in §Architecture Anchors); F-SP5-008 MEDIUM (same root cause as F-SP5-001; PC2 cured by marker-prefix re-spec; NOTE added: state-manager Commit E templates must include `trajectory-tail` marker in Last Updated cell). META-LEVEL-33 CANDIDATE codified: sibling-sweep-inside-policy-cure (POLICY 5 sub-clause extension v1.3.2→v1.3.3 per D-497 parsimony). POLICY 14 5-leg quintuple parity applied (version "1.5" + this Changelog row v1.5 + modified[] appended "2026-05-28 (v1.5)" + last_amended text-prefix "2026-05-28 (v1.5)" + BC-INDEX v2.58→v2.59). Story-writer next burst handles: F-SP5-004 (T-5 NOTES grep -n strip), F-SP5-009 (BC Table cell v1.3→v1.5), F-SP5-010 (Token Budget row description), F-SP5-011 (BC Table cell PC11/PC12 cite), F-SP5-012 (covered by F-SP5-004 cure). |
| 1.4 | 2026-05-28 | Pass-4 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). INV-019 cure (a)/(b)/(c) discipline extended. POLICY 5 v1.3.1 stable-anchor sub-clause added (META-LEVEL-32 cure-extension per F-SP4-002: all grep captures MUST use stable anchors; line numbers FORBIDDEN). Closes 10 PO findings: F-SP4-001 CRITICAL (count==4 false-Block on multi-row Phase Progress section — PC3 tightened: extract_phase_progress_latest_row returns SINGLE ROW TEXT; ONE-tail-per-extracted-region precondition; Architecture Anchors function name updated); F-SP4-002 HIGH (TD-VSDD-091 anti-volatile-pin inside POLICY 5 cure — stripped ALL line numbers from §SDK Grounding Evidence; PC3 line cite removed; POLICY 5 extended with stable-anchor sub-clause); F-SP4-004 HIGH (PC9 Dim-7 heading absent — actual heading form `^### Dim-7` confirmed by literal-shell grep on brownfield-backfill/burst-log.md; PC9 updated; scoped to brownfield-backfill with rationale; §SDK Grounding Evidence Grep 4 updated); F-SP4-005 HIGH (extract_current_cycle() unspecified — added to §Architecture Anchors: bare/single-quoted/double-quoted/trailing-comment/multi-line YAML forms; §SDK Grounding Evidence Grep 4); F-SP4-006 HIGH (substring String::contains false-positive on overlapping cycle names — §Architecture Anchors mandates Path::new(file_path).components().any(|c| c.as_os_str() == active_cycle.as_str()); Precondition 4 INDEX.md arm updated); F-SP4-007 HIGH (§Cure-Extension Parsimony Note point 3 collapse direction inverted — rewritten: old PC11→new PC11, old PC13→new PC12, net 13→12); F-SP4-009 MEDIUM (EC-020 UTF-8 fail-open mirrored from story — EC-020 added; closes [needs-po] Canonical Principle Rule 3 violation); F-SP4-010 MEDIUM (POLICY 15 violation inside POLICY 5 cure — §SDK Grounding Evidence Grep 1 replaced with literal sed -n '82,94p' stdout; closing brace line 94 confirmed; narrative paraphrase removed); F-SP4-012 MEDIUM (§Architecture Anchors cycle-name examples → structural form: NOT hardcoded to any specific cycle name; resolved from STATE.md current_cycle: at runtime); F-SP4-013 LOW (PC2 extractor whitespace: capture between 2nd and 3rd unescaped `|`; strip leading/trailing whitespace; join continuation lines). §SDK Grounding Evidence Grep 4 updated (current_cycle + Phase Progress + Dim-7 variants); Grep 1 replaced with literal enum body; all `-n` flags stripped per POLICY 5 v1.3.1. EC-020 added (monotonic; no renumbering). POLICY 14 5-leg quintuple parity applied (version "1.4" + this Changelog row v1.4 + modified[] appended "2026-05-28 (v1.4)" + last_amended text-prefix "2026-05-28 (v1.4)" + BC-INDEX v2.57→v2.58). Story-writer next burst: F-SP4-003/008/011/015 + mirrors F-SP4-006. |
| 1.3 | 2026-05-28 | Pass-3 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). SDK-GROUNDING MANDATE satisfied — 9 literal-shell greps per pass-3 root-cause mandate (§SDK Grounding Evidence). INV-019 cure (a)/(b)/(c) extended. Closes 9 PO findings: F-SP3-001 CRITICAL (dynamic STATE.md current_cycle: resolution — Precondition 4 INDEX.md arm rewired; Architecture Anchors `<active-cycle>` placeholders; option (a) per mandate; cycle-name read via secondary host::read_file on STATE.md); F-SP3-002 CRITICAL regression (LENGTH=4 STRICT equality semantics per BC-5.39.006 inv-6(b) precedent — `→(\d+)` count == 4; inv-4 + PC1..PC5 + EC-003 + EC-018 + Test Vectors + LENGTH=5 new TV row); F-SP3-003 HIGH (HostError::OutputTooLarge actual variant confirmed by host.rs grep; PC11 collapsed into PC12 uniform HostError handler; EC-004 + VP + Test Vectors updated; rationale in §Cure-Extension Parsimony Note point 3); F-SP3-005 HIGH (PC6 indirection via inv-8 documented — explicit cross-reference in PC6 cascade narrative; story-writer handles AC-7 trace extension); F-SP3-008 MEDIUM (Precondition 4 + EC-015 + EC-019 STATE.md parent-guard updated to path-component-walk form: `Path::new(file_path).components().any(|c| c.as_os_str() == ".factory")`); F-SP3-009 MEDIUM (dual-cycle attribution in §Description — F5 D-453(d) anchor + brownfield delivery + dynamic runtime resolution); F-SP3-011 LOW (duplicate ADR-018 cite in §D-NNN Anchor Coverage collapsed to single line + parenthetical moved to §ADR References); F-SP3-013 NITPICK (both D-411(a) cites retained — acceptable per adversary guidance); F-SP3-014 PROCESS-GAP HIGH (policies.yaml POLICY 8 amended v1.1→v1.2 — audit-block exclusion mandate; POLICY 8 extension chosen not new POLICY 19). §SDK Grounding Evidence section added. D-NNN table D-454(a) range updated from 1-10 to 1-11 (absorbs PC11 collapse). POLICY 14 5-leg quintuple parity applied (version "1.3" + this Changelog row v1.3 + modified[] appended "2026-05-28 (v1.3)" + last_amended text-prefix "2026-05-28 (v1.3)" + BC-INDEX v2.57 row). Story-writer next burst: F-SP3-004/006/007/010 + mirrors F-001/002/003/005. |
| 1.2 | 2026-05-28 | Pass-2 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-2 / (b) PC2/PC3/PC5 line-number anti-volatile-pin (TD-VSDD-091) — stripped line numbers from grep excerpts, kept prefix/content match only / (c) cure-extension parsimony note updated with deliberate non-extension of BC-5.39.006 marker-prefix semantics documented per D-497. Closes 7-8 of 11 pass-2 findings: F-S15.17-SP2-003 (EC-008 + test vectors + inv-3 Pre-vs-Post disambiguation: `(PC4)` → `(Precondition 4)` where PC4 referred to cycle-path guard Precondition; full EC table swept); F-S15.17-SP2-004 (`status: active` → `status: draft` pre-merge; `lifecycle_status: draft` consistent; POL-14 fires on S-15.17 merge — both fields go active); F-S15.17-SP2-005 (PC2/PC3/PC5 grep excerpts stripped of `grep -n` line numbers; grep commands without `-n` flag; anti-volatile-pin per TD-VSDD-091); F-S15.17-SP2-006 (inv-4 + §Cure-Extension Parsimony Note updated to document deliberate non-extension of BC-5.39.006 marker-prefix `trajectory-tail ` semantics; EC-006/EC-007 context quoted; per-cell heterogeneous text context rationale); F-S15.17-SP2-007 (Precondition 4 expanded to require `.factory/` parent-guard for STATE.md arm in addition to basename check; EC-019 added for non-factory STATE.md → Continue; inv-3 updated; EC-015 revised to reflect new parent-guard semantics); F-S15.17-SP2-008 (PC3 skip-list: `COMPLETE` dropped; skip only ARCHIVED/COMPACTED; bottommost-row rationale documented — state-manager Commit E appends one row per burst so bottommost IS latest); F-S15.17-SP2-009 partial (ADR-021 dropped from §ADR References and D-NNN non-D table note; frontmatter inputs updated; ADR-018 clarified to cover no-subprocess principle; story-writer drops ADR-021 from `anchored_adrs:` frontmatter); F-S15.17-SP2-010 (inv-9 rephrased from SDK-state assertion to behavioral prescription: "MUST NOT use any HookResult::Advisory variant the SDK may add"; inv-6 updated to match); F-S15.17-SP2-011 (D-453 pass-73 cite corrected from pass-74). POLICY 14 5-leg quintuple parity applied (version "1.2" + this Changelog row v1.2 + modified[] appended "2026-05-28 (v1.2)" + last_amended text-prefix "2026-05-28 (v1.2)" + BC-INDEX v2.56 row version cell v1.2). Story-writer next burst handles F-001 (AC-9/10/11/12/17 re-anchor + bidirectional parity check stdout; META-LEVEL-31 mandate) / F-002 (SS-05 narrative) / F-003 story EC-008 mirror / F-007 AC-23 / F-009 anchored_adrs drop. EC-019 added (monotonic, no renumbering). |
| 1.1 | 2026-05-28 | Pass-1 adversary fix-burst (product-owner; brownfield-backfill S-15.17 spec cascade). INV-019 cure (a) anchor ADV-EDP1-P75-HIGH-002 via spec-cascade pass-1 / (b) structural per-cell extractor-anchor specification (PC2/PC3/PC5 rewritten with production STATE.md structure evidence) / (c) cure-extension of BC-5.39.005+BC-5.39.006 pattern per D-497 parsimony. Closes 9 of 14 pass-1 findings: F-S15.17-SP1-002 (ADR-017 frontmatter path corrected to `ADR-017-per-story-adversary-phasing.md`); F-S15.17-SP1-004 (PC2 Last Updated as table cell in `## Project Metadata`, PC3 Phase Progress latest non-archived row, PC5 `## Session Resume Checkpoint` prefix-match with optional parenthetical, all with literal-shell evidence from factory-artifacts HEAD `29d08cc7`); F-S15.17-SP1-005 (inv-4 + PC6 LENGTH=4 STRICT adjudication — LENGTH=5+ also violation; EC-018 LENGTH=5 added; aligns with BC-5.39.006 inv-6(b)+EC-007+D-433(e)+D-439(c)); F-S15.17-SP1-007 (PC5/EC-016 fail-open contradiction resolved to log_warn+Continue); F-S15.17-SP1-009 (path_allow sibling cite corrected to BC-5.39.006); F-S15.17-SP1-010 (inv-12 on_error=continue added per sibling BC-5.39.004/005/006/007/008 precedent); F-S15.17-SP1-011 (D-NNN table purified — ADR/META-LEVEL/BC rows removed from table, moved to §ADR References / §Related BCs / inv-7); F-S15.17-SP1-012 (D-454(a) PC range 1-9→1-10); F-S15.17-SP1-014 (EC-017 "State.md"→"STATE.md"). POLICY 14 5-leg quintuple parity applied (version "1.1" + this Changelog row v1.1 + modified[] appended "2026-05-28 (v1.1)" + last_amended text-prefix "2026-05-28 (v1.1)" + BC-INDEX v2.55 row version cell v1.1). §Cure-Extension Parsimony Note section added. Story-writer next burst handles F-001/003/006/008/013. |
| 1.0 | 2026-05-28 | Initial authoring (product-owner; brownfield-backfill F5 pass-75 HIGH-002 anchor). Anchors ADV-EDP1-P75-HIGH-002. META-LEVEL-30 route (b) cure (INV-019 cure (a)/(b)/(c)): (a) Anchor identification — closes ADV-EDP1-P75-HIGH-002 (codified-without-runtime-gate-permits-silent-degradation-over-time; D-453(d) canonical 9-site mapping had no WASM enforcement gate); (b) Cure scope — structural (multi-site PostToolUse gate at write time on all 9 D-453(d) mechanically-checkable sites), NOT codification-only; (c) Cure-extension-parsimony evaluation per D-497 — this BC EXTENDS the BC-5.39.005 + BC-5.39.006 structural-gate pattern (STATE.md PostToolUse validation from BC-5.39.005; trajectory_tail enforcement from BC-5.39.006) rather than introducing a new INV-NNN abstraction; both predecessor cure-extensions cited in Traceability §Predecessor Cure-Extensions. BC-5.39.009 allocated as next monotonic ID after BC-5.39.008 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.17 merge). Priority 158 (next available PostToolUse slot per hooks-registry.toml audit: 155=validate-stable-anchors PreToolUse; 156=validate-closes-completeness PostToolUse; 157=validate-policies-schema PostToolUse; 158=this hook, uncollided). POLICY 14 5-leg quintuple parity applied (version "1.0" + this Changelog row v1.0 + modified: ["2026-05-28"] + last_amended text-prefix "2026-05-28 (v1.0)" + BC-INDEX upstream v2.54 row with version cell v1.0). |
