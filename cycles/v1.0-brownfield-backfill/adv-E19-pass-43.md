# Adversarial Review — E-19 Pass 43 (rubric v1.4.2; streak 0/3)

## Header

| Field | Value |
|-------|-------|
| **Pass** | 43 |
| **Type** | Periodic adversarial review |
| **Verdict** | NOT-CLEAN B0/H2/M3/L2 |
| **Finding counts** | BLOCKER: 0; HIGH: 2; MEDIUM: 3; LOW: 2 |
| **Streak before** | 0/3 |
| **Streak after** | 0/3 (reset by 2 HIGH + 3 MEDIUM findings) |
| **Model family** | Claude Opus 4.7 |
| **Iron Law** | Fresh context; zero prior passes loaded; Part A of pass-42 only |
| **Rubric** | policies.yaml v1.4.2 |
| **Perimeter** | Full E-19 suite at D-798 versions: BC-3.08.001 v1.20; VP-098/101 v1.1; VP-100 v1.1; STORY-INDEX v4.171 (epic heading v1.22; S-19.03 leading v1.16) |
| **Date** | 2026-07-10 |

---

## Part A — Findings and Evidence

### A.1 — F-P43-001: HIGH — STORY-INDEX Epic E-19 H2 Heading Stale Version (POLICY 14 legs 5 / 17 / POLICY 5 v1.3.7 category-(i))

**Severity:** HIGH
**Policy violated:** POLICY 14 leg-5 (upstream-index parity: STORY-INDEX epic H2 heading must reflect current epic frontmatter version); POLICY 17 (epic frontmatter completeness — heading version is an observable integrity field); POLICY 5 v1.3.7 category-(i) (same-file aggregation cells must be swept same-burst as per-row version bumps)

**Evidence:**

STORY-INDEX v4.171 Epic E-19 H2 heading:
```
## Epic E-19 — Post-rc.22 Operator Hardening (v1.0-feature-engine-discipline-pass-1 / E-19 F3) — draft, v1.22
```

Epic file (`epic-e19-post-rc22-operator-hardening.yaml` or equivalent frontmatter) current version: **v1.25** (SW leg fbf344da advanced v1.24→v1.25; pre-pass-43 D-798 advanced v1.23→v1.24).

Discrepancy: heading claims `v1.22` but epic frontmatter is `v1.25`. Three version bumps (v1.22→v1.23 pass-42, v1.23→v1.24 D-798, v1.24→v1.25 SW fbf344da) all failed to propagate the H2 heading cite.

**Process gap:** The D-797 fix burst (pass-42) bumped the epic from v1.22→v1.23 (cite sweep at EAC-003). The D-798 pre-pass-43 fix burst bumped it to v1.24. The pass-43 SW leg (fbf344da) bumped it to v1.25. None of these bursts updated the STORY-INDEX H2 heading. This is a **[process-gap]** class — state-manager MUST update the H2 heading whenever an epic version is bumped.

**Resolution:** CLOSED: this-commit (state-manager) — STORY-INDEX v4.171→v4.172: Epic E-19 H2 heading updated `draft, v1.22` → `draft, v1.25`.

---

### A.2 — F-P43-002: HIGH — STORY-INDEX S-19.03 Leading Cite Stale (POLICY 14 leg-5 / POLICY 5 v1.3.3)

**Severity:** HIGH
**Policy violated:** POLICY 14 leg-5 (upstream-index parity: STORY-INDEX leading story version cite must match story file frontmatter version); POLICY 5 v1.3.3 (sibling-sweep — when a story is bumped, ALL its index row cites must be swept same-burst, including the leading cite position)

**Evidence:**

STORY-INDEX v4.171 S-19.03 BCs column leading cite:
```
story v1.16 — BC-2.07.001 v1.3→v1.4 cite sweep ×3 sites (hash unchanged). [story v1.18 — D-798 pre-pass-43 consistency sweep: ...] [story v1.17 — F-P42-002/003: ...] [prior story v1.14 — ...
```

S-19.03 file frontmatter: `version: "1.18"` (D-798 pre-pass-43 consistency fix brought S-19.03 from v1.17→v1.18).

Discrepancy: leading cite is `story v1.16` but the story file is at v1.18. The D-798 pre-pass burst appended `[story v1.18 — ...]` as a non-leading bracket before the still-stale `story v1.16` leading position. POLICY 14 leg-5 requires the leading cite to match the current frontmatter version.

**Process gap:** When D-798 bumped S-19.03 from v1.17→v1.18, it correctly added a `[story v1.18 — ...]` annotation but failed to **promote** v1.18 to the leading position and demote v1.16/v1.17 to `[prior ...]` brackets. This is the same **[process-gap]** class as F-P43-001 — state-manager MUST reorder index row cites to keep the leading position at the current frontmatter version.

**Resolution:** CLOSED: this-commit (state-manager) — STORY-INDEX v4.172: S-19.03 row reordered — `story v1.18 — ...` promoted to leading; `story v1.17 — ...` moved to `[prior story v1.17 —]`; `story v1.16 — ...` moved to `[prior story v1.16 —]`.

---

### A.3 — F-P43-003: MEDIUM — BC-3.08.001 §VP Properties VP-100 Row Paraphrase Dropped Discriminating Clauses (POLICY 9 / POLICY 4)

**Severity:** MEDIUM
**Policy violated:** POLICY 9 (VP-INDEX is canonical source for VP property statements; BC §Verification Properties rows must be verbatim-derived from VP-INDEX SoT, not paraphrased); POLICY 4 (internal inconsistency — §VP Anchors section had correct reference but §Verification Properties row body contradicted VP-INDEX canonical text)

**Evidence:**

BC-3.08.001 v1.20 §Verification Properties VP-100 row (D-798 PO-authored):
```
| VP-100 | drain-timer expiry plugin.abandoned emission guarantee | integration | S-19.05 |
```

VP-INDEX v2.56 canonical H1 for VP-100:
```
Drain-Timer-Expiry Plugin.Abandoned Emission Guarantee — Mandatory Fields Cardinality and Mutual Exclusivity
```

The D-798 PO authoring dropped both discriminating clauses from the canonical VP title: (1) "Mandatory Fields Cardinality" and (2) "Mutual Exclusivity". These are the key behavioral properties that distinguish VP-100's scope. The paraphrased row `drain-timer expiry plugin.abandoned emission guarantee` is a partial title that fails POLICY 9 verbatim-derivation.

**Process gap:** D-798 PO authoring introduced the VP-100 row without verifying verbatim alignment with VP-INDEX SoT. This is a **[process-gap]** class — any BC §Verification Properties row addition MUST be verbatim-derived from VP-INDEX H1 text.

**Resolution:** CLOSED: PO ad464e09 — BC-3.08.001 v1.19→v1.20 (amended from D-798 version): VP-100 §Verification Properties row corrected to verbatim-derived cardinality + mutual-exclusivity form per VP-INDEX SoT. Input-hash 6549a11 unchanged.

---

### A.4 — F-P43-004: MEDIUM — VP-098 and VP-101 §Property Statement Inline PC Cites Off-by-One (POLICY 4 / POLICY 5 v1.3.3)

**Severity:** MEDIUM
**Policy violated:** POLICY 4 (internal inconsistency — §Property Statement inline PC parenthetical cites do not match the PC numbering in §Postconditions); POLICY 5 v1.3.3 (sibling-sweep — D-797 migration swept `source_bc`, `§Source Contract`, `§Traceability` but did NOT sweep body inline cites at same-file semantic-parallel sites)

**Evidence:**

After D-797 architect migration (VP-094/097/098/100/101 stable-anchor sweep), the `§Postconditions` sections of VP-098 and VP-101 were renumbered (PC numbers shifted). However, the `§Property Statement` sections of VP-098 and VP-101 contain inline parenthetical cites of the form `(BC-X.YY.ZZZ PCn)` that reference the OLD PC numbers.

- VP-098 §Property Statement: inline cite references `(BC-2.07.001 PC3)` but post-D-797 migration PC3 was renumbered → the correct anchor is now PC2 in §Postconditions.
- VP-101 §Property Statement: inline cite references `(BC-1.17.001 PC2)` but post-D-797 migration PC2 was renumbered → the correct anchor is now PC3 in §Postconditions.

**Process gap:** D-797 sibling-sweep covered `source_bc` / `§Source Contract` / `§Traceability` but did NOT cover `§Property Statement` inline parenthetical PC cites. This is a new sibling-sweep failure class — to be codified as POLICY 5 v1.3.8 category-(j) per this pass.

**Resolution:** CLOSED: architect 421a9e1f — VP-098 v1.1→v1.2 (PS-B PC3→PC2 + PS-C PC4→PC3 + Traceability PC4 dropped); VP-101 v1.1→v1.2 (PS-B PC2→PC3 + PS-C PC3→PC5). VP-100 v1.1→v1.2 additionally corrected: drain_window_ms added to PS-C + fixture; anchor referent → Event 5 mandatory-fields list; input-hash a2de4e4→6565e01.

---

### A.5 — F-P43-005: MEDIUM — BC-3.08.001 Changelog Missing v1.19 Row (POLICY 14 leg-2)

**Severity:** MEDIUM
**Policy violated:** POLICY 14 leg-2 (body Changelog — every version bump must have a corresponding Changelog row; v1.19 was added by D-797/D-798 PO work but the Changelog row was absent from BC-3.08.001 v1.20)

**Evidence:**

BC-3.08.001 v1.20 `## Changelog` section (D-798 state): v1.20 row present; v1.19 row absent. The v1.18→v1.19 bump (performed during the D-797/D-798 pre-pass consistency sweep) advanced the BC file version but did not add the corresponding Changelog entry documenting what changed at v1.19.

**Resolution:** CLOSED: PO ad464e09 — BC-3.08.001 v1.20 (amended): v1.19 Changelog row backfilled + v1.20 Amendment section added. Input-hash 6549a11 unchanged.

---

### A.6 — O-P43-001: LOW — BC-3.08.001 Bare-Date `last_amended` Non-Chain Form (POLICY 17)

**Severity:** LOW (observation)
**Policy noted:** POLICY 17 (epic/spec frontmatter completeness — `last_amended` MUST use chain form `"YYYY-MM-DD (vX.Y) — description [Prior: ...]"` not bare-date form)

**Evidence:** BC-3.08.001 v1.20 `last_amended` field used bare-date form rather than the canonical chain form. Prior version history was not preserved in the nested `[Prior: ...]` bracket convention.

**Resolution:** FIXED-IN-SCOPE: PO ad464e09 — `last_amended` canonicalized to chain form for v1.20 entry with nested Prior bracket for v1.19.

---

### A.7 — O-P43-002: LOW — VP-100 §Property Statement 6-of-7 Field Enumeration + Nonexistent "Invariant 6 Field Table" Referent (POLICY 4)

**Severity:** LOW (observation)
**Policy noted:** POLICY 4 (internal consistency — §Property Statement claimed 7 mandatory fields but enumerated only 6; referenced "Invariant 6 field table" which does not exist in BC-3.08.001 §Invariants)

**Evidence:**

VP-100 §Property Statement (pre-fix): enumerated 6 mandatory fields (type, trace_id, session_id, plugin_name, entry_index, exit_code) but BC-3.08.001 §Invariant 6 mandates 7 fields including `drain_window_ms`. Additionally, the prose cited "Invariant 6 field table" which is not a named sub-section in BC-3.08.001 §Invariants — the actual anchor is the Event 5 mandatory-fields list in §Postconditions.

**Resolution:** FIXED-IN-SCOPE: architect 421a9e1f — VP-100 v1.1→v1.2: drain_window_ms added to PS-C + fixture; anchor referent corrected to "Event 5 mandatory-fields list" per BC-3.08.001 §Postconditions; input-hash a2de4e4→6565e01.

---

## Part B — Per-Policy Attestations

### B.1 — POLICY 1 (Append-only IDs)
PASS. No IDs removed. All 7 S-19 stories and E-19 epic remain registered with continuous numbering. No story or BC ID deletions observed.

### B.2 — POLICY 2 (Version monotonicity)
PASS. All amendments advance version. BC-3.08.001 v1.19→v1.20 (D-798). VP-098/101 v1.1 (D-797). No downgrade observed.

### B.3 — POLICY 3 (Authoritative source)
PASS. All anchor references resolve at HEAD. F-P43-004 found stale PC cross-refs in VP body prose — closed by architect 421a9e1f.

### B.4 — POLICY 4 (Internal consistency)
PASS after fix. F-P43-003 found VP-100 row body inconsistency with VP-INDEX; F-P43-004 found VP-098/101 §Property Statement PC cite inconsistency with §Postconditions; O-P43-002 found VP-100 field enumeration gap. All closed.

### B.5 — POLICY 5 v1.3.5 (Stable-anchor BC-version-pin)
PASS. No new volatile pins introduced. D-797 migration stable-anchor sweep still holds.

### B.6 — POLICY 5 v1.3.3 (Same-burst sibling sweep)
FAIL (F-P43-004). D-797 sweep covered `source_bc` / `§Source Contract` / `§Traceability` but NOT `§Property Statement` inline parenthetical cites. New sibling-sweep category-(j) codified this pass (D-799 / POLICY 5 v1.3.8). CLOSED: architect 421a9e1f.

### B.7 — POLICY 5 v1.3.7 category-(i) (Same-file aggregation cells)
FAIL (F-P43-001). Epic H2 heading in STORY-INDEX is an aggregation cell that duplicates the epic frontmatter version — three consecutive epic bumps (v1.22→v1.23→v1.24→v1.25) all missed the heading update. CLOSED: this-commit (state-manager).

### B.8 — POLICY 6 (Subsystem canonical names)
PASS. All subsystem references (SS-01/02/03/04/05/07) match ARCH-INDEX canonical forms.

### B.9 — POLICY 7 (BC-INDEX title-cell verbatim H1)
PASS. No BC H1 changes this pass. All E-19 BC title cells in BC-INDEX remain verbatim H1.

### B.10 — POLICY 8 (BC-table propagation)
PASS. VP-100 §Verification Properties row fix (F-P43-003) requires STORY-INDEX cite sweep (S-19.05 + epic). VP-098/101 §Property Statement PC cite fix (F-P43-004) is VP-body-only — no BC version bump required (VPs reference BCs; BCs do not reference VP §Property Statement). BC-3.08.001 v1.21 (architect 421a9e1f) propagated to S-19.05 + epic by SW leg fbf344da per POLICY 8.

### B.11 — POLICY 9 (VP-INDEX propagation completeness)
FAIL (F-P43-003). BC-3.08.001 §Verification Properties VP-100 row was not verbatim-derived from VP-INDEX SoT. CLOSED: PO ad464e09. VP-INDEX itself unchanged this pass — all VP statements remain canonical.

### B.12 — POLICY 14 (5-leg quintuple parity on index bumps)
FAIL (F-P43-001, F-P43-002, F-P43-005). F-P43-001: leg-5 (upstream-index H2 heading) missed across three epic bumps. F-P43-002: leg-5 (upstream-index leading cite position) missed when S-19.03 was promoted. F-P43-005: leg-2 (body Changelog) missing v1.19 row. All closed: F-P43-001/002 this-commit; F-P43-005 PO ad464e09.

### B.13 — POLICY 15 (Traceability completeness)
PASS. VP-098/101 §Traceability sections updated at D-797 (architect 47b87f6e + a0c2c62a). F-P43-004 closed residual body-inline PC cite gap via architect 421a9e1f.

### B.14 — POLICY 16 (Decision-log global-max gate)
PASS. D-799 allocated this pass. Sequential from D-798 (confirmed max via `grep -oE "^## D-[0-9]+" decision-log.md | tail -1`).

### B.15 — POLICY 17 (Epic frontmatter completeness)
FAIL (F-P43-001, O-P43-001). F-P43-001: epic H2 heading stale in STORY-INDEX (aggregation cell). O-P43-001: BC-3.08.001 bare-date `last_amended`. Both closed: F-P43-001 this-commit; O-P43-001 PO ad464e09.

### B.16 — POLICY 18 (Input-hash completeness)
PASS. VP-100 input-hash a2de4e4→6565e01 (architect 421a9e1f). S-19.05 input-hash 9e54d68 unchanged (cite-sweep-only; BC-3.08.001 body content unchanged for cite purposes). BC-3.08.001 input-hash 6549a11 unchanged (PO ad464e09 amendment preserved hash).

### B.17 — POLICY 19 (ADR body stable-anchor form)
PASS. ADR-025 v1.13 and ADR-030 v1.3 both carry zero live volatile pins in normative Decision body sections.

### B.18 — POLICY 5 v1.3.6 (HEAD-reproducibility)
PASS. All stable-anchor greps from D-797 migration reproducible at HEAD. F-P43-004 PC cite gaps did not affect HEAD-reproducibility of `source_bc` anchors.

### B.19 — ADR body BC-cite sweep (D-795 enforcement gate)
PASS. ADR-025/ADR-030 normative sections: zero `BC-N.NN.NNN v[0-9]` hits outside amendment_reason/Changelog rows.

### B.20 — D-449(a) literal-shell gate obligation
PASS for fix bursts. POLICY 16 gate (D-799 max-grep stdout), 4-index gate (4 index version grep stdout), STORY-INDEX leading-cite audit (7-row stdout), and own-burst-log 8-block gate stdout captured in state-manager burst-log Dim-2 per D-449(a).

---

## Trajectory and Novelty Note

**Convergence trajectory (passes 40–43):** 1 → 0 → 3 → 5

Pass-43 increased from pass-42's 3 (MEDIUM-only) to 5 substantive findings (2 HIGH + 3 MEDIUM). The increase reflects two distinct defect classes that had been accumulating silently across multiple prior bursts:

**[process-gap] Class 1 — STORY-INDEX leading-cite / H2-heading not propagated on version bump (F-P43-001, F-P43-002):**
This class has recurred multiple times across E-19 history. When a story or epic is bumped, the STORY-INDEX upstream-index row leading cite (leg-5) and epic H2 heading must be updated same-burst. The D-797/D-798 bursts added version annotations in bracket form without promoting the leading cite position, leaving a stale leading cite across three consecutive bursts. State-manager discipline: every epic/story bump MUST reorder the STORY-INDEX row so the highest version is always the leading (un-bracketed) cite.

**[process-gap] Class 2 — BC §VP Properties row not verbatim-derived from VP-INDEX SoT (F-P43-003):**
D-798 PO authoring added the VP-100 §Verification Properties row by paraphrase rather than verbatim derivation from VP-INDEX canonical H1. This dropped the two discriminating clauses ("Mandatory Fields Cardinality" and "Mutual Exclusivity") that define VP-100's behavioral scope. PO discipline: BC §Verification Properties row additions MUST copy VP-INDEX H1 verbatim, not summarize.

**POLICY 5 v1.3.8 category-(j) codification:** F-P43-004 identifies a new sibling-sweep failure class — `§Property Statement` / `§Description` / `§Postcondition` body inline parenthetical PC cites at same-file semantic-parallel sites were NOT swept when D-797 renumbered PCs. POLICY 5 v1.3.8 is codified this pass to require same-burst sweep of inline parenthetical PC cites on any anchor migration or PC renumbering.

**Streak status:** 0/3. Convergence floor is not yet visible. Pass-44 dispatch with full v1.4.3 rubric.
