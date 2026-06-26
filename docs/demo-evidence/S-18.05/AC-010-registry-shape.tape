# VHS Demo Tape — S-18.05 AC-010
# AC-010 (precondition 1): hooks-registry.toml postcompact-reanchor entry
#   canonical shape — event=PostCompact, on_error=continue, legacy-bash-adapter.wasm,
#   [hooks.capabilities] block with env_allow, exec_subprocess, write_file
#
# BC-7.07.002 precondition 1 / ADR-026 §Decision 7

Output docs/demo-evidence/S-18.05/AC-010-registry-shape.gif
Output docs/demo-evidence/S-18.05/AC-010-registry-shape.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 18
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 30s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.05"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1805-ac010-registry.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
