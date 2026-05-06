---
pass_id: 58
angle: "Glossary/Terminology consistency sweep"
surface: "E-9 epic v1.50 (e6c8f4a) + BC-1.05.035 + BC-1.05.036 + STORY-INDEX v2.09 + lessons.md + open-questions.md + open-backlog-post-rc8.md + pass-53/54/55/56/57 review files; cross-checked against source-of-truth Rust at exec_subprocess.rs / host/mod.rs / internal_log.rs"
anchor_commit: "e6c8f4a"
date: "2026-05-06"
adversary_model: "claude-opus-4-7[1m]"
prior_clock_state: "2_of_3 (advanced from 1_of_3 via D-302 pass-57 NITPICK_ONLY seal)"
final_verdict: "SUBSTANTIVE — 0 HIGH / 1 MEDIUM / 0 LOW + 3 non-blocking observations"
findings_count:
  HIGH: 0
  MEDIUM: 1
  LOW: 0
  observations: 3
clock_state_output: "2_of_3 → 0_of_3 (RESET; SUBSTANTIVE verdict — 1 MEDIUM; 3 fresh NITPICK_ONLY needed for CONVERGENCE_REACHED)"
defect_class: "Narrative-fidelity content defect in v1.46 H3 LOW-P51-002 closure narrative; sibling-class to HIGH-P54-001; 3rd occurrence triggers TD-VSDD-093 codification"
---

# Adversarial Review Pass 58 — E-9 v1.50

## Glossary/Terminology Consistency Sweep

**Pass ID:** 58
**Surface:** E-9 epic v1.50 (e6c8f4a) + BC-1.05.035 + BC-1.05.036 + STORY-INDEX v2.09 + lessons.md + open-questions.md + open-backlog-post-rc8.md + pass-53/54/55/56/57 review files; cross-checked against source-of-truth Rust at exec_subprocess.rs / host/mod.rs / internal_log.rs
**Angle:** Glossary/Terminology consistency sweep — trace every technical term, identifier, function name, error code, and behavioral constant used across the E-9 surface artifacts back to its source-of-truth definition; flag any term used with a different referent in different locations, any term referenced without grounding in a canonical source, and any closure narrative that attributes properties to a term not present in the cited source
**Anchor commit:** e6c8f4a
**Date:** 2026-05-06
**Prior clock state:** 2_of_3 (advanced from 1_of_3 via D-302 pass-57 NITPICK_ONLY seal)
**Model:** claude-opus-4-7[1m]

---

## Procedure Summary

**Scope:** All terminology instances across the E-9 surface — error-code constants (CAPABILITY_DENIED, INVALID_ARGUMENT, TIMEOUT, OUTPUT_TOO_LARGE, INTERNAL_ERROR), function/method references (binary_allowed, emit_denial, execute_bounded, ctx.emit_internal, Path::canonicalize, Path::file_name, unwrap_or_else), behavioral terms (allow-list, deny-path, silent-discard, cause-erasure, best-effort), and changelog closure narratives that describe BC body content, source-code constants, or normative claims.

**Method:** Full surface read-through tracking each technical term to its definition. For changelog H3 closure narratives, cross-validate the narrative prose against the actual BC body content and source-of-truth Rust. Flag any case where the narrative uses a term with a referent not present in the source-of-truth.

**Cross-validation sources consulted:** BC-1.05.035 (BC-035) body content, BC-1.05.036 (BC-036) body content, source-of-truth Rust at exec_subprocess.rs:186-192 for binary_allowed helper, host/mod.rs:179-184 for error-code constant definitions.

---

## Findings

### MED-P58-001 — v1.46 H3 LOW-P51-002 closure narrative `file_name` two-referents misdescription

**Severity:** MEDIUM
**Location:** E-9 epic v1.50, v1.46 H3 changelog block, LOW-P51-002 closure bullet
**Finding class:** Narrative-fidelity content defect — `file_name` used with two contradictory referents in the same closure bullet; sibling-class to HIGH-P54-001 (D-295 v1.46 H3 LOW-P51-001 closure cited fabricated `INVALID_ARGUMENT (-2)`)

#### Evidence

The v1.46 H3 LOW-P51-002 closure bullet reads (approximately):

> BC-035 EC-013 gained a paragraph documenting the `file_name=None` fallback behavior: when the plugin call site does not supply a file_name, the event is still emitted with `file_name` absent from the payload.

**Referent (a) — correct use of `file_name`:** The term `file_name=None` in the first clause correctly refers to the return value of `Path::file_name()` API method — specifically, when `Path::file_name()` returns `None` because the path component ends in `..` or `.` or is `/`. This is the actual content of BC-035 EC-013, which documents the `binary_allowed` helper at exec_subprocess.rs:186-192 calling `PathBuf::from(cmd).file_name()` to extract the basename, with `unwrap_or_else(|| cmd.to_string())` fallback when `Path::file_name()` returns `None` for `cmd = "/"`, `cmd = "."`, `cmd = ".."`.

**Referent (b) — fabricated use of `file_name`:** The second clause ("when the plugin call site does not supply a file_name, the event is still emitted with `file_name` absent from the payload") introduces a completely different semantic frame: `file_name` here is used as if it were an event payload field that a plugin call site optionally supplies. This frame does not exist in BC-035 EC-013. Specifically:
- EC-013 does NOT describe an event emission with a `file_name` payload field
- EC-013 does NOT describe a plugin call site that supplies or does not supply a `file_name`
- The `binary_allowed` helper described in EC-013 does NOT emit events; it performs an allow-list lookup and returns a bool
- There is no "event payload" semantic in EC-013 at all

The closure narrative thus uses `file_name` with two contradictory semantic frames in consecutive clauses: (a) Path API method return value (correct) and (b) plugin-call-site-supplied event payload field (fabricated).

#### Root cause

Same root cause as HIGH-P54-001: the D-295 burst authored both LOW-P51-001 and LOW-P51-002 closure narratives in the same v1.46 H3 block. The author summarized BC body content from memory/paraphrase rather than quote-verifying against the actual BC-035 EC-013 paragraph text. For LOW-P51-001, the paraphrase introduced a wrong error code value. For LOW-P51-002, the paraphrase introduced a fabricated semantic frame (event payload / plugin call site) that is not present in the source-of-truth EC-013 content.

#### Why MEDIUM not HIGH

HIGH-P54-001 (the sibling finding from pass-54) was classified HIGH because the fabricated value (`INVALID_ARGUMENT (-2)`) directly contradicted a source-code constant whose correct value (`-4`) is verifiable from host/mod.rs:183. A reader who relied on the H3 closure prose to understand what BC-035 EC-013 covers would have wrong information about the error code.

MED-P58-001 is classified MEDIUM because the fabrication is semantic-frame rather than value-level: the `file_name=None` reference to the Path API method is correct; only the appended "plugin call site / event payload" clause is fabricated. A reader would understand the correct core mechanism (Path::file_name() None fallback) but would be misled about a nonexistent event-emission / payload-field semantic. Severity reduction from HIGH to MED reflects that the fabricated frame is additive (wrong extra meaning appended) rather than substitutive (wrong value replacing correct value).

#### Impact scope

Per POLICY 1 append-only: the v1.46 H3 prose is preserved as historical record. The defect must be disclosed via v1.51 H3 corrigendum per the precedent set by D-299 v1.49 H3 for HIGH-P54-001. BC-035 EC-013 body content is correct throughout — the defect is confined to the H3 closure narrative.

#### Recurrence pattern

- **1st occurrence (H-P21-001, D-264 v1.21):** Closure narrative invented `TIMEOUT (-7)` and `OUTPUT_TOO_LARGE (-8)`; actual values `-2` and `-3` per host/mod.rs.
- **2nd occurrence (HIGH-P54-001, D-295 v1.46 H3 LOW-P51-001 closure):** Cited `INVALID_ARGUMENT (-2)`; actual value `-4` per host/mod.rs:183.
- **3rd occurrence (MED-P58-001, D-295 v1.46 H3 LOW-P51-002 closure — THIS FINDING):** Used `file_name` with two contradictory referents.

**S-7.02 threshold reached at N=3.** Codification of TD-VSDD-093 (closure-narrative source-of-truth validation discipline) is mandatory at D-303.

---

## Observations (Non-Blocking)

### Obs-P58-001 — `binary_allowed` capitalization consistency across surface

**Class:** Terminology cosmetic — the function identifier `binary_allowed` appears consistently in lowercase snake_case across BC-035 EC-013, lessons.md, and open-backlog-post-rc8.md. The v1.46 H3 LOW-P51-002 closure bullet uses `file_name=None` (with `=` as a name-value separator syntax, not a comparison operator). This convention is non-standard relative to how other BC-035 EC citations use function names without `=` value qualifiers. However, `file_name=None` is how Python-style keyword argument notation is commonly used to describe optional parameters or return-value states, and the BC body itself uses this convention in the EC-013 heading. Non-blocking.

### Obs-P58-002 — `unwrap_or_else` vs `unwrap_or` consistency in surface artifacts

**Class:** Terminology cosmetic — lessons.md TD-VSDD-pattern-tracking section (Fabricated-source-code-constant-in-H3-closure-narrative) does not describe the specific fallback mechanism (`unwrap_or_else(|| cmd.to_string())`). This is appropriate for the pattern-tracking section's scope (tracking the defect pattern, not the BC content). The BC-035 EC-013 body uses `unwrap_or_else` consistently. No cross-artifact inconsistency requiring correction. Non-blocking.

### Obs-P58-003 — `anchor_commit` field across pass-53..57 review frontmatter vs v1.50 actual HEAD

**Class:** Continuity observation — pass-53 uses anchor_commit `82d2a31`, pass-54 uses `2568127`, pass-55/56 use `d17da46`, pass-57 uses `d17da46`. The v1.50 surface has not changed since D-300 (the v1.50 bump commit), so `d17da46` is the correct anchor for passes that reviewed v1.50 unchanged. This pass uses `e6c8f4a` per orchestrator specification; if `e6c8f4a` is the actual factory-artifacts HEAD at pass-58 dispatch, the anchor is correct. If `e6c8f4a` differs from `d17da46` only via non-E-9-epic commits (e.g., STATE.md-only NITPICK seal commits), the anchor is still appropriate since the E-9 epic surface at this commit hash is identical to v1.50. Non-blocking.

---

## 5-Axis Sibling Sweep (TD-VSDD-089 MANDATORY)

1. **Postcondition ↔ Edge Case parity:** MED-P58-001 concerns only the v1.46 H3 closure narrative, not BC-035 EC-013 body content. BC-035 EC-013 body content is verified correct (consistent with Postconditions 1 and 2: canonicalize-then-allow-list-check semantic). No PC↔EC drift.
2. **Cross-BC reference accuracy:** BC-1.05.036 §Related BCs row references BC-1.05.035. No inbound reference from BC-036 to EC-013's file_name fallback path. No cross-BC defect introduced by this finding.
3. **Numeric enumeration:** 3 occurrences of fabricated/misdescribed-content-in-v1.46-H3 pattern (consistent with S-7.02 threshold). 3 fresh NITPICK_ONLY passes needed for CONVERGENCE_REACHED after D-303 closes MED-P58-001.
4. **Parenthetical lists:** N/A — no parenthetical-list modifications required by this finding.
5. **Codification artifact sibling integrity:** TD-VSDD-093 codification at D-303 will add a new TD-VSDD-093 section to lessons.md and update the TD-VSDD-pattern-tracking section (Fabricated-source-code-constant-in-H3-closure-narrative) from N=2 to N=3. TD-VSDD-080 hook extension proposal will be filed in open-backlog-post-rc8.md as TD-VSDD-093-HOOK section.

---

## Self-Application Audit (TD-VSDD-090 — N/A for adversary review files)

Per TD-VSDD-090, the self-application audit gate applies to normative-rule codification bursts, not to adversary review files. This review file is authored by the adversary. The state-manager D-303 burst that seals this review and codifies TD-VSDD-093 MUST perform the TD-VSDD-090 self-application audit before seal.

---

## Novelty Assessment

Pass-58 angle (glossary/terminology consistency sweep) is novel. Prior passes that have examined terminology aspects:
- Pass-29 (D-272): cross-doc terminology drift — focused on fan-out vendor names and NUL-byte attribution
- Pass-34 (D-277): mechanism fix — NUL byte mechanism correction
- Pass-50 (D-293): SOUL #4 silent-failure sweep — focused on let-_ patterns

None of the 57 prior passes performed an exhaustive glossary sweep tracing every technical identifier across surface artifacts back to its source-of-truth definition. The pass-29 terminology angle was narrower (specific vendor/attribution terms). Pass-58's angle systematically checked all function names, error codes, and behavioral constants across all surface artifacts and changelog H3 closure narratives. Novel per TD-VSDD-057.

---

## Verdict

**SUBSTANTIVE — 0 HIGH / 1 MEDIUM / 0 LOW + 3 non-blocking observations.**

MED-P58-001 is a narrative-fidelity content defect in the v1.46 H3 LOW-P51-002 closure narrative. The 3rd occurrence of the "fabricated/misdescribed content in v1.46 H3 closure narrative" pattern reaches the S-7.02 threshold, triggering TD-VSDD-093 codification at D-303.

**ADR-013 clock:** 2_of_3 → 0_of_3 (RESET; SUBSTANTIVE verdict — 1 MEDIUM found). Three fresh NITPICK_ONLY passes (59/60/61) needed for CONVERGENCE_REACHED after D-303 seals this finding.
