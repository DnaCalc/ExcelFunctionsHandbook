---
schema: efh.function-page/v1
function_id: FUNC.ECMA.CEILING
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Excel functions (alphabetical)"
    locator: "https://support.microsoft.com/en-us/office/excel-functions-alphabetical-b3944572-255d-4efb-bb96-c6d90033e188"
    role: "public source read for this page"
  - work: "calc-func catalogue, crates/calc-func/src/catalogue/rounding.rs"
    locator: "https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/catalogue/rounding.rs"
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

# ECMA.CEILING

## What it does

`ECMA.CEILING` rounds a number up to a multiple of `significance`.

`ECMA.CEILING(number, significance)`

Excel accepts the name, and answers are recorded here, but it has no page of its own in Microsoft's function list.
In our Rust it shares the `CEILING` kernel.

## Try it

## Quirks

Our implementation treats a zero `number` or `significance` first, as `CEILING` does. The guide for
[CEILING](FUNC.CEILING.md) holds the sign and multiple rules.

## How to implement

Share one kernel with `CEILING`, and keep the entry so that workbooks using the name load and calculate.

## Version differences

Answers recorded here come from Excel 16.0 build 20527. No comparison between builds exists.

## Alternatives

- `CEILING`, `CEILING.MATH` and `CEILING.PRECISE`, which Microsoft documents.

## Sources

- [Excel functions (alphabetical)](https://support.microsoft.com/en-us/office/excel-functions-alphabetical-b3944572-255d-4efb-bb96-c6d90033e188)
- [calc-func catalogue, crates/calc-func/src/catalogue/rounding.rs](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/catalogue/rounding.rs)
