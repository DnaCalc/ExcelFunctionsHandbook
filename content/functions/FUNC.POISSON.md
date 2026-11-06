---
schema: efh.function-page/v1
function_id: FUNC.POISSON
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

# POISSON

## What it does

`POISSON(x,mean,cumulative)` is the compatibility distribution for an event count with a specified mean. FALSE returns exactly x events; TRUE returns at most x events.

## Try it

`=POISSON(0,2,FALSE)` is about `0.1353352832`, the zero-event probability `exp(-2)`. `=POISSON(1,2,TRUE)` is about `0.4060058497`, the sum of the zero- and one-event probabilities.

## Quirks

The count is truncated, but a negative fractional count is rejected before truncation in the current recorded route. Microsoft documents a negative mean as `#NUM!`; retained zero-count controls instead take EXP before that validation. This exception is input-route/build-scoped evidence, not permission to use a negative mean in the mathematical distribution.

## How to implement

Reject negative x, then truncate the count. The current zero-count branch returns measured public `EXP(-mean)` immediately and retains its subnormal result. Positive-count paths validate the mean; a zero mean yields mass zero or CDF one. General cumulative probability uses upper gamma ratio `Q(x+1,mean)`, rather than summing from an underflowed initial mass. Positive masses use the current saddle-point/deviance construction. Those producer stages remain partly unresolved. The zero-event example only needs the exponential stage.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[POISSON.DIST](FUNC.POISSON.DIST.md) is the modern spelling; [BINOM.DIST](FUNC.BINOM.DIST.md) fixes a finite number of Bernoulli trials; [EXPON.DIST](FUNC.EXPON.DIST.md) models waiting time.

## Sources

- [Microsoft: POISSON function](https://support.microsoft.com/en-us/excel/functions/poisson-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
