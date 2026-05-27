---
document_type: review-findings
story_id: S-15.12
pr_number: 155
---

# Review Findings: S-15.12 PR #155

## Convergence Table

| Cycle | Findings | Blocking | Fixed | Remaining | Status |
|-------|----------|----------|-------|-----------|--------|
| 1     | 0        | 0        | 0     | 0         | APPROVE |

## Cycle 1 — Triage

**Verdict: APPROVE — 0 blocking findings.**

### Focus Area Results

| Check | Result | Evidence |
|-------|--------|----------|
| All 4 path guards use `Path::file_name()`, NOT `ends_with` | PASS | Lines 93-138; `ends_with` only appears in doc comments and on string content (lines 370, 518) — not in path guards |
| No `regex` crate | PASS | Cargo.toml has comment explaining omission; grep confirms no regex dependency |
| `host::read_file` returns `Vec<u8>` + `String::from_utf8` | PASS | Lines 783-809; `bytes` → `String::from_utf8(bytes)` with UTF-8 error fail-open |
| `is_char_boundary()` guards on byte-index slices | PASS | Lines 346, 378, 443, 602-603 — all byte-range slice sites guarded |
| Fail-open for `host::read_file` errors | PASS | Lines 803-808; HostError → `HookResult::Continue` + `log_warn` |
| Phase 1 advisory-only for cross-site staleness | PASS | `check_cite_id_format` validates format only; code comment "Phase 2 only" at line 484; `pass-phase1-advisory-only.bats` covers this |
| `tool = "Edit|Write"` at priority 156 | PASS | hooks-registry.toml confirms both; priority 155 confirmed as `validate-stable-anchors` |
| `cited_raw: String` in Violation struct | PASS | Line 76; populated at every Violation construction site (lines 279, 296, 307, 357, 389, 481, 616) |
| 36/36 bats + 43/43 cargo tests | PASS | Verified by count: 36 `@test` blocks, 43 `fn test_` functions |

### Structural Checks

| Check | Result | Notes |
|-------|--------|-------|
| No `println!` in production code | PASS | Doc comment only; grep confirms absent in both lib.rs and main.rs |
| No `unwrap()`/`expect()` in production paths | PASS | Only `unwrap_or` on `Option` (safe defaults); no `unwrap()`/`expect()` on `Result` in lines 1-876 |
| `MAX_BYTES = 524_288` | PASS | Line 58 |
| No `file_pattern` field in registry | PASS | Confirmed absent in hooks-registry.toml entry |
| Workspace Cargo.toml member registration | PASS | Entry present in correct alphabetical position |
| `run-all.sh` glob includes new tests | PASS | `tests/validate-closes-completeness/*.bats` present in glob loop |
| Production `path_allow` is `.factory` (no `**` glob) | PASS | Single bare entry `.factory` — no glob; S-15.11 F-P2-001 lesson applied |
| `integration-production-registry.bats` present | PASS | 4 scenarios (A/B/C/D) covering lessons.md, STATE.md, decision-log.md |
| `fail-open-unreadable` has no fixture dir | PASS | By design; bats harness arranges unreadable file at runtime |

### Findings

None.

## Triage Summary

APPROVE. Zero blocking findings. All architectural constraints, test evidence, and
BC-5.39.007 compliance checks pass. The implementation correctly:

1. Uses `Path::file_name()` for all 4 path guards (not `ends_with`)
2. Omits the `regex` crate; uses hand-rolled scanning throughout
3. Handles `host::read_file` → `Vec<u8>` → `String::from_utf8` with fail-open on error
4. Guards all byte-range slice operations with `is_char_boundary()`
5. Implements Phase 1 advisory-only boundary for cross-site staleness
6. Registers at priority 156 with `tool = "Edit|Write"` and bare `.factory` path_allow
7. Carries `cited_raw: String` in `Violation` struct populated at every site
8. Passes 36/36 bats + 43/43 cargo tests; fmt/clippy clean
9. LOCAL adversary cascade CONVERGED 3/3 (8 passes; trajectory 4→2→0→0→1→0→0→0)
