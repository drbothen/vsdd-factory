# VHS Demo Tape — S-18.10 AC-006 (non-numeric value -> treated as absent)
# AC-006 (BC-6.25.001 INV3): non-numeric values treated as absent -> ADVISORY with INV3 note
# value "auto" -> ADVISORY with "is not a valid integer; treating as absent"
# Also exercises AC-005 (exit 0) and AC-007 (row always emitted).

Output docs/demo-evidence/S-18.10/AC-006-advisory-non-numeric.gif
Output docs/demo-evidence/S-18.10/AC-006-advisory-non-numeric.webm

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

Type "# AC-006: value='auto' (non-numeric) -> ADVISORY + INV3 treating-as-absent note"
Enter
Type "bash /tmp/s1810-run-auto.sh"
Enter
Sleep 3s
