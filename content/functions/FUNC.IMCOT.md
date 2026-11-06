---
schema: efh.function-page/v1
function_id: FUNC.IMCOT
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

# IMCOT

## What it does

`IMCOT(inumber)` returns the complex cotangent. Real and imaginary coefficients use radians where a circular angle is involved. The denominator vanishes at zero; the current function gives `#NUM!` there.

## Try it

`=IMCOT("i")` gives about `"-1.31303528549933i"`. On this axis it is mathematically `-i*coth(1)`. `=IMCOT("0")` has a zero denominator and the current function returns `#NUM!`.

## Quirks

The result is complex text and lowercase `i` or `j` is supported. Malformed complex text gives `#NUM!` in the current parser; a logical argument gives `#VALUE!` in the wrapper. Zero is a pole.

For exactly real input the current helper takes a stored reciprocal of scalar tangent. It returns text zero at the finite circular-domain boundary where IMTAN gives `#NUM!`. For non-real inputs it uses a doubled-angle formula, whose denominator can vanish or overflow. These are separate compatibility branches.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For `z=a+bi`, the mathematical component formula is `sin(2*a)/(cosh(2*b)-cos(2*a)) - i*sinh(2*b)/(cosh(2*b)-cos(2*a))`. Use the doubled-angle graph for non-real input. On the real axis the current tangent uses its scalar tangent leaf, while cotangent uses one stored reciprocal; their domain-boundary outputs are not identical. The formula is a derivation aid; the named runtime leaves and stores in the current source define its implementation candidate. Validate text output as well as numeric components, and keep unmeasured rounding stages open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMCOT.json).

## Alternatives

[IMTAN](FUNC.IMTAN.md) returns tangent. [IMDIV](FUNC.IMDIV.md) handles general complex division, while [COT](FUNC.COT.md) accepts a real angle.

## Sources

- [Microsoft: IMCOT function](https://support.microsoft.com/en-us/excel/functions/imcot-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
