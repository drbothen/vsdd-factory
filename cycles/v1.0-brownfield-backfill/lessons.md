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
