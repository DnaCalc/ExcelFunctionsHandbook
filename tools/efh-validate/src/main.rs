//! efh-validate (T1) - the schema set and the validator every other tool is checked against.
//!
//! FOUNDATION 6, tool table row T1 (line 1181):
//!   Inputs: the schema tables in section 2.
//!   Outputs: tools/schemas/*.schema.json; a validator binary.
//!   Acceptance test: (a) every schema file parses as JSON Schema 2020-12. (b) a field-coverage
//!   test enumerates every field the 3.7 pseudocode reads and asserts it exists in the schema it is
//!   read from, with the right type - "this is the test that would have caught A3-F1". (c) a test
//!   asserts no field declared in a content/ schema is a total function of a data/ schema (A3-S8),
//!   by an explicit allow-list of the three deleted fields.
//!
//! Subcommands:
//!   selftest                run (a), (b), (c) and the corrections cross-check
//!   check <path>...         validate instance files against the schema their path routes to
//!   check-organ             route and validate every data/ and content/ JSON file that exists
//!   schemas                 list the schema set with family, organ, writer and key-order basis
//!   field-reads             print the enumerated 3.7 field reads with their FOUNDATION lines

mod acceptance;
mod json_schema;
mod regex_lite;
mod registry;
mod rubric_field_reads;

use std::path::{Path, PathBuf};

use registry::Registry;
use rubric_field_reads::{ReadKind, FIELD_READS};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut repo: Option<PathBuf> = std::env::var_os("EFH_REPO").map(PathBuf::from);
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--repo" && i + 1 < args.len() {
            repo = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else {
            rest.push(args[i].clone());
            i += 1;
        }
    }
    let repo_root = match resolve_repo(repo) {
        Some(p) => p,
        None => {
            eprintln!(
                "efh-validate: cannot find the repository root (no ancestor contains \
                 tools/schemas). Pass --repo <path> or set EFH_REPO."
            );
            std::process::exit(2);
        }
    };
    let reg = match Registry::load(&repo_root) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("efh-validate: {}", e);
            std::process::exit(2);
        }
    };

    let cmd = rest.first().map(|s| s.as_str()).unwrap_or("selftest");
    let code = match cmd {
        "selftest" => cmd_selftest(&reg),
        "check" => cmd_check(&reg, &rest[1..]),
        "check-organ" => cmd_check_organ(&reg),
        "schemas" => cmd_schemas(&reg),
        "field-reads" => cmd_field_reads(),
        other => {
            eprintln!("efh-validate: unknown subcommand `{}`", other);
            eprintln!("  try: selftest | check <path>... | check-organ | schemas | field-reads");
            2
        }
    };
    std::process::exit(code);
}

fn resolve_repo(explicit: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(p) = explicit {
        if p.join("tools").join("schemas").is_dir() {
            return Some(p);
        }
        return None;
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(p) = registry::find_repo_root(&cwd) {
            return Some(p);
        }
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    registry::find_repo_root(manifest)
}

fn rule(title: &str) {
    println!();
    println!("{}", "=".repeat(96));
    println!("{}", title);
    println!("{}", "=".repeat(96));
}

fn cmd_selftest(reg: &Registry) -> i32 {
    println!("efh-validate selftest");
    println!("repo        : {}", reg.repo_root.display());
    println!("schema dir  : {}", reg.schema_dir.display());
    println!("schema files: {}", reg.schemas.len());
    println!(
        "spec        : FOUNDATION.md section 2 (organ layout and schema tables), section 3.7 \
         (decision procedures), section 6 T1"
    );
    println!(
        "corrections : ATTRIBUTION-CORRECTIONS.md section 7 wins where it contradicts FOUNDATION"
    );

    let outcomes = vec![
        acceptance::test_a_schemas_parse(reg),
        acceptance::test_b_field_coverage(reg),
        acceptance::test_c_no_content_field_is_a_total_function(reg),
        acceptance::test_d_corrections(reg),
    ];

    let mut failed = 0usize;
    for o in &outcomes {
        rule(&format!("{} {}", if o.passed() { "PASS" } else { "FAIL" }, o.name));
        println!("{}", o.headline);
        if !o.notes.is_empty() {
            println!();
            for n in &o.notes {
                println!("{}", n);
            }
        }
        if !o.failures.is_empty() {
            failed += 1;
            println!();
            for f in &o.failures {
                println!("  FAIL {}", f);
            }
        }
    }

    rule("SUMMARY");
    for o in &outcomes {
        println!(
            "  {}  {}",
            if o.passed() { "PASS" } else { "FAIL" },
            o.name
        );
    }
    println!();
    println!(
        "  {} of {} tests passed.",
        outcomes.len() - failed,
        outcomes.len()
    );
    println!();
    println!("  What test (a) does NOT claim: no official JSON Schema metaschema document is");
    println!("  fetched or embedded. (a) is a dialect, keyword, type, $ref, pattern and");
    println!("  key-order-annotation conformance walk against a checked-in 2020-12 keyword list.");
    println!("  It is not a proof that an arbitrary 2020-12 implementation accepts these files.");
    println!();
    println!("  What test (b) does NOT claim: it checks that a field the rubric READS is DECLARED");
    println!("  with a compatible type. It cannot check that the value written into that field is");
    println!("  correct. Every count in the evidence layer is still a transcription carrying");
    println!("  handbook_reverified: false (FOUNDATION 7.2).");
    if failed == 0 {
        0
    } else {
        1
    }
}

fn cmd_schemas(reg: &Registry) -> i32 {
    println!(
        "{:<38} {:<5} {:<8} {:<34} {}",
        "SCHEMA FILE", "FAM", "ORGAN", "PATH GLOB", "KEY-ORDER BASIS"
    );
    for (name, s) in &reg.schemas {
        println!(
            "{:<38} {:<5} {:<8} {:<34} {}",
            name,
            s.efh_str("family").unwrap_or("?"),
            s.efh_str("organ").unwrap_or("?"),
            s.efh_str("path_glob").unwrap_or("?"),
            s.efh_str("propertyOrderBasis").unwrap_or("?")
        );
    }
    println!();
    println!("writers:");
    for (name, s) in &reg.schemas {
        println!("  {:<38} {}", name, s.efh_str("writer").unwrap_or("?"));
    }
    println!();
    println!("spec sections:");
    for (name, s) in &reg.schemas {
        println!("  {:<38} {}", name, s.efh_str("spec").unwrap_or("?"));
    }
    0
}

fn cmd_field_reads() -> i32 {
    println!(
        "{} enumerated reads of the FOUNDATION 3.7 decision procedure.",
        FIELD_READS.len()
    );
    println!("Source: tools/efh-validate/src/rubric_field_reads.rs");
    println!();
    println!(
        "{:<44} {:<38} {:<10} {}",
        "READER", "SCHEMA::PATH", "FOUNDATION", "KIND"
    );
    for fr in FIELD_READS {
        println!(
            "{:<44} {:<38} {:<10} {:?}",
            fr.reader,
            format!("{}::{}", short(fr.schema_file), fr.path),
            fr.foundation_lines,
            fr.kind
        );
    }
    println!();
    println!("Reads with no schema field behind them:");
    for fr in FIELD_READS.iter().filter(|f| f.kind == ReadKind::NarrativeUnresolved) {
        println!();
        println!("  {}  [FOUNDATION:{}]", fr.reader, fr.foundation_lines);
        println!("    quote : {}", squash(fr.quote));
        println!("    note  : {}", squash(fr.note));
    }
    0
}

fn short(s: &str) -> String {
    s.trim_end_matches(".schema.json").to_string()
}

fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cmd_check(reg: &Registry, args: &[String]) -> i32 {
    let mut forced: Option<String> = None;
    let mut paths: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--as" && i + 1 < args.len() {
            forced = Some(args[i + 1].clone());
            i += 2;
        } else {
            paths.push(args[i].clone());
            i += 1;
        }
    }
    if paths.is_empty() {
        eprintln!("efh-validate check [--as <schema-file>] <path>...");
        return 2;
    }
    if let Some(f) = &forced {
        if reg.get(f).is_none() {
            eprintln!("efh-validate check: no schema named `{}` in tools/schemas/", f);
            return 2;
        }
    }
    let mut files: Vec<PathBuf> = Vec::new();
    for p in &paths {
        let path = PathBuf::from(p);
        if path.is_dir() {
            collect_json(&path, &mut files);
        } else {
            files.push(path);
        }
    }
    files.sort();
    check_files_with(reg, &files, forced.as_deref())
}

fn cmd_check_organ(reg: &Registry) -> i32 {
    let mut files: Vec<PathBuf> = Vec::new();
    for organ in ["data", "content"] {
        let d = reg.repo_root.join(organ);
        if d.is_dir() {
            collect_json(&d, &mut files);
        }
    }
    files.sort();
    println!(
        "efh-validate check-organ: {} JSON files under data/ and content/",
        files.len()
    );
    check_files(reg, &files)
}

fn collect_json(dir: &Path, out: &mut Vec<PathBuf>) {
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for e in rd.filter_map(|e| e.ok()) {
        let p = e.path();
        if p.is_dir() {
            collect_json(&p, out);
        } else if p.extension().map(|x| x == "json").unwrap_or(false) {
            out.push(p);
        }
    }
}

fn check_files(reg: &Registry, files: &[PathBuf]) -> i32 {
    check_files_with(reg, files, None)
}

fn check_files_with(reg: &Registry, files: &[PathBuf], forced: Option<&str>) -> i32 {
    let mut unrouted: Vec<String> = Vec::new();
    let mut per_schema: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    let mut findings_printed = 0usize;
    let mut total_findings = 0usize;

    for f in files {
        let rel = relativise(&reg.repo_root, f);
        let routed = match forced {
            Some(name) => reg.get(name),
            None => reg.route(&rel),
        };
        let schema = match routed {
            Some(s) => s,
            None => {
                unrouted.push(rel);
                continue;
            }
        };
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                println!("  READ-FAIL {}: {}", rel, e);
                total_findings += 1;
                continue;
            }
        };
        let inst: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                println!("  JSON-FAIL {}: {}", rel, e);
                total_findings += 1;
                continue;
            }
        };
        let findings = schema.validate(&inst);
        let entry = per_schema.entry(schema.file_name.clone()).or_insert((0, 0));
        entry.0 += 1;
        if findings.is_empty() {
            entry.1 += 1;
        } else {
            total_findings += findings.len();
            if findings_printed < 40 {
                for fd in findings.iter().take(6) {
                    println!(
                        "  {} :: {}{}: {}",
                        rel,
                        schema.efh_str("family").unwrap_or("?"),
                        fd.path,
                        fd.message
                    );
                    findings_printed += 1;
                }
            }
        }
    }

    println!();
    println!("{:<38} {:>8} {:>8}", "SCHEMA", "ROUTED", "VALID");
    for (k, (n, ok)) in &per_schema {
        println!("{:<38} {:>8} {:>8}", k, n, ok);
    }
    if !unrouted.is_empty() {
        println!();
        println!(
            "{} file(s) matched no schema path_glob (not an error - the schema set covers the \
             families FOUNDATION section 2 names, and the repository holds other JSON too):",
            unrouted.len()
        );
        for u in unrouted.iter().take(20) {
            println!("  {}", u);
        }
        if unrouted.len() > 20 {
            println!("  ... and {} more", unrouted.len() - 20);
        }
    }
    println!();
    println!("total schema findings: {}", total_findings);
    if total_findings > 0 && findings_printed >= 40 {
        println!("(output truncated at 40 findings)");
    }
    if total_findings == 0 {
        0
    } else {
        1
    }
}

fn relativise(root: &Path, p: &Path) -> String {
    let abs = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let root_abs = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let s = match abs.strip_prefix(&root_abs) {
        Ok(r) => r.to_string_lossy().to_string(),
        Err(_) => abs.to_string_lossy().to_string(),
    };
    let s = s.replace('\\', "/");
    // Windows canonicalize() yields a \\?\ verbatim prefix; strip it so paths read normally.
    s.trim_start_matches("//?/").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg() -> Registry {
        let root = registry::find_repo_root(Path::new(env!("CARGO_MANIFEST_DIR")))
            .expect("repo root");
        Registry::load(&root).expect("registry")
    }

    #[test]
    fn acceptance_a_schemas_parse_as_json_schema_2020_12() {
        let o = acceptance::test_a_schemas_parse(&reg());
        assert!(o.passed(), "{:#?}", o.failures);
    }

    #[test]
    fn acceptance_b_every_rubric_field_read_is_declared() {
        let o = acceptance::test_b_field_coverage(&reg());
        assert!(o.passed(), "{:#?}", o.failures);
    }

    #[test]
    fn acceptance_c_no_content_field_is_a_total_function_of_data() {
        let o = acceptance::test_c_no_content_field_is_a_total_function(&reg());
        assert!(o.passed(), "{:#?}", o.failures);
    }

    #[test]
    fn attribution_corrections_are_encoded() {
        let o = acceptance::test_d_corrections(&reg());
        assert!(o.passed(), "{:#?}", o.failures);
    }

    #[test]
    fn every_schema_declares_its_family_organ_writer_and_key_order_basis() {
        let r = reg();
        for (name, s) in &r.schemas {
            for k in ["family", "organ", "path_glob", "writer", "spec", "propertyOrderBasis"] {
                assert!(s.efh_str(k).is_some(), "{} is missing efh.{}", name, k);
            }
        }
    }

    #[test]
    fn path_routing_is_unambiguous_for_the_organ_families() {
        let r = reg();
        for (p, want) in [
            ("data/index.json", "f0-index.schema.json"),
            ("data/functions/FUNC.PMT.json", "f1-function.schema.json"),
            ("data/presence/FUNC.PMT.json", "f2-presence.schema.json"),
            ("data/presence/index.json", "f2i-presence-index.schema.json"),
            ("data/axes/vocabulary.json", "f3-axis-vocabulary.schema.json"),
            ("data/battery/FUNC.PMT.json", "f16-battery.schema.json"),
            ("content/model/axis-glossary.json", "f4-axis-glossary.schema.json"),
            ("content/model/scan-limits.json", "f14-scan-limits.schema.json"),
            ("content/model/name-debris.json", "f15-name-debris.schema.json"),
            ("content/evidence/records/EV-live-verification-0007.json",
             "f5-evidence-record.schema.json"),
            ("content/evidence/REGISTER.json", "f5r-evidence-register.schema.json"),
            ("content/references/bibliography.json", "f7-bibliography.schema.json"),
            ("content/categories/aliases.json", "f8-aliases.schema.json"),
            ("content/openproblems/OP-004.json", "f9-open-problem.schema.json"),
            ("content/lastbit/EPISODES.json", "f10-episodes.schema.json"),
            ("content/operators/syntax.json", "f11-operator-syntax.schema.json"),
            ("content/projection-gaps.json", "f12-projection-gaps.schema.json"),
        ] {
            let got = r.route(p).map(|s| s.file_name.clone());
            assert_eq!(got.as_deref(), Some(want), "routing {}", p);
        }
    }

    // ---- negative controls: proof that the instance validator is not vacuous ----

    fn f5_minimal_count() -> serde_json::Value {
        serde_json::json!({
            "figure": "4/4",
            "passed": 4,
            "total": 4,
            "axis": "numeric-bits",
            "comparison_predicate": "exact-typed-bit-match",
            "count_scope": "per-surface",
            "group_members": [],
            "attribution": "measured-for-this-surface",
            "measurement_subject": "production-oxfunc",
            "held_out": "source-does-not-state",
            "held_out_rows": null,
            "corpus_was_repair_target": false,
            "residual_attribution": null,
            "measurement_found": true,
            "divergence_measured": false,
            "full_pass_only": true,
            "corpus_or_build": "",
            "corpus_tracked": false,
            "measured_as_of": null,
            "citation": ""
        })
    }

    #[test]
    fn negative_control_key_order_violation_is_caught() {
        let r = reg();
        let s = r.get("f16-battery.schema.json").unwrap();
        // surface_name emitted BEFORE function_id, which the schema orders the other way.
        let bad = serde_json::json!({
            "schema": "efh.battery/v1",
            "surface_name": "PMT",
            "function_id": "FUNC.PMT",
            "battery_id": "EFH-B1",
            "oxfunc_commit": "x",
            "oxfunc_tree_clean": true,
            "runner_version": "v",
            "host": { "arch": "x86-64", "cpu": "c", "os": "o" },
            "rows": [],
            "label": "OxFunc's own answers. No Excel was involved."
        });
        let f = s.validate(&bad);
        assert!(
            f.iter().any(|x| x.message.contains("key order violates the byte-stability contract")),
            "key order violation not reported: {:#?}",
            f
        );
    }

    #[test]
    fn negative_control_missing_required_count_field_is_caught() {
        let r = reg();
        let s = r.get("f5-evidence-record.schema.json").unwrap();
        let mut c = f5_minimal_count();
        c.as_object_mut().unwrap().remove("corpus_was_repair_target");
        let f = s.validate(&serde_json::json!({ "counts": [c] }));
        assert!(
            f.iter().any(|x| x.message.contains("missing required key `corpus_was_repair_target`")),
            "missing count field not reported: {:#?}",
            f
        );
    }

    #[test]
    fn negative_control_held_out_typed_as_a_boolean_is_caught() {
        // FOUNDATION 2.5 line 234 makes held_out a FOUR-valued string enum. A schema or a writer
        // that treats it as a boolean silently turns "source-does-not-state" into a false, which
        // is the difference between W4 and W5.
        let r = reg();
        let s = r.get("f5-evidence-record.schema.json").unwrap();
        let mut c = f5_minimal_count();
        c["held_out"] = serde_json::json!(false);
        let f = s.validate(&serde_json::json!({ "counts": [c] }));
        assert!(
            f.iter().any(|x| x.message.contains("expected type") && x.path.contains("held_out")),
            "boolean held_out not reported: {:#?}",
            f
        );
    }

    #[test]
    fn negative_control_handbook_reverified_true_is_caught() {
        // FOUNDATION 2.5 line 219: const false today for every record. It is what stops the
        // Handbook wearing OxFunc's evidence as its own.
        let r = reg();
        let s = r.get("f5-evidence-record.schema.json").unwrap();
        let f = s.validate(&serde_json::json!({ "handbook_reverified": true }));
        assert!(
            f.iter().any(|x| x.path.contains("handbook_reverified") && x.message.contains("const")),
            "handbook_reverified: true not reported: {:#?}",
            f
        );
    }

    #[test]
    fn negative_control_undeclared_key_is_caught() {
        let r = reg();
        let s = r.get("f2-presence.schema.json").unwrap();
        // the exact field FOUNDATION 3.7 line 761 forbids the test predicate from reading
        let f = s.validate(&serde_json::json!({ "unit_tests_direct": 110 }));
        assert!(
            f.iter().any(|x| x.message.contains("undeclared key `unit_tests_direct`")),
            "undeclared key not reported: {:#?}",
            f
        );
    }

    #[test]
    fn presence_index_does_not_steal_the_per_function_presence_route() {
        let r = reg();
        // data/presence/index.json must route to F2i, not to F2, even though both globs match.
        assert_eq!(
            r.route("data/presence/index.json").map(|s| s.file_name.clone()).as_deref(),
            Some("f2i-presence-index.schema.json")
        );
    }
}
