## Fresh-Eyes PR Review — S-19.06 `host::read_prefix` — **APPROVE**

Independent review (different model, diff + evidence only). All 7 ACs and all critical invariants verified against the actual diff.

### Verified

**AC-001/002 (bounded + short-file):** `read_prefix_bounded` uses `File::take(max_bytes as u64)` + `read_to_end` — byte-exact head-c, no padding, no UTF-8 trimming. T-001 (100→50), T-002 (partial multi-byte boundary untrimmed), T-003 (30-byte file, no padding) all present.

**AC-003 (NEVER OUTPUT_TOO_LARGE):** `max_bytes` is the cap by construction — mechanically cannot emit `-3`. Static gate T-009g strips test/comment regions and greps production code; the only two `OUTPUT_TOO_LARGE` occurrences in `read_prefix.rs` are inside the `#[cfg(test)]` `assert_ne!` in T-004 (asserting it is *never* returned), which the gate explicitly excludes. Runtime lock T-004 confirms. Satisfied.

**AC-004 (capability independence):** `ctx.capabilities.read_prefix.as_ref().ok_or_else(...)` — no fallback to `read_file`. T-005 (no caps), T-006 (read_file-only → denied), T-013a lock. Registry adds an independent `ReadPrefixCaps` with `deny_unknown_fields`.

**AC-005 (NOT_FOUND):** `PrefixReadErr::NotFound` → `internal.file_not_found` event + `codes::NOT_FOUND`. T-007 asserts exactly 1 file_not_found + 0 capability_denied events.

**AC-006 (max_bytes=0 short-circuit):** step 3 returns `Ok((Vec::new(), 0))` before opening the file. T-008 + T-012 (absent file + max_bytes=0 → Ok, no NOT_FOUND).

**AC-007 (two-layer hook-sdk + registration + fixture):** safe wrapper in `host.rs` (`Result<Vec<u8>, HostError>`), raw `pub safe fn` extern in `ffi.rs` with `#[cfg(not(wasm32))]` stub, `read_prefix::register` wired in `mod.rs::setup_linker`, wasm32-wasip1 fixture compiles. bats T-009a..h.

### Critical invariants — all hold
- `read_file.rs` and `path_util.rs` **byte-unmodified** (empty diff stat). ✓
- OUTPUT_TOO_LARGE absent from non-comment production code. ✓
- **Ordering:** capability check (step 1) → path check (step 2) → max_bytes=0 short-circuit (step 3). Confirmed in source order and locked by T-013a/b plus two mutation-liveness witnesses (TD-VSDD-059) proving the gates are non-vacuous. ✓
- HOST_ABI_VERSION unchanged (only a doc-comment mention). ✓

### Notes (non-blocking)
- **Demo evidence** is captured-stdout `.txt` transcripts, not `.gif`/`.webm`. This is correct for a pure Rust library + WASM FFI story with no UI/CLI surface; the evidence-report frontmatter declares `product_type: Rust library (no UI)` / `evidence_mode: captured-stdout test transcripts`, and all 7 ACs have a transcript + coverage matrix. Not a blocker.
- **out_ptr=0 sentinel** and **timeout_ms non-enforcement (`let _ = timeout_ms`)** mirror `read_file` exactly and are the documented accepted-with-record items anchored to the post-E-19 architect story — not re-litigated here.
- POLICY 20: `read-prefix-fixture` excluded via dual-layer `--exclude` in ci.yml + release.yml + `[[bin]]`-floor `grep -v`, and additionally proven governing by `bundle_orphan_check.rs` T-011.
- Diff is ~2.3k lines but dominated by tests, per-AC comments, demo evidence, and bats — proportionate to the story.

Upstream deps S-19.03 (#611) and S-19.04 (#639) merged. No blocking or high-severity findings.
