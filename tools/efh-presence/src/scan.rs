//! Mechanical text matching.
//!
//! This is a byte-level re-implementation of the matching discipline documented
//! in `harvest_impl_map.py` sections (1)–(3). It is a RE-DERIVATION: no output
//! of that script is read, only its rules are reproduced, because the script's
//! downstream consumers were measured at a 65% field-level error rate.
//!
//! THE RULES, stated once (the same text is reproduced in README.md):
//!
//! * A *run* is a maximal span of `[A-Za-z0-9_.]`. Because the characters
//!   either side of a run are outside that class, the leading guard
//!   `(?<![A-Za-z0-9_])(?<![A-Za-z0-9]\.)` holds automatically at every run
//!   start, and can never hold anywhere inside a run. Therefore every match
//!   begins at a run start. This is what makes a single pass per file correct.
//! * The trailing guard `(?![A-Za-z0-9_])(?!\.[A-Za-z0-9])` restricts the
//!   candidate to a prefix of the run that either ends the run, or is followed
//!   inside the run by `..` or `._`.
//! * ID_QUOTED — the literal `"FUNC.NAME"` including both double quotes.
//!   Used for the module grep, `registered_in` and `dispatch_in`. Exact.
//! * ID_BARE — a run equal to `FUNC.NAME`. A superset of ID_QUOTED, so id
//!   occurrence counts are taken from ID_BARE only and never double-counted.
//! * NAME_WORD — surface names of length >= 4, run-boundary matching only.
//! * NAME_GUARDED — surface names of length <= 3, run-boundary matching PLUS
//!   one of four adjacency guards: followed by optional whitespace then `(`;
//!   preceded by `=` and optional whitespace; double-quoted; backtick-quoted.
//! * FORMULA_TOKEN — any surface name (short or long) followed by optional
//!   whitespace then `(`. Used for the fixture half of `fixture_hits`.
//!
//! Matching is case-sensitive throughout, exactly as the Python is.

use std::collections::{BTreeMap, BTreeSet, HashMap};

#[inline]
fn in_run(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
}

#[inline]
fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 0x0b | 0x0c)
}

/// Lexicon of every token this tool looks for, built once.
pub struct Lex {
    /// surface name -> entry index
    pub name_of: HashMap<String, usize>,
    /// function id -> entry index
    pub id_of: HashMap<String, usize>,
    /// per entry: surface name length <= 3
    pub short: Vec<bool>,
}

#[derive(Default)]
pub struct FileHits {
    /// entries whose id appears as the literal `"FUNC.NAME"`
    pub quoted_ids: BTreeSet<usize>,
    /// entry -> number of ID_BARE occurrences
    pub id_occ: BTreeMap<usize, u32>,
    /// entry -> number of NAME occurrences under the entry's name rule
    pub name_occ: BTreeMap<usize, u32>,
    /// entries matched as a formula token `NAME(`
    pub ftok: BTreeSet<usize>,
    /// entry -> byte offsets of every id or name occurrence (only when asked)
    pub offsets: BTreeMap<usize, Vec<usize>>,
}

impl FileHits {
    pub fn mentions(&self, e: usize) -> bool {
        self.id_occ.contains_key(&e) || self.name_occ.contains_key(&e)
    }
    pub fn mention_count(&self, e: usize) -> u32 {
        self.id_occ.get(&e).copied().unwrap_or(0) + self.name_occ.get(&e).copied().unwrap_or(0)
    }
}

fn short_guard(h: &[u8], start: usize, end: usize) -> bool {
    // followed by optional whitespace then '('
    let mut j = end;
    while j < h.len() && is_ws(h[j]) {
        j += 1;
    }
    if j < h.len() && h[j] == b'(' {
        return true;
    }
    // preceded by '=' with optional whitespace between
    let mut k = start;
    while k > 0 && is_ws(h[k - 1]) {
        k -= 1;
    }
    if k > 0 && h[k - 1] == b'=' {
        return true;
    }
    // double-quoted
    if start > 0 && h[start - 1] == b'"' && end < h.len() && h[end] == b'"' {
        return true;
    }
    // backtick-quoted
    if start > 0 && h[start - 1] == b'`' && end < h.len() && h[end] == b'`' {
        return true;
    }
    false
}

fn followed_by_open_paren(h: &[u8], end: usize) -> bool {
    let mut j = end;
    while j < h.len() && is_ws(h[j]) {
        j += 1;
    }
    j < h.len() && h[j] == b'('
}

/// One pass over one file.
pub fn scan(h: &[u8], lex: &Lex, record_offsets: bool) -> FileHits {
    let mut out = FileHits::default();
    let n = h.len();
    let mut i = 0usize;
    while i < n {
        if !in_run(h[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < n && in_run(h[i]) {
            i += 1;
        }
        let end = i;

        // Candidate prefix end positions inside [start, end].
        // Always the whole run; plus any k where run[k] == '.' and run[k+1]
        // is '.' or '_' (the only way the trailing guard can hold mid-run).
        let mut cands: Vec<usize> = Vec::new();
        for k in start..end.saturating_sub(1) {
            if h[k] == b'.' && (h[k + 1] == b'.' || h[k + 1] == b'_') && k > start {
                cands.push(k);
            }
        }
        cands.push(end);

        for &ce in &cands {
            let tok = match std::str::from_utf8(&h[start..ce]) {
                Ok(t) => t,
                Err(_) => continue,
            };
            if tok.is_empty() {
                continue;
            }
            if let Some(&e) = lex.id_of.get(tok) {
                *out.id_occ.entry(e).or_insert(0) += 1;
                if record_offsets {
                    out.offsets.entry(e).or_default().push(start);
                }
                if ce == end
                    && start > 0
                    && h[start - 1] == b'"'
                    && end < n
                    && h[end] == b'"'
                {
                    out.quoted_ids.insert(e);
                }
                continue;
            }
            if let Some(&e) = lex.name_of.get(tok) {
                let ok = if lex.short[e] {
                    short_guard(h, start, ce)
                } else {
                    true
                };
                if ok {
                    *out.name_occ.entry(e).or_insert(0) += 1;
                    if record_offsets {
                        out.offsets.entry(e).or_default().push(start);
                    }
                }
                if followed_by_open_paren(h, ce) {
                    out.ftok.insert(e);
                }
            }
        }
    }
    for v in out.offsets.values_mut() {
        v.sort_unstable();
        v.dedup();
    }
    out
}

/// `#[test]` attribute count. Matches the Python `#\[\s*test\s*\]` exactly:
/// plain `#[test]` only, never `#[tokio::test]`, never `#[test_case(..)]`.
pub fn count_test_attrs(h: &[u8]) -> usize {
    let mut c = 0usize;
    let n = h.len();
    let mut i = 0usize;
    while i + 1 < n {
        if h[i] == b'#' && h[i + 1] == b'[' {
            let mut j = i + 2;
            while j < n && is_ws(h[j]) {
                j += 1;
            }
            if h[j..].starts_with(b"test") {
                let mut k = j + 4;
                while k < n && is_ws(h[k]) {
                    k += 1;
                }
                if k < n && h[k] == b']' {
                    c += 1;
                }
            }
        }
        i += 1;
    }
    c
}

/// Line count: `text.count("\n") + (1 if text and not text.endswith("\n"))`,
/// the same definition the harvest script used.
pub fn count_lines(h: &[u8]) -> usize {
    let nl = h.iter().filter(|&&b| b == b'\n').count();
    if !h.is_empty() && *h.last().unwrap() != b'\n' {
        nl + 1
    } else {
        nl
    }
}

/// For a byte offset, the text of the nearest preceding Markdown ATX header
/// (`^#{1,6}\s`), uppercased. Empty when there is none.
pub fn nearest_atx_header(h: &[u8], offset: usize) -> String {
    let mut best: Option<(usize, usize)> = None; // (line_start, line_end)
    let mut line_start = 0usize;
    let mut i = 0usize;
    while i <= offset && i < h.len() {
        if h[i] == b'\n' {
            line_start = i + 1;
            i += 1;
            continue;
        }
        if i == line_start && h[i] == b'#' {
            let mut k = i;
            while k < h.len() && h[k] == b'#' {
                k += 1;
            }
            let hashes = k - i;
            if (1..=6).contains(&hashes) && k < h.len() && (h[k] == b' ' || h[k] == b'\t') {
                let mut e = k;
                while e < h.len() && h[e] != b'\n' {
                    e += 1;
                }
                if line_start <= offset {
                    best = Some((k, e));
                }
            }
        }
        i += 1;
    }
    match best {
        Some((a, b)) => String::from_utf8_lossy(&h[a..b])
            .trim()
            .to_uppercase(),
        None => String::new(),
    }
}

/// True when the byte offset lies on a line that, left-trimmed, starts with `|`
/// — i.e. inside a Markdown table row.
pub fn on_table_row(h: &[u8], offset: usize) -> bool {
    let mut s = offset.min(h.len().saturating_sub(1));
    while s > 0 && h[s - 1] != b'\n' {
        s -= 1;
    }
    let mut i = s;
    while i < h.len() && (h[i] == b' ' || h[i] == b'\t') {
        i += 1;
    }
    i < h.len() && h[i] == b'|'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex() -> Lex {
        let names = ["ABS", "SUM", "T", "PI", "HARMEAN", "NORM.INV", "LOGNORM.INV", "SUMIF"];
        let mut name_of = HashMap::new();
        let mut id_of = HashMap::new();
        let mut short = Vec::new();
        for (i, n) in names.iter().enumerate() {
            name_of.insert(n.to_string(), i);
            id_of.insert(format!("FUNC.{n}"), i);
            short.push(n.len() <= 3);
        }
        Lex { name_of, id_of, short }
    }

    #[test]
    fn long_name_word_boundaries() {
        let l = lex();
        // SUM must not match inside SUMIF
        let h = scan(b"SUMIF(a)", &l, false);
        assert_eq!(h.name_occ.get(&1), None);
        assert_eq!(h.name_occ.get(&7).copied(), Some(1));
    }

    #[test]
    fn dotted_name_not_matched_as_suffix() {
        let l = lex();
        let h = scan(b"LOGNORM.INV(x)", &l, false);
        assert_eq!(h.name_occ.get(&5), None, "NORM.INV must not match inside LOGNORM.INV");
        assert_eq!(h.name_occ.get(&6).copied(), Some(1));
    }

    #[test]
    fn short_name_needs_a_guard() {
        let l = lex();
        assert_eq!(scan(b"the T function", &l, false).name_occ.get(&2), None);
        assert_eq!(scan(b"T(1)", &l, false).name_occ.get(&2).copied(), Some(1));
        assert_eq!(scan(b"=T", &l, false).name_occ.get(&2).copied(), Some(1));
        assert_eq!(scan(b"\"T\"", &l, false).name_occ.get(&2).copied(), Some(1));
        assert_eq!(scan(b"`T`", &l, false).name_occ.get(&2).copied(), Some(1));
        assert_eq!(scan(b"PI is 3", &l, false).name_occ.get(&3), None);
    }

    #[test]
    fn id_quoted_and_bare() {
        let l = lex();
        let h = scan(b"map(\"FUNC.ABS\", abs); // FUNC.ABS again", &l, false);
        assert!(h.quoted_ids.contains(&0));
        assert_eq!(h.id_occ.get(&0).copied(), Some(2));
        // the id occurrence must not also register as a NAME occurrence
        assert_eq!(h.name_occ.get(&0), None);
    }

    #[test]
    fn id_trailing_guard() {
        let l = lex();
        // FUNC.T must not match inside FUNC.TAN-like longer ids
        let mut name_of = HashMap::new();
        let mut id_of = HashMap::new();
        name_of.insert("T".to_string(), 0);
        id_of.insert("FUNC.T".to_string(), 0);
        id_of.insert("FUNC.T.TEST".to_string(), 1);
        name_of.insert("T.TEST".to_string(), 1);
        let l2 = Lex { name_of, id_of, short: vec![true, false] };
        let h = scan(b"FUNC.T.TEST", &l2, false);
        assert_eq!(h.id_occ.get(&0), None);
        assert_eq!(h.id_occ.get(&1).copied(), Some(1));
        let _ = l;
    }

    #[test]
    fn hyphen_is_a_run_break() {
        let l = lex();
        let h = scan(b"W16-BATCH18-GEO-HARMEAN-20260315", &l, false);
        assert_eq!(h.name_occ.get(&4).copied(), Some(1));
    }

    #[test]
    fn test_attr_counting() {
        assert_eq!(count_test_attrs(b"#[test]\nfn a(){}\n#[ test ]\nfn b(){}"), 2);
        assert_eq!(count_test_attrs(b"#[tokio::test]\n#[test_case(1)]"), 0);
    }

    #[test]
    fn line_counting_matches_python() {
        assert_eq!(count_lines(b""), 0);
        assert_eq!(count_lines(b"a"), 1);
        assert_eq!(count_lines(b"a\n"), 1);
        assert_eq!(count_lines(b"a\nb"), 2);
        assert_eq!(count_lines(b"a\nb\n"), 2);
    }

    #[test]
    fn atx_header_and_table_row() {
        let doc = b"# Title\n\n## Registry Rows\n\n| `A` | x |\n\n## Rules\n\ntext A here\n";
        let off = doc.windows(3).position(|w| w == b"| `").unwrap() + 3;
        assert_eq!(nearest_atx_header(doc, off), "REGISTRY ROWS");
        assert!(on_table_row(doc, off));
        let off2 = doc.windows(4).position(|w| w == b"text").unwrap();
        assert_eq!(nearest_atx_header(doc, off2), "RULES");
        assert!(!on_table_row(doc, off2));
    }
}
