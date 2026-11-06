---
schema: efh.function-page/v1
function_id: FUNC.FIELDVALUE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "FIELDVALUE function"
    locator: "https://support.microsoft.com/en-us/office/fieldvalue-function-4a579c45-7326-4168-b556-df8b5685175b"
    role: "public source read for this page"
  - work: "How to write formulas that reference data types"
    locator: "https://support.microsoft.com/en-US/Excel/how-to-write-formulas-that-reference-data-types"
    role: "public source read for this page"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# FIELDVALUE

## What it does

`FIELDVALUE` reads one named field from a cell holding a linked data type, such as a stock or a geography.

`FIELDVALUE(value, field_name)`

- **`value`** is the cell with the data type.
- **`field_name`** is the field, as text or from a cell.

`=FIELDVALUE(A1,"Price")` reads the same field as the dot form `=A1.Price`. The function form suits formulas where the
field name comes from a cell.

## Try it

## Quirks

- Microsoft's page says `#FIELD!` appears for a field that does not exist, a cell without the data type, or a record lacking
  that field.
- Recorded in Excel 16.0 build 20430: `FIELDVALUE` of a number, text, logical, error, blank cell or LAMBDA gives `#FIELD!`
  whatever the field name. An error given as the field name is the result.
- `IFERROR` around the call hides blanks, as in Microsoft's own example.

## How to implement

Our Rust implements the function's behaviour on the values DNA Calc holds. No value it holds is a linked data type, because
a workbook cell with one is not loaded. So it answers `#FIELD!` or the argument's error, and the fields themselves are out
of reach until linked data types load.

## Version differences

Answers recorded here come from Excel 16.0 build 20430. No comparison between builds exists.

## Alternatives

- The dot form, `=A1.Price`.
- `STOCKHISTORY` for past prices.

## Sources

- [FIELDVALUE function](https://support.microsoft.com/en-us/office/fieldvalue-function-4a579c45-7326-4168-b556-df8b5685175b)
- [How to write formulas that reference data types](https://support.microsoft.com/en-US/Excel/how-to-write-formulas-that-reference-data-types)
