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

`WEIBULL(x,alpha,beta,cumulative)` evaluates a Weibull distribution at nonnegative `x`. Alpha is the positive shape; beta is the positive scale in the same units as `x`. TRUE returns a cumulative probability and FALSE a density. This is the compatibility spelling of WEIBULL.DIST.

Writing `t=(x/beta)^alpha`, the mathematical CDF is `1-exp(-t)`. In particular, at `x=beta` it is `1-exp(-1)` for every positive shape. Shape one gives the exponential family with rate `1/beta`.

## Try it

`=WEIBULL(4,2,2,TRUE)` is about `0.9816843611`: mathematically, `(4/2)^2=4` and the CDF is `1-exp(-4)`. Changing TRUE to FALSE gives a density of about `0.03663127777746836`.

The scale is not the exponential rate. In the shape-one case, a scale of two corresponds to rate one half, not rate two.

## Quirks

Negative `x` or a nonpositive shape or scale gives `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. Densities and cumulative probabilities are different quantities, even when they happen to have similar decimal values.

At zero, the mathematical CDF is zero. The density’s right-hand limit is zero for `alpha>1`, `1/beta` for `alpha=1`, and unbounded for `0<alpha<1`. The current helper returns zero at `x=0` for both cumulative and density calls, for every admitted shape. For example, its `WEIBULL(0,1,2,FALSE)` result is zero rather than the mathematical one half. This is a code-level boundary behaviour, not a newly measured Excel answer; keep it separate from the positive-input reconstruction.

## How to implement

For positive inputs, define a raw power helper `P(a,b)=EXP(R(b*LN(a)))`, using the public stored LN and EXP leaves and the [stored operation R](../model/09-staged-arithmetic.md). The current route uses this chain even for integer exponents and square roots; a host `pow` shortcut changes the candidate.

1. Validate finite inputs and the documented sign domain; handle the code’s zero-input branch separately.
2. Store `r=R(x/beta)`, then obtain `t=P(r,alpha)`.
3. For the CDF, compute `-expm1_internal(-t)`. Here the Kahan correction uses ordinary binary64 arithmetic: with `u=EXP(-t)`, return `t` if `u==1`; otherwise return `-(u-1)` for `t>=1`, or `-((u-1)*(-t)/LN(u))` for `t<1`.
4. For density, obtain `B=P(beta,alpha)`, `X=P(x,alpha-1)` and `E=EXP(-t)`. Evaluate division first and multiply left to right: `R(R(R(alpha/B)*X)*E)`.
5. The helper rejects a nonfinite result with `#NUM!`. Keep worksheet publication separate from the polynomial/power body.

For `x=4,alpha=2,beta=2`, the mathematical reduction has `r=2`, `t=4`, `alpha/beta^alpha=0.5`, and `x^(alpha-1)=4`; density is therefore `2*exp(-4)`. The raw power chain need not publish exact integer four at each intermediate stage, so the mathematical trace is not permission to remove those calls.

The cited source retains positive-input power-staging discriminants, including 113 controls separating the stored exponent product. Exact global branch boundaries and a whole-domain legacy-spelling result remain unestablished here. No Handbook suite is published for this function.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[WEIBULL.DIST](FUNC.WEIBULL.DIST.md) is the modern spelling; [EXPON.DIST](FUNC.EXPON.DIST.md) is the shape-one special family; [GAMMA.DIST](FUNC.GAMMA.DIST.md) supplies another positive-valued distribution.

## Sources

- [Microsoft: WEIBULL function](https://support.microsoft.com/en-us/excel/functions/weibull-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_dist_family.rs); implementation steps and the limits identified in the text.
- [Raw power and cancellation helpers](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs), `excel_pow_chain` and `excel_expm1_internal`.
