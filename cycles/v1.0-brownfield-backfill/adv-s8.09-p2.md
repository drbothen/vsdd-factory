---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.09-native-port-regression-gate-adapter-retirement.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.09-p1.md
target: S-8.09 v1.1
target_file: .factory/stories/S-8.09-native-port-regression-gate-adapter-retirement.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 9
findings_high: 4
findings_med: 4
findings_low: 1
findings_nit: 0
blocker: D-6
---

# Adversarial Review: S-8.09 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S809-P2-NNN`

- `F`: Fixed prefix
- `S809`: Story 8.09
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.09 v1.1 (647 lines). 14 of 16 pass-1
findings fully closed; F-002 (subsystem anchor) was partially closed and F-009
(BC-7.03.071 invariants) regressed — the new description still does not match
the actual BC file. 9 new findings: 4 HIGH, 4 MED, 1 LOW, 0 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3. D-6 blocker (host::write_file) remains.

Trajectory: 16 → 9 (44% decay). Most complex story in Tier 1; 4 HIGH findings
include a BC fabrication regression, dangling dependency, unregistered open
question, and the universal SS-04/SS-02 mis-anchoring. This story cannot
proceed to implementation until D-6 resolves AND S-8.01..S-8.08 all reach
status=ready.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

14 of 16 pass-1 findings fully closed. F-002 partial; F-009 regressed.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S809-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S809-P1-002 | subsystems += SS-04 | CLOSED (PARTIAL → F-S809-P2-001) | SS-04 added but labeled "Hook SDK" — wrong ARCH-INDEX name |
| F-S809-P1-003 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S809-P1-004 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S809-P1-005 | input-hash convention | CLOSED | Convention applied |
| F-S809-P1-006 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S809-P1-007 | Library table TBD row for host::write_file | CLOSED | TBD row present |
| F-S809-P1-008 | depends_on extended with SDK extension story | CLOSED | depends_on updated |
| F-S809-P1-009 | BC-7.03.071 invariant description | CLOSED (REGRESSED → F-S809-P2-002) | New description still doesn't match BC file |
| F-S809-P1-010 | OQ-6 audit doc schema defined | CLOSED | Schema present |
| F-S809-P1-011 | AC-007 9-pattern coverage all enumerated | CLOSED | All 9 enumerated |
| F-S809-P1-012 | AC-002 blast-radius safeguard | CLOSED | Safeguard present |
| F-S809-P1-013 | AC-011 sequencing restructured | CLOSED (PARTIAL → F-S809-P2-005) | Restructured but internally inconsistent |
| F-S809-P1-014 | AC-012 find command bounded | CLOSED | Command bounded |
| F-S809-P1-015 | AC-008 BC trace fixed | CLOSED | BC-7.03.071 invariant 2 enumerated |
| F-S809-P1-016 | D-6 Option A blocker disclosure | CLOSED | Blocker section present |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S809-P2-001 [HIGH] SS-04 mis-anchored to "Hook SDK" — ARCH-INDEX says SS-04 is "Plugin Ecosystem"; SDK surface belongs in SS-02

**Location:** Frontmatter `subsystems:` and Architecture section.

**Issue:** The pass-1 burst added SS-04 to subsystems with the description
"Hook SDK." Per ARCH-INDEX.md, SS-04 is "Plugin Ecosystem" — the crate
packaging and hooks-registry domain. SS-02 is "Hook SDK and Plugin ABI."
For S-8.09's vsdd-hook-sdk surface (emit_event, read_file, write_file when
available), the correct primary anchor is SS-02, not SS-04. This universal
mis-anchoring (also present in S-8.07 and S-8.08) was introduced by the
pass-1 burst applying SS-04 without verifying ARCH-INDEX canonical names.

**Suggested fix:** Correct subsystems entry: SS-02 (Hook SDK and Plugin ABI)
for the SDK surface. SS-04 (Plugin Ecosystem) for the plugin crate location.
Update Architecture section descriptions to match ARCH-INDEX.

**Policy:** POLICY 2 (BC anchor integrity — subsystem names must match
ARCH-INDEX verbatim).

---

### F-S809-P2-002 [HIGH] BC-7.03.071 invariants in story body FABRICATE content — regression of F-009

**Location:** BC Trace table, BC-7.03.071 invariant description.

**Issue:** Pass-1 F-009 was filed because the BC-7.03.071 invariant description
was incorrect. The fix burst updated the story body BC table, but the new
description still does not match BC-7.03.071 as it exists in the BC file. The
invariant described in the story ("adapter binary present in registry at hook
path") is not stated in BC-7.03.071. The actual BC-7.03.071 content describes
the regression-gate's postcondition on Tier 1 BC anchor completeness. This is
a regression: the fabrication was corrected with a different fabrication.

**Suggested fix:** Open BC-7.03.071 directly and transcribe the actual
postcondition text verbatim into the Trace column. Do not paraphrase. If the
actual BC text is insufficient to drive the AC, file a v1.1 BC candidate to
add the missing invariant.

**Policy:** POLICY 2 (BC anchor integrity — trace content must be transcribed
from the BC file, not written from memory).

---

### F-S809-P2-003 [HIGH] assumption_validations references unregistered "OQ-write_file"

**Location:** Frontmatter `assumption_validations:` field.

**Issue:** The story's assumption_validations field references "OQ-write_file"
as an open question requiring validation. No open question with this ID is
defined in the E-8 epic (E-8-native-wasm-migration.md). The registered OQs are
OQ-1 through OQ-8. An unregistered OQ reference is a dangling identifier that
cannot be validated or tracked.

**Suggested fix:** Either register OQ-write_file in the E-8 epic as a new OQ
(OQ-9?) with a formal question statement, or replace the reference with the
existing OQ that covers this concern (if one exists), or remove the reference
and document the concern as a D-6 dependency item.

**Policy:** POLICY 5 (dependency symmetry — all referenced OQs must be
registered in the epic).

---

### F-S809-P2-004 [HIGH] depends_on lacks SDK extension story — sibling-fix divergence with S-8.04

**Location:** Frontmatter `depends_on:` field.

**Issue:** S-8.04 was updated in the pass-1 burst to add `S-8.SDK-write-file`
to its `depends_on`. S-8.09 has the same SDK extension requirement
(host::write_file) but its `depends_on` was NOT updated to include the SDK
extension story. This creates a sibling divergence: S-8.04 declares a hard
pre-dep; S-8.09 is an orphan on the same dependency. Both must declare the
same pre-dep or both must use the same OQ-based deferral mechanism — they
cannot diverge.

**Suggested fix:** Add the same SDK extension story ID to S-8.09's depends_on
that was added to S-8.04, even if that ID is currently "TBD (SDK extension
host::write_file — D-6 Option A)."

**Policy:** POLICY 5 (dependency symmetry — sibling stories with identical
dependencies must declare them identically).

---

### F-S809-P2-005 [MED] AC-011 sequencing language internally inconsistent

**Location:** AC-011.

**Issue:** AC-011 includes both the phrase "runs as T-7" and "after T-7" in
describing when the adapter retirement audit executes. These cannot both be
true: if the audit runs AS T-7 it is part of T-7; if it runs AFTER T-7 it is
a subsequent task. The internal inconsistency prevents implementers from
determining when the audit is triggered. Note: this was flagged as a partial
close in pass-1 (F-013) but the restructuring introduced a new inconsistency.

**Suggested fix:** Choose one: "AC-011 is verified BY T-7 (runs as part of T-7)"
OR "AC-011 is a post-T-7 check (runs after T-7 completes)." Remove the other.

**Policy:** POLICY 6 (measurability — sequencing must be unambiguous).

---

### F-S809-P2-006 [MED] BC-7.03.075 title drift — BC-INDEX Unicode arrow vs story body ASCII hyphen

**Location:** BC Trace table, BC-7.03.075 row title.

**Issue:** The BC trace table for BC-7.03.075 uses the title "pass-to-fail"
(ASCII hyphen) in the story body, while BC-INDEX.md uses "pass→fail" (Unicode
right arrow). Title drift can cause grep-based cross-reference tools to fail
to locate the BC.

**Suggested fix:** Align to BC-INDEX canonical: "pass→fail" (Unicode U+2192).

**Policy:** POLICY 7 (cross-document consistency — BC titles must match
BC-INDEX exactly).

---

### F-S809-P2-007 [MED] Previous Story Intelligence references S-8.04 as "pattern reference" but S-8.04 is BLOCKED and unconverged

**Location:** Previous Story Intelligence section.

**Issue:** The Previous Story Intelligence section cites S-8.04 as a "pattern
reference" for write_file handling. S-8.04 is itself BLOCKED on the write_file
SDK extension (D-6 Option A) and has not converged (still at pass-1 fix burst;
no CONVERGENCE_REACHED). Citing a blocked, unconverged story as a pattern
reference introduces unreliable guidance — S-8.04's current spec may change
substantially before it converges.

**Suggested fix:** Replace: "See S-8.04 for write_file handling pattern [NOTE:
S-8.04 is BLOCKED on same SDK extension and has not converged; treat as
tentative reference pending D-6 resolution]."

**Policy:** POLICY 8 (scope boundary — pattern references must be stable and
converged).

---

### F-S809-P2-008 [MED] EC-005 conflates jq subprocess with state-file write capability

**Location:** EC-005.

**Issue:** EC-005 describes an error case involving "jq subprocess failure OR
state-file write failure." The jq subprocess is a bash-era dependency being
eliminated in the WASM port. In the native Rust implementation, jq is replaced
by Rust JSON parsing, and state-file write is performed via host::write_file.
The EC conflates these as equivalent failure modes when the WASM port eliminates
one of them entirely.

**Suggested fix:** Reframe EC-005 for the WASM context: "If Rust serde_json
deserialization fails, return HookResult::Error. If host::write_file fails,
return HookResult::Error with the SDK error message. jq subprocess is not
applicable to the WASM port."

**Policy:** POLICY 8 (scope boundary — bash-era failure modes must be removed
from WASM spec).

---

### F-S809-P2-009 [LOW] AC-007 stderr message format doesn't pin newline rendering

**Location:** AC-007.

**Issue:** AC-007 specifies a stderr warning message format but does not pin
whether terminal `\n` is a rendered newline character (0x0A) or the literal
two-character escape sequence. Bats tests use exact string matching and cannot
pass without knowing which form is expected. This is the same gap as
F-S807-P2-007 in sibling S-8.07.

**Suggested fix:** State: "Newlines in the stderr message are rendered as
actual newline characters (0x0A). In bats: use `$'...\n...'` quoting syntax."

**Policy:** POLICY 6 (measurability — escape rendering must be pinned for
bats assertions).

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.09 subsystems SS-02 for SDK surface | Frontmatter | FAIL (F-S809-P2-001) |
| BC-7.03.071 invariant transcribed from BC file | BC Trace table | FAIL (F-S809-P2-002) |
| OQ-write_file registered in E-8 epic | E-8-native-wasm-migration.md | FAIL (F-S809-P2-003) |
| depends_on includes SDK extension (same as S-8.04) | Frontmatter | FAIL (F-S809-P2-004) |
| BC-7.03.075 title uses Unicode arrow | BC Trace table | FAIL (F-S809-P2-006) |
| D-6 Option A blocker disclosure | Story body | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| AC-007 all 9 patterns enumerated | AC-007 | PASS |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | All required sections present | PASS | |
| POLICY 2 — BC anchor integrity | SS-04 identity wrong; BC-7.03.071 fabrication regression | FAIL | F-S809-P2-001/002 |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | OQ-write_file dangling; depends_on sibling divergence | FAIL | F-S809-P2-003/004 |
| POLICY 6 — Measurability | AC-011 sequencing; AC-007 newline rendering | FAIL | F-S809-P2-005/009 |
| POLICY 7 — Cross-document consistency | BC-7.03.075 title drift; S-8.04 unconverged reference | FAIL | F-S809-P2-006/007 |
| POLICY 8 — Scope boundary | EC-005 jq bash-era premise | FAIL | F-S809-P2-008 |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 9 findings. F-S809-P2-002 confirmed HIGH regression:
pass-1 F-009 closed with "adapter binary present" which is still not what
BC-7.03.071 actually says (postcondition on Tier 1 BC anchor completeness).
F-S809-P2-003 confirmed HIGH: "OQ-write_file" is not in OQ-1 through OQ-8.
F-S809-P2-004 confirmed HIGH: S-8.04 has `S-8.SDK-write-file` in depends_on;
S-8.09 does not — the same dependency is declared inconsistently.

**Iteration 2:** Severity confirmed. 4 HIGH (anchor, fabrication regression,
dangling OQ, dependency divergence). 4 MED (implementation-blocking). 1 LOW.

**Iteration 3:** No findings withdrawn. 9 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S809-P2-001 | Pass-1 partial closure | F-002 added SS-04 with wrong ARCH-INDEX name |
| F-S809-P2-002 | Pass-1 regression | F-009 closed with different fabrication |
| F-S809-P2-003 | Novel | OQ-write_file ID unregistered — first identified at pass-2 |
| F-S809-P2-004 | Novel | depends_on sibling divergence with S-8.04 first visible at pass-2 |
| F-S809-P2-005 | Pass-1 partial closure | F-013 restructured AC-011; new inconsistency introduced |
| F-S809-P2-006 | Novel | BC-7.03.075 title drift not examined at pass-1 |
| F-S809-P2-007 | Novel | S-8.04 unconverged reference not examined at pass-1 |
| F-S809-P2-008 | Novel | EC-005 jq/write_file conflation not examined at pass-1 |
| F-S809-P2-009 | Novel | AC-007 newline rendering not examined at pass-1 |

3 pass-1 partial closures/regressions + 6 net-new findings. Fresh-context
compounding value confirmed. The BC-7.03.071 fabrication regression (F-S809-P2-002)
is the most critical quality signal: this is the THIRD attempt to describe
this BC correctly and it is still wrong. The BC file must be read directly.

---

## Part G — Process-Gap Tags

[process-gap] D-6 Option A blocker — host::write_file absent from vsdd-hook-sdk;
implementation cannot proceed until SDK extension story merges.

[process-gap] S-8.09 depends on ALL of S-8.01..S-8.08 reaching status=ready
before implementation can begin. Currently all siblings are in adversarial
convergence at pass-2.

---

## Verdict

**SUBSTANTIVE** — 4 HIGH + 4 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; 4 HIGH + D-6 blocker).

**Trajectory:** 16 → 9 (44% decay).

**Pass-3 priors for adversary:**
- Verify BC-7.03.071 invariant transcribed verbatim from BC file (third attempt)
- Verify SS-02 added for SDK-surface primary anchor; SS-04 for plugin ecosystem
- Verify OQ-write_file registered in E-8 epic or reference removed
- Verify depends_on includes SDK extension story matching S-8.04
- Verify AC-011 "runs as T-7" vs "after T-7" resolved to one form
- Verify BC-7.03.075 title changed to Unicode "pass→fail"
