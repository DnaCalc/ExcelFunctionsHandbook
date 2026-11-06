---
schema: efh.function-page/v1
function_id: FUNC.IMSQRT
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

# IMSQRT

## What it does

`IMSQRT(inumber)` returns the principal square root as complex text. Of the two mathematical roots it selects the one determined by half the principal argument.

## Try it

`=IMSQRT("8+6i")` is approximately `"3+i"`, since `(3+i)^2=8+6i`. Squaring a displayed decimal result may expose rounding in the components.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. The current magnitude graph is distinct from IMABS: squaring large components can overflow even if a stable mathematical square root would be finite. An overflowing sum of two finite component squares follows a separate recorded zero-result path.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

The compatibility reconstruction squares each component, flushing subnormal square results, adds the squares, and takes **two** square roots to obtain the fourth root of the sum. It takes half the argument and multiplies that radius by its cosine and sine. If both squares are finite but their sum overflows, the current code returns zero. If its argument helper reports zero/ratio errors, this square-root route substitutes angle zero. A scaled complex-square-root formula is a mathematical robustness alternative, with different recorded boundaries.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 2,029 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSQRT.json).

## Alternatives

[IMPOWER](FUNC.IMPOWER.md) accepts other exponents; [SQRT](FUNC.SQRT.md) handles nonnegative real inputs; [IMABS](FUNC.IMABS.md) computes magnitude.

## Sources

- [Microsoft: IMSQRT function](https://support.microsoft.com/en-us/office/imsqrt-function-e1753f80-ba11-4664-a10e-e17368396b70), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
