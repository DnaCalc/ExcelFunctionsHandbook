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

The lambda takes exactly two parameters. Microsoft documents `#VALUE!` for an invalid callable, incorrect parameter count, dimensions below one or nonnumeric dimensions. The current code additionally rejects fractional dimensions rather than truncating them.

Each call must supply one cell value. The current helper returns `#VALUE!` for an array-valued lambda result, even a one-cell array; that differs from SCAN’s singleton-array rule. For example, `MAKEARRAY(1,1,LAMBDA(r,c,{1}))` takes this error route. A scalar value of one is a different result kind from the array `{1}`.

Dimensions are validated before the callable. With valid dimensions, a noncallable final argument is `#VALUE!` even if it is itself an error value. Spill obstructions and allocation limits belong to the workbook or evaluation host.

## How to implement

1. Read and coerce the two dimensions. Require finite positive integers; check `rows*cols` before allocating output storage.
2. Validate that the final argument is callable with two parameters. Keep its captured environment so names inside the lambda resolve as they did where it was created.
3. For each zero-based coordinate `(r,c)`, invoke the lambda with the numbers `(r+1,c+1)`. These are array coordinates, not worksheet row and column addresses.
4. Accept a scalar result at that coordinate. The current helper rejects an array or reference result with `#VALUE!`; it preserves ordinary worksheet error values as cell results.
5. Return the requested rectangular array. If evaluation can yield, retain the completed cells and next coordinate so resuming does not invoke already completed cells again.

For `10*r+c`, the first row uses `(1,1)`, `(1,2)`, `(1,3)` to produce `11,12,13`. The next row begins at `(2,1)`, producing 21. No source array is traversed: the dimensions define the coordinates.

## Version differences

A retained source comment reports `=MAKEARRAY(1,1,#REF!)` returning `#VALUE!` in Excel 16.0 build 20430 on 25 October 2026. The earlier OxFunc implementation propagated `#REF!`; the current helper validates the final argument as a callable instead. This is an implementation correction against a recorded Excel answer, not a demonstrated change between Excel builds. That comment does not specify CPU or platform.

[Recorded-control source](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/lambda_helpers.rs). No same-input comparison across named Excel builds is recorded here.

## Alternatives

[SEQUENCE](FUNC.SEQUENCE.md) is simpler for arithmetic progressions; [MAP](FUNC.MAP.md) transforms existing entries; [BYROW](FUNC.BYROW.md) reduces each row.

## Sources

- [Microsoft: MAKEARRAY function](https://support.microsoft.com/en-us/office/makearray-function-b80da5ad-b338-4149-a523-5b221da09097), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc callable validation, dimensions and result-cell rules](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/lambda_helpers.rs).
