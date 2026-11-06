---
schema: efh.function-page/v1
function_id: FUNC.IMPOWER
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

# IMPOWER

## What it does

`IMPOWER(inumber, number)` raises a complex value to a real exponent, including integer, fractional and negative exponents. The result uses a principal angle for fractional powers.

## Try it

`=IMPOWER("2+i",2)` is approximately `"3+4i"`: algebraically `(2+i)^2=4+4i-1`. A fractional exponent chooses one branch rather than returning every mathematical root.

## Quirks

Complex arguments may be real numbers or text such as `"2-3i"`. Use lowercase `i` or `j`; operands in one call must agree on the suffix. The current parser rejects malformed complex text with `#NUM!` and logical values with `#VALUE!`. A complex result is text, so ordinary worksheet arithmetic does not automatically treat it as a pair of numeric components. A nonnumeric exponent gives `#VALUE!`. The current zero rules return one for exponent zero, zero for a positive exponent and `#NUM!` for a negative exponent.

## How to implement

Parse the two coefficients and suffix before doing arithmetic; use the common complex formatter afterward. Retain suffix consistency, suppress a zero component and omit the coefficient one before the imaginary unit. Formatting and parsing have their own fifteen-significant-digit rules, so testing only the algebra misses observable behaviour.

For nonzero input compute `h=hypot(a,b)`, `theta=atan2(b,a)`, `u=pow(h,p)` and `angle=theta*p`. Form `(u*cos(angle),u*sin(angle))` and format. Even an integer exponent takes this polar route in the current kernel, so repeated multiplication can have different last bits and different text. Retain the explicit zero cases before computing a logarithm or phase.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[IMSQRT](FUNC.IMSQRT.md) is the square-root operation; [IMPRODUCT](FUNC.IMPRODUCT.md) is useful for explicit integer products; [POWER](FUNC.POWER.md) handles a real base.

## Sources

- [Microsoft: IMPOWER function](https://support.microsoft.com/en-us/office/impower-function-210fd2f5-f8ff-4c6a-9d60-30e34fbdef39), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/complex_family.rs); implementation steps and the limits identified in the text.
