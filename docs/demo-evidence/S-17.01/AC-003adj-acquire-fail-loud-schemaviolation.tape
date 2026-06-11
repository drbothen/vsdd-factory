# VHS Demo — S-17.01 AC-003-adjacent (SchemaViolation error path)
# Demonstrates: acquire on malformed/no-frontmatter file exits non-zero + SchemaViolation
#
# Usage: vhs AC-003adj-acquire-fail-loud-schemaviolation.tape
# Output: AC-003adj-acquire-fail-loud-schemaviolation.gif + .webm

Output docs/demo-evidence/S-17.01/AC-003adj-acquire-fail-loud-schemaviolation.gif
Output docs/demo-evidence/S-17.01/AC-003adj-acquire-fail-loud-schemaviolation.webm

Require bash

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 620
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01"
Enter
Sleep 1s
Type "clear"
Enter
Sleep 500ms
Show

Type "# S-17.01 error path: acquire on malformed file — SchemaViolation + non-zero exit"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-ac003adj-demo.sh"
Enter
Sleep 5s

Sleep 2s
