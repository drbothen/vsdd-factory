---
document_type: adversarial-review
level: ops
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.2 + S-15.17 v1.3"
cycle: brownfield-backfill
pass: 3
producer: adversary
timestamp: 2026-05-28
input-hash: "2dcd390"
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md
  - .factory/cycles/v1.0-brownfield-backfill/lessons.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/STATE.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/validate-policies-schema/src/lib.rs
  - .factory/cycles/v1.0-brownfield-backfill/INDEX.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md
verdict: HIGH
finding_count: 14
finding_count_by_severity:
  critical: 1
  high: 5
  medium: 4
  low: 3
  nitpick: 1
  process_gap: 1
streak_status: "STREAK 0/3 after pass-3 (HIGH verdict; reset)"
---

# Adversarial Review — BC-5.39.009 v1.2 + S-15.17 v1.3 Spec Cascade Pass 3

## Part A — Finding Set

### F-S15.17-SP3-001 — CRITICAL — INDEX.md cycle-path-guard hardcodes PAUSED F5 cycle; runtime gate silently inert on active cycle

**Severity:** CRITICAL | **Confidence:** HIGH

STATE.md frontmatter: `current_cycle: v1.0-brownfield-backfill` (active). BC-5.39.009 PC7/8/9/10 + Precondition 4 + Architecture Anchors hardcode `v1.0-feature-engine-discipline-pass-1/` (PAUSED per cycles/.../INDEX.md `paused_pending_resume: true`). All sibling artifacts in `v1.0-brownfield-backfill/INDEX.md`, `/burst-log.md`, `/lessons.md` EXIST and are edited daily — hook silently skips them. SAME META-LEVEL-30 route (b) class at the runtime layer. Sibling BCs (5.39.005/007/008) do NOT hardcode cycle paths.

**Routing:** product-owner (BC). Options: (a) dynamic resolution via STATE.md `current_cycle:` read, (b) regex-pattern guard `[a-z0-9.-]+`, (c) hardcode BOTH cycles. Story T-5 mirror to update.

### F-S15.17-SP3-002 — CRITICAL [regression of F-SP1-005] — `(→[0-9]+){4}` regex matches LENGTH=5; T-5 byte-walk `arrows >= 4` returns TRUE for LENGTH=5; LENGTH=4 STRICT unenforceable

**Severity:** CRITICAL | **Confidence:** HIGH

BC v1.2 inv-4 + EC-018 + AC-22 claim LENGTH=4 STRICT (LENGTH=5 blocks). Actual semantics: `(→[0-9]+){4}` is non-anchored — matches first 4 arrows in `→9→9→9→9→9`. T-5 `arrows >= 4` returns TRUE on LENGTH=5+. Sibling BC-5.39.006 v1.7 inv-6(b) uses different approach: count matches via separate iteration, assert equality `count == 4`. BC-5.39.009 does not. Pass-1 F-SP1-005 closed strictness in narrative only — paper-fix per TD-VSDD-059.

**Routing:** product-owner. Adopt BC-5.39.006 equality semantics OR add negative-lookahead `(?<!→[0-9])(→[0-9]+){4}(?!→[0-9])`. Update T-5 byte-walk to count + assert equality.

### F-S15.17-SP3-003 — HIGH — BC PC11 + EC-004 + Test Vector + VP cite `HostError::TooBig` — variant DOES NOT exist in SDK; actual variant is `HostError::OutputTooLarge`

**Severity:** HIGH | **Confidence:** HIGH

crates/hook-sdk/src/host.rs:80-94: HostError variants are `CapabilityDenied`, `Timeout`, `OutputTooLarge`, `InvalidArgument`, `Other(i32)`. No `TooBig`. BC cites `TooBig` in PC11, EC-004, Test Vector row, VP description. Story EC-004 mirrors. Sibling validate-policies-schema (S-15.15) does not distinguish — pattern matches `Err(e)` uniformly per inv-10.

**Routing:** product-owner. Either replace 4 BC sites + 1 story site with `OutputTooLarge`, OR collapse PC11 into PC12 (any HostError → fail-open per inv-10) — eliminates redundant PC.

### F-S15.17-SP3-004 — HIGH [partial-fix-regression] — Story body line 109/112 still cite `BC-5.39.009 v1.0 ACTIVE`; three BC version bumps did not propagate to story body prose

**Severity:** HIGH | **Confidence:** HIGH

Story line 109: `**BC ID:** BC-5.39.009 v1.0 ACTIVE`. Line 112: "codified in BC-5.39.009 v1.0". BC actual state: v1.2 draft. Frontmatter is correct (lines 37, 65, 213, 804, 829 cite v1.2); body prose is two versions stale. Standing Rule 3 §1 partial-fix-regression discipline.

**Routing:** story-writer. Replace v1.0 ACTIVE → v1.2 (draft; POL-14 promotion on merge); v1.0 codified → v1.2.

### F-S15.17-SP3-005 — HIGH — §Bidirectional Parity Audit Note grep target includes the audit's own stdout block; "PC6: 1" is self-reference; PC6 structurally orphaned; META-LEVEL-31 cure is partial false-green

**Severity:** HIGH | **Confidence:** HIGH

Story line 123 grep targets entire story including embedded audit block. Story line 148 contains literal `BC-5.39.009 PC6` text (the audit's own stdout). Story line 174 claims "PC6=1 cited by AC-7" but AC-7 (line 191) traces to invariant 8, NOT PC6. Independent grep: `grep -c "BC-5.39.009 PC6\b" S-15.17.md` returns 1 (the audit's own line). META-LEVEL-31 cure satisfies syntactic POLICY 8 letter but breaks semantic spirit.

**Routing:** story-writer + policies.yaml amendment. Either (a) audit grep excludes audit block, (b) audit lists AC numbers per PC explicitly. PC6 needs real AC trace (extend AC-7 to anchor PC6 + invariant 8).

### F-S15.17-SP3-006 — HIGH — T-5 pseudocode `Ok(c) => c` returns `Vec<u8>` but section extractors expect `&str`; missing mandatory `String::from_utf8` decode step from sibling pattern

**Severity:** HIGH | **Confidence:** HIGH

T-5 line 492-501: `match host::read_file(...) { Ok(c) => c, Err(e) => ... }`. Returns `Vec<u8>`. `check_state_md(&content)` passes `&Vec<u8>` to section extractor signatures expecting `&str`. Sibling validate-policies-schema/src/lib.rs:1142-1153 uses double-match `Ok(bytes) => match String::from_utf8(bytes) { ... }` for UTF-8 decode with fail-open on decode failure. T-5 misses this. Would not compile.

**Routing:** story-writer. Update T-5 to double-match sibling pattern. Add EC for invalid UTF-8 → log_warn + Continue.

### F-S15.17-SP3-007 — MEDIUM — Story narrative scope (line 105 ".factory/cycles/") vs BC constraint (single hardcoded cycle path) — internal contradiction

**Severity:** MEDIUM | **Confidence:** HIGH

Story line 105 implies broad coverage; T-5 + BC Precondition 4 hardcode narrow. Related to F-001 but distinct (narrative-vs-constraint coherence within spec).

**Routing:** story-writer (after F-001 closure). Sync narrative to actual coverage scope.

### F-S15.17-SP3-008 — MEDIUM — STATE.md parent-guard `starts_with(".factory/")` not robust to path normalization (Windows backslash, `./` prefix, etc.)

**Severity:** MEDIUM | **Confidence:** MEDIUM

BC Precondition 4 documents 2 string-form checks (relative + absolute). Misses normalized variants. `Path::new(file_path).components().any(|c| c.as_os_str() == ".factory")` is platform-independent and robust.

**Routing:** product-owner. Update Precondition 4 + EC-015 + EC-019 to component-walk form.

### F-S15.17-SP3-009 — MEDIUM — Story `cycle: brownfield-backfill` vs body D-453(d) anchor in `v1.0-feature-engine-discipline-pass-1/decision-log.md`; dual-cycle attribution unusual for BC family

**Severity:** MEDIUM | **Confidence:** HIGH

Cross-cycle anchors are technically acceptable per `cycle` field semantics but unusual for BC-5.39.005..009 family (siblings all single-cycle). Cycle-path guard hardcoding F5 + story housed in brownfield = confusing dispatch.

**Routing:** product-owner + story-writer (coordinated). Either (a) move story to F5 cycle, (b) expand cycle-path guard to both, (c) document dual-cycle attribution explicitly in narrative.

### F-S15.17-SP3-010 — LOW — §Bidirectional Parity Audit Note EC inventory framing — fourth grep returns 19 (BC EC count) but does not constitute parity check

**Severity:** LOW | **Confidence:** HIGH

Audit grep is inventory-only, not parity. Minor framing improvement.

**Routing:** story-writer. Re-label as inventory.

### F-S15.17-SP3-011 — LOW — BC v1.2 lists ADR-018 twice in non-D-NNN-references section (lines 446-447)

**Severity:** LOW | **Confidence:** HIGH

Duplicate cite created by additive F-SP2-009 fix (ADR-021 drop). Collapse to single line.

**Routing:** product-owner.

### F-S15.17-SP3-012 — LOW [process-gap pending intent verification] — `last_amended` text-prefix accumulation pattern unbounded across versions; POLICY 14 doesn't specify accumulation vs replace

**Severity:** LOW | **Confidence:** MEDIUM

Pattern not documented in POLICY 14. Sibling pattern consistent (accumulating). May be intentional. Process-gap tag for future codification.

**Routing:** [process-gap — orchestrator to surface at next codification].

### F-S15.17-SP3-013 — NITPICK — BC line 60 D-411(a) cite duplicated at D-NNN table line 443

**Severity:** NITPICK | **Confidence:** HIGH

Trivial duplication. No-change-required acceptable.

**Routing:** product-owner (optional).

### F-S15.17-SP3-014 — [process-gap] PROCESS-GAP HIGH — POLICY 8 extension does NOT guard against audit-stdout self-counting; META-LEVEL-31 cure has structural hole

**Severity:** PROCESS-GAP | **Confidence:** HIGH

policies.yaml:153 POLICY 8 extended verification_step does not specify audit-block exclusion in parity grep. F-SP3-005 demonstrates self-counting failure mode. META-LEVEL-31 cure has its own META-31-class structural escape hatch — "cure-of-cure recursion".

**Routing:** policies.yaml amendment + L-S-15.17-SP3 lesson. Add: audit grep target MUST exclude audit block OR audit MUST list AC numbers per PC explicitly. Add regression fixture demonstrating self-counting failure.

## Part B — Convergence Assessment

### Verdict: HIGH | Streak: 0/3 reset

Trajectory: pass-1 14 → pass-2 11 → pass-3 14. REGRESSING.

### Regression-class findings

- F-SP3-002 [regression of F-SP1-005]: LENGTH=4 STRICT closure was paper-fix (TD-VSDD-059); implementation reference does not deliver.

Other pass-1+2 closures verified intact: F-SP1-002, F-SP1-004, F-SP1-007, F-SP1-009, F-SP1-010, F-SP2-002, F-SP2-003, F-SP2-007 (with caveat F-SP3-008 robustness gap), F-SP2-009.

### POLICY 8 extension audit application

PARTIAL. §Bidirectional Parity Audit Note exists with literal-shell stdout (syntactic POLICY 8 satisfied). BUT audit self-counts own stdout (PC6 inflated from 0 to 1; AC-7 actually traces to invariant 8 not PC6). META-LEVEL-31 cure satisfies letter, breaks spirit. POLICY 8 amendment needed per F-SP3-014.

### META-LEVEL signals

- **META-LEVEL-31 sub-route SURFACED:** "audit-stdout-self-counts-as-citation" — cure has its own META-31 escape hatch.
- **META-LEVEL-X candidate (runtime-gate-targeting-stale-anchor):** F-SP3-001 cycle-path defect is META-30 route (b) class at implementation layer.
- **Cure-of-cure recursion:** Pass-3 surfaces gaps in pass-2 cures (POLICY 8 ext). Rate of new defects > rate of closure.
- **Spec-vs-Implementation drift root:** F-002 (regex semantics), F-003 (HostError variant), F-006 (Vec<u8>/String) all point to: BC narrative written without grounding in actual SDK contract. Recommend BC authoring process require literal-shell grep of any cited SDK symbol with captured stdout — analogous to POLICY 15 LL-3.

### Convergence plausibility

Pass-3 HIGH (1C+5H+4M+3L+1N+1PG). 2 new CRITICALs prior passes missed. Forecast:
- Pass-4: PO+sw fix-burst → likely HIGH→MEDIUM residual (4-7 findings).
- Pass-5: MEDIUM→LOW.
- Pass-6: potential CLEAN.
- 3-CLEAN: optimistic pass-8, pessimistic pass-10.

Critical risk: cure-of-cure recursion. Each cure layer introduces new escape hatches at increasing META levels.

**Recommendation to orchestrator:** Pause cascade. Surface trajectory + CRITICALs + cure-of-cure pattern for human direction. Continuing without addressing the structural risk (BC authored without SDK grounding) may not converge within budget.

### Top findings

1. F-SP3-001 CRITICAL: cycle-path-guard targets paused cycle — silently inert on active.
2. F-SP3-002 CRITICAL [regression]: regex LENGTH semantics — STRICT unenforceable.
3. F-SP3-005 HIGH: audit self-counting — META-31 cure partial false-green.

### Process-gap findings

- F-SP3-014: POLICY 8 extension audit-self-counting structural hole.
- F-SP3-012: POLICY 14 last_amended accumulation convention undocumented.

---

Pass-3 review COMPLETE. STREAK 0/3 reset. Convergence at risk; orchestrator decision required.
