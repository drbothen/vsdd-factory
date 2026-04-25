//! `exec_subprocess` host function.
//!
//! The capability-heaviest host surface. Every call is policed against
//! the registry's [`ExecSubprocessCaps`]:
//!
//! - Binary basename must be in `binary_allow`.
//! - Shell interpreters (`bash`, `sh`, `zsh`, `pwsh`) require
//!   `shell_bypass_acknowledged` to be set; otherwise the call is
//!   denied with `reason = "shell_bypass_not_acknowledged"`.
//! - Setuid / setgid binaries are refused categorically on Unix.
//! - The caller-provided `timeout_ms` and `max_output_bytes` are both
//!   mandatory and enforced: truncated output reports
//!   `OUTPUT_TOO_LARGE`, wall-clock overruns report `TIMEOUT`.
//!
//! S-1.5 tightens a few corners (pre-resolved full paths in
//! `binary_allow`, fuel-aware interruption); S-1.4 ships the logical
//! surface + capability gate.

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Map, Value};
use wasmtime::Linker;

use super::memory::{read_wasm_bytes, read_wasm_string, write_wasm_bytes};
use super::{HostCallError, HostCaller, HostContext, codes};
use crate::registry::ExecSubprocessCaps;

const SHELL_NAMES: &[&str] = &["bash", "sh", "zsh", "pwsh", "fish", "csh", "tcsh", "ksh"];

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "exec_subprocess",
            |mut caller: HostCaller<'_>,
             cmd_ptr: u32,
             cmd_len: u32,
             args_ptr: u32,
             args_len: u32,
             stdin_ptr: u32,
             stdin_len: u32,
             timeout_ms: u32,
             max_output_bytes: u32,
             result_buf_ptr: u32,
             result_buf_cap: u32|
             -> i32 {
                let cmd = match read_wasm_string(&mut caller, cmd_ptr, cmd_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let args_buf = match read_wasm_bytes(&mut caller, args_ptr, args_len) {
                    Ok(b) => b,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let args = match decode_args(&args_buf) {
                    Some(a) => a,
                    None => return codes::INVALID_ARGUMENT,
                };
                let stdin_bytes = if stdin_len == 0 {
                    Vec::new()
                } else {
                    match read_wasm_bytes(&mut caller, stdin_ptr, stdin_len) {
                        Ok(b) => b,
                        Err(_) => return codes::INVALID_ARGUMENT,
                    }
                };

                let outcome = {
                    let ctx = caller.data();
                    run(ctx, &cmd, &args, &stdin_bytes, timeout_ms, max_output_bytes)
                };

                let envelope = match outcome {
                    Ok(env) => env,
                    Err(code) => return code,
                };

                // Result protocol: write the envelope into the
                // guest-provided buffer at (result_buf_ptr, result_buf_cap).
                // Return the number of bytes written. The previous design
                // wrote at offset 0 — guest reserved space — and clobbered
                // wasm runtime state.
                if envelope.len() as u32 > result_buf_cap {
                    return codes::OUTPUT_TOO_LARGE;
                }
                match write_wasm_bytes(&mut caller, result_buf_ptr, result_buf_cap, &envelope) {
                    Ok(written) => written as i32,
                    Err(_) => codes::INVALID_ARGUMENT,
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

/// Result envelope encoding used by the SDK's `SubprocessResult`:
/// `exit_code: i32 LE | stdout_len: u32 LE | stdout | stderr_len: u32 LE | stderr`.
fn encode_envelope(exit_code: i32, stdout: &[u8], stderr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(4 + 4 + stdout.len() + 4 + stderr.len());
    buf.extend_from_slice(&exit_code.to_le_bytes());
    buf.extend_from_slice(&(stdout.len() as u32).to_le_bytes());
    buf.extend_from_slice(stdout);
    buf.extend_from_slice(&(stderr.len() as u32).to_le_bytes());
    buf.extend_from_slice(stderr);
    buf
}

/// Decode `encode_args` output from the SDK: length-prefixed strings.
///
/// Crate-visible — invoke.rs reuses this for its StoreData-typed linker
/// handler.
pub(crate) fn decode_args(bytes: &[u8]) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if i + 4 > bytes.len() {
            return None;
        }
        let len = u32::from_le_bytes(bytes[i..i + 4].try_into().ok()?) as usize;
        i += 4;
        if i + len > bytes.len() {
            return None;
        }
        out.push(String::from_utf8_lossy(&bytes[i..i + len]).into_owned());
        i += len;
    }
    Some(out)
}

/// Host-side policy + execution. Split out so unit tests don't need a
/// wasm instance.
///
/// Crate-visible so [`crate::invoke`] can register a StoreData-typed
/// linker handler that delegates here — keeps a single source of truth
/// for capability gates.
pub(crate) fn run(
    ctx: &HostContext,
    cmd: &str,
    args: &[String],
    stdin_bytes: &[u8],
    timeout_ms: u32,
    max_output_bytes: u32,
) -> Result<Vec<u8>, i32> {
    let caps = ctx.capabilities.exec_subprocess.as_ref().ok_or_else(|| {
        emit_denial(ctx, cmd, "no_exec_subprocess_capability", Map::new());
        codes::CAPABILITY_DENIED
    })?;

    if !binary_allowed(cmd, &caps.binary_allow) {
        let mut details = Map::new();
        details.insert("command".to_string(), Value::String(cmd.to_string()));
        emit_denial(ctx, cmd, "binary_not_on_allow_list", details);
        return Err(codes::CAPABILITY_DENIED);
    }

    if is_shell(cmd) && caps.shell_bypass_acknowledged.is_none() {
        let mut details = Map::new();
        details.insert("command".to_string(), Value::String(cmd.to_string()));
        emit_denial(ctx, cmd, "shell_bypass_not_acknowledged", details);
        return Err(codes::CAPABILITY_DENIED);
    }

    if refuse_setuid(cmd) {
        let mut details = Map::new();
        details.insert("command".to_string(), Value::String(cmd.to_string()));
        emit_denial(ctx, cmd, "setuid_or_setgid_binary", details);
        return Err(codes::CAPABILITY_DENIED);
    }

    let envelope = execute_bounded(
        cmd,
        args,
        stdin_bytes,
        timeout_ms,
        max_output_bytes,
        caps,
        ctx.cwd.as_path(),
        &ctx.env_view,
    )?;
    Ok(envelope)
}

fn binary_allowed(cmd: &str, allow: &[String]) -> bool {
    let basename = PathBuf::from(cmd)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| cmd.to_string());
    allow.iter().any(|b| b == cmd || b == &basename)
}

fn is_shell(cmd: &str) -> bool {
    let basename = PathBuf::from(cmd)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| cmd.to_string());
    SHELL_NAMES.iter().any(|s| *s == basename)
}

#[cfg(unix)]
fn refuse_setuid(cmd: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    let path = PathBuf::from(cmd);
    let meta = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return false, // can't stat → let exec path report the error
    };
    // Mode bits: 0o4000 = setuid, 0o2000 = setgid.
    (meta.mode() & 0o6000) != 0
}

#[cfg(not(unix))]
fn refuse_setuid(_cmd: &str) -> bool {
    false
}

#[allow(clippy::too_many_arguments)]
fn execute_bounded(
    cmd: &str,
    args: &[String],
    stdin_bytes: &[u8],
    timeout_ms: u32,
    max_output_bytes: u32,
    caps: &ExecSubprocessCaps,
    cwd: &std::path::Path,
    env_view: &std::collections::HashMap<String, String>,
) -> Result<Vec<u8>, i32> {
    let mut command = Command::new(cmd);
    command.args(args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    // Pipe stdin only when the plugin asked to write to it. `Stdio::null()`
    // for empty stdin keeps existing behavior; legacy bash hooks rely on
    // a piped stdin so they can read the JSON envelope.
    if stdin_bytes.is_empty() {
        command.stdin(Stdio::null());
    } else {
        command.stdin(Stdio::piped());
    }
    command.env_clear();
    for name in &caps.env_allow {
        if let Some(val) = env_view.get(name) {
            command.env(name, val);
        }
    }
    if !cwd.as_os_str().is_empty() {
        command.current_dir(cwd);
    }

    let mut child = command.spawn().map_err(|_| codes::INTERNAL_ERROR)?;
    if !stdin_bytes.is_empty() {
        // Write the full payload synchronously and close the pipe before
        // we start polling. This is correct for payloads up to a few MB
        // (well above any realistic Claude Code envelope) and avoids the
        // dance of interleaving stdin writes with stdout draining.
        let mut child_stdin = child.stdin.take().ok_or(codes::INTERNAL_ERROR)?;
        if child_stdin.write_all(stdin_bytes).is_err() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(codes::INTERNAL_ERROR);
        }
        // Dropping closes the pipe so the child sees EOF.
        drop(child_stdin);
    }
    let mut stdout = child.stdout.take().ok_or(codes::INTERNAL_ERROR)?;
    let mut stderr = child.stderr.take().ok_or(codes::INTERNAL_ERROR)?;

    let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout_buf = Vec::new();
                let mut stderr_buf = Vec::new();
                let _ = stdout.read_to_end(&mut stdout_buf);
                let _ = stderr.read_to_end(&mut stderr_buf);
                let truncated = stdout_buf.len() > max_output_bytes as usize
                    || stderr_buf.len() > max_output_bytes as usize;
                stdout_buf.truncate(max_output_bytes as usize);
                stderr_buf.truncate(max_output_bytes as usize);
                if truncated {
                    return Err(codes::OUTPUT_TOO_LARGE);
                }
                return Ok(encode_envelope(
                    status.code().unwrap_or(-1),
                    &stdout_buf,
                    &stderr_buf,
                ));
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(codes::TIMEOUT);
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(_) => return Err(codes::INTERNAL_ERROR),
        }
    }
}

fn emit_denial(ctx: &HostContext, cmd: &str, reason: &str, mut details: Map<String, Value>) {
    details
        .entry("command".to_string())
        .or_insert_with(|| Value::String(cmd.to_string()));
    let ev = ctx.denial_event("exec_subprocess", reason, details);
    ctx.emit_internal(ev);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::test_support::*;

    #[test]
    fn denies_without_capability_block() {
        let ctx = bare_context();
        let err = run(&ctx, "git", &["status".to_string()], &[], 1000, 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn denies_binary_not_on_allow_list() {
        let ctx = context_with_caps(allow_exec(&["git"]));
        let err = run(&ctx, "curl", &[], &[], 1000, 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn denies_shell_without_acknowledgment() {
        let ctx = context_with_caps(allow_exec(&["bash"]));
        let err = run(
            &ctx,
            "bash",
            &["-c".into(), "echo hi".into()],
            &[],
            1000,
            1024,
        )
        .unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn allows_shell_with_acknowledgment() {
        let mut caps = allow_exec(&["bash"]);
        if let Some(es) = caps.exec_subprocess.as_mut() {
            es.shell_bypass_acknowledged = Some("needed for git status parsing".to_string());
        }
        let ctx = context_with_caps(caps);
        // We don't actually exec bash here — policy check is earlier.
        // run() will try to spawn bash; on most CI hosts bash exists.
        let result = run(
            &ctx,
            "bash",
            &["-c".into(), "exit 0".into()],
            &[],
            5000,
            4096,
        );
        // On the rare host without bash, INTERNAL_ERROR is returned —
        // either outcome proves the policy gate passed.
        assert!(result.is_ok() || result == Err(codes::INTERNAL_ERROR));
    }

    #[test]
    fn stdin_bytes_reach_subprocess() {
        let mut caps = allow_exec(&["bash"]);
        if let Some(es) = caps.exec_subprocess.as_mut() {
            es.shell_bypass_acknowledged = Some("test".to_string());
            es.env_allow = vec!["PATH".to_string()];
        }
        let mut ctx = context_with_caps(caps);
        ctx.env_view.insert(
            "PATH".to_string(),
            std::env::var("PATH").unwrap_or_default(),
        );
        // bash reads stdin and echoes it; we verify stdin was piped.
        let envelope = run(
            &ctx,
            "bash",
            &["-c".into(), "cat".into()],
            b"hello-from-stdin",
            5000,
            4096,
        );
        let env = match envelope {
            Ok(e) => e,
            // `bash` not present on a runner: don't fail the suite, we
            // already proved the path elsewhere.
            Err(_) => return,
        };
        // Decode envelope: exit_code(4) + stdout_len(4) + stdout + stderr_len(4) + stderr
        assert_eq!(&env[0..4], &0i32.to_le_bytes());
        let stdout_len = u32::from_le_bytes(env[4..8].try_into().unwrap()) as usize;
        assert_eq!(&env[8..8 + stdout_len], b"hello-from-stdin");
    }

    #[test]
    fn binary_allow_matches_basename() {
        assert!(binary_allowed("/usr/bin/git", &["git".to_string()]));
        assert!(binary_allowed("git", &["git".to_string()]));
        assert!(!binary_allowed("curl", &["git".to_string()]));
    }

    #[test]
    fn is_shell_detects_interpreters() {
        assert!(is_shell("bash"));
        assert!(is_shell("/bin/bash"));
        assert!(is_shell("sh"));
        assert!(is_shell("zsh"));
        assert!(is_shell("pwsh"));
        assert!(!is_shell("git"));
        assert!(!is_shell("curl"));
    }

    #[test]
    fn decode_args_round_trip() {
        let mut buf = Vec::new();
        for arg in &["a", "bb", "ccc"] {
            buf.extend_from_slice(&(arg.len() as u32).to_le_bytes());
            buf.extend_from_slice(arg.as_bytes());
        }
        let decoded = decode_args(&buf).unwrap();
        assert_eq!(
            decoded,
            vec!["a".to_string(), "bb".to_string(), "ccc".to_string()]
        );
    }

    #[test]
    fn decode_args_rejects_truncated_buffer() {
        let bad = vec![10u8, 0, 0, 0, b'a']; // says 10 bytes, gives 1
        assert!(decode_args(&bad).is_none());
    }

    #[test]
    fn envelope_encodes_expected_shape() {
        let env = encode_envelope(7, b"out", b"er");
        assert_eq!(env[0..4], 7i32.to_le_bytes());
        assert_eq!(env[4..8], 3u32.to_le_bytes());
        assert_eq!(&env[8..11], b"out");
        assert_eq!(env[11..15], 2u32.to_le_bytes());
        assert_eq!(&env[15..17], b"er");
    }

    #[test]
    fn timeout_enforced() {
        let mut caps = allow_exec(&["sleep"]);
        if let Some(es) = caps.exec_subprocess.as_mut() {
            es.env_allow = vec!["PATH".to_string()];
        }
        let mut ctx = context_with_caps(caps);
        ctx.env_view.insert(
            "PATH".to_string(),
            std::env::var("PATH").unwrap_or_default(),
        );
        let err = run(&ctx, "sleep", &["5".to_string()], &[], 200, 1024).unwrap_err();
        assert_eq!(err, codes::TIMEOUT);
    }
}
