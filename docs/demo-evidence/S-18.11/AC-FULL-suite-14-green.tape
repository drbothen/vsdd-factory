# VHS Demo Tape — S-18.11 HEADLINE: full suite 14/14 green
# Demonstrates: all 14 bats tests in sprint-state-format.bats pass.
# This is the primary AC-wide evidence artifact for S-18.11.
# Covers: AC-001, AC-002, AC-003, AC-004, AC-006, EC-010 (RG-8), RG-1..RG-9
# BC clauses: BC-5.41.004 PC1-6, INV-1/2/3, EC-007/010; BC-5.41.001 PC2/PC3; BC-5.41.002 PC3

Output docs/demo-evidence/S-18.11/AC-FULL-suite-14-green.gif
Output docs/demo-evidence/S-18.11/AC-FULL-suite-14-green.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms
Set WaitTimeout 30s

Hide
Type "PS1='demo $ '"
Enter
Sleep 0.3s
Show

Type "# S-18.11: sprint-state producer format — full 14-test bats suite"
Enter
Type "bats plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 15s
Type "echo exit:$?"
Enter
Sleep 1s
