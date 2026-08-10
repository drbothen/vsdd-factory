//! POLICY 15 ATTESTATION-LOCATION GATE — binary entry point.
//!
//! Invocation (intended for CI, post human re-ratification of ADR-040 v1.8):
//!
//! ```text
//! policy15-attestation-gate [BASE_BRANCH]
//! ```
//!
//! Defaults to `develop`; the `BASE_BRANCH` environment variable also sets the default.
//! Prints the outcome identifier to stdout (greppable in CI logs) and exits with
//! the appropriate process code:
//!
//! | Outcome | Exit code |
//! |---------|-----------|
//! | `PASS-N-activations` | 0 |
//! | `PASS-zero-activations` | 0 |
//! | `FAIL: obligation violated` | 2 |
//! | `EMPTY-or-UNREACHABLE: *` | 2 |
//! | Hard error (git not found, I/O) | 1 |
//!
//! GitHub Actions job name: `policy-15-attestation-location`
//! (see ADR-040 §Decision 7 Ruling 7(a), pending devops-engineer wiring after
//! human re-ratification of ADR-040 v1.8).

use clap::Parser;
use policy15_attestation_gate::{
    FailReason, GateOutcome, PLUGIN_CRATE, UnreachableCause, run_gate,
};

#[derive(Parser)]
#[command(name = "policy15-attestation-gate")]
#[command(version)]
#[command(about = "POLICY 15 ATTESTATION-LOCATION GATE (ADR-040 §Decisions 8+9)")]
#[command(
    long_about = "Evaluates the per-commit POLICY 15 obligation for the pinned crate.\n\
    Prints a greppable outcome identifier and exits with code 0 (PASS) or 2 (FAIL/EMPTY)."
)]
struct Cli {
    /// Base branch name; merge base is computed as merge-base(HEAD, origin/<base_branch>).
    /// Defaults to `develop`; also honoured via the `BASE_BRANCH` environment variable.
    #[arg(default_value = "develop")]
    base_branch: String,
}

fn main() {
    let cli = Cli::parse();
    // Honour BASE_BRANCH env var (mirrors the ADR-040 §Decision 9 bash convention).
    let base_branch = std::env::var("BASE_BRANCH").unwrap_or(cli.base_branch);

    let repo = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("GATE ERROR: cannot determine current directory: {e}");
            std::process::exit(1);
        }
    };

    match run_gate(&repo, &base_branch) {
        Ok(outcome) => {
            // Print detail lines to stderr, identifier to stdout.
            match &outcome {
                GateOutcome::Fail(failed) => {
                    for fc in failed {
                        let reason = match &fc.reason {
                            FailReason::LogAbsent => "log absent at commit".to_string(),
                            FailReason::AttestationMissing => {
                                "attestation heading missing in log".to_string()
                            }
                        };
                        let short = if fc.commit.len() >= 12 {
                            &fc.commit[..12]
                        } else {
                            &fc.commit
                        };
                        eprintln!("  FAIL {short}: {reason}");
                    }
                }
                GateOutcome::EmptyOrUnreachable(cause) => match cause {
                    UnreachableCause::StalePin => {
                        eprintln!("  stale pin: \"{PLUGIN_CRATE}\" absent from HEAD git tree")
                    }
                    UnreachableCause::EmptyRange => {
                        eprintln!(
                            "  empty range: no commits between merge-base and HEAD, \
                            or base branch unresolvable"
                        );
                    }
                    UnreachableCause::UnmeasurableDiff { commit } => {
                        let short = if commit.len() >= 12 {
                            &commit[..12]
                        } else {
                            commit
                        };
                        eprintln!("  unmeasurable diff at commit {short}");
                    }
                },
                _ => {}
            }
            println!("{}", outcome.identifier());
            std::process::exit(outcome.exit_code());
        }
        Err(e) => {
            eprintln!("GATE ERROR: {e}");
            std::process::exit(1);
        }
    }
}
