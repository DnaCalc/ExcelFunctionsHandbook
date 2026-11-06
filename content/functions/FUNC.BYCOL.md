---
schema: efh.function-page/v1
function_id: FUNC.BYCOL
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
role_in_family: "Column-wise reduction: invokes the callable once per column and returns one row of results."
---

# BYCOL

## What it does

`BYCOL(array, lambda)` applies a one-parameter lambda to each entire column and returns one
result per column in a single row. An input with `r` rows and `c` columns therefore produces
one row with `c` results when every invocation returns one value.

`=BYCOL({1,2;3,4},LAMBDA(col,SUM(col)))` returns the row `{4,6}`. Each invocation sees a column,
not one cell at a time.

## Try it

Change `SUM(col)` to `MAX(col)` or `AVERAGE(col)` in the example. Compare the shape with
`=BYROW({1,2;3,4},LAMBDA(row,SUM(row)))`.

## Quirks

The lambda must accept one parameter and produce one result per column. Microsoft's documented
errors distinguish incorrect parameters (`#VALUE!`) from failure to produce a single value
(`#CALC!`). BYCOL does not create a row of nested arrays.

A direct error can stop the helper before its lambda is checked. On the recorded build,
`BYCOL(#N/A,#VALUE!)` is `#N/A`, while `BYCOL({#N/A},2)` is `#VALUE!`. The latter supplies an
array element but no valid lambda. This distinction matters when validating arguments.

## How to implement

Retain the input's two-dimensional shape. For each column, expose a column-shaped view to the
callable, bind its one parameter, evaluate the body and store its scalar answer in that column
of a one-row output. Avoid copying the whole input for every invocation.

Preserve references where the callable's operation needs them. Detect direct argument errors
at the appropriate stage, then validate the callable and its arity. Reject nested-array results
through the helper's result rule rather than flattening them silently. Keep errors returned as
individual scalar results in their output positions.

## Version differences

Microsoft lists BYCOL for Microsoft 365 and Excel 2024 on Windows and Mac. Build observations
below cover direct errors. Other differences between named builds have not been characterized here.

## Alternatives

[BYROW](FUNC.BYROW.md) applies a lambda to rows and returns a column. [MAP](FUNC.MAP.md) visits
individual positions and preserves the input shape. A direct SUM of a single column is shorter
when only one column needs aggregating.

## Sources

- [Microsoft: BYCOL function](https://support.microsoft.com/en-us/excel/functions/bycol-function).
- [Recorded helper error examples](../notes/FN-HSTACK-001.json).
