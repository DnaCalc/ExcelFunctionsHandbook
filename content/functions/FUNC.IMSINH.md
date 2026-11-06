---
schema: efh.function-page/v1
function_id: FUNC.IMSINH
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

# IMSINH

## What it does

`IMSINH(inumber)` returns the complex hyperbolic sine. Real and imaginary coefficients use radians where a circular angle is involved. A real-only input has a real-only result; an imaginary input relates to ordinary sine.

## Try it

`=IMSINH("0")` returns `"0"`. A real-only input has a real-only result; an imaginary input relates to ordinary sine.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Large hyperbolic factors can overflow before final cancellation. Denominator zeros and real-axis special cases need explicit handling.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For `z=a+bi`, the mathematical component formula is `sinh(a)*cos(b) + i*cosh(a)*sin(b)`. The current real-axis direct hyperbolic route enforces the exponential overflow domain and avoids an infinity-times-zero imaginary component. The formula is a derivation aid; the named runtime leaves and stores in the current source define its implementation candidate. Validate text output as well as numeric components, and keep unmeasured rounding stages open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSINH.json).

## Alternatives

[IMEXP](FUNC.IMEXP.md) expresses the exponential relation; [IMSIN](FUNC.IMSIN.md) is a related circular function; [COMPLEX](FUNC.COMPLEX.md) constructs the input.

## Sources

- [Microsoft: IMSINH function](https://support.microsoft.com/en-us/office/imsinh-function-dfb9ec9e-8783-4985-8c42-b028e9e8da3d), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
