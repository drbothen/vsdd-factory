---
document_type: review-report
level: ops
version: "1.0"
status: complete
producer: spec-reviewer
verifier: orchestrator
timestamp: 2026-05-18
phase: m3-bc-cascade-pass-1
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
input-hash: "0901064"
traces_to: STATE.md
---

# Spec Review — BC-5.39.007 + BC-5.39.008 (M3 BC Cascade Pass-1)

## Verdict

**SUGGESTIONS_ONLY — Both BCs READY_FOR_ADVERSARY without P1 remediation.**

No blocking (P1) findings in either BC. All findings are P2 (routing suggestions) or P3 (nice-to-haves). Adversary cascade may proceed immediately.

---

## BC-5.39.007 — validate-closes-completeness Phase 1

**Overall Assessment:** The BC is well-structured and addresses a real gap (BC-5.39.006's mechanical gate enforcement). Coverage of Postconditions and Error Conditions is thorough. The Phase 1 / Phase 2 split is architecturally sound per ADR-022.

### P2 Findings (routing suggestions — resolve before story-writer dispatch, not before adversary)

**SR-BC007-P2-001 — Clarity: PC2 description could be tighter**
- PC2 currently describes the marker detection logic in prose that partially overlaps with EC-018 behavior. Recommend splitting: PC2 = presence of `trajectory-tail ` marker; EC-018 = behavior when marker absent.
- Routing: product-owner to consider at next BC amendment.

**SR-BC007-P2-002 — Consistency: EC-019 regex pattern not cited verbatim**
- EC-019 references the `→[0-9]+` detection logic but does not provide the regex pattern verbatim. Other BCs in the 5.39.xxx family (BC-5.39.006) provide literal regex anchors. Recommend adding the canonical regex to EC-019.
- Routing: product-owner.

**SR-BC007-P2-003 — PC2 split recommendation**
- PC2 combines two distinct postconditions: (a) marker present → non-empty arrow-count sequence detected; (b) arrow-count sequence LENGTH == 4. These could be separate PCs for cleaner verification mapping. If left combined, the BC should note explicitly that both sub-conditions are required.
- Routing: product-owner to decide at v1.1.

**SR-BC007-P2-004 — HookResult::Advisory variant ambiguity**
- Phase 1 behavior is described as returning `HookResult::Advisory` in some places. However, `HookResult::Advisory` is a formal SDK variant. The adversary should verify whether this variant exists in `hook-sdk` or whether Phase 1 returns a block-false advisory-message result. Cross-reference needed against BC-5.39.008's formal `HookResult::Advisory` usage.
- Routing: architect to confirm SDK variant existence before story-writer dispatch.
- **Note for adversary:** This cross-cutting issue is flagged in the Cross-Cutting section below for independent probing.

### P3 Findings (nice-to-haves)

**SR-BC007-P3-001 — Token budget PC citation**
- Token budget references PC2 and PC3 but not PC1. If PC1 (basic hook invocation) is always trivially satisfied, note that explicitly.

**SR-BC007-P3-002 — EC-017 row length**
- EC-017 row is longer than sibling rows. Consider wrapping or compressing for readability.

**SR-BC007-P3-003 — Frontmatter bcs array synchronization note**
- Frontmatter `bcs` array and body BC table show BC-5.39.007. Verify these are synchronized if any amendment adds sub-BCs.

**SR-BC007-P3-004 — Phase 2 reserved section gate note**
- Phase 2 is described as reserved per ADR-022. Consider adding a brief one-line note explaining the ADR-022 gate condition so future readers understand the trigger.

---

## BC-5.39.008 — validate-policies-schema

**Overall Assessment:** Strong coverage of the policies.yaml validation problem space. The three-part structure (Part A schema, Part B cargo-audit, Part C advisory escalation) is well-designed. ADR-021 Option (b) alignment is correctly incorporated.

### P2 Findings (routing suggestions)

**SR-BC008-P2-001 — PC7 lint_hook/codified_at coupling rationale missing**
- PC7 requires both `lint_hook` and `codified_at` to be validated together. The coupling rationale (why these two fields are co-required) is not explained in the BC body. A one-sentence rationale would prevent implementer questions during story elaboration.
- Routing: product-owner.

**SR-BC008-P2-002 — Invariant 5 severity-enum union needs explicit definition**
- Invariant 5 references a severity enum. The union of permitted values is not listed explicitly in the BC. Recommend adding a canonical list (CRITICAL, HIGH, MEDIUM, LOW, NITPICK) or citing the source file where the enum is defined.
- Routing: product-owner or architect.

**SR-BC008-P2-003 — PC2 frontmatter scope note**
- PC2 covers schema validation scope. A note clarifying whether PC2 applies to all top-level keys or only declared-mandatory keys would prevent ambiguity during TDD test authoring.
- Routing: product-owner.

**SR-BC008-P2-004 — EC-021 Part-C multi-advisory batch vs per-finding behavior**
- EC-021 describes behavior when multiple advisories are found. The BC does not specify whether advisories are emitted as a batch or individually. Batch vs per-finding affects the hook result format.
- Routing: product-owner and architect (protocol question).

### P3 Findings (nice-to-haves)

**SR-BC008-P3-001 — Multi-segment slug validation note**
- The BC validates `lint_hook` field slug format. If multi-segment slugs (e.g., `foo:bar`) are valid, the BC should note this explicitly. ADR-021 Option (b) selected cargo-audit-at-runtime; the slug format for Part B hooks may differ.

---

## Cross-Cutting Finding (Routes to Architect Before Story-Writer Dispatch)

**SR-XCUT-001 — HookResult variant taxonomy clarification needed (MEDIUM priority)**

BC-5.39.007 uses the phrase "ADVISORY in Phase 1" in prose, implying Phase 1 returns an advisory result. BC-5.39.008 uses the formal `HookResult::Advisory` variant identifier. Before story-writer elaborates M3 stories, the architect must confirm:

1. Does `hook-sdk` expose a `HookResult::Advisory` variant? (Check `crates/hook-sdk/src/lib.rs` or equivalent.)
2. If yes: BC-5.39.007 should cite the variant formally, not in prose.
3. If no: BC-5.39.008's PC references to `HookResult::Advisory` are unimplemented-feature references that need ADR or BC amendment.

**This does NOT block adversary cascade.** The adversary should probe this cross-cutting consistency question independently.

---

## Summary

| BC | P1 Blockers | P2 Routing | P3 Nice-to-have | Adversary Ready |
|----|-------------|------------|-----------------|-----------------|
| BC-5.39.007 | 0 | 4 | 4 | YES |
| BC-5.39.008 | 0 | 4 | 1 | YES |

**Both BCs ready for adversary cascade. Adversary dispatch authorized.**

---

*Note: Orchestrator verified via literal shell that the `m3-bc-cascade/` subdirectory path is not in artifact-path-registry.yaml canonical patterns. This file is placed at cycles/v1.0-brownfield-backfill/ (registered `cycle-document` pattern). The D-482 burst-log entry reflects this path correction.*
