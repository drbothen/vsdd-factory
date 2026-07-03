Output AC-004-sha-mismatch-not-exempt.gif
Output AC-004-sha-mismatch-not-exempt.webm

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

Type "cargo test -p validate-burst-log --test exemption -- test_BC_5_41_003_precompact_prefix_log_valid_sha_mismatch_not_exempt test_BC_5_41_003_chain_precompact_sha_mismatch_in_valid_log_not_exempt 2>&1"
Enter
Sleep 5s
