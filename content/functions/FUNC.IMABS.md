---
schema: efh.function-page/v1
function_id: FUNC.IMABS
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

# IMABS

## What it does

`IMABS(inumber)` returns the modulus of a complex number `a+bi`. Unlike functions that return complex text, this function returns a real worksheet number.

## Try it

`=IMABS("8+15i")` returns `17`, since `8^2+15^2=289` and its square root is seventeen. `=IMABS("-4i")` returns `4`: changing direction does not change distance from zero.

## Quirks

The result is numeric and nonnegative. Real-only and pure-imaginary inputs are valid; a negative coefficient does not make a negative modulus. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`.

Squaring each component naively can overflow even when the modulus is representable. The current implementation uses a runtime hypotenuse primitive. This differs from IMSQRT's separately observed sum-of-squares graph.

## How to implement

Parse the input as the pair `(a,b)`. A numeric scalar becomes `(a,0)`; complex text may use lowercase `i` or `j`. Return the runtime `hypot(a,b)` under the worksheet number-publication policy.

For a mathematical scaled implementation, take `m=max(abs(a),abs(b))` and `t=min(abs(a),abs(b))/m`. Return zero if `m=0`; otherwise compute `m*sqrt(1+t*t)`. This bounds the squared ratio by one. It avoids intermediate square overflow, although its rounding is not a substitute for every runtime hypot result. For `8+15i`, the scaled expression is `15*sqrt(1+(8/15)^2)`, about seventeen. The lexical parser and numerical primitive are separate parts of the implementation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMABS.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs text from coordinates. [IMARGUMENT](FUNC.IMARGUMENT.md) returns the phase angle.

## Sources

- [Microsoft: IMABS function](https://support.microsoft.com/en-us/excel/functions/imabs-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
