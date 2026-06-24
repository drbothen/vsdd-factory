Output docs/demo-evidence/S-18.04a/AC-017-durability-degraded.gif
Output docs/demo-evidence/S-18.04a/AC-017-durability-degraded.webm

Require git

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 900
Set Height 550
Set Theme "Dracula"
Set Padding 16
Set Framerate 24

# AC-017: DURABILITY DEGRADED advisory + exit 0 when worktree not found

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04a"
Enter
Sleep 200ms
Type "export PATH=$PWD/target/release:$PATH"
Enter
Sleep 200ms
Show

Type "bash docs/demo-evidence/S-18.04a/demo-AC017-durability-degraded.sh"
Enter
Sleep 5s
