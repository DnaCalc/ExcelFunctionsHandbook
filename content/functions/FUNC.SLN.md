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

Cost and salvage are amounts in the same units; life supplies the number of those periods. The function does not take a current-period argument or track accumulated depreciation. The current kernel returns `#DIV/0!` for zero life and has its own validation for negative life and nonfinite inputs.

## How to implement

Coerce the three inputs, validate the life under the desired worksheet policy, compute `cost-salvage`, then divide by life. Keep subtraction before division if reproducing the current binary64 graph. For the example `1200-200=1000`, followed by `1000/5=200`. A worksheet formatting choice such as currency changes display, not the mathematical amount. The simple formula does not supply jurisdiction-specific depreciation rules.

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
