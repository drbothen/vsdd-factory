# VHS Demo Tape — S-18.11 RG-3 + RG-6 (allowlist success/error paths)
# RG-3 (test_consumer_accepts_partial_status): SUCCESS PATH —
#   partial IS in the canonical 8-value enum (BC-5.41.004 INV-1);
#   consumer must NOT reject it. CLASSIFY_RESULT=has-next-wave (draft present).
#   partial story appears in BROKEN_STORY_IDS (non-terminal, non-next-wave).
# RG-6 (test_consumer_rejects_complete_status): ERROR PATH —
#   complete is NOT in the 8-value enum; consumer MUST exit non-zero with
#   "unknown story status 'complete'" in stderr (BC-5.41.004 INV-1 / EC-007).

Output docs/demo-evidence/S-18.11/RG-3-RG6-allowlist-partial-accepted-complete-rejected.gif
Output docs/demo-evidence/S-18.11/RG-3-RG6-allowlist-partial-accepted-complete-rejected.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 560
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms
Set WaitTimeout 30s

Hide
Type "PS1='demo $ '"
Enter
Sleep 0.3s
Show

Type "# RG-3 (BC-5.41.004 INV-1): partial accepted in allowlist — CLASSIFY_RESULT=has-next-wave"
Enter
Type "bats --filter test_consumer_accepts_partial_status plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 6s

Type "# RG-6 (BC-5.41.004 EC-007): complete rejected — 'unknown story status complete'"
Enter
Type "bats --filter test_consumer_rejects_complete_status plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 6s
Type "echo exit:$?"
Enter
Sleep 1s
