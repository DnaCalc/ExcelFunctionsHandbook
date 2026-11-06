---
schema: efh.function-page/v1
function_id: FUNC.COVARIANCE.P
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

# COVARIANCE.P

## What it does

`COVARIANCE.P(array1, array2)` measures population covariance: whether paired observations tend to move in the same direction. It averages products of deviations using divisor `n`. Covariance carries the product of the two input units.

## Try it

`=COVARIANCE.P({1,2,3},{2,4,6})` returns approximately `1.33333333333333`. The means are two and four; the paired deviation products are two, zero and two.

## Quirks

Text, logical values and empty cells in the data do not contribute numeric pairs; zero does. Unequal data-point counts give `#N/A`. Empty data gives `#DIV/0!`. Do not filter each side independently and then pair what remains: preserve positional alignment.

## How to implement

Validate shapes and collect eligible paired numbers. Compute each mean, then accumulate `(x-mean_x)*(y-mean_y)` over pairs and divide by `n`. For the example the total is four, divided by three. The current product helper uses ordinary binary64 means and centered products; no stored-stage identification for this whole route is claimed here. Keep pair admission and error handling separate from the arithmetic.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CORREL](FUNC.CORREL.md) removes units and scales to a correlation coefficient. [COVARIANCE.S](FUNC.COVARIANCE.S.md) changes the denominator.

## Sources

- [Microsoft: COVARIANCE.P function](https://support.microsoft.com/en-us/excel/functions/covariance-p-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/paired_stats_common.rs); implementation steps and the limits identified in the text.
