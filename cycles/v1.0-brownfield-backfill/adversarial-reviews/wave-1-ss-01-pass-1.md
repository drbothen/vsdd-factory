---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T20:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.01-cargo-workspace-setup.md
  - .factory/stories/S-1.02-dispatcher-core.md
  - .factory/stories/S-1.04-host-function-surface.md
  - .factory/stories/S-1.05-wasmtime-integration.md
  - .factory/stories/S-1.06-tokio-parallel-tier-execution.md
  - .factory/stories/S-1.07-dispatcher-internal-log.md
  - .factory/stories/S-3.04-emit-event-host-function.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: 3471ea6
traces_to: ""
cycle: v1.0-brownfield-backfill
sub_cycle: wave-1-ss-01-re-anchor
pass: 1
verdict: FINDINGS_REMAIN
finding_count: 10
convergence_step: 0_of_3
po_commit_reviewed: d373e2b
previous_review: null
---

# Adversarial Review — Wave 1 SS-01 Re-anchor — Pass 1

**Scope:** wave-1-ss-01-re-anchor — 7 stories anchored to canonical SS-01 BCs by PO at commit d373e2b on factory-artifacts.

**Stories reviewed:** S-1.01 (5 BCs), S-1.02 (26), S-1.04 (26), S-1.05 (15), S-1.06 (8), S-1.07 (10), S-3.04 (8). Total 98 anchors (97 unique after F-004 dedup).

**Verdict:** FINDINGS_REMAIN — 10 findings (3 HIGH, 4 MEDIUM, 3 LOW).

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix — `W1SS01` for wave-1-ss-01-re-anchor
- `<PASS>`: Two-digit pass number (P01)
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples for this pass: `ADV-W1SS01-P01-HIGH-001`, `ADV-W1SS01-P01-MED-002`

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-001 [ADV-W1SS01-P01-HIGH-001] — S-1.01 anchors 4 BC-1.07.* contracts that describe runtime/test behavior, not workspace setup

**Affected:** S-1.01-cargo-workspace-setup.md
**BCs:** BC-1.07.003, BC-1.07.004, BC-1.07.005, BC-1.07.006

S-1.01 target_module is the workspace root (Cargo.toml + rust-toolchain.toml + [workspace.dependencies]). The 4 BC-1.07.* anchors describe runtime/integration-test behavior:
- BC-1.07.003: tests/loads_legacy_registry.rs round-trip parsing — runtime test contract
- BC-1.07.004: scripts/generate-registry-from-hooks-json.sh idempotency — bash script contract
- BC-1.07.005: tests/loads_legacy_registry.rs plugin resolution — integration test
- BC-1.07.006: same integration test — script_path validation

PRD §FR-007 (line 244-245) attributes these BCs to S-2.01/S-2.02 explicitly: "Status: shipped (S-2.01, S-2.02)". Anchoring them to S-1.01 is a stretch (transitive: workspace must compile for these tests to run).

**Remediation:** Remove BC-1.07.003-006 from S-1.01 frontmatter + body Behavioral Contracts table. Re-anchor to S-2.01/S-2.02 in Wave 3 (SS-04 plugin ecosystem). S-1.01 may end with zero BCs (genuine pure plumbing) or anchor to workspace-scaffolding-specific BCs only.

#### F-002 [ADV-W1SS01-P01-HIGH-002] — S-1.01 ACs cite BC clauses that don't exist in the cited BCs

**Affected:** S-1.01-cargo-workspace-setup.md (lines 66, 69, 74, 79)

S-1.01's ACs cite BC clauses that aren't present in the actual BCs:
- AC#1 cites "BC-1.07.005 invariant: plugin crate uses crate-type cdylib + wasm32-wasip1" — BC-1.07.005's actual invariant 1 is about legacy adapter routing.
- AC#2 cites "BC-1.08.003 precondition: tokio with correct features" — BC-1.08.003's actual precondition is "Dispatcher process startup."
- AC#3 cites "BC-1.07.003 precondition: toml crate available" — BC-1.07.003's actual precondition is about generated registry.
- AC#5 cites "BC-1.07.004 postcondition: registry-generation idempotent — CI enforces clean build" — the "CI enforces" framing is added; not in the BC.

Violates BC-5.36.001-002: ACs must trace to actual BC clauses, not synthesized ones.

**Remediation:** Either (a) re-anchor per F-001, or (b) keep workspace-prerequisite framing and rewrite each AC trace to cite actual BC clauses literally, accepting transitive anchoring.

#### F-003 [ADV-W1SS01-P01-HIGH-003] — S-1.04 AC#8 (host function panic recovery) traces to BC-1.05.011 (explicit log calls)

**Affected:** S-1.04-host-function-surface.md line 103

AC#8: "Panic in a host function converts to internal.host_function_panic event (traces to BC-1.05.011 postcondition...)". BC-1.05.011 is about "When a plugin calls vsdd::log(level, msg)" — explicit log calls, NOT panic-handling. Different code paths, different events (plugin.log vs internal.host_function_panic).

No SS-01 BC covers host-function-panic recovery — likely v1.1 BC candidate.

**Remediation:** (a) Add to v1.1 BC candidate list and remove the AC, (b) keep AC as forward-looking without trace, or (c) delete the AC if out of scope.

### MEDIUM

#### F-004 [ADV-W1SS01-P01-MED-001] — BC-1.08.003 double-anchored across S-1.01 and S-1.02

**Affected:** S-1.01 (line 28) + S-1.02 (line 46)

BC-1.08.003's architecture module = main.rs:40 (S-1.02 territory). S-1.02 anchor is strong (direct implementation). S-1.01 anchor is transitive (tokio pinning in [workspace.dependencies]).

Also: explains the "98 of 99" claim — unique anchored = 97.

**Remediation:** Remove BC-1.08.003 from S-1.01 frontmatter + body table. Update tally to "97 unique + 2 deferred = 99".

#### F-005 [ADV-W1SS01-P01-MED-002] — S-1.01 traces_to FR-007 contradicts PRD ownership

**Affected:** S-1.01 frontmatter lines 15, 32

PRD §FR-007: "Status: shipped (S-2.01, S-2.02)". S-1.01 traces_to FR-007 — workspace-setup story doesn't implement legacy hook routing.

**Remediation:** Re-target traces_to to a workspace/CI-scaffolding FR if one exists (or update PRD §FR-007 to acknowledge prerequisite stories — but this dilutes traceability across all dependent stories).

#### F-006 [ADV-W1SS01-P01-MED-003] — S-1.05 AC#10 (4 lifecycle event types) traces only to stderr-omission BC

**Affected:** S-1.05-wasmtime-integration.md line 91-92

AC#10: "Dispatcher emits plugin.invoked, plugin.completed, plugin.timeout, plugin.crashed events (traces to BC-1.03.006 postcondition: empty stderr omitted from lifecycle events)". BC-1.03.006 only constrains the stderr field; doesn't mandate the 4 events themselves. No BC in SS-01 catalog mandates emission of these specific event types.

**Remediation:** (a) v1.1 BC candidate "plugin lifecycle event emission", (b) backfill BC, or (c) rewrite AC#10 to cite actual existing contracts (e.g., BC-1.03.001 for plugin.timeout-equivalent semantics).

#### F-007 [ADV-W1SS01-P01-MED-004] — CAP-010 capabilities.md subsystem tagging conflicts with S-1.07's actual subsystem

**Affected:** S-1.07-dispatcher-internal-log.md (lines 39-40) + capabilities.md (line 59)

capabilities.md CAP-010: "Subsystems: SS-03, SS-10". But S-1.07 anchors BC-1.06.001-010 (all SS-01-shard BCs) and target_module = crates/factory-dispatcher/src/internal_log.rs (SS-01).

Domain-spec drift. S-1.07 compromises with subsystems: ["SS-01", "SS-03"] which papers over the inconsistency.

**Remediation:** Update capabilities.md CAP-010 subsystems to include SS-01 (dominant implementer). Reconsider S-1.07's `subsystems` to ["SS-01"] only after capabilities.md is fixed. Tag [process-gap] — sweep all 28 CAPs for similar drift.

### LOW

#### F-008 [ADV-W1SS01-P01-LOW-001] — Orchestrator-reported "98 BCs anchored" double-counts BC-1.08.003

True unique anchored = 97 (after F-004 fix). 97 + 2 deferred = 99 ✓.

#### F-009 [ADV-W1SS01-P01-LOW-002] — S-1.01 secondary capability mention (CAP-009) in body but not in `capabilities:` frontmatter

S-1.01 line 149 mentions CAP-009 in prose. Frontmatter only lists CAP-002. Intent unclear — pending convention adjudication.

#### F-010 [ADV-W1SS01-P01-LOW-003] — S-3.04 status: partial — body status note implies AC#3 (bin/emit-event deprecation) is outstanding

8 anchored BCs are decode/encode/reserved-fields contracts (shipped). AC#3 (deprecation work) is outstanding. No tdd_mode marker distinguishes shipped-vs-pending ACs.

**Remediation:** Add "Partial Status" subsection listing per-AC done/pending state.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 4 |
| LOW | 3 |

**Overall Assessment:** pass-with-findings
**Convergence:** FINDINGS_REMAIN — iterate
**Readiness:** requires revision (S-1.01 fabricated BC clause citations must be corrected before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings count** | 10 |
| **Duplicate count** | 0 (first pass — no prior baseline) |
| **Novelty score** | 1.0 (all findings novel by definition for pass-1) |
| **Median severity** | MEDIUM |
| **Severity distribution** | 0 CRITICAL, 3 HIGH, 4 MEDIUM, 3 LOW |
| **Trajectory** | starting (no prior baseline) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

This is pass-1. ADR-013 requires 3 consecutive NITPICK-only passes for CONVERGENCE_REACHED. With 3 HIGH findings, far from convergence. Estimated convergence at pass-3 to pass-5.

## Findings by Axis

| Axis | Findings |
|------|----------|
| A — BC Existence | (none) ✓ |
| B — Semantic Anchoring Integrity | F-001, F-003 |
| C — Coverage Completeness | F-006 (partial) |
| D — AC↔BC Bidirectional Symmetry | F-002, F-003, F-006 |
| E — Capability Anchor Justification | (none) ✓ |
| F — Subsystem & FR Frontmatter Hygiene | F-005, F-007 |
| G — VP Anchoring Soundness | (none) ✓ |
| H — CAP Choice Soundness | F-007 |
| I — Spec-First Gate | F-002 |
| J — POLICY 1 Reuse Check | (none) ✓ |
| K — Edge Cases | (none — no cross-SS leakage) ✓ |
| L — Bookkeeping / Trace Integrity | F-004, F-008 |
