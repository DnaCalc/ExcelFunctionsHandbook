use crate::{
    checks, implementation, implementation::CalcFunc, markdown::Renderer, reference, util::*,
};
use regex::Regex;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

const CSS: &str = include_str!("style.css");
const JS: &str = include_str!("site.js");
const MARKER: &str = ".efh-site-output";
/// The catalogue lives outside `functions/`: `functions/index.html` and the INDEX function's
/// `functions/INDEX.html` are one file on a case-insensitive file system.
pub(crate) const CATALOGUE: &str = "catalogue/index.html";
pub(crate) const A_TO_Z: &str = "catalogue/a-z.html";

fn page(path: &str, title: &str, body: &str, home: bool) -> String {
    let link = |to: &str| esc(&relative(path, to));
    let discovery = path
        .strip_prefix("functions/")
        .and_then(|name| name.strip_suffix(".html"))
        .filter(|name| *name != "index")
        .map(|name| {
            format!(
                "<link rel=\"alternate\" type=\"application/json\" href=\"{}\">",
                link(&format!("api/functions/FUNC.{name}.json"))
            )
        })
        .unwrap_or_default();
    format!(
        r##"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="light dark"><meta name="referrer" content="no-referrer"><title>{title} · The Handbook of Excel Functions</title><link rel="stylesheet" href="{css}"><script src="{js}" defer></script>{discovery}</head>
<body><a class="skip" href="#main">Skip to content</a><div class="wrap"><header><nav class="mainnav" aria-label="Main"><a class="brand" href="{root}">DNA CALC <span>Handbook</span></a><a href="{index}">Functions</a><a href="{a_to_z}">A to Z</a><a href="{notes}">Notes</a><a href="{lastbit}">The Last Bit</a><a href="{implementation}">DNA Calc vs Excel</a><a href="{coverage}">Coverage</a></nav><form class="site-search" role="search" action="{index}" method="get"><label class="sr-only" for="site-search">Search functions</label><input id="site-search" name="q" type="search" autocomplete="off" spellcheck="false" placeholder="Search functions, press /" data-instant-search><div class="search-results" id="site-search-results" hidden></div></form>{picker}<div class="lensbar" role="group" aria-label="Reading lens" hidden><span>Reading</span><button type="button" data-lens="working" aria-pressed="true">Working</button><button type="button" data-lens="explicit" aria-pressed="false">Explicit</button></div></header>
<main id="main" tabindex="-1"{class}>{body}</main><footer><p>{reconciliation}.</p><p><a href="{licence}">Content: CC BY 4.0 · Code: MIT</a> · <a href="{rules}">Measurement limits</a> · <a href="{api}">Static JSON index</a> · No tracking.</p><p class="small">The Handbook of Excel Functions · DNA Kode. Independent of Microsoft.</p></footer></div></body></html>"##,
        title = esc(title),
        css = link("assets/style.css"),
        js = link("assets/site.js"),
        root = link("index.html"),
        index = link(CATALOGUE),
        a_to_z = link(A_TO_Z),
        notes = link("notes/index.html"),
        lastbit = link("lastbit/index.html"),
        implementation = link("implementation/index.html"),
        picker = implementation::language_picker(),
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
    functions: Vec<Value>,
    notes: Vec<Value>,
    curated: BTreeSet<String>,
    revision: String,
    tree_state: String,
    note_api_matches: bool,
    calc: CalcFunc,
}
impl Inputs {
    fn load(repo: &Path) -> Result<Self, String> {
        let index = json(&repo.join("data/index.json"))?;
        let rows = index["rows"]
            .as_array()
            .ok_or("data/index.json: rows is not an array")?;
        if rows.len() != ENTRIES
            || index["counts"]["functions"] != FUNCTIONS
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
            != FUNCTIONS
            || functions
                .iter()
                .filter(|f| f["entry_kind"] == "built_in_operator")
                .count()
                != 23
            || published_rows.len() != PUBLISHED_ROWS_AND_ADDED
            || published_rows.contains("")
        {
            return Err(format!(
                "entry projections do not reconcile: {RECONCILIATION}"
            ));
        }
        // A signature is a real signature or the build stops; an unknown one is not a page state.
        let placeholder = Regex::new(r"\(\.\.\.\)?$").unwrap();
        for f in &functions {
            let display = s(&f["signature"]["display"]);
            if f["signature_placeholder"] != false
                || display.is_empty()
                || placeholder.is_match(display)
            {
                return Err(format!(
                    "{}: placeholder signature {display:?}; run tools/efh-calcfunc/regenerate.py",
                    s(&f["function_id"])
                ));
            }
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
            functions,
            notes,
            curated,
            revision,
            tree_state,
            note_api_matches,
            calc: CalcFunc::load(repo)?,
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
        "Wrote {} files, {bytes} bytes to {} ({ENTRIES} entry pages)",
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
    write(out, "assets/eval-worker.js", include_str!("eval-worker.js"))?;
    let engine_dir = repo.join("site/engine");
    if engine_dir.join("engine_bg.wasm").is_file() {
        for path in files(&engine_dir)? {
            let target = out
                .join("assets/onecalc")
                .join(path.strip_prefix(&engine_dir).unwrap());
            std::fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
            std::fs::copy(&path, target).map_err(|e| e.to_string())?;
        }
    }
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
            &reader_words(
                &serde_json::to_string_pretty(&reference::record(
                    repo,
                    f,
                    &reference::load(repo, f)?,
                    &input.notes,
                    &input.calc,
                )?)
                .unwrap(),
            ),
        )?;
    }
    routes.insert("content/notes/NOTES.md".into(), "notes/index.html".into());
    routes.insert("site/api/notes.json".into(), "api/notes.json".into());
    routes.insert("data/index.json".into(), "api/index.json".into());
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
    let series = json(&repo.join("content/lastbit/series.json"))?;
    let mut episodes = Vec::new();
    for entry in series["entries"].as_array().ok_or("series.json: entries")? {
        let (src, slug) = (s(&entry["source"]).to_string(), s(&entry["slug"]));
        let dest = format!("lastbit/{slug}.html");
        routes.insert(src.clone(), dest.clone());
        let label = match s(&entry["kind"]) {
            "article" => "Article",
            "draft" => "Draft article",
            "notes" => "Working notes",
            other => return Err(format!("series.json: unknown kind {other}")),
        };
        episodes.push((src, dest, s(&entry["title"]).to_string(), label.to_string()));
    }
    for path in files(&repo.join("content/lastbit/figures"))? {
        let target = out.join("lastbit/figures").join(path.file_name().unwrap());
        std::fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
        std::fs::copy(&path, target).map_err(|e| e.to_string())?;
    }
    std::fs::copy(
        repo.join("content/lastbit/the-last-bit-2026-07.pdf"),
        out.join("lastbit/the-last-bit-2026-07.pdf"),
    )
    .map_err(|e| e.to_string())?;
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
    write(out, "llms.txt", "# The Handbook of Excel Functions\n\nCanonical open reference for Excel functions and operators.\n\n- Catalogue: https://handbook.dnacalc.com/api/index.json\n- Function page: https://handbook.dnacalc.com/functions/ROUND.html\n- Stable function JSON: https://handbook.dnacalc.com/api/functions/FUNC.ROUND.json\n- Schema: https://handbook.dnacalc.com/api/function-reference.schema.json\n- Evaluation: POST https://handbook.dnacalc.com/api/eval with {\"formula\":\"=ROUND(1.005,2)\"}\n- Contributions: POST https://handbook.dnacalc.com/api/contributions\n\nReplace ROUND with the case-sensitive function name, including dots; use operator IDs such as OP_ADD. All catalogue IDs resolve. JSON contains section_order, sections (Markdown), try_it, versions (Excel build and platform), implementations (language and suite), variants (difference, error_bound, onecalc_udf, and for recorded Excel environments excel_build_range, cpu, state, code and evidence), implementation_state (what we conclude about our own Rust, with counts) and sources. Pinned source snapshots use flavour excel-compatibility-candidate and suite null until admitted against a published Handbook suite. Empty lists mean no records, not proof of identical behaviour. OneCalc results are not Excel measurements. API errors and engine refusals must be preserved.\n")?;
    write(
        out,
        "api/function-reference.schema.json",
        &read(&repo.join("tools/schemas/function-reference.schema.json"))?,
    )?;
    output(
        out,
        "contribute/index.html",
        "Contribute to the Handbook",
        r#"<h1>Contribute to the Handbook</h1><p>Share a correction, Excel observation, implementation or variant. Submissions are stored privately by DNA Kode for review. Your contact details will not be published.</p><form id="contribution-form"><label for="contribution-function">Function</label><input id="contribution-function" name="function_id" placeholder="FUNC.ROUND" required maxlength="100"><label for="contribution-kind">Contribution</label><select id="contribution-kind" name="kind"><option value="correction">Correction</option><option value="observation">Excel observation</option><option value="implementation">Implementation</option><option value="variant">Variant</option></select><label for="contribution-build">Excel build and platform (if relevant)</label><input id="contribution-build" name="excel_build" maxlength="200"><label for="contribution-details">Details, formula, observed answer and sources</label><textarea id="contribution-details" name="details" required maxlength="20000" rows="8"></textarea><label for="contribution-contact">Contact (optional)</label><input id="contribution-contact" name="contact" maxlength="320"><button type="submit" disabled>Send contribution</button><output id="contribution-status" role="status" aria-live="polite"></output><noscript><p>Enable JavaScript to submit this form.</p></noscript></form>"#,
    )?;
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
    let figure = Regex::new(r"<p>\[\[figure ([a-z0-9-]+\.png)\|([^\]]+)\]\]</p>").unwrap();
    let entries = series["entries"].as_array().unwrap();
    for (at, (src, dest, title, label)) in episodes.iter().enumerate() {
        let rendered = r.render(src, dest, &read(&repo.join(src))?, false);
        let rendered = figure.replace_all(
            &rendered,
            "<figure class=\"series-figure\"><img src=\"figures/$1\" alt=\"$2\" loading=\"lazy\"></figure>",
        );
        let entry = &entries[at];
        let mut body = format!(
            "<p class=\"eyebrow\">The Last Bit · part {} of {}</p><h1>{}</h1><p class=\"status\">{label} · {}</p>",
            at + 1,
            episodes.len(),
            esc(title),
            esc(s(&entry["date"]))
        );
        if let Some(pdf) = entry["pdf"].as_str() {
            body += &format!("<p><a href=\"{}\">Read the original PDF</a></p>", esc(pdf));
        }
        if let Some(note) = entry["note"].as_str() {
            body += &format!("<aside class=\"notice\"><p>{}</p></aside>", esc(note));
        }
        body += &format!(
            "<article>{rendered}</article><nav class=\"series-nav\" aria-label=\"The Last Bit\">"
        );
        if at > 0 {
            body += &format!(
                "<a href=\"{}\">← {}</a>",
                esc(&relative(dest, &episodes[at - 1].1)),
                esc(&episodes[at - 1].2)
            );
        }
        body += &format!(
            "<a href=\"{}\">The series</a>",
            esc(&relative(dest, "lastbit/index.html"))
        );
        if at + 1 < episodes.len() {
            body += &format!(
                "<a href=\"{}\">{} →</a>",
                esc(&relative(dest, &episodes[at + 1].1)),
                esc(&episodes[at + 1].2)
            );
        }
        body += "</nav>";
        output(out, dest, title, &body)?;
    }
    for f in &input.functions {
        function_page(repo, out, input, f, &mut r)?;
    }
    note_pages(out, input, &mut r)?;
    function_index(repo, out, input)?;
    search_index(repo, out, input)?;
    implementation::code_pages(&input.calc, |path, title, body| {
        output(out, path, title, body)
    })?;
    output(
        out,
        "implementation/index.html",
        "How DNA Calc compares with Excel",
        &implementation::summary_page(&input.calc, &input.functions, function_path),
    )?;
    homepage(out, input, &episodes)?;
    let mut page=String::from("<p class=\"eyebrow\">Research series</p><h1>The Last Bit</h1><p>How we worked out what Excel computes, down to the last bit, in reading order. Articles are written to be read. Working notes were kept during the work and are marked as such; later notes correct earlier ones.</p><p>Measurement is one part of this project's method. The work also uses published algorithms, instruction manuals and licensed translations. Every figure applies to its recorded inputs, Excel builds and platforms.</p><ol class=\"series-list\">");
    for (at, (_, dest, title, label)) in episodes.iter().enumerate() {
        let entry = &entries[at];
        page += &format!(
            "<li>{}<span class=\"small\">{label} · {}</span><span>{}</span>{}</li>",
            a("lastbit/index.html", dest, title),
            esc(s(&entry["date"])),
            esc(s(&entry["summary"])),
            entry["pdf"].as_str().map_or(String::new(), |pdf| format!(
                " <a class=\"small\" href=\"{}\">PDF</a>",
                esc(pdf)
            ))
        );
    }
    page += "</ol><h2>Research log</h2><p>The rounds after these notes are kept as a terse working log in the public repository <a href=\"https://github.com/DnaCalc/DnaCalc-Research\">DnaCalc/DnaCalc-Research</a>, with their evidence.</p>";
    output(out, "lastbit/index.html", "The Last Bit", &page)?;
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
    let rows: Vec<_> = input.functions.iter().map(|f| {
        let id=s(&f["function_id"]);
        json!({"function_id":id,"surface_name":f["surface_name"],"entry_kind":f["entry_kind"],"category":f["category"],"implementation_state":input.calc.entry(id)["state"],"url":format!("https://handbook.dnacalc.com/{}",function_path(id)),"json_url":format!("https://handbook.dnacalc.com/api/functions/{id}.json")})
    }).collect();
    write(out,"api/index.json",&serde_json::to_string_pretty(&json!({"schema":"efh.function-index/v1","counts":{"entries":ENTRIES,"functions":FUNCTIONS,"operators":23,"published_rows":534,"split_byte_variant_rows":7,"added_functions":13},"reconciliation":RECONCILIATION,"rows":rows})).unwrap())?;
    write(out,"api/notes.json",&reader_words(&serde_json::to_string_pretty(&json!({"schema":"efh.function-notes-api/v1","source":"content/notes/FN-*.json (efh.function-note/v1)","note_count":input.notes.len(),"notes":input.notes})).unwrap()))?;
    write(out,"api/build.json",&serde_json::to_string_pretty(&json!({"schema":"efh.static-build/v1","source_revision":input.revision,"source_tree_state":input.tree_state,"entry_pages":ENTRIES,"curated_prose_files":input.curated.len(),"notes":input.notes.len(),"input_notes_api_matches_sources":input.note_api_matches,"withheld_passages":withheld,"unresolved_source_references":unresolved,"reconciliation":RECONCILIATION})).unwrap())?;
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
    let dest = function_path(id);
    let owned = reference::load(repo, f)?;
    let record = reference::record(repo, f, &owned, &input.notes, &input.calc)?;
    let name = s(&record["display_name"]);
    let category = match s(&f["category"]) {
        "Our 10 featured functions" => "Featured in Microsoft's function list",
        other => other,
    };
    let mut html = format!("<p class=\"eyebrow\">{}</p><h1 class=\"function-name\">{}</h1><p class=\"local-name\" id=\"local-name\" hidden></p>{}<nav class=\"section-nav\" aria-label=\"On this page\">",esc(category),esc(name),implementation::status_line(&input.calc, id));
    for (key, title) in reference::SECTIONS {
        html += &format!("<a href=\"#{key}\">{title}</a>");
    }
    html += "</nav>";
    let src = format!("content/functions/{id}.md");
    for (key, title) in reference::SECTIONS {
        html += &format!("<section class=\"function-section\" aria-labelledby=\"{key}\"><h2 id=\"{key}\">{title}</h2>");
        if key == "what-it-does" {
            if let Some(signature) = record["signature"]["display"].as_str() {
                html += &format!("<p class=\"signature\"><code>{}</code></p>", esc(signature));
            }
        }
        if key == "version-differences" {
            html += "<details class=\"versions\"><summary>Show the Excel builds and answers we recorded</summary>";
        }
        let deferred = input.calc.entry(id)["state"] == "deferred";
        if key == "try-it" && deferred {
            html += &format!(
                "<p class=\"notice\">DNA Calc does not run this function yet: its answer needs an outside service. In Excel you would write:</p><pre><code>{}</code></pre>",
                esc(s(&record["try_it"]["formula"]))
            );
        } else if key == "try-it" {
            html += &format!(
                r#"<form class="try-it" data-evaluator><label for="formula">Formula</label><textarea id="formula" name="formula" rows="2" maxlength="4096" spellcheck="false">{}</textarea><details><summary>Input cells (optional)</summary><label for="eval-inputs">Cell addresses and values as JSON</label><textarea id="eval-inputs" name="inputs" rows="2" maxlength="20000" spellcheck="false">{}</textarea><p class="small">Example: {{&quot;B2&quot;: 3, &quot;B3&quot;: 4}}. A1 holds the formula. Other cells need values before a formula can read them.</p></details><button type="submit" disabled>Calculate</button><button type="button" data-reset disabled>Reset example</button><output id="eval-result" role="status" aria-live="polite">Enter a formula, then calculate.</output><p class="small">Runs in your browser on DNA Calc's formula engine, not on Excel, so its answer can differ from Excel's. Use English function names and commas between arguments.</p><noscript><p>Enable JavaScript to calculate. The reference below is readable without it.</p></noscript></form>"#,
                esc(s(&record["try_it"]["formula"])),
                esc(&record["try_it"]
                    .get("inputs")
                    .unwrap_or(&json!({}))
                    .to_string())
            );
        } else {
            html += &r.render(&src, &dest, s(&record["sections"][key]), false);
        }
        if key == "quirks" {
            for note in record["notes"].as_array().into_iter().flatten() {
                html += &reader_note(note, &dest);
            }
        }
        if key == "try-it" {
            if let Some(note) = record["try_it"]["note"].as_str().filter(|_| !deferred) {
                html += &format!("<p class=\"small\">{}</p>", esc(note));
            }
            if let Some(prose) = record["sections"][key].as_str() {
                html += &r.render(&src, &dest, prose, false);
            }
        }
        if key == "version-differences" {
            for version in record["versions"].as_array().into_iter().flatten() {
                let platform = version["platform"]
                    .as_str()
                    .unwrap_or("platform not recorded");
                html += &format!("<article class=\"version-record\"><h3>Excel {}</h3><p class=\"small\">{}</p><p>{}</p>",esc(version["excel_build"].as_str().unwrap_or("build not recorded")),esc(platform),esc(s(&version["behaviour"])));
                if let Some(observations) = version["observations"].as_array() {
                    html += "<details><summary>Observed formulas and results</summary><ul>";
                    for observation in observations {
                        html += &format!(
                            "<li><code>{}</code> → {} <span class=\"small\">({})</span></li>",
                            esc(s(&observation["call"])),
                            cell_value(&observation["excel"]),
                            esc(s(&observation["input_route"]))
                        );
                    }
                    html += "</ul></details>";
                }
                html += "</article>";
            }
        }
        if key == "how-to-implement" {
            let suites: Vec<_> = record["implementations"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|i| i["suite"].is_string())
                .collect();
            for implementation in &suites {
                html += &format!(
                    "<p>{}: <code>{}</code> · suite {}</p>",
                    esc(s(&implementation["language"])),
                    esc(s(&implementation["source"])),
                    esc(s(&implementation["suite"]))
                );
            }
            html += &implementation::plain_section(&input.calc, id, &dest);
            html += &implementation::code_section(&input.calc, id, &dest);
            if suites.is_empty() {
                html += "<p class=\"small\">No implementation has been published with a Handbook test suite for this function.</p>";
            }
        }
        if key == "version-differences" {
            html += &environment_variants(&record, &dest, r);
            html += "</details>";
        }
        if key == "alternatives" {
            html += "<h3>Named variants</h3>";
            let variants: Vec<_> = record["variants"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["kind"] != "environment")
                .collect();
            if variants.is_empty() {
                html += "<p class=\"small\">No named variants are recorded yet.</p>";
            }
            for variant in variants {
                html += &format!(
                    "<article><h4>{}</h4><p>{}</p>",
                    esc(s(&variant["name"])),
                    esc(s(&variant["difference"]))
                );
                let bound = &variant["error_bound"];
                if bound.is_null() {
                    html += "<p>Error bound: not recorded.</p>";
                } else {
                    html += &format!(
                        "<p>Error bound: {} {} over {}. {}</p>",
                        esc(&bound["bound"].to_string()),
                        esc(s(&bound["metric"])),
                        esc(s(&bound["domain"])),
                        esc(s(&bound["basis"]))
                    );
                }
                let hook = &variant["onecalc_udf"];
                if hook.is_null() {
                    html += "<p>No executable OneCalc UDF hook is published.</p>";
                } else {
                    html += &format!(
                        "<p>OneCalc UDF: <code>{}</code> ({}) · <code>{}</code></p>",
                        esc(s(&hook["name"])),
                        esc(s(&hook["language"])),
                        esc(s(&hook["source"]))
                    );
                }
                html += "</article>";
            }
        }
        html += "</section>";
    }
    html += &implementation::local_names(f);
    html += &format!("<aside class=\"source-footer\" id=\"sources\"><details><summary>Sources and evidence</summary><p>{} · {}</p>",a(&dest,&format!("api/functions/{id}.json"),"Function JSON"),a(&dest,"rules/index.html","Measurement and suite limits"));
    if let Some(url) = f["docs"]["microsoft_url"].as_str() {
        html += &format!(
            "<p><a href=\"{}\">Microsoft documentation</a></p>",
            esc(url)
        );
    }
    if let Some(sources) = record["sections"]["sources"].as_str() {
        html += &r.render(&src, &dest, sources, false);
    }
    for note in record["notes"].as_array().into_iter().flatten() {
        html += &format!(
            "<p>{}</p>",
            a(
                &dest,
                &format!("notes/{}.html", s(&note["note_id"])),
                s(&note["title"])
            )
        );
    }
    let mut capture_sources = BTreeSet::new();
    for version in record["versions"].as_array().into_iter().flatten() {
        for source in version["sources"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            if capture_sources.insert(source) {
                html += &format!(
                    "<p class=\"small\"><a href=\"{}\">Excel {}: observation source</a></p>",
                    esc(source),
                    esc(version["excel_build"]
                        .as_str()
                        .unwrap_or("build not recorded"))
                );
            }
        }
    }
    html += "<p class=\"small\">Recorded Excel answers apply to their named builds, platforms and input routes. No Handbook test suite is published for this entry. Microsoft source text is excluded from the content licence.</p></details></aside>";
    html += &format!(
        "<p class=\"small\">{} · {}</p>",
        a(
            &dest,
            "contribute/index.html",
            "Suggest a correction or contribute an implementation"
        ),
        a(&dest, "llms.txt", "For agents")
    );
    output(out, &dest, name, &html)
}

/// One record per Excel environment with its own recorded answers: build range, CPU, state of
/// our Rust for that environment, code and evidence.
fn environment_variants(record: &Value, dest: &str, r: &mut Renderer) -> String {
    let variants: Vec<_> = record["variants"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|v| v["kind"] == "environment")
        .collect();
    if variants.is_empty() {
        return String::new();
    }
    let mut h = "<h3 id=\"recorded-variants\">Recorded variants</h3><div class=\"table-scroll\" role=\"region\" aria-label=\"Recorded variants\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">Excel build</th><th scope=\"col\">CPU</th><th scope=\"col\">Recorded behaviour</th><th scope=\"col\">Our Rust</th><th scope=\"col\">Evidence</th></tr></thead><tbody>".to_string();
    for v in variants {
        let key = &v["key"];
        let evidence = v["evidence"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|e| source_ref(e, dest, r))
            .collect::<Vec<_>>()
            .join("<br>");
        let code = v["code"]
            .as_str()
            .map(|u| format!(" <a href=\"{}\">Code</a>", esc(u)))
            .unwrap_or_default();
        h += &format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}{code}</td><td class=\"small\">{evidence}</td></tr>",
            esc(s(&key["excel_build_range"])),
            esc(s(&key["cpu"])),
            esc(s(&v["difference"])),
            esc(s(&v["state_summary"]))
        );
    }
    h + "</tbody></table></div>"
}

fn reader_note(note: &Value, dest: &str) -> String {
    let id = s(&note["note_id"]);
    let mut h = format!(
        "<article class=\"note\" id=\"{}\"><h3>{}</h3><p>{}</p>",
        esc(id),
        esc(s(&note["title"])),
        esc(s(&note["statement"]))
    );
    if let Some(inferred) = note["inferred"].as_str() {
        h += &format!(
            "<p><strong>Explanation (inferred):</strong> {}</p>",
            esc(inferred)
        );
    }
    h += "<details class=\"recorded\"><summary>Excel's recorded answers</summary><div class=\"table-scroll\" role=\"region\" aria-label=\"Excel examples\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">Formula</th><th scope=\"col\">Excel</th><th scope=\"col\">Mathematical value</th></tr></thead><tbody>";
    for repro in note["repros"].as_array().into_iter().flatten() {
        let route = if repro["input_route"] == "cell-value2" {
            "Number supplied through a cell"
        } else {
            "Typed formula"
        };
        let mut details =
            format!("<details><summary>Inputs and reference value</summary><p>{route}.</p>");
        for input in repro["inputs"].as_array().into_iter().flatten() {
            details += &format!(
                "<p>{}: {}</p>",
                esc(s(&input["name"])),
                cell_value(&input["value"])
            );
        }
        if let Some(how) = repro["correct"]["how"].as_str() {
            details += &format!("<p>{}</p>", esc(how));
        }
        details += "</details>";
        h += &format!("<tr><td><code>{}</code><br><span class=\"small\">{}</span>{details}</td><td>{}</td><td>{}</td></tr>",esc(s(&repro["call"])),esc(s(&repro["recorded_in"])),cell_value(&repro["excel"]),cell_value(&repro["correct"]));
    }
    h += &format!(
        "</tbody></table></div></details><p class=\"small\">{}</p></article>",
        a(
            dest,
            &format!("notes/{id}.html"),
            "Full evidence for this quirk"
        )
    );
    h
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
    let label = if repo.is_empty() {
        path.to_string()
    } else {
        format!("{repo} · {path}")
    };
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
    let mut h=String::from("<p class=\"eyebrow\">Function notes</p><h1>Where Excel's answer is inaccurate</h1><p>Recorded calls with the mathematical value beside Excel's answer. Each note states the capture build and machine, the input route, and the evidence; missing capture information stays explicit.</p>");
    h+=&format!("<p>{} notes, attached to {} distinct entries in the {ENTRIES}-entry catalogue. {RECONCILIATION}.</p>",input.notes.len(),input.notes.iter().flat_map(|n|n["functions"].as_array().unwrap().iter().map(s)).collect::<BTreeSet<_>>().len());
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
    output(out, dest, "Where Excel's answer is inaccurate", &h)?;
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

fn function_index(repo: &Path, out: &Path, input: &Inputs) -> Result<(), String> {
    let dest = CATALOGUE;
    let mut categories = BTreeMap::<String, Vec<&Value>>::new();
    for f in &input.functions {
        categories
            .entry(s(&f["category"]).to_string())
            .or_default()
            .push(f);
    }
    let mut h=format!("<p class=\"eyebrow\">The catalogue</p><h1>Functions and operators</h1><p>Browse all {ENTRIES} entries by category, or filter by name, description, localized name, function id or the state of DNA Calc's implementation. {RECONCILIATION}.</p><div class=\"filter-controls\" hidden><label for=\"function-filter\">Filter entries</label><input id=\"function-filter\" type=\"search\" autocomplete=\"off\" placeholder=\"ROUND, payment, ARRONDI…\"><label for=\"category-filter\">Category</label><select id=\"category-filter\"><option value=\"\">All categories</option>");
    for category in categories.keys() {
        h += &format!("<option>{}</option>", esc(category));
    }
    h += "</select><label for=\"state-filter\">DNA Calc's implementation</label><select id=\"state-filter\"><option value=\"\">Any state</option>";
    for (state, title, _) in implementation::STATES {
        h += &format!(
            "<option value=\"{state}\">{} ({})</option>",
            esc(title),
            input.calc.state["counts"][state]
        );
    }
    h+="</select><p id=\"filter-status\" role=\"status\" aria-live=\"polite\"></p></div><nav class=\"category-nav\" aria-label=\"Categories\">";
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
            let owned = reference::load(repo, f)?;
            let label = owned["display_name"]
                .as_str()
                .unwrap_or(s(&f["surface_name"]));
            let localized = local_search_names(f).join(" ");
            let query = format!(
                "{} {id} {} {localized} {label}",
                s(&f["surface_name"]),
                s(&f["descriptions"]["short"])
            );
            let state = input.calc.entry(id);
            h += &format!(
                "<li data-search=\"{}\" data-state=\"{}\" data-entry=\"{id}\">{}<span>{}</span><span class=\"small state-{}\"><span class=\"state-dot\" aria-hidden=\"true\"></span>{}</span></li>",
                esc(&query),
                esc(s(&state["state"])),
                a(dest, &function_path(id), label),
                esc(s(&f["descriptions"]["short"])),
                esc(s(&state["state"])),
                esc(s(&state["summary"]))
            );
        }
        h += "</ul></section>";
    }
    h+="</div><p id=\"no-results\" hidden>No matching entries. Try a function name or clear the filters.</p>";
    output(out, dest, "Functions and operators", &h)?;
    a_to_z(repo, out, input)
}

/// Every local name that should find this entry, shared family lists included.
fn local_search_names(f: &Value) -> Vec<&str> {
    let mut names: Vec<&str> = f["localized_names"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|n| implementation::clean_name(s(&n["name"])))
        .collect();
    for list in f["localized_family_names"].as_array().into_iter().flatten() {
        names.extend(list["names"].as_array().into_iter().flatten().map(s));
    }
    names.sort();
    names.dedup();
    names
}

/// The static index behind the header search: one row per entry, with its local names.
fn search_index(repo: &Path, out: &Path, input: &Inputs) -> Result<(), String> {
    let mut rows = Vec::new();
    for f in &input.functions {
        let id = s(&f["function_id"]);
        let owned = reference::load(repo, f)?;
        let label = owned["display_name"]
            .as_str()
            .unwrap_or(s(&f["surface_name"]));
        let mut names = Vec::new();
        for n in f["localized_names"].as_array().into_iter().flatten() {
            if s(&n["name"]) != label {
                let at = implementation::LANGUAGES
                    .iter()
                    .position(|(tag, _)| *tag == s(&n["locale"]))
                    .ok_or_else(|| format!("{id}: unknown locale {}", s(&n["locale"])))?;
                names.push(format!(
                    "{at}={}",
                    implementation::clean_name(s(&n["name"])).replace('|', " ")
                ));
            }
        }
        for name in f["localized_family_names"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|n| n["names"].as_array().into_iter().flatten())
        {
            if !names.iter().any(|n| n.ends_with(&format!("={}", s(name)))) && s(name) != label {
                names.push(format!("={}", s(name).replace('|', " ")));
            }
        }
        rows.push(json!([
            label,
            function_path(id),
            f["category"],
            f["descriptions"]["short"],
            id,
            names.join("|"),
            input.calc.entry(id)["state"]
        ]));
    }
    let locales: Vec<_> = implementation::LANGUAGES
        .iter()
        .map(|(tag, _)| *tag)
        .collect();
    let data = json!({"locales": locales, "rows": rows});
    write(
        out,
        "assets/search-index.js",
        &format!("window.EFH_SEARCH={data};\n"),
    )
}

/// One alphabetical list of every entry, with a letter bar. Operators keep their own section.
fn a_to_z(repo: &Path, out: &Path, input: &Inputs) -> Result<(), String> {
    let dest = A_TO_Z;
    let mut letters = BTreeMap::<String, Vec<(String, &Value)>>::new();
    for f in &input.functions {
        let owned = reference::load(repo, f)?;
        let label = owned["display_name"]
            .as_str()
            .unwrap_or(s(&f["surface_name"]))
            .to_string();
        let first = label.chars().next().unwrap_or('#').to_ascii_uppercase();
        let group = if f["entry_kind"] == "built_in_operator" {
            "Operators".to_string()
        } else if first.is_ascii_alphabetic() {
            first.to_string()
        } else {
            "Other".to_string()
        };
        letters.entry(group).or_default().push((label, f));
    }
    let mut h = format!("<p class=\"eyebrow\">The catalogue</p><h1>Function index, A to Z</h1><p>Every entry in one alphabetical list. {RECONCILIATION}.</p><nav class=\"letter-nav\" aria-label=\"Jump to a letter\">");
    for letter in letters.keys() {
        h += &format!("<a href=\"#{}\">{}</a>", slug(letter), esc(letter));
    }
    h += "</nav>";
    for (letter, entries) in letters.iter_mut() {
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        h += &format!(
            "<section><h2 id=\"{}\">{}</h2><ul class=\"az-list\">",
            slug(letter),
            esc(letter)
        );
        for (label, f) in entries.iter() {
            h += &format!(
                "<li>{}<span class=\"small\">{}</span></li>",
                a(dest, &function_path(s(&f["function_id"])), label),
                esc(s(&f["descriptions"]["short"]))
            );
        }
        h += "</ul></section>";
    }
    output(out, dest, "Function index, A to Z", &h)
}

fn homepage(
    out: &Path,
    input: &Inputs,
    episodes: &[(String, String, String, String)],
) -> Result<(), String> {
    let dest = "index.html";
    let mut h=String::from("<div class=\"hero\"><p class=\"eyebrow\">DNA CALC</p><h1>The Handbook of Excel Functions</h1><p class=\"subtitle\">What Excel functions do. How they work. Where they surprise you.</p><p class=\"explain\">A practical reference for using and implementing Excel functions, with runnable formulas, numerical quirks and differences between versions.</p><p class=\"about\">The Handbook is written by the <a href=\"https://dnacalc.com/\">DNA Calc</a> project, which is building an open-source spreadsheet engine in Rust that aims to give the same answers as Excel. Where these pages say <em>we</em> or <em>our implementation</em>, they mean DNA Calc and its function library, CalcFunc. Each function page says how CalcFunc's answers compare with answers recorded from Excel, and its calculator runs DNA Calc's formula engine in your browser.</p><p class=\"small about-marks\">Microsoft, Excel and related marks are trademarks of Microsoft Corporation. This project is not affiliated with or endorsed by Microsoft.</p></div><form class=\"searchrow\" action=\"catalogue/index.html\" method=\"get\"><label class=\"sr-only\" for=\"home-search\">Find a function</label><input id=\"home-search\" name=\"q\" type=\"search\" autocomplete=\"off\" spellcheck=\"false\" placeholder=\"ROUND, XLOOKUP, ARRONDI, payment…\" data-instant-search><button type=\"submit\">Find a function</button></form><p class=\"browse\"><a href=\"catalogue/index.html\">Browse every entry by category →</a> · <a href=\"catalogue/a-z.html\">A to Z index</a></p>");
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
    h+="</div><section class=\"card audience\"><h2>For engine builders, auditors, and the numerically curious</h2><p>Start with a function's call signature, read the recorded surprises, or follow the research behind an algorithm.</p><p><a href=\"notes/index.html\">Where Excel's answer is inaccurate</a> · <a href=\"lastbit/index.html\">The Last Bit</a></p>";
    if let Some((_, dest, title, label)) = episodes.first() {
        h += &format!(
            "<p>{} <span class=\"small\">{label}</span></p>",
            a("index.html", dest, title)
        );
    }
    h+="</section><p class=\"small coverage-line\"><a href=\"implementation/index.html\">How DNA Calc compares with Excel, function by function →</a></p><p class=\"small coverage-line\"><a href=\"coverage/index.html\">What this Handbook covers, and its limits →</a></p>";
    output(out, dest, "The Handbook of Excel Functions", &h)
}

fn coverage(
    out: &Path,
    input: &Inputs,
    withheld: &[Value],
    unresolved: &BTreeSet<String>,
) -> Result<(), String> {
    let placeholder = input
        .functions
        .iter()
        .filter(|f| f["signature_placeholder"] == true)
        .count();
    let counts = &input.calc.state["counts"];
    let mut h=format!("<p class=\"eyebrow\">What exists in this build</p><h1>Coverage and limits</h1><p>{RECONCILIATION}. Every count below uses this full {ENTRIES}-entry catalogue. The published-row count is not a coverage denominator.</p><dl class=\"counts\"><dt>Entry pages generated</dt><dd>{ENTRIES} of {ENTRIES}</dd><dt>Curated prose files present</dt><dd>{} of {ENTRIES}</dd><dt>Projection-only pages</dt><dd>{} of {ENTRIES}</dd><dt>Placeholder signatures</dt><dd>{placeholder} of {ENTRIES}</dd><dt>Entries with Handbook vector suites</dt><dd>0 of {ENTRIES}</dd><dt>Entries with suite-admitted Handbook implementations</dt><dd>0 of {ENTRIES}</dd></dl><p>Curation counts file presence; it does not grade accuracy or depth. Notes and projected metadata can exist on a projection-only page.</p>",input.curated.len(),ENTRIES-input.curated.len());
    h += "<h2>Our Rust implementation</h2><dl class=\"counts\">";
    for (state, title, _) in implementation::STATES {
        h += &format!(
            "<dt><a href=\"../implementation/index.html#{state}\">{}</a></dt><dd>{} of {ENTRIES}</dd>",
            esc(title),
            counts[state]
        );
    }
    h += "</dl><p>Signatures, arity and admission come from the calc-func catalogue and the calc-fml built-ins at the pinned DnaCalc commit, or, for an entry those do not describe, from the named Microsoft page.</p>";
    let group = |label: &str| {
        let members: Vec<_> = input
            .functions
            .iter()
            .filter(|f| f["admission"]["label"] == label)
            .map(|f| {
                a(
                    "coverage/index.html",
                    &function_path(s(&f["function_id"])),
                    s(&f["surface_name"]),
                )
            })
            .collect();
        format!("{} of {ENTRIES}: {}", members.len(), members.join(", "))
    };
    h += &format!(
        "<h2>Entries that do not calculate here</h2><dl class=\"counts wide\"><dt>Deferred by policy</dt><dd>{}</dd><dt>Not yet implemented</dt><dd>{}</dd><dt>Catalogue row that declines every call</dt><dd>{}</dd><dt>Needs a host</dt><dd>{}</dd></dl><p>Deferred by policy means the answer needs an outside service, so DNA Calc does not run the function yet; each still has a full entry. Not yet implemented means the work is still to land. These states come from the pinned commit and change when code lands.</p><p>The formula layer handles {} without a catalogue row; they calculate here.</p>",
        group("deferred"),
        group("not_implemented"),
        group("not_modelled"),
        group("host"),
        group("formula_layer"),
    );
    h+=&format!("<h2>Notes</h2><p>{} note records, rendered from <code>content/notes/FN-*.json</code>. {} The generated output includes its own <a href=\"../api/notes.json\">notes API</a> derived from those same records. <a href=\"../notes/index.html\">Read the notes</a>.</p>",input.notes.len(),if input.note_api_matches {"The checked-in notes API agrees with these inputs."}else{"The checked-in notes API differs from source JSON; this site uses the source records to avoid dropping later notes."});
    h+="<h2>Catalogue boundary</h2><p>The first catalogue held 541 entries. Thirteen functions were added from Microsoft's pages and the calc-func catalogue: PY, FIELDVALUE, ECMA.CEILING, IMPORTCSV, IMPORTTEXT, FLATTEN, HAS, HASANY, HASALL and the four FORECAST.ETS functions. Excel macro-sheet functions and the Thai language functions stay outside the catalogue: Microsoft's alphabetical function list does not include them.</p>";
    h += &format!(
        "<p><a href=\"{}\">Function-help completeness audit, 24 August 2026</a></p>",
        source_link(
            &input.revision,
            "docs/audits/FUNCTION_HELP_COMPLETENESS_20260824.md"
        )
    );
    h+="<h2>Sources</h2><p>Function explanations cite Microsoft documentation, numerical literature and measured Excel answers. Measured answers name their build and input route; missing build information stays explicit.</p>";
    h += &format!(
        "<p><a href=\"../api/build.json\">Build inventory</a> · source <code>{}</code>.</p>",
        esc(&input.revision)
    );
    if input.tree_state == "dirty" {
        h+="<p class=\"notice\">This build includes local changes. Its source revision alone cannot reproduce the folder; review and commit those changes before selecting an artifact for publication.</p>";
    }
    h+=&format!("<h2>Editorial limits</h2><p>{} source passages are visibly withheld because their wording fails the rendered claim-language gate. The sources remain unchanged. The build inventory records their source paths and excerpts for review. Automated checks detect forbidden phrases, common unscoped status wording and missing count reconciliation; editorial review is still required to assess the meaning and evidence of every claim.</p>",withheld.len());
    if !unresolved.is_empty() {
        h+=&format!("<p>{} source links do not resolve to a known repository file. They link to the citing source page for inspection. These references require editorial review; no local dead link is substituted.</p>",unresolved.len());
    }
    h+="<h2>Drafts and working notes</h2><p>The Last Bit marks each part as an article, a draft or working notes. There are no Atlas plates in this build; the homepage exhibit uses recorded note data.</p><h2>How this folder was checked</h2><p>Generation runs the local link and claim-language checks before replacing the output folder. Browser checks are a separate engineering step; this page does not claim a browser result for an unchecked future build.</p>";
    output(out, "coverage/index.html", "Coverage and limits", &h)
}

const RULES: &str = r#"<h1>Measurements and test suites</h1><p>A measured Excel answer applies to the named input, Excel build and platform. Mathematical reference values and OneCalc results are labelled separately. Missing build information stays explicit.</p><p>No Handbook test suite is published yet. Equality claims require a published suite, an Excel build and a platform. Implementation listings must name the suite they pass.</p><p>Each function has the same six sections: What it does, Try it, Quirks, How to implement, Version differences and Alternatives. Sources and evidence appear below them. Brief entries mark guidance that is still being written.</p>"#;

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
            assert!(html.contains("Sources and evidence"));
            assert!(
                !html.contains("A source passage is withheld under Charter"),
                "{}: edit claim wording so reader guidance stays visible",
                s(&f["function_id"])
            );
            let headings: Vec<_> = reference::SECTIONS
                .iter()
                .map(|(id, title)| format!("<h2 id=\"{id}\">{title}</h2>"))
                .collect();
            let mut last = 0;
            for heading in headings {
                let at = html.find(&heading).expect(&heading);
                assert!(at > last);
                last = at;
            }
            assert!(html.contains(RECONCILIATION));
            assert_eq!(html.matches("<h1").count(), 1);
            assert_eq!(html.matches("<h2").count(), 6, "{}", s(&f["function_id"]));
            for forbidden in [
                "mixed-vintage",
                "pre-ledger",
                "OxFunc admission",
                "live-registry",
                "KernelSignatureClass",
            ] {
                assert!(
                    !html.contains(forbidden),
                    "{}: {forbidden}",
                    s(&f["function_id"])
                );
            }
            let record =
                json(&first.join(format!("api/functions/{}.json", s(&f["function_id"])))).unwrap();
            assert_eq!(record["function_id"], f["function_id"]);
            for key in ["versions", "implementations", "variants"] {
                assert!(record[key].is_array());
            }
            for implementation in record["implementations"].as_array().unwrap() {
                if implementation["suite"].is_null() {
                    assert_eq!(
                        implementation["flavour"],
                        "excel-compatibility-candidate",
                        "{}: a source snapshot without a published suite must remain a candidate",
                        s(&f["function_id"])
                    );
                }
            }
            assert_eq!(
                record["url"],
                format!(
                    "https://handbook.dnacalc.com/{}",
                    function_path(s(&f["function_id"]))
                )
            );
        }
        assert_eq!(inventory.len(), files(&second).unwrap().len());
        for path in inventory {
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
        assert!(notes.contains("<h1>Where Excel's answer is inaccurate</h1>"));
        // The catalogue must never share a case-folded path with the INDEX function's page.
        assert!(!first.join("functions/index.html").exists());
        assert!(first.join("functions/INDEX.html").exists());
        let seriessum = read(&first.join("functions/SERIESSUM.html")).unwrap();
        assert!(seriessum.contains("SERIESSUM(x, n, m, coefficients)"));
        assert!(seriessum.contains("<details class=\"rust-code\">"));
        assert!(seriessum.contains("github.com/DnaCalc/DnaCalc/blob/"));
        assert!(seriessum.contains("DNA Calc's implementation: same results as Excel in every case we compared"));
        assert!(seriessum.contains("<h3 id=\"dna-calc\">DNA Calc's implementation</h3>"));
        let series = read(&first.join("lastbit/index.html")).unwrap();
        assert!(series.contains("Working notes") && series.contains("Draft article"));
        assert!(first.join("lastbit/the-last-bit-2026-07.pdf").is_file());
        checks::check(&first, true, true).unwrap();
        std::fs::remove_dir_all(scratch).unwrap();
    }
}
