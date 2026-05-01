---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 68f3d16
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.00 v1.0
target_file: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 14
findings_high: 3
findings_med: 7
findings_low: 4
findings_nit: 0
---

# Adversarial Review: S-8.00 v1.0 (Pass 1)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix — `S8P1` for this S-8.00 pass-1 review
- `<PASS>`: Two-digit pass number (`P01`)
- `<SEV>`: Severity abbreviation (`HIGH`, `MED`, `LOW`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples used here: `ADV-S8P1-P01-HIGH-001`, `ADV-S8P1-P01-MED-001`, etc.

## Part A — Fix Verification (pass >= 2 only)

Not applicable — this is pass 1.

## Part B — New Findings (or all findings for pass 1)

**Target:** `.factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md` (v1.0, 444 lines)
**Verdict:** SUBSTANTIVE — fix burst required before pass-2
**Clock:** 0_of_3 → 0_of_3 (held; SUBSTANTIVE pass per ADR-013 does not advance the clock)
**Findings:** 14 total — 3 HIGH + 7 MED + 4 LOW + 0 NIT

### Executive Summary

S-8.00 v1.0 is a well-structured pre-work story with dual scope (perf baseline + BC-anchor
verification), but pass-1 surfaces four patterns of drift requiring fix-burst resolution:

1. **Anchor / subsystem mismatch (HIGH):** The story lists `subsystems: [SS-01, SS-07]` but
   anchors CAP-022 (SS-07), which spans the CLI/bin layer rather than the Hook Dispatcher Core
   or Hook Bash Layer in the way the story scopes it. The disclosure does not map cleanly to the
   CAP-022 definition in `specs/domain-spec/capabilities.md`.

2. **STORY-INDEX priority drift (HIGH):** Line 164 of STORY-INDEX.md lists S-8.00 as `P1`, but
   the story frontmatter (line 32) and the E-8 epic frontmatter (line 11) both list it as `P2`.
   Priority mismatch between index and source creates routing ambiguity for planners.

3. **AC-7 identifier conflation (HIGH):** Multiple locations in the story use "E-8 AC-7" and
   "E-8 AC-7b" interchangeably with "S-8.00 AC-7" — the story's own AC-7. This conflation
   across two namespaces (the epic's AC-7 and the story's own AC-7) causes definitional drift.

4. **Numbering / trace inconsistencies (MED cluster):** Tasks use a flat global numbering
   scheme (Task 1..N) that collides when readers cross-reference subsections; AC-4 trace target
   references a stale task number; the BC-7.00 sub-family is added without rationale; AC-1
   hook list is ambiguous (open-ended "including" language); input-hash provenance is unclear;
   AC-9 cites a BC that does not match the expected anchor.

---

### HIGH

#### ADV-S8P1-P01-HIGH-001: CAP-022 subsystem mismatch

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Story frontmatter `subsystems:` field + CAP-022 cross-reference in the stretch anchor disclosure paragraph
- **Description:** The story declares `subsystems: [SS-01, SS-07]`. SS-07 is the Hook Bash Layer. CAP-022 is `hook-dispatch-cli-integration` in `specs/domain-spec/capabilities.md` — it belongs to SS-10 (CLI Tools and Bin), not SS-07. Using CAP-022 as the stretch anchor while claiming SS-07 creates a cross-subsystem misrepresentation: SS-07 governs the bash hook scripts themselves; CAP-022 governs the CLI integration layer that invokes them.
- **Evidence:** `specs/domain-spec/capabilities.md` CAP-022 entry: `subsystem: SS-10`. Story frontmatter: `subsystems: [SS-01, SS-07]`. CAP-022 disclosure paragraph in story references this capability as the stretch anchor within "SS-07 scope."
- **Proposed Fix:** Either (a) add SS-10 to `subsystems:` and add the cross-CAP stretch disclosure paragraph per F-204 sanctioned-template-anchor pattern (Wave 7/8 precedent), or (b) replace the CAP-022 stretch anchor with a CAP that belongs to SS-01 or SS-07. Given the story's actual scope (perf baseline for bash hooks + BC-anchor verification for bash-layer hooks), a Wave-7-pattern stretch-anchor disclosure of "CAP-022 consumed but not owned; SS-10 not in subsystems" is appropriate and precedented.
- **Policy:** POLICY 2 (subsystem accuracy), POLICY 9 (cross-subsystem disclosure per F-204).

#### ADV-S8P1-P01-HIGH-002: STORY-INDEX priority drift

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` line 164, Priority column
- **Description:** STORY-INDEX lists S-8.00 priority as `P1`. The story file frontmatter line 32 reads `priority: P2`. The E-8 epic frontmatter line 11 reads `priority: P2` for all pre-work stories. The P1/P2 discrepancy means STORY-INDEX overrides the story's own declared priority, misleading wave schedulers.
- **Evidence:** `STORY-INDEX.md:164` — `P1` in Priority column. `S-8.00:32` — `priority: P2`. E-8 epic frontmatter — pre-work stories listed at P2.
- **Proposed Fix:** Correct STORY-INDEX line 164 Priority column from `P1` to `P2`. Bump STORY-INDEX version. (State-manager scope.)
- **Policy:** POLICY 5 (index accuracy), POLICY 6 (frontmatter-index coherence).

#### ADV-S8P1-P01-HIGH-003: AC-7 identifier conflation

- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** Multiple — Goal §2, AC-3 row, AC-7 row heading, Tasks section
- **Description:** The story uses "E-8 AC-7" and "E-8 AC-7b" in several places (Goal §2, AC-3 trace) when it means the epic-level acceptance criterion. But the story itself has an AC-7 of its own (bundle-size measurement). This creates a three-way identifier collision: (1) S-8.00 AC-7 (the story's own); (2) E-8 AC-7 (the epic's acceptance criterion); (3) E-8 AC-7b (a sub-criterion within the epic's AC-7). AC-3's trace target says "E-8 AC-7b" when it likely means "E-8 epic-level AC-7b" — but a reader seeing "AC-7b" in an S-8.00 document first looks for S-8.00 AC-7b, which does not exist.
- **Evidence:** Goal §2 line 201: "E-8 epic E-8 AC-7b's target". AC-3 line 229: "per E-8 epic E-8 AC-7b". AC-7 row heading uses same identifier space as story-level ACs.
- **Proposed Fix:** Sweep all uses of "E-8 AC-7" and "E-8 AC-7b" in the story and qualify them unambiguously as "[E-8 epic AC-7]" / "[E-8 epic AC-7b]" to distinguish from S-8.00's own AC-7. The AC-7 row heading in the story itself should be clarified: "(S-8.00 AC-7: bundle-size measurement)" to prevent future pass confusion.
- **Policy:** POLICY 1 (identifier uniqueness), POLICY 10 (cross-document reference precision).

---

### MEDIUM

#### ADV-S8P1-P01-MED-001: STORY-INDEX title missing "Tier 1" qualifier

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` line 164, Title column
- **Description:** STORY-INDEX title reads "Perf benchmark baseline + BC-anchor verification (W-15 pre-work)". The story's own H1 reads "S-8.00: Perf benchmark baseline + Tier 1 BC-anchor verification". The "Tier 1" qualifier is load-bearing: it distinguishes this story's scope (verifying the 9 Tier 1 hooks) from a hypothetical broader BC-anchor scope.
- **Evidence:** Story H1 line 1: "Tier 1 BC-anchor verification". STORY-INDEX line 164 title: no "Tier 1" qualifier.
- **Proposed Fix:** Update STORY-INDEX line 164 Title to match story H1: "Perf benchmark baseline + Tier 1 BC-anchor verification (W-15 pre-work)". (State-manager scope.)
- **Policy:** POLICY 5 (index-title accuracy).

#### ADV-S8P1-P01-MED-002: Tasks numbering collision

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** Tasks section — subsections A, B, C each restart within flat global numbering
- **Description:** The story has three task subsections (A: Perf baseline; B: BC-anchor verification; C: Acceptance criteria verification). Within each subsection, tasks share a flat global number sequence (Task 1, Task 2, … Task N across all subsections). "Task 3" is ambiguous when referenced from AC-4 or AC-9 — a reader cannot tell which subsection's Task 3 is meant.
- **Evidence:** AC-4 references "Task N" without subsection qualifier. Per-section numbering (A.1, A.2, B.1, B.2) is canonical per S-5.05 and S-5.06 precedent.
- **Proposed Fix:** Renumber tasks per-section: A.1..A.6, B.1..B.5, C.1..C.2. Update all intra-story references (AC-4 "see Task N" → "see Tasks §B.1").
- **Policy:** POLICY 7 (internal reference precision).

#### ADV-S8P1-P01-MED-003: AC-4 trace target wrong

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-4 row, trace/reference column
- **Description:** AC-4 references "Task 8" for the BC-anchor verification table delivery. After task renumbering (ADV-S8P1-P01-MED-002), Task 8 no longer exists at that location. Even before renumbering, "Task 8" in the flat sequence refers ambiguously within subsection B.
- **Evidence:** AC-4 trace: "Task 8". BC-anchor verification starts at subsection B; the first task of B would be B.1 after renumbering.
- **Proposed Fix:** After applying MED-002 renumbering, update AC-4 trace to "Tasks §B.1". (Linked to MED-002.)
- **Policy:** POLICY 7 (internal reference precision).

#### ADV-S8P1-P01-MED-004: BC-7.00 sub-family unjustified

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** v1.1 BC candidates section; references to BC-7.00.001 and BC-7.00.002
- **Description:** The story introduces BC-7.00 as a new sub-family within SS-07. Existing SS-07 BC sub-families are BC-7.01 through BC-7.09. Adding BC-7.00 is unusual — it implies a foundational sub-family below the existing BC-7.01 anchor with no rationale for why .00 rather than a new .10 extension, and no explanation of whether BC-7.00.001 (perf-baseline-measurement-protocol) is truly a testable behavioral contract.
- **Evidence:** `specs/behavioral-contracts/ss-07/` directory: no existing BC-7.00.* files. Story registers BC-7.00.001 + BC-7.00.002 as v1.1 candidates with no sub-family creation rationale.
- **Proposed Fix:** Add a rationale paragraph for the BC-7.00 sub-family introduction. Confirm via grep that BC-7.00 is unused in existing specs. Note the creation decision in the v1.1 BC candidates table rationale column.
- **Policy:** POLICY 2 (BC namespace coherence), POLICY 8 (new artifact justification).

#### ADV-S8P1-P01-MED-005: AC-1 hook list ambiguity

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** AC-1 row, hook-list enumeration
- **Description:** AC-1 lists 9 Tier 1 hooks "including" [list]. The word "including" implies the list is non-exhaustive — but AC-1 is the acceptance criterion for complete Tier 1 hook coverage. If the list is exhaustive, use "specifically" or "namely" or an explicit count.
- **Evidence:** AC-1: "including handoff-validator, pr-manager-completion-guard, …" (open-ended qualifier on a supposedly exhaustive list).
- **Proposed Fix:** Lock the AC-1 hook list: replace "including" with "specifically" or equivalent. Add explicit count "(9 hooks)".
- **Policy:** POLICY 1 (acceptance criterion precision).

#### ADV-S8P1-P01-MED-006: Input-hash provenance missing

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Story frontmatter `input-hash:` field; changelog entry
- **Description:** The story uses `input-hash: 68f3d16` (corrected post-authoring by state-manager in D-164 from 4ba3584). The story body does not include a comment block explaining what `68f3d16` refers to (last factory-artifacts commit touching E-8 file). Without provenance documentation, a future reader cannot verify whether the input-hash is current.
- **Evidence:** Frontmatter: `input-hash: 68f3d16`. No comment or narrative explaining hash source. D-164 state-manager notes document the correction but are external to the story file.
- **Proposed Fix:** Add an input-hash provenance comment: "68f3d16 = last factory-artifacts commit touching E-8-native-wasm-migration.md (verified 2026-04-30 by state-manager correction from 4ba3584)."
- **Policy:** ADR-013 (input-hash currency discipline).

#### ADV-S8P1-P01-MED-007: AC-9 BC-8.26.001 mis-anchor

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-9 row, behavioral_contracts trace
- **Description:** AC-9 anchors to BC-8.26.001 (Templates and Rules subsystem, SS-08). AC-9 concerns verification of BC-anchor table delivery — a process-governance concern that maps to the [process-gap] D-2 Option C disclosure, not to SS-08 template-compliance BCs. BC-8.26.001 is the "template-compliance verification" BC — not BC-anchor mapping verification. The anchor appears to be a carry-over from sibling stories S-5.05/S-5.06.
- **Evidence:** AC-9 behavioral_contracts: `[BC-8.26.001]`. Story frontmatter: `behavioral_contracts: []` with [process-gap] disclosure. BC-8.26.001 scope: document template adherence (SS-08), not process-integrity verification.
- **Proposed Fix:** Replace BC-8.26.001 anchor in AC-9 with [process-gap] disclosure pattern: "behavioral_contracts: [] ([process-gap] — BC-anchor verification is a process governance step; E-8 D-2 epic-fix-burst-integrity is the downstream anchor)."
- **Policy:** POLICY 2 (BC anchor accuracy), POLICY 9 (process-gap disclosure).

---

### LOW

#### ADV-S8P1-P01-LOW-001: policies.yaml cross-ref format inconsistent

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Story's policies compliance section / references
- **Description:** Cross-references to `policies.yaml` entries use the informal format "POLICY N (short-name)" without citing the canonical key as defined in `.factory/policies.yaml`. The short-name may drift if the registry key is renamed.
- **Proposed Fix:** Use canonical key format: "POLICY `enforce_subsystem_accuracy`" etc., or establish a convention table mapping POLICY N → canonical key (as S-5.05 uses).

#### ADV-S8P1-P01-LOW-002: EC-004 majority-threshold rationale missing

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** Edge Cases, EC-004
- **Description:** EC-004 states "if fewer than N/2 hooks have measurable BC coverage, the story is blocked." The "N/2" threshold is not derived from any referenced policy or decision. A reader cannot determine whether N/2 is a hard requirement from D-2 Option C or a heuristic.
- **Proposed Fix:** Add parenthetical: "(majority-threshold per [E-8 D-2 Option C heuristic / author-discretion — flag as v1.1 candidate for formalization])."

#### ADV-S8P1-P01-LOW-003: STORY-INDEX E-8 transitional points not annotated

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` total-points line (line 181)
- **Description:** STORY-INDEX total reads "219 across 48 stories (190 E-0..E-5 + 3 E-6 + 21 E-7 + 5 E-8)". The 5 E-8 points represent only S-8.00. The remaining 28 E-8 stories (S-8.01..S-8.28, ~118 additional pts) are planned but not yet authored. The subtotal is transitional and could mislead if read as E-8 complete.
- **Proposed Fix:** Add footnote: "Total story points: 219 across 48 stories (190 E-0..E-5 + 3 E-6 + 21 E-7 + 5 E-8*) *E-8 in progress — only S-8.00 authored at 5pts; ~118 additional pts pending S-8.01..S-8.28." (State-manager scope.)

#### ADV-S8P1-P01-LOW-004: EC-007 git command underspecified

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** Edge Cases, EC-007
- **Description:** EC-007 instructs a git command for verifying input-hash currency but does not specify the exact git command, log format, depth, or file path. `git log --oneline -1` vs `git log --format=%h -1 -- <file>` produce different outputs.
- **Proposed Fix:** Specify exact command: `git log --format=%h -1 -- .factory/stories/epics/E-8-native-wasm-migration.md` and note "abbreviated SHA (7 chars) for comparison against frontmatter input-hash field."

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 7 |
| LOW | 4 |

**Overall Assessment:** pass-with-findings (SUBSTANTIVE)
**Convergence:** findings remain — fix burst required; iterate to pass-2
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (14/14 — pass 1 baseline) |
| **Median severity** | MED (7 of 14 findings) |
| **Trajectory** | 14→? |
| **Verdict** | FINDINGS_REMAIN |

**Clock:** 0_of_3 → 0_of_3 (held — SUBSTANTIVE verdict per ADR-013 does not advance the
convergence clock).

**Priority fix order for story-writer:**
1. ADV-S8P1-P01-HIGH-001 (CAP-022 stretch disclosure) — affects subsystems field + cross-CAP paragraph
2. ADV-S8P1-P01-HIGH-003 (AC-7 identifier sweep) — affects Goal §2, AC-3, AC-7 row, Tasks
3. ADV-S8P1-P01-MED-002 + MED-003 (Tasks renumbering + AC-4 trace) — mechanical but broad
4. ADV-S8P1-P01-MED-004 (BC-7.00 sub-family rationale) — grep-verify + paragraph
5. ADV-S8P1-P01-MED-005 (AC-1 hook list lock) — one-line qualifier change
6. ADV-S8P1-P01-MED-006 (input-hash provenance) — comment block addition
7. ADV-S8P1-P01-MED-007 (AC-9 BC-8.26.001 replacement) — [process-gap] pattern
8. ADV-S8P1-P01-LOW-004 (EC-007 git command) — specify exact command
9. ADV-S8P1-P01-LOW-001 (policies.yaml cross-ref) — standardize format
10. ADV-S8P1-P01-LOW-002 (EC-004 threshold rationale) — parenthetical addition

**Priority fix order for state-manager:**
1. ADV-S8P1-P01-HIGH-002 (STORY-INDEX priority P1→P2)
2. ADV-S8P1-P01-MED-001 (STORY-INDEX title "Tier 1" restore)
3. ADV-S8P1-P01-LOW-003 (STORY-INDEX E-8 transitional-points footnote)

**Pass-2 priors (feedstock for adversary at pass-2; NOT fix-burst items):**
- AC-3 line 229: "E-8 epic E-8 AC-7b" double-prefix (redundant phrasing from HIGH-003 sweep)
- Goal §2 line 201: "E-8 epic E-8 AC-7b's" double-prefix (same root)
- AC-7 row heading: consider clarification "(S-8.00 AC-7: bundle-size measurement)"
