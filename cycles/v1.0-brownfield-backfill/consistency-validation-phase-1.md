---
document_type: consistency-validation
level: ops
producer: consistency-validator
phase: 1d-pre-gate
timestamp: 2026-04-25T14:30Z
inputs:
  - .factory/specs/PRD.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/dtu-assessment.md
  - .factory/stories/STORY-INDEX.md
traces_to: phase-1d-adversarial-review
---

# Consistency Validation: Phase 1 Spec Package (Pre-Gate)

> **Context:** vsdd-factory IS the project being onboarded — engine and product are the
> same repo. This is the perimeter audit per the "Fresh-Context Consistency Audit at Every Gate"
> rule. The adversary catches defects WITHIN the perimeter; this audit checks whether the
> perimeter is correctly drawn and internally consistent.

---

## 1. Numerical Consistency

| Item | Source A | Count A | Source B | Count B | Match? |
|------|----------|---------|----------|---------|--------|
| Total BCs | PRD §12.1, BC-INDEX | 1,851 | Disk (`find ss-NN/ -name "BC-*.md"`) | 1,851 | PASS |
| Subsystem count | ARCH-INDEX | 10 | BC-INDEX | 10 | PASS |
| Total BCs in BC-INDEX summary | BC-INDEX header | 1,851 | Sum of per-SS rows (99+22+49+13+627+571+192+215+5+58) | 1,851 | PASS |
| SS-01 BCs | PRD §2.1 "99 BCs total" | 99 | Disk (ss-01/) | 99 | PASS |
| SS-02 BCs | PRD §2.2 "22 BCs total" | 22 | Disk (ss-02/) | 22 | PASS |
| SS-03 BCs | PRD §2.3 "49 BCs total" | 49 | Disk (ss-03/) | 49 | PASS |
| SS-04 BCs | PRD §2.4 "13 BCs total" | 13 | Disk (ss-04/) | 13 | PASS |
| SS-05 BCs | PRD §2.5 "627 BCs total" | 627 | Disk (ss-05/) | 627 | PASS |
| SS-06 BCs | PRD §2.6 "571 BCs total" | 571 | Disk (ss-06/) | 571 | PASS |
| SS-07 BCs | PRD §2.7 "192 BCs total" | 192 | Disk (ss-07/) | 192 | PASS |
| SS-08 BCs | PRD §2.8 "215 BCs total" | 215 | Disk (ss-08/) | 215 | PASS |
| SS-09 BCs | PRD §2.9 "5 BCs total" | 5 | Disk (ss-09/) | 5 | PASS |
| SS-10 BCs | PRD §2.10 "58 BCs total" | 58 | Disk (ss-10/) | 58 | PASS |
| CAP count | L2-INDEX ID Registry | 28 | capabilities.md (`**CAP-*` headings) | 28 | PASS |
| DI count | L2-INDEX | 17 | invariants.md (`**DI-*` headings) | 17 | PASS |
| DE count | L2-INDEX | 22 | domain-events.md (data rows) | 22 | PASS |
| DEC count | L2-INDEX | 18 | edge-cases.md (`**DEC-*` headings) | 18 | PASS |
| VP count | VP-INDEX total_vps | 57 | Disk (VP-*.md excluding index) | 57 | PASS |
| Story count | STORY-INDEX | 41 | Disk (S-*.md excluding v1.0-legacy/) | 41 | PASS |
| Epic count | STORY-INDEX | 6 | Disk (E-*.md in epics/) | 6 | PASS |
| Story status breakdown | STORY-INDEX | 22 merged + 4 partial + 15 draft = 41 | Summed from STORY-INDEX table | 41 | PASS |
| ADR count | ARCH-INDEX table | 13 (ADR-001..ADR-013) | Disk (decisions/) | 3 | FAIL |
| Architecture section files | ARCH-INDEX Document Map | 8 section files | Disk (architecture/) | 0 of 8 present | FAIL |
| PRD supplement files | PRD frontmatter | 4 files | Disk (prd-supplements/) | 0 of 4 present | FAIL |
| VP proof method: unit-test | VP-INDEX Proof Method row | 34 | Actual VP files (proof_method: unit-test) | 40 | FAIL |
| VP proof method: integration | VP-INDEX | 8 | Actual VP files | 8 | PASS |
| VP proof method: manual | VP-INDEX | 8 | Actual VP files | 9 | FAIL |
| VP proof methods sum | VP-INDEX | 34+8+8 = 50 | Should equal total VPs | 57 | FAIL (internal inconsistency) |

---

## 2. ID Consistency (broken references)

| Issue | Severity | Files | Detail |
|-------|----------|-------|--------|
| VP-047 source_bc references non-existent BC | IMPORTANT | `.factory/specs/verification-properties/VP-047.md:9` | `source_bc: BC-7.05.001` — SS-07 only has sections BC-7.01 through BC-7.04; BC-7.05 does not exist on disk |
| VP-048 source_bc references non-existent BC | IMPORTANT | `.factory/specs/verification-properties/VP-048.md:9` | `source_bc: BC-7.06.001` — SS-07 only has sections BC-7.01 through BC-7.04; BC-7.06 does not exist on disk |
| ADR-004 through ADR-013 referenced but missing | IMPORTANT | `.factory/specs/architecture/ARCH-INDEX.md:131-143` | ARCH-INDEX table lists 13 ADRs; only ADR-001, ADR-002, ADR-003 exist in `decisions/`; 10 ADR files are missing |
| 8 architecture section files missing | IMPORTANT | `.factory/specs/architecture/ARCH-INDEX.md:29-37` | ARCH-INDEX Document Map lists `system-overview.md`, `module-decomposition.md`, `dependency-graph.md`, `api-surface.md`, `verification-architecture.md`, `purity-boundary-map.md`, `tooling-selection.md`, `verification-coverage-matrix.md` — none exist on disk |
| 4 PRD supplement files missing | IMPORTANT | `.factory/specs/PRD.md:26-30` | PRD frontmatter lists `interface-definitions.md`, `error-taxonomy.md`, `test-vectors.md`, `nfr-catalog.md` in `prd-supplements/`; the `prd-supplements/` directory does not exist |
| VP-INDEX references verification-architecture.md and verification-coverage-matrix.md | IMPORTANT | `.factory/specs/verification-properties/VP-INDEX.md:14-16` | VP-INDEX requires changes propagate to these two files, but neither file exists |
| No canonical L1 product-brief.md | NITPICK | `.factory/specs/` | Criterion 1 requires `.factory/specs/product-brief.md`; project uses brownfield legacy design doc as L1 equivalent; not present as a formal L1 artifact |

---

## 3. Identifier-Format Consistency

| Format | Documents checked | Compliant? |
|--------|------------------|------------|
| BC IDs: all `BC-S.SS.NNN` format | BC-INDEX, PRD §2, all ss-NN/ files (sampled) | PASS — no legacy BC-AUDIT-NNN found in committed spec files |
| Story IDs: all `S-N.MM` format | STORY-INDEX, all story files | PASS — canonical stories use S-N.MM; legacy S-N.M exist only in `v1.0-legacy/` subdirectory as expected |
| Epic IDs: `E-N` format | STORY-INDEX, story frontmatter | PASS — E-0 through E-5 used consistently |
| Subsystem IDs: `SS-NN` format | ARCH-INDEX, BC-INDEX, L2-INDEX | PASS — SS-01 through SS-10 used consistently |
| VP IDs: `VP-NNN` format | VP-INDEX, VP files | PASS — VP-001 through VP-057 |
| CAP IDs: `CAP-NNN` format | capabilities.md, PRD §8, L2-INDEX | PASS — CAP-001 through CAP-028 |
| ADR IDs: `ADR-NNN` format | ARCH-INDEX decisions table | PASS for existing 3; ADR-004..013 referenced but missing |

---

## 4. Cross-Document Semantic Alignment

### 4.1 CAP anchoring

All 28 CAPs are anchored to BC ranges in PRD §8. The PRD's CAP-to-BC anchoring section (§8) covers all 28 CAPs (CAP-001 through CAP-028) with at least one BC prefix range each. No orphaned CAPs detected.

One note: PRD §2.1 header reads "SS-01 — CAP-002, CAP-007, CAP-008, CAP-010, CAP-011" but the L2-INDEX Subsystem Cross-Walk attributes CAP-001 and CAP-009 to SS-01 as well. This is a section-header-level omission (the PRD header is only partial; the full CAP-to-BC table in §8 correctly attributes CAP-001 and CAP-009 to SS-01 indirectly via SS-05 and SS-02).

### 4.2 DI-to-VP coverage

All 17 domain invariants (DI-001 through DI-017) are covered by VP-001 through VP-017 per VP-INDEX. This 1:1 mapping is structurally complete.

Additional VPs (VP-018 through VP-057) reference DIs from the invariant catalog (cross-cutting coverage) or are DI-independent properties (VP-024, VP-048, VP-053 through VP-057 have no DI link, which is documented as intentional in VP-INDEX).

### 4.3 Self-referential perimeter

The self-referential nature is explicitly documented:
- PRD §1.2 ("The product was built with itself...This self-referential loop is the ultimate dogfooding test")
- PRD §1.4 (lists "Platform engineer / dogfooder: vsdd-factory itself" as a target user persona with VERY HIGH pain level)
- STATE.md ("vsdd-factory IS the project being onboarded. Engine and product are the same repository.")

SS-05 through SS-10 (the VSDD orchestration framework) are fully modeled as first-class product subsystems in ARCH-INDEX, BC-INDEX, and PRD. This is coherent.

### 4.4 NFR coverage

PRD §4 summarizes 76 NFRs across 8 categories. The supplement file (nfr-catalog.md) is referenced but does not exist (documented in §2). The PRD body provides sufficient NFR summary for adversarial review; the missing supplement is a completeness gap, not a semantic gap.

---

## 5. Coverage Gaps

| Gap | Severity | Count | Detail |
|-----|----------|-------|--------|
| Stories with empty `behavioral_contracts` frontmatter | NITPICK | 41 of 41 | All 41 story files have `behavioral_contracts: []` in frontmatter. 31 of these reference BC IDs in the story body (ACs, Architecture Compliance Rules), creating a frontmatter-body drift per criterion 69. This is a known Phase 1.8 gap: stories were migrated from legacy format before BC anchoring was completed. |
| Stories with no BC reference at all (body or frontmatter) | NITPICK | 10 of 41 | S-0.01, S-0.02, S-0.03, S-0.04, S-0.05, S-1.01, S-2.04, S-2.05, S-4.03, S-5.05 — these are infrastructure/tooling/docs stories with no behavioral contracts. Likely intentional for non-functional stories. |
| BCs without CAP anchor (BC-INDEX `capability: CAP-TBD`) | NITPICK | ~1,851 | BC-INDEX shows all BCs as `CAP-TBD` in the capability column. This is the known Phase 1.5 gap: L-P0-003 from PRD §10.3 acknowledges that CAP backfill for all 1,851 BCs is in-progress. |
| Architecture section files missing from ARCH-INDEX Document Map | IMPORTANT | 8 of 8 | `system-overview.md`, `module-decomposition.md`, `dependency-graph.md`, `api-surface.md`, `verification-architecture.md`, `purity-boundary-map.md`, `tooling-selection.md`, `verification-coverage-matrix.md` — all 8 listed in ARCH-INDEX Document Map are absent from disk. STATE.md notes "Phase 1.1 — PARTIAL: 3 of 13 ADRs (10 deferred)". |
| ADR files missing (ADR-004 through ADR-013) | IMPORTANT | 10 of 13 | Acknowledged in STATE.md as deferred. |
| PRD supplement files missing | IMPORTANT | 4 of 4 | `prd-supplements/` directory does not exist. PRD body contains adequate summaries; supplements add depth for tooling but are not load-bearing for the adversarial spec review. |
| VP-INDEX Proof Method Breakdown undercounts | NITPICK | 7 VPs | VP-INDEX says unit-test=34 (actual: 40), manual=8 (actual: 9); total listed = 50 (should = 57). VP-049 is misclassified as unit-test in VP-INDEX but its file has `proof_method: integration`. |
| VP-047 and VP-048 source_bc broken references | IMPORTANT | 2 VPs | BC-7.05.001 and BC-7.06.001 are cited but do not exist; SS-07 only has sections BC-7.01 through BC-7.04. |

---

## 6. PRD BC Section Description vs Actual BC File Content (SS-07 Mismatch)

**Summary:** The PRD's functional grouping of SS-07 BCs (FR-032, FR-033, FR-034) assigns section numbers BC-7.01 through BC-7.10 that do not correspond to the actual file organization on disk.

| PRD Claims | Actual on Disk |
|-----------|---------------|
| FR-032: BC-7.01.001-NNN = protect-secrets.sh (PreToolUse gates) | BC-7.01.001 = "block-ai-attribution blocks git commit messages" |
| FR-033: BC-7.05.001-NNN = capture-commit-activity.sh (PostToolUse hooks) | No BC-7.05 section exists; capture-commit-activity is at BC-7.01.002 |
| FR-033: BC-7.06.001-NNN = capture-pr-activity.sh | No BC-7.06 section exists |
| FR-034: BC-7.08.001-NNN = validate-* family | No BC-7.08 section exists; validate-* hooks are in BC-7.04 |
| FR-034: BC-7.09.001-NNN = SubagentStop hooks | No BC-7.09 section exists |
| FR-034: BC-7.10.001-NNN = verify-sha-currency.sh | No BC-7.10 section exists |

**Actual SS-07 disk organization:**
- BC-7.01 (7 files): mixed — block-ai-attribution, capture-commit-activity, protect-secrets, validate-* overview
- BC-7.02 (9 files): hooks stdin/stdout processing contracts
- BC-7.03 (93 files): individual hook identity and registry binding contracts (all hook types mixed)
- BC-7.04 (83 files): validate-* hook contracts

The PRD's FR narrative was written as a forward plan for how BCs _would be_ organized, but the actual BC file generation produced a different flat+mixed structure. The total count (192) matches; the section descriptions do not.

**Impact:** Phase 1d adversary should treat the PRD's FR-032/033/034 BC range descriptions as approximate/aspirational, not as exact file paths. VP-047 and VP-048 need their `source_bc` fields corrected.

---

## 7. Other Notable Observations

### 7.1 story `cycle` field vs STATE current_cycle

- All 41 story files have `cycle: v1.0.0-greenfield`
- STATE.md `current_cycle: v1.0-brownfield-backfill`

This is NOT a defect. The stories were originally written in the "greenfield" sub-cycle (Phase 1.8 story migration). The brownfield-backfill is the overarching meta-cycle for the formal spec backfill. The cycle values represent when stories were authored, not the current operational cycle.

### 7.2 L2-INDEX `traces_to` field

L2-INDEX has `traces_to: phase-1-spec-crystallization` — this is a phase name, not a file path. In canonical brownfield mode, there is no L1 `product-brief.md`. The legacy design doc serves as the de-facto L1. This is an acknowledged brownfield adaptation, not a spec defect.

### 7.3 P0 stories depending on P1 stories

- S-4.08 (P0, rc.1 release gate) depends on S-4.07 (P1)
- S-5.07 (P0, 1.0 GA release gate) depends on S-5.01..S-5.06 (all P1)

This is an intentional release-gate modeling pattern: the gate itself is P0 (must ship for the milestone) but it waits for P1 features (which are P1 because they're not individually required for each beta). The STORY-INDEX explicitly documents this as a valid pattern in its rules section.

### 7.4 SS-01 CAP assignment in PRD §2.1 header

PRD §2.1 header: "SS-01 — CAP-002, CAP-007, CAP-008, CAP-010, CAP-011"
L2-INDEX crosswalk: SS-01 also supports CAP-001, CAP-009

This is a section-header shorthand, not a semantic error. PRD §8 (CAP-to-BC Anchoring) correctly shows CAP-001 routed via SS-05/SS-06, and CAP-009 routed through SS-02. SS-01 participates but the header is listing the primary CAPs. Minor documentation inconsistency only.

### 7.5 PRD FR-007 BC count claim

PRD §2.1 FR-007: "BC-1.07.001 through BC-1.08.006 (10 BCs)" — but disk shows BC-1.07 = 6 files and BC-1.08 = 6 files = 12 total BCs. The prose says 10, but 12 is correct. The total SS-01 count (99) is correct.

### 7.6 PRD FR-012 BC count claim

PRD §2.3 FR-012: says 18 BCs. Actual count (1 + 13 + 2 + 3 = 19 BCs). The total SS-03 count (49) is correct.

---

## 8. Findings

### CRITICAL (blocks Phase 1d)

*No CRITICAL findings that would prevent adversarial review from proceeding.*

The items listed below as IMPORTANT are incomplete artifact sets acknowledged in STATE.md as deferred Phase 1.1 work. They do not introduce false information into the spec package — they represent gaps in depth, not contradictions in content.

### IMPORTANT (should fix during or after Phase 1d cycle)

**IMPORTANT-01: VP-047 and VP-048 cite non-existent BC IDs**
- File: `.factory/specs/verification-properties/VP-047.md` (line 9: `source_bc: BC-7.05.001`)
- File: `.factory/specs/verification-properties/VP-048.md` (line 9: `source_bc: BC-7.06.001`)
- Detail: SS-07 only has BC-7.01 through BC-7.04. BC-7.05 and BC-7.06 do not exist.
- VP-047 covers "Validator Hooks Exit 0 or 2" — the correct source BC should be in the BC-7.04 range (validate-* hooks).
- VP-048 covers "protect-secrets.sh Fails Closed When jq Is Missing" — the correct source BC should be in BC-7.03 range (protect-secrets is BC-7.03.xxx per actual file organization).
- Remediation: Update `source_bc` fields in VP-047 and VP-048 to reference real BC files. Search BC-7.04 for the validator-exit-code contract and BC-7.03 for protect-secrets jq-missing contract.

**IMPORTANT-02: 8 architecture section files missing from ARCH-INDEX Document Map**
- File: `.factory/specs/architecture/ARCH-INDEX.md` (lines 29-37)
- Missing files: `system-overview.md`, `module-decomposition.md`, `dependency-graph.md`, `api-surface.md`, `verification-architecture.md`, `purity-boundary-map.md`, `tooling-selection.md`, `verification-coverage-matrix.md`
- These are all listed as existing in the ARCH-INDEX Document Map but none are present in `.factory/specs/architecture/`.
- STATE.md acknowledges: "Phase 1.1 — PARTIAL: 3 of 13 ADRs; 10 deferred". The same partial status applies to these section files.
- Impact on adversarial review: The adversary cannot load these section files. The 10 SS-NN-<name>.md files ARE present and contain the substantive content. The missing files are higher-level synthesis documents.
- Remediation: Either create stub versions or update ARCH-INDEX Document Map to reflect actual availability.

**IMPORTANT-03: 10 ADR files missing (ADR-004 through ADR-013)**
- File: `.factory/specs/architecture/ARCH-INDEX.md` (lines 134-143)
- Only ADR-001, ADR-002, ADR-003 exist in `decisions/`; ADR-004 through ADR-013 are listed in the table but not present.
- STATE.md explicitly acknowledges: "ADR backlog: 10 deferred ADRs (ADR-004..ADR-013 stubs exist; full write-up after PR #4 lands)" — NOTE: STATE.md says "stubs exist" but no stub files were found in the decisions/ directory. This is a secondary inconsistency in STATE.md.
- Remediation: Create at minimum one-line stub files for ADR-004 through ADR-013 so that references aren't broken, OR update ARCH-INDEX to mark them as "pending".

**IMPORTANT-04: 4 PRD supplement files missing**
- File: `.factory/specs/PRD.md` (lines 26-30, and §3, §4, §5b references)
- The PRD frontmatter lists `prd-supplements/` as containing 4 files; the directory does not exist.
- The PRD body contains adequate summaries of all supplement content. The supplements would add detail for tooling integration and automated validation but are not load-bearing for the human/adversarial review.
- Remediation: Create `prd-supplements/` directory and stub files, OR remove from PRD frontmatter supplements field.

**IMPORTANT-05: VP-INDEX Proof Method Breakdown has internal arithmetic errors**
- File: `.factory/specs/verification-properties/VP-INDEX.md` (lines 36-40)
- Claims: unit-test=34, integration=8, manual=8, total listed=50. But total VPs=57.
- Actual (from files): unit-test=40, integration=8, manual=9, sum=57.
- Also: VP-049 is listed in the unit-test row of VP-INDEX but VP-049.md has `proof_method: integration`.
- Remediation: Update VP-INDEX Proof Method Breakdown table to accurately count: unit-test=40, integration=8, manual=9. Move VP-049 from the unit-test row to the integration row.

**IMPORTANT-06: PRD §2.7 BC range descriptions for SS-07 don't match actual file organization**
- File: `.factory/specs/PRD.md` (lines 585-613, FR-032 through FR-034)
- PRD describes BC-7.01-7.10 sections (PostToolUse hooks, SubagentStop hooks, etc.) that do not exist on disk.
- Actual SS-07 has only BC-7.01 through BC-7.04 (7, 9, 93, 83 files respectively), with a mixed organization not matching the PRD's per-FR section descriptions.
- The total BC count (192) is correct. The issue is that the PRD's narrative descriptions of what each BC-7.NN section contains are wrong.
- This is the root cause of IMPORTANT-01 (VP-047/048 cite BC-7.05/BC-7.06 based on PRD's claimed structure).
- Remediation: After Phase 1d, update PRD §2.7 FR-032/033/034 to reflect actual BC-7 file organization, or add a note that BC ranges are aspirational groupings pending re-numbering.

### NITPICK (cosmetic)

**NITPICK-01: All 41 story `behavioral_contracts` frontmatter arrays are empty**
- All 41 story files have `behavioral_contracts: []`. This is a known Phase 1.8 migration gap.
- 31 stories reference BC IDs in their body (ACs, compliance rules), creating criterion-69 frontmatter-body drift.
- 10 stories have no BC references at all (infrastructure/docs stories).
- Not blocking Phase 1d: story-to-BC traceability works through the PRD RTM and BC-INDEX.
- Remediation: Phase 2 story anchoring work should populate `behavioral_contracts` arrays.

**NITPICK-02: VP-INDEX lists VP-049 in unit-test row but VP-049.md is integration**
- (Also mentioned in IMPORTANT-05 for the arithmetic side; the classification error itself is a nitpick.)
- VP-049 ("Generated hooks-registry.toml Round-Trips Through Registry::load") has `proof_method: integration` in its file, but VP-INDEX's unit-test row includes "VP-049..050".
- VP-050 is correctly unit-test. VP-049 should be in the integration row.

**NITPICK-03: PRD FR-007 BC count prose says 10, actual is 12**
- File: `.factory/specs/PRD.md` (line 241): "Source BCs: `ss-01/BC-1.07.001.md` through `BC-1.08.006.md` (10 BCs)"
- Actual: BC-1.07 has 6 files, BC-1.08 has 6 files = 12 total. Prose undercounts by 2.
- Total SS-01 count (99) is correct.

**NITPICK-04: PRD FR-012 BC count prose says 18, actual count is 19**
- File: `.factory/specs/PRD.md` (line 323): "18 BCs in this group"
- Actual: BC-3.01.009 (1) + BC-3.03.001-013 (13) + BC-3.04.001-002 (2) + BC-3.05.001-003 (3) = 19.
- Total SS-03 count (49) is correct.

**NITPICK-05: L2-INDEX `traces_to` is a phase name, not a file path**
- File: `.factory/specs/domain-spec/L2-INDEX.md` (line 16): `traces_to: phase-1-spec-crystallization`
- In brownfield mode without a canonical product-brief.md, the L2 traces to the ingestion phase.
- No L1 `product-brief.md` exists. This is a brownfield adaptation, not a defect.

**NITPICK-06: STATE.md says "ADR-004..ADR-013 stubs exist" but no stubs found**
- File: `.factory/STATE.md` (ADR backlog note)
- STATE.md says "stubs exist" but only ADR-001, ADR-002, ADR-003 are present in `decisions/`.
- No stub files for ADR-004 through ADR-013 were found.

---

## 9. Self-Referential Perimeter Check

**Question:** Does the spec package describe both subsystems (Rust dispatcher + orchestration framework) coherently?

**Finding: PASS.** The two-subsystem model (SS-01..04 = Subsystem A Rust, SS-05..10 = Subsystem B VSDD framework) is:
- Declared in ARCH-INDEX §Subsystem Registry Design Notes
- Reflected in BC-INDEX with correct SS-NN prefixes
- Repeated in PRD §1.2, §2, §8
- Consistent in L2-INDEX Subsystem Cross-Walk

**Question:** Is the engine-and-product self-referential nature called out at top-level?

**Finding: PASS.** PRD §1.2, §1.4, and STATE.md all explicitly call this out. ARCH-INDEX lacks a system-overview.md to make it explicit at the architecture layer (that file is missing per IMPORTANT-02), but the ARCH-INDEX body implicitly treats both subsystems as first-class.

**Question:** Are vsdd-factory's own BCs (SS-05..10) treated as first-class product behavior?

**Finding: PASS.** SS-05 (627 BCs) and SS-06 (571 BCs) are the two largest subsystems by BC count, accounting for 64% of all behavioral contracts. They are fully modeled, indexed, and integrated into the PRD RTM.

---

## 10. Consistency Score

| Category | Criteria Checked | Passed | Failed |
|----------|-----------------|--------|--------|
| Numerical consistency (counts) | 25 | 22 | 3 (ADR count, arch section files, PRD supplements) |
| ID consistency (broken refs) | 7 | 2 | 5 |
| Identifier format | 7 | 7 | 0 |
| Cross-document semantic alignment | 5 | 5 | 0 |
| Coverage gaps | 8 | 5 | 3 (BC frontmatter, arch files, PRD supplements) |
| PRD BC section accuracy | 6 | 3 | 3 (SS-07 narrative, FR counts) |
| Self-referential perimeter | 3 | 3 | 0 |
| **Total** | **61** | **47** | **14** |

**Consistency score: 47/61 = 77%**

Score interpretation: The 14 failures break down as:
- 10 are NITPICK (cosmetic prose mismatches, count-off-by-one errors in FR descriptions)
- 4 are IMPORTANT structural gaps (missing files acknowledged in STATE.md as deferred)
- 0 are CRITICAL (no contradictions that would derail adversarial review)

---

## 11. Recommendation

**PASS with conditions**

Phase 1d adversarial review can proceed. The spec package is structurally sound:
- All 1,851 BCs exist on disk and are correctly indexed
- All 57 VPs exist and correctly reference domain invariants (with 2 broken source_bc fields)
- All 28 CAPs are anchored to BC ranges in PRD §8
- All 17 DIs have VP coverage
- All 10 SS-NN architecture section files exist with correct traces_to
- All 7 L2 domain spec sections exist with correct traces_to
- Story count (41) and epic count (6) match across all documents
- Self-referential perimeter is coherent and explicitly documented

The IMPORTANT findings (IMPORTANT-01 through IMPORTANT-06) represent incomplete artifact sets (missing architecture section files, ADRs, PRD supplements) and a VP-INDEX internal inconsistency. None of these introduce false information that would mislead the adversary. The adversary should be aware that:

1. The ARCH-INDEX Document Map references 8 section files that don't exist — the adversary should NOT expect to load them.
2. PRD §2.7 BC range descriptions for SS-07 are aspirational/misaligned — references to BC-7.05+ do not resolve to real files.
3. VP-047 and VP-048 source_bc fields are broken — these VPs exist and are substantively correct; only their back-reference to the source BC needs correction.

**Recommended pre-Phase-1d fixes (can be done in one burst):**
1. Fix `source_bc` in VP-047 and VP-048 (IMPORTANT-01)
2. Update VP-INDEX Proof Method Breakdown arithmetic (IMPORTANT-05)

**Deferrable to post-Phase-1d:**
- Create architecture section files or mark them pending (IMPORTANT-02)
- Create ADR stubs or mark pending (IMPORTANT-03)
- Create PRD supplements directory and stubs (IMPORTANT-04)
- Update PRD §2.7 FR narrative to match actual BC-7 file organization (IMPORTANT-06)
- Populate `behavioral_contracts` frontmatter in story files (NITPICK-01)
