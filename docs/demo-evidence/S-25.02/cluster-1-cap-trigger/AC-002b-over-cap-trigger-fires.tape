# VHS Demo Tape — S-25.02 Phase F4 cluster-1 "cap+trigger"
# AC-002 (over-cap leg): Write, projected_size = 50,000 bytes,
#   shard_cap_bytes = 49,152 -> trigger FIRES. This BC owns the
#   trigger-boundary DECISION only (Postcondition 3 "Ownership" bullet) --
#   the roll/block outcome is BC-1.18.006's (later cluster, not yet landed),
#   so a fired trigger surfaces as a non-fatal tracing::warn! advisory and
#   still returns HookResult::Continue (honest interim hand-off, never a
#   fabricated Block).
#
# NOTE: fixed Sleep durations used instead of Wait+Line — see
# AC-002a-under-cap-write-continue.tape's header note for why.

# Output paths are relative to THIS file's own directory — run vhs from
# inside docs/demo-evidence/S-25.02/cluster-1-cap-trigger/ (see README.md's
# Reproduction section). This avoids the doubled-path bug that results from
# running vhs from a different cwd while a tape carries a full repo-relative
# Output path.
Output AC-002b-over-cap-trigger-fires.gif
Output AC-002b-over-cap-trigger-fires.webm

Require cargo

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 18
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd $(git rev-parse --show-toplevel) && clear"
Enter
Sleep 1.5s
Show

Type "echo 'BC-1.18.005 AC-002 (over-cap leg): Write content=50,000B, cap=49,152B -> trigger fires'"
Enter
Sleep 1.5s

Type "sed -n '1062,1072p' crates/factory-dispatcher/src/shard_manager.rs"
Enter
Sleep 2.5s

Type "cargo test -p factory-dispatcher --lib shard_manager::tests::test_BC_1_18_005_vector_write_50000_bytes_over_cap_triggers -- --exact --nocapture"
Enter
Sleep 5s

Sleep 2s
