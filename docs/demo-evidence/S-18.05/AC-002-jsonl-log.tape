# VHS Demo Tape — S-18.05 AC-002
# AC-002 (postcondition 2): JSONL log entry with exactly 6 fields — event, current_cycle,
#   current_step, develop_sha, timestamp, status; no wave_id; develop_sha from git rev-parse
#
# BC-7.07.002 postcondition 2 / VP-089 §2

Output docs/demo-evidence/S-18.05/AC-002-jsonl-log.gif
Output docs/demo-evidence/S-18.05/AC-002-jsonl-log.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 18
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 30s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.05"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1805-ac002-log.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
