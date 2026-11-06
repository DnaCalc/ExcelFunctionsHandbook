---
schema: efh.function-page/v1
function_id: FUNC.RANK
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

# RANK

## What it does

`RANK(number,ref,[order])` returns a number’s position within numeric observations. The default order is descending, so the largest value has rank one; a nonzero order requests ascending rank. It is the compatibility spelling of the tied-equal rank.

## Try it

`=RANK(20,{30,20,20,10})` returns `2`. Both twenties have rank two and the ten has rank four: ties leave a gap.

## Quirks

Nonnumeric values in the reference are ignored. Equal values share the rank they first occupy, rather than receiving consecutive ranks or an average rank. A number absent from the admitted data gives `#N/A` in the current helper. Changing the order reverses which observations precede it.

## How to implement

After collecting numeric observations, determine whether the target is present. Descending rank is `1+count(value>target)`; ascending rank is `1+count(value<target)`. In the example only thirty is greater than twenty, so the result is two. Sorting is optional for a single query; repeated queries can benefit from a sorted index. Keep exact target equality distinct from text coercion, and preserve the call’s argument-validation policy.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[RANK.EQ](FUNC.RANK.EQ.md) makes this tie rule explicit; [RANK.AVG](FUNC.RANK.AVG.md) averages tied positions; [PERCENTRANK.INC](FUNC.PERCENTRANK.INC.md) returns fractional standing.

## Sources

- [Microsoft: RANK function](https://support.microsoft.com/en-us/excel/functions/rank-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/statistics.rs); implementation steps and the limits identified in the text.
