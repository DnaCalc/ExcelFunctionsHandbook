---
schema: efh.function-page/v1
function_id: FUNC.PERCENTILE
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

# PERCENTILE

## What it does

`PERCENTILE(array,k)` returns the data value at fractional position k in sorted numeric observations. This compatibility function uses the inclusive convention: zero is the minimum and one the maximum.

## Try it

`=PERCENTILE({10,20,30,40},0.25)` returns `17.5`. The two conventions interpolate at different positions even on the same four observations.

## Quirks

An empty numeric set gives `#NUM!`. Nonnumeric k gives `#VALUE!`. A k outside `[0,1]` gives `#NUM!`; the data need not be supplied in sorted order. Referenced text and blanks are not extra numeric observations.

## How to implement

Collect the admitted numbers and sort ascending. Set the one-based rank `h=1+k*(n-1)`. Reject a rank outside one through n. If h is an integer, return that observation; otherwise linearly interpolate `lower+(h-floor(h))*(upper-lower)`. In the example h is `1.75`, so the result is `10+0.75*(20-10)=17.5`. The current helper uses binary64 rank arithmetic; this outline does not establish its last bits for every Excel data set.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[PERCENTILE.INC](FUNC.PERCENTILE.INC.md) names the inclusive convention explicitly; [PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md) uses the exclusive one. [QUARTILE](FUNC.QUARTILE.md) selects a multiple of one quarter.

## Sources

- [Microsoft: PERCENTILE function](https://support.microsoft.com/en-us/excel/functions/percentile-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
