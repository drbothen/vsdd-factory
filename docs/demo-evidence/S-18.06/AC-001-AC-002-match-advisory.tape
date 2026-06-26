# VHS Demo Tape — S-18.06 AC-001 + AC-002
# AC-001 (BC-4.15.001 PC-B-B1): stderr nudge emitted on pattern match
# AC-002 (BC-4.15.001 PC-B-B2): plugin.log structured DelegationRecommended record
#
# Command: cargo test --release --workspace (matches "cargo test --release")
# Expected: advisory to stderr + plugin.log record; exit 0 (never blocks; INV2)

Output docs/demo-evidence/S-18.06/AC-001-AC-002-match-advisory.gif
Output docs/demo-evidence/S-18.06/AC-001-AC-002-match-advisory.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 680
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 30s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.06"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1806-ac001-ac002.sh"
Enter
Sleep 8s
