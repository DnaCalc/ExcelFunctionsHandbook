---
schema: efh.function-page/v1
function_id: FUNC.GAMMADIST
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

# GAMMADIST

## What it does

`GAMMADIST(x, alpha, beta, cumulative)` returns gamma cumulative probability (TRUE) or density (FALSE). Alpha is a positive shape and beta a positive **scale**. This is the compatibility name.

## Try it

`=GAMMADIST(2,1,2,TRUE)` is about `0.632120558828558`. With shape one it is an exponential waiting-time CDF with scale two.

## Quirks

Nonpositive shapes/scales give `#NUM!`; nonnumeric numeric arguments give `#VALUE!`. Negative x gives `#NUM!`. Scale beta is the reciprocal of an exponential rate, not the rate itself.

## How to implement

For a general CDF evaluate the regularized lower gamma ratio `P(alpha,x/beta)`. The current shape-half special route is ERF.PRECISE at `sqrt(x/beta)`; shape one calls the exponential CDF at `x/beta` and rate one. Forming a reciprocal beta first has distinguishable roundings. General density uses a log-normalized body, with a separately stored ratio/product consumer in the current code. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 6 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.GAMMADIST.json).

## Alternatives

[GAMMA.DIST](FUNC.GAMMA.DIST.md) is the current spelling; [EXPON.DIST](FUNC.EXPON.DIST.md) handles the shape-one waiting time; [CHISQ.DIST](FUNC.CHISQ.DIST.md) is a shape/scale specialization.

## Sources

- [Microsoft: GAMMADIST function](https://support.microsoft.com/en-us/excel/functions/gammadist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
