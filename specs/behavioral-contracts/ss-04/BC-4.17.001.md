---
document_type: behavioral-contract
level: L3
version: "1.13"
status: draft
producer: product-owner
timestamp: 2026-08-25T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md
  - crates/factory-lock/src/lib.rs
  - crates/factory-lock-parse/src/lib.rs
  - crates/hook-plugins/precompact-flush/src/lib.rs
  - crates/hook-plugins/verify-factory-lock/src/lib.rs
  - plugins/vsdd-factory/bin/factory-lock-write.sh
  - plugins/vsdd-factory/hooks-registry.toml
input-hash: "4ae09b2"
traces_to: .factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md
origin: brownfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-031"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-08-25 (v1.1) — Pass-2 spec remediation (product-owner), responding to ADR-046 v1.1 (adversarial spec review pass 1) and PRODUCT-OWNER-ROUTED findings F-002/F-004/F-007/F-008/F-009/F-014/F-015. See last_amended for full disposition."
  - "2026-08-26 (v1.2) — Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 PRODUCT-OWNER-ROUTED findings F-002/F-003/F-005/F-008 against ADR-046 (now v1.2). See last_amended for full disposition."
  - "2026-08-26 (v1.3) — Pass-3 spec remediation (product-owner), responding to adversarial spec review pass 3 findings F-001/F-003/F-004 against ADR-046 (now v1.4). See last_amended for full disposition."
  - "2026-08-26 (v1.4) — Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 findings F-P4-001/F-P4-002/O-P4-001/O-P4-002 against ADR-046 (now v1.5). See last_amended for full disposition."
  - "2026-08-26 (v1.5) — Pass-5 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.6's F-P5-001 (HIGH) correction — Precondition 4's `STATE_MD_MAX_BYTES` sourcing corrected to `factory_lock_parse::STATE_MD_MAX_BYTES`. See last_amended for full disposition."
  - "2026-08-26 (v1.6) — Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED + 2 LOW total across the ADR-046 BC cluster; this BC's cited findings: F-P6-001, F-P6-003; plus in-scope sweep fixes for stray POLICY 19 version pins). See last_amended for full disposition."
  - "2026-08-26 (v1.7) — Pass-10 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.8's F-P10-001 (HIGH) corrected write-composition model — the two arms now compose into a single `host::write_file` (PC1's transform applied first, fed into PC2's `renew_lock_if_holder`) instead of two independent per-arm writes. See last_amended for full disposition."
  - "2026-08-26 (v1.8) — Pass-11 prose-consistency remediation (product-owner), responding to adversarial spec review pass 11 finding O-P11-001 (LOW) — Invariant 9's headline sentence self-contradicted its own four-row selection table's zero-write row 4; corrected to 'AT MOST ONE host::write_file call ... never two racing writes; exactly one when at least one arm produces a change, zero when neither does.' See last_amended for full disposition."
  - "2026-08-26 (v1.9) — Pass-15 reader-vs-writer migration-fitness remediation (product-owner), responding to adversarial spec review pass 15 finding F-P15-001 (HIGH) — Invariant 7's fence-not-located handling incorrectly directed PC1/PC2 to apply to the undelimited full slice (migrated verbatim from a read-only guard's fail-open behavior, unfit for this writer hook); corrected to PC3a's fully-structural suppress-both path. VP-TBD-8 carried the identical defect, also corrected. Comprehensive writer-fitness audit of all migrated elements performed; O-P15-001 (LOW) Invariant 9 headline precision also fixed. See last_amended for full disposition."
  - "2026-08-26 (v1.10) — Pass-16 fence-not-located Precondition/Invariant parity fix + exhaustive writer-fitness precision sweep (product-owner), responding to adversarial spec review pass 16 observation O-P16-001 (LOW) — Precondition 4's mandatory `extract_frontmatter`-use directive was stated UNCONDITIONALLY while Invariant 7 (v1.9) scopes the identical mandate to the fence-located case only; corrected to carry the SAME qualifier and route the no-fence-located case to PC3a's fully-structural suppress-both path per Invariant 7. Exhaustive precision sweep of every remaining writer-fitness-frontmatter-directive locus in this BC performed; all other loci (PC1, PC3a, PC3a's anchor-absence exception, PC4, Invariant 5, Invariant 8, EC-014, EC-015, VP-TBD-7, VP-TBD-8, VP-TBD-9, Architecture Anchors) confirmed clean — no unqualified full-slice-operate directive remains anywhere in this BC. See last_amended for full disposition."
  - "2026-08-26 (v1.11) — Pass-21 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.10's F-P21-001 (MED) corrected event-sourcing — `SkipReason::IdentityResolutionFailed` corrected from a bare-`String` tuple variant to the struct variant `{ reason, holder, locked_at, expires_at }`, sourced from the `lock_state: FactoryLock` already in scope at the holder-present step. PC2 case 4's return value and PC3b's `factory.lock.renewal_indeterminate` event emission corrected to destructure the returned variant directly (no re-parse, no unbound `<lock.*>` reference); EC-004/EC-012 updated identically. See last_amended for full disposition."
  - "2026-08-26 (v1.12) — Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding F-P25-002 (MED). Traceability §Stories row and §Story Anchor's [pending] placeholders resolved to the confirmed implementing story S-17.05 (stamp-state-timestamp-hook, E-17 Wave 5). See last_amended for full disposition."
  - "2026-08-26 (v1.13) — Pass-28 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 28 finding F-P28-001a (HIGH, POLICY 18). inputs: frontmatter gap closed — crates/factory-lock-parse/src/lib.rs added, backing this BC's own load-bearing STATE_MD_MAX_BYTES/extract_frontmatter claims (Precondition 4, VP-TBD-7/8, Architecture Anchors); same path form BC-7.07.001's array already uses. See last_amended for full disposition."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.17.001
section: "4.17"
last_amended: "2026-08-26 (v1.13) — Pass-28 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 28 finding **F-P28-001a (HIGH, POLICY 18)**. Disposition: **F-P28-001a (HIGH)** — `inputs:` frontmatter omitted `crates/factory-lock-parse/src/lib.rs` despite this BC's own load-bearing claims against it: Precondition 4 cites `factory_lock_parse::STATE_MD_MAX_BYTES` as \"the single canonical declaration\" and mandates `factory_lock_parse::extract_frontmatter` use; VP-TBD-7 and VP-TBD-8 assert the same crate's constant/function; §Architecture Anchors cites the crate directly — none of these claims were backed by an `inputs:` entry for the crate itself. This is a mirror-image gap of BC-7.07.001's own v1.29 F-P27-003 fix, which added this exact file to ITS OWN `inputs:` array while claiming (falsely) to be \"mirroring sibling BC-4.17.001's input set\" — this BC's `inputs:` array did NOT yet contain the file at that time; that false cross-reference is corrected in BC-7.07.001's own v1.30, same burst (F-P28-001b), and is NOT restated here as if it were the origin of this fix. Corrected: `crates/factory-lock-parse/src/lib.rs` added to this BC's `inputs:` array, in the same path form BC-7.07.001's array already uses (independently justified against this BC's own load-bearing claims, not derivative of BC-7.07.001's). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); this is a pure `inputs:` array addition. input-hash recompute flagged to state-manager (not run by this amendment). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.12) — Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding **F-P25-002 (MED)**. Disposition: **F-P25-002 (MED)** — Traceability §Stories row and §Story Anchor's `[pending]` placeholders for the ADR-046 identity-gate amendment implementation resolved to the confirmed implementing story: S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; `.factory/stories/S-17.05-stamp-state-timestamp-hook.md`), verified present in STORY-INDEX.md. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); both are in-place corrections to existing Traceability text (§Stories row, §Story Anchor). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.11) — Pass-21 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.10's F-P21-001 (MED) corrected event-sourcing for `SkipReason::IdentityResolutionFailed` — corrected from a bare-`String` tuple variant to the struct variant `{ reason: String, holder: String, locked_at: String, expires_at: String }`, sourced from the `lock_state: FactoryLock` `renew_lock_if_holder` already holds in scope at the holder-present step (no re-parse). Disposition: **F-P21-001 (MED)** — PC2 case 4's return value corrected from `Ok((RenewOutcome::NoOp, Some(SkipReason::IdentityResolutionFailed(reason))))` to `Ok((RenewOutcome::NoOp, Some(SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at })))`, with the three non-`reason` fields sourced from `lock_state.holder`/`.locked_at`/`.expires_at` already parsed by the holder-present check — no additional parse or I/O. PC3b's `factory.lock.renewal_indeterminate` event emission corrected from an unbound `<lock.*>` reference to a direct destructure of the matched `SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at }` variant — `host::emit_event(\"factory.lock.renewal_indeterminate\", &[(\"plugin\", \"stamp-state-timestamp\"), (\"holder\", holder), (\"locked_at\", locked_at), (\"expires_at\", expires_at), (\"resolution_error\", reason)])`, sourced from the single `renew_lock_if_holder` return value, never re-parsed from `.factory/STATE.md`'s `factory_lock:` block. EC-004 and EC-012 updated with the identical destructuring clarification. BC-7.07.001's companion amendment (same burst, sequential-after-architect) mirrors the identical struct variant + event-sourcing text for `precompact-flush`'s call site, per ADR-046 Companion Amendment 3 — both BCs now state the IDENTICAL struct variant and event-sourcing mechanism for the two call sites. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); this is an in-place correction to PC2 case 4's return value and PC3b's event-emission text. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.10) — Pass-16 fence-not-located Precondition/Invariant parity fix + exhaustive writer-fitness precision sweep (product-owner), responding to adversarial spec review pass 16 observation O-P16-001 (LOW). Disposition: **O-P16-001 (LOW)** — Precondition 4's `extract_frontmatter`-use mandate ('...MUST call `factory_lock_parse::extract_frontmatter(bytes)` ... and operate exclusively on the returned frontmatter slice') was stated UNCONDITIONALLY, while Invariant 7 (corrected at v1.9, F-P15-001) now scopes the identical mandate to apply only when a valid `---` fence WAS located, routing the fence-not-located degenerate case to PC3a's fully-structural suppress-both path instead. A literal reading of Precondition 4 alone — in the no-fence case, where `extract_frontmatter` fail-opens by returning the FULL undelimited input — would still direct the hook to operate on that full slice: exactly the reader-vs-writer STATE.md-body-corruption hazard v1.9 corrected Invariant 7 to forbid. Corrected: Precondition 4 now carries the SAME 'when, and only when, a valid opening/closing `---` fence WAS located' qualifier Invariant 7 carries, and explicitly routes the no-fence-located case to PC3a's fully-structural suppress-both path (no write) per Invariant 7 — Precondition 4 and Invariant 7 now give the IDENTICAL literal directive for the same degenerate input. **Exhaustive precision sweep (this task's central obligation, not merely the point fix):** every remaining directive/mention of `extract_frontmatter`, 'frontmatter slice'/'returned slice', 'operate on/exclusively/only', 'first-match' line-scan, 'undelimited'/'full returned'/'full slice', and fence-location language across this BC's Preconditions, Postconditions, Invariants, Edge Cases, Canonical Test Vectors, VP rows, Description, and SDK Grounding Evidence was re-audited for the identical unqualified-full-slice-operate defect class. Per-locus findings: Precondition 4 — UNFIT, fixed above (this pass). Invariant 7 — CLEAN (already carries the fence-located qualifier since v1.9; its 'MUST NOT apply ... to the undelimited full slice' clause is a prohibition, not an operate-directive, and is internally consistent). PC1's rewrite-mechanism paragraph (first-match `timestamp:` line scan 'within the frontmatter region') — CLEAN: PC1 only executes once Preconditions have passed, and in the no-fence case the hook never reaches PC1 at all (PC3a fully-structural suppression, per the now-fixed Precondition 4/Invariant 7 pairing), so PC1's own text never independently claims to operate on an undelimited full slice. PC3a's fully-structural definition (including 'malformed or missing frontmatter delimiters') — CLEAN: this clause IS the fence-not-located path stated in its own terms; no unqualified operate-directive present. PC3a's scoped `timestamp:`-anchor-absence exception — CLEAN: its own stated premise is that 'the frontmatter region itself IS validly parseable' — i.e. a fence WAS located; genuinely unaffected by the no-fence case, and a different sub-case from it (see Invariant 7's own conflation warning). PC4 ('rewrite touches at most two single lines within the YAML frontmatter region') — CLEAN: describes the semantic scope of a write that only happens when a write happens at all; in the no-fence case no write occurs, so this clause is vacuously satisfied, not contradicted. Invariant 5 ('the hook MUST operate exclusively within the region bounded by the opening and closing `---` delimiters') — CLEAN: a must-operate-only-within-X constraint is satisfied by not operating at all, which is exactly what the no-fence PC3a suppress-both path does; no full-slice-operate directive is implied. Invariant 8 (soft-warn) and EC-015 (`OutputTooLarge`) — CLEAN, reconfirmed from the v1.9 audit (emit-only / already-fully-structural, no reader-vs-writer purchase). EC-014 (duplicate-`timestamp:`-key handling) — CLEAN: its own stated premise is a duplicate KEY within an already fence-located, well-formed frontmatter region; genuinely unaffected by the no-fence case. VP-TBD-7 and VP-TBD-9 — CLEAN, reconfirmed (constant-equality and read-size-threshold-boundary assertions only, no slice-operate claim). VP-TBD-8 — CLEAN: already carries the identical corrected fence-located qualifier since v1.9 (F-P15-001), verified consistent with Invariant 7's post-this-pass text. Architecture Anchors' `extract_frontmatter` citations — CLEAN: function-reference citations only, no operate-on-slice directive. **Conclusion:** after this pass, no clause in BC-4.17.001 gives an unqualified full-slice-operate directive that contradicts the PC3a/Invariant-7 no-fence suppress-both semantics; Precondition 4 was the sole remaining sibling of the F-P15-001/O-P16-001 defect class. ADR-046 confirmed CORRECT and UNCHANGED — §Decision 1's fail-open scoping already states the fence-not-located/fully-structural mapping accurately; not touched by this pass, per strict isolation. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); this is an in-place qualifier addition to Precondition 4's existing sourcing sentence, mirroring Invariant 7's already-corrected text. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.9) — Pass-15 reader-vs-writer migration-fitness remediation (product-owner), responding to adversarial spec review pass 15 finding F-P15-001 (HIGH) and observation O-P15-001 (LOW). Disposition: **F-P15-001 (HIGH)** — this BC's Invariant 7 (the guard-read contract MIGRATED from BC-5.40.001's now-dormant, read-only `verify-state-timestamp-refresh` guard per ADR-046 §Decision 5/F-P4-002) was migrated VERBATIM in its fence-not-located sub-case, carrying over reader semantics unfit for a WRITER hook: its final sentence directed that 'when `extract_frontmatter` returns the full bytes (delimiter not found — the function's own fail-open behavior), the hook applies PC1/PC2 to the full returned slice without error (consistent with PC3a's `timestamp:`-anchor-absence handling, Invariant 4)' — for this hook, applying PC1's first-match `timestamp:` line scan to an undelimited full-file slice (frontmatter AND body concatenated, since no fence could be located to separate them) would let the scan locate and rewrite the first `timestamp:`-prefixed line anywhere in the file, including STATE.md BODY content (decision-log rows, burst-log narrative) — exactly the corruption hazard PC3a's fully-structural fail-open and Invariant 5 (frontmatter-only) exist to prevent. The cited cross-reference was also the wrong sub-case: 'PC3a's `timestamp:`-anchor-absence handling' is the narrower, PC1-only-suppression path that applies only when a valid fence IS located but no `timestamp:` key exists within it — not the fully-structural, both-arms-suppressed path a fence-not-located condition actually is (ADR-046 §Decision 1 fail-open scoping, confirmed correct and unchanged: 'Fully-structural failures — suppress BOTH arms: ... unparseable/malformed frontmatter (no opening/closing `---` fence at all, or the fence cannot be located).'). Corrected Invariant 7: fence-not-located now explicitly takes PC3a's FULLY-STRUCTURAL suppress-both path (BOTH PC1 and PC2 suppressed, zero bytes written, see EC-005/EC-015) rather than operating on the undelimited slice; the 'operate only on the returned frontmatter slice' mandate is now explicitly scoped to apply only when a valid fence WAS located; the cross-reference is corrected to cite PC3a's fully-structural path, not the anchor-absence sub-case. **Comprehensive writer-fitness audit (the task's central obligation, not merely the point fix):** every element migrated from the read-only guard into this BC (Precondition 4's `STATE_MD_MAX_BYTES` cap + mandatory `extract_frontmatter` use, Invariant 7, Invariant 8's `state_md_approaching_cap` soft-warn, EC-015's `OutputTooLarge` fail-open, and VP-TBD-7/8/9) was individually re-audited for the same reader-vs-writer mismatch class: Precondition 4 — CLEAN (its own `OutputTooLarge` clause already correctly directs PC3a's fully-structural suppress-both path; it does not itself address the no-fence-found fallback, which is Invariant 7's concern, now fixed). Invariant 7 — UNFIT, fixed above. Invariant 8 — CLEAN (purely observational: emits a diagnostic event on read size, never gates or alters a write; a reader-vs-writer distinction has no purchase on an emit-only clause). EC-015 — CLEAN (already directs PC3a's fully-structural suppress-both path on `OutputTooLarge`, with 'the agent's original write is left untouched' explicit). VP-TBD-7 — CLEAN (asserts only a constant-value equality, no read/write direction). VP-TBD-8 — UNFIT (asserted the identical 'no-delimiter fallback operates on the full returned content without error' reader-semantics claim as Invariant 7's pre-fix text); corrected in the same pass to assert the fully-structural fail-open behavior instead. VP-TBD-9 — CLEAN (asserts the soft-warn/`OutputTooLarge` boundary table only, no operate-on-full-slice claim). **STEP 4 confirmation — no other LIVE writer BC inherited the unfit clause:** grepped the full behavioral-contracts tree for `extract_frontmatter`; the only hits besides this BC are (a) BC-5.40.001's own dormant Invariant 7 (the migration SOURCE — a deregistered, read-only `PreToolUse` guard; explicitly confirmed benign and left untouched, POLICY 1 append-only — a reader's 'apply parse logic to the full returned slice without error' is safe because a guard only decides Block/Continue, never writes derived content back to disk); (b) BC-4.13.001 (`verify-factory-lock`, a `PreToolUse` blocking guard — reader, not a writer); (c) BC-5.39.009 and BC-5.39.010 (both `validate-*` WASM hooks — block/advisory validators, readers, not writers). `precompact-flush` (BC-7.07.001) — the one other LIVE hook that calls the shared `renew_lock_if_holder` function this BC also calls — does not call `extract_frontmatter` at all: it is single-concern (renewal only, no `timestamp:` line-scan arm) and feeds `renew_lock_if_holder` its own raw `host::read_file` result directly (confirmed by inspection of its Postcondition/Invariant text — no `extract_frontmatter`/`STATE_MD_MAX_BYTES` reference anywhere in that file), so it never inherited this migrated clause in the first place. No other live writer BC is affected. **O-P15-001 (LOW)** — Invariant 9's headline ('the hook performs exactly ONE `host::write_file` call ... exactly one `host::write_file` call when at least one arm produces a change (rows 1–3)') mis-stated the write-trigger predicate: row 2 of the invariant's own four-row table (PC1 fired, PC2 = `NoOp`/`Err(Malformed)`) still performs a write (`content_after_pc1`) even though PC2 produced no change — the actual trigger is PC1 **firing** (`timestamp:` anchor present), not PC1 **producing a change** (the two coincide for PC1 specifically, since it is a live clock that always changes the field when it fires, but 'produces a change' is the wrong general predicate for the table's own row-selection logic). Reworded to 'when PC1 fires (anchor present) OR PC2 renews' — the four-row table itself, the invariant's title, the defect narrative, and the advisory-independence note are all UNCHANGED; this is a headline-wording-only correction, no behavioral change. ADR-046 confirmed CORRECT and UNCHANGED by this pass — §Decision 1's fail-open scoping already states the fence-not-located/fully-structural mapping accurately; the drift was confined to this BC's own Invariant 7/VP-TBD-8 migration text, per the task's explicit framing that only this BC drifted. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); all corrections are in-place. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.8) — Pass-11 prose-consistency remediation (product-owner), responding to adversarial spec review pass 11 finding O-P11-001 (LOW; adversary pass 11 CLEAN on substance — this is the sole remaining prose-consistency observation). Disposition: **O-P11-001 (LOW)** — Invariant 9's headline sentence (\"the hook performs exactly ONE `host::write_file` call per qualifying invocation, never two, and never zero-or-two depending on which arm(s) fired\") self-contradicted the same Invariant's own authoritative four-row selection table, whose row 4 (neither arm produces a change) is a zero-write outcome. Corrected the headline to: \"the hook performs AT MOST ONE `host::write_file` call per qualifying invocation — never two racing writes: exactly one `host::write_file` call when at least one arm produces a change (rows 1–3), and zero writes when neither arm does (row 4)\" — the four-row table itself, the invariant's title, the defect narrative, and the advisory-independence note are all UNCHANGED; this is a headline-wording-only correction, no behavioral change. Confirmed ADR-046 §Decision 1 (line ~98) already phrases the four-row selection (including the \"no write\" row) correctly, so no ADR-046 change was required. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.7) — Pass-10 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.8's F-P10-001 (HIGH, ARCHITECT-routed remediation of the same pass; product-owner mirrors ADR-046 v1.8's corrected write-composition text into BC-4.17.001 in a sequential follow-up burst per D-386 sequencing — strict isolation maintained: this revision touches BC-4.17.001 only, no ADR-046/BC-5.40.001/BC-7.07.001/registry/hook-source/STATE.md/ARCH-INDEX/BC-INDEX content touched). Disposition: **F-P10-001 (HIGH)** — ADR-046 v1.8 corrected §Decision 1's write-composition model from a deterministic lost-update (the two arms specified as \"independently gated ... each with its own `host::write_file` call ... not a single joint [write]\" while sharing ONE `host::read_file`, so a `Renewed` outcome's full-file `new_content` — built from the stale pre-PC1 read — silently reverted PC1's `timestamp:` advance whenever both arms fired on a held lock) to a single composed write: PC1's transform is applied FIRST, in memory, to the shared read, producing `content_after_pc1`; `renew_lock_if_holder` is fed `content_after_pc1` — never the raw shared read; exactly ONE `host::write_file` is performed, selected by the four-row table (both-fire → PC2's composed `new_content`; PC1-only → `content_after_pc1` alone; PC2-only [missing-anchor case] → PC2's `new_content` computed against the raw read; neither → no write). Mirrored into this BC per the six items below: (1) **Description** tightened from \"single re-read-and-rewrite pass\" to explicitly state ONE `host::write_file` per invocation, never two, with the composed-transform framing ADR-046's suggested phrasing gives. (2) **Postcondition 1 (PC1)** gains a new paragraph (\"PC1's output is PC2's input, not a parallel branch\") stating `content_after_pc1` — not the raw shared read — is PC2's sole input, cross-referencing new Invariant 9. (3) **Postcondition 2 (PC2)** preamble gains the required-input sentence naming `content_after_pc1` verbatim (mirroring ADR-046's mandated phrasing) plus the call-site-asymmetry contrast with `precompact-flush` (BC-7.07.001, a single-concern site correctly unaffected — feeds `renew_lock_if_holder` its own raw read directly, since there is no prior transform to preserve there). (4) **New Invariant 9** (\"Single composed write, never two independent per-arm writes\") states the four-row single-write selection table verbatim, the independent-advisory-firing rule, and the specific defect this invariant closes. (5) **Postcondition 4 (PC4)** reconciled: \"targeted... never a full-file rewrite\" is now explicitly scoped as a SEMANTIC-SCOPE guarantee (at most the `timestamp:`/`expires_at` lines' content changes) rather than a write-mechanism claim — the bounded change is persisted via one whole-file `host::write_file`, the same mechanism `factory-lock-write.sh`'s `_update_expires_at`/`rewrite_expires_at` already use (no byte-range/patch API exists in this codebase) — removing the prior apparent contradiction with `renew_lock_if_holder`'s full-content `Renewed` return shape. (6) **New MANDATORY Canonical Test Vector** (\"Anti-clobber regression\") added: both arms fire (lock held by the caller's own identity, unexpired) → assert the post-hook STATE.md carries BOTH the new `timestamp:` value AND the new `expires_at` value simultaneously in the same resulting file — the exact regression a naive two-write implementation would pass per-arm-in-isolation while still failing. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); Invariant 9 and the new test-vector row are pure additions. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.6) — Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED + 2 LOW total across the ADR-046 BC cluster, all sibling-sweep-straggler class — a prior fix landing in one location while a sibling location in the same file was missed). Disposition: **F-P6-001 (MED)** — VP-TBD-7 still read \"`STATE_MD_MAX_BYTES` constant used by this hook equals 262144 and is byte/value-identical to the constant `verify-state-timestamp-refresh` used before its ADR-046 Decision 5 deregistration (no drift; reused, not re-declared)\" — the exact pre-F-P5-001 stale sourcing that Precondition 4 was corrected away from at v1.5, but which this VP row (added in the same v1.4 pass as the original Precondition 4 text) was never swept alongside it. Corrected to state that the constant is `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (relocated to the `factory-lock-parse` crate per ADR-046 §Decision 5 / F-P5-001) — not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate, mirroring Precondition 4's own corrected sourcing verbatim. **F-P6-003 (LOW)** — the Architecture Anchors bullet `crates/factory-lock (or crates/factory-lock-parse)` hedged the home of `TTL_SECONDS`/`renew_lock_if_holder`/`IdentityResolution`/`SkipReason`, all of which are definitively `crates/factory-lock` per ADR-046's File-Change Plan (the only genuine `factory-lock-parse` residents are `STATE_MD_MAX_BYTES` and `extract_frontmatter`, cited in a separate bullet). The imprecise \"(or `crates/factory-lock-parse`)\" hedge is removed; the bullet now states `crates/factory-lock` unhedged. **Comprehensive sibling-sweep (in-scope, production-grade default):** a full grep sweep of this BC for every occurrence of the canonical values ADR-046 governs (expiry boundary, `STATE_MD_MAX_BYTES` sourcing, malformed-arm disposition, 5-case numbering, shared-fn homes, TTL cast, event fields, POLICY 19 pins, per-arm fail-open) found and fixed 1 additional stray `ADR-046 vN.N` version-pin straggler in Precondition 4's own sourcing annotation (\"corrected 2026-08-26, ADR-046 §Decision 5 v1.6 / F-P5-001\" and the matching inline cite inside the quoted sourcing text), stripped to the stable `ADR-046 §Decision N` anchor form per POLICY 19 anti-volatile-pin (mirroring O-P4-002's established pattern — the \"mirrored verbatim\" framing is corrected to \"mirrored ... version pin stripped per POLICY 19\" to accurately describe the now-slightly-non-byte-identical quote); all other swept categories (expiry boundary, malformed-arm disposition, 5-case numbering, shared-fn homes, TTL cast, event fields, per-arm fail-open) clean — zero additional stragglers. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); all corrections are in-place. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.5) — Pass-5 architecture-routed remediation mirror (product-owner), responding to adversarial spec review pass 5 (ADR-046 flipped v1.5→v1.6 via architect-routed remediation of the same pass; product-owner mirrors ADR-046 v1.6's corrected `STATE_MD_MAX_BYTES` sourcing text here in a sequential follow-up burst per D-386 sequencing — strict isolation maintained across BC-4.17.001/BC-5.40.001/BC-7.07.001, no ADR/registry/hook-source/STATE.md/ARCH-INDEX content touched). Disposition: **F-P5-001 (HIGH)** — Precondition 4's `STATE_MD_MAX_BYTES` sourcing statement (\"the SAME `STATE_MD_MAX_BYTES = 262144` constant `verify-state-timestamp-refresh` used before its ADR-046 Decision 5 deregistration\") pointed, unnamed, at that constant's only pre-v1.6 declaration site — a crate ADR-046 §Decision 5 deregisters and anticipates eventually deleting, forcing an implied Cargo dependency on a to-be-deleted crate. Corrected by mirroring ADR-046 §Decision 5 v1.6 / F-P5-001's exact sourcing text verbatim: Precondition 4 now references `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (`pub const STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to the `factory-lock-parse` crate), not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate (which no longer declares it as of this correction). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); this is an in-place correction of Precondition 4's existing sourcing sentence, not a new addition. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.4) — Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 (ADR-046 flipped v1.4→v1.5 via architect-routed remediation of the same pass; product-owner applies ADR-046 v1.5's expanded guidance here in a sequential follow-up burst per D-386 sequencing — strict isolation maintained across BC-4.17.001/BC-5.40.001/BC-7.07.001, no ADR/registry/hook-source/STATE.md/ARCH-INDEX content touched). Disposition: **F-P4-001 (MED, real bug, routed via BC-5.40.001)** — out of scope for this BC (BC-5.40.001 PC3's boundary condition; this BC does not restate `verify-factory-lock`'s `now > expires_at` comparison anywhere). **F-P4-002 (MED)** — this BC gains the MIGRATED guard-read contract ADR-046 v1.5 §Decision 5's per-element reconciliation table directs: new Precondition 4 (`STATE_MD_MAX_BYTES = 262144` cap + mandatory `factory_lock_parse::extract_frontmatter` use, reusing the SAME constant BC-5.40.001's now-historical Precondition 6 declared — not a second declaration), new Invariant 7 (`extract_frontmatter` exclusive, migrated from BC-5.40.001's now-historical Invariant 7), new Invariant 8 (`state_md_approaching_cap` soft-warn, migrated from BC-5.40.001's now-historical Invariant 8), new EC-015 (`OutputTooLarge` fail-open, migrated from BC-5.40.001's now-historical EC-010), and three new VP-TBD-7/8/9 rows (equivalent guarantees against this hook's own shared read call, migrated equivalents of BC-5.40.001's now-historical S-19.08 T-001/T-004/T-005/T-007). **O-P4-001 (LOW)** — PC2's case numbering harmonized to ADR-046's canonical five-case numbering (Malformed=1, AlreadyExpired=2, NotHolder=3, IdentityResolutionFailed=4, Success=5; absent/empty-holder is the ADR's unnumbered '0th' case) — matching BC-7.07.001's Invariant 3b table verbatim; all body cross-references to the old ad hoc step numbering (Invariants, Edge Cases, Canonical Test Vectors, §SDK Grounding Evidence) swept to the new numbering (TD-VSDD-060 sibling-site sweep within this file). **O-P4-002 (process-gap, fixed in-scope per production-grade default)** — inline `ADR-046 vN.N §Decision N` version pins in BODY prose (PC2 preamble/return-value-table cites, F-009 fold-in cite) stripped to the stable `ADR-046 §Decision N` anchor form; historical `last_amended`/Changelog narrative left untouched (dated record, not a forward-looking pin, per ADR-046's own O-P4-002 rationale). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); new Precondition 4/Invariant 7/Invariant 8/EC-015/VP-TBD-7/8/9 are pure additions. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.3) — Pass-3 spec remediation (product-owner), responding to adversarial spec review pass 3 (ADR-046 flipped v1.3→v1.4 via architect-routed remediation of the same pass; product-owner fixed the BC-routed findings F-001/F-003/F-004 from the same pass here, in parallel — strict isolation maintained, this revision touches BC-4.17.001 only). Disposition: **F-001 (HIGH)** — PC2's malformed-`expires_at` arm (step 3) previously described the outcome as `NoOp` with an advisory `log_warn`, which contradicted ADR-046 v1.4 §Decision 1(b)'s corrected `renew_lock_if_holder` signature — `Result<(RenewOutcome, Option<SkipReason>), LockError>` — under which the malformed case is a distinct `Err(LockError::Malformed(msg))` return, not a `NoOp`/`SkipReason` value (`SkipReason` has no `Malformed` variant to hold it). PC2 step 3, EC-007, and the corresponding Canonical Test Vectors row are corrected to state `Err(LockError::Malformed(msg))`, downgraded by the hook (caller) to an advisory `log_warn` with no write — matching ADR-046 v1.4's canonical five-case return-value table exactly. PC2's preamble is also corrected to state the full canonical signature — `resolve_identity: FnOnce() -> IdentityResolution` is a **lazy** closure the hook supplies but does not itself invoke; `renew_lock_if_holder` invokes it at most once, only when the decision tree reaches the identity-comparison step (case 5) — making the pre-existing 'no `exec_subprocess` call for malformed/already-expired' claim literally true rather than aspirational. This BC and BC-7.07.001 (companion amendment, same burst) now state the IDENTICAL return contract, closing the cross-BC divergence pass-3 found. **F-003 (MED)** — the Description's 'MUST NOT independently re-derive `renew_lock_if_holder`/`trim_git_email`' mandate is extended to include the shared `factory_lock::classify_identity_resolution` `(exit_code, stdout) -> IdentityResolution` classifier (ADR-046 §Decision 2/F-006); PC2 step 5's `Failed(reason)` case now explicitly cites this shared classifier by name rather than describing the classification generically, and a new PC2 'Shared-classifier mandate' paragraph makes the no-independent-re-derivation requirement explicit, mirroring BC-7.07.001's identical companion amendment. **F-004 (MED)** — out of scope for this BC (BC-7.07.001 §Architecture Anchors only; this BC does not reference `precompact-flush`'s `LOCK_RENEWAL_TTL_SECS` constant). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.2) — Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 (ADR-046 flipped v1.1→v1.2) and to the PRODUCT-OWNER-ROUTED subset of that pass's findings. Disposition: **F-002 (HIGH, POLICY 19 adr_version_cite_volatile_pin_prohibition)** — the Traceability § ADR Reference row carried a load-bearing `ADR-046 v1.1` version pin, which is both a POLICY 19 violation and already stale (ADR-046 is now v1.2); the pin is stripped to the stable `ADR-046 §Decision 1/§Decision 2/§Decision 4` anchor form — no version token remains in that row. **F-003 (HIGH)** — the v1.1 narrative's 'flagged for architect: ADR-046's Decision 1(b)/File-Change Plan should be updated ... not done here' note (last_amended and Changelog) is now RESOLVED: ADR-046 v1.2 §Decision 1(b) already enumerates `AlreadyExpired` as the third `SkipReason` variant, consistent with this BC's v1.1 F-009 disposition — no architect action remains outstanding on this item; the v1.1 historical narrative below and the v1.1 Changelog row are annotated with this resolution rather than left asserting a stale open flag. **F-005 (MED)** — PC3a's structural fail-open path incorrectly grouped 'absence of a `timestamp:` anchor line' together with fully-structural failures (`host::read_file` error, malformed/missing frontmatter delimiters, non-UTF-8) as suppressing BOTH PC1 and PC2; but PC2's five-step gate operates exclusively on the `factory_lock` block and has zero dependency on `timestamp:`'s presence. PC3a is corrected: `timestamp:`-anchor absence now suppresses PC1 ONLY — PC2 MUST still evaluate its own gate and renew `expires_at` if satisfied, even when there is no `timestamp:` key to rewrite. Invariant 4 updated to reflect the corrected per-arm suppression scope. New EC-013 added (this BC's two-independently-fail-open-arms design intent, per Invariant 4, is preserved — not relaxed — by this correction). **F-008 (LOW)** — PC1's `timestamp:` rewrite mechanism is now made explicit as a first-match line scan (mirroring PC4's targeted-line-replacement mechanism, never a full YAML parse), and its behavior when a SECOND `timestamp:` line is present in frontmatter (a pre-existing YAML-duplicate-key corruption this hook does not create) is specified: the hook rewrites ONLY the first-matched line and emits an advisory `DuplicateTimestampKey` `log_warn` — never a block, no exit-code change. The retired `verify-state-timestamp-refresh` guard could not have caught a stale second `timestamp:` line either, so this is an accepted, advisory-only residual, not a regression this hook introduces. New EC-014 added. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-25 (v1.1) — Pass-2 spec remediation (product-owner), responding to ADR-046 v1.1 (adversarial spec review pass 1: 3 BLOCKER, 5 HIGH, 7 MED, 4 LOW; ADR flipped proposed→accepted) and to the PRODUCT-OWNER-ROUTED subset of those findings. Disposition: **F-002 (BLOCKER)** — PC3's 'decline to write anything' fail-open language contradicted EC-004/TV-5 (which correctly said `timestamp:` still advances on identity-resolution failure). PC3 is SPLIT into PC3a (structural fail-open — unreadable file, missing frontmatter delimiters, non-UTF-8, missing `timestamp:` anchor — suppresses BOTH arms because there is no valid content to rewrite at all) and PC3b (identity-resolution fail-open — `exec_subprocess` failure/timeout/empty-output — suppresses ONLY the PC2 renewal arm; PC1's `timestamp:` re-stamp is unaffected and MUST still fire). **F-004 (HIGH)** — PC2's byte-equality comparison now explicitly names the canonical `trim_git_email` trim (promoted per ADR-046 Decision 2) as applied to BOTH sides of the comparison (the hook's freshly-resolved `exec_subprocess` stdout AND, defensively, the frontmatter `holder` value), declared equivalent to `factory-lock-write.sh`'s bash-side `tr -d '\n'` for the git-email domain (single-line value, at most one trailing newline) — the happy path (Resolved+Match) now actually fires under a well-defined comparison. **F-007 (HIGH)** — H1 and the VP table's 'idempotent'/'strictly greater' language is SCOPED: idempotency is a property of the `expires_at` arm ONLY (byte-identical-suppression under a `>=` comparison — same-wall-clock-second writes are a no-op, not a failure); the `timestamp:` arm is explicitly NOT idempotent (it is a live clock, unconditionally re-stamped) and its VP predicate is corrected from 'strictly greater' to `>=` for the same second-precision-collision reason. **F-008 (HIGH)** — EC-007 rewritten: a malformed/unparseable `expires_at` is NEVER repaired by this hook (no fresh `now + TTL_SECONDS` computed to 'fix' it) — `verify-factory-lock` treats a malformed block as UNLOCKED (admits any caller), so a repair would re-materialize a lock under a session the guard just treated as free; the hook's only action on malformed `expires_at` is an advisory `log_warn`, no write, no identity resolution attempted. **F-009 (MED)** — PC2's gate is extended with an explicit expiry precheck: an already-expired self-held lock (`holder` non-empty, `expires_at` parses and `now >= expires_at`) is NOT renewed — `SkipReason::AlreadyExpired` is returned WITHOUT attempting identity resolution, aligning with `verify-factory-lock` PC2 (`LockExpired`), which treats an expired lock as free for any caller. This is a product-owner disposition of an ADR-046 gap the ADR's `renew_lock_if_holder` decision-tree text does not itself enumerate (ADR-046 Decision 1(b) specifies only Resolved+Match / Resolved+Mismatch / Failed, wrapping `renew_lock_with_now`'s existing presence→parse→compute→byte-identical sequence, which has no expiry check at all — confirmed by inspection of `crates/factory-lock/src/lib.rs`'s `renew_lock_with_now`). This BC extends the `SkipReason` enum with a third variant, `AlreadyExpired`, per the task's explicit recommendation (do not resurrect) (RESOLVED at v1.2 — ADR-046 v1.2 §Decision 1(b) now enumerates this third `SkipReason` variant; see v1.2 disposition above; no architect action remains outstanding on this item). **F-014 (MED, POLICY 5)** — new `§SDK Grounding Evidence` section added with literal-shell grep evidence (stable-anchor form, no line numbers, per POLICY 5 v1.3.1) for every external-artifact claim this BC makes: the pre-fix TTL-literal drift across three independently-typed sites, the `regression-gate` PostToolUse `read_file`/`write_file` capability precedent, `factory-lock-write.sh`'s `TTL_SECONDS`/`HOLDER` trim assignment, and `verify-factory-lock`'s `trim_git_email`/`exec_subprocess` identity pattern. **F-015 (MED)** — Canonical Test Vectors table made exhaustive over the renewal gate's domain: added empty-string-holder (treated identically to absent-block — no renewal, no identity resolution attempted), `exec_subprocess` exit-0-with-empty-stdout (classified as `IdentityResolutionFailed`, aligned with BC-4.13.001 EC-009's identical classification, NOT a silent mismatch), and the expired-self-lock case (F-009). 9 test-vector rows total (was 5), 3 new edge cases (EC-010/EC-011/EC-012), EC-004 and EC-007 rewritten in place (not renumbered, per POLICY 1 append-only-numbering — these are amendments to existing IDs' behavior, not identifier reuse). Also reflects ADR-046 v1.1's non-product-owner-routed design changes as they bear on this BC: the `renew_lock_if_holder` function name (Decision 1(b)) replaces this BC's informal 'gate' description; the `factory.lock.renewal_indeterminate` event + `log_warn` (Decision 4) is now specified as PC2's error-variant side effect on `IdentityResolutionFailed`, fired only when a self-held, non-expired lock was actually present (no event for the legitimate `NotHolder`/`AlreadyExpired`/absent-lock skips — ADR-046 Decision 4 non-goal); the canonical `factory_lock::TTL_SECONDS` const (Precondition 3, unchanged in substance, re-grounded in §SDK Grounding Evidence); the email-collision residual-risk scoping ('modulo email collision', ADR-046 F-010) added to Invariant 2; the tool-write-success gate (ADR-046 Decision 1(b)/F-013 — `tool_response` null/absent or carrying a top-level `error` key skips stamping entirely) added as a new Precondition. H1 re-enriched per POLICY 7 to reflect all of the above. Story Anchor, VP Anchors, and VP-registration status UNCHANGED (still [pending] — VP authoring remains formal-verifier/architect scope, not touched by this pass). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-25 (v1.0) — Initial authoring (product-owner; ADR-046 ratification companion amendment 2): new BC for the stamp-state-timestamp PostToolUse hook — unconditional timestamp: re-stamp, identity-gated factory_lock.expires_at renewal, fail-open, frontmatter-only, no acquire/release involvement. lifecycle_status: draft (POL-14 auto-promotion to active on implementing PR merge).]]]]]]]]]]]]]"
---

# BC-4.17.001: stamp-state-timestamp PostToolUse hook MUST unconditionally re-stamp STATE.md `timestamp:` on every qualifying write (even when identity resolution fails), MUST renew `factory_lock.expires_at` via `renew_lock_if_holder` ONLY when a lock is held, unexpired, and the hook's own resolved writer identity (post-canonical-trim) is byte-equal to the recorded `holder`, treats ONLY the `expires_at` arm as idempotent (`>=` comparison), MUST NOT repair a malformed `expires_at` or resurrect an already-expired self-held lock, emits `factory.lock.renewal_indeterminate` on identity-resolution failure, and fails open per-arm — never suppressing the unconditional `timestamp:` re-stamp for a renewal-arm-only failure — without ever touching lock acquire/release

## Description

The `stamp-state-timestamp` native-WASM plugin is a `PostToolUse` hook that fires after any
`Edit`/`Write`/`MultiEdit` tool call lands a write to `.factory/STATE.md`. It performs two
**independently fail-open** frontmatter-only *transforms*, composed in memory and persisted via
a single re-read-and-rewrite pass (**exactly one `host::write_file` per invocation, never two —
PC1's transform is applied first and PC2 is evaluated against PC1's output, so a `Renewed`
outcome's write carries both arms' changes**; ADR-046 §Decision 1, F-P10-001 — corrected from a
prior two-independent-writes model that silently clobbered `timestamp:` whenever a lock was held
and both arms fired):

1. **(PC1, unconditional)** advances the top-level `timestamp:` field to the current wall-clock
   UTC instant — this arm has no identity gate and no expiry gate; it is a live clock, never
   idempotent, and fires on every qualifying invocation including every renewal-arm failure mode.
2. **(PC2, conditional)** renews `factory_lock.expires_at` to `now + TTL_SECONDS` via the shared
   pure function `factory_lock::renew_lock_if_holder` (ADR-046 Decision 1(b)) — but only when a
   `factory_lock` block is present, its `holder` is non-empty, its `expires_at` is present and
   **not already expired**, AND the hook's own independently-resolved caller identity
   (`git config user.email` via `host::exec_subprocess`, trimmed through the canonical
   `trim_git_email` function) is byte-equal to `holder`. This arm IS idempotent in the narrow
   sense that a same-wall-clock-second re-invocation produces a byte-identical `expires_at` and
   is treated as a no-op success, not a failure.

This plugin retires `verify-state-timestamp-refresh` (the PreToolUse detector it replaces):
where that guard could only *block* an agent's stale value, this hook makes staleness
structurally unreachable by removing the agent from the authorship path entirely — agents
never write `timestamp:` or `expires_at` by hand again.

This BC operationalizes BC-5.40.001 Invariant 1 ("state-manager is the sole writer," now
qualified **modulo email collision** per ADR-046 F-010) at its one previously-undefended point:
the mid-burst renewal act (BC-5.40.001 PC4). It does not participate in lock acquisition,
release, or the CAS-push discipline — those remain exclusively with `factory-lock-write.sh`
(`acquire`/`clear`, both invoked via the Bash tool, never observed by this hook's
`Edit|Write|MultiEdit` trigger) and `state-burst`'s fetch-then-CAS push (BC-5.40.001 PC5), both
unchanged by this BC.

`stamp-state-timestamp` and the amended `precompact-flush` (BC-7.07.001, ADR-046 Decision 3)
are the two call sites of the same shared `renew_lock_if_holder` function, the same
`trim_git_email` helper, and the same `factory_lock::classify_identity_resolution`
`(exit_code, stdout) -> IdentityResolution` classifier (F-003) — this BC and BC-7.07.001
MUST NOT independently re-derive any of the three.

This BC covers ADR-046's Decision points 1 (new plugin), 2 (identity model + trim/config-scope
fixes), and 4 (renewal-indeterminate diagnostic event), as they apply to this plugin
specifically. Decision 3 (the `precompact-flush` identity-gate extension) is BC-7.07.001's
concern, not this BC's — see Related BCs.

## Preconditions

1. A `PostToolUse` event has fired for a tool call matching `^(Edit|Write|MultiEdit)$` whose
   target path is `.factory/STATE.md`, AND the triggering tool call's own write landed
   successfully. **Tool-write-success check (ADR-046 Decision 1(b)/F-013):** if the `PostToolUse`
   payload's `tool_response` is `null`/absent, or is a JSON object carrying a top-level
   `"error"` key with a non-null value, the hook MUST treat the call as failed and skip both
   PC1 and PC2 entirely (`Continue` without touching STATE.md) — this is a distinct,
   earlier-evaluated gate from PC3a/PC3b below, and is not itself an error variant of either
   arm. **Accepted residual (explicit, not deferred):** a tool failure that produces a
   `tool_response` with no `error` key and no other failure signal will still be treated as
   success, causing a spurious-but-harmless `timestamp:` re-stamp on an effectively no-op edit
   — bounded and self-correcting by the next real edit; not engineered around per ADR-046
   Decision 1(b) rationale. The hook is registered with a capability scope of
   `path_allow = [".factory/STATE.md"]` for both `read_file` and `write_file` (mirroring the
   `regression-gate` PostToolUse `read_file`/`write_file` capability precedent already live in
   `hooks-registry.toml` — see §SDK Grounding Evidence) — it MUST NOT be granted read or write
   access to any other path.

2. The hook MUST be able to invoke `git config user.email` via `host::exec_subprocess`
   (`binary_allow = ["git"]`, `env_allow = ["HOME", "GIT_CONFIG_GLOBAL",
   "XDG_CONFIG_HOME"]` — identical requirement to BC-4.13.001 Precondition 4/Invariant 5;
   omitting `env_allow` produces the same `IdentityResolutionFailed`-class silent inertness
   documented there as EC-016). This capability is required ONLY for the PC2 gate check, and
   ONLY when the gate has already determined a lock is present, non-empty-`holder`, and
   unexpired (see PC2's evaluation order) — the unconditional `timestamp:` re-stamp (PC1) never
   needs to resolve caller identity, and the hook MUST NOT invoke `exec_subprocess` for an
   absent, empty-holder, malformed-`expires_at`, or already-expired lock, since there is nothing
   an identity match could legitimately renew in any of those cases.

3. `TTL_SECONDS` MUST be sourced from a single canonical constant shared with
   `plugins/vsdd-factory/bin/factory-lock-write.sh`'s own `TTL_SECONDS=2700` — the new
   `pub const TTL_SECONDS: u32 = 2700` in `crates/factory-lock` mandated by ADR-046 (Rationale
   §"Why the TTL must be sourced, not reinvented," F-006), imported by this plugin, with a
   cross-reference comment on both the Rust constant and the bash literal. Independently
   declaring a second `2700`-valued literal inside this plugin's own source is a production
   blocker. **Current state (pre-implementation), grounded:** no such canonical constant exists
   yet — three independently-typed `2700`-valued literals currently exist (`factory-lock`'s bare
   `Duration::seconds(2700)`, `precompact-flush`'s `LOCK_RENEWAL_TTL_SECS`, and
   `factory-lock-write.sh`'s `TTL_SECONDS=2700`); see §SDK Grounding Evidence for the literal
   grep evidence. This plugin's implementation (S-17.05) MUST create the canonical const as
   part of the same release, not add a fourth independent literal.

4. **`STATE_MD_MAX_BYTES` read cap + mandatory `extract_frontmatter` use (MIGRATED from
   BC-5.40.001 Precondition 6 — ADR-046 §Decision 5 reconciliation, F-P4-002):** the
   hook's single shared `host::read_file(".factory/STATE.md")` call (both PC1's `timestamp:`
   arm and PC2's `expires_at` arm read from this one call) MUST be bounded by
   `max_bytes = 262144` (256 KiB). **Sourcing (corrected 2026-08-26, ADR-046 §Decision 5 /
   F-P5-001 — mirrored per ADR-046's Companion Amendment 1 item (vi) correction, version pin
   stripped per POLICY 19 anti-volatile-pin, F-P6 sweep):**
   "Precondition 6 (`STATE_MD_MAX_BYTES` cap) MUST reference
   `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (`pub const
   STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to the `factory-lock-parse` crate per ADR-046
   §Decision 5 / F-P5-001) — not a locally re-declared constant, and not the
   now-deregistered `verify-state-timestamp-refresh` crate (which no longer declares it as of
   this correction)." (CANONICAL PRINCIPLE Rule 4 — reuse, not a second declaration; this
   corrects the prior v1.4 phrasing, which pointed at `verify-state-timestamp-refresh`'s own
   pre-relocation declaration — the crate ADR-046 §Decision 5 deregisters and anticipates
   deleting.) Before any frontmatter field rewrite (`timestamp:` or
   `factory_lock.expires_at`), the hook MUST call
   `factory_lock_parse::extract_frontmatter(bytes)` (`crates/factory-lock-parse/`; S-19.02
   PR #610 — reuse, not reimplementation; see Invariant 7) and — **when, and only when, a
   valid opening/closing `---` fence WAS located** — operate exclusively on the returned
   frontmatter slice; in the no-fence-located case the hook takes PC3a's fully-structural
   suppress-both path (no write) per Invariant 7 (O-P16-001, corrected 2026-08-26 — this
   Precondition now gives the SAME literal directive for the fence-not-located degenerate
   input that Invariant 7 gives, rather than a literal reading that would direct operating
   on the full undelimited input `extract_frontmatter` returns in that case). The 256 KiB
   cap is established by ADR-025 §Decision 12 §12.5
   parity with `verify-factory-lock` (BC-4.13.001 Phase-A Precondition 3) and is unchanged by
   this migration — only the call site moved, from the retired guard's PreToolUse read to this
   hook's PostToolUse read, per ADR-046 Decision 5's per-element reconciliation table. When
   `host::read_file` returns `OutputTooLarge` (file exceeds cap), the hook MUST treat this
   identically to PC3a's other fully-structural fail-open causes — BOTH PC1 and PC2 are
   suppressed for that invocation (see EC-015; this is now this hook's own read-hazard
   boundary, not merely the retired guard's). The soft-warn threshold contract (Invariant 8)
   MUST be observed on every successful read.

## Postconditions

### PC1 — Unconditional `timestamp:` re-stamp

On every qualifying invocation (Preconditions 1's tool-write-success gate having passed), the
hook MUST re-stamp the top-level `timestamp:` frontmatter field to `chrono::Utc::now()`
formatted as RFC-3339 UTC (`YYYY-MM-DDTHH:MM:SSZ`, matching the format already produced by
`factory-lock-write.sh`'s `_epoch_to_iso` helper), regardless of what value (if any) the
agent's own write proposed for that field, regardless of whether a `factory_lock` block is
present, and **regardless of the outcome of PC2's gate evaluation — including every PC2 skip
reason and PC3b's identity-resolution failure**. This field carries no identity concern and no
expiry concern — advancing it is safe no matter who wrote the file or what state the lock is
in — so it is stamped with no gating of any kind beyond the structural PC3a fail-open path (see
below) and the Precondition 1 tool-write-success gate.

**Comparison semantics:** the field's new value MUST be `>=` its pre-invocation value under
byte comparison (see Invariants §1 for why "idempotent" does NOT describe this arm).

**Rewrite mechanism and duplicate-key handling (F-008, added 2026-08-26):** the hook locates
the `timestamp:` line via a first-match line scan within the frontmatter region (mirroring
PC4's targeted single-line-replacement mechanism — never a full YAML parse). If a SECOND line
beginning with `timestamp:` is present anywhere in the frontmatter region — a malformed-YAML
condition this hook does not create and does not independently validate against — the hook
rewrites ONLY the first-matched line and emits an advisory `host::log_warn`
(`"DuplicateTimestampKey: STATE.md frontmatter contains more than one timestamp: line; only
the first was rewritten; the stale second line is a pre-existing corruption this hook does not
repair"`) — never a block, and no exit-code change. This is an accepted, advisory-only
residual: duplicate-key STATE.md frontmatter is a manual-corruption scenario outside this
hook's structural-repair scope (see PC3a for the narrower class of failures this hook treats
as fail-open-worthy), and the now-retired `verify-state-timestamp-refresh` PreToolUse guard
could not have caught a stale second `timestamp:` line either. See EC-014.

**PC1's output is PC2's input, not a parallel branch (F-P10-001, ADR-046 §Decision 1, added
2026-08-26):** PC1's rewrite (whether it fires, per the anchor-presence gate above, or is a
no-op because the `timestamp:` anchor is absent) produces an in-memory value, `content_after_pc1`
— the shared `host::read_file` result with, at most, its `timestamp:` line advanced. This value,
**never the raw shared read**, is the sole input PC2's gate (Postcondition 2) evaluates and, on a
`Renewed` outcome, rewrites. PC1 therefore logically executes BEFORE PC2 is evaluated on every
invocation, even though the two arms remain independently gated on their own trigger conditions
(PC1 on tool-write-success + anchor presence; PC2 on its own five-case gate) — "independently
gated" describes each arm's own trigger, not the order of evaluation or how their outputs are
persisted. See Invariant 9 ("Single composed write, never two independent per-arm writes," new
in this pass) for how `content_after_pc1` and PC2's result are resolved into the one
`host::write_file` call this hook performs per invocation.

**Error variant:** none from PC2's gate (this arm is structurally independent of PC2's outcome);
see PC3a for this arm's only fail-open path; see above for the duplicate-key advisory.

### PC2 — Identity-and-expiry-gated `factory_lock.expires_at` renewal via `renew_lock_if_holder`

**Canonical signature (F-001, aligned to ADR-046 §Decision 1(b)):** the hook calls the
shared `factory_lock::renew_lock_if_holder<F, I>(content: &str, resolve_identity: I, now_fn: F)
-> Result<(RenewOutcome, Option<SkipReason>), LockError> where F: Fn() -> DateTime<Utc>,
I: FnOnce() -> IdentityResolution`. `resolve_identity` is a **lazy** closure — the hook supplies
`|| classify_identity_resolution(host::exec_subprocess(["git", "config", "user.email"]))` (see
the shared-classifier mandate below) — that `renew_lock_if_holder` invokes **at most once, and
only when the decision tree actually reaches the identity-comparison step (cases 3–5 below)**. The
hook itself never calls `exec_subprocess` directly or eagerly; it only ever supplies the
closure and lets `renew_lock_if_holder` decide whether to invoke it.

**Required input (F-P10-001, ADR-046 §Decision 1, added 2026-08-26):** `renew_lock_if_holder` is
called with `content_after_pc1` (PC1's output — see Postcondition 1), **never the raw
`host::read_file` result** — this is required for the composed write below to carry PC1's
`timestamp:` advance forward on a `Renewed` outcome; see ADR-046 §Decision 1 (F-P10-001).
**Call-site asymmetry:** this requirement is specific to `stamp-state-timestamp` — the one call
site with two concerns (PC1 + PC2) sharing a single read and a single write. `precompact-flush`
(BC-7.07.001) has only ONE concern, renewal, so it correctly feeds `renew_lock_if_holder` its own
single `host::read_file` result directly, with no prior transform to preserve — there is nothing
analogous to `content_after_pc1` at that single-concern call site, and it is unaffected by this
requirement.

The gate evaluates, in this order, on the post-write frontmatter (i.e., on `content_after_pc1`,
per the required-input paragraph above), producing one of the
**canonical five cases** (ADR-046 §Decision 1(b) return-value table — case numbering harmonized
2026-08-26, O-P4-002, to match BC-7.07.001's Invariant 3b table and the ADR verbatim: Malformed=1,
AlreadyExpired=2, NotHolder=3, IdentityResolutionFailed=4, Success=5; absent/empty-holder is the
ADR's pre-existing, unnumbered "0th" case, not one of the five):

**0th (pre-existing, unnumbered) — no lock held at all:**
- **Block absent** (`factory_lock` key not present) → `Ok((RenewOutcome::NoOp, None))`
  (nothing to renew). No `exec_subprocess` call is made; `resolve_identity` is not invoked.
- **`holder` absent or the empty string `""`** → `Ok((RenewOutcome::NoOp, None))`. An
  empty-string `holder` is treated identically to an absent block for this gate's purposes
  (F-015) — it is not "present but trivially matchable." No `exec_subprocess` call is made;
  `resolve_identity` is not invoked.

1. **Malformed — `expires_at` present but unparseable** (malformed — EC-007) → **`Err(LockError::Malformed(msg))`**
   (F-001: a distinct `Err` return, NOT a `NoOp`/`SkipReason` value — `SkipReason` has no
   `Malformed` variant to hold this case). `resolve_identity` is **not** invoked for this case —
   the malformed check is evaluated before any identity resolution is attempted, so the "no
   `exec_subprocess` call" guarantee is literally true, not merely aspirational. **The hook,
   as caller, downgrades this `Err` to an advisory `host::log_warn` and performs no write** —
   it MUST NOT compute or write a fresh `now + TTL_SECONDS` value to "repair" the malformed
   field (F-008) — `verify-factory-lock` treats a malformed `expires_at` as **unlocked**
   (fail-open, admits any caller); a stamper that repaired the same field would silently
   convert a state the guard reads as free into one re-locked for another `TTL_SECONDS`, with
   no caller having legitimately re-acquired it. The identity of the writer is irrelevant to a
   block this gate treats as inert.
2. **AlreadyExpired — `expires_at` present, parseable, and already expired** (`now >= expires_at`, evaluated
   against the hook's own wall clock) → `Ok((RenewOutcome::NoOp, Some(SkipReason::AlreadyExpired)))`
   (F-009, folded into ADR-046 §Decision 1(b)'s canonical return-value table). The hook
   MUST NOT resurrect an already-expired self-held lock: `verify-factory-lock` PC2
   (`LockExpired`) already treats an expired lock as free for **any** caller, including one
   whose email happens to match the stale `holder` value; if this gate renewed on identity
   match alone without an expiry check, an ordinary STATE.md edit by a brand-new session
   sharing the prior holder's email would silently re-lock the branch without that session ever
   having run `/factory-lock`. No `exec_subprocess` call is made for this case either —
   `resolve_identity` is not invoked — the expiry check is evaluated before identity resolution
   is attempted, since an expired lock's holder identity is irrelevant to the outcome.

**Not expired — invoke `resolve_identity()`** (the ONLY point in the decision tree where it
is invoked, and invoked at most once), yielding cases 3–5:

3. **NotHolder — `Resolved(email)`, mismatch:** the resolved caller identity, trimmed through
   the canonical `trim_git_email` function (promoted per ADR-046 Decision 2/F-004 — the same
   trim `factory-lock-write.sh`'s `tr -d '\n'` produces for the git-email domain), compared
   **byte-equal** against `holder` (which is itself already trimmed at acquire time by the same
   bash-side mechanism) — **Mismatch** → `Ok((RenewOutcome::NoOp, Some(SkipReason::NotHolder)))`
   — the expected, correct outcome for a non-holder writer legitimately admitted through
   BC-4.13.001 PC2 (`LockExpired`); this is NOT an error.
4. **IdentityResolutionFailed — `Failed(reason)`** (subprocess error, non-zero exit, timeout, or
   **empty stdout** — classified via the shared `factory_lock::classify_identity_resolution`
   function, F-003; the empty-stdout case is classified as `IdentityResolutionFailed`, not a
   silent mismatch, aligned with BC-4.13.001 EC-009's identical classification of the same
   condition — F-015) → `Ok((RenewOutcome::NoOp, Some(SkipReason::IdentityResolutionFailed
   { reason, holder, locked_at, expires_at })))` (**F-P21-001 — struct variant, corrected from
   the bare-`String` tuple variant:** the three non-`reason` fields are `lock_state.holder`/
   `.locked_at`/`.expires_at`, already parsed by `renew_lock_if_holder` at the holder-present
   step above — reaching case 4 requires the holder-present, malformed, and already-expired
   checks to have all passed, each of which requires `lock_state` to already be parsed, so
   populating these three fields alongside `reason` costs no additional parse or I/O; it is a
   pure struct-literal construction from data already in scope, per ADR-046 §Decision 1(b)/
   §Decision 4). See PC3b for the accompanying diagnostic side effect, which destructures this
   returned variant directly — never an unbound `<lock.*>` reference and never a re-parse of
   `.factory/STATE.md`'s `factory_lock:` block.
5. **Success — `Resolved(email)`, match:** the trimmed resolved identity is byte-equal to
   `holder` → **Match** → `Ok((RenewOutcome::Renewed(new_content), None))`: `expires_at` is set
   to `now + TTL_SECONDS` (2700 seconds), subject to the existing byte-identical-suppression
   guard (same-second re-invocation is `Ok((RenewOutcome::NoOp, None))`, not a `Renewed` —
   this is the arm's idempotent behavior). `holder` and `locked_at` are left untouched.

**Shared-classifier mandate (F-003):** the hook MUST construct its `resolve_identity` closure
by calling the shared `factory_lock::classify_identity_resolution((exit_code, stdout) ->
IdentityResolution)` function on its raw `exec_subprocess` result — the same function
BC-7.07.001's amended `precompact-flush` plugin calls. This function carries the load-bearing
empty-stdout→`IdentityResolutionFailed` rule (F-015, above). This hook and BC-7.07.001's plugin
MUST NOT independently re-derive this `(exit_code, stdout) -> IdentityResolution` classification
— see Description, which extends the existing "MUST NOT independently re-derive
`renew_lock_if_holder`/`trim_git_email`" mandate to this classifier as well.

The hook NEVER writes a `factory_lock` block into existence, and NEVER modifies `holder` or
`locked_at` under any outcome above — only `expires_at`, and only under a `Renewed` outcome.

**Error variant:** `Err(LockError::Malformed(msg))` (case 1 — downgraded by the hook to an
advisory `log_warn`, no write, never a block); `RenewalSkippedNotHolder` (advisory/diagnostic
only — the gate correctly declining to renew, not a failure); see PC3b for the
`IdentityResolutionFailed` outcome's distinct diagnostic side effect.

### PC3a — Fail-open on structural errors (suppresses BOTH arms) and PC1-only `timestamp:`-anchor absence (F-005)

A failure that means there is no valid, parseable frontmatter region to operate on at all —
`host::read_file` error, malformed or missing frontmatter delimiters (opening/closing `---`
absent or malformed), or non-UTF-8 content — MUST result in the hook silently declining to
write **anything** for that invocation (both fields left exactly as the agent's own tool call
produced them) and MUST NOT alter the `PostToolUse` result in any way that would suggest the
agent's own write failed. This is the ONLY category of failure under which PC2's gate is never
attempted.

**Scoped exception — `timestamp:` anchor absence ONLY (F-005, corrected 2026-08-26):** when the
frontmatter region itself IS validly parseable (well-formed delimiters, valid UTF-8, readable)
but contains no `timestamp:` key/line for PC1 to target, PC1 has nothing to rewrite and is
suppressed for that invocation **ONLY** — this does **NOT** suppress PC2. PC2's five-step gate
(see Postcondition 2) operates exclusively on the `factory_lock` block and has zero dependency
on whether a `timestamp:` key is present; the hook MUST still evaluate PC2's gate in full and,
if it resolves to `Renewed`, MUST still write the `factory_lock.expires_at` line even though
`timestamp:` was not rewritten in the same invocation. This is one of only two conditions
under which PC1's unconditional re-stamp is itself suppressed — the other being the
fully-structural case above, which suppresses both arms. See EC-013.

**Error variant:** `StampingSkipped(reason)` (advisory only; `host::log_warn` recommended for
operator visibility, never a block) for the fully-structural case; `TimestampAnchorMissing`
(advisory only, PC1-scoped; never a block) for the `timestamp:`-anchor-absence case — PC2's own
error variants (`RenewalSkippedNotHolder` / `RenewalSkippedIdentityMismatch` /
`RenewalIndeterminate`) apply independently and unaffected in that case.

### PC3b — Fail-open on identity-resolution errors (suppresses ONLY the PC2 renewal arm)

An `exec_subprocess`/identity-resolution failure encountered while evaluating PC2's gate
(case 4's `Failed` outcome) MUST result in the hook declining to renew `expires_at` for that
invocation — but MUST NOT suppress PC1's `timestamp:` re-stamp, which proceeds unaffected
(F-002: this is the narrowing that resolves the prior contradiction between PC3 and
EC-004/the canonical test vectors — PC1 and PC2 are independently fail-open arms, not a single
shared fail-open surface). **Renewal-indeterminate diagnostic (ADR-046 Decision 4):** because
this outcome only arises after PC2's gate has already confirmed a self-held, non-expired lock
is present (the 0th case and cases 1–2 were not triggered — lock present, not malformed, not
expired) and identity resolution alone failed, the hook MUST destructure the returned
`SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at }` variant
directly (**F-P21-001**, ADR-046 §Decision 1(b)/§Decision 4 — NOT an unbound `<lock.*>`
reference, and NOT a re-parse of `.factory/STATE.md`'s `factory_lock:` block; the three lock
fields are sourced from the single `renew_lock_if_holder` return value already in scope) and
call `host::emit_event("factory.lock.renewal_indeterminate", &[("plugin",
"stamp-state-timestamp"), ("holder", holder), ("locked_at", locked_at),
("expires_at", expires_at), ("resolution_error", reason)])` and `host::log_warn` with
a human-readable message ("RenewalIndeterminate: factory_lock held by {holder}, but this
session's identity could not be confirmed ({reason}); expires_at was NOT renewed — if this
session is the holder, the lock may expire mid-burst"). **Non-goal:** no event is emitted for
`NotHolder` (case 3, Resolved+Mismatch), `AlreadyExpired` (case 2), or an absent/empty-holder
block (the 0th case) — these are working-as-designed skips, not ambiguous failures; emitting an
event for every legitimate non-renewal would be noise (ADR-046 Decision 4 non-goal, unchanged).

**Error variant:** `RenewalSkippedIdentityMismatch`/`RenewalIndeterminate` (advisory only;
never a block; PC1 is unaffected)

### PC4 — Idempotent-on-`expires_at`-only, frontmatter-only rewrite

The hook's rewrite touches at most two single lines within the YAML frontmatter region
(bounded by the opening and closing `---` delimiters) — the `timestamp:` line and, when PC2's
gate resolves to `Renewed`, the `factory_lock.expires_at` line — via a targeted line
replacement (mirroring `factory-lock-write.sh`'s own `_update_expires_at` awk-style
single-line substitution pattern — see §SDK Grounding Evidence), never any modification to
content after the closing `---` delimiter and never any modification to a third frontmatter
line beyond these two.

**"Targeted" is a semantic-scope guarantee, not a write-mechanism claim (F-P10-001, ADR-046
§Decision 1, corrected 2026-08-26).** What "targeted" bounds is WHAT changes — at most the
`timestamp:` line's content and, when PC2 renews, the `factory_lock.expires_at` line's content;
it does NOT describe HOW that bounded change is persisted to disk. The persistence mechanism is
always **one whole-file `host::write_file` call** carrying the (mostly-unchanged) full
frontmatter-plus-body content with the targeted line(s) altered — the identical mechanism
`factory-lock-write.sh`'s own `_update_expires_at`/`rewrite_expires_at` already use for the same
class of single-line rewrite (see §SDK Grounding Evidence): neither is, nor does this codebase
provide, a byte-range or patch API. This is not a new claim invalidating the "targeted" language
above — it resolves what was previously an unstated ambiguity between "targeted" (semantic
scope) and `renew_lock_if_holder`'s `RenewOutcome::Renewed(new_content)` return shape, which is
always the FULL post-rewrite file content, not a diff or line fragment. Invariant 9 (below)
specifies exactly which of `content_after_pc1` or PC2's `new_content` is selected as that single
`host::write_file` call's argument for each combination of arm outcomes.

**Idempotency is scoped to the `expires_at` arm ONLY (F-007).** Re-running the hook against
its own just-written output: PC1 always re-advances `timestamp:` to a new "now" — this arm is
**explicitly NOT idempotent** (it is a live wall clock; a same-second re-invocation happens to
produce a byte-identical value, which is a harmless coincidence, not a designed guarantee, and
is NOT itself claimed as idempotent behavior). PC2's renewal, by contrast, IS idempotent in the
sense the `factory_lock::renew_lock_with_now` byte-identical-suppression guard already
provides: given the same frontmatter + identity inputs within the same wall-clock second, the
gate evaluates to the same outcome and, if `Renewed`, produces a byte-identical `expires_at`
(a `>=` comparison against the pre-invocation value — never claimed "strictly greater," since
second-precision wall-clock resolution makes same-second collisions unprovable as
strictly-greater and are treated as a no-op-equivalent success, not a violation).

### PC5 — No involvement in lock acquire, release, or CAS push

This hook's registered tool pattern (`^(Edit|Write|MultiEdit)$`) never intercepts
`factory-lock-write.sh acquire` or `factory-lock-write.sh clear` (both mutate STATE.md via
direct file I/O inside a Bash-tool-invoked shell script — `mv "$tmpfile" "$file"` — not via
an `Edit`/`Write`/`MultiEdit` tool call), nor `state-burst`'s fetch-then-`--force-with-lease`
CAS push (a `Bash` git-push invocation, likewise outside this hook's trigger set). Lock
lifecycle (who becomes holder, when the lock is released, the CAS-push discipline for
pushing the resulting commit) is entirely out of scope for this BC and remains governed by
BC-6.23.001 (acquire/release) and BC-5.40.001 PC5 (CAS push).

## Invariants

1. **`timestamp:` re-stamping has no identity gate, no expiry gate, and is not idempotent**:
   Unlike `expires_at`, the `timestamp:` field is always advanced regardless of caller
   identity, lock state, or presence of a `factory_lock` block. There is no scenario under
   which PC1 is skipped except PC3a's structural fail-open path — critically, PC3b's
   identity-resolution failure does NOT suppress PC1 (F-002). "Idempotent" is never claimed for
   this arm; it is a live clock, unconditionally re-stamped on every qualifying invocation.

2. **`expires_at` renewal never resurrects a foreign, expired, or malformed holder's lock**: The
   identity gate (PC2 cases 3–5) combined with the expiry precheck (PC2 case 2) and the
   malformed-non-repair rule (PC2 case 1) are the load-bearing safety properties distinguishing
   this hook's automatic renewal from a naive "renew whenever a `holder` is present" design. A
   caller who is not the recorded `holder` — including one legitimately admitted through
   BC-4.13.001 PC2 (`LockExpired`) — writing to STATE.md NEVER causes `expires_at` to advance
   for a lock it does not hold; an already-expired self-matching-email lock is NEVER
   resurrected without an explicit `/factory-lock` re-acquire; a malformed `expires_at` is
   NEVER "fixed" into a fresh future value. This is the specific hazard ADR-046 identified and
   designed against (see ADR-046 Rationale §"Why the identity gate on `expires_at`"), extended
   by this BC (F-009) to also cover the expiry-resurrection sub-case ADR-046's literal
   `renew_lock_if_holder` decision tree did not itself enumerate. **Scope note (ADR-046 F-010):**
   this invariant holds **modulo email collision** — two sessions authenticated under the same
   git email are indistinguishable to this identity check, exactly as they are to
   `verify-factory-lock`'s own PC3 self-held comparison; this is a pre-existing property of the
   email-keyed identity model, not a defect this BC introduces or is expected to close.

3. **`TTL_SECONDS` is a single canonical constant, not a duplicated literal**: Per
   Precondition 3, this plugin MUST import the same `TTL_SECONDS = 2700` value that
   `factory-lock-write.sh` uses, from one shared source (`crates/factory-lock::TTL_SECONDS`).
   This BC does not change the TTL value itself (BC-5.40.001 Invariant 2/AC-007 — 2700 seconds,
   non-configurable — is UNCHANGED); it only specifies where the new plugin's copy of that
   value must come from. See §SDK Grounding Evidence for the pre-fix literal-drift evidence
   this precondition closes.

4. **Fail-open is per-arm, never a single shared surface**: PC3a's fully-structural failures
   (`host::read_file` error, malformed/missing frontmatter delimiters, non-UTF-8) suppress BOTH
   PC1 and PC2. PC3a's `timestamp:`-anchor-absence case (F-005, corrected 2026-08-26) suppresses
   ONLY PC1 — PC2 evaluates its own five-step gate and, if satisfied, still renews `expires_at`,
   since PC2 has zero dependency on the `timestamp:` key's presence. PC3b (identity-resolution)
   suppresses ONLY PC2. There is no error mode in which PC2's failure suppresses PC1, and no
   error mode — other than PC3a's fully-structural case — in which a PC1-affecting failure also
   suppresses PC2. The agent's original STATE.md write is never rolled back, retried, or
   flagged as failed because of any stamping error in either arm.

5. **Frontmatter-only; body never read or parsed**: Mirrors BC-4.13.001 Invariant 9 and (as
   originally specified, prior to the ADR-046 §Decision 5 migration below) BC-5.40.001
   Invariant 7's frontmatter-only mandate — this BC's own Invariant 7 is now the live enforcer
   of that mandate for this hook's call site. The hook MUST operate exclusively
   within the region bounded by the opening and closing `---` delimiters and MUST NOT
   inspect, parse, or depend on any STATE.md body content (decision log, burst log
   narrative, etc.).

6. **This hook is the sole enabler that lets BC-5.40.001 Invariant 1's "sole writer"
   property be mechanically true at the renewal call site**: Prior to this hook's
   existence, nothing checked that only the recorded holder could cause `expires_at` to
   advance — renewal was a manual, unenforced act. This hook is the mechanism, not a
   policy restatement; BC-5.40.001 is amended (see BC-5.40.001 v1.5 Invariant 1) to cite
   this hook as the enforcement point. This invariant is shared with BC-7.07.001's amended
   `precompact-flush` renewal path (ADR-046 Decision 3) — both call the same
   `renew_lock_if_holder` function, so both are covered by the same mechanical guarantee.

7. **`extract_frontmatter` used exclusively for the shared STATE.md read (MIGRATED from
   BC-5.40.001 Invariant 7 — ADR-046 §Decision 5 reconciliation, F-P4-002):** the hook
   MUST call `factory_lock_parse::extract_frontmatter(bytes)` on the byte slice returned by
   the single shared `host::read_file` call (Precondition 4) before scanning for either
   `timestamp:` or `factory_lock.expires_at`, and — **when, and only when, a valid opening/
   closing `---` fence WAS located** — MUST operate only on the returned frontmatter slice —
   never the file body after the closing `---` delimiter. This is the
   same mandate BC-4.13.001 Invariant 9 imposes on `verify-factory-lock`, and the mandate
   BC-5.40.001 Invariant 7 (now historical/dormant) imposed on `verify-state-timestamp-refresh`
   — migrated here because this hook is the call site that actually executes this read in
   production after ADR-046 Decision 5 deregisters `verify-state-timestamp-refresh`. The
   `extract_frontmatter` function is provided by `crates/factory-lock-parse/` (pure-core
   crate; S-19.02 PR #610; reuse-not-duplicate per CANONICAL PRINCIPLE Rule 4) —
   reimplementing it in `crates/hook-plugins/stamp-state-timestamp/` is a production blocker.

   **Fence-not-located is a PC3a fully-structural fail-open cause, NOT an
   operate-on-the-full-slice case (F-P15-001, HIGH, corrected 2026-08-26 — reader-vs-writer
   migration defect):** `extract_frontmatter`'s own fail-open behavior — returning the full
   input byte slice when no valid opening/closing `---` fence can be located — is, for THIS
   (writer) hook, identical to the condition PC3a's fully-structural cause already names
   ("malformed or missing frontmatter delimiters (opening/closing `---` absent or malformed)",
   restated in ADR-046 §Decision 1's fail-open scoping as "the fence cannot be located"). When
   `extract_frontmatter` returns the full bytes for this reason, the hook MUST take PC3a's
   fully-structural fail-open path: BOTH PC1 and PC2 are suppressed for that invocation, and
   the single composed `host::write_file` is never attempted (see EC-005/EC-015). The hook
   MUST NOT apply PC1's `timestamp:` first-match line scan (or PC2's gate) to the undelimited
   full slice — doing so would let PC1's scan locate and rewrite the first `timestamp:`-
   prefixed line anywhere in the file, including STATE.md **body** content (decision-log rows,
   burst-log narrative, etc.), which is exactly the STATE.md-body-corruption hazard PC3a's
   fully-structural suppression and Invariant 5 (frontmatter-only) exist to prevent. This
   BC's prior text (through v1.7) instead migrated `verify-state-timestamp-refresh`'s reader
   semantics verbatim — "the guard applies its parse logic to the full returned slice without
   error" is a safe statement for a **read-only, block-or-continue guard** (BC-5.40.001's now-
   dormant Invariant 7; harmless there because the guard never writes) but is UNSAFE for a
   **writer** that rewrites a located frontmatter line back to disk. **This is a different
   sub-case from PC3a's `timestamp:`-anchor-absence scoped exception** (a valid fence WAS
   located, but no `timestamp:` key exists within it — PC1-only suppression; see Postcondition
   3a and Invariant 4) — the two must not be conflated: fence-not-located is the fully-
   structural, BOTH-arms-suppressed case; anchor-absence-within-a-valid-fence is the narrower,
   PC1-only-suppressed case. See VP-TBD-8, corrected in the same pass for the identical defect.

8. **Soft-warn threshold `state_md_approaching_cap` (MIGRATED from BC-5.40.001 Invariant 8 —
   ADR-046 §Decision 5 reconciliation, F-P4-002):** `soft_warn_threshold = 200000` bytes.
   This hook fires on every qualifying STATE.md write — more frequently than the retired
   `verify-state-timestamp-refresh` guard's PreToolUse dispatches — so the compaction-
   scheduling signal (D-442(e)) this soft-warn feeds MUST NOT go dark merely because the
   emitting guard was retired. When a successful read observes
   `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)`, the hook MUST emit a
   `state_md_approaching_cap` diagnostic event carrying `bytes_read: u64` and
   `cap_bytes: u64` (262144). This event is observability-only — it never suppresses PC1 or
   PC2, and never blocks. The soft-warn range is `bytes_read ∈ (200000, 262144]` — inclusive
   at the cap boundary:

   | `bytes_read` | Outcome |
   |---|---|
   | ≤ 200000 | No warn emitted; normal read |
   | 200001 | `state_md_approaching_cap` emitted; read succeeds |
   | 262144 | `state_md_approaching_cap` emitted AND read succeeds — warn MUST fire at cap boundary |
   | 262145 | `OutputTooLarge` returned by host; see EC-015 |

   This event requires zero new registry entries. The threshold is not a hard cap; it is a
   leading indicator for compaction scheduling (D-442(e)).

9. **Single composed write, never two independent per-arm writes (F-P10-001, ADR-046
   §Decision 1, added 2026-08-26 — corrected from a prior two-independent-writes model that was
   a deterministic lost-update):** the hook performs AT MOST ONE `host::write_file` call per
   qualifying invocation — never two racing writes: exactly one `host::write_file` call **when
   PC1 fires (the `timestamp:` anchor is present) OR PC2 renews** (rows 1–3 below), and zero
   writes when neither condition holds (row 4 — see the four-row table's own "no write" outcome,
   which this headline does not contradict). **Corrected (O-P15-001, LOW, 2026-08-26):** the
   headline previously read "when at least one arm produces a change," which mis-described row 2
   — a PC1-fired, PC2-`NoOp`/`Err(Malformed)` outcome still performs the single write of
   `content_after_pc1` even though PC2 produced no change; the trigger for that write is PC1
   **firing** (anchor-presence), not PC1 **producing a change** (PC1 is a live clock and always
   changes `timestamp:` when it fires, so the two framings happen to coincide for PC1, but
   "produces a change" is the wrong predicate to name — the table's own row selection is driven
   by PC1-fired/PC2-result, not by a generic "did anything change" test). The four-row table
   itself, this invariant's title, the defect narrative, and the advisory-independence note
   below are all UNCHANGED by this correction — only the headline's selection predicate wording.
   Its content, when a write does occur, is selected by whether PC1 fired (the `timestamp:`
   anchor was present) and what PC2's
   `renew_lock_if_holder(content_after_pc1, ...)` call returned:

   | PC1 fired (anchor present)? | PC2 result | Content written |
   |---|---|---|
   | Yes | `Ok((RenewOutcome::Renewed(new_content), _))` | `new_content` — carries BOTH the `timestamp:` advance and the `expires_at` renewal, because PC2 rewrote `content_after_pc1` (PC1's output), not the raw shared read |
   | Yes | `Ok((RenewOutcome::NoOp, _))` or `Err(LockError::Malformed(msg))` | `content_after_pc1` — PC1's `timestamp:` advance stands alone; a renewal skip or a malformed-lock error never suppresses it |
   | No (missing-`timestamp:`-anchor case) | `Ok((RenewOutcome::Renewed(new_content), _))` | `new_content` — PC2's result, computed against the raw shared read since PC1 contributed no change |
   | No | `Ok((RenewOutcome::NoOp, _))` or `Err(LockError::Malformed(msg))` | **no write** — neither arm produced a change for this invocation |

   **Advisories fire independently of the write row.** A `Malformed` outcome's `host::log_warn`
   and an `IdentityResolutionFailed` outcome's `factory.lock.renewal_indeterminate` event +
   `host::log_warn` (PC3b) are emitted according to PC2's own case regardless of which row of
   this table applies — the advisory and the write are separate concerns; neither advisory gates
   whether the `content_after_pc1`-alone write (row 2) happens.

   **This is the specific defect this invariant closes:** prior to 2026-08-26, this BC (mirroring
   ADR-046's pre-v1.8 text) described the two arms as "independently gated ... each with its own
   `host::write_file` call ... not a single joint write." Because `renew_lock_if_holder` is a
   pure function fed whatever content it is called with, and a `Renewed` outcome's `new_content`
   is the FULL file content with only `expires_at` changed, feeding it the SAME shared read PC1
   also read (rather than PC1's output) meant that on the primary held-lock-mid-burst path (both
   arms fire), PC1's write advanced `timestamp:` to T1 on disk, then PC2's write of its own
   `new_content` — built from the stale T0 read — silently reverted `timestamp:` back to T0,
   clobbering PC1's guarantee on exactly the writes where a lock is held. This invariant, combined
   with Postcondition 1's "PC1's output is PC2's input" requirement, is the fix: apply PC1 first,
   in memory; feed its output to PC2; write exactly once, per the table above. See the anti-clobber
   Canonical Test Vector (below) for the regression test this invariant exists to satisfy.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent writes STATE.md while no `factory_lock` block is present (factory unlocked) | `timestamp:` re-stamped (PC1); no `expires_at` renewal attempted (PC2 0th case: block absent, `NoOp`); no `exec_subprocess` call made; no `factory_lock` block is created |
| EC-002 | Agent writes STATE.md while a lock is held by that same agent's git identity, unexpired | `timestamp:` re-stamped; `expires_at` renewed to `now + TTL_SECONDS` (PC2 gate satisfied through case 5, Success/Resolved+Match); `locked_at` untouched |
| EC-003 | A caller admitted via BC-4.13.001 PC2 (`LockExpired`) — i.e., a non-holder writing while the recorded lock has expired — writes STATE.md | `timestamp:` re-stamped; `expires_at` NOT renewed — this is actually resolved at PC2 case 2 (`AlreadyExpired`) before identity is even checked, since the lock is expired regardless of who is writing; the stale, expired `holder`/`expires_at` values remain until the next `state-manager` write that legitimately updates or clears them |
| EC-004 | `git config user.email` is unset, times out, or `host::exec_subprocess` otherwise fails during the PC2 case 4 (`IdentityResolutionFailed`) gate check (lock present, non-empty `holder`, NOT expired) | `timestamp:` STILL re-stamped (PC1 is unaffected by a PC2-arm-only failure — F-002); `expires_at` left untouched (`SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at }`, PC3b fail-open); `factory.lock.renewal_indeterminate` event + `log_warn` emitted (ADR-046 Decision 4), with the event's `holder`/`locked_at`/`expires_at` fields destructured directly from the returned struct variant (**F-P21-001** — not re-parsed from `.factory/STATE.md`), because a self-held, non-expired lock WAS present and identity could not be confirmed |
| EC-005 | STATE.md frontmatter is malformed (missing closing `---` delimiter) at the time the hook reads it | PC3a structural fail-open: hook makes no write to EITHER field; agent's original content is untouched; `host::log_warn` advisory emitted; no `renewal_indeterminate` event (this is not an identity-resolution case) |
| EC-006 | Two back-to-back qualifying writes to STATE.md within the same burst (e.g., two intermediate `Edit` calls) | Each invocation independently re-stamps `timestamp:` to a new "now"; `expires_at` renews again on the second invocation if the gate is still satisfied (renewal is not "already done this burst" — it is stateless per-invocation; a same-second second write is a `NoOp` per the byte-identical-suppression guard, not a distinct failure) |
| EC-007 | `factory_lock` block present with `holder` non-empty but `expires_at` malformed (unparseable) | **Corrected (F-008; return-type corrected F-001):** the hook does NOT compute or write a fresh `expires_at`. PC2 case 1's `renew_lock_if_holder` call resolves to `Err(LockError::Malformed(msg))` — a distinct `Err` return, not a `NoOp`/`SkipReason` value — which the hook downgrades to an advisory `log_warn`; `resolve_identity` is NOT invoked (no `exec_subprocess` call is made — identity is irrelevant to a block this gate treats as inert, same disposition `verify-factory-lock` gives a malformed block — unlocked/fail-open); `timestamp:` (PC1) is unaffected and still re-stamps |
| EC-008 | Hook's own write to STATE.md races with a concurrent `state-manager` `git add`/`git commit` reading the file | Not a hazard: per ADR-046 Rationale ("Why no new CAS/race handling is needed"), `verify-factory-lock` blocks all other sessions' mutating tool calls while a foreign unexpired lock is held, so this hook's local re-read-then-rewrite has no cross-process contender in that window; the pre-existing state-burst CAS push (BC-5.40.001 PC5) remains the safety net for the actual push layer. The pre-existing, accepted unlocked-state concurrent-write risk (ADR-046 F-012) is out of scope for this BC. |
| EC-009 | Agent proposes its own (now-superfluous) `timestamp:` or `expires_at` value in its Edit/Write payload | The hook's PC1/PC2 rewrite runs AFTER the tool's own write lands, so the agent's proposed value is unconditionally overwritten for `timestamp:` (PC1) and conditionally overwritten for `expires_at` (PC2 gate) — there is no conflict resolution needed because the hook always wins for these two fields specifically |
| EC-010 | `factory_lock` block present, `holder` non-empty, `expires_at` parseable, but ALREADY EXPIRED (`now >= expires_at`), and the hook's own git email happens to equal the stale `holder` (e.g., a new session on the same laptop under the same git identity) | **New (F-009):** `timestamp:` re-stamped; `expires_at` NOT renewed — PC2 case 2 resolves to `SkipReason::AlreadyExpired` BEFORE identity resolution is even attempted; the expired lock is left inert, exactly as `verify-factory-lock` PC2 (`LockExpired`) already treats it, until an explicit `/factory-lock` re-acquire. No `exec_subprocess` call is made; no `renewal_indeterminate` event (this is a clean, non-ambiguous skip) |
| EC-011 | `factory_lock` block present with `holder: ""` (empty string, as distinct from the key being entirely absent) | **New (F-015):** treated identically to EC-001 (block/holder absent) — PC2 0th case resolves to `NoOp`; no `exec_subprocess` call is made; `timestamp:` still re-stamps. An empty-string `holder` is not "present but universally matchable" |
| EC-012 | `host::exec_subprocess(["git","config","user.email"])` exits 0 but produces empty stdout (git email genuinely unconfigured, as opposed to a subprocess error) | **New (F-015):** classified as `Failed`/`SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at }` — identical treatment to EC-004, NOT a silent "empty string mismatch" against `holder`. This mirrors BC-4.13.001 EC-009's identical classification of the same underlying condition, keeping the two BCs' identity-resolution taxonomies aligned. `factory.lock.renewal_indeterminate` event + `log_warn` emitted if the gate had already confirmed a self-held, non-expired lock (the PC2 0th case and cases 1–2 were not triggered — lock present, not malformed, not expired), with the event's `holder`/`locked_at`/`expires_at` fields destructured directly from the returned struct variant (**F-P21-001** — not re-parsed) |
| EC-013 | STATE.md frontmatter is well-formed (valid delimiters, valid UTF-8, readable) but contains NO `timestamp:` key/line; a `factory_lock` block IS present with `holder` non-empty, `expires_at` unexpired, and identity resolves+matches | **New (F-005, 2026-08-26):** PC1 is suppressed — no `timestamp:` line exists for it to rewrite (`TimestampAnchorMissing` advisory); PC2 is UNAFFECTED — it evaluates its own five-step gate independently and renews `expires_at` to `now + TTL_SECONDS` (`Renewed`) exactly as it would if `timestamp:` were present. This is the corrected scoping that replaces the pre-2026-08-26 behavior of also suppressing PC2 in this case |
| EC-014 | STATE.md frontmatter contains TWO lines each beginning with `timestamp:` (malformed/duplicate-key YAML, pre-existing corruption) | **New (F-008, 2026-08-26):** the hook's first-match line scan rewrites ONLY the first `timestamp:` line encountered; the stale second line is left untouched; an advisory `DuplicateTimestampKey` `log_warn` is emitted; no block, no exit-code change; PC2 evaluation is unaffected by this condition (it does not read `timestamp:` at all) |
| EC-015 | The shared `host::read_file(".factory/STATE.md")` call (Precondition 4) returns `OutputTooLarge` because STATE.md exceeds `max_bytes = 262144` (256 KiB) (**MIGRATED from BC-5.40.001 EC-010** — ADR-046 §Decision 5 reconciliation, F-P4-002) | Treated as a PC3a fully-structural fail-open cause: BOTH PC1 and PC2 are suppressed for that invocation (no `timestamp:` re-stamp, no `expires_at` renewal attempt); advisory `StampingSkipped`/`StateReadError` warn emitted; the agent's original write is left untouched. The 262144-byte cap exceeds D-442(e) structural limits (≤200 KiB under 500-line compaction discipline); exceedance indicates either compaction overdue or anomalous STATE.md inflation. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Qualifying `Edit` to STATE.md; no `factory_lock` block present | `timestamp:` advanced to hook-invocation-time UTC; no `factory_lock` block created; no `exec_subprocess` call | happy-path |
| Qualifying `Write` to STATE.md; `factory_lock.holder` (post-trim) == hook's resolved `git config user.email` (post-trim); `expires_at` unexpired | `timestamp:` advanced; `expires_at` advanced to `now + TTL_SECONDS`; `holder`/`locked_at` unchanged | happy-path |
| Qualifying `MultiEdit` to STATE.md; `factory_lock.holder` != hook's resolved identity (foreign/expired holder legitimately admitted via BC-4.13.001 PC2) | `timestamp:` advanced; `expires_at` UNCHANGED (`SkipReason::NotHolder` if unexpired, or `SkipReason::AlreadyExpired` if the recorded lock is itself expired — see next row) | edge-case |
| Qualifying write; `factory_lock.holder` non-empty, `expires_at` already expired (`now >= expires_at`), writer's git email happens to equal `holder` | `timestamp:` advanced; `expires_at` UNCHANGED (`SkipReason::AlreadyExpired`, PC2 case 2 — no identity resolution attempted; F-009) | edge-case |
| Qualifying write; `factory_lock.holder: ""` (empty string, block otherwise present) | `timestamp:` advanced; no renewal attempted; no `exec_subprocess` call (`NoOp`, treated as absent — F-015) | edge-case |
| Qualifying write; `factory_lock` block present, `holder` non-empty, `expires_at` unparseable (malformed) | `timestamp:` advanced; `renew_lock_if_holder` returns `Err(LockError::Malformed(msg))` (not `NoOp`, F-001); hook downgrades to advisory `log_warn`; `expires_at` UNCHANGED and NOT repaired; `resolve_identity` NOT invoked, no `exec_subprocess` call (F-008) | edge-case |
| Qualifying write; STATE.md frontmatter missing closing `---` delimiter | No write performed by the hook for EITHER field; agent's original content unchanged; advisory `StampingSkipped` warning (PC3a structural fail-open) | error |
| Qualifying write; lock present, `holder` non-empty, unexpired; `host::exec_subprocess(["git","config","user.email"])` fails (subprocess error, non-zero exit, or timeout) | `timestamp:` advanced (PC1 unaffected — F-002); `expires_at` UNCHANGED (`SkipReason::IdentityResolutionFailed`, PC3b); `factory.lock.renewal_indeterminate` event + `log_warn` emitted | error |
| Qualifying write; lock present, `holder` non-empty, unexpired; `host::exec_subprocess` exits 0 with EMPTY stdout (git email unconfigured) | Identical to the row above — classified as `IdentityResolutionFailed`, NOT a silent mismatch (F-015; aligned with BC-4.13.001 EC-009) | error |
| Qualifying write; frontmatter well-formed but NO `timestamp:` key present; `factory_lock.holder` (post-trim) == hook's resolved identity, `expires_at` unexpired | `timestamp:` NOT rewritten (`TimestampAnchorMissing` advisory, PC1-scoped); `expires_at` STILL advanced to `now + TTL_SECONDS` — PC2 unaffected (F-005) | edge-case |
| Qualifying write; frontmatter contains TWO `timestamp:` lines (duplicate-key corruption) | Only the FIRST `timestamp:` line is rewritten; stale second line untouched; advisory `DuplicateTimestampKey` `log_warn` emitted; no block (F-008) | edge-case |
| **Anti-clobber regression (MANDATORY, F-P10-001, added 2026-08-26):** Qualifying write; `factory_lock.holder` (post-trim) == hook's resolved identity (post-trim); `expires_at` unexpired — i.e., BOTH PC1 and PC2 fire in the same invocation | The post-hook STATE.md carries **BOTH** the NEW post-hook `timestamp:` value **AND** the NEW renewed `expires_at` value **simultaneously, in the same resulting file** (Invariant 9, row 1 of the single-write selection table: written content is PC2's `new_content`, computed against `content_after_pc1`, so it carries both changes). **This is the specific case a naive two-independent-writes implementation passes every per-arm test in isolation while still failing:** each arm's own output would be individually correct (a `timestamp:`-only test would see `timestamp:` advanced; an `expires_at`-only test would see `expires_at` renewed), but a two-write implementation performs PC1's write (`timestamp:` → T1), then overwrites disk with PC2's `new_content` — built from the pre-PC1 T0 read — silently reverting `timestamp:` back to T0. Only a test that reads the FINAL on-disk file and asserts both fields' new values together (as this vector does) catches the clobber | happy-path (regression) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD-1 | `timestamp:` is always advanced to a value `>=` (wall-clock, byte comparison) the pre-invocation value, for any qualifying invocation, regardless of frontmatter content or PC2 outcome — corrected from "strictly greater" (F-007): same-wall-clock-second invocations produce byte-identical, not strictly-newer, values | proptest/unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-2 | `expires_at` renewal fires if-and-only-if (`holder` present AND non-empty AND `expires_at` parseable AND NOT expired AND, post-`trim_git_email`, byte-equal to resolved caller identity) — exhaustive truth-table coverage of the five-step gate, including the `AlreadyExpired` (F-009) and empty-holder (F-015) rows | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-3 | Every PC3a fail-open path (malformed frontmatter, read error) results in zero bytes written by this hook for EITHER field; every PC3b fail-open path (identity-resolution error) results in zero bytes written for `expires_at` ONLY, while `timestamp:` still advances | unit-test / fuzz (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-4 | `TTL_SECONDS` constant used by this plugin is byte/value-identical to `factory_lock::TTL_SECONDS` and to `factory-lock-write.sh`'s `TTL_SECONDS` at every commit (no drift) | CI lint / manual (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-5 | A malformed `expires_at` (`holder` non-empty) is NEVER overwritten by this hook with a freshly-computed value — the hook's only action is an advisory warn | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-6 | An already-expired self-held lock (`holder` matches resolved identity, `now >= expires_at`) is NEVER renewed by this hook without an intervening explicit `/factory-lock` re-acquire | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-7 | **[MIGRATED equivalent of BC-5.40.001's S-19.08 T-001 — ADR-046 §Decision 5 reconciliation, F-P4-002; sourcing corrected 2026-08-26, F-P6-001.]** `STATE_MD_MAX_BYTES` constant used by this hook equals 262144 and is byte/value-identical to `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (relocated to the `factory-lock-parse` crate per ADR-046 §Decision 5 / F-P5-001) — not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate (which no longer declares it as of this correction) | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-8 | **[MIGRATED equivalent of BC-5.40.001's S-19.08 T-004/T-005 — ADR-046 §Decision 5 reconciliation, F-P4-002; corrected 2026-08-26, F-P15-001.]** `extract_frontmatter` is called on the shared `host::read_file` result before any `timestamp:`/`expires_at` scan, and the hook's PC1/PC2 rewrites never inspect bytes after the closing `---` delimiter when a valid fence was located; when `extract_frontmatter` instead returns the full undelimited input (no valid fence located), the hook takes PC3a's fully-structural fail-open path — BOTH PC1 and PC2 suppressed, zero bytes written — rather than scanning/rewriting the full returned content (corrected from the prior "no-delimiter fallback operates on the full returned content without error" reader-semantics phrasing, which was unfit for this writer hook — see Invariant 7) | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |
| VP-TBD-9 | **[MIGRATED equivalent of BC-5.40.001's S-19.08 T-007 — ADR-046 §Decision 5 reconciliation, F-P4-002.]** `state_md_approaching_cap` fires at `bytes_read ∈ (200000, 262144]` and does NOT fire at `bytes_read ≤ 200000`; `OutputTooLarge` (EC-015) is returned at `bytes_read = 262145` with zero warn emitted | unit-test (flag for formal-verifier: no VP-NNN assigned yet) |

**Note for formal-verifier / architect (VP registration):** None of the above have VP-NNN
IDs assigned yet — this BC has not yet been through VP-INDEX registration. Per this task's
scope (product-owner: BC content only), VP authoring is explicitly NOT performed here;
flagging per the `vp_index_is_vp_catalog_source_of_truth` policy for architect/formal-verifier
follow-up once implementation is scheduled. VP-TBD-5 and VP-TBD-6 are new in this pass (F-008,
F-009). VP-TBD-7/8/9 are new in this pass (F-P4-002 — ADR-046 §Decision 5 guard-read
reconciliation; migrated equivalents of BC-5.40.001's now-historical S-19.08 T-001/T-004/T-005/
T-007 rows, re-authored against `stamp-state-timestamp`'s own shared read call rather than the
retired `verify-state-timestamp-refresh` guard's).

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — CAP-031's own description explicitly names "TTL is 45 minutes with mid-burst renewal" as part of the mechanism it covers; this BC specifies the plugin that performs exactly that mid-burst renewal mechanically (previously a documentary, unenforced act), making it the correct anchor rather than a new capability. |
| L2 Domain Invariants | none (operational infrastructure invariant, not L2 domain spec — same classification as sibling BC-5.40.001 and BC-4.13.001) |
| Architecture Module | `crates/hook-plugins/stamp-state-timestamp/` (new crate, per ADR-046 File-Change Plan); `crates/factory-lock` (new `renew_lock_if_holder`, `IdentityResolution`, `SkipReason` types, per ADR-046 Decision 1(b) — this BC extends `SkipReason` with a third variant, `AlreadyExpired`, per F-009) |
| Stories | S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; ADR-046 implementation — confirmed implementing story, F-P25-002, corrected 2026-08-26) |
| ADR Reference | ADR-046 §Decision 1/§Decision 2/§Decision 4 (new plugin `stamp-state-timestamp`, identity model + trim/config-scope fixes, renewal-indeterminate diagnostic event; ratified 2026-08-25) |

## Related BCs

- BC-5.40.001 — depends on (this BC's plugin mechanically performs BC-5.40.001's PC4 mid-burst renewal obligation, on state-manager's behalf, identity-and-expiry-gated to the recorded `holder`)
- BC-4.13.001 — sibling (both are SS-04 hook-plugin contracts governing the same `factory_lock` frontmatter block; this BC's identity-resolution mechanism — `git config user.email` via `host::exec_subprocess`, and the empty-stdout→`IdentityResolutionFailed` classification — is the identical pattern BC-4.13.001 already established and proved safe in the WASM sandbox)
- BC-6.23.001 — related to (governs `/factory-lock` acquire and `/factory-unlock` release only; this BC's hook never observes those operations — see PC5)
- BC-7.07.001 — sibling (ADR-046 Decision 3; `precompact-flush`'s `PreCompact` renewal is amended to call the SAME `factory_lock::renew_lock_if_holder` function and the SAME `trim_git_email` helper this BC's plugin uses — no independent re-derivation permitted at either call site)

## Architecture Anchors

- `crates/hook-plugins/stamp-state-timestamp/` — new crate implementing this BC (per ADR-046 File-Change Plan)
- `crates/hook-plugins/verify-factory-lock/src/lib.rs` — source of the proven `git config user.email` / `exec_subprocess` identity-resolution pattern and the `trim_git_email` function this BC's plugin reuses (promoted to a shared location per ADR-046 Decision 2/F-004)
- `crates/factory-lock` — canonical `TTL_SECONDS` constant source (Precondition 3); new `renew_lock_if_holder`/`IdentityResolution`/`SkipReason` types (ADR-046 Decision 1(b), extended by this BC's F-009 `AlreadyExpired` variant); definitively the home of these symbols per ADR-046's File-Change Plan (corrected 2026-08-26, F-P6-003 — the prior "(or `crates/factory-lock-parse`)" hedge is removed; `factory-lock-parse`'s only genuine residents are `STATE_MD_MAX_BYTES` and `extract_frontmatter`, cited separately below)
- `crates/factory-lock-parse/src/lib.rs` — `extract_frontmatter` pure-core function (S-19.02 PR #610; reused by this hook's Precondition 4/Invariant 7 read cap, MIGRATED from BC-5.40.001's `verify-state-timestamp-refresh` guard-read contract per ADR-046 §Decision 5, F-P4-002; no modifications permitted by this migration)
- `plugins/vsdd-factory/bin/factory-lock-write.sh` — `TTL_SECONDS` bash-side mirror; `_update_expires_at`/`_write_factory_lock_block` awk-pattern this BC's single-line rewrite mirrors; the `HOLDER="$(git config user.email 2>/dev/null | tr -d '\n')"` assignment, the bash-side half of the F-004 trim-equivalence argument; `renew` mode retained as break-glass fallback (BC-5.40.001 PC4)
- `plugins/vsdd-factory/hooks-registry.toml` — new `stamp-state-timestamp` PostToolUse entry (to be added); `verify-state-timestamp-refresh` entry to be deregistered (BC-4.13.001/ADR-046 concern, not this BC's); `regression-gate` entry (PostToolUse `read_file`/`write_file` capability precedent — see §SDK Grounding Evidence)
- `.factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md` — authoritative design source for this BC

## Story Anchor

S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; `.factory/stories/S-17.05-stamp-state-timestamp-hook.md`) — confirmed implementing story for this BC's ADR-046 hook (F-P25-002, corrected 2026-08-26; verified present in STORY-INDEX.md).

## VP Anchors

- [pending] — see Verification Properties table note above; VP-NNN IDs to be assigned by architect/formal-verifier once implementation is scheduled, per `vp_index_is_vp_catalog_source_of_truth` policy.

## §SDK Grounding Evidence

Per POLICY 5 (SDK-GROUNDING MANDATE, v1.3), the following literal-shell grep evidence grounds
every external-artifact claim this BC makes. All captures use stable anchors (no line numbers,
no `grep -n`) per POLICY 5 v1.3.1.

**1. Pre-fix TTL-literal drift (grounds Precondition 3 / Invariant 3's claim that no canonical
`TTL_SECONDS` constant currently exists):**

```
$ grep -oE 'Duration::seconds\([0-9]+\)' crates/factory-lock/src/lib.rs
Duration::seconds(2700)

$ grep -c 'pub const TTL_SECONDS' crates/factory-lock/src/lib.rs
0

$ grep -oE 'LOCK_RENEWAL_TTL_SECS[a-zA-Z0-9_: =]*' crates/hook-plugins/precompact-flush/src/lib.rs
LOCK_RENEWAL_TTL_SECS: u64 = 2700

$ grep -oE 'TTL_SECONDS=[0-9]+' plugins/vsdd-factory/bin/factory-lock-write.sh
TTL_SECONDS=2700
TTL_SECONDS=2700
```

Confirms: three independently-typed `2700`-valued literals exist today, zero canonical
`factory_lock::TTL_SECONDS` const exists yet — grounding the mandate that this BC's
implementation must create it, not add a fourth.

**2. `regression-gate` PostToolUse `read_file`/`write_file` capability precedent (grounds
Precondition 1's claim that a narrowly-`path_allow`-scoped PostToolUse read+write capability
pair is already precedented in this registry):**

```
$ grep -oE '^name = "regression-gate"' plugins/vsdd-factory/hooks-registry.toml
name = "regression-gate"

$ awk '/name = "regression-gate"/{f=1} f{print} f && /^\[\[hooks\]\]/ && NR>1 && c++{exit}' \
    plugins/vsdd-factory/hooks-registry.toml | grep -E '^\[|path_allow|name ='
name = "regression-gate"
[hooks.capabilities.read_file]
path_allow = [".factory/regression-state.json"]
[hooks.capabilities.write_file]
path_allow = [".factory/regression-state.json"]
```

Confirms: `regression-gate` already declares a single-file-scoped `read_file`+`write_file`
capability pair on a `.factory/`-relative path — the exact shape this BC's plugin's own
`path_allow = [".factory/STATE.md"]` capability declaration mirrors.

**3. `factory-lock-write.sh`'s trim assignment (grounds PC2 cases 3–5's claim that `holder` is
already trimmed at acquire time via the same trim this hook must apply at renewal time):**

```
$ grep -oE '^\s*if ! HOLDER=.*' plugins/vsdd-factory/bin/factory-lock-write.sh
    if ! HOLDER="$(git config user.email 2>/dev/null | tr -d '\n')" || [[ -z "$HOLDER" ]]; then
```

Confirms: the bash acquire path strips the git-config stdout with `tr -d '\n'` before ever
writing `holder` to STATE.md — this is the bash-side half of the F-004 trim-equivalence
argument this BC's `trim_git_email` reuse depends on.

**4. `verify-factory-lock`'s `trim_git_email` and `exec_subprocess` pattern (grounds PC2's
claim that the identity-resolution mechanism this BC's plugin reuses is already proven safe in
the WASM sandbox):**

```
$ grep -oE 'fn trim_git_email[a-zA-Z0-9_(): &<>,]*' crates/hook-plugins/verify-factory-lock/src/lib.rs
fn trim_git_email(raw: &str) 

$ grep -oE 'raw\.trim_end\(\)[a-zA-Z0-9_.()]*' crates/hook-plugins/verify-factory-lock/src/lib.rs
raw.trim_end().to_string()

$ grep -oE '\["git", *"config", *"user\.email"\]' crates/hook-plugins/verify-factory-lock/src/lib.rs
["git", "config", "user.email"]
["git", "config", "user.email"]
```

Confirms: `verify-factory-lock` already implements `trim_git_email(raw: &str) ->
raw.trim_end().to_string()` and already calls `exec_subprocess(["git", "config",
"user.email"])` from within the WASM sandbox — the exact function (to be promoted to a shared
location per ADR-046 Decision 2) and the exact subprocess invocation pattern this BC's plugin
and BC-7.07.001's amended `precompact-flush` both reuse rather than re-derive.

**5. `precompact-flush`'s pre-fix `renew_lock` call sites (grounds BC-7.07.001's sibling
relationship claim — the call sites this BC's shared `renew_lock_if_holder` function replaces
in the amended plugin, per ADR-046 Decision 3):**

```
$ grep -oE 'renew_lock\([a-zA-Z_&.]*\)' crates/hook-plugins/precompact-flush/src/lib.rs
renew_lock(state_md_content)
renew_lock()
renew_lock()
renew_lock(&state_md_content)
```

Confirms: `precompact-flush` currently calls the identity-blind `factory_lock::renew_lock`
(not yet `renew_lock_if_holder`) — this is the pre-fix state BC-7.07.001's own amendment
grounds and updates.

## Changelog

| Version | Date | Description |
|---------|------|--------------|
| 1.13 | 2026-08-26 | Pass-28 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 28 finding **F-P28-001a (HIGH, POLICY 18)**. `inputs:` frontmatter omitted `crates/factory-lock-parse/src/lib.rs` despite this BC's own load-bearing claims against it (Precondition 4's `factory_lock_parse::STATE_MD_MAX_BYTES`/`extract_frontmatter` citations; VP-TBD-7/VP-TBD-8; §Architecture Anchors). This is a mirror-image gap of BC-7.07.001's own v1.29 F-P27-003 fix, which falsely claimed to be "mirroring sibling BC-4.17.001's input set" — this BC's array did NOT yet contain the file at that time; that false cross-reference is corrected separately in BC-7.07.001 v1.30 (F-P28-001b), same burst. Added `crates/factory-lock-parse/src/lib.rs` to this BC's `inputs:`, same path form BC-7.07.001's array already uses, independently justified against this BC's own load-bearing claims. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); pure `inputs:` addition. input-hash recompute flagged to state-manager. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.12 | 2026-08-26 | Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding **F-P25-002 (MED)**. Traceability §Stories row and §Story Anchor's `[pending]` placeholders for the ADR-046 identity-gate amendment implementation resolved to the confirmed implementing story: S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; `.factory/stories/S-17.05-stamp-state-timestamp-hook.md`), verified present in STORY-INDEX.md. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); both are in-place corrections to existing Traceability text. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.11 | 2026-08-26 | Pass-21 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.10's **F-P21-001 (MED)** corrected event-sourcing for `SkipReason::IdentityResolutionFailed`. PC2 case 4's return value corrected from the bare-`String` tuple variant `Ok((RenewOutcome::NoOp, Some(SkipReason::IdentityResolutionFailed(reason))))` to the struct variant `Ok((RenewOutcome::NoOp, Some(SkipReason::IdentityResolutionFailed { reason, holder, locked_at, expires_at })))` — the three non-`reason` fields sourced from `lock_state.holder`/`.locked_at`/`.expires_at`, already parsed by `renew_lock_if_holder` at the holder-present step, no re-parse. PC3b's `factory.lock.renewal_indeterminate` event emission corrected from an unbound `<lock.*>` reference to a direct destructure of the matched variant — `host::emit_event("factory.lock.renewal_indeterminate", &[("plugin", "stamp-state-timestamp"), ("holder", holder), ("locked_at", locked_at), ("expires_at", expires_at), ("resolution_error", reason)])`, sourced from the single `renew_lock_if_holder` return value, never re-parsed from `.factory/STATE.md`'s `factory_lock:` block. EC-004 and EC-012 updated with the identical destructuring clarification. BC-7.07.001's companion amendment (same burst) mirrors the identical struct variant + event-sourcing text for `precompact-flush`'s call site, per ADR-046 Companion Amendment 3 — both BCs now state the IDENTICAL struct variant and event-sourcing mechanism. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction to PC2 case 4's return value and PC3b's event-emission text only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.10 | 2026-08-26 | Pass-16 fence-not-located Precondition/Invariant parity fix + exhaustive writer-fitness precision sweep (product-owner), responding to adversarial spec review pass 16 observation **O-P16-001 (LOW)**. Precondition 4's `extract_frontmatter`-use mandate ("...and operate exclusively on the returned frontmatter slice") was stated UNCONDITIONALLY, while Invariant 7 (corrected at v1.9, F-P15-001) scopes the identical mandate to apply only when a valid `---` fence WAS located, routing the fence-not-located case to PC3a's fully-structural suppress-both path instead — a literal reading of Precondition 4 alone would, in the no-fence case, still direct the hook to operate on the full undelimited slice `extract_frontmatter` fail-opens to, the same corruption hazard v1.9 corrected Invariant 7 to forbid. Corrected: Precondition 4 now carries the SAME "when, and only when, a valid opening/closing `---` fence WAS located" qualifier Invariant 7 carries, routing the no-fence case to PC3a's suppress-both path per Invariant 7 — Precondition 4 and Invariant 7 now give the IDENTICAL literal directive for the same degenerate input. **Exhaustive precision sweep** of every remaining writer-fitness-frontmatter-directive locus in this BC (PC1's first-match line-scan mechanism, PC3a's fully-structural definition and its scoped anchor-absence exception, PC4, Invariant 5, Invariant 8, EC-014, EC-015, VP-TBD-7/8/9, Architecture Anchors) confirmed all other loci CLEAN — each either already carries the fence-located qualifier (Invariant 7, VP-TBD-8) or is genuinely unaffected by the no-fence case (all others, being vacuously satisfied, emit-only, or presupposing an already fence-located region). No clause in this BC now gives an unqualified full-slice-operate directive that contradicts the PC3a/Invariant-7 no-fence suppress-both semantics. ADR-046 confirmed correct and unchanged (§Decision 1's fail-open scoping already states the fence-not-located mapping accurately) — not touched by this pass, per strict isolation. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place qualifier addition to Precondition 4 only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.9 | 2026-08-26 | Pass-15 reader-vs-writer migration-fitness remediation (product-owner), responding to adversarial spec review pass 15 finding **F-P15-001 (HIGH)** and observation **O-P15-001 (LOW)**. **F-P15-001 (HIGH)** — Invariant 7's final sentence directed the hook to "apply PC1/PC2 to the full returned slice without error" when `extract_frontmatter` returns full bytes because no valid `---` fence can be located, cross-referencing this to "PC3a's `timestamp:`-anchor-absence handling" — both wrong: this is the migrated `verify-state-timestamp-refresh` READER's fail-open behavior (harmless for a read-only block/continue guard), unfit for this WRITER hook, where scanning/rewriting the undelimited full slice would let PC1's first-match `timestamp:` line scan corrupt STATE.md BODY content. Corrected: fence-not-located now takes PC3a's FULLY-STRUCTURAL suppress-both path (no write at all), and the cross-reference is fixed to cite that path, not the anchor-absence sub-case (which is PC1-only-suppression and applies only when a valid fence exists but no `timestamp:` key does). VP-TBD-8 carried the identical migrated-reader-semantics defect ("no-delimiter fallback operates on the full returned content without error") and is corrected in the same pass. **Comprehensive writer-fitness audit** of every element migrated from the read-only guard (Precondition 4, Invariant 7, Invariant 8, EC-015, VP-TBD-7/8/9): only Invariant 7 and VP-TBD-8 were unfit; Precondition 4/Invariant 8/EC-015/VP-TBD-7/VP-TBD-9 confirmed already writer-correct (fail-open-suppresses-both or read-size-observability-only, no unsafe write direction). Confirmed no other LIVE writer BC inherited the unfit clause — BC-7.07.001 (`precompact-flush`) does not call `extract_frontmatter` at all (single-concern renewal path, no frontmatter line-scan); BC-5.40.001's dormant Invariant 7 (a read-only, deregistered guard) retains the reader phrasing verbatim and is correctly left untouched, per POLICY 1 append-only numbering, since it is benign for a non-writing guard. **O-P15-001 (LOW)** — Invariant 9's headline ("exactly one ... when at least one arm produces a change (rows 1–3)") mis-described row 2 (PC1-fired + PC2-`NoOp`/`Err(Malformed)` still writes `content_after_pc1` — the trigger is PC1 **firing**, i.e. anchor presence, not PC1 **producing a change**); reworded to "when PC1 fires (anchor present) OR PC2 renews," keeping the four-row table, invariant title, and defect narrative unchanged. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); all corrections are in-place. ADR-046 confirmed correct at §Decision 1 (fail-open scoping already states fence-not-located as a fully-structural, both-arms-suppressed cause) — no ADR-046 change required. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.8 | 2026-08-26 | Pass-11 prose-consistency remediation (product-owner), responding to adversarial spec review pass 11 finding **O-P11-001 (LOW)**: Invariant 9's headline sentence ("the hook performs exactly ONE `host::write_file` call per qualifying invocation, never two, and never zero-or-two depending on which arm(s) fired") self-contradicted the same Invariant's own authoritative four-row selection table, whose row 4 (neither arm produces a change) is a zero-write outcome. Corrected to "AT MOST ONE `host::write_file` call per qualifying invocation — never two racing writes: exactly one ... when at least one arm produces a change (rows 1–3), and zero writes when neither arm does (row 4)" — the four-row table's content and every other Invariant 9 clause (title, defect narrative, advisory-independence note) are UNCHANGED; this is a headline-wording-only correction, no behavioral change. Confirmed ADR-046 §Decision 1 already phrases the four-row selection (including the "no write" row) correctly, so no ADR-046 change was needed. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.7 | 2026-08-26 | Pass-10 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.8's F-P10-001 (HIGH) corrected write-composition model. ADR-046 v1.8 replaced the two-independent-writes model (a deterministic `timestamp:` lost-update whenever a lock is held and both arms fire) with a single composed write: PC1's transform applied first, in memory, to the shared read; `renew_lock_if_holder` fed `content_after_pc1` (never the raw read); exactly ONE `host::write_file` per invocation, selected by a four-row table. Mirrored here: (1) Description tightened to state ONE write, never two. (2) PC1 gains a new paragraph naming `content_after_pc1` as PC2's required input. (3) PC2 preamble gains the required-input sentence plus the call-site-asymmetry contrast with `precompact-flush` (BC-7.07.001, single-concern, correctly unaffected). (4) New Invariant 9 states the four-row single-write selection table and the specific defect it closes. (5) PC4 reconciled: "targeted... never a full-file rewrite" is now scoped as a semantic-scope guarantee, not a write-mechanism claim — the bounded change is persisted via one whole-file `host::write_file` (mirroring `factory-lock-write.sh`'s `_update_expires_at`/`rewrite_expires_at`), removing the apparent contradiction with `renew_lock_if_holder`'s full-content `Renewed` return. (6) New MANDATORY anti-clobber Canonical Test Vector added — both arms fire (lock held by caller's own identity, unexpired) → assert the post-hook STATE.md carries BOTH the new `timestamp:` AND the new `expires_at` simultaneously; a naive two-write implementation passes every per-arm test in isolation while failing this one. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); Invariant 9 and the new test-vector row are pure additions. STRICT ISOLATION maintained: only BC-4.17.001 touched (ADR-046, BC-5.40.001, BC-7.07.001, registry, hook source, STATE.md, policies.yaml, ARCH-INDEX/BC-INDEX version untouched by this amendment). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.6 | 2026-08-26 | Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED + 2 LOW total across the ADR-046 BC cluster, all sibling-sweep-straggler class). **F-P6-001 (MED)** — VP-TBD-7 corrected from the stale pre-F-P5-001 sourcing ("byte/value-identical to the constant `verify-state-timestamp-refresh` used before its ADR-046 Decision 5 deregistration") to `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration — mirroring Precondition 4's own already-corrected sourcing. **F-P6-003 (LOW)** — Architecture Anchors' `crates/factory-lock (or crates/factory-lock-parse)` hedge removed for `TTL_SECONDS`/`renew_lock_if_holder`/`IdentityResolution`/`SkipReason`, all definitively homed in `crates/factory-lock` per ADR-046's File-Change Plan. **Comprehensive sibling-sweep** — found and fixed 1 additional stray `ADR-046 vN.N` version-pin straggler in Precondition 4's own sourcing annotation, stripped to the stable `ADR-046 §Decision N` anchor form per POLICY 19 anti-volatile-pin; all other swept categories (expiry boundary, malformed-arm disposition, 5-case numbering, shared-fn homes, TTL cast, event fields, per-arm fail-open) clean. No PC/Invariant/EC renumbered (POLICY 1 append-only). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.5 | 2026-08-26 | Pass-5 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.6's F-P5-001 (HIGH). Precondition 4's `STATE_MD_MAX_BYTES` sourcing corrected from the ambiguous "the SAME constant `verify-state-timestamp-refresh` used" phrasing (which pointed, unnamed, at a to-be-deleted crate) to ADR-046 §Decision 5 v1.6's exact sourcing text, mirrored verbatim: Precondition 4 now references `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (`pub const STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to `factory-lock-parse`) — not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction of Precondition 4's existing sourcing sentence. BC-INDEX registration deferred to state-manager same-burst per POLICY 7/8. |
| 1.4 | 2026-08-26 | Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 findings against ADR-046 (now v1.5). **F-P4-001 (MED)** — out of scope for this BC (routed to BC-5.40.001 PC3). **F-P4-002 (MED)** — new Precondition 4 (`STATE_MD_MAX_BYTES = 262144` cap + mandatory `extract_frontmatter`), new Invariant 7 (`extract_frontmatter` exclusive), new Invariant 8 (`state_md_approaching_cap` soft-warn), new EC-015 (`OutputTooLarge` fail-open), and new VP-TBD-7/8/9 rows — all MIGRATED from BC-5.40.001's now-historical Precondition 6/Invariant 7/Invariant 8/EC-010/S-19.08 T-001/T-004/T-005/T-007 per ADR-046 v1.5 §Decision 5's per-element reconciliation table; reuses the SAME `STATE_MD_MAX_BYTES`/`extract_frontmatter` (no re-declaration). **O-P4-001 (LOW)** — PC2 case numbering harmonized to ADR-046's canonical five-case numbering (Malformed=1/AlreadyExpired=2/NotHolder=3/IdentityResolutionFailed=4/Success=5; absent/empty-holder = unnumbered "0th" case), matching BC-7.07.001's Invariant 3b table; all body cross-references swept to match. **O-P4-002** — inline `ADR-046 vN.N §Decision N` version pins in body prose (PC2 preamble, return-value-table cites) stripped to stable `ADR-046 §Decision N` anchor form. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); new additions only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.3 | 2026-08-26 | Pass-3 spec remediation (product-owner), responding to adversarial spec review pass 3 findings F-001/F-003/F-004 against ADR-046 (now v1.4). **F-001 (HIGH)** — PC2 step 3's malformed-`expires_at` outcome corrected from `NoOp` to the distinct `Err(LockError::Malformed(msg))` return ADR-046 v1.4's canonical five-case table specifies (downgraded by the hook to an advisory `log_warn`, no write); PC2's preamble now states the full canonical `renew_lock_if_holder<F, I>(content, resolve_identity: I, now_fn: F) -> Result<(RenewOutcome, Option<SkipReason>), LockError>` signature with `resolve_identity` as a lazy `FnOnce` closure invoked at most once, only at the identity-comparison step; EC-007 and the corresponding Canonical Test Vectors row updated to match; this BC and BC-7.07.001 now state the IDENTICAL return contract. **F-003 (MED)** — Description's "MUST NOT independently re-derive" mandate extended to the shared `factory_lock::classify_identity_resolution` classifier; PC2 step 5 and a new "Shared-classifier mandate" paragraph cite it explicitly. **F-004 (MED)** — out of scope for this BC (BC-7.07.001-only finding). No PC/Invariant/EC renumbered. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.2 | 2026-08-26 | Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 PRODUCT-OWNER-ROUTED findings against ADR-046 (now v1.2). **F-002 (HIGH, POLICY 19)** — stripped the load-bearing `ADR-046 v1.1` version pin from the Traceability § ADR Reference row to the stable `ADR-046 §Decision 1/§Decision 2/§Decision 4` anchor form. **F-003 (HIGH)** — resolved the v1.1 last_amended/Changelog "flagged for architect: AlreadyExpired not yet enumerated" notes: ADR-046 v1.2 §Decision 1(b) now enumerates `AlreadyExpired`; historical narrative annotated accordingly. **F-005 (MED)** — PC3a corrected: `timestamp:`-anchor absence now suppresses PC1 ONLY (not PC2, which has zero dependency on `timestamp:`'s presence); Invariant 4 updated; new EC-013. **F-008 (LOW)** — PC1's first-match `timestamp:` line-rewrite mechanism made explicit; duplicate-`timestamp:`-line behavior specified (rewrite first match only, advisory `DuplicateTimestampKey` warn, never a block); new EC-014. 2 new canonical test-vector rows. No PC/Invariant/EC renumbered. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.1 | 2026-08-25 | Pass-2 spec remediation (product-owner), responding to ADR-046 v1.1 and PRODUCT-OWNER-ROUTED findings F-002/F-004/F-007/F-008/F-009/F-014/F-015. PC3 split into PC3a (structural, suppresses both arms) and PC3b (identity-resolution, suppresses renewal arm ONLY — F-002). PC2 gate rewritten as a five-step evaluation order via `renew_lock_if_holder`, adding an explicit malformed-`expires_at` non-repair step (F-008) and an explicit already-expired-self-lock non-resurrection step with new `SkipReason::AlreadyExpired` (F-009, product-owner disposition of an ADR-046 gap — flagged for architect follow-up; RESOLVED at v1.2 — ADR-046 v1.2 §Decision 1(b) now enumerates `AlreadyExpired`, see v1.2 row above). Byte-equality comparison now names the canonical `trim_git_email` function applied to both sides (F-004). H1 and VP-TBD-1 rescoped: "idempotent"/"strictly greater" language now applies ONLY to the `expires_at` arm under a `>=` comparison; `timestamp:` is explicitly non-idempotent (F-007). New `§SDK Grounding Evidence` section added with 5 literal-shell grep blocks (F-014, POLICY 5). Canonical Test Vectors table expanded from 5 to 9 rows, made exhaustive over the gate domain: empty-string `holder`, `exec_subprocess` exit-0-empty-stdout (classified `IdentityResolutionFailed`, aligned with BC-4.13.001 EC-009), and the expired-self-lock case (F-015). 3 new edge cases (EC-010/EC-011/EC-012); EC-004 and EC-007 rewritten in place (append-only numbering preserved). `factory.lock.renewal_indeterminate` event + `log_warn` specified as PC3b's diagnostic side effect (ADR-046 Decision 4). 2 new VP-TBD rows (VP-TBD-5, VP-TBD-6). Related BCs / Architecture Anchors updated to cite BC-7.07.001 as the sibling call site of the shared `renew_lock_if_holder`/`trim_git_email` functions. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.0 | 2026-08-25 | Initial authoring (product-owner; ADR-046 ratification companion amendment 2, per ADR-046 §Companion BC/Policy Amendments item 2). New BC under SS-04 Plugin Ecosystem specifying the `stamp-state-timestamp` PostToolUse hook: unconditional `timestamp:` re-stamp (PC1); identity-gated `factory_lock.expires_at` renewal (PC2, gated on writer git-email == recorded `holder`); fail-open on any read/write/parse/identity-resolution error (PC3); idempotent frontmatter-only rewrite (PC4); explicit non-involvement in lock acquire/release/CAS-push (PC5). 6 invariants, 9 edge cases EC-001..EC-009, 5 canonical test vectors. Capability anchor CAP-031 (justified via capabilities.md §CAP-031's explicit "mid-burst renewal" language). lifecycle_status: draft (POL-14 auto-promotion pending implementing PR merge). BC-INDEX registration deferred to state-manager same-burst per POLICY 7/8 (not applied by this authoring). |
