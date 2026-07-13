# PR #610 — S-19.02 verify-factory-lock guard hardening — Final Fresh-Eyes Review

**Verdict: APPROVE** (no blocking findings). One LOW and one ADVISORY finding, both non-merge-gating. One CI process observation.

Branch: `feature/S-19.02` → `develop`
BCs: BC-4.13.001 v1.15 Phase-A (Precondition 3 = 262144, Invariant 9 = frontmatter-only parse, Invariant 10 = soft-warn 200000 strict `>`). VPs: VP-095, VP-096.

---

## What I verified

### 1. Correctness — `extract_frontmatter` (crates/factory-lock-parse/src/lib.rs)
Four-branch delimiter search is correct with sound precedence:
- LF-inline `\n---\n` → `bytes[0..pos]` (pos = leading `\n`).
- CRLF-inline `\r\n---\r\n` → `bytes[0..pos]` (pos = leading `\r`).
- CRLF-EOF `\r\n---` checked **before** LF-EOF (load-bearing: `\r\n---` also ends with `\n---`; LF-first would strip only 4 bytes and leak a stray `\r`).
- LF-EOF `\n---`, then full-input fallback.
No panics, no `unwrap`/`expect`, no `println!`, pure/deterministic. Idiomatic.

### 2. Wiring in the guard (crates/hook-plugins/verify-factory-lock/src/lib.rs)
- `delimiter_found = frontmatter_owned.len() < state_bytes.len()` is valid given the function contract.
- Synthetic `\n---\n` re-append is correct: verified against the **unchanged** `parse_factory_lock` (lines 78–107), which normalises CRLF→LF at entry, so a mixed synthetic input `---\r\n…\n---\n` parses correctly. This is why CRLF wiring test T-012 legitimately returns `Block`.
- Behavior equivalent to pre-change for well-formed files; adds Invariant-9 defense without altering the verdict.
- `.to_vec()` borrow-release comment accurate; `state_bytes` move in the no-delimiter branch is correct.

### 3. Soft-warning boundary (AC-006 / Invariant 10)
`bytes_read > 200_000 && bytes_read <= STATE_MD_MAX_BYTES as usize` — strict `>` lower, inclusive upper at 262144. Matches spec. T-009 A–E cover 150000/200000-exact/210000/262144-exact/262145 with distinct boundary sub-cases.

### 4. Cap raise (AC-001/002/004)
65536→262144. T-001 asserts constant; T-002/T-003 exercise 70 KiB fixture; new integration `t006_vp095_real_cap_enforcement_sizes` uses a genuine cap-enforcement mock (Err when `fixture_size > max_bytes`) over 65535/65536/131072/262144 (Block) and 262145 (fail-open + warn). Correctly replaces the previously-tautological T-006 — a real test-integrity improvement.

### 5. Proptest VP-096
Structural oracle (prefix / delimiter-partition / minimality Invariant C), determinism, CRLF known-answer. Invariant C is genuinely non-tautological. `.proptest-regressions` seeds checked in.

### 6. No regressions
Net PR diff is additive; no develop-side test removed. Clean red→green TDD history with LOCAL cascade CRLF fix folded in.

---

## Findings

### [LOW] test-clarity — vacuous match in T-009 sub-test D
`crates/hook-plugins/verify-factory-lock/src/lib.rs` (~865–870). The `read_errored` match returns `false` on every arm, so `read_errored` is a constant `false` and `&& !read_errored` is dead. The meaningful assertion is `!has_read_error_warn`. Suggest deleting the `read_errored` block and asserting on `has_read_error_warn` alone. No functional impact.

### [ADVISORY] purity-boundary — global LF-inline search precedes CRLF handling
`extract_frontmatter` searches the whole buffer for LF-inline `\n---\n` first. In a hypothetical *mixed* line-ending file (CRLF frontmatter, body containing a lone `\n---\n`) it would over-extract into the body — a technical Invariant-9 boundary nuance. Confirmed **no impact on the lock verdict**: `parse_factory_lock` normalises CRLF→LF and re-scopes to its own first `\n---\n`. Not a realistic STATE.md scenario (autocrlf checkout is homogeneous CRLF). No action required; noted for completeness.

---

## Process observation (not a code finding)
CI `cargo-host (ubuntu-latest / macos-latest)` — the fmt/clippy/test gate — was **pending** at review time (`validate`, `SAST`, `bats-wave-handoff`, `platforms-drift` pass). PR body attests local green. Merge should be gated on `cargo-host` going green.

---

## Checklist summary
- Diff coherence: clean (all changes trace to S-19.02 ACs).
- Description accuracy: matches diff.
- Test coverage: all 6 ACs + distinct boundary sub-cases.
- Demo evidence: `docs/demo-evidence/S-19.02/` present (evidence-report.md + 5 transcripts). Captured-stdout is the correct mode for a library/WASM-plugin story with no UI.
- Commit quality: conventional, story-ID tagged, no AI attribution.
- Diff size: +1759/-14, overwhelmingly tests + evidence; production delta ~90 lines.
- Missing changes: none.
- O-S1902 (sibling `verify-state-timestamp-refresh` 64 KiB cap) correctly disclosed as out-of-scope (BC-5.40.001); not flagged per review scope.

**Recommendation: APPROVE pending `cargo-host` CI green.** No blocking or high-severity findings.
