# AC-001 — emit-handoff subcommand dispatch + no bash redirect
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 PC10 — four-step agent-orchestrated flow; Write tool mandate
# Oracle: (a) } > "$output_path" absent from lib/write-handoff.sh
#         (b) --emit-handoff dispatch arm present in wave-handoff.sh

Output docs/demo-evidence/S-18.13/AC-001-emit-handoff-subcommand-dispatch.gif
Output docs/demo-evidence/S-18.13/AC-001-emit-handoff-subcommand-dispatch.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 13
Set Width 1200
Set Height 500
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.13 && clear"
Enter
Sleep 1s
Show

Type "# AC-001: verify --emit-handoff arm present + no bash redirect"
Enter
Sleep 0.5s

Type "grep -c -- '--emit-handoff' plugins/vsdd-factory/skills/wave-handoff/wave-handoff.sh"
Enter
Sleep 2s

Type "cd plugins/vsdd-factory/tests && bats --filter 'S18_13_AC001' wave-handoff.bats"
Enter
Sleep 10s

Sleep 2s
