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

Obtain the basis-specific days A from settlement to maturity and year denominator B. The mathematical expression is `((redemption-pr)/pr)*(B/A)`. The current kernel preserves a particular division/multiplication graph and day-count helper; use those stored steps when comparing its numerical output. In the example the return fraction is `4/96`, annualization is `360/30=12`, and their product is one half. Do not replace an actual/actual year denominator by 365 in every calendar interval.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[DISC](FUNC.DISC.md) uses a discount-on-redemption rate; [PRICEDISC](FUNC.PRICEDISC.md) solves price; [TBILLYIELD](FUNC.TBILLYIELD.md) applies its bill-specific convention.

## Sources

- [Microsoft: YIELDDISC function](https://support.microsoft.com/en-us/excel/functions/yielddisc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bond_core_family.rs); implementation steps and the limits identified in the text.
