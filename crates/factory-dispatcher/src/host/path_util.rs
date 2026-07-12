//! Shared path resolution helper for host read/write allowlist enforcement.
//!
//! S-19.03: Extracted from `write_file.rs` to provide a single, testable
//! ancestor-walk + rejoin algorithm for both `read_file.rs` and `write_file.rs`.
//! The injectable `canonicalize_fn` parameter enables unit testing without a
//! real sandboxed filesystem (BC-2.07.001 EC-007).
//!
//! Purity classification: pure-core — no I/O; the canonicalize function is
//! injected by the caller.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs (belt-and-suspenders;
// the workspace already sets check-cfg = ["cfg(kani)"]). Matches the inline
// Kani-harness convention in partition.rs / aggregator.rs.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use std::path::{Path, PathBuf};

/// Result of the two-step path allowlist check (architect Ruling-2 / S-19.03 AC-001).
///
/// Shared by `read_file.rs` and `write_file.rs` — both use the same two-step
/// decomposed pattern with distinct reason tokens per the Architecture Mapping.
#[derive(Debug, PartialEq, Eq)]
pub enum PathAllowDecision {
    /// Path resolved and lies within an allowed prefix.
    Allowed,
    /// Ancestor-walk failed to canonicalize any ancestor — filesystem/traversal error.
    /// Caller emits `internal.capability_denied reason=path_resolution_failed`.
    DeniedResolutionFailed,
    /// Path resolved successfully but lies outside all allowed prefixes.
    /// Caller emits `internal.capability_denied reason=path_not_allowed`.
    DeniedNotAllowed,
}

/// Resolve a target path for allowlist comparison using an ancestor-walk + rejoin
/// algorithm, canonicalizing to defeat `..` traversal attacks.
///
/// Accepts an injectable `canonicalize_fn` (signature `fn(&Path) -> std::io::Result<PathBuf>`)
/// to enable unit testing without a real sandboxed filesystem (BC-2.07.001 EC-007). In
/// production, callers pass `|p| p.canonicalize()`.
///
/// Algorithm: if the full path canonicalizes, return it directly. Otherwise walk
/// ancestor components bottom-up, collecting non-existent tail components, until an
/// ancestor canonicalizes. Rejoin the collected tail onto the canonical ancestor in
/// original order. If no ancestor canonicalizes (all ancestors fail), return `None` —
/// the caller emits `path_resolution_failed` reason token (not `path_not_allowed`).
///
/// Purity classification: pure-core (path manipulation only; no filesystem I/O
/// beyond the injected canonicalize_fn calls).
pub fn resolve_path_for_allowlist(
    target: &Path,
    canonicalize_fn: impl Fn(&Path) -> std::io::Result<PathBuf>,
) -> Option<PathBuf> {
    // Fast path: the full path canonicalizes (file exists or all symlinks resolve).
    if let Ok(canon) = canonicalize_fn(target) {
        return Some(canon);
    }
    // Slow path: walk ancestors bottom-up collecting the non-existent tail
    // components, then canonicalize the deepest existing ancestor and rejoin
    // the tail in original (top-to-bottom) order.
    //
    // Example: target = /project/.factory/wave-state.yaml
    //   1. canonicalize_fn(full path) → Err (file absent)
    //   2. tail.push("wave-state.yaml"), cur = /project/.factory
    //   3. canonicalize_fn(/project/.factory) → Ok(/project/.factory) (exists)
    //   4. rejoin: /project/.factory/wave-state.yaml → return Some
    //
    // If NO ancestor canonicalizes (EC-007 / injectable-mock failure), returns None.
    // Caller emits `internal.capability_denied reason=path_resolution_failed`.
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    let mut cur = target.to_path_buf();
    loop {
        // `file_name()` returns None for a root path (e.g. "/") — stop here.
        let filename = cur.file_name()?.to_os_string();
        tail.push(filename);
        let parent = cur.parent()?.to_path_buf();
        if let Ok(canon_parent) = canonicalize_fn(&parent) {
            // Deepest canonicalizable ancestor found. Rejoin the collected tail
            // in original order (tail was collected bottom-up, so iterate in reverse).
            let mut result = canon_parent;
            for component in tail.iter().rev() {
                result = result.join(component);
            }
            return Some(result);
        }
        cur = parent;
    }
}

/// Two-step allowlist check shared by `read_file.rs` and `write_file.rs`
/// (architect Ruling-2 / S-19.03 AC-001; BC-2.07.001 Invariant 5).
///
/// Step 1: resolve via ancestor-walk+rejoin (handles absent files correctly,
///   unlike `Path::canonicalize()` which returns Err for non-existent files).
///   Returns `DeniedResolutionFailed` when even the root ancestor fails —
///   structurally impossible on real Unix filesystems, but testable via the
///   injectable mock (BC-2.07.001 EC-007).
///
/// Step 2: pure `starts_with` prefix check against each allow-list entry.
///   Allow-list entries that are relative are expanded under `base`.
///   Returns `DeniedNotAllowed` when the resolved path lies outside all prefixes.
///
/// Separating resolution failure from allowlist failure lets operators distinguish
/// filesystem errors from genuine access-policy violations in telemetry.
///
/// The injectable `canonicalize_fn` (production callers pass `|p| p.canonicalize()`)
/// makes the `DeniedResolutionFailed` arm reachable under test (BC-2.07.001 EC-007).
pub(crate) fn check_path_allowed(
    resolved: &Path,
    allow: &[String],
    base: &Path,
    canonicalize_fn: impl Fn(&Path) -> std::io::Result<PathBuf> + Copy,
) -> PathAllowDecision {
    // Step 1: resolve with ancestor-walk+rejoin so absent-but-allowlisted files
    // get a synthesized canonical path instead of an opaque resolution failure.
    let canon_resolved = match resolve_path_for_allowlist(resolved, canonicalize_fn) {
        Some(p) => p,
        None => return PathAllowDecision::DeniedResolutionFailed,
    };

    // Step 2: prefix check. Allow-list entries are also resolved via ancestor-walk+rejoin
    // so that file-scoped allow-list entries (e.g. ".factory/wave-state.yaml") work
    // correctly even when the file does not yet exist. If the prefix's entire ancestor
    // chain fails canonicalization, that prefix is skipped.
    for pref in allow {
        let pref_path = if Path::new(pref).is_absolute() {
            PathBuf::from(pref)
        } else {
            base.join(pref)
        };
        let canon_pref = match resolve_path_for_allowlist(&pref_path, canonicalize_fn) {
            Some(p) => p,
            None => continue, // configured prefix's ancestors also absent — skip
        };
        if canon_resolved.starts_with(&canon_pref) {
            return PathAllowDecision::Allowed;
        }
    }
    PathAllowDecision::DeniedNotAllowed
}

// ---------------------------------------------------------------------------
// VP-097 Kani proof harness — `resolve_path_for_allowlist` traversal defense.
//
// Proves the BC-2.07.001 Invariant 1 + BC-2.02.011 EC-001 traversal-defense
// property: `..` sequences cannot resolve to an `Allowed` path outside the
// declared allowlist prefixes.
//
// Defined inline in this file as `#[cfg(kani)] mod kani_proofs { ... }`,
// matching the established repo convention for in-crate Kani harnesses (see the
// inline `kani_proofs` modules in `partition.rs` and `aggregator.rs`, and
// `kani_harnesses` in `resolver_classify_trap.rs`). The `#[cfg(kani)]` gate
// excludes these harnesses from `cargo test` / `cargo build` / `cargo clippy`;
// they compile only under `cargo kani`.
//
// # How to run
// `cargo kani -p factory-dispatcher --harness <name>` (requires the kani
// toolchain).
//
// # Mapping to the shipped S-19.03 code (deviation from VP-097 §Proof Harness
// Skeleton)
//
// VP-097 v1.2 §Proof Harness Skeleton is written against a monolithic
// signature `resolve_path_for_allowlist(path: &str, allowlist: &[&str]) ->
// Result<PathBuf, CapabilityDenied>` that predates the S-19.03 implementation.
// The shipped design splits the concern into two functions:
//
//   * `resolve_path_for_allowlist` — `(target: &Path, canonicalize_fn) ->
//     Option<PathBuf>`. Pure ancestor-walk + rejoin. Does NOT perform the
//     allowlist check; by design it returns `Some` even for escaping inputs
//     (the `..` is absorbed by ancestor canonicalization).
//   * `check_path_allowed` (shared by `read_file.rs` and `write_file.rs`) —
//     performs the `starts_with` containment check that is the actual
//     traversal-defense gate.
//
// The harnesses below therefore target BOTH functions and prove the property
// against the real code rather than the stale skeleton signature. The
// skeleton's phantom signature is surfaced to the orchestrator as a spec-drift
// finding for the architect (VP content is architect-owned; not editable here).
//
// # The injectable-canonicalize seam and its model (BC-2.07.001 EC-007)
//
// `resolve_path_for_allowlist` accepts an injectable `canonicalize_fn` so the
// algorithm is testable/provable without a live sandboxed filesystem. Real
// `std::fs::canonicalize` performs I/O and cannot be model-checked, so these
// harnesses inject `model_canonicalize`: a pure, deterministic model of the
// syscall over a fixed modeled filesystem. The model is faithful to the two
// properties the real syscall guarantees and that the algorithm depends on:
//   1. It succeeds only for a path that denotes an EXISTING object reached by
//      traversing existing directories (so a `..` requires its parent dir to
//      exist, exactly like the kernel), and
//   2. It returns a lexically-normalized, `..`-free, absolute path.
//
// # Bounds (Kani requires finite input space)
//
// Symbolic input is the path STRUCTURE: a fixed existing base (`/r/b`) plus a
// symbolic tail of up to `TAIL_MAX` components, each drawn from the finite
// vocabulary `COMPONENT_VOCAB` which includes the traversal token `..`, the
// no-op `.`, an existing subdir name `s`, and an absent name `x`. Component
// CONTENT is concrete; only WHICH components appear (and how many) is
// symbolic, so Kani explores a bounded tree of concrete path executions. This
// keeps the `Path`/`PathBuf` byte-level operations concrete and tractable
// while still covering every traversal shape up to depth `TAIL_MAX`.
// ---------------------------------------------------------------------------

#[cfg(kani)]
mod kani_proofs {
    use super::{PathAllowDecision, check_path_allowed, resolve_path_for_allowlist};
    use std::path::{Component, Path, PathBuf};

    /// Maximum number of symbolic tail components appended to the base.
    const TAIL_MAX: usize = 3;

    /// Finite component vocabulary for the symbolic tail. Includes the
    /// traversal token `..`, the no-op `.`, an existing subdir name `s`
    /// (so the `..`-collapses-against-existing-ancestor case is covered),
    /// and an absent name `x` (so the absent-tail case is covered).
    const COMPONENT_VOCAB: [&str; 4] = ["s", "x", "..", "."];

    /// The modeled filesystem: the set of existing directories. Everything
    /// else is absent. `/r/b/s` is nested under the base so harnesses exercise
    /// `..` both where it collapses (parent exists) and where the walk
    /// short-circuits (parent absent).
    fn dir_exists(p: &Path) -> bool {
        matches!(
            p.to_str(),
            Some("/") | Some("/r") | Some("/r/b") | Some("/r/b/s")
        )
    }

    /// Pure, deterministic model of `std::fs::canonicalize` over the modeled
    /// filesystem. Succeeds iff `p` is absolute and denotes an existing object
    /// reached by traversing only existing directories (a `..` requires the
    /// current dir to exist, matching the real syscall). Returns the
    /// lexically-normalized, `..`-free absolute path.
    fn model_canonicalize(p: &Path) -> std::io::Result<PathBuf> {
        let not_found = || std::io::Error::from(std::io::ErrorKind::NotFound);
        let mut comps = p.components();
        // Model only handles absolute paths (all dispatcher targets are
        // resolved to absolute before the allowlist check).
        match comps.next() {
            Some(Component::RootDir) => {}
            _ => return Err(not_found()),
        }
        let mut cur = PathBuf::from("/");
        for c in comps {
            match c {
                Component::CurDir => { /* `.` is a no-op */ }
                Component::ParentDir => {
                    // Ascending requires the current directory to exist.
                    if !dir_exists(&cur) {
                        return Err(not_found());
                    }
                    cur.pop();
                }
                Component::Normal(name) => {
                    // Descending requires the current directory to exist.
                    if !dir_exists(&cur) {
                        return Err(not_found());
                    }
                    cur.push(name);
                }
                // RootDir/Prefix mid-path: not producible by our builder.
                _ => return Err(not_found()),
            }
        }
        // The final object must itself exist for canonicalize to succeed.
        if !dir_exists(&cur) {
            return Err(not_found());
        }
        Ok(cur)
    }

    /// Pick one symbolic component from the finite vocabulary.
    fn any_component() -> &'static str {
        let idx: usize = kani::any();
        kani::assume(idx < COMPONENT_VOCAB.len());
        COMPONENT_VOCAB[idx]
    }

    /// Build `/r/b` plus a symbolic tail of up to `TAIL_MAX` components.
    fn symbolic_target() -> PathBuf {
        let n: usize = kani::any();
        kani::assume(n <= TAIL_MAX);
        let mut target = PathBuf::from("/r/b");
        for _ in 0..n {
            target.push(any_component());
        }
        target
    }

    /// VP-097 H1 — traversal safety: a resolved path never contains `..`.
    ///
    /// Task 9 clause "the synthesized canonical path ... contains no `..`
    /// components." This is the load-bearing traversal-defense theorem: for
    /// ANY target structure (including arbitrary `..` sequences), if
    /// `resolve_path_for_allowlist` returns `Some(r)`, then `r` has zero
    /// `ParentDir` components — so no `..` can survive into the path that the
    /// `starts_with` allowlist check (and later `File::open`) sees.
    ///
    /// Why it holds: `resolve` collects tail components via `cur.file_name()`,
    /// which returns `None` for any `cur` ending in `..`, short-circuiting the
    /// whole function to `None`. Every surviving tail component is therefore a
    /// `Normal` name, and the anchor is `model_canonicalize`'s `..`-free output.
    #[kani::proof]
    #[kani::unwind(20)]
    fn verify_resolved_path_has_no_parent_dir_component() {
        let target = symbolic_target();
        if let Some(resolved) = resolve_path_for_allowlist(&target, model_canonicalize) {
            let has_parent_dir = resolved
                .components()
                .any(|c| matches!(c, Component::ParentDir));
            kani::assert(
                !has_parent_dir,
                "VP-097 H1: a resolved (synthesized canonical) path must contain \
                 no `..` components — traversal defense (BC-2.07.001 Invariant 1)",
            );
        }
    }

    /// VP-097 H2 — containment gate: `Allowed` implies `starts_with` a prefix.
    ///
    /// Task 9 clause "starts_with an allowed prefix" + VP-097 §Property
    /// Statement (`Ok(p)` permitted only when some allowlist prefix is a
    /// `starts_with` prefix of `p`). Proves `check_path_allowed` returns
    /// `Allowed` ONLY when the resolved target is contained under a resolved
    /// allowlist prefix — i.e. no `..`-escape is ever accepted. The re-derived
    /// containment is sound because `model_canonicalize` is a pure,
    /// deterministic function, so the harness recomputes the exact values the
    /// function computed internally.
    #[kani::proof]
    #[kani::unwind(20)]
    fn verify_allowed_implies_starts_with_prefix() {
        let target = symbolic_target();
        // Fixed existing prefix; `base` is irrelevant because the prefix is
        // absolute (check_path_allowed only joins `base` onto relative prefixes).
        let allow = ["/r/b/s".to_string()];
        let base = Path::new("/");

        let decision = check_path_allowed(&target, &allow, base, model_canonicalize);

        if matches!(decision, PathAllowDecision::Allowed) {
            let resolved = resolve_path_for_allowlist(&target, model_canonicalize)
                .expect("Allowed decision requires the target to resolve to Some");
            let resolved_prefix =
                resolve_path_for_allowlist(Path::new("/r/b/s"), model_canonicalize)
                    .expect("Allowed decision requires the prefix to resolve to Some");
            kani::assert(
                resolved.starts_with(&resolved_prefix),
                "VP-097 H2: check_path_allowed may return Allowed only when the \
                 resolved target starts_with a resolved allowlist prefix \
                 (BC-2.02.011 EC-001 containment gate)",
            );
        }
    }

    /// VP-097 H3 — escape witness (non-vacuity + positive traversal defense).
    ///
    /// A concrete escaping absent target `/r/b/s/../x` under allow=[`/r/b/s`]:
    /// the `..` collapses against the existing `s` ancestor to yield
    /// `/r/b/x` (outside `s`), which fails the `starts_with` check and is
    /// DENIED. Also witnesses that a legitimate in-prefix target IS accepted,
    /// so H2's implication is not vacuously satisfied.
    #[kani::proof]
    #[kani::unwind(20)]
    fn verify_escape_is_denied_and_legit_is_allowed() {
        let allow = ["/r/b/s".to_string()];
        let base = Path::new("/");

        // Escape: /r/b/s/../x resolves to /r/b/x, outside /r/b/s → denied.
        let escaping = Path::new("/r/b/s/../x");
        let escape_decision = check_path_allowed(escaping, &allow, base, model_canonicalize);
        kani::assert(
            matches!(escape_decision, PathAllowDecision::DeniedNotAllowed),
            "VP-097 H3: escaping target /r/b/s/../x must be DeniedNotAllowed \
             (the .. canonicalizes out of the allowed prefix)",
        );

        // Legit: the existing allowed directory itself is accepted.
        let legit = Path::new("/r/b/s");
        let legit_decision = check_path_allowed(legit, &allow, base, model_canonicalize);
        kani::assert(
            matches!(legit_decision, PathAllowDecision::Allowed),
            "VP-097 H3: in-prefix target /r/b/s must be Allowed (non-vacuity witness)",
        );
    }

    /// VP-097 H4 — determinism of the resolver under a pure canonicalize seam.
    ///
    /// Two calls on identical inputs yield identical results. Underpins the
    /// soundness of H2's re-derivation (the harness recomputes internal values).
    #[kani::proof]
    #[kani::unwind(20)]
    fn verify_resolve_is_deterministic() {
        let target = symbolic_target();
        let r1 = resolve_path_for_allowlist(&target, model_canonicalize);
        let r2 = resolve_path_for_allowlist(&target, model_canonicalize);
        kani::assert(
            r1 == r2,
            "VP-097 H4: resolve_path_for_allowlist must be deterministic for \
             identical inputs under a pure canonicalize seam",
        );
    }
}

// ---------------------------------------------------------------------------
// S-19.03 Red Gate tests for path_util::resolve_path_for_allowlist
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// test_S19_03_T001_helper_absent_target_under_existing_parent_returns_some
    ///
    /// T-001 support (AC-001): `resolve_path_for_allowlist` with an absent target file
    /// whose PARENT directory exists must return `Some(synthesized canonical path)`.
    /// The rejoin algorithm canonicalizes the parent, then appends the absent filename.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` body is `todo!()` (S-19.03 stub).
    ///
    /// Traces to: BC-2.07.001 part b (rejoin algorithm); S-19.03 AC-001.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T001_helper_absent_target_under_existing_parent_returns_some() {
        let dir = tempfile::tempdir().unwrap();
        let absent_target = dir.path().join("wave-state.yaml");
        assert!(!absent_target.exists(), "test setup: target must not exist");
        assert!(dir.path().exists(), "test setup: parent must exist");

        let result = resolve_path_for_allowlist(&absent_target, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "T-001 AC-001: absent file with existing parent must return \
             Some(synthesized_canonical_path); got None. Rejoin algorithm must walk \
             ancestors and rejoin the absent tail onto the deepest existing ancestor."
        );
        let resolved = result.unwrap();
        assert_eq!(
            resolved.file_name().and_then(|n| n.to_str()),
            Some("wave-state.yaml"),
            "T-001: synthesized canonical path must end with the original target filename"
        );
    }

    /// test_S19_03_EC007_mock_canonicalize_all_fail_returns_none
    ///
    /// T-001 Negative Control B (BC-2.07.001 EC-007): when the injected canonicalize
    /// function returns `Err` for every path, `resolve_path_for_allowlist` must return
    /// `None`. The caller (path_allowed) then emits `internal.capability_denied` with
    /// `reason=path_resolution_failed` (NOT `path_not_allowed`).
    ///
    /// On production Unix filesystems this branch is structurally unreachable
    /// (the root `/` always canonicalizes), but it MUST be testable via the
    /// injectable `canonicalize_fn` parameter per BC-2.07.001 EC-007.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`.
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 AC-001 negative-control B.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_EC007_mock_canonicalize_all_fail_returns_none() {
        let target = Path::new(".factory/wave-state.yaml");
        let mock_fail = |_p: &Path| -> std::io::Result<PathBuf> {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        };

        let result = resolve_path_for_allowlist(target, mock_fail);
        assert!(
            result.is_none(),
            "EC-007: when mock canonicalize returns Err for ALL ancestors, \
             resolve_path_for_allowlist must return None — the caller must then \
             emit reason=path_resolution_failed (not path_not_allowed)."
        );
    }

    /// test_S19_03_absent_path_with_dotdot_in_tail_still_resolves_to_some
    ///
    /// Traversal defense integration (BC-2.07.001 Invariant 1): an absent target that
    /// contains `..` in the absent-tail portion still resolves to `Some` — the `..`
    /// is absorbed by the real-filesystem ancestor canonicalization, producing a
    /// synthesized canonical path that the `starts_with` check in `path_allowed` can
    /// then correctly reject (or accept). The traversal defense is the `starts_with`
    /// check, not this function.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`.
    ///
    /// Traces to: BC-2.07.001 Invariant 1 (traversal defense via starts_with);
    ///            BC-2.02.011 EC-001.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_absent_path_with_dotdot_in_tail_still_resolves_to_some() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subdir");
        std::fs::create_dir_all(&sub).unwrap();
        // Target: subdir/../wave-state.yaml — `..` in the absent-tail region.
        let target = sub.join("..").join("wave-state.yaml");
        assert!(
            !target.exists(),
            "test setup: target with .. must not exist"
        );

        let result = resolve_path_for_allowlist(&target, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "Traversal defense: path with '..' in absent tail must still return Some — \
             the starts_with check in path_allowed is where traversal escapes are caught."
        );
        // After implementation: the synthesized canonical path must be under dir (not sub),
        // because the .. was absorbed by the ancestor canonicalization.
        let resolved = result.unwrap();
        let canonical_dir = dir.path().canonicalize().unwrap();
        assert!(
            resolved.starts_with(&canonical_dir),
            "Traversal: synthesized path {:?} must be under the canonical dir {:?} \
             (.. absorbed during ancestor walk)",
            resolved,
            canonical_dir
        );
    }

    /// test_S19_03_P1_001_escape_rejection_absent_path_resolves_outside_prefix
    ///
    /// F-S1903-P1-001 (BC-2.07.001 EC-003 canonical vector, resolve level):
    ///
    /// An ABSENT target whose `..` components cross OUT OF the declared prefix
    /// (e.g. `.factory/../secrets/key` with allow=[`.factory/`]) must:
    ///   1. Still return `Some` — the function resolves via ancestor-walk+rejoin.
    ///   2. Return a path that is OUTSIDE the `.factory/` prefix so that the
    ///      `starts_with` check in `check_path_allowed` returns `DeniedNotAllowed`.
    ///
    /// This test covers the escaping case that was missing in the red-gate suite.
    /// The stays-inside dotdot case is covered by
    /// `test_S19_03_absent_path_with_dotdot_in_tail_still_resolves_to_some` above.
    ///
    /// Traces to: BC-2.07.001 EC-003; BC-2.02.011 EC-001;
    ///            S-19.03 adversary pass-1 F-S1903-P1-001.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_P1_001_escape_rejection_absent_path_resolves_outside_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        let secrets_dir = dir.path().join("secrets");
        std::fs::create_dir_all(&factory_dir).unwrap();
        std::fs::create_dir_all(&secrets_dir).unwrap();

        // Target: .factory/../secrets/key — attempts to escape .factory/ into secrets/.
        // The file does not exist.
        let target = factory_dir.join("..").join("secrets").join("key");
        assert!(
            !target.exists(),
            "test setup: escaping absent target must not exist"
        );

        let result = resolve_path_for_allowlist(&target, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "P1-001 EC-003 (resolve level): escaping absent path must return Some — \
             the `..` is absorbed by ancestor canonicalization. The starts_with check \
             in check_path_allowed is where traversal escapes are detected and denied."
        );

        let resolved = result.unwrap();
        let canonical_factory = factory_dir.canonicalize().unwrap();

        // The critical assertion: resolved path must be OUTSIDE .factory/
        // so that starts_with(.factory/) returns false → DeniedNotAllowed.
        assert!(
            !resolved.starts_with(&canonical_factory),
            "P1-001 EC-003 (resolve level): escaping absent path {:?} must resolve to a \
             canonical path OUTSIDE the .factory/ prefix {:?}; got {:?}. \
             The ancestor-walk canonicalized .factory/../secrets → secrets/, so the \
             result must start with secrets/ not .factory/. \
             The starts_with check in check_path_allowed must deny this as DeniedNotAllowed.",
            target,
            canonical_factory,
            resolved
        );

        // Confirm it resolved into secrets/ as expected.
        let canonical_secrets = secrets_dir.canonicalize().unwrap();
        assert!(
            resolved.starts_with(&canonical_secrets),
            "P1-001 EC-003: escaping path must canonicalize into secrets/ ({:?}), got {:?}",
            canonical_secrets,
            resolved
        );
    }
}
