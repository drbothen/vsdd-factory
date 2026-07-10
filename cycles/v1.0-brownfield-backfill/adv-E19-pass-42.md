# Adversarial Review — E-19 Pass 42 (no-delta confirming pass; perimeter = epic v1.22 + full E-19 suite at D-795 versions; streak 1/3)

## Header

| Field | Value |
|-------|-------|
| **Pass** | 42 |
| **Type** | No-delta confirming pass |
| **Verdict** | NOT-CLEAN B0/H0/M3/L0 |
| **Finding counts** | BLOCKER: 0; HIGH: 0; MEDIUM: 3; LOW: 0 |
| **Streak before** | 1/3 |
| **Streak after** | 0/3 (reset by 3 MEDIUM findings) |
| **Model family** | Claude Opus 4.7 |
| **Iron Law** | Fresh context; zero prior passes loaded |
| **Rubric** | policies.yaml v1.4.2 |
| **Perimeter** | Full E-19 suite at D-795 versions (ADR-025 v1.13 + ARCH-INDEX v2.98 + BC-INDEX v3.89 + full carry-forward); no delta — artifacts frozen at D-796 |
| **Date** | 2026-07-09 |

---

## Part A — Findings and Evidence

### A.1 — 23-Artifact Attestation Table

| # | Artifact | Version attested | Hash / Notes | Status |
|---|----------|-----------------|--------------|--------|
| 1 | ADR-025 | v1.13 | §Decision 14 stable anchor `§Precondition 3 (Phase-A) and §Invariant 9` | ✓ |
| 2 | ADR-030 | v1.3 | SubagentStop canonical TOML stanza | ✓ |
| 3 | BC-4.13.001 | v1.14 | §Traceability `and Deliverable D18`; §Description Decisions 14/15 + D18 | ✓ |
| 4 | BC-1.17.001 | v1.5 | domain_invariants: []; 03fa998 | ✓ |
| 5 | BC-2.07.001 | v1.4 | DI-TBD→none; 9d60fc5 | ✓ (version attested; FINDING below) |
| 6 | BC-2.02.011 | v1.5 | (D-780) | ✓ |
| 7 | BC-3.08.001 | v1.19 | Event 6 plugin.completed (D-756) | ✓ |
| 8 | BC-5.42.001 | v1.5 | DI-TBD→none; 4fd18a4 (D-787) | ✓ |
| 9 | VP-095.md | v1.1 | stable §Precondition 3 anchor; ce25941 | ✓ |
| 10 | VP-096.md | v1.1 | exclusive boundary form (D-782) | ✓ |
| 11 | VP-097.md | v1.0 | initial creation (E-19 VP package) | ✓ (version attested; FINDING below) |
| 12 | VP-098.md | v1.0 | initial creation (E-19 VP package) | ✓ |
| 13 | S-19.01 | v1.16 | d40bd21 | ✓ |
| 14 | S-19.02 | v1.17 | 604f45d | ✓ |
| 15 | S-19.03 | v1.16 | 8d1225d | ✓ |
| 16 | S-19.04 | v1.11 | (D-763) | ✓ |
| 17 | S-19.05 | v1.14 | 9e54d68 | ✓ |
| 18 | S-19.06 | v1.18 | 998ac74 (Token Budget Total ~22,000) | ✓ |
| 19 | S-19.07 | v1.16 | 534c85c | ✓ |
| 20 | epic (E-19) | v1.22 | EAC-003 BC-2.07.001 v1.3→v1.4; a18ea87 | ✓ |
| 21 | policies.yaml | v1.4.2 | POLICY 5 v1.3.7 category-(i) codified | ✓ |
| 22 | BC-INDEX | v3.89 | 3 title cells verbatim H1 (D-794 F-P39-001) | ✓ |
| 23 | ARCH-INDEX | v2.98 | ADR-025 row v1.13 annotation (D-795 F-P40-001) | ✓ |

**Attestation result: 23/23 version attestations confirmed.**

---

### A.2 — 15-Axis Re-derivation

15 axes examined; 12 PASS; 1 failing axis (F-P42-001 source); 1 failing axis (F-P42-002/003 combined).

| Axis # | Axis description | Result | Notes |
|--------|-----------------|--------|-------|
| 1 | AC gate executability — all gates in S-19.01..S-19.07 executable as written | PASS | No fictional binaries; all tools available |
| 2 | Tasks↔ACs coverage — every AC has at least one Task row that implements it | PASS | Coverage confirmed across 7 stories |
| 3 | Shared-file wave-conflict analysis — no two W1 stories write the same file | PASS | Wave 1 (S-19.01/02/03) file sets confirmed disjoint |
| 4 | Frontmatter schema uniformity — all E-19 story frontmatter fields present and typed | PASS | All 7 stories comply |
| 5 | DAG acyclicity — E-19 dependency graph has no cycles | PASS | W1→W2→W3 is acyclic; confirmed |
| 6 | VP body ↔ VP-INDEX title/description parity — VP-INDEX Full Index descriptions match VP file H1 | PASS | All 8 VP-094..VP-101 H1s match index |
| 7 | BC §VP Anchors ↔ VP-INDEX §Story Anchors cross-reference completeness | PASS for anchor existence; FAIL for BC body row content | **F-P42-002/003** — BC-2.07.001 §VP Properties VP-097 row and duplicate VP-098 row |
| 8 | VP source_bc field — all VP source_bc values cite current stable-anchor form | FAIL — VP-097 volatile pins found | **F-P42-001** — `BC-2.07.001 v1.0` + `BC-2.02.011 v1.4 EC-001` in source_bc + §Source Contract |
| 9 | POLICY 5 v1.3.3 sibling-sweep completeness — same volatile-pin class in sibling VPs | FAIL extension — siblings VP-094/098/100/101 carry volatile pins | **F-P42-001 sibling class** — D-784 VP-095 fix missed 5 siblings; a0c2c62a swept all |
| 10 | POLICY 5 v1.3.7 category-(i) aggregation cells in S-19.06 Token Budget | PASS | Total ~22,000; row-sum matches (D-792 fix confirmed stable) |
| 11 | BC-INDEX title cells verbatim H1 (POLICY 7) — for all 6 E-19 BCs | PASS | char-diff confirmed; D-794 fix stable |
| 12 | ADR body BC-cite sweep (D-795 L-BB-adr-body-bc-cites-are-sweep-sites) | PASS | ADR-025/ADR-030 zero live volatile pins in normative body |
| 13 | Story File-Structure ↔ inputs[] frontmatter alignment | PASS | All 7 stories confirmed |
| 14 | EAC gates executability — all epic EAC gates match actual story/BC structure | PASS with note | BC-2.07.001 §VP body content is the defect (F-P42-002/003), not EAC gate structure |
| 15 | BC §VP-table-row ↔ VP-body Property-Statement parity (NEW AXIS) | FAIL — BC-2.07.001 §VP table VP-097 row + duplicate VP-098 row | **F-P42-002/003** closed same-burst PO e4b1c8d9 |

**12 PASS; axis 8/9 fail → F-P42-001; axis 7/15 fail → F-P42-002/003. All 3 findings novel axes.**

---

### A.3 — F-P42-001: MEDIUM — VP-097 Stale Volatile BC-Version Pins (POLICY 5 v1.3.5 + v1.3.3 regression; D-784 VP-095 sibling-sweep miss)

**Severity:** MEDIUM  
**Policy violated:** POLICY 5 v1.3.5 (stale volatile BC-version-pin in normative body); POLICY 5 v1.3.3 (same-burst sibling sweep obligation — D-784 VP-095 fix burst missed VP-097 and siblings)  
**Classification:** Regression class — D-784 VP-095 fix burst introduced the stable-anchor discipline but did not sweep all same-epic VP files.

**Verbatim evidence (VP-097.md source_bc field before fix):**
```
source_bc: "BC-2.07.001 v1.0 + BC-2.02.011 v1.4 EC-001"
```

**Verbatim evidence (VP-097.md §Source Contract before fix):**
```
BC-2.07.001 v1.0 + BC-2.02.011 v1.4 EC-001
```

**Verbatim evidence (VP-097.md §Traceability before fix):**
```
BC-2.07.001 v1.0 + BC-2.02.011 v1.4 EC-001
```

**Why volatile:** BC-2.07.001 is at v1.4 at time of finding (was v1.0 at VP creation). BC-2.02.011 is at v1.5 at time of finding (was v1.4 at VP creation). Per TD-VSDD-091, normative body prose must cite stable-anchor form (`§Invariant 1`, `§EC-001`) not version-pinned form. Version-pinned cites fail POLICY 5 v1.3.6 HEAD-reproducibility when BC is amended.

**Ground-truth anchors:**
- BC-2.07.001 `§Invariant 1` — "Traversal defense preserved." exists at HEAD; EC-003 explicitly cross-references Invariant 1 for traversal defense.
- BC-2.02.011 `§EC-001` — "Path traversal attempt (e.g. `../../../etc/passwd`) | CAPABILITY_DENIED (-1)" exists at HEAD; EC-001 covers path-traversal-cannot-escape-path_allow.

**Sibling-sweep extension finding:** Adversary identifies the same volatile-pin class live in VP-094, VP-098, VP-100, VP-101:
- VP-094.md source_bc: `BC-5.42.001 v1.0` (volatile)
- VP-098.md source_bc: `BC-2.07.001 v1.0` (volatile)
- VP-100.md source_bc: `BC-3.08.001 v1.16` (volatile — BC-3.08.001 is at v1.19)
- VP-101.md source_bc: `BC-1.17.001 v1.0` (volatile — BC-1.17.001 is at v1.5)

Per POLICY 5 v1.3.3, these sibling-class defects MUST be swept same-burst as the primary finding.

**Resolution:**
- CLOSED at architect commit **47b87f6e**: VP-097 v1.0→v1.1 — `source_bc`, `§Source Contract`, `§Traceability` all migrated to stable anchor form (`BC-2.07.001 §Invariant 1` + `BC-2.02.011 §EC-001`). D-779 gate PASS (literal-shell `grep -nE 'BC-[0-9]+\.[0-9]+\.[0-9]+ v[0-9]+\.[0-9]+'` — only last_amended historical row match; zero live-body hits). Input-hash 784ee82 unchanged (hash of BC input files).
- SIBLING-SWEEP CLOSED at architect commit **a0c2c62a**: VP-094/098/100/101 v1.0→v1.1 — all four stable-anchor migrations executed same-burst per POLICY 5 v1.3.3. D-779 gate PASS on all 4 (literal-shell greps captured). Input-hashes updated: VP-094 4ab6a12→9eff742; VP-098 76d6259→0d7d3aa; VP-100 1072e05→a2de4e4; VP-101 4f41d79→2fe5a22. VP-095/096/099 sentinel-clean confirmed.

---

### A.4 — F-P42-002: MEDIUM — BC-2.07.001 §Verification Properties VP-097 Row Property Cell Mis-anchor (POLICY 9 + POLICY 4)

**Severity:** MEDIUM  
**Policy violated:** POLICY 9 (VP-INDEX is canonical source for VP property statements; BC §VP table must be consistent with VP-INDEX SoT); POLICY 4 (internal inconsistency within BC-2.07.001 itself — §VP Anchors already correctly described traversal defense; §Verification Properties VP-097 row described absent-path semantics instead)

**Verbatim evidence (BC-2.07.001 §Verification Properties VP-097 row, property cell, before fix):**
```
| VP-097 | Absent file at allowlisted path returns NOT_FOUND (-5) not CAPABILITY_DENIED (-1); internal.file_not_found event emitted | Kani proof | S-19.03 |
```
(paraphrased from adversary read; exact token: the VP-097 row described absent-file semantics, which is VP-098's scope, not traversal-defense, which is VP-097's actual scope)

**VP-INDEX canonical row (lines 457, source-of-truth):**
```
| [VP-097](VP-097.md) | path_util::resolve_path_for_allowlist Traversal Defense — .. Sequences Cannot Resolve Outside Allowlist Prefixes — allowlist-bounded path resolution; .. sequences cannot escape declared path_allow prefixes; path traversal rejected before capability check (BC-2.07.001 + BC-2.02.011 EC-001); Kani proof verifies traversal defense for all symbolic path inputs. | safety | kani-proof | SS-01 | — | draft |
```

**Internal inconsistency:** BC-2.07.001 §VP Anchors section correctly cited VP-097 as Kani traversal-defense proof. The §Verification Properties table row was out of sync with both §VP Anchors and VP-INDEX.

**Resolution:** CLOSED at product-owner commit **e4b1c8d9** — BC-2.07.001 v1.4→v1.5: §Verification Properties VP-097 row property cell corrected to traversal-defense framing per VP-INDEX SoT. Input-hash 9d60fc5→d31ddd5.

---

### A.5 — F-P42-003: MEDIUM — BC-2.07.001 §Verification Properties Duplicate VP-098 Rows (POLICY 9 + POLICY 4)

**Severity:** MEDIUM  
**Policy violated:** POLICY 9 (VP-INDEX canonical row for VP-098 defines it as integration postcondition; BC's second VP-098 row redefines scope as static grep-gate — contradicts canonical definition); POLICY 4 (two VP-098 rows in the same §Verification Properties table creates internal contradiction)

**Verbatim evidence (BC-2.07.001 §Verification Properties, duplicate VP-098 rows before fix):**
Row 1 (original, from VP-INDEX canonical definition):
```
| VP-098 | Allowlisted-but-absent file returns NOT_FOUND (-5) not CAPABILITY_DENIED (-1); integration test; zero false-positive capability_denied for allowlisted paths | integration | S-19.03 |
```
Row 2 (duplicate, added at pass-3 wiring, redefining scope):
```
| VP-098 | AC-003 grep gate: zero `capability_denied` events for allowlisted-absent-file path (static test prerequisite) | grep-gate | S-19.03 |
```

**VP-INDEX canonical row (line 458, source-of-truth):**
```
| [VP-098](VP-098.md) | Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives — allowlisted path that is absent returns NOT_FOUND (-5) not CAPABILITY_DENIED (-1); internal.file_not_found event emitted; callers can distinguish missing-file from policy-denied (BC-2.07.001). v1.0: initial creation (E-19 VP package). source_bc: BC-2.07.001. | postcondition | integration | SS-01 | — | draft |
```

**VP-INDEX H1 (line 458):** `Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives`

The second row incorrectly scoped VP-098 as a "static grep-gate" rather than the canonical "integration postcondition" definition. The AC-003 grep gate is a prerequisite testing step, not a separate VP scope.

**Resolution:** CLOSED at product-owner commit **e4b1c8d9** — BC-2.07.001 v1.4→v1.5: duplicate VP-098 row consolidated to single canonical-postcondition row matching VP-INDEX SoT; AC-003 grep gate noted as static prerequisite in single row, not separate VP scope. Input-hash 9d60fc5→d31ddd5.

---

### A.6 — Story-writer Propagation (964048de)

F-P42-002 + F-P42-003 required BC-2.07.001 v1.4→v1.5 cite sweep in S-19.03 and epic.

- **S-19.03 v1.16→v1.17** (story-writer 964048de): BC-2.07.001 v1.4→v1.5 cite sweep ×3 body sites (AC-001 negative-control B gate, BC table Version cell, Token Budget). Input-hash 8d1225d unchanged (VP-table-only amendment; EC-007 and all PCs/Invariants unchanged).
- **Epic v1.22→v1.23** (story-writer 964048de): BC-2.07.001 v1.4→v1.5 cite sweep at EAC-003 negative-control B (1 body site). Input-hash a18ea87→9a1ba40 (hash drift from BC-2.07.001.md content change).

---

## Part B — Per-Policy Attestations

### B.1 — POLICY 1 (Append-only IDs)
PASS. No IDs removed. All 7 S-19 stories and E-19 epic remain registered with continuous numbering.

### B.2 — POLICY 2 (Version monotonicity)
PASS. All amendments advance version. No downgrade observed.

### B.3 — POLICY 3 (Authoritative source)
PASS. All anchor references resolve. BC-2.07.001 §VP Anchors corrected to match VP-INDEX SoT via F-P42-002/003 fix.

### B.4 — POLICY 4 (Internal consistency)
PASS after fix. F-P42-002/003 found and closed — BC-2.07.001 §VP body now consistent with §VP Anchors and VP-INDEX.

### B.5 — POLICY 5 v1.3.5 (Stable-anchor BC-version-pin)
PASS after fix. F-P42-001 found and closed — VP-097 (47b87f6e) + VP-094/098/100/101 sibling sweep (a0c2c62a). All 5 VP files now carry stable anchor form. D-784 VP-095 sibling-sweep miss remediated.

### B.6 — POLICY 5 v1.3.3 (Same-burst sibling sweep)
PASS after fix. Sibling-sweep extension executed same-burst at a0c2c62a per POLICY 5 v1.3.3 obligation. All 5 same-epic VPs swept: VP-094/097/098/100/101. Sentinels VP-095/096/099 confirmed clean.

### B.7 — POLICY 5 v1.3.7 category-(i) aggregation cells
PASS. S-19.06 Token Budget Total ~22,000 stable (D-792 fix confirmed; row-sum parity maintained across v1.18).

### B.8 — POLICY 6 (Subsystem canonical names)
PASS. All subsystem references (SS-01/02/04/05/07) match ARCH-INDEX canonical forms.

### B.9 — POLICY 7 (BC-INDEX title-cell verbatim H1)
PASS. D-794 fix confirmed stable. All 6 E-19 BC title cells in BC-INDEX match verbatim H1. char-diff gate applicable to any future amendment.

### B.10 — POLICY 8 (POLICY 8 BC-table propagation)
PASS. BC-2.07.001 §VP body update (F-P42-002/003) is a §Verification Properties section change, not a new BC — no POLICY 8 story propagation required beyond the BC-version cite sweep (S-19.03 + epic — completed by story-writer 964048de).

### B.11 — POLICY 9 (VP-INDEX propagation completeness)
PASS after fix. F-P42-002/003 found and closed. BC-2.07.001 §Verification Properties VP-097 row and VP-098 row now consistent with VP-INDEX canonical definitions. No VP title changes in this pass — no POLICY 9 same-burst propagation beyond BC §VP body.

### B.12 — POLICY 14 (5-leg quintuple parity on index bumps)
PASS. All affected BC/VP/story/epic bumps applied with full 5-leg parity: (1) version: frontmatter (2) body Changelog (3) modified[] (4) last_amended: (5) upstream-index.

### B.13 — POLICY 15 (Traceability completeness)
PASS. VP-097/094/098/100/101 traceability sections updated to stable-anchor form via fix bursts.

### B.14 — POLICY 16 (Decision-log global-max gate)
PASS. D-797 allocated this pass. Sequential from D-796.

### B.15 — POLICY 17 (Epic frontmatter completeness)
PASS. Epic v1.23 maintains all required frontmatter fields. modified[] and last_amended maintained.

### B.16 — POLICY 18 (Input-hash completeness)
PASS. VP-094/098/100/101 input-hashes updated per a0c2c62a (recomputed via compute-input-hash --update). VP-097 input-hash 784ee82 unchanged (hash of BC input files; VP-097 body change does not alter the BC source hash). S-19.03 input-hash 8d1225d unchanged (VP-table-only; EC-007 and PCs/Invariants unchanged).

### B.17 — POLICY 19 (ADR body stable-anchor form)
PASS. ADR-025 v1.13 and ADR-030 v1.3 both carry zero live volatile pins in normative Decision body sections (D-795 gate confirmed stable).

### B.18 — POLICY 5 v1.3.6 (HEAD-reproducibility)
PASS after fix. VP-097 source_bc and body anchors now reproducible against BC-2.07.001 HEAD and BC-2.02.011 HEAD.

### B.19 — ADR body BC-cite sweep (D-795 enforcement gate)
PASS. ADR-025/ADR-030 normative sections: zero `BC-N.NN.NNN v[0-9]` hits outside amendment_reason/Changelog rows.

### B.20 — D-449(a) literal-shell gate obligation
PASS for fix bursts. D-779 gate (volatile-pin sweep) captured stdout included in architect commit messages (47b87f6e and a0c2c62a). State-manager burst-log Dim-2 must capture POLICY 16 gate and 4-index gate stdout per D-449(a).

---

## Sibling-Sweep Extension Record

**Class:** F-P42-001 volatile-pin sibling-sweep (VP source_bc + §Source Contract + §Traceability)

**Primary finding VP:** VP-097 (architect 47b87f6e)

**Sibling VPs swept same-burst (a0c2c62a):**

| VP | Old source_bc | New form | Input-hash change |
|----|--------------|----------|------------------|
| VP-094 | `BC-5.42.001 v1.0` | `BC-5.42.001 §Postcondition 1 + §Postcondition 2 + §Postcondition 3` | 4ab6a12→9eff742 |
| VP-098 | `BC-2.07.001 v1.0` | `BC-2.07.001 §Postcondition 2 + §Postcondition 3 + §Postcondition 4` | 76d6259→0d7d3aa |
| VP-100 | `BC-3.08.001 v1.16 Invariant 6a/6b` | `BC-3.08.001 §Invariant 6` (×6 sites) | 1072e05→a2de4e4 |
| VP-101 | `BC-1.17.001 v1.0` | `BC-1.17.001 §Postcondition 1 + §Postcondition 3 + §Postcondition 5` | 4f41d79→2fe5a22 |

**Sentinel VPs confirmed clean (report-only):** VP-095 (historical row only, exempt), VP-096 (historical row only, exempt), VP-099 (zero hits).

**D-779 gate captured stdout for all 5 VP files:** PASS (literal-shell `grep -nE 'BC-[0-9]+\.[0-9]+\.[0-9]+ v[0-9]+\.[0-9]+'` — only last_amended historical rows match; zero live-body hits on all 4 sibling VPs and VP-097).

---

## Trajectory Note

Passes 22–42 count trajectory: 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3.

Pass-42: NOT-CLEAN B0/H0/M3/L0 (3 total). Streak 1/3→0/3 (reset). All 3 findings from genuinely new axes (VP source_bc volatile-pin sibling class; BC §VP body ↔ VP-INDEX property-statement parity). All 3 CLOSED same-burst.

Zero BLOCKER: 20 consecutive passes (p22–p42). Zero HIGH: 7 consecutive passes (p36–p42 inclusive).

**Lesson codified:** L-BB-vp-source-contract-pins-are-sibling-class — when a stable-anchor cure lands on any VP's source_bc/§Source Contract, the sweep enumeration MUST include ALL same-epic VP files in the same burst (D-784 VP-095 fix missed 5 siblings; caught at pass-42). The a0c2c62a same-burst sweep is the enforcement precedent.
