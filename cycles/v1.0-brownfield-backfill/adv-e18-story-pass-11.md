# E-18 Story Cascade — Adversarial Pass-11 Review

**Date:** 2026-06-17
**Cascade:** E-18 Story (F3 story adversarial cascade)
**Pass:** 11
**Adversary:** fresh-context
**Prior-pass artifacts read:** adv-e18-story-pass-10.md Part A only

---

## Part A — Findings

**Verdict: NOT-CLEAN (1 BLOCKER, 0 MAJOR, 0 load-bearing MEDIUM, 0 LOW, 1 observation)**

### F-P11-001 (BLOCKER — AC-008 RAW_LABEL extraction regex `[^ )+-]+` excludes `-`, truncating hyphenated labels)

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

**Consequence:** The gate bats wrapper (`assert_success` + `refute_output --partial "FAIL"` per lines 411-419) will never pass while S-18.06 is in the scan set and the regex is broken. AC-008 is the centerpiece enforcement of the O-P4-004 process gap; a structurally broken gate is a BLOCKER under the adversarial rubric ("AC shell snippet that has a logical error … produces known-false outputs").

**Fix:** Replace `[^ )+-]+` with `[^ )]+` in both grep invocations. The `+` compound-split delimiter is absent from label tokens (segments are already split on `+` before RAW_LABEL extraction), so omitting `+` from the exclusion class does not cause `+` to be included in labels.

### O-P11-1 (Observation — STORY-INDEX v4.12 while other-index D-625 rows cite STORY-INDEX v4.10)

STORY-INDEX is at v4.12 while VP-INDEX, BC-INDEX, and ARCH-INDEX changelog rows for D-625 reference STORY-INDEX as v4.10. This is consistent with STORY-INDEX advancing two versions (v4.11, v4.12) in bursts after D-625 without companion cross-index re-citations. Per D-448(b), STORY-INDEX is exempt from structured-changelog parity, so no structural gap exists. Informational only; no fix required.

**Index parity check (O-P10-1 class — mechanical):**
- VP-INDEX: frontmatter v2.37 == changelog top v2.37 — PASS
- BC-INDEX: frontmatter v3.07 == changelog top v3.07 — PASS
- ARCH-INDEX: frontmatter v2.54 == changelog top v2.54 — PASS
- STORY-INDEX: v4.12 (exempt from structured array per D-448(b)) — N/A

**VP-091 changelog ordering check (F-P10-002 fix target):** v1.1 row appears above v1.0 row — PASS (descending order confirmed).

**AC-007 TOML hook block shape:** BC-4.14.001 and BC-4.15.001 both have `name = "..."` + `plugin = "hook-plugins/....wasm"` — PASS.

**Fence-strip logic (F-P10-003):** Both AC_SECTION assignments pipe through `awk '/^```/{fence=!fence;next}!fence'`. Worked-example cites in AC-008 bash fence correctly stripped. F-P10-003 fix verified intact — the RAW_LABEL bug is independent of fence-strip correctness.

**3-CLEAN streak:** Pass-11 NOT-CLEAN → streak RESET 0/3. D-629 fix burst applied (F-P11-001 addressed). Pass-12 re-verify NEXT.

---

## Part B — Consistency Check

Consistency-validator pass-11 returned **CONSISTENT**. All checks PASS (see consistency-e18-story-pass-11.md). No consistency findings.

---

## Part C — Post-D-629 Closure Note

D-629 FIX BURST (2026-06-17) addressed F-P11-001:

- **F-P11-001 CLOSED:** S-18.09 v1.11 — RAW_LABEL extraction regex fixed: `[^ )+-]+` → `[^ )]+` in both grep invocations. Literal shell verification: `echo "postcondition PC-B-B1 — desc" | grep -oiE "(precondition|postcondition|invariant) [^ )]+" | grep -oE " [^ )]+$" | tr -d ' '` → `PC-B-B1` (correct). Old regex output: `PC` (truncated). New regex output: `PC-B-B1` (full hyphenated label preserved).

**Pass-11 NOT-CLEAN → streak RESET 0/3; D-629 fix burst applied; pass-12 re-verify NEXT.**
