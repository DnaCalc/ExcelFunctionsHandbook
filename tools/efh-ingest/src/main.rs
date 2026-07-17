//! efh-ingest: builds the Handbook's `data/` projections from OxFunc truth surfaces.
//!
//! Row spine: OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv (the 534 published rows).
//! Enrichment: the OxFunc runtime registry (canonical consumption path) where the row
//! exists there, plus the deferred inventory, the English docs catalog, the localization
//! seed, and the correlation ledger. OxFunc is read read-only; output is deterministic
//! (byte-stable for a fixed OxFunc state) and carries no timestamps.

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize)]
struct FunctionDoc {
    function_id: String,
    surface_name: String,
    /// The published export row this entry came from. Differs from `function_id`
    /// only for entries split out of a combined byte-variant documentation row
    /// (e.g. `FUNC.FIND` and `FUNC.FINDB` both come from `FUNC.FIND, FINDB`).
    published_row_id: String,
    entry_kind: String,
    category: Option<String>,
    admission: Admission,
    metadata_status: Option<String>,
    xlcall: Option<Xlcall>,
    arity: ArityDoc,
    classification: BTreeMap<String, String>,
    axes: Option<Axes>,
    signature: Option<SignatureDoc>,
    signature_placeholder: bool,
    descriptions: Descriptions,
    docs: Option<DocsLink>,
    localized_names: Vec<LocalizedName>,
    artifacts: Option<Artifacts>,
    version_marker: Option<String>,
    special_interface_kind: Option<String>,
    admission_interface_kind: Option<String>,
    registry_backed: bool,
}

#[derive(Serialize)]
struct Admission {
    label: String,
    reason: Option<String>,
}

#[derive(Serialize)]
struct Xlcall {
    symbol: String,
    code: String,
}

#[derive(Serialize)]
struct ArityDoc {
    min: Option<u64>,
    max: Option<u64>,
}

#[derive(Serialize)]
struct Axes {
    lift_broadcast_profile: String,
    precision_rounding_profile: String,
    real_result_policy: String,
    axes_version: String,
    semantic_kernel_version: String,
    arg_admission_version: String,
}

#[derive(Serialize)]
struct SignatureDoc {
    display: String,
    trailing_repeats: bool,
    parameters: Vec<ParameterDoc>,
}

#[derive(Serialize)]
struct ParameterDoc {
    name: String,
    optional: bool,
    repeats: bool,
    description: Option<String>,
}

#[derive(Serialize)]
struct Descriptions {
    short: Option<String>,
    long: Option<String>,
}

#[derive(Serialize)]
struct DocsLink {
    microsoft_url: String,
    english_description: Option<String>,
}

#[derive(Serialize)]
struct LocalizedName {
    locale: String,
    name: String,
}

#[derive(Serialize)]
struct Artifacts {
    contract_row_id: Option<String>,
    lean_module: Option<String>,
    rust_module: Option<String>,
    status: Option<String>,
}

#[derive(Serialize)]
struct IndexDoc {
    provenance: Provenance,
    counts: Counts,
    rows: Vec<IndexRow>,
}

#[derive(Serialize)]
struct Provenance {
    export_snapshot_id: String,
    export_snapshot_generation: String,
    export_source_commit: String,
    export_source_tree_state: String,
    oxfunc_commit_at_ingest: String,
    registry_snapshot_family: String,
    registry_generation: u64,
    registry_content_fingerprint: String,
    tool: String,
}

#[derive(Serialize)]
struct Counts {
    total_entries: usize,
    published_rows: usize,
    split_combined_rows: usize,
    functions: usize,
    operators: usize,
    registry_backed: usize,
    export_only: usize,
    deferred: usize,
    placeholder_signatures: usize,
    with_docs_link: usize,
    with_localized_names: usize,
}

#[derive(Serialize)]
struct IndexRow {
    function_id: String,
    surface_name: String,
    entry_kind: String,
    category: Option<String>,
    admission: String,
}

fn none_if_empty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn read_csv(path: &Path) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(&raw);
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(raw.as_bytes());
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record?;
        rows.push(record.iter().map(|f| f.to_string()).collect());
    }
    Ok((headers, rows))
}

fn col(headers: &[String], name: &str) -> usize {
    headers
        .iter()
        .position(|h| h == name)
        .unwrap_or_else(|| panic!("column {name} not found in {headers:?}"))
}

fn guid_of_url(url: &str) -> Option<String> {
    if url.len() < 36 {
        return None;
    }
    let tail = &url[url.len() - 36..];
    let ok = tail.chars().enumerate().all(|(i, c)| match i {
        8 | 13 | 18 | 23 => c == '-',
        _ => c.is_ascii_hexdigit(),
    });
    if ok { Some(tail.to_ascii_lowercase()) } else { None }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("tools/efh-ingest layout")
        .to_path_buf();
    let oxfunc_root = std::env::var("EFH_OXFUNC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.parent().expect("DnaCalc tree").join("OxFunc"));
    let lane = oxfunc_root.join("docs").join("function-lane");
    let data_dir = repo_root.join("data");
    let functions_dir = data_dir.join("functions");
    if functions_dir.exists() {
        fs::remove_dir_all(&functions_dir)?;
    }
    fs::create_dir_all(&functions_dir)?;

    // Row spine: the published snapshot export (534 rows).
    let (eh, export_rows) = read_csv(&lane.join("OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv"))?;
    let e_id = col(&eh, "surface_stable_id");
    let e_kind = col(&eh, "entry_kind");
    let e_name = col(&eh, "canonical_surface_name");
    let e_cat = col(&eh, "category");
    let e_status = col(&eh, "metadata_status");
    let e_sym = col(&eh, "xlcall_builtin_symbol");
    let e_code = col(&eh, "xlcall_builtin_code");
    let e_amin = col(&eh, "arity_min");
    let e_amax = col(&eh, "arity_max");
    let e_vm = col(&eh, "version_marker");
    let e_sik = col(&eh, "special_interface_kind");
    let e_aik = col(&eh, "admission_interface_kind");
    let e_snap = col(&eh, "snapshot_id");
    let e_gen = col(&eh, "snapshot_generation");
    let e_commit = col(&eh, "source_commit_short");
    let e_tree = col(&eh, "source_tree_state");
    let class_cols = [
        "determinism_class",
        "volatility_class",
        "host_interaction_class",
        "thread_safety_class",
        "arg_preparation_profile",
        "coercion_lift_profile",
        "kernel_signature_class",
        "fec_dependency_profile",
        "surface_fec_dependency_profile",
    ];
    let class_idx: Vec<(String, usize)> = class_cols
        .iter()
        .map(|c| (c.to_string(), col(&eh, c)))
        .collect();

    // Deferred inventory: surface name -> reason.
    let (dh, drows) = read_csv(&lane.join("W50_DEFERRED_CURRENT_VERSION_INVENTORY.csv"))?;
    let d_name = col(&dh, "entry_name");
    let d_notes = col(&dh, "notes");
    let deferred: BTreeMap<String, String> = drows
        .iter()
        .map(|r| (r[d_name].clone(), r[d_notes].clone()))
        .collect();

    // English docs catalog: surface name -> (url, description).
    let (ch, crows) = read_csv(&lane.join("FUNCTION_CATALOG_CURRENT_BASELINE_LOCAL.csv"))?;
    let c_name = col(&ch, "function_name");
    let c_url = col(&ch, "function_url");
    let c_desc = col(&ch, "description");
    let mut catalog: BTreeMap<String, (String, Option<String>)> = BTreeMap::new();
    for r in &crows {
        catalog.insert(r[c_name].clone(), (r[c_url].clone(), none_if_empty(&r[c_desc])));
    }

    // Localization seed: article guid -> [(locale, localized name)].
    let (lh, lrows) = read_csv(&lane.join("W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv"))?;
    let l_locale = col(&lh, "locale_tag");
    let l_guid = col(&lh, "function_article_guid");
    let l_name = col(&lh, "localized_name");
    let mut locales: BTreeMap<String, Vec<LocalizedName>> = BTreeMap::new();
    for r in &lrows {
        locales
            .entry(r[l_guid].to_ascii_lowercase())
            .or_default()
            .push(LocalizedName {
                locale: r[l_locale].clone(),
                name: r[l_name].clone(),
            });
    }
    for v in locales.values_mut() {
        v.sort_by(|a, b| a.locale.cmp(&b.locale));
    }

    // Correlation ledger: function id -> artifact bindings.
    let (rh, rrows) = read_csv(&lane.join("FUNCTION_SLICE_CORRELATION_LEDGER.csv"))?;
    let r_id = col(&rh, "function_id");
    let r_contract = col(&rh, "contract_row_id");
    let r_lean = col(&rh, "lean_module");
    let r_rust = col(&rh, "rust_module");
    let r_status = col(&rh, "status");
    let mut artifacts: BTreeMap<String, Artifacts> = BTreeMap::new();
    for r in &rrows {
        artifacts.insert(
            r[r_id].clone(),
            Artifacts {
                contract_row_id: none_if_empty(&r[r_contract]),
                lean_module: none_if_empty(&r[r_lean]),
                rust_module: none_if_empty(&r[r_rust]),
                status: none_if_empty(&r[r_status]),
            },
        );
    }

    // Runtime registry (canonical enrichment source).
    let registry = oxfunc_core::registry::builtin_registry();
    let by_id: BTreeMap<&str, &oxfunc_core::registry::FunctionEntry> = registry
        .iter()
        .map(|e| (e.meta.function_id.as_str(), e))
        .collect();
    let registry_identity = registry.snapshot_identity();

    if std::env::var("EFH_DEBUG").is_ok() {
        let export_ids: std::collections::BTreeSet<&str> =
            export_rows.iter().map(|r| r[e_id].as_str()).collect();
        let unmatched_export: Vec<&&str> = export_ids
            .iter()
            .filter(|id| !by_id.contains_key(**id))
            .collect();
        let unmatched_registry: Vec<&&str> = by_id
            .keys()
            .filter(|id| !export_ids.contains(**id))
            .collect();
        eprintln!("export ids with no registry entry ({}): {unmatched_export:?}", unmatched_export.len());
        eprintln!("registry ids with no export row ({}): {unmatched_registry:?}", unmatched_registry.len());
    }

    let oxfunc_commit = Command::new("git")
        .args(["-C", oxfunc_root.to_str().unwrap(), "rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Expand the spine: one entry per function. Combined byte-variant documentation
    // rows ("FIND, FINDB") split into per-function entries, because the runtime
    // registry (the canonical source) models the variants as distinct functions
    // with distinct semantics. `published_row_id` preserves the link.
    struct SpineEntry<'a> {
        row: &'a Vec<String>,
        function_id: String,
        surface_name: String,
        published_row_id: String,
    }
    let mut spine: Vec<SpineEntry> = Vec::new();
    let mut split_rows = 0usize;
    for row in &export_rows {
        let published_id = row[e_id].clone();
        let name = row[e_name].clone();
        if name.contains(", ") {
            split_rows += 1;
            for part in name.split(", ") {
                spine.push(SpineEntry {
                    row,
                    function_id: format!("FUNC.{part}"),
                    surface_name: part.to_string(),
                    published_row_id: published_id.clone(),
                });
            }
        } else {
            spine.push(SpineEntry {
                row,
                function_id: published_id.clone(),
                surface_name: name,
                published_row_id: published_id,
            });
        }
    }
    spine.sort_by(|a, b| a.function_id.cmp(&b.function_id));

    let mut counts = Counts {
        total_entries: 0,
        published_rows: export_rows.len(),
        split_combined_rows: split_rows,
        functions: 0,
        operators: 0,
        registry_backed: 0,
        export_only: 0,
        deferred: 0,
        placeholder_signatures: 0,
        with_docs_link: 0,
        with_localized_names: 0,
    };
    let mut index_rows = Vec::new();

    for entry_row in &spine {
        let row = entry_row.row;
        let id = entry_row.function_id.clone();
        let name = entry_row.surface_name.clone();
        let kind = row[e_kind].clone();
        counts.total_entries += 1;
        match kind.as_str() {
            "built_in_operator" => counts.operators += 1,
            _ => counts.functions += 1,
        }

        let admission = if let Some(reason) = deferred.get(&name) {
            counts.deferred += 1;
            Admission {
                label: "deferred".to_string(),
                reason: Some(reason.clone()),
            }
        } else {
            Admission {
                label: "supported".to_string(),
                reason: None,
            }
        };

        let mut classification: BTreeMap<String, String> = BTreeMap::new();
        for (cname, cidx) in &class_idx {
            if let Some(v) = none_if_empty(&row[*cidx]) {
                classification.insert(cname.clone(), v);
            }
        }

        let entry = by_id.get(id.as_str());
        let mut signature = None;
        let mut signature_placeholder = false;
        let mut descriptions = Descriptions { short: None, long: None };
        let mut axes = None;
        if let Some(entry) = entry {
            counts.registry_backed += 1;
            let placeholder_prefix = format!("{}(...", entry.surface_name);
            let sig = &entry.display_signature;
            signature_placeholder = sig.signature_display.starts_with(&placeholder_prefix);
            if signature_placeholder {
                counts.placeholder_signatures += 1;
            } else {
                signature = Some(SignatureDoc {
                    display: sig.signature_display.clone(),
                    trailing_repeats: sig.trailing_repeats,
                    parameters: sig
                        .parameters
                        .iter()
                        .map(|p| ParameterDoc {
                            name: p.name.clone(),
                            optional: p.optional,
                            repeats: p.repeats,
                            description: p.short_description.clone(),
                        })
                        .collect(),
                });
            }
            descriptions = Descriptions {
                short: entry.short_description.clone(),
                long: entry.long_description.clone(),
            };
            let m = &entry.meta;
            axes = Some(Axes {
                lift_broadcast_profile: m.function_spec_axes_metadata.lift_broadcast_profile.clone(),
                precision_rounding_profile: m
                    .function_spec_axes_metadata
                    .precision_rounding_profile
                    .clone(),
                real_result_policy: m.function_spec_axes_metadata.real_result_policy.clone(),
                axes_version: m.function_spec_axes_metadata_version.clone(),
                semantic_kernel_version: m.semantic_kernel_metadata_version.clone(),
                arg_admission_version: m.arg_admission_metadata_version.clone(),
            });
        } else {
            counts.export_only += 1;
        }

        let docs = catalog
            .get(&name)
            .or_else(|| catalog.get(&row[e_name]))
            .map(|(url, desc)| DocsLink {
                microsoft_url: url.clone(),
                english_description: desc.clone(),
            });
        if docs.is_some() {
            counts.with_docs_link += 1;
        }

        let localized_names: Vec<LocalizedName> = docs
            .as_ref()
            .and_then(|d| guid_of_url(&d.microsoft_url))
            .and_then(|g| locales.get(&g))
            .map(|v| {
                v.iter()
                    .map(|ln| LocalizedName {
                        locale: ln.locale.clone(),
                        name: ln.name.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        if !localized_names.is_empty() {
            counts.with_localized_names += 1;
        }

        let doc = FunctionDoc {
            function_id: id.clone(),
            surface_name: name.clone(),
            published_row_id: entry_row.published_row_id.clone(),
            entry_kind: kind.clone(),
            category: none_if_empty(&row[e_cat]),
            admission,
            metadata_status: none_if_empty(&row[e_status]),
            xlcall: match (none_if_empty(&row[e_sym]), none_if_empty(&row[e_code])) {
                (Some(symbol), Some(code)) => Some(Xlcall { symbol, code }),
                _ => None,
            },
            arity: ArityDoc {
                min: row[e_amin].trim().parse().ok(),
                max: row[e_amax].trim().parse().ok(),
            },
            classification,
            axes,
            signature,
            signature_placeholder,
            descriptions,
            docs,
            localized_names,
            artifacts: artifacts.remove(&id),
            version_marker: none_if_empty(&row[e_vm]),
            special_interface_kind: none_if_empty(&row[e_sik]),
            admission_interface_kind: none_if_empty(&row[e_aik]),
            registry_backed: entry.is_some(),
        };

        let path = functions_dir.join(format!("{id}.json"));
        let mut json = serde_json::to_string_pretty(&doc)?;
        json.push('\n');
        fs::write(&path, json)?;

        index_rows.push(IndexRow {
            function_id: id,
            surface_name: name,
            entry_kind: kind,
            category: none_if_empty(&row[e_cat]),
            admission: doc_admission_label(&deferred, &entry_row.surface_name),
        });
    }

    let first = spine.first().map(|s| s.row).expect("export has rows");
    let index = IndexDoc {
        provenance: Provenance {
            export_snapshot_id: first[e_snap].clone(),
            export_snapshot_generation: first[e_gen].clone(),
            export_source_commit: first[e_commit].clone(),
            export_source_tree_state: first[e_tree].clone(),
            oxfunc_commit_at_ingest: oxfunc_commit,
            registry_snapshot_family: registry_identity.snapshot_family,
            registry_generation: registry_identity.generation,
            registry_content_fingerprint: registry_identity.content_fingerprint,
            tool: format!("efh-ingest {}", env!("CARGO_PKG_VERSION")),
        },
        counts,
        rows: index_rows,
    };
    let mut json = serde_json::to_string_pretty(&index)?;
    json.push('\n');
    fs::write(data_dir.join("index.json"), json)?;

    let c = &index.counts;
    println!(
        "efh-ingest: {} entries from {} published rows ({} split) — {} functions, {} operators; {} registry-backed, {} export-only, {} deferred, {} placeholder signatures, {} with docs link, {} with localized names",
        c.total_entries,
        c.published_rows,
        c.split_combined_rows,
        c.functions,
        c.operators,
        c.registry_backed,
        c.export_only,
        c.deferred,
        c.placeholder_signatures,
        c.with_docs_link,
        c.with_localized_names
    );
    Ok(())
}

fn doc_admission_label(deferred: &BTreeMap<String, String>, name: &str) -> String {
    if deferred.contains_key(name) {
        "deferred".to_string()
    } else {
        "supported".to_string()
    }
}
