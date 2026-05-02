//! `legacy-bash-adapter` — runs unported v0.79.x bash hooks under v1.0.
//!
//! The dispatcher loads this WASI command once per registry entry that
//! references it. Per-hook configuration in the registry's
//! `[hooks.config]` table tells the adapter which bash script to run:
//!
//! ```toml
//! [[hooks]]
//! name = "validate-template-compliance"
//! event = "PostToolUse"
//! tool = "Edit|Write"
//! plugin = "hook-plugins/legacy-bash-adapter.wasm"
//!
//! [hooks.config]
//! script_path = "legacy-hooks/validate-template-compliance.sh"
//!
//! [hooks.capabilities.exec_subprocess]
//! binary_allow = ["bash"]
//! shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
//! env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR"]
//! ```
//!
//! Flow:
//! 1. Pull `script_path` from registry-supplied `plugin_config`.
//! 2. Resolve under the host's `${CLAUDE_PLUGIN_ROOT}`.
//! 3. Re-serialize the original `HookPayload` (with `plugin_config`
//!    stripped — bash hooks predate that field) and pipe it to
//!    `bash <script>` via `exec_subprocess`.
//! 4. Map bash exit code to `HookResult`:
//!    - `0` → `Continue`
//!    - `2` → `Block` (reason = first stderr line, or empty)
//!    - other → `Error` (message = stderr; routed via `on_error` policy)
//! 5. Forward stdout/stderr to the dispatcher's log pipeline via
//!    `host::log_*` so existing bash hooks that `echo` for debugging
//!    still surface.
//!
//! The adapter is a *temporary* bridge. As bash hooks port to native
//! WASM (Phase 3 stories and post-1.0 backlog), each port reduces the
//! adapter's reach. When the last bash hook is ported, the adapter can
//! be retired.

use vsdd_hook_sdk::{HookPayload, HookResult, host};

/// 1 MiB cap on combined stdout+stderr. Bash hooks today emit small
/// JSON deltas; runaway output is treated as a malformed hook and gets
/// truncated. The host enforces the same cap.
pub const MAX_OUTPUT_BYTES: u32 = 1024 * 1024;

/// Wall-clock cap for the bash subprocess. Picked higher than the
/// dispatcher's per-hook `timeout_ms` ceiling so the wasmtime epoch
/// interrupt is the source of truth — the bash timeout is a backstop
/// for the rare case where the dispatcher's epoch deadline didn't fire.
pub const BASH_TIMEOUT_MS: u32 = 60_000;

/// Pure adapter logic. Splits out from the `#[hook]` entry so unit
/// tests can drive it through synthetic payloads + a mocked subprocess
/// runner without standing up wasmtime.
pub fn adapter_logic<F>(payload: HookPayload, run_bash: F) -> HookResult
where
    F: FnOnce(&str, &[u8]) -> Result<BashOutcome, String>,
{
    let script_path = match payload.plugin_config.get("script_path") {
        Some(v) => match v.as_str() {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return HookResult::error(
                    "legacy-bash-adapter: plugin_config.script_path must be a non-empty string",
                );
            }
        },
        None => {
            return HookResult::error(
                "legacy-bash-adapter: missing plugin_config.script_path — \
                 a bash adapter registry entry must declare [hooks.config] script_path",
            );
        }
    };

    // Host-side path resolution lives in adapter_run; adapter_logic
    // works on a path string the caller decides how to resolve.
    let mut bash_payload = payload.clone();
    bash_payload.plugin_config = serde_json::Value::Null;
    let payload_bytes = match serde_json::to_vec(&bash_payload) {
        Ok(b) => b,
        Err(e) => {
            return HookResult::error(format!(
                "legacy-bash-adapter: serialize payload for bash: {e}"
            ));
        }
    };

    let outcome = match run_bash(&script_path, &payload_bytes) {
        Ok(o) => o,
        Err(e) => {
            return HookResult::error(format!(
                "legacy-bash-adapter: bash subprocess failed ({e}) — \
                 verify [hooks.capabilities.exec_subprocess] declares bash + \
                 shell_bypass_acknowledged"
            ));
        }
    };

    match outcome.exit_code {
        0 => HookResult::Continue,
        2 => {
            let reason = outcome
                .stderr
                .lines()
                .next()
                .filter(|l| !l.is_empty())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("legacy bash hook {script_path} blocked"));
            HookResult::block(reason)
        }
        code => HookResult::error(format!(
            "legacy bash hook {script_path} exited with code {code}: {}",
            outcome.stderr,
        )),
    }
}

/// Adapter outcome surface a `run_bash` callback hands back to
/// [`adapter_logic`].
#[derive(Debug, Clone)]
pub struct BashOutcome {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Production driver: resolves `script_path` against the host's plugin
/// root and dispatches via `exec_subprocess`. Called from the binary
/// crate's `#[hook]` entry.
pub fn run_bash_via_host(script_path: &str, payload_bytes: &[u8]) -> Result<BashOutcome, String> {
    let plugin_root = host::plugin_root();
    let resolved = if is_absolute(script_path) {
        script_path.to_string()
    } else {
        join_path(&plugin_root, script_path)
    };

    let result = host::exec_subprocess(
        "bash",
        &[resolved.as_str()],
        payload_bytes,
        BASH_TIMEOUT_MS,
        MAX_OUTPUT_BYTES,
    )
    .map_err(|e| format!("{e:?}"))?;

    let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    if !stdout.is_empty() {
        host::log_info(&format!("legacy-bash[{}] stdout: {}", &resolved, stdout));
    }
    if !stderr.is_empty() {
        host::log_warn(&format!("legacy-bash[{}] stderr: {}", &resolved, stderr));
    }

    Ok(BashOutcome {
        exit_code: result.exit_code,
        stdout,
        stderr,
    })
}

fn is_absolute(p: &str) -> bool {
    // The hook runs under wasm32-wasip1 but operates on dispatcher-host
    // paths (the path string is consumed by `bash` on the host). On
    // Unix we treat a leading `/` as absolute; on Windows the host
    // accepts drive-letter paths. Anything else relative-resolves under
    // plugin_root.
    p.starts_with('/') || (p.len() >= 3 && p.as_bytes()[1] == b':')
}

fn join_path(root: &str, rel: &str) -> String {
    if root.is_empty() {
        return rel.to_string();
    }
    let needs_sep = !root.ends_with('/') && !root.ends_with('\\');
    if needs_sep {
        format!("{root}/{rel}")
    } else {
        format!("{root}{rel}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload_with_config(cfg: serde_json::Value) -> HookPayload {
        HookPayload {
            event_name: "PostToolUse".to_string(),
            tool_name: "Edit".to_string(),
            session_id: "sess-1".to_string(),
            dispatcher_trace_id: "trace-1".to_string(),
            tool_input: serde_json::json!({"file_path": "src/lib.rs"}),
            tool_response: None,
            plugin_config: cfg,
            agent_type: None,
            subagent_name: None,
            last_assistant_message: None,
            result: None,
        }
    }

    fn always_ok(
        exit: i32,
        stderr: &str,
    ) -> impl FnOnce(&str, &[u8]) -> Result<BashOutcome, String> + use<> {
        let stderr = stderr.to_string();
        move |_path, _bytes| {
            Ok(BashOutcome {
                exit_code: exit,
                stdout: String::new(),
                stderr,
            })
        }
    }

    #[test]
    fn errors_when_plugin_config_missing_script_path() {
        let p = payload_with_config(serde_json::json!({}));
        match adapter_logic(p, |_, _| panic!("must not run")) {
            HookResult::Error { message } => {
                assert!(message.contains("script_path"), "got: {message}");
            }
            other => panic!("expected Error, got {other:?}"),
        }
    }

    #[test]
    fn errors_when_script_path_not_a_string() {
        let p = payload_with_config(serde_json::json!({"script_path": 42}));
        match adapter_logic(p, |_, _| panic!("must not run")) {
            HookResult::Error { message } => {
                assert!(message.contains("non-empty string"), "got: {message}");
            }
            other => panic!("expected Error, got {other:?}"),
        }
    }

    #[test]
    fn errors_when_script_path_empty() {
        let p = payload_with_config(serde_json::json!({"script_path": ""}));
        match adapter_logic(p, |_, _| panic!("must not run")) {
            HookResult::Error { message } => assert!(message.contains("non-empty")),
            other => panic!("expected Error, got {other:?}"),
        }
    }

    #[test]
    fn maps_exit_zero_to_continue() {
        let p = payload_with_config(serde_json::json!({"script_path": "x.sh"}));
        let r = adapter_logic(p, always_ok(0, ""));
        assert!(matches!(r, HookResult::Continue));
    }

    #[test]
    fn maps_exit_two_to_block_with_first_stderr_line() {
        let p = payload_with_config(serde_json::json!({"script_path": "validate.sh"}));
        let r = adapter_logic(
            p,
            always_ok(2, "policy violation: imports unsafe\nstack trace..."),
        );
        match r {
            HookResult::Block { reason } => {
                assert_eq!(reason, "policy violation: imports unsafe");
            }
            other => panic!("expected Block, got {other:?}"),
        }
    }

    #[test]
    fn maps_exit_two_with_no_stderr_to_synthetic_block_reason() {
        let p = payload_with_config(serde_json::json!({"script_path": "v.sh"}));
        let r = adapter_logic(p, always_ok(2, ""));
        match r {
            HookResult::Block { reason } => {
                assert!(reason.contains("v.sh"));
                assert!(reason.contains("blocked"));
            }
            other => panic!("expected Block, got {other:?}"),
        }
    }

    #[test]
    fn maps_other_nonzero_to_error_with_stderr() {
        let p = payload_with_config(serde_json::json!({"script_path": "broken.sh"}));
        let r = adapter_logic(p, always_ok(127, "command not found: jq"));
        match r {
            HookResult::Error { message } => {
                assert!(message.contains("127"));
                assert!(message.contains("command not found: jq"));
                assert!(message.contains("broken.sh"));
            }
            other => panic!("expected Error, got {other:?}"),
        }
    }

    #[test]
    fn passes_payload_bytes_to_bash_with_plugin_config_stripped() {
        let p = payload_with_config(serde_json::json!({"script_path": "echo.sh", "extra": 1}));
        // capture what the runner receives
        let captured = std::cell::RefCell::new(Vec::new());
        let r = adapter_logic(p, |path, bytes| {
            assert_eq!(path, "echo.sh");
            *captured.borrow_mut() = bytes.to_vec();
            Ok(BashOutcome {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            })
        });
        assert!(matches!(r, HookResult::Continue));
        let bytes = captured.into_inner();
        let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(
            parsed
                .get("plugin_config")
                .map(|v| v.is_null())
                .unwrap_or(false),
            "plugin_config should be stripped to null, got {:?}",
            parsed.get("plugin_config"),
        );
        assert_eq!(
            parsed.get("event_name").and_then(|v| v.as_str()),
            Some("PostToolUse"),
        );
        assert_eq!(
            parsed.get("dispatcher_trace_id").and_then(|v| v.as_str()),
            Some("trace-1"),
        );
    }

    #[test]
    fn surfaces_runner_error_as_hook_error() {
        let p = payload_with_config(serde_json::json!({"script_path": "x.sh"}));
        let r = adapter_logic(p, |_, _| Err("CapabilityDenied".to_string()));
        match r {
            HookResult::Error { message } => {
                assert!(message.contains("CapabilityDenied"));
                assert!(message.contains("shell_bypass_acknowledged"));
            }
            other => panic!("expected Error, got {other:?}"),
        }
    }

    #[test]
    fn is_absolute_recognizes_unix_root() {
        assert!(is_absolute("/usr/bin/bash"));
        assert!(!is_absolute("rel/path.sh"));
    }

    #[test]
    fn is_absolute_recognizes_windows_drive() {
        assert!(is_absolute("C:/tools/bash.sh"));
        assert!(is_absolute("d:\\tools\\bash.sh"));
        assert!(!is_absolute("ar.sh"));
    }

    #[test]
    fn join_path_inserts_separator_when_missing() {
        assert_eq!(join_path("/plugin", "hooks/x.sh"), "/plugin/hooks/x.sh");
    }

    #[test]
    fn join_path_respects_existing_trailing_separator() {
        assert_eq!(join_path("/plugin/", "hooks/x.sh"), "/plugin/hooks/x.sh");
        assert_eq!(
            join_path("C:\\plugin\\", "hooks/x.sh"),
            "C:\\plugin\\hooks/x.sh"
        );
    }

    #[test]
    fn join_path_with_empty_root_returns_relative() {
        assert_eq!(join_path("", "hooks/x.sh"), "hooks/x.sh");
    }
}
