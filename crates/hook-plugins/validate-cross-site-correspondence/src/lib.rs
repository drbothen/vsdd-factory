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
    host::{self, HostError},
    HookPayload, HookResult,
};

/// Maximum bytes to read for a primary target file.
/// Used when reading BC files, story files, and cycle artifacts.
const PRIMARY_READ_MAX_BYTES: u32 = 1_048_576;
const PRIMARY_READ_TIMEOUT_MS: u32 = 5_000;

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
/// 2. Read the primary target via `host::read_file`. On `CapabilityDenied` OR any
///    other `HostError` on the primary target: BLOCK (fail-closed per BC-5.39.010
///    invariant 4 + BC-5.39.008 v1.6). On UTF-8 decode failure: Continue (fail-open).
/// 3. Classify the file via `dispatch::classify_file`. If unclassified: Continue.
/// 4. Dispatch to applicable arms based on file classification:
///    - BC file: Arm A1 + Class E.
///    - Story file: Arm A2 + Arm B1 + Class E.
///    - STORY-INDEX.md: Arm B2.
///    - Cycle artifact: Arm D (advisory only).
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

    // Step 2: read primary target; fail-closed on any HostError (invariant 4)
    let primary_max = if dispatch::is_story_index(&file_path) {
        arm_b::STORY_INDEX_B2_MAX_BYTES
    } else {
        PRIMARY_READ_MAX_BYTES
    };
    let primary_bytes = match host::read_file(&file_path, primary_max, PRIMARY_READ_TIMEOUT_MS) {
        Ok(bytes) => bytes,
        Err(e) => {
            // Fail-closed: CapabilityDenied or any error on primary target → block
            return combine_violations_into_block(
                "validate-cross-site-correspondence",
                &[Violation {
                    description: format!(
                        "validate-cross-site-correspondence [primary-read] POLICY 14: \
                        cannot read primary target '{file_path}': {e:?}. \
                        Fail-closed per BC-5.39.010 invariant 4."
                    ),
                }],
            );
        }
    };

    // Step 3: decode UTF-8; fail-open on decode failure (invariant 9)
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

    // Step 4: classify and dispatch
    let mut violations: Vec<Violation> = Vec::new();
    let mut advisories: Vec<Advisory> = Vec::new();

    // Arm B2: STORY-INDEX.md write
    if dispatch::is_story_index(&file_path) {
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

    // Arm D: cycle artifact advisory (never blocks)
    if let Some(kind) = dispatch::is_cycle_artifact(&file_path) {
        let scoped = arm_d::extract_scope_limited_region(&content, kind);
        let d_advisories = arm_d::run_arm_d(scoped, &file_path);
        for adv in d_advisories {
            host::log_warn(&adv.message);
        }
        return HookResult::Continue;
    }

    // BC file: Arm A1 + Class E
    if dispatch::is_bc_file(&file_path) {
        let bc_id = extract_stem_from_path(&file_path);
        let bc_version = frontmatter::extract_frontmatter_field(&content, "version")
            .unwrap_or_default();
        let (a1_v, a1_a) = arm_a1::run_arm_a1(&bc_id, &bc_version, &file_path);
        violations.extend(a1_v);
        advisories.extend(a1_a);
    }

    // Story file: Arm A2 + Arm B1 + Class E
    if dispatch::is_story_file(&file_path) {
        let story_id = extract_story_id_from_path(&file_path);
        let (a2_v, a2_a) = arm_a2::run_arm_a2(&story_id, &content);
        violations.extend(a2_v);
        advisories.extend(a2_a);

        let (b1_v, b1_a) = arm_b::run_arm_b1(&story_id, &content);
        violations.extend(b1_v);
        advisories.extend(b1_a);
    }

    // Class E (frontmatter parity) for BC/VP/story/epic
    if dispatch::is_frontmatter_parity_target(&file_path) {
        let (e1_v, e1_a) = arm_e::run_arm_e1(&content);
        violations.extend(e1_v);
        advisories.extend(e1_a);

        let e2_v = arm_e::run_arm_e2(&content);
        violations.extend(e2_v);
    }

    // Step 5/6: emit all advisories, then return
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
    use vsdd_hook_sdk::HookPayload;

    // Helper: build a minimal HookPayload with the given file_path.
    fn make_payload(file_path: &str) -> HookPayload {
        let json = format!(
            r#"{{"event_name":"PostToolUse","tool_name":"Edit","session_id":"test-session","dispatcher_trace_id":"test-trace","tool_input":{{"file_path":{:?}}}}}"#,
            file_path
        );
        serde_json::from_str(&json).expect("payload fixture must parse")
    }

    // -----------------------------------------------------------------------
    // AC-004: primary target CapabilityDenied blocks (lib.rs level)
    // -----------------------------------------------------------------------

    // NOTE: AC-004 (primary target CapabilityDenied → block) tests the effectful
    // dispatch path in on_post_tool_use which requires host mocking. This is verified
    // via integration (bats) tests in validate-cross-site-correspondence.bats.
    // The test here exercises a pure helper to document the fail-closed expectation.
    #[test]
    fn test_BC_5_39_010_arm_a1_primary_target_capability_denied_contract() {
        // Document: on_post_tool_use MUST block when primary target read returns CapabilityDenied.
        // This is a specification documentation test — it calls on_post_tool_use with a
        // payload for a BC file path. The function is todo!() → panics → RED gate.
        let payload = make_payload(".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md");
        let _result = on_post_tool_use(payload);
        // If todo!() is implemented correctly: when host::read_file returns CapabilityDenied
        // on the primary target, result must be HookResult::Block (exit_code 2).
        // For now: todo!() panics → test fails → RED gate holds.
    }

    // -----------------------------------------------------------------------
    // AC-018: combined violations from multiple arms → single block
    // -----------------------------------------------------------------------

    /// AC-018: combined A1 + E1 violations → one block message.
    #[test]
    fn test_BC_5_39_010_combined_a1_and_e1_single_block() {
        // When both Class A Arm1 and Class E1 fire simultaneously, the result must be
        // ONE combined block. Verified via combine_violations_into_block.
        let violations = vec![
            Violation {
                description:
                    "validate-cross-site-correspondence [Class A Arm1]: BC-INDEX stale".to_string(),
            },
            Violation {
                description:
                    "validate-cross-site-correspondence [Class E1]: version mismatch".to_string(),
            },
        ];
        let result = combine_violations_into_block(
            "validate-cross-site-correspondence",
            &violations,
        );
        // combine_violations_into_block is todo!() → panics → RED gate holds.
        assert_eq!(
            result.exit_code(),
            2,
            "combined violations must produce a single block (exit code 2)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-019: multi-byte UTF-8 no panic
    // -----------------------------------------------------------------------

    /// AC-019: multi-byte UTF-8 in content must not cause panic.
    #[test]
    fn test_BC_5_39_010_multibyte_utf8_no_panic() {
        // Fixture: BC file path; content contains multi-byte UTF-8 in BC title.
        // The hook must not panic on valid UTF-8 multi-byte content.
        let payload = make_payload(".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md");
        // on_post_tool_use is todo!() → panics → test fails → RED gate holds.
        // When implemented, this test verifies no panic on content containing:
        // "version: \"1.6\"\ntitle: \"Überprüfung der Korrespondenz\"\n"
        let _result = on_post_tool_use(payload);
    }
}
