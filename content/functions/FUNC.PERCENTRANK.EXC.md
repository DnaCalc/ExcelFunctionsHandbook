---
schema: efh.function-page/v1
function_id: FUNC.PERCENTRANK.EXC
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

# PERCENTRANK.EXC

## What it does

`PERCENTRANK.EXC(array,x,[significance])` returns x’s fractional standing among numeric observations. The exclusive convention puts observed endpoints strictly between zero and one. The omitted precision argument is three.

## Try it

`=PERCENTRANK.EXC({10,20,30,40},15)` returns `0.3`. Fifteen is halfway between the first two observations, so its rank is interpolated before the returned decimal places are truncated.

## Quirks

A query outside the numeric data range gives `#N/A` in the current code. Significance below one gives `#NUM!`. Microsoft describes this argument as significant digits; the recorded helper instead truncates **decimal places**. Microsoft documents `#NUM!` for an empty array, while the current helper returns `#N/A`; that difference remains visible rather than silently promoted to an Excel rule. Duplicate-rank policy needs separate controls.

## How to implement

Sort the admitted numbers. For distinct observations give zero-based index i the fraction `(i+1)/(n+1)`. Locate x exactly or bracket it by neighbouring values, then interpolate between their fractions. Finally apply `trunc(rank*10^d)/10^d`, using the admitted precision d. For the example the endpoint fractions are `1/5` and `2/5`; their midpoint is `0.3`. The current exact-match branch uses a binary search; its duplicate selection is an implementation detail and its tie policy outside these controls remains open.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md) maps a fraction back to a value. [RANK.EQ](FUNC.RANK.EQ.md) returns an integer rank rather than a fraction.

## Sources

- [Microsoft: PERCENTRANK.EXC function](https://support.microsoft.com/en-us/excel/functions/percentrank-exc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentrank_common.rs); implementation steps and the limits identified in the text.
