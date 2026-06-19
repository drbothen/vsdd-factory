# VHS tape: Anti-fabrication hard-block
# Demonstrates: phantom story ID not in STORY-INDEX → exit 1 + AntiFabricationFailed + clean worktree
# ACs: AC-015 (anti-fabrication preflight guard, pre-flight write prevention)
Output docs/demo-evidence/S-18.01/AC-anti-fabrication.gif
Output docs/demo-evidence/S-18.01/AC-anti-fabrication.webm

Require bash

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1280
Set Height 720
Set Theme "TokyoNight"
Set Padding 20
Set WaitTimeout 30s

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01"
Enter
Sleep 500ms
Show

Type "bash docs/demo-evidence/S-18.01/demo-anti-fabrication.sh"
Enter
Wait+Screen@15s /DONE.*AntiFabricationFailed/
Sleep 2s
