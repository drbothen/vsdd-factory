Output docs/demo-evidence/S-8.09/AC-003-9-pattern-matching.gif
Output docs/demo-evidence/S-8.09/AC-003-9-pattern-matching.webm

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

Type "# AC-003: 9 test-runner patterns + Bash guard (BC-7.03.072)"
Enter
Sleep 800ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_072 2>&1 | grep -E 'test |ok|FAILED|result'"
Enter
Sleep 4s

Type "# Error path: non-Bash tool exits 0 immediately (EC-001)"
Enter
Sleep 800ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_071_non_bash_tool_exits_0_immediately 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s
