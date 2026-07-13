# PR #611 Fresh-Eyes Review — S-19.03

**Story:** S-19.03 — `warn-pending-wave-gate` FINDING-2: `read_file` `NOT_FOUND (-5)` semantics + graceful absent-file handling
**PR:** #611 (`feature/S-19.03` → `develop`), HEAD `3f88a313`
**BCs:** BC-2.07.001 v1.5, BC-2.02.011 v1.7 · **VPs:** VP-097 v1.5, VP-098 v1.2
**Reviewer:** pr-reviewer (fresh-eyes, diff + description + evidence only)

## Verdict: APPROVE

No BLOCKER or MAJOR defects. The core change is correct, well-tested, and the write-side
sibling sweep + `invoke.rs` dedup are sound. Four MINOR findings below, all independently
verified as non-blocking. FINDING-1 and FINDING-2 are recommended as in-scope follow-ups
(complete the `read_file` consumer sweep now that a dedicated `NotFound` code exists).

## What I verified (no defect found)

- **Algorithm** (`path_util::resolve_path_for_allowlist`): ancestor-walk+rejoin terminates
  correctly (root `file_name()==None → None`); fast path canonicalizes existing paths
  (resolving symlink escapes); `Path::starts_with` is component-wise, so no partial-component
  prefix bypass (`/a/b/cfoo` ⊄ `/a/b/c`). Escape `.factory/../secrets/key` resolves outside
  the prefix → `DeniedNotAllowed`.
- **read_file two-step gate**: absent-allowlisted → `Allowed` → `File::open` →
  `ErrorKind::NotFound` → `ReadErr::NotFound` → emits `internal.file_not_found` + returns
  `-5`, zero `capability_denied`. Correct.
- **invoke.rs dedup**: delegation to `write_file::prepare` preserves byte-cap semantics
  (`max_bytes.min(cap_override)`), path resolution, and now *adds* denial telemetry the
  inline copy lacked. No regression.
- **SDK `HostError::NotFound`** (no `#[non_exhaustive]`): audited every `HostError` match
  arm across all crates — all have wildcard/`Other`/`Err(_)` fallbacks, so the new variant is
  compile-safe.
- **Bats T-008 validity**: `InternalEvent` serializes flat via `#[serde(flatten)]`, so the jq
  paths `.type`/`.reason`/`.plugin_name` are correct — the red→green transition is real.
- **Demo evidence**: for a no-UI Rust+WASM product, captured-stdout transcripts +
  real-dispatcher bats is the correct demo-recorder mode; evidence-report.md maps all 6 ACs
  to tests+transcripts. The `.gif/.webm` checklist rule does not apply here.

## Findings

### FINDING-1: [MINOR] Incomplete sibling-sweep of the `read_file` absent-file contract change
- **File:** `crates/hook-plugins/validate-per-story-adversary-convergence/src/main.rs:78`
  (RealCallbacks) + docstring ~lines 63-65.
- The shared contract changed: absent-allowlisted files now return `NOT_FOUND (-5)` instead
  of `CAPABILITY_DENIED (-1)`. This consumer maps `Err(CapabilityDenied) => Ok(None)` and its
  docstring says "HostError maps to None for capability-denied / **not-found**", but has no
  `NotFound` arm — absent files now fall through to `Err(e) => Err(IoError("...NotFound"))`,
  contradicting the documented contract.
- **Blast radius verified benign:** caller `hook_logic` (lib.rs:544) collapses both `Ok(None)`
  and `Err(_)` to the same `CONVERGENCE_STATE_MISSING` block, so runtime behavior is
  unchanged. Contract/comment drift, not a behavioral regression.
- **Fix:** add `Err(HostError::NotFound) => Ok(None)` and refresh the comment (TD-VSDD-060).
  Other explicit-`CapabilityDenied` handlers (update-wave-state-on-merge, verify-factory-lock,
  validate-wave-handoff) audited — unaffected (wildcard / generic fail-open).

### FINDING-2: [MINOR] Story theme not applied to sibling consumer of the same file
- **File:** `crates/vsdd-context-resolvers/src/lib.rs:91`.
- This story's purpose is eliminating false-positive noise on an absent
  `.factory/wave-state.yaml`. `vsdd-context-resolvers` reads the *same* file; its comment
  intends file-not-found to be silent (mapped to `Other(_)`), but absent now returns
  `NotFound`, hitting `_ => log_warn(...)` — still logs a spurious warning on fresh projects.
- **Fix:** map `HostError::NotFound => { /* silent, expected on fresh projects */ }` to fully
  realize the intended behavior, consistent with the story goal.

### FINDING-3: [MINOR/INFO] VP-097 traversal-defense proof is not CI-enforced
- **File:** `crates/factory-dispatcher/src/host/path_util.rs` (`#[cfg(kani)] mod kani_proofs`).
- Two stacked gaps: (a) disclosed D-826 — harness unrun in CI (Kani 0.67 / MSRV 1.95
  mismatch); (b) the harness's own note that VP-097 §Proof Harness Skeleton has a stale
  monolithic signature (spec-drift routed to architect). Net: the traversal-defense theorem is
  currently unverified in the pipeline; the `starts_with` gate is covered only by unit tests.
  Harness quality itself looks sound (faithful `model_canonicalize`, non-vacuity witness H3).
- **Fix:** ensure a tracked follow-up exists to (1) run Kani once the toolchain gap resolves
  and (2) reconcile the VP-097 skeleton with the shipped two-function design.

### FINDING-4: [MINOR] `warn-pending-wave-gate.wasm` untracked while 37 sibling plugins tracked
- **File:** `plugins/vsdd-factory/hook-plugins/warn-pending-wave-gate.wasm` (deleted + covered
  by pre-existing `.gitignore` dir rule); registry `hooks-registry.toml:1113` still references it.
- **Verified benign:** CI (`ci.yml` Stage WASM) and release (`release.yml:306` + registry-vs-
  staged assertion at `:340`) rebuild every registry-declared WASM from `crates/hook-plugins/`
  and fail if any is absent; bats `setup_file` builds on demand. The released bundle self-heals.
- **Suggestion:** resolve the lone 37-tracked/1-untracked inconsistency — either track it like
  its siblings or document why it is the exception (cf. `RELEASING.md:268`
  `git checkout main -- '*.wasm'`).

## Checklist
1. Diff coherence — PASS (all changes trace to S-19.03 ACs)
2. Description accuracy — PASS (PR body matches diff)
3. Test coverage — PASS (T-001..T-008 + Kani harness; red→green verified)
4. Demo evidence — PASS (library/WASM mode: transcripts + real-dispatcher bats; all 6 ACs)
5. Commit quality — PASS (conventional, story-ID prefixed, no AI attribution)
6. Diff size — ~2186 additions; justified (new shared module + full test suites + evidence)
7. Missing changes — see FINDING-1/2 (read_file consumer sweep incomplete; benign)
8. Dependency status — target `develop`; no unmerged upstream dep observed
