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

Values and dates must align; a date before the first supplied date or differing counts gives `#NUM!`, while invalid dates give `#VALUE!`. Date serials are truncated. At least one positive and one negative flow are required. Later dates may be out of order, but the first supplied date remains the anchor.

Several sign changes can admit multiple roots. Guess can affect the selected result and its last digits even when the mathematical root is unique. Microsoft documents an iterative result accuracy of 0.000001 percent and a limit of 100 tries; failure gives `#NUM!`. That public stopping description does not identify every private residual or iteration step.

## How to implement

The mathematical equation is `sum(value_i/(1+r)^t_i)=0`, with `t_i=(date_i-date_0)/365`. It provides a residual, not a complete rule for choosing one root.

The current reconstruction uses the following routes after validating signed flows and aligned dates:

1. Its internal residual forms `base=1+r` with ordinary binary64 arithmetic. For each input in supplied order it calculates the binary64 year fraction, calls the internal runtime power leaf `crt_pow(base,t_i)`, divides the flow by it, and adds to a binary64 total. Rates at or below `-0.999999999` are outside this current solver’s accepted domain.
2. For exactly two flows, compute the candidate `(-value_1/value_0)^(1/t_1)-1`. Reject a nonpositive date difference or an invalid ratio. A negative candidate can return directly. For a nonnegative candidate, a negative or zero guess is rejected by the current path; it does not simply publish the closed form.
3. For the two-flow positive route, evaluate the residual at zero and at the guess. If they have the same sign, repeatedly move the lower endpoint up to the upper one and double the upper rate, with a limit of 128 expansions. Return an endpoint whose residual is exactly zero.
4. Bisect the bracket. Before testing the midpoint residual, the current route returns the midpoint when `(upper-lower)/abs(midpoint)<=2e-8`; otherwise replace the endpoint with the same residual sign. It permits at most 100 bisections.
5. For more than two flows, the positive-guess route also brackets and bisects, but uses width `<=1e-8*(1+2*abs(midpoint))`. A failed positive route can fall back to the source’s bounded secant/Newton solver. That fallback has its own derivative and bounds; it is not established by the two-flow recipe.

For the example, the residual is `-1000+1250/(1+r)`. Starting at guess 0.1 gives a positive residual, as does 0.2; 0.4 gives a negative one. Bisection tests 0.3 and then 0.25, where this simple example reaches zero. On less convenient inputs the width test can publish an approximation before an exactly zero residual appears.

Do not call the public XNPV wrapper as this internal residual: it uses worksheet POWER, stored x87 operations and a different rate guard. The route above describes the cited implementation and its retained controls. Full root-selection coverage, runtime-power last bits and transfer of every stopping boundary remain open.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[XNPV](FUNC.XNPV.md) is the paired dated calculation; [NPV](FUNC.NPV.md) and [IRR](FUNC.IRR.md) use equally spaced periods.

## Sources

- [Microsoft: XIRR function](https://support.microsoft.com/en-us/excel/functions/xirr-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/cashflow_rate_family.rs); implementation steps and the limits identified in the text.
