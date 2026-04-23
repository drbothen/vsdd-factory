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

@test "compose: otel-collector depends on loki being healthy" {
  run yq eval '.services.otel-collector.depends_on.loki.condition' "$OBS_DIR/docker-compose.yml"
  [ "$output" = "service_healthy" ]
}

@test "compose: mounts .factory/logs into collector" {
  run yq eval '.services.otel-collector.volumes' "$OBS_DIR/docker-compose.yml"
  [[ "$output" == *"/var/log/factory"* ]]
  [[ "$output" == *"ro"* ]]
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

@test "collector config: has loki exporter" {
  run yq eval '.exporters.loki.endpoint' "$OBS_DIR/otel-collector-config.yaml"
  [[ "$output" == *"loki"* ]]
  [[ "$output" == *"/loki/api/v1/push"* ]]
}

@test "collector config: pipeline wires filelog → loki" {
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
