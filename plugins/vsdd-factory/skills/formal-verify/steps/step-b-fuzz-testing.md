---
name: step-b-fuzz-testing
description: Create and run fuzz targets for parsers, deserializers, state machines, and API handlers.
---

# Step B: Fuzz Testing

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains templates and prerequisites.

## Procedure

1. Identify fuzz targets:
   - Parser inputs (any function that accepts `&str` or `&[u8]`)
   - Deserialization functions
   - State machine transitions
   - API request handlers

2. Create fuzz targets in `fuzz/fuzz_targets/`:

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = parse_function(input);
    }
});
```

3. Run each target for at least 5 minutes:
```bash
cargo fuzz run <target> -- -max_total_time=300
```

## Artifacts

- Fuzz targets in `fuzz/fuzz_targets/`
- Fuzz Results section in `.factory/cycles/<current>/formal-verification-report.md`
- Any crash artifacts in `fuzz/artifacts/`

## Success Criteria

- All identified parser/deserializer/handler functions have fuzz targets
- Each target runs for ≥5 minutes
- Zero crashes (or crashes documented with reproduction steps)
