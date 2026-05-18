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

## TD-VSDD-pattern-tracking — BC-INDEX-not-synced-to-BC-H1-reframe

### Pattern: bc-index-not-synced-to-bc-h1-reframe

**N=2 occurrences** (below S-7.02 3-occurrence threshold; TD-VSDD-094 NOT yet codified; monitoring for future codification).

- **1st (HIGH-P59-001 D-279 v1.33 → detected at pass-59 D-304):** v1.33 D-279 BC-035 reframe (symlink_traversal_escape → TOCTOU prevention) propagated to BC-035 H1 + body but not to BC-INDEX line 122. BC-INDEX trailing fragment remained `symlink-based traversal rejected` while BC-035 H1 was updated to `TOCTOU prevention via canonicalize-then-allow-list-check ordering`. Detected approximately 24 burst-iterations after the originating burst (D-279 through D-304 pre-fix).
- **2nd (BC-3.03.001 BC-INDEX line 215 truncation — detected at D-304 proactive sweep):** BC-3.03.001 BC-INDEX cell was truncated at `\`interval_ms\` (default 5000` — missing `ms) — either trigger fires a flush` (second em-dash segment). BC-3.03.001 H1 has a two-part em-dash title. Truncation origin unknown; corrected D-304.
- **Detection angle:** Capability anchoring per POLICY 4/5 audit (Step 7 BC-INDEX cross-reference audit per POLICY 7). Proactive sweep triggered by Obs-P59-001 process-gap observation.
- **Hook-extension proposal (Obs-P59-001):** Extend post-edit grep verification discipline to mandate BC-INDEX scan when BC body H1 changes. Filed for orchestrator cycle-closing-checklist. Not yet codified as TD-VSDD-094 (N=2 below S-7.02 threshold).
- **Codification trigger:** NOT YET reached — N=2 below S-7.02 3-occurrence threshold. Tracking for future codification at N=3.

**Date:** 2026-05-06
**Burst:** D-304 (pattern entry created at N=2 via proactive BC-INDEX-vs-H1 sweep)

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

---

## TD-VSDD-pattern-tracking — ID-assignment-without-free-slot-verification

### Pattern: id-assignment-without-free-slot-verification (architect routing decisions)

**N=1 occurrence** (below S-7.02 3-occurrence threshold; full codification NOT yet triggered; monitoring for recurrence).

- **1st (D-312 / D-311 ID-collision slip):** D-311 architect routing burst assigned `BC-1.12.008 → ss-03/BC-3.05.001.md` as the target file path without first running `ls .factory/specs/behavioral-contracts/ss-03/BC-3.05.*` to verify the slot was free. BC-3.05.001/002/003 already existed, authored by codebase-analyzer on 2026-04-25 as brownfield BCs documenting the OLD pre-ADR-015 multi-sink behavior. Brownfield BC clusters from codebase-analyzer ingestion fill slots non-contiguously; greenfield routing decisions can collide with them. D-312 architect corrigendum: (1) new v2 schema BC ID reassigned to BC-3.05.004 (next free slot; SS-03-event-emission.md prescriptive cluster designation: "observability-config.toml v2 schema validation (BC-3.05.*)"). (2) Legacy BC-3.05.001/002/003 marked `lifecycle_status: retired` + `superseded_by: ADR-015` in frontmatter; bodies preserved verbatim per POLICY 1. Justified by ADR-015 Consequences section: "BC-3.* contracts covering multi-sink fan-out, DLQ, and sink health events are withdrawn or revised." (3) E-10 epic v1.3→v1.4 with corrigendum acknowledgment. (4) OQ-W16-012 filed-and-resolved.

**Symptoms:** Architect or BC-creator agent assigns a target file path / BC ID without first running `ls .factory/specs/behavioral-contracts/ss-XX/BC-S.NN.*` to verify the slot is free. Brownfield BC clusters from codebase-analyzer ingestion fill non-contiguously; greenfield routing decisions can collide with them.

**At N=3 codification trigger:** Codify TD-VSDD-NNN requiring architect-routing prompts and PO BC-creation prompts to include explicit free-slot verification step with quoted `ls` output as evidence before any new BC ID is assigned to a file path.

- **D-313 (2026-05-06):** PO Phase 1b honored the D-312 remediation procedure — quoted `ls` output as free-slot evidence for all 5 BC IDs (BC-1.12.005, BC-1.12.006, BC-1.12.007, BC-1.12.009, BC-3.05.004). No ID collision. Pattern occurrence count remains 1 of N=3. Practice is operational; codification trigger not yet hit.

**Date:** 2026-05-06
**Burst:** D-312 (architect corrigendum for D-311 BC-3.05.001 ID-collision slip; occurrence 1 of N=3 codification threshold)

---

## TD-VSDD-pattern-tracking — BC-authorship-without-same-burst-story-propagation (occurrence 1 of N=3)

### Pattern: bc-authorship-without-same-burst-story-propagation (PO BC-authoring bursts)

**N=1 occurrence** (below S-7.02 3-occurrence threshold; full codification NOT yet triggered; monitoring for recurrence).

- **D-310 (2026-05-06):** Phase 1a authored 4 BCs but did NOT propagate them into the relevant stories' `behavioral_contracts:` arrays the same burst — stories continued carrying `behavioral_contracts: []` placeholder. The deferral was structural (Phase 1a/Phase 1b split) but not codified or documented as a deferral.
- **D-313 (2026-05-06):** Phase 1c healed the deferral by propagating ALL 9 new BCs (Phase 1a + Phase 1b) plus BC-1.11.003 (architect-authored 2026-05-04, never propagated) into 5 stories under POLICY 8. 13 BC-story slot insertions; sync verified.

**Symptoms:** A BC-authoring burst completes (PO or architect) without updating the relevant stories' `behavioral_contracts:` frontmatter arrays and body BC tables in the same burst. Stories carry stale `behavioral_contracts: []` or incomplete arrays until a follow-up burst heals the gap.

**At N=3 codification trigger:** Codify a new TD-VSDD entry mandating same-burst BC→story propagation OR a documented deferral-justification entry for the multi-burst case.

**Date:** 2026-05-06
**Burst:** D-313 (Phase 1c story-writer propagation healing D-310 deferral; occurrence 1 of N=3 codification threshold)

---

## E-10 adversary pass-1 outcome + D-317 seal (2026-05-06)

### Pass-1 outcome and fix-burst summary

**Adversary verdict:** CRITICAL — 22 findings across the E-10 spec package (ADR-015 + E-10 epic v1.4 + 10 BCs + BC-1.11.003 + 5 stories). ADR-013 pass-counter RESET to 0. See cycles/v1.0-brownfield-backfill/E-10-pass-1.md for full report.

**Four-burst fix cycle (D-314 → D-317):**
- D-314 (architect, 69408f6): capabilities.md CAP-029+CAP-030 authored, CAP-003 REWRITTEN per ADR-015 D-15.1, CAP-023+CAP-024 SUPERSEDED. Invariants DI-007/008/011/012/013/014/017 amended. E-10 epic v1.4→v1.5 (re-anchored to CAP-029/030). BC-1.11.003 v1.0→v1.1 (CAP-009 anchor + EC-004 emit_pair rewrite per F-6 + F-20).
- D-315 (PO, 5803d28): 8 BC bodies rewritten — BC-1.12.001/002/003/004/005/007/009 + BC-3.05.004, all v1.0→v1.1. H1 text changed for BC-1.12.002 (two-key debug-stream gate wording precision, F-13) and BC-1.12.009 (five-state event taxonomy enumeration, F-12).
- D-316 (story-writer, 07f946c): 5 stories propagated per POLICY 8. S-10.04 gained BC-1.12.003/004/005 (F-7+F-8, bcs count 1→4). S-10.05 gained SS-02 subsystem (F-5). S-10.02/03/09 version bumps.
- D-317 (state-manager seal, this burst): BC-INDEX v1.7→v1.8 (9 BC capability + H1 updates); ARCH-INDEX v1.0→v1.1 (F-19 renumbering footnote); STORY-INDEX v2.17→v2.18 (5 story row updates); STATE.md current_step; lessons.md this entry.

**Cycle still OPEN:** 3-of-3 NITPICK_ONLY not yet reached. D-317 is the burst seal, not the cycle close. Cycle-closing checklist (S-7.02) re-applies after CONVERGENCE_REACHED.

### Architect routing override for F-6 + F-20 (TD-VSDD-088 author-ownership rule)

F-6 (BC-1.11.003 CAP-TBD — capability anchor missing) and F-20 (BC-1.11.003 EC-004 rewrite — emit_pair Wave-3 removal) were listed in the adversary's pass-1 summary routing table under "PO" as the suggested owner. The orchestrator overrode to "architect" for both.

**Rationale:** BC-1.11.003 was architect-authored on 2026-05-04 (D-314 commit history). TD-VSDD-088 NORMATIVE rule establishes that the original author-agent is the correct owner for amendments to their own BCs (author-ownership principle). PO Phase 1 BC rewrites are reserved for BCs that PO authored or for cross-BC capability sweep work; BC-1.11.003 is an architect-domain BC covering the emit_pair host helper. The EC-004 rewrite (F-20) also required architectural judgment (deciding what the Wave-3 replacement contract should be, not just adjusting test vectors).

**Lesson for future adversary routing tables:** When the adversary's summary table assigns "PO" to a finding on an architect-authored BC, the orchestrator should cross-check the BC's `producer:` frontmatter field before dispatching. If `producer: architect`, override to architect unless the finding is purely a capability-column or story-propagation mechanical update. Document the override reasoning in the burst commit message and lessons.md.

### Process-gap pattern tracking (pass-1 findings)

**F-3 — line-N citations across BCs (occurrence 5+ / N=3 threshold already passed):**
Pass-1 flagged 5 BCs with unstable line-number citations (pattern: quoting "line 42" rather than a quoted prose anchor). This is the 5th+ occurrence; Task #77 already tracks the engine-level codification of the line-citation ban (TD-VSDD-091-ENGINE backlog ticket filed D-291). Pass-1 adds another data point to the existing TD ticket. No new codification action required here — the engineering fix already has a home.

**F-6 — CAP-TBD persistence (occurrence 1 / N=3 trigger threshold: not yet reached):**
BC-1.11.003 carried `capability: CAP-TBD` through two sealed bursts (architect-authored D-314 was the first time it was corrected to CAP-009). Pattern occurrence count = 1 as of pass-1. S-7.02 codification threshold is N=3. No new follow-up story or TD-VSDD entry created yet. If this recurs in pass-1' or a subsequent cycle on a different BC, escalate to story creation under self-improvement epics.

**F-21 — mechanical CAP justifications across 4–5 BCs (occurrence 1 / N=3 trigger threshold: not yet reached):**
Pass-1 found 4–5 BCs with capability anchors justified by boilerplate phrases rather than substantive rationale linking the BC's observable behavior to the capability's definition. Pattern occurrence count = 1 as of pass-1. S-7.02 codification threshold is N=3. No new follow-up story or TD-VSDD entry created yet. If this recurs in pass-1' or a subsequent cycle, escalate to story creation.

**Date:** 2026-05-06
**Burst:** D-317 (state-manager seal — E-10 pass-1 fix burst D-314/D-315/D-316/D-317 SEALED; cycle still OPEN pending 3-of-3 NITPICK_ONLY)

---

## E-10 adversary pass-2 outcome + D-321 seal (2026-05-06)

### Pass-2 outcome and fix-burst summary

**Adversary verdict:** CRITICAL — 11 findings across the E-10 spec package. ADR-013 pass-counter still 0. See cycles/v1.0-brownfield-backfill/E-10-pass-2.md (archived SHA 4720490) for full report.

**Four-burst fix cycle (D-318 → D-321):**
- D-318 (architect, 85507f5): capabilities.md CAP-030 errata (abstract-reference fix; v1.2→v1.3). BC-1.11.001 CAP-TBD→CAP-029 (v1.0→v1.1). BC-1.12.003 Capability Anchor Justification text added (v1.2→v1.3). BC-1.12.006 CAP-008→CAP-029 primary + CAP-008 secondary (v1.0→v1.1). BC-1.12.007 Invariant 2 rewrite + Open Questions section (v1.1→v1.2). 2 NEW BCs authored: BC-2.06.001 (SS-02, CAP-009, SDK semver bump for D-15.3 host-field-precedence) and BC-4.09.001 (SS-04, CAP-009 primary + CAP-029 secondary, plugin event-name migration with dual-emit).
- D-319 (PO, 8cfffec): BC-1.12.003 Story Anchor + Stories cell updated (+=S-10.04; v1.2→v1.3). BC-1.12.004 v1.1→v1.2 (same). BC-1.12.005 v1.1→v1.2 (same + EC-008 rewrite). BC-1.12.009 v1.1→v1.2 (five-state body prose sweep; H1 unchanged). BC-3.05.004 v1.1→v1.2 (CTV strictness improvements).
- D-320 (story-writer, 2e1e190): S-10.04 v1.2→v1.3 (BC-1.11.001 added to behavioral_contracts; bcs count 4→5). S-10.05 v1.2→v1.3 (BC-2.06.001 + BC-4.09.001 added; bcs count 2→4). S-10.02/03/09 unchanged.
- D-321 (state-manager seal, this burst): BC-INDEX v1.8→v1.9 (8 BCs amended + 2 NEW rows; total_bcs 1929→1931); ARCH-INDEX v1.1→v1.2 (SS-02 25→26; SS-04 30→31); STORY-INDEX v2.18→v2.19; STATE.md current_step; lessons.md this entry.

**Cycle still OPEN:** 3-of-3 NITPICK_ONLY not yet reached. D-321 is the burst seal, not the cycle close.

### Regression-rich pass: pass-2 findings were partially self-introduced by the pass-1 fix burst

Pass-2 was a regression-rich pass — several of its 11 findings were defects INTRODUCED by the pass-1 fix burst (D-314/D-315/D-316/D-317), not pre-existing gaps:

- **F-1 (CAP-030 wrong enumeration):** D-314 authored CAP-030 with a 15-field enumeration that diverged from ADR-015's abstract specification at D-15.2. D-318 closed via abstract-reference + errata note.
- **F-2 (incomplete five-state sweep):** D-315 updated BC-1.12.009 H1 to five-state taxonomy but body prose retained "four-state" references in multiple places. D-319 closed via body sweep. Closed as H1-update-without-body-sweep pattern occurrence (see below).
- **F-8 (EC-008 prose self-contradicting):** D-315's BC-1.12.005 v1.0→v1.1 rewrite introduced a self-contradiction in EC-008. D-319 closed via EC-008 rewrite.
- **F-11 (TD-015-a left as open question):** D-318's BC-1.12.007 Invariant 2 rewrite left the TD-015-a deferred CI check framing as an open question instead of asserting the invariant as normative deferred. D-318 also closed this in the same burst per architectural judgment.

**Process discipline note:** Every fix-burst commit should include an internal-consistency check before sealing. The state-manager seal (POLICY 3 last agent) is a bookkeeping role, not a content-review role; the regression detection depends on the next adversary pass. Consider adding a pre-seal checklist step for the agent that authored BC body changes: "after writing, re-read each changed section for internal self-consistency before commit."

### Process-gap pattern tracking (pass-2 findings)

**POLICY 8 reverse-direction drift (occurrence 1 of N=3 trigger):**
Pattern: frontmatter of a story gains new BC slots in a fix burst, but the corresponding BCs' Story Anchor / Stories fields are NOT back-propagated to acknowledge the new story mapping. D-316 expanded S-10.04's behavioral_contracts array (F-7+F-8: added BC-1.12.003/004/005) but did not back-propagate to those BCs' Story Anchor and Stories cells. D-319 closed the gap by updating BC-1.12.003/004/005 Story Anchor + Stories cells. Occurrence count: 1 of N=3 codification threshold. Adding a state-manager validation step to scan bidirectional asymmetry is a codification candidate at N=3.

**Date:** 2026-05-06
**Burst:** D-321 (state-manager seal — pattern occurrence 1 of N=3 codification threshold)

---

**CAP-TBD survival across capability-authoring bursts (occurrence 2 of N=3 trigger):**
Pattern: a BC carries `capability: CAP-TBD` through multiple sealed bursts including a capability-authoring burst that should have resolved it. Occurrence 1: D-310/D-313 Phase 1a/Phase 1b BC authorship did not resolve BC-1.11.001 CAP-TBD. Occurrence 2: D-314 was a capability-authoring burst (authored CAP-029 and CAP-030) but missed BC-1.11.001 — still carried CAP-TBD into the sealed D-317 package. D-318 closed BC-1.11.001 → CAP-029. Occurrence count: 2 of N=3 codification threshold. Codification candidate at N=3: capability-author skill checklist step "scan workspace for `capability: CAP-TBD` and adjudicate each against newly-authored CAPs before sealing."

**Date:** 2026-05-06
**Burst:** D-321 (state-manager seal — pattern occurrence 2 of N=3 codification threshold)

---

**CAP enumeration vs source ADR enumeration (occurrence 1 of N=3 trigger):**
Pattern: a capability file (capabilities.md) enumerates fields/attributes for a capability that diverges from the authoritative source document's enumeration. Occurrence 1: D-314 authored CAP-030 with a 15-field enumeration that didn't match ADR-015 D-15.2's abstract-reference clause. D-318 closed via abstract-reference approach + errata note in CAP-030 body. Occurrence count: 1 of N=3 codification threshold. Codification candidate at N=3: capability-author skill validation step "when authoring a CAP that cites a specific ADR clause, verify the enumeration in the CAP body matches the ADR clause's enumeration verbatim, or use abstract reference + errata pattern."

**Date:** 2026-05-06
**Burst:** D-321 (state-manager seal — pattern occurrence 1 of N=3 codification threshold)

---

**H1 update without body-sweep (occurrence 1 of N=3 trigger):**
Pattern: a fix burst updates a BC's H1 heading text to new terminology, but body sections retain the old terminology from the previous H1. Occurrence 1: D-315 updated BC-1.12.009 H1 to "five-state" taxonomy but body prose retained "four-state" in 3+ places. D-319 closed via body sweep. Occurrence count: 1 of N=3 codification threshold. Codification candidate at N=3: a validation step "after H1 changes, scan body for terms named in old H1 vs new H1; warn on residual old-term usage."

**Date:** 2026-05-06
**Burst:** D-321 (state-manager seal — pattern occurrence 1 of N=3 codification threshold)

---

**No new follow-up stories created:** None of the 4 process-gap pattern entries above have hit the N=3 codification trigger threshold. Pattern-track only; record so future occurrences increment toward codification. No self-improvement epic stories created this burst.

**Date:** 2026-05-06
**Burst:** D-321 (state-manager seal — E-10 pass-2 fix burst D-318/D-319/D-320/D-321 SEALED; cycle still OPEN pending 3-of-3 NITPICK_ONLY)

---

## E-10 Pass-3 Fix Burst — D-322/D-323/D-324 SEALED (2026-05-06)

### Pass-3 outcome

Pass-3 verdict: **HIGH** (improvement from CRITICAL in passes 1+2). 16 findings total. ADR-013 pass counter still 0 (HIGH is not NITPICK_ONLY). Fix burst dispatched as D-322/D-323/D-324. Pass-3 report: `cycles/v1.0-brownfield-backfill/E-10-pass-3.md` (archived at SHA 8aed9cc).

### 3-burst fix cycle pattern

The D-322/D-323/D-324 fix cycle reduced from the 4-burst D-318→D-321 pattern by combining the architect's F-8 finding (BC-1.11.003 subsystem justification) into PO scope (D-322). This worked because the adjudication was clear: keep BC-1.11.003 in SS-01 with a clarified justification, not a structural move. No separate architect burst was needed.

**Burst sequence:** D-322 (PO, with F-8 architect routing folded in; SHA 42555e5) → D-323 (story-writer; SHA 42adb27) → D-324 (state-manager seal; this burst).

### Process-gap pattern tracking (pass-3 findings)

**POLICY 8 reverse-direction drift (occurrence 2 of N=3 trigger):**
Pattern: story or BC index cells become stale after fix bursts that touch story-anchor or capability fields without back-propagating to the BC-INDEX. D-316 first instance (prior cycle). D-322 second instance: BC-INDEX line 166 BC-1.11.001 Stories cell S-10.03 stale (should be S-10.04 per BC body Story Anchor); BC-1.11.003 Stories cell "Wave 2 TBD" stale (should be S-10.05 per D-322 Story Anchor addition); BC-1.11.002 missed in CAP-anchor sweep (CAP-TBD survived into pass-3 sealed package). D-324 closes all three gaps. Occurrence count: 2 of N=3 codification threshold. Codification candidate accelerating; if next pass finds another instance, N=3 trigger is reached.

**Date:** 2026-05-06
**Burst:** D-324 (state-manager seal — POLICY 8 reverse-direction drift occurrence 2 of N=3)

---

**CAP-TBD survival across capability-authoring bursts (occurrence 3 of N=3 TRIGGER REACHED):**
Pattern: a BC carries `capability: CAP-TBD` through multiple sealed bursts including a capability-authoring burst that should have resolved it. Occurrence 1: D-310/D-313 Phase 1a/Phase 1b BC authorship did not resolve BC-1.11.001 CAP-TBD. Occurrence 2: D-314 authored CAP-029 and CAP-030 but missed BC-1.11.001 — still carried CAP-TBD into D-317 sealed package; D-318 closed. Occurrence 3: D-310/D-313 Phase 1a/Phase 1b, D-314 (CAP-029/030 burst), and D-318 all missed BC-1.11.002 — still carried CAP-TBD through the entire pass-2 fix cycle into the pass-3 sealed package; D-322 finally closed. **Occurrence count: 3 of N=3 — CODIFICATION TRIGGER REACHED.**

**CODIFICATION TRIGGER REACHED (N=3): CAP-TBD survival across capability-authoring bursts**

- **Pattern recurrence count:** 3
- **Instances:**
  1. BC-1.11.001 had `capability: CAP-TBD` from 2026-05-04 authoring through D-310/D-313/D-314/D-317; finally resolved in D-318.
  2. BC-1.12.008 was authored with CAP-TBD; renumbered to BC-3.05.004 in D-312 corrigendum (also resolved later in D-313).
  3. BC-1.11.002 had `capability: CAP-TBD` from initial authoring through D-310/D-313/D-314/D-317/D-318; finally resolved in D-322.
- **Required action per S-7.02:** File a follow-up story under self-improvement epic E-1 (or appropriate self-improvement epic) titled "capability-author skill checklist: CAP-TBD scan after CAP authoring". Acceptance criterion: every capability-authoring agent (architect) must, after authoring new CAP-IDs, scan the BC corpus for `capability: CAP-TBD` and re-attempt anchoring against the newly-authored CAPs OR explicitly defer with a recorded reason.
- **Status:** TODO — story not yet authored. Orchestrator to file as drift item or self-improvement story in next planning cycle.

**Date:** 2026-05-06
**Burst:** D-324 (state-manager seal — CAP-TBD survival occurrence 3 of N=3 CODIFICATION TRIGGER REACHED)

---

**CAP enumeration vs source ADR enumeration (occurrence 1 of N=3 trigger):**
No new occurrence this pass. Pattern count remains 1 of N=3. (D-314 CAP-030 vs ADR-015 D-15.2; D-318 closed via abstract-reference + errata.)

---

**H1 update without body-sweep (occurrence 2 of N=3 trigger):**
D-322 updated BC-1.12.007 H1 (TD-015-a PARTIAL CLOSURE reframe) and the change was propagated to story body BC tables in D-323 (S-10.02, S-10.09) and to BC-INDEX in D-324. The H1 update was properly propagated within the same fix cycle, but the propagation required two separate bursts (D-323 story-writer, D-324 state-manager) rather than a same-burst sync. This is occurrence 2 because the H1 update and its full propagation were not atomic — the BC-INDEX propagation was left to the state-manager seal. Occurrence count: 2 of N=3 codification threshold. Pattern: a fix burst updates a BC H1 but the BC-INDEX cell still shows the old title until the state-manager seal burst.

**Date:** 2026-05-06
**Burst:** D-324 (state-manager seal — H1 update without body-sweep occurrence 2 of N=3)

---

**NEW PATTERN: Partial-fix regression discipline (S-7.01) recurrence (occurrence 1 of N=3 trigger):**
Pass-2 found regressions introduced by the pass-1 fix burst (D-314→D-317): F-1 CAP-030 wrong enumeration, F-8 EC-008 incomplete prose, F-11 TD-015-a left as open question. Pass-3 found regressions introduced by the pass-2 fix burst (D-318→D-321): F-2 BC-INDEX line 166 stale, F-3 S-10.05 body four-state (BC-1.12.009 five-state sweep in D-319 not propagated to story body), F-4 BC-1.11.002 sibling missed in CAP-anchor sweep, F-7 changelog gap on 5 BCs. Pattern: each fix burst introduces new findings caught in the next adversary pass. Codification candidate: every fix-burst commit should include an internal-consistency sweep before sealing (scan for old terminology in updated sections; verify bidirectional BC-story linkage; verify BC-INDEX matches BC body H1 and capability fields). Occurrence count: 1 of N=3.

**Date:** 2026-05-06
**Burst:** D-324 (state-manager seal — partial-fix regression pattern NEW occurrence 1 of N=3)

---

**Secondary capability anchors (F-15 process-gap noted, not codified):**
F-15 [process-gap]: secondary capability anchors (CAP-029+CAP-008 in BC-1.12.006, CAP-029+CAP-010 in BC-3.05.004, CAP-009+CAP-029 in BC-4.09.001) live only in BC body Traceability tables, not in frontmatter or BC-INDEX. Pattern occurrence 1 of N=3. If recurrence in future passes, codify either schema extension (`secondary_capabilities:` array) or BC-INDEX rendering policy (e.g., "secondary" column or footnote notation). No action this burst; recording for future tracking.

**Date:** 2026-05-06

---

**Cycle still OPEN:** 3-of-3 NITPICK_ONLY not yet reached. D-324 is the BURST SEAL, not the cycle close.

**Date:** 2026-05-06
**Burst:** D-324 (state-manager seal — E-10 pass-3 fix burst D-322/D-323/D-324 SEALED; cycle still OPEN pending 3-of-3 NITPICK_ONLY)

---

## LESSON-D-327: Brownfield BC drift from engine release not caught by adversarial passes

**Date:** 2026-05-06
**Burst:** D-327 (state-manager seal — E-10 ↔ rc.12 format-alignment cycle SEALED)
**Category:** spec-vs-engine-drift / release-cycle-discipline
**Severity:** MEDIUM — 4 DRIFT_MINOR items; no CRITICAL or HIGH engine↔spec contradictions

### Pattern: Brownfield BC drift from engine release not caught by adversarial passes

Pass-1 through pass-4 of the E-10 adversarial review cycle did NOT catch BC-4.02.002 / BC-4.01.003's stale "first stderr line" postconditions. Root causes:

1. **Scope framing:** Those BCs are in SS-04 (brownfield-extracted from legacy-bash-adapter behavior), outside the E-10 core scope (ADR-015 D-15.x OTel migration). The adversary was focused on the E-10 BC cluster (BC-1.12.x, BC-1.11.x, BC-2.06.001 primary). SS-04 was outside the primary reading list for passes 1–4.

2. **Drift class mismatch:** Adversarial review checks intra-spec consistency (BC-vs-BC, BC-vs-story, BC-vs-ADR). It does NOT check spec-vs-engine alignment. BC-4.02.002 and BC-4.01.003 were internally consistent throughout — the drift was between the BC postcondition and the rc.12 source code change (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs`).

3. **Branch separation:** The engine change (rc.12 stderr-capture fix on `develop`) and the spec amendments (on `factory-artifacts`) live on different branches with no automatic cross-branch consistency gate.

### The right tool

The architect audit at 119e70e was the correct tool: a deliberate spec-vs-engine drift scan, triggered by a meaningful engine release (rc.12). The audit read the released source and compared it to the BC postconditions — exactly the class of check that adversarial review cannot do without explicit source-code grounding.

### Codification candidate (occurrence 1 of N=3 trigger)

Spec-vs-engine drift detection should be a release-cycle gate. Every release on develop should trigger an architect audit of the spec corpus on factory-artifacts to catch drift before the next adversary pass. This currently happens by orchestrator dispatch; it could be automated as a release-step hook (e.g., a CI job triggered on `v*-rc*` tag push that diffs released source against BC postconditions for active subsystem BCs).

**Pattern occurrence count:** 1 of N=3 trigger. If recurrence happens (next release introduces another spec-vs-engine drift not caught organically), escalate to codification.

### Cycle bookkeeping at D-327 seal

- E-10 spec corpus now aligned with rc.12 (4cf59bc). 4 BCs amended (BC-4.02.002 v1.1, BC-4.01.003 v1.1, BC-1.12.006 v1.3, BC-2.06.001 v1.3).
- Step (vi) E-10 adversarial-review cycle resumes from pass-4's verdict (HIGH, counter at 0). Pass-5 dispatches next.
- BC-INDEX v1.10→v1.11; ARCH-INDEX v1.3→v1.4; STORY-INDEX v2.20→v2.21.
- 4 follow-up tracking patterns from pass-3/D-324 still open: POLICY 8 reverse-direction drift (occurrence count per prior tracking), CAP enumeration mismatch, H1 update without body sweep (occurrence 2 of N=3), partial-fix regression (occurrence 1 of N=3).

---

## LESSON-2026-05-06-D331 [process-gap + pattern-tracking] E-10 pass-5 fix-cycle seal

**Burst:** D-331 (state-manager seal — E-10 pass-5 fix-cycle SEALED; 4 commits D-328→D-331)
**Category:** partial-fix-regression / cross-author-burst-completion / POLICY-8-reverse-direction
**Severity:** HIGH verdict for pass-5 (12 findings; trend 22→11→16→16→12 — improving)

### Pass-5 outcome summary

Adversary pass-5 returned HIGH (12 findings) on the post-D-327 (rc.12-aligned) E-10 spec package. Counter still 0 (3-of-3 NITPICK_ONLY not yet reached). The 4-burst fix cycle D-328→D-331 closed 8 of 12 findings. F-7 and F-8 were deferred to dedicated cleanup stories #115 and #116 per the adversary's own recommendation (large-blast rename across architecture + domain docs, and TD-VSDD-091 line-N citation sweep — both too large to bundle in this fix cycle without risk of introducing new regressions).

**Fix cycle routing:** D-328 architect (F-2, F-4, F-9, F-12 — 5 BCs amended) → D-329 PO (F-5 — BC-1.12.006 v1.4→v1.5 PC2 reason field) → D-330 story-writer (F-1, F-3, F-11 — 3 stories amended) → D-331 state-manager (index propagation + F-1/F-2 final propagation).

### Pattern-tracking updates (three simultaneous triggers)

#### Partial-fix regression discipline (S-7.01) — OCCURRENCE 3 of N=3 TRIGGER REACHED

**Previous occurrences:** occurrence 1 (D-304: BC-035 D-279 TOCTOU reframe propagated to BC H1 but NOT to BC-INDEX), occurrence 2 (D-326: BC-1.11.001 v1.2 D-325 changelog flagged a story-writer follow-up that D-326 architect-only burst missed).

**This occurrence (F-1, F-3):** BC-1.11.002 D-322 fix added Story Anchor S-10.02 to the BC body but never propagated the reverse direction to BC-INDEX line 171 (Stories cell stayed S-10.03) or to S-10.02 frontmatter `behavioral_contracts:` array. F-3: broader pattern of architect/PO/story-writer cross-author follow-ups dangling between bursts. The D-326 architect burst flagged `Story-writer D-NNN to ...` in its changelog but the dispatch was never explicitly verified.

**Codification triggered (N=3 threshold met):** Every architect/PO burst that flags `Story-writer D-XXX to ...` or `state-manager D-XXX to ...` in its changelog MUST file a corresponding follow-up task in the orchestrator's TaskList AND the orchestrator MUST verify the follow-up is dispatched before declaring the cycle closed. Per S-7.02 cycle-closing checklist requirement: file follow-up story under self-improvement epic.

#### Cross-author burst-completion gap [process-gap]

When an architect-authored amendment requires story-writer or state-manager follow-up to fully propagate, the routing dispatch must explicitly include all required agents in the same fix cycle. D-326 was architect-only despite BC-1.11.001 v1.2 flagging a story-writer follow-up. The fix cycle was not closed until D-330 authored the story propagation.

**Codification candidate (occurrence 1 of N=3 trigger):** Orchestrator dispatch checklist must include "scan recent BC changelogs for `Story-writer D-XXX` / `PO D-XXX` / `state-manager D-XXX` followup flags before closing the burst." Track recurrence.

#### POLICY 8 reverse-direction drift — OCCURRENCE 3 of N=3 TRIGGER REACHED

**Previous occurrences:** occurrence 1 (D-324: BC-1.11.001 D-320 added Story Anchor S-10.04 but BC-INDEX line 166 Stories cell was not propagated reverse-direction until D-324 seal), occurrence 2 (D-324: BC-1.11.003 Story Anchor Wave 2 TBD → S-10.05 fix was delayed by one burst).

**This occurrence (F-1):** D-322 added BC-1.11.002 Story Anchor S-10.02 in the BC body but never propagated to BC-INDEX line 171 (Stories cell = S-10.03 instead of S-10.02) or to S-10.02 frontmatter `behavioral_contracts:` array. The BC body pointed at S-10.02; BC-INDEX pointed at S-10.03; S-10.02 didn't list BC-1.11.002. All three needed synchronization.

**Codification triggered (N=3 threshold met):** State-manager validation step added: "after every BC Story Anchor change, scan BC-INDEX Stories cell AND the named story's frontmatter `behavioral_contracts:` array for matching propagation." This is a mandatory pre-commit check for every burst that modifies a BC's Story Anchor field.

### F-7 / F-8 deferral rationale

**F-7 (dispatcher_trace_id rename):** Approximately 7+ files across architecture docs and domain docs require renaming. The adversary itself flagged this as a candidate for a dedicated cleanup story rather than holding E-10 convergence. Story #115 filed.

**F-8 (TD-VSDD-091 line-N citation sweep):** Approximately 5+ files across stories and ADR-015 require a citation sweep. Same recommendation. Story #116 filed.

Both deferrals are explicitly blessed by the adversary's pass-5 report. The E-10 convergence cycle continues; the deferred items are not blocking.

### Cycle bookkeeping at D-331 seal

- E-10 spec corpus at D-331: 8 of 12 pass-5 findings closed; F-7+F-8 deferred to #115/#116.
- Pass-6 dispatches next on the post-D-331 spec package.
- BC-INDEX v1.11→v1.12; ARCH-INDEX v1.4→v1.5; STORY-INDEX v2.21→v2.22.
- Convergence trend: pass-1 CRIT (22) → pass-2 CRIT (11) → pass-3 HIGH (16) → pass-4 HIGH (16) → pass-5 HIGH (12). Approaching but not at NITPICK_ONLY. Counter still 0.

---

## D-333 Seal — E-10 Pass-6 Fix Cycle (2026-05-06)

### Pass-6 outcome

Pass-6 verdict: HIGH. Only 2 substantive findings (F-1 HIGH + F-2 HIGH) + 1 LOW polish
finding (F-3). The drop from 12 to 2 substantive findings is the largest single-pass
improvement in the E-10 adversarial review cycle. Quality has clearly converged. Counter
remains at 0 (not NITPICK_ONLY), but pass-7 may be the first NITPICK_ONLY pass.

### 2-burst fix cycle

D-332 PO (BC-1.12.009 F-2 Inv 4 disambiguation + F-3 PC4 State 5 label) → D-333
state-manager seal (pass-6 archival + F-1 ARCH-INDEX line 96 propagation + index seal).
Smallest fix cycle yet in the E-10 review. F-1 was an architect-typically-routed fix
(ARCH-INDEX text update) but rolled into the state-manager seal as a 1-line text change.

### Pattern-tracking entries

#### Same-document sibling-paragraph drift — occurrence 2 of N=3 trigger

**History:**
- Occurrence 1 (D-322/D-331): BC-1.11.002 Story Anchor was updated in the BC body but
  BC-INDEX and S-10.02 were not propagated in the same burst. POLICY 8 reverse-direction
  drift; eventually closed in D-331.
- Occurrence 2 (this pass — D-331/D-333): D-331 fixed ARCH-INDEX SS-03 row (line 85) from
  D-15.4 → D-15.1 but missed the renumbering-history paragraph in the same file (line 96).
  The paragraph still cited `ADR-015 D-15.4` even after line 85 was corrected in the same
  file in the same burst.

**Watch-item (NOT yet codified):** If pass-7 surfaces a third instance of same-document
sibling-paragraph drift (same canonical value changed in one location but not all locations
within the same file), codification is triggered. Rule to codify: "state-manager fix bursts
that change a canonical value in a file MUST grep the same file for ALL occurrences of the
changed value before sealing." Track as occurrence 2 of N=3.

#### Convergence trend signal

6 passes in, finding count: 22 → 11 → 16 → 16 → 12 → 2. The non-monotone passes (3 and 4
both at 16, with pass-3 higher than pass-2) reflected genuine spec complexity being surfaced;
they were not quality regressions. The overall direction is convergence. Pass-7 should be
the first NITPICK_ONLY pass or very close to it.

After 3 consecutive NITPICK_ONLY passes, CONVERGENCE_REACHED is declared. Step (vi) closes
and Step (vii) begins (S-10.01 Wave 0 read-only audit).

### F-7 / F-8 deferred status at D-333 seal

Tasks #115 + #116 still pending. The adversary explicitly skipped F-7 and F-8 in pass-6
per task instruction. They remain follow-up cleanup stories not blocking E-10 convergence.
No progress in the pass-6 fix cycle.

### Cycle bookkeeping at D-333 seal

- E-10 spec corpus at D-333: all 3 pass-6 findings closed; F-7+F-8 still deferred to #115/#116.
- Pass-7 dispatches next on the post-D-333 spec package.
- BC-INDEX v1.12→v1.13 (BC-1.12.009 v1.3→v1.4); ARCH-INDEX v1.5→v1.6 (F-1 line 96); STORY-INDEX v2.22 unchanged.
- Convergence trend: 22→11→16→16→12→2. Counter still 0. Pass-7 may be first NITPICK_ONLY.

---

## E-10 Pass-7 Outcome — 2026-05-06 (pre-compact pause)

### Pass-7 result

**Verdict: HIGH** — 1 substantive finding (F-1). Closure axes CC, DD, EE all VERIFIED PASS: pass-6 fixes landed cleanly in BC-1.12.009 and ARCH-INDEX.

**Trend:** 22 → 11 → 16 → 16 → 12 → 2 → 1 substantive findings across 7 passes. Genuinely approaching convergence.

**Counter:** Still 0. Pass-7 verdict is HIGH → counter does not advance. 3 consecutive NITPICK_ONLY still required.

### F-1 pattern-flag escalation (4th occurrence — D-15.4 → D-15.1 misattribution)

**History across 4 occurrences:**
- Occurrence 1: BC-3.05.004 Description line 33 — closed D-328 (architect burst)
- Occurrence 2: ARCH-INDEX line 83 — closed D-331 (state-manager seal)
- Occurrence 3: ARCH-INDEX line 96 — closed D-333 (state-manager seal, same-document sibling drift)
- Occurrence 4: invariants.md DI-013 line 102 — NEW pass-7 (different document than prior 3)

**Pattern:** The D-15.4 → D-15.1 fix was applied to BC-3.05.004, then propagated to ARCH-INDEX (twice — two separate sibling paragraphs). But it never propagated to the domain-spec layer (invariants.md). Each fix burst swept `specs/` and `stories/` but did not include `specs/domain-spec/` in the sweep.

**[process-gap] candidate:** Fix-burst checklists for cross-document canonical-value corrections must explicitly include `specs/domain-spec/` alongside `specs/behavioral-contracts/`, `specs/architecture/`, and `stories/`. This is a codification candidate for the next planning cycle if a 5th occurrence surfaces (current occurrence 4 already exceeds N=3 threshold — but the process-gap is filed as a task-level note per pass-7 F-1, not yet codified as a normative rule pending D-334 architect review).

### Closure axes CC/DD/EE — pass-6 fix verification

Pass-7 adversary ran explicit closure verification on all 3 pass-6 findings:
- **CC (F-1 D-333 closure):** ARCH-INDEX lines 85-98 consistently reference D-15.1. Sibling-paragraph drift closed.
- **DD (F-2 D-332 closure):** BC-1.12.009 Invariant 4 correctly disambiguates all 3 downgrade routes. EC-006 consistent.
- **EE (F-3 D-332 closure):** BC-1.12.009 PC4 carries explicit "State 5 — Non-paired" label, ordinal-uniform with States 1-4.

All 3 axes PASS. This confirms D-332 + D-333 fix bursts landed cleanly.

### Pre-compact pause context

Orchestrator preparing for context compact. STATE.md updated with comprehensive resumption pointer in current_step frontmatter field. Two parallel async tracks at pause point:

1. **E-10 convergence track:** D-334 architect burst (F-1 invariants.md fix) → D-335 seal → pass-8.
2. **Engine CI track:** async agent `ad190d8106711cb39` fixing release CI regression (Slice 3 reason code rename broke 22 bats test suites; binaries never bundled; marketplace PR never opened for rc.12).

### Cycle bookkeeping at pass-7 archival

- E-10 spec corpus at pass-7 archival: BC-INDEX v1.13; ARCH-INDEX v1.6; STORY-INDEX v2.22 (all unchanged from D-333).
- invariants.md DI-013 line 102 carries the unresolved D-15.4 misattribution — awaiting D-334 architect fix.
- F-7 + F-8 still deferred to cleanup stories #115/#116. Not re-flagged in pass-7.
- Convergence trend: 22→11→16→16→12→2→1. Counter still 0. Pass-8 may be first NITPICK_ONLY.
- Cycle still OPEN — 3-of-3 NITPICK_ONLY not yet reached.

---

### Lesson — D-15.4→D-15.1 misattribution propagation pattern (4th occurrence) [process-gap]

**Trigger:** Pass-7 F-1 (D-334) — invariants.md DI-013 line 102 cited ADR-015 D-15.4 when the correct decision is D-15.1.

**Pattern:** When a BC body cites the wrong ADR decision number, the misattribution propagates verbatim through downstream artifacts that reference the BC. We've now seen the same D-15.4→D-15.1 swap in:
1. BC-3.05.004 body (D-326 fix)
2. ARCH-INDEX.md line 83 (D-326 propagation fix)
3. ARCH-INDEX.md line 96 (sibling-paragraph; D-327 catch)
4. invariants.md DI-013 line 102 (D-334 — current fix)

**N=3 trigger reached.** Codification candidate: when a BC body references an ADR decision, the adversary should explicitly trace every downstream "Refined by:" / "Per ADR-015 D-X.Y:" citation back to the source BC and verify the decision number matches. A `validate-adr-decision-citation.sh` lint hook could catch this mechanically: parse `D-15.\d+` references in spec files, look up the cited decision in `architecture-decisions.md`, and flag mismatches against the surrounding context (e.g., a paragraph about "warn-and-skip schema validation" should cite the schema-validation decision, not the trace-propagation decision).

**Routing:** TD-VSDD entry to be opened post-cycle. Tag: `[codification-candidate]`.

---

### Lesson — DI-017 dispatcher_trace_id rename propagation gap [process-gap]

**Trigger:** Pass-8 F-1 (D-336) — BC-1.11.001 Precondition 2 line 50 carried `dispatcher_trace_id` despite DI-017 v1.1 (D-314, 2026-05-06) explicitly canonicalizing the rename to `trace_id`. Comprehensive sweep on D-336 found additional instances at: BC-1.05.012 (Description+Postconditions+Invariants+Test Vectors), BC-1.05.018 (Description RESERVED_FIELDS list), BC-1.06.007 (Description test vector field), BC-1.06.008 (Description+Postcondition 1), BC-1.06.009 (Description envelope field), BC-1.05.010 (H1 title+Description), BC-1.05.033 (Description+Postcondition 1), BC-1.10.001 (Invariant 1), BC-3.03.008 (Description+Preconditions+Postconditions+TVs+VPs), BC-3.05.003 (Description+Postcondition 1, retired BC), BC-4.04.001 (Description+Postconditions+TVs+DI-017 row), BC-4.05.001 (Description+Postconditions+TVs+Related BCs+DI-017 row), BC-4.07.001 (Description+Postconditions+TVs+Related BCs+DI-017 row), BC-4.07.002 (Description+Postconditions+TVs+DI-017 row), BC-4.08.001 (Description+Postconditions+TVs+Related BCs+DI-017 row). Total: 15 BC files modified, 40+ individual occurrences corrected.

**Pattern:** When `invariants.md` declares a rename mandate ("Any reference to `<OLD>` in existing code or specs is a drift artifact to be corrected"), there is no automated gate that grep-verifies the entire spec tree at seal time. The same mandate exists for `schema_version=1`→`schema_version=2` for observability-config.toml (DI-014 v2 update) — and that rename ALSO leaked into ARCH-INDEX line 151 (D-336 F-3, separate fix).

**Codification candidate:** Add a `validate-rename-propagation.sh` lint hook that:
1. Reads invariants.md for any "Renamed by ... `<OLD>` → `<NEW>`" or "Any reference to `<OLD>`... is a drift artifact" pattern
2. Greps the entire `specs/` tree for `<OLD>`
3. Excludes lines that match historical-reference patterns ("renamed from", "Updated per", inside CHANGELOG sections, inside Q&A blocks)
4. Reports remaining hits as findings on next state-manager seal

---

### Lesson — perf-baseline.bats relative-path bug (rc.13 release blocker) [process-gap]

**Trigger:** rc.13 tag push on 2026-05-07 — release.yml validate job failed at perf-baseline.bats line 23. PR #96 (Slice 3 reason code test alignment) had restored 22 previously-failing bats suites; this exposed perf-baseline.bats as the ONE remaining validate-job failure that's been latent since PR #91 (perf baseline + bundle ceiling addition).

**Pattern:** Latent CI bugs in test-infrastructure files become visible only when prior bugs in the SAME workflow are fixed. perf-baseline.bats was added in PR #91 but never verified end-to-end on a release tag because the OTHER bats failures (Slice 3 reason codes) blocked validate from ever reaching it. POLICY 11 (`ci_positive_coverage_assertion`) would have caught this: a regression-detector CI job that doesn't emit a positive-coverage assertion in its log on every run is undetectable when ANOTHER regression masks it.

**Bug detail:** perf-baseline.bats line 23 used `git -C "$BATS_TEST_DIRNAME" rev-parse --git-common-dir 2>/dev/null | sed 's|/\.git$||'` which returns a RELATIVE path (e.g., `../../.git` → after sed → `../../..`). When `run-all.sh` runs bats from `plugins/vsdd-factory/`, `FACTORY_DIR="../../../.factory"` resolves THREE levels above the repo root — pointing into the parent of `/Users/jmagady/Dev/`. Fix: replace with `git rev-parse --show-toplevel` (cwd-independent absolute path).

**Routing:** Open follow-up: TD-VSDD entry to enforce POLICY 11 retroactively on all `tests/*.bats` that gate validate. Specifically: every bats suite that touches the `.factory` mount must emit a positive-coverage assertion ("FACTORY_DIR resolved to: <abs path>; mount confirmed") so a missing assertion in the run log signals a path-resolution latent bug. Tag: `[codification-candidate]`.

**Routing:** TD-VSDD entry post-cycle. Tag: `[codification-candidate]`. Already have N=2 trigger here (DI-013 D-15.4→D-15.1 was 4-occurrence; DI-017 dispatcher_trace_id is 2nd known propagation pattern in this cycle). Recommend codifying after the 3rd distinct rename-propagation event.

---

### Lesson — bats inline `_write_registry()` heredocs diverge from production registry shape with no automated parity check [codified] [process-gap]

**Discovered:** 2026-05-17 (S-15.11 LOCAL adversary pass-2 as O-P2-003 / PG-1)
**Category:** test-infrastructure-parity
**Severity:** HIGH — masked a HIGH-severity production defect (F-S15.11-LOCAL-P2-001)

### Gap Description

bats integration tests for WASM hook stories inline a `_write_registry()` helper that writes a synthetic hooks-registry.toml fixture. In S-15.11, the inline `path_allow` array used `.factory/cycles/` (a literal directory path, no glob), while the production hooks-registry.toml entry for `validate-burst-log` used `path_allow = [".factory/cycles/**"]` (an unsupported glob pattern). The `canonicalize()` call inside the resolver-linker silently returned `CapabilityDenied` for the glob path → the hook fail-opened → production capability semantics were never exercised by the bats suite.

This gap DIRECTLY CAUSED F-S15.11-LOCAL-P2-001 (HIGH severity): production hook `validate-burst-log` was silently neutered via `canonicalize()` failure on the unsupported glob, and the bats suite never detected it because the inline registry used a valid non-glob path. The HIGH finding was only caught by the LOCAL adversary pass-2 adversarial review — not by any automated test gate.

### Root Cause

bats inline `_write_registry()` heredocs are authored manually per-story without a gate verifying they are byte-identical to the production registry entry for the same hook. There is no CI lint or pre-commit check that diffs the inline `path_allow` arrays against the production `hooks-registry.toml` entry for the same hook and blocks on drift.

### Systemic Pattern

Any future per-hook story can introduce the same divergence: bats inline fixture passes locally (because the test uses simplified registry shape), production hook silently misbehaves (because the actual registry shape triggers a different code path). The recurrence risk is HIGH because every new WASM hook story in S-15.03 PRIORITY-A wave (S-15.09, S-15.14, etc.) authors its own `_write_registry()` inline fixture.

### Lesson

Future per-hook stories MUST include an integration test that extracts `path_allow` verbatim from production `hooks-registry.toml` and exercises the hook against that EXACT registry shape. This pattern is proven load-bearing: S-15.11 integration-production-registry.bats Scenario B demonstrated that the production-registry-shape test catches the `canonicalize()` / glob failure that the inline-fixture test misses.

**Codification target:** S-15.03 PRIORITY-A automation wave should include a CI lint or pre-commit gate that diffs bats inline `_write_registry()` `path_allow` arrays against the production hooks-registry.toml entry for the same hook — block on any drift. This is out-of-scope for S-15.11; tracked as Drift Items entry `PG-S-15.11-bats-prod-registry-parity-gate` in STATE.md.

### Disposition

- **S-7.02 codification satisfied 2026-05-17** — O-P2-003 `[process-gap]` finding from S-15.11 LOCAL adversary pass-2 codified here per S-7.02 Cycle-Closing Checklist step 3.
- **Drift Items entry:** `PG-S-15.11-bats-prod-registry-parity-gate` OPEN in STATE.md Drift Items table; target release: S-15.03 PRIORITY-A automation wave (CI lint or pre-commit gate; story TBD).
- **S-15.11 cascade report reference:** `.factory/code-delivery/S-15.11/adv-local-pass-2.md` — O-P2-003 source document.

---

### Lesson — real-target file-size test discipline: bats tests for WASM validators MUST exercise the validator against the actual production target file, not a synthetic fixture [codified] [process-gap]

**Discovered:** 2026-05-17 (S-15.09 LOCAL adversary pass-5 as F-P5-002 [LOW — ORCHESTRATOR-ELEVATED TO CRITICAL])
**Category:** test-infrastructure-parity
**Severity:** CRITICAL (orchestrator elevation from LOW) — hook was silently inert against STATE.md in production

### Gap Description

S-15.09 shipped validate-state-structure with `max_bytes = 65536` in its `host::read_file` call. The bats tests used small synthetic STATE.md fixtures that fit within the cap. Real STATE.md is 95,185 bytes (95 KB), well above the cap. When the hook was dispatched against real STATE.md on develop, `host::read_file` returned `Err(OutputTooLarge)` → hook silently fail-opened → `exit_code=0` with no block → validator was SILENTLY INERT for its entire intended purpose.

The pass-4 bats suite returned green because the synthetic fixtures were small. No bats test ever loaded the real `.factory/STATE.md` and verified that the hook actually blocked on a deliberate structural violation of that file.

### Root Cause

bats integration tests for WASM validators use small synthetic fixtures authored per-story. There is no gate that verifies the hook can successfully read and process the ACTUAL production target file. The `host::read_file` fuel/size cap is a silent failure mode: the hook returns `exit_code=0` as if it checked the file and found nothing wrong, when in fact it never read the file at all.

### Systemic Pattern

Every WASM hook story in S-15.03 PRIORITY-A wave (S-15.07, S-15.11, S-15.09, S-15.14) is vulnerable to this class: if `max_bytes` is below the real production target file size, the hook is silently inert in production while appearing to pass all bats tests. This is the META-LEVEL-24 false-green pattern at the integration layer.

### Lesson

Future per-hook stories MUST include at least one bats test that (a) copies the actual production target file (`.factory/STATE.md`, `.factory/cycles/v1.0-brownfield-backfill/burst-log.md`, etc.) into the bats fixture directory, (b) deliberately introduces a structural violation into that copy, and (c) asserts that the hook blocks (`exit_code=2`). If the hook silently pass-opens against a 95 KB file, that is the FIRST finding to fix before shipping. A "test the test" verification step must be part of the per-story implementation checklist.

Fix applied in S-15.09: `max_bytes` raised from 65,536 → 524,288 (512 KB); regression test added that loads real STATE.md size and verifies blocking behavior on deliberate banner-wc-l mutation.

**Codification label:** PG-S-15.09-real-target-test-discipline

### Disposition

- **S-7.02 codification satisfied 2026-05-17** — F-P5-002 `[process-gap]` class from S-15.09 LOCAL adversary pass-5 codified here per S-7.02 Cycle-Closing Checklist step 3.
- **Structural fix shipped in S-15.09:** `host::read_file` max_bytes 65536→524288; regression test on real-size STATE.md added.
- **S-15.09 cascade report reference:** `.factory/code-delivery/S-15.09/adv-local-pass-5.md` — F-P5-002 source document.

---

### Lesson — same-burst self-cite sweep on version-bump: when a burst bumps a file's version, ALL intra-file citations of the previous version must be updated in the same burst [codified] [process-gap]

**Discovered:** 2026-05-17 (S-15.09 LOCAL adversary pass-7 as F-P7-001 [MEDIUM])
**Category:** sibling-site-sweep
**Severity:** MEDIUM — stale version cite in Token Budget section after same-burst spec version bump

### Gap Description

S-15.09 pass-6 fix-burst promoted the story spec from v1.5 to v1.6 AND updated the Token Budget section to cite "v1.5" (the immediate predecessor). The Token Budget cite was stale the moment it was written: the burst that bumped the version should have cited the NEW version (v1.6), not the prior one.

This is a recurrence of the TD-VSDD-060 sibling-site sweep discipline applied to intra-file self-references: a version-bump burst must grep for ALL occurrences of the old version string within the same file and update them before committing. The same-burst Token Budget cite is a prescribed site per D-421(c) / D-446(c) class.

### Root Cause

When a burst bumps a frontmatter `version:` field, the implementer updates the frontmatter but does not grep the file body for residual citations of the old version. Token Budget sections, AC tables, and body narrative sections that cite the current spec version are prescribed sibling sites that must be updated atomically with the version bump.

### Systemic Pattern

This pattern has recurred across multiple S-15.03 PRIORITY-A cascade stories. Every spec version bump creates a new stale-cite window if the same-burst sibling sweep is not applied to the entire file. The stale cite is always caught by the adversary in the following pass (never in the bumping burst itself), adding one wasted pass to every cascade.

### Lesson

When a burst bumps a spec file's version (frontmatter `version:` field or `spec_version:` field), the implementer MUST run a literal grep for the old version string across the entire file and update all occurrences before committing. The command pattern:

```bash
grep -n "v<OLD_VERSION>" <spec-file>
```

Any hit is a prescribed sibling site. Zero hits = sweep complete. This grep must appear in the burst-log Dim-2 Attestation for any burst that includes a version bump.

**Codification label:** PG-S-15.09-self-cite-sweep-on-version-bump

### Disposition

- **S-7.02 codification satisfied 2026-05-17** — F-P7-001 `[process-gap]` class from S-15.09 LOCAL adversary pass-7 codified here per S-7.02 Cycle-Closing Checklist step 3.
- **Fix applied in S-15.09 pass-7 fix-burst:** story-writer applied same-burst self-cite sweep; zero remaining live body cites of v1.6 or earlier verified.
- **S-15.09 cascade report reference:** `.factory/code-delivery/S-15.09/adv-local-pass-7.md` — F-P7-001 source document.

---

## PG-S-15.14-tdd-micro-commit-discipline

**Source:** S-15.14 LOCAL adversary pass-1 F-P1-007 [process-gap]
**Date:** 2026-05-17
**Cross-reference:** TD-VSDD-095 (Re-allocated from TD-VSDD-064 — wrongly reused at S-15.14 pass-1 burst a3b133b8; corrected in pass-2 burst per F-P2-001)

S-15.14 LOCAL adversary pass-1 F-P1-007 finding: implementer chunked all 6 performance criteria (PCs) into a single commit instead of per-PC micro-commits as required. CLAUDE.md routing-table specifies implementer = "one failing test → minimum code → micro-commit". The expected pattern for a multi-PC hook story is 3-6 commits, each scoped to a logical AC or PC group.

### Root Cause

Hook-implementer dispatch packages did not include an explicit reminder of the micro-commit discipline. The implementer treated the entire story as one atomic unit rather than decomposing it per the TDD protocol.

### Lesson

Going-forward: hook-implementer dispatches MUST commit per logical scope (3-6 commits for multi-PC stories). The dispatch package MUST include the instruction: "Commit after each logical AC group or performance criterion; do not batch the entire story into a single commit." Verified in S-15.14 fix-burst (7 micro-commits e4427df4..f20bbdab applied post-adversary).

### Disposition

- **F-P1-007 [process-gap] CLOSED 2026-05-17** — codified here per S-7.02 Cycle-Closing Checklist step 3; no code-level fix required (structural fix applied in fix-burst micro-commits).
- **S-15.14 cascade report reference:** `.factory/code-delivery/S-15.14/adv-local-pass-1.md` — F-P1-007 source document.

---

## PG-S-15.14-registry-priority-literal-evidence

**Source:** S-15.14 LOCAL adversary pass-1 F-P1-013 [process-gap]
**Date:** 2026-05-17
**Cross-reference:** TD-VSDD-096 (Re-allocated from TD-VSDD-065 — wrongly reused at S-15.14 pass-1 burst a3b133b8; corrected in pass-2 burst per F-P2-001)

S-15.14 LOCAL adversary pass-1 F-P1-013 finding: `hooks-registry.toml` priority allocation inline comment used narrative-attested grep verification ("priority range 100-199 confirmed") without citing literal `file:line:` grep stdout. This is a META-LEVEL-24 D-449(a) self-application class: the comment claims a mechanical grep was run but does not show captured stdout.

### Root Cause

Registry priority allocation comments are produced by implementers who follow the D-449(a) discipline for burst-log Dim-2 blocks but do not apply the same standard to inline code comments. The inline comment is treated as narrative documentation rather than a mechanical attestation site.

### Lesson

Going-forward: registry priority allocation inline comments MUST cite literal `file:line:` grep stdout showing the full priority range in use for all co-registered hooks. The command pattern:

```bash
grep -n "^priority" plugins/vsdd-factory/hooks-registry.toml | grep -E "^[0-9]+:priority = 1[0-9]{2}"
```

Output must be captured and cited verbatim in the inline comment (or in the commit's burst-log Dim-2 if the registry itself is not the right place for multi-line stdout). Zero ambiguity in the citation form — `file:line: priority = NNN` format.

### Disposition

- **F-P1-013 [process-gap] CLOSED 2026-05-17** — codified here per S-7.02 Cycle-Closing Checklist step 3; no code-level fix required (priority range is correct; only evidence form was deficient).
- **S-15.14 cascade report reference:** `.factory/code-delivery/S-15.14/adv-local-pass-1.md` — F-P1-013 source document.
