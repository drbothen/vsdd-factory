---
document_type: adversarial-review
level: ops
pass: 21
subject: E-9 v1.21 + BC-1.05.035 + BC-1.05.036 (BC-only deep-dive angle per TD-VSDD-057)
verdict: SUBSTANTIVE
severity: 2H / 3M / 2L
adr_013_clock: RESET 0_of_3
angle: BC-only deep-dive (NEW) — reads BC-1.05.035 and BC-1.05.036 as a standalone implementer with no E-9 epic context; then cross-validates BC content against source code constants and ADR-015 D-15.2 registry
date: 2026-05-05
reviewer: adversary (fresh context)
---

# Adversarial Review — E-9 v1.7 Amendment Pass 21

## Angle: BC-only deep-dive (TD-VSDD-057 rotation)

Reads BC-1.05.035 and BC-1.05.036 as a standalone implementer with zero E-9 epic context. Then cross-validates cited source-code constants, ADR-015 registry categories, and field semantics against authoritative sources. This angle has not been applied in passes 1-20, all of which read from the E-9 epic surface downward.

---

## Verdict: SUBSTANTIVE (2H / 3M / 2L)

ADR-013 clock RESET to 0_of_3.

---

## HIGH Findings

### H-P21-001 — BC-1.05.036:52 + E-9 v1.21 changelog line 1183: fabricated error codes -7/-8 for TIMEOUT/OUTPUT_TOO_LARGE

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md` line 52; E-9 epic v1.21 H3 line 1183

**Finding:** BC-1.05.036 §Postconditions item 5 (added by v1.21 L-P20-002 fix) states:

> "TIMEOUT (-7) and OUTPUT_TOO_LARGE (-8) paths return error codes WITHOUT emitting any event."

E-9 epic v1.21 H3 section (D-263 entry) repeats the same values: "TIMEOUT (-7) and OUTPUT_TOO_LARGE (-8) paths return error codes WITHOUT emitting any event."

**Actual values per source code** (`crates/factory-dispatcher/src/host/mod.rs` lines 181-182):

```rust
pub const TIMEOUT: i32 = -2;
pub const OUTPUT_TOO_LARGE: i32 = -3;
```

The v1.21 fix burst (D-263 L-P20-002) invented `-7` and `-8` without reading the source. The actual ABI codes are `-2` and `-3`. This is a regression — the fix burst introducing BC-1.05.036 §Postconditions item 5 introduced factually wrong data.

**Blast radius:** BC-1.05.036 §Postconditions item 5 (live spec content consumed by S-9.07 implementer). The v1.21 H3 changelog entry also repeats the wrong values but is subject to POLICY 1 (prior-version block immutability); leave as historical record, document correction in v1.22 H3.

**Severity:** HIGH — S-9.07 implementer reads BC-1.05.036 §Postconditions 5 to understand error semantics; wrong error codes would cause code written against the spec to misidentify error conditions.

**Fix:** BC-1.05.036 line 52: replace `-7` with `-2` and `-8` with `-3`. Add v1.22 H3 correction note in E-9 epic documenting the historical error.

---

### H-P21-002 — open-questions.md:21: citation off-by-one (gap-analysis line 325 vs actual line 326)

**File:** `.factory/specs/open-questions.md` line 21

**Finding:** OQ-W16-001 `Source:` field reads:

> "gap-analysis line 325 ("Resolution tracked in **OQ-W16-001**")"

**Actual position via grep:** `grep -n 'Resolution tracked in \*\*OQ-W16-001\*\*' .factory/architecture/gap-analysis-w16-subprocess.md` → line **326**.

The v1.20 fix burst (D-261) added `last_amended: 2026-05-05` to gap-analysis-w16-subprocess.md frontmatter at line 8. This inserted a new frontmatter field, shifting every subsequent line by +1. The v1.16 D-256 fix set the citation to 325 (correct at that time). The v1.20 D-261 burst did not refresh the now-stale dependent citation.

This is the THIRD recurrence of the line-citation off-by-one defect class (after L-P9-001 and M-P13-001). S-7.02 codification threshold (3+) met — warrants TD codification.

**Severity:** HIGH — cited text does not match the stated line number; implementer following the bidirectional anchor finds the wrong text.

**Fix:** Replace `gap-analysis line 325` with `gap-analysis line 326`.

---

## MEDIUM Findings

### M-P21-001 — BC-1.05.035: missing ADR-015 awareness clause (TD-VSDD-074 asymmetric application)

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md`

**Finding:** BC-1.05.036 received an ADR-015 awareness clause in v1.21 (M-P20-002), but BC-1.05.035 — authored in the same burst, referencing the same `internal.capability_denied` event — did not. BC-1.05.035 §Postconditions 4 reads:

> "If the canonicalized path contains `..` components after resolution (symlink-based escape attempt), returns `codes::INVALID_ARGUMENT` (-4) and the existing `internal.capability_denied` event is emitted."

The `internal.capability_denied` event name is INTERIM per the E-9 v1.7 amendment (must be renamed to `vsdd.capability.denied.exec_subprocess.v1` per ADR-015 D-15.2). TD-VSDD-074 requires that when an amendment burst changes a contract a BC implements, the same burst MUST add an awareness clause. The D-263 burst added it to BC-1.05.036 but not BC-1.05.035 — asymmetric application.

**Severity:** MEDIUM — implementer of BC-1.05.035 denial path will emit the wrong event name without the awareness clause.

**Fix:** Add ADR-015 awareness clause to BC-1.05.035 §Description, mirroring BC-1.05.036's form.

---

### M-P21-002 — BC-1.05.036:34: fabricated "host" ADR-015 category

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md` line 34

**Finding:** BC-1.05.036 §Description ADR-015 awareness clause option (a) reads:

> "(host category mapping per OQ-W16-001 acceptance criterion (a))"

ADR-015 D-15.2 registry has exactly 5 categories: `lifecycle | domain | audit | error | unknown`. There is NO `host` category. This is a fabricated category name — the same class of invented-value defect as H-P21-001.

**Severity:** MEDIUM — S-9.07 implementer reading the ADR-015 awareness clause would search ADR-015 D-15.2 for a "host" category and find nothing.

**Fix:** Replace "host category mapping per OQ-W16-001 acceptance criterion (a)" with "category to be assigned per OQ-W16-001 acceptance criterion (a) — ADR-015 D-15.2 registry has 5 categories: lifecycle, domain, audit, error, unknown".

---

### M-P21-003 — BC-1.05.036 §Postcondition 2 + EC-006: `truncated: bool` is dead weight in v1

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md` lines 49, 88

**Finding:** BC-1.05.036 §Postcondition 2 declares 8-field payload including `truncated: bool`. §Postconditions item 5 (as corrected by H-P21-001 fix) states that OUTPUT_TOO_LARGE returns `Err(OUTPUT_TOO_LARGE -3)` — the host returns an error code without emitting the success-path event. This means the success-path event (which contains `truncated: bool`) can only be emitted on the Ok path — i.e., output was NOT truncated. Therefore `truncated: bool` is always `false` in v1.

gap-analysis-w16-subprocess.md Section 5 "fundamentally insufficient" Gap 1 notes the truncation architecture issue. The `truncated: bool` field as declared implies it could ever be `true` — which is misleading.

**Severity:** MEDIUM — implementer may add code to set `truncated = true` under certain conditions, wasting implementation effort on a semantically impossible state.

**Fix (Option b):** Retain field but document as reserved. Add inline note to §Postcondition 2 field declaration and EC-006 row: `[reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1]`.

---

## LOW Findings (DEFERRED)

### L-P21-001 — Interim-vs-canonical event name whiplash in BC-1.05.036

**Finding:** BC-1.05.036 §Description states the event name `host.exec_subprocess.completed` is INTERIM, then §Postconditions item 1 and §Canonical Test Vectors both use `host.exec_subprocess.completed` without the INTERIM qualifier. Implementer flipping between §Description and §Postconditions experiences name whiplash without seeing the qualifier.

**Severity:** LOW — cosmetic readability. The awareness clause already appears in §Description; the postcondition usage is correct (interim name is the current implementation target). The S-9.07 implementer reads §Description first.

**Disposition:** DEFERRED. The awareness clause + canonical use both serve purposes. Cosmetic cross-section redundancy is not worth a fix burst.

---

### L-P21-002 — Rust line citations in BC-1.05.036 (exec_subprocess.rs:230, :270)

**Finding:** BC-1.05.035 cites `exec_subprocess.rs:230` and BC-1.05.036 cites `exec_subprocess.rs:270` as implementation anchors. These line-number anchors were set during gap analysis (2026-05-03); any refactoring of exec_subprocess.rs in the intervening period would make them stale.

**Severity:** LOW — architecture anchors are informational; the actual implementation locates code by grep. Same class as L-P19-002.

**Disposition:** DEFERRED. Per S-7.03/D-231 SHIP-AS-IS pattern for Rust line citations. Will be refreshed by the implementing story.

---

## Convergence Trajectory

| Pass | Verdict | H | M | L | Clock |
|------|---------|---|---|---|-------|
| 19 | NITPICK_ONLY | 0 | 0 | 2 | 0→1 |
| 20 | SUBSTANTIVE | 0 | 2 | 2 | 1→0 RESET |
| 21 | SUBSTANTIVE | 2 | 3 | 2 | 0→0 RESET |

ADR-013 clock: **0_of_3**. Three consecutive NITPICK_ONLY passes (22/23/24) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.

---

## Process Notes

- **TD-VSDD-075 triggered:** H-P21-001 (fabricated error codes) and H-P21-002 (citation off-by-one due to frontmatter-line-shift) both result from fix bursts that did not verify source-code values or refresh dependent citations. Codification warranted:
  - **Source-code-verification sub-rule:** Fix bursts that cite source-code constants MUST read the actual source before commit and quote the exact line in the commit message body.
  - **Dependent-citation-propagation sub-rule:** When a fix burst adds `last_amended:` (or any frontmatter field that shifts line numbers), the same burst MUST grep all in-scope files for inbound citations of form `<filename> line N`. For each match, re-grep the cited file for the quoted text and verify the line number still resolves.
- H-P21-001 is the **regression** introduced by the v1.21 fix burst (D-263). The burst that fixed L-P20-002 invented the wrong error codes. New discipline (TD-VSDD-075) will prevent recurrence.
- H-P21-002 is the **third recurrence** of the line-citation off-by-one class. S-7.02 threshold met; TD-VSDD-075 codified.
