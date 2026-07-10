# Adversarial Review — E-19 Pass 40 (post-D-794 delta; perimeter = epic v1.22 + full E-19 suite at D-794 versions)

**Perimeter:** BC-INDEX v3.89 + ADR-025 v1.12 + ADR-030 v1.3 + epic v1.22 + S-19.01 v1.16 / S-19.02 v1.17 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.18 / S-19.07 v1.16 + STORY-INDEX v4.169 + VP-INDEX v2.55 + BC-4.13.001 v1.14 + BC-5.42.001 v1.5 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ARCH-INDEX v2.97 + policies.yaml v1.4.2

**Reviewer:** fresh-context adversary; Iron Law; rubric policies.yaml v1.4.2

**Date:** 2026-07-09

**Verdict:** NOT-CLEAN — B0/H0/M1/L0 (1 finding, 0 observations)

**Streak:** 0/3 (pass-40 NOT-CLEAN; streak does not advance)

**Model family:** Claude Opus 4.7

---

## Part A — Version Attestations + D-794 Delta Verification + Finding

### A.1 — Version + Input-Hash Perimeter Attestation (23 artifacts)

All perimeter artifact versions attested at D-794 levels (BC-INDEX v3.89 + full E-19 suite carry-forward at D-792 versions):

| Artifact | Version | Input-hash / Notes |
|----------|---------|-------------------|
| BC-INDEX | v3.89 | — D-794 fix burst (3 title cells corrected to verbatim H1); PASS ✓ |
| ADR-025 | v1.12 | — PASS ✓ (D-787; D-788..D-794 UNCHANGED) |
| ADR-030 | v1.3 | — PASS ✓ (D-777; D-778..D-794 UNCHANGED) |
| epic (E-19) | v1.22 | a18ea87 — PASS ✓ |
| S-19.01 | v1.16 | d40bd21 — PASS ✓ |
| S-19.02 | v1.17 | 604f45d — PASS ✓ |
| S-19.03 | v1.16 | 8d1225d — PASS ✓ |
| S-19.04 | v1.11 | 67eee80 — PASS ✓ |
| S-19.05 | v1.14 | 9e54d68 — PASS ✓ |
| S-19.06 | v1.18 | 998ac74 — PASS ✓ |
| S-19.07 | v1.16 | 534c85c — PASS ✓ |
| STORY-INDEX | v4.169 | — PASS ✓ |
| VP-INDEX | v2.55 | — PASS ✓ |
| BC-4.13.001 | v1.14 | 58518e8 — PASS ✓ |
| BC-5.42.001 | v1.5 | 4fd18a4 — PASS ✓ |
| BC-2.07.001 | v1.4 | 9d60fc5 — PASS ✓ |
| BC-2.02.011 | v1.5 | — PASS ✓ |
| BC-3.08.001 | v1.19 | — PASS ✓ |
| BC-1.17.001 | v1.5 | 03fa998 — PASS ✓ |
| VP-095.md | v1.1 | ce25941 — PASS ✓ |
| VP-096.md | v1.1 | — PASS ✓ |
| ARCH-INDEX | v2.97 | — PASS ✓ |
| policies.yaml | v1.4.2 | — PASS ✓ |

All 23 perimeter artifact versions attested ✓.

### A.2 — D-794 Delta Verification: BC-INDEX v3.89 6-BC Title-Cell Byte-Verbatim Table

D-794 fix burst corrected 3 of 6 E-19 BC title cells in BC-INDEX. Fresh-context adversary performed byte-exact H1 extraction and comparison against BC-INDEX v3.89 for all 6 E-19 BCs.

| BC ID | BC File H1 (after prefix) | BC-INDEX v3.89 Title Cell | Match? |
|-------|--------------------------|--------------------------|--------|
| BC-1.17.001 | `host::read_prefix — bounded partial read (head-c semantics), NEVER OUTPUT_TOO_LARGE, additive FFI entry point, path_allow + rejoin capability model` | (same) | **VERBATIM ✓** (FIXED D-794) |
| BC-2.07.001 | `host::read_file absent-file semantics — codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied` | (same) | **VERBATIM ✓** (FIXED D-794) |
| BC-5.42.001 | `pr-manager READY-verdict + merge-strategy enforcement — covered_sha pin, stale-verdict detection, and release-PR squash prevention` | (same) | **VERBATIM ✓** (FIXED D-794) |
| BC-4.13.001 | (unchanged) | (unchanged) | **VERBATIM ✓** |
| BC-3.08.001 | (unchanged; mid-title em-dash preserved) | (same) | **VERBATIM ✓** |
| BC-2.02.011 | (unchanged) | (same) | **VERBATIM ✓** |

**Summary:** 6/6 E-19 BCs VERBATIM ✓. F-P39-001 CLOSED — D-794 fix is load-bearing and correct. Control (BC-3.08.001 em-dash) preserved.

### A.3 — F-P40-001: MEDIUM — POLICY 5 v1.3.5 Part A + v1.3.6 HEAD-Reproducibility; ADR-025 v1.12 §Decision 14 Body Stale BC-Version-Token

**Finding ID:** F-P40-001
**Severity:** MEDIUM
**Policy:** POLICY 5 v1.3.5 Part A (spec prose carries stale version token in normative body — BC-version-pin form `BC-4.13.001 v1.4`); POLICY 5 v1.3.6 HEAD-reproducibility (anchor references a specific version, not stable behavioral section)
**Perimeter artifact:** ADR-025 v1.12

**Evidence (verbatim from ADR-025 v1.12 §Decision 14 body):**

```
Normative twin: BC-4.13.001 v1.4 Precondition 3 and Invariant 9. Any STATE.md structural
```

This line appears in ADR-025 §Decision 14 ("verify-factory-lock STATE_MD_MAX_BYTES 65536→262144 + frontmatter-only parse"). The token `BC-4.13.001 v1.4` is a load-bearing version-pinned cite.

**Why this is a defect:**

1. **Stale version pin (POLICY 5 v1.3.5 Part A):** BC-4.13.001 is at v1.14 at D-794 HEAD. The `v1.4` pin is 10 versions behind. Version-pinned BC cites in normative ADR bodies violate POLICY 5 because they become misleading when the BC evolves — a reader following the pin would consult the wrong version.

2. **Semantic staleness:** Three specifically relevant amendments occurred after v1.4:
   - v1.6 (D-755): Phase-A / Phase-B split introduced; Precondition 3 (Phase-A) and Invariant 9 diverge in meaning from their v1.4 forms.
   - v1.7 (D-761): §Precondition 3 Phase-A path_allow semantics updated (`[".factory/STATE.md"]`).
   - v1.10 (D-781): §Invariant 9 exclusive-boundary `0..delimiter_start_offset` added.
   A reader following `v1.4 Precondition 3 and Invariant 9` would miss all three of these behavioral refinements.

3. **HEAD-reproducibility violation (POLICY 5 v1.3.6):** The cite is not reproducible at HEAD — `BC-4.13.001 v1.4 Precondition 3` no longer has the same behavioral meaning as current `BC-4.13.001 §Precondition 3 (Phase-A)`.

4. **Sole non-historical site:** This is the only non-changelog/non-amendment_reason `BC-4.13.001 v[0-9]` token in the ADR-025 body. Sibling §Decision 15 cites BC-4.13.001 §Precondition 3 Phase-B in stable anchor form (no version pin); §Decision 14 is the sole divergent case.

5. **Novel axis:** This is the first confirmed **reverse-direction ADR→BC** stale cite class in the E-19 cascade. All prior POLICY 5/19 stale-cite findings (F-P34-001 through F-P37-001) were BC→ADR or story→BC in direction. An ADR body citing a BC with a version pin is a distinct escape route not previously enumerated in the per-burst BC-cite preflight sweep enumeration (which listed story files, epic, BC §Traceability rows, VP-INDEX rows — but not ADR body prose).

**Routing:** architect (ADR body content; no BC file content changes required; stable anchor form matches sibling §Decision 15 pattern).

**Proposed fix:** Replace `BC-4.13.001 v1.4 Precondition 3 and Invariant 9` with stable anchor form `BC-4.13.001 §Precondition 3 (Phase-A) and §Invariant 9` (matching sibling §Decision 15 cite style).

**CLOSED:** Fixed in ADR-025 v1.13 by architect commit `7a58f292` (stable-anchor form; §Decision 14 Normative-twin line corrected). Ground-truth anchors verified against BC-4.13.001 v1.14 at HEAD: `§Precondition 3 (Phase-A)` exists as a named behavioral anchor; `§Invariant 9` exists as a named invariant. ADR sibling-sweep: zero non-historical `BC-4.13.001 v[0-9]` residuals in ADR-025 body after fix. ADR-030 zero-match sentinel: no BC-4.13.001 version-token in ADR-030.

**Enforcement note (adversary, not a formal process-gap):** POLICY 5 v1.3.5 Part A already covers ADR→BC body cites in principle; the gap was enforcement discipline — fix bursts touching E-19 BCs have included story-file sweeps, epic sweeps, VP-INDEX sweeps, and BC §Traceability ADR Reference cell sweeps (F-P34-001 class), but ADR body prose has not been enumerated as a sweep site. Record as lesson note `L-BB-adr-body-bc-cites-are-sweep-sites` [codified D-795] in lessons.md; no policy amendment required.

---

## Part B — Policy Attestations + Perimeter Integrity

### B.1 — POLICY 7 BC-INDEX Title-Cell Verbatim-Parity (6/6 CLEAN)

D-794 fix resolved all 3 drifted title cells. Per A.2 byte-exact table above: all 6 of 6 E-19 BCs have verbatim-parity between BC file H1 and BC-INDEX v3.89 title cells. Gate (D-794 codification): `grep -cF "$H1" BC-INDEX.md` would return ≥1 for each. No further drift found. **PASS ✓**

### B.2 — POLICY 14 BC-INDEX v3.89 Parity (6/6 E-19 BCs indexed at correct versions)

BC-INDEX v3.89 catalog rows for all 6 E-19 BCs cite their current versions:

| BC ID | Current Version | BC-INDEX v3.89 Row Version | PASS? |
|-------|----------------|---------------------------|-------|
| BC-1.17.001 | v1.5 | v1.5 | ✓ |
| BC-2.07.001 | v1.4 | v1.4 | ✓ |
| BC-2.02.011 | v1.5 | v1.5 | ✓ |
| BC-3.08.001 | v1.19 | v1.19 | ✓ |
| BC-4.13.001 | v1.14 | v1.14 | ✓ |
| BC-5.42.001 | v1.5 | v1.5 | ✓ |

**PASS ✓ (6/6)**

### B.3 — Category-(i) 14-Cell Aggregation Sweep (POLICY 5 v1.3.7; 14/14 CLEAN)

All 14 category-(i) aggregation cells at D-794 versions verified CLEAN per policies.yaml v1.4.2 POLICY 5 v1.3.7. No new aggregation-cell drift. S-19.06 Token Budget Total reads `~22,000` (D-792 fix). Wave-summary Input-hash parity verified at D-791 levels (S-19.02=604f45d, S-19.07=534c85c). No regressions found. **PASS ✓ (14/14)**

### B.4 — POLICY 8 BC Propagation + 7-Story Coherence Check

All 7 stories (S-19.01..S-19.07) reference their governing BCs at current versions. EC-mirror checks for all 6 E-19 BCs confirm ACs ↔ BC invariants/ECs consistently enumerated. No stale cite found across 7 stories.

Note on S-19.07 Token Budget context-anchor pattern: S-19.07 §Token Budget cites `BC-1.17.001` as a context anchor for the `read_prefix` capability scope rather than citing BC-1.17.001 in a `behavioral_contracts:` frontmatter dependency form. This cite pattern is adjudicated **legitimate** — BC-1.17.001 governs the `read_prefix` host function that S-19.07 consumes; the reference is architecturally accurate and provides necessary context without creating a false frontmatter dependency (S-19.07 is gated on S-19.02 and S-19.06, both of which carry the BC-1.17.001 formal dependency chain). **PASS ✓**

### B.5 — POLICY 9 VP Arithmetic (101 total — both breakdowns reconciled)

**Summary table breakdown** (17 categories): 17+10+10+5+10+5+3+2+2+4+4+4+2+1+1+13+8 = **101** ✓

**Proof method breakdown**: unit-test 46 + integration 34 + manual 10 + static-check 1 + kani-proof 5 + proptest 5 = **101** ✓

Both breakdowns sum to 101. VP-INDEX `total_vps: 101` matches both. POLICY 9 parity: all 8 E-19 VPs (VP-094..VP-101) correctly allocated in Story Anchors section. **PASS ✓**

### B.6 — ADR-030 §Decision 1 Canonical TOML 6-Field Byte-Match (D-777 Gate Re-Verified)

ADR-030 v1.3 §Decision 1 canonical registry stanza specifies 6 named fields:
`name = "pr-manager-completion-guard"` / `event = "SubagentStop"` / `plugin = "hook-plugins/pr-manager-completion-guard.wasm"` / `priority = 920` / `timeout_ms = 5000` / `on_error = "continue"`. Per D-777 gate, live `hooks-registry.toml` entry must byte-match these 6 fields. All 6 fields confirmed present and correct in ADR-030 v1.3; no residual `on_error = "advisory"` / `priority = 150` / `tool =` / `tier =` fields (removed at v1.3 per F-P23-001). **PASS ✓**

### B.7 — ADR-025 Ground Truth (15 Decisions + D1..D18; §Decision 14 Stable Anchor at v1.13)

ADR-025 v1.13 (post-F-P40-001 fix by architect 7a58f292):
- Decision headers count: 15 (grep confirms `^### Decision [0-9]` = 15 matches)
- Concrete Deliverables: D1..D18 (table runs from D1 to D18; D13 is State-manager.md amendment; D15 factory-lock-parse; D16 verify-state-timestamp-refresh; D17 Rust tests; D18 host::read_prefix)
- §Decision 14 Normative-twin now reads: `BC-4.13.001 §Precondition 3 (Phase-A) and §Invariant 9` (stable anchor form; F-P40-001 CLOSED)
- §Decision 15 cite: `BC-4.13.001 §Precondition 3 Phase-B` (stable anchor form; no version pin; sibling consistency confirmed)
- Zero non-historical `BC-4.13.001 v[0-9]` tokens in ADR body proper (amendment_reason + Changelog entries are historical and exempt per TD-VSDD-091)

**PASS ✓**

### B.8 — POLICY 19 Sentinel Check (6/6 Stable-Anchor Form in E-19 Perimeter)

All 6 E-19 BCs, 8 E-19 VPs, and 7 E-19 stories checked for POLICY 19 volatile version-pin violations (pattern: `BC-N.NN.NNN v[0-9]+\.[0-9]+` in normative prose). No new volatile pins found in the E-19 perimeter. ADR-025 §Decision 14 volatile pin (F-P40-001) was in the ADR, not a BC or story; CLOSED at architect leg. BC-4.13.001 §Traceability cites stable §Decision form (D-789 fix). **PASS ✓ (6/6)**

### B.9 — Phase-A / Phase-B Coherence (4-way consistency)

Phase-A (read_file, STATE_MD_MAX_BYTES=262144, host::read_file+cap, S-19.02) / Phase-B (read_prefix, max_bytes=8192, host::read_prefix+cap, S-19.07) coherence verified across:
1. BC-4.13.001 v1.14 §Precondition 3 (Phase-A/B split) ✓
2. ADR-025 §Decision 14 (Phase-A, 262144, read_file) ✓
3. ADR-025 §Decision 15 (Phase-B, 8192, read_prefix per §Precondition 3 Phase-B) ✓
4. S-19.07 (Phase-B delivery, depends_on=[S-19.02, S-19.06]) ✓

No 4-way coherence violations found. **PASS ✓**

### B.10 — DAG / Wave Partition Check

E-19 epic DAG: W1 = {S-19.01, S-19.02, S-19.03} (parallel-eligible); W2 = {S-19.04, S-19.05, S-19.06} (gated on W1); W3 = {S-19.07} (gated on W2). Dependency edges: S-19.02→S-19.07, S-19.06→S-19.07 (W3 gate confirmed). Mermaid diagram isolated nodes: none (S-19.01 connected via `check-stale-verdict.sh` dependency; S-19.05 connected via telemetry sink chain). No phantom edges. **PASS ✓**

### B.11 — EAC ↔ AC Coverage Map (epic v1.22 §OutOfScope Cites Current)

Epic v1.22 §Out of Scope: BC-1.17.001 referenced as "LANDED as v1.5" (D-785 correction; D-794 UNCHANGED). All 3 EAC entries (EAC-001, EAC-002, EAC-003) correctly reference current BC versions. EAC-003 cites BC-2.07.001 v1.4 (D-787 correction; current ✓). **PASS ✓**

### B.12 — Trajectory Note

Pass-40 result: NOT-CLEAN B0/H0/M1/L0 (1 finding F-P40-001 MEDIUM; CLOSED at architect 7a58f292 before this state-manager burst).

Trajectory (passes 22–40): 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1.

Zero BLOCKER: 19 consecutive passes (p22–p40). Zero HIGH: 5 consecutive passes (p36–p40 inclusive). Streak: 0/3 (pass-40 NOT-CLEAN; streak does not advance from 0). Three consecutive CLEANs still required for convergence.

Root-cause of F-P40-001: reverse-direction ADR→BC stale cite axis not previously enumerated in fix-burst BC-cite preflight sweep sites. Fix-bursts touching E-19 BCs should include ADR body prose audits in the sweep enumeration (lesson `L-BB-adr-body-bc-cites-are-sweep-sites` [codified D-795]).

NEXT: E-19 adversary pass-41 (fresh context; Iron Law; rubric policies.yaml v1.4.2; perimeter = D-795 delta: ADR-025 v1.13 + ARCH-INDEX v2.98 + full E-19 suite carry-forward; streak 0/3; three consecutive CLEANs required).
