---
schema: efh.function-page/v1
function_id: FUNC.COUNTIF
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

# COUNTIF

## What it does

`COUNTIF(range, criteria)` counts cells satisfying one condition. The criterion can be a number, text, a cell containing the condition, or a comparison such as `">=10"`.

## Try it

Put `4,11,7,13` in B2:B5. `=COUNTIF(B2:B5,">=10")` returns `2`. `=COUNTIF(B2:B5,">"&B4)` also returns `2`, comparing each cell with seven.

## Quirks

Text matching is case-insensitive. `*` matches a sequence and `?` one character; prefix either with `~` to match it literally. Operators belong inside quoted criteria, and an operator combined with a cell value needs `&`. Microsoft documents misleading results with strings longer than 255 characters and `#VALUE!` for references into a closed workbook.

## How to implement

Parse a leading comparison (`=`, `<>`, `<`, `<=`, `>`, `>=`) separately from its operand. Distinguish a numeric comparison from a text pattern. For text patterns support `?`, `*`, and tilde escaping, with Excel’s case-insensitive matching. Keep blank cells, empty strings and zero as distinct inputs until the criteria rule decides how each matches.

Scan the range once, evaluate the predicate for each cell and increment for a match. In the example, `4` fails, `11` passes, `7` fails and `13` passes. Do not implement a criterion by evaluating arbitrary formula text. Error-valued criteria and unusual blank comparisons need the same criteria machinery as SUMIF and COUNTIFS.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[COUNTIFS](FUNC.COUNTIFS.md) combines conditions with AND. For OR conditions, account for overlap when adding separate counts. [FILTER](FUNC.FILTER.md) returns matching rows.

## Sources

- [Microsoft: COUNTIF function](https://support.microsoft.com/en-us/excel/get-started/use-the-countif-function-in-microsoft-excel), opened 6 November 2026; syntax, documented argument rules and errors.
