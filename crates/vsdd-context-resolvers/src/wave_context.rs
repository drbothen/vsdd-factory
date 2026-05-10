//! Wave-context YAML parsing types.
//!
//! # Design note (Pass-1 fix F-003)
//! `WaveState` now mirrors the canonical schema from `update-wave-state-on-merge`
//! (the SHIPPED producer of `.factory/wave-state.yaml`). The canonical schema uses
//! `waves: Vec<WaveEntry>` rather than flat top-level fields. The `cycle_id` comes
//! from `.factory/STATE.md` frontmatter, read separately by `resolve_impl`.
//!
//! # Schema alignment
//! `WaveEntry` fields match `update-wave-state-on-merge/src/lib.rs` exactly:
//!   - `wave: String` — wave identifier (e.g., "F4")
//!   - `stories: Vec<String>` — stories planned for this wave
//!   - `stories_merged: Vec<String>` — stories already merged
//!   - `gate_status: Option<String>` — None / "not_started" / "pending" / "completed"
//!   - `current_wave: Option<serde_yaml::Value>` — optional extra field (preserved)
//!   - `next_gate_required: Option<serde_yaml::Value>` — optional extra field (preserved)
//!
//! # Active wave determination
//! An active wave is the LAST entry in `waves` whose `gate_status !=
//! Some("completed")`. This matches the producer's semantics: once a wave is
//! gate-approved it transitions to "completed"; the next wave becomes active.
//!
//! # No panic guarantee
//! AC-010 / BC-4.12.004 INV1: no fallible unwrap or panic-on-error calls anywhere
//! in this module's production source. All error paths return `None`.

use serde::Deserialize;

/// Represents a single wave entry in `.factory/wave-state.yaml`.
///
/// Matches the canonical schema produced by `update-wave-state-on-merge`.
/// All list fields use `#[serde(default)]` to tolerate partial YAML entries.
/// `gate_status` uses `Option<String>` with `#[serde(default)]` to handle the
/// AC-005 four-case truth table:
///   Case 1: key absent → None (serde default) → wave NOT completed
///   Case 2: key present, YAML null/~ → None → wave NOT completed
///   Case 3: key present, "not_started" / "pending" → Some("...") → NOT completed
///   Case 4: key present, "completed" → Some("completed") → wave is done
#[derive(Debug, Clone, Deserialize)]
pub struct WaveEntry {
    /// Wave identifier (e.g., "F4", "wave-3").
    pub wave: String,
    /// Story IDs planned for this wave.
    #[serde(default)]
    pub stories: Vec<String>,
    /// Story IDs already merged in this wave.
    #[serde(default)]
    pub stories_merged: Vec<String>,
    /// Gate status — None means not yet set (wave is not completed).
    #[serde(default)]
    pub gate_status: Option<String>,
    /// Optional extra field from producer (round-tripped to preserve unknown data).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_wave: Option<serde_yaml::Value>,
    /// Optional extra field from producer (round-tripped to preserve unknown data).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_gate_required: Option<serde_yaml::Value>,
}

/// Top-level `.factory/wave-state.yaml` structure.
///
/// Mirrors the canonical schema from `update-wave-state-on-merge/src/lib.rs`.
/// `Default` yields an empty waves list — used as the all-None post-parse-failure
/// path in `resolve_impl`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WaveState {
    /// All waves in the pipeline, in order from earliest to latest.
    #[serde(default)]
    pub waves: Vec<WaveEntry>,
}

/// Pure output type for the `wave_context` JSON payload injected into `plugin_config`.
///
/// Constructed by `resolve_wave_context_pure`. Serialized to JSON for
/// `ResolverOutput.value`. (BC-4.12.002 PC3, AC-001)
#[derive(Debug, Clone)]
pub struct WaveContext {
    /// Cycle identifier from STATE.md frontmatter `current_cycle:` field.
    pub cycle_id: String,
    /// Wave identifier from the active `WaveEntry.wave` field.
    pub wave_id: String,
    /// Story list from the active `WaveEntry.stories` field.
    pub stories: Vec<String>,
}

/// Parse a YAML string into a `WaveState`.
///
/// Returns `Err(serde_yaml::Error)` on malformed YAML; callers map errors to
/// `ResolverOutput { value: None }` (AC-002, EC-003). Does NOT panic.
/// BC-4.12.004 INV1: no fallible unwrap or panic-on-error calls.
pub fn parse_wave_state(yaml: &str) -> Result<WaveState, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Determine the active wave from a `WaveState`.
///
/// The active wave is the LAST entry in `waves` whose `gate_status` is NOT
/// `Some("completed")`. Returns `None` if `waves` is empty or all waves
/// are completed.
pub fn find_active_wave(wave_state: &WaveState) -> Option<&WaveEntry> {
    wave_state
        .waves
        .iter()
        .rev()
        .find(|w| w.gate_status.as_deref() != Some("completed"))
}

/// Parse the `current_cycle:` value from `.factory/STATE.md` YAML frontmatter.
///
/// STATE.md uses YAML front-matter delimited by `---` markers. This function
/// extracts the frontmatter block and parses `current_cycle` from it.
/// Returns `None` if the file has no frontmatter, the frontmatter is malformed,
/// or the `current_cycle` key is absent.
///
/// BC-4.12.004 INV1: does NOT panic.
pub fn parse_cycle_id_from_state_md(state_md: &str) -> Option<String> {
    // STATE.md starts with a YAML frontmatter block delimited by `---` markers.
    // Format:
    //   ---
    //   document_type: pipeline-state
    //   ...
    //   current_cycle: v1.0-feature-engine-discipline-pass-1
    //   ...
    //   ---
    let trimmed = state_md.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    // Find the closing `---` marker after the opening one.
    let after_open = trimmed.get(3..)?;
    // Skip past any immediate newline after the opening `---`
    let content_start = after_open.trim_start_matches(['\n', '\r']);
    let end_pos = content_start.find("\n---")?;
    let frontmatter = &content_start[..end_pos];

    // Parse the frontmatter as YAML.
    #[derive(Deserialize)]
    struct Frontmatter {
        #[serde(default)]
        current_cycle: Option<String>,
    }

    let fm: Frontmatter = serde_yaml::from_str(frontmatter).ok()?;
    fm.current_cycle
}
