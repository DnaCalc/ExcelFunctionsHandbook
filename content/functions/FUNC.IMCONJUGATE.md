---
schema: efh.function-page/v1
function_id: FUNC.IMCONJUGATE
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

# IMCONJUGATE

## What it does

`IMCONJUGATE(inumber)` reflects a complex number across the real axis, changing the sign of the imaginary coefficient and keeping the real coefficient.

## Try it

`=IMCONJUGATE("2-3i")` returns `"2+3i"`. Multiplying those two inputs mathematically gives thirteen, the square of their modulus.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. A real-only input is unchanged by conjugation.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

The arithmetic step is just `(a,b) → (a,-b)`. Normalise signed zero when assembling text. This operation needs no transcendental approximation; nevertheless, conversion to and from complex text can affect long decimal coefficients. Test the textual route separately from a mathematical pair operation.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 3,245 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMCONJUGATE.json).

## Alternatives

[IMREAL](FUNC.IMREAL.md) and [IMAGINARY](FUNC.IMAGINARY.md) expose the parts; [IMABS](FUNC.IMABS.md) measures the distance to zero.

## Sources

- [Microsoft: IMCONJUGATE function](https://support.microsoft.com/en-us/excel/functions/imconjugate-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
