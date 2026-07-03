Output docs/demo-evidence/S-18.04a/AC-009-push.gif
Output docs/demo-evidence/S-18.04a/AC-009-push.webm

Require git

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 900
Set Height 500
Set Theme "Dracula"
Set Padding 16
Set Framerate 24

# AC-009: push to local bare remote succeeds; remote ref advances

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04a"
Enter
Sleep 200ms
Type "export PATH=$PWD/target/release:$PATH"
Enter
Sleep 200ms
Show

Type "bash docs/demo-evidence/S-18.04a/demo-AC009-push.sh"
Enter
Sleep 5s
