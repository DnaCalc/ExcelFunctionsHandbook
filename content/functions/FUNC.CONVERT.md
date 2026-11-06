---
schema: efh.function-page/v1
function_id: FUNC.CONVERT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
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

# CONVERT

## What it does

`CONVERT(number, from_unit, to_unit)` converts a measured quantity between supported units of the same dimension. Unit names are text codes, not the labels shown by cell formatting.

## Try it

`=CONVERT(3,"ft","in")` returns `36`. `=CONVERT(32,"F","C")` returns zero: temperature conversion needs an offset as well as a scale.

## Quirks

Unit codes and prefixes are case-sensitive. Unknown units, unsupported binary prefixes and incompatible dimensions give `#N/A`; wrong data types give `#VALUE!`. For example, a mass unit cannot be converted to a distance unit just because both have numeric values.

## How to implement

Parse a unit code into a dimension, scale and any offset. Check that dimensions agree, apply the source transform into a canonical unit and then invert the destination transform. For foot-to-inch conversion the factor is twelve; for Fahrenheit-to-Celsius it is `(x-32)*5/9`. Prefix parsing must distinguish supported decimal prefixes, binary prefixes and unit names. Use a reviewed explicit table rather than guessing from spelling. Exact conversion factors alone do not identify Excel’s floating-point operation order for every unit pair.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit, CPU not recorded | 33,972 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CONVERT.json).

## Alternatives

[EUROCONVERT](FUNC.EUROCONVERT.md) handles its specified currency table. Direct multiplication is clear for a single known factor, but does not validate units or handle offsets.

## Sources

- [Microsoft: CONVERT function](https://support.microsoft.com/en-us/excel/functions/convert-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/misc_conversion_family.rs); implementation steps and the limits identified in the text.
