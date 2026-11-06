---
schema: efh.function-page/v1
function_id: FUNC.YIELDDISC
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

# YIELDDISC

## What it does

`YIELDDISC(settlement,maturity,pr,redemption,[basis])` returns the annual investment yield of a discounted security. Both amounts are expressed per the same face-value unit.

## Try it

`=YIELDDISC(DATE(2027,1,1),DATE(2027,1,31),96,100,2)` is about `0.5`. The gain is four on a price of 96, annualized from thirty actual days using a 360-day year.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. Price and redemption must be positive and settlement before maturity. Invalid dates give `#VALUE!`; invalid amounts, date order or basis give `#NUM!`. This is yield on purchase price, not the discount rate on redemption.

## How to implement

1. Truncate the dates and basis, validate the workbook dates, require positive price and redemption and settlement before maturity.
2. Obtain the numerator day count `days` for the selected basis. Bases 0 and 4 use their respective 30/360 rules; bases 1, 2 and 3 use actual serial days.
3. Obtain the year denominator B: 360 for bases 0, 2 and 4; 365 for basis 3; the source’s calendar-dependent `days_in_year_for_mat` for basis 1. The latter can use 365, 366 or a multi-year average. Do not silently substitute a fixed 365 or call a different YEARFRAC algorithm.
4. Using ordinary binary64 stores D, form `gain=D(redemption-price)`, `relative_gain=D(gain/price)`, `year_scale=D(B/days)` and `yield=D(relative_gain*year_scale)`.

For the example the gain is four, relative gain is about `0.041666666666666664`, and year scale is `360/30=12`. Their product is one half. The year ratio is computed separately: `((red-price)/price)*B/days` is another association with potentially different low bits.

The retained source records 25 of 25 readings distinguishing this grouped year ratio; the left-to-right candidate matched 17. Its calendar scaffolding and initial formula include a credited, changed translation of the Apache-licensed ExcelFinancialFunctions project, alongside project-owned changes guided by recorded answers. This scope does not establish every date seam or whole-function numerical agreement.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[DISC](FUNC.DISC.md) uses a discount-on-redemption rate; [PRICEDISC](FUNC.PRICEDISC.md) solves price; [TBILLYIELD](FUNC.TBILLYIELD.md) applies its bill-specific convention.

## Sources

- [Microsoft: YIELDDISC function](https://support.microsoft.com/en-us/excel/functions/yielddisc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bond_core_family.rs); implementation steps and the limits identified in the text.
