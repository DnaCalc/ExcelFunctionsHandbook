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

Degree counts are truncated and must be positive within the function’s admitted limits. Nonnumeric arguments give `#VALUE!`; invalid counts, probabilities or negative forward x give `#NUM!`. Left and right probabilities are different arguments to an inverse. The shared current inverse uses an open probability interval; endpoint behaviour must be handled explicitly.

## How to implement

For the inverse, invert a regularized beta ratio and transform back to x, or bracket x and solve the appropriate monotone forward probability. Preserve a direct right-tail route rather than subtracting a tiny probability from one. For equal df=2, solving `1/(1+x)=0.25` or `x/(1+x)=0.75` yields three. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 80 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 32 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.FINV.json).

## Alternatives

[F.DIST.RT](FUNC.F.DIST.RT.md) gives a forward probability; [F.INV.RT](FUNC.F.INV.RT.md) gives a critical value; [F.TEST](https://handbook.dnacalc.com/functions/F.TEST.html) starts from samples.

## Sources

- [Microsoft: FINV function](https://support.microsoft.com/en-us/excel/functions/finv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
