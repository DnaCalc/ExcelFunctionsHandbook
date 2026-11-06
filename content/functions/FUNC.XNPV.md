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

Values and dates must align; a date before the first supplied date or differing counts gives `#NUM!`, while invalid dates give `#VALUE!`. Date serials are truncated. The first date is an anchor, rather than an instruction to sort the inputs.

The current XNPV kernel requires a positive rate and returns `#NUM!` at zero or a negative rate, even when the mathematical dated sum is finite. Its source records that guard as measured; the opened Microsoft page does not explicitly state it. This differs from the internal residual used by XIRR, whose iterates can go negative.

Microsoft describes both a positive and a negative cash flow as required. The current helper and its retained all-positive source control accept `100,200` at dates `45000,45100` and rate `0.1`, giving about `294.8451203808644`. Keep this source-scoped observation separate from the documented sign requirement.

## How to implement

Use `D` for ordinary binary64 storage and [R](../model/09-staged-arithmetic.md) for the stored x87-style operation.

1. Validate finite numeric flows, aligned truncated dates and the first-date anchor. Require a finite positive rate in the current public route.
2. Calculate `base=R(1+rate)` and initialize `total=0`.
3. For each flow in its supplied order, calculate `years=D((date-date_0)/365)`. Call the public worksheet [POWER](FUNC.POWER.md) route on `(base,years)`; integer year counts can select its integer-exponent path.
4. Store `term=R(value/power)`, then `total=R(total+term)`. Preserve every store and the forward accumulation order. Apply the final finite-result and worksheet publication rules.

For the example, the year counts are zero and one, the powers one and 1.25, and the terms minus 1000 and plus 1000. The stored sum is zero. By contrast, putting the first payment into NPV would discount it by one period.

This current arithmetic chain is distinct from XIRR’s runtime-power, ordinary-binary64 residual. POWER’s complete fractional-exponent behaviour and transfer to other environments are not solved by the dated-sum loop.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 1,703 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.XNPV.json).

## Alternatives

[XIRR](FUNC.XIRR.md) is the paired dated calculation; [NPV](FUNC.NPV.md) and [IRR](FUNC.IRR.md) use equally spaced periods.

## Sources

- [Microsoft: XNPV function](https://support.microsoft.com/en-us/excel/functions/xnpv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/cashflow_rate_family.rs); implementation steps and the limits identified in the text.
