# Adversarial Review — E-19 Pass 53 (post-D-808 delta; perimeter = D-808 delta: epic v1.27 + STORY-INDEX v4.176 + full E-19 suite at D-808 versions)

**Verdict:** NOT-CLEAN — B0/H0/M2/L0
**Streak:** 0/3 (unchanged — pass-52 already reset to 0/3; pass-53 NOT-CLEAN does not increment)
**Model:** Claude Opus 4.7
**Date:** 2026-07-10
**Rubric:** policies.yaml v1.4.3
**Iron Law:** SATISFIED — fresh context, no prior pass reports loaded

## Part A — Delta Verification + Fresh Axis Analysis

### D-808 Delta Verification

D-808 burst fixed F-P52-001 MEDIUM: epic §BC Traceability BC-2.02.011 row rewritten from S-19.03 body SoT (path traversal prevention via `resolve_path_for_allowlist`; EC-001 → CAPABILITY_DENIED). STORY-INDEX v4.175→v4.176 (§Epic E-19 H2 heading + delivery-summary clause). 8th standing gate codified.

**Gate — epic v1.27 BC-2.02.011 row semantic parity vs S-19.03 SoT:**

| Row | BC | Post-fix Story Column | S-19.03 SoT Check | Result |
|-----|----|----------------------|-------------------|--------|
| 1 | BC-5.42.001 | S-19.01 (pr-manager READY verdict SHA pinning + merge-strategy guard) | S-19.01 §BC anchor: READY verdict SHA pinning + merge-strategy enforcement | PASS |
| 2 | BC-4.13.001 | S-19.02 (Phase-A: raised byte budget + frontmatter-only extraction) + S-19.07 (Phase-B: migrate verify-factory-lock to host::read_prefix; removes STATE_MD_MAX_BYTES + TooLarge/OutputTooLarge handling) | S-19.02 Phase-A + S-19.07 Phase-B descriptions match story body SoT | PASS |
| 3 | BC-2.07.001 | S-19.03 (host::read_file absent-file semantics: codes::NOT_FOUND + HostError::NotFound) | S-19.03 §BC-2.07.001 anchor: absent-file → NOT_FOUND/HostError::NotFound | PASS |
| 4 | BC-2.02.011 | S-19.03 (path traversal prevention via resolve_path_for_allowlist in path_util.rs; EC-001 traversal → CAPABILITY_DENIED) | S-19.03 §BC-2.02.011 anchor: resolve_path_for_allowlist + EC-001 traversal → CAPABILITY_DENIED. Semantic match confirmed. | PASS (post-D-808 fix) |
| 5 | BC-3.08.001 | S-19.05 (amended v1.21: Event 5 plugin.abandoned all 7 mandatory fields including type + timestamp + entry_index; Invariant 6 key extension; Event 6 plugin.completed async path...) | S-19.05 §BC-3.08.001 anchor: plugin.abandoned + plugin.completed schema-level defense | PASS |
| 6 | BC-1.17.001 | S-19.06 (new: host::read_prefix bounded partial read) | S-19.06 §BC-1.17.001 anchor: read_prefix bounded partial read | PASS |

6/6 PASS. F-P52-001 closure confirmed correct.

**Gate — STORY-INDEX v4.176 heading parity:**
`grep "## Epic.*E-19" STORY-INDEX.md` → `## Epic E-19 (E-19-post-rc22-operator-hardening; draft, v1.27)`. Matches epic frontmatter `version: "v1.27"`. PASS. Delivery-summary `(Pass-52 updates: ...)` clause present. PASS.

### 14 Independent Fresh Axes

| Axis | Scope | Verdict |
|------|-------|---------|
| Pointer-class whole-perimeter | All 2 E-19 ADRs (ADR-025 v1.15 + ADR-030 v1.3) | PASS — 0 normative file:line pointers; ADR-025 line 1708 Changelog historical (exempt per TD-VSDD-091) |
| ADR-025 content-description registry parity | ADR-025 §Decision 1, §Decision 12, §12.6 | PASS — tool matcher `Edit\|Write\|MultiEdit\|Agent` matches hooks-registry.toml ground truth; `[hooks.capabilities.read_file]` block anchor stable |
| ADR-030 6-field parity | ADR-030 v1.3 all 6 structured fields | PASS — no drift from D-777 anchor; UNCHANGED D-797..D-808 |
| POLICY 7 char-diff 6/6 | BC-INDEX title cells for all 6 E-19 BCs | PASS — BC-1.17.001/BC-2.07.001/BC-2.02.011/BC-3.08.001/BC-4.13.001/BC-5.42.001 H1↔index char-exact |
| modified[] monotonicity whole-perimeter | All 6 E-19 BCs + VP-094..101 | PASS — all modified[] arrays version-monotonic ascending |
| 30-artifact version table | All §Artifact Versions at D-808 Closure entries | PASS — 28 of 30 versions match live file frontmatter; VP-097 v1.1 + VP-101 v1.2 are the FAIL rows (see findings) |
| POLICY 14 sweep (last-change artifacts) | epic v1.27 (D-808 SW da5fba2f) + STORY-INDEX v4.176 (D-808 SM) | PASS — both artifacts: 5-leg parity complete |
| POLICY 19 sweep | ADR-025 v1.15 + ADR-030 v1.3 normative sections | PASS — 0 volatile BC/ADR version pins in non-changelog sections |
| VP-INDEX Full-Index parity | VP-INDEX v2.59 all E-19 VP entries (VP-094..VP-101) | PASS — Full Index versions match VP file frontmatter at D-808 (VP-097 v1.1; VP-101 v1.2; both pre-fix — VP-INDEX consistency confirmed at D-808 state) |
| S-19.04 spec-first ground-truth | S-19.04 v1.11 artifact-path-registry.yaml references | PASS — no normative references require registry file changes in current E-19 perimeter |
| **BC Traceability row description → story-body SoT parity (D-808 8th gate)** | **Epic §BC Traceability table — all 6 BC rows (post-D-808 fix)** | **PASS — 6/6 rows confirmed correct per D-808 delta verification above** |
| STORY-INDEX §Epic E-19 heading parity | STORY-INDEX v4.176 §Epic E-19 H2 heading annotation vs epic frontmatter | PASS — heading `draft, v1.27` matches epic frontmatter `version: "v1.27"` |
| Wave-summary input-hash aggregation parity | STORY-INDEX §E-19 delivery wave-summary input-hash values vs story file frontmatter | PASS — all 7 story input-hashes match live file frontmatter; no drift |
| **VP frontmatter module: + §Feasibility Artifact + §Traceability Function-anchor vs SoT (adjacent-axis: description-bearing anchor-prose)** | **All 8 E-19 VP files (VP-094..VP-101) — path-anchor fields as description-bearing anchor prose per POLICY 4** | **FAIL — VP-097 v1.1: 3 stale path anchors citing src/path_util.rs (missing host/ component); VP-101 v1.2: 2 stale path anchors citing host/read_file.rs instead of host/read_prefix.rs. See F-P53-001 and F-P53-002.** |

### Findings

**F-P53-001 — MEDIUM [POLICY 4 description-bearing anchor-prose mis-anchor]**

**Location:** VP-097 v1.1 (`VP-097.md`), three description-bearing anchor-prose fields.

**Pre-fix evidence (VP-097 v1.1, D-808 versions):**
```
Stale field 1 — frontmatter module:
  module: crates/factory-dispatcher/src/path_util.rs

Stale field 2 — §Feasibility Assessment Artifact cell:
  | Artifact | `crates/factory-dispatcher/src/path_util.rs` pure function |

Stale field 3 — §Proof Harness Skeleton // File: comment:
  // File: crates/factory-dispatcher/src/path_util_kani.rs
```

**SoT canonical grep evidence (10 normative sites vs 2 stale in VP-097 v1.1):**
```
$ grep -rn "host/path_util" \
    .factory/specs/behavioral-contracts/ \
    .factory/specs/verification-properties/ \
    .factory/stories/ \
  | grep -v "VP-097" | grep -v "VP-INDEX" | grep -v "adv-E19" \
  | grep -c "host/path_util"
10
(sites: BC-2.07.001 ×3, BC-2.02.011 ×3, BC-1.17.001 ×2, S-19.03 ×1, S-19.06 ×1)

$ grep -n "path_util\.rs" .factory/specs/verification-properties/VP-097.md \
  | grep -v "host/"
26:module: crates/factory-dispatcher/src/path_util.rs
137:| Artifact | `crates/factory-dispatcher/src/path_util.rs` pure function |
(2 stale normative hits in VP-097 v1.1; harness comment path_util_kani.rs is a 3rd adjacent instance)
```
SoT ratio: 10 canonical to 2 primary normative stale (plus 1 harness comment variant). All three stale instances lack the `host/` path component.

**Defect:** VP-097 v1.1 `module:` field and §Feasibility Artifact cell cite `crates/factory-dispatcher/src/path_util.rs` — missing the `host/` subdirectory component. The BC-2.07.001 §Architecture Anchors (line 52) and BC-2.02.011 §Architecture Anchors (line 110) both establish `crates/factory-dispatcher/src/host/path_util.rs` as the canonical location. S-19.03 §File Structure and §Architecture Mapping also use the canonical `src/host/path_util.rs` path. The `// File:` harness comment used `path_util_kani.rs` without the `host/` component.

**Root cause:** F-P42-001 (D-797) stable-anchor sweep was scoped to BC-version-pin cites (volatile `BCN.NN.NNN vN.N` patterns → stable `§SectionName` anchors). Path-anchor fields (`module:`, Artifact cells, harness `// File:` comments) are description-bearing anchor prose per POLICY 4 but were not in scope of the F-P42-001 sweep. The VP was created with the path before `host/` subdirectory placement was finalized, and the F-P42-001 sweep missed this class entirely.

**CLOSED:** Architect 8cd9e391 — VP-097 v1.1→v1.2: `module: src/path_util.rs` → `src/host/path_util.rs`; §Feasibility Artifact cell updated; §Proof Harness `// File:` → `src/host/path_util_kani.rs`; input-hash re-stamped c47964f (BC inputs updated since v1.1). 8-VP class sweep (16 path-anchor fields across VP-094..VP-101): only VP-097 and VP-101 drifted; 14 others CLEAN.

---

**F-P53-002 — MEDIUM [POLICY 4 description-bearing anchor-prose mis-anchor]**

**Location:** VP-101 v1.2 (`VP-101.md`), two description-bearing anchor-prose fields.

**Pre-fix evidence (VP-101 v1.2, D-808 versions):**
```
Stale field 1 — frontmatter module:
  module: crates/factory-dispatcher/src/host/read_file.rs

Stale field 2 — §Traceability Function-anchor bullet:
  - **Function anchor:** `host::read_file` in
    `crates/factory-dispatcher/src/host/read_file.rs` (co-located with the implementation)
```

**SoT canonical grep evidence (6 normative sites vs 2 stale in VP-101 v1.2):**
```
$ grep -rn "host/read_prefix\.rs\|read_prefix\.rs" \
    .factory/specs/behavioral-contracts/ \
    .factory/stories/ \
  | grep -v "VP-101" | grep -v "VP-INDEX" | grep -v "adv-E19" \
  | grep -c "read_prefix"
6
(sites: BC-1.17.001 §Architecture Anchors ×1, S-19.06 §File Structure/§ACs ×4, S-19.07 §Dependencies ×1)

$ grep -n "read_file\.rs" .factory/specs/verification-properties/VP-101.md
10:...read_file.rs → crates/factory-dispatcher/src/host/read_prefix.rs...   (last_amended — already fixed)
24:module: crates/factory-dispatcher/src/host/read_file.rs                    (pre-fix stale field 1)
34:...read_file.rs → read_prefix.rs...                                        (modified[] entry — already fixed)
219:  `crates/factory-dispatcher/src/host/read_file.rs` (new host-function file; parallels...)  (post-fix reference)
```
(Pre-fix VP-101 v1.2 stale instances: module: citing read_file.rs; §Traceability Function-anchor citing read_file.rs. 2 stale normative hits.)
SoT ratio: 6 canonical to 2 stale normative.

**Defect:** VP-101 v1.2 `module:` field cited `host/read_file.rs` instead of `host/read_prefix.rs`. The §Traceability Function-anchor bullet cited `host::read_file` in `read_file.rs`. `host::read_prefix` is implemented in a NEW file `crates/factory-dispatcher/src/host/read_prefix.rs` per BC-1.17.001 §Architecture Anchors and S-19.06 §Architecture Mapping. S-19.06 explicitly states "read_file.rs is NOT modified" (Architecture Mapping). VP-101 is the proof property for `read_prefix`, not `read_file`.

**Root cause:** VP-101 was authored with `read_file.rs` as the module cite (the function was originally planned co-located) before S-19.06 Architecture Mapping established the new-file form. The same F-P42-001 sweep (D-797) that scoped to BC-version-pin cites did not touch path-anchor fields; the new-file placement was codified in S-19.06 but VP-101's path fields were not updated to match. F-P43-004b (D-799) corrected PC cites in the same VP but did not scope to module: / Function-anchor fields.

**CLOSED:** Architect 8cd9e391 — VP-101 v1.2→v1.3: `module: host/read_file.rs` → `host/read_prefix.rs`; §Traceability Function-anchor rewritten: `host::read_prefix` in `crates/factory-dispatcher/src/host/read_prefix.rs` (new host-function file; parallels `host::read_file` in `host/read_file.rs`); input-hash 531cd2f UNCHANGED (BC inputs unchanged since v1.2).

---

### 8-VP Class Sweep (POLICY 5 sibling-sweep — 16 path-anchor fields audited)

| VP | module: field | §Feasibility Artifact / §Traceability Function-anchor | Result |
|----|--------------|------------------------------------------------------|--------|
| VP-094 | `crates/factory-dispatcher/src/host/write_file.rs` | §Traceability: `host::write_file` in `write_file.rs` | PASS |
| VP-095 | `crates/factory-dispatcher/src/host/read_file.rs` | §Feasibility: `read_file.rs` effectful-shell | PASS |
| VP-096 | `crates/factory-dispatcher/src/host/write_file.rs` | §Feasibility: `write_file.rs` | PASS |
| **VP-097** | **`src/path_util.rs` (missing host/) — STALE** | **§Feasibility: `src/path_util.rs` — STALE; §Harness: `path_util_kani.rs` missing host/ — STALE** | **FAIL → CLOSED 8cd9e391** |
| VP-098 | N/A (scope: `hooks-registry.toml` entry structure) | §Feasibility: registry-level, no module path | PASS |
| VP-100 | `crates/factory-dispatcher/src/host/write_file.rs` | §Feasibility: drain_window_ms in dispatcher | PASS |
| **VP-101** | **`src/host/read_file.rs` — wrong file (should be read_prefix.rs)** | **§Traceability Function-anchor: `host::read_file in read_file.rs` — STALE (should be read_prefix.rs)** | **FAIL → CLOSED 8cd9e391** |
| VP-099 | (S-19.04 scope; no path-anchor module: field) | N/A | PASS |

14 PASS + 2 FAIL → CLOSED. Only VP-097 and VP-101 drifted; 6 other VPs with explicit module: cites all CLEAN.

## Part B — Full Perimeter Attestation

### B.1 Perimeter Completeness (30 artifacts)

All 30 perimeter artifacts verified at D-808 closure versions.

| Artifact | Version at Pass-53 | Status |
|----------|-------------------|--------|
| ADR-025 | v1.15 | PASS |
| ADR-030 | v1.3 | PASS |
| BC-1.17.001 | v1.6 | PASS |
| BC-2.02.011 | v1.7 | PASS |
| BC-2.07.001 | v1.5 | PASS |
| BC-3.08.001 | v1.21 | PASS |
| BC-4.13.001 | v1.14 | PASS |
| BC-5.42.001 | v1.6 | PASS |
| VP-094 | v1.1 | PASS |
| VP-095 | v1.1 | PASS |
| VP-096 | v1.1 | PASS |
| VP-097 | v1.1 (pre-fix); v1.2 (post-fix 8cd9e391) | FAIL → CLOSED (F-P53-001) |
| VP-098 | v1.2 | PASS |
| VP-100 | v1.2 | PASS |
| VP-101 | v1.2 (pre-fix); v1.3 (post-fix 8cd9e391) | FAIL → CLOSED (F-P53-002) |
| S-19.01 | v1.17 | PASS |
| S-19.02 | v1.17 | PASS |
| S-19.03 | v1.19 | PASS |
| S-19.04 | v1.11 | PASS |
| S-19.05 | v1.16 | PASS |
| S-19.06 | v1.19 | PASS |
| S-19.07 | v1.16 | PASS |
| epic (E-19) | v1.27 | PASS (F-P52-001 fix confirmed; 6-row re-audit 6/6 PASS) |
| policies.yaml | v1.4.3 | PASS |
| BC-INDEX | v3.95 | PASS |
| VP-INDEX | v2.59 | PASS (at D-808 closure; VP-097/101 annotation updates part of D-809 SM burst) |
| STORY-INDEX | v4.176 | PASS |
| ARCH-INDEX | v3.00 | PASS |
| verification-architecture.md | v1.8 | PASS — POLICY 9 not triggered (description-only VP annotation updates; no title changes) |
| verification-coverage-matrix.md | v1.5 | PASS — POLICY 9 not triggered |

28 PASS + 2 FAIL → CLOSED.

### B.2–B.23 Policy Attestations

| Policy | Status | Notes |
|--------|--------|-------|
| POLICY 1 (append-only BC catalog) | PASS | No BC changes this pass; BC-INDEX v3.95 UNCHANGED |
| POLICY 2 (BC versioning) | PASS | No BC version changes this pass |
| POLICY 3 (no tool bypass) | PASS | Architect fix burst via Edit tool; no bypass |
| POLICY 4 (semantic anchoring) | FAIL → FIXED | F-P53-001 + F-P53-002: VP path-anchor fields are description-bearing anchor prose; CLOSED architect 8cd9e391 |
| POLICY 5 v1.3.8 (sibling-sweep) | PASS | 8-VP class sweep performed (16 path-anchor fields); only 2 drifted; 6 CLEAN (no sibling escape beyond VP-097 + VP-101) |
| POLICY 6 (ARCH-INDEX canonical) | PASS | ARCH-INDEX v3.00 UNCHANGED this pass |
| POLICY 7 (BC H1 verbatim parity) | PASS | Char-diff 6/6 PASS |
| POLICY 8 (BC frontmatter atomicity) | PASS | No BC changes |
| POLICY 9 (VP-INDEX as SoT) | PASS | VP annotation updates are description-only (no title changes); same-burst propagation to verification-architecture.md / verification-coverage-matrix.md not required per POLICY 9 exemption (D-809 SM burst) |
| POLICY 10 (mandatory gating) | PASS | 8 standing gates operational; adjacent-axis extension codified as D-809 process-gap note |
| POLICY 11 (production-grade) | PASS | VP path-anchor fields require production-grade path accuracy |
| POLICY 12 (decision-log continuity) | PASS | D-808 max → D-809 allocated (POLICY 16 gate) |
| POLICY 13 (L-BB lessons carry-across) | PASS | All 8 standing gates operational; new [process-gap] lesson codified (L-BB-description-bearing-anchor-prose-derives-from-target-SoT) |
| POLICY 14 (5-leg parity) | PASS | VP-097 v1.2 + VP-101 v1.3: 5-leg parity complete at D-809 SM closure |
| POLICY 15 (verification steps attestation) | PASS | Literal-shell evidence in burst-log D-809 Block 2 (D-449(a)) |
| POLICY 16 (D-NNN global-max gate) | PASS | D-808 confirmed max → D-809 allocated |
| POLICY 17 (last_amended chain form) | PASS | VP-097 + VP-101 last_amended updated by architect 8cd9e391; chain form maintained |
| POLICY 18 (input-hash mechanically computed) | PASS | VP-097 input-hash c47964f (BC inputs updated; legitimate drift); VP-101 input-hash 531cd2f UNCHANGED |
| POLICY 19 (stable anchor form) | PASS | 0 volatile BC/ADR pins in normative sections; path anchors corrected to canonical form |
| DTU | N/A | Not applicable |
| Multi-repo | N/A | Not applicable |
| Formal-verify | N/A | Not applicable |

### 8-Gate Roster (8 OPERATIONAL; 2 FAIL → FIXED)

1. 4-index literal-shell gate (D-494) — PASS: BC v3.95 / VP v2.59 / STORY v4.176 / ARCH v3.00 at D-808 closure
2. Heading-parity gate (D-803) — PASS: STORY-INDEX v4.176 §Epic E-19 heading `draft, v1.27` matches epic frontmatter v1.27; 6 other versioned-heading epics PASS
3. Pointer-class gate (D-806) — PASS: 0 normative hits; ADR-025 line 1708 Changelog historical (exempt per TD-VSDD-091)
4. ADR body BC-cite sweep (D-795) — PASS: 0 volatile BC-version pins in normative ADR sections
5. ADR body content-description sweep (D-805) — PASS: tool matcher + block anchor descriptions match live registry ground truth
6. POLICY 7 char-diff gate (D-794) — PASS: 6/6 BC title cells char-exact
7. D-779 whole-file predicate gate — PASS (N/A — no cite sweeps this burst; no residual stale tokens)
8. **BC Traceability row description parity gate (D-808)** — **PASS**: 6/6 BC rows verified against target SoT (post-D-808 fix; all correct per delta verification above)

**Adjacent-axis FAIL (VP path-anchor description-bearing prose):** The 8th gate covers epic §BC Traceability row description → SoT parity. F-P53-001 + F-P53-002 are an adjacent axis: VP `module:` fields, §Feasibility Artifact cells, and §Traceability Function-anchor bullets are also description-bearing anchor prose per POLICY 4. These are NOT covered by the current 8th gate definition (which scopes to epic Traceability tables). The adjacent-axis is now codified as D-809 POLICY 4 verification_steps extension: DESCRIPTION-BEARING ANCHOR-PROSE PARITY — extends POLICY 4 adversary audit scope to VP frontmatter module: fields, §Traceability Function-anchor/Subsystem/Anchor-story bullets, and §Feasibility Artifact cells. Dispatched to state-manager as D-809 codification obligation.

### Zero-Finding Attestation Evidence (for PASS axes)

**Pointer-class whole-perimeter grep:**
```
grep -nE 'line [0-9]+([–-][0-9]+)? of|at line [0-9]+' \
  .factory/specs/architecture/decisions/ADR-025-*.md \
  .factory/specs/architecture/decisions/ADR-030-*.md
```
→ ADR-025 line 1708: EXEMPT — Changelog v1.15 bullet (historical-by-construction per TD-VSDD-091)
→ ADR-030: 0 hits
→ 0 normative-live hits.

**POLICY 7 char-diff evidence:**
- BC-1.17.001, BC-2.07.001, BC-2.02.011, BC-3.08.001, BC-4.13.001, BC-5.42.001: H1↔BC-INDEX title cell char-exact ✓

### Trajectory Note

Passes 22–53 trajectory: 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2→3→1→1→0→1→1→0→1→2. Pass-53 NOT-CLEAN (2 MEDIUM findings F-P53-001 + F-P53-002). Trajectory tail passes 50/51/52/53 = 0,1,1,2 → `→0→1→1→2`. Wait — pass-51 was CLEAN (0), pass-52 was NOT-CLEAN (1), pass-53 is NOT-CLEAN (2). Trajectory tail = passes 50/51/52/53 = 1,0,1,2 → `→1→0→1→2`. Streak 0/3 (unchanged; NOT-CLEAN resets remain at 0/3). Three consecutive CLEANs required for 3/3 convergence.

### Novelty

**[process-gap] — VP path-anchor fields are description-bearing anchor prose subject to POLICY 4.** The D-808 8th gate established that epic §BC Traceability row DESCRIPTIONS are a POLICY 4 audit axis. F-P53-001 + F-P53-002 demonstrate the adjacent axis: VP frontmatter `module:` fields, §Feasibility Artifact cells, and §Traceability Function-anchor bullets all describe the target artifact's file/module location — they are description-bearing anchor prose per the same POLICY 4 principle. These fields must derive from the implementation SoT (BC §Architecture Anchors, story §File Structure). The F-P42-001 sweep (stable-anchor discipline) only touched BC-version-pin cites; path-anchor fields were never in scope for any prior sweep. Detection predicate: for each VP path-anchor field, grep the cited path against the target BC file's §Architecture Anchors and the anchor story's §File Structure — zero-match or contradiction = POLICY 4 mis-anchor MEDIUM+. **Recommendation: extend POLICY 4 verification_steps with DESCRIPTION-BEARING ANCHOR-PROSE PARITY step** (dispatched to state-manager as D-809 codification; supersedes-extends D-808 8th-gate scope to all VP spec-level anchor-prose fields).

### Iron Law Compliance

Confirmed. Fresh context for pass-53. Prior pass reports (adv-E19-pass-52.md and earlier) NOT loaded. Rubric policies.yaml v1.4.3 applied. Zero cross-contamination from prior passes.
