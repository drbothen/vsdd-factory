Output docs/demo-evidence/S-8.09/AC-006-pass-to-fail-warning.gif
Output docs/demo-evidence/S-8.09/AC-006-pass-to-fail-warning.webm

Require cargo

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 20

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.09"
Enter
Sleep 500ms
Type "clear"
Enter
Sleep 300ms
Show

Type "# AC-006: Pass-to-fail transition warning (BC-7.03.075)"
Enter
Sleep 800ms

Type "# Success: hook.block severity=warn emitted + stderr written"
Enter
Sleep 600ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_075_pass_to_fail_emits_hook_block_warn 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s

Type "# Error path: fail-to-fail must NOT emit warning"
Enter
Sleep 600ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_075_fail_to_fail_no_warning 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s

Type "# Stderr rendered newlines (0x0A) validated"
Enter
Sleep 600ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_075_stderr_message_contains_rendered_newlines 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s
