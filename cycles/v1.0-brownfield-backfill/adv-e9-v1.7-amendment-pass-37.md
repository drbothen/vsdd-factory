# Adversarial Review — Pass 37 (E-9 v1.7 Amendment Surface)

**Subject:** E-9 epic v1.33 (sealed at 5f491e5 on factory-artifacts) — symmetry audit of BC-1.05.035 vs BC-1.05.036.

**ADR-013 clock at start of pass:** 0_of_3 (reset by pass-36 D-279).

**TD-VSDD lessons reviewed:** TD-VSDD-057 through TD-VSDD-083 in `.factory/cycles/v1.0-brownfield-backfill/lessons.md`. Confirmed angle is NEW (not in 36-angle inventory) and not recapitulating any closed-out finding.

## 1. Angle (NEW)

**Symmetry audit: BC-1.05.035 vs BC-1.05.036 read side-by-side as a sibling pair.**

Prior 36 angles audited each BC standalone, intra-document siblings, source-code mechanism claims, terminology-family grep coverage, ADR cross-anchors, semantic anchoring, frontmatter coherence, etc. The pass-22 "intra-document semantic-sibling sweep" angle worked WITHIN one BC. The pass-21 "BC-only deep-dive" worked on each BC standalone. **No prior pass treated 035 and 036 as a structural pair and asked: where do their normative surfaces fail to mesh?**

This pass reads both files line-by-line in parallel and looks for: cross-BC enumeration coherence, cross-BC mechanism dependency, cross-BC routing/emission consistency, cross-BC citation symmetry, cross-BC scope-asymmetry.

## 2. Findings

### HIGH-P37-001 — BC-1.05.035 introduces a 5th CAPABILITY_DENIED path that contradicts BC-1.05.036 EC-003's exhaustive 4-reason enumeration

**Severity:** HIGH
**Location:**
- `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:49` (Postcondition 3 — canonicalize() Err → CAPABILITY_DENIED -1)
- `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:85` (EC-003 enumerates exactly 4 denial reasons)

**Description:** BC-1.05.035 Postcondition 3 introduces a NEW CAPABILITY_DENIED (-1) return path for canonicalize() failure (missing binary, IO error, symlink loop). Postcondition 3 explicitly acknowledges this is a BEHAVIOR CHANGE from `INTERNAL_ERROR (-99)` → `CAPABILITY_DENIED (-1)`. BC-1.05.036 EC-003 enumerates capability-check failure reasons as EXACTLY four: `no_exec_subprocess_capability`, `binary_not_on_allow_list`, `shell_bypass_not_acknowledged`, `setuid_or_setgid_binary` (citing exec_subprocess.rs:148/155/162/169). After BC-1.05.035 lands, there will be a 5th CAPABILITY_DENIED return-site (canonicalize() failure, before line 152). The asymmetry surfaces a deeper specification gap in BC-1.05.035 itself: Postcondition 3 does NOT specify whether the new canonicalize-failure CAPABILITY_DENIED path emits an `internal.capability_denied` event. The four existing denial paths all emit. If the canonicalize-failure path emits → BC-1.05.036 EC-003's enumeration is wrong by 1. If it does NOT emit → BC-1.05.035 introduces a silent CAPABILITY_DENIED path (a SOUL.md #4 violation and asymmetric with the 4 existing emit-on-deny paths).

**Recommendation:** Decide: does canonicalize-failure emit `internal.capability_denied` with a 5th `reason` (e.g., `"binary_canonicalize_failed"`)? If yes, add to BC-1.05.035 Postcondition 3 explicitly AND extend BC-1.05.036 EC-003 enumeration to 5 reasons. If no, add explicit "no event emitted in v1; first silent CAPABILITY_DENIED path" disclosure to both. Either way, both BCs must reflect the same outcome.

### HIGH-P37-002 — BC-1.05.035 mechanism is incomplete: canonical path fed only to allow-check, not to Command::spawn — undermines stated TOCTOU goal

**Severity:** HIGH
**Location:**
- `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:28,35,37,47` (TOCTOU framing + Postcondition 1)
- `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` (current `Command::new(cmd)` site, INSIDE execute_bounded)

**Description:** BC-1.05.035's H1, §Description, and §Description all assert TOCTOU prevention as the architectural rationale. Postcondition 1 ONLY mandates: "The canonical path (resolved symlinks, no `..` segments) is fed to `binary_allowed()` instead of the raw `cmd`." It does NOT mandate that the canonical path also be passed to `Command::new(...)` at line 230. After BC-1.05.035 lands as currently specified, an implementer would canonicalize at line 152, leave line 173 (`execute_bounded(cmd, args, ...)`) unchanged, and line 230 still becomes `Command::new(cmd)` with raw cmd. Result: between line 152 (allow-list check on canonical) and line 230 (spawn on raw), an attacker can swap a symlink target — the EXACT TOCTOU scenario the BC claims to prevent. Asymmetric with BC-1.05.036 EC-006 which declares the event payload `binary` field as "canonicalized full path" — the success-path event LITERALLY DEPENDS on BC-1.05.035 propagating the canonical value to the spawn site. Currently neither BC closes that loop.

**Recommendation:** Amend BC-1.05.035 Postcondition 1 to require the canonical path be used at BOTH the allow-list check AND the `Command::new` spawn site (canonical path propagated through `execute_bounded`). Cross-reference BC-1.05.036 EC-006 (event payload binary field) to make the propagation requirement bidirectional.

### HIGH-P37-003 — BC-1.05.036 Postcondition 4 asserts ctx.emit_internal routes to events-*.jsonl, but source code shows it routes to internal_log (dispatcher-internal-*.jsonl) — same-as-existing-emit_denial claim is misleading

**Severity:** HIGH
**Location:**
- `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:51` (Postcondition 4)
- `crates/factory-dispatcher/src/host/mod.rs:109-116` (emit_internal definition)

**Description:** BC-1.05.036 Postcondition 4 asserts: "Event is routed through `ctx.emit_internal` to the single-stream `FileSink` writing to `events-*.jsonl` per ADR-015 D-15.1 [...] Same code path as the existing `emit_denial` call." Source-of-truth at host/mod.rs:109-116 shows emit_internal writes to `internal_log` (dispatcher-internal-*.jsonl per ADR-007), PLUS pushes to in-memory `events: Vec<InternalEvent>`. The existing `emit_denial` calls `ctx.emit_internal(ev)` and that `internal.capability_denied` event currently lands in `dispatcher-internal-*.jsonl` (this is the ENTIRE motivation of ADR-015 D-15.1). The "Same code path as the existing emit_denial call" is true (same fn pointer), but the claim that this code path writes to `events-*.jsonl` is FALSE for the current source state. BC-1.05.035 carefully tags the event NAME as INTERIM. BC-1.05.036 makes both an INTERIM claim about NAME and an asserted-as-current claim about ROUTING. The routing claim is contradicted by the source.

**Recommendation:** Either mark the routing claim as INTERIM ("Currently routes to dispatcher-internal-*.jsonl; ADR-015 D-15.1 mandates migration to events-*.jsonl before the host-emit-fix story merges") OR specify the ADR-015 implementation work as a normative requirement of BC-1.05.036's Postconditions. Either way, restore symmetry with BC-1.05.035's careful INTERIM tagging.

### MED-P37-001 — BC-1.05.036 Architecture Anchors line 304-309 cites the emit_denial FUNCTION DEFINITION as "existing emit_denial call" — semantic mis-anchor

**Severity:** MEDIUM
**Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:68`; `crates/factory-dispatcher/src/host/exec_subprocess.rs:304-309` (function definition, NOT a call)

**Description:** BC-1.05.036 §Architecture Anchors reads: "`crates/factory-dispatcher/src/host/exec_subprocess.rs:304-309` — existing `emit_denial` call". Lines 304-309 are the **function definition** of `emit_denial`, not a CALL site. The actual `emit_denial(...)` CALL sites are at lines 148, 155, 162, 169. Asymmetric with BC-1.05.035 which correctly cites line 155 as the CALL site.

**Recommendation:** Either change cite to one of the four call sites (preferred — matches 035), OR keep 304-309 and change "existing emit_denial call" to "existing emit_denial function (the template the new emit-on-success call will follow)".

### MED-P37-002 — BC-1.05.035 EC-005 NUL-byte mechanism is Unix-specific (CString conversion / EINVAL) but BC makes no cross-platform parity disclosure

**Severity:** MEDIUM
**Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:48,52,86`

**Description:** BC-1.05.035 EC-005: "Returns `CAPABILITY_DENIED` (-1) via Precedence Ladder step 2 (`Path::new(cmd).canonicalize()` returns Err with EINVAL on NUL-containing paths via Unix CString conversion in std::path layer)". The cited mechanism is **Unix-specific**. The source code at exec_subprocess.rs:202-217 explicitly splits Unix vs non-Unix for `refuse_setuid` — meaning the BC family DOES recognize cross-platform parity as a concern. EC-005 silently assumes Unix.

**Recommendation:** Either generalize EC-005 to assert NUL-containing paths fail at canonicalize() across all supported platforms with platform-specific subnotes, OR explicitly scope EC-005 to Unix and add Windows handling (or out-of-scope-for-v1 disclosure).

### MED-P37-003 — Architecture Anchors structural asymmetry: BC-1.05.035 has 2 anchor bullets, BC-1.05.036 has 3; the missing 035 anchor is the sibling-pattern reference

**Severity:** MEDIUM
**Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:65-68`; `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:65-69`

**Description:** BC-1.05.036 §Architecture Anchors cites a SIBLING IMPLEMENTATION PATTERN (the `emit_denial` template the new emit-on-success will mirror). BC-1.05.035 has NO equivalent sibling-pattern anchor — the BC describes a NEW behavior (canonicalize-then-allow-check) but doesn't anchor it to an existing analog like read_file.rs:122-148 (path_allow canonicalize-and-loop pattern).

**Recommendation:** Add a third Architecture Anchor bullet to BC-1.05.035 citing `read_file.rs:122-148` (path_allow canonicalize-and-loop sibling pattern) OR BC-1.05.005 / BC-1.05.032 as BC-level siblings. Restores 3-bullet symmetry with 036.

### LOW-P37-001 — TD-VSDD-074 citation discipline asymmetry between the two ADR-015 awareness clauses

**Severity:** LOW
**Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:32` (cites "TD-VSDD-074"); `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:32` (no TD-VSDD-074 cite)

**Recommendation:** Add "per TD-VSDD-074" to BC-1.05.036 line 32 to match BC-1.05.035's citation discipline.

### LOW-P37-002 — INTERNAL_ERROR line citation imprecision in BC-1.05.036

**Severity:** LOW
**Location:** `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:52,89`; `crates/factory-dispatcher/src/host/exec_subprocess.rs:259,262`

**Description:** BC-1.05.036 cites ":259" for "stdin take/write failure". Line 259 is the **error-check predicate** (`if write_all().is_err()`), not the return statement. The actual `return Err(codes::INTERNAL_ERROR)` is line 262. Off by 3 lines.

**Recommendation:** Change ":259" → ":259-262" (or ":259-262" for the check+return range) in both Postcondition 5 and EC-007.

## 3. Verdict

**SUBSTANTIVE.** Three HIGH severity findings (P37-001, P37-002, P37-003) and three MEDIUM findings substantively affect implementer interpretation of the BC pair. ADR-013 clock RESETS to 0_of_3.

## 4. Process-Gap Tagging

- HIGH-P37-001 and HIGH-P37-003: 2nd recurrence of mechanism-verification class within v1.31..v1.33 burst window — does NOT yet warrant a new TD-VSDD entry. **No `[process-gap]` tag.**
- HIGH-P37-002: NEW class — "asserted goal vs mandated mechanism mismatch." BC asserts TOCTOU prevention but Postconditions only mandate canonicalize at allow-check site, not at spawn site. Specifically not covered by TD-VSDD-083 (UPSTREAM concept anchoring); this is INTERNAL goal-vs-mechanism coherence within a single BC. **Tagging as [process-gap] (provisional).** Recommend: codify as TD-VSDD-084 if recurrence observed.
- MEDs and LOWs: content defects, not process gaps.

## 5. Source-of-Truth Verification Log

| Claim | Source file:line | What source says | BC matches? |
|-------|------------------|------------------|-------------|
| `binary_allowed()` is at exec_subprocess.rs:152 | exec_subprocess.rs:152 | `if !binary_allowed(cmd, &caps.binary_allow) {` | YES |
| `Command::new(cmd)` at line 230 INSIDE execute_bounded | exec_subprocess.rs:230,220 | `let mut command = Command::new(cmd);` at 230, inside `execute_bounded` starting 220 | YES |
| 4 emit_denial CALL sites at 148/155/162/169 | exec_subprocess.rs:148,155,162,169 | All four contain `emit_denial(...)` calls | YES |
| emit_denial FUNCTION DEFINITION at 304-309 | exec_subprocess.rs:304-309 | `fn emit_denial(...) { ... }` | BC-1.05.036 line 68 calls these "existing emit_denial call" — INCORRECT. See MED-P37-001. |
| Spawn at line 252 returns INTERNAL_ERROR on err | exec_subprocess.rs:252 | `let mut child = command.spawn().map_err(\|_\| codes::INTERNAL_ERROR)?;` | YES |
| stdin take at :258, write at :259, return at :262 | exec_subprocess.rs:258,259,262 | 258 take, 259 write_all is_err check, 262 return | PARTIAL: write-failure return is 262 not 259. See LOW-P37-002. |
| stdout/stderr take at :267-268 | exec_subprocess.rs:267,268 | both `take().ok_or(codes::INTERNAL_ERROR)?` | YES |
| try_wait error at :299 | exec_subprocess.rs:299 | `Err(_) => return Err(codes::INTERNAL_ERROR),` | YES |
| INTERNAL_ERROR=-99 at host/mod.rs:184 | host/mod.rs:184 | `pub const INTERNAL_ERROR: i32 = -99;` | YES |
| ctx.emit_internal at host/mod.rs:109-116 | host/mod.rs:109-116 | Calls `internal_log.write` and `events.lock().push`; does NOT write to FileSink/events-*.jsonl | BC-1.05.036 Postcondition 4 INCORRECT. See HIGH-P37-003. |
| read_wasm_string at memory.rs:47-54 only rejects non-UTF-8 | memory.rs:47-54 | `String::from_utf8(bytes).map_err(\|_\| HostCallError::InvalidUtf8)` | YES |
| ADR-015 D-15.2 has 5 categories | ADR-015 lines 317-332 | `lifecycle \| domain \| audit \| error \| unknown` | YES |
| ADR-015 D-15.2 line 329 maps `vsdd.capability.denied.*` to audit | ADR-015 line 329 | YES | YES |
| OQ-W16-001 exists | open-questions.md lines 19-34 | YES | YES |
| Gap-analysis line 326 anchors OQ-W16-001 | gap-analysis-w16-subprocess.md line 326 | "Resolution tracked in **OQ-W16-001**" | YES |

## 6. TD-VSDD Lesson Awareness

I confirm I reviewed TD-VSDD-057 through TD-VSDD-083. Pass-37 angle (cross-BC sibling-pair structural mesh) is NEW. Findings are net-new at the cross-BC sibling-pair level. No closed-out finding recapitulated.
