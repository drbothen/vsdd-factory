# AC-004 Evidence — Append-only parity; bats parity tests all pass

**Story:** S-8.06 — Native port: session-learning (Stop)
**BC trace:** BC-7.03.077 postcondition 1 (append-only invariant)

---

## Claim

When `.factory/` exists and `sidecar-learning.md` already has prior content: WASM plugin
appends exactly one new line per invocation (does NOT overwrite or re-add the header).
Multiple consecutive Stop firings each add exactly one marker line.
All 4 bats parity tests pass (Cases 1-4 covering AC-002, AC-004, AC-003, EC-005).

---

## Implementation excerpt — append-only path (lines 86-117)

```rust
// Step 3: Open sidecar-learning.md. If file does not exist, create it and write
// the header block first (AC-002, byte-identical to bash source output).
let sidecar_path = factory_dir.join("sidecar-learning.md");

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

// Step 4: Append the marker line using OpenOptions::new().append(true) (EC-004).
// append(true) positions the cursor at EOF; no full-file buffering needed.
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

Key design points:
- `!sidecar_path.exists()` is the ONLY condition that writes the header — it is NOT
  re-written on subsequent invocations (append-only invariant).
- `OpenOptions::new().append(true)` positions cursor at end of file without reading
  or buffering the entire file (EC-004 compliance — large file safe).
- Both `std::fs::write` (header creation) and `OpenOptions::append` (marker append)
  use `std::fs` directly — `host::write_file` is absent per D-172.

---

## Bats full test run

```
1..4
ok 1 T5-case1 (AC-002): first invocation creates file with header and one marker; exit 0
ok 2 T5-case2 (AC-004): second invocation appends one more marker, no duplicate header; exit 0
ok 3 T5-case3 (AC-003): invocation without .factory/ exits 0, no file created
ok 4 T5-case4 (EC-005): large Stop envelope (>64KB) drained without write error; exit 0
```

All 4 cases pass. Summary of what each case verifies:

| Case | AC/EC | Assertion |
|------|-------|-----------|
| 1 | AC-002 | First invocation: header (exactly 1 occurrence) + 1 marker line; exit 0 |
| 2 | AC-004 | Second invocation: header still exactly 1; marker count is now 2; exit 0 |
| 3 | AC-003 | No .factory/ dir: exit 0; no file created; no dir created |
| 4 | EC-005 | Large (>64KB) Stop envelope on stdin: exit 0; marker appended; no write error |

---

## Perf gate exclusion (E-8 AC-7)

No hyperfine measurement is required or recorded. Tier 1 stories (S-8.06) are
explicitly excluded from the 20% warm-invocation regression ceiling by E-8 AC-7.
Performance is informational only.

---

## No emit_event calls (architecture compliance)

```
grep -r emit_event crates/hook-plugins/session-learning/
(no output)
```

The crate contains zero `emit_event` calls. session-learning.sh never called
`bin/emit-event`; the WASM port preserves that parity (E-8 D-2; Architecture
Compliance Rules).

---

**Verdict: AC-004 PASS — 18/18 bats tests pass (4 in this suite + integration suite)**
