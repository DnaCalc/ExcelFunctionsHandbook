---
schema: efh.function-page/v1
function_id: FUNC.BESSELY
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

# BESSELY

## What it does

`BESSELY(x,n)` evaluates the order-n Bessel function of the second kind. It is a special function used in radial and oscillatory problems, rather than an elementary trigonometric function. The order is truncated to an integer.

## Try it

`=BESSELY(2,0)` evaluates the order-zero function at two. The mathematical value is about `0.51037567`; `=BESSELY(0,0)` is outside the current positive-x domain and gives `#NUM!`.

## Quirks

Nonnumeric arguments give `#VALUE!`; a negative order gives `#NUM!`. The current real-valued kernel requires positive x. Recorded code also limits truncated order to 10,000,000 and applies phase-based large-argument admission. Published approximation methods remain distinct from an identification of Excel’s numerical body.

## How to implement

Validate x and the order before selecting a numerical route. Use order-zero and order-one seed approximations. For higher orders the recurrence is `C_(j+1)=(2*j/x)*C_j-C_(j-1)`. The current code runs forward from its Y0 and Y1 seeds. For example the recurrence gives `C2=(2/x)*C1-C0`; its numbers depend on both seed values. Seed coefficients, phase grouping and flushing are separate compatibility questions, with residual differences still recorded. This is a published-method implementation outline, not a claim that every Excel stage is solved.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[BESSELI](FUNC.BESSELI.md) and [BESSELK](FUNC.BESSELK.md) are modified Bessel functions. [BESSELJ](FUNC.BESSELJ.md) supplies the other ordinary kind.

## Sources

- [Licensed seed and backward helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bessel_rational.rs).
- [Microsoft: BESSELY function](https://support.microsoft.com/en-us/excel/functions/bessely-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bessel.rs); implementation steps and the limits identified in the text.
