# Adversarial Review — Pass 41 (E-9 v1.7 amendment surface, epic v1.37 sealed at f7c229e)

## 1. Angle (NEW per TD-VSDD-057)

**Type-signature-verification audit.** Methodologically novel vs. all 40 prior angles. Forces Rust type-system reasoning: for every function-signature claim in BC prose (return type, parameters, error pattern, control flow), open the Rust source at the cited file:line and verify the actual signature, the actual `Result`/`()`/`Option`/panic posture, and whether downstream BC claims that depend on the signature are coherent. Extends HIGH-P40-001's lesson — that mechanism descriptions can be wrong at the type-system level and survive multiple bursts because no prior angle catches type-mismatch.

20 function citations audited.

## 2. Findings

### MED-P41-001 — `host/mod.rs:72` mischaracterized as "planned-implementation comment" for INTERNAL_HOST_FUNCTION_PANIC

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035 EC-014 (line 100); OQ-W16-008(a) in open-questions.md (line 183)

BC-035 EC-014 reads: "(comment at host/mod.rs:72 references a planned implementation)". OQ-W16-008(a) repeats: "host/mod.rs:72 comment references a planned `internal.host_function_panic` implementation."

Actual source at host/mod.rs:71-75 is a 4-line struct-field doc comment for `internal_log: Option<Arc<crate::internal_log::InternalLog>>`. Line 72 names `internal.host_function_panic` because the field is *designed* to carry the log handle for that event class — not because emission is "planned" or "TODO". The actual evidence emission is unimplemented is the **absence** of any call site invoking `ctx.emit_internal(...)` with `INTERNAL_HOST_FUNCTION_PANIC` (verified: only the const declaration at internal_log.rs:83; zero call sites). Drift duplicated across BC-035 + open-questions.md (TD-VSDD-082 sibling-disclosure scope = 2 files).

**Recommendation:** Replace mis-citation with grep-confirmed call-site absence statement.

### MED-P41-002 — Pre-canonicalize "Panic semantics" paragraph cites infallible functions as panic vectors

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035 Panic semantics paragraph (line 54); EC-014

Per Rust std documentation:
- `Path::canonicalize` returns `io::Result<PathBuf>`. NUL-byte → `Err(EINVAL)`. ELOOP → Err. ENOENT → Err. ENAMETOOLONG → Err. **No documented panic vectors for filesystem inputs.**
- `Command::new(cmd)` is an **infallible builder** — returns `Command`, no Result. Does not panic on documented inputs.

BC-035's panic-semantics paragraph cites these as panic sources, which is type-system-incoherent. A panic from `Path::canonicalize` would be a stdlib bug, not a normal failure mode. Verified: NO `unwrap()` or `expect()` in BC-035's prescribed code path (canonicalize+allow-check+spawn chain).

**Recommendation:** Generalize panic-semantics paragraph to "any panic in the host-call body propagates to wasmtime as a Trap" without naming specific functions as panic sources.

### LOW-P41-003 — Off-by-one in BC-036 P5 stdin write_all span

**Severity:** LOW
**Location:** BC-036 P5 cites stdin write_all span as `:259-262`; actual code block spans 259-263 (closing brace at 263). Cosmetic.

### LOW-P41-007 — ETIMEDOUT not enumerated in Precedence Ladder step (2)

**Severity:** LOW
**Location:** BC-035 Ladder step (2). Precedence Ladder enumerates "path doesn't exist, NUL-containing path via EINVAL, or symlink loop" but doesn't include ETIMEDOUT (networked-filesystem slow paths). Contract is correct (any IO Err → CAPABILITY_DENIED) but example list is incomplete.

## 3. Verdict

**SUBSTANTIVE.** 0 HIGH + 2 MEDIUM + 2 LOW (transparency-disclosed). ADR-013 clock RESETS to 0_of_3.

## 4. Process-Gap Tagging

No `[process-gap]` tag warranted. P41-001 and P41-002 are sibling instances of the class addressed by TD-VSDD-081 NORMATIVE (mechanism-verification beyond string-presence-grep) and TD-VSDD-088 NORMATIVE (orchestrator routing). Mechanization filed as TD-VSDD-088-HOOK in `cycles/v1.0-brownfield-backfill/open-backlog-post-rc8.md`.

If P41-001/P41-002 class recurs in pass-43+ post-fix, escalate to `[process-gap]` and request immediate mechanization.

## 5. Source-of-Truth Verification Log

20 function citations audited; 17 MATCH; 1 NEAR-MATCH (off-by-one); 2 DRIFT (P41-001 line 72 mis-cite; P41-002 panic-vector mis-claim).

Files read in full: BC-1.05.035, BC-1.05.036, exec_subprocess.rs (463 lines), host/mod.rs (237 lines), memory.rs (113 lines), internal_log.rs (654 lines).

## 6. TD-VSDD Lesson Awareness (057-088 reviewed)

Confirmed reviewed. Pass-41 angle (type-signature-verification) NEW. Specific verification:
- TD-VSDD-085 NORMATIVE: every new EC in v1.37 has TV witness verified. EC-014 (panic) has no TV but is documented v1-not-testable in EC-014 + OQ-W16-008(a) — acceptable carve-out.
- TD-VSDD-087 NORMATIVE: BC-035 + BC-036 EC tables 3-column verified.
- TD-VSDD-088 NORMATIVE codification text reviewed at lessons.md:813-839; type claim "internal_log.rs:228 shows pub fn write returns ()" verified correct against source.

**Positive observation:** v1.37 prose evidences successful application of the TD-VSDD-088 routing pattern. The corrected internal_log.write description (P6, EC-010, TV row 11) is accurate at the type-system level — direct evidence that PO-authored content with source-of-truth verification produces better content than prior orchestrator-pre-designed prose.
