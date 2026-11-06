---
schema: efh.function-page/v1
function_id: FUNC.SLN
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

# SLN

## What it does

`SLN(cost,salvage,life)` returns one period’s straight-line depreciation: an equal share of the depreciable amount over the stated life.

## Try it

`=SLN(1200,200,5)` returns `200`. The depreciable amount is 1000 and five equal periods divide it into shares of 200.

## Quirks

Life is a count of periods, with the same time unit used for the requested allowance. There is no current-period argument and no accumulated balance. The current kernel admits either sign of cost and salvage and any nonzero life, including negative life; it returns `#DIV/0!` for exactly zero life. That arithmetic admission is broader than an ordinary asset-depreciation schedule.

The subtraction and division have different rounding steps. A subnormal difference is retained until division, while a final subnormal amount is published as positive zero. Nonfinite numeric inputs or a nonfinite result give `#NUM!` in this kernel; failed worksheet numeric conversion is handled outside it. Microsoft's opened page defines the arguments but does not enumerate these edge codes.

## How to implement

1. Validate finite numeric inputs. Reject `life=0` with `#DIV/0!`; do not introduce a positive-life guard when reproducing the current route.
2. Compute `difference=cost-salvage` with ordinary binary64 subtraction. Preserve a subnormal difference at this point.
3. Compute `result=R(difference/life)` with the [stored operation](../model/09-staged-arithmetic.md): round the quotient to a 64-bit significand, then store to binary64. A single ordinary binary64 divide is a different graph.
4. If the result is nonfinite, return `#NUM!`. If its magnitude is below the minimum normal binary64 value, return positive zero. Otherwise publish it.

For `1200,200,5`, the difference is exactly `1000` and the quotient exactly `200`. Changing life to `-5` makes this code return `-200`; it does not create a different depreciation method. Changing salvage changes the depreciable amount before the division.

The source attributes this quotient and final-flush route to retained Excel build-20430, compatibility-version-2 probes. Its input conversion and worksheet context are separate obligations; this recipe is not a published Handbook conformance suite.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 8,852 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SLN.json).

## Alternatives

[SYD](FUNC.SYD.md) front-loads the allowance; [DB](FUNC.DB.md) and [DDB](FUNC.DDB.md) use declining-balance methods.

## Sources

- [Microsoft: SLN function](https://support.microsoft.com/en-us/excel/functions/sln-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/depreciation_family.rs); implementation steps and the limits identified in the text.
