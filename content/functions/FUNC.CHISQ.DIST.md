---
schema: efh.function-page/v1
function_id: FUNC.CHISQ.DIST
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

# CHISQ.DIST

## What it does

`CHISQ.DIST(x, deg_freedom, cumulative)` returns a chi-square CDF when cumulative is TRUE, or its density when FALSE.

## Try it

`=CHISQ.DIST(2,2,TRUE)` is about `0.632120558828558`. At two degrees of freedom the right tail is `exp(-x/2)`. The CDF is its mathematical complement, while density at x=2 is about `0.183939720585721`.

## Quirks

Degrees of freedom are truncated. Negative x and an admitted degree count below one or above `10^10` give `#NUM!` in the current shared family. Nonnumeric arguments give `#VALUE!`. A density is not the probability of the exact input value; use a CDF difference for an interval.

## How to implement

For the cumulative branch the current kernel calls the gamma CDF at shape `df/2`, scale two. Its df=1 route is ERF.PRECISE of `sqrt(x/2)`; its df=2 route uses the exponential CDF. For a density at positive x the mathematical expression is `x^(df/2-1)*exp(-x/2)/(2^(df/2)*Gamma(df/2))`; handle x=0 limits explicitly. The worked df=2 value follows from `x/2=1`. The numerical outline is an implementation route, not an identification of every Excel approximation or return-rounding stage. The current problem-functions report retains unresolved distribution mismatches.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[CHISQ.INV](FUNC.CHISQ.INV.md) inverts the corresponding probability. [CHISQ.TEST](FUNC.CHISQ.TEST.md) computes a test probability from observed and expected counts.

## Sources

- [Microsoft: CHISQ.DIST function](https://support.microsoft.com/en-us/excel/functions/chisq-dist-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/chi_f_t_family.rs); implementation steps and the limits identified in the text.
