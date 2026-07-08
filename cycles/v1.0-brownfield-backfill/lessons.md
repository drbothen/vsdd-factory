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

---

### PG-orchestrator-dispatch-template-canonical-marker (TD-VSDD-097)

**Date:** 2026-05-18
**Discovered:** S-15.14 LOCAL adversary pass-6 finding F-P6-001
**Source:** orchestrator dispatch template (pass-5 state-manager dispatch prompt)

**Issue:** Orchestrator wrote state-manager dispatch template for pass-5 persistence containing `current_step:` pattern that omitted the canonical `trajectory-tail →N→N→N→N` marker. State-manager followed template verbatim. Result: STATE.md:15 current_step VIOLATED BC-5.39.006 v1.2 PC-6 — the very PC the S-15.14 cascade was building the validator to enforce. Pass-6 fresh-context review caught it.

**Class:** META-LEVEL-class self-violation — codifying-burst violates its own codified rule. Same class as F-P2-001 (state-manager violated POLICY 1 in burst codifying POLICY 1 lesson) and F-P3-001 (state-manager violated D-449(a) in burst codifying D-449(a) lesson via TD-VSDD-096).

**Going-forward rule:** ALL orchestrator dispatch templates for state-manager that produce STATE.md `current_step:` writes MUST include the canonical `trajectory-tail →N→N→N→N` marker per BC-5.39.006 v1.2 PC-6. The marker is non-optional regardless of narrative content.

**Verification gate:** before state-manager dispatch, orchestrator must verify the dispatch prompt's current_step pattern contains `trajectory-tail ` (with trailing space). If absent, dispatch is INVALID and must be rewritten.

**Cross-reference:** TD-VSDD-095 (TDD micro-commit) + TD-VSDD-096 (registry priority literal-evidence) + TD-VSDD-097 (orchestrator dispatch template marker) form a triplet of META-LEVEL-class codification-without-application failures discovered across S-15.14 passes 1-6.

**Codification:** TD-VSDD-097 (verified next available: prior max was TD-VSDD-096 per grep -rohE "TD-VSDD-[0-9]+" .factory/ | sort -u | tail)

### Disposition

- **F-P6-001 [process-gap] CLOSED 2026-05-18** — state-manager fix burst restored canonical marker in STATE.md current_step; TD-VSDD-097 codified here; orchestrator dispatch-template going-forward rule recorded in §3 User Directive carry-forward.
- **S-15.14 cascade report reference:** `.factory/code-delivery/S-15.14/adv-local-pass-6.md` — F-P6-001 source document.

---

**EXTENSION 2026-05-18 (pass-7 F-P7-001 root cause):** TD-VSDD-097 initially scoped to PC6 marker only is INSUFFICIENT. Pass-6 fix-burst followed the narrow rule but DROPPED PC5 D-chain currency, introducing F-P7-001 stale-D-chain regression.

**EXTENDED rule:** Orchestrator dispatch templates for state-manager STATE.md current_step writes MUST satisfy ALL 5 BC-5.39.006 v1.2 PCs simultaneously:
- PC2: NO forbidden meta-commentary regex patterns (META-LEVEL-N WATCH, self-app TEST, expected verdict)
- PC3: All 4 index version cites present (BC-INDEX vX, VP-INDEX vX, STORY-INDEX vX, ARCH-INDEX vX)
- PC4: trajectory-tail LENGTH=4 (exactly 4 → arrow-separated values after the canonical marker)
- PC5: D-chain currency (max D-(\d+) extracted from current_step ≥ max D-NNN in STATE.md body)
- PC6: canonical `trajectory-tail ` marker present (with trailing space)

**Verification gate (pre-dispatch):** orchestrator must mentally run all 5 PC checks against the candidate current_step template BEFORE dispatching state-manager. If ANY PC would fail, rewrite template. The dispatch template is the upstream defect site; state-manager merely executes.

**Structural countermeasure (forward-looking, not yet implemented):** `bin/preflight-current-step <staged-STATE.md>` script that runs validate_state_md against staged content; if violations found, state-manager refuses to commit until orchestrator fixes template. This would prevent the next-class self-violation. Track as forward-backlog item.

**Cross-reference:** PG-S-15.14-tdd-micro-commit-discipline (TD-VSDD-095) + PG-S-15.14-registry-priority-literal-evidence (TD-VSDD-096) + PG-orchestrator-dispatch-template-canonical-marker (TD-VSDD-097 EXTENDED) — triplet of META-LEVEL-class codification-without-application failures.

- **F-P7-001 [replacement-regression] CLOSED 2026-05-18** — state-manager fix burst restored D-chain cite to D-476 in STATE.md current_step per BC-5.39.006 v1.2 PC5; TD-VSDD-097 extended here; 5-PC dispatch rule recorded in §3 User Directive carry-forward.
- **S-15.14 cascade report reference:** `.factory/code-delivery/S-15.14/adv-local-pass-7.md` — F-P7-001 source document.

---

### PG-orchestrator-compaction-burst-sibling-sweep (TD-VSDD-098)

**Date:** 2026-05-18
**Discovered:** S-15.14 LOCAL adversary pass-9 findings F-P9-001 + F-P9-002 + F-P9-003

**Issue:** Pass-8 STATE.md compaction burst introduced 3 sibling-sweep defects:
- Compaction summary rows mis-cited E-10 pass-9..14 preservation location (burst-log.md vs per-pass files)
- Active Branches factory-artifacts SHA-advance missed (still pass-7 SHA after pass-8 compaction)
- Concurrent Cycles Status header bold-summary stale at pass-3 (not advanced across 5 bursts)
- Compaction trend label inaccurate (claimed "passes 9-14" but data was full 14-pass cascade)

**Root cause:** Orchestrator's compaction-burst dispatch template did NOT instruct state-manager to:
(a) verify cited preservation paths exist via literal shell BEFORE committing
(b) advance Active Branches row SHA to compaction-burst HEAD per D-445(c)+D-446(d)+D-447(c)+D-449(e)
(c) advance Concurrent Cycles Status bolded-summary header at every persist burst
(d) verify compaction summary labels (trend, count, etc.) match source data exactly

**Going-forward rule:** Compaction bursts MUST include sibling-sweep verification of:
1. Every cited preservation path: `ls <cited-path> && grep <expected-content-marker> <cited-path>` to prove existence + faithfulness
2. Active Branches factory-artifacts row SHA advance to current burst HEAD
3. Concurrent Cycles Status bolded-summary header advance to current persist burst
4. Trend labels match data (cardinality + range)

**Cross-reference:** TD-VSDD-095/096/097 (TDD micro-commit + literal-evidence + dispatch-template canonical-marker) form a triplet of orchestrator-dispatch-template completeness gaps. TD-VSDD-098 extends to compaction-burst-specific scope.

**Closes:** F-P9-001, F-P9-002, F-P9-003 (process-gap class)

---

### PG-orchestrator-own-burst-log-structural-integrity (TD-VSDD-099)

**Date:** 2026-05-18
**Discovered:** S-15.14 LOCAL adversary pass-10 finding F-P10-001

**Issue:** Pass-9 fix-burst committed with own burst-log entry missing Dim-7 block; Dim-6 narratively attested "8 blocks present" without shell-verified count. Same META-LEVEL self-violation class as F-P6-001/F-P7-001/F-P9-001 (codifying-burst violates rule in its own attestation).

**Root cause:** Orchestrator dispatch template for pass-9 fix-burst said "Standard 8-block D-444(c) entry" without specifying VERIFICATION via literal shell of all 4 Dim blocks. State-manager omitted Dim-7 inadvertently; Dim-6 narratively self-attested without grep proof.

**Going-forward rule:** Every state-manager burst-log entry MUST include:
1. All 4 Dim attestation blocks (Dim-2, Dim-5, Dim-6, Dim-7) — non-optional, even if narrative is brief
2. Dim-6 attestation MUST contain literal `awk '/^## <h2-name>/,/^## [^S]/' burst-log.md | grep -cE '^### (Parent-commit|Adversary verdict|Files touched|Codifications|Dim-2|Dim-5|Dim-6|Dim-7|Closes|Factory-artifacts commits)'` with captured stdout, not narrative paraphrase
3. Pre-commit gate: orchestrator dispatch template MUST instruct state-manager to verify all 4 Dim blocks via literal shell BEFORE staging the burst-log file

**Cross-reference:** TD-VSDD-095/096/097/098/099 form an ever-expanding META-LEVEL orchestrator-dispatch-template completeness perimeter; F-P10-001 is the 5th class.

**Structural countermeasure (forward-looking):** the `bin/preflight-current-step` script proposed in TD-VSDD-097 EXTENSION could be extended to a more general `bin/preflight-state-manager-burst` that runs:
- All 5 BC PC checks on candidate current_step
- D-444(c) 8-block structural check on candidate burst-log entry
- D-446(a) own-burst-log gate (4 Dim blocks present)
- Cited preservation path existence + content faithfulness for compaction bursts
- Active Branches SHA-advance check
- Concurrent Cycles Status header advance check

Until that script lands, the orchestrator dispatch template must enumerate ALL these checks explicitly.

**Closes:** F-P10-001 (own-burst-log structural-integrity false-green class)

---

### PG-orchestrator-dim2-pc-attestations-must-read-production (TD-VSDD-100)

**Date:** 2026-05-18
**Discovered:** S-15.14 LOCAL adversary pass-11 finding F-P11-002

**Issue:** Pass-9 + pass-10 burst-log Gate 3 (PC4 LENGTH=4) used `echo 'trajectory-tail ->9->9->9->9' | grep -oE "->[0-9]+" | wc -l` — synthetic ASCII string, not production STATE.md current_step (which uses Unicode →). Result is structurally a literal-shell invocation with captured stdout (satisfying TD-VSDD-096 / D-449(a)) BUT the input is hand-crafted, not the real artifact. Gates present-and-running but content-inert.

**Class:** 6th META-LEVEL self-violation in S-15.14 cascade. TD-VSDD-099 closed structural completeness of Dim blocks; this class is content-validity of the attestation evidence.

**Going-forward rule:** Every Dim-2 PC attestation in state-manager burst-log entries MUST read the actual production artifact, not synthetic / placeholder / hand-crafted input.

Specifically for BC-5.39.006 v1.3 PC checks on STATE.md current_step:
- PC2 (forbidden meta): `grep "^current_step:" .factory/STATE.md | grep -E "META-LEVEL-[0-9]+ WATCH|self-app TEST|expected verdict"`
- PC3 (4 index cites): `grep "^current_step:" .factory/STATE.md | grep -oE "BC-INDEX v[0-9.]+|VP-INDEX v[0-9.]+|STORY-INDEX v[0-9.]+|ARCH-INDEX v[0-9.]+" | sort -u`
- PC4 (LENGTH=4 in semicolon segment per v1.3): `grep "^current_step:" .factory/STATE.md | awk -F'trajectory-tail ' '{print $2}' | awk -F';' '{print $1}' | grep -oE "→[0-9]+" | wc -l`
- PC5 (D-chain currency): `grep "^current_step:" .factory/STATE.md | grep -oE "D-[0-9]+" | sort -t- -k2 -n | tail -1` vs `grep -oE "D-[0-9]+" .factory/STATE.md | sort -t- -k2 -n | tail -1`
- PC6 (marker present): `grep "^current_step:" .factory/STATE.md | grep -c "trajectory-tail "`

Synthetic `echo` form is FORBIDDEN regardless of how syntactically correct the shell pattern is.

**Cross-reference:** TD-VSDD-095/096/097/098/099/100 form 6-class orchestrator-dispatch-template completeness perimeter.

**Structural countermeasure (forward-looking, still not implemented):** `bin/preflight-state-manager-burst` script that runs all 5 PC checks against production STATE.md before state-manager commits. Until that lands, dispatch templates must enumerate every check explicitly with production-read pattern.

**Closes:** F-P11-002 (Dim-2 PC attestation content-validity class)

---

### L-S-15.14-asymptotic-acceptance

**Date:** 2026-05-18
**Class:** asymptotic-acceptance precedent (cumulative third instance after F5 D-386 + E-10 D-471)

**Cascade trajectory:** 16 → 9 → 8 → 2 → 0 → 1 → 1 → 0 → 4 → 1 → 2 (11 passes)
**Best streak achieved:** 1/3 (passes 5 and 8 — both immediately followed by HIGH-finding regression)
**META-LEVEL classes codified:** TD-VSDD-095..100 (6 classes — see Appendix D of SK-MCP-001 for INV-NNN mapping)

**Empirical confirmation of prose-rule convergence failure:**
S-15.14 confirms the prism POL-29 amendment chain pattern: each prose-rule codification closes one violation class and opens an adjacent class. Under prose-only governance, 3-CLEAN convergence per BC-5.39.001 is structurally impossible because the orchestrator dispatch templates accumulate complexity faster than they reach closure.

**Resolution:** asymptotic-acceptance at recurrence floor. The S-15.14 cascade is SEALED. Per-story-delivery proceeds. Structural resolution gated on SK-MCP-001 Tier 2 implementation (typed graph invariants enforced by MCP mutations + dispatcher pre-commit hook).

**Cross-reference:** F5 D-386 Option C (engine-discipline cycle paused 2026-05-13); E-10 D-471 (brownfield E-10 sub-cycle partial-closed 2026-05-14); D-477 (S-15.14 asymptotic-acceptance 2026-05-18). Three independent cycles reached the same conclusion: structural countermeasures (SK-MCP-001 typed invariants + dispatcher hook) are required for 3-CLEAN convergence; prose rules cannot.

**Forward-looking:** When SK-MCP-001 Tier 2 ships, run a retrospective fresh-context adversary review of S-15.14 + E-10 + F5 cycle artifacts to verify INV-011/012/013/014 catch all 6 TD-VSDD-095..100 classes mechanically. If yes, the cascades can be marked CONVERGED retroactively under structural governance. If no, schema-version increments fill remaining gaps.

**Closes:** D-477 ASYMPTOTIC-ACCEPTANCE authorization (S-15.14 cascade SEALED)

## L-S-15.14-SHIPPED — Asymptotic-Acceptance Precedent Chain: Third Independent Instance Confirms Structural Resolution Required

**Date:** 2026-05-18
**Source:** S-15.14 post-merge burst D-479
**Class:** Asymptotic-acceptance precedent chain codification

S-15.14's LOCAL adversary cascade reaching asymptotic-acceptance at D-477 (11 passes; trajectory 16→9→8→2→0→1→1→0→4→1→2; best streak 1/3) established the third independent instance of the same pattern: F5 cycle D-386, E-10 sub-cycle D-471, and now S-15.14 D-477. All three cycles reached identical conclusions independently — each fix-burst closed one META-LEVEL class while opening an adjacent one, producing a recurrence floor that prose-rule codification alone cannot close. This triple-instance confirmation elevates the finding from "pattern" to "structural law" for vsdd-factory: asymptotic convergence under prose-only rules is the expected outcome; BC-5.39.001 3-CLEAN convergence requires structural automation (SK-MCP-001 typed invariants + dispatcher hooks). The 6 META-LEVEL classes from this cascade (TD-VSDD-095..100) were forwarded to SK-MCP-001 Appendix D as INV-011..014, forming the empirical basis for the structural fix design. S-15.14 itself (the validate-dispatch-advance WASM hook) is the first shipped artifact from the S-15.03 PRIORITY-A automation wave targeting this structural floor — 22 ACs in production, all PASS, PR #148 merged at `6d2ba5ad`. M2 COMPLETE. M3 gate SATISFIED.

## L-M3-commissioning — M3 Commissioning at S-15.14 SHIPPED: Natural-Ordering Holds, CI Paper-Fix Scheduled Within M3

**Date:** 2026-05-18
**Source:** D-480 M3 commissioning state advance
**Class:** Pipeline decision discipline + Production-Grade Principle Rule 4 in practice

M3 commissioning at S-15.14 SHIPPED exemplifies the engine-discipline natural-ordering decision: M3 closure before SK-MCP-001+UNI-PLUG-001 perimeter expansion. The asymptotic-acceptance precedent triple-stamp (F5 D-386 → E-10 D-471 → S-15.14 D-477) confirms that structural countermeasures (SK-MCP-001 typed invariants) are the correct eventual resolution, but the natural ordering of S-15.03 PRIORITY-A milestones must proceed first to ship the mechanical automation wave before expanding scope. ADR-021 (WASM cargo-audit sandboxing) and ADR-022 (hook current-pass context discovery) were ACCEPTED 2026-05-15 and are already M3-gating; no new ADR authorship is needed in M3. The M3 scope (5 stories: S-15.10/12/13/15/16-Part-B) follows the wave-plan precedence order established at s-15.03-wave-plan-2026-05-15.md §4. CI paper-fix TD-VSDD-101 (VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 env-var skip) scheduled within M3 anchored to S-15.15, confirming Production-Grade Principle Rule 4 in practice: an AI-built defect (the env-var test gate) was routed to fix-in-scope-of-next-touching-story rather than deferred to a register entry without concrete anchor. The Canonical Principle Rule 3 three-condition gate (human-directed + concrete future dependency + future-story anchor) was satisfied explicitly: human directed 2026-05-18, structural fix requires CI-infrastructure work that S-15.15 already touches, anchor is S-15.15 precisely.

**Closes:** D-480 M3 commissioning codified.

---

## L-M3-BC-cascade-pass-1 — BC spec format must be verified against actual artifact corpus; adversary fresh-context dispatch must grep canonical source

**Date:** 2026-05-18
**Source:** D-482 M3 BC cascade pass-1 state-manager persistence burst
**Class:** Spec-authorship ground-truth discipline + adversary-process canonical-source discipline

Spec-reviewer and adversary were dispatched in parallel for BC-5.39.007 + BC-5.39.008 (M3 BC cascade pass-1, 2026-05-18). The parallel-dispatch pattern produced valuable cognitive-diversity coverage: spec-reviewer found 0 P1 blockers (SUGGESTIONS_ONLY verdict), while adversary found 2 verified CRITICAL findings that spec-reviewer missed.

**META-LEVEL finding 1: "BC spec format claims have NO load-bearing validation against actual artifact format."** BC-5.39.007 prescribes a format for the Closes section of lessons.md entries (`### Closes` h3). The actual artifact corpus uses `**Closes:**` bold-prefix-line form (verified at lessons.md lines 1748/1778/1806/1828/1846 via literal grep). This discrepancy was not caught by spec-reviewer because spec-reviewer read only the BC text, not the actual artifact corpus. The adversary's fresh-context artifact-corpus cross-check caught it. Root cause: BC authorship burst for BC-5.39.007 did not include a literal-shell grep of the actual lessons.md corpus to confirm the prescribed format matches reality. Normative discipline: when a BC specifies a format that must match an existing artifact convention, the BC authorship burst MUST include a literal-shell grep of actual artifacts before the BC is sealed. Forwarded to SK-MCP-001 Appendix D as INV-016-CANDIDATE: "BC-authorship-must-grep-actual-artifact-format."

**META-LEVEL finding 2: "Adversary fresh-context dispatch must grep canonical source of truth."** The adversary produced a CRITICAL finding (F-BC008P1-001) claiming TD-VSDD-101 was absent and VSDD_SKIP_PRODUCTION_STATE_MD_TEST was absent from CI. Orchestrator verified via literal shell: TD-VSDD-101 EXISTS at `tech-debt-register.md:45`; the env-var EXISTS at `origin/develop:.github/workflows/ci.yml` lines 141/153/398/405. Root cause: the adversary grepped the stale local main checkout (`392b56d6`) instead of the canonical sources (factory-artifacts branch for `.factory/` files; `origin/develop` for source code). Local main was 5+ commits behind develop. Normative discipline: adversary fresh-context dispatches MUST explicitly cite and grep the canonical source for each TD/file/identifier check — factory-artifacts for spec/state files, `origin/develop` for source-code files, NOT local main. This false positive was recovered by orchestrator literal-shell verification before the false positive was persisted as a finding. Forwarded to SK-MCP-001 Appendix D as INV-015 process-gap: L-EDP1-067-CANDIDATE "adversary-fresh-context-must-grep-canonical-source."

**Closes:** D-482 + F-BC007P1-001 (verified CRITICAL — `**Closes:**` vs `### Closes` format mismatch) + F-BC008P1-002 (verified CRITICAL — PC13 ADR-021 Option (a) contradiction) + L-EDP1-067-CANDIDATE-INV-015-forward (adversary-fresh-context-must-grep-canonical-source forwarded SK-MCP-001 Appendix D INV-015) + INV-016-CANDIDATE-forward (BC-authorship-must-grep-actual-artifact-format forwarded SK-MCP-001 Appendix D INV-016).

## L-M3-BC-cascade-pass-1-PO-fix-burst — PO fix-burst closed 41/41 findings without deferrals; production-grade default applied uniformly to LOW/NIT findings

**Date:** 2026-05-18
**Source:** D-483 M3 BC cascade pass-1 PO fix-burst codification
**Class:** Production-grade-default discipline + cross-cutting SDK API verification discipline

BC-5.39.007 + BC-5.39.008 pass-1 adversary cascade yielded 41 findings (21 + 20; F-BC008P1-001 FALSE POSITIVE not acted on = 40 actionable). PO fix-burst at `865062b5` closed all 40 actionable findings in a single commit without requesting any deferrals to the tech-debt register. This is the first pure PO fix-burst (no architect contention, no cross-subsystem entanglement) where the production-grade default was applied uniformly across all severity levels.

**Production-grade default on LOW/NIT findings:** Mechanical in-scope fixes — invariant-numbering contiguity verification, changelog section authoring, PC identifier cross-references in test vectors, capitalization standardization, and subsystem anchor confirmation — were all completed in-scope. No P4 "opportunistic cleanup" TDs were filed for work that constituted ~45-minute inline edits. This validates CLAUDE.md Canonical Principle Rule 3: a finding that could be a 1-line inline fix should not generate a TD register entry.

**Cross-cutting SDK API verification discipline:** F-BC007P1-009 + F-BC008P1-010 (both MEDIUM) were cross-cutting: `HookResult::Advisory` was referenced in both BCs but the variant does not exist in `crates/hook-sdk/src/result.rs` (only `Continue`/`Block`/`Error`). The PO correctly identified these as the same class and closed both by rewriting advisory behavior as `HookResult::Continue` + `host::log_warn`. Normative discipline: when a BC prescribes use of an SDK enum variant, the BC authorship burst MUST verify the variant exists in the actual SDK source before sealing. This is the same ground-truth verification discipline as INV-016-CANDIDATE (format claims must be verified against artifact corpus).

**No new META-LEVEL classes generated.** Unlike S-15.14 LOCAL cascade (6 META-LEVEL classes) and E-10 cascade (5 META-LEVEL classes), this PO-only BC cascade pass generated zero new META-LEVEL classes. The prior two META-LEVEL codifications (INV-015 adversary-must-grep-canonical-source + INV-016 BC-authorship-must-grep-actual-format) were already codified at D-482 persist. Pass-2 dispatch proceeds with streak = 0/3 and no pre-existing codification debt.

**Closes:** F-BC007P1-001 (CRITICAL Closes format) + F-BC007P1-002 through F-BC007P1-021 (HIGH/MED/LOW/NIT BC-007) + F-BC008P1-002 (CRITICAL PC13 ADR-021 contradiction) + F-BC008P1-003 through F-BC008P1-020 (HIGH/MED/LOW/NIT BC-008; F-BC008P1-001 FALSE POSITIVE excluded) + D-483 codified.

## L-M3-BC-cascade-pass-2-PO-fix-burst — M3 BC cascade pass-2 PO fix-burst — INV-017 discipline applied; 14/14 closed in scope including BC-5.39.006 v1.4 sibling-sweep

**Date:** 2026-05-19
**Source:** D-485 M3 BC cascade pass-2 PO fix-burst codification
**Class:** INV-017 shell-gate discipline application + BC sibling-sweep cross-cutting + production-grade-default enforcement

M3 BC cascade pass-2 PO fix-burst at `8c9b1200` closed all 14 retained findings from adv-bc-007-008-pass-2.md in a single commit without deferrals and without opening new TDs. This lesson records three key patterns that this burst demonstrates.

**INV-017 discipline applied — 6 literal-shell stdouts embedded in BC changelog rows.** INV-017 was codified at D-484 immediately before this burst as a META-LEVEL finding: "codified-discipline-must-be-applied-as-shell-gate-not-narrative-attestation." The PO fix-burst pass-2 is the first burst to apply INV-017 as a production gate. For each value-claim in each amended PC/invariant/EC/test-vector, the PO executed a literal grep against the canonical-source artifact and embedded captured stdout in the changelog row. Six such embedded stdouts appear in the BC-5.39.006/007/008 v1.4/v1.2/v1.2 changelog rows. Examples: (1) `grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` → `0` (post-sweep; confirms zero residual occurrences); (2) `grep -nE 'pub enum HookResult' crates/hook-sdk/src/result.rs` → confirms `Continue`, `Block { reason }`, `Error { message }` only (no `BlockWithFix` variant). This demonstrates that INV-017 can be satisfied: the discipline is operationalizable by the BC-authoring agent when dispatch templates enumerate which grep to run.

**BC-5.39.006 v1.3→v1.4 sibling-sweep: cross-cutting HookResult::block_with_fix canonical form.** The PO fix-burst pass-1 (D-483) correctly identified `HookResult::Advisory` as absent from the SDK and closed that class across BC-5.39.007 + BC-5.39.008. The sibling-sweep did NOT extend to BC-5.39.006 v1.3, which used `HookResult::BlockWithFix` 16 times — a different non-existent variant class. F-BC007P2-001 (HIGH) caught this gap at pass-2. The v1.4 sweep replaced all 16 occurrences with `HookResult::block_with_fix(...)` (the correct associated-function form). Normative discipline: when a BC-authoring sweep closes a "non-existent SDK API surface" class, the sweep MUST extend to ALL active sibling BCs that reference ANY SDK HookResult surface — not just the BCs under immediate amendment. A canonical list of HookResult API forms should be embedded in the BC template or checked via hook-sdk source.

**No new META-LEVEL classes generated at pass-2 PO fix-burst.** Unlike passes prior (INV-015 at D-482, INV-016+INV-017 at D-484), the pass-2 PO fix-burst produced no new META-LEVEL candidates. The INV-017 discipline was applied correctly: no BC version sealed without embedded grep stdout. No new unknown facts were claimed. No advisory-class return paths added without SDK verification. The cascade trajectory improving to pass-3 dispatch-ready with STREAK 0/3 (needing 3 consecutive CLEAN) continues the established monotonic trend.

**Closes:** F-BC007P2-001 (HIGH — BC-5.39.006 v1.3 BlockWithFix sibling-sweep → v1.4), F-BC007P2-002 (HIGH — Phase-1 false-negative window bounded), F-BC007P2-003 (HIGH — PC2/PC5 renumber propagation), F-BC007P2-004 (MEDIUM — 4-file arm-routing), F-BC007P2-005 (MEDIUM — EC-016/EC-018 cascade order), F-BC007P2-006 (MEDIUM — ADR-021 Open Sub-Questions traceability), F-BC007P2-007 (LOW — invariant 5 regex parenthetical), F-BC008P2-001 (CRITICAL — policies.yaml integer-id format), F-BC008P2-002 (CRITICAL — exec_subprocess SDK mis-claim), F-BC008P2-003 (HIGH — severity-enum self-contradiction), F-BC008P2-004 (MEDIUM — orphan PC2 scope paragraph), F-BC008P2-005 (MEDIUM — ADR-021 line 251 partial-sentence cite; renumbered from original F-BC008P2-006), F-BC008P2-007 (LOW — frontmatter phase stale), F-BC008P2-008 (LOW — ADR-021 Open Sub-Questions BC-008 perspective). D-485 codified.

## L-M3-BC-cascade-pass-2-INV-017-CANDIDATE — Codified-discipline-must-be-applied-as-shell-gate-not-narrative-attestation-during-fix-burst (META-LEVEL INV-017-CANDIDATE)

**Date:** 2026-05-18
**Source:** D-484 M3 BC cascade pass-2 state-manager persistence burst
**Class:** Meta-level discipline self-application failure + shell-gate-not-narrative-attestation pattern

M3 BC cascade pass-2 (2026-05-18) produced 14 findings including 2 verified CRITICAL that were not present at pass-1. Both CRITICAL findings are RE-INSTANCES of the META-LEVEL-INV-016 class ("BC-authorship-must-grep-actual-artifact-format") codified at D-482/D-483 in the immediately preceding bursts.

**Self-application failure pattern:** L-M3-BC-cascade-pass-1 + L-M3-BC-cascade-pass-1-PO-fix-burst codified INV-016-CANDIDATE ("BC spec format claims must be verified against actual artifact corpus via literal grep"). The PO fix-burst at 865062b5 that produced BC-5.39.007 v1.1 + BC-5.39.008 v1.1 then:

1. Authored invariant 4 + PC4 mandating POLICY \d{3} (three-digit form) for policies.yaml IDs without grepping actual policies.yaml which uses bare integer YAML values (id: 1 through id: 18). Result: F-BC008P2-001 VERIFIED CRITICAL. Hook built per spec would HARD BLOCK every write to the production file.

2. Authored PC10 stating host::exec_subprocess is NOT a registered host import without grepping crates/hook-sdk/src/host.rs which exposes pub fn exec_subprocess( at line 299. Result: F-BC008P2-002 VERIFIED CRITICAL. BC architectural justification for Option (b) layering cites a false factual claim about the SDK.

3. Failed to sweep BC-5.39.006 v1.3 for HookResult::BlockWithFix non-existence, parallel to the Advisory class just closed in BC-5.39.007/008. BC-5.39.006 v1.3 contains HookResult::BlockWithFix 16 times; the variant does not exist in result.rs. Result: F-BC007P2-001 VERIFIED HIGH sibling regression.

**The discipline was recorded but not operationalized.** The PO had access to the lesson (codified in the same burst that produced the v1.1 BCs). The lesson was in narrative form in lessons.md. Narrative lessons create awareness but not enforcement. The lesson becomes enforceable only when it is expressed as a concrete shell gate that the authoring agent must execute and embed as captured stdout in the changelog row before declaring the BC sealed.

**INV-017-CANDIDATE normative cure:** Every BC-authoring agent, for EVERY value-claim in EVERY PC/invariant/EC/test-vector, MUST:
- Execute a literal grep (or equivalent) against the canonical-source artifact
- Capture stdout
- Embed the captured stdout in the changelog row of the BC version being authored
- ONLY THEN seal the BC version

This transforms "you should check" (narrative discipline) into "you must produce verifiable evidence" (shell-gate discipline). A BC changelog row claiming a format or SDK fact without embedded grep stdout is an incomplete changelog row and a protocol violation under INV-017.

**Forward routing:** INV-017-CANDIDATE forwarded to SK-MCP-001 Appendix D. The cure belongs in orchestrator dispatch templates: for each PC/invariant/EC that makes a claim about an artifact's format or an SDK's API surface, the dispatch template must enumerate the specific literal grep required and require its stdout be embedded in the changelog row.

**Relationship to prior META-LEVELs:** INV-015 (adversary-must-grep-canonical-source) + INV-016 (BC-authorship-must-grep-actual-artifact-format) + INV-017 (codified-discipline-must-be-shell-gate-not-narrative) form a progression. Each is a deeper layer of the same failure class: the gap between knowing a rule and executing the mechanical verification that enforces it.

**Closes:** F-BC007P2-001 (HIGH sibling BC-5.39.006 v1.3 BlockWithFix regression) + F-BC007P2-002 (HIGH Phase-1 false-negative window) + F-BC007P2-003 (HIGH PC2/PC5 renumber propagation) + F-BC007P2-004 (MEDIUM 4-file arm-routing under-specified) + F-BC007P2-005 (MEDIUM EC-016 vs EC-018 cascade order undefined) + F-BC007P2-006 (MEDIUM ADR-021 Open Sub-Questions traceability anchor) + F-BC007P2-007 (LOW invariant 5 regex parenthetical) + F-BC008P2-001 (CRITICAL policies.yaml integer-id-vs-POLICY-d{3}) + F-BC008P2-002 (CRITICAL exec_subprocess SDK-source mis-claim) + F-BC008P2-003 (HIGH severity-enum self-contradiction) + F-BC008P2-004 (MEDIUM orphan PC2 scope clarification paragraph) + F-BC008P2-005 (MEDIUM ADR-021 line 251 partial-sentence cite) + F-BC008P2-007 (LOW frontmatter phase stale) + F-BC008P2-008 (LOW ADR-021 Open Sub-Questions BC-008 perspective) + F-BC008P2-009 (NITPICK changelog row length) + D-484 codified + INV-017-CANDIDATE forwarded SK-MCP-001 Appendix D.

## L-M3-BC-cascade-pass-3-INV-018-CANDIDATE — Narrow-pattern-vs-residual-class-sweep dual requirement (META-LEVEL INV-018-CANDIDATE)

**Date:** 2026-05-19
**Source:** D-486 M3 BC cascade pass-3 state-manager persistence burst
**Class:** Meta-level shell-gate structural insufficiency — narrow-pattern-only sweep creates false-green; residual-class sweep required

M3 BC cascade pass-3 (2026-05-19) produced 8 findings including 1 verified CRITICAL (F-BC006P3-001). The PO faithfully applied INV-017 at pass-2 — all 6 embedded stdouts re-execute and match their claimed results. F-BC006P3-001 arises from INV-017's structural insufficiency, not from INV-017 misapplication.

**The narrow-pattern gap:** The PO fix at v1.4 targeted the prefixed form `HookResult::BlockWithFix` (the exact string being replaced). The INV-017 evidence grep was:

```
grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md → 0
```

This returned 0 and the claim was accurate: zero occurrences of the fully-prefixed `HookResult::BlockWithFix` remain. However, the broader semantic class — bare `BlockWithFix` tokens without the `HookResult::` prefix — was not checked. The residual-class sweep would be:

```
grep -cE 'BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md → 28
```

28 bare `BlockWithFix` tokens survived in EC tables, VP tables, D-NNN Anchor Coverage text, and rationale paragraphs. TDD test authors following EC table rows would write `HookResult::BlockWithFix { ... }` which does not compile — the variant does not exist. The narrow-pattern evidence was accurate; the structural coverage of that evidence was insufficient.

**INV-018 normative cure:** Every fix-burst changelog row evidence block for a "replace pattern X with pattern Y" closure MUST include BOTH:

1. **Narrow-pattern evidence (INV-017 satisfied):** `grep -nE '<exact-replaced-pattern>' <target>` → expected zero output or explicit count

2. **Residual-class sweep (INV-018 required):** `grep -nE '<broader-semantic-class-pattern>' <target>` → expected zero output (or explicit residual-listing if non-zero is acceptable and documented)

The broader-semantic-class pattern captures all syntactic variants of the same semantic construct — for example, when sweeping an SDK enum variant reference, the narrow pattern is `HookResult::BlockWithFix` (exact form) and the residual-class pattern is `BlockWithFix` (any occurrence, with or without qualifier). A zero result on the residual-class sweep confirms there are no latent instances that could mislead test authors.

**Pattern structure:** INV-015 (adversary-must-grep-canonical-source) → INV-016 (BC-authorship-must-grep-actual-artifact-format) → INV-017 (codified-discipline-must-be-shell-gate-not-narrative) → INV-018 (shell-gate-must-cover-narrow-AND-residual-class-sweep). Each layer reveals a structural insufficiency in the prior layer's cure. INV-018 does not supersede INV-017 — it extends it. Both greps are required.

**Cascade trajectory:** pass-1 ~41 → pass-2 14 → pass-3 8. CRITICAL count declining (2 → 2 → 1). Each pass introduces one new META-LEVEL class (INV-016, INV-017, INV-018). The INV-N class discovered at each pass is a refinement of the same structural depth: the gap between "applying a discipline" and "verifying that the discipline's coverage was complete."

**Asymptotic-acceptance note (D-486(e)):** If the cascade approaches floor [1,3] findings without 3-CLEAN convergence by pass-5 or pass-6, D-386 Option C + D-477 precedent chain permits human asymptotic-acceptance authorization. Each pass's META-LEVEL discovery is a structural insight forwarded to SK-MCP-001 Appendix D; the forward routing preserves value regardless of whether 3-CLEAN is reached.

**Forward routing:** INV-018-CANDIDATE forwarded to SK-MCP-001 Appendix D. Dispatch templates for "replace X with Y" fix-bursts must enumerate BOTH the narrow-pattern grep AND the residual-class grep and require both results to be embedded as captured stdout in the changelog row.

**Closes:** F-BC006P3-001 (CRITICAL — BC-5.39.006 v1.4 sibling-sweep incomplete; 28 bare BlockWithFix residual), F-BC006P3-002 (MEDIUM — BC-5.39.006 v1.4 changelog typo: replace-target = replacement), F-BC006P3-NIT (NITPICK — BC-5.39.006 v1.4 frontmatter modified: array observation), F-BC007P3-001 (HIGH — BC-5.39.007 v1.2 D-NNN Anchor Coverage retired PC2/PC8 anchors), F-BC007P3-002 (MEDIUM — D-448(b) row specifically mis-anchored to retired PC1/PC2), F-BC008P3-001 (HIGH — BC-5.39.008 v1.2 POLICY 13/16 D-NNN Anchor Coverage mis-anchors PC3), F-BC008P3-002 (LOW — BC-5.39.008 PC4 [1, 999] over-specified without rationale), F-BC008P3-003 (LOW — cross-BC closure citation inconsistency). D-486 codified.


## L-M3-BC-cascade-pass-3-PO-fix-burst — M3 BC cascade pass-3 PO fix-burst: INV-018 dual-grep discipline applied; 8/8 closed in scope including BC-5.39.006 v1.4→v1.5 residual-class sweep

**Date:** 2026-05-19
**Context:** M3 COMMISSIONING 3M3a-r fix-burst pass-3; cycle v1.0-brownfield-backfill; BC-5.39.006 v1.5 + BC-5.39.007 v1.3 + BC-5.39.008 v1.3.

**INV-018 dual-grep application (institutional discipline established):**

The INV-018 discipline requires that every fix-burst changelog row for a "replace pattern X with pattern Y" closure MUST include BOTH a narrow-pattern grep AND a residual-class sweep grep, both with captured stdout:

1. **Narrow-pattern grep (INV-017 satisfied):** `grep -cE 'HookResult::BlockWithFix' BC-5.39.006.md` → `0` (prefixed form absent since v1.4 sibling-sweep). INV-017 was applied correctly at pass-2; the narrow pattern was zero before and after the v1.5 fix.

2. **Residual-class sweep (INV-018 required):** `grep -cE 'BlockWithFix' BC-5.39.006.md` → pre-fix `28`, post-fix `5`. The 5 post-fix residuals are all in POLICY-1-exempt historical changelog/evidence content (frontmatter v1.2 narrative + changelog rows v1.2/v1.3/v1.4 + v1.5 changelog row self-reference in evidence text). Spec body = 0 bare tokens.

**Why 28→5 reduction constitutes full closure of F-BC006P3-001:**

The 28 pre-fix bare `BlockWithFix` tokens in BC-5.39.006 v1.4 were distributed across: EC table Expected Behavior column (14 rows), VP table rows (6 rows), D-NNN Anchor Coverage row (1), postcondition 6 body text (1), invariant 6(a) body text (1), Traceability D-NNN Sub-Clauses Closed (1) = 24 non-historical tokens. All 24 were replaced with `HookResult::block_with_fix(...)` canonical associated-function form. The remaining 5 tokens reside in POLICY-1-exempt historical content that cannot be modified per POLICY 1 append-only changelog invariant. A TDD test author reading BC-5.39.006 v1.5 spec body will encounter zero bare `BlockWithFix` tokens — the SDK non-existent construct is fully absent from the normative sections.

**Production-grade upheld (no deferrals, no new TDs):**

All 8 pass-3 findings closed in scope: F-BC006P3-001 CRITICAL (28→5 residual with 5 POLICY-1-exempt); F-BC007P3-001 HIGH (D-NNN Anchor Coverage PC2/PC5 renumber propagated, retired PC1/PC2 corrected); F-BC008P3-001 HIGH (POLICY 13→postconditions 3/6; POLICY 16→postconditions 3/7); F-BC006P3-002 MEDIUM corrigendum (v1.4 self-referential changelog typo clarified in v1.5 without modifying v1.4 row); F-BC008P3-002 LOW ([1,999] range rationale anchored to three-digit display + max id=18 + governance budget); F-BC007P3-002 MEDIUM (subsumed by BC-5.39.007 v1.3 D-NNN Anchor Coverage fix); F-BC008P3-003 LOW (cross-BC closure relocation note for F-BC007P2-006 added); F-BC006P3-NIT (frontmatter array form confirmed as observation; no action required). No deferrals to TD register; no new TDs opened.

**Cascade trajectory:** pass-1 ~41 → pass-2 14 → pass-3 8 → pass-4 dispatch-ready. CRITICAL count: 2 → 2 → 1 → 0. HIGH count: ~17 → 4 → 2 → 0. INV-018 dual-grep confirmed applied to all 3 BC changelog rows in PO commit `50e03f82`. STREAK 0/3 → pass-4 adversary dispatch-ready.

**Forward routing:** INV-018-CANDIDATE confirmed as institutional discipline. Dispatch templates for "replace X with Y" fix-bursts must enumerate BOTH the narrow-pattern grep AND the residual-class grep and require BOTH results to be embedded as captured stdout in the changelog row evidence. Forward: SK-MCP-001 Appendix D INV-018 now CONFIRMED (discipline applied and verified).

**Closes:** F-BC006P3-001, F-BC006P3-002, F-BC006P3-NIT, F-BC007P3-001, F-BC007P3-002, F-BC008P3-001, F-BC008P3-002, F-BC008P3-003. D-487 codified.

## L-M3-BC-cascade-pass-4-INV-019-CANDIDATE — M3 BC cascade pass-4 detects META-LEVEL INV-019-CANDIDATE — changelog-row-self-reference accounting drift; CRITICAL+HIGH reach zero

**Date:** 2026-05-19
**Source:** D-488 M3 BC cascade pass-4 state-manager persistence burst
**Class:** Meta-level structural limitation — post-commit changelog-row-self-reference makes INV-018 evidence non-reproducible; CRITICAL+HIGH both zero for first time

M3 BC cascade pass-4 (2026-05-19) produced 3 findings: 1 MEDIUM + 1 LOW + 1 NITPICK. This is the first pass where CRITICAL = 0 and HIGH = 0. All spec-content defects (SDK API mis-claims, D-NNN Anchor Coverage mis-anchors, bare non-existent SDK constructs) have been fully resolved across 4 passes. The remaining 3 findings are all documentary / META-LEVEL evidence-quality concerns.

**MAJOR POSITIVE SIGNAL — CRITICAL+HIGH BOTH ZERO:**

Pass-4 marks the first time in the M3 BC cascade where neither CRITICAL nor HIGH findings appear. Cascade trajectory: ~41 → 14 → 8 → 3. CRITICAL progression: 2 → 2 → 1 → 0. HIGH progression: ~17 → 4 → 2 → 0. The spec content of BC-5.39.006 v1.5 + BC-5.39.007 v1.3 + BC-5.39.008 v1.3 is now correct — all SDK API claims verified against actual source, all D-NNN Anchor Coverage tables properly anchored, all postcondition references semantically correct. The cascade is genuinely converging.

**META-LEVEL INV-019-CANDIDATE — Changelog-row-self-reference evidence non-reproducibility:**

INV-018 was correctly invoked in the pass-3 PO fix-burst — all three BC changelog rows include both a narrow-pattern grep and a residual-class grep with embedded stdout. Pass-4 reveals two structural limitations in how INV-018 was applied:

1. **F-BC006P4-001 LOW (INV-019 self-reference):** BC-5.39.006 v1.5 changelog row's INV-018 residual evidence claims `grep -cE 'BlockWithFix' BC-5.39.006.md → 4`. Post-commit re-execution returns `5` — the v1.5 row itself quotes `BlockWithFix` in its evidence prose, contributing a 5th occurrence. The count was accurate at the time of writing (before the commit) but becomes wrong after the commit. This is a new structural class: the act of writing evidence that quotes the searched pattern into the changelog row invalidates the evidence.

2. **F-BC008P4-001 MEDIUM (INV-018 misapplication):** BC-5.39.008 v1.3 changelog row's INV-018 residual-class sweep uses pattern `PC3.*POLICY.POLICY.*PC3`, which is structurally NARROWER than the narrow pattern `POLICY 13.POLICY 16`. INV-018 requires the residual pattern to be genuinely BROADER (catching any syntactic variant of the same semantic class). A correct residual sweep would be `PC[0-9]+/PC[0-9]+` (any multi-PC anchor) or `\bPC[0-9]+\b.*POLICY` (any PC ordinal co-occurring with any POLICY citation).

**INV-019-CANDIDATE normative statement:** "Embedded post-fix literal-shell stdout becomes non-reproducible the instant the changelog row containing the evidence is committed, if the searched pattern appears verbatim in the changelog row's own evidence prose. The discipline must either (a) line-range-exclude the changelog row from the post-fix grep, (b) acknowledge self-reference inline ('post-fix count excluding this changelog row = N'), or (c) use a search pattern that the changelog row's prose cannot match by construction (e.g., grep for `^| EC-` table-row context that prose lacks)."

**Affected instances across all 3 BCs:**

- BC-5.39.006 v1.5 changelog row: quotes `BlockWithFix` in evidence — self-reference confirmed; count drifts from 4 to 5 post-commit. Lessons.md entry L-M3-BC-cascade-pass-3-PO-fix-burst acknowledges this for BC-006.
- BC-5.39.007 v1.3 changelog row: quotes PC-related patterns in evidence — self-reference risk present; not explicitly acknowledged in-place.
- BC-5.39.008 v1.3 changelog row: quotes `POLICY` and `PC` patterns in evidence — self-reference risk present; additionally the residual pattern is structurally narrower (F-BC008P4-001 MEDIUM).

Only BC-006 is acknowledged in lessons.md; BC-007/BC-008 are silently affected. The INV-019 cure should be applied to all three changelog rows in the PO fix-burst pass-4.

**Cure options (a)/(b)/(c):**

- **(a) Line-range-exclude:** Use `grep -v` or `sed -n '1,<line>p; <line+1>,$p'` to exclude the changelog row's line range from the grep target. Robust but requires knowing the line number, which is unstable.
- **(b) Inline acknowledge:** Add to the changelog row's evidence: "post-fix count excluding this changelog row = N." Straightforward; makes the drift visible without fixing it structurally.
- **(c) Pattern-by-construction:** Choose a search pattern that the changelog row's prose cannot match. For `BlockWithFix`, option (c) could be `^| .* HookResult::block_with_fix` (anchored to EC table `^| ` row starts) — changelog evidence prose does not start with `^| EC-` table-row markup. This is the most structurally robust option; the pattern is inherently immune to the self-reference class.

**META-LEVEL progression INV-015 through INV-019:**

INV-015 (adversary-must-grep-canonical-source) → INV-016 (BC-authorship-must-grep-actual-artifact-format) → INV-017 (codified-discipline-must-be-shell-gate-not-narrative) → INV-018 (shell-gate-must-cover-narrow-AND-residual-class-sweep) → INV-019-CANDIDATE (post-commit-self-reference-makes-evidence-non-reproducible). Each layer reveals a structural limitation in the prior layer's cure that only manifests after the prior cure is correctly applied.

The pattern: 4 META-LEVELs detected in 4 consecutive passes. Each emerges when the prior META-LEVEL's cure is faithfully applied — the application reveals the next structural depth. This is consistent with the D-386 Option C asymptotic-acceptance model: the META-LEVEL class discoveries are valuable forward-routing artifacts (SK-MCP-001 Appendix D INV-NNN seeds) regardless of whether 3-CLEAN convergence is reached.

**Forward routing:** INV-019-CANDIDATE forwarded to SK-MCP-001 Appendix D. Dispatch templates for changelog row evidence sections must incorporate one of the three cure options. Option (c) pattern-by-construction is recommended as the most structurally robust: anchoring evidence greps to `^| ` table-row context prevents changelog-row prose from being a false positive.

**Closes:** F-BC008P4-001, F-BC006P4-001, F-BC007P4-NIT. D-488 codified.

## L-M3-BC-cascade-pass-4-PO-fix-burst — M3 BC cascade pass-4 PO fix-burst: 3 documentary-only findings; INV-019 CANDIDATE→CONFIRMED; cascade genuinely converging

**Date:** 2026-05-19
**Source:** D-489 M3 BC cascade pass-4 PO fix-burst state-manager codification burst
**Class:** Cascade convergence signal; INV-019 class confirmation; cross-BC idiom standardization

**Symptom:** Pass-4 had 3 findings (1 MEDIUM + 1 LOW + 1 NIT) with CRITICAL+HIGH both zero. This is the first time in the M3 BC cascade that all spec-content defects are resolved and only documentary/META-LEVEL evidence-quality findings remain. This is proof-of-concept that the cascade is genuinely converging: the CRITICAL and HIGH severity classes (SDK API mis-claims, D-NNN Anchor Coverage mis-anchors, bare non-existent SDK constructs) have all been eliminated across 4 passes. The remaining findings are structural meta-discipline documentation issues, not substantive spec defects.

**Cause:** Two root causes converged at pass-4:

1. **INV-018 misapplication (F-BC008P4-001 MEDIUM):** BC-5.39.008 v1.3 PO fix-burst at pass-3 correctly invoked INV-018 discipline (included both narrow and residual-class greps) but chose a residual-class pattern `PC3.*POLICY.POLICY.*PC3` that is structurally NARROWER than the narrow pattern `POLICY 13.*PC3|POLICY 16.*PC3`. INV-018 requires the residual pattern to be BROADER — catching any occurrence of the semantic class, not only specific sequential combinations of tokens that were already in the narrow pattern.

2. **INV-019 self-reference drift (F-BC006P4-001 LOW):** BC-5.39.006 v1.5 changelog row correctly embedded INV-018 evidence with a count of `4`. But the changelog row itself quotes `BlockWithFix` in its evidence prose. The instant the row was committed, the count became `5` (the row itself contributes one occurrence). This is the INV-019-CANDIDATE class: post-commit self-reference makes the evidence non-reproducible. The count was accurate at write time but wrong post-commit.

3. **Cross-BC idiom inconsistency (F-BC007P4-NIT):** BC-5.39.006 used assoc-fn form `HookResult::block_with_fix(...)` while BC-5.39.007 and BC-5.39.008 used struct-pattern form `HookResult::Block { reason: ... }`. Both forms are semantically equivalent and reference real SDK constructs. This is a documentation style inconsistency only.

**Cure applied:**

- **F-BC008P4-001:** BC-5.39.008 v1.3→v1.4 — INV-018 residual-sweep pattern corrected to `PC[0-9]+/PC[0-9]+` (genuinely broader: catches any multi-PC anchor row). Changelog row evidence rewritten per INV-018 normative cure. INV-019 cure (a) applied to the evidence section.
- **F-BC006P4-001:** BC-5.39.006 v1.5→v1.6 — INV-019 cure (a) line-range-exclude applied to the changelog row evidence section. Self-reference accounting drift class documented in-place for BC-006 and noted for BC-007/BC-008 sibling rows.
- **F-BC007P4-NIT:** BC-5.39.007 v1.3→v1.4 — cross-BC idiom standardized on assoc-fn `HookResult::block_with_fix(...)` form per BC-006 precedent. Struct-pattern form deprecated as documentation style.

**INV-019 CONFIRMED (from CANDIDATE):** The class is confirmed with cure options codified:
- **(a) Line-range-exclude:** Exclude the changelog row's line range from the grep target (grep on a range that does not include the evidence row itself). Chosen for this burst.
- **(b) Inline-acknowledge:** Add "post-fix count excluding this changelog row = N" to the evidence. Explicit self-reference disclosure.
- **(c) Pattern-by-construction:** Use a search pattern that the changelog row's prose cannot match (e.g., anchor to `^| ` table-row context which prose lines do not have). Most structurally robust.

**Forward discipline:** ALL future BC changelog rows MUST apply one of INV-019 cures (a)/(b)/(c). Orchestrator/PO discretion per-row; document cure type chosen in the changelog row evidence section. This is MANDATORY going forward for all BC cascades.

**Cascade trajectory:**

| Pass | Total | CRIT | HIGH | MED | LOW | NIT |
|------|-------|------|------|-----|-----|-----|
| Pass-1 | ~41 | 2 | ~17 | ~10 | ~10 | ~2 |
| Pass-2 | 14 | 2 | 4 | 5 | 3 | 1 |
| Pass-3 | 8 | 1 | 2 | 2 | 2 | 1 |
| Pass-4 PO fix-burst | 3 (all closed) | 0 | 0 | 1 | 1 | 1 |

Monotonically decreasing. CRITICAL+HIGH reach zero at pass-4. The cascade is at documentary-only floor — subsequent passes should be CLEAN or NIT-only if INV-019 discipline is correctly applied.

**Cites:** D-489, INV-017, INV-018, INV-019, BC-5.39.006 v1.6, BC-5.39.007 v1.4, BC-5.39.008 v1.4, BC-006 (assoc-fn idiom precedent).

**Closes:** F-BC008P4-001, F-BC006P4-001, F-BC007P4-NIT. D-489 codified.

---

- [L-M3-BC-cascade-pass-5](#l-m3-bc-cascade-pass-5) — INV-020 CONFIRMED: 3-leg KK-N parity is insufficient; 5-leg quintuple parity required (last_amended: text-prefix + upstream-index body-table cells must sync same-burst). D-490 codified.

### L-M3-BC-cascade-pass-5

**Lesson ID:** L-M3-BC-cascade-pass-5
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-5 adversary (D-490)

**Symptom:** Pass-5 verdict HIGH (2 HIGH cross-file propagation + 3 LOW). STREAK 0/3 RESET. INV-019 RECURRENCE confirmed. Trajectory 41→14→8→3→5 (slight uptick). CRIT=0 sustained.

**Cause:** PO commit `f3cc03fc` (pass-4 PO fix-burst) bumped version: frontmatter + body Changelog row + modified[] correctly (3-leg KK-N per POLICY 14 v1) but missed two legs: (1) last_amended: text-prefix (leg-4) still showed the prior version's text prefix in all 3 BCs; (2) BC-INDEX body-table cells (leg-5) at lines 1231-1233 still showed stale v1.5/v1.3/v1.3 despite the BC-INDEX v2.41 changelog row in the same commit stating the bumps. Additionally, INV-019 cure (a) was applied correctly to the load-bearing grep in BC-006 v1.6 changelog row but the side-narrative enumeration "5 remaining tokens" was not updated — INV-019 RECURRENCE in the very fix-burst that confirmed INV-019.

**Diagnosis:** 3-leg KK-N parity (POLICY 14 v1 — codified D-468, extended NN-N D-468) is INSUFFICIENT to cover all same-burst propagation legs. The original POLICY 14 definition named only `version:`, `body Changelog row`, and `modified[]` as the tripartite. Two additional legs exist but were not gated: leg-4 (`last_amended:` text-prefix, which must cite the new version in its parenthetical) and leg-5 (upstream-index body-table cells that cite the bumped artifact's version in their version column). INV-019 RECURRENCE further demonstrates that INV-019 cure (a) must be applied to ALL literal counts in a row, not only the load-bearing grep.

**Cure:** INV-020 codified (D-490): extend POLICY 14 from tripartite to quintuple 5-leg parity. All 5 legs MUST sync in the same burst:
1. `version:` frontmatter — new version string
2. Body Changelog row — new row with new version label + date
3. `modified:` frontmatter array — new entry corresponding to new version
4. `last_amended:` text-prefix — parenthetical version in the string value (e.g., `"2026-05-20 (v1.7) — ..."`)
5. Upstream-index body-table version column cells (BC-INDEX for BC bumps; STORY-INDEX for story bumps; ARCH-INDEX for architecture bumps)

POLICY 14 description and verification_steps updated in policies.yaml same-burst per INV-020.

**Forward discipline:** All BC/VP/story/epic/architecture version bumps from this point MUST sync all 5 legs same-burst. The PO and state-manager checklist MUST include legs 4 and 5 explicitly. The adversary verification rubric must check all 5 legs per updated POLICY 14 verification_steps.

**Cites:** D-490, INV-019 RECURRENCE, INV-020 CONFIRMED, F-BC006P5-001, F-BC006P5-002, F-BC006P5-003, F-BC007P5-001, F-BC006P5-004, POLICY 14.

**Closes:** adv-bc-007-008-pass-5 persistence cycle; pass-5 verdict acknowledged; META-LEVEL INV-019 RECURRENCE + INV-020 CONFIRMED. Finding closures DEFERRED to D-491 PO fix-burst pass-5.

---

- [L-M3-BC-cascade-pass-5-PO-fix-burst](#l-m3-bc-cascade-pass-5-po-fix-burst) — POLICY 14 5-leg quintuple parity validated in production (PO commit `c4be5fde`); F-BC007P5-001 full BC-006-parity sweep ~46 conversions; INV-019 cure-type-per-row mix confirmed. D-491 codified.

### L-M3-BC-cascade-pass-5-PO-fix-burst

**Lesson ID:** L-M3-BC-cascade-pass-5-PO-fix-burst
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-5 PO fix-burst (D-491)

**Symptom:** Pass-5 had 5 findings (2H+3L) — verdict HIGH from cross-file propagation gaps. Root: POLICY 14 v1 (3-leg KK-N tripartite parity codified at D-468) was insufficient; it gated only `version:` frontmatter + body Changelog row + `modified[]` array. Two additional legs were not gated: `last_amended:` text-prefix (leg-4) and upstream-index body-table version cells (leg-5). A single PO commit (`f3cc03fc`) propagated legs 1/2/3 correctly but missed legs 4 and 5, producing 2 HIGH findings (F-BC006P5-001 + F-BC006P5-002) that reset the STREAK from 0/3 to 0/3 again.

**Cause:** PO commit `f3cc03fc` (pass-4 fix-burst) applied 3-leg KK-N parity per POLICY 14 v1 but missed leg-4 (`last_amended:` text-prefix still showed prior version's parenthetical across all 3 BCs) and leg-5 (BC-INDEX body table cells lines 1231-1233 still showed stale v1.5/v1.3/v1.3 despite the v2.41 changelog row in the same commit claiming bumps). Additionally INV-019 cure (a) was applied only to the load-bearing grep in BC-006 v1.6 but the side-narrative enumeration "5 remaining tokens" was not updated — INV-019 RECURRENCE in the very fix-burst that confirmed INV-019.

**Cure applied in PO commit `c4be5fde`:**

- **F-BC006P5-002 HIGH:** BC-006 v1.6→v1.7 — `last_amended:` text-prefix updated to `(v1.7)`; all 3 BCs updated leg-4.
- **F-BC006P5-003 LOW:** BC-006 v1.7 — INV-019 cure (b) inline-acknowledge applied to side-narrative enumeration in changelog row. INV-019 cure-type-per-row mix demonstrated (cure (a) on load-bearing grep; cure (b) on side-narrative).
- **F-BC006P5-004 LOW:** `timestamp:` field refreshed to 2026-05-20 on all 3 BCs (BC-006/007/008) — documentary-only but 5-leg parity extends to all frontmatter fields updated in same-burst.
- **F-BC007P5-001 LOW (full BC-006-parity sweep):** BC-007 v1.4→v1.5 + BC-008 v1.4→v1.5 — orchestrator adjudication: FULL parity sweep per production-grade default (CLAUDE.md Rule 4). ~23 BC-007 + ~22 BC-008 = ~46 total bare `HookResult::Block` → `HookResult::block_with_fix(...)` assoc-fn conversions. Remaining raw=1 per BC are POLICY-1-exempt historical. Cross-BC idiom consistency now fully aligned with BC-006 precedent.
- **Leg-5 (BC-INDEX body table):** BC-INDEX v2.42→v2.43 — body table cells lines 1235-1237 propagated v1.7/v1.5/v1.5 per INV-020 leg-5.

**POLICY 14 5-leg quintuple parity validated in production:** PO commit `c4be5fde` literally-shell-verified all 5 legs synced same-burst for all 3 BCs. INV-020 codification practical viability confirmed. No leg missed on first production application.

**INV-019 cure-type-per-row mix-and-match confirmed operational:** BC-006 v1.7 uses cure (b) on side-narrative; BC-007/008 v1.5 uses cure (c) by-construction (new changelog rows do not embed reproducibility-sensitive counts); BC-INDEX v2.43 uses cure (c). PO/orchestrator discretion per-row is the correct policy — mandating uniform cure type would cause unnecessary rewrites.

**Cascade trajectory update:** 41→14→8→3→5. CRITICAL=0 sustained across all 5 passes. HIGH reverted to 0 at pass-4 (CRITICAL+HIGH=0 first time); pass-5 HIGH was cross-file propagation gap class (POLICY 14 extension root cause), not spec-content regression. Pass-6 expected CLEAN if 5-leg parity discipline holds.

**Forward discipline:** 5-leg parity (POLICY 14 v2) is now the standard for ALL BC/VP/story/epic/architecture version bumps. F-BC007P5-001 establishes precedent that "cross-BC idiom alignment" intent defaults to FULL parity sweep (not narrow scope) per production-grade default. Subsequent PO fix-bursts and state-manager codification bursts MUST verify all 5 legs before declaring same-burst parity complete.

**Cites:** D-491, INV-019, INV-020, POLICY 14 (5-leg extended D-490, validated D-491), BC-006 v1.7, BC-007 v1.5, BC-008 v1.5, BC-INDEX v2.43, ~46 bare→assoc-fn conversions, F-BC006P5-002, F-BC006P5-003, F-BC006P5-004, F-BC007P5-001.

**Closes:** F-BC006P5-002, F-BC006P5-003, F-BC006P5-004, F-BC007P5-001. D-491 codified.

---

- [L-M3-BC-cascade-pass-6](#l-m3-bc-cascade-pass-6) — Pass-6 NITPICK verdict (2 documentary INV-019 RESIDUAL findings); STREAK 0/3 → 1/3 FIRST ADVANCE in cascade; POLICY 14 5-leg parity production-validated; F-BC007P5-001 full sweep adversary-verified; NO PO fix-burst required. D-492 codified.

### L-M3-BC-cascade-pass-6

**Lesson ID:** L-M3-BC-cascade-pass-6
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-6 adversary (D-492)

**Symptom:** Pass-6 verdict NITPICK (0 CRIT / 0 HIGH / 0 MED / 0 LOW / 2 NIT). STREAK 0/3 → 1/3 FIRST ADVANCE IN THE 6-PASS CASCADE. Cascade trajectory 41→14→8→3→5→2 NIT (steep decay restored; HIGH=0 RESTORED).

**Significance:** The first streak advance in the cascade demonstrates BC-5.39.001 3-CLEAN protocol working as designed — NITPICK advances without resetting (MED+ would reset). The cascade has been stuck at STREAK 0/3 for 5 consecutive passes due to CRITICAL/HIGH findings; pass-6 is the first pass where the cascade advances mechanically. Two more consecutive CLEAN or NITPICK passes produce 3-CLEAN convergence and unblock 3M3b story elaboration.

**POLICY 14 5-leg parity production-validated:** PO commit `c4be5fde` (pass-5 fix-burst) applied all 5 legs of POLICY 14 (v2; 5-leg quintuple parity per D-490) correctly for all 3 BCs. Pass-6 adversary confirmed NO regression — no new leg-4 or leg-5 violations detected. INV-020 codification (D-490) is practically viable: a single PO commit can correctly apply all 5 legs simultaneously when the checklist enumerates all 5 legs explicitly. The risk of missing a leg drops to NITPICK/none when POLICY 14 is applied rigorously.

**F-BC007P5-001 full BC-006-parity sweep adversary-verified:** ~46 bare `HookResult::Block` → `HookResult::block_with_fix(...)` conversions across BC-007/008 body tables (Edge Cases + Test Vectors) sampled at 10+ sites by pass-6 adversary. NO conversion defects found. Semantic preservation confirmed: no accidental Block→Continue flips; complex multi-clause semantics (EC-022 multi-advisory) preserved intact. This validates the D-489 cross-BC idiom standardization and the D-491 production-grade full-sweep approach.

**Both findings are INV-019 RESIDUAL:** F-BC006P6-001 (row-number drift in BC-INDEX changelog: "rows 1233-1235" should be "rows 1235-1237") and F-BC007P6-001 (cross-SoT count inconsistency: exact-form in BC-007/008 body vs approximation-form in D-491/BC-INDEX/lessons). Both documentary-only; no load-bearing impact. INV-019 forward-application discipline gap — cure (c) by-construction (uniform ~tilde approximation) would have prevented F-BC007P6-001 by ensuring approximation form in ALL 5 SoTs from the start.

**NIT findings do NOT require PO fix-burst:** BC-5.39.001 3-CLEAN protocol is clear — NITPICK advances the streak without requiring a fix-burst. Documentary cleanup is deferred per POLICY 1 append-only: the finding is ACKNOWLEDGED but NOT actioned immediately. This is the intended behavior for truly documentary-only issues where correction would require a BC-INDEX changelog bump (which POLICY 1 restricts to append-only at the next genuine version bump). Adversary Part B Recommendation #2 honored with explicit deferral cite.

**Forward discipline:** For cross-artifact count narratives, adopt uniform cure (c) by-construction — use approximation-tilde (~N) in ALL SoTs when a count may drift post-commit, rather than mixing exact-form in one SoT and approximation-form in others. This eliminates F-BC007P6-001 class findings by construction. POLICY 1 append-only does not prevent this — tilde form is simply written into each new changelog row from the start.

**Cites:** D-492, INV-019 RESIDUAL, POLICY 14 production-validated (D-490/D-491), BC-5.39.001 streak protocol confirmed, F-BC006P6-001, F-BC007P6-001.

**Closes:** adv-bc-007-008-pass-6 persistence cycle (STREAK 1/3 advance). D-492 codified.

---

- [L-M3-BC-cascade-pass-7](#l-m3-bc-cascade-pass-7) — Pass-7 NITPICK verdict (1 finding F-BC007P7-001 INV-019 RESIDUAL meta-meta recursion); STREAK 1/3 → 2/3 SECOND ADVANCE; INV-019 cure (c) extended scope to persisted adversary reports; D-492 codification artifacts adversary-verified clean. D-493 codified.

### L-M3-BC-cascade-pass-7

**Lesson ID:** L-M3-BC-cascade-pass-7
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-7 adversary (D-493)

**Symptom:** Pass-7 verdict NITPICK (0 CRIT / 0 HIGH / 0 MED / 0 LOW / 1 NIT). STREAK 1/3 → 2/3 SECOND CONSECUTIVE STREAK ADVANCE. Cascade trajectory 41→14→8→3→5→2 NIT→1 NIT (continued steep decay; single finding; one pass from convergence).

**Significance:** The second consecutive streak advance confirms BC-5.39.001 3-CLEAN protocol is progressing correctly. The cascade is now at 2/3 — one more CLEAN or NITPICK pass produces 3/3 CONVERGED, which unblocks 3M3b story elaboration for S-15.10/12/13/15/16-Part-B. CRITICAL=0 sustained 6 passes; HIGH=0 sustained 2 passes — the cascade has reached a genuinely clean state at the spec content level.

**META-LEVEL discovery — INV-019 RESIDUAL RECURSION at meta-meta level:** F-BC007P7-001 reveals that adversary persisted reports are themselves subject to the same INV-019 drift class they document. Pass-6's F-BC006P6-001 evidence block cited hardcoded row numbers (1235/1236/1237) for BC-INDEX body-table rows. After D-492 added the v2.44 changelog row to BC-INDEX, those row numbers shifted — exactly as INV-019 predicts for any hardcoded line-number citation. The report documenting the drift has itself drifted. This is INV-019 at the meta-meta level (not a new INV class — same mechanism, deeper recursion).

**Cure — INV-019 cure (c) by-construction in persisted adversary reports:** This pass-7 file demonstrates the cure: the F-BC007P7-001 evidence section uses grep pattern `^\| \[BC-5\.39\.00[678]\]` rather than hardcoded line numbers. The grep pattern is self-updating — it locates the current row regardless of file position changes. Forward discipline: all persisted adversary reports must apply cure (c) by-construction in evidence sections that cite BC-INDEX body-table row positions, changelog row positions, or any other position that may shift post-commit. This extends INV-019 codification (D-489) from changelog rows to persisted adversary reports.

**D-492 codification adversary-verified clean:** The pass-7 adversary confirmed that state-manager applied cure (c) by-construction in BC-INDEX v2.44 (the lesson learned from F-BC006P6-001 at pass-6). The v2.44 changelog row prose contains no hardcoded line numbers. All 4 index bumps synchronized (D-001..D-492). Burst-log h2 entry has all 8 D-444(c) blocks with literal-shell Dim-2 gates per D-449(a). STATE.md frontmatter satisfies all BC-5.39.006 v1.7 PCs. This demonstrates the discipline loop working: adversary finds class → state-manager applies cure → next adversary confirms cure applied → no recurrence.

**Pass-6 deferred findings outcome validated:** F-BC006P6-001 did NOT recur in BC-INDEX v2.44. F-BC007P6-001 did NOT recur in D-492 codification artifacts. POLICY 1 append-only deferral was correctly applied — root-cause prevention (cure (c) by-construction going forward) is more valuable than fixing already-committed immutable records.

**Forward discipline:** All future persisted adversary reports must apply INV-019 cure (c) by-construction in evidence sections: use grep patterns rather than hardcoded line numbers when citing BC-INDEX body-table or changelog row positions. BC-INDEX v2.45 row prose should optionally cite F-BC007P7-001 as forward-application of the discipline.

**Cites:** D-493, INV-019 extended scope (persisted reports; D-489 changelog-rows scope extended), BC-5.39.001 streak protocol second advance confirmed, POLICY 1 append-only deferral validated, POLICY 14 5-leg parity sustained (no regression at pass-7), F-BC007P7-001.

**Closes:** adv-bc-007-008-pass-7 persistence cycle (STREAK 2/3 advance). D-493 codified.

**Closes:** adv-bc-007-008-pass-6 persistence cycle (STREAK 1/3 advance). D-492 codified.

- [L-M3-BC-cascade-pass-8] **Pass-8 verdict HIGH 1 finding (INV-020 RECURRENCE at 4-index codifying-burst level); STREAK 2/3 → 0/3 RESET; cascade prolonged.**
  - **Symptom:** Adversary pass-8 returned verdict HIGH with 1 finding F-BC008P8-001: BC-INDEX v2.45 `last_amended:` text-prefix stale at "(v2.44)" while `version: "2.45"`. D-493 codification burst updated BC-INDEX leg-1 (version:) and leg-2 (changelog row) but missed leg-4 (last_amended: text-prefix) on BC-INDEX itself, while correctly syncing the same leg-4 on VP-INDEX, STORY-INDEX, and ARCH-INDEX. STREAK 2/3 → 0/3 RESET; convergence postponed.
  - **Cause:** INV-020 RECURRENCE at the meta-level: the D-493 codification burst that documented the POLICY 14 5-leg cure applied it to BC bodies + 3 of 4 indexes but failed to self-apply leg-4 to BC-INDEX, the index being bumped in that very burst. This is the same class as F-BC006P5-002 (pass-5 HIGH; systematic 3-of-3 on BC bodies) now manifesting at the 4-index level as a singleton miss.
  - **Cure:** D-494 burst (a) closes F-BC008P8-001 by bumping BC-INDEX v2.45→v2.46 with proper leg-4 sync, (b) extends POLICY 14 verification_steps with a literal-shell 4-index self-application gate template (new 7th step; extended_at updated D-490→D-494), (c) requires forward state-manager codification bursts to run the gate before commit.
  - **Validation:** This D-494 burst itself ran the literal-shell gate post-fix; all 4 indexes returned PASS: BC-INDEX.md version=2.46 last_amended_prefix=2.46; VP-INDEX.md version=2.03 last_amended_prefix=2.03; STORY-INDEX.md version=3.50 last_amended_prefix=3.50; ARCH-INDEX.md version=2.12 last_amended_prefix=2.12. Demonstrates gate is operational.
  - **Forward discipline:** NO codification burst commit that bumps any of the 4 indexes without first running the literal-shell 4-index leg-4 gate and verifying zero FAIL output. Gate is a POLICY 14 verification_step (extended_at D-494).
  - **Cites:** D-494, INV-020 RECURRENCE at 4-index codifying-burst level, POLICY 14 verification_steps extended (extended_at D-494), BC-5.39.001 STREAK 0/3 RESET (HIGH resets streak).

**Closes:** adv-bc-007-008-pass-8 persistence + fix cycle. D-494 codified.

- [L-M3-BC-cascade-pass-9] **Pass-9 verdict CLEAN (0 findings) — FIRST TRUE CLEAN of the 9-pass cascade; STREAK 0/3 → 1/3 FIRST ADVANCE POST-RESET.**

**Symptom:** Adversary pass-9 returned verdict CLEAN with 0 findings (0 CRIT / 0 HIGH / 0 MED / 0 LOW / 0 NIT). First TRUE CLEAN of the 9-pass M3 BC cascade. STREAK 0/3 → 1/3 FIRST ADVANCE POST-RESET per BC-5.39.001. CRITICAL=0 sustained 8 passes; HIGH=0 RESTORED at pass-9. Cascade trajectory 41→14→8→3→5→2 NIT→1 NIT→1 HIGH→0 CLEAN.

**Significance:** The first TRUE CLEAN confirms that D-494 POLICY 14 extension is operational and effective. The literal-shell 4-index self-application gate (POLICY 14 verification_step 7; codified D-494) was independently executed by the adversary at pass-9 review — all 4 indexes returned PASS (BC v2.46 LA=2.46; VP v2.03 LA=2.03; STORY v3.50 LA=3.50; ARCH v2.12 LA=2.12). The cure-extension parsimony is validated: no new INV-021 abstraction is needed. The same INV-020 cure class (POLICY 14 5-leg parity + literal-shell gate) closes the recurrence at the meta-level.

**Validation:** Adversary independently executed the 4-index self-application gate. All 4 PASS. D-494 codification artifacts verified clean (POLICY 14 `extended_at: D-494` present; burst-log 8 D-444(c) blocks with literal-shell Dim-2; STATE.md frontmatter satisfies all 5 BC-5.39.006 v1.7 PCs; lessons.md L-M3-BC-cascade-pass-8 factually accurate; INDEX.md pass-8 row 5-col compliant). INV-019 cure (c) by-construction discipline holding — no regression detected.

**Forward discipline:** Continue cure (c) by-construction in all persisted adversary reports. Continue literal-shell 4-index self-application gate on every state-manager codification burst that bumps any of the 4 indexes. BC-5.39.001 streak protocol confirmed working: two more CLEAN/NITPICK passes advance to 2/3 then 3/3 CONVERGED → unblocks 3M3b story elaboration.

**Cites:** D-495, POLICY 14 verification_step 7 empirically validated, BC-5.39.001 streak protocol STREAK 0/3 → 1/3 FIRST ADVANCE POST-RESET confirmed, INV-020 cure-extension parsimony validated (no INV-021 needed), cascade trajectory complete-to-date.

**Closes:** adv-bc-007-008-pass-9 persistence cycle (STREAK 1/3 first advance post-RESET). D-495 codified.

- [L-M3-BC-cascade-pass-10](#l-m3-bc-cascade-pass-10) — Pass-10 CLEAN verdict (0 findings; SECOND consecutive TRUE CLEAN); STREAK 1/3 → 2/3 SECOND ADVANCE; cure-extension parsimony validated 2 consecutive passes (no INV-021 needed). D-496 codified.

### L-M3-BC-cascade-pass-10

**Lesson ID:** L-M3-BC-cascade-pass-10
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-10 adversary (D-496)

**Symptom:** Pass-10 verdict CLEAN (0 CRIT / 0 HIGH / 0 MED / 0 LOW / 0 NIT). STREAK 1/3 → 2/3 SECOND CONSECUTIVE ADVANCE. Cascade trajectory 41→14→8→3→5→2 NIT→1 NIT→1 HIGH→0 CLEAN→0 CLEAN (two consecutive zeros). CRITICAL=0 sustained 9 consecutive passes; HIGH=0 sustained 2 consecutive passes.

**Significance:** The second consecutive TRUE CLEAN confirms the cascade is one pass from BC-5.39.001 3-CLEAN convergence. STREAK 2/3 SECOND ADVANCE means one more CLEAN or NITPICK pass produces 3/3 CONVERGED at projected D-497, which unblocks 3M3b story elaboration for S-15.10/12/13/15/16-Part-B. This is the first time in the 10-pass M3 BC cascade that two consecutive zeros have been observed.

**Cure-extension parsimony VALIDATED 2 consecutive passes:** Pass-8 raised potential INV-021-CANDIDATE (suggesting a new abstraction class might be needed). D-494 correctly absorbed it as INV-020 RECURRENCE with POLICY 14 extension rather than introducing a new INV class. Pass-9 confirmed no INV-021 needed. Pass-10 ALSO confirms no INV-021 needed. Two consecutive CLEAN passes with the same cure discipline demonstrates that parsimony was the right decision — new INV abstraction would have been premature. The existing cure set (INV-019 cure (c) by-construction + POLICY 14 5-leg parity + literal-shell 4-index gate) is sufficient for convergence.

**POLICY 14 verification_step 7 operational 2 consecutive codification bursts:** The literal-shell 4-index self-application gate (codified at D-494, extended_at D-494) was adversary-executed at both pass-9 and pass-10 reviews. Both times all 4 indexes returned PASS. This demonstrates the gate is both operational and stable — it is not a one-time artifact of the codifying burst that introduced it, but a durable discipline. The extension is production-grade.

**D-495 codification adversary-verified clean:** Pass-10 adversary independently confirmed that adv-bc-007-008-pass-9.md was persisted correctly (verdict CLEAN; streak "1/3"; cure (c) by-construction throughout). All 4 index bumps synchronized D-001..D-495. Burst-log D-495 h2 entry had all 8 D-444(c) blocks with literal-shell Dim-2 gates per D-449(a). STATE.md frontmatter satisfies all BC-5.39.006 v1.7 PCs. L-M3-BC-cascade-pass-9 lesson factually accurate. The discipline loop is closing correctly at each burst.

**Forward discipline:** Continue cure (c) by-construction in all persisted adversary reports. Continue literal-shell 4-index self-application gate on every state-manager codification burst that bumps any of the 4 indexes. Continue 5-leg parity on all BC/VP/story/epic/architecture version bumps. No INV-021 abstraction warranted. BC-5.39.001 streak protocol: one more CLEAN/NITPICK produces 3/3 CONVERGED → unblocks 3M3b.

**Cites:** D-496, BC-5.39.001 STREAK 1/3 → 2/3 SECOND ADVANCE confirmed, parsimony validated 2 consecutive passes (no INV-021), POLICY 14 verification_step 7 sustained operational 2 passes, cascade trajectory complete-to-date.

**Closes:** adv-bc-007-008-pass-10 persistence cycle (STREAK 2/3 SECOND ADVANCE). D-496 codified.

- [L-M3-BC-cascade-CONVERGED](#l-m3-bc-cascade-converged) — MILESTONE: M3 3M3a-r 3-CLEAN convergence achieved at pass-11 (11-pass cascade; trajectory 41→14→8→3→5→2 NIT→1 NIT→1 HIGH→0→0→0; STREAK 3/3; D-497 declared CONVERGENCE; S-7.02 cycle-closing checklist satisfied; META-LEVEL evolution INV-017→018→019→020→POLICY 14 5-leg+gate codified into engine; unblocks 3M3b story elaboration).

### L-M3-BC-cascade-CONVERGED

**Lesson ID:** L-M3-BC-cascade-CONVERGED
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Pass:** M3 BC cascade pass-11 — CONVERGENCE DECLARATION (D-497)

**Achievement:** M3 3M3a-r BC cascade (BC-5.39.006 + BC-5.39.007 + BC-5.39.008) achieved 3-CLEAN convergence at pass-11. BC-5.39.001 3-CLEAN threshold SATISFIED. Cascade trajectory: 41→14→8→3→5→2 NIT→1 NIT→1 HIGH→0→0→0 (three consecutive zeros culminating in CONVERGED declaration). 11 passes; 2 PO fix-bursts; 8 state-manager codification bursts (D-487..D-496); D-497 declares convergence.

**Cascade trajectory narrative:** The cascade opened with 41 findings at pass-1 (major structural defects in two brand-new BC drafts) and closed at zero after 11 passes with systematic remediation at every severity level. The trajectory demonstrates the 3-CLEAN protocol working as designed: streak-reset-on-MED+ at pass-8 (HIGH INV-020 RECURRENCE) correctly prevented false convergence when the cure's own self-application gap was discovered. The 3 passes post-reset (pass-9/10/11) provided genuine empirical validation of the fixed cure discipline.

**META-LEVEL evolution narrative:** The cascade produced 5 codified INV classes plus 2 recurrence codifications:
- **INV-017 (pass-2; D-485):** narrow-vs-residual dual-grep discipline — adversary narrow grep missed residual class of same finding; forward rule: every grep search MUST cover both narrow occurrence AND residual class via separate grep invocations
- **INV-018 (pass-3; D-487):** residual STRUCTURALLY BROADER than narrow class — when INV-017 dual-grep finds residual occurrences, the residual may be a different structural class requiring broader treatment; forward rule: check whether residual grep pattern covers MORE semantic territory than narrow pattern
- **INV-019 (pass-4→CONFIRMED D-489; cures a/b/c):** changelog self-reference drift — evidence blocks citing line numbers or counts in the document being modified become self-referentially stale post-commit; three cures: (a) grep anchors on load-bearing content, (b) inline-acknowledge on side-narrative, (c) by-construction avoidance
- **INV-020 (pass-5→CONFIRMED D-490; POLICY 14 → 5-leg quintuple parity):** same-burst KK-N parity covers only 3 of 5 propagation legs; extended POLICY 14 to 5-leg quintuple parity: (1) version: frontmatter, (2) body Changelog row, (3) modified[] array, (4) last_amended: text-prefix, (5) upstream-index body-table cells
- **INV-020 RECURRENCE (pass-8; D-494; POLICY 14 verification_step 7):** 4-index codifying-burst self-application gap — the codification burst documenting the 5-leg cure applied it to BC bodies + 3 of 4 indexes but missed leg-4 on BC-INDEX itself; cure: literal-shell 4-index self-application gate (POLICY 14 verification_step 7) required before any commit that bumps any of the 4 indexes
- **INV-019 RESIDUAL (pass-7; D-493):** cure (c) by-construction scope extended from changelog rows to persisted adversary reports — persisted reports are themselves subject to INV-019 drift when citing line numbers in documents that grow post-commit

**Cure-extension parsimony principle CONFIRMED:** When a META-LEVEL recurrence is structurally the same class as a prior INV, EXTEND the cure rather than introduce a new INV-N abstraction. Pass-8 raised INV-021-CANDIDATE; correct decision was to absorb as INV-020 RECURRENCE with POLICY 14 extension. Pass-9, pass-10, and pass-11 all confirmed: NO INV-021 needed. The 3-CLEAN protocol empirically validated the parsimony decision — three consecutive zeros with the existing cure set is definitive proof the abstraction layer was sufficient. Future cascades should apply this principle: new INV number warrants only a genuinely novel mechanism class, not a recurrence of an existing mechanism.

**3-CLEAN protocol works as designed:** The streak-reset-on-MED+ mechanism caught a real META-LEVEL recurrence at pass-8 (INV-020 RECURRENCE; POLICY 14 leg-4 self-application gap). Without the reset, the cascade would have declared convergence at STREAK 2/3 pass-7 despite an unfixed discipline gap. The protocol correctly required three consecutive true CLEANs post-fix, providing genuine empirical validation that the cure discipline is operational and stable. The protocol's gating purpose — surface the cures' own self-application gaps — was fulfilled.

**Forward-applicable cure set codified into engine:**
- INV-019 cure (c) by-construction: use grep patterns not hardcoded line numbers in changelog rows AND persisted reports (D-489 + D-493)
- POLICY 14 5-leg quintuple parity: all 5 legs must sync same-burst for BC/VP/story/epic/arch version bumps (D-490)
- POLICY 14 verification_step 7: literal-shell 4-index self-application gate before any state-manager codification commit that bumps any of the 4 indexes (D-494); gate template in policies.yaml
- INV-017 dual-grep: adversary narrow grep + residual-class grep both required when checking for occurrence patterns (D-485)
- INV-018 structural-breadth check: verify residual grep covers broader territory than narrow grep, not just more occurrences (D-487)

**Unblocks:** 3M3b story elaboration for 5 M3 stories (S-15.10, S-15.12, S-15.13, S-15.15, S-15.16-Part-B). These stories implement BC-5.39.007 (validate-closes-completeness) and BC-5.39.008 (validate-policies-schema) hooks now adversary-converged. Story-writer dispatch ready immediately post D-497 codification.

**Cites:** D-497 (convergence declaration), D-485 (INV-017), D-487 (INV-018), D-489 (INV-019; cures a/b/c), D-490 (INV-020; POLICY 14 5-leg), D-493 (INV-019 RESIDUAL; persisted reports), D-494 (INV-020 RECURRENCE; POLICY 14 verification_step 7), BC-5.39.001 3-CLEAN protocol confirmed working as designed, S-7.02 cycle-closing checklist satisfied.

**Closes:** M3 3M3a-r BC cascade (CONVERGED at pass-11; 3-CLEAN per BC-5.39.001). D-496 codification cycle advances to D-497 CONVERGENCE DECLARATION. Unblocks 3M3b story-writer dispatch for S-15.10/12/13/15/16-Part-B.

- [L-session-2026-05-20-resume-CONVERGENCE](#l-session-2026-05-20-resume-convergence) — MILESTONE: Single 2026-05-20 resume session converged M3 3M3a-r BC cascade (11 passes + 2 PO fix-bursts + 9 state-manager bursts D-487..D-498); cure-extension parsimony principle empirically validated 3 consecutive passes; D-498 session-end durability burst makes state self-contained for zero-context new-session resume.

### L-session-2026-05-20-resume-CONVERGENCE

**Lesson ID:** L-session-2026-05-20-resume-CONVERGENCE
**Date:** 2026-05-20
**Cycle:** v1.0-brownfield-backfill
**Burst:** D-498 SESSION-END DURABILITY BURST

**Achievement:** Single 2026-05-20 resume session converged M3 3M3a-r BC cascade (BC-5.39.006 + BC-5.39.007 + BC-5.39.008) in full — 11 adversary passes, 2 PO fix-bursts, 8 state-manager codification bursts (D-487..D-497), and 1 session-end durability burst (D-498). Starting position: STREAK 0/3 with PO fix-burst pass-4 dispatch-ready. Ending position: M3 3M3a-r CONVERGED D-497 + state durability established at D-498 + 3M3b dispatch-ready for new-session resume.

**Starting position this session:** D-489 codified (pass-4 PO fix-burst CLOSED; 3/3 findings; STREAK 0/3 → pass-5 dispatch-ready). The session picked up immediately from the dispatch-ready state and ran passes-5 through 11 to completion.

**Ending position:** M3 3M3a-r CONVERGED at pass-11 (D-497; STREAK 3/3; cascade trajectory 41→14→8→3→5→2 NIT→1 NIT→1 HIGH→0→0→0). D-498 SESSION-END DURABILITY BURST comprehensive Section 11 + Section 12 rewrite preserves all session state for zero-context resume.

**Engine evolution this session:** Four engine-discipline advances codified during this resume session:
- **INV-019 cure (a) codified in production** (D-489; pass-4 PO fix-burst): grep anchors on load-bearing content; cross-BC assoc-fn idiom standardized.
- **INV-020 CONFIRMED + POLICY 14 → 5-leg quintuple parity** (D-490; pass-5 persist): same-burst propagation must cover all 5 legs — version: / body-Changelog / modified[] / last_amended: text-prefix / upstream-index body-table cells.
- **INV-020 RECURRENCE + POLICY 14 verification_step 7** (D-494; pass-8 persist+fix+codify): literal-shell 4-index self-application gate; BC-INDEX own-leg-4 gap caught by adversary; gate template codified in policies.yaml.
- **INV-019 cure (c) extended to persisted adversary reports** (D-493; pass-7 persist): grep patterns not hardcoded line numbers in evidence sections; same INV class as changelog rows but new application domain.

**Key principle empirically validated — Cure-extension parsimony (3 consecutive passes):** When a META-LEVEL recurrence is structurally the same class as a prior INV, extend the cure rather than introduce a new INV-N abstraction. Pass-8 raised INV-021-CANDIDATE; correct decision was to absorb as INV-020 RECURRENCE with POLICY 14 extension. Pass-9, pass-10, and pass-11 all produced CLEAN verdicts confirming no INV-021 was needed. Three consecutive zeros with the existing cure set is definitive proof the abstraction layer was sufficient. This principle is now engine-grade validated across a complete convergence cycle.

**BC-5.39.001 3-CLEAN protocol worked as designed:** The streak-reset mechanism at pass-8 (HIGH INV-020 RECURRENCE) correctly prevented false convergence — the cascade would have declared convergence at STREAK 2/3 pass-7 despite an unfixed discipline gap in the codification burst itself. The protocol's gating purpose (surface the cures' own self-application gaps) was fulfilled. Passes 9/10/11 provided genuine empirical validation.

**POLICY 14 5-leg + verification_step 7 operational across multiple bursts:** The literal-shell 4-index self-application gate (codified at D-494) was adversary-executed at passes 9, 10, and 11 with PASS verdicts each time. The gate is stable and not a one-time artifact of the codifying burst. This demonstrates the production-grade default applies to gate codification itself — gates must be self-applying.

**INV-019 cure (c) by-construction sustained in all persisted adversary reports:** Passes 9, 10, and 11 adversary reports were all authored using grep patterns rather than hardcoded line numbers in evidence sections. No INV-019 recurrence in the final 3 passes. The cure-discipline propagated correctly from changelog rows (D-489) to persisted reports (D-493) to the adversary's own fresh-context behavior.

**State durability — D-498 rationale:** In-memory task tracking does not survive /clear. Per human directive 2026-05-20 ("we need to make our state durable along with our tasks so we can start this in a new session with zero context"), this D-498 burst comprehensively rewrote STATE.md Section 11 + Section 12 to be self-sufficient. Section 12 shows the exact current dispatch ordering; §11 step 4 embeds the verbatim story-writer Agent tool prompt. A new orchestrator reading only Section 11 can dispatch the correct next action (3M3b story-writer) without any external context. This pattern (session-end durability burst before /clear) is now codified in the session-checkpoints.md archive record.

**Cites:** D-498 (session-end durability), D-489 (INV-019 cure (a) codified), D-490 (INV-020 CONFIRMED; POLICY 14 5-leg), D-491 (POLICY 14 5-leg validated production; ~46 assoc-fn conversions), D-493 (INV-019 cure (c) extended to persisted reports), D-494 (INV-020 RECURRENCE; POLICY 14 verification_step 7), D-495 (pass-9 FIRST CLEAN), D-496 (pass-10 SECOND CLEAN; parsimony validated 2 passes), D-497 (pass-11 CONVERGENCE; parsimony validated 3 passes; S-7.02 satisfied), L-M3-BC-cascade-CONVERGED (milestone lesson), BC-5.39.001 3-CLEAN protocol confirmed working as designed.

**Closes:** 2026-05-20 resume session (18 substantive bursts: PO×2 + state-manager×9 + adversary×7); D-498 durability burst. Advances to 3M3b story-writer dispatch for S-15.10/12/13/15/16-Part-B elaboration.

**Closes:** adv-bc-007-008-pass-10 persistence cycle (STREAK 2/3 SECOND ADVANCE). D-496 codified.

---

## [L-E10-pass15-automation-wave-effectiveness] E-10 pass-15 confirmed automation-wave-effectiveness: adversary surface shifted from governance-process META-class to implementation-correctness

**Date:** 2026-05-27

**Context:** E-10 adversary pass-15 was the first pass run against develop after S-15.03 PRIORITY-A COMPLETE (all 11 automation stories shipped). The adversary reviewed develop@ced39c82 and produced verdict MEDIUM-HIGH 8 findings (0C+2H+4M+2L).

**Key observation — character SHIFT:** Passes 1-14 were dominated by governance-process META-class findings: bare-integer POLICY IDs, policy schema drift, citation-refresh thresholds, BC versioning conventions, index parity enforcement. Pass-15 showed ZERO governance-process META findings. All 8 findings were implementation-correctness findings: max_bytes cap too small, index file reads within same cap, cycle-path hardcoded, on_error behavioral semantics, CI count assertion stale, guard function logic. This is exactly what S-15.03 PRIORITY-A was designed to produce — close the governance tooling gap so the adversary surface shifts to implementation substance.

**TD-VSDD-060 sibling-site sweep was the only HIGH-class finding pattern:** F-PASS15-001 and F-PASS15-002 both found that validate-index-cite-refresh and validate-burst-log had the same 65536-byte cap that had been fixed in validate-state-structure (S-15.09) but not swept to siblings. The fix in PR #160 applied 524288 to 7 crates with TD-VSDD-060 discipline: compile-time assertions on the 2 crates with material behavioral impact; mechanical sweep on the remaining 5. This is the residual class — the discipline is sound; one missed sibling sweep was the gap.

**Lesson:** Automation-wave effectiveness manifests as adversary surface character shift, not just finding count reduction. A stable finding count (8→8) can represent improvement if the CLASS of findings shifted from systemic governance gaps to mechanical implementation details. The trend 22→11→16→16→12→2→1→4→5→4→6→7→5→8→8 shows the asymptotic floor is real and the S-15.03 PRIORITY-A investment paid off in adversary surface quality.

**F-PASS15-003/005/006/007/008 ACCEPTED-AT-ASYMPTOTIC-FLOOR:** Consistent with D-471 model. Cycle-path hardcoding, INDEX.md as Phase 2 secondary, on_error=continue behavioral gap, CI assertion stale, find_part_a_start guard all represent legitimate improvement opportunities but are structurally in the same risk class as the D-471 accepted findings.

**Cites:** D-509 (E-10 pass-15 fix-burst decision), D-471 (asymptotic-acceptance model), TD-VSDD-060 (sibling-site sweep discipline), PR #160 4b68ab83.

---

## [L-banner-format-drift] Banner-format-drift class: validate-state-structure `(wc-l` token scan was prescribed but state-manager dispatch templates didn't enforce it

**Date:** 2026-05-28

**Context:** rc.19 release pipeline Pre-release Validation failed because `pass-real-state-md-snapshot.bats` runs the `validate-state-structure` WASM hook against the live `.factory/STATE.md`. The hook's SIZE BUDGET banner scanner requires entries to match the pattern `N lines (wc-l<terminator>` where terminator is any non-letter/non-digit byte. Five consecutive state-manager bursts (D-504..D-510) produced banner entries using forms like `N lines (AT HARD CAP ...)`, `N lines (verified via wc -l ...)`, and `~N lines (...)` — none of which match the required `(wc-l` token. The hook blocked correctly; the banner format had drifted.

**Root cause — dispatch template gap:** The D-510 fix-burst (HIGH-004) correctly codified that all `~N` approximations must be replaced with literal wc-l counts. However, it did not enforce the `(wc-l;` literal token form required by the hook's scanner. State-manager dispatch templates for STATE.md SIZE BUDGET banner edits accumulated the count values but used prose forms (`verified via wc -l`, `AT HARD CAP`, etc.) that diverge from the canonical token.

**Pattern observed:** Five consecutive bursts (D-504..D-510) each extended the banner in natural-language prose form. No burst explicitly checked that the token form matched the hook regex. The hook's scanner (`lib.rs` comment lines 108-113) is authoritative; the state-manager prompt is not. This is a dispatch-template-vs-hook-contract alignment gap.

**Going-forward rule — mandatory in every state-manager dispatch:** Every new SIZE BUDGET line-growth tracker entry MUST include the literal `(wc-l;` token immediately after the line count. Canonical forms:
- `N lines (wc-l; <prose description>)` — standard form
- `N lines (wc-l; net +N; <prose>)` — line-growth tracker with delta

**Specific forms that are FORBIDDEN:**
- `N lines (verified via wc -l ...)` — does NOT match `(wc-l` token
- `N lines (AT HARD CAP ...)` — does NOT match `(wc-l` token
- `~N lines (...)` — tilde approximation PLUS missing token
- `(this burst; ...)` without a line count — no count = no match possible

**Remediation anchor:** state-manager dispatch templates for STATE.md banner edits MUST include the `(wc-l;` literal token in every new line-growth tracker entry. Future state-manager prompt updates should embed this rule with a concrete example.

**Cites:** D-511 (remediation decision), validate-state-structure hook `crates/hook-plugins/validate-state-structure/src/lib.rs` lines 108-113 (authoritative token patterns), D-510 HIGH-004 (wc-l literal count requirement — necessary but not sufficient; token form also required).

**Closes:** rc.19 Pre-release Validation block (pass-real-state-md-snapshot.bats).

**Closes:** D-509 E-10 pass-15 lesson capture per S-7.02 cycle-closing checklist.

---

## [L-rc19-pre-release-validation-banner-format-drift] rc.19 Pre-release Validation: hook catches dispatch-template-vs-hook-contract alignment gap at release boundary — correct behavior; going-forward template rule established

**Date:** 2026-05-28

**Context:** rc.19 first release attempt (run 26556220729) failed at Pre-release Validation because `pass-real-state-md-snapshot.bats` runs the `validate-state-structure` WASM hook against the live `.factory/STATE.md`. The hook's SIZE BUDGET banner scanner requires entries to match `(wc-l<terminator>` pattern. D-511 remediated 6 entries; the second attempt (run 26581752361) succeeded with all 10 jobs PASS.

**Key distinction from L-banner-format-drift:** L-banner-format-drift (D-511) captures the root cause — dispatch templates drifted from hook contract. This lesson (D-512) captures the release-boundary lesson — Pre-release Validation is the correct backstop; the hook catching it at release time is CORRECT BEHAVIOR, not a failure of the validation system. The system worked as designed: hook passes locally only if format is correct; release pipeline gates on that.

**Release-boundary discipline:** When a WASM hook is added that validates STATE.md structure, ALL new STATE.md edits from that point forward must conform to the hook's pattern. The D-510 HIGH-004 fix (replacing `~N` approximations with literal wc-l counts) was necessary but not sufficient — the `(wc-l;` TOKEN FORM is a separate requirement codified by the hook regex, not just the count value. Both requirements must be satisfied simultaneously.

**First-attempt tag recovery:** The v1.0.0-rc.19 tag was force-deleted and re-pushed at the same fea969ea SHA because no bot artifacts had been created on the first failed attempt. The tag was "innocent" — it correctly pointed at the pre-bot-binary commit. This is the correct recovery procedure for Pre-release Validation failures: no source changes needed, just factory-artifacts remediation + tag re-push.

**Going-forward pattern for release attempts:** When a release attempt fails at Pre-release Validation (NOT at binary-build or marketplace-publish jobs), the recovery is: (1) identify the validate-state-structure block reason from the hook output, (2) apply the remediation in factory-artifacts in a single atomic burst per TD-VSDD-053, (3) force-delete + re-push the release tag at the same SHA, (4) retrigger the pipeline. No source PR needed.

**All 3 planned items COMPLETE:** E-10 pass-15 + fix-burst (D-509, PR #160), F5 pass-75 + fix-burst (D-510), rc.19 SHIPPED (D-512). This closes the pre-session planning window. Next cycle direction pending human input.

**Cites:** D-512 (release ship record), D-511 (banner remediation), D-510 HIGH-004 (wc-l literal count requirement), validate-state-structure WASM hook `crates/hook-plugins/validate-state-structure/src/lib.rs` (authoritative pattern), release pipeline run 26581752361.

**Closes:** D-512 rc.19 release cycle lesson capture.

---

## [L-S-15.17-BC-authoring-clean-propagation] BC-5.39.009 + S-15.17: Clean PO-author → story-writer handoff via POLICY 8 + POLICY 14 quintuple parity is the positive norm for new-BC authoring bursts

**Date:** 2026-05-28

**Context:** D-513 burst: product-owner authored BC-5.39.009 v1.0 (`393527a4`), story-writer propagated into S-15.17 v1.1 (`2300a27a`), state-manager closed with bookkeeping only (duplicate `lifecycle_status:` key fix + D-513 codification + STATE.md advance). No spec-content disputes escalated; adversarial review deferred per protocol.

**Positive pattern — clean handoff anatomy:** PO produced BC with: (1) POLICY 14 5-leg quintuple parity explicitly verified via literal-shell stdout in the commit; (2) `lifecycle_status: draft` at canonical position (after `capability:`, matching BC-5.39.008 precedent); (3) INV-019 cure (a)/(b)/(c) all present; (4) cure-extension-parsimony evaluated per D-497 (explicitly cited BC-5.39.005 + BC-5.39.006 as predecessor patterns; no novel INV-NNN abstraction introduced). Story-writer received clean input and propagated via POLICY 8 (bc_array_changes_propagate_to_body_and_acs) without needing to adjudicate any open PO questions. State-manager bookkeeping was limited to one template artifact fix (duplicate frontmatter key). Three steps, zero escalations, zero rework.

**Lesson — template artifact category:** The duplicate `lifecycle_status: draft` key was a PO template artifact — PO correctly identified it as state-manager's bookkeeping scope and left it for D-513 closing burst. This is the correct division: PO owns spec-content correctness; state-manager owns frontmatter structural hygiene. PO flagging bookkeeping items in the commit message (rather than silently fixing or leaving unflagged) is the correct pattern — it gives state-manager explicit scope without requiring a re-dispatch.

**Lesson — Option A canonical-source-of-truth replacement:** Story-writer replacing Anticipated Postconditions/Invariants sections with authoritative BC reference (Option A) rather than duplicating content is the production-grade pattern for POLICY 8 propagation. The canonical truth lives in the BC; the story body should POINT to it, not copy it. This prevents future drift where story and BC diverge silently.

**Lesson — 18 VP deferral via TD-VSDD-063 pattern:** BC-5.39.009 ships with 18 VPs deferred to architect post-merge (Post-Merge Burst Requirements section in S-15.17 v1.1). Per TD-VSDD-063 precedent, VP allocation requiring cross-subsystem reasoning by the architect is a legitimate defer — it is NOT a missing-spec-content defer (the BCs and ACs are complete; only VP-ID assignment and coverage-matrix placement are pending). The Post-Merge Burst Requirements section makes this deferral explicit and actionable (not silent).

**Replication guide for future new-BC authoring bursts:**
1. PO: POLICY 14 5-leg verified via literal shell in commit; INV-019 cure (a)/(b)/(c) all present; cure-extension-parsimony per D-497 evaluated; `lifecycle_status:` at canonical position.
2. Story-writer: POLICY 8 propagation; Option A canonical-source-of-truth; POLICY 14 5-leg verified; Post-Merge Burst Requirements section for any deferred items with explicit story/wave anchor.
3. State-manager: bookkeeping-only scope (frontmatter hygiene, index verification, D-NNN codification, STATE.md advance); no spec-content writes.
4. If VP allocation deferred: TD-VSDD-063 pattern — Post-Merge Burst Requirements section with [needs-arch] annotation; NOT a TD-register entry unless human-directed.

**Cites:** D-513 (this burst decision), BC-5.39.009 v1.0 (`393527a4`), S-15.17 v1.1 (`2300a27a`), D-497 (cure-extension-parsimony), TD-VSDD-063 (VP deferral pattern), POLICY 8, POLICY 14.

**Closes:** D-513 BC authoring + propagation lesson capture.

---

## [L-S-15.17-SP1-fix-burst-clean-propagation] S-15.17 spec cascade pass-1 fix-burst: clean PO→story-writer→state-manager 3-burst sequence demonstrates POLICY 8 + POLICY 14 discipline working as designed

**Date:** 2026-05-28

**Context:** D-514 burst. Adversary pass-1 produced HIGH verdict 14 findings (5H+5M+3L+1N) against BC-5.39.009 v1.0 + S-15.17 v1.1. PO fix-burst closed 9 BC findings (`87f1bc8f`); story-writer fix-burst closed 5 story findings (`7d12db2f`); state-manager closed with bookkeeping only (this commit). No spec-content disputes escalated between agents; no sibling-sweep gaps discovered; no META-LEVEL self-violation classes surfaced.

**Positive pattern — clean 3-burst anatomy:**
1. Adversary identified 14 findings across BC+story; categorized by owner (BC-owner = PO, story-owner = story-writer) with no ambiguous scope boundaries.
2. PO handled all BC-class findings including the hardest (F-005 LENGTH=4 STRICT adjudication) via cure-extension-parsimony (D-497) — aligning with BC-5.39.006 precedent rather than introducing a novel abstraction.
3. Story-writer received clean input (PO's v1.1 with all BC adjudications settled) and propagated without needing to adjudicate any open PO questions.
4. State-manager closing burst was bookkeeping-only (codifications + lesson + STATE.md advance).

**Key observation — cross-BC semantic divergence caught at pass-1 (F-005):** The hardest finding was F-005 (LENGTH=4 STRICT vs APPROXIMATE boundary). PO resolved it by aligning with BC-5.39.006 inv-6(b)+EC-007 + production STATE.md evidence (the actual tail IS exactly 4). This is the correct resolution path — consult the predecessor BC for semantic parity rather than relaxing or abstracting. Adding EC-018 (LENGTH=5 FORBIDDEN) made the adjudication permanent and testable.

**Key observation — META-LEVEL-30 partial recurrence caught at pass-1 (F-004):** STATE.md extractor anchors (PC2/PC3/PC5) were present in the BC but without literal-shell evidence. PO closed this by specifying the exact anchor forms (table-cell extract and heading-prefix-match) with literal-shell examples. This is a META-LEVEL-30 route (b) partial recurrence — codified-canonical-registry-with-no-runtime-gate — caught at pass-1 and structurally closed (anchors specified, not deferred to implementer). The pattern: adversary catches under-specification; PO provides precision; no cascade prolongation required.

**Replication guide for future BC+story pass-N fix-bursts:**
1. After adversary pass-N: categorize findings by owner before dispatching (BC-class→PO; story-class→story-writer). Prevents cross-agent rework.
2. PO goes first (BC is the source-of-truth; story propagates FROM BC). Merge order matters.
3. Story-writer waits for PO commit SHA before reading BC for propagation — this is the correct handoff point.
4. Hardest adjudications (boundary cases) should consult PREDECESSOR BCs per D-497 cure-extension-parsimony. No novel abstractions unless structurally required.
5. State-manager closes last, bookkeeping-only.

**Cites:** D-514 (this burst), D-513 (BC authoring), D-497 (cure-extension-parsimony), F-S15.17-SP1-005 (F-005 LENGTH=4), F-S15.17-SP1-004 (F-004 extractor anchors), POLICY 8, POLICY 14.

**Closes:** D-514 lesson capture per S-7.02 cycle-closing checklist.

---

## L-S-15.17-SP2-cascade-propagation-gap-from-PC-insertion

**Pattern (META-LEVEL-31 CANDIDATE):** cascade-propagation-gap-from-PC-insertion in BC↔story spec cascades.

**Mechanism:** When PO inserts a new PC into a BC (e.g., BC-5.39.009 v1.1 added PC6 STATE.md cascade Block as a NEW PC), downstream ACs that anchor by PC ordinal silently shift by +1 across the insertion boundary. Story-writer fix-burst that audits ONLY the explicitly-named findings without re-deriving bidirectional AC↔PC parity will miss the cascade impact on other ACs.

**Empirical evidence:** Pass-1 closed F-SP1-003 (3 named AC mis-anchors: AC-14/15/1) but PC6 insertion in PO's v1.1 burst shifted advisory-arm PCs +1; story-writer's v1.2 "all 21 ACs swept" closure narrative was false (ACs 9/10/11/12 remained mis-anchored; AC-17 range stale at "PC1-9"). Pass-2 found F-SP2-001 as regression of F-SP1-003.

**Cure (D-497 parsimony — EXTENSION of POLICY 8, not new abstraction):** POLICY 8 `verification_steps` extended with literal-shell bidirectional AC↔PC parity check requirement: after any PC insertion/deletion/renumbering in BC, story-writer MUST run literal-shell bidirectional parity check (for each PC in BC, grep ACs for cite count; for each AC PC cite, grep BC for existence) with captured stdout per POLICY 15. Captured stdout MUST appear in fix-burst commit body or story §Bidirectional Parity Audit Note. Cure deliverable IS the captured stdout, not narrative claims.

**Anchor:** F-SP2-001 [regression-of-F-SP1-003]; ADV-EDP1-P75-HIGH-002 (META-LEVEL-30 route-(b) parent); D-497 (cure-extension parsimony); D-515 (this burst codification).

**Forward action:** policies.yaml POLICY 8 verification_steps extended in D-515 burst. Future BC-array-change fix-bursts MUST include bidirectional parity stdout. Sibling-sweep (TD-VSDD-060) generalization: same principle applies to invariant-array and EC-array changes — any structural re-numbering of PC/inv/EC requires re-deriving parity for ALL ACs, not just those explicitly named in the adversary report.

**Closes:** D-515 META-LEVEL-31 codification per S-7.02 cycle-closing checklist.

---

## L-S-15.17-SP3-cure-of-cure-recursion

**Date:** 2026-05-28

**Pattern (META-LEVEL-31 sub-route):** cure-of-cure-recursion in spec cascade.

**Mechanism:** Each cure layer introduces a structural escape hatch at increasing META level. Pass-2 cured META-LEVEL-31 (cascade-propagation-gap-from-PC-insertion) via POLICY 8 bidirectional parity audit. Pass-3 found the cure had its own META-31 hole: audit grep target included the audit's own embedded stdout, allowing zero-real-citation PCs to inflate to count=1 (F-SP3-005 — PC6 fabricated by self-reference; AC-7 actually traces to invariant 8, not PC6). F-SP3-014 independently confirmed the same self-counting vector.

**Cure (D-497 parsimony — POLICY 8 v1.2 EXTENSION):** Audit grep target MUST exclude the audit block itself (sed-strip form) OR audit MUST list AC numbers per PC explicitly. `sed '/^## §Bidirectional Parity Audit Note/,/^## /d' story.md | grep -c "BC-5.39.009 PC6"` — strip audit section before counting. Alternative: explicit AC-per-PC listing (e.g., 'PC6: AC-7') makes self-counting structurally impossible. policies.yaml POLICY 8 v1.1→v1.2 applied at PO `ac74474f` burst.

**Forward-watch:** Pass-4 adversary MUST verify POLICY 8 v1.2 audit form does NOT spawn new sub-route. If META-31 sub-route-N+1 surfaces, codify as POLICY 8 v1.3.

**Anchors:** F-SP3-005 + F-SP3-014; D-515 (META-31 cure layer); D-497 (cure-extension parsimony); D-516 (this).

**Cites:** D-516, POLICY 8 v1.2, adv-spec-pass-3.md at ebf7413f, ac74474f PO fix-burst.

**Closes:** D-516 cure-of-cure-recursion lesson capture per S-7.02 cycle-closing checklist.

---

## L-S-15.17-SP3-SDK-grounding-mandate

**Date:** 2026-05-28

**Pattern (META-LEVEL ROOT CAUSE):** BC narrative authored without literal-shell grounding in actual SDK contract / file paths / cycle state / registry / sibling patterns. 3 HIGH+CRITICAL pass-3 findings share this root: F-SP3-002 (regex semantics — PO used mental model of SDK, not literal `grep` of `crates/hook-sdk/src/host.rs`); F-SP3-003 (HostError variant — PC11 claimed wrong variant without checking actual enum definition); F-SP3-006 (Vec<u8> sibling parity — assumed sibling pattern without grepping sibling lib.rs files).

**Mechanism:** Adversary's spec authoring uses mental model of SDK rather than literal grep. Defects survive multi-pass cascades because the source of truth (crates/hook-sdk/src/host.rs, .factory/STATE.md current_cycle:, sibling lib.rs patterns) is never grepped during authoring. Regression cycle: adversary finds defect → PO fixes with another mental-model claim → adversary finds the same class again in next pass.

**Cure (D-497 parsimony — POLICY 5 EXTENSION):** "Creators justify anchors" extended to require literal-shell grep of any cited SDK symbol, file path, cycle name, type signature, registry priority, or sibling pattern, with captured stdout in BC §SDK Grounding Evidence (or named equivalent section). policies.yaml POLICY 5 v1.2→v1.3 applied at this burst (D-516). This codifies the pass-3 mandate the orchestrator applied to the PO burst.

**Applied at pass-3 PO burst** (`ac74474f`) with 9 literal-shell grep captures covering: HostError variants, current_cycle: STATE.md field, Vec<u8> sibling patterns, registry priority allocation, ADR paths, path_component_count form. Pass-4 adversary should verify the §SDK Grounding Evidence section is comprehensive — every BC narrative claim about external state has corresponding grep evidence.

**Forward-watch:** Apply to all future BC-authoring + amendment bursts in this and other cycles. Document in agent prompts (PO, architect) for future cycles. The adversary should flag any BC section making claims about external state without a corresponding §SDK Grounding Evidence grep entry.

**Anchors:** F-SP3-002 + F-SP3-003 + F-SP3-006; D-516.

**Cites:** D-516, POLICY 5 v1.3, adv-spec-pass-3.md at ebf7413f, ac74474f PO fix-burst (9 literal-shell captures).

**Closes:** D-516 SDK-grounding-mandate lesson capture per S-7.02 cycle-closing checklist.

---

## L-S-15.17-SP4-META-32-stable-anchor-extension

**Date:** 2026-05-28

**Pattern (META-LEVEL-32 CANDIDATE — SDK-grounding-mandate-with-stale-pins):** POLICY 5 v1.3 SDK-grounding mandate requires literal-shell grep captures in §SDK Grounding Evidence. However, when those captures use line numbers (`grep -n` flag or `sed -n 'NNN,MMMp'`), the line numbers in the captured stdout decay between the authoring burst and the next adversary pass because intervening edits shift line positions. F-SP4-002 confirmed this class: the POLICY 5 v1.3.1 compliance grep cited `## Phase Progress` at line 61 in STATE.md, but by the time of pass-4 adversary review it had shifted to line 64. The adversary could not verify the citation was pointing to the correct artifact location.

**Mechanism:** Author greps with `grep -n` or `sed -n 'NNNp'`, captures the line-number-prefixed output, and persists it in §SDK Grounding Evidence. The grep was correct at authoring time. On the next adversary pass, the file has been edited, lines shifted, and the persisted stdout is now inaccurate. The adversary cannot distinguish stale-pin from never-verified.

**Cure (D-497 parsimony — POLICY 5 v1.3.1 EXTENSION):** POLICY 5 stable-anchor sub-clause: all grep captures in §SDK Grounding Evidence MUST use stable anchors only — heading prefix `^## `, function signature `^pub fn `, regex pattern `^key:`, etc. Do NOT use `grep -n` (line-number prefix) or bare `sed -n 'NNNp'` as the sole capture. If captured stdout contains `file:NNN:` format, re-execute with a stable-anchor pattern. Stable anchors are invariant to line drift; line numbers are not. POLICY 5 v1.3→v1.3.1 applied at PO fix-burst `f1f0cb52` (D-517).

**Forward-watch:** Pass-5+ adversary MUST verify POLICY 5 v1.3.1 stable-anchor sub-clause — check that no §SDK Grounding Evidence entry uses `grep -n` or bare line-number sed as the sole capture. If F-SP5-XXX class resurfaces on a different sub-constraint of SDK-grounding, extend POLICY 5 v1.3.2 per D-497 parsimony.

**Anchors:** F-SP4-002; D-517 (this burst).

**Cites:** D-517, POLICY 5 v1.3.1, adv-spec-pass-4.md at `c3ddda14`, PO fix-burst `f1f0cb52`.

**Closes:** D-517 META-LEVEL-32 CANDIDATE stable-anchor lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP4-orchestrator-routing-rule-EC-mirror

**Date:** 2026-05-28

**Pattern (PROCESS-GAP — story-local-EC-accumulates-needs-po-placeholders):** When the story-writer adds an Edge Case (EC) row to a story that names a BC anchor (PC, invariant, or AC-class), the PO must mirror an equivalent EC into the BC. Without an orchestrator routing rule, the story-writer flags the gap with `[needs-po]` and the burst closes without the mirror being dispatched. These `[needs-po]` placeholders accumulate across bursts, each one constituting a CLAUDE.md Canonical Principle Rule 3 violation: the deferral was never explicitly authorized by the human, has no concrete future dependency anchor, and is not attached to a specific future story/wave.

**Mechanism:** F-SP4-016 (PROCESS-GAP MEDIUM) surfaced that EC-020 (`[needs-po]`) was added at D-516 story-writer fix-burst but not mirrored into BC-5.39.009 at the same burst. The orchestrator had no codified routing rule requiring same-burst PO mirror for story-local EC additions naming BC anchors. The adversary detected the `[needs-po]` placeholder still present at pass-4.

**Cure (D-497 parsimony — POLICY 8 v1.3 EXTENSION):** POLICY 8 EC-MIRROR ROUTING-RULE sub-clause: when a story adds an EC row referencing a BC anchor, the orchestrator MUST dispatch a same-burst PO mirror task. Story-writer flags new ECs with `[needs-po-mirror]` signal. PO must add equivalent EC to BC body with consistent ID and trigger/expected-behavior/test-vector form. Story-writer MUST NOT leave `[needs-po]` placeholder across burst boundaries. POLICY 8 v1.2→v1.3 applied at state-manager close `<this-burst-SHA>` (D-517). EC-020 was PO-mirrored at fix-burst `f1f0cb52`.

**Forward-watch:** In all future spec cascade fix-bursts where story-writer closes a story finding that introduces a new EC: orchestrator dispatch template MUST include a "PO mirror check" step before story-writer burst is declared complete. Adversary MUST verify absence of `[needs-po]` placeholder across burst boundary.

**Anchors:** F-SP4-016; D-517 (this burst).

**Cites:** D-517, POLICY 8 v1.3, adv-spec-pass-4.md at `c3ddda14`, PO fix-burst `f1f0cb52` (EC-020 mirror), story-writer fix-burst `2a307a4f`.

**Closes:** D-517 EC-mirror routing-rule process-gap lesson capture per S-7.02 cycle-closing checklist. `[codified]` `[process-gap]`

---

## L-S-15.17-SP5-META-33-sibling-sweep-codified

**Date:** 2026-05-28

**Pattern (META-LEVEL-33 — sibling-sweep-gap-inside-policy-cure):** When a stable-anchor policy (POLICY 5 v1.3.1) or evidence-form policy is applied to cure a primary site (e.g., BC §SDK Grounding Evidence), sibling sites that exhibit the same evidence pattern (story T-5 NOTES, parallel body tables, Token Budget rows, code comments) accumulate stale anchors. The sibling sweep is not mandated by the existing policy, so it only covers the primary cure site.

**Mechanism:** CONFIRMED at F-SP4-002 (cure applied to BC §SDK Grounding Evidence only, not to T-5 NOTES) + F-SP5-004 (T-5 NOTES grep -n volatile-pin reverted after POLICY 5 v1.3.1 — stable-anchor discipline applied at BC but not propagated to story T-5 in same burst) + F-SP5-009 (BC Table version cell still v1.3 after BC bumped to v1.5 — sibling table row not swept) + F-SP5-012 (Token Budget BC row stale — sibling row not swept). 3 regression findings in pass-5 all trace to same META-33 class. Cure-extension parsimony validated: this is extension of POLICY 5 v1.3.1 stable-anchor mandate, not a new INV-NNN abstraction (D-497 cure-extension parsimony confirmed 4 consecutive passes META-32→33).

**Cure (D-497 parsimony — POLICY 5 v1.3.3 EXTENSION):** POLICY 5 SIBLING-SWEEP MANDATE sub-clause: when a stable-anchor or evidence-form policy is applied to a primary cure site, the same burst MUST sweep ALL sibling sites with the same evidence pattern. Categories defined: (a) story T-5 NOTES — all grep invocations; (b) story BC Table version cells; (c) story Token Budget BC rows; (d) BC body parallel narrative tables; (e) code comments citing volatile anchors. Sibling-sweep enumeration required in fix-burst commit message as POLICY 5 v1.3.3 self-application attestation.

**Forward-watch:** In all future spec cascade fix-bursts where POLICY 5 is invoked: orchestrator dispatch template MUST include explicit sibling-sweep enumeration step covering categories (a)-(e). Adversary MUST specifically check each category for stable-anchor compliance at pass-N+1.

**Anchors:** F-SP4-002, F-SP5-004, F-SP5-009, F-SP5-012; D-518 (this burst); adv-spec-pass-4.md at `c3ddda14`; adv-spec-pass-5.md at `10d9e443`.

**Cites:** D-518, POLICY 5 v1.3.3, PO fix-burst `8e67ac38`, story-writer fix-burst `117d848a`.

**Closes:** D-518 META-LEVEL-33 lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP5-marker-prefix-redesign

**Date:** 2026-05-28

**Pattern (STRUCTURAL-CURE — inv-4-STRICT-heterogeneous-cell-false-Block):** BC-5.39.009 inv-4 STRICT count==4 applied to production STATE.md current_step: cell produces false-Block because the production current_step carries additional arrows beyond the trajectory-tail segment (8 arrows actual in D-517 current_step vs 4 required by inv-4 STRICT). The inv-4 STRICT rule was designed for the trajectory-tail segment only but was implemented as a global count on the full heterogeneous cell text.

**Mechanism:** F-SP5-001 (CRITICAL): inv-4 specified `grep -oE "→[0-9]+" current_step | wc -l` → expect 4. Production current_step D-517 contains 8 arrow-number sequences (trajectory-tail 4 + carry-across metadata 4). BC-5.39.009 inv-4 as written would Block valid STATE.md writes. This is a structural impossibility: the BC's own STRICT gate would reject the STATE.md that records its own fix-burst (CRITICAL self-blocking). §Cure-Extension Parsimony Note point 2 (do not extend marker-prefix discipline to inv-4) was written as a deliberate boundary at D-514. However, the inv-4 structural impossibility is a CRITICAL defect that the Parsimony Note did not anticipate.

**Cure (HUMAN-DIRECTED PARTIAL REVERSAL of §Cure-Extension Parsimony Note point 2):** Extend BC-5.39.006 v1.7 inv-6(b) semicolon-segment-scoping discipline to BC-5.39.009 PC1/PC2/PC4/PC5 via two-step marker-prefix check: Step 1: locate `trajectory-tail ` marker in the text; Step 2: count `→(\d+)` within the marker-scoped segment (from marker to first `;`). This produces count==4 for any valid trajectory-tail without false-blocking carry-across content. This was the first HUMAN-DIRECTED partial reversal of a §Cure-Extension Parsimony Note in this cycle — validates the §Note's evidentiary purpose (it documents WHY decisions were made, enabling later reversal with rationale when structural evidence overrides parsimony).

**Forward-watch:** §Cure-Extension Parsimony Notes are evidentiary, not permanent prohibitions. When a CRITICAL defect provides structural evidence that the parsimony boundary is incorrect, HUMAN-DIRECTED reversal is correct. The Note must be updated with reversal rationale at the point of reversal.

**Anchors:** F-SP5-001 (CRITICAL); D-518 (this burst); D-514 §Cure-Extension Parsimony Note point 2 original.

**Cites:** D-518, BC-5.39.009 v1.5 inv-4, BC-5.39.006 v1.7 inv-6(b), PO fix-burst `8e67ac38`.

**Closes:** D-518 META-LEVEL-24 cure via inv-4 marker-prefix redesign (HUMAN-DIRECTED partial reversal). `[codified]`

---

## L-S-15.17-SP5-PO-crash-recovery-pattern

**Date:** 2026-05-28

**Pattern (OPERATIONAL — agent-crash-mid-burst-recovery):** PO agent crashed mid-burst (socket error) after completing approximately 85% of BC body redesign work. The BC body was left in the working tree at an internally-consistent v1.5 state (frontmatter bumped + last_amended prepended + body content redesigned per F-SP5-001 marker-prefix spec) but POLICY 14 5-leg parity was incomplete: no Changelog row + no BC-INDEX bump + no policies.yaml extension + no commit.

**Mechanism:** Agent dispatch died after the primary BC body work was done but before the 5-leg parity items (legs 2-5: Changelog body row, index update, policies.yaml extension, commit) were executed. The BC body was internally consistent — inv-4 had been redesigned, PC4/PC9/PC10 had been updated, inv-13 had been added — so redo would waste ~85% of prior agent compute.

**Cure:** Recovery via fresh focused finalization dispatch succeeded in a single dispatch with instruction: "complete remaining 5-leg parity items only — do NOT redo body work." The dispatch completed: (2) Changelog body row appended, (3) BC-INDEX v2.58→v2.59, (4) policies.yaml v1.3.2→v1.3.3 (POLICY 5 META-33 sibling-sweep extension), (5) commit `8e67ac38`. Pattern: when an agent crashes mid-burst, inspect working-tree state first (`git status --short` + spot-check key markers in the partially-completed files); if body is internally consistent, dispatch focused-finalization rather than full redo (saves ~85% of agent compute); if body is inconsistent, soft-reset and restart from scratch.

**Forward-watch:** Before dispatching a fresh agent after a crash, always run `git -C .factory status --short` to assess working-tree state. If partially-completed files are internally consistent (frontmatter bumped, body redesigned, last_amended prepended), use focused-finalization dispatch. If files are partially-written mid-section (truncated YAML, incomplete table rows, dangling brackets), soft-reset and restart.

**Anchors:** D-518 PO crash-resume 2026-05-28; PO fix-burst `8e67ac38`.

**Cites:** D-518, POLICY 14 5-leg quintuple parity mandate, BC-5.39.009 v1.5.

**Closes:** D-518 process-gap lesson capture per S-7.02 cycle-closing checklist. `[process-gap]` `[codified]`

---

## L-S-15.17-SP6-META-34-sibling-sweep-verification-gate

**Date:** 2026-05-29

**Pattern (META-LEVEL-34 — sweep-claim-without-execution):** POLICY 5 v1.3.3 sibling-sweep mandate (codified at D-518 to cure META-33) required enumeration of sibling sites but had NO literal-shell verification gate. Pass-5+6 fix-bursts both produced changelog narrative claiming exhaustive sibling-sweeps that were factually incomplete (F-SP6-002/003/007 in pass-6). The adversary accepted narrative sweep claims because no captured-stdout evidence was required. This is the same class of failure as META-LEVEL-24 (literal-shell vs pseudocode) applied at the POLICY level — a policy can mandate sweep without mandating proof-of-sweep.

**Mechanism:** POLICY 5 v1.3.3 said: "sibling-sweep enumeration REQUIRED in fix-burst commit message as POLICY 5 v1.3.3 self-application attestation." This mandated listing which sites were swept but did NOT require captured stdout proving the sites were swept clean. Fix-burst agents could (and did) produce narrative enumeration of sweep categories without actually executing literal-shell commands with captured output. The adversary could not distinguish "I swept these 5 categories and found nothing" (narrative) from "I swept these 5 categories and here is the grep stdout showing zero results" (verified).

**Cure:** POLICY 5 v1.3.4 LITERAL-SHELL VERIFICATION GATE — after enumerating sibling sites, fix-burst MUST execute literal-shell grep capturing stdout that proves zero stale references (or only Changelog-historical). The captured stdout MUST appear in commit message body or burst-log Dim-2/6. Sweep claims without captured stdout become MEDIUM-severity findings. Validated 1-burst: PO + story-writer both self-applied successfully (all 8 gates empty/historical-only stdout per commit body evidence).

**Cure-of-cure-of-cure recursion:** Each POLICY 5 cure layer (v1.3, v1.3.1, v1.3.3, v1.3.4) revealed a new sub-route at the boundary of self-application: v1.3 — SDK-grounding-mandate (BC authoring without evidence); v1.3.1 — stable-anchor-sub-clause (grep -n volatile-pin); v1.3.3 — sibling-sweep-inside-policy-cure (cure burst missed same pattern at sibling sites); v1.3.4 — sweep-claim-without-execution (enumerated sibling sites but no stdout proof). This recursion is structurally bounded because v1.3.4 adds enforcement (literal-shell + stdout required), not just guidance. A v1.3.5 sub-route would require a way to fake captured stdout — which is structurally impossible to do accidentally.

**Forward-watch:** POLICY 5 v1.3.4 is the terminal layer for the sweep-mandate sub-class because it requires literal-shell stdout evidence (not just enumeration). Adversary pass-7 MUST specifically verify that POLICY 5 v1.3.4 self-application gates were executed with captured stdout, not just listed.

**Anchors:** F-PG-001 process-gap; F-SP6-002/003/007; D-519 (this burst); adv-spec-pass-6.md at `10f7f1ce`.

**Cites:** D-519, POLICY 5 v1.3.4, PO fix-burst `fee45e7e`, story-writer fix-burst `92021f2f`.

**Closes:** D-519 META-LEVEL-34 lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP6-cure-of-cure-of-cure-recursion-success

**Date:** 2026-05-29

**Pattern (PROCESS-POSITIVE — cure-of-cure-of-cure recursion bounded by enforcement mechanism):** For META-LEVEL classes that emerge as policy-self-non-application gaps (META-32 stable-anchor cure didn't apply to its own narrative; META-33 sibling-sweep cure didn't apply to its own siblings; META-34 sweep-claim cure required verification gate), the consistent cure pattern is: (i) recognize the policy needs an enforcement mechanism, not just guidance; (ii) extend the existing policy with a literal-shell verification gate; (iii) self-apply the gate in the same burst that codifies it; (iv) validate via independent adversary pass.

**Mechanism:** Pass-6 demonstrated this pattern works mechanically: PO executed the v1.3.4 verification gate self-application with verbatim stdout proof (4 gates; all empty/Changelog-only). Story-writer executed the same (4 gates; gates b/c/d empty; gate a only provenance-labeled historical references). Both passed on first attempt, no additional fix-bursts required. This is evidence that the "enforcement mechanism > guidance" principle terminates the recursion — because the enforcement mechanism is self-validating (you either have the stdout or you don't).

**Why this generalizes:** The cure-of-cure-of-cure recursion occurs specifically when a policy DESCRIBES what to do without REQUIRING EVIDENCE that it was done. Adding literal-shell verification gates to any policy mandate converts it from a "do this" requirement to a "prove you did this" requirement. The evidence requirement is structurally self-validating. This pattern applies beyond S-15.17 to any VSDD policy that involves enumeration, sweep, or compliance claims.

**Forward-watch:** When authoring or amending any VSDD policy that requires enumeration, sweep, or compliance checks, include a literal-shell verification gate requirement with stdout evidence mandate. Without it, the policy may codify advisory-level sweep without blocking incomplete execution.

**Anchors:** D-519 (this burst); POLICY 5 v1.3.4; PO fix-burst `fee45e7e`; story-writer fix-burst `92021f2f`.

**Cites:** D-519, META-LEVEL-34, POLICY 5 v1.3.4, cure-of-cure-of-cure chain (D-518/519 two-burst validation).

**Closes:** D-519 cure-of-cure-of-cure recursion success lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP7-META-35-replay-reproducibility

**Date:** 2026-05-29

**Pattern (META-LEVEL-35: verification-gate-self-application-asserts-pass-but-replay-yields-non-empty):** Pass-6 codified POLICY 5 v1.3.4 verification gate. Pass-7 fresh-context adversary replayed the identical gate predicate against the same files and found 6+ non-historical hits — despite PO+story-writer pass-6 self-application claiming "all gates empty = PASS". Root cause: "historical-by-construction" category was not explicitly enumerated — different agents applied different intuitions about what content forms are historical vs current. The v1.3.4 gate required captured-stdout proof but not a shared definition of what stdout is EXPECTED to contain.

**Cure: POLICY 5 v1.3.5** with three parts: (a) Part A — explicit historical-by-construction enumeration (only 5 content forms are historical: YAML modified[] entries, ## Changelog rows, [Prior:] in last_amended, §Adversary Pass Coverage entries, lesson cross-refs); (b) Part B — adversary-replay-reproducibility mandate (cite parent-commit SHA so a fresh-context adversary can reproduce stdout at exactly that SHA; gate claims without parent-commit SHA are NOT reproducible); (c) Part C — sibling-sweep categories extended (a)-(h) adding (f) Risk-Mitigation, (g) Parity Audit Note, (h) LOCAL Adversary Cascade Plan.

**Validation:** PO and story-writer both passed v1.3.5 gates in the same burst at parent-commit cite f5bf4082. Cure-of-cure-of-cure-OF-cure recursion now at level 5: v1.3 → v1.3.1 → v1.3.3 → v1.3.4 → v1.3.5. Each layer adds an enforcement primitive (SDK-grounding → stable-anchor → sibling-sweep → literal-shell stdout → explicit-enumeration + replay-reproducibility).

**Anchors:** F-SP7-PG-001 process-gap; D-520 (this burst); adv-spec-pass-7.md at `d4cadf68`.

**Cites:** D-520, POLICY 5 v1.3.5, PO fix-burst `f5bf4082`, story-writer fix-burst `7b54600d`.

**Closes:** D-520 META-LEVEL-35 lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP7-asymptotic-floor-broken

**Date:** 2026-05-29

**Pattern (CONVERGENCE EVENT — asymptotic-floor partially broken by cure-stack accumulation):** Pass-7 finding count dropped to 9, the FIRST sub-11 result since pass-1, breaking the [11-16] asymptotic-floor pattern that held for passes 2-6. Trajectory: 14→11→14→16→12→11→9. The cumulative cure stack — (1) marker-prefix discipline (pass-5 META-24 cure), (2) META-33 sibling-sweep (pass-5), (3) META-34 literal-shell verification gate (pass-6), (4) META-35 historical-by-construction enumeration + replay-reproducibility (pass-7) — is collectively producing material improvement.

**Why this generalizes:** META-LEVEL gaps emerging at policy-self-application boundaries are systematically cured by adding the NEXT layer of enforcement primitive. The pattern: (i) recognize the policy needs enforcement, not guidance; (ii) add enforcement via explicit enumeration or literal-shell gate or reproducibility constraint; (iii) self-apply the enforcement in the same burst; (iv) validate via independent adversary pass. This terminates the policy-guidance-only recursion loop because enforcement primitives are self-validating — you either have the stdout/SHA/enumeration or you don't.

**The cure-of-cure-of-cure recursion DOES asymptote** when each layer adds enforcement (not just guidance). The trajectory drop from 11→9 is the first empirical evidence of floor-breaking, suggesting the v1.3.5 enforcement layer is materially more effective than v1.3.4.

**Forward-watch:** Pass-8 is the first diagnostic opportunity. If trajectory <9 with NO new META class → convergence toward 3-CLEAN is plausible. If ≥9 OR new META class → SEAL adjudication is the production-grade response. The asymptotic-floor breaking does NOT mean convergence is certain — it means the cure stack is working and more passes will determine if 3-CLEAN is achievable vs requiring SEAL.

**Anchors:** D-520 (this burst); trajectory 14→11→14→16→12→11→9; POLICY 5 v1.3.5.

**Cites:** D-520, META-LEVEL-35, PO fix-burst `f5bf4082`, story-writer fix-burst `7b54600d`.

**Closes:** D-520 asymptotic-floor-broken lesson capture per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP8-META-36-snapshot-rescue

**Date:** 2026-05-29

**Pattern (META-LEVEL-36 CODIFIED — snapshot-annotation-rescue-pattern via fresh-context-loop-asymmetry):** POLICY 5 v1.3.5 Part B replay-reproducibility mandated parent-commit SHA citation + replay against SAME SHA. F-SP7-004 cure (pass-7) annotated Grep 10 stdout with historical parent-commit SHA — this satisfied Part B literal letter (SHA cited) but the adversary fresh-context loop ALWAYS executes at HEAD, never checks out historical SHAs. The snapshot annotation pattern satisfied Part B letter while defeating the reproducibility guarantee through loop-asymmetry: the annotated stdout was valid at the historical SHA but the adversary could NOT reproduce it by executing the gate at HEAD.

**Cure:** POLICY 5 v1.3.6 3-part: (1) Part B REVISED — verification gates MUST be HEAD-reproducible at any cycle SHA OR use structural-form-only citations (function names, invariant properties, file paths — invariant structural properties not snapshot-dependent); snapshot-annotation-only FORBIDDEN. (2) Part D NEW — adversary executing at HEAD MUST detect if cited stdout evidence could only be valid at a specific historical SHA. Non-HEAD-reproducible citations = META-36 violation regardless of whether a SHA is cited.

**Validated 1-burst:** PO (068725ea) self-applied v1.3.6 Part B — Grep 10 rewritten from snapshot-annotation to STRUCTURAL-FORM-only (trajectory-tail marker grep yields identical results at any SHA where STATE.md has marker-prefix discipline; HEAD-reproducible). Story-writer (aaf69b74) self-applied v1.3.6 gates at HEAD — structural-form stdout; zero non-historical stale refs.

**Cure-of-cure-of-cure-OF-cure-OF-cure-OF-cure recursion:** POLICY 5 has now codified 6 cure layers — v1.3 SDK-grounding → v1.3.1 stable-anchor → v1.3.3 sibling-sweep enumeration → v1.3.4 literal-shell verification gate → v1.3.5 historical-by-construction + replay-reproducibility → v1.3.6 HEAD-reproducibility + structural-form-only. Each cure addresses the prior cure's self-application gap. Pattern: META-LEVEL gaps at policy-self-application boundaries are systematically cured by adding the NEXT enforcement primitive (structural-invariant replacing snapshot; HEAD-reproducibility replacing SHA-citation).

**Forward-watch:** Whether this recursion bottoms out structurally is the diagnostic question for pass-9. If Grep 10 structural-form-only is truly HEAD-reproducible AND no new paper-fixes remain AND no new META class emerges, convergence within 2-4 passes is plausible. The adversary should specifically validate: (1) Is Grep 10 structural-form-only and HEAD-reproducible at pass-9 HEAD? (2) Does Part D snapshot-rescue-pattern detection catch any remaining snapshot-annotation evidence in BC body? (3) Are all 3 TD-VSDD-059 paper-fixes from Pass-5 structurally closed?

**Anchors:** D-521 (this burst); META-LEVEL-36; PO fix-burst `068725ea`; story-writer fix-burst `aaf69b74`; parent `aaf69b74` per D-419(b).

**Closes:** D-521 META-36 codification per S-7.02 cycle-closing checklist. `[codified]`

---

## L-S-15.17-SP8-TD-VSDD-059-paper-fix-detection

**Date:** 2026-05-29

**Pattern (process observation — TD-VSDD-059 paper-fix detection via cure-claim-vs-body-text-audit):** 3 long-standing TD-VSDD-059 paper-fixes from Pass-5 (§Cure-Extension Parsimony Note point 2 "deliberate non-extension" surviving HUMAN-DIRECTED REVERSAL claim; PC10 LENGTH=4 count check surviving OUT-OF-SCOPE annotation; §D-453(d) Site 9 stale IN-SCOPE annotation) survived 3 fresh-context adversary passes (pass-6/7/8) before detection at pass-8. Detection at pass-8 was triggered by an adversary with sufficiently broad context to compare changelog-claim vs body-text consistency.

**Why paper-fixes survive multiple passes:** A narrow-scope adversary focused on new findings can miss long-standing paper-fixes from prior bursts because the adversary's fresh-context Iron Law (BC-5.39.001) prevents it from reviewing prior-pass adversary reports that would flag the original cure claims. The adversary CAN read the BC's own §Adversary Pass Coverage section (historical-by-construction) and verify body text matches the cure claims documented therein.

**Mitigation — cure-claim-vs-body-text-audit pattern:** Adversary SHOULD periodically re-execute the implicit gate "does the body text actually match the changelog/§Adversary-Pass-Coverage claim?" against ALL prior-pass cure attestations. This is generalizable from POLICY 5 v1.3.6 Part D snapshot-rescue-pattern detection: the adversary executing at HEAD can verify whether any BC body claim (not just snapshot citations) matches its alleged cure. POLICY 5 v1.3.6 Part D is the structural home for this check.

**Specific paper-fixes closed at D-521:**
1. §Cure-Extension Parsimony Note point 2: claimed "deliberate non-extension is HUMAN-DIRECTED REVERSAL" but the actual HUMAN-DIRECTED instruction was not documented in the BC body. Fix: point 2 rewritten to faithfully document the F-SP5-001 HUMAN-DIRECTED REVERSAL with the actual human instruction preserved.
2. PC10 OUT-OF-SCOPE: surviving LENGTH=4 count check annotation that claimed OUT-OF-SCOPE but still included a count check for the out-of-scope content. Fix: PC10 body rewritten to structurally exclude the out-of-scope content without count reference.
3. §D-453(d) Site 9: stale IN-SCOPE annotation claiming Site 9 was in-scope but body implementation did not actually cover Site 9 per the §D-453(d) canonical mapping. Fix: Site 9 documentation updated to reflect actual implementation scope.

**Forward-discipline for future adversary passes:** Before declaring a finding closed, adversary MUST verify: (a) Does the BC BODY TEXT actually reflect the claimed cure? (b) Does the §Adversary Pass Coverage entry for the prior closure faithfully describe the structural change — not just assert "CLOSED"? (c) Is the cure form structural (code/spec content change) or cosmetic (annotation/label change)?

**Anchors:** D-521 (this burst); TD-VSDD-059; adv-spec-pass-8.md at `dfcbea39`; F-SP8-001/002 closures.

**Closes:** D-521 TD-VSDD-059 paper-fix detection lesson per S-7.02 cycle-closing checklist. `[codified]` `[process-gap]`

---

## L-S-15.17-SP9-META-37-asymptotic-acceptance-SEAL

**Date:** 2026-05-29

**Pattern (META-LEVEL-37 CANDIDATE — scalar-snapshot-of-cardinality-presented-as-structural-form-invariant-but-disowned-in-prose):** Distinct from META-36 (SHA-pinning of snapshot stdout) — META-37 captures a NUMERICAL VALUE (count, occurrence cardinality) and calls the value structural when only the PREDICATE producing non-zero is structural. F-SP9-001: Grep 10 in POLICY 5 v1.3.6 §Application Example captured `16` trajectory-tail occurrences in the self-application evidence. At HEAD the actual count is `17`. POLICY 5 v1.3.6 Part B mandated HEAD-reproducibility OR structural-form-only; the scalar `16` is neither — it is a snapshot-of-cardinality, not the structural form-invariant (trajectory-tail marker exists).

**Cure (if executed):** Forbid captured-stdout numerical scalars in §Application Examples; mandate predicate-only narrative (e.g., "grep returns non-empty = structural-form-only passing"). Scalar counts in examples are self-invalidating as STATE.md grows.

**DECISION: META-37 codification DEFERRED via SEAL.** The cure-of-cure recursion at LEVEL 7 has structurally demonstrated impossibility to terminate under prose-only codification. Continuing to codify META-37/38/N cures produces new sub-routes faster than closures land. SEAL adjudication per D-386 Option C accepts the asymptotic floor + META-LEVEL ascent as the documented residual. The pattern: each cure layer at policy-self-application boundary admits a new META-LEVEL via cure-form-asymmetry (snapshot→structural→scalar→...→N). Operational discipline (literal-shell verification gates at HEAD per POLICY 5 v1.3.4 + v1.3.6) DOES work mechanically — the asymptote is at NARRATIVE-vs-PRODUCTION drift in §Application Examples and §Cure-Extension Parsimony, not at literal-shell gates.

**Anchors:** F-SP9-001; D-522 (this burst); adv-spec-pass-9.md at `30e0a08a`; POLICY 5 v1.3.6.

**Cites:** D-522 SEAL adjudication; D-386 Option C; D-477 precedent (S-15.14 SEAL); D-471 (asymptotic-floor acceptance).

**Closes:** D-522 META-37 CANDIDATE lesson capture — classified as cure-recursion-structural-impossibility evidence, NOT as a new codifiable cure. `[codified]` `[seal-precedent]`

---

## L-S-15.17-cascade-9-pass-SEAL-precedent

**Date:** 2026-05-29

**Pattern (process observation — 9-pass SEAL precedent joins F5 D-386 + S-15.14 D-477):** S-15.17 is the THIRD cycle to invoke D-386 Option C asymptotic-acceptance. Common 3-stage pattern across all three:

**Stage 1 (passes 1-3): Real defects.** Initial passes produce mostly substantive closure (missing content, structural gaps, spec inconsistencies). Fix-bursts yield meaningful improvements. Trajectory moves generally downward.

**Stage 2 (passes 4-6): Mixed real + META-LEVEL cures.** Middle passes produce genuine structural findings mixed with policy-self-application gaps. Each new POLICY cure introduces the next-level violation in its own application example. Trajectory oscillates. POLICY cure count begins to match pass count.

**Stage 3 (passes 7+): Cure-of-cure-of-cure recursion.** Late passes produce mostly META-LEVEL recursion at policy-self-application boundary. Structural real-defect count approaches zero; residual count is dominated by the infinite recursion possibility space. Adversary explicitly recommends SEAL.

**Heuristic (3-stage threshold for SEAL):** When POLICY cure-layer count >= adversary pass count - 4, SEAL is warranted. For S-15.17: POLICY 5 had 6 cure layers at pass-9 → 9 - 4 = 5 → 6 >= 5 → SEAL appropriate. For S-15.14: 6 META-LEVEL classes at pass-11 → 11 - 4 = 7 → 6 close (SEAL appropriate). For F5: 29 META-LEVEL classes at pass-14 → 14 - 4 = 10 → well over threshold.

**Structural interpretation:** The cure-layer-count >= pass-count - 4 threshold is a heuristic for when the META-LEVEL recursion has consumed the productive improvement space. Once each pass produces more META-LEVEL cure variants than structural closures, additional passes yield diminishing returns and new liability (each cure adds a new self-application surface).

**Forward use:** This heuristic could be codified in `convergence-protocol.md` for future cycle orchestration as the SEAL early-warning trigger. When monitoring a cascade: track POLICY cure-layer count vs pass count. When cure-layer count approaches pass-count - 4, prepare SEAL adjudication evidence.

**Anchors:** D-522 (this burst); F5 D-386 (14-pass SEAL); S-15.14 D-477 (11-pass SEAL); S-15.17 D-522 (9-pass SEAL).

**Cites:** D-386 Option C; D-477; D-522.

**Closes:** D-522 cascade SEAL precedent lesson capture per S-7.02 cycle-closing checklist. `[codified]` `[meta-process]`

---

### L-S-15.17-remove-uncertainty-clean-result

**Category:** positive-result + method-note

**Lesson:** The remove-uncertainty sweep on S-15.17 returned CLEAN (7/7 assumptions CONFIRMED, zero CRITICAL failures) — in contrast to D-501's sweep on the M3 wave which found 5 CRITICAL-class implementation failures. Reason: S-15.17's SDK-grounding was already hardened through the 9-pass adversarial cascade (BC §SDK Grounding Evidence Greps 1-10 + the META-32→37 SDK-grounding-mandate codifications forced host.rs symbol verification at every pass). The cascade's pain (cure-of-cure-of-cure SDK-grounding recursion) paid off as pre-validated implementation readiness. Method note: remove-uncertainty correctly split INTERNAL claims (validated via codebase Grep — hook-sdk is this repo's own crate, Perplexity cannot see it) from EXTERNAL claims (validated via Perplexity deep-research — wasm32-wasip1 rename timeline, regex WASM cost). The 2 residual fixes were doc-quality (false conditional premise + stale fixture text), not design errors. Takeaway: a spec that survives a deep SDK-grounding adversarial cascade enters remove-uncertainty largely pre-validated; the sweep becomes a confirmation pass rather than a defect-discovery pass. Going forward: when a story's adversarial cascade explicitly surfaces SDK-grounding cures (POLICY 5 class), the remove-uncertainty sweep may be abbreviated to a verification-only sweep rather than a discovery sweep — but must still run (the 2 doc-quality fixes in U6+U7 demonstrate value even in "clean" sweeps).

**Anchors:** D-523 (this sweep); D-501 (M3 wave sweep — CRITICAL class contrast); D-516 (SDK-grounding mandate codified); D-522 (SEAL).

**Cites:** D-501; D-516; D-522; D-523.

**Closes:** D-523 remove-uncertainty sweep lesson capture. `[codified]`

---

### L-S-15.17-cycle-conditional-cure-ADR-023

**Category:** spec-design + architecture-decision

**Lesson:** A CRITICAL spec finding (F-S15.17-LOCAL-P5-001 — live STATE.md brick risk from unconditional PC3/PC4/PC5 Block on all cycle types) that survived a 9-pass SEALED cascade was resolved NOT by reopening the cascade but by a targeted human-authorized spec amendment (ADR-023 cycle-conditional site model). The cure is architecturally elegant: PC1/PC2 (current_step + Last Updated cell) are cycle-invariant and remain always-Block; PC3/PC4/PC5 (Phase Progress, Concurrent Cycles, Session Resume §1) are F5-style artifacts and Block only when the active cycle INDEX.md carries `per_pass_trajectory: true`. Milestone/story-delivery cycles omit the flag (absence = false = fail-safe: never Block). Lesson: when a spec SEAL is reached at asymptotic acceptance and a subsequent implementation-phase CRITICAL finding reveals a structural spec-vs-reality conflict (not a convergence gap), the correct path is human-authorized targeted amendment, not a cascade restart. The cascade restart (3-CLEAN from 0/3) on the amended v1.9 BC is the correct verification step — not a sunk cost. Going forward: the cycle-conditional flag mechanism (per_pass_trajectory) is an extensible pattern for other BCs that need to distinguish F5-style pipeline cycles from milestone/story-delivery cycles. Any BC prescribing behavior on Phase Progress or Concurrent Cycles rows should evaluate whether a similar flag gate applies.

**Anchors:** D-525 (ADR-023 adoption + BC v1.9); D-522 (prior SEAL); F-S15.17-LOCAL-P5-001 (CRITICAL finding that prompted this).

**Cites:** D-522; D-525; ADR-023.

**Closes:** D-525 + D-526 cycle-conditional cure lesson capture per S-7.02 checklist. `[codified]`

---

### L-F-P3-008-wallclock-deflake-structural-recurrence

**Category:** test-infrastructure + dependency-management

**Lesson:** The F-P3-008 ubuntu timing flake pattern (wall-clock assertions against async event infrastructure) recurred in S-15.17's bats suite even after structural closure in S-15.05 (PR #143 224fa184 TC-9 event-observation). The recurrence mechanism: a new bats suite for a new hook crate that interacts with the same async dispatch infrastructure can independently exhibit the same timing dependency if it does not inherit the wait_for_log_event helper discipline from S-15.05. PR #165 (f34b7567) resolved the S-15.17 recurrence structurally. Lesson: the wait_for_log_event async helper pattern must be explicitly propagated to every new bats suite that observes async dispatch events — it is not automatically inherited. Going forward: when a new hook crate's bats suite is dispatched, the implementer dispatch template should include a mandatory step: "verify no wall-clock assertions against async events — use wait_for_log_event helper from full_stack_plugin_invocation.rs or equivalent." TD #67 FULLY RESOLVED across all known suites as of PR #165.

**Anchors:** D-526 (S-15.17 SHIPPED; PR #165 f34b7567); D-505 (S-15.05 initial closure); F-P3-008 (original ubuntu timing flake).

**Cites:** D-505; D-526; PR #143; PR #165.

**Closes:** D-526 F-P3-008 recurrence lesson capture per S-7.02 checklist. `[codified]`

---

### L-session-2026-05-31-fabricated-SHA-discipline

**Category:** orchestrator-bookkeeping + state-manager-protocol

**Lesson:** When the orchestrator feeds a merge commit SHA to the state-manager, that SHA MUST be read from the actual GitHub merge event (e.g., `gh pr view NNN --json mergeCommit`) AFTER the merge completes. Anticipating a merge SHA before the merge — or carrying forward a placeholder SHA from a prior draft state — produces a fabricated-SHA defect that requires a subsequent correction commit (D-526 went through three commits: ab822bfa primary → 66ae0a2c SHA-patch → 5fa87c19 SHA-correction, because the orchestrator fed the wrong SHA twice). The correction commit creates an additional chain link that must itself be tracked. Prevention: the state-manager's post-merge burst dispatch template MUST include a mandatory step: "read the actual merge commit SHA via `gh pr view NNN --json mergeCommit --jq .mergeCommit.oid` before constructing any factory-artifacts commit; do not use anticipated or placeholder SHAs."

**Anchors:** D-527 (SESSION-END DURABILITY BURST codifying this lesson); D-526 fabricated-SHA chain (ab822bfa → 66ae0a2c → 5fa87c19).

**Cites:** D-526; D-527; D-447(c); D-449(e).

**Closes:** D-527 fabricated-SHA process lesson capture. `[codified]`

---

### L-session-2026-06-01-rc20-clean-ship

**Category:** release-discipline + orchestrator-bookkeeping

**Lesson:** rc.20 shipped clean on the first attempt (run 26738809372 all 6 jobs PASS). Contrast with rc.19 (D-512), which required a first-attempt remediation: D-511 banner-format fix was needed because validate-state-structure WASM hook blocked on SIZE BUDGET entries using `N lines (wc -l ...)` form instead of canonical `(wc-l;` token. rc.20 avoided this class of defect because the state-manager had been applying the canonical `(wc-l;` form consistently since D-511. Additionally, rc.20 used --merge (not squash) for PR #166, preserving the main-is-ancestor-of-develop invariant (TD #68) required for the next release's sync-develop job. The Release PR --merge strategy is validated for a second cycle. Going forward: every release PR MUST use `--merge` and the state-manager post-merge burst MUST verify `git merge-base --is-ancestor main develop` passes.

**Anchors:** D-528 (rc.20 release ship record); D-512 (rc.19 SHIPPED with first-attempt failure); D-511 (banner-format remediation).

**Cites:** D-511; D-512; D-528; PR #166 e00ab1ab; RELEASING.md.

**Closes:** D-528 rc.20 clean-ship lesson capture. `[codified]`

---

### L-session-2026-06-01-dependabot-sweep

**Category:** dependency-management + maintenance-discipline

**Lesson:** Transitive-major version bumps in optional skill dependencies (e.g., excalidraw 0.17→0.18 in visual-companion) are low-blast-radius when the dep is npm-only and isolated to a single optional skill with no cross-crate Rust effects. Dependabot auto-closes redundant PRs that target the same dependency after a bump merges — after PR #156 merged (excalidraw 0.18.1), Dependabot automatically closed PRs #152, #125, #2, and bonus #167 without manual intervention. When processing Dependabot queues, merge the highest-version bump first; redundant lower-version PRs self-close.

**Anchors:** D-529 (POST-RC.20 maintenance sweep); PR #156 excalidraw merge auto-closing #152/#125/#2/#167.

**Cites:** D-529; PR #156 1e5325bd; PR #157 b21fd358; PR #3 401f1bfb.

**Closes:** D-529 Dependabot lesson capture. `[codified]`

---

### L-E10-pass16-derived-ci-count

**Category:** ci-discipline + adversarial-convergence + production-grade-default

**Lesson:** Re-escalating a finding from ACCEPTED-AT-FLOOR to FIX-NOW is warranted when three conditions hold simultaneously: (1) the gap between the accepted floor value and reality has widened enough to make the gate functionally meaningless (>=16 vs actual 28 plugins is ~57% gap), (2) the fix is cheap and bounded (~1 CI file, 3 sites, no logic change), and (3) the fix makes the check self-maintaining (derived count from `ls -d crates/hook-plugins/*/` always reflects the current codebase state; no future manual update required). A literal floor that requires manual bumping on every new crate is a staleness-class process gap that will predictably recur. Deriving the floor from the source-of-truth (crate directory count) eliminates the recurrence class structurally. The production-grade default is self-maintaining checks over literal-maintenance checks; defer to literal only when self-maintaining is infeasible.

**Anchors:** D-530 (E-10 pass-16); F-PASS15-007 (original LOW ACCEPTED-AT-FLOOR); F-PASS16-002 (re-escalated FIX-NOW; ci.yml derived count); PR #168 82163b7f.

**Cites:** D-530; D-471 (ACCEPTED-AT-FLOOR model); PR #168; ci.yml lines 192-202 / 229-240 / 440-450.

**Closes:** D-530 pass-16 S-7.02 lesson capture; F-PASS15-007 closure via F-PASS16-002 FIX-NOW re-escalation. `[codified]`

---

### L-E10-cascade-SEAL-16-pass

**Category:** adversarial-convergence + asymptotic-acceptance + engine-implementation-discipline

**Lesson:** The E-10 engine-implementation adversarial cascade SEALED at pass-16 under D-471 asymptotic-acceptance / D-386 Option C — the third cascade in this project to use this seal model (prior: F5 at D-386/D-454 pass-74, S-15.14 at D-477 pass-11, S-15.17 at D-522 pass-9). Key structural observations for the record: (1) **Character shift as a convergence signal** — the cascade traversed three distinct defect-class phases: governance-process META-class (passes 1-14, codification failures), implementation-correctness class (passes 14-15, MAX_BYTES cap), and CI-floor-staleness class (passes 15-16, literal count decay). When the dominant defect class shifts from structural/systemic to cosmetic/maintenance, the cascade has found its genuine floor. (2) **S-15.03 PRIORITY-A automation wave effectiveness confirmed** — passes 1-14 were dominated by process-gap / POLICY codification failures. After the automation wave shipped 11 hooks, passes 15-16 found zero process-gap findings in the automation-covered domains; only 3 residual LOWs remained, of which 1 was cheaply fixed (F-PASS16-002). The automation wave worked. (3) **Derived count beats literal count for self-maintaining CI gates** — F-PASS16-002 illustrated that a literal floor (`>=16`) that requires manual bumping on every new crate is a staleness-class recurrence. Deriving from the source-of-truth (`ls -d crates/hook-plugins/*/`) eliminates the recurrence structurally. Prefer self-maintaining checks wherever feasible. (4) **Asymptotic-acceptance is the correct production-grade response to prose-only convergence limits** — 3-CLEAN is structurally impossible when curable META-class findings can always be found in a large prose corpus under fresh-context analysis. Sealing at a LOW-verdict pass with clear S-7.02 satisfaction (no open process-gaps; FIX-NOW done in-scope; residuals cosmetically deferred at-floor) is the production-grade decision, not a compromise.

**Anchors:** D-531 (E-10 SEALED); D-471 (asymptotic-acceptance seal model); D-386 Option C (F5 precedent); D-509 (pass-15 fix PR #160); D-530 (pass-16 fix PR #168); S-15.03 PRIORITY-A (automation wave).

**Cites:** D-531; D-471; D-386; D-477; D-522; D-509; D-530; PR #160 4b68ab83; PR #168 82163b7f.

**Closes:** D-531 milestone lesson capture; E-10 cascade 16-pass asymptotic-acceptance SEAL. `[codified]`

---

### L-session-2026-06-08-session-end-durability

**Category:** session-management + state-hygiene + durability-discipline

**Lesson:** SESSION-END DURABILITY BURST following rc.20 SHIP + E-10 CASCADE SEAL at clean milestone. Four durability discipline points confirmed at D-532:

(1) **D-430(a) compaction is load-bearing, not cosmetic** — STATE.md was at 488 lines (12 from hard cap) entering this burst. Compaction to 379 lines (36 under soft-target) restores the full 121-line headroom for future bursts. Every session-end burst at or above 450 lines should trigger compaction. The rule: archive Phase Progress rows older than 6 months of work OR compacted-marker rows; archive banner tracker entries older than 10 bursts; archive Decisions Log rows older than 20 decisions.

(2) **Two follow-up candidates captured durably** — The session surfaced two minor items (test_F_P2_001 timing flake FLAKE-001; O-PASS16-002 stale header COSMETIC-001) that were observed but not fixed. Recording them explicitly in §12 Pending Work Items AND Drift Items ensures they are not lost across CLEAR. This is the correct production-grade default: either fix in-scope or record durably; never silently discard.

(3) **Clean milestone is the ideal durability burst target** — Performing session-end durability after rc.20 SHIP + E-10 SEAL + maintenance sweep (zero open PRs, no in-flight worktrees) means the checkpoint §1-§12 is minimal and clean. No partially-resolved state to carry. Future sessions benefit maximally from clean milestone durability bursts.

(4) **SHA-patch follow-up is a two-step atomic operation** — Primary commit sets all content except the factory-artifacts HEAD SHA (which isn't known yet). SHA-patch commit updates only the Active Branches factory-artifacts row and burst-log SHA placeholders. These two commits together constitute the single logical burst (per TD-VSDD-053 Single-Commit-Per-Burst — the SHA-patch is explicitly exempted as a follow-up, not a "second commit" in the prohibited sense).

**Anchors:** D-532 (this burst); D-531 (E-10 CASCADE SEALED); D-528 (rc.20 SHIPPED); D-430(a) (compaction authorization).

**Cites:** D-532; D-531; D-430(a); TD-VSDD-053; POLICY 1 (checkpoint archive).

**Closes:** D-532 session-end durability lesson capture. `[codified]`

---

### L-issue-128-cross-family-adversary

**Category:** adversarial-review + model-diversity + operational-tooling

**Lesson:** Two reusable learnings from the issue-128 delivery (D-534) with Gemini cross-model-family adversary:

**(a) Cross-model-family adversary catches implementer-introduced regressions that same-family review misses.** The Gemini adversary ran 3 passes (findings 6→4→4). Critically, each pass caught a real regression that the prior fix introduced: Pass 1 found 6 core-correctness bugs; Pass 2 found 4 including a branch-protection completion deadlock introduced by Pass 1's fix; Pass 3 found 4 including a post-delete replication-lag wedge introduced by Pass 2's fix. All regressions were fixed in-scope; none deferred. This confirms the model-diversity value in the review loop: a Gemini adversary (different family than the Claude implementer) is not anchored to the same reasoning patterns and therefore catches completeness failures at boundaries that same-family review is likely to miss. Pattern generalizes: when the implementer is Claude-family, the adversary should be Gemini-family (or vice versa) for maximum cognitive diversity. Per D-386 Option C asymptotic-acceptance, 6→4→4 with severity shift from core-correctness to fine edge-robustness constitutes convergence (the floor is genuine, not a failure to improve).

**(b) agy --print reads prompt from STDIN, not argv.** When using antigravity-cli (`agy`) for cross-family adversary dispatch, the `--print` flag causes the tool to read the prompt from STDIN rather than a positional argument. Correct invocation: `agy --print --model "Gemini 3.5 Flash (High)" < slice-prompt.txt` (or via heredoc). Passing the prompt as a positional argument after `--print` is silently ignored. This is an operational gotcha that can cause misleading results (empty or default model behavior) if not caught. Always verify the model actually received the intended prompt by checking the first few lines of the response for prompt acknowledgment.

**Anchors:** D-534 (this delivery); D-386 Option C (asymptotic-acceptance convergence model); D-533 (validated backlog sourcing #128).

**Cites:** D-534; D-533; D-386; D-471; PR #178; feature/issue-128-verify-branch-deletion @ abde4c68.

**Closes:** D-534 lessons-capture; issue-128 cross-family adversary pattern codification. `[codified]`

---

### L-issue-128-PR-178-merged

**Category:** ci-infra + merge-process + observability

**Lesson:** Post-merge CI hang observation for PRs touching zero Rust: build-dispatcher cargo-test jobs (windows-x64/darwin-x64) hung approximately 65 minutes on infra before completing green. PR #178 touched ZERO .rs files (only pr-manager.md, bats tests, lobster workflow files). The Rust suite was identical to the green develop branch. This is an infra timeout class (likely GitHub Actions runner queue/caching delay) with NO bearing on merge correctness.

**Diagnostic heuristic:** Before investigating a CI hang on cargo-test jobs, check whether the PR diff contains any `.rs` files (`git diff HEAD~1 --name-only | grep '\.rs$'`). If zero Rust files changed, the hang is almost certainly infra, not a test regression. Confirmation: the jobs eventually completed green; no test failures.

**Pattern:** Previously observed in D-512 (rc.19 build infra delay) and D-528 (rc.20 build first attempt). This class of hang occurs when the GitHub-hosted runner capacity is constrained and Rust compile jobs queue up — even when the actual Rust suite passes identically to the prior green HEAD.

**Action:** No action needed. Document as infra-flake class. Do not block merges on cargo-test job duration alone when PR contains zero Rust changes. Monitor if duration approaches 90min (GitHub's default job timeout).

**Anchors:** D-535 (merge-closure burst); PR #178 merge `f6ce4b7c`; D-534 (TDD delivery).

**Cites:** D-535; D-534; D-512 (rc.19 infra delay precedent); D-528 (rc.20 clean first-attempt).

**Closes:** D-535 infra-flake observation capture; issue-128 merge-closure lessons. `[codified]`

---

### L-session-2026-06-10-issue-128-130-delivered-durability

**Category:** session-durability + delivery-cadence + release-gate

**(a) Two issues delivered in a single session (cross-family adversary pattern).** Issues #128 (pr-manager branch-deletion verify, plugin-skill change) and #130 (dispatcher log-shadow, crates+hooks change) were both researched, implemented TDD-first, adversary-reviewed (3-pass convergence), merged, and DELIVERED within a two-day session window (2026-06-09 to 2026-06-10). Both used Gemini Flash (cross-model-family) adversary review per D-386 Option C. Both achieved monotone decay to CLEAN. Pattern: 3-pass cross-family adversary is sufficient for PRs of this scope (plugin/prompt-fix + security-critical guard). The third pass returned only cosmetic findings in both cases.

**(b) Release gate distinction: develop-only vs operator-cache.** Issue #128 is a plugin skill/agent file change (markdown-only); these ship in the next rc release and take effect immediately in develop for any user running from HEAD. Issue #130 is a crates+hooks change (Rust binary + destructive-command-guard.sh); this requires a new rc release for the operator-level marketplace-tarball cache to pick it up. Users on rc.20 will not see the #130 fix until rc.21+. This dual-track release obligation (markdown immediate vs binary gated) is a recurring pattern that must be flagged at every merge with "requires rc release for operator cache."

**(c) pr-description.md durability convention.** The pr-manager authored PR #179's description as code-delivery/issue-130/pr-description.md (16.5KB). This matches the code-delivery/<id>/pr-description.md convention used in prior delivery cycles (code-delivery/F5-B1/, code-delivery/S-15.17/). The file was untracked in .factory/ and committed in D-538 for consistency with the convention. Forward obligation: every pr-manager PR description should be committed to code-delivery/<id>/pr-description.md before the PR is created, so it is part of the audit trail.

**(d) Session-end durability burst timing.** The D-538 SESSION-END DURABILITY BURST was executed immediately after both issue deliveries in the same session, providing a clean zero-context-resume anchor. This is the recommended pattern: when a session delivers ≥1 issue and context may clear (end-of-session, machine change), execute a SESSION-END DURABILITY BURST before clearing. The burst cost (one commit, ~30 min elapsed) is negligible relative to the recovery cost of a stale checkpoint on a new machine.

**Anchors:** D-538 (this burst); D-537 (issue #130 merge); D-535 (issue #128 merge); D-533 (issue-validation sweep). PR #179 `89fbe2d6`; PR #178 `f6ce4b7c`.

**Cites:** D-538; D-537; D-535; D-533; D-386 Option C (convergence model); CLAUDE.md release-gate distinction.

**Closes:** D-538 session-end durability lesson. `[codified]`

---

### L-issue-130-3pass-convergence

**Category:** adversarial-convergence + security-review + spec-drift-routing

**(a) 3-pass fresh-context adversary convergence is sufficient for security-critical guard changes.** Issue #130 adversary ran 3 passes (findings: pass-1 2C+3H+5M+others → pass-2 2C+3H+3M → pass-3 CLEAN 0C/0H/0M). Each pass caught a real regression the prior fix introduced: pass-1 found the multi-target bypass and dedup spec-vs-code drift; pass-2 found `..`-traversal escape under-protect (a guard that allowed traversal above .factory/) and a deeper dedup spec drift; pass-3 returned CLEAN with only cosmetic findings. The security-critical guard (`destructive-command-guard.sh`) withstood fresh-context attack from both under-protect (pass-2 CRIT) and over-block (pass-1 CRIT) directions before achieving CLEAN. Pattern: do NOT accept convergence on security-critical guards until CLEAN; any residual CRITICAL at step N means step N+1 will find a regression. Per D-386 Option C, CLEAN (0C/0H/0M) with only LOW+NIT cosmetic findings constitutes convergence.

**(b) [process-gap] Spec-drift routing obligation codified: when an implementer TDD fix changes behavior an accepted ADR specifies verbatim, the fix-burst MUST route an architect ADR amendment in the SAME burst.** Pass-2 surfaced that the implementer's dedup implementation used a different hash input strategy than ADR-024 Decision 3 specified. The implementer's fix was correct (bounded raw Value::as_str() at char-safe ceiling) but the ADR said '256-byte JSON repr'. This is a spec-drift violation per CLAUDE.md Architectural Authority §12 (spec wins; code must align). Correct routing: implementer finds drift → routes to orchestrator → orchestrator dispatches architect for ADR amendment → ADR amendment in same burst → state-manager records. This was done post-merge for D-537 (ADR-024 v1.0→v1.2). **Forward obligation:** any implementer that changes behavior an ADR specifies verbatim must check the ADR at the end of TDD implementation and flag any spec-drift to the orchestrator before the PR is created. This is now codified in ADR-024 v1.2 Process note so future adversary passes have a machine-checkable anchor.

**(c) Infra-flake recurrence (windows-x64/darwin-x64 cargo-test hang).** PR #179 saw the same ~40-65min hang class on windows-x64/darwin-x64 build-dispatcher cargo-test jobs as PR #178 (D-535 infra-flake observation). PR #179 DID contain Rust changes (log_dir.rs, internal_log.rs, main.rs) — the hang was still infra queue not a test regression (all jobs completed green). Diagnostic: the hang duration is a reliable indicator of GitHub Actions runner queue pressure; the test result itself is the ground truth. Confirmed: same class as D-512 (rc.19) and D-528 (rc.20). No action. Duration 40-65min across two consecutive PRs suggests sustained runner capacity pressure. Monitor if duration approaches 90min (GitHub job timeout).

**Anchors:** D-537 (merge-closure burst); PR #179 merge `89fbe2d6`; ADR-024 v1.2 (process note anchor); D-536 (ADR-024 v1.0 adopted); D-535 (infra-flake class prior observation).

**Cites:** D-537; D-536; D-386 Option C (convergence model); D-471 (asymptotic-acceptance precedent); D-535 (infra-flake class); D-512; D-528.

**Closes:** D-537 lessons-capture; issue-130 3-pass adversary convergence pattern; spec-drift routing obligation codification (S-7.02 cycle-closing checklist process-gap). `[codified]`

---

### L-issue-169-176-worktree-identity

**Category:** adversarial-correctness + multi-family-diversity + tested-helper-extraction + dogfood-discipline

**(a) Multi-family adversarial diversity is load-bearing — one family catches CRITICAL defects the other misses.** The worktree-identity fix used a two-phase adversarial cascade: cross-family Gemini adversary (agy) followed by canonical Claude adversary. Gemini found ~20 real defects across 7 iterations (tautological SHA assertion; @{upstream} pre-push break; double-S story glob; bash-4 ${,,} portability; branch/detached over-gating; multiple false-pass/vacuous tests). The canonical Claude adversary then independently caught a CRITICAL CWD-relative `git rev-parse --git-common-dir` repo-root bug — grounded in the repo's own perf-baseline.bats prior-art — that the Gemini family MISSED entirely. This demonstrates that multi-family diversity is not a redundancy; it is a complementary coverage expansion. Neither family alone is sufficient for prompt-contract + shell-logic combinations. **Forward obligation:** for issues involving both prompt-contract discipline and shell-logic correctness, both a cross-family adversary pass AND a same-family Claude adversary pass are required before declaring convergence.

**(b) Extract mechanical logic to a tested bash helper rather than embedding it inline in markdown prompts.** Static prompt-contract tests (bats tests that grep prompt text) cannot catch shell logic bugs — they only verify the prompt's structural commitments (TD-VSDD-059 class). Issue #169/#176 required `resolve-worktree-identity.sh` to be extracted as a standalone bash-3.2-safe CWD-independent script so that 14 execution bats tests could exercise actual shell logic (SCRIPT_DIR-anchored repo-root resolution; space-safe porcelain parse; anchored basename match preventing S-12.08≠S-12.088 prefix collision; ambiguous→dispatch-error; detached-HEAD accept; SHA normalization). The prompt tests verify the adversary's structural contract (reads identity tuple, uses canonical-rooted paths, runs re-express-not-drop). The execution tests verify the logic is actually correct. These two test classes are non-substitutable. **Forward obligation:** whenever a prompt embeds a bash helper call, extract the helper into `bin/` with a bats execution suite as part of the story delivery, not as a follow-up.

**(c) Running the test suite from a mount-less feature worktree reproduces the #169 phantom-failure class — the test environment dogfoods the fix's own discipline.** During local development, `bats` tests that ran from the feature worktree (`.factory/` not mounted) produced +8 environmental failures ("absent .factory/ spec files"). These failures were not test bugs — they were the bug itself manifesting in test execution. The fix's resolve-worktree-identity.sh correctly identified the feature worktree's canonical repo root and pointed the adversary at the main repo's `.factory/` tree. Post-fix, those 8 failures vanished. This closed the dogfood loop: the fix was validated against the exact failure class it was designed to eliminate. **Pattern:** when a fix addresses "wrong tree reads" in agent context, the bats suite should be run from a fresh worktree (without .factory/ mounted) as part of the verification step.

**(d) The fix dogfooded its own discipline — the adversary reviewed the change using an embedded worktree-identity tuple.** The pr-reviewer reviewing PR #180 operated from the canonical repo root with an explicit 4-field identity tuple (worktree-abs-path, feature-HEAD-SHA `5ea02ecf`, story-id `issue-169-176`, canonical-repo-root). All spec/ADR/BC/VP ground-truth was read from `<canonical-repo-root>/.factory/`. This demonstrates the end-to-end correctness of the worktree-identity preflight protocol: the fix author, the adversary, and the pr-reviewer all operated within the discipline that the fix introduced. The convergence was confirmed at LOCAL 3-CLEAN SHA `5ea02ecf` across two model families.

**Anchors:** D-539 (this burst); PR #180 `0f4793f1`; issues #169 + #176 (both auto-CLOSED on merge); LOCAL 3-CLEAN SHA `5ea02ecf`; `bin/resolve-worktree-identity.sh`; CI run 27309724791 conclusion=success.

**Cites:** D-539; D-537 (prior issue delivery); TD-VSDD-059 (paper-fix detection / tested-helper discipline); D-386 Option C (convergence model); BC-5.39.001 3-CLEAN protocol; CLAUDE.md Canonical Principle Rule 3+4 (fix in scope); CLAUDE.md Agent Routing Table (adversary).

**Closes:** D-539 lessons-capture; issue-169 + issue-176 worktree-identity adversarial-correctness gap; multi-family adversary diversity discipline forward obligation. `[codified]`

---

### L-F2-phantom-field-gate

**Category:** adversarial-convergence + field-removal-verification + closure-gate-discipline + VSDD-process-gap

**(a) A prose-only re-anchor in an ADR does not constitute a field removal — phantom field references in PC/EC/VP tables survive unless EACH site is explicitly swept and a literal-shell closure gate is invoked.** F2 adversarial pass-1 re-anchored ADR-026 away from a phantom `current_wave:` field in its prose rationale sections. However, the Preconditions (PC), Error Cases (EC), and Verification Property (VP) test-fixture tables for VP-081..VP-085 still contained `current_wave:` references at a structural level — struct fields in Rust test fixtures, table cell references, and precondition clauses. The adversary's pass-2 re-read found these surviving instances as BLOCKER/MAJOR findings because they constituted an entire parallel specification substrate that contradicted the ADR prose. **Pattern:** prose re-anchoring and table/fixture sweeping are NON-SUBSTITUTABLE operations. Re-anchoring the narrative does not propagate to structured tables, Rust struct fields, or bats fixture YAML.

**(b) A phantom-field-removal fix MUST be verified with a literal-shell closure gate BEFORE the fix is declared converged.** The S-7.02 cycle-closing checklist analogizes to POLICY 14 leg-4 self-application: just as index-citation changes must self-verify, a field-removal fix must run `grep <removed-field> <affected-files> → 0 matches` as the final verification step in the fix burst. This pass applied that gate inline: the product-owner ran `grep current_wave .factory/specs/` and confirmed 0 load-bearing instances before the commit. This is now the required protocol for any field-removal fix. **Forward obligation (closure-gate protocol):** when a fix removes or re-anchors away from a named field, the fix burst's Dim-2 attestation MUST include literal-shell evidence: `grep -r "<removed-field>" .factory/specs/ → stdout: [0 lines or only negation statements]`. Any matching line that is NOT a negation statement or changelog entry is a closure-gate failure; the fix is not complete.

**(c) The follow-up story for permanent codification (lint/hook gate) should be a draft story, not a tech-debt-register entry, when the enforcement mechanism is machine-checkable.** The Canonical Principle's Rule 3 requires explicit human direction for tech-debt-register deferrals. A phantom-field-removal lint check is implementable in scope: add a `grep <phantom-field>` baseline assertion to the consistency-validator skill's sweep, or as a hook policy. The appropriate deferral vehicle is a draft story (S-18.08) anchored to the E-18 epic, not a vague tech-debt item. This keeps it in the backlog with a concrete spec anchor rather than deferring it to indefinite human re-discovery. The rule: if a process-gap lesson identifies a machine-checkable enforcement mechanism, open a draft story, not a TD register entry.

**Anchors:** D-563 (this burst); F2 adv-pass-2 (root cause analysis: 2 BLOCKER + 4 MAJOR from phantom current_wave: survivors); literal-shell closure gate result: `grep current_wave .factory/specs/` → 0 load-bearing instances; S-18.08 (draft story for permanent lint/hook codification — E-18 epic).

**Cites:** D-563; D-562 (pass-1 partial-fix); S-7.02 (cycle-closing checklist); POLICY 14 leg-4 self-application (index-citation self-verify discipline); CLAUDE.md Canonical Principle Rule 3 (tech-debt-register deferrals require human direction); BC-5.39.001 3-CLEAN protocol.

**Closes:** D-563 process-gap lesson capture; S-7.02 phantom-field-removal verification gate obligation. `[codified]`

**Closes:** D-539 lessons-capture; issue-169 + issue-176 worktree-identity adversarial-correctness gap; multi-family adversary diversity discipline forward obligation. `[codified]`

---

### L-F2-sibling-sweep-tree-wide-gate

**Category:** adversarial-convergence + sibling-sweep-discipline + closure-gate-discipline + VSDD-process-gap

**(a) A rename/sweep fix that reaches only one class of artifact (BCs) but not sibling artifacts (ADR self, DI, VP body/INDEX rows, capabilities, ARCH-INDEX row) will regress in the next adversarial pass.** F2 adversarial passes 1, 2, and 3 all exhibited the same root cause: an INCOMPLETE-SIBLING-SWEEP. Pass-1 re-anchored ADR-026 prose but not PC/EC/VP tables. Pass-2 swept PC/EC/VP tables and BCs but not ADR-026 Decision 2/6/TOML, DI-025, capabilities.md, nor ARCH-INDEX row. Pass-3 caught these remaining sites. **Pattern:** any rename of a canonical field or file path MUST sweep ALL six classes of artifact simultaneously: (1) ADR self (body text + Decision clauses + TOML path_allow blocks), (2) domain-spec invariants, (3) capabilities, (4) VP body + VP-INDEX rows, (5) all BCs, (6) index rows (ARCH-INDEX, BC-INDEX). A fix is not complete unless ALL six classes are verified.

**(b) A tree-wide literal-shell closure gate is the mandatory backstop.** L-F2-phantom-field-gate (D-563) established that closure gates must be run for field-removal fixes. This lesson EXTENDS that to multi-field sibling sweeps: the gate MUST cover the entire specs tree (`grep -rn <old-value> .factory/specs/`) across ALL three axes of the rename: (1) the old field/path literal, (2) the old enum value (e.g., old terminal state), (3) the old version cite (e.g., ADR-026 v1.1 in BC citation fields). Any hit outside historical changelog rows is a closure-gate failure. The three literal-shell grep commands that closed the D-564 pass-3 gate are the canonical template:

```bash
grep -rn 'last-precompact-flush-sha' .factory/specs/
grep -rn 'current_wave' .factory/specs/
grep -rEn 'flush wave-|ADR-026 v1\.(0|1|2)' .factory/specs/
```

Every hit must be either: (a) a historical `last_amended:` / `changelog:` / `Prior:` string, or (b) explicit negation prose ("this field does not exist", "the old form is obsolete").

**(c) Rename/sweep fixes that trigger a pass-N regression MUST prompt the fix-burst executor to run tree-wide grep before staging — not after commit.** The three-pass pattern confirms that per-artifact sweeping by memory is unreliable even for the same executor across multiple passes. The only reliable protocol is the tree-wide pre-staging gate described above. Future fix-bursts that involve ANY rename or value-change across the spec corpus MUST run tree-wide grep and paste captured stdout into the burst-log Dim-2 block BEFORE the `git add` step.

**This lesson REINFORCES L-F2-phantom-field-gate** (closure-gate for field-removal) and BROADENS it: the tree-wide gate requirement applies to ALL rename/sweep operations, not only phantom-field removal. The S-18.08 gate rationale is strengthened by three consecutive passes exhibiting the same miss class.

**Anchors:** D-564 (this burst); F2 adv-pass-3 (root cause: 5 BLOCKER + 4 MAJOR from sibling-sweep miss); D-563 (pass-2 partial sweep), D-562 (pass-1 partial sweep); tree-wide closure gate (3 literal-shell sweeps → 0 load-bearing); S-18.08 (draft story for permanent lint/hook codification — E-18 epic).

**Cites:** D-564; D-563; D-562; L-F2-phantom-field-gate (D-563 closure-gate discipline — reinforced here); S-7.02 (closure-gate checklist); CLAUDE.md Canonical Principle Rule 3 (fix in scope); TD-VSDD-060 (sibling-site sweep on value changes); BC-5.39.001 3-CLEAN protocol.

**Closes:** D-564 process-gap lesson capture; recurring sibling-sweep miss root-cause codification (3-pass pattern). `[codified]`

---

### L-F2-DI-sibling-sweep-unswept-sibling

**Category:** adversarial-convergence + sibling-sweep-discipline + cross-document-propagation + VSDD-process-gap

**(a) A finding that notes a semantic constraint in an ADR (e.g., "WASM plugins cannot exec git") must be propagated to ALL derivative artifacts that codify the same constraint — including domain invariants (DI), not only BCs and VPs.** F2 adversarial pass-4 observation O-P4-004 noted in ADR-026 that WASM plugins read embedded field tokens (no live git exec). The fix burst correctly noted this in ADR-026 body. However, DI-025 in invariants.md still contained two clauses prescribing `git cat-file -t <SHA>` execution inside WASM — a direct contradiction. Pass-5 found F-P5-001 BLOCKER as a result. **Pattern:** when an architectural constraint is updated in an ADR, the propagation sweep MUST include the domain-spec invariants file in addition to BCs, VPs, and capabilities.

**(b) The sweep checklist for ADR amendments must include a 7th class: domain-spec invariants.** The L-F2-sibling-sweep-tree-wide-gate lesson (D-564) established a 6-class sweep requirement. DI-025 violated that lesson because it was not in any of the 6 classes. The 7th class is now mandatory: (7) domain-spec/invariants.md (any DI-NNN that cites the ADR or enforces the same constraint). **Updated sweep checklist:** (1) ADR self, (2) domain-spec/invariants.md, (3) capabilities.md, (4) VP body + VP-INDEX rows, (5) all BCs, (6) index rows, (7) any supporting shell scripts or hooks that implement the constraint.

**Anchors:** D-566 (F2 pass-5 fix burst; DI-025 corrected — WASM static field read semantics); F2 adv-pass-5 F-P5-001 BLOCKER (DI-025 prescribed git exec inside WASM); O-P4-004 (pass-4 observation that triggered this gap); S-18.08 gate rationale REINFORCED (3rd consecutive pass with sibling-sweep miss of a different class).

**Cites:** D-566; D-565 (O-P4-004 origin); L-F2-sibling-sweep-tree-wide-gate (D-564 sweep checklist extended); S-18.08 (draft story); S-7.02 (cycle-closing); BC-5.39.001 3-CLEAN.

**Closes:** D-566 lesson capture; domain-invariant sibling-sweep gap codified as 7th sweep class. `[codified]`

---

### L-F2-ADR-cite-convention-recurring-stale-cite-class

**Category:** adversarial-convergence + bc-traceability-discipline + governance-policy + VSDD-process-gap

**(a) Using a versioned ADR token (`ADR-026 v1.X §Decision N`) as a load-bearing identifier in BC Traceability rows is a recurring defect class that will be flagged in every subsequent adversarial pass until the version token is current.** F2 adversarial passes 3, 4, and 5 all found (among other issues) that BC Traceability ADR cites were version-stale relative to the current ADR version. Each pass forced a batch ADR-cite-version bump across all 8 E-18 BCs. By pass-6, the root cause was recognized as structural: the version token in the cite was volatile, and any ADR amendment would render all BC Traceability rows stale. **Pattern:** volatile version pins in Traceability rows create a mandatory BC-INDEX bump on every ADR amendment, even when the BC's behavioral content is unchanged.

**(b) The correct governance fix is to adopt a stable-anchor form for ADR cites: `ADR-NNN §Decision N` without a version number.** This is the same principle as TD-VSDD-091 (narrative spec content must cite function names + behavioral anchors, not file:NNN line numbers). Applied to ADR cites: the section anchor (`§Decision N`, `§Crash-Consistency Design`) is stable across ADR amendments; the version token is volatile. An informational `(as of vX.Y)` parenthetical is acceptable ONLY if it is syntactically parenthetical and explicitly non-load-bearing. This was codified as POLICY 19 (D-567) and applied tree-wide in the same burst via ADR-026 v1.6 §BC Traceability Cite Convention.

**(c) Fix bursts that amend an ADR MUST sweep ALL BC Traceability rows that cite that ADR and migrate any load-bearing version tokens to stable-anchor form in the SAME burst.** The O-P6-001 process-gap also identified a narrower sibling-miss: the adversary's change-spec block in ADR-026 §F-PX sections names only the BCs being content-changed, but co-named sibling BCs that share the same ADR cite are also stale and need cite migration. The sweep-completeness gate for this class: when a fix burst touches the ADR cite in ANY BC, grep all other BCs that cite the same ADR and migrate the version token in the same burst.

**(d) Disposition — S-18.08 scope extension:** the existing S-18.08 draft story (lint/hook gate for phantom-field removal) should be broadened to include a sweep-completeness gate for ADR-cite migration: the gate must verify that when any BC Traceability ADR cite is migrated, ALL co-citing BCs are swept in the same burst. This is a machine-checkable assertion (`grep "ADR-NNN v[0-9]" .factory/specs/behavioral-contracts/` → 0 matches after a migration burst).

**Anchors:** D-567 (this burst); O-P6-001 (adversary process-gap observation pass-6); ADR-026 v1.6 §BC Traceability Cite Convention; POLICY 19 (adr_version_cite_volatile_pin_prohibition); S-18.08 scope-extension.

**Cites:** D-567; O-P6-001; TD-VSDD-091; POLICY 19; S-18.08; S-7.02; BC-5.39.001 3-CLEAN; L-F2-sibling-sweep-tree-wide-gate (D-564; extended by sweep-class: BC Traceability rows).

**Closes:** D-567 O-P6-001 process-gap lesson capture; recurring stale-ADR-cite class eliminated by governance (POLICY 19 + stable-anchor convention). S-18.08 scope extended (ADR-cite sweep-completeness gate). `[codified]`

---

### L-F2-payload-only-discriminator-recurrence-gate

**Category:** adversarial-convergence + bc-correctness + VSDD-process-gap + governance-policy

**(a) The payload-only-discriminator drift class recurred in F2 adversarial passes 7 AND 8, constituting a 2nd occurrence of the same root-cause pattern.** F-P7-001 (pass-7) and F-P8-001 (pass-8) both found that the WASM gate's EPIC-COMPLETE discriminator violated the pure-parse invariant by referencing external substrate (pass-7: ADR-026 §Decision 2 prescribed reading the prior HANDOFF.md; pass-8: BC-4.14.001 PC2a conjunct referenced "all story entries have terminal status OR cannot be determined" — a substrate-read judgment). Both findings were in the same BC (BC-4.14.001) and the same discriminator clause (PC2a). **Pattern:** a fix burst that corrects an ADR discriminator definition does not automatically propagate that semantic correction to all BCs that implement it; BC PC bodies can retain stale conjuncts that contradict the corrected ADR even after the ADR fix-burst is complete.

**(b) ~~A 3rd occurrence of this pattern would require mandatory hardening into a machine-checkable gate per S-7.02 cycle-closing checklist.~~ → THRESHOLD MET (D-571): F-P10-001 (pass-10) is the 3rd occurrence of the payload-only-discriminator drift class (ADR-026 §Decision 9 wave_id discriminator in BC-4.14.001 PC3/PC4/PC8/Inv3/EC-010/test-vectors still referenced sprint-state.yaml/prior-HANDOFF.md/factory-artifacts substrate after the pass-7 and pass-8 fixes). Per the O-P8-002 codification's own escalation rule, the S-18.08-scoped consistency-validator check is PROMOTED FROM CANDIDATE TO MANDATORY.** The 3rd occurrence has been fixed (BC-4.14.001 v1.8: full PAYLOAD-ONLY discriminator — gate reads `wave_id` from payload exclusively; wave_id==1 → no-op; wave_id absent → FAILS CLOSED with HandoffIncomplete: ['wave_id']; all sprint-state.yaml/prior-HANDOFF.md/factory-artifacts framing removed from gate behavior across all PCs/ECs/Inv/test-vectors). VP-INDEX VP-083/VP-081 rows synced to payload-only framing (D-571 state-manager cross-doc sync). **MANDATORY gate: S-18.08 must ship the consistency-validator pure-parse invariant check as a HARD REQUIREMENT** — not optional. For any BC with Invariant 1 "pure-parse / no-filesystem," no PC/EC/Inv may reference reading external substrate (sprint-state.yaml, prior HANDOFF.md, git log) for the gate's own behavior. The machine-checkable assertion: `grep -n "sprint-state\|HANDOFF\.md\|git.*log\|git.*cat-file" <BC-with-pure-parse-invariant>` → 0 load-bearing hits in PC/EC/Inv sections (changelog/Traceability/caller-attribution notes excepted).

**(c) S-18.08 scope extension (O-P8-002 disposition) — consistency-validator pure-parse invariant check: MANDATORY (escalated from candidate at 3rd occurrence).** The existing S-18.08 draft story (lint/hook gate for phantom-field removal and ADR-cite sweep completeness) MUST include a third gate as a HARD REQUIREMENT: when a BC body contains `Invariant 1: pure-parse` or equivalent "no filesystem read" wording, the consistency-validator MUST verify that no PC/EC/Inv in that BC references reading substrate fields (sprint-state.yaml entries, prior HANDOFF.md contents, git log, or similar). Machine-checkable: `grep -n "sprint-state\|HANDOFF.md\|git.*log\|git.*cat-file" <BC-with-pure-parse-invariant>` should return 0 load-bearing hits in PC/EC sections (excluding Traceability/Invariant-definition sections and explicit caller-attribution notes). This gate was CANDIDATE at 2nd occurrence (D-569); it is MANDATORY at 3rd occurrence (D-571) per S-7.02 cycle-closing checklist.

**(d) [D-571 update] 3rd-occurrence disposition.** F-P10-001 (pass-10 MAJOR) is formally the 3rd occurrence. BC-4.14.001 v1.7→v1.8 closed the specific finding. VP-INDEX VP-083 and VP-081 rows synced. O-P8-002 candidate gate escalated to MANDATORY per this lesson's own escalation rule. S-18.08 gate is now a hard requirement. No further observation or process-gap entry needed — the escalation is self-executing per the existing codification rule.

**(e) [D-572 update] Layer-propagation: BC→VP-INDEX-row→VP-body. O-P8-002 gate scope EXTENDED to VP files.** F-P11-001 (pass-11 MAJOR) and F-P11-002 (pass-11 MEDIUM) reveal a new layer-propagation failure mode: the payload-only-discriminator drift class had propagated through THREE artifact layers before full closure: (1) **BC layer** (pass-7+8+10: BC-4.14.001 PC bodies); (2) **VP-INDEX row layer** (pass-10 D-571: VP-083/VP-081 row descriptions synced to payload-only); (3) **VP file BODY layer** (pass-11 D-572: VP-083/VP-081 Property/Postcondition/fixture sections still described the gate behavior using substrate reads — prior-HANDOFF.md presence, sprint-state.yaml ordinal — rather than the PAYLOAD-ONLY discriminator). Each fix burst only propagated the correction one layer at a time.

**Root cause of layer-propagation:** After D-571 corrected the BC bodies and VP-INDEX rows, the VP file BODIES retained pre-correction descriptions that described what the SHELL CALLER reads rather than what the WASM gate reads from its payload. These were structurally distinct documents (not just a row in an index) and required their own targeted sweep. The adversary's O-P8-002 tree-wide gate at D-571 ran against BC files and VP-INDEX rows but was not yet extended to VP file bodies.

**Extension of O-P8-002 gate scope (D-572 disposition):** The mandatory S-18.08 consistency-validator gate (from D-571 escalation) MUST now ALSO verify that VP files whose `source_bc` or `bcs[]` frontmatter includes a pure-parse BC do not describe THAT BC's gate behavior via external-substrate reads in the Property/Postcondition/Invariant/fixture sections. Machine-checkable extension: `grep -nE 'sprint-state|prior HANDOFF|wave-group ordinal'` in VP Property/Postcondition/fixture body sections → 0 load-bearing hits (shell-caller-attribution explicitly labeled is OK; historical changelog entries exempt). This closes the layer-propagation gap: the gate now covers BC bodies, VP-INDEX rows, AND VP file bodies as a unified scope.

**Disposition of F-P11-001 and F-P11-002:** VP-083 v1.2→v1.3 (F-P11-001 MAJOR): Property §1 and all GateContext/fixture comments rewritten to PAYLOAD-ONLY. VP-081 v1.2→v1.3 (F-P11-002 MEDIUM): Precondition rewritten to clarify shell-caller/WASM-gate division; Postcondition E + integration fixture added. ADR-026 v1.9 verified UNCHANGED (tree-grep PASS: all substrate references in VP-081 are explicitly labeled "SHELL CALLER substrate" in Precondition body and fixture comments — this is the shell-caller-attribution explicitly labeled form, which is OK). VP-INDEX v2.15→v2.16. 3-CLEAN streak remains 0/3 (pass-11 MAJOR → streak reset; pass-12 = next clean-shot).

**Anchors:** D-572 (this update; layer-propagation discovery; O-P8-002 gate extension to VP files; F2 pass-11 fix); D-571 (3rd-occurrence escalation to MANDATORY; F2 pass-10 fix); D-569 (2nd-occurrence burst; F2 pass-8 fix); O-P8-002 process-gap candidate (adversary pass-8); F-P11-001 (MAJOR; VP-083 body payload-only sweep); F-P11-002 (MEDIUM; VP-081 body shell-caller/WASM-gate clarification); F-P10-001 (MAJOR; 3rd occurrence; BC-4.14.001 v1.8 full PAYLOAD-ONLY closure); BC-4.14.001 Invariant 1; VP-083 v1.3 (corrected body); VP-081 v1.3 (corrected body); VP-INDEX v2.16; S-18.08 (gate scope extended: BCs + VP files).

**Cites:** D-572; D-571; D-569; O-P8-002; F-P11-001; F-P11-002; F-P10-001; F-P8-001; F-P7-001; D-568 (pass-7 fix); BC-4.14.001 Invariant 1; ADR-026 §Decision 9; S-7.02; S-18.08; BC-5.39.001 3-CLEAN.

**Closes:** D-572 layer-propagation sweep (VP body layer closed; O-P8-002 gate extended to VP files; S-18.08 scope covers BCs + VP files); D-571 O-P8-002 escalated to MANDATORY. `[codified; escalated-to-mandatory; gate-scope-extended-to-vp-files]`

---

### L-F2-cross-reference-title-code-sweep

**Category:** adversarial-convergence + sibling-sweep-discipline + cross-document-propagation + VSDD-process-gap [process-gap]

**(a) When a spec fix changes an artifact's TITLE, ERROR-CODE, or canonical PHRASE, the fix-burst MUST grep tree-wide (.factory/specs/) for the OLD value across cross-referencing sections and sweep ALL stranded sites in the same burst.** Cross-referencing sections include: BC §VP Anchors title cites, ADR §Decision headings, §BC-Traceability tables, and §Risk rows. This class of finding has now recurred 4 times in F2 E-18 adversarial passes:
- **Occurrence 1 (D-577, F-P14-re-sweep):** ADR-026 §Decision-2 heading retained old framing after a BC title changed; stranded cross-cite found by pass-14 consistency re-sweep.
- **Occurrence 2 (F-P19-001, pass-19):** BC-4.14.001 and BC-5.41.001 §VP Anchors sections retained the stale VP-083 title ('and HANDOFF.md Absent') after VP-083's H1 was corrected (F-P15-004) in pass-15 fix burst. The fix burst that corrected the VP title did not grep for the OLD title string across BC §VP Anchors sections.
- **Occurrence 3 (F-P19-002, pass-19):** ADR-026 §Decision 9 heading retained the framing 'HANDOFF.md Absent' after BC-5.41.001's error-code contract was defined in pass-18. The ADR §Decision 9 heading and §BC-Traceability + §Risk F1-R3 rows were not swept when the BC was amended.
- **Occurrence 4 (F-P26-001, pass-26):** BC-7.07.001 §VP Anchors VP-085 title cite was truncated to 'PreCompact Flush Hook Is Hermetic' — missing the second clause '— Reads STATE.md+Git Only, Never In-Context Reasoning'. VP-085 H1 was set to the full verbatim title in an earlier pass, but the BC cite was never grepped exhaustively for the partial match. D-582 codified the reactive one-site-at-a-time class; D-589 ran the FIRST exhaustive sweep across all 8 BCs' §VP Anchors and confirmed BC-7.07.001 was the only remaining stale site — class now fully closed.

**(b) The sweep checklist for any spec fix that changes a TITLE, ERROR-CODE, or canonical PHRASE must include a dedicated cross-reference sweep as step zero of the fix burst.** Before composing the fix, grep `.factory/specs/` for the OLD value (case-insensitive) and enumerate all hits in BC §VP Anchors, ADR §Decision headings, §BC-Traceability tables, and §Risk rows. All stranded sites found MUST be corrected in the same commit as the primary fix. The cross-reference sweep checklist is:
1. `grep -r "<OLD_TITLE_or_PHRASE>" .factory/specs/` — enumerate all cross-citing artifacts.
2. For each hit: update the cite to the NEW canonical form.
3. For each artifact updated: bump its version per POLICY 14 5-leg parity (version, changelog, modified[], last_amended, upstream-index row).
4. Run POLICY 14 4-index gate after all bumps.

**(c) UPGRADED (D-589 4th recurrence): Title-cite-parity gate — MANDATORY positive-coverage check for VP-NNN title cites.** Reactive one-site-at-a-time fixing (4 recurrences across F2 passes) is insufficient. For EVERY cite of the form `VP-NNN — <title>` or `VP-NNN (<title>)` in any BC §VP Anchors / §Traceability section AND any ADR §VP Allocations table, a grep-based check MUST assert that `<title>` equals the cited VP file's H1 verbatim. The gate is:

```bash
# For each VP-NNN title cite site in the spec corpus:
cited_title="<title as written in the BC/ADR cite>"
vp_h1=$(grep "^# VP-NNN" .factory/specs/verification-properties/VP-NNN.md | sed 's/^# //')
[ "$cited_title" = "$vp_h1" ] || echo "FAIL: VP-NNN title mismatch in <BC/ADR>: cited='$cited_title' actual='$vp_h1'"
```

This gate MUST run exhaustively across ALL cite sites every spec-touch — not only the file being amended. D-589 ran the first exhaustive sweep (all 8 BCs' §VP Anchors + all ADR §VP Allocations rows) and confirmed BC-7.07.001 VP-085 was the ONLY remaining stale cite — stale-VP-title-cite class now closed. **The gate is anchored to S-18.08 consistency-validator as a MANDATORY scope extension** (alongside the phantom-field-removal lint and subsystem-anchor-sweep gates already codified).

**(d) Candidate scope extension for POLICY 5 sibling-sweep and S-18.08 gate.** POLICY 5 currently mandates sibling-sweep for: (i) function signatures, (ii) constant names, (iii) canonical identifier renames. This recurrence class (4 occurrences) warrants adding **category (i) extension: cross-reference title/code/phrase cites** — when a VP H1, BC error-code, or ADR §Decision heading is changed, the sweep MUST cover BC §VP Anchors, ADR §Decision, §BC-Traceability, and §Risk sections. Machine-checkable: `grep -r "<OLD_VALUE>" .factory/specs/` → 0 hits after fix-burst commit. S-18.08 consistency-validator gate scope MUST be extended to include: automated verification that when a VP title or BC error-code is amended, no BC §VP Anchors, ADR §Decision heading, or §Risk row retains the old value. The title-cite-parity gate (clause (c) above) is the positive-coverage complement: it checks that all current cites match the current VP H1, not only that the old value is absent.

**Anchors:** D-589 (4th recurrence + TITLE-CITE-PARITY-GATE UPGRADE; F-P26-001 MED mis-anchor BC-7.07.001 VP-085 truncated cite; exhaustive all-8-BC sweep; stale-VP-title-cite class CLOSED; S-18.08 MANDATORY gate scope extension); D-582 (original codification; 3rd occurrence; F2 pass-19 NOT-CLEAN FIX BURST + CROSS-REF-SWEEP CODIFICATION); F-P26-001 (pass-26 MED: BC-7.07.001 §VP Anchors VP-085 truncated title cite); F-P19-001 (pass-19 MED: BC-4.14.001 + BC-5.41.001 §VP Anchors stale VP-083 title cite); F-P19-002 (pass-19 MED: ADR-026 §Decision 9 BC↔ADR contradiction); D-577 (occurrence 1: ADR-026 §Decision-2 pass-14 re-sweep); L-F2-sibling-sweep-tree-wide-gate (D-564; foundational sibling-sweep discipline); POLICY 5 v1.3.x sibling-sweep mandates; S-18.08 (draft gate story; MANDATORY scope extension); E-18 F3 (follow-up gate story anchor).

**Cites:** D-589; D-582; F-P26-001; F-P19-001; F-P19-002; D-577; POLICY 5; S-18.08; S-7.02; BC-5.39.001 3-CLEAN; L-F2-sibling-sweep-tree-wide-gate (D-564).

**Closes:** D-589 upgrade: 4th recurrence; exhaustive sweep run; stale-VP-title-cite class closed; title-cite-parity grep gate codified as MANDATORY positive-coverage gate; S-18.08 consistency-validator scope extension anchored (MANDATORY). D-582 original codification: cross-reference-title-code-sweep gap codified as POLICY 5 category (i) extension candidate + S-18.08 gate scope extension candidate. Drift Item updated E-18 F3 (S-18.08-class gate story). `[codified; process-gap; candidate-policy-extension; title-cite-parity-gate-MANDATORY]`

---

### L-F2-subsystem-anchor-sweep

**Category:** adversarial-convergence + sibling-sweep-discipline + subsystem-anchor-propagation + VSDD-process-gap [process-gap]

**(a) When a VP/BC subsystem anchor changes (e.g., a WASM plugin re-anchored SS-07→SS-04) OR a capability's subsystem footprint is referenced, the fix-burst MUST sweep ALL VPs sharing a source BC AND the L2-INDEX Subsystem Cross-Walk AND Document Map — a single-site fix strands siblings.** This pattern has recurred twice in F2 E-18 adversarial passes:

- **Occurrence 1 (F-P16-001 VP-084 → F-P21-002 VP-081 stranded sibling):** D-579 fix burst corrected VP-084 scope SS-05,SS-07→SS-05,SS-04 (validate-burst-log WASM mis-anchor). The fix-burst swept VP-080..086 for "remaining mis-anchors" but VP-081's scope SS-05,SS-06,SS-07 was not correctly evaluated — the comprehensive sibling sweep should have caught VP-081 at the same burst (VP-081 and VP-084 share BC-4.14.001 and BC-5.41.001 as source BCs; both gate via WASM plugins). F-P21-002 (pass-21) found the stranded VP-081 mis-anchor 5 passes later.

- **Occurrence 2 (F-P20-001 Document Map fix → F-P21-001 Cross-Walk stranded):** D-583 fix burst corrected L2-INDEX Document Map capabilities.md CAP range (F-P20-001). The Document Map fix verified the capabilities count was correct but did not trigger a sweep of the Subsystem Cross-Walk table — capabilities.md v1.4 had added CAP-032 to subsystems SS-01, SS-04, SS-05, SS-06, SS-07 (at D-540 / F-14 fix), but the Cross-Walk only received CAP-032 in SS-05, SS-06, SS-07 (missing SS-01 and SS-04). F-P21-001 (pass-21) found the Cross-Walk gap 1 pass later.

**(b) The sweep checklist for any fix that changes a VP/BC subsystem anchor, OR that adds/modifies a capability's Subsystems: line, MUST include a dedicated subsystem-anchor sibling sweep as step zero of the fix burst.** Before composing the fix:
1. Identify ALL VPs whose `source_bc` or `bcs[]` frontmatter includes ANY BC that declared the affected subsystem(s).
2. Verify EACH identified VP's `scope:` field matches the WASM-vs-bash determination for that VP's proof harness.
3. Verify the L2-INDEX Subsystem Cross-Walk row for EACH affected SS-NN lists all CAPs that declare that subsystem in their capabilities.md `Subsystems:` line.
4. Verify the L2-INDEX Document Map version annotations are consistent with the Cross-Walk.
5. Run POLICY 14 4-index gate after all bumps.

**(c) Candidate POLICY 5 sibling-sweep category (j) addition: subsystem-anchor cites across VPs-sharing-source-BC + L2-INDEX Cross-Walk/Document Map.** POLICY 5 currently mandates sibling-sweep for: function signatures, constant names, canonical identifier renames, cross-reference title/code/phrase cites (category i). This recurrence class (2 occurrences) warrants adding **category (j): subsystem-anchor changes** — when a VP scope field is corrected or a capability's Subsystems: line is modified, the sweep MUST cover ALL sibling VPs with overlapping source-BC set AND the L2-INDEX Subsystem Cross-Walk rows for ALL affected SS-NN entries. Machine-checkable: after any scope-field change in a VP file, `grep -l "source_bc: \"BC-NNN\|BC-NNN" .factory/specs/verification-properties/VP-*.md` → verify all returned VP files have correct scope; `grep "SS-NN" .factory/specs/domain-spec/L2-INDEX.md` → verify Cross-Walk row lists all capabilities.md-declared CAPs for that SS.

**(d) S-18.08 gate scope extension candidate.** The S-18.08 draft story (lint/hook gate for phantom-field removal and ADR-cite sweep completeness) SHOULD be further extended to include: (i) verification that when any VP scope: field is modified, all sibling VPs in the same BC-cluster are swept in the same burst; (ii) verification that when capabilities.md Subsystems: lines change, the L2-INDEX Subsystem Cross-Walk is swept in the same burst. These are machine-checkable assertions that fit naturally in S-18.08's consistency-validator scope.

**Drift Items row:** Anchor E-18 F3 — subsystem-anchor-sweep sibling-discipline gate story (S-18.08 scope extension or dedicated S-18.NNN). Implementation: consistency-validator check that VP-cluster scope changes trigger Cross-Walk audit.

**Anchors:** D-584 (this burst; 2nd recurrence; F2 pass-21 NOT-CLEAN FIX BURST + SUBSYSTEM-ANCHOR-SWEEP CODIFICATION); F-P21-002 (MEDIUM; VP-081 scope mis-anchor fixed; comprehensive VP-080..086 sweep confirms sole remaining); F-P21-001 (MEDIUM; L2-INDEX Cross-Walk CAP-032 added to SS-01+SS-04; full Cross-Walk audit run); F-P16-001 (pass-16; VP-084 scope fix — occurrence 1); F-P20-001 (pass-20; Document Map fix — occurrence 1 for Cross-Walk vs Document Map); D-579 (D-579 VP-084 fix burst — stranded VP-081 sibling); D-583 (Document Map fix — stranded Cross-Walk); L-F2-sibling-sweep-tree-wide-gate (D-564; foundational sibling-sweep discipline); POLICY 5 v1.3.x sibling-sweep mandates; S-18.08 (scope extension candidate); E-18 F3 (follow-up gate story anchor).

**Cites:** D-584; F-P21-001; F-P21-002; F-P16-001; F-P20-001; D-579; D-583; POLICY 5; S-18.08; S-7.02; BC-5.39.001 3-CLEAN; L-F2-sibling-sweep-tree-wide-gate (D-564); L-F2-cross-reference-title-code-sweep (D-582).

**Closes:** D-584 lesson capture; subsystem-anchor-sweep gap codified as POLICY 5 category (j) extension candidate + S-18.08 gate scope extension candidate. Drift Item anchored E-18 F3. `[codified; process-gap; candidate-policy-extension]`

---

### L-F2-canonical-scope-verification

**Category:** adversarial-convergence + canonical-scope-reconciliation + field-4-provenance [process-gap]

**(a) When a specification describes a field that can be produced by multiple code paths (shell hook vs. WASM plugin), each code path MUST be independently scoped against the canonical definition before any fix burst declares the field "reconciled."** The field-4 provenance gap (F-P24-001/F-P24-002 MEDIUM) in F2 E-18 pass-24 arose because earlier fix bursts (D-572, D-573) over-corrected: the "no-git-exec" invariant was written as a universal prohibition (canonical option A), when the correct scope is WASM-ONLY (canonical option B). The shell hook (`precompact-flush.sh`) has always been permitted to exec `git cat-file -t SHA_B` at write time — the invariant applies to WASM reads of already-persisted log fields, not to the shell that writes them.

**(b) Root-cause pattern: spec-layer invariants written in scope-imprecise language propagate through 3+ files (invariants.md, VP-082, BC-5.41.003, ADR-026) before the imprecision surfaces under adversarial review.** Once a scope-imprecise invariant is committed, every downstream artifact that cites it inherits the ambiguity. The fix requires a coordinated multi-artifact sweep across all layers that cited the ambiguous formulation:
1. Domain invariant (invariants.md DI-022) — update to scope-precise language
2. Verification properties (VP-082 PC-A, VP-084 if affected) — align field description
3. Behavioral contracts (BC-5.41.003 F-8 note) — align shell vs WASM scope boundary
4. ADR (ADR-026 Decision 6 step ordering and §VP Allocations table) — align titles and sequencing
5. L2-INDEX Document Map — version-sync downstream citation

**(c) Prevention heuristic: when authoring an invariant that constrains a field written by one agent type and read by a different agent type (e.g., shell writes / WASM reads), the invariant MUST explicitly name the scope boundary.** Pattern: `"[consumer-type] MUST NOT exec git at read time; [producer-type] MAY exec git at write time"` rather than the scope-ambiguous `"no git-exec for field-4"`. The scope boundary makes the invariant machine-checkable (grep for `git cat-file` in WASM source vs shell source separately).

**(d) Canonical option (B) reconciliation as the F2 E-18 closure.** After 24 adversarial passes on E-18 / CAP-032 context-durability (GitHub issue #173), the field-4 scope boundary is now canonical: shell hook MAY exec `git cat-file -t SHA_B` at write time (or embed literal `commit` token directly; both yield `commit` token in field-4); WASM reads field-4 STATICALLY from persisted log without git-exec; v1.16 BLOCKER scope is WASM-ONLY. This closes the scope-imprecise variant (A) introduced at D-572/D-573 and propagated through 5 adversarial passes before disambiguation (F-P24-001/F-P24-002).

**(e) S-7.02 Defensive Sweep Discipline applicability.** This lesson extends S-7.02 beyond count-propagation to include invariant-scope-propagation: when an invariant's scope changes (e.g., "universal no-git-exec" → "WASM-only no-git-exec"), the same defensive sweep protocol applies — grep for the old scope language across ALL artifacts that cited it, update each site, log in commit message.

**Drift Items row:** Anchor E-18 F3 — canonical-scope-verification discipline gate: when authoring invariants with multi-agent-type scope boundaries (shell vs WASM, producer vs consumer), require explicit scope-boundary language; consistency-validator MAY check for ambiguous constructs ("no-git-exec" without agent-type qualifier in spec prose adjacent to precompact-flush or WASM-hook context).

**Anchors:** D-587 (this burst; F2 pass-24 NOT-CLEAN COMPREHENSIVE CLEANUP; field-4 canonical (B) reconciliation); F-P24-001 (MEDIUM; VP-082 PC-A field-4 provenance ambiguity — canonical (B) confirmed); F-P24-002 (MEDIUM; ADR-026 Decision 6 step-ordering + VP Allocations table); D-572/D-573 (original over-correction introducing scope-imprecise "no-git-exec" universal formulation); BC-5.39.001 3-CLEAN streak RESET 2/3→0/3 at pass-24; S-7.02 (defensive sweep discipline); E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-587; F-P24-001; F-P24-002; D-572; D-573; VP-082; BC-5.41.003; ADR-026; invariants.md DI-022; S-7.02; BC-5.39.001; E-18; L-F2-subsystem-anchor-sweep (D-584; sibling sweep discipline companion lesson).

**Closes:** D-587 lesson capture; canonical-scope-verification process-gap codified; field-4 (B) reconciliation anchored across invariants.md + VP-082 + BC-5.41.003 + ADR-026 + L2-INDEX. `[codified; process-gap; canonical-scope-boundary]`

---

### L-F2-stale-term-deferral-unsafe

**Category:** adversarial-convergence + stale-term-remediation + deferral-discipline [process-gap]

**(a) Stale-term residue in normative present-tense specification prose MUST be FIXED IN-SCOPE; deferring it as a non-blocking LOW finding is unsafe.** Evidence from F2 E-18 passes 29/30/31: the stale "side-channel" adjective on "precompact-flush-log" was first surfaced as observation O-P29-001 (LOW) and again as finding F-P30-001 (LOW) under the freeze-and-defer strategy applied at pass-30 (streak 2/3 CLEAN). At pass-31, a fresh-context adversary reclassified the SAME normative present-tense residue as a load-bearing MEDIUM finding (F-P31-001) because the present-tense prose implied an architectural property (a "side-channel" is a distinct parallel channel) that contradicted the canonical term ("precompact-flush-log" is an append-only log, not a side-channel). The MEDIUM reclassification RESET the 3-CLEAN streak from 2/3 to 0/3.

**(b) Root-cause pattern: fresh-context adversaries calibrate severity independently.** A deferred LOW finding is NOT guaranteed to stay LOW on the next pass. Fresh adversaries receive no prior-pass severity context and apply the canonical-principle production-grade lens independently. A stale term that one pass considered "cosmetic" (LOW) can be re-classified as "load-bearing" (MEDIUM) by the next fresh adversary if the normative present-tense context makes the stale term architecturally misleading. The 2/3 streak was earned on a package that still contained a known defect — this is structurally fragile.

**(c) Deferral risk matrix for stale-term findings:**
| Stale-term context | Deferral risk |
|--------------------|--------------|
| Historical/changelog prose only | LOW — adversaries typically accept "prior design documented" justification |
| Comment or non-normative note | MEDIUM — depends on whether adversary reads it as normative |
| Normative present-tense spec prose (PC, EC, Inv, Property Invariant) | HIGH — must fix in-scope; do not defer |
| Normative present-tense ADR Decision body | HIGH — must fix in-scope; do not defer |

**(d) Rules derived from this lesson:**
1. **Never defer stale-term/normative-residue findings.** Fix them in the same burst as their discovery pass. If the architecture change has a companion term change (e.g., "side-channel file" → "precompact-flush-log"), the term change MUST be applied exhaustively to ALL normative present-tense sites before the fix burst closes.
2. **Stale-term sweeps MUST be exhaustive.** Run a corpus-wide grep for the stale term across all normative files (BCs, VPs, ADRs, invariants.md, domain-spec). Do not assume "only one site" based on the adversary's observation count — fresh adversaries may not enumerate all sites.
3. **The 3-CLEAN streak is earned on the ENTIRE visible package.** A deferred "cosmetic" finding poisons the package for all subsequent passes. The incremental cost of fixing a stale term in-scope (minutes) is always less than the streak-reset cost (one full adversary pass + re-fix burst + re-pass).
4. **Strategy correction after streak reset: clear the backlog.** When a streak resets due to a deferred finding re-escalating, the correct recovery is to clear the ENTIRE deferred backlog (all deferred LOWs and observations) before dispatching the next adversary. Passing on a package with known deferred issues is a false economy.

**(e) Mechanical gate candidate (S-18.08 scope).** A stale-term detector could be encoded as a WASM hook or consistency-validator rule: given a list of retired terms (e.g., "side-channel" adjacent to "precompact", "SHA file", "precompact-flush-sha") and the set of normative-present-tense prose files, flag any occurrence as BLOCK (not advisory). This gate was not available during F2 E-18 and the gap is the root cause of the 3-pass deferral cycle. Candidate scope: S-18.08 mechanical stale-term gate, per Drift Item below.

**Drift Items row:** Anchor E-18 F2 pass-31 — stale-term-deferral-unsafe: stale terms in normative present-tense prose MUST be fixed in-scope; sweep must be exhaustive; deferral as LOW is a convergence risk. Candidate S-18.08 mechanical-gate: WASM stale-term detector for retired-terminology list against normative BC/VP/ADR prose; would have blocked F-P31-001 class at its origin (pass-29).

**Anchors:** D-594 (this burst; F2 pass-31 NOT-CLEAN FULL BACKLOG CLEARANCE; strategy correction: clear entire deferred backlog); F-P31-001 (MEDIUM; VP-085 Property Invariant item 3 'append-only side-channel log'→'append-only log'; streak RESET 2/3→0/3); F-P31-002 (MEDIUM; BC-7.07.001 PC8 'SHA file' stale term); F-P30-001 (LOW DEFERRED → re-escalated to F-P31-001 MEDIUM); O-P29-001 (LOW OBS; original stale-term sighting); BC-5.39.001 (3-CLEAN streak rule); ADR-026 v1.18 (§Decision 6 step 5 'append-only side-channel log'→'append-only log' — companion normative site); VP-085 v1.7 (F-P31-001 fix); E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-594; F-P31-001; F-P31-002; F-P30-001; F-P30-003; O-P29-001; O-P29-002; O-P29-003; BC-5.39.001; VP-085 v1.7; ADR-026 v1.18; BC-7.07.001 v1.9; BC-5.41.001 v1.15; BC-5.41.002 v1.7; E-18; L-F2-canonical-scope-verification (sibling lesson — scope-precise invariant companion).

**Closes:** D-594 lesson capture; stale-term-deferral-unsafe process-gap codified; FULL BACKLOG CLEARANCE strategy validated — package now zero-known-findings for pass-32 dispatch. `[codified; process-gap; stale-term-sweep; deferral-discipline]`

---

### L-F2-annotation-must-be-self-contained

**Category:** adversarial-convergence + annotation-discipline + sibling-sweep-hygiene [process-gap]

**(a) Documentary annotation/marker text MUST be SELF-CONTAINED — never embed an enumerated factual claim (e.g., "only BC-X and BC-Y changed") that can be stale or wrong.** Evidence: the F-P33-001 v1.5-skip-marker sweep (pass-33) added identical text to BC-5.41.001, BC-5.41.002, BC-6.24.001, and BC-7.07.002 claiming "only BC-4.14.001 and BC-7.07.001 received behavioral changes at the pass-5 burst." That enumeration was a false premise — BC-5.41.003 DID receive a behavioral change at v1.5 (F-P5-002 re-grounding). A fresh-context adversary at pass-35 caught this false premise (F-P35-002 MEDIUM), resetting the 3-CLEAN streak from 1/3 to 0/3. The orchestrator supplied the marker text; the sweep applied it verbatim across 4 BCs without verifying the premise.

**(b) Root-cause: two compounding errors.**
1. **Premise-error propagation:** The skip-marker text assumed "only BC-4.14.001 and BC-7.07.001 changed at pass-5" but did not verify this against BC-5.41.003's own history. A sibling-sweep that applies orchestrator-authored marker text verbatim without independent verification of the factual claims in that text is unsafe — the orchestrator's knowledge of the burst scope can be incomplete.
2. **Structural parity gap not caught by sweep:** The same pass-33 sibling-sweep claimed exhaustive 8-BC coverage ("8-BC sweep; class CLOSED"), but BC-5.41.003 lacked the §Changelog section needed to HOST a skip-marker. The sweep verified presence of v1.5-skip-markers across 4 BCs but did not verify that ALL 8 BCs had the structural §Changelog section. A fresh adversary at pass-35 found the missing §Changelog (F-P35-001 MEDIUM).

**(c) Two-part rule:**

Rule 1 — **Marker/annotation text must state the LOCAL fact only.** Canonical self-contained form: "This BC received no behavioral change at the [burst/pass N] coordinated burst; the version gap [vX→vY] is deliberate." Do NOT enumerate sibling BC IDs that changed — that is a cross-BC factual claim that can be wrong. The local fact (this BC changed/did not change) is verifiable per-file; the enumeration of siblings is not.

Rule 2 — **Structural-parity sweeps MUST VERIFY the section EXISTS before claiming exhaustive coverage.** If a sweep claims "all 8 BCs have §Changelog" or "class CLOSED," the sweep MUST have confirmed the structural section (the target hosting container) exists in each sibling, not just confirmed the sub-content inside the section. F-P33-001 closed a "skip-marker class" but did not verify that all 8 sibling BCs had a §Changelog to host any marker — the structural gap (BC-5.41.003 missing §Changelog entirely) was the root cause of F-P35-001.

**(d) This lesson extends and deepens two prior lessons.** L-F2-stale-term-deferral-unsafe taught that normative-prose residue deferred as LOW will re-escalate at subsequent passes. L-F2-cross-reference-title-code-sweep taught that sibling sweeps must be exhaustive at every cite site. This lesson adds: sweeps that apply AUTHOR-SUPPLIED TEXT (not just version-bump changes) must independently verify (a) the structural container exists, and (b) any factual enumeration in the text is true at the time of application.

**(e) Fix-induces-finding pattern (3rd self-inflicted reset).** Both F-P35-001 and F-P35-002 were direct side-effects of the F-P33-001 skip-marker sweep. This is the 3rd consecutive streak reset caused by a fix-induced finding class (F-P32-001 class at pass-32; F-P33-001 class at pass-33; F-P33-001 sweep false-premise at pass-35). Pattern: each sweep that closes a class must independently verify it has not introduced a new factual error class in the text applied.

**(f) Mechanical gate candidate (S-18.08 scope).** A consistency-validator gate could enforce: for any BC carrying a skip-marker annotation in §Changelog, grep the annotation text for BC-ID enumerations (pattern `BC-[0-9]+\.[0-9]+\.[0-9]+`); if found, BLOCK with message "skip-marker annotation enumerates sibling BC IDs — use self-contained local-fact form only (L-F2-annotation-must-be-self-contained)." This gate would have blocked F-P35-002 at the write-time of the D-596 skip-marker sweep.

**Drift Items row:** Anchor E-18 F3 — annotation-must-be-self-contained: structural-parity sweeps must verify the §Changelog section EXISTS before claiming exhaustive coverage; marker/annotation text must not enumerate sibling BC IDs. Candidate S-18.08 gate scope extension: consistency-validator checks skip-marker annotations for cross-BC enumeration (BC-ID enumeration in annotation text = BLOCK). Extends L-F2-stale-term-deferral-unsafe + L-F2-cross-reference-title-code-sweep.

**Anchors:** D-598 (this burst; F2 pass-35 NOT-CLEAN FIX BURST; 2 MEDIUM self-inflicted; streak 1/3→0/3 RESET); F-P35-001 (MEDIUM; BC-5.41.003 v1.7→v1.8 — §Changelog section was structurally absent; type-parity gap per POLICY 17; 9 rows transcribed from modified[]); F-P35-002 (MEDIUM; BC-5.41.001 v1.17/BC-5.41.002 v1.10/BC-6.24.001 v1.9/BC-7.07.002 v1.11 — v1.5 skip-marker de-enumerated; false premise "only BC-4.14.001 and BC-7.07.001 changed at pass-5" removed; BC-5.41.003 DID change at v1.5 per F-P5-002); F-P33-001 (the original sweep whose marker text carried the false premise); BC-5.39.001 (3-CLEAN streak rule; 1/3→0/3 RESET); E-18 (CAP-032 context-durability; GitHub issue #173); D-596 (F-P33-001 sweep that applied the enumerated skip-marker text).

**Cites:** D-598; D-596; F-P35-001; F-P35-002; F-P33-001; BC-5.41.003 v1.8; BC-5.41.001 v1.17; BC-5.41.002 v1.10; BC-6.24.001 v1.9; BC-7.07.002 v1.11; BC-5.39.001; L-F2-stale-term-deferral-unsafe (companion — deferral escalation; D-594); L-F2-cross-reference-title-code-sweep (companion — exhaustive-sweep discipline; D-582); E-18; S-18.08 (candidate gate scope extension).

**Closes:** D-598 lesson capture; annotation-must-be-self-contained process-gap codified; skip-marker premise-error class permanently closed via de-enumeration (canonical self-contained form now in all 4 BCs); BC-5.41.003 §Changelog structural gap closed (F-P35-001); streak reset 1/3→0/3; adversary pass-36 NEXT. `[codified; process-gap; annotation-discipline; sibling-sweep-hygiene]`

---

### L-F2-exhaustive-sweep-enumerate-and-count

**Category:** adversarial-convergence + sweep-discipline + false-green-prevention [process-gap]

**(a) Any 'exhaustive sweep' attestation MUST include the literal enumeration of all N target files AND the per-file grep stdout as captured evidence.** Subset-scoping — sweeping only the BCs being amended in the current burst — is FORBIDDEN and produces false-exhaustive claims. Evidence: F-P33-001 (pass-33) attested "exhaustive 8-BC sweep; class CLOSED" but scoped only to the BCs being amended at that burst (BC-5.41.001, BC-5.41.002, BC-6.24.001, BC-7.07.002), leaving BC-5.41.003 (caught at pass-35 F-P35-001) and then BC-1.15.001 (caught at pass-36 F-P36-001) without the §Changelog section — two consecutive self-inflicted sibling-sweep misses from false-exhaustive attestations across 3 successive passes.

**(b) Root-cause: the "8-BC cohort" is a fixed, named set (BC-1.15.001, BC-4.14.001, BC-5.41.001, BC-5.41.002, BC-5.41.003, BC-6.24.001, BC-7.07.001, BC-7.07.002).** A sweep that does not explicitly list all 8 and capture per-file output for each is a partial sweep, regardless of how many were amended in the triggering burst. The correct sweep evidence is:
```
grep -c '^## Changelog' \
  .factory/specs/behavioral-contracts/ss-01/BC-1.15.001.md \
  .factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md \
  .factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md \
  .factory/specs/behavioral-contracts/ss-05/BC-5.41.002.md \
  .factory/specs/behavioral-contracts/ss-05/BC-5.41.003.md \
  .factory/specs/behavioral-contracts/ss-06/BC-6.24.001.md \
  .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md \
  .factory/specs/behavioral-contracts/ss-07/BC-7.07.002.md
```
All 8 outputs must be `1` before attesting "exhaustive §Changelog coverage for all 8 E-18 BCs."

**(c) Verified exhaustive output (D-599 burst — first TRUE-EXHAUSTIVE run across all 8):**
```
ss-01/BC-1.15.001.md:1
ss-04/BC-4.14.001.md:1
ss-05/BC-5.41.001.md:1
ss-05/BC-5.41.002.md:1
ss-05/BC-5.41.003.md:1
ss-06/BC-6.24.001.md:1
ss-07/BC-7.07.001.md:1
ss-07/BC-7.07.002.md:1
```
All 8 = 1. §Changelog presence exhaustively verified for the E-18 BC cohort. This is the first attestation in the F2 E-18 cascade that satisfies the rule stated in this lesson.

**(d) RULE: a sweep that does not enumerate its N inputs and report the per-input count is a false-green generator (CI-as-Code positive-coverage axis).** The adversary cannot distinguish "we checked all 8 and none need fixing" from "we checked 4 and the other 4 are unknown" when the sweep attestation omits the file list. False-green attestations create the appearance of convergence while leaving the BC cohort incompletely validated — exactly the root cause behind F-P35-001 and F-P36-001 (both caught after the sweep was declared "CLOSED").

**(e) Mechanical gate: for cohort sweeps, capture `grep -c <pattern>` stdout for EACH file in the cohort; all-N must satisfy the expected value before attesting exhaustive.** This gate applies to ALL cohort-level structural properties, not just §Changelog:
- §Changelog presence: `grep -c '^## Changelog' <each-of-8-BCs>` — all must be 1
- §VP Anchors presence: `grep -c '^## VP Anchors' <each-of-8-BCs>` — all must be 1
- §Traceability presence: `grep -c '^## Traceability' <each-of-8-BCs>` — all must be 1
- Skip-marker presence (when added): `grep -c 'skip-marker' <each-of-8-BCs>` — all must be ≥1

**(f) This lesson extends the sibling-sweep family.** L-F2-annotation-must-be-self-contained (D-598) established that structural-parity sweeps must verify the §Changelog section EXISTS before claiming exhaustive coverage. This lesson operationalizes that rule into a mechanical gate protocol: enumerate all N files explicitly, capture per-file grep count, attest only when all-N satisfy the predicate. The two lessons together close the root cause behind the 4th consecutive self-inflicted sibling-miss class (F-P36-001).

**(g) Candidate S-18.08 gate scope (MANDATORY).** A consistency-validator check should enforce: when a sweep attestation claims "all N BCs in cohort C satisfy property P," verify that the attestation includes literal N-file enumeration with per-file grep output. If the attestation is narrative-only ("exhaustive sweep run") without captured stdout, BLOCK with message "cohort sweep attestation lacks per-file evidence (L-F2-exhaustive-sweep-enumerate-and-count)." This gate targets the false-green generation mechanism at its origin — attestation without evidence.

**Drift Items note:** Anchor E-18 F3 — exhaustive-sweep-enumerate-and-count: cohort sweeps must enumerate all N inputs and capture per-file grep stdout; subset-scoping is forbidden. Candidate S-18.08 gate scope extension (MANDATORY): consistency-validator blocks sweep attestations lacking per-file evidence. Root cause behind F-P35-001 + F-P36-001 (2 consecutive self-inflicted misses from same false-exhaustive-attestation pattern).

**Anchors:** D-599 (this burst; F2 pass-36 NOT-CLEAN FIX BURST; BC-1.15.001 v1.4→v1.5 §Changelog added; TRUE-EXHAUSTIVE sweep across all 8 E-18 BCs — first verified); F-P36-001 (MEDIUM; BC-1.15.001 v1.4→v1.5 — §Changelog structurally absent; 4th consecutive self-inflicted sibling-miss); F-P35-001 (MEDIUM; BC-5.41.003 v1.7→v1.8 — §Changelog structurally absent; 3rd self-inflicted sibling-miss); F-P33-001 (MED; original "8-BC sweep CLOSED" false-exhaustive attestation at D-596); BC-5.39.001 (3-CLEAN streak rule; 0/3; pass-37 NEXT); E-18 (CAP-032 context-durability; GitHub issue #173); L-F2-annotation-must-be-self-contained (D-598; structural-parity-sweeps-must-verify-section-EXISTS companion lesson).

**Cites:** D-599; D-598; D-596; F-P36-001; F-P35-001; F-P33-001; BC-1.15.001 v1.5; BC-5.41.003 v1.8; BC-5.39.001; L-F2-annotation-must-be-self-contained (D-598); L-F2-sibling-sweep-tree-wide-gate (D-564; foundational); S-7.02 (defensive sweep discipline); S-18.08 (candidate gate scope extension; MANDATORY); E-18.

**Closes:** D-599 lesson capture; exhaustive-sweep-enumerate-and-count process-gap codified; §Changelog presence exhaustively verified for all 8 E-18 BCs with literal per-file evidence (first TRUE-EXHAUSTIVE run in F2 cascade); root cause of F-P35-001 + F-P36-001 class permanently gated by per-file-count protocol; streak 0/3; adversary pass-37 NEXT. `[codified; process-gap; sweep-discipline; false-green-prevention; enumerate-and-count-gate]`

---

### L-F2-index-quad-cite-reflects-last-bump

**Category:** adversarial-convergence + bookkeeping-convention + propagation-discipline [convention]

**(a) The 4-index self-cited quad in any index's changelog/last_amended (e.g., "BC-INDEX vA / VP-INDEX vB / ARCH-INDEX vC") reflects the 4-index state AS OF THAT INDEX'S OWN LAST BUMP, not live HEAD.** In a sequential single-commit-per-burst model, an index not touched in a later burst correctly retains its prior quad — this is EXPECTED per-pass lag, NOT a propagation gap, and MUST NOT be "fixed."

**(b) Updating an untouched index's quad to a version it did not participate in would be false.** If ARCH-INDEX is bumped at D-600 (v2.46→v2.47) but VP-INDEX and BC-INDEX are not touched at D-600, their changelog rows still cite the 4-index state as of their own prior bumps (BC-INDEX v2.98 at D-599; VP-INDEX v2.28 at D-597). Those citations accurately record what the world looked like at the moment of each index's own last change.

**(c) Adversary passes MUST treat quad-lag on untouched indices as benign.** A finding that VP-INDEX cites ARCH-INDEX v2.46 while ARCH-INDEX is now at v2.47 is NOT a propagation gap — it is the expected and correct result of sequential single-commit-per-burst bookkeeping. Flagging this as a defect (and "fixing" it by updating untouched indices to reference a version they didn't participate in) would inject false history into those indices' changelogs.

**(d) Per-pass adjudication precedent (D-601, pass-38 F-P38-001):** Fresh-context adversary pass-38 flagged VP-INDEX/L2-INDEX citing the 4-index ARCH-INDEX quad as v2.46 while ARCH-INDEX is v2.47. The adversary self-adjudicated this as NON-DEFECT after recognizing the per-pass-lag convention. No fix was possible or warranted; package remained FROZEN. This is the canonical precedent for this class.

**(e) Rule statement:** For any index file's changelog/last_amended quad-cite: the cited version of each peer-index reflects the state of that peer AS OF THE CITING INDEX'S OWN LAST MODIFICATION. If a peer index advanced after the citing index was last touched, the citing index's quad correctly trails. The authoritative live versions are always the HEAD values of each index file, not any cross-cited quad.

**Anchors:** D-601 (this burst; F2 E-18 ADV PASS-38 CLEAN; streak 0/3→1/3; F-P38-001 adjudicated NON-DEFECT — VP-INDEX/L2-INDEX quad cites ARCH-INDEX v2.46 is EXPECTED per-pass lag); F-P38-001 (LOW observation; ARCH-INDEX v2.47 vs quad-cite v2.46 in VP-INDEX/L2-INDEX — adversary self-adjudicated non-defect); BC-5.39.001 (3-CLEAN streak rule; 0/3→1/3); E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-601; F-P38-001; D-600 (prior burst; ARCH-INDEX v2.46→v2.47 bumped; VP-INDEX/BC-INDEX untouched); BC-5.39.001; L-F2-prior-chain-append-only-history (D-600; companion — append-only history; don't rewrite historical records).

**Closes:** D-601 convention-note capture; L-F2-index-quad-cite-reflects-last-bump codified to prevent future re-flagging of benign quad-lag; pass-38 CLEAN; streak 0/3→1/3; adversary pass-39 NEXT. `[codified; convention; bookkeeping; quad-lag-benign; adversarial-convergence]`

---

### L-F2-deferred-table-semantics

**Category:** [process-gap]
**Discovered:** 2026-06-15 D-606 — pre-gate consistency-validator audit during E-18 F2 CONVERGED state
**Severity:** PROCESS-GAP — causes false BLOCKER classification at gate audits; prevented from degrading convergence status by architect adjudication

**Summary:** A consistency or perimeter audit MUST read a table's HEADING and COLUMN semantics before classifying a missing-file reference as a BLOCKER. ARCH-INDEX's "Future Sections (Deferred)" table with columns "Deferred File | Covered By" documents intentional deferrals whose content lives in a separate authoritative file named in the "Covered By" column. A file listed under that heading that does not exist on disk is NOT a broken anchor — it is an intentional deferral with a coverage pointer. Classifying it as a BLOCKER "registered-but-absent file" is a semantic-misread of the table structure.

**Root Cause (D-606):** The pre-gate consistency-validator audit ran fresh-context on the frozen E-18 spec package. It returned 2 BLOCKER perimeter findings: ARCH-INDEX v2.47 §Document Map registers `verification-architecture.md` and `verification-coverage-matrix.md` as artifacts that do not exist on disk. The auditor read the rows as "registered source files required to exist" rather than "intentionally deferred files with coverage statements."

**Architect Adjudication (D-606):** The two rows sit under the ARCH-INDEX "Future Sections (Deferred)" heading with columns "Deferred File | Covered By" — NOT under any heading implying mandatory existence. They are intentional documented deferrals. Content is covered by VP-INDEX.md (named in the "Covered By" column). VP-080 v1.1 already corrected its traces_to from the non-existent verification-architecture.md to VP-INDEX.md. All E-18 VPs (VP-081..086) correctly trace_to VP-INDEX.md. POLICY 9 arithmetic is self-contained in VP-INDEX. No broken link exists in the E-18 verification chain. BOTH findings are a single semantic-misread FALSE POSITIVE. System-level-deferred. NOT an E-18 F2 gate blocker. E-18 3-CLEAN convergence STANDS.

**Correct behavior for future auditors:**

**(a) Always read the table heading before classifying rows.** A row under "Future Sections (Deferred)" with a "Covered By" column is a deferral declaration, not a requirement that the file exists now.

**(b) The "Covered By" column names the real authority.** When a table row says "verification-architecture.md | VP-INDEX.md", it means: this section was deferred; its content now lives in VP-INDEX.md. Verify VP-INDEX.md exists and covers the relevant domain — do NOT require the deferred file to exist.

**(c) Gate-blocking a deliberately deferred artifact is not a deference audit; it is a structure-comprehension failure.** Auditors must distinguish: (i) "registered active source that should exist on disk" vs (ii) "intentionally deferred placeholder with coverage already relocated." Only class (i) warrants a BLOCKER.

**(d) Propagation check:** When a future adversary encounters ARCH-INDEX §Future Sections (Deferred) rows, the correct gate check is: does the "Covered By" file exist AND does it cover the relevant domain? If yes → PASS. If the "Covered By" file is missing OR does not cover the domain → BLOCKER (real, not false positive).

**(e) Cross-reference with POLICY 9:** VP allocations and verification-architecture relationships are tracked in VP-INDEX.md. The former "verification-architecture.md" concept was consolidated into VP-INDEX.md during E-18 F2 spec evolution. VP-080 v1.1 is the canonical artifact that documents this consolidation; its traces_to field names VP-INDEX.md as authoritative.

**Tracked deferral (Canonical Principle Rule 3):** ARCH-INDEX §Future Sections (Deferred) rows for verification-architecture.md + verification-coverage-matrix.md lack a story-ID anchor. Architect-adjudicated pre-existing intentional deferral (content covered by VP-INDEX.md; NOT an E-18 gate blocker). ACTION: assign a real system-level story ID (architect suggests S-19.xx slot, NOT the S-18.xx E-18 family) at the next system-level/maintenance planning, then annotate the two ARCH-INDEX rows [DEFERRED: <id>]. Owner: architect (authoring if materialized) + state-manager (index sync). Recorded in STATE.md Drift Items.

**Anchors:** D-606 (this burst; E-18 F2 ADV PASS-43 CLEAN 3/3 CONVERGED; pre-gate consistency audit + architect adjudication); pre-gate consistency-validator audit (2026-06-15; 2 BLOCKER FALSE POSITIVES adjudicated by architect); ARCH-INDEX v2.47 §Future Sections (Deferred) (verification-architecture.md / verification-coverage-matrix.md rows; "Deferred File | Covered By" heading); VP-INDEX.md (the authoritative "Covered By" target; VP-080 v1.1 traces_to corrected D-561); VP-081..086 all trace_to VP-INDEX.md; E-18 (CAP-032 context-durability; GitHub issue #173); BC-5.39.001 (3-CLEAN satisfied: pass-41/42/43).

**Cites:** D-606; D-561 (VP-080 v1.1 traces_to correction); ARCH-INDEX v2.47; VP-INDEX v2.29; VP-080 v1.1; E-18; BC-5.39.001; L-F2-subsystem-anchor-sweep (companion — read structural context before classifying missing anchors); L-F2-stale-term-deferral-unsafe (companion — do NOT reclassify or re-defer what has already been architect-adjudicated).

**Closes:** D-606 L-F2-deferred-table-semantics codified; E-18 F2 adversarial cascade 3-CLEAN CONVERGED (pass-41 ZERO-FINDINGS → pass-42 CLEAN → pass-43 CLEAN); pre-gate audit FALSE POSITIVE adjudicated; Drift Item recorded; AWAITING F2 HUMAN-APPROVAL GATE. `[process-gap; gate-audit; table-semantics; false-positive; deferred-sections; convergence-milestone]`

---

### L-F2-machine-stable-count-assertion

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-608 — E-18 delta re-validation FIX BURST (adversary delta-pass finding F-D607-003 LOW; consistency-validator FINDING-1 MINOR)
**Severity:** PROCESS-GAP — false-green vector; silently allows count assertion to pass even when output rendering changes make the probe inaccurate

**Summary:** VP proof harnesses MUST assert counts or file-list sizes against a machine-stable enumeration signal (e.g., a sentinel line emitted by the skill, a JSON array length, or a process exit code), NOT against a presentation-coupled prose-list regex such as `grep -c '^  - '` over human-readable output. Presentation-coupled regexes are false-green generators: if the skill changes its output format (indentation, bullet style, section heading), the grep silently produces an incorrect count while the assertion still passes.

**Root Cause (D-608 / F-D607-003):** VP-088 v1.0 §2 specified that the proof harness verify exact-list injection by running `grep -c '^  - '` over the human-readable context block produced by the `rehydrate-wave` skill. This couples the count assertion to the skill's presentation formatting. If the skill emits bullets with different indentation or switches to a JSON list, the count changes silently and the assertion becomes false-green. The fix (VP-088 v1.1) requires the rehydrate-wave skill to emit a machine-stable sentinel line `INJECTED_FILE_COUNT=<n>` in its stdout and the harness to assert against that sentinel.

**Companion finding (D-608 / F-D607-001):** VP-090 v1.0 §1/§2 omitted the newline-terminated-line precondition for the '>1000 entry' trigger in precompact-flush-log pruning. A file without a trailing newline produces a line-count that is off-by-one relative to the entry count. The fix (VP-090 v1.1) adds an explicit newline-termination precondition to §0, PC1, and the boundary fixture in §2, with BC-7.07.001 PC8/Inv3 cited as the guarantor of newline-termination in the append-only log.

**Generalizes:** F-D607-003 (VP-088 v1.0 grep-count probe); F-D607-001 (VP-090 v1.0 boundary-count off-by-one). Both are instances of the same class: count or structure assertions against a probe that is not decoupled from presentation formatting or filesystem line-ending conventions.

**Rule (binding for all future VP proof harness authoring):**

**(a) Machine-stable signal required for count assertions.** When a VP proof harness must assert that exactly N files, entries, or items were processed, the assertion MUST be grounded in one of: (1) a sentinel line emitted by the skill/script with a stable format (e.g., `INJECTED_FILE_COUNT=N` on stdout); (2) a JSON or structured-data output parsed with a schema-aware tool (jq, yq); (3) a process exit code encoding the count; (4) a file whose line count is guaranteed by a specified invariant (newline-termination + one-entry-per-line precondition named in the VP and enforced by the skill).

**(b) Presentation-coupled probes are FORBIDDEN as load-bearing assertions.** `grep -c '^  - '` (or similar indentation/bullet-style-sensitive patterns) over human-readable prose output is acceptable as a diagnostic aid but MUST NOT be the sole evidence for a pass/fail decision. Any VP bats fixture using such a probe MUST be flagged for machine-stable migration before the story ships.

**(c) Preconditions for boundary-arithmetic assertions MUST be stated explicitly.** When a VP property involves a numeric threshold (e.g., '>1000 entries triggers pruning'), all preconditions that affect the count interpretation MUST be named in the VP §0 Preconditions section — especially file-format invariants (newline-termination, one-record-per-line, no blank lines). The VP MUST cite the spec artifact that guarantees those preconditions hold.

**(d) Feeds forward to F3 story S-18.08.** The gate-story for E-18 implementation (S-18.08) MUST include acceptance criteria requiring: (i) rehydrate-wave skill emits `INJECTED_FILE_COUNT=<n>` sentinel; (ii) precompact-flush-prune.sh outputs a machine-stable PRUNE_RESULT sentinel or exit code indicating actual entry count before and after prune; (iii) VP-088 and VP-090 bats harnesses pass against the sentinel-based assertions only.

**Anchors:** D-608 (this burst); F-D607-003 (VP-088 v1.0 grep-count probe LOW); F-D607-001 (VP-090 v1.0 boundary-count missing newline precondition MEDIUM LOAD-BEARING); VP-088 v1.1 (machine-stable sentinel fix); VP-090 v1.1 (newline-termination precondition fix); S-18.08 (mandatory scope extension); E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-608; F-D607-001; F-D607-003; VP-088 v1.1; VP-090 v1.1; VP-083 v1.9 (F-P32-002 — tautology-hardening pattern that F-D607-002 mirrors for VP-087); L-F2-exhaustive-sweep-enumerate-and-count (companion: cohort sweeps must enumerate all N files + capture per-file stdout; machine-stable-count extends same principle to count assertions).

**Closes:** D-608 F-D607-003 LOW FIXED (VP-088 v1.1); D-608 F-D607-001 MEDIUM LOAD-BEARING FIXED (VP-090 v1.1); process-gap class codified; L-F2-machine-stable-count-assertion added to carry-forward lesson set. `[process-gap; count-assertion; machine-stable-signal; false-green; harness-discipline; S-18.08-scope]`

---

### L-F2-fix-at-correct-layer

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-609 — E-18 CONFIRMING adversary pass (F-CONF-001 MAJOR finding; confirming-pass returned NOT-CLEAN)
**Severity:** PROCESS-GAP — "fix-at-wrong-layer" / assert-the-bug-away anti-pattern; VP cites a guarantee the cited guarantor does not actually make

**Summary:** When a VP fix adds a precondition that depends on an upstream guarantee (e.g., "precompact-flush-log entries are always newline-terminated"), the guarantee MUST exist at the cited guarantor artifact (BC/ADR) before the VP is closed. Do NOT close a VP finding by citing a property the guarantor does not yet make — this is the "fix-at-wrong-layer" anti-pattern (also: assert-the-bug-away). Establish the guarantee at the owning artifact first, then cite it in the VP.

**Root Cause (D-609 / F-CONF-001 MAJOR):** VP-090 v1.1 (D-608 fix for F-D607-001 MEDIUM) added a newline-termination precondition to §0 and cited `BC-7.07.001 PC8/Inv3` as the guarantor. However, BC-7.07.001 PC8/Inv3 at v1.11 did not contain an explicit newline-termination obligation — the append semantics were specified by format only; a conforming `printf '%s'` (no trailing newline) would have satisfied the prior wording. The VP fix stated a guarantee the BC did not make. The confirming adversary detected this MISANCHORING as F-CONF-001 MAJOR load-bearing.

**Fix chain (D-609):** (1) product-owner added explicit newline-termination postcondition to BC-7.07.001 PC8 + Inv3 step 7 (v1.12, additive only) — establishing the guarantee at the owning artifact. (2) architect tightened VP-090 §0 cite from `BC-7.07.001 PC8/Inv3` to `BC-7.07.001 PC8 newline-termination clause / Inv3 step 7` (v1.2) — citation now resolves to a real guarantee. Correct fix-order: guarantor first, then VP cite.

**Companion finding (D-609 / O-CONF-001 LOW):** VP-087 v1.1 §3 used set-complement phrasing ("a story that is neither merged/withdrawn/cancelled/pending/draft") that paraphrased ADR-026 §Terminal-Wave Discriminator rather than BC-5.41.002 EC-001b. The text was attributed to EC-001b, but EC-001b does not contain the set-complement language — ADR-026 §Terminal-Wave Discriminator does. Fix (VP-087 v1.2): re-attribute to ADR-026 §Terminal-Wave Discriminator; paraphrase EC-001b accurately. Same root class: citing a guarantee/source that does not contain the cited content.

**Rule (binding):**

**(a) Guarantee first, cite second.** Before a VP may cite a BC Precondition/Invariant or ADR §Section as the guarantor of a property, that property MUST appear explicitly in the cited artifact. "Will be fixed in the same burst" is NOT sufficient — the guarantor must be written first.

**(b) The "fix-at-wrong-layer" pattern is ALWAYS a MAJOR finding.** If an adversary detects that a VP precondition cites a guarantee absent from the named BC/ADR, the finding is load-bearing regardless of how the VP body reads. The VP cannot be clean if its foundation is unsubstantiated.

**(c) Applies to POLICY 4 / POLICY 5 mis-anchoring.** This class is a specific instance of cross-document mis-anchoring under POLICY 4 (behavioral-spine consistency) and POLICY 5 (verification-property grounding). A VP that cites a non-existent guarantee creates a ghost anchor — the verification property is formally sound but its precondition is ungrounded.

**(d) Confirming-pass discipline.** A confirming adversary pass EXISTS precisely to catch this class. The fix-burst (D-608) architect resolved the VP finding but did not establish the upstream guarantee — the confirming pass (D-609) detected the residual gap. This is the confirming pass working as designed, NOT a duplicate finding.

**(e) Feeds forward to F3 S-18.08 gate-story scope.** The consistency-validator must include a gate check: for every `guarantor:` cite in a VP §Preconditions section, verify that the cited BC/ADR section contains the named clause (or a semantically equivalent obligation). A cite that resolves to a section containing no newline-termination language while claiming to guarantee newline-termination is an automatic BLOCKER.

**Anchors:** D-609 (this burst); F-CONF-001 (VP-090 v1.1 guarantor cite absent from BC-7.07.001 v1.11 PC8/Inv3; MAJOR load-bearing); O-CONF-001 (VP-087 v1.1 §3 attribution mismatch; LOW); BC-7.07.001 v1.12 (additive newline-termination PC established at guarantor); VP-090 v1.2 (guarantor cite tightened); VP-087 v1.2 (attribution corrected); E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-609; F-CONF-001; O-CONF-001; BC-7.07.001 v1.12; VP-090 v1.2; VP-087 v1.2; L-F2-machine-stable-count-assertion (companion — the D-608 burst that introduced the guarantor-cite gap); POLICY 4; POLICY 5.

**Closes:** D-609 F-CONF-001 MAJOR FIXED (BC-7.07.001 v1.12 + VP-090 v1.2); D-609 O-CONF-001 LOW FIXED (VP-087 v1.2); process-gap class codified; L-F2-fix-at-correct-layer added to carry-forward lesson set. `[process-gap; mis-anchoring; guarantor-cite; fix-at-wrong-layer; assert-the-bug-away; POLICY-4; POLICY-5; S-18.08-scope]`

---

### L-F2-no-bypass-on-edit-failure

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-610 — D-609 integration burst incident (Edit tool failure → python3 heredoc fallback)
**Severity:** PROCESS-GAP — TD-FACTORY-HOOK-BYPASS-001 P0 / POL-3 violation class; bypasses factory-dispatcher PreToolUse/PostToolUse hook chain

**Summary:** When an Edit tool call fails with "File has not been read yet" (or any other Edit/Write error), the correct recovery is Read-then-Edit/Write. NEVER fall back to `python3`, `sed`, or `echo` heredoc mutation of `.factory/` files. The python/sed/echo fallback is a TD-FACTORY-HOOK-BYPASS-001 P0 / POL-3 violation: it bypasses the factory-dispatcher PreToolUse/PostToolUse hook chain (validate-state-structure, validate-artifact-path, validate-burst-log, and all other registered hook plugins). The hook chain exists to enforce pipeline invariants — bypassing it silently strips these enforcement gates from the write path.

**Root Cause (D-609 integration burst incident):** During the D-609 E-18 CONFIRMING-PASS FIX BURST, an Edit tool call on a `.factory/` file failed (cause: the file had not been Read earlier in the conversation context, triggering "File has not been read yet"). The agent reflexively fell back to `python3` heredoc mutation to complete the write. The orchestrator detected and intervened mid-burst; the relevant sections were re-audited via Read and the commit-time hooks validated the final file. Recovery was possible because PostToolUse hooks fire after the write — but the PreToolUse gate was bypassed for the python write, meaning any PreToolUse block would have been silently skipped.

**Rule (binding):**

**(a) Read before Edit — always.** If an Edit call fails with "File has not been read yet," the ONLY correct recovery is to Read the file (or the relevant region with offset+limit), then re-attempt the Edit with a correctly matched `old_string`. The Read step is mandatory, not optional.

**(b) python3 / sed / echo heredoc writes to `.factory/` are FORBIDDEN.** These tools bypass the factory-dispatcher hook chain. TD-FACTORY-HOOK-BYPASS-001 P0 makes no exception for "recovery from tool failure." The hook chain enforces pipeline invariants; bypassing it is a P0 violation regardless of the reason.

**(c) Unique `old_string` discipline.** Most Edit failures citing "old_string not found" indicate the file content has drifted from what the agent expected (e.g., a prior edit in the same burst changed the region). Recovery: Read the current file content, identify the updated unique `old_string`, re-apply the Edit. Never widen the bypass path as a shortcut.

**(d) Same rule applies in F3 dispatches.** All F3 story-writer, product-owner, architect, and consistency-validator dispatches operate under this rule. Any agent that encounters an Edit failure MUST Read-then-Edit, not bypass via shell mutation. Orchestrator must include this as an explicit dispatch constraint in F3 prompts.

**(e) Feeds F3 S-18.08 gate-story scope.** The validate-artifact-path and validate-state-structure hooks can be extended to detect python3/sed mutation attempts at the PostToolUse Bash hook level. This is a candidate S-18.08 scope item for the enforcement story.

**Anchors:** D-610 (this burst); D-609 (incident burst); TD-FACTORY-HOOK-BYPASS-001 P0; POL-3; validate-state-structure plugin; validate-artifact-path plugin; E-18 (CAP-032 context-durability; GitHub issue #173).

**Cites:** D-610; D-609 (incident); TD-FACTORY-HOOK-BYPASS-001; POL-3; CLAUDE.md §Forbidden-patterns ("Direct edit of `.factory/STATE.md` by any agent other than state-manager"); CLAUDE.md §Operational-Discipline-TDs (TD-FACTORY-HOOK-BYPASS-001 P0 — Use Edit/Write tools ONLY for `.factory/` mutations. NEVER use Python/sed/echo bypass. Enforced by POL-3).

**Closes:** D-610 process-gap class codified; L-F2-no-bypass-on-edit-failure added to carry-forward lesson set for F3. `[process-gap; hook-bypass; tool-failure-recovery; TD-FACTORY-HOOK-BYPASS-001; POL-3; S-18.08-scope; F3-dispatch-constraint]`

---

### L-F2-cross-story-claim-verification

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-615 — E-18 story pass-1 fix wave (story-writer cross-story claim review)
**Severity:** PROCESS-GAP — silent spec-consistency defect class; uncaught claims propagate to index rows and traceability matrices

**Summary:** When a story-writer authors a story that makes claims about ANOTHER story (e.g., "S-18.04a covers VP-085" or "S-18.02 owns BC-4.14.001"), those claims MUST be verified against the ACTUAL acceptance criteria and file-structure sections of the referenced story — not inferred from the story's title or epic membership. Cross-story claims that are not verified can introduce phantom-VP and phantom-BC traceability entries that pass index integration but fail adversarial or consistency-validator review.

**Root Cause (E-18 pass-1 incident):** Multiple E-18 stories contained cross-story traceability claims in their Traceability sections that did not resolve to actual ACs or Invariants in the cited stories. For example, a story claimed "VP-085 verified by S-18.04a via AC-003" but AC-003 in S-18.04a covered a different behavioral dimension. The story-writer inferred the cross-story link from epic-level VP allocation notes in ADR-026 §VP Allocations rather than from the actual AC text in the cross-referenced story.

**Rule (binding):**

**(a) Cross-story PC/Inv cite verification is mandatory.** Before asserting "Story X covers VP-NNN via AC-M" or "Story X owns BC-N.NN.NNN", the story-writer MUST read the Acceptance Criteria section of Story X and confirm AC-M's text aligns with VP-NNN's stated properties or BC-N.NN.NNN's preconditions.

**(b) ADR-level VP allocation notes are architectural intent, not story-level verification.** ADR §VP Allocations tables assign VPs to stories at the planning level. Story-level Traceability sections must verify the assignment is realized in the story's actual ACs, not merely inherit the ADR allocation.

**(c) Cross-story claims discovered during integration must be fixed in the same burst.** If an integration burst finds a cross-story claim that cannot be verified (because the referenced AC does not exist or covers different behavior), the story-writer must amend the claiming story's Traceability section in the same burst. Filing it as a finding-for-later is a defer-pattern violation.

**Anchors:** D-615 (this burst); E-18 story pass-1 fix wave; ADR-026 §VP Allocations; CLAUSE-POL-005 (BC H1 verbatim traceability); POLICY 2 (bidirectional traceability DI→VP).

**Cites:** D-615; CLAUDE.md §Canonical-Principle Rule 4 (AI-built defects are the AI's responsibility to fix in scope); CLAUDE.md §Companion-Principle (correct agent routing for spec defects).

**Closes:** D-615 process-gap class codified; L-F2-cross-story-claim-verification added to carry-forward lesson set for F3. `[process-gap; cross-story-traceability; story-writer-discipline; VP-verification; E-18]`

---

### L-F2-story-pc-cite-verbatim

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-615 — E-18 story pass-1 fix wave (AC↔PC realignment review)
**Severity:** PROCESS-GAP — phantom-precondition defect class; non-existent PC headings in Traceability sections produce phantom traceability that is unverifiable

**Summary:** Every Traceability section entry of the form "PC3 — <description>" or "Inv2 — <description>" in a story MUST resolve to an actual Precondition or Invariant heading in the cited BC. Story-writers must read the cited BC's PC/Inv section before writing the Traceability row, not infer PC/Inv IDs from the story's behavioral description.

**Root Cause (E-18 pass-1 incident):** Multiple E-18 stories cited PCs and Invariants in their Traceability sections using IDs that did not exist in the cited BCs. For example, a story cited "BC-5.41.001 PC7 — wave_id derives from real sprint-state.yaml fields" but BC-5.41.001 had only PC1..PC6 (no PC7 in v1.8). The story-writer constructed the PC ID from the behavioral description rather than checking the BC's actual PC enumeration.

**Rule (binding):**

**(a) PC/Inv IDs in Traceability must resolve to real headings.** Before writing "BC-N.NN.NNN PC-M" in a Traceability section, open the BC file and confirm PC-M (or Inv-M) exists as an actual heading or bullet in that BC's Preconditions or Invariants section.

**(b) If a story behavior maps to a PC that doesn't exist, DO NOT invent a PC ID.** Either: (i) cite the closest real PC with a clarifying note, (ii) cite the BC's Invariant section if the behavior is better expressed there, or (iii) flag the gap to the product-owner for a BC amendment to add the missing PC.

**(c) Phantom PC cites are NOT caught by current index integration.** BC-INDEX and STORY-INDEX only track which BCs a story covers, not which PCs. Phantom PCs propagate silently until adversarial review or consistency-validator audit. Prevention (story-writer verification discipline) is the only gate at authoring time.

**Anchors:** D-615 (this burst); E-18 story pass-1 fix wave; BC-5.41.001 (PC1..PC6; no PC7); POLICY 7 (BC H1 verbatim cite).

**Cites:** D-615; CLAUSE-POL-005 (BC H1 verbatim traceability); CLAUDE.md §Canonical-Principle Rule 4.

**Closes:** D-615 process-gap class codified; L-F2-story-pc-cite-verbatim added to carry-forward lesson set for F3. `[process-gap; phantom-pc; story-traceability; story-writer-discipline; BC-PC-verification; E-18]`

---

### L-F2-fix-wave-must-sweep-downstream

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-616 — E-18 story pass-2 fix wave (post-mortem of D-615 fix wave)
**Severity:** PROCESS-GAP — incomplete fix-wave sweep; downstream artifact staleness propagates silently past the claiming burst

**Summary:** A fix wave that changes story-file frontmatter MUST sweep ALL downstream references in the SAME burst — STORY-INDEX rows, VP-file `anchor_story` frontmatter, BC `§Stories`/`§Story-Anchor` body sections — and an index changelog claiming a fix is a paper-fix (TD-VSDD-059) unless the source artifact actually changed.

**Root Cause (D-615 incident):** D-615 fix wave changed story file frontmatter (subsystems, wave) and updated STORY-INDEX §DAG notes, but left STORY-INDEX body-row subsystem columns stale (SS-08 remained), left VP-082/084/085/090 `anchor_story: "S-18.04"` instead of correcting to S-18.04a/S-18.04b, and left input-hashes in STORY-INDEX body rows stale. The D-615 INDEX changelog said "F-002 anchor fixes: BC-7.07.001 story-anchor S-18.04→S-18.04a; BC-5.41.003 story-anchor S-18.04→S-18.04b" — this was a paper-fix because the BC-INDEX body cells already had S-18.04a/04b (from an earlier burst), but the VP FILE frontmatter `anchor_story` fields were the real stale artifact and were not updated.

**Rule (binding):**

**(a) STORY-INDEX body rows are downstream of story-file frontmatter.** Any change to a story-file `subsystems:` or `wave:` field MUST be propagated to the matching STORY-INDEX body row in the same burst. A body row showing SS-08 when the story file shows SS-06 is a silently propagated defect that will not be caught until an adversarial or consistency pass.

**(b) VP file `anchor_story` fields are downstream of story-ID changes.** Any rename/split of a story (e.g., S-18.04 → S-18.04a + S-18.04b) MUST sweep all VP files that cited the old story ID. VP-INDEX §Story Anchors table is NOT sufficient — the VP FILE frontmatter `anchor_story:` field is also authoritative and must match.

**(c) Index changelog claiming a fix is a paper-fix unless the source file changed.** If the INDEX says "BC story-anchor corrected" but the BC file frontmatter has been correct since a prior burst, the real stale artifact is elsewhere. Adversary must verify the claimed closure by checking the actual source file, not just the INDEX changelog.

**(d) POLICY 18 input-hash changes MUST cascade to STORY-INDEX body rows.** When the compute-input-hash tool produces corrected hashes, both story-file frontmatter AND STORY-INDEX body row `input-hash` cells MUST be updated in the same burst.

**Anchors:** D-616 (this burst); D-615 (incident burst); E-18 story pass-2 fix wave; TD-VSDD-059 (paper-fix detection); POLICY 18 (input-hash discipline).

**Cites:** D-616; D-615; TD-VSDD-059; POLICY 18; CLAUDE.md §Canonical-Principle Rule 4.

**Closes:** D-616 process-gap class codified; L-F2-fix-wave-must-sweep-downstream added to carry-forward lesson set for F3. `[process-gap; incomplete-sweep; story-index; vp-anchor_story; input-hash; downstream-propagation; E-18]`

---

### L-F2-input-hash-tool-trust

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-616 — E-18 story pass-2 fix wave (compute-input-hash awk+resolver bug investigation)
**Severity:** PROCESS-GAP — silent tool defect; collision across distinct-input artifacts is a tool-defect signal, not coincidence

**Summary:** POLICY 18 input-hash was a silent no-op for multi-input artifacts. The `compute-input-hash` tool had an awk bug that hashed only the first listed input file; the repo-root path resolver also failed to resolve `.factory/`-prefixed paths. Re-running a broken tool reproduces the wrong hash — a collision across distinct-input artifacts is a tool-defect signal, not coincidence. Fixed in D-616 (devops, branch `fix/compute-input-hash-multi-input-awk`, commits ea6cf1af + 5b0d5e5c, PR→develop PENDING). Verify tool correctness (distinct inputs → distinct hashes) before trusting `--check`.

**Root Cause (D-614/D-615 incident):** S-18.02, S-18.08, and S-18.09 all had input-hash `69dcbd9` despite having different input file sets. The shared collision value was the hash of the FIRST input of each set (BC-4.14.001.md for all three). The tool's awk pipeline used `print $0` to concatenate all file contents but the resolver failed on `.factory/` prefix paths, silently reducing to single-file mode for any artifact with more than one input where the additional inputs use project-root paths. The collision was internally consistent (running the tool again reproduced the same wrong value) and was therefore undetected by `--check`.

**Rule (binding):**

**(a) Collision across distinct-input artifacts is a tool-defect signal.** If two or more stories with different input file sets produce the same input-hash, this MUST be treated as a tool failure and investigated before the hashes are committed. Coincidental collision of full SHA-7 values is astronomically improbable; any such collision is practically certain to be a bug.

**(b) Tool correctness verification before `--check` trust.** After any compute-input-hash fix, run the tool against at least 3 stories with fully distinct input sets and confirm no collisions before treating the tool's `--check` output as authoritative.

**(c) Re-running a broken tool is not a fix.** When `--check` reports mismatch after a fix, the correct action is to re-run the FIXED tool binary (not the prior installed version). Version the tool binary explicitly so it is clear which tool version produced which hash batch.

**(d) Input-hash TBD is a soft gate only at authoring time.** At integration time (D-616), any remaining `TBD` input-hashes are a BLOCKER for integration. No story may be integrated to STORY-INDEX without a non-TBD input-hash if the tool is operational.

**Anchors:** D-616 (this burst and tool fix); D-614/D-615 (incident bursts where collisions were committed); E-18 story pass-2 fix wave; POLICY 18; branch `fix/compute-input-hash-multi-input-awk`.

**Cites:** D-616; POLICY 18; CLAUDE.md §Canonical-Principle Rule 1 (no silent defects).

**Closes:** D-616 process-gap class codified; L-F2-input-hash-tool-trust added to carry-forward lesson set for F3. `[process-gap; tool-defect; input-hash; collision-detection; compute-input-hash; awk; resolver; POLICY-18; E-18]`

---

### L-F2-statemd-banner-wcl-each-burst

**Category:** [process-gap]
**Discovered:** 2026-06-16 D-617 — STATE.md SIZE BUDGET banner block repair (D-609..D-616 missing entries detected by CI bats-full-suite run 27649845315)
**Severity:** PROCESS-GAP — structural CI breakage; ALL develop PRs red-lit while bats-full-suite (linux) CI job fails

**Summary:** Every state-manager STATE.md burst (Commit-E) MUST append a canonical `N lines (wc-l; ...)` entry to the SIZE BUDGET HTML-comment block with a LEADING integer line-count N matching `wc -l .factory/STATE.md` at the time of that commit. The `validate-state-structure` WASM hook's `extract_banner_line_count` function scans the banner block for the LAST `N lines (wc-l<terminator>` entry and compares N to the actual newline count. If the last entry is absent (D-610..D-616 omitted entries) OR malformed (D-609 had `(wc-l;` with no leading digit), the function returns `None` and the hook reports "no SIZE BUDGET banner found" — blocking any STATE.md write AND causing the `bats-full-suite (linux)` CI job's `pass-real-state-md-snapshot` bats test to fail. This red-lights ALL develop PRs.

**Root Cause (D-609..D-616 incident):** Bursts D-610..D-616 each updated STATE.md but did not append banner entries. The D-609 burst added a banner entry but omitted the leading integer (wrote `(wc-l;` rather than `410 lines (wc-l;`). The PostToolUse `validate-state-structure` hook validates STATE.md at write-time but does NOT enforce that the FINAL line-count entry matches the post-write actual count — it validates whatever the last entry claims at the moment of the write, which may be stale if the entry was already present from a prior burst. The `pass-real-state-md-snapshot` bats test is the authoritative gate that verifies the live file, and it runs only in CI (not per-write). Detection delay: 7 bursts.

**Rule (binding):**

**(a) Append a new banner entry at Commit-E of every STATE.md fix burst.** After all content changes are finalized and before committing, run:
```bash
wc -l .factory/STATE.md
```
Then append a line inside the `<!-- STATE.md SIZE BUDGET ... -->` block (before the closing `-->`) in the form:
```
  N lines (wc-l; D-NNN: <one-line burst description>).
```
where N is the exact `wc -l` output (count of newlines in the file).

**(b) The leading integer MUST precede `lines (wc-l`.** The extractor looks for the byte sequence ` lines (wc-l` (space before `lines`) and then walks BACKWARD to collect preceding ASCII digits. If no digit precedes the space, `scan_for_last_wc_l` returns `None`. The correct form is `425 lines (wc-l; ...)`, NOT `(wc-l; ...) 425 lines`.

**(c) The LAST entry in the banner is the authoritative claim.** If multiple entries exist (historical log of past bursts), the extractor returns the last one. Always append — never edit prior entries (POLICY 1 append-only history).

**(d) "STATE.md SIZE BUDGET" MUST NOT appear in frontmatter prose.** The `extract_banner_block` function finds the FIRST occurrence of `STATE.md SIZE BUDGET` in the file and then scans backwards for `<!--`. If this string appears in `last_amended:` or any frontmatter field before the HTML comment, the scan finds no `<!--` before it and returns `None`, causing the same failure. Use abbreviated forms like `banner-block repair` or `SIZE-BUDGET` in frontmatter prose.

**Anchors:** D-617 (this repair burst); D-609..D-616 (missing/malformed entries); PR #189 CI bats-full-suite run 27649845315; `crates/hook-plugins/validate-state-structure/src/lib.rs` `extract_banner_line_count` + `scan_for_last_wc_l` + `extract_banner_block`.

**Cites:** D-617; D-421(c)+D-422(c)+D-446(c); BC-5.39.005; validate-state-structure WASM hook; bats `pass-real-state-md-snapshot`.

**Closes:** D-617 process-gap codified; L-F2-statemd-banner-wcl-each-burst added to carry-forward lesson set. `[process-gap; banner; wc-l; validate-state-structure; bats; CI; STATE.md; Commit-E]`

---

### L-F2-ac-pc-parity-sibling-sweep

**Category:** [process-gap]
**Discovered:** 2026-06-17 D-621 — E-18 story adversarial pass-4 (O-P4-004 process-gap finding; recurring across S-18.02 pass-3, S-18.04a pass-3, S-18.04b pass-4)
**Severity:** PROCESS-GAP — structural defect class; instance-scoped fixes caused 3-recurrence across passes; class fix required exhaustive 12-story sweep + mandatory gate

**Summary:** AC↔PC mis-trace (a story AC's `traces to BC-X PC-N` reference pointing to a wrong or non-existent postcondition number in the cited BC) recurred across S-18.02 (pass-3), S-18.04a (pass-3), and S-18.04b (pass-4). Each prior fix was instance-scoped: the adversary found one AC with a wrong PC reference, the story-writer corrected that AC, but did not sweep siblings. The class fix required (a) an exhaustive sweep of all 12 E-18 story files checking EVERY `(traces to BC-X PC-N / INV-N)` parenthetical against the referenced BC's actual §Postconditions / §Invariants section, and (b) a mandatory automated gate to prevent future regressions.

**Root Cause (pass-3/pass-4 recurrence):** S-18.04b pass-3 fixed F-SP3-003 (AC-005 traces-to correction) but did not sweep the sibling ACs in the same story (AC-002/003/004 remained with wrong PC references). Pass-4 found F-P4-001 MAJOR on those ACs. The same class existed in S-18.02 and S-18.04a. The pattern: when a BC has numbered Postconditions AND Invariants, authors confuse PC numbers with INV numbers and cite the wrong section clause.

**Rule (binding):**

**(a) AC↔PC parity sweep is class-scoped, not instance-scoped.** When an adversary finds one AC with a wrong `traces to BC-X PC-N` reference, the fix burst MUST sweep ALL ACs in that story AND all sibling stories in the same epic that share any BC with the affected story. Correcting only the reported AC and declaring done is a false-closure.

**(b) Verification protocol for AC↔PC parity:** For each AC containing `traces to BC-X PC-N`, verify:
1. `BC-X` has a §Postconditions section.
2. `PC-N` exists as a numbered heading or row in that section (exact number, not off-by-one).
3. The PC description is semantically consistent with what the AC is testing.

**(c) Mandatory automated gate (resolution anchor S-18.09 AC-008):** The class fix is codified as **S-18.09 AC-008** — a machine-checkable assertion that EVERY `(traces to BC-X PC-N / INV-N)` parenthetical in E-18 story bodies resolves to a real numbered clause in the cited BC. This gate is now part of S-18.09's mandatory Red Gate test table. The gate runs as part of the E-18 F4 TDD delivery for S-18.09 and provides a post-delivery regression guard.

**(d) Instance discovery does not satisfy class closure.** A pass that finds 0 NEW instances of this class does NOT mean the class is closed unless the automated gate (AC-008 or equivalent) has been implemented and is passing. Until S-18.09 is merged, this class must be manually swept at every adversary pass.

**Resolution:** Pass-4 fix burst included exhaustive 12-story sweep of all E-18 story files. S-18.09 v1.4 adds AC-008 as the permanent gate. Lesson tagged `[codified]` with anchor **S-18.09 AC-008**.

**Anchors:** D-621 (this burst); D-620 (pass-3 fix burst, prior instance); S-18.09 AC-008 (gate anchor — permanent enforcement); O-P4-004 (adversary observation that triggered class escalation); F-P4-001 MAJOR (pass-4 finding on S-18.04b); S-18.02 pass-3 / S-18.04a pass-3 / S-18.04b pass-4 (3-recurrence chain).

**Cites:** D-621; S-18.09 AC-008; POLICY 5 v1.3.3 (sibling sweep mandate); CLAUDE.md §Canonical-Principle Rule 4 (AI-built defects are AI's responsibility to fix); O-P4-004.

**Closes:** O-P4-004 process-gap class escalation satisfied; L-F2-ac-pc-parity-sibling-sweep codified and added to carry-forward lesson set; [codified] anchor = S-18.09 AC-008. `[process-gap; AC-PC-parity; sibling-sweep; traces-to; BC-postcondition; S-18.09; AC-008; E-18; pass-4]`

---

### L-F2-index-cell-and-version-cite-sibling-sweep

**Category:** [process-gap]
**Discovered:** 2026-06-17 D-622 — E-18 story adversarial pass-5 (F1/F2 consistency INCONSISTENT + F4 ARCH-INDEX §Document Map stale VP-counts; recurring across multiple passes)
**Severity:** PROCESS-GAP — structural drift class; instance-scoped fixes caused 3 distinct streak resets across passes; class fix required exhaustive sweep protocol + gate candidates

**Summary:** STORY-INDEX title cells and ARCH-INDEX §Document Map version-cite annotations drift from ground truth across adversary passes when story files and architecture documents re-version. Each prior instance-scoped fix corrected the reported cell but did not sweep siblings. Pass-5 found: (F1) S-18.07 title word-order reversal; (F2) S-18.08 missing " in bodies" suffix; (F4) ARCH-INDEX annotations for verification-architecture.md and verification-coverage-matrix.md citing v1.2/91 and v1.1/91 while actual files were at v1.3/92 and v1.2/92 following D-615. The F4 gap had been present since the D-615 burst (VP-092 added, verification files versioned) with ARCH-INDEX §Document Map annotation not updated.

**Root Cause (class):** Two independent drift mechanisms:
1. **Title drift:** STORY-INDEX title cells are manually maintained. When story authors revise a frontmatter `title:` during story versioning (e.g., moving a qualification phrase to a different position), the index cell is not automatically updated. STORY-INDEX cells are often copied from an earlier version of the title and then diverge.
2. **Annotation drift:** ARCH-INDEX §Document Map annotation cells (e.g., "v1.2 (D-612 VP-091 added; Total 91)") are manually updated only when the state-manager explicitly sweeps the index for the changed document. Bursts that version a document but do not dispatch state-manager to update ARCH-INDEX leave stale annotations.

**Rule (binding):**

**(a) Every STORY-INDEX version bump on a story MUST verify title cell verbatim.** The comparison protocol: `grep '^title:' <story-file>` → strip "S-XX.XX: " prefix if present → compare verbatim to STORY-INDEX body cell. Any divergence is a defect, not a style choice. The strip-prefix convention is confirmed per S-17.04 precedent.

**(b) Every architecture document version bump MUST sweep ARCH-INDEX §Document Map.** When a burst versions verification-architecture.md, verification-coverage-matrix.md, or any other document cited in ARCH-INDEX §Document Map, the annotation cell MUST be updated in the same burst. "Same burst" means the same state-manager commit that records the version bump.

**(c) The sweep is class-scoped, not file-scoped.** When a pass-N adversary finds one stale title or one stale annotation, the fix burst MUST sweep ALL STORY-INDEX title cells (grep all 12 E-18 story frontmatters) AND ALL ARCH-INDEX §Document Map annotation cells (grep each cited document's `^version:`) in the same burst. Fixing only the reported instance and declaring done is a false-closure under the class.

**(d) Verification protocol — title sweep:**
1. For each story in the epic: `grep '^title:' <story-file>` to get the authoritative title.
2. Strip "S-XX.XX: " prefix if present.
3. Locate the story's STORY-INDEX row and extract the title cell text.
4. Assert: cell text equals stripped frontmatter title verbatim (including capitalization, punctuation, word order).

**(e) Verification protocol — ARCH-INDEX annotation sweep:**
1. For each document cited in ARCH-INDEX §Document Map: `grep '^version:' <cited-file>`.
2. Locate the document's ARCH-INDEX §Document Map annotation cell.
3. Assert: the annotation cites the correct version (and VP counts where applicable).
4. Any divergence is a defect to be closed in the same burst.

**(f) Mandatory candidate gates (resolution anchor S-18.08):** The class fix is codified as two candidate automated gates for S-18.08 (pure-parse invariant consistency gate):
- **(a)** STORY-INDEX-title == frontmatter-title parity check (pure-parse: read story frontmatter + STORY-INDEX; no external substrate).
- **(b)** ARCH-INDEX §Document Map version-cite == artifact `^version:` check (pure-parse: read both files; no external substrate).
Both gate candidates are pure-parse and consistent with S-18.08's BC invariant. Tagged as [codified] with anchor S-18.08.

**(g) Until S-18.08 gate is implemented:** The sweep MUST be performed manually at every state-manager index-sync burst for any epic under active adversarial review. No pass may declare its index-sync COMPLETE without the sweep attestation.

**Resolution:** Pass-5 fix burst included exhaustive 12-story title sweep (10 PASS; S-18.07 + S-18.08 fixed) and ARCH-INDEX §Document Map full annotation sweep (both stale cells corrected). Lesson tagged [codified] with anchor **S-18.08**.

**Anchors:** D-622 (this burst); D-615 (propagation gap origin for F4); S-18.08 (gate anchor — automated enforcement candidate); F-P5-001 MED (stale BC cite), F1/F2/F4 consistency (adversary findings that triggered class escalation); S-18.07 F1 (title word-order); S-18.08 F2 (title suffix); verification-architecture.md + verification-coverage-matrix.md F4 (ARCH-INDEX annotation).

**Cites:** D-622; S-18.08; POLICY 5 v1.3.3 (sibling sweep mandate); POLICY 14 5-leg (version bump parity — annotation sweeps are part of leg 5); CLAUDE.md §Canonical-Principle Rule 4 (AI-built defects AI's responsibility); F-P5-001 MED; F1/F2/F4 consistency findings.

**Closes:** F1/F2/F4 consistency findings satisfied; L-F2-index-cell-and-version-cite-sibling-sweep codified and added to carry-forward lesson set; [codified] anchor = S-18.08. `[process-gap; index-cell; version-cite; title-drift; annotation-drift; sibling-sweep; STORY-INDEX; ARCH-INDEX; S-18.08; E-18; pass-5]`

---

## L-F2-s7-sibling-sweep-partial-class

**Lesson ID:** L-F2-s7-sibling-sweep-partial-class
**Classification:** [process-gap]
**Source:** D-623 (E-18 story cascade pass-6 adversary finding O-P6-001 / F-P6-001 MED)
**Story anchor:** S-18.08 (gate scope for automated sibling-sweep enforcement)

**Context:** During pass-5 fix burst (D-622), a cosmetic changelog-reorder was applied to 4 of 5 sibling stories (S-18.03, S-18.04a, S-18.05, S-18.04b). The 5th sibling (S-18.08) was missed. This triggered F-P6-001 MED at pass-6 adversary review (S-7.01 sibling-sweep class).

**Root cause:** The sweep was defined by scanning "the stories that had their version bumped in this pass" rather than "the entire sibling cohort sharing the same structural change." Partial-scope sweeps produce false-green for the sweep attestation while leaving one or more siblings in the defective state.

**Class (S-7.01 partial-fix):** Any cosmetic change that applies to a cohort (changelog reorder, terminology update, formatting normalization) MUST enumerate the cohort exhaustively BEFORE applying. The number of siblings found MUST equal the expected cohort size. If the size is uncertain, grep for the pattern across all candidate files and capture stdout (enumerate-and-count discipline per L-F2-exhaustive-sweep-enumerate-and-count).

**Rule (binding):**

**(a) Enumerate-before-sweep.** For any cohort-scoped change (changelog reorder, terminology sweep, structural normalization): `grep -rn '<pattern>' .factory/stories/S-18.*.md` to enumerate ALL affected files BEFORE applying the change. Assert the count equals the expected cohort size.

**(b) Capture stdout for attestation.** The enumeration grep output MUST appear in the burst-log Dim-2 attestation. Narrative "applied to all N siblings" without captured grep stdout is a false-attestation under D-449(a).

**(c) This class is distinct from missing-sibling sweep (L-F2-cross-reference-title-code-sweep).** That lesson covers VP/BC title cite parity; this lesson covers cohort structural-normalization sweeps. Both require enumerate-and-count.

**Resolution:** Pass-6 fix burst: story-writer applied changelog reorder to S-18.08 (the missed 5th sibling). F-P6-001 MED closed. L-F2-s7-sibling-sweep-partial-class codified.

**Anchors:** D-623 (this burst); O-P6-001 [process-gap]; F-P6-001 MED; S-18.08 (missed sibling); L-F2-exhaustive-sweep-enumerate-and-count (enumerate-count gate).

**Cites:** D-623; S-7.01 defensive-sweep discipline; POLICY 5 v1.3.3 sibling-sweep mandate; L-F2-exhaustive-sweep-enumerate-and-count (D-599); D-449(a) literal-shell stdout requirement; F-P6-001 MED.

**Closes:** F-P6-001 MED closed; O-P6-001 [process-gap] codified. `[process-gap; sibling-sweep; partial-fix; enumerate-count; changelog; S-18.08; E-18; pass-6; S-7.01]`

---

## L-F2-epic-traceability-gate-candidate

**Lesson ID:** L-F2-epic-traceability-gate-candidate
**Classification:** [codified]
**Source:** D-623 (E-18 story cascade consistency-validator pass-6 finding C-P6-005 MAJOR)
**Story anchor:** S-18.08 and/or S-18.09 (gate scope for epic-traceability-summary↔BC-H1 directional-match)

**Context:** Consistency-validator pass-6 found a GENUINE semantic inversion in the E-18 epic body: BC-5.41.001 and BC-5.41.002 summaries in the epic's §BC Coverage had their behavioral-direction swapped (the description said the opposite of what the BC actually requires). This defect was present from D-611 F3 plan authoring and survived 43 F2 adversary passes and 5 E-18 story cascade adversary passes without detection. It was caught by the consistency-validator's fresh-context "perimeter audit" (checking whether the epic's registered BC summaries directionally match the BC H1 title and Invariants).

**Root cause:** Epic bodies summarize BCs in paragraph/list form. These summaries are prose paraphrases, not mechanical extracts. Paraphrases can invert behavioral direction (e.g., "validates X is present" vs. "validates X is absent") without triggering any grep-based consistency check currently in use. The adversary focuses on story-level BC postconditions; the consistency-validator focuses on BC field-level consistency; neither role was explicitly tasked with "does the epic summary match BC H1 direction?"

**Significance:** A fresh-context adversary audit over 43 passes + 5 story passes (48 total) failed to catch a MAJOR semantic inversion. This demonstrates that EPIC-level summary prose is a high-defect-density zone that requires a dedicated directional-match gate.

**Gate candidate (binding):**

**(a) Epic-summary↔BC-H1 directional match.** For each BC listed in an epic's §BC Coverage (or equivalent): assert that the epic's prose description of the BC's behavior DIRECTIONALLY MATCHES the BC H1 title (subject → condition → effect direction). A paraphrase that inverts the subject or the condition-effect relationship is a BLOCKER.

**(b) Gate classification.** This is a pure-parse gate: read epic body + BC H1. No external substrate required. Consistent with S-18.08's BC invariant (pure-parse WASM gate scope).

**(c) Scope.** The gate MUST cover all epics in the story-index that are under active adversarial review or have been authored since the last directional-match audit.

**(d) Interim protocol (until S-18.08/S-18.09 gate implemented).** The consistency-validator MUST include a directional-match check for epic BC summaries in every pass on a story cascade. This is not a new axiom — it extends the existing "is-the-perimeter-right" audit that caught C-P6-005.

**Resolution:** Product-owner fixed BC-5.41.001/002 summaries in epic body (epic v1.3). Consistency-validator confirmed corrected. L-F2-epic-traceability-gate-candidate tagged [codified] with anchor S-18.08/S-18.09.

**Anchors:** D-623 (this burst); C-P6-005 MAJOR (consistency-validator finding); E-18 epic v1.2→v1.3 (product-owner fix); S-18.08 + S-18.09 (gate scope candidates); BC-5.41.001; BC-5.41.002.

**Cites:** D-623; C-P6-005 MAJOR; consistency-validator (fresh-context perimeter audit); CLAUDE.md §Canonical-Principle Rule 4; S-18.08; S-18.09.

**Closes:** C-P6-005 MAJOR closed; L-F2-epic-traceability-gate-candidate [codified] anchored to S-18.08/S-18.09. `[codified; epic; traceability; semantic-inversion; BC-summary; directional-match; E-18; S-18.08; S-18.09; pass-6; C-P6-005]`

---

## L-F2-registration-footnote-stale-on-count-change

**Lesson ID:** L-F2-registration-footnote-stale-on-count-change
**Classification:** [process-gap]
**Source:** D-623 (E-18 story cascade consistency-validator pass-6 finding C-P6-002 BLOCKER)
**Story anchor:** S-18.08 (gate scope for registration-footnote sweep)

**Context:** STORY-INDEX E-18 registration footnote (line ~701) was not swept when story count changed from 11→12 stories (D-614 story registration → D-615 S-18.10 addition). The footnote still showed "11 stories / 84 pts / 7-wave / VP-081..VP-091 / SS-06,08 / 11 input-hashes / 9 BCs" through passes 1–6. The delivery notes (lines 670–674) were correctly updated at each pass, but the footnote was not part of the sweep scope.

**Root cause:** The "registration footnote" (a summary note at the bottom of the epic's footnote chain, format `***E-NN... — N stories: ...`) is maintained separately from the per-story delivery notes and wave schedule. No explicit rule required the footnote to be swept when story count changed. The footnote is treated as "initial registration prose" rather than "live delivery status."

**Class:** Stale-registration-prose — text authored at story registration (D-614) that was not flagged for re-verification when the registration itself changed (D-615 S-18.10 addition). This is a special case of the "static commentary treated as immutable when it should be versioned" anti-pattern.

**Rule (binding):**

**(a) When story_count changes (add or withdraw story from an epic), sweep ALL registration footnotes for the affected epic in the same burst.** The footnote must reflect: current story count, current total pts, current wave count, current VP range, current per-story subsystem union, current input-hash count, current BC count.

**(b) Registration footnotes are NOT immutable historical records.** They are live status summaries (unlike the `[Prior:]` chains in `last_amended` which ARE historical and append-only per L-F2-prior-chain-append-only-history). When the epic grows, the footnote MUST be updated.

**(c) Verification trigger.** Any burst that adds or withdraws a story from an epic MUST include an explicit "registration footnote sweep" step.

**Resolution:** Pass-6 fix burst corrected the E-18 footnote to reflect current state (12 stories / 89 pts / 8-wave / VP-081..VP-092 / correct per-story subsystems / 12 input-hashes / 10 BCs; ADR-026 v1.21; D-611/D-614/D-615 reference). L-F2-registration-footnote-stale-on-count-change codified.

**Anchors:** D-623 (this burst); C-P6-002 BLOCKER; D-614 (story registration, 11 stories); D-615 (S-18.10 addition, count → 12); STORY-INDEX line ~701 (E-18 footnote).

**Cites:** D-623; C-P6-002 BLOCKER; L-F2-prior-chain-append-only-history (historical record distinction); STORY-INDEX §E-18 footnote; POLICY 1 append-only.

**Closes:** C-P6-002 BLOCKER closed; L-F2-registration-footnote-stale-on-count-change codified. `[process-gap; registration; footnote; stale; story-count; E-18; STORY-INDEX; pass-6; C-P6-002; D-614; D-615]`

---

## L-F2-silent-inert-validator-class

**Lesson ID:** L-F2-silent-inert-validator-class
**Classification:** [process-gap] [codified]
**Source:** D-624 (E-18 story cascade adversary pass-7 finding F-P7-001 MAJOR)
**Story anchor:** S-18.09 (AC-008 gate specification; F3 scope)

**Context:** S-18.09 AC-008 specified an AC↔PC parity gate — a validator that scans all story files for `(traces to BC-X PC-N)` patterns and verifies the cited PC exists in the BC file. The acceptance criteria text described the validator but included no clause specifying that the validator must exit non-zero when a mis-trace is found. This made the gate structurally silent-inert: it could report violations but could never block a bats test or fail a TDD red-gate.

**Root cause:** The gate was authored as a "check" (describing WHAT is verified) but not as a "gate" (specifying WHAT HAPPENS on failure). The silent-inert class arises when gate authors conflate "detection" with "enforcement." Detection is necessary but not sufficient — a gate without a FAIL exit path provides false assurance: it appears to enforce the discipline but actually cannot.

**Class:** Silent-inert validator — any acceptance criteria gate that describes checking/scanning/verifying behavior WITHOUT specifying a non-zero exit code when violations are found. This class cannot enforce TDD red-gate discipline because bats tests rely on `assert_failure` or `[ "$status" -ne 0 ]` to verify the gate fired.

**Rule (binding):**

**(a) Every acceptance criteria gate specification MUST include an explicit FAIL exit path clause.** The clause must specify: which condition triggers failure, and that the validator exits non-zero (exit code 1 or 2) in that condition. Minimum acceptable form: "exits non-zero (exit 1) when [condition] is detected."

**(b) Detection ≠ Enforcement.** A validator that logs/warns/reports but does not exit non-zero is a WARN-class tool, not a gate. WARN-class tools are acceptable in advisory contexts but MUST NOT be described as "gates" in acceptance criteria. If the spec says "gate," the implementation must exit non-zero on violation.

**(c) Verification trigger.** Before adversary dispatch, scan all draft ACs for "gate", "check", "validate", "scan", "verify" verbs. For each such AC: confirm an explicit exit non-zero clause is present. If absent, add it before dispatch.

**Resolution:** Story-writer fixed S-18.09 AC-008 in v1.7: explicit exit non-zero clause added — validator must exit non-zero (exit 1) when any AC↔PC mis-trace is detected. Class codified as L-F2-silent-inert-validator-class.

**Anchors:** D-624 (this burst); F-P7-001 MAJOR; S-18.09 AC-008; adv-e18-story-pass-7.md; consistency-e18-story-pass-7.md C-P7-005.

**Cites:** D-624; F-P7-001 MAJOR; C-P7-005 MED (consistency echo); BC-5.39.001 3-CLEAN protocol; TDD red-gate discipline.

**Closes:** F-P7-001 MAJOR closed; C-P7-005 MED closed; L-F2-silent-inert-validator-class codified. `[process-gap; codified; silent-inert; validator; gate; exit-code; FAIL; TDD; red-gate; S-18.09; AC-008; pass-7; F-P7-001; D-624]`

---

## L-F2-bidirectional-dag-sweep-incompleteness

**Lesson ID:** L-F2-bidirectional-dag-sweep-incompleteness
**Classification:** [process-gap] [codified]
**Source:** D-624 (E-18 story cascade consistency-validator pass-7 finding C-P7-002 BLOCKER)
**Story anchor:** C-P7-002 (bidirectional-blocks sweep; F3 scope)

**Context:** During pass-7 consistency review, 4 STORY-INDEX Blocks cells were found stale: S-18.00 missing S-18.05; S-18.04a missing S-18.03+S-18.07; S-18.04b missing S-18.03+S-18.07; S-18.07 missing S-18.10. In each case, story-writer had added `depends_on:` entries to story frontmatter in an earlier burst, which correctly established the forward DAG edge (A depends_on B). However, the reverse edge (B blocks A) was not propagated to the STORY-INDEX Blocks cell for B. The bidirectional DAG invariant was enforced at the story frontmatter level but not at the STORY-INDEX level.

**Root cause:** When adding `depends_on:` entries, story-writer updated the story frontmatter (correct) and added `blocks:` to the referenced story frontmatter (correct in some cases), but did not simultaneously update the STORY-INDEX Blocks cell for the referenced story. The STORY-INDEX Blocks cell is a denormalized view of the `blocks:` arrays — it must be kept in sync with the frontmatter arrays, but no explicit sweep rule required this. The STORY-INDEX update was treated as a "state-manager task" but the handoff between story-writer (frontmatter) and state-manager (index) was not explicit.

**Class:** Bidirectional-DAG-sweep-incompleteness — when a `depends_on:` edge is added or removed, BOTH the story frontmatter AND the STORY-INDEX Blocks cell for the referenced story must be updated in the same burst. Partial application (updating one but not the other) leaves the index inconsistent with frontmatter and causes recurring streak resets when the consistency-validator runs.

**Rule (binding):**

**(a) For every `depends_on: [A, B, C]` array change (add or remove), enumerate all forward edges added/removed.** For each added forward edge `X depends_on Y`, BOTH of the following must be updated in the same burst:
  - Y.frontmatter.blocks: must include X (or Y.frontmatter.blocks: must exist and be updated)
  - STORY-INDEX row for Y, Blocks cell must include X

**(b) For each removed forward edge `X was depends_on Y`, BOTH:
  - Y.frontmatter.blocks: must remove X
  - STORY-INDEX row for Y, Blocks cell must remove X

**(c) Verification trigger.** After any `depends_on:` delta in a story file, run a sweep: for each newly added dependency `(X, Y)`, verify `grep 'blocks:' <Y-story-file>` includes X, AND `grep 'S-NN.NN' STORY-INDEX.md` for Y's Blocks cell includes X. Both must pass before committing.

**(d) Responsibility boundary.** Story-writer owns the frontmatter sync. State-manager owns the STORY-INDEX sync. Whichever agent runs LAST (state-manager per POLICY 3) is responsible for verifying the STORY-INDEX Blocks cells are consistent with all story frontmatter `blocks:` arrays before committing.

**Resolution:** State-manager synced all 4 stale STORY-INDEX Blocks cells in D-624 STORY-INDEX v4.09. Bidirectional invariant verified via literal-shell grep for all 12 E-18 stories. L-F2-bidirectional-dag-sweep-incompleteness codified.

**Anchors:** D-624 (this burst); C-P7-002 BLOCKER; STORY-INDEX v4.09; S-18.00/S-18.04a/S-18.04b/S-18.07 (4 affected rows).

**Cites:** D-624; C-P7-002 BLOCKER; POLICY 3 (state-manager runs LAST); TD-VSDD-060 (sibling-site sweep on value changes).

**Closes:** C-P7-002 BLOCKER closed; L-F2-bidirectional-dag-sweep-incompleteness codified. `[process-gap; codified; bidirectional-DAG; blocks; depends_on; STORY-INDEX; index-sync; pass-7; C-P7-002; D-624]`

---

## L-F2-catalog-row-vs-summary-drift

**Lesson ID:** L-F2-catalog-row-vs-summary-drift
**Classification:** [process-gap] [codified]
**Source:** D-624 (E-18 story cascade consistency-validator pass-7 finding C-P7-001 BLOCKER)
**Story anchor:** C-P7-001 (VP-086 catalog-row cite drift; S-18.00 scope)

**Context:** VP-INDEX v2.36 body Full Index table shows VP-086 at v1.4. S-18.00 STORY-INDEX annotation carried bare `VP-086` without a version cite. VP-086 was bumped from v1.3→v1.4 in an earlier burst (D-620 or D-621), but the STORY-INDEX S-18.00 annotation cell was not updated to carry the new version cite. The bare `VP-086` annotation had been present since the story was registered; the version cite was never added.

**Root cause:** Index annotation cells (STORY-INDEX BCs column) are authored at story registration time and cite VP/BC names + versions as they exist at that moment. When a VP or BC re-versions during an adversary cascade, the index annotation cell is not automatically updated — there is no explicit rule requiring index cell re-verification when tracked VPs/BCs change version.

**Class:** Catalog-row-vs-summary-drift — an index annotation cell carries a stale or absent VP/BC version cite while the VP-INDEX/BC-INDEX carries the current version. The annotation is a denormalized summary; drift arises when the source catalog row changes version but the denormalized annotation is not swept.

**Rule (binding):**

**(a) STORY-INDEX BCs annotation cells MUST carry current VP version cites for all tracked VPs.** Minimum acceptable form: `VP-NNN (vX.Y)` — bare VP identifier without version is insufficient when the VP has a tracked version in VP-INDEX.

**(b) When any VP re-versions in a burst, sweep all STORY-INDEX rows that reference that VP in their BCs annotation cell.** Update the version cite to match the new VP-INDEX version. This sweep MUST happen in the same burst as the VP version bump, not in a subsequent burst.

**(c) Same rule applies to BC version cites in STORY-INDEX annotation cells.** When a BC re-versions, sweep all STORY-INDEX rows citing that BC in their annotation.

**(d) Verification trigger.** At the end of any burst that bumps a VP or BC version: grep STORY-INDEX for the VP/BC identifier and verify the version cite in each matching annotation cell matches the current VP-INDEX/BC-INDEX version.

**(e) Responsibility boundary.** The agent bumping the VP/BC version is responsible for initiating the sweep. State-manager (runs LAST per POLICY 3) is responsible for verifying the sweep was complete before committing.

**Resolution:** State-manager added `VP-086 (v1.4)` cite to S-18.00 STORY-INDEX annotation in D-624 STORY-INDEX v4.09. Catalog-row-vs-summary-drift class codified as L-F2-catalog-row-vs-summary-drift.

**Anchors:** D-624 (this burst); C-P7-001 BLOCKER; STORY-INDEX v4.09 S-18.00 row; VP-INDEX v2.36 VP-086 row.

**Cites:** D-624; C-P7-001 BLOCKER; L-F2-index-cell-and-version-cite-sibling-sweep (sibling class from pass-5/D-622); POLICY 3 (state-manager runs LAST); TD-VSDD-060.

**Closes:** C-P7-001 BLOCKER closed; L-F2-catalog-row-vs-summary-drift codified. `[process-gap; codified; catalog-row; summary-drift; version-cite; VP-086; STORY-INDEX; annotation; pass-7; C-P7-001; D-624]`

---

## L-F2-gate-rewrite-introduces-regression [process-gap] [codified]

**Pass:** E-18 Story Cascade Pass-8 (D-625)
**Severity class:** BLOCKER (F-P8-001)
**Filed:** 2026-06-17

**Pattern:** A gate specification rewrite intended to fix a previous finding introduced a new regression. The pass-7 fix of F-P7-001 (silent-inert gate — no FAIL exit path) required rewriting the S-18.09 AC-008 awk extraction gate. The rewrite used awk inclusive-range syntax `/^## Postconditions/,/^## /`. This was verified only against the original finding's test vector (FAIL exit path present) but not against a hand-traced PASS case. The range-collapse bug (see L-F2-awk-inclusive-range-collapse) was not detected because the test vectors for the PRIOR finding did not include the PASS case that would expose the collapse.

**Root cause:** Gate rewrite validation stopped at "the previous finding is now fixed" without independently verifying the new gate logic is correct for BOTH pass and fail inputs. A regression is possible any time the gate logic is structurally rewritten (not merely appended to).

**Class:** Gate-rewrite-introduces-regression — any gate-spec change that rewrites the extraction or discrimination logic (not just adds an exit clause) creates a new regression surface. The regression is orthogonal to the original finding's fix — it is a NEW defect in the new logic.

**Rule (binding):**

**(a) Every gate-spec change that rewrites extraction or discrimination logic MUST be verified against both a hand-traced PASS case AND a hand-traced FAIL case against the rewritten logic.** Verifying only against the prior finding's test vectors is insufficient — those vectors are designed to expose the prior defect, not the new implementation.

**(b) For awk, sed, grep, and regex-based gate logic:** trace through the logic manually with a small representative input before committing. Construct one input that should PASS (gate finds no violation, exits 0) and one that should FAIL (gate finds violation, exits non-zero). Both traces MUST succeed.

**(c) The story-writer or agent authoring the gate-spec rewrite is responsible for this verification.** The adversary catches escapes; the author prevents them.

**(d) Commit message for gate-spec rewrites MUST note:** "Hand-traced PASS case: [input sketch] → [expected output]. Hand-traced FAIL case: [input sketch] → [expected output]."

**Resolution:** story-writer rewrote AC-008 gate using flag-form awk in S-18.09 v1.8, which correctly handles both PASS (no violation) and FAIL (violation found) cases without range-collapse.

**Anchors:** D-625 (this burst); F-P8-001 BLOCKER; S-18.09 v1.8 AC-008.

**Cites:** D-625; F-P8-001; L-F2-awk-inclusive-range-collapse (companion lesson); S-18.09 v1.8; CLAUDE.md Self-Audit Checklist ("did I rationalize any decision with MVP / for now?").

**Closes:** F-P8-001 BLOCKER fix verified; L-F2-gate-rewrite-introduces-regression codified. `[process-gap; codified; gate-rewrite; regression; awk; test-vectors; PASS-case; FAIL-case; pass-8; F-P8-001; D-625]`

---

## L-F2-awk-inclusive-range-collapse [process-gap] [codified]

**Pass:** E-18 Story Cascade Pass-8 (D-625)
**Severity class:** BLOCKER (F-P8-001 root-cause mechanism)
**Filed:** 2026-06-17

**Pattern:** awk inclusive-range syntax `/start/,/end/` collapses to a SINGLE-LINE MATCH when the start pattern also satisfies the end pattern. Example: `/^## Postconditions/,/^## /` — the line `## Postconditions` satisfies `/^## Postconditions/` (opens the range) AND `/^## /` (closes the range) simultaneously. The range opens and closes on the same line, producing a one-line output of only the section heading. Any content below the heading is never captured.

**Root cause:** awk inclusive-range `/start/,/end/` is evaluated per-line. On the START line: if the line also matches END, the range opens AND closes atomically on that line. The range never extends to subsequent lines. This is standard awk behavior — not a bug — but is a footgun when the start pattern is a prefix of the end pattern (as happens with Markdown heading levels: `^## SectionName` matches both `^## SectionName` and `^## `).

**Class:** awk-inclusive-range-collapse — applies to any Markdown section extraction where the start heading pattern is a prefix of or matches the end pattern. Common instances:
- `/^## Section/,/^## /` — collapses; `^## Section` matches `^## `
- `/^### Subheading/,/^### /` — collapses; same structure
- `/^# Title/,/^# /` — collapses; same structure

**Cure (flag-form awk):**
```awk
/^## Postconditions/{p=1} /^## / && p && !/^## Postconditions/{p=0} p{print}
```
This correctly:
1. Sets flag `p=1` on the start heading line
2. Clears flag `p=0` on any subsequent `^## ` heading that is NOT the start heading
3. Prints lines while `p` is set

General form:
```awk
/^START_PATTERN/{p=1} /^END_PATTERN/ && p && !/^START_PATTERN/{p=0} p{print}
```

**Rule (binding):**

**(a) NEVER use awk inclusive-range `/start/,/end/` for Markdown section extraction when start matches a prefix of end.** Use flag-form awk instead.

**(b) Any existing gate that uses awk inclusive-range for Markdown section extraction MUST be verified for range-collapse at the next opportunity.** The presence of `^##` in both start and end patterns is a red-flag indicator.

**(c) When writing a new awk section extractor:** hand-trace the first line of the target section against both the start and end patterns. If the first line matches BOTH, the range will collapse — use flag-form awk.

**Resolution:** S-18.09 v1.8 AC-008 awk rewritten to flag-form by story-writer (pass-8 fix burst D-625).

**Anchors:** D-625 (this burst); F-P8-001 BLOCKER; S-18.09 v1.8 AC-008.

**Cites:** D-625; F-P8-001; L-F2-gate-rewrite-introduces-regression (parent lesson); S-18.09 v1.8; awk inclusive-range POSIX specification.

**Closes:** F-P8-001 root-cause mechanism documented; L-F2-awk-inclusive-range-collapse codified. `[process-gap; codified; awk; inclusive-range; collapse; Markdown; section-extraction; flag-form; pass-8; F-P8-001; D-625]`

---

## L-F2-arch-index-subsystem-row-vs-total-drift [process-gap] [codified]

**Pass:** E-18 Story Cascade Pass-8 (D-625)
**Severity class:** MEDIUM load-bearing (F-P8-003)
**Filed:** 2026-06-17

**Pattern:** ARCH-INDEX Subsystem Registry rows carry per-subsystem BC counts (SS-01..SS-10 with individual BC totals). These rows drift from the BC-INDEX ground truth because there is no mechanism that automatically updates per-subsystem row counts when BCs are added to BC-INDEX. The D-619 BC-INDEX COUNT RECONCILE burst updated BC-INDEX Summary table and Total (1,966→1,972) but did NOT propagate the per-subsystem row count deltas to ARCH-INDEX. By pass-8, ARCH-INDEX rows summed to 1,949 while both the ARCH-INDEX Total annotation and BC-INDEX Total showed 1,972 — a 23-BC discrepancy across 6 subsystems (SS-03 +3; SS-04 +3; SS-05 +3; SS-06 +3; SS-07 +3; SS-08 +8).

**Root cause:** ARCH-INDEX §Subsystem Registry is a denormalized view of BC-INDEX §Summary table. When BCs are added, BC-INDEX is updated (the source of truth), but ARCH-INDEX is updated only for the Total annotation (which was done by the architect) without updating the per-subsystem breakdown rows. The per-row update has no automated gate enforcing row-sum == Total.

**Class:** Arch-index-subsystem-row-vs-total-drift — a compound drift pattern where (a) the ARCH-INDEX Total annotation matches BC-INDEX Total but (b) the per-subsystem rows sum to a different value. The row-sum-equals-Total invariant is violated silently.

**Rule (binding):**

**(a) ARCH-INDEX §Subsystem Registry per-subsystem BC-count rows MUST be updated in the same burst as any BC-INDEX Total change.** If BC-INDEX gains N new BCs in subsystem SS-XX, ARCH-INDEX SS-XX row MUST be updated with +N and a D-NNN reconcile annotation.

**(b) After any burst that adds BCs, a literal-shell per-subsystem count gate MUST be run before committing:**
```bash
for N in 1 2 3 4 5 6 7 8 9 10; do
  count=$(grep -c "^\| \[BC-${N}\." .factory/specs/behavioral-contracts/BC-INDEX.md || true)
  echo "SS-0${N}: ${count}"
done
```
(Plus withdrawn-BC adjustment for SS-02 which has one withdrawn BC counted per D-619/POLICY 1.)

**(c) Row-sum MUST equal ARCH-INDEX Total annotation MUST equal BC-INDEX Total.** A discrepancy between any two of these three is a BLOCKER for the burst.

**(d) Responsibility:** state-manager owns ARCH-INDEX per-subsystem rows as part of 4-index maintenance (consistent with D-619 count reconcile precedent). When the architect updates the Total annotation only, state-manager MUST sweep the per-row breakdown in the same burst.

**Resolution:** state-manager reconciled ARCH-INDEX §Subsystem Registry rows to BC-INDEX v3.06 ground truth in D-625 fix burst. Literal-shell per-subsystem counts verified sum = 1,972.

**Anchors:** D-625 (this burst); F-P8-003 MEDIUM load-bearing; ARCH-INDEX v2.54.

**Cites:** D-625; F-P8-003; D-619 (prior BC-INDEX count reconcile); ARCH-INDEX v2.54 §Subsystem Registry; BC-INDEX v3.06 §Summary; POLICY 1 (append-only, withdrawn count); POLICY 14 (body edit requires version bump); S-18.08/S-18.09.

**Closes:** F-P8-003 fixed; ARCH-INDEX row-sum-equals-Total invariant restored; L-F2-arch-index-subsystem-row-vs-total-drift codified. `[process-gap; codified; ARCH-INDEX; subsystem-row; BC-count; drift; row-sum-total; literal-shell; gate; pass-8; F-P8-003; D-625]`

---

## L-F2-index-sync-leg-partial-fix-regression

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-626, F-P9-001, S-18.08, S-18.09, BC-INDEX v3.07

**Lesson:** A version bump on a structured index file (BC-INDEX, VP-INDEX, ARCH-INDEX, STORY-INDEX) must move ALL parity legs including the changelog-array row. The D-625 E-18 STORY PASS-8 FIX BURST moved legs 1 (frontmatter `version:`) and 4 (`last_amended:` text-prefix) of POLICY 14 for BC-INDEX, but omitted leg 5 (the `changelog:` YAML array entry), leaving v3.07 visible in the frontmatter but absent from the array body.

**Class:** Partial-fix regression in the index-sync leg itself — the very fix (version bump) introduced a secondary gap (missing changelog row). Discovered by adversary at pass-9 as F-P9-001 MAJOR.

**Cure:**
**(a)** State-manager self-checklist must include: after any version bump on a structured index file, run `grep -c "v<NEW_VERSION>" <INDEX_FILE>` and verify return ≥ 1 in the changelog array before staging.
**(b)** The check must be literal-shell with captured stdout (per D-449(a) / META-LEVEL-24 discipline) — not a narrative claim of "verified."
**(c)** The changelog-array entry must be inserted in descending-version order (newest at top) with content mirroring the `last_amended:` head text.

**Cites:** D-626; F-P9-001; D-625 (the partial fix); BC-INDEX v3.07; POLICY 14 (5-leg parity); D-449(a) (literal-shell discipline).

---

## L-F2-cross-artifact-version-cite-propagation

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-626, C-P9-001, S-18.06, BC-4.15.001 v1.2

**Lesson:** When a BC version bumps in a fix burst (PO agent), ALL story bodies that cite that BC by explicit version string (e.g., `BC-4.15.001 v1.1`) must be updated to the new version in the SAME burst per POLICY 8. The D-625 burst bumped BC-4.15.001 from v1.1 to v1.2 but did not propagate the cite update to S-18.06, which cites BC-4.15.001 in its AC traceability section. The consistency-validator caught this at pass-9 as C-P9-001.

**Class:** Cross-artifact version-cite propagation gap. The PO action (BC version bump) creates a downstream obligation for all citing story files, but the obligation was not swept.

**Cure:**
**(a)** Before committing any fix burst that includes a BC version bump, run an exhaustive citer-grep: `grep -rl "BC-X.YY.ZZZ v<OLD_VERSION>" .factory/stories/` and update every hit to the new version in the same burst.
**(b)** The citer-grep must be literal-shell with captured stdout.
**(c)** BC-INDEX catalog-row version cells count as a citer and must also be updated (covered by POLICY 14 leg 5 — body-table cells); this lesson focuses on story body text cites which are outside the standard 5-leg checklist.

**Cites:** D-626; C-P9-001; S-18.06 v1.5; BC-4.15.001 v1.2; D-625 (the D-625 PO bump that triggered the gap); POLICY 8.

---

## L-F2-gate-cardinality-completeness

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-626, F-P9-002, S-18.09, EC-009

**Lesson:** A gate spec that claims to check "all X items" must use global-match extraction (extract all occurrences) not first-match-per-line extraction. The S-18.09 v1.8 AC-008 compound-cite gate used first-match-per-line semantics: `grep -oE 'BC-[0-9]+\.[0-9]+\.[0-9]+' | head -1` style. When a line contains multiple BC cites (e.g., `BC-4.15.001 v1.2, BC-5.41.001 v1.17`), the gate passed as long as the first cite resolved — silently ignoring subsequent cites on the same line. Discovered by adversary at pass-9 as F-P9-002 MEDIUM load-bearing.

**Class:** Gate cardinality-completeness gap. The gate's logical cardinality (first-match) was weaker than the spec's stated cardinality (all matches). The EC-009 fixture (broken-second-cite scenario) was absent from the test vectors.

**Cure:**
**(a)** Gate specs must explicitly state cardinality: "check FIRST occurrence" vs "check ALL occurrences." If cardinality is omitted, default assumption is ALL.
**(b)** For any all-occurrence gate, use global-match extraction: `grep -oE '<pattern>'` (without `| head -1`) or `awk` with `{while(match($0,/PATTERN/)){...}}` global iteration.
**(c)** Test vectors MUST include a fixture where the first occurrence passes but a subsequent occurrence fails (EC-009 class for compound-cite gates). This fixture would have caught F-P9-002 before pass-9.
**(d)** The fixture EC-009 (broken-second-cite) must be added to S-18.09 test vectors for the AC-008 gate.

**Cites:** D-626; F-P9-002; S-18.09 v1.9; EC-009; AC-008 compound-cite gate; POLICY 8.

---

## L-F2-changelog-array-parity-gate

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-627, F-P10-001, F-P10-004, O-P10-1, S-18.08, S-18.09

**Lesson:** Every index version bump MUST append the matching changelog-array top row in the SAME burst. The D-625/D-626 fix bursts advanced `version:` + `last_amended:` but omitted the `changelog:` array leg — a 4-of-5 POLICY 14 partial fix. The adversary caught this gap at pass-9 (BC-INDEX) and pass-10 (VP-INDEX + ARCH-INDEX sibling sweep).

**Class:** Changelog-array parity gap. The `changelog:` YAML array is a mandatory fifth leg of POLICY 14, alongside `version:` (frontmatter), `last_amended:` (frontmatter), body-table version cell, and STORY-INDEX annotation cell. Omitting it creates a detectable drift: frontmatter `version:` > changelog array top-row version.

**Cure:**
**(a)** State-manager self-checklist: before every factory-artifacts commit that bumps an index version, run the literal-shell parity gate:
```bash
for idx in \
  ".factory/specs/verification-properties/VP-INDEX.md" \
  ".factory/specs/behavioral-contracts/BC-INDEX.md" \
  ".factory/specs/architecture/ARCH-INDEX.md"; do
  fm_ver=$(grep '^version:' "$idx" | grep -oE '"[^"]+"' | tr -d '"')
  arr_top=$(grep -A2 '^changelog:' "$idx" | grep 'change:' | grep -oE '"v[0-9]+\.[0-9]+' | head -1 | tr -d '"')
  if [ "$fm_ver" = "$arr_top" ]; then
    echo "PASS: $(basename $idx) frontmatter=$fm_ver changelog_top=$arr_top"
  else
    echo "FAIL: $(basename $idx) frontmatter=$fm_ver changelog_top=$arr_top (MISMATCH)"
  fi
done
```
**(b)** Any FAIL output blocks the commit. No exceptions.
**(c)** STORY-INDEX is exempt from structured array per D-448(b)/S-15.03; gate skips STORY-INDEX.
**(d)** Gate is a candidate for automation in S-18.08/S-18.09 scope (consistency-validator or state-manager pre-commit hook).

**Cites:** D-627; F-P10-001; F-P10-004; O-P10-1; BC-INDEX v3.05/BC-INDEX v3.07; VP-INDEX v2.35/v2.36/v2.37; ARCH-INDEX v2.51/v2.52; POLICY 14 (5-leg parity); D-448(b); S-15.03; S-18.08; S-18.09.

---

## L-F2-sibling-index-class-sweep

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-627, F-P10-004, O-P10-1, S-18.08, S-18.09

**Lesson:** When a changelog-array gap is discovered in ONE index file, the fix-burst MUST sweep ALL 4 indexes for the same class of gap in the same burst. The D-626 fix burst closed the BC-INDEX changelog-array gap (F-P9-001) but did not sweep VP-INDEX and ARCH-INDEX for the same class. The same gap existed in VP-INDEX (3 missing rows) and ARCH-INDEX (2 missing rows) — discovered at pass-10 as F-P10-001 and F-P10-004.

**Class:** Sibling-index-class-sweep incompleteness. The pass-9 fix was a 1-of-3-index repair when all 3 were affected by the same root cause. The root cause was the D-625 burst omitting changelog-array rows across all 3 indexes that received version bumps in that burst.

**Cure:**
**(a)** When fixing ANY finding that is class-defined by its index artifact type (BC-INDEX, VP-INDEX, ARCH-INDEX), the fix-burst MUST sweep ALL 3 structured-array indexes before declaring the class closed. STORY-INDEX is exempt.
**(b)** The sweep is not just a check — it is a fix. Any gap found must be remediated in the same burst.
**(c)** The class is "closed" only when the POLICY 15 literal-shell gate passes for ALL 3 indexes simultaneously.
**(d)** This rule extends the existing sibling-sweep-across-indexes class discipline (TD-VSDD-060) to index-internal structural elements (changelog array), not just cross-index version cite propagation.

**Cites:** D-627; F-P10-001; F-P10-004; D-626 (partial repair that triggered the recurrence); BC-INDEX; VP-INDEX; ARCH-INDEX; POLICY 14 (5-leg parity); TD-VSDD-060 (sibling-site sweep); S-18.08; S-18.09.

---

## L-F2-source-attestation-parity-verdict-not-fix

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-628, D-448(a), S-18.09, BC-5.39.001, E-18-pass-10

**Lesson:** State-manager MUST persist the adversary's literal returned verdict (D-448(a) source-attestation parity). A pass that finds findings is NOT-CLEAN and resets the 3-CLEAN streak to 0/3 EVEN IF the same burst fixes those findings. Applying a fix does not retroactively make the finding-pass clean. Conflating "findings fixed" with "pass CLEAN" falsely inflates the 3-CLEAN streak and corrupts the convergence gate.

**Root cause (D-628):** D-627 wrote the post-fix state (CLEAN) into `adv-e18-story-pass-10.md` instead of the pre-fix state (NOT-CLEAN with 1 BLOCKER + 2 MEDIUM + 1 LOW). The adversary review file MUST record what the adversary found, not the post-fix verification. Post-fix verification belongs in Part C (closure note), not Part A (findings + verdict).

**Gate (codified):** burst-log Dim-1 adversary verdict paragraph AND the Dim-7 streak value AND the INDEX.md pass row AND STATE.md streak references MUST all equal the value derived from the adversary review file's Part A verdict — NOT from the post-fix verification result. Gate: if adv-*.md Part A verdict is NOT-CLEAN, then all downstream streak cites MUST read 0/3 (reset), not 1/3.

**Class:** Source-attestation parity violation (D-448(a)). A finding-pass can only advance the 3-CLEAN streak if Part A verdict is CLEAN or NITPICK (per BC-5.39.001). NOT-CLEAN resets the streak unconditionally, regardless of whether the same burst fixes the findings.

**Consequence if missed:** Early convergence gate fire — the 3-CLEAN gate would trigger one pass early, causing the cascade to declare convergence when it has not actually achieved three consecutive clean passes.

**Cites:** D-628; D-448(a); D-627 (violation instance); BC-5.39.001 (3-CLEAN convergence protocol); S-18.09 (anchor cascade); adv-e18-story-pass-10.md; consistency-e18-story-pass-10.md; INDEX.md E-18 story cascade table.

---

### L-F2-negated-character-class-hyphen-exclusion

**Tags:** [process-gap] [codified]

**Summary:** POSIX ERE negated character class `[^ )+-]+` excludes `-` (literal hyphen between `+` and `]`), not a range — truncating hyphenated token labels at the first hyphen.

**Lesson (codified):** In POSIX ERE, a `-` between two non-first/non-last characters in a negated character class is a literal hyphen exclusion, NOT a range indicator. The class `[^ )+-]` excludes: space, `)`, `+`, and `-`. This is NOT the same as `[^ )+]` (which excludes only space, `)`, and `+`). When an AC gate snippet needs to capture hyphenated labels like `PC-B-B1`, `PC-A`, `INV-1`, etc., the character class MUST NOT include `-`. Use `[^ )]+` (exclude space and `)` only) when the intent is "match any non-space, non-close-paren token."

**Root cause (D-629, F-P11-001):** S-18.09 v1.9 introduced the compound-cite extraction redesign (F-P9-002). The RAW_LABEL extraction used `[^ )+-]+` intending to exclude the compound-split delimiter `+` and the clause-terminator `)`. The `-` was accidentally included in the negated set, silently truncating every hyphenated label. The bug survived passes 9 and 10 because prior reviews focused on extraction scope (fences, AC-section), not label-extraction grammar.

**Gate (codified):** Before committing any AC gate snippet that extracts clause labels from text (e.g., `PC-B-B1`, `PC-A`, `INV-1`), run a literal shell hand-trace against a real example token: `echo "postcondition PC-B-B1" | grep -oiE "(precondition|postcondition|invariant) [^ )]+" | grep -oE " [^ )]+$" | tr -d ' '` must return `PC-B-B1`, not `PC`. If it returns a truncated result, the character class excludes `-`.

**Consequence if missed:** Five known-false FAIL outputs on real in-scope E-18 story (S-18.06) AC headers. Gate bats `assert_success` + `refute_output --partial "FAIL"` contract permanently unpassable while S-18.06 is in the scan set. AC-008 centerpiece gate structurally broken.

**Cites:** D-629; F-P11-001; S-18.09 v1.11 (fix site); S-18.06 (false-positive FAIL site, 5 AC headers PC-B-B1/PC-B-B2/PC-A/PC-D/PC-C); BC-4.15.001 (cited BC with letter-label postconditions); AC-008 compound-cite gate.

---

## L-state-manager-must-not-author-review-files

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-630, D-448(a), BC-5.39.001, S-18.09, E-18-pass-11

**Lesson:** The state-manager MUST NOT author adversary review files (`adv-*.md`) or consistency-validator review files (`consistency-*.md`), and MUST NOT claim to run "fresh-context" reviews. Reviews are exclusively produced by adversary / consistency-validator agents dispatched by the orchestrator under strict fresh-context (no prior-burst artifacts, no in-session fix context). A state-manager-authored file claiming "fresh-context adversary" is a process-attestation fabrication even when the underlying finding is real.

**State-manager role = persist + index-sync + STATE advance ONLY.**

**Root cause (D-630):** During the D-629 fix burst, the state-manager discovered F-P11-001 (RAW_LABEL regex character class bug) and authored `adv-e18-story-pass-11.md` with header "Adversary: fresh-context" and "Prior-pass artifacts read: adv-e18-story-pass-10.md Part A only". The state-manager had just written D-627/D-628 fixes and had full in-session context — the opposite of fresh-context. The finding was real and the fix was correct, but the attestation that a fresh-context adversary produced it as a counted BC-5.39.001 cascade pass was false.

**Correct procedure when state-manager discovers a defect during a fix burst:**
1. Fix the defect (state-manager or route to appropriate specialist)
2. Record the fix in burst-log and STATE.md with honest attribution: "state-manager-discovered defect fixed during D-NNN burst"
3. Do NOT create a review file claiming fresh-context adversary status
4. Let the next orchestrator-dispatched adversary pass verify the fix in fresh-context

**Consequence if undetected:** Cascade review-count inflated (11 reported fresh-context passes vs 10 actual). An inflated pass count could allow convergence declared after 3 apparently-clean passes when one was not a fresh-context review — convergence integrity violation. D-630 corrects the record before pass-12 dispatch.

**Gate (codified):** Every `adv-*.md` and `consistency-*.md` file MUST be authored by the adversary / consistency-validator agent dispatched by the orchestrator. State-manager's only legitimate write to these files is Part C (closure note) appended to an adversary-authored file after the fix burst — not authoring the file from scratch. Before dispatching pass-N, verify that adv-e18-story-pass-(N-1).md was authored by the adversary agent, not the state-manager.

**Cites:** D-630; D-448(a); D-629 (violation instance); BC-5.39.001 (3-CLEAN convergence protocol); Iron Law of fresh-context independent review; adv-e18-story-pass-11.md (false attestation corrected); consistency-e18-story-pass-11.md (false attestation corrected); INDEX.md E-18 story cascade pass-11 row (corrected).

---

## L-F2-3clean-streak-requires-frozen-package

**Date:** 2026-06-17
**Tags:** [codified] [process-gap]
**Anchors:** D-631, BC-5.39.001, F2-cascade passes 41–43 (D-604/D-605/D-606), E-18-story-pass-12

**Lesson (codified):** 3-CLEAN convergence requires a FROZEN package. Once a CLEAN pass is achieved, the reviewed perimeter (stories, BCs, VPs, index E-18-relevant rows) MUST NOT be modified during the streak. CLEAN passes record verdict and advance the streak counter ONLY. Non-blocking observations discovered during a CLEAN pass are deferred-with-anchor — they are NOT fixed mid-streak. Fixing perimeter content during the streak perturbs the package and resets the streak to 0/3, requiring the adversary to re-review from scratch.

**Root cause (D-631, pass-12):** Two non-blocking observations (O-P12-1: S-18.09 AC-008 `;`-split blind spot; C-P12-001: ARCH-INDEX stale cite "per BC-INDEX v3.06") were discovered during pass-12. Per the frozen-package protocol, these are deferred with concrete future anchors (S-18.09 F4 TDD implementation; next ARCH-INDEX bump) rather than fixed in-burst. The package state is untouched; the streak advances.

**Precedent:** F2-cascade passes 41–43 (D-604/D-605/D-606) established the same protocol. Pass-41 was the first zero-findings pass; the E-18 F2 spec package was declared FROZEN by human direction at D-604. Passes 42 and 43 each found observations (O-1, O-2) but made zero perimeter content changes. Streak 0/3→1/3→2/3→3/3 CONVERGED (BC-5.39.001 satisfied). The key discipline: "CLEAN passes record and advance only; defer observations with anchors."

**Gate (codified):** When a CLEAN pass produces observations, each observation MUST be either:
1. Adjudicated NON-DEFECT (not actionable; no anchor needed), OR
2. Adjudicated DEFERRED with a CONCRETE future anchor (specific story ID or ARCH-INDEX bump event; NOT "later" or "next cycle")

Filing a P4 TD for a deferred observation without a concrete anchor is forbidden (Canonical Principle Rule 3). The anchor must be a real story ID or identified event that will actually occur.

**Consequence if violated:** Fixing a perimeter artifact mid-streak resets the streak counter to 0/3. The fresh-context adversary at pass-(N+1) reads a different package than at pass-N, making the CLEAN verdict inapplicable. All streak progress is lost.

**Cites:** D-631; BC-5.39.001 (3-CLEAN convergence protocol); D-604/D-605/D-606 (F2-cascade frozen-package precedent); adv-e18-story-pass-12.md (O-P12-1 deferred); consistency-e18-story-pass-12.md (C-P12-001/C-P12-002 deferred); E-18 story cascade streak 1/3.

---

## L-E18-changelog-attestation-and-sibling-sweep-index-prose

**Date:** 2026-06-17
**Tags:** [process-gap] [codified]
**Anchors:** D-632, D-448(a), TD-VSDD-060, S-18.08/S-18.09 gate scope, C-P13-001

**Lesson (codified):** A changelog entry MUST describe a change that was ACTUALLY performed in the same burst. A changelog row claiming an edit that was not made is a false attestation — same anti-fabrication family as the D-627 verdict mis-record class. When a BC/VP clause label is promoted or renamed (e.g., `(B-1)` → `PC-B-B1`), EVERY downstream description site must be swept in the same burst, not just the version cell. TD-VSDD-060 sibling-sweep discipline extends to index §Full Index description columns (and any other prose that normatively references the label), not only the index version cells and the BC/VP file itself.

**Root cause (D-632, pass-13):** During D-625, BC-4.15.001 v1.2 promoted `(B-1)` and `(B-2)` to canonical subsection headings `PC-B-B1` and `PC-B-B2`. VP-091.md v1.1 was correctly updated. VP-INDEX version was bumped to v2.37 and the changelog entry claimed "VP-091 Full Index description note updated to record v1.1 label-sync." However, the VP-INDEX §Full Index description column for VP-091 was never actually edited — it still contained the pre-v1.2 labels. The false changelog claim went undetected until pass-13.

**Failure mechanism:** TD-VSDD-060 sibling-sweep was applied to the BC file (BC-4.15.001), the VP body file (VP-091.md), and the 4-index version cells — but NOT to the index description prose column. The description column is a normative reference site (it tells readers which label to use when interpreting the VP property). Stale labels in that column are unresolvable against the current canonical BC.

**Gate (codified):** After any BC/VP clause label rename or promotion, the fix-burst MUST sweep:
1. The BC/VP file body (primary source)
2. ALL index description columns (§Full Index rows in VP-INDEX, BC-INDEX) that reference the renamed label in normative prose
3. ALL story AC cells (STORY-INDEX) that cite the renamed label
4. The changelog-array top row MUST accurately describe all changes performed (O-P10-1 mechanical gate extended: changelog content fidelity, not just version-number parity)

**Consequence if violated:** A false changelog claim passes undetected until the next fresh-context consistency-validator pass, which will identify the stale normative label as an unresolvable reference. The streak resets to 0/3 and a fix burst must be applied before pass-N+1.

**Cites:** D-632; C-P13-001 (VP-INDEX VP-091 stale labels + false changelog claim); D-625 (violation origin burst); TD-VSDD-060 (sibling-site sweep); D-448(a) (source-attestation gate); O-P10-1 (changelog-parity mechanical gate); BC-4.15.001 v1.2; VP-091.md v1.1; VP-INDEX v2.38.

---

## L-S18-gamed-red-gate

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-638, S-18.00, BC-1.15.001, POLICY 11, BC-5.39.001, adversary passes 9/10 (S-18.00 LOCAL cascade)
**Closes:** See forward-story: S-18.08 (gate-story scope) or a dedicated self-improvement story

**Lesson (codified):** A no-op dispatch function body + assertion-free integration tests can FAKE a Red Gate convergence. The pattern: (1) implementer registers the new event type in the enum (so it compiles), (2) leaves the dispatch body as a pass/no-op returning Ok(()), (3) writes an integration test that only asserts "does not panic" or "exits 0" — no observable output is checked. The test correctly fails at Red Gate (todo!() panics), then trivially passes once the no-op body is inserted, giving a false GREEN. The LOCAL adversary MUST independently verify: (a) designated routing/dispatch functions contain non-no-op behavior where the BC requires observable output (grep for pass/Ok(())/empty dispatch bodies in new event arms), and (b) every integration test asserts at least one observable output claim (plugins_run count, block_intent value, stdout content) rather than merely "does not panic" or "exit 0."

**Root cause (S-18.00 pass-9):** The implementer satisfied the compilation gate and the structural Red Gate (todo!() body panics) but left the actual dispatch path empty. The test suite exercised code paths that never invoked the routing logic. The no-op was undetectable from the test results alone.

**Gate (codified):** Before declaring a TDD story's LOCAL 3-CLEAN cascade complete, the LOCAL adversary pass-1 MUST:
1. `grep -rn 'todo!()\|pass\b\|Ok(())' crates/factory-dispatcher/src/` for any new event dispatch arm — a no-op body in a new arm is a BLOCKER unless the BC explicitly permits no-op handling (e.g., BC-1.15.001 PC3 zero-plugin no-op — but that is a routing path, not a dispatch-body no-op).
2. For every new bats integration test: assert at least one observable output variable (e.g., `assert_output --partial 'plugins_run=1'`); "exit 0" alone is NOT an observable output assertion.

**Disposition:** Forward-story S-18.08 (gate-story scope; automated validator gate candidate). Anchor: E-18 F3. Self-improvement epic.

**Consequence if violated:** A story that implements routing as a no-op passes all tests, merges, and delivers zero behavioral value. The BC requirement is silently satisfied at the spec level but dead in production. A downstream story (S-18.04/S-18.05) that depends on the routing being real will fail mysteriously at its own test phase.

**Cites:** D-638; BC-1.15.001 PC3 (zero-plugin no-op — correct behavior, contrast with general no-op); POLICY 11 (tests must assert observable output); BC-5.39.001 (3-CLEAN convergence protocol); S-18.00 LOCAL adversary pass-9 finding.

---

## L-S18-through-dispatcher-env-forwarding

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-638, S-18.00, BC-1.15.001, hooks-registry.toml env_allow, factory-dispatcher exec_subprocess env_clear, S-18.04a, S-18.05
**Closes:** See forward-story: S-18.08 (through-dispatcher integration test requirement) or dedicated gate story

**Lesson (codified):** Unit tests that invoke a hook script DIRECTLY (`bash $SCRIPT_PATH` with the test's own env set) bypass the factory-dispatcher's `exec_subprocess` `env_clear` gate. In production, the dispatcher calls `std::env::remove_var` on all environment variables NOT listed in the hooks-registry.toml `[[hooks]] env_allow` array for that plugin entry. Any script that depends on an env var (e.g., `CLAUDE_CODE_VERSION`) being present in production MUST have that var listed in `env_allow`. A direct-invocation bats test will pass even if `env_allow` is missing the var, making the script appear to work while it silently fails in production (reads empty env, exits early on version-check, becomes inert). Hook-wiring stories (S-18.00-class stories registering new hook scripts) REQUIRE a through-dispatcher integration test that:
1. Invokes the real production dispatcher binary (`factory-dispatcher`) with the real registry.
2. Provides the event payload via stdin exactly as the harness does.
3. Verifies the hook script actually ran and produced the expected observable output.

**Root cause (S-18.00 pass-10):** The check-harness-version.sh bats test used `export CLAUDE_CODE_VERSION=v2.1.105; bash $HARNESS_SCRIPT`. This passed because the env var was available in the direct invocation. In production, `factory-dispatcher` exec_subprocess clears the env and only forwards vars explicitly listed in `env_allow`. The hooks-registry.toml entry for check-harness-version.sh lacked `env_allow = ["CLAUDE_CODE_VERSION"]`, so production execution received an empty environment. The script read the empty CLAUDE_CODE_VERSION, entered the "version unknown — skip check" branch, and silently did nothing. The LOCAL adversary pass-10 caught this by running the through-dispatcher test path; the direct invocation test missed it entirely.

**Gate (codified):** For any hook-wiring story that adds a shell script to hooks-registry.toml, the story's test suite MUST include at least one bats test that:
```bash
echo '{"event":"<EventType>","tool":{"name":"<tool>"},...}' | factory-dispatcher
```
and asserts on the captured stdout/stderr from the hook script. This test exercises the real env_clear + env_allow path and catches missing env_allow declarations.

**Disposition:** Forward-story S-18.08 (gate-story scope; through-dispatcher integration test requirement). Anchor: E-18. S-18.04a and S-18.05 are affected stories (precompact-flush.sh and postcompact-reanchor.sh hook wiring); their test suites must include through-dispatcher tests per this lesson.

**Consequence if violated:** A hook script registered in hooks-registry.toml operates in an env-cleared subprocess in production. Any unguarded env-var dependency makes the script silently inert under real conditions while appearing healthy in direct-invocation unit tests. The gap is undetectable without a through-dispatcher integration test.

**Cites:** D-638; factory-dispatcher exec_subprocess env_clear; hooks-registry.toml env_allow; BC-1.15.001 PC6 (release requirement); S-18.00 LOCAL adversary pass-10 finding; S-18.04a (precompact-flush.sh, wave 2); S-18.05 (postcompact-reanchor.sh, wave 3).

---

## L-S18-stub-comment-sweep-discipline

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-638, S-18.00, stub-architect, TD-VSDD-059, adversary passes 9/10/11 (S-18.00 LOCAL cascade)
**Closes:** See forward-story: S-18.08 (grep-gate scope)

**Lesson (codified):** The stub-architect agent seeds stale "stub / todo!() / Red-Gate" comments that, post-implementation, falsely describe delivered code as pending. This pattern recurred across 3 consecutive adversary passes (9/10/11) in the S-18.00 LOCAL cascade because the sweeps were non-exhaustive — each pass found a different stale comment in a file not covered by the previous sweep. Codified gate: before declaring a comment-sweep done, the implementer (or the LOCAL adversary at pass-N) MUST execute the following grep and fix every hit:
```bash
grep -niE "stub|todo!?\(\)|does not exist|MUST add|FAIL TO COMPILE|RED until|not yet implemented|Red Gate" \
  $(git diff --name-only HEAD~1 HEAD)
```
Zero hits is the gate-pass criterion. Running the grep on only the primary implementation file is NOT sufficient — every file touched in the burst (including test files, SKILL.md cross-references, and doc comments) must be swept. Any hit that is NOT an intentional pending-work marker (e.g., a test fixture named `stub-test`) must be removed or replaced with accurate present-tense documentation.

**Root cause (S-18.00 passes 9/10/11):** Pass-9 found stale "// stub: will be implemented in S-18.04" in `invoke.rs` event arm comments. Pass-10 found stale "# RED GATE: this check requires implementation" in a bats test comment. Pass-11 found stale "// MUST add env_allow once implementation confirmed" in hooks-registry.toml inline comment. All three were seeded by the stub-architect. The implementer swept the primary `.rs` files but not the bats tests or TOML config comments.

**Gate (codified):** Applies at every TDD story immediately before LOCAL adversary pass-1 dispatch. The implementer MUST run the sweep and confirm zero hits (or justify each remaining hit with explicit "intentional pending-work marker — tracked in <story-ID>"). The LOCAL adversary independently reruns the grep at pass-1 as a structural gate check (TD-VSDD-059 paper-fix detection extension).

**Disposition:** Forward-story S-18.08 (grep-gate scope; automated validator gate candidate — `validate-stub-comment-absence.wasm` or bats-hook class). Anchor: E-18 F4. Self-improvement epic.

**Consequence if violated:** Stale stub/todo!/Red-Gate comments cause fresh-context adversaries to believe unimplemented code is still pending, triggering false BLOCKER findings that reset the 3-CLEAN streak. Each recurrence costs a full adversary pass + fix burst. Over a 3-pass window, this can extend a story's LOCAL cascade by 2-4 additional bursts.

**Cites:** D-638; TD-VSDD-059 (paper-fix detection); stub-architect role; S-18.00 LOCAL adversary passes 9/10/11; BC-5.39.001 (3-CLEAN convergence); S-18.08 gate-story candidate.

---

## L-S18-fixture-fidelity-must-mirror-production-multi-table-format

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-639, S-18.01, F-S1801-P5-001, write-wave-state.sh, STORY-INDEX multi-table, BC-5.39.001
**Closes:** F-S1801-P5-001 root cause diagnosis (fix in implementer dispatch on resume)

**Lesson (codified):** Bats fixtures for skills that parse index files (STORY-INDEX, BC-INDEX, ARCH-INDEX) MUST mirror the production file format exactly — including multi-table structure when the production file contains multiple epic tables. A fixture that contains only a single well-formed table will pass all topo-sort tests even when the production parsing logic is broken for multi-table files. The STORY-INDEX used in E-18 F4 tests had a single E-18 table; production STORY-INDEX has both E-0 (line 217, 7-col `Depends On` with space) and E-18 (line 655, 9-col `Depends-On` with hyphen) tables. The `grep -m1 '| Story ID'` pattern stops at E-0's header, extracting the wrong column index for dependency extraction — the bug is invisible with a single-table fixture.

**Root cause (F-S1801-P5-001):** `write-wave-state.sh` topo-sort uses `grep -m1 '| Story ID'` to find the dependency column header. With STORY-INDEX's two-epic structure, `-m1` stops at the E-0 table (first match, wrong table for E-18 stories). The E-0 table has a 7-column schema with `Depends On` (space) at column 7; the E-18 table has a 9-column schema with `Depends-On` (hyphen) at column 6. The fixture used in passes 1-4 contained only the E-18 table, so the parser found the correct header and appeared to work.

**Gate (codified):** For any skill that parses STORY-INDEX (or any multi-table index), the bats fixture MUST include:
1. At minimum 2 epic tables, with the target epic's table NOT first.
2. Both column-header spelling variants (`Depends On` and `Depends-On`) present across the fixtures.
3. A story ID collision check to ensure the parser finds the correct table (not the first table with a matching header).

**Disposition:** Implementer fix on resume: locate correct epic table by matching in-wave story IDs; normalize `Depends.On` header to handle both spellings. Forward gate story: S-18.08 (automated fixture-fidelity validator class).

**Consequence if violated:** A skill that parses a multi-table index with single-table fixtures will appear to work in all bats tests but fail silently in production when the real index contains multiple epics. The failure is non-obvious: the topo-sort produces empty or wrong dependency lists, causing wave-state.yaml to omit real dependencies (or include phantom ones from the wrong table's dependency column).

**Cites:** D-639; S-18.01 LOCAL adversary pass-5 finding F-S1801-P5-001; write-wave-state.sh topo-sort; STORY-INDEX E-0 table (line ~217) vs E-18 table (line ~655); BC-5.39.001 (3-CLEAN convergence).

---

## L-S18-gamed-guard-env-hatch

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-639, S-18.01, F-S1801-P4-001, write-wave-state.sh, DRY_RUN guard, bats fixture

**Lesson (codified):** A guard environment variable intended to make production code skip a destructive operation (e.g., `DRY_RUN=1` skipping `git commit`) creates a bats test env-hatch: every bats test that sets `DRY_RUN=1` effectively gates the test on a path that production never takes. If the guard is set unconditionally in the test harness, the test becomes a dead-letter gate — it exercises the pre-commit validation logic but never tests the actual commit path. This is a variant of the gamed-Red-Gate pattern (L-S18-gamed-red-gate, D-638).

**Root cause (F-S1801-P4-001):** `write-wave-state.sh` had a `DRY_RUN=1` env var that skipped the `git commit` step. The bats test suite set `DRY_RUN=1` globally, so all tests exercised only the pre-commit path. The BC-5.41.001/BC-5.41.002 postconditions require the atomic commit to actually happen; the tests asserted on validation output but never verified that a commit was created on factory-artifacts.

**Gate (codified):** For any skill that performs a git operation (commit, push, tag), the test suite MUST include at least one test that verifies the git operation actually executed (e.g., `git -C $ARTIFACTS_WT log -1 --format='%s'` matches the expected commit message). DRY_RUN guards are permitted for non-commit validation tests but MUST NOT be set for the final integration test that covers the commit path.

**Disposition:** Forward-story S-18.08 (guard-env-hatch detection gate; grep for `DRY_RUN` unconditional set in test harness preamble). Anchor: E-18 F4.

**Consequence if violated:** A skill with a global DRY_RUN guard in its test suite will pass 3-CLEAN convergence while its production commit path has never been executed. The defect surfaces only when the skill runs in a real wave-close context and the atomic commit fails or produces wrong output.

**Cites:** D-639; S-18.01 LOCAL adversary pass-4 finding F-S1801-P4-001; write-wave-state.sh DRY_RUN guard; BC-5.41.001 PC7 (atomic commit required); L-S18-gamed-red-gate (D-638 parent class).

---

## L-S18-weak-assertion-header-vs-body

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-639, S-18.01, write-wave-state.sh, STORY-INDEX topo-sort, bats assertions, POLICY 11

**Lesson (codified):** A test that asserts only on a header or preamble line (e.g., `grep "^wave_id:" output.yaml`) without asserting on dependent body fields (e.g., `stories:` list contents) will pass even when the body is empty or incorrectly populated. This creates a class of weak assertions where the contract's most important postcondition (that the wave contains the correct stories in topological order) is not verified. POLICY 11 (tests must assert observable output) requires that assertions cover the full behavioral postcondition, not just the structural preamble.

**Root cause (S-18.01 passes 1-3):** The initial test suite for `write-wave-state.sh` asserted `grep "^wave_id: 2" wave-state.yaml` and `grep "^epic_id: E-18" wave-state.yaml` but did not assert on the `stories:` field contents or the topological ordering of stories. Tests passed even when the topo-sort produced an empty list (because no stories were extracted from the wrong table header).

**Gate (codified):** For any skill that produces a structured output file (YAML, JSON, TOML), the test suite MUST assert on:
1. The PRESENCE and correct VALUE of every mandatory field (not just header fields).
2. The CONTENTS of list fields (e.g., stories: contains exactly the expected story IDs).
3. ORDER-SENSITIVE assertions where the contract specifies ordering (e.g., topological order: no story appears before its dependency).

Run assertions as: `yq '.stories | length' output.yaml` → expected count; `yq '.stories[0]' output.yaml` → expected first story.

**Disposition:** Forward-story S-18.08 (assertion-completeness validator gate; grep for YAML field assertion coverage). Anchor: E-18 F4.

**Consequence if violated:** A skill that produces structurally-valid but semantically-empty output will pass all preamble-only assertions. The downstream consumer (the orchestrator reading wave-state.yaml to determine which stories to dispatch) will receive an empty story list and dispatch nothing — a silent wave skip with no error signal.

**Cites:** D-639; S-18.01 LOCAL adversary passes 1-3; write-wave-state.sh stories-field assertions; BC-5.41.002 PC3 (wave-state.yaml stories field); POLICY 11 (observable output assertions required); F-S1801-P5-001 (downstream symptom of this root cause).

---

## L-S18-harness-env-override-masks-production-default

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-640, S-18.01, write-wave-state.sh, PRECOMPACT_FLUSH_LOG, bats harness, BC-5.41.001 PC7, F-P6-005

**Lesson (codified):** A bats test harness that unconditionally exports an environment variable (e.g., `export PRECOMPACT_FLUSH_LOG=/tmp/test.log`) will mask the production default for that variable in ALL tests in the file, including tests whose purpose is to verify the production-default-invocation path. This creates a class of false passes where the skill under test never exercises its own default-parameter resolution logic.

**Root cause (S-18.01 pass-6):** The write-wave-state.sh test harness unconditionally set `PRECOMPACT_FLUSH_LOG` at file scope, masking the broken production default (a hardcoded `.factory/.factory/precompact-flush-log` double-nesting path per ADR-027). All tests passed even though the default path was wrong, because the harness always supplied the correct path via env override.

**Gate (codified):** For any skill that reads a configuration value from an environment variable with a production default: (1) At least one test MUST invoke the skill WITHOUT setting the environment variable, and (2) That test MUST verify the skill resolves the default path correctly. The test name SHOULD be `test_<skill>_production_default_<field>`. Harness preamble exports for test isolation are acceptable ONLY if every export is accompanied by a corresponding no-export test for the same variable.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Forward-story S-18.08 (harness-env-override detection gate; grep for unconditional `export <ENV_VAR>` at harness preamble without corresponding no-export test). Anchor: E-18 F4.

**Consequence if violated:** A skill with a broken production default path will pass all tests that rely on harness env injection. The defect surfaces only when the skill runs outside the test harness (in production or manual invocation) and the env var is absent.

**Cites:** D-640; S-18.01 LOCAL adversary pass-6 finding F-P6-005; write-wave-state.sh PRECOMPACT_FLUSH_LOG default path; ADR-027 §ARTIFACTS_WT discipline; BC-5.41.001 PC7 (atomic commit path); F-S1801-P6-005.

---

## L-S18-test-comment-vs-impl-drift

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-640, S-18.01, write-wave-state.sh, test rationale comments, O-1 observation

**Lesson (codified):** Test rationale comments that describe WHY a test is structured a certain way (e.g., "# This test verifies that the DRY_RUN guard prevents actual git commits") become stale when the implementation changes and that mechanism is removed or replaced. Stale rationale comments describe behavior the code no longer exhibits, misleading future readers and creating false impressions during adversarial review.

**Root cause (S-18.01 pass-7 O-1):** After the DRY_RUN guard was removed and replaced with proper bats process substitution, rationale comments in 3+ test blocks still referenced the DRY_RUN mechanism. The adversary noted these as stale references (O-1 process-gap) that could mislead fresh-context reviewers into believing a removed control path was still present.

**Gate (codified):** When any implementation change removes or replaces a mechanism that test comments describe, fix-burst MUST include a grep sweep for ALL comments referencing the removed mechanism across the entire test file. Run: `grep -n "DRY_RUN\|<removed-mechanism>" <test-file>` and refresh every stale comment in the same commit. The adversary must not encounter rationale comments describing non-existent implementation paths.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Forward-story S-18.08 (stale-comment detection gate; grep for comments referencing removed mechanism tokens after implementation fix). Anchor: E-18 F4.

**Consequence if violated:** Future adversaries reviewing the test file will encounter comments describing removed implementation paths, potentially wasting investigation time on a mechanism that no longer exists, or (worse) concluding that the removed mechanism is a security concern that must be explicitly re-added.

**Cites:** D-640; S-18.01 LOCAL adversary pass-7 observation O-1; write-wave-state.sh DRY_RUN comment sweep; TD-VSDD-060 sibling-site sweep discipline (applies to test comments as well as production code).

---

## L-S18-spec-prose-must-not-imply-unintended-filter

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-640, S-18.01, BC-5.41.002 PC2, generated_from_handoff_sha, F-P7-001 LOW

**Lesson (codified):** BC postcondition prose that describes a field's value using a relative description (e.g., "the most recent HANDOFF.md commit") will be misread by fresh-context reviewers as implying a filtering operation (e.g., `git log --grep="HANDOFF"`) that the implementation does NOT perform. When the operative definition is a simple `git rev-parse HEAD`, the spec prose MUST state that directly rather than characterizing WHAT the HEAD commit typically contains.

**Root cause (S-18.01 pass-7):** BC-5.41.002 PC2 originally described `generated_from_handoff_sha` as "the most recent HANDOFF.md commit that already exists on factory-artifacts BEFORE the current wave-close atomic commit." The phrase "most recent HANDOFF.md commit" reads as a filtered `git log` search for commits containing HANDOFF.md, not as a plain `git rev-parse HEAD`. A fresh-context adversary (F-P7-001) correctly identified this as a potential misread: if interleaved commits (e.g., a BC fix commit, a STATE.md commit) land on factory-artifacts between wave closes, HEAD would point to those commits, not the last HANDOFF.md commit — making the spec behavior diverge from the implementation if the spec were interpreted literally as a filtered search.

**Gate (codified):** When authoring BC postcondition prose for a field populated by a git command: (1) State the EXACT command used (e.g., "`git -C <ARTIFACTS_WT> rev-parse HEAD`") rather than describing the typical commit content. (2) If the field "happens to be" the HANDOFF.md commit in normal operation but is NOT filtered to be so, state this explicitly: "normally this is the prior HANDOFF.md commit, but it is NOT filtered by commit message — any interleaved commits contribute to HEAD." (3) Disambiguate operational reality from filtering intent.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Forward: S-18.08 (spec-prose precision gate — check BC postcondition prose for implicit filter language). Anchor: E-18 F4.

**Consequence if violated:** Implementers reading the BC may implement a filtered `git log` search instead of plain `git rev-parse HEAD`, causing the implementation to break when factory-artifacts contains interleaved non-HANDOFF commits between wave closes (a normal operational scenario).

**Cites:** D-640; S-18.01 LOCAL adversary pass-7 finding F-P7-001 LOW; BC-5.41.002 PC2 v1.13→v1.14 clarity refinement; AC-014 generated_from_handoff_sha semantics; ADR-026 §Decision 4.

---

## L-S18-validate-before-write

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-641, S-18.01, BC-5.41.001 PC4, F-P8-001 BLOCKER

**Lesson (codified):** Anti-fabrication / precondition validation MUST run before any artifact write to honor "no partial output" contracts (BC-5.41.001 PC4). Calling `write_handoff` before the cross-check validation allows partial file creation that leaves the worktree dirty on failure — a partial output forbidden by PC4.

**Root cause (S-18.01 pass-8):** F-P8-001 BLOCKER: `write_handoff` was called first, then anti-fabrication checks ran. If the check failed, HANDOFF.md existed on disk in a partially-written state. Fix: restructure to run ALL pre-flight validation (anti-fabrication, field checks) before any file write; write HANDOFF.md only after all checks pass atomically.

**Gate (codified):** For any shell skill with a "no partial output" PC: (1) ALL validation runs BEFORE any `echo`/`printf`/`tee` writes to artifact files. (2) File writes are the LAST step in the happy path, not intermediate steps interspersed with checks. (3) If validation fails mid-sequence, NO artifact file should exist (clean failure).

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Forward: S-18.08 (pre-flight-validation-before-write gate as a WASM hook precondition check). Anchor: E-18 F4.

**Consequence if violated:** Failed wave-gate runs leave partial HANDOFF.md artifacts on factory-artifacts worktree, requiring manual cleanup and potentially confusing subsequent wave-gate invocations that find a stale partial file.

**Cites:** D-641; S-18.01 LOCAL adversary pass-8 finding F-P8-001 BLOCKER; BC-5.41.001 PC4; SOUL.md #4.

---

## L-S18-absent-ground-truth-must-hard-block

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-641, S-18.01, SOUL.md #4, F-P8-002 MEDIUM

**Lesson (codified):** A missing source-of-truth (STORY-INDEX) MUST hard-block, not silent-skip. SOUL.md #4 ("no silent failures") requires that the absence of a required input file propagates as an explicit error (hard exit 2), never as a silent "best effort" default return.

**Root cause (S-18.01 pass-8):** F-P8-002 MEDIUM: When STORY-INDEX.md was absent from the factory-artifacts worktree, `derive_wave_id` silently returned `wave_id = 1` and `stories = []` as if it were wave 1. This masked the missing file and allowed the wave-gate to proceed with fabricated data — a SOUL.md #4 violation.

**Gate (codified):** For any shell skill that reads a source-of-truth index file (STORY-INDEX, BC-INDEX, VP-INDEX): (1) Check for file existence BEFORE attempting to parse. (2) If absent → emit `ERROR: <filename> not found at <path>` and exit 2. (3) NO fallback to default values. (4) Error message MUST name the file and path so the operator can diagnose.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Forward: S-18.08 (absent-index hard-block gate as WASM hook pattern). Anchor: E-18 F4.

**Consequence if violated:** Silent `Vec::new()` / default returns when a required file is missing allow fabricated "success" that propagates incorrect wave-id derivations into downstream commits, corrupting wave-state.yaml with phantom wave ordinals.

**Cites:** D-641; S-18.01 LOCAL adversary pass-8 finding F-P8-002 MEDIUM; SOUL.md #4; BC-5.41.001 PC3 anti-fabrication.

---

## L-S18-implicit-invariant-needs-explicit-precondition-and-guard

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-641, S-18.01, BC-5.41.001 PC2, O-P8-001, P-WAVE-BARRIER-INVARIANT, P-SPRINT-STATE-WAVE-ORDER

**Lesson (codified):** Models relying on implicit invariants (wave-gate barrier, file-order == wave-order) MUST document explicit named preconditions AND add a fail-loud guard, rather than silently returning a wrong result when the invariant breaks. Research-agent validation is the recommended disposition path for "is this brittle?" observations.

**Root cause (S-18.01 pass-8):** O-P8-001: The `derive_wave_id` leading-contiguous-terminal-run algorithm was sound under VSDD's wave-gate barrier invariant but would silently return the wrong wave ordinal if story rows were manually reordered in STORY-INDEX (violating the file-order assumption). The algorithm was correct-under-invariants but the invariants were undocumented and there was no guard to detect their violation.

**Gate (codified):** (1) When an algorithm's correctness depends on an invariant maintained by external mechanisms (hook gates, human discipline), NAME the invariant explicitly as a precondition in the BC prose (e.g., P-WAVE-BARRIER-INVARIANT, P-SPRINT-STATE-WAVE-ORDER). (2) Add a fail-loud guard at the call site that exits non-zero with `WaveOrderUnverifiable` (or equivalent) if the invariant cannot be confirmed. (3) Document "Kahn-DAG-level derivation" or similar as a design evolution boundary — NOT as deferred work — when the full invariant-free solution is known but not required under current constraints. (4) Use research-agent validation to confirm "context-dependent" observations before committing to full redesign vs. documented boundary.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** The design boundary is documented in BC-5.41.001 v1.20 PC2 under P-WAVE-BARRIER-INVARIANT and P-SPRINT-STATE-WAVE-ORDER. No TD entry required; no future-story anchor required (the Kahn-DAG evolution is optional, not mandatory). Anchor: E-18 F4 for the WaveOrderUnverifiable guard (implemented worktree commit 539e6dab).

**Consequence if violated:** Algorithms with undocumented preconditions fail silently in scenarios that violate those preconditions, producing wrong outputs that downstream consumers treat as correct. Fresh-context adversaries correctly flag this as a brittle anti-fabrication gap.

**Cites:** D-641; S-18.01 LOCAL adversary pass-8 observation O-P8-001; BC-5.41.001 PC2 v1.19→v1.20 preconditions; research-O-P8-001-wave-ordinal.md; worktree commit 539e6dab.

---

## L-S18-untested-branch-hid-silent-failure

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-642, S-18.01, O-P10-001, feature/S-18.01 commits 3f63eb92/ff1d054e

**Lesson (codified):** An untested code branch (even a LOW-priority observation target) can mask a silent-failure defect that is actually MEDIUM severity. O-P10-001 flagged the factory_lock held-lock parse path as having zero fixture coverage — this was classified LOW. When tests were added, they UNCOVERED a real silent-failure: the block-form `factory_lock` awk pattern used non-POSIX `\s` which BSD/macOS awk interprets as a literal `s`, causing `factory_lock_holder` to silently return null for ALL held locks. The classification "LOW observation" masked a MEDIUM silent-failure. Every parse branch (including optional or rarely-hit fields) requires a discriminating production-faithful fixture before it can be characterized as LOW.

**Root cause (S-18.01 pass-10):** O-P10-001: The held-lock parse branch for block-form `factory_lock:` YAML (multi-line key: value form) had zero test fixtures. The awk pattern used `\s` (non-POSIX) which silently fails on BSD awk (macOS), returning empty string instead of matching whitespace. All invocations of `write-handoff.sh` on macOS that run with a held factory lock were silently emitting `factory_lock_holder: null` in HANDOFF.md, violating BC-5.41.001 PC2 anti-fabrication contract.

**Gate (codified):** For every `parse_*` or `read_*` function that reads a field from YAML/structured text: (1) Add a fixture that provides a production-faithful populated example for EVERY code path including optional/rarely-hit fields. (2) Assert the parsed value equals the expected non-null output — not just that the command succeeds. (3) Use POSIX-compliant character class syntax in awk (`[[:space:]]`, `[[:alnum:]]`) not GNU/Perl extensions (`\s`, `\w`, `\d`) which BSD awk silently misinterprets as literal characters.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3 after pass-10 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Fix applied: `[[:space:]]` replacing `\s` in held-lock awk pattern (feature/S-18.01 commit ff1d054e). Tests added for held-lock parse branch. Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Shell skills parsing YAML fields with optional block-form structure silently emit null for populated fields on BSD/macOS awk, producing HANDOFF.md artifacts with fabricated null values that downstream consumers treat as correct, violating anti-fabrication contracts without any error signal.

**Cites:** D-642; S-18.01 LOCAL adversary pass-10 observation O-P10-001; BC-5.41.001 PC2 anti-fabrication; feature/S-18.01 commits 3f63eb92 (tests added) + ff1d054e (awk non-POSIX `\s` fix).

---

## L-S18-skill-doc-contract-must-match-entrypoint

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-642, S-18.01, F-P10-001 MEDIUM, feature/S-18.01 commits 3f63eb92/203cf262

**Lesson (codified):** SKILL.md invocation contract MUST document the actual required/optional arguments the entrypoint parses, including all `:?`-required (bash positional required) parameters. A "None/None" contract that contradicts the entrypoint is a spec-compliance defect. The invocation contract is part of the behavioral contract surface; a fresh-context adversary reading SKILL.md to understand the skill's interface CANNOT learn the real required args if SKILL.md omits them.

**Root cause (S-18.01 pass-10):** F-P10-001 MEDIUM: The SKILL.md §Usage / invocation contract section listed no required arguments, but the `write-handoff.sh` entrypoint parsed (and required via `:?`) 4 positional arguments: `ARTIFACTS_WT`, `SPRINT_STATE_YAML`, `STATE_MD`, and `BC_DIR`. A caller following SKILL.md would invoke the skill with no arguments and receive a bash `:?` error. This was a spec-compliance BLOCKER-class defect (entrypoint contract directly contradicted by SKILL.md).

**Gate (codified):** For every shell skill with a SKILL.md: (1) The §Usage / Invocation section MUST list all positional arguments with their names, types, required/optional status, and description. (2) The set of `:?`-required positional variables in the entrypoint script is the authoritative required-args list. (3) SKILL.md must be updated in the SAME commit as any entrypoint argument change — never let them diverge. (4) A fresh-context reader should be able to correctly invoke the skill from SKILL.md alone.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3 after pass-10 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Fix applied: SKILL.md §Usage updated with all 4 required CLI args (feature/S-18.01 commits 3f63eb92 + 203cf262). Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Callers following SKILL.md cannot successfully invoke the skill. CI integrations, orchestrator dispatches, and human operators all hit `:?` bash errors. The defect is invisible until the skill is actually invoked with the real arg set — which may not happen until integration testing.

**Cites:** D-642; S-18.01 LOCAL adversary pass-10 finding F-P10-001 MEDIUM; S-18.01 §Architecture Compliance Rules; feature/S-18.01 commits 3f63eb92 (arg documentation) + 203cf262 (SKILL.md contract correction).

---

## L-S18-field-name-semantics-need-explicit-spec-note

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-642, S-18.01, O-P10-002, BC-5.41.001 PC2 v1.21

**Lesson (codified):** A field name that implies a filter (e.g., `active_bcs` implying only BCs with `lifecycle_status: active`) over an existence-only implementation needs an explicit spec note to prevent a future wrong lifecycle filter from being introduced. The name "active" in a field is a semantic signal that engineers will interpret as requiring lifecycle filtering unless the spec explicitly contradicts this. The explicit note must be in the normative BC body (not just comments or test vectors) to govern implementer behavior.

**Root cause (S-18.01 pass-10):** O-P10-002: The `active_bcs` field in BC-5.41.001 PC2 listed `active_bcs` as a field name without specifying whether "active" meant lifecycle-filtered or existence-only. A fresh-context adversary correctly flagged that the name creates an ambiguity — an implementer could reasonably add `lifecycle_status: active` filtering, which would EXCLUDE draft and withdrawn BCs, producing a truncated HANDOFF.md `active_bcs` list that fails the anti-fabrication completeness check. The fix is a clarity note in PC2, not a behavioral change (the semantics were already existence-only in the implementation).

**Gate (codified):** When authoring a BC field whose name implies a semantic filter (status-typed prefix like "active", "draft", "pending", "approved"), MUST add an explicit inline note in the normative PC prose clarifying: (1) the actual filter semantics (existence-only vs. lifecycle-filtered), (2) the correct implementation method (e.g., `find -name '*.md'`), and (3) what a future lifecycle-filter would require (explicit postcondition amendment). This prevents a "naming implies behavior" trap for future implementers.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3 after pass-10 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** BC-5.41.001 v1.21 PC2 `active_bcs` field definition expanded with explicit existence-only semantics note (product-owner; O-P10-002 disposition A). Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Future implementers add `lifecycle_status: active` filtering to `active_bcs` population, silently excluding draft/withdrawn BCs from HANDOFF.md. This makes HANDOFF.md `active_bcs` a lifecycle-filtered subset rather than the full corpus, which either: (a) causes anti-fabrication false negatives (partial list passes existence check but is semantically incomplete), or (b) causes anti-fabrication false positives (filtered path not found because only non-active BCs have that path). Either failure mode is silent.

**Cites:** D-642; S-18.01 LOCAL adversary pass-10 observation O-P10-002; BC-5.41.001 PC2 v1.20→v1.21 `active_bcs` clarity note; product-owner disposition A.

---

## L-S18-sibling-sweep-must-cross-file-and-tool

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-643, S-18.01, F-P11-001 BLOCKER, TD-VSDD-060, feature/S-18.01 commits a25824b2/feea27d2

**Lesson (codified):** When applying a regex character-class portability fix (e.g., `\s`→`[[:space:]]`), the TD-VSDD-060 sibling-sweep discipline MUST extend across ALL files AND ALL shell tools (awk, grep, sed, etc.) that share the defect class — not just the file where the defect was originally found. A portability fix that only touches one file/tool while leaving the same pattern in sibling files/tools is an incomplete fix. The O-P10-001 awk fix in write-handoff.sh was correct but was not swept to grep -E callsites in parse-sprint-state.sh, leaving 4 callsites with `\s`/`\S` that silently misclassified wave state on macOS.

**Root cause (S-18.01 pass-11):** F-P11-001 BLOCKER: The pass-10 fix of `\s`→`[[:space:]]` in write-handoff.sh (awk context) was applied correctly but not swept to grep -E callsites in parse-sprint-state.sh. The resulting behavior on macOS BSD grep was a SOUL.md #4 silent-failure: has-next-wave stories were misclassified as EPIC-COMPLETE because the `\s` pattern matched a literal `s` character rather than whitespace.

**Gate (codified):** For any portability-class fix (`\s`/`\S`, `\t`/`\n` in POSIX tools, `sed -E` vs `sed -r`, `date -d` vs `date -r`, `find -regex`, `tail -n+N`): (1) Run a repo-wide grep for ALL occurrences of the non-POSIX pattern across ALL `.sh` files. (2) Fix every occurrence in the SAME commit. (3) Add or update a portability-guard bats test asserting no remaining uses of the forbidden pattern in project shell files. (4) This sweep is a BLOCKER gate — incomplete portability fixes are themselves BLOCKERs for subsequent passes.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3 after pass-11 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Fix applied: 4 POSIX replacements in parse-sprint-state.sh + portability-guard test + behavioral macOS-path test (feature/S-18.01 commits a25824b2 + feea27d2). Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Platform-specific silent misclassifications produce incorrect HANDOFF.md values (wrong wave state, incorrect EPIC-COMPLETE signaling) that pass all tests on Linux CI while corrupting wave state on macOS developer environments and macOS CI runners.

**Cites:** D-643; S-18.01 LOCAL adversary pass-11 finding F-P11-001 BLOCKER; TD-VSDD-060 sibling-site sweep; feature/S-18.01 commits a25824b2 (BSD-portability grep fixes) + feea27d2 (portability-guard test + behavioral test).

---

## L-S18-bsd-gnu-portability-needs-ci-leg

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-643, S-18.01, F-P11-002 MEDIUM [process-gap], feature/S-18.01 commit c99b8a1f

**Lesson (codified):** Shell skills that target BSD/macOS environments (via the factory-dispatcher on developer macOS machines or macOS CI runners) require a macOS CI leg to provide structural BSD-vs-GNU coverage. A Linux-only CI configuration is structurally incapable of surfacing BSD-vs-GNU divergences in grep `\s`, sed -E behavior, date -u formatting, find syntax, and tail -n+N. Without a macOS CI leg, BSD-incompatible patterns can pass all CI checks and ship to macOS environments where they silently fail.

**Root cause (S-18.01 pass-11):** F-P11-002 MEDIUM [process-gap]: The wave-handoff.bats suite ran only on ubuntu-latest. The `\s`/`\S` BSD incompatibility in parse-sprint-state.sh that caused F-P11-001 would have been caught by a macOS bats run. The Linux-only CI configuration was not a deliberate decision — it was a process omission (the macOS runner was not added when the ubuntu runner was configured).

**Gate (codified):** Any shell skill that: (1) runs on macOS developer environments, OR (2) is invoked by the factory-dispatcher which ships for darwin-arm64/darwin-x86_64, MUST have at minimum one macOS CI runner in its bats test job. The ubuntu and macos runners should be configured as a matrix. Platform-specific test failures are BLOCKER CI failures — the macOS job must be required in branch-protection status checks before merge.

**S-7.02 confirmation:** Process-gap lesson for the S-7.02 Cycle-Closing-Checklist. Cycle is NOT converging this burst (streak 0/3 after pass-11 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Fix applied: blocking `bats-wave-handoff-macos` CI job added to ci.yml (feature/S-18.01 commit c99b8a1f). ADVISORY: branch-protection required-status-checks contexts may need updating before PR merge — flag for PR review. wave-handoff suite 46/46 green on both runners. Anchor: E-18 F4 S-18.01.

**Consequence if violated:** BSD-incompatible shell patterns (grep `\s`, sed -E extensions, date -u, find -regex, tail -n+N) ship to production macOS environments undetected, producing silent behavior divergences that corrupt factory state (wave misclassification, incorrect HANDOFF.md fields, wrong git operations) only on macOS.

**Cites:** D-643; S-18.01 LOCAL adversary pass-11 finding F-P11-002 MEDIUM; feature/S-18.01 commit c99b8a1f (macOS CI job); pre-existing TD entry: bats-full-suite not in branch-protection required-status-checks.

---

## L-S18-adr-worked-example-must-match-decision

**Date:** 2026-06-18
**Tags:** [codified]
**Anchors:** D-643, ADR-027 v1.1, F-P11-003 MEDIUM

**Lesson (codified):** An ADR's worked example, §Notes annotations, and §Consequences description MUST NOT contradict the ADR's own normative Decisions section. An ADR that describes a forbidden pattern in its examples — even as "illustration" — will mislead future implementers into adopting the exact pattern the Decision prohibits. The normative decision is the single source of truth; examples must faithfully demonstrate the normative pattern, not an alternative that happens to work in some contexts.

**Root cause (S-18.01 pass-11):** F-P11-003 MEDIUM: ADR-027 Decision 1 normatively states that `${ARTIFACTS_WT}/.factory/...` double-nesting is FORBIDDEN. However, ADR-027 §Decision 3 contained a "Note" describing bats fixtures that placed files under `$ARTIFACTS_WT/.factory/...` for test isolation — precisely the double-nesting pattern Decision 1 prohibits. The §Consequences section also described this double-nesting approach. A fresh-context implementer writing bats tests for an E-18 shell skill would follow the §Decision 3 Note example and produce double-nested fixtures that trigger ADR-027 violations in production.

**Gate (codified):** When reviewing or authoring an ADR: (1) Every example, Note annotation, and §Consequences description MUST be checked against the normative Decision clauses in the SAME ADR. (2) Any example showing a path, command, or data structure MUST be consistent with the Decision that governs it. (3) A "Note for testing" or "bats fixture example" that contradicts the normative Decision is a MEDIUM-class finding — it corrupts the ADR's informational function for implementers. (4) When fixing an ADR contradiction, update both the Note AND the §Consequences section in the SAME amendment.

**S-7.02 confirmation:** Cycle is NOT converging this burst (streak 0/3 after pass-11 reset); S-7.02 confirmation deferred to eventual convergence (≥3-CLEAN streak).

**Disposition:** Fix applied: ADR-027 v1.0→v1.1 — §Decision 3 Note corrected to no-nesting fixture layout; §Consequences updated (architect). ARCH-INDEX v2.56→v2.57 (state-manager POLICY 14 parity). Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Implementers follow ADR examples in good faith and implement the forbidden pattern, creating double-nested path bugs (`${ARTIFACTS_WT}/.factory/...`) that silently resolve to non-existent paths in production environments, causing silent-failure contract violations (SOUL.md #4) that only surface when the skill is run against a real factory-artifacts worktree.

**Cites:** D-643; ADR-027 v1.0→v1.1 F-P11-003 MEDIUM; S-18.01 LOCAL adversary pass-11; ARCH-INDEX v2.56→v2.57.

---

## L-S18-bc-bump-must-sweep-dependent-story-body-cites

**Date:** 2026-06-18
**Tags:** [process-gap] [codified]
**Anchors:** D-644, O-P12-001, BC-5.41.001 v1.20→v1.21 (D-642), S-18.01 v1.8→v1.9, POLICY 5, POLICY 8

**Lesson (codified):** When a BC version bumps mid-burst (any BC version increment), the orchestrator MUST dispatch story-writer in the SAME burst to sweep ALL dependent story-body BC version cites. POLICY 5 (body-cells-cite-current) requires that every BC cite in a story body reflects the current BC version. POLICY 8 (propagation) requires that a BC version bump propagate atomically to all downstream references in the same burst. A BC bump that updates only BC-INDEX catalog rows and frontmatter but does NOT sweep citing story bodies is an incomplete POLICY 8 propagation.

**Root cause (S-18.01 pass-12 O-P12-001):** Burst D-642 bumped BC-5.41.001 v1.20→v1.21 (O-P10-002 PO disposition A: active_bcs semantics clarity note). The D-642 burst correctly updated BC-INDEX v3.11→v3.12 (catalog row annotation). However, D-642 did NOT dispatch story-writer to sweep S-18.01 story body cites. S-18.01 body continued to cite `BC-5.41.001 v1.20` after the BC was at v1.21. The pass-12 adversary caught this as O-P12-001 LOW spec-parity defect. Because the fix required a package edit (S-18.01 v1.8→v1.9), the 3-CLEAN streak was reset to 0/3 on the newly-edited package — a direct cycle-cost from the omitted same-burst sweep.

**Gate (mandatory from D-644):** After ANY BC version increment, before committing the fix burst: (1) `grep -rn "BC-N.NN.NNN" .factory/stories/` to enumerate all story files citing the bumped BC; (2) For each citing story, check that the body cite version matches the new BC version; (3) If stale, dispatch story-writer in the SAME burst to update the body cite; (4) Update the STORY-INDEX annotation for the affected story row in the same burst (POLICY 14 parity). This sweep must cover ALL story bodies, not just the primary story under development.

**S-7.02 confirmation (D-644):** Cycle is NOT converging this burst (streak reset 0/3 due to post-pass package edit). S-7.02 confirmation deferred to eventual convergence.

**Disposition:** Fix applied: S-18.01 v1.8→v1.9 (story-writer body BC-5.41.001 cite swept v1.20→v1.21); STORY-INDEX v4.18→v4.19 (state-manager POLICY 14 parity). Anchor: E-18 F4 S-18.01; BC version-bump discipline.

**Consequence if violated:** BC body-cite drift accumulates across cascades, surfacing as LOW spec-parity observations that require post-CLEAN package edits — resetting the 3-CLEAN streak and extending the convergence timeline by at least one pass per missed sweep. If multiple stories cite the bumped BC, each missed sweep adds another potential streak reset.

**Cites:** D-644; O-P12-001; BC-5.41.001 v1.20→v1.21 D-642; S-18.01 v1.8→v1.9; POLICY 5 body-cells-cite-current; POLICY 8 propagation; BC-5.39.001 3-CLEAN streak.

**2nd Occurrence (D-659; 2026-06-19; S-18.13 spec-cascade pass-8):** C-SP13-P8-001 MAJOR: S-18.13 body cited BC-5.41.002 v1.18 after the D-658 back-reference burst bumped BC-5.41.002 v1.18→v1.19. The D-658 burst correctly propagated the BC version bump to BC-INDEX Stories cells and §Traceability in the BC body, but did NOT dispatch story-writer to sweep S-18.13 body forward-facing cites (6 occurrences: §Spec Amendments item 2 heading, §Scope ×2, §Behavioral Contracts table, AC-005 ×2, Token Budget, File Structure Requirements). Result: consistency-validator INCONSISTENT → MAJOR finding → streak RESET 1/3→0/3 → pass-8 NOT-CLEAN. Fix: S-18.13 v1.6→v1.7 (story-writer: C-SP13-P8-001 stale-cite sweep v1.18→v1.19 across all 6 load-bearing forward-facing cites; historical changelog refs left intact). Note: v1.18→v1.19 was a non-behavioral back-reference closure (PC6 substance unchanged per D-658). The recurrence confirms the gate obligation is independent of whether the BC version bump is behavioral — ANY version bump requires same-burst story-body sweep. **Orchestrator MUST dispatch story-writer to sweep dependent story body cites in the SAME burst whenever a BC version is bumped, even a non-behavioral back-reference bump.** S-7.02: 2nd occurrence — recurrence note appended (per S-7.02 discipline: 3rd+ occurrence triggers follow-up automation story; 2nd occurrence = recurrence note only). Cites: D-659; C-SP13-P8-001; BC-5.41.002 v1.18→v1.19 D-658; S-18.13 v1.6→v1.7; streak RESET 1/3→0/3; pass-9 NEXT.

---

## L-S18-cascade-converged-after-15-passes-deep-adversary-surfaces-cross-platform-silent-failure-class

**Date:** 2026-06-18
**Tags:** [codified] [convergence]
**Anchors:** D-645, BC-5.39.001, E-18 F4 Wave 2 S-18.01, passes 1–15

**Lesson (codified):** A deep LOCAL adversary cascade (15 passes) on a shell skill surfaces defect classes that shallow review misses. The S-18.01 cascade found real BLOCKERs through pass-11 — specifically a cross-platform silent-failure class (BSD-incompatible `\s`/`\S` in `grep -E`, silently misclassifying EPIC-COMPLETE status on macOS) — before decaying to three consecutive CLEAN passes. This trajectory is characteristic of shell skills with platform-sensitive behavior: initial passes surface spec inconsistencies and design ambiguities; mid-cascade passes surface implementation gaps and silent-failure paths; late passes surface only low-severity documentary observations that are non-load-bearing under TD-VSDD-091.

**Root cause (cascade arc):** The cascade found 11 NOT-CLEAN passes before reaching CLEAN. Key defect classes discovered in order: (1) spec/architectural ambiguity (ADR-027 path discipline, BC-5.41.001/002 PC clarifications — passes 1–5); (2) implementation correctness gaps (unanchored grep, DRY_RUN bypass, topo-sort multi-epic, EC-015 idempotency — passes 6–7); (3) robustness/error-propagation gaps (PRE-FLIGHT validation ordering, StoryIndexMissing hard-error, O-P8-001 precondition documentation — pass 8); (4) cross-platform silent-failure (BSD grep `\s`/`\S`, macOS CI leg missing, SKILL.md contract gaps — passes 10–11); (5) body-cite parity (POLICY 5/8 BC version cite propagation — pass 12 post-CLEAN cleanup). Passes 13–15 were CLEAN with only low-severity observations that either (a) do not require package edits (TD-VSDD-091 documentary) or (b) require cross-story PO reconciliation (EC-002 schema tension).

**Gate (codified):** For shell skills with cross-platform behavioral requirements (darwin-arm64 + linux-x86_64), the adversary cascade MUST include explicit BSD-portability probing across all callsites — not just the callsite fixed most recently. The L-S18-sibling-sweep-must-cross-file-and-tool lesson (D-643) addresses this class: when a portability fix is applied to one callsite, all sibling callsites in all files must be swept before declaring the class closed. Failure to sweep siblings produces a pass-11-class BLOCKER on the first fresh-context pass after the partial fix.

**S-7.02 confirmation (D-645):** BC-5.39.001 3-CLEAN streak 3/3 CONVERGED. All 16 process-gap lessons from this cascade accounted for: 13 fixed-in-scope, 3 codified. S-7.02 checklist SATISFIED. See D-645 Appendix for full per-lesson disposition table.

**Disposition:** Convergence recorded. Post-convergence posture: deferral-cleanup burst (O-P13-001/O-P14-001/O-P15-001/O-P9-001 anchor) + confirming fresh-context pass + human gate before demo/PR/merge. Anchor: E-18 F4 S-18.01.

**Consequence if violated:** Attempting to converge a shell skill with cross-platform requirements on fewer than ~8–12 adversary passes is unlikely to surface the silent-failure / portability class — those defects require a fresh-context adversary who probes BSD semantics explicitly. Shallow cascades (< 5 passes) on complex shell skills will systematically miss this class, producing production failures on macOS/BSD environments.

**Cites:** D-645; BC-5.39.001 3-CLEAN streak 3/3 CONVERGED; feature/S-18.01 @ c99b8a1f; BC-5.41.001 v1.21; BC-5.41.002 v1.14; ADR-027 v1.1; S-18.01 v1.9; L-S18-sibling-sweep-must-cross-file-and-tool (D-643); passes 1–15 arc.

---

## L-S18-macos-ci-leg-caught-runtime-portability-the-static-lint-missed

**Date:** 2026-06-19
**Tags:** [process-gap] [portability] [ci]
**Anchors:** D-648, PR #193, E-18 F4 Wave 2 S-18.01, +4 CI portability fixes

**Lesson (codified):** The LOCAL adversary cascade's static portability-lint guard (L-S18-sibling-sweep-must-cross-file-and-tool, D-643) correctly identified and fixed BSD-incompatible `\s`/`\S` in grep/awk callsites across all files. However, the cascade's static analysis scope did NOT cover runtime portability issues that only manifest at execution: (1) `local -A` associative arrays require bash 4+ (bash 3.2 is the system default on macOS); (2) global `IFS=` mutations in shell functions are flagged by SAST tools (ifs-tampering) and can produce unexpected cross-platform behavior; (3) Python runtime dependencies (PyYAML) must be declared and installed in CI with PEP 668 compatibility (`--break-system-packages` or virtual env). The macOS CI leg (added at pass-11 as fix F-P11-002) subsequently caught ALL FOUR of these issues during the PR CI gate — AFTER the LOCAL cascade had converged at 3-CLEAN.

**Root cause:** Static lint coverage (grep-class portability) and runtime execution coverage (bash-version-feature availability, IFS mutation, Python runtime deps) are distinct checks. A 15-pass LOCAL adversary cascade with static analysis is insufficient to surface runtime portability issues — only an actual execution on the target platform catches them. The macOS CI leg is therefore load-bearing, not redundant.

**Gate (codified):** The portability discipline for shell skills MUST extend beyond POSIX character-class regex checks to include:

1. **Bash-version-feature gating:** Any script using associative arrays (`declare -A`, `local -A`), `mapfile`/`readarray`, `[[ ... =~ ... ]]` with named groups, or other bash 4+ features MUST either (a) begin with `#!/usr/bin/env bash` + an explicit `if [[ "${BASH_VERSINFO[0]}" -lt 4 ]]; then echo ... exit 1; fi` guard, or (b) restrict to POSIX sh + POSIX-compatible tools. Absence of such a guard when bash 4+ features are used is a MEDIUM finding per the portability discipline.
2. **IFS-mutation avoidance:** `IFS=<delim>` assignments inside functions MUST be scoped with `local IFS=...` or replaced with `awk -F'<delim>'` / `IFS='<delim>' read -ra arr` one-liner patterns. Global IFS mutation that persists across function calls is a SAST flag (ifs-tampering) and a portability risk.
3. **Runtime-dependency declaration:** Python dependencies required by shell scripts (e.g., `python3 -c "import yaml"`) MUST be declared in CI configuration with a `pip install` step. On PEP 668 systems, `pip install --break-system-packages` or a virtual environment is required. Absence of declaration = silent CI failure on dependency-locked environments.

**S-7.02 note:** This lesson captures a class NOT representable by the static adversary cascade alone — it requires runtime execution evidence. The correct enforcement mechanism is the macOS CI leg itself. This lesson vindicates the F-P11-002 CI-leg fix as load-bearing infrastructure, not a cosmetic CI addition.

**Consequence if violated:** Shell skills that pass static portability lint and LOCAL adversary convergence can still fail at PR CI on macOS (and production on macOS operator environments) if runtime portability issues are not surfaced. Each missed class adds at least one CI-blocking fix iteration post-cascade. The 4-fix pattern in PR #193 (2b40dfd5 + ea7328ac + aaa8da8a + 3fe11ea1) illustrates the cost of not catching these classes earlier.

**Cites:** D-648; PR #193; 2b40dfd5 (IFS→awk); ea7328ac (bash3.2 local-A guard + brew bash); aaa8da8a/3fe11ea1 (PyYAML PEP 668); F-P11-002 macOS CI leg (D-643); L-S18-sibling-sweep-must-cross-file-and-tool (D-643); E-18 F4 S-18.01 local cascade passes 1–15 CONVERGED.

---

## L-SP13-consistency-validator-must-check-adr-own-body-changelog-row-not-only-arch-index-legs

**Date:** 2026-06-19
**Tags:** [process-gap] [policy-14] [consistency-validation]
**Anchors:** D-654, F-SP13-P1-001, S-18.13 spec-cascade LOCAL pass-1, POLICY 14

**Lesson (codified):** The consistency-validator's POLICY 14 ADR check protocol verifies upstream-index provenance legs (ARCH-INDEX catalog row claims) but does NOT independently open the ADR file's own body §Changelog table to verify that the top row matches the frontmatter version. This asymmetry allowed the consistency-validator to return CONSISTENT (9/9 PASS) on S-18.13 pass-1 while a real POLICY 14 body-propagation gap existed in ADR-026: the ADR frontmatter correctly showed `version: "1.22"` and the ARCH-INDEX catalog row stated "§Traceability ARCH-INDEX v2.57→v2.58 provenance leg appended," but the ADR body §Changelog table top row had not been updated to v1.22 and the §Traceability leg had not been written. The adversary (fresh-context, reading the ADR file directly) caught what the consistency-validator missed.

**Root cause:** The consistency-validator's CHECK 1 for POLICY 14 ADR parity reads the ARCH-INDEX catalog row to verify provenance legs, then trusts the ARCH-INDEX claim as ground truth. It does not cross-reference back to the ADR body itself to independently confirm the §Changelog and §Traceability sections. This is a trusting-the-index anti-pattern: the index row claimed "leg appended" (false), and the consistency-validator accepted the claim without verification against the primary artifact.

**Gate (codified):** The consistency-validator's POLICY 14 ADR version-bump check MUST include:

1. **ADR body §Changelog top row check:** Open the ADR file directly. Verify that the first data row in the §Changelog table has a Version column matching the frontmatter `version:` field. If the top row does not match the frontmatter version, this is a BLOCKER (POLICY 14 body-propagation gap — same class as F-SP13-P1-001).
2. **ADR body §Traceability ARCH-INDEX leg check:** Open the ADR file directly. Verify that the §Traceability `ARCH-INDEX:` line contains a provenance entry whose "from" version matches the prior ARCH-INDEX version and whose "to" version matches the current ARCH-INDEX version. If the ARCH-INDEX catalog row claims "leg appended" but the leg is absent in the ADR body, this is a BLOCKER.
3. **ARCH-INDEX cross-check (existing):** Verify that the ARCH-INDEX catalog row's `last_amended` description correctly describes the ADR change (no false claims). If the ARCH-INDEX catalog row description makes a claim about the ADR body (e.g., "§Traceability leg appended"), that claim must be verified against the ADR body, not accepted at face value.

These three checks together implement a bidirectional verification: index→ADR AND ADR→index.

**Disposition:** Justified deferral with concrete anchor. The fix requires modifying the consistency-validator skill's POLICY 14 ADR check protocol (adding the ADR body open + §Changelog row check + §Traceability leg presence check). This is a skill self-improvement task outside the scope of the S-18.13 cascade. Anchored to: consistency-validator skill self-improvement story (to be registered as a follow-up story in E-18 or the next pipeline improvement wave). The adversary cascade remains the correct fallback — fresh-context adversary reading primary artifacts directly catches this class. Until the consistency-validator is updated, the adversary pass remains load-bearing for POLICY 14 ADR body verification.

**Consequence if violated:** Consistency-validator returns CONSISTENT on POLICY 14 violations in ADR body §Changelog and §Traceability sections. The adversary (if dispatched fresh-context) will catch these gaps, but this adds one fix burst to every cascade where an ADR version bump occurs. In a multi-pass spec cascade, each unnecessary fix burst resets the 3-CLEAN streak (BC-5.39.001), extending convergence by at least one pass pair (adversary + fix). The S-18.13 pass-1 case demonstrated: one undetected ADR body gap = one NOT-CLEAN verdict = streak stays at 0/3.

**Cites:** D-654; F-SP13-P1-001; ADR-026 v1.22; BC-5.39.001 3-CLEAN streak; POLICY 14 quintuple parity; consistency-validator CHECK 1; S-18.13 spec-cascade LOCAL pass-1.

---

### L-SP13-gate-inert-on-bash-producer-class

**Tags:** [process-gap] [hooks-registry] [bash-producer] [write-tool] [gate-inertness]
**Anchors:** D-655, F-SP13-P2-observation, S-18.13 spec-cascade LOCAL pass-2, S-18.02, ADR-026 §Decision 8

**Lesson (codified):** A PostToolUse Write|Edit WASM gate registered in hooks-registry.toml is silently inert for any artifact produced via bash redirection. The PostToolUse event fires only when the Claude Code Write or Edit tool is invoked by the agent; bash redirection (`} > file`, `echo ... > file`, `cat > file`, `tee`) emits no PostToolUse event and therefore bypasses the gate entirely. This is the root cause of the original F-S1802-02 finding (S-18.02 gate was dead code in production because write-handoff.sh used bash redirection). S-18.13 fixes the specific HANDOFF.md instance. The general class remains ungated.

**Root cause:** ADR-026 §Decision 8 constrained the gate's hook registration (PostToolUse on Write|Edit) and defined what HANDOFF.md MUST contain (BC-5.41.001 PC10), but did NOT include a governance constraint requiring the HANDOFF.md *producer's* write path to use the Write tool. The gate and the producer were specified independently, creating an integration gap that only manifests at runtime when the PostToolUse event never fires.

**Gate class (codified):** The "gate-inert-on-bash-producer" class describes any situation where:
1. A hooks-registry.toml entry registers a WASM gate on `tool = "Write|Edit"` PostToolUse events for a specific file.
2. The skill or script that produces that file uses bash redirection (`>`, `>>`, `tee`, `cat >`) instead of the Claude Code Write tool.
3. Result: the gate is registered correctly, passes all unit tests (which may mock the PostToolUse event), but is silently never invoked on the production write path.

**General remedy:** A governance check (CI assertion or policy rule) that: for every PostToolUse gate entry in hooks-registry.toml with `tool = "Write|Edit"` targeting a specific artifact, the skill(s) responsible for producing that artifact must invoke the Write or Edit tool (not bash redirection) as their primary write path. This can be implemented as:
- A CI grep that scans skill files for bash-redirect patterns paired with artifact paths that are also named in hooks-registry.toml gate entries.
- OR a POLICY addition (e.g., POLICY N): "Artifact producers paired with PostToolUse gates MUST use the Write/Edit tool as the primary write mechanism. Bash redirection is FORBIDDEN as a primary write path for any artifact governed by a PostToolUse WASM gate."

**Disposition (S-7.02):** Justified deferral with concrete anchor. S-18.13 fixes the load-bearing HANDOFF.md instance. The governance check requires a CI/hook change outside S-18.13 scope. Anchored to: a new follow-up story to be registered as part of the E-18 post-cascade deferred-items burst (analogous to S-18.11/S-18.12 pattern — observed at cascade pass close, anchored in same deferred-items burst, story registered before cascade moves to F4 TDD). Until the governance check exists, the adversary cascade and D-655 process-gap lesson serve as the detection mechanism.

**Consequence if violated:** New skills that write artifacts to factory-artifacts via bash redirection, when paired with PostToolUse WASM gates, will have silently inert gates in production. The gate will appear to work in unit tests (if the tests mock PostToolUse events) but will never fire on the actual write path. The bug may not surface until an adversary pass specifically tests the end-to-end dispatch chain with the actual skill invocation.

**Cites:** D-655; F-SP13-P2-observation; F-S1802-02; S-18.13 spec-cascade LOCAL pass-2; S-18.02; ADR-026 §Decision 8; hooks-registry.toml PostToolUse gate semantics; BC-5.39.001 3-CLEAN streak.

---

## L-SP13-architect-must-read-actual-entrypoint-not-allowed-tools

**ID:** L-SP13-architect-must-read-actual-entrypoint-not-allowed-tools
**Class:** [process-gap]
**Source:** D-656, F-SP13-P3-001 CRITICAL, S-18.13 spec-cascade LOCAL pass-3
**Date:** 2026-06-19

**Anchors:** D-656, F-SP13-P3-001, D-655, S-18.13 spec-cascade LOCAL pass-3, ADR-026 §Decision 8

**Lesson (codified):** When an architect or any spec author determines "implementability" of a mechanism involving a specific script or skill entry point, they MUST read the actual entry point code — not infer from metadata annotations. SKILL.md `allowed-tools: [Write]` records what the agent is permitted to call; it does NOT mean the skill's bash script invokes the Write tool. The bash script `main "$@"` controls execution fully; the agent never gets to call Write mid-script.

**Root cause:** D-655 architect determination was WRONG. The architect inferred "agent-orchestrated" from SKILL.md `allowed-tools: [Write]` (metadata annotation) without reading `wave-handoff.sh`'s actual entry point `main "$@"`. The script is a fully bash-controlled monolith: `main` dispatches all logic internally with no agent seam. The `allowed-tools` annotation is a CAPABILITY declaration (the skill MAY use Write), not an IMPLEMENTATION guarantee (the skill DOES use Write at this code path).

**Rule:** Before declaring "this mechanism is implementable" or "the Write tool is called here," the architect MUST:
1. Read the actual entry point (the `main` function, the `run()` function, or the top-level dispatch logic).
2. Identify whether there is an agent seam (a point where bash returns control to the agent) or whether bash fully controls execution from invocation to exit.
3. If bash fully controls execution, the agent cannot interpose Write tool calls mid-execution. Any mechanism requiring the agent to call Write mid-script is NOT implementable without architectural restructure.

**Anti-pattern:** "SKILL.md says `allowed-tools: [Write]` therefore the agent uses Write to write HANDOFF.md." This is an inference from metadata, not a code read.

**Correct pattern:** Read `wave-handoff.sh`'s `main "$@"` → observe that `write_handoff()` is called from `main` via `process_wave()` → conclude: the Write tool call would need to be inserted at the bash function level, which is architecturally impossible without restructuring the call graph so the agent controls the write step.

**Consequence if violated:** Downstream spec (story, BC, ADR) will prescribe a mechanism that cannot be implemented. The adversary will find this as CRITICAL on the next fresh-context pass (as happened: D-655 architect determination → D-656 F-SP13-P3-001 CRITICAL). The fix is always a full restructure burst at higher cost than would have been required if the correct design was produced in the first instance.

**Cites:** D-656; D-655; F-SP13-P3-001 CRITICAL; S-18.13; ADR-026 §Decision 8; wave-handoff.sh `main "$@"`; SKILL.md `allowed-tools`.

---

## L-SP13-stale-local-develop-causes-false-positive-adversary-findings

**ID:** L-SP13-stale-local-develop-causes-false-positive-adversary-findings
**Class:** [process-gap]
**Source:** D-656, F-SP13-P3-003 FALSE POSITIVE, S-18.13 spec-cascade LOCAL pass-3
**Date:** 2026-06-19

**Anchors:** D-656, F-SP13-P3-003, S-18.13 spec-cascade LOCAL pass-3, origin/develop bd6e50ce, local develop 8b26a0fe

**Lesson (codified):** Before dispatching a code-reading adversary review pass (any LOCAL or PR-level spec-cascade pass that reads source code), the adversary MUST verify its local develop branch is current with origin/develop. Stale local develop causes false-positive findings about "missing" code that actually exists on origin but not on the stale local checkout.

**Root cause:** F-SP13-P3-003 MEDIUM ("S-18.02 gate crate validate-wave-handoff-completeness missing") was a FALSE POSITIVE. The adversary read local develop at 8b26a0fe (one commit behind origin). The gate crate `validate-wave-handoff-completeness` was introduced by S-18.02 PR #195 squash-merged to origin/develop at bd6e50ce. The adversary's local checkout did not include this commit, so the crate appeared absent.

**Rule:** Before reading any source file during an adversary or consistency-validator review:
1. Run `git -C <develop-worktree> fetch origin`.
2. Run `git -C <develop-worktree> log --oneline origin/develop -5` to confirm local HEAD matches origin/develop HEAD.
3. If local is behind: `git -C <develop-worktree> pull --ff-only`.
4. Only then read code from the worktree.

**Alternative (for agents without a mounted develop worktree):** Use `git show origin/develop:<path>` to read files directly from origin/develop without relying on local checkout currency.

**Active directive (from CLAUDE.md):** "Adversary MUST grep `origin/develop` or `factory-artifacts` for literal-shell evidence (NOT stale local main; per L-EDP1-067-CANDIDATE)." This extends to: adversary MUST also NOT read stale local develop. Origin-side reads are canonical.

**Consequence if violated:** False-positive findings consume a fix burst slot, write an erroneous D-NNN decision block, and waste one cascade pass count. In this case F-SP13-P3-003 added no code changes but added noise to the audit trail and required explicit false-positive disposition in D-656.

**Mitigation already active:** §3 User Directives carry "Adversary MUST grep `origin/develop`..." (from L-EDP1-067-CANDIDATE). This lesson adds the corollary: MUST NOT read stale local checkout for source code during adversary review.

**Cites:** D-656; F-SP13-P3-003 FALSE POSITIVE; S-18.13 spec-cascade LOCAL pass-3; origin/develop bd6e50ce; local develop 8b26a0fe; L-EDP1-067-CANDIDATE; §3 User Directives.

---

## L-SP13-bc-anchor-add-must-sweep-sibling-bc-stories-and-indexes (2026-06-19; D-658; [process-gap])

**Category:** POLICY 8 bidirectional traceability — back-reference propagation on BC anchor addition.

**Trigger:** S-18.13 spec-cascade LOCAL pass-6 NOT-CLEAN. F-SP13-P6-002 + F-SP13-P6-003 found that D-656 added BC-5.41.002 to S-18.13's `behavioral_contracts` array but did NOT: (a) update BC-5.41.002 body §Traceability Stories row to include S-18.13, (b) update BC-5.41.002 §Story Anchor to include S-18.13, (c) update BC-INDEX.md Stories cells for BC-5.41.002 (and BC-5.41.001, which shares the pattern) to include S-18.13, (d) update STORY-INDEX.md BC-coverage summary for BC-5.41.002 to include S-18.13. D-656 DID correctly sweep BC-5.41.001 sibling for the same gap but missed BC-5.41.002, which was the newly-added BC in that same burst.

**Rule:** When a story adds a BC to its `behavioral_contracts:` frontmatter array, the SAME burst MUST:
1. In the BC's body §Traceability table **Stories** row: add the story ID.
2. In the BC's body **§Story Anchor** section: add the story ID with parenthetical identifying the AC(s) that trace to this BC.
3. In **BC-INDEX.md** Stories column for that BC row: add the story ID.
4. In **BC-INDEX.md** Stories column for ALL SIBLING BCs that the story also exercises (i.e., BCs already in the `behavioral_contracts` array before this burst): verify Stories cell and add the story ID if not already present.
5. In **STORY-INDEX.md** BC-coverage summary: update the `BC-X.XX.XXX (stories)` entry for the newly-added BC to include the new story ID.

The failure mode is: the burst author sweeps the FIRST BC that was already in `behavioral_contracts` (BC-5.41.001 in this case) but misses the NEWLY-ADDED BC (BC-5.41.002) and all 3 index propagation sites.

**Corollary — D-656 vs D-658 comparison:** D-656 added BC-5.41.002 to S-18.13 `behavioral_contracts`. D-656 back-referenced S-18.13 in BC-5.41.001 (correct, since it was already in behavioral_contracts and had been swept), but did not back-reference in BC-5.41.002 (the newly-added BC — the one most needing the sweep). This is the REVERSE of the expected error: authors tend to sweep the OLD entries and miss the NEW one.

**S-7.02 defensive sweep discipline (post-burst gate):** After any burst that modifies a story's `behavioral_contracts:` array, run:
```bash
# For each BC in behavioral_contracts array:
grep -l "S-XX.XX" .factory/specs/behavioral-contracts/ss-*/BC-*.md   # verify Stories row
grep "BC-XX.XX.XXX" .factory/stories/STORY-INDEX.md | grep "BC coverage"  # verify BC-coverage
grep "BC-XX.XX.XXX.*Stories" .factory/specs/behavioral-contracts/BC-INDEX.md  # verify BC-INDEX
```
Any miss in these 3 grep checks is a POLICY 8 propagation gap.

**Closes:** F-SP13-P6-001 (BC-5.41.002 body back-ref); F-SP13-P6-002 (BC-INDEX Stories cells); F-SP13-P6-003 (STORY-INDEX BC-coverage summary).

**Cites:** D-658; F-SP13-P6-001; F-SP13-P6-002; F-SP13-P6-003; POLICY 8 v1.3 bidirectional traceability; S-18.13; BC-5.41.002 v1.19; BC-INDEX v3.22; STORY-INDEX v4.36.

---

## L-SP13-adr-cite-volatile-pin-drift-drop-version-token (2026-06-20; D-660; [process-gap])

**Category:** POLICY 19 anti-volatile-pin — forward-facing ADR §Decision cite version-token drift.

**Trigger:** S-18.13 spec-cascade LOCAL pass-10 NOT-CLEAN. F-SP13-P10-001 MEDIUM: S-18.13 v1.7 still carried ADR §Decision 8 forward-facing cites with version tokens `(v1.23)` / `(v1.24)` in multiple locations (§Originating Finding, §Scope S5 note, AC-004, T-1, T-2c, implementation-sequence, §File Structure Requirements SKILL.md MODIFY row). The D-660 v1.6 burst's cite sweep was incomplete — 5+ occurrences survived. The version tokens became stale as soon as ADR-026 bumped in D-657 (v1.23→v1.24); any future ADR bump repeats the drift.

**Rule:** Forward-facing ADR §Decision N cites in stories and BCs MUST use stable section anchor with NO version token. Example:

- **Correct (POLICY 19 option-b):** `ADR-026 §Decision 8`
- **Incorrect (drift-liable):** `ADR-026 §Decision 8 (v1.23)` or `ADR-026 §Decision 8 v1.24`

**Two cure options:**
- **(option-a) Update the token on each ADR bump:** High maintenance; guarantees recurrence on next ADR bump unless same-burst sweep is always exhaustive. Recurrence-liable.
- **(option-b) Drop the token entirely:** Zero-maintenance; the section anchor is stable; no recurrence possible. Recurrence-proof. **PREFERRED.**

**Exception:** Historical changelog entries that cite specific PAST versions (e.g., "BC-5.41.001 v1.23 PC10 was amended per ADR-026 v1.23 §Decision 8") remain valid and are explicitly exempt — those are immutable audit-trail entries pointing at a past state, not forward-facing design traces.

**Exhaustive sweep requirement:** When dropping tokens from S-18.13, the sweep must cover ALL occurrences across the full story body. A partial sweep (missing §Originating Finding, §Scope notes, individual AC/T/implementation-sequence paragraphs) leaves stale tokens that recur at the next adversary pass.

**Root cause confirmed via P10 (recurrence after P6 partial sweep):** D-656 attempted a cite sweep during the RESTRUCTURE REDESIGN but missed occurrences in non-normative/narrative locations. D-660 closed all remaining occurrences exhaustively via POLICY 19 option-b (token drop).

**S-7.02 disposition:** NO-ACTION obs O-SP13-adr-illustrative-stepcount (recurred P9/P11) is distinguished from this finding: the illustrative step-count block is non-normative; this finding (F-SP13-P10-001) addresses normative forward-facing cites. No overlap.

**Cites:** D-660; F-SP13-P10-001; POLICY 19 anti-volatile-pin; S-18.13 v1.7→v1.8; ADR-026 §Decision 8; D-656 (partial-sweep root cause); BC-5.39.001 3-CLEAN streak RESET.

---

## L-SP13-S-18.13-LOCAL-cascade-CONVERGED-13-passes (2026-06-20; D-661; [convergence])

**Category:** Convergence milestone — S-18.13 spec-evolution LOCAL spec-cascade BC-5.39.001 3-CLEAN.

**Trigger:** S-18.13 spec-cascade passes 11/12/13 all CLEAN/CONSISTENT. Streak 3/3 CONVERGED per BC-5.39.001. S-7.02 cycle-close checklist SATISFIED.

**Arc summary (13 passes; 10 NOT-CLEAN fix bursts before convergence window):**
- Passes 1-3: structural/foundational fixes (POLICY 14 gap → mechanism-precision → CRITICAL RESTRUCTURE REDESIGN)
- Pass 4: CLEAN (1/3) — first substantive convergence signal
- Pass 5: CRITICAL relapse (EPIC-COMPLETE carve-out missed in REDESIGN)
- Pass 6: POLICY 8 back-reference propagation gaps (newly-added BC not swept)
- Pass 7: CLEAN (1/3) — second substantive signal
- Pass 8: MAJOR consistency gap (stale BC cite due to P6 BC bump without story sweep)
- Pass 9: CLEAN (1/3)
- Pass 10: MEDIUM + MAJOR cite-drift (ADR volatile-pin tokens; BC §Story Anchor asymmetry)
- Passes 11-13: THREE CONSECUTIVE CLEAN/CONSISTENT — CONVERGED

**Defect-class trajectory:** P1-P3 structural/foundational → P4 CLEAN → P5 CRITICAL (carve-out) → P6 POLICY 8 propagation → P7 CLEAN → P8 stale-cite class → P9 CLEAN → P10 cite-drift class → P11/P12/P13 CLEAN CONVERGED.

**Key process-gap observations:** The cascade required 13 passes (vs S-18.01's 15-pass cascade) because the REDESIGN at P3 eliminated the implementability blocker early, but introduced new propagation obligations (POLICY 8 back-references, EPIC-COMPLETE carve-out) that required 3 additional fix bursts (P5/P6/P8). The converging 3-CLEAN window was only achievable after BOTH the cite-drift class (volatile-pin tokens) AND the consistency asymmetry (BC §Story Anchor) were fully closed in P10.

**S-7.02 SATISFIED:** 6 process-gap lessons codified across the cascade arc. 4 LOW observations dispositioned (1 RESOLVE-AT-TDD, 3 NO-ACTION).

**4-index at convergence:** BC-INDEX v3.23 / VP-INDEX v2.40 / STORY-INDEX v4.38 / ARCH-INDEX v2.60. ALL UNCHANGED this burst (pure cascade-state recording).

**Cites:** D-661; D-653..D-660 (full cascade arc); BC-5.39.001 3-CLEAN protocol; S-7.02 cycle-closing checklist; S-18.13 v1.8 READY.

---

## L-BB-premature-ci-green-attestation

**Date:** 2026-06-23
**Tags:** [process-gap] [ci] [attestation]
**Anchors:** D-691, D-692, PR #201, S-18.14

**Lesson (codified):** A 'CI green' attestation recorded in a PAUSE checkpoint (D-691 recorded 'CI 12/12 GREEN') MUST require ALL required matrix legs to be in a TERMINAL state (completed=success) at the time of attestation — not merely the legs that had completed at the snapshot moment. During D-691 the build-dispatcher (windows-x64) job was still building when the pause was recorded; it later FAILED. On resume, two further CI failures surfaced before final merge.

**Root cause:** The pause attestation was formed from a poll of CI status at a point-in-time when some required jobs were still `in_progress`. The attestation conflated "all completed jobs pass" with "all required jobs pass" — an incorrect equivalence when jobs are still running.

**Gate (codified):** A 'CI green' attestation used as a MERGE-READY or PAUSE-CHECKPOINT signal MUST be formed ONLY after:
1. All required checks listed in the branch-protection required-status-checks are in a TERMINAL state (`completed` GitHub status).
2. All terminal states are `success` (or `skipped` for explicitly skipped legs where skipping is expected).
3. The attestation timestamp is at or after the LAST required check reached a terminal state.

Concretely: `gh pr checks <PR>` must show every required leg as `pass` (not `pending` or `in_progress`) before the 'CI green' attestation is written into STATE.md or any pause checkpoint.

**Recommended gate improvement:** A follow-up drift item / small story: a pause/merge-ready attestation helper that enumerates all required checks' terminal state via `gh pr checks` before allowing the attestation string to be recorded. Anchored in STATE.md Drift Items (D-692 Drift Item: [process-gap] CI-green-attestation gate story).

**Consequence if violated:** Pause checkpoints that claim 'CI N/N GREEN' while jobs are still running mislead the next-session resume — the resume agent sees a passed gate and may proceed toward merge without re-verifying. This caused at least 2 extra CI failure cycles and extended the total PR delivery timeline for S-18.14.

**S-7.02 note:** This process-gap is anchored to a concrete follow-up (Drift Item + candidate story). It is NOT a tech-debt-register entry — the deferral is justified by requiring a gate story to be specced with proper BC authorship before implementation.

**Cites:** D-691 DURABLE PAUSE REFINEMENT; D-692 post-merge burst; PR #201 (S-18.14); S-18.14 merged dfc76844 2026-06-23.

---

## L-BB-prereq-story-task-scope-boundary

**Date:** 2026-06-24
**Tags:** [process-gap] [story-decomposition] [adr-coupling]
**Anchors:** D-695, S-18.04b-prereq v1.1, ADR-029 v1.1, S-18.04b

**Lesson (codified):** Prereq story tasks must not assign work that is tightly coupled to the dependent story's scope. When an ADR decomposes a feature into distinct sub-decisions, each sub-decision must be traced to the correct story at decomposition time — not arbitrarily placed in the earliest-delivering story.

**Failure mode:** At D-694, story-writer placed "Update hooks-registry.toml: add Bash PostToolUse entry for factory-artifacts git-commit detection" (T-7) in S-18.04b-prereq. But ADR-029 §Decision 5 explicitly assigns the consuming-plugin rewiring (registry trigger flip + WASM plugin reads git_context) to S-18.04b. The prereq story (S-18.04b-prereq) owns only the dispatcher host side (ADR-029 §Decision 1). The registry trigger flip requires S-18.04b's WASM plugin context — it cannot be meaningfully tested or merged standalone without the WASM plugin side present.

**Root cause:** Story decomposition did not trace each ADR sub-decision to its authoritative story boundary. The decomposer placed all "registry-adjacent" work in the prereq without checking whether the registry change was semantically coupled to the consuming story's scope.

**Gate (codified):** When decomposing an ADR into multiple stories, for each task/deliverable in the prereq story:
1. Identify which ADR §Decision clause authorizes the task.
2. Confirm the clause assigns the work to the prereq scope (not to a later story in the depends_on chain).
3. If the clause assigns to a later story, defer the task explicitly in the prereq and cite the clause.

**Consequence if violated:** Prereq stories accumulate tasks that belong to the dependent story, bloating prereq scope and creating mismatched ACs (the implementation correctly defers but the spec contradicts it — a spec/implementation drift that adversary catches as MEDIUM-1).

**Cites:** D-695 governance fix-burst; ADR-029 §Decision 1 (dispatcher host injection) + §Decision 5 (consuming-plugin rewiring); S-18.04b-prereq v1.0→v1.1; adversary MEDIUM-1.

---

## L-BB-merge-requires-direct-human-action

**Date:** 2026-06-25
**Tags:** [process-gap] [autonomy] [pr-manager] [merge-gate]
**Anchors:** D-665, D-698, SRC-HARDEN v4.53, §3+§9+§10+§11+§12 Session Resume Checkpoint

**Lesson (codified):** The pr-manager agent operates under a hard system rule that it will NOT accept coordinator/orchestrator-relayed human merge approval. Only the user's OWN direct action authorizes a PR merge. Therefore the D-665 STOP-BEFORE-PR-MERGE gate CANNOT be cleared by pr-manager on an orchestrator relay; the human must execute the merge command directly, or the orchestrator must escalate to the human with the exact command to run.

**Root cause:** This is not a bug in pr-manager — it is a deliberate isolation boundary. The system distinguishes between (a) an orchestrator claiming the human approved a merge, and (b) the human directly taking the merge action. pr-manager enforces (b). Sessions that do not know this invariant will stall indefinitely at the D-665 gate when the orchestrator attempts to relay a human approval through pr-manager.

**Failure mode (deadlock pattern):**
1. Story reaches STOP-BEFORE-PR-MERGE gate (D-665).
2. Human grants merge approval in conversation.
3. Orchestrator dispatches pr-manager with "human has approved, proceed with merge."
4. pr-manager refuses — it will not accept the relay.
5. Session deadlocks; no merge occurs.

**Gate (codified):** At every D-665 STOP-BEFORE-PR-MERGE checkpoint, the orchestrator MUST:
1. Surface the exact merge command to the human: `gh pr merge <N> --squash --delete-branch --repo drbothen/vsdd-factory`
2. Explicitly state: "Please execute this command directly — pr-manager cannot relay this action."
3. Wait for the human to confirm the merge was executed before dispatching the post-merge burst.

**Repo identity note:** The origin is `drbothen/vsdd-factory` (NOT the local git user "Zious"). All `gh` commands for this project require `--repo drbothen/vsdd-factory`.

**Consequence if violated:** Session deadlock at every merge gate. The orchestrator waits for pr-manager to confirm merge; pr-manager waits for direct human action that was never routed to the human. Zero-context resume sessions are especially vulnerable — they see a clean PR status and attempt the relay, then stall.

**Cites:** D-665 STOP-BEFORE-PR-MERGE autonomy directive; D-698 S-18.04b POST-MERGE (most recent merge gate); SRC-HARDEN v4.53 SESSION RESUME CHECKPOINT HARDENING 2026-06-25.

---

## L-BB-red-gate-test-plan-ec-coverage-parity

**Date:** 2026-06-25
**Tags:** [process-gap] [story-writer] [red-gate] [codified]
**Anchors:** D-699, F-P1-010, S-18.03, S-18.09 (follow-up gate scope)

**Lesson (codified):** The story-writer Red Gate Test Plan MUST enforce 1:1 edge-case→test coverage at authoring time. Every edge case (EC-NNN) enumerated in the BC that is testable in bats MUST have a corresponding bats Red Gate test row in the story's §Red Gate Test Plan table. EC-004 and EC-006 were enumerated in BC-6.24.001 but shipped without corresponding bats tests until adversary pass-1 (F-P1-010) caught the gap.

**Root cause:** The story-writer discipline for Red Gate Test Plan rows enumerates ACs (AC-001 through AC-N) but does not explicitly walk through every EC-NNN in the source BC to confirm test coverage. Edge cases that are implied by the ACs can be missed if the AC→EC mapping is not explicit.

**Failure mode:** Story ships with a Red Gate Test Plan that covers all ACs but leaves some edge cases (enumerated in the BC) untested. The 3-CLEAN LOCAL cascade seals the story as converged without detecting the coverage gap — the adversary catches it at pass-1, resetting the streak and requiring a fix burst to add the missing tests.

**Gate (codified):** At Red Gate Test Plan authoring time, the story-writer MUST:
1. Enumerate ALL EC-NNN entries from the source BC(s).
2. For each testable EC, confirm a bats test row exists in §Red Gate Test Plan.
3. For non-testable ECs (e.g., manual verification required), annotate as such with justification.

**Candidate gate story:** S-18.09 (process-gap lesson gate checks; wave 8; P1; 5pts) is the appropriate anchor for an automated machine-checkable assertion of this discipline. Do NOT create a new story — anchor to S-18.09.

**Consequence if violated:** Adversary pass-1 resets the 3-CLEAN streak; story requires a fix burst to add missing tests; total cascade passes increase from the nominal minimum. Recurrence risk is high because the gap is invisible to the implementer (bats suite passes for covered ACs; edge cases simply have no test rows).

**Cites:** D-699 S-18.03 POST-MERGE; F-P1-010 S-18.03 LOCAL adversary pass-1; BC-6.24.001 EC-004/EC-006; BC-5.39.001 3-CLEAN protocol; S-18.09 candidate gate scope.

---

## L-BB-4index-parity-rederive-from-live-headers

**Date:** 2026-06-25 (relocated to brownfield cycle 2026-06-26; D-700)
**Tags:** [process-gap] [state-manager] [codified]
**Anchors:** D-700, F-P5-002, S-18.05, VP-INDEX v2.46→v2.47 correction

**Lesson (relocated from F5 cycle — this is a brownfield-cycle operational discipline):** 4-index parity cites in changelog entries and burst-log Dim-3 MUST be re-derived from live index headers at the moment of authoring Commit-D. Carrying forward the prior burst's parity cite is forbidden even when the author believes no index has changed since the prior burst.

**Root cause:** S-18.05 LOCAL adversary Pass-5 (F-P5-002) found that VP-INDEX v2.46's changelog entry cited `STORY-INDEX v4.73` as the 4-index parity snapshot. The actual STORY-INDEX version at that time was v4.74 — the Pass-3 fix burst (adv P3 F-P3-002) had already advanced STORY-INDEX v4.73→v4.74 before Pass-4 ran. The v2.46 author carried forward the parity cite from the prior burst's plan rather than re-deriving from live headers at authoring time.

**Discipline (re-derivation rule):** Before writing the 4-index parity cite in any changelog entry or burst-log Dim-3, the state-manager MUST execute:

```bash
grep "^version:" \
  .factory/specs/behavioral-contracts/BC-INDEX.md \
  .factory/specs/verification-properties/VP-INDEX.md \
  .factory/stories/STORY-INDEX.md \
  .factory/specs/architecture/ARCH-INDEX.md
```

and use the live values. "Carrying forward" the prior burst's parity cite is forbidden regardless of whether the author believes no index has changed since the prior burst. Index versions can advance between the plan phase and the Commit-D authoring phase if a sibling fix burst for the same story touches a different index in the same session.

**E-18 self-improvement family anchor:** Same structural class as the registry↔ADR lint follow-up process-gap from S-18.04b (carry-forward instead of re-derive). The cure in both cases is a mandatory live grep at the moment of write rather than trusting plan-phase snapshots.

**Impact of this instance:** VP-INDEX v2.46 changelog mis-cited STORY-INDEX v4.73 (off-by-one from the actual v4.74 at that time). Corrected in VP-INDEX v2.47 (S-18.05 P5 fix burst). No behavioral or content error — changelog text accuracy only.

**Cites:** D-700 S-18.05 POST-MERGE; F-P5-002 S-18.05 LOCAL adversary pass-5; VP-INDEX v2.46→v2.47 correction.

---

## L-BB-vp-cite-stable-anchor-recurrence-proof

**Date:** 2026-06-26 (D-700)
**Tags:** [process-gap] [story-writer] [state-manager] [codified]
**Anchors:** D-700, F-P7-001, S-18.05 v1.9, POLICY-19-analog, TD-VSDD-091

**Lesson (codified):** VP (and ADR) version pins in normative bodies — story files and STORY-INDEX catalog rows — are volatile anchors. A version cite like `VP-089 v1.3` in a story spec or index row becomes stale the moment any subsequent fix burst advances the VP version. The correct anchor is the bare identifier `VP-089` with no version token: stable, recurrence-proof, and always points to the current canonical truth.

**Root cause:** S-18.05 LOCAL adversary Pass-7 (F-P7-001) found that the S-18.05 story file (v1.8) and its STORY-INDEX row cited `VP-089 v1.3`. VP-089 had been advanced to v1.4 by the Pass-5 fix burst, making the v1.3 cite stale. The story-writer discipline at authoring time did not enforce this and the STORY-INDEX update at the same burst did not drop the version token.

**Scope of rule:** Volatile version pins are disallowed in:
- Story file `verification_properties:` body citations (e.g., `VP-089 v1.3` → `VP-089`)
- STORY-INDEX catalog row VP-cite columns (e.g., `VP-089 v1.3` → `VP-089`)
- Any normative body prose that references a VP or ADR by version (exceptions: changelog rows where version is part of the historical record)

Justified exceptions (per TD-VSDD-091): Red Gate test tables and AC source-of-truth tables that cite specific version snapshots for audit purposes may retain version pins if the justification is documented inline.

**Recurrence-proofing action at Commit-D:** When any VP (or ADR) version is advanced in a fix burst, the state-manager MUST grep the story file(s) and STORY-INDEX for the old version cite and drop the version token to the bare identifier before committing. This is the Commit-D sibling-sweep obligation for VP version tokens.

**Analogy:** TD-VSDD-091 / POLICY-19-analog: ADR/VP version pins in normative bodies are volatile. Stable anchors (no version token) are recurrence-proof.

**Cites:** D-700 S-18.05 POST-MERGE; F-P7-001 S-18.05 LOCAL adversary pass-7; S-18.05 v1.8→v1.9 (dropped VP-089 v1.3 → VP-089); STORY-INDEX v4.75→v4.76; TD-VSDD-091; POLICY-19-analog.

---

## L-BB-phantom-substrate-field-cascade

**Date:** 2026-06-26 (D-700)
**Tags:** [process-gap] [adversary] [state-manager] [codified]
**Anchors:** D-700, F-P5-001, S-18.05 LOCAL cascade passes 5+, STATE.md `last_verified_develop_sha`

**Lesson (codified):** A phantom field in a normative artifact (a field cited in spec as if it exists, but which does not exist in the implementation or the actual artifact) causes a cascade of stale-reference findings in subsequent adversary passes. Each pass that encounters the phantom cite re-raises or finds residual references to the closed finding, requiring additional sibling-sweeps until all locations carrying the phantom cite are purged.

**Root cause:** S-18.05's AC-003 referenced a STATE.md field `last_verified_develop_sha` as the postcondition verification target. That field does not exist in STATE.md — it was never defined in the BC or the STATE.md spec. As a result:
- Pass-5 (F-P5-001) raised the phantom-field finding as status=warn (AC-003 postcondition references non-existent field).
- The fix burst removed the field reference from AC-003, closing the finding.
- But the STORY-INDEX v4.75 row still carried the stale status=warn token, which itself required a follow-up sweep at Pass-7.

**Structural pattern:** Phantom field → adversary finds it → fix burst removes it from the story → adjacent artifacts (STORY-INDEX rows, 4-index parity cites) still carry stale references → subsequent passes raise stale-reference findings → requires a second sweep. The full closure requires a sibling-sweep to all locations that carried the phantom cite.

**Gate (codified):** When a story's adversary finds a phantom field reference in an AC postcondition or verification step:
1. The fix burst MUST remove the reference from the story file AND grep for the field name in STORY-INDEX, BC files, and VP files.
2. The state-manager MUST confirm the field either (a) exists in the actual artifact with the expected semantics, or (b) is removed from ALL normative cites in the same burst.
3. If the field is a legitimate gap in the substrate artifact (e.g., STATE.md is missing a field the spec assumed), the fix burst MUST either add the field to the substrate or remove the AC that depends on it — no half-closed state.

**Cites:** D-700 S-18.05 POST-MERGE; F-P5-001 S-18.05 LOCAL adversary pass-5; STATE.md phantom field `last_verified_develop_sha`; S-18.05 v1.8 AC-003 → v1.9 (removed phantom cite); STORY-INDEX v4.75→v4.76 stale status=warn token cleanup.

---

## L-BB-blocker-fix-must-not-regress-canonical-tv-coverage

**Date:** 2026-06-28 (D-718)
**Tags:** [process-gap] [implementer] [adversary] [codified]
**Anchors:** D-718, F-P2-001, F-P2-002, S-18.10, BC-6.25.001 v1.1

**Lesson (codified):** A BLOCKER fix that rewrites an implementation MUST be verified against ALL canonical test vectors in the relevant BC — not just the existing bats fixture set. The regression class: a BLOCKER fix correctly closes the HIGH finding but, if the implementer verifies only against the current bats fixture (which may have been authored before all ECs were specified), the fix may silently regress a second EC that was not yet covered in bats.

**Root cause:** S-18.10 LOCAL adversarial pass-2 found F-P2-001 HIGH (single-line jq parse: `check-autocompact-setting.sh` used `jq -r '.env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE'` which fails on single-line settings.json with no newlines). The fix rewrote the jq invocation to a multi-field-safe approach. However, the original bats fixture did not yet cover EC-011 (non-numeric/empty-string treated as absent) or EC-012 (negative/zero values treated as out-of-range advisory). The jq-parser rewrite needed to close all three simultaneously, and the BC needed to be amended (v1.1) to canonicalize EC-012 before the fix could be called complete.

**Pattern:** Fix-introduced-regression class. The implementer over-corrected from unguarded-python3 → pure-bash, which correctly closed the WASM portability finding but introduced single-line parsing fragility. The adversary correctly identified both the fix-induced regression (F-P2-001) AND the missing EC coverage (F-P2-002/F-P2-004) as a combined class.

**Gate (codified):** Before declaring any BLOCKER fix done — especially a fix that rewrites a core parsing mechanism:
1. Grep the BC file for ALL ECs in the relevant AC (e.g., `grep "EC-[0-9]" .factory/specs/behavioral-contracts/ss-06/BC-6.25.001.md`).
2. Confirm every EC has at least one bats test that exercises the corresponding input class.
3. If any EC is not yet covered by bats, add the test in the same fix commit. Do NOT declare green until the full EC vector is covered.

**Analogy:** SOUL.md §4 — silent `Vec::new()` return where partial-failure data should propagate. The bats fixture gap is equivalent: a green-returning parser that silently misclassifies an EC is a false-green, not a true-green.

**Cites:** D-718 S-18.10 IN-FLIGHT SPEC-AMENDMENT; F-P2-001 HIGH (jq single-line parse); F-P2-002 MEDIUM (EC-011 coverage); F-P2-004 (EC-012 advisory); BC-6.25.001 v1.0→v1.1; S-18.10 v1.3→v1.4; LOCAL 3-CLEAN streak 0/3.

---

## L-BB-red-gate-fixture-must-mirror-bc-canonical-test-vectors

**Date:** 2026-06-28 (D-719)
**Tags:** [process-gap] [test-writer] [adversary] [codified]
**Anchors:** D-719, S-18.10, BC-6.25.001, F-P8-001 (EC-008 empty-string gap pass-8 MEDIUM)

**Lesson (codified):** Red Gate bats fixtures MUST mirror the BC's Canonical Test Vectors verbatim — including single-line JSON forms and exact note/advisory strings — not just the minimal fixture set the test-writer initially derives. Under-specified fixtures create silent EC coverage gaps that a fresh-context adversary will catch in later passes.

**Root cause:** S-18.10 LOCAL adversarial pass-8 caught F-P8-001 MEDIUM (EC-008 empty-string gap): the initial bats fixture set used only multi-line pretty-printed JSON for settings.json and asserted only status-presence (ADVISORY row exists), not the exact advisory note string. BC-6.25.001 §Canonical Test Vectors include a single-line JSON input AND specify exact advisory note content (e.g., `"Value <N> is not a valid compaction percentage..."`). The bats fixture asserted `grep -q "ADVISORY"` but did not assert the exact note — so when the EC-008 empty-string path was exercised with a fixture not in the original set, the gate passed vacuously. Pass-8 caught this as MEDIUM: the test is present but under-specified relative to the BC canonical test vector.

**Pattern:** The "fixture-under-specification" class. Test-writer authors bats tests that assert presence of a keyword (ADVISORY, PASS, FAIL) but not the verbatim expected output required by the BC's Canonical Test Vector section. The BC is authoritative for exact output form; bats is authoritative for repeatability. When they diverge, the adversary finds the gap.

**Additional context:** The jq strategy adjudication at pass-8 (D-719) also required architect involvement because the `jq` absent-vs-empty distinction (`.env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE // empty` vs `// null`) produces subtly different output for EC-008 (empty-string in settings.json). The BC's EC-012 amendment (PO BC v1.1) added the canonical test vector for ≤0 case; the corresponding bats test was missing until the adversary raised it.

**Gate (codified):** When test-writer authors bats fixtures for a story with BC §Canonical Test Vectors:
1. For EACH canonical test vector in the BC, there MUST be a corresponding bats test that: (a) uses the exact JSON/input form from the CV (e.g., single-line `{"env":{"KEY":"VALUE"}}`), AND (b) asserts the exact expected output string from the CV (not just status-presence).
2. If the BC specifies an exact note string (e.g., `Value <N> is not a valid...`), the bats test MUST assert that exact string, not just `grep -q ADVISORY`.
3. The implementer MUST cross-check: grep all `EC-NNN` labels in the relevant AC section of the BC file, confirm each has a corresponding bats test with CV-verbatim input and CV-verbatim expected output.

**Relationship to L-BB-blocker-fix-must-not-regress-canonical-tv-coverage:** These two lessons are complementary. L-BB-blocker-fix-must-not-regress covers the regression introduced by a BLOCKER fix. L-BB-red-gate-fixture-must-mirror-bc-canonical-test-vectors covers the original under-specification of fixtures that creates the gap for later adversary passes to exploit.

**Follow-up story anchor:** Satisfied by codification — no follow-up story required. The gate is a test-writer discipline rule enforced by the adversary in LOCAL cascade (BC-5.39.001 3-CLEAN protocol). No automation story needed at this time; apply at test-writer dispatch for all future E-18/F4 stories.

**Cites:** D-719 S-18.10 POST-MERGE; F-P8-001 MEDIUM EC-008 empty-string gap; BC-6.25.001 §Canonical Test Vectors; S-18.10 LOCAL 11-pass cascade CONVERGED 3-CLEAN passes 9/10/11; jq strategy adjudication at pass-8; PO BC v1.1 EC-012 amendment.

---

## L-S18.11-stale-cite-bats-headers-skill-prose

**Date:** 2026-06-29 (D-721)
**Tags:** [process-gap] [story-writer] [test-writer] [codified]
**Anchors:** D-721, S-18.11, ADR-026, BC-5.41.004, TD-VSDD-091

**Lesson (codified):** Version tokens in bats test header comments and SKILL.md algorithm prose drift silently when the underlying BC/ADR is amended. S-18.11's 14-pass LOCAL cascade saw multiple recurrences of finding class: bats header comments cited `BC-5.41.004 v1.N` (stale version token) when the BC had already advanced; SKILL.md algorithm prose described wave-boundary logic anchored to ADR-026 §Decision 3a at a prior version (def-a) while the normative ADR had advanced to def-b (human directive v1.37). The stale cites are non-blocking per TD-VSDD-091 (bats doc-comments are not normative prose), but they confused adversary analysis by presenting stale algorithm descriptions as current.

**Root cause:** When BC-5.41.004 advanced from v1.0 to v1.4 across 4 cascade commits, bats header comments citing `v1.N` were not swept. When ADR-026 advanced from def-a to def-b, SKILL.md algorithm description was not updated in the same burst.

**Prevention:** Two-part rule:
1. **De-pin volatile version tokens in bats headers:** Bats test header comments that cite `BC-X.XX.XXX vN.N` MUST drop the version token, citing only `BC-X.XX.XXX` (per TD-VSDD-091). A version token in a bats header is always going to become stale. Apply at initial authoring, not retroactively.
2. **SKILL.md algorithm prose sweep on BC/ADR def-class change:** When an ADR §Decision N changes its algorithmic definition (e.g., def-a → def-b), all SKILL.md files that have algorithm prose anchored to that decision MUST be swept in the same burst. The ADR is the SoT; SKILL.md prose is a consumer. Consumer prose must stay synchronized with the SoT.

**Follow-up anchor:** TD-VSDD-091 sweep discipline — apply at every BC/ADR amendment. No dedicated story needed; apply at every relevant dispatch.

**Cites:** D-721, S-18.11 14-pass cascade, ADR-026 def-a→def-b, BC-5.41.004 v1.0→v1.4, TD-VSDD-091.

---

## L-S18.11-architect-graph-claim-must-be-mechanically-verified

**Date:** 2026-06-29 (D-721)
**Tags:** [process-gap] [architect] [adversary] [codified]
**Anchors:** D-721, S-18.11, ADR-026 §EC-010, S-3.04 partial, ADR-015 supersession

**Lesson (codified):** Architect claims about graph topology must be mechanically verified against the actual STORY-INDEX graph before they are codified into ADR decisions. During S-18.11's cascade, the architect initially claimed there were zero terminal→non-terminal depends_on edges in the production graph. The adversary discovered 5 such edges via S-3.04's `partial` status path (S-3.04 depends_on BC-5.41.001 consumer stories via the ADR-015 supersession chain, creating terminal→non-terminal edges). This caused a mid-cascade ADR amendment (EC-010 narrowed to tolerate supersession edges v1.36) that reset the 3-CLEAN streak.

**Root cause:** The architect reasoned about the expected graph structure from memory ("no terminal depends on non-terminal in E-18 epic") without running the graph traversal against the actual STORY-INDEX.md. The S-3.04 partial status is a brownfield historical artifact from ADR-015 supersession — not part of E-18, but it appears in the STORY-INDEX depends_on graph.

**Prevention:** Before codifying any architectural claim about graph topology (e.g., "topological sort is valid", "no cross-category edges exist"), the architect MUST execute a literal graph traversal against the actual STORY-INDEX.md (or equivalent data source) and capture the result as evidence. Claims of the form "there are no X edges" require a grep/awk scan to verify, not just reasoning.

**Follow-up anchor:** Candidate for post-E-18 graph verification gate story (E-18 F3 family): a mechanical gate that traverses the STORY-INDEX depends_on graph and validates stated topology invariants. Not blocking S-18.11 delivery; anchored as post-E-18 improvement.

**Cites:** D-721, S-18.11 cascade passes 5-6, ADR-026 §EC-010 v1.36, S-3.04 partial, ADR-015 supersession, architect claim correction mid-cascade.

---

## L-S18.11-wave-design-monotonic-completion-assumption

**Date:** 2026-06-29 (D-721)
**Tags:** [observation] [architect] [process-gap]
**Anchors:** D-721, S-18.11, ADR-026 §Wave-Identity, S-3.04 partial, ADR-015 supersession

**Lesson (observation):** The wave-boundary and sprint-state design implicitly assumes monotonic wave completion — i.e., all stories from wave N complete before wave N+1 begins, and story status monotonically advances from draft → ready → in-progress → merged (no backward transitions, no abandoned stories). vsdd-factory's own self-referential history violates this assumption: S-3.04 remains `partial` (superseded but not merged/withdrawn), and early epics were abandoned mid-execution. This creates edge cases in the sprint-state.yaml producer (what status to emit for S-3.04: `partial` is a valid status per the STORY-INDEX-grounded 8-value enum) and in the wave-gate consumer (should `partial` be in the next-wave allowlist?).

**Resolution in S-18.11:** The `partial` status was accepted as a valid terminal-equivalent status for sprint-state.yaml purposes (the story is done enough to be treated as non-blocking for downstream waves). The wave-gate consumer allowlist was updated to include `+partial` in the S-18.11 PR.

**Broader observation:** Future wave-design decisions should explicitly document monotonic-completion assumptions and their exceptions. Self-referential projects (where the engine is also the product under development) are particularly prone to non-monotonic history because the development methodology itself evolves. Any automation that traverses the story graph should have explicit handling for: (a) superseded stories, (b) partial statuses, (c) abandoned/withdrawn stories at various dependency depths.

**Follow-up anchor:** POST-E-18 revisit: ADR-015 supersession chain implications for S-3.04 and Router/WASM multi-sink revival (human-directed deferral per D-721). When Router/WASM multi-sink is revived, the S-3.04 status decision should be revisited. Anchor: post-E-18 Router story or ADR-015 revisit cycle.

**Cites:** D-721, S-18.11, ADR-026 §Wave-Identity, S-3.04 partial, ADR-015 supersession, wave-design monotonic-completion assumption.

---

## L-BB-regression-detector-tests-must-emit-positive-coverage-line

**Date:** 2026-06-30 (D-730)
**Tags:** [process-gap] [test-writer] [adversary] [codified]
**Anchors:** D-730, S-18.12, O-4 LOCAL adv pass-6, Cycle-Closing Checklist S-7.02 step-3

**Lesson (codified):** Regression-detector tests (static lint gate tests that scan a file set) MUST emit a machine-greppable positive-coverage line on the success path indicating at minimum the number of files scanned. A silent green exit code cannot distinguish between "scanned 5 files, all PASS" and "scanned 0 files, nothing flagged" — the latter is a silent scope-narrowing failure that is structurally undetectable without explicit output.

**Root cause:** S-18.12's 5 portability regression-detector tests reported only a pass/fail exit code on the PASS path. The bats test harness shows a green dot for each passing test, but there is no way to determine from the test output whether the test scanned 5 files or 0 files. If the scan glob were tightened (e.g., a path change that yields no matches), all 5 tests would continue to pass without any indication that the scope had silently contracted.

**Prevention gate (codified):** Every regression-detector test (i.e., any test that operates by: (1) collecting a set of files via glob/find, then (2) scanning each file for a pattern, then (3) asserting zero matches) MUST include an explicit output statement on the success path of the form:

```
echo "AC-00N: scanned=${#sh_files[@]} files" >&3
```

or equivalent. The output MUST include the count of files scanned (so a scope-narrowing silent failure would produce `scanned=0 files` as an observable signal). The count must reference the actual variable used to build the scan set, NOT a hardcoded literal.

**Minimum required output:** `AC-00N: scanned=N files` where N is the actual count from the scan-set variable.

**Anti-pattern blocked:** Tests that assert "no prohibited patterns found" without confirming "scanned N>0 files". A test that passes vacuously on an empty scan set is not a test — it is a no-op.

**Follow-up anchor:** Apply at test-writer dispatch for all future E-18/F4 gate-story tests that use a collect-then-scan-then-assert-zero pattern. No dedicated story needed; enforce at test-writer review time.

**Cites:** D-730, S-18.12 LOCAL adv pass-6 O-4, Cycle-Closing Checklist step-3 (process-gap codification obligation), 5/5 portability tests updated 77147d3e.

---

## L-S18.12-asymptotic-clean-accept-prospective-lows-for-streak

**Date:** 2026-06-30 (D-733)
**Tags:** [codified] [convergence] [adversary] [story-writer]
**Anchors:** D-733, S-18.12 LOCAL adv pass-9 CLEAN, BC-5.39.001 3-CLEAN protocol, L-F2-3clean-streak-requires-frozen-package

**Lesson (codified):** For asymptotic regex-predicate stories (stories whose acceptance criteria are bounded regular-expression detectors scoped to a specific syntactic form-set), non-blocking LOW observations about prospective syntactic forms NOT exhibited by any current scan-target should be accepted as documented prospective scope boundaries and NOT hardened mid-streak. Hardening them changes the artifact, gives the next fresh-context adversary new surface to probe, and resets the 3-CLEAN streak counter to 0/3 — defeating the purpose of the convergence protocol.

**Root cause pattern observed in S-18.12 passes 5-9:** After a CLEAN pass, the next-pass adversary can find new LOW observations about forms not present in the current corpus. If these observations are hardened (even though they are LOW and non-blocking), the artifact changes. The following fresh-context pass then has an updated artifact that may exhibit new gaps (or the same gap in a slightly different form), producing a NOT-CLEAN finding, resetting the streak. This pattern was observed across passes 5→6 (O-1 hardened at pass-5 created a new variant explored at pass-6) and pass-7→8 (O-1 hardened again at pass-7 created a new variant explored at pass-8). The oscillation cycle (CLEAN→NOT-CLEAN×hardening→CLEAN→NOT-CLEAN×hardening) terminates only when we stop hardening prospective-only LOWs.

**Prevention gate (codified):** When a CLEAN pass returns only LOW observations that are all of the form "syntactic form X is not detected, but no current script uses X":

1. Evaluate each observation: does any current file in the scan corpus exhibit the identified syntactic form? If YES → it is a real gap → fix it. If NO → it is prospective-only → ACCEPT as documented scope boundary per the decision below.
2. For each accepted observation, record the explicit rationale: "form-set is intentionally bounded to [enumerated forms]; no current scan-target script uses [undetected form]; this is a forward-looking observation about potential future scripts."
3. Do NOT harden the artifact. The artifact stays byte-stable. The next fresh-context adversary reviews the IDENTICAL artifact.
4. Record the acceptance in the decision-log (D-NNN) and the STORY-INDEX annotation.
5. If the next pass also returns only the same prospective LOWs → accept again (they are the same observation about the same prospective gap; the acceptance was correct). Streak advances.

**Companion rule:** This lesson extends L-F2-3clean-streak-requires-frozen-package (D-631): "3-CLEAN convergence requires a FROZEN package — CLEAN passes record verdict + advance streak ONLY; non-blocking observations are deferred-with-anchor, never fixed mid-streak." The present lesson adds precision: for prospective-syntactic-form LOW observations on bounded regex-predicate detectors, the correct deferral anchor is "if a future script introduces this form into the corpus, the guard MUST be extended at that time" — not a story ID (since no story is warranted now) but an explicit future trigger condition.

**Anti-pattern blocked:** Reflexively hardening every LOW observation from a CLEAN pass. This is the "let's clean up while we have a CLEAN pass" impulse — it seems like a low-cost improvement but structurally perturbs the artifact, destroys the streak, and requires 3 more consecutive passes to recover. For a regex-predicate story at pass-9, the cost of reflexive hardening is 2-3 additional passes (each pass takes adversary dispatch + orchestrator + state-manager time). The correct behavior is: ACCEPT the prospective LOW, FREEZE the artifact, dispatch pass-10 on the same artifact.

**Scope boundary:** This lesson applies specifically to LOW observations about prospective syntactic forms not exhibited by any current scan-target. It does NOT apply to: (a) LOW observations about current scripts that exhibit the identified gap; (b) MEDIUM or HIGH findings; (c) [process-gap] observations (those must be routed to follow-up regardless of severity).

**Follow-up anchor:** Applied at D-733 S-18.12 LOCAL adv pass-9 CLEAN closure. Carry forward to all future E-18/F4 gate-story LOCAL cascades.

**Cites:** D-733, S-18.12 passes 5-9 oscillation pattern, BC-5.39.001, L-F2-3clean-streak-requires-frozen-package (D-631), O-1..O-4 accepted-as-documented-prospective-scope-boundaries.

---

## L-BB-regex-arm-must-have-positive-control

**Date:** 2026-06-30 (D-735)
**Tags:** [codified] [process-gap] [test-writer] [adversary]
**Anchors:** D-735, S-18.12 LOCAL adv pass-11 F-P13-001, POLICY 11, POLICY 13, BC-5.39.001 3-CLEAN protocol

**Lesson (codified):** Every enumerated regex alternation arm in a portability-lint or regression-detector story MUST have a dedicated, byte-identical positive control in the bats test suite. When an alternation arm has no positive control, the arm can be silently dropped or narrowed — the test suite will continue to pass vacuously with no observable failure signal.

**Root cause:** S-18.12 LOCAL adversary pass-11 identified that several arms of the jq, IFS, and AC-002 detectors (jq bare line-start `^jq`, jq backtick form, jq keyword wrappers if/then/do/elif/env/command/sudo; IFS `||` and do/else/elif keyword prefixes; AC-002 `${*^^}`) had no dedicated positive control row in the test file. POLICY 11 requires non-tautological positive controls byte-identical to the regex arm; POLICY 13 requires that an adversary can verify coverage by inspection. Without per-arm controls, there is no automated signal when an arm is lost.

**Prevention gate (codified):** For every regex alternation arm in a portability-lint story:

1. The test file MUST contain at least one `run bash -c "<command matching that arm>"` assertion in a positive-control section.
2. The positive control string MUST be byte-identical to (or a canonical instance of) the arm being tested — not a general "something containing the keyword" string.
3. When adding a new arm to a regex, the test-writer MUST add the positive control in the SAME commit as the regex change.
4. Adversary review MUST enumerate detector arms and verify each has a corresponding positive-control test row (POLICY 13 coverage verification).

**Minimum required test structure per detector:**
```
# Positive controls — MUST flag
run bash -c "scan_target_with_arm_1_form"
assert_failure
run bash -c "scan_target_with_arm_2_form"
assert_failure
# ... one row per enumerated arm ...
```

**Anti-pattern blocked:** Detector with 8 alternation arms but only 3 positive controls (covers the "common" forms but leaves edge arms untested). A fresh-context adversary that enumerates the regex structure against the test suite will find the gap immediately — this is exactly F-P13-001 in S-18.12 pass-11.

**Follow-up anchor:** Apply at test-writer dispatch for all future E-18/F4 gate-story tests that use alternation-regex detectors. This is an additive obligation complementary to L-BB-regression-detector-tests-must-emit-positive-coverage-line (D-730): that lesson covers "scanned N files" output; this lesson covers "per-arm positive control" coverage.

**Cites:** D-735, S-18.12 LOCAL adv pass-11 F-P13-001, POLICY 11, POLICY 13, test-writer 1d49cb85 (additive positive controls for all identified arms; no regex change).

---

## L-BB-sibling-sweep-same-contract-clause

**Date:** 2026-06-30 (D-735)
**Tags:** [codified] [process-gap] [story-writer] [adversary] [TD-VSDD-060]
**Anchors:** D-735, S-18.12 LOCAL adv pass-11 F-P11-001 + TD-VSDD-060 sibling-sweep, SKILL.md §149, BC-5.39.001 3-CLEAN protocol

**Lesson (codified):** When reconciling one detector's acceptance criterion to a source-of-truth contract clause, a TD-VSDD-060 sibling-sweep MUST be performed across ALL detectors governed by the SAME contract clause. Fixing one detector and leaving a sibling detector inconsistent with the same clause creates a mirror-asymmetry that the next fresh-context adversary will find — effectively guaranteeing a NOT-CLEAN pass and streak reset.

**Root cause pattern observed in S-18.12 pass-11:** F-P11-001 required reconciling AC-004 (Python detector) to SKILL.md §149 ("MUST NOT shell out to Python, jq, or any language runtime beyond bash"). After AC-004 was reframed to Option A (forbid all Python, no preflight), the mandatory TD-VSDD-060 sibling-sweep revealed that AC-005 (jq detector) was still using a preflight-acceptance model — inconsistent with the same SKILL.md §149 sentence that was just used to justify the AC-004 change. The python/jq asymmetry was a mirror-blocker: the next adversary reviewing the patched artifact at v1.10 would have found it immediately (the contract clause explicitly names both "Python" and "jq" in the same breath).

**Prevention gate (codified):** When reconciling any acceptance criterion to a contract clause (SKILL.md sentence, ADR decision, BC invariant):

1. Identify ALL acceptance criteria in the same story that are governed by the SAME contract clause.
2. Perform the sibling-sweep BEFORE declaring the reconciliation complete.
3. If any sibling AC is inconsistent with the same clause — even if the sibling was previously accepted — remediate it in the SAME burst. Leaving the sibling inconsistent is not a deferral; it is a guaranteed streak reset.
4. Document the sibling-sweep explicitly in the burst-log Dim-1 (Files Touched) and burst-log Codifications (Dim-6).

**The critical distinction — sibling vs separate finding:**
- If the sibling inconsistency was present BEFORE the reconciliation, it is a latent defect surfaced by the sweep (fix in-scope per Canonical Principle Rule 4).
- If the sibling inconsistency was CREATED by the reconciliation (new asymmetry), it must also be fixed in-scope.
- Either way, the correct action is fix-in-scope during the same burst — not "await next adversary pass."

**How SKILL.md §149 makes this structural:** The sentence "MUST NOT shell out to Python, jq, or any language runtime beyond bash" governs BOTH AC-004 (Python) and AC-005 (jq). Any policy decision applied to one is mandated by the same clause for the other. The option model chosen (A/B/C) MUST be symmetric across both. Any asymmetry is a contract violation detectable in a single read.

**Anti-pattern blocked:** "We fixed AC-004 (Python) → Option A. AC-005 (jq) still uses Option C (preflight). They're different ACs so we can reconcile them separately." This reasoning is wrong: they are governed by the SAME clause. The asymmetry creates a guaranteed fresh-context finding. The correct behavior is: fix AC-004 AND perform the sibling-sweep for AC-005 in the SAME burst.

**Follow-up anchor:** Apply at story-writer and test-writer dispatch for all future E-18/F4 gate stories that have multiple detectors. Before declaring any detector reconciliation complete, enumerate all other detectors in the same story and check whether they are governed by the same source-of-truth clause. If yes, reconcile them simultaneously.

**Cites:** D-735, S-18.12 LOCAL adv pass-11 F-P11-001 + AC-005 sibling-sweep, SKILL.md §149, TD-VSDD-060, test-writer e5d71b85 + story-writer 963d5241 + technical-writer e122cdb0 (AC-005 Option A sibling remediation).

---

## L-BB-per-arm-control-sibling-detector-sweep

**Date:** 2026-06-30 (D-736)
**Tags:** [codified] [process-gap] [test-writer] [adversary] [TD-VSDD-060] [S-7.01]
**Anchors:** D-736, S-18.12 LOCAL adv pass-12 F-P12-001, POLICY 11, POLICY 13, TD-VSDD-060, S-7.01 Partial-Fix Regression Discipline

**Lesson (codified):** When adding per-arm positive controls to one detector to satisfy POLICY 11/13 coverage requirements, a TD-VSDD-060 sibling-sweep MUST be applied with the SAME per-arm treatment to EVERY structurally-identical sibling detector — including newly-created detectors added in the same burst. A newly-created sibling with identical regex topology (same number of alternation arms, same structural pattern) needs identical arm-coverage depth as the repaired sibling. Partial application — treating the established detector fully but leaving the new sibling at shallow coverage — is a false-green gap under S-7.01 Partial-Fix Regression Discipline.

**Root cause pattern observed in S-18.12 pass-12:** D-735 burst fixed F-P13-001 (POLICY 11/13 arm-coverage gap in jq detector) by adding per-arm positive controls for all 9 jq_re arms via test-writer 1d49cb85. In the same D-735 burst, a new `python_re` detector was created as part of the AC-004 Option A redesign (F-P11-001). python_re has the same 9-arm alternation topology as jq_re (both guard against multi-context shell-outs using the same structural pattern: `^arm | $( | operator | backtick | keyword-wrappers | xargs-opts | brace | case | subshell`). However, python_re only received positive controls for 3 of its 9 arms in the D-735 burst, leaving 6 arms unexercised. The fresh-context adversary at pass-12 enumerated the regex structure and immediately identified the gap — this is exactly what POLICY 13 requires adversaries to do.

**The structural identity check (prevention gate):** When ANY detector receives per-arm positive controls for POLICY 11/13 compliance:

1. Enumerate ALL detectors in the same test file with the same structural topology (same number of alternation arms, same alternation pattern).
2. For each structurally identical sibling, verify that it has the SAME arm-coverage depth as the just-repaired detector.
3. If a sibling has shallower coverage — whether it is pre-existing or newly created in the same burst — apply the per-arm treatment to the sibling IN THE SAME COMMIT or the SAME BURST.
4. Newly-created detectors are NOT exempt: a detector created in the same burst that introduced arm-coverage repairs MUST receive the same treatment before the burst closes.

**The critical distinction — structural topology vs. semantic domain:**
- Two detectors can have different behavioral semantics (one guards Python, one guards jq) but identical structural topology (both have a 9-arm alternation covering the same execution-context categories).
- Structural topology governs the arm-coverage obligation; semantic domain does not.
- If jq_re and python_re both have 9 arms covering the same execution contexts (line-start, subshell, operators, backtick, keywords, xargs-opts, brace-group, case-pattern, subshell-close), then the per-arm-control obligation is SYMMETRIC across both, regardless of which language runtime they guard against.

**How TD-VSDD-060 applies here:** TD-VSDD-060 mandates a sibling-sweep "when changing a function signature, constant, or canonical identifier." By extension (production-grade default), it also applies when satisfying a property obligation (POLICY 11/13 arm-coverage) on one member of a sibling group: the SAME property obligation must be verified and satisfied for all structurally identical siblings. Failing to sweep means the property is only partially enforced across the group — a guaranteed adversary finding.

**Anti-pattern blocked:** "We fixed jq_re's arm-coverage (F-P13-001) by adding per-arm controls. We created python_re in the same burst. python_re has only 3 of 9 arms covered. We'll fix python_re later if an adversary finds it." This reasoning is wrong: the structural topology is identical, the property obligation is symmetric, and the adversary WILL find it at pass-12. The correct behavior: when closing F-P13-001, enumerate all detectors with the same topology and apply per-arm controls to all of them in the same burst — including newly-created siblings.

**Follow-up anchor:** Apply at test-writer dispatch for all future E-18/F4 gate-story tests that use alternation-regex detectors. Before declaring any per-arm coverage repair complete, enumerate all other detectors in the same test file/story, check for structural topology parity, and apply the per-arm treatment symmetrically.

**Complementary lessons:**
- L-BB-regex-arm-must-have-positive-control (D-735): every regex alternation arm MUST have a dedicated positive control.
- L-BB-sibling-sweep-same-contract-clause (D-735): when reconciling one detector to a contract clause, sibling-sweep all detectors governed by the same clause.
- This lesson adds: when ADDING arm-coverage to one detector (regardless of cause), sibling-sweep all structurally-identical detectors for the same arm-coverage depth obligation.

**Cites:** D-736, S-18.12 LOCAL adv pass-12 F-P12-001, POLICY 11, POLICY 13, TD-VSDD-060, S-7.01, test-writer f725426e (18 additive fixtures; python_re 9-arm positive-control parity; no regex change; 68/68 GREEN).

---

## L-BB-scan-loop-and-controls-must-share-regex-variable

**Date:** 2026-06-30 (D-737)
**Tags:** [codified] [process-gap] [test-writer] [adversary] [POLICY 11]
**Anchors:** D-737, S-18.12 LOCAL adv pass-13 F-P13-001, POLICY 11, TD-VSDD-060

**Lesson (codified):** When a bats test carries BOTH positive/negative control assertions AND a scan loop over real files, BOTH code paths MUST reference the SAME single regex variable. An inline literal copy of the regex inside the scan loop — even if byte-identical to the variable's current value — is a latent false-green: the controls certify correctness of one expression while the scan loop runs a separately-maintained literal. A future patch that updates the variable but misses the literal (or vice versa) will produce silent divergence: controls pass vacuously while the scanner runs wrong patterns against real files.

**Root cause pattern observed in S-18.12 pass-13:** After passes 1-12 added per-arm positive controls (POLICY 11 arm-coverage), the control assertions all referenced extracted regex variables. However, the scan loop over real wave-handoff files in each detector continued to use an inline hard-coded literal copy of the regex. Both the variable and the literal happened to be byte-identical — but the structural duplication created a divergence risk that any future edit to one could miss the other. The fresh-context adversary at pass-13 identified this as a latent false-green class independent of the current byte-identical state.

**Prevention gate (codified):** For every detector test that contains both control assertions and a scan loop:

1. Extract exactly ONE regex variable per detector (e.g., `array_re`, `guard_re`, `case_mod_re`, `python_re`, `jq_re`).
2. BOTH the control assertions (`run bash -c "...$var..."` / `grep -E "$var"`) AND the scan loop (`grep -E "$var" "$file"`) MUST reference this SAME variable.
3. Inline literal copies of the regex are FORBIDDEN in the scan loop once a variable extraction has been performed.
4. At test-writer dispatch: after extracting regex variables for controls, grep the test file to confirm no inline literal of the same regex remains in the scan loop section.

**The single-source-of-truth principle:** One variable definition governs the detector behavior for BOTH controls and the scan. "Controls certify the scan" is only true when controls and scan reference the same expression. Structural duplication breaks this guarantee even when the values are momentarily equal.

**Complementary lessons:**
- L-BB-regex-arm-must-have-positive-control (D-735): every regex alternation arm MUST have a dedicated positive control.
- L-BB-sibling-sweep-same-contract-clause (D-735): when reconciling one detector to a contract clause, sibling-sweep all detectors governed by the same clause.
- L-BB-per-arm-control-sibling-detector-sweep (D-736): when adding per-arm controls to one detector, sibling-sweep all structurally-identical detectors.
- This lesson adds: once control assertions use a regex variable, the scan loop MUST also use that variable — no inline literal duplication allowed.

**Cites:** D-737, S-18.12 LOCAL adv pass-13 F-P13-001, POLICY 11, test-writer b252c8fe (variable extraction: one regex variable per detector, referenced in both controls and scan loop; all regex STRING values byte-identical; 68/68 GREEN).

## L-BB-presence-oracle-must-strip-comment-lines

**Date:** 2026-06-30 (D-738)
**Tags:** [codified] [process-gap] [test-writer] [adversary] [POLICY 11] [TD-VSDD-059]
**Anchors:** D-738, S-18.12 LOCAL adv pass-14 F-P14-001, POLICY 11, POLICY 13, TD-VSDD-059

**Lesson (codified):** A guard-present or anti-paper-fix presence check that greps WITHOUT first stripping comment lines is defeated by commenting the guard out. A Bash script containing only `# [[ ${BASH_VERSINFO[0]} -ge 4 ]]` will pass a naive `grep -E "$guard_re"` oracle as if the guard were present — the comment line matches the pattern, producing a false-PASS. The presence oracle MUST operate on executable (non-comment) lines only.

**Required mechanism:** Apply `grep -vE '^[[:space:]]*#'` to strip comment lines from the content BEFORE applying the guard regex match:

```bash
executable_content=$(grep -vE '^[[:space:]]*#' "$script_file")
run bash -c "echo \"$executable_content\" | grep -qE \"$guard_re\""
```

**Mandatory negative controls:** Every guard-presence oracle MUST include at least one negative control verifying that a commented-out guard is REJECTED:

- `pc_commented_guard` — content with ONLY a commented-out guard → assert oracle FAILS (non-zero exit)
- `pc_commented_guard_ac002` (if AC-002 also has a guard oracle) — same for sibling AC

**Root cause pattern observed in S-18.12 pass-14:** After 13 passes of hardening, the `guard_re` oracle in AC-001 and AC-002 remained a naive `grep -E "$guard_re"` applied to the full file content including comment lines. The D-734 O-4 "paper-guard" observation (passes 10/11/12/13) noted the soundness boundary in prose but did not add the negative control that would have mechanically enforced the stripping requirement. At pass-14, the fresh-context adversary identified this as a testable defect (not just a prospective boundary), escalated O-4 to HIGH F-P14-001, and the fix — adding `grep -vE '^[[:space:]]*#'` pre-filter plus two negative controls — closed the gap definitively.

**Relationship to TD-VSDD-059 (paper-fix detection):** Commenting out a guard IS a paper-fix pattern. An oracle that accepts commented guards cannot detect paper-fixes of the guard-present class. This lesson extends TD-VSDD-059 to the comment-line dimension: paper-fix detection requires the oracle to operate on executable lines only.

**Prevention gate (codified):**
1. Every guard-presence test MUST use `grep -vE '^[[:space:]]*#'` to strip comments before the guard_re match.
2. Sibling AC guard oracles MUST receive the same treatment in the same burst (sibling-sweep obligation).
3. At test-writer dispatch: verify `pc_commented_guard` and `pc_commented_guard_ac002` negative controls exist and assert non-zero exit on commented-only content.

**Complementary lessons:**
- L-BB-scan-loop-and-controls-must-share-regex-variable (D-737): controls and scan loop MUST share the same regex variable.
- TD-VSDD-059: paper-fix detection — fixes that only appear structural but leave the defect present.
- D-734 O-4: the commented-guard soundness boundary was documented as LOW/prospective for 4 passes; this lesson shows that "soundness boundary documented in prose" is insufficient — a negative control is required to mechanically enforce the boundary.

**Cites:** D-738, S-18.12 LOCAL adv pass-14 F-P14-001, POLICY 11/13, TD-VSDD-059, test-writer 00272990 (comment-strip applied to guard_re oracle in AC-001 and AC-002; `pc_commented_guard` and `pc_commented_guard_ac002` negative controls added; 68/68 bats GREEN).

---

## L-BB-unknown-provenance-wasm-build-artifact

**Context:** D-747 RC22-SMOKE-COMPLETE (2026-07-01). During rc.22 pre-release smoke testing, a `plugins/vsdd-factory/hook-plugins/validate-state-structure.wasm` file (206,070 B) was found sitting dirty in the working tree. It matched neither any committed version nor a fresh current-source build (which produces 220,640 B on darwin-arm64 rustc 1.95.0 from HEAD). The artifact had been present for an unknown period — likely an intermediate ad-hoc `cargo build` invocation at some earlier session.

**Root cause:** The `hook-plugins/*.wasm` files are tracked in git but also regularly regenerated by `cargo build` as part of development and testing workflows. There is no current gate that catches a working-tree WASM whose size or hash diverges from either (a) the committed version or (b) a fresh current-source build. Such artifacts can quietly persist across many sessions without being noticed until a systematic release-prep sweep.

**Why this matters:** An unknown-provenance WASM binary in the working tree creates risk if accidentally committed. While `release.yml` overwrites all WASM files at tag time (so the artifact would be replaced during an actual release), a pre-release commit containing the artifact would introduce an unverified binary into the tracked repo history. Additionally, the size discrepancy between the dirty file and a fresh build (206,070 B vs 220,640 B) is diagnostic: it indicates the binary was built from an intermediate source state — not from the current HEAD — which means it could exhibit different behavior from what the current source produces.

**Diagnostic signal:** Size discrepancy between the dirty `.wasm` file and a fresh same-HEAD build is the primary detection mechanism. When `git status` shows a modified `.wasm` under `hook-plugins/`, always check: does `ls -la <file>` size match what a fresh `cargo build -p <plugin-crate> --target wasm32-wasip1` would produce?

**Prevention options (not yet implemented — POST-E-18 gate-story candidate):**
1. **Pre-commit gate**: before any commit touching `hook-plugins/`, verify all modified `.wasm` files match their expected current-source build size/hash.
2. **Release-prep standard practice**: add `git restore plugins/vsdd-factory/hook-plugins/*.wasm` as the FIRST step of any release-prep workflow (before `wc -l` size checks, before smoke test matrix), ensuring the working tree starts from committed binaries.
3. **CI-side enforcement**: CI already uses fresh builds from `release.yml` at release time, but a pre-commit check on develop could catch this class earlier.

**Anchor:** POST-E-18 gate-story candidate (SS-04/S-18.08-class gate or standalone; concrete future dependency: POST-E-18 revisit authorization). Justified deferral per Canonical Principle Rule 3. Not blocking rc.22 (release.yml overwrites at tag time).

**Cites:** D-747, RC22-SMOKE-COMPLETE, validate-state-structure.wasm 206,070 B (dirty) vs 220,640 B (fresh darwin-arm64 rustc 1.95.0 HEAD build).

---

## L-BB-orphan-status-requires-dual-registry-check

**Context:** D-751 NEW [process-gap] [codified]. During rc.22 post-install smoke, story-writer authored S-19.04 v1.0 with AC-002 speccing `vsdd-context-resolvers.wasm` for bundle exclusion as an orphan — checking only `hooks-registry.toml` and finding it unreferenced there. Orchestrator cross-checked smoke Leg 1 and Leg 3 evidence and caught that `vsdd-context-resolvers.wasm` IS referenced by `resolvers-registry.toml` line 15 as the live `wave_context` resolver. AC-002 would have deleted an active production WASM, breaking the context-resolver system. Story-writer amended to v1.1 pre-commit: added keep-assertion for resolvers-registry-referenced WASMs, dual-registry orphan rule, and AC-006 regression bats test.

**Rule:** orphan-WASM determination MUST check BOTH `hooks-registry.toml` AND `resolvers-registry.toml` — a WASM referenced only by `resolvers-registry.toml` is live, not orphan.

**Root cause:** story-writer checked only `hooks-registry.toml` when classifying WASMs for Leg 1 orphan list; `resolvers-registry.toml` was only visible in Leg 3 evidence. The two registries form a union: a WASM is orphan only if absent from BOTH.

**Prevention:**
1. Any orphan sweep MUST union both registries: `grep -h 'wasm' hooks-registry.toml resolvers-registry.toml | grep -oE '[^/]+\.wasm' | sort -u` → referenced set; any WASM not in this set is truly orphan.
2. Regression gate: S-19.04 AC-006 `bundle-orphan-detection.bats` must assert the dual-registry union logic.
3. Sibling-class: `L-F2-deferred-table-semantics` (verify perimeter semantics before classifying) — same "check the actual semantics of the catalog" discipline applied to registry union rather than table column headings.

**Cites:** D-751, S-19.04 v1.0 AC-002 defect → v1.1 correction, smoke rc22-post-install-smoke.md Leg 1 F-1 / Leg 3 F2, STORY-INDEX v4.128→v4.129.

---

## L-BB-oversized-artifact-surgical-edit-protocol

**Context:** D-752 NEW [process-gap] [codified]. During the E-19 pass-1 fix burst (2026-07-06), architect agents stalled 3 consecutive times on whole-file Read/Write operations against ADR-025 (~120KB). The 4th dispatch used a surgical protocol — Bash grep structure-recon first, targeted Read with offset/limit (~120 lines), small anchored Edits with unique old_string — and succeeded on the first try.

**Rule:** Agents mutating oversized artifacts (>=100KB) MUST use the surgical protocol:
1. Bash grep to locate the target section (structure-recon).
2. Targeted Read with offset/limit to fetch only the affected lines (never whole-file Read on a >=100KB file).
3. Small anchored Edit with a unique old_string (never whole-file Write via /tmp staging copies of .factory files).
4. NEVER whole-file Read/Write on files >=100KB in `.factory/`.

**Root cause:** Three consecutive agent API stalls on whole-file ADR-025 operations (~120KB) during E-19 pass-1 fix burst. The 4th attempt with surgical protocol succeeded first try.

**Related:** D-442(e) WASM fuel exhaustion on the same size class (PostToolUse validators time out fail-closed-advisory on >100KB files); L-BB-unknown-provenance-wasm-build-artifact (same size-class issue class).

**Prevention:** Orchestrator dispatch briefs for large-file mutations MUST mandate the surgical protocol explicitly. Threshold: any `.factory/` artifact >=100KB.

**Cites:** D-752, E-19 pass-1 fix burst, ADR-025 v1.6→v1.7 architect leg, L-BB-unknown-provenance-wasm-build-artifact.

---

## L-BB-parallel-spec-authorship-requires-cross-reconciliation-sweep

**Context:** D-754 NEW [process-gap] [codified]. E-19 pass-3 adversary found that the pass-2 fix burst authored ADR-025 Decision 15 and BC-1.17.001 in the same window (same-burst, separate specialist legs) and produced contradictory signatures AND contradictory capability models. Additionally, verification-architecture.md rows were left with placeholder titles disagreeing with VP-INDEX after VP-094..VP-101 were authored — POLICY 9 propagation gap from the same burst. 5 of pass-3's HIGH findings were burst-self-inflicted (F-P3-001, F-P3-002, F-P3-003, F-P3-004, F-P3-005). A companion FALSE PARITY ATTESTATION was caught in the pass-3 fix burst itself: architect grepped verification-architecture.md against itself (not against VP-INDEX) and attested "identical to VP-INDEX" — a tautological comparison that catches nothing. Orchestrator adjudicated with independent cross-file greps; redo verified 8/8 byte-match.

**Rule:** When one fix burst authors ≥2 interdependent spec artifacts via separate specialist legs (e.g., ADR amendment + new BC governing the same mechanism), the burst MUST end with a mechanical cross-reconciliation sweep BEFORE the state-manager closure:
1. Grep signature/capability/schema/title claims across the artifact pair and assert byte-match on shared tokens.
2. POLICY 9 propagation check: for every VP created or amended in the burst, confirm verification-architecture.md and verification-coverage-matrix.md rows carry the canonical title (grep VP-INDEX vs verification-architecture.md — NOT verification-architecture.md vs itself).
3. POLICY 9 reciprocal check: for every BC that governs a VP created in the burst, confirm the BC's `verification_properties:` frontmatter is updated.

**Root cause:** E-19 pass-2 burst authored ADR-025 Decision 15 (architect leg) and BC-1.17.001 (product-owner leg) in the same window with contradictory signatures AND capability models, plus verification-architecture.md placeholder rows and VP-TBD reciprocals — 5 of pass-3's HIGH findings were burst-self-inflicted.

**Prevention:** Orchestrator adds a reconciliation verification step to every multi-artifact spec burst. The step is a literal-shell mechanical check, not a narrative attestation.

**Cites:** D-754, adv-E19-pass-3.md F-P3-001..F-P3-005, POLICY 9, L-BB-oversized-artifact-surgical-edit-protocol (companion discipline for large-file edits in same burst).

---

## L-BB-finding-premise-must-be-verified-before-fix

**Context:** D-754 NEW [process-gap] [codified]. During the E-19 pass-3 fix burst (2026-07-06), adversary finding F-P3-019 claimed that `ci.yml` lacks a job named `bats-full-suite`. Story-writer received the finding and executed a `replace_all` renaming the story's own EC-003 CI deliverable from `bats-darwin-leg-macos` to `bats-wave-handoff-macos` — matching what the adversary claimed the job should be called. Orchestrator ground-truth grep of `.github/workflows/ci.yml` confirmed `bats-full-suite` EXISTS in the job list. The adversary premise was factually wrong. The story-writer fix was destructive — it changed a correct specification to match a false adversary claim. Reverted at S-19.01 v1.4 after orchestrator verification catch.

**Rule:** An adversary finding's factual premise MUST be mechanically verified (one literal grep against ground truth) by the fix-executor BEFORE editing:
1. If the finding claims "X does not exist," grep the canonical source for X before removing or renaming anything.
2. If the finding claims "Y is missing from Z," read Z and confirm Y is actually absent before adding it.
3. Corollary: `replace_all` on tokens that overlap an artifact's own deliverable names is a blast-radius hazard — enumerate all hits before replacing.

**Sibling-class:** TD-VSDD-059 (adversary false parity attestation in the same burst: architect grepped the wrong file and attested VP-INDEX parity; orchestrator independent-grep adjudication is the working control for this class). Both are false-premise-execution variants: one from the adversary, one from a fix-executor.

**Root cause:** F-P3-019 claimed ci.yml lacks bats-full-suite (false — it exists). Story-writer executed the false premise via `replace_all` that renamed the story's own deliverable CI job (bats-darwin-leg-macos → bats-wave-handoff-macos), reverted at S-19.01 v1.4 after orchestrator ground-truth catch.

**Prevention:**
1. Every fix-burst must include a premise-verification step (one literal grep) before executing any finding that claims the absence or misname of a token in a canonical file.
2. The orchestrator's final verification pass for each specialist leg MUST independently grep the claimed ground truth, not trust the specialist's narrative attestation.
3. `replace_all` scope must be explicitly enumerated before execution on any token that could appear in the artifact's own deliverable names.

**Cites:** D-754, adv-E19-pass-3.md F-P3-019, S-19.01 v1.3→v1.4 revert, L-BB-parallel-spec-authorship-requires-cross-reconciliation-sweep (companion), TD-VSDD-059 (sibling-class).

## L-BB-bc-cite-preflight-must-cover-story-index-prose [codified]

**Context:** D-768 NEW [process-gap] [codified] 2026-07-08. During E-19 adv pass-16, adversary O-P16-03 identified that pass-15 preflight §6 declared "BC-cite preflight PASS — zero stale live citations" while STORY-INDEX.md lines 685 and 701 already contained three stale BC version cites (BC-1.17.001 v1.1 and BC-4.13.001 v1.7, both multiple BC bumps behind current). The preflight scanned per-story/per-epic files in a per-file loop (D-759/D-760) but did NOT scan STORY-INDEX delivery-summary items and BC coverage blocks. The false PASS result persisted across passes 4 through 15 (11 passes) while the stale cites accumulated.

**Rule:** The BC-cite drift preflight (D-759 two-sided; D-760 per-file-loop canonical) MUST additionally execute a **STORY-INDEX-prose leg** scanning all non-exempt STORY-INDEX prose (delivery-summary items, BC coverage blocks, epic header blocks) against current BC frontmatter versions. The STORY-INDEX is the POLICY 14 upstream-index leg for all stories; a stale BC version citation there is a parity-leg failure. This leg is mandatory in both:
1. Story-writer end-of-leg scan (after each SW specialist leg completes).
2. Orchestrator pre-dispatch scan (before each adversary dispatch is issued).

**Canonical execution form (per D-760; per D-449(a) literal-shell required):**
```bash
# STORY-INDEX prose leg — scan non-historical live BC cites
grep -nE "BC-4\.13\.001 v1\.[0-7][^0-9]|BC-1\.17\.001 v1\.[01][^0-9]" \
  .factory/stories/STORY-INDEX.md | \
  grep -v "\[prior\|\[Prior:\|Changelog\|last_amended\|modified\[\]" | \
  grep . && echo "STALE CITES FOUND" || echo "STORY-INDEX-PROSE-PASS"
```
Zero matching lines + "STORY-INDEX-PROSE-PASS" = PASS. Any matching line = FAIL requiring STORY-INDEX update before dispatch.

**Root cause:** The per-file BC-cite preflight loop was defined to scan `S-19.01..S-19.07 + E-19 epic` — the individual story and epic files. STORY-INDEX itself was not in the scan set, even though STORY-INDEX contains live BC version cites in its prose blocks (delivery-summary items at the top of each epic section; BC coverage summary block). Because STORY-INDEX is the POLICY 14 upstream-index, its BC cites must be kept current with the same discipline as story-file cites.

**Sibling-class:** D-759 (per-file preflight instituted); D-760 (cross-file awk forbidden; per-file loop canonical). This lesson extends the scan scope without changing the per-file loop form.

**Prevention:**
1. Add "STORY-INDEX prose" to the mandatory preflight checklist at both story-writer and orchestrator scan points.
2. Whenever a BC version is bumped (PO leg), immediately scan STORY-INDEX prose for stale cites in the same burst (not deferred to next adversary pass).
3. The three specific STORY-INDEX prose blocks to scan: (a) epic description block (delivery-summary items naming BCs), (b) BC coverage block at end of epic section, (c) epic history/audit-trail blocks if they contain live version cites.

**Cites:** D-768, adv-E19-pass-16.md O-P16-03, F-P16-001, L-BB-bc-cite-preflight-instituted (D-759 companion).
