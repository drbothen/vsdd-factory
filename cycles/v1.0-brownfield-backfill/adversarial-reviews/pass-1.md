---
document_type: adversarial-review-pass
level: ops
producer: adversary
phase: 1d
pass: 1
timestamp: 2026-04-25T20:00:00Z
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/PRD.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/policies.yaml
traces_to: phase-1d-pass-2
---

# Adversarial Review Pass 1 — Phase 1 Spec Package

## Coverage

Read all 6 indexes (PRD, ARCH-INDEX, BC-INDEX header + SS-01/02/03 sections, L2-INDEX, VP-INDEX, STORY-INDEX), invariants.md, plus 5 sample BCs (BC-1.05.001, BC-5.01.001, BC-6.01.001, BC-7.01.001, BC-8.01.001), 3 sample VPs (VP-001, VP-020, VP-044), and ADR-001-rust-dispatcher.md.

## Policy Rubric (10 baseline policies loaded from `.factory/policies.yaml`)

POLICY 1 `append_only_numbering` (HIGH); POLICY 2 `lift_invariants_to_bcs` (MEDIUM); POLICY 3 `state_manager_runs_last` (HIGH); POLICY 4 `semantic_anchoring_integrity` (MEDIUM); POLICY 5 `creators_justify_anchors` (MEDIUM); POLICY 6 `architecture_is_subsystem_name_source_of_truth` (HIGH); POLICY 7 `bc_h1_is_title_source_of_truth` (HIGH); POLICY 8 `bc_array_changes_propagate_to_body_and_acs` (HIGH); POLICY 9 `vp_index_is_vp_catalog_source_of_truth` (HIGH); POLICY 10 `demo_evidence_story_scoped` (HIGH)

## Findings Summary

| Severity | Count | Adjusted (F-001 false positive) |
|----------|-------|-------------------------------|
| CRITICAL | 2 | 1 |
| HIGH | 7 | 7 |
| MEDIUM | 5 | 6 (F-001 downgraded) |
| LOW | 3 | 3 |
| **Total** | 17 | 17 |

## Findings

### F-001 [DOWNGRADED to LOW — FALSE POSITIVE] — Story file lookup pattern

**Original claim:** Story files referenced by STORY-INDEX do not exist on disk.
**Verification (post-pass):** 41 story files DO exist at `.factory/stories/S-N.MM-<short>.md` (with descriptive filename suffixes per Phase 1.8 migration). The adversary searched for `.factory/stories/S-1.01.md` without the descriptive suffix and got "file not found." Verified via `find .factory/stories -maxdepth 1 -name "S-*.md" | wc -l` = 41.
**Action:** Downgraded to LOW. STORY-INDEX could clarify the filename convention for future readers. Filename pattern is `S-N.MM-<short-description>.md`.

### CRITICAL

**F-002 — Systemic VP-001 mis-anchoring in BC files**
- Files: `ss-06/BC-6.01.001.md` line 73, 98 cites VP-001 as "Only the literal token NITPICK..."; `ss-08/BC-8.01.001.md` line 70 cites VP-001 as "TBD — promote acceptance criterion to a structural/lint test."
- VP-001 actually defines: "Tier Execution Is Sequential; Intra-Tier Is Parallel" (DI-001, SS-01) per VP-INDEX line 50.
- Two unrelated BCs in SS-06 and SS-08 erroneously cite VP-001. POLICY 4 violation.
- Suggested fix: sweep all 1,851 BC files for stale VP-NNN citations from template defaults; replace with TBD or correct VP.
- Confidence: HIGH

### HIGH

**F-003 — Systemic CAP-TBD / DI-TBD / Stories-TBD in BC files**
- All 5 sampled BCs (across SS-01/05/06/07/08) show `capability: CAP-TBD`, `L2 Domain Invariants: TBD`, `Stories: TBD`.
- PRD §8 claims CAP→BC anchoring; BCs themselves don't carry the anchoring.
- POLICY 2/4/5 violation.
- Suggested fix: Phase 1.5 follow-up CAP/DI anchoring sweep across 1,851 BCs (substantial — defer if scope too large for current cycle).

**F-004 — VP-INDEX cites verification-architecture.md / verification-coverage-matrix.md (don't exist)**
- VP-INDEX line 16-22 declares VP-INDEX authoritative for these 2 files; ARCH-INDEX deferred them.
- POLICY 9 verification step ("VP appears in verification-architecture.md") is unrunnable.
- Suggested fix: Either create stub verification-architecture.md (auto-generated from VP-INDEX) or amend POLICY 9 to acknowledge deferred-files.

**F-005 — ARCH-INDEX "Approx BCs" diverges from BC-INDEX actuals**
- ARCH-INDEX line 73-83: SS-08 ~130, SS-10 ~143; BC-INDEX: SS-08 215, SS-10 58. Big swap.
- Pre-gate consistency-validator d5b1e7e was supposed to fix counts.
- Suggested fix: Sync ARCH-INDEX "Approx BCs" column to BC-INDEX exact counts. Investigate SS-08↔SS-10 misallocation.
- **STATUS: FIXED in this commit** (ARCH-INDEX synced to BC-INDEX exact counts; column renamed from "Approx BCs" to "BCs")

**F-006 — BC-INDEX status column all "draft"**
- 1,851 rows: status=draft, capability=CAP-TBD, stories=TBD.
- PRD asserts FR-level shipped/partial/pending; BC-INDEX has no per-BC traceability backstop.
- Suggested fix: Backfill BC-INDEX status column from PRD FR status + STORY-INDEX. At minimum, populate the 22 merged stories' BCs as `shipped`.

**F-007 — STORY-INDEX title says "greenfield" in brownfield project**
- STORY-INDEX.md line 16: `# Story Index — vsdd-factory v1.0.0-greenfield`.
- PRD frontmatter line 9 sets `origin: brownfield`; cycle is `v1.0-brownfield-backfill`.
- Suggested fix: Rename heading to `# Story Index — vsdd-factory v1.0 (brownfield)`.
- **STATUS: FIXED in this commit**

**F-008 — VP-INDEX has 0 kani / 0 proptest VPs**
- VP-INDEX lines 43-44: kani=0, property-test=0; security-critical VPs (VP-005 setuid, VP-021 deny-by-default, VP-023 truncated buffer) are unit-test only.
- PRD KD-002 implies sandbox security; VPs are unit-test only.
- Suggested fix: Either downgrade KD-002 wording or commit VP-020/VP-023/VP-042 to kani in Phase 1.6c.

**F-009 — VPs without DI link (orphans): VP-024, VP-048, VP-053-VP-057**
- VP-INDEX Full Index shows these have `—` in Domain Invariant column.
- Suggested fix: Either retrofit DI for orphan VPs, or explicitly mark "non-DI-derived VPs" as a category.

### MEDIUM

**F-010 — PRD §1.5 typo "securecomplex" (line 123)**
- PRD §1.5 Out of Scope: `securecomplex authentication at the dispatcher boundary`.
- "securecomplex" appears nowhere else; corrupted token.
- Suggested fix: Replace with intended phrase ("Complex authentication" or similar).
- **STATUS: FIXED in this commit** (replaced with "Complex authentication")

**F-011 — PRD §2.1 FR-005 "partial" status; BC file has no status field**
- PRD FR-005 claims partial status (DRIFT-001 anchor); BC-1.05.001 has no `status:` field.
- POLICY 5 (`creators_justify_anchors`) requires substantiation.
- Suggested fix: Add `lifecycle_status:` to BC frontmatter or per-BC status field.

**F-012 — PRD §2 vs §7 CAP traceability mismatch for FR-005**
- §2.1 header: SS-01 = 5 CAPs; §7 row FR-005 = 2 CAPs. Conflict.
- Suggested fix: Use per-FR CAP listing (§7 matrix) as source of truth; remove §2.1 header CAP enumeration.

**F-013 — BC-7.01.001 H1 vs PRD FR-032 disagreement**
- BC-7.01.001 H1: "block-ai-attribution blocks git commit messages with AI attribution"
- PRD FR-032 line 587: "BC-7.01.001–NNN | protect-secrets.sh: pattern-based secret detection"
- POLICY 7 violation (BC H1 != PRD claim about BC-7.01).
- Suggested fix: Either re-shard BC-7.01 (move block-ai-attribution to BC-7.07; move protect-secrets to BC-7.01), or amend PRD FR-032/FR-033.

**F-014 — Self-referential blind spot: no spec on concurrent self-modification race**
- vsdd-factory IS its own product. Phase 3 TDD edits hooks-registry.toml or hook scripts; dispatcher invoked on tool calls during agent work.
- No DI guards against script_path rewrite during dispatch.
- Suggested fix: Add DI-018 "registry/hook-script changes during dispatch are deferred to next dispatcher invocation" or DRIFT item flagging the risk.

### LOW

**F-001 (DOWNGRADED) — Story file lookup pattern** (see top — file convention is `S-N.MM-<short>.md`, not `S-N.MM.md`)

**F-015 — VP-053..VP-057 all "manual" proof method**
- Workflow VPs are manual; could be automated via lobster-parse lint.
- Flag for future hardening.

**F-016 — 34 agents not enumerated canonically**
- PRD §1.2: "34 specialist LLM sub-agents"; ARCH-INDEX SS-05 describes them; no `agents.md` registry.
- Suggested fix: Add `agents.md` under domain-spec/ or reference enumeration.

**F-017 — NFR-PERF-NNN missing from PRD §4.2 top-5 priority NFRs**
- §4.1 lists 16 perf NFRs (largest category); §4.2 top-5 doesn't include any NFR-PERF.
- Suggested fix: Add >=1 NFR-PERF-NNN to top-5 or expand to top-7.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 16 |
| **Duplicate/variant findings** | 1 (F-001 false positive, downgraded to LOW) |
| **Novelty score** | 1.0 (first pass; all findings new) |
| **Median severity** | 3.0 (HIGH) |
| **Trajectory** | 17 (pass 1 baseline) |
| **Verdict** | FINDINGS_REMAIN |

## Triage routing for Pass 2 dispatch

- F-002 (VP-001 mis-anchoring): sweep BC files for stale VP citations
- F-005 (ARCH-INDEX count drift): mechanical — FIXED this commit
- F-007 (greenfield→brownfield label): mechanical — FIXED this commit
- F-010 (typo): mechanical — FIXED this commit
- F-013 (BC-7.01 H1 mismatch): investigate, possibly re-shard
- F-003, F-006 (CAP/DI/Stories systemic TBD): wave-scale follow-up; defer if too large for cycle
- F-004, F-008, F-009, F-014: spec evolution / next cycle
