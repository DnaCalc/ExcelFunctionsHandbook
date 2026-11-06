---
schema: efh.function-page/v1
function_id: FUNC.IMDIV
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

# IMDIV

## What it does

`IMDIV(inumber1, inumber2)` divides a complex numerator by a complex denominator and returns complex text.

## Try it

`=IMDIV("4+2i","1+i")` returns `"3-i"`: multiplying `3-i` by `1+i` recovers `4+2i`.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. A zero complex denominator gives `#NUM!` in the current implementation. Large components make the choice of division formula observable.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

Use the scaled division implemented in the product. For numerator `(a,b)` and denominator `(c,d)`, if `abs(c)>=abs(d)`, set `r=d/c`, `v=c+d*r`, and return `((a+b*r)/v,(b-a*r)/v)`. Otherwise set `r=c/d`, `v=d+c*r`, and return `((a*r+b)/v,(b*r-a)/v)`. This avoids directly forming `c*c+d*d`. In the example `r=1`, `v=2`, giving `(6/2,-2/2)=(3,-1)`. The code flushes subnormal output components before formatting. The retained witnesses distinguish this graph from the squared-denominator formula.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 2,067 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMDIV.json).

## Alternatives

[IMPRODUCT](FUNC.IMPRODUCT.md) multiplies; [IMCONJUGATE](FUNC.IMCONJUGATE.md) supplies the conjugate often used in a mathematical derivation of division.

## Sources

- [Microsoft: IMDIV function](https://support.microsoft.com/en-us/office/imdiv-function-a505aff7-af8a-4451-8142-77ec3d74d83f), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
