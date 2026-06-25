# Bats test suite run: 16/16 tests passing for rehydrate-wave (S-18.03)
# Supporting evidence: all ACs (001-010 plus coverage tests) green
# AC-001 through AC-010 (minus AC-005 manual) + EC-004/006 + F-P1/F-P2 robustness tests

Output docs/demo-evidence/S-18.03/bats-suite-16-16.gif
Output docs/demo-evidence/S-18.03/bats-suite-16-16.webm

Require bash
Require bats

Set Theme "Dracula"
Set Width 1200
Set Height 800
Set Padding 20
Set FontSize 13

Type "# rehydrate-wave bats suite — 16/16 tests green"
Enter
Sleep 400ms
Type "# Covers AC-001..AC-010 (AC-005 is manual) + EC-004/006 + F-P1/F-P2"
Enter
Sleep 600ms
Type ""
Enter
Sleep 200ms

Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.03/plugins/vsdd-factory/tests && bats rehydrate-wave.bats"
Enter
Sleep 35s
