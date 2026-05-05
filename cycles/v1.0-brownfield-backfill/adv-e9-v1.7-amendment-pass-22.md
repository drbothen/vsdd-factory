---
document_type: adversarial-review
level: ops
pass: 22
subject: E-9 v1.22 + BC-1.05.035 + BC-1.05.036 (diff-only line-by-line + intra-document semantic-sibling sweep angle per TD-VSDD-057)
verdict: SUBSTANTIVE
severity: 2H / 3M / 2L
adr_013_clock: RESET 0_of_3
angle: diff-only line-by-line + intra-document semantic-sibling sweep (NEW) — reads each line introduced or modified in v1.22 fix burst; then sweeps each affected BC's sibling sections (§Related BCs, §Edge Cases, §Canonical Test Vectors, §Postconditions, §Description) for prior wording that contradicts the v1.22 corrections
date: 2026-05-05
reviewer: adversary (fresh context)
---

# Adversarial Review — E-9 v1.7 Amendment Pass 22

## Angle: diff-only line-by-line + intra-document semantic-sibling sweep (TD-VSDD-057 rotation)

Reads each line changed in the v1.22 fix burst (D-264) against its authoritative
source. Then sweeps the same BCs' sibling sections for prior wording that contradicts
the corrections applied in v1.22. This angle has not been applied in passes 1-21.

Specifically:
- v1.22 Fix 1 corrected BC-1.05.036 §Postcondition 5 to state TIMEOUT/OUTPUT_TOO_LARGE
  return error codes WITHOUT emitting any event. This review checks whether all other
  sections of BC-1.05.036 are consistent with that corrected Postcondition 5.
- v1.22 Fix 3 added ADR-015 awareness clause to BC-1.05.035 §Description. This review
  checks whether BC-1.05.035 §Postconditions fully reflect that awareness.

---

## Verdict: SUBSTANTIVE (2H / 3M / 2L)

ADR-013 clock RESET to 0_of_3.

---

## HIGH Findings

### H-P22-001 — BC-1.05.036 §Related BCs (lines 61-62) + §EC-004 (line 86): sibling sections contradict v1.22 Postcondition 5 correction

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md`
**Lines:** 61, 62, 86
**Severity:** HIGH — intra-document contradiction; implementer reading §Related BCs or §Edge Cases is misled about timeout/output-too-large error paths

**Finding:**

v1.22 Fix 1 (D-264) correctly updated BC-1.05.036 §Postcondition 5 (line 52) to read:
> "TIMEOUT (-2) and OUTPUT_TOO_LARGE (-3) paths return error codes WITHOUT emitting any event."

But the SAME BC has three sibling locations that still carry pre-correction wording implying error-path events are emitted:

**Line 61 (§Related BCs, BC-1.05.032 row):**
> "BC-1.05.032 — timeout enforcement (sibling: timeout path **emits a different event**; this event is NOT emitted on timeout)"

The phrase "emits a different event" contradicts Postcondition 5's corrected statement that the timeout path emits NO event. An implementer reading §Related BCs receives the false impression that BC-1.05.032 covers a "different event" on the timeout path, when Postcondition 5 now explicitly states there is no event on the timeout path in v1.

**Line 62 (§Related BCs, BC-1.05.005 row):**
> "BC-1.05.005 — OUTPUT_TOO_LARGE path (sibling: output-too-large path **emits a different event**; this event is NOT emitted)"

Same class of contradiction: implies BC-1.05.005 covers an "output-too-large event" when Postcondition 5 states no event is emitted on the output-too-large path.

**Line 86 (§Edge Cases, EC-004):**
> "Subprocess times out | **Timeout error event emitted**; `host.exec_subprocess.completed` NOT emitted"

This EC-004 Expected Behavior cell directly states "Timeout error event emitted" — a direct contradiction of Postcondition 5 which states TIMEOUT path emits NO event.

**Root cause:** v1.22 burst (D-264) applied Fix 1 to §Postcondition 5 but did NOT sweep §Related BCs and §Edge Cases for prior wording contradicting the correction. TD-VSDD-075 sub-rule 1 (source-code-verification) was applied; but no intra-document sibling sweep was required by any existing TD. This is the gap that motivates TD-VSDD-076.

**Required fix:**
- Line 61: replace "emits a different event" with reality — timeout path returns `Err(TIMEOUT -2)` and emits NO event in v1 per Postcondition 5; future error-path emit is out-of-scope.
- Line 62: replace "emits a different event" with reality — output-too-large path returns `Err(OUTPUT_TOO_LARGE -3)` and emits NO event in v1 per Postcondition 5.
- Line 86: replace "Timeout error event emitted" with "Returns `Err(TIMEOUT -2)`; NO event emitted in v1 (per Postcondition 5; future error-path emit is out-of-scope)".

---

### H-P22-002 — BC-1.05.035 §Postcondition 4 (line 48): uses unqualified interim event name despite §Description INTERIM clause

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md`
**Line:** 48
**Severity:** HIGH — the very ADR-015 awareness clause added in v1.22 Fix 3 says `internal.capability_denied` MUST be renamed; Postcondition 4 still uses the bare interim name without the INTERIM qualifier

**Finding:**

v1.22 Fix 3 (D-264) added to BC-1.05.035 §Description:
> "This BC's denial-path postcondition references the existing `internal.capability_denied` event name. Per ADR-015 D-15.2 reverse-DNS naming requirement ... this event MUST be renamed to `vsdd.capability.denied.exec_subprocess.v1` ... The current name is INTERIM."

But §Postcondition 4 (line 48) still reads:
> "If the canonicalized path contains `..` components after resolution (symlink-based escape attempt), returns `codes::INVALID_ARGUMENT` (-4) and the existing `internal.capability_denied` event is emitted."

No INTERIM qualifier. No forward reference to the §Description ADR-015 awareness clause. No canonical target name cited. An implementer reading only §Postconditions (the normative section) receives an unqualified instruction to emit `internal.capability_denied`, contradicting the §Description requirement to rename it.

This is an intra-document contradiction: §Description says INTERIM/MUST-rename; §Postcondition 4 says unqualified current name.

**Required fix:** Append to Postcondition 4 the INTERIM qualifier with rename target, e.g.:
> " (event name `internal.capability_denied` is INTERIM — see §Description ADR-015 awareness clause; rename to `vsdd.capability.denied.exec_subprocess.v1` per ADR-015 D-15.2 registry line 329)"

---

## MEDIUM Findings

### M-P22-001 — BC-1.05.036 Postcondition 1 absoluteness contradicts Postcondition 5 error-path reality

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md`
**Line:** 48
**Severity:** MEDIUM — structural ambiguity in normative section; implementer may read Postcondition 1 in isolation without reaching Postcondition 5

**Finding:**

Postcondition 1 reads:
> "Exactly one `host.exec_subprocess.completed` event is emitted via `ctx.emit_internal`."

No conditional clause. No carve-out for error paths. The plain reading is "on any invocation that reaches §Postconditions, exactly one event is always emitted." But Postcondition 5 (added v1.22) establishes error paths (TIMEOUT, OUTPUT_TOO_LARGE) that DO NOT emit. A careful reader resolves the tension via ordering, but an implementer skimming for "when is this event emitted?" hits Postcondition 1's absolute wording before reaching Postcondition 5.

**Required fix:** Add a conditional clause to Postcondition 1 scoping it to the success path, e.g.:
> "On successful subprocess completion (subprocess exits before timeout AND within output cap; see Postcondition 5 for error-path reality), exactly one `host.exec_subprocess.completed` event is emitted via `ctx.emit_internal`."

---

### M-P22-002 — OQ-W16-001 acceptance (a) under-specified: registry-add alone does not bind canonical event name

**File:** `.factory/specs/open-questions.md`
**Line:** 29
**Severity:** MEDIUM — acceptance criterion (a) does not require that the canonical event name be specified; a registry amendment for `vsdd.host.*` could be merged without naming `host.exec_subprocess.completed.v1` and criterion (a) would technically pass

**Finding:**

Acceptance criterion (a) reads:
> "(a) ADR-015 D-15.2 registry amended to include `vsdd.host.* | <category>` BEFORE E-10 Wave 1 host-emit-fix story merges, OR"

This criterion only requires the prefix-to-category registry line to exist. It does NOT require that the specific canonical event name `vsdd.host.exec_subprocess.completed.v1` be formally confirmed in the same amendment. A story-writer could satisfy (a) by adding `vsdd.host.* | lifecycle` to the registry without specifying the full event name — leaving a downstream ambiguity about whether the event is `.completed.v1` or `.done.v1` or any other suffix.

**Required fix:** Tighten acceptance (a) to AND-link the registry-prefix addition with the canonical event name:
> "(a) ADR-015 D-15.2 registry amended to include `vsdd.host.* | <category>` AND the canonical event.name for the host-emit-fix is `vsdd.host.exec_subprocess.completed.v1` (mapped to that prefix's category) BEFORE E-10 Wave 1 host-emit-fix story merges, OR"

---

### M-P22-003 — BC-1.05.035 §Postconditions lack an explicit precedence ladder for multi-condition validation

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md`
**Lines:** 45-48 (§Postconditions)
**Severity:** MEDIUM — implementer working from Postconditions alone cannot determine which error code fires when multiple conditions could apply simultaneously

**Finding:**

BC-1.05.035 §Postconditions define 4 conditions:
- Postcondition 1: string-level validation passes AND canonicalize succeeds AND no `..` segments → allow-list check proceeds.
- Postcondition 2: NUL byte or basic string validation fails → `INVALID_ARGUMENT` (-4).
- Postcondition 3: `canonicalize()` fails → `CAPABILITY_DENIED` (-1).
- Postcondition 4: canonicalized path contains `..` → `INVALID_ARGUMENT` (-4) + `internal.capability_denied` emitted.

There is no stated precedence ladder. An implementer asks: if `cmd` contains both a NUL byte AND the path also fails canonicalize, which error code fires? The postconditions are stated as independent conditions, not as an ordered decision tree. The implementation presumably checks them in a specific order (as evidenced by `exec_subprocess.rs:230`), but that order is not normative in the BC.

**Required fix:** Add a precedence-ladder note to §Postconditions (or as an explicit ordering invariant):
- (1) NUL byte in `cmd` → `INVALID_ARGUMENT` (-4) — checked first via `read_wasm_string` path.
- (2) `Path::new(cmd).canonicalize()` fails → `CAPABILITY_DENIED` (-1).
- (3) Canonicalized path contains `..` segments → `INVALID_ARGUMENT` (-4).
- (4) Not in `binary_allow` list → `CAPABILITY_DENIED` (-1).
Per `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` entry point.

---

## LOW Findings

### L-P22-001 — BC-1.05.036 v1.22 H3 changelog: forward-looking conditional prose in permanent spec body

**Severity:** LOW
**File:** E-9 epic v1.22 H3 section, and BC-1.05.036 Postcondition 5 body
**Finding:** Postcondition 5 contains "Adding TIMEOUT/OUTPUT_TOO_LARGE error-path events is OUT OF SCOPE for this BC and may be tracked in a future OQ if needed." The phrase "may be tracked in a future OQ" is forward-looking conditional language in a normative BC body. Conditionals in postconditions weaken normative force. The OQ either exists or it does not; if it does not exist, the sentence is vacuous. If the intent is to defer observability for error paths, a stronger formulation would simply state: "No event is emitted on TIMEOUT or OUTPUT_TOO_LARGE error paths in v1. Tracking of future error-path observability is deferred."

**Disposition:** SKIP per S-7.03/D-231 SHIP-AS-IS pattern. The v1.22 H3 conditional language is defensible; the normative claim (no event on error paths) is clear. Cosmetic only.

---

### L-P22-002 — BC-1.05.036 Postcondition 5 and §Canonical Test Vectors line 97 both reserve "Timeout event emitted" language; DRY duplication

**Severity:** LOW
**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md`
**Line:** 97
**Finding:** §Canonical Test Vectors line 97 reads: "Subprocess timeout | Timeout event emitted; `host.exec_subprocess.completed` NOT emitted | error". The "Timeout event emitted" cell is a peer-duplication of the pre-v1.22 wording that Postcondition 5 now overrides. The test vector should be consistent with the corrected Postcondition 5 (NO event emitted). However, fixing this is folded into H-P22-001 scope (the sibling sweep should also catch §Canonical Test Vectors as a sibling section).

**Note:** This finding is resolved as part of H-P22-001 if the fix burst sweeps §Canonical Test Vectors as well. If H-P22-001 fix is scoped narrowly to §Related BCs + §EC-004, the §Canonical Test Vectors inconsistency remains. Recommend the fix burst scope include §Canonical Test Vectors line 97 update.

**Disposition:** Absorbed into H-P22-001 fix scope. Not a standalone action item.

---

## ADR-013 Convergence Clock

**Clock: RESET to 0_of_3**

Pass-22 is SUBSTANTIVE (2H/3M). 2 HIGH findings require correction before pass-23. Three consecutive NITPICK_ONLY passes (23/24/25) needed from the v1.23 surface to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

---

## TD-VSDD-076 Codification Trigger

Pass-22 H-P22-001 is the canonical evidence for a new technical debt rule:

**TD-VSDD-076 — Intra-document semantic-sibling sweep (extension to TD-VSDD-075):**

When a fix burst corrects a Postcondition (or any normative claim) within a BC, the same burst MUST grep the SAME BC for sibling sections — specifically: §Related BCs, §Edge Cases, §Canonical Test Vectors, §Postconditions, §Description — for prior wording that contradicts the correction. Each sibling section that contradicts must be updated in the same burst.

TD-VSDD-075 (codified at v1.22) covered inter-document line-citation refresh (sub-rule 2) and source-code-verification (sub-rule 1). It did NOT cover intra-document semantic siblings within the same BC. TD-VSDD-076 closes this gap.

---

## Summary

| Finding | Severity | File | Status |
|---------|----------|------|--------|
| H-P22-001 | HIGH | BC-1.05.036 §Related BCs lines 61-62 + §EC-004 line 86 | Requires fix |
| H-P22-002 | HIGH | BC-1.05.035 §Postcondition 4 line 48 | Requires fix |
| M-P22-001 | MEDIUM | BC-1.05.036 Postcondition 1 line 48 | Requires fix |
| M-P22-002 | MEDIUM | open-questions.md OQ-W16-001 acceptance (a) | Requires fix |
| M-P22-003 | MEDIUM | BC-1.05.035 §Postconditions (no precedence ladder) | Requires fix |
| L-P22-001 | LOW | BC-1.05.036 Postcondition 5 conditional prose | SKIPPED |
| L-P22-002 | LOW | BC-1.05.036 §Canonical Test Vectors line 97 | Absorbed into H-P22-001 |

**Trajectory:** pass-19 NITPICK (0/0/2, clock 0→1) → pass-20 SUB (0/2/2, clock 1→0) → pass-21 SUB (2/3/2, clock 0) → pass-22 SUB (2/3/2, clock 0). ADR-013 0_of_3.
