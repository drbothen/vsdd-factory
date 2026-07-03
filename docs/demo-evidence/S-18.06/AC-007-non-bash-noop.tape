# VHS Demo Tape — S-18.06 AC-007 / PC-D
# AC-007 (BC-4.15.001 PC-D): non-Bash tool call no-op via registry tool filter
#
# Event: PreToolUse Write (tool_name="Write")
# Expected: plugin NOT dispatched (registry tool="Bash" filter);
#           sync_plugins=0 in dispatcher trace; no advisory; exit 0

Output docs/demo-evidence/S-18.06/AC-007-non-bash-noop.gif
Output docs/demo-evidence/S-18.06/AC-007-non-bash-noop.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 500
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

Type "bash /tmp/demo-s1806-ac007.sh"
Enter
Sleep 6s
