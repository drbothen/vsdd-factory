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
//! This module implements BC-1.18.005 ONLY (tasks T-1/T-2/T-3; AC-001..AC-005)
//! — fully, not as a stub; see the "BC-5.38.001 Red Gate discipline" section
//! below. BC-1.18.006 (the observable roll/block outcome once the `"flat"` trigger
//! fires), BC-1.18.009 (the observable rotate/block-and-retry outcome once
//! the item-count trigger fires), and BC-1.18.012 (the one-time changelog
//! backfill migration) are LATER clusters and are explicitly OUT OF SCOPE
//! here — this module owns the trigger-boundary decision and the hand-off
//! point only, per Postcondition 3's and Postcondition 8's "Ownership"
//! bullets.
//!
//! # BC-5.38.001 Red Gate discipline — implemented (S-25.02 F4 BC-cluster 1)
//!
//! Every function below now carries a real implementation driving the
//! test-writer's Red Gate suite green. A fired trigger (either shape) never
//! constructs `HookResult::Block` from this module — that observable
//! roll/rotate-and-retry outcome is owned by the later BC-1.18.006 /
//! BC-1.18.009 clusters (see the "Scope note" above); this module surfaces a
//! fired trigger as a non-fatal `tracing::warn!` advisory and returns
//! `Continue`, an honest hand-off rather than a fabricated block.

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
/// `MissingShape` (EC-009), `InvalidLowWaterMark` (EC-011),
/// `CapExceedsFormulaCeiling` (Postcondition 9 / EC-013),
/// `InvalidWorstCaseFuelPerByte` (Postcondition 9's divisor-door closure /
/// EC-015), `MissingN` (Postcondition 8's load-time `N`-presence
/// requirement / EC-016), and `FormulaCeilingSaturated` (Postcondition 9's
/// residual divisor-door closure / EC-017) are fail-loud conditions this
/// BC's config surface owns — all are NEVER silently defaulted or clamped
/// around.
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

    /// Postcondition 9 / EC-013: a `[[shard]]` entry declares `shard_cap_bytes`
    /// GREATER than `compute_shard_cap_bytes(entry.cap_formula_inputs())` —
    /// the declared cap exceeds its own formula-derived ceiling. Applies to
    /// EVERY `shape`, not `"flat"`-only (Postcondition 8 already establishes
    /// that `shard_cap_bytes` bounds an artifact's TOTAL byte footprint even
    /// for the `"frontmatter-changelog-array"` shape). Fail-loud: NEVER
    /// silently accepted, NEVER silently clamped down to the computed
    /// ceiling — mirrors EC-009/EC-011's established fail-loud-at-load-time
    /// posture for this same config surface. The `==` boundary is legal
    /// (Postcondition 4's `<=` comparison is inclusive — EC-002 precedent).
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" declares shard_cap_bytes = \
         {shard_cap_bytes}, which EXCEEDS its own formula-derived ceiling \
         compute_shard_cap_bytes(inputs) = {computed_ceiling} (BC-1.18.005 Postcondition 9 / \
         EC-013). Fail-loud: never silently accepted, never silently clamped down to the \
         computed ceiling."
    )]
    CapExceedsFormulaCeiling {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
        /// The entry's own declared `shard_cap_bytes`.
        shard_cap_bytes: u64,
        /// `compute_shard_cap_bytes(entry.cap_formula_inputs())` — the ceiling
        /// the entry's OWN four formula inputs justify.
        computed_ceiling: u64,
    },

    /// EC-015 (BC-1.18.005 v1.10, Postcondition 9's "divisor-door" closure
    /// sub-bullet): a `[[shard]]` entry declares `worst_case_fuel_per_byte`
    /// as non-finite (`NaN`, `inf`, `-inf`) or `<= 0.0` (including `0.0`
    /// itself — the divisor-door case that saturates
    /// `compute_shard_cap_bytes`'s division to near-`u64::MAX`, defeating
    /// the `CapExceedsFormulaCeiling` check above for ANY declared
    /// `shard_cap_bytes`). Fail-loud: rejected BEFORE the cap-vs-formula
    /// comparison is even attempted. `practical_fuel_ceiling`,
    /// `max_single_record_bytes`, and `safety_margin` need no analogous
    /// check — all three are `u64`-typed (TOML excludes non-finite/negative
    /// values for them) and a degenerate `0` on any of them drives the
    /// ceiling toward the SAFE direction (0 via `saturating_sub`), never the
    /// exploitable-widening direction only `worst_case_fuel_per_byte`'s
    /// `f64` type creates.
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" declares \
         worst_case_fuel_per_byte = {worst_case_fuel_per_byte}, which must be finite and > 0.0 \
         (BC-1.18.005 EC-015). Fail-loud: never silently substituted with a default rate, and \
         the cap-vs-formula comparison (Postcondition 9) is never silently skipped for this \
         entry."
    )]
    InvalidWorstCaseFuelPerByte {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
        /// The entry's configured (invalid) `worst_case_fuel_per_byte`.
        worst_case_fuel_per_byte: f64,
    },

    /// EC-016 (BC-1.18.005 v1.10, Postcondition 8's load-time presence
    /// requirement for `N`): a `"frontmatter-changelog-array"`-shaped entry
    /// omits the required `N` item-count trigger threshold. Fail-loud AT
    /// LOAD TIME (not merely deferred to the first gate-time write against
    /// the artifact), evaluated BEFORE `low_water_mark` is examined, so an
    /// entry that also declares an invalid `low_water_mark` is never
    /// silently accepted merely because `N` happened to be absent.
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" declares \
         shape = \"frontmatter-changelog-array\" but omits the required `n` item-count trigger \
         threshold (BC-1.18.005 EC-016). Fail-loud: caught at ShardRegistry::load() time, not \
         deferred to the first gate-time write against the artifact."
    )]
    MissingN {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
    },

    /// EC-017 (BC-1.18.005 v1.11, Postcondition 9's "Residual divisor-door
    /// closure" sub-paragraph): a `[[shard]]` entry declares a
    /// `worst_case_fuel_per_byte` that is finite and `> 0.0` (passes
    /// EC-015's guard above) but is so small (e.g. `1e-300`) that the RAW,
    /// pre-cast division `practical_fuel_ceiling as f64 /
    /// worst_case_fuel_per_byte` is itself non-finite or `>= u64::MAX as
    /// f64` — i.e. the computed ceiling saturates the subsequent
    /// `.floor() as u64` cast to (or beyond) `u64::MAX`, even though the
    /// divisor itself is legal under EC-015. Distinct from
    /// `InvalidWorstCaseFuelPerByte`, which catches an illegal `<=
    /// 0.0`/non-finite DIVISOR itself, not a saturated RESULT arising from
    /// an otherwise-legal tiny-positive divisor. Fail-loud: rejected BEFORE
    /// the `CapExceedsFormulaCeiling` comparison is even attempted — a
    /// saturated ceiling cannot meaningfully bound any declared
    /// `shard_cap_bytes`, so it is never silently accepted and never
    /// silently substituted with a fallback ceiling.
    #[error(
        "[[shard]] entry for artifact_stem = \"{artifact_stem}\" declares \
         practical_fuel_ceiling = {practical_fuel_ceiling} and worst_case_fuel_per_byte = \
         {worst_case_fuel_per_byte}, whose raw division saturates the formula's ceiling \
         computation to (or beyond) u64::MAX (BC-1.18.005 EC-017, residual divisor-door \
         closure). Fail-loud: a saturated ceiling cannot meaningfully bound any declared \
         shard_cap_bytes, so it is never silently accepted and never silently substituted with \
         a fallback ceiling."
    )]
    FormulaCeilingSaturated {
        /// The offending entry's `artifact_stem`, so the operator can locate it.
        artifact_stem: String,
        /// The entry's configured `practical_fuel_ceiling`.
        practical_fuel_ceiling: u64,
        /// The entry's configured (legal-but-degenerate) `worst_case_fuel_per_byte`.
        worst_case_fuel_per_byte: f64,
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
    /// Validation (Preconditions 2 + EC-009/EC-010/EC-011/EC-012/Postcondition 9/EC-013):
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
    /// - `"frontmatter-changelog-array"`-shaped entries MUST declare `n`
    ///   (fail-loud [`ShardConfigError::MissingN`] — EC-016), checked BEFORE
    ///   `low_water_mark` is examined, so an entry missing `n` AND declaring
    ///   an out-of-range `low_water_mark` is never silently accepted merely
    ///   because `n` was absent.
    /// - EVERY entry's `worst_case_fuel_per_byte` MUST be finite and strictly
    ///   positive (fail-loud [`ShardConfigError::InvalidWorstCaseFuelPerByte`]
    ///   — EC-015's "divisor-door" closure), checked BEFORE
    ///   `compute_shard_cap_bytes` is even called for that entry — a `0.0`
    ///   or non-finite divisor would otherwise saturate the computed ceiling
    ///   toward `u64::MAX`, defeating the cap-vs-formula check below for ANY
    ///   declared `shard_cap_bytes`.
    /// - EVERY entry that passes the check above MUST ALSO have its RAW
    ///   `practical_fuel_ceiling as f64 / worst_case_fuel_per_byte` division
    ///   result be finite and `< u64::MAX as f64` (fail-loud
    ///   [`ShardConfigError::FormulaCeilingSaturated`] — EC-017's residual
    ///   divisor-door closure), checked BEFORE the cap-vs-formula comparison
    ///   below — a legal-but-tiny-positive divisor can still saturate the
    ///   computed ceiling even though it passes the check above.
    /// - EVERY entry, regardless of `shape`, MUST NOT declare `shard_cap_bytes`
    ///   greater than `compute_shard_cap_bytes(entry.cap_formula_inputs())`
    ///   (fail-loud [`ShardConfigError::CapExceedsFormulaCeiling`] —
    ///   Postcondition 9 / EC-013; the `==` boundary is inclusive, mirroring
    ///   EC-002's precedent for the per-write trigger).
    pub fn load(path: &Path) -> Result<Self, ShardConfigError> {
        let text = std::fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&text)?;

        for entry in &parsed.shards {
            // EC-009: `shape` is fail-loud-required, never silently defaulted.
            let shape = entry.shape.ok_or_else(|| ShardConfigError::MissingShape {
                artifact_stem: entry.artifact_stem.clone(),
            })?;

            // EC-015 (Postcondition 9's "divisor-door" closure): validate
            // BEFORE compute_shard_cap_bytes is called for this entry — a
            // 0.0 divisor makes the internal division yield +inf, whose
            // saturating `as u64` cast is u64::MAX, defeating the
            // cap-vs-formula comparison below for ANY declared
            // shard_cap_bytes. A NaN divisor is likewise non-finite and
            // rejected regardless of which direction its own degenerate
            // arithmetic happens to saturate. practical_fuel_ceiling,
            // max_single_record_bytes, and safety_margin need no analogous
            // check (all u64-typed; a degenerate 0 on any of them drives the
            // ceiling toward the safe direction, never the exploitable one).
            if !entry.worst_case_fuel_per_byte.is_finite() || entry.worst_case_fuel_per_byte <= 0.0
            {
                return Err(ShardConfigError::InvalidWorstCaseFuelPerByte {
                    artifact_stem: entry.artifact_stem.clone(),
                    worst_case_fuel_per_byte: entry.worst_case_fuel_per_byte,
                });
            }

            // EC-017 (Postcondition 9's "Residual divisor-door closure"):
            // the EC-015 guard above is NECESSARY but INSUFFICIENT — a legal
            // tiny-positive divisor (e.g. 1e-300) passes it yet still drives
            // the RAW, pre-cast division result to a value so large that
            // compute_shard_cap_bytes's subsequent `.floor() as u64` cast
            // saturates to u64::MAX, defeating the CapExceedsFormulaCeiling
            // comparison below for ANY declared shard_cap_bytes exactly as
            // EC-015's own divisor-door does. Evaluate the SAME raw f64
            // quantity compute_shard_cap_bytes computes internally, BEFORE
            // that comparison is attempted, and reject the entry if it is
            // non-finite or at/beyond the saturation boundary of the
            // subsequent lossy cast (u64::MAX as f64).
            let raw_ceiling_division =
                entry.practical_fuel_ceiling as f64 / entry.worst_case_fuel_per_byte;
            if !raw_ceiling_division.is_finite() || raw_ceiling_division >= u64::MAX as f64 {
                return Err(ShardConfigError::FormulaCeilingSaturated {
                    artifact_stem: entry.artifact_stem.clone(),
                    practical_fuel_ceiling: entry.practical_fuel_ceiling,
                    worst_case_fuel_per_byte: entry.worst_case_fuel_per_byte,
                });
            }

            // Postcondition 9 / EC-013: applies to EVERY entry regardless of
            // `shape` — Postcondition 8 already establishes that
            // `shard_cap_bytes` bounds an artifact's TOTAL byte footprint as
            // a whole-artifact concern even for the
            // `"frontmatter-changelog-array"` shape, so this is NOT a
            // `"flat"`-shape-only check.
            let computed_ceiling = compute_shard_cap_bytes(&entry.cap_formula_inputs());
            if entry.shard_cap_bytes > computed_ceiling {
                return Err(ShardConfigError::CapExceedsFormulaCeiling {
                    artifact_stem: entry.artifact_stem.clone(),
                    shard_cap_bytes: entry.shard_cap_bytes,
                    computed_ceiling,
                });
            }

            // Postcondition 8's rotation-target-config bullet / EC-010/EC-011/
            // EC-012 apply to the item-count shape's OPTIONAL explicit
            // low_water_mark only — an omitted value resolves to floor(N/2)
            // (EC-010) without ever running the fail-loud/advisory check
            // below (that default is, by construction, never poorly
            // amortizing and never invalid).
            if shape == ShardShape::FrontmatterChangelogArray {
                // EC-016 (Postcondition 8's load-time presence requirement
                // for `N`): required for this shape, evaluated BEFORE
                // low_water_mark is examined — the prior destructure-and-
                // short-circuit pattern (`if let (Some(n), Some(low_water_
                // mark)) = ...`) ran NO validation at all when `n` was
                // `None`, silently letting an entry that ALSO declared an
                // out-of-range low_water_mark load undetected. Requiring
                // `n`'s presence as an unconditional, shape-scoped check
                // first closes that gap structurally: every
                // FrontmatterChangelogArray entry now either (a) fails loud
                // here on a missing `n`, or (b) has `n` present, in which
                // case the existing low_water_mark numeric-range validation
                // below runs exactly as before.
                let Some(n) = entry.n else {
                    return Err(ShardConfigError::MissingN {
                        artifact_stem: entry.artifact_stem.clone(),
                    });
                };

                if let Some(low_water_mark) = entry.low_water_mark {
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
/// negative overall). A large negative delta MUST NOT underflow an unsigned
/// `current_shard_bytes` — this function uses a saturating (floor-at-zero)
/// computation, the production-grade choice VP-116's kani-proof verifies
/// for overflow/underflow safety.
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
    /// Deserialization target isolating just the `changelog:` sequence —
    /// mirrors `last-amended-migrate/src/yaml_guard.rs`'s `MinimalFrontmatter`
    /// pattern (extra frontmatter fields are ignored by default struct
    /// deserialization; no `deny_unknown_fields`).
    #[derive(Deserialize)]
    struct ChangelogFrontmatter {
        #[serde(default)]
        changelog: Option<Vec<serde_norway::Value>>,
    }

    let raw = match std::fs::read_to_string(target_path) {
        Ok(raw) => raw,
        // EC-014 (BC-1.18.005 v1.9): a not-yet-existing
        // "frontmatter-changelog-array"-shaped target file is a legitimate
        // first-ever Write CREATING it, not a fail-loud condition — treated
        // as holding 0 existing changelog items, mirroring
        // current_shard_bytes_flat's EC-004 NotFound->Ok(0) precedent above.
        // Any OTHER io::Error kind stays fail-loud.
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e),
    };
    // Opportunistic hardening (S-25.02 Phase F4 LOCAL adversary pass-1
    // cluster-1 observation): tolerate a `---\r\n` (CRLF) opening fence in
    // addition to `---\n`, so a CRLF-line-ended frontmatter file is not
    // spuriously treated as having no fence at all.
    let block = raw
        .strip_prefix("---\r\n")
        .or_else(|| raw.strip_prefix("---\n"))
        .and_then(|after_open| after_open.find("\n---").map(|end| &after_open[..end]))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}: no well-formed --- frontmatter fence found (BC-1.18.005 Postcondition 8)",
                    target_path.display()
                ),
            )
        })?;

    let parsed: ChangelogFrontmatter = serde_norway::from_str(block).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{}: frontmatter YAML parse failed: {e}",
                target_path.display()
            ),
        )
    })?;

    Ok(parsed
        .changelog
        .map(|items| items.len() as u64)
        .unwrap_or(0))
}

/// `true` iff `current_item_count + 1 > N` — the item-count trigger boundary
/// (Postcondition 8's trigger condition; EC-008 off-by-one: `N-1` items ->
/// `false`, exactly `N` or `N+1` items -> `true`).
///
/// NEVER a byte-size comparison for this shape — `shard_cap_bytes` still
/// bounds the artifact's total byte footprint as a whole-artifact concern,
/// but the rotation decision within this shape is item-count-based only.
pub fn item_count_trigger_fires(current_item_count: u64, n: u64) -> bool {
    current_item_count.saturating_add(1) > n
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
    match low_water_mark {
        // Caller-validated: already known non-negative and < N.
        Some(v) => v as u64,
        // EC-010: integer division floors naturally.
        None => n / 2,
    }
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
    // i128 comparison sidesteps any u64/i64 range mismatch for the `>= N`
    // check — N is realistically far below i64::MAX/u64::MAX for this BC's
    // item-count trigger, but the wider comparison type costs nothing and
    // removes the need to reason about cast overflow at the boundary.
    if low_water_mark < 0 || i128::from(low_water_mark) >= i128::from(n) {
        return Err(ShardConfigError::InvalidLowWaterMark {
            artifact_stem: artifact_stem.to_string(),
            n,
            low_water_mark,
        });
    }

    let default_low_water_mark = n / 2;
    // Safe: low_water_mark just checked >= 0 above.
    Ok(low_water_mark as u64 > default_low_water_mark)
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
///   cluster fully implements the trigger boundary and hand-off point (this
///   function's own logic is not a stub), and deliberately never implements
///   the roll/rotation behavior itself — that is later clusters' own scope.
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
    // Postcondition 1 / Invariant 3: unmatched paths return Continue before
    // any stat() call or shape dispatch.
    let Some(entry) = find_matching_entry(shard_registry, target_path) else {
        return HookResult::Continue;
    };

    let Some(shape) = entry.shape else {
        // Should never happen for an entry that went through
        // `ShardRegistry::load` (EC-009 rejects this at load time), but a
        // caller may construct a `ShardEntry` directly (e.g. in tests) —
        // fail loud rather than silently guessing a shape (mirrors EC-009's
        // posture for the same missing-shape condition).
        return ShardConfigError::MissingShape {
            artifact_stem: entry.artifact_stem.clone(),
        }
        .into();
    };

    match shape {
        ShardShape::Flat => {
            let Some(tool_kind) = ToolKind::from_tool_name(tool_name) else {
                // Not a candidate mutating tool for the "flat" byte-size
                // trigger — zero-cost-bypass spirit of Postcondition 1
                // extended to an unsupported tool kind.
                return HookResult::Continue;
            };

            // F-002 fix (S-25.02 Phase F4 LOCAL adversary pass-1 cluster-1,
            // MEDIUM): `current_shard_bytes_flat` (a stat()/metadata() call)
            // is ONLY needed by the Edit/MultiEdit legs of Postcondition 3's
            // CORRECTED formula — `Write`'s `projected_size = len(content)`
            // alone never consults `current_shard_bytes` at all. Calling it
            // unconditionally here (i.e. before this match) would fail a
            // `Write` on a stat() error the Write formula doesn't even need
            // (e.g. a non-`NotFound` I/O error such as `ELOOP`), which is
            // never sound: `Write` must never be blocked by a read it
            // doesn't perform. So the stat() call is pushed down into ONLY
            // the Edit/MultiEdit arms below, each of which does need
            // `current_shard_bytes` for the current+delta formula.
            // EC-004 (a missing shard file treated as size 0) is preserved:
            // `current_shard_bytes_flat` itself still maps `NotFound` to
            // `Ok(0)`, unchanged.
            let projected_size = match tool_kind {
                ToolKind::Write => {
                    let content_len = tool_input
                        .get("content")
                        .and_then(|v| v.as_str())
                        .map(|s| s.len() as u64)
                        .unwrap_or(0);
                    projected_size_write(content_len)
                }
                ToolKind::Edit => {
                    let current_bytes = match current_shard_bytes_flat(target_path) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            return HookResult::Error {
                                message: format!(
                                    "BC-1.18.005: failed to stat() shard '{}' for artifact_stem \
                                     \"{}\": {e}",
                                    target_path.display(),
                                    entry.artifact_stem
                                ),
                            };
                        }
                    };
                    let old_len = tool_input
                        .get("old_string")
                        .and_then(|v| v.as_str())
                        .map(|s| s.len() as u64)
                        .unwrap_or(0);
                    let new_len = tool_input
                        .get("new_string")
                        .and_then(|v| v.as_str())
                        .map(|s| s.len() as u64)
                        .unwrap_or(0);
                    let net_delta = net_delta_bytes_for_edit(old_len, new_len);
                    projected_size_edit(current_bytes, net_delta)
                }
                ToolKind::MultiEdit => {
                    let current_bytes = match current_shard_bytes_flat(target_path) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            return HookResult::Error {
                                message: format!(
                                    "BC-1.18.005: failed to stat() shard '{}' for artifact_stem \
                                     \"{}\": {e}",
                                    target_path.display(),
                                    entry.artifact_stem
                                ),
                            };
                        }
                    };
                    let edits: Vec<EditDelta> = tool_input
                        .get("edits")
                        .and_then(|v| v.as_array())
                        .map(|edits| {
                            edits
                                .iter()
                                .map(|edit| {
                                    let old_len = edit
                                        .get("old_string")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.len() as u64)
                                        .unwrap_or(0);
                                    let new_len = edit
                                        .get("new_string")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.len() as u64)
                                        .unwrap_or(0);
                                    EditDelta {
                                        old_len_bytes: old_len,
                                        new_len_bytes: new_len,
                                    }
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    let net_delta = net_delta_bytes_for_multi_edit(&edits);
                    projected_size_edit(current_bytes, net_delta)
                }
            };

            if size_trigger_fires(projected_size, entry.shard_cap_bytes) {
                // Postcondition 3's "Ownership" bullet: this BC owns the
                // trigger-boundary DECISION only. BC-1.18.006 owns the
                // observable roll/block outcome once it fires — that BC is
                // explicitly out of scope for this cluster (see this
                // module's own "Scope note"), so this function itself NEVER
                // constructs a `HookResult::Block` for a fired trigger. The
                // fired decision is still surfaced (non-fatally) via
                // `tracing::warn!` so it is visible in telemetry rather than
                // silently swallowed while the roll implementation lands.
                tracing::warn!(
                    artifact_stem = %entry.artifact_stem,
                    projected_size,
                    shard_cap_bytes = entry.shard_cap_bytes,
                    "BC-1.18.005: byte-size shard-cap trigger fired; roll/block outcome is \
                     owned by BC-1.18.006 (not yet implemented in this cluster) — allowing \
                     the call to proceed"
                );
            }

            HookResult::Continue
        }
        ShardShape::FrontmatterChangelogArray => {
            // O-C1-P4-001 (LOW, S-25.02 Phase F4 LOCAL adversary cluster-1
            // pass-4 observation): mirror the "flat" arm's
            // ToolKind::from_tool_name early-Continue guard here, so a
            // non-Edit/Write/MultiEdit tool_name never reads the target
            // file. Benign in production today (executor.rs's caller
            // already pre-filters to mutating tools before this function is
            // reached at all) but restores internal symmetry between the
            // two shape arms and avoids surfacing a spurious
            // HookResult::Error for a non-mutating tool call that happens
            // to hit a malformed/unreadable frontmatter fence.
            if ToolKind::from_tool_name(tool_name).is_none() {
                return HookResult::Continue;
            }

            // Postcondition 8's item-count trigger is state-based (current
            // item count in the file on disk), not payload-based — every
            // candidate tool call is evaluated identically regardless of
            // Edit/Write/MultiEdit shape.
            let Some(n) = entry.n else {
                return HookResult::Error {
                    message: format!(
                        "BC-1.18.005: artifact_stem \"{}\" declares \
                         shape = \"frontmatter-changelog-array\" but omits the required \
                         `n` item-count trigger threshold",
                        entry.artifact_stem
                    ),
                };
            };

            let current_item_count = match read_changelog_item_count(target_path) {
                Ok(count) => count,
                Err(e) => {
                    return HookResult::Error {
                        message: format!(
                            "BC-1.18.005: failed to read changelog: item count for \
                             artifact_stem \"{}\" at '{}': {e}",
                            entry.artifact_stem,
                            target_path.display()
                        ),
                    };
                }
            };

            if item_count_trigger_fires(current_item_count, n) {
                // Ownership bullet (Postcondition 8): BC-1.18.009 owns the
                // observable rotate-then-block-and-retry outcome once this
                // trigger fires — out of scope for this cluster (see this
                // module's own "Scope note"). Same non-Block, honest
                // hand-off posture as the "flat" shape's trigger-fired
                // branch above.
                tracing::warn!(
                    artifact_stem = %entry.artifact_stem,
                    current_item_count,
                    n,
                    "BC-1.18.005: item-count shard-cap trigger fired; rotate/block outcome is \
                     owned by BC-1.18.009 (not yet implemented in this cluster) — allowing \
                     the call to proceed"
                );
            }

            HookResult::Continue
        }
    }
}

// ---------------------------------------------------------------------------
// Tests — BC-1.18.005 (S-25.02 F4 BC-cluster 1 "cap+trigger")
// ---------------------------------------------------------------------------
//
// Every test below exercises a fully implemented production function (or the
// fully wired `shard_cap_gate_check` dispatch entry point) and is green. The
// two GREEN-BY-DESIGN/WIRING-EXEMPT helpers this file already implements
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

    /// Serializes every test in this module that can execute the
    /// `tracing::warn!` callsite inside `ShardRegistry::load` (BC-1.18.005
    /// EC-012's amortization advisory) — i.e. every test whose fixture has
    /// `low_water_mark > floor(N/2)`, whether or not that particular test
    /// cares about counting the resulting warn.
    ///
    /// `tracing-core`'s per-callsite `Interest` cache
    /// (`tracing_core::callsite`) is a single table shared by the whole
    /// **process**, keyed by source-location callsite — not by which
    /// `Subscriber` happens to be installed on a given thread. The *first*
    /// time this specific callsite is ever executed anywhere in this test
    /// binary, `tracing-core` performs a one-time `Interest` registration
    /// using whatever `Subscriber` is ambient **on the thread that gets
    /// there first**, and cheaply caches that verdict forever after
    /// (`DefaultCallsite`'s registration state machine never revisits a
    /// callsite once it flips from `UNREGISTERED` to `REGISTERED`). Two
    /// tests reach this exact callsite:
    /// `test_BC_1_18_005_EC_012_load_accepts_n_minus_1_low_water_mark_and_succeeds`
    /// (fixture `low_water_mark=49`, `n=50`) and
    /// `test_BC_1_18_005_EC_012_load_emits_warn_advisory_when_low_water_mark_exceeds_default`
    /// below (same fixture, but this one asserts the resulting warn count).
    /// Confirmed by instrumenting `WarnCapture` and re-running under
    /// `--test-threads>1`: if the FIRST test does not install a
    /// `WarnCapture` default before calling `ShardRegistry::load`, the
    /// ambient subscriber on its thread is the no-op default (never
    /// interested), so `Interest::never()` gets cached for the callsite
    /// **permanently** — and a later `tracing::subscriber::with_default`
    /// call does not, by itself, invalidate that cached decision:
    /// `with_default`/`set_default` only swap the thread-local dispatch,
    /// they do not call `tracing::callsite::rebuild_interest_cache()`.
    /// Whichever of the two tests the `cargo test` thread pool happens to
    /// schedule first therefore decides, non-deterministically, whether
    /// the warn is ever observable again in this process — this is
    /// exactly the intermittent (~2/5) failure this fixes.
    ///
    /// The fix requires BOTH tests that reach this callsite to be under
    /// the SAME discipline (see the two now-locked call sites below); a
    /// serialized capture-only helper does not, by itself, close the race
    /// against an unsynchronized non-capturing sibling. The discipline has
    /// two parts:
    ///
    /// 1. This `Mutex` ensures at most one such call is ever "in flight"
    ///    at once, so no two of these calls can race each other's
    ///    `Interest` recomputation or the callsite's one-time
    ///    registration.
    /// 2. `count_warns` calls `tracing::callsite::rebuild_interest_cache()`
    ///    *after* installing its `WarnCapture` subscriber as the thread's
    ///    default and *immediately before* invoking the code under test.
    ///    This forces `tracing-core` to unconditionally recompute the
    ///    warn! callsite's `Interest` against the currently-installed
    ///    (always-`enabled`) `WarnCapture` subscriber, overwriting any
    ///    `Interest::never()` left behind by an earlier unsynchronized
    ///    touch — regardless of registration history — and, when this is
    ///    the callsite's very first-ever touch, guarantees the ambient
    ///    dispatch at the moment of registration is this call's own
    ///    `WarnCapture` (same thread, same lock-protected scope), rather
    ///    than whatever happened to be ambient on some other thread.
    ///
    /// Since every reachable call site now goes through `count_warns`
    /// under the same lock, and every `Subscriber` ever installed here is
    /// a `WarnCapture` (always `enabled`), the callsite cannot be
    /// re-poisoned after that: any later recomputation triggered by
    /// another call's own `Dispatch::new` (see `tracing_core::callsite`'s
    /// "Rebuilding Cached Interest" docs) can only ever agree that the
    /// callsite is `Interest::always()`.
    static CAPTURE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn count_warns(
        f: impl FnOnce() -> Result<ShardRegistry, ShardConfigError>,
    ) -> (usize, Result<ShardRegistry, ShardConfigError>) {
        // A prior capture test panicking mid-assertion (lock still held via
        // its guard's unwind drop) must not cascade into every subsequent
        // capture test failing on a poisoned-lock panic; the poisoning
        // carries no information about *this* test's fixture, so recover
        // and proceed.
        let _guard = CAPTURE_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let warn_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let subscriber = WarnCapture {
            warn_count: warn_count.clone(),
        };
        let result = tracing::subscriber::with_default(subscriber, || {
            // See CAPTURE_LOCK's doc comment: force tracing-core to
            // recompute this callsite's cached `Interest` against the
            // `WarnCapture` subscriber just installed above, closing the
            // race against any non-capturing test that touched the same
            // `warn!` callsite first (and thus cached `Interest::never()`
            // against the ambient no-op default).
            tracing::callsite::rebuild_interest_cache();
            f()
        });
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
    // EC-014 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3
    // finding F-C1-P3-001, MEDIUM, product-owner adjudication, BC-1.18.005
    // v1.9) — item-count shape's missing-file graceful degradation,
    // mirroring EC-004's flat-shape precedent above. NOT fail-loud: a
    // legitimate first-ever Write that CREATES a not-yet-existing
    // "frontmatter-changelog-array"-shaped target artifact MUST be treated
    // as holding 0 existing changelog items, never hard-blocked as
    // HookResult::Error.
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_EC_014_read_changelog_item_count_missing_file_is_ok_zero() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("never-written-index.md");
        assert!(!path.exists(), "precondition: fixture path must not exist");
        let count = read_changelog_item_count(&path).expect(
            "EC-014: a missing frontmatter-changelog-array target file MUST be Ok(0), not an \
             io::Error — read_changelog_item_count must map io::ErrorKind::NotFound to Ok(0), \
             mirroring EC-004's current_shard_bytes_flat precedent",
        );
        assert_eq!(
            count, 0,
            "EC-014: first write ever -> current_item_count treated as 0, mirroring EC-004's \
             flat-shape missing-file precedent"
        );
    }

    #[test]
    fn test_BC_1_18_005_EC_014_shard_cap_gate_check_create_path_missing_file_continues() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Deliberately do NOT create the target file — this is the
        // first-ever Write that CREATES a new "frontmatter-changelog-array"-
        // shaped sharded index file, with valid frontmatter (including a
        // changelog: array) present in the content payload.
        let target = dir.path().join("BC-INDEX.md");
        assert!(
            !target.exists(),
            "precondition: fixture path must not exist"
        );

        let mut entry = flat_entry("BC-INDEX", 49_152);
        entry.shape = Some(ShardShape::FrontmatterChangelogArray);
        entry.n = Some(50);
        let registry = ShardRegistry {
            shards: vec![entry],
        };

        let content = "---\ntitle: \"BC-INDEX\"\nchangelog:\n  - version: \"1.0\"\n---\n\n# Body\n";
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            &target,
            &serde_json::json!({"content": content}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "EC-014: a Write that CREATES a not-yet-existing frontmatter-changelog-array-shaped \
             target MUST Continue — read_changelog_item_count's NotFound->Ok(0) mapping makes \
             current_item_count + 1 = 0 + 1 = 1 <= N=50, so the legitimate create is NEVER \
             hard-blocked as HookResult::Error"
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

        // Routed through `count_warns` (its warn count is unused here)
        // rather than a bare `ShardRegistry::load` call: this fixture's
        // `low_water_mark=49` (> floor(50/2)) walks through the SAME
        // `tracing::warn!` callsite the EC-012 *emission* test below
        // asserts on. See `CAPTURE_LOCK`'s doc comment — this is required,
        // not cosmetic: an uncoordinated call through that callsite can
        // win its one-time `Interest` registration using the ambient
        // no-op subscriber, permanently caching `Interest::never()` and
        // making the emission test's assertion flaky.
        let (_warn_count, result) = count_warns(|| ShardRegistry::load(&cfg_path));
        let registry = result.expect(
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

    // ===================================================================
    // F-002 (MED, S-25.02 Phase F4 LOCAL adversary pass-1 cluster-1) — a
    // Write against a [[shard]]-matched path whose current shard cannot be
    // stat()-ed for a reason OTHER than NotFound (e.g. EC-004's already-
    // covered missing-file case) MUST NOT be blocked when the Write's own
    // content length is under cap. Postcondition 3's CORRECTED Write leg
    // computes projected_size = len(content) ALONE — current_shard_bytes
    // (and, by extension, any failure reading it) is irrelevant to Write.
    // ===================================================================

    #[cfg(unix)]
    #[test]
    fn test_BC_1_18_005_F002_write_under_cap_continues_despite_irrelevant_stat_failure() {
        // Same portable technique as the existing unmatched-path zero-cost
        // canary test above (a self-referential symlink makes ANY
        // stat()/metadata() call on this path fail with ELOOP) — but here the
        // path's STEM MATCHES a [[shard]] entry, so the gate does NOT
        // short-circuit before stat(); it must instead recognize that a
        // Write's formula never needs current_shard_bytes at all.
        //
        // Portability caveat: ELOOP via a self-referential symlink is a
        // Unix-only technique (`std::os::unix::fs::symlink`), hence
        // `#[cfg(unix)]` — mirroring this file's own existing precedent for
        // the same landmine (`test_BC_1_18_005_INV3_EC_001_...`). No portable
        // cross-platform non-NotFound stat() failure was substituted because
        // this exact technique is already the codebase's established pattern
        // for this class of test.
        let dir = tempfile::tempdir().expect("tempdir");
        let looped = dir.path().join("decision-log.md");
        std::os::unix::fs::symlink(&looped, &looped).expect("create self-referential symlink");

        let registry = ShardRegistry {
            shards: vec![flat_entry("decision-log", 49_152)],
        };
        let result = shard_cap_gate_check(
            &registry,
            "Write",
            &looped,
            &serde_json::json!({"content": "x".repeat(5_000)}),
        );
        assert_eq!(
            result,
            HookResult::Continue,
            "F-002: a Write's projected_size = len(content) alone (Postcondition 3 CORRECTED) — \
             current_shard_bytes, and any stat() failure reading it (here ELOOP on a \
             self-referential symlink), is irrelevant to the Write formula and MUST NOT block it. \
             shard_cap_gate_check's ToolKind::Write arm never calls current_shard_bytes_flat() at \
             all — that stat() call is pushed down into ONLY the Edit/MultiEdit arms (F-002 fix), \
             so this non-NotFound stat error on a Write's own target path is never observed."
        );
    }

    // ===================================================================
    // Postcondition 9 / EC-013 (BC-1.18.005 v1.7, S-25.02 Phase F4 LOCAL
    // adversary pass-1 cluster-1 finding) — load-time fail-loud enforcement
    // of shard_cap_bytes <= compute_shard_cap_bytes(entry.cap_formula_inputs()).
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_PC9_EC013_load_rejects_cap_greater_than_formula_ceiling() {
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
             shard_cap_bytes = 100000\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");

        // compute_shard_cap_bytes(these four inputs) = 50,640 (BC-1.18.005
        // Postcondition 6's own worked example — see
        // test_BC_1_18_005_PC6_compute_shard_cap_bytes_bc_provisional_worked_example
        // above) — 100,000 is far above its own formula-derived ceiling.
        //
        // TIGHTENED (S-25.02 Phase F4 LOCAL adversary cluster-1 pass-3
        // finding F-C1-P3-004, OBSERVATION): matches the specific
        // ShardConfigError::CapExceedsFormulaCeiling variant and its three
        // fields, aligning with the sibling EC-009/EC-011 unit tests
        // (test_BC_1_18_005_EC_009_load_missing_shape_field_is_fail_loud,
        // test_BC_1_18_005_EC_011_load_rejects_low_water_mark_equal_to_n)
        // that already match their own specific variant rather than a
        // generic is_err().
        let err = ShardRegistry::load(&cfg_path).expect_err(
            "PC9/EC-013: an entry declaring shard_cap_bytes (100,000) GREATER than its own \
             compute_shard_cap_bytes(inputs) ceiling (50,640) MUST fail-loud at load() time",
        );
        match err {
            ShardConfigError::CapExceedsFormulaCeiling {
                artifact_stem,
                shard_cap_bytes,
                computed_ceiling,
            } => {
                assert_eq!(artifact_stem, "decision-log");
                assert_eq!(shard_cap_bytes, 100_000);
                assert_eq!(computed_ceiling, 50_640);
            }
            other => panic!("expected CapExceedsFormulaCeiling, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_PC9_load_accepts_cap_exactly_equal_to_formula_ceiling() {
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
             shard_cap_bytes = 50640\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");

        // compute_shard_cap_bytes(these four inputs) = 50,640 exactly.
        // Postcondition 4's `<=` comparison is inclusive of the boundary
        // (Postcondition 9's own text: "mirroring EC-002's inclusive-boundary
        // precedent"), so an exactly-equal declared cap MUST load
        // successfully, never fail-loud.
        let result = ShardRegistry::load(&cfg_path);
        assert!(
            result.is_ok(),
            "PC9 boundary: shard_cap_bytes EXACTLY EQUAL to compute_shard_cap_bytes(inputs) MUST \
             load successfully (inclusive <=), mirroring EC-002's boundary precedent: {result:?}"
        );
    }

    // ===================================================================
    // EC-015 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-4
    // finding F-C1-P4-001, MEDIUM/HIGH, BC-1.18.005 v1.10) — "divisor-door"
    // closure. `compute_shard_cap_bytes` divides `practical_fuel_ceiling` by
    // `worst_case_fuel_per_byte`; the latter is `f64`-typed and therefore
    // NOT type-guaranteed finite or positive (TOML's float grammar accepts
    // `nan`/`inf`/`-inf`/`0.0`/negative literals). `ShardRegistry::load()`
    // MUST validate `worst_case_fuel_per_byte.is_finite() &&
    // worst_case_fuel_per_byte > 0.0` for every entry BEFORE computing
    // `compute_shard_cap_bytes(entry.cap_formula_inputs())`, fail-loud on
    // violation — otherwise a degenerate divisor lets an arbitrarily-large
    // `shard_cap_bytes` sail past Postcondition 9's cap-vs-formula
    // comparison entirely, reintroducing the exact fuel-exhaustion failure
    // mode that comparison exists to prevent.
    //
    // `ShardRegistry::load` validates `worst_case_fuel_per_byte.is_finite()
    // && worst_case_fuel_per_byte > 0.0` for every entry BEFORE
    // `compute_shard_cap_bytes` is ever called for it, returning
    // `Err(ShardConfigError::InvalidWorstCaseFuelPerByte { artifact_stem,
    // worst_case_fuel_per_byte })` on violation. The three tests below pin
    // the two rejection cases (zero and NaN) plus the valid-input control.
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_EC_015_load_rejects_zero_worst_case_fuel_per_byte_divisor_door() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"zero-divisor-log\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 0.0\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 100000000\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");

        // Without the load-time guard, worst_case_fuel_per_byte = 0.0 would
        // make compute_shard_cap_bytes's internal division yield +inf, whose
        // saturating `as u64` cast is u64::MAX — the resulting ceiling would
        // comfortably exceed this fixture's already-enormous
        // shard_cap_bytes = 100,000,000, letting it sail past Postcondition
        // 9's `shard_cap_bytes > computed_ceiling` check via the degenerate
        // divisor. The guard rejects the entry BEFORE that comparison is
        // ever attempted.
        let err = ShardRegistry::load(&cfg_path).expect_err(
            "EC-015: worst_case_fuel_per_byte = 0.0 (the divisor-door case) MUST fail-loud at \
             load() time, BEFORE the cap-vs-formula comparison is even attempted — an \
             arbitrarily-large shard_cap_bytes must never sail through via a degenerate divisor.",
        );
        match err {
            ShardConfigError::InvalidWorstCaseFuelPerByte {
                artifact_stem,
                worst_case_fuel_per_byte,
            } => {
                assert_eq!(artifact_stem, "zero-divisor-log");
                assert_eq!(worst_case_fuel_per_byte, 0.0);
            }
            other => panic!("expected InvalidWorstCaseFuelPerByte, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_EC_015_load_rejects_nan_worst_case_fuel_per_byte() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"nan-divisor-log\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = nan\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 0\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");

        // Without the load-time guard, worst_case_fuel_per_byte = NaN would
        // make the internal division yield NaN, whose saturating `as u64`
        // cast is 0 (Rust maps a NaN float-to-int cast to 0) — the computed
        // ceiling would then also saturate at 0
        // (0.saturating_sub(16_384).saturating_sub(8_192) = 0), which is NOT
        // LESS than this fixture's own shard_cap_bytes = 0, so Postcondition
        // 9's `shard_cap_bytes > computed_ceiling` check (0 > 0 = false)
        // would never fire either. The guard rejects the entry regardless of
        // which direction the resulting degenerate arithmetic would have
        // saturated — the config is malformed either way and must never be
        // silently accepted.
        let err = ShardRegistry::load(&cfg_path).expect_err(
            "EC-015: worst_case_fuel_per_byte = NaN MUST fail-loud at load() time regardless of \
             which direction the resulting degenerate arithmetic happens to saturate.",
        );
        match err {
            ShardConfigError::InvalidWorstCaseFuelPerByte {
                artifact_stem,
                worst_case_fuel_per_byte,
            } => {
                assert_eq!(artifact_stem, "nan-divisor-log");
                assert!(worst_case_fuel_per_byte.is_nan());
            }
            other => panic!("expected InvalidWorstCaseFuelPerByte, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_EC_015_load_accepts_valid_positive_worst_case_fuel_per_byte_control() {
        // Control: a well-formed, finite, strictly-positive
        // worst_case_fuel_per_byte with shard_cap_bytes AT (not over) its own
        // formula ceiling MUST continue to load successfully once EC-015's
        // guard is added — this test stays GREEN both before and after the
        // fix, pinning that the new finiteness/positivity check does not
        // regress the happy path.
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

        let result = ShardRegistry::load(&cfg_path);
        assert!(
            result.is_ok(),
            "control: a valid finite positive worst_case_fuel_per_byte with shard_cap_bytes <= \
             its own formula ceiling MUST load successfully. Got: {result:?}"
        );
    }

    // ===================================================================
    // EC-017 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-5
    // finding F-C1-P5, MEDIUM/HIGH, BC-1.18.005 v1.11) — residual
    // divisor-door closure: a saturated COMPUTED CEILING, not merely an
    // illegal divisor. EC-015's `worst_case_fuel_per_byte.is_finite() &&
    // > 0.0` guard is necessary but insufficient — a legal tiny-positive
    // divisor (e.g. `1e-300`) satisfies that guard yet still drives
    // `compute_shard_cap_bytes`'s internal division
    // (`practical_fuel_ceiling as f64 / worst_case_fuel_per_byte`) to a
    // value so large that its `.floor() as u64` cast saturates to
    // `u64::MAX`, reproducing the identical failure mode EC-015's guard
    // exists to close: an oversized `shard_cap_bytes` sails past the
    // `CapExceedsFormulaCeiling` comparison because the computed ceiling
    // itself is meaninglessly large.
    //
    // `ShardRegistry::load` MUST, for every entry that passes the EC-015
    // guard and BEFORE the `CapExceedsFormulaCeiling` comparison is
    // evaluated, independently evaluate the RAW, pre-cast division result
    // `practical_fuel_ceiling as f64 / worst_case_fuel_per_byte` and reject
    // the entry if that raw f64 value is non-finite or `>= u64::MAX as
    // f64` — a NEW `ShardConfigError::FormulaCeilingSaturated` variant,
    // distinct from `InvalidWorstCaseFuelPerByte` (which catches an
    // illegal `<= 0.0`/non-finite DIVISOR itself, not a saturated RESULT
    // arising from an otherwise-legal tiny-positive divisor).
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_EC_017_load_rejects_saturating_formula_ceiling() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"saturating-ceiling-log\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 1e-300\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 100000000\n\
             shape = \"flat\"\n",
        )
        .expect("write fixture");

        // worst_case_fuel_per_byte = 1e-300 is finite and > 0.0, so it
        // PASSES the EC-015 guard. But the raw division
        // 8,000,000 / 1e-300 = ~8e306, which is >= u64::MAX as f64 — the
        // subsequent `.floor() as u64` cast in `compute_shard_cap_bytes`
        // saturates to u64::MAX, so the resulting ceiling would
        // comfortably exceed this fixture's already-enormous
        // shard_cap_bytes = 100,000,000, letting it sail past
        // Postcondition 9's `shard_cap_bytes > computed_ceiling` check via
        // a legal-but-degenerate divisor. The residual closure rejects the
        // entry BEFORE that comparison is ever attempted.
        let result = ShardRegistry::load(&cfg_path);
        assert!(
            result.is_err(),
            "EC-017: worst_case_fuel_per_byte = 1e-300 is legal under EC-015's is_finite() && \
             > 0.0 guard, yet the raw division practical_fuel_ceiling as f64 / \
             worst_case_fuel_per_byte saturates the subsequent .floor() as u64 cast to \
             u64::MAX — ShardRegistry::load() MUST reject this entry (residual divisor-door \
             closure) BEFORE the CapExceedsFormulaCeiling comparison is even attempted, \
             regardless of the arbitrarily-large declared shard_cap_bytes. Got: {result:?}"
        );
    }

    // ===================================================================
    // EC-016 (NEW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-4
    // finding F-C1-P4-002, MEDIUM, BC-1.18.005 v1.10) — a
    // "frontmatter-changelog-array"-shaped [[shard]] entry that OMITS the
    // required `n` item-count trigger threshold MUST fail-loud at
    // ShardRegistry::load() time, not merely deferred to the first
    // gate-time write (shard_cap_gate_check's existing
    // HookResult::Error-on-None-`n` check runs too late relative to this
    // BC's own EC-009/EC-011/EC-013 load-time posture).
    //
    // `ShardRegistry::load` requires `n` for every
    // `"frontmatter-changelog-array"`-shaped entry via a
    // `let Some(n) = entry.n else { return Err(ShardConfigError::MissingN {
    // artifact_stem }) }` guard, evaluated BEFORE `low_water_mark` is
    // examined — so an entry missing `n` AND declaring an independently
    // invalid `low_water_mark` is rejected on the missing-`n` condition, not
    // silently accepted. The two tests below pin both the plain-omission
    // case and the omitted-`n`-plus-invalid-`low_water_mark` case.
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_EC_016_load_rejects_frontmatter_shape_missing_n() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"BC-INDEX\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 106.36\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 49152\n\
             shape = \"frontmatter-changelog-array\"\n",
        )
        .expect("write fixture");

        // entry.n is None here (this fixture never declares `n`), so
        // load()'s `let Some(n) = entry.n else { ... }` guard fires and
        // rejects the entry at load time — never deferred to the first
        // gate-time write inside shard_cap_gate_check.
        let err = ShardRegistry::load(&cfg_path).expect_err(
            "EC-016: a \"frontmatter-changelog-array\"-shaped entry that omits the required `n` \
             item-count trigger threshold MUST fail-loud at ShardRegistry::load() time, mirroring \
             EC-009/EC-011/EC-013's uniform load-time posture for this same config surface.",
        );
        match err {
            ShardConfigError::MissingN { artifact_stem } => {
                assert_eq!(artifact_stem, "BC-INDEX");
            }
            other => panic!("expected MissingN, got {other:?}"),
        }
    }

    #[test]
    fn test_BC_1_18_005_EC_016_load_rejects_missing_n_even_with_invalid_low_water_mark() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("shard-config.toml");
        std::fs::write(
            &cfg_path,
            "[[shard]]\n\
             artifact_stem = \"BC-INDEX\"\n\
             practical_fuel_ceiling = 8000000\n\
             worst_case_fuel_per_byte = 106.36\n\
             max_single_record_bytes = 16384\n\
             safety_margin = 8192\n\
             shard_cap_bytes = 49152\n\
             shape = \"frontmatter-changelog-array\"\n\
             low_water_mark = -1\n",
        )
        .expect("write fixture");

        // entry.n is still None here (this fixture never declares `n`), so
        // load()'s `let Some(n) = entry.n else { ... }` guard fires and
        // reports the missing-`n` condition BEFORE EC-011's
        // negative-low_water_mark check is ever reached for this entry —
        // per Postcondition 8's ordering requirement, MissingN is the
        // reported error even though low_water_mark = -1 is independently
        // invalid against any realistic N.
        let err = ShardRegistry::load(&cfg_path).expect_err(
            "EC-016: an entry missing `n` AND declaring an independently-invalid low_water_mark \
             (-1) MUST still be rejected at load() time — with EC-016's guard ordered before the \
             low_water_mark check, the reported error is the missing-`n` condition, but either \
             way this entry must never silently load.",
        );
        match err {
            ShardConfigError::MissingN { artifact_stem } => {
                assert_eq!(artifact_stem, "BC-INDEX");
            }
            other => panic!("expected MissingN, got {other:?}"),
        }
    }

    // ===================================================================
    // O-C1-P4-001 (LOW, S-25.02 Phase F4 LOCAL adversary cluster-1 pass-4
    // observation) — symmetry between the two `shard_cap_gate_check` shape
    // arms. The `"flat"` arm guards against a non-mutating `tool_name` via
    // `ToolKind::from_tool_name` BEFORE ever touching the target file
    // (Postcondition 1's zero-cost-bypass spirit, extended to an
    // unsupported tool kind). The `FrontmatterChangelogArray` arm mirrors
    // that same guard: it early-returns `Continue` for a
    // non-Edit/Write/MultiEdit `tool_name` BEFORE reading `entry.n` or
    // calling `read_changelog_item_count(target_path)`.
    //
    // This test drives that symmetry into an observable difference: the
    // target file's content is deliberately malformed (no well-formed `---`
    // frontmatter fence), so `read_changelog_item_count` would fail loud
    // with an `io::Error` if the FrontmatterChangelogArray arm ever reached
    // it for a non-mutating call. The guarded arm returns `Continue` for the
    // non-mutating `"Read"` tool_name WITHOUT ever reading the file, so the
    // malformed content is never observed.
    // ===================================================================

    #[test]
    fn test_BC_1_18_005_P4_frontmatter_arm_non_mutating_tool_continues() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("BC-INDEX.md");
        std::fs::write(&target, "no frontmatter fence at all\n").expect("write fixture");

        let mut entry = flat_entry("BC-INDEX", 49_152);
        entry.shape = Some(ShardShape::FrontmatterChangelogArray);
        entry.n = Some(50);
        let registry = ShardRegistry {
            shards: vec![entry],
        };

        // The FrontmatterChangelogArray arm's tool_name guard early-returns
        // Continue for the non-mutating "Read" tool_name WITHOUT ever
        // calling read_changelog_item_count(target_path), so the fixture's
        // deliberately malformed (fence-less) frontmatter is never observed.
        let result = shard_cap_gate_check(&registry, "Read", &target, &serde_json::json!({}));

        assert_eq!(
            result,
            HookResult::Continue,
            "O-C1-P4-001: a non-mutating tool_name (\"Read\") reaching the \
             FrontmatterChangelogArray arm MUST Continue WITHOUT reading the target file, \
             mirroring the \"flat\" arm's ToolKind::from_tool_name guard — got {result:?} instead \
             (the ungated arm proceeded to read_changelog_item_count and surfaced the fixture's \
             deliberately malformed frontmatter as a fail-loud HookResult::Error)"
        );
    }
}
