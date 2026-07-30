//! A JSON Schema 2020-12 subset evaluator, plus the dialect-conformance walk that backs
//! T1 acceptance test (a).
//!
//! WHAT ACCEPTANCE TEST (a) ACTUALLY CHECKS, stated precisely so the claim is not read wider than
//! it is: no official metaschema document is fetched or embedded. The check is
//!   1. the file parses as JSON and its root is an object;
//!   2. `$schema` is exactly the 2020-12 dialect URI;
//!   3. every keyword appearing anywhere in the schema tree is either a JSON Schema 2020-12
//!      keyword (the checked-in list below) or a declared EFH annotation keyword;
//!   4. every `type` value is one of the seven JSON Schema types;
//!   5. every `$ref` resolves inside the document;
//!   6. every `pattern` compiles (see regex_lite: an unsupported construct is an error);
//!   7. `enum` is a non-empty array, `required` is an array of strings;
//!   8. EFH structural rules: `required` is a subset of `propertyOrder`, `propertyOrder` is exactly
//!      the declared property set, and every object node with `properties` declares a
//!      `propertyOrder` (FOUNDATION 2.3 line 156 makes key order part of the contract, and JSON
//!      Schema cannot express it, so it is carried as an annotation and checked here).
//!
//! That is a keyword-and-structure conformance walk. It is not a proof that a general-purpose
//! 2020-12 implementation would accept the file.

use serde_json::{Map, Value};
use std::collections::BTreeSet;

use crate::regex_lite::Regex;

pub const DIALECT_2020_12: &str = "https://json-schema.org/draft/2020-12/schema";

/// JSON Schema 2020-12 keywords. Core + applicator + validation + format-annotation + content +
/// meta-data, per the 2020-12 vocabulary set.
pub const JSON_SCHEMA_2020_12_KEYWORDS: &[&str] = &[
    // core
    "$schema", "$id", "$ref", "$anchor", "$dynamicRef", "$dynamicAnchor", "$vocabulary",
    "$comment", "$defs",
    // applicator
    "allOf", "anyOf", "oneOf", "not", "if", "then", "else", "dependentSchemas",
    "prefixItems", "items", "contains", "properties", "patternProperties", "additionalProperties",
    "propertyNames",
    // unevaluated
    "unevaluatedItems", "unevaluatedProperties",
    // validation
    "type", "enum", "const", "multipleOf", "maximum", "exclusiveMaximum", "minimum",
    "exclusiveMinimum", "maxLength", "minLength", "pattern", "maxItems", "minItems", "uniqueItems",
    "maxContains", "minContains", "maxProperties", "minProperties", "required",
    "dependentRequired",
    // format / content
    "format", "contentEncoding", "contentMediaType", "contentSchema",
    // meta-data
    "title", "description", "default", "deprecated", "readOnly", "writeOnly", "examples",
];

/// EFH annotation keywords. JSON Schema 2020-12 permits unknown keywords as annotations; these are
/// the ones this project declares, so an undeclared one is a typo and fails test (a).
pub const EFH_ANNOTATION_KEYWORDS: &[&str] = &[
    // key ORDER is part of the byte-stability contract (FOUNDATION 2.3 line 156) and JSON Schema
    // cannot enforce order, so it is recorded as an annotation and enforced by this tool.
    "propertyOrder",
    // per-family metadata: which organ, who writes it, which FOUNDATION section specifies it.
    "efh",
    // fields FOUNDATION requires that the organ does not carry yet, each with the reason.
    "pendingRequired",
    // fields that must NOT be declared, each with the finding that forbids them.
    "efhForbiddenProperties",
    // a field that exists but may never be parsed by a tool (counts[].figure, guard G-9).
    "efhForbiddenRead",
    // measured divergences between a spec table and the generated organ.
    "efhObservedValuesNotInSpec",
    "efhObservationNote",
];

/// The seven JSON types 2020-12's `type` keyword accepts.
pub const JSON_TYPES: &[&str] =
    &["null", "boolean", "object", "array", "number", "string", "integer"];

#[derive(Debug)]
pub struct Schema {
    pub file_name: String,
    pub root: Value,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub path: String,
    pub message: String,
}

impl Finding {
    fn new(path: &str, message: impl Into<String>) -> Finding {
        Finding { path: path.to_string(), message: message.into() }
    }
}

impl Schema {
    pub fn parse(file_name: &str, text: &str) -> Result<Schema, String> {
        let root: Value = serde_json::from_str(text)
            .map_err(|e| format!("{}: not valid JSON: {}", file_name, e))?;
        if !root.is_object() {
            return Err(format!("{}: schema root is not a JSON object", file_name));
        }
        Ok(Schema { file_name: file_name.to_string(), root })
    }

    pub fn efh_str(&self, key: &str) -> Option<&str> {
        self.root.get("efh")?.get(key)?.as_str()
    }

    // ---------------------------------------------------------------- acceptance test (a)

    /// Dialect and structure conformance. Returns every finding; empty means the file passes.
    pub fn conformance_findings(&self) -> Vec<Finding> {
        let mut out = Vec::new();
        match self.root.get("$schema").and_then(|v| v.as_str()) {
            Some(DIALECT_2020_12) => {}
            Some(other) => out.push(Finding::new(
                "$schema",
                format!("dialect is `{}`, expected `{}`", other, DIALECT_2020_12),
            )),
            None => out.push(Finding::new("$schema", "missing; the dialect must be declared")),
        }
        if self.root.get("$id").and_then(|v| v.as_str()).is_none() {
            out.push(Finding::new("$id", "missing"));
        }
        if self.root.get("title").and_then(|v| v.as_str()).is_none() {
            out.push(Finding::new("title", "missing"));
        }
        if self.root.get("efh").and_then(|v| v.as_object()).is_none() {
            out.push(Finding::new("efh", "missing the EFH family annotation block"));
        } else {
            for req in ["family", "organ", "path_glob", "writer", "spec", "propertyOrderBasis"] {
                if self.efh_str(req).is_none() {
                    out.push(Finding::new(
                        &format!("efh.{}", req),
                        "missing; every schema must name its family, organ, path glob, writer, \
                         specifying section and the basis of its key order",
                    ));
                }
            }
        }
        self.walk_node(&self.root, "#", &mut out, true);
        out
    }

    fn walk_node(&self, node: &Value, path: &str, out: &mut Vec<Finding>, is_root: bool) {
        let obj = match node.as_object() {
            Some(o) => o,
            None => return,
        };
        for (k, v) in obj {
            let known = JSON_SCHEMA_2020_12_KEYWORDS.contains(&k.as_str())
                || EFH_ANNOTATION_KEYWORDS.contains(&k.as_str());
            if !known {
                out.push(Finding::new(
                    &format!("{}/{}", path, k),
                    format!(
                        "`{}` is neither a JSON Schema 2020-12 keyword nor a declared EFH \
                         annotation keyword",
                        k
                    ),
                ));
            }
            if k == "$ref" {
                if let Some(r) = v.as_str() {
                    if self.resolve_ref(r).is_none() {
                        out.push(Finding::new(
                            &format!("{}/$ref", path),
                            format!("`{}` does not resolve inside this document", r),
                        ));
                    }
                } else {
                    out.push(Finding::new(&format!("{}/$ref", path), "$ref is not a string"));
                }
            }
        }
        // type
        if let Some(t) = obj.get("type") {
            match t {
                Value::String(s) => {
                    if !JSON_TYPES.contains(&s.as_str()) {
                        out.push(Finding::new(
                            &format!("{}/type", path),
                            format!("`{}` is not a JSON Schema type", s),
                        ));
                    }
                }
                Value::Array(a) => {
                    for (i, e) in a.iter().enumerate() {
                        match e.as_str() {
                            Some(s) => {
                                if !JSON_TYPES.contains(&s) {
                                    out.push(Finding::new(
                                        &format!("{}/type/{}", path, i),
                                        format!("`{}` is not a JSON Schema type", s),
                                    ));
                                }
                            }
                            None => out.push(Finding::new(
                                &format!("{}/type/{}", path, i),
                                "type array member is not a string",
                            )),
                        }
                    }
                }
                _ => out.push(Finding::new(
                    &format!("{}/type", path),
                    "type must be a string or an array of strings",
                )),
            }
        }
        if let Some(e) = obj.get("enum") {
            match e.as_array() {
                Some(a) if !a.is_empty() => {}
                Some(_) => out.push(Finding::new(&format!("{}/enum", path), "enum is empty")),
                None => out.push(Finding::new(&format!("{}/enum", path), "enum is not an array")),
            }
        }
        if let Some(p) = obj.get("pattern") {
            match p.as_str() {
                Some(s) => {
                    if let Err(e) = Regex::compile(s) {
                        out.push(Finding::new(&format!("{}/pattern", path), e));
                    }
                }
                None => out.push(Finding::new(&format!("{}/pattern", path), "pattern is not a string")),
            }
        }
        // required must be an array of strings, and a subset of propertyOrder
        let required: Vec<String> = match obj.get("required") {
            Some(Value::Array(a)) => {
                let mut v = Vec::new();
                for (i, e) in a.iter().enumerate() {
                    match e.as_str() {
                        Some(s) => v.push(s.to_string()),
                        None => out.push(Finding::new(
                            &format!("{}/required/{}", path, i),
                            "required member is not a string",
                        )),
                    }
                }
                v
            }
            Some(_) => {
                out.push(Finding::new(&format!("{}/required", path), "required is not an array"));
                Vec::new()
            }
            None => Vec::new(),
        };
        // EFH structural rule: an object node with `properties` must declare `propertyOrder`
        // covering exactly those properties, and `required` must be a subset of it.
        if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
            let declared: BTreeSet<&str> = props.keys().map(|k| k.as_str()).collect();
            match obj.get("propertyOrder") {
                Some(Value::Array(order)) => {
                    let mut seen: Vec<&str> = Vec::new();
                    for (i, e) in order.iter().enumerate() {
                        match e.as_str() {
                            Some(s) => {
                                if seen.contains(&s) {
                                    out.push(Finding::new(
                                        &format!("{}/propertyOrder/{}", path, i),
                                        format!("`{}` appears twice", s),
                                    ));
                                }
                                seen.push(s);
                                if !declared.contains(s) {
                                    out.push(Finding::new(
                                        &format!("{}/propertyOrder/{}", path, i),
                                        format!("`{}` is ordered but not declared in properties", s),
                                    ));
                                }
                            }
                            None => out.push(Finding::new(
                                &format!("{}/propertyOrder/{}", path, i),
                                "propertyOrder member is not a string",
                            )),
                        }
                    }
                    for d in &declared {
                        if !seen.contains(d) {
                            out.push(Finding::new(
                                &format!("{}/propertyOrder", path),
                                format!(
                                    "`{}` is declared in properties but has no position; key order \
                                     is part of the byte-stability contract (FOUNDATION 2.3)",
                                    d
                                ),
                            ));
                        }
                    }
                    for r in &required {
                        if !seen.contains(&r.as_str()) {
                            out.push(Finding::new(
                                &format!("{}/required", path),
                                format!("`{}` is required but has no position in propertyOrder", r),
                            ));
                        }
                    }
                }
                Some(_) => out.push(Finding::new(
                    &format!("{}/propertyOrder", path),
                    "propertyOrder is not an array",
                )),
                None => out.push(Finding::new(
                    &format!("{}/propertyOrder", path),
                    "an object node with `properties` must declare `propertyOrder`; FOUNDATION 2.3 \
                     line 156 makes key order part of the byte-stability contract and JSON Schema \
                     cannot express it",
                )),
            }
            for r in &required {
                if !declared.contains(r.as_str()) {
                    let closed = obj.get("additionalProperties") == Some(&Value::Bool(false));
                    if closed {
                        out.push(Finding::new(
                            &format!("{}/required", path),
                            format!(
                                "`{}` is required, additionalProperties is false, and it is not \
                                 declared in properties: nothing can satisfy this schema",
                                r
                            ),
                        ));
                    }
                }
            }
        }
        let _ = is_root;
        // recurse into every subschema position
        for key in ["properties", "patternProperties", "$defs", "dependentSchemas"] {
            if let Some(m) = obj.get(key).and_then(|v| v.as_object()) {
                for (k, v) in m {
                    self.walk_node(v, &format!("{}/{}/{}", path, key, k), out, false);
                }
            }
        }
        for key in ["items", "additionalProperties", "contains", "not", "if", "then", "else",
                    "propertyNames", "unevaluatedItems", "unevaluatedProperties", "contentSchema"]
        {
            if let Some(v) = obj.get(key) {
                if v.is_object() {
                    self.walk_node(v, &format!("{}/{}", path, key), out, false);
                }
            }
        }
        for key in ["allOf", "anyOf", "oneOf", "prefixItems"] {
            if let Some(a) = obj.get(key).and_then(|v| v.as_array()) {
                for (i, v) in a.iter().enumerate() {
                    self.walk_node(v, &format!("{}/{}/{}", path, key, i), out, false);
                }
            }
        }
    }

    pub fn resolve_ref(&self, r: &str) -> Option<&Value> {
        let rest = r.strip_prefix("#/")?;
        let mut cur = &self.root;
        for seg in rest.split('/') {
            let seg = seg.replace("~1", "/").replace("~0", "~");
            cur = cur.get(&seg)?;
        }
        Some(cur)
    }

    // ---------------------------------------------------------------- instance validation

    pub fn validate(&self, instance: &Value) -> Vec<Finding> {
        let mut out = Vec::new();
        self.eval(&self.root, instance, "", &mut out);
        out
    }

    fn eval(&self, schema: &Value, inst: &Value, path: &str, out: &mut Vec<Finding>) {
        let obj = match schema.as_object() {
            Some(o) => o,
            None => return,
        };
        if let Some(r) = obj.get("$ref").and_then(|v| v.as_str()) {
            match self.resolve_ref(r) {
                Some(target) => self.eval(target, inst, path, out),
                None => out.push(Finding::new(path, format!("unresolvable $ref `{}`", r))),
            }
            return;
        }
        if let Some(t) = obj.get("type") {
            let allowed: Vec<&str> = match t {
                Value::String(s) => vec![s.as_str()],
                Value::Array(a) => a.iter().filter_map(|v| v.as_str()).collect(),
                _ => vec![],
            };
            if !allowed.is_empty() && !allowed.iter().any(|a| type_matches(a, inst)) {
                out.push(Finding::new(
                    path,
                    format!("expected type {:?}, found {}", allowed, type_name(inst)),
                ));
                return;
            }
        }
        if let Some(c) = obj.get("const") {
            if inst != c {
                out.push(Finding::new(
                    path,
                    format!("expected const {}, found {}", compact(c), compact(inst)),
                ));
            }
        }
        if let Some(e) = obj.get("enum").and_then(|v| v.as_array()) {
            if !e.contains(inst) {
                out.push(Finding::new(
                    path,
                    format!(
                        "value {} is not in enum [{}]",
                        compact(inst),
                        e.iter().map(compact).collect::<Vec<_>>().join(", ")
                    ),
                ));
            }
        }
        if let Some(branches) = obj.get("oneOf").and_then(|v| v.as_array()) {
            let mut hits = 0usize;
            let mut branch_findings: Vec<Vec<Finding>> = Vec::new();
            for b in branches {
                let mut f = Vec::new();
                self.eval(b, inst, path, &mut f);
                if f.is_empty() {
                    hits += 1;
                }
                branch_findings.push(f);
            }
            if hits == 0 {
                let detail = branch_findings
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        format!(
                            "  branch {}: {}",
                            i,
                            f.iter().map(|x| x.message.clone()).collect::<Vec<_>>().join("; ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                out.push(Finding::new(path, format!("matches no oneOf branch:\n{}", detail)));
            }
        }
        if let Some(branches) = obj.get("anyOf").and_then(|v| v.as_array()) {
            let ok = branches.iter().any(|b| {
                let mut f = Vec::new();
                self.eval(b, inst, path, &mut f);
                f.is_empty()
            });
            if !ok {
                out.push(Finding::new(path, "matches no anyOf branch"));
            }
        }
        if let Some(branches) = obj.get("allOf").and_then(|v| v.as_array()) {
            for b in branches {
                self.eval(b, inst, path, out);
            }
        }

        match inst {
            Value::String(s) => {
                if let Some(p) = obj.get("pattern").and_then(|v| v.as_str()) {
                    match Regex::compile(p) {
                        Ok(re) => {
                            if !re.is_match(s) {
                                out.push(Finding::new(
                                    path,
                                    format!("`{}` does not match pattern `{}`", s, re.source()),
                                ));
                            }
                        }
                        Err(e) => out.push(Finding::new(path, format!("schema pattern error: {}", e))),
                    }
                }
                if let Some(n) = obj.get("minLength").and_then(|v| v.as_u64()) {
                    if (s.chars().count() as u64) < n {
                        out.push(Finding::new(path, format!("shorter than minLength {}", n)));
                    }
                }
                if let Some(n) = obj.get("maxLength").and_then(|v| v.as_u64()) {
                    if (s.chars().count() as u64) > n {
                        out.push(Finding::new(
                            path,
                            format!("longer than maxLength {} ({} chars)", n, s.chars().count()),
                        ));
                    }
                }
            }
            Value::Number(n) => {
                if let Some(m) = obj.get("minimum").and_then(|v| v.as_f64()) {
                    if n.as_f64().unwrap_or(f64::NAN) < m {
                        out.push(Finding::new(path, format!("less than minimum {}", m)));
                    }
                }
                if let Some(m) = obj.get("maximum").and_then(|v| v.as_f64()) {
                    if n.as_f64().unwrap_or(f64::NAN) > m {
                        out.push(Finding::new(path, format!("greater than maximum {}", m)));
                    }
                }
            }
            Value::Array(a) => {
                if let Some(n) = obj.get("minItems").and_then(|v| v.as_u64()) {
                    if (a.len() as u64) < n {
                        out.push(Finding::new(
                            path,
                            format!("has {} items, minItems is {}", a.len(), n),
                        ));
                    }
                }
                if let Some(n) = obj.get("maxItems").and_then(|v| v.as_u64()) {
                    if (a.len() as u64) > n {
                        out.push(Finding::new(
                            path,
                            format!("has {} items, maxItems is {}", a.len(), n),
                        ));
                    }
                }
                if let Some(items) = obj.get("items") {
                    if items.is_object() {
                        for (i, e) in a.iter().enumerate() {
                            self.eval(items, e, &format!("{}[{}]", path, i), out);
                        }
                    }
                }
            }
            Value::Object(m) => {
                if let Some(req) = obj.get("required").and_then(|v| v.as_array()) {
                    for r in req.iter().filter_map(|v| v.as_str()) {
                        if !m.contains_key(r) {
                            out.push(Finding::new(path, format!("missing required key `{}`", r)));
                        }
                    }
                }
                let props = obj.get("properties").and_then(|v| v.as_object());
                if let Some(p) = props {
                    for (k, v) in m {
                        if let Some(sub) = p.get(k) {
                            self.eval(sub, v, &format!("{}.{}", path, k), out);
                        }
                    }
                }
                match obj.get("additionalProperties") {
                    Some(Value::Bool(false)) => {
                        for k in m.keys() {
                            let declared = props.map(|p| p.contains_key(k)).unwrap_or(false);
                            if !declared {
                                out.push(Finding::new(
                                    path,
                                    format!("undeclared key `{}` (additionalProperties is false)", k),
                                ));
                            }
                        }
                    }
                    Some(sub) if sub.is_object() => {
                        for (k, v) in m {
                            let declared = props.map(|p| p.contains_key(k)).unwrap_or(false);
                            if !declared {
                                self.eval(sub, v, &format!("{}.{}", path, k), out);
                            }
                        }
                    }
                    _ => {}
                }
                // EFH key-order contract
                if let Some(order) = obj.get("propertyOrder").and_then(|v| v.as_array()) {
                    let order: Vec<&str> = order.iter().filter_map(|v| v.as_str()).collect();
                    if let Some(msg) = key_order_violation(m, &order) {
                        out.push(Finding::new(path, msg));
                    }
                }
            }
            _ => {}
        }
    }
}

/// Returns a message when the instance's keys are not in the schema's declared relative order.
fn key_order_violation(m: &Map<String, Value>, order: &[&str]) -> Option<String> {
    let mut last_rank: i64 = -1;
    let mut last_key = String::new();
    for k in m.keys() {
        let rank = match order.iter().position(|o| o == k) {
            Some(p) => p as i64,
            None => continue, // undeclared keys are reported by additionalProperties, not here
        };
        if rank < last_rank {
            return Some(format!(
                "key order violates the byte-stability contract: `{}` appears after `{}`, but the \
                 schema orders `{}` first (FOUNDATION 2.3 line 156)",
                k, last_key, k
            ));
        }
        last_rank = rank;
        last_key = k.clone();
    }
    None
}

pub fn type_matches(t: &str, v: &Value) -> bool {
    match t {
        "null" => v.is_null(),
        "boolean" => v.is_boolean(),
        "object" => v.is_object(),
        "array" => v.is_array(),
        "string" => v.is_string(),
        "number" => v.is_number(),
        "integer" => v.as_i64().is_some() || v.as_u64().is_some()
            || v.as_f64().map(|f| f.fract() == 0.0).unwrap_or(false),
        _ => false,
    }
}

pub fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn compact(v: &Value) -> String {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "?".into());
    if s.chars().count() > 60 {
        format!("{}...", s.chars().take(57).collect::<String>())
    } else {
        s
    }
}

/// The set of JSON types a schema node permits, flattened across `type`, `oneOf` and `$ref`.
/// Used by the field-coverage test to decide whether a declared field can carry what the rubric
/// reads out of it.
pub fn declared_types(schema: &Schema, node: &Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    collect_types(schema, node, &mut out, 0);
    out
}

fn collect_types(schema: &Schema, node: &Value, out: &mut BTreeSet<String>, depth: usize) {
    if depth > 8 {
        return;
    }
    let obj = match node.as_object() {
        Some(o) => o,
        None => return,
    };
    if let Some(r) = obj.get("$ref").and_then(|v| v.as_str()) {
        if let Some(t) = schema.resolve_ref(r) {
            collect_types(schema, t, out, depth + 1);
        }
    }
    match obj.get("type") {
        Some(Value::String(s)) => {
            out.insert(s.clone());
        }
        Some(Value::Array(a)) => {
            for e in a.iter().filter_map(|v| v.as_str()) {
                out.insert(e.to_string());
            }
        }
        _ => {}
    }
    for key in ["oneOf", "anyOf"] {
        if let Some(a) = obj.get(key).and_then(|v| v.as_array()) {
            for b in a {
                collect_types(schema, b, out, depth + 1);
            }
        }
    }
    if out.is_empty() {
        if let Some(c) = obj.get("const") {
            out.insert(type_name(c).to_string());
        }
    }
}
