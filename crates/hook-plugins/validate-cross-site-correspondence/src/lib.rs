//! validate-cross-site-correspondence — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write/MultiEdit to governed paths that contains cross-site
//! value-correspondence violations:
//!
//! **Class A Arm1 (BC file write):** BC `version:` vs BC-INDEX.md row version cell.
//! **Class A Arm2 (story file write):** story body BC-table version citations vs BC frontmatter.
//! **Class B Arm1 (story file write):** story `input-hash:` vs STORY-INDEX catalog row vs blockquote.
//! **Class B Arm2 (STORY-INDEX.md write):** internal catalog vs blockquote hash parity.
//! **Class D (cycle artifact write):** finding-ID namespace format advisory (NEVER blocks).
//! **Class E1 (BC/VP/story/epic write):** `version:` vs `last_amended:` outer prefix mismatch.
//! **Class E2 (BC/VP/story/epic write):** `modified:` sequence date monotonicity.
//!
//! # Governing BC
//! BC-5.39.010 v1.2 — five-arm PostToolUse cross-site value-correspondence gate.
//!
//! # Architecture
//! - **Tier 2A** (ADR-035 §Decision 1): PostToolUse read-only validator.
//! - PostToolUse ONLY — never PreToolUse (BC-5.39.010 invariant 2).
//! - No writes (BC-5.39.010 invariant 1).
//! - No `read_dir`, no `list_dir` enumeration (BC-5.39.010 invariant 3 / ADR-035 §Alternatives).
//! - Fail-closed on primary-target `CapabilityDenied` (BC-5.39.010 invariant 4 / BC-5.39.008 v1.6).
//! - `on_error = "continue"`: fuel exhaustion → silent non-finding, not spurious block (ADR-035 §Decision 5).
//! - `is_char_boundary()` guards on all byte-index slicing (BC-5.39.010 invariant 9).
//!
//! # Forbidden dependencies (enforced by Cargo.toml absence):
//! - `serde_yaml`: deprecated/archived 2024; TD #72 migrated workspace to `serde_norway`.
//! - `wasmtime`: only dispatcher binary links wasmtime.
//! - `regex`: WASM fuel budget; hand-rolled scanning mandated (ADR-035 §Decision 5).

pub mod arm_a1;
pub mod arm_a2;
pub mod arm_b;
pub mod arm_d;
pub mod arm_e;
pub mod dispatch;
pub mod frontmatter;

use vsdd_hook_sdk::{
    HookPayload, HookResult,
    host::{self},
};

/// Read cap for BC file and story file primary reads.
/// BC-5.39.010 AC-019: `max_bytes = 524288`, `timeout_ms = 3000`.
/// Separate from cycle artifact cap — BC/story files are bounded by spec
/// (F-S2107-P1C-002: former PRIMARY_READ_MAX_BYTES=1 MiB was 2× spec cap).
const BC_STORY_PRIMARY_MAX_BYTES: u32 = 524_288;
const BC_STORY_PRIMARY_TIMEOUT_MS: u32 = 3_000;

/// Read cap for cycle artifact primary reads.
/// BC-5.39.010 AC-019: `max_bytes = 2097152`, `timeout_ms = 5000`.
/// Larger cap required for large cycle artifacts (e.g., lessons.md ≤4000 lines).
/// Former PRIMARY_READ_MAX_BYTES=1 MiB caused OutputTooLarge → Block on 1–2 MiB
/// artifacts, violating invariant 6 (Class D advisory-only, F-S2107-P1C-003).
const CYCLE_ARTIFACT_PRIMARY_MAX_BYTES: u32 = 2_097_152;
const CYCLE_ARTIFACT_PRIMARY_TIMEOUT_MS: u32 = 5_000;

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
pub const HOST_ABI_VERSION: u32 = 1;

/// A blocking cross-site correspondence violation.
///
/// Carries a human-readable `description` used verbatim in the combined block message
/// and the arm `class` for message prefixing.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description citing the arm class, values, and policy anchor.
    pub description: String,
}

/// A non-blocking advisory warning emitted via `host::log_warn`.
///
/// Advisories are always Continue (never block). Used for:
/// - Class D finding-ID namespace format anomalies (invariant 6: advisory-only).
/// - Bootstrap-ordering conditions (v1.0 BC not yet in INDEX, STORY-INDEX absent/NotFound).
/// - Unparseable `last_amended:` format (precondition 37 last sentence).
#[derive(Debug, Clone)]
pub struct Advisory {
    /// Human-readable advisory message for `host::log_warn`.
    pub message: String,
}

/// PostToolUse entry point called by `vsdd_hook_sdk::__internal::run`.
///
/// # Dispatch flow
/// 1. Extract `file_path` from `payload.tool_input`. If absent: Continue + log_warn.
/// 2. Classify the file via `dispatch::*`. If unclassified (all classifiers return
///    false/None): return Continue immediately — no read attempted (F-S2107-P1C-001).
///    This ensures paths outside the hook's scope (`.factory/STATE.md`, source files,
///    `CLAUDE.md`, `policies.yaml`, etc.) never produce spurious blocks.
/// 3. Read the primary target via `host::read_file` with branch-specific caps
///    (BC-5.39.010 AC-019 / F-S2107-P1C-002 / F-S2107-P1C-003):
///    - STORY-INDEX.md: B2_MAX_BYTES / B2_TIMEOUT_MS
///    - Cycle artifact: CYCLE_ARTIFACT_PRIMARY_MAX_BYTES / CYCLE_ARTIFACT_PRIMARY_TIMEOUT_MS
///    - BC/story/VP/epic: BC_STORY_PRIMARY_MAX_BYTES / BC_STORY_PRIMARY_TIMEOUT_MS
///    On HostError for cycle artifacts: advisory + Continue (invariant 6 — Class D
///    is advisory-only, never blocking). On HostError for all other classified targets:
///    BLOCK (fail-closed per invariant 4 + BC-5.39.008 v1.6).
///    On UTF-8 decode failure: Continue (fail-open, invariant 9).
/// 4. Dispatch to applicable arms based on classification:
///    - STORY-INDEX.md: Arm B2 only.
///    - Cycle artifact: Arm D advisory (returns Continue).
///    - BC file: Arm A1 + Class E.
///    - Story file: Arm A2 + Arm B1 + Class E.
///    - All BC/VP/story/epic files: Class E (frontmatter parity).
/// 5. Collect ALL violations from all arms.
/// 6. Emit ALL advisories via `host::log_warn` (regardless of block state).
/// 7. If violations non-empty: return ONE combined `HookResult::block_with_fix`.
///    If violations empty: return `HookResult::Continue`.
///
/// # BC trace
/// BC-5.39.010 preconditions 1-39; postconditions 1-23; invariants 1-11.
/// BC-5.39.010 invariant 7: arms MUST NOT suppress each other.
/// BC-5.39.010 invariant 9: `is_char_boundary()` on byte-index slicing.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    // Step 1: extract file_path from tool_input
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "validate-cross-site-correspondence: file_path absent from tool_input — skipping",
            );
            return HookResult::Continue;
        }
    };

    // Step 2: classify first; return Continue for unclassified paths (F-S2107-P1C-001).
    // Classify before any read — the registry fires on tool="^(Edit|Write|MultiEdit)$"
    // with path_allow covering only four prefixes. Paths outside those prefixes
    // (e.g., .factory/STATE.md, crates/**, CLAUDE.md) get CapabilityDenied from
    // host::read_file, causing spurious blocks if we read before classifying.
    let is_si = dispatch::is_story_index(&file_path);
    let cycle_kind = dispatch::is_cycle_artifact(&file_path);
    let is_bc = dispatch::is_bc_file(&file_path);
    let is_story = dispatch::is_story_file(&file_path);
    let is_fpm = dispatch::is_frontmatter_parity_target(&file_path);

    if !is_si && cycle_kind.is_none() && !is_bc && !is_story && !is_fpm {
        // Unclassified — outside this hook's scope (PC1/PC9/PC16/PC22/PC28/PC34).
        // No arm fires; Continue immediately without reading the file.
        return HookResult::Continue;
    }

    // Step 3: read primary target with branch-specific cap (BC-5.39.010 AC-019).
    let (primary_max, primary_timeout_ms) = if is_si {
        (
            arm_b::STORY_INDEX_B2_MAX_BYTES,
            arm_b::STORY_INDEX_B2_TIMEOUT_MS,
        )
    } else if cycle_kind.is_some() {
        (
            CYCLE_ARTIFACT_PRIMARY_MAX_BYTES,
            CYCLE_ARTIFACT_PRIMARY_TIMEOUT_MS,
        )
    } else {
        (BC_STORY_PRIMARY_MAX_BYTES, BC_STORY_PRIMARY_TIMEOUT_MS)
    };

    let primary_bytes = match host::read_file(&file_path, primary_max, primary_timeout_ms) {
        Ok(bytes) => bytes,
        Err(e) if cycle_kind.is_some() => {
            // Class D: advisory-only, never blocking (invariant 6).
            // NotFound → advisory + Continue per PC33; any other error → advisory (invariant 6).
            host::log_warn(&format!(
                "validate-cross-site-correspondence [Class D primary-read]: \
                cannot read cycle artifact '{file_path}': {e:?}. \
                Advisory-only per BC-5.39.010 invariant 6."
            ));
            return HookResult::Continue;
        }
        Err(e) => {
            // Fail-closed: CapabilityDenied or any error on classified non-cycle target → block
            // (BC-5.39.010 invariant 4 + BC-5.39.008 v1.6)
            return combine_violations_into_block(
                "validate-cross-site-correspondence",
                &[Violation {
                    description: format!(
                        "validate-cross-site-correspondence [primary-read] POLICY 14: \
                        cannot read primary target '{file_path}': {e:?}. \
                        Fail-closed per BC-5.39.010 invariant 4. Fix: review and fix all \
                        cross-site correspondence issues listed above, then retry the write."
                    ),
                }],
            );
        }
    };

    // Step 4: decode UTF-8; fail-open on decode failure (invariant 9)
    let content = match std::str::from_utf8(&primary_bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            host::log_warn(&format!(
                "validate-cross-site-correspondence: UTF-8 decode failure on '{}' — skipping (fail-open)",
                file_path
            ));
            return HookResult::Continue;
        }
    };

    // Step 5: classify and dispatch
    let mut violations: Vec<Violation> = Vec::new();
    let mut advisories: Vec<Advisory> = Vec::new();

    // Arm B2: STORY-INDEX.md write
    if is_si {
        let b2_violations = arm_b::run_arm_b2(&content);
        violations.extend(b2_violations);
        // STORY-INDEX.md: no E arm (no version:/last_amended: frontmatter)
        emit_advisories(&advisories);
        return if violations.is_empty() {
            HookResult::Continue
        } else {
            combine_violations_into_block("validate-cross-site-correspondence", &violations)
        };
    }

    // Arm D: cycle artifact advisory (never blocks; primary-read errors already handled above)
    if let Some(kind) = cycle_kind {
        let scoped = arm_d::extract_scope_limited_region(&content, kind);
        let d_advisories = arm_d::run_arm_d(scoped, &file_path);
        for adv in d_advisories {
            host::log_warn(&adv.message);
        }
        return HookResult::Continue;
    }

    // BC file: Arm A1 + Class E
    if is_bc {
        let bc_id = extract_stem_from_path(&file_path);
        let bc_version =
            frontmatter::extract_frontmatter_field(&content, "version").unwrap_or_default();
        let (a1_v, a1_a) = arm_a1::run_arm_a1(&bc_id, &bc_version, &file_path);
        violations.extend(a1_v);
        advisories.extend(a1_a);
    }

    // Story file: Arm A2 + Arm B1 + Class E
    if is_story {
        let story_id = extract_story_id_from_path(&file_path);
        let (a2_v, a2_a) = arm_a2::run_arm_a2(&story_id, &content);
        violations.extend(a2_v);
        advisories.extend(a2_a);

        let (b1_v, b1_a) = arm_b::run_arm_b1(&story_id, &content);
        violations.extend(b1_v);
        advisories.extend(b1_a);
    }

    // Class E (frontmatter parity) for BC/VP/story/epic
    if is_fpm {
        let (e1_v, e1_a) = arm_e::run_arm_e1(&content);
        violations.extend(e1_v);
        advisories.extend(e1_a);

        let e2_v = arm_e::run_arm_e2(&content);
        violations.extend(e2_v);
    }

    // Step 6: emit all advisories, then return
    emit_advisories(&advisories);

    // Step 7: combine violations or continue
    if violations.is_empty() {
        HookResult::Continue
    } else {
        combine_violations_into_block("validate-cross-site-correspondence", &violations)
    }
}

/// Combine a list of violations into a single `HookResult::block_with_fix`.
///
/// Enumerates all violations in a single combined block message per
/// BC-5.39.010 invariant 7 (arms must not suppress each other) and postcondition 23
/// (combined violations → single block).
///
/// # BC trace
/// BC-5.39.010 postcondition 23; invariant 7.
pub fn combine_violations_into_block(hook_name: &str, violations: &[Violation]) -> HookResult {
    let combined = violations
        .iter()
        .enumerate()
        .map(|(i, v)| format!("[{}] {}", i + 1, v.description))
        .collect::<Vec<_>>()
        .join(" | ");
    HookResult::block_with_fix(
        hook_name,
        &combined,
        "review and fix all cross-site correspondence issues listed above, then retry the write",
        "POLICY 14/18",
    )
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Emit advisories via `host::log_warn`.
fn emit_advisories(advisories: &[Advisory]) {
    for adv in advisories {
        host::log_warn(&adv.message);
    }
}

/// Extract the file stem (filename without `.md` extension) from a path.
///
/// Used to derive bc_id from a BC file path like
/// `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md` → `BC-5.39.010`.
fn extract_stem_from_path(file_path: &str) -> String {
    std::path::Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_path)
        .to_string()
}

/// Extract the story ID from a story file path.
///
/// Story paths have the form `.factory/stories/S-21.07-<name>.md`.
/// The story ID is the `S-XX.YY` prefix (first two dash-separated segments).
///
/// For a file named `S-21.07-validate-cross-site-correspondence.md`, returns `S-21.07`.
fn extract_story_id_from_path(file_path: &str) -> String {
    let stem = std::path::Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    // stem = "S-21.07-validate-cross-site-correspondence"
    // Split by '-': ["S", "21.07", "validate", ...]
    let parts: Vec<&str> = stem.splitn(3, '-').collect();
    if parts.len() >= 2 {
        format!("{}-{}", parts[0], parts[1])
    } else {
        stem.to_string()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;
    // BC_STORY_PRIMARY_MAX_BYTES and CYCLE_ARTIFACT_PRIMARY_MAX_BYTES are
    // pub(super) in lib.rs; imported explicitly here for constant-level assertions.
    use super::{BC_STORY_PRIMARY_MAX_BYTES, CYCLE_ARTIFACT_PRIMARY_MAX_BYTES};
    use vsdd_hook_sdk::{HookPayload, HookResult};

    // -----------------------------------------------------------------------
    // T-046 / F-S2107-P1C-001: unclassified paths must return Continue, not block.
    //
    // Current bug: on_post_tool_use reads the primary target file (Step 2) BEFORE
    // it classifies the path (Step 4). In the non-wasm test host, `ffi::read_file`
    // always returns -1 (CapabilityDenied), so EVERY path — including completely
    // unclassified ones like ordinary source files, CLAUDE.md, and critically
    // .factory/STATE.md — produces a blocking HookResult.
    //
    // The most severe consequence: .factory/STATE.md is inside .factory/ but
    // NOT under any of the four `path_allow` prefixes
    // (.factory/specs/behavioral-contracts/, .factory/specs/verification-properties/,
    // .factory/stories/, .factory/cycles/). State-manager writes .factory/STATE.md
    // on every fix burst, so the factory cannot record its own state.
    // .factory/policies.yaml has the same problem.
    //
    // After fix (classify-then-read): classify first; if unclassified, return
    // Continue without calling host::read_file at all.
    //
    // BC-5.39.010: unclassified paths are outside the hook's scope (PC1/PC9/PC16/
    // PC22/PC28/PC34). No arm fires → Continue.
    //
    // RED GATE: current code returns Block (CapabilityDenied on primary read for
    // all paths). assert_eq!(result, HookResult::Continue) FAILS for every case.
    // -----------------------------------------------------------------------

    /// T-046 / F-S2107-P1C-001: unclassified paths must return Continue.
    ///
    /// RED GATE: Step 2 reads before Step 4 classifies → CapabilityDenied → Block.
    /// assert_eq!(result, Continue) FAILS for all four paths (Block ≠ Continue).
    /// After fix (classify-then-read): unclassified → Continue, no read attempted.
    #[test]
    fn test_BC_5_39_010_unclassified_path_returns_continue_not_block() {
        fn payload(file_path: &str) -> HookPayload {
            serde_json::from_value(serde_json::json!({
                "event_name": "PostToolUse",
                "tool_name": "Edit",
                "session_id": "test-session",
                "dispatcher_trace_id": "test-trace",
                "tool_input": {"file_path": file_path}
            }))
            .expect("test payload must parse")
        }

        // These paths are all OUTSIDE the four path_allow prefixes and must
        // never be blocked by this hook.
        let cases: &[&str] = &[
            // Ordinary source file — no .factory component
            "crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs",
            // Repo root doc — no .factory component
            "CLAUDE.md",
            // .factory/STATE.md — inside .factory/ but NOT under any arm's prefix.
            // CRITICAL: state-manager writes this on every burst. Blocking it breaks
            // the factory's ability to record its own state or repair itself.
            ".factory/STATE.md",
            // .factory/policies.yaml — YAML, not .md; never matches any arm classifier
            ".factory/policies.yaml",
        ];

        for path in cases {
            let result = on_post_tool_use(payload(path));
            assert_eq!(
                result,
                HookResult::Continue,
                "unclassified path '{}' must return HookResult::Continue, not Block. \
                F-S2107-P1C-001: classify-first fix required. \
                RED GATE: current Step-2 read fires before classification → \
                CapabilityDenied (non-wasm stub returns -1) → Block.",
                path
            );
        }
    }

    // -----------------------------------------------------------------------
    // T-047 / F-S2107-P1C-002 + HIGH F-S2107-P1C-003: all six read caps pinned.
    //
    // Existing bats AC-019 pinned only four constants (BC_INDEX and BC secondary).
    // Three divergences stayed invisible because the primary read caps were not
    // pinned. This test covers all six BC-5.39.010 read cap spec values.
    //
    // Spec (BC-5.39.010 v1.3, unchanged):
    //   BC file primary:        524288 / 3000 ms
    //   Story file primary:     524288 / 3000 ms
    //   Cycle artifact primary: 2097152 / 5000 ms
    //   BC-INDEX secondary:     1048576 / 3000 ms
    //   STORY-INDEX B1:         1048576 / 3000 ms
    //   STORY-INDEX B2:         2097152 / 5000 ms
    //
    // BC_MAX_BYTES = 524_288 already exists and is correct BUT is only passed
    // for Arm A2 secondary reads (arm_a2::run_arm_a2_for_bc). The primary read
    // in lib.rs Step 2 uses PRIMARY_READ_MAX_BYTES = 1_048_576 for EVERYTHING,
    // producing two divergences:
    //
    //   F-S2107-P1C-002 (BLOCKER): BC/story primary reads use 1 MiB instead of
    //   512 KiB. This is 2× the spec cap; causes false-negative pass-through on
    //   oversized BC files that should be rejected.
    //
    //   F-S2107-P1C-003 (HIGH): Cycle artifact primary reads use 1 MiB instead
    //   of 2 MiB. A cycle artifact between 1 MiB and 2 MiB (e.g., a large
    //   lessons.md) returns OutputTooLarge → Block, violating BC-5.39.010
    //   invariant 6 ("Class D is advisory-only, never blocking"). Also:
    //   NotFound on cycle artifact primary should be advisory + Continue per
    //   PC33, but currently produces Block (all Step-2 errors produce Block
    //   before classification runs).
    //
    //   Non-wasm host note: host::read_file always returns CapabilityDenied (-1)
    //   in the test context, so OutputTooLarge and NotFound behavioral cases
    //   cannot be triggered here. Both are asserted at constant level per brief
    //   (F-S2107-P1C-003 scope).
    //
    // After fix: introduce separate named constants for BC/story primary
    // (= BC_MAX_BYTES = 524_288) and cycle artifact primary (= 2_097_152).
    // Update assertions (9) and (10) below to reference the new constant names.
    // -----------------------------------------------------------------------

    /// T-047 / F-S2107-P1C-002 + HIGH F-S2107-P1C-003: all six read caps pinned.
    ///
    /// RED GATE assertions (9) and (10):
    ///   (9) PRIMARY_READ_MAX_BYTES (1_048_576) == BC_MAX_BYTES (524_288) → FAILS.
    ///   (10) PRIMARY_READ_MAX_BYTES (1_048_576) == STORY_INDEX_B2_MAX_BYTES (2_097_152) → FAILS.
    /// After fix: replace both assertions with the new per-path constants.
    #[test]
    fn test_BC_5_39_010_ac019_extended_all_six_read_caps_fully_pinned() {
        // (1–8) Already-correct constants — regression protection only:
        assert_eq!(
            arm_a1::BC_INDEX_MAX_BYTES,
            1_048_576,
            "BC-INDEX secondary: 1 MiB"
        );
        assert_eq!(
            arm_a1::BC_INDEX_TIMEOUT_MS,
            3_000,
            "BC-INDEX secondary: 3000 ms"
        );
        assert_eq!(
            arm_a1::BC_MAX_BYTES,
            524_288,
            "BC secondary (arm A2): 512 KiB"
        );
        assert_eq!(arm_a1::BC_TIMEOUT_MS, 3_000, "BC secondary: 3000 ms");
        assert_eq!(
            arm_b::STORY_INDEX_B1_MAX_BYTES,
            1_048_576,
            "STORY-INDEX B1: 1 MiB"
        );
        assert_eq!(
            arm_b::STORY_INDEX_B1_TIMEOUT_MS,
            3_000,
            "STORY-INDEX B1: 3000 ms"
        );
        assert_eq!(
            arm_b::STORY_INDEX_B2_MAX_BYTES,
            2_097_152,
            "STORY-INDEX B2: 2 MiB"
        );
        assert_eq!(
            arm_b::STORY_INDEX_B2_TIMEOUT_MS,
            5_000,
            "STORY-INDEX B2: 5000 ms"
        );

        // (9) F-S2107-P1C-002 (BLOCKER — now CLOSED):
        // BC/story file primary reads must use BC_STORY_PRIMARY_MAX_BYTES = 524_288 (512 KiB).
        // Former PRIMARY_READ_MAX_BYTES = 1_048_576 (1 MiB) was 2× the spec cap.
        // Fix: renamed constant BC_STORY_PRIMARY_MAX_BYTES = 524_288 wired to BC/story reads.
        assert_eq!(
            BC_STORY_PRIMARY_MAX_BYTES,
            arm_a1::BC_MAX_BYTES,
            "BC/story primary read cap MUST equal BC_MAX_BYTES=524288. \
            F-S2107-P1C-002 (BLOCKER — closed by BC_STORY_PRIMARY_MAX_BYTES rename)."
        );

        // (10) HIGH F-S2107-P1C-003 + invariant 6 (now CLOSED):
        // Cycle artifact primary reads must use CYCLE_ARTIFACT_PRIMARY_MAX_BYTES = 2_097_152.
        // Former PRIMARY_READ_MAX_BYTES = 1_048_576 caused OutputTooLarge → Block on
        // 1 MiB–2 MiB files, violating invariant 6 (Class D advisory-only).
        // Behavioral tests blocked by non-wasm CapabilityDenied; asserting at constant level.
        assert_eq!(
            CYCLE_ARTIFACT_PRIMARY_MAX_BYTES,
            arm_b::STORY_INDEX_B2_MAX_BYTES,
            "Cycle artifact primary read cap MUST equal STORY_INDEX_B2_MAX_BYTES=2097152. \
            HIGH F-S2107-P1C-003 + invariant 6 (closed by CYCLE_ARTIFACT_PRIMARY_MAX_BYTES)."
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-005: BC-INDEX.md must not be classified as a BC file.
    // dispatch::is_bc_file uses starts_with("BC-") && ends_with(".md") which
    // admits "BC-INDEX.md" as a valid BC path. When admitted, arm A1 tries to
    // read BC-INDEX.md as both primary target AND secondary index, producing
    // spurious version mismatches (F-S2107-P1B-005 + F-S2107-P1B-007 cascade).
    // BC-5.39.010 v1.3 §Classification invariant: index files are excluded.
    // -----------------------------------------------------------------------

    /// T-035 lib-level: BC-INDEX.md must NOT be classified as a BC file (F-S2107-P1B-005).
    ///
    /// RED GATE: current `starts_with("BC-") && ends_with(".md")` matches "BC-INDEX.md".
    /// dispatch::is_bc_file returns true → assert!(!result) FAILS → RED gate.
    /// After fix (exclude by name: file_stem == "BC-INDEX"): returns false → PASSES.
    #[test]
    fn test_BC_5_39_010_dispatch_bc_index_not_bc_file_lib_integration() {
        let result = dispatch::is_bc_file(".factory/specs/behavioral-contracts/BC-INDEX.md");
        assert!(
            !result,
            "BC-INDEX.md must NOT be classified as a BC file — it is the index, \
            not a behavioral contract (F-S2107-P1B-005)"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-006: escaped-pipe version chains in production BC-INDEX rows.
    // arm_a1::run_arm_a1_with_index_result with production-shaped content must
    // correctly identify `1.6` as current when version history is `v1.3 \| v1.6`.
    // Current code: split on '|' → first token "1.3" → "1.3" ≠ "1.6" → violation.
    // After fix: last token "1.6" → match → no violation.
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-006: lib-level escaped-pipe chain integration test.
    ///
    /// RED GATE: current code returns "1.3" → "1.3" ≠ "1.6" → violation → NOT empty.
    /// assert!(violations.is_empty()) FAILS → RED gate.
    /// After fix (last token wins): "1.6" == "1.6" → empty → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_a1_escaped_pipe_chain_stale_blocks_lib_level() {
        let index = concat!(
            "---\ndocument_type: bc-index\n---\n\n",
            "| [BC-5.39.010](ss-05/BC-5.39.010.md) | title | draft | CAP-032 | S-21.07",
            " | v1.3 \\| v1.4 \\| v1.5 \\| v1.6 |\n",
        );
        let (violations, _advisories) = arm_a1::run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Ok(index.as_bytes().to_vec()),
        );
        assert!(
            violations.is_empty(),
            "escaped-pipe chain `v1.3 \\| ... \\| v1.6` with current version '1.6' must not block. \
            Red Gate: current code extracts first token '1.3' → violation (F-S2107-P1B-006)"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1C-014: 15-byte last_amended string rejected by length guard.
    // "2026-07-30 (v2)" — single-digit outer version, no sub-version suffix.
    // BC-5.39.010 v1.3 §E1: this is a valid format.
    // Current code: `if len < 17 { return None }` — 15 < 17 → None.
    // -----------------------------------------------------------------------

    /// T-045 lib-level: extract_last_amended_outer_version must accept 15-byte format.
    ///
    /// RED GATE: `if len < 17 { return None }` → 15 < 17 → None.
    /// assert_eq!(result, Some("2".to_string())) FAILS → RED gate.
    /// After fix (lower threshold to 14): returns Some("2") → PASSES.
    #[test]
    fn test_BC_5_39_010_class_e1_15_byte_last_amended_accepted_lib_level() {
        let result = arm_e::extract_last_amended_outer_version("2026-07-30 (v2)");
        assert_eq!(
            result,
            Some("2".to_string()),
            "15-byte last_amended '2026-07-30 (v2)' must parse to outer version '2'. \
            BC-5.39.010 v1.3 §E1 (F-S2107-P1C-014). \
            Red Gate: len < 17 guard rejects 15-byte strings → None → assertion FAILS"
        );
    }
}
