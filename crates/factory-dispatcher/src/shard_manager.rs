//! Native (non-WASM) shard-cap formula and deterministic size/item-count
//! trigger for Layer-2 artifact sharding.
//!
//! Implements BC-1.18.005 (Byte-Size-Denominated Shard-Cap Formula and
//! Native Deterministic Size-Trigger; ADR-051 §Decision 1/2). This module
//! owns BOTH trigger shapes the dispatcher's PreToolUse handling path
//! dispatches on for `Edit`/`Write`/`MultiEdit` tool calls against a
//! `[[shard]]`-registered artifact:
//!
//! - `"flat"` shape — Postconditions 1-7: `stat()`-only byte-size trigger.
//! - `"frontmatter-changelog-array"` shape — Postcondition 8: item-count
//!   trigger over a frontmatter `changelog:` array.
//!
//! Architecturally analogous to the existing native `block_if_marker_check`
//! precedent ([`crate::indeterminate_marker::block_if_marker_check`]),
//! consulted from `executor.rs` BEFORE the registry-driven WASM plugin loop
//! (Invariant 1). The config-match check (Postcondition 1 / Invariant 3)
//! MUST occur before any `stat()` call, so the ~99% of `Edit`/`Write`/
//! `MultiEdit` calls that do not target a sharded artifact pay zero added
//! latency.
//!
//! # Scope note (S-25.02 F4 BC-cluster 1 "cap+trigger")
//!
//! This module stubs BC-1.18.005 ONLY (tasks T-1/T-2/T-3; AC-001..AC-005).
//! BC-1.18.006 (the observable roll/block outcome once the `"flat"` trigger
//! fires), BC-1.18.009 (the observable rotate/block-and-retry outcome once
//! the item-count trigger fires), and BC-1.18.012 (the one-time changelog
//! backfill migration) are LATER clusters and are explicitly OUT OF SCOPE
//! here — this module owns the trigger-boundary decision and the hand-off
//! point only, per Postcondition 3's and Postcondition 8's "Ownership"
//! bullets.
//!
//! # BC-5.38.001 Red Gate discipline
//!
//! All non-trivial function bodies use `todo!()`. The implementer fills in
//! real logic in the TDD step that follows this stub. See the stub commit
//! report for the GREEN-BY-DESIGN and WIRING-EXEMPT exceptions (both
//! narrowly scoped: one pure field-copy constructor, one `From` delegation).

// BC-5.38.001 Red Gate stub: every non-trivial function body below is
// `todo!()`, so its parameters are structurally unused until the
// implementer fills them in. `unused_variables` is allowed at MODULE scope
// (rather than a `#[allow(...)]` repeated on each of the ~20 stubbed
// signatures) so every parameter name is retained, undocumented and
// un-underscored, giving the test-writer and implementer stages the full,
// real public signature to write tests and implementations against — per
// the stub-architect contract's "public function signatures ... must be
// complete and correct" requirement. This is the ONLY clippy/rustc
// allowance this module adds; `todo!()` itself needs no allow (it is not a
// clippy lint under this workspace's lint table).
#![allow(unused_variables)]

use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use vsdd_hook_sdk::HookResult;

// ---------------------------------------------------------------------------
// Config surface — `[[shard]]` table (Preconditions 2/3)
// ---------------------------------------------------------------------------

/// Trigger shape a `[[shard]]` config entry declares (Postcondition 8's
/// shape-dispatch field). Read once per entry at config-load time, never
/// inferred from the target path's content or extension (Invariant 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShardShape {
    /// `stat()`-only byte-size trigger (Postconditions 1-7).
    Flat,
    /// Item-count trigger over a frontmatter `changelog:` array (Postcondition 8).
    FrontmatterChangelogArray,
}

/// The four calibrated cap-formula inputs (Postcondition 4/6). Configuration,
/// never embedded Rust constants (Invariant 4) — this is what makes the F4
/// harness's recalibration a config change, not a code change.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct CapFormulaInputs {
    /// `PRACTICAL_FUEL_CEILING` — today's `DEFAULT_FUEL_CAP`-derived reliably-
    /// completes fuel ceiling (Postcondition 6; PROVISIONAL until F4 lock).
    pub practical_fuel_ceiling: u64,
    /// `WORST_CASE_FUEL_PER_BYTE` — per-plugin measured fuel/byte coefficient,
    /// or the local marginal rate at the largest tested size if superlinear
    /// (Postcondition 7 / EC-006).
    pub worst_case_fuel_per_byte: f64,
    /// `MAX_SINGLE_RECORD_BYTES` — largest single physical line/record margin.
    pub max_single_record_bytes: u64,
    /// `SAFETY_MARGIN` — buffer for shard-index-entry + shard-header overhead.
    pub safety_margin: u64,
}

/// One `[[shard]]` config table entry — one registered sharded artifact.
///
/// `#[serde(deny_unknown_fields)]` is deliberately OMITTED (unlike
/// [`crate::registry::Registry`]'s hooks table) because the `[[shard]]`
/// config location and full field set is explicitly "TBD at F4" per
/// BC-1.18.005's Architecture Anchors — a stricter schema is a follow-up
/// concern once the F4 calibration harness locks the final shape.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShardEntry {
    /// Artifact stem this entry matches against a tool call's target path
    /// (Precondition 3; Postcondition 1's config-match predicate).
    pub artifact_stem: String,

    /// `PRACTICAL_FUEL_CEILING` cap-formula input (Postcondition 4/6).
    pub practical_fuel_ceiling: u64,
    /// `WORST_CASE_FUEL_PER_BYTE` cap-formula input (Postcondition 4/6).
    pub worst_case_fuel_per_byte: f64,
    /// `MAX_SINGLE_RECORD_BYTES` cap-formula input (Postcondition 4/6).
    pub max_single_record_bytes: u64,
    /// `SAFETY_MARGIN` cap-formula input (Postcondition 4/6).
    pub safety_margin: u64,

    /// This entry's own per-plugin `shard_cap_bytes` ceiling. Subject to the
    /// Cross-Validator Minimum Rule (Postcondition 5) across every Cohort B
    /// validator that reads this artifact — this field is the SINGLE
    /// validator's own cap; [`effective_shard_cap_bytes`] combines it with
    /// sibling per-validator caps at call time. Byte-denominated only
    /// (Invariant 2) — never a line-count proxy.
    pub shard_cap_bytes: u64,

    /// Trigger-shape dispatch field (Postcondition 8). `#[serde(default)]`
    /// so an entry that omits it deserializes to `None` at the TOML layer
    /// (rather than a hard parse failure with no artifact-stem context) —
    /// EC-009's fail-loud `HookResult::Error` is enforced by
    /// [`ShardRegistry::load`]'s post-deserialize validation pass, which
    /// can name the offending `artifact_stem` in the error.
    #[serde(default)]
    pub shape: Option<ShardShape>,

    /// Item-count trigger threshold `N` (`"frontmatter-changelog-array"`
    /// shape only; Postcondition 8). `None` for `"flat"`-shaped entries.
    #[serde(default)]
    pub n: Option<u64>,

    /// Rotation-target config (`"frontmatter-changelog-array"` shape only;
    /// Postcondition 8's rotation-target-config bullet). `None` when
    /// omitted — resolves to `floor(N/2)` at config-load time (EC-010).
    ///
    /// `i64` (not `u64`) so a negative config value round-trips for
    /// EC-011's fail-loud validation instead of failing opaquely at the
    /// TOML/serde layer with no artifact-stem context.
    #[serde(default)]
    pub low_water_mark: Option<i64>,
}

impl ShardEntry {
    /// Extract this entry's four cap-formula inputs (Postcondition 4/6) as a
    /// standalone [`CapFormulaInputs`] value for use with
    /// [`compute_shard_cap_bytes`].
    ///
    /// # GREEN-BY-DESIGN (BC-5.38.002)
    ///
    /// Pure 1:1 field copy into a type constructor — zero branching, no I/O,
    /// no calls to non-trivial helpers, single-expression body. Behavior is
    /// fully determined by the two types' shapes; there is no domain
    /// decision here for a test to exercise non-trivially.
    pub fn cap_formula_inputs(&self) -> CapFormulaInputs {
        CapFormulaInputs {
            practical_fuel_ceiling: self.practical_fuel_ceiling,
            worst_case_fuel_per_byte: self.worst_case_fuel_per_byte,
            max_single_record_bytes: self.max_single_record_bytes,
            safety_margin: self.safety_margin,
        }
    }
}

/// The whole parsed `[[shard]]` config file (Precondition 2).
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ShardRegistry {
    #[serde(default, rename = "shard")]
    pub shards: Vec<ShardEntry>,
}

/// Config-load-time errors for the `[[shard]]` registry.
///
/// `MissingShape` (EC-009) and `InvalidLowWaterMark` (EC-011) are the two
/// fail-loud conditions this BC's config surface owns — both are NEVER
/// silently defaulted or clamped around.
#[derive(Debug, Error)]
pub enum ShardConfigError {
    #[error("shard config read failed: {0}")]
    Io(#[from] io::Error),

    #[error("shard config parse failed: {0}")]
    Toml(#[from] toml::de::Error),

    /// EC-009: a `[[shard]]` entry omits `shape` entirely. Fail-loud — this
    /// BC's implementation MUST NOT default silently to either shape.
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" omits the required `shape` \
         field (BC-1.18.005 EC-009). Fail-loud: shape is never defaulted; the dispatch is \
         treated as a config error, never a silent Continue that would leave an oversized \
         artifact unguarded."
    )]
    MissingShape {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
    },

    /// EC-011: `low_water_mark >= N` (the `== N` boundary included) or negative.
    /// `low_water_mark == N - 1` is explicitly NOT in this error's scope — see
    /// EC-012 / [`validate_low_water_mark`]'s amortization-advisory path.
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" declares low_water_mark = \
         {low_water_mark}, which violates 0 <= low_water_mark < N (N = {n}) \
         (BC-1.18.005 EC-011). Fail-loud: never silently clamped or defaulted around."
    )]
    InvalidLowWaterMark {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
        /// The entry's configured `N` (item-count trigger threshold).
        n: u64,
        /// The entry's configured (invalid) `low_water_mark`.
        low_water_mark: i64,
    },
}

/// Fail-loud config-load errors surface to the dispatcher's PreToolUse
/// handling path as `HookResult::Error` (EC-009 / EC-011's posture).
///
/// # WIRING-EXEMPT (BC-5.38.003)
///
/// `From<T>` blanket delegation to a single `Display`-forwarding call — the
/// canonical WIRING-EXEMPT example (`Self(value.into())`-shaped). No domain
/// decision: the error's own `Display` impl (via `thiserror`) already
/// carries the full, artifact-stem-scoped diagnostic text.
impl From<ShardConfigError> for HookResult {
    fn from(err: ShardConfigError) -> Self {
        HookResult::Error {
            message: err.to_string(),
        }
    }
}

impl ShardRegistry {
    /// Load + validate a `[[shard]]` config file from disk.
    ///
    /// Validation (Preconditions 2 + EC-009/EC-010/EC-011/EC-012):
    /// - every entry MUST declare `shape` (fail-loud [`ShardConfigError::MissingShape`]
    ///   on omission — EC-009).
    /// - `"frontmatter-changelog-array"`-shaped entries with an EXPLICIT
    ///   `low_water_mark` MUST satisfy `0 <= low_water_mark < N` (fail-loud
    ///   [`ShardConfigError::InvalidLowWaterMark`] — EC-011; `N-1` is a VALID
    ///   boundary value, never routed to this error — see EC-012).
    /// - `"frontmatter-changelog-array"`-shaped entries that OMIT
    ///   `low_water_mark` resolve to `floor(N/2)` (EC-010) — see
    ///   [`resolved_low_water_mark`]. Loading NEVER fails for an omitted value.
    /// - a legal-but-poorly-amortizing `low_water_mark` in `(floor(N/2), N)`
    ///   (up to and including `N-1`) loads normally (`Ok`) but emits a
    ///   non-fatal `tracing::warn!` amortization advisory (EC-012) — see
    ///   [`validate_low_water_mark`].
    pub fn load(path: &Path) -> Result<Self, ShardConfigError> {
        let text = std::fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&text)?;

        for entry in &parsed.shards {
            // EC-009: `shape` is fail-loud-required, never silently defaulted.
            let shape = entry.shape.ok_or_else(|| ShardConfigError::MissingShape {
                artifact_stem: entry.artifact_stem.clone(),
            })?;

            // Postcondition 8's rotation-target-config bullet / EC-010/EC-011/
            // EC-012 apply to the item-count shape's OPTIONAL explicit
            // low_water_mark only — an omitted value resolves to floor(N/2)
            // (EC-010) without ever running the fail-loud/advisory check
            // below (that default is, by construction, never poorly
            // amortizing and never invalid).
            if shape == ShardShape::FrontmatterChangelogArray {
                if let (Some(n), Some(low_water_mark)) = (entry.n, entry.low_water_mark) {
                    let fires_advisory =
                        validate_low_water_mark(&entry.artifact_stem, n, low_water_mark)?;
                    if fires_advisory {
                        let default_low_water_mark = n / 2;
                        tracing::warn!(
                            artifact_stem = %entry.artifact_stem,
                            n,
                            low_water_mark,
                            amortization_factor = n.saturating_sub(low_water_mark as u64),
                            default_low_water_mark,
                            default_amortization_factor = n.saturating_sub(default_low_water_mark),
                            "BC-1.18.005 EC-012: configured low_water_mark amortizes rotation \
                             worse than the recommended default floor(N/2); config load still \
                             succeeds (non-fatal advisory only)"
                        );
                    }
                }
            }
        }

        Ok(parsed)
    }
}

// ---------------------------------------------------------------------------
// Postcondition 1 / Invariant 3 — config-match-before-stat() zero-cost bypass
// ---------------------------------------------------------------------------

/// Find the `[[shard]]` config entry (if any) matching `target_path`.
///
/// MUST be called — and return — before any filesystem `stat()` call
/// (Invariant 3): the ~99% of `Edit`/`Write`/`MultiEdit` calls that do not
/// target a sharded artifact pay zero cost (Postcondition 1 / EC-001).
pub fn find_matching_entry<'a>(
    registry: &'a ShardRegistry,
    target_path: &Path,
) -> Option<&'a ShardEntry> {
    // `Path::file_stem()` is a pure string operation over the path's own
    // components — it never touches the filesystem, so this comparison is
    // free to run before any stat() call (Invariant 3).
    let stem = target_path.file_stem()?.to_str()?;
    registry
        .shards
        .iter()
        .find(|entry| entry.artifact_stem == stem)
}

// ---------------------------------------------------------------------------
// Postcondition 4/6/7 — byte-size-denominated cap formula
// ---------------------------------------------------------------------------

/// Compute the byte-size cap ceiling from the four calibrated inputs
/// (Postcondition 4): `shard_cap_bytes <= (PRACTICAL_FUEL_CEILING /
/// WORST_CASE_FUEL_PER_BYTE) - MAX_SINGLE_RECORD_BYTES - SAFETY_MARGIN`.
///
/// Byte-denominated only (Invariant 2) — never a line-count proxy. VP-116's
/// kani-proof exercises this formula's arithmetic over symbolic inputs for
/// overflow/underflow safety (Postcondition 7 / EC-006/EC-007's superlinear-
/// rate and ceiling-change re-derivation cases feed `worst_case_fuel_per_byte`
/// and `practical_fuel_ceiling` respectively — this function itself is
/// rate-agnostic; the caller supplies the already-corrected inputs).
pub fn compute_shard_cap_bytes(inputs: &CapFormulaInputs) -> u64 {
    let fuel_budget_bytes =
        (inputs.practical_fuel_ceiling as f64 / inputs.worst_case_fuel_per_byte).floor();
    // Defensive saturating arithmetic: the formula's provisional inputs
    // (Postcondition 6) always yield a comfortably positive result, but a
    // pathological config (e.g. an oversized MAX_SINGLE_RECORD_BYTES +
    // SAFETY_MARGIN pair) must never underflow/panic — floor at 0.
    (fuel_budget_bytes as u64)
        .saturating_sub(inputs.max_single_record_bytes)
        .saturating_sub(inputs.safety_margin)
}

// ---------------------------------------------------------------------------
// Postcondition 5 — Cross-Validator Minimum Rule
// ---------------------------------------------------------------------------

/// Effective `shard_cap_bytes` for a multi-reader artifact: the MINIMUM
/// across every Cohort B validator's own per-plugin cap (Postcondition 5).
/// A single global cap across all mechanism-A artifacts MUST NOT be
/// substituted — it would be needlessly conservative for artifacts fewer
/// validators read.
///
/// `per_validator_caps` is the set of `cap_for(validator)` values for every
/// Cohort B validator that reads the target artifact (per ADR-047 §8a's
/// Cohort B table) — NOT every validator in the system.
pub fn effective_shard_cap_bytes(per_validator_caps: &[u64]) -> u64 {
    // A vacuous minimum (no Cohort B validator reads this artifact) imposes
    // no constraint at all — `u64::MAX` rather than `0`, so an
    // unreferenced-but-registered artifact never spuriously self-triggers.
    per_validator_caps.iter().copied().min().unwrap_or(u64::MAX)
}

// ---------------------------------------------------------------------------
// Postcondition 2 — stat()-only current-shard byte-size read ("flat" shape)
// ---------------------------------------------------------------------------

/// Read the current shard's byte size via `stat()`/`metadata()` ONLY — never
/// reading file content into memory for the size determination
/// (Postcondition 2). `"flat"` shape only; see [`read_changelog_item_count`]
/// for the `"frontmatter-changelog-array"` shape's different, more-than-
/// `stat()` read cost.
///
/// EC-004: a shard that does not yet exist on disk (first write ever) is
/// treated as size 0, not an `io::Error`.
pub fn current_shard_bytes_flat(shard_path: &Path) -> io::Result<u64> {
    match std::fs::metadata(shard_path) {
        Ok(meta) => Ok(meta.len()),
        // EC-004: first write ever -> treated as size 0, not an io::Error.
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(0),
        Err(e) => Err(e),
    }
}

// ---------------------------------------------------------------------------
// Postcondition 3 — per-tool-semantics projected-size formula ("flat" shape)
// ---------------------------------------------------------------------------

/// The three tool kinds this BC's trigger discriminates on (Postcondition 3
/// CORRECTED tool-discriminated formula).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    /// `content` is the file's complete post-apply state, not a delta.
    Write,
    /// Mutates existing content in place; delta-against-current-size model.
    Edit,
    /// Mutates existing content in place via multiple edit blocks; net delta
    /// is the SUM of every block's own delta (EC-005).
    MultiEdit,
}

impl ToolKind {
    /// Map a Claude Code `tool_name` string onto the discriminated
    /// [`ToolKind`] this BC's formula dispatches on. `None` for any other
    /// tool name — the caller's own zero-cost bypass is expected to have
    /// already excluded non-mutating tools before reaching this point.
    pub fn from_tool_name(tool_name: &str) -> Option<Self> {
        match tool_name {
            "Write" => Some(Self::Write),
            "Edit" => Some(Self::Edit),
            "MultiEdit" => Some(Self::MultiEdit),
            _ => None,
        }
    }
}

/// One `MultiEdit` edit block's raw length inputs, prior to net-delta
/// reduction (EC-005: some blocks net-negative, some net-positive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditDelta {
    /// `len(old_string)` for this edit block.
    pub old_len_bytes: u64,
    /// `len(new_string)` for this edit block.
    pub new_len_bytes: u64,
}

/// `Write`'s post-apply projected size: `len(content)` ALONE
/// (Postcondition 3 CORRECTED formula).
///
/// `current_shard_bytes` is deliberately NOT a parameter — the WITHDRAWN
/// uniform `current_shard_bytes + payload_bytes` formula double-counted a
/// `Write`'s own already-complete content on top of the shard's
/// pre-existing bytes, over-triggering rolls on ordinary same-size-or-
/// shrinking full-file `Write` calls.
pub fn projected_size_write(content_len_bytes: u64) -> u64 {
    content_len_bytes
}

/// `Edit`/`MultiEdit`'s post-apply projected size:
/// `current_shard_bytes + net_delta_bytes` (Postcondition 3 — UNCHANGED leg;
/// `Edit`/`MultiEdit` mutate existing content in place, so the delta-
/// against-current-size model was always correct for these two tools).
///
/// `net_delta_bytes` is signed (EC-005: a net-shrinking `MultiEdit` may be
/// negative overall). The eventual implementation MUST NOT let a large
/// negative delta underflow an unsigned `current_shard_bytes` — a
/// saturating (floor-at-zero) computation is the production-grade choice,
/// left to the implementer per Red Gate discipline (this stub does not
/// pre-decide the overflow-safety strategy VP-116's kani-proof verifies).
pub fn projected_size_edit(current_shard_bytes: u64, net_delta_bytes: i64) -> u64 {
    if net_delta_bytes >= 0 {
        // Safe: net_delta_bytes >= 0, so the cast is lossless for any value
        // that fits an i64 in the first place.
        current_shard_bytes.saturating_add(net_delta_bytes as u64)
    } else {
        current_shard_bytes.saturating_sub(net_delta_bytes.unsigned_abs())
    }
}

/// Net length delta for a single `Edit` call: `len(new_string) - len(old_string)`.
pub fn net_delta_bytes_for_edit(old_len_bytes: u64, new_len_bytes: u64) -> i64 {
    new_len_bytes as i64 - old_len_bytes as i64
}

/// Net length delta for a `MultiEdit` call: the SUM of every edit block's own
/// `len(new_string) - len(old_string)` (EC-005 — may be negative overall even
/// when individual blocks are large).
pub fn net_delta_bytes_for_multi_edit(edits: &[EditDelta]) -> i64 {
    edits
        .iter()
        .map(|e| net_delta_bytes_for_edit(e.old_len_bytes, e.new_len_bytes))
        .sum()
}

/// `true` iff `projected_size > shard_cap_bytes` — the `"flat"` shape's
/// roll-trigger boundary (Postcondition 3's `<=`/`>` comparison; EC-002
/// inclusive boundary at exact equality, EC-003 exactly-one-byte-over).
///
/// This function owns the TRIGGER decision only. BC-1.18.006 owns the
/// observable roll/block outcome once it fires (Postcondition 3's
/// "Ownership" bullet) — out of scope for this cluster.
pub fn size_trigger_fires(projected_size: u64, shard_cap_bytes: u64) -> bool {
    projected_size > shard_cap_bytes
}

// ---------------------------------------------------------------------------
// Postcondition 8 — item-count trigger ("frontmatter-changelog-array" shape)
// ---------------------------------------------------------------------------

/// Read the target file's frontmatter far enough to count the existing
/// `changelog:` sequence's items (Postcondition 8's read-cost bullet).
///
/// MORE than a `stat()` call — reads and lightly parses frontmatter content
/// — but remains native, fuel-budget-free dispatcher code, never a WASM
/// plugin invocation (the "why native, not WASM" rationale, Postcondition 2,
/// applies identically to this shape).
///
/// This function's contract is the SAME read regardless of cold-state
/// (pre-BC-1.18.012 migration, ~1,997-item, not-N-relative-bounded) vs.
/// steady-state (post-migration, genuinely `<= N`-item-bounded) — the
/// cold/steady-state split BC-1.18.005 documents is a PERFORMANCE
/// characterization, not a different code path this function branches on.
pub fn read_changelog_item_count(target_path: &Path) -> io::Result<u64> {
    todo!("BC-1.18.005 Postcondition 8: parse frontmatter, count changelog: array items")
}

/// `true` iff `current_item_count + 1 > N` — the item-count trigger boundary
/// (Postcondition 8's trigger condition; EC-008 off-by-one: `N-1` items ->
/// `false`, exactly `N` or `N+1` items -> `true`).
///
/// NEVER a byte-size comparison for this shape — `shard_cap_bytes` still
/// bounds the artifact's total byte footprint as a whole-artifact concern,
/// but the rotation decision within this shape is item-count-based only.
pub fn item_count_trigger_fires(current_item_count: u64, n: u64) -> bool {
    todo!("BC-1.18.005 Postcondition 8 / EC-008: current_item_count + 1 > N")
}

/// Resolve the effective `low_water_mark` for a `"frontmatter-changelog-
/// array"`-shaped entry: the entry's explicit value if present, otherwise
/// `floor(N/2)` (EC-010's config-load-time default).
///
/// Callers MUST have already validated the entry via [`ShardRegistry::load`]
/// (or [`validate_low_water_mark`] directly) before calling this — this
/// function does NOT re-validate `0 <= low_water_mark < N`; it only resolves
/// the omitted-vs-explicit default.
pub fn resolved_low_water_mark(n: u64, low_water_mark: Option<i64>) -> u64 {
    todo!("BC-1.18.005 EC-010: default floor(N/2) when low_water_mark is omitted")
}

/// Validate a `"frontmatter-changelog-array"`-shaped entry's `low_water_mark`
/// against `0 <= low_water_mark < N` (EC-011), and determine whether the
/// EC-012 non-fatal amortization advisory applies.
///
/// - `low_water_mark >= N` (the `== N` boundary included) or negative ->
///   `Err(ShardConfigError::InvalidLowWaterMark)` (EC-011). NEVER silently
///   clamped or defaulted around.
/// - Any other legal value (including the boundary value `N - 1`) ->
///   `Ok(fires_advisory)`, where `fires_advisory` is `true` iff
///   `low_water_mark > floor(N/2)` (EC-012) — the caller is responsible for
///   emitting the actual `tracing::warn!` amortization advisory (citing the
///   configured `(N, low_water_mark)` pair and the amortization factor
///   `N - low_water_mark`, compared against the default's `N -
///   floor(N/2)` amortization) when this returns `Ok(true)`. Config load
///   ALWAYS succeeds (`Ok`, never `Err`) for any value satisfying
///   `0 <= low_water_mark < N` — this function never conflates the
///   fail-loud numeric-constraint check with the advisory, non-fatal one.
pub fn validate_low_water_mark(
    artifact_stem: &str,
    n: u64,
    low_water_mark: i64,
) -> Result<bool, ShardConfigError> {
    todo!(
        "BC-1.18.005 EC-011/EC-012: fail-loud on low_water_mark >= N or negative; \
         otherwise Ok(low_water_mark > floor(N/2)) as the advisory-fires flag"
    )
}

// ---------------------------------------------------------------------------
// Invariant 1 / Precondition 1 — the single native gate dispatch entry point
// ---------------------------------------------------------------------------

/// Native (non-WASM) shard-cap gate check for a single `Edit`/`Write`/
/// `MultiEdit` PreToolUse tool call.
///
/// This is the single entry point `executor.rs` calls BEFORE the
/// registry-driven WASM plugin loop (Invariant 1; architecturally analogous
/// to [`crate::indeterminate_marker::block_if_marker_check`]).
/// Postcondition 1's zero-cost bypass applies identically regardless of
/// which shape a matched entry declares (Postcondition 8's shape-dispatch
/// bullet: "both shapes share the SAME config-match/no-match entry point").
///
/// Returns:
/// - `HookResult::Continue` — no config match (Postcondition 1 / EC-001), or
///   a match whose projected size / item count does not exceed its trigger
///   threshold (EC-002, happy-path item-count rows).
/// - `HookResult::Block { .. }` — NEVER returned by this function itself.
///   BC-1.18.006 (byte-size roll) and BC-1.18.009 (item-count rotate-then-
///   retry) own the observable Block outcome once THIS function's trigger
///   fires (Postcondition 3 / Postcondition 8 "Ownership" bullets) — this
///   cluster stubs the trigger boundary and hand-off point only, never the
///   roll/rotation implementation itself.
/// - `HookResult::Error { .. }` — fail-loud on a malformed `[[shard]]`
///   config entry (EC-009 missing `shape`; EC-011 invalid `low_water_mark`),
///   via [`ShardConfigError`]'s `From<ShardConfigError> for HookResult` impl.
///
/// `tool_input` carries the tool-specific payload (`content` for `Write`;
/// `old_string`/`new_string` for `Edit`; an `edits` array for `MultiEdit`) —
/// kept as an opaque `serde_json::Value` here, matching
/// [`crate::payload::HookPayload::tool_input`]'s own representation, so this
/// function's signature does not have to special-case three different typed
/// tool-input shapes at the call boundary.
pub fn shard_cap_gate_check(
    shard_registry: &ShardRegistry,
    tool_name: &str,
    target_path: &Path,
    tool_input: &serde_json::Value,
) -> HookResult {
    todo!(
        "BC-1.18.005 Invariant 1 / T-2: native PreToolUse shard-cap gate entry point. \
         Dispatch order: find_matching_entry (Postcondition 1 zero-cost bypass) -> \
         shape dispatch (Postcondition 8) -> stat()-only read (flat) or frontmatter \
         item-count read (frontmatter-changelog-array) -> per-tool projected-size / \
         item-count trigger evaluation -> Continue, or hand off to BC-1.18.006 / \
         BC-1.18.009 (out of scope for this cluster's stub)."
    )
}

// ---------------------------------------------------------------------------
// Tests — BC-1.18.005 Red Gate (S-25.02 F4 BC-cluster 1 "cap+trigger")
// ---------------------------------------------------------------------------
//
// Every test below exercises a `todo!()` production function (or the fully
// wired `shard_cap_gate_check` dispatch entry point, itself `todo!()`) and
// therefore MUST currently FAIL (panic) — Red Gate per BC-5.38.001. The two
// GREEN-BY-DESIGN/WIRING-EXEMPT helpers this file already implements
// (`ShardEntry::cap_formula_inputs`, `From<ShardConfigError> for HookResult`)
// are intentionally NOT covered here — they are trivial field-copy /
// delegation code, not part of this BC's tested trigger/formula logic (see
// their own doc comments), and are outside the enumerated test-writer
// dispatch surface for this cluster.
//
// Scope boundary this suite deliberately respects: `shard_cap_gate_check`
// NEVER returns `HookResult::Block` itself (BC-1.18.006/BC-1.18.009 own the
// observable roll/rotate outcome once a trigger fires — see this BC's
// Postcondition 3/8 "Ownership" bullets and the module's own "Scope note").
// No test below asserts an outcome for the trigger-FIRES branch at the
// `shard_cap_gate_check` level; that branch's observable behavior belongs to
// the later BC-1.18.006/BC-1.18.009 clusters. The trigger-fires DECISION
// itself is fully covered via the lower-level `size_trigger_fires` /
// `item_count_trigger_fires` functions, which this BC does own.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Test fixture helpers
    // -----------------------------------------------------------------

    /// A well-formed `"flat"`-shaped entry using the BC's own provisional
    /// calibration constants (Postcondition 6), parameterized only on
    /// `artifact_stem` and `shard_cap_bytes` for per-test readability.
    fn flat_entry(stem: &str, shard_cap_bytes: u64) -> ShardEntry {
        ShardEntry {
            artifact_stem: stem.to_string(),
            practical_fuel_ceiling: 8_000_000,
            worst_case_fuel_per_byte: 106.36,
            max_single_record_bytes: 16_384,
            safety_margin: 8_192,
            shard_cap_bytes,
            shape: Some(ShardShape::Flat),
            n: None,
            low_water_mark: None,
        }
    }

    /// Renders a single `[[shard]]` TOML entry for the
    /// `"frontmatter-changelog-array"` shape, with an optional
    /// `low_water_mark` line (omitted entirely when `None`, exercising
    /// EC-010's config-load-time default path).
    fn shard_toml_frontmatter_entry(stem: &str, n: u64, low_water_mark: Option<i64>) -> String {
        let lwm_line = match low_water_mark {
            Some(v) => format!("low_water_mark = {v}\n"),
            None => String::new(),
        };
        format!(
            "[[shard]]\n\
             artifact_stem = \"{stem}\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 106.36\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 49152\n\
             shape = \"frontmatter-changelog-array\"\n\
             n = {n}\n\
             {lwm_line}"
        )
    }

    /// Minimal `tracing::Subscriber` that counts WARN-level events. Used to
    /// assert the EC-012 non-fatal `tracing::warn!` amortization advisory
    /// fires exactly when VP-140's biconditional requires
    /// (`low_water_mark > floor(N/2)`), without pulling in a
    /// `tracing-subscriber` dev-dependency.
    struct WarnCapture {
        warn_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl tracing::Subscriber for WarnCapture {
        fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}
        fn event(&self, event: &tracing::Event<'_>) {
            if *event.metadata().level() == tracing::Level::WARN {
                self.warn_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        fn enter(&self, _span: &tracing::span::Id) {}
        fn exit(&self, _span: &tracing::span::Id) {}
    }

    fn count_warns(
        f: impl FnOnce() -> Result<ShardRegistry, ShardConfigError>,
    ) -> (usize, Result<ShardRegistry, ShardConfigError>) {
        let warn_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let subscriber = WarnCapture {
            warn_count: warn_count.clone(),
        };
        let result = tracing::subscriber::with_default(subscriber, f);
        (warn_count.load(std::sync::atomic::Ordering::SeqCst), result)
    }

    // ===================================================================
    // Postcondition 1 / Invariant 3 / EC-001 — zero-cost bypass
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC1_find_matching_entry_returns_entry_for_matching_stem() {
        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let target = Path::new("/repo/.factory/cycles/pass-1/decision-log.md");
        assert_eq!(
            find_matching_entry(&registry, target),
            Some(&registry.shards[0]),
            "PC1: a target path whose stem matches a [[shard]] entry's artifact_stem MUST resolve to that entry"
        );
    }

    #[test]
    fn test_BC_1_18_005_AC_001_PC1_EC_001_find_matching_entry_returns_none_for_unmatched_path() {
        let registry = ShardRegistry {
            shards: vec![
                flat_entry("decision-log", 49_152),
                flat_entry("lessons", 49_152),
            ],
        };
        let target = Path::new("/repo/some/unrelated/file.md");
        assert_eq!(
            find_matching_entry(&registry, target),
            None,
            "EC-001: a target path matching no [[shard]] entry's artifact_stem MUST return None (zero-cost bypass)"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_BC_1_18_005_INV3_EC_001_shard_cap_gate_check_unmatched_path_never_pays_stat_cost() {
        // A self-referential symlink makes ANY stat()/metadata() call on this
        // path fail with ELOOP. This path's stem does NOT match any [[shard]]
        // entry, so a correct implementation MUST short-circuit (Invariant 3)
        // BEFORE ever attempting to stat() it — the call must cleanly return
        // Continue despite the landmine. A naive "always stat, then check
        // match" implementation would instead surface the ELOOP failure
        // (as an io::Error it has nowhere sound to route, or a panic).
        let dir = tempfile::tempdir().expect("tempdir");
        let looped = dir.path().join("unmatched-canary.md");
        std::os::unix::fs::symlink(&looped, &looped).expect("create self-referential symlink");

        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            &looped,
            &serde_json::json!({"content": "hi"}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "INV3/EC-001: an unmatched path MUST return Continue with NO stat() call — a stat() \
             attempt on this self-referential symlink would surface ELOOP, not a clean Continue"
        );
    }

    #[test]
    fn test_BC_1_18_005_INV1_shard_cap_gate_check_unmatched_path_returns_continue() {
        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let target = Path::new("/repo/some/unrelated/file.md");
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            target,
            &serde_json::json!({"content": "hello"}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "PC1/EC-001: no [[shard]] match MUST return Continue"
        );
    }

    // ===================================================================
    // Postcondition 4/6/7 — byte-size-denominated cap formula (AC-004)
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC4_compute_shard_cap_bytes_clean_round_numbers() {
        let inputs = CapFormulaInputs {
            practical_fuel_ceiling: 1_000,
            worst_case_fuel_per_byte: 10.0,
            max_single_record_bytes: 10,
            safety_margin: 5,
        };
        assert_eq!(
            compute_shard_cap_bytes(&inputs),
            85,
            "PC4: shard_cap_bytes = floor(PRACTICAL_FUEL_CEILING / WORST_CASE_FUEL_PER_BYTE) \
             - MAX_SINGLE_RECORD_BYTES - SAFETY_MARGIN"
        );
    }

    #[test]
    fn test_BC_1_18_005_PC6_compute_shard_cap_bytes_bc_provisional_worked_example() {
        // BC-1.18.005 Postcondition 6 worked example (today's provisional
        // constants): floor(8,000,000 / 106.36) - 16,384 - 8,192
        //           = 75,216 - 24,576 = 50,640.
        // This is the formula's own raw ceiling output — the BC's separate,
        // more-conservative editorial choice of a 49,152-byte (48 KiB)
        // provisional `shard_cap_bytes` CONFIG value is a human-chosen value
        // satisfying Postcondition 4's `shard_cap_bytes <= (formula)`
        // constraint (49,152 <= 50,640), not something this function itself
        // computes or rounds to.
        let inputs = CapFormulaInputs {
            practical_fuel_ceiling: 8_000_000,
            worst_case_fuel_per_byte: 106.36,
            max_single_record_bytes: 16_384,
            safety_margin: 8_192,
        };
        assert_eq!(
            compute_shard_cap_bytes(&inputs),
            50_640,
            "PC6: formula output for today's provisional constants MUST equal 50,640"
        );
    }

    // ===================================================================
    // Postcondition 5 — Cross-Validator Minimum Rule (AC-003)
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_AC_003_PC5_effective_shard_cap_bytes_burst_log_min_of_three_validators() {
        // Canonical vector: burst-log.md read by validate-burst-log(40000),
        // regression-gate(49152), convergence-tracker(52000) -> MIN = 40,000.
        assert_eq!(
            effective_shard_cap_bytes(&[40_000, 49_152, 52_000]),
            40_000,
            "PC5 Cross-Validator Minimum Rule: effective cap = MIN across all Cohort B readers"
        );
    }

    #[test]
    fn test_BC_1_18_005_AC_003_PC5_effective_shard_cap_bytes_decision_log_min_of_two_validators() {
        // Canonical vector: decision-log.md NOT read by validate-burst-log ->
        // MIN(regression-gate=49152, convergence-tracker=52000) = 49,152.
        assert_eq!(
            effective_shard_cap_bytes(&[49_152, 52_000]),
            49_152,
            "PC5: validate-burst-log's (possibly tighter) cap MUST NOT apply to artifacts it doesn't read"
        );
    }

    #[test]
    fn test_BC_1_18_005_effective_shard_cap_bytes_single_validator_returns_itself() {
        assert_eq!(effective_shard_cap_bytes(&[49_152]), 49_152);
    }

    #[test]
    fn test_BC_1_18_005_PC5_effective_shard_cap_bytes_min_is_order_independent() {
        assert_eq!(
            effective_shard_cap_bytes(&[52_000, 40_000, 49_152]),
            40_000,
            "PC5: MIN must be order-independent"
        );
    }

    // ===================================================================
    // Postcondition 2 / EC-004 — stat()-only current-shard byte-size read
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC2_current_shard_bytes_flat_reads_real_file_size() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("decision-log.md");
        std::fs::write(&path, vec![b'x'; 1_234]).expect("write fixture");
        let size =
            current_shard_bytes_flat(&path).expect("stat() must succeed for an existing file");
        assert_eq!(
            size, 1_234,
            "PC2: current_shard_bytes_flat MUST report the file's real byte size via stat()/metadata()"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_004_current_shard_bytes_flat_missing_file_is_ok_zero() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("never-written.md");
        assert!(!path.exists(), "precondition: fixture path must not exist");
        let size = current_shard_bytes_flat(&path)
            .expect("EC-004: a missing shard file MUST be Ok(0), not an io::Error");
        assert_eq!(
            size, 0,
            "EC-004: first write ever -> current_shard_bytes treated as 0"
        );
    }

    // ===================================================================
    // Postcondition 3 — per-tool-semantics projected-size formula (AC-002)
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC3_tool_kind_from_tool_name_write() {
        assert_eq!(ToolKind::from_tool_name("Write"), Some(ToolKind::Write));
    }

    #[test]
    fn test_BC_1_18_005_PC3_tool_kind_from_tool_name_edit() {
        assert_eq!(ToolKind::from_tool_name("Edit"), Some(ToolKind::Edit));
    }

    #[test]
    fn test_BC_1_18_005_PC3_tool_kind_from_tool_name_multi_edit() {
        assert_eq!(
            ToolKind::from_tool_name("MultiEdit"),
            Some(ToolKind::MultiEdit)
        );
    }

    #[test]
    fn test_BC_1_18_005_tool_kind_from_tool_name_unknown_tool_returns_none() {
        assert_eq!(ToolKind::from_tool_name("Bash"), None);
        assert_eq!(ToolKind::from_tool_name("Read"), None);
        assert_eq!(ToolKind::from_tool_name(""), None);
    }

    #[test]
    fn test_BC_1_18_005_vector_write_5000_bytes_under_cap_continues() {
        // Canonical vector: Write, content=5,000 bytes, cap=49,152 -> Continue
        // (current_shard_bytes is irrelevant to the Write formula).
        let projected = projected_size_write(5_000);
        assert_eq!(
            projected, 5_000,
            "PC3 CORRECTED Write leg: projected_size = len(content) alone"
        );
        assert!(
            !size_trigger_fires(projected, 49_152),
            "5,000 <= 49,152 must NOT trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_vector_write_50000_bytes_over_cap_triggers() {
        // Canonical vector: Write, content=50,000 bytes, cap=49,152 -> roll triggers.
        let projected = projected_size_write(50_000);
        assert_eq!(projected, 50_000);
        assert!(
            size_trigger_fires(projected, 49_152),
            "50,000 > 49,152 MUST trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_regression_same_size_write_does_not_double_count_current_shard_bytes() {
        // NEW regression vector (fix-burst pass-2, F-P2-002): current shard
        // 40,000 bytes, content 40,000 bytes (same-size full-file rewrite) ->
        // Continue. The WITHDRAWN formula would have computed
        // 40,000+40,000=80,000>49,152 and wrongly rolled.
        let current_shard_bytes_irrelevant_to_write = 40_000u64;
        let projected = projected_size_write(40_000);
        assert_eq!(
            projected, 40_000,
            "Write projected_size MUST equal len(content) alone, current_shard_bytes \
             ({current_shard_bytes_irrelevant_to_write}) MUST NOT be added"
        );
        assert!(
            !size_trigger_fires(projected, 49_152),
            "regression: same-size full-file Write MUST NOT trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_net_delta_bytes_for_edit_positive_delta() {
        assert_eq!(
            net_delta_bytes_for_edit(1_000, 6_000),
            5_000,
            "net_delta_bytes_for_edit = len(new_string) - len(old_string)"
        );
    }

    #[test]
    fn test_BC_1_18_005_net_delta_bytes_for_edit_negative_delta_shrinks() {
        assert_eq!(
            net_delta_bytes_for_edit(6_000, 1_000),
            -5_000,
            "net_delta_bytes_for_edit is signed — a shrinking edit MUST be negative"
        );
    }

    #[test]
    fn test_BC_1_18_005_vector_edit_current_45000_plus_5000_net_delta_triggers() {
        // Canonical vector: Edit, current shard 45,000, net +5,000, cap=49,152
        // -> Edit/MultiEdit formula UNCHANGED: projected = 50,000 > 49,152 -> roll triggers.
        let net = net_delta_bytes_for_edit(1_000, 6_000);
        let projected = projected_size_edit(45_000, net);
        assert_eq!(
            projected, 50_000,
            "Edit/MultiEdit leg UNCHANGED: projected_size = current_shard_bytes + net_delta_bytes"
        );
        assert!(
            size_trigger_fires(projected, 49_152),
            "50,000 > 49,152 MUST trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_005_vector_multi_edit_lessons_mixed_sign_deltas_triggers() {
        // Canonical vector: MultiEdit on lessons.md, edits netting
        // +2,000/-500/+100, current shard 48,000, cap=49,152 -> net=1,600;
        // projected=49,600 > 49,152 -> roll triggers.
        let edits = [
            EditDelta {
                old_len_bytes: 0,
                new_len_bytes: 2_000,
            },
            EditDelta {
                old_len_bytes: 500,
                new_len_bytes: 0,
            },
            EditDelta {
                old_len_bytes: 0,
                new_len_bytes: 100,
            },
        ];
        let net = net_delta_bytes_for_multi_edit(&edits);
        assert_eq!(
            net, 1_600,
            "EC-005: MultiEdit net_delta_bytes = SUM of per-edit-block net deltas"
        );
        let projected = projected_size_edit(48_000, net);
        assert_eq!(projected, 49_600);
        assert!(
            size_trigger_fires(projected, 49_152),
            "49,600 > 49,152 MUST trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_005_multi_edit_net_shrinking_never_triggers_even_with_large_individual_blocks()
     {
        // "a net-shrinking MultiEdit never triggers a roll even if individual
        // edit blocks are large" — one block is a large +1,000, another a
        // large -5,000 deletion; net shrinks overall.
        let edits = [
            EditDelta {
                old_len_bytes: 0,
                new_len_bytes: 1_000,
            },
            EditDelta {
                old_len_bytes: 5_000,
                new_len_bytes: 0,
            },
        ];
        let net = net_delta_bytes_for_multi_edit(&edits);
        assert_eq!(net, -4_000);
        let projected = projected_size_edit(49_000, net);
        assert_eq!(projected, 45_000);
        assert!(
            !size_trigger_fires(projected, 49_152),
            "EC-005: a net-shrinking MultiEdit MUST NOT trigger a roll even with large individual blocks"
        );
    }

    #[test]
    fn test_BC_1_18_005_projected_size_edit_saturates_at_zero_for_underflowing_negative_delta() {
        // Doc-mandated production-grade behavior (Postcondition 3's Edit/
        // MultiEdit leg doc comment): a net_delta_bytes more negative than
        // current_shard_bytes MUST saturate at 0, never underflow/panic.
        assert_eq!(
            projected_size_edit(100, -500),
            0,
            "projected_size_edit MUST saturate at 0 for a large negative net_delta_bytes, never underflow"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_002_size_trigger_fires_false_at_exact_boundary() {
        assert!(
            !size_trigger_fires(49_152, 49_152),
            "EC-002: projected_size == shard_cap_bytes MUST NOT trigger (inclusive <=)"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_003_size_trigger_fires_true_one_byte_over_boundary() {
        assert!(
            size_trigger_fires(49_153, 49_152),
            "EC-003: projected_size == shard_cap_bytes + 1 MUST trigger"
        );
    }

    #[test]
    fn test_BC_1_18_005_size_trigger_fires_false_comfortably_under_cap() {
        assert!(!size_trigger_fires(1, 49_152));
    }

    #[test]
    fn test_BC_1_18_005_regression_shard_cap_gate_check_same_size_write_continues() {
        // Full-stack regression vector via the top-level dispatch entry
        // point: current shard 40,000 bytes on disk, Write content also
        // 40,000 bytes -> Continue (the withdrawn formula would have wrongly
        // rolled here).
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("decision-log.md");
        std::fs::write(&target, "y".repeat(40_000)).expect("write fixture shard");
        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let content = "z".repeat(40_000);
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            &target,
            &serde_json::json!({"content": content}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "regression: same-size full-file Write MUST NOT trigger a roll"
        );
    }

    #[test]
    fn test_BC_1_18_005_shard_cap_gate_check_write_under_cap_continues() {
        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let target = Path::new("/repo/.factory/decision-log.md");
        let content = "x".repeat(5_000);
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            target,
            &serde_json::json!({"content": content}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "matched Write with projected_size 5,000 <= cap 49,152 MUST Continue"
        );
    }

    // ===================================================================
    // Postcondition 8 — item-count trigger (AC-005)
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC8_read_changelog_item_count_counts_yaml_list_items() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("BC-INDEX.md");
        std::fs::write(
            &path,
            "---\n\
             title: \"BC-INDEX\"\n\
             changelog:\n\
             \x20\x20- version: \"1.0\"\n\
             \x20\x20\x20\x20date: \"2026-01-01\"\n\
             \x20\x20- version: \"1.1\"\n\
             \x20\x20\x20\x20date: \"2026-01-02\"\n\
             \x20\x20- version: \"1.2\"\n\
             \x20\x20\x20\x20date: \"2026-01-03\"\n\
             ---\n\n# Body\n",
        )
        .expect("write fixture");
        let count = read_changelog_item_count(&path)
            .expect("PC8: read_changelog_item_count must succeed for a well-formed frontmatter changelog array");
        assert_eq!(
            count, 3,
            "PC8: item count MUST equal the number of changelog: array entries"
        );
    }

    #[test]
    fn test_BC_1_18_005_PC8_read_changelog_item_count_empty_array_is_zero() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("BC-INDEX.md");
        std::fs::write(
            &path,
            "---\ntitle: \"BC-INDEX\"\nchangelog: []\n---\n\n# Body\n",
        )
        .expect("write fixture");
        let count = read_changelog_item_count(&path)
            .expect("read must succeed for an empty changelog: array");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_BC_1_18_005_vector_item_count_trigger_10_of_50_continues() {
        // Canonical vector: N=50, current 10 items -> 11 <= 50 -> Continue.
        assert!(!item_count_trigger_fires(10, 50));
    }

    #[test]
    fn test_BC_1_18_005_EC_008_vector_item_count_trigger_fires_at_exactly_n() {
        // Canonical vector: N=50, current 50 items -> 51 > 50 -> fires.
        assert!(item_count_trigger_fires(50, 50));
    }

    #[test]
    fn test_BC_1_18_005_VP_140_item_count_trigger_does_not_fire_at_n_minus_1() {
        assert!(
            !item_count_trigger_fires(49, 50),
            "N-1 items -> current_item_count+1 == N -> must NOT fire"
        );
    }

    #[test]
    fn test_BC_1_18_005_VP_140_item_count_trigger_fires_at_n_plus_1() {
        assert!(item_count_trigger_fires(51, 50), "N+1 items -> must fire");
    }

    #[test]
    fn test_BC_1_18_005_shard_cap_gate_check_item_count_happy_path_continues() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("BC-INDEX.md");
        // 10 changelog items -> 11 <= 50 -> Continue.
        let mut body = String::from("---\ntitle: \"BC-INDEX\"\nchangelog:\n");
        for i in 0..10 {
            body.push_str(&format!("  - version: \"1.{i}\"\n"));
        }
        body.push_str("---\n\n# Body\n");
        std::fs::write(&target, body).expect("write fixture");

        let mut entry = flat_entry("BC-INDEX", 49_152);
        entry.shape = Some(ShardShape::FrontmatterChangelogArray);
        entry.n = Some(50);
        let registry = ShardRegistry {
            shards: vec![entry],
        };

        let result = shard_cap_gate_check(
            &registry,
            "Edit",
            &target,
            &serde_json::json!({"old_string": "a", "new_string": "ab"}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "item-count happy path: 10 items -> 11 <= N=50 -> Continue"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_010_resolved_low_water_mark_defaults_to_floor_n_div_2_when_omitted() {
        assert_eq!(resolved_low_water_mark(50, None), 25);
    }

    #[test]
    fn test_BC_1_18_005_resolved_low_water_mark_floor_of_odd_n_when_omitted() {
        assert_eq!(resolved_low_water_mark(51, None), 25, "floor(51/2) = 25");
    }

    #[test]
    fn test_BC_1_18_005_resolved_low_water_mark_uses_explicit_value_when_present() {
        assert_eq!(resolved_low_water_mark(50, Some(10)), 10);
    }

    #[test]
    fn test_BC_1_18_005_EC_011_validate_low_water_mark_rejects_equal_to_n() {
        let err = validate_low_water_mark("BC-INDEX", 50, 50)
            .expect_err("EC-011: low_water_mark == N (degenerate boundary) MUST fail-loud");
        match err {
            ShardConfigError::InvalidLowWaterMark {
                artifact_stem,
                n,
                low_water_mark,
            } => {
                assert_eq!(artifact_stem, "BC-INDEX");
                assert_eq!(n, 50);
                assert_eq!(low_water_mark, 50);
            }
            other => panic!("expected InvalidLowWaterMark, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_EC_011_validate_low_water_mark_rejects_negative() {
        let err = validate_low_water_mark("BC-INDEX", 50, -1)
            .expect_err("EC-011: negative low_water_mark MUST fail-loud");
        assert!(matches!(err, ShardConfigError::InvalidLowWaterMark { .. }));
    }

    #[test]
    fn test_BC_1_18_005_EC_011_validate_low_water_mark_rejects_greater_than_n() {
        let err = validate_low_water_mark("BC-INDEX", 50, 100)
            .expect_err("low_water_mark > N MUST fail-loud too, not just the == N boundary");
        assert!(matches!(err, ShardConfigError::InvalidLowWaterMark { .. }));
    }

    #[test]
    fn test_BC_1_18_005_EC_012_validate_low_water_mark_n_minus_1_is_valid_and_fires_advisory() {
        // Canonical vector: N=50, low_water_mark=49 (N-1) -> Ok(true) (fires
        // advisory), never Err.
        let fires = validate_low_water_mark("BC-INDEX", 50, 49)
            .expect("EC-012: N-1 MUST load successfully, never HookResult::Error");
        assert!(fires, "EC-012: 49 > floor(50/2)=25 -> advisory MUST fire");
    }

    #[test]
    fn test_BC_1_18_005_VP_140_validate_low_water_mark_default_value_does_not_fire_advisory() {
        // low_water_mark == floor(N/2) exactly (the recommended default) -> Ok(false).
        let fires =
            validate_low_water_mark("BC-INDEX", 50, 25).expect("25 is legal (0 <= 25 < 50)");
        assert!(
            !fires,
            "VP-140: low_water_mark == floor(N/2) MUST NOT fire the advisory (not strictly greater)"
        );
    }

    #[test]
    fn test_BC_1_18_005_VP_140_validate_low_water_mark_floor_plus_one_fires_advisory() {
        let fires =
            validate_low_water_mark("BC-INDEX", 50, 26).expect("26 is legal (0 <= 26 < 50)");
        assert!(
            fires,
            "VP-140: low_water_mark == floor(N/2)+1 MUST fire the advisory"
        );
    }

    #[test]
    fn test_BC_1_18_005_validate_low_water_mark_zero_is_valid_and_does_not_fire_advisory() {
        let fires = validate_low_water_mark("BC-INDEX", 50, 0).expect("0 is legal (0 <= 0 < 50)");
        assert!(!fires);
    }

    #[test]
    fn test_BC_1_18_005_load_valid_flat_shape_entry_succeeds() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"decision-log\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 106.36\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 49152\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");
        let registry = ShardRegistry::load(&cfg_path)
            .expect("Precondition 2: a well-formed [[shard]] config MUST load");
        assert_eq!(registry.shards.len(), 1);
        assert_eq!(registry.shards[0].artifact_stem, "decision-log");
        assert_eq!(registry.shards[0].shape, Some(ShardShape::Flat));
    }

    #[test]
    fn test_BC_1_18_005_EC_009_load_missing_shape_field_is_fail_loud() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"decision-log\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 106.36\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 49152\n",
        )
        .expect("write fixture");
        let err = ShardRegistry::load(&cfg_path)
            .expect_err("EC-009: an entry omitting `shape` entirely MUST be a config error, never a silent default");
        match err {
            ShardConfigError::MissingShape { artifact_stem } => {
                assert_eq!(artifact_stem, "decision-log");
            }
            other => panic!("expected MissingShape, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_EC_010_load_frontmatter_changelog_array_omits_low_water_mark_loads_ok() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, None),
        )
        .expect("write fixture");
        let registry = ShardRegistry::load(&cfg_path)
            .expect("EC-010: omitting low_water_mark MUST NOT fail config load");
        assert_eq!(
            registry.shards[0].low_water_mark, None,
            "load itself does not fill in the floor(N/2) default — resolved_low_water_mark is the caller's job"
        );
        assert_eq!(registry.shards[0].n, Some(50));
    }

    #[test]
    fn test_BC_1_18_005_EC_011_load_rejects_low_water_mark_equal_to_n() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, Some(50)),
        )
        .expect("write fixture");
        let err = ShardRegistry::load(&cfg_path)
            .expect_err("EC-011: low_water_mark == N MUST fail-loud at load time");
        assert!(matches!(err, ShardConfigError::InvalidLowWaterMark { .. }));
    }

    #[test]
    fn test_BC_1_18_005_EC_011_load_rejects_negative_low_water_mark() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, Some(-1)),
        )
        .expect("write fixture");
        let err = ShardRegistry::load(&cfg_path)
            .expect_err("EC-011: negative low_water_mark MUST fail-loud at load time");
        assert!(matches!(err, ShardConfigError::InvalidLowWaterMark { .. }));
    }

    #[test]
    fn test_BC_1_18_005_EC_012_load_accepts_n_minus_1_low_water_mark_and_succeeds() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, Some(49)),
        )
        .expect("write fixture");
        let registry = ShardRegistry::load(&cfg_path).expect(
            "EC-012: low_water_mark = N-1 = 49 MUST load successfully, never HookResult::Error",
        );
        assert_eq!(registry.shards[0].low_water_mark, Some(49));
    }

    #[test]
    fn test_BC_1_18_005_EC_012_load_emits_warn_advisory_when_low_water_mark_exceeds_default() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, Some(49)),
        )
        .expect("write fixture");

        let (warn_count, result) = count_warns(|| ShardRegistry::load(&cfg_path));

        assert!(
            result.is_ok(),
            "EC-012: low_water_mark=49 (N-1, N=50) MUST load successfully: {result:?}"
        );
        assert_eq!(
            warn_count, 1,
            "EC-012/VP-140: config-load MUST emit exactly one non-fatal tracing::warn! \
             amortization advisory when low_water_mark(49) > floor(N/2)(25)"
        );
    }

    #[test]
    fn test_BC_1_18_005_VP_140_load_default_low_water_mark_does_not_emit_warn_advisory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, Some(25)),
        )
        .expect("write fixture");

        let (warn_count, result) = count_warns(|| ShardRegistry::load(&cfg_path));

        assert!(result.is_ok());
        assert_eq!(
            warn_count, 0,
            "low_water_mark == floor(N/2) (the recommended default) MUST NOT fire the amortization advisory"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_010_load_omitted_low_water_mark_does_not_emit_warn_advisory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            shard_toml_frontmatter_entry("BC-INDEX", 50, None),
        )
        .expect("write fixture");

        let (warn_count, result) = count_warns(|| ShardRegistry::load(&cfg_path));

        assert!(result.is_ok());
        assert_eq!(
            warn_count, 0,
            "EC-010: an omitted low_water_mark resolves to floor(N/2) — the default itself — \
             and MUST NOT fire the advisory"
        );
    }
}
