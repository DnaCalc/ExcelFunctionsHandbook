---
schema: efh.function-page/v1
function_id: FUNC.EXPON.DIST
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

# EXPON.DIST

## What it does

`EXPON.DIST(x, lambda, cumulative)` models a nonnegative waiting time at a positive rate `lambda`. TRUE returns the probability of a wait no longer than `x`; FALSE returns a density at `x`. Rate has inverse-time units, so if `x` is in hours, `lambda` is per hour. The mathematical mean wait is `1/lambda`.

## Try it

`=EXPON.DIST(2,0.5,TRUE)` is about `0.632120558828558`: the rate-times-time product is one. About 63.2% of the modeled waits finish within two time units. Changing TRUE to FALSE gives density about `0.183939720585721`.

`=EXPON.DIST(0,2,FALSE)` returns `2`. A density can exceed one; it describes probability per unit of time. It is not the probability of a wait lasting exactly zero.

## Quirks

Negative `x` or a nonpositive rate gives `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. Zero time is admitted: the CDF is zero and the density is the rate. Do not pass a mean waiting time as `lambda` without taking its reciprocal.

A naive `1-exp(-lambda*x)` can erase a small positive CDF when the exponential rounds to one. The current route has an explicit correction for this case. It also checks the stored rate-times-time product for overflow and flushes a subnormal product to zero, so a representable small CDF is not enough by itself to predict the published result.

## How to implement

The current kernel uses `R` for the [stored arithmetic operation](../model/09-staged-arithmetic.md), with the measured public LN and EXP leaves.

1. Validate `x>=0` and `lambda>0`. Store `t=R(lambda*x)`. A nonfinite `t` gives `#NUM!`; replace a subnormal `t` with zero.
2. For density, obtain `u=EXP(-t)` and return `R(lambda*u)` under the finite-result policy.
3. For the CDF, obtain the same `u`. If it is exactly one, return `t`.
4. Otherwise store `d=R(u-1)`. If `t<1`, store `c=R(R(d*(-t))/LN(u))`; otherwise use `c=d`. Return `-c`. Keep the product and division stores separate.
5. The current finite-result helper turns nonfinite answers into `#NUM!` and flushes final subnormal answers to zero.

For the main example, `t=1`, `u` is about `0.36787944117144233`, and the CDF is about `0.632120558828558`. The density multiplies `u` by `0.5`. At `t=1e-20`, the public exponential rounds to one; returning `t` preserves the small CDF that direct subtraction would lose.

This is the stored Kahan correction also used by the [PMT discount recipe](FUNC.PMT.md#how-to-implement). The leaves and operation order are part of the candidate; substituting a language’s `exp` or `expm1` can change final bits. The problem report retains a numerical residual, so this is not a whole-domain equality claim.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 39,412 |
| 20430 | Windows, 64-bit, CPU not recorded | 9,934 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.EXPON.DIST.json).

## Alternatives

[GAMMA.DIST](FUNC.GAMMA.DIST.md) with shape one is related, but its scale transformation has its own stores. [WEIBULL.DIST](FUNC.WEIBULL.DIST.md) generalises the waiting-time shape.

## Sources

- [Microsoft: EXPON.DIST function](https://support.microsoft.com/en-us/excel/functions/expon-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/discrete_dist.rs); implementation steps and the limits identified in the text.
- [Stored cancellation correction](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs), `excel_expm1_spilled`; [remaining problem rows](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/problem-functions.md).
