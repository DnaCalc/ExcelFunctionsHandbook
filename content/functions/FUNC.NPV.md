---
schema: efh.function-page/v1
function_id: FUNC.NPV
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

# NPV

## What it does

`NPV(rate,value1,[value2],...)` discounts equally spaced future cash flows to the point one period before the first amount. The first listed amount is at the end of period one, not at time zero.

## Try it

`=NPV(0.25,125,156.25)` returns about `200`: `125/1.25=100` and `156.25/1.25^2=100`. A separate time-zero outflow of 200 gives a net value near zero when added outside NPV.

## Quirks

Order determines timing. Use numeric zero to represent a period with no cash flow; an ignored blank or text entry can remove a position from the numeric sequence. The documented data-admission rules distinguish numerical values from text, logical values and blanks. Rate minus one makes a discount denominator zero.

## How to implement

Collect the admitted numeric flows in worksheet order. The current kernel sets `base=1+rate`, `discount=1`, `total=0`, then for each flow assigns `discount*=base` followed by `total+=flow/discount`. Reject a zero base and nonfinite final results under its error policy. A power-per-term implementation or reverse Horner fold has a different rounding history. The two-flow example advances the factor through 1.25 and 1.5625. This numerical fold remains distinct from its array/reference admission policy.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 142 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.NPV.json).

## Alternatives

[XNPV](https://handbook.dnacalc.com/functions/XNPV.html) uses actual dates; [PV](FUNC.PV.md) assumes constant payments; [IRR](FUNC.IRR.md) finds a periodic cash-flow root.

## Sources

- [Microsoft: NPV function](https://support.microsoft.com/en-us/excel/functions/npv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/financial_time_value_family.rs); implementation steps and the limits identified in the text.
