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

`=PERCENTRANK.EXC({10,20,30,40},15)` returns `0.3`. Fifteen is halfway between the first two observations. Their exclusive fractions are `1/5` and `2/5`, so the midpoint is three tenths.

The endpoints ten and forty have fractions `0.2` and `0.8`. PERCENTRANK.INC instead places them at zero and one. Percent-rank maps a value to standing; PERCENTILE.EXC maps a requested standing to a value.

## Quirks

A query outside the numeric data range gives `#N/A` in the current code. Significance below one gives `#NUM!`. Microsoft describes this argument as significant digits; the current helper instead truncates **decimal places**. Microsoft documents `#NUM!` for an empty array, while the current helper returns `#N/A`; that difference remains visible rather than silently promoted to an Excel rule. Duplicate-rank policy needs separate controls.

## How to implement

1. Collect and sort numeric observations. A query outside their numeric range follows the current helper’s `#N/A` path.
2. For distinct observations, give zero-based index `i` fraction `(i+1)/(n+1)`.
3. Find an exact match, or bracket `x` with neighbouring observations. The current interpolation is `lower_rank+(x-lower_x)*(upper_rank-lower_rank)/(upper_x-lower_x)`, with multiplication before division in ordinary binary64.
4. Truncate the admitted significance to integer `d`; values below one give `#NUM!`. The current helper caps its scale exponent at 308, computes `scale=10^min(d,308)`, then returns `trunc(rank*scale)/scale`.

For fifteen between ten and twenty, the rank is `0.2+0.5*(0.4-0.2)=0.3`. The precision step discards information, so applying a percentile afterward need not recover the original value exactly.

The current exact-match branch uses binary search. Which member of a repeated block it selects is a separate implementation and measurement question; the distinct-data example does not establish a duplicate policy. Microsoft’s significant-digit wording and the current decimal-place rule also need to remain distinct.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

The retained catalogue capture on Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), AMD Ryzen 7 4800H, en-ZA, Workbook Compatibility Version 1 records =PERCENTRANK.EXC({12.7,51.9,92.5},87) = 0.716 (0x3fe6e978d4fdf3b6). Inputs were entered as the shown formula literal. Raw, saved and reopened views belong to one capture; this is not a comparison between builds.

## Alternatives

[PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md) maps a fraction back to a value. [RANK.EQ](FUNC.RANK.EQ.md) returns an integer rank rather than a fraction.

## Sources

- [Microsoft: PERCENTRANK.EXC function](https://support.microsoft.com/en-us/excel/functions/percentrank-exc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentrank_common.rs); implementation steps and the limits identified in the text.
- [Related precision controls](../notes/FN-PERCENTRANK-001.json): retained PERCENTRANK and PERCENTRANK.INC discriminators; the note does not measure the exclusive precision rule.
