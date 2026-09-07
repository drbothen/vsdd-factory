# VHS Demo Tape — S-25.02 Phase F4 cluster-1 "cap+trigger"
# EC-014 / AC-005 (BC-1.18.005 Postcondition 8 missing-file/first-write):
#   a "frontmatter-changelog-array"-shaped target does not yet exist on
#   disk -- a legitimate first-ever Write CREATING it.
#   read_changelog_item_count maps io::ErrorKind::NotFound -> Ok(0), so
#   current_item_count + 1 = 1 <= N=50 -> Continue. The legitimate create
#   is NEVER hard-blocked as HookResult::Error.
#
# NOTE: fixed Sleep durations used instead of Wait+Line — see
# AC-002a-under-cap-write-continue.tape's header note for why.

# Output paths are relative to THIS file's own directory — run vhs from
# inside docs/demo-evidence/S-25.02/cluster-1-cap-trigger/ (see README.md's
# Reproduction section). This avoids the doubled-path bug that results from
# running vhs from a different cwd while a tape carries a full repo-relative
# Output path.
Output EC-014-missing-changelog-first-write.gif
Output EC-014-missing-changelog-first-write.webm

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

Type "echo 'BC-1.18.005 EC-014: BC-INDEX.md does not exist yet -- first-ever Write CREATES it'"
Enter
Sleep 1.5s

Type "sed -n '1597,1610p' crates/factory-dispatcher/src/shard_manager.rs"
Enter
Sleep 2.5s

Type "cargo test -p factory-dispatcher --lib shard_manager::tests::test_BC_1_18_005_EC_014_shard_cap_gate_check_create_path_missing_file_continues -- --exact --nocapture"
Enter
Sleep 5s

Sleep 2s
