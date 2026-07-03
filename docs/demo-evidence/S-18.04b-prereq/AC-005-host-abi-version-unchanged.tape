# AC-005-host-abi-version-unchanged.tape
# AC-005 / BC-1.16.001 PC5: HOST_ABI_VERSION remains 1.
# git_context rides in payload.extra (HashMap); no new named HookPayload field.
#
# Shows:
#   1. Source constant: pub const HOST_ABI_VERSION: u32 = 1
#   2. Dispatcher runtime summary always emits host_abi=1
#
# Traces to: AC-005 (ADR-029 §Decision 4 — no ABI bump), AC-013 (VP-093).
# Story: S-18.04b-prereq

Output docs/demo-evidence/S-18.04b-prereq/AC-005-host-abi-version-unchanged.gif
Output docs/demo-evidence/S-18.04b-prereq/AC-005-host-abi-version-unchanged.webm

Require bash
Require grep

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 30ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04b-prereq/docs/demo-evidence/S-18.04b-prereq && clear"
Enter
Sleep 2s
Show

Type "bash check-abi.sh"
Enter
Sleep 5s

Sleep 2s
