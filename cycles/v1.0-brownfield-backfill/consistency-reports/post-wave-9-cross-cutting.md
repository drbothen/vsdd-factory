---
document_type: consistency-validation-report
level: ops
version: "1.0"
producer: consistency-validator
phase: post-wave-9-cross-cutting
status: findings_remain
timestamp: 2026-04-27T00:00:00Z
inputs:
  - .factory/STATE.md
  - .factory/specs/PRD.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/tech-debt-register.md
  - .factory/cycles/v1.0-brownfield-backfill/INDEX.md
  - (all 45 story files in .factory/stories/S-*.md)
cycle: v1.0-brownfield-backfill
traces_to: .factory/cycles/v1.0-brownfield-backfill/INDEX.md
---

# Consistency Validation Report: Post-Wave-9 Cross-Cutting Audit

> **Scope:** Full perimeter audit of the v1.0-brownfield-backfill cycle after Wave 9 SS-01
> straggler convergence (41/41 stories re-anchored). This is the cycle's FIRST fresh-context
> cross-wave audit — per-wave adversarial reviews caught defects within each wave's perimeter;
> this sweep verifies internal consistency across all 9 waves.
>
> **Auditor:** consistency-validator (fresh context, no prior wave review session)
> **Date:** 2026-04-27
> **Gate:** Pre-TD-006 POLICY 11/12 gating; pre-factory-dispatcher next-phase resumption

---

## 1. Audit Summary

### 1.1 Pass/Fail by Check Category

| Category | Rules | PASS | FAIL | WARN | Notes |
|----------|-------|------|------|------|-------|
| Spec Format (L1-L4 hierarchy) | Rule 0 | PASS | — | — | All 4 levels present; prd-supplements deferred (documented) |
| PRD/Epic/Story/Architecture chain | Rules 1–10 | PASS | FAIL | WARN | See §4 findings |
| Semantic Drift | Rule 11 | WARN | — | — | Pre-existing, tracked as TD-001..TD-004 |
| Upstream + Downstream traceability | Rules 13–14 | PASS | FAIL | WARN | See §4 findings |
| L1→L4 chain completeness | Rules 15–18 | PASS | — | WARN | 7 stories have empty FR field (MINOR) |
| BC-to-Story mapping | Rule 19 | WARN | — | — | Tracked under TD-001 |
| AC-to-BC traceability | Rule 20 | PASS | — | WARN | [process-gap] pattern used per STORY-INDEX policy |
| VP Registry Completeness | Rule 21 | PASS | — | — | 64 VPs, arithmetic consistent |
| BC clause reverse coverage | Rules 25–28 | PASS | — | WARN | [process-gap] markers used per policy |
| PRD scope / RTM / frontmatter | Rules 30–33 | PASS | FAIL | WARN | FR count discrepancy in PRD; story status drift |
| BC lifecycle field coherence | Rule 35 | PASS | — | — | All BCs have active lifecycle_status |
| L2 Domain Spec sharding integrity | Rule 35 | PASS | — | WARN | L2-INDEX cross-walk stale for 3 CAP expansions |
| Cross-wave convention drift | CW-1..6 | PASS | FAIL | WARN | See §5 cross-wave findings |

### 1.2 Severity Breakdown

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| MAJOR | 5 |
| MINOR | 7 |
| INFO / Acknowledged (TD-NNN) | 9 |
| **Total new findings** | **12** |

### 1.3 Consistency Score

- Criteria checked: 52 of 52 in-scope (Rules 22–24, 36 skipped per brief — CLI-only product)
- Criteria passing cleanly: 44
- Criteria with findings: 8
- **Consistency score: 85% (44/52)**

---

## 2. Numerical Consistency Verification

This section cross-checks counts across all artifact layers. Values are filesystem-observed unless noted.

| Item | Expected (index) | Actual (disk/parse) | Match? | Note |
|------|-----------------|---------------------|--------|------|
| Total BCs (BC-INDEX total_bcs) | 1,891 | 1,891 | PASS | SS-05 logical 646 = 642 files + 4 BC-8 moved per POLICY 1 |
| SS-01 BCs | 99 | 99 | PASS | |
| SS-02 BCs | 22 | 22 | PASS | |
| SS-03 BCs | 49 | 49 | PASS | |
| SS-04 BCs | 13 | 13 | PASS | |
| SS-05 BCs (logical) | 646 | 642 files + 4 BC-8 rows in SS-05 section | PASS | D-013/D-014 POLICY 1 documented |
| SS-06 BCs | 585 | 585 | PASS | |
| SS-07 BCs | 196 | 196 | PASS | |
| SS-08 BCs (logical) | 218 | 222 files − 4 moved to SS-05 = 218 | PASS | D-013/D-014 POLICY 1 documented |
| SS-09 BCs | 5 | 5 | PASS | |
| SS-10 BCs | 58 | 58 | PASS | |
| BC-INDEX Summary vs Detail (total) | 1,891 | 1,891 | PASS | Sum of per-SS detail rows = 1,891 |
| Total VPs (VP-INDEX) | 64 | 64 | PASS | VP-001..VP-064 |
| VP category sum | 64 | 17+10+10+5+10+5+3+2+2=64 | PASS | |
| VP proof method sum | 64 | 40+12+10+1+0+1=64 | PASS | |
| Total stories (STORY-INDEX) | 45 | 45 | PASS | S-0.01..S-7.03 |
| Stories on disk | 45 | 45 | PASS | |
| Unique FR IDs in PRD | 44 (§7 RTM says 44) | FR-001..FR-044 = 44 | PASS | FR-043 appears 4x (3 SS-slices + 1 summary = 1 logical FR) |
| CAP count | 28 | 28 | PASS | CAP-001..CAP-028 |
| DI count | 17 | 17 | PASS | DI-001..DI-017 in invariants.md |
| STORY-INDEX summary merged count | 22 | 21 (table rows) | **FAIL** | See F-002 |
| STORY-INDEX summary draft count | 15 | 16 (table rows) | **FAIL** | See F-002 |
| S-7.03 status: index vs file | completed (index) | ready (file) | **FAIL** | See F-003 |

---

## 3. Spec Format Validation (Rule 0)

### 3.1 L1-L4 Hierarchy

| Level | Artifact | Present | Frontmatter | Notes |
|-------|----------|---------|-------------|-------|
| L1 (Product Brief) | PRD.md (≈ L1+L2 merged) | YES | Yes (document_type: prd) | No standalone brief; PRD serves dual function per brownfield onboarding |
| L2 (Domain Spec) | domain-spec/ (sharded) | YES | L2-INDEX.md with 7 sections | PASS |
| L3 (BCs + Architecture) | behavioral-contracts/ + architecture/ | YES | BC-INDEX.md + ARCH-INDEX.md | PASS |
| L4 (Verification Properties) | verification-properties/ | YES | VP-INDEX.md | PASS |
| L2 Supplements | prd-supplements/ | ABSENT | — | DOCUMENTED deferral — PRD frontmatter §1 notes "Supplements deferred — PRD body contains summary versions." Not a blocking finding; tracked under pre-existing deferral. |

**Verdict: PASS** (deferral of prd-supplements is documented and pre-existing)

### 3.2 Sharding Integrity

| Index | Present | Detail files | Orphans | Notes |
|-------|---------|-------------|---------|-------|
| BC-INDEX.md | YES | 1,891 files | 0 (verified per SS above) | PASS |
| ARCH-INDEX.md | YES | 10 SS-NN files | 0 | PASS |
| VP-INDEX.md | YES | 64 VP files | 0 | PASS |
| L2-INDEX.md | YES | 7 section files | 0 | PASS |
| STORY-INDEX.md | YES | 45 story files | 0 | PASS |

---

## Detailed Findings

> Full findings are structured in §4–§5 below, grouped by severity and cross-wave category.
> The tables below satisfy the template's `## Detailed Findings` anchor; see §7 for the
> consolidated summary tables.

### CRITICAL

| Criterion | Description | Evidence | Remediation |
|-----------|-------------|----------|-------------|
| — | No critical findings. | — | — |

### MAJOR

| Criterion | Description | Evidence | Remediation |
|-----------|-------------|----------|-------------|
| Rule 35 / CW-5 | L2-INDEX Subsystem Cross-Walk stale after Wave 5/6 CAP expansions (F-001) | L2-INDEX.md lines 80, 84, 85 | Update SS-01, SS-05, SS-06 rows |
| Rule 31 | STORY-INDEX summary status counts inconsistent with detail table (F-002) | STORY-INDEX.md lines 29–36 | Correct merged=21, draft=16 |
| Rule 53 | S-7.03 status drift: STORY-INDEX=completed, file=ready, actual=merged (F-003) | S-7.03 line 9; STORY-INDEX line 121 | Update to merged in both |
| CW-3 | Dep graph asymmetry: S-1.04→S-3.04 and S-3.04→S-4.07 edges missing (F-004) | S-1.04 blocks; S-3.04 blocks | Add 2 missing blocks entries |
| Rule 15 | 6 stories have empty functional_requirements (L1→L4 chain gap) (F-005) | S-0.05, S-5.05, S-5.06, S-6.01, S-7.01, S-7.02 frontmatter | Populate FR-NNN fields |

### MINOR

| Criterion | Description | Evidence | Remediation |
|-----------|-------------|----------|-------------|
| Rule 30 | PRD §2/§14 say 43 FRs; §7 RTM says 44 FRs (F-006, D-028 lineage) | PRD lines 47, 1083, 1337 | Update §2 and §14 to 44 |
| CW-1 | Wave 8 stories violate F-305 section ordering — pending intent (F-007) | S-0.05:74, S-5.05:66, S-5.06:63 | Move v1.1 candidates after AC-traces |
| CW-1 | Wave 9 S-2.07 Stretch-Anchor Disclosure after Arch Compliance (F-008) | S-2.07 line 161 | Move before Behavioral Contracts |
| Rule 18 | capabilities: frontmatter field used non-uniformly across stories (F-009) | S-0.05, S-2.07, S-3.04 frontmatter | Standardize or document in template |
| Rule 22 | BC-INDEX and BC files traces_to bc-id-mapping.md (pre-existing) (F-010) | BC-INDEX.md frontmatter | Deferred to TD-001 BC backfill |
| Rule 53 | PRD milestone references stale — D-030 acknowledged (F-011) | PRD §1.2, CAP-028 outcome text | Deferred per D-030 to v1.1 |
| Rule 66 | prd-supplements/ directory absent — documented deferral (F-012) | PRD frontmatter §1 | Pre-existing; track under PRD footnote |

---

## 4. Substantive Findings

### 4.1 MAJOR Findings

---

#### F-001 — L2-INDEX Subsystem Cross-Walk Stale After Wave 5/6 CAP Expansions (MAJOR)

**Rule:** Rule 35 (L2 Domain Spec sharding integrity), CW-5 (capabilities subsystem-column drift)  
**Severity:** MAJOR  
**Status:** NEW

**Description:** The L2-INDEX.md Subsystem Cross-Walk table was produced in Phase 1.3 (2026-04-25) and has not been updated to reflect three subsystem expansions applied during Waves 5 and 6:

1. **CAP-018 missing SS-05:** L2-INDEX `SS-05` row lists `CAP-001, CAP-004, CAP-005, CAP-006, CAP-012, CAP-014, CAP-016` — missing `CAP-018`. Wave 6 F-302 (pass-4 fix burst) expanded CAP-018 to include `SS-05, SS-06` in capabilities.md and PRD §8. L2-INDEX was not updated.

2. **CAP-028 missing SS-06:** L2-INDEX `SS-06` row does not include `CAP-028`. Wave 6 F-005 (sanctioned per Wave 3 F-007 precedent) expanded CAP-028 to `SS-06, SS-09` in capabilities.md. L2-INDEX `SS-09` row correctly shows `CAP-028` but `SS-06` row is missing it.

3. **CAP-007 and CAP-009 erroneously in SS-01:** L2-INDEX `SS-01` row lists `CAP-001, CAP-002, CAP-007, CAP-008, CAP-009, CAP-010, CAP-011`. `CAP-007` was explicitly REVERTED from SS-01 in Wave 5 pass-1 HIGH-002 fix (capabilities.md:46 comment: "SS-01 expansion reverted at Wave 5 pass-1 fix burst per HIGH-002 — no SS-01 BC anchored to activate-skill stories"). `CAP-009` is SS-02-only per capabilities.md. L2-INDEX SS-01 row was never corrected.

**Evidence:**
- L2-INDEX.md line 84: `SS-05 | CAP-001, CAP-004, CAP-005, CAP-006, CAP-012, CAP-014, CAP-016` — missing CAP-018
- L2-INDEX.md line 85: `SS-06 | ... CAP-026` — missing CAP-028
- L2-INDEX.md line 80: `SS-01 | CAP-001, CAP-002, CAP-007, CAP-008, CAP-009, CAP-010, CAP-011` — stale
- capabilities.md line 46: Wave 5 HIGH-002 SS-01 revert comment
- capabilities.md line 128: F-302 CAP-018 expanded to SS-05, SS-06
- capabilities.md line 87: CAP-028 expanded to SS-06, SS-09

**Remediation:** Update L2-INDEX.md Subsystem Cross-Walk:
- SS-01 row: remove CAP-007 and CAP-009; retain CAP-001, CAP-002, CAP-008, CAP-010, CAP-011
- SS-05 row: add CAP-018
- SS-06 row: add CAP-028

---

#### F-002 — STORY-INDEX Summary Table Status Counts Inconsistent with Detail Table (MAJOR)

**Rule:** Rule 31 (Accumulated story count matches STORY-INDEX)  
**Severity:** MAJOR  
**Status:** NEW

**Description:** The STORY-INDEX Status Summary table (lines 29–36) contains counts that disagree with the actual table rows in the index body:

| Status | Summary Says | Table Rows | Actual Files |
|--------|-------------|------------|--------------|
| merged | 22 | 21 | 21 |
| draft | 15 | 16 | 16 |
| ready | 3 | 3 | 4 |
| completed | 1 | 1 | 0 (S-7.03 file = ready) |

The summary `merged=22` overcounts by 1. The summary `draft=15` undercounts by 1. These two errors cancel out at the Total=45 level (Total is correct). However, the per-status summary is wrong on both lines.

The root cause: when S-7.03 was marked `completed` in the STORY-INDEX table (reflecting PR #13 delivery), the `merged` count was incremented by 1 (suggesting someone counted S-7.03 as merged) while `draft` was not adjusted, leaving the summary row counts inconsistent with both the detail table and the actual story files.

**Evidence:**
- STORY-INDEX.md lines 30–36 (Status Summary table)
- STORY-INDEX.md line 121: `| S-7.03 | ... | completed |`
- STORY-INDEX.md table detail rows: 21 merged entries, 16 draft entries
- Disk verification: 21 story files with `status: merged`, 16 with `status: draft`

**Remediation:** Correct STORY-INDEX Status Summary table:
- `merged`: 22 → 21
- `draft`: 15 → 16
- Separately, resolve F-003 (S-7.03 status drift) first, then update summary accordingly.

---

#### F-003 — S-7.03 Status Field Inconsistency: STORY-INDEX = "completed", Story File = "ready" (MAJOR)

**Rule:** Rule 53 (Frontmatter Cross-Reference integrity)  
**Severity:** MAJOR  
**Status:** NEW

**Description:** S-7.03-tdd-discipline-hardening.md frontmatter declares `status: ready`. STORY-INDEX.md table shows `S-7.03 | ... | completed`. These are inconsistent. The STORY-INDEX footnote confirms PR #13 merged to develop at 4db2340 on 2026-04-26 (implementation complete), so the correct status should be `merged` in both places.

The `completed` value is non-canonical per STORY-INDEX policy (valid statuses: `draft, ready, in-progress, merged, partial, blocked`). `completed` is not in the declared valid values list.

**Evidence:**
- S-7.03-tdd-discipline-hardening.md line 9: `status: ready`
- STORY-INDEX.md line 121: status column = `completed`
- STORY-INDEX.md line 146: Valid status values list does not include `completed`
- STATE.md line 102: S-7.03 GREEN implementation COMPLETE (PR #13 merged)

**Remediation:**
1. Update S-7.03 frontmatter: `status: ready` → `status: merged`
2. Update STORY-INDEX table row: `completed` → `merged`
3. Update STORY-INDEX Summary: ready=3 → ready=3, merged=21+1=22 (after resolving F-002 merged=21 correction, the final count with S-7.03=merged would be 22 total merged stories)
4. Update STORY-INDEX summary: completed=1 → 0; merged=21 → 22; draft=16 → 16; ready=4 → 3

Note: If S-7.03 is truly not yet merged to `main` (only to `develop`), the status `partial` or an annotation is more appropriate. Triage should confirm merge state vs. branch.

---

#### F-004 — Dependency Graph Symmetry: S-1.04 and S-3.04 Asymmetric (MAJOR, CW-3)

**Rule:** CW-3 (STORY-INDEX dep edge symmetry)  
**Severity:** MAJOR  
**Status:** NEW

**Description:** Two dependency edge symmetry violations exist:

**Gap 1:** S-3.04 declares `depends_on: ["S-1.04"]`, meaning S-1.04 must complete before S-3.04. However, S-1.04 `blocks: ["S-1.05", "S-2.01", "S-3.01", "S-3.02", "S-3.03"]` — S-3.04 is not in S-1.04's `blocks` list. S-1.04 blocks S-3.01/02/03 (which depend on S-3.04 and thus transitively on S-1.04) but does not declare the direct blocking relationship to S-3.04.

**Gap 2:** S-4.07 declares `depends_on: ["S-3.01", "S-3.02", "S-3.03", "S-3.04", ...]`, meaning S-3.04 must complete before S-4.07. However, S-3.04 `blocks: ["S-3.01", "S-3.02", "S-3.03"]` — S-4.07 is not in S-3.04's `blocks` list.

**Evidence:**
- S-3.04 frontmatter: `depends_on: ["S-1.04"]`; S-1.04 frontmatter: `blocks: ["S-1.05", "S-2.01", "S-3.01", "S-3.02", "S-3.03"]` — S-3.04 absent
- S-4.07 frontmatter: `depends_on: ["S-3.01", "S-3.02", "S-3.03", "S-3.04", ...]`; S-3.04 frontmatter: `blocks: ["S-3.01", "S-3.02", "S-3.03"]` — S-4.07 absent

**Impact:** Missing edges don't create incorrect ordering (transitive deps still order correctly) but leave `blocks:` lists incomplete, which any tooling-based topological sort would treat as missing edges.

**Remediation:**
1. Add `S-3.04` to S-1.04 `blocks:` list
2. Add `S-4.07` to S-3.04 `blocks:` list
3. Update STORY-INDEX dep cells correspondingly

---

#### F-005 — 7 Stories Have Empty `functional_requirements:` (MAJOR)

**Rule:** Rule 15 (L1→L4 chain: story → FR)  
**Severity:** MAJOR  
**Status:** NEW (partial)

**Description:** Seven stories have empty `functional_requirements:` arrays, creating gaps in the L1→L4 traceability chain at the story→FR level:

| Story | Status | Expected FR | Justification |
|-------|--------|-------------|---------------|
| S-0.05 | merged | FR-036 (Rules and policy docs) | docs-skeleton story aligns with template/rules FR |
| S-1.01 | merged | N/A (justified) | STORY-INDEX explicitly notes "0 BCs, pure scaffolding justified" |
| S-5.05 | partial | FR-036 (Rules) or FR-035 (templates) | migration-guide docs aligns with templates/docs FR |
| S-5.06 | draft | FR-036 or FR-035 | semver-commitment-docs story |
| S-6.01 | ready | FR-041 (ADR authoring workflow) | create-adr skill story maps to FR-041 |
| S-7.01 | ready | FR-042 (Process self-improvement) | agent prompt discipline maps to FR-042 |
| S-7.02 | ready | FR-042 (Process self-improvement) | count-propagation hook maps to FR-042 |

S-1.01 is justified (pure scaffolding, no BCs per STORY-INDEX explicit note). The other 6 stories have identifiable FR anchors that were not populated.

**Evidence:**
- S-0.05, S-5.05, S-5.06, S-6.01, S-7.01, S-7.02, S-1.01 frontmatter: `functional_requirements:` empty or absent

**Remediation:** For stories S-0.05, S-5.05, S-5.06, S-6.01, S-7.01, S-7.02: populate `functional_requirements:` with the corresponding FR-NNN. S-1.01 may retain empty list with a justification comment.

---

### 4.2 MINOR Findings

---

#### F-006 — PRD FR Count Internal Inconsistency (MINOR — Pre-existing per wave reviews)

**Rule:** Rule 30 (PRD scope enforcement)  
**Severity:** MINOR  
**Status:** PRE-EXISTING (D-028 / D-030 lineage)

**Description:** The PRD contains three conflicting FR counts:
- PRD §2 header (line 47): "43 logical FRs"
- PRD §14 status table (line 1337): "FRs defined: 43"
- PRD §7 RTM footer (line 1083): "Total: 44 FRs across 10 subsystems"

Actual count: 44 unique FR IDs (FR-001..FR-044, with FR-043 appearing as 3 cross-SS subsection headings + 1 summary row = 1 logical FR, and FR-044 appearing once). The §7 RTM count of 44 is correct; the §2 header and §14 count of 43 are stale (FR-044 was added in Wave 2 for per-sink resilience).

**Triage note:** D-028 and D-030 document this as a process-gap finding deferred to v1.1. Per sweep brief, flag as MINOR with deferral rationale rather than CRITICAL.

**Remediation:** Update PRD §2 header and §14 status table: 43 → 44.

---

#### F-007 — Wave 8 Stories (S-0.05, S-5.05, S-5.06): F-305 Section Ordering Violation (MINOR — Documented Pending Intent)

**Rule:** CW-1 (Convention drift across waves), F-305  
**Severity:** MINOR  
**Status:** DOCUMENTED PENDING INTENT (Wave 8 pass-4 F-301)

**Description:** Wave 8 stories S-0.05, S-5.05, S-5.06 have `## v1.1 BC/VP Candidates` section appearing BEFORE `## Acceptance criteria` and BEFORE `## Acceptance Criteria with BC Traces`, violating the F-305 canonical ordering convention (v1.1 BC/VP Candidates MUST appear AFTER Acceptance Criteria with BC Traces, BEFORE Architecture Compliance Rules).

Correct ordering (per F-305): Behavioral Contracts → Verification Properties → Acceptance Criteria with BC Traces → **v1.1 BC/VP Candidates** → Architecture Compliance Rules

Actual ordering in Wave 8 stories: Capability Anchor Justification → Behavioral Contracts → Verification Properties → **v1.1 BC/VP Candidates** → Acceptance criteria → Acceptance Criteria with BC Traces

Wave 6 stories (S-2.03, S-2.04, S-2.08), Wave 7 stories (S-0.02, S-4.08, S-5.07), and Wave 9 story (S-2.07) all comply with F-305 ordering.

**Evidence:**
- S-0.05: `## v1.1 BC/VP Candidates` at line 74, before `## Acceptance criteria` at line 83
- S-5.05: `## v1.1 BC/VP Candidates` at line 66, before `## Narrative` at line 75
- S-5.06: `## v1.1 BC/VP Candidates` at line 63, before `## Narrative` at line 73
- Wave 8 adversarial pass-4 verdict: F-301 LOW pending intent; 3_of_3 NITPICK achieved despite this

**Triage note:** This was a documented "pending intent" (LOW) at Wave 8 convergence and explicitly did not reset the convergence clock. Flag as MINOR/pending-intent; remediation is cosmetic.

---

#### F-008 — S-7.03 Stretch-Anchor Disclosure Position in Wave 9 Story S-2.07 (MINOR, CW-1)

**Rule:** CW-1 (Stretch-Anchor Disclosure shape uniformity)  
**Severity:** MINOR  
**Status:** NEW

**Description:** Wave 9 story S-2.07 places `## Stretch-Anchor Disclosure` AFTER `## Architecture Compliance Rules` (line 161 vs line 154). In Wave 7 stories S-0.02, S-4.08, S-5.07, the `## Stretch-Anchor Disclosure` section appears BEFORE `## Behavioral Contracts`. The positioning is non-uniform across waves.

Note: S-2.07's Stretch-Anchor Disclosure explains that no stretch-anchor is present (the primary BCs SS-01 match the story's SS-01 subsystem); the section's purpose is disclosure, not remediation. However, its position after Arch Compliance Rules deviates from the Waves 7/8 placement pattern.

**Evidence:**
- S-2.07: `## Stretch-Anchor Disclosure` at line 161, after `## Architecture Compliance Rules` at line 154
- S-0.02: `## Stretch-Anchor Disclosure` at line 160, before `## Behavioral Contracts` at line 170

**Remediation:** Move `## Stretch-Anchor Disclosure` in S-2.07 to before `## Behavioral Contracts` for uniformity with Wave 7 established convention.

---

#### F-009 — S-0.05 Missing `functional_requirements:` and `capabilities:` in Frontmatter (MINOR, Overlaps F-005)

**Rule:** Rule 18 (Canonical frontmatter)  
**Severity:** MINOR  
**Status:** NEW (subset of F-005)

**Description:** S-0.05 frontmatter includes a `capabilities: ["CAP-014"]` field (Wave 8 added) but does not have a `functional_requirements:` field populated. This is noted in F-005 above. Additionally, S-0.05 uses a non-standard `capabilities:` frontmatter field that appears only in a few stories (S-2.07, S-3.04, S-5.05, S-5.06) without being part of the canonical template. This cross-story inconsistency in frontmatter schema use is a minor nit.

---

#### F-010 — BC-INDEX traces_to Field Points to bc-id-mapping.md Rather Than BC-INDEX Parent (MINOR)

**Rule:** Rule 18, Rule 22 (detail files trace to index)  
**Severity:** MINOR  
**Status:** PRE-EXISTING

**Description:** BC-INDEX.md frontmatter `traces_to: bc-id-mapping.md`. The canonical pattern per DF-021 is that an index file should `traces_to` its parent phase or domain artifact. `bc-id-mapping.md` is a legacy ID mapping file, not the expected parent (which would be something like `phase-1-spec-crystallization` or `PRD.md`). This is the same pattern visible in all 1,891 BC file frontmatters which also `traces_to: bc-id-mapping.md`.

This is consistent across all BC files (uniform behavior) and does not break navigation, but does not conform strictly to the traces_to hierarchy described in DF-021. Pre-existing from Phase 1.4b migration.

---

#### F-011 — PRD Header and §14 FR Count Stale — S-7.03 Milestone References Stale (MINOR — D-030)

**Rule:** Rule 53 (Frontmatter cross-reference integrity)  
**Severity:** MINOR  
**Status:** ACKNOWLEDGED (D-030)

**Description:** D-030 documented that PRD §1.2 milestone references are stale (beta.4 → should be beta.6 or later) and that CAP-028 outcome string references 1.0.0-beta.4. These are release-cycle drift items from D-030 deferred to v1.1. Confirming they are still present and unresolved, as expected.

**Triage note:** Cite D-030; MINOR with deferral rationale per sweep brief.

---

#### F-012 — Wave 9 Story S-2.07 Has `blocks: ["S-2.08"]` But S-2.08 `depends_on` Already Includes S-2.07 — No STORY-INDEX Sync Issue Found (INFO)

**Rule:** CW-3  
**Severity:** INFO  
**Status:** PASS

**Description:** S-2.07 `blocks: ["S-2.08"]` and S-2.08 `depends_on` includes `S-2.07`. This edge is fully symmetric. State-manager fixed STORY-INDEX in Wave 9 pass-1 fix burst (F-002 there). Verified clean. Logging as INFO to confirm the Wave 9 fix was correctly applied.

---

## 5. Cross-Wave-Specific Findings (CW-1 through CW-6)

### CW-1: Convention Drift Across Waves

**Verdict:** MOSTLY PASS with 2 minor deviations (documented as F-007 and F-008 above)

| Convention | Waves 1-6 | Waves 7-9 | Uniformity |
|-----------|-----------|-----------|------------|
| F-206 5-col v1.1 BC/VP candidate table | Wave 6 first introduced; `Candidate ID \| Type \| Title \| Rationale \| Blocking?` format | Waves 7/8/9 use consistent 5-col format | PASS |
| F-305 section ordering (v1.1 before Arch Compliance) | Wave 6 compliant (F-305 introduced there) | Wave 7/9 compliant; Wave 8 NON-COMPLIANT (F-301 pending intent) | MINOR (F-007) |
| F-204 cross-wave complementary HTML comment shape | Wave 6: `F-001 (Wave N) description` shape; Wave 8: `F-002 (Wave 8 pass-1)` shape | Wave 7: `F-204 (Wave 7 pass-3)` reference format | PASS — all cite sanctioned finding IDs |
| Stretch-Anchor Disclosure section placement | Wave 7: before Behavioral Contracts | Wave 9: after Architecture Compliance Rules | MINOR (F-008) |
| `[process-gap]` annotation format | Uniform across all waves | Uniform | PASS |
| `v1.1 BC Candidates` section name | Waves 1-6: `## v1.1 BC Candidates` | Waves 6-9 (explicitly): `## v1.1 BC/VP Candidates` | Slight name drift but semantically equivalent; no corrective action needed |

**Sampled stories per wave (CW-1):**
- Wave 1: S-1.02 (dispatcher-core) — has `## v1.1 BC Candidates`, no Stretch-Anchor, no 5-col table (pre-F-206)
- Wave 2: S-1.08 (sink-file-driver) — has `## v1.1 BC Candidates`, no Stretch-Anchor
- Wave 3: S-3.01 (port-capture-commit-activity) — has v1.1 candidates, F-001 sanctioned-template-anchor cited
- Wave 4: S-1.03 (hook-sdk-crate) — has `## v1.1 BC Candidates`, VP anchors
- Wave 5: S-2.06 (activation-skill-integration) — has `## v1.1 BC Candidates`, F-007 pattern
- Wave 6: S-2.03 (ci-cross-platform-matrix) — F-305 compliant, 5-col candidates
- Wave 7: S-0.02 (release-workflow-prerelease) — Stretch-Anchor before BCs, F-305 compliant
- Wave 8: S-0.05 (docs-scaffolding) — F-301 section ordering violation (documented pending intent)
- Wave 9: S-2.07 (regression-test-validation) — Stretch-Anchor after Arch Compliance (F-008)

---

### CW-2: BC-INDEX Bidirectional Integrity Post-Cycle

**Verdict:** PASS

| Check | Result |
|-------|--------|
| BC-INDEX total_bcs vs actual file count | 1,891 = 1,891 PASS |
| BC-INDEX detail rows vs filesystem (per SS) | All match within POLICY 1 conventions (documented SS-05/SS-08 logical/physical split) |
| BC-INDEX SS-09 (5 BCs) vs ss-09/ directory | BC-9.01.001..005 present, all 5 referenced in index |
| BC-INDEX SS-10 (58 BCs) vs ss-10/ directory | 58 rows, 58 files: BC-10.01..12 families, all present |
| BC-10.13 v1.1 candidates in BC-INDEX | NOT present (correct: BC-10.13.001-012 are future-cycle candidates, not yet created) |
| BC-8.29.001/002/003, BC-8.30.002 in SS-05 section | Present in SS-05 section per D-013/D-014 POLICY 1 |
| ARCH-INDEX SS-05=646, SS-08=218 consistent with BC-INDEX | PASS (both use logical post-reanchor counts) |

---

### CW-3: STORY-INDEX Dep Edge Symmetry

**Verdict:** FAIL — 2 asymmetric edges found (F-004)

| Edge | S-A blocks S-B? | S-B depends_on S-A? | Symmetric? |
|------|----------------|---------------------|------------|
| S-1.01 → S-1.02 | YES | YES | PASS |
| S-1.01 → S-1.03 | YES | YES | PASS |
| S-1.02 → S-2.03 | YES | YES | PASS |
| S-1.04 → S-3.04 | **NO** | YES (S-3.04 depends_on S-1.04) | **FAIL (F-004 Gap 1)** |
| S-3.04 → S-4.07 | **NO** | YES (S-4.07 depends_on S-3.04) | **FAIL (F-004 Gap 2)** |
| S-2.01 → S-2.02 | YES | YES | PASS |
| S-2.01 → S-2.07 | YES | YES | PASS |
| S-2.02 → S-2.07 | YES | YES | PASS |
| S-4.08 → S-5.01..06 | YES (all 6) | YES (all 6) | PASS |
| S-0.02 → S-2.08 | YES | YES | PASS |
| S-0.02 → S-4.08 | YES | YES | PASS |
| S-0.02 → S-5.07 | YES | YES | PASS |
| All E-6/E-7 stories | No deps declared | No deps (correct) | PASS |

---

### CW-4: PRD §FR Rows vs Stories FR Traces

**Verdict:** PASS (with noted gaps in F-005)

Stories reference FR-NNN IDs matching PRD §7 RTM rows. Verified:
- FR-001, FR-002, FR-007: S-1.02 (correct; covers dispatcher core + routing + legacy hook routing)
- FR-037: All release-workflow stories (correct; covers activation + release-tooling discipline)
- FR-043: S-7.03 only (correct; sole FR-043 story)
- FR-044: S-4.01..S-4.05 (correct; per-sink resilience stories)
- FR-007: S-2.07, S-5.01-04 (correct; legacy hook routing + new hook events)

The reverse check (PRD §7 Stories column cites all stories tracing to that FR): PRD §7 rows do not enumerate story IDs by column — stories are cited inline in PRD §2 section prose and wave-anchor HTML comments. FR-007 inline comment on PRD line 1043 cites S-2.07 (Wave 9 anchor). No formal reverse-trace column exists to audit; this is a pre-existing structural gap documented under TD-001.

F-005 (7 stories with empty FR fields) is the main gap. Non-blocking for existing traced stories; MAJOR for spec completeness.

---

### CW-5: Capabilities Subsystem-Column Drift

**Verdict:** FAIL — 3 undisclosed drifts in L2-INDEX (covered by F-001)

| CAP | capabilities.md subsystems | L2-INDEX cross-walk | Disclosed? |
|-----|---------------------------|---------------------|-----------|
| CAP-007 | SS-06, SS-09 | L2-INDEX SS-01 still lists CAP-007 | NO — undisclosed in L2-INDEX (F-001) |
| CAP-009 | SS-02 | L2-INDEX SS-01 still lists CAP-009 | NO — undisclosed in L2-INDEX (F-001) |
| CAP-018 | SS-05, SS-06 | L2-INDEX SS-05 missing CAP-018 | NO — undisclosed in L2-INDEX (F-001) |
| CAP-028 | SS-06, SS-09 | L2-INDEX SS-06 missing CAP-028 | Partially (PRD §8 has F-101 disclosure comment) |
| CAP-003 | SS-01, SS-03, SS-10 | PRD §8 + capabilities.md aligned | PASS |
| CAP-008 | SS-01, SS-02, SS-04, SS-07 | PRD §8 has F-208 disclosure comment | PASS (disclosed) |
| CAP-013 | SS-01, SS-04, SS-07 | PRD §8 has F-207 disclosure comment | PASS (disclosed) |
| CAP-017 | SS-06, SS-08, SS-10 | PRD §8 has F-301 disclosure comment | PASS (disclosed) |

**Conclusion:** The Wave 6 F-302/F-005 fixes updated capabilities.md and PRD §8 but NOT L2-INDEX. L2-INDEX remains the only artifact with stale subsystem-column data. This is the same drift class as D-030 (arch-doc ID scheme staleness) but for L2-INDEX rather than architecture docs.

---

### CW-6: Cross-Wave HTML Disclosure Comments

**Verdict:** PASS

All HTML comments found in PRD.md, capabilities.md, and ARCH-INDEX.md cite sanctioned finding IDs or wave-anchor annotations:

**PRD.md HTML comments verified:**
- Line 246: `<!-- Wave 9 SS-01 straggler re-anchor: ... -->` — wave anchor (sanctioned)
- Line 697: `<!-- F-008 (Wave 8 pass-1): ... -->` — finding citation (sanctioned)
- Line 727: `<!-- Wave 7 SS-10 cross-wave anchor: ... -->` — wave anchor (sanctioned)
- Line 1043: `<!-- Wave 9: ... --><!-- F-004 (Wave 9 pass-1): ... -->` — dual citation (sanctioned)
- Line 1073: `<!-- F-201 (Wave 8 pass-3): ... -->` — finding citation (sanctioned)
- Line 1102: `<!-- F-208 (Wave 6 pass-3): ... -->` — finding citation (sanctioned)
- Line 1108: `<!-- F-207 (Wave 6 pass-3): ... -->` — finding citation (sanctioned)
- Line 1110: `<!-- F-101 (Wave 8 pass-2): ... -->` — finding citation (sanctioned)
- Line 1114: `<!-- F-301 (Wave 6 pass-4): ... -->` — finding citation (sanctioned)
- Line 1116: `<!-- F-302 (Wave 6 pass-4): ... -->` — finding citation (sanctioned)
- Line 1127: `<!-- F-101 (Wave 6 pass-2): ... -->` — finding citation (sanctioned)
- Line 1128: `<!-- F-002 (Wave 7 pass-1): ... -->` — finding citation (sanctioned)

**capabilities.md HTML comments verified:**
- Line 46: `<!-- [process-gap] F-007 fix (Wave 5 SS-06 re-anchor): ... -->` — sanctioned
- Line 51: `<!-- [process-gap] F-102 fix (Wave 3 SS-04 pass-2): ... -->` — sanctioned
- Line 63: `<!-- [process-gap] F-007: SS-01 added as dominant implementer ... -->` — sanctioned
- Line 71: `<!-- [process-gap] F-103 fix (Wave 3 SS-04 pass-2): ... -->` — sanctioned
- Line 78: `<!-- F-009 (Wave 8 pass-1): ... -->` — sanctioned
- Line 87: `<!-- Expanded SS-09 → SS-06,SS-09 per Wave 6 F-005 ... -->` + 2 others — sanctioned
- Line 129: `<!-- F-302 (Wave 6 pass-4): SS-05 added ... -->` — sanctioned

**SS-09 BC files:** No HTML comments (clean).

All HTML disclosure comments resolve to real finding IDs and sanctioned patterns. No orphaned or unsanctioned comments found.

---

## 6. Pre-Existing Acknowledged Findings (TD-NNN References)

The following findings from prior audit phases are acknowledged and NOT re-raised as new issues:

| TD ID | Description | Severity | Deferral |
|-------|-------------|----------|----------|
| TD-001 | BC-level CAP/DI/Stories anchoring incomplete (1,891 BCs carry TBD defaults) | P2 | v1.0.1 |
| TD-002 | BC-INDEX status column all "draft" regardless of implementation state | P2 | v1.0.1 |
| TD-003 | BC frontmatter lacks per-BC lifecycle_status field | P3 | v1.1 |
| TD-004 | BC-7.01 family mixed; FR-032 BC-group labeling conflict | P2 | v1.0.1 |
| TD-005 | Agent registry missing; NFR-PERF not in PRD §4.2 top-5 | P3 | v1.1 |
| TD-006 | validate-consistency Check 8/9 missing executable runner, tests, TDD trail | P1 | v1.0.1 |
| D-013 | SS-08→SS-05 BC reanchor POLICY 1 convention (files in ss-08/, logical in ss-05/) | INFO | Permanent per POLICY 1 |
| D-028 | PRD per-SS footer counts drifted (not in validate-count-propagation.sh hook scope) | INFO | v1.1 |
| D-030 | PRD §1.2 milestone stale (beta.4); CAP-028 outcome stale; arch-doc BC ID scheme | INFO | v1.1 |

**TD-006-pre-emptive note:** Any consistency finding detectable by Check 8/9 once the runners exist (e.g., BC canonical TV ↔ emitter field gaps) is tagged TD-006-pre-emptive and deferred to that effort. No such findings were detected in this sweep.

---

## 7. Summary of All Findings

### MAJOR Findings (5)

| ID | Description | Rule | Remediation |
|----|-------------|------|-------------|
| F-001 | L2-INDEX Subsystem Cross-Walk stale: SS-01 has CAP-007/CAP-009 erroneously; SS-05 missing CAP-018; SS-06 missing CAP-028 | Rule 35, CW-5 | Update 3 rows in L2-INDEX.md Subsystem Cross-Walk |
| F-002 | STORY-INDEX summary status counts wrong: merged=22 (actual 21), draft=15 (actual 16) | Rule 31 | Correct summary table after resolving F-003 |
| F-003 | S-7.03 status field drift: STORY-INDEX=completed (non-canonical value), file=ready; actual state=merged | Rule 53 | Update S-7.03 frontmatter and STORY-INDEX to `merged`; update summary |
| F-004 | Dep graph symmetry gaps: S-1.04 not in S-3.04 blocks; S-3.04 not in S-4.07 blocks | CW-3 | Add S-3.04 to S-1.04 blocks; add S-4.07 to S-3.04 blocks |
| F-005 | 7 stories with empty functional_requirements (6 missing real FR anchors; 1 justified) | Rule 15 | Populate FR-NNN in 6 stories; document S-1.01 justification in-story |

### MINOR Findings (7)

| ID | Description | Rule | Remediation |
|----|-------------|------|-------------|
| F-006 | PRD §2 and §14 say "43 FRs"; §7 RTM says "44 FRs" (pre-existing D-028 lineage) | Rule 30 | Update §2 header and §14 to 44; MINOR/deferred |
| F-007 | Wave 8 stories (S-0.05, S-5.05, S-5.06) violate F-305 section ordering (pending intent from Wave 8 pass-4) | CW-1 | Move v1.1 candidates section to after AC-traces; cosmetic |
| F-008 | Wave 9 story S-2.07 places Stretch-Anchor Disclosure after Arch Compliance (Wave 7 convention = before BCs) | CW-1 | Move section to before Behavioral Contracts |
| F-009 | S-0.05 `capabilities:` frontmatter field used non-uniformly across stories | Rule 18 | Standardize or document field in template; low priority |
| F-010 | BC-INDEX and BC files traces_to bc-id-mapping.md rather than spec hierarchy parent (pre-existing) | Rule 22 | Deferred to BC backfill (TD-001 wave) |
| F-011 | PRD milestone references stale (beta.4 → beta.7; CAP-028 outcome) — D-030 acknowledged | Rule 53 | Deferred per D-030 to v1.1 |
| F-012 | prd-supplements/ directory absent (deferred structural gap documented in PRD frontmatter) | Rule 66 | Pre-existing deferral; track under existing PRD footnote |

---

## 8. Consistency Score by Domain

| Domain | Items Checked | Pass | Fail/Warn | Score |
|--------|--------------|------|-----------|-------|
| Numerical counts (BC, VP, Story, CAP, DI) | 22 | 20 | 2 | 91% |
| Sharding integrity (all indexes) | 5 | 5 | 0 | 100% |
| Frontmatter canonical fields (45 stories) | 45 | 45 | 0 | 100% |
| Dep graph symmetry | 13 edges checked | 11 | 2 | 85% |
| L1→L4 FR traceability chain | 45 stories | 38 | 7 | 84% |
| VP-INDEX arithmetic | 3 checks | 3 | 0 | 100% |
| Cross-wave convention uniformity | 8 conventions | 6 | 2 | 75% |
| HTML disclosure comment validity | ~18 comments | 18 | 0 | 100% |
| CAP subsystem cross-walk | 28 CAPs | 25 | 3 | 89% |
| BC-INDEX bidirectional integrity | 10 shards | 10 | 0 | 100% |
| **Overall** | **~200 checks** | **~181** | **~19** | **~85%** |

---

## Validation Gate: PASS | FAIL

**Gate Verdict: FINDINGS_REMAIN**

No CRITICAL-severity findings exist. Gate does not hard-block on MINOR findings alone.

**MAJOR findings requiring triage before convergence:**

| Finding | Resolution Path | Blocking? |
|---------|----------------|-----------|
| F-001 (L2-INDEX stale CAP subsystems) | Edit L2-INDEX.md (3 rows); no downstream cascade | Yes — closes a known spec drift |
| F-002 (STORY-INDEX summary counts wrong) | Edit summary table; depends on F-003 | Yes — with F-003 |
| F-003 (S-7.03 status=completed/ready vs merged) | Update S-7.03 frontmatter + STORY-INDEX table | Yes — spec/status consistency |
| F-004 (dep graph asymmetry) | Add 2 missing blocks entries; update STORY-INDEX dep cells | Yes — tooling-visible gap |
| F-005 (empty FR fields on 6 stories) | Populate FR-NNN on 6 stories | Yes — L1→L4 chain gap |

**MINOR findings (non-blocking for TD-006 gate):**
- F-006 through F-012 are cosmetic, pre-existing, or acknowledged. None block factory-dispatcher next-phase work.

**Recommended next step:** Triage findings F-001..F-005. All five are low-effort fixes (targeted file edits, no spec-level redesign required). After resolving, re-run a targeted consistency check on the modified artifacts, then proceed to TD-006 POLICY 11/12 gating and factory-dispatcher next-phase work.

---

## Appendix A: Scope Notes

- Rules 22–24 (UI/design system) and Rule 36 (UX sharding) were skipped per sweep brief. vsdd-factory is a CLI-only product with no UI surfaces. This is consistent with prior wave adversarial reviews and STATE.md Skip Log.
- All 45 stories were individually verified for frontmatter field presence. BC file spot-checks covered SS-01 (BC-1.01.001, BC-1.07.001), SS-09 (BC-9.01.001), and SS-10 (sample). Full BC-level canonical TV and emitter consistency checking deferred to TD-006 runner implementation.
- Dependency graph verified bidirectionally for all 45 stories against their `depends_on` / `blocks` counterparts. Two asymmetric edges found (F-004).
- HTML comment audit covered PRD.md, capabilities.md, ARCH-INDEX.md, and SS-09 BC files. Stories were not scanned for HTML comments (beyond the documented F-204/F-002 patterns verified in Waves 7/8).

---

## Appendix B: Artifact Versions Audited

| Artifact | Version / Timestamp | Producer |
|----------|---------------------|---------|
| STATE.md | 2026-04-27T00:00:00Z | state-manager |
| PRD.md | (per PRD §14, v1.0 equivalent) | create-prd / state-manager |
| BC-INDEX.md | 1.0 / 2026-04-26T00:00:00 | state-manager |
| VP-INDEX.md | 1.0 / 2026-04-26T00:00:00 | architect |
| ARCH-INDEX.md | 1.0 / 2026-04-25T00:00:00 | architect |
| L2-INDEX.md | 1.0 / 2026-04-25T00:00:00 | business-analyst |
| STORY-INDEX.md | 1.0 / 2026-04-27T12:00:00 | story-writer |
| Stories (Wave 9 final) | v1.2 (all 45 post-re-anchor) | product-owner |
| tech-debt-register.md | 1.0 / 2026-04-25T00:00:00 | state-manager |
