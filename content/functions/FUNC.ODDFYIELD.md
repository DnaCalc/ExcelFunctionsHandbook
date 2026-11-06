---
schema: efh.function-page/v1
function_id: FUNC.ODDFYIELD
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

# ODDFYIELD

## What it does

`ODDFYIELD` solves annual yield from price for a security with an irregular first coupon. Its dates are settlement, maturity, issue and first_coupon. They are followed by coupon rate, price, redemption, frequency and optional basis.

## Try it

`=ODDFYIELD(DATE(2027,2,1),DATE(2027,10,1),DATE(2027,1,1),DATE(2027,4,1),0.06,104,100,2,0)` gives a yield near zero. The first coupon spans January to April, a short half-year coupon. Under 30/360 its payment is 1.5; the regular maturity coupon is three; accrued interest at February settlement is 0.5. At zero yield the clean price is `100+1.5+3-0.5=104`.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. The required date order is `issue < settlement < first_coupon < maturity`. Frequency is one, two or four. Negative coupon rate, invalid selectors and invalid amount/domain cases produce errors. These schedules cannot be reduced to PRICE by changing one date.

## How to implement

Build the odd-first pricing function over a quasi-coupon calendar, then bracket a yield that reprices the target clean price. The current helper uses bisection, while Microsoft’s page describes a Newton method. Their stopping and boundary results are separate compatibility questions. The worked zero-yield example isolates the coupon and accrual arithmetic from discount powers. Calendar seams, long/short-period branches and residual numerical differences remain open where the recorded reports say so. The family combines a credited Apache-licensed ExcelFinancialFunctions translation in its odd-first kernel and calendar helpers with project-owned kernels and changes guided by recorded Excel answers.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[ODDFPRICE](FUNC.ODDFPRICE.md) prices the same first-coupon schedule at a supplied yield. [ODDLYIELD](FUNC.ODDLYIELD.md) handles an irregular final coupon; [YIELD](FUNC.YIELD.md) handles regular coupons.

## Sources

- [Microsoft: ODDFYIELD function](https://support.microsoft.com/en-us/excel/functions/oddfyield-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/odd_bond_family.rs); implementation steps and the limits identified in the text.
