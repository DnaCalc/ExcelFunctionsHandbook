---
schema: efh.function-page/v1
function_id: FUNC.IMREAL
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

# IMREAL

## What it does

`IMREAL(inumber)` returns the real coefficient of a complex number `a+bi`. Unlike functions that return complex text, this function returns a real worksheet number.

## Try it

`=IMREAL("2-3i")` returns `2`. The coefficient retains its sign; the other component does not affect this extraction.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. The output here is numeric. The coefficient is a worksheet number, so it can feed ordinary numerical functions.

## How to implement

Parse `inumber` into `a`, `b` and an optional suffix, then compute `a`. The coefficient is a worksheet number, so it can feed ordinary numerical functions. Apply the worksheet error policy before returning a number. Separate lexical validation from the numerical operation so inputs like a real number, a pure imaginary unit and a full pair can share the same calculation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMREAL.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs text from coordinates. [IMARGUMENT](https://handbook.dnacalc.com/functions/IMARGUMENT.html) returns the phase angle.

## Sources

- [Microsoft: IMREAL function](https://support.microsoft.com/en-us/office/imreal-function-d12bc4c0-25d0-4bb3-a25f-ece1938bf366), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
