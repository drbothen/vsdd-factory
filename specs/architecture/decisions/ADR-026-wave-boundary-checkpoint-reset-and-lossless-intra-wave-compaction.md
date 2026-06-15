---
document_type: architecture-decision-record
level: L3
adr_id: ADR-026
version: "1.14"
status: accepted
producer: architect
timestamp: 2026-06-14T00:00:00Z
title: "ADR-026: Wave-boundary checkpoint+reset and lossless intra-wave compaction"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
  - issue-173
subsystems_affected:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "All open questions from F1 (OQ-18-001 through OQ-18-004) are resolved by LOCKED DECISIONS D1–D5 confirmed by human prior to F2. Harness-version precondition documented. No remaining human-gated questions. Implementation dispatch ready via E-18 story decomposition."
last_amended: "2026-06-15 (v1.14) — fix burst (architect): (F-P20-002 LOW pending-intent) §Wave-Identity-Derivation third bullet ('For "wave 1 / first wave" discrimination') cross-note added: clarifies that the prior-HANDOFF.md-absence proxy is a CALLER-SIDE substrate used by the shell wave-handoff skill to DERIVE wave_id before writing the payload, and that `payload.wave_id == 1` is the WASM gate's own PAYLOAD-ONLY discriminator (established by F-P10-001/Decision 9). The two substrates are reconcilable: prior-HANDOFF.md absence is the caller's derivation signal → caller embeds wave_id=1 in payload → WASM gate reads `payload.wave_id == 1`. The proxy is not a competing substrate — it is the upstream input from which the caller produces the payload field the gate consumes. §Traceability ARCH-INDEX provenance leg appended: v2.41 to v2.42 (ADR-026 v1.13 to v1.14 F-P20-002 fix). Sibling-strand sweep: VP-081 Precondition already correctly describes the SHELL CALLER / WASM GATE division (no edit needed); BC-4.14.001 already sanctioned by ADR-026 §Decision 9 (no routing needed). Refs: F-P20-002, ADR-026 §Decision 9, BC-4.14.001, VP-081 v1.5. [Prior: 2026-06-15 (v1.13) — fix burst (architect): (F-P19-002 MEDIUM BC-ADR contradiction) §Decision 9 heading retitled from 'no-op on wave-1 / HANDOFF.md absent' to 'no-op on wave-1 / non-HANDOFF.md writes': the prior heading embedded obsolete file-absence proxy framing superseded by F-P10-001 and by VP-083 v1.5 corrected title. §Decision 9 third bullet ('Wave close attempted...Block with HandoffMissing') removed: HandoffMissing is a SHELL wave-gate error code (BC-5.41.001 PC9/EC-014) emitted when wave-gate is invoked to close a wave but HANDOFF.md is absent on factory-artifacts — the check is SHELL-SIDE, performed before any WASM gate invocation; the validate-wave-handoff-completeness WASM gate validates a Write/Edit payload, fires only on HANDOFF.md writes (BC-4.14.001 EC-011), cannot detect file-absence, and NEVER emits HandoffMissing. Third bullet replaced with: (b) non-HANDOFF.md write target no-op bullet per BC-4.14.001 EC-011; (c) explicit 'Semantic boundary — HandoffMissing is SHELL-SIDE' paragraph documenting the disjoint (HandoffMissing=SHELL/file-absent, HandoffIncomplete=WASM/present-but-incomplete), superseding any prior framing that attributed HandoffMissing to the WASM gate. §BC Traceability Cite Convention table BC-4.14.001 row §Decision 9 description updated to payload-only framing and HandoffMissing/HandoffIncomplete boundary. §Risk Mitigations F1-R3 row updated to reflect two distinct WASM-gate no-op conditions and shell-side HandoffMissing attribution. §Traceability ARCH-INDEX provenance leg appended: v2.40 to v2.41 (ADR-026 v1.12 to v1.13 F-P19-002 fix). Comprehensive tree-wide sibling-strand sweep conducted — findings: VP-081 PC-A 'HANDOFF.md absent' label correctly asserts shell-side behavior; BC-4.14.001/BC-5.41.001/BC-INDEX HandoffMissing/HandoffIncomplete attributions all CORRECT; no out-of-ADR edits required. Refs: F-P19-002, BC-4.14.001 EC-011, BC-5.41.001 PC9/EC-014, VP-083 v1.5, E-18, issue-173. [Prior: 2026-06-15 (v1.12) — fix burst (architect): (F-P17-001 MEDIUM) §Decision 2 `precompact_flush_sha` null rule made wave-agnostic for the log-present-and-valid case. Prior rule had a wave-1 unconditional-null carve-out (case 2: 'null when wave_id=1 — permitted unconditionally') that reopened the fabricated/dropped-SHA failure class (DI-021/CAP-032) for wave 1: a hallucinating session could write `precompact_flush_sha: null` at wave-1 close while a genuine flush SHA existed in `precompact-flush-log`. This was asymmetric with wave_id>1 which already HARD-BLOCKed the same scenario. Fix: the three-state uniform null rule now makes the wave_id discriminator survive ONLY for the log-absent case: (1) log genuinely ABSENT — wave_id=1 → null VALID (silent, normal first-wave); wave_id>1 → null ADVISORY only, no hard block; (2) log present but last-line CORRUPT (FIELD-4 absent/empty/≠`commit`) → stale entry → EXEMPT, ANY wave_id (DI-025 arm unchanged); (3) log present with VALID last-line (FIELD-4==`commit`) → null → HARD BLOCK (PrecompactShaMismatch), ANY wave_id INCLUDING wave_id=1. The 'no PreCompact fired before first wave close' unenforced assumption removed from rationale; replaced with explicit statement that mid-wave-1 compaction is a supported Part B scenario. §Decision 2 schema row Anti-fabrication column rewritten; Wave-1 and Genuine-Log-Absence special-cases note rewritten to match three-state rule. WASM gate boundary (BC-4.14.001 pure-parse) is NOT implicated — the null-vs-log check is BC-5.41.001 PC5 shell-side; DI-025 corruption→EXEMPT arm is unchanged. No VP update required (no VP covers the PrecompactShaMismatch null-SHA path by name — routing note provided in fix report). [Prior: v1.11 — F2 adversarial-pass-15 fix burst (architect): (F-P15-002 LOW) §Decision 2 `precompact_flush_sha` Anti-fabrication column de-versioned: load-bearing volatile pin `BC-5.41.001 v1.8 PC2/PC5/EC-006/EC-011` replaced with stable-anchor form `BC-5.41.001 §PC2/PC5/EC-006/EC-011 three-case null rule` per TD-VSDD-091 / POLICY 19. No behavioral change — the three-case logic text is unchanged. [Prior: v1.10 — CV-P14-001 follow-up (architect): §Decision 2 `precompact_flush_sha` schema row and Wave-1 special case note reconciled to BC-5.41.001 v1.8 three-case null-SHA rule. The schema row Anti-fabrication column now encodes the three-case logic: (1) 40-char hex SHA corroborated against precompact-flush-log last-line — VALID; (2) null when wave_id=1 (no PreCompact fired) — permitted; (3) null when wave_id>1 AND precompact-flush-log genuinely absent (verified via `test -f`) — ADVISORY warning, no hard block; (4) null when wave_id>1 AND log EXISTS with valid last-line FIELD-4==commit — HARD BLOCK (PrecompactShaMismatch). Log corruption (FIELD-4 absent/empty/≠commit) → stale entry → prefix-match-alone → EXEMPT (consistent with DI-025/BC-5.41.003/EC-003). Wave-1 note expanded to 'Wave-1 and Genuine-Log-Absence special cases' covering the wave>1 advisory-on-genuine-absence path. §F-P4-004/§Decision A/§Crash-Consistency corruption→EXEMPT and SHA-mismatch→NOT-EXEMPT arms are unchanged. Refs: CV-P14-001, BC-5.41.001 v1.8, E-18. [Prior: 2026-06-14 (v1.9) — F2 adversarial-pass-10 fix burst (architect): (F-P10-001 MAJOR) §Decision 9 wave-1 no-op discriminator corrected to PAYLOAD-ONLY: the validate-wave-handoff-completeness WASM gate computes `is_first_wave = (payload.wave_id == 1)` from the current HANDOFF.md payload being written — NO sprint-state.yaml read, NO factory-artifacts HANDOFF.md read, NO filesystem or git access. Rationale: `wave_id` is Field 1 of the 9 base required fields; the caller (wave-handoff skill) derives it from real substrate (sprint-state.yaml topo-sort ordinal or STATE.md pass number) BEFORE writing HANDOFF.md. The WASM gate reads it back from the Write/Edit tool-call payload. The 'prior HANDOFF.md absence / sprint-state.yaml ordinal' framing was a PROXY for wave-1 that required external reads, violating BC-4.14.001 Invariant 1 (pure-parse). The payload-only `wave_id == 1` discriminator closes the gap: the gate never reads git, filesystem, sprint-state.yaml, or factory-artifacts. The same payload-only principle was established for the EPIC-COMPLETE discriminator by F-P7-001/F-P8-001; F-P10-001 applies it to the sibling wave-1 discriminator. VP-083 `is_first_wave: bool` wording is consistent with this model (`is_first_wave` is computed as `payload.wave_id == 1` by the gate itself). §Decision 9 text amended to document payload-only wave_id==1 discriminator. §F-P10-001 PO Wording section added with exact BC-4.14.001 change spec (rewrite Description, PC3, PC4, PC8, Inv3, EC-001/002/003/005/006/010, test vectors, VP-083/VP-081 row descriptions). (F-P10-002 NOTE) §Decision 6 push-step contract: ADR §Decision 6 step 5 already prescribes `exit 2` on push failure; the 'commit local; push to remote → push failure = exit 2' boundary is now stated explicitly in §F-P10-002 PO Note section to enable PO BC-7.07.001 alignment (BC-7.07.001 PC6/PC8 mislabeling of commit-vs-push is PO-owned). (F-P10-003 MEDIUM) VP-082 append-failure Postcondition E added: VP-082.md v1.3→v1.4 — PC E: 'on precompact-flush-log append failure → if HEAD==SHA_B: git reset --soft SHA_B^ + exit 2; if HEAD≠SHA_B: no reset + exit 2 (human intervention) per §F-P6-006'. VP-INDEX v2.13→v2.14 (VP-082 row description updated, NN-2 POLICY 17 parity). ARCH-INDEX v2.36→v2.37. Refs: F-P10-001, F-P10-002, F-P10-003, issue-173, E-18. [Prior: 2026-06-14 (v1.8) — F2 adversarial-pass-9 fix burst (architect): (F-P9-001 MEDIUM) §F-P5-002 PO Wording PC8 block and §Decision A annotated as SUPERSEDED by §F-P6-006 SHA-pinned form: both `HEAD~1` tokens replaced/annotated — §Decision A live text now prescribes the SHA-pinned `SHA_B^` guard (per §F-P6-006) and labels the HEAD~1 form as historical-only; §F-P5-002 PC8 block given explicit SUPERSEDED-by-§F-P6-006 banner and test-vector note updated to assert `SHA_B^` (not `HEAD~1`) + concurrent-commit path. (F-P9-002 LOW) VP-INDEX VP-082 Full Index row description amended to include append-failure → SHA-pinned-reset (SHA_B^) + concurrent-commit-guard behavior; VP-INDEX v2.12→v2.13. (F-P9-003 MINOR) ARCH-INDEX v2.35→v2.36 ADR-026 v1.7→v1.8 amendment row; §Traceability provenance leg appended (ADR v1.7→v1.8; VP-INDEX v2.12→v2.13). Refs: F-P9-001, F-P9-002, issue-173, E-18. [Prior: 2026-06-14 (v1.7) — F2 adversarial-pass-7 fix burst (architect): (F-P7-001 MAJOR) §F-P6-004 PO Wording BC-4.14.001 change-spec amended: EPIC-COMPLETE discriminator changed from filesystem-read prescription (prior HANDOFF.md on factory-artifacts absent OR non-empty next_wave_stories) to PAYLOAD-ONLY discriminator (current payload next_wave_stories: [] → EPIC-COMPLETE; non-empty → non-EPIC-COMPLETE). BC-4.14.001 Invariant 1 pure-parse constraint satisfied: WASM gate reads only the Write/Edit tool-call payload; no git read, no filesystem read of prior HANDOFF.md. Richer terminal-state judgment (broken-sprint vs genuine final wave) remains in shell-context wave-gate/wave-handoff BC-5.41.002. UnexpectedEpicStatus on non-EPIC payload and MissingEpicStatus on EPIC payload remain coherent with payload-only discriminator. (F-P7-002 MAJOR) §Traceability downstream-index provenance trace completed: VP-INDEX line appended v2.11→v2.12 leg (pass-6 cite-convention migration); ARCH-INDEX line appended v2.33→v2.34 (ADR-026 v1.5→v1.6) and v2.34→v2.35 (ADR-026 v1.6→v1.7 this pass-7 fix). ARCH-INDEX v2.34→v2.35. VP-INDEX unchanged. Refs: F-P7-001, F-P7-002, issue-173, E-18. [Prior: 2026-06-14 (v1.6) — F2 adversarial-pass-6 fix burst (architect): (GOVERNANCE DECISION) BC Traceability ADR cite convention adopted: BC Traceability rows referencing this ADR MUST cite `ADR-026 §Decision N` (stable section anchor) WITHOUT a load-bearing version number. Rationale: the ADR version number is a volatile pin — it increments on every fix-burst while the behavioral content of the cited Decision N is unchanged; carrying it in BCs generates a false-positive finding class each pass (F-P6: BC-5.41.001, BC-5.41.002, BC-5.41.003, BC-6.24.001, BC-7.07.002, BC-1.15.001 all flagged as stale v1.4 while ADR was at v1.5). Under TD-VSDD-091 (anti-volatile-pin), version numbers that decay on every ADR bump are forbidden in load-bearing citations. The stable behavioral anchor is the section heading (e.g., `§Decision 2`, `§Decision 6`), not the version tag. An informational `(as of vX.Y)` parenthetical is PERMITTED only when it is explicitly marked non-load-bearing and is not used as a gate criterion by any validator or adversary check. Uniform PO cite text for all 8 E-18 BCs is specified in §BC Traceability Cite Convention below. (F-P6-006 MAJOR) Concurrent-commit race guard for reset-on-append-failure: Decision 6 step 6 strengthened — the soft reset MUST be pinned to the captured flush SHA, not to a relative `HEAD~1` offset. Root cause: `git reset --soft HEAD~1` assumes HEAD equals the local flush commit (SHA_B); but the PreCompact hook fires between LLM turns, and `factory_lock` is opt-in (absent = no lock). A concurrent state-manager burst could land a commit on factory-artifacts between SHA_B and the attempted reset, advancing HEAD to SHA_C (state-manager commit). `git reset --soft HEAD~1` would then discard SHA_C (silent commit loss, SOUL.md #4 violation — strictly worse than the original orphan problem). Fix: capture SHA_B immediately after `git commit` succeeds; on append failure, reset ONLY if `$(git rev-parse HEAD) == SHA_B`; if HEAD has moved (HEAD != SHA_B), do NOT reset — exit 2 immediately with error message `precompact-flush: append failed after concurrent commit advanced HEAD; SHA_B=<sha>; human intervention required`. Also reconciled the overstated claim that `--soft preserves same staged state across retry`: STATE.md may change between the failed attempt and the retry (the intervening turn may call state-manager), so the retry commits a potentially different tree. The correct statement: `--soft restores the working tree and index to the state they were in when the local commit was created; if STATE.md or other files change between the soft-reset and the retry, the retry commit will include those additional changes`. (F-P6-004 MAJOR) HANDOFF.md schema field-count reconciliation: The schema has 9 base required fields (wave_id, last_verified_develop_sha, active_bcs, next_wave_stories, open_decisions, pending_fixes, process_gaps, precompact_flush_sha, factory_lock_holder). The `epic_status` field is a CONDITIONAL 10th field: required ONLY on the final/EPIC-COMPLETE wave (when `next_wave_stories: []` and all stories are terminal). On non-final waves, `epic_status` must be absent (not `null`, not `complete` — absent). Statements of `all 9 required fields` remain correct for non-final waves. The validate-wave-handoff-completeness WASM gate (BC-4.14.001) must conditionally require `epic_status: complete` ONLY when it detects EPIC-COMPLETE context (no prior pending/draft stories remain). The HANDOFF.md schema table in Decision 2 is amended to add `epic_status` as a conditional field with explicit conditionality rule. BC-4.14.001 and BC-5.41.001 change specs for PO are in §F-P6-004 PO Wording below. [Prior: 2026-06-14 (v1.5) — F2 adversarial-pass-5 fix burst (architect): (F-P5-002 MAJOR) Orphaned un-logged flush commit eliminated via reset-on-append-failure. DECISION: §Decision A in §Crash-Consistency Design now prescribes `git reset --soft HEAD~1` BEFORE `exit 2` when the log append fails. Rationale: without the reset, SHA_B is a real local commit that is never logged (append failed) and never pushed (exit 2 blocked push). On the next retry, SHA_C is committed, logged, and pushed. `validate-burst-log` reads only the LAST LINE of the log (SHA_C); when it sees SHA_B in the HEAD/HEAD^ pair it cannot find SHA_B in the log, and BC-5.41.003 PC1 case (b) fails to grant the exemption → false-positive MULTI_COMMIT_CHAIN_NOT_ALLOWED block. The soft reset restores HEAD to the pre-flush commit, eliminating SHA_B from the local history; the next retry commits SHA_C cleanly. No working-tree changes are lost (--soft). Decision 6 step 6 updated to prescribe: on append failure, reset --soft HEAD~1 then exit 2. (F-P5-002 MAJOR) Stale-entry justification re-grounded. The v1.4 §Crash-Consistency Design (BC-5.41.003 change spec) describes BC-5.41.003 PC1 case (b) `write-before-push crash` as `field-4 ≠ commit`. This justification is mechanically impossible: the shell hook always appends the literal string `commit` (the output of `git cat-file -t <SHA>` on a local commit that already exists) as field-4 at append time; the append succeeds or fails atomically. A partial-write that writes a non-`commit` field-4 would require filesystem-level truncation or corruption — not a `write-before-push crash`. The re-grounded description: field-4 ≠ commit arises ONLY from file corruption or truncation; the fallback (treat as stale, allow bare-prefix match) is still correct but must be attributed to the real cause. BC-5.41.003 PC1 case (b) label changed from `write-before-push crash` to `log corruption or truncation`. §F-P5-002 PO Wording below has exact replacement text for PO. (O-P5-002 MINOR) §Crash-Consistency Design §Decision A subsection heading added: `### Decision A — Append-only side-channel log replaces point-file (F-P2-003; reset-on-append-failure added F-P5-002)`. This provides the stable anchor DI-025 Justification cites as `ADR-026 §Crash-Consistency Design §Decision A`. [Prior: 2026-06-14 (v1.4) — F2 adversarial-pass-4 fix burst (architect): (F-P4-001 MAJOR) Append-failure ↔ exemption incoherence resolved. DECISION: append failure is durability-relevant → BC-7.07.001 PC8 now exits 2 on append failure (not fail-open). Rationale: log append is a local filesystem write that almost never fails; on failure, blocking compaction is correct (the log would be inconsistent, causing a false-positive chain-block on the next burst — the exact regression CAP-032 Part B exists to prevent); fail-open semantics remain ONLY for full hook crashes via on_error=continue. BC-5.41.003 PC1/Inv1 keep the three-gate logic (prefix + last-line SHA match + stale-entry fallback) but the stale-entry case now arises only from push failure (not append failure), so the log is always consistent when compaction proceeds. Decision 6 §step 6 and the §Crash-Consistency Design BC-7.07.001 change spec updated: append failure → exit 2. §VP Allocations note updated: VP-INDEX allocated at v2.08; now at v2.10 (no VP change in this pass). (O-P4-004 MINOR) §Crash-Consistency Design BC-5.41.003 change spec: added explicit statement that WASM corroboration reads the embedded fourth field from the log line and does NOT exec `git cat-file` — the shell hook (which CAN exec git) writes the token at flush time; WASM reads it statically. This is the canonical division: shell execs git, WASM reads the pre-embedded token. PC1/Inv1 replacement wording for PO in §F-P4-004 PO Wording below. (O-P4-001 MINOR) §VP Allocations stale narrative fixed: `VP-INDEX bumped to v2.08` is historical; annotated `(allocated at v2.08; VP-INDEX now at v2.10 — no VP change in this pass)`. [Prior: 2026-06-14 (v1.3) — F2 adversarial-pass-3 COMPLETE-SWEEP fix burst: (F-P3-003 BLOCKER) Decision 2 HANDOFF.md schema `precompact_flush_sha` source column: `last-precompact-flush-sha` → `precompact-flush-log` append-only log last-line SHA field. Decision 6 step 6: redesigned to four-field log line format `<ISO-timestamp> <SHA> <cycle>/<step> <cat-file-type>` with embedded `git cat-file -t` token — enables WASM corroboration without live git exec (O-P3-004 closure). All three TOML `path_allow` blocks corrected: `last-precompact-flush-sha` → `precompact-flush-log` (Decision 6 read_file + write_file + Decision 7 read_file). (F-P3-002 BLOCKER) BC-5.41.003 corroboration spec updated: WASM reads four-field log line, checks SHA (field 2) + embedded `commit` token (field 4) — no live `git cat-file` exec from WASM. (F-P3-001 BLOCKER adjudicated) Terminal-Wave Discriminator expanded: terminal set = `{merged, withdrawn, cancelled}`. `cancelled` added as legitimate terminal state with rationale. BC-5.41.002 v1.2 is already correctly aligned (no PO action needed). ADR-026 now authoritative. (O-P3-001 closure) Pruning ownership attached: S-18.04 MUST include explicit pruning AC for `precompact-flush-log` (no dangling sentence). (O-P3-004 closure) WASM capability constraint documented: `validate-burst-log` and `validate-dispatch-advance` are WASM with no `exec_subprocess`; four-field log format resolves. BC change spec for BC-7.07.001, BC-5.41.003, BC-5.41.001 updated with four-field format. [Prior: 2026-06-14 (v1.2) — F2 adversarial-pass-2 revision: (F-P2-001) Canonical wave-identity derivation expression authored as a single normative §Wave-Identity Derivation section (all BCs/VPs/DIs cite verbatim; eliminates drift between partial re-anchors). (F-P2-002) VP inputs: path corrected from shorthand `ADR-026.md` to real slug `.factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md` across VP-081..VP-086 (TASK 3). VP-081/082/083/085 swept: all `current_wave` references in Preconditions, Invariants, and test-harness fixtures replaced with canonical derivation substrate (see §Wave-Identity Derivation); VP files bumped to v1.2. (F-P2-003) Crash-consistency design for side-channel file: Decision 6 redesigned with append-only side-channel log (`.factory/hooks/precompact-flush-log`) — entries survive hook crash; BC-5.41.003 EC-003 must corroborate against the log, not a single-SHA point file; BC-7.07.001 side-channel write step redesigned as append-only. Exact BC change specs in §Crash-Consistency Design below. (F-P2-004) Terminal-wave discriminator added to Decision 2 and Decision 4: empty `next_wave_stories` AND all stories `merged` or `withdrawn` → EPIC-COMPLETE success path; empty `next_wave_stories` AND stories in other states → hard error (broken sprint-state). BC-5.41.002 EC-001 and BC-6.24.001 must be updated per §Terminal-Wave Discriminator below. [Prior: 2026-06-14 (v1.1) — F2 adversarial-pass-1 revision: (F-1) Re-anchored wave-identity to real substrate; (F-2) Reconciled with ADR-025 opt-in lock model; (F-3) next_wave_stories real derivation specified; (F-4) Bounded timeout_ms=30000; (F-5) Harness-version runtime assertion; (F-6) PostCompact re-anchor best-effort; (F-7) TOML registration corrected; (F-9) SS-08/S-18.06/S-18.07 scope; (F-10) VP-086 allocated; (F-11) 83% clamp MEDIUM-confidence; (F-15) Prerequisite-verification table. [Prior: 2026-06-14 (v1.0) — Initial ADR.]]"
---

# ADR-026: Wave-boundary checkpoint+reset and lossless intra-wave compaction

## Status

**ACCEPTED — all open questions resolved via LOCKED DECISIONS D1–D5 (human-authorized F2 gate). F2 adversarial-pass-6 revision (v1.6) complete. E-18 story decomposition may proceed (F3). Implementation dispatch ready.**

This ADR resolves the architecture for issue #173: enforced wave-boundary checkpoint/reset (Part A), synchronous PreCompact flush for lossless intra-wave compaction (Part B), and the PreToolUse delegation guard (Part C). All four open questions from F1 (OQ-18-001 through OQ-18-004) are answered by the locked decisions recorded here.

---

## BC Traceability Cite Convention (Governance — F2 pass-6)

**This section is a BINDING GOVERNANCE DECISION codified at ADR-026 v1.6. It permanently resolves the recurring stale-ADR-version-cite finding class.**

### Problem

Every adversarial pass flags stale `ADR-026 vX.Y` version tokens in BC Traceability rows. Root cause: the ADR version number increments on every fix-burst (v1.0 → v1.1 → v1.2 → v1.3 → v1.4 → v1.5 → v1.6 across six passes). BC Traceability rows in only the BCs that have behavioral changes in that pass get bumped; the remaining BCs carry stale version tokens and generate false adversary findings.

### Conflict resolution

Under **TD-VSDD-091 (anti-volatile-pin)**: narrative spec content must cite stable behavioral anchors, NOT volatile identifiers that decay on every revision. The ADR version number in a BC Traceability row is a volatile pin — it decays on EVERY ADR bump regardless of whether the cited Decision N's behavioral content changed.

Under **POLICY 5 v1.3.5 (traceability cites must track the current artifact version)**: this policy was designed for artifact-level cites (e.g., BC version numbers in story BC tables), not for inline identifier tokens inside a prose field. Applying it to ADR version numbers within a prose Traceability cell directly conflicts with TD-VSDD-091.

**Resolution: TD-VSDD-091 wins for inline-prose version tokens.** POLICY 5 continues to apply to BC file version numbers, story BC table version cells, and index table version columns. It does NOT mandate carrying volatile inline ADR version tokens inside a Traceability cell prose string.

### Convention (normative)

> BC Traceability rows referencing this ADR MUST cite the stable section anchor `ADR-026 §Decision N` WITHOUT a load-bearing version number token.
>
> An informational `(as of vX.Y)` parenthetical is PERMITTED only when it is: (a) placed AFTER the behavioral anchor text, not before it; and (b) explicitly not load-bearing — no validator, hook, or adversary check may gate on the version token. The informational parenthetical is OPTIONAL and not required.
>
> This convention is ADR-026-specific. It does not change the cite conventions for BCs citing other ADRs or other artifact types.

### Uniform replacement text for all 8 E-18 BC Traceability ADR rows

The PO MUST apply the following exact wording to replace the current load-bearing version-tagged cites. The decision descriptions (behavioral content) are unchanged from the current v1.5 text; only the version token format changes.

| BC | Current (stale pattern) | Replacement text |
|----|------------------------|-----------------|
| BC-1.15.001 | `ADR-026 v1.4 Decision 11 ...` | `ADR-026 §Decision 11 (dispatcher routing verification/addition for PreCompact/PostCompact events; S-18.00; VP-086 allocated for exit-2 propagation verification — introduced v1.1, behaviorally unchanged through v1.6)` |
| BC-4.14.001 | `ADR-026 v1.5 Decision 8 ..., Decision 9 ...` | `ADR-026 §Decision 8 (WASM for completeness gate; deterministic parse-heavy validation; shell for flush), §Decision 9 (no-op on wave-1 / non-HANDOFF.md writes; wave-1 discriminator PAYLOAD-ONLY: payload.wave_id==1; no sprint-state.yaml read, no factory-artifacts probe, no git access; wave_id absent → fail-closed HandoffIncomplete; non-HANDOFF.md write target → no-op; note: HandoffMissing is SHELL wave-gate code in BC-5.41.001 PC9, DISJOINT from this WASM gate which cannot detect file-absence)` |
| BC-5.41.001 | `ADR-026 v1.4 Decision 2 ..., Decision 9 ..., Decision 1` | `ADR-026 §Decision 2 (HANDOFF.md schema + anti-fabrication cross-checks; wave_id from real substrate; factory_lock_holder nullable; precompact_flush_sha hard cross-check against side-channel file; 9 base required fields + epic_status conditional on EPIC-COMPLETE wave per v1.6), §Decision 9 (wave-1 no-op), §Decision 1 (wave-boundary reset is primary mechanism)` |
| BC-5.41.002 | `ADR-026 v1.4 Decision 4 ...` | `ADR-026 §Decision 4 (wave-state.yaml curated manifest; RAG explicitly deferred; next_wave_stories derived from sprint-state.yaml status:pending/draft entries + dependency-order; empty list = hard error)` |
| BC-5.41.003 | `ADR-026 v1.5 Decision 10 ...` | `ADR-026 §Decision 10 (PreCompact flush lifecycle distinct from state-manager burst lifecycle; validate-burst-log + validate-dispatch-advance must exempt commits with PreCompact flush prefix + SHA corroboration via precompact-flush-log last-line fourth-field commit token; WASM reads embedded field, does not exec git; exemption is case-sensitive; stale-entry label is log corruption or truncation, not write-before-push-crash per F-P5-002)` |
| BC-6.24.001 | `ADR-026 v1.4 Decision 3 ..., Decision 4 ...` | `ADR-026 §Decision 3 (prompt-the-human; operator clears session), §Decision 4 (curated wave-state.yaml manifest; RAG deferred; reads from factory-artifacts via git; working-tree not authoritative)` |
| BC-7.07.001 | `ADR-026 v1.5 Decision 6 ...` | `ADR-026 §Decision 6 (PreCompact shell hook; hermetic; blocking; fail-open on crash; lock renewal no-op when lock absent; on_error=continue; reset --soft pinned to captured SHA before exit 2 on append failure; concurrent-commit guard: only reset if HEAD == captured SHA_B per F-P6-006)` |
| BC-7.07.002 | `ADR-026 v1.4 Decision 7 ...` | `ADR-026 §Decision 7 (PostCompact re-anchor: advisory shell hook; best-effort; cannot block; NOT in CAP-032 guarantee chain; re-reads STATE.md pointer from factory-artifacts; emits re-anchor block; does not commit)` |

### Policy note for state-manager (recommended addition to policies.yaml)

The following one-line addition to policies.yaml prevents re-emergence of this finding class. State-manager SHOULD add this to policies.yaml (suggested as POLICY 19 or the next available ID):

> `adr_version_cite_volatile_pin_prohibition`: BC Traceability rows citing an ADR (any ADR) MUST use the stable section-anchor form `ADR-NNN §Decision N` without a load-bearing version number token (TD-VSDD-091 compliance). Version tokens inside Traceability cell prose strings are forbidden as load-bearing identifiers. Informational `(as of vX.Y)` parentheticals are permitted non-load-bearing addenda only.

---

## Context

### The gap: context-window loss is currently unremediated

The factory externalizes durable pipeline state to `STATE.md` and the `factory-artifacts` orphan branch, but does not use that externalized state as a *deliberate context-management* mechanism. Long autonomous runs exhaust the context window. When the Claude Code harness triggers auto-compaction mid-wave, the summarizer runs without any factory-side coordination: critical SHAs, active decisions, BC identifiers, and open-findings lists may be silently dropped or hallucinated.

Two independently verified failure modes motivate this ADR:

1. **Fabricated-SHA risk:** The jira-cli sequence documented in issue #170 (and research file `issue-173.md`) shows that a hallucinated SHA in STATE.md can survive a compaction-summarization and re-enter the next turn as authoritative state. The PreCompact flush (Part B) closes this by committing the real, verified SHA to `factory-artifacts` before compaction can run.

2. **Cross-wave continuity collapse:** Without an enforced wave-boundary handoff, the in-context state at wave N close is the only record of what must be carried forward to wave N+1. A session reset (for any reason) after wave N closes but before the handoff is written loses that record entirely. The wave-boundary checkpoint (Part A) closes this by requiring a verified `HANDOFF.md` on `factory-artifacts` before a wave can be considered closed.

### Prior art in this codebase

The building blocks exist:
- `state-burst` skill: single-commit push to `factory-artifacts` — this is the flush primitive (SS-06).
- `wave-gate` skill: already gates wave close on multiple prerequisites (SS-06). Reads `sprint-state.yaml` as authority for story status per wave.
- `sprint-state.yaml` at `.factory/stories/sprint-state.yaml`: current authoritative source for story status (merged/ready/draft/partial/withdrawn) and epic membership.
- `factory-artifacts` orphan branch: already the durable external state store (SS-05).
- `hooks-registry.toml` plus the `legacy-bash-adapter` pattern: established model for shell hooks (SS-07/SS-04).
- WASM plugin fleet: established model for deterministic parse-heavy gate validators (SS-04).
- ADR-025 (single-writer lock/lease): provides the `factory_lock` frontmatter block (absent = no lock held) and `state-burst` renewal step — Part B's flush MUST invoke lock renewal before committing WHEN a lock is held; when `factory_lock` is absent, the renewal step is a no-op.

### Real state substrate (F-1 / F-15 re-anchor)

The design anchors to TWO real substrates depending on pipeline context:

| Context | Wave/Phase identity source | Story sequence source |
|---------|--------------------------|----------------------|
| **Product pipeline** (wirerust, jira-cli, engineering-report — products the factory builds) | `sprint-state.yaml` `current_wave`-equivalent grouping derived from dependency order produced by `wave-scheduling` skill | `sprint-state.yaml` story entries with `status: pending` or `status: draft` ordered by dependency graph |
| **Self-referential engine** (vsdd-factory's own STATE.md) | STATE.md frontmatter `current_cycle:` + `phase:` + `current_step:` fields | Story-INDEX.md + sprint-state.yaml hybrid (same `.factory/stories/sprint-state.yaml` path) |

**No `current_wave:` field is invented.** No `wave:` frontmatter on story files. The design reads the real existing fields.

For the `precompact-flush.sh` hermetic flush (Part B): the hook determines context identification from STATE.md `current_cycle:` + `current_step:` fields (always present). The HANDOFF.md `wave_id` is a logical identifier that the `wave-handoff` skill derives from the sprint-state.yaml wave-group numbering for product pipelines, OR from the cycle pass number (e.g., the engine's `pass-N` in `current_step:`) for the self-referential engine.

---

## Wave-Identity Derivation (Normative — F-P2-001)

This section is the **single canonical expression** of how wave identity is derived. All BCs, VPs, DIs, and test fixtures MUST cite this section verbatim. No other definition is authoritative. Downstream documents must NOT invent a `current_wave:` field on STATE.md — that field does not exist.

### Canonical Derivation

> **Wave identity (wave group index or pass identifier) is derived from real substrate as follows:**
>
> - **Product-pipeline context** (products the factory builds — wirerust, jira-cli, engineering-report, etc.): wave identity is the wave-group index derived by the `wave-scheduling` skill from `sprint-state.yaml` dependency-order topological grouping. The ordinal position of the current wave group in that topological sort (position 1 = wave 1, position 2 = wave 2, etc.) is the `wave_id`. No `current_wave:` field exists on STATE.md and no `wave:` frontmatter field exists on story files.
>
> - **Self-referential engine context** (vsdd-factory's own STATE.md): wave identity is the `current_step:` pass identifier from STATE.md frontmatter (e.g., `pass-N` in an F5 cycle). The `current_cycle:` field provides the enclosing cycle name. No `current_wave:` field exists on STATE.md.
>
> **There is no `current_wave:` field** on STATE.md. Test fixtures that construct a STATE.md MUST NOT include `current_wave:` as a field. Instead:
>
> - For product-pipeline tests: parameterize by wave-group ordinal derived from a synthetic `sprint-state.yaml` with `status: pending/draft` entries.
> - For engine-context tests: parameterize by STATE.md `current_step:` value (e.g., `current_step: "pass-2"`).
> - For "wave 1 / first wave" discrimination: derive from the **absence of a prior HANDOFF.md** on `factory-artifacts` (no prior handoff → first wave) OR from the wave-group ordinal being 1 in the topological sort. **Cross-note (F-P20-002 / §Decision 9):** The prior-HANDOFF.md-absence proxy is the CALLER-SIDE substrate: the shell wave-handoff skill reads it to derive `wave_id=1`, then embeds `wave_id=1` in the HANDOFF.md payload before writing. The WASM gate (`validate-wave-handoff-completeness`) uses a PAYLOAD-ONLY discriminator — `is_first_wave = (payload.wave_id == 1)` — and does NOT read prior HANDOFF.md existence, factory-artifacts, sprint-state.yaml, or git (F-P10-001; §Decision 9). These are NOT competing substrates for the same determination: prior-HANDOFF.md absence is the upstream signal from which the CALLER derives the `wave_id=1` value that the WASM gate then reads from the payload. See §Decision 9 for the canonical WASM gate behavior and VP-083 §Property §1 for the PAYLOAD-ONLY discriminator specification.

### Verbatim replacement for all prior `current_wave` references

Any prior text reading "reads `current_wave:` from STATE.md frontmatter" or "STATE.md with `current_wave = N`" MUST be replaced with:

> "reads `current_step:` and `current_cycle:` from STATE.md frontmatter (engine context) OR derives wave-group ordinal from sprint-state.yaml dependency-order topological sort (product-pipeline context) — there is no `current_wave:` field on STATE.md"

For test fixture descriptions, "Setup: STATE.md with `current_wave = N`" MUST become:

> "Setup: STATE.md with `current_step: \"pass-N\"` (engine context) OR synthetic sprint-state.yaml with N-1 stories in `status: merged/withdrawn` preceding current wave group (product-pipeline context)"

For HANDOFF.md `wave_id` field: the value is an integer derived from the above substrate, NOT read from a `current_wave:` field. The anti-fabrication cross-check for `wave_id` is: computed value from `wave-scheduling` topological sort (product) or pass number from `current_step:` (engine) must match the `wave_id` written in HANDOFF.md.

---

### Confirmed harness capability (F1 research)

The Claude Code harness (v2.1.105+) supports:
- `PreCompact` hook: fires before context compaction; can block via `exit 2` or `{"decision":"block","reason":"..."}`.
- `PostCompact` hook: fires after compaction; cannot block (advisory only).
- `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` env var: triggers auto-compaction earlier (effective band ≤ approximately 83% internal ceiling per MEDIUM-confidence research; see Decision 5 note).
- Sub-agent isolation: sub-agent tool-call history stays in the sub-agent's context; only the final summary reaches the parent.

**Runtime precondition:** The PreCompact blocking capability requires Claude Code harness version ≥ v2.1.105. On earlier versions, PreCompact fires as a notification-only hook (no veto). This ADR documents this as a hard runtime precondition. The factory cannot assume an older harness is sufficient for Part B.

**Honest degrade behavior on pre-v2.1.105 (F-5):** On pre-v2.1.105, the `precompact-flush.sh` hook fires as a notification; `exit 2` is visible to the user as stderr output but does NOT block compaction. The flush runs (state is written to `factory-artifacts`) but cannot prevent the context window from being replaced. The CAP-032 continuity guarantee is NOT satisfied on pre-v2.1.105. This is not a safe degradation — it is a reduction to best-effort only. Operators on pre-v2.1.105 must be warned.

**Settings.json env-verification (F-11):** The `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` setting requires verification in the active `settings.json`. The `check-state-health` skill must verify this env var is present with value `70` in `settings.json` and emit an advisory if absent.

---

## Prerequisite Verification Discipline (F-15)

The design depends on the following real artifacts and fields. S-18.00 (prerequisite verification story) MUST verify each exists and carries the needed data before E-18 implementation proceeds:

| Artifact / Field | Required content | Verification action |
|-----------------|-----------------|---------------------|
| `plugins/vsdd-factory/hooks-registry.toml` | Parseable TOML; `[[hooks]]` array present | `cargo test --workspace` passes; registry loads without error |
| `.factory/stories/sprint-state.yaml` | Contains `stories:` list with status-tagged entries | `grep -n 'status:' .factory/stories/sprint-state.yaml` returns non-empty |
| STATE.md frontmatter `current_cycle:` | Present, non-empty string | S-18.00 inspection of `head -5 .factory/STATE.md` |
| STATE.md frontmatter `phase:` | Present, non-empty string | S-18.00 inspection |
| `crates/factory-dispatcher/src/` | Contains event-routing logic; `PreCompact`/`PostCompact` enumerated OR absent (S-18.00 determines which) | S-18.00 reads source and documents finding |
| `plugins/vsdd-factory/bin/factory-cas-push.sh` | Exists and is executable | `test -x plugins/vsdd-factory/bin/factory-cas-push.sh` |
| ADR-025 lock model (`factory_lock` block in STATE.md) | `factory_lock:` block present = lock held; absent = no lock; null holder = no lock | Confirmed by ADR-025 §Decision 2 and ADR-025 §D2 canonical form |

---

## Decisions

### Decision 1 — Part A is primary; Part B is the safety net

**Decision:** Wave-boundary hard session reset is the PRIMARY cross-wave continuity mechanism. Intra-wave PreCompact flush (Part B) is the SAFETY NET for compactions that happen mid-wave before a wave boundary is reached.

**Rationale:** External research consensus (Anthropic context-engineering blog; Microsoft Agent Framework; multiple secondary sources cited in `issue-173.md`) favors hard session resets at wave boundaries over continuous compaction for pipelines with externalized durable state. Resets avoid stacking multiple lossy summarization passes. The precondition for preferring reset over compaction — that durable external state exists and is reliable — is already met by the factory's `factory-artifacts` branch. The PreCompact flush ensures no in-wave state is lost on compaction events that occur between wave boundaries.

**Alternative rejected:** Treating auto-compaction as reliable enough on its own (without PreCompact flush). The documented absence of retention-steering capability (research finding #3 in `research-precompact-gating-2026-06-13.md`) means the summarizer can silently drop any fact. The flush is a required safety net.

---

### Decision 2 — HANDOFF.md on factory-artifacts as the verified wave-close checkpoint

**Decision:** Wave close is declared by writing a structured `HANDOFF.md` file on the `factory-artifacts` branch. The file is the authoritative cross-wave checkpoint artifact. A wave is not closed until `HANDOFF.md` exists with all required fields verified against external git/test sources (anti-fabrication).

**Schema — required fields:**

| Field | Type | Source (real substrate) | Anti-fabrication rule |
|-------|------|------------------------|----------------------|
| `wave_id` | integer | Product pipelines: wave group number derived by `wave-handoff` from sprint-state.yaml dependency order. Engine self-referential: pass number from STATE.md `current_step:` | Cross-checked: `wave_id` in HANDOFF.md must match the value wave-handoff skill computed from the real sprint-state.yaml or STATE.md — not from any phantom `current_wave:` frontmatter field |
| `last_verified_develop_sha` | string (40-char hex) | `git rev-parse origin/develop` at handoff time | Cross-checked: must equal `git rev-parse origin/develop` at handoff time |
| `active_bcs` | list of strings | BC-INDEX.md file list | Each must resolve to an existing file under `.factory/specs/behavioral-contracts/` |
| `next_wave_stories` | list of objects `{id, status}` | sprint-state.yaml entries with `status: pending` OR `status: draft`, ordered by dependency graph | Each `id` must exist in STORY-INDEX.md; **empty list is a hard error** (see Decision 3a) |
| `open_decisions` | list of objects `{id, anchor_type, anchor_ref}` | decision-log.md open rows | `anchor_ref` must be a commit hash, test function name, or file path — NOT a memory assertion |
| `pending_fixes` | list of objects `{finding_id, pr_or_issue_ref}` | Active adversary findings | Each must cite a PR number or issue ref — NOT a memory assertion |
| `process_gaps` | list (may be empty) | Carry-forward from issue #171 mechanism | May be empty; must be explicitly listed or `[]` |
| `precompact_flush_sha` | string (40-char hex) OR null | `precompact-flush-log` append-only side-channel file (last-line SHA field) | Three-state wave-agnostic null rule (F-P17-001; BC-5.41.001 §PC2/PC5/EC-006/EC-011): The wave_id discriminator applies ONLY for the log-absent case. **(1) log genuinely ABSENT** (`test -f .factory/hooks/precompact-flush-log` false, or `host::read_file` returning FileNotFound in WASM-context): null when wave_id=1 → VALID (silent; normal first-wave, no flush ever ran); null when wave_id>1 → ADVISORY warning, no hard block (log absence means no PreCompact flush ever ran this wave; absence verified by filesystem probe, NOT operator attestation). **(2) log present but last-line CORRUPT** (FIELD-4 absent, empty, or ≠`commit`) → stale entry → prefix-match-alone → **EXEMPT** (consistent with DI-025/BC-5.41.003/§F-P4-004 EC-003 corruption arm — do NOT block; ANY wave_id). **(3) log present with VALID last-line** (FIELD-4 == `commit`): null → **HARD BLOCK (PrecompactShaMismatch)**, **ANY wave_id including wave_id=1** (mid-wave-1 compaction IS a supported Part B scenario; a genuine flush commit in the log at wave-1 close means `precompact_flush_sha: null` is fabricated or dropped); non-null SHA must equal last-line field 2 (FIELD-2), mismatch → **HARD BLOCK**, match → VALID. **(40-char hex SHA value with log absent or corrupt):** VALID per standard corroboration rules above. |
| `factory_lock_holder` | string OR null | STATE.md `factory_lock.holder` (if present) OR null | Must match `factory_lock.holder` in STATE.md at handoff time; null when `factory_lock` block is absent from STATE.md (lock not held) |
| `epic_status` | string `"complete"` OR **absent** | EPIC-COMPLETE discriminator: all sprint-state.yaml stories terminal AND `next_wave_stories: []` | Must equal `"complete"` when present; derived from sprint-state.yaml terminal-status exhaustion — NOT from in-context assertion. **CONDITIONAL: required ONLY on the final wave (EPIC-COMPLETE); MUST be absent on all non-final waves.** See §F-P6-004 PO Wording for validate-wave-handoff-completeness gate behavior. |

**Field count summary (F-P6-004 v1.6 reconciliation):** The HANDOFF.md schema has **9 base required fields** (wave_id through factory_lock_holder). `epic_status` is a **conditional 10th field**: required ONLY on the EPIC-COMPLETE final wave, MUST be absent on non-final waves. Any BC text asserting "9 required fields" is correct for non-final waves. The validate-wave-handoff-completeness gate (BC-4.14.001) must conditionally include or exclude `epic_status` from its required-field set based on EPIC-COMPLETE context detection.

**Empty `next_wave_stories` is a hard error (F-3):** If the sprint-state.yaml contains no stories with `status: pending` or `status: draft`, the `wave-handoff` skill MUST abort with a non-zero exit and an explicit error message: "No next-wave stories found in sprint-state.yaml — either this is the final wave (declare epic complete) or sprint-state.yaml needs updating." A silent advisory no-op is forbidden (SOUL.md #4).

**Wave-agnostic null rule for log-present-and-valid case (F-P17-001; three-state rule):** The `precompact_flush_sha` null logic is now uniform across all wave_id values when the log is present and valid. The wave_id discriminator survives ONLY for the log-absent case:

1. **precompact-flush-log genuinely ABSENT** (verified mechanically via `test -f .factory/hooks/precompact-flush-log` in shell-context, or `host::read_file` returning FileNotFound in WASM-context; NOT by operator attestation): null when wave_id=1 → VALID (silent; normal first-wave scenario where no PreCompact flush has ever fired and the log has never been created). null when wave_id>1 → ADVISORY warning emitted to stderr, no hard block (genuine log absence means no PreCompact flush ever ran during this wave).

2. **log present but last-line CORRUPT** (FIELD-4 absent, empty, or ≠ `commit`): stale entry → prefix-match-alone → EXEMPT, ANY wave_id. The `wave-handoff` skill documents this case in its output header. The corruption→EXEMPT arm is unchanged per DI-025/BC-5.41.003/§F-P4-004/EC-003.

3. **log present with VALID last-line** (FIELD-4 == `commit`): null → **HARD BLOCK (PrecompactShaMismatch)**, **ANY wave_id including wave_id=1**. Mid-wave-1 compaction is a fully supported Part B scenario (wave 1 is the longest, most compaction-prone autonomous run). When a genuine flush commit has landed and the log reflects it, writing `precompact_flush_sha: null` at wave-1 close is a fabricated or dropped SHA — the same failure class (DI-021/CAP-032) that wave_id>1 already blocks. wave_id>1 and wave_id=1 are now treated identically when the log is present with a valid last line.

A non-null SHA value is VALID when the log is absent or corrupt (no corroboration possible), and VALID when the log is present and last-line FIELD-2 matches exactly. A non-null SHA that does NOT match FIELD-2 when the log is present and valid → **HARD BLOCK (PrecompactShaMismatch)**, any wave_id.

**Rationale:** Every field that could be fabricated from in-context memory requires cross-checking against a verifiable external source (git, filesystem, index files). This directly closes the jira-cli fabricated-SHA failure class. The schema is intentionally narrow — only what the next session needs to resume, not a full state dump.

---

### Decision 3 — Wave-boundary reset: prompt-the-human (D3 LOCKED)

**Decision:** The wave-boundary session reset is initiated by prompting the human. The orchestrator writes and verifies `HANDOFF.md`, then asks the human to clear the session and start wave N+1. The human clears the session and the new session rehydrates from `wave-state.yaml`.

**Auto-reset is an explicit v2 deferral.** Auto-reset (orchestrator self-clearing its own context) is a destructive, irreversible action. If the handoff is incomplete or wrong, in-session state is lost with no recovery path. This risk is not acceptable for v1 without additional safeguards (human confirmation is the safeguard). Auto-reset may be revisited after wave-boundary checkpoint mechanisms have been validated in production.

**v2 deferral recorded:** `auto-reset: enable-when-handoff-verified-for-N-consecutive-waves` — deferred to E-18 follow-up or a future feature cycle.

---

### Decision 4 — Scoped rehydration: curated wave-state.yaml manifest (D4 LOCKED)

**Decision:** After a wave-boundary reset, the new session rehydrates from a curated `wave-state.yaml` manifest. The manifest is produced by the `wave-handoff` skill as part of the wave-close checkpoint. It explicitly lists the next wave's stories and the spec files they depend on (BC files, ADR files, relevant SS-NN files).

**next_wave_stories derivation from real substrate (F-3):** The `wave-handoff` skill derives the next wave's story list from `sprint-state.yaml` by selecting entries with `status: pending` OR `status: draft`, then applying the dependency order graph from STORY-INDEX.md `depends_on:` arrays to produce the wave sequence. This is the SAME algorithm used by the `wave-scheduling` skill's topological sort step. No `wave:` frontmatter field on story files is referenced — that field does not exist.

**RAG is an explicit v2 deferral.** Semantic retrieval over the spec corpus is non-deterministic and introduces the same hallucination risk that wave-boundary resets are designed to eliminate. The curated manifest approach is deterministic and auditable. The manifest can be generated mechanically from STORY-INDEX.md dependency lists and sprint-state.yaml status entries.

**v2 deferral recorded:** `rehydration: rag-over-spec-corpus` — deferred to E-18 follow-on or a future feature cycle when manifest approach proves too rigid for large epics.

**`wave-state.yaml` schema (minimum required fields):**

```yaml
wave_id: 2
generated_at: "2026-06-14T00:00:00Z"
generated_from_handoff_sha: "<sha>"
stories:
  - id: "S-18.02"
    status: "pending"
    spec_files:
      - ".factory/specs/behavioral-contracts/ss-04/BC-4.XX.001.md"
      - ".factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
arch_files:
  - ".factory/specs/architecture/ARCH-INDEX.md"
  - ".factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"
  - ".factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
state_pointer: ".factory/STATE.md"
```

---

### Decision 5 — Proactive compaction threshold: 70% via CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (D5 LOCKED)

**Decision:** The proactive auto-compaction threshold is 70% of the effective context window capacity, configured via `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` in the factory's `settings.json` env block. This fires auto-compaction earlier than the harness default, giving the PreCompact flush enough headroom to complete synchronously before the context is fully exhausted.

**Rationale:** Research consensus from Anthropic context-engineering guidance and Microsoft Agent Framework documentation places the optimal proactive-compaction band at 70–75%. 70% is chosen as a conservative default that leaves headroom above the threshold before the harness's own ceiling.

**MEDIUM-confidence note on 83% ceiling (F-11):** The claim that `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is clamped to approximately 83% is a MEDIUM-confidence research finding, not a formally documented API guarantee. The 70% setting is chosen to be well within any plausible effective range. This claim does NOT substitute for the bounded `timeout_ms` specified in Decision 6 — the timeout is the correct engineering control for flush-time bounding; the headroom argument is an additional belt.

**Settings.json env-verification requirement:** Deployment of E-18 MUST include verifying that `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` is present in `settings.json`. The `check-state-health` skill MUST emit an explicit advisory if this var is absent or set to a value > 80.

**Per-autonomy-level configurability is an explicit v2 deferral.** v1 uses a single global default. A fully-autonomous long-run may want a lower threshold (e.g., 60%); a human-in-the-loop interactive session may want 75%. Configurability per autonomy level is deferred to a follow-on feature cycle.

**v2 deferral recorded:** `threshold-config: per-autonomy-level` — deferred.

---

### Decision 6 — PreCompact flush: synchronous shell hook, blocking on flush incompleteness

**Decision:** The intra-wave flush is implemented as a shell hook (`precompact-flush.sh`) registered under `[[hooks]] event = "PreCompact"` in `hooks-registry.toml`. The hook:

1. Reads `STATE.md` frontmatter fields `current_cycle:` and `current_step:` to determine context identity. Does NOT rely on in-context reasoning — reads STATE.md from the filesystem only. Does NOT look for a non-existent `current_wave:` field.
2. Reads the `factory_lock:` block from STATE.md. If `factory_lock:` is absent or `factory_lock.holder` is absent/null, the lock-renewal step is skipped (no lock held; ADR-025 opt-in model). If lock is held, renews per ADR-025 Decision 11 Mechanism 1 (calls `factory-lock-write.sh renew .factory/STATE.md` before `git add`/commit).
3. Invokes `state-burst` flush logic (equivalent to the flush portion of the `state-burst` skill) to write current wave-critical state to `factory-artifacts`.
4. Commits to `factory-artifacts` with message: `PreCompact flush <cycle>/<step> <timestamp>`.
5. **Exits with `exit 2` (blocking)** if the flush was required and the commit did not land successfully (git commit failure, git push failure). Exits 0 if the flush landed or was not needed (no state changes since last flush).
6. Appends `<ISO-timestamp> <SHA> <cycle>/<step> commit` to the append-only side-channel log (`.factory/hooks/precompact-flush-log`) BEFORE the `git push` step. This write is local filesystem and precedes the push to establish crash-consistent ordering. The fourth field (`commit`) is the output of `git cat-file -t <SHA>` executed HERE in the shell hook (which CAN exec git). WASM corroboration validators (`validate-burst-log`, `validate-dispatch-advance`) read this pre-embedded token statically via `host::read_file` — they do NOT exec `git cat-file` (WASM has no `exec_subprocess`). **Append failure: SHA-pinned reset-then-exit-2 (F-P5-002 + F-P6-006).** If the append to `precompact-flush-log` fails, the hook MUST: (a) capture SHA_B = `git -C <worktree> rev-parse HEAD` immediately after the local commit succeeds (this capture MUST happen at step 4 before the append, so it is available when the append fails); (b) compare CURRENT_HEAD = `git -C <worktree> rev-parse HEAD` to SHA_B; (c) if CURRENT_HEAD == SHA_B: execute `git -C <worktree> reset --soft SHA_B^` (SHA-PINNED, not `HEAD~1`); then exit 2; (d) if CURRENT_HEAD != SHA_B: a concurrent commit landed — do NOT reset; exit 2 immediately with error message `precompact-flush: append failed; concurrent commit advanced HEAD; SHA_B=<sha_b>; human intervention required`; (e) if the reset itself fails: exit 2; human intervention required. Rationale for SHA-pinned form: `HEAD~1` is a relative reference that resolves to whatever HEAD~1 is at reset time, which is the concurrent commit SHA_C if a state-manager burst landed between the flush commit and the reset — discarding SHA_C silently (SOUL.md #4). The SHA-pinned `SHA_B^` always resolves to the parent of the KNOWN flush commit, which is the pre-flush HEAD regardless of any intervening commits. See §F-P6-006 Concurrent-Commit Race Guard for full analysis. The `on_error = "continue"` fail-open at the harness level applies ONLY to full hook crashes (before any exit code is emitted) — not to deliberate exit 2 signals.

**Timeout semantics (F-4):** A `timeout_ms = 30000` (30 second) cap is registered in `hooks-registry.toml`. Timeout (no response within 30s) is treated as a hook failure with `on_error = "continue"` semantics: compaction proceeds, flush is assumed non-blocking (best-effort). This prevents a hung git push from wedging the session indefinitely. Timeout is explicitly distinguished from commit failure: a git push that returns a non-zero exit code causes `exit 2` (blocking); a git push that simply hangs until the 30s timeout is treated as a crash and fails open. The hook MUST emit progress to stderr within 5 seconds of invocation to aid diagnostics.

**`on_error = "continue"` (fail-open):** If the hook script crashes before emitting a result, the harness must not wedge the session. Durability is best-effort on crash. A crashed flush is a loss of a flush cycle, not a session-ending event.

**Hermetic requirement:** The flush hook reads ONLY from `STATE.md` and git. It MUST NOT read from in-context state (it runs in a subprocess, not as an LLM tool). This is the anti-deadlock invariant (F1 regression risk §4.1 R1 closure).

**`custom_instructions` is NOT used:** Research confirmed that `custom_instructions` is unreliable on auto-compaction (live official docs omit it; older docs indicate it is empty for `auto` trigger). The flush relies entirely on external state persistence, not summarizer retention.

**Hook registration spec (corrected TOML schema per F-7):**

```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/precompact-flush.sh"

[hooks.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash", "git", "jq"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md", ".factory/hooks/precompact-flush-log"]

[hooks.capabilities.write_file]
path_allow = [".factory/hooks/precompact-flush-log"]
```

---

### Decision 7 — PostCompact re-anchor: advisory shell hook (cannot block) — BEST-EFFORT ONLY

**Decision:** A `postcompact-reanchor.sh` advisory hook is registered under `[[hooks]] event = "PostCompact"`. It fires after compaction completes, re-reads the `factory-artifacts` STATE.md pointer, re-asserts the current cycle/phase context, and emits a summary to the harness (visible to the LLM as context). It CANNOT block (PostCompact is inherently non-blocking in the Claude Code harness).

**CAP-032 continuity guarantee clarification (F-6):** The PostCompact re-anchor provides convenience context injection after compaction but is NOT a correctness guarantee and is NOT part of the CAP-032 continuity-guarantee chain. The CAP-032 guarantee rests exclusively on:
- Part A: HANDOFF.md verified on `factory-artifacts` before wave close (Decision 2)
- Part B: PreCompact flush (Decision 6) — commits state BEFORE compaction

If the PostCompact re-anchor hook fails, crashes, or is skipped, the CAP-032 guarantee is unaffected — Part A and Part B are sufficient. The re-anchor is explicitly best-effort.

**Hook registration spec (corrected TOML schema per F-7):**

```toml
[[hooks]]
name = "postcompact-reanchor"
event = "PostCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 10000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/postcompact-reanchor.sh"

[hooks.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash", "git", "jq"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md", ".factory/hooks/precompact-flush-log"]
```

**Note:** PostCompact firing is harness-internal. `on_error = "continue"` is required (PostCompact cannot block regardless; setting is defensive).

---

### Decision 8 — WASM for completeness gate; shell for flush (hook split)

**Decision:** The wave-close completeness gate (`validate-wave-handoff-completeness`) is implemented as a native WASM plugin. The PreCompact flush and PostCompact re-anchor are implemented as shell scripts via `legacy-bash-adapter`. This split follows the factory's established convention (ADR-014, ADR-019):

| Hook | Implementation | Why |
|------|---------------|-----|
| `validate-wave-handoff-completeness` | Native WASM (new crate) | Deterministic parse-heavy validation: checks required fields present, `last_verified_develop_sha` format valid, `active_bcs` list non-empty. No git/FS side effects needed for the check itself. Tamper-resistant; auditable. |
| `precompact-flush.sh` | Shell via `hook-plugins/legacy-bash-adapter.wasm` | Effectful: git commit to `factory-artifacts`. WASM cannot exec git in the sandbox per ADR-002/ADR-003 WASI preview 1. |
| `postcompact-reanchor.sh` | Shell via `hook-plugins/legacy-bash-adapter.wasm` | Effectful: reads git refs, emits stdout. Same constraint. |
| `validate-heavy-op-delegation` | Native WASM (new crate) | Deterministic: command-string pattern matching. Pure function; no side effects. |

**WASM fuel budget:** Both new WASM crates use `timeout_ms = 5000` (consistent with existing WASM gates). HANDOFF.md body is capped at 200 lines (similar to STATE.md soft limit) to prevent fuel exhaustion.

---

### Decision 9 — validate-wave-handoff-completeness: no-op on wave-1 / non-HANDOFF.md writes

**Decision:** The `validate-wave-handoff-completeness` WASM gate fires as a PostToolUse gate on Write/Edit operations that produce `HANDOFF.md` on `factory-artifacts`. Its blocking behavior is:

- **Payload `wave_id == 1` (first wave):** Return `Continue` (no-op) unconditionally, without reading sprint-state.yaml or factory-artifacts. The gate computes `is_first_wave = (payload.wave_id == 1)` from the HANDOFF.md payload being written — this is PAYLOAD-ONLY, no external read. See §F-P10-001 for full rationale and BC-4.14.001 change spec.
- **Payload `wave_id > 1` (not first wave):** Validate all required fields are present and syntactically correct. Block with `HandoffIncomplete` if any required field is missing or malformed.
- **Non-HANDOFF.md write target:** The gate is a strict no-op when the tool call does not target `HANDOFF.md` (per BC-4.14.001 EC-011). The gate validates a Write/Edit payload; it cannot probe for file presence or absence on factory-artifacts.

**Semantic boundary — HandoffMissing is SHELL-SIDE, not WASM-gate (BC-5.41.001 PC9):** The `HandoffMissing` error code is emitted by the SHELL `wave-gate` skill (BC-5.41.001 PC9 / EC-014) when `wave-gate` is invoked to close a wave but no `HANDOFF.md` is present on `factory-artifacts`. This check is performed by the shell wave-gate BEFORE any WASM gate is invoked. The `validate-wave-handoff-completeness` WASM gate is NOT triggered on HANDOFF.md file-absence (BC-4.14.001 EC-011) and therefore NEVER emits `HandoffMissing`. The two error codes are disjoint: `HandoffMissing` = SHELL, file absent at wave-close; `HandoffIncomplete` = WASM gate, file present but payload fields missing or malformed. Any prior framing that attributed `HandoffMissing` to the WASM gate's own blocking behavior was incorrect and is superseded by this decision effective v1.13.

**Wave-1 discriminator — PAYLOAD-ONLY (F-P10-001):** The WASM gate MUST NOT read sprint-state.yaml, factory-artifacts (prior HANDOFF.md existence), git refs, or any filesystem resource to determine wave-1 status. The `wave_id` field is already present in the HANDOFF.md payload (Field 1 of the 9 base required fields). The gate evaluates `payload.wave_id == 1` directly from the parsed Write/Edit tool-call payload. The wave-handoff skill (caller) derives `wave_id` from real substrate (sprint-state.yaml topo-sort ordinal for product pipelines; STATE.md `current_step:` pass number for self-referential engine) BEFORE writing HANDOFF.md. The WASM gate is a CONSUMER of `wave_id`, not a DERIVER. The prior framing ('prior HANDOFF.md absent OR wave-group position 1 per sprint-state.yaml') was a PROXY that required git/filesystem reads, violating BC-4.14.001 Invariant 1. It is superseded by this decision effective v1.9.

**Rationale:** This closes F1 regression risk §4.1 R3 — the gate must not add friction for short single-wave runs or for users who have not yet produced a HANDOFF.md. The gate only activates when a transition between waves (wave N > 1) is being attempted. The payload-only approach also closes F-P10-001: the defect class of 'WASM gate reading external filesystem' is fully eliminated for the wave-1 discriminator, consistent with how F-P7-001/F-P8-001 eliminated it for the EPIC-COMPLETE discriminator.

---

### Decision 10 — PreCompact flush lifecycle is distinct from state-manager burst lifecycle

**Decision:** The PreCompact flush commit on `factory-artifacts` is a separate, distinct lifecycle from a state-manager burst (the A/B/C/D/E sequence per TD-VSDD-053). The PreCompact flush:

- Fires on a harness-internal event between LLM turns, NOT during a state-manager burst.
- Its commit to `factory-artifacts` is NOT counted as a "burst commit" for TD-VSDD-053 single-commit-per-burst enforcement.
- The burst-log entry for the enclosing burst MUST NOT cite the PreCompact commit as a "Commit A/B/C/D/E" — it is a lifecycle-orthogonal commit.
- The `validate-burst-log` and `validate-dispatch-advance` hooks MUST be configured to ignore PreCompact flush commits (identified by commit message prefix `PreCompact flush `).

**Rationale:** This closes F1 regression risk §4.1 R5. Without this explicit boundary, a future adversary could flag the PreCompact commit as a TD-VSDD-053 multi-commit-chain violation.

**Bats test requirement:** A bats test MUST verify that firing a simulated PreCompact hook (triggering a flush commit) and then a state-manager burst produces exactly one "burst commit" in the burst-log entry — not two.

---

### Decision 11 — S-18.00: Dispatcher routing addition for PreCompact/PostCompact

**Decision:** Before E-18 Part B stories (S-18.04, S-18.05) can be implemented, the vsdd-factory dispatcher binary MUST route `PreCompact` and `PostCompact` events to registered plugins. F1 confirmed the Claude Code harness emits these events (live docs, v2.1.105+). The gap question is whether the vsdd-factory dispatcher's plugin invocation layer passes them through.

**Resolution approach (S-18.00):** Story S-18.00 (wave-1 prerequisite) MUST verify by inspection of the dispatcher event-routing source in `crates/factory-dispatcher/src/` (the specific file to inspect is determined by S-18.00 — look for the event-type enum or match arms) whether `PreCompact` and `PostCompact` are enumerated as supported events. If absent, S-18.00 adds routing support. If present, S-18.00 is a no-op verification story that documents the confirmation.

**This decision does NOT pre-judge the outcome.** The architect does not have access to the live dispatcher source in this F2 phase. S-18.00 is the resolution vehicle.

**Release requirement:** Any change to `crates/factory-dispatcher/` requires a release cut for the operator-level cache to pick up the update (CLAUDE.md self-referential note). E-18 stories that depend on PreCompact/PostCompact MUST be sequenced after an rc cut that includes S-18.00's changes (if any).

---

### Decision 12 — validate-heavy-op-delegation WASM gate: advisory-only in v1

**Decision:** The `validate-heavy-op-delegation` WASM gate (PreToolUse on `Bash` tool calls) launches in advisory mode in v1. It emits a finding to stderr but does NOT block. Promotion to blocking mode requires calibration in F3 adversarial review after measuring false-positive rates against real production pipeline sessions.

**No BC required for v1 advisory (F-9):** An advisory-only gate with no blocking behavior has no enforceable behavioral postcondition suitable for a BC. If v2 promotes this gate to blocking mode, product-owner MUST author a BC at that time. S-18.06 story is correctly scoped as advisory-only; no BC is authored in F2.

**Pattern set for advisory nudge (v1):**
- Commands matching `cargo test --release` with output likely > 10MB
- Commands matching `grep -r` or `find . -name` traversals against large directory trees
- Commands matching known heavy bats test runners (`.run-all.sh`, `./run-bats.sh`)

**Rationale:** False-positive blocking of legitimate Bash commands is a harder regression than missing a delegation nudge. The advisory mode collects real data without impeding pipeline operation.

---

### Decision 13 — Harness version runtime assertion (F-5)

**Decision:** The harness ≥ v2.1.105 precondition must be actively checked at runtime, not merely documented. Two enforcement mechanisms:

1. **SessionStart advisory hook:** A `check-harness-version.sh` advisory hook (or extension of the existing `check-state-health` skill) reads the harness version from the Claude Code environment (if accessible via env var or harness-provided metadata) and emits an ADVISORY warning if the version is below v2.1.105. Advisory only — does not block (a misconfigured harness that blocks SessionStart is worse than one that silently runs without PreCompact blocking).

2. **check-state-health skill addition:** The `check-state-health` skill MUST include a harness-version check step that reads any available version signal and reports: "PreCompact blocking: SUPPORTED (harness >= v2.1.105)" or "PreCompact blocking: UNSUPPORTED (harness < v2.1.105 — Part B guarantee not enforced)."

**Honest degrade documentation (F-5):** S-18.04 AC-001 and the `precompact-flush.sh` script header MUST state plainly: "On harness < v2.1.105, this script runs but cannot prevent compaction. exit 2 is visible as stderr only. CAP-032 Part B guarantee is not satisfied."

---

## SS-08 and S-18.07 Scope Clarification (F-9)

**Why SS-08 is not in `anchors:`:** The ADR-026 `anchors:` list enumerates subsystems where architectural decisions are made (SS-04: plugin ecosystem; SS-05: factory-artifacts state store; SS-06: skill catalog; SS-07: bash hook layer). SS-08 (Templates and Rules) owns templates and rules artifacts, not the hook pipeline.

**S-18.07 terminology disambiguation:** This story updates documentation in `compact-state/SKILL.md`, `check-state-health/SKILL.md`, and `CLAUDE.md`. These documentation targets span SS-06 (skills) and SS-08 (templates/rules). However, the *architectural decision* is that three terms are distinct — "context compaction" (harness), "state compaction" (`compact-state` file op), "PreCompact flush" (new). This disambiguation is a documentation-only change; it makes no new architectural commitments and creates no behavioral postcondition that requires a BC. **No new BC is required for S-18.07.** Product-owner does not need to act on S-18.07.

---

## Consequences

### Positive

- **Wave-boundary continuity becomes a hard guarantee**, not a best-effort. Every wave close is verifiable against git/filesystem (not memory).
- **Mid-wave compaction becomes lossless** for the factory's load-bearing state. The flush guarantees that `STATE.md`, wave context, and active decision SHAs land on `factory-artifacts` before any compaction event can drop them.
- **Fabricated-SHA risk class is directly addressed.** The anti-fabrication cross-check in `HANDOFF.md` (git-verified SHAs, filesystem-verified BC paths) eliminates the fabrication failure mode documented in issues #170 and #173.
- **Sub-agent isolation becomes the recommended first-line defense** for heavy ops, reducing orchestrator window pressure without requiring any new hook (the delegation guard is advisory in v1).
- **Terminology collision is resolved** by the disambiguation story (S-18.07): "context compaction" (harness), "state compaction" (`compact-state` file op), and "PreCompact flush" (new, this ADR) are distinct concepts with distinct documentation targets.

### Negative / Trade-offs

- **Wave-boundary reset requires human action.** The human must clear the session and confirm the handoff before wave N+1 begins. This is a deliberate friction point — it is the safety mechanism. Automation (auto-reset) is an explicit v2 deferral per Decision 3.
- **PreCompact flush adds latency at every compaction event.** The flush runs synchronously inside the PreCompact hook. If the flush takes more than 30 seconds (e.g., slow git push under poor network conditions), the 30s timeout triggers and the hook fails open (compaction proceeds unblocked). The timeout is the engineering control for this trade-off.
- **New WASM crates inflate the CI WASM floor-count gate.** Two new `[[bin]]`-bearing crates (`validate-wave-handoff-completeness`, `validate-heavy-op-delegation`) must be added to the floor-count expectation in CI at the same commit that adds them (standard procedure per ADR-014 precedent).
- **E-18 stories must ship atomically** in a single rc cut. Partial shipping (e.g., shell hooks without the WASM completeness gate) would create a regression where the flush fires but the wave-close gate does not block. S-18.00 through S-18.07 must be sequenced into a single rc cut.

---

## Risks Addressed

This ADR directly addresses the F1 regression risks:

| Risk | F1 ID | Mitigation in This ADR |
|------|-------|------------------------|
| PreCompact flush deadlock (flush needs in-context reasoning) | §4.1 R1 | Decision 6 hermetic requirement: flush reads STATE.md+git only, never in-context reasoning |
| Blocking PreCompact on crash wedges session | §4.1 R2 | Decision 6 `on_error = "continue"` |
| Completeness gate blocks wave-1 / short-run no-ops | §4.1 R3 | Decision 9 explicit no-op rules: (a) wave_id == 1 (PAYLOAD-ONLY); (b) non-HANDOFF.md write target (per BC-4.14.001 EC-011). HandoffMissing is SHELL wave-gate (BC-5.41.001 PC9), not a WASM-gate emission. |
| WASM fuel exhaustion on large HANDOFF.md | §4.1 R4 | Decision 8 200-line body cap on HANDOFF.md |
| Single-commit-per-burst discipline ambiguity | §4.1 R5 | Decision 10 explicit lifecycle boundary + bats test |
| Hung git push wedges session (F-4) | new | Decision 6 `timeout_ms = 30000`; timeout = fail-open |
| Precondition not checked at runtime (F-5) | new | Decision 13 SessionStart advisory hook + check-state-health extension |
| PostCompact re-anchor misrepresented as guarantee (F-6) | new | Decision 7 explicit best-effort annotation; removed from CAP-032 chain |
| `wave_id` source phantom / next_wave_stories phantom source (F-1/F-3) | new | Decision 2 explicit real-substrate derivation; empty list = hard error |

---

## Crash-Consistency Design for Side-Channel (F-P2-003)

### Decision A — Append-only side-channel log replaces point-file (F-P2-003; reset-on-append-failure added F-P5-002)

**Problem:** Decision 6 originally emits `precompact_flush_sha` to `.factory/hooks/last-precompact-flush-sha` as a single-SHA point file written AFTER the flush commit lands. If the hook crashes between the `git commit --push` and the `write last-precompact-flush-sha` step, the committed flush is real but the side-channel file is absent. BC-5.41.003 EC-003 then tries to corroborate the PreCompact flush commit's SHA against the side-channel, finds the file absent, and must decide: treat absence as "no flush ever ran" (wrong — the flush DID run and commit) or treat absence as "file not yet written" (correct, but allows a bypass).

**Chosen design: append-only flush log** (production-grade option). This design provides full crash-consistency without requiring atomic filesystem operations:

1. **Side-channel becomes an append-only log**: The file `.factory/hooks/last-precompact-flush-sha` is renamed to `.factory/hooks/precompact-flush-log`. Each successful flush APPENDS a line to the log BEFORE the `git push` step. The line format is: `<ISO-8601-timestamp> <40-char-SHA> <cycle>/<step> <cat-file-type>` where `<cat-file-type>` is the output of `git cat-file -t <SHA>` on the just-created commit (value: `commit`). Example: `2026-06-14T12:00:00Z abc123...def456 v1.0-feature-context-durability-E18/pass-4 commit`. The append is performed via `echo "..." >> precompact-flush-log` immediately after the local `git commit` succeeds but BEFORE the `git push`. The four-field format is mandatory: it allows WASM corroboration validators (which cannot exec `git cat-file` in the WASM sandbox per ADR-002/ADR-003 WASI preview 1) to verify the commit type from the log itself without live git access. **Append failure: SHA-pinned reset-then-exit-2 (F-P5-002 + F-P6-006 supersedes HEAD~1 form):** If the append fails, the hook MUST execute the SHA-pinned guard from §F-P6-006 (capture SHA_B immediately after commit; if HEAD == SHA_B: `git reset --soft SHA_B^`; if HEAD != SHA_B: do NOT reset, exit 2 with human-intervention error). The SHA-pinned form supersedes the original `git reset --soft HEAD~1` prescription (F-P5-002) — the HEAD~1 form is retained here only as historical context: without the reset, the local commit (SHA_B) would be an orphan — real in local git history, never logged, never pushed. On the next retry, SHA_C is committed and logged; `validate-burst-log` (WASM) reads the log's LAST LINE (SHA_C) and cannot corroborate SHA_B, producing a false-positive `MULTI_COMMIT_CHAIN_NOT_ALLOWED` block. The SHA-pinned reset eliminates SHA_B before it can become an orphan; the retry starts from clean state. See §F-P6-006 for the full concurrent-commit guard specification — use that form, not `HEAD~1`. If the hook crashes after the append but before a git push, the log entry exists with a SHA that does not yet appear in `factory-artifacts` — the corroboration step reads the LAST entry and validates it against the commit SHA being evaluated. **Note on WASM capability constraint (O-P3-004 closure):** `validate-burst-log` and `validate-dispatch-advance` are WASM plugins registered in `hooks-registry.toml` with no `exec_subprocess` capability. They cannot exec `git cat-file`. The four-field log format resolves this by embedding the `cat-file -t` token at write time (in the shell hook, which CAN exec git). WASM validators read the fourth field from the log line directly.

2. **BC-5.41.003 corroboration revised**: The exemption corroboration step for `validate-burst-log` and `validate-dispatch-advance` (both WASM plugins, no `exec_subprocess` capability) now: (a) reads the last line of `precompact-flush-log` via `host::read_file` (if the file exists); (b) compares the SHA field (field 2) in that last line against the PreCompact flush commit SHA being evaluated; (c) checks the fourth field in that last line equals `commit` — this is the `git cat-file -t` token embedded by the shell hook at write time, resolving the WASM sandbox exec constraint (O-P3-004). All three checks must pass for EXEMPT determination. If the log is absent, prefix-match alone is sufficient (same as prior behavior for genuine absence). WASM corroboration reads the embedded `commit` token rather than execing git — this is the canonical design; any implementation that execs `git cat-file` from within WASM is incorrect and will fail at capability-check time.

3. **BC-7.07.001 side-channel write step revised**: PC6 (emit `precompact_flush_sha` to side-channel) is changed from: "Write commit SHA to `.factory/hooks/last-precompact-flush-sha`" to: "Append `<ISO-timestamp> <SHA> <cycle>/<step> <cat-file-type>` to `.factory/hooks/precompact-flush-log` BEFORE the git push step. The `<cat-file-type>` field is the output of `git cat-file -t <SHA>` on the just-created local commit (value: `commit`). The append must occur on the local filesystem after `git commit` succeeds but before `git push`, to establish crash-consistent ordering and to embed the cat-file verification token before any network failure can occur." On hook crash, the appended entry (if present) records the intent; WASM corroboration validates the SHA via the embedded fourth field.

4. **Retention and pruning ownership (O-P3-001 closure):** The log is never truncated by the flush hook itself. Log compaction (keeping only the last N entries) is a maintenance operation with an explicit owner: S-18.04 MUST include an AC that specifies the pruning policy and implements it as a periodic maintenance step (e.g., invoked by `check-state-health` or as a cron-style AC within S-18.04). The S-18.04 story-writer MUST add: "AC-N: Log pruning — `precompact-flush-log` is pruned to the most recent 500 entries when entry count exceeds 1000. Pruning is performed by a dedicated `precompact-flush-prune.sh` helper invoked by `check-state-health` skill." This is not a dangling sentence — it is attached to S-18.04 as a concrete AC, not deferred to an unnamed future cycle. Initial v1 implementation: unlimited growth until S-18.04 AC-N ships. Entries are ~120 bytes/line (four-field format); 1000 flushes ≈ 120KB — acceptable as a bridge until S-18.04 pruning lands.

**Exact BC change specifications for product-owner:**

- **BC-5.41.003** (validate-burst-log/validate-dispatch-advance exemption):
  - **F-P4-001 coherence note**: With BC-7.07.001 PC8 now exiting 2 on append failure, the log is always commit-consistent when compaction proceeds. PC1(a) exact-last-line match is therefore always correct for the compaction-allowed case. The stale-entry case (PC1 case (b): last-line SHA fails `git cat-file -t` → treat as absent → prefix match alone) covers the push-failure scenario: git commit landed locally, append succeeded with SHA_B, push failed → SHA_B is in the log but not yet on the remote. WASM reading the fourth field confirms `commit` but the SHA is not yet visible on `factory-artifacts` remote — treat as stale. This is correct behavior: the stale-entry toleration covers push-failure-before-remote-visibility, NOT append-failure (which now exits 2).
  - **O-P4-004 WASM/shell division (normative)**: PC1 and Inv1 MUST be unambiguous: `validate-burst-log` and `validate-dispatch-advance` are WASM plugins registered with NO `exec_subprocess` capability. They CANNOT exec `git cat-file` or any subprocess. The corroboration check works as follows: (1) WASM reads the last line of `precompact-flush-log` via `host::read_file`; (2) WASM checks the fourth field of that line — if it equals the string `commit`, the commit-type check passes WITHOUT any git exec; (3) the shell hook (`precompact-flush.sh`) embeds this token by calling `git cat-file -t <SHA>` at write time (shell HAS `exec_subprocess`). Any BC text that says "WASM runs `git cat-file -t`" or "verified via `git cat-file -t <SHA>` returning `commit`" is INCORRECT for WASM-context corroboration — it must say "WASM reads the fourth field of the log line (embedded by the shell hook at write time)". Shell-context gates (e.g., `wave-gate` skill, `BC-5.41.001 PC5`) CAN exec `git cat-file -t` directly because they run in a shell subprocess.
  - Postcondition 1 replacement wording for PO (see §F-P4-004 PO Wording below).
  - Postcondition 1, clause (a): Change "must appear in `.factory/hooks/last-precompact-flush-sha`" to "must appear as the SHA field (field 2) on the last line of `.factory/hooks/precompact-flush-log` AND the fourth field of that line must equal the string `commit` (the pre-embedded `git cat-file -t` token written by the shell hook)"
  - Postcondition 1, clause (b): Change "if the side-channel file is genuinely absent" to "if `.factory/hooks/precompact-flush-log` is genuinely absent (verified via `test -f` in shell-context; or `host::read_file` returning FileNotFound in WASM-context)"
  - EC-003: Replace current HEAD/HEAD^ truth table row 4 (`side-channel file exists with different SHA → NOT EXEMPT`) to match the new log semantics: "last line of `precompact-flush-log` exists with SHA that does not match the exempted commit's SHA → NOT EXEMPT (SHA mismatch; suspicious prefix-only commit not corroborated by log)"
  - Architecture Anchors: replace `.factory/hooks/last-precompact-flush-sha` with `.factory/hooks/precompact-flush-log` throughout. WASM corroboration reads via `host::read_file` — NO `exec_subprocess` — checks fourth field equals `commit`. Shell-context gates may exec `git cat-file -t` directly. The fourth log field IS the embedded `cat-file -t` token written by the shell hook.

- **BC-7.07.001** (precompact-flush.sh behavioral contract):
  - PC8 (exit-code contract) — **F-P4-001 change**: Change "append failure → exit 0 + stderr WARNING (fail-open)" to "append failure → **exit 2** (block compaction). Rationale: append failure leaves the log inconsistent with HEAD on factory-artifacts; BC-5.41.003 PC1(a) would then produce a false-positive MULTI_COMMIT_CHAIN_NOT_ALLOWED block on the next burst — the exact regression CAP-032 Part B exists to prevent. The `on_error = continue` fail-open remains active ONLY for full hook crashes before any exit code is emitted." Exit code taxonomy after this change: **only exit 0 and exit 2** (exit 0: success, clean-state, or STATE.md unreadable; exit 2: git commit failure OR append failure). Exit 1 is not used. The stale-entry toleration in BC-5.41.003 EC-003 now covers the push-failure scenario (append succeeded, push failed, SHA in log but not yet on remote) — this is a different scenario than append failure and remains valid.
  - PC6 / §Crash-Consistency Design note on step 6: The `git cat-file -t <SHA>` is executed by the SHELL hook (which has `exec_subprocess` capability) and its output (`commit`) is embedded as the fourth field of the log line. WASM validators read this fourth field statically — they do NOT exec `git cat-file`. This is the canonical shell/WASM division: shell execs, WASM reads the pre-embedded token.
  - Inv 4 and any test fixture referencing `last-precompact-flush-sha`: replace with `precompact-flush-log`. Update four-field log line format in test fixtures. Update exit-code table: remove exit 0 from append-failure path; replace with exit 2.
  - `[hooks.capabilities.read_file]` and `[hooks.capabilities.write_file]` path_allow: replace `last-precompact-flush-sha` with `precompact-flush-log`

- **BC-5.41.001** (wave-gate HANDOFF.md): The `precompact_flush_sha` hard cross-check (PC5, F-12) references the side-channel file — replace `.factory/hooks/last-precompact-flush-sha` with `.factory/hooks/precompact-flush-log` and update the read semantics to "read last line of log; extract SHA field (field 2); verify fourth field equals `commit` (embedded cat-file-t token)."

---

## F-P4-004 / F-P5-002 PO Wording (exact replacement text for BC-5.41.003 PC1 and Inv1)

This section provides the exact replacement wording for BC-5.41.003 PC1 and Inv1. It supersedes the v1.4 PO Wording with corrections from F-P5-002 (stale-entry re-grounding). The PO MUST apply these changes to BC-5.41.003 in the same burst. The PO MUST NOT add any wording implying WASM execs `git cat-file` — that exec happens only in the shell hook. The PO MUST use `log corruption or truncation` (not `write-before-push crash`) as the label for case (b) field-4 ≠ commit.

**Why the stale-entry label changes (F-P5-002):** The v1.4 PC1 described case (b) field-4 ≠ commit as a `write-before-push crash`. This is mechanically impossible: the shell hook appends a four-field line AFTER `git commit` succeeds locally; field-4 is always the string `commit` (the output of `git cat-file -t` on a commit object that already exists). A partial write that leaves field-4 incomplete or non-`commit` cannot arise from a push-failure crash — it would require filesystem-level truncation or corruption of the log file. The label is corrected to `log corruption or truncation`. The fallback behavior (treat as stale, fall through to prefix-match-alone case (c)) remains correct regardless of the true cause. Note: with reset-on-append-failure (Decision 6 step 6, v1.5), field-4 ≠ commit from a "never-appended" path is even more clearly impossible — the append either succeeds (field-4 = `commit`) or the commit is rolled back via reset --soft, leaving no orphan to corroborate.

### PC1 replacement (Postcondition 1)

Replace the entire current PC1 text (beginning "Both `validate-burst-log` and `validate-dispatch-advance` treat any commit...") with:

> **Exemption by prefix match + SHA corroboration (F-8, F-P4-001, O-P4-004, F-P5-002)**: Both `validate-burst-log` and `validate-dispatch-advance` treat any commit whose subject matches the pattern `^PreCompact flush ` as lifecycle-orthogonal, BUT ONLY when the commit's SHA can be corroborated against the side-channel log. The three-gate logic is:
>
> **(a) Log present and last-line SHA matches:** If `.factory/hooks/precompact-flush-log` exists (verified via `host::read_file` in WASM-context or `test -f` in shell-context) and its last line is non-empty: read the last line and parse its four fields (`<ISO-timestamp> <SHA> <cycle>/<step> <commit-token>`). Gate 1: the exempted commit's SHA MUST equal the SHA field (field 2) — exact 40-char hex match, no partial match. Gate 2: the commit-token field (field 4) MUST equal the string `commit`. **WASM gates read field 4 directly from the log line — they do NOT exec `git cat-file`. The shell hook embeds this token at write time by executing `git cat-file -t <SHA>`. WASM reads the pre-embedded token statically.** Both gates (SHA match + field-4 equals `commit`) must pass for EXEMPT determination.
>
> **(b) Log present but last-line SHA does not match OR field-4 corrupted (log corruption or truncation):** If the last-line SHA field is present and field 4 equals `commit` but the SHA does not match the exempted commit's SHA → the exempted commit is NOT corroborated → NOT EXEMPT (mismatch; suspicious prefix-only commit). If the last-line SHA field is present but field 4 does NOT equal `commit` (log corruption or truncation: the line was written but is damaged) → treat as stale entry → fall through to case (c). NOTE: field-4 ≠ commit does NOT indicate a `write-before-push crash` — field-4 is always written as the string `commit` (embedded by the shell hook after a successful local git commit); field-4 can only be non-`commit` due to filesystem-level corruption or truncation.
>
> **(c) Log genuinely absent or last line empty:** If the log is genuinely absent or the last line is empty → prefix match alone is sufficient for EXEMPT determination. This covers first-flush-ever (no prior log) and hook-crash-before-append scenarios (which trigger `on_error=continue` at the harness level). Note: under BC-7.07.001 PC8 (append failure → reset --soft + exit 2), no orphan local commit can exist without a log entry; case (c) arises only from hook crash BEFORE the commit step, not after.
>
> Exempt commits are excluded from the HEAD/HEAD^ chain comparison that detects `MULTI_COMMIT_CHAIN_NOT_ALLOWED`. NLP inference, regex over the full commit body, and sentiment analysis are forbidden.

### Inv1 replacement (Invariant 1)

Replace the entire current Inv1 text (beginning "Prefix + SHA corroboration, not subject-based inference alone") with:

> **Prefix + SHA corroboration, WASM reads embedded token (not exec)**: The exemption check has three gates: (1) prefix match: `subject.starts_with("PreCompact flush ")` on the raw commit subject string; (2) when `.factory/hooks/precompact-flush-log` exists and its last line is non-empty: parse field 2 (SHA) and field 4 (commit-token string); the exempted commit's SHA must equal field 2 AND field 4 must equal the string `commit`; (3) if the log is genuinely absent, empty, or field 4 does not equal `commit` (log corruption or truncation — NOT a write-before-push crash; that label is mechanically impossible per Decision A), the prefix match alone is sufficient. **WASM gates (`validate-burst-log`, `validate-dispatch-advance`) read field 4 from the log via `host::read_file` — they CANNOT exec `git cat-file` (no `exec_subprocess` capability). The shell hook (`precompact-flush.sh`) executes `git cat-file -t <SHA>` and embeds the result as field 4 at write time. Any implementation that execs git from within WASM is incorrect and will fail at capability-check time.** NLP inference, regex over the full commit body, and sentiment analysis are all forbidden.

### EC-003 row 4 replacement

The SHA-mismatch row in EC-003 currently reads: `HEAD="PreCompact flush injected"; HEAD^="state: burst-X"; precompact-flush-log last-line SHA is a different SHA (mismatch) → NOT EXEMPT`. Replace with:

> `HEAD="PreCompact flush injected"; HEAD^="state: burst-X"; precompact-flush-log last-line SHA field (field 2) is a different SHA (mismatch) AND field 4 equals "commit" → NOT EXEMPT (SHA corroboration fails; suspicious prefix-only commit).`

Add a new row after EC-003 row 4 (F-P5-002 label correction):

> `HEAD="PreCompact flush v1.0/S-18.04 ..."; precompact-flush-log last-line field 4 does NOT equal "commit" (log corruption or truncation — NOT a write-before-push crash; that cause is mechanically impossible per Decision A) → EXEMPT (stale-entry case; prefix match alone sufficient).`

### BC-7.07.001 PC8 update — reset-on-append-failure (F-P5-002)

> **SUPERSEDED by §F-P6-006 (v1.6): The v1.5 PC8 text below used `HEAD~1` — that form has been replaced by the SHA-pinned `SHA_B^` form with a concurrent-commit guard (HEAD == SHA_B precondition; else exit 2 with human-intervention error). Use the §F-P6-006 PC8 text as the authoritative prescription. The v1.5 text is retained below only as historical context showing the F-P5-002 evolution.**

**Existing PC8 text (v1.4):** "append failure → exit 2 (block compaction)."

**Replacement (v1.5 — historical; superseded by v1.6 §F-P6-006):**

> **PC8 (exit-code contract, v1.5 — F-P5-002; superseded by v1.6 §F-P6-006 SHA-pinned form):** On append failure: (1) execute `git -C <factory-artifacts-worktree> reset --soft HEAD~1` to undo the local commit BEFORE exiting; (2) then exit 2 (block compaction). [NOTE: `HEAD~1` form superseded at v1.6 by `SHA_B^` with concurrent-commit guard per §F-P6-006 — see that section for the normative form.] The soft reset eliminates the orphan local commit (SHA_B) that would otherwise be never-logged and never-pushed. Without the reset, SHA_B is real in local git history; on retry, SHA_C is committed and logged; `validate-burst-log` reads only the last log line (SHA_C) and cannot find SHA_B → false-positive `MULTI_COMMIT_CHAIN_NOT_ALLOWED`. The reset restores HEAD to the pre-flush commit; the retry commits SHA_C from the same staged state. If the reset itself fails, exit 2 immediately; human intervention required. Exit code taxonomy (unchanged from v1.4 in net effect, but with reset step added before exit 2 on append failure): exit 0: success, clean-state, or STATE.md unreadable; exit 2: git commit failure OR (append failure after reset) OR reset failure.

**Note on test vectors:** Any BC-7.07.001 test vector for append-failure MUST be updated to assert: (a) the SHA-pinned reset `git reset --soft SHA_B^` (NOT `HEAD~1`) is executed before exit 2 when HEAD == SHA_B; (b) when HEAD != SHA_B (concurrent commit), no reset is executed and exit 2 fires with human-intervention error; (c) the local commit is absent from git log after the reset (case a only); (d) exit code is 2 in all append-failure scenarios. Use §F-P6-006 test vectors as authoritative.

---

## F-P6-006 Concurrent-Commit Race Guard for Reset-on-Append-Failure

**Problem (F-P6-006 MAJOR):** The v1.5 Decision 6 step 6 prescribes `git -C <factory-artifacts-worktree> reset --soft HEAD~1` on append failure. This assumes HEAD equals SHA_B (the just-created local flush commit). But there is a concrete race: PreCompact fires between LLM turns, and `factory_lock` is OPT-IN (default absent = no lock held per ADR-025). A concurrent state-manager burst COULD land a commit on factory-artifacts between the local `git commit` (creating SHA_B) and the attempted `git reset --soft HEAD~1`. In that scenario, HEAD has advanced to SHA_C (the state-manager commit) when the reset executes; `git reset --soft HEAD~1` discards SHA_C — a silent concurrent commit loss. This is strictly WORSE than the original orphan-SHA_B problem: SHA_C was a legitimate state-manager burst commit that is now silently gone from the local tree. SOUL.md Rule 4 (no silent data loss) is violated.

**Additionally:** The v1.5 rationale stated "The soft reset restores HEAD to the pre-flush commit; the retry commits SHA_C from the same staged state." This overstates the guarantee. STATE.md may be modified by the concurrent state-manager burst between the failed flush and the retry. The working tree after `--soft` reflects the staged state at SHA_B commit time, not necessarily the same state as the retry will produce. The correct statement: `--soft` restores the working tree index to what was staged for SHA_B; intervening changes to other files (e.g., STATE.md updated by a concurrent burst) will be included in the retry commit as uncommitted working-tree changes if they were not staged.

### Corrected Decision 6 step 6 (v1.6 replacement, normative)

Replace the v1.5 step 6 reset prescription with:

> **Append failure → SHA-pinned reset or hard exit (F-P5-002 + F-P6-006):** If the append to `precompact-flush-log` fails after a successful local `git commit`:
>
> 1. Capture SHA_B immediately after `git commit`: `SHA_B=$(git -C <worktree> rev-parse HEAD)`
> 2. Check whether HEAD has moved: `CURRENT_HEAD=$(git -C <worktree> rev-parse HEAD)`
> 3. **If CURRENT_HEAD == SHA_B** (no concurrent commit landed): execute `git -C <worktree> reset --soft SHA_B^` (equivalent to `HEAD~1` but pinned to the known pre-flush parent). Exit 2 after the reset succeeds.
> 4. **If CURRENT_HEAD != SHA_B** (a concurrent commit landed between step 1 and step 2): do NOT reset. Exit 2 immediately with the error message: `precompact-flush: append failed; concurrent commit advanced HEAD beyond SHA_B. SHA_B=<sha_b>; current HEAD=<current_head>. Human intervention required: verify factory-artifacts HEAD and remove orphan commit SHA_B if present.`
> 5. If the reset itself fails (step 3): exit 2 immediately; human intervention required.
>
> **Rationale for SHA-pinned reset vs HEAD~1:** `HEAD~1` is a relative reference that moves as HEAD moves. If a concurrent commit lands between SHA_B creation and the reset, `HEAD~1` points to the concurrent commit (SHA_C), not to the pre-flush parent. The SHA-pinned form `reset --soft SHA_B^` resolves the parent of the KNOWN flush commit, which is always the pre-flush HEAD regardless of any intervening commits on the ref.
>
> **Concurrency note:** The check in steps 1-2 is a best-effort TOCTOU window check, not an atomic operation. A concurrent commit CAN still land between steps 1 and 2. The check closes the COMMON case (slow git push timeout; no competing session); it does not fully close the adversarial race (two concurrent sessions simultaneously flushing). Full closure requires the factory lock (ADR-025) to be held; PreCompact flush SHOULD hold the lock when any concurrent work is expected.
>
> **Honest staged-state behavior:** `--soft` restores the index to the state staged for SHA_B. Files modified by a concurrent state-manager burst (e.g., STATE.md updated between SHA_B and the reset) are NOT reverted — they appear as unstaged working-tree changes in the retry's working directory. The retry commit WILL include these changes if the retry re-stages and re-commits. This is the correct behavior for the common case (STATE.md drift is acceptable as a carry-forward). Implementers MUST document this in `precompact-flush.sh` header comments.

### BC-7.07.001 PC8 update — concurrent-commit guard (F-P6-006)

**Replace the v1.5 PC8 text** with:

> **PC8 (exit-code contract, v1.6 — F-P5-002 + F-P6-006):** On append failure: (1) capture SHA_B = `git rev-parse HEAD` immediately after the local commit succeeds; (2) compare CURRENT_HEAD = `git rev-parse HEAD` to SHA_B; (3a) IF CURRENT_HEAD == SHA_B: execute `git -C <worktree> reset --soft SHA_B^` (NOT `HEAD~1` — SHA-pinned); then exit 2; (3b) IF CURRENT_HEAD != SHA_B: do NOT reset; exit 2 with error `precompact-flush: append failed; concurrent commit advanced HEAD; SHA_B=<sha_b>; human intervention required`; (4) If the reset fails: exit 2; human intervention required. Exit code taxonomy: exit 0: success, clean-state, or STATE.md unreadable; exit 2: git commit failure OR append failure (after SHA-pinned reset or concurrent-commit guard) OR reset failure.

**Test vectors to add:** (a) normal append-failure path: SHA-pinned reset executes, local commit absent after reset, exit 2; (b) concurrent-commit path: HEAD advanced before reset; reset NOT executed; exit 2 with error message; (c) reset-failure path: exit 2 with human-intervention message.

---

## F-P10-001 PO Wording — Wave-1 Discriminator Corrected to Payload-Only

**Problem (F-P10-001 MAJOR):** BC-4.14.001 §Description, PC3, PC4, PC8, Inv3, EC-001/002/003/005/006/010, test vectors, and VP-083/VP-081 row descriptions all frame the wave-1 no-op discriminator as reading sprint-state.yaml (wave-group position 1) OR probing factory-artifacts for prior HANDOFF.md absence. This requires external filesystem/git reads, which directly violates BC-4.14.001 Invariant 1 (pure-parse, no git/filesystem side effects). This is the same defect class as F-P7-001/F-P8-001 (which fixed the EPIC-COMPLETE discriminator to payload-only), applied to the sibling wave-1 discriminator that was never swept in those passes.

**Resolution (F-P10-001):** The HANDOFF.md payload already contains `wave_id` as Field 1 of the 9 base required fields. The gate computes `is_first_wave = (payload.wave_id == 1)` from the current HANDOFF.md Write/Edit payload — PURELY from the parsed payload, no external read. VP-083 carries the correct intent (`is_first_wave: bool` pre-computed by its caller); this decision makes the implementation pathway explicit: the gate is the one computing `is_first_wave` from `payload.wave_id == 1` (not from a dispatcher-injected context field), which is equally pure-parse. The 'prior HANDOFF.md absence / sprint-state.yaml ordinal' language MUST be removed from every site in BC-4.14.001 that applies to the WASM GATE's own behavior.

**Division of labor (preserved):** The wave-handoff SKILL (shell-context, BC-5.41.001) continues to derive `wave_id` from real substrate (sprint-state.yaml topo-sort for product pipelines; STATE.md pass number for engine). That derivation is validated by the anti-fabrication cross-check in BC-5.41.001 PC2. The WASM gate is a consumer of the already-derived `wave_id` field — it reads it from the payload and compares to 1. No behavioral change to the shell-context wave-gate is needed.

### BC-4.14.001 change spec (exact sites — PO MUST apply in next burst)

The PO MUST apply the following changes to BC-4.14.001. The principle: **remove every occurrence of 'sprint-state.yaml', 'wave-group position', 'prior HANDOFF.md', 'factory-artifacts', or 'wave context cannot be determined' from the WASM gate's own behavior description.** Those concepts belong to the shell-context wave-gate (BC-5.41.001), not the WASM gate.

**Version bump:** BC-4.14.001 v1.7 → v1.8.

**H1 amendment:** Append `; wave-1 no-op is payload-only: payload.wave_id == 1 → Continue unconditionally` to the H1 title (no other H1 change needed).

**§Description (line ~41) replacement:** Replace:

> "It is a strict no-op (returns `Continue`) when the pipeline context is the first wave (wave-group position 1 per sprint-state.yaml dependency order OR no prior HANDOFF.md exists on factory-artifacts) OR when the tool call does not target `HANDOFF.md`."

With:

> "It is a strict no-op (returns `Continue`) when the parsed HANDOFF.md payload has `wave_id == 1` OR when the tool call does not target `HANDOFF.md`. The gate computes `is_first_wave = (payload.wave_id == 1)` from the Write/Edit tool-call payload — no sprint-state.yaml read, no factory-artifacts read, no filesystem or git access. The `wave_id` field (Field 1 of the 9 base required fields) is derived from real substrate by the wave-handoff skill (caller) before writing HANDOFF.md; the WASM gate is a consumer of the pre-computed value."

**PC3 (wave-1 no-op postcondition) replacement:** Replace:

> "When the pipeline context is the first wave (wave-group position 1 per sprint-state.yaml dependency order, OR when no prior HANDOFF.md exists on factory-artifacts, OR when wave context cannot be determined), the gate returns `Continue` unconditionally without parsing HANDOFF.md."

With:

> "When the parsed HANDOFF.md payload has `wave_id == 1`, the gate returns `Continue` unconditionally. The gate reads `wave_id` from the Write/Edit tool-call payload — no sprint-state.yaml read, no factory-artifacts probe, no git access. `wave_id` Field 1 MUST be parseable for this check; if `wave_id` is absent from the payload, the gate falls through to full validation (not no-op) — the gate does NOT treat missing `wave_id` as wave-1."

**PC4 (precondition — Precondition 4) replacement:** Replace the existing PC4 text about "wave identity context is determinable from real substrate: sprint-state.yaml wave-group ordering... OR derives first-wave status from the absence of a prior HANDOFF.md on factory-artifacts" with:

> "The HANDOFF.md payload being written contains a `wave_id` field. This field is authored by the wave-handoff skill from real substrate before writing; the WASM gate reads it from the Write/Edit payload to determine wave identity without any external read. No phantom `current_wave:` field is referenced — this field does not exist on STATE.md."

**PC8 (wave-1 no-op is unconditional) replacement:** Replace:

> "Wave-1 no-op is unconditional: Even if `HANDOFF.md` is written with deliberate content on the first wave, the gate does not validate it. Validation only activates when the pipeline is on wave > 1 (or when wave context cannot be determined from real substrate, defaulting to fail-open Continue)."

With:

> "Wave-1 no-op is unconditional: if `wave_id == 1` in the payload, the gate returns `Continue` without validation, regardless of HANDOFF.md content. Validation only activates when `wave_id > 1` in the payload. There is no 'wave context cannot be determined' fail-open for the WASM gate — if `wave_id` is absent from the payload, the gate proceeds to full validation (fail-closed on the missing field, not fail-open)."

**Inv3 replacement:** Replace:

> "No-op conditions are checked first: The gate checks the no-op conditions (non-HANDOFF.md target, wave-1) before any parsing. A gate that parses HANDOFF.md and then returns Continue on wave-1 is correct in outcome but wastes fuel unnecessarily."

With:

> "No-op conditions are checked first: The gate checks (1) non-HANDOFF.md target → Continue; (2) `wave_id == 1` in payload → Continue. Checking `wave_id` requires parsing the `wave_id` field from the payload (minimal parse), not a full validation pass. A gate that fully validates and then returns Continue on wave-1 is correct in outcome but wastes fuel. No external read (sprint-state.yaml, factory-artifacts, git) is performed at any point during the no-op check."

**EC-001 replacement:** Replace:

> "HANDOFF.md write on first wave (no prior HANDOFF.md on factory-artifacts; or wave-group position 1 per sprint-state.yaml)"

With:

> "HANDOFF.md write with `wave_id: 1` in payload"

Expected behavior unchanged: `Continue (wave-1 no-op); no validation`.

**EC-002 replacement:** Replace:

> "HANDOFF.md write; all 9 fields present; wave-group position 2 per sprint-state.yaml (not first wave — prior HANDOFF.md exists on factory-artifacts)"

With:

> "HANDOFF.md write; `wave_id: 2` in payload; all 9 fields present"

Expected behavior unchanged: `Continue; no block`.

**EC-003 replacement:** Replace:

> "HANDOFF.md write; `last_verified_develop_sha` field missing; not first wave (prior HANDOFF.md exists on factory-artifacts)"

With:

> "HANDOFF.md write; `wave_id: 2` in payload; `last_verified_develop_sha` field missing"

Expected behavior unchanged: `HandoffIncomplete: ["last_verified_develop_sha"]`.

**EC-005 replacement:** Replace:

> "HANDOFF.md write; 4 fields missing; wave-group position 3 per sprint-state.yaml (not first wave)"

With:

> "HANDOFF.md write; `wave_id: 3` in payload; 4 fields missing"

Expected behavior unchanged: `HandoffIncomplete: names all 4 missing fields in one message`.

**EC-006 replacement:** Replace:

> "HANDOFF.md write; `precompact_flush_sha: null`; not first wave (prior HANDOFF.md exists on factory-artifacts)"

With:

> "HANDOFF.md write; `wave_id: 2` in payload; `precompact_flush_sha: null`"

Expected behavior unchanged: `Continue if null is explicitly permitted...`.

**EC-010 replacement:** Replace:

> "Wave context cannot be determined (sprint-state.yaml absent; factory-artifacts unreachable; STATE.md unreadable)"

With:

> "`wave_id` field absent from HANDOFF.md payload"

Expected behavior: **fail-closed — proceed to full validation (gate attempts to validate all 9 required fields; `wave_id` will fail as missing → `HandoffIncomplete: ["wave_id"]`)**. NOTE: This reverses the prior fail-open default for indeterminate context. The WASM gate is pure-parse: it cannot probe factory-artifacts or sprint-state.yaml. If `wave_id` is absent from the payload, the gate treats it as wave > 1 and validates, which correctly catches a malformed HANDOFF.md missing its first required field.

**Test vector (line ~122) replacement:** Replace:

> "Write to HANDOFF.md; first wave (no prior HANDOFF.md on factory-artifacts) | Continue | wave-1-no-op"

With:

> "Write to HANDOFF.md; `wave_id: 1` in payload | Continue | wave-1-no-op"

**Test vector (line ~123) replacement:** Replace:

> "Write to HANDOFF.md; second wave (prior HANDOFF.md exists on factory-artifacts); all 9 fields present + well-formed | Continue | happy-path"

With:

> "Write to HANDOFF.md; `wave_id: 2` in payload; all 9 fields present + well-formed | Continue | happy-path"

**VP-083 row (§Verification Properties table) replacement:** Replace:

> "validate-wave-handoff-completeness returns Continue unconditionally on first wave (wave-group position 1 per sprint-state.yaml, OR absence of prior HANDOFF.md on factory-artifacts); also no-op on non-HANDOFF.md writes"

With:

> "validate-wave-handoff-completeness returns Continue unconditionally when payload `wave_id == 1` (gate reads `wave_id` from the Write/Edit tool-call payload — no sprint-state.yaml read, no factory-artifacts probe, no git access); also no-op on non-HANDOFF.md writes"

**VP-081 row (§Verification Properties table) replacement:** Replace:

> "Gate blocks HandoffIncomplete when any required ADR-026 §D2 field is missing on a HANDOFF.md write on a non-first wave (prior HANDOFF.md exists on factory-artifacts, OR wave-group position > 1 per sprint-state.yaml)"

With:

> "Gate blocks HandoffIncomplete when any required ADR-026 §D2 field is missing on a HANDOFF.md write when payload `wave_id > 1`"

**VP-INDEX row updates (PO + state-manager in same burst):**
- VP-083 Full Index row description: update to remove 'prior-HANDOFF.md absence on factory-artifacts OR sprint-state.yaml topo-sort ordinal=1'; replace with 'payload `wave_id == 1` (gate reads wave_id from Write/Edit tool-call payload — no external read)'
- VP-081 Full Index row description: update to remove 'wave_id > 1' framing that references 'prior HANDOFF.md exists on factory-artifacts, OR wave-group position > 1 per sprint-state.yaml'; replace with 'payload `wave_id > 1`'

---

## F-P10-002 Note — ADR §Decision 6 Push-Step Contract (for PO BC-7.07.001 alignment)

**Problem (F-P10-002 MEDIUM — PO-owned):** BC-7.07.001 PC6 and EC-003 conflate commit and push steps, and PC8 mislabels some commit-vs-push failure modes. The finding is PO-owned (BC authoring domain), but the ADR must state the push-step contract clearly so PO can align BC-7.07.001.

**ADR §Decision 6 push-step contract (normative):**

Decision 6 step 5 prescribes:
> "Exits with `exit 2` (blocking) if the flush was required and the commit did not land successfully (git commit failure, git push failure). Exits 0 if the flush landed or was not needed (no state changes since last flush)."

The canonical two-leg contract is:
1. **Local commit leg:** `git commit` creates SHA_B on the local `factory-artifacts` worktree. Commit failure (git error) → exit 2 immediately (no push attempted; no log entry).
2. **Remote push leg:** After the local commit and log append succeed, `git push` sends SHA_B to the remote. Push failure → exit 2. The local commit (SHA_B) and log entry already exist; the next retry will attempt to re-push without re-committing (idempotent retry).

**Exit code taxonomy (normative, from §F-P6-006):**
- `exit 0`: flush not required (clean state) OR flush committed AND pushed successfully OR STATE.md unreadable (fail-open)
- `exit 2`: local git commit failure; OR log append failure (after SHA-pinned reset per §F-P6-006); OR git push failure; OR reset failure

**PO action required (F-P10-002):** PO MUST align BC-7.07.001 PC6, PC8, and EC-003 to match this two-leg contract. Specifically:
- PC6 description MUST NOT conflate commit and push into a single step. It MUST describe: (a) local commit creates SHA_B; (b) log append with SHA_B (four-field format); (c) git push SHA_B to remote.
- PC8 exit-code table MUST distinguish push failure (exit 2; local commit and log entry already present; retry is push-only) from commit failure (exit 2; no local commit, no log entry).
- EC-003 (if present) MUST distinguish "push failure after successful local commit" from "commit failure". These produce different local state and different retry behavior.
- The SHA-pinned reset (§F-P6-006) applies ONLY to log-append failure after local commit — it does NOT apply to push failure after log append (the local commit is already logged; push retry is safe without reset).

---

## F-P6-004 PO Wording — HANDOFF.md Field-Count Reconciliation

**Problem (F-P6-004 MAJOR):** The HANDOFF.md schema in Decision 2 specifies 9 fields in the schema table. The Terminal-Wave Discriminator (§Terminal-Wave Discriminator) introduces `epic_status: "complete"` as a required field on the final wave. But BC-4.14.001 §Description says "all 9 required fields specified in ADR-026 §Decision 2" in multiple load-bearing places, and BC-5.41.001 PC2 also says "All 9 required fields present." These specs are now inconsistent: `epic_status` is a 10th field but 7+ load-bearing places say "9 required fields."

**Resolution (normative):**

The correct model is: **9 base required fields + `epic_status` conditionally required on the final/EPIC-COMPLETE wave only.**

- `epic_status` is NOT a base required field. On non-final waves, it MUST be absent from HANDOFF.md (not present as `null`, not present as any value — genuinely absent). Presence of `epic_status` on a non-final wave is a malformed HANDOFF.
- On the final wave (EPIC-COMPLETE: all stories terminal, `next_wave_stories: []`), `epic_status: "complete"` is required. Absence on the final wave is a malformed HANDOFF.
- The validate-wave-handoff-completeness WASM gate (BC-4.14.001) must implement conditional field validation: on detecting EPIC-COMPLETE context (no prior HANDOFF.md entries with pending/draft stories), add `epic_status` to the required field set. On non-EPIC-COMPLETE waves, `epic_status` MUST NOT be required and MUST NOT be present.

### Decision 2 schema amendment (add conditional field row)

Amend the Decision 2 schema table by adding this row after `factory_lock_holder`:

| Field | Type | Source (real substrate) | Anti-fabrication rule | Conditionality |
|-------|------|------------------------|----------------------|----------------|
| `epic_status` | string `"complete"` OR **absent** | EPIC-COMPLETE discriminator: all sprint-state.yaml stories in terminal status AND `next_wave_stories: []` | Must equal `"complete"` when present; derived from sprint-state.yaml terminal-status exhaustion — NOT from in-context assertion | **CONDITIONAL: required ONLY on the final wave (EPIC-COMPLETE); MUST be absent on all non-final waves** |

### BC change specifications for PO

**BC-4.14.001** (validate-wave-handoff-completeness WASM gate):

- H1 and §Description: Replace every occurrence of "all 9 required fields" with "all 9 base required fields (and `epic_status: complete` additionally required on the EPIC-COMPLETE wave)".
- Add to PC1 (or a new PC adjacent to the field-validation PC): "**Conditional field validation (payload-only discriminator — F-P7-001):** The gate determines EPIC-COMPLETE context from the CURRENT HANDOFF.md payload being written — it reads ONLY the Write/Edit tool-call payload; NO git read, NO filesystem read, NO prior-HANDOFF.md read. The discriminator: if `next_wave_stories: []` in the current payload → EPIC-COMPLETE branch; if `next_wave_stories` is non-empty → non-EPIC-COMPLETE branch. When EPIC-COMPLETE context is detected, the gate adds `epic_status` to the required field set and validates it equals `complete`. When NOT in EPIC-COMPLETE context (non-empty `next_wave_stories`), the gate must FAIL with `UnexpectedEpicStatus` if `epic_status` is present (presence on a non-final wave is malformed). **Rationale:** BC-4.14.001 Invariant 1 mandates pure-parse semantics — the WASM gate reads only the tool-call payload. Reading the prior HANDOFF.md from factory-artifacts (a git/filesystem read) violates the pure-parse constraint and is forbidden. The richer 'is this genuinely the final wave vs broken-sprint' judgment lives in wave-gate/wave-handoff BC-5.41.002, which IS shell-context and CAN read sprint-state.yaml. Division: WASM completeness gate does payload-only structural validation; the shell wave-gate does substantive terminal-state discrimination."
- Test vector to add: (a) non-final wave HANDOFF.md with `epic_status: complete` → gate returns `HandoffIncomplete` with `UnexpectedEpicStatus`; (b) final-wave HANDOFF.md with `next_wave_stories: []`, all terminal, and `epic_status: complete` → gate returns `Continue`; (c) final-wave HANDOFF.md missing `epic_status` → gate returns `HandoffIncomplete` with `MissingEpicStatus`.

**BC-5.41.001** (wave-gate HANDOFF.md behavioral contract):

- PC2 heading and body: Replace "All 9 required fields present" with "All 9 base required fields present; `epic_status: complete` additionally required on EPIC-COMPLETE wave".
- Add to PC2 field list: "`epic_status` — string `complete`; CONDITIONAL: required on EPIC-COMPLETE wave only; MUST be absent on non-final waves."
- Test vector to update: "wave-gate close with all 9 fields correct + all cross-checks pass → HANDOFF.md committed" should add branching: non-final wave (9 fields, no epic_status) and final wave (9 fields + epic_status: complete).

---

## Terminal-Wave Discriminator (F-P2-004)

**Problem:** Decision 2 and Decision 4 currently make empty `next_wave_stories` an unconditional hard error. But the FINAL wave of an epic has no next-wave stories by design — all stories are `merged` or `withdrawn`. The hard error fires on the legitimate final wave, blocking EPIC-COMPLETE.

**Discriminator rule (normative):**

> When `next_wave_stories` is empty (no sprint-state.yaml entries with `status: pending` or `status: draft`), the behavior depends on all other story statuses:
>
> - **EPIC-COMPLETE path**: all stories in sprint-state.yaml are in terminal states (`merged`, `withdrawn`, OR `cancelled`) AND the empty pending/draft set is a true exhaustion → `wave-handoff` MUST declare EPIC-COMPLETE success. The HANDOFF.md is written with `next_wave_stories: []` and a top-level field `epic_status: "complete"`. No `wave-state.yaml` is produced (there is no next wave). No error.
>
> - **BROKEN-SPRINT-STATE path**: at least one story in sprint-state.yaml is in a non-terminal state other than `pending` or `draft` (e.g., `partial`, `in-progress`, `blocked`, `unknown`) AND no `pending/draft` stories exist → hard error: `BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states.`
>
> Terminal states are: `merged`, `withdrawn`, `cancelled`. Non-terminal active states are: `pending`, `draft`, `partial`, `in-progress`, `blocked`. Unknown/unrecognized status values are treated as non-terminal for safety.
>
> **Rationale for `cancelled` as terminal (F-P3-001 adjudication):** A story may be cancelled by explicit human decision — descoped, superseded, or blocked indefinitely with no intent to resume. In that case the work is done: no forward effort remains, and the epic is legitimately complete. Treating `cancelled` as non-terminal would force a BROKEN-SPRINT-STATE error on a valid final-wave scenario. BC-5.41.002 v1.2 already correctly includes `cancelled` in the EPIC-COMPLETE set. ADR-026 is the authoritative architectural source and must agree. Product-owner does NOT need to change BC-5.41.002 — it is already aligned. This is a pass-3 closure: ADR-026 catches up to the BC, not the other way around.

**HANDOFF.md on EPIC-COMPLETE:** The HANDOFF.md for the final wave:
- `next_wave_stories: []` (explicitly empty — not absent)
- `epic_status: "complete"` (new field; optional on non-final waves; required on final wave)
- All other fields still required and anti-fabrication cross-checked
- No `wave-state.yaml` is produced. `wave-gate` declares the epic complete and surfaces EPIC-COMPLETE to the operator.

**Exact BC change specifications for product-owner:**

- **BC-5.41.002** (wave-state.yaml manifest):
  - **STATUS:** BC-5.41.002 v1.2 is already correctly aligned. It includes `cancelled` in the terminal set for the EPIC-COMPLETE path (EC-001a) and in the BROKEN-SPRINT-STATE exclusion clause. Product-owner does NOT need to modify BC-5.41.002 for F-P3-001. The only BC-5.41.002 action needed is confirming the corroboration language (EC-001a test vector `precompact-flush-log` last-line reference) is already present or adding it if absent.
  - **Verification:** The F-P3-001 reconcile is CLOSED by ADR-026 v1.3 expanding the terminal set to include `cancelled`. ADR-026 is now the authoritative architectural source. BC-5.41.002 EC-001a and the body's EPIC-COMPLETE clause (`merged, withdrawn, or cancelled`) are correct as-authored in v1.2.

- **BC-6.24.001** (rehydrate-wave):
  - Add EC to edge cases: "EC-004 replacement: `wave-state.yaml` absent because EPIC-COMPLETE was declared on final wave" → "`rehydrate-wave` must not attempt rehydration if it detects EPIC-COMPLETE state from HANDOFF.md. If `/rehydrate-wave` is invoked after EPIC-COMPLETE, the skill surfaces: 'This epic is complete (EPIC-COMPLETE declared in HANDOFF.md). No next wave to rehydrate.' No injection; no error."
  - Note: the existing EC-004 (`wave-state.yaml stories: []`) should be updated to indicate that a `[]` stories list with `epic_status: complete` in HANDOFF.md is valid on the final wave, not an error.

---

## v2 Deferrals (explicit, with rationale)

| Deferred Capability | Deferral Reason | Tracking |
|--------------------|----------------|---------|
| Auto-reset at wave boundary | Destructive irreversible action; requires human safeguard in v1 | E-18 follow-on or separate feature cycle |
| RAG over spec corpus for rehydration | Non-deterministic; hallucination risk; deterministic manifest is safer in v1 | E-18 follow-on or separate feature cycle |
| Per-autonomy-level threshold configuration | Single global default (70%) is sufficient for v1 | E-18 follow-on |
| `validate-heavy-op-delegation` blocking mode | Requires false-positive calibration before blocking | S-18.06 F3 adversarial review → promotion if rate acceptable; BC authored at promotion time |

---

## Deliverables (for story-writer reference in F3)

| Deliverable | Story | Subsystem(s) |
|-------------|-------|-------------|
| `HANDOFF.md` schema (defined in this ADR §Decision 2) | S-18.01 | SS-05 |
| `wave-handoff` skill | S-18.01 | SS-06 |
| `wave-state.yaml` manifest schema (defined in this ADR §Decision 4) | S-18.01 | SS-05 |
| S-18.00 dispatcher routing verification/addition for PreCompact/PostCompact | S-18.00 | SS-01 |
| `validate-wave-handoff-completeness` WASM crate (`crates/hook-plugins/validate-wave-handoff-completeness/`) | S-18.02 | SS-04 |
| `hooks-registry.toml` WASM gate entry for `validate-wave-handoff-completeness` | S-18.02 | SS-07 |
| `wave-reset` skill (loads `wave-state.yaml`, rehydrates session) | S-18.03 | SS-06 |
| `precompact-flush.sh` shell hook with 30s timeout | S-18.04 | SS-07 |
| `hooks-registry.toml` PreCompact entry (corrected TOML schema) | S-18.04 | SS-07 |
| `postcompact-reanchor.sh` advisory hook (best-effort, not in CAP-032 guarantee chain) | S-18.05 | SS-07 |
| `hooks-registry.toml` PostCompact entry (corrected TOML schema) | S-18.05 | SS-07 |
| `validate-heavy-op-delegation` WASM crate (advisory mode; no BC in v1) | S-18.06 | SS-04 |
| Terminology disambiguation: `compact-state/SKILL.md`, `check-state-health/SKILL.md`, `CLAUDE.md` callout | S-18.07 | SS-06, SS-08 |
| `check-harness-version.sh` advisory hook OR check-state-health extension | S-18.04 or S-18.00 | SS-07, SS-06 |
| `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` settings.json verification in check-state-health | S-18.03 or S-18.07 | SS-06 |

All story estimates are per the F1 provisional decomposition in the F1 delta analysis. Story-writer produces final AC-level decomposition in F3.

---

## Dependency Chain

```
S-18.00 (dispatcher routing verification; prerequisite field/artifact verification)
    |
S-18.01 (HANDOFF.md schema + wave-handoff skill; derives next_wave_stories from sprint-state.yaml)
    |
    +---> S-18.02 (validate-wave-handoff-completeness WASM)
    |         |
    +---> S-18.04 (precompact-flush.sh; reads current_cycle+current_step; lock-renew no-op when unlocked)  [depends_on: S-17.04 for lock renewal]
              |
              +---> S-18.03 (wave-reset skill)
                        |
                        +---> S-18.05 (postcompact-reanchor.sh — advisory, not in guarantee chain)
                        +---> S-18.06 (validate-heavy-op-delegation WASM — advisory)
                                    |
                                    +---> S-18.07 (terminology disambiguation — doc-only, no BC)
```

Dependency on E-17: S-18.04 (`precompact-flush.sh`) MUST invoke `factory-lock-write.sh renew` when lock is held per ADR-025 Decision 11 Mechanism 1. When `factory_lock` is absent from STATE.md, the lock-renew call is skipped (no-op). Story S-18.04 MUST declare `depends_on: [S-17.04]` at story-writer authoring time.

---

## Harness Version Precondition

**REQUIRED:** Claude Code harness version ≥ v2.1.105.

The PreCompact blocking capability (Decision 6) is a no-op on harness versions < v2.1.105. On pre-v2.1.105, `PreCompact` fires as a notification-only hook; `exit 2` shows stderr to the user but does NOT block compaction. **On pre-v2.1.105, the CAP-032 Part B guarantee is NOT satisfied — context loss remains possible.** This is not a safe degradation. The E-18 Part B stories MUST document this precondition in their AC-001.

The current operator harness version is confirmed as Claude Code v2.1.177 (>= v2.1.105) per F1 delta analysis — this precondition is currently satisfied.

Runtime check: Decision 13 wires an active assertion via check-state-health skill and SessionStart advisory hook.

---

## VP Allocations (F-10)

| VP | Title | BC | Type |
|----|-------|-----|------|
| VP-081 | Wave Cannot Close Without Verified Handoff | BC-5.41.001, BC-4.14.001 | safety/integration |
| VP-082 | PreCompact Flush Commits Before Compaction | BC-7.07.001 | safety/integration |
| VP-083 | Completeness Gate Is No-Op on Wave-1 | BC-4.14.001 | invariant/unit-test |
| VP-084 | PreCompact Flush Lifecycle Distinct From Burst | BC-5.41.003 | invariant/integration |
| VP-085 | PreCompact Flush Hook Is Hermetic | BC-7.07.001 | safety/unit-test |
| **VP-086** | **Dispatcher Exit-2 Propagation for PreCompact Block-Intent** | **BC-1.15.001** | **safety/integration** |

VP-086 is the concrete verification property for BC-1.15.001 PC4 (exit-2 propagation from PreCompact plugin to harness). This was the only BC with `TBD-VP` in its Traceability section and represents a safety-critical linchpin: if the dispatcher silently drops exit-2 block-intent on PreCompact, the entire PreCompact flush blocking mechanism is a silent no-op. VP-086 was authored and added to VP-INDEX as part of ADR-026 v1.1. VP-086 file: `.factory/specs/verification-properties/VP-086.md`. VP-INDEX allocated at v2.08 (O-P4-001: note corrected — VP-INDEX is now at v2.10; no VP changes in pass-4).

---

## Traceability

- **Feature:** issue #173
- **Epic:** E-18 (CAP-032 context-durability)
- **Composes with:** E-17 (CAP-031 single-writer lock/lease) via S-18.04 depends_on S-17.04
- **Composes with:** issue #171 (deferred process-gaps carry-forward via `HANDOFF.md` process_gaps field)
- **Subsystems affected:** SS-04, SS-05, SS-06, SS-07 (and potentially SS-01 per S-18.00 outcome)
- **ADRs composed with:** ADR-019 (async semantics — PreCompact hooks must be `async: false`), ADR-025 (factory lock — flush must renew lock per Decision 11 Mechanism 1 WHEN lock is held; no-op when `factory_lock` absent)
- **ADRs not conflicting:** ADR-012 (legacy-bash-adapter — shell hooks route through it per established pattern), ADR-014 (Tier 2 WASM migration — new WASM crates follow the native migration path)
- **Real substrate fields used:** STATE.md `current_cycle:`, STATE.md `phase:`, STATE.md `current_step:`, STATE.md `factory_lock:` (optional), sprint-state.yaml story status entries, STORY-INDEX.md `depends_on:` arrays
- **VP-INDEX:** v2.07→v2.08 (VP-086 added); v2.08→v2.09 (VP-081..VP-085 v1.2 inputs-path fix; VP-081/082/083/085 current_wave sweep); v2.09→v2.10 (VP-082 v1.2→v1.3 last-precompact-flush-sha→precompact-flush-log; VP-084 v1.2→v1.3 flush-wave prefix→canonical format; VP-085 v1.2→v1.3 last-precompact-flush-sha→precompact-flush-log); v2.10→v2.10 (no VP change in pass-4; VP-INDEX remains at v2.10); v2.10→v2.11 (VP-084 v1.4→v1.5 F-P5-004: harness re-anchored from validate-burst-log.sh → dispatcher WASM invocation; VP-INDEX v2.10→v2.11); v2.11→v2.12 (pass-6 cite-convention migration: VP-084 v1.5→v1.6 cite de-versioned; VP-INDEX v2.11→v2.12); v2.12→v2.13 (pass-9 F-P9-002: VP-082 description amended to include append-failure → SHA-pinned reset (SHA_B^) + concurrent-commit guard; VP-INDEX v2.12→v2.13); v2.13→v2.14 (pass-10 F-P10-003: VP-082 v1.3→v1.4 Postcondition E added; VP-INDEX row updated; total_vps 86 unchanged; VP-INDEX v2.13→v2.14)
- **ARCH-INDEX:** v2.28→v2.29 (ADR-026 v1.0→v1.1); v2.29→v2.30 (ADR-026 v1.1→v1.2 F-P2 revision); v2.30→v2.31 (ADR-026 v1.2→v1.3 F-P3 COMPLETE-SWEEP: canonical §Wave-Identity Derivation + precompact-flush-log append-only + EPIC-COMPLETE cancelled terminal state); v2.31→v2.32 (ADR-026 v1.3→v1.4 F-P4 revision: append-failure→exit-2; WASM/shell division explicit; §F-P4-004 PO wording; §VP Allocations stale-narrative fixed); v2.32→v2.33 (ADR-026 v1.4→v1.5 F-P5 revision: reset-on-append-failure; stale-entry re-grounding; §Decision A anchor; VP-084 v1.4→v1.5 harness re-anchor); v2.33→v2.34 (ADR-026 v1.5→v1.6 GOVERNANCE cite-convention + F-P6-006 SHA-pinned guard + F-P6-004 epic_status conditional field); v2.34→v2.35 (ADR-026 v1.6→v1.7 F-P7-001 payload-only EPIC-COMPLETE discriminator + F-P7-002 provenance trace completion); v2.35→v2.36 (ADR-026 v1.7→v1.8 F-P9-001 §F-P5-002 PC8 + §Decision A HEAD~1 superseded annotations; F-P9-002 VP-082 description + VP-INDEX v2.12→v2.13); v2.36→v2.37 (ADR-026 v1.8→v1.9 F-P10-001 Decision 9 payload-only wave-1 discriminator + §F-P10-001 PO wording + §F-P10-002 push-step contract note; F-P10-003 VP-082 v1.3→v1.4 Postcondition E; VP-INDEX v2.13→v2.14); v2.37→v2.38 (ADR-026 v1.9→v1.10 CV-P14-001 §Decision 2 precompact_flush_sha schema row + Wave-1/Genuine-Log-Absence note reconciled to BC-5.41.001 v1.8 three-case null-SHA rule); v2.38→v2.39 (ADR-026 v1.10→v1.11 F-P15-002 de-versioned stable-anchor cite); v2.39→v2.40 (ADR-026 v1.11→v1.12 F-P17-001 wave-agnostic null rule: log-present-and-valid HARD BLOCK now applies to ANY wave_id including wave_id=1; wave_id discriminator survives only for log-absent case); v2.40→v2.41 (ADR-026 v1.12→v1.13 F-P19-002 §Decision 9 MEDIUM fix: heading retitled 'non-HANDOFF.md writes'; HandoffMissing misattribution to WASM gate corrected — HandoffMissing is BC-5.41.001 PC9 SHELL-SIDE; disjoint boundary paragraph added; §BC Traceability Cite Convention table + §Risk Mitigations F1-R3 row updated); v2.41→v2.42 (ADR-026 v1.13→v1.14 F-P20-002 §Wave-Identity-Derivation cross-note: prior-HANDOFF.md-absence proxy is CALLER-SIDE derivation signal; WASM gate PAYLOAD-ONLY discriminator `payload.wave_id == 1` is the authoritative gate behavior — two substrates are reconcilable not competing; §Decision 9 remains canonical for WASM gate behavior)
