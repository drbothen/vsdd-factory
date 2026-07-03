# AC-BUILD: Build validate-wave-handoff-completeness WASM artifact
# Story S-18.02 — validate-wave-handoff-completeness WASM PostToolUse gate
# Shows: cargo build --target wasm32-wasip1 --release (already built) and deployed .wasm size

Output docs/demo-evidence/S-18.02/AC-BUILD-wasm-artifact.gif
Output docs/demo-evidence/S-18.02/AC-BUILD-wasm-artifact.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 400
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.02 && clear"
Enter
Sleep 1s
Show

Type "cargo build --target wasm32-wasip1 --release -p validate-wave-handoff-completeness 2>&1 | tail -3"
Enter
Sleep 5s

Type "ls -lh target/wasm32-wasip1/release/validate-wave-handoff-completeness.wasm plugins/vsdd-factory/hook-plugins/validate-wave-handoff-completeness.wasm"
Enter
Sleep 2s
