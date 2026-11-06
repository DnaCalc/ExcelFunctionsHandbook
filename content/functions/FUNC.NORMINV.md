---
schema: efh.function-page/v1
function_id: FUNC.NORMINV
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

# NORMINV

## What it does

`NORMINV(probability,mean,standard_dev)` is the compatibility normal quantile: it returns the value with the requested cumulative probability for the supplied mean and positive standard deviation.

## Try it

`=NORMINV(0.5,10,2)` returns `10`. The median of this symmetric distribution equals its mean.

## Quirks

The documented probability interval is strictly between zero and one. Endpoints and a nonpositive standard deviation give `#NUM!`; nonnumeric arguments give `#VALUE!`. A tiny p refers to the far **left** tail, not the upper-tail critical value.

## How to implement

Validate the probability and scale. The current standard inverse uses Acklam’s piecewise rational approximation, then one Newton correction based on a direct complementary-error tail and normal density. Use a lower-tail residual for negative candidates and an upper-tail residual for positive candidates to avoid cancellation at one. Return `mean+sigma*z` under the current binary64 combination. At p=0.5 the symmetric candidate is zero. This is a published-method candidate: the generic inverse’s Excel coefficients and stopping/rounding stages are still open, even though the median control is exact.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[NORM.INV](FUNC.NORM.INV.md) is the modern spelling; [NORMDIST](FUNC.NORMDIST.md) gives the forward probability.

## Sources

- [Microsoft: NORMINV function](https://support.microsoft.com/en-us/excel/functions/norminv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
