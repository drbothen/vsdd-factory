# AC-007: host::emit_event replaces bin/emit-event; bin/emit-event not removed; perf gate excluded

**Criterion:** `bin/emit-event` calls replaced with `vsdd_hook_sdk::host::emit_event`.
No reference to `bin/emit-event` in the WASM crate. `bin/emit-event` binary NOT removed
(E-8 D-10). Perf gate excluded for this Tier 1 YAML-I/O hook.

**Trace:** BC-7.03.083 postcondition 1 (emit_event host fn).

---

## emit_event Usage (main.rs)

The production code path (non-standalone) calls `vsdd_hook_sdk::host::emit_event` directly:

```rust
vsdd_hook_sdk::host::emit_event(
    "hook.action",
    &[
        ("hook",              "update-wave-state-on-merge"),
        ("matcher",           "SubagentStop"),
        ("reason",            "wave_merge_recorded"),
        ("story_id",          story_id),
        ("wave",              wave.as_str()),
        ("total",             &total.to_string()),
        ("merged",            &merged.to_string()),
        ("gate_transitioned", &gate_transitioned.to_string()),
    ],
);
```

Second argument is `&[(&str, &str)]` — per T-6 call form specification.

### Error path emit_event (EC-005)

```rust
vsdd_hook_sdk::host::emit_event(
    "hook.error",
    &[
        ("hook",             "update-wave-state-on-merge"),
        ("reason",           "write_failed"),
        ("gate_transitioned","false"),
        ("error",            &format!("{e:?}")),
    ],
);
```

Structured error event fires on `write_file` failure (EC-005), distinct from the
stderr human-readable warning on the same path.

---

## Verification

### emit_event call count in main.rs

```bash
$ grep -c emit_event crates/hook-plugins/update-wave-state-on-merge/src/main.rs
3
```

Three call sites: `hook.action` (success path), `hook.error` (write failure EC-005),
plus one `host::emit_event` in the error type annotation.

### No bin/emit-event reference

```bash
$ grep bin/emit-event crates/hook-plugins/update-wave-state-on-merge/src/main.rs 2>&1
(no output — no reference to bin/emit-event)
```

### bin/emit-event binary preserved

```bash
$ ls bin/emit-event
bin/emit-event
```

Binary exists as required by E-8 D-10. It will be removed when S-8.29 lands.

---

## Perf Gate Exclusion

This hook is classified as a **YAML-I/O hook** (Tier 1 exclusion class). YAML file
read + parse + write is expected to dominate invocation time at >50% of wall time,
making comparison with the S-8.00 baseline (measured without file I/O) misleading.

Perf tracking for YAML-I/O hooks is deferred to a dedicated T-2 E-8 measurement story.
No perf gate is enforced for this story (AC-007 exclusion policy per E-8 epic).

---

## Recording

![AC-007 demo](AC-007-emit-event-no-bin.gif)

**Status: PASS**
