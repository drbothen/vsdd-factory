# AC-26 — LENGTH=4 marker + LENGTH=5 prose coexist → PASS (locks inv-4 marker-prefix scoping)
# Demonstrates: trajectory-tail marker scopes count to segment before first semicolon;
#               a LENGTH=5 prose trajectory elsewhere in the cell is ignored → scoped count=4 → PASS
#
# Traces to: BC-5.39.009 EC-022; invariant 4

Output docs/demo-evidence/S-15.17/AC-26-length4-marker-length5-prose.gif
Output docs/demo-evidence/S-15.17/AC-26-length4-marker-length5-prose.webm

Set Shell "zsh"
Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 18
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-15.17/plugins/vsdd-factory/tests"
Enter
Sleep 2s
Type "clear"
Enter
Sleep 1s
Show

Type "# AC-26: LENGTH=4 trajectory-tail marker + LENGTH=5 prose → PASS (inv-4 marker-prefix scoping)"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/pass-length4-marker-with-length5-prose.bats"
Enter
Sleep 8s
