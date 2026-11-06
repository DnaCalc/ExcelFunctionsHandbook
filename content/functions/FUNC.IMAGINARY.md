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

`=IMAGINARY("2-3i")` returns `-3`; `=IMAGINARY("-j")` returns `-1`. A real-only input such as `=IMAGINARY(7)` returns zero. The answer is the signed coefficient, without its suffix.

## Quirks

A pure imaginary unit has an implicit coefficient of one; `-i` and `-j` have coefficient minus one. A real-only input has coefficient zero. The current parser rejects malformed text with `#NUM!` and the wrapper rejects logical values with `#VALUE!`. Fifteen-leading-digit text conversion is separate from the numeric result, so long coefficient strings need their own parsing tests.

## How to implement

Admit a numeric scalar as `(value,0)`. For text, recognize lowercase `i` or `j`, locate the component separator without splitting an exponent sign, and parse the two decimal coefficients. An absent real part means zero; a suffix with no magnitude means one, with its written sign.

Return the imaginary coefficient as a numeric value. For `2-3i`, the parsed pair is `(2,-3)` and the output is minus three. For `-j`, the pair is `(0,-1)`. No multiplication, modulus or complex-text formatting is needed after parsing. Use the source parser's fifteen-leading-digit conversion when reproducing its text route, rather than silently admitting the full precision of a different language parser.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMAGINARY.json).

## Alternatives

[IMREAL](FUNC.IMREAL.md) extracts the real coefficient. [COMPLEX](FUNC.COMPLEX.md) assembles a pair, while [IMCONJUGATE](FUNC.IMCONJUGATE.md) changes the imaginary sign and returns complex text.

## Sources

- [Microsoft: IMAGINARY function](https://support.microsoft.com/en-us/excel/functions/imaginary-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
