# VHS Demo Tape — S-18.10 AC-001 (key absent -> ADVISORY)
# AC-001 (BC-6.25.001 PC1): CLAUDE_AUTOCOMPACT_PCT_OVERRIDE absent -> ADVISORY with remediation hint
# Fixture: env block present but key absent: {"env": {}}
# Also exercises AC-007 (row always emitted) and AC-005 (exit 0, non-blocking).

Output docs/demo-evidence/S-18.10/AC-001-advisory-key-absent.gif
Output docs/demo-evidence/S-18.10/AC-001-advisory-key-absent.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 480
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 30s

Hide
Type "PS1='demo $ ' && cd /tmp"
Enter
Sleep 0.5s
Show

Type "# AC-001: env block present but key absent -> ADVISORY + remediation hint"
Enter
Type "bash /tmp/s1810-run-nokey.sh"
Enter
Sleep 3s
