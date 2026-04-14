---
name: systematic-debugging
description: >
  Use when encountering any bug, test failure, or unexpected behavior — before
  proposing fixes. Enforces root cause investigation through 4 phases: investigate,
  analyze patterns, test hypotheses, implement with failing test first.
---

# Systematic Debugging

## Hard Gate

NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST.

If you haven't completed Phase 1, you cannot propose fixes. Violating the letter of this process is violating the spirit of debugging.

## When to Use

Use for ANY technical issue:
- Test failures during TDD (Red Gate violations, unexpected passes)
- Implementation bugs (wrong behavior vs BC postconditions)
- Build failures (compilation errors, dependency conflicts)
- Integration issues (module boundary mismatches)
- Performance problems
- CI/CD pipeline failures

**Use ESPECIALLY when:**
- Under time pressure (emergencies make guessing tempting)
- "Just one quick fix" seems obvious
- You've already tried multiple fixes
- Previous fix didn't work
- You don't fully understand the issue

## The Four Phases

Complete each phase before proceeding to the next.

### Phase 1: Root Cause Investigation

**BEFORE attempting ANY fix:**

1. **Read error messages carefully**
   - Don't skip past errors or warnings
   - Read stack traces completely
   - Note line numbers, file paths, error codes
   - Check if the error references a behavioral contract (BC-S.SS.NNN)

2. **Reproduce consistently**
   - Can you trigger it reliably?
   - What are the exact steps?
   - If not reproducible, gather more data — don't guess

3. **Check recent changes**
   - `git diff` and recent commits
   - New dependencies, config changes
   - Environmental differences

4. **Gather evidence in multi-component systems**
   - For each component boundary: log what enters, what exits
   - Run once to see WHERE it breaks
   - THEN investigate that specific component

5. **Trace data flow**
   - Where does the bad value originate?
   - What called this with the bad value?
   - Keep tracing up until you find the source
   - Fix at source, not at symptom

### Phase 2: Pattern Analysis

1. **Find working examples** — locate similar working code in the same codebase
2. **Compare against references** — if implementing a pattern, read reference implementation COMPLETELY
3. **Identify differences** — list every difference between working and broken, however small
4. **Understand dependencies** — what other components, settings, config does this need?

### Phase 3: Hypothesis and Testing

1. **Form single hypothesis** — "I think X is the root cause because Y" — write it down
2. **Test minimally** — make the SMALLEST possible change to test the hypothesis. One variable at a time.
3. **Verify** — did it work? Yes → Phase 4. No → form NEW hypothesis. Don't add more fixes on top.

### Phase 4: Implementation

1. **Write a failing test** — simplest possible reproduction as an automated test. MUST have before fixing. Follow Red Gate discipline.
2. **Implement single fix** — address the root cause. ONE change at a time. No "while I'm here" improvements.
3. **Verify** — test passes? No other tests broken? Issue actually resolved?
4. **If fix doesn't work** — count how many fixes you've tried:
   - < 3: Return to Phase 1, re-analyze with new information
   - **≥ 3: STOP — this is an architectural problem, not a failed hypothesis**

### Phase 4.5: When 3+ Fixes Fail

**Pattern indicating architectural problem:**
- Each fix reveals new shared state, coupling, or problems in different places
- Fixes require "massive refactoring" to implement
- Each fix creates new symptoms elsewhere

**STOP and escalate.** Report BLOCKED with:
- What you investigated (Phase 1 findings)
- What hypotheses you tested (Phase 3 results)
- Why you believe this is architectural, not a simple bug
- Recommended architectural discussion topics

Do NOT attempt Fix #4 without human approval.

## Red Flags — STOP and Return to Phase 1

| Thought | Reality |
|---------|---------|
| "Quick fix for now, investigate later" | Later never comes. Investigate now. |
| "Just try changing X and see if it works" | Guess-and-check averages 2-3 hours. Systematic averages 15-30 min. |
| "Add multiple changes, run tests" | Can't isolate what worked. Causes new bugs. |
| "Skip the test, I'll manually verify" | Untested fixes don't stick. Write the test. |
| "It's probably X, let me fix that" | Seeing symptoms ≠ understanding root cause. |
| "I don't fully understand but this might work" | Partial understanding guarantees partial fix. |
| "One more fix attempt" (after 2+ failures) | 3+ failures = architectural problem. Escalate. |
| "I see the problem, let me fix it" | You see a SYMPTOM. Phase 1 finds the CAUSE. |

## BC-Aware Debugging

When the bug violates a behavioral contract:
1. Read the BC's preconditions, postconditions, and invariants
2. Identify which clause is violated
3. Trace from the violated clause back to the implementation
4. The fix must restore the BC's guarantees — not just make the test pass

## Reporting

When done, report with standard status protocol:
- **DONE:** Root cause found and fixed, failing test added, all tests pass
- **DONE_WITH_CONCERNS:** Fixed but uncertain about side effects or related issues
- **BLOCKED:** 3+ fixes failed, architectural problem suspected
- **NEEDS_CONTEXT:** Missing information to complete investigation

## Quick Reference

| Phase | Key Activities | Success Criteria |
|-------|---------------|------------------|
| 1. Root Cause | Read errors, reproduce, check changes, trace data | Understand WHAT and WHY |
| 2. Pattern | Find working examples, compare differences | Identify the pattern break |
| 3. Hypothesis | Form theory, test minimally, one variable | Confirmed or new hypothesis |
| 4. Implementation | Write failing test, single fix, verify | Bug resolved, tests pass |
