//! efh-battery — runs the fixed `EFH-B1` input battery against the OxFunc reference engine and
//! writes `data/battery/<function_id>.json` (F16) for every Handbook entry.
//!
//! What this tool publishes is **OxFunc's own answers**. No Excel is involved at any point, and
//! there is no oracle of any kind in this pipeline. See `BATTERY.md`.
//!
//! Everything emitted here is mechanical: the battery inputs are fixed and versioned, the argument
//! count for each row is a pure function of the declared `Arity`, and every refusal to call carries
//! a typed reason derived from a declared `FunctionMeta` axis. Nothing is estimated, rounded or
//! carried across functions.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use oxfunc_core::function::{Arity, DeterminismClass, FecDependencyProfile};
use oxfunc_core::function_call::FunctionCallTarget;
use oxfunc_core::functions::surface_dispatch::{eval_surface_value_call, resolve_surface_dispatch_key};
use oxfunc_core::resolver::{
    ReferenceDereferenceRequest, ReferenceResolutionError, ReferenceSystemCapabilities,
    ReferenceSystemProvider,
};
use oxfunc_core::value::{CalcArray, CalcValue, CoreValue, ExcelText, WorksheetErrorCode};

const SCHEMA: &str = "efh.battery/v1";
const BATTERY_ID: &str = "EFH-B1";
const RUNNER_VERSION: &str = "efh-battery/0.1.0";
const LABEL: &str = "OxFunc's own answers. No Excel was involved.";

/// A declared `arity.max` at or above this is treated as unbounded, so no `too-many-args` case
/// exists. OxFunc publishes `usize::MAX` for the genuinely unbounded surfaces; the largest real
/// bound in the catalog is 255.
const UNBOUNDED_ARITY_MAX: usize = 512;

/// Per-call wall-clock budget. A row that exceeds it is published as `not-dispatchable` with the
/// typed reason `no-answer:timed-out-in-reference-engine`; the summary reports how many fired, and
/// a non-zero count is a determinism defect, not a result.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

// ---------------------------------------------------------------------------------------------
// The battery. Twelve rows, fixed order, fixed labels — the order and the labels are the schema
// (FOUNDATION §2.9). Constancy across functions is the whole point: it is what makes two
// functions' answers comparable.
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RowKind {
    /// Every argument position gets the same fixed value; the argument count comes from `Arity`.
    Value(Fixed),
    TooFew,
    TooMany,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Fixed {
    Zero,
    NegativeOne,
    EmptyString,
    BooleanTrue,
    EmptyCell,
    ErrorNa,
    MaxDouble,
    MinSubnormal,
    InlineArray,
    TextNumeral,
    /// Used only by the two arity rows.
    One,
}

const ROWS: [(&str, RowKind); 12] = [
    ("zero", RowKind::Value(Fixed::Zero)),
    ("negative-one", RowKind::Value(Fixed::NegativeOne)),
    ("empty-string", RowKind::Value(Fixed::EmptyString)),
    ("boolean-true", RowKind::Value(Fixed::BooleanTrue)),
    ("empty-range", RowKind::Value(Fixed::EmptyCell)),
    ("error-na", RowKind::Value(Fixed::ErrorNa)),
    ("max-double", RowKind::Value(Fixed::MaxDouble)),
    ("min-subnormal", RowKind::Value(Fixed::MinSubnormal)),
    ("inline-array", RowKind::Value(Fixed::InlineArray)),
    ("text-numeral", RowKind::Value(Fixed::TextNumeral)),
    ("too-few-args", RowKind::TooFew),
    ("too-many-args", RowKind::TooMany),
];

fn fixed_value(f: Fixed) -> CalcValue {
    match f {
        Fixed::Zero => CalcValue::number(0.0),
        Fixed::NegativeOne => CalcValue::number(-1.0),
        Fixed::EmptyString => CalcValue::text(ExcelText::from_interop_assignment("")),
        Fixed::BooleanTrue => CalcValue::logical(true),
        Fixed::EmptyCell => CalcValue::empty(),
        Fixed::ErrorNa => CalcValue::error(WorksheetErrorCode::NA),
        Fixed::MaxDouble => CalcValue::number(f64::MAX),
        Fixed::MinSubnormal => CalcValue::number(f64::from_bits(1)),
        Fixed::InlineArray => CalcValue::array(
            CalcArray::from_rows(vec![
                vec![CalcValue::number(1.0), CalcValue::number(2.0)],
                vec![CalcValue::number(3.0), CalcValue::number(4.0)],
            ])
            .expect("2x2 literal array"),
        ),
        Fixed::TextNumeral => CalcValue::text(ExcelText::from_interop_assignment("2.5")),
        Fixed::One => CalcValue::number(1.0),
    }
}

fn fixed_display(f: Fixed) -> String {
    match f {
        Fixed::Zero => "0".to_string(),
        Fixed::NegativeOne => "-1".to_string(),
        Fixed::EmptyString => "\"\"".to_string(),
        Fixed::BooleanTrue => "TRUE".to_string(),
        Fixed::EmptyCell => "<empty>".to_string(),
        Fixed::ErrorNa => "#N/A".to_string(),
        Fixed::MaxDouble => number_display(f64::MAX),
        Fixed::MinSubnormal => number_display(f64::from_bits(1)),
        Fixed::InlineArray => "{1,2;3,4}".to_string(),
        Fixed::TextNumeral => "\"2.5\"".to_string(),
        Fixed::One => "1".to_string(),
    }
}

// ---------------------------------------------------------------------------------------------
// Resolver. The battery supplies no workbook: every argument is a literal, so no reference is ever
// constructed and `dereference` can only be reached by a function that manufactures one itself.
// ---------------------------------------------------------------------------------------------

struct NoWorkbookResolver;

impl ReferenceSystemProvider for NoWorkbookResolver {
    fn capabilities(&self) -> ReferenceSystemCapabilities {
        ReferenceSystemCapabilities::permissive_local()
    }

    fn dereference(
        &self,
        request: &ReferenceDereferenceRequest,
    ) -> Result<CalcValue, ReferenceResolutionError> {
        Err(ReferenceResolutionError::UnresolvedReference {
            target: request.reference.target().to_string(),
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Outcome rendering.
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Outcome {
    kind: &'static str,
    display: String,
    bits: Option<String>,
}

/// 17 significant decimal digits, always. `{:.16e}` is a 1-digit integral part plus 16 fractional
/// digits, which is exactly 17 significant digits and round-trips every binary64.
fn number_display(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }
    format!("{:.16e}", v)
}

fn number_bits(v: f64) -> String {
    format!("0x{:016x}", v.to_bits())
}

fn error_display(code: WorksheetErrorCode) -> &'static str {
    match code {
        WorksheetErrorCode::Null => "#NULL!",
        WorksheetErrorCode::Div0 => "#DIV/0!",
        WorksheetErrorCode::Value => "#VALUE!",
        WorksheetErrorCode::Ref => "#REF!",
        WorksheetErrorCode::Name => "#NAME?",
        WorksheetErrorCode::Num => "#NUM!",
        WorksheetErrorCode::NA => "#N/A",
        WorksheetErrorCode::Busy => "#BUSY!",
        WorksheetErrorCode::GettingData => "#GETTING_DATA",
        WorksheetErrorCode::Spill => "#SPILL!",
        WorksheetErrorCode::Calc => "#CALC!",
        WorksheetErrorCode::Field => "#FIELD!",
        WorksheetErrorCode::Blocked => "#BLOCKED!",
        WorksheetErrorCode::Connect => "#CONNECT!",
    }
}

fn scalar_cell_display(value: &CalcValue) -> String {
    match value.core() {
        CoreValue::Number(n) => number_display(*n),
        CoreValue::Text(t) => format!("\"{}\"", t.to_string_lossy()),
        CoreValue::Logical(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
        CoreValue::Error(c) => error_display(*c).to_string(),
        CoreValue::Empty => "<empty>".to_string(),
        CoreValue::Missing => "<missing>".to_string(),
        CoreValue::Array(_) => "<nested-array>".to_string(),
        CoreValue::Reference(r) => format!("<reference:{}>", r.target()),
    }
}

fn render_outcome(result: Result<CalcValue, WorksheetErrorCode>) -> Outcome {
    match result {
        Err(code) => Outcome {
            kind: "error",
            display: error_display(code).to_string(),
            bits: None,
        },
        Ok(value) => match value.core() {
            CoreValue::Number(n) => Outcome {
                kind: "number",
                display: number_display(*n),
                bits: Some(number_bits(*n)),
            },
            CoreValue::Text(t) => Outcome {
                kind: "text",
                display: t.to_string_lossy(),
                bits: None,
            },
            CoreValue::Logical(b) => Outcome {
                kind: "boolean",
                display: if *b { "TRUE" } else { "FALSE" }.to_string(),
                bits: None,
            },
            CoreValue::Error(c) => Outcome {
                kind: "error",
                display: error_display(*c).to_string(),
                bits: None,
            },
            CoreValue::Array(a) => {
                let (rows, cols) = {
                    let shape = a.shape();
                    (shape.rows, shape.cols)
                };
                let mut out = String::from("{");
                for r in 0..rows {
                    if r > 0 {
                        out.push(';');
                    }
                    for c in 0..cols {
                        if c > 0 {
                            out.push(',');
                        }
                        match a.get(r, c) {
                            Some(cell) => out.push_str(&scalar_cell_display(cell)),
                            None => out.push_str("<absent>"),
                        }
                    }
                }
                out.push('}');
                Outcome {
                    kind: "array",
                    display: out,
                    bits: None,
                }
            }
            CoreValue::Empty => Outcome {
                kind: "text",
                display: "<empty>".to_string(),
                bits: None,
            },
            CoreValue::Missing => Outcome {
                kind: "text",
                display: "<missing>".to_string(),
                bits: None,
            },
            CoreValue::Reference(r) => Outcome {
                kind: "text",
                display: format!("<reference:{}>", r.target()),
                bits: None,
            },
        },
    }
}

fn refused(reason: &str) -> Outcome {
    Outcome {
        kind: "not-dispatchable",
        display: reason.to_string(),
        bits: None,
    }
}

// ---------------------------------------------------------------------------------------------
// Typed cannot-call reasons — every one derived from a declared axis, never from prose.
// ---------------------------------------------------------------------------------------------

fn fec_token(p: FecDependencyProfile) -> &'static str {
    match p {
        FecDependencyProfile::None => "none",
        FecDependencyProfile::RefOnly => "ref-only",
        FecDependencyProfile::Composite => "composite",
        FecDependencyProfile::LocaleProfile => "locale-profile",
        FecDependencyProfile::ExternalProvider => "external-provider",
        FecDependencyProfile::CallerContext => "caller-context",
        FecDependencyProfile::RandomProvider => "random-provider",
        FecDependencyProfile::TimeProvider => "time-provider",
    }
}

fn determinism_token(d: DeterminismClass) -> &'static str {
    match d {
        DeterminismClass::Deterministic => "deterministic",
        DeterminismClass::PseudoRandom => "pseudo-random",
        DeterminismClass::TimeDependent => "time-dependent",
        DeterminismClass::ExternalEventDependent => "external-event-dependent",
    }
}

/// `None` = the reference engine can be called for this entry.
fn cannot_call_reason(target: &FunctionCallTarget) -> Option<String> {
    let meta = target.function_meta();
    if target.requires_invoker() {
        return Some("cannot-call:requires-callable-argument".to_string());
    }
    if meta.determinism != DeterminismClass::Deterministic {
        return Some(format!(
            "cannot-call:nondeterministic-by-declaration:{}",
            determinism_token(meta.determinism)
        ));
    }
    match meta.surface_fec_dependency_profile {
        FecDependencyProfile::None | FecDependencyProfile::RefOnly => None,
        other => Some(format!(
            "cannot-call:requires-host-facility:{}",
            fec_token(other)
        )),
    }
}

// ---------------------------------------------------------------------------------------------
// Calling.
// ---------------------------------------------------------------------------------------------

fn argument_count_for_value_rows(arity: Arity) -> usize {
    if arity.max == 0 {
        0
    } else {
        std::cmp::max(arity.min, 1).min(arity.max)
    }
}

/// Argument lists longer than this are elided in `input_display` (the count is printed instead), so
/// a 256-argument arity row does not render as a wall of commas. The call itself is unaffected.
const INPUT_DISPLAY_ELIDE_AFTER: usize = 6;

fn input_display(surface: &str, arg_display: &str, argc: usize) -> String {
    let mut s = String::new();
    let _ = write!(s, "{}(", surface);
    let shown = argc.min(INPUT_DISPLAY_ELIDE_AFTER);
    for i in 0..shown {
        if i > 0 {
            s.push_str(", ");
        }
        s.push_str(arg_display);
    }
    if argc > shown {
        s.push_str(", ...");
    }
    s.push(')');
    if argc > shown {
        let _ = write!(s, " [{argc} arguments]");
    }
    s
}

/// Runs one call on a worker thread and renders the result there, so nothing but plain strings
/// crosses the thread boundary. A panic inside the reference engine drops the sender and is
/// published as a typed no-answer, never as a fabricated value.
fn call_rendered(function_id: &str, value: Fixed, argc: usize) -> Outcome {
    let (tx, rx) = mpsc::channel::<Outcome>();
    let id = function_id.to_string();
    let handle = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let args: Vec<CalcValue> = (0..argc).map(|_| fixed_value(value)).collect();
            let resolver = NoWorkbookResolver;
            let result = eval_surface_value_call(&id, &args, &resolver, None, None, None, None);
            let _ = tx.send(render_outcome(result));
        });
    let handle = match handle {
        Ok(h) => h,
        Err(_) => return refused("no-answer:could-not-spawn-worker"),
    };
    match rx.recv_timeout(CALL_TIMEOUT) {
        Ok(outcome) => {
            let _ = handle.join();
            outcome
        }
        Err(mpsc::RecvTimeoutError::Timeout) => refused("no-answer:timed-out-in-reference-engine"),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            refused("no-answer:panicked-in-reference-engine")
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Minimal JSON emission — key order is the schema, so the writer is hand-rolled.
// ---------------------------------------------------------------------------------------------

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ---------------------------------------------------------------------------------------------
// Handbook inputs.
// ---------------------------------------------------------------------------------------------

/// Pulls `"key": "value"` / `"key": true` out of a flat-enough JSON text without a JSON dependency.
/// Only used on `data/functions/*.json`, whose shape this repository generates itself.
fn top_level_string(text: &str, key: &str) -> Option<String> {
    let needle = format!("\n  \"{}\": \"", key);
    let start = text.find(&needle)? + needle.len();
    let rest = &text[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn read_entries(handbook: &Path) -> Vec<(String, String)> {
    let dir = handbook.join("data/functions");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let text = std::fs::read_to_string(&path).expect("read function file");
        let id = top_level_string(&text, "function_id").expect("function_id");
        let surface = top_level_string(&text, "surface_name").expect("surface_name");
        out.push((id, surface));
    }
    out.sort();
    out
}

/// The pinned x87 scope list (see `BATTERY.md` §5 and `derive_x87_scope.py`). One function id per
/// line; `#` comments and blank lines ignored.
fn read_x87_scope(handbook: &Path) -> BTreeSet<String> {
    let path = handbook.join("tools/efh-battery/x87-scope.txt");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read pinned x87 scope {}: {e}", path.display()));
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}

fn git(oxfunc: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(oxfunc)
        .args(args)
        .output()
        .expect("git");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn host_cpu() -> String {
    std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not-detected".to_string())
}

// ---------------------------------------------------------------------------------------------

fn main() {
    let mut args = std::env::args().skip(1);
    let handbook = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "C:/Work/DnaCalc/ExcelFunctionsHandbook".to_string()),
    );
    let oxfunc = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "C:/Work/DnaCalc/OxFunc".to_string()),
    );
    let mode = args.next().unwrap_or_else(|| "battery".to_string());

    if mode == "catalog" {
        // Phase-0 helper: the exact catalog-index -> function_id map, used once to turn the
        // source-derived x87 module scope into the pinned function-id list.
        for (index, meta) in oxfunc_core::xll_export_specs::function_catalog()
            .iter()
            .enumerate()
        {
            println!("{}\t{}", index, meta.function_id);
        }
        return;
    }

    let commit = git(&oxfunc, &["rev-parse", "HEAD"]);
    let porcelain = git(&oxfunc, &["status", "--porcelain"]);
    let tree_clean = porcelain.is_empty();
    if !tree_clean {
        eprintln!(
            "efh-battery: OxFunc working tree is not clean at {commit}; writing nothing.\n{porcelain}"
        );
        std::process::exit(2);
    }

    // Panics inside the reference engine are an outcome, not a crash; keep the console readable.
    std::panic::set_hook(Box::new(|_| {}));

    let entries = read_entries(&handbook);
    let x87_scope = read_x87_scope(&handbook);
    let out_dir = handbook.join("data/battery");
    std::fs::create_dir_all(&out_dir).expect("create data/battery");

    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let cpu = host_cpu();

    let mut reason_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut answered_entries = 0usize;
    let mut refused_entries = 0usize;
    let mut row_kind_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut host_scoped_rows = 0usize;
    let mut host_scoped_entries: BTreeSet<String> = BTreeSet::new();

    for (function_id, surface_name) in &entries {
        let target = FunctionCallTarget::from_function_id(function_id).ok();
        let dispatchable = resolve_surface_dispatch_key(function_id).is_some();

        let entry_reason: Option<String> = if !dispatchable || target.is_none() {
            Some("cannot-call:not-in-reference-catalog".to_string())
        } else {
            cannot_call_reason(target.as_ref().unwrap())
        };

        let arity = target
            .as_ref()
            .map(|t| t.arity())
            .unwrap_or(Arity { min: 0, max: 0 });

        if let Some(reason) = &entry_reason {
            *reason_counts.entry(reason.clone()).or_insert(0) += 1;
            refused_entries += 1;
        } else {
            answered_entries += 1;
        }

        let in_x87_scope = x87_scope.contains(function_id);
        let mut rows: Vec<(String, String, Outcome, bool)> = Vec::with_capacity(12);

        for (label, kind) in ROWS.iter() {
            let (argc, value, no_case): (usize, Fixed, Option<&'static str>) = match kind {
                RowKind::Value(f) => (argument_count_for_value_rows(arity), *f, None),
                RowKind::TooFew => {
                    if arity.min == 0 {
                        (0, Fixed::One, Some("no-such-case:declared-arity-min-is-zero"))
                    } else {
                        (arity.min - 1, Fixed::One, None)
                    }
                }
                RowKind::TooMany => {
                    if arity.max >= UNBOUNDED_ARITY_MAX {
                        (
                            0,
                            Fixed::One,
                            Some("no-such-case:declared-arity-max-is-unbounded"),
                        )
                    } else {
                        (arity.max + 1, Fixed::One, None)
                    }
                }
            };

            let display = input_display(surface_name, &fixed_display(value), argc);
            let outcome = if let Some(reason) = &entry_reason {
                refused(reason)
            } else if let Some(nc) = no_case {
                refused(nc)
            } else {
                let called = call_rendered(function_id, value, argc);
                // An arity row that the engine rejected is reported as such, not as a plain error:
                // the declared arity says the call is inadmissible, and the engine agreed.
                if matches!(kind, RowKind::TooFew | RowKind::TooMany)
                    && called.kind == "error"
                    && !arity.accepts(argc)
                {
                    Outcome {
                        kind: "refused-by-arity",
                        display: called.display,
                        bits: None,
                    }
                } else {
                    called
                }
            };

            let host_scoped = in_x87_scope && outcome.kind == "number";
            if host_scoped {
                host_scoped_rows += 1;
                host_scoped_entries.insert(function_id.clone());
            }
            *row_kind_counts.entry(outcome.kind.to_string()).or_insert(0) += 1;
            rows.push((label.to_string(), display, outcome, host_scoped));
        }

        let mut json = String::new();
        json.push_str("{\n");
        let _ = writeln!(json, "  \"schema\": {},", json_string(SCHEMA));
        let _ = writeln!(json, "  \"function_id\": {},", json_string(function_id));
        let _ = writeln!(json, "  \"surface_name\": {},", json_string(surface_name));
        let _ = writeln!(json, "  \"battery_id\": {},", json_string(BATTERY_ID));
        let _ = writeln!(json, "  \"oxfunc_commit\": {},", json_string(&commit));
        let _ = writeln!(json, "  \"oxfunc_tree_clean\": {},", tree_clean);
        let _ = writeln!(json, "  \"runner_version\": {},", json_string(RUNNER_VERSION));
        json.push_str("  \"host\": {\n");
        let _ = writeln!(json, "    \"arch\": {},", json_string(arch));
        let _ = writeln!(json, "    \"cpu\": {},", json_string(&cpu));
        let _ = writeln!(json, "    \"os\": {}", json_string(os));
        json.push_str("  },\n");
        json.push_str("  \"rows\": [\n");
        for (i, (label, display, outcome, host_scoped)) in rows.iter().enumerate() {
            json.push_str("    {\n");
            let _ = writeln!(json, "      \"label\": {},", json_string(label));
            let _ = writeln!(json, "      \"input_display\": {},", json_string(display));
            let _ = writeln!(json, "      \"outcome_kind\": {},", json_string(outcome.kind));
            let _ = writeln!(
                json,
                "      \"outcome_display\": {},",
                json_string(&outcome.display)
            );
            match &outcome.bits {
                Some(bits) => {
                    let _ = writeln!(json, "      \"outcome_bits\": {},", json_string(bits));
                }
                None => json.push_str("      \"outcome_bits\": null,\n"),
            }
            let _ = writeln!(json, "      \"host_scoped\": {}", host_scoped);
            json.push_str(if i + 1 == rows.len() {
                "    }\n"
            } else {
                "    },\n"
            });
        }
        json.push_str("  ],\n");
        let _ = writeln!(json, "  \"label\": {}", json_string(LABEL));
        json.push_str("}\n");

        let path = out_dir.join(format!("{function_id}.json"));
        std::fs::write(&path, json.as_bytes()).expect("write battery file");
    }

    let _ = std::panic::take_hook();
    println!("entries: {}", entries.len());
    println!("entries called: {answered_entries}");
    println!("entries refused: {refused_entries}");
    for (reason, n) in &reason_counts {
        println!("  {reason}: {n}");
    }
    println!("row outcome kinds:");
    for (kind, n) in &row_kind_counts {
        println!("  {kind}: {n}");
    }
    println!("x87 scope pinned entries: {}", x87_scope.len());
    println!(
        "host_scoped rows: {host_scoped_rows} across {} entries",
        host_scoped_entries.len()
    );
}
