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

## Quirks

The lambda needs two parameters; an invalid callable or parameter count gives `#VALUE!`. Use `""` as the initial value for text accumulation. The output follows the input shape, so its size is known before accumulation.

## How to implement

Traverse the input in the evaluator’s scan order. Initialise `a`, invoke the lambda with `(a,value)`, assign its result to `a`, and store that result in the corresponding output position. In the example the successive states are `3`, `4`, `8`. Preserve errors and value kinds through the callable; an implementation that keeps only a numeric running sum cannot implement text scans.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[REDUCE](FUNC.REDUCE.md) returns just the final accumulator; [MAP](FUNC.MAP.md) transforms elements independently; [MAKEARRAY](FUNC.MAKEARRAY.md) builds from coordinates.

## Sources

- [Microsoft: SCAN function](https://support.microsoft.com/en-us/office/scan-function-d58dfd11-9969-4439-b2dc-e7062724de29), opened 6 November 2026; syntax, documented argument rules and errors.
