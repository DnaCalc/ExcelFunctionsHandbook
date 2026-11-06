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

`=PERCENTILE.EXC({10,20,30,40},0.25)` returns `12.5`: its one-based position is `5*0.25=1.25`, a quarter of the distance from ten to twenty.

For these four observations, `k=0.2` reaches the minimum ten and `k=0.8` the maximum forty. `k=0.1` is below the first supported position and gives `#NUM!`. Being strictly between zero and one is necessary but does not guarantee an interpolable position.

## Quirks

An empty numeric set gives `#NUM!`; nonnumeric `k` gives `#VALUE!`. Zero and one are excluded. In exact arithmetic, `n` observations support fractions from `1/(n+1)` through `n/(n+1)`; outside that interval the function does not extrapolate.

Referenced text, logical values and blanks are not additional numeric observations. Zeros and repeated numbers do count, so deleting them changes the rank positions. The current helper with one observation accepts the middle position `k=0.5` and returns that observation; other fractions fall outside its one-position interval. These collection and indexing rules are separate from floating-point boundary rounding.

## How to implement

1. Collect and sort the admitted numeric observations, keeping duplicates. Reject an empty set.
2. Form the one-based rank `h=k*(n+1)` in binary64. Reject it if `h<1` or `h>n`; no neighbouring values exist for extrapolation beyond those bounds.
3. Set `j=floor(h)` and `u=ceil(h)`. An integral rank returns sorted observation `j` directly.
4. Otherwise interpolate `lower+(h-j)*(upper-lower)` between observations `j` and `u`, after converting one-based positions to array indices.

With four observations and `k=0.25`, `h=1.25`, so the result is `10+0.25*(20-10)=12.5`. The inclusive convention uses `1+k*(n-1)=1.75` on the same data and consequently gives 17.5.

This is the current helper’s complete rank construction, with ordinary binary64 interpolation. Exact floating-point endpoint edges and last-bit agreement for arbitrary Excel data sets are not established by these worked examples. A different percentile convention cannot be repaired merely by changing output formatting.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[PERCENTILE.INC](FUNC.PERCENTILE.INC.md) includes the endpoint convention. [QUARTILE.EXC](FUNC.QUARTILE.EXC.md) selects exclusive quarter positions; [PERCENTRANK.EXC](FUNC.PERCENTRANK.EXC.md) maps a value to a fractional standing.

## Sources

- [Microsoft: PERCENTILE.EXC function](https://support.microsoft.com/en-us/excel/functions/percentile-exc-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
