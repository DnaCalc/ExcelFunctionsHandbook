---
schema: efh.function-page/v1
function_id: FUNC.NORMSINV
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

# NORMSINV

## What it does

`NORMSINV(probability)` is the compatibility quantile of the standard normal distribution: it returns z with cumulative probability p.

## Try it

`=NORMSINV(0.5)` returns `0`, the centre of the standard normal distribution. A probability below one half has a negative quantile.

## Quirks

The documented probability interval is strictly between zero and one. Endpoints and a nonpositive standard deviation give `#NUM!`; nonnumeric arguments give `#VALUE!`. A tiny p refers to the far **left** tail, not the upper-tail critical value.

## How to implement

Validate the probability and scale. The current standard inverse uses Acklam’s piecewise rational approximation, then one Newton correction based on a direct complementary-error tail and normal density. Use a lower-tail residual for negative candidates and an upper-tail residual for positive candidates to avoid cancellation at one. Return the refined z. At p=0.5 the symmetric candidate is zero. This is a published-method candidate: the generic inverse’s Excel coefficients and stopping/rounding stages are still open, even though the median control is exact.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 2 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NORMSINV.json).

## Alternatives

[NORM.S.INV](FUNC.NORM.S.INV.md) is the modern spelling; [NORMSDIST](FUNC.NORMSDIST.md) gives the forward probability.

## Sources

- [Microsoft: NORMSINV function](https://support.microsoft.com/en-us/excel/functions/normsinv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
