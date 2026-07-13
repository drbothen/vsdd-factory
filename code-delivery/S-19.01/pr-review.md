# PR #613 — Final Fresh-Eyes Review (S-19.01 pr-manager hardening)

**Verdict: APPROVE** — no blockers.
**Reviewer:** pr-reviewer (fresh-eyes, different model family; final PR-level check after LOCAL 16-pass cascade)
**Scope reviewed:** PR diff + description only (information wall respected).

## What was verified

Reviewed the full diff, both new shell scripts in their entirety, the WASM hook
changes, the `pr-manager.md` Step 8 wiring, and ran a bash-3.2 compatibility
sweep. Checked against BC-5.42.001 v1.7 / ADR-030 shell semantics and the four
named attack vectors.

### (a) Can `gh pr merge` be reached without `enforce-merge-strategy.sh`? — NO
In `pr-manager.md` the only merge invocation is Step 8-pre-B
(`enforce-merge-strategy.sh`, line 257). All other `gh pr merge` strings
(lines 262, 512, 521, 525, 535) are prose describing the prohibition.
`grep -E 'Agent\(.*gh pr merge'` returns zero direct invocations. The wrapper
is the sole caller of `gh pr merge` (lines 147/149).

### (b) Can a READY verdict reach merge without a valid `covered_sha`? — NO mechanical path
`check-stale-verdict.sh` Step 1 (lines 47-50) rejects any non-`^[0-9a-f]{40}$`
value with `READY_SHA_MISSING` before any network call. Step 8-pre-A halts if
absent (line 241, no re-fetch; confirmed by T-032). The WASM hook is
advisory-only (returns `Continue` even on the block branch, line 163),
consistent with ADR-030 §Decision 1; the load-bearing gate is the script,
correctly wired ahead of Step 8-pre-B.

### (c) Deny-list bypass? — NONE FOUND
Traced every gh strategy/admin flag form against the four `case` globs
(lines 80-104): long, `=`-fused, bare/leading short (`-[smrA]*`), and embedded
short clusters (`-[!-]*[smrA]*`). `-ds`, `-sd`, `-A`, `--squash=auto` all
caught; `-d`/`--delete-branch` correctly pass. Deny-list (Step 2) runs before
the release check (Step 5), so residual smuggling is rejected even when `$2`
is legitimate. `$2 = --admin/-A` separately rejected as `INVALID_STRATEGY`
(lines 111-119).

### (d) bash 3.2 incompatibilities? — NONE
No `mapfile`/`readarray`/`${var,,}`/`declare -A` (only in comments). Uses
`while IFS= read -r` + heredoc, `grep -oE`, `tr`, `case` globs, and `"${@:3}"`
positional slicing (safe under `set -u` when empty). All bash 3.2.x compatible.

## Findings

### MINOR-1 — `enforce-merge-strategy.sh`: fail-OPEN branch resolution (lines 60-72)
If `gh pr view --json headRefName` fails or returns null `headRefName`,
`BRANCH_NAME` stays empty → `IS_RELEASE="false"` → a caller-supplied
`--squash`/`--rebase` on an actual `release/v*` PR would be validated and
delegated to `gh pr merge --squash` — the exact invariant D-750 closes.
This is a deliberate, tested choice (T-021) and the inline comment cites
ADR-030 §Decision 3 rationale, so per spec-wins this is NOT a blocker. Surfaced
because the production-grade posture would normally be fail-CLOSED (refuse to
merge when the branch cannot be proven non-release); the mitigation (failed
read implies failed merge) is not guaranteed. Recommend confirming ADR-030
§Decision 3 genuinely specifies fail-open vs. the safer fail-closed. If it does,
no change needed.

## Accepted-with-record disclosures — acknowledged, NOT re-raised
1. BC-5.42.001 Inv-7 "before any gh invocation" wording nit (read-only
   `gh pr view` at line 63 precedes deny-list) — non-load-bearing.
2. `covered_sha` cross-referenced in READY Verdict Format section rather than
   inline at Step 8-pre-A (which is fail-closed HALT-if-absent) — non-load-bearing.
3. EC-009 delete-verify mechanism divergence (`ls-remote`+`push --delete` vs
   `gh api`) — functionally equivalent; pre-existing.

## Other observations (non-findings)
- WASM hook `println!` at lib.rs:162 emits the SubagentStop verdict JSON
  (`advisory_code` is a static JSON-safe string) — this is the hook I/O
  protocol, consistent with existing `emit`/`block_stderr` pattern; not a
  "no println! in production Rust" violation.
- `check-stale-verdict.sh` arm ordering (arm 4 → 3a → 3b → headRefOid-absent →
  SHA compare) is sound; the documented state-field-absent fall-through
  (lines 99-107) is unreachable in practice (script always queries
  `--json headRefOid,state`) and still requires exact SHA match to exit 0.

## Recommendation
APPROVE. MINOR-1 is spec-sanctioned and test-backed; forward to the
orchestrator/architect only to confirm the ADR-030 §Decision 3 fail-open intent.
