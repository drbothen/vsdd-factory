---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.05 v1.0
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 12
findings_high: 4
findings_med: 5
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.05 v1.0 (Pass 1)

## Finding ID Convention

`F-S805-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S805-P1-001: Behavior parity bug — AC-006 Check 3a inverts bash verdict accumulation semantics

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-006 Check 3a
- **Description:** AC-006 Check 3a describes verdict accumulation logic that inverts the bash semantics. The bash implementation accumulates verdicts as: if review state is NOT "approved", increment the non-approved counter. AC-006 Check 3a describes the condition in the opposite sense, which would cause the WASM port to produce inverted results (counting approvals instead of non-approvals, or vice versa).
- **Evidence:** Bash source logic: `[[ $state != "approved" ]]` increments counter. AC-006 Check 3a: wording implies the opposite condition.
- **Proposed Fix:** Rewrite AC-006 Check 3a to faithfully mirror the bash verdict accumulation: increment counter when review state is NOT approved; emit block verdict if counter >= threshold.

#### F-S805-P1-002: Behavior parity bug — stderr formatting omits third remediation line

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-006 / EC stderr format specification
- **Description:** The bash source emits 3 lines to stderr on block: (1) a header line, (2) a count/threshold summary, (3) a remediation instruction with the PR URL. AC-006 and the EC specify only 1 "instruction" line. The WASM port will emit incomplete error output, breaking any downstream tools or human reviewers that rely on the full 3-line format.
- **Evidence:** Bash source: 3 distinct `echo >&2` calls on block path.
- **Proposed Fix:** Expand AC-006 stderr specification to include all 3 lines with exact format strings from the bash source.

#### F-S805-P1-003: Tasks omit explicit removal of `gh` from binary_allow

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Task list — capability migration
- **Description:** The story was flagged by the story-writer as requiring explicit removal of `gh` from `binary_allow` during the WASM port. The Task list does not include this step. The implementer must infer it, risking omission.
- **Evidence:** Story-writer prior flag; current bash hook has `gh` in binary_allow; WASM hook does not invoke gh.
- **Proposed Fix:** Add an explicit task: "Remove `gh` from binary_allow in hooks-registry.toml entry for validate-pr-review-posted."

#### F-S805-P1-004: Tasks omit explicit removal of `bash` and `jq` from binary_allow

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Task list — capability migration
- **Description:** Same class as F-S805-P1-003. The Task list omits explicit removal of `bash` and `jq` from binary_allow. All three binaries (bash, jq, gh) must be removed when porting to WASM; only gh is called out.
- **Proposed Fix:** Add tasks for removing `bash` and `jq` from binary_allow, alongside the `gh` removal task.

### MEDIUM

#### F-S805-P1-005: Stretch-anchor disclosure cites SS-07 ownership but SS-07 will not own a deleted .sh

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Stretch-Anchor Disclosure
- **Description:** The stretch-anchor disclosure claims SS-07 ownership for the post-port state. After the WASM port, the .sh file is deleted and the hook moves to SS-01 (dispatcher) + SS-04 (plugin). SS-07 (operations/tooling) does not own the WASM plugin. This is the same pattern as F-S802-P1-010.
- **Proposed Fix:** Update stretch-anchor disclosure to correctly state SS-01 + SS-04 as post-port owners.

#### F-S805-P1-006: input-hash drift — shared hash 5015917 comment references S-8.00 commit not E-8 epic

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash comment
- **Description:** The input-hash comment says it derives from the S-8.00 commit. The convention (per S-8.00 and D-170) is to use the E-8 epic content hash. The comment diverges from the established convention.
- **Proposed Fix:** Update input-hash comment to say "E-8 epic content hash" per convention.

#### F-S805-P1-007: EC-005 contradicts bash semantics on emit failure (silent no-op vs Result::Err)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-005
- **Description:** EC-005 specifies how to handle emit_event failures. The bash source uses `bin/emit-event || true` (silent no-op on failure). EC-005 does not specify whether the WASM port should silently swallow `host::emit_event` errors or propagate them as `Result::Err`. These are different behaviors with different observability characteristics.
- **Proposed Fix:** Explicitly specify EC-005 emit failure semantics: "on host::emit_event error, log to stderr and exit 0 (matching bash OR-true semantics)."

#### F-S805-P1-008: Goal regex spec includes `Write.*pr-review` but Rust regex escape rules differ from bash

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Goal regex specification
- **Description:** The goal regex `Write.*pr-review` is specified as-is. In bash grep/awk, `.` matches any character and `*` is a quantifier. In Rust's regex crate, the same syntax is valid but case sensitivity, anchoring, and escape rules may differ from the bash context in which the original regex was used. The spec should specify whether the regex is case-sensitive and confirm Rust regex crate compatibility.
- **Proposed Fix:** Add explicit notes: regex is case-sensitive (or not), uses Rust regex crate syntax, confirm `.*` semantics match bash grep behavior.

#### F-S805-P1-009: POLICY 8 — 5 BCs referenced but body table has single-AC entries where multi-AC needed

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Body BC table vs AC traces
- **Description:** The story has 5 BCs in behavioral_contracts. Multiple ACs trace to different BCs but the body BC table does not cross-reference all AC→BC mappings. Some BCs appear in the table without the corresponding AC references that ground them.
- **Proposed Fix:** Expand the body BC table to include all AC→BC mappings for each of the 5 BCs.

### LOW

#### F-S805-P1-010: Sibling consistency — BC-7.04 sub-family vs S-8.03's BC-7.03 — verify sub-family is correct

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** behavioral_contracts frontmatter
- **Description:** S-8.05 anchors to BC-7.04.* sub-family while other Tier 1 siblings use BC-7.03.*. This is expected (different functional area) but should be explicitly verified that BC-7.04.040-044 are the correct anchors and not a mis-family error.
- **Proposed Fix:** Confirm BC-7.04.040-044 are the canonical BCs for validate-pr-review-posted (not a typo for BC-7.03.*).

#### F-S805-P1-011: bin/emit-event invocation — bash OR-true semantics vs host::emit_event undocumented

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** AC-005 / EC-005
- **Description:** The bash source uses `bin/emit-event || true` meaning emit failures are silently swallowed. The story does not document whether the WASM host::emit_event can fail (the host function may always succeed or may return a Result). The failure mode is undocumented.
- **Proposed Fix:** Add a note in AC-005 or EC-005 specifying the expected host::emit_event failure behavior based on SDK documentation.

### NIT

#### F-S805-P1-012: File Structure modify entries duplicate work — 5 platform-specific hooks.json files listed

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** File Structure section
- **Description:** The File Structure section lists 5 platform-specific hooks.json file paths as modify entries. These are the same file across 5 platforms, which is verbose. A note like "hooks.json (5 platform variants)" would be more concise.
- **Proposed Fix:** Consolidate 5 platform hooks.json entries into a single entry with a "(5 platform variants)" annotation.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 5 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 12 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 12/12 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 12 |
| **Verdict** | FINDINGS_REMAIN |
