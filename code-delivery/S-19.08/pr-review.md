# PR Review — S-19.08 (PR #646)

**Title:** feat(S-19.08): verify-state-timestamp-refresh — raise 64 KiB byte cap to 256 KiB + wire extract_frontmatter
**Branch:** feature/S-19.08 (HEAD 47b5e0c6) → develop
**Reviewer:** pr-reviewer (fresh-eyes, cognitive-diversity; sees only diff + description + demo evidence)

## Verdict: APPROVE (CLEAN)

No BLOCKING or HIGH findings. All 8 checklist items evaluated; all 6 assessment questions verified against the diff. A few LOW/NIT advisory notes are recorded below (none are merge blockers).

---

## What was verified

**1. Cap raise (AC-001) — CORRECT.** `STATE_MD_MAX_BYTES: u32 = 262144` (was 65536). `test_BC_5_40_001_T001` asserts the constant directly. The guard passes this constant as `max_bytes` to `read_file` (call site unchanged), so the raise is effective — the host uses the guard-supplied cap. Mirrors the S-19.02 `verify-factory-lock` pattern.

**2. `extract_frontmatter` wiring (AC-003) — CORRECT and properly ordered.** Called on raw `&on_disk_bytes` before any `String::from_utf8` conversion (Invariant 7). `delimiter_found = frontmatter_owned.len() < on_disk_bytes.len()` is sound: `extract_frontmatter` returns a strictly shorter slice when a closing delimiter is present, and the full input when absent. Frontmatter-only content is used for both `timestamp` (Step 5) and `factory_lock` subfield (Step 7) extraction. T-004 validates the wiring via a non-UTF-8-body fixture that would fail `String::from_utf8` on full bytes — a genuinely falsifiable red gate.

**3. Soft-warn `state_md_approaching_cap` (AC-005) — CORRECT boundaries.** Implementation: `bytes_read > 200_000 && bytes_read <= STATE_MD_MAX_BYTES as usize` exactly matches the spec interval `(200000, 262144]`. The T-007 A–E matrix confirms all boundaries: 200000-exact silent (strict `>`), 210000 warns, 262144-cap warns AND read succeeds, 262145 over-cap fails-open with zero warn. Warn is emitted only on successful reads and never alters the Continue/Block verdict (observability-only), confirmed by sub-test E.

**4. F-P2-001 reconstruction-base fix — SOUND.** Introduction of `on_disk_reconstruction_base` (full content for Edit/MultiEdit) separate from `on_disk_field_content` (frontmatter-only for field extraction) correctly fixes the truncation regression that frontmatter-only extraction would otherwise introduce for body-targeting edits. Non-UTF-8-body fallback to frontmatter-only base is defensible (an Edit cannot match inside invalid-UTF-8 body bytes). Covered by body-target Block tests and the F-P3-001 fallback regression lock.

**5. Test quality — MEANINGFUL, non-tautological.** T-006 (integration) is the load-bearing cap-enforcement test: its mock applies the real `fixture_size > max_bytes` comparator against the live `STATE_MD_MAX_BYTES`, so it is RED at 65536 and GREEN at 262144. T-007 boundary assertions are falsifiable against threshold regressions. F-P2-002 body-target and F-P3-001 fallback tests carry documented mutation checks. All references to the removed `on_disk_content` are accounted for (compilation + green CI confirm the sweep is complete).

**6. Diff boundary — MATCHES claim.** Only `src/lib.rs`, the new integration test, and 5 `docs/demo-evidence/S-19.08/` files. No `Cargo.toml`, `hooks-registry.toml`, `read_file.rs`, or host-ABI changes. "Zero new dependencies" and "no registry change" claims hold.

---

## Checklist

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — all changes relate to S-19.08 |
| 2 | Description accuracy | PASS — body matches actual changes |
| 3 | Test coverage | PASS — changed lines covered by T-001..T-007 + integration + regression locks |
| 4 | Demo evidence | PASS (product-type) — see LOW note; captured-stdout transcripts appropriate for no-UI WASM plugin; evidence-report.md covers all 5 ACs, success + error paths |
| 5 | Commit quality | PASS — conventional format, story ID, clear messages |
| 6 | Diff size | ADVISORY — >500 lines but ~1128 are tests; ~90 lines production logic |
| 7 | Missing changes | PASS — all 5 ACs implemented and traced |
| 8 | Dependency status | PASS — S-19.02 (#610) merged on develop; ships `extract_frontmatter` |

---

## Findings

| Severity | Category | Finding | Suggestion |
|----------|----------|---------|------------|
| NIT | size | `lib.rs` +1243/-19; heuristic >500-line flag. ~1128 lines are test code, ~90 lines production logic. | No action; acceptable for a well-scoped change with thorough coverage. |
| LOW | coverage | T-002/T-003 use the cap-ignoring `make_callbacks_with_raw_bytes` mock and rely on the `assert!(STATE_MD_MAX_BYTES >= 70_000)` pre-condition as their red gate; they would pass at the old cap if that assert were removed. | Acceptable — genuine cap-enforcement coverage lives in T-006 (integration). No change required. |
| NIT | style | Helper inconsistency: unit-test fixture uses `std::iter::repeat_n(b'#', remaining)`; integration fixture uses `std::iter::repeat(b'#').take(remaining)` with a `clippy::manual_repeat_n` allow. | Cosmetic, test-only; harmonize if convenient. |
| LOW | coverage | Demo evidence is captured-stdout `.txt` transcripts, not `.gif`/`.webm`. Appropriate for a Rust WASM hook plugin with no visual/CLI surface; evidence-report.md is per-AC with success + error paths. | Treated as satisfied for product type. |

## Out of scope (could not verify from diff; no concern)
- Production `read_bounded` comparator in `read_file.rs` (strict `>` vs `>=`) is not in this diff. Tests assume strict `>` (cap-inclusive read at 262144), internally consistent and matching claimed S-19.02 parity; `read_file.rs` is unchanged.

---

**Conclusion:** No changes requested. PR #646 is approvable on its own merits.
