//! VP-070 kani harness — path-matching purity and determinism.
//!
//! Verifies that `matches_canonical` is pure and deterministic, and that
//! specific safety invariants hold unconditionally (BC-4.11.001 invariant 2).
//!
//! # BC traces
//! - BC-4.11.001 invariant 2: enforcement_level respected per entry; not all block
//! - BC-4.11.001 PC7: non-.factory/ paths always return early (no match)
//! - VP-070: path-matching is pure and deterministic
//!
//! # How to run
//! `cargo kani -p validate-artifact-path` (requires kani toolchain)
//!
//! # How this file is used in Step 3 (Red Gate)
//! The `cargo test` build includes this file. Under `cargo test` the
//! `#[cfg(kani)]` blocks are excluded, so no compilation failure occurs.
//! The non-kani test `test_kani_harness_file_structure` below acts as a
//! compile-time presence check that fails with an assertion error
//! (not a todo! panic) until the production functions exist.
//!
//! AC trace: AC-002 traces to BC-4.11.001 invariant 2 + VP-070 kani.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

// Under cargo test (not cargo kani), we import the production types to
// verify the test file compiles alongside the stub.
use std::panic;
use validate_artifact_path::{MatchResult, PathRegistry, RegistryEntry, matches_canonical};

// ---------------------------------------------------------------------------
// Cargo-test equivalents of the kani proofs (compile + Red Gate check)
//
// These tests exercise the same behavioral properties as the kani proofs but
// run under `cargo test`. They fail with assertion errors because the
// production functions are todo!().
// ---------------------------------------------------------------------------

/// Build a single-entry PathRegistry with enforcement_level=block for kani-style testing.
/// Constructs the registry directly (no YAML parse) to avoid depending on load_registry.
fn fixture_single_entry_block() -> PathRegistry {
    let entry = RegistryEntry {
        artifact_type: "behavioral-contract".to_string(),
        canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
            .to_string(),
        description: "Behavioral contract spec".to_string(),
        enforcement_level: "block".to_string(),
    };
    PathRegistry {
        version: 1,
        artifacts: vec![entry],
    }
}

/// Build a single-entry PathRegistry with enforcement_level=advisory.
fn fixture_advisory_only() -> PathRegistry {
    let entry = RegistryEntry {
        artifact_type: "behavioral-contract".to_string(),
        canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
            .to_string(),
        description: "Behavioral contract spec".to_string(),
        enforcement_level: "advisory".to_string(),
    };
    PathRegistry {
        version: 1,
        artifacts: vec![entry],
    }
}

/// AC-002 / VP-070 Proof 1 (cargo test equivalent):
/// matches_canonical is deterministic — same (path, registry) always yields same MatchResult.
/// Tests a fixed set of representative paths.
#[test]
fn test_BC_4_11_001_vp070_proof1_determinism_cargo_test_equivalent() {
    let registry = fixture_single_entry_block();
    let test_paths = [
        ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
        ".factory/feature-deltas/F1-delta.md",
        "src/lib.rs",
        "",
        ".factory/specs/prd.md",
    ];
    for path in &test_paths {
        let r1 = panic::catch_unwind({
            let reg = registry.clone();
            let p = path.to_string();
            move || matches_canonical(&p, &reg)
        });
        let r2 = panic::catch_unwind({
            let reg = registry.clone();
            let p = path.to_string();
            move || matches_canonical(&p, &reg)
        });
        assert!(
            r1.is_ok(),
            "VP-070 Proof 1 (cargo test): matches_canonical panicked on first call for path '{}'. \
             Must not panic — pure function. Production unimplemented.",
            path
        );
        assert!(
            r2.is_ok(),
            "VP-070 Proof 1 (cargo test): matches_canonical panicked on second call for path '{}'. \
             Must not panic — pure function. Production unimplemented.",
            path
        );
        if let (Ok(result1), Ok(result2)) = (r1, r2) {
            assert_eq!(
                result1, result2,
                "VP-070 Proof 1 (cargo test): matches_canonical must be deterministic for path '{}'. \
                 Got different results on two calls with identical inputs.",
                path
            );
        }
    }
}

/// AC-002 / VP-070 Proof 2 (cargo test equivalent):
/// Non-.factory/ paths must always return MatchResult::NoMatch.
#[test]
fn test_BC_4_11_001_vp070_proof2_non_factory_path_returns_nomatch() {
    let registry = fixture_single_entry_block();
    let non_factory_paths = [
        "src/lib.rs",
        "",
        "Cargo.toml",
        "README.md",
        "crates/hook-plugins/validate-artifact-path/src/lib.rs",
        // These all lack the ".factory/" prefix
        "factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md", // missing leading dot
        "specs/behavioral-contracts/ss-04/BC-4.11.001.md",
    ];
    for path in &non_factory_paths {
        let result = panic::catch_unwind({
            let reg = registry.clone();
            let p = path.to_string();
            move || matches_canonical(&p, &reg)
        });
        assert!(
            result.is_ok(),
            "VP-070 Proof 2: matches_canonical panicked for non-.factory/ path '{}'. \
             Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::NoMatch,
                "VP-070 Proof 2: non-.factory/ path '{}' must always return MatchResult::NoMatch \
                 (BC-4.11.001 PC7 — hook has no opinion on writes outside .factory/)",
                path
            );
        }
    }
}

/// AC-002 / VP-070 Proof 3 (cargo test equivalent):
/// Empty path must return MatchResult::NoMatch.
#[test]
fn test_BC_4_11_001_vp070_proof3_empty_path_returns_nomatch() {
    let registry = fixture_single_entry_block();
    let result = panic::catch_unwind(move || matches_canonical("", &registry));
    assert!(
        result.is_ok(),
        "VP-070 Proof 3: matches_canonical panicked for empty path. Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::NoMatch,
            "VP-070 Proof 3: empty path must return MatchResult::NoMatch \
             (cannot match any .factory/ prefix)"
        );
    }
}

/// AC-002 / VP-070 Proof 4 (cargo test equivalent):
/// Advisory-only registry must never produce MatchResult::Block from matches_canonical.
/// MatchResult::Block means enforcement_level=block; advisory entries produce Advisory.
#[test]
fn test_BC_4_11_001_vp070_proof4_advisory_registry_never_produces_block() {
    let registry = fixture_advisory_only();
    let test_paths = [
        ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md", // would match if block
        ".factory/feature-deltas/F1-delta.md",                      // no match
        "src/lib.rs",                                               // non-.factory/
        "",
    ];
    for path in &test_paths {
        let result = panic::catch_unwind({
            let reg = registry.clone();
            let p = path.to_string();
            move || matches_canonical(&p, &reg)
        });
        assert!(
            result.is_ok(),
            "VP-070 Proof 4: matches_canonical panicked for path '{}' with advisory-only registry. \
             Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_ne!(
                mr,
                MatchResult::Block,
                "VP-070 Proof 4: advisory-only registry must never return MatchResult::Block \
                 for path '{}'. Block variant is reserved for enforcement_level=block entries.",
                path
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Kani proof harnesses (cargo kani only)
// These mirror the four VP-070 proofs with kani::any() bounded inputs.
// ---------------------------------------------------------------------------

#[cfg(kani)]
mod kani_proofs_vp070 {
    use super::*;
    use validate_artifact_path::{MatchResult, PathRegistry, RegistryEntry, matches_canonical};

    fn make_single_entry_block_registry() -> PathRegistry {
        PathRegistry {
            version: 1,
            artifacts: vec![RegistryEntry {
                artifact_type: "behavioral-contract".to_string(),
                canonical_path_pattern:
                    ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
                description: "Behavioral contract spec".to_string(),
                enforcement_level: "block".to_string(),
            }],
        }
    }

    fn make_advisory_only_registry() -> PathRegistry {
        PathRegistry {
            version: 1,
            artifacts: vec![RegistryEntry {
                artifact_type: "behavioral-contract".to_string(),
                canonical_path_pattern:
                    ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md".to_string(),
                description: "Behavioral contract spec".to_string(),
                enforcement_level: "advisory".to_string(),
            }],
        }
    }

    /// VP-070 Proof 1: matches_canonical is deterministic.
    /// Same (path, registry) always yields same MatchResult.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_match_path_is_deterministic() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);
        let registry = make_single_entry_block_registry();
        let decision_1 = matches_canonical(&path, &registry);
        let decision_2 = matches_canonical(&path, &registry);
        kani::assert(
            decision_1 == decision_2,
            "VP-070 Proof 1: matches_canonical must be deterministic: \
             same inputs must yield same MatchResult (BC-4.11.001 invariant 2)",
        );
    }

    /// VP-070 Proof 2: Non-.factory/ paths must return MatchResult::NoMatch.
    /// "Non-.factory/" means neither a relative .factory/ path (starts_with(".factory/"))
    /// nor an absolute path whose components include .factory/ (contains("/.factory/")).
    /// After 8b4f697f introduced absolute-path matching, the prior assumption
    /// `!starts_with(".factory/")` was insufficient — an absolute path like
    /// `/abs/proj/.factory/specs/foo.md` passed the assume but produced Block, not NoMatch.
    /// (F-P19-002)
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_non_factory_path_always_returns_nomatch() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);
        kani::assume(
            !path.starts_with(".factory/")
                && !path.contains("/.factory/"),
        );
        let registry = make_single_entry_block_registry();
        let decision = matches_canonical(&path, &registry);
        kani::assert(
            matches!(decision, MatchResult::NoMatch),
            "VP-070 Proof 2: genuinely out-of-scope paths must always return MatchResult::NoMatch \
             (BC-4.11.001 PC7 — hook scoped to .factory/ only)",
        );
    }

    /// VP-070 Proof 3: Empty path must return MatchResult::NoMatch.
    #[kani::proof]
    fn proof_vp070_empty_path_returns_nomatch() {
        let registry = make_single_entry_block_registry();
        let decision = matches_canonical("", &registry);
        kani::assert(
            matches!(decision, MatchResult::NoMatch),
            "VP-070 Proof 3: empty path must return MatchResult::NoMatch",
        );
    }

    /// VP-070 Proof 4: Advisory-only registry must never produce MatchResult::Block.
    #[kani::proof]
    #[kani::unwind(16)]
    fn proof_vp070_advisory_only_registry_never_blocks() {
        let path: String = kani::any();
        kani::assume(path.len() <= 64);
        let registry = make_advisory_only_registry();
        let decision = matches_canonical(&path, &registry);
        kani::assert(
            !matches!(decision, MatchResult::Block),
            "VP-070 Proof 4: advisory-only registry must never produce MatchResult::Block \
             (BC-4.11.001 invariant 2 — enforcement_level respected per entry)",
        );
    }
}
