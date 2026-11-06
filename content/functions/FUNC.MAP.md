---
schema: efh.function-page/v1
function_id: FUNC.MAP
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
role_in_family: "Elementwise application: invokes the callable once per cell position and returns an array of the results."
---

# MAP

## What it does

`MAP(array1, [array2, …], lambda)` applies a calculation to corresponding positions of one or
more arrays and returns the resulting array. The lambda goes last and takes one parameter per
input array.

`=MAP({1;2;3},LAMBDA(x,x*x))` returns a column containing 1, 4 and 9.
With two equally shaped arrays, `=MAP({1;2},{10;20},LAMBDA(x,y,x+y))` returns 11 and 22.
Positions are paired; MAP does not form every possible pair of inputs.

## Try it

Square the example column, then change `x*x` to `IF(x>1,x*x,0)`. Use arrays with the same shape
when adapting the two-input example. Mismatched shapes need their own compatibility checks.

## Quirks

Microsoft documents `#VALUE!` when the final argument is not a valid lambda or its parameter
count is wrong. A result must fit one output position; a lambda that returns an array introduces
a separate nested-array problem rather than automatically expanding the result.

A direct error argument and an error inside an array are observably different. On the recorded
Excel build, `MAP(NA(),LAMBDA(x,IFERROR(x,3)))` returns `#N/A` before invoking the lambda, while
`MAP({#N/A},LAMBDA(x,3))` returns 3. The array element can reach the lambda even though the same
error supplied directly stops the helper.

The result still needs space in the worksheet. A blocked spill can produce `#SPILL!`; it does
not change the array calculation itself. Try it shows the array without placing it in a workbook.

## How to implement

Separate the last argument from the input arrays. Validate the callable and its parameter count
without treating it as a displayed error. For a shared input shape, allocate that output shape
and visit each coordinate. Read one value from each input at that coordinate, bind the lambda's
parameters, evaluate its body and store the scalar result at the same coordinate.

Keep direct argument errors distinct from errors held in an array or referenced cell. Passing an
array element through the lambda is necessary for formulas that deliberately replace errors.
Likewise, preserve any callable carried inside an array value.

Test mismatched input dimensions, reference inputs, a lambda returning an array, and the order
of calls to volatile expressions separately. A general array-broadcast rule is not sufficient
evidence for MAP's shape rule. The recorded examples below settle the direct-error distinction;
they do not settle every shape case.

## Version differences

Microsoft lists MAP for Microsoft 365 and Excel 2024 on Windows and Mac. The build-specific
observations below concern error handling. They do not establish that older or later builds
share every helper-function rule.

## Alternatives

Use an ordinary array expression for a simple elementwise calculation such as squaring. Use
[BYROW](FUNC.BYROW.md) or [BYCOL](FUNC.BYCOL.md) for whole-row or whole-column calculations.
[REDUCE](FUNC.REDUCE.md) carries one accumulator through the array; [SCAN](FUNC.SCAN.md) also
returns the intermediate accumulators. [MAKEARRAY](FUNC.MAKEARRAY.md) generates values from
row and column indices.

## Sources

- [Microsoft: MAP function](https://support.microsoft.com/en-us/excel/functions/map-function).
- [Recorded helper-function error examples](../notes/FN-HSTACK-001.json).
