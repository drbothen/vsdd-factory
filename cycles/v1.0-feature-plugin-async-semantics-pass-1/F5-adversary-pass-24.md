---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 24
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T01:00:00Z
strategic_recommendation: escalate-to-user
---

# F5 Pass-24 Adversary Review

## Verdict

**HIGH** — 7th consecutive HIGH. 3 HIGH + 2 MEDIUM + 1 process-gap observation. ADR-013 clock RESETS to 0_of_3.

Fix-burst-22 was the broadest sweep ever (~640+ files) and CLOSED all four pass-23 findings. POLICY 8 + POLICY 1 spot-checks pass. However pass-24 finds three NEW HIGH instances of the EXACT sibling-class recurrence pattern L-P22-001 named as structurally non-convergent. All three are at lateral sibling cite sites of fabrications fixed in fix-burst-21 sub-burst 2 (BC-1.14.001 fabricated-symbol cluster: `RegistryEntry.async`, `run_tiers`, `spawn_detached`). The L-P23-001 retroactive sweep was scoped narrowly to the BC-1.07.005/006 cluster that triggered the lesson, NOT corpus-wide for previously-fixed clusters.

Plus a HIGH POLICY 9 violation: VP-INDEX v1.31 claims VP-043 v1.1→v1.2 propagation; VP-043 file frontmatter still v1.1.

**Strategic recommendation: ESCALATE-TO-USER. The user's prose-only-continue choice at pass-23 had unknown evidence; pass-24 provides novel data validating L-P22-001's structural-non-convergence claim with new layer instances.**

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→HIGH(P19)→HIGH(P20)→HIGH(P21)→HIGH(P22)→HIGH(P23)→**HIGH(P24)**

**Seven consecutive HIGH passes.** Each fix-burst was broader than the last. Each next pass found NEW sibling-class instances at NEW layers. **Empirical confirmation that prose-only codification is structurally non-convergent for sibling-class sweeps.**

## Findings

### F-P24-001 [HIGH] VP-INDEX/VP-043 version-propagation gap (POLICY 9)

**Confidence:** HIGH

**Evidence:**
- VP-INDEX.md:13 claims `v1.31 (2026-05-09): VP-043 v1.1→v1.2`.
- VP-043.md:4 frontmatter: `version: "1.1"` (NOT v1.2).
- VP-043.md:122 — file ends at v1.1 changelog row; no v1.1→v1.2 Amendment block.
- VP-043.md:49 — §Source Contract DID get rebranded to cite `loads_generated_registry_from_disk`.

**Impact:** Body change happened, audit trail (frontmatter + Amendment) not updated. POLICY 9 same-burst propagation gap.

**Fix:** VP-043 frontmatter v1.1→v1.2; add v1.1→v1.2 Amendment block.

---

### F-P24-002 [HIGH] BC-7.06.001:130 fabricated `RegistryEntry.async: bool` field (L-P23-001 sibling-class recurrence)

**Confidence:** HIGH

**Evidence:**
- BC-7.06.001.md:130 — `RegistryEntry.async: bool` field with `#[serde(default)]`.
- registry.rs:244-245: real source `#[serde(default, rename = "async")] pub async_flag: bool`.
- BC-1.14.001 (sibling) was patched in fix-burst-21 sub-burst 2 to `RegistryEntry.async_flag`. BC-7.06.001 sibling missed.

**Impact:** Per L-P23-001 ("when patching ONE cite site of a fabricated symbol, ALL OTHER cite sites must be patched same-burst"), this is exactly the violation the lesson was meant to prevent.

**Fix:** Replace `RegistryEntry.async: bool` with `RegistryEntry.async_flag` (with `#[serde(default, rename = "async")]`) at BC-7.06.001:130.

---

### F-P24-003 [HIGH] S-15.01:494, 762 fabricated `run_tiers()` (L-P23-001 sibling-class recurrence)

**Confidence:** HIGH

**Evidence:**
- S-15.01:494 (T-3c task): `run `sync_group` via existing `run_tiers()``.
- S-15.01:762 (Critical Coupling): `Async tasks are spawned only after `run_tiers(sync_group)` completes.`
- `grep -rn "fn run_tiers" crates/` → 0 matches.
- executor.rs:79 — real `pub async fn execute_tiers(`.
- BC-1.14.001 was patched in fix-burst-21 sub-burst 2; S-15.01 sibling missed.

**Impact:** S-15.01 is `status: ready` (not retrospective). Implementers reading T-3c will hunt for `run_tiers()` and find 0 matches.

**Fix:** Replace `run_tiers()` with `execute_tiers()` at S-15.01:494, 762.

---

### F-P24-004 [MEDIUM] ADR-019:286-287 stale `routing.rs or engine.rs (partition implementation)` cite

**Confidence:** HIGH

**Evidence:**
- ADR-019:285-288 §Subsystem Assignments: `routing.rs or engine.rs (partition implementation)`.
- partition.rs:90 — real `partition_plugins` fn.
- main.rs:218 — real call site.

**Impact:** MEDIUM (disjunction "or" qualifier signals design-time speculation; §Implementation Pointers cites BCs by ID).

**Fix:** Replace with `partition.rs (partition_plugins) and executor.rs (execute_tiers/spawn_async_plugin)`.

---

### F-P24-005 [MEDIUM pending intent] bc-id-mapping.md:349-350 cites fabricated symbols

**Confidence:** MEDIUM (intent-pending — bc-id-mapping.md description column may be governed or carved out)

**Evidence:**
- bc-id-mapping.md:349-350 still cite `every_entry_routes_through_legacy_bash_adapter` and `every_entry_carries_a_script_path`.
- Frontmatter declares document_type: bc-id-mapping; producer: codebase-analyzer; phase: 1.4a.

**Adjudication needed:** orchestrator/human decides whether description column is L-P23-001-governed (active body) or carved out (Phase 0 historical record).

---

## Process-gap findings

### O-P24-001 [process-gap] L-P23-001 codifying burst applied the rule narrowly, not corpus-wide

L-P23-001 codified at fix-burst-22 sub-burst 3. Per L-P19-001, the codifying burst MUST run a corpus-wide retroactive sweep. The sweep was scoped to the BC-1.07.005/006 + VP-043 cluster (the cluster that triggered the lesson) and did NOT corpus-sweep for previously-fixed fabricated symbols (BC-1.14.001 cluster: `RegistryEntry.async`, `run_tiers`, `spawn_detached`).

F-P24-002, F-P24-003, F-P24-004 are direct consequences: BC-7.06.001 + S-15.01 + ADR-019 sibling cites of fabricated symbols already fixed in BC-1.14.001 should have been swept in the same fix-burst that codified L-P23-001.

**Codification proposal:** when a lesson is codified, the retroactive sweep MUST cover not just the symbol/pattern that triggered the lesson but ALL previously-codified symbols/patterns of the same class. Without this, the lesson reduces to a single-instance fix.

## Notable observations

1. **Fix-burst-22 closure verified:**
   - F-P23-001 27-BC + 440+ ss-05 sweep: spot-checks pass (BC-5.27.024 carve-out + Amendment v1.2 properly applied).
   - F-P23-002 cross-subsystem ss-06 sweep: BC-6.04.027 §Red Flag Indicators stable anchor verified.
   - F-P23-003 BC-1.07.005/006 H1 rebrand: H1 + BC-INDEX rows + VP-043 §Source Contract verified rebranded.
   - F-P23-004 L-P21-001 disposition: retroactive verification block added.
2. POLICY 1 + POLICY 8 spot-checks pass.
3. Index versions verified: BC-INDEX v1.48, VP-INDEX v1.31, STORY-INDEX v2.52, ARCH-INDEX v1.28.
4. ARCH-INDEX BC-INDEX cite at line 142 reads `per BC-INDEX v1.48` — current per L-P20-002.
5. **Active-body lobster-line-cite scan clean** (per L-P23-002 sweep): no unfixed lobster-cites detected outside carve-out vocabulary.
6. **Active-body fabricated-symbol scan finds BC-1.14.001 cluster sibling sites missed** (per L-P23-001 narrow application):
   | Symbol | Active-body matches |
   |---|---|
   | `RegistryEntry.async` (without `_flag`) | BC-7.06.001:130 — F-P24-002 |
   | `run_tiers` | S-15.01:494, 762; ADR-019:116 — F-P24-003 + F-P24-004 |
   | `spawn_detached` | ADR-019:121 — F-P24-004 |

## Convergence assessment

**Novelty: MEDIUM-HIGH.** Findings are new fresh-context evidence, but at the SAME structural pattern L-P22-001 named.

**ADR-013 clock: 0_of_3** (RESET).

**Recurrence pattern (7-pass empirical):**

| Pass | Fix-burst scope | Verdict | Recurrence |
|------|----------------|---------|------------|
| P18 | 1 crate | HIGH | sibling-hook predicate |
| P19 | corpus, literal grep | HIGH | range form missed |
| P20 | corpus, semantic grep | HIGH | range/source-line missed |
| P21 | 10 BCs | HIGH | 88 ss-05 + 7 fabrications |
| P22 | 88 BCs + 7 fabrications | HIGH | 27 postcondition-form + ss-06 |
| P23 | 27+440+174 BCs cross-subsystem | HIGH | BC-1.14.001 cluster siblings |
| P24 | (this pass) | HIGH | BC-7.06.001 + S-15.01 + VP-043 metadata + ADR-019 |

Each fix-burst was the broadest yet. Each next pass found NEW sibling-class instances at NEW layers.

## Strategic recommendation

**ESCALATE-TO-USER** — recommending halt-and-implement-S-15.03.

**Rationale:**
1. User chose continue-prose-only at pass-23 with prior pass evidence.
2. Pass-24 evidence is novel: broadest sweep ever still produced 3 HIGH sibling-class recurrences + 1 HIGH POLICY 9 propagation gap.
3. Codifying burst-22 sub-burst 3 applied L-P23-001 narrowly (BC-1.07.005/006 cluster only) — same failure mode L-P19-001 codified.
4. Recurrence is mechanically detectable: S-15.03's `validate-symbol-cite` hook would prevent F-P24-002, F-P24-003, F-P24-004 by design.
5. The user's prior decision had unknown evidence; pass-24's quantitative data shifts the calculus.

**If continue-prose-only:** fix-burst-23 must EXPLICITLY run corpus-wide grep for ENTIRE historical fabricated-symbol set + every previously-codified pattern. Anything less repeats recurrence.

**If halt-and-implement-S-15.03:** cycle pauses; S-15.03 implements `validate-symbol-cite` + `validate-index-cite-refresh` + `validate-lesson-retroactive-sweep` hooks. F5 resumes with mechanical enforcement.
