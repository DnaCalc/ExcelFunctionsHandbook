---
schema: efh.function-page/v1
function_id: FUNC.IMEXP
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

# IMEXP

## What it does

`IMEXP(inumber)` returns e raised to a complex power, represented as complex text. Its real input coefficient controls magnitude; the imaginary coefficient controls angle in radians.

## Try it

`=IMEXP("0")` returns `"1"`. Mathematically `IMEXP("i")` has components `cos(1)` and `sin(1)`, approximately `0.54030230586814+0.841470984807897i`.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. Large positive real coefficients can overflow the exponential; large negative ones can underflow. Values at imaginary multiples of a mathematical quarter turn still depend on the floating-point argument that actually arrived.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For `a+bi`, compute `u=exp(a)`, then the pair `(u*cos(b),u*sin(b))`. This is the current runtime-leaf composition. At zero, `u=1`, cosine is one and sine zero, so formatting returns text one. Keep component nonfinite checks and zero handling before text assembly; an arbitrary language complex-library string is not an Excel representation.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[IMLN](FUNC.IMLN.md) is the principal logarithm; [IMPOWER](FUNC.IMPOWER.md) accepts a real exponent.

## Sources

- [Microsoft: IMEXP function](https://support.microsoft.com/en-us/office/imexp-function-c6f8da1f-e024-4c0c-b802-a60e7147a95f), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
