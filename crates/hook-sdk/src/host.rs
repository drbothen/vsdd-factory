//! Ergonomic Rust wrappers around the vsdd-factory host function ABI.
//!
//! These are the surface plugin authors should use. The raw FFI lives in
//! [`crate::ffi`]; using it directly bypasses the safety and string
//! handling these wrappers provide.
//!
//! Every wrapper is bounded — timeouts and byte caps are mandatory at
//! the API level so plugins can't accidentally make an unbounded host
//! call. The dispatcher enforces the bounds again, but expressing them
//! in the type system catches misuse at compile time.

use crate::ffi;

/// Severity for [`log`] / [`log_info`] etc. Mirrors the dispatcher's
/// internal levels; integer values are stable as part of the ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

/// Write a structured log line to the dispatcher's internal log.
///
/// Always succeeds — log writes are best-effort.
pub fn log(level: LogLevel, msg: &str) {
    let bytes = msg.as_bytes();
    ffi::log(level as u32, bytes.as_ptr(), bytes.len() as u32);
}

/// Convenience: `log(LogLevel::Info, msg)`.
pub fn log_info(msg: &str) {
    log(LogLevel::Info, msg);
}

/// Convenience: `log(LogLevel::Warn, msg)`.
pub fn log_warn(msg: &str) {
    log(LogLevel::Warn, msg);
}

/// Convenience: `log(LogLevel::Error, msg)`.
pub fn log_error(msg: &str) {
    log(LogLevel::Error, msg);
}

/// Emit a structured event into the configured sinks.
///
/// `event_type` is the canonical event name (e.g. `"commit.made"`).
/// `fields` are key/value pairs serialized as length-prefixed UTF-8.
pub fn emit_event(event_type: &str, fields: &[(&str, &str)]) {
    let payload = encode_fields(fields);
    let etype = event_type.as_bytes();
    ffi::emit_event(
        etype.as_ptr(),
        etype.len() as u32,
        payload.as_ptr(),
        payload.len() as u32,
    );
}

fn encode_fields(fields: &[(&str, &str)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(
        fields
            .iter()
            .map(|(k, v)| 8 + k.len() + v.len())
            .sum::<usize>(),
    );
    for (k, v) in fields {
        buf.extend_from_slice(&(k.len() as u32).to_le_bytes());
        buf.extend_from_slice(k.as_bytes());
        buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
        buf.extend_from_slice(v.as_bytes());
    }
    buf
}

/// Errors returned by bounded host calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostError {
    /// The caller does not have the capability for this operation.
    CapabilityDenied,
    /// The host call exceeded its `timeout_ms` budget.
    Timeout,
    /// The output exceeded the per-call cap; truncated.
    OutputTooLarge,
    /// The argument failed host-side validation (path traversal, etc.).
    InvalidArgument,
    /// The host operation failed for a reason not classified above.
    /// `code` is the negative error number returned by the host.
    Other(i32),
}

impl HostError {
    fn from_code(code: i32) -> Self {
        match code {
            -1 => HostError::CapabilityDenied,
            -2 => HostError::Timeout,
            -3 => HostError::OutputTooLarge,
            -4 => HostError::InvalidArgument,
            other => HostError::Other(other),
        }
    }
}

/// Read a fixed-capacity string from a `(out_ptr, out_cap) -> bytes_written`
/// host call into a Rust `String`.
fn read_string<F>(call: F) -> String
where
    F: Fn(*mut u8, u32) -> u32,
{
    let mut buf = vec![0u8; 256];
    let written = call(buf.as_mut_ptr(), buf.len() as u32);
    if written as usize > buf.len() {
        // Host wanted more than we offered — re-call with the requested capacity.
        buf.resize(written as usize, 0);
        let written = call(buf.as_mut_ptr(), buf.len() as u32);
        buf.truncate(written as usize);
    } else {
        buf.truncate(written as usize);
    }
    String::from_utf8(buf).unwrap_or_default()
}

// `read_string` takes a closure rather than the `extern "C" fn` items
// directly because `pub safe fn` items don't coerce to a `Fn(...)` bound
// the way bare function pointers do. Clippy on host targets sees these
// as redundant (the host stubs ARE plain fns) — silenced per call site.

/// Current Claude Code session id.
#[allow(clippy::redundant_closure)]
pub fn session_id() -> String {
    read_string(|p, c| ffi::session_id(p, c))
}

/// Per-invocation dispatcher trace id.
#[allow(clippy::redundant_closure)]
pub fn dispatcher_trace_id() -> String {
    read_string(|p, c| ffi::dispatcher_trace_id(p, c))
}

/// `${CLAUDE_PLUGIN_ROOT}` resolved at the host.
#[allow(clippy::redundant_closure)]
pub fn plugin_root() -> String {
    read_string(|p, c| ffi::plugin_root(p, c))
}

/// SemVer string of the running plugin.
#[allow(clippy::redundant_closure)]
pub fn plugin_version() -> String {
    read_string(|p, c| ffi::plugin_version(p, c))
}

/// Current working directory.
#[allow(clippy::redundant_closure)]
pub fn cwd() -> String {
    read_string(|p, c| ffi::cwd(p, c))
}

/// Read a single environment variable name. Returns `None` if the
/// dispatcher's env allow-list does not include the name (capability
/// denial) or the variable is unset.
pub fn env(name: &str) -> Result<Option<String>, HostError> {
    let name_bytes = name.as_bytes();
    let mut buf = vec![0u8; 1024];
    let written = ffi::env(
        name_bytes.as_ptr(),
        name_bytes.len() as u32,
        buf.as_mut_ptr(),
        buf.len() as u32,
    );
    if written < 0 {
        return Err(HostError::from_code(written));
    }
    if written == 0 {
        return Ok(None);
    }
    buf.truncate(written as usize);
    Ok(Some(String::from_utf8(buf).unwrap_or_default()))
}

/// Read a file at the given path through the dispatcher's bounded
/// host function. Always pass a `max_bytes` cap and a `timeout_ms`
/// budget — the API requires both.
pub fn read_file(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError> {
    let path_bytes = path.as_bytes();
    let mut out_ptr: u32 = 0;
    let mut out_len: u32 = 0;
    let code = ffi::read_file(
        path_bytes.as_ptr(),
        path_bytes.len() as u32,
        max_bytes,
        timeout_ms,
        &mut out_ptr,
        &mut out_len,
    );
    if code < 0 {
        return Err(HostError::from_code(code));
    }
    // The host writes into a buffer it owns; we copy it before the
    // call returns so the SDK doesn't expose pointer lifetimes.
    Ok(read_owned_bytes(out_ptr, out_len))
}

/// Result of an `exec_subprocess` call.
#[derive(Debug, Clone)]
pub struct SubprocessResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Spawn a subprocess against the dispatcher's binary allow-list.
/// `timeout_ms` and `max_output_bytes` are mandatory.
pub fn exec_subprocess(
    cmd: &str,
    args: &[&str],
    timeout_ms: u32,
    max_output_bytes: u32,
) -> Result<SubprocessResult, HostError> {
    let cmd_bytes = cmd.as_bytes();
    let args_buf = encode_args(args);
    let mut result_ptr: u32 = 0;
    let mut result_len: u32 = 0;
    let code = ffi::exec_subprocess(
        cmd_bytes.as_ptr(),
        cmd_bytes.len() as u32,
        args_buf.as_ptr(),
        args_buf.len() as u32,
        timeout_ms,
        max_output_bytes,
        &mut result_ptr,
        &mut result_len,
    );
    if code < 0 {
        return Err(HostError::from_code(code));
    }
    let owned = read_owned_bytes(result_ptr, result_len);
    decode_subprocess_result(&owned).ok_or(HostError::Other(-99))
}

fn encode_args(args: &[&str]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(args.iter().map(|a| 4 + a.len()).sum::<usize>());
    for a in args {
        buf.extend_from_slice(&(a.len() as u32).to_le_bytes());
        buf.extend_from_slice(a.as_bytes());
    }
    buf
}

fn decode_subprocess_result(bytes: &[u8]) -> Option<SubprocessResult> {
    if bytes.len() < 4 {
        return None;
    }
    let exit_code = i32::from_le_bytes(bytes[0..4].try_into().ok()?);
    let stdout_len = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let stdout_end = 8 + stdout_len;
    if bytes.len() < stdout_end + 4 {
        return None;
    }
    let stdout = bytes[8..stdout_end].to_vec();
    let stderr_len =
        u32::from_le_bytes(bytes[stdout_end..stdout_end + 4].try_into().ok()?) as usize;
    let stderr_end = stdout_end + 4 + stderr_len;
    if bytes.len() < stderr_end {
        return None;
    }
    let stderr = bytes[stdout_end + 4..stderr_end].to_vec();
    Some(SubprocessResult {
        exit_code,
        stdout,
        stderr,
    })
}

#[cfg(target_arch = "wasm32")]
fn read_owned_bytes(ptr: u32, len: u32) -> Vec<u8> {
    if ptr == 0 || len == 0 {
        return Vec::new();
    }
    // Safety: the host promises the (ptr, len) range is valid and
    // owned by the wasm module after the call returns; the caller
    // (the SDK) takes ownership and copies on read.
    unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize).to_vec() }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_owned_bytes(_ptr: u32, _len: u32) -> Vec<u8> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_fields_uses_length_prefix() {
        let buf = encode_fields(&[("k", "vv"), ("aa", "b")]);
        // 4 + 1 + 4 + 2 + 4 + 2 + 4 + 1
        assert_eq!(buf.len(), 4 + 1 + 4 + 2 + 4 + 2 + 4 + 1);
        // First key length
        assert_eq!(&buf[0..4], &1u32.to_le_bytes());
        assert_eq!(buf[4], b'k');
        // First value length
        assert_eq!(&buf[5..9], &2u32.to_le_bytes());
        assert_eq!(&buf[9..11], b"vv");
    }

    #[test]
    fn encode_args_round_trip() {
        let buf = encode_args(&["a", "bb", "ccc"]);
        assert_eq!(buf.len(), 4 + 1 + 4 + 2 + 4 + 3);
        assert_eq!(&buf[0..4], &1u32.to_le_bytes());
        assert_eq!(buf[4], b'a');
    }

    #[test]
    fn host_error_code_mapping() {
        assert_eq!(HostError::from_code(-1), HostError::CapabilityDenied);
        assert_eq!(HostError::from_code(-2), HostError::Timeout);
        assert_eq!(HostError::from_code(-3), HostError::OutputTooLarge);
        assert_eq!(HostError::from_code(-4), HostError::InvalidArgument);
        assert_eq!(HostError::from_code(-99), HostError::Other(-99));
    }

    #[test]
    fn decode_subprocess_result_parses_envelope() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&7i32.to_le_bytes());
        bytes.extend_from_slice(&3u32.to_le_bytes());
        bytes.extend_from_slice(b"out");
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(b"er");
        let r = decode_subprocess_result(&bytes).expect("parses");
        assert_eq!(r.exit_code, 7);
        assert_eq!(r.stdout, b"out");
        assert_eq!(r.stderr, b"er");
    }

    #[test]
    fn decode_subprocess_result_rejects_truncated() {
        assert!(decode_subprocess_result(&[]).is_none());
        assert!(decode_subprocess_result(&[1, 2]).is_none());
    }

    #[test]
    fn log_levels_are_stable() {
        assert_eq!(LogLevel::Trace as u32, 0);
        assert_eq!(LogLevel::Debug as u32, 1);
        assert_eq!(LogLevel::Info as u32, 2);
        assert_eq!(LogLevel::Warn as u32, 3);
        assert_eq!(LogLevel::Error as u32, 4);
    }
}
