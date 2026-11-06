---
schema: efh.function-page/v1
function_id: FUNC.IMARGUMENT
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

# IMARGUMENT

## What it does

`IMARGUMENT(inumber)` returns the phase angle in radians of a complex number `a+bi`. Unlike functions that return complex text, this function returns a real worksheet number.

## Try it

`=IMARGUMENT("i")` returns approximately `1.5707963267949`. The input lies on the positive imaginary axis, a quarter turn from the positive real axis.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. The output here is numeric. The current code returns `#DIV/0!` at zero and when a nonzero real component makes the ratio `b/a` overflow. It uses a reduced arctangent of that ratio plus quadrant correction; replacing it by atan2 changes those compatibility boundaries.

## How to implement

Parse `inumber` into `a`, `b` and an optional suffix, then compute `the principal angle of (a,b)`. The current code returns `#DIV/0!` at zero and when a nonzero real component makes the ratio `b/a` overflow. It uses a reduced arctangent of that ratio plus quadrant correction; replacing it by atan2 changes those compatibility boundaries. Apply the worksheet error policy before returning a number. Separate lexical validation from the numerical operation so inputs like a real number, a pure imaginary unit and a full pair can share the same calculation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMARGUMENT.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs text from coordinates. [IMABS](FUNC.IMABS.md) returns the magnitude rather than the angle.

## Sources

- [Microsoft: IMARGUMENT function](https://support.microsoft.com/en-us/excel/functions/imargument-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
