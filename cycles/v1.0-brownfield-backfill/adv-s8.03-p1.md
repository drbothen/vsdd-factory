---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.03-native-port-track-agent-stop.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.03 v1.0
target_file: .factory/stories/S-8.03-native-port-track-agent-stop.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 13
findings_high: 4
findings_med: 5
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.03 v1.0 (Pass 1)

## Finding ID Convention

`F-S803-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S803-P1-001: Byte-count vs char-count parity defect (AC-003 / T-3)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-003 and T-3
- **Description:** The bash implementation uses `tr -d '[:space:]' | wc -c` which produces a BYTE count. The spec's T-3 guidance for the Rust port uses `.chars().filter().count()` which produces a CHAR (Unicode scalar value) count. For ASCII-only input these are equivalent, but for multibyte UTF-8 input (e.g., multi-byte Unicode characters in tool output) the counts will differ. This is a detectable parity defect for any non-ASCII input.
- **Evidence:** `wc -c` counts bytes; `.chars().count()` counts Unicode scalar values. For UTF-8 multibyte characters, byte count > char count.
- **Proposed Fix:** Either use `.as_bytes().iter().filter(|b| !b.is_ascii_whitespace()).count()` in Rust to match bash byte semantics, or document that the port intentionally switches to char semantics and update the AC accordingly.

#### F-S803-P1-002: Multiline regex anchor parity not specified (bash grep ^ per-line vs Rust default)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-003 regex specification
- **Description:** The bash implementation uses `grep` which matches `^` per-line by default. Rust's `regex` crate treats `^` as start-of-string unless the `(?m)` multiline flag is set. If the hook processes multiline input (e.g., multi-line tool output), the Rust port will silently fail to match patterns that bash matches.
- **Evidence:** Standard regex crate behavior: `^` = start of string. bash grep: `^` = start of line.
- **Proposed Fix:** Specify that the Rust regex must use the `(?m)` multiline flag, or document the equivalence proof if input is guaranteed single-line.

#### F-S803-P1-003: AC-005 BC trace mis-anchors bin/emit-event clause to BC-7.03.081 (identity) vs BC-7.03.082 (emit_event)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-005 → BC-7.03.081
- **Description:** AC-005 grounds the bin/emit-event invocation to BC-7.03.081. BC-7.03.081 is the identity/registration BC for track-agent-stop. The emit_event behavior contract is BC-7.03.082. Using BC-7.03.081 as the emit_event anchor is semantically incorrect.
- **Proposed Fix:** Re-anchor AC-005's emit_event clause to BC-7.03.082.

#### F-S803-P1-004: Token Budget claims 5 ACs but no malformed-JSON AC (sibling S-8.01 has explicit AC-006 for this)

- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Location:** Token Budget and AC list
- **Description:** The story claims 5 ACs. Sibling S-8.01 includes an explicit AC-006 for graceful malformed-JSON handling. track-agent-stop also parses JSON input (tool_use data) and the malformed-JSON edge case is absent from both the ACs and edge cases. This is a missing behavioral contract for a realistic failure mode.
- **Evidence:** S-8.01 AC-006 covers malformed-JSON; S-8.03 has no equivalent despite similar JSON parsing.
- **Proposed Fix:** Add an explicit AC for malformed/null JSON input handling, specifying the exit code and emit behavior.

### MEDIUM

#### F-S803-P1-005: T-3 contradictory whitespace-classifier guidance

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** T-3
- **Description:** T-3 gives contradictory guidance on whitespace classification: it first suggests `.chars()` + `.is_whitespace()` (Unicode whitespace), then in the same paragraph suggests `.is_ascii_whitespace()` (ASCII only). These produce different results for non-ASCII whitespace characters. The implementer cannot follow both.
- **Proposed Fix:** Pick one whitespace classifier that matches the bash `[:space:]` semantics (which is locale-dependent but typically ASCII whitespace) and remove the contradictory alternative.

#### F-S803-P1-006: EC-003 misrepresents bash fallback semantics for null last_assistant_message

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-003
- **Description:** EC-003 states the fallback is directly to empty when `last_assistant_message` is null. The bash source uses `jq // null` which falls through to the `result` field, not directly to empty. The spec's two-step fallback (last_assistant_message → result → empty) is not faithfully represented.
- **Proposed Fix:** Correct EC-003 to reflect the actual bash fallback chain: `last_assistant_message ?? result ?? ""`.

#### F-S803-P1-007: [process-gap] CAP-022 cross-CAP stretch disclosure missing (sibling S-8.01 has it)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Capability Anchor Justification
- **Description:** S-8.01 and other siblings include a CAP-022 cross-CAP stretch disclosure. S-8.03 omits this disclosure entirely, creating an inconsistency in the audit trail across Tier 1 stories.
- **Proposed Fix:** Add the CAP-022 cross-CAP stretch disclosure per the S-8.01 pattern, tailored to track-agent-stop's specific behaviors.

#### F-S803-P1-008: [process-gap] wave: 15 provisional disclosure missing (sibling S-8.00 has it)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter / Goal section
- **Description:** S-8.00 and other siblings include a `wave: 15 [provisional]` disclosure. S-8.03 omits this, creating an audit trail gap.
- **Proposed Fix:** Add the wave: 15 provisional disclosure per the S-8.00 pattern.

#### F-S803-P1-009: AC-001 omits [hooks.capabilities] block migration specification

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** AC-001
- **Description:** AC-001 covers the registry migration for the hook but omits the specification for migrating the `[hooks.capabilities]` block (binary_allow, read_file/write_file path allowlists). Without this, the implementer must infer capability migration from other stories, creating inconsistency risk.
- **Proposed Fix:** Extend AC-001 to explicitly specify the `[hooks.capabilities]` block migration: which fields to remove (binary_allow bash/jq), which to retain, and any new WASM-specific capability declarations.

### LOW

#### F-S803-P1-010: BC-7.03.082 H1 backslash artifact verification deferred (BC files inaccessible)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** BC-7.03.082 title
- **Description:** There may be a backslash artifact in the BC-7.03.082 file title, but BC files are currently inaccessible for direct verification. This finding is flagged as a deferred verification item.
- **Proposed Fix:** Verify BC-7.03.082 title against the canonical BC file when accessible; fix any backslash artifact.

#### F-S803-P1-011: T-7 perf comparison ambiguous — S-8.00 baseline does not include track-agent-stop

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** T-7
- **Description:** T-7 references performance comparison vs S-8.00 baseline. The S-8.00 baseline measurement set does not include track-agent-stop (it covers a different hook set). The comparison target is undefined.
- **Proposed Fix:** Remove the S-8.00 baseline reference from T-7, or define a new measurement baseline specific to track-agent-stop.

#### F-S803-P1-012: Token Budget BC file token estimate unverifiable

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget
- **Description:** The Token Budget claims a specific token count for BC files but BC files are inaccessible for verification. The estimate may be incorrect (as found in S-8.01 where actual was ~950 vs claimed ~400).
- **Proposed Fix:** Update Token Budget with accurate BC token counts when files are accessible. Use S-8.01's finding as a calibration signal (~950 tokens/BC).

### NIT

#### F-S803-P1-013: Body BC table title separator inconsistency (`/` vs `|`)

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** Body BC table header separator
- **Description:** The body BC table uses `/` as a title separator in one row and `|` in another. Minor cosmetic inconsistency.
- **Proposed Fix:** Standardize on one separator character throughout the body BC table.

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
