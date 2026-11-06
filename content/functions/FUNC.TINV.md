---
schema: efh.function-page/v1
function_id: FUNC.TINV
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

# TINV

## What it does

`TINV(probability,deg_freedom)` returns a nonnegative Student-t critical magnitude for a **two-tailed** probability. It is the compatibility name for the two-tail inverse.

## Try it

`=TINV(0.5,1)` returns `1` under the current explicit Cauchy control. Half the one-degree distribution lies beyond plus or minus one.

## Quirks

Degrees are truncated. Invalid degrees or probabilities give `#NUM!`; nonnumeric inputs give `#VALUE!`. The public page admits probability one in its range wording, while the current shared inverse requires `0<p<1`. For a one-tail upper critical value use twice that tail probability in the two-tail inverse, within its admitted range.

## How to implement

For degree one, the retained route is `1/TAN(PI()*p/2)`, with a separate exact `p=0.5` return of one. For higher degrees the code currently brackets and inverts the decreasing two-tail beta-ratio probability. Public-composition readings support a `sqrt(v*(1/BETA.INV(p,v/2,0.5)-1))` construction on their scope, but the current beta inverse still differs; the code therefore retains its generic inverse. Do not describe that unresolved composition as a shipped exact route.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 14 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 32 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.TINV.json).

## Alternatives

[T.INV.2T](FUNC.T.INV.2T.md) is the explicit two-tail name; [T.INV](FUNC.T.INV.md) is a signed left-CDF quantile; [TDIST](FUNC.TDIST.md) returns a tail probability.

## Sources

- [Microsoft: TINV function](https://support.microsoft.com/en-us/excel/functions/tinv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
