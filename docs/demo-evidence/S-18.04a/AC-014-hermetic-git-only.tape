Output docs/demo-evidence/S-18.04a/AC-014-hermetic-git-only.gif
Output docs/demo-evidence/S-18.04a/AC-014-hermetic-git-only.webm

Require git

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 900
Set Height 550
Set Theme "Dracula"
Set Padding 16
Set Framerate 24

# AC-014 / AC-001: Hermetic — only git invoked; no bash subprocess

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04a"
Enter
Sleep 200ms
Type "export PATH=$PWD/target/release:$PATH"
Enter
Sleep 200ms
Show

Type "bash docs/demo-evidence/S-18.04a/demo-AC014-hermetic-git-only.sh"
Enter
Sleep 5s
