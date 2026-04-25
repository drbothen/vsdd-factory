---
document_type: adversarial-review-pass
level: ops
producer: adversary
phase: 1d
pass: 2
timestamp: 2026-04-25T22:30:00Z
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/PRD.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/tech-debt-register.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/pass-1.md
traces_to: phase-1d-pass-3
---

# Adversarial Review Pass 2 — Phase 1 Spec Package

## Coverage

Full re-read of all 6 indexes plus: invariants.md, capabilities.md, VP-INDEX.md (body),
VP-053.md, SS-05-orchestration.md, BC-5.20.001.md, tech-debt-register.md, PRD.md
§10.4/§11. Cross-checked pass-1 findings disposition against current file state.

## Policy Rubric

POLICY 1 `append_only_numbering` (HIGH); POLICY 2 `lift_invariants_to_bcs` (MEDIUM);
POLICY 3 `state_manager_runs_last` (HIGH); POLICY 4 `semantic_anchoring_integrity` (MEDIUM);
POLICY 5 `creators_justify_anchors` (MEDIUM); POLICY 6 `architecture_is_subsystem_name_source_of_truth` (HIGH);
POLICY 7 `bc_h1_is_title_source_of_truth` (HIGH); POLICY 8 `bc_array_changes_propagate_to_body_and_acs` (HIGH);
POLICY 9 `vp_index_is_vp_catalog_source_of_truth` (HIGH); POLICY 10 `demo_evidence_story_scoped` (HIGH)

## Pass-1 Follow-Up Audit

| Finding | Severity | Disposition | Status |
|---------|----------|-------------|--------|
| F-001 | LOW | Downgraded to false positive in pass-1; deferred filename convention note | ACCEPTED/DEFERRED → F-027 (this pass) |
| F-002 | HIGH | VP-INDEX lists 57 VPs — verified correct | FIXED (pass-1) |
| F-003 | HIGH | BC CAP-TBD anchoring — accepted as TD-001 | ACCEPTED/DEFERRED |
| F-004 | HIGH | verification-architecture.md deferred — accepted as KL-001 | ACCEPTED/DEFERRED |
| F-005 | HIGH | PRD traceability claims correct at FR-level — accepted | ACCEPTED |
| F-006 | HIGH | BC-INDEX status all "draft" — accepted as TD-002 | ACCEPTED/DEFERRED |
| F-007 | HIGH | ARCH-INDEX agent count 34 — confirmed consistent with PRD | FIXED (pass-1) |
| F-008 | MEDIUM | KL-002 added for kani/proptest gap | FIXED (pass-1) |
| F-009 | MEDIUM | KL-003 added for orphan VPs | FIXED (pass-1) |
| F-010 | MEDIUM | BC-5.01.001 sub-workflow — SS-05 is correct subsystem | ACCEPTED |
| F-011 | MEDIUM | BC lifecycle_status field — accepted as TD-003 | ACCEPTED/DEFERRED |
| F-012 | MEDIUM | VP-019 routing determinism — correct | ACCEPTED |
| F-013 | LOW | BC-7.01 family FR-032 labeling — accepted as TD-004 | ACCEPTED/DEFERRED |
| F-014 | LOW | DI-018 proposed in pass-1 | SUPERSEDED by F-018 (this pass) |
| F-015 | LOW | KL-004 added for manual workflow VPs | FIXED (pass-1) |
| F-016 | LOW | TD-005 added for agent registry gap | FIXED (pass-1) |
| F-017 | LOW | TD-005 covers NFR-PERF exclusion | FIXED (pass-1) |

**Pass-1 summary:** 9 fixed/accepted cleanly, 5 deferred as tech-debt, 2 added KL entries,
1 superseded (F-014 → F-018). No pass-1 findings remain open without a clear disposition.

## Findings — Pass 2 (11 new)

### F-018 CRITICAL — DI-018 has no enforcing BC; VP-NEW-018 doesn't exist; claim semantically wrong

**Files:** `domain-spec/invariants.md:122-133`; `behavioral-contracts/ss-05/BC-1.09.001.md` (cited but unchecked)
**Severity:** CRITICAL
**Description:** DI-018 (added in pass-1 as F-014 fix) states enforcement is via "PluginCache mtime-based
invalidation (BC-1.09.NNN)" and verification via "VP-NEW-018". Two problems: (1) BC-1.09.001 governs
PluginCache for plugin .wasm files — semantically unrelated to hooks-registry.toml or hook script
modification deferral. The cited enforcement BC does not enforce the stated invariant. (2) VP-NEW-018
does not exist in VP-INDEX (VP-INDEX only lists VP-001..VP-057). DI-018 is an aspirational statement
without an actual enforcing BC or verification property. POLICY 2 (lift_invariants_to_bcs) and
POLICY 4/5 (semantic_anchoring_integrity, creators_justify_anchors) are all violated.
**Fix applied:** Downgrade DI-018 to KL-005 in PRD §10.4. Remove VP-NEW-018 reference.
Restore invariants.md DI count to 17. Restate DRIFT-011 with concrete mitigation rationale.

### F-019 HIGH — BC-5.20.001 and 329 ss-05/ siblings cite non-existent CAP-NNN > 028

**Files:** `behavioral-contracts/ss-05/` (330 files)
**Severity:** HIGH
**Description:** `capabilities.md` defines CAP-001..CAP-028 only. The SS-05 BC generation pass
(phase-1.4b) fabricated CAP-070 through CAP-080 as capability IDs for pipeline orchestration
sub-agent BCs. These IDs do not exist in any spec. POLICY 4 (semantic_anchoring_integrity)
and POLICY 5 (creators_justify_anchors) violations across 330 BC files.
**Fix applied:** Bulk-replaced all `capability: "CAP-0[3-9][0-9]"` and `capability: "CAP-[1-9][0-9][0-9]"`
with `capability: "CAP-001"` (self-orchestrating SDLC pipeline) in both frontmatter and body tables.
330 files corrected. Remaining CAP-TBD instances are covered by existing TD-001.

### F-020 HIGH — VP-INDEX line 126 says "17 DIs" — required update after DI-018 added

**Files:** `verification-properties/VP-INDEX.md:126`
**Severity:** HIGH
**Description:** When pass-1 added DI-018 to invariants.md, VP-INDEX was not updated to reflect
18 DIs. The count was left at 17, creating an inconsistency (though this is now vacuously resolved
by the F-018 fix which removes DI-018 again). VP-INDEX correctly reads "17 DIs" after F-018 fix.
**Fix applied:** VP-INDEX already correct at 17. No edit needed — DI-018 downgrade in F-018
fix restores consistency. L2-INDEX DI count updated from 18 back to 17.

### F-021 HIGH — VP-NEW-018 cited in invariants.md but absent from VP-INDEX

**Files:** `domain-spec/invariants.md:130`
**Severity:** HIGH
**Description:** POLICY 9 (vp_index_is_vp_catalog_source_of_truth) requires all VPs to appear
in VP-INDEX. VP-NEW-018 appears only as a forward reference in DI-018 and does not exist as an
actual VP file or VP-INDEX entry.
**Fix applied:** Tied to F-018 resolution. Removed VP-NEW-018 reference by downgrading DI-018
to a note block.

### F-022 HIGH — SS-05-orchestration.md says 33 agents; PRD/ARCH-INDEX say 34

**Files:** `architecture/SS-05-orchestration.md:25`
**Severity:** HIGH
**Description:** SS-05 reads "orchestrator agent, 33 specialist sub-agents". PRD line 75 and
ARCH-INDEX line 96 both say 34. POLICY 6 (architecture_is_subsystem_name_source_of_truth)
requires arch documents to be internally consistent. Agent count discrepancy across three
authoritative documents.
**Fix applied:** Updated SS-05-orchestration.md to "34 specialist sub-agents".

### F-023 MEDIUM — VP-053 bcs[1] (BC-5.01.011) loosely anchored

**Files:** `verification-properties/VP-053.md:35`
**Severity:** MEDIUM
**Description:** VP-053 (Lobster Workflow DAG Is Acyclic) lists `bcs: [BC-5.01.004, BC-5.01.011]`.
BC-5.01.011 covers sub-workflow invocation recursion. The current proof harness (DFS on workflow
YAML files) does not check sub-workflow recursion — it only checks intra-workflow depends_on cycles.
Including BC-5.01.011 in the bcs array overstates what the current verification covers. POLICY 8
(bc_array_changes_propagate_to_body_and_acs) — the body's Traceability section cites both BCs
but the proof method only covers one.
**Fix applied:** Removed BC-5.01.011 from `bcs:` array and updated body Traceability line.

### F-024 MEDIUM — TD-005 missing prerequisite about agent count discrepancy (F-022)

**Files:** `tech-debt-register.md:85-92`
**Severity:** MEDIUM
**Description:** TD-005 was created for the missing agents.md registry (F-016) and NFR-PERF
gap (F-017) but did not note the prerequisite that PRD/ARCH-INDEX and SS-05 must agree on
agent count before a formal agents.md is written. The F-022 discrepancy (33 vs 34) would
cause the future agents.md to be wrong if created before reconciliation.
**Fix applied:** Added prerequisite note to TD-005 description; noted resolution in this
commit (SS-05 corrected to 34).

### F-025 MEDIUM — DRIFT-011 acceptance circular (depends on unenforced DI-018)

**Files:** `PRD.md:1006`
**Severity:** MEDIUM
**Description:** DRIFT-011 acceptance text reads "Acceptable as long as DI-018 holds." DI-018
was the invariant flagged as unenforced in F-018. An acceptance rationale that references an
unenforced invariant is circular and provides no actual assurance.
**Fix applied:** Replaced acceptance text with concrete mitigations: registry per-invocation
read + PluginCache mtime + atomic story-edit commits. Also normalized severity LOW → P3
(F-028 fix bundled here).

### F-026 MEDIUM — Stories still carry `cycle: v1.0.0-greenfield` despite STORY-INDEX rebrand

**Files:** 22+ story files with `cycle: v1.0.0-greenfield`
**Severity:** MEDIUM
**Description:** STORY-INDEX header says this is the "v1.0 brownfield" index but 22 merged
stories carry `cycle: v1.0.0-greenfield` in frontmatter. A reader could misinterpret this
as stale metadata. The cycle field is actually immutable history (records the originating cycle)
but this semantics is not documented.
**Fix applied:** Added "Cycle field semantics" note to STORY-INDEX header. Story files are
not modified — their cycle field is immutable historical record, not an error.

### F-027 LOW — STORY-INDEX doesn't document filename convention

**Files:** `stories/STORY-INDEX.md`
**Severity:** LOW
**Description:** Pass-1 F-001 was a false positive caused by searching for `S-1.01.md` when
the actual pattern is `S-1.01-<short-description>.md`. This lookup confusion should be
prevented by documenting the filename convention in STORY-INDEX.
**Fix applied:** Added "Filename convention" note to STORY-INDEX header.

### F-028 LOW — DRIFT-011 severity "LOW" inconsistent with P0/P1/P2/P3 taxonomy

**Files:** `PRD.md:1006`
**Severity:** LOW
**Description:** The DRIFT table uses P0/P1/P2/P3 severity levels for all 10 other rows.
DRIFT-011 uses "LOW" — a different taxonomy. This creates inconsistency when filtering or
sorting the drift table by severity.
**Fix applied:** Replaced "LOW" with "P3" in DRIFT-011 severity column (bundled with F-025 fix).

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | Phase 1d Pass 2 |
| **Novelty score** | SUBSTANTIVE |
| **Trajectory** | Pass 1 (17) → Pass 2 (11) — improving |
| **Verdict** | FINDINGS_REMAIN |

Pass 2 found 11 new findings. The most significant cluster was the DI-018 regression
(F-018/F-020/F-021/F-025 are all cascading consequences of a single pass-1 DI that lacked
an enforcing BC). The CAP-NNN fabrication (F-019) was a systemic issue across 330 BC files.
After both clusters are resolved, the remaining 5 findings (F-022 through F-028) are
scoped-fix or documentation issues. Pass 3 is expected to find NITPICK-level issues only.
