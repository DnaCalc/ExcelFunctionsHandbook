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

`=QUARTILE({10,20,30,40},1)` returns `17.5`, the inclusive 25th percentile. Selector two returns `25`, halfway between twenty and thirty.

## Quirks

The selector is truncated to an integer. Empty numeric data or an admitted selector outside zero through four gives `#NUM!`. The exclusive quartile convention can produce different values from this compatibility function.

## How to implement

Collect and sort numeric observations, validate the truncated selector q, and call the inclusive percentile calculation with `k=q/4`. Its one-based rank is `1+(n-1)*k`. For q=1 and four observations this is 1.75; interpolate `10+0.75*(20-10)=17.5`. Keep data admission separate from sorting and retain numeric zeros. A direct split into “halves” and then taking their medians can implement a different quartile definition.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[QUARTILE.INC](FUNC.QUARTILE.INC.md) names this convention; [QUARTILE.EXC](FUNC.QUARTILE.EXC.md) uses the exclusive convention; [PERCENTILE.INC](FUNC.PERCENTILE.INC.md) accepts arbitrary fractions.

## Sources

- [Microsoft: QUARTILE function](https://support.microsoft.com/en-us/excel/functions/quartile-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/percentile_common.rs); implementation steps and the limits identified in the text.
