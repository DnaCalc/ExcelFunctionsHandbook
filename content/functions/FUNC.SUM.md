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

`=SUM(7,-2,5)` returns `10`. With A1:A3 containing `7`, the text `pending`, and `5`, `=SUM(A1:A3)` returns `12` because the referenced text is skipped.

## Quirks

Referenced text and empty cells are ignored. A zero is a number and participates. Ordinary error values in the data can stop the total. A cell formatted as text is not made numeric by its appearance. Changing the evaluation order of large positive and negative terms can change a floating-point result.

## How to implement

Walk arguments in order and retain whether a value arrived directly or through a reference. Apply the aggregate inclusion rules before adding. Begin at zero and add admitted numbers sequentially; propagate errors according to the argument route. The imported implementation declares a sequential fold. A compensated or pairwise sum can improve numerical accuracy, but has a different rounding history. For `7,-2,5`, the accumulator is `0 → 7 → 5 → 10`. The reduction order and worksheet error policy need separate tests from the mathematical sum.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit-bit, CPU not recorded | 1,211 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SUM.json).

## Alternatives

Use [SUMIFS](FUNC.SUMIFS.md) for conditions, [SUMPRODUCT](FUNC.SUMPRODUCT.md) for weighted products, or [SUBTOTAL](FUNC.SUBTOTAL.md) when filtering and hidden-row rules matter.

## Sources

- [Microsoft: SUM function](https://support.microsoft.com/en-us/office/sum-function-043e1c7d-7726-4e80-8f32-07b23e057f89), opened 6 November 2026; syntax, documented argument rules and errors.
