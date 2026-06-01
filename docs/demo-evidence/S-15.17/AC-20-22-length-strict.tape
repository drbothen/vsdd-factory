# AC-20 + AC-22 — LENGTH=4 STRICT (inv-4): LENGTH=3 absent, LENGTH=4 present, LENGTH=5 Block
# Demonstrates: LENGTH=4 is the canonical match; LENGTH=3 and LENGTH=5 are both violations
#
# Traces to: BC-5.39.009 invariant 4; EC-018 (LENGTH=5 → Block)

Output docs/demo-evidence/S-15.17/AC-20-22-length-strict.gif
Output docs/demo-evidence/S-15.17/AC-20-22-length-strict.webm

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

Type "# AC-20/22: LENGTH=4 STRICT — LENGTH=3 absent (Block), LENGTH=4 present (Pass), LENGTH=5 Block"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/pass-length-4-present.bats validate-trajectory-tail-cell-completeness/fail-length-3-absent.bats validate-trajectory-tail-cell-completeness/fail-state-length-5-block.bats"
Enter
Sleep 10s
