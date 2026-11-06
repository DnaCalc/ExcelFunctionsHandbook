---
schema: efh.function-page/v1
function_id: FUNC.IMCOSH
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

# IMCOSH

## What it does

`IMCOSH(inumber)` returns the complex hyperbolic cosine. Real and imaginary coefficients use radians where a circular angle is involved. A pure imaginary input has the real cosine as its mathematical result.

## Try it

`=IMCOSH("i")` gives real text about `"0.54030230586814"`, since the mathematical value is `cos(1)`. `=IMCOSH("0")` returns `"1"`. The pure imaginary example distinguishes hyperbolic cosine from IMCOS.

## Quirks

The result is text, including for a real-only answer. Malformed complex text gives `#NUM!` in the current parser and a logical argument gives `#VALUE!` in the wrapper.

On the real axis the current direct function applies the public EXP overflow domain to `abs(real)`, then calls the runtime hyperbolic primitive. It explicitly keeps the imaginary coordinate zero. IMSECH, its reciprocal companion, retains a wider runtime route, so computing the reciprocal of a separately formatted IMCOSH result need not reproduce it at the domain edge.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For `z=a+bi`, the mathematical component formula is `cosh(a)*cos(b) + i*sinh(a)*sin(b)`. The current real-axis direct hyperbolic route enforces the exponential overflow domain and avoids an infinity-times-zero imaginary component. The formula is a derivation aid; the named runtime leaves and stores in the current source define its implementation candidate. Validate text output as well as numeric components, and keep unmeasured rounding stages open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMCOSH.json).

## Alternatives

[IMSECH](FUNC.IMSECH.md) is the reciprocal hyperbolic function. [IMCOS](FUNC.IMCOS.md) and [IMSIN](FUNC.IMSIN.md) are the circular companions. [COMPLEX](FUNC.COMPLEX.md) constructs an input from two numeric coefficients.

## Sources

- [Microsoft: IMCOSH function](https://support.microsoft.com/en-us/excel/functions/imcosh-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
