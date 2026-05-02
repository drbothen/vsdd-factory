# AC-5: Dispatcher integration tests — 5 capability scenarios

**Criterion:** `crates/factory-dispatcher/src/host/write_file.rs` unit tests in the
`prepare()` function cover the 5 required scenarios (parallel to `read_file.rs`).

**Trace:** BC-2.02.011 postcondition 4 (timeout behavior and dispatcher test coverage).

---

## Test Results (`cargo test --package factory-dispatcher`)

All 5 `prepare()`-level scenarios from `host::write_file::tests`:

```
test host::write_file::tests::denies_when_no_capability_block ... ok
test host::write_file::tests::writes_allowed_file ... ok
test host::write_file::tests::rejects_path_outside_allow_list ... ok
test host::write_file::tests::rejects_content_exceeding_max_bytes ... ok
test host::write_file::tests::writes_empty_contents_creates_file ... ok
test host::write_file::tests::rejects_missing_parent_directory ... ok
```

6 tests pass (5 required + 1 bonus: `writes_empty_contents_creates_file` covers EC-005).

---

## Scenario Mapping

| Scenario | Test | Result |
|----------|------|--------|
| (a) No `capabilities.write_file` block | `denies_when_no_capability_block` | ok |
| (b) Write allowed file in `path_allow` | `writes_allowed_file` | ok |
| (c) Path outside `path_allow` | `rejects_path_outside_allow_list` | ok |
| (d) `contents.len() > max_bytes` -> `OUTPUT_TOO_LARGE` | `rejects_content_exceeding_max_bytes` | ok |
| (bonus) Empty contents creates/truncates file | `writes_empty_contents_creates_file` | ok |
| (e) Missing parent directory -> `INTERNAL_ERROR` | `rejects_missing_parent_directory` | ok |

---

## Key Assertions

### (a) Deny without capability

```rust
fn denies_when_no_capability_block() {
    let ctx = bare_context();
    let err = prepare(&ctx, "out.txt", b"data", 1024).unwrap_err();
    assert_eq!(err, codes::CAPABILITY_DENIED);
}
```

### (b) Write allowed file

```rust
fn writes_allowed_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("ok.txt");
    let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
    ctx.plugin_root = dir.path().to_path_buf();
    prepare(&ctx, file.to_str().unwrap(), b"hello", 1024).unwrap();
    assert_eq!(std::fs::read(&file).unwrap(), b"hello");
}
```

### (d) Byte cap — no bytes written on rejection

```rust
fn rejects_content_exceeding_max_bytes() {
    // ...
    let data = vec![0u8; 2048];
    let err = prepare(&ctx, file.to_str().unwrap(), &data, 512).unwrap_err();
    assert_eq!(err, codes::OUTPUT_TOO_LARGE);
    // BC-2.02.011 postcondition 2: no bytes written to disk.
    assert!(!file.exists());
}
```

---

## `allow_write` Test Helper

`test_support::allow_write(paths: &[&str]) -> Capabilities` added to
`crates/factory-dispatcher/src/host/mod.rs` test_support module (parallel to
`allow_read` at line 215), used by all 6 tests above.

**Status: PASS**
