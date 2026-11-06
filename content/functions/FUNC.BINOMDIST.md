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

The all-success stored product is distinguished by 300 numeric discriminants and 256 controls in the cited code. Its scope does not cover the zero-success producer, interior masses or every separately recorded domain answer.

The later R31 source isolates additional **interior-mass stages**. These start from Loader’s published saddle-point form, then preserve separately measured arithmetic sites. Let `D` mean ordinary binary64 storage and `E` resident 64-bit-significand arithmetic; their [portable definitions](../model/09-staged-arithmetic.md#retain-an-extended-intermediate-when-the-recipe-requires-it) keep E distinct from R.

1. For `0<k<n`, set `q=D(1-p)`. Obtain Stirling correction s and deviance d from the explicit [Poisson helper recipes](FUNC.POISSON.DIST.md#how-to-implement).
2. Form `c=R(s(n)-s(k))`, then successively `c=R(c-d(k,R(n*p)))`, `c=R(c-s(n-k))` and `c=R(c-d(n-k,R(n*q)))`. Keep this subtraction order. The source records 13,000 controls across six `(n,k)` pairs for that exponent grid.
3. Form the ordered binary64 terms `[-LN(n)/2,LN(k)/2,LN(n-k)/2,0.9189385332046728]`. Their mathematical effect supplies `sqrt(n/(2*pi*k*(n-k)))` without directly forming that ratio.
4. Initialize stored a and resident b from c. For each term, update `a=R(a-term)` and separately `b=E(b-term)`, retaining b across the whole list. Compute `lost=D(E(b-a))`, `factor=R(1+lost)`, and return `R(EXP(a)*factor)`.

BINOM uses one correction for the running resident difference. POISSON’s prefactor instead keeps one correction factor for each of its two subtractions. They are distinct reconstructions even though both compensate a stored exponent.

For `k=3,n=10,p=0.25`, the stated interior reconstruction gives `0.2502822875976563`. Direct exact rational evaluation of `120*(1/4)^3*(3/4)^7` gives the representable mathematical value `0.25028228759765625`. The first number is a worked reconstruction, rather than a new Excel reading or a statement that the candidate is correct on that input.

The current cumulative route sums masses forward with ordinary binary64 additions and caps the final sum at one; it has its own endpoint shortcuts. A stable recurrence or incomplete-beta CDF is another mathematical algorithm with a different rounding history. The source’s zero-success logarithm remains unidentified, the Stirling table and deviance retain exceptions, and the problem report still records unresolved distribution answers. These stages do not establish whole-function agreement; the separately pinned interactive calculator can also predate the later interior update.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[BINOM.INV](FUNC.BINOM.INV.md) finds a success-count quantile; [BINOM.DIST.RANGE](FUNC.BINOM.DIST.RANGE.md) covers an interval of counts; [HYPGEOM.DIST](FUNC.HYPGEOM.DIST.md) models sampling without replacement.

## Sources

- [Microsoft: BINOMDIST function](https://support.microsoft.com/en-us/excel/functions/binomdist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
- [Remaining numerical failures](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/docs/generated/problem-functions.md).
- [R31 running exponential correction](https://github.com/DnaCalc/DnaCalc/blob/481f4ab85d3bf5a95b6daf959bb75ccb2f8b40f8/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); `exp_running`, with resident and stored subtraction chains.
- [Loader’s published saddle-point method](https://www.r-project.org/doc/reports/CLoader-dbinom-2002.pdf), opened 6 November 2026; mathematical starting form, separate from the measured store identification.
