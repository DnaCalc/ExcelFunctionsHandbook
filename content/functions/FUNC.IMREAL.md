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

`=IMREAL("2-3i")` returns `2`; `=IMREAL("-j")` returns `0`. A numeric argument such as `=IMREAL(-7)` returns that real number. The extracted value can feed ordinary arithmetic without complex-text formatting.

## Quirks

The output is a worksheet number, even when the input is text. A missing real component means zero, not an error. Lowercase `i` and `j` identify the imaginary suffix; an uppercase or malformed spelling is rejected by the current parser. Logical values give `#VALUE!` in the wrapper. Text parsing keeps fifteen leading significant decimal digits, so extracting a long decimal coefficient need not preserve every typed digit.

## How to implement

Admit a numeric input as `(value,0)`. For text, parse the optional real component and optional imaginary component. A plus or minus in an exponent, such as the sign in `2e-3`, does not divide the two components. An omitted coefficient before `i` or `j` means one, and a leading minus means minus one.

Return the parsed real component directly as a number; do not round it through the complex output formatter. Thus `-j` parses as `(0,-1)` and returns zero, while `2-3i` parses as `(2,-3)` and returns two. The current decimal-text parser truncates after fifteen significant digits before converting to nearest binary64; a native language complex parser may use a different grammar or more digits.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMREAL.json).

## Alternatives

[IMAGINARY](FUNC.IMAGINARY.md) extracts the other coefficient. [COMPLEX](FUNC.COMPLEX.md) assembles coefficients into text, [IMABS](FUNC.IMABS.md) gives magnitude, and [IMARGUMENT](FUNC.IMARGUMENT.md) gives direction.

## Sources

- [Microsoft: IMREAL function](https://support.microsoft.com/en-us/office/imreal-function-d12bc4c0-25d0-4bb3-a25f-ece1938bf366), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
