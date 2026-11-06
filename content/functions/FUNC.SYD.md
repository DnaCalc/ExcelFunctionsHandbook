---
schema: efh.function-page/v1
function_id: FUNC.SYD
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

# SYD

## What it does

`SYD(cost,salvage,life,per)` returns depreciation for a specified period by the sum-of-years’ digits method. Earlier periods receive larger weights.

## Try it

`=SYD(1200,200,4,1)` returns `400`. The four weights are four, three, two and one; the first period gets `4/10` of the depreciable 1000. The last period gets 100.

## Quirks

Period and life use the same time unit. The current kernel requires positive life and a period inside its admitted range; invalid values produce worksheet errors. The result is a single period’s amount, not cumulative depreciation through that period.

## How to implement

Compute `depreciable=cost-salvage` and the mathematical denominator `life*(life+1)/2`. Weight the requested period by `life-per+1`, giving `depreciable*(life-per+1)/denominator`. Preserve the current source’s multiplication/division ordering when numerical compatibility matters; rearranging the factor two changes possible rounding. For life four the denominator is ten and the weights total ten. Fractional life or period needs the kernel’s explicit admission policy rather than an invented calendar convention.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 50,859 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.SYD.json).

## Alternatives

[SLN](FUNC.SLN.md) uses equal allowances; [DB](FUNC.DB.md) and [DDB](FUNC.DDB.md) use a declining balance.

## Sources

- [Microsoft: SYD function](https://support.microsoft.com/en-us/excel/functions/syd-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/depreciation_family.rs); implementation steps and the limits identified in the text.
