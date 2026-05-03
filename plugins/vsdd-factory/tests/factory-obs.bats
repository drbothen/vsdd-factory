#!/usr/bin/env bats
# factory-obs.bats — tests for the observability stack lifecycle CLI.
#
# These tests deliberately do NOT run docker. They validate:
#   - script syntax + help output
#   - compose file parses + has expected services
#   - collector config parses + has expected receivers/exporters
#   - Grafana provisioning files parse
#   - dashboard JSON parses
#
# End-to-end validation (actually starting containers) requires docker
# and is performed manually per tools/observability/README.md.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-obs"
  OBS_DIR="${BATS_TEST_DIRNAME}/../tools/observability"
}

# ---------- Script structural ----------

@test "factory-obs: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-obs: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-obs: no args shows usage" {
  run "$TOOL"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-obs: help shows usage" {
  run "$TOOL" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-obs: --help shows usage" {
  run "$TOOL" --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-obs: unknown subcommand fails" {
  run "$TOOL" nonexistent-subcommand
  [ "$status" -ne 0 ]
}

@test "factory-obs: dashboard subcommand prints URL without requiring docker" {
  # VSDD_OBS_OPEN_BROWSER=0 ensures the subcommand doesn't try to launch a
  # browser window during test runs. BATS captures stdout so -t 1 would be
  # false anyway, but set the flag explicitly to document intent.
  VSDD_OBS_OPEN_BROWSER=0 run "$TOOL" dashboard
  [ "$status" -eq 0 ]
  [[ "$output" == *"http://localhost:"* ]]
}

@test "factory-obs: custom VSDD_OBS_GRAFANA_PORT reflected in dashboard URL" {
  VSDD_OBS_OPEN_BROWSER=0 VSDD_OBS_GRAFANA_PORT=8080 run "$TOOL" dashboard
  [[ "$output" == *"http://localhost:8080"* ]]
}

# ---------- Compose file ----------

@test "compose: file exists" {
  [ -f "$OBS_DIR/docker-compose.yml" ]
}

@test "compose: valid YAML" {
  yq eval "$OBS_DIR/docker-compose.yml" >/dev/null
}

@test "compose: has otel-collector service" {
  run yq eval '.services.otel-collector.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"opentelemetry-collector-contrib"* ]]
}

@test "compose: has loki service" {
  run yq eval '.services.loki.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"loki"* ]]
}

@test "compose: has grafana service" {
  run yq eval '.services.grafana.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"grafana/grafana"* ]]
}

@test "compose: otel-collector waits for loki-ready init container" {
  # v0.76.0: Loki 3.6+ is distroless (no wget/curl), so a CMD healthcheck
  # isn't possible. We gate downstream services on a `loki-ready` init
  # container that polls /ready from outside and exits 0 once Loki
  # accepts writes.
  run yq eval '.services.otel-collector.depends_on.loki-ready.condition' "$OBS_DIR/docker-compose.yml"
  [ "$output" = "service_completed_successfully" ]
}

@test "compose: loki-ready init container polls /ready endpoint" {
  run yq eval '.services.loki-ready.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"busybox"* ]]
  run yq eval '.services.loki-ready.command' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"/ready"* ]]
}

@test "compose: base compose no longer hard-codes a .factory/logs bind (v0.78.0+)" {
  # Multi-factory model: the base compose intentionally does NOT mount
  # any .factory/logs directory. Per-factory mounts are generated into
  # docker-compose.override.yml by `factory-obs up` from the user
  # registry. Scan actual volume values (not raw output with comments)
  # so documentation-mentions of the path don't break this test.
  run yq eval -o=json '.services.otel-collector.volumes[]' "$OBS_DIR/docker-compose.yml"
  [ "$status" -eq 0 ]
  # No volume entry should point at a .factory/logs path or reference
  # the obsolete env var.
  ! grep -q '\.factory/logs' <<<"$output"
  ! grep -q 'VSDD_FACTORY_LOGS' <<<"$output"
}

@test "collector config: filelog globs per-factory subdirectories" {
  # Multi-factory glob — catches each registered factory's events
  # from /var/log/factory/<safe-name>/events-*.jsonl.
  run yq eval '.receivers.filelog.include' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"/var/log/factory/*/events-*.jsonl"* ]]
}

@test "compose: exposes Grafana on port 3000 by default" {
  run yq eval '.services.grafana.ports' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"3000"* ]]
}

# ---------- Collector config ----------

@test "collector config: file exists" {
  [ -f "$OBS_DIR/otel-collector-config.yaml" ]
}

@test "collector config: valid YAML" {
  yq eval "$OBS_DIR/otel-collector-config.yaml" >/dev/null
}

@test "collector config: has filelog receiver" {
  run yq eval '.receivers.filelog.include' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"events-*.jsonl"* ]]
}

@test "collector config: has otlphttp exporter pointed at Loki's OTLP endpoint" {
  # v0.76.0: collector-contrib removed the standalone `loki` exporter
  # around 0.112. We now ship to Loki's native OTLP ingester via
  # otlphttp/loki; /v1/logs is appended automatically.
  run yq eval '.exporters["otlphttp/loki"].endpoint' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"loki:3100/otlp"* ]]
}

@test "collector config: pipeline wires filelog → otlphttp/loki" {
  run yq eval '.service.pipelines.logs.receivers' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"filelog"* ]]
  run yq eval '.service.pipelines.logs.exporters' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"loki"* ]]
}

# ---------- Grafana provisioning ----------

@test "grafana: datasource YAML parses" {
  yq eval "$OBS_DIR/grafana-provisioning/datasources/loki.yaml" >/dev/null
}

@test "grafana: datasource is Loki pointing at service" {
  run yq eval '.datasources[0].url' "$OBS_DIR/grafana-provisioning/datasources/loki.yaml"
  [[ "$output" == *"loki:3100"* ]]
}

@test "grafana: dashboard provider YAML parses" {
  yq eval "$OBS_DIR/grafana-provisioning/dashboards/provider.yaml" >/dev/null
}

@test "grafana: dashboard provider points at /var/lib/grafana/dashboards" {
  run yq eval '.providers[0].options.path' "$OBS_DIR/grafana-provisioning/dashboards/provider.yaml"
  [ "$output" = "/var/lib/grafana/dashboards" ]
}

# ---------- Dashboard JSON ----------

@test "dashboard: factory-overview.json parses" {
  jq . "$OBS_DIR/grafana-dashboards/factory-overview.json" >/dev/null
}

@test "dashboard: factory-overview has stable UID" {
  run jq -r '.uid' "$OBS_DIR/grafana-dashboards/factory-overview.json"
  [ "$output" = "vsdd-factory-overview" ]
}

@test "dashboard: factory-overview has at least 5 panels" {
  run jq -r '.panels | length' "$OBS_DIR/grafana-dashboards/factory-overview.json"
  [ "$output" -ge 5 ]
}

@test "dashboard: all panels reference loki datasource" {
  run jq -r '[.panels[] | .datasource.type] | unique[]' "$OBS_DIR/grafana-dashboards/factory-overview.json"
  # Only "loki" should be present (plus possibly "grafana" for annotations)
  [[ "$output" == *"loki"* ]]
  [[ "$output" != *"prometheus"* ]]
  [[ "$output" != *"tempo"* ]]
}

@test "dashboard: every target has an expr" {
  local missing
  missing=$(jq -r '[.panels[] | .targets[]? | select(.expr == null)] | length' "$OBS_DIR/grafana-dashboards/factory-overview.json")
  [ "$missing" -eq 0 ]
}

# ---------- README ----------

@test "README: file exists" {
  [ -f "$OBS_DIR/README.md" ]
}

@test "README: mentions all four core services" {
  grep -q 'otel-collector' "$OBS_DIR/README.md"
  grep -q 'loki' "$OBS_DIR/README.md"
  grep -q -i 'prometheus' "$OBS_DIR/README.md"
  grep -q 'grafana' "$OBS_DIR/README.md"
}

# ---------- Prometheus (v0.72) ----------

@test "compose: has prometheus service" {
  grep -q 'prometheus:' "$OBS_DIR/docker-compose.yml"
}

@test "compose: prometheus enables remote_write receiver" {
  grep -q -- '--web.enable-remote-write-receiver' "$OBS_DIR/docker-compose.yml"
}

@test "compose: prometheus-data volume declared" {
  grep -q 'prometheus-data:' "$OBS_DIR/docker-compose.yml"
}

@test "compose: otel-collector depends_on prometheus" {
  # Sanity-check the block ordering so the collector doesn't start before
  # prometheus is healthy (otherwise remote_write fails on first push).
  awk '/^  otel-collector:/,/^  [a-z]+:/ {print}' "$OBS_DIR/docker-compose.yml" \
    | grep -q 'prometheus:'
}

@test "prometheus config: file exists and parses" {
  [ -f "$OBS_DIR/prometheus-config.yaml" ]
  yq -e . "$OBS_DIR/prometheus-config.yaml" >/dev/null
}

@test "collector config: has prometheusremotewrite exporter" {
  grep -q 'prometheusremotewrite:' "$OBS_DIR/otel-collector-config.yaml"
}

@test "collector config: metrics pipeline uses prometheusremotewrite" {
  awk '/metrics:/,/traces:|logs:/' "$OBS_DIR/otel-collector-config.yaml" \
    | grep -q prometheusremotewrite
}

@test "grafana: prometheus datasource file exists and parses" {
  local f="$OBS_DIR/grafana-provisioning/datasources/prometheus.yaml"
  [ -f "$f" ]
  yq -e . "$f" >/dev/null
}

@test "grafana: prometheus datasource uid is prometheus" {
  local f="$OBS_DIR/grafana-provisioning/datasources/prometheus.yaml"
  # Dashboards that reference datasource by uid will break if this changes.
  [ "$(yq -r '.datasources[0].uid' "$f")" = "prometheus" ]
}

# ---------- Factory Today dashboard (v0.72) ----------

@test "dashboard: factory-today.json parses" {
  python3 -c "import json; json.load(open('$OBS_DIR/grafana-dashboards/factory-today.json'))"
}

@test "dashboard: factory-today has stable UID" {
  [ "$(jq -r .uid "$OBS_DIR/grafana-dashboards/factory-today.json")" = "factory-today" ]
}

@test "dashboard: factory-today has at least 5 panels" {
  local n
  n=$(jq '.panels | length' "$OBS_DIR/grafana-dashboards/factory-today.json")
  [ "$n" -ge 5 ]
}

@test "dashboard: factory-today all panels reference loki datasource" {
  # All panels on the today dashboard are Loki-backed (by design — Prometheus
  # panels live on the cost dashboard). If one ever references prometheus,
  # revisit this test.
  local non_loki
  non_loki=$(jq '[.panels[] | select(.datasource.uid != "loki" and (.datasource.type // "") != "grafana")] | length' \
    "$OBS_DIR/grafana-dashboards/factory-today.json")
  [ "$non_loki" -eq 0 ]
}

# ---------- Factory PRs dashboard (v0.72.3) ----------

@test "dashboard: factory-prs.json parses" {
  python3 -c "import json; json.load(open('$OBS_DIR/grafana-dashboards/factory-prs.json'))"
}

@test "dashboard: factory-prs has stable UID" {
  [ "$(jq -r .uid "$OBS_DIR/grafana-dashboards/factory-prs.json")" = "factory-prs" ]
}

@test "dashboard: factory-prs has at least 5 panels" {
  local n
  n=$(jq '.panels | length' "$OBS_DIR/grafana-dashboards/factory-prs.json")
  [ "$n" -ge 5 ]
}

@test "dashboard: factory-prs all panels reference loki datasource" {
  local non_loki
  non_loki=$(jq '[.panels[] | select(.datasource.uid != "loki" and (.datasource.type // "") != "grafana")] | length' \
    "$OBS_DIR/grafana-dashboards/factory-prs.json")
  [ "$non_loki" -eq 0 ]
}

@test "dashboard: factory-prs queries pr-manager-completion-guard" {
  grep -q "pr-manager-completion-guard" "$OBS_DIR/grafana-dashboards/factory-prs.json"
}

@test "dashboard: factory-prs queries update-wave-state-on-merge" {
  grep -q "update-wave-state-on-merge" "$OBS_DIR/grafana-dashboards/factory-prs.json"
}

@test "dashboard: factory-today cross-references PRs merged" {
  # PRs merged stat must be present on Factory Today so users see the
  # top-line number without jumping between dashboards.
  jq -e '.panels[] | select(.title == "PRs merged")' \
    "$OBS_DIR/grafana-dashboards/factory-today.json" >/dev/null
}

# ---------- Claude Cost dashboard (v0.73) ----------

@test "dashboard: claude-cost.json parses" {
  python3 -c "import json; json.load(open('$OBS_DIR/grafana-dashboards/claude-cost.json'))"
}

@test "dashboard: claude-cost has stable UID" {
  [ "$(jq -r .uid "$OBS_DIR/grafana-dashboards/claude-cost.json")" = "claude-cost" ]
}

@test "dashboard: claude-cost has at least 10 panels" {
  local n
  n=$(jq '.panels | length' "$OBS_DIR/grafana-dashboards/claude-cost.json")
  [ "$n" -ge 10 ]
}

@test "dashboard: claude-cost all panels reference prometheus datasource" {
  # Every panel target on this dashboard must hit Prometheus — no Loki
  # queries belong on the cost dashboard.
  local non_prom
  non_prom=$(jq '[.panels[] | .targets[]? | select(.datasource.uid != "prometheus")] | length' \
    "$OBS_DIR/grafana-dashboards/claude-cost.json")
  [ "$non_prom" -eq 0 ]
}

@test "dashboard: claude-cost queries claude_code_cost_usage_USD_total" {
  # OTel→Prometheus converter appends the unit ('USD' for cost, 'tokens'
  # for token counts, 'seconds' for active time) between the metric name
  # and `_total`. Verified live against the running Prometheus label
  # index — see v0.73.1 release notes.
  grep -q "claude_code_cost_usage_USD_total" "$OBS_DIR/grafana-dashboards/claude-cost.json"
}

@test "dashboard: claude-cost queries claude_code_token_usage_tokens_total" {
  grep -q "claude_code_token_usage_tokens_total" "$OBS_DIR/grafana-dashboards/claude-cost.json"
}

# ---------- Factory ROI dashboard (v0.74.0) ----------

@test "dashboard: factory-roi.json parses" {
  python3 -c "import json; json.load(open('$OBS_DIR/grafana-dashboards/factory-roi.json'))"
}

@test "dashboard: factory-roi has stable UID" {
  [ "$(jq -r .uid "$OBS_DIR/grafana-dashboards/factory-roi.json")" = "factory-roi" ]
}

@test "dashboard: factory-roi has at least 8 panels" {
  local n
  n=$(jq '.panels | length' "$OBS_DIR/grafana-dashboards/factory-roi.json")
  [ "$n" -ge 8 ]
}

@test "dashboard: factory-roi queries both cost and pr.merged event" {
  # Cross-datasource panels are the whole point of this dashboard.
  grep -q "claude_code_cost_usage_USD_total" "$OBS_DIR/grafana-dashboards/factory-roi.json"
  grep -q "event_type=\\\\\"pr.merged\\\\\"" "$OBS_DIR/grafana-dashboards/factory-roi.json"
}

@test "dashboard: factory-roi Cost per commit uses capture-commit-activity stream (v0.77.0+)" {
  # In v0.77.0 this panel was rebuilt. The original ran a Prom-native
  # division against claude_code_commit_count_total — a metric documented
  # upstream but never emitted by Claude's SDK in our environment, so the
  # panel silently read N/A from v0.74.0 through v0.76.1. We now compute
  # it as a cross-datasource divide: Prometheus cost (A) / Loki commit.made
  # event count (B) via server-side expression refC = "$A / $B".
  local roi="$OBS_DIR/grafana-dashboards/factory-roi.json"
  jq -e '.panels[] | select(.title == "Cost per commit") | select(.type == "stat")' "$roi" >/dev/null
  # Must have the three-refId shape.
  jq -e '.panels[] | select(.title == "Cost per commit") | .targets[] | select(.refId == "A" and .datasource.type == "prometheus")' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per commit") | .targets[] | select(.refId == "B" and .datasource.type == "loki" and (.expr | contains("event_type=\"commit.made\"")))' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per commit") | .targets[] | select(.refId == "C" and .expression == "$A / $B")' "$roi" >/dev/null
  # Must NOT reference the dead Prom metric anywhere in the panel.
  ! jq -e '.panels[] | select(.title == "Cost per commit") | .targets[] | select(.expr // "" | contains("claude_code_commit_count_total"))' "$roi" >/dev/null
}

@test "dashboard: factory-roi Commits stat uses Loki commit.made stream (v0.77.0+)" {
  # Sibling of the Cost per commit rebuild — the raw commit count panel
  # also shifts from the dead Prom metric to Loki's commit.made events.
  local roi="$OBS_DIR/grafana-dashboards/factory-roi.json"
  jq -e '.panels[] | select(.title == "Commits") | .datasource.type' "$roi" | grep -q loki
  jq -e '.panels[] | select(.title == "Commits") | .targets[] | select(.expr | contains("event_type=\"commit.made\""))' "$roi" >/dev/null
  ! jq -e '.panels[] | select(.title == "Commits") | .targets[] | select(.expr // "" | contains("claude_code_commit_count_total"))' "$roi" >/dev/null
}

@test "dashboard: factory-roi has Cost per active hour panel (v0.77.0+)" {
  # Companion to Cost per commit — pure Prom-native, independent of git
  # workflow. Uses cost / (active_time/3600).
  local roi="$OBS_DIR/grafana-dashboards/factory-roi.json"
  jq -e '.panels[] | select(.title == "Cost per active hour") | select(.type == "stat")' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per active hour") | .targets[] | select(.expr | contains("claude_code_cost_usage_USD_total") and contains("claude_code_active_time_seconds_total") and contains("3600"))' "$roi" >/dev/null
}

@test "dashboard: factory-roi has Cost per active minute + second re-unit panels (v0.77.1+)" {
  # Same signal as Cost per active hour, re-unit to minute + second so
  # the displayed number is intuitive at typical Claude usage patterns
  # ($50-ish/min, <$1/sec). All three share a row at y=8.
  local roi="$OBS_DIR/grafana-dashboards/factory-roi.json"
  jq -e '.panels[] | select(.title == "Cost per active minute") | select(.type == "stat")' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per active minute") | .targets[] | select(.expr | contains("claude_code_active_time_seconds_total") and contains("/ 60"))' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per active second") | select(.type == "stat")' "$roi" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per active second") | .targets[] | select(.expr | contains("claude_code_active_time_seconds_total"))' "$roi" >/dev/null
  # Per-second query must NOT have a divisor on active_time (raw seconds).
  ! jq -e '.panels[] | select(.title == "Cost per active second") | .targets[] | select(.expr | contains("/ 60") or contains("/ 3600"))' "$roi" >/dev/null
  # All three panels share y=8 (same row).
  local row_count
  row_count=$(jq -r '[.panels[] | select(.title | test("^Cost per active ")) | .gridPos.y] | unique | length' "$roi")
  [ "$row_count" -eq 1 ]
}

# NOTE: capture-commit-activity.sh was deleted in commit 818fb95 because
# it has a native WASM equivalent in the dispatcher. The existence /
# executability / registration check that used to live here is no longer
# applicable — the WASM hook is tested via the dispatcher test suite.

# ---------- Observability stack upgrade (v0.76.0) ----------

@test "compose: Grafana pinned to a 13.x image" {
  # v0.76.0 upgraded the stack. Grafana must be 13.x — v10 lacks
  # server-side expression queries for cross-datasource math.
  run yq -r '.services.grafana.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == grafana/grafana:13.* ]]
}

@test "compose: otel-collector-contrib pinned to 0.115+ for deltatocumulative" {
  # The deltatocumulative processor was introduced in collector-contrib 0.115.
  # Required so we can drop the OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE
  # env var from the claude-telemetry skill.
  run yq -r '.services.otel-collector.image' "$OBS_DIR/docker-compose.yml"
  local ver="${output##*:}"   # strip image prefix
  # Expect 0.X.Y where X >= 115.
  [[ "$ver" =~ ^0\.([0-9]+)\.[0-9]+$ ]]
  [ "${BASH_REMATCH[1]}" -ge 115 ]
}

@test "compose: loki pinned to 3.x image" {
  run yq -r '.services.loki.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == grafana/loki:3.* ]]
}

@test "compose: prometheus pinned to v3.x image" {
  run yq -r '.services.prometheus.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == prom/prometheus:v3.* ]]
}

@test "compose: renderer sidecar is wired with shared auth token" {
  # Grafana v11+ refuses to start with the default renderer_token. Both
  # sides (GF_RENDERING_RENDERER_TOKEN on grafana, AUTH_TOKEN on renderer)
  # must share the same non-default value.
  local g_token r_token
  g_token=$(yq -r '.services.grafana.environment.GF_RENDERING_RENDERER_TOKEN' "$OBS_DIR/docker-compose.yml")
  r_token=$(yq -r '.services.renderer.environment.AUTH_TOKEN' "$OBS_DIR/docker-compose.yml")
  [ -n "$g_token" ]
  [ "$g_token" != "null" ]
  [ "$g_token" = "$r_token" ]
}

@test "compose: renderer pinned to 3.11.x image" {
  run yq -r '.services.renderer.image' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == grafana/grafana-image-renderer:3.11.* ]]
}

@test "loki config: file exists and parses" {
  [ -f "$OBS_DIR/loki-config.yaml" ]
  yq -e . "$OBS_DIR/loki-config.yaml" >/dev/null
}

@test "loki config: accepts events up to 30 days old" {
  # Required so that a stack restart can backfill historical events from
  # .factory/logs/events-*.jsonl without the collector dropping batches
  # on "entry too far behind" 400s from Loki.
  run yq -r '.limits_config.reject_old_samples_max_age' "$OBS_DIR/loki-config.yaml"
  [ "$output" = "720h" ]
}

@test "loki config: otlp_config promotes 5 factory labels to stream index" {
  # event_type/hook/reason/severity become stream labels so dashboards can
  # filter with {event_type="pr.merged"} etc. service.name is also index_label
  # by default but listing it explicitly is safer against future Loki default
  # changes.
  local attrs
  attrs=$(yq -r '.limits_config.otlp_config.resource_attributes.attributes_config[0].attributes | join(",")' \
    "$OBS_DIR/loki-config.yaml")
  [[ "$attrs" == *"service.name"* ]]
  [[ "$attrs" == *"event_type"* ]]
  [[ "$attrs" == *"hook"* ]]
  [[ "$attrs" == *"reason"* ]]
  [[ "$attrs" == *"severity"* ]]
}

@test "collector config: metrics pipeline runs deltatocumulative before export" {
  # Claude's SDK defaults to DELTA temporality, Prometheus requires
  # CUMULATIVE. deltatocumulative must convert before prometheusremotewrite.
  local processors
  processors=$(yq -r '.service.pipelines.metrics.processors | join(",")' "$OBS_DIR/otel-collector-config.yaml")
  [[ "$processors" == *"deltatocumulative"* ]]
  # deltatocumulative should be LISTED before the export happens (it's in
  # the processors list, which runs in declared order).
  [[ "$processors" =~ deltatocumulative ]]
}

@test "collector config: has deltatocumulative processor defined" {
  run yq -e '.processors.deltatocumulative' "$OBS_DIR/otel-collector-config.yaml"
  [ "$status" -eq 0 ]
}

@test "claude-telemetry skill: does NOT list the temporality env var" {
  # v0.76.0 dropped the 6th env var after deltatocumulative went live.
  # The temporality key should only appear inside the `del(...)` block
  # (for backwards-compatible cleanup on re-run) and NOT in the merge
  # block that sets env values.
  run grep -c 'OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE' \
    "${BATS_TEST_DIRNAME}/../skills/claude-telemetry/SKILL.md"
  # Expected occurrences:
  #   1× in the v0.76.0 deprecation note
  #   1× in the `on` jq del(...) block (back-compat pruning)
  #   1× in the `off` jq del(...) block
  #   1× in the `status` legacy-key warning
  # That's 4 references total. More than that means the merge block still
  # sets it, which is a regression.
  [ "$output" -le 4 ]
}

@test "dashboard: factory-roi has cross-datasource Cost per PR / Cost per story derived stat panels" {
  # v0.76.0: upgraded to Grafana v13, which supports server-side expression
  # queries (datasource.type="__expr__"). The Cost per PR merged and Cost
  # per story touched panels use a Prometheus query (A) + a Loki query (B)
  # + a math expression query (C: "$A / $B"), then filterByRefId keeps
  # only C. This replaces the markdown text fallbacks that shipped in
  # v0.73.0–v0.75.0 when v10.4.2's calculateField transform couldn't
  # reliably divide cross-datasource frames.
  jq -e '.panels[] | select(.title == "Cost per PR merged") | select(.type == "stat")' \
    "$OBS_DIR/grafana-dashboards/factory-roi.json" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per story touched") | select(.type == "stat")' \
    "$OBS_DIR/grafana-dashboards/factory-roi.json" >/dev/null
  # Both panels must declare the server-side expression refC.
  jq -e '.panels[] | select(.title == "Cost per PR merged") | .targets[] | select(.refId == "C" and .expression == "$A / $B")' \
    "$OBS_DIR/grafana-dashboards/factory-roi.json" >/dev/null
  jq -e '.panels[] | select(.title == "Cost per story touched") | .targets[] | select(.refId == "C" and .expression == "$A / $B")' \
    "$OBS_DIR/grafana-dashboards/factory-roi.json" >/dev/null
}
