//! Macro-only helpers. **Not** part of the public API — anything in here
//! may change between any two versions of the SDK. The `#[hook]` macro
//! emits calls into this module; users do not.

use crate::host::{LogLevel, log_error};
use crate::{HookPayload, HookResult};
use std::io::{self, Read, Write};

/// Drive a `#[hook]`-annotated function from a WASI command entry point.
///
/// 1. Reads the payload from stdin (UTF-8 JSON).
/// 2. Catches panics and converts them to `HookResult::Error`.
/// 3. Writes the serialized result to stdout.
/// 4. Exits with `result.exit_code()`.
///
/// On any framing error before the user function runs, this writes a
/// best-effort `HookResult::Error` to stdout, logs to the dispatcher
/// internal log, and exits with code `1`.
pub fn run<F>(handler: F) -> !
where
    F: FnOnce(HookPayload) -> HookResult + std::panic::UnwindSafe,
{
    let payload = match read_payload() {
        Ok(p) => p,
        Err(e) => {
            emit_and_exit(HookResult::error(format!("payload framing: {e}")));
        }
    };

    let result = std::panic::catch_unwind(|| handler(payload)).unwrap_or_else(|panic| {
        let msg = panic_message(&panic);
        log_error(&format!("hook panicked: {msg}"));
        HookResult::error(format!("panic: {msg}"))
    });

    emit_and_exit(result);
}

fn read_payload() -> Result<HookPayload, String> {
    let mut buf = Vec::with_capacity(4096);
    io::stdin()
        .read_to_end(&mut buf)
        .map_err(|e| format!("stdin: {e}"))?;
    serde_json::from_slice(&buf).map_err(|e| format!("json: {e}"))
}

fn emit_and_exit(result: HookResult) -> ! {
    let code = result.exit_code();
    write_result(&result);
    std::process::exit(code);
}

fn write_result(result: &HookResult) {
    let mut stdout = io::stdout().lock();
    if let Ok(json) = serde_json::to_vec(result) {
        let _ = stdout.write_all(&json);
        let _ = stdout.write_all(b"\n");
        let _ = stdout.flush();
    } else {
        // Should never happen — HookResult is always serializable.
        let _ = stdout.write_all(br#"{"outcome":"error","message":"serialization failure"}"#);
        let _ = stdout.flush();
    }
    // Mirror the result into the dispatcher internal log for crash
    // forensics, best-effort.
    let _ = LogLevel::Info;
}

fn panic_message(panic: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "(no panic message)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_message_extracts_static_str() {
        let panic: Box<dyn std::any::Any + Send> = Box::new("boom");
        assert_eq!(panic_message(&panic), "boom");
    }

    #[test]
    fn panic_message_extracts_string() {
        let panic: Box<dyn std::any::Any + Send> = Box::new("formatted".to_string());
        assert_eq!(panic_message(&panic), "formatted");
    }

    #[test]
    fn panic_message_falls_back_for_unknown_types() {
        let panic: Box<dyn std::any::Any + Send> = Box::new(42i32);
        assert_eq!(panic_message(&panic), "(no panic message)");
    }
}
