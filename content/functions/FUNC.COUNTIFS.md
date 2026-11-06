---
schema: efh.function-page/v1
function_id: FUNC.COUNTIFS
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

# COUNTIFS

## What it does

`COUNTIFS(criteria_range1, criteria1, ...)` counts positions where every range meets its paired condition. The ranges align by position; they need not be adjacent.

## Try it

Put `4,11,7,13` in A1:A4 and `east,east,west,east` in B1:B4. `=COUNTIFS(A1:A4,">=10",B1:B4,"east")` returns `2`.

## Quirks

All criteria ranges must have the same row and column dimensions. Conditions use AND, including repeated conditions on the same range. A criterion referring to an empty cell is treated as zero. Wildcards use `?`, `*`, and `~` escaping.

## How to implement

Parse a leading comparison (`=`, `<>`, `<`, `<=`, `>`, `>=`) separately from its operand. Distinguish a numeric comparison from a text pattern. For text patterns support `?`, `*`, and tilde escaping, with Excel’s case-insensitive matching. Keep blank cells, empty strings and zero as distinct inputs until the criteria rule decides how each matches.

Validate range shapes and build one predicate per pair. At each offset evaluate all predicates against the cells at that offset; increment only if all pass. A1/B1 fails the numeric test; A2/B2 passes both; A3/B3 fails; A4/B4 passes. Return two. Preserve the distinction between the criterion reference being empty and a blank cell being tested.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[COUNTIF](FUNC.COUNTIF.md) handles one condition; [SUMIFS](FUNC.SUMIFS.md) sums matching amounts; [FILTER](FUNC.FILTER.md) exposes the matching records.

## Sources

- [Microsoft: COUNTIFS function](https://support.microsoft.com/en-us/excel/functions/countifs-function), opened 6 November 2026; syntax, documented argument rules and errors.
