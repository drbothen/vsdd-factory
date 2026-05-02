# BONUS: macOS /var/folders/ Symlink Fix + W-15 Finale

**Story:** S-8.09  
**Status:** SHIPPED

## macOS /var/folders/ Symlink Fix

macOS `TMPDIR` resolves through `/var/folders/...` which is actually a symlink to
`/private/var/folders/...`. The factory-dispatcher's `write_file` path-allow check
uses `canonicalize()`, which follows symlinks and resolves to `/private/var/...`.

The path-allow list in the registry uses the non-canonicalized form. The fix in
`crates/factory-dispatcher/src/invoke.rs` adds an ancestor fallback: when the
canonicalized path is under `/private/`, the dispatcher also checks the unresolved
(non-canonicalized) form for the `path_allow` whitelist match.

This fix enables `regression-gate` (and all other WASM hooks that write state files)
to work correctly in macOS bats tests where `BATS_TEST_TMPDIR` lives under `/var/folders/`.

## W-15 Finale Summary

With S-8.09 merged:

- **9/9 Tier 1 hooks** now run as native WASM (not legacy-bash-adapter)
- **0 Tier 1 entries** reference `legacy-bash-adapter.wasm`
- **adapter retirement (S-8.29)** is now READY to be scheduled for W-17
- **bin/emit-event** remains (D-10 deferral to S-8.29)
- **S-8.11 wave gate** is UNBLOCKED

## Recordings

- [BONUS-macos-varfolders-fix.gif](BONUS-macos-varfolders-fix.gif)
- [BONUS-macos-varfolders-fix.webm](BONUS-macos-varfolders-fix.webm)
- [BONUS-macos-varfolders-fix.tape](BONUS-macos-varfolders-fix.tape)
