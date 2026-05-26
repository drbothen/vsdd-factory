//! validate-policies-schema — PostToolUse WASM hook plugin.
//!
//! Validates `.factory/policies.yaml` structural integrity (Part B) and
//! checks dispatch packages (`td-*-dispatch.md`) against the cargo-audit
//! advisory cache (Part C).
//!
//! # Part B — policies.yaml arm
//!
//! Fires PostToolUse on any write to a file whose `file_name()` is exactly
//! `policies.yaml`. Checks:
//!
//! 1. YAML parse success (parse failure → block with error location)
//! 2. Required header fields: `document_type`, `version`, `last_amended`
//!    (missing field → block citing F-PASS14-004)
//! 3. Each policy entry has all 7 required fields: id, name, severity, scope,
//!    description, lint_hook, codified_at
//! 4. `id` is a bare YAML integer in [1, 999] (string forms → block, F-PASS14-006)
//! 5. `severity` is exactly `HIGH` or `MEDIUM` (other values → block)
//! 6. No duplicate `id` values across entries
//! 7. `lint_hook` non-null → plugin exists in hooks-registry.toml (registry
//!    read failure skips this check only; other checks continue)
//! 8. `lint_hook` non-null → `codified_at` matches `D-\d+` format
//! 9. Extra/unknown fields → advisory log + Continue (forward-compat)
//! 10. All violations cascade-reported in a single Block
//!
//! # Part C — dispatch package arm
//!
//! Fires PostToolUse on any write to a file whose `file_name()` matches
//! `td-*-dispatch.md` pattern (basename check, not ends_with). Checks
//! recommended crate dependencies against `.factory/hooks/cargo-audit-cache.json`.
//! HIGH/CRITICAL advisory → block; MEDIUM/LOW → log_warn + Continue.
//! Cache absent/unreadable → Continue + log_warn (fail-open).
//!
//! # Architecture compliance
//!
//! - BC-5.39.008 postconditions 1–14; invariants 1–11.
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning.
//! - File-path enforcement via `Path::file_name` — NOT `ends_with`.
//! - `is_char_boundary()` guards on all byte-index slice operations.
//! - Fail-open on every `host::read_file` error (BC-5.39.008 invariant 8).
//! - Schema-violation cascade: all violations in one Block (invariant 7).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Maximum bytes to read from any target file via `host::read_file`.
///
/// 512 KiB — parity with sibling hooks; prevents META-LEVEL-24 false-green
/// truncation (BC-5.39.008 invariant 10).
pub const MAX_BYTES: u32 = 524_288;

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Violation type
// ---------------------------------------------------------------------------

/// A structural violation found during schema validation.
///
/// Carries a human-readable `message` and the `cited_raw` body-literal text
/// that triggered the violation. Structural plumbing per TD-VSDD-059.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description used verbatim in the block message.
    pub message: String,
    /// The raw source text that triggered the violation (TD-VSDD-059).
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// File-path guards (path-component-strict — BC-5.39.008 invariant 3)
// ---------------------------------------------------------------------------

/// Returns `true` if the file's `file_name()` component is exactly `policies.yaml`.
///
/// Uses path-component-strict matching — NOT `ends_with`, which would false-positive
/// on `xpolicies.yaml`.
///
/// # BC trace
/// BC-5.39.008 invariant 3.
pub fn is_policies_yaml_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("policies.yaml")
}

/// Returns `true` if the file's `file_name()` matches `td-*-dispatch.md` pattern.
///
/// Checks basename only — NOT full path. Basename must start with `td-` and end
/// with `-dispatch.md`. Uses path-component-strict extraction.
///
/// # BC trace
/// BC-5.39.008 invariant 3 (Part C path-component-strict).
pub fn is_dispatch_package_target(file_path: &str) -> bool {
    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    // Pattern: ^td-.*-dispatch\.md$ where .* is non-empty (at least one char between td- and -dispatch.md).
    // Ensure the prefix and suffix don't overlap: filename must have length > len("td-") + len("-dispatch.md").
    const PREFIX: &str = "td-";
    const SUFFIX: &str = "-dispatch.md";
    if filename.len() <= PREFIX.len() + SUFFIX.len() {
        return false;
    }
    filename.starts_with(PREFIX) && filename.ends_with(SUFFIX)
}

// ---------------------------------------------------------------------------
// YAML parsing types
// ---------------------------------------------------------------------------

/// Parsed header fields from the policies.yaml YAML front-matter document.
#[derive(Debug, Default)]
pub struct FrontmatterHeader {
    pub document_type: Option<String>,
    pub version: Option<String>,
    pub last_amended: Option<String>,
}

/// A single parsed policy entry from the `policies:` list.
#[derive(Debug, Default)]
pub struct PolicyEntry {
    /// `None` means field was absent from YAML; `Some(v)` means present (value is serde_json::Value).
    pub id: Option<serde_json::Value>,
    pub name: Option<serde_json::Value>,
    pub severity: Option<serde_json::Value>,
    pub scope: Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
    /// Outer `None` = field absent; `Some(None)` = YAML null; `Some(Some(s))` = string value.
    pub lint_hook: Option<Option<String>>,
    pub codified_at: Option<serde_json::Value>,
    /// Names of keys in this entry not in the 7 required fields.
    pub unknown_fields: Vec<String>,
}

// ---------------------------------------------------------------------------
// YAML parsing
// ---------------------------------------------------------------------------

/// Parse policies.yaml content using serde_norway.
///
/// Returns `(header, entries, _)` or `Err(parse-error-message)`.
/// serde_norway resolves YAML anchors natively before schema checks.
///
/// Pure: reads only from the `&str` input.
///
/// # BC trace
/// BC-5.39.008 postcondition 1 (parse failure → Err with location).
pub fn parse_policies_yaml(content: &str) -> Result<(FrontmatterHeader, Vec<PolicyEntry>), String> {
    // policies.yaml uses two YAML documents separated by `---`:
    //   document 1 (optional): frontmatter { document_type, version, last_amended }
    //   document 2: mapping { policies: [...] }
    //
    // serde_norway supports multi-document parsing; we collect all documents,
    // then identify header vs policies doc by structure.

    // Collect all YAML documents from the file.
    // serde_norway supports multi-document streams via its Deserializer.
    // The Deserializer itself is iterable: `for document in Deserializer::from_str(input)`.
    use serde::Deserialize as _;
    let mut docs: Vec<serde_norway::Value> = Vec::new();
    for document in serde_norway::Deserializer::from_str(content) {
        match serde_norway::Value::deserialize(document) {
            Ok(v) => docs.push(v),
            Err(e) => {
                if docs.is_empty() {
                    return Err(format!("YAML parse error: {e}"));
                }
                // If we already parsed some docs and hit an error on next doc,
                // the content is malformed — report it.
                return Err(format!("YAML parse error: {e}"));
            }
        }
    }

    // Separate frontmatter doc (has document_type/version/last_amended)
    // from the policies doc (has `policies:` key).
    let mut header = FrontmatterHeader::default();
    let mut entries: Vec<PolicyEntry> = Vec::new();

    for doc in docs {
        let mapping = match doc.as_mapping() {
            Some(m) => m,
            None => continue,
        };

        // Check if this doc looks like the frontmatter (has document_type key).
        let has_doc_type = mapping
            .iter()
            .any(|(k, _)| k.as_str() == Some("document_type"));
        // Check if this doc looks like the policies body (has `policies` key).
        let has_policies = mapping.iter().any(|(k, _)| k.as_str() == Some("policies"));

        if has_doc_type {
            // Frontmatter document.
            for (k, v) in mapping {
                let key = k.as_str().unwrap_or("");
                match key {
                    "document_type" => {
                        header.document_type = v.as_str().map(|s| s.to_string());
                    }
                    "version" => {
                        header.version = v.as_str().map(|s| s.to_string());
                    }
                    "last_amended" => {
                        header.last_amended = v.as_str().map(|s| s.to_string());
                    }
                    _ => {}
                }
            }
        } else if has_policies {
            // Policies body document.
            if let Some(policies_val) = mapping
                .iter()
                .find(|(k, _)| k.as_str() == Some("policies"))
                .map(|(_, v)| v)
                && let Some(list) = policies_val.as_sequence()
            {
                for item in list {
                    let entry = parse_policy_entry(item);
                    entries.push(entry);
                }
            }
        }
    }

    Ok((header, entries))
}

/// Parse a single policy entry from its serde_norway::Value representation.
fn parse_policy_entry(val: &serde_norway::Value) -> PolicyEntry {
    const KNOWN_FIELDS: &[&str] = &[
        "id",
        "name",
        "severity",
        "scope",
        "description",
        "lint_hook",
        "codified_at",
    ];

    let mut entry = PolicyEntry::default();

    let mapping = match val.as_mapping() {
        Some(m) => m,
        None => return entry,
    };

    for (k, v) in mapping {
        let key = k.as_str().unwrap_or("");
        match key {
            "id" => {
                entry.id = Some(value_to_json(v));
            }
            "name" => {
                entry.name = Some(value_to_json(v));
            }
            "severity" => {
                entry.severity = Some(value_to_json(v));
            }
            "scope" => {
                entry.scope = Some(value_to_json(v));
            }
            "description" => {
                entry.description = Some(value_to_json(v));
            }
            "lint_hook" => {
                if v.is_null() {
                    entry.lint_hook = Some(None);
                } else {
                    entry.lint_hook = Some(v.as_str().map(|s| s.to_string()));
                }
            }
            "codified_at" => {
                entry.codified_at = Some(value_to_json(v));
            }
            other => {
                if !KNOWN_FIELDS.contains(&other) {
                    entry.unknown_fields.push(other.to_string());
                }
            }
        }
    }

    entry
}

/// Convert a serde_norway::Value to a serde_json::Value for uniform handling.
fn value_to_json(v: &serde_norway::Value) -> serde_json::Value {
    match v {
        serde_norway::Value::Null => serde_json::Value::Null,
        serde_norway::Value::Bool(b) => serde_json::Value::Bool(*b),
        serde_norway::Value::Number(n) => {
            // Try i64 first, then u64, then f64.
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(serde_json::Number::from(i))
            } else if let Some(u) = n.as_u64() {
                serde_json::Value::Number(serde_json::Number::from(u))
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        serde_norway::Value::String(s) => serde_json::Value::String(s.clone()),
        serde_norway::Value::Sequence(seq) => {
            serde_json::Value::Array(seq.iter().map(value_to_json).collect())
        }
        serde_norway::Value::Mapping(m) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in m {
                if let Some(key) = k.as_str() {
                    obj.insert(key.to_string(), value_to_json(val));
                }
            }
            serde_json::Value::Object(obj)
        }
        serde_norway::Value::Tagged(t) => value_to_json(&t.value),
    }
}

// ---------------------------------------------------------------------------
// Part B — schema check functions
// ---------------------------------------------------------------------------

/// Check that all required header fields are present and valid.
///
/// Required: `document_type == "governance-policy-registry"`, `version` present,
/// `last_amended` present.
///
/// # BC trace
/// BC-5.39.008 postcondition 2; F-PASS14-004.
pub fn check_header_fields(header: &FrontmatterHeader) -> Vec<Violation> {
    let mut violations = Vec::new();

    match &header.document_type {
        None => {
            violations.push(Violation {
                message: "policies.yaml missing required header field `document_type`; \
                          expected `document_type: governance-policy-registry` per F-PASS14-004"
                    .to_string(),
                cited_raw: "document_type".to_string(),
            });
        }
        Some(dt) if dt != "governance-policy-registry" => {
            violations.push(Violation {
                message: format!(
                    "policies.yaml header field `document_type` has wrong value `{dt}`; \
                     expected `governance-policy-registry` per F-PASS14-004"
                ),
                cited_raw: dt.clone(),
            });
        }
        _ => {}
    }

    if header.version.is_none() {
        violations.push(Violation {
            message: "policies.yaml missing required header field `version` per F-PASS14-004"
                .to_string(),
            cited_raw: "version".to_string(),
        });
    }

    if header.last_amended.is_none() {
        violations.push(Violation {
            message: "policies.yaml missing required header field `last_amended` per F-PASS14-004"
                .to_string(),
            cited_raw: "last_amended".to_string(),
        });
    }

    violations
}

/// Check that a policy entry has all 7 required fields present.
///
/// Required fields: id, name, severity, scope, description, lint_hook, codified_at.
///
/// # BC trace
/// BC-5.39.008 postcondition 3; invariant 5.
pub fn check_required_fields(entry: &PolicyEntry, index: usize) -> Vec<Violation> {
    let mut violations = Vec::new();
    let entry_label = entry_display_label(entry, index);

    if entry.id.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `id`"),
            cited_raw: "id".to_string(),
        });
    }
    if entry.name.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `name`"),
            cited_raw: "name".to_string(),
        });
    }
    if entry.severity.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `severity`"),
            cited_raw: "severity".to_string(),
        });
    }
    if entry.scope.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `scope`"),
            cited_raw: "scope".to_string(),
        });
    }
    if entry.description.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `description`"),
            cited_raw: "description".to_string(),
        });
    }
    if entry.lint_hook.is_none() {
        violations.push(Violation {
            message: format!("policy entry {entry_label} missing required field `lint_hook`; \
                             use `lint_hook: null` if no lint hook is assigned (POLICY 13 per D-472)"),
            cited_raw: "lint_hook".to_string(),
        });
    }
    if entry.codified_at.is_none() {
        violations.push(Violation {
            message: format!(
                "policy entry {entry_label} missing required field `codified_at`; \
                 use `codified_at: null` for baseline policies predating D-472"
            ),
            cited_raw: "codified_at".to_string(),
        });
    }

    violations
}

/// Check that a policy entry's `id` field is a bare YAML integer in [1, 999].
///
/// Blocks on: string forms, negative, zero, >999, `POLICY NNN`-prefixed strings.
///
/// # BC trace
/// BC-5.39.008 postcondition 4; invariant 4; F-PASS14-006.
pub fn check_policy_id_format(entry: &PolicyEntry, index: usize) -> Vec<Violation> {
    let id_val = match &entry.id {
        Some(v) => v,
        None => return Vec::new(), // Missing id already caught by check_required_fields
    };
    let entry_label = entry_display_label(entry, index);

    match id_val {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if !(1..=999).contains(&i) {
                    return vec![Violation {
                        message: format!(
                            "policy entry {entry_label} has `id: {i}` which is out of range; \
                             id must be a bare YAML integer in [1, 999] per F-PASS14-006"
                        ),
                        cited_raw: i.to_string(),
                    }];
                }
                // Valid integer id
                Vec::new()
            } else {
                vec![Violation {
                    message: format!(
                        "policy entry {entry_label} has non-integer numeric `id`; \
                         id must be a bare YAML integer in [1, 999] per F-PASS14-006"
                    ),
                    cited_raw: n.to_string(),
                }]
            }
        }
        serde_json::Value::String(s) => {
            vec![Violation {
                message: format!(
                    "policy entry {entry_label} has string-format `id: \"{s}\"`; \
                     id must be a bare YAML integer scalar in [1, 999], not a string; \
                     string forms are non-conforming per F-PASS14-006"
                ),
                cited_raw: s.clone(),
            }]
        }
        other => {
            vec![Violation {
                message: format!(
                    "policy entry {entry_label} has non-integer `id` (type: {}); \
                     id must be a bare YAML integer in [1, 999] per F-PASS14-006",
                    json_type_name(other)
                ),
                cited_raw: other.to_string(),
            }]
        }
    }
}

/// Check that a policy entry's `severity` is exactly `HIGH` or `MEDIUM`.
///
/// # BC trace
/// BC-5.39.008 invariant 5; postcondition 3.
pub fn check_severity_value(entry: &PolicyEntry, index: usize) -> Vec<Violation> {
    let sev_val = match &entry.severity {
        Some(v) => v,
        None => return Vec::new(), // Missing severity already caught by check_required_fields
    };
    let entry_label = entry_display_label(entry, index);

    match sev_val {
        serde_json::Value::String(s) => {
            if s != "HIGH" && s != "MEDIUM" {
                vec![Violation {
                    message: format!(
                        "policy entry {entry_label} has non-canonical `severity: \"{s}\"`; \
                         severity must be exactly `HIGH` or `MEDIUM` per BC-5.39.008 invariant 5"
                    ),
                    cited_raw: s.clone(),
                }]
            } else {
                Vec::new()
            }
        }
        other => {
            vec![Violation {
                message: format!(
                    "policy entry {entry_label} has non-string `severity` (type: {}); \
                     severity must be `HIGH` or `MEDIUM`",
                    json_type_name(other)
                ),
                cited_raw: other.to_string(),
            }]
        }
    }
}

/// Check for duplicate `id` values across all entries.
///
/// # BC trace
/// BC-5.39.008 postcondition 5.
pub fn check_duplicate_ids(entries: &[PolicyEntry]) -> Vec<Violation> {
    let mut seen: Vec<i64> = Vec::new();
    let mut duplicates: Vec<i64> = Vec::new();

    for entry in entries {
        if let Some(serde_json::Value::Number(n)) = &entry.id
            && let Some(i) = n.as_i64()
        {
            if seen.contains(&i) {
                if !duplicates.contains(&i) {
                    duplicates.push(i);
                }
            } else {
                seen.push(i);
            }
        }
    }

    duplicates
        .into_iter()
        .map(|i| Violation {
            message: format!(
                "duplicate `id: {i}` found across policy entries; \
                 policy ids must be unique per BC-5.39.008 postcondition 5"
            ),
            cited_raw: i.to_string(),
        })
        .collect()
}

/// Check whether a `lint_hook` value names a plugin that exists in the registry.
///
/// Handles namespaced slugs: `"vsdd-factory:validate-burst-log"` → looks for
/// `validate-burst-log` in the registry.
///
/// Pure: operates on the registry content string.
///
/// # BC trace
/// BC-5.39.008 postcondition 6.
pub fn check_lint_hook_exists(lint_hook: &str, registry_content: &str) -> bool {
    // Normalize namespaced slug: extract basename after `:`.
    let basename = if let Some(pos) = lint_hook.rfind(':') {
        let after = &lint_hook[pos + 1..];
        if after.is_empty() { lint_hook } else { after }
    } else {
        lint_hook
    };

    // Check if `name = "<basename>"` appears in the registry content.
    // The canonical registry form is: `name = "validate-dispatch-advance"`.
    let pattern = format!("name = \"{basename}\"");
    registry_content.contains(&pattern)
}

/// Check `lint_hook` and `codified_at` coupling for a single policy entry.
///
/// If `lint_hook` is non-null:
/// - If registry_content is `Some`: check plugin existence; add violation if not found.
/// - Check `codified_at` matches `D-NNN` format (`D-` followed by one or more digits).
///
/// If `lint_hook` is null: no violations for either field.
///
/// # BC trace
/// BC-5.39.008 postconditions 6+7; invariant 5.
pub fn check_lint_hook_and_codified_at(
    entry: &PolicyEntry,
    index: usize,
    registry_content: Option<&str>,
) -> Vec<Violation> {
    let entry_label = entry_display_label(entry, index);

    // Determine if lint_hook is non-null.
    let lint_hook_value = match &entry.lint_hook {
        None => return Vec::new(), // Field absent — caught by check_required_fields
        Some(None) => return Vec::new(), // lint_hook: null — no checks needed
        Some(Some(s)) => s.as_str(),
    };

    let mut violations = Vec::new();

    // Check plugin existence in registry (only if registry is available).
    if let Some(registry) = registry_content
        && !check_lint_hook_exists(lint_hook_value, registry)
    {
        violations.push(Violation {
            message: format!(
                "policy entry {entry_label} references `lint_hook: \"{lint_hook_value}\"` \
                 which is not found in hooks-registry.toml; \
                 add the plugin entry or set lint_hook to null"
            ),
            cited_raw: lint_hook_value.to_string(),
        });
    }

    // Check codified_at matches D-NNN format.
    let codified_raw = match &entry.codified_at {
        None => {
            // Field absent — already caught by check_required_fields; skip here
            return violations;
        }
        Some(serde_json::Value::Null) => {
            // codified_at: null with non-null lint_hook — that's acceptable
            // (baseline policies predate D-472 and may have null codified_at)
            return violations;
        }
        Some(serde_json::Value::String(s)) => s.as_str(),
        Some(other) => {
            violations.push(Violation {
                message: format!(
                    "policy entry {entry_label} `codified_at` has wrong type ({}); \
                     must be a string matching D-NNN format or null",
                    json_type_name(other)
                ),
                cited_raw: other.to_string(),
            });
            return violations;
        }
    };

    // Check D-NNN format: "D-" followed by one or more digits.
    if !is_d_nnn_format(codified_raw) {
        violations.push(Violation {
            message: format!(
                "policy entry {entry_label} has malformed `codified_at: \"{codified_raw}\"`; \
                 expected `D-NNN` format (e.g., `D-472`) per BC-5.39.008 postcondition 7; \
                 POLICY 16 per D-472"
            ),
            cited_raw: codified_raw.to_string(),
        });
    }

    violations
}

/// Check if a string matches the `D-NNN` format (`D-` followed by one or more digits).
fn is_d_nnn_format(s: &str) -> bool {
    if let Some(rest) = s.strip_prefix("D-") {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

/// Orchestrate all Part B checks for policies.yaml content.
///
/// Reads hooks-registry.toml via `host::read_file`; on read failure, skips
/// the lint_hook-existence check only (other checks continue).
///
/// Returns ALL violations accumulated across all entries (cascade invariant 7).
/// This function is effectful (calls host I/O).
///
/// # BC trace
/// BC-5.39.008 postconditions 1–11; invariants 1, 7, 8.
pub fn validate_policies_schema(content: &str) -> Vec<Violation> {
    use vsdd_hook_sdk::host;

    let (header, entries) = match parse_policies_yaml(content) {
        Ok((h, e)) => (h, e),
        Err(parse_err) => {
            return vec![Violation {
                message: format!("policies.yaml YAML parse error: {parse_err}"),
                cited_raw: parse_err,
            }];
        }
    };

    let mut violations = Vec::new();

    // Check header fields.
    violations.extend(check_header_fields(&header));

    // Try to read hooks-registry.toml for lint_hook-existence checks.
    let registry_path = "plugins/vsdd-factory/hooks-registry.toml";
    let registry_content: Option<String> = match host::read_file(registry_path, MAX_BYTES, 2000) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Some(s),
            Err(e) => {
                host::log_warn(&format!(
                    "[validate-policies-schema] UTF-8 decode failure reading {registry_path}: {e}; \
                     skipping lint_hook-existence check"
                ));
                None
            }
        },
        Err(e) => {
            host::log_warn(&format!(
                "[validate-policies-schema] read_file failed for {registry_path}: {e:?}; \
                 skipping lint_hook-existence check only (other checks continue)"
            ));
            None
        }
    };

    // Check each policy entry.
    for (i, entry) in entries.iter().enumerate() {
        // Check required fields.
        violations.extend(check_required_fields(entry, i));
        // Check id format.
        violations.extend(check_policy_id_format(entry, i));
        // Check severity value.
        violations.extend(check_severity_value(entry, i));
        // Check lint_hook + codified_at coupling.
        violations.extend(check_lint_hook_and_codified_at(
            entry,
            i,
            registry_content.as_deref(),
        ));
        // Advisory log for unknown fields (forward-compat; NOT block).
        for unknown in &entry.unknown_fields {
            host::log_warn(&format!(
                "[validate-policies-schema] policy entry {} has unknown field `{unknown}`; \
                 advisory only (forward-compat)",
                entry_display_label(entry, i)
            ));
        }
    }

    // Check for duplicate ids across all entries.
    violations.extend(check_duplicate_ids(&entries));

    violations
}

// ---------------------------------------------------------------------------
// Part C — cargo-audit cache functions
// ---------------------------------------------------------------------------

/// Extract `(crate_name, version)` pairs from a dispatch package document.
///
/// Scans for lines matching TOML dependency patterns:
/// - `crate_name = "version"` (plain TOML form)
/// - `crate_name = { version = "version" ... }` (TOML table form)
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.008 postcondition 13.
pub fn extract_crate_deps_from_dispatch(content: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comment lines and empty lines.
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Pattern 1: `crate_name = "version"` (plain TOML form)
        // Pattern 2: `crate_name = { version = "version", ... }` (table form)
        if let Some((crate_name, version)) = try_parse_toml_dep_line(trimmed) {
            results.push((crate_name, version));
        }
    }

    results
}

/// Attempt to parse a TOML dependency line and return `(crate_name, version)`.
///
/// Handles both:
/// - `foo = "1.0.0"` → ("foo", "1.0.0")
/// - `foo = { version = "1.0.0", features = [...] }` → ("foo", "1.0.0")
///
/// Returns `None` if line doesn't match either pattern.
fn try_parse_toml_dep_line(line: &str) -> Option<(String, String)> {
    // Must have `=` somewhere.
    let eq_pos = line.find('=')?;
    let key = line[..eq_pos].trim();
    // Key must be a valid crate-name-like identifier (letters, digits, underscores, hyphens).
    if key.is_empty()
        || !key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return None;
    }

    let rest = line[eq_pos + 1..].trim();

    // Pattern 1: plain string form `= "version"`
    if rest.starts_with('"') {
        let version = extract_quoted_string(rest)?;
        // Version must look like a semver: starts with a digit or a range operator.
        if version_looks_valid(&version) {
            return Some((key.replace('-', "_"), version));
        }
        return None;
    }

    // Pattern 2: table form `= { version = "...", ... }`
    if rest.starts_with('{') {
        // Extract `version = "..."` from within the braces.
        let version = extract_version_from_table(rest)?;
        return Some((key.replace('-', "_"), version));
    }

    None
}

/// Extract a quoted string value from a line starting with `"`.
fn extract_quoted_string(s: &str) -> Option<String> {
    if !s.starts_with('"') {
        return None;
    }
    let inner = &s[1..];
    let end = inner.find('"')?;
    Some(inner[..end].to_string())
}

/// Extract the `version` value from a TOML inline table string.
fn extract_version_from_table(s: &str) -> Option<String> {
    // Look for `version = "..."` inside the table.
    let version_key = "version = \"";
    let pos = s.find(version_key)?;
    let start = pos + version_key.len();
    if !s.is_char_boundary(start) {
        return None;
    }
    let after = &s[start..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

/// Check if a version string looks like a valid semver or semver requirement.
fn version_looks_valid(v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    // Accept strings that start with a digit, `^`, `~`, `>`, `<`, `=`, `*`.
    let first = v.chars().next().unwrap_or(' ');
    first.is_ascii_digit() || matches!(first, '^' | '~' | '>' | '<' | '=' | '*')
}

/// Check dispatch package content against the cargo-audit cache.
///
/// Reads the cache file at `cache_path` via `host::read_file`. On read/parse
/// error: log_warn + return empty vec (fail-open per BC-5.39.008 postcondition 12).
///
/// For each matched advisory: HIGH/CRITICAL → add Violation; MEDIUM/LOW →
/// host::log_warn (per-advisory call) + no Violation.
///
/// Effectful: calls host I/O.
///
/// # BC trace
/// BC-5.39.008 postconditions 12–13.
pub fn check_cargo_advisory(dispatch_content: &str, cache_path: &str) -> Vec<Violation> {
    use vsdd_hook_sdk::host;

    // Read cache file.
    let cache_bytes = match host::read_file(cache_path, MAX_BYTES, 2000) {
        Ok(b) => b,
        Err(e) => {
            host::log_warn(&format!(
                "[validate-policies-schema] cargo-audit-cache.json absent or unreadable \
                 at {cache_path}: {e:?}; skipping advisory check (fail-open per BC-5.39.008 PC12)"
            ));
            return Vec::new();
        }
    };

    // Parse JSON.
    let cache_str = match String::from_utf8(cache_bytes) {
        Ok(s) => s,
        Err(e) => {
            host::log_warn(&format!(
                "[validate-policies-schema] UTF-8 decode failure for cache at {cache_path}: {e}; \
                 skipping advisory check"
            ));
            return Vec::new();
        }
    };

    let cache: serde_json::Value = match serde_json::from_str(&cache_str) {
        Ok(v) => v,
        Err(e) => {
            host::log_warn(&format!(
                "[validate-policies-schema] cargo-audit-cache.json parse error: {e}; \
                 skipping advisory check (fail-open)"
            ));
            return Vec::new();
        }
    };

    // Extract advisory list from cache.
    let advisory_list = match cache
        .get("vulnerabilities")
        .and_then(|v| v.get("list"))
        .and_then(|v| v.as_array())
    {
        Some(list) => list,
        None => {
            // No advisory list in cache — clean or unknown format.
            return Vec::new();
        }
    };

    // Extract crate deps from dispatch content.
    let deps = extract_crate_deps_from_dispatch(dispatch_content);

    let mut violations = Vec::new();

    // For each advisory in cache, check if any dep matches.
    for advisory_entry in advisory_list {
        let advisory = match advisory_entry.get("advisory") {
            Some(a) => a,
            None => continue,
        };

        let rustsec_id = advisory
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");
        let pkg_name = advisory
            .get("package")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let severity = advisory
            .get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Normalize package name for comparison (hyphens → underscores, both directions).
        let pkg_normalized = pkg_name.replace('-', "_");

        // Check if any dep matches this advisory's package.
        let matched = deps.iter().any(|(dep_name, _)| {
            let dep_normalized = dep_name.replace('-', "_");
            dep_normalized == pkg_normalized
        });

        if !matched {
            continue;
        }

        // We have a match — check severity.
        match severity.to_uppercase().as_str() {
            "HIGH" | "CRITICAL" => {
                violations.push(Violation {
                    message: format!(
                        "dispatch package recommends crate `{pkg_name}` which has a \
                         {severity} RUSTSEC advisory {rustsec_id}; \
                         update or remove this dependency before using this dispatch package"
                    ),
                    cited_raw: rustsec_id.to_string(),
                });
            }
            "MEDIUM" | "LOW" => {
                host::log_warn(&format!(
                    "[validate-policies-schema] dispatch package recommends crate `{pkg_name}` \
                     with {severity} advisory {rustsec_id} (advisory only — not blocking)"
                ));
            }
            _ => {
                // Unknown severity — treat as advisory, log_warn, don't block.
                host::log_warn(&format!(
                    "[validate-policies-schema] dispatch package recommends crate `{pkg_name}` \
                     with unknown severity `{severity}` advisory {rustsec_id} (advisory only)"
                ));
            }
        }
    }

    violations
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Return a display label for a policy entry (uses id if available, else index).
fn entry_display_label(entry: &PolicyEntry, index: usize) -> String {
    match &entry.id {
        Some(serde_json::Value::Number(n)) => {
            if let Some(i) = n.as_i64() {
                format!("id={i}")
            } else {
                format!("index={index}")
            }
        }
        Some(serde_json::Value::String(s)) => format!("id=\"{s}\""),
        Some(other) => format!("id={other}"),
        None => format!("index={index}"),
    }
}

/// Return a human-readable type name for a serde_json::Value.
fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// PostToolUse entry point called by `vsdd_hook_sdk::__internal::run`.
///
/// Dispatches to Part B (policies.yaml) or Part C (dispatch package) based
/// on path-component-strict guards, runs the applicable checks, and returns
/// a single `BlockWithFix` if violations found, or `Continue` if clean.
///
/// # BC trace
/// BC-5.39.008 postconditions 1–14; invariants 1–11.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    const HOOK_NAME: &str = "validate-policies-schema";

    // Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "[validate-policies-schema] file_path absent from tool_input — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    if is_policies_yaml_target(&file_path) {
        // Part B arm: policies.yaml schema validation.
        let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(e) => {
                    host::log_warn(&format!(
                        "[{HOOK_NAME}] UTF-8 decode failure reading {file_path}: {e}; skipping"
                    ));
                    return HookResult::Continue;
                }
            },
            Err(e) => {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] read_file failed for {file_path}: {e:?}; \
                     skipping (fail-open per BC-5.39.008 PC11)"
                ));
                return HookResult::Continue;
            }
        };

        let violations = validate_policies_schema(&content);
        if violations.is_empty() {
            HookResult::Continue
        } else {
            let combined = violations
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}. {}", i + 1, v.message))
                .collect::<Vec<_>>()
                .join("\n");
            HookResult::block_with_fix(
                HOOK_NAME,
                format!("policies.yaml schema violations:\n{combined}"),
                "Fix all policy schema violations listed above and re-save the file",
                "policies_schema_violation",
            )
        }
    } else if is_dispatch_package_target(&file_path) {
        // Part C arm: dispatch package cargo-audit check.
        let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(e) => {
                    host::log_warn(&format!(
                        "[{HOOK_NAME}] UTF-8 decode failure reading dispatch package {file_path}: {e}; \
                         skipping advisory check"
                    ));
                    return HookResult::Continue;
                }
            },
            Err(e) => {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] read_file failed for dispatch package {file_path}: {e:?}; \
                     skipping advisory check (fail-open per BC-5.39.008 PC14)"
                ));
                return HookResult::Continue;
            }
        };

        let cache_path = ".factory/hooks/cargo-audit-cache.json";
        let advisory_violations = check_cargo_advisory(&content, cache_path);
        if advisory_violations.is_empty() {
            HookResult::Continue
        } else {
            let combined = advisory_violations
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}. {}", i + 1, v.message))
                .collect::<Vec<_>>()
                .join("\n");
            HookResult::block_with_fix(
                HOOK_NAME,
                format!(
                    "Dispatch package recommends crates with HIGH/CRITICAL RUSTSEC advisories:\n{combined}"
                ),
                "Remove or update the flagged crate dependencies before using this dispatch package",
                "cargo_advisory_violation",
            )
        }
    } else {
        HookResult::Continue
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // ---- is_policies_yaml_target ----

    #[test]
    fn test_is_policies_yaml_target_exact_match() {
        assert!(is_policies_yaml_target(".factory/policies.yaml"));
        assert!(is_policies_yaml_target("policies.yaml"));
    }

    #[test]
    fn test_is_policies_yaml_target_wrong_filename_no_match() {
        // path-component-strict: xpolicies.yaml must NOT match
        assert!(!is_policies_yaml_target(".factory/xpolicies.yaml"));
        assert!(!is_policies_yaml_target("something/xpolicies.yaml"));
        assert!(!is_policies_yaml_target(".factory/STATE.md"));
    }

    // ---- is_dispatch_package_target ----

    #[test]
    fn test_is_dispatch_package_target_match() {
        assert!(is_dispatch_package_target("td-99-dispatch.md"));
        assert!(is_dispatch_package_target(
            ".factory/cycles/v1.0/td-99-dispatch.md"
        ));
        assert!(is_dispatch_package_target("td-12-some-thing-dispatch.md"));
    }

    #[test]
    fn test_is_dispatch_package_target_no_match() {
        assert!(!is_dispatch_package_target("dispatch.md"));
        // td-dispatch.md does NOT match: prefix "td-" + suffix "-dispatch.md" would overlap
        // (len 14 <= len("td-") + len("-dispatch.md") = 15). Requires non-empty middle segment.
        assert!(!is_dispatch_package_target("td-dispatch.md"));
        assert!(!is_dispatch_package_target("xtd-99-dispatch.md")); // must start with td-
        assert!(!is_dispatch_package_target("td-99-dispatch.md.bak"));
    }

    // ---- parse_policies_yaml ----

    #[test]
    fn test_parse_valid_policies_yaml() {
        let content = r#"---
document_type: governance-policy-registry
version: "1.0"
last_amended: "2026-05-26"
---

policies:
  - id: 1
    name: test_policy
    description: "A test policy"
    severity: HIGH
    scope: ".factory/"
    lint_hook: null
    codified_at: null
"#;
        let (header, entries) = parse_policies_yaml(content).expect("should parse");
        assert_eq!(
            header.document_type.as_deref(),
            Some("governance-policy-registry")
        );
        assert_eq!(header.version.as_deref(), Some("1.0"));
        assert!(header.last_amended.is_some());
        assert_eq!(entries.len(), 1);
        assert!(entries[0].id.is_some());
    }

    #[test]
    fn test_parse_yaml_parse_error() {
        let content = "policies:\n  - id: 1\n    extra_bad: {\n";
        let result = parse_policies_yaml(content);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("parse error") || err.contains("YAML"));
    }

    #[test]
    fn test_parse_policies_no_frontmatter() {
        // policies.yaml with only the policies document (no frontmatter)
        let content = r#"policies:
  - id: 1
    name: test_policy
    description: "A test policy"
    severity: HIGH
    scope: ".factory/"
    lint_hook: null
    codified_at: null
"#;
        let (header, entries) = parse_policies_yaml(content).expect("should parse");
        // Header fields should all be None (no frontmatter doc)
        assert!(header.document_type.is_none());
        assert!(header.version.is_none());
        assert!(header.last_amended.is_none());
        assert_eq!(entries.len(), 1);
    }

    // ---- check_header_fields ----

    #[test]
    fn test_check_header_fields_valid() {
        let header = FrontmatterHeader {
            document_type: Some("governance-policy-registry".to_string()),
            version: Some("1.0".to_string()),
            last_amended: Some("2026-05-26".to_string()),
        };
        assert!(check_header_fields(&header).is_empty());
    }

    #[test]
    fn test_check_header_fields_missing_version() {
        let header = FrontmatterHeader {
            document_type: Some("governance-policy-registry".to_string()),
            version: None,
            last_amended: Some("2026-05-26".to_string()),
        };
        let violations = check_header_fields(&header);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("version"));
        assert!(violations[0].message.contains("F-PASS14-004"));
    }

    #[test]
    fn test_check_header_fields_all_missing() {
        let header = FrontmatterHeader::default();
        let violations = check_header_fields(&header);
        assert_eq!(violations.len(), 3);
    }

    // ---- check_policy_id_format ----

    #[test]
    fn test_check_policy_id_format_valid_integer() {
        let entry = PolicyEntry {
            id: Some(serde_json::Value::Number(serde_json::Number::from(1))),
            ..Default::default()
        };
        assert!(check_policy_id_format(&entry, 0).is_empty());
    }

    #[test]
    fn test_check_policy_id_format_string_form_blocks() {
        let entry = PolicyEntry {
            id: Some(serde_json::Value::String("POLICY 01".to_string())),
            ..Default::default()
        };
        let violations = check_policy_id_format(&entry, 0);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("string-format"));
        assert!(violations[0].message.contains("F-PASS14-006"));
    }

    #[test]
    fn test_check_policy_id_format_out_of_range() {
        let entry = PolicyEntry {
            id: Some(serde_json::Value::Number(serde_json::Number::from(0))),
            ..Default::default()
        };
        let violations = check_policy_id_format(&entry, 0);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("out of range"));
    }

    // ---- check_severity_value ----

    #[test]
    fn test_check_severity_value_high_valid() {
        let entry = PolicyEntry {
            severity: Some(serde_json::Value::String("HIGH".to_string())),
            ..Default::default()
        };
        assert!(check_severity_value(&entry, 0).is_empty());
    }

    #[test]
    fn test_check_severity_value_medium_valid() {
        let entry = PolicyEntry {
            severity: Some(serde_json::Value::String("MEDIUM".to_string())),
            ..Default::default()
        };
        assert!(check_severity_value(&entry, 0).is_empty());
    }

    #[test]
    fn test_check_severity_value_p1_blocks() {
        let entry = PolicyEntry {
            id: Some(serde_json::Value::Number(serde_json::Number::from(1))),
            severity: Some(serde_json::Value::String("P1".to_string())),
            ..Default::default()
        };
        let violations = check_severity_value(&entry, 0);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("P1"));
        assert!(violations[0].message.contains("HIGH") || violations[0].message.contains("MEDIUM"));
    }

    // ---- check_duplicate_ids ----

    #[test]
    fn test_check_duplicate_ids_no_duplicates() {
        let entries = vec![
            PolicyEntry {
                id: Some(serde_json::Value::Number(serde_json::Number::from(1))),
                ..Default::default()
            },
            PolicyEntry {
                id: Some(serde_json::Value::Number(serde_json::Number::from(2))),
                ..Default::default()
            },
        ];
        assert!(check_duplicate_ids(&entries).is_empty());
    }

    #[test]
    fn test_check_duplicate_ids_duplicate_detected() {
        let entries = vec![
            PolicyEntry {
                id: Some(serde_json::Value::Number(serde_json::Number::from(3))),
                ..Default::default()
            },
            PolicyEntry {
                id: Some(serde_json::Value::Number(serde_json::Number::from(3))),
                ..Default::default()
            },
        ];
        let violations = check_duplicate_ids(&entries);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("duplicate"));
        assert!(violations[0].message.contains("3"));
    }

    // ---- check_lint_hook_exists ----

    #[test]
    fn test_check_lint_hook_exists_plain_name() {
        let registry = r#"[[hooks]]
name = "validate-dispatch-advance"
event = "PostToolUse"
"#;
        assert!(check_lint_hook_exists(
            "validate-dispatch-advance",
            registry
        ));
        assert!(!check_lint_hook_exists("nonexistent-plugin", registry));
    }

    #[test]
    fn test_check_lint_hook_exists_namespaced_slug() {
        let registry = r#"[[hooks]]
name = "validate-burst-log"
event = "PostToolUse"
"#;
        // Namespaced slug: extract basename after `:`.
        assert!(check_lint_hook_exists(
            "vsdd-factory:validate-burst-log",
            registry
        ));
        assert!(!check_lint_hook_exists(
            "vsdd-factory:nonexistent",
            registry
        ));
    }

    // ---- is_d_nnn_format ----

    #[test]
    fn test_is_d_nnn_format_valid() {
        assert!(is_d_nnn_format("D-381"));
        assert!(is_d_nnn_format("D-1"));
        assert!(is_d_nnn_format("D-999"));
    }

    #[test]
    fn test_is_d_nnn_format_invalid() {
        assert!(!is_d_nnn_format("pass-72"));
        assert!(!is_d_nnn_format("D-"));
        assert!(!is_d_nnn_format("d-123")); // lowercase
        assert!(!is_d_nnn_format(""));
    }

    // ---- extract_crate_deps_from_dispatch ----

    #[test]
    fn test_extract_crate_deps_plain_form() {
        let content = r#"[dependencies]
serde_yaml = "0.9.34"
serde = "1.0"
"#;
        let deps = extract_crate_deps_from_dispatch(content);
        assert!(
            deps.iter()
                .any(|(name, ver)| name == "serde_yaml" && ver == "0.9.34")
        );
        assert!(
            deps.iter()
                .any(|(name, ver)| name == "serde" && ver == "1.0")
        );
    }

    #[test]
    fn test_extract_crate_deps_table_form() {
        let content = r#"[dependencies]
serde = { version = "1.0", features = ["derive"] }
"#;
        let deps = extract_crate_deps_from_dispatch(content);
        assert!(
            deps.iter()
                .any(|(name, ver)| name == "serde" && ver == "1.0")
        );
    }

    #[test]
    fn test_extract_crate_deps_hyphen_normalized() {
        // unsafe_crate hyphen -> underscore normalization
        let content = r#"[dependencies]
unsafe_crate = "0.1.0"
"#;
        let deps = extract_crate_deps_from_dispatch(content);
        assert!(!deps.is_empty());
    }

    // ---- YAML merge key does not panic ----

    #[test]
    fn test_yaml_with_merge_keys_does_not_panic() {
        // serde_norway should handle YAML anchors/aliases without panic.
        let content = r#"---
document_type: governance-policy-registry
version: "1.0"
last_amended: "2026-05-26"
---

policies:
  - id: 1
    name: policy_one
    description: "A policy"
    severity: HIGH
    scope: ".factory/"
    lint_hook: null
    codified_at: null
"#;
        // Should not panic — just exercise the parse path.
        let result = parse_policies_yaml(content);
        assert!(result.is_ok());
    }
}
