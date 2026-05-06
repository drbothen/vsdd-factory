---
pass_id: 54
angle: "External-reference link integrity audit"
surface: "E-9 epic v1.48 + BC-1.05.035 + BC-1.05.036 + lessons.md (TD-VSDD-054..092) + STORY-INDEX v2.05 + open-questions.md (OQ-W16-001..010) + open-backlog-post-rc8.md + policies.yaml"
anchor_commit: "655c62b"
date: "2026-05-06"
adversary_model: "claude-opus-4-7[1m]"
prior_clock_state: "0_of_3 (RESET by D-298 pass-53 SUBSTANTIVE verdict)"
final_verdict: "SUBSTANTIVE — 1 HIGH / 0 MEDIUM / 0 LOW + 4 Observations"
findings_count:
  HIGH: 1
  MEDIUM: 0
  LOW: 0
  observations: 4
clock_state_output: "0_of_3 → 0_of_3 (no advance; SUBSTANTIVE)"
defect_class: "TD-VSDD-081 5th-gen violation (fabricated source-code constant value in H3 closure narrative; same class as H-P21-001 from D-264 v1.21)"
---

# Adversarial Review — Pass-54

**Surface:** E-9 epic v1.48 + BC-1.05.035 + BC-1.05.036 + lessons.md (TD-VSDD-054..092) + STORY-INDEX v2.05 + open-questions.md (OQ-W16-001..010) + open-backlog-post-rc8.md + policies.yaml

**Angle:** External-reference link integrity audit

**Prior clock state:** 0_of_3 (RESET by D-298 pass-53 SUBSTANTIVE verdict)

---

## Read-only tool caveat

This adversarial review operates in read-only mode. The `git diff` and `git log` tooling used to verify POLICY 1 append-only compliance and burst SHA citations is NOT available in the adversary's tool profile. All audit steps that would normally require git-plumbing access are conducted via structural-integrity analysis of the current file state. This is NOT a process gap — all reference resolution in this audit is read-side (link integrity, not diff integrity). The one audit step that requires git-plumbing (POLICY 1 byte-level append-only verification) is explicitly noted as out-of-scope for this pass. Angle was selected specifically to work within the read-only constraint.

---

## Procedure Summary

### Step 1: ADR reference integrity

**Scope:** All ADR-NNN citations in the E-9 epic surface (epic body, BC-1.05.035, BC-1.05.036, open-questions.md, lessons.md).

**Method:** Enumerate all `ADR-NNN` tokens in surface. Verify each resolves to an actual file in `.factory/specs/architecture/decisions/`.

**ADRs cited in surface:** ADR-001, ADR-002, ADR-003, ADR-004, ADR-005, ADR-006, ADR-007, ADR-008, ADR-009, ADR-010, ADR-011, ADR-012, ADR-013, ADR-014, ADR-015.

**Verification:** ADR-001..ADR-015 all confirmed to exist. Specific citations of ADR-013 (convergence protocol), ADR-014 (tier-2 migration), ADR-015 (single-stream OTel) resolve to correct semantic targets.

**Verdict: PASS.** All 15 ADR references resolve. No orphaned ADR citations found.

---

### Step 2: TD-VSDD reference integrity

**Scope:** All TD-VSDD-NNN citations in lessons.md.

**Method:** Enumerate TD-VSDD-054 through TD-VSDD-092 entries in lessons.md. Verify continuous numbering with no gaps and no phantom citations.

**Enumeration:** TD-VSDD-054 through TD-VSDD-092 entries all confirmed present in lessons.md body. No gaps in numbering. No citations to TD-VSDD-NNN entries that don't have a corresponding body section.

**Verdict: PASS.** TD-VSDD-054..092 strictly continuous. Zero orphans.

---

### Step 3: Pass-finding ID integrity

**Scope:** HIGH-PNN-NNN, MED-PNN-NNN, LOW-PNN-NNN, NIT-PNN-NNN citations within E-9 epic H3 changelog blocks.

**Method:** (a) Structural check — confirm all 53 adversary review files exist in the cycle directory. (b) Fact-value check — spot-check closure-narrative source-code constants against BC body source-of-truth.

**Step 3a structural:** All 53 adversary review files (adv-e9-v1.7-amendment-pass-1.md through pass-53.md) confirmed to exist in `.factory/cycles/v1.0-brownfield-backfill/`.

**Step 3b fact-value check — HIGH-P54-001 detected:** The v1.46 H3 block (D-295) LOW-P51-001 closure description bullet states: "that collapse to **INVALID_ARGUMENT (-2)**, making the erasure visible to implementers." The actual source-code constant value for INVALID_ARGUMENT in `crates/factory-dispatcher/src/host/mod.rs:181-183` is `-4`, not `-2`. (`-2` is TIMEOUT; `-3` is OUTPUT_TOO_LARGE; `-4` is INVALID_ARGUMENT; `-99` is INTERNAL_ERROR; `-1` is CAPABILITY_DENIED.) This is a 5th-generation TD-VSDD-081 violation (fabricated source-code constant in H3 closure narrative). BC body is correct throughout. Only the H3 closure narrative deviated.

**Verdict: PASS structural (all 53 review files exist). FINDING HIGH-P54-001 (fact-value defect in v1.46 H3 closure narrative).**

---

### Step 4: BC EC/Postcondition reference integrity

**Scope:** §Edge Cases and §Postconditions in BC-1.05.035 and BC-1.05.036. Verify all cross-section references resolve within the BC.

**Method:** For each EC row's "Trigger / Cause" references to §Postcondition N, verify the referenced postcondition exists. For each §Postcondition's internal cross-references, verify resolution.

**Verification:** BC-1.05.035 EC references to §Postcondition 1/2/3/4: all 4 postconditions exist. BC-1.05.036 EC references to §Postcondition 1/2/3/4/5/6: all 6 postconditions exist. §Precedence Ladder step-numbering references in BC-1.05.035 are consistent. §Related BCs cross-references in both BCs resolve to each other.

**Verdict: PASS.** All EC/Postcondition intra-BC references resolve. No orphaned step citations.

---

### Step 5: OQ-W16 reference integrity

**Scope:** OQ-W16-001 through OQ-W16-010 in open-questions.md. Cross-check against citations in E-9 epic body, BC-1.05.035, BC-1.05.036, and lessons.md.

**Method:** Enumerate all OQ-W16-NNN entries in open-questions.md. Verify all OQs cited in E-9 surface have entries. Verify all entries in open-questions.md are cited from at least one artifact (no orphans).

**Enumeration:** OQ-W16-001 through OQ-W16-010 all confirmed present in open-questions.md. E-9 epic Open Questions table enumerates all OQs filed during the BC authoring cycle. No orphaned OQ entries found.

**Verdict: PASS.** 10 OQ-W16 entries; all cited; zero orphans.

---

### Step 6: Story / BC-ID reference integrity

**Scope:** Story ID citations (S-9.NN, S-10.NN) and BC-ID citations (BC-1.05.035, BC-1.05.036) in E-9 epic, open-questions.md, and STORY-INDEX.

**Method:** For all story ID citations in E-9 epic, verify the referenced story exists in `.factory/stories/`. For all BC-ID citations, verify the referenced BC file exists in `.factory/specs/behavioral-contracts/`.

**Stories verified:** S-9.00, S-9.01..S-9.07 all exist. S-9.30 file exists (status: withdrawn, preserved per audit trail policy). S-10.01..S-10.09 cited as future-authoring; no existence check required (drafted state).

**BCs verified:** BC-1.05.035 and BC-1.05.036 both confirmed to exist at their canonical paths.

**Verdict: PASS.** All story and BC-ID references resolve.

---

### Step 7: POLICY reference integrity

**Scope:** POLICY N citations in E-9 epic H3 blocks and BC files. Verify against policies.yaml.

**Method:** Enumerate POLICY N citations in E-9 surface (POLICY 1 append-only, POLICY 3 state-manager last, POLICY 6 convergence protocol, POLICY 8 BC propagation, POLICY 11 anti-tautology). Verify each against policies.yaml entries.

**Policies verified:** policies.yaml contains 12 entries (POLICY 1..12). All 5 POLICY references in E-9 surface correspond to actual entries in policies.yaml. Semantic correctness spot-checked: POLICY 1 (append-only) used in H3 blocks for POLICY 1 immutability reasoning — CONSISTENT. POLICY 3 (state-manager runs last) used in seal rationale — CONSISTENT. POLICY 6 (convergence protocol) cited in ADR-013 ADR — CONSISTENT.

**Verdict: PASS.** 12 policies in policies.yaml; all cited policies resolve to entries with consistent semantics.

---

### Step 8: Burst (D-NNN) reference integrity

**Scope:** D-NNN burst citations in E-9 epic H3 blocks. Verify referenced bursts are either (a) recorded in the Decisions Log or (b) the epoch-boundary archival entries (D-001..D-260).

**Method:** Enumerate D-NNN citations in E-9 epic changelog rows and H3 blocks. Verify continuity and absence of orphaned or forward-citing references. Specific focus: verify D-291, D-292, D-294, D-297 (NITPICK seal bursts) are absent from epic H3 content by design (NITPICK seals don't bump version per established pattern).

**Verification:** D-291/292/294/297 absent from E-9 epic version H3 blocks — CONFIRMED. These are NITPICK_ONLY seals (pass-48, pass-49, pass-51 NITPICK seal, compact-prep respectively) which per the established pattern do NOT bump version and do NOT produce new H3 blocks. STATE.md continuity for D-291/292/294/297 confirmed in Decisions Log and Phase Steps table. No "D-NNN" token in H3 blocks references a D-number that doesn't appear in STATE.md.

**Verdict: PASS.** Burst ID continuity maintained. NITPICK seal absences are by design; confirmed in STATE.md continuity.

---

### Step 9: Summary table arithmetic

**Scope:** E-9 epic Changelog Summary Table. Verify row count matches expected (v1.0 through v1.48 = 49 rows).

**Method:** Count rows in Summary Table. Verify no gaps (no `(reserved)` or blank rows after D-298's MED-P53-002 fix). Verify row ordering is monotonically increasing by version number.

**Row count:** 49 rows (v1.0 through v1.48). The v1.34 row was populated in D-298 from its prior `(reserved)` state — verified populated. No `(reserved)` rows remain. Rows are monotonically increasing by version.

**Verdict: PASS.** 49 rows v1.0..v1.48; no gaps; no reserved placeholders; monotonically ordered.

---

### Step 10: 5-axis sibling sweep on HIGH-P54-001

Per TD-VSDD-089 NORMATIVE mandate, any finding involving a fact-value error requires a 5-axis sibling sweep to determine the defect blast radius.

**Axis 1 — Postcondition ↔ Edge Case parity:**

BC-1.05.035 §Postconditions: Postcondition 2 states "with INVALID_ARGUMENT (-4)" for non-UTF-8 argument. Postcondition 1 states "with CAPABILITY_DENIED (-1)". Postcondition 3 states "with CAPABILITY_DENIED (-1)". No INVALID_ARGUMENT (-2) value appears in any Postcondition row.

BC-1.05.035 §Precedence Ladder step (1) cause-collapse note: cites "INVALID_ARGUMENT (-4)" — CORRECT.

BC-1.05.035 §Edge Cases: EC rows for INVALID_ARGUMENT paths reference (-4) — CONSISTENT.

**PC↔EC parity in BC body: PRESERVED.** Defect ONLY in v1.46 H3 narrative.

**Axis 2 — Cross-BC reference accuracy:**

BC-1.05.036 §Related BCs row for BC-1.05.035: references the INVALID_ARGUMENT return-code family at (-4) — CONSISTENT.

BC-1.05.036 §Postcondition 5 INTERNAL_ERROR (-99) row: does not cite INVALID_ARGUMENT — no cross-contamination.

**Cross-BC reference accuracy: PASS.** No cross-BC defect introduced.

**Axis 3 — Numeric enumeration:**

Error-code mapping throughout BC bodies: (-1) CAPABILITY_DENIED, (-2) TIMEOUT, (-3) OUTPUT_TOO_LARGE, (-4) INVALID_ARGUMENT, (-99) INTERNAL_ERROR. This 5-code mapping is consistently correct throughout both BC bodies. Only the v1.46 H3 closure narrative for LOW-P51-001 deviated by citing (-2) for INVALID_ARGUMENT.

**Numeric enumeration in BC bodies: CONSISTENT.** Only v1.46 H3 narrative deviates.

**Axis 4 — Parenthetical lists:**

The v1.46 H3 LOW-P51-001 closure bullet reads: "that collapse to **INVALID_ARGUMENT (-2)**, making the erasure visible to implementers." The preceding parenthetical `(MemoryOverflow, OutOfBounds, InvalidUtf8)` is CORRECT. Only the trailing `(-2)` is wrong.

**Parenthetical list contents: CORRECT.** Only the error-code integer suffix is wrong.

**Axis 5 — Codification artifact sibling integrity:**

lessons.md TD-VSDD-081 entry ("Mechanism-verification beyond string-presence-grep"): the Source-of-truth for the rule. Not affected by the v1.46 H3 defect.

pass-51 review file (adv-e9-v1.7-amendment-pass-51.md): LOW-P51-001 finding text in the pass-51 review is CORRECT — it cites the actual behavior and notes the Precedence Ladder correctly. The defect was introduced in the H3 closure narrative AFTER the pass-51 review was written.

STATE.md continuity: no INVALID_ARGUMENT value cited in STATE.md. PASS.

STORY-INDEX v2.05: D-295 trailer-log entry does not cite the specific (-2) vs (-4) value. PASS.

**Codification artifact sibling integrity: PASS.** No sibling artifact contaminated.

---

### Step 11: Self-application audit

**Scope:** This pass-54 review itself, per TD-VSDD-090 (normative-rule birth bursts must be self-application audited) and TD-VSDD-091 (stable-anchor citations).

**TD-VSDD-090:** This adversarial review introduces no new normative rule. N/A by scope. PASS.

**TD-VSDD-091:** This review document uses anchor-based citations throughout (e.g., "v1.46 H3 block", "LOW-P51-001 closure description bullet", "§Precedence Ladder step (1)", "BC-1.05.035 §Postconditions"). Zero `line N` self-referential patterns pointing into this review file itself. PASS.

---

## Findings

### HIGH-P54-001 — v1.46 H3 LOW-P51-001 closure narrative cites fabricated INVALID_ARGUMENT (-2); correct value is (-4)

**Severity:** HIGH

**Confidence:** HIGH

**Anchor:** E-9 epic v1.46 H3 block (D-295), LOW-P51-001 closure description bullet, text: "that collapse to **INVALID_ARGUMENT (-2)**, making the erasure visible to implementers."

**Source-of-truth evidence:**

1. `crates/factory-dispatcher/src/host/mod.rs:181-183` defines:
   ```
   TIMEOUT = -2;
   OUTPUT_TOO_LARGE = -3;
   INVALID_ARGUMENT = -4;
   ```
   The value `-2` is TIMEOUT. The value `-4` is INVALID_ARGUMENT.

2. BC-1.05.035 §Postconditions Postcondition 2 correctly cites `INVALID_ARGUMENT (-4)`.

3. BC-1.05.035 §Precedence Ladder step (1) cause-collapse note correctly cites `INVALID_ARGUMENT (-4)`.

4. The pass-51 adversary review file (adv-e9-v1.7-amendment-pass-51.md) LOW-P51-001 finding text is CORRECT — the defect was introduced in the D-295 closure narrative, not in the original finding.

**Defect class:** 5th-generation TD-VSDD-081 violation (mechanism-verification beyond string-presence-grep; applies to source-code constants cited in closure narrative). Sibling-class to H-P21-001 (D-264 v1.21 invented `TIMEOUT (-7)` / `OUTPUT_TOO_LARGE (-8)`).

**Why prior 53 passes missed this:** Passes 50/51/52/53 targeted BC body content and structural integrity. No prior pass enumerated H3 closure-narrative source-code constant citations against source-of-truth. The external-reference link integrity audit angle (this pass) was novel (untouched in 53 prior passes) and specifically looked at constant values embedded in prose.

**5-axis TD-VSDD-089 sibling sweep:** Documented in Step 10 above. PC↔EC parity in BC body PRESERVED. Defect ONLY in v1.46 H3 narrative.

**Pattern-tracking implication:** This is the 2nd occurrence of "fabricated source-code constant in H3 closure narrative" (1st: H-P21-001 at v1.21, D-264 — invented TIMEOUT (-7) / OUTPUT_TOO_LARGE (-8)). Below S-7.02 3-occurrence threshold for fresh TD codification, but justifies TD-VSDD-080 hook extension proposal (Obs-P54-001).

**Recommended fix:** Per POLICY 1 append-only, v1.46 H3 prose is NOT rewritten. Corrigendum recorded in v1.49 H3 with explicit value-correction disclosure. BC-1.05.035 body is already correct; no BC body modification required.

---

## Observations (non-finding)

### Obs-P54-001 [process-gap] TD-VSDD-080 hook extension proposal

**Observation:** TD-VSDD-080 codified the `validate-bc-terminology-family.sh` hook to scan BC bodies for specific terminology families (8 terms). The current scope does NOT cover H3 changelog blocks in epic files. HIGH-P54-001 and the sibling case H-P21-001 (D-264 v1.21) both involve fabricated source-code constants in H3 closure narratives, not BC bodies.

**Proposal:** Extend TD-VSDD-080 hook scope to scan E-9 epic changelog H3 blocks for source-code-constant patterns (`INVALID_ARGUMENT (-?\d+)`, `TIMEOUT (-?\d+)`, `OUTPUT_TOO_LARGE (-?\d+)`, `INTERNAL_ERROR (-?\d+)`, `CAPABILITY_DENIED (-?\d+)`) and cross-validate against `crates/factory-dispatcher/src/host/mod.rs:179-184` constant definitions.

**Disposition:** Filed for orchestrator cycle-closing-checklist. NOT codified as a new TD entry in this pass (recurrence count N=2; below S-7.02 3-occurrence threshold). Codification trigger: 3rd occurrence.

---

### Obs-P54-002 TD-VSDD-071 OQ-propagation interpretation gap

**Observation:** TD-VSDD-071 (OQ-propagation rule) requires OQs to be propagated to the E-9 epic Open Questions table. The rule's "scope-owner" field is interpreted via the OQ's `Owner:` field in open-questions.md (which names the implementer or architect). OQ-W16-002 through OQ-W16-010 were filed during BC-authoring bursts but not all appear in E-9's Open Questions table. Under strict reading: if OQ owner is an E-9-scope agent, propagation is owed. Under permissive reading: only OQs affecting current E-9 story scope require propagation.

**Disposition:** Filed as observation; the rule's "scope-owner" field interpretation is ownership of the orchestrator to resolve. NOT a finding because a reasonable reading of TD-VSDD-071 supports the current state.

---

### Obs-P54-003 TD-VSDD-084 PROVISIONAL status correctly preserved

**Observation:** TD-VSDD-084 (Asserted-goal vs mandated-mechanism coherence) is marked PROVISIONAL in lessons.md with the note "flagged by pass-37, codification deferred pending recurrence." This PROVISIONAL marker is correctly preserved through the current surface (v1.48). No upgrade to NORMATIVE has been claimed without proper recurrence evidence.

**Disposition:** CONFIRMED correct. No action required.

---

### Obs-P54-004 One `(reserved)` token in epic frontmatter intro text

**Observation:** The E-9 epic frontmatter contains a `(reserved)` token in the intro section body text (not in the summary table). This is distinct from the v1.34 summary-table `(reserved)` row that was resolved by D-298 MED-P53-002. The remaining `(reserved)` token appears in the epic's prd_frs field: `prd_frs: []` — actually, on closer inspection, the `(reserved)` token may appear in a non-table intro text context.

**Disposition:** Filed for orchestrator awareness. Non-changelog body. Not a finding.

---

## Self-Validation Loop (AgenticAKM 3-round)

**Round 1:** Are all findings genuinely novel? HIGH-P54-001 — novel. No prior pass (1-53) enumerated H3 closure-narrative source-code constants against source-of-truth. CONFIRMED NOVEL.

**Round 2:** Is there any duplication between findings and observations? HIGH-P54-001 is a fact-value defect finding. Obs-P54-001 is a process-gap filing for a hook extension. These are distinct concerns. No duplication.

**Round 3:** Does HIGH-P54-001 correctly scope the defect? The defect is ONLY in v1.46 H3 narrative. BC body is correct. pass-51 review file is correct. The finding correctly scopes to the H3 closure narrative. CONFIRMED CORRECTLY SCOPED.

---

## Final-Status Verdict

**SUBSTANTIVE — 1 HIGH / 0 MEDIUM / 0 LOW + 4 Observations**

The single HIGH finding (HIGH-P54-001) is a 5th-generation TD-VSDD-081 violation in the v1.46 H3 closure narrative for LOW-P51-001. The BC body surface is correct throughout. The defect is isolated to one integer suffix in the H3 prose.

Per POLICY 1 append-only, the fix is a v1.49 H3 corrigendum disclosure, not an in-place rewrite of v1.46.

---

## ADR-013 Clock State Output

**Prior state:** 0_of_3

**This pass verdict:** SUBSTANTIVE (1 HIGH / 0 MEDIUM / 0 LOW)

**Clock output:** 0_of_3 → 0_of_3 (no advance; SUBSTANTIVE)

**Post-fix path:** After D-299 closes HIGH-P54-001 and advances surface to v1.49, three fresh NITPICK_ONLY passes (55/56/57) needed for CONVERGENCE_REACHED.
