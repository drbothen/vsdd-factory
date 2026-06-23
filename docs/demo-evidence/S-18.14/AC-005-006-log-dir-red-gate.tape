# S-18.14 AC-005/AC-006 — dispatcher.started log_dir absolutize Red Gate test
# RG-005: test_BC_1_13_001_dispatcher_started_event_log_dir_absolutized_at_emission
#
# RED before fix: no log_dir field in dispatcher.started payload at all
# GREEN after fix: log_dir present, non-empty, and is_absolute() == true
# Discrimination: test constructs InternalLog with RELATIVE log_dir under controlled CWD;
# pre-fix emits relative path (fails is_absolute()), post-fix absolutizes at emission site.

Output docs/demo-evidence/S-18.14/AC-005-006-log-dir-red-gate.gif
Output docs/demo-evidence/S-18.14/AC-005-006-log-dir-red-gate.webm

Require cargo

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Dracula"
Set Padding 20
Set Margin 10
Set Shell "zsh"

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.14" Enter
Sleep 2s
Type "clear" Enter
Sleep 1s
Show

Type "# S-18.14: log_dir observability — Red Gate test RG-005" Enter
Sleep 1s

Type "cargo test -p factory-dispatcher 'red_gate_s18_14_log_dir' 2>&1 | grep -E '(test BC|test result)'" Enter
Sleep 8s
