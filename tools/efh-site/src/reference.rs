//! Reader sections shared by HTML and the stable function JSON.
use crate::{markdown, util::*};
use regex::Regex;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path, sync::LazyLock};

pub const SECTIONS: [(&str, &str); 6] = [
    ("what-it-does", "What it does"),
    ("try-it", "Try it"),
    ("quirks", "Quirks"),
    ("how-to-implement", "How to implement"),
    ("version-differences", "Version differences"),
    ("alternatives", "Alternatives"),
];

pub fn load(repo: &Path, f: &Value) -> Result<Value, String> {
    let id = s(&f["function_id"]);
    let owned = json(&repo.join(format!("content/reference/{id}.json")))?;
    validate(repo, id, &owned)?;
    Ok(owned)
}

fn validate(repo: &Path, id: &str, owned: &Value) -> Result<(), String> {
    if owned["schema"] != "efh.reference/v1" || owned["function_id"] != id {
        return Err(format!("{id}: invalid handbook reference identity"));
    }
    let formula = s(&owned["try_it"]["formula"]);
    if !formula.starts_with('=') || formula.len() > 4096 {
        return Err(format!(
            "{id}: Try it needs a formula starting with =, at most 4096 bytes"
        ));
    }
    if let Some(inputs) = owned["try_it"].get("inputs") {
        if inputs.as_object().is_none_or(|cells| cells.len() > 256) {
            return Err(format!(
                "{id}: Try it inputs must be an object with at most 256 cells"
            ));
        }
    }
    if let Some(signature) = owned.get("signature") {
        if !signature.is_null()
            && (signature["display"].as_str().is_none_or(str::is_empty)
                || !signature["parameters"].is_array())
        {
            return Err(format!("{id}: signature needs a display and parameters"));
        }
    }
    if let Some(status) = owned["try_it"].get("preview_status") {
        if !matches!(
            status.as_str(),
            Some("calculated" | "worksheet-error" | "refused")
        ) {
            return Err(format!("{id}: unknown preview status"));
        }
    }
    for (field, keys) in [
        (
            "versions",
            &["excel_build", "platform", "behaviour", "sources"][..],
        ),
        (
            "implementations",
            &["language", "flavour", "source", "suite"][..],
        ),
        (
            "variants",
            &["name", "difference", "error_bound", "onecalc_udf"][..],
        ),
    ] {
        let records = owned[field]
            .as_array()
            .ok_or_else(|| format!("{id}: {field} must be an array"))?;
        for record in records {
            if !record.is_object() || keys.iter().any(|k| record.get(k).is_none()) {
                return Err(format!("{id}: {field} record needs {}", keys.join(", ")));
            }
            let nonempty = |key: &str| record[key].as_str().is_some_and(|s| !s.trim().is_empty());
            match field {
                "versions" => {
                    if !nonempty("behaviour")
                        || ["excel_build", "platform"]
                            .iter()
                            .any(|k| !record[k].is_null() && !nonempty(k))
                        || record["sources"].as_array().is_none_or(|sources| {
                            sources.is_empty()
                                || sources.iter().any(|s| s.as_str().is_none_or(str::is_empty))
                        })
                    {
                        return Err(format!(
                            "{id}: build observations need typed scope, behaviour and sources"
                        ));
                    }
                }
                "implementations" => {
                    if ["language", "flavour", "source", "suite"]
                        .iter()
                        .any(|k| !nonempty(k))
                    {
                        return Err(format!("{id}: a listed implementation needs a language, source and published suite"));
                    }
                    let suite = s(&record["suite"]);
                    let Some(path) = normalize(Path::new(suite)) else {
                        return Err(format!("{id}: invalid suite path"));
                    };
                    if !path.starts_with(format!("vectors/{id}"))
                        || !suite.ends_with("/MANIFEST.json")
                        || !repo.join(path).is_file()
                    {
                        return Err(format!(
                            "{id}: implementation suite manifest is not published: {suite}"
                        ));
                    }
                }
                "variants" => {
                    if !nonempty("name") || !nonempty("difference") {
                        return Err(format!("{id}: a variant needs a name and difference"));
                    }
                    if !record["error_bound"].is_null() {
                        let bound = &record["error_bound"];
                        if !bound.is_object()
                            || ["metric", "domain", "basis"]
                                .iter()
                                .any(|k| bound[k].as_str().is_none_or(str::is_empty))
                            || bound
                                .get("bound")
                                .is_none_or(|v| !(v.is_null() || v.is_number() || v.is_string()))
                        {
                            return Err(format!(
                                "{id}: an error bound needs metric, bound, domain and basis"
                            ));
                        }
                    }
                    if !record["onecalc_udf"].is_null() {
                        let hook = &record["onecalc_udf"];
                        if !hook.is_object()
                            || ["name", "language", "source"]
                                .iter()
                                .any(|k| hook[k].as_str().is_none_or(str::is_empty))
                        {
                            return Err(format!(
                                "{id}: a UDF hook needs name, language and source"
                            ));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

/// Existing essays are divided by subject, never placed above the reader's explanation.
/// Editorial/admin paragraphs stay in their source documents, not in the reference body.
pub fn sections(source: &str) -> BTreeMap<String, String> {
    let normalized = source.replace("\r\n", "\n");
    let source = markdown::body(&normalized);
    static GENERATED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?s)## Notes\s*<!-- efh:notes begin:.*?<!-- efh:notes end -->").unwrap()
    });
    let source = GENERATED.replace_all(source, "").into_owned();
    let mut parts = BTreeMap::<String, String>::new();
    let mut key = "what-it-does";
    let mut blocks = Vec::new();
    let mut heading = String::new();
    let flush = |key: &str,
                 heading: &str,
                 blocks: &mut Vec<String>,
                 parts: &mut BTreeMap<String, String>| {
        if key.is_empty() {
            blocks.clear();
            return;
        }
        let text = reader_prose(&blocks.join("\n"));
        if !text.trim().is_empty() {
            let slot = parts.entry(key.into()).or_default();
            if !heading.is_empty() {
                *slot += &format!("\n### {heading}\n\n");
            }
            *slot += &text;
            *slot += "\n\n";
        }
        blocks.clear();
    };
    for line in source.lines() {
        if line.starts_with("# ") {
            continue;
        }
        if let Some(h) = line.strip_prefix("## ") {
            flush(key, &heading, &mut blocks, &mut parts);
            heading = h.replace(
                "The admission table, side by side with AVERAGE",
                "Values included by AVERAGE and AVERAGEA",
            );
            key = match h {
                "What it computes" | "What it does" | "Arguments" => "what-it-does",
                "Try it" => "try-it",
                "Numerical notes" | "Notes for implementers" | "How to implement" => {
                    "how-to-implement"
                }
                "Relationships" | "Alternatives" => "alternatives",
                "Excel versions, platforms and CPUs" | "Version differences" => {
                    "version-differences"
                }
                "Sources" => "sources",
                "What has not been checked" | "Page vocabulary" | "Notes" => "",
                _ => "quirks",
            };
            if matches!(
                h,
                "What it computes"
                    | "What it does"
                    | "Try it"
                    | "Quirks"
                    | "Sources"
                    | "How to implement"
                    | "Alternatives"
                    | "Version differences"
            ) {
                heading.clear();
            }
        } else {
            blocks.push(line.into());
        }
    }
    flush(key, &heading, &mut blocks, &mut parts);
    parts
}

fn reader_prose(text: &str) -> String {
    static ADMIN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\b(projection|projected|registry|admission|reference.engine|lane.csv|curation|curated_at|contract.row|axis.provenance|export.only|export.snapshot|oxfunc|handbook.voice|page vocabulary|ledger|this checkout|this pass|source organ|provenance|this revision|KernelSignatureClass|ErrorAlgebra|ErrorCollapseProfile|ArgPreparationProfile|LiftBroadcastProfile|admission_interface_kind)\b").unwrap()
    });
    text.split("\n\n")
        .filter(|p| !ADMIN.is_match(p) && !p.trim().starts_with("<!--"))
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn record(repo: &Path, f: &Value, owned: &Value, notes: &[Value]) -> Result<Value, String> {
    let id = s(&f["function_id"]);
    let path = repo.join(format!("content/functions/{id}.md"));
    let mut sections = if path.is_file() {
        sections(&read(&path)?)
    } else {
        BTreeMap::new()
    };
    if let Some(overrides) = owned["sections"].as_object() {
        for (key, text) in overrides {
            if !SECTIONS.iter().any(|(id, _)| id == key) || !text.is_string() {
                return Err(format!("{id}: unknown section {key:?} or non-string prose"));
            }
            sections.insert(key.clone(), s(text).into());
        }
    }
    sections.entry("what-it-does".into()).or_insert_with(|| {
        let description = s(&f["descriptions"]["short"]);
        if description.is_empty() {
            "A worksheet operator. Detailed guidance is still being written.".into()
        } else {
            format!("{description}.\n\nDetailed argument guidance is still being written.")
        }
    });
    for (key, fallback) in [
        ("quirks", "No additional quirks are documented here yet."),
        ("how-to-implement", "Implementation guidance is still being written. Check argument coercion, errors and array behaviour as well as the central calculation."),
        ("version-differences", "No comparison between specific Excel builds is recorded here yet. This does not establish that all builds behave alike."),
        ("alternatives", "Alternative functions and algorithms are still being documented."),
    ] { sections.entry(key.into()).or_insert_with(|| fallback.into()); }
    let attached: Vec<_> = notes
        .iter()
        .filter(|n| {
            n["functions"]
                .as_array()
                .is_some_and(|fs| fs.iter().any(|v| s(v) == id))
        })
        .collect();
    Ok(json!({
        "schema":"efh.function-reference/v1", "function_id":id, "name":f["surface_name"], "display_name":owned.get("display_name").unwrap_or(&f["surface_name"]),
        "category":f["category"], "entry_kind":f["entry_kind"],
        "url":format!("https://handbook.dnacalc.com/functions/{}.html",id.trim_start_matches("FUNC.")),
        "json_url":format!("https://handbook.dnacalc.com/api/functions/{id}.json"),
        "signature":owned.get("signature").cloned().unwrap_or_else(|| if f["signature_placeholder"] == true { Value::Null } else { f["signature"].clone() }),
        "section_order":SECTIONS.iter().map(|(id,_)| *id).collect::<Vec<_>>(),
        "sections":sections, "try_it":owned["try_it"],
        "versions":owned["versions"], "implementations":owned["implementations"], "variants":owned["variants"],
        "notes":attached, "sources":{"microsoft":f["docs"]["microsoft_url"], "metadata":f},
        "eval_api":{"url":"https://handbook.dnacalc.com/api/eval","method":"POST"}
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn implementation_records_cannot_name_an_unpublished_suite() {
        let record = serde_json::json!({"schema":"efh.reference/v1","function_id":"FUNC.EXP","try_it":{"formula":"=EXP(1)"},"versions":[],"implementations":[{"language":"Rust","flavour":"math-correct","source":"implementations/FUNC.EXP/rust.rs","suite":"vectors/FUNC.EXP/v99999/MANIFEST.json"}],"variants":[]});
        assert!(
            validate(Path::new(env!("CARGO_MANIFEST_DIR")), "FUNC.EXP", &record)
                .unwrap_err()
                .contains("not published")
        );
    }
    #[test]
    fn typed_missingness_and_variant_bounds_are_distinct_from_accuracy_claims() {
        let mut record = serde_json::json!({"schema":"efh.reference/v1","function_id":"FUNC.EXP","try_it":{"formula":"=EXP(1)"},"versions":[{"excel_build":null,"platform":null,"behaviour":"Build not recorded.","sources":["https://example.invalid/capture"]}],"implementations":[],"variants":[{"name":"EXP.PROPOSED","difference":"A proposal, not an implementation.","error_bound":null,"onecalc_udf":null}]});
        validate(Path::new("."), "FUNC.EXP", &record).unwrap();
        record["variants"][0]["error_bound"] = serde_json::json!({"bound":0.5});
        assert!(validate(Path::new("."), "FUNC.EXP", &record)
            .unwrap_err()
            .contains("metric, bound, domain and basis"));
        record["variants"][0]["error_bound"] = Value::Null;
        record["versions"][0]["sources"] = serde_json::json!([]);
        assert!(validate(Path::new("."), "FUNC.EXP", &record)
            .unwrap_err()
            .contains("sources"));
    }
    #[test]
    fn reader_order_ignores_administration_and_keeps_mathematical_details() {
        let p=sections("---\na: b\n---\n# ROUND\n\n## What it computes\n\nRound a number.\n\nThe projection is old.\n\n## Arguments\n\nTwo numeric slots.\n\n## Numerical notes\n\nUse decimal digits.\n\n## Page vocabulary\n\nInternal label.\n\n## Sources\n\nA book.");
        assert!(p["what-it-does"].contains("Two numeric slots"));
        assert!(!p["what-it-does"].contains("projection"));
        assert!(p["how-to-implement"].contains("Use decimal digits"));
        assert!(!p.values().any(|x| x.contains("Internal label")));
    }
}
