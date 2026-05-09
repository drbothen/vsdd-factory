---
document_type: tech-debt-register
producer: state-manager
version: "1.0"
last_updated: 2026-05-08T00:00:00Z
---

# Technical Debt Register

## Purpose

This register is the canonical inbox for deferred work — anything the project should address later but is not actively in flight. That includes:

- **Bugs and defects** that aren't critical enough to fix today
- **Architectural improvements** identified during reviews or audits
- **Process gaps** discovered during pipeline runs (often tagged `[process-gap]`)
- **Follow-up enhancements** to specs, hooks, agent prompts, or workflows
- **Documentation gaps** that don't block current work
- **Optimization opportunities** without urgent need
- **Migration items** that ride a future release

Entries do not require the issue to be a hard block or the bug to be reproducible today. The register's job is to capture intent — to ensure that "we should do X later" doesn't get lost in a pipeline cycle's burst log or session checkpoint.

Every entry must have: title, severity, effort estimate, target release (or "TBD"), problem statement, proposed solution sketch, and references back to the originating evidence.

Closure: an entry moves to status `resolved` when the work ships, or `withdrawn` when the project decides not to do it (with rationale).

---

## Summary

| Priority | Count | Estimated Points |
|----------|-------|-----------------|
| P0 (next cycle) | 1 | TD-013 branch protection restore |
| P1 (within 3 cycles) | 3 | XL (29–39 across 6 sub-stories) + TD-010 publish + TD-017 bats-orphan-detection |
| P2 (backlog) | 18 | — (TD-015 per-invocation correlation ~30 pts; TD-016 run-all.sh glob; TD-018 clippy debt; TD-019 activate-helpers PowerShell-parity + Rust consolidation; TD-019a pwsh syntactic CI gate; TD-020 RESOLVED 2026-05-04 — sweep landed; TD-022 novelty-assessment hook phase-f5 path gap; TD-023 RESOLVED 2026-05-04 — single-commit burst protocol; TD-024 SKIP_SUITES un-skip needs CI-equivalent validation; TD-025 S-9.00 perf-baseline minor follow-ups [passes 1+2+3+4+5]) |
| P2 (backlog) | 2 new | TD-027 Stop-hook async-block surfacing (S-M effort, v1.1 target; added 2026-05-07); TD-030 Canonical-string sweep discipline (added 2026-05-08) |
| P1 (within 3 cycles) | +1 (enforcement implemented; Kani proof pending CI) | TD-031 post-EC-012 line-drift enforcement — escalated P2→P1 fix-burst-15; ENFORCEMENT IMPLEMENTED fix-burst-16 + generalized+tightened this session (validate-stable-anchors WASM hook; source-code allowlist; 316 violations; 62 tests; chunked sweep complete; validate-artifact-path sibling fixed; Kani harness assumption fixed fix-burst-18 026272ae; Kani proof execution deferred to CI pending rustc version upgrade) |
| P3 (v1.1+) | 6 | — (TD-021 frontmatter↔STORY-INDEX status drift, added 2026-05-04; TD-026 unaccounted_wasm_bytes policy, added 2026-05-05) |

## Debt Items

| ID | Source | Description | Priority | Introduced | Cycle | Story | Due |
|----|--------|-------------|----------|-----------|-------|-------|-----|
| TD-031 | Process gap (F5 pass-15 — 2026-05-08; F-P15-007) | Recurrent post-EC-012 line-drift pattern (4+ instances: F-P10-002 VP-079 SITE_3/4; F-P13-002 BC-7.06.001 §Fail-Closed Symmetry; F-P14-002 DI-019; pattern continues). TD-VSDD-091 stable-anchor convention exists but no rule enforces it. Codify: add rule under `rules/` directory enforcing TD-VSDD-091 — spec body text MUST use stable symbol anchors (e.g., `factory_dispatcher::main::run`, `RegistryError::AsyncBlockConflict arm`) instead of `main\.rs:[0-9]+` line citations, with changelog/historical text exempted. Enforcement options: (a) lint-hook scan; (b) PR template checklist; (c) adversary-prompt review axis. Decision pending architect adjudication on enforcement mechanism. **Escalated P2→P1 in fix-burst-15 per O-P16-001 (2026-05-08). Recurrence pattern confirmed: fix-burst-14 (which codified TD-031) introduced 5 fresh TD-031 violations — F-P16-001 (4 SITES line cites stale), F-P16-002 (BC-3.08.001 v1.9 amendment cites wrong line 289 → actual 300), F-P16-004 (Scenario 7/8 line range stale across 3 documents). Surface-only codification fails without enforcement. ENFORCEMENT IMPLEMENTED 2026-05-08 fix-burst-16 (bb661eaa): native WASM Rust lint-hook `validate-stable-anchors` (renamed from validate-td031-stable-anchors in rename commit bb661eaa; generalizes to all spec stable-anchor conventions). Hook registered PreToolUse+Edit|Write+on_error=block+priority=155. Detection: `<word_chars>.rs:<digits>` in .factory/specs body; exempt: ## Amendment/Changelog headings + SITES=( bash fences. 180 pre-existing violations (Rust-only scope) surfaced; verification 98.9% TP (178/180). 2 FPs in VP-INDEX YAML frontmatter (exempt-zone gap; optional fix: skip ^---$ delimiters at line 1). HOOK GENERALIZED+TIGHTENED 2026-05-08 (this session): (1) d6dcdd9f: regex generalized from `.rs:NNN` to `<word>.<lowercase 1-8>:<digits>` — language-agnostic; caught 1,229 violations including 938 `.md` cross-doc refs (different class). (2) ab25e45d: tightened with source-code allowlist (ext ∈ {rs, toml, sh, bash, py, ts, tsx, js, jsx, go, bats, yaml, yml, json, lock, lobster, wasm, c, cpp, h, hpp, rb}); excludes .md/.html/.txt. Final scope: 316 violations across 60+ source-code/config/test spec files. Markdown cross-doc refs (e.g., `STATE.md:42`) are out of scope — different drift class; would require separate hook design if added. 58/58 tests pass. Chunked mass-sweep approved (post-compact). **FOLLOW-UP 2026-05-08 fix-burst-17 (cc5a016b + 8b4f697f): (1) cc5a016b — validated `is_spec_target` semantics in validate-stable-anchors: absolute-path Edit/Write payloads from Claude Code were silently exiting Continue (false-negative on absolute paths); 4 absolute-path tests added; 58/58 → 62/62 for validate-stable-anchors. (2) 8b4f697f — sibling propagation gap fixed: validate-artifact-path had identical absolute-path bug in both `matches_canonical` and `hook_logic`; 4 tests added to validate-artifact-path; both WASM artifacts rebuilt; validate-artifact-path test count 54/54 → 58/58. STATUS: P1 — enforcement implemented + validated across both production hook plugins. NOT yet RESOLVED: Kani harness deferral — lib.rs:593 + tests/kani_path_matching.rs:271 in validate-artifact-path have stale `kani::assume(!path.starts_with(".factory/"))` that needs updating to reflect absolute-path semantics; deferred to next Kani hardening pass. **FOLLOW-UP 2026-05-08 fix-burst-18 sub-burst 1 (026272ae on fix branch + e23a82ab on factory-artifacts): F-P19-002 Kani harness fix — VP-070 Proof 2 assumption tightened to exclude both relative and absolute .factory/ paths in 3 locations (lib.rs:593, kani_path_matching.rs:271, VP-070.md:103). VP-070 v1.1→v1.2. Kani CLI version mismatch (cargo kani 0.67.0 → rustc 1.93.0-nightly < workspace 1.95) defers actual proof execution to CI. FOLLOW-UP 2026-05-08 fix-burst-18 sub-burst 2 (91bb304c on factory-artifacts): F-P19-001 corpus-wide prose-form sweep — 18 grep matches; 6 body refs migrated across 4 files (BC-7.03.009 v1.2→v1.3, open-questions.md v1.1→v1.2, prd.md v1.2→v1.3, ADR-019 v1.8→v1.9); 12 refs left as historical record. FOLLOW-UP 2026-05-08 fix-burst-18 sub-burst 3 (ce848f24 on factory-artifacts): F-P19-003 BC-4.11.001 amendment — new Invariant 8 (Path Form Invariance) + cross-references in Postconditions 2 and 7. v1.1→v1.2. STATUS: TD-031 enforcement — both production hooks now fixed; both have absolute-path tests; both WASM rebuilt; spec corpus updated. Kani proof confidence pending CI run with newer rustc (cargo kani 0.67.0 / rustc 1.93.0-nightly vs workspace 1.95 mismatch). **FOLLOW-UP 2026-05-08 fix-burst-21 (87dd64aa, 2ea5ee5a, 56f0b883): F-P22-001 corpus-wide lobster-line-cite sweep (88 BCs in ss-05 v1.1→v1.2; broadest single-burst application of L-P19-001 + L-P20-001 to date); F-P22-002 BC-1.14.001 v1.10→v1.11 cycle-anchor fabricated symbols fix (RegistryEntry.async_flag, executor.rs::execute_tiers, executor.rs::spawn_async_plugin) + L-P21-001 retroactive sweep found 7 additional fabricated symbols: BC-1.07.005 v1.0→v1.1, BC-1.07.006 v1.0→v1.1, edge-cases.md v?→v?, domain-events.md v?→v?, VP-016 v1.0→v1.1, VP-043 v1.0→v1.1 (2ea5ee5a); F-P22-003/004/005 E-12 epic v1.0→v1.3 + L-P21-002 retroactive sweep on 9 stories under E-12 (56f0b883). Fix-burst-21 is the largest single fix-burst in the F5 cycle (91 BC bumps + 7 additional fabrication corrections). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-22 (9ebd5c31, 60072605): F-P23-001 + F-P23-002 broadest lobster-line-cite sweep ever — ~440 ss-05 BCs + 174 ss-06 BCs + ss-03 (BC-3.08.001 v1.12) + ss-04 (BC-4.04.005 v1.2) + ADR-008/009 historical annotations + open-questions.md updates; FULL syntactic class coverage (postcondition-form double-backtick + cross-subsystem references) per L-P23-002 (9ebd5c31). F-P23-003 BC-1.07.005 + BC-1.07.006 H1 + BC-INDEX rows 226-227 + VP-043 §Source Contract all rebranded to real test fn `loads_generated_registry_from_disk` per L-P23-001 all-cite-sites discipline (60072605). STATUS: P1, prose-only path is empirically non-convergent (6-pass HIGH streak). User overrode adversary halt recommendation; continuing prose-only per user direction.** **FOLLOW-UP 2026-05-09 fix-burst-23 (3576f1a6, 78977e26, this sub-burst): Comprehensive corpus audit per user directive (option 2 at pass-24 strategic decision) — every historical fabricated symbol grepped corpus-wide; 11 active-body cite sites patched across 4 files: ADR-019 v1.9→v1.10 (4 sites), BC-7.06.001 v1.10→v1.11 (1 site — F-P24-002), S-15.01 v1.20→v1.21 (2 sites — F-P24-003), E-15 v1.1→v1.2 (2 sites — F-P24-004, NEW sibling); post-sweep grep 0 active-body matches for all 10 fabricated symbols (3576f1a6). F-P24-001 VP-043 v1.2 frontmatter sync + Amendment block propagation gap closed (78977e26). L-P24-001 codified (Phase 0 brownfield-ingestion carve-out; F-P24-005 adjudicated). L-P24-002 codified (comprehensive-corpus-grep discipline — the meta-lesson from 7-pass HIGH streak). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-24 (609cae4f): F-P25-001..F-P25-007 sweep — ADR-019 v1.10→v1.11, BC-3.08.001 v1.12→v1.13, BC-INDEX v1.49→v1.50, VP-077 v1.11→v1.12, VP-079 v1.16→v1.17, S-15.01 v1.21→v1.22 (post-merge retrofit annotation per L-P25-001), E-15 v1.2→v1.3. L-P25-001 codified (post-merge story-body retrofit discipline — O-P25-002 recurrence carrier). L-P25-002 codified (F1 architect-proposal carve-out — extends L-P24-001 to greenfield Phase F1). 8 consecutive HIGH passes (P18-P25). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-25 (4c386236, a2c390cd): F-P26-001 PluginEntry → RegistryEntry corpus sweep across 12 active-body sites in VP-077 v1.12→v1.13, VP-078 v1.8→v1.9, S-15.01 v1.22→v1.23. F-P26-007 VP-077 harness skeleton tuple → PluginPartition struct. F-P26-002/003/004/005 post-merge frontmatter retrofit on 5 stories (S-15.01 v1.23→v1.24, S-13.01 v1.0→v1.1, S-12.01 v1.0→v1.1, S-12.02 v1.0→v1.1, S-12.06 v1.0→v1.1) + S-9.00 v1.6→v1.7 + POST-MERGE-STATE §Tasks annotations (6 stories) + F1 carve-outs on engine-discipline cycle artifacts. L-P26-001 codified (codifying-burst MUST run L-P24-002 corpus-sweep on new lesson). L-P26-002 codified (state-manager PR-merge handler MUST update story frontmatter atomically). 9 consecutive HIGH passes (P18-P26). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-27 (bc7ae728 factory-artifacts + 7b841eca fix branch): F-P28-001 — VP-070.md v1.2→v1.3 + VP-071.md v1.2→v1.3 source frontmatter proof_method synced kani→kani-proof; VP-INDEX rows already correct (v1.35). F-P28-003/004/005 index + STATE.md + lessons updates. L-P28-001 codified (field-value drift class: codifying burst grep MUST include both index AND source-of-truth artifact frontmatter). L-P26-002 amended: merged_in: none sentinel for pre-GitHub-PR merges (F-P28-002). VP-INDEX v1.35→v1.36. 11 consecutive HIGH passes (P18-P28). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-28 (this burst): F-P29-001 — VP-074.md v1.0→v1.1 source frontmatter proof_method kani→kani-proof (last remaining kani instance; L-P28-001 META self-application fix — VP-074 was missed by fix-burst-27 sweep). VP-INDEX v1.36→v1.37 (VP-074 source sync recorded). VP-070 last_amended field added (F-P29-004). L-P26-002 verification block added (F-P29-003; 21 pre-PR-6 stories confirmed carrying merged_in: none). STATE.md story arithmetic reconciled: Partial (2): S-2.05 + S-3.04; Draft (23 file-resident); 15 unauthored stub IDs documented separately (F-P29-002). 12 consecutive HIGH passes (P18-P29). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-29: F-P30-001 STATE.md ADR count 19→20 (ADR-020 added 2026-05-08; count not bumped across multiple fix-bursts). F-P30-002 L-P28-001 META-self-application verification block appended (VP-074 was 3rd kani instance missed by fix-burst-27; fix-burst-28 closed it; fix-burst-29 records META-META acknowledgment). F-P30-003 STATE.md historic E-10 line annotated "total_bcs 1931 at D-313 (now 1947)". F-P30-004 deferred → TD-032 (subsystem field format drift; S-15.03 scope). STATE.md compacted: older Current Phase Steps rows extracted to burst-log. 13 consecutive HIGH passes (P18-P30). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-30 (this burst): F-P31-001 VP-INDEX Breakdown table reconciled with VP-074 source frontmatter (kani-proof canonical per VP-074.md:19 + Kani harness section). integration 22→21 (VP-074 removed); kani-proof 3→4 (VP-074 added: now VP-070, VP-071, VP-074, VP-077). Full Index VP-074 row Proof Method integration→kani-proof. VP-INDEX v1.37→v1.38. F-P31-002 STORY-INDEX Status Summary corrected (merged 57→62, draft 28→23, prose epic count 15→16). STORY-INDEX v2.56→v2.57. F-P31-003 STATE.md Identifier Conventions Epic count 15→16 (identifier-table full audit: all other rows verified correct). F-P31-004 STATE.md:186 checkpoint pass-29→pass-30 (partial-fix regression of fix-burst-29 STATE.md sweep). L-P28-001 amended: Breakdown-table audit step added (process-gap from F-P31-001). 14 consecutive HIGH passes (P18-P31). STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-31: F-P32-001 VP-INDEX VP-074 Full Index Scope cell SS-04→SS-01, SS-04 (L-P28-001 sub-rule — sibling-cell layer; same fix-burst-30 that patched Proof Method column missed adjacent Scope column). VP-INDEX v1.38→v1.39. L-P28-001 sub-rule added (Full Index per-row sibling cells). 15th consecutive non-NIT pass (first 0-HIGH pass since P17); trajectory bending. STATUS: still P1; Kani CI gate still deferred.** **FOLLOW-UP 2026-05-09 fix-burst-32: F-P33-001 VP-INDEX VP-074 + VP-076 Domain Invariant cells — → DI-002/DI-004 (L-P28-001 sub-rule META-META-META recurrence — per-row sibling-cell audit missed DI column on same VP-074 row + VP-076 also drifted). Corpus-wide per-row sibling-cell audit on all 79 VP-INDEX rows: 2 DI drifts (VP-074+VP-076) fixed; 0 scope drifts; all other rows clean. 10-row BC-INDEX sample: 0 status drifts. 11-row STORY-INDEX sample: 0 spec-layer drifts. L-P28-001 META-META-META verification block records 3-burst recurrence pattern. VP-INDEX v1.39→v1.40. 16th consecutive non-NIT pass. STATUS: still P1; Kani CI gate still deferred.** | P1 | F5 pass-15 fix-burst-14 2026-05-08 (F-P15-007); escalated fix-burst-15 2026-05-08 (O-P16-001); enforcement implemented fix-burst-16 2026-05-08 (bb661eaa); sibling fix + absolute-path fix fix-burst-17 2026-05-08 (cc5a016b + 8b4f697f); Kani harness fix + corpus sweep + BC-4.11.001 amendment fix-burst-18 2026-05-08 (026272ae + 91bb304c + ce848f24) | v1.0-feature-plugin-async-semantics-pass-1 | S-15.01 | v1.0.1 |
| TD-033 | O-P32-002 disposition (F5 pass-32 — 2026-05-09) | SS-10 architecture spec vs current dev repo separation. ARCH-INDEX SS-10 row documents `plugins/vsdd-factory/commands/` (110 files, 110 slash-command bindings) and 12 bin tools. The `commands/` directory does not exist in the current dev repo; it is materialized at plugin-build time from skill catalog source. Bin tool count has drifted (12 documented vs 13 on disk as of pass-32 observation). Per POLICY 4 (semantic anchoring): the SS-10 spec describes the plugin distribution model (post-packaging state), not the current dev repo state. This is intentional architectural separation — SS-10 is a distribution artifact spec, not a dev-tree spec. No spec edit required. If the bin tool count drift becomes actionable (e.g., before next release), update ARCH-INDEX SS-10 row to reflect actual on-disk count. Effort: XS (count update only). Not blocking current adversary protocol. O-P32-002 adjudicated as documentation gap, not defect. | P3 | F5 pass-32 2026-05-09 (O-P32-002) | v1.0-feature-plugin-async-semantics-pass-1 | — | v1.1 |
| TD-032 | Process gap (F5 pass-30 — 2026-05-09; F-P30-004) | Subsystem field format drift — 564 BCs use bare `subsystem: SS-NN` while the remainder use the quoted form `subsystem: "SS-NN"`. Both forms are YAML-equivalent and do not break any current parser or validator. Relevant if any downstream tool string-greps formatted fields rather than parsing YAML. Document the canonical convention (quoted or unquoted) in `rules/` and add a normalizer to S-15.03's `validate-index-source-coherence` hook scope so the hook can detect or normalize the drift class mechanically. Effort: S (rules/ doc addition + hook scope extension). Not blocking current adversary protocol. | P3 | F5 pass-30 2026-05-09 (F-P30-004) | v1.0-feature-plugin-async-semantics-pass-1 | S-15.03 | v1.1 |
| TD-030 | Process gap (F5 pass-10 — 2026-05-08; O-P10-003) | Canonical-string sweeps need separate discipline from version-label sweeps. Pattern observed in F5 cycle: F-P9-003 swept 38 version-label sites across 9 files; F-P10-001 surfaced a canonical enum-string drift (`on_error_block_with_async_true` → `async_block_conflict`) that the version-label sweep could not detect because it targeted doc-comment BC/VP version strings, not wire-format enum values. Codify: future canonical-value renames in any BC's wire format MUST include a grep over `**/*.rs` and `**/*.bats` for the OLD string value as part of the fix-burst sweep checklist. This is distinct from and complementary to version-label sweeps (S-7.01 discipline). Enforcement: extend POLICY 8 propagation rules OR add new policy "canonical_string_sync" that triggers on any BC amendment that renames an event-type, error-code, or violation-string field value. Effort: S (policy + checklist addition; no code changes required). | P2 | F5 pass-10 fix-burst-9 2026-05-08 (O-P10-003) | v1.0-feature-plugin-async-semantics-pass-1 | S-15.01 | v1.0.1 |
| TD-029 | Process gap (F5 pass-8 — 2026-05-08) | Bats integration test coverage gap for `RegistryError` variants: fix-burst-7 (test-writer bats Scenario 8) demonstrates that DuplicateEntry had zero bats coverage prior to F-P8-001. Codified rule: every `RegistryError` variant in `registry.rs` MUST have at least one dedicated bats scenario in `tests/bats/async-event-schema-conformance.bats` (or equivalent integration test file). When a new variant is added to the enum, a failing bats test MUST be authored in the same commit before the implementer wires the arm. Enforcement suggestion: CI step that cross-references `RegistryError` variant names against bats `@test` descriptions. | P2 | F5 pass-8 fix-burst-7 2026-05-08 | v1.0-feature-plugin-async-semantics-pass-1 | S-15.01 | v1.0.1 |
| TD-028 | Process gap (F5 pass-8 — 2026-05-08) | Spec-impl drift on fail-closed/fail-open classification for `RegistryError` variants: F-P8-001 revealed that `RegistryError::DuplicateEntry` was handled by a catch-all `_ => 0` (silent exit-0/fail-open) while BC-7.06.001 mandated "refuses to start" (fail-closed). A CI gate should enforce that every `RegistryError` variant has an explicit exit-code match arm in `main.rs` BEFORE the catch-all, and that the catch-all itself maps to non-zero (e.g., `_ => 1`). Implementation: a clippy lint or a `cargo test`-level assertion that exhaustively checks `match err` arms. Until automated, human code review MUST verify exhaustiveness of the `match` on `RegistryError` before any PR that adds a new variant. | P2 | F5 pass-8 fix-burst-7 2026-05-08 | v1.0-feature-plugin-async-semantics-pass-1 | S-15.01 | v1.0.1 |
| TD-025 | S-9.00 adversary pass 1+2+3 deferred findings (2026-05-05) | S-9.00 perf-baseline minor follow-ups deferred from adversary passes 1, 2, and 3. Pass-1 items (six): LOW-1 — 30MB kill-switch regex in bats (`.wasm` glob) is too permissive, acceptable risk; LOW-2 — AC-7 test does not assert `cold_start_p95_measured_ms ≤ 500ms` (currently record-but-don't-enforce per spec; methodologically suspect per adversary — fold into a future enforcement gate when CI linux-x64 measurement is available); LOW-3 — IFS discipline in bats AC-8 sorted-array computation (theoretical IFS clobber risk; bash local scope limits blast radius); LOW-4 — AC-8 median doc-vs-test divergence (test independently computes median from bundle; doc records a specific value; both pass regardless of drift); NITPICK — `darwin` vs `Darwin` platform string in baseline doc (cosmetic, does not affect functionality); NITPICK — cold-start fixture envelope realism (fixture uses synthetic envelope; a real Claude Code recording would be more representative but requires live session capture). Pass-2 additional deferrals (two): MEDIUM-4 — hyperfine command in `measure-bundle-sizes.sh` does not quote paths (`${DISPATCHER_BINARY} < ${FIXTURE}`); currently safe because REPO_ROOT has no spaces, but paths with spaces would break the shell-string passed to hyperfine — defer until a path-with-spaces test scenario is added; MEDIUM-5 — RESOLVED in adversary pass 3 (2026-05-05): p95 formula corrected to NIST nearest-rank `ceil(N*0.95)-1` (methodology_version 2); canonical baseline updated to 642.6ms; TD-025 entry preserved for audit trail. Pass-3 additional deferrals (three): LOW-1-P3 — hello-hook accounting drift between rc.1 baseline and current (denominator-numerator scope mismatch; not a correctness issue, the frozen-17 sum is correctly scoped); LOW-2-P3 — fixture missing `transcript_path`/`cwd` fields (may bias cold-start measurement; synthetic envelope used; requires live session capture to fix); LOW-3-P3 — dispatcher binary path hardcoded to `target/release/`, but CI matrix uses `target/${target}/release/` (safe for darwin-arm64 local; CI cross-compilation contexts may differ). Fix when: LOW-2 (original pass-1) is the highest-priority item — resolve when CI linux-x64 cold-start measurement is available (recommended before S-9.01 dispatch). Pass-3 LOWs may be addressed in a batch follow-up. Pass-4 additional deferral (one): LOW-4-P4 — frontmatter version drift in evidence-report.md (cosmetic; evidence-report version bumped from 1.3 → 1.4 per pass-4 fix, but no structural enforcement on evidence-report frontmatter versioning exists; defer until a formal evidence-report versioning policy is adopted). Pass-5 additional deferrals (two): LOW-1-P5 — AC-5 anti-tautology check validates script output vs OS `wc` counts, but does NOT compare doc values vs script output (script-vs-doc gap); renaming the test description to clarify scope would help discoverability; defer until a doc-vs-script integration test is warranted; LOW-2-P5 — AC-1.md bats output section is template-shaped rather than a real captured run (cosmetic; CI is the actual gate for correctness; the bats test itself exercises the correct assertion path). | P2 | S-9.00 adversary passes 1+2+3+4+5 (2026-05-05) | v1.0-brownfield-backfill | S-9.00 | v1.0.1 |
| TD-026 | S-9.00 adversary pass 3 (2026-05-05) | `unaccounted_wasm_bytes` is reported by `measure-bundle-sizes.sh` but ungated by any AC. Per `perf-baseline-w16.md` Unaccounted Bytes Policy section: S-9.07 must reduce to documented minimum and assert floor. Current value: 155053B (hello-hook.wasm SDK example + underscore-named stubs). Silent drift into `unaccounted_wasm_bytes` by S-9.01..S-9.06 stories is forbidden — each wave must explicitly add to frozen-17 OR to a known-overhead allowlist with a TD entry. | P3 | S-9.00 adversary pass 3 (2026-05-05) | v1.0-brownfield-backfill | S-9.07 | v1.1 |
| TD-027 | Prism audit 2026-05-07 + F1 delta analysis OQ-1 | Stop-hook (or SubagentStop) to surface accumulated async-hook block decisions at turn end. Today, plugins with `on_error = "block"` that run async emit `hook.block` to `events-*.jsonl` invisibly. The plugin-async-semantics cycle closes this for validators that should never be async; but plugins that legitimately stay async (telemetry, optional advisories) will continue to log silently. A Stop hook reads the events log and emits a summary to stderr at turn end: "FYI: N async-hook block decisions this turn — hook name, file, reason." Soft-blocked-by v1.0-feature-plugin-async-semantics-pass-1 (need correct partition first). | P2 | prism audit + F1 delta analysis 2026-05-07 | v1.0-feature-plugin-async-semantics-pass-1 | — | v1.1 |
| TD-024 | rc.11 retag process-gap (2026-05-04 → 2026-05-05) | Un-skipping a previously-skipped `SKIP_SUITES` bats suite must be validated in a CI-equivalent environment before being marked passing. TD-020 sweep (RESOLVED 2026-05-04) marked `state-health` and `generate-registry` as UN-SKIPPED with no test changes based on local-pass evidence; both failed in CI during rc.11 release, requiring two retag rounds (external TD-VSDD-054: shallow-clone history dependency in `scripts/generate-registry-from-hooks-json.sh`, fixed by vendoring `scripts/legacy/hooks-json-pre-templating.json`; external TD-VSDD-055: missing local git config in `state-health.bats` setup, fixed by adding `git config user.email/user.name/commit.gpgsign` after `git init`). Local-pass is necessary but not sufficient. Required CI-equivalence checks: empty global git config; shallow-clone (no history beyond depth=1); no operator-installed CLI tools beyond what the workflow declares; clean shell env. Disposition: add a checklist item to the SKIP_SUITES un-skip workflow / agent prompt / CONTRIBUTING note and (optionally) a CI smoke job that runs the full bats suite under a minimal-environment matrix. | P2 | rc.11 retag rounds (2026-05-04 → 2026-05-05) | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-023 | External TD-VSDD-053 — single-commit burst protocol (2026-05-04) | RESOLVED 2026-05-04. Cross-references external TD-VSDD-044 (self-referential factory-artifacts HEAD cite in STATE.md/HANDOFF.md "current state" sections caused 6× recurrence loops in real-world dogfood) and external TD-VSDD-053 (the structural fix). **Engine-side changes shipped:** (a) `templates/verify-sha-currency.sh` — removed factory-arts cite extraction + cite-vs-HEAD comparison + fabrication check on factory-arts SHAs (~80 lines); preserved develop cite check, MULTI_COMMIT_CHAIN_NOT_ALLOWED guard, wave-state↔STATE cross-record python check. (b) `agents/state-manager.md` — protocol references updated from "Single Canonical SHA + Two-Commit Protocol" to "Single-Commit Burst Protocol"; added explicit guidance that current factory-artifacts HEAD is `git -C .factory log -1`, not a string in any artifact. (c) `skills/state-burst/SKILL.md` — full rewrite to single-commit; Stage 1/2 sections removed; `15fa97e6` placeholder pattern removed; commit message must NOT contain `backfill` (regression-guard token). (d) `templates/state-manager-checklist-template.md` — full rewrite to single-commit; `remediation_sha:` field handling now offers (a) omit-and-look-up vs (b) post-commit amendment; historical past-pass `remediation_sha` values stay immutable. **Preserved unchanged:** `validate-input-hash.sh` (artifact-level drift detection), `validate-state-pin-freshness.sh` (version-pin freshness), historical SHA references in changelog rows / decisions log / cycle manifests / TL;DR History. **Acceptance:** input-hash drift detection unchanged; historical SHA audit trail unchanged; single-commit state-manager protocol works for 10+ consecutive bursts (verify in next 10 wave-gate convergence cycles). | P2 | engine fix 2026-05-04 | v1.0-brownfield-backfill | — | RESOLVED |
| TD-022 | TD-020 sweep follow-on (2026-05-04) | `validate-novelty-assessment.sh` case-statement does not match `.factory/phase-f5-adversarial/adversarial-delta-review.md` — path falls through to `exit 0`. `phase-f5-scoped-adversarial` skill writes to that path (SKILL.md:84,171). Hook silently passes phase-f5 delta reviews without Novelty Assessment validation. Fix: add case arm + 2 bats tests. TD-020 deleted the tests that described this gap (correct — they asserted unimplemented behavior). | P2 | TD-020 sweep PR #82 (2026-05-04) | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-021 | post-ADR-015 story-housekeeping audit (2026-05-04) | Story-frontmatter vs STORY-INDEX status drift. Several stories (initial sample: S-4.06, S-4.07; possibly broader) have stale `status: ready` in frontmatter while STORY-INDEX correctly tracks them as `merged` (per merged PRs #30, #31). Pre-existing drift not caused by recent ADR-015 supersession work. Fix: either add a CI lint that asserts frontmatter status matches STORY-INDEX status, OR establish a one-time backfill burst to reconcile. | P3 | post-ADR-015 audit 2026-05-04 | v1.0-brownfield-backfill | — | v1.1+ |
| TD-020 | run-all.sh skip-suites cleanup (referenced but never registered) | RESOLVED 2026-05-04 — sweep PR (`fix/td-020-sweep-skipped-bats-suites`) closed all four entries. **Per-suite outcomes:** (a) `codify-lessons` — FIXED: removed BC-5.36.007 (referenced a worktree that no longer exists post-merge); rewrote 15 path references to use `BATS_TEST_DIRNAME`-based absolute paths so the suite passes under run-all.sh's `cd $PLUGIN_ROOT`. 15/15 pass. (b) `generate-registry` — UN-SKIPPED with no test changes; generator stabilized after the original TD-016 churn. 6/6 pass. (c) `novelty-assessment` — FIXED: deleted 3 tests that asserted hook behavior the current `validate-novelty-assessment.sh` does not implement (delta-review path matching + a story-adversarial-review path that doesn't exist anywhere in the plugin) plus the paired happy-path test that was a misleading false-positive. 15/15 remaining pass. (d) `state-health` — UN-SKIPPED with no test changes. 31/31 pass. SKIP_SUITES is now empty; the comment block in run-all.sh updated; CHANGELOG entry "TD-020 sweep — bats SKIP_SUITES cleanup" documents the deletions. **Postmortem (rc.11):** the (b) and (d) "UN-SKIPPED with no test changes" sub-resolutions were locally-validated only and broke in CI during rc.11 release. Fixed via external TD-VSDD-054 (vendored `scripts/legacy/hooks-json-pre-templating.json`) and external TD-VSDD-055 (local git config in `state-health.bats` setup). See TD-024 for the codified process gap. | P2 | rc.3 recovery → registered 2026-05-04 → resolved 2026-05-04 | v1.0-brownfield-backfill | — | RESOLVED |
| TD-019a | PR #78 review follow-up (2026-05-04) | Add a lightweight pwsh syntactic-validation gate to CI — `[System.Management.Automation.Language.Parser]::ParseFile()` on both `.ps1` files, plus 3 smoke invocations (`-Help`, mocked-Linux, bad-platform). ~10 lines of YAML, no Pester runner needed, catches future parse errors before they hit a Windows operator. Recommended as the v1.0-cycle stopgap before TD-019(a)'s full Pester suite lands. Acceptance: PR opens against `develop` adding a `lint-pwsh` job to the existing CI workflow that runs on `windows-latest` (or `ubuntu-latest` with pwsh installed) on any PR touching `plugins/vsdd-factory/skills/activate/*.ps1`. | P2 | PR #78 review (test-analyzer) 2026-05-04 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-019 | post-rc.8 PowerShell-parity work (2026-05-04) | Two-part: (a) Add Pester test suite at `plugins/vsdd-factory/tests/activate.Tests.ps1` mirroring the existing `.bats` matrix — same MOCK_UNAME_S/M envvar overrides, same VSDD_PLUGIN_ROOT_OVERRIDE for synthetic-root apply tests; wire into CI on a `windows-latest` job. **Acceptance for (a)**: All 18 supported/unsupported platform tuples and all 13 apply-platform scenarios from `activate.bats` have Pester equivalents that pass on a `windows-latest` runner; the JSON output of every MOCK tuple is diffed byte-for-byte against the bash sibling (run on `ubuntu-latest`) and matches. (b) Consolidate `apply-platform.sh` + `apply-platform.ps1` file-copy/verify logic into a `factory-dispatcher activate --platform <p>` Rust subcommand; keep thin shell shims (~40 lines each) for "verify binary exists before invoking it" since the binary cannot self-verify. **Acceptance for (b)**: (1) shims are ≤40 LOC each and only verify binary presence + invoke `factory-dispatcher activate`; (2) the Rust subcommand lands in the existing `factory-dispatcher` crate, not a new crate; (3) shared bats+Pester matrix passes against the shim+Rust path; (4) deletion of legacy `apply-platform.{sh,ps1}` body code is included in the same PR; (5) migration path documented for in-flight v1.0 activations (re-run activate is sufficient). Defer until v1.0.0 ships and apply-logic growth (drift detection, settings.local.json merging, dry-run mode) justifies the refactor. | P2 | post-rc.8 PS1-parity 2026-05-04 | v1.0-brownfield-backfill | — | v1.1 |
| TD-018 | rc.3 recovery (D-209, D-210) | Workspace clippy debt sweep — `non_snake_case` test fn names, type_complexity, unused imports surfaced by `--all-targets -- -D warnings` on rc.3 PR #62; currently suppressed via `#[allow]` attrs at file/fn level; future cleanup: rename test fns OR establish project-wide `#![allow(non_snake_case)]` for test modules with BC-named tests | P2 | rc.3 cut 2026-05-03 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-017 | rc.3 recovery (D-210) | Add bats-orphan-detection CI step — fail fast if any bats file references a non-existent hook script; prevent future "deleted file" failures caused by native WASM ports that remove .sh hooks without cleaning bats test references | P1 | rc.3 recovery 2026-05-03 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-016 | rc.3 recovery (D-211) | Refactor `run-all.sh` to use glob discovery (`tests/*.bats`) instead of hardcoded enumeration — prevents future failures when bats files are deleted without updating run-all.sh | P2 | rc.3 recovery 2026-05-03 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-015 | S-8.08 pass-5 adjudication (D-181) | Per-invocation telemetry correlation: host::invocation_id() SDK extension + schema enrichment + cross-hook sweep (track-agent-start/stop, pr-manager-completion-guard, validate-pr-review-posted, handoff-validator, regression-gate); ~30 pts epic | P2 | 2026-05-01 | v1.0-brownfield-backfill | S-8.08 | v1.1+ |
| TD-014 | User audit | Full native WASM migration of remaining 43 bash hooks (8 dispatcher-routed + ~35 inline); legacy-bash-adapter deletable post-migration; S-5.05 migration guide "26 hooks" claim stale (actual ~43) | P2 | 2026-04-30 | v1.0-brownfield-backfill | — | v1.1+ |
| TD-013 | Release process | Restore main branch protection with proper bot bypass before v1.0.0 GA — required_pull_request_reviews rule DELETED during rc.1 release ritual; main currently unprotected | P0 | rc.1 cut 2026-04-29 | v1.0-brownfield-backfill | S-5.07 | v1.0.0 GA (S-5.07) |
| TD-001 | Phase 5 deferred | BC-level CAP/DI/Stories anchoring incomplete: all 1,851 BC files carry CAP-TBD/DI-TBD/Stories-TBD defaults from Phase 1.4b migration | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-002 | Phase 5 deferred | BC-INDEX status column all "draft" regardless of shipped/partial/pending reality | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-003 | Spec drift | BC frontmatter lacks per-BC lifecycle_status field; PRD claims FR-level status but BCs have no per-BC marker | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |
| TD-004 | Phase 5 deferred | BC-7.01 family is mixed (multiple hooks); FR-032 BC-group labeling conflicts with BC-7.01.001 H1 (block-ai-attribution vs protect-secrets) | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-005 | Phase 5 deferred | Agent registry missing (34 agents not enumerated); NFR-PERF not in PRD §4.2 top-5 | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |
| TD-006 | Process gap | validate-consistency Check 8/9 ship as procedural spec only — no executable runner, no bats/Rust tests, Rust-only language scope, bypassed TDD (no test-writer/implementer dispatch), and was authored directly on main instead of feature-branch-off-develop | P1 | post-Wave-9 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-007 | Spec deferral (narrowed 2026-05-04) | S-3.04 AC-003 ONLY (bash bin/emit-event retirement) — the sole S-3.04 AC genuinely deferred to v1.1. Authoritative trackers: S-10.08 (Wave 4 bash-hook migration), S-10.09 (Wave 5 retirement), TD-014 (full bash-hook native-WASM workstream). This TD is now a breadcrumb only; closes when those land. | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | S-3.04 → S-10.08 + S-10.09 | v1.1 |
| TD-008 | Process gap | S-4.10 RED gate test pattern: emission tests created channel (tx, rx) but never passed tx to sink; required test-writer fix burst. Lessons-codification candidate for S-7.03 update — RED gate must wire all dependencies the implementer is expected to use. | P2 | Wave 12 | v1.0-brownfield-backfill | S-4.10 | v1.0.1 |
| TD-009 | Process gap | Pre-flight git-diff check before merging release/develop branches — caught at L2 risk in this cycle. Should be codified as a process check in CONTRIBUTING.md or pre-merge lint hook. | P2 | Wave 12 | v1.0-brownfield-backfill | — | v1.0.1 |

### Source Types

| Source | Detection Agent | Description |
|--------|----------------|-------------|
| Phase 5 deferred | adversary | Finding deferred as "fix later" from adversarial review |
| Phase 6 deferred | formal-verifier | Finding deferred from formal hardening |
| Spec drift | spec-steward | BC postcondition not enforced in code |
| Dependency | security-reviewer | Major version bump available or vulnerability |
| DTU fidelity | dtu-validator | Real API changed, clone is stale |
| Pattern inconsistency | code-reviewer | Legacy pattern in older code |
| Holdout decay | holdout-evaluator | Scenario tests removed/changed feature |
| Maintenance sweep | consistency-validator | Anti-pattern or code smell detected |

### Item Details

#### TD-001 — BC-level CAP/DI/Stories anchoring incomplete
**Source:** Phase 1d pass 1 (F-003, F-005, F-011)
**Description:** All 1,851 BC files have `capability: CAP-TBD`, `L2 Domain Invariants: TBD`,
`Stories: TBD` — best-effort default from Phase 1.4b migration. PRD §8 anchors at FR-level;
BC-level reverse anchoring is incomplete.
**Severity:** P2 (does not block v1.0 GA — PRD has full traceability; per-BC anchoring
is a navigability improvement).
**Plan:** Wave-scale follow-up. After 3-clean-pass adversarial convergence on the spec
package, run a backfill burst: for each BC, read PRD §7 traceability matrix → assign
CAP, lookup DI from L2, populate Stories from STORY-INDEX.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-002 — BC-INDEX status column all "draft"
**Source:** Phase 1d pass 1 (F-006)
**Description:** All 1,851 BC-INDEX rows show status=draft regardless of whether the
underlying behavior is shipped, partial, or pending. Status should reflect the
implementation reality (22 stories merged + 4 partial + 15 draft).
**Severity:** P2 (PRD FR-level status is correct).
**Plan:** Same backfill burst as TD-001 — derive BC status from STORY-INDEX status of
the implementing story.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-003 — BC frontmatter lacks per-BC `lifecycle_status` field
**Source:** Phase 1d pass 1 (F-011)
**Description:** PRD claims FR-level partial/shipped/pending status but BC files have
no per-BC lifecycle marker (only the `lifecycle_status: active` from the template).
**Severity:** P3 (covered by BC-INDEX status column once TD-002 resolves).
**Plan:** Either schema decision (add field) or rely on BC-INDEX as the status authority.
**Cycle estimate:** v1.1.

#### TD-004 — BC-7.01 family is mixed; FR-032 BC-group labeling is ambiguous
**Source:** Phase 1d pass 1 (F-013)
**Description:** BC-7.01 family contains BCs for multiple hooks (block-ai-attribution in
BC-7.01.001, protect-secrets in BC-7.01.004, capture-commit-activity in BC-7.01.002,
regression-gate in BC-7.01.003). PRD FR-032 labels the BC-7.01 range as "protect-secrets.sh"
which conflicts with BC-7.01.001 H1 ("block-ai-attribution"). The actual alpha-sort order
of SS-07 hooks assigns block-ai-attribution as alphabetically first.
**Severity:** P2 (spec navigability; does not affect implementation).
**Plan:** Rationalize SS-07 BC family assignments: re-shard so each BC family maps to
exactly one hook script. Update PRD FR-032..FR-034 BC-group listings to match.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-005 — Agent registry and NFR-PERF top-5 not enumerated in spec
**Source:** Phase 1d pass 1 (F-016, F-017)
**Description:**
- F-016: 34 agents dispatched by the factory are not enumerated in any formal registry. An `agents.md` under `domain-spec/` would list each agent with its role, model tier, and tool access.
- F-017: PRD §4.2 Top-5 Priority NFRs does not include any NFR-PERF (performance) entry. At least one NFR-PERF (e.g., sub-100ms hook latency per DI-011/BC-7.02.005) should appear in the top-7 or be explicitly excluded with rationale.
**Severity:** P3 (docs/navigability).
**Pre-requisite note:** Reconcile PRD line 75 + ARCH-INDEX line 96 (34 agents) vs former SS-05 line 25 (33 agents). [Resolved in phase-1d pass-2 fix burst — SS-05 now says 34 specialist sub-agents.]
**Plan:** v1.1 — add `domain-spec/agents.md` registry; expand PRD §4.2 to top-7 or add NFR-PERF exclusion note.
**Cycle estimate:** v1.1.

#### TD-006 — validate-consistency Check 8/9 missing executable predicate, tests, and proper TDD trail
**Source:** User audit of Task #114 (post-deliverable review, 2026-04-27)
**Description:** Task #114 extended `plugins/vsdd-factory/skills/validate-consistency/SKILL.md` with two new advisory checks (Check 8 — Test Tautology Detector, MEDIUM; Check 9 — BC Canonical TV ↔ Emitter Field Consistency, HIGH) and registered POLICY 11 + POLICY 12 as opt-in promotion paths. The deliverable is procedural-spec grade, not code grade:

1. **No executable runners.** No `bin/check-tautology` or `bin/check-bc-tv-consistency` script exists. The check fires only if a human (or LLM following SKILL.md) manually performs the procedure. The skill cannot be invoked end-to-end.
2. **No tests.** Zero bats / Rust unit / integration tests exist. The 9 fixture files under `fixtures/tautology/` and `fixtures/bc-tv-consistency/` are illustrative pseudo-code (each carries `FIXTURE: this file is NOT compiled`). Nothing asserts that the predicate actually flags `flagged_*.rs` and ignores `clean_*.rs`.
3. **Rust-only language scope.** The detection regex (`rg --type rust`), the production-fn pattern (snake_case Rust idioms), and the skip semantics (`#[serde(skip_serializing_if)]`, `Option<T>`) are all Rust+serde-specific. For TypeScript/Python/Go/etc projects, both checks no-op and provide zero value. The SKILL.md acknowledges this with a "skipped (no Rust sources detected)" line but does not propose a generalization path.
4. **Bypassed TDD.** Task #114 was executed directly by the orchestrator in auto mode without dispatching test-writer (RED gate) or implementer (GREEN cycle). No story spec, no BC anchor, no per-AC demo evidence, no PR review.
5. **Git Flow violation.** Plugin source edits were authored directly against `main` working tree instead of branching from `develop` per the Git Flow process established in Task #33. Subsequently moved to feature branch + PR (this remediation), but the process slip should be recorded.

**Severity:** P1 — the skill claims two new checks in its public contract (SKILL.md + policy registry references them) but they're not actually runnable. This is "promised behavior, no implementation" — same drift class as the BC-vs-emitter pattern Check 9 itself targets.

**Plan:**
1. **Story spec** — write `S-X.YY-validate-consistency-tautology-bc-tv.md` anchoring to a real or candidate BC; trace ACs to fixture-driven test outputs.
2. **Test harness** — add bats tests under `plugins/vsdd-factory/tests/skills/validate-consistency/` that:
   - run a `bin/check-tautology` prototype against `fixtures/tautology/` and assert: 2 flags from `flagged_tautological_test.rs`, 0 from each `clean_*` file, opt-out exception honored.
   - run `bin/check-bc-tv-consistency` against `fixtures/bc-tv-consistency/` and assert flagged BC↔emitter pair surfaces 1 finding, clean Option-skip pair surfaces 0, `tv_emitter_check: skip` BC is skipped entirely.
3. **Executable runner** — implement `bin/check-tautology.sh` and `bin/check-bc-tv-consistency.sh` as bats-driven Rust-only probes. Emit JSON-Lines per finding so consistency-validator can splice into the Advisories section.
4. **Language scope decision** — record an ADR ("Check 8/9 are Rust-only by design — generalization to TS/Python is a future story") OR generalize via per-language matchers (Rust now, TS next, etc.). User to decide.
5. **Run through proper TDD** — test-writer first (RED gate), then implementer (GREEN), then demo-recorder (per-AC evidence), then PR via pr-manager. Per per-story-delivery cycle.

**Cycle estimate:** v1.0.1 (PATCH if Rust-only; MINOR if generalized to multi-language).

**PR reference:** PR #17 (`feat/validate-consistency-tautology-bc-tv` → `develop`) — ships the procedural spec + 9 fixtures. Merge of #17 does NOT close TD-006; the runner+tests follow-up does.

**Pre-decision questions (RESOLVED 2026-04-27 — story-writer can now proceed):**

Q1 + Q5 (blocking pivotals) answered by user. Q2/Q3/Q4/Q6/Q7/Q8 defaulted to the recommended values per session 2026-04-27. Story spec for TD-006 follow-up is unblocked.

##### Q1 — Language scope ✅ ANSWERED: Path B (all four languages)

**Decision:** Runner supports **Rust + Python + TypeScript + Golang** in the initial story (no phasing). The skill becomes universal across vsdd-factory's full language footprint.

**Implications:**
- Per-language **test-fn matchers**: Rust `fn test_*`, Python `def test_*`, TS `test(...)/it(...)/describe(...)`, Go `func Test*`.
- Per-language **production-fn patterns**: Rust snake_case, Python snake_case, TS camelCase + class methods, Go PascalCase exported + camelCase unexported.
- Per-language **struct/type matchers**: Rust `struct`/`enum`, Python dataclass/pydantic-`BaseModel`/attrs, TS `interface`/`class`/`type`/`zod schema`, Go `type X struct`.
- Per-language **serialization opt-out conventions**:
  - Rust: `#[serde(skip_serializing_if = ...)]`, `Option<T>`, `#[serde(skip_serializing)]`.
  - Python: `field(default=None, repr=False)`, `Optional[T]`, pydantic `Field(exclude=True)`, `model_dump(exclude={...})`, `attrs.field(metadata={"serialize": False})`.
  - TS: `class-transformer @Exclude()`, `class-transformer @Expose({groups: [...]})`, `zod .optional()`, JSON-omit via `omit` utility, `?` field marker.
  - Go: `json:"-"` tag, `json:",omitempty"`, pointer types (`*T`) with nil-handling, custom `MarshalJSON`.
- **Language detection**: file extension primary (`.rs`, `.py`, `.ts`/`.tsx`/`.mts`/`.cts`, `.go`); content sniffing for ambiguous cases (e.g., `.d.ts` declaration-only files skipped).
- **Test fixtures**: 4 flagged + 4 clean per check per language = ~32 new fixture files (current 9 → ~41 total).
- **No skip path** for any of the four languages. The "skipped — no Rust sources detected" advisory line is removed entirely.

**Sizing impact:** ~3× the Path C estimate. See Q7 update below.

**Out of scope (this story):** Java, Kotlin, Ruby, C#, PHP, Swift, Elixir, etc. Treat those as future stories if the user adds projects in those languages later.

##### Q2 — File path concretization ✅ DEFAULTED

**Decision (recommended default accepted):**

- **Runners:** `plugins/vsdd-factory/skills/validate-consistency/runners/check-tautology` and `runners/check-bc-tv-consistency` (skill-local). Implementation language: bash dispatcher + per-language matcher modules (Rust matcher in awk/ripgrep, Python matcher in Python AST, TS matcher in node + ts-morph or tree-sitter, Go matcher in Go's `go/ast`). Each language matcher emits the same JSON-L schema so the dispatcher concatenates without translation.
- **Bats tests:** `plugins/vsdd-factory/skills/validate-consistency/tests/check-tautology.bats` and `tests/check-bc-tv-consistency.bats` (skill-local, co-located).
- **Fixtures:** keep current `fixtures/tautology/` and `fixtures/bc-tv-consistency/` directories. Reorganize to per-language subdirectories: `fixtures/tautology/rust/`, `fixtures/tautology/python/`, `fixtures/tautology/typescript/`, `fixtures/tautology/golang/`. Same for `bc-tv-consistency`.
- **Expected-output fixtures:** `fixtures/tautology/<language>/expected/<fixture-name>.expected.jsonl` (one expected file per input fixture).
- **Language matcher modules:** `plugins/vsdd-factory/skills/validate-consistency/runners/matchers/rust.sh`, `matchers/python.py`, `matchers/typescript.ts`, `matchers/golang.go`.

##### Q3 — Output schema ✅ DEFAULTED

**Decision (recommended default accepted; extended for multi-language):**

```jsonl
{"check": "VC8", "policy_id": "POLICY-VC-008", "severity": "MEDIUM", "language": "rust", "file": "crates/foo/tests/bc.rs", "line": 42, "function": "test_BC_3_02_001_emits_entry", "evidence": "...", "suggestion": "...", "finding_id": "VC8-rust-<sha1-of-file-line-fn>", "opted_out_via": null}
{"check": "VC9", "policy_id": "POLICY-VC-009", "severity": "HIGH", "language": "rust", "bc": "BC-3.02.013", "field": "trace_id", "struct": "LogEntry", "location": "crates/sink-core/src/entry.rs:24", "evidence": "...", "suggestion": "...", "finding_id": "VC9-<sha1-of-bc-field-struct>", "opted_out_via": null}
```

**Required fields:** `check`, `policy_id`, `severity`, `language`, `file` (or `location` for VC9), `evidence`, `suggestion`, `finding_id`.

**Optional fields:** `opted_out_via` (string when opt-out fires; null otherwise — emitted in a separate "skipped" stream so reviewers can see what was deliberately silenced).

**Suppression file:** `plugins/vsdd-factory/skills/validate-consistency/.suppressions/<finding_id>.md` (per-finding rationale doc). Suppression file presence at runtime causes the runner to drop the finding from the output stream but log it to `.suppressed.jsonl` for audit.

##### Q4 — Edge cases (mandatory for story-spec Edge Cases table)

The story spec's Edge Cases table needs these enumerated. Provisional list:

| ID | Scenario | Expected behavior |
|----|----------|-------------------|
| EC-001 | BC frontmatter missing `target_module:` | Skip the BC; emit advisory `VC9-skipped-no-target-module` once |
| EC-002 | BC has `## Canonical Test Vectors` but no excluded-field columns | Skip the BC silently |
| EC-003 | Rust file has `#[cfg(test)]` but zero matching test fns | Skip the file silently |
| EC-004 | Empty fixture directory | Runner exits 0 with empty output |
| EC-005 | Multiple structs with same name in different crates | Resolve via BC `target_module:` path; if ambiguous, emit warning + skip |
| EC-006 | `Option<T>` without `skip_serializing_if` but inside an explicit `Serialize` impl | Currently FLAG; story should decide whether to inspect impl block |
| EC-007 | BC frontmatter has `tv_emitter_check: skip` but no `## Canonical Test Vectors` section | Skip silently (directive applies vacuously) |
| EC-008 | Test fn name matches pattern but has zero body lines | Skip (vacuous tautology, not interesting) |
| EC-009 | Production fn called as a method (`x.emit_event()`) instead of as a free fn | Currently regex would miss; story should decide whether to broaden pattern |
| EC-010 | Workspace has no Rust at all (TS/Python project) | Runner emits one advisory `Check 8/9: skipped (no Rust sources detected)` and exits 0 |

Recommended default: accept the 10 above as the baseline; story-writer adds any others discovered during test-design.

**Decision:** ✅ DEFAULTED. Story-writer accepts the 10 baseline edge cases plus per-language additions:

- **EC-011** (Python-specific): pydantic v1 `class Config: ...` opt-out vs pydantic v2 `model_config = ConfigDict(...)` opt-out — both must be honored.
- **EC-012** (TS-specific): TypeScript `interface` (compile-time only, no runtime) vs `class` (runtime) — interface fields cannot be runtime-skipped, so Check 9 must distinguish.
- **EC-013** (Go-specific): exported field (`PascalCase`) without `json:"-"` tag vs unexported field (`camelCase`) which is implicitly excluded by `encoding/json` — Check 9 must not flag unexported fields.
- **EC-014** (cross-language): a polyglot project with Rust + Python — per-language results must be aggregated into one JSON-L stream with `language` field disambiguating.

##### Q5 — Self-application policy promotion ✅ ANSWERED: Gating (immediate)

**Decision:** vsdd-factory promotes POLICY 11 + POLICY 12 from opt-in to **active gating** in its own `.factory/policies.yaml` as part of TD-006 closure. Adversary fails the gate if vsdd-factory's own tests are tautological or its BC TVs drift from emitters.

**Implications:**

1. **Legacy-finding triage is part of TD-006's scope.** Before the gate can pass, every legacy finding the runner surfaces in vsdd-factory's existing code/BCs must be either fixed (preferred) or formally suppressed via `.suppressions/<finding_id>.md` with explicit rationale.
2. **Forcing function on Phase 5 / future adversarial reviews.** Once gating is on, no spec or implementation change can pass adversarial review without clearing both checks. This is the strongest dogfooding stance — it explicitly accepts the cost.
3. **Volume unknown until runner exists.** vsdd-factory has 1,893 BCs (many brownfield-extracted, may not have Canonical Test Vectors sections) and a Rust workspace with tests under various naming conventions. The actual legacy-finding volume could be 0 (if naming/TV conventions don't match the matchers) or hundreds (if they do). The story spec must include a triage budget that allows for both extremes.
4. **Suppression policy.** Suppressions are explicit, time-bounded, and attached to a TD entry (e.g., suppression cites `TD-NNN`). No silent silencing.
5. **Sequencing.** PR for TD-006 follow-up lands the runner + tests + per-language matchers in one PR; a SECOND PR (or commit-in-same-PR if scope allows) flips POLICY 11/12 to gating in `.factory/policies.yaml` AFTER triage is complete.

**Risk acknowledgment:** If legacy-finding volume is high (>50), TD-006 expands into a multi-week effort. The story spec must explicitly handle "stop, triage as TD-NNN, suppress, gate" as a fallback path so a finding storm doesn't block all forward progress.

##### Q6 — NFRs / performance budgets ✅ DEFAULTED (adjusted upward for multi-language)

**Decision (recommended defaults accepted; latency budget adjusted for 4 languages):**

- **Latency budget:** Check 8 < 30s for the full 1,893-BC repo with Rust + Python + TS + Go matchers running. Check 9 < 30s. (Previous Rust-only budget was <10s; multi-language tripling accounts for tree-sitter / AST parsing in non-Rust languages.)
- **Cacheable?** No cache in v1. Premature optimization given the budget.
- **Parallel-safe:** Yes by construction (runners are read-only).
- **Side-effect free:** Yes — no writes anywhere, no `.factory/` mutation, no fixture rewriting.
- **Per-language sub-budgets:** Rust matcher < 5s, Python matcher < 8s (AST parsing), TS matcher < 12s (tree-sitter or ts-morph node startup is the dominant cost), Go matcher < 5s. Total dispatcher orchestration overhead < 5s.
- **Failure mode:** If any per-language matcher exceeds its budget, runner emits `<lang>-matcher-timeout` advisory and skips that language for the run. Other matchers continue. The dispatcher exits 1 only if ALL matchers fail; partial coverage exits 0 with the advisory.

##### Q7 — Story sizing ✅ UPDATED for Path B + Gating

**Decision:** Sizing now reflects Q1=Path B (4 languages) + Q5=Gating (legacy triage included).

**Initial-build phase (story-writer + test-writer + implementer + demo-recorder):**

| Phase | Activity | Days |
|-------|----------|------|
| Story spec | Write S-X.YY-validate-consistency-checks-runner.md with 4-language scope | 1.0 |
| BC creation | Pre-emptive BCs for Check 8/9 promised behavior (BC-6.NN.NNN candidates) | 0.5 |
| Test-writer (RED) | 4 langs × 2 checks × ~4 fixtures × expected-output files = ~32 fixture+expected pairs + bats harness | 2.0 |
| Implementer (GREEN) | Per-language matchers (Rust+Python+TS+Go) + dispatcher + JSON-L output + edge cases | 4.0 |
| Demo-recorder | Per-AC evidence across all 4 languages | 1.0 |
| Adversarial pass-1..N | Spec + impl convergence (3 clean passes) | 1.5 |

**Subtotal (initial build):** ~10 days.

**Legacy-finding triage phase (Q5 gating prep — variable, scope-dependent):**

| Volume scenario | Triage days |
|-----------------|-------------|
| 0–10 findings (best case) | 0.5 |
| 11–50 findings (likely) | 2.0 |
| 51–200 findings (storm) | 5–10 (becomes a sub-cycle of its own; may split into multiple TD entries) |
| 200+ findings (catastrophic) | Reassess: defer Q5 gating to v1.1, ship advisory-only first |

**Recommended budget:** Plan for 11–50 (likely), reserve story-spec language to allow re-scope to advisory-first if volume exceeds 200. Triage outputs are TD-NNN entries with rationale-suppressed findings.

**Promotion phase (POLICY 11/12 → gating):**

| Phase | Activity | Days |
|-------|----------|------|
| Policy edit | Flip `enforced_by:` lists to include adversary-prompt + lint-hook in `.factory/policies.yaml` | 0.25 |
| Hook integration | Add validate-consistency invocation to pre-PR CI hook | 0.5 |
| First-gate-pass run | Run gating against `develop` HEAD, confirm clean (or all suppressions doc'd) | 0.25 |

**Subtotal (gating promotion):** ~1 day.

**Total estimate:** **12–14 days** (best case 0–10 legacy findings) to **17–22 days** (likely 11–50 findings). **Points: 21** (best case) to **34** (likely). **Effort: XL.**

**Story splitting recommendation:** This is too big for a single story spec. Recommend decomposing into sub-stories:

| Story | Scope | Points |
|-------|-------|--------|
| S-X.01 | Runner architecture + dispatcher + Rust matcher + Rust fixtures + bats | 8 |
| S-X.02 | Python matcher + Python fixtures + bats | 5 |
| S-X.03 | TypeScript matcher + TS fixtures + bats | 5 |
| S-X.04 | Golang matcher + Go fixtures + bats | 5 |
| S-X.05 | Legacy-finding triage on vsdd-factory's own code | 3–13 (volume-dependent) |
| S-X.06 | POLICY 11/12 promotion to gating + CI hook integration | 3 |

**Total decomposed: 29–39 points across 6 stories.** Each story can converge independently before the next starts. S-X.05 is the unknown-volume gate before S-X.06 can land.

##### Q8 — Definition of Done ✅ FINALIZED for Q1=Path B + Q5=Gating

**TD-006 closes when ALL of the following are true:**

- [ ] **Story specs merged.** S-X.01 through S-X.06 all merged into `develop` with their PRs.
- [ ] **All bats tests pass.** Per-language fixtures (Rust + Python + TS + Go) — `flagged_*` yields expected findings, `clean_*` yields zero, all opt-outs honored, all 14 edge cases covered.
- [ ] **CI integration live.** validate-consistency runs as a pre-PR hook on every PR targeting `develop`. Hook exit code blocks merge on findings (post-promotion).
- [ ] **Legacy-finding triage complete.** Every finding from running the runner against vsdd-factory's own `develop` HEAD is either: (a) fixed in code/BCs, OR (b) explicitly suppressed via `.suppressions/<finding_id>.md` linked to a TD-NNN entry.
- [ ] **POLICY 11/12 promoted to gating.** `.factory/policies.yaml` `enforced_by:` lists for both policies include `adversary-prompt` + `lint-hook` (currently they include `validate-consistency` + `adversary-prompt`; lint-hook addition is the gate-flip).
- [ ] **First-gate-pass clean.** Adversarial review run against `develop` HEAD post-promotion confirms zero unsuppressed findings.
- [ ] **ADR documenting language-scope architecture.** ADR records per-language matcher plug-in pattern; explains why Rust+Python+TS+Go are the v1 set; documents extension procedure for future languages (Java/Kotlin/Ruby/etc).
- [ ] **STATE.md D-NNN entry recording closure** with the 6 sub-stories' commit SHAs and a final cumulative-findings count.
- [ ] **Resolution History row appended** to this register with date, sub-story IDs, final outcome.
- [ ] **Pre-flight check codified.** The `git fetch && git log --oneline origin/develop..origin/main && git diff origin/main origin/develop -- <files>` pattern is added to a developer-onboarding doc or a pre-feature-branch lint hook (this is a process gap surfaced during TD-006 remediation).

**Cycle estimate:** v1.0.1 (PATCH if no public-API changes) or v1.1 (MINOR if the runner exposes a new public skill API surface that other plugins can call).

#### TD-008 — S-4.10 RED gate test-wiring gap
**Source:** Wave 12 per-story-delivery cycle test-writer fix burst (2026-04-27)
**Description:** During S-4.10 TDD cycle, the test-writer's RED gate created a channel `(tx, rx)` for sink communication but never passed `tx` to the sink under test. The sink consequently never received events, causing tests to hang or pass vacuously without exercising the intended behavior. A separate test-writer fix burst was required to wire `tx` correctly. This is a lessons-codification candidate for the S-7.03 TDD Discipline Hardening story — the RED gate contract must explicitly require that all dependencies the implementer is expected to use are wired into the test harness before declaring RED.
**Severity:** P2 — does not block any current story; creates risk of similar wiring gaps in future sink test harnesses.
**Plan:**
1. Add an explicit checklist item to the test-writer agent prompt: "Before declaring RED, verify every dependency (channels, handles, clients) passed to the SUT is actually wired and exercised by at least one test assertion."
2. Consider adding a bats fixture pattern showing the correct channel-wiring pattern for sink tests.
3. Candidate for S-7.03 follow-up story or an additional AC in a future process-hardening story.
**Cycle estimate:** v1.0.1.

#### TD-009 — Pre-flight git-diff check before release/develop merges
**Source:** Wave 12 cycle risk triage (2026-04-27)
**Description:** During this cycle, a risk was flagged (L2) that release branches could be merged into develop without a pre-flight diff check, potentially introducing unintended changes. The pattern `git fetch && git log --oneline origin/develop..origin/main && git diff origin/main origin/develop -- <files>` is known (referenced in TD-006 DoD) but not codified as a mandatory process step.
**Severity:** P2 — does not block current work; represents an operational risk for future release cycles.
**Plan:**
1. Add a pre-merge checklist section to `CONTRIBUTING.md` documenting the pre-flight diff command.
2. Alternatively, implement as a pre-merge lint hook that runs on PRs targeting `main` or release branches.
3. Coordinate with TD-006 closure (Q8 pre-flight check item) to avoid duplicating the codification.
**Cycle estimate:** v1.0.1.

#### TD-007 — S-3.04 AC-003 deferred: bash bin/emit-event retirement (NARROWED 2026-05-04)
**Source:** S-3.04 spec body explicit deferral note (v1.0.0-beta.4 ship); confirmed by post-Wave-9 status-drift audit 2026-04-27. Scope narrowed by Q7 disposition investigation 2026-05-04.

**Current scope (post-narrowing):** ONLY AC-003 of S-3.04 — "bin/emit-event deprecated; callers migrated." This is the sole S-3.04 AC genuinely deferred to v1.1; the AC-001/AC-002 falsely-shipped concerns are now owned by E-10 + LESSON-2026-05-04-001 (see Historical Context below).

**Description:** AC-003 was carved out of v1.0 scope because 30 bash hooks under `plugins/vsdd-factory/hooks/*.sh` still call `bin/emit-event`. Full retirement requires migrating all bash hook callers to route through the dispatcher's `host::emit_event` (via legacy-bash-adapter) and then physically removing `bin/emit-event` from the dispatcher binary tree.

**Severity:** P3 — does not block v1.0 GA. The host fn IS implemented and works for native WASM plugins; only legacy bash hooks still use the old binary. Both code paths coexist during the migration window.

**Plan:** TD-007 is now a breadcrumb. The actual implementation work is owned by:
- **S-10.08** (Wave 4 bash-hook parity) — migrate bash callers through `legacy-bash-adapter` to `host::emit_event`; deprecate `bin/emit-event` with runtime warning at the end of Wave 4.
- **S-10.09** (Wave 5 crate retirement + SS-03 rewrite) — physical removal of `bin/emit-event` after Wave 4's migration completes.
- **TD-014** (full native WASM migration of remaining 43 bash hooks) — the broader bash-hook retirement workstream within which `bin/emit-event` removal lives.

**Closure criteria:** Close TD-007 when ALL THREE land:
1. S-10.08 ships (bash hooks migrated, runtime warning emitted)
2. S-10.09 ships (`bin/emit-event` physically removed)
3. TD-014 closes (all bash hooks retired or routed)

**Cycle estimate:** v1.1.

##### Historical Context (preserved from 2026-05-04 amendment)

TD-007 was previously amended with a note about S-3.04 AC-001 ("emit_event() routes events to configured sinks") having been falsely marked shipped in v1.0.0-beta.4. That concern is **NOT** in TD-007's scope as of the 2026-05-04 narrowing — it is fully owned by:

- **S-10.02** (Wave 1 FileSink single-stream wiring) — the implementation fix for the false-shipped routing
- **LESSON-2026-05-04-001** (`.factory/cycles/v1.0-brownfield-backfill/lessons.md`) — the codified process gap (story shipped without end-to-end AC verification)
- **S-7.04** (universal-discipline AC-test-link enforcement) — the systemic delivery-discipline gap that allowed the false-ship to occur

The unimplemented Router::submit integration at `crates/factory-dispatcher/src/sinks/mod.rs:11-15` is being retired (not fixed) by ADR-015's single-stream architecture; Router/SinkRegistry are deprecated under ADR-015. This historical note is preserved here so auditors following S-3.04's "DEFERRED — see TD-007" annotation understand the full disposition chain.

#### TD-010 — vsdd-hook-sdk-macros not published to crates.io (AC-2 of S-4.08 deferred)
**Source:** rc.1 release-prep burst (2026-04-29)
**Description:** AC-2 of S-4.08 requires `cargo publish --dry-run` to pass for `vsdd-hook-sdk-macros`. This AC cannot be satisfied until `vsdd-hook-sdk-macros` is published to crates.io (the dry-run exercises publish readiness including registry resolution). rc.1 does not publish to crates.io — the tag is a release candidate for shakedown, not a GA publish. The AC is therefore deferred. Block on v1.0.0 GA cut: the GA release MUST publish `vsdd-hook-sdk-macros` to crates.io and verify AC-2 passes.
**Severity:** P1 — directly blocks a named AC in a shipped story spec; must be resolved before v1.0.0 GA.
**Plan:**
1. Publish `vsdd-hook-sdk-macros` to crates.io as part of v1.0.0 GA release procedure.
2. Run `cargo publish --dry-run -p vsdd-hook-sdk-macros` in the GA release checklist.
3. Verify AC-2 of S-4.08 passes; close TD-010 in STATE.md.
**Cycle estimate:** v1.0.0 GA.

#### TD-011 — check-changelog-monotonicity.sh strict-`<` policy rejects same-day beta.6/beta.7 entries
**Source:** rc.1 release-prep burst (2026-04-29)
**Description:** `scripts/check-changelog-monotonicity.sh` enforces strict `<` ordering on CHANGELOG dates. Pre-existing data has same-day beta.6 + beta.7 entries (both 2026-04-26) which the script rejects with a non-zero exit. The script is informational; `release.yml` does not invoke it and rc.1 is not blocked. However, the policy diverges from real-world release patterns where same-day releases are valid. Either (a) loosen to `<=` (allow same-day) and update the rationale, or (b) add ISO-8601 timestamps to CHANGELOG entries so strict ordering is preserved.
**Severity:** P3 — not blocking rc.1; informational script only; no CI gate depends on it.
**Plan:**
1. Decide between option (a) `<=` loosening or option (b) timestamp augmentation.
2. Update `check-changelog-monotonicity.sh` accordingly.
3. Verify script passes on current CHANGELOG before closing.
**Cycle estimate:** v1.0.1 or sooner.

#### TD-012 — 9 pre-existing bats test failures investigation (allowlist-masked)
**Source:** rc.1 release-prep burst (2026-04-29)
**Description:** `run-all.sh` allowlist masks 9 pre-existing bats failures as PASS at runner level, but raw `bats plugins/vsdd-factory/tests/` reports 9 of 1316 fail. Failure categories: worktree-missing, registry-generator, novelty-assessment. devops-engineer's rc.1 prep run reproduced identical 9 fails — confirmed pre-existing, not a Wave 11/12/13/14 regression. The allowlist justification may or may not be documented per-test. Risk: future failures in the same categories could be silently masked.
**Severity:** P2 — allowlist protects CI green; but undocumented masking creates audit gap and regression-detection risk.
**Plan:**
1. Enumerate the 9 failing test names from raw bats output.
2. For each: verify the allowlist entry has an explicit justification comment in `run-all.sh` or a linked issue.
3. If justification is missing, add it (with TD-012 reference) OR remediate the test.
4. Add a process note: any new allowlist entry MUST include a justification comment citing a TD or issue.
5. Close TD-012 when all 9 entries have documented justification or are remediated.
**Cycle estimate:** v1.0.0 GA or v1.0.1.

#### TD-013 — Restore main branch protection with proper bot bypass before v1.0.0 GA
**Source:** rc.1 cut on 2026-04-29 — bot push to main was blocked by `required_pull_request_reviews` even after disabling `enforce_admins`. Workaround was to DELETE the rule entirely; user authorized "leave it loose for now."
**Severity:** P0 (security: main is currently unprotected)
**Due:** v1.0.0 GA cut (S-5.07)
**Current state:** No PR-review requirement on main. `enforce_admins` disabled. Effectively any commit can be pushed to main directly.

**Root cause** (per research-agent on 2026-04-29):
- GitHub `bypass_pull_request_allowances` field on legacy branch protection is silently org-gated. PATCH attempt returns HTTP 500 with empty body (vs documented 422). Confirmed via Community Discussion #29771.
- Modern Rulesets API (`bypass_actors[]` with `actor_type: Integration`, `actor_id: 15368` for github-actions app) ALSO requires org ownership for actual enforcement.
- Web UI Settings → Branches → main shows no bypass option on user-owned repos (Discussion #29771 staff confirmation).
- **Bot identity correction:** github-actions APP id is `15368` (use as Rulesets `actor_id`); github-actions[bot] USER id is `41898282` (NOT for bypass).

**Why beta.7 (2026-04-26) worked but rc.1 (2026-04-29) didn't:** Most likely `required_approving_review_count` was raised from 0 to 1 on the project side between those dates. No GitHub changelog change in that window.

**Three remediation paths** (research-agent recommendation, ranked):

1. **Migrate `vsdd-factory` to a free GitHub organization** (5-minute user action). After transfer, configure modern Ruleset:
   ```json
   {
     "name": "main-protection",
     "target": "branch",
     "enforcement": "active",
     "bypass_actors": [
       {"actor_type": "Integration", "actor_id": 15368, "bypass_mode": "always"}
     ],
     "conditions": {"ref_name": {"include": ["refs/heads/main"], "exclude": []}},
     "rules": [
       {"type": "pull_request", "parameters": {"required_approving_review_count": 1}}
     ]
   }
   ```
   Then default GITHUB_TOKEN works in release.yml without ritual. **Recommended.**

2. **Convert release.yml `commit-binaries` job to bot-opens-PR + auto-merge pattern.** Bot creates a `release-binaries-bundle/v1.0.0-rc.N` branch, opens PR, sets `--auto-merge`. Survives org migration. What release-please / goreleaser / semantic-release do.

3. **Toggle-protection-around-release ritual** (current state, but with rule re-enabled when not releasing). Brief unprotected window per release; manual ritual; racy.

**Recommended fix path:** Path (1) before next release. Optionally add Path (2) as v1.0.0 GA hardening to remove all toggle rituals.

**Citations:**
- GitHub Community Discussion #29771 — bypass actors org-only
- GitHub Community Discussion #25305 — chrispat staff explanation
- GitHub REST API docs — Branch protection / Rules
- GitHub Changelog 2025-09-10 — Ruleset exemptions (`bypass_mode: always` vs `exempt`)
- api.github.com/apps/github-actions — verified app_id 15368

**Cycle estimate:** v1.0.0 GA (must resolve before cutting GA tag).

#### TD-014 — Full native WASM migration of remaining bash hooks (post-v1.0)

**Source:** User audit of hook migration completeness post-rc.1 (2026-04-30)
**Severity:** P2 (no blocker for v1.0 GA; affects post-1.0 cross-platform parity)
**Due:** v1.1+ epic decomposition; planning starts post-rc.1 shakedown
**Discovered:** 2026-04-30 during user audit of hook migration completeness post-rc.1

**Current state (verified against `plugins/vsdd-factory/hooks-registry.toml` + `plugins/vsdd-factory/hooks/`):**

- **Native WASM (8 plugin crates in `crates/hook-plugins/`):**
  - capture-commit-activity (S-3.01 PR #20) — port from bash
  - capture-pr-activity (S-3.02 PR #21) — port from bash
  - block-ai-attribution (S-3.03 PR #22) — port from bash
  - legacy-bash-adapter (S-3.04) — compat layer (not a port itself)
  - session-start-telemetry (S-5.01 PR #35) — new lifecycle hook
  - session-end-telemetry (S-5.02 PR #36) — new lifecycle hook
  - worktree-hooks (S-5.03 PR #37) — new lifecycle hook (handles WorktreeCreate + WorktreeRemove)
  - tool-failure-hooks (S-5.04 PR #38) — new lifecycle hook
- **Bash routed via dispatcher → legacy-bash-adapter (8 unique scripts; 44 [[hooks]] entries in registry):**
  1. handoff-validator
  2. pr-manager-completion-guard
  3. regression-gate
  4. session-learning
  5. track-agent-stop
  6. update-wave-state-on-merge
  7. validate-pr-review-posted
  8. warn-pending-wave-gate
- **Bash wired inline in hooks.json (NOT via dispatcher; ~35 scripts):**
  - ~20 `validate-*.sh` PostToolUse:Edit/Write validators (bc-title, input-hash, finding-format, story-bc-sync, anchor-capabilities-union, count-propagation, demo-evidence-story-scoped, factory-path-root, novelty-assessment, pr-description-completeness, pr-merge-prerequisites, red-ratio, state-index-status-coherence, state-pin-freshness, state-size, subsystem-names, table-cell-count, template-compliance, vp-consistency, wave-gate-completeness, wave-gate-prerequisite, index-self-reference)
  - `protect-bc.sh`, `protect-vp.sh`, `protect-secrets.sh` — PreToolUse protected-file gates
  - `purity-check.sh`, `red-gate.sh`, `convergence-tracker.sh`, `brownfield-discipline.sh`, `factory-branch-guard.sh`, `destructive-command-guard.sh`, `check-factory-commit.sh` — process discipline
  - `track-agent-start.sh` — sibling of track-agent-stop (already routed)
  - `verify-git-push.sh` — git pre-push hook (not Claude Code; used locally)

**Migration scope (proposed):** Port the 43 remaining bash hooks to native WASM. This eliminates the Windows git-bash dependency entirely; legacy-bash-adapter becomes deletable post-migration.

**Prioritization (proposed):**
- **Tier 1 — dispatcher-routed (8 hooks):** highest priority because they affect Windows-native operation TODAY. Already wired through dispatcher; native port substitutes the .wasm cleanly.
- **Tier 2 — PostToolUse validators (~20 hooks):** medium priority. Frequent fire (every file edit). Mostly grep/jq/awk — straightforward Rust ports.
- **Tier 3 — PreToolUse protections + process discipline (~10 hooks):** lower priority. Less frequent fire. Some have non-trivial bash logic worth audit during port.
- **Tier 4 — verify-git-push.sh:** local-only git hook, not Claude Code; possibly out-of-scope (or stay bash forever).

**Dependencies:**
- Hook SDK ABI must remain stable (per S-5.06 v1.0 commitment); v1.1 ports use the same ABI.
- Some validators have heavy file I/O — need to evaluate `read_file` host fn vs. running grep externally vs. embedding regex matchers in Rust.
- PostToolUse hooks need careful matcher migration (currently inline in hooks.json `matcher:` regex; native plugins move matcher logic into the plugin).

**Documentation correction needed:** migration guide (S-5.05) line ~62 says "Other 26 hooks remain on the legacy-bash-adapter" — actual count is ~43 (8 routed + ~35 inline). Either correct the guide or clarify "26" referred to a specific subset (e.g., dispatcher-routed at v0.79 final state). Update before v1.0.0 GA OR roll into a hot-fix release-notes amendment.

**Citations:**
- Verified counts via `grep -c "^\[\[hooks\]\]"` and `find plugins/vsdd-factory/hooks -name "*.sh"` on develop @ `6686aec` (2026-04-30).
- S-3.04 epic (legacy-bash-adapter) for adapter design; planned to be retired when migration completes.
- S-5.06 semver commitment doc: hook-sdk ABI is stable surface (HOST_ABI_VERSION = 1) — v1.1 native ports leverage this.

**Epic spec status (2026-04-30):** E-8 epic spec (E-8-native-wasm-migration.md) CONVERGED at v1.7 status=ready. ADV-E8-P11 CONVERGENCE_REACHED — 11 adversarial passes, 41 substantive + 11 LOW findings closed. Story-writer dispatch unblocked for S-8.00..S-8.28 (29 sub-stories). Full TD-014 closure requires W-15/W-16/W-17 TDD implementation + S-8.28 legacy-bash-adapter retirement. E-8 wave routing pending v1.0.0 GA close (S-5.07).

**Next steps:**
1. Story-writer produces per-story spec bursts for E-8 sub-stories (start with S-8.00 pre-work + Tier 1 dispatcher-routed hooks, ~3pts each).
2. Adversarial spec convergence per story (or batched per tier).
3. TDD implementation with bats tests for each.
4. Track migration progress via "% of bash hooks migrated" metric in STATE.md.

## TD-015 — Per-invocation telemetry correlation: host::invocation_id() SDK extension + schema enrichment + cross-hook sweep

**Severity:** P2 (post-v1.0 enhancement)
**Adopted:** 2026-05-01 (D-181)
**Origin:** S-8.08 pass-5 adjudication (F-S808-P5-001 HIGH bash-parity violation)

### Context

E-8 Tier 1 native WASM port for `track-agent-start` initially specified `agent_id` and `tool_name` event fields not present in the bash source. Pass-5 fresh-context adversary caught the parity violation. Per E-8 D-2 (parity-only port), the additive fields were removed in S-8.08 v1.4. The empirical bash output is: `type=agent.start hook=track-agent-start matcher=Agent subagent=<subagent> [story_id=<...>]` only.

### Use Case (deferred)

If a downstream consumer (dashboard, factory-sla, or analytics sink) emerges that requires correlating "all events from this specific Task invocation" — e.g., "show me every event emitted during the run of this single subagent dispatch" — the current schema cannot satisfy it because `host::session_id()` returns the parent CC session ID (per-launch), not per-invocation identity.

Today the bash hooks emit a stable `subagent` name (e.g., "pr-manager") and an optional `story_id`, but those don't disambiguate multiple sequential invocations of the same subagent within one CC session.

### Scope (when triggered)

This is a post-v1.0 epic with multiple coordinated workstreams:

1. **SDK ABI extension:**
   - Add `host::invocation_id() -> &str` to `crates/hook-sdk/src/host.rs`
   - Dispatcher generates a unique ID (UUID or short hash) at envelope construction time per Task tool invocation
   - HOST_ABI_VERSION may stay at 1 (additive ABI per AS-DEC reasoning, same as S-8.10 host::write_file extension) or bump if non-additive
   - New BC in BC-2.02.x family (host-shim ABI invariants)

2. **Schema BC amendments:**
   - BC-7.03.080 (track-agent-start) postcondition update: emit `invocation_id` field
   - BC-7.03.082 (track-agent-stop) postcondition update: emit `invocation_id` field
   - Possibly BC-7.04.041/043 (pr-manager-completion-guard, validate-pr-review-posted) for cross-event correlation

3. **Cross-hook sweep:** Update emit_event calls in:
   - `track-agent-start` (S-8.08)
   - `track-agent-stop` (S-8.03 — currently CONVERGED at v1.3 ready; would need re-spec)
   - `pr-manager-completion-guard` (S-8.02)
   - `validate-pr-review-posted` (S-8.05)
   - `handoff-validator` (S-8.01) — emits hook.block events, may benefit from invocation_id
   - `regression-gate-adapter-retirement` (S-8.09)

4. **Sink correlation tests:** End-to-end fixture asserting that paired start/stop events carry matching invocation_id values, distinct across multiple subagent dispatches in one session.

### Trigger Criteria

Promote from P2 to P1 when ANY of the following occurs:
- Stakeholder request for a dashboard view requiring per-invocation drill-down
- factory-sla module needs to disambiguate concurrent or sequential same-subagent invocations
- A new hook is added that requires per-invocation tracing for debugging

### Estimated Cost

- SDK extension: 3-5 pts (similar to S-8.10 host::write_file pattern)
- Per-hook BC + spec updates: 2 pts × ~6 hooks = 12 pts
- Sink correlation tests: 5 pts
- Adversarial spec convergence per affected story: ~3-5 passes each
- Total: ~30 pts + 1-2 weeks coordination

### Cross-references

- D-181 (this decision)
- E-8 D-2 (parity-only constraint)
- D-6 Option A (precedent for additive SDK ABI extension via S-8.10 host::write_file)
- AS-DEC for HOST_ABI_VERSION = 1 (additive ABI semantics)
- S-8.08 v1.4 changelog (the parity restoration that opened this debt)

---

## TD-021 — Story-frontmatter vs STORY-INDEX status drift

**Severity:** P3 (v1.1+; pre-existing; non-blocking)
**Adopted:** 2026-05-04 (post-ADR-015 story-housekeeping audit)
**Origin:** Q3 finding from post-ADR-015 audit follow-up burst

### Context

Several story files have stale `status: ready` in their frontmatter while STORY-INDEX correctly tracks them as `merged`. Initial sample: **S-4.06** and **S-4.07** — both were merged to develop on 2026-04-28 (PR #30 at 6ef564c and PR #31 at 1d4edb7 respectively, per STORY-INDEX rows), yet their frontmatter still reads `status: ready`. The drift is pre-existing — not introduced by the ADR-015 supersession annotation work — and likely reflects the convention that the post-merge status flip happens in STORY-INDEX rather than per-story frontmatter.

### Scope (when triggered)

Two viable remediation paths:

1. **CI lint hook:** Add `validate-story-status-vs-index.sh` (analogous to existing `validate-story-bc-sync.sh`) that asserts every story's frontmatter `status:` value matches the STORY-INDEX row for the same story_id. Fails CI if drift detected. Forces both files to stay in sync going forward.

2. **One-time backfill burst:** Walk all story files; for each, look up the STORY-INDEX status; if frontmatter ≠ index, update the frontmatter. Single PR clears existing drift; no enforcement going forward.

The CI lint is preferred (prevents recurrence) but requires enumerating which side is canonical for ambiguous transitions (draft → ready → in-progress → merged); the backfill is cheaper if STORY-INDEX is treated as source-of-truth.

### Trigger Criteria

Promote from P3 to P2 if:
- A future automated metric/dashboard reads frontmatter `status:` (today everything reads STORY-INDEX); drift would mislead the metric
- A story-writer agent uses frontmatter `status:` for routing decisions and the stale value causes incorrect routing

### Estimated Cost

- CI lint hook approach: 2-3 pts (script + bats tests + wiring)
- Backfill-only approach: 1-2 pts (one-shot script + PR)
- Combined approach (lint + initial backfill): 3-5 pts

### Cross-references

- Q3 finding from 2026-05-04 post-ADR-015 audit follow-up burst
- Sample drifted stories: S-4.06, S-4.07 (frontmatter `status: ready`; STORY-INDEX `merged`)
- Related lint hook pattern: `plugins/vsdd-factory/hooks/validate-story-bc-sync.sh`

---

## TD-022 — novelty-assessment hook missing phase-f5 delta-review path

**Severity:** P2 (backlog; hook silently passes unvalidated files)
**Adopted:** 2026-05-04 (TD-020 sweep follow-on, PR #82)
**Origin:** TD-020 sweep surfaced that three novelty-assessment.bats tests described real production behavior that was never implemented in `validate-novelty-assessment.sh`.

### Context

`validate-novelty-assessment.sh` has a case-statement matcher (lines 39-47) that determines which files receive Novelty Assessment validation. The `phase-f5-scoped-adversarial` skill writes its adversarial-delta-review output to `.factory/phase-f5-adversarial/adversarial-delta-review.md` (documented in SKILL.md and step-d-adversary-report.md). That path does NOT match any arm in the case-statement — it falls through to `exit 0`, meaning phase-f5 delta reviews are silently accepted by the hook without Novelty Assessment validation.

This is a pre-existing gap: the hook was not updated when the `phase-f5-scoped-adversarial` skill introduced the `phase-f5-adversarial/` output path. TD-020 deleted the three bats tests that asserted this validation (correct — they asserted behavior the hook does not have), but did not fix the hook (out of TD-020 scope).

### Fix

Add a case arm to `validate-novelty-assessment.sh`:

```bash
  *.factory/phase-f5-adversarial/adversarial-delta-review.md) ;;
```

and add corresponding bats tests in `novelty-assessment.bats`:
- "validates adversarial-delta-review files" (negative path — missing section → exit 2)
- "valid delta review passes" (positive path — complete section → exit 0)

### Estimated Cost

1-2 pts (hook one-liner + 2 bats tests + run-all.sh verification).

### Cross-references

- `plugins/vsdd-factory/hooks/validate-novelty-assessment.sh` lines 39-47 (case-statement)
- `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/SKILL.md` lines 84, 171
- `plugins/vsdd-factory/skills/phase-f5-scoped-adversarial/steps/step-d-adversary-report.md` lines 14, 30
- TD-020 sweep PR #82 (deleted the original asserting tests)

---

## TD-024 — SKIP_SUITES un-skip requires CI-equivalent validation before claiming pass

**Severity:** P2 (process gap; target v1.0.1)
**Adopted:** 2026-05-05 (rc.11 retag post-mortem)
**Origin:** TD-020 sweep follow-on — rc.11 release failure

### Context

TD-020's 2026-05-04 sweep resolved all four `SKIP_SUITES` entries. Two of them — `generate-registry` and `state-health` — were closed as "UN-SKIPPED with no test changes" based on local test runs. Both passed in the developer's local environment. Neither passed in CI during the rc.11 release workflow.

### Impact

rc.11 required two retag rounds (force-delete + re-push tag) before the release workflow went green. Tag finally settled at fb3e297. Two hotfix PRs were required, and the Marketplace PR #5 was delayed until 2026-05-05T03:33:21Z.

**generate-registry failure (external TD-VSDD-054, PRs #85 + #86):**
`scripts/generate-registry-from-hooks-json.sh` used `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json` to read a historical version of `hooks.json`. In a shallow clone (the default for GitHub Actions `checkout`), the parent commit `7b4b774^` is not present in the object store, causing `git show` to fail. Fixed by vendoring the historical file as `scripts/legacy/hooks-json-pre-templating.json` and rewriting the script to use `cat` on the static file. PR #85 was later rebased and re-merged as PR #86 after a branch conflict.

**state-health failure (external TD-VSDD-055, PR #87):**
`state-health.bats` `setup()` calls `git commit` to establish a baseline repository state. In CI runners, no global `user.email` or `user.name` is configured, causing `git commit` to exit 128 ("Author identity unknown"). Fixed by adding `git config user.email "test@example.com"`, `git config user.name "Test"`, and `git config commit.gpgsign false` after the `git init` call in the bats `setup()` function.

### Root Cause

Local test environments carry side-channels that CI runners do not:

- **Global git config** — developer machines always have `user.email`/`user.name` set; CI runners start with empty config
- **Full git history** — developer machines have full `git log`; GitHub Actions `checkout` defaults to `--depth=1` (shallow clone)
- **Operator-installed CLI tools** — local shells may have tools installed beyond what the workflow's `apt`/`brew` block declares
- **Mature shell environment** — local shells inherit years of env var configuration; CI runners start with a minimal env

The TD-020 sweep workflow had no checklist item or validation gate requiring CI-equivalence before declaring an un-skipped suite as passing. "Passes locally" was treated as sufficient evidence, conflating two different validation regimes.

### Resolution Sketch

1. Add a checklist item to the SKIP_SUITES un-skip workflow documentation (agent prompt, CONTRIBUTING note, or both): "Before marking a previously-skipped bats suite as passing, validate it under CI-equivalent conditions: empty global git config; shallow clone with `--depth=1`; no operator-installed CLI tools beyond what the workflow declares; clean shell env."
2. Optionally: add a CI smoke job that runs the full bats suite under a minimal-environment matrix to catch future regressions before they reach the release workflow.

### References

- External TD-VSDD-054 — shallow-clone `git show` history dependency in `generate-registry-from-hooks-json.sh` (PR #85, re-merged as PR #86 after rebase)
- External TD-VSDD-055 — missing local git config in `state-health.bats` setup (PR #87)
- TD-020 — the SKIP_SUITES sweep whose "UN-SKIPPED with no test changes" closure triggered this gap
- Lesson: `.factory/cycles/v1.0-brownfield-backfill/lessons.md` — "TD-020 sweep: un-skipping bats suites without CI-equivalent validation shipped CI regressions"

---

## TD-027 — Stop-hook to surface accumulated async-block decisions at turn end

**Severity:** MEDIUM — silent-block bleed today; will partially persist post-async-semantics for plugins that legitimately stay async.
**Effort:** S–M (one new WASM hook plugin + one event-log reader; ~3-5 days).
**Target release:** v1.1 (post-async-semantics ship).
**Status:** identified.
**Adopted:** 2026-05-07

### Problem Statement

Today, every async hook plugin that emits a `hook.block` decision logs to `events-*.jsonl`
but never surfaces to the user. The prism audit (2026-05-07) revealed
`validate-template-compliance` blocking 55 times silently in a single day on real template
violations (S-3.03/04/05, S-MAINT-001 missing `tdd_mode`). The plugin-async-semantics cycle
(v1.0-feature-plugin-async-semantics-pass-1) addresses this for plugins that should never be
async (validators with `on_error = "block"`), but plugins that legitimately stay async —
telemetry, optional advisories — will continue to log block decisions invisibly.

A second-tier fix: a Stop hook (or SubagentStop hook) reads the events log accumulated during
the assistant turn and emits a summary message at turn end. The user sees: "FYI: 3 async-hook
block decisions during this turn — validate-X on file Y reason Z, …". This makes async-block
bleed observable without forcing every advisory plugin into the sync critical path.

### Proposed Solution

- New native WASM plugin: `surface-async-blocks` (or similar name).
- Event: Stop (PostUserMessage end) and/or SubagentStop.
- Behavior: read `events-*.jsonl` filtered by `dispatcher_trace_id` matching the current turn
  (or by timestamp window since turn start; needs design) — collect all `type: hook.block`
  records — emit a summary stderr message with file paths, hook names, reasons.
- Output: visible to the user via stop-hook stdout/stderr (Claude Code surfaces Stop hook output).
- Configuration: registry-driven; user can disable via `.claude/settings.local.json` if too noisy.

### Acceptance Criteria (sketch)

- AC-1: After any assistant turn that triggers ≥1 async-hook block, the Stop hook produces
  a structured summary visible in the next user-facing message.
- AC-2: The summary identifies plugin name, file_path, reason, and timestamp for each block.
- AC-3: Zero async-hook blocks → Stop hook is silent (no spam).
- AC-4: Stop hook latency ≤100ms p95 (does not noticeably delay turn end).
- AC-5: Filter scope is the current turn only (not historical days).

### Dependencies

- Soft-blocked-by: v1.0-feature-plugin-async-semantics-pass-1 (correct partition of
  "should-be-sync" vs "legitimately-async" plugins must exist first; otherwise this hook
  would surface noise from misclassified plugins).
- Related-to: ADR-015 single-stream OTel emission (`events-*.jsonl` is the read source).
- Not-blocked-by: any specific story.

### Risk if Deferred

LOW. Async-semantics work alone closes the bulk of today's silent-block bleed. The Stop hook
closes the residual ~5-10% (telemetry advisories that legitimately stay async). Deferral
acceptable until async-semantics ships and we measure residual bleed.

### References

- Prism audit, 2026-05-07: 1965 invocations of `validate-template-compliance`, 55 silent
  blocks, all `template_noncompliant`.
- F1 delta analysis: `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md`
  § Open Questions OQ-1 (proposes a similar idea inline).

---

## Resolution History

| ID | Resolved In | Story | Resolution |
|----|------------|-------|------------|
| TD-014 (epic spec phase) | E-8 epic spec converged 2026-04-30 (status: ready) | story-writer dispatch unblocked | CONVERGENCE_REACHED at v1.7 (11 passes, 41+11 findings closed); awaiting v1.0.0 GA close + S-8.00 pre-work for full TD-014 closure (per-story implementation in W-15/W-16/W-17) |
| TD-020 | sweep PR `fix/td-020-sweep-skipped-bats-suites` (2026-05-04) | — | All 4 suites resolved — 2 un-skipped clean (`generate-registry`, `state-health`); `codify-lessons` had 1 obsolete worktree-path test removed plus path-portability rewrite (15/15 pass); `novelty-assessment` had 3 tests removed for un-implemented paths (15/15 remaining pass). SKIP_SUITES list reduced from 4 → 0. CHANGELOG entry under `## Unreleased`. |

## Tech Debt as Feature Mode Cycles

When P0 items accumulate, they become a Feature Mode cycle (Path 3) with
cycle type "refactor":

```
orchestrator: "Tech debt P0 items need attention"
  -> Path 3 (Feature Mode) with cycle type "refactor"
  -> cycles/vX.Y.Z-refactor-[name]/
  -> Same VSDD rigor: specs updated, tests updated, adversarial review
  -> Release: PATCH (no new features) or MINOR (if public behavior changes)
```
