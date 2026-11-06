---
schema: efh.function-page/v1
function_id: FUNC.QUARTILE
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

# QUARTILE

## What it does

`QUARTILE(array,quart)` is the compatibility function for inclusive quartiles. The selector zero means minimum, one lower quartile, two median, three upper quartile, and four maximum.

## Try it

`=QUARTILE({10,20,30,40},1)` returns `17.5`, the inclusive 25th percentile. The five selectors give:

| Selector | Meaning | Value for these observations |
|---:|---|---:|
| 0 | minimum | 10 |
| 1 | lower quartile | 17.5 |
| 2 | median | 25 |
| 3 | upper quartile | 32.5 |
| 4 | maximum | 40 |

Using medians of the lower and upper halves would give different lower and upper quartiles for this example.

## Quirks

The selector is truncated to an integer. Empty numeric data or a selector outside zero through four after the current truncation gives `#NUM!`; a nonnumeric selector gives `#VALUE!`. Referenced text, logical values and blanks do not extend the numeric sample; zeros and repeated numbers do.

The array need not already be sorted. The selector is a quartile number, not a percentile fraction: pass one for the lower quartile, rather than `0.25`. The exclusive quartile convention uses different positions and can reject a requested quartile when the sample is too small.

## How to implement

Collect the numeric observations, truncate and validate the selector `q`, then call the inclusive percentile helper with `k=q/4`. Sorting and interpolation follow [PERCENTILE](FUNC.PERCENTILE.md#how-to-implement).

For `q=1,n=4`, the one-based position is `1+(4-1)/4=1.75`; interpolate `10+0.75*(20-10)=17.5`. For `q=3`, the position is `3.25`, giving `30+0.25*(40-30)=32.5`. Selectors zero and four select endpoints without interpolation. A single numeric observation supplies all five inclusive quartiles.

This definition is a specific percentile convention. Splitting the sample into halves, dropping repeated observations or changing how an odd middle observation joins those halves implements another convention. The current helper’s rank and interpolation arithmetic still need their own numerical controls before a whole-domain Excel claim.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[QUARTILE.INC](FUNC.QUARTILE.INC.md) names this convention; [QUARTILE.EXC](FUNC.QUARTILE.EXC.md) uses the exclusive convention; [PERCENTILE.INC](FUNC.PERCENTILE.INC.md) accepts arbitrary fractions.

## Sources

- [Microsoft: QUARTILE function](https://support.microsoft.com/en-us/excel/functions/quartile-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
