---
document_type: adversarial-review
pass: 25
verdict: SUBSTANTIVE
epic: E-9
version_reviewed: "1.24"
angle: source-code-traceability-exhaustive-sweep
date: 2026-05-05
findings_high: 1
findings_med: 2
findings_low: 2
adr_013_clock: 0_of_3
clock_action: RESET
---

# Pass-25 Adversarial Review — E-9 v1.24 Amendment Surface

**Angle:** Source-code traceability exhaustive sweep (NEW per TD-VSDD-057) — reads the actual source files cited in BCs and verifies every source-code claim against the live codebase. Enumerates every `emit_denial`, `emit_event`, constant definition, and line citation in BC-1.05.036 and BC-1.05.035 against `crates/factory-dispatcher/src/host/exec_subprocess.rs` and related source files.

**Verdict:** SUBSTANTIVE — 1 HIGH / 2 MED / 2 LOW. ADR-013 clock RESET to 0_of_3.

**Source-code verification scope:** 22 source-claims verified PASS; 1 HIGH source-fabrication caught (BC-1.05.036:52 denial-path enumeration). Full verification log below.

---

## Findings Summary

| ID | Severity | Location | Description |
|----|----------|----------|-------------|
| H-P25-001 | HIGH | BC-1.05.036:52 | Fabricated 4-path denial enumeration ("env not allowed, cwd not allowed") — not present in source |
| M-P25-001 | MED | BC-1.05.036:85 §EC-003 | EC-003 description "Capability check fails" does not enumerate the 4 real denial reasons |
| M-P25-002 | MED | BC-1.05.036:50 | Instant cite at exec_subprocess.rs:270 is post-spawn deadline, not spawn-time Instant |
| L-P25-001 | LOW | gap-analysis line 216 | Unbalanced parenthesis — cosmetic markdown |
| L-P25-002 | LOW | perf-baseline frontmatter | `references:` field convention non-canonical vs formal `inputs:` pattern |

---

## H-P25-001 — HIGH: BC-1.05.036:52 fabricates 4-path denial enumeration

**Location:** BC-1.05.036 §Postconditions item 5 (line 52)

**Current text (fabricated):**
> "Today only `internal.capability_denied` is emitted on the 4 denial paths (binary not allowed, shell bypass not acknowledged, env not allowed, cwd not allowed)."

**Source-of-truth verification:** Read `crates/factory-dispatcher/src/host/exec_subprocess.rs` lines 147–171.

Actual `emit_denial` callsites:
- Line 148: `emit_denial(ctx, cmd, "no_exec_subprocess_capability", Map::new());`
- Line 155: `emit_denial(ctx, cmd, "binary_not_on_allow_list", details);`
- Line 162: `emit_denial(ctx, cmd, "shell_bypass_not_acknowledged", details);`
- Line 169: `emit_denial(ctx, cmd, "setuid_or_setgid_binary", details);`

**Fabrication analysis:**
- "binary not allowed" is an imprecise paraphrase of `binary_not_on_allow_list` — reason string differs.
- "shell bypass not acknowledged" paraphrases `shell_bypass_not_acknowledged` — close but not the source literal.
- "env not allowed" — NO `emit_denial` callsite for env filtering exists in source. Per gap-analysis Section 1, `env_allow` violations are silently filtered (items not in allow-list are stripped without emitting any event).
- "cwd not allowed" — NO `emit_denial` callsite for cwd filtering exists in source. Per gap-analysis Section 1, `cwd_allow` is currently unenforced.
- "no_exec_subprocess_capability" — MISSING from BC enumeration. This is the FIRST denial path checked (line 148) and fires when the capability block is absent entirely.
- "setuid_or_setgid_binary" — MISSING from BC enumeration. This fires at line 169.

**Impact:** An implementer writing tests or documentation from this BC would test for env/cwd denial events that do not exist, and would miss the capability-absent and setuid denial paths that do exist. Source fabrication directly undermines the BC's purpose as a behavioral contract.

**Required fix:** Replace the parenthetical with the 4 actual emit_denial reason strings per source lines 148/155/162/169, plus a clarification note about env_allow (silently filtered) and cwd_allow (unenforced per gap-analysis §1).

---

## M-P25-001 — MED: §EC-003 description needs the 4 real denial reasons

**Location:** BC-1.05.036 §Edge Cases table EC-003 (line 85)

**Current text:**
> "Capability check fails | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted"

**Issue:** The Description column says only "Capability check fails" without specifying WHICH capability check paths are covered. Now that H-P25-001 fixes §Postcondition 5 to enumerate the 4 real denial reasons, §EC-003 should be tightened as a sibling section per TD-VSDD-076 discipline to reference the actual check paths and explicitly note that env_allow + cwd_allow violations do NOT trigger this EC.

**Required fix:** Expand EC-003 Description to enumerate the 4 real denial reasons and note the env_allow/cwd_allow non-triggering paths.

---

## M-P25-002 — MED: BC-1.05.036:50 Instant cite references wrong line

**Location:** BC-1.05.036 §Postconditions item 3 (line 50)

**Current text:**
> "the deadline `Instant` already present in `execute_bounded` (exec_subprocess.rs:270) is the reference."

**Source-of-truth verification:** Read `exec_subprocess.rs` lines 248–275:
- Line 252: `let mut child = command.spawn().map_err(|_| codes::INTERNAL_ERROR)?;` — this is the actual spawn point.
- Line 270: `let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);` — this is the POST-SPAWN deadline computation. It creates the deadline, not a spawn-time duration reference.

**Fabrication analysis:** The Postcondition says `duration_ms` is measured using the line 270 `Instant` as reference. But line 270 creates a DEADLINE `Instant` (spawn time + timeout), not a spawn-time capture. Using it as the `duration_ms` reference would compute `deadline - now_at_exit` which yields negative values when exit is before deadline. The actual implementation should add a `let started = Instant::now();` immediately before `command.spawn()` at line 252. The implementer following this BC verbatim would produce incorrect duration measurements.

**Required fix:** Correct line 270 reference to note it is a post-spawn deadline (not spawn time) and direct implementer to add a `let started = Instant::now();` capture before `command.spawn()` at line 252.

---

## L-P25-001 — LOW: gap-analysis line 216 unbalanced parenthesis

**Location:** `.factory/architecture/gap-analysis-w16-subprocess.md` line 216

**Issue:** Closing parenthesis without matching open parenthesis in a list item. Cosmetic. Markdown renders acceptably; no semantic impact.

**Disposition:** SKIP. Cosmetic markdown defect. Markdown renderers handle gracefully. No semantic impact on implementer-facing content.

---

## L-P25-002 — LOW: perf-baseline `references:` field non-canonical frontmatter convention

**Location:** `.factory/architecture/perf-baseline-w16.md` frontmatter `references:` field

**Issue:** The `references:` field convention is not in the canonical frontmatter schema (which uses `inputs:` for source documents and citations). The `references:` field may be a deliberate distinction (supplementary citations vs primary inputs) or a convention drift.

**Disposition:** SKIP. Pending intent clarification. L-P15-002 deferred with rationale per D-257 (field may be deliberate distinction from formal `inputs:`). No change warranted without PO judgment on schema intent.

---

## Source-Code Verification Log (TD-VSDD-075 + TD-VSDD-078)

All 22 source-claims in BC-1.05.036 and BC-1.05.035 audited against source:

| Claim | File | Line | Verified? |
|-------|------|------|-----------|
| `emit_denial(ctx, cmd, "no_exec_subprocess_capability", ...)` | exec_subprocess.rs | 148 | PASS |
| `emit_denial(ctx, cmd, "binary_not_on_allow_list", ...)` | exec_subprocess.rs | 155 | PASS |
| `emit_denial(ctx, cmd, "shell_bypass_not_acknowledged", ...)` | exec_subprocess.rs | 162 | PASS |
| `emit_denial(ctx, cmd, "setuid_or_setgid_binary", ...)` | exec_subprocess.rs | 169 | PASS |
| `command.spawn()` (actual spawn point) | exec_subprocess.rs | 252 | PASS |
| `let deadline = Instant::now() + Duration::from_millis(...)` (post-spawn deadline) | exec_subprocess.rs | 270 | PASS |
| BC-1.05.036:52 — denial paths enumeration "env not allowed, cwd not allowed" | exec_subprocess.rs | N/A | **FAIL — FABRICATED** |
| BC-1.05.036:50 — "exec_subprocess.rs:270 is the reference" for duration_ms | exec_subprocess.rs | 270 | **FAIL — wrong line role** |
| TIMEOUT = -2 (per mod.rs:181) | mod.rs | 181 | PASS |
| OUTPUT_TOO_LARGE = -3 (per mod.rs:182) | mod.rs | 182 | PASS |
| success path at exec_subprocess.rs:285-288 (no emit currently) | exec_subprocess.rs | 285–288 | PASS |
| existing emit_denial at exec_subprocess.rs:304-309 (gap-analysis cite) | exec_subprocess.rs | 304 | PASS (pattern exists) |
| All other 10 source-file-agnostic claims | N/A | N/A | PASS |

**Result:** 22 claims audited. 2 FAIL (H-P25-001 + M-P25-002 basis). All other 20 PASS.

---

## ADR-013 Clock

- Prior clock: 0_of_3 (reset by pass-24 SUBSTANTIVE per D-267)
- This pass: SUBSTANTIVE (1H/2M/2L)
- Clock action: **RESET — remains 0_of_3**
- Required path to CONVERGENCE_REACHED: 3 consecutive NITPICK_ONLY passes (26/27/28)

---

## Pass Methodology Note

This pass uses the source-code-traceability exhaustive sweep angle (NEW per TD-VSDD-057 menu extension per TD-VSDD-078 rationale). The angle:
1. Enumerates every source-file citation (file:line) in all BCs in scope.
2. Reads each cited line in the actual source file.
3. Verifies the claim made in the BC matches the actual source.
4. Specifically targets enumeration claims (lists of error codes, denial reasons, field names) per TD-VSDD-078 extension.

This angle was the first of 25 passes to grep exec_subprocess.rs for the actual `emit_denial` callsites. Passes 21/22 (D-264/D-265) verified error codes (-2/-3) but did not enumerate the denial reason strings. Pass-25 specifically targeted reason-string enumeration and caught the fabrication.
