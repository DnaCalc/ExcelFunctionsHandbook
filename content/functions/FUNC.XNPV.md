---
schema: efh.function-page/v1
function_id: FUNC.XNPV
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

# XNPV

## What it does

`XNPV(rate,values,dates)` discounts cash flows from their actual dates to the first supplied date. Annual exponents use day differences divided by 365, rather than a selectable bond basis.

## Try it

Put `-1000,1250` in B2:B3 and enter `=DATE(2027,1,1)` and `=DATE(2028,1,1)` in C2:C3. `=XNPV(0.25,B2:B3,C2:C3)` is near zero: the first amount is undiscounted and the second divides by 1.25.

## Quirks

Values and dates must align; a date before the first supplied date or differing counts gives `#NUM!`, while invalid dates give `#VALUE!`. Date serials are truncated. The current XNPV kernel requires a positive rate; its zero/negative-rate refusal is a compatibility limitation beyond the finite mathematical sum. The first date is an anchor, not an instruction to sort the inputs.

## How to implement

Validate aligned dated values and construct the stored base `R(1+r)`. For each date form the binary64 year fraction `(date-date0)/365`, call the worksheet power route, store `R(value/power)` and accumulate using stored addition. These are current x87 operation sites. In the example the exponents are zero and one, giving `-1000+1000=0`. Root and power last-bit stages remain partly open.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 1,703 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.XNPV.json).

## Alternatives

[XIRR](https://handbook.dnacalc.com/functions/XIRR.html) is the paired dated calculation; [NPV](FUNC.NPV.md) and [IRR](FUNC.IRR.md) use equally spaced periods.

## Sources

- [Microsoft: XNPV function](https://support.microsoft.com/en-us/excel/functions/xnpv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/cashflow_rate_family.rs); implementation steps and the limits identified in the text.
