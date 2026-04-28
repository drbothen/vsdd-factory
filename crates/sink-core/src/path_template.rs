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
    use std::path::Path;

    let date_s = date.format("%Y-%m-%d").to_string();
    let project_s = project
        .and_then(|p| {
            Path::new(p)
                .file_name()
                .and_then(|s| s.to_str())
                .map(str::to_owned)
        })
        .unwrap_or_default();

    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else {
            // Unbalanced brace — treat the rest literally.
            out.push('{');
            out.push_str(after);
            rest = "";
            break;
        };
        let key = &after[..close];
        match key {
            "date" => out.push_str(&date_s),
            "name" => out.push_str(name),
            "project" => out.push_str(&project_s),
            other => {
                return Err(PathTemplateError::UnknownPlaceholder {
                    placeholder: other.to_owned(),
                });
            }
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    Ok(PathBuf::from(out))
}
