---
document_type: behavioral-contract
level: L3
version: "1.8"
status: active
producer: product-owner
timestamp: 2026-07-13T14:49:50Z
last_amended: "(v1.8) — POL-14 auto-promotion: lifecycle_status draft→active on PR #613 squash-merge 8d1721f7 (S-19.01 MERGED 2026-07-13T14:49:50Z); BC-INDEX v3.97→v3.98; D-833. [Prior: (v1.7) — S-19.01 F-P7-001 (product-owner): ADR-030 §Decision 3 governed-pass-through wrapper contract mirror (human-directed): §Description (c) expanded with sole-gateway obligation, positional strategy $2, residual ${@:3} governed pass-through, deny-list STRATEGY_SMUGGLING_FORBIDDEN, best-effort-delete note; Invariants 6–7 added; EC-009 added; Canonical Test Vectors 2 new rows. [Prior: (v1.6) — orchestrator pre-pass-43 consistency sweep (product-owner): §Verification Properties VP-094 row-3 proof-method drift fixed — `unit (WASM test harness; S-19.01)` → `integration (bats; S-19.01)` per VP-INDEX v2.56 canonical proof method. [Prior: (v1.5) — E-19 pass-33 O-P33-001 (product-owner): §Traceability L2 Domain Invariants TBD → none (pipeline-orchestration operational) — sibling-convention alignment (BC-1.17.001/BC-4.13.001/BC-2.07.001 class); input-hash 509c8f8→4fd18a4 (within-burst hash refresh: S-19.01 v1.16 input drift — SW updated S-19.01 after PO computed hash; same-burst correction per D-782/D-783 precedent). [Prior: (v1.4) — E-19 pass-30 F-P30-002 (product-owner): input-hash placeholder retired per POLICY 18 (compute-input-hash --update; real digest); mechanical metadata fix; spec content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. [Prior: (v1.3) — E-19 pass-22 fix burst F-P22-001 BC leg (product-owner): §Architecture Anchors hooks/pr-manager-completion-guard.wasm → hook-plugins/pr-manager-completion-guard.wasm; ground-truth: hooks-registry.toml plugin field reads hook-plugins/pr-manager-completion-guard.wasm; hook-plugins/ dir confirms WASM present. Anchoring correction only; behavioral content unchanged. BC-INDEX bump state-manager same-burst. [Prior: (v1.2) — W1-validation fix burst F-W1V-001 (product-owner): §Architecture Anchors script paths hooks/ → bin/ per architect ADR-030 §Decision 2/3 adjudication (bin/ = orchestrator-invoked SS-10 CLI tools; hooks/ = dispatcher-fired). [Prior: (v1.1) — E-19 pass-3 PO finalization (product-owner): F-P3-004 §VP Anchors + §Verification Properties VP-TBD → VP-094; F-P3-015 §Traceability CAP-TBD → CAP-033, ADR-TBD → ADR-030. [Prior: (v1.0) — initial creation (product-owner): E-19 pass-2 fix burst Package 2 — pr-manager READY-verdict SHA pinning, stale-verdict detection script, and release-PR merge-strategy guard (story anchor S-19.01); closes L-BB-merge-race-ready-report-stale-head (D-749) + L-BB-release-pr-squash-merge-not-mechanically-enforced (D-750).]]]]]]]"
phase: F3
inputs:
  - .factory/stories/S-19.01-pr-manager-hardening.md
  - plugins/vsdd-factory/agents/pr-manager.md
  - .github/workflows/release.yml
input-hash: "7b8d199"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-033"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-E19
modified:
  - "2026-07-06 (v1.1)"
  - "2026-07-08 (v1.2)"
  - "2026-07-08 (v1.3)"
  - "2026-07-09 (v1.4)"
  - "2026-07-09 (v1.5)"
  - "2026-07-09 (v1.6)"
  - "2026-07-11 (v1.7)"
  - "2026-07-13 (v1.8)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.42.001: pr-manager READY-verdict + merge-strategy enforcement — covered_sha pin, stale-verdict detection, and release-PR squash prevention

## Description

Three mechanically-enforced behaviors close the window between a pr-reviewer's READY verdict and the orchestrator's `gh pr merge` call where the PR HEAD could advance, and close the separate window where a release PR could be accidentally squash-merged.

**(a) covered_sha requirement on READY verdicts.** Every pr-reviewer READY verdict emitted as the final SubagentStop message MUST carry a `covered_sha: <40-hex>` field recording the `gh pr view --json headRefOid` value at the moment of assessment. The `pr-manager-completion-guard` WASM hook (SubagentStop event) inspects the `last_assistant_message` field of the SubagentStop payload; if the message contains a READY verdict token but no `covered_sha:` 40-hex field, the hook emits an advisory block with code `READY_SHA_MISSING`. The READY verdict is not acted upon until a compliant re-emit is received.

**(b) Stale-verdict detection before merge (check-stale-verdict.sh).** Before the orchestrator calls `gh pr merge` on any READY verdict, it MUST invoke `check-stale-verdict.sh <pr_number> <covered_sha>`. The script calls `gh pr view <pr_number> --json headRefOid` and compares the returned SHA against `covered_sha`. If the SHAs differ (PR HEAD advanced since the verdict was issued), the script exits non-zero with diagnostic line `STALE_READY_VERDICT: PR #<pr_number> HEAD <current_sha> != covered_sha <covered_sha>` on stderr. The orchestrator MUST treat a non-zero exit as blocking: re-dispatch pr-reviewer for a fresh review of the new HEAD before any merge action.

**(c) Release-PR merge-strategy guard (enforce-merge-strategy.sh — governed pass-through wrapper).** The `enforce-merge-strategy.sh` script is the **sole gateway** for all `gh pr merge` invocations in the pr-manager operative flow; direct `gh pr merge` calls bypassing this wrapper are a protocol violation (ADR-030 §Decision 3). Invocation: `enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase] [residual-args…]`. The merge strategy is owned positionally as `$2`; residual arguments `"${@:3}"` (e.g., `--delete-branch`) are forwarded verbatim to `gh pr merge` after the resolved strategy flag, making the wrapper a faithful governed pass-through for non-strategy flags.

Before forwarding, the wrapper scans `"${@:3}"` against a residual-arg deny-list. Any argument matching a strategy override or privilege-escalation flag — `--squash`, `--merge`, `--rebase`, `--admin` in long form, `=`-fused form (e.g., `--squash=auto`), bare-short form (`-s`, `-m`, `-r`, `-A`), or combined short-flag clusters containing any of those characters — is rejected: the wrapper prints `STRATEGY_SMUGGLING_FORBIDDEN: residual arg <arg> is a strategy or admin flag` to stderr and exits 1 before any `gh` invocation. Non-strategy flags (e.g., `--delete-branch`, `-d`) pass through in residual position.

The release rule is unchanged: before delegating to `gh`, the script reads the PR's headRefName via `gh pr view <pr_number> --json headRefName`. If the branch matches `^release/v`, the strategy resolves to `--merge` regardless of `$2`; if `$2` is `--squash` or `--rebase`, the script exits non-zero with `RELEASE_PR_SQUASH_FORBIDDEN: branch <name> requires --merge per RELEASING.md`. Non-release-branch PRs pass `$2` unchanged. `--delete-branch` may be passed as a residual arg and is forwarded, but merge success is NOT gated on deletion success; see EC-009.

Root lessons codified by this BC: L-BB-merge-race-ready-report-stale-head (D-749) and L-BB-release-pr-squash-merge-not-mechanically-enforced (D-750).

## Preconditions

1. `gh` CLI is available on `$PATH` and authenticated to the target repository.
2. The PR identified by `pr_number` is open and accessible via the GitHub API.
3. For (a): The pr-manager agent is operating with the `pr-manager-completion-guard` WASM hook registered for SubagentStop events in `hooks-registry.toml`.
4. For (b): `covered_sha` is the value recorded in the most recent READY verdict; `pr_number` identifies the same PR. `check-stale-verdict.sh` is executable and present on `$PATH` or at its registered script path.
5. For (c): `enforce-merge-strategy.sh` is the sole script used by the orchestrator to execute `gh pr merge`; direct `gh pr merge` calls outside this wrapper are a protocol violation.

## Postconditions

1. **(a) covered_sha present on actionable READY verdicts**: No READY verdict without a valid `covered_sha: <40-hex>` field is forwarded to the orchestrator merge step. The `pr-manager-completion-guard` hook fires `READY_SHA_MISSING` before the verdict propagates, giving the pr-reviewer an opportunity to re-emit with the SHA.

2. **(b) Staleness gate enforced**: `check-stale-verdict.sh` exits 0 iff `gh pr view --json headRefOid` returns a SHA equal to `covered_sha` at invocation time. Any mismatch is a non-zero exit with `STALE_READY_VERDICT` diagnostic. The orchestrator re-dispatches pr-reviewer before proceeding.

3. **(c) Release-branch merge integrity**: Any `gh pr merge` call on a `release/v*` branch is guaranteed to use `--merge`. The squash path is mechanically impossible via `enforce-merge-strategy.sh` for release branches; the script returns `RELEASE_PR_SQUASH_FORBIDDEN` on any squash or rebase attempt before any GitHub API call is made.

## Invariants

1. **A READY verdict without covered_sha is never actionable.** The orchestrator's merge sequence requires both a valid READY verdict token AND a `covered_sha: <40-hex>` field. A verdict lacking the field is treated as incomplete; the orchestrator waits for re-emit.

2. **A stale verdict is never merged.** `check-stale-verdict.sh` MUST be invoked synchronously before every `gh pr merge` call on a READY verdict. A successful (exit-0) staleness check is a mandatory precondition for the merge action; skipping it is a protocol violation equivalent to acting on a stale verdict.

3. **Release-PR squash is mechanically impossible via the wrapper.** `enforce-merge-strategy.sh` exits non-zero before any `gh` invocation when given `--squash` or `--rebase` for a `release/v*` branch. The GitHub API merge call is never attempted in that path.

4. **Non-release-branch PRs are unaffected.** `enforce-merge-strategy.sh` applies merge-strategy enforcement only to branches matching `^release/v`. Feature branches, fix branches, and maintenance branches pass the caller-supplied merge flag through unchanged.

5. **covered_sha format: 40 hex characters, lowercase.** The `pr-manager-completion-guard` hook and `check-stale-verdict.sh` both validate that `covered_sha` is exactly 40 lowercase hex characters. A malformed value (wrong length, non-hex chars) is treated the same as absent: `READY_SHA_MISSING` or `STALE_READY_VERDICT` respectively.

6. **enforce-merge-strategy.sh is the sole gateway for all `gh pr merge` invocations.** The pr-manager operative flow MUST NOT invoke `gh pr merge` directly; all such calls go through `enforce-merge-strategy.sh`. Direct bypass is a protocol violation (ADR-030 §Decision 3). This structural constraint ensures merge-strategy enforcement is architecturally guaranteed rather than prompt-dependent.

7. **Strategy-smuggling via residual args is mechanically rejected.** The wrapper scans `"${@:3}"` against the residual-arg deny-list before any `gh` invocation. Any arg matching `--squash`, `--merge`, `--rebase`, or `--admin` in long, `=`-fused, bare-short (`-s`/`-m`/`-r`/`-A`), or combined-short-cluster form exits 1 with `STRATEGY_SMUGGLING_FORBIDDEN: residual arg <arg> is a strategy or admin flag`. Non-strategy flags (e.g., `--delete-branch`, `-d`) are permitted in residual position.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `gh pr view --json headRefOid` network failure in `check-stale-verdict.sh` | Script exits non-zero with `READY_SHA_FETCH_FAILED: gh pr view failed for PR #<pr_number>` on stderr; merge is blocked. Orchestrator may retry after transient delay; persistent failure requires human intervention. |
| EC-002 | `covered_sha` in READY verdict is 40 chars but not valid hex | `pr-manager-completion-guard` treats as malformed and emits `READY_SHA_MISSING` (same code as absent SHA). `check-stale-verdict.sh` also rejects on the same validation path. |
| EC-003 | PR is closed or merged between READY verdict and the `check-stale-verdict.sh` call | `gh pr view` returns a non-open PR state; the script exits non-zero with a diagnostic describing the PR state. Orchestrator logs and surfaces to human for resolution. |
| EC-004 | PR HEAD advanced (new commit pushed) between READY verdict and `check-stale-verdict.sh` | Standard stale-verdict path: non-zero exit with `STALE_READY_VERDICT` diagnostic; orchestrator re-dispatches pr-reviewer on the new HEAD. |
| EC-005 | `enforce-merge-strategy.sh` invoked with no merge-strategy flag on a release branch | Script defaults to `--merge` (the required strategy for release branches) and proceeds. Missing flag is not an error for release branches. |
| EC-006 | `enforce-merge-strategy.sh` invoked with `--merge` on a non-release branch | Script passes `--merge` through unchanged; no enforcement action needed. |
| EC-007 | pr-manager-completion-guard hook is invoked on a SubagentStop that is NOT a READY verdict | Hook takes no action (READY verdict detection is the trigger; other stop messages pass through). |
| EC-008 | READY verdict message contains multiple `covered_sha:` occurrences with conflicting values | Hook treats the first 40-hex value matching the `covered_sha:` pattern as authoritative. `check-stale-verdict.sh` requires the single value passed as an argument; ambiguity is the orchestrator's responsibility to resolve before invocation. |
| EC-009 | `--delete-branch` passed as a residual arg; `gh pr merge` reports success but remote branch deletion fails or emits a false-success "Deleted remote branch" stdout line | Merge success is NOT gated on branch deletion success. Per cli/cli #13380 (non-main worktree partial-failure: GitHub API merge succeeds but local base-checkout step fails) and cli/cli #12980 (false-success stdout regression), the "Deleted remote branch" stdout line MUST NOT be trusted as confirmation of deletion. Caller MUST verify the remote branch is gone by calling `gh api repos/{owner}/{repo}/git/refs/heads/{branch}` and expecting HTTP 404. If the branch persists (non-404), caller issues an explicit `gh api -X DELETE repos/{owner}/{repo}/git/refs/heads/{branch}` as a follow-up. The merged PR state is unaffected by deletion status. |

## Canonical Test Vectors

### check-stale-verdict.sh

| Input (pr_number, covered_sha, mocked headRefOid) | Expected stdout/stderr | Exit code |
|---------------------------------------------------|----------------------|-----------|
| pr=42, covered_sha=`abc123...` (40 hex), headRefOid=`abc123...` (same) | (silent) | 0 |
| pr=42, covered_sha=`abc123...` (40 hex), headRefOid=`def456...` (different) | `STALE_READY_VERDICT: PR #42 HEAD def456... != covered_sha abc123...` on stderr | non-zero |
| pr=42, covered_sha=`abc123...`, `gh` exits non-zero (network failure) | `READY_SHA_FETCH_FAILED: gh pr view failed for PR #42` on stderr | non-zero |
| pr=42, covered_sha=`ZZZZZZ` (invalid hex, <40 chars) | `READY_SHA_MISSING: covered_sha is malformed` on stderr | non-zero |

### enforce-merge-strategy.sh

| Input (pr_number, mocked headRefName, merge flag arg) | Expected action | Exit code |
|------------------------------------------------------|----------------|-----------|
| pr=10, headRefName=`release/v1.0.0-rc.23`, flag=`--merge` | Delegates `gh pr merge 10 --merge` | 0 (pass-through) |
| pr=10, headRefName=`release/v1.0.0-rc.23`, flag=`--squash` | Prints `RELEASE_PR_SQUASH_FORBIDDEN: branch release/v1.0.0-rc.23 requires --merge per RELEASING.md`; no gh call | non-zero |
| pr=10, headRefName=`release/v1.0.0-rc.23`, flag=`--rebase` | Prints `RELEASE_PR_SQUASH_FORBIDDEN: ...`; no gh call | non-zero |
| pr=10, headRefName=`feature/S-19.01`, flag=`--squash` | Delegates `gh pr merge 10 --squash` unchanged | 0 (pass-through) |
| pr=10, headRefName=`release/v1.0.0-rc.23`, no flag supplied | Defaults to `--merge`; delegates `gh pr merge 10 --merge` | 0 |
| pr=10, headRefName=`release/v1.0.0-rc.23`, flag=`--merge`, residual=`--delete-branch` | Deny-list passes (`--delete-branch` is non-strategy); delegates `gh pr merge 10 --merge --delete-branch` | 0 (pass-through) |
| pr=10, headRefName=`feature/S-19.01`, flag=`--squash`, residual=`--squash` | Deny-list rejects residual `--squash` before branch check; prints `STRATEGY_SMUGGLING_FORBIDDEN: residual arg --squash is a strategy or admin flag`; no gh call | non-zero |

## Related BCs

- BC-5.39.001 — per-story adversarial convergence loop (3-CLEAN protocol); this BC adds staleness enforcement as a prerequisite gate before the convergence-loop result is acted upon
- BC-5.41.001 — wave-gate; HANDOFF.md preconditions are complementary merge-integrity obligations
- BC-5.41.003 — PreCompact flush exemption; complementary WASM hook behavior in SubagentStop family

## Architecture Anchors

- `plugins/vsdd-factory/agents/pr-manager.md` — pr-manager agent prompt; MUST cite BC-5.42.001 PC1/PC2/PC3 as the covered_sha + stale-verdict + merge-strategy authority
- `plugins/vsdd-factory/hook-plugins/pr-manager-completion-guard.wasm` — SubagentStop hook enforcing (a); emits `READY_SHA_MISSING`
- `plugins/vsdd-factory/bin/check-stale-verdict.sh` — staleness detection script enforcing (b)
- `plugins/vsdd-factory/bin/enforce-merge-strategy.sh` — merge-strategy enforcement script enforcing (c)
- `RELEASING.md` — canonical authority for `--merge` requirement on release PRs (referenced in `RELEASE_PR_SQUASH_FORBIDDEN` diagnostic)

## Story Anchor

S-19.01 (pr-manager hardening: READY verdict HEAD-SHA pinning + release-PR merge-strategy guard + shell-dialect simulation discipline)

## VP Anchors

- VP-094 — pr-manager READY-Verdict Covered-SHA Pin, Stale-Verdict Halt, and Release-PR Merge-Strategy Enforcement (integration; S-19.01)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-094 | check-stale-verdict.sh exits non-zero with STALE_READY_VERDICT when headRefOid != covered_sha; exits 0 on match | integration (bats; S-19.01) |
| VP-094 | enforce-merge-strategy.sh exits non-zero with RELEASE_PR_SQUASH_FORBIDDEN when --squash on release/v* branch; passes --merge through on match | integration (bats; S-19.01) |
| VP-094 | pr-manager-completion-guard emits READY_SHA_MISSING advisory when SubagentStop READY verdict lacks covered_sha field | integration (bats; S-19.01) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-033 |
| Capability Anchor Justification | No existing capability covers merge-operation integrity at the pr-manager level. CAP-004 governs the BC→test→demo coverage gate — a content completeness check distinct from PR HEAD currency. BC-5.42.001 closes two separate safety windows outside CAP-004's scope: the merge-race window (stale READY verdict acting on an advanced PR HEAD, D-749) and the release-branch strategy window (accidental squash-merge violating RELEASING.md invariants, D-750). Components span SS-05 (pr-manager agent + completion-guard hook) and SS-07 (check-stale-verdict.sh + enforce-merge-strategy.sh). |
| L2 Domain Invariants | none (pipeline-orchestration operational invariant, not L2 domain spec) |
| Architecture Module | SS-05 (Pipeline Orchestration) — pr-manager agent + completion-guard hook + staleness + merge-strategy scripts |
| ADR | ADR-030 |
| Stories | S-19.01 |
| Cycle | v1.0-feature-engine-discipline-E19 (F3) |
| Feature | E-19 — Post-rc.22 Operator Hardening |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.8 | 2026-07-13 | state-manager | POL-14 auto-promotion: lifecycle_status draft→active on PR #613 squash-merge 8d1721f7 (S-19.01 MERGED 2026-07-13T14:49:50Z); BC-INDEX v3.97→v3.98; D-833. |
| 1.7 | 2026-07-11 | product-owner | S-19.01 F-P7-001 (human-directed): ADR-030 §Decision 3 governed-pass-through wrapper contract mirror — §Description (c) expanded with sole-gateway obligation, positional strategy `$2`, residual `"${@:3}"` governed pass-through, deny-list (`STRATEGY_SMUGGLING_FORBIDDEN`); Invariants 6–7 added; EC-009 (best-effort-delete) added; Canonical Test Vectors: 2 new enforce-merge-strategy.sh rows (delete-branch pass-through; residual strategy-smuggling). ADR-030 §Decision 3 cited via stable anchor per POLICY 19. |
| 1.6 | 2026-07-09 | product-owner | orchestrator pre-pass-43 consistency sweep: §Verification Properties VP-094 row-3 proof-method drift fixed — `unit (WASM test harness; S-19.01)` → `integration (bats; S-19.01)` per VP-INDEX v2.56 canonical proof method. |
| 1.5 | 2026-07-09 | product-owner | E-19 pass-33 O-P33-001: §Traceability L2 Domain Invariants TBD → none (pipeline-orchestration operational invariant, not L2 domain spec) — sibling-convention alignment (BC-1.17.001/BC-4.13.001/BC-2.07.001 class). Behavioral content unchanged. BC-INDEX bump state-manager same-burst; S-19.01 cite sweep story-writer same-burst. |
| 1.4 | 2026-07-09 | product-owner | E-19 pass-30 F-P30-002: input-hash placeholder retired per POLICY 18 (compute-input-hash --update; real digest); mechanical metadata fix; spec content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. |
| 1.3 | 2026-07-08 | product-owner | E-19 pass-22 fix burst F-P22-001 BC leg: §Architecture Anchors hooks/pr-manager-completion-guard.wasm → hook-plugins/pr-manager-completion-guard.wasm; ground-truth confirmed by hooks-registry.toml plugin field (hook-plugins/pr-manager-completion-guard.wasm) and hook-plugins/ directory listing (pr-manager-completion-guard.wasm). Anchoring correction only; behavioral content unchanged. BC-INDEX bump state-manager same-burst. |
| 1.2 | 2026-07-08 | product-owner | W1-validation fix burst F-W1V-001: §Architecture Anchors script paths updated hooks/ → bin/ per architect ADR-030 §Decision 2 + §Decision 3 adjudication (bin/ = orchestrator-invoked SS-10 CLI tools; hooks/ = dispatcher-fired namespace). Path-only propagation; no behavioral content changed. BC-INDEX bump (v3.76→v3.77) performed by state-manager same-burst. |
| 1.1 | 2026-07-06 | product-owner | E-19 pass-3 PO finalization: (a) F-P3-004 — §VP Anchors VP-TBD → VP-094 (pr-manager READY-Verdict Covered-SHA Pin, Stale-Verdict Halt, and Release-PR Merge-Strategy Enforcement); §Verification Properties three VP-TBD rows → VP-094. (b) F-P3-015 — §Traceability L2 Capability CAP-TBD → CAP-033 with business-analyst justification; Capability Anchor Justification TBD → full text; ADR ADR-TBD → ADR-030. Frontmatter capability: "CAP-TBD" → "CAP-033". |
| 1.0 | 2026-07-06 | product-owner | Initial creation. E-19 pass-2 fix burst Package 2. Three mechanically-enforced pr-manager behaviors: (a) covered_sha mandatory on READY verdicts + READY_SHA_MISSING advisory from pr-manager-completion-guard SubagentStop hook; (b) check-stale-verdict.sh stale-verdict detection with STALE_READY_VERDICT exit; (c) enforce-merge-strategy.sh release-branch merge-strategy enforcement with RELEASE_PR_SQUASH_FORBIDDEN exit. Closes L-BB-merge-race-ready-report-stale-head (D-749) + L-BB-release-pr-squash-merge-not-mechanically-enforced (D-750). |
