---
schema: efh.function-page/v1
function_id: FUNC.IMARGUMENT
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

# IMARGUMENT

## What it does

`IMARGUMENT(inumber)` returns the phase angle in radians of a complex number `a+bi`. Unlike functions that return complex text, this function returns a real worksheet number.

## Try it

`=IMARGUMENT("-1+i")` is about `2.356194490192345` radians, or three quarters of a half-turn. The negative real and positive imaginary components put the point in the second quadrant. `=IMARGUMENT("i")` is about `1.5707963267949`, a quarter turn.

## Quirks

Angles are in radians. Zero has no direction and the current kernel returns `#DIV/0!`. Its ratio-based route also gives `#DIV/0!` if `b/a` overflows for nonzero a, even though a mathematical principal angle exists. This differs from a general `atan2` implementation and from the fallback inside IMSQRT.

Complex text uses lowercase `i` or `j`. Malformed text gives `#NUM!` in the parser; a logical argument gives `#VALUE!` in the wrapper. The result here is a number, not complex text.

## How to implement

Parse the pair `(a,b)` and normalize signed zeros. If `a=0`, reject `b=0`; otherwise return `sign(b)*pi/2`. For nonzero a, compute the binary64 ratio `q=b/a` and reject an infinite ratio. Evaluate the current reduced scalar arctangent at q. If `a<0`, subtract pi when `b<0`, or add pi otherwise. Flush a subnormal angle to zero before numeric publication.

For `-1+i`, the ratio is minus one. Its reduced arctangent is near `-pi/4`; adding pi for the second quadrant gives `3*pi/4`. This is an explicit evaluation order from the current code. The arctangent approximation is a separate leaf; algebraically replacing the route with `atan2(b,a)` changes the recorded overflow boundary.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,209 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.IMARGUMENT.json).

## Alternatives

[COMPLEX](FUNC.COMPLEX.md) constructs text from coordinates. [IMABS](FUNC.IMABS.md) returns the magnitude rather than the angle.

## Sources

- [Microsoft: IMARGUMENT function](https://support.microsoft.com/en-us/excel/functions/imargument-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
