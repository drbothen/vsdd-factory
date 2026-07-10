# Adversarial Review — E-19 Pass 52 (post-D-807 delta; perimeter = epic v1.26 + full E-19 suite at D-807 versions)

**Verdict:** NOT-CLEAN — B0/H0/M1/L0
**Streak:** 1/3 → 0/3
**Model:** Claude Opus 4.7
**Date:** 2026-07-10
**Rubric:** policies.yaml v1.4.3
**Iron Law:** SATISFIED — fresh context, no prior pass reports loaded

> **Operational note:** This pass was dispatched following D-807 GOVERNANCE-ONLY closure (pass-51 CLEAN; streak 0/3→1/3). The state-manager closure leg (D-808) was executed in a resumed session after prior session ran out of context. No adversary content or findings changed as a result of the mid-stream resume; adversary review was completed before the resume.

## Part A — Delta Verification + Fresh Axis Analysis

### D-807 Delta Verification

D-807 burst was GOVERNANCE-ONLY: pass-51 CLEAN B0/H0/M0/L0; 4-index ALL UNCHANGED BC v3.95/VP v2.59/STORY v4.175/ARCH v3.00; streak 0/3→1/3. No spec artifact changes to delta-verify. Perimeter carries forward at D-806 closure versions. Epic at v1.26.

**Gate — 4-index UNCHANGED verification:**
`grep "^version:" BC-INDEX.md VP-INDEX.md STORY-INDEX.md ARCH-INDEX.md` → v3.95 / v2.59 / v4.175 / v3.00. PASS.

### 14 Independent Fresh Axes

| Axis | Scope | Verdict |
|------|-------|---------|
| Pointer-class whole-perimeter | All 2 E-19 ADRs (ADR-025 v1.15 + ADR-030 v1.3) | PASS — 0 normative file:line pointers; ADR-025 line 1708 Changelog historical (exempt per TD-VSDD-091) |
| ADR-025 content-description registry parity | ADR-025 §Decision 1, §Decision 12, §12.6 | PASS — tool matcher `Edit\|Write\|MultiEdit\|Agent` matches hooks-registry.toml ground truth; `[hooks.capabilities.read_file]` block anchor matches registry lines 1260–1261 |
| ADR-030 6-field parity | ADR-030 v1.3 all 6 structured fields | PASS — no drift from D-777 anchor; UNCHANGED D-797..D-807 |
| POLICY 7 char-diff 6/6 | BC-INDEX title cells for all 6 E-19 BCs | PASS — BC-1.17.001/BC-2.07.001/BC-2.02.011/BC-3.08.001/BC-4.13.001/BC-5.42.001 H1↔index char-exact |
| modified[] monotonicity whole-perimeter | All 6 E-19 BCs + VP-094..101 | PASS — all modified[] arrays version-monotonic ascending; BC-2.02.011 v1.3→v1.4→v1.5→v1.6→v1.7 PASS |
| 30-artifact version table | All §Artifact Versions at D-807 Closure entries | PASS — all 30 versions match live file frontmatter at D-807 versions |
| POLICY 14 sweep (last-change artifacts) | epic v1.26 (last D-802 SW 71be7861) + ARCH-INDEX v3.00 (last D-806 SM) | PASS — both artifacts: 5-leg parity complete (version:/Changelog/modified[]/last_amended:/upstream-index) |
| POLICY 19 sweep | ADR-025 v1.15 + ADR-030 v1.3 normative sections | PASS — 0 volatile BC/ADR version pins in non-changelog sections; Changelog rows all historical-by-construction |
| VP-INDEX Full-Index parity | VP-INDEX v2.59 all E-19 VP entries (VP-094..VP-101) | PASS — Full Index versions match VP file frontmatter; Story Anchors suffixed F-P43-004a/b correct |
| S-19.04 spec-first ground-truth | S-19.04 v1.11 artifact-path-registry.yaml references | PASS — no normative references require registry file changes in current E-19 perimeter |
| **BC Traceability row description → story-body SoT parity** | **Epic §Behavioral Contract Traceability table — all 6 BC rows** | **FAIL — BC-2.02.011 row: Story column describes `host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound` — BC-2.07.001's behavioral semantic, not BC-2.02.011's. See F-P52-001.** |
| STORY-INDEX §Epic E-19 heading parity | STORY-INDEX §Epic E-19 H2 heading annotation vs epic frontmatter version | PASS — heading `draft, v1.26` matches epic frontmatter `version: "v1.26"` |
| Wave-summary input-hash aggregation parity | STORY-INDEX §E-19 delivery wave-summary input-hash values vs story file frontmatter | PASS — all 7 story input-hashes match live file frontmatter; no drift |
| ADR body BC-version-pin sweep | ADR-025 v1.15 + ADR-030 v1.3 non-changelog sections | PASS — 0 volatile `BC-N.NN.NNN vN.N` version pins in normative ADR sections |

### Findings

**F-P52-001 — MEDIUM [POLICY 4 semantic mis-anchor]**

**Location:** Epic E-19 (`E-19-post-rc22-operator-hardening.md`) §Behavioral Contract Traceability table, BC-2.02.011 row.

**Pre-fix evidence (epic v1.26, D-807 versions):**
```
| BC-2.02.011 | S-19.03 (host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound) |
```

**Defect:** The Story column for BC-2.02.011 describes `host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound` — this is verbatim BC-2.07.001's behavioral semantic, not BC-2.02.011's. BC-2.02.011 is a write-side / path-safety BC; its S-19.03 role per the story body SoT (S-19.03 v1.19 §Behavioral Contract) is: path traversal prevention via `resolve_path_for_allowlist` in `path_util.rs`; EC-001 traversal attack → `CAPABILITY_DENIED`. The mis-anchor creates a semantically false entry — an auditor reading the Traceability table would incorrectly conclude that BC-2.02.011 is about read-file absent-file handling, when it is actually about write-path traversal defense.

**Root cause:** Traceability row description was not derived from BC-2.02.011 or S-19.03 body SoT; it was copied from or conflated with BC-2.07.001's entry during epic authoring (both BCs are implemented in S-19.03, creating adjacent-row copy risk).

**CLOSED:** SW da5fba2f — BC-2.02.011 row rewritten from S-19.03 body SoT; post-fix row:
```
| BC-2.02.011 | S-19.03 (path traversal prevention via resolve_path_for_allowlist in path_util.rs; EC-001 traversal → CAPABILITY_DENIED) |
```

**Lesson:** L-BB-traceability-row-descriptions-must-derive-from-target-SoT [process-gap][codified D-808] — epic/story Traceability-table row DESCRIPTIONS must derive from the target artifact's H1/story-body SoT; POLICY 4 audit axis includes description-prose semantic parity, not only BC-ID/story-ID accuracy; detection predicate: per-row grep of described semantics against target BC file. Joins standing gate roster as 8th gate.

### 6-Row BC Traceability Class Audit (POLICY 4; all rows audited against story-body SoT)

| Row | BC | Pre-audit Story Column | SoT Check | Result |
|-----|----|------------------------|-----------|--------|
| 1 | BC-5.42.001 | S-19.01 (pr-manager READY verdict SHA pinning + merge-strategy guard) | S-19.01 §BC anchor: READY verdict SHA pinning + merge-strategy enforcement | PASS |
| 2 | BC-4.13.001 | S-19.02 (Phase-A: raised byte budget + frontmatter-only extraction) + S-19.07 (Phase-B: migrate verify-factory-lock to host::read_prefix; removes STATE_MD_MAX_BYTES + TooLarge/OutputTooLarge handling) | S-19.02 Phase-A + S-19.07 Phase-B descriptions match story body SoT | PASS |
| 3 | BC-2.07.001 | S-19.03 (host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound) | S-19.03 §BC-2.07.001 anchor: absent-file → NOT_FOUND/HostError::NotFound | PASS |
| 4 | **BC-2.02.011** | **S-19.03 (host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound)** | **S-19.03 §BC-2.02.011 anchor: resolve_path_for_allowlist + EC-001 traversal → CAPABILITY_DENIED. MISMATCH — BC-2.07.001 semantics copied.** | **FIXED (SW da5fba2f)** |
| 5 | BC-3.08.001 | S-19.05 (amended v1.21: Event 5 plugin.abandoned all 7 mandatory fields including type + timestamp + entry_index; Invariant 6 key extension; Event 6 plugin.completed async path...) | S-19.05 §BC-3.08.001 anchor: plugin.abandoned + plugin.completed schema-level defense | PASS |
| 6 | BC-1.17.001 | S-19.06 (new: host::read_prefix bounded partial read) | S-19.06 §BC-1.17.001 anchor: read_prefix bounded partial read | PASS |

5 PASS + 1 FIXED. No additional semantic mis-anchors found.

## Part B — Full Perimeter Attestation

### B.1 Perimeter Completeness (30 artifacts)

All 30 perimeter artifacts verified at D-807 closure versions (D-807 GOVERNANCE-ONLY; perimeter unchanged from D-806 closure). No out-of-scope changes detected.

| Artifact | Version at Pass-52 | Status |
|----------|-------------------|--------|
| ADR-025 | v1.15 | PASS |
| ADR-030 | v1.3 | PASS |
| BC-1.17.001 | v1.6 | PASS |
| BC-2.02.011 | v1.7 | PASS — F-P52-001 CLOSED SW da5fba2f (epic only) |
| BC-2.07.001 | v1.5 | PASS |
| BC-3.08.001 | v1.21 | PASS |
| BC-4.13.001 | v1.14 | PASS |
| BC-5.42.001 | v1.6 | PASS |
| VP-094 | v1.1 | PASS |
| VP-095 | v1.1 | PASS |
| VP-096 | v1.1 | PASS |
| VP-097 | v1.1 | PASS |
| VP-098 | v1.2 | PASS |
| VP-100 | v1.2 | PASS |
| VP-101 | v1.2 | PASS |
| S-19.01 | v1.17 | PASS |
| S-19.02 | v1.17 | PASS |
| S-19.03 | v1.19 | PASS |
| S-19.04 | v1.11 | PASS |
| S-19.05 | v1.16 | PASS |
| S-19.06 | v1.19 | PASS |
| S-19.07 | v1.16 | PASS |
| epic (E-19) | v1.26 (pre-fix); v1.27 (post-fix da5fba2f) | FIXED (F-P52-001) |
| policies.yaml | v1.4.3 | PASS |
| BC-INDEX | v3.95 | PASS |
| VP-INDEX | v2.59 | PASS |
| STORY-INDEX | v4.175 | PASS (heading parity v1.26 correct at pass-52 time) |
| ARCH-INDEX | v3.00 | PASS |
| verification-architecture.md | v1.8 | PASS |
| verification-coverage-matrix.md | v1.5 | PASS |

### B.2–B.23 Policy Attestations

| Policy | Status | Notes |
|--------|--------|-------|
| POLICY 1 (append-only BC catalog) | PASS | No BC changes this pass; BC-INDEX v3.95 UNCHANGED |
| POLICY 2 (BC versioning) | PASS | No BC version changes this pass |
| POLICY 3 (no tool bypass) | N/A | Governance-only burst |
| POLICY 4 (semantic anchoring) | FAIL → FIXED | F-P52-001 epic §BC Traceability BC-2.02.011 row semantic mis-anchor; CLOSED SW da5fba2f |
| POLICY 5 v1.3.8 (sibling-sweep) | PASS | Epic is sole artifact with BC Traceability table; 6-row class audit performed; no sibling escape |
| POLICY 6 (ARCH-INDEX canonical) | PASS | ARCH-INDEX v3.00 UNCHANGED this pass |
| POLICY 7 (BC H1 verbatim parity) | PASS | Char-diff 6/6 PASS |
| POLICY 8 (BC frontmatter atomicity) | PASS | No BC changes |
| POLICY 9 (VP-INDEX as SoT) | PASS | VP-INDEX v2.59 UNCHANGED; VP body↔index semantics verified |
| POLICY 10 (mandatory gating) | PASS | 7 standing gates operational; F-P52-001 finding triggers 8th gate codification |
| POLICY 11 (production-grade) | PASS | Epic Traceability table semantic accuracy is production-grade requirement |
| POLICY 12 (decision-log continuity) | PASS | D-807 max → D-808 allocated (POLICY 16 gate) |
| POLICY 13 (L-BB lessons carry-across) | PASS | All 7 standing gates operational; new [process-gap] lesson codified |
| POLICY 14 (5-leg parity) | PASS | Epic v1.27 (post-fix): 5-leg parity complete at D-808 SM closure |
| POLICY 15 (verification steps attestation) | PASS | Literal-shell evidence in burst-log Block 5 (D-808 burst-log Dim-2) |
| POLICY 16 (D-NNN global-max gate) | PASS | D-807 confirmed max → D-808 allocated |
| POLICY 17 (last_amended chain form) | PASS | Epic last_amended updated at D-808 SM closure (SW da5fba2f + SM this-commit chain) |
| POLICY 18 (input-hash mechanically computed) | PASS | Epic input-hash fb55113 UNCHANGED per orchestrator verification (path_util content unchanged; SW da5fba2f traceability-table-only amendment) |
| POLICY 19 (stable anchor form) | PASS | 0 volatile BC/ADR pins in normative sections |
| DTU | N/A | Not applicable |
| Multi-repo | N/A | Not applicable |
| Formal-verify | N/A | Not applicable |

### 8-Gate Roster (7 OPERATIONAL + 1 NEW CODIFIED)

1. 4-index literal-shell gate (D-494) — PASS: BC v3.95 / VP v2.59 / STORY v4.175 / ARCH v3.00 UNCHANGED D-807
2. Heading-parity gate (D-803) — PASS: STORY-INDEX §Epic E-19 heading `draft, v1.26` matches epic frontmatter v1.26
3. Pointer-class gate (D-806) — PASS: 0 normative hits; 1 exempt (ADR-025 line 1708 Changelog historical-by-construction)
4. ADR body BC-cite sweep (D-795) — PASS: 0 volatile BC-version pins in normative ADR sections
5. ADR body content-description sweep (D-805) — PASS: tool matcher descriptions match live registry ground truth
6. POLICY 7 char-diff gate (D-794) — PASS: 6/6 BC title cells char-exact
7. D-779 whole-file predicate gate — PASS (N/A — no cite sweeps this burst; no residual stale tokens)
8. **BC Traceability row description parity gate (D-808 NEW)** — **FAIL → FIXED**: per-row grep of described semantics against target BC/story-body SoT; BC-2.02.011 row FIXED SW da5fba2f; remaining 5 rows PASS

### Zero-Finding Attestation Evidence (post-closure)

**Pointer-class whole-perimeter grep (pre-fix state):**
```
grep -nE 'line [0-9]+([–-][0-9]+)? of|at line [0-9]+' \
  .factory/specs/architecture/decisions/ADR-025-*.md \
  .factory/specs/architecture/decisions/ADR-030-*.md
```
→ ADR-025 line 1708: EXEMPT — Changelog v1.15 bullet (historical-by-construction per TD-VSDD-091)
→ ADR-030: 0 hits
→ 0 normative-live hits.

**6-BC char-diff strings (POLICY 7 gate):**
- BC-1.17.001 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓
- BC-2.07.001 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓
- BC-2.02.011 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓
- BC-3.08.001 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓
- BC-4.13.001 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓
- BC-5.42.001 H1 verbatim ↔ BC-INDEX title cell: char-exact ✓

**BC Traceability row class audit evidence (F-P52-001):**
Pre-fix BC-2.02.011 row in epic v1.26: `S-19.03 (host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound)` — matches BC-2.07.001's behavioral semantic. S-19.03 body SoT for BC-2.02.011: `resolve_path_for_allowlist` / `EC-001` / `CAPABILITY_DENIED`. Semantic mismatch confirmed. CLOSED SW da5fba2f.

### Trajectory Note

Passes 22–52 trajectory: 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2→3→1→1→0→1→1→0→1. Pass-52 NOT-CLEAN (1 MEDIUM finding F-P52-001). Trajectory tail passes 49/50/51/52 = 1,1,0,1 → `→1→1→0→1`. Streak 1/3→0/3 (reset). Three consecutive CLEANs required for 3/3 convergence.

### Novelty

**[process-gap] — Traceability-table row DESCRIPTIONS are a POLICY 4 audit axis.** Prior standing gates covered BC-ID accuracy and story-ID accuracy in Traceability tables, but not the SEMANTIC correctness of the row's description prose. With two BCs mapped to the same story (BC-2.07.001 and BC-2.02.011 both implemented in S-19.03), the adjacent-row copy risk is elevated. Detection predicate: for each Traceability row, grep the described semantics (key behavioral phrases) against the target BC file and target story body — expect semantic alignment. **Recommendation: add per-row description parity check as the 8th standing gate** (joins the 7 existing gates; dispatched to state-manager as D-808 codification obligation).

### Iron Law Compliance

Confirmed. Fresh context for pass-52. Prior pass reports (adv-E19-pass-51.md and earlier) NOT loaded. Rubric policies.yaml v1.4.3 applied. Zero cross-contamination from prior passes.
