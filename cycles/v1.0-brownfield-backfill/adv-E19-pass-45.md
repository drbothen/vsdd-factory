# Adversarial Review — E-19 Pass 45 (rubric v1.4.3; streak 0/3)

## Header

| Field | Value |
|-------|-------|
| **Pass** | 45 |
| **Type** | Periodic adversarial review |
| **Verdict** | NOT-CLEAN B0/H0/M3/L1 |
| **Finding counts** | BLOCKER: 0; HIGH: 0; MEDIUM: 3; LOW: 1 |
| **Streak before** | 0/3 |
| **Streak after** | 0/3 (reset by 3 MEDIUM findings) |
| **Model family** | Claude Opus 4.7 |
| **Iron Law** | Fresh context; zero prior passes loaded; Part A of pass-44 only |
| **Rubric** | policies.yaml v1.4.3 + L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row (D-800 codification) |
| **Perimeter** | D-800 delta + full E-19 carry-forward: BC-INDEX v3.93; VP-INDEX v2.57; STORY-INDEX v4.172; BC-2.02.011 v1.6 (e650b4b); BC-5.42.001 v1.6 (4fd18a4); policies.yaml v1.4.3 + D-800 carry-forward + full E-19 suite |
| **Date** | 2026-07-10 |

---

## Part A — Findings and Evidence

### A.1 — F-P45-001: MEDIUM — BC-INDEX v3.93 BC-2.02.011 v1.6 Catalog-Cell Date Wrong (POLICY 4 / POLICY 14 leg-5)

**Severity:** MEDIUM
**Policy violated:** POLICY 4 (internal consistency — BC-INDEX version cell date must faithfully match the BC Changelog row for that version); POLICY 14 leg-5 (upstream-index parity — BC-INDEX version cell text must be derivable from and consistent with the BC body Changelog row)

**Evidence:**

BC-INDEX v3.93 BC-2.02.011 catalog row, v1.6 cell (pre-fix, as reviewed):
```
v1.6 (v1.6 2026-07-10 D-798: C-PP43-002 VP-097 row+co-anchor added per VP-INDEX SoT traversal-defense framing; 9253c492; input-hash e650b4b)
```

BC-2.02.011 v1.6 Changelog row (authoritative source — ground truth):
```
- v1.6 (2026-07-09): orchestrator pre-pass-43 consistency sweep — §Verification Properties VP-097 row added...
```

BC-2.02.011 frontmatter modified[] v1.6 entry (authoritative source):
```
- "v1.6 (2026-07-09): orchestrator pre-pass-43 consistency sweep — ..."
```

Both authoritative sources agree: BC-2.02.011 v1.6 was authored on **2026-07-09**. The BC-INDEX cell claims `2026-07-10` — one day off.

**Root cause:** D-798 SM leg authored the BC-INDEX BC-2.02.011 v1.6 catalog cell using the burst date (2026-07-10, the date D-798 ran) rather than consulting BC-2.02.011's own Changelog row for v1.6 (which was authored 2026-07-09). This is the same `[process-gap]` class codified at D-800: `L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row`. The D-800 codification was prompted by BC-3.08.001 cells (F-P44-001/002); the L-BB sweep of D-798 carry-forward catalog cells now surfaces the same defect for BC-2.02.011 v1.6.

**Resolution:** CLOSED: state-manager this-commit — BC-INDEX v3.93→v3.94: BC-2.02.011 v1.6 catalog cell date corrected `2026-07-10` → `2026-07-09` per BC-2.02.011 Changelog SoT.

---

### A.2 — F-P45-002: MEDIUM — BC-INDEX v3.93 BC-5.42.001 v1.6 Catalog-Cell Date Wrong (POLICY 4 / POLICY 14 leg-5)

**Severity:** MEDIUM
**Policy violated:** POLICY 4 (internal consistency — BC-INDEX version cell date must faithfully match the BC Changelog row for that version); POLICY 14 leg-5 (upstream-index parity — BC-INDEX version cell text must be derivable from and consistent with the BC body Changelog row)

**Evidence:**

BC-INDEX v3.93 BC-5.42.001 catalog row, v1.6 cell (pre-fix, as reviewed):
```
v1.6 (v1.6 2026-07-10 D-798: C-PP43-001 VP-094 row-3 proof-method unit→integration per VP-INDEX SoT; 9253c492; input-hash 4fd18a4 UNCHANGED)
```

BC-5.42.001 v1.6 Changelog row (authoritative source — ground truth):
```
| 1.6 | 2026-07-09 | product-owner | orchestrator pre-pass-43 consistency sweep: §Verification Properties VP-094 row-3 proof-method drift fixed — `unit (WASM test harness; S-19.01)` → `integration (bats; S-19.01)` per VP-INDEX v2.56 canonical proof method.
```

BC-5.42.001 frontmatter modified[] v1.6 entry (authoritative source):
```
- "2026-07-09 (v1.6)"
```

BC-5.42.001 last_amended (authoritative source): `(v1.6) — orchestrator pre-pass-43 consistency sweep (product-owner): §Verification Properties VP-094 row-3 proof-method drift fixed...`

All four authoritative sources agree: BC-5.42.001 v1.6 was authored on **2026-07-09**. The BC-INDEX cell claims `2026-07-10` — one day off.

**Root cause:** Same authoring session as F-P45-001 — the D-798 SM leg wrote both BC-2.02.011 v1.6 and BC-5.42.001 v1.6 catalog cells with the burst date (2026-07-10) rather than each artifact's own Changelog date (2026-07-09). The L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row process-gap applies equally here.

**Resolution:** CLOSED: state-manager this-commit — BC-INDEX v3.94: BC-5.42.001 v1.6 catalog cell date corrected `2026-07-10` → `2026-07-09` per BC-5.42.001 Changelog SoT.

---

### A.3 — F-P45-003: MEDIUM — BC-2.02.011 Frontmatter modified[] Array Not Version-Monotonic (POLICY 14 leg-3)

**Severity:** MEDIUM
**Policy violated:** POLICY 14 leg-3 (modified[] array MUST be version-monotonic — entries ordered v1.N ascending; last entry must match Changelog top version)

**Evidence:**

BC-2.02.011 frontmatter modified[] array (pre-fix, as reviewed):
```yaml
modified:
  - "v1.3 (2026-06-20): S-18.04a-prereq / ADR-028 §Decision 8 — ..."
  - "v1.4 (2026-07-06): E-19 pass-2 F-P2-004 fix burst — ..."
  - "v1.6 (2026-07-09): orchestrator pre-pass-43 consistency sweep — ..."
  - "v1.5 (2026-07-08): E-19 pass-24 F-P24-001 fix burst — ..."
```

The modified[] array lists entries in order: v1.3 → v1.4 → v1.6 → v1.5 (non-monotonic: v1.6 appears before v1.5). This violates POLICY 14 leg-3, which requires monotonically ascending version order. The last entry `v1.5` also contradicts the Changelog top entry `v1.6`, violating the additional POLICY 14 leg-3 constraint that the last modified[] entry must match the Changelog top version.

**Root cause:** The D-798 PO leg added the v1.6 entry to modified[] by appending at the WRONG position — after v1.4 instead of at the end after v1.5. This placed v1.6 before the existing v1.5 entry. The carry-forward from D-798 through D-799 and D-800 did not catch this ordering defect.

**Resolution:** CLOSED: product-owner 6f813e9e — BC-2.02.011 v1.6→v1.7: frontmatter modified[] array re-sorted version-monotonic (v1.3→v1.4→v1.5→v1.6→v1.7); no body content change.

---

### A.4 — O-P45-001: LOW — VP-INDEX v2.57 VP-100 v1.2 Cell Finding-ID Over-Attribution (POLICY 9 observation)

**Severity:** LOW (observation)
**Policy:** POLICY 9 (VP-INDEX is canonical SoT for VP property statements; version cell annotations must faithfully reflect the artifact's own last_amended SoT)

**Evidence:**

VP-INDEX v2.57 VP-100 v1.2 catalog row annotation (pre-fix, as reviewed):
```
D-799: F-P43-004+O-P43-002 drain_window_ms
```

VP-100 last_amended SoT (authoritative source from VP-100.md):
```
"2026-07-10 (v1.2) — E-19 pass-43 fix burst (architect): O-P43-002: §Property Statement PC-C field list incomplete..."
```

VP-100's own last_amended cites only `O-P43-002` — `F-P43-004` is absent.

**Adjudication:** F-P43-004 covered §Property Statement inline parenthetical PC-cite corrections across VP-098, VP-100, and VP-101. However, on review of the actual VP-100 v1.2 amendment content:

- VP-100's v1.2 changes per O-P43-002: `drain_window_ms` added to PC-C field list; fixture field list updated; anchor referent corrected to `Event 5 mandatory-fields list`
- F-P43-004 for VP-100: the pass-43 adversary report would attribute the VP-098 PS-B PC3→PC2 and PS-C PC4→PC3 corrections to F-P43-004, and VP-101 PS-B PC2→PC3 and PS-C PC3→PC5 corrections to F-P43-004. VP-100's §Property Statement inline PC-cite corrections were limited to those flowing from O-P43-002 (anchor referent correction).

Per VP-100.md last_amended SoT, the v1.2 amendment is attributed exclusively to O-P43-002. F-P43-004 was VP-098a/VP-101b PC-cite scope only; VP-100's contribution in D-799 was entirely from O-P43-002. The VP-INDEX v2.57 VP-100 v1.2 cell over-attributed by including F-P43-004.

**Disposition:** CLOSED: state-manager this-commit — VP-INDEX v2.57→v2.58: VP-100 v1.2 cell corrected `D-799: F-P43-004+O-P43-002 drain_window_ms` → `D-799: O-P43-002 drain_window_ms` per VP-100.md last_amended SoT.

---

## Part B — Delta Verifications and Policy Attestations

### B.1 — D-800 Delta Verifications (a–g)

**B.1.a — BC-3.08.001 v1.21 catalog cell corrected (F-P44-001 fix by SM D-800):**
PASS. BC-INDEX v3.93 BC-3.08.001 v1.21 cell reads: `v1.21 (v1.21 2026-07-10 D-799: F-P43-003+F-P43-005+O-P43-001 §VP VP-100 row verbatim-derived per POLICY 9 + v1.19 Changelog row backfill + last_amended chain form; product-owner ad464e09; input-hash 6549a11 unchanged)`. Cell faithfully describes BC-3.08.001 v1.21 Changelog SoT (F-P43-003+F-P43-005+O-P43-001; product-owner ad464e09). F-P44-001 closed.

**B.1.b — BC-3.08.001 v1.20 catalog cell date corrected (F-P44-002 fix by SM D-800):**
PASS. BC-INDEX v3.93 BC-3.08.001 v1.20 cell date reads `2026-07-09`. BC-3.08.001 Changelog row v1.20: `| v1.20 | 2026-07-09 | product-owner |`. Date matches. F-P44-002 closed.

**B.1.c — L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row codification (D-800 SM leg):**
PASS. lessons.md contains `L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row [process-gap][codified D-800]`. Decision-log D-800 documents the lesson codification and the process-gap class. Codification is complete and appropriately scoped.

**B.1.d — BC-2.02.011 v1.6 cell (D-798 carry-forward — L-BB sweep):**
FAIL → F-P45-001. BC-INDEX v3.93 BC-2.02.011 v1.6 cell date `2026-07-10` contradicts BC-2.02.011 Changelog row v1.6 date `2026-07-09`. L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row enforcement: this cell was authored during D-798 SM leg without consulting BC-2.02.011's own Changelog SoT. Same defect class as F-P44-001/F-P44-002 in prior pass.

**B.1.e — BC-5.42.001 v1.6 cell (D-798 carry-forward — L-BB sweep):**
FAIL → F-P45-002. BC-INDEX v3.93 BC-5.42.001 v1.6 cell date `2026-07-10` contradicts BC-5.42.001 Changelog row v1.6 date `2026-07-09`. Same authoring session as F-P45-001. L-BB sweep confirms same root cause.

**B.1.f — BC-2.02.011 modified[] ordering audit (carry-forward):**
FAIL → F-P45-003. BC-2.02.011 frontmatter modified[] entries ordered v1.3→v1.4→v1.6→v1.5 (not monotonic). D-798 PO leg appended v1.6 entry after v1.4 instead of at list end. POLICY 14 leg-3 violated.

**B.1.g — VP-INDEX v2.57 VP-100 v1.2 cell finding-ID audit (carry-forward):**
OBSERVATION → O-P45-001. VP-INDEX v2.57 VP-100 v1.2 cell cites `F-P43-004+O-P43-002`; VP-100.md last_amended SoT cites only `O-P43-002`. Over-attribution — see A.4.

---

### B.2 — POLICY 1 (Append-only IDs)
PASS. No IDs removed. All 7 S-19 stories and E-19 epic remain registered. No BC, VP, story, or epic ID deletions observed across D-800 delta.

### B.3 — POLICY 2 (Version monotonicity)
PASS for D-800 delta. BC-INDEX v3.92→v3.93 is monotonic. BC-3.08.001 Changelog row v1.20 and v1.21 dates were corrected (content fix, not version regression). F-P45-003 (BC-2.02.011 modified[] non-monotonic ordering) is carry-forward from D-798, not D-800 regression.

### B.4 — POLICY 3 (Authoritative source)
PASS for D-800 delta. BC-3.08.001 v1.21 BC-INDEX cell now faithfully derived from BC-3.08.001 Changelog SoT. All anchor references in D-800-touched artifacts resolve at HEAD.

### B.5 — POLICY 4 (Internal consistency)
FAIL on BC-INDEX v3.93 (F-P45-001 + F-P45-002). F-P45-001: BC-INDEX v1.6 cell for BC-2.02.011 contradicts BC-2.02.011 Changelog row on date. F-P45-002: BC-INDEX v1.6 cell for BC-5.42.001 contradicts BC-5.42.001 Changelog row on date. CLOSED: state-manager this-commit BC-INDEX v3.93→v3.94. FAIL on BC-2.02.011 (F-P45-003): modified[] ordering contradicts version-monotonicity invariant. CLOSED: product-owner 6f813e9e BC-2.02.011 v1.6→v1.7.

### B.6 — POLICY 5 v1.3.5 (Stable-anchor BC-version-pin)
PASS. No new volatile pins in D-800 delta. D-800 touched only BC-INDEX catalog cell text (annotation correction); no BC body stable-anchor fields changed.

### B.7 — POLICY 5 v1.3.3 (Same-burst sibling sweep)
PASS. D-800 SM leg corrected BC-3.08.001 v1.21 and v1.20 cells — carry-forward corrections only; no new BC version bumps triggered same-burst sibling-sweep obligations. F-P45-001/002 were pre-existing in BC-INDEX from D-798 SM authoring.

### B.8 — POLICY 5 v1.3.7 category-(i) (Same-file aggregation cells)
PASS. No aggregation cell updates required in D-800 delta. BC-2.02.011 v1.7 citation propagation (S-19.03 cite sweep) is SW ae37b246 within D-801 burst — within-burst same-burst obligation honored.

### B.9 — POLICY 5 v1.3.8 category-(j) (Inline parenthetical PC cites)
PASS. No category-(j) site changes in D-800 delta. VP-098/100/101 v1.2 category-(j) sites verified PASS at D-799 (B.1.d–f of pass-44 report). No regressions in D-800 delta.

### B.10 — POLICY 6 (Subsystem canonical names)
PASS. All subsystem references (SS-01/02/03/04/05/07) match ARCH-INDEX v2.98 canonical forms. No SS rename in D-800 delta.

### B.11 — POLICY 7 (BC-INDEX title-cell verbatim H1)
PASS. No BC H1 changes in D-800 delta. BC-3.08.001 title cell in BC-INDEX remains verbatim H1. BC-2.02.011 title cell unchanged. BC-5.42.001 title cell unchanged. F-P45-001/002 concern the version annotation date (last column), not the title cell.

### B.12 — POLICY 8 (BC-table propagation)
PASS. BC-2.02.011 v1.7 (D-801 PO leg 6f813e9e) — no body content change, modified[] re-sort only. No VP cite changes triggered. No BC §Traceability row change required. S-19.03 cite-sweep (ae37b246) propagates v1.6→v1.7 reference within D-801 burst. BC-5.42.001 v1.6 date correction is annotation-only; no BC body change; no cascade propagation required.

### B.13 — POLICY 9 (VP-INDEX propagation completeness)
PASS for D-800 delta. No VP version changes in D-800. VP-INDEX v2.57 unchanged by D-800. O-P45-001 (VP-100 v1.2 cell finding-ID over-attribution) is annotation correction — no VP statement change. CLOSED: state-manager this-commit VP-INDEX v2.57→v2.58 (description-only annotation correction; no VP statement change; POLICY 9 propagation not triggered).

### B.14 — POLICY 14 (5-leg quintuple parity on index bumps)
FAIL on BC-2.02.011 modified[] (F-P45-003): leg-3 (modified[] array version-monotonic) violated. CLOSED: product-owner 6f813e9e BC-2.02.011 v1.7 (re-sort only). FAIL on BC-INDEX v3.93 (F-P45-001 + F-P45-002): leg-5 (upstream-index version cell) dates wrong for BC-2.02.011 v1.6 and BC-5.42.001 v1.6. CLOSED: state-manager this-commit BC-INDEX v3.94.

### B.15 — POLICY 15 (Traceability completeness)
PASS. VP-100 §Traceability section verified at D-799 (no new VPs in D-800 delta). BC-5.42.001 §Traceability unchanged. BC-2.02.011 §Traceability unchanged (v1.7 modified[] re-sort is metadata-only, no body content change).

### B.16 — POLICY 16 (Decision-log global-max gate)
PASS. D-801 allocated this pass. Sequential from D-800 (confirmed max via grep of decision-log.md tail: last entry heading `D-800-PASS-44-FIX-BURST-CLOSED`). D-801 is correct next allocation.

### B.17 — POLICY 17 (Epic frontmatter completeness)
PASS. Epic v1.25 frontmatter complete (verified D-799). No epic changes in D-800 delta.

### B.18 — POLICY 18 (Input-hash completeness)
PASS. BC-2.02.011 input-hash e650b4b: v1.7 modified[] re-sort only (frontmatter-only metadata change — no body content change); input-hash UNCHANGED per PO 6f813e9e attestation. BC-5.42.001 input-hash 4fd18a4 UNCHANGED (v1.6 date correction was BC-INDEX annotation correction only; BC-5.42.001 body unchanged). S-19.03 input-hash 8d1225d UNCHANGED (cite-sweep-only per SW ae37b246).

### B.19 — POLICY 19 (ADR body stable-anchor form)
PASS. ADR-025 v1.13 and ADR-030 v1.3 carry zero live volatile pins in normative Decision body sections. No ADR changes in D-800 delta.

### B.20 — ADR body BC-cite sweep (D-795 enforcement gate)
PASS. ADR-025/ADR-030 normative sections: zero `BC-N.NN.NNN v[0-9]` volatile pins in non-amendment_reason/non-Changelog rows. D-800 delta introduced no new ADR edits requiring sweep.

### B.21 — D-449(a) literal-shell gate obligation
PASS for D-800 fix burst per burst-log D-800 Dim-2 gates (i–iii captured stdout). For D-801 closure burst (this-commit): 4-index gate and own-burst-log 8-block gate will be captured in burst-log D-801 Dim-2 per D-449(a).

---

## Trajectory and Novelty Note

**Convergence trajectory (passes 22–45):** 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2→3

Pass-45 increased from pass-44's 2 to 3 MEDIUM findings. All findings are in the same `[process-gap]` class: `L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row`. The D-800 codification of the L-BB lesson triggered a retroactive sweep that found two more instances of the same defect (BC-2.02.011 and BC-5.42.001 v1.6 cells authored during D-798 SM leg with burst date rather than Changelog date). The D-800 SM leg itself correctly derived BC-3.08.001 cells per L-BB discipline — the corrective discipline functioned as designed but exposed pre-existing D-798 residue.

**Second L-BB enforcement refinement codified this pass — `L-BB-remediation-predicate-must-enumerate-all-same-burst-touched-artifacts`:**

F-P45-001 + F-P45-002 originate from D-798: the SM leg that amended BC-2.02.011 v1.5→v1.6 and BC-5.42.001 v1.5→v1.6 (C-PP43-002 and C-PP43-001 sweep) wrote catalog cells using the burst date. The D-800 remediation closed BC-3.08.001 cells but did NOT sweep ALL same-burst-touched cells (BC-2.02.011 and BC-5.42.001 were also touched in D-798 SM leg). The L-BB predicate must enumerate ALL rows in ALL indexes the burst touched.

**The process-gap extension:** On any fix-burst that corrects an index-cell defect under the L-BB predicate, the SM must also sweep ALL other catalog cells authored in the SAME burst (same `git -C .factory log --oneline` SHA range) for the same class of defect. The D-800 SM leg fixed BC-3.08.001 v1.20/v1.21 but did not extend the sweep to BC-2.02.011 v1.6 and BC-5.42.001 v1.6 cells authored in the same D-798 SM leg (commit 9253c492).

This is a state-manager discipline issue. It applies whenever SM closes an L-BB finding: the remediation sweep predicate MUST be: "ALL catalog cells authored in the same burst as the defective cell, not just the reported defective cell."

**Zero-HIGH fifth consecutive pass.** Pass-45 finding severity: M3/L1. The asymptotic floor continues to show structural bias toward BC-INDEX catalog-cell accuracy on multi-artifact bursts, now refined by the sweep-predicate gap. The new L-BB-remediation-predicate extension directly addresses the remaining root cause.

**Streak status:** 0/3. Pass-46 dispatch with full v1.4.3 rubric + L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row + L-BB-remediation-predicate-must-enumerate-all-same-burst-touched-artifacts enforcement.

---

## CLOSED Annotations

| Finding | Status | Commit |
|---------|--------|--------|
| F-P45-001 | CLOSED | state-manager this-commit (BC-INDEX v3.94) |
| F-P45-002 | CLOSED | state-manager this-commit (BC-INDEX v3.94) |
| F-P45-003 | CLOSED | product-owner 6f813e9e (BC-2.02.011 v1.7) |
| O-P45-001 | CLOSED | state-manager this-commit (VP-INDEX v2.58) |
