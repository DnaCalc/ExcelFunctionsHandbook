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

Degree counts are truncated and must be positive within the function’s admitted limits. Nonnumeric arguments give `#VALUE!`; invalid counts, probabilities or negative forward x give `#NUM!`. Left and right probabilities are different arguments to an inverse. The density option in F.DIST is not an upper-tail probability.

## How to implement

For the CDF, set `z=df1*x/(df1*x+df2)` and evaluate `I_z(df1/2,df2/2)`. The complementary beta tail supplies the right probability without near-one subtraction. Scale the transformation to avoid overflowing `df1*x`. A density uses the beta normalization and requires explicit x=0 limits. At df1=df2=2 and x=3, z=3/4, so the uniform beta CDF is 3/4. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 6 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.F.DIST.RT.json).

## Alternatives

[F.DIST.RT](https://handbook.dnacalc.com/functions/F.DIST.RT.html) gives a forward probability; [F.INV.RT](https://handbook.dnacalc.com/functions/F.INV.RT.html) gives a critical value; [F.TEST](https://handbook.dnacalc.com/functions/F.TEST.html) starts from samples.

## Sources

- [Microsoft: F.DIST.RT function](https://support.microsoft.com/en-us/excel/functions/f-dist-rt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
