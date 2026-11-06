---
schema: efh.function-page/v1
function_id: FUNC.IMSIN
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

# IMSIN

## What it does

`IMSIN(inumber)` returns the complex sine. Real and imaginary coefficients use radians where a circular angle is involved. For pure imaginary `bi`, the mathematical result is `i*sinh(b)`.

## Try it

`=IMSIN("i")` gives about `"1.1752011936438i"`. With zero real coordinate the real sine component vanishes, leaving `i*sinh(1)`. `=IMSIN("0")` returns text zero.

## Quirks

The input may be numeric or complex text with lowercase `i` or `j`; the result is complex text. The current parser rejects malformed text with `#NUM!` and the wrapper rejects logical values with `#VALUE!`.

The current circular helper has a real-coordinate magnitude boundary at `134217728`. The imaginary coordinate instead feeds an exponential and its reciprocal. Large coordinates can fail in either leaf; replacing the pair with independent native sine and hyperbolic calls changes the implementation's rounding history.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For `z=a+bi`, the mathematical component formula is `sin(a)*cosh(b) + i*cos(a)*sinh(b)`. The current sine/cosine route forms `exp(b)` using the worksheet exponential and a stored reciprocal for `exp(-b)`, then forms the half-sum and half-difference. Independent sinh/cosh calls need not share that rounding history. The formula is a derivation aid; the named runtime leaves and stores in the current source define its implementation candidate. Validate text output as well as numeric components, and keep unmeasured rounding stages open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMSIN.json).

## Alternatives

[IMEXP](FUNC.IMEXP.md) expresses the exponential relation; [IMCOS](FUNC.IMCOS.md) is a related circular function; [COMPLEX](FUNC.COMPLEX.md) constructs the input.

## Sources

- [Microsoft: IMSIN function](https://support.microsoft.com/en-us/office/imsin-function-1ab02a39-a721-48de-82ef-f52bf37859f6), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
