Output docs/demo-evidence/S-8.09/AC-005-state-file-write.gif
Output docs/demo-evidence/S-8.09/AC-005-state-file-write.webm

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

Type "# AC-005: State file written with {status, timestamp, command} (BC-7.03.074)"
Enter
Sleep 800ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_074 2>&1 | grep -E 'test |ok|FAILED|result'"
Enter
Sleep 4s

Type "# Error path: .factory/ absent guard (exits 0, no write)"
Enter
Sleep 800ms

Type "PATH=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH cargo test -p regression-gate --lib test_BC_7_03_074_factory_dir_absent 2>&1 | grep -E 'ok|FAILED'"
Enter
Sleep 3s
