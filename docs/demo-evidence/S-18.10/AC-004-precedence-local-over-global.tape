# VHS Demo Tape — S-18.10 AC-004 (project-local precedence over global)
# AC-004 (BC-6.25.001 PC4 + INV2): project-local settings.json always wins over global
# Sub-case 1: both present, local=85 (ADVISORY) wins; global=70 not consulted
# Sub-case 2: local absent -> global=70 (PASS) used as fallback

Output docs/demo-evidence/S-18.10/AC-004-precedence-local-over-global.gif
Output docs/demo-evidence/S-18.10/AC-004-precedence-local-over-global.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 560
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

Type "# AC-004 case 1: both present, local=85 wins (ADVISORY); global=70 ignored"
Enter
Type "bash /tmp/s1810-run-prec-local.sh"
Enter
Sleep 3s

Type "# AC-004 case 2: local absent -> global fallback=70 (PASS)"
Enter
Type "bash /tmp/s1810-run-prec-global.sh"
Enter
Sleep 3s
