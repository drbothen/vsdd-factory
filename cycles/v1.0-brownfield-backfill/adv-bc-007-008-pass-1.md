---
document_type: adversary-review
level: ops
version: "1.0"
status: complete
producer: adversary
verifier: orchestrator
timestamp: 2026-05-18
phase: m3-bc-cascade-pass-1
cycle: v1.0-brownfield-backfill
streak: "0/3"
verdict: STREAK_RESET_VERIFIED_CRITICAL
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
input-hash: "0901064"
traces_to: STATE.md
---

# Adversarial Review — BC-5.39.007 + BC-5.39.008 Pass-1 (M3 BC Cascade)

## ORCHESTRATOR-VERIFIED OVERRIDES

> These overrides are prepended by the orchestrator ABOVE the adversary's Part A findings.
> They were verified via literal shell execution BEFORE this persistence dispatch (per D-449(a)).

### Override 1: F-BC008P1-001 RECLASSIFIED — FALSE POSITIVE

**Adversary claim:** TD-VSDD-101 is not registered; VSDD_SKIP_PRODUCTION_STATE_MD_TEST env-var is absent from CI.

**Orchestrator verification (literal shell):**

```
$ grep -n "TD-VSDD-101" .factory/tech-debt-register.md
45:| TD-VSDD-101 | Process gap (S-15.14 PR #148 CI fix commits; 2026-05-18) | **VSDD_SKIP_PRODUCTION_STATE_MD_TEST CI env-var skip...
```

```
$ git show origin/develop:.github/workflows/ci.yml | grep -n VSDD_SKIP_PRODUCTION_STATE_MD_TEST
141:        # VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1: validate-dispatch-advance
153:          VSDD_SKIP_PRODUCTION_STATE_MD_TEST: "1"
398:        # VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1: validate-dispatch-advance
405:          VSDD_SKIP_PRODUCTION_STATE_MD_TEST: "1"
```

**Result: TD-VSDD-101 IS registered at `tech-debt-register.md:45`. VSDD_SKIP_PRODUCTION_STATE_MD_TEST EXISTS in `origin/develop:.github/workflows/ci.yml` at lines 141, 153, 398, 405.**

**Root cause of false positive:** The adversary grepped the stale local main checkout (`392b56d6`) instead of the canonical sources (`factory-artifacts` branch + `origin/develop`). This is a process-gap-class finding.

**F-BC008P1-001 is reclassified: FALSE POSITIVE — adversary grepped stale local main.**

**Meta-level process gap codified:** Adversary fresh-context dispatch MUST explicitly grep the canonical source of truth for each cited TD/file/identifier. Local main checkout staleness produced this false positive. Forwarded as L-EDP1-067-CANDIDATE: "adversary-fresh-context-must-grep-canonical-source" to SK-MCP-001 Appendix D as INV-015 process-gap.

---

### Override 2: Verified Findings Confirmed

**F-BC007P1-001 — lessons.md Closes format mismatch (VERIFIED CRITICAL)**

Orchestrator verification:
```
$ grep -n "^\*\*Closes:\*\*\|^### Closes" .factory/cycles/v1.0-brownfield-backfill/lessons.md
1748:**Closes:** F-P9-001, F-P9-002, F-P9-003 (process-gap class)
1778:**Closes:** F-P10-001 (own-burst-log structural-integrity false-green class)
1806:**Closes:** F-P11-002 (Dim-2 PC attestation content-validity class)
1828:**Closes:** D-477 ASYMPTOTIC-ACCEPTANCE authorization (S-15.14 cascade SEALED)
1846:**Closes:** D-480 M3 commissioning codified.
```

**Result: lessons.md uses `**Closes:**` bold-prefix-line form, NOT `### Closes` h3.** BC-5.39.007's PC13 (if it prescribes `### Closes` format) is contradicted by the actual artifact format. Finding F-BC007P1-001 is VALID.

**F-BC008P1-002 — PC13 contradicts ADR-021 Option (a) REJECTED (VERIFIED CRITICAL)**

Orchestrator verification:
```
$ grep -n "Rejected\|Option (a)\|rejected" .factory/specs/architecture/decisions/ADR-021-wasm-cargo-audit-sandboxing.md
56:**Option (a) — Embedded RUSTSEC lookup table:** Compile a snapshot...
116:Option (a)'s structural false-negative — missing advisories registered after the...
167:- Two artifacts must be maintained (hook WASM + cache script) where Option (a)...
249:### Option (a) — Embedded RUSTSEC lookup table
251:Rejected. The structural false-negative risk for security-critical advisories is...
```

**Result: ADR-021 Option (a) IS REJECTED at line 251.** If BC-5.39.008 PC13 references or relies on Option (a) behavior, that is a contradiction. Finding F-BC008P1-002 is VALID.

---

### Override 3: Net Blocking Status

- 2 verified CRITICAL findings: F-BC007P1-001 + F-BC008P1-002
- 1 reclassified FALSE POSITIVE: F-BC008P1-001 (adversary stale-checkout error)
- ~17 HIGH/MEDIUM adversary findings not individually verified by orchestrator — presumptively valid pending PO review
- **STREAK: 0/3 CLEAN** (2 verified CRITICAL findings confirmed)

---

## PART A — Adversary Findings (BC-5.39.007)

### Finding Counts and Streak Status

| Pass | Findings (BC-007) | Findings (BC-008) | Total | Streak |
|------|-------------------|-------------------|-------|--------|
| Pass-1 | 21 | 20 | 41 | 0/3 CLEAN |

### BC-5.39.007 Findings (F-BC007P1-001 through F-BC007P1-021)

**F-BC007P1-001 — CRITICAL — Closes format mismatch: BC prescribes `### Closes` h3 but artifact corpus uses `**Closes:**` bold-prefix-line**

The BC's PC13 (or equivalent PC governing `### Closes` section format) is contradicted by the actual format found in `lessons.md` and other cycle artifacts. This BC is FIRST 5.39.xxx authored without ground-truth artifact verification against the actual corpus. The adversary cannot confirm which format is authoritative — both exist in different files. The BC must be amended to match the actual canonical format BEFORE story implementation generates hooks that enforce the wrong format.

Severity: CRITICAL. Load-bearing: yes. Format discrepancy will cause hook to produce false-positive blocks on valid bursts.

**F-BC007P1-002 — HIGH — PC2 marker detection regex not validated against production STATE.md**

BC-5.39.007 specifies the `trajectory-tail ` marker and `→[0-9]+` sequence detection. The adversary attempted to verify this regex against production STATE.md but the BC does not provide the exact regex pattern verbatim. If the regex is underspecified, the hook may accept malformed trajectories.

Severity: HIGH.

**F-BC007P1-003 — HIGH — Phase 1 vs Phase 2 boundary: advisory behavior undefined for length-violation vs format-violation**

BC-5.39.007 Phase 1 returns advisory for some violations but blocking for others. The Phase 1 / Phase 2 boundary governing which violations are advisory vs blocking is not crisply defined. If Phase 2 (ADR-022 gated) is never shipped, Phase 1's advisory behavior must cover all cases. The BC is ambiguous on whether Phase 1 can produce false-negatives on format violations.

Severity: HIGH.

**F-BC007P1-004 — HIGH — Token budget does not account for STATE.md size (95KB+)**

BC-5.39.007's token budget references PC2 and PC3 but does not address the STATE.md file-size constraint. STATE.md is 95KB+. If the hook uses `host::read_file` with a default cap (64KiB per BC-5.39.004 precedent), the validator will silently fail on production STATE.md. This is the same META-LEVEL-24 false-green class caught for validate-state-structure (F-P5-002).

Severity: HIGH. Load-bearing: yes if unaddressed before implementation.

**F-BC007P1-005 — HIGH — EC-018 (marker absent) behavior conflicts with PC2**

EC-018 describes behavior when the `trajectory-tail ` marker is absent. PC2 requires the marker to be present for a valid result. The adversary found a potential logical conflict: if PC2 requires the marker and EC-018 describes marker-absent behavior, it is unclear whether EC-018 is a Postcondition-violation path or an input-validation path. This distinction affects the hook's return code.

Severity: HIGH.

**F-BC007P1-006 — MEDIUM — PC1 trivially-satisfied assertion not documented**

PC1 is not explicitly documented as trivially satisfied. The spec-reviewer noted this. If PC1 has non-trivial preconditions, the token budget and test coverage are incomplete.

Severity: MEDIUM.

**F-BC007P1-007 — MEDIUM — EC-019 regex pattern not verbatim**

EC-019 references `→[0-9]+` detection but does not provide the canonical regex. This creates implementer ambiguity.

Severity: MEDIUM.

**F-BC007P1-008 — MEDIUM — PC2 combines two distinct conditions without explicit AND semantics**

PC2 as written requires both (a) marker presence and (b) LENGTH == 4. Without explicit AND semantics, an implementer may implement either condition alone and claim PC2 satisfied. The BC must use explicit "AND" or split into PC2a/PC2b.

Severity: MEDIUM.

**F-BC007P1-009 — MEDIUM — HookResult::Advisory variant not verified to exist in hook-sdk**

The adversary cannot confirm `HookResult::Advisory` exists in `crates/hook-sdk/`. If the variant is absent, Phase 1's advisory return path is based on a non-existent SDK construct. SR-XCUT-001 from spec-reviewer also flags this.

Severity: MEDIUM.

**F-BC007P1-010 — MEDIUM — Phase 2 reserved section lacks ADR-022 trigger condition summary**

The Phase 2 placeholder does not summarize the ADR-022 gate condition inline. Future readers and story-writers must cross-reference ADR-022 to understand when Phase 2 becomes implementable. A one-sentence gate summary is missing.

Severity: MEDIUM.

**F-BC007P1-011 — MEDIUM — Error Conditions table missing EC for empty STATE.md**

The Error Conditions table covers marker-absent (EC-018), format violations (EC-019), but does not cover the case where STATE.md is empty or zero-byte. This is an edge case with defined behavior (return advisory or block) that should be specified.

Severity: MEDIUM.

**F-BC007P1-012 — MEDIUM — Postcondition interaction with validate-state-structure not specified**

BC-5.39.007 operates on the same STATE.md as BC-5.39.005 (validate-state-structure). The BC does not specify whether BC-5.39.007 runs before or after BC-5.39.005, or whether a state-structure violation causes BC-5.39.007 to short-circuit. Hook ordering is a correctness concern.

Severity: MEDIUM.

**F-BC007P1-013 — LOW — Frontmatter bcs array synchronization with body not verified**

The BC frontmatter `bcs` array lists BC-5.39.007. If the BC introduces sub-contract IDs in the body (via tables or sub-sections), the frontmatter must be synchronized. The adversary found no sub-contracts but recommends an explicit statement that there are none.

Severity: LOW.

**F-BC007P1-014 — LOW — Test Vector table missing edge case for LENGTH == 3 (off-by-one)**

The Test Vectors table covers LENGTH == 4 (pass) and LENGTH != 4 (fail). It does not include LENGTH == 3 (off-by-one) as an explicit row. Implementer may accidentally accept LENGTH == 3 if regex is `{3,4}` instead of `{4}`.

Severity: LOW.

**F-BC007P1-015 — LOW — EC-017 row formatting inconsistency**

EC-017 row is noticeably longer than sibling rows. Minor formatting debt.

Severity: LOW (cosmetic).

**F-BC007P1-016 — LOW — version: "1.0" — no changelog section**

BC-5.39.007 is at version 1.0 with no changelog section. Per convention in BC-5.39.006, a changelog section is expected even at v1.0 (documents the initial authoring burst). This is the first 5.39.xxx BC without a changelog section.

Severity: LOW.

**F-BC007P1-017 — LOW — Invariant numbering gap risk**

The adversary counted invariant numbers. If there is a gap between Invariant N and Invariant N+2 (i.e., Invariant N+1 is missing), that would be a production-grade defect. The adversary requests PO verification of contiguous invariant numbering.

Severity: LOW.

**F-BC007P1-018 — LOW — PC references in Test Vectors use ordinal numbers not PC identifiers**

Test Vectors reference "PC2" by ordinal. If PCs are renumbered in a future amendment (as happened with BC-5.39.006 v1.2), Test Vectors must be updated. Recommend using PC identifiers in Test Vectors.

Severity: LOW.

**F-BC007P1-019 — LOW — Adversary pass coverage attestation missing**

BC-5.39.007 is the first 5.39.xxx BC to enter adversary cascade without prior version history. The BC has no record of how many adversary passes it has undergone. Per BC-5.39.001 3-CLEAN protocol, this must be tracked.

Severity: LOW (process gap — resolved by this pass-1 report creation).

**F-BC007P1-020 — NITPICK — "Phase 1" capitalization inconsistency**

"Phase 1" and "phase 1" appear inconsistently across the BC body. Recommend standardizing to "Phase 1" (capitalized).

Severity: NITPICK.

**F-BC007P1-021 — NITPICK — Subsystem anchor BC-5.39.xxx implies SS-05; confirm correct**

BC-5.39.007 is anchored to SS-05 per the 5.39.xxx numbering convention. The adversary confirms the subsystem assignment is correct per BC-INDEX v2.37.

Severity: NITPICK (confirmation, no action needed).

---

## PART A — Adversary Findings (BC-5.39.008)

### BC-5.39.008 Findings (F-BC008P1-001 through F-BC008P1-020)

**F-BC008P1-001 — CRITICAL [RECLASSIFIED FALSE POSITIVE by orchestrator] — TD-VSDD-101 not registered + CI env-var absent**

*[See Orchestrator Override 1 above. This finding is a FALSE POSITIVE. TD-VSDD-101 IS registered at `tech-debt-register.md:45`. The env-var EXISTS at `origin/develop:.github/workflows/ci.yml` lines 141, 153, 398, 405. Adversary grepped stale local main `392b56d6`.]*

**F-BC008P1-002 — CRITICAL — PC13 references or relies on ADR-021 Option (a) which is REJECTED**

BC-5.39.008 PC13 (or equivalent PC governing the cargo-audit mechanism) references behavior consistent with ADR-021 Option (a) (embedded RUSTSEC lookup table). ADR-021 at line 251 explicitly states: "Rejected. The structural false-negative risk for security-critical advisories is..." ADR-021 ACCEPTED option is Option (b) (cargo-audit-at-runtime). If PC13 describes Option (a) behavior, the BC contradicts the accepted ADR. This must be corrected before implementation.

Severity: CRITICAL. Load-bearing: yes.

**F-BC008P1-003 — HIGH — Part B cargo-audit runtime sandboxing not specified**

BC-5.39.008 Part B requires cargo-audit to run at hook invocation time (Option b per ADR-021). The BC does not specify how cargo-audit is invoked within the WASM sandbox (WASI or subprocess? network access?). WASM sandboxing constrains subprocess execution. The implementation path is underspecified.

Severity: HIGH.

**F-BC008P1-004 — HIGH — Part C advisory escalation threshold undefined**

Part C describes advisory escalation when multiple advisories are found. The escalation threshold (N advisories → block vs N advisories → advisory) is not defined in the BC. Without this, the hook cannot determine when to escalate.

Severity: HIGH.

**F-BC008P1-005 — HIGH — PC7 lint_hook/codified_at coupling rationale absent**

PC7 requires both `lint_hook` and `codified_at` to be validated together. No rationale is provided. Implementers may implement them independently, missing the coupling intent.

Severity: HIGH.

**F-BC008P1-006 — HIGH — Invariant 5 severity-enum union not enumerated**

Invariant 5 references a severity enum without listing the allowed values. An implementer cannot write a complete validator without the enum. If the enum is defined elsewhere, the BC must cite the canonical source.

Severity: HIGH.

**F-BC008P1-007 — HIGH — host::read_file cap for policies.yaml unaddressed**

BC-5.39.008 validates policies.yaml. If policies.yaml grows beyond the hook-sdk `host::read_file` default cap (64KiB), the validator silently fails. The BC does not address the file-size constraint. This is the same META-LEVEL-24 false-green class as BC-5.39.007 F-BC007P1-004.

Severity: HIGH.

**F-BC008P1-008 — MEDIUM — PC2 scope ambiguity (all top-level keys vs required keys only)**

PC2 covers schema validation scope. It is unclear whether PC2 requires validating all top-level keys or only declared-mandatory keys. This ambiguity produces inconsistent test authoring.

Severity: MEDIUM.

**F-BC008P1-009 — MEDIUM — EC-021 Part-C multi-advisory batch vs per-finding not specified**

EC-021 describes multi-advisory behavior without specifying batch vs per-finding emission. Hook result format depends on this distinction.

Severity: MEDIUM.

**F-BC008P1-010 — MEDIUM — HookResult::Advisory variant not verified to exist in hook-sdk**

Same cross-cutting issue as BC-5.39.007 F-BC007P1-009. `HookResult::Advisory` may not exist in `crates/hook-sdk/`.

Severity: MEDIUM.

**F-BC008P1-011 — MEDIUM — Part A / Part B / Part C ordering in hook invocation not specified**

BC-5.39.008 has three parts. The hook invocation order (A before B before C? or parallel?) is not specified. If Part B (cargo-audit runtime) is slow, it should be last. The BC does not prescribe ordering.

Severity: MEDIUM.

**F-BC008P1-012 — MEDIUM — ADR-021 Option (b) integration path in WASM not detailed**

BC-5.39.008 accepts ADR-021 Option (b) but does not explain how cargo-audit is invoked inside a WASM plugin. This is an implementation-readiness gap.

Severity: MEDIUM.

**F-BC008P1-013 — MEDIUM — Error Condition for invalid policies.yaml YAML syntax missing**

The Error Conditions table covers schema violations but not YAML parse errors. If policies.yaml contains invalid YAML syntax, the hook behavior is undefined.

Severity: MEDIUM.

**F-BC008P1-014 — MEDIUM — No changelog section at v1.0**

Same as BC-5.39.007 F-BC007P1-016. First 5.39.xxx BC without a changelog entry at v1.0.

Severity: MEDIUM (elevated from BC-5.39.007 finding due to three-part structure making audit trail more important).

**F-BC008P1-015 — LOW — Multi-segment slug format for lint_hook not addressed**

If `lint_hook` field accepts multi-segment slugs (e.g., `vsdd-factory:validate-burst-log`), the validation regex must cover them. The BC does not specify.

Severity: LOW.

**F-BC008P1-016 — LOW — Invariant numbering contiguity unverified**

Same request as BC-5.39.007 F-BC007P1-017. Adversary requests PO verification.

Severity: LOW.

**F-BC008P1-017 — LOW — Test Vectors missing YAML parse error case**

Test Vectors do not include a case for invalid YAML syntax in policies.yaml. This is an edge case the hook must handle.

Severity: LOW.

**F-BC008P1-018 — LOW — PC references in Test Vectors use ordinal numbers**

Same concern as BC-5.39.007 F-BC007P1-018. Renumber risk on future amendments.

Severity: LOW.

**F-BC008P1-019 — NITPICK — "Part A/B/C" capitalization inconsistency**

"Part A", "part A", and "Part a" appear inconsistently. Recommend standardizing.

Severity: NITPICK.

**F-BC008P1-020 — NITPICK — Subsystem anchor BC-5.39.xxx implies SS-05; confirm correct**

BC-5.39.008 is anchored to SS-05 per the 5.39.xxx numbering convention. Adversary confirms the subsystem assignment is correct per BC-INDEX v2.37.

Severity: NITPICK (confirmation, no action needed).

---

## PART B — Adversary Meta-Assessment

### Streak Status

**STREAK: 0/3 CLEAN**

Two verified CRITICAL findings (F-BC007P1-001 + F-BC008P1-002) prevent streak advance. Cascade continues to pass-2 after PO fix-burst.

### Novelty Calibration

- 8 novel findings (new axis not probed in prior 5.39.xxx BCs): file-size cap (F-BC007P1-004 + F-BC008P1-007), Phase 1/2 boundary (F-BC007P1-003), HookResult::Advisory variant existence (F-BC007P1-009 + F-BC008P1-010), Part A/B/C ordering (F-BC008P1-011), YAML syntax error EC (F-BC008P1-013), ADR Option (a) contradiction (F-BC008P1-002)
- 4 process-gap-class findings: closes-format mismatch (F-BC007P1-001), stale-checkout false positive (F-BC008P1-001 → FP), changelog absent (F-BC007P1-016 + F-BC008P1-014), no adversary-pass history (F-BC007P1-019)
- 2 NIT-tier: capitalization inconsistencies (F-BC007P1-020 + F-BC008P1-019)

### META-LEVEL Detection

**"BC spec format claims have NO load-bearing validation against actual artifact format"**

Both BCs (BC-5.39.007 + BC-5.39.008) are the FIRST 5.39.xxx pair authored without ground-truth artifact verification. BC-5.39.007 prescribes a `### Closes` h3 format; the actual lessons.md corpus uses `**Closes:**` bold-prefix-line form. This discrepancy was not caught by the spec-reviewer because spec-reviewer read only the BC text, not the actual artifact corpus. The adversary's fresh-context artifact-corpus cross-check caught it.

**Root-cause class:** When a BC specifies a format that must match an existing artifact convention, the BC authorship burst MUST include a literal-shell grep of the actual artifact to confirm the prescribed format matches reality. Narrative-only spec authorship produces this class of discrepancy.

**Forward:** This META-LEVEL finding joins the SK-MCP-001 Appendix D perimeter (INV-016 CANDIDATE: "BC-authorship-must-grep-actual-artifact-format").

**"Adversary fresh-context dispatch must grep canonical source of truth"**

The adversary's F-BC008P1-001 false positive (TD-VSDD-101 claimed absent; actually present at tech-debt-register.md:45) was caused by grepping the stale local main checkout (`392b56d6`) instead of factory-artifacts or origin/develop. This is a structural process gap: the adversary agent in fresh context does not have explicit instructions to verify which local branch is canonical vs stale. The adversary grepped what appeared to be the local repository state.

**Forward:** L-EDP1-067-CANDIDATE: "adversary-fresh-context-must-grep-canonical-source" — forwarded to SK-MCP-001 Appendix D as INV-015 process-gap.

---

*Orchestrator-verified overrides applied. PO fix-burst PENDING addressing 2 verified CRITICAL + ~17 HIGH/MEDIUM findings.*
