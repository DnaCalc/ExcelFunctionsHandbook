---
schema: efh.function-page/v1
function_id: FUNC.CONFIDENCE
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

# CONFIDENCE

## What it does

`CONFIDENCE(alpha,standard_dev,size)` returns a normal-based confidence-interval **half-width**. Alpha is the excluded two-tail probability; standard_dev is a known positive population standard deviation; size is the observation count. It is the compatibility normal-confidence name.

## Try it

`=CONFIDENCE(0.05,10,100)` is about `1.96`. With a sample mean of fifty the corresponding arithmetic interval is about `48.04` to `51.96`; the function itself returns only the margin.

## Quirks

Alpha must be strictly between zero and one, standard deviation positive, and truncated size at least one, otherwise `#NUM!`. Nonnumeric arguments give `#VALUE!`. The returned margin does not incorporate a sample mean, and the normal-known-scale model differs from a Student-t margin.

## How to implement

1. Truncate size toward zero and validate `0<alpha<1`, positive standard deviation and `n>=1`.
2. Form `p=1-alpha/2` with a binary64 division and subtraction, then obtain the current standard-normal inverse at p.
3. Multiply that critical value by standard_dev, storing to binary64. Divide the stored product by the binary64 square root of n. The current order is `(critical*standard_dev)/sqrt(n)`, rather than multiplying critical by a precomputed standard error.

In the example n=100 has square root ten, so the margin is approximately `1.9599639845*10/10`, or 1.96. With sample mean fifty, add and subtract that margin outside the function.

For very small alpha, the stored probability expression can round to one even while alpha is positive. A tail-aware inverse can avoid that loss in a mathematical implementation, but changes the current operation tree. The normal inverse approximation and its return rounding remain separate compatibility obligations; a valid quantile algorithm alone is not evidence of Excel’s private method.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CONFIDENCE.NORM](FUNC.CONFIDENCE.NORM.md) is the modern normal spelling; [CONFIDENCE.T](FUNC.CONFIDENCE.T.md) uses the t critical value; [NORM.S.INV](FUNC.NORM.S.INV.md) exposes the normal quantile.

## Sources

- [Microsoft: CONFIDENCE function](https://support.microsoft.com/en-us/excel/functions/confidence-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [Current normal-confidence implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); `confidence_norm_kernel`, probability expression and product/division order.
