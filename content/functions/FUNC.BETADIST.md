---
schema: efh.function-page/v1
function_id: FUNC.BETADIST
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

# BETADIST

## What it does

`BETADIST(x, alpha, beta, [A], [B])` returns the cumulative beta probability up to x. The two positive shape parameters control the distribution, and optional bounds A and B default to zero and one. This is a compatibility name.

## Try it

`=BETADIST(3,1,1,2,6)` is about `0.25`. The shape-one beta is uniform on two to six, and three is one quarter of the way across.

## Quirks

Nonpositive shapes and invalid bounds give `#NUM!`; nonnumeric arguments give `#VALUE!`. An x outside the bounds gives `#NUM!`. This old spelling has no density/CDF switch: it is cumulative.

## How to implement

Validate the interval and shapes, form `z=(x-A)/(B-A)`, and evaluate the regularized incomplete beta `I_z(alpha,beta)`. Use symmetry and a tail-aware beta routine rather than directly integrating an unstable density. The current kernel routes even integer shapes through the shared beta-ratio body. For `(3,1,1,2,6)`, `z=1/4` and the uniform CDF is `1/4`. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 28 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 294 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.BETADIST.json).

## Alternatives

[BETA.DIST](FUNC.BETA.DIST.md) is the current spelling. [BETAINV](https://handbook.dnacalc.com/functions/BETAINV.html) reverses the value/probability direction.

## Sources

- [Microsoft: BETADIST function](https://support.microsoft.com/en-us/excel/functions/betadist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/beta_gamma_stats_family.rs); implementation steps and the limits identified in the text.
