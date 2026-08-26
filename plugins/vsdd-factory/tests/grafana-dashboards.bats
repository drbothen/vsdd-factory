#!/usr/bin/env bats
# grafana-dashboards.bats — static guards for Grafana dashboard JSON
#
# There is no harness exercising Grafana panel `expr` (factory-dashboard.bats
# covers the terminal renderer, not these panels), which is why the
# `attributes_`-prefixed LogQL idiom (#243, #244, #245) shipped and sat
# broken: the OTEL collector promotes those attributes to structured
# metadata, so `| json | attributes_x` parses the log BODY and matches
# nothing. These grep-level guards are the cheap ratchet against the same
# class recurring — they assert the stale idiom stays out of the dashboards
# without needing a live Loki stack.
#
# Refs: #243

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  DASHBOARDS="$PLUGIN_ROOT/tools/observability/grafana-dashboards"
}

@test "grafana-dashboards: dashboard dir exists and is non-empty" {
  [ -d "$DASHBOARDS" ]
  count=$(find "$DASHBOARDS" -maxdepth 1 -name '*.json' | wc -l)
  [ "$count" -gt 0 ]
}

@test "grafana-dashboards: no '| json | attributes_' body-parse stage survives" {
  # The exact broken pipeline shape: parse body as JSON, then filter on an
  # attributes_-prefixed field that only exists pre-promotion.
  run grep -l 'json | attributes_' "$DASHBOARDS"/*.json
  [ "$status" -ne 0 ]
}

@test "grafana-dashboards: no attributes_-prefixed label reference anywhere" {
  # Broader ratchet: attributes_ prefixes (exprs, legendFormat,
  # ${__field.labels.attributes_*} displayNames) are the stale pre-promotion
  # idiom in every form. Structured-metadata labels carry the bare name.
  run grep -l 'attributes_' "$DASHBOARDS"/*.json
  [ "$status" -ne 0 ]
}

@test "grafana-dashboards: every dashboard is valid JSON" {
  for f in "$DASHBOARDS"/*.json; do
    jq empty "$f" || { echo "invalid JSON: $f"; return 1; }
  done
}
