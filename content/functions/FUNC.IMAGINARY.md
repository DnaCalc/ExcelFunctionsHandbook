---
schema: efh.function-page/v1
function_id: FUNC.IMAGINARY
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

# IMAGINARY

## What it does

`IMAGINARY(inumber)` returns the imaginary coefficient of a complex number `a+bi`. Unlike functions that return complex text, this function returns a real worksheet number.

## Try it

`=IMAGINARY("2-3i")` returns `-3`. The coefficient retains its sign; the other component does not affect this extraction.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. The output here is numeric. The result is the signed coefficient, without the imaginary unit; a real-only input has imaginary coefficient zero.

## How to implement

Parse `inumber` into `a`, `b` and an optional suffix, then compute `b`. The result is the signed coefficient, without the imaginary unit; a real-only input has imaginary coefficient zero. Apply the worksheet error policy before returning a number. Separate lexical validation from the numerical operation so inputs like a real number, a pure imaginary unit and a full pair can share the same calculation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMAGINARY.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs text from coordinates. [IMARGUMENT](https://handbook.dnacalc.com/functions/IMARGUMENT.html) returns the phase angle.

## Sources

- [Microsoft: IMAGINARY function](https://support.microsoft.com/en-us/excel/functions/imaginary-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
