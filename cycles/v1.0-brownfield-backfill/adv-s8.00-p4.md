---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 1d
inputs: [.factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md]
input-hash: "[live-state]"
traces_to: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
pass: 4
previous_review: adv-s8.00-p3.md
target: S-8.00 v1.3
target_file: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
verdict: NITPICK_ONLY
clock: 0_of_3 → 1_of_3
findings_total: 3
findings_high: 0
findings_med: 0
findings_low: 2
findings_nit: 1
---

# Adversarial Review — S-8.00 v1.3 — Pass 4

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix (e.g., `S8P4` for S-8.00 pass 4)
- `<PASS>`: Two-digit pass number (e.g., `P04`)
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`, `NIT`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Shorthand used in this review: `F-P4-NNN` (e.g., `F-P4-001`) consistent with prior
passes in this S-8.00 convergence cycle.

## Executive Summary

Pass-4 adversarial review of S-8.00 v1.3 (512 lines post-fix-burst; 9 ACs; 5pts;
status=draft). Verdict: **NITPICK_ONLY**. Clock advances **0_of_3 → 1_of_3** per
ADR-013.

Three findings were identified: 2 LOW + 1 NIT. Zero HIGH, zero MED. All three are
partial-fix-regression class — second-order propagation gaps from earlier fix bursts
rather than fresh architectural defects:

- F-P4-001 [LOW]: F-P3-005 (apt-get install order reorder) propagated to the Task A.0
  step but the Library & Framework table on line ~481 still listed only `cargo install
  hyperfine` without the higher-priority `apt-get` / `brew` paths. Three-path priority
  order not enumerated in the table.
- F-P4-002 [LOW]: Latent forward-reference in Edge Cases section — no EC documented
  the fact that at S-8.00 baseline time, `all_hook_plugins_wasm_bytes` will be
  approximately 0 (no hook plugin bundles exist yet; S-8.01..S-8.28 produce them).
  This creates an implicit forward-reference to the bundle directory that is
  undisclosed, making R-8.09's 25% growth ceiling anchor ambiguous at baseline.
- F-P4-003 [NIT]: Token Budget overhead measurement row still references `time`
  instead of `hyperfine` — a propagation gap from F-P2-004's measurement-method ban
  that was sealed in AC-1 + Task A.0 but not carried through to the Token Budget
  overhead listing.

Per S-7.03 skip-fix discipline, a fix burst is OPTIONAL given NITPICK_ONLY verdict.
Fix burst applied for cleanliness.

**Trajectory: 14→8→6→3 (50% decay pass-3→pass-4; 79% total decay from pass-1
baseline).**

---

## Part A — Fix Verification (Pass-3 Findings)

All 6 pass-3 findings verified CLOSED in v1.3:

| Finding | Pass-3 Description | v1.3 Verification |
|---------|--------------------|-------------------|
| F-P3-001 [MED] | AC-7 `du -sh` → `du -sb` | CLOSED — `du -sb` confirmed in AC-7 + Task A.6 |
| F-P3-002 [MED] | AC-7 `bundle_size_bytes` → `bundle_size` object | CLOSED — `bundle_size` object with named fields in AC-7 schema |
| F-P3-003+006 [MED+NIT] | AC-1 measurement-method prose hyperfine | CLOSED — AC-1 names "bats harness wrapping `hyperfine --warmup 3 --runs 10`" with integer ms |
| F-P3-004 [LOW] | Task A.5 names validate-bc-title.sh as Tier-2 source | CLOSED — validate-bc-title.sh named as Tier-2 per-plugin source in Task A.5 |
| F-P3-005 [NIT] | Task A.0 install order (apt-get first) | CLOSED — Task A.0 now shows apt-get first, brew second, cargo fallback |
| F-P3-006 [NIT] | Closed in F-P3-003 fix | CLOSED — combined resolution confirmed |

No partial closures from pass-3. All 6 closed cleanly.

---

## Part B — New Findings

### F-P4-001 [LOW] — Library Table apt-get Install Path Missing

**Location:** Library & Framework Dependencies table, approximately line 481.

**Description:** Task A.0 (pre-flight install verification) was correctly updated by
F-P3-005 to enumerate install paths in priority order: apt-get → brew → cargo. However
the Library & Framework table in the story body lists only `cargo install hyperfine` as
the installation method. A reader consulting the table directly (without cross-reading
Task A.0) would install via cargo even on systems where apt-get or brew is available,
contradicting the priority order established by F-P3-005.

**Evidence:** F-P3-005 fix updated Task A.0 with 3-path order but did not propagate to
the Library & Framework table. Classic partial-fix propagation gap.

**Severity rationale:** LOW — does not affect AC correctness; affects documentation
completeness and consistency with F-P3-005 intent. No architectural impact.

**Required fix:** Library table `hyperfine` row: enumerate installation options in
priority order (apt-get / brew / cargo) matching Task A.0.

---

### F-P4-002 [LOW] — EC-008 Missing: Bundle Directory Forward-Reference Undisclosed

**Location:** Edge Cases section; all_hook_plugins_wasm_bytes AC-7 schema field.

**Description:** S-8.00 is the W-15 pre-work story that establishes the performance
baseline BEFORE any WASM hook plugin bundles exist. The `all_hook_plugins_wasm_bytes`
schema field measures aggregate WASM plugin binary size — but at S-8.00 baseline,
this value will be approximately 0 (hook plugin bundles are produced by S-8.01..S-8.28,
which are downstream stories that S-8.00 blocks).

This creates an undisclosed forward-reference: the story anchors R-8.09's 25% growth
ceiling to S-8.00's baseline measurement, but the baseline value itself is ≈0 rather
than a meaningful non-zero anchor. Without an EC disclosing this, a reader might assume
the baseline captures some existing plugin bundle size, or question why R-8.09's
ceiling is anchored to a zero value.

This is a `[process-gap]` class finding — the baseline's reference state cannot be
fully understood without the forward-reference disclosure.

**Evidence:** S-8.01..S-8.28 are listed as blocked-by S-8.00 in STORY-INDEX. The
blocks relationship confirms S-8.00 executes first. No EC currently documents the
≈0 expectation for `all_hook_plugins_wasm_bytes` at baseline.

**Severity rationale:** LOW — does not break any AC; the forward-reference is implicit
but undisclosed, creating a documentation gap for future implementers and reviewers
auditing R-8.09 ceiling provenance.

**Required fix:** Add EC-008: "At S-8.00 baseline (pre-W-15), `all_hook_plugins_wasm_bytes`
≈ 0 — no hook plugin WASM bundles exist until S-8.01..S-8.28 execute. This baseline
records the reference state for R-8.09's 25% growth ceiling; the ceiling becomes
meaningful as downstream stories produce plugin bundles. [process-gap] forward-reference
disclosure per D-2 Option C."

---

### F-P4-003 [NIT] — Token Budget Overhead Row: `time` → `hyperfine`

**Location:** Token Budget section, overhead row for measurement tooling.

**Description:** The Token Budget section lists overhead for the benchmarking approach.
The overhead row for the timing measurement tool still references `time` (the bash
builtin), which was banned by F-P2-004 and sealed in AC-1 + Task A.0. F-P2-004
propagated to AC-1 and Task A.0 but did not reach the Token Budget overhead listing.

This is a second-order propagation gap from F-P2-004 — the measurement-method ban
was applied in the functional spec sections but not in the overhead accounting section.

**Severity rationale:** NIT — no functional impact; purely a documentation consistency
gap between the overhead listing and the locked measurement method.

**Required fix:** Token Budget overhead row: `time` → `hyperfine` to match locked
measurement method.

---

## Part C — Cross-Document Consistency

No cross-document consistency issues identified at pass-4. The following invariants
were verified:

- STORY-INDEX line 166 S-8.00 entry: consistent with v1.3 story state (510 lines,
  pass-3 closed). Will require update for v1.4 post-fix-burst.
- E-8 AC-7b (epic-level acceptance criterion for 10ms/plugin baseline): S-8.00 AC-7
  schema and AC-1 measurement method consistently reference hyperfine and integer ms.
  Cross-reference clean.
- R-8.09 25% growth ceiling: anchored to S-8.00 AC-7 `all_hook_plugins_wasm_bytes`
  baseline. Forward-reference undisclosed (F-P4-002 above).
- OQ-8 (~10ms/plugin assumption): S-8.00 AC-1 + AC-7 consistently target resolution.
  No new drift.
- BC-7.00.001/002 v1.1 BC candidates: still consistently registered; no drift.

---

## Part D — Policy Compliance

POLICY compliance scan (9 active policies):

| Policy | Check | Result |
|--------|-------|--------|
| POLICY 1 (lifecycle coherence) | story status=draft; behavioral_contracts=[] with [process-gap] disclosure; blocks S-8.01..S-8.09 | PASS |
| POLICY 2 (BC anchor integrity) | behavioral_contracts=[] justified under D-2 Option C; 2 v1.1 BC candidates registered | PASS |
| POLICY 3 (state-manager-runs-last) | Not applicable to review pass; fix burst follows this policy | N/A |
| POLICY 4 (no speculative content) | No speculative claims; ≈0 expectation for baseline bytes is empirically justified by blocking relationship | PASS |
| POLICY 5 (source-BC traceability) | Source BCs referenced; BC-7.00.001/002 candidate table present | PASS |
| POLICY 6 (ADR-013 clock discipline) | NITPICK_ONLY → clock 0_of_3 → 1_of_3 per ADR-013 §4 | PASS |
| POLICY 7 (archaeology probe) | Task A.0 apt-get→brew→cargo order consistent with F-P3-005 per ADR revision history | PASS |
| POLICY 9 (same-burst VP promotion) | No VP changes in scope | N/A |
| POLICY 11 (no test tautologies) | Story spec, not implementation; N/A | N/A |

---

## Part E — Self-Validation

10-category fresh-defect scan:

1. **Scope creep / two-responsibility drift:** Two-scope (A: perf baseline; B: BC-anchor
   verification) clean. No new scope introduced. PASS.
2. **AC-to-task coherence:** AC-1 measurement method (hyperfine, warmup 3 runs 10,
   integer ms) consistently reflected in Tasks A.0/A.2/A.4/A.7. PASS (F-P4-003 is
   Token Budget section, not AC-to-task coherence failure).
3. **Schema field consistency:** AC-7 schema fields (`*_ms`, `bundle_size` object with
   `*_bytes` subfields) consistent across AC-7 body, Tasks, File Structure. PASS.
4. **OQ resolution completeness:** OQ-8 addressed by AC-1+AC-7 measurement target.
   OQ-6 correctly deferred to S-8.09. OQ-1 assumption_validations confirmed. PASS.
5. **BC anchor coherence:** behavioral_contracts=[] with D-2 Option C [process-gap]
   disclosure. v1.1 BC candidates correctly formatted. PASS.
6. **Edge case coverage:** 7 ECs cover main failure modes. F-P4-002 identifies missing
   EC-008 (forward-reference for ≈0 baseline). LOW finding.
7. **Task sequencing:** A.0 pre-flight → A.1..A.7 measurement + schema output →
   B.1..B.5 BC-anchor → C.1..C.2 file structure verification. Logical order confirmed.
   PASS.
8. **Library & Framework completeness:** hyperfine REQUIRED status correct; jq
   justified via Task A.7; bats as harness. F-P4-001 identifies install-path
   inconsistency vs Task A.0. LOW finding.
9. **Cross-subsystem disclosure:** CAP-022 stretch disclosure present. SS-01+SS-07
   subsystems declared. PASS.
10. **Changelog completeness:** v1.0/v1.1/v1.2/v1.3 entries present in ascending
    order; no ordering defects from fix-burst injection. PASS.

Summary: 8 PASS, 2 findings (F-P4-001 LOW, F-P4-002 LOW). Self-validation confirms no
HIGH or MED defects missed.

---

## Part F — Novelty Assessment

All 3 findings are second-order propagation gaps from prior fix bursts:

- F-P4-001: propagation gap from F-P3-005 (install-order reorder). Install-order fix
  propagated to Task A.0 but not to Library table. Standard partial-fix propagation.
- F-P4-002: latent forward-reference that was not addressed by any prior pass. The ≈0
  baseline value implication was implicit in the blocking relationship but never
  documented. First surfaced at pass-4 (pass-1/2/3 did not probe EC coverage for
  forward-reference disclosure depth).
- F-P4-003: propagation gap from F-P2-004 (measurement-method ban). Ban propagated to
  AC-1 + Task A.0 but not to Token Budget overhead row. Standard partial-fix
  propagation.

Fresh-Context Compounding Value: pass-3 reviewed the same content and missed all 3
findings. Pass-4 fresh-context approach surfaced them by probing different document
sections (Library table, Edge Cases coverage, Token Budget).

The 50% decay (6→3) at pass-4 confirms late-convergence residue behavior consistent
with ADR-013 clock criteria. Trajectory 14→8→6→3 matches expected late-convergence
pattern.

---

## Part G — Process-Gap Tags

No new process-gap tags identified. All 3 findings are partial-fix-regression class
already covered by S-7.01 lessons (Fresh-Context Compounding Value) and S-7.03
skip-fix discipline.

**Tags:** none

---

## Part H — Priority Fix Order

Fix burst order (all optional per S-7.03 skip-fix at NITPICK_ONLY verdict; applied
for cleanliness):

1. F-P4-003 [NIT] — Token Budget `time`→`hyperfine` (1-word substitution; lowest
   complexity; confirms F-P2-004 closure propagation complete)
2. F-P4-001 [LOW] — Library table install paths (adds 2 rows to existing table row;
   enumerate apt-get / brew / cargo in priority order)
3. F-P4-002 [LOW] — EC-008 bundle directory forward-reference disclosure (new EC entry;
   ~3 sentences; [process-gap] tag; highest content value of the three)

Regression sweep required after fix burst: verify (a) Library table apt-get/brew/cargo
paths all present; (b) EC count 7→8; (c) no remaining `time` references in Token Budget
or measurement-method sections.

---

## Verdict

**NITPICK_ONLY** — 0 HIGH + 0 MED + 2 LOW + 1 NIT. No substantive defects.

**Clock: 0_of_3 → 1_of_3 ADVANCED** per ADR-013 §4.

Fix burst applied (OPTIONAL per S-7.03; applied for cleanliness):
- F-P4-001 [LOW]: Library table apt-get / brew / cargo 3-path priority order added
- F-P4-002 [LOW]: EC-008 added with [process-gap] forward-reference disclosure
- F-P4-003 [NIT]: Token Budget overhead `time`→`hyperfine`

Story moves to v1.4 (510→512 lines).

**Pass-5 priors:**
- Regression sweep for F-P4-001/002/003 closures (verify apt-get in Library table;
  EC-008 present; no `time` in Token Budget)
- Probe: EC-008 wording precision (≈0 vs exactly 0; confirm "reference state" language
  is unambiguous)
- Probe: Task A.7 schema validation — does it account for ≈0 `all_hook_plugins_wasm_bytes`
  at baseline? (May be a follow-on to F-P4-002 if EC-008 fix introduces schema-coherence
  question)
- Full fresh-context re-read; no anchoring to pass-4 findings

**Expected pass-5 outcome:** NITPICK_ONLY + clock 1_of_3 → 2_of_3. CONVERGENCE_REACHED
possible at pass-6 if pass-5 clean (clock 2_of_3 → 3_of_3 per ADR-013).

Trajectory so far: **14→8→6→3** (79% total decay).
