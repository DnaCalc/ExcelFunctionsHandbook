---
schema: efh.function-page/v1
function_id: FUNC.IMSECH
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

# IMSECH

## What it does

`IMSECH(inumber)` returns the complex hyperbolic secant, mathematically the reciprocal of IMCOSH. It returns complex text even if the imaginary component is zero.

## Try it

`=IMSECH("0")` returns `"1"`. The reciprocal reverses a pure imaginary value’s imaginary sign; it is not a component-by-component reciprocal.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. A zero denominator gives `#NUM!` in the current reciprocal helper. Near a pole, small changes in the input can cause large changes in the result.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

First compute the unformatted complex IMCOSH pair `(c,d)`. If `abs(c)>=abs(d)`, use `r=d/c`, `v=1/(c+d*r)`, and return `(v,-r*v)`. Otherwise use `r=c/d`, `v=1/(d+c*r)`, and return `(r*v,-v)`. Reject `(0,0)` before division. The current reciprocal forms one reciprocal and multiplies a component by it; calling generic IMDIV with numerator one forms a different rounded graph. Keep this distinction when reproducing recorded results.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSECH.json).

## Alternatives

[IMCOSH](FUNC.IMCOSH.md) supplies the denominator; [IMDIV](FUNC.IMDIV.md) divides arbitrary complex inputs; [COMPLEX](FUNC.COMPLEX.md) creates a value from coefficients.

## Sources

- [Microsoft: IMSECH function](https://support.microsoft.com/en-us/office/imsech-function-f250304f-788b-4505-954e-eb01fa50903b), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
