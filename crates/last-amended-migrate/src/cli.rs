//! Command-line surface for `last-amended-migrate` (BC-10.13.001
//! §Description).
//!
//! Two subcommands: `migrate` (PC1-PC4, PC6) and `rotate` (PC5, PC6). Both
//! support `--check` (report violations without writing, mirroring
//! `compute-input-hash`'s `--check` convention) and default to apply mode
//! when `--check` is absent.

use crate::rotate::DEFAULT_KEEP_RECENT;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

/// `last-amended-migrate` top-level CLI (POLICY 21-compliant Rust tool;
/// BC-10.13.001).
#[derive(Parser, Debug)]
#[command(name = "last-amended-migrate")]
#[command(version)]
#[command(
    about = "Sanctioned Rust CLI for one-time last_amended/changelog: migration \
    plus changelog rotation (BC-10.13.001, POLICY 21-compliant)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Subcommands mapping 1:1 to BC-10.13.001's two functions (§Description).
#[derive(Subcommand, Debug)]
pub enum Command {
    /// One-time migration of one file, or all 5 ADR-049-governed files,
    /// into the ADR-049 shape (PC1-PC4, PC6).
    Migrate {
        /// Target file path. Omit to migrate all 5 D-1149 files under
        /// `--factory-root`.
        #[arg(long)]
        path: Option<PathBuf>,
        /// Root of the `.factory/` tree; used only when `--path` is
        /// omitted.
        #[arg(long, default_value = ".factory")]
        factory_root: PathBuf,
        /// Report violations without writing (mirrors `compute-input-hash`
        /// `--check`).
        #[arg(long)]
        check: bool,
    },
    /// Rotate an over-long `changelog:` sequence into a per-cycle archive
    /// (PC5, PC6).
    Rotate {
        /// Target file path.
        #[arg(long)]
        path: PathBuf,
        /// Destination cycle name under `.factory/cycles/<cycle_name>/`.
        #[arg(long)]
        cycle_name: String,
        /// Number of most-recent items to retain in the source file.
        #[arg(long, default_value_t = DEFAULT_KEEP_RECENT)]
        keep_recent: usize,
        /// Report violations without writing.
        #[arg(long)]
        check: bool,
    },
}

/// Dispatch a parsed CLI invocation to the corresponding subcommand handler
/// (BC-10.13.001 §Description) — routes to `migrate_file`/`migrate_all`
/// (PC1-PC4, PC6) or `rotate_changelog` (PC5, PC6) depending on which
/// subcommand was selected, prints the resulting report, and maps any
/// `Err` to a non-zero `ExitCode`.
///
/// `MigrationMode::Check`/`MigrationMode::Apply` are derived here from each
/// subcommand's `check: bool` flag.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: branches on `cli.command` and on `Result` success/failure,
/// and performs I/O (stdout/stderr printing) — every dispatch/print/exit-
/// code decision this function makes is real behavior a test can observe.
/// Body is `todo!()`. This is the ONLY branching point in the CLI surface —
/// `main()` itself is a single-line delegate to `run(Cli::parse())`, which
/// is WIRING-EXEMPT (see the stub commit report's WIRING-EXEMPT table).
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; correct subcommand
/// routing, report formatting, and exit-code mapping are exactly what a CLI
/// integration test would probe. Therefore: `todo!()`.
pub fn run(cli: Cli) -> ExitCode {
    todo!(
        "dispatch {cli:?} to migrate_file/migrate_all or rotate_changelog, \
        print the resulting report, and map Err to ExitCode::FAILURE \
        (BC-10.13.001 subcommand routing)"
    )
}
