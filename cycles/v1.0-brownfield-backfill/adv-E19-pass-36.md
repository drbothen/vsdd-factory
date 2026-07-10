# Adversarial Review — E-19 Pass 36 (post-D-790 delta; perimeter = epic v1.22 + full E-19 suite at D-790 versions)

**Perimeter:** epic v1.22 + S-19.01 v1.16 / S-19.02 v1.17 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.16 + STORY-INDEX v4.167 + VP-INDEX v2.55 + BC-5.42.001 v1.5 + BC-4.13.001 v1.14 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ADR-025 v1.12 + ADR-030 v1.3 + BC-INDEX v3.88 + ARCH-INDEX v2.97

**Reviewer:** fresh-context adversary; Iron Law; rubric policies.yaml v1.4.1

**Date:** 2026-07-09

**Verdict:** NOT-CLEAN — B0/H0/M1/L0 (1 finding)

**Model family:** Claude Opus 4.7

---

## Part A — Amendment Verifications + Finding

### A.1 — D-790 Amendment Verifications (5 verifications; all ✓)

**Verification 1: BC-4.13.001 v1.14 §Traceability anchors (§Decision 1/§Decision 14/§Decision 15/Deliverable D18) existence-verified at ADR-025 v1.12**

Anchor grep results:
- `### Decision 1` at line 109 ✓
- `### Decision 14` at line 1096 ✓
- `### Decision 15` at line 1132 ✓
- Deliverable D18 at line 1210 ✓ (Concrete Deliverables table row for `host::read_prefix`)
- Confirmed: no `### Decision 18` header exists (only 15 Decision headers; D18 = Deliverable row) ✓

**Verification 2: BC-4.13.001 v1.14 §Description enumeration**

§Description second paragraph cites Decisions 1/2/3/4/7/9/10/14/15 + Deliverables D1/D2/D9/D18. All 9 decisions and 4 deliverables existence-verified against ADR-025 v1.12 §Decision headers and Concrete Deliverables table. ✓

**Verification 3: S-19.02 v1.17 cite sweep**

All 18 body-scope BC-4.13.001 v1.13 cites replaced with v1.14. D-779 whole-file predicate: zero live-body v1.13 references remain (historical [Prior:] clauses excluded per POLICY 5 v1.3.5 Part A). Input-hash 604f45d matches frontmatter. ✓

**Verification 4: S-19.07 v1.16 cite sweep**

All 12 body-scope BC-4.13.001 v1.13 cites replaced with v1.14. D-779 whole-file predicate: zero live-body v1.13 references remain. Input-hash 534c85c matches frontmatter. ✓

**Verification 5: STORY-INDEX v4.167 delivery-summary and BC coverage**

S-19.02 row: version v1.17, input-hash 604f45d ✓. S-19.07 row: version v1.16, input-hash 534c85c ✓. BC coverage cell: `BC-4.13.001 v1.14 (S-19.02 Phase-A + S-19.07 Phase-B; F-P35-001/002 D-790)` ✓. Pass-35 note prepended to delivery-summary. BC-INDEX v3.88 ✓. VP-INDEX v2.55 UNCHANGED ✓. ARCH-INDEX v2.97 UNCHANGED ✓.

---

### A.2 — F-P36-001 MEDIUM Finding

**F-P36-001 MEDIUM [POLICY 5 v1.3.3/v1.3.5 sibling-sweep + S-7.01 partial-fix regression]**

**Location:** `STORY-INDEX v4.167` — wave-summary aggregation paragraph (the `> **E-19 delivery:** ...` blockquote after the S-19.07 row).

**Defect:** The wave-summary aggregation paragraph contains an `Input-hashes:` line that is a same-file aggregation cell duplicating the per-row input-hash values. At D-790, the per-row cells in the S-19.02 and S-19.07 table rows were correctly updated (S-19.02 row: v1.17 / 604f45d; S-19.07 row: v1.16 / 534c85c). However, the wave-summary aggregation `Input-hashes:` line was NOT swept — it still reads:

```
Input-hashes: S-19.01=d40bd21; S-19.02=d208e66; S-19.03=8d1225d; S-19.04=67eee80; S-19.05=9e54d68; S-19.06=998ac74; S-19.07=83e8cc4. All 7 distinct.
```

The stale values `S-19.02=d208e66` (current: `604f45d`) and `S-19.07=83e8cc4` (current: `534c85c`) are sourced from S-19.02 v1.15 and S-19.07 v1.14 respectively (two versions behind at time of discovery). Additionally, the trailing `(Pass-NN updates: ...)` chain lacks a Pass-35 clause, which is the most recently completed fix burst.

**Escape class:** Novel sub-route of META-33 (sibling-sweep-inside-policy-cure). The POLICY 5 v1.3.5 Part C sibling-sweep categories (a)–(h) enumerate 8 explicit sibling site types but do not include same-file aggregation cells that restate per-row values (delivery summaries, "All N distinct" attestations, wave-summary Input-hashes lines, index aggregation paragraphs). D-790 swept per-row cells correctly but missed the same-file aggregation cell because no category in (a)–(h) covers this type of site. This is the first confirmed instance of this META-33 sub-route.

**Fix:** STORY-INDEX v4.167 → v4.168:
- `S-19.02=d208e66` → `S-19.02=604f45d`
- `S-19.07=83e8cc4` → `S-19.07=534c85c`
- Prepend `(Pass-35 updates: S-19.02 v1.16→v1.17 d208e66→604f45d; S-19.07 v1.15→v1.16 83e8cc4→534c85c; BC-4.13.001 v1.13→v1.14 cite sweep.)` before `(Pass-34 updates:...)`.

**POLICY 5 v1.3.7 codification recommendation:** Extend categories (a)–(h) with category (i) covering same-file aggregation cells.

**CLOSED** — Fixed in this D-791 fix burst (state-manager; STORY-INDEX v4.167→v4.168 + policies.yaml v1.4.1→v1.4.2 POLICY 5 v1.3.7 category-(i) codification).

---

## Part B — POLICY Sweeps

### B.1 — POLICY 19 Sweep (ADR volatile-pin check; E-19 perimeter)

All E-19 perimeter BCs verified for POLICY 19 compliance:

| BC | §Traceability ADR Reference | Status |
|----|----------------------------|--------|
| BC-4.13.001 v1.14 | `ADR-025 §Decision 1/14/15 and Deliverable D18` | CLEAN ✓ |
| BC-1.17.001 v1.5 | `ADR-025 §Decision 15` (stable form) | CLEAN ✓ |
| BC-2.07.001 v1.4 | `ADR-025 §Decision 1` (stable form) | CLEAN ✓ |
| BC-2.02.011 v1.5 | No ADR pin (path-traversal defense) | CLEAN ✓ |
| BC-3.08.001 v1.19 | `ADR-026 §Decision 1/2/4` (stable form) | CLEAN ✓ |
| BC-5.42.001 v1.5 | No ADR pin | CLEAN ✓ |

Out-of-perimeter note: BC-5.40.001 + BC-6.23.001 carry `ADR-025 v1.2` volatile-pins (O-P35-001 Drift Item from D-790). Not re-reported — already tracked in STATE.md Drift Items. Target next maintenance sweep.

### B.2 — POLICY 4 Sweep (semantic anchoring)

BC-4.13.001 v1.14: §Traceability anchors existence-verified (Part A, Verification 1). §Description + §Traceability now consistent. CLEAN ✓.

### B.3 — POLICY 5 Sweep (anchor justification / SDK grounding)

All SDK Grounding Evidence sections in perimeter BCs verified structurally intact. STORY-INDEX v4.167 per-row cells correct; wave-summary cell — DEFECT (F-P36-001, finding above). CLEAN except F-P36-001 (CLOSED in this burst).

### B.4 — POLICY 6 Sweep (ARCH-INDEX subsystem names)

All E-19 stories reference correct subsystem IDs (SS-01, SS-02, SS-03, SS-04, SS-07, SS-09) per ARCH-INDEX v2.97. CLEAN ✓.

### B.5 — POLICY 7 Sweep (BC title verbatim parity)

BC-INDEX v3.88 BC-4.13.001 catalog row title cell verified verbatim against H1. CLEAN ✓.

### B.6 — POLICY 8 Sweep (atomic frontmatter→body propagation)

BC-4.13.001 v1.14 frontmatter `bcs:` array + body BC table + ACs + Token Budget all cite v1.14. CLEAN ✓.

### B.7 — POLICY 9 Sweep (VP anchor traceability)

BC-4.13.001 v1.14 §VP Anchors: VP-095 + VP-096 cited. VP-INDEX v2.55 back-reference ✓. CLEAN ✓.

### B.8 — POLICY 13 Sweep (no-unauthorized-hardening)

No hardening-scope changes in D-790 delta. CLEAN ✓.

### B.9 — POLICY 14 Sweep (5-leg quintuple parity)

BC-4.13.001 v1.14: all 5 POLICY 14 legs verified (version, Changelog, modified[], last_amended, upstream BC-INDEX row). S-19.02 v1.17 + S-19.07 v1.16: 5-leg parity verified. CLEAN ✓.

### B.10 — POLICY 15 Sweep (literal-shell evidence)

D-790 burst-log Dim-2 contains literal shell gates for D-446(a) 8-block presence and D-448(a) source-attestation. D-449(a) satisfied per D-790 attestation. CLEAN ✓.

### B.11 — POLICY 16 Sweep (global-max D-NNN)

D-790 = global max confirmed pre-burst. D-791 allocated here. CLEAN ✓.

### B.12 — POLICY 17 Sweep (RELEASED_BUNDLE_ONLY)

No bundle or release content in E-19 spec cascade. N/A ✓.

### B.13 — POLICY 18 Sweep (input-hash non-placeholder)

All E-19 perimeter stories: input-hashes non-placeholder (604f45d, 534c85c, d40bd21, 67eee80, 9e54d68, 998ac74, 8d1225d all verified non-placeholder). BC-4.13.001 v1.14: 58518e8 non-placeholder. CLEAN ✓ (STORY-INDEX wave-summary aggregation: DEFECT F-P36-001 CLOSED this burst).

---

## Sibling-sweep audit (categories a–h + failed same-file-aggregation site)

| Category | Site type | Status |
|----------|-----------|--------|
| (a) | extractor NOTES in story T-N task pseudocode | CLEAN (pass-36 no change) |
| (b) | code-comment line-pin examples | CLEAN |
| (c) | parallel BC body tables citing primary BC | CLEAN |
| (d) | Token Budget row descriptions | CLEAN |
| (e) | ADR architecture-anchor citations | CLEAN |
| (f) | Risk-Mitigation table cells | CLEAN |
| (g) | §Bidirectional Parity Audit Note version-cite cells | CLEAN |
| (h) | §LOCAL Adversary Cascade Plan prerequisite refs | CLEAN |
| **(i) FAILED** | **same-file aggregation cells duplicating per-row values** | **DEFECT — F-P36-001 (CLOSED D-791)** |

Category (i) gap: POLICY 5 v1.3.5 Part C enumerated (a)–(h) but did not enumerate same-file aggregation cells. D-790 swept per-row cells (categories covered by (a)–(h)) but missed the wave-summary `Input-hashes:` aggregation line. POLICY 5 v1.3.7 codification adds category (i).

---

## Trajectory note (passes 22–36)

Full trajectory: 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1 (passes 22–36).

Length-4 tail (passes 33–36): →4→1→2→1.

Novelty: MEDIUM. F-P36-001 reveals a novel META-33 sub-route not covered by any of categories (a)–(h). Same-file aggregation cells are structurally distinct from per-row cells (they summarize/restate rather than serve as primary source-of-truth). Codification recommendation: POLICY 5 v1.3.7 category (i).

Streak: 0/3 after pass-36 (NOT-CLEAN). Pass-37 required.
