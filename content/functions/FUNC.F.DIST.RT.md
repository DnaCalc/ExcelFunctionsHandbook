---
schema: efh.function-page/v1
function_id: FUNC.F.DIST.RT
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

# F.DIST.RT

## What it does

`F.DIST.RT(x, deg_freedom1, deg_freedom2)` returns an F-distribution right-tail probability. The first degree count belongs to the numerator and the second to the denominator.

## Try it

`=F.DIST.RT(3,2,2)` is about `0.25`. With both degree counts two, the mathematical left CDF is `x/(1+x)` and the right tail is `1/(1+x)`.

## Quirks

Degree counts are truncated. Microsoft documents `#VALUE!` for nonnumeric arguments and `#NUM!` for negative x or degree counts below one. The current shared helper also rejects a nonfinite x and a truncated degree count above `10^10`; that code cap is separate from the opened page’s lower-bound wording.

The left CDF rises from zero and the right tail falls from one. The density option in F.DIST is a third quantity, rather than either of those probabilities. At x=0 the current density gives `#NUM!` for numerator degree one, one for degree two and zero for higher degrees. Those density cases do not change the endpoint probabilities.

The current beta transformation forms the product of x and numerator degrees directly. Extreme products or beta/gamma approximations need their own numerical error and publication checks; a finite mathematical probability alone does not ensure this candidate evaluates it successfully.

## How to implement

The current right-tail route evaluates a lower beta ratio after swapping the degree roles. Use ordinary binary64 stores D.

1. Validate finite nonnegative x and the truncated positive degree counts d1 and d2.
2. Calculate `num=D(d1*x)` and `den=D(d2+num)`.
3. Independently store `z=D(d2/den)` and `w=D(num/den)`. Pass both to `bratio(d2/2,d1/2,z,w)` and return its first, lower-ratio result. The explicit complement w is part of the current argument construction.
4. Apply worksheet number publication, preserving the host’s error/refusal distinction.

With d1=d2=2 and x=3, num is six and den eight. The transformed beta shapes are one and one, a uniform distribution, so z=2/8 gives the right probability one quarter. The companion left route uses 6/8 instead.

Do not obtain this tail by subtracting a rounded left CDF from one. A separately scaled ratio can avoid overflowing `d1*x` in a mathematical implementation, but changes the current computation tree. The paired-argument wrapper is more specific than a generic beta identity; the inner beta approximation and remaining recorded mismatches still prevent a whole-domain identification.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 6 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.F.DIST.RT.json).

## Alternatives

[F.INV.RT](FUNC.F.INV.RT.md) maps an upper-tail probability to a critical value. [F.DIST](FUNC.F.DIST.md) provides the left CDF or density; [F.TEST](FUNC.F.TEST.md) starts with two samples.

## Sources

- [Microsoft: F.DIST.RT function](https://support.microsoft.com/en-us/excel/functions/f-dist-rt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
