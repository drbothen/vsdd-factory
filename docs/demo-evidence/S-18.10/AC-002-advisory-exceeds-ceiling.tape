# VHS Demo Tape — S-18.10 AC-002 (value >80 -> ADVISORY)
# AC-002 (BC-6.25.001 PC2): value exceeds ceiling 80 -> ADVISORY row
# value 85 -> ADVISORY with "exceeds ADR-026 §Decision 5 ceiling of 80"
# Also exercises AC-005 (exit 0, non-blocking) and AC-007 (row always emitted).

Output docs/demo-evidence/S-18.10/AC-002-advisory-exceeds-ceiling.gif
Output docs/demo-evidence/S-18.10/AC-002-advisory-exceeds-ceiling.webm

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

Type "# AC-002 value=85 (>80 ceiling) -> ADVISORY; exit 0 (non-blocking)"
Enter
Type "bash /tmp/s1810-run-85.sh"
Enter
Sleep 3s
