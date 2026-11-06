---
schema: efh.function-page/v1
function_id: FUNC.F.INV
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

# F.INV

## What it does

`F.INV(probability, deg_freedom1, deg_freedom2)` finds an F-distribution value from a left cumulative probability. The first degree count belongs to the numerator and the second to the denominator.

## Try it

`=F.INV(0.75,2,2)` is about `3`. With both degree counts two, the mathematical left CDF is `x/(1+x)` and the right tail is `1/(1+x)`.

## Quirks

Degree counts are truncated. Nonnumeric arguments give `#VALUE!`; invalid counts or probabilities give `#NUM!`. The current shared inverse admits each truncated degree from one through `10^10` and requires an open probability interval `0<p<1`.

The opened Microsoft F.INV page states the positive degree-count requirement without this shared helper’s upper cap. Keep the documentation and the current code’s boundary tests distinct: a source guard alone is not an additional Excel measurement.

A left-CDF probability and a right-tail probability select different quantiles. Subtracting an extremely small right-tail probability from one can round to one and lose the target entirely.

## How to implement

The current left-tail candidate searches its paired beta forward surface. Let `D` mean ordinary binary64 storage and `I_z(a,b)` the regularized lower beta ratio.

1. Truncate and validate numerator degrees d1 and denominator degrees d2; require `0<p<1`.
2. At each nonnegative trial x, form `num=D(d1*x)` and `den=D(num+d2)`. Compute `z=D(num/den)` and `w=D(d2/den)` separately, then evaluate `bratio(d1/2,d2/2,z,w).first`. Supplying both independently rounded beta arguments preserves more tail information than setting w to `1-z`.
3. Start with `lo=0,hi=1`; double hi until the forward value reaches p, with at most 200 expansion checks.
4. Bisect the nonnegative binary64 encodings as in the [shared inverse search](FUNC.CHISQ.INV.RT.md#how-to-implement), using this increasing CDF and target p. Return the upper endpoint when its encoding is adjacent to the lower one.

For d1=d2=2 the mathematical CDF is `x/(1+x)`. At p=0.75, solving `x/(1+x)=0.75` yields three.

The beta routine, its complement pair and the adjacent-value return are separate numerical stages. A beta-quantile transform `x=d2*z/(d1*(1-z))` is another mathematical route with different stores. The source’s problem report retains unresolved distribution mismatches; neither route identifies every Excel approximation or certifies the full function.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[F.DIST](FUNC.F.DIST.md) supplies the forward left CDF. [F.INV.RT](FUNC.F.INV.RT.md) takes an upper-tail probability; [F.TEST](FUNC.F.TEST.md) computes a sample test probability.

## Sources

- [Microsoft: F.INV function](https://support.microsoft.com/en-us/excel/functions/f-inv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
- [Paired beta and ordered-value search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); `bratio` and `bisect_inverse`.
