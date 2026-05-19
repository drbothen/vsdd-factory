---
document_type: adversary-review
level: ops
version: "1.0"
status: complete
producer: adversary
verifier: orchestrator
timestamp: 2026-05-18
phase: m3-bc-cascade-pass-2
cycle: v1.0-brownfield-backfill
streak: "0/3"
verdict: CRITICAL
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
input-hash: "abe34e3"
traces_to: STATE.md
---

# Adversarial Review — BC-5.39.007 + BC-5.39.008 Pass-2 (M3 BC Cascade)

## ORCHESTRATOR-VERIFIED OVERRIDES

> These overrides are prepended by the orchestrator ABOVE the adversary's Part A findings.
> They were verified via literal shell execution BEFORE this persistence dispatch (per D-449(a)).

### Override 1: F-BC008P2-001 VERIFIED CRITICAL

**Adversary claim:** BC-5.39.008 v1.1 invariant 4 (+ PC4 + EC-003/004/005 + Test Vectors) all mandate `POLICY \d{3}` (three-digit form). Production `.factory/policies.yaml` uses bare integer YAML values (`id: 1` through `id: 18`). Zero matches for `POLICY \d{3}` outside comments. If hook ships per spec, it would produce a HARD BLOCK on every Edit/Write to the real production file.

**Orchestrator verification (literal shell):**

```
$ grep -nE '^  - id:' /Users/jmagady/Dev/vsdd-factory/.factory/policies.yaml | head -5
33:  - id: 1
47:  - id: 2
62:  - id: 3
75:  - id: 4
90:  - id: 5

$ grep -nE 'POLICY [0-9]{3}' /Users/jmagady/Dev/vsdd-factory/.factory/policies.yaml | grep -v '^[0-9]*:#'
(zero output — empty result set)
```

**Result: VERIFIED CRITICAL.** Production `policies.yaml` uses integer IDs (`id: 1` through `id: 18`); BC's invariant 4 + PC4 + EC-003/004/005 + Test Vectors all mandate `POLICY \d{3}` (e.g., `POLICY 001`–`POLICY 999`). Hook built per spec would block every legitimate write to the production file.

### Override 2: F-BC008P2-002 VERIFIED CRITICAL

**Adversary claim:** BC-5.39.008 v1.1 PC10 (line 141) explicitly states "the WASM sandbox CANNOT invoke subprocesses (`host::exec_subprocess` is NOT a registered host import)". SDK source proves otherwise.

**Orchestrator verification (literal shell):**

```
$ grep -nE '^pub fn exec_subprocess' /Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/host.rs
299:pub fn exec_subprocess(
```

**Result: VERIFIED CRITICAL.** `exec_subprocess` IS a registered, bounded host import at `crates/hook-sdk/src/host.rs:299`. BC's PC10 justification for Option (b) layering is factually wrong against SDK source. The operational reason for bash-provisions-cache + WASM-reads-cache architecture (per ADR-021) is valid, but the BC's chosen justification is a false factual claim.

### Override 3: F-BC007P2-001 VERIFIED HIGH (sibling regression)

**Adversary claim:** PO fix-burst closed `HookResult::Advisory` in BC-5.39.007 + BC-5.39.008 but did NOT sweep sibling BC-5.39.006 v1.3 for the same `HookResult::BlockWithFix` non-existence class.

**Orchestrator verification (literal shell):**

```
$ grep -cE 'HookResult::BlockWithFix' /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
16

$ grep -nE 'pub enum HookResult|^\s+(Continue|Block|Error|BlockWithFix|Advisory)' /Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/result.rs
18:pub enum HookResult {
20:    Continue,
24:    Block { reason: String },
31:    Error { message: String },
```

**Result: VERIFIED HIGH.** BC-5.39.006 v1.3 references `HookResult::BlockWithFix` 16 times. The hook-sdk `HookResult` enum has only `Continue`, `Block { reason }`, and `Error { message }` — `BlockWithFix` does NOT exist. This is a SPEC-INTERNAL-CONSISTENCY defect parallel to the `Advisory` class closed at pass-1. PO scope — BC-5.39.006 v1.3 → v1.4 required in fix-burst pass-2.

### Override 4: Net Blocking Status

- 2 verified CRITICAL findings: F-BC008P2-001 + F-BC008P2-002
- 1 verified HIGH sibling regression: F-BC007P2-001 (CLAUDE.md S-7.01 partial-fix discipline violation)
- 12 additional HIGH/MEDIUM/LOW/NITPICK findings (including orchestrator promotion of F-BC007P2-006 from LOW to MEDIUM)
- **STREAK: 0/3 → 0/3 RESET** (2 verified CRITICAL prevent advance; cascade continues to pass-3 required per BC-5.39.001 3-CLEAN protocol)

---

## PART A — Adversary Findings

### Finding Counts (pass-2)

| Pass | Findings (BC-007) | Findings (BC-008) | Total | Streak |
|------|-------------------|-------------------|-------|--------|
| Pass-1 | ~21 | ~20 | ~41 | 0/3 |
| Pass-2 | 7 | 7 | 14 | 0/3 |

> Note: F-BC008P2-005 (original numbering) was demoted and withdrawn by the adversary during Level-1 self-correct. Final retained count is 14. F-BC008P2-006 was promoted to MEDIUM and renumbered as F-BC007P2-006 by orchestrator. The 14 retained findings are: F-BC007P2-001..007 (7) + F-BC008P2-001..004 + F-BC008P2-007 + F-BC008P2-008 (7 BC-008 findings).

---

### CRITICAL Findings (2)

**F-BC008P2-001 — CRITICAL — policies.yaml uses integer `id:` values; BC's `POLICY \d{3}` invariant would BLOCK production file (META-LEVEL-INV-016 RE-INSTANCED in same fix-burst that codified it)**

Production `.factory/policies.yaml` uses bare integer YAML values (`id: 1` through `id: 18` at lines 33-282). Zero matches for `POLICY \d{3}` outside comments. BC-5.39.008 v1.1 invariant 4 (line 170), PC4 (line 266-268), EC-003/004/005 (lines 317-319), Test Vectors (lines 346-347) all mandate `POLICY \d{3}` (e.g., `POLICY 001`–`POLICY 999`). If the hook ships per spec, it would produce a HARD BLOCK on every Edit/Write to the real production file.

Orchestrator-verified evidence: see Override 1 above.

Load-bearing: yes. **F-BC008P2-001 = VERIFIED CRITICAL.**

---

**F-BC008P2-002 — CRITICAL — PC10 claims `host::exec_subprocess` is NOT a registered host import; hook-sdk proves it IS at host.rs:299**

BC-5.39.008 v1.1 PC10 line 141 explicitly states: "the WASM sandbox CANNOT invoke subprocesses (wasmtime sandbox model per ADR-002; `host::exec_subprocess` is NOT a registered host import)". This is presented as load-bearing constraint dictating the bash-data-provisioner + WASM-decision-maker architecture.

Orchestrator-verified evidence: see Override 2 above.

The operational reason for Option (b) layering (bash-provisions-cache + WASM-reads-cache) per ADR-021 is valid (network access, freshness, binary-allow-list) but the BC's chosen justification "exec_subprocess is NOT a registered host import" is factually wrong. **F-BC008P2-002 = VERIFIED CRITICAL.**

---

### HIGH Findings (4)

**F-BC007P2-001 — HIGH — Sibling-discipline divergence: BC-5.39.006 v1.3 uses `HookResult::BlockWithFix` 16 times but hook-sdk has only `Continue`, `Block`, `Error`. PO fix-burst closed `HookResult::Advisory` in 007/008 but did NOT sweep the SAME class in active sibling BC-5.39.006 v1.3.**

Orchestrator-verified evidence: see Override 3 above.

This is a SPEC-INTERNAL-CONSISTENCY defect: BC-5.39.006 v1.3 references a non-existent SDK construct 16 times. Resolution requires PO to amend BC-5.39.006 v1.3 → v1.4 to use the actually-existing form (`HookResult::Block { reason }` with appropriate constructor if available, or `HookResult::Block { reason: format!(...) }`). Orchestrator adjudication: PO scope (BC content amendment) — in scope of PO fix-burst pass-2.

**Severity: HIGH** (sibling-sweep discipline; CLAUDE.md S-7.01).

---

**F-BC007P2-002 — HIGH — Phase-1 false-negative window not bounded for never-shipped Phase 2 case**

BC-5.39.007 v1.1 description states cross-site staleness "remains a Phase 1 advisory log item (not a block)... If Phase 2 is never shipped, Phase 1's blocking coverage governs all structural violation classes." Contradiction: if Phase 2 is never shipped, the advisory-only cross-site staleness check ALSO governs forever. The BC must specify indefinite-deferral default behavior. Production-grade-default Rule 1 violation.

**Severity: HIGH.**

---

**F-BC007P2-003 — HIGH — PC2/PC5 renumber propagation incomplete: Phase-1 boundary table (line 240) + Test Vectors (line 349) still cite old "PC2" and "PC5 violated" labels after F-BC007P1-008 split PC2 → PC2a/PC2b**

F-BC007P1-008 closure regressed: the split was applied in the Preconditions section but consumer references in the Phase-1 boundary table + Test Vectors still cite the OLD "PC2" label. Additionally, Test Vectors line 349 says "PC5 violated" — but PC5 in v1.1 is "Lesson entry detection" which has no violation case as written. This is a sibling-sweep-of-own-fix defect.

**Severity: HIGH** (sibling-sweep-of-own-fix).

---

**F-BC008P2-003 — HIGH — Severity-enum invariant 5 self-contradiction**

BC-5.39.008 v1.1 invariant 5 (lines 272-277): "severity: string; allowed values are HIGH and MEDIUM... The P0/P1/P2/P3/P4 severity vocabulary is an alternative form that MAY be introduced in a future amendment; if present, the hook accepts both vocabularies." This is simultaneously: (a) P0-P4 is hypothetical/future, AND (b) hook accepts P0-P4 NOW. For a security-critical schema gate, this ambiguity violates Production-Grade Default Rule 6. Production-grade decision: HIGH+MEDIUM only in v1.1; defer P0-P4 to future v1.2 amendment with explicit invariant stating "P0-P4 vocabulary NOT accepted in v1.1."

**Severity: HIGH.**

---

### MEDIUM Findings (5)

**F-BC007P2-004 — MEDIUM — 4-file dispatch arm-routing under-specified**

BC-5.39.007 v1.1 fires on Edit/Write to any of 4 files but PC2a is STATE.md-specific. Hook must declare arm-routing as a top-level structural feature, not a buried PC. Risk: false-positive Block on lessons.md write when PC2a precondition fails.

**Severity: MEDIUM.**

---

**F-BC008P2-004 — MEDIUM — Orphan "PC2 scope clarification" paragraph references unnamed precondition**

BC-5.39.008 v1.1 has `### Part B — PC2 scope clarification` (lines 122-127) but Preconditions are numbered 1, 2, 3, ..., 10 by ordinal — no named "PC2". The paragraph is orphaned with no anchored target.

**Severity: MEDIUM.**

---

**F-BC007P2-005 — MEDIUM — EC-016 (lessons.md timeout fail-open) vs EC-018 (lessons.md `### Closes` blocking) cascade order undefined**

What happens when lessons.md read times out partially AND the partial content contains a `### Closes` block? Cascade order (invariant 9 vs postcondition 2) is undefined. Production-grade hook requires this precedence to be explicit.

**Severity: MEDIUM.**

---

**F-BC007P2-006 — MEDIUM — Forward-traceability ADR-021 "Open Sub-Questions for S-15.15 Part C Implementer" not cited as source of advisory-severity-threshold question closed by PC13**

BC-5.39.008 v1.1 closes advisory-severity-threshold question (PC13 lines 239-249) but doesn't cite ADR-021 "Open Sub-Questions" §2 (line 268+) as the source of what was closed. This is a traceability anchor for an ADR-anchored decision.

**Severity: MEDIUM** (orchestrator promotion from LOW — POLICY 5 creators_justify_anchors).

---

**F-BC008P2-005 — MEDIUM — ADR-021 line 251 cite is partial-sentence (POLICY 15 LL-N adjacent class)**

BC-5.39.008 v1.1 PC10 cites "ADR-021 line 251" but line 251 alone reads only "Rejected. The structural false-negative risk for security-critical advisories is" — a partial sentence. Spec-internal citation should be human-resolvable to a complete unit (section heading + line range).

**Severity: MEDIUM.**

---

### LOW Findings (3)

**F-BC007P2-007 — LOW — Invariant 5 regex parenthetical inconsistency**

Invariant 5 references `\(per D-413(b) (?:completeness )?mandate\)` regex but EC-004's quoted example reads `(per D-413(b) completeness mandate)` — non-capture syntax mismatch. Minor; not load-bearing.

**Severity: LOW.**

---

**F-BC008P2-007 — LOW — frontmatter `phase: section-12-step-3M3a` stale after v1.1 amendment burst (was 3M3a-r)**

Both BC files have `phase: section-12-step-3M3a` despite v1.1 amendment occurring at 3M3a-r. State-manager adjudicates whether phase field should update at amendment.

**Severity: LOW.**

---

**F-BC008P2-008 — LOW (demoted from MEDIUM) — ADR-021 "Open Sub-Questions" not cited independently**

Promoted to F-BC007P2-006 (MEDIUM) by orchestrator; this finding is retained as LOW for the BC-5.39.008 perspective: BC-5.39.008's own PC13 also lacks the ADR-021 Open Sub-Questions §2 backlink. Companion to F-BC007P2-006 in a different BC.

**Severity: LOW.**

---

### NITPICK Findings (1)

**F-BC008P2-009 — NITPICK — Changelog v1.1 row is one long paragraph (~2000 chars); sibling BC-5.39.006 v1.3 uses shorter changelog rows**

Stylistic; not load-bearing. Companion changelog rows should be concise for readability parity.

**Severity: NITPICK.**

---

> **Note on withdrawn finding:** F-BC008P2-005 (original adversary numbering — a MEDIUM finding about a different issue) was demoted and withdrawn by the adversary during Level-1 self-correct before this persistence. It is not included above. The findings retained above are the 14 validated findings (7 BC-007-class + 7 BC-008-class). F-BC008P2-005 above is the renumbered MEDIUM finding originally numbered F-BC008P2-006 by the adversary before orchestrator promotion/renumbering.

---

## PART B — Adversary Meta-Assessment

### Streak Status

**STREAK: 0/3 CLEAN** (2 verified CRITICAL findings prevent advance; cascade to pass-3 required per BC-5.39.001 3-CLEAN protocol).

### Novelty Calibration

- 2 NEW CRITICAL findings that pass-1 missed entirely: integer-vs-`POLICY \d{3}` corpus mismatch (F-BC008P2-001), `exec_subprocess` SDK-source mis-claim (F-BC008P2-002). Both are RE-INSTANCES of the META-LEVEL-INV-016 class (BC-vs-ground-truth drift) that pass-1 codified at L-M3-BC-cascade-pass-1 + L-M3-BC-cascade-pass-1-PO-fix-burst. The PO fix-burst declared the META-LEVEL "closed" but introduced 2 fresh instances in the same fix.
- 1 NEW HIGH (F-BC007P2-001) sibling-discipline regression: BC-5.39.006 v1.3 not swept for `HookResult::BlockWithFix` non-existence parallel to `HookResult::Advisory` non-existence. CLAUDE.md S-7.01 partial-fix discipline violation.
- 2 HIGH spec-incoherence findings (F-BC007P2-003 PC2/PC5 renumber-propagation; F-BC008P2-003 severity-enum self-contradiction).

### META-LEVEL Detection

**META-LEVEL INV-017-CANDIDATE: "Codifying a discipline-class in lessons.md does NOT prevent the same class from re-occurring in the very fix-burst that closes its prior instance."**

L-M3-BC-cascade-pass-1 + L-M3-BC-cascade-pass-1-PO-fix-burst codified INV-016-CANDIDATE ("BC-authorship-must-grep-actual-artifact-format") at D-482/D-483. The PO fix-burst at `865062b5` then:

1. Authored PC4 + invariant 4 against `POLICY \d{3}` without grepping actual `policies.yaml` → F-BC008P2-001 CRITICAL.
2. Authored PC10 stating `exec_subprocess` is NOT a registered host import without grepping `crates/hook-sdk` → F-BC008P2-002 CRITICAL.
3. Failed to sweep BC-5.39.006 v1.3 for `HookResult::BlockWithFix` non-existence parallel to closed `Advisory` class → F-BC007P2-001 HIGH.

This is a self-application failure of the META-LEVEL-INV-016 codification: the PO knew the discipline (it is in `lessons.md` from the same burst that produced this fix) but did not apply it operationally during fix-burst authorship. The mechanical-shell-execution gate (D-449(a)) ensures the codification-record-of-execution gate fires; it does not ensure the codification's NORMATIVE CONTENT is applied to spec authorship.

**Forward routing:** Forwarded as INV-017-CANDIDATE for SK-MCP-001 Appendix D: "codified-discipline-must-be-applied-as-shell-gate-not-narrative-attestation-during-fix-burst." The cure is: BC-authoring agent MUST execute literal grep against canonical-source artifacts for EVERY value-claim in EVERY PC/invariant/EC, with stdout captured in the changelog row.

---

## PART C — Policy Rubric Coverage

| Policy | ID | Coverage Status | Notes |
|--------|----|----------------|-------|
| `spec_is_authoritative_over_code` | POLICY 1 | APPLIES — CRITICAL VIOLATION | BC-5.39.008 PC10 + invariant 4 contradict canonical sources (hook-sdk host.rs + policies.yaml). Per POLICY 1 spec wins over code — but here spec cites false facts about the code. Both must be corrected (spec to cite accurate ground-truth). |
| `prd_is_requirements_source_of_truth` | POLICY 2 | APPLIES — OK | PRD is not in scope of these BCs' direct requirements. |
| `arch_index_is_subsystem_registry` | POLICY 3 | APPLIES — OK | ARCH-INDEX subsystem names not in scope of BC-007/008 specific claim-sets reviewed. |
| `bc_h1_is_authoritative_title` | POLICY 4 | APPLIES — OK | BC H1 titles are well-formed in v1.1 amendments. |
| `creators_justify_anchors` | POLICY 5 | APPLIES — MEDIUM FINDING | F-BC007P2-006 / F-BC008P2-008: ADR-021 Open Sub-Questions §2 not cited as source of what PC13 closes. POLICY 5 requires creators to justify traceability anchors. |
| `bc_index_is_bc_catalog_source_of_truth` | POLICY 6 | APPLIES — OK | BC-INDEX v2.38 is authoritative; this burst does not change BC content. |
| `bcs_frontmatter_array_changes_propagate_atomically` | POLICY 7 | APPLIES — BLOCKED | PO fix-burst pass-2 must propagate BC-5.39.006 v1.3→v1.4 `bcs` frontmatter change to body BC table + ACs + Token Budget atomically per POLICY 7. |
| `vp_index_is_vp_catalog_source_of_truth` | POLICY 8 | APPLIES — OK (pending) | VP allocations for BC-5.39.007/008 v1.1 deferred per D-483(e); no new VP citations added in this pass. |
| `vp_changes_propagate_same_burst` | POLICY 9 | APPLIES — OK (pending) | Same as POLICY 8 — deferred to architect dispatch. |
| `story_index_is_story_catalog` | POLICY 10 | NOT IN SCOPE | No story changes in this burst. |
| `no_silent_deferrals` | POLICY 11 | APPLIES — OK | All findings explicitly enumerated; no silent deferrals. |
| `no_ai_attribution_in_commits` | POLICY 12 | APPLIES — OK | Commit message format per project convention. |
| `no_bypass_hook_chain` | POLICY 13 | APPLIES — OK | No `--no-verify` used. |
| `bc_authorship_must_grep_actual_artifact_format` | POLICY 14 | APPLIES — CRITICAL VIOLATION | INV-016 re-instance: PO authored PC4/invariant-4 without grepping `policies.yaml`; authored PC10 without grepping `crates/hook-sdk/src/host.rs`. POLICY 14 (if registered) is exactly this discipline. Forward as INV-017 cure mechanism. |
| `three_digit_policy_id_canonical` | POLICY 15 | APPLIES — CRITICAL VIOLATION | F-BC008P2-001 root cause: production `policies.yaml` uses integers, not `POLICY \d{3}`. BC mandates three-digit form on a file that doesn't use it. Migration not yet executed per D-483(e) / POLICY 15 deferred to S-15.15. |
| `decisions_log_umbrella_range_auto_advance` | POLICY 16 | APPLIES — OK | D-range preamble will be advanced in this burst per D-452 precedent. |
| `sibling_sweep_on_value_changes` | POLICY 17 | APPLIES — HIGH VIOLATION | F-BC007P2-001: sibling BC-5.39.006 v1.3 not swept for `HookResult::BlockWithFix` when `Advisory` was swept in 007/008. TD-VSDD-060 applies. |
| `production_grade_default` | POLICY 18 | APPLIES — OK (meta-level) | The 14 findings above are all production-grade defects; none are cosmetic-only. INV-017-CANDIDATE correctly classifies the systemic failure mode at meta-level. |
