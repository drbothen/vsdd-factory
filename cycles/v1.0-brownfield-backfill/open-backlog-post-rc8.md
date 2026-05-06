# Open Backlog — post-rc.8 (HISTORICAL SNAPSHOT 2026-05-04)

> **Extracted from STATE.md during D-237 state-hygiene burst 2026-05-05.**
> This was the backlog state as of 2026-05-04 (rc.8 ship). Current active backlog is tracked in tech-debt-register.md and the active cycle's sprint-state.

## Carried over from pre-rc.8 (still open as of 2026-05-04)

- TD-019 ci.yml develop trigger (ci.yml does not run on push to develop; PR-time coverage diverges from release.yml tag-time)
- TD-020 broken bats suites in SKIP_SUITES: codify-lessons, generate-registry, novelty-assessment (2 fail at lines 171/186), state-health — fix-or-delete each; NO new SKIP_SUITES entries without a TD ticket
- TD-021 release.yml fail-fast + cache continue-on-error: set `fail-fast: false` on build-binaries matrix; add `continue-on-error: true` to Cache cargo step
- TD-014 Tier 2/3 retirement (folded into W-16/W-17)
- TD-015 per-invocation telemetry correlation (post-v1.0)
- TD: 1,137 pre-existing STALE input-hashes
- HIGH-W15-001 plugin version drift (1.0.0-rc.1 vs 0.0.1)
- SEC-002/004/005/006 deferred dispositions for v1.0 GA
- Scheduled remote agent 2026-05-22 — independently verifies sync-develop fired correctly on rc.3

## New from Phase B-bis (rc.5..rc.8 work)

- **TD-022 apply-platform.sh bash dependency on Windows.** apply-platform.sh + detect-platform.sh are bash scripts. Windows users need git-bash. Tracked enhancement: rewrite as a `factory-dispatcher activate` subcommand so the dispatcher binary owns activation logic — drops the bash dependency entirely + makes activation truly OS-portable.
- **TD-023 commands-reference.md staleness.** rc.5 deleted the entire `plugins/vsdd-factory/commands/` shim directory (111 files); `docs/guide/commands-reference.md` may still reference patterns from before that. Audit + refresh.
- **TD-024 lessons.md update for Phase B-bis.** Capture in `cycles/v1.0-brownfield-backfill/lessons.md`: (1) `claude plugin validate` doesn't validate marketplace.json source schema; (2) `secrets` context unavailable in job-level `if:`; (3) `[lib] crate-type cdylib` produces underscored wasm names — use `[[bin]] name = "hyphenated"`; (4) same-repo marketplace + git-subdir is empirically broken — cross-repo works.
- **TD-025 generate-marketplace-pr.sh** (optional). The bump-marketplace job duplicates jq + git logic from claude-mp's manual flow. Could extract a small bin/ helper script that both the workflow and an operator-run path can call. Low priority.
- **TD-026 dual maintenance burden.** vsdd-factory release workflow now needs to coordinate with claude-mp's marketplace.json updates. Currently automated via bump-marketplace + CLAUDE_MP_PAT secret. Document the secret rotation policy somewhere (maintainer ops doc).

## New from Phase D-4 (2026-05-05)

- **TD-VSDD-056 ADR template extension — Downstream Epics Requiring Amendment.** ADR-015 was authored after E-9 v1.6 reached convergence but did not enumerate E-9 in its `subsystems_affected` field, causing a post-convergence 4-file amendment burst (E-9 v1.7, d9f2c86). The ADR template should require a "Downstream epics requiring amendment" section listing every in-flight epic whose convergence-frozen body must be reopened. ADR review checklist should assert each downstream epic has an explicit cross-ref. Source: D-238 architect report. Codified in lessons.md (LESSON: ADR authors should enumerate in-flight epics requiring amendment).
- **TD-VSDD-057 Adversary angle-of-attack rotation rule for ADR-013 convergence.** Pass-3 forward-simulation angle found H-1 that passes 1+2 (verify-architect + reverse-trace angles) missed. ADR-013 skill and adversary dispatch prompt should require angle-of-attack diversity — at least 2 distinct angles exercised across 3-of-3 NITPICK_ONLY passes, with each fresh-context pass selecting an angle not yet used. Source: D-241 pass-3 finding H-1. Codified in lessons.md (LESSON: Multi-pass adversarial review with rotating angles of attack catches defects that single-angle passes miss).
- **TD-VSDD-058 Architect fix-burst rationale citation re-verification rule.** Pass-4 citation-grounding angle (D-243) found H-P4-001: M-2 leg (c) of v1.8 fix burst cited "Wave 3 acceptance criterion 3" which does not exist in ADR-015 (only AC-1 + AC-2 defined). Architect prompt for fix bursts must require re-reading any cited ADR section before commit, or including a copy-paste of the cited text in the commit message body. Adversary review skill should add a "citation-grounding" pass mode. Source: D-243 pass-4 finding H-P4-001. Codified in lessons.md (LESSON: Fix-burst rationale citations must be re-verified against authoritative source).
- **TD-VSDD-059 Frontmatter-version-vs-summary-table validator hook.** Third recurrence (F-P6-002 + F-P7-001 + H-P5-001) of frontmatter `version:` not matching latest non-reserved Changelog summary table row. Add a hook asserting `frontmatter.version == max(changelog_summary_table.version where date != '—')` for every story/epic file. Run at pre-commit. Source: D-245 pass-5 finding H-P5-001. Codified in lessons.md (LESSON: Frontmatter `version:` field must track latest non-reserved Changelog summary table row).
- **TD-VSDD-060 POLICY 1 amendment to forbid in-place edits to prior version blocks.** v1.9 burst rewrote v1.8 changelog prose in-place (M-2 closure entry). POLICY 1 (append_only_numbering) is silent on whether prose corrections within prior version blocks are allowed. Recommended: explicitly forbid in-place edits; require corrections-only-in-new-version-block. Source: D-245 pass-5 finding M-P5-001. Codified in lessons.md (LESSON: POLICY 1 (append_only_numbering) silent on prose corrections to prior version blocks).
- **TD-VSDD-061 Architect closure-claim enumeration rule.** D-246 M-P5-003 closure claimed "all 5 block-mode hooks now have explicit H-1 option (b) coverage" but only 3 of 5 (B-1, B-3, B-7) received explicit dispatcher-emits-automatically wording; B-2 and B-6 were lumped into a "Standard." row. Architect prompt for fix bursts must require enumeration of N/M items when writing "all N covered" closure claims. Adversary review skill should treat 'all N covered' closure claims as falsifiable. Source: D-247 pass-6 finding H-P6-001. Codified in lessons.md (LESSON: Closure-claim verification — "all N items covered" must be substantiated by enumerating the items).
- **TD-VSDD-062 Sibling-template-consistency check.** TD-VSDD-061 enumeration verified all 5 block-mode hooks were named but did not verify consistent wording template. D-249 pass-7 found M-P7-002 (H-1 option (b) nomenclature asymmetry) and M-P7-003 (PostToolUse parenthetical omission) on audit-w16.md line 38 vs sibling rows 36/37. Extend TD-VSDD-061 to require: when adding to an enumerated list of N items, diff new entry against existing siblings and verify same event-type parenthetical, same terminology, same citation prefix. Source: D-249 pass-7 findings M-P7-002 + M-P7-003. Codified in lessons.md (LESSON: Sibling-entry wording template consistency must be verified when adding to enumerated lists).
- **TD-VSDD-063 Fix-burst-internal nomenclature leakage check.** "H-1 option (b)" phrase originated in pass-3 adversarial review as an internal finding ID; v1.11 fix burst leaked it into audit-w16.md line 38 — a permanent L4 architecture artifact. Adversary prompts should include a "fix-burst-internal nomenclature scan" axis grepping permanent specs for `H-\d`, `M-P\d`, `F-P\d`, `L-P\d` patterns in non-changelog body sections. Architect prompts for fix bursts must require plain-language descriptions instead of internal IDs in permanent spec bodies. Source: D-249 pass-7 finding M-P7-002. Codified in lessons.md (LESSON: Fix-burst-internal nomenclature must not leak into permanent architecture documents).
- **TD-VSDD-064 Parallel-burst commit collision prevention rule.** D-249 seal + D-250 fix burst dispatched in parallel; state-manager's broad `git commit -a` swept architect's staged changes into a single merged commit (353c172), losing architect's distinct commit message and audit-trail boundary. Rule: orchestrator must either (a) run state-manager AFTER architect commits land (sequential dispatch), or (b) require both agents to use scoped `git add <specific-paths>` with no `-a` flag. State-manager and architect prompts must be updated with the scoped-add constraint. Source: D-250 retroactive seal. Codified in lessons.md (LESSON: Parallel agent dispatches must NOT both use `git commit -a` — burst commit collision risk).
- **TD-VSDD-066 TD-VSDD-063 scope extension to register-class permanent specs.** TD-VSDD-063 fix-burst-internal nomenclature leakage check was scoped to architect docs (audit-w16, gap-analysis, perf-baseline). Pass-9 L-P9-001 found the same leak class in open-questions.md `Source:` field (`M-P6-002` surviving from D-248 OQ-W16-001 authorship). Extend TD-VSDD-063 pre-commit scan scope to `.factory/specs/**` (open-questions register + any future register-class permanent-spec artifacts: TD-register, OQ-register, etc.). Source: D-252 pass-9 finding L-P9-001. Codified in lessons.md (LESSON: TD-VSDD-063 fix-burst nomenclature scan must include register-class permanent specs).
- **TD-VSDD-065 Decision-ID outbound semantic-anchor check.** perf-baseline-w16.md "W-16 Gate Model" line 156 cited `E-9 D-9.4 "Option C"` as gate-model authority; D-9.4 is "BC Anchor Strategy — reuse existing BC-7.xx family per hook", not a gate-model decision. No pre-commit hook or convergence check validates that decision IDs cited from arch docs to epic docs (D-9.x, AC-N) actually correspond to cited semantic content in target. Add outbound-decision-ID semantic-anchor check: light version verifies target ID exists; strong version (LLM-assisted at convergence time) asserts target heading is semantically compatible with reference context. Source: D-251 pass-8 finding M-P8-001. Codified in lessons.md (LESSON: Outbound cross-document decision-ID anchors must be semantically validated, not just syntactically valid).
- **TD-VSDD-067 Numeric-cross-anchor review axis for adversary.** E-9 AC-3 cited "(~14MB)" advisory soft cap — ADR-014 line 45 had explicitly retired this projection in favor of `643686 bytes` (rc.1 × 2 = ~644KB), three orders of magnitude smaller. 10 prior adversarial passes missed this because no pass enumerated every numeric claim against its underlying measurement source. Adversary skill must add a "numeric-cross-anchor" review axis: enumerate every numeric claim (bytes, ms, %, counts) in spec/AC text and cross-validate against the underlying measurement source (perf-baseline file:line, ADR amendment, S-N.NN baseline). Apply at convergence-time at minimum. Source: D-254 pass-11 finding H-P11-001. Codified in lessons.md (LESSON: AC-numbered values must cross-validate against underlying measurement source).
- **TD-VSDD-068 Recursive-scrub check for nomenclature replacement text.** v1.14 fix burst replaced fix-burst-internal tokens in open-questions.md line 20 with a different set of fix-burst-internal tokens (`M-1 closure forward-pointer`) — non-compliant with the same TD-VSDD-063 rule it cited. Scrub-rule enforcement must include a recursive check: after replacing forbidden-pattern text, re-grep the just-edited region using the same forbidden pattern set to confirm the replacement is also clean. Also: forward-pointers in scrubbed text must resolve in the named target file (grep to verify). Source: D-255 pass-12 finding H-P12-001. Codified in lessons.md (LESSON: Recursive-scrub — fix-burst replacement text must be re-verified against the same forbidden-pattern set the original violated).
- **TD-VSDD-069 Line-accuracy extension to recursive-scrub.** v1.15 H-P12-001 fix replaced unresolvable forward-pointer with a "bidirectional anchor" citing `gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**")`. The quoted substring was correct; the line-number annotation was off-by-one (actual line is 325). TD-VSDD-068 recursive-scrub checked forbidden tokens but did not validate line-number accuracy of newly added cross-document citations. Extension: when a citation of form `<filename> line N ("<quoted text>")` is added in a fix burst, the scrub MUST grep `"<quoted text>"` in `<filename>` and confirm exactly one match at line `N`. If line number doesn't match grep result, abort. Source: D-256 pass-13 finding M-P13-001. Codified in lessons.md (LESSON: TD-VSDD-068 recursive-scrub must extend to line-number accuracy for new cross-document citations).
- **TD-VSDD-070 TD-VSDD-065 scope extension to section/subsection headings.** perf-baseline-w16.md H2 heading at line 154 carried "Option C" non-resolving anchor that pass-8 missed when fixing the same-section inline citation (line 156). TD-VSDD-065 (decision-ID outbound semantic-anchor check) was scoped to in-text citations only. Extension: all section/subsection headings (`# H1`, `## H2`, `### H3`) that name an external authority's decision/option/choice/amendment are subject to the same semantic-anchor resolution check. Source: D-257 pass-14 finding M-P14-001. Codified in lessons.md (LESSON: TD-VSDD-065 outbound-decision-ID semantic-anchor check must extend to section/subsection headings).
- **TD-VSDD-071 OQ-table propagation hook.** OQ-W16-001 was filed in v1.11/D-248 to `.factory/specs/open-questions.md` citing SS-01 implementer or E-10 Wave 1 architect as owner. gap-analysis-w16-subprocess.md received the bidirectional forward-pointer. But E-9 epic's Open Questions table (the canonical discoverability hub) was NOT updated — story-writer authoring S-9.07 would miss the binary-choice gate. Process-step required: when an OQ is filed citing an E-N epic as scope-owner, same burst MUST verify (or append) a corresponding row in the epic's Open Questions table. Adversary discoverability-audit angle should enumerate every OQ in open-questions.md and verify each is listed in its scope-owner epic's Open Questions table. Source: D-258 pass-15 finding M-P15-001. Codified in lessons.md (LESSON: When an OQ is filed citing an epic as scope-owner, the epic's Open Questions table must contain a corresponding row).
- **TD-VSDD-072 Retired-figure body-grep extension to recursive-scrub.** v1.14 / D-254 fix-burst sealed H-P11-001 by scrubbing AC-3 of "(~14MB)" but did not body-grep the rest of E-9 epic for sibling residue. Pass-17 H-P17-001 caught R-W16-003 mitigation cell at line 353 still carrying the retired figure (16 passes later). Extend TD-VSDD-068 recursive-scrub: when a fix burst replaces a retired numeric or named value, the same burst MUST body-grep the entire file and all amendment-scope files for the retired value before commit. Any non-changelog occurrence is a sibling regression that must be fixed in same burst. Source: D-260 pass-17 finding H-P17-001. Codified in lessons.md (LESSON: Body-grep extension to recursive-scrub — fix-burst must body-grep for retired figures across whole file, not just at primary fix site).
- **TD-VSDD-073** (last_amended mandatory for amended arch-doc-class files). When a fix burst body-amends an arch-doc-class file (gap-analysis, audit, perf-baseline, open-questions, or any future L4 arch doc), the same burst MUST update `last_amended: <YYYY-MM-DD>` in frontmatter. D-239 annotate-in-place convention preserved; frontmatter gains parallel structured signal. Source: D-261 pass-18 finding M-P18-001 (5th re-flag; S-7.02 recurrence threshold met). Codified in lessons.md (LESSON: Arch-doc-class files MUST carry last_amended: field when body amendments occur in fix bursts).
- **TD-VSDD-074 TD-VSDD-073 scope extension to BCs cited in amendment landings.** TD-VSDD-073 (last_amended convention) covers arch-doc-class files only. Pass-20 M-P20-002 found that BC-1.05.036 — created at D-224 within the amendment cycle and cited from gap-analysis + audit-w16 amendment blocks — was never updated for ADR-015 awareness. Symptom: BC-1.05.036 §Description names `host.exec_subprocess.completed` (no `vsdd.` prefix, no `.v1` suffix), violating ADR-015 D-15.2 reverse-DNS and contradicting OQ-W16-001. Extension: when an amendment burst changes a contract that a BC implements, the same burst MUST update the BC's frontmatter `last_amended:` AND add an awareness clause to the BC body. Adversary "downstream-implementer-simulation" angle should be a callable TD-VSDD-057 menu entry. Source: D-263 pass-20 finding M-P20-002 + PG-P20-001. Codified in lessons.md (LESSON: TD-VSDD-073 last_amended convention must extend to BCs cited in amendment landings).
- **TD-VSDD-075 Last_amended dependent-citation propagation requirement + source-code-verification discipline.** Two sub-rules codified by D-264 pass-21 findings (both HIGH): (1) **Source-code-verification:** Fix bursts citing source-code constants MUST read the actual source file before commit and quote the exact line in the commit message body. H-P21-001 found D-263 invented TIMEOUT (-7) / OUTPUT_TOO_LARGE (-8) — actual values are -2/-3 per `crates/factory-dispatcher/src/host/mod.rs:181-182`. (2) **Dependent-citation-propagation:** When a fix burst adds `last_amended:` (or any frontmatter field that shifts line numbers), same burst MUST grep all in-scope files for inbound `<filename> line N` citations and verify each still resolves. H-P21-002 was the THIRD recurrence of line-citation off-by-one class (after L-P9-001 and M-P13-001); S-7.02 threshold met. Source: D-264 pass-21 findings H-P21-001 + H-P21-002. Codified in lessons.md (LESSON: Frontmatter last_amended adds MUST trigger dependent-citation propagation refresh; fix bursts citing source-code constants MUST verify against source).
- **TD-VSDD-076 Intra-document semantic-sibling sweep extension to TD-VSDD-075.** When a fix burst corrects a Postcondition or any normative claim within a BC, the same burst MUST grep the SAME BC's sibling sections (§Related BCs, §Edge Cases, §Canonical Test Vectors, §Postconditions, §Description) for prior wording contradicting the correction, and update all contradicting siblings in the same burst. TD-VSDD-075 covered inter-document citation refresh and source-code-verification; TD-VSDD-076 extends it to intra-document semantic siblings. Source: D-265 pass-22 finding H-P22-001. Codified in lessons.md (LESSON: Intra-document semantic-sibling sweep — fix-burst correcting a Postcondition MUST sweep the same BC's sibling sections for contradicting prior wording).
- **TD-VSDD-077 Lessons-corpus bidirectional coherence validation hook.**
- **TD-VSDD-078 BC postcondition source-of-truth enumeration verification — extends TD-VSDD-075.**
- **TD-VSDD-080 Mechanize TD-VSDD-079 family-grep as pre-commit hook.** Five consecutive narrative-discipline failures (passes 24/25/28/29 + at-least-one pre-pass-24) demonstrate that narrative-discipline alone cannot enforce the TD-VSDD-079 8-term terminology-family grep. The burst that codified TD-VSDD-079 (D-271) ran only a 2-term grep (`sink chain\|try_send`), leaving `fan-out`, `Datadog`, `Honeycomb` at BC-1.05.036:51. Required: implement `validate-bc-terminology-family.sh` pre-commit hook that runs the TD-VSDD-079 8-term grep against any modified BC or arch-doc file and FAILS the commit if any term matches outside `### Changelog` or `### v1.X` H3 sections. Source: D-272 pass-29 finding H-P29-001. Codified in lessons.md (LESSON: TD-VSDD-079 narrative-discipline must mechanize as pre-commit hook to prevent self-violations).
- **TD-VSDD-079 TD-VSDD-076 extension: terminology-family grep checklist for sibling-sweep fixes.** Three TD-VSDD-076 self-violations recorded across passes 24/25/28 (S-7.02 recurrence threshold met). Each fix burst grepped for the EXACT phrase the prior adversary cited rather than the BROADER terminology family — leaving parallel stale wording in §Description, §Purity Classification, §Edge Cases, and §Canonical Test Vectors. Extension: before committing any amendment-class sibling-sweep fix burst, grep ALL retired-terminology variants across the full BC ("sink chain", "Router", "SinkRegistry", "multi-sink", "fan-out", "datadog", "honeycomb", "try_send", etc.) across ALL normative sections (§Description, §Postconditions, §Invariants, §Edge Cases, §Canonical Test Vectors, §Purity Classification, §Refactoring Notes). The fix-burst MUST achieve zero matches in non-changelog body for ALL retired-terminology variants. Source: D-271 pass-28 findings H-P28-001/002 + M-P28-001/002/003. Codified in lessons.md (LESSON: TD-VSDD-076 sibling-sweep needs explicit terminology-grep checklist for amendment-class fixes). When a BC postcondition cites a CONCRETE ENUMERATION (list of error codes, list of denial reasons, list of fields, list of paths) derived from source code, the fix-burst MUST grep the cited source file for each enumeration item and verify presence/absence. Pass-25 H-P25-001 found BC-1.05.036:52 listed "env not allowed, cwd not allowed" denial paths which have no `emit_denial` callsite in source, while `no_exec_subprocess_capability` (exec_subprocess.rs:148) and `setuid_or_setgid_binary` (exec_subprocess.rs:169) were entirely missing. The fabrication survived 4 prior passes because none specifically targeted enumeration correctness. Extension of TD-VSDD-075 sub-rule (source-code-verification) to cover enumerations. Adversary source-code-traceability angle (TD-VSDD-057 menu) should be a regular axis. Source: D-268 pass-25 finding H-P25-001. Codified in lessons.md (LESSON: BC postconditions citing concrete enumerations from source code MUST be source-of-truth-verified). The 20 codified TD-VSDD lessons (057-076) live in two canonical artifacts: `cycles/<cycle>/lessons.md` (full lesson body + Source citation + [codified] marker) and `cycles/<cycle>/open-backlog-post-rc8.md` (TD-VSDD-NNN bullet entry under appropriate H2 section). Pass-24 found 6 distinct coherence defects between these two files: (1) 3 stub entries in open-backlog where body content was merged into the next numbered entry; (2) 3 entries placed after the wrong H2 boundary; (3) 5+ orphaned `[codified] by D-NNN` markers in lessons.md displaced from their associated lesson; (4) bidirectional drift on TD-VSDD-074 Source field; (5) non-monotonic TD-VSDD-NNN ordering; (6) duplicated content concatenation across adjacent entries. Source: D-267 pass-24 findings M-P24-001..006. Codified in lessons.md (LESSON: Lessons-corpus artifacts MUST maintain bidirectional consistency).

## Lessons codified during the cycle (needing follow-up in lessons.md)

| Lesson | Where it bit us | Codification |
|---|---|---|
| `claude plugin validate` is plugin-only — it does NOT validate marketplace.json source schema | rc.2..rc.5 shipped a schema-violating `github + path` source that passed validate | Add to release-checklist: "validate" passing is necessary but NOT sufficient; require clean-room install verification |
| `secrets` not available in job-level `if:` | rc.7 hotfix #1 (PR #74) — every release.yml run failed silently with "workflow file issue" | Pattern: preflight step sets output, subsequent steps gate on output |
| Cargo defaults `[lib] name` to underscore-converted package name | block-ai-attribution + capture-pr-activity crashed silently for weeks because registry expected hyphens but cdylib produced underscores | `[[bin]] name = "hyphenated"` is the only path; document in plugin-marketplace-architecture.md (already done) |
| Self-referential same-repo `git-subdir` is broken empirically (works for cross-repo) | rc.6 attempted git-subdir within same repo as marketplace; cache stayed empty | Always split marketplace into a separate repo when nested layout is involved |
| `hooks.json` is per-machine output of activate; tests should assert against `hooks-registry.toml` | rc.7 untracked hooks.json correctly per S-0.4, broke 11 bats suites that asserted against it | Done in rc.8 — bats migrated to registry assertions via `tests/helpers/registry.bash` |

## New from D-283 (2026-05-05)

## TD-VSDD-088-HOOK — Pre-dispatch hook for orchestrator-routing rule mechanical enforcement

**Source:** TD-VSDD-088 (codified D-283 / 2026-05-05)

**Class:** Mechanical enforcement of orchestrator-routing rule (TD-VSDD-088). Without a hook, the rule depends on orchestrator narrative discipline — which has demonstrably failed across 22 prior fix bursts.

**Hook design:** Pre-dispatch validator inspects the Agent tool's `subagent_type` + `prompt` parameters. If `subagent_type == "vsdd-factory:state-manager"` AND the prompt contains keywords matching substantive BC content edit patterns (e.g., "Postcondition", "Edge Case row", "Test Vector row", "rewrite Postcondition", "add EC", "rewrite EC", "Add new EC", "amend Postcondition", "mechanism description"), reject the dispatch with error message: "Routing violation per TD-VSDD-088 NORMATIVE: BC content authorship must dispatch to product-owner or architect first; state-manager runs LAST per POLICY 3."

**Implementation surface:** Either (a) Claude Code hook (PreToolUse on Agent tool); (b) orchestrator self-check at dispatch-time using a pattern-match function; (c) plugin-level hook in the orchestrator agent prompt enforcing the rule.

**Acceptance criteria:**
- Hook detects 100% of state-manager dispatches that contain BC content authorship instructions
- Hook does NOT false-positive on legitimate state-manager dispatches (STATE.md updates, lessons.md codifications, STORY-INDEX bumps, seal commits)
- Hook produces a clear error message citing TD-VSDD-088 and listing the offending keywords

**Priority:** HIGH (the user explicitly requested stronger routing enforcement; TD-VSDD-088 narrative discipline alone is insufficient given the 22-burst drift history)

**Status:** OPEN — to be implemented in a future maintenance burst or as part of factory-engine work

**Date:** 2026-05-05
**Burst:** D-283

## New from D-285 (2026-05-05)

## TD-VSDD-089-HOOK — Pre-commit sibling-sweep validator for BC authoring bursts

**Source:** TD-VSDD-089 (codified D-285 / 2026-05-05)

**Class:** Mechanical enforcement of PO authoring sibling-sweep (TD-VSDD-089). Approximate but high-value automation.

**Hook design:** Pre-commit hook script `validate-bc-sibling-sweep.sh` runs:
- Numeric enumeration grep: find any narrative count ("\d+ denial paths", "\d+ fields", "\d+ categories") in modified BC files; for each, grep adjacent sections for matching counts; flag mismatches.
- Cross-BC section pointer grep: for any line matching "BC-N.NN.NNN (Postcondition|EC-\d+|P\d+)" in modified BC files, run a sanity check on the cited target.
- New mechanism string detection: extract recently-added emit_denial reason strings; verify each appears in at least one TV row.

**Implementation surface:** Bash script in `dark-factory-engine/hooks/`; invoked as part of TD-VSDD-080-style pre-commit chain.

**Acceptance criteria:**
- Hook detects 100% of numeric enumeration drift in modified BC files
- Hook detects 100% of cross-BC section pointer drift
- Hook does NOT false-positive on legitimate quotation marks or unrelated digit sequences
- **5th axis (D-286 extension):** when lessons.md is modified, hook verifies that no `**Burst:**` trailer from an adjacent TD entry has bled into a neighboring TD entry's body (codification artifact sibling integrity per TD-VSDD-089 axis 5)

**Priority:** MEDIUM (PO discipline narrative will catch most cases; hook is belt-and-suspenders)

**Status:** OPEN — to be implemented in a future maintenance burst

**Date:** 2026-05-05
**Burst:** D-285

---

## TD-VSDD-090-HOOK — Pre-commit self-application audit hook for normative-rule codification bursts

**Source:** TD-VSDD-090 (codified D-287 / 2026-05-05)

**Class:** Mechanical enforcement of TD-VSDD-090 self-application audit (orthogonal to TD-VSDD-088-HOOK and TD-VSDD-089-HOOK).

**Hook design:** Pre-commit script `validate-td-vsdd-self-application.sh`:
- Detects when commit adds a new `## TD-VSDD-NNN` entry to lessons.md
- For each new entry, parses the rule's stated scope (axes / artifact types)
- Runs lightweight automated self-application checks where machine-verifiable (e.g., trailer format consistency for TD-089; new mechanism string TV-witness presence for TD-085; numeric enumeration consistency for TD-059)
- Flags un-checkable rules for manual review checklist (returned in commit message preface)

**Implementation surface:** Bash script at `dark-factory-engine/hooks/validate-td-vsdd-self-application.sh`; invoked as part of the TD-VSDD-080-style pre-commit chain alongside `validate-bc-table-arity.sh` and `validate-bc-terminology-family.sh`. Triggered when a commit modifies `cycles/*/lessons.md` and adds a line matching `^## TD-VSDD-\d+`. Outputs a checklist of machine-verifiable sub-checks (e.g., grep for H3 detail block in modified epics, trailer format check for sibling TD entries) and a manual-review prompt for non-automatable axes.

**Acceptance criteria:**
- Hook detects 100% of new TD-VSDD-NNN entries
- Hook performs automated self-application for at least TD-085, TD-087, TD-089 axes 1-5
- Hook produces clear manual review checklist for non-automatable axes

**Priority:** HIGH (this is the third meta-rule the user has explicitly tracked under "make routing requirement stronger" thread; mechanical enforcement of TD-VSDD-088 + 089 + 090 is the structural payoff)

**Status:** OPEN — to be implemented as part of the validate-bc-* hook chain

**Date:** 2026-05-05
**Burst:** D-287

---

## TD-VSDD-091-HOOK — Pre-commit hook detecting self-referential line-number citations

**Source:** TD-VSDD-091 (codified D-290 / 2026-05-05)

**Class:** Mechanical enforcement of stable-anchor citation discipline. Detects when an H3 block (or any newly-added content) contains line-number citations to a file the same commit modifies.

**Hook design:** Pre-commit script `validate-self-referential-citations.sh`:
- For each modified .md file in commit, scan added lines for patterns matching `line \d+` or `:\d+` or similar line-number references.
- For each match, check whether the cited file is also modified in the same commit.
- If self-referential (citation to a file the commit modifies), flag for stable-anchor replacement OR require post-stage grep verification.

**Implementation surface:** Bash script in `dark-factory-engine/hooks/validate-self-referential-citations.sh`. Invoked alongside validate-bc-table-arity.sh, validate-bc-terminology-family.sh, validate-td-vsdd-self-application.sh as part of the codification pre-commit chain.

**Acceptance criteria:**
- Hook detects 100% of self-referential line-number citations in H3 blocks
- Hook does NOT false-positive on cross-file line citations (e.g., BC files citing source-code lines)
- Hook produces clear error message identifying the offending citation and suggesting stable-anchor alternative

**Priority:** HIGH (this is the empirically validated root cause of the 6/6 codification-burst-self-violation pattern; mechanization is the only proven path)

**Status:** OPEN — to be implemented in dark-factory-engine alongside TD-088-HOOK / TD-089-HOOK / TD-090-HOOK

**Date:** 2026-05-05
**Burst:** D-290

---

## TD-VSDD-091-ENGINE — Engine-level ban on line-number citations across VSDD artifacts

**Source:** User directive (2026-05-05 mid-pass-48): "we need to not use line number citations, lets add that to the technical debt for us to implement in vsdd"

**Class:** Promote TD-VSDD-091 from project-level codification (lessons.md) to VSDD-engine-level enforcement. The structural insight from pass-47/48 (line numbers shift on insertion → manual narrative discipline cannot fix this) generalizes beyond self-referential intra-file: line-number citations across ANY artifact pair where the citing burst modifies the cited file are equally susceptible. The empirically validated fix (stable-anchor citations) should be the VSDD-wide default.

**Scope extension vs TD-VSDD-091:**

- **TD-VSDD-091** (project-level, NORMATIVE in lessons.md): Bans self-referential intra-file line citations; mandates stable anchors (section headings, ticket section names, postcondition numbers, frontmatter field names).
- **TD-VSDD-091-ENGINE** (engine-level, NEW): Extends ban to ALL line-number citations across VSDD artifacts (BC bodies, epic changelogs, lessons.md, open-backlog tickets, pass review files, STATE.md). Carve-outs: read-only source-code references (e.g., `host/mod.rs:152` where citing burst doesn't modify the cited file) and external standards documents (e.g., RFC 9999 Section 4 line 12). Default: prefer anchors.

**Implementation surfaces (in dark-factory engine repo):**

1. **Agent prompts** — state-manager, product-owner, architect, adversary, spec-steward agent prompts updated to: (a) prefer anchor-based citations; (b) when line numbers must be cited, require post-stage grep verification + stable-anchor comment annotation explaining why line numbers were chosen.

2. **Pre-commit hook** (extends TD-VSDD-091-HOOK proposal): `validate-stable-anchor-citations.sh` script in `dark-factory-engine/hooks/`. Detects line-number citations in modified .md files within commits; for each detected citation, classifies as (a) self-referential within same commit's modified files (REJECT — must use anchor); (b) cross-file to commit-modified file (REJECT — anchor preferred); (c) read-only source-code or external reference (ALLOW). Hook produces clear error message identifying the offending citation and suggesting the anchor-based alternative.

3. **Citation linter** (eventual): integrated into validate-consistency / validate-bc-* skill chain. Surfaces stale line citations on every PR.

**Acceptance criteria:**
- All 5 dark-factory engine agent prompts (state-manager, PO, architect, adversary, spec-steward) reference TD-VSDD-091-ENGINE in their citation discipline guidance.
- `validate-stable-anchor-citations.sh` hook implemented and added to the pre-commit chain.
- Hook detects 100% of self-referential and within-commit cross-file line citations; zero false-positives on read-only source-code citations.
- Existing project-level codifications (lessons.md TD-VSDD-091) reference TD-VSDD-091-ENGINE as the engine-level enforcement counterpart.

**Priority:** HIGH (user explicitly requested "stronger routing" / mechanical enforcement; this is one of three concurrent engine-level hooks needed: TD-088-HOOK, TD-089-HOOK, TD-090-HOOK, TD-091-HOOK, plus this engine-level extension).

**Status:** OPEN — to be implemented in dark-factory engine maintenance work alongside TD-088/089/090/091-HOOK chain.

**Date:** 2026-05-06
**Burst:** D-291 (filed during pass-48 NITPICK_ONLY seal)

---

## TD-VSDD-092-HOOK — Pre-commit hook scanning source-of-truth for silent-discard pattern coverage

**Source:** TD-VSDD-092 (codified D-293 / 2026-05-06)

**Class:** Mechanical enforcement of BC-SOUL4-coverage. For each BC, scan the cited source-of-truth function for silent-discard patterns; verify each has corresponding EC coverage.

**Hook design:** Pre-commit script `validate-bc-soul4-coverage.sh`:
- For each modified BC file, parse §Architecture Anchors / §Postconditions for cited source files
- For each cited source file, grep for silent-discard patterns: `let _ =`, `map_err(|_|`, `unwrap_or(`, `unwrap_or_else(|_|`, `\.ok();`, `if let Err(_) =`
- For each match, search the BC body for an EC row that anchor-references the source line OR contains an out-of-scope declaration
- Report mismatches; reject commit if any uncovered silent-discard

**Implementation surface:** Bash script in `dark-factory-engine/hooks/validate-bc-soul4-coverage.sh`. Invoked alongside validate-bc-table-arity.sh, validate-bc-terminology-family.sh, validate-td-vsdd-self-application.sh, validate-self-referential-citations.sh as part of the codification pre-commit chain.

**Acceptance criteria:**
- Hook detects 100% of silent-discard patterns in source files cited by modified BCs
- Hook does NOT false-positive on legitimate non-silent-discard patterns (e.g., `let _ = serde_json::to_string()` where the discard IS the intent and is documented)
- Hook produces clear error message identifying uncovered silent-discards and suggesting EC row authoring

**Priority:** HIGH (this is the 5th engine-level hook needed; combined with TD-088/089/090/091-HOOK chain, this completes the BC-coverage pre-commit suite)

**Status:** OPEN — to be implemented in dark-factory engine maintenance work alongside TD-088/089/090/091-HOOK chain

**Date:** 2026-05-06
**Burst:** D-293
