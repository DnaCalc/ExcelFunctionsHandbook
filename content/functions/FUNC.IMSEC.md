---
schema: efh.function-page/v1
function_id: FUNC.IMSEC
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

# IMSEC

## What it does

`IMSEC(inumber)` returns the complex secant, mathematically the reciprocal of IMCOS. It returns complex text even if the imaginary component is zero.

## Try it

`=IMSEC("i")` gives real text about `"0.648054273663885"`, the reciprocal of the mathematical `cosh(1)`. This is less than one, although a real secant away from its poles has magnitude at least one.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. A zero denominator gives `#NUM!` in the current reciprocal helper. Near a pole, small changes in the input can cause large changes in the result.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

First compute the unformatted complex IMCOS pair `(c,d)`. If `abs(c)>=abs(d)`, use `r=d/c`, `v=1/(c+d*r)`, and return `(v,-r*v)`. Otherwise use `r=c/d`, `v=1/(d+c*r)`, and return `(r*v,-v)`. Reject `(0,0)` before division. The current reciprocal forms one reciprocal and multiplies a component by it; calling generic IMDIV with numerator one forms a different rounded graph. Keep this distinction when reproducing recorded results.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[IMCOS](FUNC.IMCOS.md) supplies the denominator; [IMDIV](FUNC.IMDIV.md) divides arbitrary complex inputs; [COMPLEX](FUNC.COMPLEX.md) creates a value from coefficients.

## Sources

- [Microsoft: IMSEC function](https://support.microsoft.com/en-us/office/imsec-function-6df11132-4411-4df4-a3dc-1f17372459e0), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
