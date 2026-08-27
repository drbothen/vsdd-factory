---
document_type: behavioral-contract
level: L3
version: "1.18"
status: active
producer: product-owner
timestamp: 2026-06-11T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.17.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md
  - crates/hook-plugins/verify-factory-lock/src/lib.rs
  - crates/factory-lock/src/lib.rs
  - crates/factory-lock-parse/src/lib.rs
  - crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs
  - crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs
  - plugins/vsdd-factory/hooks-registry.toml
  - plugins/vsdd-factory/hooks/verify-git-push.sh
  - plugins/vsdd-factory/bin/factory-lock-write.sh
  - plugins/vsdd-factory/skills/state-burst/SKILL.md
input-hash: "e5499da"
traces_to: .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
origin: brownfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-031"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified:
  - "2026-08-27 (v1.18) — Pass-44 remediation (product-owner), responding to adversarial spec-convergence pass 44 finding O-P44-001 (LOW, POLICY 4/5). O-P44-001: the v1.17 last_amended disposition prose misattributed an illustrative \"verbatim quote\" of CAP-031's description — it quoted this BC's OWN Capability Anchor Justification prose ('this BC defines the authoritative lock state data structure...') as if it were CAP-031's description text, when that phrase is this BC's own justification prose, not CAP-031's description. Ground truth (capabilities.md §CAP-031): the actual verbatim description text is \"Enforce single-writer cross-session exclusivity on factory-artifacts state.\" Corrected the v1.17 disposition's illustrative parenthetical to CAP-031's actual verbatim text, matching sibling BC-4.17.001 v1.20's (\"TTL is 45 minutes with mid-burst renewal\", present verbatim in capabilities.md §CAP-031) and BC-7.07.001 v1.34's (CAP-032's title, present verbatim in capabilities.md §CAP-032) already-correctly-anchored illustrative-quote pattern; sibling-parity check confirmed both siblings clean, no edit required. `capabilities.md` remains correctly listed in `inputs:` (that substance is unchanged — only the illustrative quote-snippet is corrected). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-27 (v1.17) — Pass-43 remediation (product-owner), responding to adversarial spec-convergence pass 43 finding F-P43-001 (MED, POLICY 18) plus a mandatory grep-complete inputs audit. F-P43-001: this BC's Capability Anchor Justification quotes CAP-031's description verbatim against `.factory/specs/domain-spec/capabilities.md`, absent from `inputs:` — added. Grep-complete inputs audit additionally found four more genuinely-cited-and-missing files: PC4's break-glass-fallback claim about `factory-lock-write.sh renew`'s exact current behavior (\"performs the same unconditional expires_at = now + 2700s update with no identity check\") and Precondition 5's `--force-with-lease` claim about `verify-git-push.sh` (\"only blocks raw --force; --force-with-lease is permitted\") are both load-bearing current-state claims against files absent from `inputs:` — added `plugins/vsdd-factory/bin/factory-lock-write.sh` and `plugins/vsdd-factory/hooks/verify-git-push.sh`; the VP Anchors section's literal grep-evidence block (shipped test function names) quotes `crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs` verbatim (T-006 function names) as grounding evidence distinct from the already-listed `src/lib.rs` — added. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-26 (v1.16) — Pass-37 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 37 finding F-P37-001 (MED, POLICY 4). Corrected ADR-046 Decision-count 1–5→1–6 in the v1.15 modified/last_amended/Changelog entries' ADR §Decision anchor audit prose (the flat `## Decision` list has 6 items, not 5 — item 6 is same-release ship + CI-gating registry-invariant XOR check). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-26 (v1.15) — Pass-35 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 35 finding F-P35-001 (HIGH, POLICY 4), plus a mandatory comprehensive ADR §Decision anchor audit. F-P35-001: two loci in this BC cited \"ADR-025 §Decision 12 §12.5\" for the 256 KiB `STATE_MD_MAX_BYTES` read-cap — Precondition 6's sourcing sentence and the Architecture Anchors ADR-025 bullet's \"cap parity\" clause — verified against ADR-025: §Decision 12 §12.5 is \"Shared parse logic — no duplication\" (the `factory-lock-parse` crate-extraction decision; states no byte-cap value anywhere in its text); the decision that actually raised the cap from 65536 to 262144 is §Decision 14 (\"verify-factory-lock read-cap 262144 + frontmatter-only parse\"), whose own \"Normative twin\" line names BC-4.13.001 §Precondition 3 (Phase-A) — the same BC these two loci already cross-cite. Corrected both loci from `ADR-025 §Decision 12 §12.5` to `ADR-025 §Decision 14`. The Architecture Anchors bullet's `§Decision 7 fail-open` clause was separately verified against ADR-025 §Decision 7 (\"Crash behavior — `on_error = \\\"continue\\\"` (fail-open)\") and confirmed CORRECT — left unchanged. **Comprehensive ADR §Decision anchor audit (mandatory, in-scope, newly-revealed dimension):** every `ADR-NNN §Decision N` citation in this BC's body was checked against the cited ADR's actual section content. ADR-025's §Decision 2/3/5/7/8/9/10/12/14 headings were read in full and cross-checked against every distinct citation this BC makes to them: all confirmed CORRECT beyond the two F-P35-001 defects fixed above. ADR-046's flat `## Decision` 1–6 numbered list (1 = new plugin + `renew_lock_if_holder`, 2 = identity model, 3 = extend gate to `precompact-flush`, 4 = renewal-indeterminate event, 5 = retire `verify-state-timestamp-refresh`, 6 = same-release + XOR CI-gating registry-invariant) was read in full and cross-checked against every distinct `ADR-046 Decision N` citation this BC makes: all confirmed CORRECT, no mis-anchors found. `inputs:` re-audited: ADR-025 already present (no gap here — this BC was never missing it, unlike sibling BC-4.17.001, whose own F-P35-002 gap is fixed same burst). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place sourcing-citation corrections only. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-26 (v1.14) — Pass-31 spec-convergence remediation + comprehensive cross-anchor/spec-inputs audit (product-owner), responding to adversarial spec-convergence pass 31 finding F-P31-001 (MED, POLICY 18) plus a mandatory comprehensive audit across the BC-4.17.001/BC-5.40.001/BC-7.07.001 cluster. F-P31-001: inputs: frontmatter omitted BC-4.13.001 and BC-6.23.001 despite this BC's own load-bearing current-state citations of both (BC-4.13.001 PC2/PC3/PC4/Invariant 9/Invariant 10; BC-6.23.001 PC1/PC3/PC4) — both sibling BCs (BC-4.17.001, BC-7.07.001) and ADR-046 already list both; added, same path form. Cross-anchor audit additionally found this BC's own PC1(precondition-4)/PC2 citations of \"BC-6.23.001 PC3/PC4\" for /factory-unlock clearing behavior wrongly included PC3 — verified BC-6.23.001 PC3 is \"/factory-lock foreign lock held: refuse\" (an ACQUIRE-path refusal, unrelated to /factory-unlock); the self-release clearing act this BC describes is BC-6.23.001 PC4 alone. Corrected both occurrences from \"BC-6.23.001 PC3/PC4\" to \"BC-6.23.001 PC4\". No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-26 (v1.13) — Pass-30 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 30 finding F-P30-001 (HIGH, POLICY 14/17). modified: array was ordered ASCENDING (v1.1 at top, v1.12 at bottom) while the Changelog table was correctly ordered DESCENDING (v1.12 at top) — a parity mismatch between the two POLICY 14 legs. Corrected: modified: array reordered to strict descending-chronological (newest at top), matching the Changelog table and sibling BC-7.07.001's own already-correct pattern (fixed at its v1.31, F-P29-003). Dated HISTORICAL entry text (v1.1 through v1.12) unchanged — only array position corrected, per POLICY 1 append-only numbering. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment)."
  - "2026-08-26 (v1.12) — Pass-29 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 29 finding F-P29-002 (MED, POLICY 18). inputs: frontmatter gap closed — this BC makes load-bearing exact-code-body current-state claims (PC3's is_expired comparison against verify-factory-lock; the migrated Precondition 6/Invariant 7/Invariant 8/EC-010 STATE_MD_MAX_BYTES/extract_frontmatter claims; the crates/factory-lock/renew_lock_if_holder mid-burst renewal claims) without listing any of the underlying code files in inputs:. Added: crates/hook-plugins/verify-factory-lock/src/lib.rs, crates/factory-lock/src/lib.rs, crates/factory-lock-parse/src/lib.rs, crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs, plugins/vsdd-factory/hooks-registry.toml — same path form BC-4.17.001/BC-7.07.001 already use. See last_amended for full disposition."
  - "2026-08-26 (v1.11) — Pass-27 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 27 finding F-P27-001 (HIGH, POLICY 4). §Story Anchor's Dual-story anchor quantifier corrected to Tri-story anchor; S-17.05 added to the story list (sibling-sweep straggler of the v1.10 Traceability-row fix). See last_amended for full disposition."
  - "2026-08-26 (v1.10) — Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding F-P25-002 (MED). Traceability §Stories row's [pending] placeholder resolved to the confirmed implementing story S-17.05 (stamp-state-timestamp-hook, E-17 Wave 5). See last_amended for full disposition."
  - "2026-08-26 (v1.9) — Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED total against this BC: F-P6-002; plus in-scope sweep fixes for stray POLICY 19 version pins). See last_amended for full disposition."
  - "2026-08-26 (v1.8) — Pass-5 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.6's F-P5-001 (HIGH) and O-P5-001 (LOW). See last_amended for full disposition."
  - "2026-08-26 (v1.7) — Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 findings F-P4-001/F-P4-002 against ADR-046 (now v1.5). See last_amended for full disposition."
  - "2026-08-26 (v1.6) — Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 PRODUCT-OWNER-ROUTED findings F-002/F-003 against ADR-025/ADR-046 (ADR-046 now v1.2). See last_amended for full disposition."
  - "2026-08-25 (v1.5) — ADR-046 v1.1 Companion Amendment 1 (i)-(v) (product-owner; pass-2 spec remediation): Invariant 1 email-collision scope note added; PC4 >= predicate + expires_at-only idempotency scoping added; PC4 malformed-expires_at non-repair statement added; PC4/Architecture Anchors TTL_SECONDS canonical-const sourcing note added; Canonical Test Vectors truth-table rows added covering {Resolved+Match, Resolved+Mismatch, Failed} x {stamp-state-timestamp, precompact-flush}; PC4 AlreadyExpired non-resurrection disposition added for consistency with BC-4.17.001/BC-7.07.001."
  - "2026-08-25 (v1.4)"
  - "2026-07-14 (v1.3)"
  - "2026-07-13 (v1.2)"
  - "2026-06-11 (v1.1)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.40.001
section: "5.40"
last_amended: "2026-08-27 (v1.18) — Pass-44 remediation (product-owner), responding to adversarial spec-convergence pass 44 finding **O-P44-001 (LOW, POLICY 4/5)**. Disposition: **O-P44-001** — the v1.17 disposition below misattributed an illustrative \"verbatim quote\" of CAP-031's description: it quoted this BC's OWN Capability Anchor Justification prose ('this BC defines the authoritative lock state data structure...') as if that phrase were CAP-031's description text, when it is in fact this BC's own justification sentence, not any part of CAP-031's description. Ground truth, verified by opening `.factory/specs/domain-spec/capabilities.md` §CAP-031: the capability's actual verbatim description opens \"Enforce single-writer cross-session exclusivity on factory-artifacts state\" (and separately states \"TTL is 45 minutes with mid-burst renewal\" for the mechanism sub-clause). Corrected the v1.17 disposition's illustrative parenthetical (below, in the nested `[Prior: ...]` text) from the misattributed self-quote to CAP-031's actual verbatim text. **Sibling-parity check (in-scope, this pass):** BC-4.17.001 v1.20's analogous illustrative quote (\"TTL is 45 minutes with mid-burst renewal\") and BC-7.07.001 v1.34's analogous illustrative quote (CAP-032's title, \"Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush\") were each independently verified against their cited CAP's actual description/title text in `capabilities.md` — both confirmed CORRECT, no misattribution found; neither sibling BC required an edit. `capabilities.md` remains correctly listed in this BC's `inputs:` (that substance — the file IS load-bearing — is unchanged by this fix); only the illustrative quote-snippet inside the dated v1.17 historical prose is corrected, per POLICY-1-sanctioned in-place correction of a current-state cross-reference claim in historical disposition prose (same class as the F-P43-002 in-place historical-prose correction). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-27 (v1.17) — Pass-43 remediation (product-owner), responding to adversarial spec-convergence pass 43 finding **F-P43-001 (MED, POLICY 18)** plus a mandatory grep-complete inputs audit. Disposition: **F-P43-001** — this BC's Capability Anchor Justification (Traceability section) quotes CAP-031's description verbatim ('Enforce single-writer cross-session exclusivity on factory-artifacts state') against `.factory/specs/domain-spec/capabilities.md`, but that file was absent from `inputs:` — added. **Mandatory grep-complete inputs audit (in-scope, this pass):** every `.factory/specs/*.md`, `crates/*.rs`, `plugins/*.{sh,toml}`, and `capabilities.md`/`invariants.md`/`prd.md`/`domain-spec` citation in this BC's body was enumerated and cross-checked against `inputs:`. Beyond the `capabilities.md` gap, three more genuinely-cited-and-missing files were found: (1) Postcondition 4's break-glass-fallback paragraph makes a load-bearing current-state claim about `factory-lock-write.sh renew`'s exact behavior ('performs the same unconditional expires_at = now + 2700s update with no identity check'), and Precondition 5/Invariant 5 make an equivalent claim about `verify-git-push.sh` ('only blocks raw --force; --force-with-lease is permitted') — neither file was listed in `inputs:` despite both sibling BCs already carrying `factory-lock-write.sh` in similar form; added `plugins/vsdd-factory/bin/factory-lock-write.sh` and `plugins/vsdd-factory/hooks/verify-git-push.sh`. (2) The §VP Anchors section's literal grep-evidence block ('Shipped test function names verified from commit 1304d280') quotes exact function names from `crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs` (the T-006 integration test file) verbatim — a distinct file from the already-listed `src/lib.rs` in the same crate; added. `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs` (cited only as a Proof Method locator for VP-096, no verbatim content quoted) was evaluated and judged NOT load-bearing by this standard — not added, to avoid padding. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.16) — Pass-37 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 37 finding **F-P37-001 (MED, POLICY 4)**. Disposition: corrected ADR-046 Decision-count 1–5→1–6 in the v1.15 modified/last_amended/Changelog entries' ADR §Decision anchor audit prose — ADR-046's `## Decision` section is a flat numbered list of 6 items (1–6), not 5; item 6 is same-release ship + CI-gating registry-invariant XOR check (`has_entry(verify-state-timestamp-refresh) XOR has_entry(stamp-state-timestamp)`). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). Also balanced a pre-existing nested-history bracket gap in this field's `Prior` chain (16 opens vs. 13 closes) to 16/16. [Prior: 2026-08-26 (v1.15) — Pass-35 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 35 finding **F-P35-001 (HIGH, POLICY 4)**, plus a mandatory comprehensive ADR §Decision anchor audit. Disposition: **F-P35-001 (HIGH)** — two loci in this BC cited \"ADR-025 §Decision 12 §12.5\" for the 256 KiB `STATE_MD_MAX_BYTES` read-cap: Precondition 6's sourcing sentence (\"The 256 KiB cap is established by ADR-025 §Decision 12 §12.5 parity with `verify-factory-lock`...\") and the Architecture Anchors ADR-025 bullet's \"cap parity\" clause. Ground truth, verified by opening ADR-025: §Decision 12 §12.5 is \"Shared parse logic — no duplication\" — the decision that promotes `parse_factory_lock`/`LockState`/`extract_yaml_string_value` to the shared `factory-lock-parse` crate; it states no byte-cap value anywhere in its text. The decision that actually raised the cap from the original 65536 to 262144 is §Decision 14 (\"verify-factory-lock read-cap 262144 + frontmatter-only parse\"), whose own \"Normative twin\" line names `BC-4.13.001 §Precondition 3 (Phase-A)` — exactly the BC these two loci already cross-cite. Corrected: both loci from `ADR-025 §Decision 12 §12.5` to `ADR-025 §Decision 14`, with the corrected decision's title and normative-twin citation quoted inline. The Architecture Anchors bullet's separate `§Decision 7 fail-open` clause was independently verified against ADR-025 §Decision 7 (\"Crash behavior — `on_error = \\\"continue\\\"` (fail-open)\") and confirmed CORRECT — this half of the bullet is unchanged; only the cap-parity half was mis-anchored. **Comprehensive ADR §Decision anchor audit (mandatory, in-scope, newly-revealed dimension prior passes did not run):** every `ADR-NNN §Decision N` citation in this BC's body was checked against the cited ADR's actual section content. ADR-025's `### Decision 2/3/5/7/8/9/10/12/14` headings were read in full and cross-checked against every distinct citation this BC makes to them (Decision 2 — lock state schema; Decision 3 — session identity; Decision 5 — TTL auto-expiry; Decision 7 — crash fail-open; Decision 8 — CAS push fix; Decision 9 — future git-ref CAS; Decision 10 — single-developer zero-friction; Decision 12/14 — the read-cap decisions, F-P35-001 above): all confirmed CORRECT beyond the two F-P35-001 defects. ADR-046's flat `## Decision` 1–6 numbered list was read in full and cross-checked against every distinct `ADR-046 Decision N` citation this BC makes: all confirmed CORRECT, no mis-anchors found. `inputs:` re-audited: ADR-025 already present in this BC's `inputs:` array (no gap here — unlike sibling BC-4.17.001, whose own F-P35-002 `inputs:`-omission gap is fixed in the same burst, separately). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); both corrections are in-place sourcing-citation fixes to existing text. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.14) — Pass-31 spec-convergence remediation + comprehensive cross-anchor/spec-inputs audit (product-owner), responding to adversarial spec-convergence pass 31 finding **F-P31-001 (MED, POLICY 18)** plus the mandatory comprehensive audit run across the BC-4.17.001/BC-5.40.001/BC-7.07.001 cluster this pass. Disposition: **F-P31-001 (MED, POLICY 18)** — this BC's `inputs:` frontmatter omitted BC-4.13.001 and BC-6.23.001 despite this BC's body citing both as load-bearing current-state authorities (BC-4.13.001: PC2 `is_expired`/`LockExpired` TTL-boundary comparison in PC3; PC1's `SchemaViolation` error-variant cite to BC-4.13.001's malformed-block fail-open path, which is BC-4.13.001 PC4; PC6's self-held-`Continue` cite, BC-4.13.001 PC3; Invariant 6's malformed-block-unlocked cite, BC-4.13.001 PC4; Invariant 8's soft-warn-threshold adjudication cite, BC-4.13.001 Invariant 10. BC-6.23.001: PC1's acquire-writes-the-block cite; PC2/Precondition-4's unlock-clears-the-block cites). Both sibling BCs (BC-4.17.001, BC-7.07.001) and ADR-046 already list both files in their own `inputs:` arrays — this BC was never itself swept for this specific pair. Corrected: `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` and `.factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md` added to `inputs:`, same path form the sibling BCs already use. This is NOT the accepted BC-4.17.001↔BC-7.07.001 mutual-inputs cyclic-hash TD (that concerns only that pair's mutual ADR/BC edges) — BC-4.13.001 and BC-6.23.001 are outside the mutual set. **Comprehensive cross-anchor audit finding (in-scope, production-grade default, found while auditing every BC-X.YY.ZZZ §Section/PCn/Invariant-N cross-reference in this BC's body against the cited BC's actual section content):** this BC's Precondition 4 (\"When the factory is unlocked... `state-manager` removes the block on `/factory-unlock`\") and Postcondition 2 (\"When `state-manager` writes an unlock... the `factory_lock` key MUST be absent\") both cited `BC-6.23.001 PC3/PC4` — verified against BC-6.23.001's actual section content: PC3 is `/factory-lock` foreign-lock-held ACQUIRE refusal (mirrors BC-4.13.001 PC1), entirely unrelated to `/factory-unlock`; the self-release unlock-clearing act this BC describes is BC-6.23.001 PC4 alone (\"`/factory-unlock` self-release: lock cleared\"). Corrected both occurrences from `BC-6.23.001 PC3/PC4` to `BC-6.23.001 PC4`. All other cross-anchors in this BC's body (17 body-section citations to BC-4.13.001 PC2/PC3/PC4/Invariant 9, BC-4.17.001 Precondition 4/Invariant 7/Invariant 8/EC-015, and BC-6.23.001 PC1) were individually verified against the cited BC's actual section content and confirmed CORRECT — no other wrong-section anchors found. Invariant 8's BC-4.13.001 Invariant 10 cite is left unchanged: that Invariant 8 paragraph is itself explicitly MIGRATED/historical-dormant text (superseded by BC-4.17.001's own live Invariant 8), and the cite accurately describes what BC-4.13.001 Invariant 10 covered AT THE TIME this dormant text was authored (v1.2, 2026-07-13) — BC-4.13.001 Invariant 10 has since evolved to a Phase-B envelope-diagnostic mechanism, but rewriting frozen historical adjudication prose is out of scope for a wrong-section-anchor fix and would violate POLICY 1 append-only preservation of dated historical text. No PC/Invariant/EC renumbered. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.13) — Pass-30 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 30 finding **F-P30-001 (HIGH, POLICY 14/17)**. Disposition: **F-P30-001 (HIGH)** — this BC's `modified:` frontmatter array was ordered ASCENDING (v1.1 at the top, v1.12 at the bottom) while the `## Changelog` table below was correctly ordered DESCENDING (v1.12 at the top, newest-first) — a POLICY 14 parity mismatch between the two required-to-agree legs. Sibling BC-4.17.001 carried the identical mismatch (fixed same burst, its own v1.15). Sibling BC-7.07.001 was NOT affected — its `modified:` array was already descending (fixed at its own v1.31, F-P29-003) and required no change; a full cluster parity audit across all three BC-4.17.001/BC-5.40.001/BC-7.07.001 confirmed BC-7.07.001's five POLICY 14 parity legs (version/Changelog-head/modified-head/last_amended-prefix), `inputs:` completeness, and §Story-Anchor↔§Traceability-§Stories cardinality are all already clean — no BC-7.07.001 edit required. Corrected: this BC's `modified:` array reordered to strict descending-chronological (newest at top) — this v1.13 entry, then v1.12, v1.11, ... down to v1.1 — now matching the Changelog table's existing order and BC-7.07.001's established pattern. Dated HISTORICAL entry text (v1.1 through v1.12) is unchanged — only array position was corrected, per POLICY 1 append-only numbering. No PC/Invariant/EC renumbered. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.12) — Pass-29 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 29 finding **F-P29-002 (MED, POLICY 18)**. Disposition: **F-P29-002 (MED)** — this BC makes load-bearing exact-code-body current-state claims (PC3's `now >= factory_lock.expires_at` comparison, cited verbatim against `crates/hook-plugins/verify-factory-lock/src/lib.rs::is_expired(now, expires_at) -> bool { now >= expires_at }`; the migrated Precondition 6/Invariant 7/Invariant 8/EC-010 `STATE_MD_MAX_BYTES`/`extract_frontmatter` claims against `crates/factory-lock-parse/src/lib.rs` and the now-dormant `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`; PC4's `factory_lock::renew_lock_if_holder`/`TTL_SECONDS` mid-burst-renewal claims against `crates/factory-lock/src/lib.rs`; and the `hooks-registry.toml` deregistration referenced throughout) but `inputs:` frontmatter listed none of the underlying code files — only ADRs, sibling BCs, BC-INDEX, and `state-burst/SKILL.md`. This is the same POLICY 18 sweep already applied to BC-7.07.001 (v1.29, F-P27-003) and BC-4.17.001 (v1.13, F-P28-001a) — BC-5.40.001 was never itself swept for this gap. Corrected: `crates/hook-plugins/verify-factory-lock/src/lib.rs`, `crates/factory-lock/src/lib.rs`, `crates/factory-lock-parse/src/lib.rs`, `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`, and `plugins/vsdd-factory/hooks-registry.toml` added to `inputs:`, in the same path form BC-4.17.001's and BC-7.07.001's arrays already use. This is NOT the accepted BC-4.17.001↔BC-7.07.001 mutual-inputs cyclic-hash TD (which concerns only that pair's mutual ADR/BC edges) — these are missing CODE inputs, legitimately in-scope and independently justified against this BC's own load-bearing claims. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); pure `inputs:` addition. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.11) — Pass-27 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 27 finding **F-P27-001 (HIGH, POLICY 4)**. Disposition: **F-P27-001 (HIGH)** — §Story Anchor's `Dual-story anchor` quantifier and story list were not swept when this BC's Traceability §Stories row was corrected at v1.10 (F-P25-002) to add a third story, S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; ADR-046 PC4 hook-authorship implementation) — §Story Anchor still listed only S-17.01 and S-19.08 under a stale `Dual-story anchor` label, a sibling-sweep straggler of the v1.10 fix. Corrected: §Story Anchor now lists all three stories (S-17.01, S-19.08, S-17.05) and the quantifier is corrected from `Dual-story anchor` to `Tri-story anchor`, matching the now-three-story count and BC-7.07.001's own §Stories row (which already lists both S-18.04a and S-17.05). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction to §Story Anchor's existing text only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.10) — Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding **F-P25-002 (MED)**. Disposition: **F-P25-002 (MED)** — Traceability §Stories row's `[pending]` placeholder for the ADR-046 `stamp-state-timestamp` PC4 hook-authorship implementation resolved to the confirmed implementing story: S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; `.factory/stories/S-17.05-stamp-state-timestamp-hook.md`), verified present in STORY-INDEX.md. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction to the Traceability §Stories row's existing text only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.9) — Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED + 2 LOW total across the ADR-046 BC cluster, all sibling-sweep-straggler class; this BC's cited finding: F-P6-002). Disposition: **F-P6-002 (MED)** — PC3's 'Failure mode — long burst TTL self-eviction' sub-paragraph still read `now > expires_at` (strictly-greater), a straggler of the same guard-boundary comparison PC3's own normative statement above already corrected to `now >= expires_at` at v1.7 (F-P4-001) — swept to `now >=` here so the sub-paragraph matches PC3's own corrected statement and PC4 condition (c). **Comprehensive sibling-sweep (in-scope, production-grade default):** a full grep sweep of this BC for every occurrence of the canonical values ADR-046 governs (expiry boundary, `STATE_MD_MAX_BYTES` sourcing, malformed-arm disposition, 5-case numbering, shared-fn homes, TTL cast, event fields, POLICY 19 pins, per-arm fail-open) found and fixed 5 additional stray `ADR-046 vN.N` version-pin stragglers in body prose (the 'verify-state-timestamp-refresh read capability' Precondition annotation and the Architecture Anchors bullet for that same crate) — both stripped to the stable `ADR-046 §Decision N` anchor form per POLICY 19 anti-volatile-pin (mirroring O-P4-002's established pattern); all other categories swept clean (zero additional stragglers). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); all corrections are in-place. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.8) — Pass-5 architecture-routed remediation mirror (product-owner), responding to adversarial spec review pass 5 (ADR-046 flipped v1.5→v1.6 via architect-routed remediation of the same pass; product-owner mirrors ADR-046 v1.6's corrections here in a sequential follow-up burst per D-386 sequencing — strict isolation maintained across BC-4.17.001/BC-5.40.001/BC-7.07.001, no ADR/registry/hook-source/STATE.md/ARCH-INDEX content touched). Disposition: **F-P5-001 (HIGH)** — the migrated-Precondition-6 annotation's `STATE_MD_MAX_BYTES` sourcing statement (\"the same `STATE_MD_MAX_BYTES = 262144` cap ... reused (not re-declared) — now lives at BC-4.17.001's Precondition 4\") never named the constant's actual home, and its unnamed pre-v1.6 declaration site was the very `verify-state-timestamp-refresh` crate ADR-046 §Decision 5 deregisters and anticipates eventually deleting. Corrected by mirroring ADR-046 §Decision 5 v1.6 / F-P5-001's exact sourcing text verbatim into this Precondition's annotation: it now references `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (`pub const STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to the `factory-lock-parse` crate) — not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate. **O-P5-001 (LOW)** — the Architecture Anchors bullet for `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` still called the crate an \"S-19.08 implementation target\" and cited it as the home of `STATE_MD_MAX_BYTES`; annotated MIGRATED/DEREGISTERED consistently with the sibling Precondition 6/Invariant 7/Invariant 8/EC-010 annotations already applied at v1.7, and the constant's home reference corrected to `factory-lock-parse`. No PC/Invariant/EC renumbered or content deleted (append-only numbering preserved — POLICY 1); both are in-place corrections of existing annotation text, not new additions. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.7) — Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 (ADR-046 flipped v1.4→v1.5 via architect-routed remediation of the same pass; product-owner applies ADR-046 v1.5's expanded guidance here in a sequential follow-up burst per D-386 sequencing — strict isolation maintained across BC-4.17.001/BC-5.40.001/BC-7.07.001, no ADR/registry/hook-source/STATE.md/ARCH-INDEX content touched). Disposition: **F-P4-001 (MED, real bug)** — PC3's guard-boundary statement corrected from `now > factory_lock.expires_at` (strictly-greater) to `now >= factory_lock.expires_at`, matching this BC's OWN PC4 condition (c) (`now < expires_at` for 'not yet expired'), ADR-046's ground-truth citation of `verify-factory-lock` PC2 (`LockExpired`), and the actual code (`crates/hook-plugins/verify-factory-lock/src/lib.rs::is_expired(now, expires_at) -> bool { now >= expires_at }`) — the prior `now >` wording was internally self-contradictory with this BC's own PC4 and factually wrong against ground truth. **F-P4-002 (MED)** — applies ADR-046 v1.5 §Decision 5's per-element reconciliation table for the guard-read contract this BC specified on behalf of the now-deregistered `verify-state-timestamp-refresh` guard: Precondition 6, Invariant 7, Invariant 8, and EC-010 are RETAINED (not deleted, POLICY 1 append-only) but annotated MIGRATED-to-BC-4.17.001 (Precondition 4/Invariant 7/Invariant 8/EC-015 there), since `stamp-state-timestamp`'s shared `host::read_file` call is the identical read-hazard class these elements existed to bound and is now the actual production call site; the S-19.08 Verification Properties rows T-001..T-007 are annotated RETAINED AS HISTORICAL/DORMANT (the crate they test remains in-tree and its tests still pass, per Decision 5's crate-retention clause — not deleted), with BC-4.17.001 gaining its own new, equivalent VP-TBD-7/8/9 rows for the migrated guarantees. No PC/Invariant/EC content deleted or renumbered (append-only numbering preserved — POLICY 1); the migrated elements are annotated superseded-by-migration in place, per ADR-046's own File-Change Plan row for this BC. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-26 (v1.6) — Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 PRODUCT-OWNER-ROUTED findings F-002/F-003 against ADR-025/ADR-046 (ADR-046 now v1.2). Disposition: **F-002 (HIGH, POLICY 19 adr_version_cite_volatile_pin_prohibition)** — the Traceability § ADR Reference row carried load-bearing `ADR-025 v1.2` and `ADR-046 v1.1` version pins, both POLICY 19 violations and both already stale (ADR-025 is v1.2 only coincidentally current but the pin itself is the violation; ADR-046 is now v1.2); both are stripped to the stable `ADR-025 §Decision 2/3/5/8/10` and `ADR-046 §Decision 1(b)` anchor forms — no version token remains in that row. **F-003 (HIGH)** — the v1.5 last_amended/Changelog 'Flagged for architect: ADR-046's own Decision 1(b) text does not yet enumerate this AlreadyExpired case' note is now RESOLVED: ADR-046 v1.2 §Decision 1(b) already enumerates `AlreadyExpired` as the third `SkipReason` variant, consistent with this BC's own PC4/Invariant 1 AlreadyExpired disposition (added at v1.5) — no architect action remains outstanding on this item; the v1.5 historical narrative below and the v1.5 Changelog row are annotated with this resolution rather than left asserting a stale open flag. No PC/Invariant/EC content otherwise changed; no renumbering (append-only numbering preserved — POLICY 1). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). [Prior: 2026-08-25 (v1.5) — ADR-046 v1.1 Companion Amendment 1 (i)-(v) (product-owner; pass-2 spec remediation, responding to ADR-046's flip from v1.0 to v1.1 accepted): the v1.4 amendment (below) reflected only ADR-046 v1.0's PC4 actor reassignment. This pass adds the five v1.1-specific obligations ADR-046's Companion Amendments §item 1 states the BC 'MUST also state': **(i)** Invariant 1's 'sole writer' claim is qualified modulo email collision — two sessions sharing one git email are indistinguishable to the hook's holder-identity check, exactly as they are to `verify-factory-lock`'s own PC3 self-held comparison (ADR-046 F-010); this is a pre-existing property of the email-keyed identity model, not a new defect. **(ii)** PC4's renewal comparison predicate is `>=` (not strictly-greater) at second-precision wall-clock resolution — a same-wall-clock-second re-invocation produces a byte-identical `expires_at`, treated as a no-op-equivalent success; 'idempotent' describes ONLY this byte-identical-suppression behavior on the `expires_at` arm (ADR-046 F-007). **(iii)** a malformed/unparseable `expires_at` is NEVER repaired by the renewing hook — `verify-factory-lock` treats a malformed block as unlocked (fail-open, admits any caller), so a repair would silently re-materialize a lock under a session the guard just treated as free (ADR-046 F-008; matches BC-4.17.001 PC2 step 3 and BC-7.07.001's amended Invariant 3). **(iv)** `TTL_SECONDS` is sourced from the new canonical `factory_lock::TTL_SECONDS` const (ADR-046 F-006), not a BC-local literal — the 2700-second value itself (Invariant 2/AC-007) is UNCHANGED, only its source-of-truth citation is added. **(v)** the Canonical Test Vectors table gains a truth-table row set covering `{Resolved+Match, Resolved+Mismatch, Failed}` x `{stamp-state-timestamp, precompact-flush}` so both automatic-renewal call sites' identical behavior is spec-visible here, not only in BC-4.17.001/BC-7.07.001. Also added for consistency with the two hook BCs' F-009 disposition (not itself one of ADR-046's five enumerated items, but required for PC4 to remain internally consistent with its own enforcing hooks): an already-expired self-held lock is NOT resurrected by either automatic-renewal hook — the expiry precheck runs before identity resolution is attempted. (RESOLVED at v1.6 — ADR-046 v1.2 §Decision 1(b) now enumerates this `AlreadyExpired` case; no architect action remains outstanding). No PC/Invariant number renumbered; all additions are in-place amendments to PC4/Invariant 1/Canonical Test Vectors. BC-5.40.001 v1.4→v1.5. [Prior: 2026-08-25 (v1.4) — ADR-046 ratification amendment (product-owner; PC4 actor reassignment): PC4 (\"Mid-burst TTL renewal\") actor reassigned from state-manager (manual) to the `stamp-state-timestamp` PostToolUse hook (automatic, identity-gated). Invariant 1 (\"state-manager is the sole writer\") amended to carve out the mid-burst expires_at keep-alive as the hook's mechanically-enforced exception (holder-identity check: writer git user.email == factory_lock.holder), operationalizing the invariant rather than relaxing it. `factory-lock-write.sh renew` retained as an explicit break-glass/manual fallback. TTL=2700s (Invariant 2/AC-007) UNCHANGED — only renewal authorship moved. Related BCs + Architecture Anchors updated to cite new sibling BC-4.17.001 (`stamp-state-timestamp` plugin contract). H1 title re-enriched per POLICY 7 (BC H1 Title Authority) to reflect hook authorship. [Prior: 2026-07-14 (v1.3) — F-P1-002 resolution (product-owner; post-merge burst; S-19.08 PR #646 squash 1304d280 2026-07-14): VP Anchors pending-placeholder replaced with definitive statement — S-19.08 verification delivered via BC-anchored unit/integration tests T-001..T-007 (no VP-NNN IDs assigned; per-story unit tests follow (unit-test) row convention); VP-096 reused by transitivity. [Prior: 2026-07-13 (v1.2) — S-19.08 Spec-First amendment (human-authorized; D-826/D-835): Precondition 6 added (verify-state-timestamp-refresh read capability: max_bytes=262144 (256 KiB); frontmatter-only via factory_lock_parse::extract_frontmatter (crates/factory-lock-parse/; S-19.02 PR #610; reuse-not-duplicate); cap mirrors BC-4.13.001 Phase-A Precondition 3 + ADR-025 §Decision 12 §12.5 parity; fail-open on OutputTooLarge per ADR-025 Decision 7). Invariant 7 added: frontmatter-only mandate for verify-state-timestamp-refresh (extract_frontmatter exclusive; mirrors BC-4.13.001 Invariant 9). Invariant 8 added: soft-warn threshold adjudication — verify-state-timestamp-refresh reads STATE.md in full → BC-4.13.001 Invariant 10 scope confirmed → state_md_approaching_cap MUST emit at bytes_read > 200000 AND ≤ 262144 (boundary table parity with BC-4.13.001 Invariant 10). EC-010 added (STATE.md exceeds 262144 bytes: OutputTooLarge → guard fail-open). Verification Properties updated: unit-test rows T-001..T-007 added; VP-096 back-cited (extract_frontmatter reuse). Story Anchor updated: S-17.01 + S-19.08. Traceability Stories updated: S-17.01 + S-19.08. Architecture Anchors: crates/factory-lock-parse/ added. [Prior: 2026-06-11 (v1.1) — POL-14 auto-promotion: lifecycle_status draft→active on PR #181 squash-merge c64b46d2 (S-17.01 merged); status draft→active; D-544 codified. [Prior: 2026-06-10 (v1.0) — Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md frontmatter schema, TTL auto-expiry, mid-burst renewal, state-burst CAS push fix.]]]]]]]]]]]]]]]]]]"
---

# BC-5.40.001: STATE.md MUST carry a factory_lock frontmatter block (holder, locked_at, expires_at) as the authoritative lock state, state-manager MUST be its sole writer (modulo email collision), TTL auto-expiry MUST be enforced at 45 minutes (TTL_SECONDS, canonical constant), the stamp-state-timestamp PostToolUse hook MUST renew expires_at (>= comparison) on every qualifying STATE.md write gated on writer-identity == holder AND the lock NOT already expired (never resurrecting an expired self-held lock, never repairing a malformed expires_at), the identical gate applies to precompact-flush's renewal (shared renew_lock_if_holder), and state-burst MUST use fetch-then-force-with-lease CAS push (state-manager's manual factory-lock-write.sh renew retained as break-glass fallback)

## Description

The `factory_lock` block in STATE.md frontmatter is the authoritative cross-session lock state
for the factory-artifacts orphan branch. It carries three fields: `holder` (git user email of
the locking session), `locked_at` (ISO-8601 acquisition timestamp), and `expires_at` (ISO-8601
expiry timestamp = `locked_at + 45min` initially; refreshed to `now + 45min` on each intermediate
burst commit). Absent or null block = unlocked. Malformed block (missing required fields or
unparseable values) = treated as unlocked (fail-open, consistent with BC-4.13.001 PC4). The
`state-manager` agent is the sole writer of this block's structural fields — `holder`,
`locked_at`, and the block's presence/absence via acquire/unlock (TD-VSDD-053 single-writer
discipline). **Per ADR-046 (ratified 2026-08-25):** the mid-burst `expires_at` keep-alive
renewal (PC4) is written automatically by the `stamp-state-timestamp` PostToolUse hook
(SS-04 Plugin Ecosystem; BC-4.17.001) after every qualifying `Edit`/`Write`/`MultiEdit` to
`.factory/STATE.md`, gated on the hook's own resolved writer identity
(`git config user.email`) being byte-equal to the recorded `holder` — never on state-manager's
own initiative. `state-manager` no longer manually invokes `factory-lock-write.sh renew` as
part of ordinary content edits; that manual mode is retained as an explicit break-glass
fallback (e.g., recovering a burst where the hook is unavailable).

This BC also specifies the replacement of the blind `git push origin factory-artifacts` in
`skills/state-burst/SKILL.md` with a fetch-then-`--force-with-lease` CAS push (ADR-025
Decision 8, deliverable D6). This is a standalone complementary mitigation: even without the
WASM guard (BC-4.13.001), the CAS push converts concurrent pushes from silent clobbers to
detected collisions. The `/factory-lock` acquire path uses the same CAS primitive (BC-6.23.001).

This BC covers ADR-025 Decisions 2, 3, 5, 8, and 10, and deliverables D3 and D6.

## Preconditions

### STATE.md structure

1. STATE.md MUST contain a YAML frontmatter region (bounded by `---\n` delimiters at lines 1
   and N). The `factory_lock` block lives within this frontmatter region as a YAML mapping key.
   The three required sub-fields are `holder`, `locked_at`, and `expires_at`.

2. `state-manager` is the sole agent permitted to write STATE.md (TD-VSDD-053). No other agent,
   skill, or tool may modify the `factory_lock` block directly.

### Lock state schema

3. When a lock is in force, the `factory_lock` block MUST have the following canonical YAML form
   in STATE.md frontmatter:
   ```yaml
   factory_lock:
     holder: "developer@example.com"   # git config user.email of the locking session
     locked_at: "2026-06-10T14:00:00Z" # ISO-8601; when /factory-lock was run
     expires_at: "2026-06-10T14:45:00Z" # ISO-8601; = locked_at + 45min (refreshed on mid-burst commits)
   ```
   All three fields MUST be present when a lock is in force. The `holder` field MUST be the
   exact string returned by `git config user.email` at acquire time. `locked_at` and
   `expires_at` MUST be ISO-8601 UTC timestamps (format: `YYYY-MM-DDTHH:MM:SSZ`).

4. When the factory is unlocked, the `factory_lock` block MUST be absent from STATE.md
   frontmatter entirely (no null placeholder, no empty mapping). `state-manager` removes the
   block on `/factory-unlock` (BC-6.23.001 PC4).

### state-burst CAS push precondition

5. Before the CAS push, `state-manager` MUST perform a `git fetch origin factory-artifacts`
   to synchronize the local `factory-artifacts` ref with the remote. The expected SHA is
   captured immediately after the fetch: `EXPECTED_SHA=$(git -C .factory rev-parse
   origin/factory-artifacts)`.

### verify-state-timestamp-refresh read capability

**[MIGRATED to BC-4.17.001 Precondition 4 — ADR-046 §Decision 5 reconciliation, F-P4-002;
sourcing corrected 2026-08-26 per ADR-046 §Decision 5 / F-P5-001.]**
`verify-state-timestamp-refresh` is deregistered from `hooks-registry.toml` per ADR-046 Decision
5; the read-capability contract below describes that guard's own (now-dormant) `host::read_file`
call. Its live enforcement now lives at BC-4.17.001's Precondition 4. **Sourcing (mirrored per
ADR-046's Companion Amendment 1 item (vi) correction, version pin stripped per POLICY 19
anti-volatile-pin, F-P6 sweep):** "Precondition 6
(`STATE_MD_MAX_BYTES` cap) MUST reference `factory_lock_parse::STATE_MD_MAX_BYTES` — the single
canonical declaration (`pub const STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to the
`factory-lock-parse` crate per ADR-046 §Decision 5 / F-P5-001) — not a locally re-declared
constant, and not the now-deregistered `verify-state-timestamp-refresh` crate (which no longer
declares it as of this correction)." Since `stamp-state-timestamp`'s shared
`host::read_file(".factory/STATE.md")` call is the identical read-hazard class this Precondition
existed to bound. This Precondition is RETAINED HERE, unmodified in substance (only its
constant-sourcing annotation corrected), as a historical/dormant record — the
`verify-state-timestamp-refresh` crate and its S-19.08 tests remain in-tree (not yet deleted, per
ADR-046 Decision 5's crate-retention clause; per the same Decision's File-Change Plan, the
crate's own local `STATE_MD_MAX_BYTES` declaration is removed and its call sites repointed to
import `factory_lock_parse::STATE_MD_MAX_BYTES`), so the text below stays a factually true
description of that crate's own (now-relocated-constant) contract; it is not deleted per POLICY 1
append-only numbering.

6. **Phase-A (active spec; implemented by S-19.08):** The `verify-state-timestamp-refresh`
   guard reads `.factory/STATE.md` via `host::read_file` with `max_bytes = 262144` (256 KiB).
   The plugin-side compile-time cap MUST be `STATE_MD_MAX_BYTES = 262144`. Before any YAML
   field extraction (`timestamp:` or `factory_lock.expires_at`), the guard MUST call
   `factory_lock_parse::extract_frontmatter(bytes)` (available in `crates/factory-lock-parse/`
   via S-19.02 PR #610 — reuse, not reimplementation; see Invariant 7) and operate exclusively
   on the returned frontmatter slice. The 256 KiB cap is established by `ADR-025 §Decision 14`
   ("verify-factory-lock read-cap 262144 + frontmatter-only parse"; corrected 2026-08-26,
   F-P35-001 — §Decision 12 §12.5 is "Shared parse logic — no duplication," the
   `factory-lock-parse` crate-extraction decision, and states no byte-cap value anywhere in its
   text; §Decision 14 is the decision that actually raised the cap from 65536 to 262144, and its
   own "Normative twin" line names `BC-4.13.001 §Precondition 3 (Phase-A)`) parity with
   `verify-factory-lock` (BC-4.13.001 Phase-A Precondition 3); this cap is
   above the worst-case observed STATE.md size (<200 KiB under 500-line compaction discipline
   per D-442(e)), giving ≥25% headroom. When `host::read_file` returns `OutputTooLarge` (file
   exceeds cap), the guard MUST fall back to `HookResult::Continue` (fail-open per ADR-025
   Decision 7; see EC-010). The soft-warn threshold contract (Invariant 8) MUST be observed on
   every successful read.

   **Defect context:** Without this Precondition, `read_file.rs::read_bounded()` checks
   `metadata.len()` BEFORE reading any bytes: when STATE.md exceeds the 64 KiB legacy cap
   (`STATE_MD_MAX_BYTES = 65536`), the host returns `OUTPUT_TOO_LARGE (-3)` before the guard
   reads a single byte, causing the timestamp-freshness gate to fail open silently on every
   PreToolUse dispatch to STATE.md — the same defect class as S-19.02 FINDING-1 for
   `verify-factory-lock`. D-826/D-835 confirm 3× production occurrences of this failure mode.

## Postconditions

### PC1 — Factory lock schema correctness

When `state-manager` writes a lock (via the `/factory-lock` acquire skill — BC-6.23.001 PC1),
the resulting STATE.md frontmatter MUST contain a well-formed `factory_lock` block with:
- `holder`: the exact output of `git config user.email` at acquire time (no trimming beyond
  trailing newline removal)
- `locked_at`: ISO-8601 UTC timestamp of the acquire instant (precision: seconds)
- `expires_at`: `locked_at + 45 minutes` (exactly 2700 seconds added to `locked_at`)

**Error variant:** `SchemaViolation` (if any field is missing or malformed after write — detected
by BC-4.13.001's PC4 malformed-block fail-open path)

### PC2 — Unlock clears the block

When `state-manager` writes an unlock (via `/factory-unlock` — BC-6.23.001 PC4), the
`factory_lock` key MUST be absent from STATE.md frontmatter entirely after the write. A null
value (`factory_lock: null`) is NOT an acceptable unlock representation — the key must be
removed.

**Error variant:** `StaleNullBlock` (if key remains as null post-unlock)

### PC3 — TTL auto-expiry: guard treats expired lock as absent

The `verify-factory-lock` guard (BC-4.13.001 PC2) checks `now >= factory_lock.expires_at`
(F-P4-001, corrected 2026-08-26) at invocation time — the exact-expiry instant `now ==
expires_at` IS treated as expired, matching this BC's own PC4 condition (c) (`now < expires_at`
for "not yet expired") and the guard's actual implementation
(`crates/hook-plugins/verify-factory-lock/src/lib.rs::is_expired(now, expires_at) -> bool { now
>= expires_at }`). The prior `now >` (strictly-greater) wording was internally inconsistent with
this BC's own PC4 and with the ground-truth code; it is corrected here, not renumbered
(POLICY 1). When true, the lock is treated as absent and the operation proceeds. The expired
lock block remains in STATE.md frontmatter until the next `state-manager` write (which either
refreshes it if the session is still active, or removes it at explicit unlock). This "stale
expired block" state is safe: the guard's TTL check treats it as unlocked, and the next
`state-manager` commit cleans it up.

**Failure mode — long burst TTL self-eviction (ADR-025 Decision 5):**
A burst longer than 45 minutes between intermediate commits self-evicts the lock: `now >=
expires_at` becomes true mid-burst (corrected 2026-08-26, F-P6-002 — matching this same PC3's
own corrected guard-boundary statement above and PC4 condition (c)), allowing another developer
to acquire. Mitigation: mid-burst renewal (PC4). Residual risk: fencing token absent — see
Invariant 4.

### PC4 — Mid-burst TTL renewal (hook-authored; ADR-046)

**Actor (amended 2026-08-25, ADR-046):** the `stamp-state-timestamp` PostToolUse hook
(SS-04 Plugin Ecosystem; BC-4.17.001) — NOT `state-manager` directly — performs this
renewal, via the SHARED `factory_lock::renew_lock_if_holder` function (ADR-046 Decision
1(b)) also used by `precompact-flush`'s amended `PreCompact` renewal (BC-7.07.001). At EVERY
`Edit`/`Write`/`MultiEdit` that lands a write to `.factory/STATE.md` within a burst (not only
at burst-close), the hook fires after the write and:
- Refreshes `factory_lock.expires_at = now + TTL_SECONDS` (2700 seconds, sourced from the
  canonical `factory_lock::TTL_SECONDS` const — ADR-046 F-006; see Invariant 2/Architecture
  Anchors) **if and only if** ALL of: (a) a `factory_lock` block is present in the post-write
  frontmatter with a non-empty `holder`; (b) `expires_at` is present and parses successfully;
  (c) the recorded lock is NOT already expired (`now < expires_at`, evaluated against the
  hook's own wall clock); and (d) the hook's own resolved writer identity (`git config
  user.email`, via `host::exec_subprocess` — the identical mechanism BC-4.13.001 already
  uses), after applying the canonical `trim_git_email` trim (ADR-046 Decision 2/F-004), is
  byte-equal to `holder`. **Comparison predicate (ADR-046 F-007):** the field's new value is
  compared `>=` (not strictly-greater) against its pre-invocation value — a same-wall-clock-
  second re-invocation produces a byte-identical `expires_at`, which is a no-op-equivalent
  success, not a violation; "idempotent" describes ONLY this byte-identical-suppression
  behavior on this arm (contrast BC-4.17.001's `timestamp:` arm, which is explicitly NOT
  idempotent). If any of (a)-(d) fails — including a non-holder writer legitimately admitted
  through BC-4.13.001 PC2 (`LockExpired`), or an already-expired self-held lock (condition
  (c)) — `expires_at` is left untouched (no renewal; a foreign, expired, OR
  already-expired-and-not-yet-reacquired holder's lock is never silently resurrected —
  condition (c) is evaluated BEFORE identity resolution is attempted, since an expired lock's
  outcome does not depend on who is asking). **Malformed `expires_at` is never repaired
  (ADR-046 F-008):** if `expires_at` is present but unparseable, the hook does NOT compute a
  fresh value to "fix" it — `verify-factory-lock` (BC-4.13.001 PC4) treats a malformed block
  as unlocked (fail-open, admits any caller), so a repair would silently re-materialize a
  lock under a session the guard just treated as free; the hook's only action is an advisory
  warning, no write.
- If no lock is held (block absent), no renewal occurs — there is nothing to keep alive.

This resets the TTL clock to 45 minutes from each qualifying write rather than from the
original `locked_at`. Because the hook fires immediately after the tool's own write and
before `state-manager`'s subsequent `git commit` of the same burst step, the renewed value
is captured in that commit — the renewal is atomic with the commit in effect, with no
separate background timer or subprocess required. `state-manager` no longer needs to
remember to author this refresh as part of any Commit-A/B/C/D/E payload; the obligation
that was previously a documentary "state-manager MUST renew" instruction is now a
mechanically-guaranteed property of every qualifying STATE.md write.

**Break-glass fallback:** `factory-lock-write.sh renew` (manual invocation) remains
available and is unchanged — e.g., for recovering a burst where the hook is unavailable or
disabled. It performs the same unconditional `expires_at = now + 2700s` update with no
identity check (as today); this is acceptable for the manual path because invoking it is
itself a conscious, deliberate act by whoever runs it.

**Error variant:** `RenewalMissed` (if a burst completes without `expires_at` advancing while
a lock is held by the writer — detectable by comparing old and new `expires_at` values
post-write; now diagnostic of a hook fail-open/unavailability condition rather than agent
forgetfulness)

### PC5 — state-burst fetch-then-CAS push

The `state-burst` skill MUST replace its blind `git push origin factory-artifacts` with the
following fetch-then-CAS push sequence:

```bash
git -C .factory fetch origin factory-artifacts
EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
```

On non-fast-forward rejection (push fails with a non-zero exit code indicating
`--force-with-lease` check failed), `state-burst` MUST:
1. Exit with a non-zero status.
2. Emit a human-readable error: "state-burst CAS push failed — concurrent write detected.
   Fetch origin/factory-artifacts and retry."
3. NOT silently clobber the remote state.

This is independently valuable whether or not the WASM guard is deployed: a concurrent push
from any source (another developer, a self-vs-self two-session case, a guard-crash fail-open
scenario) causes a detected collision rather than a silent clobber.

**Error variant:** `CASPushRejected` (non-fast-forward rejection from `--force-with-lease`)

### PC6 — Single-developer path: zero added friction

A developer who has run `/factory-lock` and holds the lock sees no friction during normal
`state-manager` burst operations. The guard reads STATE.md locally (no network calls in the
guard hot path per ADR-025 Decision 10), compares identities, and returns `Continue`
immediately on self-held lock (BC-4.13.001 PC3). The only observable changes are:
- The `factory_lock` block is present in STATE.md during the session.
- The `expires_at` field updates on each intermediate commit (renewal).
- `/factory-health` shows `Factory lock: HELD by this session (expires <time>)` (BC-6.23.001).

A developer who has NOT run `/factory-lock` is in the same position as today: the guard reads
`factory_lock: null` (absent) and passes all checks. The lock is opt-in.

## Invariants

1. **`state-manager` is the sole writer** (amended 2026-08-25, ADR-046): No other agent,
   skill, or tool writes the `factory_lock` block's structural fields (`holder`, `locked_at`,
   or the block's presence/absence). The `/factory-lock` and `/factory-unlock` skills
   DELEGATE to `state-manager` to write STATE.md (they do not write directly). This preserves
   TD-VSDD-053. **Mechanized exception (PC4):** the mid-burst `expires_at` keep-alive renewal
   is the one field this invariant no longer requires a human-directed `state-manager` act
   for — it is written by the `stamp-state-timestamp` PostToolUse hook (BC-4.17.001), gated
   on a holder-identity check (the hook's independently-resolved writer git email must equal
   the recorded `holder`). This is not a relaxation of the sole-writer invariant; it is the
   invariant's first *mechanical* enforcement at the renewal call site — previously the
   invariant was purely documentary there (nothing checked that only the holder renewed;
   see ADR-046 Rationale). A non-holder's write to STATE.md (e.g., a caller legitimately
   admitted through BC-4.13.001 PC2 `LockExpired`), or a write by the same holder after their
   lock has already expired (never resurrected — see PC4), never renews under this gate.
   **Scope note (added 2026-08-25, v1.5 — ADR-046 F-010):** this invariant's "sole writer"
   guarantee holds **modulo email collision** — the hook's identity check is an email-equality
   comparison, not a session-identity comparison. Two sessions authenticated under the same
   git email (e.g., two terminal tabs on the same laptop, or a CI runner and a human sharing a
   service account) are indistinguishable to this check, to `verify-factory-lock`'s own PC3
   self-held comparison, and to the amended `precompact-flush`'s identical check (BC-7.07.001)
   — this is a pre-existing property of the email-keyed identity model, not a defect this
   amendment introduces or is expected to close.

2. **Default TTL is 45 minutes (2700 seconds)**: The TTL value is not configurable by users.
   45 minutes is the production-grade default (ADR-025 Decision 5 rationale: midpoint of
   2–5× expected burst duration range; expected burst duration ~10 minutes).

3. **`expires_at` is always = `now + 2700s` at the moment of write**: Whether the write is the
   initial acquire, a mid-burst renewal, or any other state-manager write that refreshes the
   lock, `expires_at` is computed as the wall-clock instant of the commit + 2700 seconds.
   The `locked_at` field is immutable after the initial acquire — it records when the session
   started, not the last renewal.

4. **Fencing token absent — residual risk accepted**: The current design has no fencing token
   (monotonically increasing value that storage can check to reject stale-holder writes). If
   the TTL expires between two intermediate commits under extreme network delay or WASM fuel
   exhaustion, a second developer could acquire between renewals and both parties proceed.
   This residual risk is explicitly attributed to the Decision 9 git-ref-CAS future path
   (ADR-025 §Decision 9). Under the cooperative threat model, this is accepted.

5. **`--force-with-lease` is already permitted by `verify-git-push.sh`**: The bash hook
   `hooks/verify-git-push.sh` only blocks raw `--force`; `--force-with-lease` is permitted
   (ADR-025 Decision 8 source verification). The CAS push change requires no modifications
   to the existing push-hook allow-list.

6. **Malformed block = unlocked (fail-open)**: The guard (BC-4.13.001 PC4) and this BC both
   treat any malformed `factory_lock` block as unlocked. `state-manager` MUST write
   well-formed blocks; however, if STATE.md is corrupted (e.g., manual edit), the system
   fails open rather than wedging.

7. **[MIGRATED to BC-4.17.001 Invariant 7 — ADR-046 §Decision 5 reconciliation, F-P4-002;
   retained here as historical/dormant, POLICY 1 append-only.] `verify-state-timestamp-refresh`
   uses `extract_frontmatter` exclusively**: The guard MUST
   call `factory_lock_parse::extract_frontmatter(bytes)` on the byte slice returned by
   `host::read_file` before scanning for `timestamp:` or `factory_lock.expires_at`. The guard
   MUST operate only on the returned frontmatter slice and MUST NOT process the file body after
   the closing `---` delimiter. This mirrors BC-4.13.001 Invariant 9's frontmatter-only mandate
   applied to the `verify-state-timestamp-refresh` guard. The `extract_frontmatter` function is
   provided by `crates/factory-lock-parse/` (pure-core crate; S-19.02 PR #610;
   reuse-not-duplicate per CANONICAL PRINCIPLE Rule 4). Reimplementing the function in
   `crates/hook-plugins/verify-state-timestamp-refresh/` is a production blocker. When
   `extract_frontmatter` returns the full bytes (delimiter not found — fail-open behavior of
   the function), the guard applies its parse logic to the full returned slice without error.
   The `factory-lock-parse` crate is already a dependency of `verify-state-timestamp-refresh`
   (S-19.02 established the pattern); no new Cargo.toml dependencies are required.

8. **[MIGRATED to BC-4.17.001 Invariant 8 — ADR-046 §Decision 5 reconciliation, F-P4-002;
   retained here as historical/dormant, POLICY 1 append-only.] Soft-warn threshold for
   `verify-state-timestamp-refresh` (BC-4.13.001 Invariant 10
   adjudication)**: `soft_warn_threshold = 200000` bytes. **Adjudication:** BC-4.13.001
   Invariant 10 applies to "a hook that already reads STATE.md in full (i.e., calls
   `host::read_file` on `.factory/STATE.md`)." The `verify-state-timestamp-refresh` guard
   reads STATE.md on every PreToolUse Edit/Write/MultiEdit dispatch, placing it within
   Invariant 10's explicit scope. When a successful read observes
   `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)`, the guard MUST emit a
   `state_md_approaching_cap` diagnostic event carrying `bytes_read: u64` and
   `cap_bytes: u64` (262144). This event is observability-only — it NEVER triggers a block
   or alters the `Continue`/`Block` verdict. The soft-warn range is
   `bytes_read ∈ (200000, 262144]` — inclusive at the cap boundary:

   | `bytes_read` | Outcome |
   |---|---|
   | ≤ 200000 | No warn emitted; normal read |
   | 200001 | `state_md_approaching_cap` emitted; read succeeds |
   | 262144 | `state_md_approaching_cap` emitted AND read succeeds — warn MUST fire at cap boundary |
   | 262145 | `OutputTooLarge` returned by host; soft-warn path not reached |

   This event requires zero new registry entries. The threshold is not a hard cap; it is a
   leading indicator for compaction scheduling (D-442(e)).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `state-manager` crash mid-burst before writing `expires_at` renewal | Existing lock remains with old `expires_at`; if old expiry elapses, lock auto-expires; next developer can acquire; crashed session leaves a stale lock (max 45 min wedge) |
| EC-002 | `state-burst` CAS push rejected (non-fast-forward) | `state-burst` exits non-zero; actionable error emitted; `.factory/` commit already exists locally; developer must `git -C .factory fetch origin factory-artifacts` and re-run state-burst after resolving the divergence |
| EC-003 | `git fetch origin factory-artifacts` fails (network error) in state-burst CAS path | `state-burst` exits non-zero; actionable error: "fetch failed before CAS push"; do NOT proceed with push using potentially stale EXPECTED_SHA |
| EC-004 | `EXPECTED_SHA` fetch succeeds but remote advances before push (true concurrent write) | `--force-with-lease` rejects the push (non-fast-forward); `CASPushRejected` error; safe — no silent clobber |
| EC-005 | Lock held by self; `expires_at` within 5 minutes of current time | Mid-burst renewal MUST still refresh `expires_at = now + 2700s` on the next commit; the approaching expiry does not trigger any special behavior |
| EC-006 | STATE.md `factory_lock` block has `holder`, `locked_at`, `expires_at` but additional unknown fields | Unknown fields are ignored (fail-open to unlocked is NOT triggered; the block is valid if the three required fields are present and parseable) |
| EC-007 | Factory is unlocked (`factory_lock` absent); `state-burst` CAS push proceeds | Fetch + CAS push proceeds normally; if remote has advanced (another developer pushed), `CASPushRejected` error; developer fetches and retries |
| EC-008 | `git -C .factory rev-parse origin/factory-artifacts` returns a SHA that does not exist locally after fetch | This indicates a fetch/parse race; `state-burst` MUST re-fetch before retrying; emit `CASPushRejected` with "stale SHA after fetch" detail |
| EC-009 | Long burst: 3 intermediate commits, each refreshing `expires_at`; total burst duration = 70 min | Lock remains valid throughout: each commit resets `expires_at = now + 45min`; at burst-end, `expires_at` is 45 minutes in the future from the last commit |
| EC-010 | **[MIGRATED to BC-4.17.001 EC-015 — ADR-046 §Decision 5 reconciliation, F-P4-002; retained here as historical/dormant, POLICY 1 append-only.]** `verify-state-timestamp-refresh` guard reads STATE.md exceeding `max_bytes = 262144` (256 KiB) | `host::read_file` returns `OutputTooLarge`; guard falls back to `HookResult::Continue` (fail-open per ADR-025 Decision 7); `StateReadError` warn emitted. The 262144-byte cap exceeds D-442(e) structural limits (≤200 KiB under 500-line compaction discipline); exceedance indicates either compaction overdue or anomalous STATE.md inflation. Timestamp-freshness gate is silently inert for this invocation. |

## Canonical Test Vectors

| Scenario | STATE.md `factory_lock` before | Operation | STATE.md `factory_lock` after | Result |
|----------|-------------------------------|-----------|-------------------------------|--------|
| Lock acquire | absent | `/factory-lock` (via state-manager) | `{holder: "dev@x.com", locked_at: T, expires_at: T+2700s}` | Block written; push succeeds |
| Unlock (self) | `{holder: "dev@x.com", ..., expires_at: T+1h}` | `/factory-unlock` (self) | absent | Block removed; push succeeds |
| Mid-burst renewal | `{holder: "dev@x.com", ..., expires_at: T}` | state-manager intermediate commit | `{holder: "dev@x.com", locked_at: T_orig, expires_at: now+2700s}` | `expires_at` refreshed; `locked_at` unchanged |
| CAS push: concurrent write | N/A | `state-burst` push; remote advanced | N/A | `CASPushRejected`; error emitted; no clobber |
| Expired lock cleanup | `{holder: "dev@x.com", ..., expires_at: T-1s}` | Guard check at any mutating tool | Guard: `HookResult::Continue` (expired); block remains in STATE.md until next state-manager write | Safe pass-through |

**Renewal-gate truth table (added 2026-08-25, v1.5 — ADR-046 F-006 (v)):** both automatic
renewal call sites (`stamp-state-timestamp`, BC-4.17.001; `precompact-flush`, BC-7.07.001)
call the SAME shared `factory_lock::renew_lock_if_holder` function and therefore MUST exhibit
identical behavior for each outcome below:

| Call site | Identity outcome | `expires_at` result | Diagnostic |
|-----------|-------------------|----------------------|------------|
| `stamp-state-timestamp` | `Resolved+Match` (post-`trim_git_email` byte-equal to `holder`) | Renewed to `now + TTL_SECONDS` | none |
| `stamp-state-timestamp` | `Resolved+Mismatch` (`SkipReason::NotHolder`) | Unchanged | none (legitimate skip) |
| `stamp-state-timestamp` | `Failed` (`SkipReason::IdentityResolutionFailed` — subprocess error/timeout/empty stdout) | Unchanged | `factory.lock.renewal_indeterminate` event + `log_warn` |
| `precompact-flush` | `Resolved+Match` | Renewed to `now + TTL_SECONDS` | none |
| `precompact-flush` | `Resolved+Mismatch` (`SkipReason::NotHolder`) | Unchanged | none (legitimate skip) |
| `precompact-flush` | `Failed` (`SkipReason::IdentityResolutionFailed`) | Unchanged | `factory.lock.renewal_indeterminate` event + `log_warn` |

Both call sites additionally share the `SkipReason::AlreadyExpired` precheck (an already-expired
self-held lock is never resurrected by either hook — evaluated BEFORE identity resolution is
attempted) and the malformed-`expires_at` non-repair rule (advisory warn only, no write, no
identity resolution attempted).

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (unit-test) | `factory_lock` block written with correct schema on acquire | Rust unit test: call acquire; assert frontmatter has all three fields; assert `expires_at = locked_at + 2700s` |
| (unit-test) | Unlock removes `factory_lock` key entirely (not null) | Rust unit test: unlock; assert key absent from frontmatter |
| (unit-test) | Mid-burst renewal updates `expires_at` but preserves `locked_at` | Rust unit test: intermediate commit; assert `expires_at` advanced; `locked_at` unchanged |
| (unit-test) | `state-burst` CAS push rejects concurrent write | Rust unit test: mock remote advancing after fetch; assert non-zero exit + error message |
| (bats) | Bats integration: lock blocked when held by other developer | D9 T-2 (BC-4.13.001 canonical test vectors) |
| (bats) | Bats integration: acquire CAS rejection on concurrent acquire | D9 T-10 (BC-6.23.001 canonical test vectors) |
| (unit-test) | **[RETAINED AS HISTORICAL/DORMANT — ADR-046 §Decision 5, F-P4-002: the `verify-state-timestamp-refresh` crate is deregistered from `hooks-registry.toml` but retained in-tree per Decision 5's crate-retention clause, so this row remains a factually true statement about existing, passing tests against code that no longer executes in production; not deleted per POLICY 1. BC-4.17.001 carries its own new, equivalent VP-TBD-7 for the migrated guarantee.]** `STATE_MD_MAX_BYTES` constant equals 262144 in `verify-state-timestamp-refresh` | Rust unit test: `assert_eq!(STATE_MD_MAX_BYTES, 262144)` (S-19.08 T-001; AC-001) |
| (unit-test) | **[HISTORICAL/DORMANT — see T-001 annotation above.]** Guard reads STATE.md successfully when 64 KiB < file size < 256 KiB; detects stale timestamp and returns block intent | Rust unit test: 70 KiB fixture + stale `timestamp:` → `TimestampStale`; advanced timestamp → `Continue` (S-19.08 T-002/T-003; AC-002) |
| (unit-test) | **[HISTORICAL/DORMANT — see T-001 annotation above. BC-4.17.001 carries its own new, equivalent VP-TBD-8 for the migrated guarantee.]** `extract_frontmatter` wired before parse; body content excluded from parsed slice; no-delimiter fallback returns full content | Rust unit test: fixture with body after `---`; assert guard processes frontmatter only; delimiter-absent fixture → full content without error (S-19.08 T-004/T-005; AC-003) |
| (integration) | **[HISTORICAL/DORMANT — see T-001 annotation above.]** Zero `output_too_large` events emitted for `verify-state-timestamp-refresh` on 70 KiB STATE.md | Integration test: 70 KiB fixture; captured event stream asserts zero `internal.capability_denied reason=output_too_large` (S-19.08 T-006; AC-004) |
| (unit-test) | **[HISTORICAL/DORMANT — see T-001 annotation above. BC-4.17.001 carries its own new, equivalent VP-TBD-9 for the migrated guarantee.]** `state_md_approaching_cap` warn at bytes_read > 200000 ≤ 262144; no warn at ≤ 200000 (strict threshold); warn+read-success at cap-exact 262144; `StateReadError`+zero-warn at 262145 | Rust unit tests A/B/C/D/E (S-19.08 T-007; AC-005) |
| VP-096 | `extract_frontmatter` purity — output byte-equals file prefix up to (excluding) the second `---` delimiter line (bytes 0..delimiter_start_offset); deterministic for any input | proptest (S-19.02; `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs`); applies to `verify-state-timestamp-refresh` Invariant 7 use by transitivity — reuse of same function, same correctness guarantee |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — this BC defines the authoritative lock state data structure and the state-manager write discipline that underpins the entire CAP-031 mechanism. The `factory_lock` frontmatter schema is what the guard (BC-4.13.001) reads and what the skills (BC-6.23.001) manage; without a correct schema and renewal discipline, the guard cannot enforce exclusivity. |
| L2 Domain Invariants | none (operational infrastructure invariant, not L2 domain spec) |
| Architecture Module | `.factory/STATE.md` (frontmatter schema); `plugins/vsdd-factory/skills/state-burst/SKILL.md` (CAS push replacement D6); `agents/state-manager.md` (sole writer discipline); `plugins/vsdd-factory/hooks/verify-git-push.sh` (allows `--force-with-lease` — no change required) |
| Stories | S-17.01 (initial implementation; PR #181 merged 2026-06-11; D-544; v1.0-brownfield-backfill); S-19.08 (`verify-state-timestamp-refresh` read-cap amendment; implements Precondition 6, Invariants 7 and 8; E-19 Wave-2; D-826/D-835); S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; ADR-046 PC4 hook-authorship implementation — confirmed implementing story, F-P25-002, corrected 2026-08-26) |
| ADR Reference | ADR-025 §Decision 2/3/5/8/10 and deliverables D3, D6; ADR-046 §Decision 1(b) (PC4 actor reassignment — mid-burst `expires_at` renewal moves from state-manager to `stamp-state-timestamp` PostToolUse hook, identity-and-expiry-gated via the shared `renew_lock_if_holder`; also the renewal call site for the amended `precompact-flush`, BC-7.07.001; ratified 2026-08-25) |

## Related BCs

- BC-4.13.001 — depends on (the guard reads the schema defined here; PC4 malformed-block semantics mirror this BC's Invariant 6)
- BC-6.23.001 — composes with (the skills write the schema defined here; acquire/unlock operations produce the pre/postconditions defined in this BC)
- BC-5.39.009 — sibling (STATE.md mutation discipline; state-manager Commit-E cadence; the renewal heartbeat for `expires_at` follows the same state-manager burst commit discipline)
- BC-4.17.001 — depends on (ADR-046; the `stamp-state-timestamp` PostToolUse hook mechanically performs this BC's PC4 mid-burst `expires_at` renewal and the `timestamp:` re-stamp, on state-manager's behalf, identity-and-expiry-gated to the recorded `holder`)
- BC-7.07.001 — depends on (ADR-046 Decision 3, added 2026-08-25, v1.5; the amended `precompact-flush` PreCompact renewal is the SECOND automatic call site of the same `renew_lock_if_holder` function that performs this BC's PC4 renewal — both hooks MUST exhibit identical behavior per the Canonical Test Vectors truth table)

## Architecture Anchors

- `plugins/vsdd-factory/skills/state-burst/SKILL.md` — `git push origin factory-artifacts` (blind push; must be replaced with fetch-then-CAS; D6 target)
- `plugins/vsdd-factory/hooks/verify-git-push.sh` — allows `--force-with-lease` (no changes required; confirmed by ADR-025 §Decision 8)
- `crates/hook-plugins/stamp-state-timestamp/` — PostToolUse hook that performs PC4's mid-burst `expires_at` renewal (identity-and-expiry-gated) and the unconditional `timestamp:` re-stamp (ADR-046; BC-4.17.001)
- `crates/factory-lock/src/lib.rs` — canonical `TTL_SECONDS: u32 = 2700` const (added 2026-08-25, v1.5 — ADR-046 F-006; sourced by PC4's renewal computation) and the shared `renew_lock_if_holder`/`IdentityResolution`/`SkipReason` types both automatic-renewal hooks call (BC-4.17.001, BC-7.07.001)
- `.factory/STATE.md` — frontmatter region; `factory_lock:` block (new schema field)
- `crates/factory-lock-parse/src/lib.rs` — `extract_frontmatter` pure-core function (S-19.02 PR #610; reuse by `verify-state-timestamp-refresh` per Invariant 7; no modifications permitted by this story)
- `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` — **[MIGRATED/DEREGISTERED — ADR-046 §Decision 5, F-P4-002; constant home corrected 2026-08-26 per ADR-046 §Decision 5 / F-P5-001, O-P5-001; version pins stripped per POLICY 19 anti-volatile-pin, F-P6 sweep.]** `extract_frontmatter` call site; soft-warn emission (S-19.08 historical implementation; guard now deregistered from `hooks-registry.toml` per ADR-046 Decision 5, crate retained-in-tree-but-dormant). This crate no longer declares its own `STATE_MD_MAX_BYTES` constant — per ADR-046's File-Change Plan, the crate's local declaration is removed and its call sites (the `host::read_file` cap argument and the soft-warn comparison) are repointed to import the single canonical declaration, `factory_lock_parse::STATE_MD_MAX_BYTES: u32 = 262144`, from `crates/factory-lock-parse/src/lib.rs` — the same crate this line already cites for `extract_frontmatter`. `stamp-state-timestamp` (BC-4.17.001 Precondition 4) is the live production consumer of that same relocated constant.
- `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — authoritative design (§Decision 14 cap parity, corrected 2026-08-26 F-P35-001 — was mis-cited as §Decision 12 §12.5, the "Shared parse logic — no duplication" decision, which states no byte-cap value; §Decision 7 fail-open confirmed correct, unchanged — "Crash behavior — `on_error = \"continue\"` (fail-open)")

## Story Anchor

Tri-story anchor: S-17.01 (initial implementation; `factory_lock` schema + state-burst CAS push; PR #181 merged 2026-06-11; D-544; v1.0-brownfield-backfill); S-19.08 (`verify-state-timestamp-refresh` read-cap amendment; implements Precondition 6, Invariants 7 and 8; E-19 Wave-2; D-826/D-835 tracked defect); S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`; ADR-046 PC4 hook-authorship implementation — confirmed implementing story, F-P27-001, corrected 2026-08-26).

## VP Anchors

- VP-096 — `extract_frontmatter` Purity — Output Byte-Equals File Prefix Up To (Excluding) the Second `---` Delimiter Line (bytes 0..delimiter_start_offset; opening `---\n` included); Deterministic for Any Input (proptest; S-19.02; `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs`); back-cited per Invariant 7 reuse obligation — `verify-state-timestamp-refresh` calls the same pure function; VP-096 covers its correctness by transitivity.
- (S-19.08 unit/integration tests T-001..T-007) — **[RETAINED AS HISTORICAL/DORMANT — ADR-046
  §Decision 5 reconciliation, F-P4-002. `verify-state-timestamp-refresh` is deregistered
  from `hooks-registry.toml`, so the code path these tests verify no longer executes in
  production; the crate and its tests remain in-tree (not deleted, per Decision 5's
  crate-retention clause) and still pass, so this bullet stays a factually true record — not
  deleted, per POLICY 1 append-only numbering. BC-4.17.001 gains its own new, equivalent
  VP-TBD-7/8/9 rows for the migrated guarantees (`STATE_MD_MAX_BYTES` cap, `extract_frontmatter`
  wiring, `state_md_approaching_cap` soft-warn) against `stamp-state-timestamp` itself.]**
  S-19.08 verification is delivered via BC-anchored unit and integration tests following the
  `(unit-test)` / `(integration)` convention used throughout this BC. No VP-NNN IDs are assigned
  to per-story unit tests (established pattern; VP-096 is the load-bearing catalogued VP for the
  shared `extract_frontmatter` function, already active from S-19.02 and reused by Invariant 7
  transitivity). Shipped test function names verified from commit 1304d280 (`origin/develop`,
  PR #646, 2026-07-14):

  ```
  # crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs
  # grep -E "fn (test_BC_5_40_001_T[0-9]+)" src/lib.rs
  fn test_BC_5_40_001_T001_state_md_max_bytes_is_262144()
  fn test_BC_5_40_001_T002_70kib_fixture_stale_timestamp_blocks()
  fn test_BC_5_40_001_T003_70kib_fixture_advanced_timestamp_continues()
  fn test_BC_5_40_001_T004_extract_frontmatter_wired_body_bytes_excluded()
  fn test_BC_5_40_001_T005_no_delimiter_full_content_fail_open()
  fn test_BC_5_40_001_T007_state_md_approaching_cap_warn_boundary()

  # crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs
  # grep -E "^fn t006_" tests/integration_t006_no_output_too_large.rs
  fn t006_zero_output_too_large_on_70kib_state_md()
  fn t006_companion_advanced_timestamp_70kib_continues()
  ```

  T-001 covers `STATE_MD_MAX_BYTES = 262144` constant assertion (AC-001). T-002/T-003 cover 70 KiB fixture guard-operational (AC-002). T-004/T-005 cover `extract_frontmatter` wiring and no-delimiter fail-open (AC-003). T-006 is the integration cap-enforcement test for zero `output_too_large` on a 70 KiB file (AC-004). T-007 covers `state_md_approaching_cap` soft-warn boundary sub-tests A–E (AC-005). VP-096 (`extract_frontmatter` purity proptest) covers Invariant 7 by transitivity — the same pure function, same correctness guarantee.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.18 | 2026-08-27 | Pass-44 remediation (product-owner), responding to adversarial spec-convergence pass 44 finding **O-P44-001 (LOW, POLICY 4/5)**. The v1.17 `last_amended` disposition prose misattributed an illustrative "verbatim quote" of CAP-031's description — it quoted this BC's OWN Capability Anchor Justification prose ('this BC defines the authoritative lock state data structure...') as if it were CAP-031's description text, when that phrase is this BC's own justification sentence, not any part of CAP-031's description. Ground truth (capabilities.md §CAP-031): the capability's actual verbatim description opens "Enforce single-writer cross-session exclusivity on factory-artifacts state." Corrected the v1.17 disposition's illustrative parenthetical to CAP-031's actual verbatim text. **Sibling-parity check:** BC-4.17.001 v1.20's illustrative quote ("TTL is 45 minutes with mid-burst renewal") and BC-7.07.001 v1.34's illustrative quote (CAP-032's title) were each independently verified against `capabilities.md` — both confirmed CORRECT, no edit required. `capabilities.md` remains correctly listed in `inputs:` (unchanged — the file IS load-bearing); only the illustrative quote-snippet in the dated v1.17 historical prose is corrected. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.17 | 2026-08-27 | Pass-43 remediation (product-owner), responding to adversarial spec-convergence pass 43 finding **F-P43-001 (MED, POLICY 18)** plus a mandatory grep-complete inputs audit. **F-P43-001** — this BC's Capability Anchor Justification quotes CAP-031's description verbatim against `.factory/specs/domain-spec/capabilities.md`, absent from `inputs:` — added. **Grep-complete inputs audit** — every `.factory/specs/*.md`/`crates/*.rs`/`plugins/*.{sh,toml}`/`capabilities.md`/`invariants.md`/`prd.md`/`domain-spec` citation in this BC's body was enumerated against `inputs:`. Three more genuinely-cited-and-missing files found: `plugins/vsdd-factory/bin/factory-lock-write.sh` and `plugins/vsdd-factory/hooks/verify-git-push.sh` (both cited with specific current-behavior claims in Postcondition 4/Precondition 5), and `crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs` (quoted verbatim in the §VP Anchors grep-evidence block, distinct from the already-listed `src/lib.rs`). All three added. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.16 | 2026-08-26 | Pass-37 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 37 finding **F-P37-001 (MED, POLICY 4)**. Corrected ADR-046 Decision-count 1–5→1–6 in the v1.15 modified/last_amended/Changelog entries' ADR §Decision anchor audit prose (flat `## Decision` list has 6 items, not 5; item 6 = same-release ship + CI-gating registry-invariant XOR check). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.15 | 2026-08-26 | Pass-35 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 35 finding **F-P35-001 (HIGH, POLICY 4)**, plus a mandatory comprehensive ADR §Decision anchor audit. Two loci cited `ADR-025 §Decision 12 §12.5` for the 256 KiB `STATE_MD_MAX_BYTES` read-cap — Precondition 6's sourcing sentence and the Architecture Anchors ADR-025 bullet's "cap parity" clause. Verified against ADR-025: §Decision 12 §12.5 is "Shared parse logic — no duplication" (the `factory-lock-parse` crate-extraction decision; states no byte-cap value). The decision that actually raised the cap 65536 → 262144 is §Decision 14 ("verify-factory-lock read-cap 262144 + frontmatter-only parse"), whose "Normative twin" line names `BC-4.13.001 §Precondition 3 (Phase-A)` — the same BC both loci already cross-cite. Corrected both to `ADR-025 §Decision 14`. The Architecture Anchors bullet's separate `§Decision 7 fail-open` clause was independently verified correct (ADR-025 §Decision 7 = "Crash behavior — `on_error = \"continue\"` (fail-open)") and left unchanged. **Comprehensive ADR §Decision anchor audit:** every `ADR-NNN §Decision N` citation in this BC checked against the cited ADR's actual section content (ADR-025 §Decision 2/3/5/7/8/9/10/12/14; ADR-046's flat 1–6 `## Decision` list) — all confirmed CORRECT beyond the two F-P35-001 defects. `inputs:` re-audited: ADR-025 already present (no gap here). No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1). input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.14 | 2026-08-26 | Pass-31 spec-convergence remediation + comprehensive cross-anchor/spec-inputs audit (product-owner), responding to adversarial spec-convergence pass 31 finding **F-P31-001 (MED, POLICY 18)**. `inputs:` frontmatter gap closed: this BC cites BC-4.13.001 (PC2/PC3/PC4/Invariant 9/Invariant 10) and BC-6.23.001 (PC1/PC4) as load-bearing current-state authorities but listed neither in `inputs:`; both sibling BCs (BC-4.17.001, BC-7.07.001) and ADR-046 already list both. Added `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` and `.factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md`, same path form the siblings already use — NOT the accepted BC-4.17.001↔BC-7.07.001 mutual-inputs cyclic-hash TD (BC-4.13.001/BC-6.23.001 are outside that mutual set). **Comprehensive cross-anchor audit (mandatory, in-scope):** every `BC-X.YY.ZZZ §Section`/`PCn`/`Invariant N` cross-reference in this BC's body was verified against the cited BC's actual section content. Found and fixed: Precondition 4 and Postcondition 2 both cited `BC-6.23.001 PC3/PC4` for `/factory-unlock` clearing behavior — verified BC-6.23.001 PC3 is `/factory-lock` foreign-lock-held ACQUIRE refusal, unrelated to `/factory-unlock`; the self-release clearing act is BC-6.23.001 PC4 alone. Corrected both occurrences to `BC-6.23.001 PC4`. All other 17 body cross-anchors (BC-4.13.001 PC2/PC3/PC4/Invariant 9; BC-4.17.001 Precondition 4/Invariant 7/Invariant 8/EC-015; BC-6.23.001 PC1) verified CORRECT — no further defects found. Invariant 8's BC-4.13.001 Invariant 10 cite left unchanged as dated historical/dormant text (POLICY 1 append-only) describing BC-4.13.001 Invariant 10 as it existed at v1.2 authoring time (2026-07-13), before that invariant's later Phase-B envelope-diagnostic evolution — not a live wrong-section claim. No PC/Invariant/EC renumbered. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.13 | 2026-08-26 | Pass-30 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 30 finding **F-P30-001 (HIGH, POLICY 14/17)**. `modified:` frontmatter array was ordered ASCENDING (v1.1 at top, v1.12 at bottom) while this Changelog table was correctly ordered DESCENDING (v1.12 at top) — a POLICY 14 parity mismatch between the two required-to-agree legs. Sibling BC-4.17.001 carried the identical mismatch (fixed same burst, its own v1.15); sibling BC-7.07.001 was already clean (fixed at its own v1.31, F-P29-003) and a full three-BC cluster parity audit found no BC-7.07.001 regression. Corrected: `modified:` array reordered to strict descending-chronological (newest at top), matching this Changelog table and BC-7.07.001's established pattern. Dated HISTORICAL entry text (v1.1 through v1.12) unchanged — only array position corrected, per POLICY 1 append-only numbering. No PC/Invariant/EC renumbered. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.12 | 2026-08-26 | Pass-29 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 29 finding **F-P29-002 (MED, POLICY 18)**. `inputs:` frontmatter gap closed: this BC makes load-bearing exact-code-body current-state claims (PC3's `is_expired` comparison against `verify-factory-lock`; the migrated Precondition 6/Invariant 7/Invariant 8/EC-010 `STATE_MD_MAX_BYTES`/`extract_frontmatter` claims; PC4's `renew_lock_if_holder`/`TTL_SECONDS` claims; the `hooks-registry.toml` deregistration) without listing the underlying code files in `inputs:`. Same POLICY 18 sweep already applied to BC-7.07.001 (v1.29) and BC-4.17.001 (v1.13) — this BC was never itself swept. Added `crates/hook-plugins/verify-factory-lock/src/lib.rs`, `crates/factory-lock/src/lib.rs`, `crates/factory-lock-parse/src/lib.rs`, `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`, `plugins/vsdd-factory/hooks-registry.toml`, same path form the sibling BCs already use. Not the accepted BC-4.17.001↔BC-7.07.001 mutual-inputs cyclic-hash TD (that concerns only that pair's mutual ADR/BC edges) — these are missing CODE inputs, legitimately in-scope. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); pure `inputs:` addition. input-hash recompute and BC-INDEX title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.11 | 2026-08-26 | Pass-27 sibling-sweep remediation (product-owner), responding to adversarial spec-convergence pass 27 finding **F-P27-001 (HIGH, POLICY 4)**. §Story Anchor's `Dual-story anchor` quantifier and story list were not swept when the Traceability §Stories row was corrected at v1.10 (F-P25-002) to add S-17.05; §Story Anchor is corrected to a `Tri-story anchor` listing S-17.01, S-19.08, and S-17.05. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction to §Story Anchor's existing text only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.10 | 2026-08-26 | Pass-25 spec-convergence remediation (product-owner), responding to adversarial spec-convergence pass 25 finding **F-P25-002 (MED)**. Traceability §Stories row's `[pending]` placeholder for the ADR-046 `stamp-state-timestamp` PC4 hook-authorship implementation resolved to the confirmed implementing story: S-17.05 (`stamp-state-timestamp-hook`, E-17 Wave 5, 8pts, `tdd_mode: strict`), verified present in STORY-INDEX.md. No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1); in-place correction to the Traceability §Stories row's existing text only. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.9 | 2026-08-26 | Pass-6 sibling-sweep remediation (product-owner), responding to adversarial spec review pass 6 (1 MED + 2 LOW total across the ADR-046 BC cluster, all sibling-sweep-straggler class). **F-P6-002 (MED)** — PC3's "Failure mode — long burst TTL self-eviction" sub-paragraph corrected from `now >` (strictly-greater) to `now >=`, matching PC3's own already-corrected normative statement (F-P4-001, v1.7) and PC4 condition (c) — this sub-paragraph was a straggler the v1.7 fix missed. **Comprehensive sibling-sweep** — full grep sweep of this BC found and fixed 5 additional stray `ADR-046 vN.N` version-pin stragglers (the "verify-state-timestamp-refresh read capability" Precondition annotation + the Architecture Anchors bullet for that crate), stripped to the stable `ADR-046 §Decision N` anchor form per POLICY 19 anti-volatile-pin; all other swept categories (expiry boundary, STATE_MD_MAX_BYTES sourcing, malformed-arm disposition, 5-case numbering, shared-fn homes, TTL cast, event fields, per-arm fail-open) clean. No PC/Invariant/EC renumbered (POLICY 1 append-only). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.8 | 2026-08-26 | Pass-5 architecture-routed remediation mirror (product-owner), responding to ADR-046 v1.6's F-P5-001 (HIGH) and O-P5-001 (LOW). **F-P5-001 (HIGH)** — the migrated-Precondition-6 annotation's `STATE_MD_MAX_BYTES` sourcing corrected from unnamed "reused (not re-declared)" phrasing to ADR-046 §Decision 5 v1.6's exact sourcing text, mirrored verbatim: references `factory_lock_parse::STATE_MD_MAX_BYTES` — the single canonical declaration (`pub const STATE_MD_MAX_BYTES: u32 = 262144;`, relocated to `factory-lock-parse`) — not a locally re-declared constant, and not the now-deregistered `verify-state-timestamp-refresh` crate. **O-P5-001 (LOW)** — Architecture Anchors bullet for `verify-state-timestamp-refresh/src/lib.rs` annotated MIGRATED/DEREGISTERED (was calling the crate an "S-19.08 implementation target"), constant home reference corrected to `factory-lock-parse`. No PC/Invariant/EC deleted or renumbered (POLICY 1 append-only); both in-place corrections. BC-INDEX registration deferred to state-manager same-burst per POLICY 7/8. |
| 1.7 | 2026-08-26 | Pass-4 spec remediation (product-owner), responding to adversarial spec review pass 4 findings against ADR-046 (now v1.5). **F-P4-001 (MED, real bug)** — PC3's guard-boundary corrected from `now >` (strictly-greater) to `now >=`, matching this BC's own PC4 condition (c), ADR-046, and ground-truth code (`verify-factory-lock::is_expired`). **F-P4-002 (MED)** — applies ADR-046 v1.5 §Decision 5's per-element reconciliation table: Precondition 6/Invariant 7/Invariant 8/EC-010 RETAINED (not deleted) but annotated MIGRATED-to-BC-4.17.001; S-19.08 VP rows T-001..T-007 annotated RETAINED AS HISTORICAL/DORMANT. No PC/Invariant/EC deleted or renumbered (POLICY 1 append-only). BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.6 | 2026-08-26 | Pass-2 spec remediation round 2 (product-owner), responding to adversarial spec review pass 2 PRODUCT-OWNER-ROUTED findings against ADR-025/ADR-046 (ADR-046 now v1.2). **F-002 (HIGH, POLICY 19)** — stripped the load-bearing `ADR-025 v1.2` and `ADR-046 v1.1` version pins from the Traceability § ADR Reference row to the stable `ADR-025 §Decision 2/3/5/8/10` and `ADR-046 §Decision 1(b)` anchor forms. **F-003 (HIGH)** — resolved the v1.5 last_amended/Changelog "flagged for architect: AlreadyExpired not yet enumerated" note: ADR-046 v1.2 §Decision 1(b) now enumerates `AlreadyExpired`; historical narrative annotated accordingly. No PC/Invariant/EC content otherwise changed; no renumbering. BC-INDEX registration and title-cell sync deferred to state-manager same-burst per POLICY 7/8. |
| 1.5 | 2026-08-25 | ADR-046 v1.1 Companion Amendment 1 (i)-(v) (product-owner; pass-2 spec remediation): Invariant 1 email-collision scope note added (F-010, ADR-046 v1.1 item (i)); PC4 comparison predicate corrected to `>=` and "idempotent" scoped to ONLY the `expires_at` arm's byte-identical-suppression behavior (F-007, item (ii)); PC4 malformed-`expires_at` non-repair statement added — the hook never "fixes" an unparseable value, matching `verify-factory-lock`'s own fail-open-as-unlocked read (F-008, item (iii)); PC4/Architecture Anchors TTL_SECONDS canonical-const sourcing added — `factory_lock::TTL_SECONDS: u32 = 2700` (F-006, item (iv); 2700-second VALUE itself unchanged); Canonical Test Vectors gains a 6-row truth table covering `{Resolved+Match, Resolved+Mismatch, Failed}` x `{stamp-state-timestamp, precompact-flush}` so both automatic-renewal call sites' identical behavior is spec-visible here (item (v)). Also added for cross-BC consistency: PC4/Invariant 1 `AlreadyExpired` non-resurrection disposition (an already-expired self-held lock is never resurrected by either hook, evaluated before identity resolution) — flagged for architect since ADR-046 Decision 1(b) text does not yet enumerate this case (RESOLVED at v1.6 — ADR-046 v1.2 §Decision 1(b) now enumerates `AlreadyExpired`, see v1.6 row above). Related BCs: BC-7.07.001 added as second `renew_lock_if_holder` call site. Traceability ADR Reference row updated to ADR-046 v1.1 and to name BC-7.07.001. No PC/Invariant renumbered. |
| 1.4 | 2026-08-25 | ADR-046 ratification amendment (product-owner): PC4 ("Mid-burst TTL renewal") actor reassigned from `state-manager` (manual `factory-lock-write.sh renew`) to the new `stamp-state-timestamp` PostToolUse hook (SS-04; BC-4.17.001) — automatic, fires on every qualifying `Edit`/`Write`/`MultiEdit` to `.factory/STATE.md`, gated on writer-identity (`git config user.email`) == recorded `holder`; non-holder writes (e.g., admitted via BC-4.13.001 PC2 `LockExpired`) never renew. Invariant 1 ("state-manager is the sole writer") amended: carves out the mid-burst `expires_at` renewal as the hook's mechanized exception, operationalizing rather than relaxing the invariant (previously documentary/unenforced at the renewal call site). `factory-lock-write.sh renew` explicitly retained as a break-glass/manual fallback (unconditional, no identity check — acceptable because invocation is a conscious manual act). TTL=2700s constant (Invariant 2/AC-007) UNCHANGED — only renewal authorship moved, per ADR-046 Companion Amendment 1. H1 title re-enriched per POLICY 7. Description, Related BCs, Architecture Anchors, Traceability (Stories, ADR Reference) updated to cite sibling BC-4.17.001. `inputs:` frontmatter appended ADR-046. modified[] appended 2026-08-25 (v1.4). BC-INDEX/4-index registration deferred to state-manager same-burst per POLICY 7/8 (not applied by this amendment). |
| 1.3 | 2026-07-14 | F-P1-002 resolution (product-owner; post-merge burst; S-19.08 merged PR #646 squash 1304d280 2026-07-14): VP Anchors pending-placeholder "to be assigned by state-manager after S-19.08 VP authoring pass" replaced with definitive statement — S-19.08 verification delivered via BC-anchored unit/integration tests T-001..T-007 following the `(unit-test)` / `(integration)` convention in the VP table; no new VP-NNN IDs assigned (per-story unit tests follow established `(unit-test)` row convention); VP-096 (`extract_frontmatter` proptest, active from S-19.02) reused by transitivity for Invariant 7. Shipped test function names cited from commit 1304d280 (`origin/develop`). Canonical Principle Rule 6 compliance: placeholder was answerable in scope. modified[] appended 2026-07-14 (v1.3). |
| 1.2 | 2026-07-13 | S-19.08 Spec-First amendment (human-authorized; D-826/D-835): Precondition 6 added (`verify-state-timestamp-refresh` read capability: `max_bytes = 262144` (256 KiB); frontmatter-only via `factory_lock_parse::extract_frontmatter` (`crates/factory-lock-parse/`; S-19.02 PR #610; reuse-not-duplicate); cap rationale mirrors BC-4.13.001 Phase-A Precondition 3 + ADR-025 §Decision 12 §12.5 parity; fail-open on `OutputTooLarge` per ADR-025 Decision 7). Invariant 7 added: frontmatter-only mandate for `verify-state-timestamp-refresh` (`extract_frontmatter` exclusive; mirrors BC-4.13.001 Invariant 9). Invariant 8 added: soft-warn threshold adjudication — `verify-state-timestamp-refresh` reads STATE.md in full → Invariant 10 scope confirmed → `state_md_approaching_cap` MUST emit at `bytes_read > 200000 AND ≤ 262144` (boundary table parity with BC-4.13.001 Invariant 10). EC-010 added (STATE.md exceeds 262144 bytes: `OutputTooLarge` → guard fail-open). Verification Properties updated: unit-test rows T-001..T-007 added; VP-096 back-cited (extract_frontmatter reuse). Story Anchor updated: S-17.01 + S-19.08. Traceability Stories updated: S-17.01 + S-19.08. modified[] appended 2026-07-13 (v1.2). |
| 1.1 | 2026-06-11 | POL-14 auto-promotion (state-manager; D-544; PR #181 squash-merged c64b46d2 2026-06-11; S-17.01 MERGED; status draft→active; lifecycle_status draft→active; modified[] appended 2026-06-11 (v1.1)). No BC content changes. BC-INDEX v2.66→v2.67 (body row draft→active). |
| 1.0 | 2026-06-10 | Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md schema (holder, locked_at, expires_at); TTL=45min; mid-burst renewal; state-burst CAS push fix; fail-open on malformed; sole-writer invariant. PC1 (schema correctness), PC2 (unlock clears block), PC3 (TTL expiry guard), PC4 (mid-burst renewal), PC5 (CAS push), PC6 (single-dev zero friction). 4 error variants: SchemaViolation, StaleNullBlock, RenewalMissed, CASPushRejected. 9 edge cases EC-001..EC-009. CAP-031 registered same burst. lifecycle_status: draft (POL-14 auto-promotion pending). |
