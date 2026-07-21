# PR #742 — Security Triage: Enforcement-Weakening Sign-Off

**PR:** #742 — `fix(adr-032): payload-targeted timestamp enforcement + bats placement tests`
**Branch:** `fix/adr-032-timestamp-hook-edit-enforcement` (worktree `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/adr-032-timestamp-hook`)
**Reviewer role:** security-reviewer (T2 — classify and adjudicate; read-only)
**Scope:** Findings 2 and 3 from pr-reviewer report (enforcement-weakening items requiring security sign-off)
**Spec reviewed:** ADR-032 v1.13 (accepted 2026-07-20 via strict 3-CLEAN BC-5.39.001 convergence: passes 9/10/11 CLEAN B0/H0/M0/L0 by independent fresh-context adversaries at frozen commit bc7f6d8b); ADR-025 v1.24.

---

## Threat model

Single-developer factory. `STATE.md` is written exclusively by the `state-manager` agent; no concurrent writers exist in any supported deployment configuration. The `factory_lock` is an advisory concurrency control whose purpose is intra-session burst sequencing, not multi-process mutual exclusion. ADR-025 Decision 7 ("efficiency-class lock") and BC-5.40.001 PC6 ("single-developer zero-friction") establish that fail-open behavior on lock self-interaction is the intended design. CWE-362 (concurrent execution with improper synchronization) describes the class of race that this lock defends against; in a single-dev environment with a cooperative agent as the sole writer, the residual risk in that class is near zero.

---

## Finding 2 — ADVISORY (enforcement-weakening): body Edit while expired factory_lock passes guard

**Pr-reviewer severity:** ADVISORY (enforcement-weakening)
**Security classification:** LOW / CWE-840 (Business Logic Errors — weakened but disclosed enforcement gate)

### What changed

`verify-state-timestamp-refresh` now returns `Continue` for any Edit/MultiEdit whose `new_string` sets neither `timestamp:` nor `factory_lock:` at column 0 (payload-neutral). The confirmed consequence, via test `ac020_edit_body_lock_held_no_factory_lock_continues` (a Red Gate test that is explicitly designed to pass post-fix), is that a body-only Edit made while a held `factory_lock` is expired now returns `Continue` where it previously returned `Block(TimestampStale)`.

### Adjudication: ACCEPTED-BY-SPEC

The behavior is disclosed and accepted. Exact clause citations:

1. **ADR-032 §Consequences Negative, bullet 1** (verbatim): *"Slight weakening of the per-Edit guarantee: a burst that issues only body-only Edits (zero explicit timestamp Edit) and then commits will pass through the hook without a TimestampStale block. Under the prior design, the final body-only Edit would have blocked (its reconstructed content compared against the same stale on-disk timestamp)."* This sentence exactly describes the scenario the reviewer flags: a body Edit previously blocked; post-fix it passes.

2. **ADR-032 §Consequences Negative, mitigations (a)–(c)**: The spec explicitly enumerates three compensating controls: (a) state-manager process discipline requires an explicit timestamp-advancing Edit per burst; (b) the Write-tool path enforces unconditionally; (c) the `verify-state-timestamp-advisory` PostToolUse advisory (AC-021) fires at commit time on `state_md_in_commit == true` commits, detecting a burst committed without timestamp advancement.

3. **ADR-032 Decision 1** (normative): *"If NO new_string in the payload sets EITHER timestamp: OR factory_lock: (payload-neutral — neither field is set), the guard skips Steps 4–7 entirely, then returns Continue."* This is the operative decision; the lock-expiry case under Step 7 is within scope of "Steps 4–7 skipped."

4. **ADR-025 §12.3 ADR-032 supersedes-in-part annotation (v1.23)**: *"If no new_string in the payload sets EITHER timestamp: OR factory_lock: (payload-neutral — neither field), the guard returns Continue after skipping the timestamp and lock-expiry checks, bypassing all §12.3 rows for that invocation."* The annotation was reviewed and corrected across adversary passes 6R and 7R and is current.

5. **`ac020_edit_body_lock_held_no_factory_lock_continues` is an ADR-032 Red Gate test** (explicitly designated as such in the Required new tests table, ADR-032 §Implementer Work Spec): it must fail against the unmodified guard and pass only after the fix. A Red Gate test is a contract that the spec intends this behavior change. Its presence in the passing test suite is the acceptance signal.

### Reviewer's concern about AC-021 compensation gap

The pr-reviewer correctly notes that AC-021 does not cover the "stale-lock-body-edit" case specifically. This is accurate: AC-021 keys off whether the factory-artifacts HEAD commit advanced `timestamp:` vs HEAD^ — it catches "burst committed without timestamp advance." It does not separately signal "Edit was made while lock was expired." However, this is not a gap relative to the specified invariant. The ADR-025 §12.2 invariant is *"every STATE.md commit must advance timestamp:"* — this is a per-commit, not per-Edit invariant. An expired lock does not change that invariant. State-manager process discipline holds the remainder: the burst's timestamp-advancing Edit still goes through full enforcement (pre-condition: `sets_timestamp == true` → Steps 4–7 run). The expired-lock-body-edit is a relaxation of an over-constraint (the pre-fix spurious `TimestampStale` block), not a relaxation of a genuine safety invariant.

### Threat model fit

The pr-reviewer's failure scenario states *"concurrent writer may hold the lock."* This contradicts the established threat model. In a single-dev factory, there is no concurrent writer; the `factory_lock` is advisory coordination between sequential agent calls in the same session, not a mutual-exclusion primitive against external processes. Under the actual threat model, an expired lock signals that the prior session terminated abnormally — the factory is in a recovery state, and body edits proceeding is operationally correct behavior.

### Verdict: ACCEPTED-BY-SPEC

**Cited clauses:** ADR-032 §Consequences Negative bullet 1; mitigations (a)–(c); Decision 1; ADR-025 §12.3 ADR-032 annotation v1.23. No spec amendment required.

---

## Finding 3 — MINOR (enforcement-gap): timestamp-deletion Edit classified payload-neutral

**Pr-reviewer severity:** MINOR
**Security classification:** LOW / CWE-693 (Protection Mechanism Failure — guard no longer blocks a specific anomalous Edit class)

### What changed

Pre-fix: an Edit where `old_string` contains the `timestamp:` line and `new_string` replaces it with body text (or empty string) causes the reconstructed proposed content to have no `timestamp:` field. The §12.3 row "timestamp: absent in proposed content → Block: TimestampStale" fires. Post-fix: `sets_timestamp = false` (the `new_string` does not set `timestamp:`) → payload-neutral → `Continue`. The guard no longer blocks this deletion pattern.

### Important correction to the pr-reviewer's stated compensating control

The pr-reviewer states: *"the guard blocks the NEXT timestamp-setting edit if stale."* This is **inaccurate for the deletion sub-case**. After a committed STATE.md with no `timestamp:` field, the on-disk content lacks `timestamp:`. The §12.3 row "timestamp: absent in on-disk content → Continue" applies to the NEXT Edit that explicitly sets `timestamp:`. That Edit would pass regardless of whether the value it sets is stale, because the absent-in-on-disk row exits early with Continue. The downstream enforcement is therefore weaker than the pr-reviewer described: the deletion path could persist across a commit boundary before the guard regains traction (at the next Write, which enforces unconditionally per Decision 2). The git-commit advisory (AC-021) also does not cover this case: after a committed deletion, `head_state_timestamp` is empty/absent and `head_parent_state_timestamp` has a value — these are NOT byte-identical, so AC-021 does NOT emit an advisory, treating the absent-field commit as an "advancement."

### Adjudication: ACCEPTED-BY-SPEC

Despite the deeper gap than the pr-reviewer described, this remains within the disclosed scope of ADR-032. Reasoning:

1. **The §Consequences Negative "payload-neutral → Continue" disclosure is the operative one.** The bullet states: *"a burst that issues only body-only Edits (zero explicit timestamp Edit) and then commits will pass through the hook without a TimestampStale block."* An Edit where `new_string` does not set `timestamp:` is logically covered by "zero explicit timestamp Edit" — this includes the deletion sub-case (where `old_string` removes the field and `new_string` replaces it with non-timestamp content). The spec does not enumerate every possible Edit payload that lacks `timestamp:` in `new_string`; the general class is "any Edit whose new_string does not set timestamp."

2. **ADR-025 §12.3 ADR-032 annotation explicitly covers the class**: *"If no new_string in the payload sets EITHER timestamp: OR factory_lock: (payload-neutral — neither field), the guard returns Continue after skipping the timestamp and lock-expiry checks."* A deletion Edit (new_string does not set `timestamp:` or `factory_lock:`) satisfies this condition by definition.

3. **Process discipline bounds the risk.** `factory-lock-write.sh acquire` fails loudly with a `SchemaViolation` when `timestamp:` is absent from STATE.md frontmatter. Any subsequent lock-acquisition attempt after a deletion commit would surface the field absence. State-manager operating under VSDD pipeline discipline never issues an Edit whose purpose is to delete the `timestamp:` field; the operation is outside the defined state-manager workflow. The deletion case is a degenerate scenario, not a routine one.

4. **ADR-032 achieved strict 3-CLEAN BC-5.39.001 convergence with 11 passes (8 genuine fresh-context adversaries + 3 integrity events) before acceptance.** The deletion sub-case was within the reviewable scope of every pass that examined the payload-neutral-Continue policy. Independent adversaries at passes 9, 10, and 11 staked the acceptance gate on this design.

### Genuine gap acknowledged

The reviewer's note is correct that pre-fix this class of Edit blocked, and post-fix it does not, with no compensating control at the per-Edit or per-commit level that catches the specific deletion scenario. This is a genuine behavioral regression from the pre-fix design for this specific case. The ADR-032 §Consequences Negative section does not enumerate the deletion sub-case explicitly; it only covers it implicitly via the broader "payload-neutral → Continue" disclosure. A spec amendment to ADR-032 §Consequences Negative explicitly naming the deletion sub-case (old_string contains timestamp, new_string removes it → payload-neutral → Continue; enforcement resumes at next Write or explicit timestamp-setting Edit) would improve spec completeness. However, this is a documentation improvement, not a blocking concern.

The risk is bounded by:
- Single-dev threat model (no concurrent writers to exploit the deletion window)
- `factory-lock-write.sh acquire` SchemaViolation gate on absent `timestamp:` field
- Write-path enforcement remaining unconditional (Decision 2)
- State-manager process discipline (never deletes `timestamp:` in normal operation)

### Verdict: ACCEPTED-BY-SPEC

**Cited clauses:** ADR-032 §Consequences Negative bullet 1 (implicit coverage); ADR-032 Decision 1; ADR-025 §12.3 ADR-032 annotation v1.23. No spec amendment required to unblock the PR. A non-blocking spec note is recommended: enumerate the deletion sub-case explicitly in §Consequences Negative as a known edge case of the payload-neutral-Continue policy. This note should be filed as a follow-up spec refinement (deferred per the accepted residual-risk model), not as a PR blocker.

---

## Overall PR Recommendation (security perspective)

**APPROVE** from the security perspective.

Both Finding 2 and Finding 3 are ACCEPTED-BY-SPEC under the established threat model (single-dev factory, advisory concurrency control, cooperative agent as sole writer). Neither represents an undisclosed enforcement gap that would require spec amendment before merge. The reduced enforcement surface for payload-neutral Edits is the explicit, intended outcome of ADR-032 Decision 1, which was reviewed across 11 adversary passes and accepted via strict BC-5.39.001 3-CLEAN convergence with human-directed STRICT mode.

The remaining blocking concern for PR #742 is Finding 1 (MAJOR, correctness regression in `factory-lock-write.sh _write_factory_lock_block`), which is outside security-reviewer scope and is a correctness/silent-failure matter for the implementer and pr-manager to resolve.

| Finding | Verdict | Severity | CWE | Security recommendation |
|---------|---------|----------|-----|------------------------|
| 2 (stale-lock body Edit passes) | ACCEPTED-BY-SPEC | LOW | CWE-840 (Business Logic) | APPROVE |
| 3 (timestamp-deletion Edit passes) | ACCEPTED-BY-SPEC | LOW | CWE-693 (Protection Mechanism Failure) | APPROVE; recommend non-blocking follow-up spec note for §Consequences Negative |
