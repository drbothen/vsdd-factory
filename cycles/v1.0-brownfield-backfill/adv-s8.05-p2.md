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
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.05-p1.md
target: S-8.05 v1.1
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 4
findings_high: 0
findings_med: 3
findings_low: 1
findings_nit: 0
---

# Adversarial Review: S-8.05 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S805-P2-NNN`

- `F`: Fixed prefix
- `S805`: Story 8.05
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.05 v1.1 (395 lines). All 12 pass-1 findings
verified closed. 4 new findings: 0 HIGH, 3 MED, 1 LOW, 0 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3 per ADR-013 (MED findings prevent advance).

Trajectory: 12 → 4 (67% decay). Dominant issue class: stdin envelope field
semantics (RESULT fallback chain, AC-003 agent pattern notation) and test
design ambiguity (AC-007 Check 3a/3b triggering). Pass-3 expected to advance
clock if fix burst is comprehensive.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

All 12 pass-1 findings verified closed in v1.1. No partial-fix regressions.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S805-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S805-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S805-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S805-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S805-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S805-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S805-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S805-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S805-P1-009 | AC-006 Check 3a verdict accumulation logic | CLOSED | Logic corrected |
| F-S805-P1-010 | stderr format verbatim 3-line remediation block | CLOSED | Block present |
| F-S805-P1-011 | binary_allow cleanup (bash/jq/gh) | CLOSED | All three listed in tasks |
| F-S805-P1-012 | emit failure semantics specified | CLOSED | Semantics documented |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S805-P2-001 [MED] Stdin RESULT field dual-fallback semantics not specified

**Location:** T-3 stdin envelope parsing, RESULT field extraction.

**Issue:** The hook reads the RESULT field from the PostToolUse stdin envelope.
When `result` is absent or null, the story does not specify the dual-fallback
semantics. The correct fallback chain for Claude Code hook envelopes is:
`last_assistant_message ?? result ?? ""`. This matters because Stop hooks (and
some PostToolUse events) can receive envelopes where `result` is absent and
`last_assistant_message` is the semantic equivalent. Without pinning the chain,
an implementer may use a single-arm fallback that silently drops legitimate
PR review content.

**Suggested fix:** Add to T-3: "Extract RESULT using fallback chain:
`envelope.get("last_assistant_message").or(envelope.get("result")).unwrap_or("")`."

**Policy:** POLICY 6 (measurability — field extraction order must be pinned).

---

### F-S805-P2-002 [MED] AC-003 agent-pattern prose uses dotted notation — ambiguous vs hyphenated bash globs

**Location:** AC-003.

**Issue:** AC-003 describes agent matching patterns using dotted notation (e.g.,
`pr.reviewer`, `code.reviewer`). The original bash hook uses hyphenated form
(`pr-reviewer`, `code-reviewer`). For the Rust port, the matching semantics must
be explicit: is the pattern a glob? A regex? An exact match? And which separator
form is canonical — dot or hyphen? The dotted vs hyphenated ambiguity would cause
behavioral divergence between the bash and WASM implementations if the patterns
are used as-is.

**Suggested fix:** Resolve to one canonical form and state the matching
mechanism. If glob: `fn match_agent(agent: &str, pattern: &str) -> bool {
glob::Pattern::new(pattern)?.matches(agent) }` with patterns as `pr-reviewer`.

**Policy:** POLICY 6 (measurability — pattern matching must be unambiguous).

---

### F-S805-P2-003 [MED] AC-007 case (e) input shape not specified — Check 3a/3b triggering ambiguous

**Location:** AC-007, test case (e).

**Issue:** AC-007 includes test case (e) to verify that only Check 3b fires and
not Check 3a. The input shape for case (e) is not specified. Without a concrete
stdin envelope example, a bats test cannot reliably assert that Check 3a is NOT
triggered while 3b IS. An underspecified input could accidentally satisfy both
checks or neither, producing a false pass.

**Suggested fix:** Specify case (e) input verbatim:
```json
{"event_type": "PostToolUse", "tool_name": "Bash", "result": "",
 "subagent_name": "pr-reviewer", "pr_number": "42"}
```
And state: "Check 3b fires (PR number present); Check 3a must NOT fire (result
is empty string, not absent — implementation must distinguish null from empty)."

**Policy:** POLICY 6 (measurability — bats test inputs must be concrete).

---

### F-S805-P2-004 [LOW] T-8 removes [hooks.capabilities.exec_subprocess] but silent on top-level [hooks.capabilities] env_allow block

**Location:** Task T-8.

**Issue:** Task T-8 specifies removal of `[hooks.capabilities.exec_subprocess]`.
The hooks.json for validate-pr-review-posted also includes a top-level
`[hooks.capabilities]` block with an `env_allow` list. T-8 is silent on whether
this parent block and its `env_allow` children are also removed. If the WASM
port eliminates all subprocess execution, the entire `[hooks.capabilities]`
block (header + all children) should be removed.

**Suggested fix:** Add to T-8: "Also remove the top-level `[hooks.capabilities]`
block and its `env_allow` entries — all subprocess capability declarations are
eliminated by the WASM port."

**Policy:** POLICY 1 (lifecycle completeness — all file changes must be listed).

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.05 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.04.040-044 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| vsdd-hook-sdk path dep | Cargo.toml section | PASS |
| RESULT fallback chain pinned | T-3 | FAIL (F-S805-P2-001) |
| Agent pattern notation canonical | AC-003 | FAIL (F-S805-P2-002) |
| AC-007 case (e) input specified | AC-007 | FAIL (F-S805-P2-003) |
| [hooks.capabilities] env_allow removal in T-8 | T-8 | FAIL (F-S805-P2-004) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | env_allow block removal missing from T-8 | FAIL | F-S805-P2-004 |
| POLICY 2 — BC anchor integrity | BC-7.04.040-044 anchored | PASS | |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | RESULT fallback, agent pattern, AC-007(e) input | FAIL | F-S805-P2-001/002/003 |
| POLICY 7 — Cross-document consistency | No cross-doc issues | PASS | |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 4 findings. F-S805-P2-001 confirmed: `result ??
last_assistant_message` vs `last_assistant_message ?? result` ordering is
non-trivial; Claude Code Stop hooks commonly omit `result` in favor of
`last_assistant_message`. F-S805-P2-002 confirmed: dot vs hyphen separator
is a genuine behavioral ambiguity.

**Iteration 2:** All 3 MED findings confirmed: each blocks deterministic bats
test authoring. LOW finding is informational.

**Iteration 3:** No findings withdrawn. 4 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S805-P2-001 | Novel | RESULT/last_assistant_message fallback chain not examined at pass-1 |
| F-S805-P2-002 | Novel | Agent pattern notation format not examined at pass-1 |
| F-S805-P2-003 | Novel | AC-007 case (e) input shape not examined at pass-1 |
| F-S805-P2-004 | Novel | env_allow block removal scope not examined at pass-1 |

4 entirely novel findings. No pass-1 partial closures.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 3 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; MED findings prevent advance).

**Trajectory:** 12 → 4 (67% decay). Healthy decay shape.

**Pass-3 priors for adversary:**
- Verify RESULT/last_assistant_message fallback chain pinned in T-3
- Verify agent pattern notation resolved to canonical form (dot vs hyphen + mechanism)
- Verify AC-007 case (e) concrete stdin envelope specified
- Verify T-8 covers full [hooks.capabilities] block removal
