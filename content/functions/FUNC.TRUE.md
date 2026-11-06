---
schema: efh.function-page/v1
function_id: FUNC.TRUE
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
family: true_fn
role_in_family: "The nullary logical constant TRUE."
---

# TRUE

## What it does

`TRUE()` returns the logical value TRUE. It takes no arguments. The literal `TRUE` can be
written directly in a formula or cell with the same purpose; Microsoft retains the function
primarily for compatibility with other spreadsheet programs.

TRUE is a logical value, distinct from numeric one, the text `"TRUE"`, and an empty cell.
A later operation may convert it to a number, but that conversion belongs to the operation
consuming the value.

## Try it

Calculate `=TRUE()`, then try `=ISLOGICAL(TRUE())` and `=ISNUMBER(TRUE())` to inspect its kind.
Use `=N(TRUE())` to request a numeric conversion explicitly.

## Quirks

A valid call has no input-dependent error. A call such as `TRUE(0)` has the wrong argument
count; handling an invalid formula is separate from calculating a valid constant.

Do not use TRUE as a substitute for a missing value. `ISBLANK(TRUE())` and an actual blank
cell answer different questions. Likewise, `"TRUE"` is text until a particular consumer
chooses to interpret it.

## How to implement

Check that the call has zero arguments and return a tagged logical true value. Do not return
numeric 1: `TYPE`, `ISLOGICAL`, `ISNUMBER` and comparisons can observe the distinction.

A compiler may fold the call to a logical literal if that preserves the formula representation
and any evaluation tracing it promises. FALSE and TRUE can share this path with one boolean
parameter. No floating-point arithmetic or clock is needed.

## Version differences

Microsoft documents the function for Microsoft 365 and Excel 2016 through 2024. No comparison
between particular builds is recorded here; the documentation's applicability list is not a
build-by-build test.

## Alternatives

Write the literal `TRUE` in ordinary formulas. [FALSE](FUNC.FALSE.md) is the opposite constant;
[NOT](FUNC.NOT.md) negates a logical test. [NA](FUNC.NA.md) is an explicit unavailable-value
marker when that is the intended meaning, and is an error rather than a logical value.

## Sources

- [Microsoft: TRUE function](https://support.microsoft.com/en-us/excel/functions/true-function).
- [Coercion and lifting](../model/02-coercion-and-lifting.md).
