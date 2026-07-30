//! FOUNDATION §6, T2 acceptance tests.
//!
//! (a) 541 files, all carrying every required F2 field in schema key order.
//! (b) Regenerating twice byte-compares equal.
//! (c) `module_tests_minus_sibling_count` is negative for exactly N modules
//!     touching exactly M entries. FOUNDATION predicts 58/202; this suite pins
//!     the RE-DERIVED 57/198 and the single named module that accounts for the
//!     whole difference, so the discrepancy can never be lost again.
//! (d) `scan_inventory.rust_function_modules_scanned == 254`,
//!     `test_files_scanned == 17`, `lean_sources_scanned == 247`.
//! (e) `oxfunc_tree_clean == false` aborts with a non-zero exit and writes
//!     nothing.
//! (f) `doc_mention_classification` for HARMEAN yields 10 files, all
//!     `bulk-inventory`.
//!
//! Every test runs the real binary. Nothing is written outside a temp dir.

use std::path::{Path, PathBuf};
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_efh-presence");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn oxfunc() -> PathBuf {
    repo_root().parent().unwrap().join("OxFunc")
}

fn tmp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("efh-presence-test-{tag}"));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn generate(out: &Path) {
    let st = Command::new(BIN)
        .args(["--oxfunc", oxfunc().to_str().unwrap()])
        .args(["--out", out.to_str().unwrap()])
        .status()
        .expect("run efh-presence");
    assert!(st.success(), "generator exited {st:?}");
}

/// Ultra-small reader: these tests only need string / int / array shapes, and
/// pulling in a JSON crate would break the "no dependencies" property the tool
/// exists to keep.
fn slurp(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

/// Top-level keys, in file order.
fn top_level_keys(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in src.lines() {
        if let Some(rest) = line.strip_prefix("  \"") {
            if let Some(i) = rest.find("\":") {
                out.push(rest[..i].to_string());
            }
        }
    }
    out
}

const REQUIRED_KEY_ORDER: [&str; 30] = [
    "schema",
    "function_id",
    "surface_name",
    "oxfunc_commit",
    "oxfunc_tree_clean",
    "name_match_confidence",
    "name_match_rule",
    "impl_modules",
    "exclusion_rule_id",
    "mention_modules",
    "module_shared_by",
    "module_shared_by_count",
    "tests_in_impl_modules",
    "tests_are_module_level",
    "tests_per_module",
    "module_tests_minus_sibling_count",
    "source_lines_per_module",
    "lean_modules",
    "lean_parsed",
    "registered_in",
    "dispatch_in",
    "declared_artifacts_present",
    "doc_mention_counts",
    "doc_mention_classification",
    "doc_mentions_are_not_verdicts",
    "fuzzer_corpus_hit_files",
    "fixture_hits",
    "scan_inventory",
    "searched_where",
    "limits",
];

#[test]
fn t2a_541_files_with_every_required_field_in_schema_order() {
    let out = tmp("a");
    generate(&out);

    let mut files: Vec<PathBuf> = std::fs::read_dir(&out)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name().unwrap().to_string_lossy().starts_with("FUNC.")
        })
        .collect();
    files.sort();
    assert_eq!(files.len(), 541, "expected 541 per-entry presence files");

    for f in &files {
        let src = slurp(f);
        assert!(src.ends_with("}\n"), "{}: no trailing newline", f.display());
        assert!(!src.starts_with('\u{feff}'), "{}: BOM", f.display());
        assert!(!src.contains('\r'), "{}: CR in output", f.display());
        let keys = top_level_keys(&src);
        // declared_vs_scanned is conditional (O): present only when the entry
        // declares artifacts, and always immediately after declared_artifacts_present.
        let filtered: Vec<&str> = keys
            .iter()
            .map(|s| s.as_str())
            .filter(|k| *k != "declared_vs_scanned")
            .collect();
        assert_eq!(
            filtered,
            REQUIRED_KEY_ORDER.to_vec(),
            "{}: key set / order",
            f.display()
        );
        if let Some(i) = keys.iter().position(|k| k == "declared_vs_scanned") {
            assert_eq!(keys[i - 1], "declared_artifacts_present");
            assert!(src.contains("\"declared_artifacts_present\": true"));
        }
        assert!(src.contains("\"schema\": \"efh.presence/v2\""));
        assert!(src.contains("\"oxfunc_tree_clean\": true"));
        assert!(src.contains("\"tests_are_module_level\": true"));
        assert!(src.contains("\"lean_parsed\": false"));
        assert!(src.contains("\"doc_mentions_are_not_verdicts\": true"));
        assert!(src.contains("\"exclusion_rule_id\": \"EXCL-SURFACE-DISPATCH-1\""));
        for l in ["L1", "L2", "L3", "L4", "L5", "L6", "L7", "L8", "L9"] {
            assert!(src.contains(&format!("\"{l}\"")), "{}: missing {l}", f.display());
        }
    }
    assert!(out.join("index.json").is_file());
}

#[test]
fn t2b_two_runs_byte_compare_equal() {
    let a = tmp("b1");
    let b = tmp("b2");
    generate(&a);
    generate(&b);
    let mut names: Vec<String> = std::fs::read_dir(&a)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    assert_eq!(names.len(), 542);
    for n in &names {
        let x = std::fs::read(a.join(n)).unwrap();
        let y = std::fs::read(b.join(n)).unwrap();
        assert_eq!(x, y, "{n} differs between two runs");
    }
}

#[test]
fn t2c_pigeonhole_modules_and_entries() {
    let out = tmp("c");
    generate(&out);
    let idx = slurp(&out.join("index.json"));

    let grab = |k: &str| -> i64 {
        let pat = format!("\"{k}\": ");
        let i = idx.find(&pat).unwrap_or_else(|| panic!("no {k} in index"));
        let rest = &idx[i + pat.len()..];
        let end = rest.find(|c: char| c != '-' && !c.is_ascii_digit()).unwrap();
        rest[..end].parse().unwrap()
    };

    // RE-DERIVED at OxFunc 473efa3. FOUNDATION §3.3 predicts 58 / 202.
    // The whole difference is crates/oxfunc_core/src/locale_format.rs, which
    // FOUNDATION carries as "0 / 4" because harvest_impl_map.py only counted
    // `#[test]` inside crates/oxfunc_core/src/functions/ and that module lives
    // one directory up. It actually holds 6 `#[test]` attributes for 4 mapped
    // ids, so it is NOT short of tests and its 4 ids (DOLLAR, FIXED, TEXT,
    // VALUE) are touched by no other short module.  57 + 1 = 58, 198 + 4 = 202.
    assert_eq!(
        grab("impl_modules_with_negative_module_tests_minus_sibling_count"),
        57
    );
    assert_eq!(grab("entries_touched_by_a_negative_impl_module"), 198);
    assert!(idx.contains("\"module_path\": \"crates/oxfunc_core/src/locale_format.rs\""));

    // and the per-entry field really is negative exactly where the module is
    let neg_named = [
        ("complex_family.rs", -20),
        ("chi_f_t_family.rs", -11),
        ("engineering_radix_family.rs", -7),
        ("normal_log_family.rs", -6),
        ("database_family.rs", -5),
        ("text_b_compat_family.rs", -4),
        ("ceiling_floor_family.rs", -3),
    ];
    for (m, d) in neg_named {
        assert!(
            idx.contains(&format!(
                "\"module_path\": \"crates/oxfunc_core/src/functions/{m}\""
            )),
            "missing module row for {m}"
        );
        assert!(
            idx.contains(&format!("\"module_tests_minus_sibling_count\": {d}")),
            "no module has module_tests_minus_sibling_count {d}"
        );
    }
}

#[test]
fn t2d_scan_inventory_pins() {
    let out = tmp("d");
    generate(&out);
    let src = slurp(&out.join("FUNC.ABS.json"));
    assert!(src.contains("\"rust_function_modules_scanned\": 254"));
    assert!(src.contains("\"test_files_scanned\": 17"));
    assert!(src.contains("\"lean_sources_scanned\": 247"));
    assert!(src.contains("\"searched_where\""));
    assert!(src.contains("crates/oxfunc_core/src/functions/surface_dispatch.rs"));
}

#[test]
fn t2e_dirty_tree_aborts_and_writes_nothing() {
    // A throwaway git repo, made dirty on purpose. Neither OxFunc nor the
    // Handbook is touched.
    let repo = tmp("e-repo");
    let out = tmp("e-out");
    let g = |args: &[&str]| {
        let st = Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(args)
            .output()
            .expect("git");
        assert!(st.status.success(), "git {args:?}: {:?}", st);
    };
    g(&["init", "-q"]);
    g(&["config", "user.email", "t@example.invalid"]);
    g(&["config", "user.name", "t"]);
    std::fs::write(repo.join("a.txt"), b"x").unwrap();
    g(&["add", "-A"]);
    g(&["commit", "-qm", "seed"]);
    // now dirty it
    std::fs::write(repo.join("a.txt"), b"y").unwrap();

    let o = Command::new(BIN)
        .args(["--oxfunc", repo.to_str().unwrap()])
        .args(["--out", out.to_str().unwrap()])
        .output()
        .expect("run");
    assert!(!o.status.success(), "dirty tree must abort non-zero");
    let msg = String::from_utf8_lossy(&o.stderr);
    assert!(
        msg.contains("oxfunc_tree_clean == false"),
        "stderr did not name the gate: {msg}"
    );
    let n = std::fs::read_dir(&out).unwrap().flatten().count();
    assert_eq!(n, 0, "dirty tree must write nothing; found {n} files");
}

#[test]
fn t2f_harmean_doc_classification() {
    let out = tmp("f");
    generate(&out);
    let src = slurp(&out.join("FUNC.HARMEAN.json"));
    let start = src.find("\"doc_mention_classification\": [").unwrap();
    let end = src[start..].find("\n  ]").unwrap() + start;
    let block = &src[start..end];
    let files = block.matches("\"path\": ").count();
    let bulk = block.matches("\"kind\": \"bulk-inventory\"").count();
    let behav = block.matches("\"kind\": \"behavioural-finding\"").count();
    assert_eq!(files, 10, "HARMEAN must classify 10 mentioning files");
    assert_eq!(bulk, 10, "all 10 must be bulk-inventory");
    assert_eq!(behav, 0);
    // and the count field agrees with the classification list
    assert!(src.contains("\"function_lane_file_count\": 10"));
}

#[test]
fn t2g_no_entry_carries_a_verdict_shaped_field() {
    // The schema forbids a pass/match/verdict field. Guard it textually so a
    // later edit cannot smuggle one in.
    let out = tmp("g");
    generate(&out);
    let src = slurp(&out.join("FUNC.PMT.json"));
    for forbidden in [
        "\"passed\"", "\"failed\"", "\"verdict\"", "\"matched\"", "\"correct\"",
        "\"pass_rate\"", "\"verified\"", "\"tested\"",
    ] {
        assert!(!src.contains(forbidden), "presence record contains {forbidden}");
    }
}
