use serde_json::Value;
use std::path::{Component, Path, PathBuf};

pub const RECONCILIATION: &str = "550 entries = 527 functions + 23 operators = 534 published Excel rows + 7 split byte-variant rows + 9 added functions";
pub const ENTRIES: usize = 550;
pub const FUNCTIONS: usize = 527;
pub const PUBLISHED_ROWS_AND_ADDED: usize = 543;
pub const SOURCE: &str = "https://github.com/DnaCalc/ExcelFunctionsHandbook";

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
pub fn s(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
pub fn read(p: &Path) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}
pub fn json(p: &Path) -> Result<Value, String> {
    serde_json::from_str(&read(p)?).map_err(|e| format!("{}: {e}", p.display()))
}
pub fn write(root: &Path, path: &str, text: &str) -> Result<(), String> {
    let p = root.join(path);
    std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(&p, text).map_err(|e| format!("{}: {e}", p.display()))
}
pub fn files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(root).map_err(|e| format!("{}: {e}", root.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_symlink() {
            return Err(format!("symlink input: {}", entry.path().display()));
        }
        if entry.path().is_dir() {
            paths.extend(files(&entry.path())?);
        } else {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}
pub fn normalize(p: &Path) -> Option<PathBuf> {
    let mut result = PathBuf::new();
    for c in p.components() {
        match c {
            Component::Normal(x) => result.push(x),
            Component::ParentDir => {
                if !result.pop() {
                    return None;
                }
            }
            Component::CurDir => (),
            _ => return None,
        }
    }
    Some(result)
}
pub fn relative(from: &str, to: &str) -> String {
    let a: Vec<_> = from.split('/').collect();
    let b: Vec<_> = to.split('/').collect();
    let dirs = &a[..a.len() - 1];
    let common = dirs.iter().zip(&b).take_while(|(x, y)| x == y).count();
    format!(
        "{}{}",
        "../".repeat(dirs.len() - common),
        b[common..].join("/")
    )
}
pub fn slug(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}
