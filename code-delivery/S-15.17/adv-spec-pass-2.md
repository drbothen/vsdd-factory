---
document_type: adversarial-review
level: ops
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.1 + S-15.17 v1.2"
cycle: brownfield-backfill
pass: 2
producer: adversary
timestamp: 2026-05-28
input-hash: "7fe95f1"
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
  - .factory/STATE.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md
  - .factory/policies.yaml
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/validate-policies-schema/src/lib.rs
  - crates/hook-plugins/validate-state-structure/src/lib.rs
  - .factory/stories/S-15.15-validate-policies-schema.md
verdict: HIGH
finding_count: 11
finding_count_by_severity:
  critical: 0
  high: 3
  medium: 4
  low: 3
  nitpick: 1
  process_gap: 0
streak_status: "STREAK 0/3 after pass-2 (HIGH verdict resets per BC-5.39.001)"
---

# Adversarial Review — BC-5.39.009 v1.1 + S-15.17 v1.2 Spec Cascade Pass 2

## Part A — Finding Set

### F-S15.17-SP2-001 [HIGH] [regression] AC-9/10/11/12 PC mis-anchor cascade — PO's PC6 insertion broke advisory-arm AC anchors by exactly one position

**Severity:** HIGH (regression of pass-1 F-003)
**Confidence:** HIGH (literal-shell evidence; cross-file verified)
**Production-grade lens:** "could degrade silently" — implementer reading AC table will write tests against wrong PC.

BC v1.1 PCs: PC1 frontmatter; PC2 Last Updated; PC3 Phase Progress; PC4 Concurrent Cycles; PC5 Session Resume; **PC6 STATE.md cascade Block** (added v1.1); PC7 INDEX Convergence; PC8 INDEX adv-table; PC9 burst-log Dim-7; PC10 lessons trend-table; PC11 fail-open TooBig; PC12 fail-open HostError; PC13 pass.

Story v1.2 AC table cites:
- AC-9 INDEX.md Convergence → `PC6` (should be PC7)
- AC-10 INDEX.md adv-table → `PC7` (should be PC8)
- AC-11 burst-log Dim-7 → `PC8` (should be PC9)
- AC-12 lessons trend-table → `PC9` (should be PC10)
- AC-17 "postconditions 1-9" → should be "1-10" (undercounts lessons.md PC10)

Story v1.2 changelog claim "all 21 ACs swept against BC v1.1, no additional mis-anchors found beyond the 3 named (AC-14, AC-15, AC-1)" is false. PC6-insertion cascade-effect not propagated.

**Routing:** story-writer
**Recommended fix:** Re-map AC-9 (→PC7), AC-10 (→PC8), AC-11 (→PC9), AC-12 (→PC10), AC-17 ("PC1-10"). **Mandatory:** literal-shell bidirectional parity check (`for each PC in BC, grep ACs for that PC cite; for each AC PC cite, grep BC for that PC`) with captured stdout per POLICY 15 — this is the deliverable, not a verification.

This is a TRUE REGRESSION of F-S15.17-SP1-003.

### F-S15.17-SP2-002 [HIGH] POLICY 6 violation — story claims SS-05 is "validation-hooks subsystem" but ARCH-INDEX Subsystem Registry says SS-05 = "Pipeline Orchestration"

**Routing:** story-writer (or PO if BC must amend)

S-15.17:541-543: "SS-05 owns this story's scope because SS-05 is the validation-hooks subsystem per ARCH-INDEX Subsystem Registry..."
ARCH-INDEX.md:311: "| SS-05 | Pipeline Orchestration | ..."

POLICY 6 (HIGH): canonical SS name SoT is ARCH-INDEX. Free-text narrative claim contradicts SoT.

**Recommended fix:** Rewrite the SS-05 justification paragraph to use the canonical name "Pipeline Orchestration" with rationale for why WASM validation hooks under `crates/hook-plugins/` are anchored to SS-05 by historical convention (BC-5.39.001..009 family precedent).

### F-S15.17-SP2-003 [HIGH] BC + story EC-008 cite "(PC4)" for INDEX.md cycle-path-guard rejection, but PC4 is the STATE.md Concurrent Cycles Block — should cite "Precondition 4"

**Routing:** product-owner (BC SoT) + story-writer (story EC mirror)

BC-5.39.009.md:380 EC-008 + S-15.17:686 EC-008 both cite "(PC4)" for cycle-path discrimination. PC4 is the STATE.md Concurrent Cycles Block. The cycle-path-guard rule is BC Precondition 4 (lines 125-131).

POLICY 4 (MEDIUM) + POLICY 7 (HIGH) — semantic anchor integrity. Pre-vs-Post abbreviation ambiguity.

**Recommended fix:** Replace `(PC4)` → `(Precondition 4)` in both BC EC-008 and story EC-008. Sweep full EC table for similar Pre-vs-Postcondition ambiguity.

### F-S15.17-SP2-004 [MEDIUM] BC frontmatter `status: active` contradicts `lifecycle_status: draft`

**Routing:** product-owner

BC-5.39.009.md frontmatter lines 5 (`status: active`) and 30 (`lifecycle_status: draft`). POL-14 auto-promotion fires on S-15.17 merge — but `status:` is already `active` premature of merge.

**Recommended fix:** Reconcile per BC field-naming convention (likely `status: draft` until POL-14 fires, OR verify `status:` is independent of lifecycle).

### F-S15.17-SP2-005 [MEDIUM] BC PC2/PC3/PC5 literal-shell evidence cites factory-artifacts HEAD 29d08cc7 + line 280, but current HEAD is aa028965 + line 283

**Routing:** product-owner

BC v1.1 PC5 cites `280:## Session Resume Checkpoint (2026-05-28 — D-513 ...)` but current STATE.md has `283:## Session Resume Checkpoint (2026-05-28 — D-514 ...)`. Line numbers drifted on same-day burst. PC2 cites line 57 (now 58).

TD-VSDD-091 anti-volatile-pin: file:line citations in normative narrative are forbidden unless justified.

**Recommended fix:** Strip line numbers from PC2/PC3/PC5 literal-shell blocks; keep grep command + content match excerpt only. Prefix-match semantics are what's load-bearing.

### F-S15.17-SP2-006 [MEDIUM] BC cure-extension claim of BC-5.39.006 is mis-applied — BC-5.39.006 requires canonical marker `trajectory-tail ` prefix before LENGTH check; BC-5.39.009 does not

**Routing:** product-owner

BC-5.39.009.md:153-162 + 314-319 + 508-512 cite BC-5.39.006 inv-6(b) + EC-007 as LENGTH=4-strict precedent. But BC-5.39.006 EC-006/007 condition the LENGTH check on the canonical marker `trajectory-tail ` (with trailing space) being present. BC-5.39.006 EC-023: if marker absent, LENGTH count does not run.

BC-5.39.009 v1.1 PC1 says "any `(→[0-9]+){4}` sequence anywhere in extracted text is present" — fundamentally different semantics.

**Recommended fix:** Replace cure-extension narrative with: "this BC adopts the LENGTH=4-strict invariant from BC-5.39.006 EC-006/007 but does not require the 'trajectory-tail ' canonical marker because per-cell sites 2-9 are heterogeneous text contexts where the marker convention does not apply". Document deliberate non-extension in Cure-Extension Parsimony Note.

### F-S15.17-SP2-007 [MEDIUM] AC coverage gap — false-positive STATE.md case (e.g., `/tmp/STATE.md` outside `.factory/`) untested

**Routing:** product-owner (BC parent-guard amendment) OR story-writer (AC-23 addition)

BC inv-3 example acknowledges `/other/STATE.md` triggers STATE.md arm. AC-16 + EC-015 cover only the negative case (`not-STATE.md`). No AC for false-positive `/tmp/STATE.md` or `/home/user/notes/STATE.md`.

**Recommended fix (production-grade preferred):** PO amends BC to require `.factory/` parent guard for STATE.md; story-writer adds corresponding AC. A WASM hook firing on any STATE.md anywhere on the filesystem and emitting Block-grade exit code is surprising.

### F-S15.17-SP2-008 [LOW] BC PC3 'COMPLETE' skip-list over-inclusive — most Phase Progress rows are `**COMPLETE**`

**Routing:** product-owner

BC PC3 extractor: "skipping any rows whose Status cell contains 'ARCHIVED', 'COMPACTED', or 'COMPLETE'". But STATE.md Phase Progress rows are dominantly `**COMPLETE**`/`**SHIPPED**`/`**MERGED**`/`**CYCLE CLOSED**` — extractor would skip the most recent row (D-514 burst with `→9→9→9→11`).

**Recommended fix:** Drop `COMPLETE` from skip list (keep ARCHIVED/COMPACTED only), OR replace with "bottommost row" (state-manager Commit E discipline appends one row per burst — bottommost IS latest by construction).

### F-S15.17-SP2-009 [LOW] BC ADR-021 cite is out-of-scope (cargo-audit sandboxing, not general host::read_file no-subprocess)

**Routing:** product-owner

BC-5.39.009.md:466 cites "ADR-021 (WASM Cargo-Audit Sandboxing — sandboxed file access model; `host::read_file` only, no subprocess)". ADR-021 is cargo-audit-specific (gates S-15.15 Part C). Generic no-subprocess principle is ADR-018 / hook-sdk contract.

**Recommended fix:** Drop ADR-021 from ADR References, OR verify ADR-021 establishes general no-subprocess principle. Story frontmatter `anchored_adrs: [ADR-017, ADR-018, ADR-021]` inherits.

### F-S15.17-SP2-010 [LOW] BC inv-9 phrasing creates volatile-pin risk if SDK evolves

**Routing:** product-owner

BC inv-9: "There is no `HookResult::Advisory` variant in `crates/hook-sdk/src/result.rs`". If a future PR adds the variant, invariant becomes false. Anti-volatile-pin principle (TD-VSDD-091).

**Recommended fix:** Rephrase as: "Use `HookResult::Continue` + `host::log_warn` for advisory behavior. This hook MUST NOT use any `HookResult::Advisory` variant the SDK may add."

### F-S15.17-SP2-011 [NITPICK] BC Description cites "D-453(d) codified at pass-74" but decision-log heading shows pass-73

**Routing:** product-owner

BC line ~54 says "D-453(d) codified a canonical 9-site mapping table at pass-74". decision-log.md:346: "### D-453 (F5 pass-73 codification block; META-LEVEL-28 CANDIDATE CONFIRMED — ...)".

**Recommended fix:** Update BC to "D-453(d) codified at pass-73".

## Part B — Convergence Assessment

### Verdict: HIGH

STREAK 0/3 after pass-2 (HIGH resets per BC-5.39.001).

- HIGH: 3 (F-001 PC mis-anchor cascade regression; F-002 SS-05 name violation; F-003 PC4-vs-Pre-4 ambiguity)
- MEDIUM: 4 (F-004 status/lifecycle; F-005 stale line numbers; F-006 BC-5.39.006 precedent mis-match; F-007 false-positive STATE.md gap)
- LOW: 3 (F-008 PC3 skip-list; F-009 ADR-021 cite; F-010 inv-9 phrasing)
- NITPICK: 1 (F-011 pass-73 vs pass-74)
- Process-gap: 0

### Regression-class findings

F-S15.17-SP2-001 IS a regression of F-S15.17-SP1-003. The fix-burst closure narrative claim "all 21 ACs swept" was incomplete — story-writer only audited the 3 explicitly-named ACs without re-running full bidirectional AC↔PC parity after PC numbering shifted from PO's PC6 insertion. This is the dominant pass-2 signal.

### META-LEVEL signals

- **META-LEVEL-31 candidate (cascade-propagation-gap-from-PC-insertion):** PC insertion is a structural change to BC; downstream ACs that anchor by PC ordinal are silently shifted. Story-writer's audit did not re-derive the full mapping. Class-3 sibling-sweep gap (TD-VSDD-060 generalization).
- **POLICY 8 verification_steps gap:** covers BC array changes but not PC-ordinal-shift cascade detection. Consider extension to "verify bidirectional AC↔PC parity after any PC insertion/deletion/renumber".
- **POLICY 4 (semantic_anchoring_integrity) pattern recurrence:** F-002 + F-003 + F-009 + F-011 are POLICY 4 instances. Pass-1 closed 2 (ADR-017, EC numbering) but spawned new ones (SS-05 narrative likely new in v1.2).

### Next dispatch routing

HIGH verdict → fix-burst (story-writer + PO) → pass-3.

Recommended sequencing:
1. **Story-writer first (Commit A):** F-001 (AC-9/10/11/12/17 re-anchor + literal-shell bidirectional parity check); F-002 (SS-05 narrative rewrite); F-003 story EC mirror; F-007 AC-23 (if option-A chosen) OR mirror BC parent-guard.
2. **PO (Commit B):** F-003 BC SoT; F-004 status reconciliation; F-005 line-number anti-volatile-pin; F-006 cure-extension precedent rephrase; F-007 BC parent-guard (option-B); F-008 PC3 skip-list; F-009 ADR-021 cite; F-010 inv-9 phrasing; F-011 pass-73.
3. **State-manager (Commit C/E):** D-515 codification + L-S-15.17-SP2-cascade-propagation-gap lesson (META-LEVEL-31 candidate); BC-INDEX v2.55→v2.56; STORY-INDEX v3.73→v3.74; STATE.md advance; POLICY 14 5-leg.

### Convergence plausibility

- Pass-2 (11) vs pass-1 (14) modest improvement, but regression class worsens trajectory.
- Estimate: 6-8 more passes for 3-CLEAN (pass-3..pass-10 budget).
- Critical risk: if pass-3 fix-burst again closes only named findings, regression class recurs.
- Mandate: pass-3 fix-burst MUST include literal-shell bidirectional parity check stdout per POLICY 15.
- No findings require architectural changes (no new BCs, no new ADRs). Recoverable.

### What pass-2 found that pass-1 did not

- Cascade effect of PC insertion (F-001).
- SS-05 subsystem-name narrative violation (F-002) — may be new in v1.1/v1.2 fix-burst.
- Pre-vs-Post anchor ambiguity (F-003) — semantic micro-defect.
- BC-5.39.006 precedent semantic mismatch (F-006) — fresh-context found the marker-prefix difference.

Pass-3 dispatch should emphasize fresh-context audit of FULL package, not just named-closure verification.
