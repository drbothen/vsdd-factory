# O-P18-001: Audit-Event Timestamp Format — "ISO-8601 UTC" (spec) vs Local-Offset ISO-8601 (code)

Author: architect (analysis-only; no ADR/BC/code modified). Verified against `.worktrees/S-25.01` (`feature/S-25.01` @ `3919ebcb`) + ADR-048 + BC-3.08.001.

**Status:** POLICY 22 intake document — PENDING human Direction selection (A / B / hybrid). Persisted by state-manager per S-25.01 finalization-doc-sweep D-1156 (the harness blocked the architect from writing this file directly during the pass-18 burst; content persisted here faithfully, unmodified from the architect's own text). **Filename note:** the orchestrator's brief specified `analysis-O-P18-001-timestamp-utc-vs-offset.md`; this file is persisted at `O-P18-001-timestamp-utc-vs-offset-analysis.md` instead (leading `analysis-` prefix is blocked by a harness-level report-file guard; the trailing `-analysis.md` suffix form, matching the precedent `e-21-arch-delta-analysis.md`, is not). Content is otherwise verbatim.

**Origin:** S-25.01 LOCAL adversary pass 18 (D-1155) observation O-P18-001, recorded non-blocking LOW `[spec-vs-code-convention]` — does not affect S-25.01's LOCAL BC-5.39.001 3-CLEAN convergence.

---

## 1. Confirmed current behavior

The local-offset convention is dispatcher-wide (wider than the 3 named emitters). `InternalEvent::now()` (`internal_log.rs:152-155`) calls `Local::now()`; `with_ts` (`:160-166`) formats via `ts.format("%Y-%m-%dT%H:%M:%S%z")` — `%z` produces no-colon offset (e.g. `-0500`). The struct doc (`:122`) documents this as intentional ("ISO-8601 timestamp with offset"). `emit_indeterminate` (Event 8, `executor.rs:1374-1404`), `emit_marker_cleared` (Event 9, `indeterminate_marker.rs:475-511`), `emit_marker_written` (Event 10, `:542-558`) all construct via `InternalEvent::now`. Where a distinct top-level `timestamp` is required (Events 8/9 only; Event 10 has none) the code clones the already-local `ts` verbatim (`executor.rs:1388`, `indeterminate_marker.rs:485`) — `timestamp==ts` byte-identical, both local-offset, from one `Local::now()` call.

This is CRATE-WIDE: `sink-file/src/lib.rs:305,555` and `sink-otel-grpc/src/lib.rs:672` independently duplicate `Local::now().format("%Y-%m-%dT%H:%M:%S%z")` (they do not route through `InternalEvent`). `sink-file` also uses `Local::now()` for daily log-rotation (`:706,749,832`, template validation `:266`) — parallel to `InternalLog::date_stamp()` (first 10 bytes of the local `ts`) and `reconcile_raw_delete`'s own `chrono::Local::now().format("%Y-%m-%d")` (`indeterminate_marker.rs:829`, names today's log file to scan). All three date-stamps are mutually consistent only because all three use `Local` — a hidden coupling any UTC-normalization must preserve.

Same-record dual-convention: `MarkerFields.timestamp`/`.expires_at` (`executor.rs:599-612`) use `Utc::now().to_rfc3339()` (UTC, `Z`/colon, strict-RFC3339 — the only fields anything `parse_from_rfc3339`'s, `indeterminate_marker.rs:437,728`); Event 10 copies the marker's UTC `expires_at` verbatim (`:553-556`) next to a local-offset `ts` on the same JSON line.

Secondary defect: `%z` is ISO-8601 basic offset (`-0500`), not RFC3339 §5.6 extended form (`-05:00`) — valid ISO-8601, not strict RFC3339; no consumer breaks today but matters for the chosen wording.

## 2. Consumers (decisive input)

NOTHING in the codebase is sensitive to the UTC-Z-vs-local-offset FORM. `internal_log.rs` tests prefix-match the date only; `parse_expires_at` operates on the always-UTC marker field; `reconcile_raw_delete` reads `marker.written`'s `ts` as an OPAQUE string keyed on `(plugin_name, artifact_path)`; `date_stamp()` slices the first 10 bytes (format-agnostic to string form, but the calendar-date VALUE shifts if the clock basis moves local→UTC); `reconcile_raw_delete`'s independent `Local::now()` "today" (`:829`) must stay same-basis as `date_stamp()` or the scan targets the wrong file near midnight (a required sibling-sweep site under UTC-normalization); `factory-query`/`factory-report` do `jq -r` raw string interpolation, no `fromdate`/`strptime` on `.ts`; the test-suite's `parse_from_rfc3339` calls operate on marker-TOML fixtures only.

The one real sensitivity is the DAY-BOUNDARY BASIS shared between `date_stamp()` and `reconcile_raw_delete`'s "today". Forward caveat: `jq`'s `fromdate` is strict-UTC-`Z`-only and would reject the current offset form — a speculative interop argument for UTC-Z if `factory-query`/`report` ever grow date-math on `.ts`.

## 3. Directions

**Direction A (relax wording to "ISO-8601 with explicit offset"):** zero wire bytes change, zero breakage of already-shipped/audited events, immediate cross-event uniformity (matches factory-dispatcher + sink-file + sink-otel-grpc alike). NIST AU-8 contemplates a self-consistent time reference mappable to UTC — an offset-qualified ISO-8601 string is losslessly mappable, so AU-3/AU-8 are satisfied identically. Standing Rule ("spec wins") not violated: routed through POLICY 22 ratification; "ISO-8601 UTC" in the event tables reads as an unexamined copy-forward from the marker-field prose.

**Direction B (UTC-normalize emitters):** zero in-repo consumers break, but sink-file's doc names external not-yet-wired consumers (OTel filelog) — "no in-repo breakage" ≠ "no risk". True scope is undersized by "touches every emitter": `InternalEvent::now`/`with_ts` is the shared constructor for EVERY dispatcher event (`resolver.*`/`dispatcher.*`/`plugin.*`/`internal.*`); `prune_old_inner`'s `Local::now()` retention cutoff (`:510`) needs re-basing; `reconcile_raw_delete`'s `Local::now()` "today" (`:829`) is a REQUIRED (not optional) sibling-sweep — omitting it is a genuine day-boundary correctness regression in the AU-3/AU-10 reconciliation machinery; `sink-file` (6 sites) + `sink-otel-grpc` (1 site) sit outside `factory-dispatcher`. A complete Direction B is a 4-crate, ~9-callsite sweep with one non-optional correctness-sensitive site; a partial sweep is worse than the status quo (two live conventions).

## 4. Recommendation

**PRIMARY = Direction A**, with the RFC3339 colon-form fix (`%z`→`%:z`) as a SEPARABLE companion decision under the same POLICY 22 request (it too changes wire values — punctuation only — for already-shipped events). Leave marker-TOML `timestamp`/`expires_at` (ADR-048 §Decision 2) untouched (correctly UTC, load-bearing).

Amend ADR-048 §Decision 4 event field-contract table (the "timestamp | ISO-8601 UTC" row, ~line 621) + any other wire-event "ISO-8601 UTC" occurrence to "ISO-8601 with explicit offset (RFC 3339 extended form)" — v1.6, new D-NNN, POLICY 22. Amend BC-3.08.001 Event 9 timestamp example (~L367) + Event 10 `expires_at`/`ts` examples (~L408-409) to drop "UTC", state RFC3339 extended-offset; bring Event 8's `timestamp` (~L327) up to the same explicit wording — v1.35, `## Amendment` entry.

POLICY 22 ratification: YES. Code changes for Direction A alone: NONE.

Not in scope: sink-file/sink-otel-grpc (outside BC-3.08.001's governing scope pending S-4.07) — forward-compat note: once S-4.07 wires them in they are already conformant at zero cost.

If the human selects Direction B instead: complete sweep = `InternalEvent::now` default clock→`Utc::now()` (keep a generic `with_ts<Tz>` for fixed-clock tests); `prune_old_inner` cutoff basis; `reconcile_raw_delete`'s `Local::now()` "today" (REQUIRED); all 6 sink-file sites; 1 sink-otel-grpc site; full `cargo test --workspace --all-targets` + a fresh grep for literal-offset test assertions; a release-notes callout for the named external-consumer risk. Size: small-to-medium, not "3 function edits".

## 5. Follow-up story

Dedicated story, DO NOT fold into S-25.01 (frozen artifact `3919ebcb` already 3-CLEAN converged; the convention predates S-25.01). Proposed title: **"Audit-event timestamp format reconciliation — ADR-048 §D4 wording vs dispatcher convention (O-P18-001)."**

Precondition: human selects Direction A / B / hybrid (e.g. A now, revisit B at S-4.07).

- If A: architect (ADR-048 v1.6) + product-owner (BC-3.08.001 v1.35) spec-only amendments, optional `%z`→`%:z` code diff.
- If B: implementer executes the §4 sweep.

Routing: architect (ADR) + product-owner (BC) jointly own the spec side; implementer only for the RFC-3339 companion fix or Direction B.

No story ID allocated yet — see `.factory/STATE.md` Drift Items for the tracked anchor (D-1156).
