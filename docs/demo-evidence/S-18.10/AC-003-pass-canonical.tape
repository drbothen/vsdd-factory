# VHS Demo Tape — S-18.10 AC-003 (canonical PASS path)
# AC-003 (BC-6.25.001 PC3): value in [1,80] -> PASS row
#   value 70 (canonical per ADR-026 §Decision 5) -> PASS
#   value 80 (boundary inclusive <=80) -> PASS
# Also exercises AC-007 (row always emitted) and AC-005 (exit 0).

Output docs/demo-evidence/S-18.10/AC-003-pass-canonical.gif
Output docs/demo-evidence/S-18.10/AC-003-pass-canonical.webm

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

Type "# AC-003 value=70 (canonical) -> PASS"
Enter
Type "bash /tmp/s1810-run-70.sh"
Enter
Sleep 3s

Type "# AC-003 boundary value=80 (<=80) -> PASS"
Enter
Type "bash /tmp/s1810-run-80.sh"
Enter
Sleep 3s
