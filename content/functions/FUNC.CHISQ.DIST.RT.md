---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.DIST.RT
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

# CHISQ.DIST.RT

## What it does

`CHISQ.DIST.RT(x, deg_freedom)` returns the chi-square probability to the right of x.

## Try it

`=CHISQ.DIST.RT(2,2)` is about `0.367879441171442`. At two degrees of freedom the right tail is `exp(-x/2)`. The tail at x=0 is one.

## Quirks

Degrees of freedom are truncated. Negative x and an admitted degree count below one or above `10^10` give `#NUM!` in the current shared family. Nonnumeric arguments give `#VALUE!`. A right tail is not a density.

## How to implement

For df=1 the current route calls ERFC.PRECISE at `sqrt(x/2)`, preserving divide-then-square-root order. For df=2 it calls the measured public EXP at `-(x/2)`. Otherwise evaluate the regularized upper gamma ratio `Q(df/2,x/2)` directly. Avoid obtaining a tiny tail by subtracting a near-one CDF. The worked df=2 value follows from `x/2=1`. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 6 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.CHISQ.DIST.RT.json).

## Alternatives

[CHISQ.INV.RT](FUNC.CHISQ.INV.RT.md) inverts the corresponding probability. [CHISQ.TEST](FUNC.CHISQ.TEST.md) computes a test probability from observed and expected counts.

## Sources

- [Microsoft: CHISQ.DIST.RT function](https://support.microsoft.com/en-us/excel/functions/chisq-dist-rt-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
