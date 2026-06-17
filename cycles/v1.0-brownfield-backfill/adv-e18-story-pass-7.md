---
document_type: adversarial-review
cycle: v1.0-brownfield-backfill
cascade: E-18-story
pass: 7
date: 2026-06-17
adversary_model: claude-sonnet-4-6
verdict: NOT-CLEAN
finding_count: 2
findings_by_severity:
  critical: 0
  major: 1
  medium: 0
  low: 1
  nitpick: 0
  process_gap: 0
streak_before: 0/3
streak_after: 0/3
parent_pass: 6
parent_decision: D-623
---

# E-18 Story Cascade Adversarial Review — Pass 7

**Date:** 2026-06-17
**Verdict:** NOT-CLEAN
**Finding count:** 2 (1 MAJOR + 1 LOW)
**3-CLEAN streak:** 0/3 (unchanged — NOT-CLEAN resets streak)
**Next:** Pass-8 adversary dispatch + consistency re-verify

## Part A — Findings

### F-P7-001 [MAJOR] — S-18.09 AC-008 gate is silent-inert: validator can never fire

**Story:** S-18.09 v1.6
**Severity:** MAJOR
**Type:** Silent-inert validator gate

**Finding:**
S-18.09 AC-008 specifies an AC↔PC parity gate that must fire at TDD time to verify that every `(traces to BC-X PC-N / INV-N)` reference in a story resolves to a real numbered clause in the cited BC. However, the gate as specified in the story has no mechanism to actually block or fail — it is described as a "check" that emits output but the story does not specify an exit-code failure path when a mis-trace is found. The gate can "find" violations but the Acceptance Criteria do not specify that the validator MUST exit non-zero on violation. This makes AC-008 structurally silent-inert: it can report but never block TDD red-gate discipline.

**Evidence (from S-18.09 v1.6 AC-008):**
The AC-008 text reads: "AC↔PC parity gate: validator scans all story files for `(traces to BC-X PC-N)` patterns and verifies the cited PC exists in the BC file." There is no clause "exits non-zero when any mis-trace found" — making this a WARN-class validator, not a FAIL-class validator. A WARN-class validator cannot enforce the AC↔PC parity discipline required by L-F2-ac-pc-parity-sibling-sweep.

**Required fix:**
S-18.09 AC-008 must explicitly state that the validator exits non-zero (exit code 1 or 2) when any AC↔PC mis-trace is detected. The gate is only meaningful if it can fail a bats test. Story-writer must add the failure-path specification to AC-008.

---

### F-P7-002 [LOW] — Consistency-validator WARN vs FAIL ambiguity in S-18.08 scope definition

**Story:** S-18.08 v1.4
**Severity:** LOW
**Type:** Specification ambiguity

**Finding:**
S-18.08 specifies a "consistency-validator scan" but does not distinguish between WARN-level and FAIL-level violations. For the pure-parse invariant consistency gate (enforcing BC-4.14.001 Inv1 + BC-4.15.001 Inv1), it is essential that violations cause a FAIL result (non-zero exit from the consistency check) not merely a warning log. The current AC text is ambiguous: "consistency-validator scan of BCs declaring pure-parse against substrate-read patterns in bodies" — does this exit non-zero on finding? The story text is unclear.

**Required fix:**
S-18.08 acceptance criteria must explicitly state that the scanner exits non-zero when a pure-parse-declaring BC body contains substrate-read patterns. Low severity because the ambiguity is in specification prose rather than a structural implementation gap — story-writer can clarify in-pass-7 fix.

---

## Part B — Observations (non-actionable)

### O-P7-001 [OBSERVATION] — bats fatal-path contract documentation pattern

**Observation:**
Several E-18 stories specify bats test counts but do not explicitly enumerate the fatal-path (exit non-zero) test cases. While this is not a finding — exit-code behavior is implied by bats `run` + `assert_failure` conventions — it would improve story quality if AC tables explicitly called out which ACs exercise the "FAIL / exit non-zero" path. This is a documentation quality observation, not a behavioral gap. The pattern is common to S-18.08 and S-18.09 and is a candidate for a future process-gap lesson if the same issue recurs in implementation review.

**Disposition:** NON-ACTIONABLE at this pass. Not fixed.

---

## Summary

Pass-7 identified 2 findings: F-P7-001 MAJOR (S-18.09 AC-008 silent-inert gate — no FAIL exit path specified) and F-P7-002 LOW (S-18.08 WARN vs FAIL ambiguity). Both require story-writer fixes. 3-CLEAN streak remains 0/3. Pass-8 dispatch after story-writer + state-manager fix burst.
