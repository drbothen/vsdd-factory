//! WASI command entry point for session-learning.
//!
//! session-learning does NOT parse the stdin envelope — it performs only
//! filesystem I/O (check .factory/, create/append sidecar-learning.md).
//! Per S-8.06 EC-005 and T-3, stdin MUST be drained to EOF to prevent
//! WASI SIGPIPE-equivalent failures when the dispatcher writes a large
//! Stop envelope (e.g. >64KB) to this plugin's stdin pipe.
//!
//! Unlike other hook plugins that use `vsdd_hook_sdk::__internal::run`
//! (which requires a fully-valid JSON HookPayload on stdin), session-learning
//! drains stdin manually and invokes the logic directly. This avoids
//! deserialization overhead on an envelope we never inspect, and matches the
//! bash source's behavior of ignoring stdin entirely (session-learning.sh
//! never reads stdin).
//!
//! Behavioral contracts: BC-7.03.076, BC-7.03.077, BC-7.03.078

use session_learning::{format_utc_now, session_learning_logic};
use std::io::{self, Read, Write};
use vsdd_hook_sdk::HookResult;

fn main() {
    // EC-005: drain stdin to EOF and discard.
    // This prevents WASI SIGPIPE-equivalent failures when the dispatcher is
    // still writing to our stdin pipe (e.g. large Stop envelopes >64KB).
    // We do NOT parse the payload — session-learning needs no fields from it.
    {
        let mut buf = Vec::with_capacity(4096);
        let _ = io::stdin().read_to_end(&mut buf);
        // buf intentionally dropped here — content is discarded.
    }

    // Run the hook logic with the real clock and cwd-relative .factory/ path.
    let result = session_learning_logic(format_utc_now, ".");

    // Write the result as JSON to stdout (dispatcher reads this for block detection).
    // We emit a minimal JSON string without pulling in serde_json as a dep —
    // session-learning never blocks, so the only outcomes are Continue and Error.
    let code = result.exit_code();
    let json_line: &[u8] = match &result {
        HookResult::Continue => b"{\"outcome\":\"continue\"}\n",
        HookResult::Error { message } => {
            // Build error JSON inline to avoid serde_json dependency.
            let mut out = io::stdout();
            let _ = writeln!(
                out,
                "{{\"outcome\":\"error\",\"message\":\"{}\"}}",
                message.replace('\\', "\\\\").replace('"', "\\\"")
            );
            let _ = out.flush();
            std::process::exit(code);
        }
        HookResult::Block { .. } => {
            // session-learning never blocks; this arm is unreachable in practice.
            b"{\"outcome\":\"continue\"}\n"
        }
    };

    {
        let mut stdout = io::stdout();
        let _ = stdout.write_all(json_line);
        let _ = stdout.flush();
    }

    std::process::exit(code);
}
