# Adversarial Review — Pass 42 (E-9 v1.7 amendment surface, epic v1.38 sealed at 0831db6)

## 1. Angle (NEW per TD-VSDD-057)

**Partial-fix regression discipline (S-7.01) audit at the seam between changed and unchanged sections.** Methodology: identify what D-283 (v1.36→v1.37) and D-284 (v1.37→v1.38) actually changed, then verify whether sibling sections those bursts did NOT touch remain coherent with the changes that DID land. Differs from 41 prior angles by explicitly framing: "what did these bursts intentionally NOT touch, and does that untouched surface still cohere with what they DID touch?"

## 2. Findings

### MED-P42-001 — BC-035 EC-004 not refreshed when Postcondition 3 introduced binary_canonicalize_failed emission

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035.md line 90 (EC-004)

EC-004 (untouched by v1.37/v1.38) reads "`cmd` binary does not exist on disk | `canonicalize()` fails with IO error; returns `CAPABILITY_DENIED` (-1)" — does not mention emit_denial. Postcondition 3 (touched in v1.37) explicitly normatively requires `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` for ALL canonicalize-failure cases. Sibling ECs that WERE updated (EC-005 NUL byte, EC-006 generic IO error, EC-008 ELOOP, EC-010 ENAMETOOLONG, EC-011 empty string) all explicitly cite the emit. EC-004 — the FOUNDATIONAL example explicitly named in P3's "(NOTE: This is a BEHAVIOR CHANGE..." discussion — was left in pre-v1.37 terse framing. Textbook S-7.01 partial-fix regression.

### MED-P42-002 — BC-036 P5 narrative says "4 denial paths" but EC-003 enumerates 5 (incl. binary_canonicalize_failed)

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.036.md line 52 (P5) vs line 88 (EC-003)

P5: "Today only `internal.capability_denied` is emitted on the 4 denial paths (`no_exec_subprocess_capability` per :148, `binary_not_on_allow_list` per :155, `shell_bypass_not_acknowledged` per :162, `setuid_or_setgid_binary` per :169)." But EC-003 (updated v1.37) enumerates 5: 4 + `binary_canonicalize_failed` per BC-035 P3. P5's "4 denial paths" lead enumeration was not refreshed in same burst.

### MED-P42-003 — BC-035 §Related BCs cross-references "BC-036 EC-006" for "canonicalized full path" annotation that lives in BC-036 P2

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035.md line 65

BC-035 line 65 says: "BC-1.05.036 EC-006 declares the success-path event payload `binary` field as 'canonicalized full path'." But BC-036 EC-006 just lists field types; "canonicalized full path" annotation lives in BC-036 P2 (`binary: String /* canonicalized full path */`). Stale cross-reference target.

### LOW-P42-001 — BC-036 EC-012 + EC-014 lack TV witnesses (TD-VSDD-085 scope clarification)

**Severity:** LOW

EC-012 (cwd_allow unenforced) and EC-014 (env_allow names absent silent omission) are NEW silent-no-op ECs introduced in v1.37/D-283 with no TV witnesses. Whether TD-VSDD-085 NORMATIVE scope covers silent-no-op ECs is interpretive — codification body says "any new emit_denial reason string introduced in a BC body MUST appear in at least one row of the BC's §Canonical Test Vectors table" (narrow); adversary axis says "For each new normative Edge Case row or new mechanism string introduced in this burst, grep the BC's §Canonical Test Vectors for a row witnessing it" (broad).

### LOW-P42-002 — BC-035 P1 trailer redundant with P2 lead

**Severity:** LOW (cosmetic per S-7.03 SHIP-AS-IS)

P1 ends with "If `read_wasm_string` returns Err (non-UTF-8 in WASM memory), the existing host-call error path returns INVALID_ARGUMENT (-4) BEFORE any canonicalize attempt." P2 begins with the same statement (verbatim except for added NOTE). Redundancy.

## 3. Verdict

**SUBSTANTIVE.** 0 HIGH + 3 MEDIUM + 2 LOW. ADR-013 clock RESETS to 0_of_3.

## 4. Process-Gap Tagging

No process-gap findings. All 3 MED findings are content defects fixable in-place by a PO authoring burst. The TD-VSDD-076 / TD-VSDD-079 / TD-VSDD-082 lessons are already codified NORMATIVE; the failure mode caught here is sibling-sweep INCOMPLETENESS in the v1.37 PO authoring burst, not absence of the codified rule. If MED-P42-001/002 patterns recur in v1.39→v1.40 (third occurrence of "P-edit landed but EC sibling not refreshed"), would warrant codification of "PO authoring prompt MUST explicitly invoke TD-VSDD-076 sibling-grep before commit."

## 5. Source-of-Truth Verification Log

20+ source-claim citations re-verified. All MATCH. 1 cite-span tolerance (emit_denial fn def cited :304-309; actual closing brace :310 — within tolerance).

## 6. TD-VSDD Lesson Awareness

Reviewed TD-VSDD-057 through TD-VSDD-088. All 3 MED findings are net-new instances of the partial-fix regression class at the seam between changed/unchanged sections — not recapitulating closed findings. The class itself is addressed by TD-VSDD-076 NORMATIVE (intra-document sibling-sweep) and TD-VSDD-082 NORMATIVE (bidirectional-sibling-disclosure) but pass-42 surfaces NEW instances that prior 41 angles did not catch.
