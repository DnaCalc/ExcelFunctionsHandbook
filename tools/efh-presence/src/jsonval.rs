//! Minimal JSON value, parser and deterministic emitter.
//!
//! Deliberately dependency-free. The emitter implements exactly the
//! FOUNDATION §2.3 convention set: UTF-8 without BOM, `\n`, 2-space indent,
//! object keys emitted in insertion order (the caller is responsible for
//! putting them in schema order, or for sorting data-keyed maps ordinally),
//! integers only (no float formatting is ever needed by this tool).

use std::fmt::Write as _;

#[derive(Debug, Clone, PartialEq)]
pub enum J {
    Null,
    Bool(bool),
    Int(i64),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

impl J {
    pub fn s(v: &str) -> J {
        J::Str(v.to_string())
    }
    pub fn arr_str<I: IntoIterator<Item = String>>(it: I) -> J {
        J::Arr(it.into_iter().map(J::Str).collect())
    }
    pub fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::Obj(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, vv)| vv),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            J::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }
    pub fn obj_len(&self) -> usize {
        match self {
            J::Obj(v) => v.len(),
            _ => 0,
        }
    }
    pub fn keys(&self) -> Vec<String> {
        match self {
            J::Obj(v) => v.iter().map(|(k, _)| k.clone()).collect(),
            _ => Vec::new(),
        }
    }
}

// ---------------------------------------------------------------- emitter

pub fn to_string_pretty(v: &J) -> String {
    let mut out = String::with_capacity(4096);
    emit(v, 0, &mut out);
    out.push('\n');
    out
}

fn indent(n: usize, out: &mut String) {
    for _ in 0..n {
        out.push(' ');
    }
}

fn emit(v: &J, depth: usize, out: &mut String) {
    match v {
        J::Null => out.push_str("null"),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Int(i) => {
            let _ = write!(out, "{}", i);
        }
        J::Num(f) => {
            // Not used by this tool's output, but kept total.
            if f.fract() == 0.0 && f.abs() < 9.0e15 {
                let _ = write!(out, "{}", *f as i64);
            } else {
                let _ = write!(out, "{}", f);
            }
        }
        J::Str(s) => emit_str(s, out),
        J::Arr(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (i, e) in a.iter().enumerate() {
                indent(depth + 2, out);
                emit(e, depth + 2, out);
                if i + 1 < a.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(depth, out);
            out.push(']');
        }
        J::Obj(o) => {
            if o.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            for (i, (k, e)) in o.iter().enumerate() {
                indent(depth + 2, out);
                emit_str(k, out);
                out.push_str(": ");
                emit(e, depth + 2, out);
                if i + 1 < o.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(depth, out);
            out.push('}');
        }
    }
}

fn emit_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

// ----------------------------------------------------------------- parser

pub fn parse(bytes: &[u8]) -> Result<J, String> {
    // Strip a UTF-8 BOM if the input file has one; we never write one.
    let b = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };
    let s = std::str::from_utf8(b).map_err(|e| format!("not utf-8: {e}"))?;
    let mut p = P {
        b: s.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return Err(format!("trailing bytes at {}", p.i));
    }
    Ok(v)
}

struct P<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\r' | b'\n') {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn value(&mut self) -> Result<J, String> {
        match self.peek() {
            None => Err("unexpected end".into()),
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(J::Str(self.string()?)),
            Some(b't') => self.lit("true", J::Bool(true)),
            Some(b'f') => self.lit("false", J::Bool(false)),
            Some(b'n') => self.lit("null", J::Null),
            Some(_) => self.number(),
        }
    }
    fn lit(&mut self, w: &str, v: J) -> Result<J, String> {
        if self.b[self.i..].starts_with(w.as_bytes()) {
            self.i += w.len();
            Ok(v)
        } else {
            Err(format!("bad literal at {}", self.i))
        }
    }
    fn object(&mut self) -> Result<J, String> {
        self.i += 1; // {
        let mut out = Vec::new();
        self.ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(J::Obj(out));
        }
        loop {
            self.ws();
            let k = self.string()?;
            self.ws();
            if self.peek() != Some(b':') {
                return Err(format!("expected ':' at {}", self.i));
            }
            self.i += 1;
            self.ws();
            let v = self.value()?;
            out.push((k, v));
            self.ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b'}') => {
                    self.i += 1;
                    return Ok(J::Obj(out));
                }
                _ => return Err(format!("expected ',' or '}}' at {}", self.i)),
            }
        }
    }
    fn array(&mut self) -> Result<J, String> {
        self.i += 1; // [
        let mut out = Vec::new();
        self.ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(J::Arr(out));
        }
        loop {
            self.ws();
            out.push(self.value()?);
            self.ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b']') => {
                    self.i += 1;
                    return Ok(J::Arr(out));
                }
                _ => return Err(format!("expected ',' or ']' at {}", self.i)),
            }
        }
    }
    fn string(&mut self) -> Result<String, String> {
        if self.peek() != Some(b'"') {
            return Err(format!("expected string at {}", self.i));
        }
        self.i += 1;
        let mut s = String::new();
        loop {
            let c = *self.b.get(self.i).ok_or("unterminated string")?;
            self.i += 1;
            match c {
                b'"' => return Ok(s),
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("unterminated escape")?;
                    self.i += 1;
                    match e {
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        b'/' => s.push('/'),
                        b'b' => s.push('\u{08}'),
                        b'f' => s.push('\u{0c}'),
                        b'n' => s.push('\n'),
                        b'r' => s.push('\r'),
                        b't' => s.push('\t'),
                        b'u' => {
                            let h = self.hex4()?;
                            if (0xD800..0xDC00).contains(&h) {
                                if self.b.get(self.i) == Some(&b'\\')
                                    && self.b.get(self.i + 1) == Some(&b'u')
                                {
                                    self.i += 2;
                                    let lo = self.hex4()?;
                                    let cp =
                                        0x10000 + ((h - 0xD800) << 10) + (lo - 0xDC00);
                                    s.push(char::from_u32(cp).ok_or("bad surrogate")?);
                                } else {
                                    s.push('\u{fffd}');
                                }
                            } else {
                                s.push(char::from_u32(h).ok_or("bad code point")?);
                            }
                        }
                        _ => return Err("bad escape".into()),
                    }
                }
                _ => {
                    // copy the raw UTF-8 sequence
                    let start = self.i - 1;
                    let len = utf8_len(c);
                    self.i = start + len;
                    let sl = self
                        .b
                        .get(start..self.i)
                        .ok_or("truncated utf-8 in string")?;
                    s.push_str(std::str::from_utf8(sl).map_err(|_| "bad utf-8")?);
                }
            }
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let sl = self.b.get(self.i..self.i + 4).ok_or("short \\u")?;
        self.i += 4;
        let t = std::str::from_utf8(sl).map_err(|_| "bad \\u")?;
        u32::from_str_radix(t, 16).map_err(|_| "bad \\u".into())
    }
    fn number(&mut self) -> Result<J, String> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        while self
            .peek()
            .map(|c| c.is_ascii_digit() || matches!(c, b'.' | b'e' | b'E' | b'+' | b'-'))
            .unwrap_or(false)
        {
            self.i += 1;
        }
        let t = std::str::from_utf8(&self.b[start..self.i]).map_err(|_| "bad number")?;
        if let Ok(i) = t.parse::<i64>() {
            return Ok(J::Int(i));
        }
        t.parse::<f64>()
            .map(J::Num)
            .map_err(|_| format!("bad number {t:?}"))
    }
}

fn utf8_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_basic() {
        let src = br#"{"a":1,"b":[1,2],"c":{"d":null},"e":"x\ny","f":true}"#;
        let v = parse(src).unwrap();
        assert_eq!(v.get("a"), Some(&J::Int(1)));
        assert_eq!(v.get("e").unwrap().as_str(), Some("x\ny"));
        let out = to_string_pretty(&v);
        assert!(out.ends_with("}\n"));
        assert!(out.contains("  \"a\": 1,\n"));
    }

    #[test]
    fn empty_containers_are_compact() {
        let v = J::Obj(vec![
            ("a".into(), J::Arr(vec![])),
            ("b".into(), J::Obj(vec![])),
        ]);
        assert_eq!(to_string_pretty(&v), "{\n  \"a\": [],\n  \"b\": {}\n}\n");
    }

    #[test]
    fn parses_real_handbook_shape() {
        let src = br#"{"function_id":"FUNC.ABS","artifacts":{"rust_module":"a;b"}}"#;
        let v = parse(src).unwrap();
        assert_eq!(
            v.get("artifacts").unwrap().get("rust_module").unwrap().as_str(),
            Some("a;b")
        );
    }
}
