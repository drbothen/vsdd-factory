//! Pure path-template interpolation extracted from `sink-file` (S-4.05 Task 0).
//!
//! Supports `{date}`, `{name}`, and `{project}` placeholders.
//! No I/O. No filesystem calls. No side effects.
//!
//! ## Placeholder vocabulary
//!
//! | Placeholder | Replacement |
//! |-------------|-------------|
//! | `{date}`    | `YYYY-MM-DD` derived from `date` parameter (BC-3.02.001) |
//! | `{name}`    | value of the `name` parameter |
//! | `{project}` | value of the `project` parameter if `Some`, else empty string |

use chrono::{DateTime, TimeZone};
use std::path::PathBuf;
use thiserror::Error;

/// Errors returned by [`resolve_path_template`].
#[derive(Debug, Error)]
pub enum PathTemplateError {
    /// The template string contains a placeholder that is not in the
    /// supported vocabulary (`{date}`, `{name}`, `{project}`).
    #[error("unknown placeholder '{placeholder}' in path template")]
    UnknownPlaceholder {
        /// The unrecognised placeholder text (without the braces).
        placeholder: String,
    },
}

/// Resolve `{date}`, `{name}`, and `{project}` placeholders in `template`,
/// returning a [`PathBuf`].
///
/// The `date` parameter supplies the calendar day used for `{date}`.
/// `name` is typically the operator-assigned sink name.
/// `project` is the basename of `CLAUDE_PROJECT_DIR` (or `None`).
///
/// # Errors
///
/// Returns [`PathTemplateError::UnknownPlaceholder`] when the template
/// contains a `{…}` token that is not one of the supported three.
pub fn resolve_path_template<Tz: TimeZone>(
    template: &str,
    date: DateTime<Tz>,
    name: &str,
    project: Option<&str>,
) -> Result<PathBuf, PathTemplateError>
where
    Tz::Offset: std::fmt::Display,
{
    // Stub — not implemented. RED-gate: tests must fail here.
    let _ = (template, date, name, project);
    unimplemented!("resolve_path_template stub — implement in GREEN phase")
}
