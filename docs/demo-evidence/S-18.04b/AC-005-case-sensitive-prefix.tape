Output AC-005-case-sensitive-prefix.gif
Output AC-005-case-sensitive-prefix.webm

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set Padding 24
Set TypingSpeed 40ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04b"
Enter
Sleep 1s
Show

Type "cargo test -p validate-burst-log --test exemption -- test_BC_5_41_003_precompact_prefix_case_sensitive test_BC_5_41_003_precompact_prefix_mixed_case_not_exempt 2>&1"
Enter
Sleep 5s
