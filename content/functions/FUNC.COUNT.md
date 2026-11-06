---
schema: efh.function-page/v1
function_id: FUNC.COUNT
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

# COUNT

## What it does

`COUNT(value1, [value2], ...)` counts numeric entries. It measures how many numbers are present, rather than adding their values. Dates stored as Excel serial numbers count as numbers.

## Try it

`=COUNT(8,"12",TRUE)` returns `3` under the documented direct-argument rules. With A1:A4 containing `8`, the text `12`, `TRUE`, and an empty cell, `=COUNT(A1:A4)` returns `1`.

## Quirks

References and arrays count numbers only; their text, logical values, blanks and errors do not count. Direct numeric text and logical constants can count. Consequently COUNT is not a nonempty-cell count, and a formula returning an empty string is text.

## How to implement

Visit every argument while preserving its direct/reference origin. For a referenced or array element increment only for numeric values. For a direct scalar also admit the documented logical and numeric-text cases. Do not increment for an error or unconvertible text. Use an integer accumulator and return the count as a worksheet number. The direct example admits three values even though only one is a numeric literal.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,212 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.COUNT.json).

## Alternatives

[COUNTA](FUNC.COUNTA.md) counts nonempty values; [COUNTBLANK](FUNC.COUNTBLANK.md) counts blanks; [COUNTIF](FUNC.COUNTIF.md) and [COUNTIFS](FUNC.COUNTIFS.md) count matches.

## Sources

- [Microsoft: COUNT function](https://support.microsoft.com/en-us/excel/functions/count-function), opened 6 November 2026; syntax, documented argument rules and errors.
