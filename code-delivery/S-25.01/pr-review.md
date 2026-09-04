# PR #807 (S-25.01) — Fresh-Eyes Pre-Merge Review

**Verdict: APPROVE-WITH-NITS** — no blocking code-correctness findings.

Approval is **contingent** on two merge-gate conditions (CI) plus one description/scoping
reconciliation that needs a human glance (Finding 1). None require a code change to this diff.

The implementation is careful and production-grade: best-effort-with-logging error handling
throughout, no `unwrap`/`expect` in critical paths, a fail-safe security gate (blocks on unknown
flags / unmatched quotes / corrupt marker), correctly write-tied audit emission (never fabricated
on `Err`), and load-bearing tests (real tempfile write/read/delete, real TOML parse, assertions on
content and return values — not tautological). The marker lifecycle, TTL deadman,
SUPERSEDED/OPERATOR_OVERRIDE reconciliation, and the `reconcile_raw_delete` premise (keyed on
`marker.written`, not `plugin.indeterminate`) hold up under scrutiny.

## CI status (observed)

- **`bats-darwin-leg (macos)` — FAIL, but UNRELATED to this PR.** Root cause is in
  `plugins/vsdd-factory/tests/pr-manager-hardening.bats` (T-017, line 858): a `run` helper
  `fragment-mapfile.sh` exits 127 "command not found" (bats warning BW01), plus submodule-cleanup
  noise. All 33 assertions report `ok`. This PR does not touch that file. **Re-run or waive** —
  not attributable to S-25.01.
- **PENDING (not yet green): `cargo-host (ubuntu/macos)`, all `build-dispatcher (*)`,
  `bats-full-suite (linux)`.** These are the jobs that actually exercise the 291+13+19 Rust tests
  the PR relies on. The "323/323" claim is local-only until these land. **Do not merge until green.**
- Passing: SAST, deny-advisories, validate, platforms-drift, policy-15, attestation-gate,
  bats-wave-handoff, build-dispatcher (linux-arm64).

## Findings

### [MAJOR — description/wiring coherence; NOT a code bug] Layer-1 is latent in the shipped registry

All three fail-closed Cohort A validators are registered **PreToolUse**:
- `validate-factory-path-staging` — PreToolUse `^Bash$` (described as "Cohort A-immediate, live-enforced")
- `validate-pr-merge-prerequisites` — PreToolUse `^Agent$`
- `validate-wave-gate-prerequisite` — PreToolUse `^Agent$`

But the marker write is gated `should_write_marker(...) && entry.event == "PostToolUse"` in both
`execute_tier` and `spawn_async_plugin`. Since no fail-closed validator is PostToolUse, **no marker
is ever written in production** — the marker→gate machinery has no runtime trigger path and is
exercised only by tests. For the "live" validator, a fuel/epoch/OTL exhaustion at PreToolUse emits
an advisory `plugin.indeterminate` event and, with `on_error = "continue"`, **allows the dispatch** —
the exact CWE-754 silent-pass the story exists to eliminate.

This is **spec-correct** (BC-1.18.001 INV4 mandates PostToolUse-only, and the code honors it), so it
needs **no code change**. The gap is between the wiring and the PR description's
"live-enforced / effective-now" language — `failure_policy = "fail-closed"` on a PreToolUse plugin
is behaviorally inert in this implementation. **Before merge, a human should confirm**: is Layer-1
intended to be dormant until a future PostToolUse fail-closed validator lands, and should the
description be softened accordingly?

### [MINOR — security, defense-in-depth] `is_git_commit_or_push` misses command substitution

`split_shell_segments` splits on `&&/||/;/|/&/\n` but not `$(…)` or backticks, so `echo $(git commit …)`
or `x=$(git push)` tokenizes with a non-`git` executable basename and returns `false` → Arm 2 bypass.
Acceptable for Layer 1: the realistic actor is the factory's own agents invoking `git commit`
directly; this is a quarantine-forcing gate, not an adversarial sandbox; under-blocking is
documented as the known failure mode with fail-safe on unknown flags/quotes. Note for a future
hardening pass; not a blocker.

### [MINOR — audit completeness] `reconcile_raw_delete` midnight-boundary blind spot

It scans only *today's* `dispatcher-internal-<Local date>.jsonl`. A `marker.written` just before
local midnight that is raw-deleted after midnight never gets its `OPERATOR_OVERRIDE` reconciliation
record (its `marker.written` lives in yesterday's file). Documented as bounded/best-effort and never
gates dispatch — audit-only. Acceptable; noted.

### [NIT] Cosmetic

- `classify_outcome._policy` is genuinely unused (documented, spec-mandated signature).
- `PluginResult::Crashed` maps to synthetic `Fail { exit_code: 1 }` — conflatable with a real exit-1,
  but `Crashed` is handled separately by `plugin_fail_closed`, so no impact.
- `reconcile` emits OPERATOR_OVERRIDE events in nondeterministic `HashMap::into_values()` order
  (audit-only).

No action required on any NIT.

## Checklist verification

1. Diff coherence — all changes relate to S-25.01. PASS.
2. Description accuracy — matches the diff EXCEPT the "live-enforced/effective-now" claim (Finding 1). MOSTLY PASS.
3. Test coverage — changed lines have real, load-bearing tests. PASS.
4. Demo evidence — `docs/demo-evidence/S-25.01/` has 4 VHS `.gif`/`.webm` demos + tapes + transcripts + README, mapped to ACs. PASS.
5. Commit quality — conventional, story ID present, clear lineage table. PASS.
6. Diff size — ~8.6k additions, but heavily test + demo + doc; core code is proportionate. Acceptable for a new subsystem.
7. Missing changes — none identified against the story scope.
8. Dependency status — S-21.10 (PR #780) merged. PASS.

## Merge-gate summary for the orchestrator

1. Wait for pending Rust CI (`cargo-host`, `build-dispatcher`, `bats-full-suite`) to go green.
2. Re-run or waive `bats-darwin-leg` (unrelated `pr-manager-hardening.bats` env flake).
3. Human reconciles Finding 1 (Layer-1 is inert in production; soften "effective-now" language or
   confirm intended latency). No code fix required.

https://claude.ai/code/session_01Y7xTK7sGwtpZDDKRSumE3f
