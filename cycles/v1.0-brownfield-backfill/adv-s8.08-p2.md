---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.08-p1.md
target: S-8.08 v1.1
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 9
findings_high: 2
findings_med: 5
findings_low: 2
findings_nit: 0
---

# Adversarial Review: S-8.08 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S808-P2-NNN`

- `F`: Fixed prefix
- `S808`: Story 8.08
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.08 v1.1 (385 lines). 11 of 12 pass-1
findings fully closed; F-003, F-004, and F-007 had residuals contributing to
pass-2 findings. 9 new findings: 2 HIGH, 5 MED, 2 LOW, 0 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3.

Trajectory: 12 → 9 (25% decay). Dominant issue class: subsystem anchor
mis-identification (SS-04 vs primary anchor), BC invariant conflation,
sibling parity gaps (vsdd-hook-sdk path, wasm32-wasip1 rule, wave disclosure).

---

## Part A — Fix Verification (Pass-1 Closure Audit)

11 of 12 pass-1 findings fully closed. F-003/F-004/F-007 had residuals.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S808-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S808-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S808-P1-003 | subsystems += SS-04 | CLOSED (PARTIAL → F-S808-P2-001) | SS-04 added but labeled "cross-CAP stretch" incorrectly |
| F-S808-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S808-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S808-P1-006 | input-hash convention | CLOSED (PARTIAL → F-S808-P2-005) | Hash present but explanatory comment inconsistent |
| F-S808-P1-007 | AC perf gate dropped (Tier 1 exclusion) made advisory | CLOSED | Made advisory |
| F-S808-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED (PARTIAL → F-S808-P2-006) | Path referenced; `path =` specifier missing from Cargo.toml block |
| F-S808-P1-009 | hooks.json positive verification across ACs/Tasks | CLOSED | Verification present |
| F-S808-P1-010 | Goal — registry-only reality | CLOSED | Goal updated |
| F-S808-P1-011 | AC-002 BC-7.03.079 invariant 1 re-anchored | CLOSED (PARTIAL → F-S808-P2-002) | Re-anchored but conflates two distinct invariants |
| F-S808-P1-012 | T-6 exec_subprocess removal aligned | CLOSED | T-6 updated |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S808-P2-001 [HIGH] SS-04 mislabeled as "cross-CAP stretch" — it is the primary anchor for crates/hook-plugins/

**Location:** Frontmatter subsystems description and Architecture section.

**Issue:** The pass-1 burst added SS-04 to subsystems but labeled it as a
"cross-CAP stretch." For track-agent-start, which lives in
`crates/hook-plugins/` (SS-04's primary domain — Plugin Ecosystem), SS-04 is
the PRIMARY anchor, not a stretch. Only SS-01 (dispatcher-core routing) and
SS-07 (bash-layer heritage being replaced) are legitimate stretches. The
mislabeling understates the SS-04 primary relationship and may cause the
BC trace audit to reject SS-04 anchors as improperly categorized.

**Suggested fix:** Remove the "cross-CAP stretch" label from the SS-04 entry.
Label SS-04 as "Primary — Plugin Ecosystem (crates/hook-plugins/)." Reserve
the "stretch" label for SS-01 and SS-07 cross-subsystem anchors.

**Policy:** POLICY 2 (BC anchor integrity — primary vs stretch designation
must be accurate per ARCH-INDEX).

---

### F-S808-P2-002 [HIGH] AC-002 BC trace conflates two distinct BC-7.03.079 invariants

**Location:** AC-002 and BC Trace table, BC-7.03.079.

**Issue:** The pass-1 fix re-anchored AC-002 to BC-7.03.079 Invariant 1
("identity stable over active lifetime"). However, AC-002's acceptance criterion
text conflates this with the "hook path lifecycle" concern — a distinct invariant
about the registry entry remaining valid. These are orthogonal: identity stability
is about agent_id constancy within a session; hook path lifecycle is about the
registry entry persisting across restarts. The conflation risks an implementer
writing a bats fixture that tests the wrong invariant.

**Suggested fix:** Separate AC-002 into two distinct concerns: (a) assert that
the agent_id recorded by track-agent-start matches the agent_id recorded by
track-agent-stop for the same session (identity stability); (b) note hook path
lifecycle as a separate concern covered by the Architecture Compliance section,
not by this AC.

**Policy:** POLICY 2 (BC anchor integrity — each AC must test one BC invariant,
not a conflation of two).

---

### F-S808-P2-003 [MED] T-3 emit_event API call signature unspecified — sibling parity gap

**Location:** Task T-3.

**Issue:** T-3 specifies that the hook calls host::emit_event to record the
agent-start event, but the call signature is not specified. Sibling S-8.03
(after pass-1 fixes) specifies the positional form
`emit_event(event_type, &[(&str, &str)])` with typed examples. S-8.08 has no
equivalent pinned form. An implementer must guess the call form.

**Suggested fix:** Add to T-3 (matching sibling pattern):
```rust
host::emit_event("agent.start", &[
    ("agent_id", &agent_id),
    ("tool_name", &tool_name),
]);
```

**Policy:** POLICY 6 (measurability) and POLICY 7 (cross-document consistency
with sibling S-8.03).

---

### F-S808-P2-004 [MED] T-3 omits stdin read I/O error path and tool_name-absent case

**Location:** Task T-3.

**Issue:** T-3 covers the main path for reading the stdin envelope but does not
specify the error path when stdin read fails (I/O error, empty input, malformed
JSON) or when `tool_name` is absent from the envelope. Both are production cases:
the dispatcher can send partial envelopes under error conditions, and tool_name
is optional in some PostToolUse event variants.

**Suggested fix:** Add to T-3: "If stdin is empty or malformed JSON: return
HookResult::Error with message 'track-agent-start: failed to parse stdin
envelope'. If tool_name absent: use 'unknown' as the fallback value."

**Policy:** POLICY 6 (measurability — all input states must be handled).

---

### F-S808-P2-005 [MED] input-hash explanatory comment internally inconsistent

**Location:** Frontmatter input-hash comment.

**Issue:** The input-hash comment claims "E-8 epic content hash" as the
rationale but the explanatory text describes the hash as "the short-SHA of
S-8.00's seal commit." These are different definitions and cannot both be
the explanation for the same hash value. This is the same conflict identified
across sibling stories (F-S804-P2-007) — the convention is not uniformly
applied.

**Suggested fix:** Remove the explanatory comment and replace with the
canonical form: `# computed by: compute-input-hash <this-file> --update`.

**Policy:** POLICY 7 (cross-document consistency — input-hash convention
must be uniform).

---

### F-S808-P2-006 [MED] vsdd-hook-sdk path specifier missing — sibling parity gap with S-8.03

**Location:** Cargo.toml dependency block.

**Issue:** The Cargo.toml dependency block for vsdd-hook-sdk does not include
the `path` specifier. Sibling S-8.03 specifies:
`vsdd-hook-sdk = { path = "../../hook-sdk", version = "0.1.0" }`.
S-8.08 omits the `path` field, which causes cargo to attempt crates.io
resolution — hook-sdk is not published, so the build fails.

**Suggested fix:** Add `path = "../../hook-sdk"` to the vsdd-hook-sdk
dependency entry.

**Policy:** POLICY 7 (cross-document consistency — sibling Cargo.toml form
must be identical).

---

### F-S808-P2-007 [MED] Architecture Compliance table missing wasm32-wasip1 canonical-target rule

**Location:** Architecture Compliance section.

**Issue:** The Architecture Compliance section does not include a row asserting
`target = "wasm32-wasip1"` as the canonical compilation target. Sibling stories
that converged in earlier passes include this rule. Without it, an implementation
could compile for wasm32-wasi (old target) and pass all other AC checks without
detection.

**Suggested fix:** Add Architecture Compliance row: "Compilation target MUST
be wasm32-wasip1. Verify: `cargo build --target wasm32-wasip1 -p
track-agent-start`."

**Policy:** POLICY 7 (cross-document consistency — sibling parity required).

---

### F-S808-P2-008 [LOW] wave: 15 [process-gap] provisional comment absent — sibling parity drift

**Location:** Frontmatter and story body wave disclosure.

**Issue:** Sibling stories S-8.03 and S-8.06 include `wave: 15 [process-gap]`
provisional disclosure in both frontmatter and body. S-8.08 is missing this
disclosure in one or both locations, creating sibling-parity drift. While
informational, the disclosure pattern should be uniform across all Tier 1
stories.

**Suggested fix:** Add `[process-gap]` qualifier to the wave: 15 disclosure
in frontmatter and body, matching sibling S-8.03 pattern.

**Policy:** POLICY 7 (cross-document consistency — disclosure form must be
uniform).

---

### F-S808-P2-009 [LOW] Behavioral Contracts body table omits AC-002/AC-005/AC-006 trace coverage

**Location:** BC Trace table.

**Issue:** The BC Trace table traces BC-7.03.079 and BC-7.03.080 but does not
cross-reference which ACs cover each BC. Specifically, AC-002 (invariant 1),
AC-005 (emit_event postcondition), and AC-006 (agent-start ordering) are not
cited in the Trace column. Incomplete cross-referencing means reviewers cannot
verify coverage without reading each AC individually.

**Suggested fix:** Add an "Covered by ACs" column to the BC Trace table:
BC-7.03.079 → AC-002, AC-005; BC-7.03.080 → AC-006.

**Policy:** POLICY 2 (BC anchor integrity — trace must show AC→BC coverage).

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.08 subsystems SS-04 labeled as primary | Frontmatter | FAIL (F-S808-P2-001) |
| BC-7.03.079 invariant 1 vs hook path lifecycle | AC-002 | FAIL (F-S808-P2-002) |
| vsdd-hook-sdk path = "../../hook-sdk" present | Cargo.toml | FAIL (F-S808-P2-006) |
| wasm32-wasip1 Architecture Compliance row | Story body | FAIL (F-S808-P2-007) |
| wave: 15 [process-gap] in both locations | Frontmatter + body | FAIL (F-S808-P2-008) |
| emit_event call form pinned in T-3 | T-3 | FAIL (F-S808-P2-003) |
| BC-7.03.080 trace present | BC Trace table | PASS |
| input-hash comment consistent | Frontmatter | FAIL (F-S808-P2-005) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | All required sections present | PASS | |
| POLICY 2 — BC anchor integrity | SS-04 primary label; BC-7.03.079 invariant conflation; trace gaps | FAIL | F-S808-P2-001/002/009 |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | emit_event form; stdin error path; tool_name absent | FAIL | F-S808-P2-003/004 |
| POLICY 7 — Cross-document consistency | vsdd-hook-sdk path; wasm32-wasip1 rule; wave disclosure; input-hash | FAIL | F-S808-P2-005/006/007/008 |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 9 findings. F-S808-P2-001 confirmed HIGH: SS-04
is the primary subsystem for crates/hook-plugins/; "cross-CAP stretch" is
definitionally wrong for this domain. F-S808-P2-002 confirmed HIGH: identity
stability and hook path lifecycle are orthogonal BC concerns; conflating them
in AC-002 risks a false-positive bats test.

**Iteration 2:** Severity confirmed. 2 HIGH (anchor and invariant integrity).
5 MED (implementation-blocking parity gaps). 2 LOW (informational).

**Iteration 3:** No findings withdrawn. 9 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S808-P2-001 | Pass-1 partial closure | F-003 added SS-04 but "cross-CAP stretch" label incorrect |
| F-S808-P2-002 | Pass-1 partial closure | F-011 re-anchored AC-002 but conflated two invariants |
| F-S808-P2-003 | Novel | emit_event call form not examined at pass-1 |
| F-S808-P2-004 | Novel | stdin error path and tool_name absent not examined at pass-1 |
| F-S808-P2-005 | Pass-1 partial closure | F-006 applied input-hash but comment inconsistent |
| F-S808-P2-006 | Pass-1 partial closure | F-008 referenced path; `path =` specifier missing |
| F-S808-P2-007 | Novel | wasm32-wasip1 Architecture Compliance row not examined at pass-1 |
| F-S808-P2-008 | Novel | wave [process-gap] sibling parity not examined at pass-1 |
| F-S808-P2-009 | Novel | BC Trace AC coverage gaps not examined at pass-1 |

4 pass-1 partial closures + 5 net-new findings. Fresh-context compounding
value confirmed.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 2 HIGH + 5 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; HIGH findings present).

**Trajectory:** 12 → 9 (25% decay).

**Pass-3 priors for adversary:**
- Verify SS-04 labeled as Primary (not cross-CAP stretch)
- Verify AC-002 split into identity-stability vs hook-path-lifecycle concerns
- Verify vsdd-hook-sdk path = "../../hook-sdk" present in Cargo.toml
- Verify wasm32-wasip1 Architecture Compliance row added
- Verify emit_event Rust call form pinned in T-3
