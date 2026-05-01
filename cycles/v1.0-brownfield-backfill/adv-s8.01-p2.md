---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.01-native-port-handoff-validator.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.01-p1.md
target: S-8.01 v1.1
target_file: .factory/stories/S-8.01-native-port-handoff-validator.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 4
findings_high: 0
findings_med: 2
findings_low: 2
findings_nit: 0
---

# Adversarial Review: S-8.01 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S801-P2-NNN`

- `F`: Fixed prefix
- `S801`: Story 8.01
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.01 v1.1 (380 lines). All 14 pass-1 findings
verified closed. 4 new findings discovered: 0 HIGH, 2 MED, 2 LOW, 0 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3 per ADR-013 (MED findings prevent advance).

Trajectory: 14 → 4 (71% decay). Dominant issue class: SDK surface ambiguity
(emit_event signature, capabilities block removal scope) and bash-era
premise leakage into WASM spec. Pass-3 expected to resolve to NITPICK_ONLY
if fix burst is comprehensive.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

All 14 pass-1 findings verified closed in v1.1. No partial-fix regressions detected.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S801-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S801-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S801-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S801-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure paragraph present |
| F-S801-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S801-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S801-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S801-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S801-P1-009 | emit_event signature referenced | CLOSED (PARTIAL → F-S801-P2-001) | Signature referenced but Rust call form not pinned |
| F-S801-P1-010 | BC trace re-anchored | CLOSED | BC trace table updated |
| F-S801-P1-011 | read_file capability declaration | CLOSED | Declaration present |
| F-S801-P1-012 | hooks.json positive verification in tasks | CLOSED | T-6 updated |
| F-S801-P1-013 | exec_subprocess removal scope | CLOSED (PARTIAL → F-S801-P2-002) | T-6 covers; AC-001 omits parent-block scope |
| F-S801-P1-014 | wasm compilation target pin | CLOSED | Architecture Compliance row present |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S801-P2-001 [MED] emit_event signature ambiguity — Rust SDK call form not pinned

**Location:** AC-003, AC-004, and T-3.

**Issue:** AC-003, AC-004, and T-3 describe event emission using `type=hook.block`
in bash KV-pair form without disambiguating the SDK translation. The canonical
SDK call is `emit_event(event_type: &str, fields: &[(&str, &str)])` — a positional
event_type argument plus a slice of key-value tuples. The story does not pin
which bash KV field maps to `event_type` and which become entries in the `fields`
slice. The pass-1 fix added emit_event references but stopped short of pinning
the Rust call site with concrete arguments.

**Evidence:** AC-003/AC-004 use bash KV-pair prose ("type=hook.block,
subagent=...") without a Rust call-site example locking the mapping.

**Suggested fix:** Add a concrete Rust call example to T-3:
```rust
host::emit_event("hook.block", &[
    ("subagent", agent_name),
    ("reason", "handoff-not-complete"),
]);
```

**Policy:** POLICY 6 (measurability — implementer must know exact call form).

---

### F-S801-P2-002 [MED] AC-001 missing exec_subprocess block / binary_allow removal scope

**Location:** AC-001 and T-6.

**Issue:** T-6 covers removal of `[hooks.capabilities.exec_subprocess]`. AC-001
verifies the hook runs without that block. However AC-001 does not specify the
removal scope: does the entire `[hooks.capabilities]` top-level header get
removed (if exec_subprocess was its only child), or does a parent stub remain?
The two cases produce different hooks.json structures and different bats fixture
assertions.

**Suggested fix:** Explicitly state in AC-001: "After T-6, the hooks.json entry
MUST NOT contain any `[hooks.capabilities]` block (full block removal, not just
sub-key removal)."

**Policy:** POLICY 6 (measurability — AC must be deterministically testable).

---

### F-S801-P2-003 [LOW] File Structure src/main.rs "if standalone binary pattern" hedging contradicts canonical pattern

**Location:** File Structure section, src/main.rs entry.

**Issue:** The src/main.rs entry carries the qualifier "if standalone binary
pattern" — a hedge that contradicts the canonical hook pattern used by sibling
`capture-commit-activity`, which ships both `lib.rs` and a `[[bin]]` target in
the same crate. The story should commit to one form or explicitly disclose both.

**Suggested fix:** Remove the qualifier; state canonical form:
`src/main.rs` as `[[bin]]` entry mirroring `capture-commit-activity`.

**Policy:** POLICY 1 (lifecycle completeness — implementation guidance must be
unambiguous).

---

### F-S801-P2-004 [LOW] EC-005 premise inapplicable to native Rust port — CLAUDE_PLUGIN_ROOT is bash-context

**Location:** EC-005.

**Issue:** EC-005 references `CLAUDE_PLUGIN_ROOT` as an environment variable for
error handling context. This is a bash-context shell variable with no mapping in
the Rust WASM SDK environment. For a native Rust port, file paths are resolved
through `host::read_file` path arguments, not environment variables.

**Suggested fix:** Reframe EC-005: "If host::read_file returns
HostError::PathNotAllowed, the hook should return HookResult::Error with a
structured message."

**Policy:** POLICY 8 (scope boundary clarity — bash-era premises must not leak
into WASM spec).

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.01 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.042/043/044 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| vsdd-hook-sdk path dep | Cargo.toml section | PASS |
| emit_event Rust call form pinned | T-3 | FAIL (F-S801-P2-001) |
| [hooks.capabilities] full-block removal in AC-001 | AC-001 | FAIL (F-S801-P2-002) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | All required sections present | PASS | |
| POLICY 2 — BC anchor integrity | BC-7.03.042/043/044 anchored | PASS | |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | emit_event call form and removal scope unspecified | PARTIAL | F-S801-P2-001, F-S801-P2-002 |
| POLICY 7 — Cross-document consistency | No cross-doc issues | PASS | |
| POLICY 8 — Scope boundary clarity | CLAUDE_PLUGIN_ROOT bash residue | FAIL | F-S801-P2-004 |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 4 findings. F-S801-P2-001 confirmed: pass-1 added
emit_event text but the Rust call site with concrete `(&str, &[(&str, &str)])`
form is absent. F-S801-P2-002 confirmed: T-6 removes exec_subprocess sub-key;
parent block fate unspecified in AC-001.

**Iteration 2:** Severity check confirmed. F-S801-P2-001 and F-S801-P2-002 are
MED — both affect implementer decision-making on the hook's primary function.
F-S801-P2-003 and F-S801-P2-004 are LOW — informational/clarity gaps.

**Iteration 3:** No findings withdrawn. 4 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S801-P2-001 | Pass-1 partial closure | F-P1-009 added emit_event refs; Rust call form not pinned |
| F-S801-P2-002 | Pass-1 partial closure | F-P1-013 covered T-6; AC-001 parent-block scope omitted |
| F-S801-P2-003 | Novel | main.rs binary pattern hedge not examined at pass-1 |
| F-S801-P2-004 | Novel | EC-005 bash-era CLAUDE_PLUGIN_ROOT not examined at pass-1 |

Fresh-context compounding value confirmed: 2 pass-1 partial closures
re-surfaced + 2 net-new findings.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 2 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; MED findings prevent advance).

**Trajectory:** 14 → 4 (71% decay). Healthy late-pass decay shape.

**Pass-3 priors for adversary:**
- Verify emit_event Rust call form pinned with concrete argument mapping
- Verify [hooks.capabilities] full-block removal specified in AC-001
- Confirm EC-005 reframed in WASM SDK terms (no CLAUDE_PLUGIN_ROOT)
