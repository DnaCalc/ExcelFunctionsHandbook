//! Loading `tools/schemas/*.schema.json`, resolving a dotted field path inside a schema, and
//! routing an instance file path to the schema that governs it.

use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::json_schema::Schema;

pub struct Registry {
    pub repo_root: PathBuf,
    pub schema_dir: PathBuf,
    pub schemas: BTreeMap<String, Schema>,
}

impl Registry {
    pub fn load(repo_root: &Path) -> Result<Registry, String> {
        let schema_dir = repo_root.join("tools").join("schemas");
        let mut names: Vec<PathBuf> = std::fs::read_dir(&schema_dir)
            .map_err(|e| format!("cannot read {}: {}", schema_dir.display(), e))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.to_string_lossy().ends_with(".schema.json"))
            .collect();
        names.sort();
        let mut schemas = BTreeMap::new();
        for p in names {
            let file_name = p.file_name().unwrap().to_string_lossy().to_string();
            let text = std::fs::read_to_string(&p)
                .map_err(|e| format!("cannot read {}: {}", p.display(), e))?;
            let s = Schema::parse(&file_name, &text)?;
            schemas.insert(file_name, s);
        }
        if schemas.is_empty() {
            return Err(format!("no *.schema.json found in {}", schema_dir.display()));
        }
        Ok(Registry { repo_root: repo_root.to_path_buf(), schema_dir, schemas })
    }

    pub fn get(&self, file_name: &str) -> Option<&Schema> {
        self.schemas.get(file_name)
    }

    /// Route a repo-relative, forward-slashed path to the schema whose `efh.path_glob` matches.
    pub fn route(&self, rel_path: &str) -> Option<&Schema> {
        let mut hits: Vec<&Schema> = Vec::new();
        for s in self.schemas.values() {
            if let Some(g) = s.efh_str("path_glob") {
                if glob_match(g, rel_path) {
                    hits.push(s);
                }
            }
        }
        // Prefer the most specific glob (fewest wildcards, then longest literal prefix).
        hits.sort_by_key(|s| {
            let g = s.efh_str("path_glob").unwrap_or("");
            (g.matches('*').count(), usize::MAX - g.len())
        });
        hits.into_iter().next()
    }
}

/// Segment-wise glob supporting `*` (no `/`) and `**` (any number of segments).
pub fn glob_match(pattern: &str, path: &str) -> bool {
    let p: Vec<&str> = pattern.split('/').collect();
    let t: Vec<&str> = path.split('/').collect();
    seg_match(&p, &t)
}

fn seg_match(p: &[&str], t: &[&str]) -> bool {
    if p.is_empty() {
        return t.is_empty();
    }
    if p[0] == "**" {
        for i in 0..=t.len() {
            if seg_match(&p[1..], &t[i..]) {
                return true;
            }
        }
        return false;
    }
    if t.is_empty() {
        return false;
    }
    if !one_seg_match(p[0], t[0]) {
        return false;
    }
    seg_match(&p[1..], &t[1..])
}

fn one_seg_match(pat: &str, seg: &str) -> bool {
    if !pat.contains('*') {
        return pat == seg;
    }
    let parts: Vec<&str> = pat.split('*').collect();
    let mut pos = 0usize;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i == 0 {
            if !seg[pos..].starts_with(part) {
                return false;
            }
            pos += part.len();
        } else if i == parts.len() - 1 {
            if !seg[pos..].ends_with(part) {
                return false;
            }
        } else {
            match seg[pos..].find(part) {
                Some(k) => pos += k + part.len(),
                None => return false,
            }
        }
    }
    true
}

/// Resolve a dotted path against a schema's declared properties.
///
/// Segment forms:
///   `name`     -> properties.name
///   `name[]`   -> properties.name.items
///   `<any>`    -> additionalProperties (the schema for data-keyed members)
///
/// `$ref`, `oneOf` and `anyOf` are transparently descended, so `oracle.builds` resolves even
/// though `oracle` is `oneOf: [{$ref: ...}, {type: null}]`.
pub fn resolve_path<'a>(
    schema: &'a Schema,
    path: &str,
) -> Result<&'a Value, String> {
    let mut node: &Value = &schema.root;
    for raw in path.split('.') {
        let (key, into_items) = match raw.strip_suffix("[]") {
            Some(k) => (k, true),
            None => (raw, false),
        };
        node = if key == "<any>" {
            descend_additional(schema, node)
                .ok_or_else(|| format!("`{}`: no additionalProperties schema at `{}`", path, raw))?
        } else {
            descend_property(schema, node, key)
                .ok_or_else(|| format!("`{}`: no declared property `{}`", path, key))?
        };
        if into_items {
            node = descend_items(schema, node)
                .ok_or_else(|| format!("`{}`: `{}` declares no `items` schema", path, key))?;
        }
    }
    Ok(node)
}

fn deref<'a>(schema: &'a Schema, node: &'a Value) -> &'a Value {
    let mut cur = node;
    for _ in 0..8 {
        match cur.get("$ref").and_then(|v| v.as_str()) {
            Some(r) => match schema.resolve_ref(r) {
                Some(t) => cur = t,
                None => return cur,
            },
            None => return cur,
        }
    }
    cur
}

fn descend_property<'a>(schema: &'a Schema, node: &'a Value, key: &str) -> Option<&'a Value> {
    let node = deref(schema, node);
    if let Some(p) = node.get("properties").and_then(|v| v.as_object()) {
        if let Some(v) = p.get(key) {
            return Some(v);
        }
    }
    for combinator in ["oneOf", "anyOf", "allOf"] {
        if let Some(a) = node.get(combinator).and_then(|v| v.as_array()) {
            for b in a {
                if let Some(v) = descend_property(schema, b, key) {
                    return Some(v);
                }
            }
        }
    }
    None
}

fn descend_items<'a>(schema: &'a Schema, node: &'a Value) -> Option<&'a Value> {
    let node = deref(schema, node);
    if let Some(v) = node.get("items") {
        if v.is_object() {
            return Some(v);
        }
    }
    for combinator in ["oneOf", "anyOf", "allOf"] {
        if let Some(a) = node.get(combinator).and_then(|v| v.as_array()) {
            for b in a {
                if let Some(v) = descend_items(schema, b) {
                    return Some(v);
                }
            }
        }
    }
    None
}

fn descend_additional<'a>(schema: &'a Schema, node: &'a Value) -> Option<&'a Value> {
    let node = deref(schema, node);
    if let Some(v) = node.get("additionalProperties") {
        if v.is_object() {
            return Some(v);
        }
    }
    for combinator in ["oneOf", "anyOf", "allOf"] {
        if let Some(a) = node.get(combinator).and_then(|v| v.as_array()) {
            for b in a {
                if let Some(v) = descend_additional(schema, b) {
                    return Some(v);
                }
            }
        }
    }
    None
}

/// Collect every declared property NAME anywhere in a schema tree. Used by acceptance test (c).
pub fn all_declared_property_names(node: &Value, out: &mut Vec<String>) {
    let obj = match node.as_object() {
        Some(o) => o,
        None => return,
    };
    if let Some(p) = obj.get("properties").and_then(|v| v.as_object()) {
        for (k, v) in p {
            out.push(k.clone());
            all_declared_property_names(v, out);
        }
    }
    for key in ["$defs", "patternProperties", "dependentSchemas"] {
        if let Some(m) = obj.get(key).and_then(|v| v.as_object()) {
            for v in m.values() {
                all_declared_property_names(v, out);
            }
        }
    }
    for key in ["items", "additionalProperties", "contains", "not", "if", "then", "else",
                "propertyNames", "unevaluatedItems", "unevaluatedProperties"]
    {
        if let Some(v) = obj.get(key) {
            if v.is_object() {
                all_declared_property_names(v, out);
            }
        }
    }
    for key in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(a) = obj.get(key).and_then(|v| v.as_array()) {
            for v in a {
                all_declared_property_names(v, out);
            }
        }
    }
}

/// Find the repository root: the nearest ancestor of `start` that contains `tools/schemas`.
pub fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let mut cur = Some(start.to_path_buf());
    while let Some(p) = cur {
        if p.join("tools").join("schemas").is_dir() {
            return Some(p);
        }
        cur = p.parent().map(|q| q.to_path_buf());
    }
    None
}
