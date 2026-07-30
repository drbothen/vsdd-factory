---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-07-30T00:00:00Z
phase: 3
inputs:
  - stories/S-21.04-story-worktree-write-path-discipline.md
  - specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md
  - specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
  - policies.yaml
input-hash: "2d1d78b"
traces_to: "BC-6.26.001 v1.18; story v1.33"
pass: 29
verdict: NOT-CLEAN
reviewed_head: "eba02788"
fixes_landed_head: "44547051"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-28.md"
findings_count: 13
severity_breakdown: "B0/H5/M6/L2"
streak: "0/3"
trajectory_append: 13
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-28 Part A inlined; prior pass files (pass-1 through pass-27) and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy22_note: "POLICY 22 SUBAGENT-REPORT-FIDELITY active: adversary findings reported directly; no relay chain between adversary and state-manager in this pass"
---

# Adversary Pass 29 — S-21.04 story-worktree write-path discipline

**Date:** 2026-07-30
**Reviewed HEAD:** `eba02788`
**Fixes-landed HEAD:** `44547051` (feature-branch; factory-artifacts fixes land in this burst's Commits A–E; SHA-patch will update after Commit E)
**Verdict:** NOT-CLEAN — B0/H5/M6/L2 = 13 findings
**Streak:** 0/3 (BC-5.39.001) — first zero-BLOCKER pass of this cascade

## Finding ID Convention

Finding IDs for this cascade use the format: `F-S2104-P<PASS>-<SEV><SEQ>` (project-local convention established at pass-1). Map to template ADV severity abbreviations: B→BLOCKER, H→HIGH, M→MEDIUM, L→LOW. Full format example: `F-S2104-P29-H01`.

## Mandatory Provenance Disclosure

**(1) Model override:** NO model override applied. Frontmatter `model: opus` → `claude-opus-5`. Fresh context enforced.

**(2) ADR-033 cross-family limitation:** ADR-033 §Decision 2 cross-family independence (GPT-5) NOT satisfied. The reviewing model is in the same family as the authoring agents. Fresh context and information asymmetry are intact; cross-family cognitive diversity is absent. Disclosed per ADR-033 §Decision 3.

**(3) Information asymmetry enforcement:** Pass-28 Part A findings were inlined as permitted context. Prior pass files (pass-1 through pass-27) and the cycle `INDEX.md` were off-limits. The adversary confirms it opened none.

**(4) Scope at reviewed HEAD:** The adversary reviewed `eba02788` — the feature-branch state after pass-28 factory-artifacts burst but before the pass-29 fix-burst commits. Gates (13)–(18) were re-derived from bats source at this HEAD.

## Part A — Fix Verification (Pass-28 Closures Re-Verified at `eba02788`)

| ID | Pass-28 Severity | Status | Notes |
|----|-----------------|--------|-------|
| P28-B01 | BLOCKER | GENUINELY-CLOSED | Coupling gate updated to handle detached-HEAD CI worktree; `fa_wt` now resolved via `git -C "$WT_PATH" symbolic-ref --short HEAD 2>/dev/null || git -C "$WT_PATH" rev-parse --short HEAD`; CI gate passes on detached-HEAD factory-artifacts mount. |
| P28-H01 | HIGH | PARTIAL | `story_count` re-anchored to AC-001 Gate cell behavioral anchor. Anchor fix behaviourally inert at this HEAD — reverting to whole-file `head -1` still yields 23 (the same value), so the fix cannot be independently confirmed to catch a divergence. → H01 new finding. |
| P28-H02 | HIGH | GENUINELY-CLOSED | Dead arithmetic blocks deleted; T-008 re-attested to the single equality check at line 1070. |
| P28-H03 | HIGH | GENUINELY-CLOSED | STORY-INDEX blockquote `S-21.04=04b393e→d6d6a6a`. Story v1.30 provenance hash corrected in STORY-INDEX. |
| P28-H04 | HIGH | GENUINELY-CLOSED | T-016 registered in §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks. |
| P28-H05 | HIGH | PARTIAL | BC leg and bats leg closed (EC-009 structural rationale added; T-010/RG-010 added to bats). Story leg not closed — EC-009 in story has no T-010/RG-010 cite and no residual-disclosure for socket/device-node non-coverage. → H05 new finding note; PARTIAL classification applies to pass-28's H05. |
| P28-H06 | HIGH | GENUINELY-CLOSED | AC-007 rewritten to predicate-agnostic prohibition. T-008 updated with widened predicate check. |
| P28-H07 | HIGH | GENUINELY-CLOSED | Fixture README updated: `! -type d` predicate form documented; EC-009 symlink/FIFO edge case added. |
| P28-M01 | MEDIUM | GENUINELY-CLOSED | §File Structure Requirements bats count 15→16. |
| P28-M02 | MEDIUM | GENUINELY-CLOSED | Four live M1–M9→M1–M14 / M5–M9→M5–M14 corpus-range sites updated. |
| P28-M03 | MEDIUM | GENUINELY-CLOSED | Task 15 marked `[x]`; scope extended to M5–M14. |
| P28-M04 | MEDIUM | GENUINELY-CLOSED | Story BC table + Token Budget v1.15→v1.16; v1.16→v1.17 via state-manager Commit C sweep. |
| P28-M05 | MEDIUM | GENUINELY-CLOSED | Gate (14) section-wide extractor tightened; SIBLING-PARAGRAPH mutant added. |
| P28-M06 | MEDIUM | GENUINELY-CLOSED | Gate (16) direction-statement annotation added; adversary.md rule ordinal pins replaced with behavioral anchors at 6 guard sites. |
| P28-M07 | MEDIUM | GENUINELY-CLOSED | Pipeline probe corpus leg extended to M10–M14 in T-010 test; gate (13) re-derived with expanded corpus. |
| P28-L01 | LOW | GENUINELY-CLOSED | T-010 delta-proof: test invocations now assert POSIX find semantics on the fixture directory directly; fixture README elevated-evidence claim removed. |
| P28-L02 | LOW | GENUINELY-CLOSED | Gate (18) extractor tightened to `#### Worktree-Identity Preflight` section boundary. |

**Summary:** 15 GENUINELY-CLOSED · 2 PARTIAL (P28-H01 anchor behaviourally inert; P28-H05 story leg missing residual-disclosure).

**Adversary retractions (evidence of genuine analysis):** `factory-health`/`factory-worktree-health` skills operate on `.factory` mount rather than story worktrees — outside BC-6.26.001 trigger scope; the three AC-007(a)–(c) surfaces contain no unguarded `git worktree remove`; `rules/worktree-protocol.md:76` satisfies the mandate-token gate via line 52's `**MUST**`; T-004's PC2c domain excludes `step-g-cleanup.md:100` (that location is a prose rationale block, not a gate invocation); `main_root` = first `--porcelain` entry is documented git behaviour for `git worktree list --porcelain`; `relocate-artifact/SKILL.md` is outside AC-007(d)'s trigger (it relocates artifacts, does not perform story-worktree teardown).

---

**Orchestrator-verified additions (not adversary findings; record verbatim):**

- **The AC-001 gate count was wrong.** T-016 asserted 23; the mechanical count is **24** (`awk` over the AC-001(a) section, `grep -cE '^[[:space:]]+echo "DOC-PARITY FAIL'` → 24). The figure 23 was introduced at pass-27 (`Twenty-one`→`Twenty-three`) by author judgment excluding boundary-completeness (`F-S2104-P19-004`) on a non-reproducible editorial basis. T-016 structurally could not detect this mismatch because both its operands were literal strings. Corrected in story v1.33 and the T-001 comment. ADR-034 v1.1 closes the structural hole: story gate-count cell is now documentation-only; T-016 reads a product-branch `T001_GATE_COUNT=24` sentinel instead.

- **H01 closure verified end-to-end by orchestrator:** Deleting one real gate inside the AC-001(a) section drives `not ok 16`; restoring returns count 24 and 16/16 green.

- **Fabricated input-hash.** story-writer wrote `input-hash: "1acf3c6"` without a `compute-input-hash` invocation. Verified: `compute-input-hash <file> --check` → `DRIFT — 1acf3c6 ≠ computed 47a65c9`. POLICY 18 requires literal mechanical backing. Contributing cause: dispatch instructions said "do NOT run compute-input-hash --update" without noting that read-only forms (`<file>`, `--check`, `--resolve`) are safe — the agent had no sanctioned path to a real value and invented one.

- **CLAUDE.md path error:** CLAUDE.md documents `bin/compute-input-hash`; the actual tool location is `plugins/vsdd-factory/bin/compute-input-hash`.

## Part B — New Findings (Pass 29)

---

### HIGH

---

**Finding ID:** `F-S2104-P29-H01`
**Severity:** HIGH
**Surface:** `worktree-identity-preflight.bats` — T-016 coupling gate operands
**Policies violated:** POLICY 13 (DOMAIN-COMPLETENESS); TD-VSDD-059 (paper-fix — anchor fix behaviourally inert)

**Description:** T-016 (`test_coupling_gate_story_gate_count_matches_bats_count_word`) re-anchored `story_count` from the frontmatter line to the AC-001 Gate cell, but both operands remain literal strings extracted by pattern-match — `story_count` extracts a digit string from the story file on the `.factory` mount, and `bats_count` extracts a count-word from the bats suite. Neither operand is runtime-derived from actual gate execution. Deleting a gate shifts the bats count but leaves the story cell unchanged, so the gate correctly catches deletions — but the story cell itself was wrong (23 vs actual 24). The gate read the wrong story value for two full passes without detection because both "23" and "24" satisfy the same extraction pattern. The mismatch was invisible until orchestrator independently counted the blocks.

**Fix:** test-writer — per ADR-034 v1.1 Decision 2: replace `story_count` with runtime-derived assertion count (grep-count of `echo "DOC-PARITY FAIL` blocks within T-001's body in the bats suite); replace `bats_count` with product-branch sentinel `T001_GATE_COUNT=N` constant. Both operands must be product-branch sources or runtime-derived counts per ADR-034 v1.1 Decision 5.

---

**Finding ID:** `F-S2104-P29-H02`
**Severity:** HIGH
**Surface:** `worktree-identity-preflight.bats` — T-008 path-expression axis coverage
**Policies violated:** POLICY 13 (FAIL-CLOSED-IMPLICATION-DIRECTION); POLICY 3 (coverage completeness)

**Description:** Pass-28 M03 widened T-008 to predicate-agnostic prohibition, but the path-expression axis was left unswept. Forms like `find "$WT" -path "*/.factory/*"`, `find "$WT" -name .factory`, and backslash-continuation variants of `find .factory` (which evade both `find.*\.factory` and `find.*\.factory/`) pass T-008 even though they violate the canonical `find "<worktree-path>/.factory" ! -type d` form. The predicate is now checked; the path expression is not.

**Fix:** test-writer — extend T-008 regex set to cover path-expression variants: quoted vs unquoted path, `$VAR/.factory` vs literal `.factory`, and backslash-continuation forms. Verify against the existing corpus and add representative negative examples.

---

**Finding ID:** `F-S2104-P29-H03`
**Severity:** HIGH
**Surface:** `STORY-INDEX.md:727`, `VP-INDEX.md:8`, `STATE.md:134`, `STATE.md:226` — retracted `B01 = policies.yaml` identity
**Policies violated:** POLICY 18 (THREE-WAY-INPUT-HASH-EQUALITY); POLICY 22 (SUBAGENT-REPORT-FIDELITY — propagation residue)

**Description:** The pass-28 fabrication correction (commit `3d12b780`) corrected `adversary-pass-28.md` Part B but did not propagate to four downstream record sites: (a) STORY-INDEX:727 still records `B01 policies.yaml \\+ BLOCKED` — the fabricated identity and wrong status; (b) VP-INDEX:8 `last_amended` names `B01 policies.yaml \\+ escapes` in the D-942 provenance note; (c) STATE.md:134 D-943 Decisions Log row records `B01 policies.yaml YAML-parse BLOCKED`; (d) STATE.md:226 Session Resume Checkpoint pass-28 summary records `BLOCKER: policies.yaml YAML-parse failure FIXED`. All four carry the fabricated identity. The real B01 is `F-S2104-P28-B01`: coupling-gate hard-fails in CI due to detached-HEAD factory-artifacts worktree. Status is CLOSED at `44547051`.

**Fix (state-manager Commit C):** Append-only error-acknowledgment at all four sites. Do not rewrite history to hide the fabrication. Add `[CORRECTED H03: B01 identity was FABRICATED; real B01 = F-S2104-P28-B01 coupling-gate detached-HEAD CI failure CLOSED 44547051]` annotations.

---

**Finding ID:** `F-S2104-P29-H04`
**Severity:** HIGH
**Surface:** `stories/S-21.04-story-worktree-write-path-discipline.md` — `test_write_discipline_gates` anchor
**Policies violated:** TD-VSDD-091 (anti-volatile-pin compliance produced a permanently-wrong anchor); POLICY 4 (spec-behavior correspondence)

**Description:** Pass-27's fix for the volatile `line 638` pin replaced it with anchor `test_write_discipline_gates`. This function never existed in the bats file — `grep -rn 'test_write_discipline_gates' .worktrees/S-21.04` returns zero hits. The anti-volatile-pin remediation traded a decaying-but-correct pin for a permanently-wrong anchor that survived two full passes (27 and 28) across the most-cited cell in the cascade. Any anchor written under TD-VSDD-091 MUST be `grep`-verified to exist at authoring time.

**Fix:** story-writer — correct the three live sites (lines 108/213/272) to the real `@test` function name: `T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called`.

---

**Finding ID:** `F-S2104-P29-H05`
**Severity:** HIGH
**Surface:** `worktree-identity-preflight.bats` T-016 — reads story operand from `.factory` mount in CI
**Policies violated:** POLICY 3 (CI coverage gate — permanent failure mode after merge); POLICY 13 (DOMAIN-COMPLETENESS)

**Description:** T-016 reads `story_count` from the story file on the `.factory` mount (`$fa_wt`). The `bats-full-suite` job runs on every PR to `main` and `develop`. Any state-manager burst that amends the AC-001 gate count in the story creates an immediate CI failure on every open PR including release PRs — this has already triggered twice. After S-21.04 merges, the story file is permanently pinned to the merge-state value; any subsequent story amendment will misalign the operand. The root is architectural: a required CI coupling check derives an equality-assertion operand from a cross-branch artifact.

**Fix:** architect + test-writer — per ADR-034 v1.1: move story operand to a product-branch sentinel constant `T001_GATE_COUNT=N` immediately preceding T-001's first assertion in the bats suite; T-016 reads the sentinel rather than the story file. Downstream: story-writer amends AC-001 T-016 operand description; product-owner reviews BC-6.26.001 for any invariant requiring T-016 to read the story.

---

### MEDIUM

---

**Finding ID:** `F-S2104-P29-M01`
**Severity:** MEDIUM
**Surface:** Multiple artifact sites — pass-28 numeric finding IDs
**Policies violated:** POLICY 8 (finding-ID namespace consistency); POLICY 22 (SUBAGENT-REPORT-FIDELITY)

**Description:** Pass-28's numeric finding-ID namespace (an orchestrator dispatch artifact; IDs `001`–`017` rather than `B01`/`H01`–`H07`/`M01`–`M07`/`L01`–`L02`) leaked into 42+ artifact sites across adversary-pass-28.md, STATE.md, STORY-INDEX.md, burst-log.md, and agents/adversary.md. The numeric IDs never existed in the canonical record. `agents/adversary.md` ships in the released plugin, so dangling numeric IDs reached operator-level artifacts at v1.0.0-rc.23.

**Fix:** multi-agent — at all live sites: remap numeric IDs to canonical `F-S2104-P28-SEV-SEQ` form (`005→H04`, `007→H06`, `011→M03`, `012→M04`, `013→M05`). Leave provenance lines in `last_amended` / `modified` chain unchanged (append-only discipline).

---

**Finding ID:** `F-S2104-P29-M02`
**Severity:** MEDIUM
**Surface:** Story frontmatter `last_amended` and `modified[v1.30]` — terminal hash annotation ID
**Policies violated:** POLICY 18 (provenance accuracy); POLICY 22 (SUBAGENT-REPORT-FIDELITY — wrong finding ID)

**Description:** The terminal-hash correction annotation in both the `last_amended` chain and the `modified[v1.30]` entry reads `[CORRECTED: was 67eaeea; F-S2104-P28-H03]`. The correct canonical finding ID is `F-S2104-P28-M02` — the terminal-hash finding. `F-S2104-P28-H03` is the STORY-INDEX blockquote stale-hash finding, not the story provenance finding. The mislabel propagates incorrect finding traceability.

**Fix (state-manager Commit C):** In story frontmatter `last_amended` (line 11) and `modified[v1.30]` entry: `F-S2104-P28-H03` → `F-S2104-P28-M02`.

---

**Finding ID:** `F-S2104-P29-M03`
**Severity:** MEDIUM
**Surface:** Story `EC-009` — T-010/RG-010 cross-reference absence and socket/device-node residual
**Policies violated:** POLICY 4 (BC body test-evidence completeness); POLICY 8 (inventory accuracy)

**Description:** Pass-28 H05 closed the BC leg (EC-009 structural rationale) and the bats leg (T-010/RG-010 added to bats). The story leg was not closed: EC-009 in the story body has no T-010/RG-010 cite, and there is no residual-disclosure note that socket (type `s`; requires bound process) and device node (types `b`/`c`; `mknod` root-only on Linux) are not exercised by T-010/RG-010.

**Fix:** story-writer — add T-010/RG-010 coverage annotation and partial-coverage residual to EC-009 (`story:171`), RG-010 row (`story:254`), and T-010 Test Plan row (`story:270`): socket and device-node types not exercised.

---

**Finding ID:** `F-S2104-P29-M04`
**Severity:** MEDIUM
**Surface:** `worktree-identity-preflight.bats` — T-008 exclusion rationale and ASCII `...` sites
**Policies violated:** POLICY 4 (rationale accuracy); TD-VSDD-091 (declared-escape must be the load-bearing mechanism)

**Description:** T-008's annotation named the escape mechanism as "U+2026 HORIZONTAL ELLIPSIS codepoint" — which is wrong. The load-bearing mechanism is the intervening whitespace between `find` and `.factory`, not the codepoint. Additionally, 2 of the 8 annotation sites used ASCII `...` (three ASCII periods), not the U+2026 codepoint. The declared escape and the actual escape mechanism are misaligned; a reviewer auditing the annotation would reach an incorrect understanding of why the corpus examples pass T-008.

**Fix:** test-writer + implementer — correct the exclusion rationale in all 8 annotation sites to the actual mechanism (intervening whitespace); ensure all sites consistently describe the same escape property.

---

**Finding ID:** `F-S2104-P29-M05`
**Severity:** MEDIUM
**Surface:** `worktree-identity-preflight.bats` — T-016 count-word map
**Policies violated:** POLICY 13 (unreachable code / dead path); TD-VSDD-059 (paper-fix detection)

**Description:** T-016's word-to-integer map contained an unreachable `"thirty")` arm. The `Twenty-[a-z]+` regex in the extraction step can only produce `"twenty-something"` values; `"thirty"` can never be matched by that upstream pattern and will never reach the map. The arm produced a false RED signal at 30 gates during testing, which masked detection of the real 23-vs-24 discrepancy for an additional pass.

**Fix:** test-writer — remove the unreachable `"thirty"` arm from the count-word map (or replace the entire map with ADR-034 v1.1 sentinel approach, which makes the map obsolete).

---

**Finding ID:** `F-S2104-P29-M06`
**Severity:** MEDIUM
**Surface:** `BC-INDEX.md:8` (v4.38 `last_amended`) — Refs field cites H07
**Policies violated:** POLICY 8 (finding-reference accuracy); POLICY 14 (5-leg parity)

**Description:** `BC-INDEX.md:8` v4.38 `last_amended` Refs field reads `Refs: F-S2104-P28-H05/H07, D-943`. `H07` (fixture README filter description and EC-009 omission) has no BC-INDEX leg — its sole leg is the fixture README, not a BC artifact. The BC-INDEX v4.38 row records only BC-6.26.001 v1.15→v1.17 changes (H05's EC-009 structural rationale + H07's T-010/RG-010 cross-ref). Product-owner adjudication: the v1.17 row describes H05's BC leg; H07's fixture-README leg does not produce a BC artifact.

**Fix (state-manager Commit C):** `BC-INDEX.md:8` v4.38 Refs → `F-S2104-P28-H05` only (remove `H07`).

---

### LOW

---

**Finding ID:** `F-S2104-P29-L01`
**Severity:** LOW
**Surface:** `worktree-identity-preflight.bats` — T-010 delta-proof POSIX semantics legs
**Policies violated:** POLICY 15 (evidence level — test asserts semantics on test's own invocations, not on deliverable)

**Description:** T-010's delta-proof legs assert POSIX `find` semantics on the test's own invocations rather than on the deliverable code. The fixture README elevated these legs to "secondary behavioral evidence." A test that proves its own correctness against itself (rather than against the deliverable) is a TD-VSDD-059 pattern: the evidence is self-certifying.

**Fix:** test-writer — ensure T-010 proof legs reference the deliverable's actual `find` invocation path; update fixture README to clearly distinguish between deliverable-facing assertions and test-internal correctness lemmas.

---

**Finding ID:** `F-S2104-P29-L02`
**Severity:** LOW
**Surface:** Story `EC-003`/`EC-004` — find-command path form
**Policies violated:** POLICY 4 (spec-behavior correspondence — stale form from BC v1.14)

**Description:** Story `EC-003`/`EC-004` still use the unquoted form `find <worktree-path>/.factory ! -type d` rather than BC v1.15's mandated quoted plain-path form `find "<worktree-path>/.factory" ! -type d`. Pass-27 M02 updated 14 live sites but EC-003/EC-004 were missed.

**Fix:** story-writer — normalize EC-003/EC-004 find-command path to quoted plain-path form per BC v1.15 M02 sweep.

---

## Fix Mapping

| Finding | Owner | Status | Notes |
|---------|-------|--------|-------|
| **H01** | test-writer | **CLOSED `44547051`** | T-016 operands rewritten per ADR-034 v1.1: `story_count` removed; `bats_count` → runtime grep-count of `echo "DOC-PARITY FAIL` blocks within T-001 body; story-writer amends AC-001 T-016 operand description in story |
| **H02** | test-writer | **CLOSED `44547051`** | T-008 regex extended to cover path-expression axis variants; negative examples added to corpus |
| **H03** | state-manager | **FIX Commit C** | Error-acknowledgment annotations at STORY-INDEX:727, VP-INDEX:8, STATE.md:134, STATE.md:226 |
| **H04** | story-writer | **CLOSED (story v1.32)** | Three live anchor sites corrected to real `@test` function name `T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called` |
| **H05** | architect + test-writer | **CLOSED `44547051` per ADR-034 v1.1** | T-016 moved to product-branch sentinel `T001_GATE_COUNT=24`; story operand removed from `$fa_wt` read; ADR-034 v1.1 Design 2+3 |
| **M01** | multi-agent | **CLOSED** | Numeric IDs remapped to canonical form at 7 live sites in story (F-S2104-P29-M01 story sweep); agents/adversary.md, STATE.md, and burst-log sites swept at this burst |
| **M02** | state-manager | **FIX Commit C** | Story `last_amended` line 11 + `modified[v1.30]` line 49: `F-S2104-P28-H03` → `F-S2104-P28-M02` |
| **M03** | story-writer | **CLOSED (story v1.32)** | EC-009 in story body synced to BC v1.17: T-010/RG-010 cite + socket/device-node partial-coverage residual added |
| **M04** | test-writer + implementer | **CLOSED** | T-008 annotation rationale corrected to intervening-whitespace mechanism; all 8 sites use consistent explanation; 2 ASCII `...` sites corrected |
| **M05** | test-writer | **CLOSED — map removed entirely** | Unreachable `"thirty"` arm removed; entire count-word map superseded by ADR-034 v1.1 sentinel approach |
| **M06** | state-manager | **FIX Commit C** | `BC-INDEX:8` v4.38 Refs: `F-S2104-P28-H05/H07` → `F-S2104-P28-H05` |
| **L01** | test-writer | **CLOSED** | T-010 proof legs re-anchored to deliverable invocation path; fixture README language clarified |
| **L02** | story-writer | **CLOSED (story v1.32)** | EC-003/EC-004 find-command normalized to `find "<worktree-path>/.factory" ! -type d` |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 5 |
| MEDIUM | 6 |
| LOW | 2 |

**Total findings:** 13 (B0/H5/M6/L2)

**Overall Assessment:** NOT-CLEAN — first zero-BLOCKER pass across 29 passes; cascade structural pattern shows persistent T-016 architecture debt (two separate T-016 findings this pass: H01 string-literal operands, H05 cross-branch mount read) now jointly addressed by ADR-034 v1.1 rewrite; fabrication propagation residue (H03) not yet cleared from four downstream record sites; TD-VSDD-091 compliance produced a worse anchor than the volatile pin it replaced (H04).

**Convergence:** findings remain — iterate. Streak: 0/3 (zero BLOCKERs; streak advances only on NITPICK-OR-CLEAN verdict). Structural pattern: T-016 architecture (two findings, one ADR) resolved in this burst; fabrication residue (H03/M02/M06) → state-manager Commit C.

**Readiness:** requires revision. Pass-30 NEXT.

**Pass-28 closure assessment:** 15 GENUINELY-CLOSED · 2 PARTIAL (P28-H01 anchor behaviourally inert; P28-H05 story leg missing).

## Novelty Assessment

| Field | Value |
|-------|-------|
| Total findings | 13 |
| New class | 2 (H01 string-literal equality both operands; H04 TD-VSDD-091 produces permanently-wrong anchor) |
| Known class recurrence | 11 |
| Novelty ratio | 13/13=1.0 (all findings represent unresolved or newly manifested defects) |
| META-LEVEL observation | T-016 architecture debt spawned two findings this pass (H01 + H05) resolved jointly by a single ADR decision (ADR-034 v1.1); this is the first pass where an ADR was the primary fix vehicle for a coupling-gate finding class. Fabrication propagation remains structurally unresolved until a systematic sweep gate is codified. |

## Completeness Statement

The adversary did NOT open `policies.yaml` (22 policies' `verification_steps` unexecuted — second consecutive pass without policy rubric review; orchestrator will inline full rubric for pass-30). AC-001 gates (1)–(24) not individually re-derived. Pipeline probe Legs A–E, T-002/T-003/T-005/T-006 bodies, `agents/devops-engineer.md`, T-007, T-009/AC-009/AC-010, ADR-031, `step-d5-adversary-convergence.md` not read. Tests not executed (read-only session, no Bash). `crates/hook-plugins/validate-policies-schema/` out of perimeter. BC-6.26.001 carries four `(TBD)` VP IDs and `§VP Anchors` unassigned while POL-14 will auto-promote it `draft → active` — flagged, not counted, on TD-VSDD-063 precedent.

**D-448(a) source-attestation note:** Part B above was derived exclusively from the authoritative 13-finding list provided in the state-manager dispatch prompt. The adversary did NOT derive findings from INDEX.md, STATE.md, or prior pass files. This differs from pass-28's Commit A fabrication, which confabulated from INDEX.md leads. The state-manager attests Part B correspondence to the dispatch-specified finding set per D-448(a).
