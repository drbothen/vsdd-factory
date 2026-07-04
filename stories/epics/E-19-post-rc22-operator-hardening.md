---
document_type: epic
epic_id: "E-19"
version: "v1.0"
status: draft
title: "Post-rc.22 Operator Hardening — pr-manager race fixes, verify-factory-lock size defect, warn-pending-wave-gate false-positive, registry/bundle hygiene, async telemetry + VSDD_SINK_FILE"
prd_capabilities: []
subsystems_affected: [SS-01, SS-03, SS-04, SS-05, SS-06, SS-07, SS-09]
target_release: "v1.0.0-rc.23"
story_count: 5
producer: story-writer
timestamp: 2026-07-04T00:00:00Z
phase: F3
cycle: v1.0-feature-engine-discipline-pass-1
depends_on: [E-18]
inputs:
  - .factory/logs/dispatcher-internal-2026-07-04.jsonl
  - .factory/stories/S-19.01-pr-manager-hardening.md
  - .factory/stories/S-19.02-verify-factory-lock-output-too-large.md
  - .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md
  - .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md
  - .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
input-hash: "714085e"
---

# Epic E-19: Post-rc.22 Operator Hardening

## Description

E-19 collects the five hardening stories authorized by the rc.22 post-install smoke gate
(2026-07-04; 73/73 PASS-WITH-FINDINGS). The findings expose four distinct defect classes
discovered only after the v1.0.0-rc.22 marketplace tarball was installed and exercised
against a live production-state vsdd-factory repository:

1. **pr-manager process gaps (S-19.01):** Three lessons codified from the rc.22
   brownfield-backfill cycle (L-BB-merge-race-ready-report-stale-head / D-749,
   L-BB-release-pr-squash-merge-not-mechanically-enforced / D-750,
   L-BB-simulation-shell-dialect-gap / D-750) expose silent failure modes: READY
   verdicts with no SHA pinning, no mechanical squash-merge guard on release PRs,
   and darwin-leg scripts validated under the wrong Bash version.

2. **verify-factory-lock byte-cap defect (S-19.02):** STATE.md in the rc.22 production
   install is ~90 KB, exceeding the 64 KiB cap in `verify-factory-lock`
   (`STATE_MD_MAX_BYTES = 65536`). Every PreToolUse Edit/Write/Agent dispatch triggers
   `capability_denied reason=output_too_large`, causing the single-writer enforcement
   gate (BC-4.13.001) to fail-open silently (ADR-025 Decision 7). Confirmed in
   `.factory/logs/dispatcher-internal-2026-07-04.jsonl` on traces a4b26f12, bcc3e6ef,
   cf4c2e4d, 2551d7db.

3. **warn-pending-wave-gate false-positive (S-19.03):** In a fresh install where
   `.factory/wave-state.yaml` does not yet exist, `read_file.rs::path_allowed()` calls
   `canonicalize()` which fails for absent files and returns `false` (path not allowed),
   even though the path IS in the allowlist. Every Stop event emits a false-positive
   `capability_denied reason=path_not_allowed` telemetry event. Confirmed on trace
   bc687a0f.

4. **Registry/bundle hygiene and observability gaps (S-19.04, S-19.05):** Four orphan
   WASM files ship unreferenced in the release bundle; unanchored tool-filter regex
   entries fire on unintended tool names (e.g., `Edit` matches `MultiEdit`); async
   plugins emit `plugin.invoked` but never `plugin.completed` or `plugin.abandoned`;
   `VSDD_SINK_FILE` is compile-gated out of release builds, making diagnostic replay
   impossible for operators.

E-19 is intentionally narrow: it fixes the known defects without scope expansion. All
five stories are production-grade closures per the Canonical Principle — no MVP deferrals,
no `TODO for later`, no paper-fixes.

## Trigger / Motivation

rc.22 post-install smoke gate (2026-07-04) run against the marketplace tarball at
`~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.22/`. The gate captured
dispatcher internal log evidence for all four defect classes. Stories are authored under
the story-writer dispatch dated 2026-07-04 with explicit human authorization.

## Epic Placement Justification

E-18 is the immediately preceding epic (Factory Context Durability; COMPLETE as of
S-18.12 PR #384 ec05606a 2026-07-01). E-19 is the next free ID under POLICY 1
(append-only numbering; STORY-INDEX confirmed no E-19 row at time of creation
2026-07-04). The post-rc.22 smoke findings are logically distinct from E-18
(context-durability) and warrant a new epic because they span four different defect
classes across six subsystems. Grouping them under E-18 would conflate the
context-durability epic with unrelated hardening work.

## PRD Capabilities Covered

No new PRD capabilities. E-19 stories fix defects in existing capabilities and add
observability infrastructure. BC-4.13.001 (verify-factory-lock behavioral contract)
is amended by S-19.02 to reflect the raised byte budget. BC-3.08.001 (async event
catalog) amendment required by S-19.05 is flagged for product-owner routing and is
NOT authored within this epic.

## Stories

| Story ID | Title | Wave | Points | BCs |
|----------|-------|------|--------|-----|
| S-19.01 | pr-manager hardening: READY verdict HEAD-SHA pinning + release-PR merge-strategy guard + shell-dialect simulation discipline | W1 | 8 | (behavioral_contracts: []) |
| S-19.02 | verify-factory-lock FINDING-1: frontmatter-only STATE.md read + raised byte budget | W1 | 8 | BC-4.13.001 |
| S-19.03 | warn-pending-wave-gate FINDING-2: read_file file_not_found semantics + graceful absent-file handling | W1 | 5 | (behavioral_contracts: []) |
| S-19.04 | Registry/bundle hygiene: orphan WASM removal + tool-filter regex anchoring convention + lint check | W2 | 5 | (behavioral_contracts: []) |
| S-19.05 | Observability gaps: async plugin completion telemetry + VSDD_SINK_FILE release-mode opt-in | W2 | 8 | (behavioral_contracts: []; BC-3.08.001 amendment flagged for PO routing) |

**Total:** 5 stories, 34 story points.

**Sequencing rationale:**

- Wave 1 (S-19.01, S-19.02, S-19.03): The three P0/P1 defect fixes. S-19.02 is P0 (the
  verify-factory-lock single-writer gate is silently bypassed in production). S-19.01 and
  S-19.03 are P1 (process gaps and false-positive telemetry; no data-loss risk). All three
  are independent; they can run in parallel within W1.

- Wave 2 (S-19.04, S-19.05): The P2 hygiene and observability stories. No hard dependency
  on W1; wave ordering is priority-driven. Can also run in parallel within W2.

## Dependency Graph

```
S-19.01 (W1, P1) ─┐
S-19.02 (W1, P0) ─┤ (all independent; W1 can run in parallel)
S-19.03 (W1, P1) ─┘

S-19.04 (W2, P2) ─┐ (no hard deps on W1; W2 can run in parallel)
S-19.05 (W2, P2) ─┘
```

Topological order: W1 → W2 (by priority, not hard coupling). No cycles. Acyclic confirmed.

## Out of Scope

- **BC-3.08.001 async event catalog amendment:** Required by S-19.05 to add
  `plugin.abandoned` as a new event type. Flagged for product-owner routing. This
  epic does NOT author the BC; the implementer proceeds with the structural work and
  routes the BC amendment separately.

- **ADR amendment for raised STATE_MD_MAX_BYTES:** ADR-025 Decision 5 documents the
  original 64 KiB cap. The S-19.02 fix raises the cap to 256 KiB in source; an ADR-025
  amendment may be authored by the implementer inline during S-19.02 or deferred to a
  follow-up state-manager burst. This decision is left to the implementer.

- **WASM fuel-budget increase for lessons.md:** D-442(e) documents the WASM fuel exhaustion
  issue on large lessons.md files. This is NOT part of E-19; it is tracked under S-15.03
  PRIORITY-A and is a separate concern.

- **S-15.03 hook path-pattern narrowing:** The convergence-tracker.sh false-positive on
  `.factory/cycles/` path patterns (Factory Hook Diagnostics table, row 5) is not part
  of E-19; it is tracked under S-15.03 PRIORITY-A.

## Behavioral Contract Traceability

| BC ID | Title | Story |
|-------|-------|-------|
| BC-4.13.001 | verify-factory-lock WASM PreToolUse guard — single-writer enforcement on .factory/ | S-19.02 (amended: raised byte budget + frontmatter-only extraction) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-07-04 | story-writer | Initial creation. Post-rc.22 smoke gate authorization. 5 stories S-19.01..S-19.05 spanning SS-01/SS-03/SS-04/SS-05/SS-06/SS-07/SS-09. 2 waves; 34 pts. No new PRD capabilities. BC-4.13.001 amended by S-19.02. BC-3.08.001 amendment flagged for PO routing (S-19.05). |
