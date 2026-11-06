---
schema: efh.function-page/v1
function_id: FUNC.FINV
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

# FINV

## What it does

`FINV(probability, deg_freedom1, deg_freedom2)` finds an F-distribution value from a right-tail probability. The first degree count belongs to the numerator and the second to the denominator. This is a compatibility spelling.

## Try it

`=FINV(0.25,2,2)` is about `3`. With both degree counts two, the mathematical left CDF is `x/(1+x)` and the right tail is `1/(1+x)`.

## Quirks

Degree counts are truncated. Nonnumeric arguments give `#VALUE!`; invalid counts or probabilities give `#NUM!`. The current shared inverse admits each truncated degree from one through `10^10` and requires an open probability interval `0<p<1`.

The Microsoft FINV page documents degree counts strictly below `10^10` and `#N/A` after 100 unsuccessful iterations. Keep the documentation and the current code’s boundary tests distinct: a source guard alone is not an additional Excel measurement.

A left-CDF probability and a right-tail probability select different quantiles. Subtracting an extremely small right-tail probability from one can round to one and lose the target entirely.

## How to implement

The current right-tail candidate uses an accurate pair of beta arguments and solves the decreasing tail directly. Let `D` mean ordinary binary64 storage and `I_z(a,b)` the regularized lower beta ratio.

1. Truncate and validate numerator degrees d1 and denominator degrees d2; require `0<p<1`.
2. At each nonnegative trial x, form `num=D(d1*x)` and `den=D(d2+num)`. Compute `z=D(d2/den)` and `w=D(num/den)` separately, then the paired beta helper `bratio(d2/2,d1/2,z,w).first`. Do not construct w as `1-z`.
3. Negate that tail to obtain an increasing function H and set target to `-p`. Start with `lo=0,hi=1`; double hi until H(hi) reaches the target, with at most 200 expansion checks.
4. Use the [ordered-binary64 search](FUNC.CHISQ.INV.RT.md#how-to-implement): bisect the endpoint encodings until adjacent, and return the upper endpoint. The search is over the stored forward surface, not an exact real beta function.

For d1=d2=2 the mathematical tail is `1/(1+x)`. At p=0.25, solving `1/(1+x)=0.25` yields three.

The beta routine, its complement pair and the adjacent-value return are separate numerical stages. A beta-quantile transform `x=d2*z/(d1*(1-z))` is another mathematical route with different stores. The source’s problem report retains unresolved distribution mismatches; neither route identifies every Excel approximation or certifies the full function.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 80 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 32 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.FINV.json).

## Alternatives

[F.DIST.RT](FUNC.F.DIST.RT.md) gives a forward probability; [F.INV.RT](FUNC.F.INV.RT.md) gives a critical value; [F.TEST](FUNC.F.TEST.md) starts from samples.

## Sources

- [Microsoft: FINV function](https://support.microsoft.com/en-us/excel/functions/finv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
- [Paired beta and ordered-value search](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/special_math_common.rs); `bratio` and `bisect_inverse`.
