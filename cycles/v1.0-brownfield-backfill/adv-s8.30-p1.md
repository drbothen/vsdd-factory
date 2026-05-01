---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.30-sdk-extension-hookpayload-subagentstop-fields.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.012.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.011.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - crates/hook-sdk/src/payload.rs
  - crates/hook-sdk/HOST_ABI.md
  - plugins/vsdd-factory/hooks/handoff-validator.sh
  - plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
  - plugins/vsdd-factory/hooks/track-agent-stop.sh
input-hash: "a030493"
traces_to: prd.md
pass: p1
previous_review: none
story_id: "S-8.30"
pass_number: 1
story_version: "1.0"
story_input_hash: "a030493"
target: story
target_file: .factory/stories/S-8.30-sdk-extension-hookpayload-subagentstop-fields.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 3
findings_medium: 4
findings_low: 5
findings_nit: 1
---

# Adversarial Review Pass-1 — S-8.30 v1.0

## Finding ID Convention

`F-S830-P1-NNN` (zero-padded sequence within this pass).

## Part A — Pass-0 Fix Verification

**N/A for pass-1.** First adversarial pass on S-8.30 v1.0 (fresh spec, no prior reviews). No prior fixes to verify.

## Part B — New Findings (Pass-1)

### HIGH

#### F-S830-P1-001 — AC-5 mis-anchors Invariant 1 (HOST_ABI_VERSION) instead of Invariant 4 (field names canonical)

- **Location:** S-8.30:142, 256
- **Evidence:**
  - Trace cell line 142: `AC-5 (Inv 1)`
  - AC-5 body trace footer line 256: "(traces to BC-2.02.012 invariant 1 — fields are canonical and immutable; round-trip preserves them)"
  - BC-2.02.012:57 Invariant 1: HOST_ABI_VERSION = 1 unchanged
  - BC-2.02.012:60 Invariant 4: Field names are canonical and immutable
- **Why this fails:** AC-5 asserts round-trip preservation of field VALUES through serde — does NOT exercise HOST_ABI_VERSION. Semantics described in parenthetical map to Invariant 4, but story labels as Invariant 1. POLICY 4 mis-anchor; would mislead implementer.
- **Fix:** Change `(Inv 1)` → `(Inv 4)` in trace cell + update AC-5 body footer to "invariant 4 — field names canonical and immutable; round-trip preserves field values".

#### F-S830-P1-002 — AC-7 trace cell cites Invariant 4 phantom; AC-7 doesn't exercise it

- **Location:** S-8.30:142, 279
- **Evidence:**
  - Trace cell: `AC-7 (Inv 1, 4)`
  - AC-7 body footer: "(traces to BC-2.02.012 invariant 1 and BC-2.01.003 invariant 1)"
  - AC-7 content: only HOST_ABI_VERSION grep checks — no field-name assertion
- **Why this fails:** Trace cell adds phantom Inv 4. AC-7's grep does not enforce field names. Cell and body disagree.
- **Fix:** Either (a) drop `, 4` → `AC-7 (Inv 1)`, OR (b) add a sub-step to AC-7 grep'ing field names in payload.rs. Option (a) is the lighter fix.

#### F-S830-P1-003 — EC-008 internal contradiction about jq vs Rust empty-string semantics

- **Location:** S-8.30:299
- **Evidence (verbatim):** "Note: this diverges from jq's `//` operator, which advances on `null` and `false` but NOT on empty string. Both Rust and jq behave the same here: an empty string does not advance the fallback."
- **Why this fails:** Sentence 1 sets up "this diverges", sentence 2 reverses to "Both behave the same." Ground truth: both treat empty string identically (no advance). Sentence 1 wrong.
- **Fix:** Rewrite to: "Both jq's `//` operator and Rust's `Option::or` chain behave the same here: an empty string does NOT advance the fallback. (jq's `//` advances on `null` and `false` only; Rust's `or` advances on `None` only.)"

### MEDIUM

#### F-S830-P1-004 — SS-02 architecture doc fallback-chain example diverges from BC-2.02.012 canonical chain

- **Location:** SS-02-hook-sdk.md:242-248 vs BC-2.02.012:51-52
- **Evidence:**
  - SS-02 uses consuming `payload.agent_type.or(payload.subagent_name).unwrap_or_else(|| "unknown".to_string())` (returns String)
  - BC-2.02.012 uses borrowing `.as_deref().or(...).unwrap_or("unknown")` (returns &str)
  - BC Invariant 5 mandates story specs use the BC's canonical chain
- **Why this fails:** Cross-document drift. Architecture doc itself violates BC normativity. S-8.30 follows BC correctly; SS-02 is the divergent doc. Downstream stories reading SS-02 first may pick wrong pattern.
- **Fix:** Update SS-02 lines 242-248 to use BC-2.02.012's canonical `as_deref()` chain. (Architect scope.)

#### F-S830-P1-005 — track-agent-stop `// ""` vs other hooks `// empty` divergence not explained

- **Location:** S-8.30:78-84, 126
- **Evidence:**
  - Goal section quotes only `// empty` chain
  - Empirical Anchors table line 126: track-agent-stop uses `// ""` (verified at track-agent-stop.sh:23)
  - Other 2 non-handoff hooks use `// empty`
- **Why this fails:** Three of four hooks use `// empty`; one uses `// ""`. Both yield empty string under `jq -r`, so Rust's `unwrap_or("")` matches both. Story doesn't explain — implementer porting S-8.03 may wonder if different Rust chain needed.
- **Fix:** Add note: "track-agent-stop.sh:23 uses `// \"\"` where other three hooks use `// empty`. In `jq -r` mode both yield empty-string stdout, matching Rust's `unwrap_or(\"\")`."

#### F-S830-P1-006 — AC-8 "no panic, no error" framing borderline tautological + "Stop" event type unverified

- **Location:** S-8.30:282
- **Evidence:** "Deserialization succeeds (no panic, no error) for fixture envelopes representing each of: PreToolUse, PostToolUse, SessionStart, SessionEnd, Stop, SubagentStop. For all non-SubagentStop event types, all four new fields are None."
- **Why this fails:** "No panic, no error" alone is POLICY 11 tautology. Second sentence rescues with field assertions, but framing leads with weak check. Plus "Stop" event type not enumerated in BC, SS-02, or any cited bash hook — existence in Claude Code plausible but unverified.
- **Fix:** Reorder AC-8 to lead with substantive field assertion: "For each event type (PreToolUse, PostToolUse, SessionStart, SessionEnd, SubagentStop), construct fixture envelope, deserialize, then assert: (a) deserialization is `Ok`, AND (b) for non-SubagentStop event types, all four new fields are `None`." Remove `Stop` from enumeration unless cited (defer Stop to a follow-up if Claude Code OTel docs confirm it's a real event with relevant fields).

#### F-S830-P1-007 — Trace-cell phrasing "Post 7, Inv 2" diverges from body footer "postcondition 7 and invariant 2"

- **Location:** S-8.30:142, 191
- **Why this fails:** Compressed table notation vs longer body-footer phrasing. Tooling-friendly trace strings should be canonical across cell and footer.
- **Fix:** Standardize trace-cell to body-footer phrasing.

### LOW

#### F-S830-P1-008 — T-1 invariant/postcondition counts verification (counts match)

- **Location:** S-8.30:305
- **Status:** No fix needed; counts verified consistent.

#### F-S830-P1-009 — Tasks Summary table column heading "Estimate" non-uniform with row content

- **Location:** S-8.30:451-458
- **Disposition:** SKIP-FIX-eligible per S-7.03.

#### F-S830-P1-010 — BC-2.02.012 line count "153" vs actual 159

- **Location:** S-8.30:454, 469
- **Disposition:** SKIP-FIX-eligible (estimates).

#### F-S830-P1-011 — wave: 15 [process-gap] note has no W-15 schedule artifact link

- **Location:** S-8.30:12
- **Disposition:** SKIP-FIX-eligible.

#### F-S830-P1-012 — verification_properties: [] [process-gap] OK per project convention

- **Location:** S-8.30:19
- **Disposition:** No fix; convention-compliant.

### NIT

#### F-S830-P1-013 — OQ-3 RESOLVED claim verified across cross-references

- **Location:** S-8.30:64
- **Disposition:** No fix; verified clean.

## Open Questions

- **OQ-A1:** Should AC-7 also grep field names (extend to enforce Inv 4 in CI)?
- **OQ-A2:** Is "Stop" event type a real Claude Code event with relevant fields, or fictional in this enumeration?
- **OQ-A3:** Will S-8.01's `output` 3rd-arm need BC-2.02.012 amendment or story-specific divergence?

## Pass-2 Priors

1. Verify HIGH fixes (F-001, F-002, F-003) propagate to body footers AND trace cells.
2. Verify EC-008 contradiction resolved.
3. Verify SS-02 cross-doc parity (architect scope).
4. Verify AC-8 reordering with field-first assertion + "Stop" disposition.
5. Verify HOST_ABI_VERSION grep still passes in pass-2.
6. Re-confirm POLICY 1, 6, 7, 8 compliance.

## Verdict

**SUBSTANTIVE** — clock 0/3.

3 HIGH (all anchor-integrity / contradiction defects), 4 MED, 5 LOW, 1 NIT. Story is structurally complete and BC-aligned but has anchor-precision defects that mislead implementers about which BC clauses each AC enforces. v1.1 revision burst required.

## Trajectory

| Pass | C | H | M | L | NIT | Total |
|------|---|---|---|---|-----|-------|
| p1 | 0 | 3 | 4 | 5 | 1 | 13 |

## Novelty Assessment

Fresh-spec pass-1; all findings novel. Story is well-structured (follows S-8.10's template). Most defects are anchor-precision and prose clarity — no structural rewrite needed. Surgical edits resolve HIGH findings.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 4 |
| LOW | 5 |
| NIT | 1 |

**Overall Assessment:** SUBSTANTIVE. Story v1.0 is structurally complete and BC-aligned. Three HIGH findings cluster around BC clause cross-referencing precision (POLICY 4 mis-anchors). Four MED findings address cross-doc drift, hook divergence not explained, AC-8 framing risk, trace-cell phrasing. POLICY 1, 6, 7, 8, process-gap-D-183-A, 11 (mostly): VERIFIED.

**Convergence:** Not reached. Recommend v1.1 revision burst on HIGH + MED before pass-2.

**Readiness:** NOT YET READY. Story-writer must address F-001/002/003 (HIGH) before next pass. F-004 requires architect coordination (SS-02 cross-doc fix). MED findings should ideally land in same revision.
