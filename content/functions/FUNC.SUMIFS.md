---
schema: efh.function-page/v1
function_id: FUNC.SUMIFS
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

# SUMIFS

## What it does

`SUMIFS(sum_range, criteria_range1, criteria1, ...)` adds amounts whose corresponding cells satisfy all supplied conditions. The amount range is the first argument.

## Try it

With amounts `4,11,7,13` in B2:B5 and regions `east,east,west,east` in C2:C5, `=SUMIFS(B2:B5,C2:C5,"east",B2:B5,">=10")` returns `24`: eleven plus thirteen.

## Quirks

Every criteria range must have the same shape as the amount range. Wildcards work in text criteria. The ordering differs from SUMIF, where the sum range is optional and comes last. Multiple criteria mean AND; adding two SUMIFS calls can count a row twice if the conditions overlap.

## How to implement

Parse a leading comparison (`=`, `<>`, `<`, `<=`, `>`, `>=`) separately from its operand. Distinguish a numeric comparison from a text pattern. For text patterns support `?`, `*`, and tilde escaping, with Excel’s case-insensitive matching. Keep blank cells, empty strings and zero as distinct inputs until the criteria rule decides how each matches.

Check the dimensions, then walk corresponding offsets. Evaluate every criterion before admitting that position’s amount. Fold only the admitted numeric amounts into the sum. In the example the admitted cells are B3 and B5, yielding `0+11+13=24`. Keep criteria matching separate from numeric reduction, so errors and blank rules can be tested independently.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[SUMIF](FUNC.SUMIF.md) is shorter for one condition; [COUNTIFS](FUNC.COUNTIFS.md) counts rows; [SUMPRODUCT](FUNC.SUMPRODUCT.md) expresses weighted or more general array conditions.

## Sources

- [Microsoft: SUMIFS function](https://support.microsoft.com/en-us/office/sumifs-function-c9e748f5-7ea7-455d-9406-611cebce642b), opened 6 November 2026; syntax, documented argument rules and errors.
