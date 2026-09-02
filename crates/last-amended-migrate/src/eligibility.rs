//! Eligibility classification for BC-10.13.001 Precondition 2 / EC-003.

/// Whether a file's `last_amended` value is eligible for this tool's
/// migration subcommand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    /// Already a single dated entry with no nested `[Prior: ...]` chain —
    /// migration for this field is a verified no-op (PC2, PC4).
    CurrentEntryOnly,
    /// Still contains a nested `[Prior: <date> (vX.Y) — ...]` bracket
    /// referencing a different dated entry. Out of scope for this tool
    /// (Precondition 2) — the tool must report NOT ELIGIBLE (EC-003) and
    /// never attempt to split it.
    NotEligiblePriorChain,
}

/// Classify `last_amended_raw` per BC-10.13.001 Precondition 2 / EC-003.
///
/// MUST NOT perform or begin any bracket-splitting surgery — this function
/// only classifies; splitting a detected chain is explicitly out of scope
/// (Precondition 2, EC-003) and remains a human-authorized POL-3 exception if
/// it recurs elsewhere.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: requires scanning for a `[Prior: ` marker while tolerating
/// arbitrarily long input (Invariant 3) without misclassifying a literal `[`
/// that is not the prior-chain marker. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; correctly
/// distinguishing the two states, including on the 323,499-char calibration
/// fixture, requires real scanning logic. Therefore: `todo!()`.
pub fn check_eligibility(last_amended_raw: &str) -> Eligibility {
    todo!(
        "classify {} bytes of last_amended as CurrentEntryOnly vs \
        NotEligiblePriorChain by scanning for a `[Prior: ` marker without \
        backtracking (BC-10.13.001 PC2 / EC-003)",
        last_amended_raw.len()
    )
}
