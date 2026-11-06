//! What we conclude about our own Rust implementation, and the code itself. Everything here reads
//! `data/calcfunc/`, which `tools/efh-calcfunc/regenerate.py` writes from a pinned DnaCalc commit.
use crate::util::*;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};

pub const STATES: [(&str, &str, &str); 7] = [
    ("matches", "Matches every recorded answer", "Every recorded Excel answer for the entry is reproduced exactly, in the answer bank and in the workbook corpus."),
    ("close", "Close: last-bit deviations", "Only last-bit deviations remain: finite numbers within 16 ULP of Excel's answer. The worst distance is stated."),
    ("differs", "Known larger differences", "At least one recorded answer differs by more than 16 ULP, differs in kind or value, or is refused."),
    ("refuses", "Refuses or not yet implemented", "The entry declines every recorded call, or has no implementation yet."),
    ("deferred", "Deferred: needs an outside service", "Deferred by policy. The answer comes from a service outside the workbook, so DNA Calc does not run the function yet. The function still has a full entry."),
    ("host", "Needs a host", "The answer depends on a capability of the host application, such as loading a library, reading a file or starting a data server."),
    ("unmeasured", "Implemented, no attributed answers", "The entry is implemented, but no recorded answer is attributed to it alone. These are mostly reference operators, seen only inside whole formulas."),
];

pub const LANGUAGES: [(&str, &str); 40] = [
    ("ar-SA", "Arabic"),
    ("bg-BG", "Bulgarian"),
    ("cs-CZ", "Czech"),
    ("da-DK", "Danish"),
    ("de-DE", "German"),
    ("el-GR", "Greek"),
    ("en-AU", "English (Australia)"),
    ("en-GB", "English (United Kingdom)"),
    ("en-US", "English (United States)"),
    ("es-ES", "Spanish"),
    ("et-EE", "Estonian"),
    ("fi-FI", "Finnish"),
    ("fr-FR", "French"),
    ("he-IL", "Hebrew"),
    ("hr-HR", "Croatian"),
    ("hu-HU", "Hungarian"),
    ("id-ID", "Indonesian"),
    ("it-IT", "Italian"),
    ("ja-JP", "Japanese"),
    ("ko-KR", "Korean"),
    ("lt-LT", "Lithuanian"),
    ("lv-LV", "Latvian"),
    ("nb-NO", "Norwegian"),
    ("nl-NL", "Dutch"),
    ("pl-PL", "Polish"),
    ("pt-BR", "Portuguese (Brazil)"),
    ("pt-PT", "Portuguese (Portugal)"),
    ("ro-RO", "Romanian"),
    ("ru-RU", "Russian"),
    ("sk-SK", "Slovak"),
    ("sl-SI", "Slovenian"),
    ("sr-Latn-RS", "Serbian (Latin)"),
    ("sv-SE", "Swedish"),
    ("th-TH", "Thai"),
    ("tr-TR", "Turkish"),
    ("uk-UA", "Ukrainian"),
    ("vi-VN", "Vietnamese"),
    ("zh-CN", "Chinese (Simplified)"),
    ("zh-HK", "Chinese (Hong Kong)"),
    ("zh-TW", "Chinese (Traditional)"),
];

pub struct CalcFunc {
    pub pin: Value,
    pub state: Value,
    pub code: Value,
    pub fragments: Value,
    pub variants: Value,
}

impl CalcFunc {
    pub fn load(repo: &Path) -> Result<Self, String> {
        let dir = repo.join("data/calcfunc");
        let this = Self {
            pin: json(&dir.join("pin.json"))?,
            state: json(&dir.join("state.json"))?,
            code: json(&dir.join("code.json"))?,
            fragments: json(&dir.join("fragments.json"))?,
            variants: json(&dir.join("variants.json"))?,
        };
        if this.pin["commit"] != this.state["commit"] || this.pin["commit"] != this.code["commit"] {
            return Err("data/calcfunc files name different DnaCalc commits; run tools/efh-calcfunc/regenerate.py".into());
        }
        Ok(this)
    }
    pub fn commit(&self) -> &str {
        s(&self.pin["commit"])
    }
    pub fn short(&self) -> &str {
        &self.commit()[..12]
    }
    pub fn entry(&self, id: &str) -> &Value {
        &self.state["functions"][id]
    }
    pub fn github(&self, fragment: &Value) -> String {
        format!(
            "{}/blob/{}/{}#L{}-L{}",
            s(&self.pin["repository"]),
            self.commit(),
            s(&fragment["file"]),
            fragment["start_line"],
            fragment["end_line"]
        )
    }
    /// The published JSON for one entry: the conclusion, and its code reference.
    pub fn implementation(&self, id: &str) -> Vec<Value> {
        let row = self.entry(id);
        let Some(key) = row["code"].as_str() else {
            return Vec::new();
        };
        let own = &self.code["entries"][key]["own"];
        let first = &self.fragments[s(&own[0])];
        vec![json!({
            "language": "Rust",
            "flavour": "excel-bitexact",
            "name": "CalcFunc",
            "repository": self.pin["repository"],
            "commit": self.commit(),
            "source": self.github(first),
            "suite": Value::Null,
            "state": row["state"],
            "summary": row["summary"],
            "own_code": own.as_array().map(|keys| keys.iter().map(|k| self.github(&self.fragments[s(k)])).collect::<Vec<_>>()),
            "shared_code": self.code["entries"][key]["shared"],
        })]
    }
}

const KEYWORDS: [&str; 34] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while",
];

fn span(out: &mut String, class: &str, text: &str) {
    out.push_str(&format!("<span class=\"tok-{class}\">{}</span>", esc(text)));
}

/// Build-time syntax highlighting. A lexer of comments, strings, numbers, keywords, types and
/// calls: nothing here needs to understand Rust, and nothing runs in the browser.
pub fn highlight(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let text = |a: usize, b: usize| chars[a..b].iter().collect::<String>();
    let (mut i, n) = (0, chars.len());
    let mut out = String::with_capacity(code.len() * 2);
    while i < n {
        let c = chars[i];
        let next = chars.get(i + 1).copied().unwrap_or(' ');
        if c == '/' && next == '/' {
            let start = i;
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            span(&mut out, "c", &text(start, i));
        } else if c == '/' && next == '*' {
            let start = i;
            i += 2;
            while i + 1 < n && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(n);
            span(&mut out, "c", &text(start, i));
        } else if c == '"' {
            let start = i;
            i += 1;
            while i < n && chars[i] != '"' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i = (i + 1).min(n);
            span(&mut out, "s", &text(start, i));
        } else if c == '\'' && (next == '\\' || chars.get(i + 2) == Some(&'\'')) {
            let start = i;
            i += 1;
            while i < n && chars[i] != '\'' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i = (i + 1).min(n);
            span(&mut out, "s", &text(start, i));
        } else if c == '#' && next == '[' {
            let start = i;
            while i < n && chars[i] != ']' && chars[i] != '\n' {
                i += 1;
            }
            i = (i + 1).min(n);
            span(&mut out, "a", &text(start, i));
        } else if c.is_ascii_digit() {
            let start = i;
            while i < n
                && (chars[i].is_ascii_alphanumeric()
                    || chars[i] == '_'
                    || (chars[i] == '.' && chars.get(i + 1).is_some_and(char::is_ascii_digit)))
            {
                i += 1;
            }
            span(&mut out, "n", &text(start, i));
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word = text(start, i);
            if KEYWORDS.contains(&word.as_str()) {
                span(&mut out, "k", &word);
            } else if chars.get(i) == Some(&'!') {
                span(&mut out, "m", &word);
            } else if word.chars().next().is_some_and(char::is_uppercase) {
                span(&mut out, "t", &word);
            } else if chars.get(i) == Some(&'(') {
                span(&mut out, "f", &word);
            } else {
                out.push_str(&esc(&word));
            }
        } else {
            out.push_str(&esc(&c.to_string()));
            i += 1;
        }
    }
    out
}

pub fn code_page(module: &str) -> String {
    format!("code/{}.html", module.replace('/', "-"))
}
fn fragment_id(fragment: &Value) -> String {
    format!(
        "fn-{}-{}",
        slug(&s(&fragment["name"]).replace("::", "-")),
        fragment["start_line"]
    )
}
fn block(calc: &CalcFunc, fragment: &Value) -> String {
    let lines = fragment["end_line"].as_u64().unwrap_or(0)
        - fragment["start_line"].as_u64().unwrap_or(0)
        + 1;
    format!(
        "<figure class=\"code-block\"><figcaption><code>{}</code> · {} lines · <a href=\"{}\">{} on GitHub</a></figcaption><pre class=\"rust\"><code>{}</code></pre></figure>",
        esc(s(&fragment["name"])),
        lines,
        esc(&calc.github(fragment)),
        esc(s(&fragment["file"]).trim_start_matches("crates/")),
        highlight(s(&fragment["text"]))
    )
}

const INLINE_LINES: u64 = 1500;

/// The collapsed Rust section of a function page.
pub fn code_section(calc: &CalcFunc, id: &str, page: &str) -> String {
    let row = calc.entry(id);
    let Some(key) = row["code"].as_str() else {
        return "<h3 id=\"rust-code\">Rust code</h3><p class=\"small\">This entry has no Rust implementation yet.</p>".into();
    };
    let entry = &calc.code["entries"][key];
    let mut shown = String::new();
    let mut total = 0;
    let mut remaining = Vec::new();
    for fragment_key in entry["own"].as_array().into_iter().flatten() {
        let fragment = &calc.fragments[s(fragment_key)];
        let lines = fragment["end_line"].as_u64().unwrap_or(0)
            - fragment["start_line"].as_u64().unwrap_or(0)
            + 1;
        if total + lines > INLINE_LINES && total > 0 {
            remaining.push(fragment);
            continue;
        }
        total += lines;
        shown += &block(calc, fragment);
    }
    let mut list = |title: &str, keys: &Value| {
        let keys = keys.as_array().map(Vec::as_slice).unwrap_or_default();
        if keys.is_empty() {
            return;
        }
        shown += &format!("<h4>{title}</h4><ul class=\"kernel-list\">");
        for fragment_key in keys {
            let fragment = &calc.fragments[s(fragment_key)];
            shown += &format!(
                "<li><a href=\"{}#{}\"><code>{}</code></a> <span class=\"small\">{}</span></li>",
                esc(&relative(page, &code_page(s(&fragment["module"])))),
                fragment_id(fragment),
                esc(s(&fragment["name"])),
                esc(s(&fragment["module"]))
            );
        }
        shown += "</ul>";
    };
    list("Kernels shared with other functions", &entry["shared"]);
    list("Argument and result plumbing", &entry["plumbing"]);
    for fragment in remaining {
        shown += &format!(
            "<p class=\"small\">Longer code continues at <a href=\"{}\"><code>{}</code> on GitHub</a>.</p>",
            esc(&calc.github(fragment)),
            esc(s(&fragment["name"]))
        );
    }
    format!(
        "<h3 id=\"rust-code\">Rust code</h3><details class=\"rust-code\"><summary>Show the CalcFunc source of this function</summary><p class=\"small\">Quoted from the public repository DnaCalc/DnaCalc, crates <code>calc-func</code> and <code>calc-fml</code>, at commit <code>{}</code>. Comments are the source's own. A kernel shared with other functions appears once, on its module page.</p>{shown}</details>",
        calc.short()
    )
}

pub fn state_block(calc: &CalcFunc, id: &str, page: &str) -> String {
    let row = calc.entry(id);
    let state = s(&row["state"]);
    format!(
        "<section class=\"impl-state state-{state}\" aria-label=\"State of our implementation\"><p class=\"state-line\"><span class=\"state-dot\" aria-hidden=\"true\"></span><strong>{}</strong></p><p class=\"small\">{} <a href=\"{}\">How we measure this</a>{}</p></section>",
        esc(s(&row["summary"])),
        esc(s(&row["detail"])),
        esc(&relative(page, "implementation/index.html")),
        if row["code"].is_string() { " · <a href=\"#rust-code\">See the Rust code</a>" } else { "" }
    )
}

/// The names of this entry in every language the source has, and the page's local-name line.
pub fn local_names(f: &Value) -> String {
    let names = f["localized_names"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let family = f["localized_family_names"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let language = |tag: &str| {
        LANGUAGES
            .iter()
            .find(|(t, _)| *t == tag)
            .map_or(tag, |(_, name)| *name)
            .to_string()
    };
    let mut body = String::new();
    if !names.is_empty() {
        for n in names {
            body += &format!(
                "<tr data-locale=\"{}\"><td>{}</td><td lang=\"{}\"><code>{}</code></td></tr>",
                esc(s(&n["locale"])),
                esc(&language(s(&n["locale"]))),
                esc(s(&n["locale"])),
                esc(s(&n["name"]))
            );
        }
    } else if !family.is_empty() {
        for n in family {
            let list = n["names"]
                .as_array()
                .into_iter()
                .flatten()
                .map(s)
                .collect::<Vec<_>>()
                .join(", ");
            body += &format!(
                "<tr data-locale=\"{}\"><td>{}</td><td lang=\"{}\">{}</td></tr>",
                esc(s(&n["locale"])),
                esc(&language(s(&n["locale"]))),
                esc(s(&n["locale"])),
                esc(&list)
            );
        }
    } else {
        return "<p class=\"small local-none\">No local names are recorded for this entry.</p>"
            .into();
    }
    let distinct = names
        .iter()
        .filter(|n| n["name"] != f["surface_name"])
        .count();
    let summary = if !names.is_empty() {
        format!("Local names in {} languages", names.len())
    } else {
        format!("Local names of this family in {} languages", family.len())
    };
    let note = if names.is_empty() {
        "<p class=\"small\">Microsoft's list gives these names for the whole family without saying which belongs to which function, so they are shown together.</p>"
    } else if distinct == 0 {
        "<p class=\"small\">Every language in the source keeps the English name.</p>"
    } else {
        ""
    };
    format!(
        "<details class=\"local-names\" id=\"local-names\"><summary>{summary}</summary>{note}<div class=\"table-scroll\" role=\"region\" aria-label=\"Local names\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">Language</th><th scope=\"col\">Name in Excel</th></tr></thead><tbody>{body}</tbody></table></div><p class=\"small\">Names from Microsoft's support site.</p></details>"
    )
}

pub fn language_picker() -> String {
    let mut h = String::from("<label class=\"language-picker\" hidden><span>Local names</span><select id=\"language-picker\"><option value=\"\">English only</option>");
    for (tag, name) in LANGUAGES {
        h += &format!("<option value=\"{tag}\">{}</option>", esc(name));
    }
    h + "</select></label>"
}

/// `code/index.html` and one page per module, holding each shared kernel once.
pub fn code_pages(
    calc: &CalcFunc,
    mut output: impl FnMut(&str, &str, &str) -> Result<(), String>,
) -> Result<(), String> {
    let mut modules = BTreeMap::<String, Vec<&Value>>::new();
    let fragments = calc
        .fragments
        .as_object()
        .ok_or("fragments.json is not an object")?;
    let mut listed = std::collections::BTreeSet::new();
    for entry in calc.code["entries"]
        .as_object()
        .ok_or("code.json: entries")?
        .values()
    {
        for field in ["shared", "plumbing"] {
            for key in entry[field].as_array().into_iter().flatten() {
                listed.insert(s(key).to_string());
            }
        }
    }
    for (key, fragment) in fragments {
        if listed.contains(key) {
            modules
                .entry(s(&fragment["module"]).to_string())
                .or_default()
                .push(fragment);
        }
    }
    let mut index = format!("<p class=\"eyebrow\">Our implementation</p><h1>Shared Rust kernels</h1><p>CalcFunc has no repository of its own. It is the crate <code>calc-func</code> in the public repository <a href=\"{0}\">DnaCalc/DnaCalc</a>; catalogue rows are in <code>catalogue/</code> and the numeric kernels in <code>ported/</code>. These pages quote commit <code>{1}</code>. Code used by one function is on that function's page. Code used by several is here, once.</p><ul class=\"entries\">", esc(s(&calc.pin["repository"])), calc.short());
    for (module, items) in modules.iter_mut() {
        items.sort_by_key(|f| f["start_line"].as_u64());
        let dest = code_page(module);
        let mut h = format!("<p class=\"eyebrow\">Shared Rust kernels</p><h1><code>{}</code></h1><p class=\"small\"><a href=\"index.html\">All modules</a> · quoted at commit <code>{}</code> of DnaCalc/DnaCalc. Comments are the source's own.</p>", esc(module), calc.short());
        for fragment in items.iter() {
            h += &format!(
                "<section id=\"{}\">{}<p class=\"small\">Used by {} functions.</p></section>",
                fragment_id(fragment),
                block(calc, fragment),
                fragment["users"]
            );
        }
        output(&dest, module, &h)?;
        index += &format!("<li><a href=\"{}\"><code>{}</code></a><span class=\"small\">{} shared functions</span></li>", esc(&relative("code/index.html", &dest)), esc(module), items.len());
    }
    index += "</ul>";
    output("code/index.html", "Shared Rust kernels", &index)
}

/// The summary page: how many entries are in each state, and which.
pub fn summary_page(
    calc: &CalcFunc,
    functions: &[Value],
    function_path: impl Fn(&str) -> String,
) -> String {
    let dest = "implementation/index.html";
    let bank = &calc.pin["answer_bank"];
    let corpus = &calc.pin["corpus"];
    let mut h = format!(
        "<p class=\"eyebrow\">Our implementation</p><h1>How close our Rust is to Excel</h1><p>This page states what we conclude about our own implementation of each entry, CalcFunc, from recorded Excel answers. It is regenerated from DnaCalc commit <a href=\"{0}/commit/{1}\"><code>{2}</code></a>; nothing on it is typed by hand. CalcFunc has no repository of its own: it is the crate <code>calc-func</code> in <a href=\"{0}\">DnaCalc/DnaCalc</a>.</p><p>{RECONCILIATION}.</p><div class=\"table-scroll\" role=\"region\" aria-label=\"Entries in each state\" tabindex=\"0\"><table><thead><tr><th scope=\"col\">State</th><th scope=\"col\">Entries</th><th scope=\"col\">What it means</th></tr></thead><tbody>",
        esc(s(&calc.pin["repository"])), calc.commit(), calc.short()
    );
    for (state, title, meaning) in STATES {
        let count = calc.state["counts"][state].as_u64().unwrap_or(0);
        h += &format!(
            "<tr class=\"state-{state}\"><td><span class=\"state-dot\" aria-hidden=\"true\"></span><a href=\"#{state}\">{}</a></td><td>{count}</td><td>{}</td></tr>",
            esc(title), esc(meaning)
        );
    }
    h += "</tbody></table></div>";
    h += &format!(
        "<h2>What was compared</h2><p>Two sources of recorded Excel answers. The answer bank holds {} rows, of which {} match; the workbook corpus holds {} formula cells, of which {} match. Every mismatch, refusal and pending cell stays a failure in those totals. A row or cell belongs to one function, the outermost call of its formula. The two sources may overlap, so a function's count is the number of recorded answers compared, not the number of distinct calls.</p><p>A number matches only when all 64 bits of its binary64 value agree. The answer-bank counts were measured at revision <code>{}</code> and committed in DnaCalc; {} The corpus was run again at <code>{}</code> for this page and gives the same totals. These statements cover the recorded inputs, Excel builds and platforms and nothing else. No Handbook test suite is published yet.</p><p class=\"small\">{}</p>",
        fmt_count(&bank["rows"]),
        fmt_count(&bank["matched"]),
        fmt_count(&corpus["formula_cells"]),
        fmt_count(&corpus["matched"]),
        esc(&s(&calc.pin["measured_application_revision"])[..12]),
        if calc.pin["calc_func_unchanged_since_report"] == true { "<code>calc-func</code> has not changed since that report landed." } else { "<code>calc-func</code> has changed since that report landed, so the code shown may be newer than the counts." },
        calc.short(),
        esc(s(&calc.pin["rule"]))
    );
    for (state, title, meaning) in STATES {
        let members: Vec<_> = functions
            .iter()
            .filter(|f| calc.entry(s(&f["function_id"]))["state"] == state)
            .collect();
        h += &format!(
            "<section><h2 id=\"{state}\">{} ({})</h2><p>{}</p><ul class=\"state-list\">",
            esc(title),
            members.len(),
            esc(meaning)
        );
        for f in members {
            let id = s(&f["function_id"]);
            h += &format!(
                "<li><a href=\"{}\">{}</a><span class=\"small\">{}</span></li>",
                esc(&relative(dest, &function_path(id))),
                esc(s(&f["surface_name"])),
                esc(s(&calc.entry(id)["summary"]))
            );
        }
        h += "</ul></section>";
    }
    h
}

pub fn fmt_count(v: &Value) -> String {
    let digits = v.as_u64().unwrap_or(0).to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
