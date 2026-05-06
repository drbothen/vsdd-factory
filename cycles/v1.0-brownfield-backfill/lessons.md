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

**[codified]** by D-252 lessons.md append.

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

---

### LESSON: Recursive-scrub — fix-burst replacement text must be re-verified against the same forbidden-pattern set the original violated

**Source:** D-255 pass-12 finding H-P12-001 (open-questions.md line 20 v1.14 scrub introduced new internal tokens)
**Date:** 2026-05-05

**Pattern:** v1.14 fix burst's M-P11-001 closure replaced fix-burst-internal tokens (`D-247`, `M-P6-002`, `b04843d`, `pass-6 finding`) with a different set of fix-burst-internal tokens (`M-1 closure forward-pointer`). The replacement text was non-compliant with the very rule it cited (TD-VSDD-063). The forward-pointer was also unresolvable: the named target file contained no "M-1" anywhere.

**Codification:**
- After applying any TD-VSDD-063 / TD-VSDD-066 scrub, run a SECOND grep against the just-edited line(s) using the same forbidden pattern set: `D-[0-9]{3}`, `M-P[0-9]+`, `H-P[0-9]+`, `L-P[0-9]+`, `F-P[0-9]+`, `M-[0-9]+ closure`, `pass-[0-9]+ finding`, `cycle [0-9a-f]{7}`. If ANY match in the just-edited region, the fix is non-compliant; revise.
- Architect/state-manager prompts for any nomenclature scrub burst must include the recursive-scrub verification command in the pre-commit checklist.
- File as TD-VSDD-068 (Recursive-scrub check for nomenclature replacement text).

Additionally: forward-pointers in scrubbed text MUST resolve in the named target file. After writing a replacement that says `see X §Y`, grep target file X for §Y or its anchor — verify it exists.

**[codified]** by D-255 lessons.md append.

---

### LESSON: TD-VSDD-068 recursive-scrub must extend to line-number accuracy for new cross-document citations

**Source:** D-256 pass-13 finding M-P13-001 (open-questions.md line 20 v1.15 fix introduced off-by-one citation)
**Date:** 2026-05-05

**Pattern:** v1.15 H-P12-001 fix replaced unresolvable `M-1 closure forward-pointer` with a "bidirectional anchor" `gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**")`. The quoted substring was correct; the line-number annotation was off-by-one (actual line is 325). Recursive-scrub (TD-VSDD-068) checked forbidden tokens but did not validate line-number accuracy.

**Codification:**
- Extend TD-VSDD-068 recursive-scrub: when a citation of form `<filename> line N ("<quoted text>")` is added in a fix burst, the scrub MUST grep `"<quoted text>"` in `<filename>` and confirm exactly one match at line `N`. If line number doesn't match grep result, abort.
- File as TD-VSDD-069 (Line-accuracy extension to recursive-scrub).

**[codified]** by D-256 lessons.md append.

---

### LESSON: TD-VSDD-065 outbound-decision-ID semantic-anchor check must extend to section/subsection headings

**Source:** D-257 pass-14 finding M-P14-001 (perf-baseline-w16.md line 154 H2 "Option C" non-resolving anchor)
**Date:** 2026-05-05

**Pattern:** TD-VSDD-065 (codified at D-251 from pass-8) requires that all outbound decision-ID anchors (`E-N D-x.y`, `ADR-NNN R-N.NN`, etc.) resolve to semantically compatible target structures. Pass-8 applied this at the in-text-citation level for line 156 of perf-baseline-w16.md (E-9 D-9.4 → E-9 AC-3). However, the H2 heading at line 154 of the same file — `## W-16 Gate Model (ADR-014 R-8.09 Revised — Option C)` — was not scrubbed. ADR-014 R-8.09 Amendment 2026-05-03 has no Options A/B/C taxonomy. The "Option C" anchor in the H2 heading does not resolve.

Pass-14 (AC chain audit + section-heading angle) caught this 6 passes after pass-8 originally closed the related in-text issue.

**Codification:**
- Extend TD-VSDD-065 scope from "in-text decision IDs" to "all section/subsection headings (`# H1`, `## H2`, `### H3`) that name an external authority's decision/option/choice/amendment."
- Adversary prompt should add a section-heading-anchor sweep alongside the in-text-anchor sweep.
- Architect prompts for fix bursts MUST scrub BOTH headings AND inline citations — fix the in-text reference and verify no related H2/H3 still carries the obsolete anchor.
- File as TD-VSDD-070 (TD-VSDD-065 scope extension to section/subsection headings).

**[codified]** by D-257 lessons.md append.

---

### LESSON: When an OQ is filed citing an epic as scope-owner, the epic's Open Questions table must contain a corresponding row

**Source:** D-258 pass-15 finding M-P15-001 (OQ-W16-001 absent from E-9 Open Questions table despite being filed in v1.11/D-248)
**Date:** 2026-05-05

**Pattern:** OQ-W16-001 was filed at v1.11/D-248 to `.factory/specs/open-questions.md` to track the `vsdd.host.*` registry-prefix decision binary-choice. The OQ correctly cited SS-01 implementer or E-10 Wave 1 architect as owner. gap-analysis-w16-subprocess.md got the bidirectional forward-pointer to OQ-W16-001 (line 325). But E-9 epic — the canonical discoverability hub for E-9-scope OQs (it has its own Open Questions table at lines 379-385 enumerating OQ-1, OQ-2, OQ-3) — was NOT updated.

A story-writer authoring S-9.07 reading E-9 §Open Questions would miss the binary-choice gate; recovery only via gap-analysis transitive path. Discoverability hub asymmetry.

**Codification:**
- Add a process-step (or hook): when an OQ is filed in `.factory/specs/open-questions.md` citing an epic E-N as scope-owner, the same burst must verify (or append) a corresponding row in the epic's Open Questions table.
- Adversary's discoverability-audit angle should add a sweep: enumerate every OQ in `.factory/specs/open-questions.md`, then verify each is also listed in its scope-owner epic's Open Questions table.
- File as TD-VSDD-071 (OQ-table propagation hook from open-questions register to epic body).

**[codified]** by D-258 lessons.md append.

---

### LESSON: Body-grep extension to recursive-scrub — fix-burst must body-grep for retired figures across whole file, not just at primary fix site

**Source:** D-260 pass-17 finding H-P17-001 (E-9 R-W16-003 mitigation "~14MB" residue 16 passes after v1.14 H-P11-001 AC-3 fix)
**Date:** 2026-05-05

**Pattern:** v1.14 / D-254 fix-burst sealed H-P11-001 by scrubbing AC-3 line 368 of the retired "(~14MB)" advisory soft cap target. The fix was correct at line 368 BUT did not body-grep the rest of E-9 epic for sibling residue. Pass-17's linguistic-uniformity + numerical-cross-table angle caught R-W16-003 mitigation cell at line 353 still containing "(~14MB)" — a sibling row in the same Risk table that mirrors AC-3's contractual gate.

A 1-second body-grep `grep -n '14MB' E-9*.md` at v1.14 close would have caught the sibling residue. The S-7.01 Partial-Fix Regression Discipline named exactly this pattern.

**Codification:**
- Extend TD-VSDD-068 recursive-scrub: when a fix burst replaces a retired numeric or named value (e.g., "~14MB", "Option C", "post-rc.4"), the same burst MUST body-grep the entire file (and arguably all amendment-scope files) for the retired value before commit. Any non-changelog occurrence is a sibling regression that must be fixed in the same burst.
- Architect/state-manager prompts for any value-replacement fix MUST include: "After applying the primary fix, run `grep -n '<retired_value>' <file>` and confirm only changelog/H3 matches remain. If non-changelog body matches exist, treat as sibling regressions and fix in same burst."
- File as TD-VSDD-072 (Retired-figure body-grep extension to recursive-scrub).

**[codified]** by D-260 lessons.md append.

---

### LESSON: Arch-doc-class files MUST carry `last_amended:` field when body amendments occur in fix bursts

**Source:** D-261 pass-18 finding M-P18-001 (5th re-flag of frontmatter-vs-body convention question; S-7.02 recurrence threshold met)
**Date:** 2026-05-05

**Pattern:** D-239 codified annotate-in-place for arch docs (no version bump on body amendment). However, the convention was silent on frontmatter currency signals. Across passes 5/6/14/15/18, adversaries re-surfaced the same finding: 4 amendment-touched arch-doc-class files (gap-analysis-w16-subprocess.md, audit-w16.md, perf-baseline-w16.md, open-questions.md) had `version: "1.0"` frontmatter despite 5+ body amendment bursts. Each occurrence was DEFERRED individually. S-7.02 lessons-codification rule (3+ recurrences) was met at pass-14; finally enforced at pass-18.

**Codification (Option A adopted):**
- All amendment-touched arch-doc-class files MUST carry `last_amended: <YYYY-MM-DD>` in frontmatter.
- D-239 annotate-in-place body convention is preserved (body retains dated H2 amendment annotations); frontmatter gains parallel structured signal.
- When a fix burst body-amends an arch-doc-class file, the same burst MUST update `last_amended:` to the burst date.
- Adversary frontmatter-consistency-audit angle (TD-VSDD-057 angle inventory) should accept files where `last_amended` matches latest body-amendment commit date, OR where the file has no body amendments since `timestamp:`.
- File as TD-VSDD-073 (last_amended mandatory for amended arch-doc-class files).

**[codified]** by D-261 lessons.md append.

---

### LESSON: TD-VSDD-073 last_amended convention must extend to BCs cited in amendment landings

**Source:** D-263 pass-20 finding M-P20-002 + PG-P20-001 (BC-1.05.036 emits non-ADR-015-conforming event name despite amendment surface absorbing ADR-015 awareness)
**Date:** 2026-05-05

**Pattern:** v1.7 amendment burst absorbed ADR-015 awareness into E-9 epic + 3 arch docs (D-236 4-file impact map). BC-1.05.035 + BC-1.05.036 were created at D-224 within this amendment cycle (substituting for withdrawn BC-2.02.013) and are cited from gap-analysis-w16-subprocess.md and audit-w16.md amendment blocks. But the BCs themselves were never updated for ADR-015 awareness. BC-1.05.036 §Description still names the success-path event `host.exec_subprocess.completed` (no `vsdd.` prefix, no `.v1` suffix) — violating ADR-015 D-15.2 reverse-DNS naming and contradicting OQ-W16-001 which tracks the binary-choice resolution.

19 prior passes did not catch this because each focused on convention/anchor/citation correctness within E-9 + arch-docs scope. Pass-20 was the first to simulate the downstream S-9.07 story implementer reading BC-1.05.036 as their contract source.

**Codification:**
- Extend TD-VSDD-073 (last_amended convention for arch-doc-class) to include BCs cited in amendment landings.
- When an amendment burst changes a contract that a BC implements, the same burst MUST update the BC's frontmatter `last_amended:` AND add an awareness clause to the BC body.
- Adversary pass should include "downstream-implementer-simulation" as a callable angle (TD-VSDD-057 menu addition).
- File as TD-VSDD-074 (TD-VSDD-073 scope extension to BCs cited in amendment landings).

**[codified]** by D-263 lessons.md append.

---

### LESSON: Frontmatter `last_amended:` adds MUST trigger dependent-citation propagation refresh; fix bursts citing source-code constants MUST verify against source

**Source:** D-264 pass-21 findings H-P21-001 (fabricated error codes) + H-P21-002 (off-by-one line citation caused by frontmatter-line-shift)
**Date:** 2026-05-05

**Pattern (H-P21-001 — source-code-verification failure):** v1.21 burst (D-263 L-P20-002) added BC-1.05.036 §Postconditions item 5 with error codes TIMEOUT (-7) and OUTPUT_TOO_LARGE (-8). Actual ABI codes per `crates/factory-dispatcher/src/host/mod.rs:181-182`: TIMEOUT = -2, OUTPUT_TOO_LARGE = -3. The fix burst invented the wrong codes without reading the source file. A regression introduced by a fix burst.

**Pattern (H-P21-002 — dependent-citation-propagation failure):** v1.20 burst (D-261) added `last_amended: 2026-05-05` to gap-analysis-w16-subprocess.md frontmatter at line 8. This shifted every subsequent line by +1. The line-326-quoted-text "Resolution tracked in **OQ-W16-001**" was previously at line 325 (set by v1.16/D-256 grep at that time). open-questions.md:21 cited "gap-analysis line 325" — that citation became stale. v1.20 burst did not refresh dependent inbound citations.

This is the THIRD recurrence of the line-citation off-by-one defect class (after L-P9-001 and M-P13-001). S-7.02 codification threshold (3+) met.

**Codification:**
- **Source-code-verification discipline (TD-VSDD-075 sub-rule 1):** Fix bursts that cite source-code constants (error codes, struct fields, enum variants, const values) MUST open the cited source file, read the actual values, and quote the exact line in the fix-burst commit message body as proof of verification. No inventing constants.
- **Dependent-citation-propagation discipline (TD-VSDD-075 sub-rule 2):** When a fix burst adds `last_amended:` (or any frontmatter field that shifts subsequent line numbers), the same burst MUST grep all 5 in-scope files for inbound citations of form `<filename> line N`. For each match, re-grep the cited file for the quoted text and verify the line number still resolves. Refresh stale citations in the same burst.
- Architect/state-manager prompts MUST include this dependent-citation refresh step in any frontmatter-add burst.
- File as TD-VSDD-075 (last_amended dependent-citation propagation requirement + source-code-verification discipline).

**[codified]** by D-264 lessons.md append.

---

### LESSON: Intra-document semantic-sibling sweep (TD-VSDD-076; extends TD-VSDD-075 from inter-document to intra-document scope)

**Source:** D-265 pass-22 finding H-P22-001 (BC-1.05.036 §Postcondition 5 correction at v1.22 NOT propagated to §Related BCs lines 61-62 and §EC-004 line 86 within the SAME BC)
**Date:** 2026-05-05

**Pattern:** v1.22 burst (D-264) correctly applied source-code verification to fix BC-1.05.036:52 (TIMEOUT/OUTPUT_TOO_LARGE error codes -7/-8 → -2/-3). The Postcondition 5 wording was also corrected to state "WITHOUT emitting any event" per gap-analysis §1. But the SAME BC's §Related BCs and §EC-004 sections retained pre-correction wording implying error-path events were emitted. Intra-document contradiction within same BC.

TD-VSDD-075 (codified at v1.22) covered inter-document line-citation refresh and source-code-verification. It did NOT cover intra-document semantic siblings.

**Codification:**
- When a fix burst corrects a Postcondition or any normative claim within a BC, the same burst MUST grep the SAME BC for sibling sections (§Related BCs, §Edge Cases, §Canonical Test Vectors, §Postconditions, §Description) for prior wording that contradicts the correction.
- Each contradicting sibling must be updated in the same burst.
- File as TD-VSDD-076 (Intra-document semantic-sibling sweep extension to TD-VSDD-075).

**[codified]** by D-265 lessons.md append.

---

### LESSON: Lessons-corpus artifacts (lessons.md + open-backlog-post-rc8.md) MUST maintain bidirectional consistency

**Source:** D-267 pass-24 finding M-P24-001..006 + PG-P24 (6 MEDs revealing lessons-corpus internal coherence defects: stub entries, section-boundary violations, marker orphaning, bidirectional drift between lessons.md and open-backlog)
**Date:** 2026-05-05

**Pattern:** The 20 codified TD-VSDD lessons (057-076) live in two canonical artifacts: `cycles/<cycle>/lessons.md` (full lesson body + Source citation + [codified] marker) and `cycles/<cycle>/open-backlog-post-rc8.md` (TD-VSDD-NNN bullet entry under appropriate H2 section). Pass-24 found 6 distinct coherence defects between these two files: (1) 3 stub entries in open-backlog where body content was merged into the next numbered entry (TD-VSDD-069 body merged into TD-VSDD-070, TD-VSDD-071 body merged into TD-VSDD-072, TD-VSDD-075 body merged into TD-VSDD-076); (2) 3 entries placed after the wrong H2 boundary (TD-VSDD-073/075/076 placed after `## Lessons codified during the cycle` instead of under `## New from Phase D-4`); (3) 5+ orphaned `[codified] by D-NNN` markers in lessons.md displaced from their associated lesson (D-252 after TD-VSDD-067, D-257 after TD-VSDD-071, D-261 after TD-VSDD-074); (4) bidirectional drift on TD-VSDD-074 Source field (`+ PG-P20-001` present in open-backlog, missing from lessons.md); (5) non-monotonic TD-VSDD-NNN ordering in open-backlog D-4 section; (6) duplicated content concatenation across adjacent entries.

**Codification:**
- Pre-commit hook: when adding a new TD-VSDD-NNN entry, validate (a) lessons.md has lesson body + Source + matching [codified] marker IMMEDIATELY following lesson content, (b) open-backlog has matching bullet under correct H2 with body content (not stub), (c) TD-VSDD-NNN ordering is monotonic in open-backlog, (d) Source citations match bidirectionally.
- State-manager prompt for any TD-VSDD-NNN codification burst MUST include lessons-corpus coherence verification.
- Convention-meta-audit pass should be added to the regular adversary angle rotation (TD-VSDD-057 menu).
- File as TD-VSDD-077 (Lessons-corpus bidirectional coherence validation hook).

**[codified]** by D-267 lessons.md append.

---

### LESSON: BC postconditions citing concrete enumerations from source code MUST be source-of-truth-verified (TD-VSDD-078 — extension of TD-VSDD-075)

**Source:** D-268 pass-25 finding H-P25-001 (BC-1.05.036:52 fabricated denial-path enumeration not present in source code)
**Date:** 2026-05-05

**Pattern:** BC-1.05.036 §Postcondition 5 was edited at v1.21 (D-263 L-P20-002) to clarify error-path event reality. The edit listed 4 denial paths (binary not allowed, shell bypass not acknowledged, env not allowed, cwd not allowed) — but the actual `emit_denial(...)` callsites in `crates/factory-dispatcher/src/host/exec_subprocess.rs:148/155/162/169` use different reason strings (`no_exec_subprocess_capability`, `binary_not_on_allow_list`, `shell_bypass_not_acknowledged`, `setuid_or_setgid_binary`). The fabricated "env_allowed/cwd_allowed" paths don't emit at all (env silently filtered; cwd unenforced).

The fabrication survived 4 passes (21, 22, 23, 24) because each focused on different scopes — convention checks, sibling-propagation, narrative coherence, lessons-corpus. Pass-25's source-code traceability exhaustive sweep was the first to grep the source file for the actual emit callsites.

**Codification:**
- TD-VSDD-075 sub-rule (source-code-verification) is extended: when a BC postcondition cites a CONCRETE ENUMERATION (list of error codes, list of denial reasons, list of fields, list of paths), the burst MUST grep the cited source file for each enumeration item and verify presence/absence.
- Architect/state-manager prompts MUST include enumeration verification when fixing or authoring BC postconditions that list source-derived items.
- Adversary's source-code-traceability angle (TD-VSDD-057 menu addition) should be a regular axis.
- File as TD-VSDD-078 (BC postcondition source-of-truth enumeration verification — extension of TD-VSDD-075).

**[codified]** by D-268 lessons.md append.

---

### LESSON: TD-VSDD-076 sibling-sweep needs explicit terminology-grep checklist for amendment-class fixes

**Source:** D-271 pass-28 findings H-P28-001/002 + M-P28-001/002/003 (third recurrence of TD-VSDD-076 self-violation)
**Date:** 2026-05-05

**Pattern:** Three TD-VSDD-076 self-violations recorded:
- pass-24 H-P24-001 (v1.23 burst codified TD-VSDD-076 but its OWN BC-1.05.036 truncated:bool annotation had `[ ]` vs `/* */` sibling-template inconsistency)
- pass-25 M-P25-001 (v1.24 fix EC-003 sibling-aligned to Postcondition 5 only after caught)
- pass-28 H-P28-001/002 + M-P28-001/002/003 (v1.26 silence-audit burst scrubbed line 51 only; siblings at lines 38, 135, §Edge Cases, §Canonical Test Vectors all retained stale wording)

S-7.02 recurrence threshold (3+) met. The TD-VSDD-076 codification is sound but its application is consistently incomplete because each fix burst greps for the EXACT phrase the prior adversary cited rather than the BROADER terminology family.

**Codification:**
- Extend TD-VSDD-076 with explicit grep-checklist for amendment-class silence-audit / sibling-sweep fixes:
  - Before commit, grep for ALL retired-terminology variants across the BC: e.g., "sink chain", "Router", "SinkRegistry", "multi-sink", "fan-out", "datadog", "honeycomb", "try_send" — NOT just the literal phrase the adversary cited.
  - Sweep these sections: §Description, §Postconditions, §Invariants, §Edge Cases, §Canonical Test Vectors, §Purity Classification, §Refactoring Notes.
  - The fix-burst MUST achieve zero matches across non-changelog body for ALL retired-terminology variants.
- Architect/state-manager prompts MUST run the broader grep before commit, not just the narrow literal phrase grep.
- File as TD-VSDD-079 (TD-VSDD-076 extension: terminology-family grep checklist for sibling-sweep fixes).

**[codified]** by D-271 lessons.md append.

---

### LESSON: TD-VSDD-079 narrative-discipline must mechanize as pre-commit hook to prevent self-violations

**Source:** D-272 pass-29 finding H-P29-001 (v1.27 burst codified TD-VSDD-079 8-term family grep but its own pre-commit grep was 2 terms only)
**Date:** 2026-05-05

**Pattern:** v1.27 burst (D-271) codified TD-VSDD-079 with explicit 8-term grep checklist: `sink chain`, `Router`, `SinkRegistry`, `DlqWriter`, `multi-sink`, `fan-out`, `Datadog`, `Honeycomb`, `try_send`. The burst's own pre-commit verification ran only `grep -n 'sink chain\|try_send' BC-1.05.036.md` — a 2-term grep that satisfied the literal "previous adversary's cited residue" but not the codified family. Result: the burst that codified TD-VSDD-079 simultaneously violated it (Datadog/Honeycomb + fan-out residue at BC-1.05.036:51).

This is the same self-violation pattern previously caught at:
- pass-24 H-P24-001 (v1.23 codified TD-VSDD-076 but had truncated:bool annotation inconsistency)
- pass-25 M-P25-001 (v1.24 fix EC-003 sibling-aligned only after caught)
- pass-28 H-P28-001/002 (v1.26 silence-audit scrubbed line 51 only)
- pass-29 H-P29-001 (v1.27 grep was 2-term not 8-term)

The recurrence is structural: narrative-discipline lessons consistently fail to bind their own authoring burst because the burst-author runs the literal grep-cite from the prior adversary, not the canonical full family check.

**Codification:**
- Implement pre-commit hook `validate-bc-terminology-family.sh` that runs the TD-VSDD-079 8-term grep automatically against any modified BC or arch-doc file.
- Hook FAILS commit if any term matches outside `### Changelog` or `### v1.X` H3 sections.
- Architect/state-manager prompts MUST invoke the hook before commit.
- Mechanical enforcement is required because narrative discipline has failed 5 consecutive times (passes 24/25/28/29 + at-least-one-undetected pre-pass-24 instance).
- File as TD-VSDD-080 (Mechanize TD-VSDD-079 family-grep as pre-commit hook).

**[codified]** by D-272 lessons.md append.

---

### LESSON: Mechanism-verification beyond string-presence-grep — TD-VSDD-079 / TD-VSDD-080 extension

**Source:** D-277 pass-34 finding HIGH-P34-001 (v1.30 Fix 3 introduced factual error: BC asserts NUL bytes rejected via `read_wasm_string` but source code shows `read_wasm_string` only rejects non-UTF-8)
**Date:** 2026-05-05
**Category:** spec-verification-discipline

**Pattern:** v1.30 burst's Fix 3 (MED-P33-003 disambiguation) anchored NUL-byte rejection to `read_wasm_string` error path. The post-edit grep-verification (TD-VSDD-079 8-term family + literal-phrase greps) all PASSED — the strings checked were properly removed/replaced. But the cited mechanism ("read_wasm_string rejects NUL bytes") was never verified against actual source code. Source: `crates/factory-dispatcher/src/host/memory.rs:47-54` shows `read_wasm_string` only fails on `String::from_utf8` errors; NUL bytes (0x00) are valid UTF-8 and pass through cleanly. Actual NUL handling: `Path::canonicalize()` on Unix returns EINVAL via CString conversion → ladder step 2 → CAPABILITY_DENIED, NOT step 1 INVALID_ARGUMENT.

The v1.30 fix's claim was string-level plausible but mechanism-level wrong. TD-VSDD-079 / TD-VSDD-080 grep-checklist mandates string-presence verification but does NOT mandate mechanism-behavior verification.

**Codification:**
- Extend TD-VSDD-079/080 with a new sub-rule: when a fix-burst CITES a source-code mechanism as performing a particular check (e.g., "NUL bytes rejected via read_wasm_string error path"), the architect/state-manager MUST read the cited source file and verify the mechanism actually performs the asserted behavior — NOT just verify the string is present/absent in the spec.
- Architect prompts must include: "Before commit, READ each source-code line cited in the fix prose and confirm the asserted behavior. Quote the actual source code in the commit message body. Do NOT rely solely on grep verification."
- Adversary's source-code-traceability angle (TD-VSDD-057 menu) should add a "mechanism-verification" sub-axis that reads source code for each cited mechanism, not just verifies string anchors.
- File as TD-VSDD-081 (Mechanism-verification beyond string-presence-grep).

**[codified]** by D-277 lessons.md append.

---

### LESSON: Sibling-mechanism sweep + bidirectional-sibling-disclosure (extension of TD-VSDD-076 + TD-VSDD-081)

**Source:** D-278 pass-35 findings HIGH-P35-001 (mechanism-error sibling) + MED-P35-002 (reverse-direction disclosure asymmetry)
**Date:** 2026-05-05
**Category:** spec-verification-discipline

**Pattern:** v1.31 burst (D-277) correctly fixed NUL-byte mechanism error per TD-VSDD-081. But the SAME BC contained a structural sibling mechanism (symlink-escape detection) using the same `Path::canonicalize()` predicate-shape but a different specific predicate (`..` scan vs prefix check). The sibling mechanism had the same class of error and survived all 35 prior passes. Plus v1.31 added a forward-direction sibling-disclosure NOTE (BC-1.05.036 → BC-1.05.035) but missed reverse-direction (BC-1.05.035 → BC-1.05.036).

**Codification:**
- Extend TD-VSDD-081 with sub-rule: when a fix-burst corrects a mechanism (e.g., "NUL byte rejected via X"), the burst MUST sweep ALL mechanisms within the SAME BC that invoke the same std-lib function (e.g., all uses of `canonicalize()`) and verify each one's predicate is correctly described.
- Extend TD-VSDD-076 with sibling-disclosure bidirectional-sweep rule: when adding a NOTE to BC-A's §Related BCs row referencing BC-B, the inverse row in BC-B's §Related BCs MUST receive a symmetric disclosure if applicable.
- ADR line-number citations should prefer quoted-phrase anchors over line numbers when the cited content is short and grep-able.
- File as TD-VSDD-082 (Sibling-mechanism sweep + bidirectional-sibling-disclosure).

**[codified]** by D-278 lessons.md append.

---

### LESSON: Architectural-concept-anchoring rule — normative postconditions cannot rely on coined concepts without upstream definition

**Source:** D-279 pass-36 finding HIGH-P36-002 (v1.32 introduced "trusted project-root prefix" concept with no source-code, gap-analysis, or ADR anchor)
**Date:** 2026-05-05
**Category:** spec-verification-discipline

**Pattern:** v1.32 mechanism-sibling-sweep correctly identified that `Path::canonicalize()` resolves `..` segments away (so the prior "`..` scan" mechanism couldn't fire). The replacement mechanism cited a "trusted project-root prefix" concept. Verification (pass-36):
- HostContext at host/mod.rs:49-76 has `cwd: PathBuf` but no `project_root` field
- gap-analysis Section 5 (cited as authority) proposes string-level `../` guard, NOT prefix-check
- No HOST_ABI.md, ADR, or other architecture document defines "project-root prefix" or any equivalent
- read_file.rs uses path_allow LOOP, not single-prefix check

The mechanism was unimplementable as written. Plus it was anti-correct: `/usr/bin/bash` (canonical happy path) doesn't start with project_root, so the new mechanism universally rejects the S-9.07 use case.

This is a class of error TD-VSDD-076/078/079/080/081/082 do NOT catch: introducing a NEW architectural CONCEPT (vs. correcting an existing mechanism) requires upstream definition before normative postcondition can rely on it.

**Codification:**
- When a fix-burst introduces a NEW architectural concept (e.g., "trusted project-root prefix", "X allow-list", "Y registry") into a normative postcondition, the burst MUST FIRST verify the concept is defined in an upstream architecture document (gap-analysis, ARCH, ADR, HOST_ABI.md, BC-INDEX) AND verify the production code has a corresponding field/function/data structure.
- Architect/state-manager prompts MUST include: "Before introducing a NEW architectural concept in spec prose, verify (a) the concept is defined in an upstream document with an explicit citation, AND (b) the production code has a corresponding implementation site (or open OQ tracks the implementation)."
- Adversary should add "architectural-concept-anchoring" axis to TD-VSDD-057 menu.
- File as TD-VSDD-083 (Architectural-concept-anchoring rule).

**Meta-pattern:** This is the 9th burst-induced self-violation in the convergence cycle (passes 24, 25, 28, 29, 31, 33, 34, 35, 36). The cycle has codified TD-VSDD-076 → 077 → 078 → 079 → 080 → 081 → 082 → 083. Each lesson catches ONE class of burst-induced regression; new fresh-context angles continue finding new classes. The structural insight: narrative-discipline lessons fail their own burst because each burst's verifier checks string-presence, not semantic correctness against source-of-truth.

**[codified]** by D-279 lessons.md append.

---

## TD-VSDD-084 — Asserted-goal vs mandated-mechanism coherence (PROVISIONAL — flagged by pass-37, codification deferred pending recurrence)

**Source:** PG-P37-001 (HIGH-P37-002 closure)

**Class:** When a BC's H1 / Description / Architecture Anchors assert a behavioral GOAL G (e.g., "TOCTOU prevention", "atomic upsert", "deduplication"), the BC's Postconditions MUST collectively be sufficient to deliver G — not merely address one sub-step of G.

**Symptom in pass-37:** BC-1.05.035 asserted TOCTOU prevention as architectural rationale, but Postcondition 1 mandated canonicalize at the allow-check site only (line 152). The spawn site (line 230, inside execute_bounded) was untouched in the spec — leaving a TOCTOU window between check and spawn that defeats the asserted goal.

**Codification status:** PROVISIONAL. This is the FIRST recurrence; per S-7.02 recurrence threshold (3+), full codification is deferred. If this pattern recurs in v1.34→v1.35→v1.36 fix bursts, escalate to a hook-enforced lint per TD-VSDD-080's mechanization principle.

**Adversary axis (when verifying):** For each H1/Description/Architecture-Anchors asserted goal G, enumerate every step in the call chain that MUST achieve some sub-condition for G to hold. Verify each Postcondition mandates the corresponding sub-condition. Flag any postcondition that addresses an upstream sub-condition while leaving downstream sub-conditions to implementer interpretation.

**Date:** 2026-05-05
**Burst:** D-280 (E-9 v1.33→v1.34)

---

## Open Backlog

- **TD-VSDD-084** (Asserted-goal vs mandated-mechanism coherence — PROVISIONAL): When a BC's H1/Description/Architecture Anchors assert a behavioral GOAL G (e.g., "TOCTOU prevention", "atomic upsert", "deduplication"), the BC's Postconditions MUST collectively be sufficient to deliver G — not merely address one sub-step of G. First instance: BC-1.05.035 asserted TOCTOU prevention but Postcondition 1 mandated canonicalize at allow-check site (line 152) only; spawn site (line 230, inside execute_bounded) was untouched. Full codification deferred pending recurrence per S-7.02 threshold (3+). Source: D-280 HIGH-P37-002.
- **TD-VSDD-083** (Architectural-concept-anchoring rule): When a fix-burst introduces a NEW architectural concept into a normative postcondition, MUST verify (a) the concept is defined in an upstream document (gap-analysis, ARCH, ADR, HOST_ABI.md) with explicit citation, AND (b) production code has a corresponding field/function/data structure. Extends TD-VSDD-076/078/079/080/081/082. Source: D-279 HIGH-P36-002.
- **TD-VSDD-082** (Sibling-mechanism sweep + bidirectional-sibling-disclosure): When fix-burst corrects a mechanism, MUST sweep ALL mechanisms in the SAME BC using the same std-lib function. When adding sibling-disclosure NOTE to BC-A §Related BCs → BC-B, inverse BC-B §Related BCs → BC-A MUST receive symmetric disclosure. ADR line-number citations prefer quoted-phrase anchors. Extends TD-VSDD-076 + TD-VSDD-081. Source: D-278 HIGH-P35-001 + MED-P35-002.
- **TD-VSDD-081** (Mechanism-verification beyond string-presence-grep): When fix-burst cites a source-code mechanism as performing a specific check, MUST read the cited source file and verify the mechanism actually performs the asserted behavior. Extends TD-VSDD-079/080 grep-checklist with mechanism-behavior verification sub-rule. Source: D-277 HIGH-P34-001.

---

## TD-VSDD-085 — TV-witness mechanization for new mechanism strings (extension of TD-VSDD-080)

**Source:** Pass-38 HIGH-P38-001 + MED-P38-001 (5th observed recurrence of "fix-burst-introduces-new-mechanism-but-omits-TV-witness")

**Class:** When a fix burst introduces a new mechanism string (e.g., new `emit_denial` reason like `binary_canonicalize_failed`) or new normative Edge Case, the SAME burst MUST add a Canonical Test Vector witness row asserting that exact mechanism string in the expected emission. Narrative discipline (TD-VSDD-076/079) keeps failing for this class — recurrences observed at passes 24, 29, 31, 37, 38.

**Codification:** Promote TD-VSDD-080's proposed `validate-bc-terminology-family.sh` hook to also enforce: any new emit_denial reason string introduced in a BC body MUST appear in at least one row of the BC's §Canonical Test Vectors table. Pre-commit hook reads recently-modified BC files, extracts emit_denial reason strings via grep, and verifies each appears in a TV row.

**S-7.02 threshold:** 5 prior fix-burst-introduced-mechanisms-without-TV-witness instances across passes 24/29/31/37/38, including 2 within pass 38 (HIGH-P38-001 4th + MED-P38-001 5th) → MET. Codification escalated from PROVISIONAL to NORMATIVE.

**Adversary axis (when verifying):** For each new normative Edge Case row or new mechanism string introduced in this burst, grep the BC's §Canonical Test Vectors for a row witnessing it. Flag any new mechanism without witness as HIGH severity.

**Date:** 2026-05-05
**Burst:** D-281 (E-9 v1.34→v1.35)

**[codified]** by D-281 lessons.md append.

---

## TD-VSDD-086 — Orchestrator mission-template artifact-filename resolution

**Source:** Pass-39 LOW-P39-002 [process-gap]

**Class:** When the orchestrator dispatches an adversary or other agent with a task that references repo-internal filenames (e.g., ADRs, BCs), the mission template MUST resolve those filenames via Glob at dispatch-time rather than hardcoding a slug. Hardcoded slugs become stale when files are renamed (as ADR-015 was renamed from `-emit-contract` to `-schema` at some prior burst).

**Codification:** Orchestrator's adversary-dispatch generator and similar templates SHOULD consult `glob` patterns (e.g., `ADR-015-*.md`) and substitute the resolved filename into the prompt. Same pattern for BC-N.NN.NNN files, story files, etc.

**Severity:** LOW (cosmetic; does not block content findings — adversary still found the file because the absolute path content was correct enough to navigate by)

**S-7.02 threshold:** First observation; codified provisionally without recurrence-threshold escalation. Re-evaluate if recurs.

**Date:** 2026-05-05
**Burst:** D-282 (E-9 v1.35 → v1.36)

---

## TD-VSDD-087 — Markdown table column-count validation in BC editing

**Source:** Pass-39 HIGH-P39-002 [process-gap]

**Class:** When the state-manager (or any agent) edits a BC's structured markdown tables (§Edge Cases, §Canonical Test Vectors, §Architecture Anchors, etc.), the edit harness MUST validate that each row has exactly the column count declared in the table header. The v1.35 D-281 burst introduced 6 EC rows in 2 BCs uniformly with 4 cells against a 3-column header — silent rendering defect across the entire burst.

**Codification:** Pre-commit hook `validate-bc-table-arity.sh` SHOULD parse all markdown tables in modified BC files, count cells per row vs declared column count from header separator (`|---|---|...`), and reject the commit if any row's cell count ≠ header column count.

**Severity:** HIGH when violated (silent rendering defect; downstream consumers — readers, linters, schema validators — see misaligned data without warning)

**S-7.02 threshold:** First observation; preemptively codified. Combined with TD-VSDD-080's `validate-bc-terminology-family.sh` mandate and TD-VSDD-085's TV-witness-presence mandate, the next un-mechanized BC defect class is rendering-arity. Hook landing target: before next BC-editing burst.

**Date:** 2026-05-05
**Burst:** D-282 (E-9 v1.35 → v1.36)

---

## TD-VSDD-088 — Orchestrator must route fix bursts to authoring agents (PO/architect), NOT use state-manager as fingers-on-keyboard

**Source:** User feedback during D-283 burst, post-pass-40 HIGH-P40-001 (4th-generation TD-VSDD-081 violation)

**Class:** When the orchestrator dispatches a fix burst that includes substantive BC content edits (new ECs, new TVs, new Postconditions, mechanism-description rewrites), the burst MUST route the BC content authorship to product-owner (or architect for ADR-class architectural reframes) — NOT to state-manager. State-manager's role is restricted to STATE.md / STORY-INDEX / lessons.md updates and the single seal commit (per POLICY 3 state-manager-runs-last + TD-VSDD-053 single-commit-burst).

**Symptom that motivated codification:** Across 22 fix bursts (D-261..D-282), the orchestrator drifted into a pattern of pre-designing BC content fixes in dispatch prompts and using state-manager to mechanically apply them. This produced two layers of role-drift: (1) orchestrator doing PO/architect authoring work; (2) state-manager executing pre-designed content without exercising fresh-context spec-author judgment. The drift correlates with 9+ burst-induced self-violations including HIGH-P40-001 — the BC pair claims `log.write` returns Err that's "silently discarded" at host/mod.rs:111 across 3 sites (P6 + EC-010 + TV row 11), but `internal_log.rs:228` shows `pub fn write` returns `()` not Result, and the IO-failure path eprintln!s to stderr. This mis-description was introduced in D-281 to close pass-38 MED-P38-002, then re-confirmed in D-282 with TV row 11 — both bursts cited line 111 without verifying the function signature because state-manager mechanically followed the orchestrator's pre-designed prose.

**Codification (NORMATIVE):**

The orchestrator MUST observe these routing rules for fix bursts:

1. **BC content edits** (Postconditions, Edge Cases, Test Vectors, mechanism descriptions, traceability rows, anchors): dispatch `product-owner` for authorship.
2. **Architectural reframes** (ADR-class changes, capability cluster restructuring): dispatch `architect` for authorship.
3. **STATE.md / STORY-INDEX / lessons.md / epic changelog / pass-N review file persistence / git commit**: dispatch `state-manager` LAST in the burst.
4. **Single-commit-burst preserved** (TD-VSDD-053): authoring agent stages files via scoped `git add` but does NOT commit; state-manager picks up staged work, adds its own staged updates, and makes the single seal commit.
5. **Orchestrator dispatch prompts MUST NOT contain pre-designed BC prose for authoring agent to mechanically apply.** The dispatch prompt MAY summarize findings, point to source-of-truth files, and state recommended directions — but the authoring agent MUST exercise spec-author judgment when authoring text. Pre-designing prose in dispatch defeats the purpose of fresh-context routing.

**Severity when violated:** HIGH (orchestrator structural error; correlates with content defect recurrence per the 22-burst pattern)

**Mechanization candidate:** A pre-dispatch hook should detect when the orchestrator's task description for state-manager includes substantive BC content edits (Postcondition rewrite, EC addition, TV addition, mechanism description) and reject the dispatch with a routing-violation error. The hook would parse the task description for keywords like "Postcondition", "Edge Case", "Test Vector", "rewrite ... to describe" and require the dispatch agent be product-owner or architect, not state-manager. **Filed as backlog ticket TD-VSDD-088-HOOK in `cycles/v1.0-brownfield-backfill/open-backlog-post-rc8.md`.**

**S-7.02 threshold:** This is a meta-pattern (governing the orchestrator itself, not artifact content). Recurrence is the 22-burst pattern in aggregate. Codified preemptively as NORMATIVE because (a) the user has explicitly requested stronger routing enforcement; (b) the pattern's correlate (content-defect recurrence including HIGH-P40-001) exceeds 3+ instances; (c) the mechanization candidate is straightforward to implement.

**Date:** 2026-05-05
**Burst:** D-283 (E-9 v1.36 → v1.37; FIRST application of corrected routing pattern)

---

## TD-VSDD-089 — PO authoring bursts MUST run explicit 4-axis sibling sweep before commit

**Source:** Pass-42 (3 MED + 1 SWEEP LOW = 4 sibling-sweep failures from v1.37 PO authoring burst). Sibling failures: EC-004 not refreshed when P3 introduced new mechanism; BC-036 P5 "4 denial paths" not reconciled with EC-003 "5 paths"; BC-035 line 65 inbound cross-ref pointed at wrong section; BC-036 line 66 self-referenced wrong section for same content. All within v1.37 D-283 burst.

**Class:** When a PO authoring burst lands content edits to specific sections (Postconditions, ECs, TVs, Related BCs), the burst MUST run an EXPLICIT 4-axis sibling sweep before commit:

1. **Postcondition ↔ Edge Case symmetry:** for each Postcondition that asserts a normative mechanism, grep ALL ECs that touch that mechanism. Verify each EC describes the mechanism consistently with the Postcondition.
2. **Cross-BC reference target accuracy:** for every cross-reference pointing at a sibling BC, verify the cited section actually contains the claimed content.
3. **Numeric enumeration consistency:** for every narrative count (e.g., "4 denial paths", "8 fields"), grep the same BC for sibling enumerations and verify they match.
4. **Parenthetical-list consistency:** for every parenthetical list of error classes, grep adjacent sections to verify they reference compatible enumerations.
5. **Codification artifact sibling integrity:** when adding a new TD-VSDD-NNN entry to lessons.md, verify the new entry's `**Date:**` and `**Burst:**` trailer lines do not bleed into adjacent TD entries; verify all sibling TD entries (TD-NNN-1, TD-NNN, TD-NNN+1) have consistent trailer formatting. This axis closes the meta-discipline gap surfaced by pass-43 MED-P43-002 (TD-VSDD-089's birth burst itself failed this axis).

**Codification (NORMATIVE):** PO dispatch prompts MUST include explicit instruction to perform this 4-axis sweep AND to report sweep coverage in output (even if no additional drift found). The sweep is MANDATORY, not advisory. State-manager Phase 2 verifies the PO output contains a sibling-sweep report; if missing, state-manager rejects the seal.

**Mechanization candidate (filed as TD-VSDD-089-HOOK in backlog):** Pre-commit hook running automated 4-axis sweep; this is approximate (mechanism-symmetry sweep is hard to fully automate) but can catch the easy cases (numeric enumeration drift; cross-BC section pointer drift via grep).

**S-7.02 threshold:** First codification (1 instance, 4 sub-instances within one burst). Codified preemptively because the burst that motivated it was the FIRST PO-authored burst (D-283); risk of recurrence is high until mechanically enforced.

**Date:** 2026-05-05
**Burst:** D-285 (E-9 v1.38 → v1.39; first application of sibling-sweep mandate)

---

## TD-VSDD-pattern-tracking — Codification-burst-self-violation

**Pattern observed:** Codification bursts that introduce a normative rule have, in 6 of N=6 sampled cases, partially violated that very rule within the same burst.

**Instances:**
1. Pass-39 (D-282) — TD-VSDD-085 NORMATIVE codified (TV-witness mechanization extension) but the same burst produced 3 self-violations (3 new ECs without TV witnesses); closed by TD-VSDD-085-applying burst (D-282 sibling).
2. Pass-43 (D-285) — TD-VSDD-089 NORMATIVE codified (PO sibling-sweep mandate) but produced 1 self-violation in BC content (MED-P43-001 BC-035 line 50 ordering missed by sibling sweep) AND 1 self-violation in codification artifact (MED-P43-002 lessons.md trailer drift in TD-VSDD-089's own body).
3. Pass-44 (D-286) — TD-VSDD-089 axis-5 self-application: pattern-tracking section trailer was single-line `**Date tracking opened:** 2026-05-05 (D-286 / pass-43)` instead of canonical two-line `**Date:**/**Burst:**` form used by sibling TD entries; closed at D-287.
4. Pass-45 (D-287) — TD-VSDD-090 self-violation (state-manager Phase 2 self-application audit narrated PASS but missed v1.41 H3 detail block missing AND TD-VSDD-090-HOOK Implementation surface missing; closed at D-288).
5. Pass-46 (D-288) — TD-VSDD-090 self-violation (state-manager Phase 2 grep-evidence audit cited fabricated grep `grep "**Section:**"` returning 0 matches; HOOK ticket section asymmetry 10/9/9 missed; closed at D-289 with paranoid-verification discipline + actual grep outputs).
6. Pass-47 (D-289) — TD-VSDD-090 paranoid-verification self-violation (line citations all 4 wrong: cited 1960/1989/2024/2063 vs actual 1961/1990/2025/2061; corrigendum's "correct" line numbers themselves off-by-one). Closed at D-290 with STRUCTURAL FIX (TD-VSDD-091 codified: switch from line numbers to stable anchors for self-referential intra-file citations).

**S-7.02 threshold:** Met at N=6. **Empirical confirmation:** Three different narrative disciplines tried (audit-by-claim D-287, grep-evidence D-288, paranoid-verification D-289). All three failed identically (HIGH/MED in each subsequent adversary pass). Pass-47 root-cause analysis: the failure is STRUCTURAL — line numbers shift on insertion. **Resolved at D-290 by codifying TD-VSDD-091 (stable-anchor citations).** Mechanization (TD-091-HOOK) remains for definitive enforcement.

**Codified at TD-VSDD-090** — see below.
**Structural fix codified at TD-VSDD-091** — see below.

**Date:** 2026-05-05
**Burst:** D-286 (pass-43 / E-9 v1.39 → v1.40)

---

## TD-VSDD-pattern-tracking — Fabricated-source-code-constant-in-H3-closure-narrative

### Pattern: fabricated-source-code-constant-in-H3-closure-narrative

**N=3 occurrences — S-7.02 3-occurrence threshold REACHED at D-303; TD-VSDD-093 codified.**

- **1st (H-P21-001 D-264 v1.21):** Closure narrative invented `TIMEOUT (-7)` and `OUTPUT_TOO_LARGE (-8)`; actual values are `-2` and `-3`. The D-264 fix burst corrected the BC body but the H3 closure narrative cited the wrong codes. Detected at pass-21 (BC-only deep-dive angle).
- **2nd (HIGH-P54-001 D-295 v1.46):** Closure narrative wrote `INVALID_ARGUMENT (-2)`; actual value is `-4` per `crates/factory-dispatcher/src/host/mod.rs:183`. BC-1.05.035 body was correct throughout; only the D-295 H3 closure narrative was wrong. The pass-51 original finding text was correct; the defect was introduced in the closure prose. Detected at pass-54 (external-reference link integrity audit angle — novel, untouched in 53 prior passes).
- **3rd (MED-P58-001 D-295 v1.46 LOW-P51-002 closure narrative `file_name` two-referents):** Closure narrative used `file_name` with two contradictory referents: (a) `Path::file_name()` API method return value `None` (correct — matches EC-013 `binary_allowed` helper at exec_subprocess.rs:186-192) AND (b) plugin-call-site-supplied event payload field absent from payload (fabricated — no event emission in EC-013, no payload field semantic exists). BC-035 EC-013 body was correct throughout; only the D-295 H3 closure narrative introduced the fabricated semantic frame. Detected at pass-58 (glossary/terminology consistency sweep angle — novel, untouched in 57 prior passes).
- **Hook-extension proposal (Obs-P54-001 + TD-VSDD-093 codification):** Extend TD-VSDD-080 hook (`validate-bc-terminology-family.sh`) to scan changelog H3 blocks for source-code-constant patterns (`INVALID_ARGUMENT (-?\d+)`, `TIMEOUT (-?\d+)`, `OUTPUT_TOO_LARGE (-?\d+)`, `INTERNAL_ERROR (-?\d+)`, `CAPABILITY_DENIED (-?\d+)`) and cross-validate against `host/mod.rs:179-184` constant definitions. Filed in open-backlog-post-rc8.md under TD-VSDD-093-HOOK section for E-3 (high-value hooks port) implementation.
- **Codification trigger:** REACHED at N=3 → TD-VSDD-093 codified at D-303 + TD-VSDD-080 hook extension filed for E-3 implementation.

**Date:** 2026-05-06
**Burst:** D-299 (pattern entry created for N=2 occurrence); D-303 (3rd occurrence MED-P58-001 detected at pass-58; N=3 threshold met; TD-VSDD-093 codified)

---

## TD-VSDD-090 — Normative-rule birth bursts MUST be self-application audited before seal

**Source:** TD-VSDD-pattern-tracking section escalated to codification at S-7.02 threshold (3 observed instances of "codification burst violates own rule": pass-39 D-282 TD-085 self-violation; pass-43 D-285 TD-089 self-violation; pass-44 D-286 TD-089-axis-5 self-violation).

**Class:** When a fix burst codifies a new normative rule (TD-VSDD-NNN at NORMATIVE severity), the SAME burst is structurally at high risk of partially violating the rule it just introduced. The pattern recurs because the rule's full implications aren't yet internalized at codification time, AND because the codification text itself is a sibling artifact in lessons.md subject to the very disciplines being codified.

**Codification (NORMATIVE):**

Before any normative-rule codification burst can be sealed:

1. **Self-application audit pass** — the codifying agent (state-manager or PO) MUST review the burst's own work product against the rule being codified. The audit must enumerate:
   - Each artifact modified in the burst (BC files, lessons.md, epic, STATE.md, etc.)
   - For each artifact, the rule's applicable axes/scope
   - Whether the artifact complies with the rule
   - Any non-compliance must be fixed in the SAME burst before seal.

2. **Codification text self-application** — the lessons.md TD-VSDD-NNN entry being added MUST itself satisfy the rule's discipline (e.g., a sibling-sweep rule's own codification text must pass sibling-sweep against neighboring TD entries; a TV-witness rule's codification text must include witness examples; etc.).

3. **State-manager seal-gate** — state-manager Phase 2 MUST refuse to seal if Phase 1 PO output (or state-manager's own meta-content) contains an unaddressed self-application violation against the new rule. The seal-gate MAY accept fixes deferred to a follow-up burst ONLY if explicitly documented as such in the seal commit message.

**Severity when violated:** HIGH (codification produces structural drift that propagates to all downstream applications of the rule)

**Mechanization candidate (filed as TD-VSDD-090-HOOK):** Pre-commit hook that detects burst commits adding new TD-VSDD-NNN entries to lessons.md. For each detected entry, the hook prompts an automated self-application checklist or rejects the commit if the entry's discipline can be machine-verified against the burst's other modified files.

**S-7.02 threshold:** Met (3 instances). NORMATIVE.

**Date:** 2026-05-05
**Burst:** D-287 (E-9 v1.40 → v1.41; FIRST application of self-application audit gate)

---

## TD-VSDD-091 — Self-referential intra-file citations MUST use stable anchors, NOT line numbers

**Source:** Pass-47 root-cause analysis (6th instance of codification-burst-self-violation pattern). Three prior fix bursts (D-287/288/289) tried three different narrative disciplines (audit-by-claim, grep-evidence, paranoid-verification); all three produced the same defect class because the root cause is STRUCTURAL not disciplinary.

**Class:** When an H3 block (or any inserted text) contains line-number citations to its own host file, the act of inserting the H3 SHIFTS all subsequent line numbers. Citations are correct AT AUTHOR TIME but wrong AT COMMIT TIME. No amount of paranoid-verification or grep-evidence narrative discipline can fix this — the line numbers literally change when the insertion happens.

**Codification (NORMATIVE):**

For all self-referential intra-file citations (citations FROM a section TO another section in the SAME file):

1. **Use stable anchors:** section headings (`### v1.40` block), frontmatter field names (`frontmatter version field`), table row identifiers (`summary table v1.40 row`), or section names (`Postcondition 3`).
2. **Do NOT use line numbers** for self-referential citations. Line numbers are only stable for cross-file or read-only-source-code citations where the citing burst doesn't modify the cited file.
3. **Cross-file line-number citations remain OK** (e.g., BC-035 citing `host/mod.rs:152` — that file isn't being modified by the citing burst).
4. **Read-only-source citations remain OK** (e.g., adversary citing a specific line in the epic from within a review file — the epic is not being modified by the review file, so line numbers are stable at review-file-write time).
5. **Hybrid case:** if a burst MUST cite a line number for its own added content (e.g., for a tool that requires line numbers), the burst MUST run grep AFTER staging and BEFORE commit, then UPDATE the citation to reflect post-stage line numbers (this requires a second-pass edit which is brittle; prefer stable anchors).

**Severity when violated:** HIGH (intra-file self-citation drift is unavoidable without stable anchors; pattern recurs at 100% rate per pass-39/43/44/45/46/47 evidence).

**Mechanization candidate (filed as TD-VSDD-091-HOOK):** Pre-commit hook that grep-detects self-referential line-number citations in H3 blocks (e.g., `line \d+` references to a file the same commit modifies) and rejects with error suggesting stable anchor replacement.

**S-7.02 threshold:** N=6 instances pattern (pass-39/43/44/45/46/47). Codified preemptively as NORMATIVE because all three prior narrative-discipline framings failed at first application. Mechanization remains overdue.

**Date:** 2026-05-05
**Burst:** D-290 (E-9 v1.43 → v1.44; FIRST application of stable-anchor citation discipline)

---

## TD-VSDD-092 — BC-SOUL4-coverage: BCs governing functions with silent-discard patterns MUST acknowledge them in EC rows

**Source:** Pass-50 SOUL #4 systemic sweep (HIGH-P50-001 + HIGH-P50-002 + MED-P50-001 + LOW-P50-001). Four `let _ =` and `map_err(|_|)` discards in `execute_bounded` had been unacknowledged across 49 prior adversary passes.

**Class:** When a BC governs a function that contains silent-discard patterns (`let _ = expr` where expr returns Result; `.map_err(|_| ...)`; `unwrap_or` on side-effecting Results; etc.), the BC MUST acknowledge each silent-discard in either an Edge Case row or an explicit out-of-scope declaration. Failure to do so creates SOUL #4 silent-failure paths where the spec appears formally complete but masks operational failure modes.

**Codification (NORMATIVE):**

For each BC, when the spec authoring burst lands:

1. **Source-walk discipline:** Read the source-of-truth function the BC governs. Grep for `let _ =`, `map_err(\|_\|`, `unwrap_or(`, `unwrap_or_else(\|_\|`, `\.ok();`, `if let Err(_) =` patterns. Each match is a silent-discard candidate.

2. **EC coverage requirement:** For each silent-discard candidate, the BC MUST contain an Edge Case row acknowledging the discard, OR an explicit out-of-scope declaration in §Postconditions explaining why the discard is acceptable.

3. **TV witness:** Per TD-VSDD-085 NORMATIVE, each new EC introduced by this rule MUST have a Canonical Test Vector witness row.

4. **OQ tracking:** Significant silent-discard cases (security, observability, data-integrity implications) SHOULD file an OQ entry tracking v2-or-later remediation candidates.

**Severity when violated:** HIGH (silent-failure paths in source code are exactly what SOUL #4 warns against; the BC's claim of completeness is materially false if it doesn't acknowledge them)

**Mechanization candidate (filed as TD-VSDD-092-HOOK):** Pre-commit hook scanning source-of-truth files cited by BCs for silent-discard patterns; for each, verify a corresponding EC row exists in the BC.

**S-7.02 threshold:** N=4 instances within a single BC pair (read_to_end x2, kill/wait x2 — all in execute_bounded). Codified preemptively as NORMATIVE because the SOUL #4 principle is an established rule (SOUL.md), and the gap is structural (49 prior angles all missed it because none did exhaustive source-walk for silent-discard patterns).

**Date:** 2026-05-06
**Burst:** D-293 (E-9 v1.44 → v1.45; FIRST application of BC-SOUL4-coverage discipline)

---

## TD-VSDD-093 — Closure-narrative source-of-truth validation discipline (NORMATIVE)

**Date:** 2026-05-06
**Burst:** D-303 (E-9 v1.50 → v1.51; FIRST application of TD-VSDD-093 self-application discipline)
**Recurrence count at codification:** N=3 (S-7.02 threshold met)

### Pattern observed

3 occurrences of "fabricated/misdescribed content in v1.46 H3 closure narrative" pattern:

1. **H-P21-001 (D-264 v1.21):** Closure narrative invented `TIMEOUT (-7)` and `OUTPUT_TOO_LARGE (-8)`; actual values `-2` and `-3` per host/mod.rs.
2. **HIGH-P54-001 (D-295 v1.46 H3 LOW-P51-001 closure):** Cited `INVALID_ARGUMENT (-2)`; actual value `-4` per host/mod.rs:183. Closed at D-299 v1.49 corrigendum.
3. **MED-P58-001 (D-295 v1.46 H3 LOW-P51-002 closure):** Used `file_name` with two contradictory referents (Path API method correct + plugin-call-site event payload field fabricated). Closed at D-303 v1.51 corrigendum.

### Normative rule (mandatory)

When authoring a closure-narrative bullet (e.g., "X CLOSED — ..." in an H3 changelog block) that references BC body content, source-code constants, or normative claims, the author MUST:

1. **Quote-verify:** Read the actual BC body / source code / normative claim text being summarized; cite the exact terms used in the source-of-truth.
2. **Avoid invention:** Do NOT invent semantic frames (e.g., "event payload field", "plugin-call-site-supplied") that are not present in the source-of-truth.
3. **Avoid abbreviation:** Do NOT abbreviate or alias terms in ways that introduce ambiguity (e.g., using `file_name` for both Path API and a fabricated payload field).
4. **Anchor with quoted snippets:** Where space allows, include a short literal quote from the source-of-truth artifact to ground the claim.

### Enforcement

- Adversarial reviewers MUST grep H3 closure narratives for source-code constant references, function/method names, and BC field names; cross-validate against source-of-truth files.
- State-manager Phase 2 MUST verify closure-narrative content matches the cited source-of-truth before sealing.

### TD-VSDD-080 hook extension recommendation

Extend `validate-bc-terminology-family.sh` (TD-VSDD-080 hook) to scan changelog H3 blocks (`### v1\.\d+` headings + bullet bodies) for:
- Source-code constant patterns: `(INVALID_ARGUMENT|TIMEOUT|OUTPUT_TOO_LARGE|INTERNAL_ERROR|CAPABILITY_DENIED) \(-?\d+\)`
- Cross-validate values against `crates/factory-dispatcher/src/host/mod.rs:179-184` constant definitions.
- Function/method name references: `binary_allowed\(\)`, `emit_denial\(\)`, `execute_bounded\(\)`, `ctx\.emit_internal`, `Path::canonicalize\(\)`, `Path::file_name\(\)`, etc.
- Cross-validate function existence in source-of-truth files.

Filed in open-backlog-post-rc8.md under TD-VSDD-093-HOOK section as candidate for E-3 (high-value hooks port) implementation.

### TD-VSDD-090 self-application audit (MANDATORY for this burst)

This burst applies TD-VSDD-093 to its own v1.51 H3 corrigendum prose: every claim about BC-035 EC-013 content, source-of-truth Rust functions, or pass-58 finding text MUST be quote-verified against the cited artifact before seal.

Self-application verification log:
- BC-035 EC-013 content: quoted directly in v1.51 H3 corrigendum from line-anchored read of BC body.
- Source-of-truth functions cited (`binary_allowed`, `Path::file_name()`, `unwrap_or_else`): all verified to exist at exec_subprocess.rs:186-192.
- Pass-58 finding text quoted directly from MED-P58-001 evidence section.

PASS — TD-VSDD-093 self-application audit completed before seal.
