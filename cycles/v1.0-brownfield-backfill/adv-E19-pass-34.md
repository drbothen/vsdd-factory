# Adversarial Review — E-19 Pass 34 (post-D-787 delta; perimeter = epic v1.22 + full E-19 suite at D-787 versions)

**Perimeter:** epic v1.22 + S-19.01..S-19.07 at D-787 versions + STORY-INDEX v4.165 E-19 section + VP-INDEX v2.55 VP-094..VP-101 + BC-5.42.001 v1.5 + BC-4.13.001 v1.12 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ADR-025 v1.12 + ADR-030 v1.3
**Reviewer:** fresh-context adversary (Iron Law; rubric policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 (1 finding)
**Streak:** 0/3 (pass-34 NOT-CLEAN; F-P34-001 CLOSED in D-789 fix burst)
**Model family:** Claude Opus 4.7

## Part A — D-787 Delta Verification + New Findings

### Amendment 1 — ADR-025 v1.11 → v1.12 (F-P33-002: §Decision 18 Deliverables path column corrected)

F-P33-002 fix applied — §Decision 18 Deliverables table path column corrected for all three rows:
- (a) dispatcher host-function implementation: `host.rs` → `host/read_prefix.rs` ✓
- (b) hook-sdk safe-wrapper layer: `sdk.rs` → `host.rs` ✓
- (c) hook-sdk FFI boundary: `host_ffi.rs` → `ffi.rs` ✓

Input-hash propagated ✓. No further findings in ADR-025 v1.12.

### Amendment 2 — BC-5.42.001 v1.4 → v1.5 (O-P33-001: §Traceability L2 Domain Invariants TBD → none)

O-P33-001 fix applied — §Traceability L2 Domain Invariants now reads `none (pipeline-workflow constraint; no L2 domain invariants applicable)` ✓. Input-hash 4fd18a4 ✓. Aligned to BC-1.17.001/BC-4.13.001/BC-2.07.001 convention ✓.

No further findings in BC-5.42.001 v1.5. POLICY 14 quintuple parity verified (version: frontmatter / body Changelog / modified[] / last_amended: / upstream-index) ✓.

### Amendment 3 — epic v1.21 → v1.22 (F-P33-001: EAC-003 BC-2.07.001 v1.3→v1.4 cite updated)

F-P33-001 fix applied — EAC-003 §Negative control B cite updated from `BC-2.07.001 v1.3 EC-007` to `BC-2.07.001 v1.4 EC-007` ✓. Input-hash a18ea87 ✓. STORY-INDEX E-19 section header updated to v1.22 ✓.

No further findings in epic v1.22.

### Amendment 4 — S-19.01 v1.15 → v1.16 (O-P33-001: BC-5.42.001 v1.4→v1.5 cite sweep ×3 sites)

O-P33-001 SW-leg applied — BC-5.42.001 v1.4→v1.5 cite sweep ×3 sites (BC table Version cell; AC gate cite; Token Budget cite) ✓. Input-hash d40bd21 unchanged ✓.

New finding identified during full E-19 suite review of BC-4.13.001 §Traceability:

**F-P34-001 MEDIUM — BC-4.13.001 v1.12 §Traceability ADR Reference row contains a volatile version pin `ADR-025 v1.2 (primary — all 10 decisions)` that is both POLICY 19-violating (version-pinned reference to a living ADR) and factually stale (ADR-025 is at v1.12 with 18 decisions; BC-4.13.001 is governed by Decisions 1, 14, 15, and 18 — none of which existed at ADR-025 v1.2).**

The ADR Reference cell in §Traceability reads:

| ADR Reference | ADR-025 v1.2 (primary — all 10 decisions); ADR-016 (artifact path guard pattern + `on_error = "continue"` precedent); ADR-019 (sync/async partition; `async = false` CI lint invariant); ADR-020 (Class A latency budget ≤1500ms p95) |

The volatile pin `ADR-025 v1.2 (primary — all 10 decisions)` was the version at BC-4.13.001 initial authoring (v1.0). Since then, ADR-025 has advanced through 12 versions (v1.2 → v1.12) adding Decisions 11–18 (per D-786/D-787 work). BC-4.13.001 is now directly governed by four specific Decisions that did not exist at v1.2:
- Decision 1 (verify-factory-lock guard; primary enforcement)
- Decision 14 (STATE_MD_MAX_BYTES=262144 + frontmatter-only parse; Precondition 3 / Invariant 9 — added at Phase-A)
- Decision 15 (host::read_prefix; Phase-B activation)
- Decision 18 (host::read_prefix deliverables; Phase-B migration path)

The claim "all 10 decisions" is factually wrong — ADR-025 v1.12 has 18 decisions (POLICY 19: content-stale volatile-pin). Sibling BCs BC-1.17.001/BC-2.07.001/BC-3.08.001/BC-5.42.001 all reference ADR-025 in stable §Decision-enumerated form — BC-4.13.001 is the sole outlier using a version-pinned scope claim.

This is a novel escape class: BC-internal Traceability-row sibling-sweep against a moving ADR anchor. The volatile pin has been present since BC-4.13.001 v1.0 (initial authoring) and was not corrected during any of the 12 subsequent passes, because prior fix bursts targeted body-content cites (invariants, Phase-A/B framing, VP Anchors) rather than §Traceability metadata rows. The finding class is distinct from the prior epic-EAC class (F-P33-001) and ADR-table class (F-P33-002).

**Severity:** MEDIUM. POLICY 19 HIGH-class finding tempered by: blast radius = 1 cell in 1 BC; sibling BCs already use stable §Decision form (targeted drift, not systemic); behavioral content of BC-4.13.001 is unaffected (traceability-only metadata row).

**Locus:** `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` §Traceability ADR Reference row — volatile version-pinned scope claim.

**Routing:** product-owner (BC §Traceability ADR Reference row correction); story-writer (S-19.02 and S-19.07 BC-4.13.001 version cite sweep per POLICY 8).

**Fix:** Product-owner BC-4.13.001 v1.12→v1.13: §Traceability ADR Reference row rewritten to stable §Decision-enumerated form: `ADR-025 §Decision 1 (verify-factory-lock guard; primary); §Decision 14 (STATE_MD_MAX_BYTES=262144 + frontmatter-only parse; Precondition 3 / Invariant 9); §Decision 15 (host::read_prefix; Phase-B activation); §Decision 18 (host::read_prefix deliverables; Phase-B migration path); ADR-016 (artifact path guard pattern + on_error="continue" precedent); ADR-019 (sync/async partition; async=false CI lint invariant); ADR-020 (Class A latency budget ≤1500ms p95)`. Input-hash 86fab85. Story-writer S-19.02 v1.15→v1.16: BC-4.13.001 v1.12→v1.13 cite sweep ×18 sites; input-hash d208e66. Story-writer S-19.07 v1.14→v1.15: BC-4.13.001 v1.12→v1.13 cite sweep ×12 sites; input-hash 83e8cc4. **CLOSED F-P34-001.**

### Full E-19 Suite Review

All D-787 amendments verified closed as documented above. No further findings in the full E-19 story suite (S-19.01 v1.16 / S-19.02 v1.15 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.14). STORY-INDEX E-19 section verified consistent with story versions at D-787 perimeter versions. BC-INDEX v3.86 / VP-INDEX v2.55 / ARCH-INDEX v2.97 verified consistent with the exception of F-P34-001 identified above. No other POLICY violations detected in full suite scan.

## Part B — Per-Policy Verification + Severity

### POLICY 19 — Volatile-pin sweep (ADR-025 version cites in BC §Traceability)

Grep: `grep -nE 'ADR-025 v[0-9]+\.[0-9]+'` across all E-19 BCs in §Traceability ADR Reference cells:

| BC | ADR Reference form | POLICY 19 status |
|----|-------------------|-----------------|
| BC-2.07.001 v1.4 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-5.42.001 v1.5 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-3.08.001 v1.19 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-1.17.001 v1.5 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-4.13.001 v1.12 | `ADR-025 v1.2 (primary — all 10 decisions)` (volatile version-pin) | ✗ FAIL — F-P34-001 |
| BC-2.02.011 v1.5 | does not reference ADR-025 | n/a |

Zero `ADR-025 v1.N` hits in stories (zero-match sentinel: grep of S-19.01..S-19.07 body for `ADR-025 v[0-9]` returns no live-body matches — stories cite ADR-025 by section name/Decision form only). POLICY 19 finding: BC-4.13.001 sole outlier. All other E-19 BCs stable ✓.

### POLICY 9 — VP arithmetic reconciliation

VP-094..VP-101 counts verified across VP-INDEX v2.55 + verification-architecture.md + verification-coverage-matrix.md: 34 integration / 5 kani / 5 proptest / total 101. All three documents agree on counts; no arithmetic staleness. POLICY 9 CLEAN ✓.

### POLICY 1/6/7/8/13/14/17/18 — Clean passes

POLICY 1 (append-only IDs): no new BC/VP/story IDs in this pass; verified. POLICY 6 (subsystem names): SS-04 references consistent with ARCH-INDEX. POLICY 7 (BC-INDEX title verbatim parity): BC-INDEX v3.86 BC-4.13.001 H1 unchanged (POLICY 7 CLEAN; title elision fix from O-P33-002 applied at D-787). POLICY 8 (BC frontmatter cycle propagation): BC-4.13.001 cite sweep required in S-19.02/S-19.07 — POLICY 8 cascades routed to story-writer leg ✓. POLICY 13 (standing disciplinary constraints): all active standing controls carried ✓. POLICY 14 (5-leg quintuple parity): applied to BC-4.13.001 v1.13 PO leg; carried to SW story cites. POLICY 17 (epic POLICY 17 EAC compliance): epic v1.22 EAC-001..EAC-005 verified ✓. POLICY 18 (input-hash non-placeholder): BC-4.13.001 v1.13 input-hash 86fab85 ✓; S-19.02 input-hash d208e66 ✓; S-19.07 input-hash 83e8cc4 ✓.

### Severity + Novelty

**Severity (B0/H0/M1/L0):** One MEDIUM finding (F-P34-001). Total: 1 item. Severity improvement from pass-33 (4 total) — regression broken; first single-finding pass since pass-31. No HIGH or BLOCKER items. F-P34-001 MEDIUM is a POLICY 19 volatile-pin in §Traceability metadata (BC-internal Traceability-row sibling-sweep miss against a moving ADR anchor).

**Novelty:** LOW. F-P34-001 (BC §Traceability volatile ADR version pin) is a new escape sub-class: prior sweeps fixed body-content BC-version cites (invariants, Phase clauses, VP Anchors, EAC cites) and ADR §Decision table cells, but did not extend to BC §Traceability ADR Reference row metadata. The §Traceability ADR Reference cell is a metadata cell (not behavioral body), which explains why it was not included in prior body-content cite sweeps. The class is adjacent to the ADR-path-drift class (F-P33-002) but distinct: that was ADR body cell staleness; this is BC metadata cell volatile-pin against the ADR. Sibling BCs were already in stable form (targeted drift, not systemic), and zero story hits confirm the stories use §Decision-form cites throughout.

**Cascade trajectory (passes 22–34):** 4→3→4→2→2→4→6→5→4→1→3→4→1. Trajectory tail (passes 31–34): →3→4→1. Single-finding pass. Asymptotic floor pattern continues.

## Fix Burst Closure (D-789)

**Fix burst D-789 applied.** Product-owner BC-4.13.001 v1.12→v1.13 (F-P34-001: §Traceability ADR Reference volatile-pin → stable §Decision 1/14/15/18 form; input-hash e1e1a0a→86fab85). Story-writer S-19.02 v1.15→v1.16 (BC-4.13.001 v1.12→v1.13 cite sweep ×18 sites; input-hash d377821→d208e66). Story-writer S-19.07 v1.14→v1.15 (BC-4.13.001 v1.12→v1.13 cite sweep ×12 sites; input-hash 938e7fb→83e8cc4). State-manager BC-INDEX v3.86→v3.87 (BC-4.13.001 row Version cell v1.13 + F-P34-001/D-789 change note). STORY-INDEX v4.165→v4.166 (S-19.02 row v1.16; S-19.07 row v1.15; BC coverage BC-4.13.001 v1.13; delivery-summary pass-34 note). VP-INDEX v2.55 UNCHANGED (exhaustive). ARCH-INDEX v2.97 UNCHANGED (exhaustive). STATE.md v5.39→v5.40 (SM: D-789 advance; trajectory →1→3→4→1; checkpoint refresh). Commits: 9a3dc191 (PO BC-4.13.001 v1.13) + 5238e5d5 (SW S-19.02 v1.16 + S-19.07 v1.15). Streak 0/3. NEXT: E-19 adv pass-35 (fresh context).
