//! update-wave-state-on-merge — SubagentStop WASM hook plugin.
//!
//! Fires on SubagentStop for pr-manager agents only. When a successful merge
//! signal is detected in the result text, this plugin:
//!
//! 1. Extracts the story ID from the result text (`S-N.NN` or `STORY-NNN`).
//! 2. Reads `.factory/wave-state.yaml` via `vsdd_hook_sdk::host::read_file`.
//! 3. Finds the wave containing the story ID in its `stories` list.
//! 4. Appends the story ID to `stories_merged` if not already present.
//! 5. Writes the updated YAML back via `vsdd_hook_sdk::host::write_file`
//!    (4-param form: path, contents, max_bytes=65536, timeout_ms=10000).
//! 6. If all stories in the wave are now merged: sets `gate_status="pending"`,
//!    `next_gate_required=wave_name`, writes a stderr reminder.
//! 7. Emits a `hook.action` event with all merge/gate fields.
//! 8. Always exits 0 (on_error=continue — advisory hook).
//!
//! # BC Anchors
//!
//! - BC-7.03.083: identity & registry binding (SubagentStop, priority=940,
//!   on_error=continue, timeout_ms=10000)
//! - BC-7.03.084: scopes to pr-manager + successful merge signal
//! - BC-7.03.085: appends story to wave_data.stories_merged via YAML
//! - BC-7.03.086: flips gate_status to pending when wave fully merged
//!
//! # GREEN-phase implementation notes
//!
//! The GREEN-phase implementer must:
//!
//! - Use `vsdd_hook_sdk::host::write_file(path, contents, max_bytes, timeout_ms)`
//!   with `max_bytes=65536` and `timeout_ms=10000` (4-param form per S-8.10 v1.1
//!   AC-1; capability block `[hooks.capabilities.write_file] path_allow =
//!   [".factory/wave-state.yaml"]` required in hooks-registry.toml at T-9).
//! - Use `serde_yaml::from_str` / `serde_yaml::to_string` for YAML
//!   parse/serialize. The `gate_status` field MUST be typed as
//!   `Option<String>` with `#[serde(default)]` to handle the 4-case truth table
//!   (absent, YAML-null/~, "not_started", other) defined in AC-005 of S-8.04.
//! - Preserve key ordering (`sort_keys=False` parity) by using
//!   `serde_yaml::Mapping` which wraps `IndexMap` for insertion-order maps.
//! - Port the merge signal regex verbatim (port-as-is per OQ-001):
//!   `STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge` (case-insensitive).
//!   ERE precedence quirk preserved intentionally; TD filed for v1.2 fix.
//! - Extract story_id: `S-[0-9]+\.[0-9]+` first, fall back to `STORY-[0-9]+`.
//! - Do NOT duplicate-append: if story_id already in `stories_merged`, exit 0
//!   silently with NO event emitted (EC-003 / AC-006 case (c)).
//! - `host::write_file` requires the `WriteFileCaps` capability block in
//!   hooks-registry.toml (added at T-9 GREEN time; not present in STUB phase).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Agent-scoping logic (BC-7.03.084)
// ---------------------------------------------------------------------------

/// Returns `true` when the agent name matches the pr-manager scope pattern.
///
/// Matches `*pr-manager*` or `*pr_manager*` (bash: `*pr-manager*|*pr_manager*`).
/// Called before any YAML I/O; non-matching agents exit immediately (exit 0).
///
/// # BC trace
/// BC-7.03.084 precondition 1: agent name matches `*pr-manager*`.
pub fn is_pr_manager_agent(agent_type: &str) -> bool {
    agent_type.contains("pr-manager") || agent_type.contains("pr_manager")
}

/// Returns `true` when the result text contains a merge completion signal.
///
/// Pattern (ported verbatim from bash, OQ-001 — port-as-is per D-2 Option C):
/// `STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge` (case-insensitive).
///
/// ERE alternation precedence note: the bash regex has a known precedence
/// ambiguity — it parses as three arms:
///   `(STEP_COMPLETE: step=8.*status=ok)|(merged)|(squash.*merge)`
/// rather than the intended grouped form. Port-as-is preserves this behavior
/// including the side-effect that the bare string "merge_complete" matches
/// the `merged` arm. TD filed for v1.2 fix with grouped alternation.
///
/// # BC trace
/// BC-7.03.084 precondition 1: result matches merge signal.
///
/// # STUB note
/// GREEN-phase: replace this stub with a real `regex::Regex` match using the
/// case-insensitive flag (`(?i)`). The `regex` crate is declared as a
/// dependency for GREEN use.
pub fn has_merge_signal(result: &str) -> bool {
    // STUB: always returns false so no YAML mutation happens during scaffold
    // verification. GREEN impl: use regex::Regex::new(r"(?i)STEP_COMPLETE:\
    // step=8.*status=ok|merged|squash.*merge").unwrap().is_match(result)
    let _ = result;
    false
}

// ---------------------------------------------------------------------------
// Story ID extraction (BC-7.03.085 precondition 1)
// ---------------------------------------------------------------------------

/// Extract a story ID from the result text.
///
/// Tries `S-[0-9]+\.[0-9]+` first (e.g. `S-8.04`); falls back to
/// `STORY-[0-9]+` (e.g. `STORY-042`). Returns `None` if neither matches.
/// Uses first match only (`head -1` parity).
///
/// # BC trace
/// BC-7.03.085 precondition 1: story ID extracted from result.
///
/// # STUB note
/// GREEN-phase: replace stub with `regex::Regex` captures.
pub fn extract_story_id(result: &str) -> Option<String> {
    // STUB: returns None so no YAML mutation happens during scaffold verification.
    // GREEN impl: try r"S-[0-9]+\.[0-9]+" first, then r"STORY-[0-9]+".
    let _ = result;
    None
}

// ---------------------------------------------------------------------------
// YAML mutation outcome (BC-7.03.085 + BC-7.03.086)
// ---------------------------------------------------------------------------

/// Outcome of a `process_wave_state` call, consumed by the entry point to
/// drive event emission and stderr output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveStateOutcome {
    /// The hook had nothing to do (file absent, story not found, or duplicate).
    NoOp,
    /// The story was appended to `stories_merged`.
    Appended {
        wave: String,
        total: usize,
        merged: usize,
        gate_transitioned: bool,
    },
}

/// Process wave-state.yaml: find the wave containing `story_id`, append it
/// to `stories_merged`, conditionally flip `gate_status` to "pending".
///
/// The `read_yaml` / `write_yaml` callbacks abstract the host file I/O so
/// unit tests can drive the logic without a WASM runtime.
///
/// `read_yaml`:  `() -> Option<String>` — returns the raw YAML string, or
///               `None` if the file is absent / unreadable.
/// `write_yaml`: `(String) -> ()` — writes the updated YAML string back.
///               May also write to stderr for the gate-transition reminder.
///
/// # AC-005 gate_status 4-case truth table
///
/// | Case | YAML                       | Serde `Option<String>` | Action         |
/// |------|----------------------------|------------------------|----------------|
/// | 1    | key absent                 | `None`                 | flip pending   |
/// | 2    | key present, YAML null/~   | `None`                 | flip pending   |
/// | 3    | key present, "not_started" | `Some("not_started")`  | flip pending   |
/// | 4    | key present, any other     | `Some("...")`          | do NOT flip    |
///
/// # BC trace
/// BC-7.03.085 postcondition 1: append story_id to stories_merged; write YAML.
/// BC-7.03.086 postcondition 1: flip gate_status when all stories merged.
///
/// # STUB note
/// GREEN-phase: implement serde_yaml parse/serialize here. Use
/// `serde_yaml::from_str::<serde_yaml::Value>(&yaml_str)` and traverse
/// `state["waves"]` as a sequence. The `gate_status` field uses
/// `Option<String>` + `#[serde(default)]` on the Rust struct. Serialize
/// with `serde_yaml::to_string` (produces `default_flow_style=false`,
/// `sort_keys=false` parity via IndexMap-backed Mapping).
pub fn process_wave_state<R, W>(
    story_id: &str,
    read_yaml: R,
    write_yaml: W,
) -> WaveStateOutcome
where
    R: FnOnce() -> Option<String>,
    W: FnOnce(String),
{
    // STUB: no-op implementation. All logic is deferred to GREEN phase.
    // GREEN-phase implementer: read YAML, find wave, append, write back.
    let _ = story_id;
    let _ = read_yaml;
    let _ = write_yaml;
    WaveStateOutcome::NoOp
}

// ---------------------------------------------------------------------------
// Top-level hook logic (all BCs)
// ---------------------------------------------------------------------------

/// Top-level hook logic. Accepts a `HookPayload` and injectable callbacks
/// so tests can drive every branch without host function calls.
///
/// `read_yaml`:  `() -> Option<String>` — reads `.factory/wave-state.yaml`.
/// `write_yaml`: `(String) -> ()` — writes updated YAML and any stderr.
/// `emit`:       `(WaveStateOutcome) -> ()` — emits the `hook.action` event.
///
/// Always returns `HookResult::Continue` (advisory, on_error=continue).
///
/// # BC trace
/// BC-7.03.083: identity/registry — SubagentStop, priority=940, always exit 0.
/// BC-7.03.084: agent scope + merge signal gate.
/// BC-7.03.085: YAML append via process_wave_state.
/// BC-7.03.086: gate_status flip via process_wave_state.
pub fn wave_state_hook_logic<R, W, E>(
    payload: HookPayload,
    read_yaml: R,
    write_yaml: W,
    emit: E,
) -> HookResult
where
    R: FnOnce() -> Option<String>,
    W: FnOnce(String),
    E: FnOnce(&WaveStateOutcome, &str),
{
    // BC-7.03.084: scope to pr-manager agents only.
    let agent_type = payload
        .tool_input
        .get("agent_type")
        .and_then(|v| v.as_str())
        .or_else(|| {
            payload
                .tool_input
                .get("subagent_name")
                .and_then(|v| v.as_str())
        })
        .unwrap_or("unknown");

    if !is_pr_manager_agent(agent_type) {
        return HookResult::Continue;
    }

    // BC-7.03.084: only act on merge completion signals.
    let result = payload
        .tool_input
        .get("last_assistant_message")
        .and_then(|v| v.as_str())
        .or_else(|| payload.tool_input.get("result").and_then(|v| v.as_str()))
        .unwrap_or("");

    if !has_merge_signal(result) {
        return HookResult::Continue;
    }

    // BC-7.03.085: extract story ID.
    let story_id = match extract_story_id(result) {
        Some(id) => id,
        None => return HookResult::Continue,
    };

    // BC-7.03.085 + BC-7.03.086: YAML mutation.
    let outcome = process_wave_state(&story_id, read_yaml, write_yaml);

    // Emit event only when something was actually appended (EC-003: no emit
    // on duplicate detection — process_wave_state returns NoOp for duplicates).
    if let WaveStateOutcome::Appended { .. } = &outcome {
        emit(&outcome, &story_id);
    }

    HookResult::Continue
}
