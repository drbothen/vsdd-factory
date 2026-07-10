# Adversarial Review — E-19 Pass 37 (post-D-791 delta; perimeter = epic v1.22 + full E-19 suite at D-791 versions)

**Perimeter:** epic v1.22 + S-19.01 v1.16 / S-19.02 v1.17 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.16 + STORY-INDEX v4.168 + VP-INDEX v2.55 + BC-4.13.001 v1.14 + BC-5.42.001 v1.5 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ADR-025 v1.12 + ADR-030 v1.3 + BC-INDEX v3.88 + ARCH-INDEX v2.97 + policies.yaml v1.4.2

**Reviewer:** fresh-context adversary; Iron Law; rubric policies.yaml v1.4.2

**Date:** 2026-07-09

**Verdict:** NOT-CLEAN — B0/H0/M1/L0 (1 finding)

**Model family:** Claude Opus 4.7

---

## Part A — Amendment Verifications + Finding

### A.1 — D-791 Amendment Verifications (2 verifications; all ✓)

**Verification 1: STORY-INDEX v4.168 wave-summary Input-hashes existence-verified against story frontmatter**

Wave-summary `Input-hashes:` line extracted from STORY-INDEX v4.168 aggregation paragraph:
`Input-hashes: S-19.01=d40bd21; S-19.02=604f45d; S-19.03=8d1225d; S-19.04=67eee80; S-19.05=9e54d68; S-19.06=998ac74; S-19.07=534c85c. All 7 distinct.`

Per-story frontmatter `input-hash:` values verified at HEAD:
- S-19.01: d40bd21 ✓
- S-19.02: 604f45d ✓
- S-19.03: 8d1225d ✓
- S-19.04: 67eee80 ✓
- S-19.05: 9e54d68 ✓
- S-19.06: 998ac74 ✓
- S-19.07: 534c85c ✓

All 7 wave-summary hash values match frontmatter. F-P36-001 fix confirmed complete. ✓

**Verification 2: policies.yaml v1.4.2 POLICY 5 v1.3.7 entry well-formed per v1.3.3..v1.3.6 pattern**

POLICY 5 v1.3.7 entry in policies.yaml v1.4.2 inspected. Category-(i) appended to Part C sibling-sweep categories:

`(i) same-file aggregation cells that duplicate per-row values (delivery summaries, "All N distinct" attestations, wave-summary Input-hashes lines, index aggregation paragraphs) — swept whenever the per-row cell they duplicate changes`

Entry structure matches the v1.3.3..v1.3.6 pattern: version header + description + categories list + verification_steps clause. ✓

---

### A.2 — Category-(i) Retro-Application Sweep

Having just codified POLICY 5 v1.3.7 category-(i) at D-791, a retro-application sweep of ALL aggregation cells in the E-19 perimeter is mandatory. Category-(i) defines: same-file aggregation cells that restate per-row values are sibling sites. Checked 14 aggregation cells across the E-19 perimeter (wave-summary Input-hashes/All-7-distinct/45-pts, epic Total, 7× story Token Budget Totals, VP-INDEX totals ×2, BC-coverage cell).

**14-cell category-(i) sweep table:**

| Cell | Location | Stated Value | Row Sum / Expected | Status |
|------|----------|-------------|-------------------|--------|
| wave-summary Input-hashes | STORY-INDEX v4.168 aggregation blockquote | S-19.01=d40bd21;...S-19.06=998ac74;S-19.07=534c85c | All 7 match story frontmatter (Verification 1 above) | CLEAN ✓ |
| wave-summary All-7-distinct | STORY-INDEX v4.168 aggregation blockquote | "All 7 distinct" | 7 unique hash values confirmed | CLEAN ✓ |
| wave-summary 45-pts | STORY-INDEX v4.168 aggregation blockquote | 45 pts | S-19.01(8)+S-19.02(8)+S-19.03(5)+S-19.04(5)+S-19.05(8)+S-19.06(8)+S-19.07(3) = 45 | CLEAN ✓ |
| epic Total | STORY-INDEX v4.168 epic header row | 45 pts | Same row-sum 45 | CLEAN ✓ |
| S-19.01 Token Budget Total | S-19.01 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| S-19.02 Token Budget Total | S-19.02 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| S-19.03 Token Budget Total | S-19.03 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| S-19.04 Token Budget Total | S-19.04 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| S-19.05 Token Budget Total | S-19.05 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| **S-19.06 Token Budget Total** | **S-19.06 v1.17 §Token Budget** | **~22,500** | **awk row-sum = 22,000** | **DEFECT — F-P37-001** |
| S-19.07 Token Budget Total | S-19.07 §Token Budget | stated Total matches row sum | sum-exact verified | CLEAN ✓ |
| VP-INDEX total VP count | VP-INDEX v2.55 frontmatter `total_vps` | 102 | matches count of VP-NNN entries in index body | CLEAN ✓ |
| VP-INDEX E-19 VP count | VP-INDEX v2.55 E-19 allocation row | 8 (VP-094..VP-101) | 8 VP-NNN entries in E-19 block | CLEAN ✓ |
| BC-coverage cell | STORY-INDEX v4.168 wave-summary blockquote | BC-5.42.001 v1.5 (S-19.01); BC-4.13.001 v1.14 (S-19.02 Phase-A + S-19.07 Phase-B; F-P35-001/002 D-790); BC-2.07.001 v1.4 (S-19.03); BC-2.02.011 v1.5 (S-19.03); BC-3.08.001 v1.19 LANDED (S-19.05); BC-1.17.001 v1.5 LANDED (S-19.06) | each BC version matches per-story row BC table Version cell | CLEAN ✓ |

**Result: 13 CLEAN / 1 DEFECT (S-19.06 Token Budget Total cell)**

---

### A.3 — F-P37-001 MEDIUM Finding

**F-P37-001 MEDIUM [POLICY 5 v1.3.7 category (i) aggregation-cell parity + POLICY 4 arithmetic anchor]**

**Location:** `S-19.06 v1.17` (`S-19.06-read-prefix-bounded-partial-read.md`) — `## Token Budget Estimate (MANDATORY)` section, `**Total**` row.

**Defect:** The `**Total**` row in S-19.06 v1.17's Token Budget Estimate table states `**~22,500**`. The verbatim 10-row table is:

```
| This story spec                                                         | ~5,500 |
| crates/factory-dispatcher/src/host/read_file.rs (reference impl)        | ~2,000 |
| crates/factory-dispatcher/src/host/path_util.rs (S-19.03 output)        | ~1,000 |
| crates/hook-sdk/src/host.rs (parallel read_file wrapper pattern)        | ~1,500 |
| plugins/vsdd-factory/hooks-registry.toml (capability schema preamble)  | ~2,000 |
| BC-1.17.001 v1.5                                                        | ~3,000 |
| ADR-025 (Decisions 13 + 15)                                             | ~1,500 |
| BC-2.07.001 (absent-file semantics reference)                           | ~1,500 |
| Test files (new unit + integration)                                     | ~2,500 |
| Tool outputs overhead                                                   | ~1,500 |
| **Total**                                                               | **~22,500** |
```

**awk extraction of row sums (excluding Total row):**

```
$ awk -F'|' '/^\|/ && !/Total/ && /~[0-9]/ { val=$3; gsub(/[^0-9]/, "", val); sum += val } END { print sum }' \
    .factory/stories/S-19.06-read-prefix-bounded-partial-read.md
22000
```

stdout: `22000`

**Arithmetic:** 5500 + 2000 + 1000 + 1500 + 2000 + 3000 + 1500 + 1500 + 2500 + 1500 = **22,000**

**Discrepancy:** stated `~22,500` vs row-sum `22,000`. Gap = 500 tokens. No rounding margin explains a 500-token over-statement when every component value is already expressed as a multiple of ~500.

**Sibling-parity table (6/7 stories sum-exact — establishes convention):**

```
$ for f in .factory/stories/S-19.0[1-9]*.md; do
    total=$(grep -m1 '\*\*Total\*\*.*~[0-9]' "$f" | grep -oE '~[0-9,]+' | grep -oE '[0-9,]+' | tr -d ',')
    rowsum=$(awk -F'|' '/^\|/ && !/Total/ && /~[0-9]/ { val=$3; gsub(/[^0-9]/, "", val); sum += val } END { print sum }' "$f")
    echo "$(basename $f): stated=~${total} rowsum=${rowsum} $([ "$total" = "$rowsum" ] && echo MATCH || echo MISMATCH)"
  done
```

Result (summary):
- S-19.01: stated=~27000 rowsum=27000 MATCH
- S-19.02: stated=~29000 rowsum=29000 MATCH
- S-19.03: stated=~19000 rowsum=19000 MATCH
- S-19.04: stated=~14000 rowsum=14000 MATCH
- S-19.05: stated=~20000 rowsum=20000 MATCH
- **S-19.06: stated=~22500 rowsum=22000 MISMATCH**
- S-19.07: stated=~22000 rowsum=22000 MATCH

6/7 stories sum-exact. Convention established: Token Budget `**Total**` row MUST equal sum of above rows. S-19.06 is the sole violator.

**Escape class:** Novel META-33 sub-route: aggregation-cell/Token-Budget-Total axis. This finding was not possible to surface as a distinct POLICY 5 finding prior to D-791 because POLICY 5 v1.3.5 categories (a)–(h) do not enumerate Token Budget Total cells as sibling-sweep targets. The freshly-codified category-(i) "same-file aggregation cells that duplicate per-row values" unambiguously covers the Token Budget `**Total**` row — it is an arithmetic aggregation of the above per-row token estimates. Retro-applying category-(i) to the current perimeter immediately surfaced this mismatch. Novelty: MEDIUM (first confirmed Token-Budget-Total axis; prior META-33 instances were hash-value or delivery-summary cells).

**Adjudication (a) — inception error:** Verified via git history. The `**Total**` value `~22,500` is present in the initial v1.0 draft commit (SHA bfab9ad7). All 17 subsequent versions (v1.0–v1.17) left the Total cell unchanged, while the 10 component rows were never touched in a way that would alter their values. The arithmetic error was introduced at authorship, not during any fix burst. No dropped row. Default adjudication (a) per finding rubric.

**CLOSED** — Fixed by story-writer commit `8c32bc3a` (S-19.06 v1.17→v1.18: `**Total**` cell `~22,500` → `~22,000`; input-hash UNCHANGED at 998ac74 — no `inputs[]` file changed; D-449(a) sum gate `22000==22000 PASS`).

---

## Part B — POLICY Sweeps

### B.1 — POLICY 1 Sweep (BC authorship + ID assignment)

All E-19 BCs carry valid BC-S.SS.NNN IDs. No phantom or duplicate IDs detected. No new BCs introduced this pass. CLEAN ✓.

### B.2 — POLICY 4 Sweep (semantic anchoring — arithmetic anchors)

F-P37-001 is a POLICY 4 violation: the `**Total**` aggregation cell is a semantic anchor that must equal the arithmetic sum of its row components. The discrepancy (~22,500 vs 22,000) violates POLICY 4's requirement that all arithmetic summary cells be exact. CLOSED via `8c32bc3a`. Remaining perimeter POLICY 4 cells verified: all other E-19 Token Budget Totals sum-exact (B.A above); all cross-artifact anchors (§Decision N, Deliverable D18, VP-NNN) verified in prior passes and unchanged. CLEAN except F-P37-001 (CLOSED).

### B.3 — POLICY 5 Sweep (version-chain completeness + sibling-sweep)

POLICY 5 v1.3.7 category-(i) retro-application sweep completed (A.2 above): 14 aggregation cells checked; 13 CLEAN; 1 DEFECT (F-P37-001, CLOSED). Per-row cells in STORY-INDEX v4.168: S-19.02 row v1.17/604f45d ✓; S-19.07 row v1.16/534c85c ✓; S-19.06 row v1.17/998ac74 ✓ (pre-fix; post-fix per this burst story-writer leg). Pass-35 clause present in wave-summary delivery summary ✓. CLEAN except F-P37-001 (CLOSED).

### B.4 — POLICY 6 Sweep (ARCH-INDEX subsystem names)

All E-19 stories reference correct subsystem IDs (SS-01, SS-02, SS-03, SS-04, SS-07, SS-09) per ARCH-INDEX v2.97. No stale or phantom SS-NNN references detected. CLEAN ✓.

### B.5 — POLICY 7 Sweep (BC H1 canonical title form)

BC-INDEX v3.88 BC-4.13.001/BC-5.42.001/BC-2.07.001/BC-2.02.011/BC-3.08.001/BC-1.17.001 catalog row title cells verified against respective BC H1 forms. All title cells accurate and complete (no POLICY 7 elision). CLEAN ✓.

### B.6 — POLICY 8 Sweep (BC table completeness — per-AC BC-trace column + Token Budget row)

All 6 E-19 perimeter BCs carry complete §Behavioral Contracts table with per-AC BC-trace column and Token Budget row. S-19.06 v1.17 §Behavioral Contracts table verified: BC-1.17.001 v1.5 cited in Version cell ✓. Token Budget row present in all 7 stories ✓ (Total cell fix is F-P37-001, CLOSED). CLEAN except F-P37-001 (CLOSED in this burst).

### B.7 — POLICY 9 Sweep (VP anchor back-citation)

BC-4.13.001 v1.14 §VP Anchors: VP-095 v1.1 + VP-096 v1.1 both cited ✓ (D-783 F-P29-004 fix). BC-1.17.001 v1.5 §VP Anchors: VP-101 v1.1 cited ✓. All other E-19 BCs with VP obligations verified. No new VP anchors introduced this pass. CLEAN ✓.

### B.8 — POLICY 13 Sweep (gate-execution evidence — shell gates must emit stdout)

All S-19.06 v1.17 shell gates reviewed: AC-003 gate (grep -vE non-comment + grep -oE pattern), AC-007 gates (Gate 1/Gate 2 three-clause/Gate 3) carry explicit capture patterns per D-766 §4 convention. No bare assertion gates without execution-evidence forms. CLEAN ✓.

### B.9 — POLICY 14 Sweep (5-leg quintuple parity on version bumps)

S-19.06 v1.17: the `v1.17` version appears consistently in (1) version: frontmatter `"1.17"`, (2) body Changelog `- "2026-07-09 v1.17: ..."`, (3) modified: array `- "2026-07-09 v1.17"`, (4) last_amended: string, (5) STORY-INDEX row note `story v1.17`. All 5 legs present ✓. Note: F-P37-001 fix will advance to v1.18 with corresponding 5-leg parity (story-writer obligation; CLOSED). CLEAN ✓.

### B.10 — POLICY 15 Sweep (input-hash freshness — POLICY 18 covered here)

S-19.06 v1.17 `input-hash: "998ac74"` — not a placeholder string. Matches the hash at last story-writer story-content update (D-784 pass-30). F-P37-001 is a story-spec internal arithmetic error in the Token Budget, NOT an inputs[] file change; hence input-hash correctly UNCHANGED at `998ac74`. Wave-summary aggregation confirms S-19.06=998ac74 ✓. POLICY 15 CLEAN ✓.

### B.11 — POLICY 16 Sweep (global-max gate — D-792 must be final entry)

Pre-update: D-791 is global max in decision-log.md. D-792 will be allocated in this burst. POLICY 16 gate compliance: prior to this burst, D-791 was the final entry ✓. Post-allocation, D-792 will be final entry ✓ (verified via gate at Commit B). CLEAN ✓.

### B.12 — POLICY 17 Sweep (epic POLICY 17 completeness)

E-19 epic v1.22 `modified[]` + `last_amended` present ✓ (per D-773 20/20 epics sweep). Epic body sections verified: §Description, §Stories, §Acceptance Criteria, §Out of Scope, §Dependencies all present ✓. CLEAN ✓.

### B.13 — POLICY 19 Sweep (ADR volatile-pin check)

All E-19 perimeter BCs §Traceability ADR Reference cells verified stable-anchor form:
- BC-4.13.001 v1.14: `ADR-025 §Decision 1/14/15 and Deliverable D18` ✓ (D-790 fix)
- BC-1.17.001 v1.5: `ADR-025 §Decision 15` (stable) ✓
- BC-2.07.001 v1.4: `ADR-025 §Decision 1` (stable) ✓
- BC-2.02.011 v1.5: no ADR pin ✓
- BC-3.08.001 v1.19: `ADR-026 §Decision 1/2/4` (stable) ✓
- BC-5.42.001 v1.5: no ADR pin ✓

Out-of-perimeter note: O-P35-001 (BC-5.40.001 + BC-6.23.001 ADR-025 v1.2 volatile-pins) tracked in STATE.md Drift Items; not re-reported. CLEAN ✓.

---

## Part B.14 — Trajectory Note

**Novelty:** MEDIUM. F-P37-001 is a novel aggregation-cell sub-route of META-33: the Token-Budget-Total axis. The Token Budget `**Total**` row aggregates per-row token estimates and must be arithmetic-exact (no rounding margin of 500 tokens). This is the first confirmed instance of the aggregation-cell pattern on the Token Budget axis — prior META-33 sub-routes (F-P36-001) targeted hash-value aggregation in delivery summaries. The POLICY 5 v1.3.7 category-(i) codification at D-791 enables mechanical detection of this class retroactively.

**Trajectory:** Pass-37 is pass number 37 of the E-19 spec cascade. Finding count: 1 (B0/H0/M1/L0). Prior pass: pass-36 = 1 finding. Trajectory tail (passes 34–37): 1→2→1→1. No severity regression (B0/H0 maintained from pass-36). Streak: 0/3 (pass-37 NOT-CLEAN). Pass-38 required.

Full trajectory (passes 22–37): 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1.
