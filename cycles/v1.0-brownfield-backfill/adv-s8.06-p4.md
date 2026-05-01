---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.06-native-port-session-learning.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.076.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.077.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.078.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - plugins/vsdd-factory/hooks-registry.toml
  - plugins/vsdd-factory/hooks/session-learning.sh
input-hash: "e441e99"
traces_to: prd.md
pass: p4
previous_review: adv-s8.06-p3.md
target: story
target_file: .factory/stories/S-8.06-native-port-session-learning.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 3
findings_high: 2
findings_medium: 1
findings_low: 2
findings_nit: 0
---

# Adversarial Review: S-8.06 v1.3 (Pass 4)

## Finding ID Convention

Finding IDs use the format: `F-S806-P4-<SEQ>`

- `F`: Fixed prefix
- `S806`: Story identifier
- `P4`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification (8/8 CLOSED)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S806-P3-001 BC-7.03.076 verbatim trace | HIGH | CLOSED | Story lines 134+244 match BC:45 char-for-char |
| F-S806-P3-002 BC title restored | HIGH | CLOSED | Canonical title "session-learning: identity & registry binding" present |
| F-S806-P3-003 hooks-registry line range flag | MEDIUM | CLOSED | T-1b includes registry line range check |
| F-S806-P3-004 top-level [hooks.capabilities] removal | MEDIUM | CLOSED | Capability block restructured correctly |
| F-S806-P3-005 read_file SDK note narrowed | LOW | CLOSED | Note scoped correctly to read_file usage pattern |
| F-S806-P3-006 AC-001 parent block disposition | LOW | CLOSED | Parent block disposition documented |
| F-S806-P3-007 comment-as-requirement softening | LOW | CLOSED | Comments converted to prose |
| F-S806-P3-008 Wave 15 [process-gap] tag removal | NIT | CLOSED | Tag removed from body prose |

**BC-7.03.076 verbatim status: CONFIRMED VERBATIM** — story lines 134 and 244 match BC:45 character-for-character. Third remediation closed.

## Part B — New Findings (9)

### CRITICAL

#### F-S806-P4-001: SS-04 canonical name fabricated claim — POLICY 6 ANTI-FABRICATION HARD GATE FAIL

- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** S-8.06 lines 70 and 77
- **Description:** Lines 70 and 77 use "SS-04 Hook Plugins Runtime" with an adjacent "canonical name confirmed" assertion. This is false at two levels: (1) The canonical SS-04 name per ARCH-INDEX:77 is "Plugin Ecosystem" — "Hook Plugins Runtime" does not match; (2) The story explicitly claims verification was performed ("canonical name confirmed") when it was not. The false verification claim elevates this from a POLICY 6 violation to an anti-fabrication HARD GATE FAIL. SS-04-plugin-ecosystem.md:17 H1 confirms "Plugin Ecosystem" as the canonical name.
- **Evidence:** ARCH-INDEX:77 = "Plugin Ecosystem". SS-04-plugin-ecosystem.md:17 = "# Plugin Ecosystem". S-8.06 lines 70/77 = "SS-04 Hook Plugins Runtime" + "canonical name confirmed".
- **Proposed Fix:** Replace all "Hook Plugins Runtime" occurrences in SS-04 context with "Plugin Ecosystem". Remove false "canonical name confirmed" assertions. Add a genuine verification note citing ARCH-INDEX:77 and SS-04-plugin-ecosystem.md:17.

#### F-S806-P4-002: Self-reference loop filter is fabricated MUST invariant with no BC anchor

- **Severity:** CRITICAL
- **Category:** anti-fabrication
- **Location:** S-8.06 lines 107-112
- **Description:** Lines 107-112 specify a MUST invariant: "The hook MUST filter self-referential events to prevent infinite loops when session-learning itself emits events." This invariant has no anchor in BC-7.03.076, BC-7.03.077, or BC-7.03.078. All three BCs were read directly for this pass — none contain any self-reference filter language. Furthermore, session-learning.sh empirically emits no events (it only writes to a file); the filter concept is semantically incoherent for a non-emitting hook. Pass-2 finding P2-001 "hardened" this invariant by adding specific behavioral language, which strengthened a fabrication rather than anchoring it. Anti-fabrication HARD GATE FAIL.
- **Evidence:** BC-7.03.076:1-80 reviewed — no self-reference filter. BC-7.03.077:1-80 reviewed — no self-reference filter. BC-7.03.078:1-80 reviewed — no self-reference filter. session-learning.sh: no emit_event calls.
- **Proposed Fix:** DELETE lines 107-112 entirely. If a self-reference filter is genuinely required, it must be anchored to a BC that specifies it — create the BC first, then reference it in the story.

#### F-S806-P4-003: EC-001 specifies non-parity behavior with empirically wrong rationale

- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** S-8.06 EC-001 (line 278)
- **Description:** EC-001 prescribes "exit 0 + stderr warning" when the append to the session-learning log file fails. The rationale given is "bash `>>` failure fails silently." This rationale is empirically wrong — session-learning.sh uses `set -euo pipefail`, which means a `>>` failure produces a non-zero exit. The bash source behavior (non-zero exit on append failure) diverges from the EC-001 prescription (exit 0 + warning). If the WASM port must match bash behavior (D-2 parity rule), EC-001 prescribes the wrong exit code. This is a D-2 parity violation with an incorrect supporting rationale.
- **Evidence:** session-learning.sh line 1: `set -euo pipefail`. POSIX: `>>` failure with `pipefail` causes non-zero exit. EC-001 prescription: exit 0.
- **Proposed Fix:** Either: (a) Correct EC-001 to "exit non-zero on append failure" (matching bash parity per D-2); or (b) Explicitly sanction divergence: "EC-001 intentionally diverges from D-2 parity — WASM port uses graceful exit-0 for append failures to avoid blocking hook chains; see [process-gap] anchor at D-2."

### HIGH

#### F-S806-P4-004: Stale cross-reference "the grep test at line 390" (actual location: line 412)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.06 body prose referencing the grep test
- **Description:** A prose cross-reference states "the grep test at line 390." The grep test was relocated during v1.3 expansion and now lives at line 412. The stale line reference will mislead implementers navigating the spec.
- **Proposed Fix:** Update cross-reference to "line 412" or replace with an anchor reference to the section heading instead of a line number.

#### F-S806-P4-005: Sibling-sweep gap — SS-04 "Hook Plugins Runtime" mis-anchor likely in other Tier 1 stories

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Cross-story (S-8.01..S-8.09 siblings)
- **Description:** S-8.06 uses "Hook Plugins Runtime" as a unique non-canonical name, while S-8.02 uses "Hook Plugins" (also non-canonical). The pattern suggests that different story authors independently coined different non-canonical names for SS-04. A systematic sibling sweep is required to identify all non-canonical SS-04 usages. Pending orchestrator dispatch of sibling sweep before pass-5.
- **Proposed Fix:** Orchestrator must conduct a grep sweep: `grep -rn "SS-04" .factory/stories/S-8.0*.md` and verify each match against ARCH-INDEX:77 canonical "Plugin Ecosystem."

### MEDIUM

#### F-S806-P4-006: EC-005 macOS skip clause undermines cross-platform coverage (pending intent)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.06 EC-005
- **Description:** EC-005 includes a macOS skip clause for the pipe-buffer overflow test. AC-001/2/3/4 motivate the WASM port specifically for cross-platform portability (addressing DRIFT-010). Skipping the pipe-buffer overflow test on macOS creates a coverage hole precisely where portability matters. The skip may be intentional (different pipe semantics on macOS) but requires explicit justification. Pending orchestrator intent verification.
- **Proposed Fix:** Pending orchestrator adjudication. If skip is intentional: add rationale ("macOS pipe buffer behavior differs; test would be a false negative"). If skip is not intentional: replace macOS-skip with a parameterized write-until-blocked pattern.

### LOW

#### F-S806-P4-007: Subsumed by F-S806-P4-004

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** N/A
- **Description:** Additional stale line references in the same prose block are subsumed by the F-S806-P4-004 fix scope.
- **Proposed Fix:** Resolved by fixing F-S806-P4-004.

#### F-S806-P4-008: Wave 15 disclosure embeds adversarial finding IDs in body prose

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.06 body prose (multiple sites)
- **Description:** The Wave 15 disclosure block and related prose embed adversarial finding IDs (e.g., "per F-S806-P2-001 hardening") directly in the story body. Story bodies should not reference adversarial review finding IDs — these belong in the Changelog only. Finding IDs in body prose will confuse implementers who read the story without the review context.
- **Proposed Fix:** Remove adversarial finding ID references from body prose. Move to Changelog rows with appropriate version context.

## Process-gap Observations

- Self-reference loop fabrication survived 3 passes and was actively "hardened" by pass-2 P2-001 without any BC anchor verification. Pattern: when an adversary flags a missing invariant, story-writer adds prose invariants from memory rather than reading BCs directly. Process rule needed: any MUST invariant added in a fix burst must cite the BC source by line number.
- Subsystem-name canonicalization is not enforced at story-writer time. Recommend pre-commit grep test: `grep -n "canonical name confirmed" story.md | xargs -I{} grep -q "Plugin Ecosystem" {} || exit 1`.
- Adversarial finding IDs embedded in story body (F-S806-P4-008) — story-writer guidance needed: finding IDs belong in Changelog only.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 2 |
| MEDIUM | 1 |
| LOW | 2 |
| NIT | 0 |

**Overall Assessment:** block — anti-fabrication HARD GATE FAIL (3 CRITICAL including fabricated invariant and false verification claim)
**Convergence:** regression (8→9; 3 CRITICAL not previously surfaced)
**Readiness:** requires revision (v1.4 fix burst required; BC-7.03.076 verbatim confirmed but surrounding fabrications invalidate convergence)

## Verdict

**SUBSTANTIVE** — 3 CRITICAL + 2 HIGH + 1 MED observations. Anti-fabrication HARD GATE FAIL despite BC-7.03.076 verbatim quotes being correct: the fabricated self-reference filter in surrounding prose (lines 107-112) and the false "canonical name confirmed" assertion constitute fabrications in the broader spec body. Clock HELD at 0_of_3.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 4 | 5 | 1 | 1 | 11 |
| p2 | 1 | 4 | 3 | 1 | 9 |
| p3 | 2 | 2 | 3 | 1 | 8 |
| p4 | 3+2C | 1 | 2 | 0 | 9 |

Regression from p3→p4: 8→9 findings; 3 CRITICAL fabrications surfaced for first time.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 9 |
| **Closures** | 8 |
| **Novelty score** | 1.0 (9/9 novel; 3 CRITICAL not previously surfaced) |
| **Median severity** | CRITICAL/HIGH boundary |
| **Trajectory** | 11→9→8→9 |
| **Verdict** | FINDINGS_REMAIN |
