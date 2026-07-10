# Adversarial Review — E-19 Pass 44 (rubric v1.4.3; streak 0/3)

## Header

| Field | Value |
|-------|-------|
| **Pass** | 44 |
| **Type** | Periodic adversarial review |
| **Verdict** | NOT-CLEAN B0/H0/M2/L1 |
| **Finding counts** | BLOCKER: 0; HIGH: 0; MEDIUM: 2; LOW: 1 |
| **Streak before** | 0/3 |
| **Streak after** | 0/3 (reset by 2 MEDIUM findings) |
| **Model family** | Claude Opus 4.7 |
| **Iron Law** | Fresh context; zero prior passes loaded; Part A of pass-43 only |
| **Rubric** | policies.yaml v1.4.3 |
| **Perimeter** | D-799 delta + full E-19 carry-forward: BC-3.08.001 v1.21 (6549a11); VP-098/100/101 v1.2; BC-INDEX v3.92; STORY-INDEX v4.172; VP-INDEX v2.57; S-19.05 v1.16; epic v1.25; policies.yaml v1.4.3 + D-798 carry-forward + full E-19 suite |
| **Date** | 2026-07-10 |

---

## Part A — Findings and Evidence

### A.1 — F-P44-001: MEDIUM — BC-INDEX v3.92 BC-3.08.001 v1.21 Catalog-Cell Four-Error Mis-Attribution (POLICY 4 / POLICY 14 leg-5)

**Severity:** MEDIUM
**Policy violated:** POLICY 4 (internal consistency — BC-INDEX version cell must faithfully describe the BC Changelog row for that version; four simultaneous errors cause the catalog cell to contradict BC-3.08.001 v1.21 Changelog SoT); POLICY 14 leg-5 (upstream-index parity — BC-INDEX version cell text must be derivable from and consistent with the BC body Changelog row)

**Evidence:**

BC-INDEX v3.92 BC-3.08.001 catalog row, v1.21 cell (current):
```
v1.21 (v1.21 2026-07-10 D-799: F-P43-004 body inline VP-100 §Property Statement PC cite fix per POLICY 5 v1.3.8 category-(j); architect 421a9e1f)
```

BC-3.08.001 v1.21 Changelog row (authoritative source — ground truth):
```
| v1.21 | 2026-07-10 | product-owner | F-P43-003: §VP VP-100 row verbatim-derived from VP-INDEX SoT (cardinality+mutual-exclusivity form; replaces latency-paraphrase). F-P43-005: v1.19 Changelog row backfilled; Amendment 2026-07-09 (v1.19→v1.20) prose section authored for structural parity. O-P43-001: last_amended canonicalized to chain form. |
```

**Four errors in the BC-INDEX v1.21 cell:**

1. **Wrong finding IDs:** Cell says `F-P43-004`. Authoritative source lists `F-P43-003 + F-P43-005 + O-P43-001`. F-P43-004 was the VP-098/101 PC cite fix (architect leg) — a completely different BC was amended for that finding (VP files, not BC-3.08.001).

2. **Wrong policy/content:** Cell says `body inline VP-100 §Property Statement PC cite fix per POLICY 5 v1.3.8 category-(j)`. BC-3.08.001 v1.21 contains no `§Property Statement` section (that is a VP-body section). The actual change was to the `§Verification Properties` table VP-100 row — a verbatim-derivation fix per POLICY 9, not a `§Property Statement` inline parenthetical cite fix per POLICY 5 v1.3.8.

3. **Wrong author:** Cell says `architect 421a9e1f`. Authoritative source: `product-owner ad464e09`. Commit `421a9e1f` is the architect leg that fixed VP-098/100/101 — not the product-owner leg that fixed BC-3.08.001.

4. **Wrong commit:** Cell says `421a9e1f` (architect). Correct commit is `ad464e09` (product-owner BC-3.08.001 amendment leg).

**Root cause:** The BC-INDEX v3.92 catalog cell for BC-3.08.001 v1.21 was authored during the D-799 fix burst (state-manager this-commit leg) without deriving the cell text from BC-3.08.001's own v1.21 Changelog row. Instead, the cell appears to have been written from the D-799 burst-level aggregate description (which emphasized F-P43-004 and the architect leg prominently). This is the **[process-gap]** class: `L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row` — on multi-artifact bursts, each upstream-index catalog cell MUST be sourced from THAT artifact's own Changelog row of the same version (verbatim-condensed), never from the burst-level aggregate description.

**Resolution:** CLOSED: state-manager this-commit — BC-INDEX v3.92→v3.93: BC-3.08.001 v1.21 catalog cell corrected to faithfully describe BC-3.08.001 Changelog row for v1.21 (F-P43-003+F-P43-005+O-P43-001; product-owner ad464e09; input-hash 6549a11 unchanged).

---

### A.2 — F-P44-002: MEDIUM — BC-INDEX v3.92 BC-3.08.001 v1.20 Catalog-Cell Date Wrong (POLICY 14 leg-5 / POLICY 4)

**Severity:** MEDIUM
**Policy violated:** POLICY 14 leg-5 (upstream-index parity — BC-INDEX version cell date must match BC body Changelog row date); POLICY 4 (internal consistency — date in catalog cell contradicts BC-3.08.001 v1.20 Changelog SoT)

**Evidence:**

BC-INDEX v3.92 BC-3.08.001 catalog row, v1.20 cell (current):
```
v1.20 (v1.20 2026-07-10 D-798: C-PP43-003 VP-100 row+anchor added per VP-INDEX SoT DI-019 drain-timer framing + C-PP43-004 §VP Anchors VP-028 bullet four→six event types per BC H1 catalog; 9253c492; input-hash 6549a11)
```

**Authoritative sources for BC-3.08.001 v1.20 date:**

1. **BC-3.08.001 v1.20 Changelog row (SoT):** `| v1.20 | 2026-07-09 | product-owner | ...`
2. **BC-3.08.001 frontmatter modified[]:** `- "2026-07-09 (v1.20)"`
3. **BC-3.08.001 Amendment header:** `## Amendment 2026-07-09 (v1.19 → v1.20 — D-798 pre-pass-43 consistency sweep)`
4. **D-798 decision-log dispatch date:** `2026-07-09`

All four authoritative sources agree: BC-3.08.001 v1.20 was authored on **2026-07-09**. The BC-INDEX cell claims `2026-07-10` — one day off.

**Root cause:** Same authoring session as F-P44-001 — the D-799 state-manager leg wrote the v1.20 cell date as 2026-07-10 (the D-799 burst date) rather than consulting BC-3.08.001's own Changelog row for v1.20 (which was authored 2026-07-09 during D-798). This is the same `[process-gap]` class as F-P44-001: catalog cells must be sourced from the artifact's own Changelog, not inferred from burst context.

**Resolution:** CLOSED: state-manager this-commit — BC-INDEX v3.93: BC-3.08.001 v1.20 catalog cell date corrected `2026-07-10` → `2026-07-09` per BC-3.08.001 Changelog SoT.

---

### A.3 — O-P44-001: LOW — BC-3.08.001 §Verification Properties VP-100 Row Sentence-Case vs Title-Case (POLICY 9 observation)

**Severity:** LOW (observation)
**Policy:** POLICY 9 (VP-INDEX is canonical SoT for VP property statements; verbatim-derivation obligation)

**Evidence:**

BC-3.08.001 v1.21 §Verification Properties VP-100 row (current post-fix):
```
Drain-Timer-Expiry Plugin.Abandoned Emission Guarantee — Mandatory Fields Cardinality and Mutual Exclusivity
```

VP-INDEX v2.57 canonical H1 for VP-100:
```
Drain-Timer-Expiry Plugin.Abandoned Emission Guarantee — Mandatory Fields Cardinality and Mutual Exclusivity
```

Observation: The BC row and VP-INDEX H1 text match. However, the specific casing is `Title-Case` in VP-INDEX; the BC row as authored by product-owner ad464e09 uses the same casing. This observation is raised for completeness — semantically equivalent, no behavioral ambiguity, no BC amendment warranted.

**Disposition:** ACCEPTED-WITH-RECORD. POLICY 9 does not mandate case-verbatim in BC §VP rows beyond the verbatim derivation already present. The VP-100 row in BC-3.08.001 v1.21 faithfully carries the full VP-INDEX H1 text including the two discriminating clauses ("Mandatory Fields Cardinality" and "Mutual Exclusivity"). Optional harmonization at next natural amendment if a reviewer finds value in normalization. No BC version bump warranted.

---

## Part B — Delta Verifications and Policy Attestations

### B.1 — D-799 Delta Verifications (a–j)

**B.1.a — BC-3.08.001 v1.21 §Verification Properties VP-100 row verbatim-derivation (F-P43-003 fix by PO ad464e09):**
PASS. BC-3.08.001 v1.21 §Verification Properties VP-100 row reads `Drain-Timer-Expiry Plugin.Abandoned Emission Guarantee — Mandatory Fields Cardinality and Mutual Exclusivity`. VP-INDEX v2.57 VP-100 Full Index row title column reads `Drain-Timer-Expiry Plugin.Abandoned Emission Guarantee — Mandatory Fields Cardinality and Mutual Exclusivity`. Verbatim match confirmed — both discriminating clauses ("Mandatory Fields Cardinality" and "Mutual Exclusivity") present.

**B.1.b — BC-3.08.001 v1.19 Changelog row backfill + v1.19→v1.20 Amendment prose section (F-P43-005 fix by PO ad464e09):**
PASS. BC-3.08.001 v1.21 Changelog table includes a v1.19 row between v1.18 and v1.20 rows. Amendment prose section `## Amendment 2026-07-09 (v1.19 → v1.20 — D-798 pre-pass-43 consistency sweep)` present at body end. Five-leg POLICY 14 parity holds for v1.19 and v1.20 (Changelog row + Amendment section + modified[] + last_amended chain + BC-INDEX row).

**B.1.c — BC-3.08.001 last_amended chain form (O-P43-001 fix by PO ad464e09):**
PASS. BC-3.08.001 v1.21 `last_amended:` carries chain form: `"2026-07-10 (v1.21) — ... [Prior: 2026-07-09 (v1.20) — ...]"`. Previous bare-date form replaced.

**B.1.d — VP-098 v1.2 §Property Statement PC cite corrections (F-P43-004 architect 421a9e1f; category-(j) audit sites 1–3):**
PASS. Category-(j) audit for VP-098: PS-B references `(PC2)` (was PC3 — corrected post-D-797 PC renumbering); PS-C references `(PC3)` (was PC4 — corrected); §Traceability PC4 cite dropped (was erroneous forward-reference). 3/3 VP-098 category-(j) sites verified correct.

**B.1.e — VP-100 v1.2 drain_window_ms + anchor referent corrections (O-P43-002 architect 421a9e1f; category-(j) audit sites 4–6):**
PASS. Category-(j) audit for VP-100: PS-C field enumeration includes `drain_window_ms` (7th field — was absent in v1.1); fixture field list includes `drain_window_ms`; anchor referent `Invariant 6 Field Table` replaced by `Event 5 mandatory-fields list` (referent now exists at correct location in BC-3.08.001 §Postconditions). 3/3 VP-100 category-(j) sites verified correct.

**B.1.f — VP-101 v1.2 §Property Statement PC cite corrections (F-P43-004 architect 421a9e1f; category-(j) audit sites 7–9):**
PASS. Category-(j) audit for VP-101: PS-B references `(PC3)` (was PC2 — corrected post-D-797 PC renumbering); PS-C references `(PC5)` (was PC3 — corrected). 2 direct VP-101 sites + 1 additional sibling site = 3/3 VP-101 category-(j) sites verified correct.

**Category-(j) audit summary: 9-of-9 sites across VP-098/100/101 all PASS. Zero residual PC cite mismatches in E-19 VP package.**

**B.1.g — STORY-INDEX v4.172 Epic E-19 H2 heading (F-P43-001 fix by SM this-commit; leading-cite audit site 1):**
PASS. STORY-INDEX v4.172 Epic E-19 H2 heading: `## Epic E-19 — Post-rc.22 Operator Hardening (v1.0-feature-engine-discipline-pass-1 / E-19 F3) — draft, v1.25`. Matches epic frontmatter version v1.25. Heading cite 1/7 verified.

**B.1.h — STORY-INDEX v4.172 S-19.03 leading cite (F-P43-002 fix by SM this-commit; leading-cite audit sites 2–7):**
PASS. S-19.03 row BCs column leading cite is now `story v1.18 — ...` (un-bracketed leading position). S-19.01 leading cite: `story v1.17` ✓; S-19.02 leading cite: `story v1.17` ✓; S-19.04 leading cite: `story v1.11` ✓; S-19.05 leading cite: `story v1.16` ✓; S-19.06 leading cite: `story v1.18` ✓; S-19.07 leading cite: `story v1.16` ✓. 7-of-7 STORY-INDEX leading cites match respective story frontmatter versions.

**7-of-7 leading-cite audit: all S-19.01–S-19.07 STORY-INDEX row leading cites match story frontmatter versions. PASS.**

**B.1.i — S-19.05 v1.16 cite sweep + epic v1.25 cite sweep (SW fbf344da):**
PASS. S-19.05 v1.16 carries BC-3.08.001 v1.21 references (×19 occurrences per D-799). Epic v1.25 carries BC-3.08.001 v1.21 cite sweep (×5 occurrences). Input-hash S-19.05 9e54d68 unchanged (cite-sweep-only per POLICY 18). Input-hash epic c3feb1c updated.

**B.1.j — policies.yaml v1.4.3 POLICY 5 v1.3.8 category-(j) codification (SM this-commit):**
PASS. policies.yaml v1.4.3 includes `category-(j): body §Property Statement / §Description / §Postcondition inline parenthetical PC cites at same-file semantic-parallel sites MUST be swept same-burst on any anchor migration or PC renumbering`. Codification matches D-799 finding disposition.

---

### B.2 — POLICY 1 (Append-only IDs)
PASS. No IDs removed. All 7 S-19 stories and E-19 epic remain registered. No BC, VP, story, or epic ID deletions observed across D-799 delta.

### B.3 — POLICY 2 (Version monotonicity)
PASS. All D-799 amendments advance version monotonically. BC-3.08.001 v1.20→v1.21; VP-098/100/101 v1.1→v1.2; S-19.05 v1.15→v1.16; epic v1.24→v1.25. No downgrade.

### B.4 — POLICY 3 (Authoritative source)
PASS. All anchor references in corrected artifacts resolve at HEAD. BC-3.08.001 v1.21 VP-100 §Verification Properties row verbatim from VP-INDEX v2.57. VP-098/100/101 PC cites verified against BC-3.08.001 §Postconditions (9-of-9 PASS per B.1.d–f).

### B.5 — POLICY 4 (Internal consistency)
FAIL on BC-INDEX v3.92 (F-P44-001 + F-P44-002). F-P44-001: BC-INDEX v1.21 cell content contradicts BC-3.08.001 Changelog row on 4 dimensions. F-P44-002: BC-INDEX v1.20 cell date contradicts BC-3.08.001 Changelog row date by 1 day. CLOSED: state-manager this-commit BC-INDEX v3.92→v3.93.

### B.6 — POLICY 5 v1.3.5 (Stable-anchor BC-version-pin)
PASS. No new volatile pins in D-799 delta. D-797 migration stable-anchor sweep holds across all VP source_bc / §Source Contract fields. D-795 ADR-025 stable anchor holds.

### B.7 — POLICY 5 v1.3.3 (Same-burst sibling sweep)
PASS. D-799 SW leg fbf344da swept all 19 S-19.05 cite sites and all 5 epic cite sites for BC-3.08.001 v1.20→v1.21 transition. No orphan cite sites found in D-799 perimeter.

### B.8 — POLICY 5 v1.3.7 category-(i) (Same-file aggregation cells)
PASS. Epic H2 heading (aggregation cell) updated same-burst as epic frontmatter bump (v1.25). S-19.03 STORY-INDEX leading cite reordered same-burst as story bump (v1.18). D-799 closed both outstanding F-P43-001/002 aggregation-cell gaps.

### B.9 — POLICY 5 v1.3.8 category-(j) (Inline parenthetical PC cites)
PASS. D-799 architect 421a9e1f corrected all 9 category-(j) sites across VP-098/100/101 (verified B.1.d–f). POLICY 5 v1.3.8 codified this cycle. Zero residual category-(j) PC cite mismatches in E-19 VP package.

### B.10 — POLICY 6 (Subsystem canonical names)
PASS. All subsystem references (SS-01/02/03/04/05/07) match ARCH-INDEX v2.98 canonical forms. No SS rename in D-799 delta.

### B.11 — POLICY 7 (BC-INDEX title-cell verbatim H1)
PASS (with MEDIUM finding on version-cell). No BC H1 changes in D-799. BC-3.08.001 title cell in BC-INDEX remains verbatim H1. F-P44-001 concerns the version annotation cell (last column), not the title cell.

### B.12 — POLICY 8 (BC-table propagation)
PASS. BC-3.08.001 v1.20→v1.21 propagated to S-19.05 (×19 cites) and epic (×5 cites) by SW fbf344da. VP-098/100/101 are VP-body-only fixes — no BC version bump required for VP §Property Statement PC cite corrections (VP references BC; BC §Traceability row is stable-anchor form and was not changed).

### B.13 — POLICY 9 (VP-INDEX propagation completeness)
PASS. BC-3.08.001 v1.21 §Verification Properties VP-100 row now verbatim-derived from VP-INDEX SoT. O-P44-001 (case observation) accepted-with-record. VP-INDEX v2.57 unchanged — all VP statements remain canonical.

### B.14 — POLICY 14 (5-leg quintuple parity on index bumps)
FAIL on BC-INDEX v3.92 (F-P44-001 + F-P44-002). Leg-5 (upstream-index version cell) failed for BC-3.08.001 v1.21 (4-error mis-attribution) and v1.20 (date wrong). CLOSED: state-manager this-commit BC-INDEX v3.93 (version + last_amended + row cells corrected).

### B.15 — POLICY 15 (Traceability completeness)
PASS. VP-098/100/101 §Traceability sections verified at D-797/D-799. BC-3.08.001 §Traceability unchanged (no new VPs added in D-799 that would require §Traceability update).

### B.16 — POLICY 16 (Decision-log global-max gate)
PASS. D-800 allocated this pass. Sequential from D-799 (confirmed max via grep of decision-log.md tail: last entry heading `D-799-PASS-43-FIX-BURST-CLOSED`).

### B.17 — POLICY 17 (Epic frontmatter completeness)
PASS. Epic v1.25 frontmatter complete. O-P43-001 (BC-3.08.001 bare-date last_amended) closed at D-799 by PO ad464e09 — chain form confirmed in B.1.c.

### B.18 — POLICY 18 (Input-hash completeness)
PASS. BC-3.08.001 input-hash 6549a11 unchanged (PO ad464e09 amendment preserved hash). VP-100 input-hash a2de4e4→6565e01 (architect 421a9e1f body content change). S-19.05 input-hash 9e54d68 unchanged (cite-sweep-only). Epic input-hash 7ec7e1d→c3feb1c (SW fbf344da content change).

### B.19 — POLICY 19 (ADR body stable-anchor form)
PASS. ADR-025 v1.13 and ADR-030 v1.3 carry zero live volatile pins in normative Decision body sections. No ADR changes in D-799 delta.

### B.20 — ADR body BC-cite sweep (D-795 enforcement gate)
PASS. ADR-025/ADR-030 normative sections: zero `BC-N.NN.NNN v[0-9]` volatile pins in non-amendment_reason/non-Changelog rows. D-799 delta introduced no new ADR edits requiring sweep.

### B.21 — D-449(a) literal-shell gate obligation
PASS for D-799 fix bursts per burst-log D-799 Dim-2 gates (i–vi captured stdout). For D-800 closure burst (this-commit): 4-index gate and own-burst-log 8-block gate will be captured in burst-log D-800 Dim-2 per D-449(a).

---

## Trajectory and Novelty Note

**Convergence trajectory (passes 22–44):** 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2

Pass-44 decreased from pass-43's 5 to 2 substantive findings (M2/L1), both in BC-INDEX only. All D-799 content fixes verified correct (9-of-9 category-(j) audit PASS; 7-of-7 leading-cite audit PASS; VP-100 7-field enumeration PASS; BC-3.08.001 v1.19 backfill + chain-form last_amended PASS; STORY-INDEX heading/leading-cite fixes PASS).

**[process-gap] class codified this pass — `L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row`:**

F-P44-001 + F-P44-002 originate from the same authoring session: the D-799 state-manager leg wrote the BC-INDEX BC-3.08.001 v1.21 and v1.20 cells using the burst-level aggregate description context rather than consulting each artifact's own Changelog row. This produced a v1.21 cell that described the architect leg (which did NOT touch BC-3.08.001) and a v1.20 cell date derived from the burst date (2026-07-10) rather than the artifact-authoring date (2026-07-09).

**The process-gap:** On multi-artifact fix bursts, the BC-INDEX (and all upstream-index catalog cells) MUST be derived from the target artifact's own Changelog row for that version, NOT from the burst-level aggregate description. The correct workflow is:
1. For each BC bump: `grep -A2 'v1.21\|v1.20' .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md | grep -E "^\| v1\.(21|20)"` — capture stdout
2. Derive the BC-INDEX version cell text from that row (verbatim-condensed)
3. Never substitute burst-level narrative for per-artifact Changelog evidence

This is a state-manager discipline issue, not a content specialist issue. It applies whenever SM writes version cells in any upstream index (BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX).

**Zero-HIGH fourth consecutive pass.** Pass-44 finding severity: M2/L1. The asymptotic floor continues to show structural bias toward BC-INDEX catalog-cell accuracy on multi-artifact bursts. The new L-BB process-gap discipline directly addresses the root cause.

**Streak status:** 0/3. Pass-45 dispatch with full v1.4.3 rubric + L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row enforcement.

---

## CLOSED Annotations

| Finding | Status | Commit |
|---------|--------|--------|
| F-P44-001 MEDIUM: BC-INDEX v3.92 BC-3.08.001 v1.21 catalog-cell 4-error mis-attribution (POLICY 4/14-leg-5) | CLOSED | state-manager this-commit (BC-INDEX v3.92→v3.93: v1.21 cell corrected per BC-3.08.001 Changelog SoT) |
| F-P44-002 MEDIUM: BC-INDEX v3.92 BC-3.08.001 v1.20 catalog-cell date `2026-07-10`→`2026-07-09` (POLICY 14-leg-5/4) | CLOSED | state-manager this-commit (BC-INDEX v3.93: v1.20 date corrected per BC-3.08.001 Changelog SoT) |
| O-P44-001 LOW: BC-3.08.001 VP-100 row sentence-case vs Title-Case | ACCEPTED-WITH-RECORD | No action; optional harmonization at next natural amendment |
| Streak | 0/3 UNCHANGED | M2 findings prevent advancement |
