---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.02-native-port-pr-manager-completion-guard.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.02 v1.0
target_file: .factory/stories/S-8.02-native-port-pr-manager-completion-guard.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 13
findings_high: 4
findings_med: 5
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.02 v1.0 (Pass 1)

## Finding ID Convention

`F-S802-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S802-P1-001: POLICY 4 mis-anchor — AC-005 step-1 hint string truncated vs bash source

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-005 step-1 hint string
- **Description:** The hint string specified in AC-005 for step 1 reads "populate PR description" (truncated). The actual bash source string is "populate PR description from template." The WASM port must produce exactly the bash hint strings to preserve behavior parity; a truncated string is a detectable behavioral difference.
- **Evidence:** Bash source `case 1)` arm contains the full string; the story truncates it.
- **Proposed Fix:** Replace the truncated string with the full bash string "populate PR description from template."

#### F-S802-P1-002: POLICY 4 — AC-005 omits catch-all step (NEXT_STEP > 9) wildcard hint arm

- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Location:** AC-005
- **Description:** AC-005 enumerates "9 step-specific hint strings" but the bash source has 10 case arms: 9 step-specific arms plus a catch-all `*)` wildcard arm that emits a generic hint for NEXT_STEP values outside 1-9. The WASM port must implement the catch-all arm to preserve parity.
- **Evidence:** bash `case $NEXT_STEP in ... *) echo "..." ;;` catch-all arm is present in source; spec omits it.
- **Proposed Fix:** Add the catch-all (NEXT_STEP > 9 / unknown) arm to AC-005 as a 10th case, with the exact hint string from bash source.

#### F-S802-P1-003: POLICY 8 — BC-7.03.046 title mismatch ASCII ">=8" vs Unicode "≥8"

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Body BC table entry for BC-7.03.046
- **Description:** The body BC table uses ASCII `>=8` in the BC-7.03.046 title. The canonical BC file uses Unicode `≥8`. While semantically equivalent, POLICY 7 (naming consistency) requires exact string matching for BC titles in cross-references to avoid grep/search mismatches.
- **Proposed Fix:** Update the body BC table to use the canonical Unicode `≥8` form from the BC file title.

#### F-S802-P1-004: jq-missing graceful-degradation contract missing; BC-7.03.045 invariant 2 contradicts bash exit-0

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** AC behavior vs BC-7.03.045 invariant 2
- **Description:** BC-7.03.045 invariant 2 states "1=jq-missing-fail-closed" but the bash source exits 0 when jq is absent (graceful degradation). The story does not include an explicit AC for jq-missing behavior, leaving the WASM port without a specified exit-code contract for this path. Same class of defect as F-S801-P1-003.
- **Evidence:** Bash: graceful exit 0 on jq absent; BC-7.03.045 invariant 2: exit 1 fail-closed. Direct contradiction.
- **Proposed Fix:** Add an explicit AC for jq-missing behavior and align the BC-7.03.045 invariant 2 with the chosen exit-code semantics. Justify whichever is selected (fail-closed or graceful degradation).

### MEDIUM

#### F-S802-P1-005: STEP_COMPLETE counting input source ambiguous

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** EC-003 / AC-004 input source specification
- **Description:** The story does not pin the fallback chain for `STEP_COMPLETE` count extraction: whether to use `last_assistant_message` first or `result` first, and what constitutes a null/missing value. The bash source has a `?? null` fallback chain; the spec omits it.
- **Proposed Fix:** Specify the exact fallback chain: `last_assistant_message ?? result ?? 0` (or whichever matches bash) and define the null-check semantics explicitly in AC-004/EC-003.

#### F-S802-P1-006: Pattern-match grammar inconsistent — T-3 "pr.manager" (BRE typo) vs bash glob

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** T-3 pattern specification
- **Description:** T-3 specifies the hook-name pattern as `pr.manager` (dot as literal, BRE-style). The bash source uses glob `*pr-manager*|*pr_manager*` (ERE alternation with two variants). The dot in `pr.manager` would match `pr_manager` only in ERE and is not equivalent to the bash glob.
- **Proposed Fix:** Replace T-3 pattern with the canonical glob/regex from bash: `*pr-manager*|*pr_manager*` or equivalent Rust regex, explicitly stated.

#### F-S802-P1-007: Regex anchoring: story uses "\\|" (BRE escaped) but bash uses "|" (ERE alternation)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** BLOCKED regex specification
- **Description:** The story uses `\|` (backslash-pipe, BRE-style escaped alternation). Bash uses `|` (unescaped pipe, ERE alternation). In Rust regex, `|` is ERE alternation. Using `\|` in a Rust regex is a parse error or matches a literal backslash-pipe, not alternation.
- **Proposed Fix:** Update all regex patterns in the spec to use ERE `|` alternation syntax, consistent with Rust's regex crate.

#### F-S802-P1-008: BC-7.03.045 line-range citation stale (839-856 vs actual 904-921)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Body BC table line reference for BC-7.03.045
- **Description:** The body BC table cites `hooks-registry.toml:839-856`. The actual line range is 904-921.
- **Proposed Fix:** Update citation to 904-921, or replace with stable anchor comment.

#### F-S802-P1-009: Capability anchor justification "gh CLI subprocess" contradicts bash source

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Capability Anchor Justification
- **Description:** The capability anchor justification claims "gh CLI subprocess invocation" as justification for SS-07 ownership. The bash source does not call `gh` — the hook reads environment variables and writes to stdout/stderr. This claim is factually incorrect and would mislead the implementer about required binary_allow entries.
- **Proposed Fix:** Correct the capability justification to accurately describe what the hook does (reads env vars, writes output) without fabricating a gh subprocess claim.

### LOW

#### F-S802-P1-010: POLICY 5 — SS-07 ownership claim inconsistent with .sh-file-deleted outcome

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Stretch-Anchor Disclosure / subsystems field
- **Description:** The story claims SS-07 ownership but the delivery outcome deletes the .sh file and moves the hook to SS-01 (WASM dispatcher). Post-port, the .sh no longer exists in SS-07's scope. SS-01 and SS-04 are the correct post-port owners.
- **Proposed Fix:** Update subsystems to include SS-01 and SS-04; clarify that SS-07 ownership applies only during the pre-port phase.

#### F-S802-P1-011: Token Budget pr-manager-completion-guard.sh line count understated

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget section
- **Description:** The Token Budget understates the line count of pr-manager-completion-guard.sh. The actual file has more lines than specified, affecting context budget planning.
- **Proposed Fix:** Update Token Budget with accurate line count from current bash source.

#### F-S802-P1-012: input-hash convention diverges from S-8.00 sibling

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash
- **Description:** S-8.00 established the input-hash convention as the E-8 epic content hash. S-8.02 uses a different derivation (commit hash or other). All Tier 1 siblings should follow the same convention.
- **Proposed Fix:** Align input-hash derivation with S-8.00 convention.

### NIT

#### F-S802-P1-013: Architecture Compliance Rules forbidden-dependency rule has cosmetic typo

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** Architecture Compliance Rules — forbidden dependency rule
- **Description:** Minor typo in the forbidden dependency rule text; cosmetic, no behavioral impact.
- **Proposed Fix:** Correct typo in next fix burst.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 5 |
| LOW | 3 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 13 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 13/13 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 13 |
| **Verdict** | FINDINGS_REMAIN |
