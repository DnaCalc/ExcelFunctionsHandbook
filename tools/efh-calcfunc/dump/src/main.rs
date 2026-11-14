//! Prints calc-func's catalogue as JSON: spelling, notation, arity, parameters and help text.
//! Built by regenerate.py against a checkout of DnaCalc at the pinned commit; never run by hand.

use calc_func::value::{CalcValue, CoreRef, NumericMode};
use calc_func::{
    catalogue, help, invoke, ArgumentDemand, Availability, BindingForm, Deferral, EntryKind,
    EvalServices, FunctionCallScratch, HostRouteKind, Implementation, Notation, ParameterClass,
    PreparedArguments,
};
use std::io::BufRead;
use std::collections::BTreeMap;

struct Given(Vec<CalcValue>);

impl PreparedArguments for Given {
    fn count(&self) -> usize {
        self.0.len()
    }

    fn argument(&mut self, index: usize) -> CalcValue {
        self.0[index].clone()
    }
}

/// `eval`: each stdin line is a function and argument bits; each answer is printed as bits.
fn evaluate() {
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("stdin");
        let mut words = line.split_whitespace();
        let spelling = words.next().expect("a function");
        let arguments = words
            .map(|bits| {
                CalcValue::number(f64::from_bits(
                    u64::from_str_radix(bits.trim_start_matches("0x"), 16).expect("hex bits"),
                ))
            })
            .collect();
        let target = catalogue()
            .resolve_surface(Notation::Call, spelling)
            .expect("a catalogued function");
        let mode = NumericMode::Fast;
        let result = invoke(
            target,
            &mut Given(arguments),
            &mut EvalServices::new(&mode),
            &mut FunctionCallScratch::new(),
        );
        match result.as_ref().map(|value| value.core_ref()) {
            Ok(CoreRef::Number(n)) => println!("0x{:016x}", n.to_bits()),
            _ => println!("other"),
        }
    }
}

/// Summarize the full judge trace using the same witness verdict as DnaCalc.
fn bank_mismatches() {
    use calc_verdict::{decode_result, Severity};
    let mut counts: BTreeMap<String, BTreeMap<&str, u64>> = BTreeMap::new();
    for line in std::io::stdin().lock().lines() {
        let row: serde_json::Value = serde_json::from_str(&line.expect("trace input")).expect("trace JSON");
        let function = row["function"].as_str().expect("function");
        let actual = row["actual"].as_str().expect("actual");
        let expected = row["expected"].as_str().expect("expected");
        let function_counts = counts.entry(function.to_owned()).or_default();
        *function_counts.entry("trace_rows").or_default() += 1;
        if actual == expected {
            *function_counts.entry("matched").or_default() += 1;
            continue;
        }
        let kind = if actual.starts_with("excluded:") {
            "excluded"
        } else if matches!(actual, "fault:NotImplemented" | "fault:WitnessReferencePosition" |
                           "fault:Deferred" | "fault:NotHeld" | "fault:NotCatalogued") {
            "refused"
        } else if let (Ok(e), Ok(a)) = (decode_result(expected), decode_result(actual)) {
            let (severity, ulp) = Severity::of(&e, &a);
            if severity == Severity::Exact {
                *function_counts.entry("matched").or_default() += 1;
                continue;
            }
            match ulp {
                Some(distance) if distance <= 16 => "last_bit",
                Some(_) => "larger_numeric",
                None => "other_mismatch",
            }
        } else {
            "other_mismatch"
        };
        *function_counts.entry(kind).or_default() += 1;
    }
    println!("{}", serde_json::to_string(&counts).expect("summary JSON"));
}

fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("eval") => return evaluate(),
        Some("bank-mismatches") => return bank_mismatches(),
        _ => {}
    }
    let mut rows = Vec::new();
    // Every entry the catalogue declares, held or not: its own kind, implementation and
    // availability say what it is, so nothing here guesses from a name.
    for target in catalogue().entries() {
        let signature = target.signature_help();
        let parameters: Vec<String> = signature
            .parameters
            .iter()
            .enumerate()
            .map(|(index, p)| {
                let class = match target.parameter_class(index) {
                    Some(ParameterClass::Value) => "value",
                    Some(ParameterClass::ValueOrReference) => "value_or_reference",
                    Some(ParameterClass::Reference) => "reference",
                    None => "none",
                };
                format!(
                    "{{\"name\":{},\"optional\":{},\"repeats\":{},\"class\":{}}}",
                    quoted(p.name),
                    p.optional,
                    p.repeats,
                    quoted(class)
                )
            })
            .collect();
        let notation = match target.notation() {
            Notation::Call => "call",
            Notation::Prefix => "prefix",
            Notation::Infix => "infix",
            Notation::Postfix => "postfix",
        };
        let demand = match target.argument_demand() {
            ArgumentDemand::Eager => "eager",
            ArgumentDemand::BranchOnCondition => "branch_on_condition",
            ArgumentDemand::FallbackOnError => "fallback_on_error",
        };
        let kind = match target.kind() {
            EntryKind::Worksheet => "{\"type\":\"worksheet\"}".to_string(),
            EntryKind::Operator { file_spelling } => format!(
                "{{\"type\":\"operator\",\"file_spelling\":{}}}",
                file_spelling.map_or("null".to_string(), quoted)
            ),
            EntryKind::Binding(form) => format!(
                "{{\"type\":\"binding\",\"form\":{}}}",
                quoted(match form {
                    BindingForm::Let => "let",
                    BindingForm::Lambda => "lambda",
                })
            ),
            EntryKind::AddIn { library, export } => format!(
                "{{\"type\":\"addin\",\"library\":{},\"export\":{}}}",
                quoted(library),
                quoted(export)
            ),
            EntryKind::HostRoute { provider, route } => format!(
                "{{\"type\":\"host_route\",\"provider\":{},\"route\":{}}}",
                quoted(provider),
                quoted(&match route {
                    HostRouteKind::Fetch(kind) => format!("fetch {kind:?}"),
                    HostRouteKind::Import(kind) => format!("import {kind:?}"),
                    HostRouteKind::Rtd => "rtd".to_string(),
                })
            ),
        };
        let implementation = match target.implementation() {
            Implementation::Held => "{\"state\":\"held\"}".to_string(),
            Implementation::Binding => "{\"state\":\"binding\"}".to_string(),
            Implementation::NotHeld { owner } => {
                format!("{{\"state\":\"not_held\",\"owner\":{}}}", quoted(owner))
            }
            Implementation::Deferred { class } => format!(
                "{{\"state\":\"deferred\",\"class\":{},\"owner\":{}}}",
                quoted(match class {
                    Deferral::ExternalService => "external_service",
                    Deferral::Python => "python",
                    Deferral::ListsInCells => "lists_in_cells",
                }),
                quoted(class.owner())
            ),
        };
        let availability = match target.availability() {
            Availability::Unspecified => "unspecified",
            Availability::ListsInCellsBeta => "lists_in_cells_beta",
            Availability::AbsentOnWindows => "absent_on_windows",
            Availability::MacroSheetOnly => "macro_sheet_only",
        };
        let own = help::help_of(target.spelling()).map(|h| h.own);
        rows.push(format!(
            "{{\"id\":{},\"spelling\":{},\"notation\":{},\"min\":{},\"max\":{},\"display\":{},\"parameters\":[{}],\"demand\":{},\"resumable\":{},\"own_description\":{},\"kind\":{},\"implementation\":{},\"availability\":{}}}",
            target.id().raw(),
            quoted(target.spelling()),
            quoted(notation),
            target.arity().min,
            target.arity().max,
            quoted(&signature.display),
            parameters.join(","),
            quoted(demand),
            target.is_resumable(),
            own.map_or("null".to_string(), quoted),
            kind,
            implementation,
            quoted(availability),
        ));
    }
    println!("[\n{}\n]", rows.join(",\n"));
}
