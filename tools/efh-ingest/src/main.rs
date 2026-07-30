//! efh-ingest v0.2: builds the Handbook's `data/` projections from OxFunc truth surfaces.
//!
//! Two "as of" bases, never fused (FOUNDATION §2.4 delta D-8, finding A3-S12):
//!
//!   * **Row spine** — `OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv`, snapshot generation
//!     2026-04-02, source commit `87ef585`, source tree state **dirty**. Those bytes cannot be
//!     regenerated from any commit, because the tree they were generated from was dirty.
//!   * **Live registry** — `oxfunc_core::registry::builtin_registry()` at the commit this binary
//!     is built against, with a clean tree (the publication gate below enforces the clean tree).
//!
//! Every emitted file carries a per-field-group `vintage` map naming which of those bases (or
//! which lane CSV) each group came from, so no page can imply a single date for the whole file.
//!
//! OxFunc is read read-only. Output is deterministic (byte-stable for a fixed OxFunc source
//! state) and carries no wall clock.

use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use oxfunc_core::function::{
    ArgPreparationProfile, CoercionLiftProfile, DeterminismClass, ErrorCollapseProfile,
    FecDependencyProfile, FunctionMeta, HostInteractionClass, KernelSignatureClass,
    LiftBroadcastProfile, NonFinite, PrecisionRoundingProfile, ThreadSafetyClass, VolatilityClass,
};
use oxfunc_core::functions::excel_numeric::ArgDomainGuard;
use oxfunc_core::registry::RichValueUsage;

// ---------------------------------------------------------------- F1 output shape

#[derive(Serialize)]
struct FunctionDoc {
    function_id: String,
    surface_name: String,
    /// The published export row this entry came from. Differs from `function_id`
    /// only for entries split out of a combined byte-variant documentation row
    /// (e.g. `FUNC.FIND` and `FUNC.FINDB` both come from `FUNC.FIND, FINDB`).
    published_row_id: String,
    entry_kind: String,
    category: Option<String>,
    admission: Admission,
    metadata_status: Option<String>,
    xlcall: Option<Xlcall>,
    arity: ArityDoc,
    classification: BTreeMap<String, String>,
    axes: Option<Axes>,
    axis_provenance: BTreeMap<String, AxisProvenance>,
    signature: Option<SignatureDoc>,
    signature_placeholder: bool,
    descriptions: Descriptions,
    docs: Option<DocsLink>,
    localized_names: Vec<LocalizedName>,
    artifacts: Option<Artifacts>,
    version_marker: Option<String>,
    special_interface_kind: Option<String>,
    admission_interface_kind: Option<String>,
    registry_backed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_integrity: Option<TextIntegrity>,
    oxfunc_tree_clean: bool,
    vintage: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct Admission {
    label: String,
    reason: Option<String>,
}

#[derive(Serialize)]
struct Xlcall {
    symbol: String,
    code: String,
}

/// D-2: `max` is a JSON number, the string `"unbounded"` for `usize::MAX`, or `null`.
#[derive(Serialize)]
struct ArityDoc {
    min: JsonValue,
    max: JsonValue,
}

#[derive(Serialize)]
struct Axes {
    lift_broadcast_profile: String,
    precision_rounding_profile: String,
    real_result_policy: String,
    axes_version: String,
    semantic_kernel_version: String,
    arg_admission_version: String,
}

/// D-3 + D-4.
#[derive(Serialize)]
struct AxisProvenance {
    provenance_class: &'static str,
    oxfunc_tier: &'static str,
}

#[derive(Serialize)]
struct SignatureDoc {
    display: String,
    /// D-5: `null` where the flag is purely arity-implied over an empty parameter list —
    /// i.e. a derivation artifact of `registry.rs::signature_from_seed`, not a seeded fact.
    trailing_repeats: Option<bool>,
    parameters: Vec<ParameterDoc>,
}

#[derive(Serialize)]
struct ParameterDoc {
    name: String,
    optional: bool,
    repeats: bool,
    description: Option<String>,
}

#[derive(Serialize)]
struct Descriptions {
    short: Option<String>,
    /// D-6.
    short_source: &'static str,
    long: Option<String>,
}

#[derive(Serialize)]
struct DocsLink {
    microsoft_url: String,
    english_description: Option<String>,
}

#[derive(Serialize)]
struct LocalizedName {
    locale: String,
    name: String,
}

#[derive(Serialize)]
struct Artifacts {
    contract_row_id: Option<String>,
    lean_module: Option<String>,
    rust_module: Option<String>,
    status: Option<String>,
}

/// D-7: the three mechanical fields only. `suspected_lost` / `verified` are inferences and
/// belong to `content/projection-gaps.json` (F12), which this tool does not write.
#[derive(Serialize)]
struct TextIntegrity {
    field: &'static str,
    observed: String,
    contains_lossy_substitution: bool,
}

// ---------------------------------------------------------------- F0 index shape

#[derive(Serialize)]
struct IndexDoc {
    provenance: Provenance,
    vintage_states: BTreeMap<String, VintageState>,
    counts: Counts,
    rows: Vec<IndexRow>,
}

#[derive(Serialize)]
struct Provenance {
    export_snapshot_id: String,
    export_snapshot_generation: String,
    export_source_commit: String,
    export_source_tree_state: String,
    oxfunc_commit_at_ingest: String,
    oxfunc_commit_at_ingest_full: String,
    oxfunc_tree_clean: bool,
    registry_snapshot_family: String,
    registry_generation: u64,
    registry_content_fingerprint: String,
    tool: String,
}

/// One resolved "as of" basis. Every `vintage` value in every F1 file is a key of this map.
#[derive(Serialize)]
struct VintageState {
    basis: &'static str,
    source: String,
    as_of: String,
    source_commit: String,
    source_tree_state: String,
    regenerable_from_named_commit: bool,
}

#[derive(Serialize)]
struct Counts {
    total_entries: usize,
    published_rows: usize,
    split_combined_rows: usize,
    functions: usize,
    operators: usize,
    registry_backed: usize,
    export_only: usize,
    deferred: usize,
    placeholder_signatures: usize,
    with_docs_link: usize,
    with_localized_names: usize,
    classification_from_live_registry: usize,
    classification_from_export_snapshot: usize,
    classification_empty: usize,
    arity_from_live_registry: usize,
    arity_from_export_snapshot: usize,
    arity_max_unbounded: usize,
    trailing_repeats_null: usize,
    text_integrity_present: usize,
    short_microsoft_english_verbatim: usize,
    short_differs_from_english_description: usize,
    short_no_english_description_to_compare: usize,
    short_absent: usize,
}

#[derive(Serialize)]
struct IndexRow {
    function_id: String,
    surface_name: String,
    entry_kind: String,
    category: Option<String>,
    admission: String,
}

// ---------------------------------------------------------------- F3 vocabulary shape

#[derive(Serialize)]
struct VocabularyDoc {
    schema: &'static str,
    oxfunc_commit: String,
    oxfunc_tree_clean: bool,
    catalog_size: usize,
    extraction: Extraction,
    axes: Vec<AxisDoc>,
}

#[derive(Serialize)]
struct Extraction {
    variant_source: &'static str,
    line_span_rule: &'static str,
    documented_in_source_rule: &'static str,
    defaultable_rule: &'static str,
    oxfunc_tier_rule: &'static str,
    occupancy_population: String,
    unset_occupancy_meaning: &'static str,
    modal_share_rule: &'static str,
    exemplars_rule: &'static str,
    fields_not_emitted: Vec<&'static str>,
}

#[derive(Serialize)]
struct AxisDoc {
    key: &'static str,
    declared_kind: &'static str,
    enum_name: &'static str,
    /// The struct field on `RegistryFunctionMeta` / `FunctionMeta` this axis reads, or the
    /// published registry-metadata CSV column, whichever the tier rule matched on.
    backing_field: &'static str,
    source_path: String,
    defaultable: bool,
    default_value: Option<String>,
    oxfunc_tier: &'static str,
    documented_in_source: bool,
    variants: Vec<VariantDoc>,
    unset_occupancy: usize,
    modal_share: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_value_occupancy: Option<Vec<PublishedValueDoc>>,
}

#[derive(Serialize)]
struct VariantDoc {
    value: String,
    occupancy: usize,
    exemplars: Vec<String>,
}

#[derive(Serialize)]
struct PublishedValueDoc {
    value: String,
    occupancy: usize,
}

// ---------------------------------------------------------------- small helpers

fn none_if_empty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn read_csv(path: &Path) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(&raw);
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(raw.as_bytes());
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record?;
        rows.push(record.iter().map(|f| f.to_string()).collect());
    }
    Ok((headers, rows))
}

fn col(headers: &[String], name: &str) -> usize {
    headers
        .iter()
        .position(|h| h == name)
        .unwrap_or_else(|| panic!("column {name} not found in {headers:?}"))
}

fn guid_of_url(url: &str) -> Option<String> {
    if url.len() < 36 {
        return None;
    }
    let tail = &url[url.len() - 36..];
    let ok = tail.chars().enumerate().all(|(i, c)| match i {
        8 | 13 | 18 | 23 => c == '-',
        _ => c.is_ascii_hexdigit(),
    });
    if ok { Some(tail.to_ascii_lowercase()) } else { None }
}

// ---------------------------------------------------------------- axis value rendering
//
// Every rendering below reproduces the Rust variant identifier exactly, which is the same
// token the 2026-04-02 export snapshot carries in its classification columns. That is what
// makes the live-registry values drop-in comparable with the rows they replace.

fn determinism_key(v: DeterminismClass) -> &'static str {
    match v {
        DeterminismClass::Deterministic => "Deterministic",
        DeterminismClass::PseudoRandom => "PseudoRandom",
        DeterminismClass::TimeDependent => "TimeDependent",
        DeterminismClass::ExternalEventDependent => "ExternalEventDependent",
    }
}

fn volatility_key(v: VolatilityClass) -> &'static str {
    match v {
        VolatilityClass::NonVolatile => "NonVolatile",
        VolatilityClass::VolatileFull => "VolatileFull",
        VolatilityClass::VolatileContextual => "VolatileContextual",
    }
}

fn host_interaction_key(v: HostInteractionClass) -> &'static str {
    match v {
        HostInteractionClass::None => "None",
        HostInteractionClass::WorkbookState => "WorkbookState",
        HostInteractionClass::ApplicationState => "ApplicationState",
        HostInteractionClass::EnvironmentState => "EnvironmentState",
        HostInteractionClass::ExternalProvider => "ExternalProvider",
    }
}

fn thread_safety_key(v: ThreadSafetyClass) -> &'static str {
    match v {
        ThreadSafetyClass::SafePure => "SafePure",
        ThreadSafetyClass::HostSerialized => "HostSerialized",
        ThreadSafetyClass::NotThreadSafe => "NotThreadSafe",
    }
}

fn arg_preparation_key(v: ArgPreparationProfile) -> &'static str {
    match v {
        ArgPreparationProfile::ValuesOnlyPreAdapter => "ValuesOnlyPreAdapter",
        ArgPreparationProfile::RefsVisibleInAdapter => "RefsVisibleInAdapter",
    }
}

fn coercion_lift_key(v: CoercionLiftProfile) -> &'static str {
    match v {
        CoercionLiftProfile::None => "None",
        CoercionLiftProfile::UnaryNumericScalarOnly => "UnaryNumericScalarOnly",
        CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise => {
            "UnaryNumericScalarOrArrayElementwise"
        }
        CoercionLiftProfile::AggregateDirectAndRangeDualPolicy => {
            "AggregateDirectAndRangeDualPolicy"
        }
        CoercionLiftProfile::LookupMatchProfile => "LookupMatchProfile",
        CoercionLiftProfile::Custom => "Custom",
    }
}

fn kernel_signature_key(v: KernelSignatureClass) -> &'static str {
    match v {
        KernelSignatureClass::NullaryConst => "NullaryConst",
        KernelSignatureClass::NumToNum => "NumToNum",
        KernelSignatureClass::NumsToNum => "NumsToNum",
        KernelSignatureClass::TextToText => "TextToText",
        KernelSignatureClass::LookupMatch => "LookupMatch",
        KernelSignatureClass::Custom => "Custom",
    }
}

fn fec_key(v: FecDependencyProfile) -> &'static str {
    match v {
        FecDependencyProfile::None => "None",
        FecDependencyProfile::RefOnly => "RefOnly",
        FecDependencyProfile::CallerContext => "CallerContext",
        FecDependencyProfile::TimeProvider => "TimeProvider",
        FecDependencyProfile::RandomProvider => "RandomProvider",
        FecDependencyProfile::ExternalProvider => "ExternalProvider",
        FecDependencyProfile::LocaleProfile => "LocaleProfile",
        FecDependencyProfile::Composite => "Composite",
    }
}

fn error_collapse_key(v: ErrorCollapseProfile) -> &'static str {
    match v {
        ErrorCollapseProfile::None => "None",
        ErrorCollapseProfile::ReductionFold => "ReductionFold",
        ErrorCollapseProfile::SelectorBranch => "SelectorBranch",
    }
}

fn precision_rounding_variant_key(v: PrecisionRoundingProfile) -> &'static str {
    match v {
        PrecisionRoundingProfile::Default => "Default",
        PrecisionRoundingProfile::IntegerExponentPublication => "IntegerExponentPublication",
    }
}

fn lift_broadcast_variant_key(v: LiftBroadcastProfile) -> &'static str {
    match v {
        LiftBroadcastProfile::SurfaceNative => "SurfaceNative",
        LiftBroadcastProfile::ByIndexScalarArrayLift(_) => "ByIndexScalarArrayLift",
    }
}

fn arg_domain_guard_key(v: ArgDomainGuard) -> &'static str {
    match v {
        ArgDomainGuard::None => "None",
        ArgDomainGuard::CircularTrigOverflow => "CircularTrigOverflow",
    }
}

fn non_finite_key(v: NonFinite) -> &'static str {
    match v {
        NonFinite::Allow => "Allow",
        NonFinite::Num => "Num",
        NonFinite::SaturateSign => "SaturateSign",
    }
}

fn rich_value_usage_key(v: RichValueUsage) -> &'static str {
    match v {
        RichValueUsage::RichBlind => "RichBlind",
        RichValueUsage::ProducesPresentation => "ProducesPresentation",
        RichValueUsage::ProducesRichObject => "ProducesRichObject",
        RichValueUsage::ProducesErrorMetadata => "ProducesErrorMetadata",
    }
}

// ---------------------------------------------------------------- OxFunc source scraping
//
// Everything below reads OxFunc's own source text. No table of axes, variants, tiers or
// defaults is typed into this tool: each is located in the source and read out.

struct ItemDecl {
    line_start: usize,
    line_end: usize,
    documented: bool,
    variants: Vec<String>,
}

/// Locate `pub enum <name> {` / `pub struct <name> {`, return its 1-based line span, whether
/// a `///` doc comment sits immediately above it (attributes skipped), and the identifiers
/// declared at brace depth 1.
fn parse_item(lines: &[&str], header: &str) -> Option<ItemDecl> {
    let i = lines.iter().position(|l| l.trim() == header)?;

    let mut depth = 0usize;
    let mut end = i;
    for (j, l) in lines.iter().enumerate().skip(i) {
        depth += l.matches('{').count();
        depth = depth.saturating_sub(l.matches('}').count());
        if depth == 0 {
            end = j;
            break;
        }
    }

    let mut documented = false;
    let mut k = i;
    while k > 0 {
        k -= 1;
        let t = lines[k].trim();
        if t.starts_with("#[") || t.starts_with("#!") {
            continue;
        }
        documented = t.starts_with("///");
        break;
    }

    let mut variants = Vec::new();
    let mut depth = 1usize;
    for l in &lines[i + 1..end] {
        let t = l.trim();
        if depth == 1 && !t.starts_with("//") && !t.starts_with("#[") && !t.is_empty() {
            let ident: String = t
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if ident
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_uppercase())
            {
                variants.push(ident);
            }
        }
        depth += t.matches('{').count();
        depth = depth.saturating_sub(t.matches('}').count());
    }

    Some(ItemDecl {
        line_start: i + 1,
        line_end: end + 1,
        documented,
        variants,
    })
}

/// `pub` field identifiers declared at brace depth 1 of a struct.
fn parse_struct_field_names(lines: &[&str], header: &str) -> Vec<String> {
    let Some(i) = lines.iter().position(|l| l.trim() == header) else {
        return Vec::new();
    };
    let mut depth = 1usize;
    let mut out = Vec::new();
    for l in &lines[i + 1..] {
        let t = l.trim();
        if depth == 1 {
            if let Some(rest) = t.strip_prefix("pub ") {
                let ident: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if !ident.is_empty() {
                    out.push(ident);
                }
            }
        }
        depth += t.matches('{').count();
        let closes = t.matches('}').count();
        if closes >= depth {
            break;
        }
        depth -= closes;
    }
    out
}

/// Axis keys that `FunctionMeta` declares a `DEFAULT_*` associated const for. `function.rs`
/// documents that const as "THIS IS THE ONE PLACE A NEW DEFAULTABLE AXIS IS DEFAULTED".
fn parse_defaultable_axis_keys(lines: &[&str]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for l in lines {
        let t = l.trim();
        if let Some(rest) = t.strip_prefix("pub const DEFAULT_") {
            let ident: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !ident.is_empty() {
                out.insert(ident.to_ascii_lowercase());
            }
        }
    }
    out
}

// ---------------------------------------------------------------- the axis table
//
// One row per declared axis. `key` is the key the Handbook publishes; `backing_field` is the
// identifier the tier rule matches on (a published registry-metadata CSV column, or a
// `RegistryFunctionMeta` field); `defaultable_key` is the axis name the `DEFAULT_*` const scan
// produces. None of the *values* below are asserted here — they are located in OxFunc's source.

struct AxisSpec {
    key: &'static str,
    declared_kind: &'static str,
    enum_name: &'static str,
    source_file: &'static str,
    backing_field: &'static str,
    defaultable_key: &'static str,
    /// true when the string published in F1 is the variant identifier itself.
    published_is_variant: bool,
}

const AXIS_SPECS: &[AxisSpec] = &[
    AxisSpec { key: "arg_domain_guard", declared_kind: "enum", enum_name: "ArgDomainGuard", source_file: "functions/excel_numeric.rs", backing_field: "arg_domain_guard", defaultable_key: "arg_domain_guard", published_is_variant: false },
    AxisSpec { key: "arg_preparation_profile", declared_kind: "enum", enum_name: "ArgPreparationProfile", source_file: "function.rs", backing_field: "arg_preparation_profile", defaultable_key: "arg_preparation_profile", published_is_variant: true },
    AxisSpec { key: "coercion_lift_profile", declared_kind: "enum", enum_name: "CoercionLiftProfile", source_file: "function.rs", backing_field: "coercion_lift_profile", defaultable_key: "coercion_lift_profile", published_is_variant: true },
    AxisSpec { key: "determinism_class", declared_kind: "enum", enum_name: "DeterminismClass", source_file: "function.rs", backing_field: "determinism", defaultable_key: "determinism", published_is_variant: true },
    AxisSpec { key: "error_algebra", declared_kind: "enum", enum_name: "ErrorAlgebra", source_file: "semantic_kernel.rs", backing_field: "error_algebra", defaultable_key: "error_algebra", published_is_variant: true },
    AxisSpec { key: "error_collapse_profile", declared_kind: "enum", enum_name: "ErrorCollapseProfile", source_file: "function.rs", backing_field: "error_collapse_profile", defaultable_key: "error_collapse_profile", published_is_variant: true },
    AxisSpec { key: "fec_dependency_profile", declared_kind: "enum", enum_name: "FecDependencyProfile", source_file: "function.rs", backing_field: "fec_dependency_profile", defaultable_key: "fec_dependency_profile", published_is_variant: true },
    AxisSpec { key: "host_interaction_class", declared_kind: "enum", enum_name: "HostInteractionClass", source_file: "function.rs", backing_field: "host_interaction", defaultable_key: "host_interaction", published_is_variant: true },
    AxisSpec { key: "kernel_signature_class", declared_kind: "enum", enum_name: "KernelSignatureClass", source_file: "function.rs", backing_field: "kernel_signature_class", defaultable_key: "kernel_signature_class", published_is_variant: true },
    AxisSpec { key: "lift_broadcast_profile", declared_kind: "enum", enum_name: "LiftBroadcastProfile", source_file: "function.rs", backing_field: "lift_broadcast_profile", defaultable_key: "lift_broadcast_profile", published_is_variant: false },
    AxisSpec { key: "non_finite", declared_kind: "enum", enum_name: "NonFinite", source_file: "functions/excel_numeric.rs", backing_field: "non_finite", defaultable_key: "non_finite", published_is_variant: false },
    AxisSpec { key: "numerical_reduction_policy", declared_kind: "enum", enum_name: "NumericalReductionPolicy", source_file: "semantic_kernel.rs", backing_field: "numerical_reduction_policy", defaultable_key: "numerical_reduction_policy", published_is_variant: true },
    AxisSpec { key: "precision_rounding_profile", declared_kind: "enum", enum_name: "PrecisionRoundingProfile", source_file: "function.rs", backing_field: "precision_rounding_profile", defaultable_key: "precision_rounding_profile", published_is_variant: false },
    AxisSpec { key: "real_result_policy", declared_kind: "struct", enum_name: "ExcelRealPolicy", source_file: "functions/excel_numeric.rs", backing_field: "real_result_policy", defaultable_key: "real_result_policy", published_is_variant: false },
    AxisSpec { key: "rich_value_usage", declared_kind: "enum", enum_name: "RichValueUsage", source_file: "registry.rs", backing_field: "rich_value_usage", defaultable_key: "rich_value_usage", published_is_variant: true },
    AxisSpec { key: "surface_fec_dependency_profile", declared_kind: "enum", enum_name: "FecDependencyProfile", source_file: "function.rs", backing_field: "surface_fec_dependency_profile", defaultable_key: "surface_fec_dependency_profile", published_is_variant: true },
    AxisSpec { key: "thread_safety_class", declared_kind: "enum", enum_name: "ThreadSafetyClass", source_file: "function.rs", backing_field: "thread_safety", defaultable_key: "thread_safety", published_is_variant: true },
    AxisSpec { key: "volatility_class", declared_kind: "enum", enum_name: "VolatilityClass", source_file: "function.rs", backing_field: "volatility", defaultable_key: "volatility", published_is_variant: true },
];

/// The 11 axes F1 publishes inside `classification`, plus the six `axes` keys — the set
/// `axis_provenance` covers.
const CLASSIFICATION_KEYS: &[&str] = &[
    "arg_preparation_profile",
    "coercion_lift_profile",
    "determinism_class",
    "error_collapse_profile",
    "fec_dependency_profile",
    "host_interaction_class",
    "kernel_signature_class",
    "rich_value_usage",
    "surface_fec_dependency_profile",
    "thread_safety_class",
    "volatility_class",
];

/// `axes` block keys → the `RegistryFunctionMeta` field / CSV column the tier rule matches on.
const AXES_BLOCK_KEYS: &[(&str, &str)] = &[
    ("arg_admission_version", "arg_admission_metadata_version"),
    ("axes_version", "function_spec_axes_metadata_version"),
    ("lift_broadcast_profile", "lift_broadcast_profile"),
    ("precision_rounding_profile", "precision_rounding_profile"),
    ("real_result_policy", "real_result_policy"),
    ("semantic_kernel_version", "semantic_kernel_metadata_version"),
];

/// The CSV classification columns the 2026-04-02 snapshot carries, used only for the
/// export-only rows the live registry has no entry for.
const EXPORT_CLASS_COLS: &[&str] = &[
    "determinism_class",
    "volatility_class",
    "host_interaction_class",
    "thread_safety_class",
    "arg_preparation_profile",
    "coercion_lift_profile",
    "kernel_signature_class",
    "fec_dependency_profile",
    "surface_fec_dependency_profile",
];

fn tier_for(
    backing_field: &str,
    published_csv_columns: &BTreeSet<String>,
    registry_meta_fields: &BTreeSet<String>,
) -> &'static str {
    if published_csv_columns.contains(backing_field) {
        "A"
    } else if registry_meta_fields.contains(backing_field) {
        "B"
    } else {
        "C"
    }
}

fn provenance_class_for(defaultable: bool, value: &str, default_value: Option<&str>) -> &'static str {
    if !defaultable {
        return "implementation-fact";
    }
    match default_value {
        Some(d) if d == value => "default-unexamined",
        Some(_) => "excel-claimed",
        None => "implementation-fact",
    }
}

// ---------------------------------------------------------------- main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("tools/efh-ingest layout")
        .to_path_buf();
    let oxfunc_root = std::env::var("EFH_OXFUNC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.parent().expect("DnaCalc tree").join("OxFunc"));
    let lane = oxfunc_root.join("docs").join("function-lane");
    let data_dir = repo_root.join("data");
    let functions_dir = data_dir.join("functions");
    let axes_dir = data_dir.join("axes");

    // ---- publication gate (D-8 / A3-S12). Abort before anything is written or removed.
    let oxfunc_root_str = oxfunc_root
        .to_str()
        .ok_or("OxFunc root path is not valid UTF-8")?
        .to_string();
    let status = Command::new("git")
        .args(["-C", &oxfunc_root_str, "status", "--porcelain"])
        .output()?;
    if !status.status.success() {
        eprintln!(
            "efh-ingest: `git status --porcelain` failed in {oxfunc_root_str}; cannot establish oxfunc_tree_clean"
        );
        std::process::exit(2);
    }
    let dirty = String::from_utf8_lossy(&status.stdout);
    let oxfunc_tree_clean = dirty.trim().is_empty();
    if !oxfunc_tree_clean {
        eprintln!("efh-ingest: OxFunc working tree is dirty; oxfunc_tree_clean == false blocks publication.");
        eprintln!("efh-ingest: nothing was written. `git status --porcelain` reported:");
        for line in dirty.lines().take(40) {
            eprintln!("  {line}");
        }
        std::process::exit(1);
    }

    let oxfunc_commit_full = git_out(&oxfunc_root_str, &["rev-parse", "HEAD"]);
    let oxfunc_commit = git_out(&oxfunc_root_str, &["rev-parse", "--short", "HEAD"]);

    // ---- inputs ------------------------------------------------------------------

    // Row spine: the published snapshot export (534 rows).
    let export_csv_name = "OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv";
    let (eh, export_rows) = read_csv(&lane.join(export_csv_name))?;
    let e_id = col(&eh, "surface_stable_id");
    let e_kind = col(&eh, "entry_kind");
    let e_name = col(&eh, "canonical_surface_name");
    let e_cat = col(&eh, "category");
    let e_status = col(&eh, "metadata_status");
    let e_sym = col(&eh, "xlcall_builtin_symbol");
    let e_code = col(&eh, "xlcall_builtin_code");
    let e_amin = col(&eh, "arity_min");
    let e_amax = col(&eh, "arity_max");
    let e_vm = col(&eh, "version_marker");
    let e_sik = col(&eh, "special_interface_kind");
    let e_aik = col(&eh, "admission_interface_kind");
    let e_snap = col(&eh, "snapshot_id");
    let e_gen = col(&eh, "snapshot_generation");
    let e_commit = col(&eh, "source_commit_short");
    let e_tree = col(&eh, "source_tree_state");
    let export_class_idx: Vec<(String, usize)> = EXPORT_CLASS_COLS
        .iter()
        .map(|c| (c.to_string(), col(&eh, c)))
        .collect();

    // Deferred inventory: surface name -> reason.
    let deferred_csv_name = "W50_DEFERRED_CURRENT_VERSION_INVENTORY.csv";
    let (dh, drows) = read_csv(&lane.join(deferred_csv_name))?;
    let d_name = col(&dh, "entry_name");
    let d_notes = col(&dh, "notes");
    let deferred: BTreeMap<String, String> = drows
        .iter()
        .map(|r| (r[d_name].clone(), r[d_notes].clone()))
        .collect();

    // English docs catalog: surface name -> (url, description).
    let catalog_csv_name = "FUNCTION_CATALOG_CURRENT_BASELINE_LOCAL.csv";
    let (ch, crows) = read_csv(&lane.join(catalog_csv_name))?;
    let c_name = col(&ch, "function_name");
    let c_url = col(&ch, "function_url");
    let c_desc = col(&ch, "description");
    let mut catalog: BTreeMap<String, (String, Option<String>)> = BTreeMap::new();
    for r in &crows {
        catalog.insert(r[c_name].clone(), (r[c_url].clone(), none_if_empty(&r[c_desc])));
    }

    // Localization seed: article guid -> [(locale, localized name)].
    let locale_csv_name = "W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv";
    let (lh, lrows) = read_csv(&lane.join(locale_csv_name))?;
    let l_locale = col(&lh, "locale_tag");
    let l_guid = col(&lh, "function_article_guid");
    let l_name = col(&lh, "localized_name");
    let mut locales: BTreeMap<String, Vec<LocalizedName>> = BTreeMap::new();
    for r in &lrows {
        locales
            .entry(r[l_guid].to_ascii_lowercase())
            .or_default()
            .push(LocalizedName {
                locale: r[l_locale].clone(),
                name: r[l_name].clone(),
            });
    }
    for v in locales.values_mut() {
        v.sort_by(|a, b| a.locale.cmp(&b.locale));
    }

    // Correlation ledger: function id -> artifact bindings.
    let ledger_csv_name = "FUNCTION_SLICE_CORRELATION_LEDGER.csv";
    let (rh, rrows) = read_csv(&lane.join(ledger_csv_name))?;
    let r_id = col(&rh, "function_id");
    let r_contract = col(&rh, "contract_row_id");
    let r_lean = col(&rh, "lean_module");
    let r_rust = col(&rh, "rust_module");
    let r_status = col(&rh, "status");
    let mut artifacts: BTreeMap<String, Artifacts> = BTreeMap::new();
    for r in &rrows {
        artifacts.insert(
            r[r_id].clone(),
            Artifacts {
                contract_row_id: none_if_empty(&r[r_contract]),
                lean_module: none_if_empty(&r[r_lean]),
                rust_module: none_if_empty(&r[r_rust]),
                status: none_if_empty(&r[r_status]),
            },
        );
    }

    // Runtime registry (canonical enrichment source, and — from v0.2 — the source of
    // `arity` and `classification`).
    let registry = oxfunc_core::registry::builtin_registry();
    let by_id: BTreeMap<&str, &oxfunc_core::registry::FunctionEntry> = registry
        .iter()
        .map(|e| (e.meta.function_id.as_str(), e))
        .collect();
    let registry_identity = registry.snapshot_identity();
    let catalog_size = by_id.len();

    // Raw `FunctionMeta` (carries `error_collapse_profile`, which the registry projection
    // deliberately does not re-export).
    let meta_by_id: BTreeMap<&str, &FunctionMeta> = oxfunc_core::xll_export_specs::function_catalog()
        .iter()
        .map(|m| (m.function_id, m))
        .collect();

    // ---- OxFunc source scraping ---------------------------------------------------

    let src_root = oxfunc_root.join("crates").join("oxfunc_core").join("src");
    let mut source_text: BTreeMap<&'static str, String> = BTreeMap::new();
    for f in ["function.rs", "registry.rs", "semantic_kernel.rs", "functions/excel_numeric.rs"] {
        source_text.insert(f, fs::read_to_string(src_root.join(f))?);
    }
    let source_lines: BTreeMap<&'static str, Vec<&str>> = source_text
        .iter()
        .map(|(k, v)| (*k, v.lines().collect::<Vec<_>>()))
        .collect();

    let defaultable_keys = parse_defaultable_axis_keys(&source_lines["function.rs"]);
    let registry_meta_fields: BTreeSet<String> = parse_struct_field_names(
        &source_lines["registry.rs"],
        "pub struct RegistryFunctionMeta {",
    )
    .into_iter()
    .collect();
    // The frozen + additive published contract, read out of its own emitter.
    let published_csv_header = oxfunc_core::registry::render_registry_metadata_csv(registry)
        .lines()
        .next()
        .expect("registry-metadata CSV has a header")
        .to_string();
    let published_csv_columns: BTreeSet<String> =
        published_csv_header.split(',').map(|s| s.to_string()).collect();

    // Default values of the five defaultable axes, read from the public consts.
    let default_axes = oxfunc_core::registry::FunctionSpecAxesMetadata::default_axes();
    let mut default_value: BTreeMap<&'static str, String> = BTreeMap::new();
    if defaultable_keys.contains("arg_preparation_profile") {
        default_value.insert(
            "arg_preparation_profile",
            arg_preparation_key(FunctionMeta::DEFAULT_ARG_PREPARATION_PROFILE).to_string(),
        );
    }
    if defaultable_keys.contains("error_collapse_profile") {
        default_value.insert(
            "error_collapse_profile",
            error_collapse_key(FunctionMeta::DEFAULT_ERROR_COLLAPSE_PROFILE).to_string(),
        );
    }
    if defaultable_keys.contains("lift_broadcast_profile") {
        default_value.insert(
            "lift_broadcast_profile",
            default_axes.lift_broadcast_profile.clone(),
        );
    }
    if defaultable_keys.contains("precision_rounding_profile") {
        default_value.insert(
            "precision_rounding_profile",
            default_axes.precision_rounding_profile.clone(),
        );
    }
    if defaultable_keys.contains("real_result_policy") {
        default_value.insert("real_result_policy", default_axes.real_result_policy.clone());
    }

    // Tier per published axis key.
    let mut tier_by_key: BTreeMap<&'static str, &'static str> = BTreeMap::new();
    for spec in AXIS_SPECS {
        tier_by_key.insert(
            spec.key,
            tier_for(spec.backing_field, &published_csv_columns, &registry_meta_fields),
        );
    }
    for (key, backing) in AXES_BLOCK_KEYS {
        tier_by_key.insert(
            key,
            tier_for(backing, &published_csv_columns, &registry_meta_fields),
        );
    }

    // ---- vintage states -----------------------------------------------------------

    let first_row = export_rows.first().expect("export has rows");
    let snap_id = first_row[e_snap].clone();
    let snap_gen = first_row[e_gen].clone();
    let snap_commit = first_row[e_commit].clone();
    let snap_tree = first_row[e_tree].clone();

    let vs_export = format!("export-snapshot:{snap_id}@{snap_commit}:{snap_tree}:{snap_gen}");
    let vs_registry = format!("live-registry:{oxfunc_commit}:clean:undated");
    let vs_absent = "absent:no-live-registry-entry-for-this-id".to_string();
    let lane_state = |file: &str| format!("lane-csv:{file}@{oxfunc_commit}:clean:undated");
    let vs_deferred = lane_state(deferred_csv_name);
    let vs_catalog = lane_state(catalog_csv_name);
    let vs_locale = lane_state(locale_csv_name);
    let vs_ledger = lane_state(ledger_csv_name);

    let mut vintage_states: BTreeMap<String, VintageState> = BTreeMap::new();
    vintage_states.insert(
        vs_export.clone(),
        VintageState {
            basis: "export-snapshot",
            source: format!("docs/function-lane/{export_csv_name}"),
            as_of: snap_gen.clone(),
            source_commit: snap_commit.clone(),
            source_tree_state: snap_tree.clone(),
            // The snapshot declares its own source tree state; `dirty` means these bytes
            // cannot be reproduced from any commit.
            regenerable_from_named_commit: snap_tree == "clean",
        },
    );
    vintage_states.insert(
        vs_registry.clone(),
        VintageState {
            basis: "live-registry",
            source: "oxfunc_core::registry::builtin_registry()".to_string(),
            as_of: "not-dated-by-this-basis".to_string(),
            source_commit: oxfunc_commit_full.clone(),
            source_tree_state: "clean".to_string(),
            regenerable_from_named_commit: true,
        },
    );
    for (id, file) in [
        (&vs_deferred, deferred_csv_name),
        (&vs_catalog, catalog_csv_name),
        (&vs_locale, locale_csv_name),
        (&vs_ledger, ledger_csv_name),
    ] {
        vintage_states.insert(
            id.clone(),
            VintageState {
                basis: "lane-csv",
                source: format!("docs/function-lane/{file}"),
                as_of: "not-dated-by-this-basis".to_string(),
                source_commit: oxfunc_commit_full.clone(),
                source_tree_state: "clean".to_string(),
                regenerable_from_named_commit: true,
            },
        );
    }
    vintage_states.insert(
        vs_absent.clone(),
        VintageState {
            basis: "absent",
            source: "no source consulted carries this field group for this entry".to_string(),
            as_of: "not-dated-by-this-basis".to_string(),
            source_commit: oxfunc_commit_full.clone(),
            source_tree_state: "clean".to_string(),
            regenerable_from_named_commit: true,
        },
    );

    if std::env::var("EFH_DEBUG").is_ok() {
        let export_ids: BTreeSet<&str> = export_rows.iter().map(|r| r[e_id].as_str()).collect();
        let unmatched_export: Vec<&&str> = export_ids
            .iter()
            .filter(|id| !by_id.contains_key(**id))
            .collect();
        let unmatched_registry: Vec<&&str> = by_id
            .keys()
            .filter(|id| !export_ids.contains(**id))
            .collect();
        eprintln!("export ids with no registry entry ({}): {unmatched_export:?}", unmatched_export.len());
        eprintln!("registry ids with no export row ({}): {unmatched_registry:?}", unmatched_registry.len());
    }

    // ---- spine ---------------------------------------------------------------------

    struct SpineEntry<'a> {
        row: &'a Vec<String>,
        function_id: String,
        surface_name: String,
        published_row_id: String,
    }
    let mut spine: Vec<SpineEntry> = Vec::new();
    let mut split_rows = 0usize;
    for row in &export_rows {
        let published_id = row[e_id].clone();
        let name = row[e_name].clone();
        if name.contains(", ") {
            split_rows += 1;
            for part in name.split(", ") {
                spine.push(SpineEntry {
                    row,
                    function_id: format!("FUNC.{part}"),
                    surface_name: part.to_string(),
                    published_row_id: published_id.clone(),
                });
            }
        } else {
            spine.push(SpineEntry {
                row,
                function_id: published_id.clone(),
                surface_name: name,
                published_row_id: published_id,
            });
        }
    }
    spine.sort_by(|a, b| a.function_id.cmp(&b.function_id));

    // Only now is anything removed from `data/`.
    if functions_dir.exists() {
        fs::remove_dir_all(&functions_dir)?;
    }
    fs::create_dir_all(&functions_dir)?;
    fs::create_dir_all(&axes_dir)?;

    let mut counts = Counts {
        total_entries: 0,
        published_rows: export_rows.len(),
        split_combined_rows: split_rows,
        functions: 0,
        operators: 0,
        registry_backed: 0,
        export_only: 0,
        deferred: 0,
        placeholder_signatures: 0,
        with_docs_link: 0,
        with_localized_names: 0,
        classification_from_live_registry: 0,
        classification_from_export_snapshot: 0,
        classification_empty: 0,
        arity_from_live_registry: 0,
        arity_from_export_snapshot: 0,
        arity_max_unbounded: 0,
        trailing_repeats_null: 0,
        text_integrity_present: 0,
        short_microsoft_english_verbatim: 0,
        short_differs_from_english_description: 0,
        short_no_english_description_to_compare: 0,
        short_absent: 0,
    };
    let mut index_rows = Vec::new();

    for entry_row in &spine {
        let row = entry_row.row;
        let id = entry_row.function_id.clone();
        let name = entry_row.surface_name.clone();
        let kind = row[e_kind].clone();
        counts.total_entries += 1;
        match kind.as_str() {
            "built_in_operator" => counts.operators += 1,
            _ => counts.functions += 1,
        }

        let admission = if let Some(reason) = deferred.get(&name) {
            counts.deferred += 1;
            Admission {
                label: "deferred".to_string(),
                reason: Some(reason.clone()),
            }
        } else {
            Admission {
                label: "supported".to_string(),
                reason: None,
            }
        };

        let entry = by_id.get(id.as_str()).copied();
        let raw_meta = meta_by_id.get(id.as_str()).copied();

        // ---- D-1: arity + classification from the live registry ---------------------
        let mut classification: BTreeMap<String, String> = BTreeMap::new();
        let arity;
        let vintage_arity;
        let vintage_classification;
        if let Some(entry) = entry {
            let m = &entry.meta;
            classification.insert("determinism_class".into(), determinism_key(m.determinism).into());
            classification.insert("volatility_class".into(), volatility_key(m.volatility).into());
            classification.insert("host_interaction_class".into(), host_interaction_key(m.host_interaction).into());
            classification.insert("thread_safety_class".into(), thread_safety_key(m.thread_safety).into());
            classification.insert("arg_preparation_profile".into(), arg_preparation_key(m.arg_preparation_profile).into());
            classification.insert("coercion_lift_profile".into(), coercion_lift_key(m.coercion_lift_profile).into());
            classification.insert("kernel_signature_class".into(), kernel_signature_key(m.kernel_signature_class).into());
            classification.insert("fec_dependency_profile".into(), fec_key(m.fec_dependency_profile).into());
            classification.insert("surface_fec_dependency_profile".into(), fec_key(m.surface_fec_dependency_profile).into());
            classification.insert("rich_value_usage".into(), rich_value_usage_key(m.rich_value_usage).into());
            if let Some(raw) = raw_meta {
                classification.insert(
                    "error_collapse_profile".into(),
                    error_collapse_key(raw.error_collapse_profile).into(),
                );
            }
            arity = ArityDoc {
                min: JsonValue::from(m.arity.min as u64),
                max: if m.arity.max == usize::MAX {
                    JsonValue::String("unbounded".to_string())
                } else {
                    JsonValue::from(m.arity.max as u64)
                },
            };
            vintage_arity = vs_registry.clone();
            vintage_classification = vs_registry.clone();
            counts.arity_from_live_registry += 1;
            counts.classification_from_live_registry += 1;
        } else {
            // No live-registry entry: fall back to the 2026-04-02 export snapshot, and say so
            // in `vintage` rather than silently mixing the two bases.
            for (cname, cidx) in &export_class_idx {
                if let Some(v) = none_if_empty(&row[*cidx]) {
                    classification.insert(cname.clone(), v);
                }
            }
            let pmin: Option<u64> = row[e_amin].trim().parse().ok();
            let pmax: Option<u64> = row[e_amax].trim().parse().ok();
            arity = ArityDoc {
                min: pmin.map(JsonValue::from).unwrap_or(JsonValue::Null),
                max: match pmax {
                    Some(v) if v == u64::MAX => JsonValue::String("unbounded".to_string()),
                    Some(v) => JsonValue::from(v),
                    None => JsonValue::Null,
                },
            };
            vintage_arity = vs_export.clone();
            vintage_classification = vs_export.clone();
            counts.arity_from_export_snapshot += 1;
            counts.classification_from_export_snapshot += 1;
        }
        if classification.is_empty() {
            counts.classification_empty += 1;
        }
        if arity.max == JsonValue::String("unbounded".to_string()) {
            counts.arity_max_unbounded += 1;
        }

        // ---- registry-backed enrichment ---------------------------------------------
        let mut signature = None;
        let mut signature_placeholder = false;
        let mut short = None;
        let mut long = None;
        let mut axes = None;
        if let Some(entry) = entry {
            counts.registry_backed += 1;
            let placeholder_prefix = format!("{}(...", entry.surface_name);
            let sig = &entry.display_signature;
            signature_placeholder = sig.signature_display.starts_with(&placeholder_prefix);
            if signature_placeholder {
                counts.placeholder_signatures += 1;
            } else {
                // D-5. `registry.rs::signature_from_seed` computes
                //   trailing_repeats = seed.trailing_repeats || (arity.max > parameters.len())
                // With an empty parameter list and arity.max > 0 the published `true` is entirely
                // arity-implied and says nothing about a trailing parameter — there is none. That
                // is the derivation artifact; it is emitted as `null`. Where arity.max == 0 the
                // implication cannot fire, so the seeded `false` is a real fact and is kept.
                let params_empty = sig.parameters.is_empty();
                let arity_implied = entry.meta.arity.max > sig.parameters.len();
                let trailing_repeats = if params_empty && arity_implied {
                    counts.trailing_repeats_null += 1;
                    None
                } else {
                    Some(sig.trailing_repeats)
                };
                signature = Some(SignatureDoc {
                    display: sig.signature_display.clone(),
                    trailing_repeats,
                    parameters: sig
                        .parameters
                        .iter()
                        .map(|p| ParameterDoc {
                            name: p.name.clone(),
                            optional: p.optional,
                            repeats: p.repeats,
                            description: p.short_description.clone(),
                        })
                        .collect(),
                });
            }
            short = entry.short_description.clone();
            long = entry.long_description.clone();
            let m = &entry.meta;
            axes = Some(Axes {
                lift_broadcast_profile: m.function_spec_axes_metadata.lift_broadcast_profile.clone(),
                precision_rounding_profile: m
                    .function_spec_axes_metadata
                    .precision_rounding_profile
                    .clone(),
                real_result_policy: m.function_spec_axes_metadata.real_result_policy.clone(),
                axes_version: m.function_spec_axes_metadata_version.clone(),
                semantic_kernel_version: m.semantic_kernel_metadata_version.clone(),
                arg_admission_version: m.arg_admission_metadata_version.clone(),
            });
        } else {
            counts.export_only += 1;
        }

        let docs = catalog
            .get(&name)
            .or_else(|| catalog.get(&row[e_name]))
            .map(|(url, desc)| DocsLink {
                microsoft_url: url.clone(),
                english_description: desc.clone(),
            });
        if docs.is_some() {
            counts.with_docs_link += 1;
        }

        // ---- D-6 --------------------------------------------------------------------
        let english = docs.as_ref().and_then(|d| d.english_description.clone());
        let short_source: &'static str = match (&short, &english) {
            (None, _) => {
                counts.short_absent += 1;
                "no-short-description"
            }
            (Some(_), None) => {
                counts.short_no_english_description_to_compare += 1;
                "no-english-description-to-compare"
            }
            (Some(s), Some(e)) if s.as_bytes() == e.as_bytes() => {
                counts.short_microsoft_english_verbatim += 1;
                "microsoft-english-verbatim"
            }
            _ => {
                counts.short_differs_from_english_description += 1;
                "differs-from-english-description"
            }
        };

        let localized_names: Vec<LocalizedName> = docs
            .as_ref()
            .and_then(|d| guid_of_url(&d.microsoft_url))
            .and_then(|g| locales.get(&g))
            .map(|v| {
                v.iter()
                    .map(|ln| LocalizedName {
                        locale: ln.locale.clone(),
                        name: ln.name.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        if !localized_names.is_empty() {
            counts.with_localized_names += 1;
        }

        // ---- D-7 --------------------------------------------------------------------
        // Mechanical rule, no judgement: the field contains U+003F QUESTION MARK, which is what
        // a lossy encoding substitution leaves behind. Fields are tested in the fixed order
        // below and the first hit is emitted. What character (if any) was lost is an inference
        // and is not recorded here.
        let signature_display = signature.as_ref().map(|s| s.display.clone());
        let mut text_integrity = None;
        for (field, value) in [
            ("descriptions.short", short.as_ref()),
            ("descriptions.long", long.as_ref()),
            ("docs.english_description", english.as_ref()),
            ("signature.display", signature_display.as_ref()),
        ] {
            if let Some(v) = value {
                if v.contains('?') {
                    text_integrity = Some(TextIntegrity {
                        field,
                        observed: v.clone(),
                        contains_lossy_substitution: true,
                    });
                    counts.text_integrity_present += 1;
                    break;
                }
            }
        }

        // ---- D-3 / D-4 ---------------------------------------------------------------
        let mut axis_provenance: BTreeMap<String, AxisProvenance> = BTreeMap::new();
        for key in CLASSIFICATION_KEYS {
            if let Some(v) = classification.get(*key) {
                let defaultable = defaultable_keys.contains(*key);
                axis_provenance.insert(
                    (*key).to_string(),
                    AxisProvenance {
                        provenance_class: provenance_class_for(
                            defaultable,
                            v,
                            default_value.get(key).map(|s| s.as_str()),
                        ),
                        oxfunc_tier: tier_by_key.get(key).copied().unwrap_or("C"),
                    },
                );
            }
        }
        if let Some(a) = &axes {
            for (key, value) in [
                ("lift_broadcast_profile", &a.lift_broadcast_profile),
                ("precision_rounding_profile", &a.precision_rounding_profile),
                ("real_result_policy", &a.real_result_policy),
                ("axes_version", &a.axes_version),
                ("semantic_kernel_version", &a.semantic_kernel_version),
                ("arg_admission_version", &a.arg_admission_version),
            ] {
                let defaultable = defaultable_keys.contains(key);
                axis_provenance.insert(
                    key.to_string(),
                    AxisProvenance {
                        provenance_class: provenance_class_for(
                            defaultable,
                            value,
                            default_value.get(key).map(|s| s.as_str()),
                        ),
                        oxfunc_tier: tier_by_key.get(key).copied().unwrap_or("C"),
                    },
                );
            }
        }

        // ---- D-8 ---------------------------------------------------------------------
        let registry_or_absent = if entry.is_some() { &vs_registry } else { &vs_absent };
        let artifacts_value = artifacts.remove(&id);
        let mut vintage: BTreeMap<String, String> = BTreeMap::new();
        vintage.insert("identity".into(), vs_export.clone());
        vintage.insert("category".into(), vs_export.clone());
        vintage.insert("metadata_status".into(), vs_export.clone());
        vintage.insert("xlcall".into(), vs_export.clone());
        vintage.insert("interface_markers".into(), vs_export.clone());
        vintage.insert("admission".into(), vs_deferred.clone());
        vintage.insert("arity".into(), vintage_arity);
        vintage.insert("classification".into(), vintage_classification);
        vintage.insert("axes".into(), registry_or_absent.clone());
        vintage.insert("axis_provenance".into(), registry_or_absent.clone());
        vintage.insert("signature".into(), registry_or_absent.clone());
        vintage.insert("signature_placeholder".into(), registry_or_absent.clone());
        vintage.insert("descriptions".into(), registry_or_absent.clone());
        vintage.insert("registry_backed".into(), vs_registry.clone());
        vintage.insert("oxfunc_tree_clean".into(), vs_registry.clone());
        vintage.insert("docs".into(), vs_catalog.clone());
        vintage.insert("localized_names".into(), vs_locale.clone());
        // The correlation ledger is consulted for every entry; a `null` here means the ledger
        // has no row for this id, not that no source was consulted.
        vintage.insert("artifacts".into(), vs_ledger.clone());
        // `text_integrity` observes one named field; its vintage is that field's group's.
        if let Some(ti) = &text_integrity {
            let state = if ti.field.starts_with("docs.") {
                vs_catalog.clone()
            } else {
                registry_or_absent.clone()
            };
            vintage.insert("text_integrity".into(), state);
        }

        let doc = FunctionDoc {
            function_id: id.clone(),
            surface_name: name.clone(),
            published_row_id: entry_row.published_row_id.clone(),
            entry_kind: kind.clone(),
            category: none_if_empty(&row[e_cat]),
            admission,
            metadata_status: none_if_empty(&row[e_status]),
            xlcall: match (none_if_empty(&row[e_sym]), none_if_empty(&row[e_code])) {
                (Some(symbol), Some(code)) => Some(Xlcall { symbol, code }),
                _ => None,
            },
            arity,
            classification,
            axes,
            axis_provenance,
            signature,
            signature_placeholder,
            descriptions: Descriptions { short, short_source, long },
            docs,
            localized_names,
            artifacts: artifacts_value,
            version_marker: none_if_empty(&row[e_vm]),
            special_interface_kind: none_if_empty(&row[e_sik]),
            admission_interface_kind: none_if_empty(&row[e_aik]),
            registry_backed: entry.is_some(),
            text_integrity,
            oxfunc_tree_clean,
            vintage,
        };

        let path = functions_dir.join(format!("{id}.json"));
        let mut json = serde_json::to_string_pretty(&doc)?;
        json.push('\n');
        fs::write(&path, json)?;

        index_rows.push(IndexRow {
            function_id: id,
            surface_name: name,
            entry_kind: kind,
            category: none_if_empty(&row[e_cat]),
            admission: doc_admission_label(&deferred, &entry_row.surface_name),
        });
    }

    // ---- F3 vocabulary --------------------------------------------------------------

    let vocabulary = build_vocabulary(
        registry,
        &meta_by_id,
        &source_lines,
        &defaultable_keys,
        &default_value,
        &tier_by_key,
        &oxfunc_commit_full,
        oxfunc_tree_clean,
        catalog_size,
    );
    let mut json = serde_json::to_string_pretty(&vocabulary)?;
    json.push('\n');
    fs::write(axes_dir.join("vocabulary.json"), json)?;

    // ---- F0 index -------------------------------------------------------------------

    let index = IndexDoc {
        provenance: Provenance {
            export_snapshot_id: snap_id,
            export_snapshot_generation: snap_gen,
            export_source_commit: snap_commit,
            export_source_tree_state: snap_tree,
            oxfunc_commit_at_ingest: oxfunc_commit.clone(),
            oxfunc_commit_at_ingest_full: oxfunc_commit_full,
            oxfunc_tree_clean,
            registry_snapshot_family: registry_identity.snapshot_family,
            registry_generation: registry_identity.generation,
            registry_content_fingerprint: registry_identity.content_fingerprint,
            tool: format!("efh-ingest {}", env!("CARGO_PKG_VERSION")),
        },
        vintage_states,
        counts,
        rows: index_rows,
    };
    let mut json = serde_json::to_string_pretty(&index)?;
    json.push('\n');
    fs::write(data_dir.join("index.json"), json)?;

    let c = &index.counts;
    println!(
        "efh-ingest: {} entries from {} published rows ({} split) — {} functions, {} operators; {} registry-backed, {} export-only, {} deferred, {} placeholder signatures, {} with docs link, {} with localized names",
        c.total_entries,
        c.published_rows,
        c.split_combined_rows,
        c.functions,
        c.operators,
        c.registry_backed,
        c.export_only,
        c.deferred,
        c.placeholder_signatures,
        c.with_docs_link,
        c.with_localized_names
    );
    println!(
        "efh-ingest: classification live-registry {} / export-snapshot {} / empty {}; arity live-registry {} / export-snapshot {}; arity.max unbounded {}; trailing_repeats null {}; text_integrity {}",
        c.classification_from_live_registry,
        c.classification_from_export_snapshot,
        c.classification_empty,
        c.arity_from_live_registry,
        c.arity_from_export_snapshot,
        c.arity_max_unbounded,
        c.trailing_repeats_null,
        c.text_integrity_present
    );
    println!(
        "efh-ingest: descriptions.short_source — microsoft-english-verbatim {}, differs-from-english-description {}, no-english-description-to-compare {}, no-short-description {}",
        c.short_microsoft_english_verbatim,
        c.short_differs_from_english_description,
        c.short_no_english_description_to_compare,
        c.short_absent
    );
    Ok(())
}

fn git_out(repo: &str, args: &[&str]) -> String {
    let mut full = vec!["-C", repo];
    full.extend_from_slice(args);
    Command::new("git")
        .args(&full)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn doc_admission_label(deferred: &BTreeMap<String, String>, name: &str) -> String {
    if deferred.contains_key(name) {
        "deferred".to_string()
    } else {
        "supported".to_string()
    }
}

// ---------------------------------------------------------------- F3 builder

#[allow(clippy::too_many_arguments)]
fn build_vocabulary(
    registry: &oxfunc_core::registry::FunctionRegistry,
    meta_by_id: &BTreeMap<&str, &FunctionMeta>,
    source_lines: &BTreeMap<&'static str, Vec<&str>>,
    defaultable_keys: &BTreeSet<String>,
    default_value: &BTreeMap<&'static str, String>,
    tier_by_key: &BTreeMap<&'static str, &'static str>,
    oxfunc_commit_full: &str,
    oxfunc_tree_clean: bool,
    catalog_size: usize,
) -> VocabularyDoc {
    // Per axis: the observed variant identifier and the published string, per catalog entry.
    // `None` means the entry carries no value on that axis at all.
    let mut observed: BTreeMap<&'static str, Vec<(String, Option<String>, Option<String>)>> =
        BTreeMap::new();
    for spec in AXIS_SPECS {
        observed.insert(spec.key, Vec::new());
    }

    let mut entries: Vec<&oxfunc_core::registry::FunctionEntry> = registry.iter().collect();
    entries.sort_by(|a, b| a.meta.function_id.cmp(&b.meta.function_id));

    for entry in entries {
        let m = &entry.meta;
        let raw = meta_by_id.get(m.function_id.as_str()).copied();
        let sn = entry.surface_name.clone();
        let axes = &m.function_spec_axes_metadata;

        let mut push = |key: &'static str, variant: Option<String>, published: Option<String>| {
            observed
                .get_mut(key)
                .expect("axis key declared in AXIS_SPECS")
                .push((sn.clone(), variant, published));
        };

        push("determinism_class", Some(determinism_key(m.determinism).into()), None);
        push("volatility_class", Some(volatility_key(m.volatility).into()), None);
        push("host_interaction_class", Some(host_interaction_key(m.host_interaction).into()), None);
        push("thread_safety_class", Some(thread_safety_key(m.thread_safety).into()), None);
        push("arg_preparation_profile", Some(arg_preparation_key(m.arg_preparation_profile).into()), None);
        push("coercion_lift_profile", Some(coercion_lift_key(m.coercion_lift_profile).into()), None);
        push("kernel_signature_class", Some(kernel_signature_key(m.kernel_signature_class).into()), None);
        push("fec_dependency_profile", Some(fec_key(m.fec_dependency_profile).into()), None);
        push("surface_fec_dependency_profile", Some(fec_key(m.surface_fec_dependency_profile).into()), None);
        push("rich_value_usage", Some(rich_value_usage_key(m.rich_value_usage).into()), None);
        push(
            "numerical_reduction_policy",
            m.semantic_kernel_metadata.numerical_reduction_policy.clone(),
            None,
        );
        push("error_algebra", m.semantic_kernel_metadata.error_algebra.clone(), None);
        push(
            "error_collapse_profile",
            raw.map(|r| error_collapse_key(r.error_collapse_profile).to_string()),
            None,
        );
        push(
            "lift_broadcast_profile",
            raw.map(|r| lift_broadcast_variant_key(r.lift_broadcast_profile).to_string()),
            Some(axes.lift_broadcast_profile.clone()),
        );
        push(
            "precision_rounding_profile",
            raw.map(|r| precision_rounding_variant_key(r.precision_rounding_profile).to_string()),
            Some(axes.precision_rounding_profile.clone()),
        );
        push("real_result_policy", None, Some(axes.real_result_policy.clone()));
        push(
            "arg_domain_guard",
            raw.map(|r| arg_domain_guard_key(r.real_result_policy.arg_domain_guard).to_string()),
            None,
        );
        push(
            "non_finite",
            raw.map(|r| non_finite_key(r.real_result_policy.non_finite).to_string()),
            None,
        );
    }

    let mut axes_docs = Vec::new();
    for spec in AXIS_SPECS {
        let lines = &source_lines[spec.source_file];
        let header = if spec.declared_kind == "struct" {
            format!("pub struct {} {{", spec.enum_name)
        } else {
            format!("pub enum {} {{", spec.enum_name)
        };
        let decl = parse_item(lines, &header)
            .unwrap_or_else(|| panic!("could not locate `{header}` in {}", spec.source_file));

        let samples = &observed[spec.key];
        let declared: Vec<String> = if spec.declared_kind == "struct" {
            Vec::new()
        } else {
            decl.variants.clone()
        };

        let mut variants = Vec::new();
        for v in &declared {
            let mut exemplars: Vec<String> = samples
                .iter()
                .filter(|(_, variant, _)| variant.as_deref() == Some(v.as_str()))
                .map(|(sn, _, _)| sn.clone())
                .collect();
            let occupancy = exemplars.len();
            exemplars.sort();
            exemplars.truncate(8);
            variants.push(VariantDoc {
                value: v.clone(),
                occupancy,
                exemplars,
            });
        }
        // An entry is "unset" on an axis only when it carries neither a variant nor a
        // published string — i.e. the axis genuinely has no value for it.
        let unset_occupancy = samples
            .iter()
            .filter(|(_, variant, published)| variant.is_none() && published.is_none())
            .count();

        let published_value_occupancy = if spec.published_is_variant {
            None
        } else {
            let mut tally: BTreeMap<String, usize> = BTreeMap::new();
            for (_, _, published) in samples {
                if let Some(p) = published {
                    *tally.entry(p.clone()).or_default() += 1;
                }
            }
            if tally.is_empty() {
                None
            } else {
                Some(
                    tally
                        .into_iter()
                        .map(|(value, occupancy)| PublishedValueDoc { value, occupancy })
                        .collect(),
                )
            }
        };

        // Modal share over the declared variants; where the axis declares none (a struct
        // rendered to a composite key), over the published strings instead.
        let modal_occupancy = variants
            .iter()
            .map(|v| v.occupancy)
            .max()
            .or_else(|| {
                published_value_occupancy
                    .as_ref()
                    .and_then(|p: &Vec<PublishedValueDoc>| p.iter().map(|v| v.occupancy).max())
            });
        let modal_share = modal_occupancy
            .filter(|_| catalog_size > 0)
            .map(|m| m as f64 / catalog_size as f64);

        axes_docs.push(AxisDoc {
            key: spec.key,
            declared_kind: spec.declared_kind,
            enum_name: spec.enum_name,
            backing_field: spec.backing_field,
            source_path: format!(
                "crates/oxfunc_core/src/{}:{}-{}",
                spec.source_file, decl.line_start, decl.line_end
            ),
            defaultable: defaultable_keys.contains(spec.defaultable_key),
            default_value: default_value.get(spec.key).cloned(),
            oxfunc_tier: tier_by_key.get(spec.key).copied().unwrap_or("C"),
            documented_in_source: decl.documented,
            variants,
            unset_occupancy,
            modal_share,
            published_value_occupancy,
        });
    }
    axes_docs.sort_by(|a, b| a.key.cmp(b.key));

    VocabularyDoc {
        schema: "efh.axis-vocabulary/v1",
        oxfunc_commit: oxfunc_commit_full.to_string(),
        oxfunc_tree_clean,
        catalog_size,
        extraction: Extraction {
            variant_source: "variant identifiers are read out of the `pub enum` declaration named in `source_path`; no variant list is typed into efh-ingest",
            line_span_rule: "1-based first line of the `pub enum` / `pub struct` header through its matching closing brace",
            documented_in_source_rule: "true iff the first non-attribute line immediately above the declaration begins with `///`",
            defaultable_rule: "true iff `function.rs` declares a `pub const DEFAULT_<AXIS>` associated const whose lowercased suffix equals the axis's backing field name",
            oxfunc_tier_rule: "A iff the backing field is a column of `registry::render_registry_metadata_csv` (OxFunc's own frozen + additive published contract); else B iff it is a `pub` field of `RegistryFunctionMeta`; else C",
            occupancy_population: format!(
                "the {catalog_size} entries of `registry::builtin_registry()` at this commit"
            ),
            unset_occupancy_meaning: "entries carrying neither a declared variant nor a published string on this axis — the axis has no value for them at all",
            modal_share_rule: "highest declared-variant occupancy divided by catalog_size; where the declaration has no variants (a struct rendered to a composite key) the highest published-string occupancy is used instead",
            exemplars_rule: "surface names carrying the variant, ordinal-sorted, first 8; the complete set when occupancy <= 8",
            fields_not_emitted: vec![
                "consumer_sites — D1 §2.3 asks for non-test read sites, which is a semantic analysis of which code consumes an axis. A textual grep is a different fact and would be published under a name that overstates it, so it is omitted rather than approximated.",
            ],
        },
        axes: axes_docs,
    }
}
