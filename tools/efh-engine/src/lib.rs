//! Small JSON transport over OneCalc's existing Session, shared by both hosts.
use calc_func::{
    locale::{CalcEnvironment, DateSystem, FormatProfile},
    value::{CalcValue, CoreValue, WorksheetErrorCode},
};
use dna_onecalc::{InputValue, Session, display, result};
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

fn input(value: &Value) -> Result<InputValue, String> {
    match value {
        Value::Null => Ok(InputValue::blank()),
        Value::Bool(v) => Ok(InputValue::logical(*v)),
        Value::Number(v) => InputValue::number(v.as_f64().ok_or("invalid number")?).map_err(|e| format!("{e:?}")),
        Value::String(v) => InputValue::text(v).map_err(|e| format!("{e:?}")),
        Value::Object(v) if v.len()==1 && v.contains_key("error") => {
            let spelling=v["error"].as_str().ok_or("error must be a worksheet error spelling")?;
            let code=WorksheetErrorCode::all().into_iter().find(|c|c.excel_spelling()==spelling).ok_or("unknown worksheet error")?;
            Ok(InputValue::error(code))
        },
        _ => Err("input cells accept a number, string, boolean, null or {error: worksheet error spelling}".into()),
    }
}

fn scalar(value: &CalcValue) -> Value {
    let mut v = json!({"kind":result::kind(value),"display":result::plain(value)});
    if let CoreValue::Number(n) = value.core() {
        v["number"] = json!(n);
        v["bits"] = json!(format!("0x{:016x}", n.to_bits()));
    }
    if let CoreValue::Logical(b) = value.core() {
        v["logical"] = json!(b);
    }
    v
}

fn calculate(request: &Value) -> Result<Value, String> {
    if !request.is_object() {
        return Err("request must be a JSON object".into());
    }
    let formula = request["formula"]
        .as_str()
        .ok_or("formula must be a string")?;
    if formula.trim().is_empty() || formula.len() > 4096 {
        return Err("formula must contain 1 to 4096 UTF-8 bytes".into());
    }
    let mut depth = 0usize;
    let mut quote = None;
    let mut chars = formula.chars().peekable();
    while let Some(ch) = chars.next() {
        if let Some(delimiter) = quote {
            if ch == delimiter {
                if chars.peek() == Some(&delimiter) { chars.next(); }
                else { quote = None; }
            }
            continue;
        }
        if ch == '\"' || ch == '\'' { quote = Some(ch); }
        else if ch == '(' {
            depth += 1;
            if depth > 64 { return Err("formula nesting exceeds 64".into()); }
        } else if ch == ')' { depth = depth.saturating_sub(1); }
    }
    let mut session = Session::new("Sheet1").map_err(|e| format!("{e:?}"))?;
    let mut env = CalcEnvironment::dates(match request["date_system"].as_str().unwrap_or("1900") {
        "1900" => DateSystem::System1900,
        "1904" => DateSystem::System1904,
        _ => return Err("date_system must be 1900 or 1904".into()),
    });
    env.profile = Some(FormatProfile::for_culture("en-US").ok_or("en-US profile unavailable")?);
    if let Some(now) = request.get("now") {
        let now = now
            .as_f64()
            .filter(|v| v.is_finite())
            .ok_or("now must be a finite Excel date serial")?;
        env.now = Some(now);
    }
    env.compatibility_version = match request.get("compatibility_version") {
        None => 1,
        Some(v) => u32::try_from(
            v.as_u64()
                .ok_or("compatibility_version must be a positive integer")?,
        )
        .ok()
        .filter(|v| *v > 0)
        .ok_or("compatibility_version must be a positive integer")?,
    };
    session.set_environment(Some(env));
    if let Some(inputs) = request.get("inputs") {
        let inputs = inputs
            .as_object()
            .ok_or("inputs must be an object keyed by cell address")?;
        if inputs.len() > 256 {
            return Err("at most 256 input cells are allowed".into());
        }
        for (address, v) in inputs {
            session
                .set_input(address, input(v)?)
                .map_err(|e| display::input_error(&session, &e))?;
        }
    }
    if let Some(udf) = request.get("udf") {
        if udf["language"] != "vba" {
            return Err("the OneCalc UDF hook currently accepts language vba".into());
        }
        let source = udf["source"]
            .as_str()
            .filter(|s| s.len() <= 12000)
            .ok_or("udf.source must be at most 12000 bytes")?;
        session.load_vba_module("Handbook", source, 10000)?;
    }
    session.set_formula(formula);
    let calc = session.calculate();
    let diagnostics: Vec<_> = calc
        .diagnostics
        .iter()
        .map(|d| display::diagnostic(&session, d))
        .collect();
    let Some(value) = calc.result.as_ref() else {
        return Ok(json!({"status":"refused","diagnostics":diagnostics}));
    };
    let mut value_json = scalar(value);
    if let Some(array) = value.as_array() {
        let shape = array.shape();
        let count = shape
            .rows
            .checked_mul(shape.cols)
            .ok_or("result shape overflow")?;
        let cells: Vec<_> = (0..shape.rows.min(32))
            .map(|r| {
                (0..shape.cols.min(16))
                    .map(|c| scalar(array.get(r, c).expect("inside array")))
                    .collect::<Vec<_>>()
            })
            .collect();
        value_json["rows"] = json!(shape.rows);
        value_json["columns"] = json!(shape.cols);
        value_json["cells"] = json!(cells);
        value_json["truncated"] = json!(count > 512 || shape.rows > 32 || shape.cols > 16);
    }
    Ok(
        json!({"status":if result::kind(value)=="error" {"worksheet-error"} else {"calculated"},"value":value_json,"diagnostics":diagnostics}),
    )
}

#[wasm_bindgen]
pub fn evaluate(request_json: &str) -> String {
    let outcome = serde_json::from_str::<Value>(request_json)
        .map_err(|e| format!("invalid JSON: {e}"))
        .and_then(|request| calculate(&request));
    let mut response =
        outcome.unwrap_or_else(|message| json!({"status":"invalid-request","message":message}));
    response["engine"] = json!("OneCalc");
    response["engine_revision"] = json!(env!("EFH_ENGINE_REVISION"));
    response.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(formula: &str) -> Value {
        serde_json::from_str(&evaluate(&json!({"formula":formula}).to_string())).unwrap()
    }
    #[test]
    fn actual_onecalc_session_preserves_values_errors_arrays_and_refusals() {
        assert_eq!(run("=SUM(1,2,3)")["value"]["number"], 6.0);
        assert_eq!(run("=1/0")["status"], "worksheet-error");
        assert_eq!(run("=UNKNOWNHANDBOOK(1)")["value"]["display"], "#NAME?");
        assert_eq!(run("=B2")["status"], "refused");
        let array = run("=SEQUENCE(2,3)");
        assert_eq!(array["value"]["cells"][1][2]["number"], 6.0);
        let cells: Value = serde_json::from_str(&evaluate(
            r#"{"formula":"=SUM(B2:B3)","inputs":{"B2":3,"B3":4}}"#,
        ))
        .unwrap();
        assert_eq!(cells["value"]["number"], 7.0);
    }
    #[test]
    fn parentheses_in_excel_strings_are_not_formula_nesting() {
        let request = json!({"formula":format!("=LEN(\"{}\")", "(".repeat(100))});
        let value:Value = serde_json::from_str(&evaluate(&request.to_string())).unwrap();
        assert_eq!(value["value"]["number"],100.0);
        assert_eq!(run(&format!("={}1{}", "(".repeat(65), ")".repeat(65)))["status"],"invalid-request");
    }
    #[test]
    fn transport_rejects_invalid_inputs_instead_of_substituting_values() {
        assert_eq!(run("")["status"], "invalid-request");
        assert_eq!(run("=B2")["status"], "refused");
        let v: Value =
            serde_json::from_str(&evaluate(r#"{"formula":"=1","inputs":{"A1":3}}"#)).unwrap();
        assert_eq!(v["status"], "invalid-request");
    }
}
