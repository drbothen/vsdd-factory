# VHS Demo Tape — S-18.10 AC-005 + jq-absent degradation
# AC-005 (BC-6.25.001 INV1 PC5): always exits 0; never blocks
# jq-absent path: PATH=/bin hides jq (/usr/bin and /opt/homebrew/bin excluded)
# helper emits graceful ADVISORY and exits 0 (INV1: advisory-only, never fatal)

Output docs/demo-evidence/S-18.10/AC-005-jq-absent-graceful.gif
Output docs/demo-evidence/S-18.10/AC-005-jq-absent-graceful.webm

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

Type "# AC-005/INV1/jq-absent: jq hidden from PATH -> graceful ADVISORY; exit 0"
Enter
Type "bash /tmp/s1810-run-nojq.sh"
Enter
Sleep 3s
