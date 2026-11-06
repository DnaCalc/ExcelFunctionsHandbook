---
schema: efh.function-page/v1
function_id: FUNC.F.DIST
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

# F.DIST

## What it does

`F.DIST(x, deg_freedom1, deg_freedom2, cumulative)` returns an F-distribution left CDF (TRUE) or density (FALSE). The first degree count belongs to the numerator and the second to the denominator.

## Try it

`=F.DIST(3,2,2,TRUE)` is about `0.75`. With both degree counts two, the mathematical left CDF is `x/(1+x)` and the right tail is `1/(1+x)`. `=F.DIST(3,2,2,FALSE)` is about `0.0625`, the density rather than a tail.

## Quirks

Degree counts are truncated. Microsoft documents `#VALUE!` for nonnumeric arguments and `#NUM!` for negative x or degree counts below one. The current shared helper also rejects a nonfinite x and a truncated degree count above `10^10`; that code cap is separate from the opened page’s lower-bound wording.

The left CDF rises from zero and the right tail falls from one. The density option in F.DIST is a third quantity, rather than either of those probabilities. At x=0 the current density gives `#NUM!` for numerator degree one, one for degree two and zero for higher degrees. Those density cases do not change the endpoint probabilities.

The current beta transformation forms the product of x and numerator degrees directly. Extreme products or beta/gamma approximations need their own numerical error and publication checks; a finite mathematical probability alone does not ensure this candidate evaluates it successfully.

## How to implement

For the current cumulative route, use ordinary binary64 stores D:

1. Validate finite nonnegative x and the truncated positive degree counts d1 and d2.
2. Calculate `num=D(d1*x)` and `den=D(num+d2)`.
3. Independently store `z=D(num/den)` and `w=D(d2/den)`. Return the first result of `bratio(d1/2,d2/2,z,w)`. Both beta arguments are supplied; w is not computed by subtracting z from one.
4. Apply worksheet number publication. The upper-tail function swaps the shapes and uses the complementary argument as its leading input.

For d1=d2=2 and x=3, num is six and den eight. Both beta shapes are one, so the uniform beta CDF gives z=6/8=0.75.

For the density at positive x, with `a=d1/2,b=d2/2`, the mathematical expression is `(d1/d2)^a*x^(a-1)/(B(a,b)*(1+d1*x/d2)^(a+b))`. The current code uses runtime powers and gamma products for that normalization, plus explicit x=0 cases. At d1=d2=2 the expression reduces to `1/(1+x)^2`; x=3 gives density `0.0625`, distinct from the right-tail probability `0.25`.

A log-density or scaled-ratio implementation can improve numerical range, but changes this candidate’s stores and leaves. The paired beta wrapper does not solve every inner approximation; the problem report retains distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit, CPU not recorded | 369 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.F.DIST.json).

## Alternatives

[F.INV](FUNC.F.INV.md) maps a left probability to a value. [F.DIST.RT](FUNC.F.DIST.RT.md) computes the upper tail directly, and [F.TEST](FUNC.F.TEST.md) starts with two samples.

## Sources

- [Microsoft: F.DIST function](https://support.microsoft.com/en-us/excel/functions/f-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
