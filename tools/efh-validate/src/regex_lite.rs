//! A deliberately tiny regular-expression engine, sufficient for the `pattern` keywords that appear
//! in `tools/schemas/*.schema.json` and for nothing else.
//!
//! Why this exists: `efh-validate` has one dependency (serde_json). Pulling in a regex crate to
//! evaluate eight anchored patterns would trade the "builds in seconds" property that FOUNDATION
//! 6 line 1181 puts at the root of the tool graph for very little.
//!
//! The important design choice is that an UNSUPPORTED construct is a hard error, never a silent
//! skip. A `pattern` this engine cannot compile fails the schema, so no schema can quietly stop
//! being enforced.
//!
//! Supported: `^` `$` anchors, literal characters, `\.` `\\` escapes, character classes with ranges
//! and negation (`[a-z0-9]`, `[^abc]`), `.`, alternation groups `(a|b)`, and the quantifiers
//! `*` `+` `?` `{n}` `{n,}` `{n,m}`.
//! Not supported: backreferences, lookaround, named groups, non-greedy quantifiers, `\d`-style
//! shorthand classes, unicode property classes.

#[derive(Debug, Clone)]
enum Atom {
    Char(char),
    Any,
    Class { ranges: Vec<(char, char)>, negated: bool },
    Alt(Vec<Vec<Term>>),
}

#[derive(Debug, Clone)]
struct Term {
    atom: Atom,
    min: usize,
    max: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Regex {
    terms: Vec<Term>,
    anchored_start: bool,
    anchored_end: bool,
    source: String,
}

impl Regex {
    pub fn compile(pattern: &str) -> Result<Regex, String> {
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0usize;
        let anchored_start = chars.first() == Some(&'^');
        if anchored_start {
            i = 1;
        }
        let (terms, next) = parse_seq(&chars, i, false)?;
        i = next;
        let mut anchored_end = false;
        if i < chars.len() && chars[i] == '$' {
            anchored_end = true;
            i += 1;
        }
        if i != chars.len() {
            return Err(format!(
                "regex_lite cannot compile `{}`: unconsumed input at char {}",
                pattern, i
            ));
        }
        Ok(Regex { terms, anchored_start, anchored_end, source: pattern.to_string() })
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        let s: Vec<char> = haystack.chars().collect();
        let starts: Vec<usize> = if self.anchored_start { vec![0] } else { (0..=s.len()).collect() };
        for st in starts {
            if match_seq(&self.terms, &s, st, self.anchored_end) {
                return true;
            }
        }
        false
    }
}

fn parse_seq(chars: &[char], mut i: usize, in_group: bool) -> Result<(Vec<Term>, usize), String> {
    let mut out: Vec<Term> = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c == '$' && !in_group {
            break;
        }
        if in_group && (c == ')' || c == '|') {
            break;
        }
        let atom = match c {
            '\\' => {
                i += 1;
                if i >= chars.len() {
                    return Err("regex_lite: trailing backslash".to_string());
                }
                let e = chars[i];
                if e.is_ascii_alphanumeric() {
                    return Err(format!(
                        "regex_lite: shorthand escape `\\{}` is not supported; write an explicit class",
                        e
                    ));
                }
                i += 1;
                Atom::Char(e)
            }
            '.' => {
                i += 1;
                Atom::Any
            }
            '[' => {
                let (a, n) = parse_class(chars, i)?;
                i = n;
                a
            }
            '(' => {
                let (a, n) = parse_group(chars, i)?;
                i = n;
                a
            }
            ')' | '|' => {
                return Err(format!("regex_lite: unexpected `{}` outside a group", c));
            }
            '*' | '+' | '?' | '{' => {
                return Err(format!("regex_lite: quantifier `{}` with nothing to repeat", c));
            }
            other => {
                i += 1;
                Atom::Char(other)
            }
        };
        let (min, max, n) = parse_quantifier(chars, i)?;
        i = n;
        out.push(Term { atom, min, max });
    }
    Ok((out, i))
}

fn parse_class(chars: &[char], mut i: usize) -> Result<(Atom, usize), String> {
    debug_assert_eq!(chars[i], '[');
    i += 1;
    let negated = i < chars.len() && chars[i] == '^';
    if negated {
        i += 1;
    }
    let mut ranges: Vec<(char, char)> = Vec::new();
    let mut closed = false;
    while i < chars.len() {
        if chars[i] == ']' {
            i += 1;
            closed = true;
            break;
        }
        let lo = if chars[i] == '\\' {
            i += 1;
            if i >= chars.len() {
                return Err("regex_lite: trailing backslash in class".to_string());
            }
            let c = chars[i];
            i += 1;
            c
        } else {
            let c = chars[i];
            i += 1;
            c
        };
        if i + 1 < chars.len() && chars[i] == '-' && chars[i + 1] != ']' {
            let hi = chars[i + 1];
            i += 2;
            ranges.push((lo, hi));
        } else {
            ranges.push((lo, lo));
        }
    }
    if !closed {
        return Err("regex_lite: unterminated character class".to_string());
    }
    Ok((Atom::Class { ranges, negated }, i))
}

fn parse_group(chars: &[char], mut i: usize) -> Result<(Atom, usize), String> {
    debug_assert_eq!(chars[i], '(');
    i += 1;
    if i < chars.len() && chars[i] == '?' {
        return Err("regex_lite: `(?...)` group extensions are not supported".to_string());
    }
    let mut branches: Vec<Vec<Term>> = Vec::new();
    loop {
        let (seq, n) = parse_seq(chars, i, true)?;
        branches.push(seq);
        i = n;
        if i >= chars.len() {
            return Err("regex_lite: unterminated group".to_string());
        }
        match chars[i] {
            '|' => i += 1,
            ')' => {
                i += 1;
                break;
            }
            other => return Err(format!("regex_lite: unexpected `{}` in group", other)),
        }
    }
    Ok((Atom::Alt(branches), i))
}

fn parse_quantifier(chars: &[char], mut i: usize) -> Result<(usize, Option<usize>, usize), String> {
    if i >= chars.len() {
        return Ok((1, Some(1), i));
    }
    let (min, max) = match chars[i] {
        '*' => {
            i += 1;
            (0usize, None)
        }
        '+' => {
            i += 1;
            (1usize, None)
        }
        '?' => {
            i += 1;
            (0usize, Some(1usize))
        }
        '{' => {
            let mut j = i + 1;
            let mut lo = String::new();
            while j < chars.len() && chars[j].is_ascii_digit() {
                lo.push(chars[j]);
                j += 1;
            }
            if lo.is_empty() {
                return Ok((1, Some(1), i));
            }
            let lo_n: usize = lo.parse().map_err(|_| "regex_lite: bad {n}".to_string())?;
            if j < chars.len() && chars[j] == '}' {
                i = j + 1;
                (lo_n, Some(lo_n))
            } else if j < chars.len() && chars[j] == ',' {
                j += 1;
                let mut hi = String::new();
                while j < chars.len() && chars[j].is_ascii_digit() {
                    hi.push(chars[j]);
                    j += 1;
                }
                if j >= chars.len() || chars[j] != '}' {
                    return Err("regex_lite: unterminated {n,m}".to_string());
                }
                i = j + 1;
                if hi.is_empty() {
                    (lo_n, None)
                } else {
                    (lo_n, Some(hi.parse::<usize>().map_err(|_| "regex_lite: bad {n,m}".to_string())?))
                }
            } else {
                return Err("regex_lite: unterminated {n}".to_string());
            }
        }
        _ => (1usize, Some(1usize)),
    };
    if chars.get(i) == Some(&'?') {
        return Err("regex_lite: non-greedy quantifiers are not supported".to_string());
    }
    Ok((min, max, i))
}

fn atom_match(atom: &Atom, s: &[char], pos: usize) -> Vec<usize> {
    match atom {
        Atom::Char(c) => {
            if pos < s.len() && s[pos] == *c {
                vec![pos + 1]
            } else {
                vec![]
            }
        }
        Atom::Any => {
            if pos < s.len() {
                vec![pos + 1]
            } else {
                vec![]
            }
        }
        Atom::Class { ranges, negated } => {
            if pos >= s.len() {
                return vec![];
            }
            let c = s[pos];
            let hit = ranges.iter().any(|(lo, hi)| c >= *lo && c <= *hi);
            if hit != *negated {
                vec![pos + 1]
            } else {
                vec![]
            }
        }
        Atom::Alt(branches) => {
            let mut ends = Vec::new();
            for b in branches {
                collect_seq_ends(b, s, pos, &mut ends);
            }
            ends.sort_unstable();
            ends.dedup();
            ends
        }
    }
}

fn collect_seq_ends(terms: &[Term], s: &[char], pos: usize, out: &mut Vec<usize>) {
    if terms.is_empty() {
        out.push(pos);
        return;
    }
    let (t, rest) = terms.split_first().unwrap();
    let mut frontier = vec![pos];
    let mut count = 0usize;
    // consume the mandatory `min` repetitions first
    while count < t.min {
        let mut next = Vec::new();
        for p in &frontier {
            next.extend(atom_match(&t.atom, s, *p));
        }
        if next.is_empty() {
            return;
        }
        next.sort_unstable();
        next.dedup();
        frontier = next;
        count += 1;
    }
    for p in &frontier {
        collect_seq_ends(rest, s, *p, out);
    }
    // then the optional repetitions
    loop {
        if let Some(m) = t.max {
            if count >= m {
                break;
            }
        }
        let mut next = Vec::new();
        for p in &frontier {
            next.extend(atom_match(&t.atom, s, *p));
        }
        next.sort_unstable();
        next.dedup();
        // a zero-width repeat would loop forever
        if next.is_empty() || next == frontier {
            break;
        }
        frontier = next;
        count += 1;
        for p in &frontier {
            collect_seq_ends(rest, s, *p, out);
        }
    }
}

fn match_seq(terms: &[Term], s: &[char], pos: usize, anchored_end: bool) -> bool {
    let mut ends = Vec::new();
    collect_seq_ends(terms, s, pos, &mut ends);
    if anchored_end {
        ends.iter().any(|e| *e == s.len())
    } else {
        !ends.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Regex;

    fn m(p: &str, s: &str) -> bool {
        Regex::compile(p).unwrap().is_match(s)
    }

    #[test]
    fn patterns_used_by_the_schemas() {
        assert!(m("^EV-[a-z-]+-[0-9]{4}$", "EV-live-verification-0007"));
        assert!(!m("^EV-[a-z-]+-[0-9]{4}$", "EV-live-verification-7"));
        assert!(!m("^EV-[a-z-]+-[0-9]{4}$", "ev-live-verification-0007"));
        assert!(m("^(FUNC|OP)\\.", "FUNC.PMT"));
        assert!(m("^(FUNC|OP)\\.", "OP.MULTIPLY"));
        assert!(!m("^(FUNC|OP)\\.", "XFUNC.PMT"));
        assert!(m("^[0-9a-f]{40}$", "473efa37db60e565ce04241d7cedf74fb3227777"));
        assert!(!m("^[0-9a-f]{40}$", "473efa3"));
        assert!(m("^L[1-9]$", "L9"));
        assert!(!m("^L[1-9]$", "L0"));
        assert!(!m("^L[1-9]$", "L10"));
        assert!(m("^OP-[0-9]{3}$", "OP-004"));
        assert!(m("^WK-", "WK-AS-1964"));
        assert!(m("^PG-", "PG-lossy-gamma"));
        assert!(m("^OP\\.", "OP.ADD"));
        assert!(!m("^OP\\.", "FUNC.ADD"));
    }

    #[test]
    fn unsupported_constructs_are_hard_errors() {
        assert!(Regex::compile("\\d+").is_err());
        assert!(Regex::compile("(?:a)").is_err());
        assert!(Regex::compile("a+?").is_err());
        assert!(Regex::compile("[a-z").is_err());
    }
}
