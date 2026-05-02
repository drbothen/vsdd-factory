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

/// Write a file at the given path through the dispatcher's bounded host function.
///
/// This is the write-side symmetric counterpart to [`read_file`].  Both
/// `max_bytes` and `timeout_ms` are mandatory — `write_file` is a bounded
/// host call per BC-2.02.011 and BC-2.02.002; the API enforces the caps at
/// the type level so plugins can't make an unbounded write call.
///
/// # Protocol
///
/// Uses the **input-pointer protocol** (BC-2.02.011 invariant 4):
/// the SDK passes guest-owned bytes `(contents_ptr, contents_len)` to the
/// dispatcher, which copies them out via `read_wasm_bytes`.  This differs
/// from [`read_file`]'s output-pointer protocol.
///
/// # Error codes
///
/// | Returned error | Cause |
/// |---|---|
/// | `HostError::CapabilityDenied` | Path not in `write_file.path_allow`, path traversal, or no capability block |
/// | `HostError::OutputTooLarge` | `contents.len() > max_bytes` |
/// | `HostError::Timeout` | Write exceeded `timeout_ms` budget |
/// | `HostError::Other(-99)` | Filesystem I/O error or missing parent directory |
///
/// # Examples
///
/// On non-wasm targets the FFI stub returns `CAPABILITY_DENIED` because no
/// dispatcher is present.  In a real plugin compiled to `wasm32-wasip1` the
/// dispatcher validates capabilities and writes the file.
///
/// ```rust
/// use vsdd_hook_sdk::host::{HostError, write_file};
///
/// // Non-wasm (doc-test) target: stub returns Err(CapabilityDenied).
/// let result = write_file(".factory/STATE.md", b"updated state", 65536, 5000);
/// assert_eq!(result, Err(HostError::CapabilityDenied));
/// ```
///
/// Both `max_bytes` and `timeout_ms` are mandatory (BC-2.02.011 invariant 2):
///
/// ```rust
/// use vsdd_hook_sdk::host::{HostError, write_file};
///
/// // max_bytes cap is enforced by the dispatcher; timeout_ms by epoch interruption.
/// // On non-wasm stub both are accepted as parameters without panic.
/// let _ = write_file("/tmp/out.txt", b"", 0, 0);
/// ```
///
/// # References
///
/// BC-2.02.011 — host::write_file: bounded write capability with allowlist enforcement.
/// Postconditions 1-5; invariants 1-5 (HOST_ABI_VERSION stays 1; max_bytes mandatory;
/// input-pointer protocol; error codes stable).
pub fn write_file(
    path: &str,
    contents: &[u8],
    max_bytes: u32,
    timeout_ms: u32,
) -> Result<(), HostError> {
    let path_bytes = path.as_bytes();
    let code = ffi::write_file(
        path_bytes.as_ptr(),
        path_bytes.len() as u32,
        contents.as_ptr(),
        contents.len() as u32,
        max_bytes,
        timeout_ms,
    );
    if code < 0 {
        return Err(HostError::from_code(code));
    }
    Ok(())
}

/// Result of an `exec_subprocess` call.
#[derive(Debug, Clone)]
pub struct SubprocessResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Spawn a subprocess against the dispatcher's binary allow-list.
/// `timeout_ms` and `max_output_bytes` are mandatory. Pass `&[]` for
/// `stdin` if the subprocess should receive no input (the common case).
///
/// Result protocol: the SDK allocates a buffer of
/// `max_output_bytes + RESULT_ENVELOPE_OVERHEAD`, hands the host a
/// `(ptr, cap)` pair, and the host writes the result envelope there.
/// The host's return value is the number of bytes written (positive)
/// or a negative error code. We avoid the previous "host writes at
/// offset 0" pattern because offset 0 is reserved guest memory and
/// writing there clobbers wasm runtime state.
pub fn exec_subprocess(
    cmd: &str,
    args: &[&str],
    stdin: &[u8],
    timeout_ms: u32,
    max_output_bytes: u32,
) -> Result<SubprocessResult, HostError> {
    let cmd_bytes = cmd.as_bytes();
    let args_buf = encode_args(args);
    // Envelope overhead: exit_code (4) + stdout_len (4) + stderr_len (4)
    // = 12 bytes. Pad a bit; this is a one-shot per-call alloc.
    let buf_cap = max_output_bytes.saturating_add(RESULT_ENVELOPE_OVERHEAD);
    let mut buf = vec![0u8; buf_cap as usize];
    let written = ffi::exec_subprocess(
        cmd_bytes.as_ptr(),
        cmd_bytes.len() as u32,
        args_buf.as_ptr(),
        args_buf.len() as u32,
        stdin.as_ptr(),
        stdin.len() as u32,
        timeout_ms,
        max_output_bytes,
        buf.as_mut_ptr(),
        buf_cap,
    );
    if written < 0 {
        return Err(HostError::from_code(written));
    }
    buf.truncate(written as usize);
    decode_subprocess_result(&buf).ok_or(HostError::Other(-99))
}

/// Bytes the host adds to each subprocess result envelope on top of
/// the user-controlled `max_output_bytes`. 12 bytes of length headers
/// (exit_code + stdout_len + stderr_len) plus 4 bytes of slack.
const RESULT_ENVELOPE_OVERHEAD: u32 = 16;

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
