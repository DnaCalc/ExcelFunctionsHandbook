---
schema: efh.function-page/v1
function_id: FUNC.WEIBULL
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

# WEIBULL

## What it does

`WEIBULL(x,alpha,beta,cumulative)` returns a Weibull CDF (TRUE) or density (FALSE). Alpha is a positive shape and beta a positive scale. This is the compatibility name.

## Try it

`=WEIBULL(4,2,2,TRUE)` is about `0.9816843611`: `(4/2)^2=4`, so the CDF is `1-exp(-4)`. Shape one reduces mathematically to an exponential distribution with rate `1/beta`.

## Quirks

Negative x or nonpositive shape/scale gives `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. The x=0 density depends on shape: it can diverge, be finite or be zero, so do not evaluate an undefined generic power blindly.

## How to implement

For positive x, form `t=(x/beta)^alpha`. The CDF is `-expm1(-t)`, using a stable expm1 implementation for small t. The density is `(alpha/beta)*(x/beta)^(alpha-1)*exp(-t)`; a log-domain version can avoid overflowing intermediate powers. Handle x=0 limits and subnormal publication explicitly. The current compatibility implementation’s leaves and stores need their own evidence; the mathematical exponential identity does not identify a whole Weibull route.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[WEIBULL.DIST](FUNC.WEIBULL.DIST.md) is the modern spelling; [EXPON.DIST](FUNC.EXPON.DIST.md) is the shape-one special family; [GAMMA.DIST](FUNC.GAMMA.DIST.md) supplies another positive-valued distribution.

## Sources

- [Microsoft: WEIBULL function](https://support.microsoft.com/en-us/excel/functions/weibull-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
