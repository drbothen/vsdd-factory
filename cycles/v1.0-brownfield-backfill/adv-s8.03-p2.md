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
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.03-p1.md
target: S-8.03 v1.1
target_file: .factory/stories/S-8.03-native-port-track-agent-stop.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 9
findings_high: 2
findings_med: 4
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.03 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S803-P2-NNN`

- `F`: Fixed prefix
- `S803`: Story 8.03
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.03 v1.1 (356 lines). 12 of 13 pass-1
findings fully closed; F-009 (multiline regex anchor parity) was partially
closed — narrative updated but bats fixture retains old bash `grep -P` form.
9 new findings: 2 HIGH, 4 MED, 2 LOW, 1 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3.

Trajectory: 13 → 9 (31% decay). Dominant issue class: SDK API description
errors (emit_event variadic claim, EC-005 fire-and-forget mismatch) and
workspace integration gaps (Cargo.toml members, [hooks.capabilities] canonical
pattern). The 2 HIGH findings are implementation-blocking.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

12 of 13 pass-1 findings fully closed. F-009 partially closed.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S803-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S803-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S803-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S803-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S803-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S803-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S803-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S803-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S803-P1-009 | Multiline regex anchor parity | CLOSED (PARTIAL → F-S803-P2-002) | Narrative updated; bats fixture still uses grep -P form |
| F-S803-P1-010 | byte-vs-char parity defect | CLOSED | wc -c vs .chars() distinction pinned |
| F-S803-P1-011 | AC-005 BC re-anchored to BC-7.03.082 | CLOSED | BC anchor corrected |
| F-S803-P1-012 | AC-007 malformed JSON graceful exit | CLOSED | AC-007 present |
| F-S803-P1-013 | T-3 whitespace-classifier contradiction | CLOSED | Contradiction resolved |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S803-P2-001 [HIGH] emit_event signature description technically wrong — variadic claim

**Location:** T-3 emit_event call description.

**Issue:** The story describes host::emit_event as accepting "remaining KV fields
as additional arguments" — language that implies a variadic API. The actual SDK
signature is `emit_event(event_type: &str, fields: &[(&str, &str)])`: a fixed
two-argument call with a slice of tuples, not variadic. An implementer reading
this description would generate non-compiling code attempting variadic call syntax.

**Suggested fix:** Replace with the correct SDK signature and a concrete call
example:
```rust
host::emit_event("agent.stop", &[
    ("agent_id", &agent_id),
    ("exit_code", &exit_code_str),
]);
```

**Policy:** POLICY 6 (measurability — implementer must have compilable call form).

---

### F-S803-P2-002 [HIGH] [hooks.capabilities] block migration spec contradicts sibling pattern

**Location:** T-6 capabilities block removal description.

**Issue:** The pass-1 partial fix to F-009 addressed multiline regex in the
narrative but left the `[hooks.capabilities]` block migration spec incomplete:
(a) the spec says to remove `env_allow` and `shell_bypass_acknowledged` fields
from the block but does not address whether the parent `[hooks.capabilities]`
header itself is removed; (b) sibling S-8.01 specifies full block removal while
S-8.03 specifies field removal — a contradiction. Without a canonical ruling,
the bats fixture for T-6 cannot be written deterministically.

**Suggested fix:** Align with S-8.01 canonical: full block removal. State:
"Remove the entire `[hooks.capabilities]` section (header + all fields) — not
just individual fields."

**Policy:** POLICY 7 (cross-document consistency — sibling stories must agree
on capabilities block removal scope).

---

### F-S803-P2-003 [MED] Goal section cites BLOCKED regex with literal `/` instead of bash `|` alternation

**Location:** Goal section, BLOCKED status regex.

**Issue:** The Goal section cites a BLOCKED pattern using literal `/` as separator
(`BLOCKED/FAILED`) in what appears to be a regex alternation. In bash ERE and
Rust regex, alternation uses `|` not `/`. The `|` version is used correctly
elsewhere in the story. The Goal section inconsistency could confuse implementers
reading the story top-down before reaching the correct usage in the body.

**Suggested fix:** Replace `BLOCKED/FAILED` with `BLOCKED|FAILED` in the Goal
section regex citation.

**Policy:** POLICY 7 (cross-document consistency — regex syntax must be
consistent within the story).

---

### F-S803-P2-004 [MED] Workspace Cargo.toml `members` update missing from File Structure / Tasks

**Location:** File Structure section and Tasks list.

**Issue:** The story specifies creation of a new crate at
`crates/hook-plugins/track-agent-stop/` but does not include a File Structure
entry or Task to update the workspace `Cargo.toml` `members = [...]` array.
Without workspace registration, `cargo build --workspace` will not build the
new crate and `cargo build -p track-agent-stop` will fail with "package not
found in workspace."

**Suggested fix:** Add to File Structure: `Cargo.toml (workspace root) — add
crates/hook-plugins/track-agent-stop to members = [...]`. Add corresponding
Task step.

**Policy:** POLICY 1 (lifecycle completeness — all file changes must be listed).

---

### F-S803-P2-005 [MED] `is_ascii_whitespace` doesn't match POSIX `[:space:]` (vertical tab divergence)

**Location:** AC-007 malformed JSON graceful exit, input trimming logic.

**Issue:** AC-007 uses `is_ascii_whitespace()` for input trimming. This diverges
from bash's POSIX `[:space:]` class, which includes vertical tab (0x0B). Bash
strips 0x0B as whitespace; Rust's `is_ascii_whitespace()` does not include 0x0B.
For hook inputs arriving through the dispatcher JSON envelope, vertical tab is
pathological but the divergence should be disclosed or the Rust implementation
should use a POSIX-equivalent check.

**Suggested fix:** Add a disclosure note to AC-007: "Rust trim uses
is_ascii_whitespace() which excludes vertical tab (0x0B); bash trim includes it.
Behavioral divergence on 0x0B inputs is accepted and not tested."

**Policy:** POLICY 8 (scope boundary — bash/Rust divergences must be disclosed).

---

### F-S803-P2-006 [MED] EC-005 specifies "host::emit_event call fails" but SDK has no return value

**Location:** EC-005.

**Issue:** EC-005 describes error handling when "the host::emit_event call fails."
The vsdd-hook-sdk emit_event function is fire-and-forget — it does not return a
Result and cannot fail in a catchable way. There is no Err arm to handle. The
EC premise is structurally incorrect for the actual SDK.

**Suggested fix:** Reframe EC-005 around observable failure modes available to
the hook: "If the hook cannot build a valid emit_event payload (e.g., agent_id
absent), return HookResult::Error with a descriptive message rather than emitting
a malformed event."

**Policy:** POLICY 6 (measurability — EC must describe a testable condition
using real SDK behavior).

---

### F-S803-P2-007 [LOW] input-hash convention cites "develop branch" but sibling S-8.01 cites "factory-artifacts branch"

**Location:** Frontmatter input-hash comment.

**Issue:** The input-hash comment in S-8.03 frontmatter references the "develop
branch" as the SHA source. Sibling S-8.01 and the established input-hash
convention cite "factory-artifacts branch." The convention must be canonicalized
across all Tier 1 stories.

**Suggested fix:** Update comment to reference factory-artifacts branch, matching
the established convention.

**Policy:** POLICY 7 (cross-document consistency).

---

### F-S803-P2-008 [LOW] AC-003 byte-count dual-description adds reader load

**Location:** AC-003.

**Issue:** AC-003 includes a parenthetical alternative: "(wc -c in bash;
.len() in Rust for ASCII-only inputs)." Having two forms in parentheses increases
reader cognitive load without adding contract precision. The story should pick
one canonical form and move the alternative to a note.

**Suggested fix:** Remove the parenthetical; the Rust form `.len()` is canonical
for the WASM port. Add a one-line note: "Note: .len() returns byte count, not
char count — consistent with wc -c semantics for ASCII inputs."

**Policy:** POLICY 1 (lifecycle completeness — spec should be unambiguous).

---

### F-S803-P2-009 [NIT] Task T-5 says "7 cases" but enumeration totals 8

**Location:** Task T-5.

**Issue:** T-5 describes bats test coverage as "7 cases" but the enumerated
list contains 8 bullet points. Count must match enumeration.

**Suggested fix:** Change "7 cases" to "8 cases."

**Policy:** NIT — internal consistency.

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.03 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.081/082 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| vsdd-hook-sdk path dep | Cargo.toml section | PASS |
| emit_event signature correct (non-variadic) | T-3 | FAIL (F-S803-P2-001) |
| [hooks.capabilities] block removal scope canonical | T-6 | FAIL (F-S803-P2-002) |
| Workspace Cargo.toml members update | File Structure / Tasks | FAIL (F-S803-P2-004) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | Workspace Cargo.toml members missing | FAIL | F-S803-P2-004 |
| POLICY 2 — BC anchor integrity | BC-7.03.081/082 anchored | PASS | |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | emit_event variadic claim, EC-005 fire-and-forget | FAIL | F-S803-P2-001, F-S803-P2-006 |
| POLICY 7 — Cross-document consistency | Regex slash-vs-pipe in Goal; sibling capabilities block pattern | FAIL | F-S803-P2-003, F-S803-P2-002 |
| POLICY 8 — Scope boundary | bash/Rust 0x0B divergence undisclosed | PARTIAL | F-S803-P2-005 |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 9 findings. F-S803-P2-001 confirmed HIGH: "variadic"
is a specific and wrong API description that would produce non-compiling code.
F-S803-P2-002 confirmed HIGH: capabilities block removal scope contradicts
sibling S-8.01 and leaves bats fixture indeterminate.

**Iteration 2:** Severity confirmed. 2 HIGH (implementation-blocking). 4 MED
(bats fixture or contract precision). 2 LOW (informational). 1 NIT.

**Iteration 3:** No findings withdrawn. 9 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S803-P2-001 | Novel | emit_event variadic claim introduced in pass-1 fix burst |
| F-S803-P2-002 | Pass-1 partial closure | F-009 narrative updated; bats fixture and sibling conflict not resolved |
| F-S803-P2-003 | Novel | Goal regex slash vs pipe separator not examined at pass-1 |
| F-S803-P2-004 | Novel | Workspace Cargo.toml members gap not examined at pass-1 |
| F-S803-P2-005 | Novel | is_ascii_whitespace 0x0B divergence not examined at pass-1 |
| F-S803-P2-006 | Novel | EC-005 fire-and-forget SDK reality not examined at pass-1 |
| F-S803-P2-007 | Novel | develop vs factory-artifacts branch inconsistency not examined at pass-1 |
| F-S803-P2-008 | Novel | AC-003 dual-description not examined at pass-1 |
| F-S803-P2-009 | Novel | T-5 count mismatch not examined at pass-1 |

1 pass-1 partial closure re-surfaced + 8 net-new findings.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 2 HIGH + 4 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; HIGH findings present).

**Trajectory:** 13 → 9 (31% decay). Lower than sibling average; deeper fix
burst required.

**Pass-3 priors for adversary:**
- Verify emit_event Rust slice-of-tuples call form (not variadic)
- Verify [hooks.capabilities] full-block removal canonical (aligned with S-8.01)
- Verify EC-005 reframed around HookResult::Error (not emit_event failure)
- Verify workspace Cargo.toml members Task present
