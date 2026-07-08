# Adversarial Review — E-19 Pass 23 (post-D-776 delta; perimeter = ADR-030 v1.2 + BC-5.42.001 v1.3 + BC-1.17.001 v1.3 + S-19.01 v1.13 + S-19.06 v1.14)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section + ADR-030 + BC-5.42.001 + BC-1.17.001 + BC-4.13.001
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 2 / LOW 0 (3 findings; 2 observations)
**Streak:** 0/3 RESET (streak was 0/3 entering pass-23 per D-776)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** ADR-030 v1.2 (was v1.1); BC-5.42.001 v1.3 (was v1.2); BC-1.17.001 v1.3 (was v1.2); S-19.01 v1.13 (was v1.12); S-19.06 v1.14 (was v1.13).

## Part A — D-776 Delta Verification + New Findings

### Amendment 1 — ADR-030 v1.1 → v1.2 (^Agent → ^Agent$ tool-filter anchoring)

Tool filter in Decision 1 TOML stanza updated from `^Agent` to `^Agent$` ✓. POLICY 14 5-leg parity: version v1.2; last_amended; modified[]; body Changelog row; ARCH-INDEX row v1.1→v1.2 ✓.

**F-P23-001 HIGH — ADR-030 v1.2 Decision 1 canonical TOML snippet wrong shape on three structural axes.**

ADR-030 Decision 1 presents a canonical SubagentStop hook TOML stanza. After the D-776 fix addressed the `^Agent` → `^Agent$` anchoring, three structural axes remain incorrect relative to the live hooks-registry.toml SubagentStop entries:

(1) `tool = "^Agent$"` — the `tool` field does not exist in live SubagentStop hook entries. SubagentStop entries in hooks-registry.toml use `tier` and `on_error` but carry no `tool` filter field. The D-776 fix corrected the regex value but did not remove the non-existent field from the canonical stanza.

(2) `on_error = "advisory"` — the live registry uses `on_error = "continue"` for this entry. The `"advisory"` enum variant is not the runtime behavior; the plugin emits advisory output via stdout (advisory-block-mode) but the `on_error` field itself must be `"continue"` to match the live registry shape.

(3) `priority = 150` — the live registry entry carries `priority = 920`. Priority 150 is the default range; the actual hook is registered at 920.

A canonical snippet in an ADR Decision section is a normative reference. Downstream engineers and spec reviewers use it as ground truth for validating or reproducing the registry entry. A snippet with three wrong structural axes will produce an incorrect registry entry if followed literally.

**Locus:** ADR-030 Decision 1 §canonical SubagentStop TOML stanza.
**Fix:** ADR-030 v1.2→v1.3: remove `tool` field entirely; set `on_error = "continue"`; set `priority = 920`. Note F-P22-003 superseded (the ^Agent$ fix was correct but did not surface the adjacent structural errors). Also: ARCH-INDEX.md ADR-030 row descriptor must be updated to reflect the corrected canonical shape (on_error="advisory"→"continue"; priority=150→920; tool= field removed).

### Amendment 2 — BC-5.42.001 v1.2 → v1.3 (hook-plugins/ WASM path)

§Architecture Anchors: `hook-plugins/pr-manager-completion-guard.wasm` ✓. POLICY 14 5-leg parity ✓.

No new findings on BC-5.42.001.

### Amendment 3 — BC-1.17.001 v1.2 → v1.3 (ffi.rs raw wire-ABI anchor)

§Architecture Anchors: `crates/hook-sdk/src/ffi.rs` raw wire-ABI bullet added alongside `host.rs` ✓. POLICY 14 5-leg parity ✓.

No new findings on BC-1.17.001.

### Amendment 4 — S-19.01 v1.12 → v1.13 (BC-5.42.001 v1.3 cite propagation)

§Architecture Mapping WASM path `hook-plugins/pr-manager-completion-guard.wasm` ✓. Input-hash 2a9f0b4 ✓. POLICY 14 5-leg parity ✓.

No new findings on S-19.01.

### Amendment 5 — S-19.06 v1.13 → v1.14 (BC-1.17.001 v1.3 cite propagation)

BC-1.17.001 v1.3 cite propagation confirmed at: Narrative paragraph, §Behavioral Contracts table Version cell, Token Budget, AC-007 Gate + BC Trace, §Architecture Compliance Rules. Input-hash 5af0d9f ✓. POLICY 14 5-leg parity ✓.

No new findings on S-19.06.

### Full E-19 Epic and Story Suite Review

**F-P23-002 MEDIUM — BC-4.13.001 v1.8 tense inconsistency throughout AC body.**

BC-4.13.001 §Acceptance Criteria and §Invariants use past-tense constructions ("was held", "was expired", "did not match") in multiple normative clauses where the VSDD BC convention requires present-tense imperative ("is held", "is expired", "does not match"). The tense inconsistency is spec-level ambiguity: past-tense framing implies a post-condition check, while present-tense imperative specifies an invariant that must hold at all times. The factory-lock guard behavioral contract is an always-enforced PreToolUse gate; its ACs should read as present-tense invariants.

Affected: S-19.02 and S-19.07 cite BC-4.13.001 v1.8 directly in §Behavioral Contracts table and AC traceability. When BC tense is corrected, both story files must propagate the updated version cite.

**Locus:** BC-4.13.001 §Acceptance Criteria, §Invariants — all past-tense normative clauses.
**Fix:** BC-4.13.001 v1.8→v1.9: replace_all past-tense normative constructions with present-tense imperative. POLICY 14 5-leg parity applied. Story-writer: S-19.02 + S-19.07 BC-4.13.001 cite propagation. State-manager: STORY-INDEX BC-coverage footer cite sweep.

**F-P23-003 MEDIUM — ARCH-INDEX ADR-030 row descriptor cites stale canonical shape.**

ARCH-INDEX.md ADR-030 subsystem row descriptor text still references the pre-D-776 canonical shape: it cites `on_error="advisory"` and `priority=150` and includes the `tool=^Agent$` field. This is the same structural error cluster as F-P23-001 but persists in the ARCH-INDEX descriptor independently of the ADR itself. Per POLICY 6 (ARCH-INDEX is canonical subsystem registry), the descriptor text for each row must accurately reflect the subsystem's behavioral contract. A descriptor citing wrong TOML field values misleads the subsystem registry consumer.

**Locus:** ARCH-INDEX.md, ADR-030 row descriptor text.
**Fix:** State-manager: ARCH-INDEX ADR-030 row descriptor updated to reflect corrected canonical shape (`on_error="continue"`, `priority=920`, `tool=` field absent). Changelog row v1.3 added. F-P22-003 superseded note added.

No other new findings on frozen stories S-19.03/S-19.04/S-19.05 or epic.

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PARTIAL — F-P23-002 (BC-4.13.001 tense inconsistency) |
| Dim-2: AC gate execution | Not re-run (frozen stories; no gate changes in D-776 delta) |
| Dim-3: POLICY 14 5-leg parity | PASS on all 5 D-776 delta amendments |
| Dim-4: POLICY 8 BC propagation | PASS |
| Dim-5: Input-hash consistency | PASS — S-19.01 2a9f0b4, S-19.06 5af0d9f match post-D-776 compute |
| Dim-6: STORY-INDEX BC-coverage | PARTIAL — F-P23-002 will require S-19.02 + S-19.07 cite sweep |
| Dim-7: ADR/BC interface parity | FAIL — F-P23-001 (ADR-030 canonical TOML 3 axes wrong); F-P23-003 (ARCH-INDEX row descriptor stale) |

## Observations

**O-P23-001 [process-gap]:** F-P23-001 reveals that ADR "canonical" config snippets can survive many passes with multiple structural axes wrong when reviewers verify internal consistency of the ADR rather than shape-parity against the live registry consumer. The D-776 fix correctly anchored the regex but did not diff the full stanza against the live hooks-registry.toml entry. Recommend: for any ADR that embeds a canonical TOML/YAML stanza, the fix burst must include a literal `diff <stanza-extracted> <live-registry-entry>` gate with captured stdout before committing — pseudocode narrative is insufficient (META-LEVEL-24 applies here too).

**O-P23-002 [spec-hygiene]:** F-P23-002 (BC-4.13.001 tense) is a spec-hygiene issue that does not change any behavior but does affect how the ACs read as invariants vs. post-conditions. The tense fix affects 2 downstream stories; the story-writer sweep is mechanical (replace_all, input-hash recompute). No architectural adjudication required.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 0 |
| Observations | 2 |

Actionable findings: 3. D-777 fix burst applied (architect ADR-030 v1.2→v1.3; product-owner BC-4.13.001 v1.8→v1.9; story-writer S-19.02 v1.9→v1.10 + S-19.07 v1.8→v1.9; state-manager ARCH-INDEX v2.92→v2.93 + STORY-INDEX BC-cite sweep).

**Overall Assessment:** NOT-CLEAN — 3 findings (B0/H1/M2/L0). Streak 0/3. NEXT: pass-24.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 23 |
| New findings | 3 |
| Duplicate/variant findings | 0 |
| Novelty score | 3/3 distinct classes |
| Median severity | MEDIUM |
| Novel observations | 2 |
| Verdict | NOT-CLEAN — streak 0/3; pass-24 NEXT |

## Coverage Attestation

Artifacts read in full: ADR-030 v1.2 (1-end); BC-5.42.001 v1.3 (1-end); BC-1.17.001 v1.3 (1-end); BC-4.13.001 v1.8 (1-end); S-19.01 v1.13 (1-end); S-19.06 v1.14 (1-end); ARCH-INDEX ADR-030 row; STORY-INDEX E-19 section (680-710); policies.yaml POLICY 14+17; hooks-registry.toml SubagentStop entry (ground truth for on_error + priority + field set).
Spot-checked (no changes): S-19.02 v1.9; S-19.03 v1.12; S-19.04 v1.11; S-19.05 v1.13; S-19.07 v1.8; E-19 epic v1.16.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-22; fix-burst records.

## Fix Burst Closure (D-777)

**Leg 1 — Architect:** ADR-030 v1.2→v1.3. Decision 1 canonical TOML stanza corrected: `tool` field removed entirely (not present in live SubagentStop entries); `on_error = "advisory"` → `on_error = "continue"` (advisory-block-mode via stdout outcome); `priority = 150` → `priority = 920` per live hooks-registry.toml. F-P22-003 superseded note added (^Agent$ anchoring was correct; adjacent structural errors were not surfaced by that pass). POLICY 14 5-leg parity applied. **CLOSED F-P23-001 (ADR-030 leg).**

**Leg 2 — Product-owner:** BC-4.13.001 v1.8→v1.9. Tense-only fix: past-tense normative constructions replaced with present-tense imperative throughout §Acceptance Criteria and §Invariants. No behavioral content changed. POLICY 14 5-leg parity applied. **CLOSED F-P23-002 (BC leg).**

**Leg 3 — Story-writer:** S-19.02 v1.9→v1.10 (BC-4.13.001 v1.8→v1.9 cite propagation: §Behavioral Contracts table Version cell, Token Budget, AC traceability, 17 sites total; input-hash 6beeac8→ccd11cf); S-19.07 v1.8→v1.9 (BC-4.13.001 v1.8→v1.9 cite propagation: §Behavioral Contracts table Version cell, Token Budget, AC traceability, 12 sites total; input-hash 46c2ffa→01bed1d). POLICY 14 5-leg parity applied to both. **CLOSED F-P23-002 (SW leg).**

**Leg 4 — State-manager:** ARCH-INDEX v2.92→v2.93: ADR-030 row descriptor corrected (`on_error="advisory"` → `on_error="continue"` advisory-block-mode via stdout outcome; `priority=150` → `priority=920`; `tool=^Agent$` field removed; F-P22-003 superseded; Changelog row v1.3 appended). **CLOSED F-P23-001 (ARCH-INDEX leg) + F-P23-003.** STORY-INDEX v4.154→v4.155: BC-4.13.001 v1.8→v1.9 cite sweep (BC-coverage footer line 702). 4-index bumps: BC-INDEX v3.78→v3.79 / VP-INDEX v2.53 UNCHANGED / STORY-INDEX v4.154→v4.155 / ARCH-INDEX v2.92→v2.93.

4-index after D-777: BC-INDEX v3.79 / VP-INDEX v2.53 UNCHANGED / STORY-INDEX v4.155 / ARCH-INDEX v2.93. Streak 0/3. **NEXT: pass-24.**
