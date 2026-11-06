---
schema: efh.function-page/v1
function_id: FUNC.SUM
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

# SUM

## What it does

`SUM(number1, [number2], ...)` adds individual numbers and numbers held in ranges or arrays. A range keeps a total tied to the cells that contain the data; mixing a range with further scalar arguments adds all of them.

## Try it

`=SUM(7,-2,5)` returns `10`. With B2:B4 containing `7`, the text `pending`, and `5`, `=SUM(B2:B4)` returns `12` because the referenced text is skipped.

Direct constants use a different admission rule: `=SUM(1,TRUE,"2")` adds one, one and two to give four. Putting those three values in an array, `=SUM({1,TRUE,"2"})`, admits only the numeric one. A cell's numeric value and its text spelling are different inputs.

## Quirks

Referenced text, logical values and blanks are skipped; numeric zero participates. Direct logicals and numeric text convert, while unconvertible direct text causes `#VALUE!`.

Recorded error controls give direct-argument failures priority over errors held inside ranges or arrays. For example, `SUM({1,#REF!},"a")` gives `#VALUE!`. Among held errors, reading order matters. A later worksheet error can also determine the result after the numeric accumulator has overflowed.

Changing the order of large positive and negative terms can change a floating-point sum. Compensation or sorting is a different numerical recipe, even when it is more accurate mathematically.

## How to implement

1. Preserve each argument's origin: direct scalar, reference or array. Validate direct arguments through numeric conversion first, in argument order, before processing held errors.
2. Visit arguments in order. For a reference visit areas in order and cells in row order; for an array visit elements in row order. Admit held numbers, skip held text/logicals/blanks, and propagate errors under the priority above.
3. Begin with `total=0`. For each admitted number x, assign `total=R(total+x)`, where [R](../model/09-staged-arithmetic.md) rounds to a 64-bit significand and then stores to binary64. Keep one store per addition. Publish the final number under the worksheet finite/subnormal policy.

For `7,-2,5`, the accumulator is `0 → 7 → 5 → 10`. A direct scalar that converts successfully must still be added at its original traversal position; the initial validation pass is not an instruction to sum scalars ahead of ranges. The current implementation also reads arrays boxed inside a GROUPBY result through to their elements. Keep that extension distinct from ordinary rectangular-array collection. The cited source and captures establish these specific routes; no published Handbook suite is supplied here.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,211 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SUM.json).

## Alternatives

Use [SUMIFS](FUNC.SUMIFS.md) for conditions, [SUMPRODUCT](FUNC.SUMPRODUCT.md) for weighted products, or [SUBTOTAL](FUNC.SUBTOTAL.md) when filtering and hidden-row rules matter.

## Sources

- [Aggregate fold and error controls](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/aggregate.rs).
- [Stored addition primitive](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/kernel.rs).
- [Microsoft: SUM function](https://support.microsoft.com/en-us/office/sum-function-043e1c7d-7726-4e80-8f32-07b23e057f89), opened 6 November 2026; syntax, documented argument rules and errors.
