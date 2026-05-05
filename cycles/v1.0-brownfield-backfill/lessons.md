---
document_type: lessons-codification
level: ops
cycle: v1.0-brownfield-backfill
producer: session-reviewer
timestamp: 2026-05-04T00:00:00Z
---

# Lessons Codified — v1.0-brownfield-backfill

> Lessons from this cycle that have been promoted to follow-up artifacts (stories or
> STATE.md Drift Items). Each entry records the gap, the evidence, and the disposition.

---

## LESSON-2026-05-04-001 [process-gap] Story marked "shipped" without AC verification

**Discovered:** 2026-05-04 (research investigation during PR #78 work)
**Category:** delivery-discipline
**Severity:** HIGH — silent data loss for downstream consumers

### Gap Description

S-3.04 (emit_event as host function refactor) was marked `status: merged` and referenced
as "shipped" in v1.0.0-beta.4. AC-001 explicitly required `emit_event()` to "route events
to the configured sinks (not just internal log)." Investigation on 2026-05-04 revealed that
the integration step at `crates/factory-dispatcher/src/sinks/mod.rs:11-15` is an
unimplemented TODO — `Router::submit` is never called from `main.rs`. Plugin events have
been silently routing to `dispatcher-internal-*.jsonl` instead of `events-*.jsonl` since
S-1.4 (April 24, 2026), invisible to every downstream consumer.

The story spec, the implementation crates (sink-core, sink-file, sink-otel-grpc, Router,
RoutingFilter) all shipped — but the integration "wire it up in main.rs" step had no AC
test enforcing it, so the gap was never caught.

### Root Cause

Story acceptance was signed off without an end-to-end verification test confirming that
AC-001 actually held. Unit tests on the individual components (sink, router) passed. There
was no bats or cargo integration test exercising the full path: dispatcher receives event
→ Router::submit called → event reaches configured sink file.

### Systemic Pattern

For "integration" ACs — those of the form "X is wired into Y" — unit testing the
components in isolation gives false confidence. Only a test that sends an event through
the running dispatcher and asserts it appears in the correct sink file would catch this
class of gap.

### Lesson

Story ship status MUST be gated on a passing AC-verification test (either bats, cargo
integration test, or equivalent), not just a story-writer signoff. For integration ACs,
the test must exercise the end-to-end path. Before an implementer marks a story complete,
every AC must be linked to a specific test name + test file path + test result. An agent
MUST refuse to mark a story complete without this artifact.

### Disposition

- **Bug fix:** In progress via E-TELEMETRY epic (wiring Router::submit in main.rs).
- **Process fix:** S-7.04 opened — "Add AC-test-link discipline to per-story-delivery flow" (see STORY-INDEX.md E-7).
- **TD-007 amendment:** Added 2026-05-04 note; original "shipped" claim corrected.
- **Reference:** `.factory/stories/S-3.04-emit-event-host-function.md`; `.factory/tech-debt-register.md` TD-007

---

## LESSON-2026-05-04-002 [process-gap] Dashboards query fields no plugin emits

**Discovered:** 2026-05-04 (forensic field inventory during PR #78 research)
**Category:** observability-contract
**Severity:** MEDIUM — panels permanently zero; users misled about pipeline health

### Gap Description

Multiple Grafana dashboard panels query event_type values or field names that no plugin
actually emits:

- `plugins/vsdd-factory/tools/observability/grafana-dashboards/factory-prs.json:96`
  queries `event_type="pr.opened"` — the WASM plugin (capture-pr-activity) emits
  `pr.created`. Panel shows zero forever.
- `plugins/vsdd-factory/tools/observability/grafana-dashboards/factory-prs.json:335,365`
  queries `open_to_merge_seconds` on `pr.merged` events — no plugin emits this field.
  Panel is permanently unwired.
- `plugins/vsdd-factory/tools/observability/grafana-dashboards/factory-roi.json:616`
  also queries `pr.opened` (same mismatch as above).

The mismatch was invisible during authoring because dashboards were written against an
assumed emitter contract, with no validation gate checking whether that contract was
actually implemented.

### Root Cause

Dashboard queries form an implicit contract with emitter plugins. There is no CI lint
hook that validates: "every event_type and field referenced in a Grafana dashboard query
has a corresponding emit call in the actual plugin sources." The contract diverged and
remained undetected indefinitely.

### Emitter Sources (ground truth)

The authoritative union of emitted fields/types is derivable from:
1. WASM plugin `host::emit_event` call sites in `crates/hook-plugins/*/src/`
2. Bash hooks invoking `bin/emit-event` in `plugins/vsdd-factory/hooks/*.sh`
3. Dispatcher lifecycle event constants in `crates/factory-dispatcher/src/internal_log.rs`

### Lesson

Dashboard queries and emitter plugins share an implicit schema contract. That contract
needs explicit CI validation. A lint hook should (a) grep all Grafana JSON files for
`event_type=` and field references, (b) grep all plugin sources for corresponding emit
definitions, and (c) fail CI if any dashboard field has no emitter. This check should
ride TD-014 (bash-hook native WASM migration) since both workstreams require emitter
contract awareness.

### Disposition

- **S-7.05 opened** — "Add dashboard-emitter-contract lint hook" (see STORY-INDEX.md E-7).
  Target: v1.0.1. Can be delivered independently of E-TELEMETRY.
- **Reference:** `plugins/vsdd-factory/tools/observability/grafana-dashboards/factory-prs.json`;
  `plugins/vsdd-factory/tools/observability/grafana-dashboards/factory-roi.json`;
  TD-014 (bash-hook retirement workstream)

---

## LESSON-2026-05-05-001 [process-gap] TD-020 sweep: un-skipping bats suites without CI-equivalent validation shipped CI regressions

**Discovered:** 2026-05-05 (rc.11 release failure post-mortem)
**Category:** delivery-discipline
**Severity:** MEDIUM — caused 2-round release retag and 2 hotfix PRs

### Gap Description

TD-020's 2026-05-04 sweep resolved all four `SKIP_SUITES` entries in `run-all.sh`. Two entries — `generate-registry` and `state-health` — were closed as "UN-SKIPPED with no test changes" because they passed locally. Neither passed in CI during the rc.11 release workflow.

**generate-registry (external TD-VSDD-054, PRs #85 + #86):** `scripts/generate-registry-from-hooks-json.sh` used `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json` to recover a historical version of `hooks.json`. GitHub Actions uses a shallow clone (`--depth=1` by default); the parent commit `7b4b774^` was not present in the object store, and `git show` exited non-zero. Fixed by vendoring the historical file as `scripts/legacy/hooks-json-pre-templating.json` and rewriting the script to use `cat`. PR #85 was rebased and re-merged as PR #86 after a branch conflict.

**state-health (external TD-VSDD-055, PR #87):** `state-health.bats` `setup()` calls `git commit` to build a baseline repo fixture. CI runners start with empty global git config (no `user.email` / `user.name`), so `git commit` exited 128 with "Author identity unknown." Fixed by adding `git config user.email`, `user.name`, and `commit.gpgsign false` after `git init` in the bats `setup()` function.

Result: rc.11 required two retag rounds (force-delete + re-push) before the release workflow went green. Tag settled at fb3e297. Marketplace PR #5 was delayed until 2026-05-05T03:33:21Z.

### Root Cause

Local test environments carry side-channels that CI runners do not:

- **Global git config** — developer machines always have `user.email`/`user.name`; CI runners start with empty config
- **Full git history** — developer machines carry full `git log`; GitHub Actions `checkout` defaults to `--depth=1` (shallow clone)
- **Operator-installed CLI tools** — local shells may have tools beyond what the workflow's setup steps declare
- **Mature shell environment** — local shells inherit years of env var configuration; CI runners start minimal

The TD-020 sweep workflow had no checklist item or validation gate requiring CI-equivalence before declaring an un-skipped suite as passing. "Passes locally" was treated as sufficient, conflating two different validation regimes.

### Lesson

Un-skipping a previously-skipped bats suite must be validated in a CI-equivalent environment before claiming pass. Concretely: empty global git config; shallow clone with `--depth=1`; no operator-installed CLI tools beyond what the workflow declares; clean shell env. Either run the suite in a CI-shaped sandbox locally (e.g., `docker run` with a clean image mirroring the runner), or land the un-skip behind a small CI smoke job that exercises the suite under that matrix. Local-pass alone is necessary but not sufficient evidence.

### Disposition

- **TD-024 opened** (P2, target v1.0.1) — codifies the process gap and sketches the checklist fix + optional CI smoke job. TD-024 is the canonical tracker.
- May fold into a future S-7.04-style discipline story if one is opened; TD-024 is sufficient in the interim.
- **References:** PRs #85, #86 (TD-VSDD-054 / generate-registry fix); PR #87 (TD-VSDD-055 / state-health fix); TD-020 (the sweep that introduced the gap)

---

## Cross-reference 2026-05-05: D-236 elevation decision

**Decision:** D-236 (STATE.md decision-log) records the 2026-05-05 elevation of E-10 (ADR-015 single-stream OTel emission) ahead of Phase D-4 Burst 2 (E-9 native-WASM port stories). The elevation is motivated by LESSON-2026-05-04-001 + LESSON-2026-05-04-002 above:

- **LESSON-2026-05-04-001 (S-3.04 marked shipped without AC verification — Router::submit never wired):** ADR-015 + E-10 Wave 1 (S-10.02 FileSink single-stream wiring) closes this gap by replacing the unwired Router/SinkRegistry/DlqWriter architecture with a single-stream FileSink writer. E-10 Wave 5 (S-10.09 crate retirement) physically removes the unwired components.
- **LESSON-2026-05-04-002 (Grafana dashboards query event_types / fields no plugin emits — pr.opened vs pr.created + open_to_merge_seconds):** E-10 Wave 2 (S-10.05 plugin schema migration + bug-fix bundle) directly addresses the contract divergence: dual-emit shims allow consumer migration without flag-day; OTel-aligned event names are stamped uniformly; the missing field `open_to_merge_seconds` is added at the emitter; the pr.opened vs pr.created mismatch is reconciled.

The elevation decision treats the lessons as fix-ahead-of-feature signals: closing the integration gap before authoring 23 new validate-*.wasm plugins (E-9) avoids a downstream rework cycle and lets the new plugins emit through the corrected contract from day one.

**Disposition:** E-10 enters spec convergence (PO BC authorship + adversarial review per ADR-013) before E-9 Burst 2 resumes. The pre-Burst-2 architect dispatch (E-9 v1.7 amendment + W-16 forward-pointers, fully specified in D-236) absorbs ADR-015 contract awareness into the E-9 epic so eventual story-writer Burst 2/3 anchors S-9.01..S-9.07 to the correct emit contract. No new lesson is codified beyond the cross-reference; this is operational sequencing, not a new process gap.

---

### LESSON: ADR authors should enumerate in-flight epics requiring amendment

**Source:** D-238 architect report (d9f2c86)
**Date:** 2026-05-05

**Pattern:** ADR-015 was authored after E-9 v1.6 reached convergence. ADR-015 imposed the single-stream OTel emit contract on all hooks, but its `subsystems_affected` field did not enumerate E-9 (or any other in-flight epic that consumes the contract). Result: E-9 had to be re-opened post-convergence and amended to v1.7 in a 4-file burst.

**Codification:**
- ADR template should require a "Downstream epics requiring amendment" section listing every in-flight epic whose convergence-frozen body must be reopened to absorb the new contract.
- ADR review checklist should enumerate epic-version cross-refs and assert each downstream epic has an explicit cross-ref, not just an implicit "any future hook will follow this".
- File this as a TD ticket if the architect agent does not already track ADR-template additions: TD-VSDD-056 (ADR template extension — Downstream Epics Requiring Amendment).

**[codified]** by D-238 lessons.md append.

**References:** D-236 (STATE.md decision-log); ADR-015 (`.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md`); SS-03-event-emission.md; BC-1.11.001 / BC-1.11.002 / BC-1.11.003 (`.factory/specs/behavioral-contracts/ss-01/`); E-10 epic (`.factory/stories/epics/E-10-single-stream-otel-event-emission.md`); E-9 epic v1.6 (target v1.7) (`.factory/stories/epics/E-9-tier-2-native-wasm-migration.md`).

---

## LESSON-2026-05-05-001 [codified] Annotate-in-place vs version-bump convention for amendment bursts

**Discovered:** 2026-05-05 (D-239 adversary pass-1 LOW-1 resolution)
**Category:** spec-versioning-convention
**Severity:** LOW — convention gap, not a correctness gap

### Gap Description

During E-9 v1.7 amendment, the adversary flagged (as LOW-1, pending intent verification) that three architecture files (`gap-analysis-w16-subprocess.md`, `perf-baseline-w16.md`, `audit-w16.md`) retained `version: "1.0"` despite receiving new amendment sections. The E-9 epic was correctly bumped v1.6 → v1.7. The adversary could not adjudicate intent.

### Resolution

This is INTENTIONAL convention. The pattern is: **epic files version-bump; architecture docs annotate-in-place with dated section headers when the underlying decisions are unchanged.**

Rationale: (i) the architect explicitly chose "Post-Audit Amendment: ADR-015 Awareness (2026-05-05)" date-stamped section headers rather than version bumps; (ii) D-236 framed this as "metadata-only amendment"; (iii) versioning architecture documents creates churn that does not aid traceability when the underlying technical decisions are unchanged. The ADR-015 cross-reference in the amendment section provides traceability without requiring a version field bump on the arch doc.

### Codification

- **Epic files:** version-bump on every substantive change (v1.N → v1.N+1). This is the existing convention and should continue.
- **Architecture docs (gap-analysis, perf-baseline, audit, SS-*, ADR-*, etc.):** use append-only dated section headers (e.g., `### Post-Audit Amendment: <topic> (<date>)`) for metadata-only amendments that add cross-refs or awareness notes without changing the underlying technical decisions. Do NOT bump the `version` field for these annotation-only sections.
- **When to break this rule:** if an architecture doc's core content (not just annotations) changes — e.g., a new Gap row, a new perf measurement, a new audit finding — version-bump is appropriate. Annotation-only sections never warrant a version bump.
- Adversary reviewers should classify retained `version: "1.0"` on arch docs as LOW (pending intent verification), not MED/HIGH, because this convention is now codified.

**[codified]** by D-239 LOW-1 resolution via orchestrator decision, recorded in lessons.md append.

**References:** D-239 (STATE.md decision-log); D-236 (resequencing + amendment scope); adv-e9-v1.7-amendment-pass-1.md LOW-1 finding; E-9 epic v1.7 (`.factory/stories/epics/E-9-tier-2-native-wasm-migration.md`).

---

### LESSON: Multi-pass adversarial review with rotating angles of attack catches defects that single-angle passes miss

**Source:** D-241 pass-3 finding H-1 (d9f2c86 v1.7 amendment counter-example)
**Date:** 2026-05-05

**Pattern:** Pass-1 took a "verify what the architect said" angle (PASS NITPICK). Pass-2 took a "reverse-trace from ADR obligations to amendment landing sites" angle (PASS NITPICK). Pass-3 took a "forward-simulate story-writer reading + construct counter-example plugin" angle and found H-1: a literal-MUST in the amendment text that conflicts with ADR-015 D-15.3 (the contract assigns block-event emission to the DISPATCHER, not the plugin; the amendment misattributed the obligation to plugins). Without pass-3's angle rotation, story-writer would have authored S-9.01..S-9.07 with redundant plugin-side block emission ACs producing duplicate audit-trail events.

**Codification:**
- Adversary dispatch prompt should suggest 3-4 distinct verification angles by name (verify-architect, reverse-trace, forward-simulate, counter-example, boundary-case, bidirectional-reachability) and instruct each fresh-context pass to pick one not yet exercised.
- ADR-013 should be amended (or its operating skill should) to require angle-of-attack diversity across 3-of-3 NITPICK_ONLY passes — at least 2 distinct angles must be exercised before convergence.
- File as TD-VSDD-057 (Adversary angle-of-attack rotation rule) if not already tracked.

**[codified]** by D-241 lessons.md append.

---

### LESSON: Fix-burst rationale citations must be re-verified against authoritative source

**Source:** D-243 pass-4 finding H-P4-001 (c3855ae v1.8 fix burst)
**Date:** 2026-05-05

**Pattern:** When closing a finding, the architect added a 3-leg rationale (a)/(b)/(c) to justify a renamed event-name choice. Legs (a) and (b) cited real ADR-015 registry entries. Leg (c) cited "Wave 3 acceptance criterion 3" which does not exist (Wave 3 has only AC-1 `pr_throughput` and AC-2 `unknown_category_events`). The choice itself was correct — only the third leg of the rationale was fabricated, likely invented during write-up to strengthen the case without re-reading the source-of-truth.

**Codification:**
- Architect prompt for fix bursts must require: "Any rationale leg of the form 'ADR-XXX [line N | section §X | acceptance criterion N | wave N AC-K] says Y' must include a copy-paste of the cited text in the fix-burst commit message body OR you MUST re-read the cited section before commit."
- Adversary review skill should add a "citation-grounding" pass mode that explicitly falsifies every cited claim against the source-of-truth document.
- File as TD-VSDD-058 (Architect fix-burst rationale citation re-verification rule).

**[codified]** by D-243 lessons.md append.

---

### LESSON: Frontmatter `version:` field must track latest non-reserved Changelog summary table row (3+ recurrent pattern)

**Source:** D-245 pass-5 finding H-P5-001 (067379c v1.9 fix burst); recurrent with F-P6-002 + F-P7-001
**Date:** 2026-05-05

**Pattern:** Architect bumped E-9 v1.8 → v1.9 by adding a v1.9 row to the Changelog summary table, a v1.9 H3 detail section, and rewriting the M-2 closure prose. But did NOT bump the frontmatter `version:` field from "1.8" to "1.9". Body says v1.9; frontmatter says v1.8. Tooling reading frontmatter (state-manager SHA recompute, input-hash, downstream cross-doc references) sees stale version.

This is the THIRD occurrence in this single epic's amendment history: F-P6-002 (v1.5 row missing from summary table), F-P7-001 (v1.4 row regression), and now H-P5-001 (frontmatter version not bumped). Per the lessons-codification rule, 3+ recurrences qualify for codification.

**Codification:**
- Add a hook (or codify in the spec-versioning skill) asserting `frontmatter.version == max(changelog_summary_table.version where date != '—')` for every story/epic file. Run at pre-commit.
- Architect prompt for fix bursts must require: "When you append a new row to the Changelog summary table, ALSO bump the frontmatter `version:` field to match. Do not consider the burst complete until both anchors agree."
- File as TD-VSDD-059 (Frontmatter-version-vs-summary-table validator hook) if not already tracked.

**[codified]** by D-245 lessons.md append.

---

### LESSON: POLICY 1 (append_only_numbering) silent on prose corrections to prior version blocks

**Source:** D-245 pass-5 finding M-P5-001 (067379c v1.9 fix burst)
**Date:** 2026-05-05

**Pattern:** v1.9 burst rewrote v1.8 changelog prose in-place (the M-2 closure entry, lines 720-722 in v1.8 block). The v1.9 block notes the rewrite but the v1.8 block's content is no longer historically accurate to what was authored at the v1.8 burst.

POLICY 1 (append_only_numbering) requires append-only changelog entries but is silent on whether prose corrections within prior version blocks are allowed.

**Codification:**
- Either explicitly forbid in-place edits to prior version blocks (require corrections-only-in-new-version-block; the v1.8 block stays as-was with the fabricated citation; v1.9 block records the correction with a note pointing to the v1.8 defect) — RECOMMENDED.
- OR explicitly allow in-place corrections with a marker (e.g., `[corrected v1.9: ...]` annotation inline in v1.8 prose).
- File as TD-VSDD-060 (POLICY 1 amendment to forbid in-place edits).

**[codified]** by D-245 lessons.md append.

---

### LESSON: Closure-claim verification — "all N items covered" must be substantiated by enumerating the items

**Source:** D-247 pass-6 finding H-P6-001 (dc9a71d v1.10 fix burst)
**Date:** 2026-05-05

**Pattern:** D-242's H-1 closure dropped plugin-side block emission MUST. D-246's M-P5-003 closure claimed "all 5 block-mode hooks now have explicit H-1 option (b) coverage" — but in audit-w16.md, only 3 of 5 block-mode hooks (B-1, B-3, B-7) got explicit dispatcher-emits-automatically wording. B-2 and B-6 were lumped into a "Standard." parenthetical row that does NOT carry the H-1 contract. The closure claim was overstated.

**Codification:**
- Architect prompt for fix bursts must require: "When closing a finding with 'all N items covered' or 'all M sites updated', enumerate the N/M items in the closure note (e.g., 'B-1: line 35; B-2: line 38a; B-3: line 37; ...'). Do not write 'all' without the enumeration."
- Adversary review skill should treat 'all N covered' closure claims as falsifiable — explicitly audit the N items.
- File as TD-VSDD-061 (Architect closure-claim enumeration rule).

**[codified]** by D-247 lessons.md append.

**Note:** D-247 also bootstrapped the OQ (open-questions) tracking category. OQ-W16-001 is the first entry, filed at `.factory/specs/open-questions.md`. Future binary-choice or decision-gate items that block downstream stories should use this register.

---

### LESSON: Sibling-entry wording template consistency must be verified when adding to enumerated lists

**Source:** D-249 pass-7 finding M-P7-002 + M-P7-003 (0ccdf4f v1.11 fix burst)
**Date:** 2026-05-05

**Pattern:** v1.11 fix burst added an explicit H-1 option (b) treatment for B-2 and B-6 in audit-w16.md line 38, satisfying TD-VSDD-061 closure-claim enumeration. But the appended sentence used:
- (a) Internal fix-burst nomenclature ("H-1 option (b)") not used by sibling rows 35/36/37.
- (b) An asymmetric structural form (omitting `(PostToolUse:Edit|Write, on_error=block)` parenthetical) that sibling rows have for their respective hooks.

The enumeration check (TD-VSDD-061) verified all 5 hooks were named — but did not verify they were described with consistent wording template.

**Codification:**
- Extend TD-VSDD-061 to include a "sibling-template-consistency" check: when adding to an enumerated list of N items, verify all N items use the same wording template (event-type parenthetical, terminology, citation prefix).
- Architect prompt for fix bursts must add a step: "Before commit, diff the new sibling entry against existing sibling entries of the same kind. Match wording template (same event-type parenthetical, same terminology, same citation prefix)."
- File as TD-VSDD-062 (Sibling-template-consistency check).

**[codified]** by D-249 lessons.md append.

---

### LESSON: Fix-burst-internal nomenclature must not leak into permanent architecture documents

**Source:** D-249 pass-7 finding M-P7-002 (0ccdf4f v1.11 fix burst)
**Date:** 2026-05-05

**Pattern:** The phrase "H-1 option (b)" originated in pass-3 adversarial review (`adv-e9-v1.7-amendment-pass-3.md`) as a finding ID. Subsequent fix bursts cited this internal ID in changelog prose. v1.11 burst then leaked the phrase into audit-w16.md line 38 — a permanent L4 architecture artifact. To a future reader without access to pass-3 review history, "H-1 option (b)" is opaque.

The same risk exists for any internal ID pattern (H-N, M-PN, L-PN, F-PN-NNN, OQ-W16-NNN if used in non-tracking contexts).

**Codification:**
- Adversary prompts should include a "fix-burst-internal nomenclature scan" axis: grep permanent specs for `H-\d`, `M-P\d`, `F-P\d`, `L-P\d` patterns; flag matches in non-changelog body sections.
- Architect prompts for fix bursts must include: "When fixing a finding identified by internal ID (H-N, M-PN, etc.), do NOT cite the internal ID in the body of permanent specs. Use plain-language descriptions. Internal IDs are reserved for changelog/closure entries."
- File as TD-VSDD-063 (Fix-burst-internal nomenclature leakage check).

**[codified]** by D-249 lessons.md append.

---

### LESSON: Parallel agent dispatches must NOT both use `git commit -a` — burst commit collision risk

**Source:** D-250 retroactive seal — state-manager D-249 + architect D-250 merged commit at 353c172
**Date:** 2026-05-05

**Pattern:** Orchestrator dispatched state-manager (sealing D-249 pass-7 review) and architect (executing D-250 v1.11 → v1.12 fix burst) in parallel. Both agents independently staged their changes via `git add` then ran `git commit`. State-manager's commit ran with `-a` flag (or equivalent broad-add behavior), sweeping up the architect's staged changes into the state-manager's commit object before the architect's commit-message could execute. Result: a single commit (353c172) carries both bursts' content with the state-manager's commit message; architect's distinct commit message and audit-trail boundary lost.

The content is correct (both bursts' edits shipped); only the audit-trail boundary was fuzzed.

**Codification:**
- Orchestrator dispatch rule: when running state-manager + architect in parallel, EITHER (a) state-manager must run AFTER architect commits land (sequential), OR (b) both agents must use scoped `git add <specific-paths>` and `git commit -m` without `-a` flag, and the orchestrator must detect simultaneous-staging via filesystem locks.
- State-manager prompt update: "Stage only the files you wrote in this burst (use `git add <specific-paths>`). Do NOT use `git add -A` or `git commit -a`. If another burst's staged changes are present at commit time, abort and signal collision to orchestrator."
- Architect prompt update: same scoped-add rule.
- File as TD-VSDD-064 (Parallel-burst commit collision prevention rule).

**Mitigation for D-249/D-250:** Audit-trail boundary fuzzed but content correct. D-250 retroactively recorded in Decisions Log; STORY-INDEX bumped twice (1.55 → 1.56 → 1.57) to maintain version-row-per-burst convention.

**[codified]** by D-250 lessons.md append.

---

### LESSON: Outbound cross-document decision-ID anchors must be semantically validated, not just syntactically valid

**Source:** D-251 pass-8 finding M-P8-001 (perf-baseline-w16.md line 156 cited fabricated E-9 D-9.4 gate-model anchor)
**Date:** 2026-05-05

**Pattern:** perf-baseline-w16.md "W-16 Gate Model" section cited `E-9 D-9.4 "Option C"` as gate-model authority. D-9.4 in the E-9 epic is "BC Anchor Strategy — reuse existing BC-7.xx family per hook" — its "Option C" was a back-reference to E-8 D-2 (BC reuse), not a gate-model decision. The cited target's H3 heading was not semantically compatible with the reference context. Defect survived 7 prior adversarial passes because each pass focused on different scopes (positive-verify, reverse-trace from ADR-015, citation-grounding, etc.) but none traced outbound references from arch docs to epic docs.

**Codification:**
- Add a pre-commit hook (or convergence check) that walks outbound decision-ID references (e.g., `E-N D-x.y`, `E-N AC-N`, `ADR-NNN R-N.NN`, `BC-S.SS.NNN`) and asserts the cited target exists. Light version: target ID exists. Strong version: target's H3 heading is semantically compatible with the reference context (LLM-assisted check at convergence time).
- Adversary prompts should add an "outbound-decision-ID semantic-anchor check" axis as a callable angle (alongside existing TD-VSDD-058 ADR-015 citation check).
- File as TD-VSDD-065 (Decision-ID outbound semantic-anchor check).

**[codified]** by D-251 lessons.md append.

---

### LESSON: TD-VSDD-063 fix-burst nomenclature scan must include register-class permanent specs

**Source:** D-252 pass-9 finding L-P9-001 (open-questions.md line 20 leaks M-P6-002)
**Date:** 2026-05-05

**Pattern:** TD-VSDD-063 was codified at v1.12 to prevent fix-burst-internal IDs (H-N, M-PN, F-PN, L-PN) from leaking into permanent specs. The original scan scope was architect documents (audit-w16, gap-analysis, perf-baseline). When OQ-W16-001 was authored at v1.11 (D-248), its `Source:` traceability field carried `M-P6-002` — leaking the same class TD-VSDD-063 prevents, but in a register-class permanent-spec file the scan didn't cover.

**Codification:**
- Extend TD-VSDD-063 pre-commit scan scope from architect docs to `.factory/specs/**` (open-questions register + any future register-class artifacts: TD-register, OQ-register, etc.).
- Architect prompt for fix bursts must now run the leak scan against the broader scope.
- File as TD-VSDD-066 (TD-VSDD-063 scope extension to register-class permanent specs).

---

### LESSON: AC-numbered values must cross-validate against underlying measurement source

**Source:** D-254 pass-11 finding H-P11-001 (E-9 AC-3 cited superseded ~14MB target)
**Date:** 2026-05-05

**Pattern:** E-9 AC-3 (the contractual gate) cited "(~14MB)" advisory soft cap. ADR-014 line 45 had explicitly retired this projection: "the prior ~14MB target derived from research §Q3's 7.2MB projection is superseded — that figure was a projection, not a measurement." perf-baseline-w16.md line 163 carries actual value `w16_advisory_bundle_soft_cap_bytes = 643686 bytes` (rc.1 × 2 = ~644KB) — three orders of magnitude smaller than the obsolete ~14MB projection.

The original ~14MB cite came from audit-w16.md Section 5 R-W16-003 (pre-amendment audit-time prose, when the projection was current). When ADR-014 retired the projection, the supersession propagated to perf-baseline-w16.md and ADR-014 itself but NOT to AC-3 in the E-9 epic. 10 prior adversarial passes missed this because they audited along axes (positive verification, ADR-015 citations, POLICY 6 names, etc.) that didn't enumerate every numeric claim.

**Codification:**
- Adversary skill should add a "numeric-cross-anchor" review axis: enumerate every numeric claim in spec/AC text and cross-validate against the underlying measurement source (perf-baseline, ADR amendment, S-N.NN baseline). Apply at convergence-time at minimum.
- Architect prompt for ACs that cite measurements must require: "When citing a measurement value (bytes, ms, %), include the file:line of the underlying measurement source so the citation is auditable."
- File as TD-VSDD-067 (Numeric-cross-anchor review axis for adversary).

**[codified]** by D-254 lessons.md append.

**[codified]** by D-252 lessons.md append.
