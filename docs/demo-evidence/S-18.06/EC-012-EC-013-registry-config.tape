# VHS Demo Tape — S-18.06 EC-012 + EC-013
# EC-012 (AC-011): empty patterns config => no advisory on any Bash command
# EC-013: custom pattern "./ci.sh" => triggers DelegationRecommended advisory
#
# Demonstrates registry-config-driven behavior:
#   EC-012: patterns=[] => normally-matching command passes silently
#   EC-013: patterns=["./ci.sh"] => "./ci.sh build" triggers advisory
# Both paths exit 0 (never blocks; INV2)

Output docs/demo-evidence/S-18.06/EC-012-EC-013-registry-config.gif
Output docs/demo-evidence/S-18.06/EC-012-EC-013-registry-config.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 760
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

Type "bash /tmp/demo-s1806-ec012-ec013.sh"
Enter
Sleep 10s
