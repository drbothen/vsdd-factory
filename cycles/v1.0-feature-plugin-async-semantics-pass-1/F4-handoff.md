---
document_type: phase-handoff
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F4
producer: state-manager
version: "1.0"
status: ready
purpose: "Post-context-compaction F4 dispatch reference"
timestamp: 2026-05-07T00:00:00Z
---

# F4 Handoff — Plugin Async Semantics (S-15.01)

Post-context-compaction reference for dispatching F4 (Option A — auto-dispatch
the full TDD chain). After `/compact`, the assistant will not retain memory of
the 10 F2 passes and 5 F3 passes. This document captures everything F4 dispatch
requires.

---

## 1. Where We Are (Status Snapshot)

| Field | Value |
|-------|-------|
| **Cycle** | v1.0-feature-plugin-async-semantics-pass-1 |
| **Current phase** | F3 CONVERGED → F4 NEXT |
| **Active story** | S-15.01 v1.6 (status: ready) |
| **factory-artifacts HEAD** | 1227036 (feat(F3): CONVERGENCE_REACHED — S-15.01 ready for F4 TDD implementation) |
| **develop HEAD** | 15432c6 (S-12.06 PR #105 squash-merge 2026-05-07) |

### Phase completion summary

| Phase | Status | Key commit / detail |
|-------|--------|---------------------|
| F1 Delta Analysis | COMPLETE | `cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` |
| F2 Spec Crystallization | CONVERGED at pass-10 | commit 3568657; 10 passes + 7 fix bursts; trajectory 19→19→7→6→3→5→4→1→2→1 |
| F3 Story Decomposition | CONVERGED at pass-5 | commit 1227036; 5 passes + 4 fix bursts; trajectory 9→3→3→1→0 |
| F4 TDD Implementation | **PENDING DISPATCH** | This handoff authorizes dispatch |
| F5 / F6 / F7 | PENDING | After F4 merges |

### Concurrent cycles

- **v1.0-feature-engine-discipline-pass-1:** PAUSED at F3-amendment complete (D-366). F4-platform delivery (S-12.03 + S-12.05) in progress. This cycle is independent of S-15.01.
- **v1.0-brownfield-backfill:** PAUSED at E-10 pass-9. Resumes after S-15.01 ships.

---

## 2. The Story to Implement

**Path:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-15.01-plugin-async-semantics.md`

| Field | Value |
|-------|-------|
| **Story ID** | S-15.01 |
| **Version** | v1.6 |
| **Status** | ready |
| **Points** | XL (13) |
| **TDD mode** | strict |
| **Acceptance criteria** | 17 |
| **File list** | 30 paths |
| **Epic** | E-15 plugin async semantics |

### Primary BCs (must all be satisfied)

| BC | Version | Contract |
|----|---------|----------|
| BC-1.14.001 | v1.6 | Dispatcher partition contract + ASYNC_DRAIN_WINDOW_MS drain |
| BC-7.06.001 | v1.3 | Registry schema v2 + per-plugin async field + CI lint invariant |
| BC-9.01.006 | v1.2 | hooks.json template envelope sync invariant |
| BC-3.08.001 | v1.4 | Event catalog (4 new event types) |
| BC-1.08.001 | v1.2 | Fail-closed exception clause |

### Secondary BCs

BC-1.01.001 v1.1, BC-1.01.007 v1.1, BC-1.08.002 v1.1, BC-4.04.004 v2.1,
BC-4.05.004 v2.1, BC-4.07.003 v1.3, BC-4.08.002 v1.3

### Verification Properties

| VP | Version | Focus |
|----|---------|-------|
| VP-077 | v1.5 | Kani formal harnesses (partition correctness) |
| VP-078 | v1.8 | CI lint (4 harnesses) |
| VP-079 | v1.6 | Event payload fault-injection (5 scenarios) |
| VP-001 | v1.1 amended | — |
| VP-002 | v1.1 amended | — |

### Domain Invariants

- **DI-014:** (see invariants.md)
- **DI-019:** `ASYNC_DRAIN_WINDOW_MS = 100ms` — canonical home; cited by reference everywhere else

### Subsystems

SS-01 (dispatcher), SS-03 (event catalog), SS-04 (plugins), SS-07 (registry
config), SS-09 (envelope template)

### ADR

**ADR-019 v1.8** — Plugin async semantics at registry layer
Path: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md`

---

## 3. User-Locked Decisions (5 — Non-Negotiable)

F4 implementation MUST honor each of these verbatim. They were resolved by the
user directly during this cycle and cannot be revisited without explicit user
direction.

### Decision 1 — Envelope sync invariant (ADR-019 §Decision 1)

> Every Claude Code hook event must be sync at the envelope. No per-event
> carve-outs in hooks.json.

All 5 platform variants of `hooks.json.template` must have `on_error: "block"`
(sync) at the envelope level. No event-by-event exceptions.

### Decision 2 — No backwards compatibility (ADR-019 §Decision 5)

> No backwards compatibility. v2 dispatcher hard-errors on v1 registry.

`registry.rs` `validate()` must return a hard error (not a warning) when
`schema_version` is missing or `< 2`. No migration shim, no fallback path.

### Decision 3 — No phased rollout (ADR-019 §Decision 6)

> Single consolidated story (S-15.01) ships all changes together.

All 9 tasks (T-3a through T-3i) ship in one PR. No partial merges, no
feature-flag gates splitting the story.

### Decision 4 — `ASYNC_DRAIN_WINDOW_MS = 100ms` (DI-019)

The constant is defined in DI-019 as the canonical source of truth. Every file
that references this value must cite DI-019, not hardcode `100` independently.

### Decision 5 — WASM-migration rule (user directive; applies retroactively)

> "We are migrating to WASM. Any new plugins need to use WASM."

The new lint plugin (`lint-registry-async-invariant`) MUST be authored as a
native WASM Rust crate — NOT bash via legacy-bash-adapter. This applies to all
new work in this session and forward to F4. The WASM-rule audit was applied to
all session-modified artifacts at commit 4a73006.

---

## 4. F4 Dispatch Plan (Option A — Auto-Dispatch the Full Chain)

Issue: "go F4 Option A" to trigger this chain in sequence.

### Step 1: stub-architect

Generate compilable stubs per BC-5.38.001 Red Gate. All stubs must compile but
fail tests.

Files to stub:

| File | Type | Notes |
|------|------|-------|
| `crates/hook-plugins/lint-registry-async-invariant/Cargo.toml` | new | Rust crate scaffold (WASM target) |
| `crates/hook-plugins/lint-registry-async-invariant/src/lib.rs` | new | `todo!()` body for lint logic |
| `crates/hook-plugins/lint-registry-async-invariant/tests/integration_test.rs` | new | Failing skeletons |
| `crates/factory-dispatcher/src/registry.rs` | amend | schema v2 fields + `todo!()` `validate()` |
| `crates/factory-dispatcher/src/partition.rs` | new | Pure partition function with `todo!()` |
| `crates/factory-dispatcher/src/host/emit_event.rs` | amend | 4 new event type stubs |
| Workspace root `Cargo.toml` | amend | Add new crate to `[workspace.members]` |

### Step 2: test-writer

Author failing tests for all 17 ACs. Tests must fail at Red Gate.

#### VP-077 Kani harnesses (partition correctness — 4 harnesses)

Formally verify the pure partition function in `partition.rs`:
- Sync group never receives an async-classified plugin
- Async group never receives a sync-classified plugin
- All plugins are assigned (no silent drop)
- Drain window constant is DI-019 value

#### VP-078 lint integration (4 harnesses)

- Harness 1: lint invariant (schema v2 required; v1 rejected)
- Harness 2: bats integration (hooks-registry.toml + registry lint end-to-end)
- Harness 3: positive classification (9 telemetry plugins correctly classified async)
- Harness 4: serde-default (missing `async` field defaults to `false`)

#### VP-079 fault-injection (5 scenarios for the 4 new event types)

Tests cover each of the 4 new event types emitted by `host/emit_event.rs` plus
one combined drain-window scenario.

#### Latency canary (AC-014)

Scaffolding for sync_group p95 latency measurement. Must show ≤ 500ms p95.

**Fixture annotation:** Legacy-bash-adapter usage in test fixtures is acceptable
transitional infrastructure per VP-078 v1.8 + VP-079 v1.6 annotations. Test
fixtures do not constitute new plugin authorship.

### Step 3: implementer (TDD micro-commits, per S-15.01 §Tasks)

Complete each task in order. Each task is a separate commit.

| Task | File(s) | What to implement |
|------|---------|-------------------|
| T-3a | `registry.rs` | Schema v2: `schema_version: u32` field, serde-default = 1, `async: Option<bool>` per plugin entry |
| T-3b | `partition.rs` | Pure partition function (sync group / async group split); Kani-friendly |
| T-3c | `dispatcher` run loop | Sync await group; async spawn group; drain at ASYNC_DRAIN_WINDOW_MS |
| T-3d | ASYNC_DRAIN_WINDOW_MS | Wire constant via DI-019 reference; do NOT hardcode 100 anywhere |
| T-3e | `host/emit_event.rs` | 4 new event type emissions; amend existing file (do NOT create `event_catalog.rs`) |
| T-3f | `registry.rs` validate() | Lint invariant: hard-error on schema_version < 2 or missing; async field type-check |
| T-3g | `hooks.json.template` | Envelope flip: `on_error: "block"` across all 5 platform variants + `.template` root |
| T-3h | `hooks-registry.toml` | 9 telemetry plugins → `async = true`; `schema_version = 2` |
| T-3i | `lint-registry-async-invariant` | Native WASM Rust crate authored + built; registered in `hooks-registry.toml` with `plugin = "hook-plugins/lint-registry-async-invariant.wasm"` (NOT legacy-bash-adapter) |

### Step 4: demo-recorder (5 demos under `docs/demo-evidence/S-15.01/`)

Per AC-016, produce evidence for each scenario:

| Demo | Scenario |
|------|----------|
| (a) | Before-state silent block: replay prism scenario (validate-template-compliance 55 silent blocks) |
| (b) | After-state visible block: same scenario, now surfaced correctly |
| (c) | Latency canary measurement: sync_group p95 ≤ 500ms |
| (d) | Schema-mismatch hard error: v1 registry → dispatcher rejects with clear error |
| (e) | Async telemetry preserved within drain window: telemetry plugins complete before drain expires |

### Step 5: pr-manager

PR targeting `develop`. Must include:
- Full demo evidence (link to `docs/demo-evidence/S-15.01/`)
- BC traceability table (all 12 BCs)
- VP evidence links (VP-077 Kani output, VP-078 CI lint, VP-079 fault-injection)
- AI review pass
- Security review pass
- Merge per autonomy level

---

## 5. Lessons Captured (Apply to F4)

These were learned during F2 and F3 passes and are non-negotiable process
discipline for F4.

1. **Most correct, not fastest.** User-corrected at multiple gates this cycle
   when speed was chosen over correctness. F4 implementation must apply the
   same discipline at every task boundary.

2. **Byte-for-byte grep beats visual verification.** Apply grep-based
   verification to every version label, constant reference, and field name
   after every fix burst. Visual scanning misses multi-file drift.

3. **Sibling-fix discipline.** When amending one artifact (e.g., BC body),
   scan every citing sibling (e.g., References table in the story, VP table in
   ARCH-INDEX). Fixes must propagate atomically.

4. **Architects must check whether cited precedents are themselves under
   migration.** In F3 pass-2, the architect cited 6 bash plugin precedents
   for the new lint plugin. All 6 were legacy-bash-adapter entries pending
   WASM migration. The WASM-rule corrected this. Do not cite legacy entries as
   precedents for new work.

5. **WASM-rule retroactive audit applies to all session artifacts.** The audit
   at commit 4a73006 swept all session-modified artifacts. F4 must continue
   WASM conformance: any new plugin = native WASM crate. This is not optional.

---

## 6. Critical Paths Reference

### Specs

| Artifact | Path |
|----------|------|
| ADR-019 v1.8 | `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` |
| BC-1.14.001 | `.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md` |
| BC-7.06.001 | `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` |
| BC-9.01.006 | `.factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md` |
| BC-3.08.001 | `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` |
| BC-1.08.001 | `.factory/specs/behavioral-contracts/ss-01/BC-1.08.001.md` |
| VP-077 | `.factory/specs/verification-properties/VP-077.md` |
| VP-078 | `.factory/specs/verification-properties/VP-078.md` |
| VP-079 | `.factory/specs/verification-properties/VP-079.md` |
| DI-019 | `.factory/specs/domain-spec/invariants.md` (search `DI-019`) |
| S-15.01 | `.factory/stories/S-15.01-plugin-async-semantics.md` |
| E-15 epic | `.factory/stories/epics/E-15-plugin-async-semantics.md` |

### Cycle artifacts

| Artifact | Path |
|----------|------|
| F1 delta analysis | `cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` |
| F2 adversary passes 1–10 | `cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-{1..10}.md` |
| F3 adversary passes 1–5 | `cycles/v1.0-feature-plugin-async-semantics-pass-1/F3-S-15.01-adversary-pass-{1..5}.md` |
| WASM-rule audit | `cycles/v1.0-feature-plugin-async-semantics-pass-1/wasm-rule-audit.md` |
| F-P2-001 mechanism investigation | `cycles/v1.0-feature-plugin-async-semantics-pass-1/F-P2-001-mechanism-investigation.md` |
| Cycle burst-log | `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` |

---

## 7. Reference Precedents for Implementer

### The WASM plugin precedent (S-13.01 — validate-artifact-path)

This is the correct model for `lint-registry-async-invariant`. Do NOT use
legacy-bash-adapter entries as a model.

| Item | Path |
|------|------|
| Rust crate source | `crates/hook-plugins/validate-artifact-path/` |
| WASM artifact | `plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm` |
| Registration in hooks-registry.toml | Search `validate-artifact-path` in `plugins/vsdd-factory/hooks-registry.toml` |

### vsdd-hook-sdk

Path: `crates/hook-sdk/`

The lint plugin must use vsdd-hook-sdk for its WASM interface, matching the
validate-artifact-path precedent.

---

## 8. Post-Compact Workflow

After `/compact`, the session starts fresh. To resume F4:

1. Read `STATE.md` — confirms F3 CONVERGED, F4 pending.
2. Read this file (`F4-handoff.md`) — full dispatch context.
3. Read `stories/S-15.01-plugin-async-semantics.md` v1.6 — 17 ACs, 30 files, all tasks.
4. Issue: **"go F4 Option A"**

The orchestrator dispatches: stub-architect → test-writer → implementer →
demo-recorder → pr-manager, in sequence.
