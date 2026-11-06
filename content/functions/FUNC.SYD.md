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

Life and period use the same time unit and are not automatically truncated to whole years. The current kernel admits negative cost, but rejects negative salvage, nonpositive life or period, and a period above life when their positive difference is at least the minimum normal binary64 value. The very-small-difference condition is an explicit compatibility guard.

The output is one period's depreciation. It is not the total through that period. The denominator's overflow/subnormal handling differs from the numerator's: the current denominator becomes zero and gives `#DIV/0!`, while a nonfinite basis or final result gives `#NUM!`. Microsoft's opened page describes the method without spelling out these numerical branches.

## How to implement

Let `R` be the [stored 64-bit-significand operation](../model/09-staged-arithmetic.md), and let `D` mean ordinary binary64 arithmetic. First validate finite inputs and the guards above.

1. Store `L=R(life+1)`. Compute `den=D(R(life*L)/2)`. If den is nonfinite or has magnitude below the minimum normal value, replace it with zero; reject a zero denominator with `#DIV/0!`.
2. Compute `basis=R(cost-salvage)`. Reject a nonfinite basis and flush a subnormal basis to positive zero.
3. Compute `remaining=R(L-per)`, then `numerator=R(basis*remaining)`. Keep `(life+1)-per` in that order: `life-per+1` can lose different low bits.
4. Compute `result=R(numerator/den)`. Reject nonfinite output with `#NUM!` and publish a subnormal result as positive zero.

Worked at `cost=1200,salvage=200,life=4,per=1`: `L=5`, `den=4*5/2=10`, `basis=1000`, `remaining=4`, `numerator=4000`, result `400`. At periods two, three and four the remaining weights are three, two and one, giving `300,200,100`. These four exact small examples total the depreciable 1000.

A ratio-first implementation of the same mathematical equation has different rounded intermediates. The current source's selected store discriminants support this operation order; they do not settle every data-admission or workbook route.

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
