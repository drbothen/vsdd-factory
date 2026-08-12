---
document_type: formal-verification-artifact
level: ops
title: "S-21.09 Gate — Mutation-Completeness Audit: Survivor Catalog"
story_id: "S-21.09"
producer: formal-verifier
consumer: state-manager
phase: S-21.09-EXHAUSTIVE-MUTATION-AUDIT-HARDENING-BURST
date: 2026-08-12
scope: >
  crates/factory-dispatcher/tests/bundle_orphan_check.rs (manual determinant sweep,
  gate-logic test-file functions) + crates/factory-dispatcher/src/registry.rs
  (cargo-mutants, production code).
disposition: >
  68 manual determinants + 23 cargo-mutants mutants. 64 manual KILLED, 4 manual
  SURVIVORS (SURV-01..04) + 18 cargo-mutants caught, 4 unviable, 1 MISSED
  (SURV-05). Combined actionable: 4 killable survivors CLOSED this burst
  (SURV-02/03/04/05 via T-054/T-055/T-056 + registry.rs unit test), 1
  provably un-isolatable accepted-residual (SURV-01). Zero killable
  surviving mutants remain in audited scope.
closes:
  - SURV-02
  - SURV-03
  - SURV-04
  - SURV-05
accepted_residual:
  - SURV-01
---

# S-21.09 Gate — Mutation-Completeness Audit: Survivor Catalog

## Method (empirical, actually executed)
Baseline: 48/48 green. Each mutation applied to a scratch copy of crates/factory-dispatcher/tests/bundle_orphan_check.rs, the suite (cargo test -p factory-dispatcher --test bundle_orphan_check) run to completion, pass/fail + killing-test recorded, file reverted. Two manual batches ran 68 determinant mutations across every class. cargo-mutants ran separately on production crates/factory-dispatcher/src/registry.rs.

## Totals
- Manual (test-file gate functions): 68 determinants tested → 64 KILLED, 4 SURVIVORS.
- cargo-mutants (production registry.rs): 23 mutants → 18 caught, 4 unviable, 1 MISSED.
- Combined actionable survivors: 4 in-gate (SURV-01..04) + 1 production accessor (SURV-05). Of the 4 in-gate: 2 provably dead/un-isolatable (accepted-residual), 1 unreachable-first (low), 1 genuine fail-closed correctness gap (SURV-04).

## SURVIVORS

### SURV-01 — lex_norm, RootDir/Prefix arm parts.clear()
Anchor: lex_norm, the `Component::RootDir | Component::Prefix(_) => { parts.clear(); }` arm.
Mutation: parts.clear(); → no-op.
Why it survives: a root/prefix component is always the FIRST component of an absolute path, so parts is guaranteed empty when this arm fires — clear() on an empty vec is a no-op. Doc comment already declares this a provable no-op (4th defensive unreachable arm). No input can make it observable.
Kill-spec: GENUINELY UN-ISOLATABLE / accepted-residual. Isolating clear() would require a path where a Normal component precedes a RootDir component in the same path — impossible for well-formed OS paths. No fixture can construct one. Document as accepted-residual; do NOT add a control.
Severity: None (provably dead defensive code).

### SURV-02 — lex_norm, CurDir arm Component::CurDir => {}
Anchor: lex_norm, the `Component::CurDir => {}` arm (skip .).
Mutation: `Component::CurDir => {}` → `Component::CurDir => { parts.push(".".to_owned()); }`.
Why it survives: Rust's std::path::Components normalizes interior . away before lex_norm's match sees it, yielding a CurDir component only for a leading . on a relative path. Every lex_norm call site passes an absolute path (registry_parent.join(plugin_path); root; registry_parent), so any ./ is interior and pre-stripped. The CurDir arm never executes in the gate.
Kill-spec: effectively un-isolatable in the gate path (dead: std normalization + absolute-only call sites). If desired, a direct unit test on a relative path exercises it: assert_eq!(lex_norm(Path::new("./a/b")), vec!["a","b"]) — live ["a","b"], mutant [".","a","b"]. Recommend accepted-residual, optionally the cheap direct unit test.
Severity: test-robustness-only (dead in gate path).

### SURV-03 — detect_ungated_declarations, TOML-parse-error arm
Anchor: detect_ungated_declarations, the content.parse::<toml::Value>() match: `Err(_) => return Vec::new()`.
Mutation: `Err(_) => return Vec::new()` → return a finding.
Why it survives: in run_t012_gate, both Registry::parse_str(&hooks_content) and parse_plugin_refs() parse the same file with the same toml crate and panic on malformed TOML BEFORE detect_ungated_declarations is reached. The only direct-caller test (T-048) always writes valid TOML. So this fail-open branch is never exercised → its return value unobservable.
Kill-spec: add a direct unit test calling detect_ungated_declarations(&registry, root) where registry contains invalid TOML; assert .is_empty() (live []; fail-open mutant returns spurious finding). Dominated by upstream panics so accepted-residual is defensible — but the control is one cheap direct test and worth it.
Severity: test-robustness-only in current wiring (unreachable-first). Would become correctness (silent fail-OPEN) if call order ever changed.

### SURV-04 — run_t012_gate, resolvers schema_version sentinel .unwrap_or(-1)  ⚠️ the one that matters
Anchor: run_t012_gate, resolvers schema-version read: resolvers_doc.get("schema_version").and_then(|v| v.as_integer()).unwrap_or(-1) feeding assert_eq!(resolvers_schema_version, 1, …).
Mutation: .unwrap_or(-1) → .unwrap_or(1).
Why it survives: the sentinel default fires only when resolvers-registry.toml has no schema_version key or a non-integer value. Every fixture reaching run_t012_gate writes an explicit integer schema_version, so .and_then(as_integer) always yields Some(n) and the unwrap_or default is never taken. Changing -1→1 is unobservable. (Sibling present-key mutation WAS killed by T-053; the absent-key sentinel is not covered.)
Kill-spec: add a fixture in the T-052/T-053 shape where resolvers-registry.toml OMITS schema_version entirely (or non-integer). Wrap #[should_panic(expected = "but production requires 1")]. Live: sentinel -1 → assert_eq!(-1,1) panics; mutant unwrap_or(1): default 1 → passes → gate proceeds → no panic → test RED. Existing T-052/T-053 stay GREEN.
Severity: CORRECTNESS / fail-closed. The -1 sentinel exists precisely to fail-closed when schema_version is absent/malformed. The mutant makes a resolvers registry with a MISSING schema_version silently PASS, even though production would reject it. Highest-priority of the four.

### SURV-05 — (production, cargo-mutants) RegistryEntry::on_error accessor default
Location: crates/factory-dispatcher/src/registry.rs — pub fn on_error(&self, defaults) -> OnError { self.on_error.unwrap_or(defaults.on_error) }.
Mutation (cargo-mutants): replace whole body with Default::default().
Why it survives: no registry.rs unit test asserts entry.on_error(defaults) returns the defaults-supplied value when the entry omits on_error distinguishably from OnError::default(). The mutant returning a fixed default is indistinguishable under current tests.
Kill-spec: in registry.rs #[cfg(test)], parse an entry with no on_error field and a RegistryDefaults whose on_error is a non-default variant, then assert entry.on_error(&defaults) == that non-default variant. Live passes; mutant fails.
Severity: test-robustness (production accessor). Out of the S-21.09 T-012 gate scope — reported for completeness; route to a registry.rs unit-test top-up. (Sibling accessors priority/timeout_ms/fuel_cap and all of validate* were CAUGHT.)

## Already-closed controls — confirmation
T-050 (length conjunct) sole killer of DUD-inrepo-ge (>→>=, M2). T-051 (prefix conjunct) sole killer of DUD-all-any and DUD-all-true. T-052 (hooks production validation) sole killer of RUN-parsestr-delete. T-053 (resolvers schema assert) sole killer of RUN-resolvers-assert-delete AND RUN-schema-asint-const1. Closure is broad: floor boundary <30 pinned both directions (T-027 lower; T-015/T-016/T-020/T-021 exactly-30 upper); extract_hook_plugin_name gate-1 length (T-033/T-032/T-039/T-048); gate-2 prefix (!=→==); gate-3 case-insensitivity (T-031); full-join vs .last() (T-032/T-039); all classification/message literals; inventory glob narrowing; check_declared_subset_tracked booleans; run_t012_gate wiring; git plumbing (ls-tree -r removal T-034, ls-files pathspec, .wasm filter, workspace_root marker); collect_orphans dual/hooks-only; lex_norm parts.pop() removal/double-pop (T-023/T-025/T-046/T-048).

## Genuinely un-isolatable / dead (accepted-residual)
SURV-01 (parts.clear() on RootDir): provably a no-op; accept with existing doc rationale. SURV-02 (CurDir arm): dead in gate; accept or pin via direct relative-path lex_norm unit test. SURV-03 (detect TOML-parse-error arm): unreachable-first; accept or add one cheap direct unit test.

## Disposition (single hardening burst)
1. SURV-04 (must-close, correctness): new should_panic fixture with resolvers schema_version omitted/non-integer.
2. SURV-03 (cheap control): direct unit test — detect_ungated_declarations on malformed TOML asserts [].
3. SURV-01/SURV-02 (accept-residual): document dead defensive arms (SURV-02 optionally pinned by direct relative-path lex_norm unit test).
4. SURV-05 (out-of-gate, optional): registry.rs unit test on RegistryEntry::on_error default-fallback.
After adding controls, re-running the 68-mutation sweep should show 0 killable survivors (only accepted-residual dead arms remain), unblocking the 3-CLEAN cascade.

Final state: git status --porcelain empty (CLEAN); all mutations reverted; mutants.out/ removed; suite 48/48 green (at audit time). No commits, no pushes, no story/STATE/index edits.

---

## Burst Disposition Record (state-manager, factory-artifacts persistence)

This audit was persisted verbatim above as the formal-verification artifact of record for
the S-21.09 EXHAUSTIVE MUTATION-AUDIT HARDENING BURST. The burst that followed this audit
closed all 4 killable survivors:

- **SURV-04** — CLOSED via **T-054** (`test_S_21_09_ac006_T054_resolvers_schema_version_absent_key_fail_closed_sentinel`), `#[should_panic(expected = "but production requires 1")]`, resolvers-registry.toml with `schema_version` key omitted entirely.
- **SURV-03** — CLOSED via **T-055** (`test_S_21_09_ac006_T055_detect_ungated_declarations_malformed_toml_fail_open_arm`), direct unit test of `detect_ungated_declarations` on malformed TOML, asserts `.is_empty()`.
- **SURV-02** — CLOSED via **T-056** (`test_S_21_09_ac006_T056_lex_norm_curdir_arm_direct_contract_pin`), direct unit test `lex_norm(Path::new("./a/b")) == ["a", "b"]`.
- **SURV-05** — CLOSED via a new `registry.rs` unit test `on_error_falls_back_to_registry_defaults_when_entry_omits_it` (pairs an omitted entry-level `on_error` with a non-default `RegistryDefaults`).
- **SURV-01** — ACCEPTED-RESIDUAL, no test added; closed via a strengthened doc comment on `lex_norm` recording the provable-no-op rationale.

Resulting suite: **51 tests T-006..T-056** (45 S-21.09-owned, T-012..T-056) plus 1
`registry.rs` unit test, all green at `feature/S-21.09` HEAD **`b761477f`** (test-writer
commit, NOT pushed). `cargo fmt --check --all`, `cargo clippy --workspace --all-targets --
-D warnings`, `cargo test --workspace --all-targets` (189 workspace tests) all clean.
Story spec **v1.26** (story-writer) documents this audit + closure under a new "Mutation
Completeness Audit" subsection of AC-006.

**This was NOT an adversary pass.** The LOCAL BC-5.39.001 3-CLEAN streak remains **0/3
UNCHANGED** (14 passes, zero CLEAN, no adversary review ran this burst). LOCAL adversary
pass-15 is the immediate NEXT step, now dispatched against a suite with zero killable
surviving mutants across the audited scope — expected to have a materially improved
chance of a CLEAN verdict on the mutation-completeness axis specifically, though pass-10
carry-over findings (MED-001, LOW-001/002/003) remain OPEN and unaddressed by this burst.
