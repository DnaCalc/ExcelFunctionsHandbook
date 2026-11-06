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

Truncate the sample size, validate the parameters, find the standard-normal quantile at `1-alpha/2`, and multiply it by `sigma/sqrt(n)` under the chosen operation order. In the example `sigma/sqrt(n)=10/10=1`, leaving the critical value, approximately 1.96. The generic normal inverse and its return rounding remain separate compatibility obligations; a mathematically valid quantile routine is not evidence of Excel’s private inverse method.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CONFIDENCE.NORM](FUNC.CONFIDENCE.NORM.md) is the modern normal spelling; [CONFIDENCE.T](FUNC.CONFIDENCE.T.md) uses the t critical value; [NORM.S.INV](FUNC.NORM.S.INV.md) exposes the normal quantile.

## Sources

- [Microsoft: CONFIDENCE function](https://support.microsoft.com/en-us/excel/functions/confidence-function), opened 6 November 2026; syntax, documented argument rules and errors.
