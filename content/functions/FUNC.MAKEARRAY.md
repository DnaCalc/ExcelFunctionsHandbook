---
schema: efh.function-page/v1
function_id: FUNC.MAKEARRAY
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

# MAKEARRAY

## What it does

`MAKEARRAY(rows, cols, LAMBDA(row,col,calculation))` builds an array by calling a formula once for each coordinate. The lambda receives one-based row and column indices.

## Try it

`=MAKEARRAY(2,3,LAMBDA(r,c,10*r+c))` spills `11,12,13` above `21,22,23`. It is a coordinate-based construction rather than a traversal of an existing array.

## Quirks

Both dimensions must be positive. Microsoft documents `#VALUE!` for invalid dimensions or an invalid lambda/parameter count. Every invocation must produce an element that fits the array model; workbook spill constraints still apply.

## How to implement

Validate dimensions and the two-parameter callable. Allocate the output, invoke the lambda with `(r+1,c+1)` for each output coordinate and place the returned value at that coordinate. Keep lambda evaluation inside the normal formula evaluator so captured names and errors retain their meaning. Do not replace row and column indices with source cell addresses. Resource limits belong to the execution host.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[SEQUENCE](FUNC.SEQUENCE.md) is simpler for arithmetic progressions; [MAP](FUNC.MAP.md) transforms existing entries; [BYROW](FUNC.BYROW.md) reduces each row.

## Sources

- [Microsoft: MAKEARRAY function](https://support.microsoft.com/en-us/office/makearray-function-b80da5ad-b338-4149-a523-5b221da09097), opened 6 November 2026; syntax, documented argument rules and errors.
