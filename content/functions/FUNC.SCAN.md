---
schema: efh.function-page/v1
function_id: FUNC.SCAN
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

# SCAN

## What it does

`SCAN([initial_value], array, LAMBDA(accumulator,value,calculation))` returns every intermediate value of a running calculation. Each invocation receives the previous accumulator and the next input.

## Try it

`=SCAN(0,{3,1,4},LAMBDA(a,v,a+v))` returns `{3,4,8}`. The initial zero starts the calculation but is not an extra output element.

For text, `=SCAN("",{"d","n","a"},LAMBDA(a,v,a&v))` returns `{"d","dn","dna"}`. Each result contains the state after consuming the next letter. The accumulator need not be numeric.

## Quirks

The lambda needs two parameters: the previous accumulator and the current value. The calculation is its body, not a third lambda parameter. An invalid callable or incorrect parameter count gives `#VALUE!`. Set the initial value to `""` for text concatenation.

The current helper traverses rows from left to right, then advances to the next row. It returns the input’s shape. It admits a one-cell array result by publishing that cell, but an intermediate result with several cells gives `#CALC!`; SCAN cannot spill a separate subarray from every input position. REDUCE has a different publication rule and can retain an array as its final accumulator.

Empty cells and error values remain values passed to the callable. The body can decide how to handle them; reducing everything to a prefiltered numeric list would change the function.

## How to implement

1. Prepare the initial accumulator and input, reading references as their held values. Preserve input shape and value kinds.
2. Validate a two-parameter callable. Traverse input cells in row order.
3. At each cell, invoke the lambda with `(accumulator,current_value)`. Replace the accumulator with the returned value, then publish the admissible scalar or singleton cell at the corresponding output coordinate.
4. Retain the updated accumulator for the next invocation. Do not restart it at a row boundary and do not include the initial value as a separate output.
5. Return the array in the input shape. A yielding evaluator must preserve the accumulator and next position across suspension.

The current route for `SCAN(0,{1,2;3,4},LAMBDA(a,v,10*a+v))` produces first row `1,12`, then second row `123,1234`. This makes traversal order visible and shows that row two continues the state from row one. The result follows from the cited helper; it is not a fresh Excel capture.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[REDUCE](FUNC.REDUCE.md) returns just the final accumulator; [MAP](FUNC.MAP.md) transforms elements independently; [MAKEARRAY](FUNC.MAKEARRAY.md) builds from coordinates.

## Sources

- [Microsoft: SCAN function](https://support.microsoft.com/en-us/office/scan-function-d58dfd11-9969-4439-b2dc-e7062724de29), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc row-order traversal, accumulator state and singleton publication](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/lambda_helpers.rs).
