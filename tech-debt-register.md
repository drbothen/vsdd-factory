---
document_type: tech-debt-register
producer: state-manager
version: "1.0"
last_updated: 2026-04-25T00:00:00
---

# Technical Debt Register

## Summary

| Priority | Count | Estimated Points |
|----------|-------|-----------------|
| P0 (next cycle) | 0 | 0 |
| P1 (within 3 cycles) | 0 | 0 |
| P2 (backlog) | 5 | — |

## Debt Items

| ID | Source | Description | Priority | Introduced | Cycle | Story | Due |
|----|--------|-------------|----------|-----------|-------|-------|-----|
| TD-001 | Phase 5 deferred | BC-level CAP/DI/Stories anchoring incomplete: all 1,851 BC files carry CAP-TBD/DI-TBD/Stories-TBD defaults from Phase 1.4b migration | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-002 | Phase 5 deferred | BC-INDEX status column all "draft" regardless of shipped/partial/pending reality | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-003 | Spec drift | BC frontmatter lacks per-BC lifecycle_status field; PRD claims FR-level status but BCs have no per-BC marker | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |
| TD-004 | Phase 5 deferred | BC-7.01 family is mixed (multiple hooks); FR-032 BC-group labeling conflicts with BC-7.01.001 H1 (block-ai-attribution vs protect-secrets) | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-005 | Phase 5 deferred | Agent registry missing (34 agents not enumerated); NFR-PERF not in PRD §4.2 top-5 | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |

### Source Types

| Source | Detection Agent | Description |
|--------|----------------|-------------|
| Phase 5 deferred | adversary | Finding deferred as "fix later" from adversarial review |
| Phase 6 deferred | formal-verifier | Finding deferred from formal hardening |
| Spec drift | spec-steward | BC postcondition not enforced in code |
| Dependency | security-reviewer | Major version bump available or vulnerability |
| DTU fidelity | dtu-validator | Real API changed, clone is stale |
| Pattern inconsistency | code-reviewer | Legacy pattern in older code |
| Holdout decay | holdout-evaluator | Scenario tests removed/changed feature |
| Maintenance sweep | consistency-validator | Anti-pattern or code smell detected |

### Item Details

#### TD-001 — BC-level CAP/DI/Stories anchoring incomplete
**Source:** Phase 1d pass 1 (F-003, F-005, F-011)
**Description:** All 1,851 BC files have `capability: CAP-TBD`, `L2 Domain Invariants: TBD`,
`Stories: TBD` — best-effort default from Phase 1.4b migration. PRD §8 anchors at FR-level;
BC-level reverse anchoring is incomplete.
**Severity:** P2 (does not block v1.0 GA — PRD has full traceability; per-BC anchoring
is a navigability improvement).
**Plan:** Wave-scale follow-up. After 3-clean-pass adversarial convergence on the spec
package, run a backfill burst: for each BC, read PRD §7 traceability matrix → assign
CAP, lookup DI from L2, populate Stories from STORY-INDEX.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-002 — BC-INDEX status column all "draft"
**Source:** Phase 1d pass 1 (F-006)
**Description:** All 1,851 BC-INDEX rows show status=draft regardless of whether the
underlying behavior is shipped, partial, or pending. Status should reflect the
implementation reality (22 stories merged + 4 partial + 15 draft).
**Severity:** P2 (PRD FR-level status is correct).
**Plan:** Same backfill burst as TD-001 — derive BC status from STORY-INDEX status of
the implementing story.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-003 — BC frontmatter lacks per-BC `lifecycle_status` field
**Source:** Phase 1d pass 1 (F-011)
**Description:** PRD claims FR-level partial/shipped/pending status but BC files have
no per-BC lifecycle marker (only the `lifecycle_status: active` from the template).
**Severity:** P3 (covered by BC-INDEX status column once TD-002 resolves).
**Plan:** Either schema decision (add field) or rely on BC-INDEX as the status authority.
**Cycle estimate:** v1.1.

#### TD-004 — BC-7.01 family is mixed; FR-032 BC-group labeling is ambiguous
**Source:** Phase 1d pass 1 (F-013)
**Description:** BC-7.01 family contains BCs for multiple hooks (block-ai-attribution in
BC-7.01.001, protect-secrets in BC-7.01.004, capture-commit-activity in BC-7.01.002,
regression-gate in BC-7.01.003). PRD FR-032 labels the BC-7.01 range as "protect-secrets.sh"
which conflicts with BC-7.01.001 H1 ("block-ai-attribution"). The actual alpha-sort order
of SS-07 hooks assigns block-ai-attribution as alphabetically first.
**Severity:** P2 (spec navigability; does not affect implementation).
**Plan:** Rationalize SS-07 BC family assignments: re-shard so each BC family maps to
exactly one hook script. Update PRD FR-032..FR-034 BC-group listings to match.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-005 — Agent registry and NFR-PERF top-5 not enumerated in spec
**Source:** Phase 1d pass 1 (F-016, F-017)
**Description:**
- F-016: 34 agents dispatched by the factory are not enumerated in any formal registry. An `agents.md` under `domain-spec/` would list each agent with its role, model tier, and tool access.
- F-017: PRD §4.2 Top-5 Priority NFRs does not include any NFR-PERF (performance) entry. At least one NFR-PERF (e.g., sub-100ms hook latency per DI-011/BC-7.02.005) should appear in the top-7 or be explicitly excluded with rationale.
**Severity:** P3 (docs/navigability).
**Pre-requisite note:** Reconcile PRD line 75 + ARCH-INDEX line 96 (34 agents) vs former SS-05 line 25 (33 agents). [Resolved in phase-1d pass-2 fix burst — SS-05 now says 34 specialist sub-agents.]
**Plan:** v1.1 — add `domain-spec/agents.md` registry; expand PRD §4.2 to top-7 or add NFR-PERF exclusion note.
**Cycle estimate:** v1.1.

## Resolution History

| ID | Resolved In | Story | Resolution |
|----|------------|-------|------------|
| — | — | — | No items resolved yet |

## Tech Debt as Feature Mode Cycles

When P0 items accumulate, they become a Feature Mode cycle (Path 3) with
cycle type "refactor":

```
orchestrator: "Tech debt P0 items need attention"
  -> Path 3 (Feature Mode) with cycle type "refactor"
  -> cycles/vX.Y.Z-refactor-[name]/
  -> Same VSDD rigor: specs updated, tests updated, adversarial review
  -> Release: PATCH (no new features) or MINOR (if public behavior changes)
```
