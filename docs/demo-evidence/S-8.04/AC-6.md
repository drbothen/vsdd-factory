# AC-006: Bats parity tests

**Criterion:** Bats parity tests cover all 6 cases from AC-006 plus edge cases.
All 20 bats tests pass (8 grep-gate + 4 fixture + 8 dispatcher parity).

**Trace:** BC-7.03.084 + BC-7.03.085 postcondition 1.

---

## Test Suite Layout

File: `tests/integration/E-8-hook-plugins/update-wave-state-on-merge.bats`

### Grep-gate tests (source-level assertions)

| Test | Assertion | AC |
|------|-----------|-----|
| grep-1 | `lib.rs` uses `Regex::new` (not stub `false`) | AC-003 |
| grep-2 | `lib.rs` uses `serde_yaml::from_str` (not stub NoOp) | AC-004 |
| grep-3 | `main.rs` calls `host::write_file` | AC-004 |
| grep-4 | `main.rs` calls `host::emit_event` | AC-007 |
| grep-5 | `hooks-registry.toml` has `write_file` capability block | AC-001 |
| grep-6 | `hooks-registry.toml` has native `.wasm` plugin entry | AC-001 |
| grep-7 | `update-wave-state-on-merge.sh` is deleted | AC-002 |
| grep-8 | `main.rs` emits stderr gate-transition reminder | AC-005 |

### Fixture existence tests

| Test | Fixture | Purpose |
|------|---------|---------|
| fixture-1 | `wave-state-single-story.yaml` | AC-006 case (a): single story not yet merged |
| fixture-2 | `wave-state-all-merged.yaml` | AC-006 case (b): gate flip scenario |
| fixture-3 | `wave-state-gate-null.yaml` | AC-006 case (f): YAML null gate_status |
| fixture-4 | `wave-state-gate-null.yaml` contains `gate_status: ~` | AC-005 case 2 |

### Dispatcher parity tests

| Test | Scenario | AC/EC |
|------|----------|-------|
| parity-1 | pm agent + merge signal + wave-state.yaml → story appended | AC-004, AC-006(a) |
| parity-2 | all stories merged → gate_status=pending + next_gate_required | AC-005, AC-006(b) |
| parity-3 | duplicate merge signal → YAML unchanged (checksum equal) | EC-003, AC-006(c) |
| parity-4 | wave-state.yaml absent → exit 0, no crash | EC-001, AC-006(d) |
| parity-5 | story_id not in any wave → exit 0, no mutation | EC-002, AC-006(e) |
| parity-6 | gate_status: ~ (YAML null) → flip to pending | AC-005 case 2, AC-006(f) |
| parity-7 | non-pm agent → no YAML mutation | AC-003 |
| parity-8 | pm agent, no merge signal → no YAML mutation | AC-003 |

---

## Full Test Run Output

```
1..20
ok 1 grep-1 [BC-7.03.084]: lib.rs has_merge_signal uses regex::Regex (not stub false)
ok 2 grep-2 [BC-7.03.085]: lib.rs process_wave_state uses serde_yaml::from_str
ok 3 grep-3 [BC-7.03.085 AC-004]: main.rs write_yaml closure calls host::write_file
ok 4 grep-4 [BC-7.03.083 AC-007]: main.rs emit closure calls host::emit_event
ok 5 grep-5 [BC-7.03.083 AC-001]: hooks-registry.toml has write_file capability block
ok 6 grep-6 [BC-7.03.083 AC-001]: hooks-registry.toml has native WASM plugin entry
ok 7 grep-7 [BC-7.03.083 AC-002]: update-wave-state-on-merge.sh is deleted
ok 8 grep-8 [BC-7.03.086 AC-005]: main.rs emits stderr gate-transition reminder
ok 9 fixture: wave-state-single-story.yaml exists
ok 10 fixture: wave-state-all-merged.yaml exists
ok 11 fixture: wave-state-gate-null.yaml exists (AC-006 null gate_status fixture)
ok 12 fixture: wave-state-gate-null.yaml contains gate_status: ~ (YAML null)
ok 13 parity-1 [BC-7.03.085 AC-004 AC-006a]: pm agent + merge signal -> story appended to stories_merged
ok 14 parity-2 [BC-7.03.086 AC-005 AC-006b]: all stories merged -> gate_status=pending + next_gate_required
ok 15 parity-3 [EC-003 AC-006c]: duplicate merge signal -> no change to stories_merged
ok 16 parity-4 [EC-001 AC-006d]: wave-state.yaml absent -> exit 0 (no crash)
ok 17 parity-5 [EC-002 AC-006e]: story not in any wave -> exit 0 no mutation
ok 18 parity-6 [BC-7.03.086 AC-005 AC-006f]: gate_status=null (YAML ~) -> flip to pending
ok 19 parity-7 [BC-7.03.084 AC-003]: non-pm agent -> exit 0 no YAML mutation
ok 20 parity-8 [BC-7.03.084 AC-003]: pm agent no merge signal -> exit 0 no mutation
```

20/20 tests pass.

---

## wave-state-gate-null.yaml Fixture (verbatim)

```yaml
waves:
  - wave: "wave-14"
    stories: ["S-8.04"]
    stories_merged: []
    gate_status: ~
    current_wave: null
    next_gate_required: null
```

`gate_status: ~` exercises AC-005 truth table case 2 (key present, YAML null) →
deserializes as `None` → triggers gate flip to "pending".

---

## Recording

![AC-006 demo](AC-006-bats-parity-tests.gif)

**Status: PASS**
