use crate::util::{files, normalize, read, RECONCILIATION};
use regex::Regex;
use std::sync::LazyLock;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

static HIDDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)<!--.*?-->|<script\b[^>]*>.*?</script>|<style\b[^>]*>.*?</style>").unwrap()
});
static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());
static BLOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"</?(?:html|head|title|body|main|header|footer|nav|article|section|aside|div|p|h[1-6]|li|ul|ol|table|tr|td|th|dl|dt|dd|details|summary|blockquote|pre|br)\b[^>]*>").unwrap()
});
static ENTITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"&(#x[0-9a-fA-F]+|#[0-9]+|[a-zA-Z]+);").unwrap());
static EQUALITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bbit[\s-]+(?:exact(?:ness|ly)?|for[\s-]+bit)\b").unwrap());
static TERM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(done|complete|verified)\b").unwrap());
static SCOPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(no|not|nothing|none|nobody|never|nowhere|unverified|unmeasured|undocumented|whether|would|should|could|if|until|without|against|scope|scoped|suite|rows|inputs|records|documentation|documented|inventory|list|table|formula|operation|algorithm|expression|characterisation|characterization|proof|mathematical)\b",
        // Here completion describes a particular mathematical object/operation,
        // rather than awarding a status to the Handbook or an implementation.
        r"|\bcomplete\s+(?:months|years|pairs|row|column|numbers|definition|cancellation|test)\b",
        r"|\b(?:collapse|counting|multiplication|division|subtraction|precision|seam)\b",
        r"|Excel\s+(?:16|build)|EV-[A-Z]+-\d+|FUNC\.[A-Z]"
    )).unwrap()
});

static COUNT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:\d+\s+(?:of\s+541|curated|projection-only|entries|functions)|\d+(?:\.\d+)?%)")
        .unwrap()
});
static READINESS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bproduction[ -]ready\b").unwrap());

pub fn text(html: &str) -> String {
    let html = HIDDEN.replace_all(html, " ").replace(['\n', '\r'], " ");
    let html = BLOCK.replace_all(&html, "\n");
    let html = TAG.replace_all(&html, "").into_owned();
    decode(&html)
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
fn decode(html: &str) -> String {
    ENTITY
        .replace_all(html, |c: &regex::Captures| {
            let e = &c[1];
            let n = e
                .strip_prefix("#x")
                .and_then(|x| u32::from_str_radix(x, 16).ok())
                .or_else(|| e.strip_prefix('#').and_then(|x| x.parse().ok()))
                .and_then(char::from_u32);
            n.map(|x| x.to_string()).unwrap_or_else(|| {
                match e {
                    "amp" => "&",
                    "lt" => "<",
                    "gt" => ">",
                    "quot" => "\"",
                    "apos" => "'",
                    "nbsp" => " ",
                    "hyphen" | "dash" | "ndash" | "mdash" | "minus" => "-",
                    _ => &c[0],
                }
                .to_string()
            })
        })
        .into_owned()
}
pub fn claim_issues(prose: &str) -> Vec<String> {
    let mut issues = Vec::new();
    let normalized = prose.replace(
        [
            '\u{2010}', '\u{2011}', '\u{2012}', '\u{2013}', '\u{2014}', '\u{2212}',
        ],
        "-",
    );
    if EQUALITY.is_match(&normalized) {
        issues.push("§7.8: equality phrase without a published Handbook suite".into());
    }
    // Conservative copy gate, not a semantic proof: inspect each sentence containing
    // completion/verification language for a stated scope or an explicit negative.
    for sentence in prose.split(['.', '!', '?', '\n']) {
        if TERM.is_match(sentence) && !SCOPE.is_match(sentence) {
            issues.push(format!(
                "§7.1: completion/verification wording needs scope: {}",
                sentence.trim()
            ));
        }
    }
    issues
}

pub fn check(root: &Path, links: bool, claims: bool) -> Result<(), String> {
    let all = files(root)?;
    let mut pages = BTreeMap::new();
    let ids = Regex::new(r#"\bid="([^"]*)""#).unwrap();
    let attrs = Regex::new(
        r#"\b(href|src|action|aria-controls|aria-describedby|aria-labelledby|for)="([^"]*)""#,
    )
    .unwrap();
    let rendered_attrs = Regex::new(r#"\b(?:alt|title|aria-label|placeholder)="([^"]*)""#).unwrap();
    let mut problems = Vec::new();
    for path in all
        .iter()
        .filter(|p| p.extension().is_some_and(|x| x == "html"))
    {
        let html = read(path)?;
        let mut anchors = BTreeSet::new();
        for c in ids.captures_iter(&html) {
            if !anchors.insert(decode(&c[1])) {
                problems.push(format!("{}: duplicate id {}", path.display(), &c[1]));
            }
        }
        pages.insert(path.clone(), (html, anchors));
    }
    if pages.is_empty() {
        return Err(format!("{}: no HTML pages to check", root.display()));
    }
    for (path, (html, _)) in &pages {
        if claims {
            for issue in claim_issues(&text(html)) {
                problems.push(format!("{}: {issue}", path.display()));
            }
            for c in rendered_attrs.captures_iter(html) {
                for issue in claim_issues(&decode(&c[1])) {
                    problems.push(format!("{}: attribute: {issue}", path.display()));
                }
            }
            let visible = text(html);
            if COUNT.is_match(&visible) && !visible.contains(RECONCILIATION) {
                problems.push(format!(
                    "{}: §7.3: counts need the full reconciliation",
                    path.display()
                ));
            }
            if READINESS.is_match(&visible) {
                problems.push(format!("{}: prohibited readiness label", path.display()));
            }
        }
        if !links {
            continue;
        }
        for c in attrs.captures_iter(html) {
            let value = decode(&c[2]);
            if value
                .to_ascii_lowercase()
                .contains("github.com/dnacalc/oxfunc-evidence")
            {
                problems.push(format!(
                    "{}: private evidence repository link; cite the mapped CalcFunc-Evidence commit: {value}",
                    path.display()
                ));
                continue;
            }
            if matches!(
                &c[1],
                "aria-controls" | "aria-describedby" | "aria-labelledby" | "for"
            ) {
                for id in value.split_whitespace() {
                    if !pages[path].1.contains(id) {
                        problems.push(format!("{}: missing local id {id}", path.display()));
                    }
                }
                continue;
            }
            if value.starts_with("https://")
                || value.starts_with("http://")
                || value.starts_with("mailto:")
            {
                continue;
            }
            if value.is_empty()
                || value.starts_with('/')
                || value.contains(':')
                || value.contains('\\')
            {
                problems.push(format!("{}: nonportable URL {value:?}", path.display()));
                continue;
            }
            let (link, fragment) = value.split_once('#').unwrap_or((&value, ""));
            let link = link.split('?').next().unwrap();
            let target = if link.is_empty() {
                path.clone()
            } else {
                let rel = path
                    .strip_prefix(root)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(percent_decode(link));
                let Some(rel) = normalize(&rel) else {
                    problems.push(format!("{}: link escapes output: {value}", path.display()));
                    continue;
                };
                root.join(rel)
            };
            let target = if target.is_dir() {
                target.join("index.html")
            } else {
                target
            };
            if !target.is_file() {
                problems.push(format!("{}: missing target {value}", path.display()));
            } else if !fragment.is_empty()
                && pages.contains_key(&target)
                && !pages[&target].1.contains(&percent_decode(fragment))
            {
                problems.push(format!("{}: missing fragment {value}", path.display()));
            }
        }
    }
    if problems.is_empty() {
        println!(
            "Checked {} HTML pages: {}{}",
            pages.len(),
            if links {
                "local links, fragments, labels"
            } else {
                ""
            },
            if claims {
                "; rendered claim language"
            } else {
                ""
            }
        );
        Ok(())
    } else {
        Err(format!(
            "{} output check failures:\n{}",
            problems.len(),
            problems.join("\n")
        ))
    }
}
fn percent_decode(s: &str) -> String {
    let mut out = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(n) =
                u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap_or(""), 16)
            {
                out.push(n);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn malformed_percent_encoding_never_slices_through_a_unicode_character() {
        assert_eq!(percent_decode("%aé"), "%aé");
        assert_eq!(percent_decode("caf%C3%A9"), "café");
    }
    #[test]
    fn dead_targets_fragments_and_reader_visible_attributes_fail_the_gate() {
        let root = std::env::temp_dir().join(format!("efh-site-check-test-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        crate::util::write(&root,"index.html",r##"<h1 id="main">Index</h1><a href="other.html#absent">Other</a><label for="unknown">Name</label><a href="missing.html">Missing</a><p title="bit-exact">Unqualified</p><a href="https://github.com/DnaCalc/OxFunc-Evidence/blob/old/answer.json">Private</a>"##).unwrap();
        crate::util::write(&root, "other.html", "<h1>Other</h1>").unwrap();
        let error = check(&root, true, true).unwrap_err();
        for expected in [
            "missing fragment",
            "missing target",
            "missing local id",
            "attribute: §7.8",
            "private evidence repository link",
        ] {
            assert!(error.contains(expected), "{expected}: {error}");
        }
        crate::util::write(
            &root,
            "index.html",
            r##"<h1 id="main">Index</h1><a href="other.html#present">Other</a>"##,
        )
        .unwrap();
        crate::util::write(&root, "other.html", r##"<h1 id="present">Other</h1>"##).unwrap();
        check(&root, true, true).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn equality_language_cannot_hide_in_markup_or_entities() {
        for html in [
            "<b>bit</b>-exact",
            "bit&#45;for&#45;bit",
            "BIT‑EXACT",
            "bit for bit",
            "bit-ex<span>act</span>",
            "bit&minus;exact",
            "bit-exactness",
        ] {
            assert!(!claim_issues(&text(html)).is_empty(), "{html}");
        }
        assert!(claim_issues("excel-bitexact names a target").is_empty());
    }
    #[test]
    fn claims_need_scope() {
        for t in ["Verified", "This is complete", "We are done"] {
            assert!(!claim_issues(t).is_empty());
        }
        for t in [
            "No Handbook suite has verified this",
            "Verified against suite v1 over 15 inputs",
            "the complete inventory of arguments",
        ] {
            assert!(claim_issues(t).is_empty(), "{t}");
        }
    }
    #[test]
    fn qualifications_cannot_leak_from_a_neighbouring_paragraph() {
        assert!(
            !claim_issues(&text("<p>Verified</p><p>No suite has been published.</p>")).is_empty()
        );
        assert!(claim_issues(&text(
            "<p>No record has been <em>verified</em> against a Handbook suite.</p>"
        ))
        .is_empty());
    }
    #[test]
    fn source_line_wrapping_and_mathematical_objects_are_not_status_claims() {
        assert!(claim_issues(&text(
            "<p>Nothing has been\n<em>verified</em> against Excel.</p>"
        ))
        .is_empty());
        for prose in [
            "fewer than two complete pairs",
            "The number of complete months",
            "None of this is verified here",
        ] {
            assert!(claim_issues(prose).is_empty(), "{prose}");
        }
    }
}
