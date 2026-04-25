//! Always-on dispatcher self-telemetry log (S-1.7).
//!
//! Every dispatcher lifecycle event and every `internal.*` event the
//! dispatcher emits lands in a daily-rotated JSONL file at
//! `<log_dir>/dispatcher-internal-YYYY-MM-DD.jsonl`. This path exists
//! independent of `observability-config.toml` (Q6 Option B) so the
//! dispatcher remains debuggable even when all configured sinks are
//! down or misconfigured.
//!
//! Contract:
//! - Writes are best-effort. All I/O errors are swallowed; `write` never
//!   panics, never propagates.
//! - Files are daily-rotated by the timestamp on the event itself (not
//!   "now") so tests can write events dated in the past/future without
//!   reaching for a mocked clock.
//! - Retention is 30 days by default; `prune_old` walks the log dir and
//!   unlinks matching files older than the threshold. Also best-effort.
//! - Appends rely on `OpenOptions::append`; atomicity at PIPE_BUF size is
//!   OS-guaranteed. Lines larger than PIPE_BUF (4096 on Linux, 512 on
//!   macOS) may interleave across concurrent writers — acceptable per
//!   the story spec, and v1.0 runs one dispatcher process per hook
//!   invocation anyway.
//!
//! Plugin-lifecycle events (`plugin.loaded`, `plugin.invoked`, …) are
//! defined here as constants but the callsites land in S-1.5; sink-error
//! events land in S-1.8. This story ships structural plumbing +
//! `dispatcher.started` + `internal.dispatcher_error` only.

use chrono::{DateTime, Duration, Local, TimeZone};
use serde::Serialize;
use serde_json::{Map, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Filename prefix for every rotated log. Matched during retention
/// scans.
const FILENAME_PREFIX: &str = "dispatcher-internal-";
/// Filename suffix for every rotated log.
const FILENAME_SUFFIX: &str = ".jsonl";
/// Default retention window in days. Callers should pass this (or
/// override) to [`InternalLog::prune_old`] at dispatcher startup.
pub const DEFAULT_RETENTION_DAYS: u32 = 30;

/// `schema_version` embedded in every event. Bumped when the event
/// shape changes in a non-backwards-compatible way.
pub const INTERNAL_EVENT_SCHEMA_VERSION: u32 = 1;

// --- Event type names ------------------------------------------------
//
// These are `pub const` strings rather than an enum so callers can
// attach arbitrary `fields` without ceremony, and downstream log
// pipelines (jq / OpenTelemetry attribute mapping) can filter by a
// stable string tag. Keep in lock-step with the catalog in the story
// spec.

pub const DISPATCHER_STARTED: &str = "dispatcher.started";
pub const DISPATCHER_SHUTTING_DOWN: &str = "dispatcher.shutting_down";
pub const PLUGIN_LOADED: &str = "plugin.loaded";
pub const PLUGIN_LOAD_FAILED: &str = "plugin.load_failed";
pub const PLUGIN_INVOKED: &str = "plugin.invoked";
pub const PLUGIN_COMPLETED: &str = "plugin.completed";
pub const PLUGIN_TIMEOUT: &str = "plugin.timeout";
pub const PLUGIN_CRASHED: &str = "plugin.crashed";
pub const INTERNAL_CAPABILITY_DENIED: &str = "internal.capability_denied";
pub const INTERNAL_HOST_FUNCTION_PANIC: &str = "internal.host_function_panic";
pub const INTERNAL_SINK_ERROR: &str = "internal.sink_error";
pub const INTERNAL_SINK_QUEUE_FULL: &str = "internal.sink_queue_full";
pub const INTERNAL_SINK_CIRCUIT_OPENED: &str = "internal.sink_circuit_opened";
pub const INTERNAL_SINK_CIRCUIT_CLOSED: &str = "internal.sink_circuit_closed";
pub const INTERNAL_DISPATCHER_ERROR: &str = "internal.dispatcher_error";

/// One line in `dispatcher-internal-YYYY-MM-DD.jsonl`.
///
/// The top-level fields form the stable shape every log line carries;
/// event-specific extras go inside `fields`, flattened via
/// `#[serde(flatten)]` so the final JSON shape is flat (not `{ fields:
/// {...} }`).
#[derive(Debug, Clone, Serialize)]
pub struct InternalEvent {
    /// Event type — one of the `pub const` names above, e.g.
    /// `"dispatcher.started"`.
    #[serde(rename = "type")]
    pub type_: String,
    /// ISO-8601 timestamp with offset, e.g. `"2026-04-24T20:13:45-0500"`.
    pub ts: String,
    /// Epoch seconds corresponding to `ts`.
    pub ts_epoch: i64,
    /// Event-schema version; bump when the shape changes.
    pub schema_version: u32,
    /// Dispatcher's per-invocation trace id (v4 UUID), when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatcher_trace_id: Option<String>,
    /// Claude Code session id, when the event carries payload context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Plugin name for plugin-scoped events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    /// Plugin version for plugin-scoped events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_version: Option<String>,
    /// Event-specific extras. Flattened into the top-level JSON object.
    #[serde(flatten)]
    pub fields: Map<String, Value>,
}

impl InternalEvent {
    /// Construct a new event with the current local time. Optional
    /// fields default to `None` and `fields` default to empty; both are
    /// chainable via the builder-ish setters below.
    pub fn now(type_: impl Into<String>) -> Self {
        let now = Local::now();
        Self::with_ts(type_, now)
    }

    /// Construct an event at an explicit time. Used by tests that need
    /// to pin a particular date-of-rotation without reaching for a
    /// clock mock.
    pub fn with_ts<Tz: TimeZone>(type_: impl Into<String>, ts: DateTime<Tz>) -> Self
    where
        Tz::Offset: std::fmt::Display,
    {
        // `%z` emits `-0500`; matches the example in the story spec.
        let ts_str = ts.format("%Y-%m-%dT%H:%M:%S%z").to_string();
        let ts_epoch = ts.timestamp();
        Self {
            type_: type_.into(),
            ts: ts_str,
            ts_epoch,
            schema_version: INTERNAL_EVENT_SCHEMA_VERSION,
            dispatcher_trace_id: None,
            session_id: None,
            plugin_name: None,
            plugin_version: None,
            fields: Map::new(),
        }
    }

    /// Builder-style setter for the trace id.
    #[must_use]
    pub fn with_trace_id(mut self, id: impl Into<String>) -> Self {
        self.dispatcher_trace_id = Some(id.into());
        self
    }

    /// Builder-style setter for the Claude Code session id.
    #[must_use]
    pub fn with_session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = Some(id.into());
        self
    }

    /// Builder-style setter for the plugin name.
    #[must_use]
    pub fn with_plugin_name(mut self, name: impl Into<String>) -> Self {
        self.plugin_name = Some(name.into());
        self
    }

    /// Builder-style setter for the plugin version.
    #[must_use]
    pub fn with_plugin_version(mut self, version: impl Into<String>) -> Self {
        self.plugin_version = Some(version.into());
        self
    }

    /// Attach one extra field. Later calls overwrite earlier ones on
    /// key collision — the last-writer-wins semantics match the
    /// `serde_json::Map` contract.
    #[must_use]
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Derive the date stamp used in the rotated filename. Uses the
    /// first 10 chars of `ts` (`YYYY-MM-DD`) so rotation matches the
    /// event's timestamp exactly — no "now" lookup inside `write`.
    fn date_stamp(&self) -> &str {
        // `ts` format is `YYYY-MM-DDThh:mm:ss±zzzz`; take the first 10
        // bytes. Safe for ASCII-only timestamps produced by `with_ts`.
        &self.ts[..10.min(self.ts.len())]
    }
}

/// Best-effort JSONL writer for dispatcher self-telemetry.
///
/// Cheap to construct and `Clone`/share across threads — it holds only
/// a `PathBuf`; each write reopens the file. The reopen cost is
/// negligible compared to the outer dispatcher latency and it keeps the
/// writer trivially `Send + Sync` without any locking.
#[derive(Debug, Clone)]
pub struct InternalLog {
    log_dir: PathBuf,
}

impl InternalLog {
    /// Build a writer rooted at `log_dir`. The directory is NOT created
    /// eagerly; `write` will `mkdir -p` on first use.
    pub fn new(log_dir: PathBuf) -> Self {
        Self { log_dir }
    }

    /// Best-effort append. Never panics, never propagates errors. On
    /// failure, emits one diagnostic line to stderr and returns.
    pub fn write(&self, event: &InternalEvent) {
        if let Err(e) = self.write_inner(event) {
            // Keep the fallback deliberately short — if even stderr is
            // broken we have bigger problems, and we do not want the
            // dispatcher to hang on a blocked tty.
            eprintln!(
                "factory-dispatcher: internal_log write failed ({}): {}",
                event.type_, e
            );
        }
    }

    fn write_inner(&self, event: &InternalEvent) -> std::io::Result<()> {
        fs::create_dir_all(&self.log_dir)?;

        let filename = format!(
            "{FILENAME_PREFIX}{date}{FILENAME_SUFFIX}",
            date = event.date_stamp()
        );
        let path = self.log_dir.join(filename);

        // Serialize first so a JSON error (should be impossible given
        // the typed shape, but defense-in-depth) does not leave a
        // half-open file descriptor.
        let mut line = serde_json::to_string(event)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        line.push('\n');

        let mut f = OpenOptions::new().append(true).create(true).open(&path)?;
        // Single write_all so the stdlib retries short writes; full
        // atomicity up to PIPE_BUF is OS-guaranteed.
        f.write_all(line.as_bytes())?;
        Ok(())
    }

    /// Delete rotated logs whose mtime is older than `max_age_days`.
    /// Best-effort: individual failures are skipped, directory-missing
    /// is not an error.
    pub fn prune_old(&self, max_age_days: u32) {
        if let Err(e) = self.prune_old_inner(max_age_days) {
            eprintln!("factory-dispatcher: internal_log prune failed: {e}");
        }
    }

    fn prune_old_inner(&self, max_age_days: u32) -> std::io::Result<()> {
        let dir = match fs::read_dir(&self.log_dir) {
            Ok(d) => d,
            // Missing log dir on a fresh install is expected; nothing to
            // prune.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
        };

        let cutoff = Local::now() - Duration::days(max_age_days as i64);
        let cutoff_epoch = cutoff.timestamp();

        for entry in dir.flatten() {
            let path = entry.path();
            // Only touch files matching the rotated naming pattern so a
            // mis-configured log dir (e.g. pointed at /tmp) cannot
            // unlink unrelated files.
            let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if !name.starts_with(FILENAME_PREFIX) || !name.ends_with(FILENAME_SUFFIX) {
                continue;
            }

            // `metadata()` + `modified()` is the portable mtime call.
            // If either fails (e.g. race with concurrent deletion), skip
            // this file rather than abort the sweep.
            let Ok(meta) = entry.metadata() else { continue };
            let Ok(modified) = meta.modified() else {
                continue;
            };
            let Ok(since_epoch) = modified.duration_since(std::time::UNIX_EPOCH) else {
                continue;
            };
            let file_epoch = since_epoch.as_secs() as i64;

            if file_epoch < cutoff_epoch {
                // Best-effort remove; ignore individual errors.
                let _ = fs::remove_file(&path);
            }
        }
        Ok(())
    }

    /// Expose the log directory — integration tests use this to read
    /// files back; dispatcher main may log it for ops.
    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::fs as stdfs;
    use std::io::BufRead;

    fn read_lines(path: &Path) -> Vec<String> {
        let f = stdfs::File::open(path).unwrap();
        std::io::BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    }

    #[test]
    fn writes_jsonl_events_with_expected_shape() {
        let dir = tempfile::tempdir().unwrap();
        let log = InternalLog::new(dir.path().to_path_buf());
        let ts = Local.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap();

        for i in 0..10 {
            let event = InternalEvent::with_ts(DISPATCHER_STARTED, ts)
                .with_trace_id(format!("trace-{i}"))
                .with_field("iteration", i as i64);
            log.write(&event);
        }

        let expected = dir
            .path()
            .join(format!("{FILENAME_PREFIX}2026-04-24{FILENAME_SUFFIX}"));
        assert!(expected.exists(), "expected log file at {expected:?}");

        let lines = read_lines(&expected);
        assert_eq!(lines.len(), 10);

        for (i, line) in lines.iter().enumerate() {
            let parsed: Value = serde_json::from_str(line).unwrap();
            assert_eq!(parsed["type"], DISPATCHER_STARTED);
            assert_eq!(parsed["schema_version"], INTERNAL_EVENT_SCHEMA_VERSION);
            assert_eq!(parsed["dispatcher_trace_id"], format!("trace-{i}"));
            assert_eq!(parsed["iteration"], i as i64);
            assert!(parsed["ts"].as_str().unwrap().starts_with("2026-04-24"));
            assert!(parsed["ts_epoch"].is_i64());
        }
    }

    #[test]
    fn auto_creates_missing_parent_dirs() {
        let dir = tempfile::tempdir().unwrap();
        // Two levels deeper than anything that exists.
        let nested = dir.path().join("a").join("b").join("c");
        let log = InternalLog::new(nested.clone());
        let ts = Local.with_ymd_and_hms(2026, 1, 15, 9, 30, 0).unwrap();

        log.write(&InternalEvent::with_ts(DISPATCHER_STARTED, ts));

        let expected = nested.join(format!("{FILENAME_PREFIX}2026-01-15{FILENAME_SUFFIX}"));
        assert!(expected.exists());
        let lines = read_lines(&expected);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    #[cfg(unix)]
    fn silently_swallows_errors_on_read_only_dir() {
        // Create a dir, chmod 0o555 (read+execute, no write), verify
        // `write` does not panic.
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let ro = dir.path().join("ro");
        stdfs::create_dir_all(&ro).unwrap();
        let mut perms = stdfs::metadata(&ro).unwrap().permissions();
        perms.set_mode(0o555);
        stdfs::set_permissions(&ro, perms.clone()).unwrap();

        let log = InternalLog::new(ro.clone());
        let ts = Local.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap();

        // Must not panic; returns ().
        log.write(&InternalEvent::with_ts(DISPATCHER_STARTED, ts));

        // Restore perms so tempdir cleanup works.
        perms.set_mode(0o755);
        stdfs::set_permissions(&ro, perms).unwrap();
    }

    #[test]
    fn daily_rotation_writes_separate_files_per_date() {
        let dir = tempfile::tempdir().unwrap();
        let log = InternalLog::new(dir.path().to_path_buf());
        let day1 = Local.with_ymd_and_hms(2026, 4, 23, 23, 59, 0).unwrap();
        let day2 = Local.with_ymd_and_hms(2026, 4, 24, 0, 1, 0).unwrap();

        log.write(&InternalEvent::with_ts(DISPATCHER_STARTED, day1));
        log.write(&InternalEvent::with_ts(DISPATCHER_STARTED, day2));
        log.write(&InternalEvent::with_ts(PLUGIN_INVOKED, day2));

        let f1 = dir
            .path()
            .join(format!("{FILENAME_PREFIX}2026-04-23{FILENAME_SUFFIX}"));
        let f2 = dir
            .path()
            .join(format!("{FILENAME_PREFIX}2026-04-24{FILENAME_SUFFIX}"));
        assert!(f1.exists(), "expected {f1:?}");
        assert!(f2.exists(), "expected {f2:?}");
        assert_eq!(read_lines(&f1).len(), 1);
        assert_eq!(read_lines(&f2).len(), 2);
    }

    #[test]
    fn prune_removes_files_older_than_max_age() {
        use filetime::{FileTime, set_file_mtime};

        let dir = tempfile::tempdir().unwrap();
        let log = InternalLog::new(dir.path().to_path_buf());

        let now = std::time::SystemTime::now();
        let now_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;

        // (age_in_days, expected_to_survive_30_day_prune)
        // Avoid the exact 30-day boundary — the test grabs `now_epoch`
        // before calling `prune_old`, but `prune_old_inner` re-evaluates
        // `Local::now()` later, so any sub-second drift between the two
        // crosses a file at exactly the boundary in or out. Windows
        // happened to flake this in CI run 24935133658; Unix never had,
        // but the same race exists. Use 29 / 31 to bracket the boundary
        // without sitting on it.
        let fixtures: &[(i64, bool)] =
            &[(1, true), (10, true), (29, true), (31, false), (60, false)];

        let day_secs: i64 = 86_400;
        let mut paths: Vec<(PathBuf, bool)> = Vec::new();
        for (i, (age, survives)) in fixtures.iter().enumerate() {
            let name = format!(
                "{FILENAME_PREFIX}2026-01-{:02}{FILENAME_SUFFIX}",
                i.saturating_add(1)
            );
            let p = dir.path().join(&name);
            stdfs::write(&p, b"{}\n").unwrap();
            let mtime = FileTime::from_unix_time(now_epoch - age * day_secs, 0);
            set_file_mtime(&p, mtime).unwrap();
            paths.push((p, *survives));
        }

        // Also drop an unrelated file to assert the matcher does not
        // sweep non-internal-log files even if they're ancient.
        let other = dir.path().join("unrelated-2020-01-01.jsonl");
        stdfs::write(&other, b"keep me").unwrap();
        set_file_mtime(
            &other,
            FileTime::from_unix_time(now_epoch - 365 * day_secs, 0),
        )
        .unwrap();

        log.prune_old(30);

        for (p, survives) in &paths {
            assert_eq!(
                p.exists(),
                *survives,
                "file {p:?} expected survives={survives}"
            );
        }
        assert!(other.exists(), "pruner must not touch non-matching names");
    }

    #[test]
    fn prune_is_no_op_when_dir_missing() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("never-created");
        let log = InternalLog::new(missing);
        // Must not panic.
        log.prune_old(30);
    }

    #[test]
    fn event_fields_flatten_to_top_level() {
        let ts = Local.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap();
        let e = InternalEvent::with_ts(PLUGIN_LOADED, ts)
            .with_plugin_name("capture-commit-activity")
            .with_plugin_version("0.1.0")
            .with_field("wasm_path", "plugins/capture.wasm");
        let s = serde_json::to_string(&e).unwrap();
        let parsed: Value = serde_json::from_str(&s).unwrap();
        // No nested `fields` object: extras are flattened.
        assert!(parsed.get("fields").is_none());
        assert_eq!(parsed["wasm_path"], "plugins/capture.wasm");
        assert_eq!(parsed["plugin_name"], "capture-commit-activity");
        assert_eq!(parsed["plugin_version"], "0.1.0");
    }

    #[test]
    fn skips_serializing_none_optional_fields() {
        let ts = Local.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap();
        let e = InternalEvent::with_ts(DISPATCHER_STARTED, ts);
        let s = serde_json::to_string(&e).unwrap();
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert!(parsed.get("dispatcher_trace_id").is_none());
        assert!(parsed.get("session_id").is_none());
        assert!(parsed.get("plugin_name").is_none());
        assert!(parsed.get("plugin_version").is_none());
    }
}
