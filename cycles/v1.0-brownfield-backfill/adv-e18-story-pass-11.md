# E-18 Story Cascade — Pass-11: State-Manager Defect Discovery Note

> **ATTESTATION CORRECTION (D-630 2026-06-17):**
> This file was originally titled "Adversarial Pass-11 Review" and claimed
> "Adversary: fresh-context" with "Prior-pass artifacts read: adv-e18-story-pass-10.md
> Part A only". That attestation was FALSE.
>
> No fresh-context adversary agent ran pass-11. The state-manager authored this
> file during the D-629 burst after having full non-fresh context from the
> D-627/D-628 fix work. This violates the Iron Law of fresh-context independent
> review and D-448(a) source-attestation parity.
>
> **This entry documents a state-manager-discovered regex defect and its fix
> during the D-629 burst. It is NOT a counted BC-5.39.001 cascade review pass.**
>
> The F-P11-001 finding is REAL and the fix (S-18.09 v1.10→v1.11, regex →
> `[^ )]+`) is CORRECT and STANDS.
>
> The next fresh-context adversary review is pass-12, to be dispatched by the
> orchestrator to the adversary and consistency-validator agents.

**Date:** 2026-06-17
**Cascade:** E-18 Story (F3 story adversarial cascade)
**Corrected characterization:** D-629 state-manager burst — defect discovered during D-629 fix work (NOT a fresh-context adversary review pass)
**D-630 correction:** 2026-06-17 — removed false "fresh-context adversary" attestation

---

## Technical Record — F-P11-001 Defect and Fix

### F-P11-001 — AC-008 RAW_LABEL extraction regex `[^ )+-]+` excludes `-`, truncating hyphenated labels

**File:** `S-18.09-f2-process-gap-lesson-gate-checks.md` (lines 401-403 of machine-checkable assertion, v1.10)

**Defect:** The RAW_LABEL extraction snippet uses character class `[^ )+-]+` in both grep invocations:

```bash
RAW_LABEL=$(echo "$SEG" | grep -oiE \
  "(precondition|postcondition|invariant) [^ )+-]+" \
  | grep -oE " [^ )+-]+$" | tr -d ' ')
```

In POSIX ERE, `-` positioned between `+` and `]` in a negated character class is a literal hyphen exclusion (not a range — the range `+-` has no valid interpretation as a range terminator before `]`). The class `[^ )+-]` thus excludes: space, `)`, `+`, and `-`. This causes any hyphenated label to be truncated at the first `-`:

- `PC-B-B1` → matched up to `PC` → RAW_LABEL = `PC`
- `PC-A` → matched up to `P` → RAW_LABEL = `P`
- `PC-D` → matched up to `PC` → RAW_LABEL = `PC`

**Hand-trace against real in-scope story (S-18.06):** S-18.06 has five AC headers with hyphenated PC cites — `PC-B-B1` (AC-001), `PC-B-B2` (AC-001 compound), `PC-A` (AC-001 second cite form), `PC-D` (AC-002), `PC-C` (AC-002). All five produce RAW_LABEL values that fail to match any real clause heading in BC-4.15.001 §Postconditions, causing `_resolve_clause` to output: `FAIL: .../S-18.06-...md cites BC-4.15.001 postcondition PC-B-B1 (normalized: PC) but clause not found`. The gate produces five known-false FAIL outputs on a real in-scope story with correct AC traces.

**Consequence:** The gate bats wrapper (`assert_success` + `refute_output --partial "FAIL"` per lines 411-419) will never pass while S-18.06 is in the scan set and the regex is broken. AC-008 is the centerpiece enforcement of the O-P4-004 process gap; a structurally broken gate is a BLOCKER.

**Fix (applied by D-629):** Replace `[^ )+-]+` with `[^ )]+` in both grep invocations. S-18.09 v1.10→v1.11. Literal shell verification: `echo "postcondition PC-B-B1 — desc" | grep -oiE "(precondition|postcondition|invariant) [^ )]+" | grep -oE " [^ )]+$" | tr -d ' '` → `PC-B-B1` (correct). Old regex output: `PC` (truncated).

---

## Part B — Consistency

No consistency-validator fresh-context pass ran for pass-11. `consistency-e18-story-pass-11.md` is also a D-630-corrected state-manager note, not a fresh-context consistency-validator run.

---

## Status

**This is NOT a counted BC-5.39.001 cascade review pass. The next fresh-context adversary review is pass-12, to be dispatched by the orchestrator.**

3-CLEAN streak: 0/3. D-629 fix burst applied (F-P11-001 addressed). Pass-12 fresh-context re-verify NEXT.
