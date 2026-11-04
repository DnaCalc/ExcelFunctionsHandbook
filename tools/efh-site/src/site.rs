use crate::{checks, markdown::Renderer, util::*};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

const CSS: &str = include_str!("style.css");
const JS: &str = include_str!("site.js");
const MARKER: &str = ".efh-site-output";

fn page(path: &str, title: &str, body: &str, home: bool) -> String {
    let link = |to: &str| esc(&relative(path, to));
    format!(
        r##"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="light dark"><meta name="referrer" content="no-referrer"><title>{title} · The Handbook of Excel Functions</title><link rel="stylesheet" href="{css}"><script src="{js}" defer></script></head>
<body><a class="skip" href="#main">Skip to content</a><div class="wrap"><header><nav class="mainnav" aria-label="Main"><a class="brand" href="{root}">DNA CALC <span>Handbook</span></a><a href="{index}">Functions</a><a href="{notes}">Notes</a><a href="{lastbit}">The Last Bit</a><a href="{coverage}">Coverage</a></nav><div class="lensbar" role="group" aria-label="Reading lens" hidden><span>Reading</span><button type="button" data-lens="working" aria-pressed="true">Working</button><button type="button" data-lens="explicit" aria-pressed="false">Explicit</button></div></header>
<main id="main" tabindex="-1"{class}>{body}</main><footer><p>{reconciliation}.</p><p><a href="{licence}">Content: CC BY 4.0 · Code: MIT</a> · <a href="{rules}">How to read the claims</a> · <a href="{api}">Static JSON index</a> · No tracking.</p><p class="small">The Handbook of Excel Functions · DNA Kode. Independent of Microsoft.</p></footer></div></body></html>"##,
        title = esc(title),
        css = link("assets/style.css"),
        js = link("assets/site.js"),
        root = link("index.html"),
        index = link("functions/index.html"),
        notes = link("notes/index.html"),
        lastbit = link("lastbit/index.html"),
        coverage = link("coverage/index.html"),
        licence = link("licence/index.html"),
        rules = link("rules/index.html"),
        api = link("api/index.json"),
        reconciliation = RECONCILIATION,
        class = if home { " class=\"homepage\"" } else { "" }
    )
}

fn output(root: &Path, path: &str, title: &str, body: &str) -> Result<(), String> {
    write(root, path, &page(path, title, body, path == "index.html"))
}
fn a(from: &str, to: &str, label: &str) -> String {
    format!(
        "<a href=\"{}\">{}</a>",
        esc(&relative(from, to)),
        esc(label)
    )
}
fn function_path(id: &str) -> String {
    format!("functions/{}.html", id.strip_prefix("FUNC.").unwrap_or(id))
}
fn source_link(revision: &str, source: &str) -> String {
    format!("{SOURCE}/blob/{revision}/{source}")
}

struct Inputs {
    index: Value,
    functions: Vec<Value>,
    notes: Vec<Value>,
    curated: BTreeSet<String>,
    revision: String,
    tree_state: String,
    note_api_matches: bool,
}
impl Inputs {
    fn load(repo: &Path) -> Result<Self, String> {
        let index = json(&repo.join("data/index.json"))?;
        let rows = index["rows"]
            .as_array()
            .ok_or("data/index.json: rows is not an array")?;
        if rows.len() != 541
            || index["counts"]["functions"] != 518
            || index["counts"]["operators"] != 23
        {
            return Err(
                "catalogue no longer reconciles to Charter §7.3; review the denominator".into(),
            );
        }
        let mut functions = Vec::new();
        let mut curated = BTreeSet::new();
        let mut ids = BTreeSet::new();
        for row in rows {
            let id = s(&row["function_id"]);
            if !id.starts_with("FUNC.")
                || !id
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_')
                || !ids.insert(id.to_string())
            {
                return Err(format!("invalid or duplicate function id {id:?}"));
            }
            let f = json(&repo.join(format!("data/functions/{id}.json")))?;
            if s(&f["function_id"]) != id
                || f["entry_kind"] != row["entry_kind"]
                || f["surface_name"] != row["surface_name"]
                || f["category"] != row["category"]
            {
                return Err(format!(
                    "projection identity/category disagrees with index row: {id}"
                ));
            }
            if repo.join(format!("content/functions/{id}.md")).is_file() {
                curated.insert(id.to_string());
            }
            functions.push(f);
        }
        let published_rows = functions
            .iter()
            .map(|f| s(&f["published_row_id"]))
            .collect::<BTreeSet<_>>();
        if functions
            .iter()
            .filter(|f| f["entry_kind"] == "built_in_function")
            .count()
            != 518
            || functions
                .iter()
                .filter(|f| f["entry_kind"] == "built_in_operator")
                .count()
                != 23
            || published_rows.len() != 534
            || published_rows.contains("")
        {
            return Err("entry projections do not reconcile to 518 functions, 23 operators and 534 published rows".into());
        }
        let mut notes = Vec::new();
        let mut note_ids = BTreeSet::new();
        for path in files(&repo.join("content/notes"))? {
            if !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("FN-")
                || path.extension().is_none_or(|x| x != "json")
            {
                continue;
            }
            let note = json(&path)?;
            let id = s(&note["note_id"]);
            if note["schema"] != "efh.function-note/v1"
                || !note_ids.insert(id.to_string())
                || path.file_stem().unwrap() != id
            {
                return Err(format!("invalid or duplicate note {}", path.display()));
            }
            let subjects = note["functions"]
                .as_array()
                .ok_or_else(|| format!("{id}: missing functions"))?;
            if subjects.is_empty()
                || [
                    "title",
                    "statement",
                    "status",
                    "kind",
                    "basis",
                    "recorded_on",
                ]
                .iter()
                .any(|key| s(&note[key]).is_empty())
                || note["repros"]
                    .as_array()
                    .is_none_or(|repros| repros.is_empty())
            {
                return Err(format!(
                    "{id}: note lacks a title, statement, scope, subjects or reproductions"
                ));
            }
            if !matches!(
                s(&note["kind"]),
                "excel-differs-from-math"
                    | "documentation-differs"
                    | "quirk"
                    | "bit-matching-detail"
            ) || !matches!(s(&note["status"]), "measured" | "inferred")
            {
                return Err(format!(
                    "{id}: unknown note kind or status; review the note index taxonomy"
                ));
            }
            for subject in subjects {
                if !ids.contains(s(subject)) {
                    return Err(format!("{id}: unknown function {subject}"));
                }
            }
            notes.push(note);
        }
        notes.sort_by_key(|n| s(&n["note_id"]).to_string());
        let mut api_notes = json(&repo.join("site/api/notes.json"))?["notes"]
            .as_array()
            .ok_or("site/api/notes.json: missing notes")?
            .clone();
        api_notes.sort_by_key(|n| s(&n["note_id"]).to_string());
        let note_api_matches = api_notes == notes;
        if files(&repo.join("ledger"))?.iter().any(|p| {
            p.extension().is_none_or(|ext| ext != "md")
                && p.file_name().is_some_and(|name| name != ".gitkeep")
        }) {
            return Err("ledger artifacts are now present: integrate their answer envelopes before using the pre-ledger renderer".into());
        }
        // Suite admission belongs to the existing pipeline; do not invent warrants if new
        // suites or implementations arrive without integration into this renderer.
        if files(&repo.join("vectors"))?
            .iter()
            .any(|p| p.file_name().is_some_and(|n| n == "MANIFEST.json"))
            || files(&repo.join("implementations"))?
                .iter()
                .any(|p| p.file_name().is_some_and(|n| n != ".gitkeep"))
        {
            return Err("new suites/implementations need an admission review before this site renderer can list them".into());
        }
        let git = std::process::Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["rev-parse", "HEAD"])
            .output()
            .map_err(|e| e.to_string())?;
        if !git.status.success() {
            return Err("cannot determine source revision".into());
        }
        let revision = String::from_utf8(git.stdout)
            .map_err(|e| e.to_string())?
            .trim()
            .to_string();
        let status = std::process::Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["status", "--porcelain", "--untracked-files=normal"])
            .output()
            .map_err(|e| e.to_string())?;
        if !status.status.success() {
            return Err("cannot determine source working-tree state".into());
        }
        let tree_state = if status.stdout.is_empty() {
            "clean"
        } else {
            "dirty"
        }
        .to_string();
        Ok(Self {
            index,
            functions,
            notes,
            curated,
            revision,
            tree_state,
            note_api_matches,
        })
    }
}

fn ensure_output_directory(out: &Path) -> Result<(), String> {
    if out
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        return Err(format!("refusing symlink output {}", out.display()));
    }
    if out.exists()
        && (!out.is_dir()
            || (!out.join(MARKER).is_file()
                && std::fs::read_dir(out)
                    .map_err(|e| e.to_string())?
                    .next()
                    .is_some()))
    {
        return Err(format!(
            "refusing to replace nonempty unmarked directory {}",
            out.display()
        ));
    }
    Ok(())
}
fn install_output(stage: &Path, out: &Path) -> Result<(), String> {
    ensure_output_directory(out)?;
    let previous = stage.with_file_name(format!(".efh-site-previous-{}", std::process::id()));
    if previous.exists() {
        return Err(format!(
            "previous build backup already exists: {}",
            previous.display()
        ));
    }
    let had_previous = out.exists();
    if had_previous {
        std::fs::rename(out, &previous).map_err(|e| e.to_string())?;
    }
    if let Err(e) = std::fs::rename(stage, out) {
        if had_previous {
            std::fs::rename(&previous, out).map_err(|rollback| {
                format!(
                    "install failed: {e}; rollback failed: {rollback}; previous output is at {}",
                    previous.display()
                )
            })?;
        }
        return Err(format!("install failed, earlier output preserved: {e}"));
    }
    if had_previous {
        std::fs::remove_dir_all(&previous).map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn build(repo: &Path, out: &Path) -> Result<(), String> {
    let out = if out.is_absolute() {
        out.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(out)
    };
    ensure_output_directory(&out)?;
    let input = Inputs::load(repo)?;
    let parent = out.parent().ok_or("output must have a parent")?;
    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let stage = parent.join(format!(".efh-site-stage-{}", std::process::id()));
    std::fs::create_dir(&stage).map_err(|e| format!("staging {}: {e}", stage.display()))?;
    let result = generate(repo, &stage, &input).and_then(|_| checks::check(&stage, true, true));
    if let Err(e) = result {
        let _ = std::fs::remove_dir_all(&stage);
        return Err(e);
    }
    install_output(&stage, &out)?;
    let paths = files(&out)?;
    let bytes: u64 = paths
        .iter()
        .map(|p| p.metadata().map(|m| m.len()).unwrap_or(0))
        .sum();
    println!(
        "Wrote {} files, {bytes} bytes to {} (541 entry pages)",
        paths.len(),
        out.display()
    );
    Ok(())
}

fn generate(repo: &Path, out: &Path, input: &Inputs) -> Result<(), String> {
    write(
        out,
        MARKER,
        "Generated by efh-site. This folder may be replaced by efh-site build.\n",
    )?;
    write(out, "assets/style.css", CSS)?;
    write(out, "assets/site.js", JS)?;
    let mut routes = BTreeMap::new();
    let mut docs = Vec::<(String, String, String)>::new();
    for f in &input.functions {
        let id = s(&f["function_id"]);
        let dest = function_path(id);
        routes.insert(
            format!("data/functions/{id}.json"),
            format!("api/functions/{id}.json"),
        );
        routes.insert(format!("content/functions/{id}.md"), dest);
        write(
            out,
            &format!("api/functions/{id}.json"),
            &read(&repo.join(format!("data/functions/{id}.json")))?,
        )?;
    }
    routes.insert("content/notes/NOTES.md".into(), "notes/index.html".into());
    routes.insert("site/api/notes.json".into(), "api/notes.json".into());
    routes.insert("data/index.json".into(), "api/index.json".into());
    routes.insert("site/api/rubric.json".into(), "api/rubric.json".into());
    for path in files(&repo.join("content/model"))? {
        if path.extension().is_some_and(|x| x == "md") {
            let src = path
                .strip_prefix(repo)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let dest = format!("model/{stem}.html");
            let raw = read(&path)?;
            let title = crate::markdown::body(&raw)
                .lines()
                .find_map(|l| l.strip_prefix("# "))
                .unwrap_or(stem)
                .to_string();
            routes.insert(src.clone(), dest.clone());
            docs.push((src, dest, title));
        }
    }
    let mut episodes = Vec::new();
    for path in files(&repo.join("content/lastbit"))? {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or("non-UTF8 episode name")?;
        if name.starts_with("MATERIAL_") || path.extension().is_none_or(|x| x != "md") {
            continue;
        }
        let raw = read(&path)?.replace("\r\n", "\n");
        let src = path
            .strip_prefix(repo)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let title = crate::markdown::body(&raw)
            .lines()
            .find_map(|l| l.strip_prefix("# "))
            .unwrap_or(name)
            .to_string();
        // Publication requires an explicit frontmatter declaration. Filename/date do not
        // silently turn a draft into a finished article.
        let ready = raw.starts_with("---\n")
            && raw
                .split("\n---\n")
                .next()
                .unwrap_or("")
                .lines()
                .any(|l| matches!(l, "status: ready" | "status: published"));
        let dest = format!(
            "lastbit/{}.html",
            slug(path.file_stem().unwrap().to_str().unwrap())
        );
        routes.insert(src.clone(), dest.clone());
        episodes.push((src, dest, title, ready));
    }
    for (src, dest, title) in [
        (
            "content/notes/HOWTO.md",
            "notes/howto.html",
            "Adding a function note",
        ),
        (
            "content/evidence/EXCEL-ENVIRONMENTS.md",
            "evidence/excel-environments.html",
            "Recorded Excel environments",
        ),
    ] {
        if repo.join(src).is_file() {
            routes.insert(src.into(), dest.into());
            docs.push((src.into(), dest.into(), title.into()));
        }
    }
    for path in files(&repo.join("content/notes/evidence"))? {
        let src = path
            .strip_prefix(repo)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let dest = format!("sources/{src}");
        if path.extension().is_some_and(|x| x == "md") {
            let html = dest.trim_end_matches(".md").to_string() + ".html";
            routes.insert(src.clone(), html.clone());
            docs.push((
                src,
                html,
                path.file_stem().unwrap().to_string_lossy().into_owned(),
            ));
        } else {
            routes.insert(src.clone(), dest.clone());
            let target = out.join(dest);
            std::fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
            std::fs::copy(&path, target).map_err(|e| e.to_string())?;
        }
    }
    let mut r = Renderer {
        repo,
        routes: &routes,
        revision: &input.revision,
        withheld: Vec::new(),
        unresolved: BTreeSet::new(),
    };
    for (src, dest, title) in docs {
        let rendered = r.render(&src, &dest, &read(&repo.join(&src))?, false);
        let body=format!("<p class=\"eyebrow\">Research record · source document</p><h1>{}</h1><p class=\"small\">{}</p><article>{rendered}</article>",esc(&title),a(&dest,"rules/index.html","Reading the scope and limits"));
        output(out, &dest, &title, &body)?;
    }
    for (src, dest, title, ready) in &episodes {
        let rendered = r.render(src, dest, &read(&repo.join(src))?, false);
        let label = if *ready {
            "Series article"
        } else {
            "Draft · research essay"
        };
        let body=format!("<p class=\"eyebrow\">The Last Bit</p><h1>{}</h1><p class=\"status\">{label}</p><article>{rendered}</article>",esc(title));
        output(out, dest, title, &body)?;
    }
    for f in &input.functions {
        function_page(repo, out, input, f, &mut r)?;
    }
    note_pages(out, input, &mut r)?;
    function_index(out, input)?;
    homepage(out, input, &episodes)?;
    let mut series=String::from("<p class=\"eyebrow\">Research essays</p><h1>The Last Bit</h1><p>Investigating how Excel computes its answers, with the scope and limits of each measurement kept in view.</p><ul class=\"entries\">");
    for (_, dest, title, ready) in &episodes {
        series += &format!(
            "<li>{}<span class=\"small\">{}</span></li>",
            a("lastbit/index.html", dest, title),
            if *ready { "Series article" } else { "Draft" }
        );
    }
    series += "</ul><p>Internal research materials are excluded from this site.</p>";
    output(out, "lastbit/index.html", "The Last Bit", &series)?;
    let licences = format!(
        "<h1>Licences and attribution</h1><article>{}<h2>Code: MIT</h2>{}</article>",
        r.render(
            "LICENSE-CONTENT.md",
            "licence/index.html",
            &read(&repo.join("LICENSE-CONTENT.md"))?,
            false
        ),
        r.render(
            "LICENSE.md",
            "licence/index.html",
            &read(&repo.join("LICENSE.md"))?,
            false
        )
    );
    output(
        out,
        "licence/index.html",
        "Licences and attribution",
        &licences,
    )?;
    let rules = format!(
        "{RULES}<p><a href=\"{}\">Read the governing charter</a>.</p>",
        source_link(&input.revision, "CHARTER.md")
    );
    output(out, "rules/index.html", "Reading the claims", &rules)?;
    let withheld = r.withheld;
    let unresolved = r.unresolved;
    coverage(out, input, &withheld, &unresolved)?;
    write(
        out,
        "api/index.json",
        &serde_json::to_string_pretty(&input.index).unwrap(),
    )?;
    write(out,"api/notes.json",&serde_json::to_string_pretty(&json!({"schema":"efh.function-notes-api/v1","source":"content/notes/FN-*.json (efh.function-note/v1)","note_count":input.notes.len(),"notes":input.notes})).unwrap())?;
    // Derivations are downloadable source records, not current ledger warrants.
    for name in ["rubric.json", "rubric-hygiene.json"] {
        if repo.join(format!("site/api/{name}")).is_file() {
            write(
                out,
                &format!("api/{name}"),
                &read(&repo.join(format!("site/api/{name}")))?,
            )?;
        }
    }
    write(out,"api/build.json",&serde_json::to_string_pretty(&json!({"schema":"efh.static-build/v1","source_revision":input.revision,"source_tree_state":input.tree_state,"basis_mode":"pre-ledger","entry_pages":541,"curated_prose_files":input.curated.len(),"notes":input.notes.len(),"input_notes_api_matches_sources":input.note_api_matches,"withheld_passages":withheld,"unresolved_source_references":unresolved,"reconciliation":RECONCILIATION})).unwrap())?;
    Ok(())
}

fn function_page(
    repo: &Path,
    out: &Path,
    input: &Inputs,
    f: &Value,
    r: &mut Renderer,
) -> Result<(), String> {
    let id = s(&f["function_id"]);
    let name = s(&f["surface_name"]);
    let dest = function_path(id);
    let curated = input.curated.contains(id);
    let depth = if curated {
        "Curated prose present"
    } else {
        "Projection only · no curated prose"
    };
    let admission = s(&f["admission"]["label"]);
    let mut html=format!("<p class=\"eyebrow\">{} · <code>{}</code></p><h1 class=\"function-name\">{}</h1><p>{}</p><div class=\"statusline\"><span class=\"status\">{depth}</span><span class=\"status\">OxFunc admission: {}</span></div>",esc(s(&f["category"])),esc(id),esc(name),esc(s(&f["descriptions"]["short"])),esc(admission));
    if let Some(reason) = f["admission"]["reason"].as_str() {
        html += &format!("<p class=\"notice\">{}</p>", esc(reason));
    }
    html+="<p class=\"explain\">Curation describes prose in this checkout. Admission describes the reference engine in the projection. Neither is a test-suite warrant or a statement that Excel behaves identically.</p>";
    if let Some(signature) = f["signature"]["display"].as_str() {
        html+=&format!("<section class=\"card signature\" aria-label=\"Projected signature\"><code>{}</code><p class=\"small\">{}</p></section>",esc(signature),if f["signature_placeholder"]==true {"Placeholder signature in the projection; argument names are not curated."} else {"Signature from the OxFunc projection; see source limits below."});
    } else {
        html += "<p class=\"notice\">No signature is recorded in this projection.</p>";
    }
    if let Some(url) = f["docs"]["microsoft_url"].as_str() {
        html += &format!(
            "<p><a href=\"{}\">Microsoft documentation</a></p>",
            esc(url)
        );
    }
    html += &provenance(input, f);
    let notes: Vec<_> = input
        .notes
        .iter()
        .filter(|n| {
            n["functions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|x| s(x) == id)
        })
        .collect();
    if !notes.is_empty() {
        html+="<section aria-labelledby=\"function-notes\"><h2 id=\"function-notes\">Function notes</h2><p class=\"notice\">These notes are the later measurement record. Older prose below may describe behaviour that had not been measured when it was written. Build and machine limits travel with each recorded answer.</p>";
        for note in notes {
            html += &render_note(note, &dest, r);
        }
        html += "</section>";
    }
    if curated {
        let src = format!("content/functions/{id}.md");
        html += &format!(
            "<hr><article aria-label=\"Curated function prose\">{}</article>",
            r.render(&src, &dest, &read(&repo.join(&src))?, true)
        );
    } else {
        html+="<section><h2>About this entry</h2><p>This entry has projected metadata. No curated explanation, argument guide or examples are present in this checkout.</p></section>";
    }
    html+="<section><h2>Implementations and test suites</h2><p>No Handbook test suite or admitted Handbook implementation is published for this entry. Reference-engine source locations are research references, without a Handbook suite warrant.</p></section>";
    if let Some(classification) = f["classification"].as_object() {
        html+="<details><summary>Reference-engine classification</summary><p>These fields describe the projected OxFunc implementation. Defaults can be unexamined; these are not Excel observations.</p><dl>";
        for (key, value) in classification {
            html += &format!(
                "<dt>{}</dt><dd><code>{}</code> · {}</dd>",
                esc(key),
                esc(s(value)),
                esc(s(&f["axis_provenance"][key]["provenance_class"]))
            );
        }
        html += "</dl></details>";
    }
    if let Some(names) = f["localized_names"].as_array().filter(|x| !x.is_empty()) {
        html+="<details><summary>Projected localized names</summary><p>Source spellings are preserved, including questionable translations. They have not been checked here against installed Excel; consult the official article. Microsoft source text is excluded from the content licence.</p><dl class=\"locales\">";
        for n in names {
            html += &format!(
                "<dt>{}</dt><dd>{}</dd>",
                esc(s(&n["locale"])),
                esc(s(&n["name"]))
            );
        }
        html += "</dl></details>";
    }
    html += &doors(input, f, &dest);
    output(out, &dest, name, &html)
}

fn provenance(input: &Inputs, f: &Value) -> String {
    let p = &input.index["provenance"];
    let date = if p["oxfunc_commit_at_ingest"] == "473efa3" {
        " The registry checkout is dated 2026-07-25 in Operations §4; individual registry fields have no recorded as-of date."
    } else {
        " The registry basis has no recorded as-of date."
    };
    let presence = match f["registry_backed"].as_bool() {
        Some(true) => "Live-registry entry: present. ",
        Some(false) => "Live-registry entry: absent; export-only metadata. ",
        None => "",
    };
    format!("<aside class=\"provenance\"><h2>Source limits</h2><p>Mixed-vintage projection: the export row spine is dated <strong>{}</strong>, commit <code>{}</code>, tree <strong>{}</strong>. Its unrecorded working-tree delta prevents regeneration from that commit. Registry metadata comes from <code>{}</code> (clean tree: {}).{date} Individual field bases are recorded in the entry JSON.</p><p class=\"small\">{presence}Currentness against today's Excel is not established.</p></aside>",esc(s(&p["export_snapshot_generation"])),esc(s(&p["export_source_commit"])),esc(s(&p["export_source_tree_state"])),esc(s(&p["oxfunc_commit_at_ingest"])),p["oxfunc_tree_clean"])
}

fn doors(input: &Inputs, f: &Value, dest: &str) -> String {
    let id = s(&f["function_id"]);
    let source = format!("data/functions/{id}.json");
    let history_source = if input.curated.contains(id) {
        format!("content/functions/{id}.md")
    } else {
        source.clone()
    };
    format!(
        r#"<section class="doors" aria-labelledby="accountability"><h2 id="accountability">Why, sources and limits</h2><p>Pre-ledger view. No ledger receipt, answer identity or context hash has been issued. Working and Explicit expand the same source record.</p>
<details><summary>Why</summary><p>Metadata is read from the committed projection; curated prose and note JSON are rendered when present. Curation labels describe file presence, without a claim of behavioural completeness.</p></details>
<details><summary>Sources</summary><p>{raw} · <a href="{source}">Repository projection</a> · <a href="{notes}">Note records</a></p></details>
<details><summary>As of</summary><p>Source checkout <code>{revision}</code>, working tree {tree_state}. The projection's export and registry bases differ; see Source limits above. Each note states its own capture context.</p></details>
<details><summary>Rules</summary><p>{rules}. No suite-backed equality label is available while Handbook suites are absent.</p></details>
<details><summary>History</summary><p><a href="{history}">Repository history for this entry</a>. No ledger decision history is available.</p></details>
<details><summary>Limits</summary><p>No Handbook suite, admitted implementation or ledger warrant. Projected source facts can be stale; per-entry prose and notes state further limits. Mathematical arguments do not establish behaviour in Excel.</p></details>
<details><summary>Replay</summary><p>No packaged Handbook suite is available to replay. Note reproductions state their input route, raw bits where recorded, and capture source. Those records are observations rather than a downloadable suite.</p></details></section>"#,
        raw = a(dest, &format!("api/functions/{id}.json"), "Entry JSON"),
        source = source_link(&input.revision, &source),
        notes = esc(&relative(dest, "api/notes.json")),
        revision = esc(&input.revision),
        tree_state = esc(&input.tree_state),
        rules = a(dest, "rules/index.html", "Claim-language rules"),
        history = format_args!("{SOURCE}/commits/{}/{history_source}", input.revision)
    )
}

fn cell_value(v: &Value) -> String {
    if v.is_null() {
        return "—".into();
    }
    let mut text = format!("<code>{}</code>", esc(s(&v["display"])));
    if let Some(bits) = v["bits"].as_str() {
        text += &format!("<br><code class=\"small\">{}</code>", esc(bits));
    }
    text
}
fn source_ref(source: &Value, dest: &str, r: &mut Renderer) -> String {
    let repo = s(&source["repository"]);
    let path = s(&source["path"]);
    let commit = s(&source["commit"]);
    let url = if repo == "ExcelFunctionsHandbook" || repo == "Handbook" || repo.is_empty() {
        r.url("README.md", dest, path)
    } else if !repo.contains(['/', ':', '\\']) && !path.contains("..") {
        Some(format!(
            "https://github.com/DnaCalc/{repo}/blob/{commit}/{path}"
        ))
    } else {
        None
    };
    let label = format!("{repo} · {path}");
    format!(
        "{} <span class=\"small\">{} · {} {}</span>",
        url.map(|u| format!("<a href=\"{}\">{}</a>", esc(&u), esc(&label)))
            .unwrap_or_else(|| esc(&label)),
        esc(commit),
        esc(s(&source["locator"])),
        esc(s(&source["note"]))
    )
}
fn render_note(note: &Value, dest: &str, r: &mut Renderer) -> String {
    let id = s(&note["note_id"]);
    let kind = match s(&note["kind"]) {
        "excel-differs-from-math" => "Excel differs from the mathematical answer",
        "documentation-differs" => "Documentation differs from behaviour",
        "bit-matching-detail" => "Detail for matching Excel's bits",
        _ => "Quirk",
    };
    let mut h=format!("<article class=\"note\" id=\"{}\"><h3>{}: {}</h3><p class=\"small\">{} · {} · recorded {}</p><p>{}</p>",esc(id),esc(id),esc(s(&note["title"])),esc(kind),esc(s(&note["status"])),esc(s(&note["recorded_on"])),esc(s(&note["statement"])));
    if note["documentation"].is_object() {
        let doc = &note["documentation"];
        let sources = regex::Regex::new(r"https?://[^\s;]+")
            .unwrap()
            .find_iter(s(&doc["source"]))
            .map(|m| format!("<a href=\"{}\">Source documentation</a>", esc(m.as_str())))
            .collect::<Vec<_>>()
            .join(" · ");
        h += &format!(
            "<p><strong>Documentation:</strong> {} {sources}</p>",
            esc(s(&doc["claim"]))
        );
    }
    h+="<div class=\"table-scroll\" role=\"region\" aria-label=\"Recorded calls and answers\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">Call and input route</th><th scope=\"col\">Inputs</th><th scope=\"col\">Excel</th><th scope=\"col\">Correct value</th><th scope=\"col\">Recorded in</th></tr></thead><tbody>";
    for repro in note["repros"].as_array().into_iter().flatten() {
        let inputs = repro["inputs"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|i| format!("{}: {}", esc(s(&i["name"])), cell_value(&i["value"])))
            .collect::<Vec<_>>()
            .join("<br>");
        h += &format!(
            "<tr><td><code>{}</code><br>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(s(&repro["call"])),
            esc(s(&repro["input_route"])),
            if inputs.is_empty() {
                "As typed"
            } else {
                &inputs
            },
            cell_value(&repro["excel"]),
            cell_value(&repro["correct"]),
            esc(s(&repro["recorded_in"]))
        );
    }
    h += "</tbody></table></div>";
    for repro in note["repros"].as_array().into_iter().flatten() {
        if let Some(how) = repro["correct"]["how"].as_str() {
            h += &format!(
                "<p class=\"small\"><strong>Correct value:</strong> {}</p>",
                esc(how)
            );
        }
    }
    h += &format!("<p><strong>Basis:</strong> {}</p>", esc(s(&note["basis"])));
    if let Some(inferred) = note["inferred"].as_str() {
        h += &format!("<p><strong>Inferred:</strong> {}</p>", esc(inferred));
    }
    for open in note["open"].as_array().into_iter().flatten() {
        h += &format!("<p><strong>Open:</strong> {}</p>", esc(s(open)));
    }
    h += "<details><summary>Evidence and capture sources</summary><ul>";
    for evidence in note["evidence"].as_array().into_iter().flatten() {
        h += &format!("<li>{}</li>", source_ref(evidence, dest, r));
    }
    for repro in note["repros"].as_array().into_iter().flatten() {
        if repro["source"].is_object() {
            h += &format!("<li>{}</li>", source_ref(&repro["source"], dest, r));
        }
    }
    h += &format!(
        "</ul><p>{}</p></details></article>",
        a(dest, "api/notes.json", "Machine-readable note records")
    );
    h
}

fn note_pages(out: &Path, input: &Inputs, r: &mut Renderer) -> Result<(), String> {
    let dest = "notes/index.html";
    let mut h=String::from("<p class=\"eyebrow\">Function notes</p><h1>Where Excel's answer is wrong</h1><p>Recorded calls with the mathematical value beside Excel's answer. Each note states the capture build and machine, the input route, and the evidence; missing capture information stays explicit.</p>");
    h+=&format!("<p>{} notes, attached to {} distinct entries in the 541-entry catalogue. {RECONCILIATION}.</p>",input.notes.len(),input.notes.iter().flat_map(|n|n["functions"].as_array().unwrap().iter().map(s)).collect::<BTreeSet<_>>().len());
    h+="<div class=\"table-scroll\" role=\"region\" aria-label=\"Excel and mathematical answers\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">Note / call</th><th scope=\"col\">Excel</th><th scope=\"col\">Correct value</th></tr></thead><tbody>";
    for n in input
        .notes
        .iter()
        .filter(|n| n["kind"] == "excel-differs-from-math")
    {
        for repro in n["repros"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|x| x["correct"].is_object())
        {
            h += &format!(
                "<tr><td>{}<br><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                a(
                    dest,
                    &format!("notes/{}.html", s(&n["note_id"])),
                    s(&n["note_id"])
                ),
                esc(s(&repro["call"])),
                cell_value(&repro["excel"]),
                cell_value(&repro["correct"])
            );
        }
    }
    h+="</tbody></table></div><h2>All function notes</h2><p>Measured means read from recorded answers over the note's stated scope. Inferred marks an explanation that goes beyond those readings. Documentation differences show both accounts.</p>";
    for (kind, label) in [
        (
            "excel-differs-from-math",
            "Excel differs from the mathematical answer",
        ),
        (
            "documentation-differs",
            "Documentation differs from behaviour",
        ),
        ("quirk", "Quirks"),
        ("bit-matching-detail", "Details for matching Excel's bits"),
    ] {
        h += &format!("<section><h2>{label}</h2><ul class=\"entries\">");
        for n in input.notes.iter().filter(|n| n["kind"] == kind) {
            let id = s(&n["note_id"]);
            h += &format!(
                "<li>{}<span class=\"small\">{} · {}</span></li>",
                a(
                    dest,
                    &format!("notes/{id}.html"),
                    &format!("{id}: {}", s(&n["title"]))
                ),
                esc(s(&n["status"])),
                n["functions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|f| a(dest, &function_path(s(f)), s(f)))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        h += "</ul></section>";
    }
    h += &format!(
        "<p>{} · {}</p>",
        a(dest, "notes/howto.html", "How to add a note"),
        a(dest, "api/notes.json", "Note JSON")
    );
    output(out, dest, "Where Excel's answer is wrong", &h)?;
    for n in &input.notes {
        let dest = format!("notes/{}.html", s(&n["note_id"]));
        let body = format!(
            "<p class=\"eyebrow\">Function note</p><h1>{}</h1><p>{}</p>{}",
            esc(s(&n["title"])),
            n["functions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|f| a(&dest, &function_path(s(f)), s(f)))
                .collect::<Vec<_>>()
                .join(" · "),
            render_note(n, &dest, r)
        );
        output(out, &dest, s(&n["title"]), &body)?;
    }
    Ok(())
}

fn function_index(out: &Path, input: &Inputs) -> Result<(), String> {
    let dest = "functions/index.html";
    let mut categories = BTreeMap::<String, Vec<&Value>>::new();
    for f in &input.functions {
        categories
            .entry(s(&f["category"]).to_string())
            .or_default()
            .push(f);
    }
    let mut h=format!("<p class=\"eyebrow\">The catalogue</p><h1>Functions and operators</h1><p>Browse all 541 entries by category, or filter by name, description, localized name, or function id. {RECONCILIATION}.</p><div class=\"filter-controls\" hidden><label for=\"function-filter\">Filter entries</label><input id=\"function-filter\" type=\"search\" autocomplete=\"off\" placeholder=\"ROUND, payment, ARRONDI…\"><label for=\"category-filter\">Category</label><select id=\"category-filter\"><option value=\"\">All categories</option>");
    for category in categories.keys() {
        h += &format!("<option>{}</option>", esc(category));
    }
    h+=&format!("</select><p id=\"filter-status\" role=\"status\" aria-live=\"polite\"></p><p class=\"small\">{RECONCILIATION}.</p></div><nav class=\"category-nav\" aria-label=\"Categories\">");
    for category in categories.keys() {
        h += &format!("<a href=\"#{}\">{}</a>", slug(category), esc(category));
    }
    h += "</nav><div id=\"catalogue\">";
    for (category, functions) in &categories {
        h += &format!(
            "<section data-category=\"{}\"><h2 id=\"{}\">{}</h2><ul class=\"entries\">",
            esc(category),
            slug(category),
            esc(category)
        );
        for f in functions {
            let id = s(&f["function_id"]);
            let localized = f["localized_names"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|n| s(&n["name"]))
                .collect::<Vec<_>>()
                .join(" ");
            let query = format!(
                "{} {id} {} {localized}",
                s(&f["surface_name"]),
                s(&f["descriptions"]["short"])
            );
            h += &format!(
                "<li data-search=\"{}\">{}<span>{}</span><span class=\"small\">{} · {}</span></li>",
                esc(&query),
                a(dest, &function_path(id), s(&f["surface_name"])),
                esc(s(&f["descriptions"]["short"])),
                if input.curated.contains(id) {
                    "Curated prose"
                } else {
                    "Projection only"
                },
                esc(s(&f["admission"]["label"]))
            );
        }
        h += "</ul></section>";
    }
    h+="</div><p id=\"no-results\" hidden>No matching entries. Try a function name or clear the filters.</p>";
    output(out, dest, "Functions and operators", &h)
}

fn homepage(
    out: &Path,
    input: &Inputs,
    episodes: &[(String, String, String, bool)],
) -> Result<(), String> {
    let dest = "index.html";
    let mut h=String::from("<div class=\"hero\"><p class=\"eyebrow\">DNA CALC</p><h1>The Handbook of Excel Functions</h1><p class=\"subtitle\">Every catalogued entry, described honestly — with its evidence and limits.</p><p class=\"explain\">This is a reference in progress. Projected metadata, curated explanations and recorded observations are labelled separately. No Handbook test suites or ledger receipts have been published.</p></div><form class=\"searchrow\" action=\"functions/index.html\" method=\"get\"><label class=\"sr-only\" for=\"home-search\">Find a function</label><input id=\"home-search\" name=\"q\" type=\"search\" placeholder=\"ROUND, XLOOKUP, payment…\"><button type=\"submit\">Find a function</button></form><p class=\"browse\"><a href=\"functions/index.html\">Browse all 541 entries by category →</a></p>");
    if let Some(n) = input.notes.iter().find(|n| n["note_id"] == "FN-ROUND-001") {
        let repro = &n["repros"][0];
        h+=&format!("<section class=\"card featured\" aria-labelledby=\"featured-note\"><p class=\"eyebrow\">From the measurement record</p><h2 id=\"featured-note\">{}</h2><div class=\"answer-comparison\"><div><span class=\"small\">Excel's recorded answer</span><strong>{}</strong></div><div><span class=\"small\">Mathematical value</span><strong>{}</strong></div></div><p><code>{}</code></p><p class=\"small\">{}</p><p>{} →</p></section>",esc(s(&n["title"])),cell_value(&repro["excel"]),cell_value(&repro["correct"]),esc(s(&repro["call"])),esc(s(&repro["recorded_in"])),a(dest,&format!("notes/{}.html",s(&n["note_id"])),"Read the inputs, evidence and limits"));
    }
    h += "<div class=\"grid exhibits\">";
    for (id, title) in [
        ("FN-SQRT-001", "When SQRT rounds twice"),
        ("FN-PMT-001", "A payment formula's arithmetic"),
        ("FN-TBILLEQ-001", "The documented formula differs"),
        ("FN-WEEKNUM-001", "One build, conflicting answers"),
    ] {
        if let Some(note) = input.notes.iter().find(|n| n["note_id"] == id) {
            h += &format!(
                "<article class=\"card\"><h2>{}</h2><p class=\"small\">{}</p></article>",
                a(dest, &format!("notes/{id}.html"), title),
                esc(s(&note["title"]))
            );
        }
    }
    h+="</div><section class=\"card audience\"><h2>For engine builders, auditors, and the numerically curious</h2><p>Start with a function's call signature, read the recorded surprises, or follow the research behind an algorithm.</p><p><a href=\"notes/index.html\">Where Excel's answer is wrong</a> · <a href=\"lastbit/index.html\">The Last Bit</a></p>";
    if let Some((_, dest, title, ready)) = episodes.first() {
        h += &format!(
            "<p>{} <span class=\"small\">{}</span></p>",
            a("index.html", dest, title),
            if *ready { "Series article" } else { "Draft" }
        );
    }
    h+=&format!("</section><p class=\"small coverage-line\">{} of 541 entries have curated prose files; {} have projection-only pages. {RECONCILIATION}. <a href=\"coverage/index.html\">Read the coverage and limits →</a></p>",input.curated.len(),541-input.curated.len());
    output(out, dest, "The Handbook of Excel Functions", &h)
}

fn coverage(
    out: &Path,
    input: &Inputs,
    withheld: &[Value],
    unresolved: &BTreeSet<String>,
) -> Result<(), String> {
    let deferred = input
        .functions
        .iter()
        .filter(|f| f["admission"]["label"] == "deferred")
        .count();
    let placeholder = input
        .functions
        .iter()
        .filter(|f| f["signature_placeholder"] == true)
        .count();
    let mut h=format!("<p class=\"eyebrow\">What exists in this build</p><h1>Coverage and limits</h1><p>{RECONCILIATION}. Every count below uses this full 541-entry catalogue. The published-row count is not a coverage denominator.</p><dl class=\"counts\"><dt>Entry pages generated</dt><dd>541 of 541</dd><dt>Curated prose files present</dt><dd>{} of 541</dd><dt>Projection-only pages</dt><dd>{} of 541</dd><dt>Deferred reference-engine entries</dt><dd>{deferred} of 541</dd><dt>Placeholder signatures</dt><dd>{placeholder} of 541</dd><dt>Entries with Handbook vector suites</dt><dd>0 of 541</dd><dt>Entries with admitted Handbook implementations</dt><dd>0 of 541</dd></dl><p>Curation counts file presence; it does not grade accuracy or depth. Notes and projected metadata can exist on a projection-only page. Deferred entries stay in the catalogue with their reasons.</p>",input.curated.len(),541-input.curated.len());
    h+=&format!("<h2>Notes</h2><p>{} note records, rendered from <code>content/notes/FN-*.json</code>. {} The generated output includes its own <a href=\"../api/notes.json\">notes API</a> derived from those same records. <a href=\"../notes/index.html\">Read the notes</a>.</p>",input.notes.len(),if input.note_api_matches {"The checked-in notes API agrees with these inputs."}else{"The checked-in notes API differs from source JSON; this site uses the source records to avoid dropping later notes."});
    h+="<h2>Catalogue boundary</h2><p>The current upstream inventory omits four Microsoft-listed functions: FORECAST.ETS, FORECAST.ETS.CONFINT, FORECAST.ETS.SEASONALITY and FORECAST.ETS.STAT. They are outside this 541-entry spine. This omission is an upstream inventory gap, not evidence that the functions are absent from Excel.</p>";
    h += &format!(
        "<p><a href=\"{}\">Function-help completeness audit, 24 August 2026</a></p>",
        source_link(
            &input.revision,
            "docs/audits/FUNCTION_HELP_COMPLETENESS_20260824.md"
        )
    );
    h+="<h2>Evidence and provenance</h2><p>This is a pre-ledger presentation of committed projections, curated prose and observation notes. The repository has no ledger database or canonical claim export. No ledger claim counts, freshness checks or receipts are asserted here. The downloadable rubric is an earlier derivation snapshot; its counters are not this build's curation counters.</p>";
    h += &provenance(input, &Value::Null);
    h+=&format!("<p>Source revision: <code>{}</code>, working tree {}. <a href=\"../api/build.json\">Build inventory and editorial findings</a> · <a href=\"../api/rubric.json\">Earlier rubric derivation JSON</a>.</p>",esc(&input.revision),esc(&input.tree_state));
    if input.tree_state == "dirty" {
        h+="<p class=\"notice\">This build includes local changes. Its source revision alone cannot reproduce the folder; review and commit those changes before selecting an artifact for publication.</p>";
    }
    h+=&format!("<h2>Editorial limits</h2><p>{} source passages are visibly withheld because their wording fails the rendered claim-language gate. The sources remain unchanged. The build inventory records their source paths and excerpts for review. Automated checks detect forbidden phrases, common unscoped status wording and missing count reconciliation; editorial review is still required to assess the meaning and evidence of every claim.</p>",withheld.len());
    if !unresolved.is_empty() {
        h+=&format!("<p>{} source links do not resolve to a known repository file. They link to the citing source page for inspection. These references require editorial review; no local dead link is substituted.</p>",unresolved.len());
    }
    h+="<h2>Drafts and unpublished material</h2><p>Series essays without an explicit ready status are labelled Draft. Internal MATERIAL_ files and the unreviewed series PDF are excluded. There are no Atlas plates in this build; the homepage exhibit uses recorded note data.</p><h2>How this folder was checked</h2><p>Generation runs the local link and claim-language checks before replacing the output folder. Browser checks are a separate engineering step; this page does not claim a browser result for an unchecked future build.</p>";
    output(out, "coverage/index.html", "Coverage and limits", &h)
}

const RULES: &str = r#"<h1>How to read the claims</h1><p>This site follows Charter §7. Compact labels state their own limits; expanding a label cannot turn an overclaim into a true statement.</p><h2>Reading lenses</h2><p>Working keeps boundary explanations compact. Explicit expands them in place. Both show the same source record, measured answers, status and limitations. With JavaScript disabled, all explanations are visible.</p><h2>Scope travels with evidence</h2><p>Recorded agreement concerns the named rows, Excel build and machine. It does not establish behaviour on unmeasured inputs. Handbook equality badges require a published, versioned suite as well as a build and platform. No such suite is present here.</p><h2>Independent statuses</h2><p>Curated prose present means a function Markdown file is in the checkout. Projection only means that no such prose file is present. OxFunc admission is a separate projected reference-engine status. None of these labels awards a Handbook test-suite warrant.</p><h2>Seven doors</h2><p>Each function provides Why, Sources, As of, Rules, History, Limits and Replay. In this pre-ledger build those doors identify sources and missing evidence. No receipt, context hash or ledger answer is invented.</p><h2>Sources and disagreement</h2><p>Function notes give recorded Excel answers and their capture provenance. When documentation differs, both the documentation and the measurement are shown. Inference and missing build information remain explicit. Mathematical reference values explain the intended operation; they are separate from measured Excel behaviour.</p><h2>Implementations</h2><p>An implementation can be listed only after admission against its function's versioned Handbook suite. No implementations are listed in this build.</p>"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_replacement_preserves_unowned_files_and_rolls_back_on_install_failure() {
        let scratch =
            std::env::temp_dir().join(format!("efh-site-install-test-{}", std::process::id()));
        std::fs::create_dir(&scratch).unwrap();
        let out = scratch.join("output");
        let stage = scratch.join("stage");
        write(&out, "valuable.txt", "user content").unwrap();
        write(&stage, MARKER, "generated").unwrap();
        write(&stage, "index.html", "new generation").unwrap();
        assert!(install_output(&stage, &out).is_err());
        assert_eq!(read(&out.join("valuable.txt")).unwrap(), "user content");
        write(&out, MARKER, "generated").unwrap();
        assert!(install_output(&scratch.join("missing-stage"), &out).is_err());
        assert_eq!(read(&out.join("valuable.txt")).unwrap(), "user content");
        install_output(&stage, &out).unwrap();
        assert!(!out.join("valuable.txt").exists());
        assert_eq!(read(&out.join("index.html")).unwrap(), "new generation");
        std::fs::remove_dir_all(scratch).unwrap();
    }
    #[test]
    fn real_corpus_reconciles_and_two_builds_have_identical_bytes() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        let input = Inputs::load(&repo).unwrap();
        let scratch = std::env::temp_dir().join(format!(
            "efh-site-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&scratch).unwrap();
        let first = scratch.join("first");
        let second = scratch.join("second");
        generate(&repo, &first, &input).unwrap();
        generate(&repo, &second, &input).unwrap();
        let inventory = files(&first).unwrap();
        for f in &input.functions {
            let html = read(&first.join(function_path(s(&f["function_id"])))).unwrap();
            assert!(html.contains("Source limits"));
            assert!(html.contains(RECONCILIATION));
            assert_eq!(html.matches("<h1").count(), 1);
        }
        assert_eq!(inventory.len(), files(&second).unwrap().len());
        for path in inventory {
            assert!(!path.to_string_lossy().contains("MATERIAL_"));
            let relative = path.strip_prefix(&first).unwrap();
            assert_eq!(
                std::fs::read(&path).unwrap(),
                std::fs::read(second.join(relative)).unwrap(),
                "{}",
                relative.display()
            );
        }
        let round = read(&first.join("functions/ROUND.html")).unwrap();
        assert!(round.contains("FN-ROUND-001"));
        assert!(round.contains("0xc2d9945ca2620000"));
        let notes = read(&first.join("notes/index.html")).unwrap();
        assert!(notes.contains("<h1>Where Excel's answer is wrong</h1>"));
        let series = read(&first.join("lastbit/index.html")).unwrap();
        assert!(series.contains("Draft"));
        checks::check(&first, true, true).unwrap();
        std::fs::remove_dir_all(scratch).unwrap();
    }
}
