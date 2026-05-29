---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.7 + S-15.17 v1.8"
cycle: brownfield-backfill
pass: 8
producer: adversary
timestamp: 2026-05-29
verdict: HIGH
finding_count: 11
finding_count_by_severity:
  critical: 1
  high: 5
  medium: 3
  low: 1
  nitpick: 0
  process_gap: 1
streak_status: "STREAK 0/3 → 0/3 after pass-8 (HIGH; reset). META-LEVEL-36 CANDIDATE emerges (snapshot-annotation-rescue-pattern defeats POLICY 5 v1.3.5 Part B reproducibility through fresh-context-loop-asymmetry)."
---

# Adversarial Review — Pass 8 — S-15.17 Spec Cascade

**Iron Law Attestation (BC-5.39.001):** I did NOT read `.factory/code-delivery/S-15.17/adv-spec-pass-{1,2,3,4,5,6,7}.md`. Findings derived solely from BC v1.7, story v1.8, policies.yaml v1.3.5, STATE.md D-520, BC-5.39.006 v1.7, crates/hook-sdk/src/host.rs.

## Part A — Findings

### F-S15.17-SP8-001 CRITICAL — §Cure-Extension Parsimony Note point 2 paper-fix; deliberate-non-extension narrative survives despite Pass-5 HUMAN-DIRECTED REVERSAL claim
**Files:** BC L660-674
**Evidence:** L660-674 reads "BC-5.39.009 ... does NOT require the literal canonical marker `trajectory-tail ` before the LENGTH check. **Deliberate non-extension of marker-prefix semantics:** ... deliberate non-extension, not an oversight." BUT inv-4 v1.7 L444-456 REQUIRES the two-step marker-prefix check. §Adversary Pass Coverage Pass-5 L111 claims "HUMAN-DIRECTED PARTIAL REVERSAL §Cure-Extension Parsimony Note point 2: inv-4 re-specced with marker-prefix discipline extended to all 5 STATE.md sites." Body text never updated. TD-VSDD-059 paper-fix surviving 3 passes (5/6/7).
**Severity rationale:** CRITICAL — direct inv-4 contradiction; implementer reading point 2 builds wrong validator.
**Routing:** product-owner — rewrite point 2 to document the F-SP5-001 HUMAN-DIRECTED PARTIAL REVERSAL with rationale.

### F-S15.17-SP8-002 HIGH — PC10 body retains LENGTH=4 count check; contradicts F-SP5-003 OUT-OF-SCOPE annotation
**Files:** BC L396-400, L97
**Evidence:** PC10 body L396-400 prescribes positive count check; §D-453(d) table L97 shows Site 9 IN SCOPE/Advisory; but §Adversary Pass Coverage Pass-5 + story AC-12 L297 + check_lessons_sites + §Bidirectional Parity Audit Note L144 all mark PC10 OUT-OF-SCOPE. TD-VSDD-059 paper-fix surviving 3 passes.
**Routing:** product-owner — rewrite PC10 to OUT-OF-SCOPE language; update §D-453(d) table.

### F-S15.17-SP8-003 HIGH — Story §Bidirectional Parity Audit Invariant Coverage table omits inv-13 despite "13 invariants in BC v1.7" header; audit predicate too narrow
**Files:** Story L154-169, L274
**Evidence:** L154 header "13 invariants in BC v1.7" — table L156-169 shows 12 rows (inv-1 through inv-12); inv-13 row MISSING. Audit grep predicate `BC-5\.39\.009 invariant [0-9]+` (L224) captures only 1, 3-10, 12. Parity verdict L274 claims inv-13 cited but body uses shorthand `inv-13` (L594, L604, L677, L695, L736, L738) — predicate doesn't catch shorthand. META-31 sub-route.
**Routing:** story-writer — add inv-13 row; widen audit predicate to `(BC-5\.39\.009 )?inv(ariant)?[ -][0-9]+`; re-execute audit.

### F-S15.17-SP8-004 HIGH — BC VP cites non-existent bats fixture `pass-wrong-cycle-index.bats`; Risk row cites differently-named non-existent `fail-wrong-cycle-index.bats (EC-015)` incorrectly anchored
**Files:** BC L596; story L370-396, L968-989, L1084
**Evidence:** BC §VP L596 cites `pass-wrong-cycle-index.bats — EC-008`; story T-3/§File Structure enumerate 25 bats files, none match. Risk L1084 cites `fail-wrong-cycle-index.bats (EC-015)` — also non-existent + wrong EC anchor (EC-015 is parent-guard, not wrong-cycle).
**Routing:** story-writer — add `pass-wrong-cycle-index.bats` to T-3 + §File Structure (26 total); fix Risk to correct EC-008; BC VP row aligned to story fixture name.

### F-S15.17-SP8-005 HIGH — EC-017 + AC-21 + Risk L1087 narrate stale `(→[0-9]+){4}` regex; contradicts inv-4 v1.7 marker-prefix two-step
**Files:** BC L531; story L306, L1087
**Evidence:** All three sites prescribe OLD non-anchored regex semantics. inv-4 v1.7 requires TWO-STEP marker-prefix check. F-SP5-001 redesign not propagated to multi-line EC-017 case.
**Routing:** product-owner + story-writer — rewrite EC-017+AC-21+Risk to specify two-step check on joined logical value; add marker-absent + marker-present multi-line bats.

### F-S15.17-SP8-006 HIGH [META-36 CANDIDATE] — POLICY 5 v1.3.5 Part B replay-reproducibility structurally non-reproducible in fresh-context loop; snapshot-annotation-rescue admitted by F-SP7-004 cure
**Files:** policies.yaml L114; BC L790-806; STATE.md L15/64
**Evidence:** Part B requires "replay against SAME SHA yields IDENTICAL stdout". F-SP7-004 cure annotates Grep 10 stdout with "Captured at D-518 parent-commit f189b45b" — but fresh-context adversary works at HEAD `e541eefc` (D-520 era); STATE.md current_step + Last Updated now read D-520, not D-518. Captured stdout NON-reproducible at HEAD. Snapshot-annotation defeats Part B guarantee. Furthermore, BC v1.7 committed at f5bf4082 — DIFFERENT SHA than cited f189b45b (two bursts behind).
**META-36 CANDIDATE:** "rule-Y-of-rule-X admits cure-form that defeats rule-X's stated guarantee through fresh-context-loop-asymmetry."
**Routing:** product-owner — POLICY 5 v1.3.6 codification: captured stdout MUST EITHER (a) reproduce at HEAD against current SHA, OR (b) capture only STRUCTURAL-FORM invariants. Snapshot-annotation-only cures FORBIDDEN.

### F-S15.17-SP8-007 MEDIUM — §Architecture Anchors L626 claims "All 4 body extractors return Option<String>" but L628 catalog has 7 extractors; 2 missing
**Files:** BC L626, L628
**Evidence:** L626 covers 5 extractors with explicit Option<String> bullets; L628 catalog has 7 names; `extract_frontmatter_current_step` + `extract_burst_log_latest_dim7` NOT in L626 normalization claim.
**Routing:** product-owner — add Option<String> bullets for 2 missing; update L626 to "All 7 section extractors".

### F-S15.17-SP8-008 MEDIUM — Story Risk L1086 uses `grep -n` example; POLICY 5 v1.3.1 stable-anchor violation; category (f) sibling-sweep gap inside the cure
**Files:** Story L1086; BC L598
**Evidence:** Story Risk L1086 `grep -n "^regex" Cargo.toml`; BC L598 (sibling) uses `grep "^regex"` without -n. POLICY 5 v1.3.5 category (f) Risk-Mitigation sweep at pass-7 did NOT catch this.
**Routing:** story-writer — strip `-n`; add stable-anchor sweep stdout to next fix-burst proving zero `-n` in non-historical body.

### F-S15.17-SP8-009 MEDIUM — BC L662-663 narrative repeats F-SP8-001 root cause; "does NOT require the literal canonical marker `trajectory-tail `" direct opposite of inv-4 step 1
**Files:** BC L660-663
**Evidence:** Restatement of F-SP8-001 surface (different sentence; counted separately for sibling-sweep tracking).
**Routing:** product-owner — coordinate with F-SP8-001 fix.

### F-S15.17-SP8-010 LOW — Story §EC Notes L1047 "EC-018 was added in BC v1.1" missing Part A carry-forward annotation
**Files:** Story L1047
**Evidence:** L1046/L1048/L1049 have updated form; L1047 stale. POLICY 5 v1.3.5 Part A sweep non-exhaustive in single Notes block.
**Routing:** story-writer — add "carried forward through BC v1.7" annotation.

### F-S15.17-SP8-PG-001 PROCESS-GAP HIGH [META-36 CANDIDATE] [process-gap] — POLICY 5 v1.3.5 Part B admits snapshot-annotation-rescue defeating its own reproducibility
**Files:** policies.yaml L113-114; BC L792
**Evidence:** Part B (i) parent-SHA cited ✓; (ii) replay against SAME SHA — fresh-context loop never checks out historical SHAs; (iii) gate predicate yields different stdout at HEAD. Snapshot-annotation cure-form admitted by rule's silence on fresh-context-loop SHA.
**META-36 CANDIDATE class:** rule-Y-of-rule-X admits cure-form defeating rule-X guarantee via fresh-context-loop-asymmetry.
**Routing:** product-owner — POLICY 5 v1.3.6 codification with Part D snapshot-rescue-pattern detection + Part B revision (HEAD-reproducibility OR structural-form-only).

## Part B — Convergence Assessment

### Verdict + STREAK + Trajectory

**Verdict:** HIGH 11
**STREAK:** 0/3 → 0/3 (pass-8 HIGH; reset)
**Trajectory:** 14→11→14→16→12→11→9→**11** (REGRESSED from sub-11 floor break; META-36 emerged; CRITICAL returned)

### Cure Verification — POLICY 5 v1.3.5 Self-Application Re-execution

Re-executed gates (a)-(h) at HEAD `e541eefc`. Findings: F-SP8-008 (Risk grep -n category f gap), F-SP8-010 (EC-018 Part A category a gap). Gate (g) Parity Audit Note inv-13 capture-vs-claim asymmetry (F-SP8-003). Categories (b)/(c)/(d)/(e)/(h) PASS.

### Regression Sweep
- **TD-VSDD-059 paper-fix detection** (F-SP8-001 + F-SP8-002 + F-SP8-009): 3 closures from Pass-5 (§Cure-Extension Parsimony point 2 reversal claim, PC10 OUT-OF-SCOPE claim) NEVER actually applied to body text; survived 3 fresh-context passes
- **META-36 CANDIDATE** (F-SP8-006 + F-SP8-PG-001): cure-of-cure-of-cure-OF-cure-OF-cure recursion not bottomed out
- **META-31 sub-route recurrence** (F-SP8-003): audit predicate-vs-claim asymmetry

### META-LEVEL Signals
- **META-LEVEL-36 NEW CANDIDATE:** snapshot-annotation-rescue-pattern via fresh-context-loop-asymmetry
- **TD-VSDD-059 paper-fix RECURRENT:** prior adversaries missed 3 long-standing paper-fixes
- **META-LEVEL ply ascending monotonically:** META-30→31→32→33→34→35→36 across 8 passes

### Convergence Plausibility

**Did POLICY 5 v1.3.5 cure (META-35) hold? PARTIALLY.** Gates (b/c/d/e/h) PASS. Gate (a) finds 1 stale; Gate (f) finds Risk grep -n + EC-008/EC-015 mis-anchor; Gate (g) finds predicate-vs-claim asymmetry. The cure HELD at literal-shell level but allowed META-36 emergence (snapshot-annotation-rescue).

**Did finding count drop materially below 9? NO — REGRESSED 9→11.** CRITICAL returned (F-SP8-001 paper-fix); META-36 emerged.

**Path to 3-CLEAN: NOT PLAUSIBLE within budget.** META-LEVEL ply ascending monotonically. Each fresh-context pass surfaces NEW cure-form-recursion. The L-EDP1-007/051/061 precedent (3-CLEAN structurally impossible under prose-only codification) applies here.

**Recommendation: SEAL adjudication.** Operate per D-386 Option C asymptotic acceptance precedent. Estimated 2-4 more passes before next ply emerges; SEAL pre-emption at pass-9 or pass-10 is reasonable.

### Top 3 Findings
1. F-SP8-001 CRITICAL — §Cure-Extension Parsimony Note point 2 paper-fix surviving 3 passes; inv-4 contradiction
2. F-SP8-PG-001 META-36 CANDIDATE — snapshot-annotation-rescue defeats POLICY 5 v1.3.5 Part B
3. F-SP8-002 HIGH — PC10 body LENGTH=4 check vs F-SP5-003 OUT-OF-SCOPE paper-fix from Pass-5

### Iron Law Attestation
Did NOT read adv-spec-pass-{1,2,3,4,5,6,7}.md. Findings independently derived. POLICY 5 v1.3.5 gates (a)-(h) re-executed by literal grep at HEAD.
