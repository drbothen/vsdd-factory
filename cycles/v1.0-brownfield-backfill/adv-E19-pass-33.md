# Adversarial Review — E-19 Pass 33 (post-D-786 delta; perimeter = epic v1.21 + full E-19 suite at D-786 versions)

**Perimeter:** E-19 epic v1.21 + S-19.01..S-19.07 (at D-786 versions) + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.4 + BC-4.13.001 v1.12 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095.md v1.1 + VP-096.md v1.1 + ADR-025 v1.11 (§Decision 18 in scope as read_prefix deliverable file list)
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 2 / LOW 2 (4 total)
**Streak:** 0/3 (pass-33 NOT-CLEAN; all items CLOSED in D-787 fix burst)
**Model family:** Claude Sonnet 4.6
**Delta artifact versions verified:** epic v1.21 (was v1.20; O-P32-02 tautological clause dropped); S-19.03 v1.16 (was v1.15; BC-2.07.001 v1.3→v1.4 cite sweep ×3); BC-2.07.001 v1.4 (was v1.3; O-P32-01 DI-TBD → none); ADR-025 v1.11 (was v1.10; F-P32-001 §Decision 15 Phase-B stale corrected).

## Part A — D-786 Delta Verification + New Findings

### Amendment 1 — E-19 epic v1.20 → v1.21 (O-P32-02: tautological "subsequently amended through v1.5" clause dropped)

O-P32-02 fix applied — §Out of Scope BC-1.17.001 bullet now reads "LANDED as v1.5" without the tautological parenthetical ✓. Input-hash 77985d8 ✓. STORY-INDEX E-19 section header updated to v1.21 ✓.

New finding identified during epic body review:

**F-P33-001 MEDIUM — E-19 epic v1.21 EAC-003 §Negative control B references BC-2.07.001 at stale version v1.3; current version is v1.4 (D-786 fix burst O-P32-01).**

EAC-003 in the epic body includes a negative-control B clause tied to BC-2.07.001 path-not-allowed semantics. The cite in EAC-003 reads "BC-2.07.001 v1.3 EC-007" — v1.3 was the version at epic authoring. D-784 (pass-30 fix burst) advanced BC-2.07.001 to v1.3 (F-P30-002: input-hash placeholder retired) and D-786 (pass-32 fix burst) advanced BC-2.07.001 to v1.4 (O-P32-01: DI-TBD → none). The epic's EAC-003 body was not swept when BC-2.07.001 was advanced at D-786, leaving a stale version cite that will mislead the implementer about which BC version governs EAC-003 execution.

**Locus:** E-19 epic v1.21 EAC-003 — "BC-2.07.001 v1.3 EC-007" cite.
**Routing:** story-writer (epic body content — EAC acceptance criteria version cite).
**Fix:** Story-writer epic v1.21→v1.22: EAC-003 BC-2.07.001 v1.3→v1.4 cite update; input-hash 77985d8→a18ea87. **CLOSED F-P33-001.**

### Amendment 2 — S-19.03 v1.15 → v1.16 (BC-2.07.001 v1.3→v1.4 cite sweep ×3 sites)

Cite sweep ×3 applied ✓: BC table Version cell v1.3→v1.4; AC-001 gate cite v1.3→v1.4; Token Budget cite v1.3→v1.4. Input-hash 8d1225d unchanged ✓. No further findings in S-19.03 v1.16.

### Amendment 3 — BC-2.07.001 v1.3 → v1.4 (O-P32-01: §Traceability L2 Domain Invariants DI-TBD → none)

O-P32-01 fix applied — §Traceability L2 Domain Invariants now reads `none (host-ABI operational; no L2 domain invariants applicable)` ✓. Input-hash 9d60fc5 ✓. Aligned to BC-1.17.001/BC-4.13.001 convention ✓.

One new finding identified in BC-2.07.001 sibling sweep:

**O-P33-001 LOW — BC-5.42.001 v1.4 §Traceability §L2 Domain Invariants cell retains TBD placeholder — pass-32 sibling-sweep miss.**

BC-5.42.001 v1.4 §Traceability section contains `L2 Domain Invariants: TBD`. The D-784 pass-30 fix burst retired the DI-TBD placeholder in BC-1.17.001 v1.5 and BC-4.13.001 v1.12; D-786 pass-32 fix burst retired it in BC-2.07.001 v1.4. BC-5.42.001 (pr-manager READY-verdict enforcement) was authored at the same session with the same placeholder pattern and was not swept at D-784 or D-786. The convention is clear: `domain_invariants:` must be an affirmative statement — either a list of invariant IDs or `[] (none)` when no domain invariants apply. For BC-5.42.001 (READY-verdict SHA-pinning, stale-verdict detection, release-PR squash prevention), there are no applicable L2 Domain Invariants — this is a CI/CD pipeline-workflow constraint, not a domain-model invariant. Orchestrator adjudication: ALIGN (PO to retire TBD; SW to sweep S-19.01 cite sites).

**Locus:** BC-5.42.001 v1.4 §Traceability §L2 Domain Invariants — value "TBD".
**Routing:** product-owner (BC body content); story-writer (S-19.01 BC-5.42.001 cite propagation sweep).
**Fix:** Product-owner BC-5.42.001 v1.4→v1.5: §Traceability L2 Domain Invariants TBD → none (pipeline-workflow constraint; no L2 domain invariants applicable); aligned to BC-1.17.001/BC-4.13.001/BC-2.07.001 convention; input-hash 509c8f8→4fd18a4 (within-burst refresh: S-19.01 updated same burst; see D-782/D-783 precedent). Story-writer S-19.01 v1.15→v1.16: BC-5.42.001 v1.4→v1.5 cite sweep ×3 sites; input-hash d40bd21 unchanged. **CLOSED O-P33-001.**

### Amendment 4 — ADR-025 v1.10 → v1.11 (F-P32-001: §Decision 15 Phase-B STATE_MD_MAX_BYTES removal + 262144 Phase-A-historical framing)

F-P32-001 fix applied — §Decision 15 Primary consumers paragraph now states STATE_MD_MAX_BYTES is removed at S-19.07 and read_prefix is called with max_bytes=8192 per BC-4.13.001 §Precondition 3 Phase-B ✓. Truncation-example sentence reframed: 262144 labeled Phase-A-historical; Phase-B bound is 8192 ✓. Input-hash propagated ✓.

New finding identified in ADR-025 v1.11 §Decision 18:

**F-P33-002 MEDIUM — ADR-025 v1.11 §Decision 18 Deliverables table path column lists stale file paths inconsistent with the actual hook-sdk and dispatcher crate structure.**

ADR-025 v1.11 §Decision 18 ("host::read_prefix — bounded partial-read host function") includes a deliverables table with three rows. The path column for these rows does not match the actual crate file structure established by the S-19.06 implementation:

(a) **Dispatcher host-function implementation:** §Decision 18 path column shows `host.rs` (top-level crates/factory-dispatcher/src/host.rs). The actual implementation deliverable is `host/read_prefix.rs` — the dispatcher uses a host/ submodule for host-function implementations (consistent with host/read_file.rs, host/path_util.rs established earlier). `host.rs` at the crate root is the module declaration, not the implementation file.

(b) **hook-sdk safe-wrapper layer:** §Decision 18 path column shows `sdk.rs`. The actual deliverable is `host.rs` within the hook-sdk crate (crates/hook-sdk/src/host.rs) — the safe-wrapper layer that exposes the Rust-friendly `host_read_prefix(max_bytes, buf)` signature. `sdk.rs` does not exist in the hook-sdk structure.

(c) **hook-sdk FFI boundary:** §Decision 18 path column shows `host_ffi.rs`. The actual FFI boundary lives in `ffi.rs` (crates/hook-sdk/src/ffi.rs) — the raw extern "C" declaration of `__vsdd_host_read_prefix`. `host_ffi.rs` was an early working name that was not used at implementation.

The stale path column will cause implementer confusion when wiring S-19.06 Task deliverables to the ADR §Decision 18 table.

**Locus:** ADR-025 v1.11 §Decision 18 Deliverables table — path column rows (a), (b), (c).
**Routing:** architect (ADR-025 owner; §Decision 18 path column correction).
**Fix:** Architect ADR-025 v1.11→v1.12: §Decision 18 path column corrected — (a) host/read_prefix.rs (dispatcher host-function implementation); (b) host.rs (hook-sdk safe-wrapper layer); (c) ffi.rs (hook-sdk FFI boundary). **CLOSED F-P33-002.**

### Full E-19 Suite Review — BC-INDEX POLICY 7 Compliance

**O-P33-002 LOW (POLICY 7 ADVISORY) — BC-INDEX v3.85 catalog row for BC-2.07.001 elides "error code" from the title cell, violating POLICY 7 verbatim-parity with H1.**

BC-2.07.001 H1 title: "BC-2.07.001: host::read_file absent-file semantics — codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied".

BC-INDEX v3.85 catalog row title cell (§ss-02 section): "host::read_file absent-file semantics: codes::NOT_FOUND (-5) additive, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied".

The title cell reads "additive," (omitting "error code") where H1 reads "additive error code,". POLICY 7 requires BC-INDEX title cells to maintain verbatim H1 parity. The elision of "error code" is a truncation introduced when the BC-INDEX row was written, not in the H1. This is a POLICY 7 violation.

**Locus:** BC-INDEX v3.85 §ss-02 BC-2.07.001 catalog row title cell — "additive," vs H1 "additive error code,".
**Routing:** state-manager (BC-INDEX catalog row title cell correction; POLICY 7 compliance).
**Fix:** State-manager BC-INDEX v3.85→v3.86: BC-2.07.001 catalog row title cell — "additive," → "additive error code,". **CLOSED O-P33-002.**

### Full Story Suite Verification

All D-786 amendments verified closed as documented above. No further findings in the full E-19 story suite (S-19.01 v1.15 / S-19.02 v1.15 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.14). STORY-INDEX E-19 section verified consistent with story versions (post-D-786). BC-INDEX v3.85 / VP-INDEX v2.55 / ARCH-INDEX v2.96 verified consistent (pre-D-787 fix burst) with the exception of O-P33-002 identified above. No other POLICY violations detected.

## Part B — Severity + Novelty

**Severity (B0/H0/M2/L2):** Two MEDIUM findings (F-P33-001, F-P33-002) and two LOW observations (O-P33-001, O-P33-002). Total: 4 items. Severity increase from pass-32 (3 total) — regression from 3 to 4. No HIGH or BLOCKER items. F-P33-001 MEDIUM is a BC version cite staleness (EAC-003 stale v1.3 vs current v1.4); F-P33-002 MEDIUM is an ADR §Decision 18 deliverables table path column staleness (3 incorrect paths); both are substantive implementer-confusion risks if unaddressed. O-P33-001 LOW is a recurrence of the DI-TBD sibling-sweep-miss class (third instance: BC-1.17.001 D-784, BC-2.07.001 D-786, BC-5.42.001 D-787). O-P33-002 LOW is a POLICY 7 title-cell elision (new sub-class: elision introduced at BC-INDEX row authoring, not at BC H1 authoring).

**Novelty:** LOW-MEDIUM. F-P33-001 (EAC-003 stale cite) is a recurrence of the partial-sweep-escape class: the D-786 fix burst swept S-19.03 but did not sweep the epic EAC body for the same BC version change. F-P33-002 (ADR §Decision 18 stale paths) is a new instance of the ADR-path-drift class: working-names for implementation files were used at §Decision 18 authoring and were not corrected when the actual implementation settled on final names. O-P33-001 is the third DI-TBD sibling-sweep miss — the class is now a confirmed systematic gap in the per-burst sweep discipline for BC §Traceability cells. O-P33-002 (BC-INDEX title elision) is a new sub-class: unlike prior POLICY 7 violations where H1 was changed post-indexing, here the elision was introduced at index authoring.

**Cascade trajectory (pass-22 onward, count):** 4→3→4→2→2→4→6→5→4→1→3→4. Trajectory tail (passes 30-33): →4→1→3→4. Second consecutive low-single-digit pass; asymptotic floor pattern continues. Pass-33 regression (3→4) driven by four independently-routable items, all closed.

## Fix Burst Closure (D-787)

**Fix burst D-787 applied.** Architect ADR-025 v1.11→v1.12 (F-P33-002: §Decision 18 path column corrected — host/read_prefix.rs dispatcher implementation; host.rs hook-sdk safe-wrapper; ffi.rs hook-sdk FFI boundary). Product-owner BC-5.42.001 v1.4→v1.5 (O-P33-001: §Traceability L2 Domain Invariants TBD → none; input-hash 509c8f8→4fd18a4 within-burst refresh). Story-writer epic v1.21→v1.22 (F-P33-001: EAC-003 BC-2.07.001 v1.3→v1.4; input-hash 77985d8→a18ea87). Story-writer S-19.01 v1.15→v1.16 (O-P33-001: BC-5.42.001 v1.4→v1.5 cite sweep ×3 sites; input-hash d40bd21 unchanged). State-manager BC-INDEX v3.85→v3.86 (O-P33-002: BC-2.07.001 title cell "additive," → "additive error code,"; BC-5.42.001 catalog row v1.5 note). ARCH-INDEX v2.96→v2.97 (SM: ADR-025 v1.12 row note). STORY-INDEX v4.164→v4.165 (SM: S-19.01 row v1.16; epic header v1.22; BC coverage BC-5.42.001 v1.5; delivery summary pass-33 note). VP-INDEX v2.55 UNCHANGED. STATE.md v5.37→v5.38 (SM: D-787 advance; trajectory →4→1→3→4; checkpoint refresh). Streak 0/3. NEXT: E-19 adv pass-34 (fresh context).
