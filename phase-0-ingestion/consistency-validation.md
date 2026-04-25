---
document_type: consistency-validation
level: ops
version: "1.0"
status: complete
producer: consistency-validator
timestamp: 2026-04-25T00:00:00Z
traces_to: "pass-8-final-synthesis.md"
---

# Phase 0 Consistency Validation

**Validator:** consistency-validator (fresh context)
**Date:** 2026-04-25
**Scope:** Phase 0 brownfield ingestion outputs (18 files in `.factory/phase-0-ingestion/`)
**Result:** NEEDS_REVISION

---

## Cross-document consistency matrix

### 1. Numerical consistency — Rust workspace

| Metric | pass-0 | pass-8 | extraction-validation (authoritative) | Consistent? |
|--------|--------|--------|---------------------------------------|-------------|
| Rust LOC | 10,226 | 10,226 | 10,226 (verified) | YES |
| Rust files | 41 | 41 | 41 (verified) | YES |
| Rust tests | 180 | 185 (§1), ~180 (§7 NFR bullet) | 185 (authoritative recounted) | **NO — inconsistency within pass-8** |
| Per-crate LOC claims | ±1 off (systematic) | ±1 off (systematic) | Documented; cosmetic | KNOWN DELTA |

### 2. Numerical consistency — Plugin layer

| Metric | pass-0 | pass-6 | pass-8 | extraction-validation | Consistent? |
|--------|--------|--------|--------|----------------------|-------------|
| Skills (SKILL.md) | 119 | 119 | 119 | 119 (verified) | YES |
| Agents (top-level identities) | 34 | 34 | 34 | 34 (verified) | YES |
| Hooks (top-level .sh) | 44 | 44 | 44 | 44 (verified) | YES |
| hooks-registry entries | 45 | 45 | 45 | 45 (verified) | YES |
| Workflow .lobster files | 16 | 16 | 16 | 16 (verified) | YES |
| Templates (top-level) | 108 | 108 | 105 | 105 (authoritative) | **NO — pass-0 and pass-6 say 108; pass-8 corrects to 105** |
| Templates (incl. subdirs) | 135 (noted) | — | 135 | 135 (verified) | YES |
| Bin tools | 12 | — | 12 | 12 (verified) | YES |
| Rules files | 9 | — | 9 | 9 (verified) | YES |
| Slash commands | 110 | — | 110 | 110 (verified) | YES |
| docs/guide files | 30 | — | — | 28 (actual) | **NO — pass-0 says 30; actual is 28; pass-8 not corrected** |

### 3. Total BC count

| Metric | pass-8 declared | Per-file heading count | Arithmetic sum | Consistent? |
|--------|-----------------|------------------------|----------------|-------------|
| Total BCs | 1,851 | Verified per heading extraction | 86+57+161+197+195+171+176+445+273+90 = 1,851 | YES |
| Per-file BC counts | All 10 match | Heading counts confirmed | Exact match | YES |

### 4. Story counts

| Metric | pass-6 | pass-8 | Consistent? |
|--------|--------|--------|-------------|
| SHIPPED stories | 22 | 22 | YES |
| PARTIAL stories | 4 | 4 | YES |
| NOT SHIPPED stories | 15 | 15 | YES |
| Total stories (excl. EPIC) | 41 | 41 | YES |
| Total files (incl. EPIC) | 42 | 42 | YES |
| SHIPPED+PARTIAL+NOT SHIPPED arithmetic | 22+4+15=41 | 22+4+15=41 | YES |

### 5. Drift findings

| Drift ID | pass-6 title | pass-8 title | Severity change? | Consistent? |
|----------|-------------|-------------|-----------------|-------------|
| DRIFT-001 | read_file at StoreData-typed linker is CAPABILITY_DENIED stub | read_file at StoreData linker is CAPABILITY_DENIED stub | MEDIUM → P1(medium) | YES (relabeled, same meaning) |
| DRIFT-002 | internal.sink_* events declared but never emitted | same | MEDIUM → P1(medium) | YES |
| DRIFT-003 | Per-sink dedicated thread | same | LOW → P2(low) | YES |
| DRIFT-004 | Two parallel hook-routing tables | same | MEDIUM → P1(medium) | YES |
| DRIFT-005 | HTTP/Datadog/Honeycomb sinks not implemented | same | LOW → P2(low) | YES |
| DRIFT-006 | Phase 5 events not wired (title abbreviated in pass-8) | Phase 5 events not yet wired to plugins | LOW → P2(low) | YES (abbreviated title; substance unchanged) |
| DRIFT-007 | dispatcher.shutting_down not emitted | same | LOW → P3(cosmetic) | YES |
| DRIFT-008 | plugin.loaded/plugin.load_failed not emitted | same | LOW → P3(cosmetic) | YES |
| DRIFT-009 | Adversary SHA-currency gate is opt-in | same | LOW → P2(low) | YES |
| DRIFT-010 | 26 unported bash hooks block Windows | same | MEDIUM-HIGH/Windows → P0/Windows,P1 overall | YES (same substance; pass-8 adds P0-for-Windows) |

All 10 drift items consistent across pass-6, pass-8, and extraction-validation. Substance unchanged; pass-8 reformatted severity to Px notation.

---

## Findings

### IMPORTANT (should fix before Phase 1)

---

#### FINDING-I-01 — pass-8 §7 NFR summary still cites 180 Rust tests (should be 185)

**Artifact:** `pass-8-final-synthesis.md` line 266
**Finding:** The Maintainability NFR bullet reads "~180 Rust tests + ~1262 bats". The corrected count is 185, as established by pass-3-deep-rust-tests.md and confirmed by extraction-validation. The executive summary (line 38) and coverage table (line 329) both correctly state 185. The NFR summary bullet was not updated.
**Impact:** A reader of §7 alone will see the uncorrected count. Not a blocking defect (the correct number appears in two other locations in the same document) but it is a residual inconsistency in the synthesis artifact that will feed Phase 1.
**Remediation:** Update pass-8-final-synthesis.md §7 Maintainability bullet to "~185 Rust tests".

---

#### FINDING-I-02 — pass-0-inventory.md and pass-5-conventions.md retain uncorrected hallucination

**Artifacts:**
- `pass-0-inventory.md` line 78: `.factory/specs/` is still listed as the design-doc directory ("`.factory/specs/` — 8 design docs")
- `pass-5-conventions.md` lines 38 and 243: The trybuild hallucination ("crates/hook-sdk-macros/ uses trybuild" and "13 trybuild fixtures") remains uncorrected in the source file

**Finding:** extraction-validation identified these as errors and recommended corrections. pass-8 acknowledges them (line 290 of pass-8 strikes through the "13 trybuild fixtures" claim inline within the conventions summary it reproduces). However:
1. `pass-0-inventory.md` itself still says `.factory/specs/` at line 78. The path was corrected at line 126 (the File Manifest entry for the design doc) but not at line 78 (the "Other notable size/age signals" paragraph). This creates a within-file inconsistency.
2. `pass-5-conventions.md` at line 38 states "crates/hook-sdk-macros/ uses trybuild for compile-fail / pass tests" and at line 243 still lists "13 trybuild fixtures for #[hook] macro". These source files were not updated in place.

**Impact:** Any downstream agent that reads pass-0-inventory.md directly (without reading extraction-validation first) will get the wrong path at line 78 and the trybuild claim at pass-5-conventions.md remains uncorrected. This matters because pass-8 §11 tells downstream skills to use those files as inputs.
**Remediation:**
- `pass-0-inventory.md` line 78: change `.factory/specs/` to `.factory/legacy-design-docs/`
- `pass-5-conventions.md` lines 38 and 243: remove or strike out the trybuild claims; add a correction note

---

#### FINDING-I-03 — extraction-validation header/body sample-count mismatch

**Artifact:** `extraction-validation.md` line 34 vs line 55 and line 260
**Finding:** The document header states "Sample size (Phase 1): 115 (6.2% of catalog)". The per-file table body sums to 125 (verified: 10+6+12+14+14+10+12+25+15+7=125). The summary at line 260 correctly states 125 at 6.8%. The header value (115, 6.2%) is incorrect and was never updated.
**Impact:** A reader skimming the header will see the wrong count. The correct authoritative figure is 125. The 6.2% figure is wrong; 125/1851 = 6.75%.
**Remediation:** Correct extraction-validation.md header to "Sample size (Phase 1): 125 (6.75% of catalog)".

---

#### FINDING-I-04 — pass-8 convergence table claims all BC deepening rounds converged to NITPICK; actual verdicts are FINDINGS_REMAIN

**Artifact:** `pass-8-final-synthesis.md` §9, table row for "Pass 3 — Behavioral Contracts", column "Final state" = NITPICK

**Finding:** Every one of the 9 BC deepening passes ended with verdict FINDINGS_REMAIN, not NITPICK:
- pass-3-behavioral-contracts.md: FINDINGS_REMAIN
- pass-3-behavioral-contracts-deep-r1.md: FINDINGS_REMAIN
- pass-3-deep-skills-batch-1.md: FINDINGS_REMAIN
- pass-3-deep-skills-batch-2.md: FINDINGS_REMAIN
- pass-3-deep-skills-batch-3.md: FINDINGS_REMAIN (explicitly notes 80 of 119 skills still uncovered when this batch ended)
- pass-3-deep-agents.md: FINDINGS_REMAIN
- pass-3-deep-hooks.md: FINDINGS_REMAIN
- pass-3-deep-workflows.md: FINDINGS_REMAIN (explicitly notes ~50 deferred steps)
- pass-3-deep-templates-tools-rules.md: FINDINGS_REMAIN
- pass-3-deep-rust-tests.md: FINDINGS_REMAIN (weak, but explicit)

None of the individual deepening passes declare NITPICK or CONVERGENCE_REACHED. Pass-8 synthesizes them and declares the combined result NITPICK, but this is a synthetic judgment made by the final-synthesis pass itself — it is not supported by individual verdicts.

**Important clarification:** The pass-8 claim that "8 deepening rounds" produced NITPICK is a retrospective evaluation, not a chain of per-pass NITPICK verdicts. Pass-8 is asserting that the combined 1,851 BCs are sufficient for Phase 1 purposes, not that each individual deepening pass ran until it hit NITPICK. This is a legitimate use of the synthesis role — the individual passes had FINDINGS_REMAIN because more work within their *scope* remained, but the aggregate scope is declared sufficient.

**However:** The table format (column header "Final state" per row = NITPICK) can mislead a reader into believing each individual pass validated to NITPICK. This is incorrect.

**Residual concern:** pass-3-deep-skills-batch-3 explicitly states that 80 of 119 skills (those alphabetically before "pr-create") are still missing per-skill BCs when that batch closes. pass-3-deep-workflows.md notes ~50 deferred workflow steps. pass-8 acknowledges slash commands at "LOW-MED" confidence (110 enumerated but not deeply walked). These are genuine coverage gaps, not just unresolved FINDINGS_REMAIN labels.

**Impact:** The convergence declaration at §14 may be overstated. Phase 1 users of this catalog will be missing per-skill BCs for ~67% of the skill layer (80/119 skills) and ~50 workflow steps.

**Remediation:** Amend pass-8 §9 convergence table to:
- Clarify the "Final state: NITPICK" column means the synthesized aggregate judgment, not each individual pass
- Add a caveat row noting known coverage gaps: 80/119 skill per-instance BCs absent, ~50 workflow steps deferred, slash command walk is LOW-MED confidence

---

#### FINDING-I-05 — pass-6 executive summary still uses uncorrected template count (108 instead of 105)

**Artifact:** `pass-6-synthesis.md` line 8
**Finding:** The executive summary states "16 workflows, 108 templates" while extraction-validation recounted 105 top-level template files as authoritative. pass-8 corrects to 105 at line 69 but pass-6 was never updated.
**Impact:** Minor; pass-6 is superseded by pass-8 as the definitive synthesis. A reader who stops at pass-6 will see the wrong count.
**Remediation:** Low priority — pass-6 is effectively superseded by pass-8.

---

#### FINDING-I-06 — pass-8 §11 spec-crystallization recommendations reference secondary design docs with an off-by-one count

**Artifact:** `pass-8-final-synthesis.md` line 413
**Finding:** The text states "7 secondary design docs from 2026-04-13 cover earlier-phase decisions (early-phase-gaps, excalidraw-integration, release-infrastructure, scaffold-claude-md, subagent-driven-gaps, writing-plans-gaps)." That sentence names only 6 documents, but claims 7. Actual file count at `.factory/legacy-design-docs/` shows 7 design docs from 2026-04-13 (the 6 listed plus `2026-04-13-remaining-superpowers-gaps-design.md`). The 7 count is correct, but the enumeration in the sentence omits `remaining-superpowers-gaps-design`.
**Impact:** Downstream agents given this list as an input specification will not know to read `remaining-superpowers-gaps-design.md`.
**Remediation:** Add `remaining-superpowers-gaps-design` to the parenthetical list in pass-8 §11.

---

### NITPICK (cosmetic, defer)

---

#### FINDING-N-01 — BC ID gaps are allocation buffers (not errors), but not explicitly documented in the summary artifact

**Artifact:** `pass-8-final-synthesis.md` (no documentation of ranges)
**Finding:** The following BC ID ranges were reserved but not allocated:
- 144–199: between deep-r1 (ends at 143) and skills-batch-1 (starts at 200). Intentional allocation buffer.
- 361–399: within skills-batch-1's declared range (200–399); batch used 200–360. Reservation documented in pass-3-deep-skills-batch-1.md line 1817 (`bc_range_remaining: BC-AUDIT-361..399`).
- 597–599: between skills-batch-2 (ends at 596) and skills-batch-3 (starts at 600). 3-ID buffer.
- 795–799: between skills-batch-3 (ends at 794) and agents (starts at 800). 5-ID buffer.
- 971–999: between agents (ends at 970) and hooks (starts at 1000). Explicitly reserved in pass-3-deep-agents.md state checkpoint for "cross-agent contract BCs in next round".
- 1176–1299: between hooks (ends at 1175) and workflows (starts at 1300).
- 1795–1799: between workflows (ends with BC-AUDIT-1794-PLN) and templates (starts at 1800).
- 2285–2299: between templates/tools/rules (ends at 2284) and rust-tests (starts at 2300).

**Assessment:** All gaps are intentional allocation buffers. The per-deepening-pass state checkpoints document them. No gap represents a missing BC. The synthesis (pass-8) does not summarize the allocation scheme, which would help Phase 1 BC renumbering.
**Remediation:** Optional — add a BC range registry table to pass-8 §11 for Phase 1 renumbering reference.

---

#### FINDING-N-02 — DRIFT-006 title is abbreviated in pass-8 vs pass-6

**Artifact:** `pass-8-final-synthesis.md` line 229 vs `pass-6-synthesis.md` line 63
**Finding:** pass-6 title: "Phase 5 events (SessionStart / SessionEnd / WorktreeCreate / WorktreeRemove / PostToolUseFailure) not yet wired". pass-8 title: "Phase 5 events not yet wired to plugins". The substance is identical; the abbreviation removes the specific event names.
**Assessment:** Cosmetic. pass-8 body text covers the same events.
**Remediation:** None required.

---

#### FINDING-N-03 — pass-8 as-built ADR numbering inconsistency between §4 and §11

**Artifact:** `pass-8-final-synthesis.md`
**Finding:** Section §4 labels as-built ADRs as ADR-AS-006 through ADR-AS-009. Section §11 (spec-crystallization for architect) promotes the same ADRs as ADR-AS-010 through ADR-AS-013. The numbering shifted by 4 because §11 treats the 5 formal ADRs + 4 Q-resolution ADRs as ADR-001 through ADR-009, then appends as-built at ADR-010. Section §4 uses a different starting index (ADR-AS-006), counting only from the formal 5 ADRs.
**Assessment:** Cosmetic. Both schemas are internally consistent within their own section. The content of each as-built ADR is identical across both sections.
**Remediation:** None required, but a Phase 1 architect should pick one numbering scheme and finalize it.

---

#### FINDING-N-04 — pass-6 convergence section "remaining_gaps_for_deepening: 13 (GAP-A through GAP-M)" already closed by pass-8 but not noted in pass-6

**Artifact:** `pass-6-synthesis.md` §State Checkpoint
**Finding:** pass-6 declares 13 remaining gaps (GAP-A through GAP-M). These gaps were addressed by the 8 deepening rounds that followed. pass-6 was not updated to reflect their closure. This is expected behavior for an append-only ingestion log (passes are not modified once written), but a reader of pass-6 in isolation would see 13 open gaps.
**Assessment:** Expected behavior. pass-8 is the authoritative synthesis and supersedes pass-6.
**Remediation:** None required.

---

## Coverage perimeter audit

### Files analyzed vs. all 18 in scope

All 18 files listed in the audit scope are present:

| File | Size | Status |
|------|------|--------|
| pass-0-inventory.md | 18KB | present |
| pass-1-architecture.md | 24KB | present |
| pass-2-domain-model.md | 32KB | present |
| pass-3-behavioral-contracts.md | 40KB | present |
| pass-3-behavioral-contracts-deep-r1.md | 65KB | present |
| pass-3-deep-skills-batch-1.md | 117KB | present |
| pass-3-deep-skills-batch-2.md | 132KB | present |
| pass-3-deep-skills-batch-3.md | 116KB | present |
| pass-3-deep-agents.md | 147KB | present |
| pass-3-deep-hooks.md | 112KB | present |
| pass-3-deep-workflows.md | 138KB | present |
| pass-3-deep-templates-tools-rules.md | 198KB | present |
| pass-3-deep-rust-tests.md | 78KB | present |
| pass-4-nfr-catalog.md | 26KB | present |
| pass-5-conventions.md | 20KB | present |
| pass-6-synthesis.md | 27KB | present |
| pass-8-final-synthesis.md | 54KB | present |
| extraction-validation.md | 26KB | present |

**Note:** There is no pass-7 file. This is consistent with the brownfield-ingest workflow structure (pass-7 is a coverage-audit step, not a document-producing step). pass-8 is the final synthesis incorporating pass-7's results.

### Perimeter completeness — what is inside that should be

All major subsystem artifacts are represented:
- Rust workspace: 41/41 source files catalogued, 7/7 crates, 185/185 tests
- Plugin layer: 119/119 skills, 34/34 agents, 44/44 hooks, 16/16 workflows, 12/12 bin tools, 9/9 rules
- Config surface: hooks-registry.toml, hooks.json, observability-config.toml (design-level), plugin.json
- Design intent: `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md` and 7 secondary docs
- Story backlog: EPIC.md + 41 stories

### Perimeter gaps — items outside the perimeter that should arguably be inside

The following areas are documented as not analyzed or under-analyzed, and their status affects Phase 1 accuracy:

**1. 80 of 119 skills missing per-instance BCs (documented, high impact)**
Skills batch-1 covers skills "activate" through "factory-worktree-health" (alphabetical order, approximately 40 skills). Batch-2 covers "feature-mode-scoping-rules" through "post-feature-validation" (approximately 40 skills). Batch-3 covers "pr-create" through "writing-skills" (approximately 39 skills, 81–119 alphabetically). However, batch-3 itself notes at its convergence declaration that "80 of 119 skills (those alphabetically before pr-create)" remain without per-skill BCs at the time batch-3 was written — meaning batches 1 and 2 had not yet been completed in parallel. Whether these 80 skills were completed in subsequent sessions is not documented in any of the 18 files. No pass file lists all 119 skill names with their BC coverage status.

This is the single most significant perimeter question. If batches 1 and 2 were later run and their BCs incorporated into the totals, then pass-8's "1,851 BCs, 100% skills coverage" claim is correct. If not, the 1,851 total includes batch-3's 195 BCs but lacks the per-instance BCs for ~80 skills.

Cross-check: extraction-validation samples 12+14+14=40 BCs from batches 1-3 and confirms all 40. The sample draw is spread across the range 200–794, which spans all three batches. The 100% Skills coverage claim at pass-8 §9 coverage table references 119/119 with HIGH confidence. Taken together, the evidence suggests the three batches were intended to collectively cover all 119 skills (with each batch handling a different alphabetical slice), and the 1,851 total includes all three slices. The "80 skills uncovered" statement in batch-3's convergence refers to batch-3's OWN scope (skills 81-119 only covered skills alphabetically from "pr-create" onward), not to the overall catalog.

**Assessment:** The perimeter appears correct at the 1,851-BC level. However, pass-8 should add an explicit statement confirming that skills 1-40 (batch-1) and skills 41-80 (batch-2) fully cover the alphabetical range below "pr-create." Currently this can only be inferred.

**2. ~50 deferred workflow steps (documented, medium impact)**
pass-3-deep-workflows.md convergence states: "FINDINGS_REMAIN — another round needed: BC IDs from BC-AUDIT-1799+ for ~50 deferred steps (multi-repo continuation, planning per-step, multi-repo sub-modes, greenfield/feature inline loop bodies)." No pass-3 continuation for workflows is present in the 18 files. The template/tools/rules file does include BC-AUDIT-1800–2284, but these are templates, not the deferred workflow steps.

**Assessment:** Approximately 50 workflow sub-step BCs are missing from the catalog. The deferred items are loop bodies and sub-modes that are harder to enumerate from static file analysis.

**3. Per-validator hook BCs collapsed (documented, medium impact)**
BC-AUDIT-068 in pass-3-behavioral-contracts.md covers all 22 validate-*.sh + 1 verify-*.sh hooks as a single aggregate behavioral contract. pass-8 §10 L-P0-003 explicitly flags this as a gap and calls for 23 individual BCs in Phase 1 backfill.

**4. `.github/workflows/` CI pipeline (not analyzed)**
pass-6 §Remaining Gaps GAP-L notes: "ci/ directory contents (only 1 file: platforms.yaml) — full CI workflow definitions live in `.github/workflows/` (per `.github/` directory presence). Not opened this pass." No subsequent pass analyzes the GitHub Actions workflows. The CI pipeline is a behavioral surface (release CI, per-platform build, binary commit automation) that has no BCs in the catalog.

**5. `tests/regression-v1.0.bats` and generate-registry bats (partial analysis)**
pass-3-deep-rust-tests.md covers Rust tests (185 BCs). The bats test suite is noted at "~1262 baseline" and "11 regression-v1.0.bats" but no per-bats-test BC extraction was performed. pass-3-deep-rust-tests.md state checkpoint notes "deep-bash-tests round (regression-v1.0.bats + generate-registry.bats + sample of validate-*.sh) — out of scope for this round."

**6. `tools/observability/` OTel collector configuration (not analyzed)**
pass-4-nfr-catalog.md notes this as out of scope. The OTel collector config, Grafana dashboards, and Loki configs are not catalogued.

**Perimeter verdict:** The perimeter is correctly drawn for the core purpose (enabling Phase 1 spec crystallization of vsdd-factory's two subsystems). Items 4–6 above are explicit non-goals or deferred items. Items 1–3 are more significant: the per-validator hooks and deferred workflow steps represent real gaps, and the 80-skills question should be resolved with an explicit confirmation statement.

---

## Check results by audit scope

### 1. Cross-document numerical consistency
**Status:** MOSTLY CONSISTENT with 3 actionable items:
- Rust test count: 180 in pass-6 and NFR bullet in pass-8 vs. correct 185 (FINDING-I-01)
- Template count: 108 in pass-0/pass-6 vs. correct 105 in pass-8 (pass-6 superseded; pass-0 not corrected)
- docs/guide: 30 claimed in pass-0; actual 28 (pass-0 not corrected per extraction-validation recommendation)

### 2. BC ID consistency
**Status:** CONSISTENT with reservations noted
- All 10 files' BC heading counts verified by grep and match claimed counts (86, 57, 161, 197, 195, 171, 176, 445, 273, 90; sum = 1,851)
- ID ranges are contiguous within each file with intentional buffer gaps between files
- All gaps are documented in per-pass state checkpoints
- No duplicate BC IDs detected across files
- Suffix variants (-PLN in workflows) are documented in source file (pass-3-deep-workflows.md)

### 3. Story coverage consistency
**Status:** CONSISTENT
- pass-6 and pass-8 agree exactly on all 41 story statuses
- extraction-validation confirmed 10/10 sampled stories with correct classifications
- Arithmetic (22+4+15=41) verified correct

### 4. Drift findings consistency
**Status:** CONSISTENT
- All 10 DRIFT items appear in pass-6, pass-8, and extraction-validation
- IDs are stable
- Substance unchanged across documents
- Severity notation changed from descriptive (MEDIUM/HIGH) to Px notation (P0/P1/P2/P3) in pass-8 — reformatting only, no substance change
- DRIFT-006 title abbreviated in pass-8 but body text unchanged

### 5. ADR consistency
**Status:** CONSISTENT with minor numbering note
- 5 explicit ADRs (001–005) from legacy design doc are consistently described across pass-8, pass-1, and pass-4
- 4 open-question resolution ADRs (Q3, Q5, Q6, Q7) consistently described
- 4 as-built ADRs consistently described in both §4 and §11 of pass-8 (with numbering discrepancy: ADR-AS-006..009 in §4 vs ADR-AS-010..013 in §11; see FINDING-N-03)
- Design doc at `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md` confirmed present with the claimed content

### 6. Path references
**Status:** PARTIALLY CORRECTED — one residual reference remains
- pass-8-final-synthesis.md: uses correct `.factory/legacy-design-docs/` path throughout
- pass-6-synthesis.md: uses correct path
- pass-1-architecture.md: no `.factory/specs/` path references to the design doc
- pass-4-nfr-catalog.md: no `.factory/specs/` path references to the design doc
- `pass-0-inventory.md` line 78: still reads `.factory/specs/ — 8 design docs` (FINDING-I-02, partial — line 126 was corrected, line 78 was not)
- `pass-3-deep-skills-batch-1.md`, `pass-3-deep-hooks.md`, `pass-3-deep-workflows.md`, `pass-3-deep-templates-tools-rules.md`, `pass-2-domain-model.md`: contain `.factory/specs/` references that are correct — they refer to the VSDD pipeline's output directories (`.factory/specs/product-brief.md`, `.factory/specs/domain-spec/`, etc.), NOT to the legacy design doc. These are correct references to future VSDD pipeline output paths and should not be changed.

**Net verdict on paths:** Only pass-0-inventory.md line 78 has the wrong path for the legacy design doc. All other `.factory/specs/` references are correct contextual references to VSDD pipeline output paths.

### 7. Self-referential consistency
**Status:** CONSISTENT
- pass-0 line 6: "vsdd-factory IS the project being onboarded — engine and product share the same repo"
- pass-1 "Critical context" header: "This project has TWO subsystems sharing one repo"
- pass-6 §executive summary: "two-subsystem self-referential project"
- pass-8 §1 executive summary: "two-subsystem self-referential project ... The user has explicitly confirmed engine and product are part of the same product"
- No contradicting language found in any pass file

### 8. Convergence claims
**Status:** SUBSTANTIALLY CONSISTENT with one important qualification (FINDING-I-04)
- Each pass file has a Novelty Assessment section with verdict field: confirmed
- pass-0 through pass-6: all declare FINDINGS_REMAIN (correct for their position in the pipeline)
- All BC deepening passes: declare FINDINGS_REMAIN (not NITPICK individually — see FINDING-I-04)
- pass-8 §9 convergence table: declares "Final state: NITPICK" for all passes (synthetic aggregate judgment, not chain-of-NITPICK)
- extraction-validation result "PASS WITH CAVEATS" is correctly cited in pass-8 at lines 334 and 538; not silently upgraded to clean PASS

### 9. Open questions vs. decisions
**Status:** CONSISTENT — all 8 open questions are genuinely open

Review of each question in pass-8 §13:
1. **Subsystem letter assignment:** A/B labeling used throughout the catalog, but formally undecided. Genuine open question — the labeling is conventional, not committed.
2. **BC ID format:** BC-AUDIT-NNN is the current format; migration to BC-S.SS.NNN is recommended but not done. Genuine open question.
3. **DTU assessment trigger:** Not decided. Genuine open question.
4. **Existing story numbering:** S-N.M is used but officially undecided whether to keep. Genuine open question.
5. **DRIFT-004 resolution path:** hooks.json retirement timeline undecided. Genuine open question.
6. **L-P0-001 vs L-P0-002 priority:** Recommended order stated but not committed. Genuine open question.
7. **ADR promotion path:** 13 ADRs exist only in pass files; formal ADR docs not yet created. Genuine open question.
8. **Validation cadence post-Phase 0:** Policy undecided. Genuine open question.

None of these are already decided in `legacy-design-docs/` or implicitly resolved by shipped code. The design doc addresses ADRs 001-005 and Q3/Q5/Q6/Q7 resolutions, but questions 1-8 above are forward-looking Phase 1 governance decisions, not retrospective design recoveries. All genuinely open.

---

## Recommendation

**NEEDS_REVISION** — with 6 actionable items, 4 of which are IMPORTANT and 2 of which require immediate attention before Phase 1 spec crystallization begins:

### Must-fix before Phase 1 (P0)

**P0-A:** Resolve the 80-skills coverage ambiguity (FINDING-I-04 + perimeter gap #1). Pass-8 §14 declares 119/119 skill coverage at 100% confidence HIGH. Pass-3-deep-skills-batch-3 declares "80 of 119 skills uncovered" at its own convergence. These two statements appear contradictory. Add an explicit clarification to pass-8 confirming that skills batch-1 (activate → factory-worktree-health) covers skills 1-40, batch-2 (feature-mode-scoping-rules → post-feature-validation) covers skills 41-80, and batch-3 (pr-create → writing-skills) covers skills 81-119 — making the three batches non-overlapping and collectively complete. If this is the correct interpretation, the 100% coverage claim holds; if not, it does not.

**P0-B:** Correct pass-0-inventory.md line 78 path (`.factory/specs/` → `.factory/legacy-design-docs/`) per extraction-validation P1 recommendation. This file is listed as a primary input for downstream skills in pass-8 §11.

### Should-fix before Phase 1 (P1)

**P1-A:** Correct pass-8-final-synthesis.md §7 Maintainability NFR bullet: 180 → 185 Rust tests (FINDING-I-01).

**P1-B:** Correct pass-5-conventions.md: remove or strike-through the trybuild claims at lines 38 and 243 (FINDING-I-02). Pass-5 is a named input to downstream skills per pass-8 §11.

**P1-C:** Correct extraction-validation.md header: "Sample size (Phase 1): 115 (6.2%)" → "125 (6.75%)" (FINDING-I-03).

**P1-D:** Correct pass-8-final-synthesis.md §11: add `remaining-superpowers-gaps-design` to the list of secondary design docs, making the enumeration complete (FINDING-I-06).

### Deferred (cosmetic)

FINDING-N-01 through FINDING-N-04 require no immediate action.

---

*Validation performed with read-only access. No source artifacts modified.*
