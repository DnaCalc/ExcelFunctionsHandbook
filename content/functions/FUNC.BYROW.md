---
schema: efh.function-page/v1
function_id: FUNC.BYROW
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: callable_helpers
role_in_family: "Row-wise reduction: invokes the callable once per row and returns one column of results."
---

# BYROW

## What it does

`BYROW(array, lambda)` applies a one-parameter lambda to each entire row and returns one result
per row in a single column. For `r` input rows and `c` input columns, a scalar result from each
invocation produces an `r`-by-one output.

`=BYROW({1,2;3,4},LAMBDA(row,SUM(row)))` returns the column `{3;7}`. The lambda receives a row
array, not one cell at a time.

## Try it

Change the sum to `MAX(row)` or `AVERAGE(row)`. Compare the output shape with BYCOL, which
passes columns instead and returns a row.

## Quirks

Microsoft documents incorrect lambda parameters as `#VALUE!` and a result that is not a single
value as `#CALC!`. A row-shaped input to the lambda does not mean that the lambda may return an
arbitrarily shaped array.

Direct error arguments are checked before ordinary array elements. The recorded
`BYROW(#N/A,#VALUE!)` returns `#N/A`, while `BYROW({#N/A},2)` returns `#VALUE!` for the invalid
lambda. Keep those cases distinct when validating a helper call.

## How to implement

Expose one row-shaped view of the input at a time, invoke the one-parameter callable, and store
its scalar answer in the corresponding position of a one-column result. Preserve the callable
and any references the operation requires instead of reconstructing them from cell display.

Validate result shape per invocation. Do not flatten a nested result silently or reduce a
returned error into a logical. Keep direct argument error handling separate from errors found
inside a row. Share this machinery with BYCOL by changing which dimension is exposed and which
output dimension is retained.

## Version differences

Microsoft lists BYROW for Microsoft 365 and Excel 2024 on Windows and Mac. The error examples
below are observations on their named build, not a cross-version comparison.

## Alternatives

[BYCOL](FUNC.BYCOL.md) passes whole columns. [MAP](FUNC.MAP.md) passes individual positions and
preserves shape. [REDUCE](FUNC.REDUCE.md) accumulates over the array rather than producing one
answer per row.

## Sources

- [Microsoft: BYROW function](https://support.microsoft.com/en-us/excel/functions/byrow-function).
- [Recorded helper error examples](../notes/FN-HSTACK-001.json).
