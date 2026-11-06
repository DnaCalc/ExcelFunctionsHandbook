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

`LOGINV(probability,mean,standard_dev)` returns a lognormal quantile. Mean and standard deviation describe **ln(X)**, not X. This is the compatibility form.

## Try it

`=LOGINV(0.5,0,1)` returns `1`: the median is `exp(0)` even though the mean of the lognormal variable is different.

## Quirks

Nonpositive standard deviation gives `#NUM!`; nonnumeric inputs give `#VALUE!`. Probability must be strictly between zero and one. Exponentiating a large positive quantile can overflow.

## How to implement

Find the standard-normal quantile z, form the log-space location `mean+sigma*z` and exponentiate. The current implementation uses the generic normal inverse and runtime exponential; it is not an identified Excel inverse program. At p=0.5 and mean zero, z=0 and exponentiation returns one. Keep overflow, tail and text coercion tests separate from this mathematical transformation.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[LOGNORM.INV](FUNC.LOGNORM.INV.md) is the modern spelling. [NORM.DIST](FUNC.NORM.DIST.md) describes the normal variable in log space.

## Sources

- [Microsoft: LOGINV function](https://support.microsoft.com/en-us/excel/functions/loginv-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_log_family.rs); implementation steps and the limits identified in the text.
