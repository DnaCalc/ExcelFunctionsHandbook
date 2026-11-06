---
schema: efh.function-page/v1
function_id: FUNC.ODDLYIELD
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

# ODDLYIELD

## What it does

`ODDLYIELD` solves annual yield from price for a security with an irregular last coupon. Its dates are settlement, maturity and last_interest. They are followed by coupon rate, price, redemption, frequency and optional basis.

## Try it

`=ODDLYIELD(DATE(2027,8,1),DATE(2027,11,1),DATE(2027,4,1),0.06,101.5,100,2,0)` gives a yield near zero. The last coupon spans April to November, seven 30/360 months. Its coupon is 3.5 and accrued interest at August settlement is two. At zero yield the clean price is `100+3.5-2=101.5`.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. The required date order is `last_interest < settlement < maturity`. Frequency is one, two or four. Negative coupon rate, invalid selectors and invalid amount/domain cases produce errors. These schedules cannot be reduced to PRICE by changing one date.

## How to implement

Build quasi-coupon periods from last_interest, accumulate normalized issue-to-maturity, accrued and remaining fractions, and solve the final simple-discount equation for yield. The current last-yield helper uses a direct algebraic formula rather than the odd-first root solver. The worked zero-yield example isolates the coupon and accrual arithmetic from discount powers. Calendar seams, long/short-period branches and residual numerical differences remain open where the recorded reports say so. The family combines a credited Apache-licensed ExcelFinancialFunctions translation in its odd-first kernel and calendar helpers with project-owned kernels and changes guided by recorded Excel answers.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[ODDLPRICE](FUNC.ODDLPRICE.md) prices the same last-coupon schedule at a supplied yield. [ODDFYIELD](FUNC.ODDFYIELD.md) handles an irregular first coupon; [YIELD](FUNC.YIELD.md) handles regular coupons.

## Sources

- [Microsoft: ODDLYIELD function](https://support.microsoft.com/en-us/excel/functions/oddlyield-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/odd_bond_family.rs); implementation steps and the limits identified in the text.
