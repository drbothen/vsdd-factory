# S-18.14 AC-001/AC-002/AC-003/AC-004 — Resolver WASM path fix Red Gate tests
# RG-001: relative path resolves against TOML-parent (not CWD)
# RG-002: absolute plugin path passes through unchanged
# RG-003: fail_closed:false arm also resolves against TOML-parent
# RG-006: production fixture test (wave_context resolver loads)
#
# Evidence: these 4 tests were RED before fix (resolver.load_error on each dispatch)
# and are GREEN after. Together they close the 8,560 load_error regression since rc.21.

Output docs/demo-evidence/S-18.14/AC-001-003-resolver-path-red-gate.gif
Output docs/demo-evidence/S-18.14/AC-001-003-resolver-path-red-gate.webm

Require cargo

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set Padding 20
Set Margin 10
Set Shell "zsh"

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.14" Enter
Sleep 2s
Type "clear" Enter
Sleep 1s
Show

Type "# S-18.14: Resolver WASM path fix — Red Gate tests (RG-001/002/003/006)" Enter
Sleep 1s

Type "cargo test -p factory-dispatcher 'red_gate_s18_14::test_BC_1_13_001' 2>&1 | grep -E '(test BC|test result)'" Enter
Sleep 8s
