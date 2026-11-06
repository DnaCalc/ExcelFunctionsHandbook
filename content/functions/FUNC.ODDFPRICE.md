---
schema: efh.function-page/v1
function_id: FUNC.ODDFPRICE
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

# ODDFPRICE

## What it does

`ODDFPRICE` computes clean price per 100 face value for a security with an irregular first coupon. Its dates are settlement, maturity, issue and first_coupon. They are followed by coupon rate, yield, redemption, frequency and optional basis.

## Try it

`=ODDFPRICE(DATE(2027,2,1),DATE(2027,10,1),DATE(2027,1,1),DATE(2027,4,1),0.06,0,100,2,0)` gives a price near `104`. The first coupon spans January to April, a short half-year coupon. Under 30/360 its payment is 1.5; the regular maturity coupon is three; accrued interest at February settlement is 0.5. At zero yield the clean price is `100+1.5+3-0.5=104`.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. The required date order is `issue < settlement < first_coupon < maturity`. Frequency is one, two or four. Negative coupon rate, invalid selectors and invalid amount/domain cases produce errors. These schedules cannot be reduced to PRICE by changing one date.

## How to implement

Generate quasi-coupon periods aligned to first_coupon and maturity. Prorate the odd first coupon across those periods, discount it and later coupons/redemption, and subtract accrued interest. Long first periods can contain multiple quasi-coupons. The worked zero-yield example isolates the coupon and accrual arithmetic from discount powers. Calendar seams, long/short-period branches and residual numerical differences remain open where the recorded reports say so. The current odd-first code includes an Apache-licensed translation of ExcelFinancialFunctions changed after Excel measurements; its method is not solely inferred from black-box readings.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[ODDFPRICE](https://handbook.dnacalc.com/functions/ODDFPRICE.html) and [ODDFYIELD](https://handbook.dnacalc.com/functions/ODDFYIELD.html) form the corresponding price/yield pair. [PRICE](FUNC.PRICE.md) and [YIELD](FUNC.YIELD.md) handle regular coupons.

## Sources

- [Microsoft: ODDFPRICE function](https://support.microsoft.com/en-us/excel/functions/oddfprice-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/odd_bond_family.rs); implementation steps and the limits identified in the text.
