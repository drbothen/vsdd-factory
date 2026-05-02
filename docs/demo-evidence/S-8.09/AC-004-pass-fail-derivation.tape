Output docs/demo-evidence/S-8.09/AC-004-pass-fail-derivation.gif
Output docs/demo-evidence/S-8.09/AC-004-pass-fail-derivation.webm

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

Type "# AC-004: Pass/fail derivation cascade (BC-7.03.073)"
Enter
Sleep 800ms

Type "# 5-step cascade: exit_code priority over interrupted; unknown=skip"
Enter
Sleep 600ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_073 2>&1 | grep -E 'test |ok|FAILED|result'"
Enter
Sleep 4s

Type "# Error path: no exit_code, no interrupted => STATUS=unknown, no state write"
Enter
Sleep 800ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_073_unknown_status_no_state_write 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s
