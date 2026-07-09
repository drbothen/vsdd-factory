# Adversarial Review — E-19 Pass 35 (post-D-789 delta; perimeter = epic v1.22 + full E-19 suite at D-789 versions)

**Perimeter:** epic v1.22 + S-19.01 v1.16 / S-19.02 v1.16 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.15 + STORY-INDEX v4.166 + VP-INDEX v2.55 VP-094..VP-101 + BC-5.42.001 v1.5 + BC-4.13.001 v1.13 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ADR-025 v1.12 + ADR-030 v1.3 + BC-INDEX v3.87 + ARCH-INDEX v2.97
**Reviewer:** fresh-context adversary (Iron Law; rubric policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 1 / LOW 0 (2 findings + 1 out-of-perimeter observation)
**Streak:** 0/3 (pass-35 NOT-CLEAN; F-P35-001 HIGH + F-P35-002 MEDIUM CLOSED in D-790 fix burst)
**Model family:** Claude Opus 4.7

## Part A — D-789 Delta Verification + New Findings

### Amendment 1 — BC-4.13.001 v1.12 → v1.13 (F-P34-001: §Traceability ADR Reference volatile-pin → stable §Decision 1/14/15/18 form)

F-P34-001 fix applied — §Traceability ADR Reference row rewritten from volatile `ADR-025 v1.2 (primary — all 10 decisions)` to stable §Decision-enumerated form. POLICY 14 quintuple parity verified (version: frontmatter / body Changelog / modified[] / last_amended: / upstream-index). Input-hash 86fab85 ✓. No recurrence of the version-pin class. BC-INDEX v3.87 row Version cell updated to v1.13 ✓.

New findings identified during thorough re-read of BC-4.13.001 v1.13 §Traceability and §Description — see F-P35-001 and F-P35-002 below.

### Amendment 2 — S-19.02 v1.15 → v1.16 (F-P34-001: BC-4.13.001 v1.12→v1.13 cite sweep ×18 sites)

F-P34-001 SW-leg applied — BC-4.13.001 v1.12→v1.13 cite sweep ×18 sites verified complete. Input-hash d208e66 ✓. D-779 whole-file predicate PASS (zero live-body v1.12 residuals). D-759 two-sided preflight PASS.

No further findings in S-19.02 v1.16.

### Amendment 3 — S-19.07 v1.14 → v1.15 (F-P34-001: BC-4.13.001 v1.12→v1.13 cite sweep ×12 sites)

F-P34-001 SW-leg applied — BC-4.13.001 v1.12→v1.13 cite sweep ×12 sites verified complete. Input-hash 83e8cc4 ✓. D-779 whole-file predicate PASS (zero live-body v1.12 residuals).

No further findings in S-19.07 v1.15.

### Amendment 4 — BC-INDEX v3.86 → v3.87 (BC-4.13.001 row Version cell v1.13 + F-P34-001/D-789 change note)

BC-INDEX v3.87 row for BC-4.13.001 updated to v1.13 with F-P34-001/D-789 change note ✓. POLICY 7 H1 title verbatim parity maintained ✓. total_bcs 1,977 UNCHANGED ✓.

No further findings in BC-INDEX v3.87.

### Amendment 5 — STORY-INDEX v4.165 → v4.166 (S-19.02 row v1.16; S-19.07 row v1.15; BC coverage BC-4.13.001 v1.13; delivery-summary pass-34 note)

STORY-INDEX v4.166 E-19 section updated: S-19.02 row v1.16 d208e66 ✓; S-19.07 row v1.15 83e8cc4 ✓; BC coverage line `BC-4.13.001 v1.13 (S-19.02 Phase-A + S-19.07 Phase-B)` ✓. Delivery-summary pass-34 note prepended ✓. POLICY 14 5-leg ✓ on all five amendments.

No further findings in STORY-INDEX v4.166 E-19 section.

---

**F-P35-001 HIGH — BC-4.13.001 v1.13 §Traceability ADR Reference row cites `§Decision 18` but ADR-025 v1.12 has exactly 15 Decisions; `§Decision 18` does not exist — the intended target is Concrete Deliverables row `D18` at line 1210 (host::read_prefix).**

The D-789 fix burst rewrote the §Traceability ADR Reference row to stable §Decision-enumerated form. The rewritten row reads:

```
| ADR Reference | ADR-025 §Decision 1 (verify-factory-lock guard; primary); §Decision 14 (STATE_MD_MAX_BYTES=262144 + frontmatter-only parse; Precondition 3 / Invariant 9); §Decision 15 (host::read_prefix; Phase-B activation); §Decision 18 (host::read_prefix deliverables; Phase-B migration path); ADR-016 (artifact path guard pattern + `on_error = "continue"` precedent); ADR-019 (sync/async partition; `async = false` CI lint invariant); ADR-020 (Class A latency budget ≤1500ms p95) |
```

The token `§Decision 18` in the fourth clause is a mis-anchor. ADR-025 v1.12 contains exactly 15 `### Decision` headers (grep `^### Decision`):

```
109:### Decision 1: Primary enforcement — native-WASM PreToolUse guard `verify-factory-lock`
146:### Decision 2: Lock state — `factory_lock` frontmatter block in STATE.md
174:### Decision 3: Session identity — `git config user.email` (developer-level, coarse)
195:### Decision 4: Block semantics and refusal message
224:### Decision 5: Stale-lock escape — TTL auto-expiry AND `/factory-unlock --force`
283:### Decision 6: Acquire/release UX — explicit `/factory-lock` and `/factory-unlock` skills
336:### Decision 7: Crash behavior — `on_error = "continue"` (fail-open)
362:### Decision 8: Complementary mitigation — blind-push fix in `state-burst` (secondary, standalone)
388:### Decision 9: Future / Out of Scope — git-ref CAS upgrade path
404:### Decision 10: Single-developer behavior — hard invariant, no added human action
425:### Decision 11: Automatic heartbeat renewal enforcement — executable skill step + PreToolUse push gate
545:### Decision 12: `verify-state-timestamp-refresh` Rust WASM PreToolUse guard (v1.6)
1066:### Decision 13 — Host ABI NOT_FOUND return code (-5)
1096:### Decision 14 — verify-factory-lock read-cap 262144 + frontmatter-only parse
1132:### Decision 15 — Host ABI `read_prefix` additive function
```

The last `### Decision` header is at line 1132: `### Decision 15`. There is no `### Decision 18` in ADR-025 v1.12. The deliverable `D18` exists at line 1210 in the Concrete Deliverables table:

```
1210:| D18 | `host::read_prefix` host function | `crates/factory-dispatcher/src/host/read_prefix.rs` ... | Additive host function per Decision 15. Signature: `read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> i32`. ... |
```

`D18` is a row in the `## Concrete Deliverables` section — it is not a `### Decision` heading. The same-document convention in BC-4.13.001 for referencing deliverable D9 uses the prefix form `§D9` (Test Vectors section line 373: "These are the D9 bats integration test vectors (from ADR-025 §D9):", VPs row line 443: "Bats D9 test coverage per ADR-025 §D9 (9 scenarios)"). The fourth clause `§Decision 18` should therefore read `Deliverable D18` (or `and Deliverable D18`).

**Escape class:** fix-introduces-adjacent-defect. The D-789 fix executor verified that `§Decision 1` (the previously-flagged anchor) resolved correctly; and verified `§Decision 14` and `§Decision 15` against the `^### Decision 14` / `^### Decision 15` grep hits. However, the proposed-fix text carried `§Decision 18` verbatim from the F-P34-001 finding's proposed-fix wording without independently greping the target ADR for the existence of `### Decision 18`. Per the production-grade default, fix executors must existence-grep EVERY anchor introduced in the replacement text, not only the previously-flagged anchor.

**Severity:** HIGH. POLICY 4 semantic-anchoring failure in a §Traceability ADR Reference cell — a non-existent Decision is cited as a governance anchor. While the behavioral content of BC-4.13.001 is unaffected (traceability-only cell), a reviewer following the §Traceability chain reaches a dead-end: `§Decision 18` in ADR-025 does not exist, making the traceability claim non-verifiable. This is a higher severity than F-P34-001 (which was a volatile-pin, still technically navigable) because the anchor itself does not exist.

**Locus:** `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` §Traceability ADR Reference row — `§Decision 18` mis-anchor.

**Routing:** product-owner (BC §Traceability ADR Reference row correction to `Deliverable D18`); story-writer (S-19.02 v1.16 + S-19.07 v1.15: the replacement text was propagated verbatim into cite sweeps — verify no story body contains the non-existent `§Decision 18` anchor; input-hash restamps required).

**Fix:** Product-owner BC-4.13.001 v1.13→v1.14: §Traceability ADR Reference row: `§Decision 18` → `and Deliverable D18` (following the `§D9` precedent already established in the BC body for deliverables). Story-writer S-19.02: sweep for `§Decision 18` residuals across story body; if present, replace with `Deliverable D18`; input-hash restamp. Story-writer S-19.07: same sweep. **CLOSED F-P35-001: PO commit 97a1b9ed (BC-4.13.001 v1.13→v1.14 → `and Deliverable D18` form); SW commit 42c0a7e9 (S-19.02 v1.16→v1.17 input-hash 604f45d, 18 sites; S-19.07 v1.15→v1.16 input-hash 534c85c, 12 sites).**

---

**F-P35-002 MEDIUM — BC-4.13.001 v1.13 §Description second paragraph governance enumeration stale: lists only "Decisions 1, 2, 3, 4, 7, 9, and 10" and "deliverables D1, D2, and D9" — missing Decisions 14/15 and Deliverable D18 which were added at BC v1.4 (D-755, F-P1-004) and v1.6 (D-755, F-P4-001) respectively.**

The §Description second paragraph of BC-4.13.001 v1.13 (line 63) reads:

```
This BC covers ADR-025 Decisions 1, 2, 3, 4, 7, 9, and 10, and deliverables D1, D2, and D9.
```

This enumeration was authored at BC v1.0 and was never updated when the governing surface was extended:
- BC v1.4 (D-755, F-P1-004): Decision 14 added (STATE_MD_MAX_BYTES=262144 + frontmatter-only parse; Precondition 3 / Invariant 9)
- BC v1.6 (D-755, F-P4-001): Decision 15 added (host::read_prefix; Phase-B activation) and Deliverable D18 added (host::read_prefix deliverables; Phase-B migration path)

The stale enumeration ("Decisions 1, 2, 3, 4, 7, 9, and 10 / D1, D2, D9") is rendered **visibly contradictory** by the v1.13 §Traceability ADR Reference row (which correctly lists §Decision 1, §Decision 14, §Decision 15, and D18 in the governance surface). A reviewer comparing §Description to §Traceability sees conflicting governance scopes in the same document at the same version. This is a S-7.01 partial-fix regression pattern: the D-789 fix (F-P34-001) correctly updated §Traceability but left §Description stale, creating a same-document contradiction.

**POLICY 4:** §Description governance enumeration is a semantic anchor — a reader relying on it as the complete governance surface would miss Decisions 14/15 and D18, which define the byte-cap, frontmatter-only parse, read_prefix Phase-B migration, and the Concrete Deliverables traceability. **POLICY 5:** within a single BC version (v1.13), §Description and §Traceability claim contradictory governance surfaces. This passes the POLICY 5 intra-document contradiction bar.

**Severity:** MEDIUM. The stale enumeration in §Description creates a verifiable same-document POLICY 4/5 violation. Behavioral content (Preconditions, Invariants, ECs, TVs) is unaffected; this is a governance-surface description gap. MEDIUM (not HIGH) because the §Traceability ADR Reference row (the normative governance cell) is correctly updated, and readers following the normative cell reach the correct set. §Description is a summary paragraph, not a normative anchor.

**Locus:** `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` §Description paragraph 2, line 63 — stale governance enumeration.

**Routing:** product-owner (BC §Description paragraph extension to include Decisions 14/15 + Deliverable D18).

**Fix:** Product-owner BC-4.13.001 v1.13→v1.14 (same burst as F-P35-001): §Description paragraph 2 updated to `"This BC covers ADR-025 Decisions 1, 2, 3, 4, 7, 9, 10, 14, and 15, and deliverables D1, D2, D9, and D18."` **CLOSED F-P35-002: PO commit 97a1b9ed (BC-4.13.001 v1.13→v1.14; same PO burst as F-P35-001).**

---

**O-P35-001 LOW [process-gap, out-of-perimeter] — BC-5.40.001 v1.5 and BC-6.23.001 v1.2 §Traceability ADR Reference rows carry the same POLICY 19 volatile-pin class: `ADR-025 v1.2 (Decisions 2, 3, 5, 8, 10 and deliverables D3, D6)` and `ADR-025 v1.2 (Decisions 5 Path B, 6, 8 and deliverables D4, D5, D7, D8)` respectively.**

BC-5.40.001 v1.5 §Traceability ADR Reference row:
```
| ADR Reference | ADR-025 v1.2 (Decisions 2, 3, 5, 8, 10 and deliverables D3, D6) |
```

BC-6.23.001 v1.2 §Traceability ADR Reference row:
```
| ADR Reference | ADR-025 v1.2 (Decisions 5 Path B, 6, 8 and deliverables D4, D5, D7, D8) |
```

Both carry the `ADR-025 v1.2` volatile version-pin pattern that is the POLICY 19 class confirmed actionable by F-P34-001 (BC-4.13.001) and closed by D-789. BC-5.40.001 (factory_lock STATE.md schema, S-17.01 lineage) and BC-6.23.001 (/factory-lock + /factory-unlock skills, S-17.03 lineage) are **out of the E-19 perimeter** (S-17.01/S-17.03 lineage; E-19 perimeter is S-19.01..S-19.07). These two BCs were authored at BC v1.0 citing ADR-025 at its v1.2 authoring-time version and have not been swept since.

**DISPOSITION:** record as STATE.md Drift Item — same POLICY 19 volatile-pin class as D-784 legacy S-17.02 item (BC-4.13.001 v1.2 cite, recorded at D-784). Target: next maintenance sweep alongside the existing D-784 item. Route product-owner at that sweep.

## Part B — Per-Policy Verification + Severity

### POLICY 19 — Volatile-pin sweep (ADR-025 version cites in BC §Traceability; E-19 perimeter)

Grep: `grep -nE 'ADR-025 v[0-9]+\.[0-9]+'` across all E-19 BCs in §Traceability ADR Reference cells (E-19 perimeter only):

| BC | Version reviewed | ADR Reference form | POLICY 19 status |
|----|-----------------|-------------------|-----------------|
| BC-2.07.001 | v1.4 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-5.42.001 | v1.5 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-3.08.001 | v1.19 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-1.17.001 | v1.5 | `ADR-025 §Decision N` (stable §Decision-enumerated) | ✓ PASS |
| BC-4.13.001 | v1.13 | `§Decision 1/14/15` (stable) + `§Decision 18` (mis-anchor) | ✗ F-P35-001 HIGH → CLOSED D-790 PO 97a1b9ed → v1.14 PASS |
| BC-2.02.011 | v1.5 | does not reference ADR-025 | n/a |

Post-fix trajectory: BC-4.13.001 v1.14 §Traceability ADR Reference row correctly reads `and Deliverable D18` (not `§Decision 18`); ADR-025 v1.12 has 15 Decisions, D18 is a Concrete Deliverables row ✓. All 6 E-19 perimeter BCs PASS on POLICY 19 post-fix. Out-of-perimeter BCs (BC-5.40.001 + BC-6.23.001) logged as O-P35-001 Drift Item.

### POLICY 4 — Anchor Verification Detail

ADR-025 v1.12 Decision existence grep (all §Decision anchors in BC-4.13.001 v1.13 §Traceability):
- `§Decision 1` → `109:### Decision 1` ✓ (POLICY 4 PASS)
- `§Decision 14` → `1096:### Decision 14` ✓ (POLICY 4 PASS)
- `§Decision 15` → `1132:### Decision 15` ✓ (POLICY 4 PASS)
- `§Decision 18` → NO MATCH in `^### Decision` grep; last Decision is 15 at line 1132 ✗ → F-P35-001 HIGH. D18 exists only as a Concrete Deliverables table row at line 1210.

Post-fix BC-4.13.001 v1.14: `and Deliverable D18` → Concrete Deliverables row at line 1210 ✓. All anchors now resolve. POLICY 4 CLEAN post-fix ✓.

### POLICY 1/6/7/8/9/13/14/15/17/18 — Clean passes

POLICY 1 (append-only IDs): no new BC/VP/story IDs introduced in this pass. ✓ CLEAN.
POLICY 6 (subsystem names): SS-04 references consistent with ARCH-INDEX v2.97 throughout. ✓ CLEAN.
POLICY 7 (BC-INDEX title verbatim parity): BC-INDEX v3.87 BC-4.13.001 H1 title unchanged. ✓ CLEAN.
POLICY 8 (BC frontmatter cycle propagation): BC-4.13.001 v1.14 cite sweep required in S-19.02/S-19.07 (fix-introduces-adjacent-defect: §Decision 18 text may have propagated into story cite sites). Story-writer leg S-19.02 v1.17 + S-19.07 v1.16 sweep performed ✓.
POLICY 9 (VP arithmetic reconciliation): VP-094..VP-101 counts verified across VP-INDEX v2.55 + verification-architecture.md + verification-coverage-matrix.md. ✓ CLEAN.
POLICY 13 (standing disciplinary constraints): all active standing controls carried ✓. ✓ CLEAN.
POLICY 14 (5-leg quintuple parity): applied to BC-4.13.001 v1.14 PO leg; input-hash restamps propagated to story-writer legs ✓. ✓ CLEAN.
POLICY 15 (BC lifecycle consistency): BC-4.13.001 lifecycle_status `active` unchanged (traceability/description-only amendment). ✓ CLEAN.
POLICY 17 (epic POLICY 17 EAC compliance): epic v1.22 EAC-001..EAC-005 verified ✓. ✓ CLEAN.
POLICY 18 (input-hash non-placeholder): BC-4.13.001 v1.14 input-hash 58518e8 ✓; S-19.02 input-hash 604f45d ✓; S-19.07 input-hash 534c85c ✓. All non-placeholder ✓.
POLICY 19 (volatile-pin): within E-19 perimeter, BC-4.13.001 v1.14 PASS post-fix; BC-5.40.001 + BC-6.23.001 out-of-perimeter per O-P35-001. ✓ CLEAN within perimeter.

### Severity + Trajectory

**Severity (B0/H1/M1/L0):** Two findings (F-P35-001 HIGH + F-P35-002 MEDIUM). Total: 2 items. Severity regression from pass-34 (1 total → 2 total). The HIGH finding (F-P35-001) is a fix-introduces-adjacent-defect escape class — the D-789 fix executor verified `§Decision 1` (the previously-flagged anchor) but did not existence-grep `§Decision 18` against the target ADR. F-P35-002 is a S-7.01 partial-fix regression: D-789 updated §Traceability correctly but left §Description stale, creating same-document contradiction.

**Novelty:** MEDIUM. F-P35-001 (fix-introduces-adjacent-defect: proposed-fix text carried an unverified anchor `§Decision 18` that resolves to nothing in the target ADR) is a new instance of a known class. The class is the converse of TD-VSDD-059 (paper-fix detection): here the fix was structurally correct but introduced an adjacent unverified anchor. The lesson codified by D-790: fix executors MUST existence-grep EVERY anchor in any replacement text against the target artifact at HEAD with captured stdout. F-P35-002 (same-document §Description / §Traceability contradiction) is a new instance of the S-7.01 partial-fix class: when a fix updates one section of a multi-section document, all related sections must be swept for stale co-references.

**Cascade trajectory (passes 22–35):** 4→3→4→2→2→4→6→5→4→1→3→4→1→2. Trajectory tail (passes 32–35): →3→4→1→2. Two-item pass. The asymptotic floor pattern continues in the [1,4] band; pass-35 is a regression from pass-34 (1 item), driven by a fix-introduces-adjacent-defect escape class not a novel behavioral gap.

## Fix Burst Closure (D-790)

**Fix burst D-790 applied.** Product-owner BC-4.13.001 v1.13→v1.14 (F-P35-001 + F-P35-002: §Traceability `§Decision 18` → `and Deliverable D18`; §Description Decisions 14/15 + D18 added; input-hash 86fab85→58518e8). Story-writer S-19.02 v1.16→v1.17 (BC-4.13.001 v1.13→v1.14 cite sweep ×18 sites; input-hash d208e66→604f45d). Story-writer S-19.07 v1.15→v1.16 (BC-4.13.001 v1.13→v1.14 cite sweep ×12 sites; input-hash 83e8cc4→534c85c). State-manager BC-INDEX v3.87→v3.88 (BC-4.13.001 row Version cell v1.14 + F-P35-001/002/D-790 change note). STORY-INDEX v4.166→v4.167 (S-19.02 row v1.17; S-19.07 row v1.16; BC coverage BC-4.13.001 v1.14; delivery-summary pass-35 note). VP-INDEX v2.55 UNCHANGED (exhaustive). ARCH-INDEX v2.97 UNCHANGED (exhaustive). STATE.md v5.40→v5.41 (SM: D-790 advance; trajectory →3→4→1→2; checkpoint refresh for pass-36). Commits: 97a1b9ed (PO BC-4.13.001 v1.14) + 42c0a7e9 (SW S-19.02 v1.17 + S-19.07 v1.16). O-P35-001: STATE.md Drift Items row added (BC-5.40.001 + BC-6.23.001 ADR-025 v1.2 volatile pins; target next maintenance sweep). Streak 0/3. NEXT: E-19 adv pass-36 (fresh context).
