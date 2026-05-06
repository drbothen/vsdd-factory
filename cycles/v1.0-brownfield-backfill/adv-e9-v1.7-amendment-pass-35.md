---
document_type: adversarial-review
pass: 35
epic: E-9
amendment_surface: v1.7-and-later
version_reviewed: "1.31"
verdict: SUBSTANTIVE
finding_counts: {HIGH: 1, MED: 3, LOW: 2}
angle: mechanism-coherence sibling sweep (NEW per TD-VSDD-057)
adr_013_clock: 0_of_3
reviewer: adversary
timestamp: 2026-05-05T00:00:00Z
sealed_by: D-278
---

# Adversarial Review — E-9 v1.7 Amendment Surface — Pass 35

**Version reviewed:** E-9 v1.31 (D-277 mechanism-fix burst)
**Verdict:** SUBSTANTIVE — 1 HIGH / 3 MED / 2 LOW
**Angle:** Mechanism-coherence sibling sweep — after v1.31 corrected the NUL-byte mechanism (TD-VSDD-081), ALL mechanisms within the same BC using the same std-lib function (`Path::canonicalize()`) were examined for correctness of their specific predicates. Also applied bidirectional-sibling-disclosure check per extension of TD-VSDD-076.
**ADR-013 clock:** RESET to 0_of_3 (SUBSTANTIVE verdict)

---

## Findings

### HIGH-P35-001: BC-1.05.035 EC-002 + Postcondition 4 + Ladder step (3) — `..` components mechanism is WRONG for symlink escape detection

**Severity:** HIGH
**Location:** BC-1.05.035 — EC-002 (line 83), Postcondition 4 (line ~50), Precedence Ladder step (3) (line ~52)

**Finding:** v1.31 correctly fixed the NUL-byte mechanism (TD-VSDD-081): `Path::canonicalize()` resolves NUL-containing paths to `EINVAL` → ladder step 2 → `CAPABILITY_DENIED`. But the SAME BC's symlink-escape mechanism (EC-002 + Postcondition 4 + Ladder step 3) uses an INCORRECT predicate: it claims "canonicalized path contains `..` components" triggers `INVALID_ARGUMENT`.

**Why this is wrong:** `Path::canonicalize()` is defined to resolve ALL `..` segments away (this is its primary purpose per Rust std). The canonical form is an absolute path with NO `..` components. A canonical path containing `..` is only possible when intermediate path components don't exist (OS-dependent), which is a marginal edge case already covered by ladder step 2 (canonicalize fails → CAPABILITY_DENIED). The actual symlink-escape detection mechanism is `canonical_path.starts_with(project_root)` prefix check, NOT a `..` scan. The `..` scan predicate is false — it describes a mechanism that rarely fires, while the actual escape detection (prefix check) is not described at all.

**Evidence:** Rust std `Path::canonicalize` documentation: "Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved." The resolution means `..` segments are resolved away. The result is absolute with no `..`.

**Impact:** Any implementer reading this BC would implement a `..` scan instead of a prefix check, creating an incomplete/incorrect security guard that misses all symlink escapes where the resolved path doesn't contain `..` (which is virtually all of them).

**Scope of fix:** EC-002 text, Postcondition 4, Precedence Ladder step (3) — all three must be corrected from `..` scan to prefix-check mechanism.

---

### MED-P35-001: BC-1.05.035 Postcondition 3 — "existing exec_subprocess error semantics preserved" is false; this is a BEHAVIOR CHANGE

**Severity:** MED
**Location:** BC-1.05.035 Postcondition 3 (line ~49)

**Finding:** Postcondition 3 states "existing exec_subprocess error semantics preserved" for the canonicalize() failure path returning `CAPABILITY_DENIED (-1)`. This is factually wrong. Currently (pre-BC implementation), missing-binary `cmd` paths fail at `command.spawn()` returning `INTERNAL_ERROR (-99)` per `exec_subprocess.rs:252`:

```
let mut child = command.spawn().map_err(|_| codes::INTERNAL_ERROR)?;
```

Adding a `canonicalize()` check BEFORE `command.spawn()` means that a missing-binary `cmd` will now fail at `canonicalize()` (ENOENT → `CAPABILITY_DENIED -1`) instead of at `command.spawn()` (IO error → `INTERNAL_ERROR -99`). The error code changes from `-99` to `-1` for the missing-binary case.

**Impact:** Downstream tests asserting `INTERNAL_ERROR` for missing-binary `cmd` will break. Implementers relying on Postcondition 3's "preserved semantics" claim will not write migration notes or update tests.

**Fix:** Postcondition 3 must acknowledge this is a BEHAVIOR CHANGE and document the -99 → -1 transition.

---

### MED-P35-002: BC-1.05.035 §Related BCs row for BC-1.05.036 — missing reverse-direction NOTE

**Severity:** MED
**Location:** BC-1.05.035 §Related BCs (line ~60-65), BC-1.05.036 row

**Finding:** v1.31 burst (D-277 MED-P34-002) correctly added a forward-direction NOTE to BC-1.05.036 §Related BCs: "BC-1.05.035 introduces a novel INVALID_ARGUMENT (-4) + internal.capability_denied pairing — test-writers MUST include this 5th path." But the reverse direction was not added. BC-1.05.035's §Related BCs row for BC-1.05.036 has no disclosure about BC-1.05.036's structural novelty: BC-1.05.036 introduces the FIRST non-denial event emitted via `ctx.emit_internal` (`host.exec_subprocess.completed`) — a structurally novel event class beyond the 4 existing denial events plus BC-1.05.035's 5th symlink-pairing.

**Consequence:** Test-writers reading BC-1.05.035 outbound will not know to include success-path event class coverage. Only readers starting from BC-1.05.036 get the disclosure. Bidirectional asymmetry — violates the cross-referencing discipline codified by the D-277 burst.

---

### MED-P35-003: BC-1.05.036 Postcondition 4 — ADR-015 line-number citations are fragile

**Severity:** MED
**Location:** BC-1.05.036 Postcondition 4 (line 51)

**Finding:** Postcondition 4 references ADR-015 using line-number anchors: "(multi-sink stanza model removed per ADR-015 line 154; Router/SinkRegistry retired per ADR-015 line 130)". Line numbers in ADR-015 are fragile: any amendment that adds or removes lines above line 130 will shift these anchors silently. The cited content is short and grep-able. Stable quoted-phrase anchors are available and should be used instead.

**Fix:** Replace line-number anchors with quoted-phrase anchors from ADR-015 D-15.1 §"Decision".

---

### LOW-P35-001: BC-1.05.035 §Precedence Ladder — "step (3)" grammar inconsistency with sibling steps

**Severity:** LOW
**Disposition:** SKIP per S-7.03 SHIP-AS-IS — cosmetic grammar; does not affect implementer behavior.

---

### LOW-P35-002: BC-1.05.035 EC-007 — conflates 4 distinct INTERNAL_ERROR sources

**Severity:** LOW
**Disposition:** SKIP per S-7.03 SHIP-AS-IS — EC-007 is already scoped at the correct granularity for this BC; detailed enumeration belongs in BC-1.05.036 Postcondition 5.

---

## Disposition Summary

| Finding | Severity | Disposition |
|---------|----------|-------------|
| HIGH-P35-001 | HIGH | CLOSE — EC-002 + Postcondition 4 + Ladder step (3) prefix-check correction |
| MED-P35-001 | MED | CLOSE — Postcondition 3 BEHAVIOR CHANGE disclosure |
| MED-P35-002 | MED | CLOSE — BC-1.05.035 §Related BCs BC-1.05.036 row reverse-direction NOTE |
| MED-P35-003 | MED | CLOSE — BC-1.05.036 Postcondition 4 quoted-phrase anchors |
| LOW-P35-001 | LOW | SKIP — cosmetic grammar per S-7.03 |
| LOW-P35-002 | LOW | SKIP — scope correct per S-7.03 |

**Fix burst:** D-278 combined seal-and-fix burst applies all 4 substantive fixes (HIGH-P35-001, MED-P35-001, MED-P35-002, MED-P35-003) and bumps E-9 v1.31 → v1.32.
