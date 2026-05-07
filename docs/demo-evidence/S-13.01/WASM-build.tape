Output docs/demo-evidence/S-13.01/WASM-build.gif
Output docs/demo-evidence/S-13.01/WASM-build.webm

Require cargo

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
Set TypingSpeed 50ms

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-13.01"
Enter
Sleep 300ms
Type "clear"
Enter
Sleep 300ms
Show

Type "# Demo: WASM — validate-artifact-path.wasm artifact exists and is correct size"
Enter
Sleep 800ms

Type "ls -lh plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm"
Enter
Sleep 1s

Type "cargo build --target wasm32-wasip1 -p validate-artifact-path --release 2>&1 | tail -3"
Enter
Sleep 30s

Sleep 2s
