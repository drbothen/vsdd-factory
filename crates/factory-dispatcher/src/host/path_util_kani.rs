//! VP-097 Kani proof harness — `resolve_path_for_allowlist` traversal defense.
//!
//! Proves the BC-2.07.001 Invariant 1 + BC-2.02.011 EC-001 traversal-defense
//! property: `..` sequences cannot resolve to an `Allowed` path outside the
//! declared allowlist prefixes.
//!
//! # How to run
//! `cargo kani -p factory-dispatcher --harness <name>` (requires the kani
//! toolchain). These harnesses compile only under `cargo kani`; the
//! `#[cfg(kani)]` gate excludes them from `cargo test` / `cargo build` /
//! `cargo clippy`, mirroring the established repo convention (see
//! `partition.rs`, `aggregator.rs`, `validate-artifact-path`).
//!
//! # Mapping to the shipped S-19.03 code (deviation from VP-097 §Proof Harness
//! Skeleton)
//!
//! VP-097 v1.2 §Proof Harness Skeleton is written against a monolithic
//! signature `resolve_path_for_allowlist(path: &str, allowlist: &[&str]) ->
//! Result<PathBuf, CapabilityDenied>` that predates the S-19.03 implementation.
//! The shipped design splits the concern into two functions:
//!
//!   * [`resolve_path_for_allowlist`] — `(target: &Path, canonicalize_fn) ->
//!     Option<PathBuf>`. Pure ancestor-walk + rejoin. Does NOT perform the
//!     allowlist check; by design it returns `Some` even for escaping inputs
//!     (the `..` is absorbed by ancestor canonicalization).
//!   * [`check_path_allowed`] (in `read_file.rs`) — performs the `starts_with`
//!     containment check that is the actual traversal-defense gate.
//!
//! The harnesses below therefore target BOTH functions and prove the property
//! against the real code rather than the stale skeleton signature. The
//! skeleton's phantom signature is surfaced to the orchestrator as a spec-drift
//! finding for the architect (VP content is architect-owned; not editable here).
//!
//! # The injectable-canonicalize seam and its model (BC-2.07.001 EC-007)
//!
//! `resolve_path_for_allowlist` accepts an injectable `canonicalize_fn` so the
//! algorithm is testable/provable without a live sandboxed filesystem. Real
//! `std::fs::canonicalize` performs I/O and cannot be model-checked, so these
//! harnesses inject [`model_canonicalize`]: a pure, deterministic model of the
//! syscall over a fixed modeled filesystem. The model is faithful to the two
//! properties the real syscall guarantees and that the algorithm depends on:
//!   1. It succeeds only for a path that denotes an EXISTING object reached by
//!      traversing existing directories (so a `..` requires its parent dir to
//!      exist, exactly like the kernel), and
//!   2. It returns a lexically-normalized, `..`-free, absolute path.
//!
//! # Bounds (Kani requires finite input space)
//!
//! Symbolic input is the path STRUCTURE: a fixed existing base (`/r/b`) plus a
//! symbolic tail of up to [`TAIL_MAX`] components, each drawn from the finite
//! vocabulary [`COMPONENT_VOCAB`] which includes the traversal token `..`, the
//! no-op `.`, an existing subdir name `s`, and an absent name `x`. Component
//! CONTENT is concrete; only WHICH components appear (and how many) is
//! symbolic, so Kani explores a bounded tree of concrete path executions. This
//! keeps the `Path`/`PathBuf` byte-level operations concrete and tractable
//! while still covering every traversal shape up to depth `TAIL_MAX`.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs (belt-and-suspenders;
// the workspace already sets check-cfg = ["cfg(kani)"]).
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

#[cfg(kani)]
mod proofs {
    use crate::host::path_util::{PathAllowDecision, resolve_path_for_allowlist};
    use crate::host::read_file::check_path_allowed;
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
