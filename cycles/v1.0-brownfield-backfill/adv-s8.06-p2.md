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
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.06-p1.md
target: S-8.06 v1.1
target_file: .factory/stories/S-8.06-native-port-session-learning.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 9
findings_high: 1
findings_med: 4
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.06 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S806-P2-NNN`

- `F`: Fixed prefix
- `S806`: Story 8.06
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.06 v1.1 (419 lines). 10 of 11 pass-1
findings fully closed; F-002 (BC-7.03.076 contradiction framing) partially
closed — the gate was softened from "includes" to "may include" rather than
hardened, leaving the contradiction unresolved. 9 new findings: 1 HIGH, 4 MED,
3 LOW, 1 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3.

Trajectory: 11 → 9 (18% decay). Lowest decay of Tier 1 pass-2 batch. The
partial closure of F-002 drove 1 HIGH finding, and several NEW findings in the
AC/EC structure are implementation-blocking.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

10 of 11 pass-1 findings fully closed. F-002 partially closed.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S806-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S806-P1-002 | BC-7.03.076 contradiction — gate hardening | CLOSED (PARTIAL → F-S806-P2-001) | "includes" softened to "may include" — weaker not harder |
| F-S806-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S806-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S806-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S806-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S806-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S806-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S806-P1-009 | AC-001 disjunction — binary_allow contradiction resolved | CLOSED | Contradiction resolved |
| F-S806-P1-010 | AC-006 fabricated claim removed | CLOSED | BC-aligned |
| F-S806-P1-011 | EC-005 stdin WASI SIGPIPE risk | CLOSED | Risk documented |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S806-P2-001 [HIGH] BC-7.03.076 contradiction gated behind "may include" soft gate — pass-1 regression

**Location:** BC-7.03.076 gate condition, self-reference loop closure.

**Issue:** Pass-1 F-002 aimed to harden the BC-7.03.076 contradiction (the
session-learning hook would emit an event that the hook itself might process
on the next session). The pass-1 fix burst changed "includes" to "may include"
in the gate condition. This is WEAKER than the pre-fix language. The gate is
now effectively optional: "may include" permits an implementation that never
gates on this condition, leaving the self-reference loop open. The gate must
use "includes" (hard invariant) or "always includes" to definitively close the
contradiction.

**Suggested fix:** Revert "may include" to "includes" (hard gate). State:
"The session-learning hook MUST include a filter that excludes events emitted
by session-learning itself to prevent self-reinforcing loops."

**Policy:** POLICY 2 (BC anchor integrity — BC invariants must be hard, not
permissive).

---

### F-S806-P2-002 [MED] AC numbering gap — AC-001/003/004/005 (no AC-002)

**Location:** AC table.

**Issue:** The AC table has a numbering gap: AC-001 is followed directly by
AC-003, with no AC-002. The pass-1 fix burst collapsed AC-002 + AC-006 to
reduce over-decomposition (6→4 ACs) but left a numbering gap rather than
renumbering sequentially. This creates reader confusion and breaks any
cross-reference that cites "AC-002."

**Suggested fix:** Renumber sequentially: AC-001 → AC-001, AC-003 → AC-002,
AC-004 → AC-003, AC-005 → AC-004. Update all cross-references within the
story that cite the old numbers.

**Policy:** POLICY 1 (lifecycle completeness — sequential numbering required
for unambiguous cross-reference).

---

### F-S806-P2-003 [MED] BC-7.03.076 Trace text asserts postcondition not present in BC file — trace fabrication

**Location:** BC Trace table, BC-7.03.076 Trace column.

**Issue:** The BC Trace table entry for BC-7.03.076 asserts a specific
postcondition in the Trace column that is not stated in the BC file excerpt
quoted in the story. The Trace column content appears to have been written from
memory or inference rather than direct citation. This is a trace fabrication —
the same defect class as F-S809-P2-002 (BC-7.03.071 invariants fabricated in
S-8.09).

**Suggested fix:** Open BC-7.03.076 directly and transcribe the actual
postcondition text into the Trace column. If the BC does not contain the
asserted postcondition, remove the Trace claim and file a v1.1 BC candidate
to add the missing postcondition.

**Policy:** POLICY 2 (BC anchor integrity — trace content must match BC file
exactly).

---

### F-S806-P2-004 [MED] EC-005 "large Stop envelope" — "large" not quantified in bytes

**Location:** EC-005.

**Issue:** EC-005 describes a test case involving a "large Stop envelope" to
verify SIGPIPE and pipe-fill behavior. "Large" is not quantified in bytes, KB,
or MB. Pipe-fill behavior is OS-dependent and requires a minimum buffer size to
trigger (typically 64KB on Linux). Without a concrete threshold, a bats test
cannot reliably exercise the pipe-fill path.

**Suggested fix:** Specify the threshold: "large Stop envelope (>64KB; e.g.,
65,536 bytes of padding)." Document the OS dependency: "Test may be
platform-specific; skip on macOS if pipe buffer differs."

**Policy:** POLICY 6 (measurability — test conditions must be quantified).

---

### F-S806-P2-005 [MED] Wave-15 disclosure exists in two places with subtly different framing

**Location:** Frontmatter (`wave: 15 [provisional]`) and story body
(`[process-gap]`).

**Issue:** The wave: 15 disclosure appears in both frontmatter and story body,
but with different qualifiers: `[provisional]` in frontmatter vs `[process-gap]`
in the body. These have distinct meanings per factory convention: `[provisional]`
signals a tentative assignment; `[process-gap]` signals a known process gap
being acknowledged. The story should use one canonical qualifier consistently
across both locations.

**Suggested fix:** Use `[process-gap]` in both locations, consistent with
sibling S-8.00 and the established disclosure pattern.

**Policy:** POLICY 7 (cross-document consistency — qualifier must be uniform).

---

### F-S806-P2-006 [LOW] BC-7.03.076 trace title "identity & registry binding" mismatches trace content

**Location:** BC Trace table, BC-7.03.076 row title.

**Issue:** The BC Trace table row for BC-7.03.076 has the title "identity &
registry binding" but the actual trace content asserts a binary_allow
postcondition. These are semantically distinct concerns. The title should
match the trace content.

**Suggested fix:** Update the title to match the trace content, or vice versa.

**Policy:** POLICY 7 (cross-document consistency — row title must match content).

---

### F-S806-P2-007 [LOW] Architecture Compliance "exec_subprocess block REMOVED" contradicts AC-001 disjunction

**Location:** Architecture Compliance Rules section and AC-001.

**Issue:** The Architecture Compliance Rules section states "exec_subprocess
block REMOVED" as an absolute rule. However, AC-001 uses disjunctive phrasing
("either...or") implying the block may optionally remain. An absolute rule and
a disjunctive AC cannot both be correct — one will be violated by any
conformant implementation.

**Suggested fix:** Align both to the canonical: exec_subprocess block is
fully removed; update AC-001 to use non-disjunctive phrasing.

**Policy:** POLICY 6 (measurability — contradictory requirements must be
resolved).

---

### F-S806-P2-008 [LOW] T-1 "S-8.06 is blocked if S-8.00 has not run" — "run" not defined

**Location:** Task T-1.

**Issue:** T-1 states S-8.06 is blocked if "S-8.00 has not run." "Run" is
undefined: does it mean S-8.00 is status=merged? status=ready? That
bc-anchor-table.md exists? That all 9 S-8.00 ACs pass? Without an observable
criterion, this blocking condition cannot be checked programmatically.

**Suggested fix:** Replace "run" with a testable condition: "S-8.06 is blocked
if S-8.00 status is not yet 'ready' or 'merged' (check STORY-INDEX)."

**Policy:** POLICY 6 (measurability — blocking conditions must be observable).

---

### F-S806-P2-009 [NIT] EC-001 expectation "exit 0" loses failure as silent

**Location:** EC-001.

**Issue:** EC-001 specifies that on a specific error path the hook exits with
code 0. Per AGENT-SOUL.md rule #4, silent failures (exit 0 with no observable
output) should be avoided. If exit 0 is intentional on this path, a rationale
should be provided.

**Suggested fix:** If exit 0 is intentional, add: "Rationale: session-learning
hook must not block the session on non-critical errors; exit 0 + stderr warning
is the correct pattern for this hook class." If not intentional, change to exit 1.

**Policy:** NIT — AGENT-SOUL.md rule #4 advisory.

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.06 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.076/077/078 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] in body | Story body | PASS |
| BC-7.03.076 gate uses "includes" (hard invariant) | Gate condition | FAIL (F-S806-P2-001) |
| AC numbering sequential (no gaps) | AC table | FAIL (F-S806-P2-002) |
| BC-7.03.076 trace content matches BC file | Trace column | FAIL (F-S806-P2-003) |
| wave qualifier consistent (frontmatter vs body) | Both locations | FAIL (F-S806-P2-005) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | AC numbering gap | FAIL | F-S806-P2-002 |
| POLICY 2 — BC anchor integrity | BC-7.03.076 soft gate + trace fabrication | FAIL | F-S806-P2-001, F-S806-P2-003 |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | Large envelope threshold; exec_subprocess contradiction; T-1 blocking | FAIL | F-S806-P2-004/007/008 |
| POLICY 7 — Cross-document consistency | BC trace title; wave qualifier; disjunction vs absolute | FAIL | F-S806-P2-005/006 |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 9 findings. F-S806-P2-001 confirmed HIGH:
"may include" is a weaker gate than the pre-fix "includes" — the fix burst
regressed the gate condition. The self-reference loop remains open.

**Iteration 2:** Severity confirmed. 1 HIGH (BC integrity regression). 4 MED
(implementation-blocking). 3 LOW (informational). 1 NIT.

**Iteration 3:** No findings withdrawn. 9 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S806-P2-001 | Pass-1 partial closure (regression) | F-002 was closed with a weaker gate |
| F-S806-P2-002 | Novel | AC numbering gap introduced by pass-1 AC collapse (6→4) |
| F-S806-P2-003 | Novel | BC trace fabrication not examined at pass-1 |
| F-S806-P2-004 | Novel | "large" envelope threshold not quantified at pass-1 |
| F-S806-P2-005 | Novel | [provisional] vs [process-gap] inconsistency not examined at pass-1 |
| F-S806-P2-006 | Novel | BC trace row title mismatch not examined at pass-1 |
| F-S806-P2-007 | Novel | Absolute rule vs disjunction contradiction not examined at pass-1 |
| F-S806-P2-008 | Novel | T-1 "run" undefined not examined at pass-1 |
| F-S806-P2-009 | Novel | EC-001 silent exit not examined at pass-1 |

1 pass-1 partial closure (regression) + 8 net-new findings.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 1 HIGH + 4 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; HIGH present).

**Trajectory:** 11 → 9 (18% decay). Lowest decay in Tier 1 pass-2 batch;
deeper fix burst needed to reach NITPICK_ONLY.

**Pass-3 priors for adversary:**
- Verify BC-7.03.076 gate uses "includes" (hard invariant, not "may include")
- Verify AC renumbered sequentially (no gap at AC-002)
- Verify BC-7.03.076 trace content transcribed from actual BC file
- Verify wave qualifier consistent ([process-gap] in both frontmatter and body)
