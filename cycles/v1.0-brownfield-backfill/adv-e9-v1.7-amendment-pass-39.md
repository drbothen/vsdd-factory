# Adversarial Review — Pass 39 (E-9 v1.35 amendment surface, sealed at 1d87307)

## 1. Angle (NEW per TD-VSDD-057)

**Two-part: diff-only of v1.35 (D-281) + TD-VSDD-085 self-application audit.** Part A: audit ONLY content added/changed in D-281 diff for internal coherence and absence of newly-introduced defects. Part B: apply the just-codified TD-VSDD-085 NORMATIVE rule retroactively to the v1.35 burst itself — for each new mechanism string or normative Edge Case introduced, verify a TV witness row exists.

Methodologically novel because TD-VSDD-085 didn't exist before pass-38 codified it; pass-39 is the first opportunity to apply it self-referentially.

## 2. Diff inventory (v1.34 → v1.35 = D-281)

| File | Change Class |
|---|---|
| `adv-e9-v1.7-amendment-pass-38.md` | NEW review file |
| `lessons.md` | TD-VSDD-085 NORMATIVE added |
| `BC-1.05.035.md` | EC-001 rewrite Branch A/B; EC-007 cross-ref; EC-008/009/010 added; TV rows 4 amended + 5 + 6; NFD/NFC §Architecture Anchors note |
| `BC-1.05.036.md` | P1 footnotes (signal-death + best-effort); P2 stdout/stderr_bytes timing; P6 best-effort emit added; EC-006 cross-ref; EC-009/010/011 added; Purity Thread safety updated; input bounds note |
| `E-9-tier-2-native-wasm-migration.md` | v1.35 H3 + frontmatter |
| `open-questions.md` | OQ-W16-002/003/004/006 added |
| `STATE.md` | D-281 row + Phase Steps |
| `STORY-INDEX.md` | 1.87→1.88 |

## 3. Findings

### HIGH-P39-001 — OQ-W16-005 dangling reference; filed but never published

**Severity:** HIGH **Confidence:** HIGH
**Location:** BC-1.05.035.md:93 (EC-009) cites `OQ-W16-005`; STATE.md:152 (D-281 row) and E-9 epic line 1792 cite "OQ-W16-005 filed"; **open-questions.md ABSENT** — file jumps W16-004 → W16-006

**Description:** Three coherent claims that OQ-W16-005 was filed contradict the actual register state. Future readers following EC-009's pointer hit a dead reference. v1.35 H3 changelog "**New OQs filed:**" omits OQ-W16-005, listing only OQ-W16-002/003/004/006.

**Recommendation:** Add OQ-W16-005 to open-questions.md (preferred; narrative repeatedly says it was filed; POLICY 1 prevents scrubbing the changelog claim).

### HIGH-P39-002 — Markdown table column-count violation on all six new EC rows

**Severity:** HIGH **Confidence:** HIGH
**Location:** BC-1.05.035.md:83-94 (EC table 3-col header; rows EC-008/009/010 have 4 cells); BC-1.05.036.md:84-96 (EC table 3-col header; rows EC-009/010/011 have 4 cells)

**Description:** All six new EC rows added in D-281 carry a trailing 4th cell with category tag (`| edge-case |`, `| known-limitation |`, `| best-effort-silent-drop |`). Table header has 3 columns — extra cells silently dropped or misaligned depending on renderer. Pre-v1.35 rows all have exactly 3 cells. Systematic markdown table syntax defect introduced uniformly across the burst — process-gap candidate (BC editor has no markdown-table-arity validation).

**Recommendation:** Either extend EC table header to 4 columns (`| ID | Description | Expected Behavior | Category |`) and add categories to all pre-existing rows for symmetry, OR merge category tag into Expected Behavior cell (e.g., `... CAPABILITY_DENIED (-1). [edge-case]`). Option (b) preserves backward symmetry.

### HIGH-P39-003 — TD-VSDD-085 self-violation: 3 new normative Edge Cases without TV witnesses

**Severity:** HIGH **Confidence:** HIGH
**Location:** BC-1.05.036.md:94-96 (EC-009 signal-death; EC-010 emit IO failure; EC-011 Mutex poison); BC-1.05.036.md:100-110 (TV table — 9 rows; none witness EC-009/010/011); lessons.md:760-775 (TD-VSDD-085 adversary axis)

**Description:** TD-VSDD-085 codified in this same burst declares: "For each new normative Edge Case row or new mechanism string introduced in this burst, grep the BC's §Canonical Test Vectors for a row witnessing it. Flag any new mechanism without witness as HIGH severity." Three new ECs in BC-1.05.036 each lack a TV witness:
- EC-009 signal-death `status.code() returns None` — no TV row
- EC-010 emit IO failure `log.write Err discarded` — no TV row
- EC-011 Mutex poison `if let Ok(mut events) lock match fails` — no TV row

The very burst that codified the rule simultaneously violates it three times. Postcondition 6 best-effort emit also lacks a TV witness (sibling to EC-010).

**Recommendation:** Add three TV rows to BC-1.05.036 witnessing (a) signal-death emission with exit_code=-1 and outcome='failure', (b) emit_internal IO failure → silent drop with host call return unaffected, (c) Mutex poisoned by concurrent panic → emit_internal silent drop. Each row should explicitly state the source-code mechanism so future POLICY 12 / TD-VSDD-085 hook checks witness mechanism strings.

### MED-P39-001 — EC-005 self-contradiction with renumbered Precedence Ladder

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035.md:89 (EC-005 cites "step (3)"); :52 (Ladder def step (2) = canonicalize Err); :104 (TV row 5 NUL cites "step (2)")

**Description:** EC-005 (NUL byte) says "via Precedence Ladder step (3)". The ladder defines step (2) as canonicalize Err → `binary_canonicalize_failed`. NUL-byte rejection happens at canonicalize step (2). TV row 5 NUL-byte witness correctly cites "step (2)". EC-008 and EC-010 (other canonicalize-fail cases) correctly cite "step (2)". Only EC-005 carries stale "step (3)" — residue from v1.32→v1.33 architectural reframe (D-279) that dropped the prefix-check step and renumbered. v1.35's TD-VSDD-076 sibling sweep failed to catch it.

**Recommendation:** Change EC-005 "step (3)" → "step (2)".

### MED-P39-002 — EC-009 in BC-1.05.035 references nonexistent "step (4)"

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035.md:93 (EC-009: "→ step (4): `Err(codes::INTERNAL_ERROR)` (-99)"); :52 (Ladder defines exactly 3 steps)

**Description:** EC-009 (cmd is a directory) describes path: canonicalize succeeds → step (3) allow-check passes → spawn fails → "step (4): Err(codes::INTERNAL_ERROR) (-99)". Ladder enumerates only 3 steps; no step (4) defined. Either extend Ladder to declare step (4), OR reword EC-009 to drop ladder-step reference.

**Recommendation:** Reword EC-009: drop "step (4)" reference; replace with "post-Ladder spawn failure path (exec_subprocess.rs:252) → Err(codes::INTERNAL_ERROR) (-99); no emit_denial; no event".

### MED-P39-003 — Postcondition 6 best-effort emit has no TV witness (sibling to HIGH-P39-003)

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.036.md:53 (P6); :100-110 (TV table)

**Description:** P6 introduces normative best-effort emit semantic but no TV row asserts that `Ok(envelope)` is returned EVEN WHEN `log.write` fails. Sibling to EC-010 same gap. POLICY 12 + TD-VSDD-085 require TV witness.

**Recommendation:** Use the EC-010 TV witness (recommended in HIGH-P39-003) with cross-reference, OR add a dedicated row.

### MED-P39-004 — Postcondition 1 main clause "subprocess process actually exits" excludes signal-death

**Severity:** MEDIUM **Confidence:** MEDIUM
**Location:** BC-1.05.036.md:48

**Description:** P1 main clause "subprocess process actually exits before timeout" strictly excludes signal-death (kernel SIGKILL/SIGSEGV — child does NOT "exit", it is "terminated"). Yet footnote AND EC-009 say signal-death IS treated as success-path emission. P1 main clause is inconsistent with its own footnote. Source `try_wait` returns `Ok(Some(status))` for both literal exits and signal-deaths uniformly.

**Recommendation:** Reword P1 main clause: "On successful subprocess completion (i.e., `child.wait()` returns `Ok(Some(status))` within timeout AND output cap; see Postcondition 5 for error-path reality and EC-009 for signal-death substitution)".

### MED-P39-005 — read_wasm_bytes error-mapping precision

**Severity:** MEDIUM **Confidence:** MEDIUM
**Location:** BC-1.05.036.md:55 (input bounds note); host/memory.rs:35 returns `HostCallError::OutOfBounds`; exec_subprocess.rs:54-67 maps to INVALID_ARGUMENT

**Description:** Input bounds note (LOW-P38-002 closure) claims `read_wasm_bytes` itself returns INVALID_ARGUMENT (-4). Function actually returns `Result<Vec<u8>, HostCallError>` with `HostCallError::OutOfBounds`. Mapping to `codes::INVALID_ARGUMENT` happens at caller. TD-VSDD-081 mechanism-precision class.

**Recommendation:** Reword: "(a) `read_wasm_bytes` at host/memory.rs:35 returns `HostCallError::OutOfBounds` for `len` exceeding guest memory size; the caller at exec_subprocess.rs:54-67 maps the error to `codes::INVALID_ARGUMENT` (-4)".

### LOW-P39-001 — TD-VSDD-085 codification "5 recurrences" accounting

**Severity:** LOW
**Location:** lessons.md:768; :764; epic line 1798

**Description:** Three slightly different counts of "recurrences" (passes vs findings granularity inconsistent across narrative). Both readings meet S-7.02 threshold; codification stands. Imprecision only.

**Recommendation:** Pick one granularity (e.g., "5 prior fix-burst-introduced-mechanisms-without-TV-witness instances across passes 24/29/31/37/38, including 2 within pass 38 (HIGH-P38-001 4th + MED-P38-001 5th)."

### LOW-P39-002 [process-gap] — Mission template references nonexistent ADR-015 filename

**Severity:** LOW (process-gap)
**Location:** orchestrator dispatch text references `ADR-015-single-stream-otel-emit-contract.md`; actual file is `ADR-015-single-stream-otel-schema.md`

**Description:** Orchestrator/mission-template defect, not artifact content defect. Tagged [process-gap]. Recommend orchestrator's adversary-dispatch template resolve ADR-015 filename via Glob.

## 4. TD-VSDD-085 Self-Application Audit Summary

| New mechanism | TV witness? | Verdict |
|---|---|---|
| `binary_canonicalize_failed` (035) | TV rows 4 (amended) + 5 (NUL) | PASS |
| ELOOP / ENAMETOOLONG (035 EC-008/EC-010) | transitive only | PARTIAL |
| EISDIR/EACCES directory-cmd (035 EC-009) | none | **FAIL** (covered by HIGH-P39-001 dangling OQ rather than missing TV; semantic OK) |
| `status.code() == None` signal-death (036 EC-009) | none | **FAIL** (HIGH-P39-003) |
| `log.write` Err discarded (036 EC-010 / P6) | none | **FAIL** (HIGH-P39-003 + MED-P39-003) |
| Mutex poison silent-drop (036 EC-011) | none | **FAIL** (HIGH-P39-003) |
| EC-001 Branch A/B rewrite (035) | TV rows 1+4 cover | PASS |
| EC-007 canonical-path propagation | TV row 6 added | PASS |
| OQ-W16-002/003/004/006 cross-refs | open-questions.md present | PASS |
| OQ-W16-005 cross-ref | **open-questions.md ABSENT** | **FAIL** (HIGH-P39-001) |

## 5. Verdict

**SUBSTANTIVE.** 3 HIGH + 5 MED + 2 LOW. ADR-013 clock RESETS to 0_of_3.

## 6. Process-Gap Tagging

- **HIGH-P39-002 [process-gap]:** BC editor introduces extra `|` cells that desync from table header without producing an error. Recommend markdown-table-arity validator alongside TD-VSDD-085 hook.
- **LOW-P39-002 [process-gap]:** Mission template references nonexistent ADR-015 filename slug. Recommend orchestrator resolve via Glob.

## 7. Source-of-Truth Verification Log

15 source-claims re-verified per TD-VSDD-075/078/081. 14 PASS; 1 DEGRADED (MED-P39-005 read_wasm_bytes mapping precision).

## 8. TD-VSDD Lesson Awareness

Confirmed TD-VSDD-057 through TD-VSDD-085 reviewed. Pass-39 angle NEW. No closed-out finding recapitulated. Self-validation complete; all findings carry specific file:line evidence.
