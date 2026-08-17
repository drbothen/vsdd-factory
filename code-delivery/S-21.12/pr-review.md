# PR #781 — Final Fresh-Eyes Pre-Merge Review (S-21.12)

**Verdict:** READY (APPROVE)
**covered_sha:** `54825b60912974fc0361e3942d6768a477789742`
**Branch:** `feature/S-21.12` → `develop`
**Scope:** P0 security — wasmtime/wasmtime-wasi 44→46.0.2 + crossbeam-epoch 0.9.20 + anyhow floor 1.0.104 + httpmock dev-dep 0.7→0.8.3; new SHA-pinned `cargo-deny` advisories CI gate.

## Summary

Fresh-eyes review against the diff, PR description, and CI/test evidence only. No blocking findings. All five in-scope RUSTSEC advisories are cleared by genuine version bumps — none suppressed. The new CI gate is non-vacuous and correctly configured. Tests are load-bearing with real RED-before/GREEN-after semantics. All CI jobs pass on the certified HEAD.

## Checklist Verification

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — every changed file relates to the dependency move or its gate/tests. No unrelated changes. |
| 2 | Description accuracy | PASS — advisory-mapping table, CHANGELOG, and manifest/lock are mutually consistent. Benign: PR body names one Rust test; the file ships two gate tests + helpers (more coverage). |
| 3 | Test coverage | PASS — Rust version gate (`s21_12_version_gate.rs`) + 277-line bats gate cover all 9 ACs; assertions load-bearing. |
| 4 | Demo evidence | N/A — non-UI infra/dependency story; per-AC static/CI evidence per #779 precedent. |
| 5 | Commit quality | PASS — conventional format, story ID present. |
| 6 | Diff size | Large Cargo.lock churn is mechanical lockfile pruning (async-std tree removal), not logic churn. Non-issue. |
| 7 | Missing changes | PASS — all AC targets present. |
| 8 | Dependency status | PASS — `depends_on: []`; no upstream gates. |

## Independent Verification (from diff + evidence)

- **No suppression:** `deny.toml` is absent from the diff → `[advisories] ignore = []` untouched. Advisories fixed, not ignored.
- **wasmtime/wasmtime-wasi = 46.0.2** in Cargo.lock (clears RUSTSEC-2026-0188 / CVE-2026-58494 + RUSTSEC-2026-0222).
- **crossbeam-epoch 0.9.18 → 0.9.20** (RUSTSEC-2026-0204); **anyhow floor 1.0 → 1.0.104**, lock 1.0.102 → 1.0.104 (RUSTSEC-2026-0190).
- **httpmock 0.7 → 0.8.3**; `async-std` fully ABSENT from Cargo.lock at HEAD (RUSTSEC-2025-0052 reachability path genuinely eliminated).
- **No `src/**.rs` changed** → 44→46 embedder migration is a real no-op, matching the PR claim.
- **CI gate SHA pin** `EmbarkStudios/cargo-deny-action@3c6349835b2b7b196a839186cb8b78e02f7b5f25` independently confirmed as the dereferenced commit of annotated tag **v2.1.1**. Job runs `cargo deny check advisories` (`command: check` + `command-arguments: advisories`).
- **ci.yml `on.pull_request`** carries `branches: [main, develop]` and NO `paths:` key → AC-007-T2 assertion is meaningful; gate fires on every PR.
- **Tests non-vacuous:** Rust gate fails <46.0.2 / <0.9.20 (RED-before) and passes after; bats AC-007-T1 strengthened to assert the real action + `command-arguments: advisories` (rename still fails the test). httpmock migration is a real API rename (`assert_hits→assert_calls`, `body_contains→body_includes`) with assertions preserved.

## CI Evidence

All 16 real jobs PASS on `54825b60` (including `deny-advisories` 44s; `bats-full-suite` 22m23s; all 5 dispatcher builds; SAST). `release/*` guardrail correctly skipping (non-release PR).

## Findings

None blocking. Per task scope, the previously-dropped ≥1.0 mock-library requirement was NOT re-litigated — httpmock 0.8.3 is the accepted permanent solution (human decision).

**PR_REVIEW_VERDICT: READY covered_sha=54825b60912974fc0361e3942d6768a477789742**
