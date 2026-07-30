//! efh-presence (T2) — the mechanical implementation-presence projection.
//!
//! Emits `data/presence/<function_id>.json` (F2, schema `efh.presence/v2`) for
//! every entry in `data/functions/*.json`, plus `data/presence/index.json`.
//!
//! This program makes NO correctness claim and has no field that could carry
//! one. It reports where a name or an id occurs in another repository's files,
//! how many `#[test]` attributes those files contain, and nothing else.
//!
//! PUBLICATION GATE: if `git -C <oxfunc> status --porcelain` is non-empty the
//! program aborts with a non-zero exit code and writes nothing at all.
//!
//! Determinism: every array is ordinal-sorted, every data-keyed object is
//! emitted in ordinal key order, every path is forward-slashed, no wall clock
//! is read, output is UTF-8 without BOM with `\n` line endings, 2-space indent
//! and a trailing newline.

mod jsonval;
mod scan;

use jsonval::{to_string_pretty, J};
use scan::{count_lines, count_test_attrs, nearest_atx_header, on_table_row, FileHits, Lex};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

// ------------------------------------------------------------- constants

const SCHEMA: &str = "efh.presence/v2";
const INDEX_SCHEMA: &str = "efh.presence.index/v2";
const EXCLUSION_RULE_ID: &str = "EXCL-SURFACE-DISPATCH-1";
const DISPATCH_BASENAME_PREFIX: &str = "surface_dispatch";

/// Limit ids only. The prose for L1..L9 lives in `content/model/scan-limits.json`
/// (F14), which this tool must never write. Build guard G-12 reconciles the two.
const LIMIT_IDS: [&str; 9] = ["L1", "L2", "L3", "L4", "L5", "L6", "L7", "L8", "L9"];

const DOC_CATALOGS: [(&str, &str); 3] = [
    ("discrepancy_catalog", "docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md"),
    ("math_deviation_catalog", "docs/EXCEL_MATH_DEVIATION_CATALOG.md"),
    ("known_exactness_deviations", "docs/KNOWN_EXACTNESS_DEVIATIONS.md"),
];

// --- the declared doc_mention_classification rule (see README.md §Rule DMC-1)
const BEHAVIOURAL_FILENAME_TOKENS: [&str; 12] = [
    "BUG-", "DEFECT", "DEVIATION", "DISCREPANCY", "DIVERGENCE", "FAILURE", "FINDING",
    "MISMATCH", "NOTES", "REGRESSION", "REPRO", "ROOT_CAUSE",
];
const BULK_FILENAME_TOKENS: [&str; 13] = [
    "CATALOG", "EXPORT", "INDEX", "INVENTORY", "LEDGER", "MANIFEST", "REGISTER", "REGISTRY",
    "ROLLUP", "SEED", "SNAPSHOT", "TABLE", "TRANCHE",
];
const BEHAVIOURAL_HEADER_TOKENS: [&str; 13] = [
    "BEHAVIOR", "BEHAVIOUR", "DEFECT", "DEVIATION", "DISCREPANCY", "DIVERGENCE", "EXPECTED",
    "FINDING", "MISMATCH", "OBSERVED", "REPRO", "ROOT CAUSE", "SYMPTOM",
];

const KIND_BULK: &str = "bulk-inventory";
const KIND_BEHAVIOURAL: &str = "behavioural-finding";

// ------------------------------------------------------------------ util

fn fslash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn rel_to(root: &Path, p: &Path) -> String {
    let r = fslash(root);
    let a = fslash(p);
    let base = format!("{}/", r.trim_end_matches('/'));
    a.strip_prefix(&base).map(|s| s.to_string()).unwrap_or(a)
}

fn excluded(p: &str) -> bool {
    p.contains(".claude/worktrees") || p.contains("/target/") || p.contains("/.git/")
}

fn walk(root: &Path, ext: Option<&str>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let rd = match std::fs::read_dir(&d) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for e in rd.flatten() {
            let p = e.path();
            let s = fslash(&p);
            if excluded(&s) {
                continue;
            }
            match e.file_type() {
                Ok(ft) if ft.is_dir() => stack.push(p),
                Ok(ft) if ft.is_file() => {
                    if let Some(x) = ext {
                        let ok = p
                            .extension()
                            .map(|e| e.to_string_lossy().to_lowercase() == x)
                            .unwrap_or(false);
                        if !ok {
                            continue;
                        }
                    }
                    out.push(p);
                }
                _ => {}
            }
        }
    }
    out.sort_by(|a, b| fslash(a).cmp(&fslash(b)));
    out
}

fn read(p: &Path) -> Vec<u8> {
    std::fs::read(p).unwrap_or_default()
}

fn basename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

// ------------------------------------------------------------------ types

struct Entry {
    function_id: String,
    surface_name: String,
    decl_rust: Vec<String>,
    decl_lean: Vec<String>,
    has_artifacts: bool,
}

struct Args {
    oxfunc: PathBuf,
    functions: PathBuf,
    out: PathBuf,
    expect_commit: Option<String>,
}

fn handbook_root() -> PathBuf {
    // tools/efh-presence -> ..\.. -> repo root. Compile-time constant.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .expect("manifest dir has two parents")
}

fn parse_args() -> Args {
    let hb = handbook_root();
    let mut a = Args {
        oxfunc: hb
            .parent()
            .map(|p| p.join("OxFunc"))
            .unwrap_or_else(|| PathBuf::from("OxFunc")),
        functions: hb.join("data").join("functions"),
        out: hb.join("data").join("presence"),
        expect_commit: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        match k.as_str() {
            "--oxfunc" => a.oxfunc = PathBuf::from(it.next().expect("--oxfunc needs a path")),
            "--functions" => {
                a.functions = PathBuf::from(it.next().expect("--functions needs a path"))
            }
            "--out" => a.out = PathBuf::from(it.next().expect("--out needs a path")),
            "--commit" => a.expect_commit = Some(it.next().expect("--commit needs a sha")),
            "--help" | "-h" => {
                println!(
                    "efh-presence [--oxfunc DIR] [--functions DIR] [--out DIR] [--commit SHA40]"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("efh-presence: unknown argument {other:?}");
                std::process::exit(64);
            }
        }
    }
    a
}

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let o = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !o.status.success() {
        return Err(format!(
            "git {:?} failed in {}: {}",
            args,
            repo.display(),
            String::from_utf8_lossy(&o.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&o.stdout).to_string())
}

// ------------------------------------------------------------------- main

fn main() {
    let args = parse_args();

    // ---------------------------------------------------- publication gate
    // Runs BEFORE anything is created, opened for writing, or removed.
    let head = match git(&args.oxfunc, &["rev-parse", "HEAD"]) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            eprintln!("efh-presence: FATAL {e}");
            std::process::exit(2);
        }
    };
    if head.len() != 40 || !head.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("efh-presence: FATAL HEAD {head:?} is not a 40-hex commit id");
        std::process::exit(2);
    }
    if let Some(want) = &args.expect_commit {
        if want != &head {
            eprintln!(
                "efh-presence: FATAL --commit {want} but OxFunc HEAD is {head}; nothing written"
            );
            std::process::exit(3);
        }
    }
    let porcelain = match git(&args.oxfunc, &["status", "--porcelain"]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("efh-presence: FATAL {e}");
            std::process::exit(2);
        }
    };
    if !porcelain.trim().is_empty() {
        let n = porcelain.lines().filter(|l| !l.trim().is_empty()).count();
        eprintln!(
            "efh-presence: FATAL oxfunc_tree_clean == false. \
             `git -C {} status --porcelain` reported {n} path(s). \
             data/presence is a publication-gated organ; NOTHING was written.",
            args.oxfunc.display()
        );
        for l in porcelain.lines().take(10) {
            eprintln!("  {l}");
        }
        std::process::exit(4);
    }

    match run(&args, &head) {
        Ok(n) => eprintln!("efh-presence: wrote {n} files to {}", fslash(&args.out)),
        Err(e) => {
            eprintln!("efh-presence: FATAL {e}");
            std::process::exit(5);
        }
    }
}

fn run(args: &Args, head: &str) -> Result<usize, String> {
    let ox = args.oxfunc.clone();

    // ------------------------------------------------------ entry loading
    let mut fnfiles: Vec<PathBuf> = std::fs::read_dir(&args.functions)
        .map_err(|e| format!("cannot read {}: {e}", args.functions.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|x| x.to_string_lossy().to_lowercase() == "json")
                .unwrap_or(false)
        })
        .collect();
    fnfiles.sort_by(|a, b| fslash(a).cmp(&fslash(b)));

    let mut entries: Vec<Entry> = Vec::with_capacity(fnfiles.len());
    for f in &fnfiles {
        let v = jsonval::parse(&read(f)).map_err(|e| format!("{}: {e}", f.display()))?;
        let fid = v
            .get("function_id")
            .and_then(|x| x.as_str())
            .ok_or_else(|| format!("{}: no function_id", f.display()))?
            .to_string();
        let sn = v
            .get("surface_name")
            .and_then(|x| x.as_str())
            .ok_or_else(|| format!("{}: no surface_name", f.display()))?
            .to_string();
        let arts = v.get("artifacts");
        let has_artifacts = matches!(arts, Some(J::Obj(o)) if !o.is_empty());
        let split = |k: &str| -> Vec<String> {
            arts.and_then(|a| a.get(k))
                .and_then(|x| x.as_str())
                .map(|s| {
                    s.split(';')
                        .map(|t| t.trim().to_string())
                        .filter(|t| !t.is_empty())
                        .collect()
                })
                .unwrap_or_default()
        };
        entries.push(Entry {
            function_id: fid,
            surface_name: sn,
            decl_rust: split("rust_module"),
            decl_lean: split("lean_module"),
            has_artifacts,
        });
    }
    entries.sort_by(|a, b| a.function_id.cmp(&b.function_id));

    // Every id must be unique, and every surface name must be unique, or the
    // single-pass lexicon silently drops entries.
    {
        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();
        for e in &entries {
            if !ids.insert(e.function_id.clone()) {
                return Err(format!("duplicate function_id {}", e.function_id));
            }
            if !names.insert(e.surface_name.clone()) {
                return Err(format!("duplicate surface_name {}", e.surface_name));
            }
        }
    }

    // ---------------------------------------------------------- lexicon
    let mut name_of = HashMap::new();
    let mut id_of = HashMap::new();
    let mut short = Vec::with_capacity(entries.len());
    for (i, e) in entries.iter().enumerate() {
        name_of.insert(e.surface_name.clone(), i);
        id_of.insert(e.function_id.clone(), i);
        short.push(e.surface_name.chars().count() <= 3);
    }
    let lex = Lex { name_of, id_of, short };

    // ------------------------------------------------------- file inventory
    let src = ox.join("crates/oxfunc_core/src");
    let rust_files = walk(&src.join("functions"), Some("rs"));
    let registry_files: Vec<PathBuf> = ["registry_signature_seed.rs", "registry_context_seed.rs"]
        .iter()
        .map(|f| src.join(f))
        .filter(|p| p.is_file())
        .collect();
    let test_files = walk(&ox.join("crates/oxfunc_core/tests"), None);
    let lean_files = walk(&ox.join("formal"), Some("lean"));
    let corpus_files = walk(&ox.join("smart-fuzzer/corpus"), None);

    let mut doc_files: Vec<PathBuf> = Vec::new();
    for (_, rel) in DOC_CATALOGS.iter() {
        let p = ox.join(rel);
        if p.is_file() {
            doc_files.push(p);
        }
    }
    let stream_files = walk(&ox.join("docs/bugs/streams"), None);
    let lane_files = walk(&ox.join("docs/function-lane"), None);
    doc_files.extend(stream_files.iter().cloned());
    doc_files.extend(lane_files.iter().cloned());
    doc_files.sort_by(|a, b| fslash(a).cmp(&fslash(b)));
    doc_files.dedup_by(|a, b| fslash(a) == fslash(b));

    let dispatch_files: Vec<PathBuf> = rust_files
        .iter()
        .filter(|p| {
            basename(&fslash(p)).starts_with(DISPATCH_BASENAME_PREFIX)
        })
        .cloned()
        .collect();

    // Rust modules declared by `artifacts.rust_module` that resolve to a real
    // file OUTSIDE crates/oxfunc_core/src/functions (today: src/locale_format.rs
    // only). They are not part of the 254-module grep set — nothing else could
    // map to them — but their `#[test]` count and line count are real and must
    // be read from disk rather than assumed to be zero.
    let mut extra_rust: BTreeSet<String> = BTreeSet::new();
    for e in &entries {
        for m in &e.decl_rust {
            if let Some(p) = rust_path_from_module(&src, m) {
                if p.is_file() {
                    let rp = rel_to(&ox, &p);
                    if !rust_files.iter().any(|q| rel_to(&ox, q) == rp) {
                        extra_rust.insert(rp);
                    }
                }
            }
        }
    }

    // --------------------------------------------------------------- scans
    // rust module metadata: rel path -> (#[test] count, source lines)
    let mut rust_meta: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut rust_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    for p in &rust_files {
        let b = read(p);
        let rp = rel_to(&ox, p);
        rust_meta.insert(rp.clone(), (count_test_attrs(&b), count_lines(&b)));
        rust_hits.insert(rp, scan::scan(&b, &lex, false));
    }
    for rp in &extra_rust {
        let b = read(&ox.join(rp));
        rust_meta.insert(rp.clone(), (count_test_attrs(&b), count_lines(&b)));
    }

    let mut registry_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    for p in &registry_files {
        registry_hits.insert(rel_to(&ox, p), scan::scan(&read(p), &lex, false));
    }
    let mut test_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    for p in &test_files {
        test_hits.insert(rel_to(&ox, p), scan::scan(&read(p), &lex, false));
    }
    let mut lean_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    for p in &lean_files {
        lean_hits.insert(rel_to(&ox, p), scan::scan(&read(p), &lex, false));
    }
    let mut corpus_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    for p in &corpus_files {
        corpus_hits.insert(rel_to(&ox, p), scan::scan(&read(p), &lex, false));
    }
    // Doc files keep occurrence offsets so the classification rule can look at
    // the nearest Markdown header and at table-row membership.
    let mut doc_hits: BTreeMap<String, FileHits> = BTreeMap::new();
    let mut doc_bytes: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for p in &doc_files {
        let b = read(p);
        let rp = rel_to(&ox, p);
        doc_hits.insert(rp.clone(), scan::scan(&b, &lex, true));
        doc_bytes.insert(rp, b);
    }

    let dispatch_rel: Vec<String> = dispatch_files.iter().map(|p| rel_to(&ox, p)).collect();
    let registry_rel: Vec<String> = registry_files.iter().map(|p| rel_to(&ox, p)).collect();
    let stream_rel: BTreeSet<String> = stream_files.iter().map(|p| rel_to(&ox, p)).collect();
    let lane_rel: BTreeSet<String> = lane_files.iter().map(|p| rel_to(&ox, p)).collect();

    // ------------------------------------------------ per-entry module sets
    let mut mention_modules: Vec<Vec<String>> = vec![Vec::new(); entries.len()];
    let mut impl_modules: Vec<Vec<String>> = vec![Vec::new(); entries.len()];
    let mut declared_found: Vec<Vec<String>> = vec![Vec::new(); entries.len()];
    let mut declared_missing: Vec<Vec<String>> = vec![Vec::new(); entries.len()];

    for (i, e) in entries.iter().enumerate() {
        let mut mods: BTreeSet<String> = BTreeSet::new();
        for (rp, h) in &rust_hits {
            if h.quoted_ids.contains(&i) {
                mods.insert(rp.clone());
            }
        }
        for m in &e.decl_rust {
            match rust_path_from_module(&src, m) {
                Some(p) if p.is_file() => {
                    let rp = rel_to(&ox, &p);
                    mods.insert(rp.clone());
                    declared_found[i].push(rp);
                }
                _ => declared_missing[i].push(m.clone()),
            }
        }
        let mut mv: Vec<String> = mods.into_iter().collect();
        mv.sort();
        impl_modules[i] = mv
            .iter()
            .filter(|m| !basename(m).starts_with(DISPATCH_BASENAME_PREFIX))
            .cloned()
            .collect();
        mention_modules[i] = mv;
    }

    // lean declarations (declared_found also carries these)
    let mut lean_modules: Vec<Vec<String>> = vec![Vec::new(); entries.len()];
    for (i, e) in entries.iter().enumerate() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for (rp, h) in &lean_hits {
            if h.id_occ.contains_key(&i) {
                set.insert(rp.clone());
            }
        }
        for m in &e.decl_lean {
            let p = ox
                .join("formal/lean")
                .join(format!("{}.lean", m.replace('.', "/")));
            if p.is_file() {
                let rp = rel_to(&ox, &p);
                set.insert(rp.clone());
                declared_found[i].push(rp);
            } else {
                declared_missing[i].push(m.clone());
            }
        }
        lean_modules[i] = set.into_iter().collect();
        declared_found[i].sort();
        declared_found[i].dedup();
        declared_missing[i].sort();
        declared_missing[i].dedup();
    }

    // ------------------------------------------- module -> ids mapped index
    let mut impl_to_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (i, e) in entries.iter().enumerate() {
        for m in &impl_modules[i] {
            impl_to_ids.entry(m.clone()).or_default().push(e.function_id.clone());
        }
    }
    for v in impl_to_ids.values_mut() {
        v.sort();
    }

    // --------------------------------------------------- shared scan fields
    let scan_inventory = J::Obj(vec![
        (
            "rust_function_modules_scanned".into(),
            J::Int(rust_files.len() as i64),
        ),
        (
            "surface_dispatch_files".into(),
            J::arr_str(sorted(dispatch_rel.clone())),
        ),
        (
            "registry_seed_files".into(),
            J::arr_str(sorted(registry_rel.clone())),
        ),
        ("test_files_scanned".into(), J::Int(test_files.len() as i64)),
        ("lean_sources_scanned".into(), J::Int(lean_files.len() as i64)),
        (
            "doc_trees_scanned".into(),
            J::arr_str(sorted(
                DOC_CATALOGS
                    .iter()
                    .map(|(_, r)| r.to_string())
                    .chain(["docs/bugs/streams".to_string(), "docs/function-lane".to_string()])
                    .collect::<Vec<_>>(),
            )),
        ),
    ]);

    let nf = |n: usize| if n == 1 { "file" } else { "files" };
    let searched_where: Vec<String> = sorted(vec![
        format!(
            "crates/oxfunc_core/src/functions/**/*.rs -- {} {}",
            rust_files.len(),
            nf(rust_files.len())
        ),
        format!(
            "crates/oxfunc_core/src registry seeds -- {} {}",
            registry_files.len(),
            nf(registry_files.len())
        ),
        format!(
            "crates/oxfunc_core/tests/** -- {} {}",
            test_files.len(),
            nf(test_files.len())
        ),
        format!(
            "docs/ deviation and discrepancy catalogs -- {} {}",
            DOC_CATALOGS.len(),
            nf(DOC_CATALOGS.len())
        ),
        format!(
            "docs/bugs/streams/** -- {} {}",
            stream_files.len(),
            nf(stream_files.len())
        ),
        format!(
            "docs/function-lane/** -- {} {}",
            lane_files.len(),
            nf(lane_files.len())
        ),
        format!(
            "formal/**/*.lean -- {} {}",
            lean_files.len(),
            nf(lean_files.len())
        ),
        format!(
            "rust modules named by data/functions/*.json artifacts that resolve outside crates/oxfunc_core/src/functions -- {} {}",
            extra_rust.len(),
            nf(extra_rust.len())
        ),
        format!(
            "smart-fuzzer/corpus/** -- {} {}",
            corpus_files.len(),
            nf(corpus_files.len())
        ),
    ]);

    let limits = J::arr_str(LIMIT_IDS.iter().map(|s| s.to_string()).collect::<Vec<_>>());

    // ------------------------------------------------------------- emit
    std::fs::create_dir_all(&args.out)
        .map_err(|e| format!("cannot create {}: {e}", args.out.display()))?;

    let mut written: BTreeSet<String> = BTreeSet::new();
    let mut index_entries: Vec<J> = Vec::new();
    let mut tier_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut sibling_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut low_conf = 0usize;
    let mut declared_present_count = 0usize;

    for (i, e) in entries.iter().enumerate() {
        let is_short = lex.short[i];
        if is_short {
            low_conf += 1;
        }
        if e.has_artifacts {
            declared_present_count += 1;
        }

        // siblings over impl modules
        let mut sibs: BTreeSet<String> = BTreeSet::new();
        for m in &impl_modules[i] {
            for id in impl_to_ids.get(m).map(|v| v.as_slice()).unwrap_or(&[]) {
                sibs.insert(id.clone());
            }
        }
        sibs.remove(&e.function_id);
        let sib_count = sibs.len();
        *sibling_hist.entry(sib_count).or_insert(0) += 1;

        let mut tests_per_module = Vec::new();
        let mut minus = Vec::new();
        let mut lines_per_module = Vec::new();
        let mut tests_total: i64 = 0;
        for m in &impl_modules[i] {
            let (t, l) = rust_meta.get(m).copied().unwrap_or((0, 0));
            tests_total += t as i64;
            let mapped = impl_to_ids.get(m).map(|v| v.len()).unwrap_or(0) as i64;
            tests_per_module.push((m.clone(), J::Int(t as i64)));
            minus.push((m.clone(), J::Int(t as i64 - mapped)));
            lines_per_module.push((m.clone(), J::Int(l as i64)));
        }

        let registered_in: Vec<String> = sorted(
            registry_rel
                .iter()
                .filter(|rp| {
                    registry_hits
                        .get(*rp)
                        .map(|h| h.quoted_ids.contains(&i))
                        .unwrap_or(false)
                })
                .cloned()
                .collect(),
        );
        let dispatch_in: Vec<String> = sorted(
            dispatch_rel
                .iter()
                .filter(|rp| {
                    rust_hits
                        .get(*rp)
                        .map(|h| h.quoted_ids.contains(&i))
                        .unwrap_or(false)
                })
                .cloned()
                .collect(),
        );

        let fixture_hits: Vec<String> = sorted(
            test_hits
                .iter()
                .filter(|(_, h)| h.id_occ.contains_key(&i) || h.ftok.contains(&i))
                .map(|(rp, _)| rp.clone())
                .collect(),
        );
        let corpus_hit_files: Vec<String> = sorted(
            corpus_hits
                .iter()
                .filter(|(_, h)| h.mentions(i))
                .map(|(rp, _)| rp.clone())
                .collect(),
        );

        // ------------------------------------------------ doc mentions
        let mut cat_counts: Vec<(String, J)> = Vec::new();
        for (key, rel) in DOC_CATALOGS.iter() {
            let n = doc_hits
                .get(*rel)
                .map(|h| h.mention_count(i))
                .unwrap_or(0);
            cat_counts.push(((*key).to_string(), J::Int(n as i64)));
        }
        let bug_stream_hits: Vec<String> = sorted(
            doc_hits
                .iter()
                .filter(|(rp, h)| stream_rel.contains(*rp) && h.mentions(i))
                .map(|(rp, _)| rp.clone())
                .collect(),
        );
        let lane_hit_count = doc_hits
            .iter()
            .filter(|(rp, h)| lane_rel.contains(*rp) && h.mentions(i))
            .count();

        let mut classification: Vec<J> = Vec::new();
        for (rp, h) in doc_hits.iter() {
            if !h.mentions(i) {
                continue;
            }
            let kind = classify_doc_file(
                rp,
                doc_bytes.get(rp).map(|v| v.as_slice()).unwrap_or(&[]),
                h.offsets.get(&i).map(|v| v.as_slice()).unwrap_or(&[]),
            );
            classification.push(J::Obj(vec![
                ("path".into(), J::s(rp)),
                ("kind".into(), J::s(kind)),
            ]));
        }

        let doc_mention_counts = J::Obj({
            let mut v = cat_counts;
            v.push(("bug_stream_files".into(), J::arr_str(bug_stream_hits)));
            v.push((
                "function_lane_file_count".into(),
                J::Int(lane_hit_count as i64),
            ));
            v
        });

        // ------------------------------------------------------- record
        let mut rec: Vec<(String, J)> = Vec::new();
        rec.push(("schema".into(), J::s(SCHEMA)));
        rec.push(("function_id".into(), J::s(&e.function_id)));
        rec.push(("surface_name".into(), J::s(&e.surface_name)));
        rec.push(("oxfunc_commit".into(), J::s(head)));
        rec.push(("oxfunc_tree_clean".into(), J::Bool(true)));
        rec.push((
            "name_match_confidence".into(),
            J::s(if is_short { "low" } else { "high" }),
        ));
        rec.push((
            "name_match_rule".into(),
            J::s(if is_short {
                "guarded_short_name"
            } else {
                "word_boundary"
            }),
        ));
        rec.push(("impl_modules".into(), J::arr_str(impl_modules[i].clone())));
        rec.push(("exclusion_rule_id".into(), J::s(EXCLUSION_RULE_ID)));
        rec.push((
            "mention_modules".into(),
            J::arr_str(mention_modules[i].clone()),
        ));
        rec.push((
            "module_shared_by".into(),
            J::arr_str(sibs.iter().cloned().collect::<Vec<_>>()),
        ));
        rec.push(("module_shared_by_count".into(), J::Int(sib_count as i64)));
        rec.push(("tests_in_impl_modules".into(), J::Int(tests_total)));
        rec.push(("tests_are_module_level".into(), J::Bool(true)));
        rec.push(("tests_per_module".into(), J::Obj(tests_per_module)));
        rec.push(("module_tests_minus_sibling_count".into(), J::Obj(minus)));
        rec.push(("source_lines_per_module".into(), J::Obj(lines_per_module)));
        rec.push(("lean_modules".into(), J::arr_str(lean_modules[i].clone())));
        rec.push(("lean_parsed".into(), J::Bool(false)));
        rec.push(("registered_in".into(), J::arr_str(registered_in)));
        rec.push(("dispatch_in".into(), J::arr_str(dispatch_in)));
        rec.push((
            "declared_artifacts_present".into(),
            J::Bool(e.has_artifacts),
        ));
        if e.has_artifacts {
            rec.push((
                "declared_vs_scanned".into(),
                J::Obj(vec![
                    (
                        "declared_found".into(),
                        J::arr_str(declared_found[i].clone()),
                    ),
                    (
                        "declared_missing_on_disk".into(),
                        J::arr_str(declared_missing[i].clone()),
                    ),
                ]),
            ));
        }
        rec.push(("doc_mention_counts".into(), doc_mention_counts));
        rec.push(("doc_mention_classification".into(), J::Arr(classification)));
        rec.push(("doc_mentions_are_not_verdicts".into(), J::Bool(true)));
        rec.push((
            "fuzzer_corpus_hit_files".into(),
            J::arr_str(corpus_hit_files),
        ));
        rec.push(("fixture_hits".into(), J::arr_str(fixture_hits)));
        rec.push(("scan_inventory".into(), scan_inventory.clone()));
        rec.push(("searched_where".into(), J::arr_str(searched_where.clone())));
        rec.push(("limits".into(), limits.clone()));

        let fname = format!("{}.json", e.function_id);
        let path = args.out.join(&fname);
        std::fs::write(&path, to_string_pretty(&J::Obj(rec)))
            .map_err(|err| format!("cannot write {}: {err}", path.display()))?;
        written.insert(fname.clone());

        // ---- index rollups (a total function of the files just written)
        let tier = depth_tier(impl_modules[i].len(), tests_total, sib_count);
        *tier_counts.entry(tier).or_insert(0) += 1;
        index_entries.push(J::Obj(vec![
            ("function_id".into(), J::s(&e.function_id)),
            ("surface_name".into(), J::s(&e.surface_name)),
            ("file".into(), J::s(&fname)),
            (
                "impl_module_count".into(),
                J::Int(impl_modules[i].len() as i64),
            ),
            ("tests_in_impl_modules".into(), J::Int(tests_total)),
            ("module_shared_by_count".into(), J::Int(sib_count as i64)),
            ("test_depth_tier".into(), J::s(tier)),
            (
                "name_match_confidence".into(),
                J::s(if is_short { "low" } else { "high" }),
            ),
        ]));
    }

    // ------------------------------------------------------------- index
    let mut modules_rows: Vec<J> = Vec::new();
    let mut negative_modules = 0usize;
    let mut touched: BTreeSet<String> = BTreeSet::new();
    for (m, ids) in impl_to_ids.iter() {
        let (t, l) = rust_meta.get(m).copied().unwrap_or((0, 0));
        let delta = t as i64 - ids.len() as i64;
        if delta < 0 {
            negative_modules += 1;
            for id in ids {
                touched.insert(id.clone());
            }
        }
        modules_rows.push(J::Obj(vec![
            ("module_path".into(), J::s(m)),
            ("ids_mapped".into(), J::Int(ids.len() as i64)),
            ("tests".into(), J::Int(t as i64)),
            ("source_lines".into(), J::Int(l as i64)),
            (
                "module_tests_minus_sibling_count".into(),
                J::Int(delta),
            ),
        ]));
    }

    let entries_with_module = entries
        .iter()
        .enumerate()
        .filter(|(i, _)| !impl_modules[*i].is_empty())
        .count();

    let index = J::Obj(vec![
        ("schema".into(), J::s(INDEX_SCHEMA)),
        ("oxfunc_commit".into(), J::s(head)),
        ("oxfunc_tree_clean".into(), J::Bool(true)),
        ("entry_count".into(), J::Int(entries.len() as i64)),
        ("exclusion_rule_id".into(), J::s(EXCLUSION_RULE_ID)),
        ("limits".into(), limits.clone()),
        ("scan_inventory".into(), scan_inventory.clone()),
        ("searched_where".into(), J::arr_str(searched_where.clone())),
        (
            "counts".into(),
            J::Obj(vec![
                (
                    "entries_with_at_least_one_impl_module".into(),
                    J::Int(entries_with_module as i64),
                ),
                (
                    "entries_with_no_impl_module".into(),
                    J::Int((entries.len() - entries_with_module) as i64),
                ),
                (
                    "distinct_impl_modules".into(),
                    J::Int(impl_to_ids.len() as i64),
                ),
                (
                    "impl_modules_with_negative_module_tests_minus_sibling_count".into(),
                    J::Int(negative_modules as i64),
                ),
                (
                    "entries_touched_by_a_negative_impl_module".into(),
                    J::Int(touched.len() as i64),
                ),
                (
                    "name_match_confidence_low".into(),
                    J::Int(low_conf as i64),
                ),
                (
                    "name_match_confidence_high".into(),
                    J::Int((entries.len() - low_conf) as i64),
                ),
                (
                    "declared_artifacts_present".into(),
                    J::Int(declared_present_count as i64),
                ),
            ]),
        ),
        (
            "test_depth_tiers".into(),
            J::Obj(
                ["D0", "D1", "D2", "D3", "D4", "D5"]
                    .iter()
                    .map(|t| ((*t).to_string(), J::Int(*tier_counts.get(t).unwrap_or(&0) as i64)))
                    .collect(),
            ),
        ),
        // Denominator stated next to the histogram, per A1-M4: this histogram
        // is over ALL entries, so the `0` bucket contains both the entries with
        // a sole-occupant module and the entries with no located module at all.
        (
            "sibling_histogram_entry_count".into(),
            J::Int(entries.len() as i64),
        ),
        (
            "sibling_histogram_entries_with_at_least_one_impl_module".into(),
            J::Int(entries_with_module as i64),
        ),
        (
            "sibling_histogram".into(),
            J::Obj({
                // data-keyed object -> ORDINAL key order, not numeric order
                let mut rows: Vec<(String, J)> = sibling_hist
                    .iter()
                    .map(|(k, v)| (k.to_string(), J::Int(*v as i64)))
                    .collect();
                rows.sort_by(|a, b| a.0.cmp(&b.0));
                rows
            }),
        ),
        ("impl_modules".into(), J::Arr(modules_rows)),
        ("entries".into(), J::Arr(index_entries)),
    ]);
    std::fs::write(args.out.join("index.json"), to_string_pretty(&index))
        .map_err(|e| format!("cannot write index.json: {e}"))?;
    written.insert("index.json".to_string());

    // Remove stale .json files this run did not produce, so a second run over a
    // shrinking id set still byte-compares.
    if let Ok(rd) = std::fs::read_dir(&args.out) {
        for f in rd.flatten() {
            let n = f.file_name().to_string_lossy().to_string();
            if n.ends_with(".json") && !written.contains(&n) {
                let _ = std::fs::remove_file(f.path());
            }
        }
    }

    Ok(written.len())
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

fn rust_path_from_module(src: &Path, m: &str) -> Option<PathBuf> {
    let parts: Vec<&str> = m.split("::").collect();
    if parts.len() < 2 || parts[0] != "oxfunc_core" {
        return None;
    }
    let mut p = src.to_path_buf();
    for seg in &parts[1..parts.len() - 1] {
        p = p.join(seg);
    }
    Some(p.join(format!("{}.rs", parts[parts.len() - 1])))
}

/// §3.7 test-depth tiers, computed only from fields written into F2.
/// D5 requires `vectors/<id>/vN/MANIFEST.json`, of which there are none, so this
/// function can never return D5 and does not pretend to look.
fn depth_tier(impl_module_count: usize, tests: i64, siblings: usize) -> &'static str {
    if impl_module_count == 0 {
        "D0"
    } else if tests == 0 {
        "D1"
    } else if siblings >= 5 {
        "D2"
    } else if siblings >= 1 {
        "D3"
    } else {
        "D4"
    }
}

/// Rule DMC-1. Purely mechanical; the prose statement lives in F14.
///
/// 1. basename (uppercased) contains a BEHAVIOURAL filename token -> behavioural-finding
/// 2. basename (uppercased) contains a BULK filename token        -> bulk-inventory
/// 3. extension is not `.md`                                      -> bulk-inventory
/// 4. some occurrence is NOT on a Markdown table row AND its nearest preceding
///    ATX header contains a BEHAVIOURAL header token              -> behavioural-finding
/// 5. otherwise                                                   -> bulk-inventory
fn classify_doc_file(rel: &str, bytes: &[u8], offsets: &[usize]) -> &'static str {
    let base = basename(rel).to_uppercase();
    if BEHAVIOURAL_FILENAME_TOKENS.iter().any(|t| base.contains(t)) {
        return KIND_BEHAVIOURAL;
    }
    if BULK_FILENAME_TOKENS.iter().any(|t| base.contains(t)) {
        return KIND_BULK;
    }
    if !base.ends_with(".MD") {
        return KIND_BULK;
    }
    for &o in offsets {
        if on_table_row(bytes, o) {
            continue;
        }
        let h = nearest_atx_header(bytes, o);
        if BEHAVIOURAL_HEADER_TOKENS.iter().any(|t| h.contains(t)) {
            return KIND_BEHAVIOURAL;
        }
    }
    KIND_BULK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmc1_filename_clauses() {
        assert_eq!(
            classify_doc_file("docs/function-lane/XLCALL_CODE_CATALOG.csv", b"", &[]),
            KIND_BULK
        );
        assert_eq!(
            classify_doc_file(
                "docs/bugs/streams/BUG-FUNC-005_power_zero_to_zero_diverges_from_excel.md",
                b"",
                &[]
            ),
            KIND_BEHAVIOURAL
        );
        assert_eq!(
            classify_doc_file("docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md", b"", &[]),
            KIND_BEHAVIOURAL
        );
        // behavioural token beats bulk token when both are present
        assert_eq!(
            classify_doc_file("docs/x/DEVIATION_CATALOG.md", b"", &[]),
            KIND_BEHAVIOURAL
        );
    }

    #[test]
    fn dmc1_header_clause() {
        let doc = b"# X\n\n## Observed behaviour\n\nABS returns 1.\n\n## Notes\n";
        let off = doc.windows(3).position(|w| w == b"ABS").unwrap();
        assert_eq!(classify_doc_file("docs/x/PLAIN.md", doc, &[off]), KIND_BEHAVIOURAL);
        let doc2 = b"# X\n\n## Rows\n\n| ABS | y |\n";
        let off2 = doc2.windows(3).position(|w| w == b"ABS").unwrap();
        assert_eq!(classify_doc_file("docs/x/PLAIN.md", doc2, &[off2]), KIND_BULK);
    }

    #[test]
    fn depth_tiers_follow_section_3_7() {
        assert_eq!(depth_tier(0, 0, 0), "D0");
        assert_eq!(depth_tier(1, 0, 9), "D1");
        assert_eq!(depth_tier(1, 3, 5), "D2");
        assert_eq!(depth_tier(1, 3, 4), "D3");
        assert_eq!(depth_tier(1, 3, 1), "D3");
        assert_eq!(depth_tier(1, 3, 0), "D4");
    }

    #[test]
    fn module_path_resolution() {
        let src = Path::new("/s");
        assert_eq!(
            fslash(&rust_path_from_module(src, "oxfunc_core::functions::abs").unwrap()),
            "/s/functions/abs.rs"
        );
        assert_eq!(
            fslash(&rust_path_from_module(src, "oxfunc_core::locale_format").unwrap()),
            "/s/locale_format.rs"
        );
        assert!(rust_path_from_module(src, "other::x").is_none());
    }
}
