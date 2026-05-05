---
document_type: open-questions-register
level: ops
version: "1.0"
status: active
producer: state-manager
timestamp: 2026-05-05T00:00:00Z
last_amended: 2026-05-05
---

# Open Questions Register

> Tracks unresolved binary-choice and decision-gate items that block downstream
> stories or epics. Each OQ has a named owner, an explicit acceptance criterion,
> and a resolution path. Numbered continuously; no resets across cycles.

---

## OQ-W16-001 — Resolve `vsdd.host.*` registry-prefix decision before E-10 Wave 1 ships

**Source:** gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" — see also gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**") for the bidirectional anchor.
**Status:** OPEN
**Owner:** SS-01 implementer or E-10 Wave 1 architect
**Filed:** 2026-05-05

**Question:** ADR-015 D-15.2 registry table (lines 317-332) does not include `vsdd.host.*`. The gap analysis of `host::exec_subprocess` (gap-analysis-w16-subprocess.md §5) proposes `vsdd.host.exec_subprocess.completed.v1` as the event name for the host-emit-fix story, with fallback `vsdd.dispatcher.subprocess_completed.v1`.

**Acceptance criterion (binary):**
- (a) ADR-015 D-15.2 registry amended to include `vsdd.host.* | <category>` AND the canonical event.name for the host-emit-fix is `vsdd.host.exec_subprocess.completed.v1` (mapped to that prefix's category) BEFORE E-10 Wave 1 host-emit-fix story merges, OR
- (b) the event.name uses `vsdd.dispatcher.subprocess_completed.v1` exactly (NOT a different unregistered prefix; NOT `vsdd.unknown.foo.bar.v1` or other variants).

**Why this matters:** unregistered event prefixes resolve to `event.category = "unknown"` per D-15.2.b; ADR-015 Wave 3 acceptance criterion 2 (line 634) installs an `unknown_category_events` Grafana alert that would actively fire.

**Resolution path:** PR amending ADR-015 to add `vsdd.host.* | lifecycle` (or another category as appropriate) OR explicit choice (b) noted in E-10 Wave 1 story acceptance criteria with rationale.
