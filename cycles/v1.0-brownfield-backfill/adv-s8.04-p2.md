---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.04-p1.md
target: S-8.04 v1.1
target_file: .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 11
findings_high: 3
findings_med: 5
findings_low: 2
findings_nit: 1
blocker: D-6
---

# Adversarial Review: S-8.04 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S804-P2-NNN`

- `F`: Fixed prefix
- `S804`: Story 8.04
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.04 v1.1 (485 lines). All 17 pass-1 findings
verified closed. 11 new findings: 3 HIGH, 5 MED, 2 LOW, 1 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3. D-6 blocker (host::write_file) remains; implementation cannot proceed.

Trajectory: 17 → 11 (35% decay). Dominant issue class: dangling dependency
IDs, fixture under-specification, BC line number drift, and input-hash
convention conflicts. The HIGHEST RISK story in Tier 1 continues to surface
the most findings per pass.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

All 17 pass-1 findings verified closed in v1.1. No partial-fix regressions.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S804-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S804-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S804-P1-003 | subsystems += SS-04 | CLOSED | Frontmatter updated |
| F-S804-P1-004 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S804-P1-005 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S804-P1-006 | input-hash convention | CLOSED | Convention applied |
| F-S804-P1-007 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S804-P1-008 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED | Path dep present |
| F-S804-P1-009 | emit_event signature | CLOSED | Signature referenced |
| F-S804-P1-010 | YAML trichotomy pinned | CLOSED | None/missing-key/"not_started" documented |
| F-S804-P1-011 | TOCTOU non-atomic write disclosed | CLOSED | Disclosure present |
| F-S804-P1-012 | bash ERE alternation precedence | CLOSED | Precedence clarified |
| F-S804-P1-013 | EC-003 emission contract corrected | CLOSED | EC-003 updated |
| F-S804-P1-014 | python3 drift addressed | CLOSED | Library table updated |
| F-S804-P1-015 | Library table TBD row for host::write_file | CLOSED | TBD row present |
| F-S804-P1-016 | depends_on extended with SDK extension story | CLOSED | depends_on updated |
| F-S804-P1-017 | D-6 Option A blocker disclosure | CLOSED | Blocker section present |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S804-P2-001 [HIGH] depends_on references non-existent story ID `S-8.SDK-write-file`

**Location:** Frontmatter `depends_on:` field.

**Issue:** The pass-1 fix burst updated `depends_on` to reference
`S-8.SDK-write-file` as the SDK extension story prerequisite. No story with
this ID exists in STORY-INDEX.md — there is no S-8.SDK-* family. The dangling
ID causes any tooling that validates depends_on against STORY-INDEX to fail.
The ID must be registered in STORY-INDEX immediately, or replaced with a
provisional placeholder (e.g., `TBD-SDK-write-file`) and tracked via an open
question.

**Suggested fix:** Either register the SDK extension story in STORY-INDEX with
a confirmed ID (e.g., S-8.10 or similar, per the E-8 Tier 2 story numbering
sequence), or replace the ID with the literal string "TBD (SDK extension for
host::write_file — see D-6 Option A)" and add an OQ.

**Policy:** POLICY 5 (dependency symmetry — depends_on entries must be
resolvable in STORY-INDEX).

---

### F-S804-P2-002 [HIGH] wave-state-gate-null.yaml fixture has no content specification

**Location:** AC-006, bats fixture table.

**Issue:** AC-006 requires a bats fixture `wave-state-gate-null.yaml` to test
the null/empty YAML edge case. The story does not specify the file content:
options include a zero-byte file, a YAML file with explicit null values (`gate:
~`), or a YAML file with missing keys entirely. These three cases trigger
different code paths in the YAML parsing logic and cannot all be covered by one
unnamed fixture. Without content specification, the implementer must guess.

**Suggested fix:** Specify the fixture content verbatim in AC-006:
```yaml
# wave-state-gate-null.yaml
gate: ~
current_wave: null
```
And note the expected behavior for each null variant if they differ.

**Policy:** POLICY 6 (measurability — fixture content must be deterministic).

---

### F-S804-P2-003 [HIGH] bash regex precedence bats fixture lacks concrete input/output in AC-006

**Location:** AC-006 bats fixture table.

**Issue:** The pass-1 fix (F-P1-012) clarified the bash ERE alternation
precedence in narrative form. AC-006 acknowledges the precedence risk but does
not enumerate a specific bats test case with a pathological input exercising the
precedence ambiguity (e.g., a STEP_COMPLETE pattern that matches with `|`
alternation but would fail with concatenation). Without a concrete
input/expected-output pair, a bats test written from this AC cannot be
deterministic.

**Suggested fix:** Add to AC-006: "Test case: STEP_COMPLETE pattern
`(merge|squash)_complete` with input containing `merge_complete` → expected
match; with input containing `squash_complete` → expected match; with input
containing `merge_squash_complete` (concatenated) → expected NO match."

**Policy:** POLICY 6 (measurability — bats test must have concrete inputs).

---

### F-S804-P2-004 [MED] EC-008 cites `make wave-state-rebuild` recovery path — command unverified

**Location:** EC-008.

**Issue:** EC-008 documents a recovery procedure using `make wave-state-rebuild`.
This make target has not been verified to exist in the repository Makefile.
Citing an unverified recovery command in an Edge Case section creates false
operator confidence and will produce a confusing error if the command is absent.

**Suggested fix:** Either verify the Makefile target exists and cite the exact
file + line, or replace with a verified alternative (e.g., a git-based rebuild
command) and mark the make target as "proposed pending Makefile addition."

**Policy:** POLICY 6 (measurability — cited commands must be verifiable).

---

### F-S804-P2-005 [MED] BC-7.03.083 postcondition "Requires jq AND python3" contradicts WASM port

**Location:** BC Trace table, BC-7.03.083 row.

**Issue:** BC-7.03.083 Postcondition includes "Requires jq AND python3 to be
available on the host." The WASM port eliminates bash subprocess execution,
which is the only reason jq and python3 are required. The WASM implementation
should not have this postcondition. The story must acknowledge this BC
postcondition divergence and propose a v1.1 BC candidate to remove the
runtime-dependency clause.

**Suggested fix:** Add to the BC Trace row for BC-7.03.083: "[WASM PORT
DIVERGENCE: Postcondition clause 'Requires jq AND python3' does not apply.
v1.1 BC candidate: remove this clause — WASM port uses host::read_file +
host::write_file; no subprocess required.]"

**Policy:** POLICY 2 (BC anchor integrity — BC divergences must be explicitly
acknowledged with v1.1 candidate proposal).

---

### F-S804-P2-006 [MED] BC-7.03.083 cites registry binding at lines 877-894 — stale line numbers

**Location:** BC Trace table, BC-7.03.083 registry binding citation.

**Issue:** The story's BC trace table cites BC-7.03.083 as "registry binding
lines 877-894." The actual binding in hooks-registry.toml is at lines 942-948
as of main HEAD at 1485d2e. Stale line citations are a known defect class —
they pass casual review but mislead implementers doing cross-reference lookups.

**Suggested fix:** Update the line citation to 942-948 or convert to a section
anchor citation that does not depend on line numbers.

**Policy:** POLICY 7 (cross-document consistency — cited line numbers must be
current).

---

### F-S804-P2-007 [MED] input-hash convention conflicts with sibling S-8.03

**Location:** Frontmatter input-hash comment.

**Issue:** The input-hash comment in S-8.04 references "E-8 epic content hash"
as the rationale. Sibling S-8.03 references "short-SHA of seal commit." These
are different conventions and cannot both be canonical. All Tier 1 stories must
use one consistent input-hash convention.

**Suggested fix:** Align all Tier 1 stories to the factory-artifacts convention
per the established pattern in S-8.00 (input-hash = MD5 of inputs files,
computed by compute-input-hash tool).

**Policy:** POLICY 7 (cross-document consistency — input-hash convention must
be uniform across sibling stories).

---

### F-S804-P2-008 [MED] T-9 silent on `jq` removal from binary_allow

**Location:** Task T-9.

**Issue:** Task T-9 covers removal of `[hooks.capabilities.exec_subprocess]`
and binary removals. The current hooks.json for update-wave-state-on-merge
includes `jq` in binary_allow. If the WASM port eliminates the jq subprocess
call (which it does — host::write_file replaces it), `jq` must be removed from
binary_allow. T-9 does not mention this removal, creating a gap between the
task description and the expected hooks.json outcome.

**Suggested fix:** Add to T-9: "Remove `jq` from `binary_allow` (jq subprocess
eliminated by host::write_file; no longer needed)."

**Policy:** POLICY 1 (lifecycle completeness — all file changes must be listed).

---

### F-S804-P2-009 [LOW] EC-005 says "via host::emit_event or stderr" — emit_event is not a stderr surface

**Location:** EC-005.

**Issue:** EC-005 lists error reporting surfaces as "host::emit_event or stderr."
These are distinct mechanisms. `host::emit_event` sends a structured event to
the dispatcher; stderr is plain text captured by the harness. The "or"
conjunction implies interchangeability. The correct formulation is:
structured events via `host::emit_event`; human-readable warnings via stderr
(or `host::log_warn` if the SDK exposes it).

**Suggested fix:** Split: "Emit a structured error event via host::emit_event;
additionally write a human-readable warning to stderr for operator visibility."

**Policy:** POLICY 6 (measurability — error reporting surface must be explicit).

---

### F-S804-P2-010 [LOW] AC-007 perf exclusion lacks objective threshold for "YAML-I/O hooks" classification

**Location:** AC-007.

**Issue:** AC-007 excludes this story from the E-8 AC-7 performance gate as a
"YAML-I/O hook." No objective threshold is given for what constitutes a
YAML-I/O hook. If the classification is determined by I/O time dominating
invocation time, a measurable threshold should be stated (e.g., ">50% of wall
time spent in YAML parse/write").

**Suggested fix:** Add: "Classification criterion: hooks that perform YAML file
I/O (read + parse + write) where I/O is expected to dominate invocation time
at >50% wall time. Performance gate excluded for this class per E-8 AC-7 Tier 1
exclusion."

**Policy:** POLICY 6 (measurability — classification criteria must be objective).

---

### F-S804-P2-011 [NIT] Token Budget table "Sonnet baseline" — stale model reference

**Location:** Token Budget section.

**Issue:** The Token Budget table entry references "Sonnet baseline" without
specifying the Sonnet version. Other Tier 1 stories updated this wording in the
pass-1 burst; S-8.04 has a residual stale reference.

**Suggested fix:** Replace "Sonnet baseline" with the current model identifier
or remove the model-version qualifier entirely.

**Policy:** NIT — minor staleness.

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.04 subsystems includes SS-04 | Frontmatter | PASS |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.083/084/085/086 trace present | BC Trace table | PASS |
| D-6 Option A blocker disclosure | Story body | PASS |
| depends_on S-8.SDK-write-file registered in STORY-INDEX | STORY-INDEX | FAIL (F-S804-P2-001) |
| wave-state-gate-null.yaml fixture content specified | AC-006 | FAIL (F-S804-P2-002) |
| BC-7.03.083 line numbers current | BC Trace table | FAIL (F-S804-P2-006) |
| input-hash convention consistent with siblings | Frontmatter comment | FAIL (F-S804-P2-007) |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | jq binary_allow removal missing from T-9 | FAIL | F-S804-P2-008 |
| POLICY 2 — BC anchor integrity | BC-7.03.083 jq/python3 postcondition divergence | FAIL | F-S804-P2-005 |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on S-8.SDK-write-file dangling | FAIL | F-S804-P2-001 |
| POLICY 6 — Measurability | Fixture content, regex input, command verification | FAIL | F-S804-P2-002/003/004 |
| POLICY 7 — Cross-document consistency | Line numbers stale; input-hash conflict | FAIL | F-S804-P2-006/007 |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 11 findings. F-S804-P2-001 confirmed HIGH: dangling
S-8.SDK-write-file ID is unresolvable by any tooling. F-S804-P2-002 confirmed
HIGH: null fixture without content specification cannot be implemented
deterministically. F-S804-P2-003 confirmed HIGH: no concrete test input for
regex precedence means the test cannot be written.

**Iteration 2:** Severity confirmed. 3 HIGH (spec-integrity blocking). 5 MED
(implementation-impacting). 2 LOW (informational). 1 NIT.

**Iteration 3:** No findings withdrawn. 11 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S804-P2-001 | Novel | Dangling story ID introduced by pass-1 fix (F-P1-016) |
| F-S804-P2-002 | Novel (story-writer prior flagged) | wave-state-gate-null.yaml content unspecified |
| F-S804-P2-003 | Novel (story-writer prior flagged) | regex precedence input/output not pinned |
| F-S804-P2-004 | Novel (story-writer prior flagged) | make target unverified |
| F-S804-P2-005 | Novel | BC-7.03.083 jq/python3 postcondition divergence not examined at pass-1 |
| F-S804-P2-006 | Novel | BC line numbers stale post-rc.1 tag |
| F-S804-P2-007 | Novel | input-hash convention conflict first visible with sibling comparison |
| F-S804-P2-008 | Novel | jq binary_allow removal gap not examined at pass-1 |
| F-S804-P2-009 | Novel | emit_event vs stderr surface conflation not examined at pass-1 |
| F-S804-P2-010 | Novel | YAML-I/O classification criterion not examined at pass-1 |
| F-S804-P2-011 | Novel | Sonnet baseline staleness residual |

8 novel + 3 story-writer-prior items. Fresh-context value confirmed.

---

## Part G — Process-Gap Tags

[process-gap] D-6 Option A blocker — host::write_file absent from vsdd-hook-sdk;
implementation cannot proceed until SDK extension story merges.

---

## Verdict

**SUBSTANTIVE** — 3 HIGH + 5 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; HIGH findings + D-6 blocker).

**Trajectory:** 17 → 11 (35% decay). Highest absolute finding count of Tier 1
pass-2 batch.

**Pass-3 priors for adversary:**
- Verify S-8.SDK-write-file ID registered in STORY-INDEX (or replaced with OQ)
- Verify wave-state-gate-null.yaml fixture content specified verbatim
- Verify bash regex precedence bats case has concrete input/output
- Verify BC-7.03.083 postcondition divergence acknowledged with v1.1 candidate
- Verify BC line numbers updated to 942-948
