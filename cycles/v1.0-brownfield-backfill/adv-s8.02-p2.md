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
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.02-p1.md
target: S-8.02 v1.1
target_file: .factory/stories/S-8.02-native-port-pr-manager-completion-guard.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 6
findings_high: 2
findings_med: 3
findings_low: 1
findings_nit: 0
---

# Adversarial Review: S-8.02 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S802-P2-NNN`

- `F`: Fixed prefix
- `S802`: Story 8.02
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.02 v1.1 (383 lines). All 13 pass-1 findings
verified closed. 6 new findings discovered: 2 HIGH, 3 MED, 1 LOW, 0 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3 per ADR-013 (HIGH findings present).

Trajectory: 13 → 6 (54% decay). Dominant issue class: test coverage gaps
(NEXT_STEP>=10 wildcard, AGENT fallback chain) and semantic ambiguity
(grep -c line-vs-match, stderr content under-specification). Pass-3 requires
focused fix on the 2 HIGH findings before clock advance is possible.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

All 13 pass-1 findings verified closed in v1.1.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S802-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S802-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S802-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S802-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S802-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S802-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S802-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S802-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S802-P1-009 | emit_event signature | CLOSED (PARTIAL → F-S802-P2-003 sibling pattern) | Signature referenced; EC-002 hint still truncated |
| F-S802-P1-010 | BC-7.03.046 ASCII/Unicode title aligned | CLOSED | BC title consistent |
| F-S802-P1-011 | jq-missing degradation contract | CLOSED | EC section updated |
| F-S802-P1-012 | 10th catch-all case arm | CLOSED | Case arm present |
| F-S802-P1-013 | AC-005 hint string completed | CLOSED (PARTIAL → F-S802-P2-004) | 1 of 11 bash stderr lines pinned |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S802-P2-001 [HIGH] AC-006 omits NEXT_STEP>=10 wildcard test coverage

**Location:** AC-006 bats fixture table and EC-006.

**Issue:** EC-006 correctly surfaces the wildcard NEXT_STEP matching issue when
NEXT_STEP values reach double digits (10+). The wildcard pattern `step_*` must
match `step_10`, `step_11`, etc. However, AC-006's bats fixture table does not
include any test case exercising NEXT_STEP values >= 10. Without a fixture
asserting this, the wildcard contract is unverified for the production case that
matters most — late-stage pipelines with 10+ steps. A naive implementation using
a fixed-width pattern (`step_[0-9]`) would pass all existing ACs while silently
failing for step_10+.

**Suggested fix:** Add to AC-006 bats fixture table:
- Input: NEXT_STEP=10, status=in-progress → expected: NOT blocked
- Input: NEXT_STEP=99, status=complete → expected: NOT blocked

**Policy:** POLICY 6 (measurability — edge case must be exercised by a fixture).

---

### F-S802-P2-002 [HIGH] AGENT extraction doesn't pin 3-arm fallback chain

**Location:** T-3 AGENT extraction logic and AC-003.

**Issue:** The AGENT field extraction is described without pinning the canonical
3-arm fallback chain: `agent_type ?? subagent_name ?? "unknown"`. The story
describes extracting AGENT from the hook envelope but does not specify the
precedence order when `agent_type` is absent or null. An implementer could use
either `agent_type` or `subagent_name` as the primary field, or omit the
"unknown" fallback entirely, producing behavioral divergence from sibling hooks
and from the observability sink schema that consumes this field.

**Suggested fix:** Pin the 3-arm chain explicitly in T-3:
```rust
let agent = envelope.get("agent_type")
    .or_else(|| envelope.get("subagent_name"))
    .unwrap_or("unknown");
```

**Policy:** POLICY 6 (measurability — extraction logic must be unambiguous).

---

### F-S802-P2-003 [MED] STEP_COUNT semantics ambiguous — grep -c counts lines not occurrences

**Location:** T-3 STEP_COUNT computation description and related AC wording.

**Issue:** The bash implementation uses `grep -c` to count STEP_COUNT. `grep -c`
counts *lines* containing the pattern, not total *occurrences*. A JSONL line with
two STEP_COMPLETE tokens counts as 1, not 2. The story's AC wording says
"occurrences" without specifying line-vs-match semantics. If a single JSONL line
can contain multiple STEP_COMPLETE tokens (pathological but possible under
concurrent writes), `grep -c` would undercount. The Rust port must decide which
semantics are required.

**Suggested fix:** State explicitly in T-3: "STEP_COUNT is the count of *lines*
containing STEP_COMPLETE (line-occurrence semantics, matching `grep -c`; one
line with multiple tokens = 1)." Update AC wording from "occurrences" to "lines
containing."

**Policy:** POLICY 6 (measurability — counting semantics must be pinned).

---

### F-S802-P2-004 [MED] AC-005 stderr-injection content under-specified — only 1 of 11 bash lines pinned

**Location:** AC-005 acceptance criterion.

**Issue:** The pass-1 fix for F-S802-P1-013 pinned the primary AC-005 stderr
line verbatim, but only 1 of the 11 possible bash stderr output lines is fully
specified. The remaining 10 are described with "similar to" or "equivalent"
prose. Bats tests must assert on exact strings; "similar to" language cannot
be translated into a deterministic assertion.

**Suggested fix:** Enumerate all 11 stderr lines verbatim, or explicitly scope
AC-005 to the 1 pinned line and acknowledge the others as implementation-defined
(documenting the choice rationale).

**Policy:** POLICY 6 (measurability — bats assertions require verbatim strings).

---

### F-S802-P2-005 [MED] EC-002 hint truncated — sibling propagation gap from F-001 fix

**Location:** EC-002.

**Issue:** The pass-1 fix burst addressed F-001 (title truncation algorithm) by
tightening the body contract. However EC-002, which describes the truncation
hint for long PR titles, was not updated to reflect the canonical truncation
algorithm established by the F-001 fix. EC-002 still cites the pre-fix wording,
creating a body/edge-case contract divergence within the same story.

**Suggested fix:** Update EC-002 to reference the truncation algorithm now
specified in the body contract, citing the specific character limit and ellipsis
form established by the F-001 fix.

**Policy:** POLICY 7 (cross-document consistency — EC must be consistent with
body contract).

---

### F-S802-P2-006 [LOW] AC-005 host::emit_event payload subagent="unknown" string handling unspecified

**Location:** AC-005.

**Issue:** When the AGENT fallback chain resolves to "unknown" (all three arms
absent), AC-005 does not specify how the `subagent="unknown"` literal is rendered
in the host::emit_event fields slice. The distinction matters for downstream
observability consumers that filter on subagent field presence vs value.

**Suggested fix:** Add a note to AC-005: "If agent resolution yields 'unknown',
emit `("subagent", "unknown")` as a literal field — do not omit the field."

**Policy:** POLICY 6 (measurability — field content must be pinned).

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.02 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.045/046/047/048 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| vsdd-hook-sdk path dep | Cargo.toml section | PASS |
| NEXT_STEP>=10 bats fixture | AC-006 | FAIL (F-S802-P2-001) |
| 3-arm AGENT fallback chain | T-3 | FAIL (F-S802-P2-002) |
| grep -c line-vs-match semantics pinned | T-3 / AC | FAIL (F-S802-P2-003) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | All required sections present | PASS | |
| POLICY 2 — BC anchor integrity | BC-7.03.045/046/047/048 anchored | PASS | |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | NEXT_STEP wildcard, grep-c semantics, stderr verbatim | FAIL | F-S802-P2-001/003/004 |
| POLICY 7 — Cross-document consistency | EC-002 truncation hint stale | FAIL | F-S802-P2-005 |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 6 findings. F-S802-P2-001 confirmed HIGH: a naive
fixed-width step pattern would pass all ACs and silently fail step_10+.
F-S802-P2-002 confirmed HIGH: 3-arm fallback chain is essential for
observability schema consistency; omission causes silent behavioral divergence.

**Iteration 2:** Severity confirmed. Two HIGH findings prevent clock advance.
Three MED findings are implementation-blocking in their respective ACs.

**Iteration 3:** No findings withdrawn. 6 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S802-P2-001 | Novel | NEXT_STEP>=10 wildcard coverage gap not examined at pass-1 |
| F-S802-P2-002 | Novel | 3-arm AGENT fallback chain not examined at pass-1 |
| F-S802-P2-003 | Novel | grep -c line-vs-match semantics not examined at pass-1 |
| F-S802-P2-004 | Pass-1 partial closure | F-P1-013 pinned 1 of 11 stderr lines |
| F-S802-P2-005 | Pass-1 partial closure | F-P1-009 fixed body but EC-002 not updated |
| F-S802-P2-006 | Novel | subagent="unknown" field rendering not examined at pass-1 |

4 novel + 2 pass-1 partial closures re-surfaced. Fresh-context compounding
value confirmed.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 2 HIGH + 3 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; HIGH findings prevent advance).

**Trajectory:** 13 → 6 (54% decay).

**Pass-3 priors for adversary:**
- Verify NEXT_STEP>=10 bats fixtures present in AC-006
- Verify 3-arm AGENT fallback chain pinned in T-3
- Verify grep -c semantics stated as "lines containing" in T-3 and AC
- Verify EC-002 truncation hint consistent with body contract
