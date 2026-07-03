# VHS Demo Tape — S-18.10 EC-011 (malformed JSON -> ADVISORY parse error)
# EC-011 (BC-6.25.001): malformed JSON in settings.json -> ADVISORY with jq parse error detail
# jq parse error captured and shown; helper exits 0 (advisory-only, never blocks).
# Also exercises AC-005 (exit 0) and AC-007 (row always emitted).

Output docs/demo-evidence/S-18.10/EC-011-advisory-malformed-json.gif
Output docs/demo-evidence/S-18.10/EC-011-advisory-malformed-json.webm

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

Type "# EC-011: malformed JSON -> ADVISORY with jq parse error; exit 0"
Enter
Type "bash /tmp/s1810-run-malform.sh"
Enter
Sleep 3s
