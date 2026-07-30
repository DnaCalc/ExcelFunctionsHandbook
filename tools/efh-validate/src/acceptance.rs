//! The three acceptance tests FOUNDATION 6 line 1181 puts on T1, plus the corrections cross-check.
//!
//!   (a) Every schema file parses as JSON Schema 2020-12.
//!   (b) FIELD COVERAGE - every field the 3.7 pseudocode reads exists in the schema it is read
//!       from, with a compatible type. "this is the test that would have caught A3-F1."
//!   (c) No field declared in a content/ schema is a total function of a data/ schema, by an
//!       explicit allow-list of the three deleted fields (A3-S8).

use serde_json::Value;
use std::collections::BTreeSet;

use crate::json_schema::declared_types;
use crate::registry::{all_declared_property_names, resolve_path, Registry};
use crate::rubric_field_reads::{
    CorrectionCheck, ReadKind, ReadSource, CORRECTION_CHECKS, EXPECTED_UNRESOLVED, FIELD_READS,
    TOTAL_FUNCTION_ALLOW_LIST,
};

pub struct TestOutcome {
    pub name: &'static str,
    pub headline: String,
    pub failures: Vec<String>,
    pub notes: Vec<String>,
}

impl TestOutcome {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}

// ------------------------------------------------------------------ (a)

pub fn test_a_schemas_parse(reg: &Registry) -> TestOutcome {
    let mut failures = Vec::new();
    let mut notes = Vec::new();
    for (name, schema) in &reg.schemas {
        let f = schema.conformance_findings();
        if f.is_empty() {
            notes.push(format!(
                "  ok   {:<38} {:<5} {}",
                name,
                schema.efh_str("family").unwrap_or("?"),
                schema.efh_str("path_glob").unwrap_or("?")
            ));
        } else {
            for finding in f {
                failures.push(format!("{} at {}: {}", name, finding.path, finding.message));
            }
        }
    }
    TestOutcome {
        name: "(a) every schema file parses as JSON Schema 2020-12",
        headline: format!(
            "{} schema files; dialect, keyword, type, $ref, pattern and key-order-annotation \
             conformance",
            reg.schemas.len()
        ),
        failures,
        notes,
    }
}

// ------------------------------------------------------------------ (b)

pub fn test_b_field_coverage(reg: &Registry) -> TestOutcome {
    let mut failures = Vec::new();
    let mut notes = Vec::new();
    let mut resolved = 0usize;
    let mut unresolved_narrative = 0usize;

    for fr in FIELD_READS {
        let label = format!("{} -> {}::{}", fr.reader, fr.schema_file, fr.path);
        match fr.kind {
            ReadKind::NarrativeUnresolved => {
                unresolved_narrative += 1;
                notes.push(format!(
                    "  GAP  {} [FOUNDATION:{}]\n       {}",
                    fr.reader,
                    fr.foundation_lines,
                    first_line(fr.note)
                ));
                continue;
            }
            ReadKind::MustNotExist => {
                let schema = match reg.get(fr.schema_file) {
                    Some(s) => s,
                    None => {
                        failures.push(format!("{}: no such schema file", label));
                        continue;
                    }
                };
                match resolve_path(schema, fr.path) {
                    Ok(_) => failures.push(format!(
                        "{}: field IS declared but FOUNDATION:{} forbids reading it, so it must \
                         have no schema slot at all",
                        label, fr.foundation_lines
                    )),
                    Err(_) => {
                        resolved += 1;
                        // the prohibition must also be documented at the point of use
                        let documented = schema
                            .root
                            .get("efhForbiddenProperties")
                            .and_then(|v| v.as_object())
                            .map(|m| m.contains_key(fr.path))
                            .unwrap_or(false);
                        if !documented {
                            failures.push(format!(
                                "{}: correctly absent, but `{}` is not named in the schema's \
                                 efhForbiddenProperties, so a future editor has no warning",
                                label, fr.path
                            ));
                        }
                    }
                }
                continue;
            }
            ReadKind::ForbiddenRead => {
                let schema = match reg.get(fr.schema_file) {
                    Some(s) => s,
                    None => {
                        failures.push(format!("{}: no such schema file", label));
                        continue;
                    }
                };
                match resolve_path(schema, fr.path) {
                    Ok(node) => {
                        resolved += 1;
                        if node.get("efhForbiddenRead") != Some(&Value::Bool(true)) {
                            failures.push(format!(
                                "{}: exists but is not marked `efhForbiddenRead: true`; guard G-9 \
                                 (FOUNDATION:{}) must travel with the schema",
                                label, fr.foundation_lines
                            ));
                        }
                        check_types(fr, node, schema, &mut failures, &label);
                    }
                    Err(e) => failures.push(format!("{}: {}", label, e)),
                }
                continue;
            }
            ReadKind::Read | ReadKind::PresentButNotRubricInput => {}
        }

        let schema = match reg.get(fr.schema_file) {
            Some(s) => s,
            None => {
                failures.push(format!("{}: no such schema file in tools/schemas/", label));
                continue;
            }
        };
        match resolve_path(schema, fr.path) {
            Ok(node) => {
                resolved += 1;
                check_types(fr, node, schema, &mut failures, &label);
            }
            Err(e) => failures.push(format!(
                "{}: {} -- FOUNDATION:{} reads it: {}",
                label,
                e,
                fr.foundation_lines,
                first_line(fr.quote)
            )),
        }
    }

    if unresolved_narrative != EXPECTED_UNRESOLVED {
        failures.push(format!(
            "the rubric has {} field reads that no schema declares; the checked-in expectation is \
             {}. A rise is finding A3-F1 recurring. A fall means a gap was closed and \
             EXPECTED_UNRESOLVED must be lowered in the same commit.",
            unresolved_narrative, EXPECTED_UNRESOLVED
        ));
    }

    let by_source = |s: ReadSource| FIELD_READS.iter().filter(|f| f.source == s).count();
    notes.push(format!(
        "  {} named field reads resolved against their schema with a compatible type",
        resolved
    ));
    notes.push(format!(
        "  sources: {} from the 3.7 pseudocode, {} from its invariants, {} from label templates, \
         {} rendering mandates, {} steward decisions, {} overclaim tests",
        by_source(ReadSource::Pseudocode37),
        by_source(ReadSource::Invariant37),
        by_source(ReadSource::LabelTemplate),
        by_source(ReadSource::RenderingMandate),
        by_source(ReadSource::StewardDecision),
        by_source(ReadSource::OverclaimTest),
    ));

    TestOutcome {
        name: "(b) field coverage: every field the 3.7 pseudocode reads exists, with a compatible \
               type",
        headline: format!(
            "{} enumerated reads: {} resolved, {} declared-must-not-exist, {} narrative gaps \
             (expected {})",
            FIELD_READS.len(),
            resolved,
            FIELD_READS.iter().filter(|f| f.kind == ReadKind::MustNotExist).count(),
            unresolved_narrative,
            EXPECTED_UNRESOLVED
        ),
        failures,
        notes,
    }
}

fn check_types(
    fr: &crate::rubric_field_reads::FieldRead,
    node: &Value,
    schema: &crate::json_schema::Schema,
    failures: &mut Vec<String>,
    label: &str,
) {
    if fr.expect_types.is_empty() {
        return;
    }
    let declared: BTreeSet<String> = declared_types(schema, node);
    if declared.is_empty() {
        failures.push(format!(
            "{}: the schema declares no `type` for this field, so the read is unchecked",
            label
        ));
        return;
    }
    for want in fr.expect_types {
        let ok = declared.contains(*want)
            || (*want == "integer" && declared.contains("number"));
        if !ok {
            failures.push(format!(
                "{}: the reader needs `{}` but the schema permits only {:?} -- FOUNDATION:{}",
                label,
                want,
                declared.iter().collect::<Vec<_>>(),
                fr.foundation_lines
            ));
        }
    }
}

fn first_line(s: &str) -> String {
    let flat: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() > 160 {
        format!("{}...", flat.chars().take(157).collect::<String>())
    } else {
        flat
    }
}

// ------------------------------------------------------------------ (c)

pub fn test_c_no_content_field_is_a_total_function(reg: &Registry) -> TestOutcome {
    let mut failures = Vec::new();
    let mut notes = Vec::new();

    // 1. Each allow-listed field must be absent from the content/ schema it was deleted from,
    //    and must be named in that schema's efhForbiddenProperties so the reason travels with it.
    for tf in TOTAL_FUNCTION_ALLOW_LIST {
        let schema = match reg.get(tf.content_schema) {
            Some(s) => s,
            None => {
                failures.push(format!("{}: no such schema file", tf.content_schema));
                continue;
            }
        };
        let mut names = Vec::new();
        all_declared_property_names(&schema.root, &mut names);
        if names.iter().any(|n| n == tf.field) {
            failures.push(format!(
                "{} declares `{}`, which A3-S8 (FOUNDATION:{}) deletes: it is a total function of \
                 {}",
                tf.content_schema, tf.field, tf.foundation_lines, tf.computed_from
            ));
        }
        let documented = schema
            .root
            .get("efhForbiddenProperties")
            .and_then(|v| v.as_object())
            .map(|m| m.keys().any(|k| k == tf.field || k.ends_with(&format!(".{}", tf.field))))
            .unwrap_or(false);
        if !documented {
            failures.push(format!(
                "{} does not name `{}` in efhForbiddenProperties; the A3-S8 prohibition must be \
                 readable at the point where someone would add it back",
                tf.content_schema, tf.field
            ));
        } else {
            notes.push(format!(
                "  ok   {}::{}\n       absent and documented; computed at build time from {}\n\
                 \x20      FOUNDATION:{} - {}",
                tf.content_schema,
                tf.field,
                tf.computed_from,
                tf.foundation_lines,
                first_line(tf.quote)
            ));
        }
    }

    // 2. No content/ schema anywhere may declare an allow-listed name.
    for (name, schema) in &reg.schemas {
        if schema.efh_str("organ") != Some("content") {
            continue;
        }
        let mut names = Vec::new();
        all_declared_property_names(&schema.root, &mut names);
        for tf in TOTAL_FUNCTION_ALLOW_LIST {
            if names.iter().any(|n| n == tf.field) && name != tf.content_schema {
                failures.push(format!(
                    "{} declares `{}`, a field A3-S8 deletes from content/ everywhere",
                    name, tf.field
                ));
            }
        }
    }

    // 3. Reported, not fatal: content/ property names that also exist in a data/ schema. Most are
    //    legitimate joins (function_id, schema, surface_name); the list is printed so a future
    //    total-function candidate is visible rather than invisible.
    let mut data_names: BTreeSet<String> = BTreeSet::new();
    for schema in reg.schemas.values() {
        if schema.efh_str("organ") == Some("data") {
            let mut v = Vec::new();
            all_declared_property_names(&schema.root, &mut v);
            data_names.extend(v);
        }
    }
    let mut collisions: BTreeSet<String> = BTreeSet::new();
    for schema in reg.schemas.values() {
        if schema.efh_str("organ") == Some("content") {
            let mut v = Vec::new();
            all_declared_property_names(&schema.root, &mut v);
            for n in v {
                if data_names.contains(&n) {
                    collisions.insert(n);
                }
            }
        }
    }
    notes.push(format!(
        "  reported: {} property names appear in both a data/ and a content/ schema: {}",
        collisions.len(),
        collisions.iter().cloned().collect::<Vec<_>>().join(", ")
    ));
    notes.push(
        "  these are join keys and shared vocabulary, not derived values; the check that matters \
         is the allow-list above, per FOUNDATION line 81."
            .to_string(),
    );

    TestOutcome {
        name: "(c) no field declared in a content/ schema is a total function of a data/ schema",
        headline: format!(
            "explicit allow-list of the {} fields A3-S8 deletes",
            TOTAL_FUNCTION_ALLOW_LIST.len()
        ),
        failures,
        notes,
    }
}

// ------------------------------------------------------------------ corrections cross-check

pub fn test_d_corrections(reg: &Registry) -> TestOutcome {
    let mut failures = Vec::new();
    let mut notes = Vec::new();
    for c in CORRECTION_CHECKS {
        match check_one_correction(reg, c) {
            Ok(()) => notes.push(format!("  ok   {:<32} {}::{}", c.id, c.schema_file, c.path)),
            Err(e) => failures.push(format!("{} [{}::{}]: {}", c.id, c.schema_file, c.path, e)),
        }
    }
    TestOutcome {
        name: "(d) ATTRIBUTION-CORRECTIONS 7 is encoded where it contradicts FOUNDATION 3.4/3.7",
        headline: format!(
            "{} corrected values must be carriable by the schema set; OT-15 is INVERTED",
            CORRECTION_CHECKS.len()
        ),
        failures,
        notes,
    }
}

fn check_one_correction(reg: &Registry, c: &CorrectionCheck) -> Result<(), String> {
    let schema = reg.get(c.schema_file).ok_or("no such schema file")?;
    let node = resolve_path(schema, c.path).map_err(|e| e.to_string())?;
    let vals = node
        .get("enum")
        .and_then(|v| v.as_array())
        .ok_or("node declares no enum")?;
    let hit = vals.iter().any(|v| v.as_str() == Some(c.must_contain));
    if hit {
        Ok(())
    } else {
        Err(format!(
            "enum does not admit `{}`. {}",
            c.must_contain,
            first_line(c.why)
        ))
    }
}
