# Adversarial Review — E-19 Pass 47 (rubric v1.4.3; streak 0/3)

## Header

| Field | Value |
|-------|-------|
| **Pass** | 47 |
| **Type** | Periodic adversarial review |
| **Verdict** | NOT-CLEAN B0/H0/M1/L0 |
| **Finding counts** | BLOCKER: 0; HIGH: 0; MEDIUM: 1; LOW: 0 |
| **Streak before** | 0/3 |
| **Streak after** | 0/3 (reset by 1 MEDIUM finding) |
| **Model family** | Claude Opus 4.7 |
| **Iron Law** | Fresh context; zero prior passes loaded; Part A of pass-46 only |
| **Rubric** | policies.yaml v1.4.3 + L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row (D-800) + L-BB-remediation-predicate-must-enumerate-all-same-burst-touched-artifacts (D-801) + L-BB-modified-array-monotonicity-perimeter-audit (D-802) + L-BB-write-frontmatter-history-after-body-replace-all (D-802) |
| **Perimeter** | D-802 delta + full E-19 carry-forward: BC-INDEX v3.95; VP-INDEX v2.59; STORY-INDEX v4.174; BC-1.17.001 v1.6 (ebf73ff); VP-098/100/101 v1.2 modified[] re-sorted; epic v1.26 (fb55113); S-19.06 v1.19 (998ac74→e6c23de); policies.yaml v1.4.3 |
| **Date** | 2026-07-10 |

---

## Part A — Findings and Evidence

### A.1 — F-P47-001: MEDIUM — STORY-INDEX §Epic E-19 Heading Cites `draft, v1.25`; Epic File is v1.26 (POLICY 14 leg-5 + POLICY 5 v1.3.3 [regression])

**Severity:** MEDIUM
**Policy violated:** POLICY 14 leg-5 (upstream-index parity — STORY-INDEX §Epic heading version token must match epic file frontmatter version); POLICY 5 v1.3.3 (same-burst sibling sweep completeness — a sweep that advances an epic version must update ALL STORY-INDEX sites citing that version, including the §heading)

**Evidence — STORY-INDEX §Epic E-19 heading (pre-fix, as reviewed):**
```
## Epic E-19 — Post-rc.22 Operator Hardening (v1.0-feature-engine-discipline-pass-1 / E-19 F3) — draft, v1.25
```
Line ~683 of STORY-INDEX.md v4.174. Version token: `v1.25`.

**Evidence — epic file frontmatter (at HEAD):**
```yaml
# .factory/stories/epics/E-19-post-rc22-operator-hardening.md
version: "v1.26"
```
Version: `v1.26`. Delta: §heading carries `v1.25` while epic file carries `v1.26` — stale by one increment.

**Control comparison — E-17 heading (PASS):**
```
## Epic E-17 — Factory State Durability and Concurrency ... — ... v1.1
```
E-17 epic file version: `v1.1`. PASS — heading matches file.

**Control comparison — E-18 heading (PASS):**
```
## Epic E-18 — Factory Context Durability ... — ... v1.3
```
E-18 epic file version: `v1.3`. PASS — heading matches file.

**D-802 attestation contrast:** D-802 SW leg (71be7861) burst-log attestation reads: "epic v1.25→v1.26 ×4 sites, input-hash c3feb1c→fb55113 (SW leg 71be7861)." The D-802 SM burst-log entry (BC-1.17.001 v1.5→v1.6 LANDED carry-forward annotation ×4 sites) also attests `epic v1.26`. Both attestations cover the epic FILE's ×4 sites (wave-summary clause, BC-coverage line, and inline citations) but DO NOT enumerate the STORY-INDEX §Epic E-19 H2 heading as a sweep site. The §heading is a fifth STORY-INDEX site that was missed in the D-802 sweep.

**Root cause:** The D-802 SW leg sweep enumerated the epic file's body cite sites (×4 occurrences of `v1.25` in the epic file body) but treated the STORY-INDEX §heading as covered by the STORY-INDEX `last_amended` carry-forward rather than as an explicit sweep obligation. The §-heading version token is structurally distinct from the row-level leading-cite and wave-summary lines — it requires explicit grep-based enumeration rather than implicit carry-forward.

**Recurrence note:** This is the exact F-P43-001 class (D-799 pass-43; HIGH finding): STORY-INDEX §Epic E-19 H2 heading carried a stale version. D-799 fix burst advanced the heading from v1.22→v1.25. D-802 burst advanced the epic to v1.26 but missed the heading again. META-note: self-application lag — the D-802 burst codified L-BB-write-frontmatter-history-after-body-replace-all and L-BB-modified-array-monotonicity-perimeter-audit (body/frontmatter ordering discipline and perimeter-audit discipline) while under-applying the D-799/D-801 enumeration lessons that specifically target STORY-INDEX §heading parity as a mandatory sweep site.

**Scope:** STORY-INDEX.md only. STORY-INDEX v4.174→v4.175 (§heading corrected v1.25→v1.26). No other artifacts require changes — this is a sole SM leg fix.

**Resolution:** CLOSED: state-manager this-commit — STORY-INDEX v4.174→v4.175: line 683 `draft, v1.25` → `draft, v1.26`; frontmatter `version: "4.174"` → `"4.175"`; `last_amended` prepended v4.175 entry. Heading-parity gate run across all 20 epic files: 11 PASS, 0 FAIL (post-fix), 9 SKIP (headings without version tokens). See D-803 Dim-2 Gate for captured stdout.

---

## Part B — Attestation Matrix

### B.1 — Carry-Forward Artifact Table (26 artifacts; D-802 delta + full E-19 carry-forward)

Exhaustive version check against §Artifact Versions at D-802 Closure table:

| Artifact | Expected Version | Carry-Forward Status |
|----------|-----------------|---------------------|
| ADR-025 | v1.13 | PASS — UNCHANGED D-797..D-802 |
| ADR-030 | v1.3 | PASS — UNCHANGED D-797..D-802 |
| BC-4.13.001 | v1.14 | PASS — UNCHANGED D-797..D-802 |
| BC-1.17.001 | v1.6 | PASS — D-802 PO c2a1f656 modified[] re-sorted; input-hash ebf73ff |
| BC-2.07.001 | v1.5 | PASS — UNCHANGED D-798..D-802 |
| BC-2.02.011 | v1.7 | PASS — D-801 PO 6f813e9e modified[] re-sorted; UNCHANGED D-802 |
| BC-3.08.001 | v1.21 | PASS — UNCHANGED D-800..D-802 |
| BC-5.42.001 | v1.6 | PASS — UNCHANGED D-799..D-802 |
| VP-094.md | v1.1 | PASS — UNCHANGED D-798..D-802 |
| VP-095.md | v1.1 | PASS — UNCHANGED D-797..D-802 |
| VP-096.md | v1.1 | PASS — UNCHANGED D-797..D-802 |
| VP-097.md | v1.1 | PASS — UNCHANGED D-798..D-802 |
| VP-098.md | v1.2 | PASS — modified[] re-sorted D-802 same-burst; input-hash updated |
| VP-100.md | v1.2 | PASS — modified[] re-sorted D-802 same-burst; input-hash updated |
| VP-101.md | v1.2 | PASS — modified[] re-sorted D-802; input-hash 531cd2f (BC-1.17.001 v1.6 input drift) |
| S-19.01 | v1.17 | PASS — UNCHANGED D-799..D-802 |
| S-19.02 | v1.17 | PASS — UNCHANGED D-797..D-802 |
| S-19.03 | v1.19 | PASS — UNCHANGED D-802 |
| S-19.04 | v1.11 | PASS — UNCHANGED D-797..D-802 |
| S-19.05 | v1.16 | PASS — UNCHANGED D-800..D-802 |
| S-19.06 | v1.19 | PASS — D-802 SW 71be7861 BC-1.17.001 v1.5→v1.6 cite sweep ×10 sites; input-hash e6c23de |
| S-19.07 | v1.16 | PASS — UNCHANGED D-797..D-802 |
| epic (E-19) | v1.26 | PASS — D-802 SW 71be7861 BC-1.17.001 v1.5→v1.6 cite sweep ×4 sites; input-hash fb55113 |
| policies.yaml | v1.4.3 | PASS — UNCHANGED D-800..D-802 |
| BC-INDEX | v3.95 | PASS — D-802 SM this-commit (BC-1.17.001 v1.6 cell) |
| VP-INDEX | v2.59 | PASS — D-802 SM this-commit (Full Index VP-098/100/101 v1.2 annotations; Story Anchors VP-098/101 suffixed IDs) |

**STORY-INDEX v4.174:** FAIL (pre-fix) — §Epic E-19 heading carries `v1.25` while epic file is `v1.26`. → F-P47-001 MEDIUM.
**ARCH-INDEX v2.98:** PASS — UNCHANGED D-796..D-802 (exhaustive).

Zero-match sentinel — `v1.25` stale token sweep in carry-forward artifacts:
```
grep -l "v1\.25" .factory/stories/STORY-INDEX.md 2>/dev/null
```
Pre-fix: `STORY-INDEX.md` (§heading only). Post-fix: zero matches expected.

### B.2 — POLICY 1 (Append-only IDs)
PASS. No IDs removed. All 7 S-19 stories, E-19 epic, VP-094..VP-101, all BCs registered at D-802 remain registered. No BC, VP, story, or epic ID deletions in D-802 delta.

### B.3 — POLICY 2 (Version monotonicity)
PASS for all D-802 delta artifacts. F-P46-001/O-P46-001 modified[] defects are closed at D-802 (PO c2a1f656 + SM D-802 burst). BC-INDEX v3.94→v3.95 monotonic. VP-INDEX v2.58→v2.59 monotonic. STORY-INDEX v4.173→v4.174 monotonic. Epic v1.25→v1.26 monotonic. S-19.06 v1.18→v1.19 monotonic.

### B.4 — POLICY 3 (Authoritative source)
PASS for D-802 delta. BC-INDEX BC-1.17.001 v1.6 cell sourced from BC-1.17.001 Changelog SoT (PO c2a1f656; input-hash ebf73ff). VP-INDEX Full Index VP-098/100/101 v1.2 annotations sourced from each VP file's last_amended SoT (per D-800 SoT-derivation rule). VP-INDEX Story Anchors VP-098 F-P43-004a, VP-101 F-P43-004b per VP file SoTs. STORY-INDEX §heading `v1.25` does NOT derive from epic file SoT `v1.26` — F-P47-001 MEDIUM.

### B.5 — POLICY 4 (Internal consistency)
FAIL on STORY-INDEX §heading v1.25 vs epic file v1.26 — inconsistency within same-file / cross-file citation scope. §heading, last_amended epic entry, and wave-summary clause must all consistently cite v1.26 after D-802 fix. The wave-summary and last_amended cite v1.26 (updated by D-802 SW 71be7861) while the §heading still cites v1.25 — internal inconsistency within STORY-INDEX. → F-P47-001.

### B.6 — POLICY 5 v1.3.3 (Same-burst sibling sweep completeness [regression])
FAIL [regression]. D-802 SW leg (71be7861) swept epic v1.25→v1.26 ×4 sites but did not enumerate the STORY-INDEX §heading as a sweep site. The §heading is a STORY-INDEX cite of the epic version and falls under POLICY 5 v1.3.3 same-burst sibling-sweep obligation alongside the wave-summary, BC-coverage, and leading-cite lines. Missing the §heading constitutes an incomplete sweep for the epic v1.25→v1.26 bump. → F-P47-001.

### B.7 — POLICY 5 v1.3.5 (Stable-anchor BC-version-pin)
PASS. No new volatile pins introduced in D-802 delta. S-19.06 v1.19 body carries BC-1.17.001 v1.6 stable-anchor form. Epic v1.26 body carries BC-1.17.001 v1.6 stable-anchor form (D-802 SW 71be7861 sweep). All carry-forward VPs and BCs at stable-anchor form.

### B.8 — POLICY 5 v1.3.7 category-(i) (Same-file aggregation cells)
PASS. S-19.06 wave-summary Input-hashes cell updated (998ac74→e6c23de) by D-802 SW 71be7861. No new aggregation-cell obligations triggered by D-802 delta. STORY-INDEX Token Budget Total for S-19.06 ~22,000 (v1.18→v1.19 UNCHANGED per D-792 — no re-computation needed for §heading fix).

### B.9 — POLICY 5 v1.3.8 category-(j) (Inline parenthetical PC cites)
PASS. VP-098/100/101 v1.2 category-(j) PC cites verified at D-799. No category-(j) sites touched in D-802 delta. §heading fix (F-P47-001) is a version-token correction, not a PC cite.

### B.10 — POLICY 6 (Subsystem canonical names)
PASS. All subsystem references (SS-01/02/03/04/05/07) match ARCH-INDEX v2.98 canonical forms. No SS rename in D-802 delta.

### B.11 — POLICY 7 (BC-INDEX title-cell verbatim H1)
PASS. No BC H1 changes in D-802 delta. BC-1.17.001 title cell in BC-INDEX carries verbatim H1 (verified at D-802 SM burst; PO c2a1f656 modified[] re-sort did not alter title).

### B.12 — POLICY 8 (BC-table propagation)
PASS. BC-1.17.001 v1.6 (D-802 PO c2a1f656) — modified[] re-sort only; no body content change; no new §Traceability row required; cite propagation (S-19.06 v1.18→v1.19 cite sweep + epic v1.25→v1.26 cite sweep) completed by D-802 SW 71be7861. No further propagation triggered by F-P47-001 §heading fix (version-token correction only; no BC body cite change).

### B.13 — POLICY 9 (VP-INDEX propagation completeness)
PASS. VP-INDEX v2.59 Full Index rows for VP-094..VP-101 verified complete. O-P46-001/002 from pass-46 closed at D-802 SM burst. VP-INDEX v2.59 carries correct Full Index + Story Anchors annotations at D-802. No VP-INDEX changes required for F-P47-001 §heading fix.

### B.14 — POLICY 14 (5-leg quintuple parity on all D-802 delta bumps)
PASS for BC-1.17.001 v1.6 (PO c2a1f656): leg-1 version frontmatter ✓; leg-2 Changelog row ✓; leg-3 modified[] re-sorted ✓; leg-4 last_amended ✓; leg-5 BC-INDEX v3.95 cell ✓. PASS for S-19.06 v1.19 (SW 71be7861): leg-1 ✓; leg-2 ✓; leg-3 ✓; leg-4 ✓; leg-5 STORY-INDEX v4.174 row ✓. PASS for epic v1.26 (SW 71be7861): all 5 legs ✓. FAIL on STORY-INDEX v4.174 leg-5 upstream-index parity for epic: §heading carries stale v1.25 → F-P47-001 MEDIUM.

### B.15 — POLICY 15 (Traceability completeness)
PASS. No new BCs or VPs introduced in D-802 delta. All existing §Traceability sections verified at D-802 carry-forward.

### B.16 — POLICY 16 (Decision-log global-max gate)
PASS. D-803 allocated this pass. Grep of decision-log.md tail confirms last entry heading `## D-802`. D-803 is correct next allocation.

### B.17 — POLICY 17 (Epic frontmatter completeness)
FAIL (pre-fix, F-P47-001): STORY-INDEX §Epic E-19 heading version token `v1.25` does not match epic frontmatter `v1.26`. Epic file itself (E-19-post-rc22-operator-hardening.md) frontmatter is complete and correct at v1.26. The defect is in the STORY-INDEX §heading, not the epic file. CLOSED: SM this-commit STORY-INDEX v4.175.

### B.18 — POLICY 18 (Input-hash completeness)
PASS. S-19.06 input-hash e6c23de (D-802 SW 71be7861); epic input-hash fb55113 (D-802 SW 71be7861). BC-1.17.001 input-hash ebf73ff (D-802 PO c2a1f656). VP-101 input-hash 531cd2f (D-802 SM burst compute-input-hash). STORY-INDEX §heading fix is a version-token text change; input-hash for STORY-INDEX is `[live-state]` — no recompute obligation.

### B.19 — POLICY 19 (ADR body stable-anchor form)
PASS. ADR-025 v1.13 and ADR-030 v1.3 carry zero volatile BC-version-pins in normative Decision body sections. No ADR changes in D-802 delta.

### B.20 — ADR body BC-cite sweep (D-795 enforcement gate)
PASS. ADR-025/ADR-030 normative sections: zero `BC-N.NN.NNN v[0-9]` volatile pins in non-amendment_reason/non-Changelog rows (verified D-802). No ADR changes in D-802 delta requiring new sweep.

### B.21 — D-449(a) literal-shell gate obligation
PASS for D-802 fix burst per burst-log D-802 Dim-2 gates (8 gates captured). For D-803 closure burst (this-commit): 4-index gate, heading-parity gate, and own-burst-log 8-block gate will be captured in burst-log D-803 Dim-2 per D-449(a).

### B.22 — L-BB-per-artifact-catalog-cell-derives-from-own-changelog-row (D-800 gate)
PASS. All D-802 SM catalog cells (BC-1.17.001 v1.6 cell; VP-INDEX Full Index VP-098/100/101 v1.2 annotations) verified against each artifact's own Changelog/last_amended SoT. No burst-date substitution in D-802 SM leg.

### B.23 — L-BB-remediation-predicate-must-enumerate-all-same-burst-touched-artifacts (D-801 gate)
PASS. D-802 SM burst perimeter audit explicitly enumerated: (a) VP files modified[] re-sort for VP-094..VP-101; (b) VP-INDEX Full Index rows for VP-098/100/101; (c) VP-INDEX Story Anchors VP-098/101 finding-ID corrections. All same-burst D-799 artifacts from architect 421a9e1f scoped and fixed. F-P47-001 is a NEW miss in D-802 (SW leg scope, not D-799 origin) — it does not represent a B.23 failure; it represents a B.23-class gap in D-802 SW sweep enumeration.

### B.24 — L-BB-modified-array-monotonicity-perimeter-audit (D-802 gate)
PASS. D-802 SM burst ran modified[] class audit across VP-094..VP-101 (all 8 VP files); all show version-monotonic ordering post-fix. No new modified[] non-monotonic defects in D-802 delta. F-P47-001 is not a modified[] issue — it is a §heading version-token sweep miss.

### B.25 — L-BB-write-frontmatter-history-after-body-replace-all (D-802 gate)
PASS for D-802 delta artifacts. BC-1.17.001 v1.6 (PO c2a1f656): modified[] re-sorted ascending; last entry v1.6 == version: "1.6" ✓. S-19.06 v1.19 (SW 71be7861): body cite sweep preceded frontmatter history append ✓. Epic v1.26 (SW 71be7861): same ✓. F-P47-001 is not a write-order defect — it is a sweep-completeness defect (a sweep site was not enumerated, not that the wrong write order was used).

### B.26 — Heading-parity gate (new standing gate — L-BB-epic-heading-parity-is-a-mandatory-commit-E-gate)
FAIL → F-P47-001. E-19 §heading carries `v1.25` while epic file carries `v1.26`. Controls: E-17 §heading v1.1 matches epic file v1.1 ✓; E-18 §heading v1.3 matches epic file v1.3 ✓. This gate should be run as a mandatory Commit-E check on every SM burst that touches any epic or story version. Codified as L-BB-epic-heading-parity-is-a-mandatory-commit-E-gate [process-gap] at D-803.

---

## Trajectory and Novelty Note

**Convergence trajectory (passes 22–47):** 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2→3→1→1

Pass-47 holds at 1 finding (B0/H0/M1/L0), matching pass-46's count of 1. The single MEDIUM finding (F-P47-001) is a POLICY 14 leg-5 + POLICY 5 v1.3.3 recurrence of the exact F-P43-001 class (D-799): STORY-INDEX §Epic E-19 H2 heading stale. Two of the seven consecutive zero-HIGH passes have now produced a 1-MEDIUM finding recurring in the heading-parity class.

**META-note — self-application lag:** The D-802 burst codified two new lessons (L-BB-modified-array-monotonicity-perimeter-audit + L-BB-write-frontmatter-history-after-body-replace-all) while UNDER-APPLYING the prior lesson from D-799/D-801 that specifically names STORY-INDEX §heading as a mandatory sweep site for epic version bumps. The codified lessons address write-order and perimeter-audit obligations, but the §heading-as-sweep-site discipline was left implicit (not codified as a literal-shell gate obligation). This self-application lag is the structural root of F-P47-001: the burst author applied body-before-frontmatter discipline correctly (L-BB-write-frontmatter-history-after-body-replace-all ✓) and ran the perimeter audit (L-BB-modified-array-monotonicity-perimeter-audit ✓) while failing to enumerate the §heading as a fifth sweep site. Result: the new lesson L-BB-epic-heading-parity-is-a-mandatory-commit-E-gate [process-gap] closes this codification gap by making the heading-parity gate a mandatory literal-shell Commit-E obligation.

**Seven consecutive zero-HIGH passes.** The finding distribution continues to thin (trajectory tail →2→3→1→1). The asymptotic floor appears to be at 0–1 MEDIUM findings per pass, with the recurrent class being §heading parity / sweep-completeness gaps rather than structural spec defects.

**Streak status:** 0/3. Pass-48 dispatch with full v1.4.3 rubric + D-803 carry-forward (heading-parity gate as standing Commit-E control).

---

## CLOSED Annotations

| Finding | Status | Commit |
|---------|--------|--------|
| F-P47-001 MEDIUM: STORY-INDEX §Epic E-19 heading `draft, v1.25` vs epic file v1.26 (POLICY 14 leg-5 + POLICY 5 v1.3.3) | CLOSED | state-manager this-commit (STORY-INDEX v4.174→v4.175: line 683 heading `v1.25`→`v1.26`; version `"4.174"`→`"4.175"`; last_amended prepended v4.175 entry; heading-parity gate all 20 epics 0 FAILs post-fix) |
