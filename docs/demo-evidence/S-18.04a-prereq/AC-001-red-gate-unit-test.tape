Output AC-001-red-gate-unit-test.gif
Output AC-001-red-gate-unit-test.webm

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set Padding 24
Set TypingSpeed 40ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04a-prereq"
Enter
Sleep 1s
Show

Type "cargo test -p factory-dispatcher test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root 2>&1 | tail -12"
Enter
Sleep 6s
