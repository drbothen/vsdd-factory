---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p7.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.012.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.040.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.041.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.042.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.043.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.044.md
  - crates/hook-sdk/src/payload.rs
input-hash: "95d6e01"
traces_to: prd.md
pass: p8
previous_review: adv-s8.05-p7.md
story_id: "S-8.05"
story_version: "1.7"
story_input_hash: "95d6e01"
pass_number: 8
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 0
findings_medium: 2
findings_low: 2
findings_nit: 1
---

# Adversarial Review Pass-8 — S-8.05 v1.7

## Finding ID Convention

`F-S805-P8-NNN`

## Part A — Pass-7 Fix Verification

Pass-7 findings:

| Finding | Severity | Status |
|---------|----------|--------|
| F-P7-001 CRITICAL (typed-projection fields cited but absent from HookPayload; S-8.30 missing from depends_on) | CRITICAL | **CLOSED** — Phase F v1.6→v1.7: S-8.30 added to depends_on; T-0 STOP CHECK added; cross-story consistency applied to S-8.01/02/03/05 |
| F-P7-002 MED (transitive SDK pin drift) | MED | **CLOSED** — transitive dependency pinning addressed in v1.7 |
| F-P7-003 LOW (T-0 grep type-drift weakness) | LOW | **OPEN** — SKIP-FIX-eligible; carried forward |

**Universal-patch anchors: ALL PASS.** wasm32-wasip1 target confirmed; vsdd-hook-sdk path `../../hook-sdk` confirmed; hooks.json positive-verification confirmed; SS-01/02/04/07 subsystems confirmed; CAP-022 cross-CAP stretch disclosure confirmed; wave:15 [process-gap] confirmed; HOST_ABI_VERSION=1 confirmed; S-8.29/S-8.30 references confirmed.

**Anti-fabrication HARD GATE: PASS.** BC-2.02.012 PC-5+PC-6 verbatim chains present in T-3 and AC-003; BC-7.04.040/041/042/043/044 quotations verbatim.

**process-gap-D-183-A audit: PASS.** Typed-projection fully specified; no `envelope.get(...)` references.

**process-gap-D-184-A audit: PASS.** T-0 STOP CHECK anchors against payload.rs field names; S-8.30 in depends_on.

## Part B — New Findings (Pass-8)

### CRITICAL / HIGH

None.

### MEDIUM

#### F-S805-P8-001 — T-5 emit_event snippet calls `.as_str()` on `&str` — compile error

- **Severity:** MEDIUM (mechanical implementation blocker)
- **Category:** spec-fidelity / interface-gaps
- **Location:** Story around T-5 emit_event snippet (approx. line 478) — `("subagent", agent.as_str())`
- **Description:** The T-5 code snippet has `("subagent", agent.as_str())`. The `agent` variable is bound via BC-2.02.012 Postcondition 5 canonical fallback chain: `payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")`. This expression produces a `&str`. The `&str` type does NOT have an `.as_str()` method — calling `.as_str()` on a `&str` is a compile error in Rust. The implementer following this snippet verbatim will get: `error[E0599]: no method named 'as_str' found for reference '&str'`.
- **Evidence:** The `as_str()` method exists on `String`, `&String`, and `OsStr` — NOT on `&str`. BC-2.02.012 Postcondition 5 canonical form produces a `&str` (via `as_deref()`/`unwrap_or()`). Using `agent` directly as the second element of the tuple `("subagent", agent)` is correct and sufficient.
- **Proposed Fix:** Change `("subagent", agent.as_str())` to `("subagent", agent)`. The `agent: &str` binding is already the correct type for the `&[(&str, &str)]` slice.

#### F-S805-P8-002 — AC-007 first JSON example mislabeled as Case (e) — content is Case (a)

- **Severity:** MEDIUM (will mislead bats test author)
- **Category:** spec-fidelity / ambiguous-language
- **Location:** AC-007 section, first concrete JSON example (approx. line 267)
- **Description:** The first JSON example in AC-007 is labeled "Case (e) concrete input" but its content — `"last_assistant_message": "wrote pr-review.md and posted gh pr review --approve"` — clearly matches Case (a) (all three checks pass, exit 0 all-pass scenario). Case (e) is the no-verdict scenario where result contains `gh pr review` but lacks `approve|request-changes` tokens. The mislabel will cause the bats test author to write a test expecting exit 2 (Case (e) = no-verdict = check 3b error) using input that should produce exit 0 (Case (a) = all-pass). This is a functional test-authoring error.
- **Evidence:** Case (a) in AC-007 = "all three checks pass → exit 0". The first JSON example has `last_assistant_message` containing "wrote pr-review.md" (passes Check 1), no "gh pr comment" (passes Check 2), "posted gh pr review --approve" (passes Check 3a + 3b verdict). This is unambiguously Case (a). The second JSON example (with "ran gh pr review --no-body") is correctly Case (e).
- **Proposed Fix (option b — preferred for symmetry):** Relabel the first JSON example as "Case (a) all-pass concrete input (for reference)" and label the second as "Case (e) concrete input (no-verdict; deterministic bats test authoring)".

### LOW

#### F-S805-P8-003 — T-0 STOP CHECK grep weaker than prose (field-name only, not Option<String>)

- **Severity:** LOW (pending intent verification)
- **Location:** S-8.05 T-0 STOP CHECK grep pattern in Tasks section
- **Description:** The T-0 STOP CHECK grep matches the four field names by name only (`pub (agent_type|subagent_name|last_assistant_message|result):`), not by declared type. A type change in payload.rs away from `Option<String>` would pass this gate silently, violating the intent of T-0 as a compile-guarantee.
- **Proposed Fix:** Strengthen to: `grep -E 'pub (agent_type|subagent_name|last_assistant_message|result): Option<String>'`
- **Disposition:** Same pattern flagged in S-8.01/02/03; pending intent verification on portability.

#### F-S805-P8-004 — assumption_validations: [] empty despite explicit T-0 STOP CHECK

- **Severity:** LOW
- **Location:** S-8.05 frontmatter `assumption_validations: []` (approx. line 44)
- **Description:** The frontmatter `assumption_validations` array is empty. The story has an explicit T-0 STOP CHECK that validates the assumption that S-8.30 has merged and HookPayload contains the four BC-2.02.012 fields. This assumption is material to compile-time correctness and should be recorded in `assumption_validations` for implementer awareness.
- **Proposed Fix:** Populate `assumption_validations`:
  ```yaml
  assumption_validations:
    - "S-8.30 SDK extension merged: HookPayload has agent_type/subagent_name/last_assistant_message/result as #[serde(default)] Option<String>; verified by T-0 STOP CHECK grep on crates/hook-sdk/src/payload.rs"
    - "BC-2.02.012 Postconditions 5+6 typed projection chains compile against the extended HookPayload"
  ```

### NIT

#### F-S805-P8-005 — T-7 cites "7 cases" but g.1/g.2 sub-cases imply 8

- **Severity:** NIT
- **Location:** S-8.05 T-7 bats parity tests task
- **Description:** T-7 says "(7 cases per AC-007)" but AC-007 enumerates cases (a) through (g) where (g) is the non-pr-reviewer case AND T-5 defines agent fallback sub-cases g.1 (agent_type present) and g.2 (agent_type absent, subagent_name fallback). The two agent-fallback sub-cases are distinct bats test inputs. The "7 cases" count may cause the implementer to write only 7 bats tests and miss the g.1/g.2 sub-case split.
- **Proposed Fix:** Change "(7 cases per AC-007)" to "(7 cases per AC-007 + 2 agent-fallback sub-cases g.1/g.2 per T-5)".
- **Disposition:** Cosmetic precision; SKIP-FIX-eligible, but the two MED findings above require a fix burst anyway.

## Open Questions

None beyond the MED findings above which require fix burst.

## Pass-9 Priors

1. Re-verify F-S805-P8-001 fix: `agent.as_str()` removed; `agent` used directly in tuple.
2. Re-verify F-S805-P8-002 fix: AC-007 first example labeled "Case (a) all-pass" correctly.
3. Verify assumption_validations populated (F-S805-P8-004).
4. Verify T-7 case count updated (F-S805-P8-005).
5. Anti-fabrication HARD GATE on BC-2.02.012 PC-5+PC-6 verbatim.

## Verdict

**SUBSTANTIVE** — clock 0/3 HELD.

Two MEDIUM findings (F-P8-001 compile error in T-5 snippet; F-P8-002 AC-007 example mislabel) require fix burst before clock can advance. Two LOW + one NIT: T-0 type-drift, empty assumption_validations, T-7 case count.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 4 | 5 | 2 | 1 | 12 |
| p2 | 0 | 3 | 1 | 0 | 4 (→ 7 per story) |
| p3 | 0 | 2 | 2 | 1 | 5 |
| p4 | 2 | 1 | 1 | 0 | 4 |
| p5 | 2 | 1 | 1 | 0 | 4 |
| p6 | 2 | 2 | 1 | 0 | 5 |
| p7 | 1 | 0 | 1 | 1 | 3 (CRITICAL closed via Phase F) |
| p8 | 0 | 2 | 2 | 1 | 5 |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 4 (F-P8-001 compile-error; F-P8-002 AC-007 mislabel; F-P8-003 T-0 type-drift; F-P8-004 assumption_validations) |
| **Duplicate/variant findings** | 1 (F-P8-005 T-7 count carryover variant) |
| **Novelty score** | 4/5 = 0.80 |
| **Median severity** | MEDIUM |
| **Trajectory** | 12→7→5→4→4→5→3→5 |
| **Verdict** | FINDINGS_REMAIN — SUBSTANTIVE (clock held at 0/3) |

Pass-8 surfaces new MEDIUM defects at the borrow/method-resolution layer (one layer deeper than pass-7's field-existence layer). F-P8-001 (`agent.as_str()` on `&str`) is the kind of defect that survives prose review but fails the compiler — confirms the process-gap-D-185-A hypothesis that adversarial review must include method-resolution against declared binding types.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** BLOCK — fix burst required for 2 MED findings before clock can advance.

**Convergence:** Clock 0/3 HELD. v1.7 → v1.8 fix burst required (5 findings: F-P8-001/002/003/004/005).

**Readiness:** Requires v1.8 fix burst, then pass-9.
