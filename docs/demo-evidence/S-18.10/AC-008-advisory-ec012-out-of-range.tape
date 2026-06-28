# VHS Demo Tape — S-18.10 AC-008 / EC-012 (value <=0 -> distinct ADVISORY)
# AC-008 (BC-6.25.001 INV3 EC-012): value 0 -> distinct ADVISORY
# "not a valid compaction percentage (must be in range 1-100)"
# Message distinct from PC2 (>80) and PC1 (non-numeric absent) advisories.
# Also exercises AC-005 (exit 0) and AC-007 (row always emitted).

Output docs/demo-evidence/S-18.10/AC-008-advisory-ec012-out-of-range.gif
Output docs/demo-evidence/S-18.10/AC-008-advisory-ec012-out-of-range.webm

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

Type "# AC-008/EC-012: value=0 (<=0 out-of-range) -> distinct ADVISORY; exit 0"
Enter
Type "bash /tmp/s1810-run-zero.sh"
Enter
Sleep 3s
