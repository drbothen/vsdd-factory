---
document_type: adversarial-review
pass: 36
subject: E-9 v1.7 Amendment Surface — BC-1.05.035 v1.32 diff-only
verdict: SUBSTANTIVE
findings: "2H/3M/1L"
angle: "Diff-only of v1.32 angle (NEW per TD-VSDD-057) — v1.32 introduced prefix-check mechanism; this pass verifies that mechanism against source-of-truth"
adr_013_clock_before: 0_of_3
adr_013_clock_after: RESET_0_of_3
sealed_by: D-279
timestamp: 2026-05-05
---

# Adversarial Review — E-9 v1.7 Amendment Surface — Pass 36

**Verdict:** SUBSTANTIVE 2H/3M/1L
**Angle:** Diff-only of v1.32 (NEW per TD-VSDD-057) — isolates mechanism introduced in D-278 and verifies against source-of-truth.
**ADR-013 clock:** 0_of_3 → RESET 0_of_3 (SUBSTANTIVE verdict)
**Sealed by:** D-279

---

## Findings

### HIGH-P36-001: BC-1.05.035 Postcondition 4 prefix-check mechanism is ANTI-CORRECT

**Severity:** HIGH
**Location:** BC-1.05.035.md §Postconditions (line 50) + §Precedence Ladder (line 52) + EC-002 (line 83) + Test Vector (line 94)

**Finding:** v1.32 Postcondition 4 states: after canonicalization, if the canonical path does NOT start with the trusted project-root prefix → `INVALID_ARGUMENT` (-4) + `symlink_traversal_escape`. The EC-003 test vector for `cmd = "/usr/bin/bash"` (canonical happy path) states "Proceeds to allow-list check".

Direct internal contradiction: `Path::canonicalize("/usr/bin/bash")` → `/usr/bin/bash`. `/usr/bin/bash` does NOT start with `$CLAUDE_PROJECT_DIR` (e.g., `/Users/jmagady/Dev/vsdd-factory`). Per Postcondition 4 + EC-002, this would trigger `symlink_traversal_escape` → `INVALID_ARGUMENT` (-4). But EC-003 + Test Vector for `/usr/bin/bash` says "proceeds to allow-list check" (happy path). The v1.32 mechanism universally rejects the canonical S-9.07 use case (`bash`, an absolute system path).

**Status:** CLOSED by D-279 Fix 1 (architectural reframe — drop prefix-check entirely).

---

### HIGH-P36-002: "Trusted project-root prefix" concept has NO architectural anchor

**Severity:** HIGH
**Location:** BC-1.05.035.md §Postconditions 1/4, §Precedence Ladder, EC-002

**Finding:** The v1.32 mechanism relies on "trusted project-root prefix" concept. Verification against all upstream documents:
- `host/mod.rs:49-76` — `HostContext` struct has `cwd: PathBuf` but NO `project_root` field. The concept has no data-structure anchor.
- gap-analysis-w16-subprocess.md Section 5 (cited authority) — proposes `if cmd.contains("../") { return CAPABILITY_DENIED; }` string-level `../` guard, NOT a prefix-check mechanism. The concept has no gap-analysis anchor.
- No HOST_ABI.md, ADR, or other architecture document defines "project-root prefix" or any equivalent.
- `read_file.rs:122-148` — uses `path_allow` LOOP (canonicalize each allow-list entry, check starts_with), not a single project-root prefix check. The concept has no sibling-implementation pattern.

The mechanism was unimplementable as written AND anti-correct per HIGH-P36-001.

**Status:** CLOSED by D-279 Fix 1 + Fix 2 + Fix 3 (architectural reframe + scour).

---

### MED-P36-001: Precedence ladder step (3) uses wrong error-code family

**Severity:** MED
**Location:** BC-1.05.035.md §Precedence Ladder (line 52) — step (3)

**Finding:** v1.32 Ladder step (3) fires `INVALID_ARGUMENT` (-4) for the symlink-traversal case. This is inconsistent with all 4 existing denial paths at exec_subprocess.rs:148/155/162/169, which uniformly fire `CAPABILITY_DENIED` (-1). The novel error-code pairing (`INVALID_ARGUMENT` + `capability_denied` event) creates a 5th denial path with a divergent error code that has no source-code, ADR, or design-doc anchor. It was justified by a coined rationale ("malformed path shape post-canonicalize") with no upstream definition.

**Status:** CLOSED by D-279 Fix 1 (drop novel pairing; symlink → normal allow-list miss → CAPABILITY_DENIED -1).

---

### MED-P36-002: Architecture Anchor cites exec_subprocess.rs:230 (wrong line)

**Severity:** MED
**Location:** BC-1.05.035.md §Architecture Anchors (line 67)

**Finding:** §Architecture Anchors row 1 cites `exec_subprocess.rs:230` as "canonicalize-before-check step added here". Line 230 is inside `execute_bounded()` which is called AFTER all 4 capability checks complete (after `binary_allowed()` at line 152, shell check at ~159, setuid check at ~166). The canonicalize insertion site is at line 152 (before `binary_allowed()`), not line 230.

**Status:** CLOSED by D-279 Fix 4.

---

### MED-P36-003: EC-001 outcome cell attributes CAPABILITY_DENIED to wrong ladder step

**Severity:** MED
**Location:** BC-1.05.035.md EC-001 (line 82)

**Finding:** EC-001 outcome cell says "caught by existing allow-list miss: basename 'passwd' not in `binary_allow` → emit_denial..." without clarifying the precedence ladder step attribution in v1.32. With v1.32's Postcondition 4, a literal `../etc/passwd` would first canonicalize (to `/etc/passwd` if it exists), then the prefix check would fire (step 3) returning `INVALID_ARGUMENT` (-4) — not the allow-list miss (step 4). The EC-001 outcome is thus wrong under v1.32's own model.

**Status:** CLOSED by D-279 Fix 5 (corrected to reflect new simplified ladder).

---

### LOW-P36-001: BC-1.05.035 §Description stale qualifier in ADR-015 awareness clause

**Severity:** LOW
**Location:** BC-1.05.035.md §Description line 33 — ADR-015 Awareness clause

**Finding:** The ADR-015 Awareness clause says this BC's denial-path postcondition references `internal.capability_denied` with reason `"symlink_traversal_escape"`. After the architectural reframe (D-279), this denial reason no longer exists. The clause needs updating to reflect that the only denial-path event emitted is via the existing `emit_denial("binary_not_on_allow_list")` path, not a novel symlink reason.

**Status:** CLOSED implicitly by D-279 Fix 1 (symlink_traversal_escape concept dropped; existing allow-list emit_denial path is the only path; ADR-015 awareness clause retained for event-rename tracking).

---

## Summary

Pass-36 caught a fundamental architectural error in v1.32: the "trusted project-root prefix" concept was coined without an upstream definition and is anti-correct for the primary S-9.07 use case. The fix: drop the prefix-check mechanism entirely, reframe BC-1.05.035 around TOCTOU prevention (canonicalize feeds the canonical path to the existing `binary_allowed()` check), and unify the symlink case under the existing `CAPABILITY_DENIED` (-1) denial path.

**ADR-013 clock:** RESET to 0_of_3 by this SUBSTANTIVE verdict. Three consecutive NITPICK_ONLY passes (37/38/39) needed for CONVERGENCE_REACHED.
