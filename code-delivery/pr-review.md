# PR #776 — Final Fresh-Eyes Review (pr-reviewer)

- **PR:** #776 — [S-21.07] validate-cross-site-correspondence WASM hook
- **Branch:** feature/S-21.07-validate-cross-site-correspondence → develop
- **Reviewed HEAD:** 3fc6d7d9
- **Reviewer:** vsdd-factory:pr-reviewer (fresh-eyes)
- **Date:** 2026-08-15

## VERDICT: APPROVE (ready to merge)

No blocking findings. No NEW correctness or security defects. The CWE-697 classifier
fix is correct and complete; regression coverage is strong; code quality conforms to
repo conventions. The settled SEC-001 bypass refutation was NOT re-litigated — this
review independently assessed FIX QUALITY.

## 1. CWE-697 fix correctness — VERIFIED CORRECT & COMPLETE

`chain_immediately_precedes_filename` (dispatch.rs:102-142):

- Chain start is DERIVED from component count (`len - (chain.len()+gap+1)`), never
  searched → the governed dir chain must be a contiguous, correctly-ordered window
  immediately preceding the filename. Non-contiguous / out-of-order decoys cannot match.
- Unconditional `Component::ParentDir` rejection (dispatch.rs:123) closes the
  leading-`..` prefix residual and any `..` in prefix/chain/gap/filename — one shared
  guard covering every arm.
- Unconstrained `Normal` prefix correctly preserves absolute/worktree paths where
  `.factory` is not component 0.
- CurDir / interior-`.` normalization edge checked: `Path::components()` drops interior
  `.`, and any byte <0x80 is a standalone ASCII char, so no CurDir-based decoy leverage.
  `CurDir`/`RootDir`/`Prefix` are correctly NOT rejected.
- All classifiers (is_bc_file, is_story_file, is_story_index) + both
  is_frontmatter_parity_target arms (VP, epic) route through the anchored helper. No
  residual `.any()`-style presence matching remains anywhere in the crate.
  is_cycle_artifact is Class-D-deferred (returns None).
- Under-inclusion direction: the only new false-negative is a governed write whose path
  literally contains `..` — the safe direction (write proceeds ungoverned; no bypass).
  The real traversal defense (Kani-proven `path_util::check_path_allowed`, VP-097) is
  untouched and downstream.

## 2. Regression test coverage — ADEQUATE

18 SEC-001 tests + full dispatch suite: non-contiguous decoys, `..`-after-chain decoys,
leading-`..` decoys, absolute-form positive controls across classifiers/arms.
231/231 crate lib tests pass; clippy `-D warnings` clean.

## 3. Rest of diff — reviewed fresh, clean

- Multibyte-panic fix (arm_b.rs `extract_input_hash_token`:679): retry steps by
  `char::len_utf8` of the char at the retry position rather than +1 → no mid-char slice
  panic. All byte-slice sites derive boundaries from ASCII-only hex runs.
- `\b` word-boundary unification via `crate::is_word_char`: `bytes[i-1] as char` (arm_a1/
  arm_a2) is provably equivalent to Arm B's `chars().last()` because `is_word_char` is
  ASCII-only (`is_ascii_alphanumeric() || '_'`). No latent multibyte boundary bug; byte
  reads only, no slice panic risk.
- INDETERMINATE disposition for STORY-INDEX UTF-8 decode failure (arm_b.rs:243-256):
  advisory + early-return instead of silently degrading to `(None,None)` fail-open. Sound.
- WASM currency: 639268b3 (last .rs change) → 3fc6d7d9 rebuilt the .wasm; no .rs changed
  after the rebuild. Binary is current with source.
- CI workflow: mounting factory-artifacts before `cargo test` (so corpus tests run under
  CI_REQUIRE_ARTIFACTS rather than silently skipping) is correct and well-commented.

## 4. Code quality — conforms

No `println!` in production; `unwrap()`/`expect()`/`panic!` only in `#[cfg(test)]`
modules and test helpers. Named error taxonomy (HostError; Violation/Advisory).
Pure-core dispatch (no I/O).

## Non-blocking advisory observations (do NOT block merge)

- **LOW / pre-existing:** `is_volatile_path` (arm_b.rs:440) uses substring
  `path.contains(".factory/cycles/")` — non-anchored, unlike the SEC-001-hardened
  classifiers. A decoy `evil/.factory/cycles/x/STATE.md` would be treated as volatile
  (input-hash check skipped). Impact is governance-integrity only (never a traversal/
  security bypass), and it is explicitly spec-mandated by BC-5.39.010 PC40 / ADR-037
  §Decision 2 ("path **contains** `.factory/cycles/`") with an inline rationale. No
  change requested.
- **INFO:** SEC-001 coverage is slightly asymmetric — story_index and epic arms lack an
  explicit `..`-after-chain test (BC/story/VP arms have all three). Behavior is identical
  since the ParentDir reject is one shared guard; cosmetic test-parity only.

## Recommendation

Merge.
