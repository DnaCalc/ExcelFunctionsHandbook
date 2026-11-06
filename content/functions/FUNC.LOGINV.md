---
schema: efh.function-page/v1
function_id: FUNC.LOGINV
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

# LOGINV

## What it does

`LOGINV(probability,mean,standard_dev)` returns the value `x` at a specified cumulative lognormal probability. The parameters describe **ln(X)**: `mean` is the location in log space and `standard_dev` its positive spread. They are not the arithmetic mean and standard deviation of the positive output variable. This is the compatibility spelling of LOGNORM.INV.

## Try it

`=LOGINV(0.5,0,1)` returns `1`. Probability one half selects the median; the log-space median is zero and exponentiating it gives one.

`=LOGINV(0.5,1,0.5)` is about `2.718281828459045`, the exponential of the log-space location one. Changing the positive spread does not move the mathematical median, although it changes other quantiles.

## Quirks

The probability must be strictly between zero and one. Either endpoint, an out-of-range probability or a nonpositive standard deviation gives `#NUM!`; nonnumeric arguments give `#VALUE!`. The location may be negative: that describes positive values below one rather than negative output values.

Extreme quantiles can overflow after exponentiation, or become so small that worksheet publication matters. A normal quantile in log space can be representable even when its exponential is not. Do not treat that outer exponential as harmless formatting.

## How to implement

1. Validate the open probability interval and positive finite log-space spread.
2. Obtain a standard-normal quantile `z` at `p`. The current candidate uses Acklam’s piecewise rational approximation followed by one Newton correction; that generic inverse is not an identified Excel inverse program.
3. Form `t=mean+standard_dev*z` with the current binary64 multiplication and addition order.
4. Call the measured public EXP chain on the stored `t`. The current code explicitly uses `excel_exp` here, rather than an arbitrary host exponential.

At `p=0.5`, the symmetric standard quantile is zero, so the calculation reduces to `EXP(mean)`. This explains both examples without settling the inverse approximation away from the median. Tail quantiles, input coercion and publication still need their own controls; a direct lognormal inverse library can have different intermediate values.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[LOGNORM.INV](FUNC.LOGNORM.INV.md) is the modern spelling. [NORM.DIST](FUNC.NORM.DIST.md) describes the normal variable in log space.

## Sources

- [Microsoft: LOGINV function](https://support.microsoft.com/en-us/excel/functions/loginv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
