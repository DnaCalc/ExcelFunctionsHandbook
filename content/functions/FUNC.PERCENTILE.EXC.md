---
schema: efh.function-page/v1
function_id: FUNC.PERCENTILE.EXC
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

# PERCENTILE.EXC

## What it does

`PERCENTILE.EXC(array,k)` returns the data value at fractional position k in sorted numeric observations. It uses the exclusive convention, whose positions are based on `n+1`; the endpoints zero and one are excluded.

## Try it

`=PERCENTILE.EXC({10,20,30,40},0.25)` returns `12.5`. The two conventions interpolate at different positions even on the same four observations.

## Quirks

An empty numeric set gives `#NUM!`. Nonnumeric k gives `#VALUE!`. Even `0<k<1` can be outside the interpolable sample positions and give `#NUM!`; no extrapolation beyond the observed endpoints is supplied. Referenced text and blanks are not extra numeric observations.

## How to implement

Collect the admitted numbers and sort ascending. Set the one-based rank `h=k*(n+1)`. Reject a rank outside one through n. If h is an integer, return that observation; otherwise linearly interpolate `lower+(h-floor(h))*(upper-lower)`. In the example h is `1.25`, so the result is `10+0.25*(20-10)=12.5`. The current helper uses binary64 rank arithmetic; this outline does not establish its last bits for every Excel data set.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[PERCENTILE.INC](FUNC.PERCENTILE.INC.md) names the inclusive convention explicitly; [PERCENTILE.EXC](https://handbook.dnacalc.com/functions/PERCENTILE.EXC.html) uses the exclusive one. [QUARTILE](https://handbook.dnacalc.com/functions/QUARTILE.html) selects a multiple of one quarter.

## Sources

- [Microsoft: PERCENTILE.EXC function](https://support.microsoft.com/en-us/excel/functions/percentile-exc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
