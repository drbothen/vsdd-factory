# AC-002 Evidence — .factory/ present + file absent: create with header then append marker

**Story:** S-8.06 — Native port: session-learning (Stop)
**BC trace:** BC-7.03.077 postcondition 1 (appends timestamped marker to .factory/sidecar-learning.md)

---

## Claim

When `.factory/` directory exists and `sidecar-learning.md` does not exist: WASM plugin
creates `.factory/sidecar-learning.md` with the exact header block (trailing blank line
required for bats byte-identical comparison), then appends one marker line
`- Session ended at <ISO8601-UTC> (awaiting /session-review)`. Exits 0.

---

## Implementation excerpt (crates/hook-plugins/session-learning/src/lib.rs)

### Header constant (line 32-33)

```rust
pub const SIDECAR_HEADER: &str =
    "# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n\n";
```

The double `\n\n` at the end produces the required trailing blank line. This is
byte-identical to the bash source output verified by bats case 1.

### File creation path (lines 90-100)

```rust
if !sidecar_path.exists() {
    // Create file and write header (byte-identical to bash source output, AC-002).
    match std::fs::write(&sidecar_path, SIDECAR_HEADER) {
        Ok(_) => {}
        Err(e) => {
            return HookResult::error(format!(
                "session-learning: failed to create sidecar-learning.md: {e}"
            ));
        }
    }
}
```

### Timestamp and append (lines 104-117)

```rust
let ts = now_fn();
let marker = MARKER_FORMAT.replace("{}", &ts);

match OpenOptions::new().append(true).open(&sidecar_path) {
    Ok(mut file) => match file.write_all(marker.as_bytes()) {
        Ok(_) => HookResult::Continue,
        ...
    },
    ...
}
```

`format_utc_now()` produces `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()`.

---

## Bats test output — Case 1 (AC-002)

```
ok 1 T5-case1 (AC-002): first invocation creates file with header and one marker; exit 0
```

Test assertions verified:
- `[ "$status" -eq 0 ]` — plugin exits 0
- `[ -f "${SIDECAR_FILE}" ]` — file created
- `grep -c '^# Sidecar Learning$' "${SIDECAR_FILE}"` returns 1 — header present exactly once
- `grep -c '^- Session ended at ' "${SIDECAR_FILE}"` returns 1 — exactly one marker line
- ISO-8601 regex `^- Session ended at [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z \(awaiting /session-review\)$` matches

---

## Expected sidecar-learning.md content after first invocation

```
# Sidecar Learning

Session-end markers for the VSDD factory. Run /session-review to synthesize.

- Session ended at 2026-05-02T12:00:00Z (awaiting /session-review)
```

(Timestamp will vary; format is fixed to `%Y-%m-%dT%H:%M:%SZ`.)

---

**Verdict: AC-002 PASS**
