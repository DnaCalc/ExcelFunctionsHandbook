use crate::{
    checks,
    util::{normalize, reader_words, relative, slug, SOURCE},
};
use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub struct Renderer<'a> {
    pub repo: &'a Path,
    pub routes: &'a BTreeMap<String, String>,
    pub revision: &'a str,
    pub withheld: Vec<serde_json::Value>,
    pub unresolved: BTreeSet<String>,
}

pub fn body(source: &str) -> &str {
    if let Some(stripped) = source.strip_prefix("---\n") {
        stripped
            .split_once("\n---\n")
            .map(|(_, b)| b)
            .unwrap_or(source)
    } else {
        source
    }
}

impl Renderer<'_> {
    pub fn url(&mut self, source: &str, page: &str, url: &str) -> Option<String> {
        let mapped;
        let url = if source.starts_with("content/functions/") && url.contains('#') {
            let (path, fragment) = url.split_once('#').unwrap();
            let fragment = match fragment {
                "what-it-computes" | "arguments" => "what-it-does",
                "numerical-notes" | "notes-for-implementers" => "how-to-implement",
                "relationships" => "alternatives",
                "result-and-edge-cases" | "errors" | "notes" => "quirks",
                "excel-versions-platforms-and-cpus" => "version-differences",
                "page-vocabulary" | "what-has-not-been-checked" => "sources",
                other => other,
            };
            mapped = format!("{path}#{fragment}");
            mapped.as_str()
        } else {
            url
        };
        if url.starts_with("https://")
            || url.starts_with("http://")
            || url.starts_with("mailto:")
            || url.starts_with('#')
        {
            return Some(url.to_string());
        }
        if url.contains(':') || url.starts_with('/') || url.contains('\\') {
            self.unresolved.insert(format!("{source}: {url}"));
            return None;
        }
        let (path, fragment) = url.split_once('#').unwrap_or((url, ""));
        let p = normalize(&Path::new(source).parent().unwrap().join(path))?;
        let p = p.to_str()?;
        // Every canonical Markdown function path is registered, including the
        // entries whose prose is absent. JSON paths retain their JSON target.
        let target = self.routes.get(p).cloned();
        if let Some(target) = target {
            Some(format!(
                "{}{}",
                relative(page, &target),
                if fragment.is_empty() {
                    String::new()
                } else {
                    format!("#{fragment}")
                }
            ))
        } else if self.repo.join(p).exists() && !p.split('/').any(|x| x.starts_with("MATERIAL_")) {
            Some(format!(
                "{SOURCE}/{}/{}/{p}{}",
                if self.repo.join(p).is_dir() {
                    "tree"
                } else {
                    "blob"
                },
                self.revision,
                if fragment.is_empty() {
                    String::new()
                } else {
                    format!("#{fragment}")
                }
            ))
        } else {
            self.unresolved.insert(format!("{source}: {url}"));
            None
        }
    }

    pub fn render(
        &mut self,
        source: &str,
        page: &str,
        markdown: &str,
        strip_notes: bool,
    ) -> String {
        // The Windows curator preserves CRLF in Git (* -text). Parse a
        // normalized in-memory copy; never rewrite the source organ.
        let normalized = markdown.replace("\r\n", "\n");
        let markdown = body(&normalized);
        let stripped;
        let markdown = if strip_notes {
            stripped = Regex::new(r"(?s)## Notes\s*<!-- efh:notes begin:.*?<!-- efh:notes end -->")
                .unwrap()
                .replace_all(markdown, "")
                .into_owned();
            &stripped
        } else {
            markdown
        };
        let events = Parser::new_ext(
            markdown,
            Options::ENABLE_TABLES
                | Options::ENABLE_STRIKETHROUGH
                | Options::ENABLE_TASKLISTS
                | Options::ENABLE_FOOTNOTES,
        );
        let mut result = String::new();
        let mut group = Vec::new();
        let mut depth = 0;
        let mut seen_ids = BTreeMap::<String, usize>::new();
        for event in events {
            match &event {
                Event::Start(_) => depth += 1,
                Event::End(_) => depth -= 1,
                _ => (),
            }
            group.push(event);
            if depth != 0 {
                continue;
            }
            // A title is already supplied by the page template.
            if matches!(
                group.first(),
                Some(Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                }))
            ) {
                group.clear();
                continue;
            }
            let visible = group
                .iter()
                .filter_map(|e| match e {
                    Event::Text(s) | Event::Code(s) => Some(s.as_ref()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");
            let issues = checks::claim_issues(&visible);
            if !issues.is_empty() {
                let id = format!("withheld-{}", self.withheld.len() + 1);
                self.withheld.push(serde_json::json!({"source":source,"page":page,"id":id,"reasons":issues,"source_excerpt":visible}));
                result.push_str(&format!("<aside class=\"notice\" id=\"{id}\"><p>A source passage is withheld under Charter §7 pending editorial review of its claim language. <a href=\"{SOURCE}/blob/{}/{source}\">Read the repository source</a>.</p></aside>", self.revision));
                group.clear();
                continue;
            }
            let mut transformed = Vec::new();
            for event in group.drain(..) {
                match event {
                    Event::Start(Tag::Heading {
                        level,
                        id,
                        classes,
                        attrs,
                    }) => {
                        let raw = visible.trim();
                        let base = slug(raw);
                        let count = seen_ids.entry(base.clone()).or_insert(0);
                        *count += 1;
                        let name = if *count == 1 {
                            base
                        } else {
                            format!("{base}-{count}")
                        };
                        transformed.push(Event::Start(Tag::Heading {
                            level,
                            id: Some(id.unwrap_or_else(|| name.into())),
                            classes,
                            attrs,
                        }));
                    }
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }) => {
                        let url = self
                            .url(source, page, &dest_url)
                            .unwrap_or_else(|| format!("{SOURCE}/blob/{}/{source}", self.revision));
                        transformed.push(Event::Start(Tag::Link {
                            link_type,
                            dest_url: url.into(),
                            title,
                            id,
                        }));
                    }
                    Event::Start(Tag::Image { .. }) | Event::End(TagEnd::Image) => (),
                    Event::Text(text)
                        if text.contains("OxFunc")
                            || text.contains("oxfunc")
                            || text.contains("eference engine") =>
                    {
                        transformed.push(Event::Text(reader_words(&text).into()));
                    }
                    Event::Code(text) if text.contains("OxFunc") || text.contains("oxfunc") => {
                        transformed.push(Event::Code(reader_words(&text).into()));
                    }
                    Event::Html(h) | Event::InlineHtml(h) => {
                        if !h.trim_start().starts_with("<!--") {
                            transformed.push(Event::Text(h));
                        }
                    }
                    Event::Start(Tag::Table(a)) => {
                        transformed.push(Event::Html(CowStr::from("<div class=\"table-scroll\" role=\"region\" aria-label=\"Scrollable table\" tabindex=\"0\">")));
                        transformed.push(Event::Start(Tag::Table(a)));
                    }
                    Event::End(TagEnd::Table) => {
                        transformed.push(Event::End(TagEnd::Table));
                        transformed.push(Event::Html(CowStr::from("</div>")));
                    }
                    e => transformed.push(e),
                }
            }
            html::push_html(&mut result, transformed.into_iter());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_notes_are_read_only_and_json_will_supply_the_page_notes() {
        let routes = BTreeMap::new();
        let mut r = Renderer {
            repo: Path::new("."),
            routes: &routes,
            revision: "main",
            withheld: Vec::new(),
            unresolved: BTreeSet::new(),
        };
        let text="---\ndepth: curated\n---\n# X\n\n## Notes\n\n<!-- efh:notes begin: generated -->\nOld text\n<!-- efh:notes end -->\n\n## Sources\n\nA source.\n";
        let h = r.render(
            "content/functions/FUNC.X.md",
            "functions/X.html",
            text,
            true,
        );
        assert!(!h.contains("Old text"));
        assert!(h.contains("Sources"));
        assert_eq!(body(text).lines().next(), Some("# X"));
        let crlf = r.render(
            "content/functions/FUNC.X.md",
            "functions/X.html",
            &text.replace('\n', "\r\n"),
            true,
        );
        assert_eq!(h, crlf);
    }
    #[test]
    fn escape_raw_html_and_withhold_claims_without_altering_source() {
        let routes = BTreeMap::new();
        let mut r = Renderer {
            repo: Path::new("."),
            routes: &routes,
            revision: "main",
            withheld: Vec::new(),
            unresolved: BTreeSet::new(),
        };
        let h = r.render(
            "test.md",
            "index.html",
            "<script>alert(1)</script>\n\nIt is bit-exact.\n\nHonest prose.",
            false,
        );
        assert!(!h.contains("<script>"));
        assert!(!h.contains("bit-exact"));
        assert!(h.contains("Honest prose"));
        assert_eq!(r.withheld.len(), 1);
    }
}
