---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-06T00:00:00Z
phase: 2
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 1
previous_review: null
perimeter: E-19 epic + S-19.01..S-19.05 + STORY-INDEX E-19 section
verdict: NOT-CLEAN
blocker_count: 1
high_count: 9
medium_count: 5
low_count: 1
observation_count: 5
streak: 0/3
parent_decision: D-751
---

# Adversarial Review — E-19 Pass 1 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.05 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law)
**Date:** 2026-07-06
**Verdict:** NOT-CLEAN — BLOCKER 1 / HIGH 9 / MEDIUM 5 / LOW 1 + 5 observations
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P1-001`, `F-P1-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-18 and E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

N/A — pass 1. No prior findings to verify.

---

## Part B — New Findings (or all findings for pass 1)

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften.*

F-P1-001 — BLOCKER — codes::NOT_FOUND = -4 collides with existing INVALID_ARGUMENT = -4. S-19.03 AC-003 asserts codes::NOT_FOUND = -4 as available. It isn't: -4 is already INVALID_ARGUMENT; -1/-2/-3/-99 also occupied. A second constant at -4 compiles but silently corrupts the host ABI. The AC-003 grep gate would green-light the broken definition. Evidence: crates/factory-dispatcher/src/host/mod.rs codes module. Routing: architect + story-writer.

F-P1-002 — HIGH — S-19.01 target_module and File Structure paths point at nonexistent plugins/vsdd-factory/skills/pr-manager/; pr-manager is an agent at plugins/vsdd-factory/agents/pr-manager.md. Story's own inputs: field contradicts target_module. Routing: story-writer.

F-P1-003 — HIGH — POLICY 8 violation: S-19.02 has BC-4.13.001 in frontmatter but no body BC table, no BC↔AC traceability, no Token Budget. Routing: story-writer.

F-P1-004 — HIGH — BC-4.13.001 amendment silently pushed to implementer; not flagged for product-owner routing (asymmetric with S-19.05's treatment of BC-3.08.001). BC Precondition 3 hardcodes max_bytes=65536 as MUST; S-19.02 changes it without PO flag. Routing: product-owner + story-writer.

F-P1-005 — HIGH — Routing violation: E-19 Out-of-Scope hands ADR-025 amendment to implementer/state-manager ("decision left to the implementer") — Agent Routing Table violation + Canonical Principle Rules 3/6. Routing: architect.

F-P1-006 — HIGH — TD-VSDD-091 violations: S-19.05 narrative prose pins main.rs line numbers (549-581, 36, 70, 78, 219, 393, 478-484). Routing: story-writer.

F-P1-007 — HIGH — Technical-claim fidelity: S-19.05 misattributes the cfg(debug_assertions) at the drain-window override to VSDD_SINK_FILE; lines 478-484 gate VSDD_ASYNC_DRAIN_WINDOW_MS (SEC-003). Literal implementation would strip release-build safety off an unrelated env var. Routing: story-writer.

F-P1-008 — HIGH — S-19.02 "both arms are required" claim materially wrong: read_bounded() checks metadata.len() BEFORE reading, so frontmatter-only extraction cannot compensate for an undersized cap; AC-001 alone closes FINDING-1. Routing: story-writer.

F-P1-009 — HIGH — S-19.04 AC-004 lint gate broken as written: grep -v '^\^' is a no-op against grep -n output (lines start with line numbers); correctly-anchored entries misclassified; story can never converge. Routing: story-writer.

F-P1-010 — HIGH — S-19.01 EC-003 silently drops darwin-leg coverage on Linux CI, defeating closure of L-BB-simulation-shell-dialect-gap; "skips gracefully" ships the defect class undetected. Routing: architect + story-writer.

F-P1-011 — MEDIUM — S-19.03 AC-001 ancestor-fallback under-specified: deepest canonical ancestor .factory fails starts_with against file-allowlist entry .factory/wave-state.yaml — AC-as-written reproduces the defect it fixes; correct algorithm is the rejoin pattern in write_file.rs resolve_path_for_allowlist. Routing: story-writer.

F-P1-012 — MEDIUM — POLICY 7 violation: E-19 epic BC Traceability paraphrases BC-4.13.001 H1 title. Routing: story-writer.

F-P1-013 — MEDIUM — S-19.05 AC-002 dangling race: abandoned async plugin that later completes — both-events vs abort vs suppression unspecified; implementer forced to guess. Routing: product-owner + story-writer.

F-P1-014 — MEDIUM — S-19.01 AC-001 gate lacks positive assertion on READY_SHA_FETCH_FAILED error string. Routing: story-writer.

F-P1-015 — LOW — S-19.05 AC-006 grep gate loosely scoped (VSDD_SINK_FILE.*release alternation can match unrelated adjacencies in CLAUDE.md). Routing: story-writer.

Observations: O-P1-001 JoinSet-vs-channel advisory for S-19.05 completion tracking (flag to architect). O-P1-002 S-19.05 sizing (8pts, two subsystems + BC amendment; split is a sizing question not correctness). O-P1-003 epic depends_on [E-18] vs story depends_on [] — consistent, confirm at F1. O-P1-004 S-19.04 v1.1 amendment history well-captured (positive signal). O-P1-005 S-19.05 EC-004 "no performance penalty" worth measuring in formal hardening.

Verdict: NOT-CLEAN. BLOCKER 1 / HIGH 9 / MEDIUM 5 / LOW 1.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 1 |
| HIGH | 9 |
| MEDIUM | 5 |
| LOW | 1 |
| Observations | 5 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision; all 15 findings + O-P1-001 FIXED same-burst (4 specialist legs)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 15 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (15 / 15) |
| **Median severity** | HIGH |
| **Trajectory** | 15 (pass 1 baseline) |
| **Verdict** | FINDINGS_REMAIN — pass 2 dispatched with fresh context |

---

## Fix-Burst Closure Section (D-752)

**All 15 findings + O-P1-001 advisory FIXED same-burst across 4 specialist legs.**

**Streak: 0/3** — pass-1 verdict is NOT-CLEAN regardless of same-burst fixes per D-628; pass-2 NEXT with fresh context.

### Product-Owner leg

- **BC-4.13.001 v1.3→v1.4:** raised read cap from 65536 to 262144; added Invariant 9 (frontmatter-only extraction). Closes F-P1-004 (PO flag honored).
- **BC-3.08.001 v1.14→v1.15:** added Event 5 `plugin.abandoned`; added Invariant 6 (abandoned-is-terminal). Closes F-P1-013 AC-002 dangling-race (PO-level race policy specified).
- **BC-INDEX v3.57→v3.59** (v3.58 for BC-3.08.001 v1.15; v3.59 for BC-4.13.001 v1.4).

### Architect leg

- **ADR-025 v1.6→v1.7:** Decision 13 (codes::NOT_FOUND=-5, HOST_ABI_VERSION=1 unchanged); Decision 14 (read-cap 262144 + frontmatter-only extraction rationale); 10 TD-031 volatile line-number cites swept. Closes F-P1-001 (BLOCKER — code -4 collision resolved by assigning -5 to NOT_FOUND) and F-P1-005 (routing violation — ADR-025 amendment routed to architect as required).
- **ARCH-INDEX v2.85→v2.86.**
- **Linux-CI strategy Option B adopted:** dedicated `bats-darwin-leg-macos` job on `macos-latest`; Apple patched 3.2.57 not faithful to vanilla GNU 3.2, so Linux emulation non-faithful. Closes F-P1-010 (architect side).
- **O-P1-001 advisory addressed:** additive channel augmentation for async completion signaling; JoinSet remains optional follow-on.

### Story-writer leg

- **S-19.01 v1.0→v1.1:** target_module corrected to agents path; EC-003 darwin-leg coverage added (macos-latest CI job); AC-001 positive READY_SHA_FETCH_FAILED assertion added. Closes F-P1-002, F-P1-010 (story side), F-P1-014.
- **S-19.02 v1.0→v1.1:** body BC table + BC↔AC traceability + Token Budget added per POLICY 8; "both arms required" claim corrected (AC-001 alone sufficient); BC-4.13.001 PO-flag documented. Closes F-P1-003, F-P1-008.
- **S-19.03 v1.1→v1.2:** AC-001 ancestor-fallback algorithm specified using rejoin pattern from write_file.rs resolve_path_for_allowlist; AC-003 code value corrected to -5. Closes F-P1-001 (story side) and F-P1-011.
- **S-19.04 v1.1→v1.2:** AC-004 lint gate fixed (grep -v 'file:' pattern replaces broken grep -v '^\^'). Closes F-P1-009.
- **S-19.05 v1.0→v1.1:** TD-VSDD-091 volatile line-number cites removed; cfg(debug_assertions) attribution corrected to VSDD_ASYNC_DRAIN_WINDOW_MS (SEC-003); AC-002 dangling-race policy added per BC-3.08.001 v1.15 Event 5; AC-006 grep gate tightened. Closes F-P1-006, F-P1-007, F-P1-013 (story side), F-P1-015.
- **E-19 epic v1.0→v1.1:** BC Traceability verbatim H1 title restored per POLICY 7; Out-of-Scope ADR-025 routing corrected to architect. Closes F-P1-005 (epic side) and F-P1-012.
- **STORY-INDEX v4.129→v4.131** (v4.130 architect NOT_FOUND=-5 fix row; v4.131 full fix package; stale v3.56 cite in v4.130 row corrected to v3.59).

**Routing deviation accepted (per D-628 precedent):** architect directly edited S-19.03 story content and bumped STORY-INDEX to v4.130 — cross-lane per Companion Principle but content-correct per ADR-025 Decision 13; accepted rather than churned; story-writer verified sweep completeness at v1.2.

**API-instability operational note:** 3 agent deaths mid-burst (2 PO stalls, 2 architect stalls incl. 1 ConnectionRefused) required idempotent disk-state-verified resume dispatches; the ~120KB ADR-025 file stalled 3 consecutive whole-file attempts; the 4th attempt succeeded using grep-recon + targeted-offset reads + small anchored Edits (surgical protocol per L-BB-oversized-artifact-surgical-edit-protocol codified D-752).

### Artifact versions at pass-1 closure

| Artifact | Version |
|----------|---------|
| BC-4.13.001 | v1.4 |
| BC-3.08.001 | v1.15 |
| BC-INDEX | v3.59 |
| ADR-025 | v1.7 |
| ARCH-INDEX | v2.86 |
| S-19.01 | v1.1 |
| S-19.02 | v1.1 |
| S-19.03 | v1.2 |
| S-19.04 | v1.2 |
| S-19.05 | v1.1 |
| E-19 epic | v1.1 |
| STORY-INDEX | v4.131 |
| VP-INDEX | v2.51 (unchanged) |

**NEXT:** E-19 adversarial pass-2 (fresh context; streak 0/3).
