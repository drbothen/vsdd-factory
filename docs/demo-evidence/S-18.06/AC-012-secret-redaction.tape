# VHS Demo Tape — S-18.06 AC-012
# AC-012 (SEC-002 / BC-4.15.001 INV5): 4-pass secret redaction in command_preview
#
# Demonstrates that secrets in a heavy-op Bash command are MASKED as ***REDACTED***
# in the plugin.log command_preview (and stderr), while the command still matches
# and the gate returns Continue (exit 0 — INV2 unaffected).
#
# 5 sub-cases:
#   1. Flag-arg secret:     grep -r . --token supersecrettoken123  → --token ***REDACTED***
#   2. Env-assignment:      API_KEY=sk-abc123 grep -r .            → API_KEY=***REDACTED***
#   3. Authorization header: grep -r . -H "Authorization: Bearer ..." → Authorization:***REDACTED***
#   4. URL credentials:     ./run-all.sh https://user:pass@example.com/db → https://***REDACTED***@...
#   5. Clean command (negative): grep -r TODO . → UNCHANGED (no over-redaction)

Output docs/demo-evidence/S-18.06/AC-012-secret-redaction.gif
Output docs/demo-evidence/S-18.06/AC-012-secret-redaction.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 900
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 60s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.06"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1806-ac012-redaction.sh"
Enter
Sleep 14s
