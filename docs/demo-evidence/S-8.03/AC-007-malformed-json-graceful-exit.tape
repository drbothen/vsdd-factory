Output docs/demo-evidence/S-8.03/AC-007-malformed-json-graceful-exit.gif
Output docs/demo-evidence/S-8.03/AC-007-malformed-json-graceful-exit.webm

Require bash

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.03"
Enter
Sleep 500ms
Type "DISPATCHER=target/release/factory-dispatcher"
Enter
Sleep 300ms
Type "PLUGIN_ROOT=plugins/vsdd-factory"
Enter
Sleep 300ms
Type "WORK=$(mktemp -d) && mkdir -p $WORK/.factory/logs"
Enter
Sleep 500ms
Show

Type "# AC-007: malformed JSON stdin -> exit 0, no panic/backtrace (BC-7.03.081 invariant)"
Enter
Sleep 500ms

Type "printf 'not-json' | env CLAUDE_PLUGIN_ROOT=$PLUGIN_ROOT CLAUDE_PROJECT_DIR=$WORK $DISPATCHER; echo exit=$?"
Enter
Sleep 2500ms

Sleep 2s
