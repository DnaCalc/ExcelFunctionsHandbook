---
schema: efh.function-page/v1
function_id: FUNC.BINOMDIST
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

# BINOMDIST

## What it does

`BINOMDIST(number_s, trials, probability_s, cumulative)` describes successes in a fixed number of independent trials with one success probability. FALSE returns exactly that many successes; TRUE returns at most that many. BINOMDIST is the compatibility name; BINOM.DIST makes the distribution role explicit.

## Try it

`=BINOMDIST(3,3,0.5,FALSE)` is about `0.125`. `=BINOMDIST(3,3,0.5,TRUE)` returns `1`, since all possible outcomes have at most three successes.

## Quirks

Counts are truncated toward zero. A probability outside zero to one, or an out-of-range success count, gives `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. “At least k” uses a complement at `k-1`, not at k. The all-success mass and its cumulative endpoint follow different paths.

## How to implement

After truncating successes `k` and trials `n` toward zero, validate `0<=k<=n` and `0<=p<=1`. The current recorded domain rejects `n>=2147483647` before selecting endpoint shortcuts. At `p=0` or `p=1`, handle the degenerate distribution explicitly; the recorded noncumulative `p=1,k=0,n>0` case gives `#NUM!`, a compatibility exception to the mathematical zero.

For the **all-success, noncumulative** route `k=n>0`, `0<p<1`:

1. Obtain the stored measured public [LN](FUNC.LN.md#how-to-implement) result for p, called `L`.
2. Compute `t=R(n*L)` with the [stored-arithmetic recipe](../model/09-staged-arithmetic.md), rounding once to a 64-bit significand and then storing to binary64.
3. Pass the stored `t` to the measured public [EXP route](FUNC.EXP.md#how-to-implement) and publish its result under the function’s finite/subnormal policy.

Worked `n=3,p=0.5`: `L` is about `-0.6931471805599453`, the stored product is about `-2.0794415416798357`, and this reconstruction’s exponential is `0.12500000000000003`, near the mathematical probability `0.125`. The stored intermediate values explain why the computation need not return the exactly representable mathematical result. A direct `pow(p,n)` has different intermediate stages and is not a substitute for this identified computation tree.

The stored product is distinguished by 300 numeric discriminants and 256 controls in the cited code. This does not solve interior masses, cumulative summation, or the separately recorded unresolved domain answers. The current interior implementation uses a saddle-point construction with Stirling/deviance components; its remaining mismatches are listed in the problem-functions report. A mathematically stable recurrence or incomplete-beta CDF is an alternative algorithm, not a claim about Excel’s internals.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[BINOM.INV](FUNC.BINOM.INV.md) finds a success-count quantile; [BINOM.DIST.RANGE](FUNC.BINOM.DIST.RANGE.md) covers an interval of counts; [HYPGEOM.DIST](FUNC.HYPGEOM.DIST.md) models sampling without replacement.

## Sources

- [Microsoft: BINOMDIST function](https://support.microsoft.com/en-us/excel/functions/binomdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
