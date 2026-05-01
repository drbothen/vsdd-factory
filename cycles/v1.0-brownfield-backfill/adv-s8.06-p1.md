---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.06-native-port-session-learning.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.06 v1.0
target_file: .factory/stories/S-8.06-native-port-session-learning.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 11
findings_high: 4
findings_med: 5
findings_low: 1
findings_nit: 1
---

# Adversarial Review: S-8.06 v1.0 (Pass 1)

## Finding ID Convention

`F-S806-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S806-P1-001: AC-002 specifies deletion of hooks.json entries that do not exist (DRIFT-004 already in AFTER state)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002
- **Description:** AC-002 prescribes deletion of hooks.json command entries for session-learning. DRIFT-004 already moved the dispatch mechanism to hooks-registry.toml (native format). The hooks.json entries targeted by AC-002 do not exist in the current codebase. Same class of phantom-deletion defect as F-S801-P1-001.
- **Evidence:** DRIFT-004 "already in AFTER state" per E-8 epic; hooks-registry.toml is sole dispatch mechanism.
- **Proposed Fix:** Remove the hooks.json deletion AC or rewrite as verification: "assert hooks.json does NOT contain session-learning entries."

#### F-S806-P1-002: BC-7.03.076 description contradicts AC-001 binary_allow=empty postcondition

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** AC-001 vs BC-7.03.076
- **Description:** AC-001 specifies the post-port binary_allow as empty (all bash/jq/gh removed). BC-7.03.076 description says "minimal binary_allow=[bash]" — mandating bash remains. This is a direct contradiction between the story's AC and its BC anchor.
- **Evidence:** AC-001: "binary_allow=[]". BC-7.03.076 description: "minimal binary_allow=[bash]". Mutually exclusive.
- **Proposed Fix:** Align AC-001 and BC-7.03.076. If the intent is to remove all binaries, update BC-7.03.076 description. If bash must remain for some reason, update AC-001 to reflect that.

#### F-S806-P1-003: POLICY 8 violation — AC-006 fabricates "emit_event deferral" claim for BC-7.03.076 invariant 2

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-006 → BC-7.03.076 invariant 2
- **Description:** AC-006 traces to "BC-7.03.076 invariant 2 (exit-code semantics + emit_event deferral)." BC-7.03.076 invariant 2 covers only exit-code semantics. The "emit_event deferral" parenthetical is not present in BC-7.03.076 invariant 2 — it is a fabricated claim that attributes behavior to a BC clause that does not contain it.
- **Evidence:** BC-7.03.076 invariant 2 text: exit-code semantics only. No emit_event deferral content.
- **Proposed Fix:** Remove the fabricated "emit_event deferral" parenthetical from AC-006's BC trace. If emit_event deferral needs grounding, find or create the correct BC.

#### F-S806-P1-004: Six ACs over-decompose a 3-pt single-file-append port

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC list
- **Description:** The story is a 3-point single-file-append port. Six ACs is atypically high for this complexity level and creates excessive test surface for a straightforward operation. The BC count (3 BCs) maps more naturally to 3 ACs. Over-decomposition may create conflicting or redundant test cases.
- **Evidence:** Comparable 3-pt siblings (S-8.03, S-8.07, S-8.08) have 5-7 ACs; S-8.06 has 6 ACs for what the goal describes as a single-file-append operation with emit_event.
- **Proposed Fix:** Review AC list for consolidation opportunities. At minimum, confirm that each AC maps to a distinct testable behavior (not just a description layer).

### MEDIUM

#### F-S806-P1-005: input-hash 5015917 not derived per documented convention

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash comment
- **Description:** The input-hash comment text is inconsistent: the verify command references the E-8 epic (correct per convention) but the comment text says "S-8.00 commit" (incorrect). Convention is E-8 epic content hash.
- **Proposed Fix:** Update input-hash comment to say "E-8 epic content hash — shared across Tier 1 siblings per convention."

#### F-S806-P1-006: AC-005 conflates behavior parity bats with perf measurement gate

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-005
- **Description:** AC-005 folds both (a) behavior parity bats tests and (b) the 20% performance gate into a single AC. These are distinct concerns: parity tests verify functional equivalence; perf gates verify non-functional properties. Conflating them makes it unclear when the AC passes (parity only? parity + perf?). Additionally, E-8 AC-7 excludes Tier 1 from the perf gate.
- **Proposed Fix:** Split AC-005 into separate concerns. Remove or mark INFORMATIONAL the perf gate sub-requirement per E-8 AC-7 Tier 1 exclusion.

#### F-S806-P1-007: Goal/Task/AC describe host::write_file vs Rust std path ambiguity

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** Goal / T-4 / AC-003
- **Description:** The story uses both "host::write_file" and "std::fs::write" interchangeably for the file-append operation. For a WASI hook, file I/O must use the host function (host::write_file, once available via SDK extension) or WASI fd_write — not std::fs::write which may not be available in the WASM sandbox. The story must lock the choice.
- **Evidence:** As established in F-S804-P1-001, host::write_file does not currently exist in the SDK. std::fs may work in WASI but is not the canonical SDK path.
- **Proposed Fix:** Specify the file I/O mechanism: (a) host::write_file (pending SDK extension D-6 Option A) or (b) WASI std::fs (if allowed by the sandbox model). Lock the choice explicitly.

#### F-S806-P1-008: EC-005 stdin handling contradicts WASI invocation model

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-005
- **Description:** EC-005 says "skip reading stdin entirely" for the stdin-empty edge case. In the WASI invocation model, skipping stdin read entirely may cause SIGPIPE-like failures depending on how the dispatcher passes input. The correct behavior should be to read and discard stdin rather than skip the read entirely.
- **Proposed Fix:** Update EC-005 to specify: "read and discard stdin if empty; do not skip the stdin read call to avoid WASI SIGPIPE-equivalent failures."

#### F-S806-P1-009: Stretch-anchor disclosure mathematics inconsistent (CAP-022 subsystems)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Stretch-Anchor Disclosure
- **Description:** The stretch-anchor disclosure claims CAP-022 spans SS-04/SS-06. However, the story's own subsystems field lists SS-01/SS-07. The two references are inconsistent.
- **Proposed Fix:** Align the CAP-022 subsystem references in the stretch-anchor disclosure with the story's actual subsystems field.

### LOW

#### F-S806-P1-010: wasm32-wasi target deprecated — should use wasm32-wasip1

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** T-2 / Library & Framework Requirements
- **Description:** Same as F-S804-P1-002. wasm32-wasi is deprecated; correct target is wasm32-wasip1.
- **Proposed Fix:** Replace wasm32-wasi with wasm32-wasip1 throughout.

### NIT

#### F-S806-P1-011: AC-003 header trailing-blank-line inconsistency vs bash source

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** AC-003
- **Description:** AC-003 specifies the session-learning file append format with a trailing blank line. The bash source behavior for trailing blank lines is not explicitly specified in the AC, creating a minor ambiguity about whether the WASM output matches exactly.
- **Proposed Fix:** Specify exact trailing whitespace/newline behavior in AC-003.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 5 |
| LOW | 1 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 11 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 11/11 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 11 |
| **Verdict** | FINDINGS_REMAIN |
