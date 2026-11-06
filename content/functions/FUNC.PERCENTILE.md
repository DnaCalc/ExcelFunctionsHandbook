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

`PERCENTILE(array,k)` returns a value at a fractional position in sorted numeric observations. This compatibility function uses the inclusive convention: `k=0` is the minimum, `k=1` the maximum, and `k=0.5` the median. Supply a fraction such as `0.25` or a numeric `25%`, not the number 25.

## Try it

`=PERCENTILE({10,20,30,40},0.25)` returns `17.5`. Its position is 1.75, between the first and second sorted observations.

`=PERCENTILE({40,10,30,20},0.25)` has the same value: input order does not define the percentile. At `k=0` and `k=1`, the same data returns 10 and 40. The exclusive convention applied at `0.25` returns `12.5`, so the percentile convention is part of the question.

## Quirks

An empty numeric set gives `#NUM!`. A fraction outside `[0,1]` gives `#NUM!`; nonnumeric `k` gives `#VALUE!`. Referenced text, logical values and blanks do not become extra numeric observations, while a numeric zero does. Duplicate numbers remain separate observations: do not deduplicate a data set before finding its percentile.

The current helper returns the sole observation for any admitted `k` when `n=1`. Interpolated answers need not occur in the source data. This is a different operation from returning the nearest observed value or taking a median of hand-selected halves.

## How to implement

1. Collect the numeric observations using the function’s admission rules and reject an empty set. Validate `0<=k<=1`.
2. Sort ascending, retaining repeated observations. For one observation, return it.
3. Form the one-based rank `h=1+k*(n-1)` in the current binary64 operation order.
4. Let `j=floor(h)`. If the rank is integral, return sorted observation `j`, converting the one-based position to a zero-based index as needed.
5. Otherwise store `f=h-j` and interpolate `lower+f*(upper-lower)` between observations `j` and `j+1`.

For four observations and `k=0.25`, `h=1.75`, `f=0.75` and the result is `10+0.75*(20-10)=17.5`. These steps specify the indexing and interpolation. It does not identify every Excel rounding boundary: the current helper uses binary64 arithmetic, and algebraically rearranging the interpolation changes its numerical candidate.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[PERCENTILE.INC](FUNC.PERCENTILE.INC.md) names the inclusive convention explicitly; [PERCENTILE.EXC](FUNC.PERCENTILE.EXC.md) uses the exclusive one. [QUARTILE](FUNC.QUARTILE.md) selects a multiple of one quarter.

## Sources

- [Microsoft: PERCENTILE function](https://support.microsoft.com/en-us/excel/functions/percentile-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
