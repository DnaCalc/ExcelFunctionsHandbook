---
schema: efh.function-page/v1
function_id: FUNC.XIRR
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

# XIRR

## What it does

`XIRR(values,dates,[guess])` finds an annualized rate that makes the date-discounted cash-flow sum zero. It accommodates irregular dates and defaults guess to 0.1.

## Try it

Put `-1000,1250` in B2:B3 and enter `=DATE(2027,1,1)` and `=DATE(2028,1,1)` in C2:C3. `=XIRR(B2:B3,C2:C3)` is about `0.25`: one year’s growth takes 1000 to 1250.

## Quirks

Values and dates must align; a date before the first supplied date or differing counts gives `#NUM!`, while invalid dates give `#VALUE!`. Date serials are truncated. At least one positive and one negative flow are required. Several sign changes can admit multiple roots, and failure to converge gives `#NUM!`.

## How to implement

Solve `sum(value_i/(1+r)^((date_i-date_0)/365))=0` using the supplied guess, domain guards and a convergence test. A bracketed fallback makes a mathematical solver safer than unguarded Newton steps, but it need not reproduce Excel’s chosen root. The current code has a distinct two-cash-flow route and other iterative routes; do not assume its internal residual is the public XNPV wrapper. In the example the equation reduces to `-1000+1250/(1+r)=0`. Root and power last-bit stages remain partly open.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[XNPV](FUNC.XNPV.md) is the paired dated calculation; [NPV](FUNC.NPV.md) and [IRR](FUNC.IRR.md) use equally spaced periods.

## Sources

- [Microsoft: XIRR function](https://support.microsoft.com/en-us/excel/functions/xirr-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/cashflow_rate_family.rs); implementation steps and the limits identified in the text.
