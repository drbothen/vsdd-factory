---
document_type: architecture-decision-record
level: L3
adr_id: ADR-030
version: "1.0"
title: "ADR-030: pr-manager merge-operation integrity enforcement"
status: accepted
date: 2026-07-06
producer: architect
timestamp: 2026-07-06T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-025 (single-writer factory lock/lease — factory-artifacts write discipline; SubagentStop hook pattern precedent)
  - ADR-017 (per-story adversarial convergence gate — three-perimeter model; shared pr-manager SubagentStop scope)
  - ADR-014 (Tier-2 native WASM migration — standing mandate: new hooks MUST be native WASM)
anchors:
  - SS-05
subsystems_affected:
  - SS-04
  - SS-05
  - SS-07
  - SS-10
last_amended: "2026-07-06 (v1.0) — initial authorship (E-19 adv-P3 F-P3-015 close-out: no existing ADR covers pr-manager merge-operation integrity domain; D-749, D-750, F-P2-002 transcribed; three-component enforcement architecture decided)."
---

# ADR-030: pr-manager merge-operation integrity enforcement

## Context

Two independent failure modes in the pr-manager pipeline agent were identified and
recorded as D-749 (L-BB-merge-race-ready-report-stale-head) and D-750
(L-BB-release-pr-squash-merge-not-mechanically-enforced):

**Stale-HEAD READY verdict race (D-749).** The pr-manager emits a structured READY
verdict that includes a `covered_sha` field pinning the exact commit SHA the verdict
was reviewed against. Between the time the pr-manager emits the READY verdict and
the time the orchestrator acts on it (invokes `gh pr merge`), additional commits may
be pushed to the PR branch by other agents or by CI automation. If the orchestrator
acts on a stale verdict, it may merge a HEAD that was not reviewed. The READY verdict's
`covered_sha` field exists precisely to detect this, but no mechanical gate enforced
that (a) every READY verdict includes `covered_sha`, and (b) the orchestrator verified
`covered_sha` against live PR HEAD before merging.

**Release-PR squash-merge not mechanically enforced (D-750).** `RELEASING.md` and the
project git-workflow mandate that release-branch PRs (`release/v*`) MUST be merged with
`--merge` (not `--squash` or `--rebase`) to preserve develop's commits as ancestors of
main. The `release-branch-guardrail.yml` workflow enforces that release branches target
`main` and are named `release/v<semver>`, but it operates on `pull_request` events which
carry no `merge_method` field — the merge strategy cannot be enforced at the GitHub
Actions layer. The only enforcement path is at the tool layer: the `gh pr merge`
invocation itself.

**E-19 adversarial F-P2-002** found that LLM prompt-spec instructions ("use --merge for
release PRs") are unenforceable by bats tests — the mechanical layer is required because:
(a) prompt instructions can be silently overridden by context length limits; (b) bats
tests validate CLI tool behavior, not orchestrator compliance with prose instructions;
and (c) a test that asserts the `--merge` flag was passed cannot intercept an
in-production orchestrator invocation that omitted it.

**F-P3-015 ruling.** No existing ADR (ADR-001 through ADR-029) covers the pr-manager
merge-operation integrity domain. `release-branch-guardrail.yml` was adjudicated NOT
extendable because GitHub Actions `pull_request` events carry no `merge_method` field.
ADR-030 is the first architectural decision for this domain.

**Normative twin:** BC-5.42.001 governs the pr-manager READY-verdict, stale-verdict,
and merge-strategy behaviors at the behavioral contract level. ADR-030 governs the
mechanical enforcement architecture that makes BC-5.42.001 structurally guaranteed
rather than prompt-dependent.

## Decision

Three components form the enforcement architecture. Together they close both failure
modes (D-749 + D-750) mechanically.

### Decision 1: pr-manager-completion-guard.wasm — SubagentStop READY-verdict completeness gate

A new native WASM hook plugin `pr-manager-completion-guard` is added as a SubagentStop
hook per ADR-014 standing mandate (new hooks MUST be native WASM). It fires on
SubagentStop events for Agent tool calls invoking the pr-manager agent.

**Trigger:** SubagentStop event, `agent_name = "vsdd-factory:pr-manager"`.

**Behavior:** The plugin reads the SubagentStop payload and looks for a READY verdict in
the agent's output. If a READY verdict is present but lacks a `covered_sha` field (empty
string, null, or absent), the plugin emits an advisory block with error code
`READY_SHA_MISSING`. The block is advisory (`on_error = "advisory"`): it does not prevent
the SubagentStop from completing, but it surfaces the missing-SHA condition as a
telemetry event so the orchestrator can detect the gap before acting on the verdict. If
no READY verdict is present, the plugin returns Continue (exit 0). Non-READY verdicts
are not inspected.

**Canonical registry TOML:**

```toml
[hook.pr-manager-completion-guard]
plugin = "hook-plugins/pr-manager-completion-guard.wasm"
event = "SubagentStop"
tier = "sync"
on_error = "advisory"
priority = 150
tool = "^Agent"
```

### Decision 2: bin/check-stale-verdict.sh — orchestrator-invocable stale-verdict detector

A new shell script `plugins/vsdd-factory/bin/check-stale-verdict.sh` is added as an
SS-10 CLI bin tool. It is NOT a hook plugin — it must be explicitly invoked by the
orchestrator before acting on any READY verdict. The design is a bin tool rather than
a hook because `covered_sha` comes from the pr-manager's output verdict, not from a
structured SubagentStop payload field accessible to a WASM gate at hook time.

**Invocation:** `check-stale-verdict.sh --pr <PR_NUMBER_OR_URL> --covered-sha <SHA>`

**Behavior:** Calls `gh pr view <PR_NUMBER_OR_URL> --json headRefOid`, compares live
HEAD SHA against `<covered_sha>`. Match: exit 0. Mismatch: prints
`STALE_READY_VERDICT: covered_sha=<SHA> live_head=<LIVE_SHA>` to stderr, exits 2.
All error paths (gh unavailable, PR not found, missing arg, JSON parse failure) exit 2
with a `CHECK_STALE_VERDICT_ERROR:` prefix (fail-closed: treat as stale).

**Orchestrator obligation:** The orchestrator MUST invoke `check-stale-verdict.sh` and
verify exit 0 before invoking `gh pr merge` for any PR with a READY verdict. Merging
without this check is a BC-5.42.001 violation.

### Decision 3: bin/enforce-merge-strategy.sh — release-PR merge-strategy enforcement wrapper

A new shell script `plugins/vsdd-factory/bin/enforce-merge-strategy.sh` is added as an
SS-10 CLI bin tool. It wraps `gh pr merge` and enforces `--merge` for release-branch PRs.

**Invocation:** `enforce-merge-strategy.sh --pr <PR_NUMBER_OR_URL> [--merge|--squash|--rebase] [--auto] [--delete-branch]`

**Behavior:**
1. Calls `gh pr view <PR_NUMBER_OR_URL> --json headRefName` to obtain the branch name.
2. If branch matches `^release/v`: if `--squash` or `--rebase` was passed, prints
   `RELEASE_PR_SQUASH_FORBIDDEN: PR branch=<BRANCH> requires --merge; --squash/--rebase are forbidden for release PRs (RELEASING.md + BC-5.42.001)`
   and exits 2. Otherwise forces `--merge` regardless of flags passed.
3. If branch does NOT match `^release/v`: passes strategy flags through unchanged.
4. Invokes `gh pr merge` with final flags; propagates gh's exit code.

Branch name resolution failure is fail-open (treat as non-release; the release guard
requires affirmative `^release/v` match — a resolution failure cannot bypass release
enforcement by causing a false non-match, because release enforcement only fires on
positive match).

### Decision 4: Non-extendability of release-branch-guardrail.yml

`release-branch-guardrail.yml` operates on `pull_request` events which do not carry
`merge_method`. The post-merge `closed` event carries `merged: true` but enforcement
at that point is too late. Disabling squash globally via repository settings would
restrict non-release PRs, which is not the intent. Therefore the GitHub Actions layer
is NOT extendable for merge-strategy enforcement; the bin-tool layer (Decision 3) is
the correct enforcement site.

## Rationale

**Why advisory (not blocking) for the SubagentStop READY-SHA gate (Decision 1)?**
A blocking SubagentStop gate would prevent the pr-manager result from being returned
to the orchestrator entirely, which is worse than returning the verdict with a missing
SHA — the orchestrator can still read the output, notice the advisory telemetry, and
refuse to act. An advisory gate surfaces the defect without destroying the result.
BC-5.42.001 specifies READY_SHA_MISSING as advisory, and ADR-030 Decision 1 aligns.

**Why a bin tool rather than a hook for stale-verdict detection (Decision 2)?**
A SubagentStop hook fires immediately when the pr-manager agent returns. At that moment
the orchestrator has not yet decided which PR to merge. The `covered_sha` from the READY
verdict would need to be propagated through the hook payload, which would require a
schema extension to HookPayload (HOST_ABI_VERSION bump) for a check that is better
expressed as an explicit pre-merge orchestrator step. The bin-tool pattern is the
established precedent for operations the orchestrator must invoke explicitly
(see `factory-lock-write.sh`, `factory-cas-push.sh`).

**Why at the `gh pr merge` wrapper layer for merge-strategy enforcement (Decision 3)?**
The merge-strategy is known at tool-call time when the orchestrator invokes `gh pr merge`.
All other enforcement sites (GitHub Actions, hooks-registry.toml, prompt instructions)
either observe post-merge (too late), lack `merge_method` in the event payload, or are
LLM prompt-dependent (F-P2-002 finding). The wrapper layer is the only site where
enforcement is both pre-merge and merge-method-aware.

**Why three separate components rather than one?**
Each component closes a distinct failure mode at the correct layer: (a) SubagentStop
WASM gate closes the "verdict lacks covered_sha" structural defect; (b) bin stale-verdict
check closes the race between verdict issuance and merge invocation; (c) bin
merge-strategy wrapper closes the release-PR squash-merge risk. Combining them would
couple unrelated invariants and complicate independent testability.

## Consequences

### Positive

- `covered_sha` completeness is mechanically verified at SubagentStop time, not by
  prompt instruction.
- Stale-verdict race is detected before merge, not discovered post-merge.
- Release-PR `--merge` enforcement is structural: a squash-merge of a release PR is now
  architecturally impossible without bypassing `enforce-merge-strategy.sh`.
- Three components are independently testable: WASM gate via bats + SubagentStop
  fixture; stale-verdict detector via bats + `gh` mock; merge-strategy enforcer via
  bats + `gh` mock.
- `release-branch-guardrail.yml` remains as a complementary layer (enforces branch
  naming and merge target); the two enforcement layers are orthogonal.

### Negative / Trade-offs

- Orchestrator must invoke `check-stale-verdict.sh` explicitly — it is not a hook.
  This is a caller obligation, not an automatic guard.
- `enforce-merge-strategy.sh` requires one additional `gh pr view` call before the
  merge call (~500ms p50 additional latency per merge invocation).
- Residual stale-verdict window: a push could arrive between `check-stale-verdict.sh`
  and the `gh pr merge` call. The window is accepted: it is deterministically bounded
  (execution time of the two scripts) rather than unbounded.

### Status as of 2026-07-06

Accepted as architecture. Pending S-19.01 implementation. No production code exists yet;
bats tests and WASM crate are S-19.01 deliverables.

## Alternatives Considered

- **Option A — GitHub Actions merge-method enforcement:** Configure repository settings
  to disable squash and rebase globally. Rejected because this would restrict non-release
  PRs, which legitimately use squash-merge. A per-branch GitHub ruleset with merge
  strategy enforcement requires GitHub Enterprise. Not available in the current
  repository plan.

- **Option B — Blocking SubagentStop gate for stale-verdict check:** Extend the
  SubagentStop WASM gate (Decision 1) to also call `gh pr view` and compare `covered_sha`
  directly. Rejected because (a) this requires `host::exec_subprocess` with `gh` in
  `binary_allow` inside the SubagentStop WASM plugin — a broad capability grant;
  (b) it couples SHA-pinning to the SubagentStop event, which fires before the
  orchestrator has decided whether to merge; and (c) SubagentStop with `on_error=block`
  would destroy the pr-manager output on any gh auth failure, which is worse than
  fail-open advisory.

- **Option C — Extend hooks-registry.toml with a pre-merge bash hook:** Add a new bash
  hook on `tool = "Bash"` with `command =~ "gh pr merge"` to intercept and validate
  the merge invocation. Rejected because hooks-registry.toml bash trigger pattern
  matching on command content is fragile (any `gh pr merge` mention in a bash command
  fires the hook, including in echo/comment contexts). The bin-tool wrapper
  (Decision 3) is explicit and controllable.

- **Option D — Prompt-engineering only:** Rely on orchestrator prompt instructions to
  always run `check-stale-verdict.sh` and use `--merge`. Rejected by F-P2-002 (E-19
  adversarial finding): LLM prompt-spec is unenforceable by bats tests; structural
  enforcement is the production-grade path.

## Source / Origin

- **Decision log D-749:** L-BB-merge-race-ready-report-stale-head. Records the
  stale-HEAD race condition observed during bb-ingest brownfield work.
- **Decision log D-750:** L-BB-release-pr-squash-merge-not-mechanically-enforced.
  Records the release-PR squash-merge risk discovered during the same session.
- **E-19 adversarial pass-2 finding F-P2-002:** LLM prompt-spec unenforceable by bats;
  mechanical enforcement layer required.
- **F-P3-015 ruling (E-19 adv-P3):** No ADR-001..ADR-029 covers this domain; ADR-030
  is the first architectural decision; PO updates BC-5.42.001 §Traceability after
  ADR-030 is authored.
- **BC-5.42.001:** Normative twin behavioral contract; `ss-05/BC-5.42.001.md`.
- **RELEASING.md:** Canonical release procedure mandating `--merge` for release PRs.
- **`.github/workflows/release-branch-guardrail.yml`:** Adjudicated NOT extendable for
  merge-method enforcement (Decision 4 analysis).

## ARCH-INDEX subsystem

SS-05 (Pipeline Orchestration) — pr-manager agent domain; BC-5.42.001 behavioral
contract. SS-04 (Plugin Ecosystem) — `pr-manager-completion-guard.wasm` WASM plugin
crate (`crates/hook-plugins/pr-manager-completion-guard/`). SS-07 (Hook Bash Layer) —
hooks-registry.toml SubagentStop entry for `pr-manager-completion-guard`. SS-10 (CLI
Tools and Bin) — `check-stale-verdict.sh` and `enforce-merge-strategy.sh` bin tools
(`plugins/vsdd-factory/bin/`).

## Traceability

| Artifact | Version | Role |
|----------|---------|------|
| BC-5.42.001 | — | Normative twin; behavioral contract for pr-manager READY-verdict, stale-verdict, and merge-strategy |
| ADR-025 | — | Factory-artifacts single-writer discipline; SubagentStop hook pattern precedent |
| ADR-017 | — | Per-story adversarial convergence gate; pr-manager three-perimeter model |
| ADR-014 | — | Tier-2 native WASM migration; standing mandate for native WASM new hooks |
| S-19.01 | — | Implementation story for all three components |
| CAP-033 | — | Capability allocation for pr-manager merge-operation integrity |
| D-749 | — | Decision log: L-BB-merge-race-ready-report-stale-head |
| D-750 | — | Decision log: L-BB-release-pr-squash-merge-not-mechanically-enforced |
| F-P2-002 | — | E-19 adversary finding: LLM prompt-spec unenforceable by bats; mechanical layer required |
| ARCH-INDEX | v2.89 | Registration of ADR-030 |
